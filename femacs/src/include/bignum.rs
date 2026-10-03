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
// batch F moves them into `femacs/lib/include/mini-gmp.rs` unchanged.
//
// `mpz_t` is C's array-of-one typedef (`typedef __mpz_struct mpz_t[1]`), so the mirror
// keeps it as `[__mpz_struct; 1]`: `sizeof (mpz_t) == sizeof (__mpz_struct)`,
// `Lisp_Bignum`'s layout and every `mpz_t`-shaped parameter (which decays to a pointer in
// C) agree with the C compiler's numbers — asserted by `xtask layout-check` from
// docs/baseline/layout/c-layout-darwin-arm64.tsv.

/* Include this header only if access to bignum internals is needed.  */

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const BIGNUM_H: bool = true;

// <gmp.h> resolves to lib/mini-gmp.h on this target; its mirror
// (femacs/lib/include/mini-gmp.rs) is included by the translation unit right before this
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
}

use bignum_externs::*;

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
