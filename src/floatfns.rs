// femacs: port of src/floatfns.c @ fd7afced403.
//
// Logic lives in `#![forbid(unsafe_code)]` `crate::safe_floatfns` (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = core::assert! (!cfg! (feature = "no_signbit") || !HAVE_SIGNBIT);
const _: () = core::assert! (!cfg! (feature = "have_haiku") || HAIKU);
const _: () = core::assert! (!cfg! (feature = "no_rint") || !HAVE_RINT);
const _: () = core::assert! (!cfg! (feature = "no_trunc") || !HAVE_TRUNC);

#[unsafe(no_mangle)]
pub extern "C-unwind" fn extract_float (num: Lisp_Object) -> c_double
{
  crate::safe_floatfns::extract_float_impl (num)
}

DEFUN! ("acos", Facos, Sacos, 1, 1, 0,
       doc: "Return the inverse cosine of ARG.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::acos_subr (arg)
});

DEFUN! ("asin", Fasin, Sasin, 1, 1, 0,
       doc: "Return the inverse sine of ARG.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::asin_subr (arg)
});

DEFUN! ("atan", Fatan, Satan, 1, 2, 0,
       doc: "Return the inverse tangent of the arguments.\nIf only one argument Y is given, return the inverse tangent of Y.\nIf two arguments Y and X are given, return the inverse tangent of Y\ndivided by X, i.e. the angle in radians between the vector (X, Y)\nand the x-axis.",
       (y: Lisp_Object, x: Lisp_Object),
       {
  crate::safe_floatfns::atan_subr (y, x)
});

DEFUN! ("cos", Fcos, Scos, 1, 1, 0,
       doc: "Return the cosine of ARG.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::cos_subr (arg)
});

DEFUN! ("sin", Fsin, Ssin, 1, 1, 0,
       doc: "Return the sine of ARG.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::sin_subr (arg)
});

DEFUN! ("tan", Ftan, Stan, 1, 1, 0,
       doc: "Return the tangent of ARG.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::tan_subr (arg)
});

DEFUN! ("isnan", Fisnan, Sisnan, 1, 1, 0,
       doc: "Return non-nil if argument X is a NaN.",
       (x: Lisp_Object),
       {
  crate::safe_floatfns::isnan_subr (x)
});

DEFUN! ("copysign", Fcopysign, Scopysign, 2, 2, 0,
       doc: "Copy sign of X2 to value of X1, and return the result.\nCause an error if X1 or X2 is not a float.",
       (x1: Lisp_Object, x2: Lisp_Object),
       {
  crate::safe_floatfns::copysign_subr (x1, x2)
});

DEFUN! ("frexp", Ffrexp, Sfrexp, 1, 1, 0,
       doc: "Get significand and exponent of a floating point number.\nBreaks the floating point number X into its binary significand SGNFCAND\n(a floating point value between 0.5 (included) and 1.0 (excluded))\nand an integral exponent EXP for 2, such that:\n\n  X = SGNFCAND * 2^EXP\n\nThe function returns the cons cell (SGNFCAND . EXP).\nIf X is zero, both parts (SGNFCAND and EXP) are zero.",
       (x: Lisp_Object),
       {
  crate::safe_floatfns::frexp_subr (x)
});

DEFUN! ("ldexp", Fldexp, Sldexp, 2, 2, 0,
       doc: "Return SGNFCAND * 2**EXPONENT, as a floating point number.\nEXPONENT must be an integer.",
       (sgnfcand: Lisp_Object, exponent: Lisp_Object),
       {
  crate::safe_floatfns::ldexp_subr (sgnfcand, exponent)
});

DEFUN! ("exp", Fexp, Sexp, 1, 1, 0,
       doc: "Return the exponential base e of ARG.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::exp_subr (arg)
});

DEFUN! ("expt", Fexpt, Sexpt, 2, 2, 0,
       doc: "Return the exponential ARG1 ** ARG2.",
       (arg1: Lisp_Object, arg2: Lisp_Object),
       {
  crate::safe_floatfns::expt_subr (arg1, arg2)
});

DEFUN! ("log", Flog, Slog, 1, 2, 0,
       doc: "Return the natural logarithm of ARG.\nIf the optional argument BASE is given, return log ARG using that base.",
       (arg: Lisp_Object, base: Lisp_Object),
       {
  crate::safe_floatfns::log_subr (arg, base)
});

DEFUN! ("sqrt", Fsqrt, Ssqrt, 1, 1, 0,
       doc: "Return the square root of ARG.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::sqrt_subr (arg)
});

DEFUN! ("abs", Fabs, Sabs, 1, 1, 0,
       doc: "Return the absolute value of ARG.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::abs_subr (arg)
});

DEFUN! ("float", Ffloat, Sfloat, 1, 1, 0,
       doc: "Return the floating point number equal to ARG.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::float_subr (arg)
});

DEFUN! ("logb", Flogb, Slogb, 1, 1, 0,
       doc: "Returns largest integer <= the base 2 log of the magnitude of ARG.\nThis is the same as the exponent of a float.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::logb_subr (arg)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn double_integer_scale (d: c_double) -> c_int
{
  crate::safe_floatfns::double_integer_scale_impl (d)
}

#[cfg(feature = "no_trunc")]
pub extern "C" fn trunc (d: c_double) -> c_double
{
  if d < 0.0 { ceil (d) } else { floor (d) }
}

DEFUN! ("ceiling", Fceiling, Sceiling, 1, 2, 0,
       doc: "Return the smallest integer no less than ARG.\nThis rounds the value towards +inf.\nWith optional DIVISOR, return the smallest integer no less than ARG/DIVISOR.",
       (arg: Lisp_Object, divisor: Lisp_Object),
       {
  crate::safe_floatfns::ceiling_subr (arg, divisor)
});

DEFUN! ("floor", Ffloor, Sfloor, 1, 2, 0,
       doc: "Return the largest integer no greater than ARG.\nThis rounds the value towards -inf.\nWith optional DIVISOR, return the largest integer no greater than ARG/DIVISOR.",
       (arg: Lisp_Object, divisor: Lisp_Object),
       {
  crate::safe_floatfns::floor_subr (arg, divisor)
});

DEFUN! ("round", Fround, Sround, 1, 2, 0,
       doc: "Return the nearest integer to ARG.\nWith optional DIVISOR, return the nearest integer to ARG/DIVISOR.\n\nRounding a value equidistant between two integers may choose the\ninteger closer to zero, or it may prefer an even integer, depending on\nyour machine.  For example, (round 2.5) can return 3 on some\nsystems, but 2 on others.",
       (arg: Lisp_Object, divisor: Lisp_Object),
       {
  crate::safe_floatfns::round_subr (arg, divisor)
});

DEFUN! ("truncate", Ftruncate, Struncate, 1, 2, 0,
       doc: "Truncate a floating point number to an int.\nRounds ARG toward zero.\nWith optional DIVISOR, truncate ARG/DIVISOR.",
       (arg: Lisp_Object, divisor: Lisp_Object),
       {
  crate::safe_floatfns::truncate_subr (arg, divisor)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn fmod_float (x: Lisp_Object, y: Lisp_Object) -> Lisp_Object
{
  crate::safe_floatfns::fmod_float_impl (x, y)
}

DEFUN! ("fceiling", Ffceiling, Sfceiling, 1, 1, 0,
       doc: "Return the smallest integer no less than ARG, as a float.\n(Round toward +inf.)",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::fceiling_subr (arg)
});

DEFUN! ("ffloor", Fffloor, Sffloor, 1, 1, 0,
       doc: "Return the largest integer no greater than ARG, as a float.\n(Round toward -inf.)",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::ffloor_subr (arg)
});

DEFUN! ("fround", Ffround, Sfround, 1, 1, 0,
       doc: "Return the nearest integer to ARG, as a float.",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::fround_subr (arg)
});

DEFUN! ("ftruncate", Fftruncate, Sftruncate, 1, 1, 0,
       doc: "Truncate a floating point number to an integral float value.\n(Round toward zero.)",
       (arg: Lisp_Object),
       {
  crate::safe_floatfns::ftruncate_subr (arg)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_floatfns ()
{
  defsubr_global (&Sacos);
  defsubr_global (&Sasin);
  defsubr_global (&Satan);
  defsubr_global (&Scos);
  defsubr_global (&Ssin);
  defsubr_global (&Stan);
  defsubr_global (&Sisnan);
  defsubr_global (&Scopysign);
  defsubr_global (&Sfrexp);
  defsubr_global (&Sldexp);
  defsubr_global (&Sfceiling);
  defsubr_global (&Sffloor);
  defsubr_global (&Sfround);
  defsubr_global (&Sftruncate);
  defsubr_global (&Sexp);
  defsubr_global (&Sexpt);
  defsubr_global (&Slog);
  defsubr_global (&Ssqrt);

  defsubr_global (&Sabs);
  defsubr_global (&Sfloat);
  defsubr_global (&Slogb);
  defsubr_global (&Sceiling);
  defsubr_global (&Sfloor);
  defsubr_global (&Sround);
  defsubr_global (&Struncate);
}
