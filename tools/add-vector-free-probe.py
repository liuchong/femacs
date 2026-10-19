#!/usr/bin/env python3
"""Insert (or remove) the free-list probe in femacs/src/alloc.rs.

4.836's tombstone read as `vector_free_lists[0]` holding -2, which only happens when a vector on
that list has `contents[0] == 0` (`next_vector` subtracts the Lisp_Int0 tag from a value that was
never written).  This probe watches the list itself and keeps the log small, so it does not move
the crash the way the 4.829 probes did:

    sweep            `sweep_vectors` started (once per collection)
    push  vindex v   `setup_on_free_list` linked a vector
    take   vindex nbytes   `allocate_vector_from_block` took the exact-size head, with the size the
    take2  vindex nbytes   ... or the larger-bin head,      record's own header reports
    pre/head/push          `setup_on_free_list`: contents[0] before the link, the bin's head, and
                           contents[0] after it
    split-push nbytes restbytes / sweep-push total_bytes   the two producers of free-list links

    tools/add-vector-free-probe.py            # insert
    tools/add-vector-free-probe.py --remove   # take it out again (byte-identical round trip)
"""
import pathlib
import sys

SRC = pathlib.Path (__file__).resolve ().parent.parent / "femacs/src/alloc.rs"

HELPER = '''/* TEMPORARY free-list probe (tools/add-vector-free-probe.py).  */
#[cfg (feature = "have_android")]
unsafe fn vf_probe (tag: *const c_char, a: isize, b: isize)
{
  unsafe extern "C" {
    #[link_name = "openat"] fn c_openat (dirfd: c_int, path: *const c_char,
                                         flags: c_int, mode: c_int) -> c_int;
    #[link_name = "write"] fn c_write (fd: c_int, buf: *const c_void, n: usize) -> isize;
    #[link_name = "close"] fn c_close (fd: c_int) -> c_int;
    #[link_name = "gettid"] fn c_gettid () -> c_int;
  }
  let mut t: usize = 0;
  while *tag.add (t) != 0 { t += 1; }
  let fd = c_openat (-100, c"/data/data/org.femacs/files/probe.log".as_ptr (), 1 | 64 | 1024, 0o666);
  if fd < 0 { return; }
  c_write (fd, c"VF ".as_ptr () as *const c_void, 3);
  let tid: c_int = c_gettid ();
  for v in [tid as isize, a, b] {
    let mut d = [0u8; 24]; let mut k = 0usize; let mut x = v;
    d[k] = b' '; k += 1;
    if x < 0 { d[k] = b'-'; k += 1; x = -x; }
    let mut s = [0u8; 24]; let mut m = 0usize;
    if x == 0 { s[0] = b'0'; m = 1; }
    while x > 0 { s[m] = b'0' + (x % 10) as u8; x /= 10; m += 1; }
    while m > 0 { m -= 1; d[k] = s[m]; k += 1; }
    c_write (fd, d.as_ptr () as *const c_void, k);
  }
  c_write (fd, c" ".as_ptr () as *const c_void, 1);
  c_write (fd, tag as *const c_void, t);
  c_write (fd, c"\\n".as_ptr () as *const c_void, 1);
  c_close (fd);
}

'''

# (anchor, "after"/"before", probe text)
SITES = [
  # the caller of the split: what size it asked for, and what the returned vector says afterwards
  ("  if nbytes <= VBLOCK_BYTES_MAX\n    {\n", "after",
   '      vf_probe (c"al-in".as_ptr (), nbytes, 0);\n'),
  ("unsafe fn setup_on_free_list (v: *mut Lisp_Vector, nbytes: ptrdiff_t)\n{\n",
   "after", '  vf_probe (c"enter-push".as_ptr (), VINDEX (nbytes), v as isize);\n'),
  # `VINDEX` of a size below VBLOCK_BYTES_MIN is negative, and C's two `eassume`s that would catch it
  # compile out without `enable_checking` - so the array is written out of bounds.  One line per
  # occurrence names the first short link and its caller (the split-push/sweep-push line above it).
  ("unsafe fn setup_on_free_list (v: *mut Lisp_Vector, nbytes: ptrdiff_t)\n{\n",
   "after", '  if nbytes < VBLOCK_BYTES_MIN\n'
            '    { vf_probe (c"short".as_ptr (), nbytes, v as isize); }\n'),
  ("  vindex = min! (vindex, VECTOR_FREE_LIST_ARRAY_SIZE - 1);\n",
   "after", '  vf_probe (c"pre".as_ptr (), vindex,\n'
            '            *(*v).contents.as_mut_ptr ().offset (0) as isize);\n'
            '  vf_probe (c"head".as_ptr (), vindex, vector_free_lists[vindex as usize] as isize);\n'),
  ("  set_next_vector (v, vector_free_lists[vindex as usize]);\n",
   "after", '  vf_probe (c"push".as_ptr (), v as isize, (*v).header.size as isize);\n'),
  # who calls `setup_on_free_list`, and with what byte count.  The 4.838 run answered this: the
  # first oversized split is `(nbytes 144, restbytes 31976)` - six times VECTOR_BLOCK_BYTES - so a
  # bin head was taken whose `vector_nbytes` is ~32 KB.  Next round logs that head's size at the pop.
  ("	  setup_on_free_list (ADVANCE (vector, nbytes), restbytes);\n",
   "before", '	  vf_probe (c"split-push".as_ptr (), nbytes, restbytes);\n'),
  ("		  setup_on_free_list (vector, total_bytes);\n",
   "before", '		  vf_probe (c"sweep-push".as_ptr (), total_bytes, 0);\n'),
  ("unsafe fn sweep_vectors ()\n{\n",
   "after", '  vf_probe (c"sweep".as_ptr (), 0, 0);\n'),
  # the exact-size path: the probe reads `vector`, so it has to sit after the assignment
  ("      vector = vector_free_lists[index as usize];\n"
   "      ASAN_UNPOISON_VECTOR_CONTENTS! (vector, nbytes - header_size);\n",
   "between", '      vf_probe (c"take".as_ptr (), vector as isize, (*vector).header.size as isize);\n'),
  ("	  vector_free_lists[index as usize] = next_vector (vector);\n\n"
   "	  /* Excess bytes are used for the smaller vector,\n",
   "before", '	  vf_probe (c"take2".as_ptr (), vector as isize, (*vector).header.size as isize);\n'),
  ("  while !block.is_null ()\n    {\n      let mut free_this_block: bool = false;\n",
   "after", '      /* nothing: the sweep marker above is enough */\n'),
]


def apply(s, remove):
    for anchor, where, probe in SITES:
        if anchor.startswith("  while !block.is_null"):
            continue
        if s.count(anchor) != 1:
            raise SystemExit("anchor count %d: %r" % (s.count(anchor), anchor[:60]))
        if where == "between":
            first, rest = anchor.split ("\n", 1)
            joined = first + "\n" + probe + rest
        elif where == "after":
            joined = anchor + probe
        else:
            joined = probe + anchor
        if remove:
            if joined not in s:
                raise SystemExit("probe missing: %r" % probe[:40])
            s = s.replace(joined, anchor, 1)
        else:
            s = s.replace(anchor, joined, 1)
    return s


def main():
    s = SRC.read_text()
    if "--remove" in sys.argv:
        s = s.replace(HELPER, "", 1)
        s = apply(s, True)
        SRC.write_text(s)
        print("removed; occurrences left:", s.count("vf_probe"))
        return
    if "vf_probe" in s:
        raise SystemExit("already inserted")
    anchor = "unsafe fn setup_on_free_list (v: *mut Lisp_Vector, nbytes: ptrdiff_t)\n{\n"
    i = s.index(anchor)
    prev = s.rfind("\n", 0, i) + 1
    s = s[:prev] + HELPER + s[prev:]
    s = apply(s, False)
    SRC.write_text(s)
    print("inserted; vf_probe occurrences:", s.count("vf_probe"))


main()
