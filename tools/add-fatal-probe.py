#!/usr/bin/env python3
"""Insert (or remove) a probe at the top of `terminate_due_to_signal` in femacs/src/emacs.rs.

4.831's device runs die with SIGSEGV and *no* output at all.  C's fatal path explains why that is
possible: `terminate_due_to_signal` runs `shut_down_emacs` (which runs Lisp) *before* it calls
`emacs_backtrace`, so a crash that has already corrupted memory can die again inside the orderly
shutdown and never reach the backtrace.  This probe takes the backtrace first:

    FP <tid> sig <signal>   the fatal handler was entered, with the durable log entry to prove it
    (then `emacs_backtrace (40)`, whose `dladdr` copy logs `femacs-bt` lines on logcat)

    tools/add-fatal-probe.py            # insert
    tools/add-fatal-probe.py --remove   # take it out again (byte-identical round trip)
"""
import pathlib
import sys

SRC = pathlib.Path (__file__).resolve ().parent.parent / "femacs/src/emacs.rs"

HELPER = '''/* TEMPORARY fatal-path probe (tools/add-fatal-probe.py).  */
unsafe fn fp_probe (sig: c_int)
{
  unsafe extern "C" {
    #[link_name = "openat"] fn c_openat (dirfd: c_int, path: *const c_char,
                                         flags: c_int, mode: c_int) -> c_int;
    #[link_name = "write"] fn c_write (fd: c_int, buf: *const c_void, n: usize) -> isize;
    #[link_name = "close"] fn c_close (fd: c_int) -> c_int;
    #[link_name = "gettid"] fn c_gettid () -> c_int;
  }
  let fd = c_openat (-100, c"/data/data/org.femacs/files/probe.log".as_ptr (), 1 | 64 | 1024, 0o666);
  if fd < 0 { return; }
  c_write (fd, c"FP ".as_ptr () as *const c_void, 3);
  let tid: c_int = c_gettid ();
  for v in [tid as isize, sig as isize] {
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

SIG = 'pub unsafe extern "C" fn terminate_due_to_signal (sig: c_int, backtrace_limit: c_int) -> !\n{\n'
PROBE = '''  fp_probe (sig);
  emacs_backtrace (40);

'''


def main():
    s = SRC.read_text()
    if "--remove" in sys.argv:
        s = s.replace(HELPER, "", 1)
        s = s.replace(SIG + PROBE, SIG, 1)
        SRC.write_text(s)
        print("removed; occurrences left:", s.count("fp_probe"))
        return
    if "fp_probe" in s:
        raise SystemExit("already inserted")
    if s.count(SIG) != 1:
        raise SystemExit("terminate_due_to_signal's signature is not unique")
    i = s.index(SIG)
    prev = s.rfind("\n", 0, i) + 1
    while prev > 0:
        start = s.rfind("\n", 0, prev - 1) + 1
        if not s[start:prev].startswith("#["):
            break
        prev = start
    s = s[:prev] + HELPER + s[prev:]
    s = s.replace(SIG, SIG + PROBE, 1)
    SRC.write_text(s)
    print("inserted; fp_probe occurrences:", s.count("fp_probe"))


main()
