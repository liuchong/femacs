// femacs: mirror of the frozen target's <stdbit.h> (C23; gnulib header, P-26).
//
// The frozen configuration has HAVE_STDBIT_H = 1, so this is gnulib's stdbit.in.h with
// `configure`'s substitutions applied: the `_GL_STDBIT_HAS_BUILTIN_CLZ/CTZ/POPCOUNT`
// branches are the live ones (this target's compiler has __builtin_clz/ctz/popcount), so
// the `_gl_stdbit_*` helpers the macro arms call are Rust's leading_zeros,
// trailing_zeros and count_ones.  The three generic macros data.c uses are mirrored
// whole - each is C's dispatcher on `sizeof (n)`, and the narrow variants subtract the
// promoted width exactly as stdbit.in.h 318-345, 500-529 and 966-992 do.
//
// The header's leading-ones/trailing-ones/bit-width and single-bit functions are not
// mirrored: no ported file calls them yet, and a mirror is added when one does (the
// stdcountof.rs precedent).

/* stdc_leading_zeros_ui … _ull (stdbit.in.h 317-345); each calls `_gl_stdbit_clz*` */
#[inline(always)]
pub fn stdc_leading_zeros_ui (n: c_uint) -> c_uint { n.leading_zeros () }
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
pub fn stdc_leading_zeros_ul (n: c_ulong) -> c_uint { n.leading_zeros () }
#[inline(always)]
pub fn stdc_leading_zeros_ull (n: c_ulonglong) -> c_uint { n.leading_zeros () }

/* stdc_trailing_zeros_ui … _ull (stdbit.in.h 499-527); each calls `_gl_stdbit_ctz*`,
   which is `n ? __builtin_ctz* (n) : 8 * sizeof n`, i.e. the width when N is zero.  */
#[inline(always)]
pub fn stdc_trailing_zeros_ui (n: c_uint) -> c_uint
{
  return if n != 0 { n.trailing_zeros () }
         else { (8 * std::mem::size_of_val (&n)) as c_uint };
}
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
pub fn stdc_trailing_zeros_ul (n: c_ulong) -> c_uint
{
  return if n != 0 { n.trailing_zeros () }
         else { (8 * std::mem::size_of_val (&n)) as c_uint };
}
#[inline(always)]
pub fn stdc_trailing_zeros_ull (n: c_ulonglong) -> c_uint
{
  return if n != 0 { n.trailing_zeros () }
         else { (8 * std::mem::size_of_val (&n)) as c_uint };
}

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
