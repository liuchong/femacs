// femacs: mirror of the frozen target's <float.h>.
#![forbid(unsafe_code)]
//
// gnulib has no `float.in.h` wrapper, so this file mirrors a *system* header directly:
// lisp.h includes <float.h> (lisp.h 31) and the reference reads FLT_RADIX, FLT_MANT_DIG,
// FLT_MAX_EXP, FLT_MIN_EXP and DBL_MAX_10_EXP from it; floatfns.c adds DBL_MANT_DIG and
// DBL_MIN_EXP (double_integer_scale, rescale_for_division).  The values are what clang 21
// reports for the frozen darwin-arm64 target
// (`printf ("%d %d %d %d %d | %d %d\n", FLT_RADIX, FLT_MANT_DIG, FLT_MAX_EXP, FLT_MIN_EXP,
// DBL_MAX_10_EXP, DBL_MANT_DIG, DBL_MIN_EXP, DBL_DIG)` -> `2 24 128 -125 308 | 53 -1021
// 15`), and PORTING.md P-26 tracks the remaining system headers that still need mirrors.
// DBL_DIG is bignum.c's (`mpz_get_d_rounded`); the rest are lisp.h's and floatfns.c's.

// [U1-MODULE-MIRROR]
use super::*;
pub const FLT_RADIX: c_int = 2;
pub const FLT_MANT_DIG: c_int = 24;
pub const FLT_MAX_EXP: c_int = 128;
pub const FLT_MIN_EXP: c_int = -125;
pub const DBL_MAX_10_EXP: c_int = 308;
pub const DBL_MANT_DIG: c_int = 53;
pub const DBL_MIN_EXP: c_int = -1021;
pub const DBL_DIG: c_int = 15;
