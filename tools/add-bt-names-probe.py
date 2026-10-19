#!/usr/bin/env python3
"""Insert (or remove) a name-resolving copy of the fatal-signal backtrace in femacs/src/sysdep.rs.

`backtrace_symbols_fd` writes to stderr, which an Android app process does not have, and its
addresses alone cannot be turned back into symbols without the module's load base.  This probe
asks `dladdr` for each frame's symbol and logs `name+offset` on logcat, so one device run names
the frame that faulted instead of narrowing it one statement at a time.

    tools/add-bt-names-probe.py            # insert
    tools/add-bt-names-probe.py --remove   # take it out again (byte-identical round trip)
"""
import pathlib
import sys

SRC = pathlib.Path (__file__).resolve ().parent.parent / "femacs/src/sysdep.rs"

HELPER = '''/* TEMPORARY probe (tools/add-bt-names-probe.py): the fatal backtrace with symbol names.  */
unsafe fn bt_names_probe (buffer: *mut *mut c_void, npointers: c_int)
{
  #[repr(C)]
  struct BtDlInfo {
    fname: *const c_char,
    fbase: *mut c_void,
    sname: *const c_char,
    saddr: *mut c_void,
  }
  unsafe extern "C" {
    fn __android_log_print (prio: c_int, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
    #[link_name = "dladdr"] fn dladdr (addr: *const c_void, info: *mut BtDlInfo) -> c_int;
  }
  let mut i: c_int = 0;
  while i < npointers && i < 24
    {
      let mut info: BtDlInfo = std::mem::zeroed ();
      let p: *mut c_void = *buffer.offset (i as isize);
      if dladdr (p as *const c_void, &raw mut info) != 0 && ! info.sname.is_null ()
        {
          __android_log_print (6, c"femacs-bt".as_ptr (), c"bt[%d] %s+0x%lx".as_ptr (), i,
                               info.sname, (p as isize - info.saddr as isize) as c_ulong);
        }
      else
        {
          __android_log_print (6, c"femacs-bt".as_ptr (), c"bt[%d] %p".as_ptr (), i, p);
        }
      i += 1;
    }
}

'''

ANCHOR = '      emacs_write (STDERR_FILENO, c"Backtrace:\\n".as_ptr () as *const c_void, 11);'
PROBE = "      bt_names_probe (buffer, npointers);\n"


def main():
    s = SRC.read_text()
    if "--remove" in sys.argv:
        s = s.replace(HELPER, "", 1)
        s = s.replace(PROBE + ANCHOR, ANCHOR, 1)
        SRC.write_text(s)
        print("removed; occurrences left:", s.count("bt_names_probe"))
        return
    if "bt_names_probe" in s:
        raise SystemExit("already inserted")
    if s.count(ANCHOR) != 1:
        raise SystemExit("anchor is not unique")
    # the helper is an item: it goes above `emacs_backtrace` itself, not inside its body
    sig = "pub unsafe extern \"C\" fn emacs_backtrace (backtrace_limit: c_int)"
    if s.count(sig) != 1:
        raise SystemExit("emacs_backtrace signature is not unique")
    i = s.index(sig)
    prev_start = s.rfind("\n", 0, i) + 1
    while prev_start > 0 and s[s.rfind("\n", 0, prev_start - 1) + 1:prev_start].startswith("#["):
        prev_start = s.rfind("\n", 0, prev_start - 1) + 1
    s = s[:prev_start] + HELPER + s[prev_start:]
    j = s.index(ANCHOR)
    s = s[:j] + PROBE + s[j:]
    SRC.write_text(s)
    print("inserted; bt_names_probe occurrences:", s.count("bt_names_probe"))


main()
