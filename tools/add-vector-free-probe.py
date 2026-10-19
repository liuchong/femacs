#!/usr/bin/env python3
"""Insert (or remove) the free-list probe in femacs/src/alloc.rs.

4.836's tombstone read as `vector_free_lists[0]` holding -2, which only happens when a vector on
that list has `contents[0] == 0` (`next_vector` subtracts the Lisp_Int0 tag from a value that was
never written).  This probe watches the list itself and keeps the log small, so it does not move
the crash the way the 4.829 probes did:

    sweep            `sweep_vectors` started (once per collection)
    push  vindex v   `setup_on_free_list` linked a vector
    pop   vindex v   `allocate_vector_from_block` took the exact-size head
    pop2  vindex v   ... or a larger-bin head
    zero  vindex v   the entry that was just linked or taken has `contents[0] == 0` <- the defect

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
  ("unsafe fn setup_on_free_list (v: *mut Lisp_Vector, nbytes: ptrdiff_t)\n{\n",
   "after", '  vf_probe (c"enter-push".as_ptr (), VINDEX (nbytes), v as isize);\n'),
  ("  vindex = min! (vindex, VECTOR_FREE_LIST_ARRAY_SIZE - 1);\n",
   "after", '  vf_probe (c"pre".as_ptr (), vindex,\n'
            '            *(*v).contents.as_mut_ptr ().offset (0) as isize);\n'
            '  vf_probe (c"head".as_ptr (), vindex, vector_free_lists[vindex as usize] as isize);\n'),
  ("  set_next_vector (v, vector_free_lists[vindex as usize]);\n",
   "after", '  vf_probe (c"push".as_ptr (), vindex,\n'
            '            *(*v).contents.as_mut_ptr ().offset (0) as isize);\n'),
  ("unsafe fn sweep_vectors ()\n{\n",
   "after", '  vf_probe (c"sweep".as_ptr (), 0, 0);\n'),
  # the exact-size path: the probe reads `vector`, so it has to sit after the assignment
  ("      vector = vector_free_lists[index as usize];\n"
   "      ASAN_UNPOISON_VECTOR_CONTENTS! (vector, nbytes - header_size);\n",
   "between", '      vf_probe (c"pop".as_ptr (), index,\n'
              '                *(*vector).contents.as_mut_ptr ().offset (0) as isize);\n'),
  ("	  vector_free_lists[index as usize] = next_vector (vector);\n\n"
   "	  /* Excess bytes are used for the smaller vector,\n",
   "before", '	  vf_probe (c"pop2".as_ptr (), index,\n'
             '		    *(*vector).contents.as_mut_ptr ().offset (0) as isize);\n'),
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
