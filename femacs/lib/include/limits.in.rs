// femacs: port of lib/limits.in.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror: it carries only the integer-width section (limits.in.h 73-87) that
// the ported units need so far; the rest of the header is tracked in
// docs/ports/limits.in.md.  Layout/linkage: constants only (PORTING.md 0.1).

/// lib/limits.in.h 81: the number of usable bits in an integer type with
/// minimum MIN and maximum MAX.  The C version works in `#if` expressions and
/// for both signed and unsigned MAX; the port instantiates it for the signed
/// machine word and instantiates unsigned forms when a unit first needs one.
#[inline(always)]
pub const fn _GL_INTEGER_WIDTH (min: isize, max: isize) -> usize {
    ((min < 0) as usize) + _GL_COB128 (max)
}
/// lib/limits.in.h 82: count the 1 bits of the low-order 128 bits.
#[inline(always)]
pub const fn _GL_COB128 (n: isize) -> usize { _GL_COB64 (n >> 31 >> 31 >> 2) + _GL_COB64 (n) }
#[inline(always)]
pub const fn _GL_COB64 (n: isize) -> usize { _GL_COB32 (n >> 31 >> 1) + _GL_COB32 (n) }
#[inline(always)]
pub const fn _GL_COB32 (n: isize) -> usize { _GL_COB16 (n >> 16) + _GL_COB16 (n) }
#[inline(always)]
pub const fn _GL_COB16 (n: isize) -> usize { _GL_COB8 (n >> 8) + _GL_COB8 (n) }
#[inline(always)]
pub const fn _GL_COB8 (n: isize) -> usize { _GL_COB4 (n >> 4) + _GL_COB4 (n) }
/// lib/limits.in.h 87: `(!!((n) & 8) + !!((n) & 4) + !!((n) & 2) + !!((n) & 1))`.
#[inline(always)]
pub const fn _GL_COB4 (n: isize) -> usize {
    (((n & 8) != 0) as usize) + (((n & 4) != 0) as usize)
        + (((n & 2) != 0) as usize) + (((n & 1) != 0) as usize)
}

// CHAR_BIT is the system <limits.h>'s byte width; mini-gmp.c multiplies its type sizes
// by it (GMP_LIMB_BITS, GMP_ULONG_BITS, gmp_umul_ppmm's branch conditions).
pub const CHAR_BIT: c_int = 8;

// <limits.h> values the reference names in its headers; UCHAR_MAX is what
// character.c's `hexdigit[UCHAR_MAX + 1]` definition is sized by (CHAR_BIT is 8
// on the frozen target).
pub const UCHAR_MAX: c_int = 0xFF;

// --- the system <limits.h> this header includes (P-26) ----------------------
//
// gnulib's limits.in.h includes the system <limits.h> (its line 26) and its INT_WIDTH
// line (117) uses INT_MIN/INT_MAX from it; the port reaches them through this mirror.
// Values measured from the frozen target's clang:
// `printf ("%d %d | %lu", INT_MIN, INT_MAX, ULONG_MAX)` -> `-2147483648 2147483647 |
// 18446744073709551615`.  ULONG_MAX is bignum.c's `GMP_NLIMBS_MAX`/`NLIMBS_LIMIT`.
pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;
// mini-gmp.c's fits/compare family names four more (mpz_fits_sint_p, mpz_fits_uint_p,
// mpz_fits_sshort_p, mpz_fits_ushort_p).
pub const UINT_MAX: c_uint = 4294967295;
pub const SHRT_MAX: c_int = 32767;
pub const SHRT_MIN: c_int = -32768;
pub const USHRT_MAX: c_int = 65535;
pub const ULONG_MAX: c_ulong = c_ulong::MAX;
pub const LONG_MIN: c_long = c_long::MIN;
pub const LONG_MAX: c_long = c_long::MAX;

// --- gnulib intprops.h's width family (P-26) --------------------------------
//
// lisp.h 96 reads LONG_WIDTH/ULONG_WIDTH from intprops.h, and bignum.c reads ULONG_WIDTH/
// INTMAX_WIDTH/UINTMAX_WIDTH from it; the port derives them from the types.
/* #define TYPE_WIDTH(t) (sizeof (t) * CHAR_BIT), CHAR_BIT == 8.  */
// limits.in.h 117-120 spells the four machine widths for this target; the port
// derives each from its type, so the mirror stays true on any width.
pub const INT_WIDTH: c_int = (8 * std::mem::size_of::<c_int> ()) as c_int;
pub const UINT_WIDTH: c_int = (8 * std::mem::size_of::<c_uint> ()) as c_int;
pub const LONG_WIDTH: c_int = (8 * std::mem::size_of::<c_long> ()) as c_int;
pub const ULONG_WIDTH: c_int = (8 * std::mem::size_of::<c_ulong> ()) as c_int;
// (`intmax_t`/`uintmax_t` are lisp.rs's aliases for i64/u64; the lib mirrors stand alone.)
pub const INTMAX_WIDTH: c_int = (8 * std::mem::size_of::<i64> ()) as c_int;
pub const UINTMAX_WIDTH: c_int = (8 * std::mem::size_of::<u64> ()) as c_int;
