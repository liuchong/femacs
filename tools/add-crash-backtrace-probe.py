#!/usr/bin/env python3
"""Insert (or remove) a logcat copy of the reference's own crash backtrace.

`emacs_backtrace` already does what C does - `backtrace` then `backtrace_symbols_fd` on
`STDERR_FILENO` - but an Android app process has no stderr: the descriptor points at `/dev/null`,
so the crash that ends startup leaves nothing behind, and every diagnosis so far has had to be
guessed one probe at a time (status 4.808 reached `startup.el`'s `normal-top-level` and died with
SIGSEGV and no message at all).  This probe logs the same addresses to logcat, which is where the
script reads:

    python3 tools/add-crash-backtrace-probe.py            # insert
    python3 tools/add-crash-backtrace-probe.py --remove   # restore from the saved copy

It changes nothing about the C path: the `emacs_write`/`backtrace_symbols_fd` calls stay, and the
loop only mirrors the frame addresses.
"""
import pathlib, shutil, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "femacs/src/sysdep.rs"
BAK = pathlib.Path("/tmp/crash-backtrace-probe-backup.rs")

HELPER = '''/* TEMPORARY probe (tools/add-crash-backtrace-probe.py): the crash frames, on logcat.  */
unsafe fn bt_probe_log (buffer: *mut *mut core::ffi::c_void, npointers: core::ffi::c_int)
{
  unsafe extern "C" {
    fn __android_log_print (prio: core::ffi::c_int, tag: *const core::ffi::c_char,
                            fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
  }
  let mut i: core::ffi::c_int = 0;
  while i < npointers && i < 32
    {
      __android_log_print (6, c"femacs-bt".as_ptr (), c"bt[%d] %p".as_ptr (), i,
                           *buffer.offset (i as isize));
      i += 1;
    }
}

'''

ANCHOR = "      emacs_write (STDERR_FILENO, c\"Backtrace:\\n\".as_ptr () as *const c_void, 11);"
PROBE = ("      bt_probe_log (buffer, npointers);\n"
         "      emacs_write (STDERR_FILENO, c\"Backtrace:\\n\".as_ptr () as *const c_void, 11);\n")


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
        if BAK.is_file():
            shutil.copyfile(BAK, SRC)
            BAK.unlink()
            print("restored %s" % SRC)
        else:
            print("no backup at %s" % BAK)
        return 0
    text = SRC.read_text()
    if PROBE in text:
        print("already probed")
        return 0
    if text.count(ANCHOR) != 1:
        raise SystemExit("anchor appears %d times" % text.count(ANCHOR))
    shutil.copyfile(SRC, BAK)
    text = text.replace(ANCHOR, PROBE, 1)
    text = insert_above_attributes(text, "pub unsafe extern \"C\" fn emacs_backtrace", HELPER)
    SRC.write_text(text)
    print("probed %s" % SRC)
    return 0


if __name__ == "__main__":
    sys.exit(main())
