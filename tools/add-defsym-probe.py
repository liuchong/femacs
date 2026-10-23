#!/usr/bin/env python3
"""Insert (or remove) a probe at `defsym` in src/lread.rs.

The checking build's first fatal error is `lread.c`'s `eassert (FIXNUMP (bucket))` in `defsym`
(status 4.843, `/Users/.../units/lread/_root.rs:5207`), i.e. `oblookup` returned something that is
not a bucket index - a symbol, which C only returns when the name is *already* interned.  The
assertion does not say which name, and on Android stderr goes to the debug pipe, so this probe
writes the name and the value it got:

    DF <tid> <len> <bucket> name...   only when the bucket is not a fixnum

    tools/add-defsym-probe.py            # insert
    tools/add-defsym-probe.py --remove   # take it out again (byte-identical round trip)
"""
import pathlib
import sys

SRC = pathlib.Path (__file__).resolve ().parent.parent / "src/lread.rs"

HELPER = '''/* TEMPORARY probe (tools/add-defsym-probe.py).  */
#[cfg (feature = "have_android")]
unsafe fn df_probe (len: isize, bucket: isize, s: *const c_char)
{
  unsafe extern "C" {
    #[link_name = "openat"] fn c_openat (dirfd: c_int, path: *const c_char,
                                         flags: c_int, mode: c_int) -> c_int;
    #[link_name = "write"] fn c_write (fd: c_int, buf: *const c_void, n: usize) -> isize;
    #[link_name = "close"] fn c_close (fd: c_int) -> c_int;
  }
  let fd = c_openat (-100, c"/data/data/org.femacs/files/probe.log".as_ptr (), 1 | 64 | 1024, 0o666);
  if fd < 0 { return; }
  c_write (fd, c"DF ".as_ptr () as *const c_void, 3);
  for v in [len, bucket] {
    let mut d = [0u8; 24]; let mut k = 0usize; let mut x = v;
    if x < 0 { d[k] = b'-'; k += 1; x = -x; }
    let mut t = [0u8; 24]; let mut m = 0usize;
    if x == 0 { t[0] = b'0'; m = 1; }
    while x > 0 { t[m] = b'0' + (x % 10) as u8; x /= 10; m += 1; }
    while m > 0 { m -= 1; d[k] = t[m]; k += 1; }
    d[k] = b' '; k += 1;
    c_write (fd, d.as_ptr () as *const c_void, k);
  }
  c_write (fd, s as *const c_void, len as usize);
  c_write (fd, c"\\n".as_ptr () as *const c_void, 1);
  c_close (fd);
}

'''

ANCHOR = """      let bucket: Lisp_Object = oblookup (initial_obarray, str_, len, len);
      eassert! (FIXNUMP (bucket));
"""
PROBE = """      let bucket: Lisp_Object = oblookup (initial_obarray, str_, len, len);
      if !FIXNUMP (bucket)
        { df_probe (len as isize, bucket as isize, str_); }
      eassert! (FIXNUMP (bucket));
"""


def main():
    s = SRC.read_text()
    if "--remove" in sys.argv:
        s = s.replace(HELPER, "", 1)
        s = s.replace(PROBE, ANCHOR, 1)
        SRC.write_text(s)
        print("removed; occurrences left:", s.count("df_probe"))
        return
    if "df_probe" in s:
        raise SystemExit("already inserted")
    if s.count(ANCHOR) != 1:
        raise SystemExit("anchor count %d" % s.count(ANCHOR))
    i = s.index(ANCHOR)
    prev = s.rfind("\n", 0, i) + 1
    while prev > 0:
        start = s.rfind("\n", 0, prev - 1) + 1
        if not s[start:prev].startswith("#["):
            break
        prev = start
    s = s[:prev] + HELPER + s[prev:]
    s = s.replace(ANCHOR, PROBE, 1)
    SRC.write_text(s)
    print("inserted; df_probe occurrences:", s.count("df_probe"))


main()
