// femacs: port of lib/strtol.c @ fd7afced403.
//
// gnulib compiles this source only through an inclusion shim: `lib/strtoll.c` is `#define
// QUAD 1` + `#include <strtol.c>`, and the frozen build compiles no `strtol.o` at all
// (`build/emacs-ref/lib` carries neither object).  PORTING.md 10.13② asks the shared body to
// land first as a prelude `macro_rules!` and each shim to instantiate it, so the body below is
// `macro_rules! strtol_body` and `femacs/lib/strtoll.rs` is its only instantiation.  This
// file's own `--check` compares C's standalone reading (QUAD and UNSIGNED undefined,
// non-wide, non-locale); `mod standalone` records the names that reading leaves defined and
// that the body takes as parameters.
//
// The frozen configuration's `#if` evaluator resolves all 36 of C's conditions
// (`tools/check-cond-eval.py lib/strtol.c` -> 36 conditions, 0 unknown, 0 disagreements), so
// the port carries the live text only: `_LIBC`, `USE_NUMBER_GROUPING`,
// `USE_IN_EXTENDED_LOCALE_MODEL`, `USE_WIDE_CHAR` and `UNSIGNED` are all undefined.  The arms
// that follow from that (`../locale/localeinfo.h` and `grouping.h`, the `wchar.h`/`wctype.h`
// wide-character body, the `_NL_CURRENT` extended-locale model, C 367's `#if UNSIGNED` return
// and C 398-410's `#ifdef USE_NUMBER_GROUPING` external entry point) are dead text; each is
// left as a one-line note where C's `#if` stood, and `docs/ports/strtol.md` records them (the
// strftime.c treatment).  The line-count basis is C's live lines
// (`docs/ports/line-drift-tolerance.tsv`).
//
// There is no `#![allow (…)]` here: an inner attribute inside an included file is E0753, and
// `femacs/lib/strtoll.rs` - the unit that compiles the body - carries the list, so this
// file's own `build-tu` compiles with warnings, not errors.

use std::ffi::{c_char, c_int, c_long, c_longlong, c_uchar, c_uint, c_ulong, c_ulonglong};
include!("../src/include/config.rs");
include!("include/limits.in.rs");
include!("include/ctype.rs");
include!("include/errno.rs");
include!("include/libc-config.rs");   // C 30-32: `__set_errno`

/* C 22-32: `_LIBC` is undefined, so C 25's `# include <config.h>` is the include above and C
   28-37's <ctype.h>/<errno.h>/<limits.h>/<stddef.h>/<stdlib.h>/<string.h> are the
   ctype/errno/limits mirrors (`<stddef.h>`'s NULL is `core::ptr::null`, and <stdlib.h> and
   <string.h> have no live user here).  C 30-32's `#ifndef __set_errno` is live, and its
   text is libc-config.h 46's macro - the `libc-config.rs` mirror above carries it now, so
   this unit no longer spells a second copy (P-26, one landing point).  */

/* C 43-50: `UNSIGNED` is undefined, so C 46's `UNSIGNED 0` and C 47's `INT LONG int` are the
   live arm; `UNSIGNED` is the ledger's constant (`INT` is one of the `mod standalone`
   records below).  */
pub const UNSIGNED: c_int = 0;

/* C 52-115's name selection: `USE_IN_EXTENDED_LOCALE_MODEL` and `QUAD` are undefined for this
   file, so C 108-112 leaves `strtol` unrenamed; the shim's `QUAD` arm makes the entry's name
   `strtoll`, which is the body's `$name`.  */

/* C 117-135: `QUAD` is undefined, so C 130-135's `#else` arm is this file's reading; the body
   takes its four bindings as parameters, and `mod standalone` records the names.  */

/* C 136-142: the grouping parameter is empty.  */
macro_rules! GROUP_PARAM_PROTO { () => { } }

/* C 144-157: the extended-locale parameter is empty.  */
macro_rules! LOCALE_PARAM { () => { } }
macro_rules! LOCALE_PARAM_PROTO { () => { } }

/* C 159-195: `USE_WIDE_CHAR` and `USE_IN_EXTENDED_LOCALE_MODEL` are undefined, so the
   non-wide arm is live: C 175's `L_(Ch)` is `Ch` (the port's cast spells Rust's `b'x'` as the
   `char` C's `'x'` promotes to), C 176-177's `UCHAR_TYPE`/`STRING_TYPE` are
   `unsigned char`/`char`, and the classification and upcase macros call the libc functions
   with C's `(unsigned char)` cast.  The `macro_rules!` records are what the ledger's macro
   check compares; the type aliases are what the body's signatures read (the strftime.rs
   CHAR_T shape).  */
macro_rules! L_ { ($ch:expr) => { $ch as c_char } }
pub type UCHAR_TYPE = c_uchar;
macro_rules! UCHAR_TYPE { () => { c_uchar } }
pub type STRING_TYPE = c_char;
macro_rules! STRING_TYPE { () => { c_char } }
macro_rules! ISSPACE { ($ch:expr) => { isspace (($ch) as c_uchar as c_int) } }
macro_rules! ISALPHA { ($ch:expr) => { isalpha (($ch) as c_uchar as c_int) } }
macro_rules! TOUPPER { ($ch:expr) => { toupper (($ch) as c_uchar as c_int) } }

/* C 190-195: `USE_NUMBER_GROUPING` is undefined, so `INTERNAL` is the identity.  */
macro_rules! INTERNAL { ($x:ident) => { $x } }

/* C 47/131-135's standalone reading.  The body below takes these five bindings as parameters
   (the shim supplies the `QUAD` arm), so the names exist only here.  */
mod standalone {
  macro_rules! INT { () => { LONG int } }
  macro_rules! LONG { () => { long } }
  macro_rules! STRTOL_LONG_MIN { () => { LONG_MIN } }
  macro_rules! STRTOL_LONG_MAX { () => { LONG_MAX } }
  macro_rules! STRTOL_ULONG_MAX { () => { ULONG_MAX } }
}

/* C 197-410 as the shared `macro_rules!` PORTING.md 10.13② requires (the ftoastr.c /
   strftime.c shape).  `$name` is C's `INTERNAL (strtol)` name (the shim's `QUAD` makes it
   `strtoll`), `$export` the symbol the including unit emits, `$long`/`$ulong` C's `LONG` /
   `unsigned LONG int`, and the last three are C's `STRTOL_LONG_MIN`/`STRTOL_LONG_MAX`/
   `STRTOL_ULONG_MAX`.  C's dead arms (the header note) are omitted; every live statement
   keeps C's order.  */
macro_rules! strtol_body {
  ($name:ident, $export:literal, $long:ty, $ulong:ty, $strtol_long_min:expr,
   $strtol_long_max:expr, $strtol_ulong_max:expr) => {
    /* C 204-213.  `register` is a storage hint with no Rust counterpart (dropped), and
       `GROUP_PARAM_PROTO`/`LOCALE_PARAM_PROTO` are the empty macros above.  */
    #[unsafe(export_name = $export)]
    pub unsafe extern "C" fn $name (nptr: *const STRING_TYPE, endptr: *mut *mut STRING_TYPE,
                                    mut base: c_int) -> $long
    {
      /* C 246-250.  */
      if base < 0 || base == 1 || base > 36
        {
          __set_errno! (EINVAL);
          return 0;
        }

      /* C 252-253.  */
      let mut s: *const STRING_TYPE = nptr;
      let mut save: *const STRING_TYPE = s;

      'noconv: {
      /* Skip white space.  */
      while ISSPACE! (*s) != 0
        { s = s.add (1); }
      if *s == 0
        { break 'noconv; }

      /* Check for a sign.  C's `int negative;` is zero-initialized (PORTING.md 1.5).  */
      let mut negative: c_int = 0;
      if *s == L_! (b'-')
        {
          negative = 1;
          s = s.add (1);
        }
      else if *s == L_! (b'+')
        {
          negative = 0;
          s = s.add (1);
        }
      else
        { negative = 0; }

      /* Recognize number prefix and if BASE is zero, figure it out ourselves.  */
      if *s == L_! (b'0')
        {
          if (base == 0 || base == 16) && TOUPPER! (*s.offset (1)) == b'X' as c_int
            {
              s = s.offset (2);
              base = 16;
            }
          else if (base == 0 || base == 2) && TOUPPER! (*s.offset (1)) == b'B' as c_int
            {
              s = s.offset (2);
              base = 2;
            }
          else if base == 0
            { base = 8; }
        }
      else if base == 0
        { base = 10; }

      /* Save the pointer so we can check later if anything happened.  */
      save = s;

      /* C 298-316: `USE_NUMBER_GROUPING` is undefined, so `end` stays NULL.  */
      let end: *const STRING_TYPE = std::ptr::null ();

      let mut cutoff: $ulong = $strtol_ulong_max / (base as $ulong);
      let mut cutlim: c_uint = ($strtol_ulong_max % (base as $ulong)) as c_uint;

      let mut overflow: c_int = 0;
      let mut i: $ulong = 0;
      let mut c: UCHAR_TYPE = *s as UCHAR_TYPE;
      while c != 0
        {
          if s == end
            { break; }
          if c >= b'0' && c <= b'9'
            { c -= b'0'; }
          else if ISALPHA! (c) != 0
            { c = (TOUPPER! (c) - b'A' as c_int + 10) as UCHAR_TYPE; }
          else
            { break; }
          if (c as c_int) >= base
            { break; }
          /* Check for overflow.  */
          if i > cutoff || (i == cutoff && (c as c_uint) > cutlim)
            { overflow = 1; }
          else
            {
              i = i.wrapping_mul (base as $ulong);
              i = i.wrapping_add (c as $ulong);
            }
          s = s.add (1);
          c = *s as UCHAR_TYPE;
        }

      /* Check if anything actually happened.  */
      if s == save
        { break 'noconv; }

      /* Store in ENDPTR the address of one character
         past the last character we converted.  */
      if ! endptr.is_null ()
        { *endptr = s as *mut STRING_TYPE; }

      /* C 354-362's live `#if !UNSIGNED` arm (C 367-368's `#if UNSIGNED` return is dead).  */
      if overflow == 0
        {
          let bound: $ulong = if negative != 0
            { (($strtol_long_min + 1) as $ulong).wrapping_neg () + 1 }
            else { $strtol_long_max as $ulong };
          if i > bound
            { overflow = 1; }
        }

      if overflow != 0
        {
          __set_errno! (ERANGE);
          return if negative != 0 { $strtol_long_min } else { $strtol_long_max };
        }

      /* Return the result of the appropriate sign.  */
      return if negative != 0 { i.wrapping_neg () as $long } else { i as $long };
      }

      /* C 377-395's `noconv:` label.  */
      if ! endptr.is_null ()
        {
          if save.offset_from (nptr) >= 2
              && (TOUPPER! (*save.offset (-1)) == b'X' as c_int
                  || TOUPPER! (*save.offset (-1)) == b'B' as c_int)
              && *save.offset (-2) == b'0' as c_char
            { *endptr = save.offset (-1) as *mut STRING_TYPE; }
          else
            { *endptr = nptr as *mut STRING_TYPE; }
        }

      return 0;
    }
  };
}
