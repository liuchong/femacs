// femacs: port of src/floatfns.c @ fd7afced403.
//
// One-to-one port: same statements in the same order, same names (PORTING.md).  The
// reference's `#if`s are the frozen configuration's branches (P-08): HAVE_LOG2/
// HAVE_RINT/HAVE_TRUNC are 1/1/1 and HAIKU is undefined (docs/baseline/config/*.json),
// so the arms gated on `feature = "no_rint"`/`"no_trunc"`/`"have_haiku"` below are the
// ones C leaves out.  `<math.h>`/`<limits.h>` come from the system mirrors
// lib/include/math.rs and limits.in.rs (P-26); `mpz_t` values are passed as
// `…as_mut_ptr ()`/`…as_ptr ()` because the port's `mpz_t` is C's array-of-one typedef
// (P-49, bignum.rs).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

/* C89 requires only the following math.h functions, and Emacs omits
   the starred functions since we haven't found a use for them:
   acos, asin, atan, atan2, ceil, cos, *cosh, exp, fabs, floor, fmod,
   frexp, ldexp, log, log10 [via (log X 10)], *modf, pow, sin, *sinh,
   sqrt, tan, *tanh.

   C99, C11 and C17 require the following math.h functions in addition to
   the C89 functions.  Of these, Emacs currently exports only the
   starred ones to Lisp, since we haven't found a use for the others.
   Also, it uses the ones marked "+" internally:
   acosh, atanh, cbrt, copysign (implemented by signbit), erf, erfc,
   exp2, expm1, fdim, fma, fmax, fmin, fpclassify, hypot, +ilogb,
   +isfinite, isgreater, isgreaterequal, +isinf, isless, islessequal,
   islessgreater, *isnan, isnormal, isunordered, lgamma, log1p, *log2
   [via (log X 2)], logb (approximately; implemented by frexp),
   +lrint/llrint, +lround/llround, nan, nearbyint, nextafter,
   nexttoward, remainder, remquo, *rint, round, scalbln, +scalbn,
   +signbit, tgamma, *trunc.

   C23 requires many more math.h functions.  Emacs does not yet export
   or use them.

   The C standard also requires functions for float and long double
   that are not listed above.  Of these functions, Emacs uses only the
   following internally: fabsf, powf, sprintf.
 */

// [U1] config.rs provided by common
// [U1] ../lib/include/stdio.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] ../lib/include/mini-gmp.rs imported via common
// [U1] include/bignum.rs imported via common
// [U1] ../lib/include/math.rs imported via common

/* Emacs needs proper handling of +/-inf; correct printing as well as
   important packages depend on it.  Make sure the user didn't specify
   -ffinite-math-only, either directly or implicitly with -Ofast or
   -ffast-math.  */
// C: `#if defined __FINITE_MATH_ONLY__ && __FINITE_MATH_ONLY__ #error … #endif` — a check
// on the C compiler's flags, which has no Rust counterpart (docs/ports/floatfns.md).

/* Check that X is a floating point number.  */

unsafe fn CHECK_FLOAT (x: Lisp_Object)
{
  CHECK_TYPE (FLOATP (x), Qfloatp (), x);
}

/* Extract a Lisp number as a `double', or signal an error.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn extract_float (num: Lisp_Object) -> c_double
{
  CHECK_NUMBER (num);
  return XFLOATINT (num);
}

/* Trig functions.  */

DEFUN! ("acos", Facos, Sacos, 1, 1, 0,
       doc: "Return the inverse cosine of ARG.",
       (arg: Lisp_Object),
       {
  let mut d: c_double = extract_float (arg);
  d = acos (d);
  return make_float (d);
});

DEFUN! ("asin", Fasin, Sasin, 1, 1, 0,
       doc: "Return the inverse sine of ARG.",
       (arg: Lisp_Object),
       {
  let mut d: c_double = extract_float (arg);
  d = asin (d);
  return make_float (d);
});

DEFUN! ("atan", Fatan, Satan, 1, 2, 0,
       doc: "Return the inverse tangent of the arguments.\nIf only one argument Y is given, return the inverse tangent of Y.\nIf two arguments Y and X are given, return the inverse tangent of Y\ndivided by X, i.e. the angle in radians between the vector (X, Y)\nand the x-axis.",
       (y: Lisp_Object, x: Lisp_Object),
       {
  let mut d: c_double = extract_float (y);

  if NILP (x)
    { d = atan (d); }
  else
    {
      let d2: c_double = extract_float (x);
      d = atan2 (d, d2);
    }
  return make_float (d);
});

DEFUN! ("cos", Fcos, Scos, 1, 1, 0,
       doc: "Return the cosine of ARG.",
       (arg: Lisp_Object),
       {
  let mut d: c_double = extract_float (arg);
  d = cos (d);
  return make_float (d);
});

DEFUN! ("sin", Fsin, Ssin, 1, 1, 0,
       doc: "Return the sine of ARG.",
       (arg: Lisp_Object),
       {
  let mut d: c_double = extract_float (arg);
  d = sin (d);
  return make_float (d);
});

DEFUN! ("tan", Ftan, Stan, 1, 1, 0,
       doc: "Return the tangent of ARG.",
       (arg: Lisp_Object),
       {
  let mut d: c_double = extract_float (arg);
  d = tan (d);
  return make_float (d);
});

DEFUN! ("isnan", Fisnan, Sisnan, 1, 1, 0,
       doc: "Return non-nil if argument X is a NaN.",
       (x: Lisp_Object),
       {
  CHECK_FLOAT (x);
  return if isnan! (XFLOAT_DATA (x)) { Qt () } else { Qnil () };
});

/* Although the substitute does not work on NaNs, it is good enough
   for platforms lacking the signbit macro.  */
// C: `#ifndef signbit` / `# define signbit(x) ((x) < 0 || (IEEE_FLOATING_POINT && !(x)
// && 1 / (x) < 0))`.  The frozen target's <math.h> *does* define signbit, so this
// fallback is inactive and the port keeps it in the same place under the same condition;
// the live definition is the `signbit` macro above (P-08, P-26).
#[cfg(feature = "no_signbit")]
macro_rules! signbit {
    ($x:expr) => {
        // C's `!(x)` on a double is `(x) == 0`; Rust's `!` is bitwise and does not
        // exist for f64 (review B's B3: this arm did not even compile).
        (($x) < 0.0 || (IEEE_FLOATING_POINT && ($x) == 0.0 && 1.0 / ($x) < 0.0))
    };
}

const _: () = core::assert! (!cfg! (feature = "no_signbit") || !HAVE_SIGNBIT);

DEFUN! ("copysign", Fcopysign, Scopysign, 2, 2, 0,
       doc: "Copy sign of X2 to value of X1, and return the result.\nCause an error if X1 or X2 is not a float.",
       (x1: Lisp_Object, x2: Lisp_Object),
       {
  let f1: c_double;
  let f2: c_double;

  CHECK_FLOAT (x1);
  CHECK_FLOAT (x2);

  f1 = XFLOAT_DATA (x1);
  f2 = XFLOAT_DATA (x2);

  /* Use signbit instead of copysign, to avoid calling make_float when
     the result is X1.  */
  return if signbit! (f1) != signbit! (f2) { make_float (-f1) } else { x1 };
});

DEFUN! ("frexp", Ffrexp, Sfrexp, 1, 1, 0,
       doc: "Get significand and exponent of a floating point number.\nBreaks the floating point number X into its binary significand SGNFCAND\n(a floating point value between 0.5 (included) and 1.0 (excluded))\nand an integral exponent EXP for 2, such that:\n\n  X = SGNFCAND * 2^EXP\n\nThe function returns the cons cell (SGNFCAND . EXP).\nIf X is zero, both parts (SGNFCAND and EXP) are zero.",
       (x: Lisp_Object),
       {
  let f: c_double = extract_float (x);
  // C leaves `exponent` uninitialised and takes its address; Rust needs a value to
  // point at, and `frexp` writes it in every path (docs/ports/floatfns.md).
  let mut exponent: c_int = 0;
  let sgnfcand: c_double = frexp (f, &mut exponent);
  return Fcons (make_float (sgnfcand), make_fixnum (exponent as EMACS_INT));
});

DEFUN! ("ldexp", Fldexp, Sldexp, 2, 2, 0,
       doc: "Return SGNFCAND * 2**EXPONENT, as a floating point number.\nEXPONENT must be an integer.",
       (sgnfcand: Lisp_Object, exponent: Lisp_Object),
       {
  CHECK_FIXNUM (exponent);
  let e: c_int = min (max (INT_MIN as isize, XFIXNUM (exponent)),
                      INT_MAX as isize) as c_int;
  return make_float (ldexp (extract_float (sgnfcand), e));
});

DEFUN! ("exp", Fexp, Sexp, 1, 1, 0,
       doc: "Return the exponential base e of ARG.",
       (arg: Lisp_Object),
       {
  let mut d: c_double = extract_float (arg);
  d = exp (d);
  return make_float (d);
});

DEFUN! ("expt", Fexpt, Sexpt, 2, 2, 0,
       doc: "Return the exponential ARG1 ** ARG2.",
       (arg1: Lisp_Object, arg2: Lisp_Object),
       {
  CHECK_NUMBER (arg1);
  CHECK_NUMBER (arg2);

  /* Common Lisp spec: don't promote if both are integers, and if the
     result is not fractional.  */
  if INTEGERP (arg1) && !NILP (Fnatnump (arg2))
    { return expt_integer (arg1, arg2); }

  return make_float (pow (XFLOATINT (arg1), XFLOATINT (arg2)));
});

DEFUN! ("log", Flog, Slog, 1, 2, 0,
       doc: "Return the natural logarithm of ARG.\nIf the optional argument BASE is given, return log ARG using that base.",
       (arg: Lisp_Object, base: Lisp_Object),
       {
  let mut d: c_double = extract_float (arg);

  if NILP (base)
    { d = log (d); }
  else
    {
      let b: c_double = extract_float (base);

      if b == 10.0
	{ d = log10 (d); }
      // C: `#if HAVE_LOG2 else if (b == 2.0) d = log2 (d); #endif` — HAVE_LOG2 is 1 in
      // the frozen configuration (P-08), and the constant keeps the arm compiled away
      // when it is 0, which is what the preprocessor would do.
      else if HAVE_LOG2 && b == 2.0
	{ d = log2 (d); }
      else
	{ d = log (d) / log (b); }
    }
  return make_float (d);
});

DEFUN! ("sqrt", Fsqrt, Ssqrt, 1, 1, 0,
       doc: "Return the square root of ARG.",
       (arg: Lisp_Object),
       {
  let mut d: c_double = extract_float (arg);
  d = sqrt (d);
  return make_float (d);
});

DEFUN! ("abs", Fabs, Sabs, 1, 1, 0,
       doc: "Return the absolute value of ARG.",
       (arg: Lisp_Object),
       {
  let mut arg = arg;
  CHECK_NUMBER (arg);

  if FIXNUMP (arg)
    {
      if XFIXNUM (arg) < 0
	{ arg = make_int (-XFIXNUM (arg) as intmax_t); }
    }
  else if FLOATP (arg)
    {
      if signbit! (XFLOAT_DATA (arg))
	{ arg = make_float (-XFLOAT_DATA (arg)); }
    }
  else
    {
      if mpz_sgn (xbignum_val (arg)) < 0
	{
	  mpz_neg (mpz[0].as_mut_ptr (), xbignum_val (arg));
	  arg = make_integer_mpz ();
	}
    }

  return arg;
});

DEFUN! ("float", Ffloat, Sfloat, 1, 1, 0,
       doc: "Return the floating point number equal to ARG.",
       (arg: Lisp_Object),
       {
  CHECK_NUMBER (arg);
  /* If ARG is a float, give 'em the same float back.  */
  return if FLOATP (arg) { arg } else { make_float (XFLOATINT (arg)) };
});

DEFUN! ("logb", Flogb, Slogb, 1, 1, 0,
       doc: "Returns largest integer <= the base 2 log of the magnitude of ARG.\nThis is the same as the exponent of a float.",
       (arg: Lisp_Object),
       {
  let mut value: EMACS_INT = 0;
  CHECK_NUMBER (arg);

  if FLOATP (arg)
    {
      let f: c_double = XFLOAT_DATA (arg);
      if f == 0.0
	{ return make_float (-HUGE_VAL); }
      if !isfinite! (f)
	{ return if f < 0.0 { make_float (-f) } else { arg }; }
      let mut ivalue: c_int = 0;
      frexp (f, &mut ivalue);
      value = ivalue as EMACS_INT - 1;
    }
  else if !FIXNUMP (arg)
    { value = mpz_sizeinbase (xbignum_val (arg), 2) as EMACS_INT - 1; }
  else
    {
      let i: EMACS_INT = XFIXNUM (arg);
      if i == 0
	{ return make_float (-HUGE_VAL); }
      value = elogb (eabs! (i) as u64) as EMACS_INT;
    }

  return make_fixnum (value);
});

/* Return the integer exponent E such that D * FLT_RADIX**E (i.e.,
   scalbn (D, E)) is an integer that has precision equal to D and is
   representable as a double.

   Return DBL_MANT_DIG - DBL_MIN_EXP (the maximum possible valid
   scale) if D is zero or tiny.  Return one greater than that if
   D is infinite, and two greater than that if D is a NaN.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn double_integer_scale (d: c_double) -> c_int
{
  let exponent = ilogb (d);
  // C gates this block on `#ifdef HAIKU`; HAIKU is undefined in the frozen
  // configuration, so the port keeps the text under the same condition (P-08).
  #[cfg(feature = "have_haiku")]
  {
    /* On Haiku, the values returned by ilogb are nonsensical when
       confronted with tiny numbers, inf, or NaN, which breaks the trick
       used by code on other platforms, so we have to test for each case
       manually, and return the appropriate value.  */
    if exponent == FP_ILOGB0
      {
	if isnan! (d)
	  { return (DBL_MANT_DIG - DBL_MIN_EXP) + 2; }
	if isinf! (d)
	  { return (DBL_MANT_DIG - DBL_MIN_EXP) + 1; }

	return (DBL_MANT_DIG - DBL_MIN_EXP);
      }
  }
  return if DBL_MIN_EXP - 1 <= exponent && exponent < INT_MAX
    { DBL_MANT_DIG - 1 - exponent }
    else
      { DBL_MANT_DIG - DBL_MIN_EXP
	  + if isnan! (d) { 2 } else { (exponent == INT_MAX) as c_int } };
}

const _: () = core::assert! (!cfg! (feature = "have_haiku") || HAIKU);

/* Convert the Lisp number N to an integer and return a pointer to the
   converted integer, represented as an mpz_t *.  Use *T as a
   temporary; the returned value might be T.  Scale N by the maximum
   of NSCALE and DSCALE while converting.  If NSCALE is nonzero, N
   must be a float; signal an overflow if NSCALE is greater than
   DBL_MANT_DIG - DBL_MIN_EXP, otherwise scalbn (XFLOAT_DATA (N), NSCALE)
   must return an integer value, without rounding or overflow.  */

unsafe fn rescale_for_division (n: Lisp_Object, t: *mut mpz_t, nscale: c_int,
				dscale: c_int) -> *const __mpz_struct
{
  let mut pn: *const __mpz_struct;

  if FLOATP (n)
    {
      if DBL_MANT_DIG - DBL_MIN_EXP < nscale
	{ overflow_error (); }
      mpz_set_d ((*t).as_mut_ptr (), scalbn (XFLOAT_DATA (n), nscale));
      pn = (*t).as_ptr ();
    }
  else
    { pn = bignum_integer (t, n); }

  if nscale < dscale
    {
      emacs_mpz_mul_2exp ((*t).as_mut_ptr (), pn,
			  ((dscale - nscale) * LOG2_FLT_RADIX) as EMACS_INT);
      pn = (*t).as_ptr ();
    }
  return pn;
}

/* the rounding functions  */

unsafe fn rounding_driver (n: Lisp_Object, d: Lisp_Object,
			   double_round: unsafe extern "C" fn (c_double) -> c_double,
			   int_divide: unsafe extern "C" fn (*mut __mpz_struct,
							     *const __mpz_struct,
							     *const __mpz_struct),
			   fixnum_divide: unsafe extern "C" fn (EMACS_INT, EMACS_INT) -> EMACS_INT)
     -> Lisp_Object
{
  CHECK_NUMBER (n);

  if NILP (d)
    { return if FLOATP (n) { double_to_integer (double_round (XFLOAT_DATA (n))) } else { n }; }

  CHECK_NUMBER (d);

  let mut dscale: c_int = 0;
  if FIXNUMP (d)
    {
      if XFIXNUM (d) == 0
	{ xsignal0 (Qarith_error ()); }

      /* Divide fixnum by fixnum specially, for speed.  */
      if FIXNUMP (n)
	{ return make_int (fixnum_divide (XFIXNUM (n), XFIXNUM (d)) as intmax_t); }
    }
  else if FLOATP (d)
    {
      if XFLOAT_DATA (d) == 0.0
	{ xsignal0 (Qarith_error ()); }
      dscale = double_integer_scale (XFLOAT_DATA (d));
    }

  let nscale: c_int = if FLOATP (n) { double_integer_scale (XFLOAT_DATA (n)) } else { 0 };

  /* If the numerator is finite and the denominator infinite, the
     quotient is zero and there is no need to try the impossible task
     of rescaling the denominator.  */
  if dscale == DBL_MANT_DIG - DBL_MIN_EXP + 1 && nscale < dscale
    { return make_fixnum (0); }

  int_divide (mpz[0].as_mut_ptr (),
	      rescale_for_division (n, &raw mut mpz[0], nscale, dscale),
	      rescale_for_division (d, &raw mut mpz[1], dscale, nscale));
  return make_integer_mpz ();
}

unsafe extern "C" fn ceiling2 (n: EMACS_INT, d: EMACS_INT) -> EMACS_INT
{
  return n / d + ((n % d != 0) as EMACS_INT & ((n < 0) == (d < 0)) as EMACS_INT);
}

unsafe extern "C" fn floor2 (n: EMACS_INT, d: EMACS_INT) -> EMACS_INT
{
  return n / d - ((n % d != 0) as EMACS_INT & ((n < 0) != (d < 0)) as EMACS_INT);
}

unsafe extern "C" fn truncate2 (n: EMACS_INT, d: EMACS_INT) -> EMACS_INT
{
  return n / d;
}

unsafe extern "C" fn round2 (n: EMACS_INT, d: EMACS_INT) -> EMACS_INT
{
  /* The C language's division operator gives us the remainder R
     corresponding to truncated division, but we want the remainder R1
     on the other side of 0 if R1 is closer to 0 than R is; because we
     want to round to even, we also want R1 if R and R1 are the same
     distance from 0 and if the truncated quotient is odd.  */
  let mut q = n / d;
  let r = n % d;
  let neg_d = d < 0;
  let neg_r = r < 0;
  let abs_r = eabs! (r);
  let abs_r1 = eabs! (d) - abs_r;
  if abs_r1 < abs_r + (q & 1)
    { q += if neg_d == neg_r { 1 } else { -1 }; }
  return q;
}

unsafe extern "C" fn rounddiv_q (q: *mut __mpz_struct, n: *const __mpz_struct,
			      d: *const __mpz_struct)
{
  /* Mimic the source code of round2, using mpz_t instead of EMACS_INT.  */
  let r: *mut mpz_t = &raw mut mpz[2];
  let abs_r: *mut mpz_t = r;
  let abs_r1: *mut mpz_t = &raw mut mpz[3];
  mpz_tdiv_qr (q, (*r).as_mut_ptr (), n, d);
  let neg_d = mpz_sgn (d) < 0;
  let neg_r = mpz_sgn ((*r).as_ptr ()) < 0;
  mpz_abs ((*abs_r).as_mut_ptr (), (*r).as_ptr ());
  mpz_abs ((*abs_r1).as_mut_ptr (), d);
  mpz_sub ((*abs_r1).as_mut_ptr (), (*abs_r1).as_ptr (), (*abs_r).as_ptr ());
  if mpz_cmp ((*abs_r1).as_ptr (), (*abs_r).as_ptr ()) < (mpz_odd_p! (q) != 0) as c_int
    { if neg_d == neg_r { mpz_add_ui (q, q, 1) } else { mpz_sub_ui (q, q, 1) } }
}

/* The code uses emacs_rint, so that it works to undefine HAVE_RINT
   if `rint' exists but does not work right.  */
// C: `#ifdef HAVE_RINT #define emacs_rint rint #else static double emacs_rint (…)
// #endif`.  HAVE_RINT is 1 in the frozen configuration (P-08), so C's `emacs_rint (d)`
// *is* `rint (d)` and the function below is dead; the port keeps both arms.
#[cfg(feature = "no_rint")]
unsafe extern "C" fn emacs_rint (d: c_double) -> c_double
{
  let d1 = d + 0.5;
  let r = floor (d1);
  return r - ((r == d1 && fmod (r, 2.0) != 0.0) as c_int as c_double);
}

#[cfg(not(feature = "no_rint"))]
macro_rules! emacs_rint {
    ($d:expr) => { rint ($d) };
}

const _: () = core::assert! (!cfg! (feature = "no_rint") || !HAVE_RINT);

// C: `#ifndef HAVE_TRUNC double trunc (double d) { return (d < 0 ? ceil : floor) (d); }
// #endif` — HAVE_TRUNC is 1 in the frozen configuration, so the <math.h> function is
// the live one and `feature = "no_trunc"` selects this definition instead.
/* No `#[unsafe(no_mangle)]` here: the frozen configuration decides `#ifndef HAVE_TRUNC` false,
   so the reference defines no `trunc` symbol and the port's dead text must not invent one
   (docs/status.md 3.26 records the class - filemode.c's `strmode` was the first).  */
#[cfg(feature = "no_trunc")]
pub unsafe extern "C" fn trunc (d: c_double) -> c_double
{
  return if d < 0.0 { ceil (d) } else { floor (d) };
}

const _: () = core::assert! (!cfg! (feature = "no_trunc") || !HAVE_TRUNC);

DEFUN! ("ceiling", Fceiling, Sceiling, 1, 2, 0,
       doc: "Return the smallest integer no less than ARG.\nThis rounds the value towards +inf.\nWith optional DIVISOR, return the smallest integer no less than ARG/DIVISOR.",
       (arg: Lisp_Object, divisor: Lisp_Object),
       {
  return rounding_driver (arg, divisor, ceil, mpz_cdiv_q, ceiling2);
});

DEFUN! ("floor", Ffloor, Sfloor, 1, 2, 0,
       doc: "Return the largest integer no greater than ARG.\nThis rounds the value towards -inf.\nWith optional DIVISOR, return the largest integer no greater than ARG/DIVISOR.",
       (arg: Lisp_Object, divisor: Lisp_Object),
       {
  return rounding_driver (arg, divisor, floor, mpz_fdiv_q, floor2);
});

DEFUN! ("round", Fround, Sround, 1, 2, 0,
       doc: "Return the nearest integer to ARG.\nWith optional DIVISOR, return the nearest integer to ARG/DIVISOR.\n\nRounding a value equidistant between two integers may choose the\ninteger closer to zero, or it may prefer an even integer, depending on\nyour machine.  For example, (round 2.5) can return 3 on some\nsystems, but 2 on others.",
       (arg: Lisp_Object, divisor: Lisp_Object),
       {
  // C: `rounding_driver (arg, divisor, emacs_rint, rounddiv_q, round2)`, where
  // `emacs_rint` is the macro `rint` because HAVE_RINT is 1 (P-08); the other arm is C's
  // `#else` function, which stays dead unless the configuration lacks rint.
  #[cfg(not(feature = "no_rint"))]
  { return rounding_driver (arg, divisor, rint, rounddiv_q, round2); }
  #[cfg(feature = "no_rint")]
  { return rounding_driver (arg, divisor, emacs_rint, rounddiv_q, round2); }
});

/* Since rounding_driver truncates anyway, no need to call 'trunc'.  */
unsafe extern "C" fn identity (x: c_double) -> c_double
{
  return x;
}

DEFUN! ("truncate", Ftruncate, Struncate, 1, 2, 0,
       doc: "Truncate a floating point number to an int.\nRounds ARG toward zero.\nWith optional DIVISOR, truncate ARG/DIVISOR.",
       (arg: Lisp_Object, divisor: Lisp_Object),
       {
  return rounding_driver (arg, divisor, identity, mpz_tdiv_q, truncate2);
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmod_float (x: Lisp_Object, y: Lisp_Object) -> Lisp_Object
{
  let mut f1: c_double = XFLOATINT (x);
  let f2: c_double = XFLOATINT (y);

  f1 = fmod (f1, f2);

  /* If the "remainder" comes out with the wrong sign, fix it.  */
  if if f2 < 0.0 { f1 > 0.0 } else { f1 < 0.0 }
    { f1 += f2; }

  return make_float (f1);
}

DEFUN! ("fceiling", Ffceiling, Sfceiling, 1, 1, 0,
       doc: "Return the smallest integer no less than ARG, as a float.\n(Round toward +inf.)",
       (arg: Lisp_Object),
       {
  CHECK_FLOAT (arg);
  let mut d: c_double = XFLOAT_DATA (arg);
  d = ceil (d);
  return make_float (d);
});

DEFUN! ("ffloor", Fffloor, Sffloor, 1, 1, 0,
       doc: "Return the largest integer no greater than ARG, as a float.\n(Round toward -inf.)",
       (arg: Lisp_Object),
       {
  CHECK_FLOAT (arg);
  let mut d: c_double = XFLOAT_DATA (arg);
  d = floor (d);
  return make_float (d);
});

DEFUN! ("fround", Ffround, Sfround, 1, 1, 0,
       doc: "Return the nearest integer to ARG, as a float.",
       (arg: Lisp_Object),
       {
  CHECK_FLOAT (arg);
  let mut d: c_double = XFLOAT_DATA (arg);
  // C: `d = emacs_rint (d);` — the macro is `rint` because HAVE_RINT is 1 (P-08).
  #[cfg(not(feature = "no_rint"))]
  { d = rint (d); }
  #[cfg(feature = "no_rint")]
  { d = emacs_rint (d); }
  return make_float (d);
});

DEFUN! ("ftruncate", Fftruncate, Sftruncate, 1, 1, 0,
       doc: "Truncate a floating point number to an integral float value.\n(Round toward zero.)",
       (arg: Lisp_Object),
       {
  CHECK_FLOAT (arg);
  let mut d: c_double = XFLOAT_DATA (arg);
  d = trunc (d);
  return make_float (d);
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_floatfns ()
{
  defsubr (&raw mut Sacos);
  defsubr (&raw mut Sasin);
  defsubr (&raw mut Satan);
  defsubr (&raw mut Scos);
  defsubr (&raw mut Ssin);
  defsubr (&raw mut Stan);
  defsubr (&raw mut Sisnan);
  defsubr (&raw mut Scopysign);
  defsubr (&raw mut Sfrexp);
  defsubr (&raw mut Sldexp);
  defsubr (&raw mut Sfceiling);
  defsubr (&raw mut Sffloor);
  defsubr (&raw mut Sfround);
  defsubr (&raw mut Sftruncate);
  defsubr (&raw mut Sexp);
  defsubr (&raw mut Sexpt);
  defsubr (&raw mut Slog);
  defsubr (&raw mut Ssqrt);

  defsubr (&raw mut Sabs);
  defsubr (&raw mut Sfloat);
  defsubr (&raw mut Slogb);
  defsubr (&raw mut Sceiling);
  defsubr (&raw mut Sfloor);
  defsubr (&raw mut Sround);
  defsubr (&raw mut Struncate);
}
