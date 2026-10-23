#!/usr/bin/env python3
"""Insert (or remove) the temporary font-enumeration probe in src/sfntfont-android.rs.

The 4.829 reading says the process stops in the allocation path, but two threads append to the one
`probe.log`: the thread that loads Lisp and the thread that runs `Fandroid_enumerate_fonts`.  A
crash in the second one leaves the first one's last line at the end of the file, which reads as a
crash in whatever the first thread was doing.  This probe gives the enumeration thread its own
tid-tagged lines:

    AF <tid> dir  <directory index>   one system font directory is being scanned
    AF <tid> enum <directory index>   `sfnt_enum_font` returned for one font file

    tools/add-android-font-probe.py            # insert
    tools/add-android-font-probe.py --remove   # take it out again (byte-identical round trip)
"""
import pathlib
import sys

SRC = pathlib.Path (__file__).resolve ().parent.parent / "src/sfntfont-android.rs"

HELPER = '''/* TEMPORARY font-enumeration probe (tools/add-android-font-probe.py).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn af_probe (tag: *const c_char, a: isize, b: isize)
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
  c_write (fd, c"AF ".as_ptr () as *const c_void, 3);
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

# (anchor, probe, "after" or "before") - never between an `if` and its `{`
SITES = [
  ("      if dir.is_null ()\n",
   '      af_probe (c"dir".as_ptr (), i as isize, 0);\n', "before"),
  ("\t      sfnt_enum_font (name.as_mut_ptr ());\n",
   '      af_probe (c"enum".as_ptr (), i as isize, 0);\n', "after"),
]


def main():
    s = SRC.read_text()
    if "--remove" in sys.argv:
        s = s.replace(HELPER, "", 1)
        for anchor, probe, where in SITES:
            joined = (anchor + probe) if where == "after" else (probe + anchor)
            s = s.replace(joined, anchor, 1)
        SRC.write_text(s)
        print("removed; occurrences left:", s.count("af_probe"))
        return
    if "af_probe" in s:
        raise SystemExit("already inserted")
    # the helper goes above the DEFUN! that expands into the function (and above its own cfg line)
    anchor = 'DEFUN! ("android-enumerate-fonts", Fandroid_enumerate_fonts,'
    if s.count(anchor) != 1:
        raise SystemExit("the DEFUN header was not found exactly once")
    i = s.index(anchor)
    prev = s.rfind("\n", 0, i) + 1
    while prev > 0:
        start = s.rfind("\n", 0, prev - 1) + 1
        if not s[start:prev].startswith("#["):
            break
        prev = start
    s = s[:prev] + HELPER + s[prev:]
    for a, probe, where in SITES:
        if s.count(a) != 1:
            raise SystemExit(f"anchor count {s.count(a)}: {a.strip()[:40]!r}")
        s = s.replace(a, (a + probe) if where == "after" else (probe + a), 1)
    SRC.write_text(s)
    print("inserted; af_probe occurrences:", s.count("af_probe"))


main()
