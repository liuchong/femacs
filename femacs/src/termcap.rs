// femacs: port of src/termcap.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, file-scope object, local), the same control flow and the same error model.
// The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  C 18-19
// says so itself, and the build system agrees: `configure.ac` 6179-6183 puts `terminfo.o` in
// `TERMCAP_OBJ` when TERMINFO is yes and `tparam.o` otherwise - `termcap.o` is never in it - and
// only the MS-DOS build adds it, by rewriting the generated Makefile's empty `TERMCAP_OBJ`
// (`emacs/msdos/sed1v2.inp` 91: `s/@TERMCAP_OBJ@/termcap.o tparam.o/`).  `emacs/src/Makefile.in`
// 194 states that rule, 308 adds `termcap.o` to the MS-DOS + window-system source list and 485
// lists it in `SOME_MACHINE_OBJECTS`, which feeds the DOC file rather than the binary.
// `docs/baseline/frozen-build-sources.txt` therefore lists no `termcap.o` for this target, C's
// text is the whole file, and this unit's `## C line ranges` table in `docs/ports/termcap.md` is
// that clause's substitute (P-86 ②).
//
// **The file gate is `feature = "msdos"`** - C 32's `#ifdef MSDOS` names the same condition - on
// every item *and on the include graph* (P-86 ⑧): an ungated `include!` would put this file's
// declarations and definitions into the frozen build at once.
//
// Conditions: `python3 tools/check-cond-eval.py src/termcap.c` -> `9 conditions, 0 unknown,
// 0 disagreements [VERIFIED]`; the frozen table decides all nine.  Seven of the nine *sites* are
// false there (`MSDOS` at C 32/326/383, `DEBUG` at C 44, `IS_EBCDIC_HOST` at C 111/193,
// `INTERNAL_TERMINAL` at C 359) and the two `#ifndef` knobs (C 43/53) are true, so *their* arms are
// C's own fallbacks.  Every branch is carried with both arms under its own gate (P-86 ①);
// `docs/ports/termcap.md`'s gate table names each arm's C lines and its verdict.
//
// **The termcap surface is not a third-party header.**  `tparam.h` (C 31) declares the five entry
// points, `PC`, `BC` and `UP`, and its mirror is `include/tparam.rs`, which this unit includes
// exactly where C does.  This unit *defines* the five entry points and `PC` - the shadowing shape
// P-21/P-39 uses (a mirror's `pub mod` plus `use …::*;` is glob-imported, so a same-named
// definition in the unit takes over, while a second *declaration* would be the `E0428` that
// `tools/check-mirror-dupes.py` reports).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

include!("include/config.rs");                  // C 21: `#include <config.h>`
/* C 24's `#include <sys/file.h>` has no mirror; the only name this file takes through it is
   `O_RDONLY`, whose home is `lib/include/fcntl.in.rs`, included right below at C 25 (the
   `lread.rs` 60 and `filelock.rs` 28 reading of the same header).  */
#[cfg (feature = "msdos")]
include!("../lib/include/stdlib.rs");           // C 23: `#include <stdlib.h>` (`getenv`)
#[cfg (feature = "msdos")]
include!("../lib/include/fcntl.in.rs");         // C 25: `#include <fcntl.h>` (`O_RDONLY`, `O_TEXT`)
#[cfg (feature = "msdos")]
include!("../lib/include/unistd.in.rs");        // C 26: `#include <unistd.h>` (`read`, `lseek`)
#[cfg (feature = "msdos")]
include!("../lib/include/intprops.rs");         // C 28: `#include <intprops.h>`
#[cfg (feature = "msdos")]
include!("include/lisp.rs");                    // C 30: `#include "lisp.h"`
/* `lisp.h` 30 includes `<string.h>`; its mirror is `lib/include/string.rs`, which the lisp.rs
   chain does not carry, so the edge is spelled here (P-86 ⑧; the `gfilenotify.rs` 48 shape).  */
#[cfg (feature = "msdos")]
include!("../lib/include/string.rs");           // lisp.h 30: `#include <string.h>` (`strcmp`, `strcpy`, `memcpy`)
#[cfg (feature = "msdos")]
include!("include/tparam.rs");                  // C 31: `#include "tparam.h"`
#[cfg (feature = "msdos")]
include!("include/msdos.rs");                   // C 32-34: `#ifdef MSDOS` / `#include "msdos.h"` / `#endif`

/* C 23's `<stdlib.h>` mirror carries `getenv`, `strtol`, `strtod` and the allocator entries but
   not `atoi`, whose only use in this file is C 87's.  The declaration stands in this unit
   (P-88 ①, the `font.rs` 386 and `tparam.rs` 35 shape) and belongs in that mirror when the mirror
   round reaches it (Owed 4).  */
#[cfg (feature = "msdos")]
unsafe extern "C" {
  /// C: `<stdlib.h>`: `int atoi (const char *);`
  pub fn atoi (s: *const c_char) -> c_int;
}

/* C 18-19: `Since 2010-03, 073589f4, <the program> 24.1, this file is only used by the MS-DOS
   port of <the program>.` - the two occurrences of the program's own name are replaced by the
   placeholder (PORTING.md 0.1-19's second half, the `emacs.rs` `RCS_Id` precedent status 3.73): a
   quotation of the reference's sentence must not read as a claim about which program *this*
   repository is.  That sentence plus the build files above are the whole of the file gate's
   authority.  */

/* BUFSIZE is the initial size allocated for the buffer
   for reading the termcap file.
   It is not a limit.
   Make it large normally for speed.
   Make it variable when debugging, so can exercise
   increasing the space dynamically.  */

/* C 43-51: `#ifndef BUFSIZE` / `#ifdef DEBUG` / `#define BUFSIZE bufsize` / `int bufsize = 128;` /
   `#else` / `#define BUFSIZE 2048` / `#endif` / `#endif`.

   The live arm is C 49's `2048`, so the constant below answers for the macro - the `sound.rs`
   1293 `DEFAULT_ALSA_SOUND_DEVICE` shape for an `#ifndef` knob with a fallback.  The `DEBUG` arm
   turns the name into a *read* of the global C 47 defines (`#define BUFSIZE bufsize`), so that arm
   keeps the object and C 425 - the one use site - spells the read; two arms of one C name cannot
   share one Rust item (`E0428`), which is why the second arm is the object plus the use site
   rather than a second `BUFSIZE`.  */
#[cfg (all (feature = "msdos", not (feature = "bufsize")))]
pub const BUFSIZE: c_int = 2048;

/// C: `int bufsize = 128;` (C 47, the `DEBUG` arm's own global)
#[cfg (all (feature = "msdos", not (feature = "bufsize"), feature = "debug"))]
#[unsafe(no_mangle)]
pub static mut bufsize: c_int = 128;

/* C 53-55: `#ifndef TERMCAP_FILE` / `#define TERMCAP_FILE "/etc/termcap"` / `#endif`.  Another
   fallback knob: nothing in the reference tree defines it (`configure.ac` does not, and there is
   no `TERMCAP_FILE` row in the frozen macro table), so C 54's literal is the value this
   configuration would use.  */
/// C: `#define TERMCAP_FILE "/etc/termcap"`.  The constant is a `&CStr` rather than an
/// `*const c_char` because C's macro *is* the literal: the ABI gate compares a C string macro's
/// bytes against a bare Rust literal (`rust_string_bytes` reads the literal, and
/// `tools/extract-abi.py` reports `different bytes` for `c"…".as_ptr ()`), which is the
/// `alloc.rs` 5497 and `emacs.rs` 359 shape.
#[cfg (all (feature = "msdos", not (feature = "termcap_file")))]
pub const TERMCAP_FILE: &core::ffi::CStr = c"/etc/termcap";

/* Looking up capabilities in the entry already found.  */

/* The pointer to the data made by tgetent is left here
   for tgetnum, tgetflag and tgetstr to find.  */
#[cfg (feature = "msdos")]
static mut term_entry: *mut c_char = std::ptr::null_mut ();

/* C 64 and C 318-321 are C's static forward declarations (`tgetst1`, `scan_file`,
   `gobble_line`, `compare_contin`, `name_match`).  C's declaration/definition pair has no Rust
   counterpart: an item is its declaration, item order is irrelevant, and a second item of one
   name is `E0428` (PORTING.md 0.1-12), so the prototypes are dropped and each definition stands
   at its own C position (Deviations 1).  */

/* Search entry BP for capability CAP.
   Return a pointer to the capability (in BP) if found,
   0 if not found.  */

#[cfg (feature = "msdos")]
unsafe fn find_capability (mut bp: *mut c_char, cap: *const c_char) -> *mut c_char
{
  /* C's `register` is a storage-class hint with no Rust counterpart; every `register` in this
     file is dropped (Deviations 2).  C's `for (; *bp; bp++)` increments *after* the body, so the
     port's `while` advances at the same point.  */
  while *bp != 0
    {
      if *bp == b':' as c_char
	  && *bp.offset (1) == *cap
	  && *bp.offset (2) == *cap.offset (1)
	{ return bp.offset (4); }
      bp = bp.offset (1);
    }
  return std::ptr::null_mut ();
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tgetnum (cap: *const c_char) -> c_int
{
  let ptr: *mut c_char = find_capability (term_entry, cap);
  if ptr.is_null () || *ptr.offset (-1) != b'#' as c_char
    { return -1; }
  return atoi (ptr);
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tgetflag (cap: *const c_char) -> c_int
{
  let ptr: *mut c_char = find_capability (term_entry, cap);
  return (!ptr.is_null () && *ptr.offset (-1) == b':' as c_char) as c_int;
}

/* Look up a string-valued capability CAP.
   If AREA is non-null, it points to a pointer to a block in which
   to store the string.  That pointer is advanced over the space used.
   If AREA is null, space is allocated with `malloc'.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tgetstr (cap: *const c_char, area: *mut *mut c_char) -> *mut c_char
{
  let ptr: *mut c_char = find_capability (term_entry, cap);
  if ptr.is_null ()
      || (*ptr.offset (-1) != b'=' as c_char && *ptr.offset (-1) != b'~' as c_char)
    { return std::ptr::null_mut (); }
  return tgetst1 (ptr, area);
}

/* C 111-120 and C 121-131: one `static const char esctab[]` spelled twice, `#ifdef
   IS_EBCDIC_HOST` and its `#else`.  Both arms are carried, each under its own gate (P-86 ①);
   IS_EBCDIC_HOST is defined nowhere in the frozen configuration, so the second is the live one.
   C's backslash-newline splices each table into one string, and C's `[]` from a string literal
   also appends the terminating NUL: 64 + 1 entries for the EBCDIC table, 32 + 1 for the other
   (only the non-NUL entries are ever read).  The elements are spelled byte by byte because Rust's
   `*b"…"` is `[u8; N]` and C's element type here is `char` (Deviations 3).  */

/* Table, indexed by a character in range 0200 to 0300 with 0200 subtracted,
   gives meaning of character following \, or a space if no special meaning.
   Sixteen characters per line within the string.  */

/// C: `static const char esctab[] = " \057\026  \047\014         \025   \015      \005 \013          ";`
#[cfg (all (feature = "msdos", feature = "is_ebcdic_host"))]
static esctab: [c_char; 65] =
  [
    b' ' as c_char, 0o57 as c_char, 0o26 as c_char, b' ' as c_char,     /* 0- 3 */
    b' ' as c_char, 0o47 as c_char, 0o14 as c_char, b' ' as c_char,     /* 4- 7 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 8-11 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 12-15 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 16-19 */
    b' ' as c_char, 0o25 as c_char, b' ' as c_char, b' ' as c_char,     /* 20-23 */
    b' ' as c_char, 0o15 as c_char, b' ' as c_char, b' ' as c_char,     /* 24-27 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 28-31 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, 0o5 as c_char,      /* 32-35 */
    b' ' as c_char, 0o13 as c_char, b' ' as c_char, b' ' as c_char,     /* 36-39 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 40-43 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 44-47 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 48-51 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 52-55 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 56-59 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,     /* 60-63 */
    0,                                                                  /* C's terminating NUL */
  ];

/* Table, indexed by a character in range 0100 to 0140 with 0100 subtracted,
   gives meaning of character following \, or a space if no special meaning.
   Eight characters per line within the string.  */

/// C: `static const char esctab[] = " \007\010  \033\014 \012 \015 \011 \013         ";`
#[cfg (all (feature = "msdos", not (feature = "is_ebcdic_host")))]
static esctab: [c_char; 33] =
  [
    b' ' as c_char, 0o7 as c_char, 0o10 as c_char, b' ' as c_char,     /* 0- 3 */
    b' ' as c_char, 0o33 as c_char, 0o14 as c_char, b' ' as c_char,    /* 4- 7 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,    /* 8-11 */
    b' ' as c_char, b' ' as c_char, 0o12 as c_char, b' ' as c_char,    /* 12-15 */
    b' ' as c_char, b' ' as c_char, 0o15 as c_char, b' ' as c_char,    /* 16-19 */
    b' ' as c_char, 0o11 as c_char, b' ' as c_char, 0o13 as c_char,    /* 20-23 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,    /* 24-27 */
    b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char,    /* 28-31 */
    0,                                                                  /* C's terminating NUL */
  ];

/* PTR points to a string value inside a termcap entry.
   Copy that value, processing \ and ^ abbreviations,
   into the block that *AREA points to,
   or to newly allocated storage if AREA is NULL.
   Return the address to which we copied the value,
   or NULL if PTR is NULL.  */

/// C 224: C's anonymous `struct { char *beg; int len; }` for `cut[11]`.  Rust needs a name for
/// it (PORTING.md 1.3, the `font.rs` 1446 `font_expand_range` shape); the fields and their order
/// are C's.
#[cfg (feature = "msdos")]
#[repr(C)]
pub struct tgetst1_cut
{
  pub beg: *mut c_char,
  pub len: c_int,
}

#[cfg (feature = "msdos")]
unsafe fn tgetst1 (ptr: *mut c_char, area: *mut *mut c_char) -> *mut c_char
{
  let mut p: *mut c_char;
  let mut r: *mut c_char;
  let mut c: c_int;
  let mut size: c_int = 0;
  let mut ret: *mut c_char;
  /* C leaves `size` and `c1` uninitialised; C's zero value is the port's starting point
     (PORTING.md 0.1-13; both are written before any read on every path C reaches).  */
  let mut c1: c_int = 0;

  if ptr.is_null ()
    { return std::ptr::null_mut (); }

  /* `ret' gets address of where to store the string.  */
  if area.is_null ()
    {
      /* Compute size of block needed (may overestimate).  */
      p = ptr;
      while { c = *p as c_int; p = p.offset (1); c != 0 && c != b':' as c_int && c != b'\n' as c_int }
	{ continue; }
      ret = xmalloc ((p.offset_from (ptr) + 1) as size_t) as *mut c_char;
    }
  else
    { ret = *area; }

  /* Copy the string value, stopping at null or colon.
     Also process ^ and \ abbreviations.  */
  p = ptr;
  r = ret;
  while { c = *p as c_int; p = p.offset (1); c != 0 && c != b':' as c_int && c != b'\n' as c_int }
    {
      if c == b'^' as c_int
	{
	  c = *p as c_int;
	  p = p.offset (1);
	  if c == b'?' as c_int
	    { c = 0o177; }
	  else
	    { c &= 0o37; }
	}
      else if c == b'\\' as c_int
	{
	  c = *p as c_int;
	  p = p.offset (1);
	  if c >= b'0' as c_int && c <= b'7' as c_int
	    {
	      c -= b'0' as c_int;
	      size = 0;

	      while { size += 1;
		      size < 3 && { c1 = *p as c_int;
				    c1 >= b'0' as c_int && c1 <= b'7' as c_int } }
		{
		  c *= 8;
		  c += c1 - b'0' as c_int;
		  p = p.offset (1);
		}
	    }
	  else
	    {
	      /* C 193-199's `#ifdef IS_EBCDIC_HOST` arm and C 200-206's `#else`.  C writes the
		 two as one `else if` chain on the inner `if`; each arm is carried under its own
		 gate inside C's `else` (P-86 ①).  */
	      #[cfg (feature = "is_ebcdic_host")]
	      if c >= 0o200 && c < 0o360
		{
		  c1 = esctab[((c & !0o100) - 0o200) as usize] as c_int;
		  if c1 != b' ' as c_int
		    { c = c1; }
		}
	      #[cfg (not (feature = "is_ebcdic_host"))]
	      if c >= 0o100 && c < 0o200
		{
		  c1 = esctab[((c & !0o40) - 0o100) as usize] as c_int;
		  if c1 != b' ' as c_int
		    { c = c1; }
		}
	    }
	}
      *r = c as c_char;
      r = r.offset (1);
    }

  /* Sometimes entries have "%pN" which means use parameter N in the
     next %-substitution.  If all such N are continuous in the range
     [1,9] we can remove each "%pN" because they are redundant, thus
     reducing bandwidth requirements.  True, Emacs is well beyond the
     days of 150baud teletypes, but some of its users aren't much so.

     This pass could probably be integrated into the one above but
     abbreviation expansion makes that effort a little more hairy than
     its worth; this is cleaner.  */
  {
    let mut last_p_param: c_int = 0;
    let mut remove_p_params: bool = true;
    let mut cut: [tgetst1_cut; 11] = std::mem::zeroed ();

    p = ret;
    cut[0].beg = p;
    while p < r.offset (-3)
      {
	if !remove_p_params
	  { break; }
	if *p == b'%' as c_char && *p.offset (1) == b'p' as c_char
	  {
	    if *p.offset (2) as c_int - b'0' as c_int == 1 + last_p_param
	      {
		cut[last_p_param as usize].len
		  = p.offset_from (cut[last_p_param as usize].beg) as c_int;
		last_p_param += 1;
		p = p.offset (3);
		cut[last_p_param as usize].beg = p;
	      }
	    else				/* not continuous: bail */
	      { remove_p_params = false; }
	    if last_p_param > 10		/* too many: bail */
	      { remove_p_params = false; }
	  }
	p = p.offset (1);
      }
    if remove_p_params && last_p_param != 0
      {
	let mut i: c_int;
	let mut wp: *mut c_char;

	cut[last_p_param as usize].len = r.offset_from (cut[last_p_param as usize].beg) as c_int;
	/* C's `for (i = 0, wp = ret; i <= last_p_param; wp += cut[i++].len)`: the body runs with
	   the current `i`, then `wp` advances by `cut[i].len` and `i` becomes `i + 1`.  */
	i = 0;
	wp = ret;
	while i <= last_p_param
	  {
	    memcpy (wp as *mut c_void, cut[i as usize].beg as *const c_void,
		    cut[i as usize].len as usize);
	    wp = wp.offset (cut[i as usize].len as isize);
	    i += 1;
	  }
	r = wp;
      }
  }

  *r = 0;
  /* Update *AREA.  */
  if !area.is_null ()
    { *area = r.offset (1); }
  return ret;
}

/* Outputting a string with padding.  */

/* C 266: `char PC;` - the pad character termcap's globals expose (tparam.h 43 declares it, and
   this definition shadows that declaration's import, P-21/P-39).  C's tentative definition is
   zero.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub static mut PC: c_char = 0;

/// C: `void tputs (const char *str, int nlines, int (*outfun) (int))` - tparam.h 31's
/// declaration, whose third parameter is a C function pointer (`Option<unsafe extern "C" fn>` in
/// the mirror).  C's parameter name is `str`; `str` is Rust's primitive type name, so the port
/// spells it `str_` (PORTING.md 0.2's renaming rule).
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tputs (str_: *const c_char, nlines: c_int,
				outfun: Option<unsafe extern "C" fn (c_int) -> c_int>)
{
  let mut padcount: c_int = 0;
  let mut str_: *const c_char = str_;

  if str_.is_null ()
    { return; }

  while *str_ as c_int >= b'0' as c_int && *str_ as c_int <= b'9' as c_int
    {
      padcount += *str_ as c_int - b'0' as c_int;
      str_ = str_.offset (1);
      padcount *= 10;
    }
  if *str_ == b'.' as c_char
    {
      str_ = str_.offset (1);
      padcount += *str_ as c_int - b'0' as c_int;
      str_ = str_.offset (1);
    }
  if *str_ == b'*' as c_char
    {
      str_ = str_.offset (1);
      padcount *= nlines;
    }
  while *str_ != 0
    {
      outfun.unwrap () (*str_ as c_int);
      str_ = str_.offset (1);
    }

  /* PADCOUNT is now in units of tenths of msec.
     BAUD_RATE is measured in characters per 10 seconds.
     Compute PADFACTOR = 100000 * (how many padding bytes are needed).  */
  let mut padfactor: intmax_t;
  /* C 298's `ckd_mul (&padfactor, padcount, baud_rate)`: gnulib's macro multiplies in the
     operands' common type and stores the wrapped product, reporting overflow (P-15, the
     `ckd_mul`/`overflowing_mul` family of `docs/ports/token-aliases.tsv`); `baud_rate` is the
     `EMACS_INT` object `globals.rs` declares (`dispnew.rs` owns it) whose type is the same
     `intmax_t` here.  */
  let (wrapped, overflowed) = (padcount as intmax_t).overflowing_mul (baud_rate as intmax_t);
  padfactor = wrapped;
  if overflowed
    { padfactor = if baud_rate < 0 { INTMAX_MIN } else { INTMAX_MAX }; }

  while 50000 <= padfactor
    {
      outfun.unwrap () (PC as c_int);
      padfactor -= 100000;
    }
}

/* Finding the termcap entry in the termcap data base.  */

#[cfg (feature = "msdos")]
#[repr(C)]
pub struct termcap_buffer
{
  pub beg: *mut c_char,
  pub size: ptrdiff_t,
  pub ptr: *mut c_char,
  pub ateof: bool,
  pub full: ptrdiff_t,
}

/* C 316-321's four static prototypes are dropped with C 64's (see the note there).  */

/* C 323-331's `valid_filename_p`, whose `#ifdef MSDOS` at C 326 is the *same* condition as the
   whole file's: the MS-DOS build is the only configuration that compiles this file, and the same
   sed file defines `MSDOS` (`emacs/msdos/sed2v2.inp` 26-28) and `INTERNAL_TERMINAL` (91).  The
   arm C 326-327 spells is therefore the file gate's own text below.  C 328-330's `#else` cannot be
   reached by any configuration that compiles this file at all, so its text is quoted rather than
   carried as a second gated item - that item would either gate on
   `all (feature = "msdos", not (feature = "msdos"))`, which nothing satisfies, or compile in the
   frozen configuration and put a symbol into the object that must stay empty (P-86 ⑧;
   Deviations 6).  */
/// C: (C 328-330, the `#else` arm) `return *fn == '/';`
#[cfg (feature = "msdos")]
unsafe fn valid_filename_p (fn_: *mut c_char) -> bool
{ *fn_ == b'/' as c_char || *fn_.offset (1) == b':' as c_char }

/* C 359-373's arm reads `INTERNAL_TERMINAL`, a configure substitution that no header of the
   reference tree declares (`configure.ac` 6454 only gives it its `AH_TEMPLATE` text: "This is
   substituted when $TERM is \"internal\".").  Its one value in the tree is the MS-DOS build's own
   (`emacs/msdos/sed2v2.inp` 91 replaces the generated `#undef INTERNAL_TERMINAL` with the entry
   below), so the declaration stands in this unit (P-88 ①) with that measurement, under the
   feature that names C's macro; its home in a generated mirror is owed (Owed 3).  */
/// C: `emacs/msdos/sed2v2.inp` 91: `#define INTERNAL_TERMINAL "pc|bios|IBM PC with color
/// display::co#80:li#25:Co#16:pa#256:km:ms:cm=<CM>:cl=<CL>:ce=<CE>::se=</SO>:so=<SO>:us=<UL>:ue=</UL>:md=<BD>:mh=<DIM>:mb=<BL>:mr=<RV>:me=<NV>::AB=<BG %d>:AF=<FG %d>:op=<DefC>:"`
#[cfg (all (feature = "msdos", feature = "internal_terminal"))]
pub const INTERNAL_TERMINAL: *const c_char =
  c"pc|bios|IBM PC with color display::co#80:li#25:Co#16:pa#256:km:ms:cm=<CM>:cl=<CL>:ce=<CE>::se=</SO>:so=<SO>:us=<UL>:ue=</UL>:md=<BD>:mh=<DIM>:mb=<BL>:mr=<RV>:me=<NV>::AB=<BG %d>:AF=<FG %d>:op=<DefC>:".as_ptr ();

/* Find the termcap entry data for terminal type NAME
   and store it in the block that BP points to.
   Record its address for future use.

   If BP is null, space is dynamically allocated.

   Return -1 if there is some difficulty accessing the data base
   of terminal types,
   0 if the data base is accessible but the type NAME is not defined
   in it, and some other value otherwise.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tgetent (mut bp: *mut c_char, name: *const c_char) -> c_int
{
  let mut termcap_name: *mut c_char;
  let mut fd: c_int;
  /* C's `struct termcap_buffer buf;` is assigned field by field at C 425-427; a Rust local has to
     start whole (PORTING.md 0.1-13), so it starts at C's zero value.  */
  let mut buf: termcap_buffer = std::mem::zeroed ();
  let mut bp1: *mut c_char;
  let mut tc_search_point: *mut c_char;
  let mut term: *mut c_char;
  let mut malloc_size: ptrdiff_t = 0;
  let mut c: c_int;
  let mut tcenv: *mut c_char = std::ptr::null_mut ();	/* TERMCAP value, if it contains :tc=.  */
  let mut indirect: *mut c_char = std::ptr::null_mut ();/* Terminal type in :tc= in TERMCAP value.  */
  let mut filep: bool;

  /* C's forward `goto ret` (C 371 in the INTERNAL_TERMINAL arm and C 407 in the indirect-value
     arm) becomes the labelled block below, and the label's own statements C 493-495 follow it
     (PORTING.md 2.2: a forward `goto` to a shared tail is `'block: { … break 'block; }`, never an
     early `return` that would skip the tail).  */
  'ret: {
    /* C 359-373: `#ifdef INTERNAL_TERMINAL`; the frozen configuration leaves the macro
       undefined, so this arm is dead text here and the `#else` side is C's own absence of it
       (P-86 ①).  INTERNAL_TERMINAL is a configure substitution whose only value in the reference
       tree is the MS-DOS build's own (`emacs/msdos/sed2v2.inp` 91 writes the `pc|bios|…` entry;
       `configure.ac` 6454 states what it is for), so the in-unit declaration below carries that
       measurement and the name is owed to the `config.rs` generator (Owed 3).  */
    #[cfg (feature = "internal_terminal")]
    {
      if strcmp (name, c"internal".as_ptr ()) == 0
	{
	  /* C's `term = INTERNAL_TERMINAL;` assigns a `const char *` (its macro's type) to
	     `char *term`; the port spells the implicit conversion (Deviation 10).  */
	  term = INTERNAL_TERMINAL as *mut c_char;
	  if bp.is_null ()
	    {
	      malloc_size = (1 + strlen (term)) as ptrdiff_t;
	      bp = xmalloc (malloc_size as size_t) as *mut c_char;
	    }
	  strcpy (bp, term);
	  break 'ret;
	}
    }

    /* For compatibility with programs like `less' that want to
       put data in the termcap buffer themselves as a fallback.  */
    if !bp.is_null ()
      { term_entry = bp; }

    termcap_name = getenv (c"TERMCAP".as_ptr ());
    if !termcap_name.is_null () && *termcap_name == 0
      { termcap_name = std::ptr::null_mut (); }
    /* C 383-388: `#if defined (MSDOS) && !defined (TEST)`; the frozen configuration leaves TEST
       undefined too, so the arm is live under the file gate and the `#else` side is C's absence
       of the three lines (P-86 ①).  */
    #[cfg (not (feature = "test"))]
    if !termcap_name.is_null ()
	&& (*termcap_name == b'\\' as c_char
	    || *termcap_name == b'/' as c_char
	    || *termcap_name.offset (1) == b':' as c_char)
      { dostounix_filename (termcap_name); }

    filep = !termcap_name.is_null () && valid_filename_p (termcap_name);

    /* If termcap_name is non-null and starts with / (in the un*x case, that is),
       it is a file name to use instead of /etc/termcap.
       If it is non-null and does not start with /,
       it is the entry itself, but only if
       the name the caller requested matches the TERM variable.  */

    if !termcap_name.is_null () && !filep
	&& strcmp (name, getenv (c"TERM".as_ptr ())) == 0
      {
	indirect = tgetst1 (find_capability (termcap_name, c"tc".as_ptr ()),
			    std::ptr::null_mut ());
	if indirect.is_null ()
	  {
	    if bp.is_null ()
	      { bp = termcap_name; }
	    else
	      { strcpy (bp, termcap_name); }
	    break 'ret;
	  }
	else
	  {			/* It has tc=.  Need to read /etc/termcap.  */
	    tcenv = termcap_name;
	    termcap_name = std::ptr::null_mut ();
	  }
      }

    if termcap_name.is_null () || !filep
      /* C 417: `termcap_name = (char *) TERMCAP_FILE;` - the `&CStr` constant's pointer.  */
      { termcap_name = TERMCAP_FILE.as_ptr () as *mut c_char; }

    /* Here we know we must search a file and termcap_name has its name.  */

    fd = emacs_open (termcap_name, O_RDONLY | O_TEXT, 0);
    if fd < 0
      { return -1; }

    /* C 425's `buf.size = BUFSIZE;` - under the `DEBUG` arm that name is C 45's `bufsize` read
       (see the note at C 43-51).  */
    #[cfg (feature = "debug")]
      { buf.size = bufsize as ptrdiff_t; }
    #[cfg (not (feature = "debug"))]
      { buf.size = BUFSIZE as ptrdiff_t; }
    /* Add 1 to size to ensure room for terminating null.  */
    buf.beg = xmalloc ((buf.size + 1) as size_t) as *mut c_char;
    term = if !indirect.is_null () { indirect } else { name as *mut c_char };

    if bp.is_null ()
      {
	malloc_size = if !indirect.is_null () { (strlen (tcenv) + 1) as ptrdiff_t } else { buf.size };
	bp = xmalloc (malloc_size as size_t) as *mut c_char;
      }
    bp1 = bp;
    tc_search_point = bp1;

    if !indirect.is_null ()
      /* Copy the data from the environment variable.  */
      {
	strcpy (bp, tcenv);
	bp1 = bp1.offset (strlen (tcenv) as isize);
      }

    while !term.is_null ()
      {
	/* Scan the file, reading it via buf, till find start of main entry.  */
	if !scan_file (term, fd, &raw mut buf)
	  {
	    emacs_close (fd);
	    xfree (buf.beg as *mut c_void);
	    if malloc_size != 0
	      { xfree (bp as *mut c_void); }
	    return 0;
	  }

	/* Free old `term' if appropriate.  */
	if term != name as *mut c_char
	  { xfree (term as *mut c_void); }

	/* If BP is malloc'd by us, make sure it is big enough.  */
	if malloc_size != 0
	  {
	    let offset1: ptrdiff_t = bp1.offset_from (bp);
	    let offset2: ptrdiff_t = tc_search_point.offset_from (bp);
	    malloc_size = offset1 + buf.size;
	    let p: *mut c_char = xrealloc (bp as *mut c_void, malloc_size as size_t) as *mut c_char;
	    bp = p;
	    termcap_name = p;
	    bp1 = termcap_name.offset (offset1);
	    tc_search_point = termcap_name.offset (offset2);
	  }

	/* Copy the line of the entry from buf into bp.  */
	termcap_name = buf.ptr;
	while { *bp1 = *termcap_name as c_char; c = *bp1 as c_int;
		bp1 = bp1.offset (1); termcap_name = termcap_name.offset (1);
		c != 0 && c != b'\n' as c_int }
	  {
	    /* Drop out any \ newline sequence.  */
	    if c == b'\\' as c_int && *termcap_name == b'\n' as c_char
	      {
		bp1 = bp1.offset (-1);
		termcap_name = termcap_name.offset (1);
	      }
	  }
	*bp1 = 0;

	/* Does this entry refer to another terminal type's entry?
	   If something is found, copy it into heap and null-terminate it.  */
	tc_search_point = find_capability (tc_search_point, c"tc".as_ptr ());
	term = tgetst1 (tc_search_point, std::ptr::null_mut ());
      }

    emacs_close (fd);
    xfree (buf.beg as *mut c_void);

    if malloc_size != 0
      { bp = xrealloc (bp as *mut c_void, (bp1.offset_from (bp) + 1) as size_t) as *mut c_char; }
  }

  term_entry = bp;
  return 1;
}

/* Given file open on FD and buffer BUFP,
   scan the file from the beginning until a line is found
   that starts the entry for terminal type STR.
   Return 1 if successful, with that line in BUFP,
   or 0 if no entry is found in the file.  */

#[cfg (feature = "msdos")]
unsafe fn scan_file (str_: *mut c_char, fd: c_int, bufp: *mut termcap_buffer) -> bool
{
  let mut end: *mut c_char;

  (*bufp).ptr = (*bufp).beg;
  (*bufp).full = 0;
  (*bufp).ateof = false;
  *(*bufp).ptr = 0;

  lseek (fd, 0, 0);

  while !(*bufp).ateof
    {
      /* Read a line into the buffer.  */
      end = std::ptr::null_mut ();
      loop
	{
	  /* if it is continued, append another line to it,
	     until a non-continued line ends.  */
	  end = gobble_line (fd, bufp, end);
	  /* C's `do { … } while (!bufp->ateof && end[-2] == '\\');` - the body runs at least once
	     and the test comes after it.  */
	  if !(!(*bufp).ateof && *end.offset (-2) == b'\\' as c_char)
	    { break; }
	}

      if *(*bufp).ptr != b'#' as c_char
	  && name_match ((*bufp).ptr, str_)
	{ return true; }

      /* Discard the line just processed.  */
      (*bufp).ptr = end;
    }
  return false;
}

/* Return true if NAME is one of the names specified
   by termcap entry LINE.  */

#[cfg (feature = "msdos")]
unsafe fn name_match (line: *mut c_char, name: *mut c_char) -> bool
{
  let mut tem: *mut c_char;

  if !compare_contin (line, name)
    { return true; }
  /* This line starts an entry.  Is it the right one?  */
  tem = line;
  while *tem != 0 && *tem != b'\n' as c_char && *tem != b':' as c_char
    {
      if *tem == b'|' as c_char && !compare_contin (tem.offset (1), name)
	{ return true; }
      tem = tem.offset (1);
    }

  return false;
}

#[cfg (feature = "msdos")]
unsafe fn compare_contin (str1: *mut c_char, str2: *mut c_char) -> bool
{
  let mut str1: *mut c_char = str1;
  let mut str2: *mut c_char = str2;

  loop
    {
      let mut c1: c_int = *str1 as c_int;
      let c2: c_int = *str2 as c_int;
      str1 = str1.offset (1);
      str2 = str2.offset (1);
      while c1 == b'\\' as c_int && *str1 == b'\n' as c_char
	{
	  str1 = str1.offset (1);
	  while { c1 = *str1 as c_int; str1 = str1.offset (1);
		  c1 == b' ' as c_int || c1 == b'\t' as c_int }
	    { continue; }
	}
      if c2 == 0
	{
	  /* End of type being looked up.  */
	  if c1 == b'|' as c_int || c1 == b':' as c_int
	    /* If end of name in data base, we win.  */
	    { return false; }
	  else
	    { return true; }
	}
      else if c1 != c2
	{ return true; }
    }
}

/* Make sure that the buffer <- BUFP contains a full line
   of the file open on FD, starting at the place BUFP->ptr
   points to.  Can read more of the file, discard stuff before
   BUFP->ptr, or make the buffer bigger.

   Return the pointer to after the newline ending the line,
   or to the end of the file, if there is no newline to end it.

   Can also merge on continuation lines.  If APPEND_END is
   non-null, it points past the newline of a line that is
   continued; we add another line onto it and regard the whole
   thing as one line.  The caller decides when a line is continued.  */

#[cfg (feature = "msdos")]
unsafe fn gobble_line (fd: c_int, bufp: *mut termcap_buffer, append_end: *mut c_char)
  -> *mut c_char
{
  let mut end: *mut c_char;
  let mut nread: c_int;
  let mut buf: *mut c_char = (*bufp).beg;
  let mut append_end: *mut c_char = append_end;

  if append_end.is_null ()
    { append_end = (*bufp).ptr; }

  loop
    {
      end = append_end;
      while *end != 0 && *end != b'\n' as c_char
	{ end = end.offset (1); }
      if *end != 0
	{ break; }
      if (*bufp).ateof
	{ return buf.offset ((*bufp).full); }
      if (*bufp).ptr == buf
	{
	  if (*bufp).full == (*bufp).size
	    {
	      let ptr_offset: ptrdiff_t = (*bufp).ptr.offset_from (buf);
	      let append_end_offset: ptrdiff_t = append_end.offset_from (buf);
	      /* Add 1 to size to ensure room for terminating null.  */
	      let mut size: ptrdiff_t = (*bufp).size + 1;
	      let p: *mut c_char = xpalloc (buf as *mut c_void, &raw mut size, 1, -1, 1)
				     as *mut c_char;
	      (*bufp).beg = p;
	      buf = p;
	      (*bufp).size = size - 1;
	      (*bufp).ptr = buf.offset (ptr_offset);
	      append_end = buf.offset (append_end_offset);
	    }
	}
      else
	{
	  append_end = append_end.offset (-((*bufp).ptr.offset_from (buf)));
	  /* C's `memcpy (buf, bufp->ptr, bufp->full -= bufp->ptr - buf)`: the third argument *is*
	     the assignment, so `full` is updated by `ptr - buf` and the new value is the count.  */
	  (*bufp).full -= (*bufp).ptr.offset_from (buf);
	  memcpy (buf as *mut c_void, (*bufp).ptr as *const c_void, (*bufp).full as usize);
	  (*bufp).ptr = buf;
	}
      /* C's `if (!(nread = read (…))) bufp->ateof = 1;` - the assignment is the condition, and
	 `read`'s `ssize_t` goes into C's `int` (the port keeps C's narrowing).  */
      nread = read (fd, buf.offset ((*bufp).full) as *mut c_void,
		    ((*bufp).size - (*bufp).full) as usize) as c_int;
      if nread == 0
	{ (*bufp).ateof = true; }
      (*bufp).full += nread as ptrdiff_t;
      *buf.offset ((*bufp).full) = 0;
    }
  return end.offset (1);
}
