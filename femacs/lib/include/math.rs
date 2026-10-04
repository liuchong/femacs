// femacs: mirror of the frozen target's <math.h> (system header, P-26).
//
// gnulib has no `math.in.h` wrapper, so C includes the system header; this file carries
// the parts the ported units use, with the frozen target's values measured from clang 21:
//
//   printf ("%f %d\n", HUGE_VAL, FP_ILOGB0)  ->  inf -2147483648
//
// `isnan`, `isfinite`, `isinf` and `signbit` are *macros* in C's <math.h>, so the port
// keeps them as macros with the same names (PORTING.md 2.2), spelled with the f64
// predicates Rust has; the rest are real library functions.  PORTING.md P-26 tracks the
// system headers that still need mirrors.

pub const HUGE_VAL: c_double = c_double::INFINITY;
/// FP_ILOGB0 is INT_MIN on this target (floatfns.c's HAIKU block is the only user).
pub const FP_ILOGB0: c_int = -2147483648;

macro_rules! isnan {
    ($x:expr) => { ($x).is_nan () };
}
macro_rules! isfinite {
    ($x:expr) => { ($x).is_finite () };
}
macro_rules! isinf {
    ($x:expr) => { ($x).is_infinite () };
}
/// C's `signbit`: the same predicate as `is_sign_negative`, *including* the sign of a
/// NaN — which is why floatfns.c's `#ifndef signbit` fallback must not be the live one
/// (docs/ports/floatfns.md records the difference).
#[cfg(not(feature = "no_signbit"))]
macro_rules! signbit {
    ($x:expr) => { ($x).is_sign_negative () };
}

/// The `<math.h>` prototypes.  They sit in a module of their own with a glob import
/// (PORTING.md 0.1-10, the composite.rs/charset.rs shape) so that a unit which *defines* one
/// of them shadows the declaration: `floatfns.c` defines `trunc` when the target has none,
/// and C's gnulib `<math.h>` declares it either way.
pub mod math_decls {
  use super::*;

  unsafe extern "C" {
  pub fn acos (x: c_double) -> c_double;
  pub fn asin (x: c_double) -> c_double;
  pub fn atan (x: c_double) -> c_double;
  pub fn atan2 (y: c_double, x: c_double) -> c_double;
  pub fn cos (x: c_double) -> c_double;
  pub fn sin (x: c_double) -> c_double;
  pub fn tan (x: c_double) -> c_double;
  pub fn ceil (x: c_double) -> c_double;
  pub fn floor (x: c_double) -> c_double;
  pub fn fmod (x: c_double, y: c_double) -> c_double;
  pub fn frexp (x: c_double, e: *mut c_int) -> c_double;
  pub fn ldexp (x: c_double, e: c_int) -> c_double;
  pub fn exp (x: c_double) -> c_double;
  pub fn pow (x: c_double, y: c_double) -> c_double;
  pub fn log (x: c_double) -> c_double;
  pub fn log10 (x: c_double) -> c_double;
  pub fn log2 (x: c_double) -> c_double;
  pub fn sqrt (x: c_double) -> c_double;
  pub fn scalbn (x: c_double, n: c_int) -> c_double;
  pub fn ilogb (x: c_double) -> c_int;
  pub fn rint (x: c_double) -> c_double;
  /* C defines its own `trunc` in floatfns.c when the target has none (HAVE_TRUNC); the
     declaration is here in **both** configurations, the way C's gnulib `<math.h>` declares
     it, and `floatfns.rs`'s own definition shadows it through this module's glob import
     (PORTING.md 0.1-10; status 3.104).  */
  pub fn trunc (x: c_double) -> c_double;
  }
}
use math_decls::*;
