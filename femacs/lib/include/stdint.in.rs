// femacs: port of lib/stdint.in.h @ fd7afced403.
//
// The reference is a gnulib template: `@GUARD_PREFIX@`, `@PRAGMA_SYSTEM_HEADER@`,
// `@PRAGMA_COLUMNS@`, `@INCLUDE_NEXT@`, `@NEXT_STDINT_H@`, `@HAVE_STDINT_H@`,
// `@HAVE_C99_STDINT_H@`, `@GNULIBHEADERS_OVERRIDE_WINT_T@` and the `@BITSIZEOF_*@` /
// `@*_T_SUFFIX@` / `@HAVE_SIGNED_*@` family are configured in, and the frozen substitution is
// the generated `build/emacs-ref/lib/stdint.h` (guard `_GL_STDINT_H`, `#include_next
// <stdint.h>`).  The port has neither `#include_next` nor placeholders, so this mirror is both
// halves: every name the template can define, with the value and the type the frozen target's
// compiler gives it, measured with the frozen clang (`docs/ports/include-stdint.in.md` records
// the probes).
//
// The template's whole type-and-limit section sits inside `#if ! @HAVE_C99_STDINT_H@`, and that
// substitution is 1 here, so C 88-677 is dead text: the system `<stdint.h>` the template
// includes next (C 69) supplies every typedef and every limit.  `extract-abi.py` reads a gnulib
// template blind (P-08/P-66: this file is not in `docs/baseline/config/cond-eval-verified.txt`,
// where the frozen macro table cannot decide a `@PLACEHOLDER@` condition), so the mirror still
// carries each name that dead text defines; the rows where a C name has no Rust item of its
// own (the `gl_*` type renames, the three 16-bit fast limits and the two placeholder bodies)
// are quoted in `docs/ports/include-stdint.in.md` for `docs/ports/excluded-macros.txt`.
//
// The `_GL_INTEGER_WIDTH` helper of the TS 18661-1 width block (C 81-87 in its own header)
// comes from `limits.in.rs`, which the template includes at C 77.

/* stdint.in.h 77: `#include <limits.h>` - `_GL_INTEGER_WIDTH`, and the `SCHAR_MIN`,
   `SCHAR_MAX`, `UCHAR_MAX`, `LONG_MIN`, `LONG_MAX`, `ULONG_MAX` the template's own bodies
   name.  */
include!("limits.in.rs");

/* stdint.in.h 23/72-73: the double-inclusion guard, `_@GUARD_PREFIX@_STDINT_H`; the frozen
   substitution is `_GL_STDINT_H`.  The extractor reads the template's own name as `_`
   (`@GUARD_PREFIX@` is not an identifier), which is the `docs/ports/excluded-macros.txt`
   row.  */
pub const _GL_STDINT_H: core::ffi::c_int = 1;

/* stdint.in.h 34: `#define _GL_JUST_INCLUDE_SYSTEM_INTTYPES_H`, the marker the template sets
   before it includes the system `<stdint.h>` so that an `<inttypes.h>` pulled in by a system
   header does not re-enter gnulib's substitute (C 30-33's comment); `lib/inttypes.in.h` 32/46
   tests it with `#ifdef`.  The port has one `<stdint.h>` mirror, so the marker is the value
   the frozen preprocessor holds.  */
pub const _GL_JUST_INCLUDE_SYSTEM_INTTYPES_H: core::ffi::c_int = 1;

/* --- the type half (C 88-360; dead text on the frozen target) -----------------------------

   `#if ! @HAVE_C99_STDINT_H@` (C 88) is false, so C never reaches these `typedef`s and the
   system `<stdint.h>` supplies the types.  The port spells each C name as a `pub type` alias of
   the frozen target's own type (the width and the type of every one is measured; the template's
   dead arms instead name `gl_*`, `long`, `long long` and `__int64` depending on the platform).
   The aliases are what a ported unit writes where C writes the typedef, and each needs one
   `docs/ports/excluded-macros.txt` row (listed in the ledger), because the template defines
   the name as a macro and a `pub type` is not an item the extractor reads (the `intmax_t`/
   `uintmax_t` rows are already there for the other platform shim; P-26 keeps one spelling per
   construct).  `intptr_t`, `intmax_t` and `uintmax_t` are declared by the `lisp.h` mirror
   (`src/include/lisp.rs` 1067, 52-53), which is where C's `lisp.h` 1113 also uses them.  */

/* C 146-161: 7.18.1.1's exact-width types (the dead arms' `gl_int8_t` family).  */
pub type int8_t = core::ffi::c_schar;
pub type uint8_t = core::ffi::c_uchar;
pub type int16_t = core::ffi::c_short;
pub type uint16_t = core::ffi::c_ushort;
pub type int32_t = core::ffi::c_int;
pub type uint32_t = core::ffi::c_uint;
pub type int64_t = core::ffi::c_longlong;
pub type uint64_t = core::ffi::c_ulonglong;

/* C 135-154: 7.18.1.2's minimum-width types; each is the exact-width type of the same width
   (`# define int_least8_t int8_t`, …).  */
pub type int_least8_t = int8_t;
pub type uint_least8_t = uint8_t;
pub type int_least16_t = int16_t;
pub type uint_least16_t = uint16_t;
pub type int_least32_t = int32_t;
pub type uint_least32_t = uint32_t;
pub type int_least64_t = int64_t;
pub type uint_least64_t = uint64_t;

/* C 167-201: 7.18.1.3's fastest minimum-width types.  The template's dead arm makes the 16-bit
   pair the 32-bit ones (`typedef gl_int_fast32_t gl_int_fast16_t`, C 187) and widens the
   32-bit pair to `long`, but the frozen target's `<stdint.h>` gives them `short` and `int`
   (`__INT_FAST16_TYPE__` is `short`, `__INT_FAST32_TYPE__` is `int`, both 2 and 4 bytes,
   measured), so the aliases follow the frozen types rather than that arm.  */
pub type int_fast8_t = core::ffi::c_schar;
pub type uint_fast8_t = core::ffi::c_uchar;
pub type int_fast16_t = core::ffi::c_short;
pub type uint_fast16_t = core::ffi::c_ushort;
pub type int_fast32_t = core::ffi::c_int;
pub type uint_fast32_t = core::ffi::c_uint;
pub type int_fast64_t = int64_t;
pub type uint_fast64_t = uint64_t;

/* C 323-324: 7.18.1.4's unsigned pointer-width type.  C's `uintptr_t` is `unsigned long` here
   (measured); the port's `usize` is that type's width and PORTING.md 1.1's mapping.  */
pub type uintptr_t = usize;

/* C 192/218: `GL_INT64_T`/`GL_UINT64_T`, the markers the template sets (with no value) once
   the 64-bit types are the system's; the port defines each type once, so the marker is
   carried the way the `endian.in.rs` mirror carries its `GNULIB_defined_*` family.  */
pub const GL_INT64_T: core::ffi::c_int = 1;
pub const GL_UINT64_T: core::ffi::c_int = 1;

/* C 223-225: `# define _UINT8_T`, `_UINT32_T`, `_UINT64_T` - the markers that keep a later
   system typedef from colliding with the aliases above.  */
pub const _UINT8_T: core::ffi::c_int = 1;
pub const _UINT32_T: core::ffi::c_int = 1;
pub const _UINT64_T: core::ffi::c_int = 1;

/* C 370: `# define GNULIB_defined_stdint_types 1`, the marker that keeps a second inclusion of
   gnulib's `<stdint.h>` from redefining the wrappers (C 135's `#if
   !GNULIB_defined_stdint_types`).  */
pub const GNULIB_defined_stdint_types: core::ffi::c_int = 1;

/* C 57/60: `__STDC_CONSTANT_MACROS`/`__STDC_LIMIT_MACROS`, the pre-C++11 opt-ins of the
   `#ifdef __cplusplus` arm (dead in C, the value the template's text gives).

   The port does **not** declare them here: `config.h` 2704 has
   `/* #undef __STDC_CONSTANT_MACROS */`, and in C a `#undef` after `<stdint.h>` cancels the
   header's definition, so the configuration's value is the only one that exists.  Declaring the
   template's `1` as well both collided with `config.rs` and contradicted it (status 4.634).  */

/* --- the template's own helper macros (C 124-133) -----------------------------------------

   C's `_STDINT_UNSIGNED_MIN`/`_STDINT_SIGNED_MIN`/`_STDINT_MAX` compute a type's minimum and
   maximum from its bit width; every *use* of them is in the dead section, so nothing in the
   port expands these, but the templates that would (the limits below) are mirrored in this
   shape.  C's order is UNSIGNED_MIN (127), SIGNED_MIN (129), MAX (132); the port defines MAX
   first because a `macro_rules!` body is only visible after its definition (PORTING.md
   P-57).  */

/* C 132: `(((((zero) + 1) << ((bits) ? (bits) - 1 - (signed) : 0)) - 1) * 2 + 1)`.  */
macro_rules! _STDINT_MAX {
  ($signed:expr, $bits:expr, $zero:expr) => {
    (((((($zero) + 1) << (if ($bits) != 0 { ($bits) - 1 - ($signed) } else { 0 }))
        - 1) * 2 + 1))
  };
}

/* C 127-128: `_STDINT_UNSIGNED_MIN (bits, zero)` is C's `(zero)`.  */
macro_rules! _STDINT_UNSIGNED_MIN {
  ($bits:expr, $zero:expr) => { $zero };
}

/* C 129-130: `_STDINT_SIGNED_MIN (bits, zero)` is C's `(~ _STDINT_MAX (1, bits, zero))`.  */
macro_rules! _STDINT_SIGNED_MIN {
  ($bits:expr, $zero:expr) => { !_STDINT_MAX! (1, $bits, $zero) };
}

/* --- the value half (C 375-675) ------------------------------------------------------------

   Every value and type below is the frozen target's, measured with the frozen clang; the
   system `<stdint.h>` the template includes next (C 69) is where they come from, and the
   template's own definitions of them are dead text.  The `_C` family (C 623-675) is a
   `macro_rules!` here because a Rust macro cannot append C's literal suffix; C defines those
   from C 623 on, and the port defines them *before* the limits that expand them, for the
   same P-57 reason.  */

/* C 625-653: 7.18.4.1's least-width constants, C's `x`, `x ## U`, `x##LL`, `x##ULL`.  */
macro_rules! INT8_C { ($x:expr) => { $x }; }
macro_rules! UINT8_C { ($x:expr) => { $x }; }
macro_rules! INT16_C { ($x:expr) => { $x }; }
macro_rules! UINT16_C { ($x:expr) => { $x }; }
macro_rules! INT32_C { ($x:expr) => { $x }; }
macro_rules! UINT32_C { ($x:expr) => { $x as core::ffi::c_uint }; }
macro_rules! INT64_C { ($x:expr) => { $x as core::ffi::c_longlong }; }
macro_rules! UINT64_C { ($x:expr) => { $x as core::ffi::c_ulonglong }; }

/* C 657-675: 7.18.4.2's greatest-width constants, C's `x##L` and `x##UL` (the frozen
   `intmax_t`/`uintmax_t` are `long`/`unsigned long`, measured).  */
macro_rules! INTMAX_C { ($x:expr) => { $x as core::ffi::c_long }; }
macro_rules! UINTMAX_C { ($x:expr) => { $x as core::ffi::c_ulong }; }

/* C 383-409: 7.18.2.1's exact-width limits.  Each minimum is C's `(~ MAX)`, and the types are
   the measured ones (`int`, `int`/`unsigned int`, `long long`/`unsigned long long`).  */
pub const INT8_MIN: core::ffi::c_int = !INT8_MAX;
pub const INT8_MAX: core::ffi::c_int = 127;
pub const UINT8_MAX: core::ffi::c_int = 255;
pub const INT16_MIN: core::ffi::c_int = !INT16_MAX;
pub const INT16_MAX: core::ffi::c_int = 32767;
pub const UINT16_MAX: core::ffi::c_int = 65535;
pub const INT32_MIN: core::ffi::c_int = !INT32_MAX;
pub const INT32_MAX: core::ffi::c_int = 2147483647;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295;
pub const INT64_MIN: core::ffi::c_longlong = -INTMAX_C! (1i64) << 63;   /* C 404 */
pub const INT64_MAX: core::ffi::c_longlong = INTMAX_C! (9223372036854775807i64);  /* C 405 */
pub const UINT64_MAX: core::ffi::c_ulonglong = UINTMAX_C! (18446744073709551615u64); /* C 409 */

/* C 421-448: 7.18.2.2's minimum-width limits; each is the exact-width limit of the same width
   (`# define INT_LEAST8_MAX INT8_MAX`, …), so each keeps that limit's type.  */
pub const INT_LEAST8_MIN: core::ffi::c_int = INT8_MIN;
pub const INT_LEAST8_MAX: core::ffi::c_int = INT8_MAX;
pub const UINT_LEAST8_MAX: core::ffi::c_int = UINT8_MAX;
pub const INT_LEAST16_MIN: core::ffi::c_int = INT16_MIN;
pub const INT_LEAST16_MAX: core::ffi::c_int = INT16_MAX;
pub const UINT_LEAST16_MAX: core::ffi::c_int = UINT16_MAX;
pub const INT_LEAST32_MIN: core::ffi::c_int = INT32_MIN;
pub const INT_LEAST32_MAX: core::ffi::c_int = INT32_MAX;
pub const UINT_LEAST32_MAX: core::ffi::c_uint = UINT32_MAX;
pub const INT_LEAST64_MIN: core::ffi::c_longlong = INT64_MIN;
pub const INT_LEAST64_MAX: core::ffi::c_longlong = INT64_MAX;
pub const UINT_LEAST64_MAX: core::ffi::c_ulonglong = UINT64_MAX;

/* C 460-493: 7.18.2.3's fastest minimum-width limits.  C's dead arm spells the 16-bit pair as
   the 32-bit macros (`# define INT_FAST16_MAX INT_FAST32_MAX`) and the 32-bit pair as
   `LONG_MIN`/`LONG_MAX`, but the frozen types are `short` and `int` (measured: 32767 and
   2147483647), so the mirror keeps the frozen limits;
   the three 16-bit names (`INT_FAST16_MIN`, `INT_FAST16_MAX`, `UINT_FAST16_MAX`) need a
   `docs/ports/excluded-macros.txt` row because their C bodies name the 32-bit macros (the
   rows are quoted in the ledger).  */
pub const INT_FAST8_MIN: core::ffi::c_int = SCHAR_MIN;
pub const INT_FAST8_MAX: core::ffi::c_int = SCHAR_MAX;
pub const UINT_FAST8_MAX: core::ffi::c_int = UCHAR_MAX;
pub const INT_FAST16_MIN: core::ffi::c_int = INT16_MIN;
pub const INT_FAST16_MAX: core::ffi::c_int = INT16_MAX;
pub const UINT_FAST16_MAX: core::ffi::c_int = UINT16_MAX;
pub const INT_FAST32_MIN: core::ffi::c_int = INT32_MIN;
pub const INT_FAST32_MAX: core::ffi::c_int = INT32_MAX;
pub const UINT_FAST32_MAX: core::ffi::c_uint = UINT32_MAX;
pub const INT_FAST64_MIN: core::ffi::c_longlong = INT64_MIN;
pub const INT_FAST64_MAX: core::ffi::c_longlong = INT64_MAX;
pub const UINT_FAST64_MAX: core::ffi::c_ulonglong = UINT64_MAX;

/* C 506-508: 7.18.2.4's pointer-width limits.  `intptr_t` is `long` here (measured), so C's
   bodies are `LONG_MIN`/`LONG_MAX`, and `uintptr_t` is `unsigned long`, so `ULONG_MAX` - which
   the port's `usize` reads through one cast.  */
pub const INTPTR_MIN: core::ffi::c_long = LONG_MIN;
pub const INTPTR_MAX: core::ffi::c_long = LONG_MAX;
pub const UINTPTR_MAX: uintptr_t = ULONG_MAX as uintptr_t;

/* C 519-528: 7.18.2.5's greatest-width limits.  The port's `intmax_t`/`uintmax_t` are `i64`/
   `u64` (PORTING.md 1.1); the frozen values are C's 64-bit `long`/`unsigned long` ones.  */
pub const INTMAX_MIN: i64 = i64::MIN;
pub const INTMAX_MAX: i64 = i64::MAX;
pub const UINTMAX_MAX: u64 = u64::MAX;

/* C 546-548: `PTRDIFF_MIN`/`PTRDIFF_MAX`, C's `_STDINT_SIGNED_MIN (64, 0l)` and `_STDINT_MAX
   (1, 64, 0l)` after substitution; the port's `ptrdiff_t` is `isize` (PORTING.md 1.1).  */
pub const PTRDIFF_MIN: isize = isize::MIN;
pub const PTRDIFF_MAX: isize = isize::MAX;

/* C 559-562: `SIG_ATOMIC_MIN`/`SIG_ATOMIC_MAX`; the frozen `sig_atomic_t` is `int` (measured),
   so C's `_STDINT_UNSIGNED_MIN (32, 0)`/`_STDINT_MAX (1, 32, 0)` substitutions are the 32-bit
   signed pair the system header holds.  */
pub const SIG_ATOMIC_MIN: core::ffi::c_int = core::ffi::c_int::MIN;
pub const SIG_ATOMIC_MAX: core::ffi::c_int = core::ffi::c_int::MAX;

/* C 576: `SIZE_MAX`; C's body is `_STDINT_MAX (0, 64, 0ul)` after substitution, and the port's
   `size_t` is `usize` (PORTING.md 1.1).  */
pub const SIZE_MAX: usize = usize::MAX;

/* C 582: `_GL_JUST_INCLUDE_SYSTEM_WCHAR_H`, the marker that keeps `<wchar.h>` from re-entering
   gnulib's substitute while the system `<stdint.h>` includes it (C 588's include).  */
pub const _GL_JUST_INCLUDE_SYSTEM_WCHAR_H: core::ffi::c_int = 1;

/* C 592-595: `WCHAR_MIN`/`WCHAR_MAX`; the frozen `wchar_t` is signed `int` (measured), so C's
   `_STDINT_SIGNED_MIN (32, 0)`/`_STDINT_MAX (1, 32, 0)` substitutions are the 32-bit signed
   pair.  */
pub const WCHAR_MIN: core::ffi::c_int = core::ffi::c_int::MIN;
pub const WCHAR_MAX: core::ffi::c_int = core::ffi::c_int::MAX;

/* C 608-611: `WINT_MIN`/`WINT_MAX`; the frozen `wint_t` is `int` (measured), the same 32-bit
   signed pair.  */
pub const WINT_MIN: core::ffi::c_int = core::ffi::c_int::MIN;
pub const WINT_MAX: core::ffi::c_int = core::ffi::c_int::MAX;

/* --- C 679-736: the ISO/IEC TS 18661-1 width family ---------------------------------------

   Each is C's `_GL_INTEGER_WIDTH (MIN, MAX)` of its type, with the signed minimum and the
   unsigned maximum (`limits.in.rs` 81 does the same with an unsigned maximum); every frozen
   value is the type's width, 8/16/32/64 as measured.  `PTRDIFF_WIDTH` is the one whose port
   type is `usize`, because the units that size a buffer with it do so in `usize` arithmetic
   (`src/sort.rs` 124, `src/buffer.rs` 1358); every other width is C's `int`.  */

pub const INT8_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT8_MIN as isize, INT8_MAX as isize) as core::ffi::c_int;      /* C 684 */
pub const UINT8_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT8_MAX as isize) as core::ffi::c_int;                     /* C 687 */
pub const INT16_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT16_MIN as isize, INT16_MAX as isize) as core::ffi::c_int;    /* C 690 */
pub const UINT16_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT16_MAX as isize) as core::ffi::c_int;                    /* C 693 */
pub const INT32_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT32_MIN as isize, INT32_MAX as isize) as core::ffi::c_int;    /* C 696 */
pub const UINT32_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT32_MAX as isize) as core::ffi::c_int;                    /* C 699 */
pub const INT64_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT64_MIN as isize, INT64_MAX as isize) as core::ffi::c_int;    /* C 702 */
pub const UINT64_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT64_MAX as isize) as core::ffi::c_int;                    /* C 705 */
pub const INT_LEAST8_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT_LEAST8_MIN as isize, INT_LEAST8_MAX as isize)
    as core::ffi::c_int;                                                            /* C 707 */
pub const UINT_LEAST8_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT_LEAST8_MAX as isize) as core::ffi::c_int;               /* C 708 */
pub const INT_LEAST16_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT_LEAST16_MIN as isize, INT_LEAST16_MAX as isize)
    as core::ffi::c_int;                                                            /* C 709 */
pub const UINT_LEAST16_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT_LEAST16_MAX as isize) as core::ffi::c_int;              /* C 710 */
pub const INT_LEAST32_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT_LEAST32_MIN as isize, INT_LEAST32_MAX as isize)
    as core::ffi::c_int;                                                            /* C 711 */
pub const UINT_LEAST32_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT_LEAST32_MAX as isize) as core::ffi::c_int;              /* C 712 */
pub const INT_LEAST64_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT_LEAST64_MIN as isize, INT_LEAST64_MAX as isize)
    as core::ffi::c_int;                                                            /* C 713 */
pub const UINT_LEAST64_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT_LEAST64_MAX as isize) as core::ffi::c_int;              /* C 714 */
pub const INT_FAST8_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT_FAST8_MIN as isize, INT_FAST8_MAX as isize)
    as core::ffi::c_int;                                                            /* C 715 */
pub const UINT_FAST8_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT_FAST8_MAX as isize) as core::ffi::c_int;                /* C 716 */
pub const INT_FAST16_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT_FAST16_MIN as isize, INT_FAST16_MAX as isize)
    as core::ffi::c_int;                                                            /* C 717 */
pub const UINT_FAST16_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT_FAST16_MAX as isize) as core::ffi::c_int;               /* C 718 */
pub const INT_FAST32_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT_FAST32_MIN as isize, INT_FAST32_MAX as isize)
    as core::ffi::c_int;                                                            /* C 719 */
pub const UINT_FAST32_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT_FAST32_MAX as isize) as core::ffi::c_int;               /* C 720 */
pub const INT_FAST64_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INT_FAST64_MIN as isize, INT_FAST64_MAX as isize)
    as core::ffi::c_int;                                                            /* C 721 */
pub const UINT_FAST64_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINT_FAST64_MAX as isize) as core::ffi::c_int;               /* C 722 */
pub const INTPTR_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INTPTR_MIN as isize, INTPTR_MAX as isize) as core::ffi::c_int;  /* C 723 */
pub const UINTPTR_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINTPTR_MAX as isize) as core::ffi::c_int;                   /* C 724 */
pub const INTMAX_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (INTMAX_MIN as isize, INTMAX_MAX as isize) as core::ffi::c_int;  /* C 725 */
pub const UINTMAX_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, UINTMAX_MAX as isize) as core::ffi::c_int;                   /* C 726 */
pub const PTRDIFF_WIDTH: usize =
  _GL_INTEGER_WIDTH (PTRDIFF_MIN, PTRDIFF_MAX);                                      /* C 727 */
pub const SIZE_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (0, SIZE_MAX as isize) as core::ffi::c_int;                      /* C 728 */
pub const WCHAR_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (WCHAR_MIN as isize, WCHAR_MAX as isize) as core::ffi::c_int;    /* C 729 */
pub const WINT_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (WINT_MIN as isize, WINT_MAX as isize) as core::ffi::c_int;      /* C 731 */
pub const SIG_ATOMIC_WIDTH: core::ffi::c_int =
  _GL_INTEGER_WIDTH (SIG_ATOMIC_MIN as isize, SIG_ATOMIC_MAX as isize)
    as core::ffi::c_int;                                                            /* C 734 */
