#!/usr/bin/env python3
"""Insert (or remove) fd-2 startup probes in `lread.rs`, `fringe.rs` and `emacs.rs`.

The Android app now exits with SIGSEGV a few `load`s into `loadup.el` (status 4.805), and the
release log cannot say where: the reference's own crash backtrace goes through
`backtrace_symbols_fd`, whose Android mirror is the gnulib no-op.  These three probes answer the
three open questions directly, and write to fd 2 (`write (2, ...)`) so nothing is buffered:

  * every `readevalloop` prints the source name  -> the last line is the file being loaded
    when the process died, which pins the crash to one `loadup.el` step;
  * `init_fringe_bitmap` prints `fringe <which>` -> that function is the port's largest
    `words_bigendian` consumer and the one arm whose polarity just changed;
  * `syms_of_emacs` prints `system-type`         -> the other behavioural change in the same
    binary (`SYSTEM_TYPE` went from the darwin to the android value).

    python3 tools/add-startup-probe.py            # insert
    python3 tools/add-startup-probe.py --remove   # restore from the saved copies
"""
import pathlib, shutil, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
BAK = pathlib.Path("/tmp/startup-probe-backups")

# The helpers declare `write` themselves: `fringe.rs` has no `write` in scope (E0423, measured
# on this tool's first run), and the declaration is the libc symbol either way.
HELPER = '''/* TEMPORARY probe (tools/add-startup-probe.py).  */
unsafe fn startup_probe (tag: *const c_char, s: *const c_char)
{
  unsafe extern "C" { fn write (fd: c_int, buf: *const c_void, n: usize) -> isize; }
  write (2, c"STARTUP ".as_ptr () as *const c_void, 8);
  write (2, tag as *const c_void, strlen (tag));
  write (2, c" ".as_ptr () as *const c_void, 1);
  if !s.is_null () { write (2, s as *const c_void, strlen (s)); }
  write (2, c"\\n".as_ptr () as *const c_void, 1);
}

unsafe fn startup_probe_n (tag: *const c_char, v: isize)
{
  unsafe extern "C" { fn write (fd: c_int, buf: *const c_void, n: usize) -> isize; }
  let mut d = [0u8; 24]; let mut k = 0usize; let mut x = v; let neg = x < 0;
  if neg { x = -x; }
  if x == 0 { d[0] = b'0'; k = 1; }
  while x > 0 { d[k] = b'0' + (x % 10) as u8; x /= 10; k += 1; }
  write (2, c"STARTUP ".as_ptr () as *const c_void, 8);
  write (2, tag as *const c_void, strlen (tag));
  write (2, c" ".as_ptr () as *const c_void, 1);
  if neg { write (2, c"-".as_ptr () as *const c_void, 1); }
  while k > 0 { k -= 1; write (2, d.as_ptr ().add (k) as *const c_void, 1); }
  write (2, c"\\n".as_ptr () as *const c_void, 1);
}

'''

EDITS = [
    ("femacs/src/lread.rs",
     "unsafe fn readevalloop (readcharfun: Lisp_Object, infile0: *mut infile,",
     "  if !NILP (sourcename)\n    { CHECK_STRING (sourcename); }\n",
     "  if !NILP (sourcename)\n    { CHECK_STRING (sourcename); }\n"
     "  if STRINGP (sourcename)\n"
     "    { startup_probe (c\"load\".as_ptr (), SSDATA (sourcename) as *const c_char); }\n"),
    ("femacs/src/fringe.rs",
     "unsafe fn init_fringe_bitmap (which: c_int, fb: *mut fringe_bitmap, once_p: c_int)",
     "unsafe fn init_fringe_bitmap (which: c_int, fb: *mut fringe_bitmap, once_p: c_int)\n{\n",
     "unsafe fn init_fringe_bitmap (which: c_int, fb: *mut fringe_bitmap, once_p: c_int)\n{\n"
     "  startup_probe_n (c\"fringe\".as_ptr (), which as isize);\n"),
    ("femacs/src/emacs.rs",
     "  Vsystem_type = intern_c_string (SYSTEM_TYPE.as_ptr ());",
     "  Vsystem_type = intern_c_string (SYSTEM_TYPE.as_ptr ());\n",
     "  Vsystem_type = intern_c_string (SYSTEM_TYPE.as_ptr ());\n"
     "  startup_probe (c\"system-type\".as_ptr (), SYSTEM_TYPE.as_ptr ());\n"),
    # `file-exists-p "/assets/build_info"` is the call `version.el` makes now that `system-type`
    # is the faithful `android` (status 4.805).  These three bracket it: the Lisp-level entry
    # (`check_file_access`), the VFS entry (`android_faccessat`), and the asset VFS's own
    # `access`.
    ("femacs/src/fileio.rs",
     "unsafe fn check_file_access (file: Lisp_Object, operation: Lisp_Object,",
     "  let mut file = file;                        /* C reassigns `file` below.  */\n"
     "  file = Fexpand_file_name (file, Qnil ());\n",
     "  let mut file = file;                        /* C reassigns `file` below.  */\n"
     "  file = Fexpand_file_name (file, Qnil ());\n"
     "  if STRINGP (file)\n"
     "    { startup_probe (c\"check-access\".as_ptr (), SSDATA (file) as *const c_char); }\n"),
    ("femacs/src/androidvfs.rs",
     "pub unsafe extern \"C\" fn android_faccessat (dirfd: c_int, mut pathname: *const c_char,",
     "					    mode: c_int, flags: c_int) -> c_int\n{\n",
     "					    mode: c_int, flags: c_int) -> c_int\n{\n"
     "  startup_probe (c\"faccessat\".as_ptr (), pathname);\n"),
    ("femacs/src/androidvfs.rs",
     "unsafe fn android_scan_directory_tree (file: *const c_char, limit_return: *mut size_t)",
     "unsafe fn android_scan_directory_tree (file: *const c_char, limit_return: *mut size_t)\n"
     "				       -> *const c_char\n{\n",
     "unsafe fn android_scan_directory_tree (file: *const c_char, limit_return: *mut size_t)\n"
     "				       -> *const c_char\n{\n"
     "  startup_probe (c\"scan-tree\".as_ptr (), file);\n"),
    ("femacs/src/androidvfs.rs",
     "unsafe extern \"C\" fn android_afs_access (vnode: *mut android_vnode, mode: c_int) -> c_int",
     "unsafe extern \"C\" fn android_afs_access (vnode: *mut android_vnode, mode: c_int) -> c_int\n{\n",
     "unsafe extern \"C\" fn android_afs_access (vnode: *mut android_vnode, mode: c_int) -> c_int\n{\n"
     "  startup_probe (c\"afs-access\".as_ptr (), (*((vnode as *mut android_afs_vnode))).name);\n"),    # The font driver: with the byte order finally right (`status 4.806`), fonts register and the
    # startup proceeds into font matching and opening - the two places a defect can now live.
    ("femacs/src/sfntfont.rs",
     "pub unsafe extern \"C\" fn sfntfont_list (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object\n{                                                          // C 1978-2027",
     "pub unsafe extern \"C\" fn sfntfont_list (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object\n{                                                          // C 1978-2027\n",
     "pub unsafe extern \"C\" fn sfntfont_list (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object\n{                                                          // C 1978-2027\n"
     "  startup_probe_n (c\"sfntfont-list\".as_ptr (), 1);\n"),
    ("femacs/src/sfntfont.rs",
     "  let mut font_info: *mut sfnt_font_info;",
     "  let mut font_info: *mut sfnt_font_info;",
     "  let mut font_info: *mut sfnt_font_info;\n"
     "  startup_probe_n (c\"sfntfont-open\".as_ptr (), 1);"),
]


def insert_above_attributes(text: str, anchor: str, block: str) -> str:
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
        for rel, _, _, _ in EDITS:
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
            # Per edit, not per file: `androidvfs.rs` takes three probes, and a file-level guard
            # silently dropped the second and third (measured on this tool's first run).
            print("%s: this probe is already there" % rel)
            continue
        if not (BAK / p.name).is_file():
            shutil.copyfile(p, BAK / p.name)
        if text.count(old) != 1:
            raise SystemExit("anchor appears %d times in %s" % (text.count(old), rel))
        text = text.replace(old, new, 1)
        if "TEMPORARY probe (tools/add-startup-probe.py)" not in text:
            text = insert_above_attributes(text, fn_anchor, HELPER)
        p.write_text(text)
        print("probed %s" % rel)
    return 0


if __name__ == "__main__":
    sys.exit(main())
