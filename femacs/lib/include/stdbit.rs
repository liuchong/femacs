// femacs: mirror of the frozen target's <stdbit.h> (C23; gnulib header, P-26).
//
// The frozen configuration has HAVE_STDBIT_H **undefined** (`build/emacs-ref/src/config.h`
// 1433 `/* #undef HAVE_STDBIT_H */`, `docs/baseline/config/darwin-arm64.macros.tsv` 1556
// `HAVE_STDBIT_H<TAB>undef`), so the system has no C23 `<stdbit.h>` and gnulib generates its
// own from `stdbit.in.h` with `configure`'s substitutions applied
// (`build/emacs-ref/lib/stdbit.h`, 2,230 lines - the judge `include-stdbit.in.md` uses): the
// `_GL_STDBIT_HAS_BUILTIN_CLZ/CTZ/POPCOUNT` branches are the live ones (this target's compiler
// *does* have __builtin_clz/ctz/popcount - that is a fact about clang, not about the system
// header), so
// the `_gl_stdbit_*` helpers the macro arms call are Rust's leading_zeros,
// trailing_zeros and count_ones.  The three generic macros the ported files use are
// mirrored whole - each is C's dispatcher on `sizeof (n)`, and the narrow variants
// subtract the promoted width exactly as stdbit.in.h 318-345, 500-529 and 966-992 do.
//
// `_GL_STDBIT_HAS_BUILTIN_BSWAP16/32/64` are the live arms as well (this target's
// compiler has __builtin_bswap*), so the four `stdc_memreverse8u*` bodies below are
// `swap_bytes`; the header's hand-written masks are the dead arm (P-51).
//
// The header's leading-ones/trailing-ones/first-*/rotate/load8/store8 functions and the
// remaining generic macros are not mirrored: no ported file calls them yet, and a mirror
// is added when one does (the stdcountof.rs precedent).
//
// The four extern-inline units that include this header - lib/stdc_leading_zeros.rs,
// lib/stdc_trailing_zeros.rs, lib/stdc_bit_width.rs and lib/stdc_memreverse8u.rs - export
// their own share of these functions (PORTING.md P-81), so this mirror stays
// declaration-only: `#[inline(always)]` items and `macro_rules!`, never a symbol.
// Every unit that includes it brings the C names into scope itself, with
// `use std::ffi::{c_int, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};`.

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
         - 8 * (std::mem::size_of::<c_uint> () - std::mem::size_of_val (&n)) as c_uint;
}
#[inline(always)]
pub fn stdc_leading_zeros_us (n: c_ushort) -> c_uint
{
  return stdc_leading_zeros_ui (n as c_uint)
         - 8 * (std::mem::size_of::<c_uint> () - std::mem::size_of_val (&n)) as c_uint;
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

/* stdc_count_ones_ui … _ull (stdbit.in.h 965-992); each calls `_gl_stdbit_popcount*`,
   which is `__builtin_popcount*` - the narrow variants need no adjustment because
   promoting a narrow value cannot add a one bit.  */
#[inline(always)]
pub fn stdc_count_ones_ui (n: c_uint) -> c_uint { n.count_ones () }
#[inline(always)]
pub fn stdc_count_ones_uc (n: c_uchar) -> c_uint { stdc_count_ones_ui (n as c_uint) }
#[inline(always)]
pub fn stdc_count_ones_us (n: c_ushort) -> c_uint { stdc_count_ones_ui (n as c_uint) }
#[inline(always)]
pub fn stdc_count_ones_ul (n: c_ulong) -> c_uint { n.count_ones () }
#[inline(always)]
pub fn stdc_count_ones_ull (n: c_ulonglong) -> c_uint { n.count_ones () }

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

/* stdbit.in.h 350-355: `#define stdc_leading_zeros(n) (sizeof (n) == 1 ? … )`.  Rust
   has no `?:` chain, so the same five-way dispatch is an if/else expression; the casts
   are C's implicit conversions to the chosen variant's parameter type.  */
macro_rules! stdc_leading_zeros {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_leading_zeros_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_leading_zeros_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_uint> ()
           { stdc_leading_zeros_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ulong> ()
           { stdc_leading_zeros_ul ($n as c_ulong) }
         else
           { stdc_leading_zeros_ull ($n as c_ulonglong) })
    };
}

/* stdbit.in.h 532-537.  */
macro_rules! stdc_trailing_zeros {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_trailing_zeros_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_trailing_zeros_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_uint> ()
           { stdc_trailing_zeros_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ulong> ()
           { stdc_trailing_zeros_ul ($n as c_ulong) }
         else
           { stdc_trailing_zeros_ull ($n as c_ulonglong) })
    };
}

/* stdbit.in.h 998-1003.  */
macro_rules! stdc_count_ones {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_count_ones_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_count_ones_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_uint> ()
           { stdc_count_ones_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ulong> ()
           { stdc_count_ones_ul ($n as c_ulong) }
         else
           { stdc_count_ones_ull ($n as c_ulonglong) })
    };
}

/* stdbit.in.h 1151-1156.  */
macro_rules! stdc_bit_width {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_bit_width_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_bit_width_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_uint> ()
           { stdc_bit_width_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ulong> ()
           { stdc_bit_width_ul ($n as c_ulong) }
         else
           { stdc_bit_width_ull ($n as c_ulonglong) })
    };
}
