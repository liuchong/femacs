#!/usr/bin/env python3
"""Insert (or remove) milestone probes inside `sfntfont_open`, which is where startup dies.

`status 4.808`: with the byte order fixed, fonts register, `sfntfont_list` and `sfntfont_open`
both run, and the process then dies with SIGSEGV and no output - and the reference's own backtrace cannot
be used, because `terminate_due_to_signal` only calls `emacs_backtrace` when
`attempt_orderly_shutdown_on_fatal_signal` is set (Android clears it, as C does) and an app
process has no stderr anyway.  So the function is bisected with tags instead:

    open-1  `sfnt_reference_font_tables` returned (C 3244)
    open-2  `sfnt_get_scale` returned (C 3267)
    open-3  the font's type was set (C 3301)
    open-4  `sfnt_normalize_blend` returned (C 3371)

The last tag printed names the stage the crash is in.

    python3 tools/add-sfnt-open-probe.py            # insert
    python3 tools/add-sfnt-open-probe.py --remove   # restore from the saved copy
"""
import pathlib, shutil, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "src/sfntfont.rs"
BAK = pathlib.Path("/tmp/sfnt-open-probe-backup.rs")

HELPER = '''/* TEMPORARY probe (tools/add-sfnt-open-probe.py).  */
unsafe fn open_probe (tag: *const core::ffi::c_char)
{
  unsafe extern "C" { fn write (fd: core::ffi::c_int, buf: *const core::ffi::c_void,
                                n: usize) -> isize; }
  let mut name = [0u8; 24]; let mut n = 0usize;
  while *tag.add (n) != 0 && n < 23 { name[n] = *tag.add (n) as u8; n += 1; }
  write (2, c"OPENPB ".as_ptr () as *const core::ffi::c_void, 7);
  write (2, name.as_ptr () as *const core::ffi::c_void, n);
  write (2, c"\\n".as_ptr () as *const core::ffi::c_void, 1);
  unsafe extern "C" {
    fn __android_log_print (prio: core::ffi::c_int, tag: *const core::ffi::c_char,
                            fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
  }
  __android_log_print (6, c"femacs-open".as_ptr (), c"%.*s".as_ptr (), n as core::ffi::c_int,
                       name.as_ptr ());
}

'''

EDITS = [
    ("      tables = sfnt_reference_font_tables (desc);          // C 3244",
     "      tables = sfnt_reference_font_tables (desc);          // C 3244\n"
     "      open_probe (c\"open-1\".as_ptr ());"),
    ("      (*font_info).scale = sfnt_get_scale ((*font_info).head, pixel_size);   // C 3267",
     "      (*font_info).scale = sfnt_get_scale ((*font_info).head, pixel_size);   // C 3267\n"
     "      open_probe (c\"open-2\".as_ptr ());"),
    ("      ASET (font_object, FONT_TYPE_INDEX as isize, sfnt_vendor_name);   // C 3301",
     "      ASET (font_object, FONT_TYPE_INDEX as isize, sfnt_vendor_name);   // C 3301\n"
     "      open_probe (c\"open-3\".as_ptr ());"),
    ("	      sfnt_normalize_blend (&raw mut (*font_info).blend);   // C 3371",
     "	      sfnt_normalize_blend (&raw mut (*font_info).blend);   // C 3371\n"
     "	      open_probe (c\"open-4\".as_ptr ());"),
]


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
            print("no backup")
        return 0
    text = SRC.read_text()
    changed = 0
    for old, new in EDITS:
        if new in text:
            continue
        if text.count(old) != 1:
            raise SystemExit("anchor appears %d times: %r" % (text.count(old), old[:70]))
        if changed == 0:
            if BAK.is_file():
                print("warning: %s exists; keeping the older backup" % BAK)
            else:
                shutil.copyfile(SRC, BAK)
        text = text.replace(old, new, 1)
        changed += 1
    if "TEMPORARY probe (tools/add-sfnt-open-probe.py)" not in text:
        text = insert_above_attributes(text, "pub unsafe extern \"C\" fn sfntfont_open (f: *mut c_void",
                                       HELPER)
    SRC.write_text(text)
    print("probed %s (%d edit(s))" % (SRC, changed))
    return 0


if __name__ == "__main__":
    sys.exit(main())
