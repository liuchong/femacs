#!/usr/bin/env python3
"""Insert (or remove) the `/assets` file-descriptor probes.

`status 4.806` localized the Android stop: `loadup.el` reaches `load version.elc`, which runs
`android-read-build-system`, which is the first call to touch `/assets/build_info` now that
`system-type` is the faithful `android`; the Lisp error is `End of file during parsing:
#<killed buffer>`, which is what `(read (current-buffer))` says when the buffer those functions
filled is empty.  So the question is what the file-descriptor path handed to
`insert-file-contents`:

      asset-size        what `AAsset_getLength` reported for the asset
      ashmem-readable   which half of `android_hack_asset_fd` ran (detection result)
      ashared-fd        the descriptor `ASharedMemory_create` returned (the API > 28 path)
      insert st_size    what `insert-file-contents` measured for the same file

A zero `asset-size`/`st_size` next to a valid `ashared-fd` says the region was created but never
filled, and the fix belongs in that branch; a positive size with an empty result says the reader
is at fault.

Every probe writes to fd 2 with its own `write` declaration and fully-qualified `core::ffi` types,
because `fringe.rs`/`androidvfs.rs` have neither in scope in the `enable_checking` leg (measured).

    python3 tools/add-asset-fd-probe.py            # insert
    python3 tools/add-asset-fd-probe.py --remove   # restore from the saved copies
"""
import pathlib, shutil, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
BAK = pathlib.Path("/tmp/asset-fd-probe-backups")

HELPER = '''/* TEMPORARY probe (tools/add-asset-fd-probe.py).  */
unsafe fn asset_probe (tag: *const core::ffi::c_char, v: isize)
{
  unsafe extern "C" { fn write (fd: core::ffi::c_int, buf: *const core::ffi::c_void,
                                n: usize) -> isize; }
  let mut d = [0u8; 48]; let mut k = 0usize; let mut x = v; let neg = x < 0;
  let mut name = [0u8; 48]; let mut n = 0usize;
  while *tag.add (n) != 0 && n < 47 { name[n] = *tag.add (n) as u8; n += 1; }
  if neg { x = -x; }
  if x == 0 { d[0] = b'0'; k = 1; }
  while x > 0 { d[k] = b'0' + (x % 10) as u8; x /= 10; k += 1; }
  write (2, c"ASSET ".as_ptr () as *const core::ffi::c_void, 6);
  write (2, name.as_ptr () as *const core::ffi::c_void, n);
  write (2, c" ".as_ptr () as *const core::ffi::c_void, 1);
  if neg { write (2, c"-".as_ptr () as *const core::ffi::c_void, 1); }
  while k > 0 { k -= 1; write (2, d.as_ptr ().add (k) as *const core::ffi::c_void, 1); }
  write (2, c"\\n".as_ptr () as *const core::ffi::c_void, 1);
}

unsafe fn asset_probe_s (tag: *const core::ffi::c_char, s: *const core::ffi::c_char)
{
  unsafe extern "C" { fn write (fd: core::ffi::c_int, buf: *const core::ffi::c_void,
                                n: usize) -> isize; }
  let mut name = [0u8; 48]; let mut n = 0usize;
  while *tag.add (n) != 0 && n < 47 { name[n] = *tag.add (n) as u8; n += 1; }
  write (2, c"ASSET ".as_ptr () as *const core::ffi::c_void, 6);
  write (2, name.as_ptr () as *const core::ffi::c_void, n);
  write (2, c" ".as_ptr () as *const core::ffi::c_void, 1);
  if !s.is_null () {
    let mut m = 0usize;
    while *s.add (m) != 0 && m < 128 { m += 1; }
    write (2, s as *const core::ffi::c_void, m);
  }
  write (2, c"\\n".as_ptr () as *const core::ffi::c_void, 1);
}

'''

EDITS = [
    # 1. the asset's reported length
    ("femacs/src/androidvfs.rs", "unsafe fn android_hack_asset_fd (asset: *mut AAsset) -> c_int",
     "  size = AAsset_getLength (asset) as size_t;                            // C 1494",
     "  size = AAsset_getLength (asset) as size_t;                            // C 1494\n"
     "  asset_probe (c\"asset-size\".as_ptr () as *const core::ffi::c_char, size as isize);"),
    # 2. which half of the function runs
    ("femacs/src/androidvfs.rs", "unsafe fn android_hack_asset_fd (asset: *mut AAsset) -> c_int",
     "  if !ashmem_readable_p",
     "  asset_probe (c\"ashmem-readable\".as_ptr () as *const core::ffi::c_char,\n"
     "	       ashmem_readable_p as isize);\n"
     "  if !ashmem_readable_p"),
    # 3. the descriptor the API > 28 path obtained
    ("femacs/src/androidvfs.rs", "unsafe fn android_hack_asset_fd (asset: *mut AAsset) -> c_int",
     "  fd = (asharedmemory_create.unwrap ()) (c\"\".as_ptr (), size);           // C 1578",
     "  fd = (asharedmemory_create.unwrap ()) (c\"\".as_ptr (), size);           // C 1578\n"
     "  asset_probe (c\"ashared-fd\".as_ptr () as *const core::ffi::c_char, fd as isize);"),
    # 4. what `insert-file-contents` measured for the file it is about to read
    ("femacs/src/fileio.rs", "DEFUN! (\"insert-file-contents\", Finsert_file_contents, Sinsert_file_contents,",
     "	    st_size = st.st_size;",
     "	    st_size = st.st_size;\n"
     "	    asset_probe_s (c\"insert\".as_ptr () as *const core::ffi::c_char,\n"
     "			   SSDATA (filename) as *const core::ffi::c_char);\n"
     "	    asset_probe (c\"insert-size\".as_ptr () as *const core::ffi::c_char, st_size as isize);"),
]


def insert_above_attributes(text, anchor, block):
    i = text.index(anchor)
    while i > 0:
        prev_end = i - 1
        prev_start = text.rfind(chr(10), 0, prev_end) + 1
        if not text[prev_start:prev_end].startswith("#["):
            break
        i = prev_start
    return text[:i] + block + text[i:]


def main():
    if "--remove" in sys.argv:
        for rel in ("femacs/src/androidvfs.rs", "femacs/src/fileio.rs"):
            p = ROOT / rel
            b = BAK / p.name
            if b.is_file():
                shutil.copyfile(b, p)
                b.unlink()
                print("restored %s" % rel)
        return 0
    BAK.mkdir(parents=True, exist_ok=True)
    for rel, fn_anchor, old, new in EDITS:
        p = ROOT / rel
        text = p.read_text()
        if new in text:
            print("%s: this probe is already there" % rel)
            continue
        if not (BAK / p.name).is_file():
            shutil.copyfile(p, BAK / p.name)
        if text.count(old) != 1:
            raise SystemExit("anchor appears %d times in %s: %r" % (text.count(old), rel, old[:60]))
        text = text.replace(old, new, 1)
        if "TEMPORARY probe (tools/add-asset-fd-probe.py)" not in text:
            text = insert_above_attributes(text, fn_anchor, HELPER)
        p.write_text(text)
        print("probed %s" % rel)
    return 0


if __name__ == "__main__":
    sys.exit(main())
