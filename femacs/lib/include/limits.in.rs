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

// <limits.h> values the reference names in its headers; UCHAR_MAX is what
// character.c's `hexdigit[UCHAR_MAX + 1]` definition is sized by (CHAR_BIT is 8
// on the frozen target).
pub const UCHAR_MAX: c_int = 0xFF;

// --- the system <limits.h> this header includes (P-26) ----------------------
//
// gnulib's limits.in.h includes the system <limits.h> (its line 26) and its INT_WIDTH
// line (117) uses INT_MIN/INT_MAX from it; the port reaches them through this mirror.
// Values measured from the frozen target's clang:
// `printf ("%d %d", INT_MIN, INT_MAX)` -> `-2147483648 2147483647`.
pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;
