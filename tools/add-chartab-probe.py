#!/usr/bin/env python3
"""Insert (or remove) the temporary char-table probe in femacs/src/chartab.rs.

Companion of tools/add-fontset-probe.py: that one showed the device dying inside
`Fset_char_table_range (fontset, (160 . 591), value)` - i.e. in `char_table_set_range`.  This probe
follows the two functions that write the entries, so the last line on disk is the loop index that
never came back:

    csr  from to     `char_table_set_range` entered
    csi  i c         one iteration of its loop
    cse  i lim       the loop left, before the ASCII fix-up
    scsr depth minc  `sub_char_table_set_range` entered (its own depth and min_char)
    scs  i c         one iteration of its loop

    tools/add-chartab-probe.py            # insert
    tools/add-chartab-probe.py --remove   # take it out again (byte-identical round trip)
"""
import pathlib
import sys

SRC = pathlib.Path (__file__).resolve ().parent.parent / "femacs/src/chartab.rs"

HELPER = '''/* TEMPORARY char-table probe (tools/add-chartab-probe.py).  */
#[cfg (feature = "have_android")]
unsafe fn cb_probe (tag: *const c_char, a: isize, b: isize)
{
  unsafe extern "C" {
    #[link_name = "openat"] fn c_openat (dirfd: c_int, path: *const c_char,
                                         flags: c_int, mode: c_int) -> c_int;
    #[link_name = "write"] fn c_write (fd: c_int, buf: *const c_void, n: usize) -> isize;
    #[link_name = "close"] fn c_close (fd: c_int) -> c_int;
    #[link_name = "gettid"] fn c_gettid () -> c_int;
  }
  let mut t: usize = 0;
  while *tag.add (t) != 0 { t += 1; }
  let fd = c_openat (-100, c"/data/data/org.femacs/files/probe.log".as_ptr (), 1 | 64 | 1024, 0o666);
  if fd < 0 { return; }
  c_write (fd, c"CB ".as_ptr () as *const c_void, 3);
  let tid: c_int = c_gettid ();
  {
    let mut td = [0u8; 12]; let mut tk = 0usize; let mut tx: isize = tid as isize;
    if tx == 0 { td[0] = b'0'; tk = 1; }
    while tx > 0 { td[tk] = b'0' + (tx % 10) as u8; tx /= 10; tk += 1; }
    while tk > 0 { tk -= 1; c_write (fd, td.as_ptr ().add (tk) as *const c_void, 1); }
  }
  c_write (fd, c" ".as_ptr () as *const c_void, 1);
  c_write (fd, tag as *const c_void, t);
  for v in [a, b] {
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

SIG_CSR = ('#[unsafe(no_mangle)]\n'
           'pub unsafe extern "C" fn char_table_set_range (table: Lisp_Object, from: c_int, to: c_int,')
SIG_SUB = 'unsafe fn sub_char_table_set_range (table: Lisp_Object, from: c_int, to: c_int,'

# The helper goes above this anchor, so the attribute stays glued to its function: putting
# anything between them drops the symbol from the shared object and `dlopen` then fails with
# `cannot locate symbol "char_table_set_range"` (measured 2026-10-19).
ANCHOR = SIG_CSR

# (function signature, anchor after which the probe goes, probe line)
SITES = [
  (SIG_CSR, '  let tbl: *mut Lisp_Char_Table = XCHAR_TABLE (table);\n',
   '\n  cb_probe (c"csr".as_ptr (), from as isize, to as isize);'),
  (SIG_CSR, '      while i <= lim\n\t{\n',
   '\t  cb_probe (c"csi".as_ptr (), i as isize, c as isize);\n'),
  (SIG_CSR, '\t  i += 1;\n\t  c += chartab_chars[0];\n\t}\n',
   '\t  cb_probe (c"cse".as_ptr (), i as isize, lim as isize);\n'),
  (SIG_SUB, '  let lim: c_int = chartab_size[depth as usize];\n',
   '\n  cb_probe (c"scsr".as_ptr (), depth as isize, min_char as isize);'),
  (SIG_SUB, '  while i < lim\n    {\n',
   '      cb_probe (c"scs".as_ptr (), i as isize, c as isize);\n'),
]


# `set-char-table-range`'s own body: the device showed the fontset macro reaching its call
# without `char_table_set_range` ever being entered, so the branches in between get their own
# tags.  Each pair is (text that must be unique, the text that replaces it).
REPLACES = [
  ("       (char_table: Lisp_Object, range: Lisp_Object, value: Lisp_Object),\n       {\n",
   "       (char_table: Lisp_Object, range: Lisp_Object, value: Lisp_Object),\n       {\n"
   "  cb_probe (c\"fr0\".as_ptr (), 0, 0);\n"),
  ("  CHECK_CHAR_TABLE (char_table);\n  if EQ (range, Qt ())",
   "  CHECK_CHAR_TABLE (char_table);\n  cb_probe (c\"fr1\".as_ptr (), 0, 0);\n"
   "  if EQ (range, Qt ())"),
  ("  else if CONSP (range)\n    {\n      CHECK_CHARACTER_CAR (range);",
   "  else if CONSP (range)\n    {\n      cb_probe (c\"fr2\".as_ptr (), 0, 0);\n"
   "      CHECK_CHARACTER_CAR (range);"),
  ("      CHECK_CHARACTER_CDR (range);\n      char_table_set_range (char_table,",
   "      CHECK_CHARACTER_CDR (range);\n      cb_probe (c\"fr3\".as_ptr (), 0, 0);\n"
   "      char_table_set_range (char_table,"),
]


def apply_replaces(s, remove):
    for old, new in REPLACES:
        if remove:
            if new not in s:
                raise SystemExit(f"replacement text is not in the file: {new[:40]!r}")
            s = s.replace(new, old, 1)
        else:
            if s.count(old) != 1:
                raise SystemExit(f"unique-text count {s.count(old)}: {old[:40]!r}")
            s = s.replace(old, new, 1)
    return s


def apply_sites(s, remove):
    for sig, anchor, probe in SITES:
        n = s.count(sig)
        if n != 1:
            raise SystemExit(f"signature count {n}: {sig[:50]!r}")
        base = s.index(sig)
        at = s.index(anchor, base)
        if remove:
            target = anchor + probe
            if s[at:at + len(target)] != target:
                raise SystemExit(f"probe not where it was inserted: {probe[:30]!r}")
            s = s[:at] + anchor + s[at + len(target):]
        else:
            s = s[:at] + anchor + probe + s[at + len(anchor):]
    return s


def one(s, old):
    n = s.count(old)
    if n != 1:
        raise SystemExit(f"anchor count {n}: {old[:60]!r}")
    return s

def main():
    s = SRC.read_text()
    if "--remove" in sys.argv:
        s = s.replace(HELPER, "", 1)
        s = apply_sites(s, True)
        s = apply_replaces(s, True)
        SRC.write_text(s)
        print("removed; occurrences left:", s.count("cb_probe"))
        return
    if "cb_probe" in s:
        raise SystemExit("already inserted")
    if s.count(ANCHOR) != 1:
        raise SystemExit("signature anchor is not unique")
    s = s.replace(ANCHOR, HELPER + ANCHOR, 1)
    s = apply_sites(s, False)
    s = apply_replaces(s, False)
    SRC.write_text(s)
    print("inserted; cb_probe occurrences:", s.count("cb_probe"))

main()
