#!/usr/bin/env python3
"""Print what `Fandroid_enumerate_fonts` actually sees, for one device run.

`status 4.803` fixed the unit's own `struct dirent` (it was the host's 1048-byte record; bionic's is
280 with `d_name` at 19), and the built object does load the name from `+19` (checked by
disassembling `sfntfont-android.aarch64-linux-android.o`).  The device still registers no font, so
the next question is not the record but the scan itself: is `opendir` failing, is `readdir`
returning nothing, or do the names simply not match?  Those three answers are indistinguishable in
the release log, which prints one line per directory.

    python3 tools/add-font-enum-probe.py      # insert the probe
    python3 tools/add-font-enum-probe.py --remove

Unlike `add-font-probe.py` this unit is dirty in the working tree (the 4.803 record fix), so the
undo path is a saved copy, never `git checkout --`.
"""
import pathlib, shutil, sys

SRC = pathlib.Path("src/sfntfont-android.rs")
BAK = pathlib.Path("/tmp/sfntfont-android.rs.probe-backup")

OPEN_ANCHOR = "      dir = opendir (system_font_directories[i as usize]);\n"
LOOP_ANCHOR = "      loop\n\t{\n\t  dirent = readdir (dir) as *mut dirent;\n"
COUNTER_ANCHOR = "      loop\n"
CLOSE_ANCHOR = "      closedir (dir);\n"

OPEN_PROBE = OPEN_ANCHOR + """      /* femacsprobe: the directory handle, before C's `if (!dir) continue;`.  */
      __android_log_print (ANDROID_LOG_VERBOSE, c"femacsprobe".as_ptr (),
			   c"probe dir=%p".as_ptr (), dir);
"""
LOOP_PROBE = LOOP_ANCHOR + """	  /* femacsprobe: every entry `readdir` hands back, matched or not.  */
	  __android_log_print (ANDROID_LOG_VERBOSE, c"femacsprobe".as_ptr (),
			       c"probe entry=%s".as_ptr (), (*dirent).d_name.as_ptr ());
	  probe_entries += 1;
"""
MATCH_ANCHOR = """	      sprintf (name.as_mut_ptr (), c"%s/%s".as_ptr (),
		       system_font_directories[i as usize], (*dirent).d_name.as_ptr ());
"""
MATCH_PROBE = """	      /* femacsprobe: the name the branch is about to hand to `sfnt_enum_font`.  */
	      __android_log_print (ANDROID_LOG_VERBOSE, c"femacsprobe".as_ptr (),
				   c"probe enumerating %s".as_ptr (), (*dirent).d_name.as_ptr ());
""" + MATCH_ANCHOR
CLOSE_PROBE = """      /* femacsprobe: how many entries this directory yielded.  */
      __android_log_print (ANDROID_LOG_VERBOSE, c"femacsprobe".as_ptr (),
			   c"probe entries=%d".as_ptr (), probe_entries);
""" + CLOSE_ANCHOR


def main():
    if "--remove" in sys.argv:
        if not BAK.is_file():
            return "no backup at %s; nothing to restore" % BAK
        shutil.copyfile(BAK, SRC)
        BAK.unlink()
        print("restored %s from %s" % (SRC, BAK))
        return 0
    text = SRC.read_text()
    if "femacsprobe" in text:
        return "already probed; run --remove first"
    for anchor in (OPEN_ANCHOR, LOOP_ANCHOR, COUNTER_ANCHOR, MATCH_ANCHOR, CLOSE_ANCHOR):
        if text.count(anchor) != 1:
            return "anchor appears %d times: %r" % (text.count(anchor), anchor)
    shutil.copyfile(SRC, BAK)
    text = text.replace(OPEN_ANCHOR, OPEN_PROBE, 1)
    text = text.replace(LOOP_ANCHOR, LOOP_PROBE, 1)
    text = text.replace(COUNTER_ANCHOR, "      let mut probe_entries: c_int = 0;\n" + COUNTER_ANCHOR, 1)
    text = text.replace(MATCH_ANCHOR, MATCH_PROBE, 1)
    text = text.replace(CLOSE_ANCHOR, CLOSE_PROBE, 1)
    SRC.write_text(text)
    print("probed %s (%d lines); backup at %s" % (SRC, text.count("\n"), BAK))
    return 0


if __name__ == "__main__":
    sys.exit(main() or 0)
