// femacs: port of src/bignum.h @ fd7afced403.
//
// Header mirror: types, constants, macros, inline functions and the `unsafe extern "C"`
// declarations C spells as `extern` (PORTING.md 0.1).  The inline bodies are C's
// statements in C's order; what lisp.h provides in C comes from lisp.rs, which the
// translation unit includes before this mirror.
//
// `<gmp.h>` (gnulib's lib/gmp.h, which is `#include "mini-gmp.h"` on this target) has no
// mirror yet: `lib/` is batch F in docs/port-plan.md §4 and the header's own mirror is
// recorded in docs/port-map.md as unported.  The declarations bignum.h and its users need
// from it are carried verbatim below, marked as such, and recorded in PORTING.md P-49;
// batch F moves them into `lib/include/mini-gmp.rs` unchanged.
//
// `mpz_t` is C's array-of-one typedef (`typedef __mpz_struct mpz_t[1]`), so the mirror
// keeps it as `[__mpz_struct; 1]`: `sizeof (mpz_t) == sizeof (__mpz_struct)`,
// `Lisp_Bignum`'s layout and every `mpz_t`-shaped parameter (which decays to a pointer in
// C) agree with the C compiler's numbers — asserted by `xtask layout-check` from
// docs/baseline/layout/c-layout-darwin-arm64.tsv.

// [U1-MODULE-MIRROR]
use super::*;
/* Include this header only if access to bignum internals is needed.  */

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const BIGNUM_H: bool = true;

// <gmp.h> resolves to lib/mini-gmp.h on this target; its mirror
// (lib/include/mini-gmp.rs) is included by the translation unit right before this
// file, as bignum.h's `#include <gmp.h>` does (P-49; the typedefs, the `mpz_*`
// declarations and the macros all come from there).

/* Compile with -DFASTER_BIGNUM=0 to disable common optimizations and
   allow easier testing of some slow-path code.  */
pub const FASTER_BIGNUM: c_int = 1;

/* Number of data bits in a limb.  */
// C: `enum { GMP_NUMB_BITS = TYPE_WIDTH (mp_limb_t) };` — gnulib's intprops.h
// spells TYPE_WIDTH (t) as `sizeof (t) * CHAR_BIT`, and CHAR_BIT is 8 here.
pub const GMP_NUMB_BITS: c_int = (8 * std::mem::size_of::<mp_limb_t> ()) as c_int;

/* struct Lisp_Bignum
   {
     union vectorlike_header header;
     mpz_t value;
   } GCALIGNED_STRUCT;  */
#[repr(C, align(8))]
pub struct Lisp_Bignum
{
  pub header: vectorlike_header,
  pub value: mpz_t,
}

// C: `extern mpz_t mpz[5];` (bignum.h 45) and the eleven helper prototypes below.
// They live in a module so that bignum.c — which *defines* every one of them, `mpz`
// included — shadows the import instead of colliding with it (P-21/P-39; the same shape
// `lisp.rs` uses for the globals and functions other units define).
pub mod bignum_externs {
  use super::*;

  unsafe extern "C-unwind" {
    pub static mpz: VmGlobal<[mpz_t; 5]>;

    /* extern void init_bignum (void);  */
    pub fn init_bignum ();
    /* extern Lisp_Object make_integer_mpz (void);  */
    pub fn make_integer_mpz () -> Lisp_Object;
    /* extern bool mpz_to_intmax (mpz_t const, intmax_t *);  */
    pub fn mpz_to_intmax (z: *const __mpz_struct, out: *mut intmax_t) -> bool;
    /* extern bool mpz_to_uintmax (mpz_t const, uintmax_t *);  */
    pub fn mpz_to_uintmax (z: *const __mpz_struct, out: *mut uintmax_t) -> bool;
    /* extern void mpz_set_intmax_slow (mpz_t, intmax_t);  */
    pub fn mpz_set_intmax_slow (r: *mut __mpz_struct, v: intmax_t);
    /* extern void mpz_set_uintmax_slow (mpz_t, uintmax_t);  */
    pub fn mpz_set_uintmax_slow (r: *mut __mpz_struct, v: uintmax_t);
    /* extern void emacs_mpz_mul (mpz_t, mpz_t const, mpz_t const);  */
    pub fn emacs_mpz_mul (r: *mut __mpz_struct, a: *const __mpz_struct,
                          b: *const __mpz_struct);
    /* extern void emacs_mpz_mul_2exp (mpz_t, mpz_t const, EMACS_INT);  */
    pub fn emacs_mpz_mul_2exp (r: *mut __mpz_struct, a: *const __mpz_struct, b: EMACS_INT);
    /* extern void emacs_mpz_pow_ui (mpz_t, mpz_t const, unsigned long);  */
    pub fn emacs_mpz_pow_ui (r: *mut __mpz_struct, a: *const __mpz_struct, b: c_ulong);
    /* extern double mpz_get_d_rounded (mpz_t const);  */
    pub fn mpz_get_d_rounded (z: *const __mpz_struct) -> c_double;
    /* extern Lisp_Object get_random_bignum (struct Lisp_Bignum const *);  */
    pub fn get_random_bignum (i: *const Lisp_Bignum) -> Lisp_Object;
  }
}

pub use self::bignum_externs::*;

/* INLINE struct Lisp_Bignum * XBIGNUM (Lisp_Object a)  */
#[inline(always)]
pub unsafe fn XBIGNUM (a: Lisp_Object) -> *mut Lisp_Bignum
{
  eassert! (BIGNUMP (a));
  return XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Bignum);
}

/* INLINE void mpz_set_intmax (mpz_t result, intmax_t v)  */
#[inline(always)]
pub unsafe fn mpz_set_intmax (result: *mut __mpz_struct, v: intmax_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  // C declares `long int i` and lets `ckd_add (&i, v, 0)` store into it; the port has
  // the same pair (the wrapped sum and the overflow flag) from `overflowing_add`
  // (P-15, and the ckd_add family in docs/ports/token-aliases.tsv).
  let (i, overflow) = v.overflowing_add (0);
  if FASTER_BIGNUM != 0 && !overflow
    {
      if let Ok (si) = c_long::try_from (i)
        {
          mpz_set_si (result, si);
          return;
        }
    }
  mpz_set_intmax_slow (result, v);

  }
}

/* INLINE void mpz_set_uintmax (mpz_t result, uintmax_t v)  */
#[inline(always)]
pub unsafe fn mpz_set_uintmax (result: *mut __mpz_struct, v: uintmax_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let (i, overflow) = v.overflowing_add (0);
  if FASTER_BIGNUM != 0 && !overflow
    {
      if let Ok (ui) = c_ulong::try_from (i)
        {
          mpz_set_ui (result, ui);
          return;
        }
    }
  mpz_set_uintmax_slow (result, v);

  }
}

/* INLINE mpz_t const *bignum_val (struct Lisp_Bignum const *i)  */
#[inline(always)]
pub unsafe fn bignum_val (i: *const Lisp_Bignum) -> *const __mpz_struct
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  // C: `return &i->value;` — `value` is an array, so the address of the array and
  // the address of its first element are the same object.
  return (*i).value.as_ptr ();

  }
}

/* INLINE mpz_t const *xbignum_val (Lisp_Object i)  */
#[inline(always)]
pub unsafe fn xbignum_val (i: Lisp_Object) -> *const __mpz_struct
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return bignum_val (XBIGNUM (i));

  }
}

/* INLINE mpz_t const *bignum_integer (mpz_t *tmp, Lisp_Object i)  */
#[inline(always)]
pub unsafe fn bignum_integer (tmp: *mut mpz_t, i: Lisp_Object) -> *const __mpz_struct
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if FIXNUMP (i)
    {
      mpz_set_intmax ((*tmp).as_mut_ptr (), XFIXNUM (i) as intmax_t);
      /* The unnecessary cast pacifies a buggy GCC 4.8.5.  */
      return (*tmp).as_ptr ();
    }
  return xbignum_val (i);

  }
}

/* INLINE void mpz_set_integer (mpz_t result, Lisp_Object i)  */
#[inline(always)]
pub unsafe fn mpz_set_integer (result: *mut __mpz_struct, i: Lisp_Object)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if FIXNUMP (i)
    {
      mpz_set_intmax (result, XFIXNUM (i) as intmax_t);
    }
  else
    {
      mpz_set (result, xbignum_val (i));
    }

  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RoundingDivMode {
  Trunc,
  Floor,
  Ceiling,
  Round,
}

#[inline(always)]
pub fn bignum_negative_p (num: Lisp_Object) -> bool {
  assert! (BIGNUMP (num));
  /* SAFETY: `BIGNUMP(num)` verified above. */
  unsafe { mpz_sgn (xbignum_val (num)) < 0 }
}

#[inline(always)]
pub fn bignum_abs_val (num: Lisp_Object) -> Lisp_Object {
  assert! (BIGNUMP (num));
  /* SAFETY: `BIGNUMP(num)` verified above; `mpz[0]` is VM scratch bignum storage owned by the single Lisp thread. */
  unsafe {
    if mpz_sgn (xbignum_val (num)) < 0 {
      mpz_neg ((*mpz.as_mut_ptr ())[0].as_mut_ptr (), xbignum_val (num));
      make_integer_mpz ()
    } else {
      num
    }
  }
}

#[inline(always)]
pub fn bignum_bit_logb (num: Lisp_Object) -> EMACS_INT {
  assert! (BIGNUMP (num));
  /* SAFETY: `BIGNUMP(num)` verified above; every Lisp bignum is non-zero (`mpz_sizeinbase(..., 2) >= 1`). */
  unsafe { mpz_sizeinbase (xbignum_val (num), 2) as EMACS_INT - 1 }
}

pub fn bignum_ldexp_val (num: Lisp_Object, e: EMACS_INT) -> c_double {
  assert! (BIGNUMP (num));
  /* SAFETY: `BIGNUMP(num)` verified above; `mpz[0..1]` are VM scratch bignum slots owned by the single Lisp thread. */
  unsafe {
    let x = xbignum_val (num);
    let neg = mpz_sgn (x) < 0;
    let limb_bits = GMP_NUMB_BITS as usize;
    let sig_bits = mpz_sizeinbase (x, 2);
    let max_prec = DBL_MANT_DIG as usize;
    let mut adj_e = e;
    let abs_x = (*mpz.as_mut_ptr ())[1].as_mut_ptr ();
    mpz_abs (abs_x, x);
    let rounded: *const __mpz_struct;
    if sig_bits > max_prec {
      let shift = sig_bits - max_prec;
      let t = (*mpz.as_mut_ptr ())[0].as_mut_ptr ();
      mpz_fdiv_q_2exp (t, abs_x, shift as mp_bitcnt_t);
      let top_idx = shift - 1;
      let rem_top = ((mpz_getlimbn (abs_x, (top_idx / limb_bits) as mp_size_t)
        >> (top_idx % limb_bits))
        & 1)
        != 0;
      if rem_top {
        let mut sticky = false;
        if shift > 1 {
          let scan = mpz_scan1 (abs_x, 0);
          sticky = scan < (shift - 1) as mp_bitcnt_t;
        }
        let odd_q = (mpz_getlimbn (t, 0) & 1) != 0;
        if sticky || odd_q {
          mpz_add_ui (t, t, 1);
        }
      }
      adj_e = adj_e.saturating_add (shift as EMACS_INT);
      rounded = t;
    } else {
      rounded = abs_x;
    }
    let mut d = mpz_get_d (rounded);
    if neg {
      d = -d;
    }
    let clamped_e = adj_e.clamp (c_int::MIN as EMACS_INT, c_int::MAX as EMACS_INT) as c_int;
    ldexp (d, clamped_e)
  }
}

pub fn bignum_frexp_val (num: Lisp_Object) -> (c_double, EMACS_INT) {
  assert! (BIGNUMP (num));
  let exp = bignum_bit_logb (num) + 1;
  let s = bignum_ldexp_val (num, -exp);
  if s.abs () >= 1.0 {
    (s * 0.5, exp + 1)
  } else {
    (s, exp)
  }
}

pub fn float_pow_bignum (x: c_double, y: Lisp_Object) -> c_double {
  assert! (BIGNUMP (y));
  /* SAFETY: `BIGNUMP(y)` verified above; reads `xbignum_val(y)` without mutation. */
  unsafe {
    let yz = xbignum_val (y);
    let y_neg = mpz_sgn (yz) < 0;
    let y_odd = mpz_odd_p! (yz) != 0;
    if x.is_nan () {
      return x;
    }
    if x == 1.0 {
      return 1.0;
    }
    if x == -1.0 {
      return if y_odd { -1.0 } else { 1.0 };
    }
    if x == 0.0 {
      if y_neg {
        return if y_odd { 1.0 / x } else { c_double::INFINITY };
      }
      return if y_odd { x } else { 0.0 };
    }
    let abs_gt_1 = x.abs () > 1.0;
    let mag = if abs_gt_1 != y_neg {
      c_double::INFINITY
    } else {
      0.0
    };
    if x < 0.0 && y_odd { -mag } else { mag }
  }
}

pub fn number_scaled_rounding_div (
  n: Lisp_Object,
  d: Lisp_Object,
  nscale: c_int,
  dscale: c_int,
  mode: RoundingDivMode,
) -> Lisp_Object {
  assert! (NUMBERP (n) && NUMBERP (d));
  /* SAFETY: `mpz[0..4]` are VM scratch bignums owned by the single Lisp thread; reproduces `rescale_for_division` + `rounddiv_q` exactly. */
  unsafe {
    unsafe fn rescale_one (
      val: Lisp_Object,
      slot: *mut mpz_t,
      own_scale: c_int,
      other_scale: c_int,
    ) -> *const __mpz_struct {
      /* SAFETY: caller passes valid `slot` from `mpz[0..1]` and verified `NUMBERP(val)`. */
      unsafe {
        let mut p: *const __mpz_struct;
        if FLOATP (val) {
          if DBL_MANT_DIG - DBL_MIN_EXP < own_scale {
            overflow_error ();
          }
          mpz_set_d ((*slot).as_mut_ptr (), scalbn (XFLOAT_DATA (val), own_scale));
          p = (*slot).as_ptr ();
        } else {
          p = bignum_integer (slot, val);
        }
        if own_scale < other_scale {
          emacs_mpz_mul_2exp (
            (*slot).as_mut_ptr (),
            p,
            ((other_scale - own_scale) * LOG2_FLT_RADIX) as EMACS_INT,
          );
          p = (*slot).as_ptr ();
        }
        p
      }
    }

    let mpz_ptr = mpz.as_mut_ptr ();
    let pn = rescale_one (n, &raw mut (*mpz_ptr)[0], nscale, dscale);
    let pd = rescale_one (d, &raw mut (*mpz_ptr)[1], dscale, nscale);
    let q = (*mpz_ptr)[0].as_mut_ptr ();
    match mode {
      RoundingDivMode::Trunc => mpz_tdiv_q (q, pn, pd),
      RoundingDivMode::Floor => mpz_fdiv_q (q, pn, pd),
      RoundingDivMode::Ceiling => mpz_cdiv_q (q, pn, pd),
      RoundingDivMode::Round => {
        let r = &raw mut (*mpz_ptr)[2];
        let abs_r1 = &raw mut (*mpz_ptr)[3];
        mpz_tdiv_qr (q, (*r).as_mut_ptr (), pn, pd);
        let neg_d = mpz_sgn (pd) < 0;
        let neg_r = mpz_sgn ((*r).as_ptr ()) < 0;
        mpz_abs ((*r).as_mut_ptr (), (*r).as_ptr ());
        mpz_abs ((*abs_r1).as_mut_ptr (), pd);
        mpz_sub ((*abs_r1).as_mut_ptr (), (*abs_r1).as_ptr (), (*r).as_ptr ());
        if mpz_cmp ((*abs_r1).as_ptr (), (*r).as_ptr ()) < (mpz_odd_p! (q) != 0) as c_int {
          if neg_d == neg_r {
            mpz_add_ui (q, q, 1);
          } else {
            mpz_sub_ui (q, q, 1);
          }
        }
      }
    }
    make_integer_mpz ()
  }
}
