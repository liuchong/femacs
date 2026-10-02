/* Copyright (C) 1991-2026 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* emacs/lib/strftime.c: the frozen configuration resolves every one of C's 115
   conditions (`tools/check-cond-eval.py lib/strftime.c` reports 0 unknown, 0
   disagreements), so this port is the live text only: `FPRINTFTIME` 0,
   `USE_C_LOCALE` 0, `HAVE_ONLY_C_LOCALE` 0, `MULTIBYTE_IS_FORMAT_SAFE` 1,
   `HAVE_NATIVE_TIME_Z` 0, `_LIBC` undefined, `HAVE_STRUCT_TM_TM_ZONE/_GMTOFF` 1.
   The arms that follow from those values (`should_remove_ampm`, `tm_diff`,
   `c_locale`, `utc_timezone`, the locale-data tables, the `COMPILE_WIDE` and
   `FPRINTFTIME` and `_NL_CURRENT` code) are dead text in the reference; each is
   left as a one-line note where C's `#if` stood, and this ledger records them.

   C 19-27's `FPRINTFTIME`/`USE_C_LOCALE` defaults are constants below (C's
   `#ifndef` arms are live: the configuration defines neither).  C 29-43's
   `#ifdef _LIBC` arm is dead, so the live includes are `libc-config.h`,
   `"strftime.h"` and `"time-internal.h"`; the system headers C names map to the
   port's mirrors (string/ctype/stdint/limits/stdlib/locale/errno/time), plus
   `intprops.h`'s `INT_STRLEN_BOUND`, which has no generic mirror (the md5
   `INT_BUFSIZE_BOUND` decision).

   `my_strftime` is C 896-916's rename: in this configuration `#define my_strftime
   strftime`, so the unit's definition *is* the symbol `strftime` (emitted through
   `#[unsafe(export_name)]`, the mempcpy/`__memrchr` precedent, PORTING.md 10.13)
   while C's `#define nl_get_alt_digit _nl_get_alt_digit` has no live user.  */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/stdint.in.rs");
include!("include/errno.rs");
include!("include/string.rs");
include!("include/ctype.rs");
include!("include/stdcountof.rs");
include!("include/time.in.rs");
include!("include/time-internal.rs");
include!("include/strftime.rs");

/* C 20 / 26 / 50: the three `#ifndef`/`#if defined __ANDROID__` defaults.  */
pub const FPRINTFTIME: c_int = 0;
pub const USE_C_LOCALE: c_int = 0;
pub const HAVE_ONLY_C_LOCALE: c_int = 0;

/* C 101-102.  */
pub const MULTIBYTE_IS_FORMAT_SAFE: c_int = 1;
pub const DO_MULTIBYTE: c_int = (MULTIBYTE_IS_FORMAT_SAFE == 0) as c_int;

/* C 138-145: the live `#else` arm of `#ifdef COMPILE_WIDE`.  The type names are
   aliases and the operation macros carry C's bodies; the port's aliases exist so
   the signatures read like C's, and the `macro_rules!` items are what the ledger's
   macro check compares (PORTING.md 0.1-5: one home per macro).  */
pub type CHAR_T = c_char;
pub type UCHAR_T = c_uchar;
macro_rules! CHAR_T { () => { c_char } }
macro_rules! UCHAR_T { () => { c_uchar } }
macro_rules! L_ { ($str:expr) => { $str } }
/* C's `L_('x')` is a character constant (an `int`) and `L_("…")` a string literal;
   Rust's `'x'` is a `char`, so the port spells the character uses with `L_C` (whose
   name keeps C's token prefix and whose value is C's `int`) and the strings with `L_`
   (the ledger records the split).  */
macro_rules! L_C { ($c:expr) => { $c as c_char } }
macro_rules! NLW { ($sym:ident) => { $sym } }
macro_rules! ABALTMON_1 { () => { _NL_ABALTMON_1 } }
macro_rules! MEMCPY { ($d:expr, $s:expr, $n:expr) => { memcpy ($d, $s, $n) } }
macro_rules! STRLEN { ($s:expr) => { strlen ($s) } }

/* C 149-162: the portable right shift (its `-1 >> 1 == -1` test is a constant
   expression here, so the live arm is C's `(a) >> (b)`; the macro keeps C's
   whole body so the ledger's token check sees both arms).  */
macro_rules! SHR {
  ($a:expr, $b:expr) => {
    if -1i32 >> 1 == -1 { ($a) >> ($b) }
    else { (($a) + (($a) < 0)) / (1 << ($b)) - (($a) < 0) }
  };
}

/* C 164-171.  */
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum pad_style
{
  ZERO_PAD,         /* (default) Pad with 0 unless format says otherwise.  */
  ALWAYS_ZERO_PAD,  /* '0'       Always pad with 0.  */
  SIGN_PAD,         /* '+'       Always output a sign.  */
  SPACE_PAD,        /* '_'       Pad with space.  */
  NO_PAD            /* '-'       Do not pad.  */
}
use pad_style::*;

/* C 173 and 865-867.  */
pub const TM_YEAR_BASE: c_int = 1900;
pub const ISO_WEEK_START_WDAY: c_int = 1; /* Monday */
pub const ISO_WEEK1_WDAY: c_int = 4; /* Thursday */
pub const YDAY_MINIMUM: c_int = -366;

/* C 175-180.  */
macro_rules! __isleap {
  ($year:expr) => {
    ((($year) % 4 == 0 && (($year) % 100 != 0 || ($year) % 400 == 0)) as c_int)
  };
}

/* C 190-192: `CAL_ARGS(x, y)` is empty in this configuration (the
   non-Gregorian-calendar arm that makes it `x, y,` is dead), so the port's
   counterpart carries C's body for the ledger and the signatures below simply omit
   the two parameters C would have added.  */
macro_rules! CAL_ARGS { ($x:expr, $y:expr) => { () } }

/* C 203-218: the two type macros and the two width types of the live arm.  */
pub type byte_count_t = usize;
pub type sbyte_count_t = isize;
macro_rules! STREAM_OR_CHAR_T { () => { CHAR_T } }
macro_rules! STRFTIME_ARG { ($x:expr) => { $x } }
pub type retval_t = sbyte_count_t;

/* C 220-230.  */
pub const FAILURE: retval_t = -1;

/* C 252-255: the live `#else` arm (the FPRINTFTIME and COMPILE_WIDE arms are
   dead).  Each is C's `(memset (P, ' ', Len), (P) += (Len))`.  */
macro_rules! memset_space {
  ($p:ident, $len:expr) => {
    {
      memset ($p as *mut c_void, b' ' as c_int, $len);
      $p = $p.add ($len);
    }
  };
}
macro_rules! memset_zero {
  ($p:ident, $len:expr) => {
    {
      memset ($p as *mut c_void, b'0' as c_int, $len);
      $p = $p.add ($len);
    }
  };
}

/* C 259-261: `advance (P, N)` is `((P) += (N))`.  */
macro_rules! advance { ($p:ident, $n:expr) => { $p = $p.add ($n) }; }

/* C 263: `add (n, f)` is `width_add (width, n, f)`; `width` is the caller's
   local, which a Rust macro cannot name (hygiene), so the port passes it (and the
   other caller state `width_add` needs) as the leading arguments - the same class
   of deviation the md5 `OP` and sha `R` macros record.  */
macro_rules! add {
  ($p:ident, $i:ident, $maxsize:expr, $pad:ident, $width:expr, $n:expr, $f:expr) => {
    width_add! ($p, $i, $maxsize, $pad, $width, $n, $f)
  };
}

/* C 265-272: `incr_overflow (incr)` is
   `(maxsize - i <= (incr) || (i += (incr), false))`.  */
macro_rules! incr_overflow {
  ($i:ident, $maxsize:expr, $incr:expr) => {
    (($maxsize).wrapping_sub ($i as usize) <= ($incr) as usize
     || { $i += $incr as sbyte_count_t; false })
  };
}

/* C 274-298.  C's `do { … } while (0)` is a block; C's `errno = ERANGE` is the
   errno mirror's assignment; C's `return FAILURE` is the port's `return FAILURE`.  */
macro_rules! width_add {
  ($p:ident, $i:ident, $maxsize:expr, $pad:ident, $width:expr, $n:expr, $f:expr) => {
    {
      let _n: byte_count_t = $n;
      let _w: byte_count_t = if $pad == NO_PAD || $width < 0 { 0 } else { $width as byte_count_t };
      let _incr: byte_count_t = if _n < _w { _w } else { _n };
      if incr_overflow! ($i, $maxsize, _incr)
        {
          *errno () = ERANGE;
          return FAILURE;
        }
      if ! $p.is_null ()
        {
          if _n < _w
            {
              let _delta: byte_count_t = _w - _n;
              if $pad == ALWAYS_ZERO_PAD || $pad == SIGN_PAD
                { memset_zero! ($p, _delta); }
              else
                { memset_space! ($p, _delta); }
            }
          $f;
          advance! ($p, _n);
        }
    }
  };
}

/* C 300-305: `add1 (c)` is `width_add1 (width, c)` with
   `width_add1 (width, c) = width_add (width, 1, *p = c)`.  */
macro_rules! add1 {
  ($p:ident, $i:ident, $maxsize:expr, $pad:ident, $width:expr, $c:expr) => {
    width_add1! ($p, $i, $maxsize, $pad, $width, $c)
  };
}
macro_rules! width_add1 {
  ($p:ident, $i:ident, $maxsize:expr, $pad:ident, $width:expr, $c:expr) => {
    width_add! ($p, $i, $maxsize, $pad, $width, 1, *$p = $c)
  };
}

/* C 307-334: `cpy (n, s)` is `width_cpy (width, n, s)`; the live arm's body is the
   `if (to_lowcase) … else if (to_uppcase) … else MEMCPY (…)` statement, so
   `to_lowcase`/`to_uppcase` are passed as well.  */
macro_rules! cpy {
  ($p:ident, $i:ident, $maxsize:expr, $pad:ident, $width:expr, $low:ident, $up:ident,
   $n:expr, $s:expr) => {
    width_cpy! ($p, $i, $maxsize, $pad, $width, $low, $up, $n, $s)
  };
}
macro_rules! width_cpy {
  ($p:ident, $i:ident, $maxsize:expr, $pad:ident, $width:expr, $low:ident, $up:ident,
   $n:expr, $s:expr) => {
    width_add! ($p, $i, $maxsize, $pad, $width, $n,
                if $low { memcpy_lowcase ($p, ($s) as *const CHAR_T, $n); }
                else if $up { memcpy_uppcase ($p, ($s) as *const CHAR_T, $n); }
                else { MEMCPY! ($p as *mut c_void, ($s) as *const c_void, $n); })
  };
}

/* C 355-388: the live arms of the locale plumbing.  `LOCALE_PARAM`,
   `LOCALE_ARG` and `HELPER_LOCALE_ARG` are empty here, which is why the port's
   functions take no locale parameter; `TOLOWER`/`TOUPPER` are C's `tolower (Ch)` /
   `toupper (Ch)` (their second argument is unused in this configuration).  */
macro_rules! LOCALE_PARAM { () => { () } }
macro_rules! LOCALE_ARG { () => { () } }
macro_rules! HELPER_LOCALE_ARG { () => { () } }
macro_rules! TOUPPER { ($ch:expr, $l:expr) => { toupper ($ch) } }
macro_rules! TOLOWER { ($ch:expr, $l:expr) => { tolower ($ch) } }

/* C 389-393.  */
macro_rules! ISDIGIT {
  ($ch:expr) => { (($ch) as c_uint).wrapping_sub (L_C! (b'0') as c_uint) <= 9 };
}

/* C 395-401: C's `#pragma GCC diagnostic` (a compiler directive with no Rust
   counterpart) is the `WIDEST_BUF` note below where it applies; nothing to write.  */

/* C 402-425: the two case-folding copies.  C's forward declarations (C 403-404,
   419-420) have no Rust counterpart (§0.1-12), so only the definitions stand, at
   C's definition site.  `while (len-- > 0)` is kept as a `loop` whose decrement
   happens before the test, and the two conversions C performs implicitly in
   `dest[len] = TOLOWER ((UCHAR_T) src[len], loc)` are written out.  */
unsafe fn memcpy_lowcase (dest: *mut CHAR_T, src: *const CHAR_T, mut len: usize)
  -> *mut CHAR_T
{
  loop
    {
      let _len: usize = len;
      len = len.wrapping_sub (1);
      if ! (_len > 0)
        { break; }
      *dest.add (len) = TOLOWER! (((*src.add (len)) as UCHAR_T) as c_int, ()) as CHAR_T;
    }
  return dest;
}

unsafe fn memcpy_uppcase (dest: *mut CHAR_T, src: *const CHAR_T, mut len: usize)
  -> *mut CHAR_T
{
  loop
    {
      let _len: usize = len;
      len = len.wrapping_sub (1);
      if ! (_len > 0)
        { break; }
      *dest.add (len) = TOUPPER! (((*src.add (len)) as UCHAR_T) as c_int, ()) as CHAR_T;
    }
  return dest;
}

/* C 428-435: `HAVE_NATIVE_TIME_Z` is C's ternary over the four configuration
   macros; the port spells the same expression so the ledger's token check sees
   C's names, with the frozen values alongside.  */
pub const HAVE_STRFTIME_L: c_int = 0;
pub const HAVE_STRFTIME_LZ: c_int = 0;
pub const HAVE_STRFTIME_Z: c_int = 0;
pub const HAVE_NATIVE_TIME_Z: c_int =
  if USE_C_LOCALE != 0 && HAVE_STRFTIME_L != 0 { HAVE_STRFTIME_LZ } else { HAVE_STRFTIME_Z };

/* C 861-877.  */
unsafe fn iso_week_days (yday: c_int, wday: c_int) -> c_int
{
  /* Add enough to the first operand of % to make it nonnegative.  */
  let big_enough_multiple_of_7: c_int = (-YDAY_MINIMUM / 7 + 2) * 7;
  return yday
         - (yday - wday + ISO_WEEK1_WDAY + big_enough_multiple_of_7) % 7
         + ISO_WEEK1_WDAY - ISO_WEEK_START_WDAY;
}

/* C 896-916: `my_strftime`'s rename and the `extra_args`/`tz`/`ns` spellings of
   this configuration.  */
macro_rules! my_strftime { () => { strftime } }
macro_rules! nl_get_alt_digit { () => { _nl_get_alt_digit } }
macro_rules! extra_args { () => { () } }
macro_rules! extra_args_spec { () => { () } }
/* C 914-915's `# define tz 1` / `# define ns 0`: the `my_strftime`-defined arm that
   makes them parameters belongs to nstrftime.c's view of the file, and C's own
   standalone text does not compile (`clang -fsyntax-only` reports that the macro
   rewrites `underlying_strftime`'s parameter name), so the port keeps them as the
   constants C's `#else` arm defines, declared where C's macro applies.  */

/* C 918-925: the forward declaration is a Rust item order thing (§0.1-12: the
   declaration is dropped and the definition stands where C defines it).  */

/* C 927-936's `#undef strftime`: the underlying implementation is the time.h
   mirror's `strftime` (the system's).  */

/* C 938-1077.  */
unsafe fn underlying_strftime (mut tz: timezone_t, ubuf: *mut c_char, ubufsize: usize,
                               modifier: c_char, format_char: c_char, tp: *const tm) -> usize
{
  /* The relevant information is available only via the
     underlying strftime implementation, so use that.  */
  let mut ufmt: [c_char; 5] = [0; 5];
  let mut u: *mut c_char = ufmt.as_mut_ptr ();

  /* The space helps distinguish strftime failure from empty
     output.  */
  *u = b' ' as c_char;
  u = u.add (1);
  *u = b'%' as c_char;
  u = u.add (1);
  *u = modifier;
  u = u.add ((modifier != 0) as usize);
  *u = format_char;
  u = u.add (1);
  *u = b'\0' as c_char;

  /* C 966-977's `#if HAVE_NATIVE_TIME_Z` arm is dead; this is the live
     `#if !HAVE_NATIVE_TIME_Z` one.  */
  if ! tz.is_null () && tz != local_tz! ()
    {
      tz = set_tz (tz);
      if tz.is_null ()
        { return 0; }
    }

  let len: usize;
  /* C 986-995: the two dead arms select `strftime_l`/`strftime_z`; the live one is
     the system `strftime`.  */
  len = strftime (ubuf, ubufsize, ufmt.as_ptr (), tp);

  /* C 997-1000's `#if !HAVE_NATIVE_TIME_Z` arm.  */
  if ! tz.is_null () && ! revert_tz (tz)
    { return 0; }

  if len != 0
    {
      /* C 1003-1073: the whole body is dead for this target (its condition names
         glibc < 2.31, NetBSD and Solaris).  */
    }
  return len;
}

/* C 1079-1124.  */
unsafe fn get_tm_zone (tz: timezone_t, ubuf: *mut c_char, ubufsize: c_int, modifier: c_int,
                       tp: *const tm) -> *const c_char
{
  /* C 1088's `#if HAVE_STRUCT_TM_TM_ZONE` arm is live (the frozen `struct tm` has
     `tm_zone`); the `#else` arm (which formats through the underlying strftime)
     is dead.  */
  /* The POSIX test suite assumes that setting
     the environment variable TZ to a new value before calling strftime()
     will influence the result (the %Z format) even if the information in
     *TP is computed with a totally different time zone.
     This is bogus: though POSIX allows bad behavior like this,
     POSIX does not require it.  Do the right thing instead.  */
  let ret: *const c_char = (*tp).tm_zone;
  /* C 1096-1099's `# if defined __ANDROID__` arm is dead.  */
  return ret;
}

/* C 1126-1203.  */
#[unsafe(export_name = "strftime")]
pub unsafe extern "C" fn my_strftime (s: *mut c_char, maxsize: usize, format: *const c_char,
                                      tp: *const tm) -> retval_t
{
  /* C 1143-1196's `#if !USE_C_LOCALE` arms are dead (this is a plain `strftime`).  */
  let mut tzset_called: bool = false;
  return __strftime_internal (s, maxsize, format, tp, false, ZERO_PAD, -1,
                              &raw mut tzset_called);
  /* C 1203's `libc_hidden_def (my_strftime)` is a glibc attribute the gnulib build
     defines to nothing.  */
}

/* C 1205-1218 and the `goto` targets of the format switch: the port keeps the shared
   tail's statements once and enters it at a *stage* whose name is C's label (§2.2's
   entry-stage shape, the tparam.c decision).  `stage::bad_percent` is the one label
   that adjusts `f` before the shared `bad_format` body.  */
mod stage
{
  pub const none: u8 = 0;
  pub const subformat: u8 = 1;
  pub const subformat_width: u8 = 2;
  pub const bad_format: u8 = 3;
  pub const underlying_strftime: u8 = 4;
  pub const do_z_conversion: u8 = 5;
  pub const do_tz_offset: u8 = 6;
  pub const do_yearish: u8 = 7;
  pub const do_number_spacepad: u8 = 8;
  pub const do_number: u8 = 9;
  pub const do_signed_number: u8 = 10;
  pub const do_maybe_signed_number: u8 = 11;
  pub const do_number_body: u8 = 12;
  pub const do_number_sign_and_padding: u8 = 13;
  pub const bad_percent: u8 = 14;
}

/* C 1466-1507: the six `DO_*` macros.  Each is C's `do { … goto label; } while (0)`;
   the port records the entry stage instead of jumping (the notes row carries C's
   `goto`/label tokens).  */
macro_rules! DO_NUMBER {
  ($stage:ident, $digits:ident, $number_value:ident, $d:expr, $v:expr) => {
    { $digits = $d; $number_value = $v; $stage = stage::do_number; }
  };
}
macro_rules! DO_SIGNED_NUMBER {
  ($stage:ident, $digits:ident, $negative_number:ident, $u_number_value:ident, $d:expr,
   $negative:expr, $v:expr) => {
    DO_MAYBE_SIGNED_NUMBER! ($stage, $digits, $negative_number, $u_number_value, $d,
                             $negative, $v, stage::do_signed_number)
  };
}
macro_rules! DO_YEARISH {
  ($stage:ident, $digits:ident, $negative_number:ident, $u_number_value:ident, $d:expr,
   $negative:expr, $v:expr) => {
    DO_MAYBE_SIGNED_NUMBER! ($stage, $digits, $negative_number, $u_number_value, $d,
                             $negative, $v, stage::do_yearish)
  };
}
macro_rules! DO_MAYBE_SIGNED_NUMBER {
  ($stage:ident, $digits:ident, $negative_number:ident, $u_number_value:ident, $d:expr,
   $negative:expr, $v:expr, $label:expr) => {
    {
      $digits = $d;
      $negative_number = $negative;
      $u_number_value = $v;
      $stage = $label;
    }
  };
}
/* The mask is not what you might think.
   When the ordinal i'th bit is set, insert a colon
   before the i'th digit of the time zone representation.  */
macro_rules! DO_TZ_OFFSET {
  ($stage:ident, $digits:ident, $tz_colon_mask:ident, $u_number_value:ident, $d:expr,
   $mask:expr, $v:expr) => {
    {
      $digits = $d;
      $tz_colon_mask = $mask;
      $u_number_value = $v;
      $stage = stage::do_tz_offset;
    }
  };
}
macro_rules! DO_NUMBER_SPACEPAD {
  ($stage:ident, $digits:ident, $number_value:ident, $d:expr, $v:expr) => {
    { $digits = $d; $number_value = $v; $stage = stage::do_number_spacepad; }
  };
}

/* C 1209-2446.  */
unsafe fn __strftime_internal (s: *mut CHAR_T, mut maxsize: usize, format: *const CHAR_T,
                               tp: *const tm, upcase: bool, yr_spec: pad_style,
                               mut width: sbyte_count_t, tzset_called: *mut bool) -> retval_t
{
  /* C 1220-1222's `_LIBC`/`USE_IN_EXTENDED_LOCALE_MODEL` arm (and its `saved_errno`)
     is dead.  */
  /* C 1223-1228's `#if FAILURE == 0` arm is dead; this is the live clamp.  */
  if (PTRDIFF_MAX as usize) < maxsize
    { maxsize = PTRDIFF_MAX as usize; }

  /* C 1230-1281's `a_wkday`-family and `aw_len`-family definitions are dead: their
     arm needs `_NL_CURRENT` or a C-locale build, and this configuration's conversions
     go through `underlying_strftime` instead.  */
  let mut i: retval_t = 0;
  let mut p: *mut CHAR_T = s;

  let mut hour12: c_int = (*tp).tm_hour;
  if hour12 > 12
    { hour12 -= 12; }
  else
    if hour12 == 0
      { hour12 = 12; }

  let mut f: *const CHAR_T = format;
  while *f != 0
    {
      /* C's `continue` inside the loop body reaches the `for`'s increment
         (`width = -1, f++`), which the port's `'iteration` block reproduces.  */
      'iteration: {
      let mut pad: pad_style = ZERO_PAD;
      let mut modifier: c_int;             /* Field modifier ('E', 'O', or 0).  */
      let mut digits: c_int = 0;           /* Max digits for numeric format.  */
      let mut number_value: c_int = 0;     /* Numeric value to be printed.  */
      let mut u_number_value: c_uint = 0;  /* (unsigned int) number_value.  */
      let mut negative_number: bool = false; /* The number is negative.  */
      let mut always_output_a_sign: bool = false; /* +/- should always be output.  */
      let mut tz_colon_mask: c_int = 0;    /* Bitmask of where ':' should appear.  */
      let mut subfmt: *const CHAR_T = std::ptr::null ();
      let mut bufp: *mut CHAR_T = std::ptr::null_mut ();
      /* C's `CHAR_T buf[1 + 2 + (sizeof (int) < sizeof (time_t) ? INT_STRLEN_BOUND
         (time_t) : INT_STRLEN_BOUND (int))]`; `sizeof (int) < sizeof (time_t)` holds
         for this target, and `INT_STRLEN_BOUND` is gnulib's intprops.h macro
         `INT_BITS_STRLEN_BOUND (TYPE_WIDTH (t) - signed) + signed` (the md5
         `INT_BUFSIZE_BOUND` decision: the one instantiation is carried here).  */
      const INT_STRLEN_BOUND: usize = ((size_of::<time_t> () * 8 - 1) * 146 + 484) / 485 + 1;
      let mut buf: [CHAR_T; 1 + 2 + INT_STRLEN_BOUND] = [0; 1 + 2 + INT_STRLEN_BOUND];
      let mut to_lowcase: bool = false;
      let mut to_uppcase: bool = upcase;
      let mut colons: usize = 0;
      let mut change_case: bool = false;
      let mut format_char: c_int;
      let mut subwidth: sbyte_count_t = 0;

      let mut stage: u8 = stage::none;

      /* C 914-915.  */
      const tz: c_int = 1;
      const ns: c_int = 0;

      /* C 1284-1407: the `DO_MULTIBYTE` arms are dead (`MULTIBYTE_IS_FORMAT_SAFE`
         is 1), so this is C's `#else` arm.  */
      if *f != L_C! (b'%')
        {
          add1! (p, i, maxsize, pad, width, *f);
          break 'iteration;
        }

      let percent: *const CHAR_T = f;

      /* Check for flags that can modify a format.  */
      loop
        {
          f = f.add (1);
          if *f == L_C! (b'_')
            { pad = SPACE_PAD; }
          else if *f == L_C! (b'-')
            { pad = NO_PAD; }
          else if *f == L_C! (b'+')
            { pad = SIGN_PAD; }
          else if *f == L_C! (b'0')
            { pad = ALWAYS_ZERO_PAD; }
          /* This changes textual output.  */
          else if *f == L_C! (b'^')
            { to_uppcase = true; }
          else if *f == L_C! (b'#')
            { change_case = true; }
          else
            { break; }
        }

      if ISDIGIT! (*f)
        {
          width = 0;
          loop
            {
              /* C's `ckd_mul (&width, width, 10) || ckd_add (&width, width, *f -
                 L_('0'))` (PORTING.md 2.6/P-15: the wrapped value is stored and the
                 overflow reported; on overflow C returns without storing).  */
              let (v, overflowed) = width.overflowing_mul (10);
              if overflowed
                { return FAILURE; }
              width = v;
              let (v, overflowed) = width.overflowing_add ((*f - L_C! (b'0')) as sbyte_count_t);
              if overflowed
                { return FAILURE; }
              width = v;
              f = f.add (1);
              if ! ISDIGIT! (*f)
                { break; }
            }
        }

      /* Check for modifiers.  */
      if *f == L_C! (b'E') || *f == L_C! (b'O')
        {
          modifier = *f as c_int;
          f = f.add (1);
        }
      else
        { modifier = 0; }

      /* Now do the specified format.  */
      format_char = *f as c_int;
      /* C's `switch (format_char)` (the tparam.c `switch`→chain decision).  */
      if format_char as u8 == b'%'
        {
          if f != percent.add (1)
            { stage = stage::bad_percent; }
          else
            { add1! (p, i, maxsize, pad, width, *f); }
        }
      else if format_char as u8 == b'a' as u8
        {
          if modifier != 0
            { stage = stage::bad_format; }
          else
            {
              if change_case
                {
                  to_uppcase = true;
                  to_lowcase = false;
                }
              /* C 1523's locale-data arm is dead: C's `#else` is the live one.  */
              stage = stage::underlying_strftime;
            }
        }
      else if format_char as u8 == b'A' as u8
        {
          if modifier != 0
            { stage = stage::bad_format; }
          else
            {
              if change_case
                {
                  to_uppcase = true;
                  to_lowcase = false;
                }
              stage = stage::underlying_strftime;
            }
        }
      else if format_char as u8 == b'b' as u8 || format_char as u8 == b'h' as u8
        {
          if change_case
            {
              to_uppcase = true;
              to_lowcase = false;
            }
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { stage = stage::underlying_strftime; }
        }
      else if format_char as u8 == b'B' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            {
              if change_case
                {
                  to_uppcase = true;
                  to_lowcase = false;
                }
              stage = stage::underlying_strftime;
            }
        }
      else if format_char as u8 == b'c' as u8
        {
          if modifier == L_C! (b'O') as c_int
            { stage = stage::bad_format; }
          else
            { stage = stage::underlying_strftime; }
        }
      else if format_char as u8 == b'C' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::underlying_strftime; }
          else
            {
              let negative_year: bool = (*tp).tm_year < - TM_YEAR_BASE;
              let zero_thru_1899: bool = !negative_year & ((*tp).tm_year < 0);
              let century: c_int = (((*tp).tm_year - 99 * zero_thru_1899 as c_int) / 100
                                    + TM_YEAR_BASE / 100);
              DO_YEARISH! (stage, digits, negative_number, u_number_value, 2,
                           negative_year, century as c_uint);
            }
        }
      else if format_char as u8 == b'x' as u8
        {
          if modifier == L_C! (b'O') as c_int
            { stage = stage::bad_format; }
          else
            { stage = stage::underlying_strftime; }
        }
      else if format_char as u8 == b'D' as u8
        {
          if modifier != 0
            { stage = stage::bad_format; }
          else
            {
              subfmt = L_!("%m/%d/%y").as_ptr () as *const CHAR_T;
              stage = stage::subformat;
            }
        }
      else if format_char as u8 == b'd' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER! (stage, digits, number_value, 2, (*tp).tm_mday); }
        }
      else if format_char as u8 == b'e' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER_SPACEPAD! (stage, digits, number_value, 2, (*tp).tm_mday); }
        }
      else if format_char as u8 == b'F' as u8
        {
          if modifier != 0
            { stage = stage::bad_format; }
          else
            {
              if pad == ZERO_PAD && width < 0
                {
                  pad = SIGN_PAD;
                  subwidth = 4;
                }
              else
                {
                  subwidth = width - 6;
                  if subwidth < 0
                    { subwidth = 0; }
                }
              subfmt = L_!("%Y-%m-%d").as_ptr () as *const CHAR_T;
              stage = stage::subformat_width;
            }
        }
      else if format_char as u8 == b'H' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER! (stage, digits, number_value, 2, (*tp).tm_hour); }
        }
      else if format_char as u8 == b'I' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER! (stage, digits, number_value, 2, hour12); }
        }
      else if format_char as u8 == b'k' as u8           /* GNU extension.  */
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER_SPACEPAD! (stage, digits, number_value, 2, (*tp).tm_hour); }
        }
      else if format_char as u8 == b'l' as u8           /* GNU extension.  */
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER_SPACEPAD! (stage, digits, number_value, 2, hour12); }
        }
      else if format_char as u8 == b'j' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_SIGNED_NUMBER! (stage, digits, negative_number, u_number_value, 3,
                                 (*tp).tm_yday < -1,
                                 (((*tp).tm_yday as c_uint).wrapping_add (1))); }
        }
      else if format_char as u8 == b'M' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER! (stage, digits, number_value, 2, (*tp).tm_min); }
        }
      else if format_char as u8 == b'm' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_SIGNED_NUMBER! (stage, digits, negative_number, u_number_value, 2,
                                 (*tp).tm_mon < -1, (((*tp).tm_mon as c_uint).wrapping_add (1))); }
        }
      else if format_char as u8 == b'N' as u8           /* GNU extension.  */
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            {
              let mut n: c_int = ns;
              let ns_digits: c_int = 9;
              if width <= 0
                { width = ns_digits as sbyte_count_t; }
              let mut ndigs: c_int = ns_digits;
              while width < ndigs as sbyte_count_t || (1 < ndigs && n % 10 == 0)
                { ndigs -= 1; n /= 10; }
              let mut j: c_int = ndigs;
              while 0 < j
                {
                  buf[(j - 1) as usize] = (n % 10 + L_C! (b'0') as c_int) as CHAR_T;
                  n /= 10;
                  j -= 1;
                }
              if pad == ZERO_PAD
                { pad = ALWAYS_ZERO_PAD; }
              width_cpy! (p, i, maxsize, pad, 0, to_lowcase, to_uppcase,
                          ndigs as byte_count_t, buf.as_ptr ());
              width_add! (p, i, maxsize, pad, width - ndigs as sbyte_count_t, 0, ());
            }
        }
      else if format_char as u8 == b'n' as u8
        {
          add1! (p, i, maxsize, pad, width, L_C! (b'\n'));
        }
      else if format_char as u8 == b'P' as u8 || format_char as u8 == b'p' as u8
        {
          /* C's `case L_('P')` sets TO_LOWCASE and (without _NL_CURRENT) rewrites
             FORMAT_CHAR to 'p', then falls through into `case L_('p')`.  */
          if format_char as u8 == b'P' as u8
            {
              to_lowcase = true;
              format_char = L_C! (b'p') as c_int;
            }
          if change_case
            {
              to_uppcase = false;
              to_lowcase = true;
            }
          stage = stage::underlying_strftime;
        }
      else if format_char as u8 == b'q' as u8           /* GNU extension.  */
        {
          DO_SIGNED_NUMBER! (stage, digits, negative_number, u_number_value, 1, false,
                             ((((*tp).tm_mon * 11) >> 5) + 1) as c_uint);
        }
      else if format_char as u8 == b'R' as u8
        {
          subfmt = L_!("%H:%M").as_ptr () as *const CHAR_T;
          stage = stage::subformat;
        }
      else if format_char as u8 == b'r' as u8
        {
          /* C 2079-2085's macOS/FreeBSD/Windows arm is the live one.  */
          subfmt = L_!("%I:%M:%S %p").as_ptr () as *const CHAR_T;
          stage = stage::subformat;
        }
      else if format_char as u8 == b'S' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER! (stage, digits, number_value, 2, (*tp).tm_sec); }
        }
      else if format_char as u8 == b's' as u8           /* GNU extension.  */
        {
          let mut ltm: tm = *tp;
          ltm.tm_yday = -1;
          let mut t: time_t = mktime_z (tz as timezone_t, &raw mut ltm);
          if ltm.tm_yday < 0
            {
              *errno () = EOVERFLOW;
              return FAILURE;
            }

          /* Generate string value for T using time_t arithmetic;
             this works even if sizeof (long) < sizeof (time_t).  */

          bufp = buf.as_mut_ptr ().add (countof! (buf));
          negative_number = t < 0;

          loop
            {
              let d: c_int = (t % 10) as c_int;
              t /= 10;
              bufp = bufp.sub (1);
              *bufp = ((if negative_number { -d } else { d }) + L_C! (b'0') as c_int) as CHAR_T;
              if t == 0
                { break; }
            }

          digits = 1;
          always_output_a_sign = false;
          stage = stage::do_number_sign_and_padding;
        }
      else if format_char as u8 == b'X' as u8
        {
          if modifier == L_C! (b'O') as c_int
            { stage = stage::bad_format; }
          else
            { stage = stage::underlying_strftime; }
        }
      else if format_char as u8 == b'T' as u8
        {
          subfmt = L_!("%H:%M:%S").as_ptr () as *const CHAR_T;
          stage = stage::subformat;
        }
      else if format_char as u8 == b't' as u8
        {
          add1! (p, i, maxsize, pad, width, L_C! (b'\t'));
        }
      else if format_char as u8 == b'u' as u8
        {
          DO_NUMBER! (stage, digits, number_value, 1,
                      (((*tp).tm_wday - 1 + 7) % 7 + 1));
        }
      else if format_char as u8 == b'U' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER! (stage, digits, number_value, 2,
                          (((*tp).tm_yday - (*tp).tm_wday + 7) / 7)); }
        }
      else if format_char as u8 == b'V' as u8 || format_char as u8 == b'g' as u8
              || format_char as u8 == b'G' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            {
              /* YEAR is a leap year if and only if (tp->tm_year + TM_YEAR_BASE)
                 is a leap year, except that YEAR and YEAR - 1 both work
                 correctly even when (tp->tm_year + TM_YEAR_BASE) would
                 overflow.  */
              let year: c_int = (*tp).tm_year
                                + (if (*tp).tm_year < 0 { TM_YEAR_BASE % 400 }
                                   else { TM_YEAR_BASE % 400 - 400 });
              let mut year_adjust: c_int = 0;
              let mut days: c_int = iso_week_days ((*tp).tm_yday, (*tp).tm_wday);

              if days < 0
                {
                  /* This ISO week belongs to the previous year.  */
                  year_adjust = -1;
                  days = iso_week_days ((*tp).tm_yday + (365 + __isleap! (year - 1)),
                                        (*tp).tm_wday);
                }
              else
                {
                  let d: c_int = iso_week_days ((*tp).tm_yday - (365 + __isleap! (year)),
                                                (*tp).tm_wday);
                  if 0 <= d
                    {
                      /* This ISO week belongs to the next year.  */
                      year_adjust = 1;
                      days = d;
                    }
                }

              if *f == L_C! (b'g')
                {
                  let yy: c_int = ((*tp).tm_year % 100 + year_adjust) % 100;
                  DO_YEARISH! (stage, digits, negative_number, u_number_value, 2, false,
                               (if 0 <= yy { yy }
                                else if (*tp).tm_year < -TM_YEAR_BASE - year_adjust { -yy }
                                else { yy + 100 }) as c_uint);
                }
              else if *f == L_C! (b'G')
                {
                  DO_YEARISH! (stage, digits, negative_number, u_number_value, 4,
                               (*tp).tm_year < -TM_YEAR_BASE - year_adjust,
                               ((*tp).tm_year as c_uint).wrapping_add (TM_YEAR_BASE as c_uint)
                               .wrapping_add (year_adjust as c_uint));
                }
              else
                {
                  DO_NUMBER! (stage, digits, number_value, 2, days / 7 + 1);
                }
            }
        }
      else if format_char as u8 == b'W' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER! (stage, digits, number_value, 2,
                          (((*tp).tm_yday - ((*tp).tm_wday - 1 + 7) % 7 + 7) / 7)); }
        }
      else if format_char as u8 == b'w' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::bad_format; }
          else
            { DO_NUMBER! (stage, digits, number_value, 1, (*tp).tm_wday); }
        }
      else if format_char as u8 == b'Y' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::underlying_strftime; }
          else
            { DO_YEARISH! (stage, digits, negative_number, u_number_value, 4,
                           (*tp).tm_year < -TM_YEAR_BASE,
                           ((*tp).tm_year + TM_YEAR_BASE) as c_uint); }
        }
      else if format_char as u8 == b'y' as u8
        {
          if modifier == L_C! (b'E') as c_int
            { stage = stage::underlying_strftime; }
          else
            {
              let mut yy: c_int = (*tp).tm_year % 100;
              if yy < 0
                { yy = if (*tp).tm_year < - TM_YEAR_BASE { -yy } else { yy + 100 }; }
              DO_YEARISH! (stage, digits, negative_number, u_number_value, 2, false,
                           yy as c_uint);
            }
        }
      else if format_char as u8 == b'Z' as u8
        {
          if change_case
            {
              to_uppcase = false;
              to_lowcase = true;
            }

          /* C 2291-2345: the `_LIBC` and `COMPILE_WIDE` arms are dead; this is the
             live `get_tm_zone` arm.  */
          let mut zonebuf: [c_char; 128] = [0; 128]; /* Enough for any time zone abbreviation.  */
          let zone: *const c_char =
            get_tm_zone (tz as timezone_t, zonebuf.as_mut_ptr (), 128, modifier, tp);
          cpy! (p, i, maxsize, pad, width, to_lowcase, to_uppcase, STRLEN! (zone), zone);
        }
      else if format_char as u8 == b':' as u8
        {
          /* :, ::, and ::: are valid only just before 'z'.
             :::: etc. are rejected later.  */
          colons = 1;
          while *f.add (colons) == L_C! (b':')
            { colons += 1; }
          if *f.add (colons) != L_C! (b'z')
            { stage = stage::bad_format; }
          else
            {
              f = f.add (colons);
              stage = stage::do_z_conversion;
            }
        }
      else if format_char as u8 == b'z' as u8
        {
          colons = 0;
          stage = stage::do_z_conversion;
        }
      /* C's `case L_('\0')` (the format ends inside a directive) falls into
         `bad_percent`; the `default:` label falls into `bad_format`.  */
      else if format_char as u8 == b'\0' as u8
        {
          stage = stage::bad_percent;
        }
      else
        {
          stage = stage::bad_format;
        }

      /* ---- the shared tail: C's labels, in C's order (§2.2).  C's labels are
         jump targets, so a stage's block must follow every block that can set it;
         `do_z_conversion` comes first because its `default: goto bad_format` is a
         *backward* jump in C and the port's cascade only moves forward.  ---- */
      if stage == stage::do_z_conversion
        {
          /* C 2358-2420's `do_z_conversion:` block.  */
          if (*tp).tm_isdst < 0
            { /* C's `break;` out of the format switch.  */ }
          else
            {
              let mut diff: c_int;
              /* C 2367's `#if HAVE_STRUCT_TM_TM_GMTOFF` arm is live.  */
              diff = (*tp).tm_gmtoff as c_int;

              negative_number = diff < 0;
              if diff == 0
                {
                  let mut zonebuf: [c_char; 128] = [0; 128]; /* Enough for any time zone abbreviation.  */
                  negative_number = *get_tm_zone (tz as timezone_t, zonebuf.as_mut_ptr (),
                                                  128, 0, tp) == b'-' as c_char;
                }
              let hour_diff: c_int = diff / 60 / 60;
              let min_diff: c_int = diff / 60 % 60;
              let sec_diff: c_int = diff % 60;

              /* C's inner `switch (colons)` with its two labels (`tz_hh_mm`,
                 `tz_hh_mm_ss`) is the same entry-stage shape.  */
              if colons == 0 /* +hhmm */
                {
                  DO_TZ_OFFSET! (stage, digits, tz_colon_mask, u_number_value, 5, 0,
                                 (hour_diff * 100 + min_diff) as c_uint);
                }
              else if colons == 1 || colons == 2 || colons == 3
                {
                  let mut zstage: c_int = colons as c_int;
                  if colons == 3
                    {
                      if sec_diff != 0
                        { zstage = 2; }
                      else if min_diff != 0
                        { zstage = 1; }
                    }
                  if zstage == 1 /* +hh:mm */
                    {
                      DO_TZ_OFFSET! (stage, digits, tz_colon_mask, u_number_value, 6,
                                     0o4, (hour_diff * 100 + min_diff) as c_uint);
                    }
                  else if zstage == 2 /* +hh:mm:ss */
                    {
                      DO_TZ_OFFSET! (stage, digits, tz_colon_mask, u_number_value, 9,
                                     0o24,
                                     (hour_diff * 10000 + min_diff * 100 + sec_diff) as c_uint);
                    }
                  else /* +hh if possible */
                    {
                      DO_TZ_OFFSET! (stage, digits, tz_colon_mask, u_number_value, 3, 0,
                                     hour_diff as c_uint);
                    }
                }
              else
                { stage = stage::bad_format; }
            }
        }
      if stage == stage::bad_percent
        {
          /* C's `bad_percent:` (entered by `case L_('\0')` and by the `%` case's
             `goto bad_percent`) runs `--f` before the shared `bad_format` body.  */
          f = f.sub (1);
        }
      if stage == stage::bad_format || stage == stage::bad_percent
        {
          cpy! (p, i, maxsize, pad, width, to_lowcase, to_uppcase,
                (f.offset_from (percent) + 1) as byte_count_t, percent);
        }
      if stage == stage::underlying_strftime
        {
          let mut ubuf: [c_char; 1024] = [0; 1024]; /* enough for any single format in practice */
          let len: usize = underlying_strftime (tz as timezone_t, ubuf.as_mut_ptr (), 1024,
                                                modifier as c_char, format_char as c_char, tp);
          if len != 0
            {
              /* C 1678-1746's arms are dead (glibc/NetBSD/Solaris and
                 `REQUIRE_GNUISH_STRFTIME_AM_PM`); the live text is C's `#else`.  */
              cpy! (p, i, maxsize, pad, width, to_lowcase, to_uppcase, len - 1,
                    ubuf.as_ptr ().add (1));
            }
        }
      if stage == stage::subformat
        { subwidth = -1; }
      if stage == stage::subformat || stage == stage::subformat_width
        {
          /* Restrict flag propagation to %F and %Y so that we get glibc
             consistent behavior for %D with glibc specific '_' and '-' flags.
             Note the year is at the start of %F so OK to propagate there.  */
          let subpad: pad_style =
            (if format_char as u8 == b'F' as u8 || format_char as u8 == b'Y' as u8
             { pad } else { yr_spec });

          let len: retval_t =
            __strftime_internal (std::ptr::null_mut (), (usize::MAX), subfmt, tp,
                                 to_uppcase, subpad, subwidth, tzset_called);
          if FAILURE < 0 && len < 0
            { return FAILURE; } /* errno is set here */
          add! (p, i, maxsize, pad, width, len as byte_count_t,
                __strftime_internal (p, maxsize.wrapping_sub (i as usize), subfmt, tp,
                                     to_uppcase, subpad, subwidth, tzset_called));
        }
      if stage == stage::do_tz_offset
        {
          always_output_a_sign = true;
          stage = stage::do_number_body;
        }
      if stage == stage::do_yearish
        {
          if pad == ZERO_PAD
            { pad = yr_spec; }
          always_output_a_sign
            = (pad == SIGN_PAD
               && ((if digits == 2 { 99 } else { 9999 }) < u_number_value
                   || (digits as sbyte_count_t) < width));
          stage = stage::do_maybe_signed_number;
        }
      if stage == stage::do_number_spacepad
        {
          if pad == ZERO_PAD
            { pad = SPACE_PAD; }
          stage = stage::do_number;
        }
      if stage == stage::do_number
        {
          /* Format NUMBER_VALUE according to the MODIFIER flag.  */
          negative_number = number_value < 0;
          u_number_value = number_value as c_uint;
          stage = stage::do_signed_number;
        }
      if stage == stage::do_signed_number
        {
          always_output_a_sign = false;
          stage = stage::do_maybe_signed_number;
        }
      if stage == stage::do_maybe_signed_number
        {
          tz_colon_mask = 0;
          stage = stage::do_number_body;
        }
      if stage == stage::do_number_body
        {
          /* Format U_NUMBER_VALUE according to the MODIFIER flag.
             NEGATIVE_NUMBER is nonzero if the original number was
             negative; in this case it was converted directly to
             unsigned int (i.e., modulo (UINT_MAX + 1)) without
             negating it.  */
          if modifier == L_C! (b'O') as c_int && !negative_number
            {
              /* C 1871-1890's non-Gregorian-calendar arm is dead (SUPPORT_NON_GREG_
                 CALENDARS_IN_STRFTIME is false).  */
            }

          bufp = buf.as_mut_ptr ().add (countof! (buf));

          if negative_number
            { u_number_value = u_number_value.wrapping_neg (); }

          loop
            {
              if tz_colon_mask & 1 != 0
                {
                  bufp = bufp.sub (1);
                  *bufp = b':' as CHAR_T;
                }
              tz_colon_mask >>= 1;

              /* C 1904-1908's `#if SUPPORT_NON_GREG_CALENDARS_IN_STRFTIME` arm is
                 dead; this is C's `#else`.  */
              bufp = bufp.sub (1);
              *bufp = (u_number_value % 10 + L_C! (b'0') as c_uint) as CHAR_T;

              u_number_value /= 10;
              if u_number_value == 0 && tz_colon_mask == 0
                { break; }
            }
          stage = stage::do_number_sign_and_padding;
        }
      if stage == stage::do_number_sign_and_padding
        {
          if pad == ZERO_PAD
            { pad = ALWAYS_ZERO_PAD; }
          if width < 0
            { width = digits as sbyte_count_t; }

          {
            let sign_char: CHAR_T = (if negative_number { L_C! (b'-') }
                                     else if always_output_a_sign { L_C! (b'+') }
                                     else { 0 });
            let number_bytes: c_int =
              (buf.as_ptr ().add (countof! (buf)) as usize - bufp as usize) as c_int;
            let number_digits: c_int = number_bytes;

            /* C 1926-1929's `#if SUPPORT_NON_GREG_CALENDARS_IN_STRFTIME` arm is dead.  */
            /* C's `byte_count_t shortage = width - !!sign_char - number_digits;`:
               the subtraction is in ptrdiff_t and the result converts to the
               unsigned byte_count_t, so C's `shortage <= 0` is an *unsigned*
               comparison (`shortage == 0`).  */
            let shortage: byte_count_t =
              width.wrapping_sub ((sign_char != 0) as sbyte_count_t)
                   .wrapping_sub (number_digits as sbyte_count_t) as byte_count_t;
            let padding: byte_count_t = (if pad == NO_PAD || shortage <= 0
                                         { 0 } else { shortage });

            if sign_char != 0
              {
                if pad == SPACE_PAD
                  {
                    if ! p.is_null ()
                      { memset_space! (p, padding); }
                    /* C's `if (ckd_add (&i, i, padding) && FPRINTFTIME) { errno =
                       ERANGE; return FAILURE; }`: ckd_add stores the wrapped sum in
                       I whatever the condition, and FPRINTFTIME is 0 for this
                       configuration, so in practice only the store takes effect.  */
                    let (_v, _overflowed) = i.overflowing_add (padding as sbyte_count_t);
                    i = _v;
                    if _overflowed && FPRINTFTIME != 0
                      {
                        *errno () = ERANGE;
                        return FAILURE;
                      }
                    width = width.wrapping_sub (padding as sbyte_count_t);
                  }
                width_add1! (p, i, maxsize, pad, 0, sign_char);
                width -= 1;
              }

            cpy! (p, i, maxsize, pad, width, to_lowcase, to_uppcase,
                  number_bytes as byte_count_t, bufp);
          }
        }
      }

      width = -1;
      f = f.add (1);
    }

  if ! p.is_null () && maxsize != 0
    { *p = L_C! (b'\0'); }

  return i;
}
