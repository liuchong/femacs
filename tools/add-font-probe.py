#!/usr/bin/env python3
"""Insert (or remove) the temporary font-enumeration probe in femacs/src/sfntfont.rs.

Why it exists: on the Android device every font candidate fails to register ("No suitable font
was found"), and the return value of `sfnt_enum_font` is ignored by its caller, so the four numbers
that tell the cases apart have to be written out of the process:

    fd        -1  => the open failed (VFS / android_open)
    tabledir  NULL and fd >= 0 => reading the table directory failed
    enum_rc   1 with bail N => `sfnt_enum_font_1` bailed at its Nth step
    enum_rc   0 with no fonts => the record registered but never matched

The probe writes straight to fd 2 because stdio's buffer and liblog's socket both lose their last
line when the process dies.  It keeps the unit under the line-drift allowance (`0.23` for
sfntfont) and *must* be inserted above the `#[cfg] #[unsafe(no_mangle)]` attributes - putting it
between them and the function makes the inventory gate report that the C symbol is missing (both
mistakes were made once, in status 4.799).

    tools/add-font-probe.py            # insert
    tools/add-font-probe.py --remove   # take it out again
"""
import pathlib
import sys

SRC = pathlib.Path(__file__).resolve().parent.parent / "femacs/src/sfntfont.rs"

HELPER = '''/* TEMPORARY font probe (tools/add-font-probe.py): see that file for the four readings.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_probe (tag: *const c_char, v: isize)
{
  let mut d = [0u8; 24]; let mut k = 0usize; let mut x = v; let neg = x < 0;
  if neg { x = -x; }
  if x == 0 { d[0] = b'0'; k = 1; }
  while x > 0 { d[k] = b'0' + (x % 10) as u8; x /= 10; k += 1; }
  write (2, c"SFNT ".as_ptr () as *const c_void, 5);
  write (2, tag as *const c_void, strlen (tag));
  write (2, c" ".as_ptr () as *const c_void, 1);
  if neg { write (2, c"-".as_ptr () as *const c_void, 1); }
  while k > 0 { k -= 1; write (2, d.as_ptr ().add (k) as *const c_void, 1); }
  write (2, c"\\n".as_ptr () as *const c_void, 1);
}

'''

CALLS = [
    ("      /* Read the table directory.  */",
     '      #[cfg (all (feature = "have_android", feature = "not_android_stubify"))]\n'
     '      { sfnt_probe (c"fd".as_ptr (), fd as isize); }\n\n'
     '      /* Read the table directory.  */'),
    ("      subtables = sfnt_read_table_directory (fd);",
     '      subtables = sfnt_read_table_directory (fd);\n'
     '      #[cfg (all (feature = "have_android", feature = "not_android_stubify"))]\n'
     '      { sfnt_probe (c"tabledir".as_ptr (), subtables as isize); }'),
    ("  rc = sfnt_enum_font_1 (fd, file, subtables, 0);",
     '  rc = sfnt_enum_font_1 (fd, file, subtables, 0);\n'
     '  #[cfg (all (feature = "have_android", feature = "not_android_stubify"))]\n'
     '  { sfnt_probe (c"enum_rc".as_ptr (), rc as isize); }'),
]


def main():
    text = SRC.read_text()
    remove = "--remove" in sys.argv
    if remove:
        # The file carries no other local edit, so the simplest reliable removal is to restore it
        # from the index - a textual "un-insert" once deleted the function's own two attributes
        # instead (status 4.799).
        import subprocess
        r = subprocess.run(["git", "checkout", "--", str(SRC)], cwd=str(SRC.parent.parent.parent))
        print("add-font-probe: removed (restored from the index)" if r.returncode == 0
              else "add-font-probe: git checkout failed")
        return r.returncode
    if "TEMPORARY font probe" in text:
        print("add-font-probe: already inserted")
        return 0
    anchor = 'pub unsafe extern "C" fn sfnt_enum_font (file: *const c_char) -> c_int'
    i = text.index(anchor)
    # the helper goes above the function's own attributes
    j = text.rindex('#[unsafe(no_mangle)]', 0, i)
    j = text.rindex('#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]', 0, j)
    text = text[:j] + HELPER + text[j:]
    for old, new in CALLS:
        if old not in text:
            print("add-font-probe: anchor not found: %r" % old, file=sys.stderr)
            return 1
        text = text.replace(old, new, 1)
    # `write` is already in scope: the unit includes lib/include/unistd.in.rs, which declares it.
    SRC.write_text(text)
    print("add-font-probe: inserted")
    return 0


if __name__ == "__main__":
    sys.exit(main())
