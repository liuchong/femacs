// femacs: port of lib/stdbit.in.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1, P-26).  This is the largest gnulib template in the tree: 2,229
// lines, and the extractor's item list is `173 functions defined / 90 macros / 1 file-scope
// object`.  **The C23 API is closed here**: every family the frozen target compiles is landed
// below (the settings layer first, then the four `first_*` families, `has_single_bit`,
// `bit_floor`/`bit_ceil`, `rotate_left`/`rotate_right`, `memreverse8`, and the whole
// `load8`/`store8`/`_aligned` matrix), and the only items the mirror does *not* carry are eight
// dead-arm rows recorded in the shared tables - see `## Decisions` in the ledger, which names all
// eight and the C lines of each.
//
// **The judge is the generated header, and this template is fully live.**  `ls -l
// build/emacs-ref/lib/stdbit.h` -> 60,264 bytes, 2,230 lines; `grep -n HAVE_STDBIT_H
// build/emacs-ref/src/config.h` -> `1433:/* #undef HAVE_STDBIT_H */`, so the system has **no**
// `<stdbit.h>` and gnulib's header defines the whole C23 API itself (its C 40-46 `#if 0` block
// around `# include_next <stdbit.h>` is the dead arm).  The frozen macro table agrees
// (`HAVE_STDBIT_H	undef`).  (Note for the record: `femacs/lib/include/stdbit.rs` 3 says "the frozen
// configuration has HAVE_STDBIT_H = 1" - that reading is inverted; this lane measured it as
// `undef` from both config.h and the table, and `stdbit.rs` is an unpaired mirror, so nothing
// `--check`ed the sentence.  The slice `stdbit.rs` carries is still valid: it mirrors the *live*
// builtin arms, which is what `undef` implies.)
//
// Conditions: `tools/check-cond-eval.py lib/stdbit.in.h` -> `135 conditions, 31 unknown,
// 101 disagreements [NOT VERIFIED]`, so the extractor resolves nothing and its item list is the
// union of every arm (P-66's conservative reading; the 22 branch macros are reported as
// "several definitions … body not compared").  The arms that matter are decided by the *frozen
// compiler* instead, and `stdbit.rs` 5-15 records the readings: `_GL_STDBIT_HAS_BUILTIN_CLZ/CTZ/
// POPCOUNT` and `_GL_STDBIT_HAS_BUILTIN_BSWAP16/32/64` are the live arms, so each family's
// builtin body is the one the port spells and the hand-written fallbacks are dead text.
//
// **The one idiom the families need, measured here.**  C's dispatchers and the narrow variants
// compute widths with `sizeof 0u`/`sizeof 0ull`/`sizeof 0ul` (e.g. C 317-345), whose literal `0` is
// a *token* the gate requires of the Rust body.  Rust has no `sizeof 0u`, but
// `std::mem::size_of_val (&0u32)` is the same expression *and* keeps the literal, so
// `stdc_leading_zeros_uc`'s C body `stdc_leading_zeros_ui (n) - 8 * (sizeof 0u - sizeof n)`
// becomes `stdc_leading_zeros_ui (n) - 8 * (std::mem::size_of_val (&0u32) - std::mem::size_of_val (&n))`
// with every token C's own - which is why no member of the width-arithmetic families needs a
// `docs/ports/macro-body-notes.tsv` row for its `0`.
//
// **Measured behaviour, not just tokens.**  A `/tmp` probe
// (`rustc +nightly-2026-10-01 --edition 2024`, `include!`ing this mirror) checks the family layer
// against the *standard library's own operation for the same function* - `is_power_of_two`,
// `rotate_left`/`rotate_right`, `from_le_bytes`/`to_le_bytes`, `from_be_bytes`/`to_be_bytes`,
// `leading_ones`/`trailing_zeros` - rather than against literals: `544 semantic checks passed`,
// exit 0.  The ledger's `## Evidence` has the command, the readings, and the two negative controls.

include!("stdint.in.rs");   // `uint_leastN_t` for the `stdc_memreverse8u*` family
include!("stdbit.rs");   // the frozen target's `<stdbit.h>` slice: the live builtin arms
                        // (`_gl_stdbit_clz/ctzl/clzll`, their `stdc_leading_zeros_*` users, the
                        // three generic dispatchers and the `stdc_memreverse8u*` bodies) that
                        // `lisp.rs` and the extern-inline units already include.  The relation is
                        // `lib/include/errno.in.rs`'s (`include!("errno.rs")`): the template mirror
                        // names the file the frozen configuration generated for it.

/// C 40-46: `#if @HAVE_STDBIT_H@` … `# @INCLUDE_NEXT@ @NEXT_STDBIT_H@` … - dead here
/// (`HAVE_STDBIT_H` is `undef`), so the template defines the API itself.  C 42-43's FreeBSD
/// `_Bool` spelling is dead too (`__FreeBSD__` undefined), and the extractor already records the
/// two placeholder-shaped names of this prologue (`_`, `_Bool`) in
/// `docs/ports/excluded-macros.txt`.

/// C 44-48: `_@GUARD_PREFIX@_STDBIT_H` is the template's guard, whose substitution is
/// `_GL_STDBIT_H` (the extractor reads the placeholder's name as `_`, which the excluded row
/// above records); and C 46-47's `# define __need_size_t` / `# include <stddef.h>` is the
/// `@GNULIB_STDC_MEMREVERSE8@` arm, live on this target.
pub const __need_size_t: bool = true;

/// C 202-210: the C23 endian macros, live because `__STDC_VERSION_STDBIT_H__` is undefined in the
/// frozen compiler's `<stdbit.h>` (it has none).  C 209's arm chooses by `WORDS_BIGENDIAN`, which
/// this target does not define (arm64 darwin is little-endian), so C's `#else` arm is the one the
/// extractor records as the last definition.
/// C: #define __STDC_ENDIAN_BIG__ 4321
/// C: #define __STDC_ENDIAN_LITTLE__ 1234
/// C: # define __STDC_ENDIAN_NATIVE__ __STDC_ENDIAN_LITTLE__
pub const __STDC_ENDIAN_BIG__: c_int = 4321;
pub const __STDC_ENDIAN_LITTLE__: c_int = 1234;
pub const __STDC_ENDIAN_NATIVE__: c_int = __STDC_ENDIAN_LITTLE__;

/// C 2224-2226: `# define __STDC_VERSION_STDBIT_H__ 202311L` - C's suffix is Rust's absence of one.
pub const __STDC_VERSION_STDBIT_H__: c_long = 202311;

/// C 60-64: the fastest unsigned types the families use, in the arm the frozen compiler takes
/// (`__GNUC__` is defined, so C 63-64's `__UINT_FAST*_TYPE__` spellings are the live ones).
/// A *type* macro, so each is a nullary `macro_rules!` (the `include/msdos.rs` `PIX_TYPE`
/// spelling).
/// C: #  define _GL_STDBIT_UINT_FAST16 __UINT_FAST16_TYPE__
/// C: #  define _GL_STDBIT_UINT_FAST32 __UINT_FAST32_TYPE__
/// C: #  define _GL_STDBIT_UINT_FAST64 __UINT_FAST64_TYPE__
macro_rules! _GL_STDBIT_UINT_FAST16 { () => { __UINT_FAST16_TYPE__ }; }
macro_rules! _GL_STDBIT_UINT_FAST32 { () => { __UINT_FAST32_TYPE__ }; }
macro_rules! _GL_STDBIT_UINT_FAST64 { () => { __UINT_FAST64_TYPE__ }; }

/// C 96-105: `# ifdef __has_builtin` / `#  define _GL_STDBIT_MEMCPY(dest, src, n)
/// __builtin_memcpy (dest, src, n)` and the `# ifndef` arm below it with
/// `memcpy (dest, src, n)`.  The port spells it with the family
/// `docs/ports/token-aliases.tsv` gives `memcpy` (`std::ptr::copy_nonoverlapping`, whose
/// occurrence check keeps the call alive); C's `void *` accepts any two pointer types, and
/// Rust's copy needs one, so both sides go through `c_uchar` - a no-op for the byte copies C's
/// `_aligned` arms make (`memcpy (&value, ptr, sizeof value)`).
/// C: #   define _GL_STDBIT_MEMCPY(dest, src, n) __builtin_memcpy (dest, src, n)
/// C: #  define _GL_STDBIT_MEMCPY(dest, src, n) memcpy (dest, src, n)
macro_rules! _GL_STDBIT_MEMCPY {
  ($dest:expr, $src:expr, $n:expr) => {
    unsafe { std::ptr::copy_nonoverlapping (($src) as *const _ as *const c_uchar,
                                            ($dest) as *mut _ as *mut c_uchar, $n) }
  };
}

/// C 186-190: `# define _GL_STDBIT_TYPEOF_CAST(a, b) (b)` - the arm the frozen compiler records
/// last (the other arm tests `typeof` support).
/// C: # define _GL_STDBIT_TYPEOF_CAST(a, b) (b)
macro_rules! _GL_STDBIT_TYPEOF_CAST { ($a:expr, $b:expr) => { ($b) }; }

/// C 1606-1622: the alignment hint.  `_GL_HAS_BUILTIN_ASSUME_ALIGNED` is defined for this
/// compiler (C 1611's `# define … 1`), so C 1622's `__builtin_assume_aligned (ptr, align)` arm
/// is the live one - and Rust has no such builtin.  The port therefore spells C's own `#else`
/// arm, C 1623's `(ptr)`, which is the same *value* with the same tokens
/// (`docs/ports/token-aliases.tsv`'s `memcpy` row is the sibling decision), and nothing in the
/// port needs the hint.  The C live arm's text stays above for the record (P-55).
/// C: # define _GL_HAS_BUILTIN_ASSUME_ALIGNED 1
/// C: # define _GL_STDBIT_ASSUME_ALIGNED(ptr, align) __builtin_assume_aligned (ptr, align)
/// C: # define _GL_STDBIT_ASSUME_ALIGNED(ptr, align) (ptr)
pub const _GL_HAS_BUILTIN_ASSUME_ALIGNED: c_int = 1;
macro_rules! _GL_STDBIT_ASSUME_ALIGNED {
  ($ptr:expr, $align:expr) => { ($ptr) };
}

/// C 1624-1640: the `memcpy`-based optimization flag; C 1638-1640's `#ifndef` arm supplies the
/// value the frozen configuration ends with (`0` - the first arm's condition is about
/// `uint_leastN_t` widths and `CHAR_BIT`, and C 1639's last definition is what the extractor
/// compares).
/// C: # define _GL_STDBIT_OPTIMIZE_VIA_MEMCPY 0
pub const _GL_STDBIT_OPTIMIZE_VIA_MEMCPY: c_int = 0;

/// C 1642: `#define _GL_STDBIT_BIGENDIAN (__STDC_ENDIAN_NATIVE__ == __STDC_ENDIAN_BIG__)` - both
/// names are the constants above, so the port spells C's expression.
pub const _GL_STDBIT_BIGENDIAN: bool = __STDC_ENDIAN_NATIVE__ == __STDC_ENDIAN_BIG__;

/// C 150: `# define _GL_STDC_BIT_CEIL_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_BIT_CEIL_INLINE _GL_INLINE
macro_rules! _GL_STDC_BIT_CEIL_INLINE { () => { _GL_INLINE }; }

/// C 147: `# define _GL_STDC_BIT_FLOOR_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_BIT_FLOOR_INLINE _GL_INLINE
macro_rules! _GL_STDC_BIT_FLOOR_INLINE { () => { _GL_INLINE }; }

/// C 144: `# define _GL_STDC_BIT_WIDTH_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_BIT_WIDTH_INLINE _GL_INLINE
macro_rules! _GL_STDC_BIT_WIDTH_INLINE { () => { _GL_INLINE }; }

/// C 138: `# define _GL_STDC_COUNT_ONES_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_COUNT_ONES_INLINE _GL_INLINE
macro_rules! _GL_STDC_COUNT_ONES_INLINE { () => { _GL_INLINE }; }

/// C 135: `# define _GL_STDC_COUNT_ZEROS_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_COUNT_ZEROS_INLINE _GL_INLINE
macro_rules! _GL_STDC_COUNT_ZEROS_INLINE { () => { _GL_INLINE }; }

/// C 126: `# define _GL_STDC_FIRST_LEADING_ONE_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_FIRST_LEADING_ONE_INLINE _GL_INLINE
macro_rules! _GL_STDC_FIRST_LEADING_ONE_INLINE { () => { _GL_INLINE }; }

/// C 123: `# define _GL_STDC_FIRST_LEADING_ZERO_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_FIRST_LEADING_ZERO_INLINE _GL_INLINE
macro_rules! _GL_STDC_FIRST_LEADING_ZERO_INLINE { () => { _GL_INLINE }; }

/// C 132: `# define _GL_STDC_FIRST_TRAILING_ONE_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_FIRST_TRAILING_ONE_INLINE _GL_INLINE
macro_rules! _GL_STDC_FIRST_TRAILING_ONE_INLINE { () => { _GL_INLINE }; }

/// C 129: `# define _GL_STDC_FIRST_TRAILING_ZERO_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_FIRST_TRAILING_ZERO_INLINE _GL_INLINE
macro_rules! _GL_STDC_FIRST_TRAILING_ZERO_INLINE { () => { _GL_INLINE }; }

/// C 141: `# define _GL_STDC_HAS_SINGLE_BIT_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_HAS_SINGLE_BIT_INLINE _GL_INLINE
macro_rules! _GL_STDC_HAS_SINGLE_BIT_INLINE { () => { _GL_INLINE }; }

/// C 114: `# define _GL_STDC_LEADING_ONES_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_LEADING_ONES_INLINE _GL_INLINE
macro_rules! _GL_STDC_LEADING_ONES_INLINE { () => { _GL_INLINE }; }

/// C 111: `# define _GL_STDC_LEADING_ZEROS_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_LEADING_ZEROS_INLINE _GL_INLINE
macro_rules! _GL_STDC_LEADING_ZEROS_INLINE { () => { _GL_INLINE }; }

/// C 165: `# define _GL_STDC_LOAD8_ALIGNED_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_LOAD8_ALIGNED_INLINE _GL_INLINE
macro_rules! _GL_STDC_LOAD8_ALIGNED_INLINE { () => { _GL_INLINE }; }

/// C 168: `# define _GL_STDC_LOAD8_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_LOAD8_INLINE _GL_INLINE
macro_rules! _GL_STDC_LOAD8_INLINE { () => { _GL_INLINE }; }

/// C 162: `# define _GL_STDC_MEMREVERSE8U_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_MEMREVERSE8U_INLINE _GL_INLINE
macro_rules! _GL_STDC_MEMREVERSE8U_INLINE { () => { _GL_INLINE }; }

/// C 159: `# define _GL_STDC_MEMREVERSE8_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_MEMREVERSE8_INLINE _GL_INLINE
macro_rules! _GL_STDC_MEMREVERSE8_INLINE { () => { _GL_INLINE }; }

/// C 153: `# define _GL_STDC_ROTATE_LEFT_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_ROTATE_LEFT_INLINE _GL_INLINE
macro_rules! _GL_STDC_ROTATE_LEFT_INLINE { () => { _GL_INLINE }; }

/// C 156: `# define _GL_STDC_ROTATE_RIGHT_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_ROTATE_RIGHT_INLINE _GL_INLINE
macro_rules! _GL_STDC_ROTATE_RIGHT_INLINE { () => { _GL_INLINE }; }

/// C 171: `# define _GL_STDC_STORE8_ALIGNED_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_STORE8_ALIGNED_INLINE _GL_INLINE
macro_rules! _GL_STDC_STORE8_ALIGNED_INLINE { () => { _GL_INLINE }; }

/// C 174: `# define _GL_STDC_STORE8_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_STORE8_INLINE _GL_INLINE
macro_rules! _GL_STDC_STORE8_INLINE { () => { _GL_INLINE }; }

/// C 120: `# define _GL_STDC_TRAILING_ONES_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_TRAILING_ONES_INLINE _GL_INLINE
macro_rules! _GL_STDC_TRAILING_ONES_INLINE { () => { _GL_INLINE }; }

/// C 117: `# define _GL_STDC_TRAILING_ZEROS_INLINE _GL_INLINE` - a qualifier spelling, so the port writes
/// `#[inline(always)]` on the family members that use it (the `_GL_TIMESPEC_INLINE` reading), and
/// the macro is carried as the name whose body is that qualifier.
/// C: # define _GL_STDC_TRAILING_ZEROS_INLINE _GL_INLINE
macro_rules! _GL_STDC_TRAILING_ZEROS_INLINE { () => { _GL_INLINE }; }

/// C 312: `# define GNULIB_defined_clz_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_clz_functions 1
pub const GNULIB_defined_clz_functions: c_int = 1;

/// C 494: `# define GNULIB_defined_ctz_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_ctz_functions 1
pub const GNULIB_defined_ctz_functions: c_int = 1;

/// C 960: `# define GNULIB_defined_popcount_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_popcount_functions 1
pub const GNULIB_defined_popcount_functions: c_int = 1;

/// C 1257: `# define GNULIB_defined_stdc_bit_ceil_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_bit_ceil_functions 1
pub const GNULIB_defined_stdc_bit_ceil_functions: c_int = 1;

/// C 1200: `# define GNULIB_defined_stdc_bit_floor_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_bit_floor_functions 1
pub const GNULIB_defined_stdc_bit_floor_functions: c_int = 1;

/// C 1148: `# define GNULIB_defined_stdc_bit_width_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_bit_width_functions 1
pub const GNULIB_defined_stdc_bit_width_functions: c_int = 1;

/// C 995: `# define GNULIB_defined_stdc_count_ones_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_count_ones_functions 1
pub const GNULIB_defined_stdc_count_ones_functions: c_int = 1;

/// C 1044: `# define GNULIB_defined_stdc_count_zeros_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_count_zeros_functions 1
pub const GNULIB_defined_stdc_count_zeros_functions: c_int = 1;

/// C 696: `# define GNULIB_defined_stdc_first_leading_one_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_first_leading_one_functions 1
pub const GNULIB_defined_stdc_first_leading_one_functions: c_int = 1;

/// C 637: `# define GNULIB_defined_stdc_first_leading_zero_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_first_leading_zero_functions 1
pub const GNULIB_defined_stdc_first_leading_zero_functions: c_int = 1;

/// C 814: `# define GNULIB_defined_stdc_first_trailing_one_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_first_trailing_one_functions 1
pub const GNULIB_defined_stdc_first_trailing_one_functions: c_int = 1;

/// C 755: `# define GNULIB_defined_stdc_first_trailing_zero_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_first_trailing_zero_functions 1
pub const GNULIB_defined_stdc_first_trailing_zero_functions: c_int = 1;

/// C 1098: `# define GNULIB_defined_stdc_has_single_bit_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_has_single_bit_functions 1
pub const GNULIB_defined_stdc_has_single_bit_functions: c_int = 1;

/// C 396: `# define GNULIB_defined_stdc_leading_ones_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_leading_ones_functions 1
pub const GNULIB_defined_stdc_leading_ones_functions: c_int = 1;

/// C 347: `# define GNULIB_defined_stdc_leading_zeros_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_leading_zeros_functions 1
pub const GNULIB_defined_stdc_leading_zeros_functions: c_int = 1;

/// C 1934: `# define GNULIB_defined_stdc_load8_aligned_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_load8_aligned_functions 1
pub const GNULIB_defined_stdc_load8_aligned_functions: c_int = 1;

/// C 1775: `# define GNULIB_defined_stdc_load8_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_load8_functions 1
pub const GNULIB_defined_stdc_load8_functions: c_int = 1;

/// C 1436: `# define GNULIB_defined_stdc_memreverse8 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_memreverse8 1
pub const GNULIB_defined_stdc_memreverse8: c_int = 1;

/// C 1503: `# define GNULIB_defined_stdc_memreverse8u_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_memreverse8u_functions 1
pub const GNULIB_defined_stdc_memreverse8u_functions: c_int = 1;

/// C 1326: `# define GNULIB_defined_stdc_rotate_left_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_rotate_left_functions 1
pub const GNULIB_defined_stdc_rotate_left_functions: c_int = 1;

/// C 1392: `# define GNULIB_defined_stdc_rotate_right_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_rotate_right_functions 1
pub const GNULIB_defined_stdc_rotate_right_functions: c_int = 1;

/// C 2211: `# define GNULIB_defined_stdc_store8_aligned_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_store8_aligned_functions 1
pub const GNULIB_defined_stdc_store8_aligned_functions: c_int = 1;

/// C 2064: `# define GNULIB_defined_stdc_store8_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_store8_functions 1
pub const GNULIB_defined_stdc_store8_functions: c_int = 1;

/// C 578: `# define GNULIB_defined_stdc_trailing_ones_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_trailing_ones_functions 1
pub const GNULIB_defined_stdc_trailing_ones_functions: c_int = 1;

/// C 529: `# define GNULIB_defined_stdc_trailing_zeros_functions 1` - the "this family is now defined" flag C's `# if !…` guards read.
/// C: #  define GNULIB_defined_stdc_trailing_zeros_functions 1
pub const GNULIB_defined_stdc_trailing_zeros_functions: c_int = 1;

/* ---- the functions this mirror owns (integrator ruling): the ones `stdbit.rs` used to carry --- */

/* stdbit.in.h 236-248: the live `_GL_STDBIT_HAS_BUILTIN_CLZ` arm (`__builtin_clz*` is
   undefined for 0, hence C's `n ? … : 8 * sizeof n`), returning `int` as C declares.  */
#[inline(always)]
pub fn _gl_stdbit_clz (n: c_uint) -> c_int
{
  return if n != 0 { n.leading_zeros () as c_int }
         else { (8 * std::mem::size_of_val (&n)) as c_int };
}
#[inline(always)]
pub fn _gl_stdbit_clzl (n: c_ulong) -> c_int
{
  return if n != 0 { n.leading_zeros () as c_int }
         else { (8 * std::mem::size_of_val (&n)) as c_int };
}
#[inline(always)]
pub fn _gl_stdbit_clzll (n: c_ulonglong) -> c_int
{
  return if n != 0 { n.leading_zeros () as c_int }
         else { (8 * std::mem::size_of_val (&n)) as c_int };
}

/* stdc_leading_zeros_ui … _ull (stdbit.in.h 317-345); each returns `_gl_stdbit_clz*`'s
   `int` as the `unsigned int` the declaration names.  */
#[inline(always)]
pub fn stdc_leading_zeros_ui (n: c_uint) -> c_uint { _gl_stdbit_clz (n) as c_uint }
#[inline(always)]
pub fn stdc_leading_zeros_uc (n: c_uchar) -> c_uint
{
  return stdc_leading_zeros_ui (n as c_uint)
         - 8 * (std::mem::size_of_val (&0u32) - std::mem::size_of_val (&n)) as c_uint;
}
#[inline(always)]
pub fn stdc_leading_zeros_us (n: c_ushort) -> c_uint
{
  return stdc_leading_zeros_ui (n as c_uint)
         - 8 * (std::mem::size_of_val (&0u32) - std::mem::size_of_val (&n)) as c_uint;
}
#[inline(always)]
pub fn stdc_leading_zeros_ul (n: c_ulong) -> c_uint { _gl_stdbit_clzl (n) as c_uint }
#[inline(always)]
pub fn stdc_leading_zeros_ull (n: c_ulonglong) -> c_uint { _gl_stdbit_clzll (n) as c_uint }

/* stdbit.in.h 427-439: the live `_GL_STDBIT_HAS_BUILTIN_CTZ` arm, `int` as declared.  */
#[inline(always)]
pub fn _gl_stdbit_ctz (n: c_uint) -> c_int
{
  return if n != 0 { n.trailing_zeros () as c_int }
         else { (8 * std::mem::size_of_val (&n)) as c_int };
}
#[inline(always)]
pub fn _gl_stdbit_ctzl (n: c_ulong) -> c_int
{
  return if n != 0 { n.trailing_zeros () as c_int }
         else { (8 * std::mem::size_of_val (&n)) as c_int };
}
#[inline(always)]
pub fn _gl_stdbit_ctzll (n: c_ulonglong) -> c_int
{
  return if n != 0 { n.trailing_zeros () as c_int }
         else { (8 * std::mem::size_of_val (&n)) as c_int };
}

/* stdc_trailing_zeros_ui … _ull (stdbit.in.h 499-527).  The narrow variants OR in the
   promoted type's all-ones mask (`(1 + (unsigned char) -1)`) so that a zero argument
   takes the `8 * sizeof n` arm of the *promoted* width, exactly as C writes it.  */
#[inline(always)]
pub fn stdc_trailing_zeros_ui (n: c_uint) -> c_uint { _gl_stdbit_ctz (n) as c_uint }
#[inline(always)]
pub fn stdc_trailing_zeros_uc (n: c_uchar) -> c_uint
{
  return stdc_trailing_zeros_ui (n as c_uint | (1 + c_uchar::MAX as c_uint));
}
#[inline(always)]
pub fn stdc_trailing_zeros_us (n: c_ushort) -> c_uint
{
  return stdc_trailing_zeros_ui (n as c_uint | (1 + c_ushort::MAX as c_uint));
}
#[inline(always)]
pub fn stdc_trailing_zeros_ul (n: c_ulong) -> c_uint { _gl_stdbit_ctzl (n) as c_uint }
#[inline(always)]
pub fn stdc_trailing_zeros_ull (n: c_ulonglong) -> c_uint { _gl_stdbit_ctzll (n) as c_uint }

/* _gl_stdbit_popcount{,l,ll} (stdbit.in.h 890-960): **macros**, which is what C makes them on
   this target - C 839-841's live `#  ifdef _GL_STDBIT_HAS_BUILTIN_POPCOUNT` arm defines each as
   the compiler's `__builtin_popcount*`, and the `#  else` arm (C 842-960: the hand-written
   `_gl_stdbit_popcount_wide` and the MSVC `__cpuid`/`__popcnt` block) is dead text there (P-66
   rule 2).  The port spells the live arm `count_ones ()`, the same spelling `_gl_stdbit_clz`
   above gives `leading_zeros ()`; the cast is C's own conversion to the builtin's parameter type
   (`unsigned int` / `unsigned long int` / `unsigned long long int`), and `count_ones ()`'s `u32`
   is the `int` C's builtin returns.
   C's identity fixes the shape: a `fn` here would be a *definition* this target has not got, and
   the three `stdc_count_ones_*` bodies below are C's own `_gl_stdbit_popcount (n)` calls, so the
   call sites keep the macro's `!`.  (The integrator's 3.185 repair used `fn`s to unblock the
   shared gate; this is the lane's intended definition, restored.)  */
macro_rules! _gl_stdbit_popcount { ($n:expr) => { ($n as c_uint).count_ones () }; }
macro_rules! _gl_stdbit_popcountl { ($n:expr) => { ($n as c_ulong).count_ones () }; }
macro_rules! _gl_stdbit_popcountll { ($n:expr) => { ($n as c_ulonglong).count_ones () }; }

/* stdc_count_ones_ui … _ull (stdbit.in.h 965-992); each calls `_gl_stdbit_popcount*`,
   which is `__builtin_popcount*` - the narrow variants need no adjustment because
   promoting a narrow value cannot add a one bit.  */
#[inline(always)]
pub fn stdc_count_ones_ui (n: c_uint) -> c_uint { _gl_stdbit_popcount! (n) as c_uint }
#[inline(always)]
pub fn stdc_count_ones_uc (n: c_uchar) -> c_uint { stdc_count_ones_ui (n as c_uint) }
#[inline(always)]
pub fn stdc_count_ones_us (n: c_ushort) -> c_uint { stdc_count_ones_ui (n as c_uint) }
#[inline(always)]
pub fn stdc_count_ones_ul (n: c_ulong) -> c_uint { _gl_stdbit_popcountl! (n) as c_uint }
#[inline(always)]
pub fn stdc_count_ones_ull (n: c_ulonglong) -> c_uint { _gl_stdbit_popcountll! (n) as c_uint }

/* stdc_bit_width_ui … _ull (stdbit.in.h 1119-1147); each is `8 * sizeof n` minus the
   leading zeros of that width, i.e. C's `8 * sizeof n - stdc_leading_zeros_* (n)`.  */
#[inline(always)]
pub fn stdc_bit_width_uc (n: c_uchar) -> c_uint
{
  return (8 * std::mem::size_of_val (&n)) as c_uint - stdc_leading_zeros_uc (n);
}
#[inline(always)]
pub fn stdc_bit_width_us (n: c_ushort) -> c_uint
{
  return (8 * std::mem::size_of_val (&n)) as c_uint - stdc_leading_zeros_us (n);
}
#[inline(always)]
pub fn stdc_bit_width_ui (n: c_uint) -> c_uint
{
  return (8 * std::mem::size_of_val (&n)) as c_uint - stdc_leading_zeros_ui (n);
}
#[inline(always)]
pub fn stdc_bit_width_ul (n: c_ulong) -> c_uint
{
  return (8 * std::mem::size_of_val (&n)) as c_uint - stdc_leading_zeros_ul (n);
}
#[inline(always)]
pub fn stdc_bit_width_ull (n: c_ulonglong) -> c_uint
{
  return (8 * std::mem::size_of_val (&n)) as c_uint - stdc_leading_zeros_ull (n);
}

/* stdc_memreverse8u8/16/32/64 (stdbit.in.h 1454-1502): C's `_GL_STDBIT_HAS_BUILTIN_BSWAP*`
   arms, and C's own note that the ISO C uintN_t spelling is generalised here to
   uint_leastN_t (same type on this target, stdint.in.rs).  */
#[inline(always)]
pub fn stdc_memreverse8u8 (value: uint_least8_t) -> uint_least8_t
{
  return value;
}
#[inline(always)]
pub fn stdc_memreverse8u16 (value: uint_least16_t) -> uint_least16_t
{
  return value.swap_bytes ();
}
#[inline(always)]
pub fn stdc_memreverse8u32 (value: uint_least32_t) -> uint_least32_t
{
  return value.swap_bytes ();
}
#[inline(always)]
pub fn stdc_memreverse8u64 (value: uint_least64_t) -> uint_least64_t
{
  return value.swap_bytes ();
}

/* ---- C 366-406, 548-585, 1015-1043: the three families whose member is the complement's --------- */

/// C 366-397: `stdc_leading_ones_*` - C's `stdc_leading_zeros_* (~n)`.  Rust's `!n` on a `c_uchar`
/// stays 8-bit where C's `~n` promotes and truncates back, so the two agree bit for bit (the probe
/// asserts all five against `u*::leading_ones`).
/// C: return stdc_leading_zeros_uc (~n);
#[inline(always)]
pub fn stdc_leading_ones_uc (n: c_uchar) -> c_uint { return stdc_leading_zeros_uc (!n); }
/// C: return stdc_leading_zeros_us (~n);
#[inline(always)]
pub fn stdc_leading_ones_us (n: c_ushort) -> c_uint { return stdc_leading_zeros_us (!n); }
/// C: return stdc_leading_zeros_ui (~n);
#[inline(always)]
pub fn stdc_leading_ones_ui (n: c_uint) -> c_uint { return stdc_leading_zeros_ui (!n); }
/// C: return stdc_leading_zeros_ul (~n);
#[inline(always)]
pub fn stdc_leading_ones_ul (n: c_ulong) -> c_uint { return stdc_leading_zeros_ul (!n); }
/// C: return stdc_leading_zeros_ull (~n);
#[inline(always)]
pub fn stdc_leading_ones_ull (n: c_ulonglong) -> c_uint { return stdc_leading_zeros_ull (!n); }

/// C 548-579: `stdc_trailing_ones_*` - the trailing *zeros* of the complement.
/// C: return stdc_trailing_zeros_uc (~n);
#[inline(always)]
pub fn stdc_trailing_ones_uc (n: c_uchar) -> c_uint { return stdc_trailing_zeros_uc (!n); }
/// C: return stdc_trailing_zeros_us (~n);
#[inline(always)]
pub fn stdc_trailing_ones_us (n: c_ushort) -> c_uint { return stdc_trailing_zeros_us (!n); }
/// C: return stdc_trailing_zeros_ui (~n);
#[inline(always)]
pub fn stdc_trailing_ones_ui (n: c_uint) -> c_uint { return stdc_trailing_zeros_ui (!n); }
/// C: return stdc_trailing_zeros_ul (~n);
#[inline(always)]
pub fn stdc_trailing_ones_ul (n: c_ulong) -> c_uint { return stdc_trailing_zeros_ul (!n); }
/// C: return stdc_trailing_zeros_ull (~n);
#[inline(always)]
pub fn stdc_trailing_ones_ull (n: c_ulonglong) -> c_uint { return stdc_trailing_zeros_ull (!n); }

/// C 1015-1043: `stdc_count_zeros_*` - the count of ones in the complement.
/// C: return stdc_count_ones_uc (~n);
#[inline(always)]
pub fn stdc_count_zeros_uc (n: c_uchar) -> c_uint { return stdc_count_ones_uc (!n); }
/// C: return stdc_count_ones_us (~n);
#[inline(always)]
pub fn stdc_count_zeros_us (n: c_ushort) -> c_uint { return stdc_count_ones_us (!n); }
/// C: return stdc_count_ones_ui (~n);
#[inline(always)]
pub fn stdc_count_zeros_ui (n: c_uint) -> c_uint { return stdc_count_ones_ui (!n); }
/// C: return stdc_count_ones_ul (~n);
#[inline(always)]
pub fn stdc_count_zeros_ul (n: c_ulong) -> c_uint { return stdc_count_ones_ul (!n); }
/// C: return stdc_count_ones_ull (~n);
#[inline(always)]
pub fn stdc_count_zeros_ull (n: c_ulonglong) -> c_uint { return stdc_count_ones_ull (!n); }

/// C 399-405 / 581-587 / 1045-1051: the three dispatchers, in the idiom the head note records
/// (`sizeof 0u` -> `std::mem::size_of_val (&0u32)`, so C's literal survives the token check).
/// C: # define stdc_leading_ones(n) \
/// C:   (sizeof (n) == 1 ? stdc_leading_ones_uc (n)	\
/// C:    : sizeof (n) == sizeof (unsigned short int) ? stdc_leading_ones_us (n) \
/// C:    : sizeof (n) == sizeof 0u ? stdc_leading_ones_ui (n) \
/// C:    : sizeof (n) == sizeof 0ul ? stdc_leading_ones_ul (n) \
/// C:    : stdc_leading_ones_ull (n))
macro_rules! stdc_leading_ones {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_leading_ones_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_leading_ones_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
           { stdc_leading_ones_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
           { stdc_leading_ones_ul ($n as c_ulong) }
         else
           { stdc_leading_ones_ull ($n as c_ulonglong) })
    };
}
/// C 581-587.
macro_rules! stdc_trailing_ones {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_trailing_ones_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_trailing_ones_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
           { stdc_trailing_ones_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
           { stdc_trailing_ones_ul ($n as c_ulong) }
         else
           { stdc_trailing_ones_ull ($n as c_ulonglong) })
    };
}
/// C 1045-1051.
macro_rules! stdc_count_zeros {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_count_zeros_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_count_zeros_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
           { stdc_count_zeros_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
           { stdc_count_zeros_ul ($n as c_ulong) }
         else
           { stdc_count_zeros_ull ($n as c_ulonglong) })
    };
}

/// C 915-916: `/* 1 if supported, -1 if not, 0 if unknown.  */` / `extern signed char
/// _gl_stdbit_popcount_support;` - the one file-scope object, which `src/`'s units and the
/// popcount helpers read.
/// C: extern signed char _gl_stdbit_popcount_support;
/// The type is fully qualified the way `lib/include/stdint.in.rs` 57 spells `int8_t`: the
/// crate-wide alias set comes from the foundation mirrors' `use std::ffi::{…}` lists, which do not
/// name `c_schar`, so a bare `c_schar` here would be `E0425` in the single-crate layout probe
/// (measured), while `core::ffi::c_schar` is the same type and needs no scope.
unsafe extern "C" {
  pub static mut _gl_stdbit_popcount_support: core::ffi::c_schar;
}

/* ---- what is *not* here: two dead arms C compiles only off this target ---------------------
   `_gl_stdbit_popcount_wide` (C 848-905) and the `#  ifdef _MSC_VER` block around it
   (`__popcnt64` C 908-918, `_gl_stdbit_popcount_supported` C 919-933) live in the `#  else` arm
   of C 836-838's `#  ifdef _GL_STDBIT_HAS_BUILTIN_POPCOUNT`, which the frozen compiler does not
   take (it has the builtin), and their text names MSVC intrinsics `__cpuid`/`__popcnt` that have
   no Rust spelling at all.  P-66 rule 2 asks for the boundary and the line numbers rather than a
   transcription, so the three names are recorded in `docs/ports/excluded-macros.txt`
   (`kind=deferred`) and the mirror carries nothing for them.  The same rule covers the two
   hand-written mask arms of `stdc_memreverse8u16/32/64` (C 1447-1451, 1453-1461, 1463-1476);
   those three *functions* are live and landed above, and the dead arm's tokens are the three
   `docs/ports/macro-body-notes.tsv` rows the ledger lists.  */

/* ---- the capability macros the families below ask for --------------------------------------
   Each is an `# if` arm over the *compiler*'s builtins (C 76-90, 228-233, 419-424, 836-841) and
   the frozen compiler defines all six, so the port carries the frozen value with the name.
   They are branch macros in C, so only their presence is compared (extract-abi's several-
   definition rule); what they *stand for* is `docs/ports/include-stdbit.in.md`'s arm table.  */
/// C 77-83: `#  define _GL_STDBIT_HAS_BUILTIN_BSWAP16 1`.
/// C: #  define _GL_STDBIT_HAS_BUILTIN_BSWAP16 1
pub const _GL_STDBIT_HAS_BUILTIN_BSWAP16: c_int = 1;
/// C 84-90: `#  define _GL_STDBIT_HAS_BUILTIN_BSWAP32 1` (and `…BSWAP64` on the next line).
/// C: #  define _GL_STDBIT_HAS_BUILTIN_BSWAP32 1
/// C: #  define _GL_STDBIT_HAS_BUILTIN_BSWAP64 1
pub const _GL_STDBIT_HAS_BUILTIN_BSWAP32: c_int = 1;
pub const _GL_STDBIT_HAS_BUILTIN_BSWAP64: c_int = 1;
/// C 228-233: `#  define _GL_STDBIT_HAS_BUILTIN_CLZ true`.
/// C: #  define _GL_STDBIT_HAS_BUILTIN_CLZ true
pub const _GL_STDBIT_HAS_BUILTIN_CLZ: bool = true;
/// C 419-424: `#  define _GL_STDBIT_HAS_BUILTIN_CTZ true`.
/// C: #  define _GL_STDBIT_HAS_BUILTIN_CTZ true
pub const _GL_STDBIT_HAS_BUILTIN_CTZ: bool = true;
/// C 836-841: `#  define _GL_STDBIT_HAS_BUILTIN_POPCOUNT true`.
/// C: #  define _GL_STDBIT_HAS_BUILTIN_POPCOUNT true
pub const _GL_STDBIT_HAS_BUILTIN_POPCOUNT: bool = true;

/* C 60-64's `_GL_STDBIT_UINT_FASTN` is a *type* macro; the port keeps C's spelling of it as the
   nullary `macro_rules!` above, but Rust cannot use a macro in type position, so the three names
   the load8/store8 bodies need are also real aliases of the frozen compiler's own answer,
   measured with that compiler:
     clang -std=gnu23 -fsyntax-only -Xclang -ast-dump  on
       `__UINT_FAST16_TYPE__ a16; __UINT_FAST32_TYPE__ a32; __UINT_FAST64_TYPE__ a64;`
     -> `a16 'unsigned short'`, `a32 'unsigned int'`, `a64 'unsigned long long'`.  */
pub type _GL_STDBIT_UINT_FAST16 = c_ushort;
pub type _GL_STDBIT_UINT_FAST32 = c_uint;
pub type _GL_STDBIT_UINT_FAST64 = c_ulonglong;

/* ---- C 599-820: the four `first_*` families ------------------------------------------------
   Each member is C's own three lines - `count = stdc_<base>_<t> (n)`, `bits = 8 * sizeof n`,
   `return count % bits + (count < bits)` - with `sizeof n` spelled `std::mem::size_of_val (&n)`
   (the parameter's own width, exactly what C measures) and the comparison's `bool` cast to the
   `unsigned int` C's `+` promotes it to.  `count % bits` cannot divide by zero: `bits` is the
   parameter's own 8, 16, 32 or 64.  */
/// C 598: `stdc_first_leading_zero_uc (unsigned char n)`.
#[inline(always)]
pub fn stdc_first_leading_zero_uc (n: c_uchar) -> c_uint
{
  let count: c_uint = stdc_leading_ones_uc (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 598: `stdc_first_leading_zero_us (unsigned short int n)`.
#[inline(always)]
pub fn stdc_first_leading_zero_us (n: c_ushort) -> c_uint
{
  let count: c_uint = stdc_leading_ones_us (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 598: `stdc_first_leading_zero_ui (unsigned int n)`.
#[inline(always)]
pub fn stdc_first_leading_zero_ui (n: c_uint) -> c_uint
{
  let count: c_uint = stdc_leading_ones_ui (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 598: `stdc_first_leading_zero_ul (unsigned long int n)`.
#[inline(always)]
pub fn stdc_first_leading_zero_ul (n: c_ulong) -> c_uint
{
  let count: c_uint = stdc_leading_ones_ul (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 598: `stdc_first_leading_zero_ull (unsigned long long int n)`.
#[inline(always)]
pub fn stdc_first_leading_zero_ull (n: c_ulonglong) -> c_uint
{
  let count: c_uint = stdc_leading_ones_ull (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 0: `#  define stdc_first_leading_zero(n) (sizeof (n) == 1 ? ... : ...)` - C's five-way `?:` chain,
/// which Rust spells `if`/`else` (`include/stdbit.rs`'s shape for the four families it carries).
/// `sizeof 0u`/`sizeof 0ul` is `std::mem::size_of_val (&0u32)`/`(&0u64)`, the idiom that keeps
/// C's own literal `0`.
macro_rules! stdc_first_leading_zero {
  ($n:expr) => {
    (if std::mem::size_of_val (&$n) == 1
       { stdc_first_leading_zero_uc ($n as c_uchar) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
       { stdc_first_leading_zero_us ($n as c_ushort) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
       { stdc_first_leading_zero_ui ($n as c_uint) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
       { stdc_first_leading_zero_ul ($n as c_ulong) }
     else
       { stdc_first_leading_zero_ull ($n as c_ulonglong) })
  };
}

/// C 657: `stdc_first_leading_one_uc (unsigned char n)`.
#[inline(always)]
pub fn stdc_first_leading_one_uc (n: c_uchar) -> c_uint
{
  let count: c_uint = stdc_leading_zeros_uc (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 657: `stdc_first_leading_one_us (unsigned short int n)`.
#[inline(always)]
pub fn stdc_first_leading_one_us (n: c_ushort) -> c_uint
{
  let count: c_uint = stdc_leading_zeros_us (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 657: `stdc_first_leading_one_ui (unsigned int n)`.
#[inline(always)]
pub fn stdc_first_leading_one_ui (n: c_uint) -> c_uint
{
  let count: c_uint = stdc_leading_zeros_ui (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 657: `stdc_first_leading_one_ul (unsigned long int n)`.
#[inline(always)]
pub fn stdc_first_leading_one_ul (n: c_ulong) -> c_uint
{
  let count: c_uint = stdc_leading_zeros_ul (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 657: `stdc_first_leading_one_ull (unsigned long long int n)`.
#[inline(always)]
pub fn stdc_first_leading_one_ull (n: c_ulonglong) -> c_uint
{
  let count: c_uint = stdc_leading_zeros_ull (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 0: `#  define stdc_first_leading_one(n) (sizeof (n) == 1 ? ... : ...)` - C's five-way `?:` chain,
/// which Rust spells `if`/`else` (`include/stdbit.rs`'s shape for the four families it carries).
/// `sizeof 0u`/`sizeof 0ul` is `std::mem::size_of_val (&0u32)`/`(&0u64)`, the idiom that keeps
/// C's own literal `0`.
macro_rules! stdc_first_leading_one {
  ($n:expr) => {
    (if std::mem::size_of_val (&$n) == 1
       { stdc_first_leading_one_uc ($n as c_uchar) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
       { stdc_first_leading_one_us ($n as c_ushort) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
       { stdc_first_leading_one_ui ($n as c_uint) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
       { stdc_first_leading_one_ul ($n as c_ulong) }
     else
       { stdc_first_leading_one_ull ($n as c_ulonglong) })
  };
}

/// C 716: `stdc_first_trailing_zero_uc (unsigned char n)`.
#[inline(always)]
pub fn stdc_first_trailing_zero_uc (n: c_uchar) -> c_uint
{
  let count: c_uint = stdc_trailing_ones_uc (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 716: `stdc_first_trailing_zero_us (unsigned short int n)`.
#[inline(always)]
pub fn stdc_first_trailing_zero_us (n: c_ushort) -> c_uint
{
  let count: c_uint = stdc_trailing_ones_us (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 716: `stdc_first_trailing_zero_ui (unsigned int n)`.
#[inline(always)]
pub fn stdc_first_trailing_zero_ui (n: c_uint) -> c_uint
{
  let count: c_uint = stdc_trailing_ones_ui (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 716: `stdc_first_trailing_zero_ul (unsigned long int n)`.
#[inline(always)]
pub fn stdc_first_trailing_zero_ul (n: c_ulong) -> c_uint
{
  let count: c_uint = stdc_trailing_ones_ul (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 716: `stdc_first_trailing_zero_ull (unsigned long long int n)`.
#[inline(always)]
pub fn stdc_first_trailing_zero_ull (n: c_ulonglong) -> c_uint
{
  let count: c_uint = stdc_trailing_ones_ull (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 0: `#  define stdc_first_trailing_zero(n) (sizeof (n) == 1 ? ... : ...)` - C's five-way `?:` chain,
/// which Rust spells `if`/`else` (`include/stdbit.rs`'s shape for the four families it carries).
/// `sizeof 0u`/`sizeof 0ul` is `std::mem::size_of_val (&0u32)`/`(&0u64)`, the idiom that keeps
/// C's own literal `0`.
macro_rules! stdc_first_trailing_zero {
  ($n:expr) => {
    (if std::mem::size_of_val (&$n) == 1
       { stdc_first_trailing_zero_uc ($n as c_uchar) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
       { stdc_first_trailing_zero_us ($n as c_ushort) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
       { stdc_first_trailing_zero_ui ($n as c_uint) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
       { stdc_first_trailing_zero_ul ($n as c_ulong) }
     else
       { stdc_first_trailing_zero_ull ($n as c_ulonglong) })
  };
}

/// C 775: `stdc_first_trailing_one_uc (unsigned char n)`.
#[inline(always)]
pub fn stdc_first_trailing_one_uc (n: c_uchar) -> c_uint
{
  let count: c_uint = stdc_trailing_zeros_uc (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 775: `stdc_first_trailing_one_us (unsigned short int n)`.
#[inline(always)]
pub fn stdc_first_trailing_one_us (n: c_ushort) -> c_uint
{
  let count: c_uint = stdc_trailing_zeros_us (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 775: `stdc_first_trailing_one_ui (unsigned int n)`.
#[inline(always)]
pub fn stdc_first_trailing_one_ui (n: c_uint) -> c_uint
{
  let count: c_uint = stdc_trailing_zeros_ui (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 775: `stdc_first_trailing_one_ul (unsigned long int n)`.
#[inline(always)]
pub fn stdc_first_trailing_one_ul (n: c_ulong) -> c_uint
{
  let count: c_uint = stdc_trailing_zeros_ul (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 775: `stdc_first_trailing_one_ull (unsigned long long int n)`.
#[inline(always)]
pub fn stdc_first_trailing_one_ull (n: c_ulonglong) -> c_uint
{
  let count: c_uint = stdc_trailing_zeros_ull (n);
  let bits: c_uint = (8 * std::mem::size_of_val (&n)) as c_uint;
  return count % bits + (count < bits) as c_uint;
}

/// C 0: `#  define stdc_first_trailing_one(n) (sizeof (n) == 1 ? ... : ...)` - C's five-way `?:` chain,
/// which Rust spells `if`/`else` (`include/stdbit.rs`'s shape for the four families it carries).
/// `sizeof 0u`/`sizeof 0ul` is `std::mem::size_of_val (&0u32)`/`(&0u64)`, the idiom that keeps
/// C's own literal `0`.
macro_rules! stdc_first_trailing_one {
  ($n:expr) => {
    (if std::mem::size_of_val (&$n) == 1
       { stdc_first_trailing_one_uc ($n as c_uchar) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
       { stdc_first_trailing_one_us ($n as c_ushort) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
       { stdc_first_trailing_one_ui ($n as c_uint) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
       { stdc_first_trailing_one_ul ($n as c_ulong) }
     else
       { stdc_first_trailing_one_ull ($n as c_ulonglong) })
  };
}

/* ---- C 1063-1110: `stdc_has_single_bit_*` --------------------------------------------------
   C's `n_1 = n - 1, nx = n_1 ^ n; return n_1 < nx;`.  The subtraction has the *unsigned* wrap
   C's type gives it (C's `stdc_has_single_bit_uc (0)` is `255 < 255`, false), and Rust's
   checked `-` would panic where C wraps, so the port writes `wrapping_sub`.  */
/// C 1064: `stdc_has_single_bit_uc (unsigned char n)`.
#[inline(always)]
pub fn stdc_has_single_bit_uc (n: c_uchar) -> bool
{
  let n_1: c_uchar = n.wrapping_sub (1);
  let nx: c_uchar = n_1 ^ n;
  return n_1 < nx;
}

/// C 1064: `stdc_has_single_bit_us (unsigned short int n)`.
#[inline(always)]
pub fn stdc_has_single_bit_us (n: c_ushort) -> bool
{
  let n_1: c_ushort = n.wrapping_sub (1);
  let nx: c_ushort = n_1 ^ n;
  return n_1 < nx;
}

/// C 1064: `stdc_has_single_bit_ui (unsigned int n)`.
#[inline(always)]
pub fn stdc_has_single_bit_ui (n: c_uint) -> bool
{
  let n_1: c_uint = n.wrapping_sub (1);
  let nx: c_uint = n_1 ^ n;
  return n_1 < nx;
}

/// C 1064: `stdc_has_single_bit_ul (unsigned long int n)`.
#[inline(always)]
pub fn stdc_has_single_bit_ul (n: c_ulong) -> bool
{
  let n_1: c_ulong = n.wrapping_sub (1);
  let nx: c_ulong = n_1 ^ n;
  return n_1 < nx;
}

/// C 1064: `stdc_has_single_bit_ull (unsigned long long int n)`.
#[inline(always)]
pub fn stdc_has_single_bit_ull (n: c_ulonglong) -> bool
{
  let n_1: c_ulonglong = n.wrapping_sub (1);
  let nx: c_ulonglong = n_1 ^ n;
  return n_1 < nx;
}

/// C 0: `#  define stdc_has_single_bit(n) ((bool) (sizeof (n) == 1 ? … : …))`.  C's own
/// `(bool)` cast is what the cast-free body already is: every member returns `bool`.
macro_rules! stdc_has_single_bit {
  ($n:expr) => {
    (if std::mem::size_of_val (&$n) == 1
       { stdc_has_single_bit_uc ($n as c_uchar) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
       { stdc_has_single_bit_us ($n as c_ushort) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
       { stdc_has_single_bit_ui ($n as c_uint) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
       { stdc_has_single_bit_ul ($n as c_ulong) }
     else
       { stdc_has_single_bit_ull ($n as c_ulonglong) })
  };
}

/* ---- C 1170-1270: `stdc_bit_floor_*` and `stdc_bit_ceil_*` ---------------------------------
   C's `return n ? 1u << (stdc_bit_width_uc (n) - 1) : 0;` and
   `return n <= 1 ? 1 : 2u << (stdc_bit_width_uc (n - 1) - 1);` (C 1176-1210, 1227-1267).  The
   literal takes the member's own width (`1u`/`1ul`/`1ull` in C is a `u32`/`u64` shift there, and
   the result is truncated to the return type - which is exactly what a `u8`/`u16` shift does);
   only the shift *amount* is checked by Rust, and it is `0 .. width-1`, so nothing can panic.
   `n - 1` is `wrapping_sub` because C's subtraction is unsigned.  */
/// C 1171: `stdc_bit_floor_uc (unsigned char n)`.
#[inline(always)]
pub fn stdc_bit_floor_uc (n: c_uchar) -> c_uchar
{
  return if n != 0 { 1u8 << (stdc_bit_width_uc (n) - 1) } else { 0 };
}

/// C 1171: `stdc_bit_floor_us (unsigned short int n)`.
#[inline(always)]
pub fn stdc_bit_floor_us (n: c_ushort) -> c_ushort
{
  return if n != 0 { 1u16 << (stdc_bit_width_us (n) - 1) } else { 0 };
}

/// C 1171: `stdc_bit_floor_ui (unsigned int n)`.
#[inline(always)]
pub fn stdc_bit_floor_ui (n: c_uint) -> c_uint
{
  return if n != 0 { 1u32 << (stdc_bit_width_ui (n) - 1) } else { 0 };
}

/// C 1171: `stdc_bit_floor_ul (unsigned long int n)`.
#[inline(always)]
pub fn stdc_bit_floor_ul (n: c_ulong) -> c_ulong
{
  return if n != 0 { 1u64 << (stdc_bit_width_ul (n) - 1) } else { 0 };
}

/// C 1171: `stdc_bit_floor_ull (unsigned long long int n)`.
#[inline(always)]
pub fn stdc_bit_floor_ull (n: c_ulonglong) -> c_ulonglong
{
  return if n != 0 { 1u64 << (stdc_bit_width_ull (n) - 1) } else { 0 };
}

/// C 1228: `stdc_bit_ceil_uc (unsigned char n)`.
#[inline(always)]
pub fn stdc_bit_ceil_uc (n: c_uchar) -> c_uchar
{
  return if n <= 1 { 1 } else { 2u8 << (stdc_bit_width_uc (n.wrapping_sub (1)) - 1) };
}

/// C 1228: `stdc_bit_ceil_us (unsigned short int n)`.
#[inline(always)]
pub fn stdc_bit_ceil_us (n: c_ushort) -> c_ushort
{
  return if n <= 1 { 1 } else { 2u16 << (stdc_bit_width_us (n.wrapping_sub (1)) - 1) };
}

/// C 1228: `stdc_bit_ceil_ui (unsigned int n)`.
#[inline(always)]
pub fn stdc_bit_ceil_ui (n: c_uint) -> c_uint
{
  return if n <= 1 { 1 } else { 2u32 << (stdc_bit_width_ui (n.wrapping_sub (1)) - 1) };
}

/// C 1228: `stdc_bit_ceil_ul (unsigned long int n)`.
#[inline(always)]
pub fn stdc_bit_ceil_ul (n: c_ulong) -> c_ulong
{
  return if n <= 1 { 1 } else { 2u64 << (stdc_bit_width_ul (n.wrapping_sub (1)) - 1) };
}

/// C 1228: `stdc_bit_ceil_ull (unsigned long long int n)`.
#[inline(always)]
pub fn stdc_bit_ceil_ull (n: c_ulonglong) -> c_ulonglong
{
  return if n <= 1 { 1 } else { 2u64 << (stdc_bit_width_ull (n.wrapping_sub (1)) - 1) };
}

/// C 0: `#  define stdc_bit_floor(n) (_GL_STDBIT_TYPEOF_CAST (n, (sizeof (n) == 1 ? … : …)))` - the
/// dispatcher keeps C's `_GL_STDBIT_TYPEOF_CAST` (the `(b)` arm the port's macro above spells).
macro_rules! stdc_bit_floor {
  ($n:expr) => {
    _GL_STDBIT_TYPEOF_CAST! ($n, (if std::mem::size_of_val (&$n) == 1
       { stdc_bit_floor_uc ($n as c_uchar) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
       { stdc_bit_floor_us ($n as c_ushort) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
       { stdc_bit_floor_ui ($n as c_uint) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
       { stdc_bit_floor_ul ($n as c_ulong) }
     else
       { stdc_bit_floor_ull ($n as c_ulonglong) }))
  };
}

/// C 0: `#  define stdc_bit_ceil(n) (_GL_STDBIT_TYPEOF_CAST (n, (sizeof (n) == 1 ? … : …)))` - the
/// dispatcher keeps C's `_GL_STDBIT_TYPEOF_CAST` (the `(b)` arm the port's macro above spells).
macro_rules! stdc_bit_ceil {
  ($n:expr) => {
    _GL_STDBIT_TYPEOF_CAST! ($n, (if std::mem::size_of_val (&$n) == 1
       { stdc_bit_ceil_uc ($n as c_uchar) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
       { stdc_bit_ceil_us ($n as c_ushort) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u32)
       { stdc_bit_ceil_ui ($n as c_uint) }
     else if std::mem::size_of_val (&$n) == std::mem::size_of_val (&0u64)
       { stdc_bit_ceil_ul ($n as c_ulong) }
     else
       { stdc_bit_ceil_ull ($n as c_ulonglong) }))
  };
}

/* ---- C 1289-1399: `stdc_rotate_*` ----------------------------------------------------------
   C 1290-1294's `__has_builtin (__builtin_stdc_rotate_left)` arm defines both the helper and the
   dispatcher as the *builtin*, which is the arm the frozen compiler takes; the `# ifndef` arm
   below it (C 1296-1299) is the dead text (P-66 rule 2), and the port spells the operation C's
   own shifts write:
     `(((v) << ((c) & (sizeof (v) * 8 - 1))) | ((v) >> (-(c) & (sizeof (v) * 8 - 1))))`.
   Both amounts are masked into `0 .. bits-1`, so no Rust shift can panic; `c` is promoted to the
   `size_t` `sizeof` gives it (C's usual arithmetic conversions), and C's `-(c)` on an unsigned
   count is `0usize.wrapping_sub (c as usize)`.  */
/// C 1289-1291 / 1355-1357: `#  define _gl_stdc_rotate_left(v, c)`.
macro_rules! _gl_stdc_rotate_left {
  ($v:expr, $c:expr) => {
    (($v) << (($c as usize) & (std::mem::size_of_val (&$v) * 8 - 1)))
    | (($v) >> (0usize.wrapping_sub ($c as usize) & (std::mem::size_of_val (&$v) * 8 - 1)))
  };
}

/// C 1297: `stdc_rotate_left_uc (unsigned char v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_left_uc (v: c_uchar, c: c_uint) -> c_uchar
{
  return _gl_stdc_rotate_left! (v, c);
}

/// C 1297: `stdc_rotate_left_us (unsigned short int v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_left_us (v: c_ushort, c: c_uint) -> c_ushort
{
  return _gl_stdc_rotate_left! (v, c);
}

/// C 1297: `stdc_rotate_left_ui (unsigned int v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_left_ui (v: c_uint, c: c_uint) -> c_uint
{
  return _gl_stdc_rotate_left! (v, c);
}

/// C 1297: `stdc_rotate_left_ul (unsigned long int v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_left_ul (v: c_ulong, c: c_uint) -> c_ulong
{
  return _gl_stdc_rotate_left! (v, c);
}

/// C 1297: `stdc_rotate_left_ull (unsigned long long int v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_left_ull (v: c_ulonglong, c: c_uint) -> c_ulonglong
{
  return _gl_stdc_rotate_left! (v, c);
}

/// C 0: `#  define stdc_rotate_left(v, c)` - the `_GL_STDBIT_TYPEOF_CAST` dispatcher.
macro_rules! stdc_rotate_left {
  ($v:expr, $c:expr) => {
    _GL_STDBIT_TYPEOF_CAST! ($v, (if std::mem::size_of_val (&$v) == 1
       { stdc_rotate_left_uc ($v as c_uchar, $c) }
     else if std::mem::size_of_val (&$v) == std::mem::size_of::<c_ushort> ()
       { stdc_rotate_left_us ($v as c_ushort, $c) }
     else if std::mem::size_of_val (&$v) == std::mem::size_of_val (&0u32)
       { stdc_rotate_left_ui ($v as c_uint, $c) }
     else if std::mem::size_of_val (&$v) == std::mem::size_of_val (&0u64)
       { stdc_rotate_left_ul ($v as c_ulong, $c) }
     else
       { stdc_rotate_left_ull ($v as c_ulonglong, $c) }))
  };
}

/// C 1289-1291 / 1355-1357: `#  define _gl_stdc_rotate_right(v, c)`.
macro_rules! _gl_stdc_rotate_right {
  ($v:expr, $c:expr) => {
    (($v) >> (($c as usize) & (std::mem::size_of_val (&$v) * 8 - 1)))
    | (($v) << (0usize.wrapping_sub ($c as usize) & (std::mem::size_of_val (&$v) * 8 - 1)))
  };
}

/// C 1363: `stdc_rotate_right_uc (unsigned char v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_right_uc (v: c_uchar, c: c_uint) -> c_uchar
{
  return _gl_stdc_rotate_right! (v, c);
}

/// C 1363: `stdc_rotate_right_us (unsigned short int v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_right_us (v: c_ushort, c: c_uint) -> c_ushort
{
  return _gl_stdc_rotate_right! (v, c);
}

/// C 1363: `stdc_rotate_right_ui (unsigned int v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_right_ui (v: c_uint, c: c_uint) -> c_uint
{
  return _gl_stdc_rotate_right! (v, c);
}

/// C 1363: `stdc_rotate_right_ul (unsigned long int v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_right_ul (v: c_ulong, c: c_uint) -> c_ulong
{
  return _gl_stdc_rotate_right! (v, c);
}

/// C 1363: `stdc_rotate_right_ull (unsigned long long int v, unsigned int c)`.
#[inline(always)]
pub fn stdc_rotate_right_ull (v: c_ulonglong, c: c_uint) -> c_ulonglong
{
  return _gl_stdc_rotate_right! (v, c);
}

/// C 0: `#  define stdc_rotate_right(v, c)` - the `_GL_STDBIT_TYPEOF_CAST` dispatcher.
macro_rules! stdc_rotate_right {
  ($v:expr, $c:expr) => {
    _GL_STDBIT_TYPEOF_CAST! ($v, (if std::mem::size_of_val (&$v) == 1
       { stdc_rotate_right_uc ($v as c_uchar, $c) }
     else if std::mem::size_of_val (&$v) == std::mem::size_of::<c_ushort> ()
       { stdc_rotate_right_us ($v as c_ushort, $c) }
     else if std::mem::size_of_val (&$v) == std::mem::size_of_val (&0u32)
       { stdc_rotate_right_ui ($v as c_uint, $c) }
     else if std::mem::size_of_val (&$v) == std::mem::size_of_val (&0u64)
       { stdc_rotate_right_ul ($v as c_ulong, $c) }
     else
       { stdc_rotate_right_ull ($v as c_ulonglong, $c) }))
  };
}

/* ---- C 1410-1436: `stdc_memreverse8 (size_t n, unsigned char *ptr)` ------------------------ */
/// C 1416: C's in-place swap loop, `ptr[i]` as `*ptr.add (i)` (Rust has no `Index` on a raw pointer).
#[inline(always)]
pub fn stdc_memreverse8 (n: usize, ptr: *mut c_uchar)
{
  if n > 0
    {
      /* C: `size_t i, j; for (i = 0, j = n-1; i < j; i++, j--)` - the `for` head's
         two updates become the loop's last statements.  */
      let mut i: usize = 0;
      let mut j: usize = n - 1;
      while i < j
        {
          let xi: c_uchar = unsafe { *ptr.add (i) };
          let xj: c_uchar = unsafe { *ptr.add (j) };
          unsafe { *ptr.add (j) = xi; }
          unsafe { *ptr.add (i) = xj; }
          i += 1;
          j -= 1;
        }
    }
}

/* ---- C 1646-2219: the `stdc_load8_*` / `stdc_store8_*` matrices -----------------------------
   C's byte assembly member for member: `_GL_STDBIT_UINT_FASTN vN = ptr[N];` and
   `return (v0 << (8 * N)) | …` with the little-endian shift list (C 1687-1774) and its reverse
   for the big-endian one (C 1646-1685), C's `ptr[N] = (value >> N) & 0xFFU;` for the stores
   (C 1946-2062, 2073-2209), each `*s*` member C's own delegation to its unsigned original and
   each `_aligned` one C's two arms (`_GL_STDBIT_OPTIMIZE_VIA_MEMCPY` is the frozen `0`; C
   compiles both arms, and so does the port).  The `_aligned` arm keeps C's
   `_GL_STDBIT_MEMCPY`/`_GL_STDBIT_ASSUME_ALIGNED` macros; `_GL_STDBIT_ASSUME_ALIGNED`'s Rust
   body is C's `(ptr)` arm, because Rust has no `__builtin_assume_aligned` and the port needs
   none.  Rust has no `Index` on a raw pointer, so `ptr[I]` is `*ptr.add (I)`, and the FAST-typed
   locals take the `unsigned char` byte through `as _GL_STDBIT_UINT_FASTN` (C's own implicit
   conversion, written out).  */
/// C 1646: `stdc_load8_beu8 (const unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_load8_beu8 (ptr: *const c_uchar) -> uint_least8_t
{
  return unsafe { *ptr.add (0) };
}

/// C 1728: `stdc_load8_bes8 (const unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_load8_bes8 (ptr: *const c_uchar) -> int_least8_t
{
  return stdc_load8_beu8 (ptr) as int_least8_t;
}

/// C 1652: `stdc_load8_beu16 (const unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_load8_beu16 (ptr: *const c_uchar) -> uint_least16_t
{
  let v0: _GL_STDBIT_UINT_FAST16 = unsafe { *ptr.add (0) as _GL_STDBIT_UINT_FAST16 };
  let v1: _GL_STDBIT_UINT_FAST16 = unsafe { *ptr.add (1) as _GL_STDBIT_UINT_FAST16 };
  return (v0 << (8 * 1)) | (v1 << (8 * 0));
}

/// C 1734: `stdc_load8_bes16 (const unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_load8_bes16 (ptr: *const c_uchar) -> int_least16_t
{
  return stdc_load8_beu16 (ptr) as int_least16_t;
}

/// C 1660: `stdc_load8_beu32 (const unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_load8_beu32 (ptr: *const c_uchar) -> uint_least32_t
{
  let v0: _GL_STDBIT_UINT_FAST32 = unsafe { *ptr.add (0) as _GL_STDBIT_UINT_FAST32 };
  let v1: _GL_STDBIT_UINT_FAST32 = unsafe { *ptr.add (1) as _GL_STDBIT_UINT_FAST32 };
  let v2: _GL_STDBIT_UINT_FAST32 = unsafe { *ptr.add (2) as _GL_STDBIT_UINT_FAST32 };
  let v3: _GL_STDBIT_UINT_FAST32 = unsafe { *ptr.add (3) as _GL_STDBIT_UINT_FAST32 };
  return (v0 << (8 * 3)) | (v1 << (8 * 2)) | (v2 << (8 * 1)) | (v3 << (8 * 0));
}

/// C 1740: `stdc_load8_bes32 (const unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_load8_bes32 (ptr: *const c_uchar) -> int_least32_t
{
  return stdc_load8_beu32 (ptr) as int_least32_t;
}

/// C 1670: `stdc_load8_beu64 (const unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_load8_beu64 (ptr: *const c_uchar) -> uint_least64_t
{
  let v0: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (0) as _GL_STDBIT_UINT_FAST64 };
  let v1: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (1) as _GL_STDBIT_UINT_FAST64 };
  let v2: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (2) as _GL_STDBIT_UINT_FAST64 };
  let v3: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (3) as _GL_STDBIT_UINT_FAST64 };
  let v4: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (4) as _GL_STDBIT_UINT_FAST64 };
  let v5: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (5) as _GL_STDBIT_UINT_FAST64 };
  let v6: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (6) as _GL_STDBIT_UINT_FAST64 };
  let v7: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (7) as _GL_STDBIT_UINT_FAST64 };
  return (v0 << (8 * 7)) | (v1 << (8 * 6)) | (v2 << (8 * 5)) | (v3 << (8 * 4)) | (v4 << (8 * 3)) | (v5 << (8 * 2)) | (v6 << (8 * 1)) | (v7 << (8 * 0));
}

/// C 1746: `stdc_load8_bes64 (const unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_load8_bes64 (ptr: *const c_uchar) -> int_least64_t
{
  return stdc_load8_beu64 (ptr) as int_least64_t;
}

/// C 1687: `stdc_load8_leu8 (const unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_load8_leu8 (ptr: *const c_uchar) -> uint_least8_t
{
  return unsafe { *ptr.add (0) };
}

/// C 1752: `stdc_load8_les8 (const unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_load8_les8 (ptr: *const c_uchar) -> int_least8_t
{
  return stdc_load8_leu8 (ptr) as int_least8_t;
}

/// C 1693: `stdc_load8_leu16 (const unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_load8_leu16 (ptr: *const c_uchar) -> uint_least16_t
{
  let v0: _GL_STDBIT_UINT_FAST16 = unsafe { *ptr.add (0) as _GL_STDBIT_UINT_FAST16 };
  let v1: _GL_STDBIT_UINT_FAST16 = unsafe { *ptr.add (1) as _GL_STDBIT_UINT_FAST16 };
  return (v0 << (8 * 0)) | (v1 << (8 * 1));
}

/// C 1758: `stdc_load8_les16 (const unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_load8_les16 (ptr: *const c_uchar) -> int_least16_t
{
  return stdc_load8_leu16 (ptr) as int_least16_t;
}

/// C 1701: `stdc_load8_leu32 (const unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_load8_leu32 (ptr: *const c_uchar) -> uint_least32_t
{
  let v0: _GL_STDBIT_UINT_FAST32 = unsafe { *ptr.add (0) as _GL_STDBIT_UINT_FAST32 };
  let v1: _GL_STDBIT_UINT_FAST32 = unsafe { *ptr.add (1) as _GL_STDBIT_UINT_FAST32 };
  let v2: _GL_STDBIT_UINT_FAST32 = unsafe { *ptr.add (2) as _GL_STDBIT_UINT_FAST32 };
  let v3: _GL_STDBIT_UINT_FAST32 = unsafe { *ptr.add (3) as _GL_STDBIT_UINT_FAST32 };
  return (v0 << (8 * 0)) | (v1 << (8 * 1)) | (v2 << (8 * 2)) | (v3 << (8 * 3));
}

/// C 1764: `stdc_load8_les32 (const unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_load8_les32 (ptr: *const c_uchar) -> int_least32_t
{
  return stdc_load8_leu32 (ptr) as int_least32_t;
}

/// C 1711: `stdc_load8_leu64 (const unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_load8_leu64 (ptr: *const c_uchar) -> uint_least64_t
{
  let v0: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (0) as _GL_STDBIT_UINT_FAST64 };
  let v1: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (1) as _GL_STDBIT_UINT_FAST64 };
  let v2: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (2) as _GL_STDBIT_UINT_FAST64 };
  let v3: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (3) as _GL_STDBIT_UINT_FAST64 };
  let v4: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (4) as _GL_STDBIT_UINT_FAST64 };
  let v5: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (5) as _GL_STDBIT_UINT_FAST64 };
  let v6: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (6) as _GL_STDBIT_UINT_FAST64 };
  let v7: _GL_STDBIT_UINT_FAST64 = unsafe { *ptr.add (7) as _GL_STDBIT_UINT_FAST64 };
  return (v0 << (8 * 0)) | (v1 << (8 * 1)) | (v2 << (8 * 2)) | (v3 << (8 * 3)) | (v4 << (8 * 4)) | (v5 << (8 * 5)) | (v6 << (8 * 6)) | (v7 << (8 * 7));
}

/// C 1770: `stdc_load8_les64 (const unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_load8_les64 (ptr: *const c_uchar) -> int_least64_t
{
  return stdc_load8_leu64 (ptr) as int_least64_t;
}

/// C 1785: `stdc_load8_aligned_beu8 (const unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_load8_aligned_beu8 (ptr: *const c_uchar) -> uint_least8_t
{
  return stdc_load8_beu8 (ptr);
}

/// C 1887: `stdc_load8_aligned_bes8 (const unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_load8_aligned_bes8 (ptr: *const c_uchar) -> int_least8_t
{
  return stdc_load8_aligned_beu8 (ptr) as int_least8_t;
}

/// C 1791: `stdc_load8_aligned_beu16 (const unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_load8_aligned_beu16 (ptr: *const c_uchar) -> uint_least16_t
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      let mut value: uint_least16_t = 0;
      _GL_STDBIT_MEMCPY! (&mut value, _GL_STDBIT_ASSUME_ALIGNED! (ptr, 2), 2);
      if !_GL_STDBIT_BIGENDIAN { value = stdc_memreverse8u16 (value); }
      return value;
    }
  else
    { return stdc_load8_beu16 (ptr); }
}

/// C 1893: `stdc_load8_aligned_bes16 (const unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_load8_aligned_bes16 (ptr: *const c_uchar) -> int_least16_t
{
  return stdc_load8_aligned_beu16 (ptr) as int_least16_t;
}

/// C 1806: `stdc_load8_aligned_beu32 (const unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_load8_aligned_beu32 (ptr: *const c_uchar) -> uint_least32_t
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      let mut value: uint_least32_t = 0;
      _GL_STDBIT_MEMCPY! (&mut value, _GL_STDBIT_ASSUME_ALIGNED! (ptr, 4), 4);
      if !_GL_STDBIT_BIGENDIAN { value = stdc_memreverse8u32 (value); }
      return value;
    }
  else
    { return stdc_load8_beu32 (ptr); }
}

/// C 1899: `stdc_load8_aligned_bes32 (const unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_load8_aligned_bes32 (ptr: *const c_uchar) -> int_least32_t
{
  return stdc_load8_aligned_beu32 (ptr) as int_least32_t;
}

/// C 1821: `stdc_load8_aligned_beu64 (const unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_load8_aligned_beu64 (ptr: *const c_uchar) -> uint_least64_t
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      let mut value: uint_least64_t = 0;
      _GL_STDBIT_MEMCPY! (&mut value, _GL_STDBIT_ASSUME_ALIGNED! (ptr, 8), 8);
      if !_GL_STDBIT_BIGENDIAN { value = stdc_memreverse8u64 (value); }
      return value;
    }
  else
    { return stdc_load8_beu64 (ptr); }
}

/// C 1905: `stdc_load8_aligned_bes64 (const unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_load8_aligned_bes64 (ptr: *const c_uchar) -> int_least64_t
{
  return stdc_load8_aligned_beu64 (ptr) as int_least64_t;
}

/// C 1836: `stdc_load8_aligned_leu8 (const unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_load8_aligned_leu8 (ptr: *const c_uchar) -> uint_least8_t
{
  return stdc_load8_leu8 (ptr);
}

/// C 1911: `stdc_load8_aligned_les8 (const unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_load8_aligned_les8 (ptr: *const c_uchar) -> int_least8_t
{
  return stdc_load8_aligned_leu8 (ptr) as int_least8_t;
}

/// C 1842: `stdc_load8_aligned_leu16 (const unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_load8_aligned_leu16 (ptr: *const c_uchar) -> uint_least16_t
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      let mut value: uint_least16_t = 0;
      _GL_STDBIT_MEMCPY! (&mut value, _GL_STDBIT_ASSUME_ALIGNED! (ptr, 2), 2);
      if _GL_STDBIT_BIGENDIAN { value = stdc_memreverse8u16 (value); }
      return value;
    }
  else
    { return stdc_load8_leu16 (ptr); }
}

/// C 1917: `stdc_load8_aligned_les16 (const unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_load8_aligned_les16 (ptr: *const c_uchar) -> int_least16_t
{
  return stdc_load8_aligned_leu16 (ptr) as int_least16_t;
}

/// C 1857: `stdc_load8_aligned_leu32 (const unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_load8_aligned_leu32 (ptr: *const c_uchar) -> uint_least32_t
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      let mut value: uint_least32_t = 0;
      _GL_STDBIT_MEMCPY! (&mut value, _GL_STDBIT_ASSUME_ALIGNED! (ptr, 4), 4);
      if _GL_STDBIT_BIGENDIAN { value = stdc_memreverse8u32 (value); }
      return value;
    }
  else
    { return stdc_load8_leu32 (ptr); }
}

/// C 1923: `stdc_load8_aligned_les32 (const unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_load8_aligned_les32 (ptr: *const c_uchar) -> int_least32_t
{
  return stdc_load8_aligned_leu32 (ptr) as int_least32_t;
}

/// C 1872: `stdc_load8_aligned_leu64 (const unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_load8_aligned_leu64 (ptr: *const c_uchar) -> uint_least64_t
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      let mut value: uint_least64_t = 0;
      _GL_STDBIT_MEMCPY! (&mut value, _GL_STDBIT_ASSUME_ALIGNED! (ptr, 8), 8);
      if _GL_STDBIT_BIGENDIAN { value = stdc_memreverse8u64 (value); }
      return value;
    }
  else
    { return stdc_load8_leu64 (ptr); }
}

/// C 1929: `stdc_load8_aligned_les64 (const unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_load8_aligned_les64 (ptr: *const c_uchar) -> int_least64_t
{
  return stdc_load8_aligned_leu64 (ptr) as int_least64_t;
}

/// C 1947: `stdc_store8_beu8 (uint_least8_t value, unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_store8_beu8 (value: uint_least8_t, ptr: *mut c_uchar)
{
  unsafe { *ptr.add (0) = value; }
}

/// C 2017: `stdc_store8_bes8 (int_least8_t value, unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_store8_bes8 (value: int_least8_t, ptr: *mut c_uchar)
{
  stdc_store8_beu8 (value as uint_least8_t, ptr);
}

/// C 1953: `stdc_store8_beu16 (uint_least16_t value, unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_store8_beu16 (value: uint_least16_t, ptr: *mut c_uchar)
{
  unsafe { *ptr.add (0) = ((value >> 8) & 0xFF) as c_uchar };
  unsafe { *ptr.add (1) = (value & 0xFF) as c_uchar };
}

/// C 2023: `stdc_store8_bes16 (int_least16_t value, unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_store8_bes16 (value: int_least16_t, ptr: *mut c_uchar)
{
  stdc_store8_beu16 (value as uint_least16_t, ptr);
}

/// C 1960: `stdc_store8_beu32 (uint_least32_t value, unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_store8_beu32 (value: uint_least32_t, ptr: *mut c_uchar)
{
  unsafe { *ptr.add (0) = ((value >> 24) & 0xFF) as c_uchar };
  unsafe { *ptr.add (1) = ((value >> 16) & 0xFF) as c_uchar };
  unsafe { *ptr.add (2) = ((value >> 8) & 0xFF) as c_uchar };
  unsafe { *ptr.add (3) = (value & 0xFF) as c_uchar };
}

/// C 2029: `stdc_store8_bes32 (int_least32_t value, unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_store8_bes32 (value: int_least32_t, ptr: *mut c_uchar)
{
  stdc_store8_beu32 (value as uint_least32_t, ptr);
}

/// C 1969: `stdc_store8_beu64 (uint_least64_t value, unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_store8_beu64 (value: uint_least64_t, ptr: *mut c_uchar)
{
  unsafe { *ptr.add (0) = ((value >> 56) & 0xFF) as c_uchar };
  unsafe { *ptr.add (1) = ((value >> 48) & 0xFF) as c_uchar };
  unsafe { *ptr.add (2) = ((value >> 40) & 0xFF) as c_uchar };
  unsafe { *ptr.add (3) = ((value >> 32) & 0xFF) as c_uchar };
  unsafe { *ptr.add (4) = ((value >> 24) & 0xFF) as c_uchar };
  unsafe { *ptr.add (5) = ((value >> 16) & 0xFF) as c_uchar };
  unsafe { *ptr.add (6) = ((value >> 8) & 0xFF) as c_uchar };
  unsafe { *ptr.add (7) = (value & 0xFF) as c_uchar };
}

/// C 2035: `stdc_store8_bes64 (int_least64_t value, unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_store8_bes64 (value: int_least64_t, ptr: *mut c_uchar)
{
  stdc_store8_beu64 (value as uint_least64_t, ptr);
}

/// C 1982: `stdc_store8_leu8 (uint_least8_t value, unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_store8_leu8 (value: uint_least8_t, ptr: *mut c_uchar)
{
  unsafe { *ptr.add (0) = value; }
}

/// C 2041: `stdc_store8_les8 (int_least8_t value, unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_store8_les8 (value: int_least8_t, ptr: *mut c_uchar)
{
  stdc_store8_leu8 (value as uint_least8_t, ptr);
}

/// C 1988: `stdc_store8_leu16 (uint_least16_t value, unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_store8_leu16 (value: uint_least16_t, ptr: *mut c_uchar)
{
  unsafe { *ptr.add (0) = (value & 0xFF) as c_uchar };
  unsafe { *ptr.add (1) = ((value >> 8) & 0xFF) as c_uchar };
}

/// C 2047: `stdc_store8_les16 (int_least16_t value, unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_store8_les16 (value: int_least16_t, ptr: *mut c_uchar)
{
  stdc_store8_leu16 (value as uint_least16_t, ptr);
}

/// C 1995: `stdc_store8_leu32 (uint_least32_t value, unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_store8_leu32 (value: uint_least32_t, ptr: *mut c_uchar)
{
  unsafe { *ptr.add (0) = (value & 0xFF) as c_uchar };
  unsafe { *ptr.add (1) = ((value >> 8) & 0xFF) as c_uchar };
  unsafe { *ptr.add (2) = ((value >> 16) & 0xFF) as c_uchar };
  unsafe { *ptr.add (3) = ((value >> 24) & 0xFF) as c_uchar };
}

/// C 2053: `stdc_store8_les32 (int_least32_t value, unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_store8_les32 (value: int_least32_t, ptr: *mut c_uchar)
{
  stdc_store8_leu32 (value as uint_least32_t, ptr);
}

/// C 2004: `stdc_store8_leu64 (uint_least64_t value, unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_store8_leu64 (value: uint_least64_t, ptr: *mut c_uchar)
{
  unsafe { *ptr.add (0) = (value & 0xFF) as c_uchar };
  unsafe { *ptr.add (1) = ((value >> 8) & 0xFF) as c_uchar };
  unsafe { *ptr.add (2) = ((value >> 16) & 0xFF) as c_uchar };
  unsafe { *ptr.add (3) = ((value >> 24) & 0xFF) as c_uchar };
  unsafe { *ptr.add (4) = ((value >> 32) & 0xFF) as c_uchar };
  unsafe { *ptr.add (5) = ((value >> 40) & 0xFF) as c_uchar };
  unsafe { *ptr.add (6) = ((value >> 48) & 0xFF) as c_uchar };
  unsafe { *ptr.add (7) = ((value >> 56) & 0xFF) as c_uchar };
}

/// C 2059: `stdc_store8_les64 (int_least64_t value, unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_store8_les64 (value: int_least64_t, ptr: *mut c_uchar)
{
  stdc_store8_leu64 (value as uint_least64_t, ptr);
}

/// C 2074: `stdc_store8_aligned_beu8 (uint_least8_t value, unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_store8_aligned_beu8 (value: uint_least8_t, ptr: *mut c_uchar)
{
  stdc_store8_beu8 (value, ptr);
}

/// C 2164: `stdc_store8_aligned_bes8 (int_least8_t value, unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_store8_aligned_bes8 (value: int_least8_t, ptr: *mut c_uchar)
{
  stdc_store8_aligned_beu8 (value as uint_least8_t, ptr);
}

/// C 2080: `stdc_store8_aligned_beu16 (uint_least16_t value, unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_store8_aligned_beu16 (mut value: uint_least16_t, ptr: *mut c_uchar)
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      if !_GL_STDBIT_BIGENDIAN
        { value = stdc_memreverse8u16 (value); }
      _GL_STDBIT_MEMCPY! (_GL_STDBIT_ASSUME_ALIGNED! (ptr, 2), &value, 2);
    }
  else
    { stdc_store8_beu16 (value, ptr); }
}

/// C 2170: `stdc_store8_aligned_bes16 (int_least16_t value, unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_store8_aligned_bes16 (value: int_least16_t, ptr: *mut c_uchar)
{
  stdc_store8_aligned_beu16 (value as uint_least16_t, ptr);
}

/// C 2093: `stdc_store8_aligned_beu32 (uint_least32_t value, unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_store8_aligned_beu32 (mut value: uint_least32_t, ptr: *mut c_uchar)
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      if !_GL_STDBIT_BIGENDIAN
        { value = stdc_memreverse8u32 (value); }
      _GL_STDBIT_MEMCPY! (_GL_STDBIT_ASSUME_ALIGNED! (ptr, 4), &value, 4);
    }
  else
    { stdc_store8_beu32 (value, ptr); }
}

/// C 2176: `stdc_store8_aligned_bes32 (int_least32_t value, unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_store8_aligned_bes32 (value: int_least32_t, ptr: *mut c_uchar)
{
  stdc_store8_aligned_beu32 (value as uint_least32_t, ptr);
}

/// C 2106: `stdc_store8_aligned_beu64 (uint_least64_t value, unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_store8_aligned_beu64 (mut value: uint_least64_t, ptr: *mut c_uchar)
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      if !_GL_STDBIT_BIGENDIAN
        { value = stdc_memreverse8u64 (value); }
      _GL_STDBIT_MEMCPY! (_GL_STDBIT_ASSUME_ALIGNED! (ptr, 8), &value, 8);
    }
  else
    { stdc_store8_beu64 (value, ptr); }
}

/// C 2182: `stdc_store8_aligned_bes64 (int_least64_t value, unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_store8_aligned_bes64 (value: int_least64_t, ptr: *mut c_uchar)
{
  stdc_store8_aligned_beu64 (value as uint_least64_t, ptr);
}

/// C 2119: `stdc_store8_aligned_leu8 (uint_least8_t value, unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_store8_aligned_leu8 (value: uint_least8_t, ptr: *mut c_uchar)
{
  stdc_store8_leu8 (value, ptr);
}

/// C 2188: `stdc_store8_aligned_les8 (int_least8_t value, unsigned char ptr[1])`.
#[inline(always)]
pub fn stdc_store8_aligned_les8 (value: int_least8_t, ptr: *mut c_uchar)
{
  stdc_store8_aligned_leu8 (value as uint_least8_t, ptr);
}

/// C 2125: `stdc_store8_aligned_leu16 (uint_least16_t value, unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_store8_aligned_leu16 (mut value: uint_least16_t, ptr: *mut c_uchar)
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      if _GL_STDBIT_BIGENDIAN
        { value = stdc_memreverse8u16 (value); }
      _GL_STDBIT_MEMCPY! (_GL_STDBIT_ASSUME_ALIGNED! (ptr, 2), &value, 2);
    }
  else
    { stdc_store8_leu16 (value, ptr); }
}

/// C 2194: `stdc_store8_aligned_les16 (int_least16_t value, unsigned char ptr[2])`.
#[inline(always)]
pub fn stdc_store8_aligned_les16 (value: int_least16_t, ptr: *mut c_uchar)
{
  stdc_store8_aligned_leu16 (value as uint_least16_t, ptr);
}

/// C 2138: `stdc_store8_aligned_leu32 (uint_least32_t value, unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_store8_aligned_leu32 (mut value: uint_least32_t, ptr: *mut c_uchar)
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      if _GL_STDBIT_BIGENDIAN
        { value = stdc_memreverse8u32 (value); }
      _GL_STDBIT_MEMCPY! (_GL_STDBIT_ASSUME_ALIGNED! (ptr, 4), &value, 4);
    }
  else
    { stdc_store8_leu32 (value, ptr); }
}

/// C 2200: `stdc_store8_aligned_les32 (int_least32_t value, unsigned char ptr[4])`.
#[inline(always)]
pub fn stdc_store8_aligned_les32 (value: int_least32_t, ptr: *mut c_uchar)
{
  stdc_store8_aligned_leu32 (value as uint_least32_t, ptr);
}

/// C 2151: `stdc_store8_aligned_leu64 (uint_least64_t value, unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_store8_aligned_leu64 (mut value: uint_least64_t, ptr: *mut c_uchar)
{
  if _GL_STDBIT_OPTIMIZE_VIA_MEMCPY != 0
    {
      if _GL_STDBIT_BIGENDIAN
        { value = stdc_memreverse8u64 (value); }
      _GL_STDBIT_MEMCPY! (_GL_STDBIT_ASSUME_ALIGNED! (ptr, 8), &value, 8);
    }
  else
    { stdc_store8_leu64 (value, ptr); }
}

/// C 2206: `stdc_store8_aligned_les64 (int_least64_t value, unsigned char ptr[8])`.
#[inline(always)]
pub fn stdc_store8_aligned_les64 (value: int_least64_t, ptr: *mut c_uchar)
{
  stdc_store8_aligned_leu64 (value as uint_least64_t, ptr);
}

