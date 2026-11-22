// femacs: port of lib/mini-gmp.h @ fd7afced403.
//
// Header mirror: the typedefs, the four macros and every `extern` declaration of the
// reference header, in its order (PORTING.md 0.1).  It is what `<gmp.h>` resolves to on
// this target: gnulib generates `lib/gmp.h` as `#include "mini-gmp.h"`, so a unit that
// includes it (bignum.c, floatfns.c) gets the declarations C gets (P-49).
//
// The declarations live in a module so that `bignum.c`, which *defines* `mpz` and the
// `mpz_*` entry points, shadows the import instead of colliding with it (the P-19/P-21/
// P-39 shape); the typedefs and the macros stay at the crate root, where the mirrors that
// use them expect them.
//
// `FILE` is <stdio.h>'s opaque type (lib/include/stdio.rs): the reference header declares
// `mpz_out_str` when the includer already has `FILE`, and lisp.h includes <stdio.h> before
// bignum.h includes mini-gmp.h, so every unit that includes this file has it too.
//
// One deviation, recorded in docs/ports/include-mini-gmp.md: C's prototypes have no
// parameter names, so the port names them positionally (`a`, `b`, …); the ABI is checked
// by spikes/gmp-abi-probe.sh, which generates a C harness from this very file.

// C: `#ifndef __MINI_GMP_H__` / `#define __MINI_GMP_H__` / `/* For size_t */
// #include <stddef.h> */` — the include guard has no Rust counterpart (PORTING.md 0.1)
// and `size_t` is the prelude's `usize` (as in the other mirrors).

// The include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const __MINI_GMP_H__: bool = true;

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
#[derive(Clone, Copy)]
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

/* #define mpn_invert_limb(x) mpn_invert_3by2 ((x), 0)  */
macro_rules! mpn_invert_limb {
    ($x:expr) => { mpn_invert_3by2 ($x, 0) };
}

/* #define mpz_odd_p(z)   (((z)->_mp_size != 0) & (int) (z)->_mp_d[0])  */
macro_rules! mpz_odd_p {
    ($z:expr) => {
        ((/* SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants. */ unsafe { (*$z)._mp_size } != 0) as c_int) & unsafe { (*(*$z)._mp_d) } as c_int
    };
}
/* #define mpz_even_p(z)  (! mpz_odd_p (z))  */
macro_rules! mpz_even_p {
    ($z:expr) => {
        (!(mpz_odd_p! ($z) != 0)) as c_int
    };
}
/* #define MPZ_ROINIT_N(xp, xs) {{0, (xs),(xp) }}  */
macro_rules! MPZ_ROINIT_N {
    ($xp:expr, $xs:expr) => {
        __mpz_struct { _mp_alloc: 0, _mp_size: $xs, _mp_d: $xp }
    };
}

// The declarations: `mp_bits_per_limb` plus every function prototype of the header, in
// the reference's order.  In a module, so a unit that defines them shadows the import.
pub mod gmp {
    use super::*;

    unsafe extern "C-unwind" {
        /* extern const int mp_bits_per_limb;  */
        pub static mp_bits_per_limb: c_int;
  pub fn mpn_copyi (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t);
  pub fn mpn_copyd (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t);
  pub fn mpn_zero (a: *mut mp_limb_t, b: mp_size_t);
  pub fn mpn_cmp (a: *const mp_limb_t, b: *const mp_limb_t, c: mp_size_t) -> c_int;
  pub fn mpn_zero_p (a: *const mp_limb_t, b: mp_size_t) -> c_int;
  pub fn mpn_add_1 (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: mp_limb_t) -> mp_limb_t;
  pub fn mpn_add_n (a: *mut mp_limb_t, b: *const mp_limb_t, c: *const mp_limb_t, d: mp_size_t) -> mp_limb_t;
  pub fn mpn_add (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: *const mp_limb_t, e: mp_size_t) -> mp_limb_t;
  pub fn mpn_sub_1 (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: mp_limb_t) -> mp_limb_t;
  pub fn mpn_sub_n (a: *mut mp_limb_t, b: *const mp_limb_t, c: *const mp_limb_t, d: mp_size_t) -> mp_limb_t;
  pub fn mpn_sub (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: *const mp_limb_t, e: mp_size_t) -> mp_limb_t;
  pub fn mpn_mul_1 (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: mp_limb_t) -> mp_limb_t;
  pub fn mpn_addmul_1 (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: mp_limb_t) -> mp_limb_t;
  pub fn mpn_submul_1 (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: mp_limb_t) -> mp_limb_t;
  pub fn mpn_mul (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: *const mp_limb_t, e: mp_size_t) -> mp_limb_t;
  pub fn mpn_mul_n (a: *mut mp_limb_t, b: *const mp_limb_t, c: *const mp_limb_t, d: mp_size_t);
  pub fn mpn_sqr (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t);
  pub fn mpn_perfect_square_p (a: *const mp_limb_t, b: mp_size_t) -> c_int;
  pub fn mpn_sqrtrem (a: *mut mp_limb_t, b: *mut mp_limb_t, c: *const mp_limb_t, d: mp_size_t) -> mp_size_t;
  pub fn mpn_gcd (a: *mut mp_limb_t, b: *mut mp_limb_t, c: mp_size_t, d: *mut mp_limb_t, e: mp_size_t) -> mp_size_t;
  pub fn mpn_lshift (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: c_uint) -> mp_limb_t;
  pub fn mpn_rshift (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t, d: c_uint) -> mp_limb_t;
  pub fn mpn_scan0 (a: *const mp_limb_t, b: mp_bitcnt_t) -> mp_bitcnt_t;
  pub fn mpn_scan1 (a: *const mp_limb_t, b: mp_bitcnt_t) -> mp_bitcnt_t;
  pub fn mpn_com (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t);
  pub fn mpn_neg (a: *mut mp_limb_t, b: *const mp_limb_t, c: mp_size_t) -> mp_limb_t;
  pub fn mpn_popcount (a: *const mp_limb_t, b: mp_size_t) -> mp_bitcnt_t;
  pub fn mpn_invert_3by2 (a: mp_limb_t, b: mp_limb_t) -> mp_limb_t;
  pub fn mpn_get_str (a: *mut u8, b: c_int, c: *mut mp_limb_t, d: mp_size_t) -> usize;
  pub fn mpn_set_str (a: *mut mp_limb_t, b: *const u8, c: usize, d: c_int) -> mp_size_t;
  pub fn mpz_init (a: *mut __mpz_struct);
  pub fn mpz_init2 (a: *mut __mpz_struct, b: mp_bitcnt_t);
  pub fn mpz_clear (a: *mut __mpz_struct);
  pub fn mpz_sgn (a: *const __mpz_struct) -> c_int;
  pub fn mpz_cmp_si (a: *const __mpz_struct, b: c_long) -> c_int;
  pub fn mpz_cmp_ui (a: *const __mpz_struct, b: c_ulong) -> c_int;
  pub fn mpz_cmp (a: *const __mpz_struct, b: *const __mpz_struct) -> c_int;
  pub fn mpz_cmpabs_ui (a: *const __mpz_struct, b: c_ulong) -> c_int;
  pub fn mpz_cmpabs (a: *const __mpz_struct, b: *const __mpz_struct) -> c_int;
  pub fn mpz_cmp_d (a: *const __mpz_struct, b: c_double) -> c_int;
  pub fn mpz_cmpabs_d (a: *const __mpz_struct, b: c_double) -> c_int;
  pub fn mpz_abs (a: *mut __mpz_struct, b: *const __mpz_struct);
  pub fn mpz_neg (a: *mut __mpz_struct, b: *const __mpz_struct);
  pub fn mpz_swap (a: *mut __mpz_struct, b: *mut __mpz_struct);
  pub fn mpz_add_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
  pub fn mpz_add (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_sub_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
  pub fn mpz_ui_sub (a: *mut __mpz_struct, b: c_ulong, c: *const __mpz_struct);
  pub fn mpz_sub (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_mul_si (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_long);
  pub fn mpz_mul_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
  pub fn mpz_mul (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_mul_2exp (a: *mut __mpz_struct, b: *const __mpz_struct, c: mp_bitcnt_t);
  pub fn mpz_addmul_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
  pub fn mpz_addmul (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_submul_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
  pub fn mpz_submul (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_cdiv_qr (a: *mut __mpz_struct, b: *mut __mpz_struct, c: *const __mpz_struct, d: *const __mpz_struct);
  pub fn mpz_fdiv_qr (a: *mut __mpz_struct, b: *mut __mpz_struct, c: *const __mpz_struct, d: *const __mpz_struct);
  pub fn mpz_tdiv_qr (a: *mut __mpz_struct, b: *mut __mpz_struct, c: *const __mpz_struct, d: *const __mpz_struct);
  pub fn mpz_cdiv_q (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_fdiv_q (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_tdiv_q (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_cdiv_r (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_fdiv_r (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_tdiv_r (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_cdiv_q_2exp (a: *mut __mpz_struct, b: *const __mpz_struct, c: mp_bitcnt_t);
  pub fn mpz_fdiv_q_2exp (a: *mut __mpz_struct, b: *const __mpz_struct, c: mp_bitcnt_t);
  pub fn mpz_tdiv_q_2exp (a: *mut __mpz_struct, b: *const __mpz_struct, c: mp_bitcnt_t);
  pub fn mpz_cdiv_r_2exp (a: *mut __mpz_struct, b: *const __mpz_struct, c: mp_bitcnt_t);
  pub fn mpz_fdiv_r_2exp (a: *mut __mpz_struct, b: *const __mpz_struct, c: mp_bitcnt_t);
  pub fn mpz_tdiv_r_2exp (a: *mut __mpz_struct, b: *const __mpz_struct, c: mp_bitcnt_t);
  pub fn mpz_mod (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_divexact (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_divisible_p (a: *const __mpz_struct, b: *const __mpz_struct) -> c_int;
  pub fn mpz_congruent_p (a: *const __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct) -> c_int;
  pub fn mpz_cdiv_qr_ui (a: *mut __mpz_struct, b: *mut __mpz_struct, c: *const __mpz_struct, d: c_ulong) -> c_ulong;
  pub fn mpz_fdiv_qr_ui (a: *mut __mpz_struct, b: *mut __mpz_struct, c: *const __mpz_struct, d: c_ulong) -> c_ulong;
  pub fn mpz_tdiv_qr_ui (a: *mut __mpz_struct, b: *mut __mpz_struct, c: *const __mpz_struct, d: c_ulong) -> c_ulong;
  pub fn mpz_cdiv_q_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong) -> c_ulong;
  pub fn mpz_fdiv_q_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong) -> c_ulong;
  pub fn mpz_tdiv_q_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong) -> c_ulong;
  pub fn mpz_cdiv_r_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong) -> c_ulong;
  pub fn mpz_fdiv_r_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong) -> c_ulong;
  pub fn mpz_tdiv_r_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong) -> c_ulong;
  pub fn mpz_cdiv_ui (a: *const __mpz_struct, b: c_ulong) -> c_ulong;
  pub fn mpz_fdiv_ui (a: *const __mpz_struct, b: c_ulong) -> c_ulong;
  pub fn mpz_tdiv_ui (a: *const __mpz_struct, b: c_ulong) -> c_ulong;
  pub fn mpz_mod_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong) -> c_ulong;
  pub fn mpz_divexact_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
  pub fn mpz_divisible_ui_p (a: *const __mpz_struct, b: c_ulong) -> c_int;
  pub fn mpz_gcd_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong) -> c_ulong;
  pub fn mpz_gcd (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_gcdext (a: *mut __mpz_struct, b: *mut __mpz_struct, c: *mut __mpz_struct, d: *const __mpz_struct, e: *const __mpz_struct);
  pub fn mpz_lcm_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
  pub fn mpz_lcm (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_invert (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct) -> c_int;
  pub fn mpz_sqrtrem (a: *mut __mpz_struct, b: *mut __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_sqrt (a: *mut __mpz_struct, b: *const __mpz_struct);
  pub fn mpz_perfect_square_root (a: *mut __mpz_struct, b: *const __mpz_struct) -> c_int;
  pub fn mpz_perfect_square_p (a: *const __mpz_struct) -> c_int;
  pub fn mpz_pow_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
  pub fn mpz_ui_pow_ui (a: *mut __mpz_struct, b: c_ulong, c: c_ulong);
  pub fn mpz_powm (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct, d: *const __mpz_struct);
  pub fn mpz_powm_ui (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong, d: *const __mpz_struct);
  pub fn mpz_rootrem (a: *mut __mpz_struct, b: *mut __mpz_struct, c: *const __mpz_struct, d: c_ulong);
  pub fn mpz_root (a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong) -> c_int;
  pub fn mpz_fac_ui (a: *mut __mpz_struct, b: c_ulong);
  pub fn mpz_2fac_ui (a: *mut __mpz_struct, b: c_ulong);
  pub fn mpz_mfac_uiui (a: *mut __mpz_struct, b: c_ulong, c: c_ulong);
  pub fn mpz_bin_uiui (a: *mut __mpz_struct, b: c_ulong, c: c_ulong);
  pub fn mpz_probab_prime_p (a: *const __mpz_struct, b: c_int) -> c_int;
  pub fn mpz_tstbit (a: *const __mpz_struct, b: mp_bitcnt_t) -> c_int;
  pub fn mpz_setbit (a: *mut __mpz_struct, b: mp_bitcnt_t);
  pub fn mpz_clrbit (a: *mut __mpz_struct, b: mp_bitcnt_t);
  pub fn mpz_combit (a: *mut __mpz_struct, b: mp_bitcnt_t);
  pub fn mpz_com (a: *mut __mpz_struct, b: *const __mpz_struct);
  pub fn mpz_and (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_ior (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_xor (a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
  pub fn mpz_popcount (a: *const __mpz_struct) -> mp_bitcnt_t;
  pub fn mpz_hamdist (a: *const __mpz_struct, b: *const __mpz_struct) -> mp_bitcnt_t;
  pub fn mpz_scan0 (a: *const __mpz_struct, b: mp_bitcnt_t) -> mp_bitcnt_t;
  pub fn mpz_scan1 (a: *const __mpz_struct, b: mp_bitcnt_t) -> mp_bitcnt_t;
  pub fn mpz_fits_slong_p (a: *const __mpz_struct) -> c_int;
  pub fn mpz_fits_ulong_p (a: *const __mpz_struct) -> c_int;
  pub fn mpz_fits_sint_p (a: *const __mpz_struct) -> c_int;
  pub fn mpz_fits_uint_p (a: *const __mpz_struct) -> c_int;
  pub fn mpz_fits_sshort_p (a: *const __mpz_struct) -> c_int;
  pub fn mpz_fits_ushort_p (a: *const __mpz_struct) -> c_int;
  pub fn mpz_get_si (a: *const __mpz_struct) -> c_long;
  pub fn mpz_get_ui (a: *const __mpz_struct) -> c_ulong;
  pub fn mpz_get_d (a: *const __mpz_struct) -> c_double;
  pub fn mpz_size (a: *const __mpz_struct) -> usize;
  pub fn mpz_getlimbn (a: *const __mpz_struct, b: mp_size_t) -> mp_limb_t;
  pub fn mpz_realloc2 (a: *mut __mpz_struct, b: mp_bitcnt_t);
  pub fn mpz_limbs_read (a: *const __mpz_struct) -> *const mp_limb_t;
  pub fn mpz_limbs_modify (a: *mut __mpz_struct, b: mp_size_t) -> *mut mp_limb_t;
  pub fn mpz_limbs_write (a: *mut __mpz_struct, b: mp_size_t) -> *mut mp_limb_t;
  pub fn mpz_limbs_finish (a: *mut __mpz_struct, b: mp_size_t);
  pub fn mpz_roinit_n (a: *mut __mpz_struct, b: *const mp_limb_t, c: mp_size_t) -> *const __mpz_struct;
  pub fn mpz_set_si (a: *mut __mpz_struct, b: c_long);
  pub fn mpz_set_ui (a: *mut __mpz_struct, b: c_ulong);
  pub fn mpz_set (a: *mut __mpz_struct, b: *const __mpz_struct);
  pub fn mpz_set_d (a: *mut __mpz_struct, b: c_double);
  pub fn mpz_init_set_si (a: *mut __mpz_struct, b: c_long);
  pub fn mpz_init_set_ui (a: *mut __mpz_struct, b: c_ulong);
  pub fn mpz_init_set (a: *mut __mpz_struct, b: *const __mpz_struct);
  pub fn mpz_init_setbit (a: *mut __mpz_struct, b: mp_bitcnt_t);
  pub fn mpz_init_set_d (a: *mut __mpz_struct, b: c_double);
  pub fn mpz_sizeinbase (a: *const __mpz_struct, b: c_int) -> usize;
  pub fn mpz_get_str (a: *mut c_char, b: c_int, c: *const __mpz_struct) -> *mut c_char;
  pub fn mpz_set_str (a: *mut __mpz_struct, b: *const c_char, c: c_int) -> c_int;
  pub fn mpz_init_set_str (a: *mut __mpz_struct, b: *const c_char, c: c_int) -> c_int;
  pub fn mpz_out_str (a: *mut FILE, b: c_int, c: *const __mpz_struct) -> usize;
  pub fn mpz_import (a: *mut __mpz_struct, b: usize, c: c_int, d: usize, e: c_int, f: usize, g: *const c_void);
  pub fn mpz_export (a: *mut c_void, b: *mut usize, c: c_int, d: usize, e: c_int, f: usize, g: *const __mpz_struct) -> *mut c_void;
  pub fn mp_set_memory_functions (a: unsafe extern "C-unwind" fn (usize) -> *mut c_void, b: unsafe extern "C-unwind" fn (*mut c_void, usize, usize) -> *mut c_void, c: unsafe extern "C-unwind" fn (*mut c_void, usize));
  pub fn mp_get_memory_functions (a: *mut Option<unsafe extern "C-unwind" fn (usize) -> *mut c_void>, b: *mut Option<unsafe extern "C-unwind" fn (*mut c_void, usize, usize) -> *mut c_void>, c: *mut Option<unsafe extern "C-unwind" fn (*mut c_void, usize)>);
    }
}

pub use self::gmp::*;
