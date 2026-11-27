// femacs: port of lib/limits.in.h @ fd7afced403 (gnulib template).
#![forbid(unsafe_code)]
//
// The reference is a gnulib template: `@GUARD_PREFIX@`, `@PRAGMA_SYSTEM_HEADER@`,
// `@PRAGMA_COLUMNS@`, `@INCLUDE_NEXT@` and `@NEXT_LIMITS_H@` are configured in, and the
// frozen substitution is the generated `build/emacs-ref/lib/limits.h` (guard
// `_GL_LIMITS_H`, `#include_next <limits.h>`).  The port has no `#include_next`, so this
// mirror is both halves: the system `<limits.h>` values the reference reaches through C 40
// (measured with the frozen target's clang) and the names the template defines itself.
//
// Every `#ifndef` in the template is false on the frozen target: the system header already
// defines LLONG_MIN/LLONG_MAX/ULLONG_MAX, WORD_BIT, LONG_BIT, MB_LEN_MAX, SSIZE_MAX and the
// whole C23 width family, so the template's own definitions of them are dead text.  The
// mirror still carries each name, because `extract-abi.py` reads a header's `#if`s blind
// unless the header is in `docs/baseline/config/cond-eval-verified.txt` and this one is not
// (P-08/P-66); each value is the one the frozen preprocessor has, and where that differs
// from the template's text the ledger and `docs/ports/excluded-macros.txt` record it.
//
// `_GL_INTEGER_WIDTH`/`_GL_COB*` (C 81-87) are `#if`-expression helpers: they count the 1
// bits of an integer type's maximum, signed or unsigned.  `_GL_INTEGER_WIDTH` keeps the
// signed machine word C's callers use and reads its MAX as the *unsigned* word - C's
// preprocessor does the same with an unsigned maximum - and `_GL_COB*` shifts that word
// logically, where the `>> 31 >> 31 >> 2` chain leaves the high part 0 for a 64-bit maximum
// (P-15).

// [U1-MODULE-MIRROR]
use super::*;
/* limits.in.h 44-45: the double-inclusion guard, `_@GUARD_PREFIX@_LIMITS_H`; the frozen
   substitution is `_GL_LIMITS_H`.  The extractor reads the template's own name as `_`
   (`@GUARD_PREFIX@` is not an identifier), which is the `docs/ports/excluded-macros.txt`
   row.  */
pub const _GL_LIMITS_H: c_int = 1;

/* --- limits.in.h 40's `# @INCLUDE_NEXT@ @NEXT_LIMITS_H@`: the system <limits.h> --------
   The port has no `#include_next`, so the values the reference reads from it are carried
   here; each is what the frozen target's compiler answers (clang on darwin-arm64).  */

pub const CHAR_BIT: c_int = 8;
pub const SCHAR_MIN: c_int = -128;
pub const SCHAR_MAX: c_int = 127;
#[cfg (not (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64"))))]
pub const CHAR_MIN: c_int = -128;         /* `char` is signed on this target */
/* Measured on bionic and aarch64-linux-gnu: `char` is unsigned (`CHAR_MIN = 0`). */
#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
pub const CHAR_MIN: c_int = 0;
pub const CHAR_MAX: c_int = 127;
pub const SHRT_MIN: c_int = -32768;
pub const SHRT_MAX: c_int = 32767;
pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;
pub const LONG_MIN: c_long = c_long::MIN;
pub const LONG_MAX: c_long = c_long::MAX;
pub const UCHAR_MAX: c_int = 0xFF;
pub const USHRT_MAX: c_int = 65535;
pub const UINT_MAX: c_uint = 4294967295;
pub const ULONG_MAX: c_ulong = c_ulong::MAX;

/* limits.in.h 47-79: `LLONG_MIN`, `LLONG_MAX`, `ULLONG_MAX` - C's `__GNUC__` arm
   (`(- __LONG_LONG_MAX__ - 1LL)` and `__LONG_LONG_MAX__`), which the system header's own
   values already agree with.  The type is named through `core::ffi` because a mirror is
   textually included and cannot add a `use` (PORTING.md 0.1/0.1-9).  */
pub const LLONG_MIN: core::ffi::c_longlong = core::ffi::c_longlong::MIN;
pub const LLONG_MAX: core::ffi::c_longlong = core::ffi::c_longlong::MAX;
pub const ULLONG_MAX: core::ffi::c_ulonglong = core::ffi::c_ulonglong::MAX;

/* limits.in.h 81: the number of usable bits in an integer type with minimum MIN and maximum
   MAX; C's `(((min) < 0) + _GL_COB128 (max))`.  MIN is signed and MAX is the same word read
   as unsigned (`ULLONG_MAX` arrives as `-1`'s bit pattern).  */
#[inline(always)]
pub const fn _GL_INTEGER_WIDTH (min: isize, max: isize) -> usize {
  ((min < 0) as usize) + _GL_COB128 (max as u64)
}
/* limits.in.h 82-87: count the 1 bits of the low-order N bits of the unsigned word.  */
#[inline(always)]
pub const fn _GL_COB128 (n: u64) -> usize { _GL_COB64 (n >> 31 >> 31 >> 2) + _GL_COB64 (n) }
#[inline(always)]
pub const fn _GL_COB64 (n: u64) -> usize { _GL_COB32 (n >> 31 >> 1) + _GL_COB32 (n) }
#[inline(always)]
pub const fn _GL_COB32 (n: u64) -> usize { _GL_COB16 (n >> 16) + _GL_COB16 (n) }
#[inline(always)]
pub const fn _GL_COB16 (n: u64) -> usize { _GL_COB8 (n >> 8) + _GL_COB8 (n) }
#[inline(always)]
pub const fn _GL_COB8 (n: u64) -> usize { _GL_COB4 (n >> 4) + _GL_COB4 (n) }
/* limits.in.h 87: `(!!((n) & 8) + !!((n) & 4) + !!((n) & 2) + !!((n) & 1))`.  */
#[inline(always)]
pub const fn _GL_COB4 (n: u64) -> usize {
  ((n & 8) != 0) as usize + ((n & 4) != 0) as usize
    + ((n & 2) != 0) as usize + ((n & 1) != 0) as usize
}

/* limits.in.h 89-92: `WORD_BIT` - C assumes `int` is 32 bits wide, which is what this
   target's system header says too.  */
pub const WORD_BIT: c_int = 32;

/* limits.in.h 94-101: `LONG_BIT` - C's `#if LONG_MAX == INT_MAX` is false here, so the
   template's value is 64 (the system header's own value as well).  */
#[cfg (not (target_os = "windows"))]
pub const LONG_BIT: c_int = 64;
#[cfg (target_os = "windows")]
pub const LONG_BIT: c_int = 32;

/* limits.in.h 103-106: `#ifndef MB_LEN_MAX / # define MB_LEN_MAX 16`.  The template's 16 is
   dead text on the frozen target - the system header the template includes next (its C 40)
   already defines `MB_LEN_MAX` as 6 - and 6 is the value the ported units size their
   multibyte buffers with, so the mirror keeps C's macro shape and the frozen value;
   docs/ports/include-limits.in.md records the measurement and
   `docs/ports/excluded-macros.txt` the deviation from the template's body.  */
macro_rules! MB_LEN_MAX {
  () => { 6 };
}

/* limits.in.h 108-123: the C23 width family.  C spells each as
   `_GL_INTEGER_WIDTH (MIN, MAX)` of the type; the port calls the same helper, with the
   signed minimum and the unsigned maximum.  */
pub const CHAR_WIDTH: c_int = _GL_INTEGER_WIDTH (CHAR_MIN as isize, CHAR_MAX as isize) as c_int;
pub const SCHAR_WIDTH: c_int = _GL_INTEGER_WIDTH (SCHAR_MIN as isize, SCHAR_MAX as isize) as c_int;
pub const UCHAR_WIDTH: c_int = _GL_INTEGER_WIDTH (0, UCHAR_MAX as isize) as c_int;
pub const SHRT_WIDTH: c_int = _GL_INTEGER_WIDTH (SHRT_MIN as isize, SHRT_MAX as isize) as c_int;
pub const USHRT_WIDTH: c_int = _GL_INTEGER_WIDTH (0, USHRT_MAX as isize) as c_int;
pub const INT_WIDTH: c_int = _GL_INTEGER_WIDTH (INT_MIN as isize, INT_MAX as isize) as c_int;
pub const UINT_WIDTH: c_int = _GL_INTEGER_WIDTH (0, UINT_MAX as isize) as c_int;
pub const LONG_WIDTH: c_int = _GL_INTEGER_WIDTH (LONG_MIN as isize, LONG_MAX as isize) as c_int;
pub const ULONG_WIDTH: c_int = _GL_INTEGER_WIDTH (0, ULONG_MAX as isize) as c_int;
pub const LLONG_WIDTH: c_int = _GL_INTEGER_WIDTH (LLONG_MIN as isize, LLONG_MAX as isize) as c_int;
pub const ULLONG_WIDTH: c_int = _GL_INTEGER_WIDTH (0, ULLONG_MAX as isize) as c_int;

/* limits.in.h 125-135: `BOOL_WIDTH` and `BOOL_MAX`; the live arm here is
   `# elif ! defined BOOL_MAX` (the system header defines BOOL_WIDTH but not BOOL_MAX), and
   both values C gives are 1.  */
pub const BOOL_WIDTH: c_int = 1;
pub const BOOL_MAX: c_int = 1;

/* limits.in.h 137-148: `SSIZE_MAX` - `_WIN64` is undefined, so C's value is `LONG_MAX`.  */
pub const SSIZE_MAX: c_long = c_long::MAX;

/* --- extra names this mirror carries for P-26's gaps -------------------------------------
   `PATH_MAX` is gnulib's pathmax.h fallback (the frozen macro table records 1024).
   `INTMAX_WIDTH`/`UINTMAX_WIDTH` were carried here as well while they had no home; C defines
   them in `lib/stdint.in.h` 725-726, so they moved to that mirror (`stdint.in.rs`), which
   every unit that names them includes (`src/bignum.rs`, `src/print.rs`).  */
pub const PATH_MAX: c_long = 1024;
