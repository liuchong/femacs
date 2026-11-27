//! `#![forbid(unsafe_code)]` implementation of floating-point and mixed-numeric
//! mathematical primitives (`src/floatfns.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;

#[inline(always)]
pub fn check_float(x: Lisp_Object) {
    CHECK_TYPE(FLOATP(x), Qfloatp(), x);
}

#[inline(always)]
pub fn extract_float_impl(num: Lisp_Object) -> c_double {
    CHECK_NUMBER(num);
    XFLOATINT(num)
}

pub fn acos_subr(arg: Lisp_Object) -> Lisp_Object {
    make_float(acos(extract_float_impl(arg)))
}

pub fn asin_subr(arg: Lisp_Object) -> Lisp_Object {
    make_float(asin(extract_float_impl(arg)))
}

pub fn atan_subr(y: Lisp_Object, x: Lisp_Object) -> Lisp_Object {
    let d = extract_float_impl(y);
    let res = if NILP(x) {
        atan(d)
    } else {
        atan2(d, extract_float_impl(x))
    };
    make_float(res)
}

pub fn cos_subr(arg: Lisp_Object) -> Lisp_Object {
    make_float(cos(extract_float_impl(arg)))
}

pub fn sin_subr(arg: Lisp_Object) -> Lisp_Object {
    make_float(sin(extract_float_impl(arg)))
}

pub fn tan_subr(arg: Lisp_Object) -> Lisp_Object {
    make_float(tan(extract_float_impl(arg)))
}

pub fn isnan_subr(x: Lisp_Object) -> Lisp_Object {
    check_float(x);
    if XFLOAT_DATA(x).is_nan() { Qt() } else { Qnil() }
}

pub fn copysign_subr(x1: Lisp_Object, x2: Lisp_Object) -> Lisp_Object {
    check_float(x1);
    check_float(x2);
    let f1 = XFLOAT_DATA(x1);
    let f2 = XFLOAT_DATA(x2);
    if f1.is_sign_negative() != f2.is_sign_negative() {
        make_float(-f1)
    } else {
        x1
    }
}

pub fn frexp_subr(x: Lisp_Object) -> Lisp_Object {
    let f = extract_float_impl(x);
    let (sgnfcand, exponent) = frexp_safe(f);
    Fcons(make_float(sgnfcand), make_fixnum(exponent as EMACS_INT))
}

pub fn ldexp_subr(sgnfcand: Lisp_Object, exponent: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(exponent);
    let e = XFIXNUM(exponent).clamp(INT_MIN as isize, INT_MAX as isize) as c_int;
    make_float(ldexp(extract_float_impl(sgnfcand), e))
}

pub fn exp_subr(arg: Lisp_Object) -> Lisp_Object {
    make_float(exp(extract_float_impl(arg)))
}

pub fn expt_subr(arg1: Lisp_Object, arg2: Lisp_Object) -> Lisp_Object {
    CHECK_NUMBER(arg1);
    CHECK_NUMBER(arg2);
    if INTEGERP(arg1) && !NILP(Fnatnump(arg2)) {
        return expt_integer(arg1, arg2);
    }
    make_float(pow(XFLOATINT(arg1), XFLOATINT(arg2)))
}

pub fn log_subr(arg: Lisp_Object, base: Lisp_Object) -> Lisp_Object {
    let mut d = extract_float_impl(arg);
    if NILP(base) {
        d = log(d);
    } else {
        let b = extract_float_impl(base);
        if b == 10.0 {
            d = log10(d);
        } else if HAVE_LOG2 && b == 2.0 {
            d = log2(d);
        } else {
            d = log(d) / log(b);
        }
    }
    make_float(d)
}

pub fn sqrt_subr(arg: Lisp_Object) -> Lisp_Object {
    make_float(sqrt(extract_float_impl(arg)))
}

pub fn abs_subr(mut arg: Lisp_Object) -> Lisp_Object {
    CHECK_NUMBER(arg);
    if FIXNUMP(arg) {
        if XFIXNUM(arg) < 0 {
            arg = make_int(-XFIXNUM(arg) as intmax_t);
        }
    } else if FLOATP(arg) {
        if XFLOAT_DATA(arg).is_sign_negative() {
            arg = make_float(-XFLOAT_DATA(arg));
        }
    } else {
        arg = bignum_abs_val(arg);
    }
    arg
}

pub fn float_subr(arg: Lisp_Object) -> Lisp_Object {
    CHECK_NUMBER(arg);
    if FLOATP(arg) { arg } else { make_float(XFLOATINT(arg)) }
}

pub fn logb_subr(arg: Lisp_Object) -> Lisp_Object {
    CHECK_NUMBER(arg);
    let value: EMACS_INT = if FLOATP(arg) {
        let f = XFLOAT_DATA(arg);
        if f == 0.0 {
            return make_float(-HUGE_VAL);
        }
        if !f.is_finite() {
            return if f < 0.0 { make_float(-f) } else { arg };
        }
        let (_s, ivalue) = frexp_safe(f);
        ivalue as EMACS_INT - 1
    } else if !FIXNUMP(arg) {
        bignum_bit_logb(arg)
    } else {
        let i = XFIXNUM(arg);
        if i == 0 {
            return make_float(-HUGE_VAL);
        }
        elogb(i.unsigned_abs() as u64) as EMACS_INT
    };
    make_fixnum(value)
}

pub fn double_integer_scale_impl(d: c_double) -> c_int {
    let exponent = ilogb(d);
    if DBL_MIN_EXP - 1 <= exponent && exponent < INT_MAX {
        DBL_MANT_DIG - 1 - exponent
    } else {
        DBL_MANT_DIG - DBL_MIN_EXP
            + if d.is_nan() { 2 } else { (exponent == INT_MAX) as c_int }
    }
}

#[inline(always)]
fn ceiling2(n: EMACS_INT, d: EMACS_INT) -> EMACS_INT {
    n / d + (((n % d != 0) && ((n < 0) == (d < 0))) as EMACS_INT)
}

#[inline(always)]
fn floor2(n: EMACS_INT, d: EMACS_INT) -> EMACS_INT {
    n / d - (((n % d != 0) && ((n < 0) != (d < 0))) as EMACS_INT)
}

#[inline(always)]
fn truncate2(n: EMACS_INT, d: EMACS_INT) -> EMACS_INT {
    n / d
}

#[inline(always)]
fn round2(n: EMACS_INT, d: EMACS_INT) -> EMACS_INT {
    let mut q = n / d;
    let r = n % d;
    let neg_d = d < 0;
    let neg_r = r < 0;
    let abs_r = r.wrapping_abs();
    let abs_r1 = d.wrapping_abs().wrapping_sub(abs_r);
    if abs_r1 < abs_r.wrapping_add(q & 1) {
        q += if neg_d == neg_r { 1 } else { -1 };
    }
    q
}

#[inline(always)]
fn rint_mode(d: c_double) -> c_double {
    #[cfg(not(feature = "no_rint"))]
    {
        rint(d)
    }
    #[cfg(feature = "no_rint")]
    {
        let d1 = d + 0.5;
        let r = floor(d1);
        r - ((r == d1 && fmod(r, 2.0) != 0.0) as c_int as c_double)
    }
}

fn rounding_driver(n: Lisp_Object, d: Lisp_Object, mode: RoundingDivMode) -> Lisp_Object {
    CHECK_NUMBER(n);

    if NILP(d) {
        if FLOATP(n) {
            let x = XFLOAT_DATA(n);
            let rounded = match mode {
                RoundingDivMode::Ceiling => ceil(x),
                RoundingDivMode::Floor => floor(x),
                RoundingDivMode::Round => rint_mode(x),
                RoundingDivMode::Trunc => x,
            };
            return double_to_integer(rounded);
        }
        return n;
    }

    CHECK_NUMBER(d);

    let mut dscale: c_int = 0;
    if FIXNUMP(d) {
        if XFIXNUM(d) == 0 {
            xsignal0(Qarith_error());
        }
        if FIXNUMP(n) {
            let vn = XFIXNUM(n);
            let vd = XFIXNUM(d);
            let q = match mode {
                RoundingDivMode::Ceiling => ceiling2(vn, vd),
                RoundingDivMode::Floor => floor2(vn, vd),
                RoundingDivMode::Round => round2(vn, vd),
                RoundingDivMode::Trunc => truncate2(vn, vd),
            };
            return make_int(q as intmax_t);
        }
    } else if FLOATP(d) {
        if XFLOAT_DATA(d) == 0.0 {
            xsignal0(Qarith_error());
        }
        dscale = double_integer_scale_impl(XFLOAT_DATA(d));
    }

    let nscale = if FLOATP(n) {
        double_integer_scale_impl(XFLOAT_DATA(n))
    } else {
        0
    };

    if dscale == DBL_MANT_DIG - DBL_MIN_EXP + 1 && nscale < dscale {
        return make_fixnum(0);
    }

    number_scaled_rounding_div(n, d, nscale, dscale, mode)
}

pub fn ceiling_subr(arg: Lisp_Object, divisor: Lisp_Object) -> Lisp_Object {
    rounding_driver(arg, divisor, RoundingDivMode::Ceiling)
}

pub fn floor_subr(arg: Lisp_Object, divisor: Lisp_Object) -> Lisp_Object {
    rounding_driver(arg, divisor, RoundingDivMode::Floor)
}

pub fn round_subr(arg: Lisp_Object, divisor: Lisp_Object) -> Lisp_Object {
    rounding_driver(arg, divisor, RoundingDivMode::Round)
}

pub fn truncate_subr(arg: Lisp_Object, divisor: Lisp_Object) -> Lisp_Object {
    rounding_driver(arg, divisor, RoundingDivMode::Trunc)
}

pub fn fmod_float_impl(x: Lisp_Object, y: Lisp_Object) -> Lisp_Object {
    let mut f1 = XFLOATINT(x);
    let f2 = XFLOATINT(y);
    f1 = fmod(f1, f2);
    if if f2 < 0.0 { f1 > 0.0 } else { f1 < 0.0 } {
        f1 += f2;
    }
    make_float(f1)
}

pub fn fceiling_subr(arg: Lisp_Object) -> Lisp_Object {
    check_float(arg);
    make_float(ceil(XFLOAT_DATA(arg)))
}

pub fn ffloor_subr(arg: Lisp_Object) -> Lisp_Object {
    check_float(arg);
    make_float(floor(XFLOAT_DATA(arg)))
}

pub fn fround_subr(arg: Lisp_Object) -> Lisp_Object {
    check_float(arg);
    make_float(rint_mode(XFLOAT_DATA(arg)))
}

pub fn ftruncate_subr(arg: Lisp_Object) -> Lisp_Object {
    check_float(arg);
    make_float(trunc(XFLOAT_DATA(arg)))
}
