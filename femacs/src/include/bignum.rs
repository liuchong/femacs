// femacs: port of src/bignum.h @ fd7afced403 (GNU Emacs 32.0.50).
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
// batch F moves them into `femacs/lib/include/mini-gmp.rs` unchanged.
//
// `mpz_t` is C's array-of-one typedef (`typedef __mpz_struct mpz_t[1]`), so the mirror
// keeps it as `[__mpz_struct; 1]`: `sizeof (mpz_t) == sizeof (__mpz_struct)`,
// `Lisp_Bignum`'s layout and every `mpz_t`-shaped parameter (which decays to a pointer in
// C) agree with the C compiler's numbers — asserted by `xtask layout-check` from
// docs/baseline/layout/c-layout-darwin-arm64.tsv.

/* Big numbers for Emacs.

Copyright 2018-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

/* Include this header only if access to bignum internals is needed.  */

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const BIGNUM_H: bool = true;

// --- <gmp.h> -> lib/mini-gmp.h, not mirrored yet (P-26, P-49) ---------------
//
// Declared here in the reference header's own order, with C's line above each one.
// Only what bignum.h, bignum.c and floatfns.c use is listed: the full header is
// mirrored when `lib/` (batch F) is ported.

/* #ifndef MINI_GMP_LIMB_TYPE
   #define MINI_GMP_LIMB_TYPE long
   #endif

   typedef unsigned MINI_GMP_LIMB_TYPE mp_limb_t;  */
pub type mp_limb_t = c_ulong;
/* typedef long mp_size_t;  */
pub type mp_size_t = c_long;
/* typedef unsigned long mp_bitcnt_t;  */
pub type mp_bitcnt_t = c_ulong;

/* typedef mp_limb_t *mp_ptr;  */
pub type mp_ptr = *mut mp_limb_t;
/* typedef const mp_limb_t *mp_srcptr;  */
pub type mp_srcptr = *const mp_limb_t;

/* typedef struct { int _mp_alloc; int _mp_size; mp_limb_t *_mp_d; } __mpz_struct;  */
#[repr(C)]
pub struct __mpz_struct
{
  pub _mp_alloc: c_int,
  pub _mp_size: c_int,
  pub _mp_d: *mut mp_limb_t,
}

/* typedef __mpz_struct mpz_t[1];  */
pub type mpz_t = [__mpz_struct; 1];

/* typedef __mpz_struct *mpz_ptr;  */
pub type mpz_ptr = *mut __mpz_struct;
/* typedef const __mpz_struct *mpz_srcptr;  */
pub type mpz_srcptr = *const __mpz_struct;

// A `mpz_t`-shaped parameter is `__mpz_struct *` after C's array decay, and a
// `mpz_t const` parameter is `const __mpz_struct *`; that is the type every
// declaration below uses, and the reason a `mpz_t` *value* is passed as
// `value.as_mut_ptr ()` / `value.as_ptr ()` at the call sites.
unsafe extern "C" {
  /* int mpz_sgn (const mpz_t);  */
  pub fn mpz_sgn (z: *const __mpz_struct) -> c_int;
  /* int mpz_cmp (const mpz_t, const mpz_t);  */
  pub fn mpz_cmp (a: *const __mpz_struct, b: *const __mpz_struct) -> c_int;

  /* void mpz_abs (mpz_t, const mpz_t);  */
  pub fn mpz_abs (r: *mut __mpz_struct, a: *const __mpz_struct);
  /* void mpz_neg (mpz_t, const mpz_t);  */
  pub fn mpz_neg (r: *mut __mpz_struct, a: *const __mpz_struct);

  /* void mpz_add_ui (mpz_t, const mpz_t, unsigned long);  */
  pub fn mpz_add_ui (r: *mut __mpz_struct, a: *const __mpz_struct, b: c_ulong);
  /* void mpz_sub_ui (mpz_t, const mpz_t, unsigned long);  */
  pub fn mpz_sub_ui (r: *mut __mpz_struct, a: *const __mpz_struct, b: c_ulong);
  /* void mpz_sub (mpz_t, const mpz_t, const mpz_t);  */
  pub fn mpz_sub (r: *mut __mpz_struct, a: *const __mpz_struct, b: *const __mpz_struct);

  /* void mpz_cdiv_q (mpz_t, const mpz_t, const mpz_t);  */
  pub fn mpz_cdiv_q (q: *mut __mpz_struct, n: *const __mpz_struct, d: *const __mpz_struct);
  /* void mpz_fdiv_q (mpz_t, const mpz_t, const mpz_t);  */
  pub fn mpz_fdiv_q (q: *mut __mpz_struct, n: *const __mpz_struct, d: *const __mpz_struct);
  /* void mpz_tdiv_q (mpz_t, const mpz_t, const mpz_t);  */
  pub fn mpz_tdiv_q (q: *mut __mpz_struct, n: *const __mpz_struct, d: *const __mpz_struct);
  /* void mpz_tdiv_qr (mpz_t, mpz_t, const mpz_t, const mpz_t);  */
  pub fn mpz_tdiv_qr (q: *mut __mpz_struct, r: *mut __mpz_struct, n: *const __mpz_struct,
                      d: *const __mpz_struct);

  /* size_t mpz_sizeinbase (const mpz_t, int);  */
  pub fn mpz_sizeinbase (z: *const __mpz_struct, base: c_int) -> usize;

  /* void mpz_set_si (mpz_t, signed long int);  */
  pub fn mpz_set_si (r: *mut __mpz_struct, v: c_long);
  /* void mpz_set_ui (mpz_t, unsigned long int);  */
  pub fn mpz_set_ui (r: *mut __mpz_struct, v: c_ulong);
  /* void mpz_set (mpz_t, const mpz_t);  */
  pub fn mpz_set (r: *mut __mpz_struct, a: *const __mpz_struct);
  /* void mpz_set_d (mpz_t, double);  */
  pub fn mpz_set_d (r: *mut __mpz_struct, v: c_double);
}

/// mpz_odd_p (z): C's object-like macro over a `const mpz_t` — the mirror is the
/// macro's body, so `mpz_odd_p! (&*p)` is C's `mpz_odd_p (p)` (PORTING.md 2.2).
macro_rules! mpz_odd_p {
    ($z:expr) => {
        ((unsafe { (*$z)._mp_size } != 0) as c_int) & unsafe { (*(*$z)._mp_d) } as c_int
    };
}

/// mpz_even_p (z): `! mpz_odd_p (z)`.
macro_rules! mpz_even_p {
    ($z:expr) => {
        (!(mpz_odd_p! ($z) != 0)) as c_int
    };
}

// --- end of the not-yet-mirrored gmp.h subset -------------------------------

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

// C: `extern mpz_t mpz[5];` — mpz_t is an array type, so this is the same object as
// `__mpz_struct mpz[5][1]`.
unsafe extern "C" {
  pub static mut mpz: [mpz_t; 5];

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
  // C declares `long int i` and lets `ckd_add (&i, v, 0)` store into it; the port has
  // the same pair (the wrapped sum and the overflow flag) from `overflowing_add`
  // (P-15, and the ckd_add family in docs/ports/token-aliases.tsv).
  let (i, overflow) = v.overflowing_add (0);
  if FASTER_BIGNUM != 0 && !overflow
    {
      mpz_set_si (result, i);
    }
  else
    {
      mpz_set_intmax_slow (result, v);
    }
}

/* INLINE void mpz_set_uintmax (mpz_t result, uintmax_t v)  */
#[inline(always)]
pub unsafe fn mpz_set_uintmax (result: *mut __mpz_struct, v: uintmax_t)
{
  let (i, overflow) = v.overflowing_add (0);
  if FASTER_BIGNUM != 0 && !overflow
    {
      mpz_set_ui (result, i);
    }
  else
    {
      mpz_set_uintmax_slow (result, v);
    }
}

/* INLINE mpz_t const *bignum_val (struct Lisp_Bignum const *i)  */
#[inline(always)]
pub unsafe fn bignum_val (i: *const Lisp_Bignum) -> *const __mpz_struct
{
  // C: `return &i->value;` — `value` is an array, so the address of the array and
  // the address of its first element are the same object.
  return (*i).value.as_ptr ();
}

/* INLINE mpz_t const *xbignum_val (Lisp_Object i)  */
#[inline(always)]
pub unsafe fn xbignum_val (i: Lisp_Object) -> *const __mpz_struct
{
  return bignum_val (XBIGNUM (i));
}

/* INLINE mpz_t const *bignum_integer (mpz_t *tmp, Lisp_Object i)  */
#[inline(always)]
pub unsafe fn bignum_integer (tmp: *mut mpz_t, i: Lisp_Object) -> *const __mpz_struct
{
  if FIXNUMP (i)
    {
      mpz_set_intmax ((*tmp).as_mut_ptr (), XFIXNUM (i) as intmax_t);
      /* The unnecessary cast pacifies a buggy GCC 4.8.5.  */
      return (*tmp).as_ptr ();
    }
  return xbignum_val (i);
}

/* INLINE void mpz_set_integer (mpz_t result, Lisp_Object i)  */
#[inline(always)]
pub unsafe fn mpz_set_integer (result: *mut __mpz_struct, i: Lisp_Object)
{
  if FIXNUMP (i)
    {
      mpz_set_intmax (result, XFIXNUM (i) as intmax_t);
    }
  else
    {
      mpz_set (result, xbignum_val (i));
    }
}
