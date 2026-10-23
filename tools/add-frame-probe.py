#!/usr/bin/env python3
"""Insert (or remove) three stage probes on the first-frame path.

`status 4.808` proved `sfntfont_open` now runs to completion on the device, so the remaining
SIGSEGV is somewhere after it: either inside the frame's creation or in the first redisplay.  Three
tags separate those decades of code in one run:

    x-create-frame   `Fx_create_frame` entered (androidfns.c 738+)
    draw             `sfntfont_draw` entered (sfntfont.c 3676+)
    put-glyphs       `sfntfont_android_put_glyphs` entered (sfntfont-android.c 402+)

**Restores come from a saved working copy, never from the git index**: withdrawing a probe with
`git show :path` once dropped uncommitted work, because the index was older than the working tree
(status 4.808, lesson recorded).

    python3 tools/add-frame-probe.py            # insert
    python3 tools/add-frame-probe.py --remove   # restore the saved copies
"""
import pathlib, shutil, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
BAK = pathlib.Path("/tmp/frame-probe-backups")

HELPER = '''/* TEMPORARY probe (tools/add-frame-probe.py).  */
unsafe fn frame_probe (tag: *const core::ffi::c_char)
{
  unsafe extern "C" {
    fn __android_log_print (prio: core::ffi::c_int, tag: *const core::ffi::c_char,
                            fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
  }
  __android_log_print (6, c"femacs-frame".as_ptr (), c"%s".as_ptr (), tag);
}

'''

EDITS = [
    ("src/androidfns.rs",
     "  let mut f: *mut frame;\n",
     "  let mut f: *mut frame;\n  frame_probe (c\"x-create-frame\".as_ptr ());\n",
     "DEFUN! (\"x-create-frame\", Fx_create_frame, Sx_create_frame,"),
    ("src/sfntfont.rs",
     "  let mut metrics: sfnt_glyph_metrics = std::mem::zeroed ();\n",
     "  let mut metrics: sfnt_glyph_metrics = std::mem::zeroed ();\n"
     "  frame_probe (c\"draw\".as_ptr ());\n",
     "pub unsafe extern \"C\" fn sfntfont_draw (s: *mut glyph_string"),
    ("src/sfntfont-android.rs",
     "  let s: *mut glyph_string = s as *mut glyph_string;\n",
     "  let s: *mut glyph_string = s as *mut glyph_string;\n"
     "  frame_probe (c\"put-glyphs\".as_ptr ());\n",
     "unsafe extern \"C\" fn sfntfont_android_put_glyphs (s: *mut c_void"),
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
        for rel, _, _, _ in EDITS:
            p = ROOT / rel
            b = BAK / p.name
            if b.is_file():
                shutil.copyfile(b, p)
                b.unlink()
                print("restored %s (from the saved working copy)" % rel)
            else:
                print("no backup for %s" % rel)
        return 0
    BAK.mkdir(parents=True, exist_ok=True)
    status = 0
    for rel, old, new, fn_anchor in EDITS:
        p = ROOT / rel
        text = p.read_text()
        if new in text:
            if "TEMPORARY probe (tools/add-frame-probe.py)" not in text:
                if not (BAK / p.name).is_file():
                    shutil.copyfile(p, BAK / p.name)
                text = insert_above_attributes(text, fn_anchor, HELPER)
                p.write_text(text)
                print("%s: helper added" % rel)
            else:
                print("%s: already probed" % rel)
            continue
        if text.count(old) != 1:
            print("%s: anchor appears %d times, skipped" % (rel, text.count(old)))
            status = 1
            continue
        if not (BAK / p.name).is_file():
            shutil.copyfile(p, BAK / p.name)
        text = text.replace(old, new, 1)
        if "TEMPORARY probe (tools/add-frame-probe.py)" not in text:
            text = insert_above_attributes(text, fn_anchor, HELPER)
        p.write_text(text)
        print("probed %s" % rel)
    return status


if __name__ == "__main__":
    sys.exit(main())
