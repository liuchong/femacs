#!/usr/bin/env python3
"""Insert (or remove) the temporary fontset probe in src/fontset.rs.

Why it exists: on the Android device the process dies with signal 7 (SIGBUS) while
`set-fontset-font` is writing entries.  `logcat` loses the tail of a dying process, so the
probe opens `/data/data/org.femacs/files/probe.log` (`O_CREAT|O_APPEND`) and closes it on
every call: what is on disk is exactly what ran.  The tags separate the four cases:

    m  a b    the macro's branch: a=1 means `add` was non-nil, b=1 means `range` was a cons
    f1/f2     around `set_fontset_fallback`   (the `add` nil, `range` nil arm)
    c1/c2     around `Fset_char_table_range`  (the `add` nil, `range` cons arm)
    e0/enter  the first two statements of `fontset_add`; e0 carries the caller's stack address
    vec rng ref set fb done   `fontset_add`'s own statements
    call ret  the caller (`Fset_fontset_font`) around each `FONTSET_ADD!`

The helper *must* be inserted above `#[cfg] #[unsafe(no_mangle)]` attributes and never
between an `if` and its `{`.

    tools/add-fontset-probe.py            # insert
    tools/add-fontset-probe.py --remove   # take it out again (byte-identical round trip)
"""
import pathlib
import sys

SRC = pathlib.Path (__file__).resolve ().parent.parent / "src/fontset.rs"

HELPER = '''/* TEMPORARY fontset probe (tools/add-fontset-probe.py).  */
#[cfg (feature = "have_android")]
unsafe fn fa_probe (tag: *const c_char, a: isize, b: isize)
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
  c_write (fd, c"FA ".as_ptr () as *const c_void, 3);
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

ANCHOR_FN = '#[cfg (feature = "have_window_system")]\nunsafe fn fontset_add (fontset: Lisp_Object, range: Lisp_Object, elt: Lisp_Object, // C 335'

# (anchor, text inserted immediately after the anchor)
EDITS = [
  ("  let mut args: [Lisp_Object; 2] = [Qnil (), Qnil ()];",
   "\n  fa_probe (c\"e0\".as_ptr (), &raw const range as isize, &raw const elt as isize);"),
  ("  let mut idx: c_int = if EQ (add, Qappend ()) { 0 } else { 1 };",
   "\n  fa_probe (c\"enter\".as_ptr (), 0, 0);"),
  ("  args[(1 - idx) as usize] = make_vector (1, elt);",
   "\n  fa_probe (c\"vec\".as_ptr (), 0, 0);"),
  ("      let to: c_int = XFIXNUM (XCDR (range)) as c_int;",
   "\n      fa_probe (c\"rng\".as_ptr (), from as isize, to as isize);"),
  ("                                                       &raw mut to1);",
   "\n        fa_probe (c\"ref\".as_ptr (), from1 as isize, to1 as isize);"),
  ("                              else { CALLMANY! (Fvconcat, args) });",
   "\n        fa_probe (c\"set\".as_ptr (), from as isize, to1 as isize);"),
  ("      args[idx as usize] = FONTSET_FALLBACK! (fontset);",
   "\n      fa_probe (c\"fb\".as_ptr (), 0, 0);"),
]

TAIL_OLD = "                            else { CALLMANY! (Fvconcat, args) });\n    }\n}"
TAIL_NEW = "                            else { CALLMANY! (Fvconcat, args) });\n      fa_probe (c\"done\".as_ptr (), 0, 0);\n    }\n}"

LOOP_OLD = "      FONTSET_ADD! (fontset_obj, XCAR (range_list), font_def, add);"
LOOP_NEW = ("      fa_probe (c\"call\".as_ptr (), 0, 0);\n" + LOOP_OLD +
            "\n      fa_probe (c\"ret\".as_ptr (), 0, 0);")

MACRO_OLD = '''    if NILP ($add) {
      if NILP ($range) {
        set_fontset_fallback ($fontset, make_vector (1, $elt));
      } else {
        Fset_char_table_range ($fontset, $range, make_vector (1, $elt));
      }
    } else {
      fontset_add ($fontset, $range, $elt, $add);
    }'''

MACRO_NEW = '''    fa_probe (c"m".as_ptr (), if NILP ($add) { 0 } else { 1 },
              if NILP ($range) { 0 } else { 1 });
    if NILP ($add) {
      if NILP ($range) {
        fa_probe (c"f1".as_ptr (), 0, 0);
        set_fontset_fallback ($fontset, make_vector (1, $elt));
        fa_probe (c"f2".as_ptr (), 0, 0);
      } else {
        fa_probe (c"c1".as_ptr (), XFIXNUM (XCAR ($range)) as isize,
                  XFIXNUM (XCDR ($range)) as isize);
        let fa_val: Lisp_Object = make_vector (1, $elt);
        fa_probe (c"c1b".as_ptr (), 0, 0);
        Fset_char_table_range ($fontset, $range, fa_val);
        fa_probe (c"c2".as_ptr (), 0, 0);
      }
    } else {
      fontset_add ($fontset, $range, $elt, $add);
    }'''

def one(s, old):
    n = s.count(old)
    if n != 1:
        raise SystemExit(f"anchor count {n}: {old[:60]!r}")
    return s

def main():
    s = SRC.read_text()
    if "--remove" in sys.argv:
        s = s.replace(HELPER, "", 1)
        for old, new in EDITS:
            s = s.replace(old + new, old, 1)
        s = s.replace(TAIL_NEW, TAIL_OLD, 1)
        s = s.replace(LOOP_NEW, LOOP_OLD, 1)
        s = s.replace(MACRO_NEW, MACRO_OLD, 1)
        SRC.write_text(s)
        print("removed; occurrences left:", s.count("fa_probe"))
        return
    if "fa_probe" in s:
        raise SystemExit("already inserted")
    one(s, ANCHOR_FN)
    s = s.replace(ANCHOR_FN, HELPER + ANCHOR_FN, 1)
    for old, new in EDITS:
        one(s, old)
        s = s.replace(old, old + new, 1)
    one(s, TAIL_OLD)
    s = s.replace(TAIL_OLD, TAIL_NEW, 1)
    one(s, LOOP_OLD)
    s = s.replace(LOOP_OLD, LOOP_NEW, 1)
    one(s, MACRO_OLD)
    s = s.replace(MACRO_OLD, MACRO_NEW, 1)
    SRC.write_text(s)
    print("inserted; fa_probe occurrences:", s.count("fa_probe"))

main()
