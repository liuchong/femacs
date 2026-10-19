#!/usr/bin/env python3
"""Insert (or remove) the temporary `sfnt_read_table_directory` probe.

`status 4.804`/4.805: on the device `fd` is always valid (109) but the table directory comes back
NULL for 90 of 166 font candidates and a real pointer for the other 76, where `sfnt_enum_font_1`
then bails anyway.  NULL has two possible origins in C 249-323 - the 12-byte header read, or the
`num_tables * sizeof (struct sfnt_table_directory)` read - and the difference matters:

    failA rc < 12        => the first read failed (a VFS/read problem)
    failA with rc == 12  => cannot happen
    failB rc < want      => the header parsed but the directory read came short
                            (a wrong `num_tables`, i.e. a byte-swap or layout defect)

`nt` prints `num_tables` and `scaler_type` as the port parsed them, and `ok` prints the first eight
successful parses for comparison, so the two sets can be told apart without a second run.  Every
probe writes the name too, so the failing set can be correlated with the file extension.

    python3 tools/add-sfnt-table-probe.py            # insert
    python3 tools/add-sfnt-table-probe.py --remove   # restore from the saved copy
"""
import pathlib, shutil, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SFNT = ROOT / "femacs/src/sfnt.rs"
FONT = ROOT / "femacs/src/sfntfont.rs"
BAK = pathlib.Path("/tmp/sfnt-table-probe-backups")

HELPER = '''/* TEMPORARY probe (tools/add-sfnt-table-probe.py).  */
unsafe fn sfnt_probe2 (tag: *const c_char, a: isize, b: isize)
{
  let mut d = [0u8; 24];
  let mut put = |v: isize| {
    let mut k = 0usize; let mut x = v; let neg = x < 0;
    if neg { x = -x; }
    if x == 0 { d[0] = b'0'; k = 1; }
    while x > 0 { d[k] = b'0' + (x % 10) as u8; x /= 10; k += 1; }
    if neg { write (2, c"-".as_ptr () as *const c_void, 1); }
    while k > 0 { k -= 1; write (2, d.as_ptr ().add (k) as *const c_void, 1); }
  };
  write (2, c"SFNTT ".as_ptr () as *const c_void, 6);
  write (2, tag as *const c_void, strlen (tag));
  write (2, c" ".as_ptr () as *const c_void, 1);
  put (a);
  write (2, c" ".as_ptr () as *const c_void, 1);
  put (b);
  write (2, c"\\n".as_ptr () as *const c_void, 1);
}

unsafe fn sfnt_probe_s (tag: *const c_char, s: *const c_char)
{
  write (2, c"SFNTT ".as_ptr () as *const c_void, 6);
  write (2, tag as *const c_void, strlen (tag));
  write (2, c" ".as_ptr () as *const c_void, 1);
  if !s.is_null () { write (2, s as *const c_void, strlen (s)); }
  write (2, c"\\n".as_ptr () as *const c_void, 1);
}

'''

EDITS_SFNT = [
    ("  rc = read (fd, subtable as *mut c_void, offset);\n",
     "  rc = read (fd, subtable as *mut c_void, offset);\n"
     "  sfnt_probe2 (c\"rc1\".as_ptr (), rc as isize, offset as isize);\n"),
    ("      if rc != -1 && (rc as size_t) >= std::mem::size_of::<uint32_t> ()\n",
     "      sfnt_probe2 (c\"failA\".as_ptr (), rc as isize, offset as isize);\n"
     "      if rc != -1 && (rc as size_t) >= std::mem::size_of::<uint32_t> ()\n"),
    ("  if rc == -1 || (rc as size_t) < subtable_size\n    {\n      xfree (subtable as *mut c_void);\n"
     "      return std::ptr::null_mut ();\n    }\n\n  /* Swap each of the subtables.  */\n",
     "  if rc == -1 || (rc as size_t) < subtable_size\n    {\n"
     "      sfnt_probe2 (c\"failB\".as_ptr (), rc as isize, subtable_size as isize);\n"
     "      sfnt_probe2 (c\"nt\".as_ptr (), (*subtable).num_tables as isize,\n"
     "		   (*subtable).scaler_type as isize);\n"
     "      xfree (subtable as *mut c_void);\n      return std::ptr::null_mut ();\n    }\n\n"
     "  static mut probe_ok: c_int = 0;\n"
     "  if probe_ok < 8\n    {\n      probe_ok += 1;\n"
     "      sfnt_probe2 (c\"ok\".as_ptr (), (*subtable).num_tables as isize,\n"
     "		   (*subtable).scaler_type as isize);\n    }\n\n  /* Swap each of the subtables.  */\n"),
]

EDITS_FONT = [
    ("      { sfnt_probe (c\"fd\".as_ptr (), fd as isize); }\n",
     "      { sfnt_probe (c\"fd\".as_ptr (), fd as isize); }\n"
     "      sfnt_probe_s (c\"file\".as_ptr (), file);\n"),
]


def apply(path, edits, insert_helper_before):
    text = path.read_text()
    for old, new in edits:
        if text.count(old) != 1:
            raise SystemExit("anchor appears %d times in %s: %r" % (text.count(old), path, old[:60]))
        text = text.replace(old, new, 1)
    # The helper must land *above* the item's own attributes: inserted between
    # `#[unsafe(no_mangle)]` and the `fn`, the attribute is orphaned, the C symbol stops being
    # exported, and the device reports
    #   dlopen failed: cannot locate symbol "sfnt_read_table_directory" ...
    # on the next launch (this tool's first run, status 4.805; the same trap is recorded in
    # tools/add-font-probe.py).  Walk up over the contiguous `#[...]` lines.
    i = text.index(insert_helper_before)
    # `i` is the start of the item's own line; step back line by line over the contiguous
    # attribute lines above it.
    while i > 0:
        prev_end = i - 1
        prev_start = text.rfind(chr(10), 0, prev_end) + 1
        if not text[prev_start:prev_end].startswith("#["):
            break
        i = prev_start
    text = text[:i] + HELPER + text[i:]
    path.write_text(text)


def main():
    if "--remove" in sys.argv:
        for p in (SFNT, FONT):
            b = BAK / p.name
            if b.is_file():
                shutil.copyfile(b, p)
                b.unlink()
                print("restored %s" % p)
            else:
                print("no backup for %s" % p)
        return 0
    BAK.mkdir(parents=True, exist_ok=True)
    for p in (SFNT, FONT):
        shutil.copyfile(p, BAK / p.name)
    apply(SFNT, EDITS_SFNT, "pub unsafe extern \"C\" fn sfnt_read_table_directory")
    if "sfnt_probe (c\"fd\"" in FONT.read_text():
        text = FONT.read_text()
        for old, new in EDITS_FONT:
            if text.count(old) != 1:
                raise SystemExit("font anchor appears %d times" % text.count(old))
            text = text.replace(old, new, 1)
        FONT.write_text(text)
        print("probed %s (fd/file lines)" % FONT)
    else:
        print("%s has no fd probe; run tools/add-font-probe.py first for the file name" % FONT)
    print("probed %s" % SFNT)
    return 0


if __name__ == "__main__":
    sys.exit(main())
