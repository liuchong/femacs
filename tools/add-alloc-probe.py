#!/usr/bin/env python3
"""Insert (or remove) the temporary allocation/GC probe in femacs/src/alloc.rs.

The device run stops between `make_vector (1, elt)` entering and returning, which is also where
a collection can start, so the two candidates are separated by four tags:

    mv0/mv1   around `allocate_clear_vector` inside `make_vector`
    gc0/gce   around `garbage_collect` itself (`gce` sits before its closing brace, found by
              matching braces rather than by a guessed anchor)

    tools/add-alloc-probe.py            # insert
    tools/add-alloc-probe.py --remove   # take it out again (byte-identical round trip)
"""
import pathlib
import sys

SRC = pathlib.Path("/Users/liu/projects/src/github.com/liuchong/femacs/femacs/src/alloc.rs")

HELPER = '''/* TEMPORARY allocation probe (tools/add-alloc-probe.py).  */
#[cfg (feature = "have_android")]
unsafe fn al_probe (tag: *const c_char, a: isize, b: isize)
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
  c_write (fd, c"AL ".as_ptr () as *const c_void, 3);
  let tid: c_int = c_gettid ();
  {
    let mut td = [0u8; 12]; let mut tk = 0usize; let mut tx: isize = tid as isize;
    if tx == 0 { td[0] = b'0'; tk = 1; }
    while tx > 0 { td[tk] = b'0' + (tx % 10) as u8; tx /= 10; tk += 1; }
    while tk > 0 { tk -= 1; c_write (fd, td.as_ptr ().add (tk) as *const c_void, 1); }
  }
  c_write (fd, c" ".as_ptr () as *const c_void, 1);
  c_write (fd, tag as *const c_void, t);
  for v in [a, b] {
    let mut d = [0u8; 24]; let mut k = 0usize; let mut x = v;
    d[k] = b' '; k += 1;
    if x < 0 { d[k] = b'-'; k += 1; x = -x; }
    let mut s = [0u8; 24]; let mut m = 0usize;
    if x == 0 { s[0] = b'0'; m = 1; }
    while x > 0 { s[m] = b'0' + (x % 10) as u8; x /= 10; m += 1; }
    while m > 0 { m -= 1; d[k] = s[m]; k += 1; }
    c_write (fd, d.as_ptr () as *const c_void, k);
  }
  c_write (fd, c"\\n".as_ptr () as *const c_void, 1);
  c_close (fd);
}

'''

MV_SIG = ('#[unsafe(no_mangle)]\n'
          'pub unsafe extern "C" fn make_vector (length: ptrdiff_t, init: Lisp_Object) -> Lisp_Object\n'
          '{\n')
GC_SIG = 'pub unsafe extern "C" fn garbage_collect ()\n{\n'
GC_HEAD = '#[unsafe(no_mangle)]\npub unsafe extern "C" fn garbage_collect ()\n{\n'

MV0 = '  al_probe (c"mv0".as_ptr (), length as isize, 0);\n'
MV1 = '  al_probe (c"mv1".as_ptr (), 0, 0);\n'
GC0 = '  al_probe (c"gc0".as_ptr (), 0, 0);\n'
GCE = '  al_probe (c"gce".as_ptr (), 0, 0);\n'
MV_ALLOC = '  let mut p: *mut Lisp_Vector = allocate_clear_vector (length, clearit);\n'


def gc_end(s):
    """Offset of the closing brace of `garbage_collect`, by matching braces."""
    i = s.index(GC_SIG)
    k = i + len(GC_SIG)
    depth = 1
    while depth > 0:
        ch = s[k]
        if ch == '{':
            depth += 1
        elif ch == '}':
            depth -= 1
        k += 1
    return k - 1


def main():
    s = SRC.read_text()
    if "--remove" in sys.argv:
        s = s.replace(HELPER, "", 1)
        s = s.replace(MV_SIG + MV0, MV_SIG, 1)
        s = s.replace(MV_ALLOC + MV1, MV_ALLOC, 1)
        s = s.replace(GC_HEAD + GC0, GC_HEAD, 1)
        end = gc_end(s)
        if s[end - len(GCE):end] == GCE:
            s = s[:end - len(GCE)] + s[end:]
        SRC.write_text(s)
        print("removed; occurrences left:", s.count("al_probe"))
        return
    if "al_probe" in s:
        raise SystemExit("already inserted")
    for anchor in (MV_SIG, GC_HEAD, MV_ALLOC):
        if s.count(anchor) != 1:
            raise SystemExit(f"anchor count {s.count(anchor)}: {anchor[:40]!r}")
    s = s.replace(MV_SIG, MV_SIG + MV0, 1)
    s = s.replace(MV_ALLOC, MV_ALLOC + MV1, 1)
    i = s.index(GC_HEAD)
    s = s[:i] + HELPER + s[i:]
    s = s.replace(GC_HEAD, GC_HEAD + GC0, 1)
    end = gc_end(s)
    s = s[:end] + GCE + s[end:]
    SRC.write_text(s)
    print("inserted; al_probe occurrences:", s.count("al_probe"))


main()
