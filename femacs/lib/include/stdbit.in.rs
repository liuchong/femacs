// femacs: port of lib/stdbit.in.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1, P-26) - **WIP**, and this note is the map for the rest.
//
// This is the largest gnulib template in the tree: 2,229 lines, and the extractor's item list is
// `173 functions defined / 90 macros / 1 file-scope object` (264 unchecked boxes in
// docs/ports/include-stdbit.in.md).  What is carried below is the *settings layer* (the constants
// and the qualifier spellings that every family refers to); the families themselves are recorded,
// not written, with their C line ranges and the idioms they need - see `## Decisions` in the ledger
// for the inventory, the arm table and the one measured idiom that makes the rest mechanical.
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
// with every token C's own.  Without that idiom each of the ~35 width-arithmetic members would need
// a `docs/ports/macro-body-notes.tsv` row for its `0` - which is why the mirror's plan uses it
// instead of asking for rows.


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

/// C 100-105: `# ifndef _GL_STDBIT_MEMCPY` / `#  include <string.h>` / `#  define
/// _GL_STDBIT_MEMCPY(dest, src, n) memcpy (dest, src, n)` - a rename of the system call, so the
/// port spells it with the family `docs/ports/token-aliases.tsv` gives `memcpy`
/// (`std::ptr::copy_nonoverlapping`, whose occurrence check keeps the call alive).
/// C: #  define _GL_STDBIT_MEMCPY(dest, src, n) memcpy (dest, src, n)
macro_rules! _GL_STDBIT_MEMCPY {
  ($dest:expr, $src:expr, $n:expr) => { std::ptr::copy_nonoverlapping ($src, $dest, $n) };
}

/// C 186-190: `# define _GL_STDBIT_TYPEOF_CAST(a, b) (b)` - the arm the frozen compiler records
/// last (the other arm tests `typeof` support).
/// C: # define _GL_STDBIT_TYPEOF_CAST(a, b) (b)
macro_rules! _GL_STDBIT_TYPEOF_CAST { ($a:expr, $b:expr) => { ($b) }; }

/// C 1606-1622: the alignment hint.  `_GL_HAS_BUILTIN_ASSUME_ALIGNED` is defined for this
/// compiler (C 1611's `# define … 1`), so C 1622's builtin arm is the live one.
/// C: # define _GL_HAS_BUILTIN_ASSUME_ALIGNED 1
/// C: # define _GL_STDBIT_ASSUME_ALIGNED(ptr, align) /// C:     __builtin_assume_aligned (ptr, align)
pub const _GL_HAS_BUILTIN_ASSUME_ALIGNED: c_int = 1;
macro_rules! _GL_STDBIT_ASSUME_ALIGNED {
  ($ptr:expr, $align:expr) => { __builtin_assume_aligned ($ptr, $align) };
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

/* ---- the families themselves: recorded, not written (the ledger's inventory) --------------- */
//
// Every one of the following is the `# if !GNULIB_defined_*` block the constant above guards: 5
// type-specific `_GL_STDC_*_INLINE` functions plus one `sizeof`-dispatch macro (C's own text:
// `stdc_leading_zeros(n)` at C 350-356, and the same shape 24 more times).  The bodies are the
// helper calls (`_gl_stdbit_clz (n)` and friends) or the width arithmetic the head note gives the
// idiom for; the `_gl_stdbit_*` helpers themselves are `stdbit.rs`'s and this mirror's, pending.
//
//   leading_zeros C 312-356, leading_ones C 359-402, trailing_zeros C 494-535, trailing_ones
//   C 541-584, first_leading_zero C 637-643, first_leading_one C 696-702, first_trailing_zero
//   C 755-761, first_trailing_one C 814-820, count_zeros C 1044-1050, count_ones C 995-1001,
//   bit_width C 1148-1154, bit_floor C 1200-1210, bit_ceil C 1257-1267, has_single_bit
//   C 1098-1104, rotate_left C 1326-1333, rotate_right C 1392-1399, memreverse8 C 1436-1443,
//   memreverse8u C 1503-1512, load8 C 1775-1785, load8_aligned C 1934-1944, store8 C 2064-2074,
//   store8_aligned C 2211-2219, and the helpers `_gl_stdbit_clz/clzl/clzll` C 228-311,
//   `ctz…` C 419-493, `popcount*` C 837-960 (three arms each: two builtin arms and the
//   hand-written fallback, which is dead text here).
