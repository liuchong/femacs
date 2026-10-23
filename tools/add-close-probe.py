#!/usr/bin/env python3
"""Insert (or remove) the `emacs_pipe`/`emacs_close` probe in src/sysdep.rs.

The 4.846 run reaches `loadup.el` and dies in the GC's `valid_pointer_p`, which builds a pipe,
writes the candidate pointer into it and closes both ends; `emacs_close`'s
`eassert (errno != EBADF || fd < 0)` fires, so one of those closes names an already-closed
descriptor.  This probe writes both descriptors and the close's errno to fd 2 (logcat).

    tools/add-close-probe.py            # insert
    tools/add-close-probe.py --remove   # take it out again (byte-identical round trip)
"""
import pathlib
import sys

SRC = pathlib.Path (__file__).resolve ().parent.parent / "src/sysdep.rs"

HELPER = '''/* TEMPORARY probe (tools/add-close-probe.py).  */
unsafe fn cl_probe (tag: *const c_char, a: c_int, b: c_int, c: c_int)
{
  unsafe extern "C" {
    #[link_name = "write"] fn c_write (fd: c_int, buf: *const c_void, n: usize) -> isize;
    #[link_name = "gettid"] fn c_gettid () -> c_int;
  }
  let mut t: usize = 0;
  while *tag.add (t) != 0 { t += 1; }
  c_write (2, tag as *const c_void, t);
  for v in [c_gettid (), a, b, c] {
    let mut d = [0u8; 24]; let mut k = 0usize; let mut x = v;
    d[k] = b' '; k += 1;
    if x < 0 { d[k] = b'-'; k += 1; x = -x; }
    let mut s = [0u8; 24]; let mut m = 0usize;
    if x == 0 { s[0] = b'0'; m = 1; }
    while x > 0 { s[m] = b'0' + (x % 10) as u8; x /= 10; m += 1; }
    while m > 0 { m -= 1; d[k] = s[m]; k += 1; }
    c_write (2, d.as_ptr () as *const c_void, k);
  }
  c_write (2, c"\\n".as_ptr () as *const c_void, 1);
}

'''

SITES = [
  ("    return pipe2! (fd, O_BINARY | O_CLOEXEC);   /* C: `pipe2` is unistd.in.h 1831's rename */\n",
   "replace",
   '    let rc = pipe2! (fd, O_BINARY | O_CLOEXEC);\n'
   '    cl_probe (c"PIPE".as_ptr (), rc, *fd, *fd.add (1));\n'
   '    return rc;\n'),
  ("\t  eassert! (*errno () != EBADF || fd < 0);\n",
   "before",
   '\t  cl_probe (c"CL".as_ptr (), fd, *errno (), r);\n'),
  ("\t  if *errno () != EINTR\n\t    { break; }\n",
   "before",
   '\t  cl_probe (c"WR".as_ptr (), fd, *errno (), n as c_int);\n'),
  # What does the descriptor look like on entry to, and right after, the write itself?
  ("      let n: ssize_t = write (fd, buf as *const c_void,\n",
   "before",
   '      cl_probe (c"WE".as_ptr (), fd, fcntl (fd, F_GETFD, 0), nbyte as c_int);\n'),
  ("\t  buf = buf.add (n as usize);\n",
   "before",
   '\t  cl_probe (c"WA".as_ptr (), fd, fcntl (fd, F_GETFD, 0), n as c_int);\n'),
  # Is the freshly created pipe really open on both ends?  And is the descriptor still open when
  # `emacs_close` is entered?  (4.847: close of a just-written pipe end reports EBADF.)
  ("    cl_probe (c\"PIPE\".as_ptr (), rc, *fd, *fd.add (1));\n",
   "after",
   '    if rc == 0\n'
   '      { cl_probe (c"PF".as_ptr (), fcntl (*fd, F_GETFD, 0), fcntl (*fd.add (1), F_GETFD, 0), 0); }\n'),
  ("\tr = if android_close (fd) == 0 || *errno () == EINTR { 0 } else { -1 };\n",
   "before",
   '\tcl_probe (c"CE".as_ptr (), fd, fcntl (fd, F_GETFD, 0), 0);\n'),
]


def main():
    s = SRC.read_text()
    if "--remove" in sys.argv:
        s = s.replace(HELPER, "", 1)
        for anchor, where, probe in reversed(SITES):
            joined = probe if where == "replace" else probe + anchor
            if joined not in s:
                raise SystemExit("probe missing: %r" % probe[:40])
            s = s.replace(joined, anchor, 1)
        SRC.write_text(s)
        print("removed; occurrences left:", s.count("cl_probe"))
        return
    if "cl_probe" in s:
        raise SystemExit("already inserted")
    i = s.index("pub unsafe extern \"C\" fn emacs_pipe")
    # The function's `#[unsafe(no_mangle)]` is the line above it: inserting between the two would
    # attach the attribute to the probe and drop the export (measured: dlopen then fails with
    # `cannot locate symbol "emacs_pipe"`, status 4.846).
    a = s.rfind("#[unsafe(no_mangle)]", 0, i)
    prev = s.rfind("\n", 0, a) + 1
    s = s[:prev] + HELPER + s[prev:]
    for anchor, where, probe in SITES:
        if s.count(anchor) != 1:
            raise SystemExit("anchor count %d: %r" % (s.count(anchor), anchor[:50]))
        s = s.replace(anchor, probe if where == "replace" else probe + anchor, 1)
    SRC.write_text(s)
    print("inserted; cl_probe occurrences:", s.count("cl_probe"))


if __name__ == "__main__":
    main()
