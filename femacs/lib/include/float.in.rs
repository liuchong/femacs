// femacs: mirror of the frozen target's <float.h>.
//
// gnulib has no `float.in.h` wrapper, so this file mirrors a *system* header directly:
// lisp.h includes <float.h> (lisp.h 31) and the reference reads FLT_RADIX, FLT_MANT_DIG,
// FLT_MAX_EXP, FLT_MIN_EXP and DBL_MAX_10_EXP from it.  The values are what clang 21
// reports for the frozen darwin-arm64 target
// (`printf ("%d %d %d %d %d\n", FLT_RADIX, FLT_MANT_DIG, FLT_MAX_EXP, FLT_MIN_EXP,
// DBL_MAX_10_EXP)` -> `2 24 128 -125 308`), and PORTING.md P-26 tracks the remaining
// system headers that still need mirrors.

pub const FLT_RADIX: c_int = 2;
pub const FLT_MANT_DIG: c_int = 24;
pub const FLT_MAX_EXP: c_int = 128;
pub const FLT_MIN_EXP: c_int = -125;
pub const DBL_MAX_10_EXP: c_int = 308;
