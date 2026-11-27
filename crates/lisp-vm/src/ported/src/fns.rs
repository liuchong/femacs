// femacs: port of src/fns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, union, enum, macro, local), same control flow, same error model.  C's 104 `DEFUN`s use
// `DEFUN!` (PORTING.md 5.1); C's non-`static` definitions carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2) and the C `static` helpers stay plain `unsafe fn`.
//
// **Carried arms.**  `src/fns.c` is *not* in `docs/baseline/config/cond-eval-verified.txt`
// (`python3 tools/check-cond-eval.py src/fns.c` reports `13 conditions, 0 unknown,
// 1 disagreements [NOT VERIFIED]`).  The single disagreement is C 5383's
// `#if EMACS_INT_MAX > INT32_MAX`: clang answers True under the frozen `CFLAGS = -g3 -O2`,
// while the evaluator cannot see `EMACS_INT_MAX` (a `config.h`-dependent macro) and answers
// False.  That arm has no `#else`, so nothing is carried for it: the port compiles the arm
// clang compiles.
//
// Because the file is not in the verified list, `tools/cinventory.py`'s `blind_conditions`
// drops the macro table and keeps *both* arms of every `#if` live, which is exactly what the
// ledger demands (docs/status.md 3.49/3.51, PORTING.md P-51).  The arms this target's
// preprocessor does *not* compile are carried as `#[cfg (feature = "…")]` text naming the C
// condition's non-frozen side:
//
//   * C 40-42, 2978-2981: `#ifdef HAVE_TREE_SITTER` (`undef`) -> `have_tree_sitter`
//     (`treesit_node_eq`, `PVEC_TS_NODE`).
//   * C 448-458: `HAVE_FAST_UNALIGNED_ACCESS` is `#define`d in both arms.  The frozen build's
//     `CFLAGS` carry `-O2`, so `__OPTIMIZE__` is defined and the `#if` arm (`1`) is the text
//     that is compiled here; the `#else` arm (`0`) is carried under `no_fast_unaligned_access`.
//   * C 632-647, 681-696: `#if defined __STDC_ISO_10646__ || defined WINDOWSNT`.  clang has
//     neither macro on this target (the frozen table agrees: `WINDOWSNT` is `undef` and
//     `__STDC_ISO_10646__` is absent), so the `#else` arms (`Fstring_lessp`/`Fstring_equal`)
//     are live and the `str_collate` arms are carried under `stdc_iso_10646`.
//   * C 3223-3228: `#ifdef subprocesses` is live (`subprocesses` is `1`).
//   * C 3892-3894, 3923, 3976, 6836-6841: `HAVE_LANGINFO_CODESET` is `1`, so the outer arms
//     are live; `DAY_1` (7) and `MON_1` (21) are defined by `<langinfo.h>` and live.  The
//     `#ifdef HAVE_LANGINFO__NL_PAPER_WIDTH` arm (`undef`) is carried under
//     `have_langinfo_nl_paper_width`.
//   * C 5383-5391: live (`EMACS_INT_MAX > INT32_MAX`); no `#else`.
//
// `gnutls.h` (C 40) contributes nothing to this unit on this target: every one of its uses
// this file makes is inside its own `#ifdef HAVE_GNUTLS`, which is `undef` here, and the file
// only mentions the name in a docstring (C 6104).  The header has no mirror (PORTING.md P-26)
// and there is nothing to declare at a use site.
//
// **Forward declarations are dropped** (PORTING.md 0.1-12): C 45-47's prototypes of
// `internal_equal` and `sxhash_obj` have no Rust counterpart (a declaration plus a definition
// is `E0428`); the definitions stay at C's positions.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe, unused_assignments, unreachable_code, while_true, unused_parens, unused_mut, unused_labels, unused_comparisons, overflowing_literals, unnecessary_cast, dropping_copy_types, unused_must_use, clashing_extern_declarations)]
// [U1-MODULE-UNIT]
use crate::common::*;

/* C 22-42: the `#include`s.  The mirrors are named with C's own header in the comment.  */
const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/stdlib.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/stdcountof.rs imported via common
// [U1] ../lib/include/sys_random.in.rs imported via common
// [U1] ../lib/include/unistd.in.rs imported via common
/* C 27: `#include <filevercmp.h>`.  The mirror's `pub mod decls` cannot be included here: see
   the digest-header note below (docs/ports/fns.md reports the collision), so `filenvercmp` is
   declared at its use site with C's own prototype.  */
// [U1] ../lib/include/limits.in.rs imported via common
// [U1] ../lib/include/errno.rs imported via common
// [U1] ../lib/include/math.rs imported via common
// [U1] ../lib/include/langinfo.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] ../lib/include/mini-gmp.rs imported via common
// [U1] include/bignum.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/intervals.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/buffer.rs imported via common
#[cfg(feature = "have_tree_sitter")]
const _: () = ();
// [U1] include/systime.rs imported via common
// [U1] include/font.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/frame.rs imported via common

/* Names this unit uses whose prototype C gets from a header with no mirror yet, or from a
   mirrored header that is missing the line (P-72 rule 3 / PORTING.md 0.1-2): each carries C's
   own prototype in a `/// C:` line and is reported in docs/ports/fns.md's gaps list.  */
unsafe extern "C-unwind" {
  /// C: lisp.h 4331-4332: `extern void insert_1_both (const char *, ptrdiff_t, ptrdiff_t,
  /// bool, bool, bool);` — insdel.c defines it (the mirror is missing the line).
  fn insert_1_both (str: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t, inherit: bool,
                    prepare: bool, before_markers: bool);







  /// C: lisp.h 4938: `extern AVOID nsberror (Lisp_Object);` (buffer.c defines it; the mirror
  /// is missing the line).
  fn nsberror (spec: Lisp_Object) -> !;


  /// C: lisp.h 4663-4664: `void *hash_table_alloc_bytes (ptrdiff_t nbytes);`
  /// `void hash_table_free_bytes (void *p, ptrdiff_t nbytes);` — alloc.c defines both (the
  /// mirror is missing the lines).
  fn hash_table_alloc_bytes (nbytes: ptrdiff_t) -> *mut c_void;




  /// C: lisp.h 4566: `extern Lisp_Object make_specified_string (const char *, ptrdiff_t,
  /// ptrdiff_t, bool);` — alloc.c defines it, and this unit only calls it (the mirror is
  /// missing the line; the task's own note names this as another unit's symbol).
  fn make_specified_string (data: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
                            multibyte: bool) -> Lisp_Object;

  /// C: lisp.h 4426-4427: `extern void parse_str_as_multibyte (const unsigned char *,
  /// ptrdiff_t, ptrdiff_t *, ptrdiff_t *);`
  fn parse_str_as_multibyte (str: *const u8, len: ptrdiff_t, chars: *mut ptrdiff_t,
                             bytes: *mut ptrdiff_t);
  /// C: lisp.h 4323: `extern ptrdiff_t copy_text (const unsigned char *, unsigned char *,
  /// ptrdiff_t, bool, bool);` (buffer.rs declares it too; the mirror is missing the line).
  fn copy_text (from: *const u8, to: *mut u8, nbytes: ptrdiff_t, from_multibyte: bool,
                to_multibyte: bool) -> ptrdiff_t;
}
unsafe extern "C" {




  /// C: <stdlib.h>: `extern int getloadavg (double [], int);` (macOS libc provides it; the
  /// stdlib mirror is missing the line).
  fn getloadavg (loadavg: *mut f64, nelem: c_int) -> c_int;

}


/* C 44: `enum equal_kind { EQUAL_NO_QUIT, EQUAL_PLAIN, EQUAL_INCLUDING_PROPERTIES };` */
pub type equal_kind = c_int;
pub const EQUAL_NO_QUIT: equal_kind = 0;
pub const EQUAL_PLAIN: equal_kind = 1;
pub const EQUAL_INCLUDING_PROPERTIES: equal_kind = 2;

DEFUN! ("identity", Fidentity, Sidentity, 1, 1, 0,
       doc: "Return the ARGUMENT unchanged.",
       (argument: Lisp_Object),
       safe {
  crate::safe_data::identity (argument)
});

/* Return a random Lisp fixnum I in the range 0 <= I < LIM,
   where LIM is taken from a positive fixnum.  */
/* C 58-73: `get_random_fixnum`.  */
unsafe fn get_random_fixnum (lim: EMACS_INT) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* Return the remainder of a random integer R (in range 0..INTMASK)
     divided by LIM, except reject the rare case where R is so close
     to INTMASK that the remainder isn't random.  */
  let mut difflim: EMACS_INT = INTMASK - lim + 1;
  let mut diff: EMACS_INT;
  let mut remainder: EMACS_INT;
  loop
    {
      let r: EMACS_INT = get_random ();
      remainder = r % lim;
      diff = r - remainder;
      if ! (difflim < diff)
	{ break; }
    }

  return make_fixnum (remainder);

  }
}

DEFUN! ("random", Frandom, Srandom, 0, 1, 0,
       doc: "Return a pseudo-random integer.\nBy default, return a random fixnum value with all fixnums equally likely.\nWith positive integer value of the optional argument LIMIT, return random\ninteger in interval [0,LIMIT); it is an error to pass a negative LIMIT.\nIf LIMIT is t, set the random number seed from the system's entropy\npool if available, otherwise from less-random volatile data such as the time.\nIf LIMIT is a string, set the seed based on the string's contents.\nAny other value of LIMIT is interpreted as nil.\n\nSee Info node `(elisp)Random Numbers' for more details.",
       (limit: Lisp_Object),
       {
  if EQ (limit, Qt ())
    { init_random (); }
  else if STRINGP (limit)
    { seed_random (SSDATA (limit) as *mut c_void, SBYTES (limit)); }
  else if FIXNUMP (limit)
    {
      let lim: EMACS_INT = XFIXNUM (limit);
      if lim <= 0
	{ xsignal1 (Qargs_out_of_range (), limit); }
      return get_random_fixnum (lim);
    }
  else if BIGNUMP (limit)
    {
      let lim: *mut Lisp_Bignum = XBIGNUM (limit);
      if mpz_sgn (bignum_val (lim)) <= 0
	{ xsignal1 (Qargs_out_of_range (), limit); }
      return get_random_bignum (lim);
    }

  return make_ufixnum (get_random ());
});

/* Random data-structure functions.  */

/* Return LIST's length.  Signal an error if LIST is not a proper list.  */

#[unsafe(no_mangle)]
pub extern "C-unwind" fn list_length (list: Lisp_Object) -> ptrdiff_t
{
  crate::safe_data::list_length_impl (list)
}


DEFUN! ("length", Flength, Slength, 1, 1, 0,
       doc: "Return the length of vector, list or string SEQUENCE.\nA byte-code function object is also allowed.\n\nIf the string contains multibyte characters, this is not necessarily\nthe number of bytes in the string; it is the number of characters.\nTo get the number of bytes, use `string-bytes'.\n\nIf the length of a list is being computed to compare to a (small)\nnumber, the `length<', `length>' and `length=' functions may be more\nefficient.",
       (sequence: Lisp_Object),
       safe {
  crate::safe_data::length_subr (sequence)
});

DEFUN! ("safe-length", Fsafe_length, Ssafe_length, 1, 1, 0,
       doc: "Return the length of a list, but avoid error or infinite loop.\nThis function never gets an error.  If LIST is not really a list,\nit returns 0.  If LIST is circular, it returns an integer that is at\nleast the number of distinct elements.",
       (list: Lisp_Object),
       safe {
  crate::safe_data::safe_length (list)
});

DEFUN! ("length<", Flength_less, Slength_less, 2, 2, 0,
       doc: "Return non-nil if SEQUENCE is shorter than LENGTH.\nSee `length' for allowed values of SEQUENCE and how elements are\ncounted.",
       (sequence: Lisp_Object, length: Lisp_Object),
       safe {
  crate::safe_data::length_less (sequence, length)
});

DEFUN! ("length>", Flength_greater, Slength_greater, 2, 2, 0,
       doc: "Return non-nil if SEQUENCE is longer than LENGTH.\nSee `length' for allowed values of SEQUENCE and how elements are\ncounted.",
       (sequence: Lisp_Object, length: Lisp_Object),
       safe {
  crate::safe_data::length_greater (sequence, length)
});

DEFUN! ("length=", Flength_equal, Slength_equal, 2, 2, 0,
       doc: "Return non-nil if SEQUENCE has length equal to LENGTH.\nSee `length' for allowed values of SEQUENCE and how elements are\ncounted.",
       (sequence: Lisp_Object, length: Lisp_Object),
       safe {
  crate::safe_data::length_equal (sequence, length)
});

DEFUN! ("proper-list-p", Fproper_list_p, Sproper_list_p, 1, 1, 0,
       doc: "Return OBJECT's length if it is a proper list, nil otherwise.\nA proper list is neither circular nor dotted (i.e., its last cdr is nil).",
       (object: Lisp_Object),
       safe {
  crate::safe_data::proper_list_p (object)
});

DEFUN! ("string-bytes", Fstring_bytes, Sstring_bytes, 1, 1, 0,
       doc: "Return the number of bytes in STRING.\nIf STRING is multibyte, this may be greater than the length of STRING.",
       (string: Lisp_Object),
       safe {
  crate::safe_data::string_bytes (string)
});

DEFUN! ("string-distance", Fstring_distance, Sstring_distance, 2, 3, 0,
       doc: "Return Levenshtein distance between STRING1 and STRING2.\nThe distance is the number of deletions, insertions, and substitutions\nrequired to transform STRING1 into STRING2.\nIf BYTECOMPARE is nil or omitted, compute distance in terms of characters.\nIf BYTECOMPARE is non-nil, compute distance in terms of bytes.\nLetter-case is significant, but text properties are ignored.",
       (string1: Lisp_Object, string2: Lisp_Object, bytecompare: Lisp_Object),
       safe {
  crate::safe_data::string_distance (string1, string2, bytecompare)
});

DEFUN! ("string-equal", Fstring_equal, Sstring_equal, 2, 2, 0,
       doc: "Return t if two strings have identical contents.\nCase is significant, but text properties are ignored.\nSymbols are also allowed; their print names are used instead.\n\nSee also `string-equal-ignore-case'.",
       (s1: Lisp_Object, s2: Lisp_Object),
       safe {
  crate::safe_data::string_equal (s1, s2)
});

DEFUN! ("compare-strings", Fcompare_strings, Scompare_strings, 6, 7, 0,
       doc: "Compare the contents of two strings, converting to multibyte if needed.\nThe arguments START1, END1, START2, and END2, if non-nil, are\npositions specifying which parts of STR1 or STR2 to compare.  In\nstring STR1, compare the part between START1 (inclusive) and END1\n(exclusive).  If START1 is nil, it defaults to 0, the beginning of\nthe string; if END1 is nil, it defaults to the length of the string.\nLikewise, in string STR2, compare the part between START2 and END2.\nLike in `substring', negative values are counted from the end.\n\nThe strings are compared by the numeric values of their characters.\nFor instance, STR1 is \"less than\" STR2 if its first differing\ncharacter has a smaller numeric value.  If IGNORE-CASE is non-nil,\ncharacters are converted to upper-case before comparing them.  Unibyte\nstrings are converted to multibyte for comparison.\n\nThe value is t if the strings (or specified portions) match.\nIf string STR1 is less, the value is a negative number N;\n  - 1 - N is the number of characters that match at the beginning.\nIf string STR1 is greater, the value is a positive number N;\n  N - 1 is the number of characters that match at the beginning.",
       (str1: Lisp_Object, start1: Lisp_Object, end1: Lisp_Object, str2: Lisp_Object,
        start2: Lisp_Object, end2: Lisp_Object, ignore_case: Lisp_Object),
       safe {
  crate::safe_data::compare_strings (str1, start1, end1, str2, start2, end2, ignore_case)
});

#[cfg (not (feature = "no_fast_unaligned_access"))]
pub const HAVE_FAST_UNALIGNED_ACCESS: bool = true;
#[cfg (feature = "no_fast_unaligned_access")]
pub const HAVE_FAST_UNALIGNED_ACCESS: bool = false;

#[inline(always)]
fn string_cmp (string1: Lisp_Object, string2: Lisp_Object) -> c_int
{
  crate::safe_data::string_cmp_safe (string1, string2)
}

pub fn filenvercmp_strings_safe (string1: Lisp_Object, string2: Lisp_Object) -> c_int
{
  CHECK_STRING (string1);
  CHECK_STRING (string2);
  // SAFETY: `string1` and `string2` are live `Lisp_String`s; byte pointers and lengths match `SBYTES`.
  unsafe { filenvercmp (SSDATA (string1), SBYTES (string1), SSDATA (string2), SBYTES (string2)) }
}

DEFUN! ("string-lessp", Fstring_lessp, Sstring_lessp, 2, 2, 0,
       doc: "Return non-nil if STRING1 is less than STRING2 in lexicographic order.\nCase is significant.\nSymbols are also allowed; their print names are used instead.",
       (string1: Lisp_Object, string2: Lisp_Object),
       safe {
  crate::safe_data::string_lessp (string1, string2)
});

DEFUN! ("string-version-lessp", Fstring_version_lessp,
       Sstring_version_lessp, 2, 2, 0,
       doc: "Return non-nil if S1 is less than S2, as version strings.\n\nThis function compares version strings S1 and S2:\n   1) By prefix lexicographically.\n   2) Then by version (similarly to version comparison of Debian's dpkg).\n      Leading zeros in version numbers are ignored.\n   3) If both prefix and version are equal, compare as ordinary strings.\n\nFor example, \"foo2.png\" compares less than \"foo12.png\".\nCase is significant.\nSymbols are also allowed; their print names are used instead.",
       (string1: Lisp_Object, string2: Lisp_Object),
       safe {
  crate::safe_data::string_version_lessp (string1, string2)
});

DEFUN! ("string-collate-lessp", Fstring_collate_lessp, Sstring_collate_lessp, 2, 4, 0,
       doc: "Return t if first arg string is less than second in collation order.\nSymbols are also allowed; their print names are used instead.\n\nThis function obeys the conventions for collation order in your\nlocale settings.  For example, punctuation and whitespace characters\nmight be considered less significant for sorting:\n\n(sort \\='(\"11\" \"12\" \"1 1\" \"1 2\" \"1.1\" \"1.2\")\n      :lessp #\\='string-collate-lessp)\n  => (\"11\" \"1 1\" \"1.1\" \"12\" \"1 2\" \"1.2\")\n\nThe optional argument LOCALE, a string, overrides the setting of your\ncurrent locale identifier for collation.  The value is system\ndependent; a LOCALE \"en_US.UTF-8\" is applicable on POSIX systems,\nwhile it would be, e.g., \"enu_USA.1252\" on MS-Windows systems.\n\nIf IGNORE-CASE is non-nil, characters are converted to lower-case\nbefore comparing them.\n\nTo emulate Unicode-compliant collation on MS-Windows systems,\nbind `w32-collate-ignore-punctuation' to a non-nil value, since\nthe codeset part of the locale cannot be \"UTF-8\" on MS-Windows.\n\nSome operating systems do not implement correct collation (in specific\nlocale environments or at all).  Then, this functions falls back to\ncase-sensitive `string-lessp' and IGNORE-CASE argument is ignored.",
       (s1: Lisp_Object, s2: Lisp_Object, locale: Lisp_Object, ignore_case: Lisp_Object),
       safe {
  crate::safe_data::string_collate_lessp (s1, s2, locale, ignore_case)
});

DEFUN! ("string-collate-equalp", Fstring_collate_equalp, Sstring_collate_equalp, 2, 4, 0,
       doc: "Return t if two strings have identical contents.\nSymbols are also allowed; their print names are used instead.\n\nThis function obeys the conventions for collation order in your locale\nsettings.  For example, characters with different coding points but\nthe same meaning might be considered as equal, like different grave\naccent Unicode characters:\n\n(string-collate-equalp (string ?\\uFF40) (string ?\\u1FEF))\n  => t\n\nThe optional argument LOCALE, a string, overrides the setting of your\ncurrent locale identifier for collation.  The value is system\ndependent; a LOCALE \"en_US.UTF-8\" is applicable on POSIX systems,\nwhile it would be \"enu_USA.1252\" on MS Windows systems.\n\nIf IGNORE-CASE is non-nil, characters are converted to lower-case\nbefore comparing them.\n\nTo emulate Unicode-compliant collation on MS-Windows systems,\nbind `w32-collate-ignore-punctuation' to a non-nil value, since\nthe codeset part of the locale cannot be \"UTF-8\" on MS-Windows.\n\nIf your system does not support a locale environment, this function\nbehaves like `string-equal', and in that case the IGNORE-CASE argument\nis ignored.\n\nDo NOT use this function to compare file names for equality.",
       (s1: Lisp_Object, s2: Lisp_Object, locale: Lisp_Object, ignore_case: Lisp_Object),
       safe {
  crate::safe_data::string_collate_equalp (s1, s2, locale, ignore_case)
});

/* C 705-709: `concat2`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn concat2 (s1: Lisp_Object, s2: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut args: [Lisp_Object; 2] = [s1, s2];
  return concat_to_string (2, args.as_mut_ptr ());

  }
}

/* C 711-715: `concat3`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn concat3 (s1: Lisp_Object, s2: Lisp_Object, s3: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut args: [Lisp_Object; 3] = [s1, s2, s3];
  return concat_to_string (3, args.as_mut_ptr ());

  }
}

DEFUN! ("append", Fappend, Sappend, 0, MANY, 0,
       doc: "Concatenate all the arguments and make the result a list.\nThe result is a list whose elements are the elements of all the arguments.\nEach argument may be a list, vector or string.\n\nAll arguments except the last argument are copied.  The last argument\nis just used as the tail of the new list.  If the last argument is not\na list, this results in a dotted list.\n\nAs an exception, if all the arguments except the last are nil, and the\nlast argument is not a list, the return value is that last argument\nunaltered, not a list.\n\nusage: (append &rest SEQUENCES)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 0
    { return Qnil (); }
  if nargs == 2
    {
      let a0: Lisp_Object = *args;
      let a1: Lisp_Object = *args.offset (1);
      if NILP (a0)
	{ return a1; }
      if CONSP (a0) && NILP (XCDR (a0))
	{ return Fcons (XCAR (a0), a1); }
    }
  return concat_to_list (nargs - 1, args, *args.offset (nargs - 1));
});

DEFUN! ("concat", Fconcat, Sconcat, 0, MANY, 0,
       doc: "Concatenate all the arguments and make the result a string.\nThe result is a string whose elements are the elements of all the arguments.\nEach argument may be a string or a list or vector of characters (integers).\n\nValues of the `composition' property of the result are not guaranteed\nto be `eq'.\nusage: (concat &rest SEQUENCES)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return concat_to_string (nargs, args);
});

DEFUN! ("vconcat", Fvconcat, Svconcat, 0, MANY, 0,
       doc: "Concatenate all the arguments and make the result a vector.\nThe result is a vector whose elements are the elements of all the arguments.\nEach argument may be a list, vector or string.\nusage: (vconcat &rest SEQUENCES)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return concat_to_vector (nargs, args);
});

pub fn copy_sequence_string_safe (arg: Lisp_Object) -> Lisp_Object
{
  CHECK_STRING (arg);
  // SAFETY: `arg` is verified `STRINGP`; allocates a fresh string of identical byte/char counts and copies intervals if present.
  unsafe {
    let bytes: ptrdiff_t = SBYTES (arg);
    let chars: ptrdiff_t = SCHARS (arg);
    let val: Lisp_Object = if STRING_MULTIBYTE (arg)
      { make_uninit_multibyte_string (chars, bytes) }
    else
      { make_uninit_string (bytes) };
    std::ptr::copy_nonoverlapping (SDATA (arg) as *const c_void,
                                   SDATA (val) as *mut c_void, bytes as usize);
    let ivs: INTERVAL = string_intervals (arg);
    if !ivs.is_null ()
      {
        let copy: INTERVAL = copy_intervals (ivs, 0, chars);
        set_interval_object (copy, val);
        set_string_intervals (val, copy);
      }
    val
  }
}

pub fn copy_sequence_vector_or_record_safe (arg: Lisp_Object, is_record: bool) -> Lisp_Object
{
  // SAFETY: `arg` is verified `VECTORP` (`!is_record`) or `RECORDP` (`is_record`) by caller.
  unsafe {
    if is_record
      {
        eassert! (RECORDP (arg));
        Frecord (PVSIZE (arg), (*XVECTOR (arg)).contents.as_mut_ptr ())
      }
    else
      {
        eassert! (VECTORP (arg));
        Fvector (ASIZE (arg), (*XVECTOR (arg)).contents.as_mut_ptr ())
      }
  }
}

pub fn copy_sequence_bool_vector_safe (arg: Lisp_Object) -> Lisp_Object
{
  CHECK_BOOL_VECTOR (arg);
  // SAFETY: `arg` is verified `BOOL_VECTOR_P`; allocates a fresh bool-vector of identical bit/byte counts.
  unsafe {
    let nbits: EMACS_INT = bool_vector_size (arg);
    let nbytes: ptrdiff_t = bool_vector_bytes (nbits);
    let val: Lisp_Object = make_uninit_bool_vector (nbits);
    std::ptr::copy_nonoverlapping (bool_vector_data (arg) as *const u8,
                                   bool_vector_data (val) as *mut u8,
                                   nbytes as usize);
    val
  }
}

DEFUN! ("copy-sequence", Fcopy_sequence, Scopy_sequence, 1, 1, 0,
       doc: "Return a copy of a list, vector, string, char-table or record.\nThe elements of a list, vector or record are not copied; they are\nshared with the original.  See Info node `(elisp) Sequence Functions'\nfor more details about this sharing and its effects.\nIf the original sequence is empty, this function may return\nthe same empty object instead of its copy.",
       (arg: Lisp_Object),
       safe {
  crate::safe_data::copy_sequence (arg)
});

/* This structure holds information of an argument of `concat_to_string'
   that is a string and has text properties to be copied.  */
#[repr(C)]
pub struct textprop_rec
{
  pub argnum: ptrdiff_t,		/* refer to ARGS (arguments of `concat') */
  pub to: ptrdiff_t,			/* refer to VAL (the target string) */
}

/* C 835-1029: `concat_to_string`.  */
unsafe fn concat_to_string (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* Check types and compute total length in chars of arguments in RESULT_LEN,
     length in bytes in RESULT_LEN_BYTE, and determine in DEST_MULTIBYTE
     whether the result should be a multibyte string.  */
  let mut result_len: EMACS_INT = 0;
  let mut result_len_byte: EMACS_INT = 0;
  let mut dest_multibyte: bool = false;
  let mut some_unibyte: bool = false;
  let mut num_with_textprops: ptrdiff_t = 0;
  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      let arg: Lisp_Object = *args.offset (i as isize);
      let mut len: EMACS_INT;

      /* We must count the number of bytes needed in the string
	 as well as the number of characters.  */

      if STRINGP (arg)
	{
	  let arg_len_byte: ptrdiff_t = SBYTES (arg);
	  len = SCHARS (arg);
	  if !string_intervals (arg).is_null ()
	    { num_with_textprops += 1; }
	  if STRING_MULTIBYTE (arg)
	    { dest_multibyte = true; }
	  else
	    { some_unibyte = true; }
	  if STRING_BYTES_BOUND - result_len_byte < arg_len_byte
	    { string_overflow (); }
	  result_len_byte += arg_len_byte;
	}
      else if VECTORP (arg)
	{
	  len = ASIZE (arg);
	  let mut arg_len_byte: ptrdiff_t = 0;
	  let mut j: ptrdiff_t = 0;
	  while j < len
	    {
	      let ch: Lisp_Object = AREF (arg, j);
	      CHECK_CHARACTER (ch);
	      let c: c_int = XFIXNAT (ch) as c_int;
	      arg_len_byte += CHAR_BYTES (c) as ptrdiff_t;
	      if !ASCII_CHAR_P (c as isize) && !CHAR_BYTE8_P (c)
		{ dest_multibyte = true; }
	      j += 1;
	    }
	  if STRING_BYTES_BOUND - result_len_byte < arg_len_byte
	    { string_overflow (); }
	  result_len_byte += arg_len_byte;
	}
      else if NILP (arg)
	{ i += 1; continue; }
      else if CONSP (arg)
	{
	  let mut arg: Lisp_Object = arg;
	  len = XFIXNAT (Flength (arg)) as EMACS_INT;
	  let mut arg_len_byte: ptrdiff_t = 0;
	  while CONSP (arg)
	    {
	      let ch: Lisp_Object = XCAR (arg);
	      CHECK_CHARACTER (ch);
	      let c: c_int = XFIXNAT (ch) as c_int;
	      arg_len_byte += CHAR_BYTES (c) as ptrdiff_t;
	      if !ASCII_CHAR_P (c as isize) && !CHAR_BYTE8_P (c)
		{ dest_multibyte = true; }
	      arg = XCDR (arg);
	    }
	  if STRING_BYTES_BOUND - result_len_byte < arg_len_byte
	    { string_overflow (); }
	  result_len_byte += arg_len_byte;
	}
      else
	{ wrong_type_argument (Qsequencep (), arg); }

      result_len += len;
      if MOST_POSITIVE_FIXNUM < result_len
	{ memory_full_up (); }
      i += 1;
    }

  if dest_multibyte && some_unibyte
    {
      /* Non-ASCII characters in unibyte strings take two bytes when
	 converted to multibyte -- count them and adjust the total.  */
      let mut i: ptrdiff_t = 0;
      while i < nargs
	{
	  let arg: Lisp_Object = *args.offset (i as isize);
	  if STRINGP (arg) && !STRING_MULTIBYTE (arg)
	    {
	      let bytes: ptrdiff_t = SCHARS (arg);
	      let s: *const u8 = SDATA (arg);
	      let mut nonascii: ptrdiff_t = 0;
	      let mut j: ptrdiff_t = 0;
	      while j < bytes
		{
		  nonascii += (*s.offset (j as isize) >> 7) as ptrdiff_t;
		  j += 1;
		}
	      if STRING_BYTES_BOUND - result_len_byte < nonascii
		{ string_overflow (); }
	      result_len_byte += nonascii;
	    }
	  i += 1;
	}
    }

  if !dest_multibyte
    { result_len_byte = result_len; }

  /* Create the output object.  */
  let result: Lisp_Object = if dest_multibyte
    { make_uninit_multibyte_string (result_len, result_len_byte) }
    else
    { make_uninit_string (result_len) };

  /* Copy the contents of the args into the result.  */
  let mut toindex: ptrdiff_t = 0;
  let mut toindex_byte: ptrdiff_t = 0;

  /* When we make a multibyte string, we can't copy text properties
     while concatenating each string because the length of resulting
     string can't be decided until we finish the whole concatenation.
     So, we record strings that have text properties to be copied
     here, and copy the text properties after the concatenation.  */
  let mut textprops: *mut textprop_rec = std::ptr::null_mut ();
  /* Number of elements in textprops.  */
  let mut num_textprops: ptrdiff_t = 0;
  let mut sa_avail: isize = MAX_ALLOCA;
  let mut sa_count: specpdl_ref = specpdl_ref { bytes: 0 };
  let mut sa_mark: usize = 0;
  if num_with_textprops > 0
    {
      sa_count = SPECPDL_INDEX ();
      sa_mark = sa_stack_mark ();
      SAFE_NALLOCA! (sa_avail, textprops, 1, num_with_textprops);
    }

  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      let arg: Lisp_Object = *args.offset (i as isize);
      if STRINGP (arg)
	{
	  if num_with_textprops > 0 && !string_intervals (arg).is_null ()
	    {
	      (*textprops.offset (num_textprops as isize)).argnum = i;
	      (*textprops.offset (num_textprops as isize)).to = toindex;
	      num_textprops += 1;
	    }
	  let nchars: ptrdiff_t = SCHARS (arg);
	  if STRING_MULTIBYTE (arg) == dest_multibyte
	    {
	      /* Between strings of the same kind, copy fast.  */
	      let arg_len_byte: ptrdiff_t = SBYTES (arg);
	      std::ptr::copy_nonoverlapping (SDATA (arg) as *const c_void,
					     SDATA (result).offset (toindex_byte) as *mut c_void,
					     arg_len_byte as usize);
	      toindex_byte += arg_len_byte;
	    }
	  else
	    {
	      /* Copy a single-byte string to a multibyte string.  */
	      toindex_byte += str_to_multibyte (SDATA (result).offset (toindex_byte),
						SDATA (arg), nchars);
	    }
	  toindex += nchars;
	}
      else if VECTORP (arg)
	{
	  let len: ptrdiff_t = ASIZE (arg);
	  let mut j: ptrdiff_t = 0;
	  while j < len
	    {
	      let c: c_int = XFIXNAT (AREF (arg, j)) as c_int;
	      if dest_multibyte
		{ toindex_byte += CHAR_STRING (c, SDATA (result).offset (toindex_byte)) as ptrdiff_t; }
	      else
		{
		  SSET (result, toindex_byte as usize, c as u8);
		  toindex_byte += 1;
		}
	      toindex += 1;
	      j += 1;
	    }
	}
      else
	{
	  let mut tail: Lisp_Object = arg;
	  while !NILP (tail)
	    {
	      let c: c_int = XFIXNAT (XCAR (tail)) as c_int;
	      if dest_multibyte
		{ toindex_byte += CHAR_STRING (c, SDATA (result).offset (toindex_byte)) as ptrdiff_t; }
	      else
		{
		  SSET (result, toindex_byte as usize, c as u8);
		  toindex_byte += 1;
		}
	      toindex += 1;
	      tail = XCDR (tail);
	    }
	}
      i += 1;
    }

  if num_textprops > 0
    {
      let mut last_to_end: ptrdiff_t = -1;
      let mut i: ptrdiff_t = 0;
      while i < num_textprops
	{
	  let arg: Lisp_Object = *args.offset ((*textprops.offset (i as isize)).argnum as isize);
	  let props: Lisp_Object = text_property_list (arg,
						       make_fixnum (0),
						       make_fixnum (SCHARS (arg)),
						       Qnil ());
	  /* If successive arguments have properties, be sure that the
	     value of `composition' property be the copy.  */
	  if last_to_end == (*textprops.offset (i as isize)).to
	    { make_composition_value_copy (props); }
	  add_text_properties_from_list (result, props,
					 make_fixnum ((*textprops.offset (i as isize)).to));
	  last_to_end = (*textprops.offset (i as isize)).to + SCHARS (arg);
	  i += 1;
	}
    }

  if num_with_textprops > 0
    { SAFE_FREE! (sa_count, sa_mark); }
  return result;

  }
}

/* Concatenate sequences into a list. */
/* C 1031-1107: `concat_to_list`.  C 699 declares it `static`, so it has internal linkage
   (the *declaration* carries the keyword, not the definition): plain `unsafe fn` here, no
   `#[unsafe(no_mangle)]` — the reference object exports only `_concat2`/`_concat3`.  */
unsafe fn concat_to_list (nargs: ptrdiff_t, args: *mut Lisp_Object,
			      last_tail: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* Copy the contents of the args into the result.  */
  let mut result: Lisp_Object = Qnil ();
  let mut last: Lisp_Object = Qnil ();	/* Last cons in result if nonempty.  */

  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      let mut arg: Lisp_Object = *args.offset (i as isize);
      /* List arguments are treated specially since this is the common case.  */
      if CONSP (arg)
	{
	  let head: Lisp_Object = Fcons (XCAR (arg), Qnil ());
	  let mut prev: Lisp_Object = head;
	  arg = XCDR (arg);
	  if ! NILP (arg)
	    {
	      let mut fast_steps: c_int = 0;
	      while CONSP (arg) && fast_steps < 64
		{
		  let next: Lisp_Object = Fcons (XCAR (arg), Qnil ());
		  XSETCDR (prev, next);
		  prev = next;
		  arg = XCDR (arg);
		  fast_steps += 1;
		}
	      if ! NILP (arg)
		{
		  /* C's FOR_EACH_TAIL leaves the terminating non-cons in ARG; the port's macro cannot
		     hand it back, so the body records it (docs/ports/fns.md, Decisions).  */
		  let mut arg_end: Lisp_Object = arg;
		  FOR_EACH_TAIL! (arg, {
		    let next: Lisp_Object = Fcons (XCAR (arg), Qnil ());
		    XSETCDR (prev, next);
		    prev = next;
		    arg_end = XCDR (arg);
		  });
		  CHECK_LIST_END (arg_end, arg_end);
		}
	    }
	  if NILP (result)
	    { result = head; }
	  else
	    { XSETCDR (last, head); }
	  last = prev;
	}
      else if NILP (arg)
	{ }
      else if VECTORP (arg) || STRINGP (arg)
	       || BOOL_VECTOR_P (arg) || CLOSUREP (arg)
	{
	  let arglen: ptrdiff_t = XFIXNUM (Flength (arg));
	  let mut argindex_byte: ptrdiff_t = 0;

	  /* Copy element by element.  */
	  let mut argindex: ptrdiff_t = 0;
	  while argindex < arglen
	    {
	      /* Fetch next element of `arg' arg into `elt', or break if
		 `arg' is exhausted. */
	      let elt: Lisp_Object;
	      if STRINGP (arg)
		{
		  let c: c_int;
		  if STRING_MULTIBYTE (arg)
		    {
		      let mut char_idx: ptrdiff_t = argindex;
		      c = fetch_string_char_advance_no_check (arg, &raw mut char_idx,
							      &raw mut argindex_byte);
		    }
		  else
		    { c = SREF (arg, argindex as usize) as c_int; }
		  elt = make_fixed_natnum (c as EMACS_INT);
		}
	      else if BOOL_VECTOR_P (arg)
		{ elt = bool_vector_ref (arg, argindex); }
	      else
		{ elt = AREF (arg, argindex); }

	      /* Store this element into the result.  */
	      let node: Lisp_Object = Fcons (elt, Qnil ());
	      if NILP (result)
		{ result = node; }
	      else
		{ XSETCDR (last, node); }
	      last = node;
	      argindex += 1;
	    }
	}
      else
	{ wrong_type_argument (Qsequencep (), arg); }
      i += 1;
    }

  if NILP (result)
    { result = last_tail; }
  else
    { XSETCDR (last, last_tail); }

  return result;

  }
}

/* Concatenate sequences into a vector.  */
/* C 1114-1193: `concat_to_vector`.  C 701 declares it `static` (as above).  */
unsafe fn concat_to_vector (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* Check argument types and compute total length of arguments.  */
  let mut result_len: EMACS_INT = 0;
  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      let arg: Lisp_Object = *args.offset (i as isize);
      if !(VECTORP (arg) || CONSP (arg) || NILP (arg) || STRINGP (arg)
	    || BOOL_VECTOR_P (arg) || CLOSUREP (arg))
	{ wrong_type_argument (Qsequencep (), arg); }
      let len: EMACS_INT = XFIXNAT (Flength (arg)) as EMACS_INT;
      result_len += len;
      if MOST_POSITIVE_FIXNUM < result_len
	{ memory_full_up (); }
      i += 1;
    }

  /* Create the output vector.  */
  let result: Lisp_Object = make_uninit_vector (result_len);
  let mut dst: *mut Lisp_Object = (*XVECTOR (result)).contents.as_mut_ptr ();

  /* Copy the contents of the args into the result.  */

  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      let mut arg: Lisp_Object = *args.offset (i as isize);
      if VECTORP (arg)
	{
	  let size: ptrdiff_t = ASIZE (arg);
	  std::ptr::copy_nonoverlapping ((*XVECTOR (arg)).contents.as_mut_ptr (),
					 dst, size as usize);
	  dst = dst.offset (size);
	}
      else if CONSP (arg)
	{
	  loop
	    {
	      *dst = XCAR (arg);
	      dst = dst.offset (1);
	      arg = XCDR (arg);
	      if NILP (arg)
		{ break; }
	    }
	}
      else if NILP (arg)
	{ }
      else if STRINGP (arg)
	{
	  let size: ptrdiff_t = SCHARS (arg);
	  if STRING_MULTIBYTE (arg)
	    {
	      let mut byte: ptrdiff_t = 0;
	      let mut i: ptrdiff_t = 0;
	      while i < size
		{
		  let c: c_int = fetch_string_char_advance_no_check (arg, &raw mut i, &raw mut byte);
		  *dst = make_fixnum (c as EMACS_INT);
		  dst = dst.offset (1);
		}
	    }
	  else
	    {
	      let mut i: ptrdiff_t = 0;
	      while i < size
		{
		  *dst = make_fixnum (SREF (arg, i as usize) as EMACS_INT);
		  dst = dst.offset (1);
		  i += 1;
		}
	    }
	}
      else if BOOL_VECTOR_P (arg)
	{
	  let size: ptrdiff_t = bool_vector_size (arg);
	  let mut i: ptrdiff_t = 0;
	  while i < size
	    {
	      *dst = bool_vector_ref (arg, i);
	      dst = dst.offset (1);
	      i += 1;
	    }
	}
      else
	{
	  eassert! (CLOSUREP (arg));
	  let size: ptrdiff_t = PVSIZE (arg);
	  std::ptr::copy_nonoverlapping ((*XVECTOR (arg)).contents.as_mut_ptr (),
					 dst, size as usize);
	  dst = dst.offset (size);
	}
      i += 1;
    }
  eassert! (dst == (*XVECTOR (result)).contents.as_mut_ptr ().offset (result_len));

  return result;

  }
}

/* C 1189-1191: the three file-scope objects the cache below uses.  */
static string_char_byte_cache_string: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
static string_char_byte_cache_charpos: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);
static string_char_byte_cache_bytepos: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);

/* Return the byte index corresponding to CHAR_INDEX in STRING.  */

/* C 1196-1254: `string_char_to_byte`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn string_char_to_byte (string: Lisp_Object, char_index: ptrdiff_t) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut i_byte: ptrdiff_t;
  let mut best_below: ptrdiff_t;
  let mut best_below_byte: ptrdiff_t;
  let mut best_above: ptrdiff_t;
  let mut best_above_byte: ptrdiff_t;

  best_below = 0;
  best_below_byte = 0;
  best_above = SCHARS (string);
  best_above_byte = SBYTES (string);
  if best_above == best_above_byte
    { return char_index; }

  if BASE_EQ (string, (*string_char_byte_cache_string.as_mut_ptr ()))
    {
      if (*string_char_byte_cache_charpos.as_mut_ptr ()) < char_index
	{
	  best_below = (*string_char_byte_cache_charpos.as_mut_ptr ());
	  best_below_byte = (*string_char_byte_cache_bytepos.as_mut_ptr ());
	}
      else
	{
	  best_above = (*string_char_byte_cache_charpos.as_mut_ptr ());
	  best_above_byte = (*string_char_byte_cache_bytepos.as_mut_ptr ());
	}
    }

  if char_index - best_below < best_above - char_index
    {
      let mut p: *mut u8 = SDATA (string).offset (best_below_byte);

      while best_below < char_index
	{
	  p = p.offset (BYTES_BY_CHAR_HEAD (*p as c_int) as isize);
	  best_below += 1;
	}
      i_byte = p.offset_from (SDATA (string));
    }
  else
    {
      let mut p: *mut u8 = SDATA (string).offset (best_above_byte);

      while best_above > char_index
	{
	  p = p.offset (-1);
	  while !CHAR_HEAD_P (*p as c_int) { p = p.offset (-1); }
	  best_above -= 1;
	}
      i_byte = p.offset_from (SDATA (string));
    }

  (*string_char_byte_cache_bytepos.as_mut_ptr ()) = i_byte;
  (*string_char_byte_cache_charpos.as_mut_ptr ()) = char_index;
  (*string_char_byte_cache_string.as_mut_ptr ()) = string;

  return i_byte;

  }
}

/* Return the character index corresponding to BYTE_INDEX in STRING.  */

/* C 1256-1320: `string_byte_to_char`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn string_byte_to_char (string: Lisp_Object, byte_index: ptrdiff_t) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut i: ptrdiff_t;
  let mut i_byte: ptrdiff_t;
  let mut best_below: ptrdiff_t;
  let mut best_below_byte: ptrdiff_t;
  let mut best_above: ptrdiff_t;
  let mut best_above_byte: ptrdiff_t;

  best_below = 0;
  best_below_byte = 0;
  best_above = SCHARS (string);
  best_above_byte = SBYTES (string);
  if best_above == best_above_byte
    { return byte_index; }

  if BASE_EQ (string, (*string_char_byte_cache_string.as_mut_ptr ()))
    {
      if (*string_char_byte_cache_bytepos.as_mut_ptr ()) < byte_index
	{
	  best_below = (*string_char_byte_cache_charpos.as_mut_ptr ());
	  best_below_byte = (*string_char_byte_cache_bytepos.as_mut_ptr ());
	}
      else
	{
	  best_above = (*string_char_byte_cache_charpos.as_mut_ptr ());
	  best_above_byte = (*string_char_byte_cache_bytepos.as_mut_ptr ());
	}
    }

  if byte_index - best_below_byte < best_above_byte - byte_index
    {
      let mut p: *mut u8 = SDATA (string).offset (best_below_byte);
      let pend: *mut u8 = SDATA (string).offset (byte_index);

      while p < pend
	{
	  p = p.offset (BYTES_BY_CHAR_HEAD (*p as c_int) as isize);
	  best_below += 1;
	}
      i = best_below;
      i_byte = p.offset_from (SDATA (string));
    }
  else
    {
      let mut p: *mut u8 = SDATA (string).offset (best_above_byte);
      let pbeg: *mut u8 = SDATA (string).offset (byte_index);

      while p > pbeg
	{
	  p = p.offset (-1);
	  while !CHAR_HEAD_P (*p as c_int) { p = p.offset (-1); }
	  best_above -= 1;
	}
      i = best_above;
      i_byte = p.offset_from (SDATA (string));
    }

  (*string_char_byte_cache_bytepos.as_mut_ptr ()) = i_byte;
  (*string_char_byte_cache_charpos.as_mut_ptr ()) = i;
  (*string_char_byte_cache_string.as_mut_ptr ()) = string;

  return i;

  }
}

/* Convert STRING (if unibyte) to a multibyte string without changing
   the number of characters.  Characters 0x80..0xff are interpreted as
   raw bytes. */

/* C 1322-1341: `string_to_multibyte`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn string_to_multibyte (string: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if STRING_MULTIBYTE (string)
    { return string; }

  let nchars: ptrdiff_t = SCHARS (string);
  let nbytes: ptrdiff_t = count_size_as_multibyte (SDATA (string), nchars);
  /* If all the chars are ASCII, they won't need any more bytes once
     converted.  */
  if nbytes == nchars
    { return make_multibyte_string (SSDATA (string), nbytes, nbytes); }

  let ret: Lisp_Object = make_uninit_multibyte_string (nchars, nbytes);
  str_to_multibyte (SDATA (ret), SDATA (string), nchars);
  return ret;

  }
}


/* Convert STRING to a single-byte string.  */

/* C 1343-1362: `string_make_unibyte`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn string_make_unibyte (string: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let nchars: ptrdiff_t;
  let mut buf: *mut u8;
  let ret: Lisp_Object;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  if ! STRING_MULTIBYTE (string)
    { return string; }

  nchars = SCHARS (string);

  buf = SAFE_ALLOCA! (sa_avail, nchars) as *mut u8;
  copy_text (SDATA (string), buf, SBYTES (string),
	     true, false);

  ret = make_unibyte_string (buf as *mut c_char, nchars);
  SAFE_FREE! (sa_count, sa_mark);

  return ret;
  }
}

DEFUN! ("string-make-multibyte", Fstring_make_multibyte, Sstring_make_multibyte,
       1, 1, 0,
       doc: "Return the multibyte equivalent of STRING.\nIf STRING is unibyte and contains non-ASCII characters, the function\nconverts each unibyte character to an eight-bit raw byte in its\nmultibyte representation.  In this case, the returned string is a\nnewly created string with no text properties.  If STRING is multibyte\nor entirely ASCII, it is returned unchanged.  In particular, when\nSTRING is unibyte and entirely ASCII, the returned string is unibyte.\n(When the characters are all ASCII, Emacs primitives will treat the\nstring the same way whether it is unibyte or multibyte.)",
       (string: Lisp_Object),
       {
  CHECK_STRING (string);

  if STRING_MULTIBYTE (string)
    { return string; }

  let nchars: ptrdiff_t = SCHARS (string);
  let nbytes: ptrdiff_t = count_size_as_multibyte (SDATA (string), nchars);
  if nbytes == nchars
    { return string; }

  let ret: Lisp_Object = make_uninit_multibyte_string (nchars, nbytes);
  str_to_multibyte (SDATA (ret), SDATA (string), nchars);
  return ret;
});

DEFUN! ("string-make-unibyte", Fstring_make_unibyte, Sstring_make_unibyte,
       1, 1, 0,
       doc: "Return the unibyte equivalent of STRING.\nMultibyte character codes above 255 are converted to unibyte\nby taking just the low 8 bits of each character's code.",
       (string: Lisp_Object),
       {
  CHECK_STRING (string);

  return string_make_unibyte (string);
});

DEFUN! ("string-as-unibyte", Fstring_as_unibyte, Sstring_as_unibyte,
       1, 1, 0,
       doc: "Return a unibyte string with the same individual bytes as STRING.\nIf STRING is unibyte, the result is STRING itself.\nOtherwise it is a newly created unibyte string, with no text properties,\nwhose bytes are identical to those of STRING, except that any character\nof charset `eight-bit' is converted to the corresponding single byte.",
       (string: Lisp_Object),
       {
  let mut string: Lisp_Object = string;
  CHECK_STRING (string);

  if STRING_MULTIBYTE (string)
    {
      let str: *mut u8 = xlispstrdup (string) as *mut u8;
      let bytes: ptrdiff_t = str_as_unibyte (str, SBYTES (string));

      string = make_unibyte_string (str as *mut c_char, bytes);
      xfree (str as *mut c_void);
    }
  return string;
});

DEFUN! ("string-as-multibyte", Fstring_as_multibyte, Sstring_as_multibyte,
       1, 1, 0,
       doc: "Return a multibyte string with the same individual bytes as STRING.\nIf STRING is multibyte, the result is STRING itself.\nOtherwise it is a newly created string, with no text properties.\n\nIf STRING is unibyte and contains an individual 8-bit byte (i.e. not\npart of a correct utf-8 sequence), it is converted to the corresponding\nmultibyte character of charset `eight-bit'.\nSee also `string-to-multibyte'.\n\nBeware, this often doesn't really do what you think it does.\nIt is similar to (decode-coding-string STRING \\='utf-8-emacs).\nIf you're not sure, whether to use `string-as-multibyte' or\n`string-to-multibyte', use `string-to-multibyte'.",
       (string: Lisp_Object),
       {
  let mut string: Lisp_Object = string;
  CHECK_STRING (string);

  if ! STRING_MULTIBYTE (string)
    {
      let new_string: Lisp_Object;
      let mut nchars: ptrdiff_t = 0;
      let mut nbytes: ptrdiff_t = 0;

      parse_str_as_multibyte (SDATA (string),
			      SBYTES (string),
			      &raw mut nchars, &raw mut nbytes);
      new_string = make_uninit_multibyte_string (nchars, nbytes);
      std::ptr::copy_nonoverlapping (SDATA (string) as *const c_void,
				     SDATA (new_string) as *mut c_void,
				     SBYTES (string) as usize);
      if nbytes != SBYTES (string)
	{ str_as_multibyte (SDATA (new_string), nbytes,
			    SBYTES (string), std::ptr::null_mut ()); }
      string = new_string;
      set_string_intervals (string, std::ptr::null_mut ());
    }
  return string;
});

DEFUN! ("string-to-multibyte", Fstring_to_multibyte, Sstring_to_multibyte,
       1, 1, 0,
       doc: "Return a multibyte string with the same individual chars as STRING.\nIf STRING is multibyte, the result is STRING itself.\nOtherwise it is a newly created string, with no text properties.\n\nIf STRING is unibyte and contains an 8-bit byte, it is converted to\nthe corresponding multibyte character of charset `eight-bit'.\n\nThis differs from `string-as-multibyte' by converting each byte of a correct\nutf-8 sequence to an eight-bit character, not just bytes that don't form a\ncorrect sequence.",
       (string: Lisp_Object),
       {
  CHECK_STRING (string);

  return string_to_multibyte (string);
});

DEFUN! ("string-to-unibyte", Fstring_to_unibyte, Sstring_to_unibyte,
       1, 1, 0,
       doc: "Return a unibyte string with the same individual chars as STRING.\nIf STRING is unibyte, the result is STRING itself.\nOtherwise it is a newly created string, with no text properties,\nwhere each `eight-bit' character is converted to the corresponding byte.\nIf STRING contains a non-ASCII, non-`eight-bit' character,\nan error is signaled.",
       (string: Lisp_Object),
       {
  CHECK_STRING (string);
  if !STRING_MULTIBYTE (string)
    { return string; }

  let chars: ptrdiff_t = SCHARS (string);
  let ret: Lisp_Object = make_uninit_string (chars);
  let mut src: *mut u8 = SDATA (string);
  let mut dst: *mut u8 = SDATA (ret);
  let mut i: ptrdiff_t = 0;
  while i < chars
    {
      let b: u8 = *src;
      src = src.offset (1);
      if b <= 0x7f
	{
	  *dst = b;						 /* ASCII */
	  dst = dst.offset (1);
	}
      else if CHAR_BYTE8_HEAD_P (b as c_int)
	{
	  *dst = 0x80 | (b & 1) << 6 | (*src & 0x3f);	 /* raw byte */
	  src = src.offset (1);
	  dst = dst.offset (1);
	}
      else
	{ error (c"Cannot convert character at index %td to unibyte".as_ptr (), i); }
      i += 1;
    }
  return ret;
});


DEFUN! ("copy-alist", Fcopy_alist, Scopy_alist, 1, 1, 0,
       doc: "Return a copy of ALIST.\nThis is an alist which represents the same mapping from objects to objects,\nbut does not share the alist structure with ALIST.\nThe objects mapped (cars and cdrs of elements of the alist)\nare shared, however.\nElements of ALIST that are not conses are also shared.",
       (alist: Lisp_Object),
       safe {
  crate::safe_data::copy_alist (alist)
});

/* Check that ARRAY can have a valid subarray [FROM..TO),
   given that its size is SIZE.
   If FROM is nil, use 0; if TO is nil, use SIZE.
   Count negative values backwards from the end.
   Set *IFROM and *ITO to the two indexes used.  */

/* C 1545-1577: `validate_subarray`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn validate_subarray (array: Lisp_Object, from: Lisp_Object, to: Lisp_Object,
				 size: ptrdiff_t, ifrom: *mut ptrdiff_t, ito: *mut ptrdiff_t)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let (f, t) = validate_subarray_safe (array, from, to, size);
  *ifrom = f;
  *ito = t;
  }
}

pub fn substring_string_bytes_safe (
  string: Lisp_Object,
  from_char: ptrdiff_t,
  from_byte: ptrdiff_t,
  to_char: ptrdiff_t,
  to_byte: ptrdiff_t,
  copy_props: bool,
) -> Lisp_Object
{
  CHECK_STRING (string);
  eassert! (0 <= from_char && from_char <= to_char && to_char <= SCHARS (string));
  eassert! (0 <= from_byte && from_byte <= to_byte && to_byte <= SBYTES (string));
  // SAFETY: `string` is a live `Lisp_String` and `from_byte..to_byte` is within `0..SBYTES(string)`.
  unsafe {
    let res = make_specified_string (SSDATA (string).offset (from_byte) as *const c_char,
                                     to_char - from_char, to_byte - from_byte,
                                     STRING_MULTIBYTE (string));
    if copy_props && !string_intervals (string).is_null ()
      {
        copy_text_properties (make_fixnum (from_char), make_fixnum (to_char),
                              string, make_fixnum (0), res, Qnil ());
      }
    res
  }
}

pub fn subvector_safe (vec: Lisp_Object, from: ptrdiff_t, to: ptrdiff_t) -> Lisp_Object
{
  CHECK_VECTOR (vec);
  eassert! (0 <= from && from <= to && to <= ASIZE (vec));
  // SAFETY: `vec` is verified `VECTORP` and `from..to` is within `0..ASIZE(vec)`.
  unsafe { Fvector (to - from, aref_addr (vec, from)) }
}

DEFUN! ("substring", Fsubstring, Ssubstring, 1, 3, 0,
       doc: "Return a new string whose contents are a substring of STRING.\nThe returned string consists of the characters between index FROM\n(inclusive) and index TO (exclusive) of STRING.  FROM and TO are\nzero-indexed: 0 means the first character of STRING.  Negative values\nare counted from the end of STRING.  If TO is nil, the substring runs\nto the end of STRING.\n\nThe STRING argument may also be a vector.  In that case, the return\nvalue is a new vector that contains the elements between index FROM\n(inclusive) and index TO (exclusive) of that vector argument.\n\nWith one argument, just copy STRING (with properties, if any).",
       (string: Lisp_Object, from: Lisp_Object, to: Lisp_Object),
       safe {
  crate::safe_data::substring_subr (string, from, to)
});


DEFUN! ("substring-no-properties", Fsubstring_no_properties, Ssubstring_no_properties, 1, 3, 0,
       doc: "Return a substring of STRING, without text properties.\nIt starts at index FROM and ends before TO.\nTO may be nil or omitted; then the substring runs to the end of STRING.\nIf FROM is nil or omitted, the substring starts at the beginning of STRING.\nIf FROM or TO is negative, it counts from the end.\n\nWith one argument, just copy STRING without its properties.",
       (string: Lisp_Object, from: Lisp_Object, to: Lisp_Object),
       safe {
  crate::safe_data::substring_no_properties (string, from, to)
});

/* Extract a substring of STRING, giving start and end positions
   both in characters and in bytes.  */

/* C 1648-1669: `substring_both`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn substring_both (string: Lisp_Object, from: ptrdiff_t, from_byte: ptrdiff_t,
			      to: ptrdiff_t, to_byte: ptrdiff_t) -> Lisp_Object
{
  crate::safe_data::substring_both_impl (string, from, from_byte, to, to_byte)
}

DEFUN! ("take", Ftake, Stake, 2, 2, 0,
       doc: "Return the first N elements of LIST.\nIf N is zero or negative, return nil.\nIf N is greater or equal to the length of LIST, return LIST (or a copy).",
       (n: Lisp_Object, list: Lisp_Object),
       safe {
  crate::safe_data::take_subr (n, list)
});

DEFUN! ("ntake", Fntake, Sntake, 2, 2, 0,
       doc: "Modify LIST to keep only the first N elements.\nIf N is zero or negative, return nil.\nIf N is greater or equal to the length of LIST, return LIST unmodified.\nOtherwise, return LIST after truncating it.",
       (n: Lisp_Object, list: Lisp_Object),
       safe {
  crate::safe_data::ntake_subr (n, list)
});

DEFUN! ("nthcdr", Fnthcdr, Snthcdr, 2, 2, 0,
       doc: "Take cdr N times on LIST, return the result.  ",
       (n: Lisp_Object, list: Lisp_Object),
       {
  let mut tail: Lisp_Object = list;

  /* A huge but in-range EMACS_INT that can be substituted for a
     positive bignum while counting down.  It does not introduce
     miscounts because a list or cycle cannot possibly be this long,
     and any counting error is fixed up later.  */
  let large_num: EMACS_INT = EMACS_INT_MAX;

  let mut num: EMACS_INT;
  if FIXNUMP (n)
    {
      num = XFIXNUM (n);

      /* Speed up small lists by omitting circularity and quit checking.  */
      if num <= SMALL_LIST_LEN_MAX as EMACS_INT
	{
	  while 0 < num
	    {
	      if ! CONSP (tail)
		{
		  CHECK_LIST_END (tail, list);
		  return Qnil ();
		}
	      num -= 1;
	      tail = XCDR (tail);
	    }
	  return tail;
	}
    }
  else
    {
      CHECK_INTEGER (n);
      if mpz_sgn (xbignum_val (n)) < 0
	{ return tail; }
      num = large_num;
    }

  let mut tortoise_num: EMACS_INT = num;
  let mut saved_tail: Lisp_Object = tail;
  /* C's `FOR_EACH_TAIL_SAFE (tail) { … li.tortoise … }` reads the macro's own iterator state
     from the body.  The port's `FOR_EACH_TAIL_SAFE!` cannot expose it (a `let` the macro
     introduces is not visible to call-site tokens: macro hygiene; docs/ports/fns.md,
     Decisions), so C's expansion (lisp.h 5873-5876 and the stepper at 5909-5912) is written
     out here, spelled exactly as the mirror spells it.  */
  let mut li: for_each_tail_internal = for_each_tail_internal { tortoise: tail, l: 0 };
  while CONSP (tail) || { likely (NILP (tail)); false }
    {
      /* If the tortoise just jumped (which is rare),
	 update TORTOISE_NUM accordingly.  */
      if BASE_EQ (tail, li.tortoise)
	{ tortoise_num = num; }

      saved_tail = XCDR (tail);
      num -= 1;
      if num == 0
	{ return saved_tail; }
      rarely_quit (num as u64);

      /* Stepper (lisp.h 5909-5912) with CHECK_QUIT false.  */
      tail = XCDR (tail);
      if BASE_EQ (tail, li.tortoise)
	{ tail = Qnil (); }
      else if ! likely (({ li.l += 1; li.l } & (FOR_EACH_TAIL_THRESHOLD - 1)) != 0)
	{
	  if false || (POWER_OF_2 (li.l) && { li.tortoise = tail; false })
	    { }
	}
    }

  tail = saved_tail;
  if ! CONSP (tail)
    {
      CHECK_LIST_END (tail, list);
      return Qnil ();
    }

  /* TAIL is part of a cycle.  Reduce NUM modulo the cycle length to
     avoid going around this cycle repeatedly.  */
  let cycle_length: intptr_t = tortoise_num - num;
  if ! FIXNUMP (n)
    {
      /* Undo any error introduced when LARGE_NUM was substituted for
	 N, by adding N - LARGE_NUM to NUM, using arithmetic modulo
	 CYCLE_LENGTH.  */
      /* Add N mod CYCLE_LENGTH to NUM.  */
      if cycle_length <= ULONG_MAX as intptr_t
	{ num += mpz_tdiv_ui (xbignum_val (n), cycle_length as c_ulong) as EMACS_INT; }
      else
	{
	  mpz_set_intmax ((*mpz.as_mut_ptr ())[0].as_mut_ptr (), cycle_length as intmax_t);
	  mpz_tdiv_r ((*mpz.as_mut_ptr ())[0].as_mut_ptr (), xbignum_val (n), (*mpz.as_mut_ptr ())[0].as_ptr ());
	  let mut iz: intptr_t = 0;
	  mpz_export (&raw mut iz as *mut c_void, std::ptr::null_mut (), -1,
		      std::mem::size_of::<intptr_t> (), 0, 0, (*mpz.as_mut_ptr ())[0].as_ptr ());
	  num += iz;
	}
      if num < cycle_length
	{ num += cycle_length; }
      num -= large_num % cycle_length;
      eassume! (num >= 0);
    }
  num %= cycle_length;

  /* One last time through the cycle.  */
  while 0 < num
    {
      tail = XCDR (tail);
      rarely_quit (num as u64);
      num -= 1;
    }
  return tail;
});

DEFUN! ("nth", Fnth, Snth, 2, 2, 0,
       doc: "Return the Nth element of LIST.\nN counts from zero.  If LIST is not that long, nil is returned.",
       (n: Lisp_Object, list: Lisp_Object),
       safe {
  crate::safe_data::nth_subr (n, list)
});

DEFUN! ("elt", Felt, Selt, 2, 2, 0,
       doc: "Return element of SEQUENCE at index N.",
       (sequence: Lisp_Object, n: Lisp_Object),
       safe {
  crate::safe_data::elt_subr (sequence, n)
});

/* C 1866-1876: the `double_and_words` union and its `WORDS_PER_DOUBLE` size.  */
pub const WORDS_PER_DOUBLE: usize = (std::mem::size_of::<f64> () / std::mem::size_of::<EMACS_UINT> ()
				     + (std::mem::size_of::<f64> () % std::mem::size_of::<EMACS_UINT> () != 0)
				       as usize);
#[repr(C)]
#[derive(Clone, Copy)]
pub union double_and_words
{
  pub val: f64,
  pub word: [EMACS_UINT; WORDS_PER_DOUBLE],
}

/* C 1877-1889: `same_float`.  */
#[inline(always)]
fn same_float (x: Lisp_Object, y: Lisp_Object) -> bool
{
  XFLOAT_DATA (x).to_bits () == XFLOAT_DATA (y).to_bits ()
}

/* True if X can be compared using `eq'.
   This predicate is approximative, for maximum speed.  */
/* C 1891-1894: `eq_comparable_value`.  */
#[inline(always)]
fn eq_comparable_value (x: Lisp_Object) -> bool
{
  SYMBOLP (x) || FIXNUMP (x)
}

DEFUN! ("member", Fmember, Smember, 2, 2, 0,
       doc: "Return non-nil if ELT is an element of LIST.  Comparison done with `equal'.\nThe value is actually the tail of LIST whose car is ELT.",
       (elt: Lisp_Object, list: Lisp_Object),
       safe {
  crate::safe_data::member_subr (elt, list)
});

DEFUN! ("memq", Fmemq, Smemq, 2, 2, 0,
       doc: "Return non-nil if ELT is an element of LIST.  Comparison done with `eq'.\nThe value is actually the tail of LIST whose car is ELT.",
       (elt: Lisp_Object, list: Lisp_Object),
       safe {
  crate::safe_data::memq_subr (elt, list)
});

/* C 1925-1931: `memq_no_quit`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn memq_no_quit (elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object
{
  crate::safe_data::memq_no_quit_impl (elt, list)
}

DEFUN! ("memql", Fmemql, Smemql, 2, 2, 0,
       doc: "Return non-nil if ELT is an element of LIST.  Comparison done with `eql'.\nThe value is actually the tail of LIST whose car is ELT.",
       (elt: Lisp_Object, list: Lisp_Object),
       safe {
  crate::safe_data::memql_subr (elt, list)
});

DEFUN! ("assq", Fassq, Sassq, 2, 2, 0,
       doc: "Return non-nil if KEY is `eq' to the car of an element of ALIST.\nThe value is actually the first element of ALIST whose car is KEY.\nElements of ALIST that are not conses are ignored.",
       (key: Lisp_Object, alist: Lisp_Object),
       safe {
  crate::safe_data::assq_subr (key, alist)
});

/* Like Fassq but never report an error and do not allow quits.
   Use only on objects known to be non-circular lists.  */

/* C 1984-1995: `assq_no_quit`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn assq_no_quit (key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object
{
  crate::safe_data::assq_no_quit_impl (key, alist)
}

/* Assq but doesn't signal.  Unlike assq_no_quit, this function still
   detects circular lists; like assq_no_quit, this function does not
   allow quits and never signals.  If anything goes wrong, it returns
   Qnil.  */
/* C 1997-2011: `assq_no_signal`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn assq_no_signal (key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object
{
  crate::safe_data::assq_no_signal_impl (key, alist)
}

DEFUN! ("assoc", Fassoc, Sassoc, 2, 3, 0,
       doc: "Return non-nil if KEY is equal to the car of an element of ALIST.\nThe value is actually the first element of ALIST whose car equals KEY.\n\nEquality is defined by the function TESTFN, defaulting to `equal'.\nTESTFN is called with 2 arguments: a car of an alist element and KEY.  ",
       (key: Lisp_Object, alist: Lisp_Object, testfn: Lisp_Object),
       safe {
  crate::safe_data::assoc_subr (key, alist, testfn)
});

/* Like Fassoc but never report an error and do not allow quits.
   Use only on keys and lists known to be non-circular, and on keys
   that are not too deep and are not window configurations.  */

/* C 2037-2047: `assoc_no_quit`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn assoc_no_quit (key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object
{
  let mut alist: Lisp_Object = alist;
  while ! NILP (alist)
    {
      let car: Lisp_Object = XCAR (alist);
      if CONSP (car)
	  && (EQ (XCAR (car), key) || equal_no_quit (XCAR (car), key))
	{ return car; }
      alist = XCDR (alist);
    }
  return Qnil ();
}

DEFUN! ("rassq", Frassq, Srassq, 2, 2, 0,
       doc: "Return non-nil if KEY is `eq' to the cdr of an element of ALIST.\nThe value is actually the first element of ALIST whose cdr is KEY.",
       (key: Lisp_Object, alist: Lisp_Object),
       safe {
  crate::safe_data::rassq_subr (key, alist)
});

DEFUN! ("rassoc", Frassoc, Srassoc, 2, 2, 0,
       doc: "Return non-nil if KEY is `equal' to the cdr of an element of ALIST.\nThe value is actually the first element of ALIST whose cdr equals KEY.",
       (key: Lisp_Object, alist: Lisp_Object),
       safe {
  crate::safe_data::rassoc_subr (key, alist)
});


DEFUN! ("delq", Fdelq, Sdelq, 2, 2, 0,
       doc: "Delete members of LIST which are `eq' to ELT, and return the result.\nMore precisely, this function skips any members `eq' to ELT at the\nfront of LIST, then removes members `eq' to ELT from the remaining\nsublist by modifying its list structure, then returns the resulting\nlist.\n\nWrite `(setq foo (delq element foo))' to be sure of correctly changing\nthe value of a list `foo'.  See also `remq', which does not modify the\nargument.",
       (elt: Lisp_Object, list: Lisp_Object),
       safe {
  crate::safe_data::delq_subr (elt, list)
});

/* Like Fdelq but do not report errors and neither quit nor process
   signals.  Use only on objects known to be non-circular lists.  */
/* C 2115-2135: `delq_no_quit`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn delq_no_quit (elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object
{
  crate::safe_data::delq_no_quit_impl (elt, list)
}

pub fn delete_string_char_safe (seq: Lisp_Object, target_char: c_int) -> Lisp_Object
{
  CHECK_STRING (seq);
  // SAFETY: `seq` is verified `STRINGP`; scans valid character boundaries and copies remaining bytes into a fresh string.
  unsafe {
    let mut i: ptrdiff_t = 0;
    let mut ibyte: ptrdiff_t = 0;
    let mut nchars: ptrdiff_t = 0;
    let mut nbytes: ptrdiff_t = 0;
    let mut cbytes: ptrdiff_t;
    let mut c: c_int;

    while i < SCHARS (seq)
      {
        if STRING_MULTIBYTE (seq)
          {
            c = STRING_CHAR (SDATA (seq).offset (ibyte)) as c_int;
            cbytes = CHAR_BYTES (c) as ptrdiff_t;
          }
        else
          {
            c = SREF (seq, i as usize) as c_int;
            cbytes = 1;
          }
        if c != target_char
          {
            nchars += 1;
            nbytes += cbytes;
          }
        i += 1;
        ibyte += cbytes;
      }

    if nchars == SCHARS (seq)
      { return seq; }

    let mut tem: Lisp_Object = make_uninit_multibyte_string (nchars, nbytes);
    if !STRING_MULTIBYTE (seq)
      { STRING_SET_UNIBYTE (&raw mut tem); }

    i = 0;
    nbytes = 0;
    ibyte = 0;
    while i < SCHARS (seq)
      {
        if STRING_MULTIBYTE (seq)
          {
            c = STRING_CHAR (SDATA (seq).offset (ibyte)) as c_int;
            cbytes = CHAR_BYTES (c) as ptrdiff_t;
          }
        else
          {
            c = SREF (seq, i as usize) as c_int;
            cbytes = 1;
          }
        if c != target_char
          {
            std::ptr::copy_nonoverlapping (SDATA (seq).offset (ibyte),
                                           SDATA (tem).offset (nbytes),
                                           cbytes as usize);
            nbytes += cbytes;
          }
        i += 1;
        ibyte += cbytes;
      }
    tem
  }
}

pub fn reverse_multibyte_string_safe (seq: Lisp_Object) -> Lisp_Object
{
  CHECK_STRING (seq);
  let size: ptrdiff_t = SCHARS (seq);
  let bytes: ptrdiff_t = SBYTES (seq);
  // SAFETY: `seq` is verified multibyte `STRINGP`; reverses each internal multibyte codepoint into `new`.
  unsafe {
    let new: Lisp_Object = make_uninit_multibyte_string (size, bytes);
    let mut p: *mut u8 = SDATA (seq);
    let mut q: *mut u8 = SDATA (new).offset (bytes);
    while q > SDATA (new)
      {
        let mut len: c_int = 0;
        let ch: c_int = string_char_and_length (p, &raw mut len);
        p = p.offset (len as isize);
        q = q.offset (-(len as isize));
        CHAR_STRING (ch, q);
      }
    new
  }
}

DEFUN! ("delete", Fdelete, Sdelete, 2, 2, 0,
       doc: "Delete members of SEQ which are `equal' to ELT, and return the result.\nSEQ must be a sequence (i.e. a list, a vector, or a string).\nThe return value is a sequence of the same type.\n\nIf SEQ is a list, this behaves like `delq', except that it compares\nwith `equal' instead of `eq'.  In particular, it may remove elements\nby altering the list structure.\n\nIf SEQ is not a list, deletion is never performed destructively;\ninstead this function creates and returns a new vector or string.\n\nWrite `(setq foo (delete element foo))' to be sure of correctly\nchanging the value of a sequence `foo'.  See also `remove', which\ndoes not modify the argument.",
       (elt: Lisp_Object, seq: Lisp_Object),
       safe {
  crate::safe_data::delete_subr (elt, seq)
});

DEFUN! ("nreverse", Fnreverse, Snreverse, 1, 1, 0,
       doc: "Reverse order of items in a list, vector or string SEQ.\nIf SEQ is a list, it should be nil-terminated.\nThis function may destructively modify SEQ to produce the value.",
       (seq: Lisp_Object),
       safe {
  crate::safe_data::nreverse_subr (seq)
});

DEFUN! ("reverse", Freverse, Sreverse, 1, 1, 0,
       doc: "Return the reversed copy of list, vector, or string SEQ.\nSee also the function `nreverse', which is used more often.",
       (seq: Lisp_Object),
       safe {
  crate::safe_data::reverse_subr (seq)
});


/* Stably sort LIST ordered by PREDICATE and KEYFUNC, optionally reversed.
   This converts the list to a vector, sorts the vector, and returns the
   result converted back to a list.  If INPLACE, the input list is
   reused to hold the sorted result; otherwise a new list is returned.  */
/* C 2392-2436: `sort_list`.  */
unsafe fn sort_list (list: Lisp_Object, predicate: Lisp_Object, keyfunc: Lisp_Object,
		     reverse: bool, inplace: bool) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut list: Lisp_Object = list;
  let mut length: ptrdiff_t = 0;
  let pred_resolved: Lisp_Object = if SYMBOLP (predicate)
    { (*XSYMBOL (predicate)).u.s.function }
    else { predicate };
  if inplace && !reverse && !(*debug_on_next_call.as_mut_ptr ())
     && (NILP (keyfunc) || EQ (keyfunc, Qidentity ()))
     && (NILP (predicate) || EQ (predicate, Qvaluelt ())
	 || (SUBRP (pred_resolved) && (*XSUBR (pred_resolved)).function.aMANY == Some (Flss)))
    {
      let mut cur: Lisp_Object = list;
      let mut tortoise: Lisp_Object = list;
      let mut sorted_fixnums: bool = true;
      let mut prev_val: EMACS_INT = MOST_NEGATIVE_FIXNUM;
      while CONSP (cur)
	{
	  if sorted_fixnums
	    {
	      let elt: Lisp_Object = XCAR (cur);
	      if FIXNUMP (elt)
		{
		  let v: EMACS_INT = XFIXNUM (elt);
		  if v < prev_val
		    { sorted_fixnums = false; }
		  else
		    { prev_val = v; }
		}
	      else
		{ sorted_fixnums = false; }
	    }
	  length += 1;
	  cur = XCDR (cur);
	  if BASE_EQ (cur, tortoise)
	    { circular_list (cur); }
	  else if (length & (FOR_EACH_TAIL_THRESHOLD as ptrdiff_t - 1)) == 0
	    {
	      maybe_quit ();
	      if (length & (length - 1)) == 0
		{ tortoise = cur; }
	    }
	}
      CHECK_LIST_END (cur, cur);
      if sorted_fixnums
	{
	  maybe_quit ();
	  return list;
	}
    }
  else
    {
      length = list_length (list);
    }
  if length < 2
    { return if inplace { list } else { list1 (XCAR (list)) }; }
  else
    {
      let mut result: *mut Lisp_Object;
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
      SAFE_ALLOCA_LISP! (sa_avail, result, length);
      let mut tail: Lisp_Object = list;
      let mut i: ptrdiff_t = 0;
      while i < length
	{
	  *result.offset (i) = XCAR (tail);
	  tail = XCDR (tail);
	  i += 1;
	}
      tim_sort (predicate, keyfunc, result, length, reverse);

      if inplace
	{
	  /* Copy sorted vector contents back onto the original list.  */
	  let mut i: ptrdiff_t = 0;
	  tail = list;
	  while CONSP (tail)
	    {
	      XSETCAR (tail, *result.offset (i));
	      tail = XCDR (tail);
	      i += 1;
	    }
	}
      else
	{
	  /* Create a new list for the sorted vector contents.  */
	  list = Qnil ();
	  let mut i: ptrdiff_t = length - 1;
	  while i >= 0
	    {
	      list = Fcons (*result.offset (i), list);
	      i -= 1;
	    }
	}
      SAFE_FREE! (sa_count, sa_mark);
      return list;
    }

  }
}

/* Stably sort VECTOR in-place ordered by PREDICATE and KEYFUNC,
   optionally reversed.  */
/* C 2438-2445: `sort_vector`.  */
unsafe fn sort_vector (vector: Lisp_Object, predicate: Lisp_Object, keyfunc: Lisp_Object,
		       reverse: bool) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let length: ptrdiff_t = ASIZE (vector);
  if length >= 2
    { tim_sort (predicate, keyfunc, (*XVECTOR (vector)).contents.as_mut_ptr (), length, reverse); }
  return vector;

  }
}

DEFUN! ("sort", Fsort, Ssort, 1, MANY, 0,
       doc: "Sort SEQ, stably, and return the sorted sequence.\nSEQ should be a list or vector.\nOptional arguments are specified as keyword/argument pairs.  The following\narguments are defined:\n\n:key FUNC -- FUNC is a function that takes a single element from SEQ and\n  returns the key value to be used in comparison.  If absent or nil,\n  `identity' is used.\n\n:lessp FUNC -- FUNC is a function that takes two arguments and returns\n  non-nil if the first element should come before the second.\n  If absent or nil, `value<' is used.\n\n:reverse BOOL -- if BOOL is non-nil, the sorting order implied by FUNC is\n  reversed.  This does not affect stability: equal elements still retain\n  their order in the input sequence.\n\n:in-place BOOL -- if BOOL is non-nil, SEQ is sorted in-place and returned.\n  Otherwise, a sorted copy of SEQ is returned and SEQ remains unmodified;\n  this is the default.\n\nFor compatibility, the calling convention (sort SEQ LESSP) can also be used;\nin this case, sorting is always done in-place.\n\nusage: (sort SEQ &key KEY LESSP REVERSE IN-PLACE)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let seq: Lisp_Object = *args.offset (0);
  let mut key: Lisp_Object = Qnil ();
  let mut lessp: Lisp_Object = Qnil ();
  let mut inplace: bool = false;
  let mut reverse: bool = false;
  if nargs == 2
    {
      /* old-style invocation without keywords */
      lessp = *args.offset (1);
      inplace = true;
    }
  else if (nargs & 1) == 0
    { error (c"Invalid argument list".as_ptr ()); }
  else
    {
      let mut i: ptrdiff_t = 1;
      while i < nargs - 1
	{
	  if EQ (*args.offset (i), QCkey ())
	    { key = *args.offset (i + 1); }
	  else if EQ (*args.offset (i), QClessp ())
	    { lessp = *args.offset (i + 1); }
	  else if EQ (*args.offset (i), QCin_place ())
	    { inplace = !NILP (*args.offset (i + 1)); }
	  else if EQ (*args.offset (i), QCreverse ())
	    { reverse = !NILP (*args.offset (i + 1)); }
	  else
	    { signal_error (c"Invalid keyword argument".as_ptr (), *args.offset (i)); }
	  i += 2;
	}
    }

  if CONSP (seq)
    { return sort_list (seq, lessp, key, reverse, inplace); }
  else if NILP (seq)
    { return seq; }
  else if VECTORP (seq)
    { return sort_vector (if inplace { seq } else { Fcopy_sequence (seq) },
			  lessp, key, reverse); }
  else
    { wrong_type_argument (Qlist_or_vector_p (), seq); }
});

/* C 2515-2558: `merge`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn merge (org_l1: Lisp_Object, org_l2: Lisp_Object,
				pred: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut org_l1: Lisp_Object = org_l1;
  let mut org_l2: Lisp_Object = org_l2;
  let mut l1: Lisp_Object = org_l1;
  let mut l2: Lisp_Object = org_l2;
  let mut tail: Lisp_Object = Qnil ();
  let mut value: Lisp_Object = Qnil ();

  loop
    {
      if NILP (l1)
	{
	  if NILP (tail)
	    { return l2; }
	  Fsetcdr (tail, l2);
	  return value;
	}
      if NILP (l2)
	{
	  if NILP (tail)
	    { return l1; }
	  Fsetcdr (tail, l1);
	  return value;
	}

      let tem: Lisp_Object;
      if !NILP (calln! (pred, Fcar (l1), Fcar (l2)))
	{
	  tem = l1;
	  l1 = Fcdr (l1);
	  org_l1 = l1;
	}
      else
	{
	  tem = l2;
	  l2 = Fcdr (l2);
	  org_l2 = l2;
	}
      if NILP (tail)
	{ value = tem; }
      else
	{ Fsetcdr (tail, tem); }
      tail = tem;
    }

  }
}

/* C 2561-2603: `merge_c`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn merge_c (org_l1: Lisp_Object, org_l2: Lisp_Object,
				  less: Option<unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object)
					      -> bool>)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut org_l1: Lisp_Object = org_l1;
  let mut org_l2: Lisp_Object = org_l2;
  let mut l1: Lisp_Object = org_l1;
  let mut l2: Lisp_Object = org_l2;
  let mut tail: Lisp_Object = Qnil ();
  let mut value: Lisp_Object = Qnil ();

  loop
    {
      if NILP (l1)
	{
	  if NILP (tail)
	    { return l2; }
	  Fsetcdr (tail, l2);
	  return value;
	}
      if NILP (l2)
	{
	  if NILP (tail)
	    { return l1; }
	  Fsetcdr (tail, l1);
	  return value;
	}

      let tem: Lisp_Object;
      if less.unwrap () (Fcar (l1), Fcar (l2))
	{
	  tem = l1;
	  l1 = Fcdr (l1);
	  org_l1 = l1;
	}
      else
	{
	  tem = l2;
	  l2 = Fcdr (l2);
	  org_l2 = l2;
	}
      if NILP (tail)
	{ value = tem; }
      else
	{ Fsetcdr (tail, tem); }
      tail = tem;
    }

  }
}


/* This does not check for quits.  That is safe since it must terminate.  */

#[cold]
#[inline(never)]
unsafe fn fplist_get_pred_slow (plist: Lisp_Object, prop: Lisp_Object, predicate: Lisp_Object) -> Lisp_Object
{
  let mut tail: Lisp_Object = plist;
  FOR_EACH_TAIL_SAFE! (tail, {
      if ! CONSP (XCDR (tail))
	{ break; }
      if !NILP (calln! (predicate, XCAR (tail), prop))
	{ return XCAR (XCDR (tail)); }
      tail = XCDR (tail);
  });

  return Qnil ();
}

DEFUN! ("plist-get", Fplist_get, Splist_get, 2, 3, 0,
       doc: "Extract a value from a property list.\nPLIST is a property list, which is a list of the form\n(PROP1 VALUE1 PROP2 VALUE2...).\n\nThis function returns the value corresponding to the given PROP, or\nnil if PROP is not one of the properties on the list.  The comparison\nwith PROP is done using PREDICATE, which defaults to `eq'.\n\nThis function doesn't signal an error if PLIST is invalid.",
       (plist: Lisp_Object, prop: Lisp_Object, predicate: Lisp_Object),
       {
  if NILP (predicate)
    { return plist_get_inline (plist, prop); }

  return fplist_get_pred_slow (plist, prop, predicate);
});

#[cold]
#[inline(never)]
unsafe fn plist_get_slow (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object
{
  let mut tail: Lisp_Object = plist;
  FOR_EACH_TAIL_SAFE! (tail, {
      if ! CONSP (XCDR (tail))
	{ break; }
      if EQ (XCAR (tail), prop)
	{ return XCAR (XCDR (tail)); }
      tail = XCDR (tail);
  });
  return Qnil ();
}

#[inline(always)]
unsafe fn plist_get_inline (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if NILP (plist)
    { return Qnil (); }
  let mut tail: Lisp_Object = plist;
  if ! (*symbols_with_pos_enabled.as_mut_ptr ())
    {
      let mut steps: ptrdiff_t = 0;
      while CONSP (tail) && steps < 2048
	{
	  let cdr: Lisp_Object = XCDR (tail);
	  if ! CONSP (cdr)
	    { return Qnil (); }
	  if BASE_EQ (XCAR (tail), prop)
	    { return XCAR (cdr); }
	  tail = XCDR (cdr);
	  steps += 1;
	}
      if NILP (tail)
	{ return Qnil (); }
    }
  return plist_get_slow (plist, prop);

  }
}

/* Faster version of Fplist_get that works with EQ only.  */
/* C 2639-2651: `plist_get`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn plist_get (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  return plist_get_inline (plist, prop);

  }
}

#[cold]
#[inline(never)]
unsafe fn fget_overriding_slow (symbol: Lisp_Object, propname: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let propval: Lisp_Object = plist_get_inline (CDR (Fassq (symbol,
							   (*Voverriding_plist_environment.as_mut_ptr ()))),
					       propname);
  if !NILP (propval)
    { return propval; }
  return plist_get_inline ((*XSYMBOL (symbol)).u.s.plist, propname);

  }
}

DEFUN! ("get", Fget, Sget, 2, 2, 0,
       doc: "Return the value of SYMBOL's PROPNAME property.\nThis is the last value stored with `(put SYMBOL PROPNAME VALUE)'.",
       (symbol: Lisp_Object, propname: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  if ! NILP ((*Voverriding_plist_environment.as_mut_ptr ()))
    { return fget_overriding_slow (symbol, propname); }
  return plist_get_inline ((*XSYMBOL (symbol)).u.s.plist, propname);
});

DEFUN! ("plist-put", Fplist_put, Splist_put, 3, 4, 0,
       doc: "Change value in PLIST of PROP to VAL.\nPLIST is a property list, which is a list of the form\n(PROP1 VALUE1 PROP2 VALUE2 ...).\n\nThe comparison with PROP is done using PREDICATE, which defaults to `eq'.\n\nIf PROP is already a property on the list, its value is set to VAL,\notherwise the new PROP VAL pair is added.  The new plist is returned;\nuse `(setq x (plist-put x prop val))' to be sure to use the new value.\nThe PLIST is modified by side effects.",
       (plist: Lisp_Object, prop: Lisp_Object, val: Lisp_Object, predicate: Lisp_Object),
       {
  if NILP (predicate)
    { return plist_put (plist, prop, val); }
  let mut prev: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object = plist;
  FOR_EACH_TAIL! (tail, {
      if ! CONSP (XCDR (tail))
	{ break; }

      if !NILP (calln! (predicate, XCAR (tail), prop))
	{
	  Fsetcar (XCDR (tail), val);
	  return plist;
	}

      prev = tail;
      tail = XCDR (tail);
  });
  CHECK_TYPE (NILP (tail), Qplistp (), plist);
  let newcell: Lisp_Object
    = Fcons (prop, Fcons (val, if NILP (prev) { plist } else { XCDR (XCDR (prev)) }));
  if NILP (prev)
    { return newcell; }
  Fsetcdr (XCDR (prev), newcell);
  return plist;
});

/* Faster version of Fplist_put that works with EQ only.  */
/* C 2708-2732: `plist_put`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn plist_put (plist: Lisp_Object, prop: Lisp_Object,
				    val: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut prev: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object = plist;
  if ! (*symbols_with_pos_enabled.as_mut_ptr ())
    {
      let mut steps: ptrdiff_t = 0;
      while CONSP (tail) && steps < 2048
	{
	  let cdr: Lisp_Object = XCDR (tail);
	  if ! CONSP (cdr)
	    { break; }
	  if BASE_EQ (XCAR (tail), prop)
	    {
	      XSETCAR (cdr, val);
	      return plist;
	    }
	  prev = tail;
	  tail = XCDR (cdr);
	  steps += 1;
	}
      if NILP (tail)
	{
	  let newcell: Lisp_Object
	    = Fcons (prop, Fcons (val, if NILP (prev) { plist } else { XCDR (XCDR (prev)) }));
	  if NILP (prev)
	    { return newcell; }
	  XSETCDR (XCDR (prev), newcell);
	  return plist;
	}
      prev = Qnil ();
      tail = plist;
    }
  FOR_EACH_TAIL! (tail, {
      if ! CONSP (XCDR (tail))
	{ break; }

      if EQ (XCAR (tail), prop)
	{
	  Fsetcar (XCDR (tail), val);
	  return plist;
	}

      prev = tail;
      tail = XCDR (tail);
  });
  CHECK_TYPE (NILP (tail), Qplistp (), plist);
  let newcell: Lisp_Object
    = Fcons (prop, Fcons (val, if NILP (prev) { plist } else { XCDR (XCDR (prev)) }));
  if NILP (prev)
    { return newcell; }
  Fsetcdr (XCDR (prev), newcell);
  return plist;

  }
}

DEFUN! ("put", Fput, Sput, 3, 3, 0,
       doc: "Store SYMBOL's PROPNAME property with value VALUE and return that value.\nIt can later be retrieved with `(get SYMBOL PROPNAME)'.",
       (symbol: Lisp_Object, propname: Lisp_Object, value: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  set_symbol_plist
    (symbol, plist_put ((*XSYMBOL (symbol)).u.s.plist, propname, value));
  return value;
});

DEFUN! ("plist-member", Fplist_member, Splist_member, 2, 3, 0,
       doc: "Return non-nil if PLIST has the property PROP.\nPLIST is a property list, which is a list of the form\n(PROP1 VALUE1 PROP2 VALUE2 ...).\n\nThe comparison with PROP is done using PREDICATE, which defaults to\n`eq'.\n\nUnlike `plist-get', this allows you to distinguish between a missing\nproperty and a property with the value nil.\nThe value is actually the tail of PLIST whose car is PROP.",
       (plist: Lisp_Object, prop: Lisp_Object, predicate: Lisp_Object),
       {
  if NILP (predicate)
    { return plist_member (plist, prop); }
  let mut tail: Lisp_Object = plist;
  FOR_EACH_TAIL! (tail, {
      if !NILP (calln! (predicate, XCAR (tail), prop))
	{ return tail; }
      tail = XCDR (tail);
      if ! CONSP (tail)
	{ break; }
  });
  CHECK_TYPE (NILP (tail), Qplistp (), plist);
  return Qnil ();
});

/* Faster version of Fplist_member that works with EQ only.  */
/* C 2775-2788: `plist_member`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn plist_member (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut tail: Lisp_Object = plist;
  FOR_EACH_TAIL! (tail, {
      if EQ (XCAR (tail), prop)
	{ return tail; }
      tail = XCDR (tail);
      if ! CONSP (tail)
	{ break; }
  });
  CHECK_TYPE (NILP (tail), Qplistp (), plist);
  return Qnil ();

  }
}

DEFUN! ("eql", Feql, Seql, 2, 2, 0,
       doc: "Return t if the two args are `eq' or are indistinguishable numbers.\nIntegers with the same value are `eql'.\nFloating-point values with the same sign, exponent and fraction are `eql'.\nThis differs from numeric comparison: (eql 0.0 -0.0) returns nil and\n(eql 0.0e+NaN 0.0e+NaN) returns t, whereas `=' does the opposite.",
       (obj1: Lisp_Object, obj2: Lisp_Object),
       {
  if FLOATP (obj1)
    { return if FLOATP (obj2) && same_float (obj1, obj2) { Qt () } else { Qnil () }; }
  else if BIGNUMP (obj1)
    { return if (BIGNUMP (obj2)
		 && mpz_cmp (xbignum_val (obj1), xbignum_val (obj2)) == 0)
	{ Qt () } else { Qnil () }; }
  else
    { return if EQ (obj1, obj2) { Qt () } else { Qnil () }; }
});

DEFUN! ("equal", Fequal, Sequal, 2, 2, 0,
       doc: "Return t if two Lisp objects have similar structure and contents.\nThey must have the same data type.\nConses are compared by comparing the cars and the cdrs.\nVectors and strings are compared element by element (so text properties\nof strings are ignored).\nNumbers are compared via `eql', so integers do not equal floats.\n(Use `=' if you want integers and floats to be able to be equal.)\nSymbols must match exactly.",
       (o1: Lisp_Object, o2: Lisp_Object),
       {
  if BASE_EQ (o1, o2)
    { return Qt (); }
  if ! (*symbols_with_pos_enabled.as_mut_ptr ())
     && (BARE_SYMBOL_P (o1) || FIXNUMP (o1) || BARE_SYMBOL_P (o2) || FIXNUMP (o2)
	 || XTYPE (o1) != XTYPE (o2))
    { return Qnil (); }
  return if internal_equal (o1, o2, EQUAL_PLAIN, 0, Qnil ()) { Qt () } else { Qnil () };
});

DEFUN! ("equal-including-properties", Fequal_including_properties, Sequal_including_properties, 2, 2, 0,
       doc: "Return t if two Lisp objects have similar structure and contents.\nThis is like `equal' except that it compares the text properties\nof strings.  (`equal' ignores text properties.)",
       (o1: Lisp_Object, o2: Lisp_Object),
       {
  return if internal_equal (o1, o2, EQUAL_INCLUDING_PROPERTIES, 0, Qnil ())
    { Qt () } else { Qnil () };
});

/* Return true if O1 and O2 are equal.  Do not quit or check for cycles.
   Use this only on arguments that are cycle-free and not too large and
   are not window configurations.  */

/* C 2837-2841: `equal_no_quit`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn equal_no_quit (o1: Lisp_Object, o2: Lisp_Object) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  return internal_equal (o1, o2, EQUAL_NO_QUIT, 0, Qnil ());

  }
}

/* Return true if O1 and O2 are equal.  EQUAL_KIND specifies what kind
   of equality test to use: if it is EQUAL_NO_QUIT, do not check for
   cycles or large arguments or quits; if EQUAL_PLAIN, do ordinary
   Lisp equality; and if EQUAL_INCLUDING_PROPERTIES, do
   equal-including-properties.

   If DEPTH is the current depth of recursion; signal an error if it
   gets too deep.  HT is a hash table used to detect cycles; if nil,
   it has not been allocated yet.  But ignore the last two arguments
   if EQUAL_KIND == EQUAL_NO_QUIT.  */

/* C 2861-3025: `internal_equal_1`.  */
unsafe fn internal_equal_1 (o1: Lisp_Object, o2: Lisp_Object, equal_kind: equal_kind,
			    depth: c_int, ht: *mut Lisp_Object) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut o1: Lisp_Object = o1;
  let mut o2: Lisp_Object = o2;
  let mut depth: c_int = depth;
  'tail_recurse: loop
    {
  if depth > 10
    {
      eassert! (equal_kind != EQUAL_NO_QUIT);
      if depth > 200
	{ error (c"Stack overflow in equal".as_ptr ()); }
      if NILP (*ht)
	{ *ht = CALLN! (Fmake_hash_table, QCtest (), Qeq ()); }
      match XTYPE (o1)
	{
	Lisp_Type::Lisp_Cons | Lisp_Type::Lisp_Vectorlike =>
	  {
	    let h: *mut Lisp_Hash_Table = XHASH_TABLE (*ht);
	    let hash: hash_hash_t = hash_from_key (h, o1);
	    let i: ptrdiff_t = hash_find_with_hash (h, o1, hash);
	    if i >= 0
	      { /* `o1' was seen already.  */
		let o2s: Lisp_Object = HASH_VALUE (h, i as hash_idx_t);
		if !NILP (Fmemq (o2, o2s))
		  { return true; }
		else
		  { set_hash_value_slot (h, i as hash_idx_t, Fcons (o2, o2s)); }
	      }
	    else
	      { hash_put (h, o1, Fcons (o2, Qnil ()), hash); }
	  }
	_ => { }
	}
    }

  /* A symbol with position compares the contained symbol, and is
     `equal' to the corresponding ordinary symbol.  */
  if BASE_EQ (o1, o2)
    { return true; }
  if (*symbols_with_pos_enabled.as_mut_ptr ())
    {
      o1 = maybe_remove_pos_from_symbol (o1);
      o2 = maybe_remove_pos_from_symbol (o2);
      if BASE_EQ (o1, o2)
	{ return true; }
    }
  if XTYPE (o1) != XTYPE (o2)
    { return false; }

  match XTYPE (o1)
    {
    Lisp_Type::Lisp_Float =>
      { return same_float (o1, o2); }

    Lisp_Type::Lisp_Cons =>
      {
      if equal_kind == EQUAL_NO_QUIT
	{
	  while CONSP (o1)
	    {
	      if ! CONSP (o2)
		{ return false; }
	      let c1: Lisp_Object = XCAR (o1);
	      let c2: Lisp_Object = XCAR (o2);
	      if ! BASE_EQ (c1, c2)
		{
		  if ! (*symbols_with_pos_enabled.as_mut_ptr ())
		     && (BARE_SYMBOL_P (c1) || FIXNUMP (c1) || BARE_SYMBOL_P (c2) || FIXNUMP (c2)
			 || XTYPE (c1) != XTYPE (c2))
		    { return false; }
		  if ! equal_no_quit (c1, c2)
		    { return false; }
		}
	      o2 = XCDR (o2);
	      if EQ (XCDR (o1), o2)
		{ return true; }
	      o1 = XCDR (o1);
	    }
	}
      else
	{
	  /* C's `FOR_EACH_TAIL_BASIC (o1,) { … FOR_EACH_TAIL_STEP_CYCLEP (o1, true) … }`: the
	     macro's body reads its own iterator state, which the port's macros cannot expose
	     (macro hygiene), so C's expansion (lisp.h 5873-5876 and 5909-5912) is written out
	     here, exactly as the mirror spells it.  */
	  let mut li: for_each_tail_internal = for_each_tail_internal { tortoise: o1, l: 0 };
	  while CONSP (o1) || { likely (NILP (o1)); false }
	    {
	      if ! CONSP (o2)
		{ return false; }
	      let c1: Lisp_Object = XCAR (o1);
	      let c2: Lisp_Object = XCAR (o2);
	      if ! BASE_EQ (c1, c2)
		{
		  if ! (*symbols_with_pos_enabled.as_mut_ptr ())
		     && (BARE_SYMBOL_P (c1) || FIXNUMP (c1) || BARE_SYMBOL_P (c2) || FIXNUMP (c2)
			 || XTYPE (c1) != XTYPE (c2))
		    { return false; }
		  if ! internal_equal_1 (c1, c2, equal_kind, depth + 1, ht)
		    { return false; }
		}
	      o2 = XCDR (o2);
	      if EQ (XCDR (o1), o2)
		{ return true; }

	      /* Stepper: C's `FOR_EACH_TAIL_STEP_CYCLEP (o1, true)` (lisp.h 5909-5912), as
		 the mirror spells it.  */
	      o1 = XCDR (o1);
	      let mut cyclep: bool = false;
	      if BASE_EQ (o1, li.tortoise)
		{ cyclep = true; }
	      else if ! likely (({ li.l += 1; li.l } & (FOR_EACH_TAIL_THRESHOLD - 1)) != 0)
		{
		  maybe_quit ();
		  if POWER_OF_2 (li.l)
		    { li.tortoise = o1; }
		}
	      if cyclep
		/* Cycle in o1; see if there is one in o2 as well.  */
		{ return (CONSP (o2)
			  && internal_equal_cycle (o2, o1, equal_kind, depth, ht)); }
	    }
	}
      depth += 1;
      continue 'tail_recurse;
      }

    Lisp_Type::Lisp_Vectorlike =>
      {
	let mut size: ptrdiff_t = ASIZE (o1);
	/* Pseudovectors have the type encoded in the size field, so this test
	   actually checks that the objects have the same type as well as the
	   same size.  */
	if ASIZE (o2) != size
	  { return false; }

	match PSEUDOVECTOR_TYPE (XVECTOR (o1))
	  {
	  pvec_type::PVEC_BIGNUM =>
	    { return mpz_cmp (xbignum_val (o1), xbignum_val (o2)) == 0; }

	  pvec_type::PVEC_OVERLAY =>
	    {
	    if OVERLAY_BUFFER (o1) != OVERLAY_BUFFER (o2)
		|| OVERLAY_START (o1) != OVERLAY_START (o2)
		|| OVERLAY_END (o1) != OVERLAY_END (o2)
	      { return false; }
	    o1 = (*XOVERLAY (o1)).plist;
	    o2 = (*XOVERLAY (o2)).plist;
	    depth += 1;
	    continue 'tail_recurse;
	    }

	  pvec_type::PVEC_MARKER =>
	    { return ((*XMARKER (o1)).buffer == (*XMARKER (o2)).buffer
		      && ((*XMARKER (o1)).buffer.is_null ()
			  || (*XMARKER (o1)).bytepos == (*XMARKER (o2)).bytepos)); }

	  pvec_type::PVEC_BOOL_VECTOR =>
	    {
	      let size: EMACS_INT = bool_vector_size (o1);
	      return (size == bool_vector_size (o2)
		      && memcmp (bool_vector_data (o1) as *const c_void,
				 bool_vector_data (o2) as *const c_void,
				 bool_vector_bytes (size) as usize) == 0);
	    }

	  /* C 2978-2981: `#ifdef HAVE_TREE_SITTER` (`undef` on this target) — carried text
	     (feature `have_tree_sitter`, P-51).  */
	  #[cfg (feature = "have_tree_sitter")]
	  pvec_type::PVEC_TS_NODE =>
	    { return treesit_node_eq (o1, o2); }

	  pvec_type::PVEC_SYMBOL_WITH_POS =>
	    {
	    eassert! (!(*symbols_with_pos_enabled.as_mut_ptr ()));
	    return (BASE_EQ (XSYMBOL_WITH_POS_SYM (o1),
			     XSYMBOL_WITH_POS_SYM (o2))
		    && BASE_EQ (XSYMBOL_WITH_POS_POS (o1),
				XSYMBOL_WITH_POS_POS (o2)));
	    }

	    /* Compare these element-wise.  */
	  pvec_type::PVEC_CLOSURE
	  | pvec_type::PVEC_CHAR_TABLE
	  | pvec_type::PVEC_SUB_CHAR_TABLE
	  | pvec_type::PVEC_RECORD
	  | pvec_type::PVEC_FONT
	  | pvec_type::PVEC_NORMAL_VECTOR =>
	    {
	      /* C's five-case group ends with `size &= PSEUDOVECTOR_SIZE_MASK; FALLTHROUGH;'
		 into `PVEC_NORMAL_VECTOR`; the port keeps one copy of the shared body and runs
		 the mask for the group (PORTING.md 2.2).  */
	      if (PSEUDOVECTOR_TYPE (XVECTOR (o1)) == pvec_type::PVEC_CLOSURE
		  || PSEUDOVECTOR_TYPE (XVECTOR (o1)) == pvec_type::PVEC_CHAR_TABLE
		  || PSEUDOVECTOR_TYPE (XVECTOR (o1)) == pvec_type::PVEC_SUB_CHAR_TABLE
		  || PSEUDOVECTOR_TYPE (XVECTOR (o1)) == pvec_type::PVEC_RECORD
		  || PSEUDOVECTOR_TYPE (XVECTOR (o1)) == pvec_type::PVEC_FONT)
		{ size &= PSEUDOVECTOR_SIZE_MASK; }

	      let mut i: ptrdiff_t = 0;
	      while i < size
		{
		  if !internal_equal_1 (AREF (o1, i), AREF (o2, i),
					equal_kind, depth + 1, ht)
		    { return false; }
		  i += 1;
		}
	      return true;
	    }

	  _ =>
	    { return false; }
	  }
      }

    Lisp_Type::Lisp_String =>
      { return (SCHARS (o1) == SCHARS (o2)
		&& SBYTES (o1) == SBYTES (o2)
		&& memcmp (SDATA (o1) as *const c_void, SDATA (o2) as *const c_void,
			   SBYTES (o1) as usize) == 0
		&& (equal_kind != EQUAL_INCLUDING_PROPERTIES
		    || compare_string_intervals (o1, o2))); }

    _ => { }
    }

  return false;
    }

  }
}

/* Slow path comparison when o1 and o2 are lists and o2 is circular.  */
/* C 3027-3052: `internal_equal_cycle`.  */
unsafe fn internal_equal_cycle (o1: Lisp_Object, o2: Lisp_Object, equal_kind: equal_kind,
				depth: c_int, ht: *mut Lisp_Object) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut o1: Lisp_Object = o1;
  let mut o2: Lisp_Object = o2;
  eassert! (CONSP (o1) && CONSP (o2));
  eassert! (equal_kind != EQUAL_NO_QUIT);

  /* C's `FOR_EACH_TAIL_BASIC (o1,) { … FOR_EACH_TAIL_STEP_CYCLEP (o1, true) … }`, expanded
     as in `internal_equal_1` above.  */
  let mut li: for_each_tail_internal = for_each_tail_internal { tortoise: o1, l: 0 };
  while CONSP (o1) || { likely (NILP (o1)); false }
    {
      if !CONSP (o2)
	{ return false; }
      if !internal_equal_1 (XCAR (o1), XCAR (o2), equal_kind, depth + 1, ht)
	{ return false; }
      o2 = XCDR (o2);
      if EQ (XCDR (o1), o2)
	{ return true; }

      /* Stepper (lisp.h 5909-5912) with CHECK_QUIT true.  */
      o1 = XCDR (o1);
      let mut cyclep: bool = false;
      if BASE_EQ (o1, li.tortoise)
	{ cyclep = true; }
      else if ! likely (({ li.l += 1; li.l } & (FOR_EACH_TAIL_THRESHOLD - 1)) != 0)
	{
	  maybe_quit ();
	  if POWER_OF_2 (li.l)
	    { li.tortoise = o1; }
	}
      if cyclep
	/* Cycle in o1 detected.  Since o2 is circular too and no
	   differences were found, they are equal.
	   (Their structures may differ but that's not relevant to `equal'.)  */
	{ return true; }
    }
  /* o1 terminated but o2 is circular: not equal.  */
  return false;

  }
}

/* C 3054-3060: `internal_equal`.  */
unsafe fn internal_equal (o1: Lisp_Object, o2: Lisp_Object, equal_kind: equal_kind,
			  depth: c_int, ht: Lisp_Object) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut ht: Lisp_Object = ht;
  return internal_equal_1 (o1, o2, equal_kind, depth, &raw mut ht);

  }
}

/* Return -1/0/1 for the </=/> lexicographic relation between bool-vectors.  */
/* C 3062-3092: `bool_vector_cmp`.  */
unsafe fn bool_vector_cmp (a: Lisp_Object, b: Lisp_Object) -> c_int
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut na: ptrdiff_t = bool_vector_size (a);
  let mut nb: ptrdiff_t = bool_vector_size (b);
  /* Skip equal words.  */
  let words_min: ptrdiff_t = min (na, nb) / BITS_PER_BITS_WORD as ptrdiff_t;
  let ad: *mut bits_word = bool_vector_data (a);
  let bd: *mut bits_word = bool_vector_data (b);
  let mut i: ptrdiff_t = 0;
  while i < words_min && *ad.offset (i) == *bd.offset (i)
    { i += 1; }
  na -= i * BITS_PER_BITS_WORD as ptrdiff_t;
  nb -= i * BITS_PER_BITS_WORD as ptrdiff_t;
  eassume! (na >= 0 && nb >= 0);
  if nb == 0
    { return if na != 0 { 1 } else { 0 }; }
  if na == 0
    { return -1; }

  let aw: bits_word = bits_word_to_host_endian (*ad.offset (i));
  let bw: bits_word = bits_word_to_host_endian (*bd.offset (i));
  let xw: bits_word = aw ^ bw;
  if xw == 0
    { return if na < nb { -1 } else { if na > nb { 1 } else { 0 } }; }

  let d: bits_word = xw & (xw.wrapping_neg ());	/* Isolate first difference.  */
  eassume! (d != 0);
  return if (d & aw) != 0 { 1 } else { -1 };

  }
}

/* Return -1 if a<b, 1 if a>b, 0 if a=b or if b is NaN (a must be a fixnum).  */
/* C 3094-3114: `fixnum_float_cmp`.  */
fn fixnum_float_cmp (a: EMACS_INT, b: f64) -> c_int
{
  let fa: f64 = a as f64;
  if fa == b
    {
      /* This doesn't mean that a=b because the conversion may have rounded.
	 However, b must be an integer that fits in an EMACS_INT,
	 because |b| <= 2|a| and EMACS_INT has at least one bit more than
	 needed to represent any fixnum.
	 Thus we can compare in the integer domain instead.  */
      let ib: EMACS_INT = b as EMACS_INT;		/* lossless conversion */
      return if a < ib { -1 } else { if a > ib { 1 } else { 0 } };
    }
  else
    { return if fa < b { -1 } else { if fa > b { 1 } else { 0 } }; }   /* return 0 if b is NaN */
}

/* Return -1, 0 or 1 to indicate whether a<b, a=b or a>b in the sense of value<.
   In particular 0 does not mean equality in the sense of Fequal, only
   that the arguments cannot be ordered yet they can be compared (same
   type).  */
/* C 3116-3274: `value_cmp`.  */
unsafe fn value_cmp (a: Lisp_Object, b: Lisp_Object, maxdepth: c_int) -> c_int
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut a: Lisp_Object = a;
  let mut b: Lisp_Object = b;
  if maxdepth < 0
    { error (c"Maximum depth exceeded in comparison".as_ptr ()); }

 'type_mismatch: {
  'tail_recurse: loop
    {
  /* Shortcut for a common case.  */
  if BASE_EQ (a, b)
    { return 0; }

  match XTYPE (a)
    {
    Lisp_Type::Lisp_Int0 | Lisp_Type::Lisp_Int1 =>
      {
	let ia: EMACS_INT = XFIXNUM (a);
	if FIXNUMP (b)
	  { return if ia < XFIXNUM (b) { -1 } else { 1 }; }   /* we know that a != b */
	if FLOATP (b)
	  { return fixnum_float_cmp (ia, XFLOAT_DATA (b)); }
	if BIGNUMP (b)
	  { return -mpz_sgn (xbignum_val (b)); }
      break 'type_mismatch;
      }

    Lisp_Type::Lisp_Symbol =>
      {
      if BARE_SYMBOL_P (b)
	{
	  return string_cmp ((*XBARE_SYMBOL (a)).u.s.name,
			     (*XBARE_SYMBOL (b)).u.s.name);
	}
      if CONSP (b) && NILP (a)
	{ return -1; }
      if SYMBOLP (b)
	/* Slow-path branch when B is a symbol-with-pos.  */
	{ return string_cmp ((*XBARE_SYMBOL (a)).u.s.name, (*XSYMBOL (b)).u.s.name); }
      break 'type_mismatch;
      }

    Lisp_Type::Lisp_String =>
      {
      if STRINGP (b)
	{ return string_cmp (a, b); }
      break 'type_mismatch;
      }

    Lisp_Type::Lisp_Cons =>
      {
      /* FIXME: Optimize for difference in the first element? */
      let mut b_loop: Lisp_Object = b;
      let mut b_end: Lisp_Object = b_loop;
      FOR_EACH_TAIL! (b_loop, {
	  let cmp: c_int = value_cmp (XCAR (a), XCAR (b_loop), maxdepth - 1);
	  if cmp != 0
	    { return cmp; }
	  a = XCDR (a);
	  if !CONSP (a)
	    {
	      b = XCDR (b_loop);
	      b_end = b;
	      continue 'tail_recurse;
	    }
	  b_end = XCDR (b_loop);
      });
      if NILP (b_end)
	{ return 1; }
      break 'type_mismatch;
      }

    Lisp_Type::Lisp_Vectorlike =>
      {
      if VECTORLIKEP (b)
	{
	  let ta: pvec_type = PSEUDOVECTOR_TYPE (XVECTOR (a));
	  let tb: pvec_type = PSEUDOVECTOR_TYPE (XVECTOR (b));
	  if ta == tb
	    {
	      match ta
		{
		pvec_type::PVEC_NORMAL_VECTOR | pvec_type::PVEC_RECORD =>
		  {
		    let mut len_a: ptrdiff_t = ASIZE (a);
		    let mut len_b: ptrdiff_t = ASIZE (b);
		    if ta == pvec_type::PVEC_RECORD
		      {
			len_a &= PSEUDOVECTOR_SIZE_MASK;
			len_b &= PSEUDOVECTOR_SIZE_MASK;
		      }
		    let len_min: ptrdiff_t = min (len_a, len_b);
		    let mut i: ptrdiff_t = 0;
		    while i < len_min
		      {
			let cmp: c_int = value_cmp (AREF (a, i), AREF (b, i),
						    maxdepth - 1);
			if cmp != 0
			  { return cmp; }
			i += 1;
		      }
		    return if len_a < len_b { -1 } else { if len_a > len_b { 1 } else { 0 } };
		  }

		pvec_type::PVEC_BOOL_VECTOR =>
		  { return bool_vector_cmp (a, b); }

		pvec_type::PVEC_MARKER =>
		  {
		    let buf_a: Lisp_Object = Fmarker_buffer (a);
		    let buf_b: Lisp_Object = Fmarker_buffer (b);
		    if NILP (buf_a)
		      { return if NILP (buf_b) { 0 } else { -1 }; }
		    if NILP (buf_b)
		      { return 1; }
		    let cmp: c_int = value_cmp (buf_a, buf_b, maxdepth - 1);
		    if cmp != 0
		      { return cmp; }
		    let pa: ptrdiff_t = (*XMARKER (a)).charpos;
		    let pb: ptrdiff_t = (*XMARKER (b)).charpos;
		    return if pa < pb { -1 } else { if pa > pb { 1 } else { 0 } };
		  }

		/* C 3223-3228: `#ifdef subprocesses` (live: `subprocesses` is 1).  */
		pvec_type::PVEC_PROCESS =>
		  {
		    a = Fprocess_name (a);
		    b = Fprocess_name (b);
		    continue 'tail_recurse;
		  }

		pvec_type::PVEC_BUFFER =>
		  {
		    /* Killed buffers lack names and sort before those alive.  */
		    let na: Lisp_Object = Fbuffer_name (a);
		    let nb: Lisp_Object = Fbuffer_name (b);
		    if NILP (na)
		      { return if NILP (nb) { 0 } else { -1 }; }
		    if NILP (nb)
		      { return 1; }
		    a = na;
		    b = nb;
		    continue 'tail_recurse;
		  }

		pvec_type::PVEC_BIGNUM =>
		  { return mpz_cmp (xbignum_val (a), xbignum_val (b)); }

		pvec_type::PVEC_SYMBOL_WITH_POS =>
		  {
		    /* Compare by name, enabled or not.  */
		    a = XSYMBOL_WITH_POS_SYM (a);
		    b = XSYMBOL_WITH_POS_SYM (b);
		    continue 'tail_recurse;
		  }

		_ =>
		  /* Treat other types as unordered.  */
		  { return 0; }
		}
	    }
	}
      /* C 3237-3248: these two are the `else if` of `if (VECTORLIKEP (b))`, not of the inner
	 `if (ta == tb)`.  Nested inside that block they were unreachable for a bignum on the
	 left and a fixnum on the right, and the arm fell off its end back into this loop
	 forever (P-171).  */
      else if BIGNUMP (a)
	{ return -value_cmp (b, a, maxdepth); }
      else if SYMBOL_WITH_POS_P (a) && (*symbols_with_pos_enabled.as_mut_ptr ())
	{
	  a = XSYMBOL_WITH_POS_SYM (a);
	  continue 'tail_recurse;
	}

      break 'type_mismatch;
      }

    Lisp_Type::Lisp_Float =>
      {
	let fa: f64 = XFLOAT_DATA (a);
	if FLOATP (b)
	  { return if fa < XFLOAT_DATA (b) { -1 } else { if fa > XFLOAT_DATA (b) { 1 } else { 0 } }; }
	if FIXNUMP (b)
	  { return -fixnum_float_cmp (XFIXNUM (b), fa); }
	if BIGNUMP (b)
	  {
	    if isnan! (fa)
	      { return 0; }
	    return -mpz_cmp_d (xbignum_val (b), fa);
      break 'type_mismatch;
	  }
      }

    _ =>
      { eassume! (false); }
    }
    }
 }
  xsignal2 (Qtype_mismatch (), a, b);

  }
}

DEFUN! ("value<", Fvaluelt, Svaluelt, 2, 2, 0,
       doc: "Return non-nil if A precedes B in standard value order.\nA and B must have the same basic type.\nNumbers are compared with `<'.\nStrings and symbols are compared with `string-lessp'.\nLists, vectors, bool-vectors and records are compared lexicographically.\nMarkers are compared lexicographically by buffer and position.\nBuffers and processes are compared by name.\nOther types are considered unordered and the return value will be `nil'.",
       (a: Lisp_Object, b: Lisp_Object),
       {
  let maxdepth: c_int = 200;		  /* FIXME: arbitrary value */
  return if value_cmp (a, b, maxdepth) < 0 { Qt () } else { Qnil () };
});

DEFUN! ("fillarray", Ffillarray, Sfillarray, 2, 2, 0,
       doc: "Store each element of ARRAY with ITEM.\nARRAY is a vector, string, char-table, or bool-vector.",
       (array: Lisp_Object, item: Lisp_Object),
       {
  let mut size: ptrdiff_t;
  let mut idx: ptrdiff_t;

  if VECTORP (array)
    {
      idx = 0;
      size = ASIZE (array);
      while idx < size
	{
	  ASET (array, idx, item);
	  idx += 1;
	}
    }
  else if CHAR_TABLE_P (array)
    {
      let mut i: c_int;

      i = 0;
      while i < (1 << CHARTAB_SIZE_BITS_0)
	{
	  set_char_table_contents (array, i as isize, item);
	  i += 1;
	}
      set_char_table_defalt (array, item);
    }
  else if STRINGP (array)
    {
      let mut p: *mut u8 = SDATA (array);
      CHECK_CHARACTER (item);
      let charval: c_int = XFIXNAT (item) as c_int;
      size = SCHARS (array);
      if size != 0
	{
	  let mut str_: [u8; MAX_MULTIBYTE_LENGTH as usize]
	    = [0; MAX_MULTIBYTE_LENGTH as usize];
	  let len: c_int;
	  if STRING_MULTIBYTE (array)
	    { len = CHAR_STRING (charval, str_.as_mut_ptr ()); }
	  else
	    {
	      str_[0] = charval as u8;
	      len = 1;
	    }

	  let size_byte: ptrdiff_t = SBYTES (array);
	  if len == 1 && size == size_byte
	    { memset (p as *mut c_void, str_[0] as c_int, size as usize); }
	  else
	    {
	      let mut product: ptrdiff_t = 0;
	      let overflow: bool = (size as i64).overflowing_mul (len as i64).1 as bool;
	      product = (size as i64).overflowing_mul (len as i64).0 as ptrdiff_t;
	      if overflow || product != size_byte
		{ error (c"Attempt to change byte length of a string".as_ptr ()); }
	      idx = 0;
	      while idx < size_byte
		{
		  *p = str_[(idx % len as ptrdiff_t) as usize];
		  p = p.offset (1);
		  idx += 1;
		}
	    }
	}
    }
  else if BOOL_VECTOR_P (array)
    { return bool_vector_fill (array, item); }
  else
    { wrong_type_argument (Qarrayp (), array); }
  return array;
});

DEFUN! ("clear-string", Fclear_string, Sclear_string, 1, 1, 0,
       doc: "Clear the contents of STRING.\nThis makes STRING unibyte, clears its contents to null characters, and\nremoves all text properties.  This may change its length.",
       (string: Lisp_Object),
       {
  let mut string: Lisp_Object = string;
  CHECK_STRING (string);
  let len: ptrdiff_t = SBYTES (string);
  Fset_text_properties (make_fixnum (0), make_fixnum (SCHARS (string)),
			Qnil (), string);
  if len != 0 || STRING_MULTIBYTE (string)
    {
      memset (SDATA (string) as *mut c_void, 0, len as usize);
      STRING_SET_CHARS (string, len);
      STRING_SET_UNIBYTE (&raw mut string);
    }
  return Qnil ();
});

/* C 3384-3387: `nconc2`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn nconc2 (s1: Lisp_Object, s2: Lisp_Object) -> Lisp_Object
{
  return CALLN! (Fnconc, s1, s2);
}

DEFUN! ("nconc", Fnconc, Snconc, 0, MANY, 0,
       doc: "Concatenate any number of lists by altering them.\nOnly the last argument is not altered, and need not be a list.\nusage: (nconc &rest LISTS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 2
    {
      let a0: Lisp_Object = *args;
      let a1: Lisp_Object = *args.offset (1);
      if NILP (a0)
	{ return a1; }
      if CONSP (a0)
	{
	  let mut tail: Lisp_Object = a0;
	  let mut steps: ptrdiff_t = 0;
	  while CONSP (XCDR (tail)) && steps < (FOR_EACH_TAIL_THRESHOLD as ptrdiff_t)
	    {
	      tail = XCDR (tail);
	      steps += 1;
	    }
	  if NILP (XCDR (tail))
	    {
	      XSETCDR (tail, a1);
	      return a0;
	    }
	}
    }
  let mut val: Lisp_Object = Qnil ();

  let mut argnum: ptrdiff_t = 0;
  while argnum < nargs
    {
      let mut tem: Lisp_Object = *args.offset (argnum);
      if NILP (tem) { argnum += 1; continue; }

      if NILP (val)
	{ val = tem; }

      if argnum + 1 == nargs { break; }

      CHECK_CONS (tem);

      /* C's `Lisp_Object tail UNINIT;` is zero-initialised (PORTING.md 1.5), and its
	 `FOR_EACH_TAIL (tem) tail = tem;` sets TAIL to each successive cons — the port's
	 macro exposes the loop variable to the body, which is exactly what the body reads.  */
      let mut tail: Lisp_Object = Qnil ();
      FOR_EACH_TAIL! (tem, {
	tail = tem;
      });

      tem = *args.offset (argnum + 1);
      Fsetcdr (tail, tem);
      if NILP (tem)
	{ *args.offset (argnum + 1) = tail; }
      argnum += 1;
    }

  return val;
});


/* This is the guts of all mapping functions.
   Apply FN to each element of SEQ, one by one, storing the results
   into elements of VALS, a C vector of Lisp_Objects.  LENI is the
   length of VALS, which should also be the length of SEQ.  Return the
   number of results; although this is normally LENI, it can be less
   if SEQ is made shorter as a side effect of FN.  */

/* C 3430-3482: `mapcar1`.  */
unsafe fn mapcar1 (leni: EMACS_INT, vals: *mut Lisp_Object, r#fn: Lisp_Object,
		   seq: Lisp_Object) -> EMACS_INT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if NILP (seq)
    { return 0; }
  else if CONSP (seq)
    {
      let mut tail: Lisp_Object = seq;
      let mut i: ptrdiff_t = 0;
      while i < leni
	{
	  if ! CONSP (tail)
	    { return i; }
	  let dummy: Lisp_Object = calln! (r#fn, XCAR (tail));
	  if !vals.is_null ()
	    { *vals.offset (i) = dummy; }
	  tail = XCDR (tail);
	  i += 1;
	}
    }
  else if VECTORP (seq) || CLOSUREP (seq)
    {
      let mut i: ptrdiff_t = 0;
      while i < leni
	{
	  let dummy: Lisp_Object = calln! (r#fn, AREF (seq, i));
	  if !vals.is_null ()
	    { *vals.offset (i) = dummy; }
	  i += 1;
	}
    }
  else if STRINGP (seq)
    {
      let mut i_byte: ptrdiff_t = 0;

      let mut i: ptrdiff_t = 0;
      while i < leni
	{
	  let i_before: ptrdiff_t = i;
	  let c: c_int = fetch_string_char_advance (seq, &raw mut i, &raw mut i_byte);
	  let dummy: Lisp_Object = calln! (r#fn, make_fixnum (c as EMACS_INT));
	  if !vals.is_null ()
	    { *vals.offset (i_before) = dummy; }
	}
    }
  else if BOOL_VECTOR_P (seq)
    {
      let mut i: EMACS_INT = 0;
      while i < leni
	{
	  let dummy: Lisp_Object = calln! (r#fn, bool_vector_ref (seq, i));
	  if !vals.is_null ()
	    { *vals.offset (i) = dummy; }
	  i += 1;
	}
    }
  else
    { wrong_type_argument (Qsequencep (), seq); }

  return leni;

  }
}

DEFUN! ("mapconcat", Fmapconcat, Smapconcat, 2, 3, 0,
       doc: "Apply FUNCTION to each element of SEQUENCE, and concat the results as strings.\nIn between each pair of results, stick in SEPARATOR.  Thus, \" \" as\n  SEPARATOR results in spaces between the values returned by FUNCTION.\n\nSEQUENCE may be a list, a vector, a bool-vector, or a string.\n\nOptional argument SEPARATOR must be a string, a vector, or a list of\ncharacters; nil stands for the empty string.\n\nFUNCTION must be a function of one argument, and must return a value\n  that is a sequence of characters: either a string, or a vector or\n  list of numbers that are valid character codepoints; nil is treated\n  as an empty string.",
       (function: Lisp_Object, sequence: Lisp_Object, separator: Lisp_Object),
       {
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let leni: EMACS_INT = XFIXNAT (Flength (sequence)) as EMACS_INT;
  if CHAR_TABLE_P (sequence)
    { wrong_type_argument (Qlistp (), sequence); }
  let args_alloc: EMACS_INT = 2 * leni - 1;
  if args_alloc < 0
    { return (*empty_unibyte_string.as_mut_ptr ()); }
  let mut args: *mut Lisp_Object;
  SAFE_ALLOCA_LISP! (sa_avail, args, args_alloc);
  /* C's `goto concat` (the identity fast paths) skips the mapcar1 call; the port's
     `jumped_to_concat` flag is the same selection (PORTING.md 2.2's flag shape).  */
  let mut jumped_to_concat: bool = false;
  if BASE_EQ (function, Qidentity ())
    {
      /* Fast path when no function call is necessary.  */
      if CONSP (sequence)
	{
	  let mut src: Lisp_Object = sequence;
	  let mut dst: *mut Lisp_Object = args;
	  loop
	    {
	      *dst = XCAR (src);
	      dst = dst.offset (1);
	      src = XCDR (src);
	      if NILP (src)
		{ break; }
	    }
	  jumped_to_concat = true;
	}
      else if VECTORP (sequence)
	{
	  /* C's `memcpy (args, XVECTOR (sequence)->contents, leni * word_size)` counts bytes;
	     `copy_nonoverlapping` counts `T`s (PORTING.md P-93), so both pointers go through `u8`.  */
	  std::ptr::copy_nonoverlapping ((*XVECTOR (sequence)).contents.as_mut_ptr () as *const u8,
					 args as *mut u8,
					 (leni * std::mem::size_of::<Lisp_Object> () as EMACS_INT) as usize);
	  jumped_to_concat = true;
	}
    }
  if !jumped_to_concat
    {
      let nmapped: ptrdiff_t = mapcar1 (leni, args, function, sequence);
      eassert! (nmapped == leni);
    }

  /* concat: ;  */
  let mut nargs: ptrdiff_t = args_alloc as ptrdiff_t;
  if NILP (separator) || (STRINGP (separator) && SCHARS (separator) == 0)
    { nargs = leni as ptrdiff_t; }
  else
    {
      let mut i: ptrdiff_t = leni as ptrdiff_t - 1;
      while i > 0
	{
	  *args.offset (i + i) = *args.offset (i);
	  i -= 1;
	}

      let mut i: ptrdiff_t = 1;
      while i < nargs
	{
	  *args.offset (i) = separator;
	  i += 2;
	}
    }

  let ret: Lisp_Object = Fconcat (nargs, args);
  SAFE_FREE! (sa_count, sa_mark);
  return ret;
});

DEFUN! ("mapcar", Fmapcar, Smapcar, 2, 2, 0,
       doc: "Apply FUNCTION to each element of SEQUENCE, and make a list of the results.\nThe result is a list just as long as SEQUENCE.\nSEQUENCE may be a list, a vector, a bool-vector, or a string.",
       (function: Lisp_Object, sequence: Lisp_Object),
       {
  if NILP (sequence)
    { return Qnil (); }
  let leni: EMACS_INT = XFIXNAT (Flength (sequence)) as EMACS_INT;
  if CHAR_TABLE_P (sequence)
    { wrong_type_argument (Qlistp (), sequence); }
  if leni <= 16
    {
      let mut stack_args: [Lisp_Object; 16] = [Qnil (); 16];
      let nmapped: ptrdiff_t = mapcar1 (leni, stack_args.as_mut_ptr (), function, sequence);
      return Flist (nmapped, stack_args.as_mut_ptr ());
    }
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let mut args: *mut Lisp_Object;
  SAFE_ALLOCA_LISP! (sa_avail, args, leni);
  let nmapped: ptrdiff_t = mapcar1 (leni, args, function, sequence);
  let ret: Lisp_Object = Flist (nmapped, args);
  SAFE_FREE! (sa_count, sa_mark);
  return ret;
});

DEFUN! ("mapc", Fmapc, Smapc, 2, 2, 0,
       doc: "Apply FUNCTION to each element of SEQUENCE for side effects only.\nUnlike `mapcar', don't accumulate the results.  Return SEQUENCE.\nSEQUENCE may be a list, a vector, a bool-vector, or a string.",
       (function: Lisp_Object, sequence: Lisp_Object),
       {
  if NILP (sequence)
    { return Qnil (); }
  let leni: EMACS_INT;

  leni = XFIXNAT (Flength (sequence)) as EMACS_INT;
  if CHAR_TABLE_P (sequence)
    { wrong_type_argument (Qlistp (), sequence); }
  mapcar1 (leni, std::ptr::null_mut (), function, sequence);

  return sequence;
});

DEFUN! ("mapcan", Fmapcan, Smapcan, 2, 2, 0,
       doc: "Apply FUNCTION to each element of SEQUENCE, and concatenate\nthe results by altering them (using `nconc').\nSEQUENCE may be a list, a vector, a bool-vector, or a string.",
       (function: Lisp_Object, sequence: Lisp_Object),
       {
  if NILP (sequence)
    { return Qnil (); }
  let leni: EMACS_INT = XFIXNAT (Flength (sequence)) as EMACS_INT;
  if CHAR_TABLE_P (sequence)
    { wrong_type_argument (Qlistp (), sequence); }
  if leni <= 16
    {
      let mut stack_args: [Lisp_Object; 16] = [Qnil (); 16];
      let nmapped: ptrdiff_t = mapcar1 (leni, stack_args.as_mut_ptr (), function, sequence);
      return Fnconc (nmapped, stack_args.as_mut_ptr ());
    }
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let mut args: *mut Lisp_Object;
  SAFE_ALLOCA_LISP! (sa_avail, args, leni);
  let nmapped: ptrdiff_t = mapcar1 (leni, args, function, sequence);
  let ret: Lisp_Object = Fnconc (nmapped, args);
  SAFE_FREE! (sa_count, sa_mark);
  return ret;
});

/* This is how C code calls `yes-or-no-p' and allows the user
   to redefine it.  */

/* C 3607-3610: `do_yes_or_no_p`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn do_yes_or_no_p (prompt: Lisp_Object) -> Lisp_Object
{
  return calln! (Qyes_or_no_p (), prompt);
}

DEFUN! ("yes-or-no-p", Fyes_or_no_p, Syes_or_no_p, 1, 1, 0,
       doc: "Ask user a yes-or-no question.\nReturn t if answer is yes, and nil if the answer is no.\n\nPROMPT is the string to display to ask the question; `yes-or-no-p'\nappends `yes-or-no-prompt' (default \"(yes or no) \") to it.  If\nPROMPT is a non-empty string, and it ends with a non-space character,\na space character will be appended to it.\n\nThe user must confirm the answer with RET, and can edit it until it\nhas been confirmed.\n\nIf the `use-short-answers' variable is non-nil, instead of asking for\n\"yes\" or \"no\", this function will ask for \"y\" or \"n\" (and\nignore the value of `yes-or-no-prompt').\n\nIf dialog boxes are supported, this function will use a dialog box\nif `use-dialog-box' is non-nil and the last input event was produced\nby a mouse, or by some window-system gesture, or via a menu.",
       (prompt: Lisp_Object),
       {
  let mut prompt: Lisp_Object = prompt;
  let mut ans: Lisp_Object;
  let mut val: Lisp_Object;

  CHECK_STRING (prompt);

  if !NILP ((*last_input_event.as_mut_ptr ()))
      && (CONSP ((*last_nonmenu_event.as_mut_ptr ()))
	  || (NILP ((*last_nonmenu_event.as_mut_ptr ())) && CONSP ((*last_input_event.as_mut_ptr ())))
	  || { val = find_symbol_value (Qfrom__tty_menu_p ());
	       !NILP (val) && !BASE_EQ (val, Qunbound ()) })
      && (*use_dialog_box.as_mut_ptr ())
    {
      let pane: Lisp_Object;
      let menu: Lisp_Object;
      let obj: Lisp_Object;
      redisplay_preserve_echo_area (4);
      pane = list2 (Fcons (build_string (c"Yes".as_ptr ()), Qt ()),
		    Fcons (build_string (c"No".as_ptr ()), Qnil ()));
      menu = Fcons (prompt, pane);
      obj = Fx_popup_dialog (Qt (), menu, Qnil ());
      return obj;
    }

  if (*use_short_answers.as_mut_ptr ())
    { return calln! (Qy_or_n_p (), prompt); }

  let promptlen: ptrdiff_t = SCHARS (prompt);
  let prompt_ends_in_nonspace: bool
    = (0 < promptlen
       && !blankp (XFIXNAT (Faref (prompt, make_fixnum (promptlen - 1))) as c_int));
  AUTO_STRING! (space_string, c" ".as_ptr ());
  prompt = CALLN! (Fconcat, prompt,
		   if prompt_ends_in_nonspace { space_string } else { (*empty_unibyte_string.as_mut_ptr ()) },
		   (*Vyes_or_no_prompt.as_mut_ptr ()));

  let count: specpdl_ref = SPECPDL_INDEX ();
  specbind (Qenable_recursive_minibuffers (), Qt ());
  /* Preserve the actual command that eventually called `yes-or-no-p'
     (otherwise `repeat' will be repeating `exit-minibuffer').  */
  specbind (Qreal_this_command (), (*Vreal_this_command.as_mut_ptr ()));

  loop
    {
      ans = Fdowncase (Fread_from_minibuffer (prompt, Qnil (), Qnil (), Qnil (),
					      Qyes_or_no_p_history (), Qnil (),
					      Qnil ()));
      if SCHARS (ans) == 3 && strcmp (SSDATA (ans) as *const c_char, c"yes".as_ptr ()) == 0
	{ return unbind_to (count, Qt ()); }
      if SCHARS (ans) == 2 && strcmp (SSDATA (ans) as *const c_char, c"no".as_ptr ()) == 0
	{ return unbind_to (count, Qnil ()); }

      Fding (Qnil ());
      Fdiscard_input ();
      message1 (c"Please answer yes or no.".as_ptr ());
      Fsleep_for (make_fixnum (2), Qnil ());
    }
});

DEFUN! ("load-average", Fload_average, Sload_average, 0, 1, 0,
       doc: "Return list of 1 minute, 5 minute and 15 minute load averages.\n\nEach of the three load averages is multiplied by 100, then converted\nto integer.\n\nWhen USE-FLOATS is non-nil, floats will be used instead of integers.\nThese floats are not multiplied by 100.\n\nIf the 5-minute or 15-minute load averages are not available, return a\nshortened list, containing only those averages which are available.\n\nAn error is thrown if the load average can't be obtained.  In some\ncases making it work would require Emacs being installed setuid or\nsetgid so that it can read kernel information, and that usually isn't\nadvisable.",
       (use_floats: Lisp_Object),
       {
  let mut load_ave: [f64; 3] = [0.0; 3];
  let mut loads: c_int = getloadavg (load_ave.as_mut_ptr (), 3);
  let mut ret: Lisp_Object = Qnil ();

  if loads < 0
    { error (c"load-average not implemented for this operating system".as_ptr ()); }

  loop
    {
      let t: c_int = loads;
      loads -= 1;
      if !(t > 0)
	{ break; }
      let load: Lisp_Object = if NILP (use_floats)
	{ double_to_integer (100.0 * load_ave[loads as usize]) }
	else
	{ make_float (load_ave[loads as usize]) };
      ret = Fcons (load, ret);
    }

  return ret;
});

DEFUN! ("featurep", Ffeaturep, Sfeaturep, 1, 2, 0,
       doc: "Return t if FEATURE is present in this Emacs.\n\nUse this to conditionalize execution of lisp code based on the\npresence or absence of Emacs or environment extensions.\nUse `provide' to declare that a feature is available.  This function\nlooks at the value of the variable `features'.  The optional argument\nSUBFEATURE can be used to check a specific subfeature of FEATURE.",
       (feature: Lisp_Object, subfeature: Lisp_Object),
       {
  let mut tem: Lisp_Object;
  CHECK_SYMBOL (feature);
  tem = Fmemq (feature, (*Vfeatures.as_mut_ptr ()));
  if !NILP (tem) && !NILP (subfeature)
    { tem = Fmember (subfeature, Fget (feature, Qsubfeatures ())); }
  return if NILP (tem) { Qnil () } else { Qt () };
});

DEFUN! ("provide", Fprovide, Sprovide, 1, 2, 0,
       doc: "Announce that FEATURE is a feature of the current Emacs.\nThe optional argument SUBFEATURES should be a list of symbols listing\nparticular subfeatures supported in this version of FEATURE.",
       (feature: Lisp_Object, subfeatures: Lisp_Object),
       {
  let mut tem: Lisp_Object;
  CHECK_SYMBOL (feature);
  CHECK_LIST (subfeatures);
  if !NILP ((*Vautoload_queue.as_mut_ptr ()))
    { (*Vautoload_queue.as_mut_ptr ()) = Fcons (Fcons (make_fixnum (0), (*Vfeatures.as_mut_ptr ())),
			       (*Vautoload_queue.as_mut_ptr ())); }
  tem = Fmemq (feature, (*Vfeatures.as_mut_ptr ()));
  if NILP (tem)
    { (*Vfeatures.as_mut_ptr ()) = Fcons (feature, (*Vfeatures.as_mut_ptr ())); }
  if !NILP (subfeatures)
    { Fput (feature, Qsubfeatures (), subfeatures); }
  LOADHIST_ATTACH (Fcons (Qprovide (), feature));

  /* Run any load-hooks for this file.  */
  tem = Fassq (feature, (*Vafter_load_alist.as_mut_ptr ()));
  if CONSP (tem)
    { Fmapc (Qfuncall (), XCDR (tem)); }

  return feature;
});


/* `require' and its subroutines.  */

/* List of features currently being require'd, innermost first.  */

static require_nesting_list: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

/* C 3776-3779: `require_unwind`.  */
unsafe extern "C-unwind" fn require_unwind (old_value: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*require_nesting_list.as_mut_ptr ()) = old_value;

  }
}

DEFUN! ("require", Frequire, Srequire, 1, 3, 0,
       doc: "If FEATURE is not already loaded, load it from FILENAME.\nIf FEATURE is not a member of the list `features', then the feature was\nnot yet loaded; so load it from file FILENAME.\n\nIf FILENAME is omitted, the printname of FEATURE is used as the file\nname, and `load' is called to try to load the file by that name, after\nappending the suffix `.elc', `.el', or the system-dependent suffix for\ndynamic module files, in that order; but the function will not try to\nload the file without any suffix.  See `get-load-suffixes' for the\ncomplete list of suffixes.\n\nTo find the file, this function searches the directories in `load-path'.\n\nIf the optional third argument NOERROR is non-nil, then, if\nthe file is not found, the function returns nil instead of signaling\nan error.  Normally the return value is FEATURE.\n\nThe normal messages issued by `load' at start and end of loading\nFILENAME are suppressed.",
       (feature: Lisp_Object, filename: Lisp_Object, noerror: Lisp_Object),
       {
  let mut feature: Lisp_Object = feature;
  let mut tem: Lisp_Object;
  let mut from_file: bool = (*load_in_progress.as_mut_ptr ());

  CHECK_SYMBOL (feature);

  /* Record the presence of `require' in this file
     even if the feature specified is already loaded.
     But not more than once in any file,
     and not when we aren't loading or reading from a file.  */
  if !from_file
    {
      let mut tail: Lisp_Object = (*Vcurrent_load_list.as_mut_ptr ());
      FOR_EACH_TAIL_SAFE! (tail, {
	if NILP (XCDR (tail)) && STRINGP (XCAR (tail))
	  { from_file = true; }
      });
    }

  if from_file
    {
      tem = Fcons (Qrequire (), feature);
      if NILP (Fmember (tem, (*Vcurrent_load_list.as_mut_ptr ())))
	{ LOADHIST_ATTACH (tem); }
    }
  tem = Fmemq (feature, (*Vfeatures.as_mut_ptr ()));

  if NILP (tem)
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      let mut nesting: c_int = 0;

      /* This is to make sure that loadup.el gives a clear picture
	 of what files are preloaded and when.  */
      if will_dump_p () && !will_bootstrap_p ()
	{
	  /* Avoid landing here recursively while outputting the
	     backtrace from the error.  */
	  (*gflags.as_mut_ptr ()).will_dump = false;
	  error (c"(require %s) while preparing to dump".as_ptr (),
		 SDATA (SYMBOL_NAME (feature)));
	}

      /* A certain amount of recursive `require' is legitimate,
	 but if we require the same feature recursively 3 times,
	 signal an error.  */
      tem = (*require_nesting_list.as_mut_ptr ());
      while ! NILP (tem)
	{
	  if ! NILP (Fequal (feature, XCAR (tem)))
	    { nesting += 1; }
	  tem = XCDR (tem);
	}
      if nesting > 3
	{ error (c"Recursive `require' for feature `%s'".as_ptr (),
		 SDATA (SYMBOL_NAME (feature))); }

      /* Update the list for any nested `require's that occur.  */
      record_unwind_protect (Some (require_unwind), (*require_nesting_list.as_mut_ptr ()));
      (*require_nesting_list.as_mut_ptr ()) = Fcons (feature, (*require_nesting_list.as_mut_ptr ()));

      /* Load the file.  */
      tem = load_with_autoload_queue
	(if NILP (filename) { Fsymbol_name (feature) } else { filename },
	 noerror, Qt (), Qnil (), if NILP (filename) { Qt () } else { Qnil () });

      /* If load failed entirely, return nil.  */
      if NILP (tem)
	{ return unbind_to (count, Qnil ()); }

      tem = Fmemq (feature, (*Vfeatures.as_mut_ptr ()));
      if NILP (tem)
        {
          let tem2: *mut u8 = SDATA (SYMBOL_NAME (feature));
          let tem3: Lisp_Object = Fcar (Fcar ((*Vload_history.as_mut_ptr ())));

          if NILP (tem3)
            { error (c"Required feature `%s' was not provided".as_ptr (), tem2); }
          else
            /* Cf autoload-do-load.  */
            { error (c"Loading file %s failed to provide feature `%s'".as_ptr (),
		     SDATA (tem3), tem2); }
        }

      feature = unbind_to (count, feature);
    }

  return feature;
});


/* C 3892-3894: `#ifdef HAVE_LANGINFO_CODESET` (live) — `#include <langinfo.h>`, which this
   unit's include list names at the top.  */

DEFUN! ("locale-info", Flocale_info, Slocale_info, 1, 1, 0,
       doc: "Access locale data ITEM for the current C locale, if available.\nITEM should be one of the following:\n\n`codeset', returning the character set as a string (locale item CODESET);\n\n`days', returning a 7-element vector of day names (locale items DAY_n);\n\n`months', returning a 12-element vector of month names (locale items MON_n);\n\n`paper', returning a list of 2 integers (WIDTH HEIGHT) for the default\n  paper size, both measured in millimeters (locale items _NL_PAPER_WIDTH,\n  _NL_PAPER_HEIGHT).\n\nIf the system can't provide such information through a call to\n`nl_langinfo', or if ITEM isn't from the list above, return nil.\n\nSee also Info node `(libc)Locales'.\n\nThe data read from the system are decoded using `locale-coding-system'.  ",
       (item: Lisp_Object),
       {
  let mut str_: *mut c_char = std::ptr::null_mut ();

  /* STR is apparently unused on Android.  */
  let _ = str_;

  /* C 3923-3975: the `codeset`, `days` and `months` arms are each inside `#ifdef
     HAVE_LANGINFO_CODESET` (the frozen configuration sets it; the Android one leaves it undefined,
     and bionic has no `nl_langinfo` - status 4.727).  */

  #[cfg (not (feature = "not_have_langinfo_codeset"))]
  if EQ (item, Qcodeset ())
    {
      str_ = nl_langinfo (CODESET);
      return build_string (str_);
    }
  #[cfg (not (feature = "not_have_langinfo_codeset"))]
  if EQ (item, Qdays ())  /* E.g., for calendar-day-name-array.  */
    {
      let v: Lisp_Object = make_nil_vector (7);
      let days: [c_int; 7] = [DAY_1, DAY_2, DAY_3, DAY_4, DAY_5, DAY_6, DAY_7];
      let mut i: c_int;
      synchronize_system_time_locale ();
      i = 0;
      while i < 7
	{
	  str_ = nl_langinfo (days[i as usize]);
	  AUTO_STRING! (val, str_);
	  /* Fixme: Is this coding system necessarily right, even if
	     it is consistent with CODESET?  If not, what to do?  */
	  ASET (v, i as ptrdiff_t, code_convert_string_norecord (val, (*Vlocale_coding_system.as_mut_ptr ()),
								 false));
	  i += 1;
	}
      return v;
    }
  #[cfg (not (feature = "not_have_langinfo_codeset"))]
  if EQ (item, Qmonths ())  /* E.g., for calendar-month-name-array.  */
    {
      let v: Lisp_Object = make_nil_vector (12);
      let months: [c_int; 12] = [MON_1, MON_2, MON_3, MON_4, MON_5, MON_6, MON_7,
				 MON_8, MON_9, MON_10, MON_11, MON_12];
      synchronize_system_time_locale ();
      let mut i: c_int = 0;
      while i < 12
	{
	  str_ = nl_langinfo (months[i as usize]);
	  AUTO_STRING! (val, str_);
	  ASET (v, i as ptrdiff_t, code_convert_string_norecord (val, (*Vlocale_coding_system.as_mut_ptr ()),
								 false));
	  i += 1;
	}
      return v;
    }
  /* C 3965-3975: `# ifdef HAVE_LANGINFO__NL_PAPER_WIDTH` (`undef` on this target) — carried
     text (feature `have_langinfo_nl_paper_width`, P-51).  */
  #[cfg (feature = "have_langinfo_nl_paper_width")]
  if EQ (item, Qpaper ())
    /* We have to convert twice here: first to a correctly-sized integer,
       then to int, because that's what nl_langinfo is documented to
       return for _NO_PAPER_{WIDTH,HEIGHT}.  The cast doesn't
       suffice because it could overflow an Emacs fixnum.  This can
       happen when running under ASan, which fills allocated but
       uninitialized memory with 0xBE bytes.  */
    { return list2i (((nl_langinfo (_NL_PAPER_WIDTH) as isize) as c_int),
		     ((nl_langinfo (_NL_PAPER_HEIGHT) as isize) as c_int)); }
  return Qnil ();
});
/* C 3983: `#define MIME_LINE_LENGTH 76`.  */
pub const MIME_LINE_LENGTH: c_int = 76;

/* The two tables of characters coding the 64 values (C 3986-4008).  The C tables are
   `char const`; the port spells the same 64 bytes as byte strings.  */
static base64_value_to_char: [&[u8; 64]; 2] = [
  b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
  b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
];

static base64_char_to_value: [[i8; UCHAR_MAX as usize + 1]; 2] =
  [
   [
     0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, -1, -1, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 64,
     53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 0, 0, 0, 0, 0, 0,
     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
     16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 0, 0, 0, 0, 0,
     0, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
     42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
   ],
   [
     0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, -1, -1, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0,
     53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 0, 0, 0, 0, 0, 0,
     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
     16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 0, 0, 0, 0, 64,
     0, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
     42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
   ],
  ];

/* C 4072-4080: the three forward declarations of this file's own `static` helpers are dropped
   (PORTING.md 0.1-12); their definitions keep C's positions below.  */

DEFUN! ("base64-encode-region", Fbase64_encode_region, Sbase64_encode_region,
       2, 3, "r",
       doc: "Base64-encode the region between BEG and END.\nThe data in the region is assumed to represent bytes, not text.  If\nyou want to base64-encode text, the text has to be converted into data\nfirst by using `encode-coding-region' with the appropriate coding\nsystem first.\n\nReturn the length of the encoded data.\n\nOptional third argument NO-LINE-BREAK means do not break long lines\ninto shorter lines.  ",
       (beg: Lisp_Object, end: Lisp_Object, no_line_break: Lisp_Object),
       {
  return base64_encode_region_1 (beg, end, NILP (no_line_break), true, false);
});


DEFUN! ("base64url-encode-region", Fbase64url_encode_region, Sbase64url_encode_region,
       2, 3, "r",
       doc: "Base64url-encode the region between BEG and END.\nReturn the length of the encoded text.\nOptional second argument NO-PAD means do not add padding char =.\n\nThis produces the URL variant of base 64 encoding defined in RFC 4648.  ",
       (beg: Lisp_Object, end: Lisp_Object, no_pad: Lisp_Object),
       {
  return base64_encode_region_1 (beg, end, false, NILP(no_pad), true);
});

/* C 4112-4166: `base64_encode_region_1`.  */
unsafe fn base64_encode_region_1 (beg: Lisp_Object, end: Lisp_Object, line_break: bool,
				  pad: bool, base64url: bool) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut beg: Lisp_Object = beg;
  let mut end: Lisp_Object = end;
  let mut encoded: *mut c_char;
  let mut allength: ptrdiff_t;
  let length: ptrdiff_t;
  let mut ibeg: ptrdiff_t;
  let mut iend: ptrdiff_t;
  let mut encoded_length: ptrdiff_t;
  let mut old_pos: ptrdiff_t = PT! ();
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  validate_region (&raw mut beg, &raw mut end);

  ibeg = CHAR_TO_BYTE (XFIXNAT (beg));
  iend = CHAR_TO_BYTE (XFIXNAT (end));
  move_gap_both (XFIXNAT (beg), ibeg);

  /* We need to allocate enough room for encoding the text.
     We need 33 1/3% more space, plus a newline every 76
     characters, and then we round up. */
  length = iend - ibeg;
  allength = length + length/3 + 1;
  allength += allength / MIME_LINE_LENGTH as ptrdiff_t + 1 + 6;

  encoded = SAFE_ALLOCA! (sa_avail, allength) as *mut c_char;
  encoded_length = base64_encode_1 (BYTE_POS_ADDR (ibeg) as *const c_char,
				    encoded, length, line_break,
				    pad, base64url,
				    !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)));
  if encoded_length > allength
    { emacs_abort (); }

  if encoded_length < 0
    {
      /* The encoding wasn't possible. */
      SAFE_FREE! (sa_count, sa_mark);
      error (c"Multibyte character in data for base64 encoding".as_ptr ());
    }

  /* Now we have encoded the region, so we insert the new contents
     and delete the old.  (Insert first in order to preserve markers.)  */
  SET_PT_BOTH (XFIXNAT (beg), ibeg);
  insert (encoded, encoded_length);
  SAFE_FREE! (sa_count, sa_mark);
  del_range_byte (ibeg + encoded_length, iend + encoded_length);

  /* If point was outside of the region, restore it exactly; else just
     move to the beginning of the region.  */
  if old_pos >= XFIXNAT (end)
    { old_pos += encoded_length - (XFIXNAT (end) - XFIXNAT (beg)); }
  else if old_pos > XFIXNAT (beg)
    { old_pos = XFIXNAT (beg); }
  SET_PT (old_pos);

  /* We return the length of the encoded text. */
  return make_fixnum (encoded_length);

  }
}

DEFUN! ("base64-encode-string", Fbase64_encode_string, Sbase64_encode_string,
       1, 2, 0,
       doc: "Base64-encode STRING and return the result.\nOptional second argument NO-LINE-BREAK means do not break long lines\ninto shorter lines.  ",
       (string: Lisp_Object, no_line_break: Lisp_Object),
       {

  return base64_encode_string_1 (string, NILP (no_line_break), true, false);
});

DEFUN! ("base64url-encode-string", Fbase64url_encode_string,
       Sbase64url_encode_string, 1, 2, 0,
       doc: "Base64url-encode STRING and return the result.\nOptional second argument NO-PAD means do not add padding char =.\n\nThis produces the URL variant of base 64 encoding defined in RFC 4648.  ",
       (string: Lisp_Object, no_pad: Lisp_Object),
       {

  return base64_encode_string_1 (string, false, NILP(no_pad), true);
});

/* C 4192-4230: `base64_encode_string_1`.  */
unsafe fn base64_encode_string_1 (string: Lisp_Object, line_break: bool,
				  pad: bool, base64url: bool) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut allength: ptrdiff_t;
  let length: ptrdiff_t;
  let mut encoded_length: ptrdiff_t;
  let mut encoded: *mut c_char;
  let encoded_string: Lisp_Object;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  CHECK_STRING (string);

  /* We need to allocate enough room for encoding the text.
     We need 33 1/3% more space, plus a newline every 76
     characters, and then we round up. */
  length = SBYTES (string);
  allength = length + length/3 + 1;
  allength += allength / MIME_LINE_LENGTH as ptrdiff_t + 1 + 6;

  /* We need to allocate enough room for decoding the text. */
  encoded = SAFE_ALLOCA! (sa_avail, allength) as *mut c_char;

  encoded_length = base64_encode_1 (SSDATA (string),
				    encoded, length, line_break,
				    pad, base64url,
				    STRING_MULTIBYTE (string));
  if encoded_length > allength
    { emacs_abort (); }

  if encoded_length < 0
    {
      /* The encoding wasn't possible. */
      error (c"Multibyte character in data for base64 encoding".as_ptr ());
    }

  encoded_string = make_unibyte_string (encoded, encoded_length);
  SAFE_FREE! (sa_count, sa_mark);

  return encoded_string;

  }
}

/* C 4232-4318: `base64_encode_1`.  */
unsafe fn base64_encode_1 (from: *const c_char, to: *mut c_char, length: ptrdiff_t,
			   line_break: bool, pad: bool, base64url: bool,
			   multibyte: bool) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut counter: c_int = 0;
  let mut i: ptrdiff_t = 0;
  let mut e: *mut c_char = to;
  let mut c: c_int;
  let mut value: c_uint;
  let mut bytes: c_int = 0;
  let b64_value_to_char: *const u8 = base64_value_to_char[base64url as usize].as_ptr ();

  while i < length
    {
      if multibyte
	{
	  c = string_char_and_length ((from.offset (i)) as *mut u8, &raw mut bytes);
	  if CHAR_BYTE8_P (c)
	    { c = CHAR_TO_BYTE8 (c); }
	  else if c >= 128
	    { return -1; }
	  i += bytes as ptrdiff_t;
	}
      else
	{
	  c = *from.offset (i) as c_int;
	  i += 1;
	}

      /* Wrap line every 76 characters.  */

      if line_break
	{
	  if counter < MIME_LINE_LENGTH as c_int / 4
	    { counter += 1; }
	  else
	    {
	      *e = b'\n' as c_char;
	      e = e.offset (1);
	      counter = 1;
	    }
	}

      /* Process first byte of a triplet.  */

      *e = *b64_value_to_char.offset ((0x3f & c >> 2) as isize) as c_char;
      e = e.offset (1);
      value = ((0x03 & c) << 4) as c_uint;

      /* Process second byte of a triplet.  */

      if i == length
	{
	  *e = *b64_value_to_char.offset (value as isize) as c_char;
	  e = e.offset (1);
	  if pad
	    {
	      *e = b'=' as c_char;
	      e = e.offset (1);
	      *e = b'=' as c_char;
	      e = e.offset (1);
	    }
	  break;
	}

      if multibyte
	{
	  c = string_char_and_length ((from.offset (i)) as *mut u8, &raw mut bytes);
	  if CHAR_BYTE8_P (c)
	    { c = CHAR_TO_BYTE8 (c); }
	  else if c >= 128
	    { return -1; }
	  i += bytes as ptrdiff_t;
	}
      else
	{
	  c = *from.offset (i) as c_int;
	  i += 1;
	}

      *e = *b64_value_to_char.offset ((value | ((0x0f & c >> 4) as c_uint)) as isize) as c_char;
      e = e.offset (1);
      value = ((0x0f & c) << 2) as c_uint;

      /* Process third byte of a triplet.  */

      if i == length
	{
	  *e = *b64_value_to_char.offset (value as isize) as c_char;
	  e = e.offset (1);
	  if pad
	    {
	      *e = b'=' as c_char;
	      e = e.offset (1);
	    }
	  break;
	}

      if multibyte
	{
	  c = string_char_and_length ((from.offset (i)) as *mut u8, &raw mut bytes);
	  if CHAR_BYTE8_P (c)
	    { c = CHAR_TO_BYTE8 (c); }
	  else if c >= 128
	    { return -1; }
	  i += bytes as ptrdiff_t;
	}
      else
	{
	  c = *from.offset (i) as c_int;
	  i += 1;
	}

      *e = *b64_value_to_char.offset ((value | ((0x03 & c >> 6) as c_uint)) as isize) as c_char;
      e = e.offset (1);
      *e = *b64_value_to_char.offset ((0x3f & c) as isize) as c_char;
      e = e.offset (1);
    }

  return e.offset_from (to);

  }
}


DEFUN! ("base64-decode-region", Fbase64_decode_region, Sbase64_decode_region,
       2, 4, "r",
       doc: "Base64-decode the region between BEG and END.\nReturn the length of the decoded data.\n\nNote that after calling this function, the data in the region will\nrepresent bytes, not text.  If you want to end up with text, you have\nto call `decode-coding-region' afterwards with an appropriate coding\nsystem.\n\nIf the region can't be decoded, signal an error and don't modify the buffer.\nOptional third argument BASE64URL determines whether to use the URL variant\nof the base 64 encoding, as defined in RFC 4648.\nIf optional fourth argument IGNORE-INVALID is non-nil invalid characters\nare ignored instead of signaling an error.  ",
       (beg: Lisp_Object, end: Lisp_Object, base64url: Lisp_Object,
        ignore_invalid: Lisp_Object),
       {
  let mut beg: Lisp_Object = beg;
  let mut end: Lisp_Object = end;
  let mut ibeg: ptrdiff_t;
  let mut iend: ptrdiff_t;
  let mut length: ptrdiff_t;
  let mut allength: ptrdiff_t;
  let mut decoded: *mut c_char;
  let mut old_pos: ptrdiff_t = PT! ();
  let mut decoded_length: ptrdiff_t;
  let mut inserted_chars: ptrdiff_t = 0;
  let multibyte: bool = !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  validate_region (&raw mut beg, &raw mut end);

  ibeg = CHAR_TO_BYTE (XFIXNAT (beg));
  iend = CHAR_TO_BYTE (XFIXNAT (end));

  length = iend - ibeg;

  /* We need to allocate enough room for decoding the text.  If we are
     working on a multibyte buffer, each decoded code may occupy at
     most two bytes.  */
  allength = if multibyte { length * 2 } else { length };
  decoded = SAFE_ALLOCA! (sa_avail, allength) as *mut c_char;

  move_gap_both (XFIXNAT (beg), ibeg);
  decoded_length = base64_decode_1 (BYTE_POS_ADDR (ibeg) as *const c_char,
				    decoded, length, !NILP (base64url),
				    multibyte, !NILP (ignore_invalid),
				    &raw mut inserted_chars);
  if decoded_length > allength
    { emacs_abort (); }

  if decoded_length < 0
    {
      /* The decoding wasn't possible. */
      error (c"Invalid base64 data".as_ptr ());
    }

  /* Now we have decoded the region, so we insert the new contents
     and delete the old.  (Insert first in order to preserve markers.)  */
  TEMP_SET_PT_BOTH (XFIXNAT (beg), ibeg);
  insert_1_both (decoded, inserted_chars, decoded_length, false, true, false);
  signal_after_change (XFIXNAT (beg), 0, inserted_chars);
  SAFE_FREE! (sa_count, sa_mark);

  /* Delete the original text.  */
  del_range_both (PT! (), PT_BYTE! (), XFIXNAT (end) + inserted_chars,
		  iend + decoded_length, true);

  /* If point was outside of the region, restore it exactly; else just
     move to the beginning of the region.  */
  if old_pos >= XFIXNAT (end)
    { old_pos += inserted_chars - (XFIXNAT (end) - XFIXNAT (beg)); }
  else if old_pos > XFIXNAT (beg)
    { old_pos = XFIXNAT (beg); }
  SET_PT (if old_pos > ZV! () { ZV! () } else { old_pos });

  return make_fixnum (inserted_chars);
});

DEFUN! ("base64-decode-string", Fbase64_decode_string, Sbase64_decode_string,
       1, 3, 0,
       doc: "Base64-decode STRING and return the result as a string.\nOptional argument BASE64URL determines whether to use the URL variant of\nthe base 64 encoding, as defined in RFC 4648.\nIf optional third argument IGNORE-INVALID is non-nil invalid characters are\nignored instead of signaling an error.  ",
       (string: Lisp_Object, base64url: Lisp_Object, ignore_invalid: Lisp_Object),
       {
  let mut decoded: *mut c_char;
  let mut length: ptrdiff_t;
  let mut decoded_length: ptrdiff_t;
  let mut decoded_string: Lisp_Object = Qnil ();
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  CHECK_STRING (string);

  length = SBYTES (string);
  /* We need to allocate enough room for decoding the text. */
  decoded = SAFE_ALLOCA! (sa_avail, length) as *mut c_char;

  /* The decoded result should be unibyte. */
  let mut decoded_chars: ptrdiff_t = 0;
  decoded_length = base64_decode_1 (SSDATA (string), decoded, length,
				    !NILP (base64url), false,
				    !NILP (ignore_invalid), &raw mut decoded_chars);
  if decoded_length > length
    { emacs_abort (); }
  else if decoded_length >= 0
    { decoded_string = make_unibyte_string (decoded, decoded_length); }
  else
    { decoded_string = Qnil (); }

  SAFE_FREE! (sa_count, sa_mark);
  if !STRINGP (decoded_string)
    { error (c"Invalid base64 data".as_ptr ()); }

  return decoded_string;
});

/* Base64-decode the data at FROM of LENGTH bytes into TO.  If
   MULTIBYTE, the decoded result should be in multibyte
   form.  If IGNORE_INVALID, ignore invalid base64 characters.
   Store the number of produced characters in *NCHARS_RETURN.  */

/* C 4454-4565: `base64_decode_1`.  */
unsafe fn base64_decode_1 (from: *const c_char, to: *mut c_char, length: ptrdiff_t,
			   base64url: bool, multibyte: bool, ignore_invalid: bool,
			   nchars_return: *mut ptrdiff_t) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut f: *const c_char = from;
  let flim: *const c_char = from.offset (length);
  let mut e: *mut c_char = to;
  let mut nchars: ptrdiff_t = 0;
  let b64_char_to_value: *const i8 = base64_char_to_value[base64url as usize].as_ptr ();
  let multibyte_bit: u8 = (multibyte as u8) << 7;

  loop
    {
      let mut c: u8;
      let mut v1: c_int;

      /* Process first byte of a quadruplet. */

      loop
	{
	  if f == flim
	    {
	      *nchars_return = nchars;
	      return e.offset_from (to);
	    }
	  c = *f as u8;
	  f = f.offset (1);
	  v1 = *b64_char_to_value.offset (c as isize) as c_int;
	  if !(v1 < 0 || (v1 == 0 && ignore_invalid))
	    { break; }
	}

      if v1 == 0
	{ return -1; }
      let mut value: c_uint = ((v1 - 1) << 18) as c_uint;

      /* Process second byte of a quadruplet.  */

      loop
	{
	  if f == flim
	    { return -1; }
	  c = *f as u8;
	  f = f.offset (1);
	  v1 = *b64_char_to_value.offset (c as isize) as c_int;
	  if !(v1 < 0 || (v1 == 0 && ignore_invalid))
	    { break; }
	}

      if v1 == 0
	{ return -1; }
      value += ((v1 - 1) << 12) as c_uint;

      c = ((value >> 16) & 0xff) as u8;
      if (c & multibyte_bit) != 0
	{ e = e.offset (BYTE8_STRING (c as c_int, e as *mut u8) as isize); }
      else
	{
	  *e = c as c_char;
	  e = e.offset (1);
	}
      nchars += 1;

      /* Process third byte of a quadruplet.  */

      loop
	{
	  if f == flim
	    {
	      if !base64url && !ignore_invalid
		{ return -1; }
	      *nchars_return = nchars;
	      return e.offset_from (to);
	    }
	  c = *f as u8;
	  f = f.offset (1);
	  v1 = *b64_char_to_value.offset (c as isize) as c_int;
	  if !(v1 < 0 || (v1 == 0 && ignore_invalid))
	    { break; }
	}

      if c == b'=' as u8
	{
	  loop
	    {
	      if f == flim
		{ return -1; }
	      c = *f as u8;
	      f = f.offset (1);
	      if !(*b64_char_to_value.offset (c as isize) < 0)
		{ break; }
	    }

	  if c != b'=' as u8
	    { return -1; }
	  continue;
	}

      if v1 == 0
	{ return -1; }
      value += ((v1 - 1) << 6) as c_uint;

      c = ((value >> 8) & 0xff) as u8;
      if (c & multibyte_bit) != 0
	{ e = e.offset (BYTE8_STRING (c as c_int, e as *mut u8) as isize); }
      else
	{
	  *e = c as c_char;
	  e = e.offset (1);
	}
      nchars += 1;

      /* Process fourth byte of a quadruplet.  */

      loop
	{
	  if f == flim
	    {
	      if !base64url && !ignore_invalid
		{ return -1; }
	      *nchars_return = nchars;
	      return e.offset_from (to);
	    }
	  c = *f as u8;
	  f = f.offset (1);
	  v1 = *b64_char_to_value.offset (c as isize) as c_int;
	  if !(v1 < 0 || (v1 == 0 && ignore_invalid))
	    { break; }
	}

      if c == b'=' as u8
	{ continue; }

      if v1 == 0
	{ return -1; }
      value += (v1 - 1) as c_uint;

      c = (value & 0xff) as u8;
      if (c & multibyte_bit) != 0
	{ e = e.offset (BYTE8_STRING (c as c_int, e as *mut u8) as isize); }
      else
	{
	  *e = c as c_char;
	  e = e.offset (1);
	}
      nchars += 1;
    }

  }
}


/***********************************************************************
 *****                                                             *****
 *****			     Hash Tables                           *****
 *****                                                             *****
 ***********************************************************************/

/* Implemented by gerd@gnu.org.  This hash table implementation was
   inspired by CMUCL hash tables.  */

/* Ideas:

   1. For small tables, association lists are probably faster than
   hash tables because they have lower overhead.

   For uses of hash tables where the O(1) behavior of table
   operations is not a requirement, it might therefore be a good idea
   not to hash.  Instead, we could just do a linear search in the
   key_and_value vector of the hash table.  This could be done
   if a `:linear-search t' argument is given to make-hash-table.  */


/***********************************************************************
			       Utilities
 ***********************************************************************/

/* C 4613-4616: `CHECK_HASH_TABLE`.  */
fn CHECK_HASH_TABLE (x: Lisp_Object)
{
  CHECK_TYPE (HASH_TABLE_P (x), Qhash_table_p (), x);
}

/* C 4619-4623: `set_hash_next_slot`.  */
unsafe fn set_hash_next_slot (h: *mut Lisp_Hash_Table, idx: ptrdiff_t, val: ptrdiff_t)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! (idx >= 0 && idx < (*h).table_size as ptrdiff_t);
  *(*h).next.offset (idx) = val as hash_idx_t;

  }
}
/* C 4625-4629: `set_hash_hash_slot`.  */
unsafe fn set_hash_hash_slot (h: *mut Lisp_Hash_Table, idx: ptrdiff_t, val: hash_hash_t)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! (idx >= 0 && idx < (*h).table_size as ptrdiff_t);
  *(*h).hash.offset (idx) = val;

  }
}
/* C 4631-4635: `set_hash_index_slot`.  */
unsafe fn set_hash_index_slot (h: *mut Lisp_Hash_Table, idx: ptrdiff_t, val: ptrdiff_t)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! (idx >= 0 && idx < hash_table_index_size (h));
  *(*h).index.offset (idx) = val as hash_idx_t;

  }
}

/* If OBJ is a Lisp hash table, return a pointer to its struct
   Lisp_Hash_Table.  Otherwise, signal an error.  */

/* C 4641-4645: `check_hash_table`.  */
unsafe fn check_hash_table (obj: Lisp_Object) -> *mut Lisp_Hash_Table
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  CHECK_HASH_TABLE (obj);
  return XHASH_TABLE (obj);

  }
}


/* Value is the next integer I >= N, N >= 0 which is "almost" a prime
   number.  A number is "almost" a prime number if it is not divisible
   by any integer in the range 2 .. (NEXT_ALMOST_PRIME_LIMIT - 1).  */

/* C 4653-4660: `next_almost_prime`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn next_almost_prime (n: EMACS_INT) -> EMACS_INT
{
  let mut n: EMACS_INT = n;
  static_assert! (NEXT_ALMOST_PRIME_LIMIT == 11);
  n |= 1;
  loop
    {
      if n % 3 != 0 && n % 5 != 0 && n % 7 != 0
	{ return n; }
      n += 2;
    }
}

/* Return a Lisp vector which has the same contents as VEC but has
   at least INCR_MIN more entries, where INCR_MIN is positive.
   If NITEMS_MAX is not -1, do not grow the vector to be any larger
   than NITEMS_MAX.  New entries in the resulting vector are nil.  */

/* C 4667-4686: `larger_vector`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn larger_vector (vec: Lisp_Object, incr_min: ptrdiff_t,
					nitems_max: ptrdiff_t) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut vec: Lisp_Object = vec;
  let v: *mut Lisp_Vector;
  let mut incr: ptrdiff_t;
  let mut incr_max: ptrdiff_t;
  let old_size: ptrdiff_t;
  let mut new_size: ptrdiff_t;
  /* P-127: C's `min (PTRDIFF_MAX, SIZE_MAX)` is compared in the ternary's common type, which is
     `size_t` here, so C's value is PTRDIFF_MAX; casting SIZE_MAX down to `ptrdiff_t` first wraps
     to -1, which made this 0 and reported memory exhaustion on a legitimate vector growth.  */
  let C_language_max: ptrdiff_t = (min (PTRDIFF_MAX as usize, SIZE_MAX)
				   / std::mem::size_of::<Lisp_Object> () as usize)
    as ptrdiff_t;
  let n_max: ptrdiff_t = if 0 <= nitems_max && nitems_max < C_language_max
    { nitems_max } else { C_language_max };
  eassert! (VECTORP (vec));
  eassert! (0 < incr_min && -1 <= nitems_max);
  old_size = ASIZE (vec);
  incr_max = n_max - old_size;
  incr = max (incr_min, min (old_size >> 1, incr_max));
  if incr_max < incr
    { memory_full_up (); }
  new_size = old_size + incr;
  v = allocate_vector (new_size);
  std::ptr::copy_nonoverlapping ((*XVECTOR (vec)).contents.as_mut_ptr (),
				 (*v).contents.as_mut_ptr (), old_size as usize);
  memclear ((*v).contents.as_mut_ptr ().offset (old_size) as *mut c_void,
	    (new_size - old_size) * word_size);
  XSETVECTOR! (vec, v);
  return vec;

  }
}


/***********************************************************************
			 Low-level Functions
 ***********************************************************************/

/* Return the index of the next entry in H following the one at IDX,
   or -1 if none.  */

/* C 4698-4703: `HASH_NEXT`.  */
unsafe fn HASH_NEXT (h: *mut Lisp_Hash_Table, idx: ptrdiff_t) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! (idx >= 0 && idx < (*h).table_size as ptrdiff_t);
  return *(*h).next.offset (idx) as ptrdiff_t;

  }
}

/* Return the index of the element in hash table H that is the start
   of the collision list at index IDX, or -1 if the list is empty.  */

/* C 4708-4713: `HASH_INDEX`.  */
unsafe fn HASH_INDEX (h: *mut Lisp_Hash_Table, idx: ptrdiff_t) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! (idx >= 0 && idx < hash_table_index_size (h));
  return *(*h).index.offset (idx) as ptrdiff_t;

  }
}

/* Restore a hash table's mutability after the critical section exits.  */

/* C 4717-4721: `restore_mutability`.  */
unsafe extern "C-unwind" fn restore_mutability (ptr: *mut c_void)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let h: *mut Lisp_Hash_Table = ptr as *mut Lisp_Hash_Table;
  (*h).bits |= HASH_MUTABLE_MASK;

  }
}

/* Return the result of calling a user-defined hash or comparison
   function ARGS[0] with arguments ARGS[1] through ARGS[NARGS - 1].
   Signal an error if the function attempts to modify H, which
   otherwise might lead to undefined behavior.  */

/* C 4729-4740: `hash_table_user_defined_call`.  */
unsafe fn hash_table_user_defined_call (nargs: ptrdiff_t, args: *mut Lisp_Object,
					h: *mut Lisp_Hash_Table) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if ((*h).bits & HASH_MUTABLE_MASK) == 0
    { return Ffuncall (nargs, args); }
  let count: specpdl_ref = inhibit_garbage_collection ();
  record_unwind_protect_ptr (Some (restore_mutability), h as *mut c_void);
  (*h).bits &= !HASH_MUTABLE_MASK;
  return unbind_to (count, Ffuncall (nargs, args));

  }
}

/* Ignore H and compare KEY1 and KEY2 using 'eql'.
   Value is true if KEY1 and KEY2 are the same.  */

/* C 4744-4748: `cmpfn_eql`.  */
unsafe extern "C-unwind" fn cmpfn_eql (key1: Lisp_Object, key2: Lisp_Object,
				h: *mut Lisp_Hash_Table) -> Lisp_Object
{
  {
  let _ = h;
  return Feql (key1, key2);

  }
}

/* Ignore H and compare KEY1 and KEY2 using 'equal'.
   Value is true if KEY1 and KEY2 are the same.  */

/* C 4753-4757: `cmpfn_equal`.  */
unsafe extern "C-unwind" fn cmpfn_equal (key1: Lisp_Object, key2: Lisp_Object,
				  h: *mut Lisp_Hash_Table) -> Lisp_Object
{
  {
  let _ = h;
  return Fequal (key1, key2);

  }
}


/* Given H, compare KEY1 and KEY2 using H->user_cmp_function.
   Value is true if KEY1 and KEY2 are the same.  */

/* C 4763-4769: `cmpfn_user_defined`.  */
unsafe extern "C-unwind" fn cmpfn_user_defined (key1: Lisp_Object, key2: Lisp_Object,
					 h: *mut Lisp_Hash_Table) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut args: [Lisp_Object; 3] = [(*(*h).test).user_cmp_function, key1, key2];
  return hash_table_user_defined_call (countof! (args) as ptrdiff_t, args.as_mut_ptr (), h);

  }
}

/* C 4771-4776: `sxhash_eq`.  */
fn sxhash_eq (key: Lisp_Object) -> EMACS_INT
{
  let k: Lisp_Object = maybe_remove_pos_from_symbol (key);
  return ((XHASH (k) as EMACS_INT) ^ (XTYPE (k) as EMACS_INT));
}

/* C 4778-4781: `sxhash_eql`.  */
unsafe fn sxhash_eql (key: Lisp_Object) -> EMACS_INT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  return if FLOATP (key) || BIGNUMP (key) { sxhash (key) as EMACS_INT } else { sxhash_eq (key) };

  }
}

/* Ignore H and return a hash code for KEY which uses 'eq' to compare keys.  */

/* C 4786-4790: `hashfn_eq`.  */
unsafe extern "C-unwind" fn hashfn_eq (key: Lisp_Object, h: *mut Lisp_Hash_Table) -> hash_hash_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let _ = h;
  return reduce_emacs_uint_to_hash_hash (sxhash_eq (key) as EMACS_UINT);

  }
}

/* Ignore H and return a hash code for KEY which uses 'equal' to
   compare keys.  */
/* C 4794-4798: `hashfn_equal`.  */
unsafe extern "C-unwind" fn hashfn_equal (key: Lisp_Object, h: *mut Lisp_Hash_Table) -> hash_hash_t
{
  {
  let _ = h;
  return reduce_emacs_uint_to_hash_hash (sxhash (key) as EMACS_UINT);

  }
}

/* Ignore H and return a hash code for KEY which uses 'eql' to compare keys.  */
/* C 4801-4805: `hashfn_eql`.  */
unsafe extern "C-unwind" fn hashfn_eql (key: Lisp_Object, h: *mut Lisp_Hash_Table) -> hash_hash_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let _ = h;
  return reduce_emacs_uint_to_hash_hash (sxhash_eql (key) as EMACS_UINT);

  }
}

/* Given H, return a hash code for KEY which uses a user-defined
   function to compare keys.  */

/* C 4810-4817: `hashfn_user_defined`.  */
unsafe extern "C-unwind" fn hashfn_user_defined (key: Lisp_Object, h: *mut Lisp_Hash_Table)
  -> hash_hash_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut args: [Lisp_Object; 2] = [(*(*h).test).user_hash_function, key];
  let hash: Lisp_Object = hash_table_user_defined_call (countof! (args) as ptrdiff_t,
							args.as_mut_ptr (), h);
  return reduce_emacs_uint_to_hash_hash (if FIXNUMP (hash)
					 { XUFIXNUM (hash) as EMACS_UINT }
					 else { sxhash (hash) as EMACS_UINT });

  }
}

/* C 4819-4824: the three `struct hash_table_test` objects.  C's `.name = LISPSYM_INITIALLY
   (Qeq)` is a constant expression; the mirror's `LISPSYM_INITIALLY` is not a `const fn`
   (lisp.rs 3211), so the port spells the same value — TAG_PTR_INITIALLY (Lisp_Symbol,
   index * sizeof (struct Lisp_Symbol)), i.e. the symbol's offset plus the tag, since
   LISP_WORD_TAG is `tag << 0` (lisp.rs 485-498) — with the `iQ*` index constants from the
   generated globals.rs (the thread.rs/main_thread precedent for a C constant initializer).
   C's `cmpfn` is a function pointer; the mirror types the field `Lisp_Object` (same size),
   so each is a function-pointer-to-Lisp_Object cast and `NILP` on it is the null test.  */
#[unsafe(no_mangle)]
pub static hashtest_eq: VmGlobal<hash_table_test> = VmGlobal::<hash_table_test>::new (hash_table_test {
  hashfn: Some (hashfn_eq),
  cmpfn: std::ptr::null_mut (),
  user_hash_function: std::ptr::null_mut (),
  user_cmp_function: std::ptr::null_mut (),
  name: (((iQeq * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	  + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),
});
#[unsafe(no_mangle)]
pub static hashtest_eql: VmGlobal<hash_table_test> = VmGlobal::<hash_table_test>::new (hash_table_test {
  hashfn: Some (hashfn_eql),
  cmpfn: cmpfn_eql as unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object, *mut Lisp_Hash_Table)
    -> Lisp_Object as *const () as Lisp_Object,
  user_hash_function: std::ptr::null_mut (),
  user_cmp_function: std::ptr::null_mut (),
  name: (((iQeql * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	  + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),
});
#[unsafe(no_mangle)]
pub static hashtest_equal: VmGlobal<hash_table_test> = VmGlobal::<hash_table_test>::new (hash_table_test {
  hashfn: Some (hashfn_equal),
  cmpfn: cmpfn_equal as unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object, *mut Lisp_Hash_Table)
    -> Lisp_Object as *const () as Lisp_Object,
  user_hash_function: std::ptr::null_mut (),
  user_cmp_function: std::ptr::null_mut (),
  name: (((iQequal * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	  + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),
});

/* `<intprops.h>` has no mirror (PORTING.md P-26): C's `TYPE_MAXIMUM (t)` is the type's own
   maximum (the editfns.rs/doprnt.rs spelling).  */
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }
/* The same for `TYPE_WIDTH (t)`, intprops.h's `sizeof (t) * CHAR_BIT` (the bignum.rs row).  */
macro_rules! TYPE_WIDTH { ($t:ty) => { std::mem::size_of::<$t> () * 8 }; }

/* Allocate basically initialized hash table.  */

/* C 4829-4832: `allocate_hash_table`.  */
unsafe fn allocate_hash_table () -> *mut Lisp_Hash_Table
{
  return ALLOCATE_PLAIN_PSEUDOVECTOR! (Lisp_Hash_Table, pvec_type::PVEC_HASH_TABLE)
    as *mut Lisp_Hash_Table;
}

/* Compute the size of the index (as log2) from the table capacity.  */
/* C 4836-4847: `compute_hash_index_bits`.  */
unsafe fn compute_hash_index_bits (size: hash_idx_t) -> c_int
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* An upper bound on the size of a hash table index index.  */
  let upper_bound: hash_idx_t = min (MOST_POSITIVE_FIXNUM as isize,
				     min (TYPE_MAXIMUM! (hash_idx_t) as isize,
					  PTRDIFF_MAX / std::mem::size_of::<hash_idx_t> ()
					    as ptrdiff_t)) as hash_idx_t;
  /* Use next higher power of 2.  This works even for size=0.  */
  let mut bits: c_int = elogb (size as u64) as c_int + 1;
  if bits >= TYPE_WIDTH! (uintmax_t) as c_int
      || ((1 as uintmax_t) << bits) > upper_bound as uintmax_t
    { error (c"Hash table too large".as_ptr ()); }
  return bits;

  }
}

/* Constant hash index vector used when the table size is zero.
   This avoids allocating it from the heap.  */
static empty_hash_index_vector: [hash_idx_t; 1] = [-1];

/* Create and initialize a new hash table.

   TEST specifies the test the hash table will use to compare keys.
   It must be either one of the predefined tests `eq', `eql' or
   `equal' or a symbol denoting a user-defined test named TEST with
   test and hash functions USER_TEST and USER_HASH.

   Give the table initial capacity SIZE, 0 <= SIZE <= MOST_POSITIVE_FIXNUM.

   WEAK specifies the weakness of the table.  */

/* C 4865-4919: `make_hash_table`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn make_hash_table (test: *const hash_table_test, size: EMACS_INT,
					  weak: hash_table_weakness_t) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! (SYMBOLP ((*test).name));
  eassert! (0 <= size && size <= min (MOST_POSITIVE_FIXNUM, PTRDIFF_MAX));

  let h: *mut Lisp_Hash_Table = allocate_hash_table ();

  (*h).test = test;
  (*h).bits = ((*h).bits & !HASH_WEAKNESS_MASK) | (weak as u8 & HASH_WEAKNESS_MASK);
  (*h).count = 0;
  (*h).table_size = size as hash_idx_t;

  if size == 0
    {
      (*h).key_and_value = std::ptr::null_mut ();
      (*h).hash = std::ptr::null_mut ();
      (*h).next = std::ptr::null_mut ();
      (*h).index_bits = 0;
      (*h).index = (&raw const empty_hash_index_vector) as *mut hash_idx_t;
      (*h).next_free = -1;
    }
  else
    {
      (*h).key_and_value = hash_table_alloc_bytes ((2 * size
						    * std::mem::size_of::<Lisp_Object> ()
						    as EMACS_INT) as ptrdiff_t) as *mut Lisp_Object;
      let mut i: ptrdiff_t = 0;
      while i < 2 * size
	{
	  *(*h).key_and_value.offset (i) = HASH_UNUSED_ENTRY_KEY ();
	  i += 1;
	}

      (*h).hash = hash_table_alloc_bytes ((size * std::mem::size_of::<hash_hash_t> ()
					    as EMACS_INT) as ptrdiff_t) as *mut hash_hash_t;

      (*h).next = hash_table_alloc_bytes ((size * std::mem::size_of::<hash_idx_t> ()
					    as EMACS_INT) as ptrdiff_t) as *mut hash_idx_t;
      let mut i: ptrdiff_t = 0;
      while i < size - 1
	{
	  *(*h).next.offset (i) = (i + 1) as hash_idx_t;
	  i += 1;
	}
      *(*h).next.offset (size as isize - 1) = -1;

      let index_bits: c_int = compute_hash_index_bits (size as hash_idx_t);
      (*h).index_bits = index_bits as u8;
      let index_size: ptrdiff_t = hash_table_index_size (h);
      (*h).index = hash_table_alloc_bytes ((index_size
					    * std::mem::size_of::<hash_idx_t> ()
					    as ptrdiff_t) as ptrdiff_t) as *mut hash_idx_t;
      let mut i: ptrdiff_t = 0;
      while i < index_size
	{
	  *(*h).index.offset (i) = -1;
	  i += 1;
	}

      (*h).next_free = 0;
    }

  (*h).next_weak = std::ptr::null_mut ();
  (*h).bits |= HASH_MUTABLE_MASK;
  return make_lisp_hash_table (h);

  }
}


/* Return a copy of hash table H1.  Keys and values are not copied,
   only the table itself is.  */

/* C 4921-4950: `copy_hash_table`.  */
unsafe fn copy_hash_table (h1: *mut Lisp_Hash_Table) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let h2: *mut Lisp_Hash_Table;

  h2 = allocate_hash_table ();
  /* C's `*h2 = *h1` is the struct assignment; through raw pointers the port writes it with
     `std::ptr::read` (docs/status.md 3.51's coding_system note).  */
  *h2 = std::ptr::read (h1);
  (*h2).bits |= HASH_MUTABLE_MASK;

  if (*h1).table_size > 0
    {
      let kv_bytes: ptrdiff_t
	= 2 * (*h1).table_size as ptrdiff_t * std::mem::size_of::<Lisp_Object> () as ptrdiff_t;
      (*h2).key_and_value = hash_table_alloc_bytes (kv_bytes as ptrdiff_t) as *mut Lisp_Object;
      /* C's `memcpy (h2->key_and_value, h1->key_and_value, kv_bytes)` is a *byte* copy; Rust's
	 `copy_nonoverlapping::<T>` counts T-sized elements, so the byte count must go through
	 `*mut u8` (PORTING.md P-93: the typed form copies `sizeof *T` times the bytes).  */
      std::ptr::copy_nonoverlapping ((*h1).key_and_value as *const u8,
				     (*h2).key_and_value as *mut u8, kv_bytes as usize);

      let hash_bytes: ptrdiff_t
	= (*h1).table_size as ptrdiff_t * std::mem::size_of::<hash_hash_t> () as ptrdiff_t;
      (*h2).hash = hash_table_alloc_bytes (hash_bytes as ptrdiff_t) as *mut hash_hash_t;
      /* same: C's byte count, through `*mut u8` (P-93).  */
      std::ptr::copy_nonoverlapping ((*h1).hash as *const u8, (*h2).hash as *mut u8,
				     hash_bytes as usize);

      let next_bytes: ptrdiff_t
	= (*h1).table_size as ptrdiff_t * std::mem::size_of::<hash_idx_t> () as ptrdiff_t;
      (*h2).next = hash_table_alloc_bytes (next_bytes as ptrdiff_t) as *mut hash_idx_t;
      std::ptr::copy_nonoverlapping ((*h1).next as *const u8, (*h2).next as *mut u8,
				     next_bytes as usize);

      let index_bytes: ptrdiff_t
	= hash_table_index_size (h1) * std::mem::size_of::<hash_idx_t> () as ptrdiff_t;
      (*h2).index = hash_table_alloc_bytes (index_bytes as ptrdiff_t) as *mut hash_idx_t;
      std::ptr::copy_nonoverlapping ((*h1).index as *const u8, (*h2).index as *mut u8,
				     index_bytes as usize);
    }
  return make_lisp_hash_table (h2);

  }
}

/* Compute index into the index vector from a hash value.  */
/* C 4952-4956: `hash_index_index`.  */
#[inline(always)]
unsafe fn hash_index_index (h: *mut Lisp_Hash_Table, hash: hash_hash_t) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  return knuth_hash (hash, (*h).index_bits as c_uint);

  }
}

/* Resize hash table H if it's too full.  If H cannot be resized
   because it's already too large, throw an error.  */

/* C 4961-5030: `maybe_resize_hash_table`.  */
unsafe fn maybe_resize_hash_table (h: *mut Lisp_Hash_Table)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if (*h).next_free < 0
    {
      let old_size: ptrdiff_t = HASH_TABLE_SIZE (h) as ptrdiff_t;
      let min_size: ptrdiff_t = 6;
      let base_size: ptrdiff_t = min (max (old_size, min_size), PTRDIFF_MAX / 2);
      /* Grow aggressively at small sizes, then just double.  */
      let new_size: ptrdiff_t =
	if old_size == 0
	{ min_size }
	else
	{ if base_size <= 64 { base_size * 4 } else { base_size * 2 } };

      /* Allocate all the new vectors before updating *H, to
	 avoid problems if memory is exhausted.  */
      let next: *mut hash_idx_t = hash_table_alloc_bytes ((new_size
							   * std::mem::size_of::<hash_idx_t> ()
							   as ptrdiff_t))
	as *mut hash_idx_t;
      let mut i: ptrdiff_t = old_size;
      while i < new_size - 1
	{
	  *next.offset (i) = (i + 1) as hash_idx_t;
	  i += 1;
	}
      *next.offset (new_size - 1) = -1;

      let key_and_value: *mut Lisp_Object
	= hash_table_alloc_bytes ((2 * new_size * std::mem::size_of::<Lisp_Object> ()
				   as ptrdiff_t) as ptrdiff_t) as *mut Lisp_Object;
      if old_size != 0
	{ /* C's `memcpy (key_and_value, h->key_and_value,
			   2 * old_size * sizeof *key_and_value)` is a *byte* copy; Rust's
	     `copy_nonoverlapping::<T> (…)` counts T-sized elements, so the byte count must go
	     through `*mut u8` (status 3.618: the `Lisp_Object` form copied 8x the bytes and
	     smashed the heap at the second resize of any table).  */
	  std::ptr::copy_nonoverlapping ((*h).key_and_value as *const u8,
					 key_and_value as *mut u8,
					 (2 * old_size
					  * std::mem::size_of::<Lisp_Object> () as ptrdiff_t)
					 as usize); }
      let mut i: ptrdiff_t = 2 * old_size;
      while i < 2 * new_size
        {
	  *key_and_value.offset (i) = HASH_UNUSED_ENTRY_KEY ();
	  i += 1;
	}

      let hash: *mut hash_hash_t
	= hash_table_alloc_bytes ((new_size * std::mem::size_of::<hash_hash_t> () as ptrdiff_t)
				  as ptrdiff_t) as *mut hash_hash_t;
      if old_size != 0
	{ /* same: C's byte count, through `*mut u8`.  */
	  std::ptr::copy_nonoverlapping ((*h).hash as *const u8, hash as *mut u8,
					 (old_size
					  * std::mem::size_of::<hash_hash_t> () as ptrdiff_t)
					 as usize); }

      let old_index_size: ptrdiff_t = hash_table_index_size (h);
      let index_bits: ptrdiff_t = compute_hash_index_bits (new_size as hash_idx_t) as ptrdiff_t;
      let index_size: ptrdiff_t = (1 as ptrdiff_t) << index_bits;
      let index: *mut hash_idx_t
	= hash_table_alloc_bytes ((index_size * std::mem::size_of::<hash_idx_t> () as ptrdiff_t)
				  as ptrdiff_t) as *mut hash_idx_t;
      let mut i: ptrdiff_t = 0;
      while i < index_size
	{
	  *index.offset (i) = -1;
	  i += 1;
	}

      (*h).index_bits = index_bits as u8;
      (*h).table_size = new_size as hash_idx_t;
      (*h).next_free = old_size as hash_idx_t;

      if old_index_size > 1
	{ hash_table_free_bytes ((*h).index as *mut c_void,
				 (old_index_size * std::mem::size_of::<hash_idx_t> ()
				  as ptrdiff_t)); }
      (*h).index = index;

      hash_table_free_bytes ((*h).key_and_value as *mut c_void,
			     (2 * old_size * std::mem::size_of::<Lisp_Object> ()
			      as ptrdiff_t));
      (*h).key_and_value = key_and_value;

      hash_table_free_bytes ((*h).hash as *mut c_void,
			     (old_size * std::mem::size_of::<hash_hash_t> ()
			      as ptrdiff_t));
      (*h).hash = hash;

      hash_table_free_bytes ((*h).next as *mut c_void,
			     (old_size * std::mem::size_of::<hash_idx_t> ()
			      as ptrdiff_t));
      (*h).next = next;

      (*h).key_and_value = key_and_value;

      /* Rehash: all data occupy entries 0..old_size-1.  */
      let mut i: ptrdiff_t = 0;
      while i < old_size
	{
	  let hash_code: hash_hash_t = HASH_HASH (h, i as hash_idx_t);
	  let start_of_bucket: ptrdiff_t = hash_index_index (h, hash_code);
	  set_hash_next_slot (h, i, HASH_INDEX (h, start_of_bucket));
	  set_hash_index_slot (h, start_of_bucket, i);
	  i += 1;
	}
    }

  }
}

/* C 5032-5043: `hash_table_test_from_std`.  */
unsafe fn hash_table_test_from_std (test: hash_table_std_test_t) -> *const hash_table_test
{
  {
  match test
    {
      Test_eq => { return hashtest_eq.as_ptr (); }
      Test_eql => { return hashtest_eql.as_ptr (); }
      Test_equal => { return hashtest_equal.as_ptr (); }
      _ => { }
    }
  emacs_abort ();

  }
}

/* Rebuild a hash table from its frozen (dumped) form.  */
/* C 5045-5089: `hash_table_thaw`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn hash_table_thaw (hash_table: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let h: *mut Lisp_Hash_Table = XHASH_TABLE (hash_table);

  /* Freezing discarded most non-essential information; recompute it.
     The allocation is minimal with no room for growth.  */
  (*h).test = hash_table_test_from_std (((((*h).bits & HASH_FROZEN_TEST_MASK) >> 3)
					  as hash_table_std_test_t));
  let size: ptrdiff_t = (*h).count as ptrdiff_t;
  (*h).table_size = size as hash_idx_t;
  (*h).next_free = -1;

  if size == 0
    {
      (*h).key_and_value = std::ptr::null_mut ();
      (*h).hash = std::ptr::null_mut ();
      (*h).next = std::ptr::null_mut ();
      (*h).index_bits = 0;
      (*h).index = (&raw const empty_hash_index_vector) as *mut hash_idx_t;
    }
  else
    {
      let index_bits: ptrdiff_t = compute_hash_index_bits (size as hash_idx_t) as ptrdiff_t;
      (*h).index_bits = index_bits as u8;

      (*h).hash = hash_table_alloc_bytes ((size * std::mem::size_of::<hash_hash_t> ()
					   as ptrdiff_t) as ptrdiff_t) as *mut hash_hash_t;

      (*h).next = hash_table_alloc_bytes ((size * std::mem::size_of::<hash_idx_t> ()
					   as ptrdiff_t) as ptrdiff_t) as *mut hash_idx_t;

      let index_size: ptrdiff_t = hash_table_index_size (h);
      (*h).index = hash_table_alloc_bytes ((index_size * std::mem::size_of::<hash_idx_t> ()
					    as ptrdiff_t) as ptrdiff_t) as *mut hash_idx_t;
      let mut i: ptrdiff_t = 0;
      while i < index_size
	{
	  *(*h).index.offset (i) = -1;
	  i += 1;
	}

      /* Recompute the hash codes for each entry in the table.  */
      let mut i: ptrdiff_t = 0;
      while i < size
	{
	  let key: Lisp_Object = HASH_KEY (h, i as hash_idx_t);
	  let hash_code: hash_hash_t = hash_from_key (h, key);
	  let start_of_bucket: ptrdiff_t = hash_index_index (h, hash_code);
	  set_hash_hash_slot (h, i, hash_code);
	  set_hash_next_slot (h, i, HASH_INDEX (h, start_of_bucket));
	  set_hash_index_slot (h, start_of_bucket, i);
	  i += 1;
	}
    }

  }
}

/* Look up KEY with hash HASH in table H.
   Return entry index or -1 if none.  */
/* C 5094-5108: `hash_find_with_hash`.  */
#[inline(always)]
unsafe fn hash_find_with_hash (h: *mut Lisp_Hash_Table, key: Lisp_Object,
			       hash: hash_hash_t) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let start_of_bucket: ptrdiff_t = hash_index_index (h, hash);
  let mut i: ptrdiff_t = HASH_INDEX (h, start_of_bucket);
  let cmpfn: Lisp_Object = (*(*h).test).cmpfn;
  if ! (*symbols_with_pos_enabled.as_mut_ptr ())
     && (cmpfn.is_null ()
	 || ((*h).test == hashtest_eql.as_ptr () && ! (FLOATP (key) || BIGNUMP (key))))
    {
      let kv: *mut Lisp_Object = (*h).key_and_value;
      let next: *mut hash_idx_t = (*h).next;
      while 0 <= i
	{
	  if BASE_EQ (key, *kv.offset (2 * i))
	    { return i; }
	  i = *next.offset (i) as ptrdiff_t;
	}
      return -1;
    }
  while 0 <= i
    {
      if EQ (key, HASH_KEY (h, i as hash_idx_t))
	  || (!cmpfn.is_null ()
	      && hash == HASH_HASH (h, i as hash_idx_t)
	      && !NILP (std::mem::transmute::<Lisp_Object,
			 unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object,
					       *mut Lisp_Hash_Table) -> Lisp_Object>
			 (cmpfn) (key, HASH_KEY (h, i as hash_idx_t), h)))
	{ return i; }
      i = HASH_NEXT (h, i);
    }

  return -1;

  }
}


/* Look up KEY in hash table H.  Return its hash value in *PHASH.
   Value is the index of the entry in H matching KEY, or -1 if not found.  */
/* C 5119-5126: `hash_find_get_hash`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn hash_find_get_hash (h: *mut Lisp_Hash_Table, key: Lisp_Object,
					     phash: *mut hash_hash_t) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let hash: EMACS_UINT = hash_from_key (h, key) as EMACS_UINT;
  *phash = hash as hash_hash_t;
  return hash_find_with_hash (h, key, hash as hash_hash_t);

  }
}

/* C 5128-5133: `check_mutable_hash_table`.  */
#[inline(always)]
unsafe fn check_mutable_hash_table (obj: Lisp_Object, h: *mut Lisp_Hash_Table)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if ((*h).bits & HASH_MUTABLE_MASK) == 0
    { signal_error (c"hash table test modifies table".as_ptr (), obj); }

  }
}



/* Remove the entry matching KEY from hash table H, if there is one.  */

/* C 5168-5204: `hash_remove_from_table`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn hash_remove_from_table (h: *mut Lisp_Hash_Table, key: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let hashval: hash_hash_t = hash_from_key (h, key);
  let start_of_bucket: ptrdiff_t = hash_index_index (h, hashval);
  let mut prev: ptrdiff_t = -1;

  let mut i: ptrdiff_t = HASH_INDEX (h, start_of_bucket);
  while 0 <= i
    {
      if EQ (key, HASH_KEY (h, i as hash_idx_t))
	  || (!(*(*h).test).cmpfn.is_null ()
	      && hashval == HASH_HASH (h, i as hash_idx_t)
	      && !NILP (std::mem::transmute::<Lisp_Object,
			 unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object,
					       *mut Lisp_Hash_Table) -> Lisp_Object>
			 ((*(*h).test).cmpfn) (key, HASH_KEY (h, i as hash_idx_t), h)))
	{
	  /* Take entry out of collision chain.  */
	  if prev < 0
	    { set_hash_index_slot (h, start_of_bucket, HASH_NEXT (h, i)); }
	  else
	    { set_hash_next_slot (h, prev, HASH_NEXT (h, i)); }

	  /* Clear slots in key_and_value and add the slots to
	     the free list.  */
	  set_hash_key_slot (h, i as hash_idx_t, HASH_UNUSED_ENTRY_KEY ());
	  set_hash_value_slot (h, i as hash_idx_t, Qnil ());
	  set_hash_next_slot (h, i, (*h).next_free as ptrdiff_t);
	  (*h).next_free = i as hash_idx_t;
	  (*h).count -= 1;
	  eassert! ((*h).count >= 0);
	  break;
	}

      prev = i;
      i = HASH_NEXT (h, i);
    }

  }
}


/* Clear hash table H.  */

/* C 5208-5235: `hash_clear`.  */
unsafe fn hash_clear (h: *mut Lisp_Hash_Table)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if (*h).count > 0
    {
      let size: ptrdiff_t = HASH_TABLE_SIZE (h) as ptrdiff_t;
      let mut i: ptrdiff_t = 0;
      while i < size
	{
	  set_hash_next_slot (h, i, if i < size - 1 { i + 1 } else { -1 });
	  set_hash_key_slot (h, i as hash_idx_t, HASH_UNUSED_ENTRY_KEY ());
	  set_hash_value_slot (h, i as hash_idx_t, Qnil ());
	  i += 1;
	}

      let index_size: ptrdiff_t = hash_table_index_size (h);
      let mut i: ptrdiff_t = 0;
      while i < index_size
	{
	  *(*h).index.offset (i) = -1;
	  i += 1;
	}

      (*h).next_free = 0;
      (*h).count = 0;
    }

  }
}

/* Look up KEY in table H.  Return entry index or -1 if none.  */
#[cold]
#[inline(never)]
unsafe fn hash_find_slow (h: *mut Lisp_Hash_Table, key: Lisp_Object) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  return hash_find_with_hash (h, key, hash_from_key (h, key));

  }
}

#[cold]
#[inline(never)]
unsafe fn puthash_slow (h: *mut Lisp_Hash_Table, key: Lisp_Object, value: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let hash: hash_hash_t = hash_from_key (h, key);
  let i: ptrdiff_t = hash_find_with_hash (h, key, hash);
  if i >= 0
    { *(*h).key_and_value.offset (2 * i + 1) = value; }
  else
    { hash_put (h, key, value, hash); }

  }
}

#[inline(always)]
unsafe fn hash_find_inline (h: *mut Lisp_Hash_Table, key: Lisp_Object) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let test: *const hash_table_test = (*h).test;
  if ! (*symbols_with_pos_enabled.as_mut_ptr ())
     && ((test == hashtest_eql.as_ptr () && ! (FLOATP (key) || BIGNUMP (key)))
	 || test == hashtest_eq.as_ptr ())
    {
      let hash: hash_hash_t = reduce_emacs_uint_to_hash_hash (
	((XHASH (key) as EMACS_INT) ^ (XTYPE (key) as EMACS_INT)) as EMACS_UINT
      );
      let start_of_bucket: ptrdiff_t = hash_index_index (h, hash);
      let mut i: ptrdiff_t = *(*h).index.offset (start_of_bucket) as ptrdiff_t;
      let kv: *mut Lisp_Object = (*h).key_and_value;
      let next: *mut hash_idx_t = (*h).next;
      while 0 <= i
	{
	  if BASE_EQ (key, *kv.offset (2 * i))
	    { return i; }
	  i = *next.offset (i) as ptrdiff_t;
	}
      return -1;
    }
  return hash_find_slow (h, key);

  }
}

/* C 5111-5115: `hash_find`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn hash_find (h: *mut Lisp_Hash_Table, key: Lisp_Object) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  return hash_find_inline (h, key);

  }
}

/* Put an entry into hash table H that associates KEY with VALUE.
   HASH is a previously computed hash code of KEY.
   Value is the index of the entry in H matching KEY.  */

/* C 5139-5164: `hash_put`.  */
#[cold]
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn hash_put (h: *mut Lisp_Hash_Table, key: Lisp_Object,
				   value: Lisp_Object, hash: hash_hash_t) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! (!hash_unused_entry_key_p (key));
  /* Increment count after resizing because resizing may fail.  */
  maybe_resize_hash_table (h);
  (*h).count += 1;

  /* Store key/value in the key_and_value vector.  */
  let i: ptrdiff_t = (*h).next_free as ptrdiff_t;
  eassert! (hash_unused_entry_key_p (HASH_KEY (h, i as hash_idx_t)));
  (*h).next_free = HASH_NEXT (h, i) as hash_idx_t;
  set_hash_key_slot (h, i as hash_idx_t, key);
  set_hash_value_slot (h, i as hash_idx_t, value);

  /* Remember its hash code.  */
  set_hash_hash_slot (h, i, hash);

  /* Add new entry to its collision chain.  */
  let start_of_bucket: ptrdiff_t = hash_index_index (h, hash);
  set_hash_next_slot (h, i, HASH_INDEX (h, start_of_bucket));
  set_hash_index_slot (h, start_of_bucket, i);
  return i;

  }
}


/************************************************************************
			   Weak Hash Tables
 ************************************************************************/

/* Whether to keep an entry whose key and value are known to be retained
   if STRONG_KEY and STRONG_VALUE, respectively, are true.  */
/* C 5238-5254: `keep_entry_p`.  */
fn keep_entry_p (weakness: hash_table_weakness_t, strong_key: bool,
			strong_value: bool) -> bool
{
  {
  match weakness
    {
      Weak_None =>          { return true; }
      Weak_Key =>           { return strong_key; }
      Weak_Value =>         { return strong_value; }
      Weak_Key_Or_Value =>  { return strong_key || strong_value; }
      Weak_Key_And_Value => { return strong_key && strong_value; }
      _ => { }
    }
  emacs_abort ();

  }
}

/* Sweep weak hash table H.  REMOVE_ENTRIES_P means remove
   entries from the table that don't survive the current GC.
   !REMOVE_ENTRIES_P means mark entries that are in use.  Value is
   true if anything was marked.  */

/* C 5258-5345: `sweep_weak_table`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sweep_weak_table (h: *mut Lisp_Hash_Table,
					   remove_entries_p: bool) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let n: ptrdiff_t = hash_table_index_size (h);
  let mut marked: bool = false;

  let mut bucket: ptrdiff_t = 0;
  while bucket < n
    {
      /* Follow collision chain, removing entries that don't survive
         this garbage collection.  */
      let mut prev: ptrdiff_t = -1;
      let mut next: ptrdiff_t;
      let mut i: ptrdiff_t = HASH_INDEX (h, bucket);
      while 0 <= i
        {
	  let key_known_to_survive_p: bool = survives_gc_p (HASH_KEY (h, i as hash_idx_t));
	  let value_known_to_survive_p: bool = survives_gc_p (HASH_VALUE (h, i as hash_idx_t));
	  let remove_p: bool = !keep_entry_p (((*h).bits & HASH_WEAKNESS_MASK)
					      as hash_table_weakness_t,
					      key_known_to_survive_p,
					      value_known_to_survive_p);

	  next = HASH_NEXT (h, i);

	  if remove_entries_p
	    {
              eassert! (!remove_p
                        == (key_known_to_survive_p && value_known_to_survive_p));
	      if remove_p
		{
		  /* Take out of collision chain.  */
		  if prev < 0
		    { set_hash_index_slot (h, bucket, next); }
		  else
		    { set_hash_next_slot (h, prev, next); }

		  /* Add to free list.  */
		  set_hash_next_slot (h, i, (*h).next_free as ptrdiff_t);
		  (*h).next_free = i as hash_idx_t;

		  /* Clear key and value.  */
		  set_hash_key_slot (h, i as hash_idx_t, HASH_UNUSED_ENTRY_KEY ());
		  set_hash_value_slot (h, i as hash_idx_t, Qnil ());

                  eassert! ((*h).count != 0);
                  (*h).count -= 1;
                }
	      else
		{
		  prev = i;
		}
	    }
	  else
	    {
	      if !remove_p
		{
		  /* Make sure key and value survive.  */
		  if !key_known_to_survive_p
		    {
		      mark_object (HASH_KEY (h, i as hash_idx_t));
                      marked = true;
		    }

		  if !value_known_to_survive_p
		    {
		      mark_object (HASH_VALUE (h, i as hash_idx_t));
                      marked = true;
		    }
		}
	    }
	  i = next;
	}
      bucket += 1;
    }

  return marked;

  }
}


/***********************************************************************
			Hash Code Computation
 ***********************************************************************/

/* Maximum depth up to which to dive into Lisp structures.  */

pub const SXHASH_MAX_DEPTH: c_int = 3;

/* Maximum length up to which to take list and vector elements into
   account.  */

pub const SXHASH_MAX_LEN: c_int = 7;

/* Return a hash for string PTR which has length LEN.  The hash value
   can be any EMACS_UINT value.  */

/* C 5349-5406: `hash_char_array`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn hash_char_array (ptr: *const c_char, len: ptrdiff_t) -> EMACS_UINT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut p: *const c_char = ptr;
  let end: *const c_char = ptr.offset (len);
  let mut hash: EMACS_UINT = len as EMACS_UINT;
  /* At most 8 steps.  We could reuse SXHASH_MAX_LEN, of course,
   * but dividing by 8 is cheaper.  */
  let step: ptrdiff_t = max (std::mem::size_of_val (&hash) as ptrdiff_t,
			     (end.offset_from (p)) >> 3);

  if (p.offset (std::mem::size_of_val (&hash) as isize)) <= end
    {
      loop
	{
	  let mut c: EMACS_UINT = 0;
	  /* We presume that the compiler will replace this `memcpy` with
	     a single load/move instruction when applicable.  */
	  std::ptr::copy_nonoverlapping (p as *const c_void, &raw mut c as *mut c_void,
					 std::mem::size_of_val (&hash));
	  p = p.offset (step);
	  hash = sxhash_combine (hash, c);
	  if !((p.offset (std::mem::size_of_val (&hash) as isize)) <= end)
	    { break; }
	}
      /* Hash the last word's worth of bytes in the string, because that is
         is often the part where strings differ.  This may cause some
         bytes to be hashed twice but we assume that's not a big problem.  */
      let mut c: EMACS_UINT = 0;
      std::ptr::copy_nonoverlapping (end.offset (-(std::mem::size_of_val (&c) as isize)) as *const c_void,
				     &raw mut c as *mut c_void,
				     std::mem::size_of_val (&c));
      hash = sxhash_combine (hash, c);
    }
  else
    {
      /* String is shorter than an EMACS_UINT.  Use smaller loads.  */
      eassume! (p <= end && end.offset_from (p) < std::mem::size_of::<EMACS_UINT> () as isize);
      let mut tail: EMACS_UINT = 0;
      static_assert! (std::mem::size_of::<EMACS_UINT> () <= 8);
      /* C 5383: `#if EMACS_INT_MAX > INT32_MAX` (live; no `#else`).  */
      if end.offset_from (p) >= 4
	{
	  let mut c: u32 = 0;
	  std::ptr::copy_nonoverlapping (p as *const c_void, &raw mut c as *mut c_void,
					 std::mem::size_of_val (&c));
	  tail = (tail << (8 * std::mem::size_of_val (&c))) + c as EMACS_UINT;
	  p = p.offset (std::mem::size_of_val (&c) as isize);
	}
      if end.offset_from (p) >= 2
	{
	  let mut c: u16 = 0;
	  std::ptr::copy_nonoverlapping (p as *const c_void, &raw mut c as *mut c_void,
					 std::mem::size_of_val (&c));
	  tail = (tail << (8 * std::mem::size_of_val (&c))) + c as EMACS_UINT;
	  p = p.offset (std::mem::size_of_val (&c) as isize);
	}
      if p < end
	{ tail = (tail << 8) + (*p as u8) as EMACS_UINT; }
      hash = sxhash_combine (hash, tail);
    }

  return hash;

  }
}

/* Return a hash for the floating point value VAL.  */

/* C 5410-5417: `sxhash_float`.  */
unsafe fn sxhash_float (val: f64) -> EMACS_UINT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut hash: EMACS_UINT = 0;
  let u: double_and_words = double_and_words { val: val };
  let mut i: c_int = 0;
  while i < WORDS_PER_DOUBLE as c_int
    {
      hash = sxhash_combine (hash, u.word[i as usize]);
      i += 1;
    }
  return hash;

  }
}

/* Return a hash for list LIST.  DEPTH is the current depth in the
   list.  We don't recurse deeper than SXHASH_MAX_DEPTH in it.  */

/* C 5423-5441: `sxhash_list`.  */
unsafe fn sxhash_list (list: Lisp_Object, depth: c_int) -> EMACS_UINT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut hash: EMACS_UINT = 0;
  let mut i: c_int;

  if depth < SXHASH_MAX_DEPTH
    {
      i = 0;
      let mut list: Lisp_Object = list;
      while CONSP (list) && i < SXHASH_MAX_LEN
	{
	  let hash2: EMACS_UINT = sxhash_obj (XCAR (list), depth + 1);
	  hash = sxhash_combine (hash, hash2);
	  list = XCDR (list);
	  i += 1;
	}
    }

  if !NILP (list)
    {
      let hash2: EMACS_UINT = sxhash_obj (list, depth + 1);
      hash = sxhash_combine (hash, hash2);
    }

  return hash;

  }
}


/* Return a hash for (pseudo)vector VECTOR.  DEPTH is the current depth in
   the Lisp structure.  */

/* C 5447-5463: `sxhash_vector`.  */
unsafe fn sxhash_vector (vec: Lisp_Object, depth: c_int) -> EMACS_UINT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut hash: EMACS_UINT = ASIZE (vec) as EMACS_UINT;
  let mut i: c_int;
  let n: c_int;

  n = min (SXHASH_MAX_LEN as isize, (if (hash & PSEUDOVECTOR_FLAG as EMACS_UINT) != 0
				     { PVSIZE (vec) as EMACS_UINT } else { hash }) as isize) as c_int;
  i = 0;
  while i < n
    {
      let hash2: EMACS_UINT = sxhash_obj (AREF (vec, i as ptrdiff_t), depth + 1);
      hash = sxhash_combine (hash, hash2);
      i += 1;
    }

  return hash;

  }
}

/* Return a hash for bool-vector VECTOR.  */

/* C 5469-5479: `sxhash_bool_vector`.  */
unsafe fn sxhash_bool_vector (vec: Lisp_Object) -> EMACS_UINT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let size: EMACS_INT = bool_vector_size (vec);
  let mut hash: EMACS_UINT = size as EMACS_UINT;
  let mut i: c_int;
  let n: c_int;

  n = min (SXHASH_MAX_LEN as isize, bool_vector_words (size) as isize) as c_int;
  i = 0;
  while i < n
    {
      hash = sxhash_combine (hash, *bool_vector_data (vec).offset (i as isize)
			     as EMACS_UINT);
      i += 1;
    }

  return hash;

  }
}

/* Return a hash for a bignum.  */

/* C 5485-5496: `sxhash_bignum`.  */
unsafe fn sxhash_bignum (bignum: Lisp_Object) -> EMACS_UINT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let n: *const __mpz_struct = xbignum_val (bignum);
  let nlimbs: ptrdiff_t = mpz_size (n) as ptrdiff_t;
  let mut hash: EMACS_UINT = if mpz_sgn (n) < 0 { 1 } else { 0 };

  let mut i: ptrdiff_t = 0;
  while i < nlimbs
    {
      hash = sxhash_combine (hash, mpz_getlimbn (n, i as mp_size_t) as EMACS_UINT);
      i += 1;
    }

  return hash;

  }
}

/* C 5498-5501: `sxhash`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn sxhash (obj: Lisp_Object) -> EMACS_UINT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  return sxhash_obj (obj, 0);

  }
}

/* Return a hash code for OBJ.  DEPTH is the current depth in the Lisp
   structure.  */

/* C 5507-5580: `sxhash_obj`.  */
unsafe fn sxhash_obj (obj: Lisp_Object, depth: c_int) -> EMACS_UINT
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut obj: Lisp_Object = obj;
  if depth > SXHASH_MAX_DEPTH
    { return 0; }

  match XTYPE (obj)
    {
    Lisp_Type::Lisp_Int0 | Lisp_Type::Lisp_Int1 =>
      { return XUFIXNUM (obj) as EMACS_UINT; }

    Lisp_Type::Lisp_Symbol =>
      { return XHASH (obj) as EMACS_UINT; }

    Lisp_Type::Lisp_String =>
      { return hash_char_array (SSDATA (obj) as *const c_char, SBYTES (obj)); }

    Lisp_Type::Lisp_Vectorlike =>
      {
	let pvec_type: pvec_type = PSEUDOVECTOR_TYPE (XVECTOR (obj));
	if ! ((pvec_type::PVEC_NORMAL_VECTOR as c_int) < (pvec_type as c_int)
	      && (pvec_type as c_int) < (pvec_type::PVEC_CLOSURE as c_int))
	  {
	    /* According to the CL HyperSpec, two arrays are equal only if
	       they are 'eq', except for strings and bit-vectors.  In
	       Emacs, this works differently.  We have to compare element
	       by element.  Same for pseudovectors that internal_equal
	       examines the Lisp contents of.  */
	    return if SUB_CHAR_TABLE_P (obj)
	            /* 'sxhash_vector' can't be applies to a sub-char-table and
	              it's probably not worth looking into them anyway!  */
	            { 42 }
	            else
	            { sxhash_vector (obj, depth) };
	  }
	/* FIXME: Use `switch`.  */
	else if pvec_type == pvec_type::PVEC_BIGNUM
	  { return sxhash_bignum (obj); }
	else if pvec_type == pvec_type::PVEC_MARKER
	  {
	    let bytepos: ptrdiff_t
	      = if !(*XMARKER (obj)).buffer.is_null () { (*XMARKER (obj)).bytepos } else { 0 };
	    let hash: EMACS_UINT
	      = sxhash_combine ((*XMARKER (obj)).buffer as intptr_t as EMACS_UINT, bytepos as EMACS_UINT);
	    return hash;
	  }
	else if pvec_type == pvec_type::PVEC_BOOL_VECTOR
	  { return sxhash_bool_vector (obj); }
	else if pvec_type == pvec_type::PVEC_OVERLAY
	  {
	    let mut hash: EMACS_UINT = OVERLAY_START (obj) as EMACS_UINT;
	    hash = sxhash_combine (hash, OVERLAY_END (obj) as EMACS_UINT);
	    hash = sxhash_combine (hash, sxhash_obj ((*XOVERLAY (obj)).plist, depth));
	    return hash;
	  }
	else
	  {
	    if (*symbols_with_pos_enabled.as_mut_ptr ()) && pvec_type == pvec_type::PVEC_SYMBOL_WITH_POS
	      { obj = XSYMBOL_WITH_POS_SYM (obj); }

	    /* Others are 'equal' if they are 'eq', so take their
	       address as hash.  */
	    return XHASH (obj) as EMACS_UINT;
	  }
      }

    Lisp_Type::Lisp_Cons =>
      { return sxhash_list (obj, depth); }

    Lisp_Type::Lisp_Float =>
      { return sxhash_float (XFLOAT_DATA (obj)); }

    _ =>
      { emacs_abort (); }
    }

  }
}

/* C 5583-5591: `hash_interval`.  */
unsafe extern "C-unwind" fn hash_interval (interval: INTERVAL, arg: *mut c_void)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let interval: *mut interval = interval as *mut interval;   /* P-72 rule 4 */
  let phash: *mut EMACS_UINT = arg as *mut EMACS_UINT;
  let mut hash: EMACS_UINT = *phash;
  hash = sxhash_combine (hash, (*interval).position as EMACS_UINT);
  hash = sxhash_combine (hash, LENGTH! (interval) as EMACS_UINT);
  hash = sxhash_combine (hash, sxhash_obj ((*interval).plist, 0));
  *phash = hash;

  }
}

/* C 5594-5604: `collect_interval`.  */
unsafe extern "C-unwind" fn collect_interval (interval: INTERVAL, arg: *mut c_void)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let interval: *mut interval = interval as *mut interval;   /* P-72 rule 4 */
  let collector: *mut Lisp_Object = arg as *mut Lisp_Object;
  *collector = Fcons (list3 (make_fixnum ((*interval).position),
			     make_fixnum ((*interval).position
					  + LENGTH! (interval)),
			     (*interval).plist),
		      *collector);

  }
}


/***********************************************************************
			    Lisp Interface
 ***********************************************************************/

/* Reduce the hash value X to a Lisp fixnum.  */
/* C 5612-5615: `reduce_emacs_uint_to_fixnum`.  */
fn reduce_emacs_uint_to_fixnum (x: EMACS_UINT) -> Lisp_Object
{
  return make_ufixnum (SXHASH_REDUCE (x as EMACS_INT));
}

DEFUN! ("sxhash-eq", Fsxhash_eq, Ssxhash_eq, 1, 1, 0,
       doc: "Return an integer hash code for OBJ suitable for `eq'.\nIf (eq A B), then (= (sxhash-eq A) (sxhash-eq B)).\n\nHash codes are not guaranteed to be preserved across Emacs sessions.  ",
       (obj: Lisp_Object),
       {
  return reduce_emacs_uint_to_fixnum (sxhash_eq (obj) as EMACS_UINT);
});

DEFUN! ("sxhash-eql", Fsxhash_eql, Ssxhash_eql, 1, 1, 0,
       doc: "Return an integer hash code for OBJ suitable for `eql'.\nIf (eql A B), then (= (sxhash-eql A) (sxhash-eql B)), but the opposite\nisn't necessarily true.\n\nHash codes are not guaranteed to be preserved across Emacs sessions.  ",
       (obj: Lisp_Object),
       {
  return reduce_emacs_uint_to_fixnum (sxhash_eql (obj) as EMACS_UINT);
});

DEFUN! ("sxhash-equal", Fsxhash_equal, Ssxhash_equal, 1, 1, 0,
       doc: "Return an integer hash code for OBJ suitable for `equal'.\nIf (equal A B), then (= (sxhash-equal A) (sxhash-equal B)), but the\nopposite isn't necessarily true.\n\nHash codes are not guaranteed to be preserved across Emacs sessions.  ",
       (obj: Lisp_Object),
       {
  return reduce_emacs_uint_to_fixnum (sxhash (obj));
});

DEFUN! ("sxhash-equal-including-properties", Fsxhash_equal_including_properties,
       Ssxhash_equal_including_properties, 1, 1, 0,
       doc: "Return an integer hash code for OBJ suitable for\n`equal-including-properties'.\nIf (sxhash-equal-including-properties A B), then\n(= (sxhash-equal-including-properties A) (sxhash-equal-including-properties B)).\n\nHash codes are not guaranteed to be preserved across Emacs sessions.  ",
       (obj: Lisp_Object),
       {
  let mut hash: EMACS_UINT = sxhash (obj);
  if STRINGP (obj)
    { traverse_intervals (string_intervals (obj), 0, Some (hash_interval),
			  &raw mut hash as *mut c_void); }
  return reduce_emacs_uint_to_fixnum (hash);
});


/* This is a cache of hash_table_test structures so that they can be
   shared between hash tables using the same test.
   FIXME: This way of storing and looking up hash_table_test structs
   isn't wonderful.  Find a better solution.  */
#[repr(C)]
pub struct hash_table_user_test
{
  pub test: hash_table_test,
  pub next: *mut hash_table_user_test,
}

static hash_table_user_tests: VmGlobal<*mut hash_table_user_test> = VmGlobal::<*mut hash_table_user_test>::new (std::ptr::null_mut ());

/* C 5679-5691: `mark_fns`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mark_fns ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut ut: *mut hash_table_user_test = (*hash_table_user_tests.as_mut_ptr ());
  while !ut.is_null ()
    {
      mark_object ((*ut).test.name);
      mark_object ((*ut).test.user_cmp_function);
      mark_object ((*ut).test.user_hash_function);
      ut = (*ut).next;
    }

  }
}

/* Find the hash_table_test object corresponding to the (bare) symbol TEST,
   creating one if none existed.  */
/* C 5693-5723: `get_hash_table_user_test`.  */
unsafe fn get_hash_table_user_test (test: Lisp_Object) -> *mut hash_table_test
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let prop: Lisp_Object = Fget (test, Qhash_table_test ());
  if !CONSP (prop) || !CONSP (XCDR (prop))
    { signal_error (c"Invalid hash table test".as_ptr (), test); }

  let equal_fn: Lisp_Object = XCAR (prop);
  let hash_fn: Lisp_Object = XCAR (XCDR (prop));
  let mut ut: *mut hash_table_user_test = (*hash_table_user_tests.as_mut_ptr ());
  while !ut.is_null ()
    && !(BASE_EQ (test, (*ut).test.name)
	 && EQ (equal_fn, (*ut).test.user_cmp_function)
	 && EQ (hash_fn, (*ut).test.user_hash_function))
    { ut = (*ut).next; }
  if ut.is_null ()
    {
      ut = xmalloc (std::mem::size_of::<hash_table_user_test> ()) as *mut hash_table_user_test;
      (*ut).test.name = test;
      (*ut).test.user_cmp_function = equal_fn;
      (*ut).test.user_hash_function = hash_fn;
      (*ut).test.hashfn = Some (hashfn_user_defined);
      (*ut).test.cmpfn = cmpfn_user_defined as unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object,
								     *mut Lisp_Hash_Table)
						     -> Lisp_Object as *const () as Lisp_Object;
      (*ut).next = (*hash_table_user_tests.as_mut_ptr ());
      (*hash_table_user_tests.as_mut_ptr ()) = ut;
    }
  return &raw mut (*ut).test;

  }
}

DEFUN! ("make-hash-table", Fmake_hash_table, Smake_hash_table, 0, MANY, 0,
       doc: "Create and return a new hash table.\n\nArguments are specified as keyword/argument pairs.  The following\narguments are defined:\n\n:test TEST -- TEST must be a symbol that specifies how to compare\nkeys.  Default is `eql'.  Predefined are the tests `eq', `eql', and\n`equal'.  User-supplied test and hash functions can be specified via\n`define-hash-table-test'.\n\n:size SIZE -- A hint as to how many elements will be put in the table.\nThe table will always grow as needed; this argument may help performance\nslightly if the size is known in advance but is never required.\n\n:weakness WEAK -- WEAK must be one of nil, t, `key', `value',\n`key-or-value', or `key-and-value'.  If WEAK is not nil, the table\nreturned is a weak table.  Key/value pairs are removed from a weak\nhash table when there are no non-weak references pointing to their\nkey, value, one of key or value, or both key and value, depending on\nWEAK.  WEAK t is equivalent to `key-and-value'.  Default value of WEAK\nis nil.\n\nThe keywords arguments :rehash-threshold, :rehash-size, and :purecopy\nare obsolete and ignored.\n\nusage: (make-hash-table &rest KEYWORD-ARGS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut nargs: isize = nargs;
  let mut test_arg: Lisp_Object = Qnil ();
  let mut weakness_arg: Lisp_Object = Qnil ();
  let mut size_arg: Lisp_Object = Qnil ();

  if (nargs & 1) != 0
    { error (c"Odd number of arguments".as_ptr ()); }
  while nargs >= 2
    {
      nargs -= 1;
      let arg: Lisp_Object = maybe_remove_pos_from_symbol (*args.offset (nargs));
      nargs -= 1;
      let kw: Lisp_Object = maybe_remove_pos_from_symbol (*args.offset (nargs));
      if BASE_EQ (kw, QCtest ())
	{ test_arg = arg; }
      else if BASE_EQ (kw, QCweakness ())
	{ weakness_arg = arg; }
      else if BASE_EQ (kw, QCsize ())
	{ size_arg = arg; }
      else if BASE_EQ (kw, QCrehash_threshold ()) || BASE_EQ (kw, QCrehash_size ())
	       || BASE_EQ (kw, QCpurecopy ())
	{ }  /* ignore obsolete keyword arguments */
      else
	{ signal_error (c"Invalid keyword argument".as_ptr (), kw); }
    }

  let test: *const hash_table_test;
  if NILP (test_arg) || BASE_EQ (test_arg, Qeql ())
    { test = hashtest_eql.as_ptr (); }
  else if BASE_EQ (test_arg, Qeq ())
    { test = hashtest_eq.as_ptr (); }
  else if BASE_EQ (test_arg, Qequal ())
    { test = hashtest_equal.as_ptr (); }
  else
    { test = get_hash_table_user_test (test_arg); }

  let size: EMACS_INT;
  if NILP (size_arg)
    { size = DEFAULT_HASH_SIZE as EMACS_INT; }
  else if FIXNATP (size_arg)
    { size = XFIXNAT (size_arg); }
  else
    { signal_error (c"Invalid hash table size".as_ptr (), size_arg); }

  let weak: hash_table_weakness_t;
  if NILP (weakness_arg)
    { weak = Weak_None; }
  else if BASE_EQ (weakness_arg, Qkey ())
    { weak = Weak_Key; }
  else if BASE_EQ (weakness_arg, Qvalue ())
    { weak = Weak_Value; }
  else if BASE_EQ (weakness_arg, Qkey_or_value ())
    { weak = Weak_Key_Or_Value; }
  else if BASE_EQ (weakness_arg, Qt ()) || BASE_EQ (weakness_arg, Qkey_and_value ())
    { weak = Weak_Key_And_Value; }
  else
    { signal_error (c"Invalid hash table weakness".as_ptr (), weakness_arg); }

  return make_hash_table (test, size, weak);
});


DEFUN! ("copy-hash-table", Fcopy_hash_table, Scopy_hash_table, 1, 1, 0,
       doc: "Return a copy of hash table TABLE.  ",
       (table: Lisp_Object),
       {
  return copy_hash_table (check_hash_table (table));
});


DEFUN! ("hash-table-count", Fhash_table_count, Shash_table_count, 1, 1, 0,
       doc: "Return the number of elements in TABLE.  ",
       (table: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (table);
  return make_fixnum ((*h).count as EMACS_INT);
});


DEFUN! ("hash-table-rehash-size", Fhash_table_rehash_size,
       Shash_table_rehash_size, 1, 1, 0,
       doc: "Return the rehash size of TABLE.\nThis function is for compatibility only; it returns a nominal value\nwithout current significance.  ",
       (table: Lisp_Object),
       {
  CHECK_HASH_TABLE (table);
  return make_float (1.5);  /* The old default rehash-size value.  */
});


DEFUN! ("hash-table-rehash-threshold", Fhash_table_rehash_threshold,
       Shash_table_rehash_threshold, 1, 1, 0,
       doc: "Return the rehash threshold of TABLE.\nThis function is for compatibility only; it returns a nominal value\nwithout current significance.  ",
       (table: Lisp_Object),
       {
  CHECK_HASH_TABLE (table);
  return make_float (0.8125);  /* The old default rehash-threshold value.  */
});


DEFUN! ("hash-table-size", Fhash_table_size, Shash_table_size, 1, 1, 0,
       doc: "Return the current allocation size of TABLE.\n\nThis is probably not the function that you are looking for.  To get the\nnumber of entries in a table, use `hash-table-count' instead.\n\nThe returned value is the number of entries that TABLE can currently\nhold without growing, but since hash tables grow automatically, this\nnumber is rarely of interest.  ",
       (table: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (table);
  return make_fixnum (HASH_TABLE_SIZE (h) as EMACS_INT);
});


DEFUN! ("hash-table-test", Fhash_table_test, Shash_table_test, 1, 1, 0,
       doc: "Return the test TABLE uses.  ",
       (table: Lisp_Object),
       {
  return (*(*check_hash_table (table)).test).name;
});

/* C 5873-5884: `hash_table_weakness_symbol`.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn hash_table_weakness_symbol (weak: hash_table_weakness_t) -> Lisp_Object
{
  {
  match weak
    {
      Weak_None =>          { return Qnil (); }
      Weak_Key =>           { return Qkey (); }
      Weak_Value =>         { return Qvalue (); }
      Weak_Key_And_Value => { return Qkey_and_value (); }
      Weak_Key_Or_Value =>  { return Qkey_or_value (); }
      _ => { }
    }
  emacs_abort ();

  }
}

DEFUN! ("hash-table-weakness", Fhash_table_weakness, Shash_table_weakness,
       1, 1, 0,
       doc: "Return the weakness of TABLE.  ",
       (table: Lisp_Object),
       {
  return hash_table_weakness_symbol (((*check_hash_table (table)).bits & HASH_WEAKNESS_MASK)
				     as hash_table_weakness_t);
});


DEFUN! ("hash-table-p", Fhash_table_p, Shash_table_p, 1, 1, 0,
       doc: "Return t if OBJ is a Lisp hash table object.  ",
       (obj: Lisp_Object),
       {
  return if HASH_TABLE_P (obj) { Qt () } else { Qnil () };
});


DEFUN! ("clrhash", Fclrhash, Sclrhash, 1, 1, 0,
       doc: "Clear hash table TABLE and return it.  ",
       (table: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (table);
  check_mutable_hash_table (table, h);
  hash_clear (h);
  /* Be compatible with XEmacs.  */
  return table;
});


DEFUN! ("gethash", Fgethash, Sgethash, 2, 3, 0,
       doc: "Look up KEY in TABLE and return its associated value.\nIf KEY is not found in table, return DEFAULT, or nil if DEFAULT is not\nprovided.\n\nusage: (gethash KEY TABLE &optional DEFAULT)  ",
       (key: Lisp_Object, table: Lisp_Object, dflt: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (table);
  let i: ptrdiff_t = hash_find_inline (h, key);
  return if i >= 0 { *(*h).key_and_value.offset (2 * i + 1) } else { dflt };
});


DEFUN! ("puthash", Fputhash, Sputhash, 3, 3, 0,
       doc: "Associate KEY with VALUE in hash table TABLE.\nIf KEY is already present in table, replace its current value with\nVALUE.  In any case, return VALUE.  ",
       (key: Lisp_Object, value: Lisp_Object, table: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (table);
  check_mutable_hash_table (table, h);

  let test: *const hash_table_test = (*h).test;
  if ! (*symbols_with_pos_enabled.as_mut_ptr ())
    && ((test == hashtest_eql.as_ptr () && ! (FLOATP (key) || BIGNUMP (key)))
	|| test == hashtest_eq.as_ptr ())
    {
      let hash: hash_hash_t = reduce_emacs_uint_to_hash_hash (
	((XHASH (key) as EMACS_INT) ^ (XTYPE (key) as EMACS_INT)) as EMACS_UINT
      );
      let start_of_bucket: ptrdiff_t = hash_index_index (h, hash);
      let mut i: ptrdiff_t = *(*h).index.offset (start_of_bucket) as ptrdiff_t;
      let kv: *mut Lisp_Object = (*h).key_and_value;
      let next: *mut hash_idx_t = (*h).next;
      while 0 <= i
	{
	  if BASE_EQ (key, *kv.offset (2 * i))
	    {
	      *kv.offset (2 * i + 1) = value;
	      return value;
	    }
	  i = *next.offset (i) as ptrdiff_t;
	}
      let free_idx: ptrdiff_t = (*h).next_free as ptrdiff_t;
      if free_idx >= 0
	{
	  (*h).count += 1;
	  (*h).next_free = *next.offset (free_idx);
	  *kv.offset (2 * free_idx) = key;
	  *kv.offset (2 * free_idx + 1) = value;
	  *(*h).hash.offset (free_idx) = hash;
	  let head: hash_idx_t = *(*h).index.offset (start_of_bucket);
	  *next.offset (free_idx) = head;
	  *(*h).index.offset (start_of_bucket) = free_idx as hash_idx_t;
	  return value;
	}
      hash_put (h, key, value, hash);
      return value;
    }
  puthash_slow (h, key, value);
  return value;
});


DEFUN! ("remhash", Fremhash, Sremhash, 2, 2, 0,
       doc: "Remove KEY from TABLE.  ",
       (key: Lisp_Object, table: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (table);
  check_mutable_hash_table (table, h);
  hash_remove_from_table (h, key);
  return Qnil ();
});


DEFUN! ("maphash", Fmaphash, Smaphash, 2, 2, 0,
       doc: "Call FUNCTION for all entries in hash table TABLE.\nFUNCTION is called with two arguments, KEY and VALUE.\nIt should not alter TABLE in any way other than using `puthash' to\nset a new value for KEY, or `remhash' to remove KEY.\n`maphash' always returns nil.  ",
       (function: Lisp_Object, table: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (table);
  /* We can't use DOHASH here since FUNCTION may violate the rules and
     we shouldn't crash as a result (although the effects are
     unpredictable).  */
  /* C's `DOHASH_SAFE (h, i) calln (…)` (lisp.h 2756-2760): the loop macro is `deferred` in
     docs/ports/excluded-macros.txt, so its `for`/`if`/`else` is written out — the call runs
     for every entry whose key is in use, exactly as C's `if (…); else <body>` does.  */
  let mut i: ptrdiff_t = 0;
  while i < HASH_TABLE_SIZE (h) as ptrdiff_t
    {
      if hash_unused_entry_key_p (HASH_KEY (h, i as hash_idx_t))
	{ }
      else
	{ calln! (function, HASH_KEY (h, i as hash_idx_t),
		  HASH_VALUE (h, i as hash_idx_t)); }
      i += 1;
    }
  return Qnil ();
});


DEFUN! ("define-hash-table-test", Fdefine_hash_table_test,
       Sdefine_hash_table_test, 3, 3, 0,
       doc: "Define a new hash table test with name NAME, a symbol.\n\nIn hash tables created with NAME specified as test, use TEST to\ncompare keys, and HASH for computing hash codes of keys.\n\nTEST must be a function taking two arguments and returning non-nil if\nboth arguments are the same.  HASH must be a function taking one\nargument and returning an object that is the hash code of the argument.\nIt should be the case that if (eq (funcall HASH x1) (funcall HASH x2))\nreturns nil, then (funcall TEST x1 x2) also returns nil.  ",
       (name: Lisp_Object, test: Lisp_Object, hash: Lisp_Object),
       {
  return Fput (name, Qhash_table_test (), list2 (test, hash));
});

DEFUN! ("internal--hash-table-histogram",
       Finternal__hash_table_histogram,
       Sinternal__hash_table_histogram,
       1, 1, 0,
       doc: "Bucket size histogram of HASH-TABLE.  Internal use only. ",
       (hash_table: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (hash_table);
  let size: ptrdiff_t = HASH_TABLE_SIZE (h) as ptrdiff_t;
  let freq: *mut ptrdiff_t = xcalloc (size as usize,
				      std::mem::size_of::<ptrdiff_t> ()) as *mut ptrdiff_t;
  let index_size: ptrdiff_t = hash_table_index_size (h);
  let mut i: ptrdiff_t = 0;
  while i < index_size
    {
      let mut n: ptrdiff_t = 0;
      let mut j: ptrdiff_t = HASH_INDEX (h, i);
      while j != -1
	{
	  n += 1;
	  j = HASH_NEXT (h, j);
	}
      if n > 0
	{ *freq.offset (n - 1) += 1; }
      i += 1;
    }
  let mut ret: Lisp_Object = Qnil ();
  let mut i: ptrdiff_t = 0;
  while i < size
    {
      if *freq.offset (i) > 0
	{ ret = Fcons (Fcons (make_int ((i + 1) as i64), make_int (*freq.offset (i) as i64)),
		       ret); }
      i += 1;
    }
  xfree (freq as *mut c_void);
  return Fnreverse (ret);
});

DEFUN! ("internal--hash-table-buckets",
       Finternal__hash_table_buckets,
       Sinternal__hash_table_buckets,
       1, 1, 0,
       doc: "(KEY . HASH) in HASH-TABLE, grouped by bucket.\nInternal use only. ",
       (hash_table: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (hash_table);
  let mut ret: Lisp_Object = Qnil ();
  let index_size: ptrdiff_t = hash_table_index_size (h);
  let mut i: ptrdiff_t = 0;
  while i < index_size
    {
      let mut bucket: Lisp_Object = Qnil ();
      let mut j: ptrdiff_t = HASH_INDEX (h, i);
      while j != -1
	{
	  bucket = Fcons (Fcons (HASH_KEY (h, j as hash_idx_t),
				 make_int (HASH_HASH (h, j as hash_idx_t) as i64)),
			  bucket);
	  j = HASH_NEXT (h, j);
	}
      if !NILP (bucket)
	{ ret = Fcons (Fnreverse (bucket), ret); }
      i += 1;
    }
  return Fnreverse (ret);
});

DEFUN! ("internal--hash-table-index-size",
       Finternal__hash_table_index_size,
       Sinternal__hash_table_index_size,
       1, 1, 0,
       doc: "Index size of HASH-TABLE.  Internal use only. ",
       (hash_table: Lisp_Object),
       {
  let h: *mut Lisp_Hash_Table = check_hash_table (hash_table);
  return make_int (hash_table_index_size (h) as i64);
});


/************************************************************************
			MD5, SHA-1, SHA-2, and SHA-3
 ************************************************************************/

/* C 6136-6140: `#include "md5.h"`, `"sha1.h"`, `"sha256.h"`, `"sha512.h"`, `"sha3.h"`.
   Seven lib mirrors declare their extern block in a generic `pub mod decls`
   (lib/include/{filevercmp,md5,sha1,sha256,sha512,sha3,utimens}.rs), so **any two of them** in
   one unit is an `E0428` — and `xtask`'s materializer flattens a wrapping `mod`, so a unit
   cannot work around it locally.  This file needs six of the seven, and this is the first unit
   that does; `docs/ports/fns.md` reports the collision (the fix is a per-header module name in
   each mirror).  Until then the port includes exactly one of them — sha1.rs, whose `sha1_ctx`
   this unit needs for `buffer-hash` — and every other symbol it needs from the five headers is
   declared at its use site below with C's own prototype: `md5_buffer`, `sha224_buffer`,
   `sha256_buffer`, `sha384_buffer`, `sha512_buffer` and the four `sha3_*_buffer` entries (each
   `void *f (const char *, size_t, void *)`), plus the ten `*_DIGEST_SIZE` constants, whose
   values are the headers' own `bits / 8` expressions.  */
// [U1] ../lib/include/sha1.rs imported via common
// [U1] ../lib/include/md5.rs imported via common
// [U1] ../lib/include/sha256.rs imported via common
// [U1] ../lib/include/sha512.rs imported via common
// [U1] ../lib/include/sha3.rs imported via common
// [U1] ../lib/include/filevercmp.rs imported via common

/* Store into HEXBUF an unterminated hexadecimal character string
   representing DIGEST, which is binary data of size DIGEST_SIZE bytes.
   HEXBUF might equal DIGEST.  */
/* C 6141-6153: `hexbuf_digest`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn hexbuf_digest (hexbuf: *mut c_char, digest: *const c_void,
					digest_size: c_int)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let p: *const u8 = digest as *const u8;

  let mut i: c_int = digest_size - 1;
  while i >= 0
    {
      static hexdigit: [c_char; 16] = [b'0' as c_char, b'1' as c_char, b'2' as c_char,
				       b'3' as c_char, b'4' as c_char, b'5' as c_char,
				       b'6' as c_char, b'7' as c_char, b'8' as c_char,
				       b'9' as c_char, b'a' as c_char, b'b' as c_char,
				       b'c' as c_char, b'd' as c_char, b'e' as c_char,
				       b'f' as c_char];
      let p_i: c_int = *p.offset (i as isize) as c_int;
      *hexbuf.offset (2 * i as isize) = hexdigit[(p_i >> 4) as usize];
      *hexbuf.offset ((2 * i + 1) as isize) = hexdigit[(p_i & 0xf) as usize];
      i -= 1;
    }

  }
}

/* C 6155-6160: `make_digest_string`.  */
unsafe fn make_digest_string (digest: Lisp_Object, digest_size: c_int) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  hexbuf_digest (SSDATA (digest) as *mut c_char, SDATA (digest) as *const c_void,
		 digest_size);
  return digest;

  }
}

DEFUN! ("secure-hash-algorithms", Fsecure_hash_algorithms,
       Ssecure_hash_algorithms, 0, 0, 0,
       doc: "Return a list of all the supported `secure-hash' algorithms.",
       (),
       {
  return list! (Qmd5 (), Qsha1 (), Qsha224 (), Qsha256 (), Qsha384 (), Qsha512 (),
	       Qsha3_224 (), Qsha3_256 (), Qsha3_384 (), Qsha3_512 ());
});

/* Extract data from a string or a buffer. SPEC is a list of
(BUFFER-OR-STRING-OR-SYMBOL START END CODING-SYSTEM NOERROR) which behave as
specified with `secure-hash' and in Info node
`(elisp)Format of GnuTLS Cryptography Inputs'.  */
/* C 6106-6204: `extract_data_from_object`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn extract_data_from_object (spec: Lisp_Object,
						   start_byte: *mut ptrdiff_t,
						   end_byte: *mut ptrdiff_t)
  -> *mut c_char
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let object: Lisp_Object = XCAR (spec);
  let mut spec: Lisp_Object = spec;

  if CONSP (spec) { spec = XCDR (spec); }
  let start: Lisp_Object = CAR_SAFE (spec);

  if CONSP (spec) { spec = XCDR (spec); }
  let end: Lisp_Object = CAR_SAFE (spec);

  if CONSP (spec) { spec = XCDR (spec); }
  let mut coding_system: Lisp_Object = CAR_SAFE (spec);

  if CONSP (spec) { spec = XCDR (spec); }
  let noerror: Lisp_Object = CAR_SAFE (spec);

  if STRINGP (object)
    {
      if NILP (coding_system)
	{
	  /* Decide the coding-system to encode the data with.  */

	  if STRING_MULTIBYTE (object)
	    /* use default, we can't guess correct value */
	    { coding_system = preferred_coding_system (); }
	  else
	    { coding_system = Qraw_text (); }
	}

      if NILP (Fcoding_system_p (coding_system))
	{
	  /* Invalid coding system.  */

	  if !NILP (noerror)
	    { coding_system = Qraw_text (); }
	  else
	    { xsignal1 (Qcoding_system_error (), coding_system); }
	}

      let mut object: Lisp_Object = object;
      if STRING_MULTIBYTE (object)
	{ object = code_convert_string (object, coding_system,
					Qnil (), true, false, true); }

      let size: ptrdiff_t = SCHARS (object);
      let mut start_char: ptrdiff_t = 0;
      let mut end_char: ptrdiff_t = 0;
      validate_subarray (object, start, end, size, &raw mut start_char, &raw mut end_char);

      *start_byte = if start_char == 0 { 0 } else { string_char_to_byte (object, start_char) };
      *end_byte = if end_char == size
                   { SBYTES (object) }
                   else
                   { string_char_to_byte (object, end_char) };
      let _ = spec;
      return if !STRINGP (object) { std::ptr::null_mut () } else { SSDATA (object) };
    }
  else if BUFFERP (object)
    {
      let prev: *mut buffer = (current_buffer! ());
      let mut b: EMACS_INT;
      let mut e: EMACS_INT;

      record_unwind_current_buffer ();

      let bp: *mut buffer = XBUFFER (object);
      set_buffer_internal (bp);

      b = if !NILP (start) { fix_position (start) } else { BEGV! () };
      e = if !NILP (end) { fix_position (end) } else { ZV! () };
      if b > e
	{
	  let temp: EMACS_INT = b;
	  b = e;
	  e = temp;
	}

      if !(BEGV! () <= b && e <= ZV! ())
	{ args_out_of_range (start, end); }

      if NILP (coding_system)
	{
	  /* Decide the coding-system to encode the data with.
	     See fileio.c:Fwrite-region */

	  if !NILP ((*Vcoding_system_for_write.as_mut_ptr ()))
	    { coding_system = (*Vcoding_system_for_write.as_mut_ptr ()); }
	  else
	    {
	      let mut force_raw_text: bool = false;

	      coding_system = BVAR! (XBUFFER (object), buffer_file_coding_system_);
	      if NILP (coding_system)
		  || NILP (Flocal_variable_p (Qbuffer_file_coding_system (), Qnil ()))
		{
		  coding_system = Qnil ();
		  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
		    { force_raw_text = true; }
		}

	      if NILP (coding_system) && !NILP (Fbuffer_file_name (object))
		{
		  /* Check file-coding-system-alist.  */
		  let val: Lisp_Object = CALLN! (Ffind_operation_coding_system,
						 Qwrite_region (),
						 make_fixnum (b), make_fixnum (e),
						 Fbuffer_file_name (object));
		  if CONSP (val) && !NILP (XCDR (val))
		    { coding_system = XCDR (val); }
		}

	      if NILP (coding_system)
		  && !NILP (BVAR! (XBUFFER (object), buffer_file_coding_system_))
		{
		  /* If we still have not decided a coding system, use the
		     default value of buffer-file-coding-system.  */
		  coding_system = BVAR! (XBUFFER (object), buffer_file_coding_system_);
		}

	      if !force_raw_text
		  && !NILP (Ffboundp ((*Vselect_safe_coding_system_function.as_mut_ptr ())))
		/* Confirm that VAL can surely encode the current region.  */
		{ coding_system = calln! ((*Vselect_safe_coding_system_function.as_mut_ptr ()),
					  make_fixnum (b), make_fixnum (e),
					  coding_system, Qnil ()); }

	      if force_raw_text
		{ coding_system = Qraw_text (); }
	    }

	  if NILP (Fcoding_system_p (coding_system))
	    {
	      /* Invalid coding system.  */

	      if !NILP (noerror)
		{ coding_system = Qraw_text (); }
	      else
		{ xsignal1 (Qcoding_system_error (), coding_system); }
	    }
	}

      let mut object: Lisp_Object = make_buffer_string (b, e, false);
      set_buffer_internal (prev);
      /* Discard the unwind protect for recovering the current
	 buffer.  */
      (specpdl_ptr! ()) = (specpdl_ptr! ()).offset (-1);

      if STRING_MULTIBYTE (object)
	{ object = code_convert_string (object, coding_system,
					Qnil (), true, false, false); }
      *start_byte = 0;
      *end_byte = SBYTES (object);
      let _ = spec;
      return SSDATA (object) as *mut c_char;
    }
  else if EQ (object, Qiv_auto ())
    {
      /* Format: (iv-auto REQUIRED-LENGTH).  */

      if ! FIXNATP (start)
        { error (c"Without a length, `iv-auto' can't be used; see Elisp manual".as_ptr ()); }
      else
        {
	  let start_hold: EMACS_INT = XFIXNAT (start);
          let object: Lisp_Object = make_uninit_string (start_hold);

          *start_byte = 0;
          *end_byte = start_hold;
	  let lim: *mut c_char = SSDATA (object).offset (start_hold) as *mut c_char;
	  let mut p: *mut c_char = SSDATA (object) as *mut c_char;
	  while p < lim
	    {
	      let gotten: ssize_t = getrandom (p as *mut c_void, lim.offset_from (p) as usize, 0);
	      if 0 <= gotten
		{ p = p.offset (gotten as isize); }
	      else if *errno () != EINTR
		{ report_file_error (c"Getting random data".as_ptr (), Qnil ()); }
	    }

          let _ = spec;
          return SSDATA (object) as *mut c_char;
        }
    }

  let _ = spec;
  if !STRINGP (object)
    { signal_error (c"Invalid object argument".as_ptr (),
		    if NILP (object) { build_string (c"nil".as_ptr ()) } else { object }); }
  return SSDATA (object) as *mut c_char;

  }
}


/* ALGORITHM is a symbol: md5, sha1, sha224 and so on. */

/* C 6289-6372: `secure_hash`.  */
unsafe fn secure_hash (algorithm: Lisp_Object, object: Lisp_Object, start: Lisp_Object,
		       end: Lisp_Object, coding_system: Lisp_Object, noerror: Lisp_Object,
		       binary: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut start_byte: ptrdiff_t = 0;
  let mut end_byte: ptrdiff_t = 0;
  let mut digest_size: c_int;
  let mut hash_func: unsafe extern "C" fn (*const c_char, usize, *mut c_void) -> *mut c_void;
  let digest: Lisp_Object;

  CHECK_SYMBOL (algorithm);

  let spec: Lisp_Object = list5 (object, start, end, coding_system, noerror);

  let input: *const c_char = extract_data_from_object (spec, &raw mut start_byte,
						       &raw mut end_byte);

  if input.is_null ()
    { error (c"secure_hash: Failed to extract data from object, aborting!".as_ptr ()); }

  if EQ (algorithm, Qmd5 ())
    {
      digest_size = MD5_DIGEST_SIZE;
      hash_func	  = md5_buffer;
    }
  else if EQ (algorithm, Qsha1 ())
    {
      digest_size = SHA1_DIGEST_SIZE;
      hash_func	  = sha1_buffer;
    }
  else if EQ (algorithm, Qsha224 ())
    {
      digest_size = SHA224_DIGEST_SIZE;
      hash_func	  = sha224_buffer;
    }
  else if EQ (algorithm, Qsha256 ())
    {
      digest_size = SHA256_DIGEST_SIZE;
      hash_func	  = sha256_buffer;
    }
  else if EQ (algorithm, Qsha384 ())
    {
      digest_size = SHA384_DIGEST_SIZE;
      hash_func	  = sha384_buffer;
    }
  else if EQ (algorithm, Qsha512 ())
    {
      digest_size = SHA512_DIGEST_SIZE;
      hash_func	  = sha512_buffer;
    }
  else if EQ (algorithm, Qsha3_224 ())
    {
      digest_size = SHA3_224_DIGEST_SIZE;
      hash_func	  = sha3_224_buffer;
    }
  else if EQ (algorithm, Qsha3_256 ())
    {
      digest_size = SHA3_256_DIGEST_SIZE;
      hash_func	  = sha3_256_buffer;
    }
  else if EQ (algorithm, Qsha3_384 ())
    {
      digest_size = SHA3_384_DIGEST_SIZE;
      hash_func	  = sha3_384_buffer;
    }
  else if EQ (algorithm, Qsha3_512 ())
    {
      digest_size = SHA3_512_DIGEST_SIZE;
      hash_func	  = sha3_512_buffer;
    }
  else
    { error (c"Invalid algorithm arg: %s".as_ptr (), SDATA (Fsymbol_name (algorithm))); }

  /* allocate 2 x digest_size so that it can be reused to hold the
     hexified value */
  digest = make_uninit_string ((digest_size * 2) as EMACS_INT);

  hash_func (input.offset (start_byte),
	     (end_byte - start_byte) as usize,
	     SSDATA (digest) as *mut c_void);

  if NILP (binary)
    { return make_digest_string (digest, digest_size); }
  else
    { return make_unibyte_string (SSDATA (digest) as *mut c_char, digest_size as ptrdiff_t); }

  }
}

DEFUN! ("md5", Fmd5, Smd5, 1, 5, 0,
       doc: "Return MD5 message digest of OBJECT, a buffer or string.\n\nA message digest is the string representation of the cryptographic checksum\nof a document, and the algorithm to calculate it is defined in RFC 1321.\nThe MD5 digest is 32-character long.\n\nThe two optional arguments START and END are character positions\nspecifying for which part of OBJECT the message digest should be\ncomputed.  If nil or omitted, the digest is computed for the whole\nOBJECT.\n\nThe MD5 message digest is computed from the result of encoding the\ntext in a coding system, not directly from the internal Emacs form of\nthe text.  The optional fourth argument CODING-SYSTEM specifies which\ncoding system to encode the text with.  It should be the same coding\nsystem that you used or will use when actually writing the text into a\nfile.\n\nIf CODING-SYSTEM is nil or omitted, the default depends on OBJECT.  If\nOBJECT is a buffer, the default for CODING-SYSTEM is whatever coding\nsystem would be chosen by default for writing this text into a file.\n\nIf OBJECT is a string, the most preferred coding system (see the\ncommand `prefer-coding-system') is used.\n\nIf NOERROR is non-nil, silently assume the `raw-text' coding if the\nguesswork fails.  Normally, an error is signaled in such case.\n\nThis function is semi-obsolete, since for most purposes it is equivalent\nto calling `secure-hash` with the symbol `md5' as the ALGORITHM\nargument.  The OBJECT, START and END arguments have the same meanings as\nin `secure-hash'.\n\nNote that MD5 is not collision resistant and should not be used for\nanything security-related.  See `secure-hash' for alternatives.",
       (object: Lisp_Object, start: Lisp_Object, end: Lisp_Object,
        coding_system: Lisp_Object, noerror: Lisp_Object),
       {
  return secure_hash (Qmd5 (), object, start, end, coding_system, noerror, Qnil ());
});

DEFUN! ("secure-hash", Fsecure_hash, Ssecure_hash, 2, 5, 0,
       doc: "Return the secure hash of OBJECT, a buffer or string.\nALGORITHM is a symbol specifying the hash to use:\n- md5      corresponds to MD5, produces a 32-character signature\n- sha1     corresponds to SHA-1, produces a 40-character signature\n- sha224   corresponds to SHA-2 (SHA-224), produces a 56-character signature\n- sha256   corresponds to SHA-2 (SHA-256), produces a 64-character signature\n- sha384   corresponds to SHA-2 (SHA-384), produces a 96-character signature\n- sha512   corresponds to SHA-2 (SHA-512), produces a 128-character signature\n- sha3-224 corresponds to SHA-3 (SHA3-224), produces a 56-character signature\n- sha3-256 corresponds to SHA-3 (SHA3-256), produces a 64-character signature\n- sha3-384 corresponds to SHA-3 (SHA3-384), produces a 96-character signature\n- sha3-512 corresponds to SHA-3 (SHA3-512), produces a 128-character signature\n\nThe two optional arguments START and END are positions specifying for\nwhich part of OBJECT to compute the hash.  If nil or omitted, uses the\nwhole OBJECT.\n\nThe full list of algorithms can be obtained with `secure-hash-algorithms'.\n\nIf BINARY is non-nil, returns a string in binary form.  In this case,\nthe function returns a unibyte string whose length is half the number\nof characters it returns when BINARY is nil.\n\nNote that MD5 and SHA-1 are not collision resistant and should not be\nused for anything security-related.  For these applications, use one\nof the other hash types instead, e.g. sha256 or sha512.  ",
       (algorithm: Lisp_Object, object: Lisp_Object, start: Lisp_Object,
        end: Lisp_Object, binary: Lisp_Object),
       {
  return secure_hash (algorithm, object, start, end, Qnil (), Qnil (), binary);
});

DEFUN! ("buffer-hash", Fbuffer_hash, Sbuffer_hash, 0, 1, 0,
       doc: "Return a hash of the contents of BUFFER-OR-NAME.\nThis hash is performed on the raw internal format of the buffer,\ndisregarding any coding systems.  If nil, use the current buffer.\n\nThis function is useful for comparing two buffers running in the same\nEmacs, but is not guaranteed to return the same hash between different\nEmacs versions.  It should be somewhat more efficient on larger\nbuffers than `secure-hash' is, and should not allocate more memory.\n\nIt should not be used for anything security-related.  See\n`secure-hash' for these applications.  ",
       (buffer_or_name: Lisp_Object),
       {
  let buffer: Lisp_Object;
  let b: *mut buffer;
  let mut ctx: sha1_ctx = std::mem::zeroed ();

  if NILP (buffer_or_name)
    { buffer = Fcurrent_buffer (); }
  else
    { buffer = Fget_buffer (buffer_or_name); }
  if NILP (buffer)
    { nsberror (buffer_or_name); }

  b = XBUFFER (buffer);
  sha1_init_ctx (&raw mut ctx);

  /* Process the first part of the buffer. */
  sha1_process_bytes (BUF_BEG_ADDR! (b) as *const c_void,
		      (BUF_GPT_BYTE! (b) - BUF_BEG_BYTE (b)) as usize,
		      &raw mut ctx);

  /* If the gap is before the end of the buffer, process the last half
     of the buffer. */
  if BUF_GPT_BYTE! (b) < BUF_Z_BYTE! (b)
    { sha1_process_bytes (BUF_GAP_END_ADDR (b) as *const c_void,
			  BUF_Z_ADDR (b).offset_from (BUF_GAP_END_ADDR (b)) as usize,
			  &raw mut ctx); }

  let digest: Lisp_Object = make_uninit_string ((SHA1_DIGEST_SIZE * 2) as EMACS_INT);
  sha1_finish_ctx (&raw mut ctx, SSDATA (digest) as *mut c_void);
  return make_digest_string (digest, SHA1_DIGEST_SIZE);
});

DEFUN! ("buffer-line-statistics", Fbuffer_line_statistics,
       Sbuffer_line_statistics, 0, 1, 0,
       doc: "Return data about lines in BUFFER.\nThe data is returned as a list, and the first element is the number of\nlines in the buffer, the second is the length of the longest line, and\nthe third is the mean line length.  The lengths returned are in bytes, not\ncharacters.  ",
       (buffer_or_name: Lisp_Object),
       {
  let buffer: Lisp_Object;
  let mut lines: ptrdiff_t = 0;
  let mut longest: ptrdiff_t = 0;
  let mut mean: f64 = 0.0;
  let b: *mut buffer;

  if NILP (buffer_or_name)
    { buffer = Fcurrent_buffer (); }
  else
    { buffer = Fget_buffer (buffer_or_name); }
  if NILP (buffer)
    { nsberror (buffer_or_name); }

  b = XBUFFER (buffer);

  let mut start: *mut u8 = BUF_BEG_ADDR! (b);
  let mut area: ptrdiff_t = BUF_GPT_BYTE! (b) - BUF_BEG_BYTE (b);
  let mut pre_gap: ptrdiff_t = 0;

  /* Process the first part of the buffer. */
  while area > 0
    {
      let n: *mut u8 = memchr (start as *const c_void, b'\n' as c_int, area as usize) as *mut u8;

      if !n.is_null ()
	{
	  let this_line: ptrdiff_t = n.offset_from (start);
	  if this_line > longest
	    { longest = this_line; }
	  lines += 1;
	  /* Blame Knuth. */
	  mean = mean + (this_line as f64 - mean) / lines as f64;
	  area = area - this_line - 1;
	  start = start.offset (this_line + 1);
	}
      else
	{
	  /* Didn't have a newline here, so save the rest for the
	     post-gap calculation. */
	  pre_gap = area;
	  area = 0;
	}
    }

  /* If the gap is before the end of the buffer, process the last half
     of the buffer. */
  if BUF_GPT_BYTE! (b) < BUF_Z_BYTE! (b)
    {
      start = BUF_GAP_END_ADDR (b);
      area = BUF_Z_ADDR (b).offset_from (BUF_GAP_END_ADDR (b));

      while area > 0
	{
	  let n: *mut u8 = memchr (start as *const c_void, b'\n' as c_int, area as usize) as *mut u8;
	  let this_line: ptrdiff_t = if !n.is_null ()
	    { n.offset_from (start) + pre_gap } else { area + pre_gap };

	  if this_line > longest
	    { longest = this_line; }
	  lines += 1;
	  /* Blame Knuth again. */
	  mean = mean + (this_line as f64 - mean) / lines as f64;
	  area = area - this_line - 1;
	  start = start.offset (this_line + 1);
	  pre_gap = 0;
	}
    }
  else if pre_gap > 0
    {
      if pre_gap > longest
	{ longest = pre_gap; }
      lines += 1;
      mean = mean + (pre_gap as f64 - mean) / lines as f64;
    }

  return list3 (make_int (lines as i64), make_int (longest as i64), make_float (mean));
});

DEFUN! ("string-search", Fstring_search, Sstring_search, 2, 3, 0,
       doc: "Search for the string NEEDLE in the string HAYSTACK.\nThe return value is the position of the first occurrence of NEEDLE in\nHAYSTACK, or nil if no match was found.\n\nThe optional START-POS argument says where to start searching in\nHAYSTACK and defaults to zero (start at the beginning).\nIt must be between zero and the length of HAYSTACK, inclusive.\n\nCase is always significant and text properties are ignored.",
       (needle: Lisp_Object, haystack: Lisp_Object, start_pos: Lisp_Object),
       {
  let mut start_byte: ptrdiff_t = 0;
  let haybytes: ptrdiff_t;
  let res: *mut u8;
  let mut haystart: *mut u8;
  let mut start: EMACS_INT = 0;

  CHECK_STRING (needle);
  CHECK_STRING (haystack);

  if !NILP (start_pos)
    {
      CHECK_FIXNUM (start_pos);
      start = XFIXNUM (start_pos);
      if start < 0 || start > SCHARS (haystack)
        { xsignal1 (Qargs_out_of_range (), start_pos); }
      start_byte = string_char_to_byte (haystack, start);
    }

  /* If NEEDLE is longer than (the remaining length of) haystack, then
     we can't have a match, and return early.  */
  if SCHARS (needle) > SCHARS (haystack) - start
    { return Qnil (); }

  haystart = SSDATA (haystack).offset (start_byte) as *mut u8;
  haybytes = SBYTES (haystack) - start_byte;

  /* We can do a direct byte-string search if both strings have the
     same multibyteness, or if the needle consists of ASCII characters only.  */
  if if STRING_MULTIBYTE (haystack)
       { STRING_MULTIBYTE (needle)
         || SCHARS (haystack) == SBYTES (haystack) || string_ascii_p (needle) }
     else
       { !STRING_MULTIBYTE (needle)
         || SCHARS (needle) == SBYTES (needle) }
    {
      if STRING_MULTIBYTE (haystack) && STRING_MULTIBYTE (needle)
          && SCHARS (haystack) == SBYTES (haystack)
          && SCHARS (needle) != SBYTES (needle)
        /* Multibyte non-ASCII needle, multibyte ASCII haystack: impossible.  */
        { return Qnil (); }
      else
        { res = memmem (haystart as *const c_void, haybytes as usize,
                        SSDATA (needle) as *const c_void, SBYTES (needle) as usize)
	    as *mut u8; }
    }
  else if STRING_MULTIBYTE (haystack)  /* unibyte non-ASCII needle */
    {
      let multi_needle: Lisp_Object = string_to_multibyte (needle);
      res = memmem (haystart as *const c_void, haybytes as usize,
		    SSDATA (multi_needle) as *const c_void,
		    SBYTES (multi_needle) as usize) as *mut u8;
    }
  else              /* unibyte haystack, multibyte non-ASCII needle */
    {
      /* The only possible way we can find the multibyte needle in the
	 unibyte stack (since we know that the needle is non-ASCII) is
	 if they contain "raw bytes" (and no other non-ASCII chars.)  */
      let nbytes: ptrdiff_t = SBYTES (needle);
      let mut i: ptrdiff_t = 0;
      while i < nbytes
        {
          let c: c_int = SREF (needle, i as usize) as c_int;
          if CHAR_BYTE8_HEAD_P (c)
            { i += 1; }                /* Skip raw byte.  */
          else if !ASCII_CHAR_P (c as isize)
            { return Qnil (); }  /* Found a char that can't be in the haystack.  */
          i += 1;
        }

      /* "Raw bytes" (aka eighth-bit) are represented differently in
         multibyte and unibyte strings.  */
      let uni_needle: Lisp_Object = Fstring_to_unibyte (needle);
      res = memmem (haystart as *const c_void, haybytes as usize,
                    SSDATA (uni_needle) as *const c_void,
                    SBYTES (uni_needle) as usize) as *mut u8;
    }

  if res.is_null ()
    { return Qnil (); }

  return make_int (string_byte_to_char (haystack, res.offset_from (SDATA (haystack))) as i64);
});

DEFUN! ("object-intervals", Fobject_intervals, Sobject_intervals, 1, 1, 0,
       doc: "Return a copy of the text properties of OBJECT.\nOBJECT must be a buffer or a string.\n\nAltering this copy does not change the layout of the text properties\nin OBJECT.  ",
       (object: Lisp_Object),
       {
  let intervals: INTERVAL;

  if STRINGP (object)
    { intervals = string_intervals (object); }
  else if BUFFERP (object)
    { intervals = buffer_intervals (XBUFFER (object)); }
  else
    { wrong_type_argument (Qbuffer_or_string_p (), object); }

  if intervals.is_null ()
    { return Qnil (); }

  let mut collector: Lisp_Object = Qnil ();
  traverse_intervals (intervals, 0, Some (collect_interval),
		      &raw mut collector as *mut c_void);
  return Fnreverse (collector);
});

DEFUN! ("line-number-at-pos", Fline_number_at_pos,
       Sline_number_at_pos, 0, 2, 0,
       doc: "Return the line number at POSITION in the current buffer.\nIf POSITION is nil or omitted, it defaults to point's position in the\ncurrent buffer.\n\nIf the buffer is narrowed, the return value by default counts the lines\nfrom the beginning of the accessible portion of the buffer.  But if the\nsecond optional argument ABSOLUTE is non-nil, the value counts the lines\nfrom the absolute start of the buffer, disregarding the narrowing.  ",
       (position: Lisp_Object, absolute: Lisp_Object),
       {
  let mut pos_byte: ptrdiff_t;
  let mut start_byte: ptrdiff_t = BEGV_BYTE! ();

  if !BUFFER_LIVE_P ((current_buffer! ()))
    { error (c"Attempt to count lines in a dead buffer".as_ptr ()); }

  if MARKERP (position)
    {
      /* We don't trust the byte position if the marker's buffer is
         not the current buffer.  */
      if (*XMARKER (position)).buffer != (current_buffer! ()) as *mut c_void
	{ pos_byte = CHAR_TO_BYTE (marker_position (position)); }
      else
	{ pos_byte = marker_byte_position (position); }
    }
  else if NILP (position)
    { pos_byte = PT_BYTE! (); }
  else
    {
      CHECK_FIXNUM (position);
      let pos: ptrdiff_t = XFIXNUM (position);
      /* Check that POSITION is valid. */
      if pos < BEG as ptrdiff_t || pos > Z! ()
	{ args_out_of_range_3 (position, make_int (BEG as ptrdiff_t as i64), make_int (Z! () as i64)); }
      pos_byte = CHAR_TO_BYTE (pos);
    }

  if !NILP (absolute)
    { start_byte = BEG_BYTE as ptrdiff_t; }
  else if NILP (absolute)
    { pos_byte = clip_to_bounds (BEGV_BYTE! (), pos_byte, ZV_BYTE! ()); }

  /* Check that POSITION is valid. */
  if pos_byte < BEG_BYTE as ptrdiff_t || pos_byte > Z_BYTE! ()
    { args_out_of_range_3 (make_int (BYTE_TO_CHAR (pos_byte) as i64),
			   make_int (BEG as ptrdiff_t as i64), make_int (Z! () as i64)); }

  return make_int ((count_lines (start_byte, pos_byte) + 1) as i64);
});
/* C 6741-6973: `syms_of_fns`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_fns ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* Hash table stuff.  */
  DEFSYM! (Qhash_table_p, c"hash-table-p");
  DEFSYM! (Qeq, c"eq");
  DEFSYM! (Qeql, c"eql");
  DEFSYM! (Qequal, c"equal");
  DEFSYM! (QCtest, c":test");
  DEFSYM! (QCsize, c":size");
  DEFSYM! (QCpurecopy, c":purecopy");
  DEFSYM! (QCrehash_size, c":rehash-size");
  DEFSYM! (QCrehash_threshold, c":rehash-threshold");
  DEFSYM! (QCweakness, c":weakness");
  DEFSYM! (Qkey, c"key");
  DEFSYM! (Qvalue, c"value");
  DEFSYM! (Qhash_table_test, c"hash-table-test");
  DEFSYM! (Qkey_or_value, c"key-or-value");
  DEFSYM! (Qkey_and_value, c"key-and-value");
  defsubr (Ssxhash_eq.as_mut_ptr ());
  defsubr (Ssxhash_eql.as_mut_ptr ());
  defsubr (Ssxhash_equal.as_mut_ptr ());
  defsubr (Ssxhash_equal_including_properties.as_mut_ptr ());
  defsubr (Smake_hash_table.as_mut_ptr ());
  defsubr (Scopy_hash_table.as_mut_ptr ());
  defsubr (Shash_table_count.as_mut_ptr ());
  defsubr (Shash_table_rehash_size.as_mut_ptr ());
  defsubr (Shash_table_rehash_threshold.as_mut_ptr ());
  defsubr (Shash_table_size.as_mut_ptr ());
  defsubr (Shash_table_test.as_mut_ptr ());
  defsubr (Shash_table_weakness.as_mut_ptr ());
  defsubr (Shash_table_p.as_mut_ptr ());
  defsubr (Sclrhash.as_mut_ptr ());
  defsubr (Sgethash.as_mut_ptr ());
  defsubr (Sputhash.as_mut_ptr ());
  defsubr (Sremhash.as_mut_ptr ());
  defsubr (Smaphash.as_mut_ptr ());
  defsubr (Sdefine_hash_table_test.as_mut_ptr ());
  defsubr (Sinternal__hash_table_histogram.as_mut_ptr ());
  defsubr (Sinternal__hash_table_buckets.as_mut_ptr ());
  defsubr (Sinternal__hash_table_index_size.as_mut_ptr ());
  defsubr (Sstring_search.as_mut_ptr ());
  defsubr (Sobject_intervals.as_mut_ptr ());
  defsubr (Sline_number_at_pos.as_mut_ptr ());
  /* Crypto and hashing stuff.  */
  DEFSYM! (Qiv_auto, c"iv-auto");
  DEFSYM! (Qmd5, c"md5");
  DEFSYM! (Qsha1, c"sha1");
  DEFSYM! (Qsha224, c"sha224");
  DEFSYM! (Qsha256, c"sha256");
  DEFSYM! (Qsha384, c"sha384");
  DEFSYM! (Qsha512, c"sha512");
  DEFSYM! (Qsha3_224, c"sha3-224");
  DEFSYM! (Qsha3_256, c"sha3-256");
  DEFSYM! (Qsha3_384, c"sha3-384");
  DEFSYM! (Qsha3_512, c"sha3-512");
  /* Miscellaneous stuff.  */
  DEFSYM! (Qstring_lessp, c"string-lessp");
  DEFSYM! (Qprovide, c"provide");
  DEFSYM! (Qrequire, c"require");
  DEFSYM! (Qyes_or_no_p_history, c"yes-or-no-p-history");
  DEFSYM! (Qcursor_in_echo_area, c"cursor-in-echo-area");
  DEFSYM! (Qwidget_type, c"widget-type");
  DEFVAR_LISP! (c"overriding-plist-environment", Voverriding_plist_environment,
	  " An alist that overrides the plists of the symbols which it lists.\nUsed by the byte-compiler to apply `define-symbol-prop' during\ncompilation.  ");
  (*Voverriding_plist_environment.as_mut_ptr ()) = Qnil ();
  DEFSYM! (Qoverriding_plist_environment, c"overriding-plist-environment");
  staticpro (string_char_byte_cache_string.as_mut_ptr ());
  (*string_char_byte_cache_string.as_mut_ptr ()) = Qnil ();
  (*require_nesting_list.as_mut_ptr ()) = Qnil ();
  staticpro (require_nesting_list.as_mut_ptr ());
  Fset (Qyes_or_no_p_history (), Qnil ());
  DEFVAR_LISP! (c"features", Vfeatures,
	  " A list of symbols which are the features of the executing Emacs.\nUsed by `featurep' and `require', and altered by `provide'.  ");
  (*Vfeatures.as_mut_ptr ()) = list1 (Qemacs ());
  DEFSYM! (Qfeatures, c"features");
  /* Let people use lexically scoped vars named `features'.  */
  Fmake_var_non_special (Qfeatures ());
  DEFSYM! (Qsubfeatures, c"subfeatures");
  DEFSYM! (Qfuncall, c"funcall");
  DEFSYM! (Qplistp, c"plistp");
  DEFSYM! (Qlist_or_vector_p, c"list-or-vector-p");
  /* #ifdef HAVE_LANGINFO_CODESET */
  DEFSYM! (Qcodeset, c"codeset");
  DEFSYM! (Qdays, c"days");
  DEFSYM! (Qmonths, c"months");
  DEFSYM! (Qpaper, c"paper");
  /* #endif	/* HAVE_LANGINFO_CODESET */ */
  DEFVAR_BOOL! (c"use-dialog-box", use_dialog_box,
	  " Non-nil means mouse commands use dialog boxes to ask questions.\nThis applies to `y-or-n-p' and `yes-or-no-p' questions asked by commands\ninvoked by mouse clicks and mouse menu items.\n\nOn some platforms, file selection dialogs are also enabled if this is\nnon-nil.  ");
  (*use_dialog_box.as_mut_ptr ()) = true;
  DEFVAR_BOOL! (c"use-file-dialog", use_file_dialog,
	  " Non-nil means mouse commands use a file dialog to ask for files.\nThis applies to commands from menus and tool bar buttons even when\nthey are initiated from the keyboard.  If `use-dialog-box' is nil,\nthat disables the use of a file dialog, regardless of the value of\nthis variable.  ");
  (*use_file_dialog.as_mut_ptr ()) = true;
  DEFVAR_BOOL! (c"use-short-answers", use_short_answers,
	  " Non-nil means `yes-or-no-p' accepts single-key answers \"y\" or \"n\".\nWhen non-nil, `yes-or-no-p' will use `y-or-n-p' to read the answer.\nThis means the user will be able to press just one key to answer, whereas\nby default the user needs to type the full \"yes\" or \"no\" response\nand then press RET.\n\nWe recommend against setting this variable non-nil, because `yes-or-no-p'\nis intended to be used when users are expected not to respond too\nquickly, but to take their time and perhaps think about the answer.\nThe same variable also affects the function `read-answer'.  See also\n`yes-or-no-prompt'.  ");
  (*use_short_answers.as_mut_ptr ()) = false;
  DEFVAR_LISP! (c"yes-or-no-prompt", Vyes_or_no_prompt,
	  " String to append when `yes-or-no-p' asks a question.\nFor best results this should end in a space.  ");
  (*Vyes_or_no_prompt.as_mut_ptr ()) = build_unibyte_string (c"(yes or no) ".as_ptr ());
  defsubr (Sidentity.as_mut_ptr ());
  defsubr (Srandom.as_mut_ptr ());
  defsubr (Slength.as_mut_ptr ());
  defsubr (Ssafe_length.as_mut_ptr ());
  defsubr (Slength_less.as_mut_ptr ());
  defsubr (Slength_greater.as_mut_ptr ());
  defsubr (Slength_equal.as_mut_ptr ());
  defsubr (Sproper_list_p.as_mut_ptr ());
  defsubr (Sstring_bytes.as_mut_ptr ());
  defsubr (Sstring_distance.as_mut_ptr ());
  defsubr (Sstring_equal.as_mut_ptr ());
  defsubr (Scompare_strings.as_mut_ptr ());
  defsubr (Sstring_lessp.as_mut_ptr ());
  defsubr (Sstring_version_lessp.as_mut_ptr ());
  defsubr (Sstring_collate_lessp.as_mut_ptr ());
  defsubr (Sstring_collate_equalp.as_mut_ptr ());
  defsubr (Sappend.as_mut_ptr ());
  defsubr (Sconcat.as_mut_ptr ());
  defsubr (Svconcat.as_mut_ptr ());
  defsubr (Scopy_sequence.as_mut_ptr ());
  defsubr (Sstring_make_multibyte.as_mut_ptr ());
  defsubr (Sstring_make_unibyte.as_mut_ptr ());
  defsubr (Sstring_as_multibyte.as_mut_ptr ());
  defsubr (Sstring_as_unibyte.as_mut_ptr ());
  defsubr (Sstring_to_multibyte.as_mut_ptr ());
  defsubr (Sstring_to_unibyte.as_mut_ptr ());
  defsubr (Scopy_alist.as_mut_ptr ());
  defsubr (Ssubstring.as_mut_ptr ());
  defsubr (Ssubstring_no_properties.as_mut_ptr ());
  defsubr (Stake.as_mut_ptr ());
  defsubr (Sntake.as_mut_ptr ());
  defsubr (Snthcdr.as_mut_ptr ());
  defsubr (Snth.as_mut_ptr ());
  defsubr (Selt.as_mut_ptr ());
  defsubr (Smember.as_mut_ptr ());
  defsubr (Smemq.as_mut_ptr ());
  defsubr (Smemql.as_mut_ptr ());
  defsubr (Sassq.as_mut_ptr ());
  defsubr (Sassoc.as_mut_ptr ());
  defsubr (Srassq.as_mut_ptr ());
  defsubr (Srassoc.as_mut_ptr ());
  defsubr (Sdelq.as_mut_ptr ());
  defsubr (Sdelete.as_mut_ptr ());
  defsubr (Snreverse.as_mut_ptr ());
  defsubr (Sreverse.as_mut_ptr ());
  defsubr (Ssort.as_mut_ptr ());
  defsubr (Splist_get.as_mut_ptr ());
  defsubr (Sget.as_mut_ptr ());
  defsubr (Splist_put.as_mut_ptr ());
  defsubr (Sput.as_mut_ptr ());
  defsubr (Seql.as_mut_ptr ());
  defsubr (Sequal.as_mut_ptr ());
  defsubr (Sequal_including_properties.as_mut_ptr ());
  defsubr (Svaluelt.as_mut_ptr ());
  defsubr (Sfillarray.as_mut_ptr ());
  defsubr (Sclear_string.as_mut_ptr ());
  defsubr (Snconc.as_mut_ptr ());
  defsubr (Smapcar.as_mut_ptr ());
  defsubr (Smapc.as_mut_ptr ());
  defsubr (Smapcan.as_mut_ptr ());
  defsubr (Smapconcat.as_mut_ptr ());
  defsubr (Syes_or_no_p.as_mut_ptr ());
  defsubr (Sload_average.as_mut_ptr ());
  defsubr (Sfeaturep.as_mut_ptr ());
  defsubr (Srequire.as_mut_ptr ());
  defsubr (Sprovide.as_mut_ptr ());
  defsubr (Splist_member.as_mut_ptr ());
  defsubr (Sbase64_encode_region.as_mut_ptr ());
  defsubr (Sbase64_decode_region.as_mut_ptr ());
  defsubr (Sbase64_encode_string.as_mut_ptr ());
  defsubr (Sbase64_decode_string.as_mut_ptr ());
  defsubr (Sbase64url_encode_region.as_mut_ptr ());
  defsubr (Sbase64url_encode_string.as_mut_ptr ());
  defsubr (Smd5.as_mut_ptr ());
  defsubr (Ssecure_hash_algorithms.as_mut_ptr ());
  defsubr (Ssecure_hash.as_mut_ptr ());
  defsubr (Sbuffer_hash.as_mut_ptr ());
  defsubr (Slocale_info.as_mut_ptr ());
  defsubr (Sbuffer_line_statistics.as_mut_ptr ());
  DEFSYM! (Qreal_this_command, c"real-this-command");
  DEFSYM! (Qfrom__tty_menu_p, c"from--tty-menu-p");
  DEFSYM! (Qyes_or_no_p, c"yes-or-no-p");
  DEFSYM! (Qy_or_n_p, c"y-or-n-p");
  DEFSYM! (QCkey, c":key");
  DEFSYM! (QClessp, c":lessp");
  DEFSYM! (QCin_place, c":in-place");
  DEFSYM! (QCreverse, c":reverse");
  DEFSYM! (Qvaluelt, c"value<");
  /* sleep-event states.  */
  DEFSYM! (Qpre_sleep, c"pre-sleep");
  DEFSYM! (Qpost_wake, c"post-wake");

  }
}

/* The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
   (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
   port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
   defines it here (status 3.29, the casetab.rs/editfns.rs shape).  */
#[unsafe(no_mangle)]
pub static Voverriding_plist_environment: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vfeatures: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vyes_or_no_prompt: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static use_dialog_box: VmGlobal<bool> = VmGlobal::<bool>::new (false);
#[unsafe(no_mangle)]
pub static use_file_dialog: VmGlobal<bool> = VmGlobal::<bool>::new (false);
#[unsafe(no_mangle)]
pub static use_short_answers: VmGlobal<bool> = VmGlobal::<bool>::new (false);
