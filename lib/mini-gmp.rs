// femacs: port of lib/mini-gmp.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same names, same statement order, same control flow, same
// data structures, comments kept.  This is gnulib's bundled mini-gmp, so `<gmp.h>` resolves
// to `lib/include/mini-gmp.rs` (P-49) and this unit defines every function of that mirror.
//
// Deviations (all recorded in docs/ports/mini-gmp.md's Decisions section):
//   * C's `assert` is Rust's `assert!`; the frozen build has NDEBUG undefined, so both are
//     live and both abort.
//   * `mpz_t`/`mpz_ptr`/`mpz_srcptr` parameters are the mirror's `*mut __mpz_struct` /
//     `*const __mpz_struct` (P-49), so C's `r->_mp_size` is `(*r)._mp_size`, and an `mpz_t`
//     local passes as `x.as_mut_ptr ()`.
//   * Carry arithmetic is spelled `wrapping_add`/`wrapping_sub`/`wrapping_mul` (PORTING.md
//     1.1: C's unsigned wrap-around must be explicit).
//   * Function-pointer values are `unsafe extern "C" fn …`; where C allows NULL
//     (the `mp_*_memory_functions` arguments) the port uses `Option<…>`.
//   * `stderr` keeps C's name; the platform symbol is `__stderrp` on darwin (P-26/P-50).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

/* NOTE: All functions in this file which are not declared in
   mini-gmp.h are internal, and are not intended to be compatible
   with GMP or with future versions of mini-gmp. */

/* Much of the material copied from GMP files, including: gmp-impl.h,
   longlong.h, mpn/generic/add_n.c, mpn/generic/addmul_1.c,
   mpn/generic/lshift.c, mpn/generic/mul_1.c,
   mpn/generic/mul_basecase.c, mpn/generic/rshift.c,
   mpn/generic/sbpi1_div_qr.c, mpn/generic/sub_n.c,
   mpn/generic/submul_1.c. */

// C: `#include <assert.h>` / `#include <ctype.h>` / `#include <limits.h>` /
// `#include <stdio.h>` / `#include <stdlib.h>` / `#include <string.h>` /
// `#include "mini-gmp.h"` — the mirrors the port reaches them through (P-26/P-49/P-50).
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

// C: `#include <limits.h>` (CHAR_BIT; lib/include/limits.in.rs carries the values).
// [U1] include/limits.in.rs imported via common
// C: `#include <stdio.h>` (FILE, and the two writers below).
// [U1] include/stdio.rs imported via common
// C: `#include "mini-gmp.h"`.
// [U1] include/mini-gmp.rs imported via common

// The C file's own configuration macro: `#ifndef MINI_GMP_ENABLE_FLOAT
// #define MINI_GMP_ENABLE_FLOAT 1 #endif`, and `#if MINI_GMP_ENABLE_FLOAT
// #include <float.h> #endif`.  The port keeps the macro (the frozen build enables it)
// and includes the <float.h> mirror unconditionally, since DBL_MANT_DIG is below.
pub const MINI_GMP_ENABLE_FLOAT: c_int = 1;
// C: `#include <float.h>` (DBL_MANT_DIG, FLT_RADIX).
// [U1] include/float.in.rs imported via common

/* <stdlib.h>: the allocator family and abort.  */
unsafe extern "C" {
  /* void *malloc (size_t size);  */
  pub fn malloc (size: usize) -> *mut c_void;
  /* void *realloc (void *ptr, size_t size);  -  gnulib's generated header redirects this name
     (`lib/include/stdlib.rs`'s note); this file includes it, so the plain entry is kept only
     where the redirection is off.  */
  #[cfg (not (feature = "replace_realloc_for_realloc_posix"))]
  pub fn realloc (ptr: *mut c_void, size: usize) -> *mut c_void;
  #[cfg (feature = "replace_realloc_for_realloc_posix")]
  #[link_name = "realloc"]
  fn realloc_system (ptr: *mut c_void, size: usize) -> *mut c_void;
  /* void free (void *ptr);  */
  pub fn free (ptr: *mut c_void);
  /* _Noreturn void abort (void);  */
  pub fn abort () -> !;
}

/* <string.h>: the one function this file uses.  */
unsafe extern "C" {
  /* size_t strlen (const char *s);  */
  pub fn strlen (s: *const c_char) -> usize;
}

/* <ctype.h>: the one function this file uses.  */
unsafe extern "C" {
  /* int isspace (int c);  */
  pub fn isspace (c: c_int) -> c_int;
}

/* <stdio.h>'s output side (the mirror carries FILE and fclose).  */
unsafe extern "C" {
  // `<stdio.h>`'s `fwrite` is the include/stdio.rs mirror's now (status 3.371).
  /* extern FILE *stderr;  — darwin spells the object `__stderrp` (stdio.h's `stderr`
     is a macro over it), so the port keeps C's name and binds the platform symbol.  */
}

/* Macros */
/* #define GMP_LIMB_BITS (sizeof(mp_limb_t) * CHAR_BIT)  */
pub const GMP_LIMB_BITS: usize = std::mem::size_of::<mp_limb_t> () * CHAR_BIT as usize;

/* #define GMP_LIMB_MAX ((mp_limb_t) ~ (mp_limb_t) 0)  */
pub const GMP_LIMB_MAX: mp_limb_t = ! (0 as mp_limb_t);
/* #define GMP_LIMB_HIGHBIT ((mp_limb_t) 1 << (GMP_LIMB_BITS - 1))  */
pub const GMP_LIMB_HIGHBIT: mp_limb_t = (1 as mp_limb_t) << (GMP_LIMB_BITS - 1);

/* #define GMP_HLIMB_BIT ((mp_limb_t) 1 << (GMP_LIMB_BITS / 2))  */
pub const GMP_HLIMB_BIT: mp_limb_t = (1 as mp_limb_t) << (GMP_LIMB_BITS / 2);
/* #define GMP_LLIMB_MASK (GMP_HLIMB_BIT - 1)  */
pub const GMP_LLIMB_MASK: mp_limb_t = GMP_HLIMB_BIT - 1;

/* #define GMP_ULONG_BITS (sizeof(unsigned long) * CHAR_BIT)  */
pub const GMP_ULONG_BITS: usize = std::mem::size_of::<c_ulong> () * CHAR_BIT as usize;
/* #define GMP_ULONG_HIGHBIT ((unsigned long) 1 << (GMP_ULONG_BITS - 1))  */
pub const GMP_ULONG_HIGHBIT: c_ulong = (1 as c_ulong) << (GMP_ULONG_BITS - 1);

/* #define GMP_ABS(x) ((x) >= 0 ? (x) : -(x))  */
macro_rules! GMP_ABS {
    ($x:expr) => { if $x >= 0 as _ { $x } else { -$x } };
}
/* #define GMP_NEG_CAST(T,x) (-((T)((x) + 1) - 1))  */
macro_rules! GMP_NEG_CAST {
    ($t:ty, $x:expr) => { ((($x) + 1) as $t).wrapping_sub (1).wrapping_neg () };
}

/* #define GMP_MIN(a, b) ((a) < (b) ? (a) : (b))  */
macro_rules! GMP_MIN {
    ($a:expr, $b:expr) => { if $a < $b { $a } else { $b } };
}
/* #define GMP_MAX(a, b) ((a) > (b) ? (a) : (b))  */
macro_rules! GMP_MAX {
    ($a:expr, $b:expr) => { if $a > $b { $a } else { $b } };
}

/* #define GMP_CMP(a,b) (((a) > (b)) - ((a) < (b)))  */
macro_rules! GMP_CMP {
    ($a:expr, $b:expr) => { (($a > $b) as c_int - ($a < $b) as c_int) };
}

/* `#if defined(DBL_MANT_DIG) && FLT_RADIX == 2 / #define GMP_DBL_MANT_BITS DBL_MANT_DIG
   / #else / #define GMP_DBL_MANT_BITS (53) / #endif`: the frozen target takes the first
   arm (FLT_RADIX is 2), and the port spells that value.  */
pub const GMP_DBL_MANT_BITS: c_int = DBL_MANT_DIG;

/* Return non-zero if xp,xsize and yp,ysize overlap.
   If xp+xsize<=yp there's no overlap, or if yp+ysize<=xp there's no
   overlap.  If both these are false, there's an overlap. */
/* #define GMP_MPN_OVERLAP_P(xp, xsize, yp, ysize) \
     ((xp) + (xsize) > (yp) && (yp) + (ysize) > (xp))  */
macro_rules! GMP_MPN_OVERLAP_P {
    ($xp:expr, $xsize:expr, $yp:expr, $ysize:expr) => {
        ((($xp) as usize).wrapping_add (($xsize) as usize) > (($yp) as usize)
         && (($yp) as usize).wrapping_add (($ysize) as usize) > (($xp) as usize))
    };
}

/* #define gmp_assert_nocarry(x) do { mp_limb_t __cy = (x); assert (__cy == 0);
   (void) (__cy); } while (0)  */
macro_rules! gmp_assert_nocarry {
    ($x:expr) => {{
        let __cy: mp_limb_t = $x;
        assert! (__cy == 0);
        let _ = __cy;
    }};
}

/* #define gmp_clz(count, x) do { ... } while (0)  */
macro_rules! gmp_clz {
    ($count:expr, $x:expr) => {{
        let mut __clz_x: mp_limb_t = $x;
        let mut __clz_c: c_uint = 0;
        let LOCAL_SHIFT_BITS: c_int = 8;
        if GMP_LIMB_BITS > LOCAL_SHIFT_BITS as usize {
            while (__clz_x & ((0xff as mp_limb_t) << (GMP_LIMB_BITS - 8))) == 0 {
                __clz_x <<= LOCAL_SHIFT_BITS;
                __clz_c += 8;
            }
        }
        while (__clz_x & GMP_LIMB_HIGHBIT) == 0 {
            __clz_x <<= 1;
            __clz_c += 1;
        }
        $count = __clz_c;
    }};
}

/* #define gmp_ctz(count, x) do { ... } while (0)  */
macro_rules! gmp_ctz {
    ($count:expr, $x:expr) => {{
        let __ctz_x: mp_limb_t = $x;
        let mut __ctz_c: c_uint = 0;
        gmp_clz! (__ctz_c, __ctz_x & __ctz_x.wrapping_neg ());
        $count = (GMP_LIMB_BITS - 1 - __ctz_c as usize) as c_uint;
    }};
}

/* #define gmp_add_ssaaaa(sh, sl, ah, al, bh, bl) do { ... } while (0)  */
macro_rules! gmp_add_ssaaaa {
    ($sh:expr, $sl:expr, $ah:expr, $al:expr, $bh:expr, $bl:expr) => {{
        let __x: mp_limb_t = ($al).wrapping_add ($bl);
        $sh = ($ah).wrapping_add ($bh)
            .wrapping_add ((__x < ($al)) as mp_limb_t);
        $sl = __x;
    }};
}

/* #define gmp_sub_ddmmss(sh, sl, ah, al, bh, bl) do { ... } while (0)  */
macro_rules! gmp_sub_ddmmss {
    ($sh:expr, $sl:expr, $ah:expr, $al:expr, $bh:expr, $bl:expr) => {{
        let __x: mp_limb_t = ($al).wrapping_sub ($bl);
        $sh = ($ah).wrapping_sub ($bh)
            .wrapping_sub (((($al) < ($bl)) as mp_limb_t));
        $sl = __x;
    }};
}

/* #define gmp_umul_ppmm(w1, w0, u, v) do { ... } while (0)  */
macro_rules! gmp_umul_ppmm {
    ($w1:expr, $w0:expr, $u:expr, $v:expr) => {{
        let LOCAL_GMP_LIMB_BITS: usize = GMP_LIMB_BITS;
        if std::mem::size_of::<c_uint> () * CHAR_BIT as usize >= 2 * GMP_LIMB_BITS
          {
            let __ww: c_uint = ($u as c_uint).wrapping_mul ($v as c_uint);
            $w0 = (__ww as mp_limb_t);
            /* C shifts a 32-bit `unsigned int` here; the shift count is this target's
               64, so the branch is dead and only has to compile (wrapping_shr).  */
            $w1 = (__ww.wrapping_shr (LOCAL_GMP_LIMB_BITS as u32) as mp_limb_t);
          }
        else if GMP_ULONG_BITS >= 2 * GMP_LIMB_BITS
          {
            let __ww: c_ulong = ($u as c_ulong).wrapping_mul ($v as c_ulong);
            $w0 = (__ww as mp_limb_t);
            $w1 = ((__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t);
          }
        else {
            let mut __x0: mp_limb_t;
            let mut __x1: mp_limb_t;
            let mut __x2: mp_limb_t;
            let mut __x3: mp_limb_t;
            let mut __ul: c_uint;
            let mut __vl: c_uint;
            let mut __uh: c_uint;
            let mut __vh: c_uint;
            let __u: mp_limb_t = $u;
            let __v: mp_limb_t = $v;
            assert! (std::mem::size_of::<c_uint> () * 2
                     >= std::mem::size_of::<mp_limb_t> ());

            __ul = (__u & GMP_LLIMB_MASK) as c_uint;
            __uh = (__u >> (GMP_LIMB_BITS / 2)) as c_uint;
            __vl = (__v & GMP_LLIMB_MASK) as c_uint;
            __vh = (__v >> (GMP_LIMB_BITS / 2)) as c_uint;

            __x0 = (__ul as mp_limb_t).wrapping_mul (__vl as mp_limb_t);
            __x1 = (__ul as mp_limb_t).wrapping_mul (__vh as mp_limb_t);
            __x2 = (__uh as mp_limb_t).wrapping_mul (__vl as mp_limb_t);
            __x3 = (__uh as mp_limb_t).wrapping_mul (__vh as mp_limb_t);

            __x1 = __x1.wrapping_add (__x0 >> (GMP_LIMB_BITS / 2));/* this can't give carry */
            __x1 = __x1.wrapping_add (__x2);		/* but this indeed can */
            if __x1 < __x2		/* did we get it? */
              { __x3 = __x3.wrapping_add (GMP_HLIMB_BIT); }	/* yes, add it in the proper pos. */

            $w1 = __x3.wrapping_add (__x1 >> (GMP_LIMB_BITS / 2));
            $w0 = (__x1 << (GMP_LIMB_BITS / 2)).wrapping_add (__x0 & GMP_LLIMB_MASK);
          }
    }};
}

/* If mp_limb_t is of size smaller than int, plain u*v implies
   automatic promotion to *signed* int, and then multiply may overflow
   and cause undefined behavior. Explicitly cast to unsigned int for
   that case. */
/* #define gmp_umullo_limb(u, v) \
     ((sizeof(mp_limb_t) >= sizeof(int)) ? (u)*(v) : (unsigned int)(u) * (v))  */
macro_rules! gmp_umullo_limb {
    ($u:expr, $v:expr) => {
        (if std::mem::size_of::<mp_limb_t> () >= std::mem::size_of::<c_int> ()
           { ($u).wrapping_mul ($v) }
         else { (($u as c_uint).wrapping_mul ($v as c_uint)) as mp_limb_t })
    };
}

/* #define gmp_udiv_qrnnd_preinv(q, r, nh, nl, d, di) do { ... } while (0)  */
macro_rules! gmp_udiv_qrnnd_preinv {
    ($q:expr, $r:expr, $nh:expr, $nl:expr, $d:expr, $di:expr) => {{
        let mut _qh: mp_limb_t = 0;
        let mut _ql: mp_limb_t = 0;
        let mut _r: mp_limb_t;
        let mut _mask: mp_limb_t;
        gmp_umul_ppmm! (_qh, _ql, ($nh), ($di));
        gmp_add_ssaaaa! (_qh, _ql, _qh, _ql, ($nh).wrapping_add (1), ($nl));
        _r = ($nl).wrapping_sub (gmp_umullo_limb! (_qh, ($d)));
        _mask = ((_r > _ql) as mp_limb_t).wrapping_neg (); /* both > and >= are OK */
        _qh = _qh.wrapping_add (_mask);
        _r = _r.wrapping_add (_mask & ($d));
        if _r >= ($d)
          {
            _r = _r.wrapping_sub (($d));
            _qh += 1;
          }

        ($r) = _r;
        ($q) = _qh;
    }};
}

/* #define gmp_udiv_qr_3by2(q, r1, r0, n2, n1, n0, d1, d0, dinv) do { ... } while (0)  */
macro_rules! gmp_udiv_qr_3by2 {
    ($q:expr, $r1:expr, $r0:expr, $n2:expr, $n1:expr, $n0:expr,
     $d1:expr, $d0:expr, $dinv:expr) => {{
        let mut _q0: mp_limb_t = 0;
        let mut _t1: mp_limb_t = 0;
        let mut _t0: mp_limb_t = 0;
        let mut _mask: mp_limb_t;
        gmp_umul_ppmm! (($q), _q0, ($n2), ($dinv));
        gmp_add_ssaaaa! (($q), _q0, ($q), _q0, ($n2), ($n1));

        /* Compute the two most significant limbs of n - q'd */
        ($r1) = ($n1).wrapping_sub (gmp_umullo_limb! (($d1), ($q)));
        gmp_sub_ddmmss! (($r1), ($r0), ($r1), ($n0), ($d1), ($d0));
        gmp_umul_ppmm! (_t1, _t0, ($d0), ($q));
        gmp_sub_ddmmss! (($r1), ($r0), ($r1), ($r0), _t1, _t0);
        ($q) += 1;

        /* Conditionally adjust q and the remainders */
        _mask = ((( ($r1) >= _q0) as mp_limb_t)).wrapping_neg ();
        ($q) = ($q).wrapping_add (_mask);
        gmp_add_ssaaaa! (($r1), ($r0), ($r1), ($r0),
                         _mask & ($d1), _mask & ($d0));
        if ($r1) >= ($d1)
          {
            if ($r1) > ($d1) || ($r0) >= ($d0)
              {
                ($q) += 1;
                gmp_sub_ddmmss! (($r1), ($r0), ($r1), ($r0), ($d1), ($d0));
              }
          }
    }};
}

/* Swap macros. */
/* #define MP_LIMB_T_SWAP(x, y) do { ... } while (0)  */
macro_rules! MP_LIMB_T_SWAP {
    ($x:expr, $y:expr) => {{
        let __mp_limb_t_swap__tmp: mp_limb_t = ($x);
        ($x) = ($y);
        ($y) = __mp_limb_t_swap__tmp;
    }};
}
/* #define MP_SIZE_T_SWAP(x, y) do { ... } while (0)  */
macro_rules! MP_SIZE_T_SWAP {
    ($x:expr, $y:expr) => {{
        let __mp_size_t_swap__tmp = ($x);
        ($x) = ($y);
        ($y) = __mp_size_t_swap__tmp;
    }};
}
/* #define MP_BITCNT_T_SWAP(x,y) do { ... } while (0)  */
macro_rules! MP_BITCNT_T_SWAP {
    ($x:expr, $y:expr) => {{
        let __mp_bitcnt_t_swap__tmp: mp_bitcnt_t = ($x);
        ($x) = ($y);
        ($y) = __mp_bitcnt_t_swap__tmp;
    }};
}
/* #define MP_PTR_SWAP(x, y) do { ... } while (0)  */
macro_rules! MP_PTR_SWAP {
    ($x:expr, $y:expr) => {{
        let __mp_ptr_swap__tmp: mp_ptr = ($x);
        ($x) = ($y);
        ($y) = __mp_ptr_swap__tmp;
    }};
}
/* #define MP_SRCPTR_SWAP(x, y) do { ... } while (0)  */
macro_rules! MP_SRCPTR_SWAP {
    ($x:expr, $y:expr) => {{
        let __mp_srcptr_swap__tmp: mp_srcptr = ($x);
        ($x) = ($y);
        ($y) = __mp_srcptr_swap__tmp;
    }};
}

/* #define MPN_PTR_SWAP(xp,xs, yp,ys) do { ... } while(0)  */
macro_rules! MPN_PTR_SWAP {
    ($xp:expr, $xs:expr, $yp:expr, $ys:expr) => {{
        MP_PTR_SWAP! ($xp, $yp);
        MP_SIZE_T_SWAP! ($xs, $ys);
    }};
}
/* #define MPN_SRCPTR_SWAP(xp,xs, yp,ys) do { ... } while(0)  */
macro_rules! MPN_SRCPTR_SWAP {
    ($xp:expr, $xs:expr, $yp:expr, $ys:expr) => {{
        MP_SRCPTR_SWAP! ($xp, $yp);
        MP_SIZE_T_SWAP! ($xs, $ys);
    }};
}

/* #define MPZ_PTR_SWAP(x, y) do { ... } while (0)  */
macro_rules! MPZ_PTR_SWAP {
    ($x:expr, $y:expr) => {{
        let __mpz_ptr_swap__tmp: mpz_ptr = ($x);
        ($x) = ($y);
        ($y) = __mpz_ptr_swap__tmp;
    }};
}
/* #define MPZ_SRCPTR_SWAP(x, y) do { ... } while (0)  */
macro_rules! MPZ_SRCPTR_SWAP {
    ($x:expr, $y:expr) => {{
        let __mpz_srcptr_swap__tmp: mpz_srcptr = ($x);
        ($x) = ($y);
        ($y) = __mpz_srcptr_swap__tmp;
    }};
}

#[unsafe(no_mangle)]
pub static mp_bits_per_limb: c_int = GMP_LIMB_BITS as c_int;

/* Memory allocation and other helper functions. */
unsafe extern "C-unwind" fn gmp_die (msg: *const c_char)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  fprintf (stderr, c"%s\n".as_ptr (), msg);
  abort();

  }
}

unsafe extern "C-unwind" fn gmp_default_alloc (size: usize) -> *mut c_void
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut p: *mut c_void;

  assert! (size > 0);

  p = malloc (size);
  if p.is_null ()
    { gmp_die(c"gmp_default_alloc: Virtual memory exhausted.".as_ptr ()); }

  return p;

  }
}

unsafe extern "C-unwind" fn gmp_default_realloc (old: *mut c_void, unused_old_size: usize,
                                          new_size: usize) -> *mut c_void
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut p: *mut c_void;

  p = realloc (old, new_size);

  if p.is_null ()
    { gmp_die(c"gmp_default_realloc: Virtual memory exhausted.".as_ptr ()); }

  return p;

  }
}

unsafe extern "C-unwind" fn gmp_default_free (p: *mut c_void, unused_size: usize)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  free (p);

  }
}

static mut gmp_allocate_func: unsafe extern "C-unwind" fn (usize) -> *mut c_void
  = gmp_default_alloc;
static mut gmp_reallocate_func: unsafe extern "C-unwind" fn (*mut c_void, usize, usize)
  -> *mut c_void = gmp_default_realloc;
static mut gmp_free_func: unsafe extern "C-unwind" fn (*mut c_void, usize) = gmp_default_free;

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mp_get_memory_functions
  (alloc_func: *mut Option<unsafe extern "C-unwind" fn (usize) -> *mut c_void>,
   realloc_func: *mut Option<unsafe extern "C-unwind" fn (*mut c_void, usize, usize)
                                 -> *mut c_void>,
   free_func: *mut Option<unsafe extern "C-unwind" fn (*mut c_void, usize)>)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if !alloc_func.is_null ()
    { *alloc_func = Some (gmp_allocate_func); }

  if !realloc_func.is_null ()
    { *realloc_func = Some (gmp_reallocate_func); }

  if !free_func.is_null ()
    { *free_func = Some (gmp_free_func); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mp_set_memory_functions
  (mut alloc_func: Option<unsafe extern "C-unwind" fn (usize) -> *mut c_void>,
   mut realloc_func: Option<unsafe extern "C-unwind" fn (*mut c_void, usize, usize)
                                     -> *mut c_void>,
   mut free_func: Option<unsafe extern "C-unwind" fn (*mut c_void, usize)>)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if alloc_func.is_none ()
    { alloc_func = Some (gmp_default_alloc); }
  if realloc_func.is_none ()
    { realloc_func = Some (gmp_default_realloc); }
  if free_func.is_none ()
    { free_func = Some (gmp_default_free); }

  gmp_allocate_func = match alloc_func { Some (f) => f, None => gmp_default_alloc };
  gmp_reallocate_func = match realloc_func { Some (f) => f, None => gmp_default_realloc };
  gmp_free_func = match free_func { Some (f) => f, None => gmp_default_free };

  }
}

/* #define gmp_alloc(size) ((*gmp_allocate_func)((size)))  */
macro_rules! gmp_alloc {
    ($size:expr) => { /* SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants. */ unsafe { (gmp_allocate_func) ($size) } };
}
/* #define gmp_free(p, size) ((*gmp_free_func) ((p), (size)))  */
macro_rules! gmp_free {
    ($p:expr, $size:expr) => { unsafe { (gmp_free_func) ($p, $size) } };
}
/* #define gmp_realloc(ptr, old_size, size) ((*gmp_reallocate_func)(ptr, old_size, size))  */
macro_rules! gmp_realloc {
    ($ptr:expr, $old_size:expr, $size:expr) => {
        /* SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants. */ unsafe { (gmp_reallocate_func) ($ptr, $old_size, $size) }
    };
}

unsafe extern "C-unwind" fn gmp_alloc_limbs (size: mp_size_t) -> mp_ptr
{
  return gmp_alloc! (size as usize * std::mem::size_of::<mp_limb_t> ())
           as mp_ptr;
}

unsafe extern "C-unwind" fn gmp_realloc_limbs (old: mp_ptr, old_size: mp_size_t,
                                        size: mp_size_t) -> mp_ptr
{
  assert! (size > 0);
  return gmp_realloc! (old as *mut c_void,
                       old_size as usize * std::mem::size_of::<mp_limb_t> (),
                       size as usize * std::mem::size_of::<mp_limb_t> ())
           as mp_ptr;
}

unsafe extern "C-unwind" fn gmp_free_limbs (old: mp_ptr, size: mp_size_t)
{
  gmp_free! (old as *mut c_void, size as usize * std::mem::size_of::<mp_limb_t> ());
}

/* MPN interface */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_copyi (d: mp_ptr, s: mp_srcptr, n: mp_size_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;
  i = 0;
  while i < n
    { *d.offset (i as isize) = *s.offset (i as isize); i += 1; }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_copyd (d: mp_ptr, s: mp_srcptr, mut n: mp_size_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  while { n -= 1; n >= 0 }
    { *d.offset (n as isize) = *s.offset (n as isize); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_cmp (ap: mp_srcptr, bp: mp_srcptr, mut n: mp_size_t)
  -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  while { n -= 1; n >= 0 }
    {
      if *ap.offset (n as isize) != *bp.offset (n as isize)
	{ return if *ap.offset (n as isize) > *bp.offset (n as isize) { 1 } else { -1 }; }
    }
  return 0;

  }
}

unsafe extern "C-unwind" fn mpn_cmp4 (ap: mp_srcptr, an: mp_size_t, bp: mp_srcptr,
                               bn: mp_size_t) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if an != bn
    { return if an < bn { -1 } else { 1 }; }
  else
    { return mpn_cmp (ap, bp, an); }

  }
}

unsafe extern "C-unwind" fn mpn_normalized_size (xp: mp_srcptr, mut n: mp_size_t)
  -> mp_size_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  while n > 0 && *xp.offset ((n-1) as isize) == 0
    { n -= 1; }
  return n;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_zero_p (rp: mp_srcptr, n: mp_size_t) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return (mpn_normalized_size (rp, n) == 0) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_zero (rp: mp_ptr, mut n: mp_size_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  while { n -= 1; n >= 0 }
    { *rp.offset (n as isize) = 0; }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_add_1 (rp: mp_ptr, ap: mp_srcptr, n: mp_size_t,
                                    mut b: mp_limb_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;

  assert! (n > 0);
  i = 0;
  loop
    {
      let r: mp_limb_t = (*ap.offset (i as isize)).wrapping_add (b);
      /* Carry out */
      b = (r < b) as mp_limb_t;
      *rp.offset (i as isize) = r;
      i += 1;
      if ! (i < n) { break; }
    }

  return b;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_add_n (rp: mp_ptr, ap: mp_srcptr, bp: mp_srcptr,
                                    n: mp_size_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;
  let mut cy: mp_limb_t;

  i = 0;
  cy = 0;
  while i < n
    {
      let mut a: mp_limb_t;
      let b: mp_limb_t;
      let mut r: mp_limb_t;
      a = *ap.offset (i as isize); b = *bp.offset (i as isize);
      r = a.wrapping_add (cy);
      cy = (r < cy) as mp_limb_t;
      r = r.wrapping_add (b);
      cy += (r < b) as mp_limb_t;
      *rp.offset (i as isize) = r;
      i += 1;
    }
  return cy;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_add (rp: mp_ptr, ap: mp_srcptr, an: mp_size_t,
                                  bp: mp_srcptr, bn: mp_size_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut cy: mp_limb_t;

  assert! (an >= bn);

  cy = mpn_add_n (rp, ap, bp, bn);
  if an > bn
    { cy = mpn_add_1 (rp.offset (bn as isize), ap.offset (bn as isize), an - bn, cy); }
  return cy;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_sub_1 (rp: mp_ptr, ap: mp_srcptr, n: mp_size_t,
                                    mut b: mp_limb_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;

  assert! (n > 0);

  i = 0;
  loop
    {
      let a: mp_limb_t = *ap.offset (i as isize);
      /* Carry out */
      let cy: mp_limb_t = (a < b) as mp_limb_t;
      *rp.offset (i as isize) = a.wrapping_sub (b);
      b = cy;
      i += 1;
      if ! (i < n) { break; }
    }

  return b;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_sub_n (rp: mp_ptr, ap: mp_srcptr, bp: mp_srcptr,
                                    n: mp_size_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;
  let mut cy: mp_limb_t;

  i = 0;
  cy = 0;
  while i < n
    {
      let a: mp_limb_t;
      let mut b: mp_limb_t;
      a = *ap.offset (i as isize); b = *bp.offset (i as isize);
      b = b.wrapping_add (cy);
      cy = (b < cy) as mp_limb_t;
      cy += (a < b) as mp_limb_t;
      *rp.offset (i as isize) = a.wrapping_sub (b);
      i += 1;
    }
  return cy;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_sub (rp: mp_ptr, ap: mp_srcptr, an: mp_size_t,
                                  bp: mp_srcptr, bn: mp_size_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut cy: mp_limb_t;

  assert! (an >= bn);

  cy = mpn_sub_n (rp, ap, bp, bn);
  if an > bn
    { cy = mpn_sub_1 (rp.offset (bn as isize), ap.offset (bn as isize), an - bn, cy); }
  return cy;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_mul_1 (rp: mp_ptr, up: mp_srcptr, mut n: mp_size_t,
                                    vl: mp_limb_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut ul: mp_limb_t;
  let mut cl: mp_limb_t;
  let mut hpl: mp_limb_t = 0;
  let mut lpl: mp_limb_t = 0;
  let mut rp = rp;
  let mut up = up;

  assert! (n >= 1);

  cl = 0;
  loop
    {
      ul = *up;
      up = up.add (1);
      gmp_umul_ppmm! (hpl, lpl, ul, vl);

      lpl = lpl.wrapping_add (cl);
      cl = ((lpl < cl) as mp_limb_t).wrapping_add (hpl);

      *rp = lpl;
      rp = rp.add (1);
      n -= 1;
      if ! (n != 0) { break; }
    }

  return cl;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_addmul_1 (rp: mp_ptr, up: mp_srcptr, mut n: mp_size_t,
                                       vl: mp_limb_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut ul: mp_limb_t;
  let mut cl: mp_limb_t;
  let mut hpl: mp_limb_t = 0;
  let mut lpl: mp_limb_t = 0;
  let mut rl: mp_limb_t;
  let mut rp = rp;
  let mut up = up;

  assert! (n >= 1);

  cl = 0;
  loop
    {
      ul = *up;
      up = up.add (1);
      gmp_umul_ppmm! (hpl, lpl, ul, vl);

      lpl = lpl.wrapping_add (cl);
      cl = ((lpl < cl) as mp_limb_t).wrapping_add (hpl);

      rl = *rp;
      lpl = rl.wrapping_add (lpl);
      cl += (lpl < rl) as mp_limb_t;
      *rp = lpl;
      rp = rp.add (1);
      n -= 1;
      if ! (n != 0) { break; }
    }

  return cl;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_submul_1 (rp: mp_ptr, up: mp_srcptr, mut n: mp_size_t,
                                       vl: mp_limb_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut ul: mp_limb_t;
  let mut cl: mp_limb_t;
  let mut hpl: mp_limb_t = 0;
  let mut lpl: mp_limb_t = 0;
  let mut rl: mp_limb_t;
  let mut rp = rp;
  let mut up = up;

  assert! (n >= 1);

  cl = 0;
  loop
    {
      ul = *up;
      up = up.add (1);
      gmp_umul_ppmm! (hpl, lpl, ul, vl);

      lpl = lpl.wrapping_add (cl);
      cl = ((lpl < cl) as mp_limb_t).wrapping_add (hpl);

      rl = *rp;
      lpl = rl.wrapping_sub (lpl);
      cl += (lpl > rl) as mp_limb_t;
      *rp = lpl;
      rp = rp.add (1);
      n -= 1;
      if ! (n != 0) { break; }
    }

  return cl;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_mul (rp: mp_ptr, up: mp_srcptr, un: mp_size_t,
                                  vp: mp_srcptr, mut vn: mp_size_t) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  assert! (un >= vn);
  assert! (vn >= 1);
  assert! (!GMP_MPN_OVERLAP_P!(rp, un + vn, up, un));
  assert! (!GMP_MPN_OVERLAP_P!(rp, un + vn, vp, vn));

  /* We first multiply by the low order limb. This result can be
     stored, not added, to rp. We also avoid a loop for zeroing this
     way. */

  let mut rp = rp;
  let mut vp = vp;
  *rp.offset (un as isize) = mpn_mul_1 (rp, up, un, *vp.offset (0));

  /* Now accumulate the product of up[] and the next higher limb from
     vp[]. */

  while { vn -= 1; vn >= 1 }
    {
      rp = rp.add (1); vp = vp.add (1);
      *rp.offset (un as isize) = mpn_addmul_1 (rp, up, un, *vp.offset (0));
    }
  return *rp.offset (un as isize);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_mul_n (rp: mp_ptr, ap: mp_srcptr, bp: mp_srcptr,
                                    n: mp_size_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpn_mul (rp, ap, n, bp, n);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_sqr (rp: mp_ptr, ap: mp_srcptr, n: mp_size_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpn_mul (rp, ap, n, ap, n);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_lshift (rp: mp_ptr, up: mp_srcptr, mut n: mp_size_t,
                                     cnt: c_uint) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut high_limb: mp_limb_t;
  let mut low_limb: mp_limb_t;
  let mut tnc: c_uint;
  let mut retval: mp_limb_t;
  let mut rp = rp;
  let mut up = up;

  assert! (n >= 1);
  assert! (cnt >= 1);
  assert! ((cnt as usize) < GMP_LIMB_BITS);

  up = up.offset (n as isize);
  rp = rp.offset (n as isize);

  tnc = (GMP_LIMB_BITS - cnt as usize) as c_uint;
  up = up.offset (-1);
  low_limb = *up;
  retval = low_limb >> tnc;
  high_limb = (low_limb << cnt);

  while { n -= 1; n != 0 }
    {
      up = up.offset (-1);
      low_limb = *up;
      rp = rp.offset (-1);
      *rp = high_limb | (low_limb >> tnc);
      high_limb = (low_limb << cnt);
    }
  rp = rp.offset (-1);
  *rp = high_limb;

  return retval;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_rshift (rp: mp_ptr, up: mp_srcptr, mut n: mp_size_t,
                                     cnt: c_uint) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut high_limb: mp_limb_t;
  let mut low_limb: mp_limb_t;
  let mut tnc: c_uint;
  let mut retval: mp_limb_t;
  let mut rp = rp;
  let mut up = up;

  assert! (n >= 1);
  assert! (cnt >= 1);
  assert! ((cnt as usize) < GMP_LIMB_BITS);

  tnc = (GMP_LIMB_BITS - cnt as usize) as c_uint;
  high_limb = *up;
  up = up.add (1);
  retval = (high_limb << tnc);
  low_limb = high_limb >> cnt;

  while { n -= 1; n != 0 }
    {
      high_limb = *up;
      up = up.add (1);
      *rp = low_limb | (high_limb << tnc);
      rp = rp.add (1);
      low_limb = high_limb >> cnt;
    }
  *rp = low_limb;

  return retval;

  }
}

unsafe extern "C-unwind" fn mpn_common_scan (mut limb: mp_limb_t, mut i: mp_size_t,
                                      up: mp_srcptr, un: mp_size_t, ux: mp_limb_t)
  -> mp_bitcnt_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut cnt: c_uint = 0;

  assert! (ux == 0 || ux == GMP_LIMB_MAX);
  assert! (0 <= i && i <= un );

  while limb == 0
    {
      i += 1;
      if i == un
	{ return if ux == 0 { !(0 as mp_bitcnt_t) } else { (un as usize * GMP_LIMB_BITS) as mp_bitcnt_t }; }
      limb = ux ^ *up.offset (i as isize);
    }
  gmp_ctz! (cnt, limb);
  return (i as usize * GMP_LIMB_BITS) as mp_bitcnt_t + cnt as mp_bitcnt_t;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_scan1 (ptr: mp_srcptr, bit: mp_bitcnt_t) -> mp_bitcnt_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;
  i = (bit / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;

  return mpn_common_scan ( *ptr.offset (i as isize)
                           & (GMP_LIMB_MAX << (bit % GMP_LIMB_BITS as mp_bitcnt_t)),
			  i, ptr, i, 0);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_scan0 (ptr: mp_srcptr, bit: mp_bitcnt_t) -> mp_bitcnt_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;
  i = (bit / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;

  return mpn_common_scan ((!*ptr.offset (i as isize))
                          & (GMP_LIMB_MAX << (bit % GMP_LIMB_BITS as mp_bitcnt_t)),
			  i, ptr, i, GMP_LIMB_MAX);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_com (rp: mp_ptr, up: mp_srcptr, mut n: mp_size_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut rp = rp;
  let mut up = up;
  while { n -= 1; n >= 0 }
    { *rp = ! *up; rp = rp.add (1); up = up.add (1); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_neg (rp: mp_ptr, up: mp_srcptr, mut n: mp_size_t)
  -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut rp = rp;
  let mut up = up;
  while *up == 0
    {
      *rp = 0;
      n -= 1;
      if n == 0
	{ return 0; }
      up = up.add (1); rp = rp.add (1);
    }
  *rp = (*up).wrapping_neg ();
  rp = rp.add (1); up = up.add (1); n -= 1;
  mpn_com (rp, up, n);
  return 1;

  }
}

/* MPN division interface. */

/* The 3/2 inverse is defined as

     m = floor( (B^3-1) / (B u1 + u0)) - B
*/
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_invert_3by2 (u1: mp_limb_t, u0: mp_limb_t) -> mp_limb_t
{
  let mut r: mp_limb_t;
  let mut m: mp_limb_t;

  {
    let mut p: mp_limb_t;
    let mut ql: mp_limb_t;
    let mut ul: c_uint;
    let mut uh: c_uint;
    let mut qh: c_uint;

    assert! (std::mem::size_of::<c_uint> () * 2 >= std::mem::size_of::<mp_limb_t> ());
    /* For notation, let b denote the half-limb base, so that B = b^2.
       Split u1 = b uh + ul. */
    ul = (u1 & GMP_LLIMB_MASK) as c_uint;
    uh = (u1 >> (GMP_LIMB_BITS / 2)) as c_uint;

    /* Approximation of the high half of quotient. Differs from the 2/1
       inverse of the half limb uh, since we have already subtracted
       u0. */
    qh = ((u1 ^ GMP_LIMB_MAX) / uh as mp_limb_t) as c_uint;

    /* Adjust to get a half-limb 3/2 inverse, i.e., we want

       qh' = floor( (b^3 - 1) / u) - b = floor ((b^3 - b u - 1) / u
	   = floor( (b (~u) + b-1) / u),

       and the remainder

       r = b (~u) + b-1 - qh (b uh + ul)
       = b (~u - qh uh) + b-1 - qh ul

       Subtraction of qh ul may underflow, which implies adjustments.
       But by normalization, 2 u >= B > qh ul, so we need to adjust by
       at most 2.
    */

    r = ((!u1).wrapping_sub ((qh as mp_limb_t).wrapping_mul (uh as mp_limb_t))
         << (GMP_LIMB_BITS / 2)) | GMP_LLIMB_MASK;

    p = (qh as mp_limb_t).wrapping_mul (ul as mp_limb_t);
    /* Adjustment steps taken from udiv_qrnnd_c */
    if (r < p)
      {
        qh = qh.wrapping_sub (1);
        r = r.wrapping_add (u1);
        if (r >= u1) /* i.e. we didn't get carry when adding to r */
          { if (r < p)
	    {
	      qh = qh.wrapping_sub (1);
	      r = r.wrapping_add (u1);
	    } }
      }
    r = r.wrapping_sub (p);

    /* Low half of the quotient is

       ql = floor ( (b r + b-1) / u1).

       This is a 3/2 division (on half-limbs), for which qh is a
       suitable inverse. */

    p = (r >> (GMP_LIMB_BITS / 2)).wrapping_mul (qh as mp_limb_t).wrapping_add (r);
    /* Unlike full-limb 3/2, we can add 1 without overflow. For this to
       work, it is essential that ql is a full mp_limb_t. */
    ql = (p >> (GMP_LIMB_BITS / 2)).wrapping_add (1);

    /* By the 3/2 trick, we don't need the high half limb. */
    r = (r << (GMP_LIMB_BITS / 2)).wrapping_add (GMP_LLIMB_MASK)
          .wrapping_sub (ql.wrapping_mul (u1));

    if (r >= (GMP_LIMB_MAX & (p << (GMP_LIMB_BITS / 2))))
      {
        ql = ql.wrapping_sub (1);
        r = r.wrapping_add (u1);
      }
    m = ((qh as mp_limb_t) << (GMP_LIMB_BITS / 2)).wrapping_add (ql);
    if (r >= u1)
      {
        m += 1;
        r = r.wrapping_sub (u1);
      }
  }

  /* Now m is the 2/1 inverse of u1. If u0 > 0, adjust it to become a
     3/2 inverse. */
  if (u0 > 0)
    {
      let mut th: mp_limb_t = 0;
      let mut tl: mp_limb_t = 0;
      r = !r;
      r = r.wrapping_add (u0);
      if (r < u0)
	{
	  m -= 1;
	  if (r >= u1)
	    {
	      m -= 1;
	      r = r.wrapping_sub (u1);
	    }
	  r = r.wrapping_sub (u1);
	}
      gmp_umul_ppmm! (th, tl, u0, m);
      r = r.wrapping_add (th);
      if (r < th)
	{
	  m -= 1;
	  m -= ((r > u1) | ((r == u1) & (tl > u0))) as mp_limb_t;
	}
    }

  return m;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct gmp_div_inverse
{
  /* Normalization shift count. */
  pub shift: c_uint,
  /* Normalized divisor (d0 unused for mpn_div_qr_1) */
  pub d1: mp_limb_t,
  pub d0: mp_limb_t,
  /* Inverse, for 2/1 or 3/2. */
  pub di: mp_limb_t,
}

unsafe extern "C-unwind" fn mpn_div_qr_1_invert (inv: *mut gmp_div_inverse, d: mp_limb_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut shift: c_uint = 0;

  assert! (d > 0);
  gmp_clz! (shift, d);
  (*inv).shift = shift;
  (*inv).d1 = d << shift;
  (*inv).di = mpn_invert_limb! ((*inv).d1);

  }
}

unsafe extern "C-unwind" fn mpn_div_qr_2_invert (inv: *mut gmp_div_inverse,
                                          mut d1: mp_limb_t, mut d0: mp_limb_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut shift: c_uint = 0;

  assert! (d1 > 0);
  gmp_clz! (shift, d1);
  (*inv).shift = shift;
  if (shift > 0)
    {
      d1 = (d1 << shift) | (d0 >> (GMP_LIMB_BITS - shift as usize));
      d0 <<= shift;
    }
  (*inv).d1 = d1;
  (*inv).d0 = d0;
  (*inv).di = mpn_invert_3by2 (d1, d0);

  }
}

unsafe extern "C-unwind" fn mpn_div_qr_invert (inv: *mut gmp_div_inverse,
                                        dp: mp_srcptr, dn: mp_size_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  assert! (dn > 0);

  if (dn == 1)
    { mpn_div_qr_1_invert (inv, *dp); }
  else if (dn == 2)
    { mpn_div_qr_2_invert (inv, *dp.offset (1), *dp); }
  else
    {
      let mut shift: c_uint = 0;
      let mut d1: mp_limb_t;
      let mut d0: mp_limb_t;

      d1 = *dp.offset ((dn-1) as isize);
      d0 = *dp.offset ((dn-2) as isize);
      assert! (d1 > 0);
      gmp_clz! (shift, d1);
      (*inv).shift = shift;
      if (shift > 0)
	{
	  d1 = (d1 << shift) | (d0 >> (GMP_LIMB_BITS - shift as usize));
	  d0 = (d0 << shift)
	         | (*dp.offset ((dn-3) as isize) >> (GMP_LIMB_BITS - shift as usize));
	}
      (*inv).d1 = d1;
      (*inv).d0 = d0;
      (*inv).di = mpn_invert_3by2 (d1, d0);
    }

  }
}

/* Not matching current public gmp interface, rather corresponding to
   the sbpi1_div_* functions. */
unsafe extern "C-unwind" fn mpn_div_qr_1_preinv (qp: mp_ptr, np: mp_srcptr, mut nn: mp_size_t,
                                          inv: *const gmp_div_inverse) -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut d: mp_limb_t;
  let mut di: mp_limb_t;
  let mut r: mp_limb_t;
  let mut tp: mp_ptr = std::ptr::null_mut ();
  let mut tn: mp_size_t = 0;
  let mut np = np;

  if ((*inv).shift > 0)
    {
      /* Shift, reusing qp area if possible. In-place shift if qp == np. */
      tp = qp;
      if tp.is_null ()
        {
	   tn = nn;
	   tp = gmp_alloc_limbs (tn);
        }
      r = mpn_lshift (tp, np, nn, (*inv).shift);
      np = tp;
    }
  else
    { r = 0; }

  d = (*inv).d1;
  di = (*inv).di;
  while { nn -= 1; nn >= 0 }
    {
      let mut q: mp_limb_t = 0;

      gmp_udiv_qrnnd_preinv! (q, r, r, *np.offset (nn as isize), d, di);
      if !qp.is_null ()
	{ *qp.offset (nn as isize) = q; }
    }
  if tn != 0
    { gmp_free_limbs (tp, tn); }

  return r >> (*inv).shift;

  }
}

unsafe extern "C-unwind" fn mpn_div_qr_2_preinv (qp: mp_ptr, np: mp_ptr, nn: mp_size_t,
                                          inv: *const gmp_div_inverse)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut shift: c_uint = 0;
  let mut i: mp_size_t;
  let mut d1: mp_limb_t;
  let mut d0: mp_limb_t;
  let mut di: mp_limb_t;
  let mut r1: mp_limb_t;
  let mut r0: mp_limb_t;

  assert! (nn >= 2);
  shift = (*inv).shift;
  d1 = (*inv).d1;
  d0 = (*inv).d0;
  di = (*inv).di;

  if (shift > 0)
    { r1 = mpn_lshift (np, np, nn, shift); }
  else
    { r1 = 0; }

  r0 = *np.offset ((nn - 1) as isize);

  i = nn - 2;
  loop
    {
      let n0: mp_limb_t;
      let mut q: mp_limb_t = 0;
      n0 = *np.offset (i as isize);
      gmp_udiv_qr_3by2! (q, r1, r0, r1, r0, n0, d1, d0, di);

      if !qp.is_null ()
	{ *qp.offset (i as isize) = q; }
      i -= 1;
      if ! (i >= 0) { break; }
    }

  if (shift > 0)
    {
      assert! ((r0 & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - shift as usize))) == 0);
      r0 = (r0 >> shift) | (r1 << (GMP_LIMB_BITS - shift as usize));
      r1 >>= shift;
    }

  *np.offset (1) = r1;
  *np = r0;

  }
}

unsafe extern "C-unwind" fn mpn_div_qr_pi1 (qp: mp_ptr,
                                     np: mp_ptr, nn: mp_size_t, mut n1: mp_limb_t,
                                     dp: mp_srcptr, dn: mp_size_t,
                                     dinv: mp_limb_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;

  let mut d1: mp_limb_t;
  let mut d0: mp_limb_t;
  let mut cy: mp_limb_t;
  let mut cy1: mp_limb_t;
  let mut q: mp_limb_t = 0;

  assert! (dn > 2);
  assert! (nn >= dn);

  d1 = *dp.offset ((dn - 1) as isize);
  d0 = *dp.offset ((dn - 2) as isize);

  assert! ((d1 & GMP_LIMB_HIGHBIT) != 0);
  /* Iteration variable is the index of the q limb.
   *
   * We divide <n1, np[dn-1+i], np[dn-2+i], np[dn-3+i],..., np[i]>
   * by            <d1,          d0,        dp[dn-3],  ..., dp[0] >
   */

  i = nn - dn;
  loop
    {
      let mut n0: mp_limb_t = *np.offset ((dn-1+i) as isize);

      if (n1 == d1 && n0 == d0)
	{
	  q = GMP_LIMB_MAX;
	  mpn_submul_1 (np.offset (i as isize), dp, dn, q);
	  n1 = *np.offset ((dn-1+i) as isize);	/* update n1, last loop's value will now be invalid */
	}
      else
	{
	  gmp_udiv_qr_3by2! (q, n1, n0, n1, n0, *np.offset ((dn-2+i) as isize),
	                     d1, d0, dinv);

	  cy = mpn_submul_1 (np.offset (i as isize), dp, dn-2, q);

	  cy1 = (n0 < cy) as mp_limb_t;
	  n0 = n0.wrapping_sub (cy);
	  cy = (n1 < cy1) as mp_limb_t;
	  n1 = n1.wrapping_sub (cy1);
	  *np.offset ((dn-2+i) as isize) = n0;

	  if (cy != 0)
	    {
	      n1 = n1.wrapping_add (d1.wrapping_add (
	             mpn_add_n (np.offset (i as isize), np.offset (i as isize), dp,
	                        dn - 1)));
	      q -= 1;
	    }
	}

      if !qp.is_null ()
	{ *qp.offset (i as isize) = q; }
      i -= 1;
      if ! (i >= 0) { break; }
    }

  *np.offset ((dn - 1) as isize) = n1;

  }
}

unsafe extern "C-unwind" fn mpn_div_qr_preinv (qp: mp_ptr, np: mp_ptr, nn: mp_size_t,
                                        dp: mp_srcptr, dn: mp_size_t,
                                        inv: *const gmp_div_inverse)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  assert! (dn > 0);
  assert! (nn >= dn);

  if (dn == 1)
    { *np = mpn_div_qr_1_preinv (qp, np, nn, inv); }
  else if (dn == 2)
    { mpn_div_qr_2_preinv (qp, np, nn, inv); }
  else
    {
      let mut nh: mp_limb_t = 0;
      let mut shift: c_uint = 0;
      let mut dp = dp;

      assert! ((*inv).d1 == *dp.offset ((dn-1) as isize));
      assert! ((*inv).d0 == *dp.offset ((dn-2) as isize));
      assert! (((*inv).d1 & GMP_LIMB_HIGHBIT) != 0);

      shift = (*inv).shift;
      if (shift > 0)
	{ nh = mpn_lshift (np, np, nn, shift); }
      else
	{ nh = 0; }

      mpn_div_qr_pi1 (qp, np, nn, nh, dp, dn, (*inv).di);

      if (shift > 0)
	{ gmp_assert_nocarry! (mpn_rshift (np, np, dn, shift)); }
    }

  }
}

unsafe extern "C-unwind" fn mpn_div_qr (qp: mp_ptr, np: mp_ptr, nn: mp_size_t,
                                 dp: mp_srcptr, dn: mp_size_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut inv: gmp_div_inverse
    = gmp_div_inverse { shift: 0, d1: 0, d0: 0, di: 0 };
  let mut tp: mp_ptr = std::ptr::null_mut ();
  let mut dp = dp;

  assert! (dn > 0);
  assert! (nn >= dn);

  mpn_div_qr_invert (&mut inv, dp, dn);
  if (dn > 2 && inv.shift > 0)
    {
      tp = gmp_alloc_limbs (dn);
      gmp_assert_nocarry! (mpn_lshift (tp, dp, dn, inv.shift));
      dp = tp;
    }
  mpn_div_qr_preinv (qp, np, nn, dp, dn, &inv);
  if !tp.is_null ()
    { gmp_free_limbs (tp, dn); }

  }
}

/* MPN base conversion. */
unsafe extern "C-unwind" fn mpn_base_power_of_two_p (b: c_uint) -> c_uint
{
  match b
    {
    2 => { return 1; }
    4 => { return 2; }
    8 => { return 3; }
    16 => { return 4; }
    32 => { return 5; }
    64 => { return 6; }
    128 => { return 7; }
    256 => { return 8; }
    _ => { return 0; }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mpn_base_info
{
  /* bb is the largest power of the base which fits in one limb, and
     exp is the corresponding exponent. */
  pub exp: c_uint,
  pub bb: mp_limb_t,
}

unsafe extern "C-unwind" fn mpn_get_base_info (info: *mut mpn_base_info, b: mp_limb_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut m: mp_limb_t;
  let mut p: mp_limb_t;
  let mut exp: c_uint;

  m = GMP_LIMB_MAX / b;
  exp = 1;
  p = b;
  while p <= m
    { p = p.wrapping_mul (b); exp += 1; }

  (*info).exp = exp;
  (*info).bb = p;

  }
}

unsafe extern "C-unwind" fn mpn_limb_size_in_base_2 (u: mp_limb_t) -> mp_bitcnt_t
{
  let mut shift: c_uint = 0;

  assert! (u > 0);
  gmp_clz! (shift, u);
  return (GMP_LIMB_BITS - shift as usize) as mp_bitcnt_t;
}

unsafe extern "C-unwind" fn mpn_get_str_bits (sp: *mut u8, bits: c_uint, up: mp_srcptr,
                                      un: mp_size_t) -> usize
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut mask: u8;
  let mut sn: usize;
  let mut j: usize;
  let mut i: mp_size_t;
  let mut shift: c_uint;

  sn = ((un - 1) as usize * GMP_LIMB_BITS
	+ mpn_limb_size_in_base_2 (*up.offset ((un-1) as isize)) as usize
	+ bits as usize - 1) / bits as usize;

  mask = (((1 as c_uint) << bits) - 1) as u8;

  i = 0;
  j = sn;
  shift = 0;
  while j > 0
    {
      j -= 1;
      let mut digit: u8 = (*up.offset (i as isize) >> shift) as u8;

      shift += bits;

      if shift as usize >= GMP_LIMB_BITS
	{
	  i += 1;
	  if i < un
	    {
	      shift -= GMP_LIMB_BITS as c_uint;
	      digit |= (*up.offset (i as isize) << (bits - shift)) as u8;
	    }
	}
      *sp.offset (j as isize) = digit & mask;
    }
  return sn;

  }
}

/* We generate digits from the least significant end, and reverse at
   the end. */
unsafe extern "C-unwind" fn mpn_limb_get_str (sp: *mut u8, mut w: mp_limb_t,
                                       binv: *const gmp_div_inverse) -> usize
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;
  i = 0;
  while w > 0
    {
      let h: mp_limb_t;
      let l: mp_limb_t;
      let mut r: mp_limb_t = 0;

      h = w >> (GMP_LIMB_BITS - (*binv).shift as usize);
      l = w << (*binv).shift;

      gmp_udiv_qrnnd_preinv! (w, r, h, l, (*binv).d1, (*binv).di);
      assert! ((r & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - (*binv).shift as usize))) == 0);
      r >>= (*binv).shift;

      *sp.offset (i as isize) = r as u8;
      i += 1;
    }
  return i as usize;

  }
}

unsafe extern "C-unwind" fn mpn_get_str_other (sp: *mut u8,
                                        base: c_int, info: *const mpn_base_info,
                                        up: mp_ptr, mut un: mp_size_t) -> usize
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut binv: gmp_div_inverse = gmp_div_inverse { shift: 0, d1: 0, d0: 0, di: 0 };
  let mut sn: usize;
  let mut i: usize;

  mpn_div_qr_1_invert (&mut binv, base as mp_limb_t);

  sn = 0;

  if (un > 1)
    {
      let mut bbinv: gmp_div_inverse = gmp_div_inverse { shift: 0, d1: 0, d0: 0, di: 0 };
      mpn_div_qr_1_invert (&mut bbinv, (*info).bb);

      loop
	{
	  let mut w: mp_limb_t;
	  let mut done: usize;
	  w = mpn_div_qr_1_preinv (up, up, un, &bbinv);
	  un -= (*up.offset ((un-1) as isize) == 0) as mp_size_t;
	  done = mpn_limb_get_str (sp.offset (sn as isize), w, &binv);

	  sn += done;
	  while done < (*info).exp as usize
	    { *sp.offset (sn as isize) = 0; sn += 1; done += 1; }
	  if ! (un > 1) { break; }
	}
    }
  sn += mpn_limb_get_str (sp.offset (sn as isize), *up, &binv);

  /* Reverse order */
  i = 0;
  while 2*i + 1 < sn
    {
      let t: u8 = *sp.offset (i as isize);
      *sp.offset (i as isize) = *sp.offset ((sn - i - 1) as isize);
      *sp.offset ((sn - i - 1) as isize) = t;
      i += 1;
    }

  return sn;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_get_str (sp: *mut u8, base: c_int, up: mp_ptr,
                                      un: mp_size_t) -> usize
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut bits: c_uint;

  assert! (un > 0);
  assert! (*up.offset ((un-1) as isize) > 0);

  bits = mpn_base_power_of_two_p (base as c_uint);
  if bits != 0
    { return mpn_get_str_bits (sp, bits, up, un); }
  else
    {
      let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };

      mpn_get_base_info (&mut info, base as mp_limb_t);
      return mpn_get_str_other (sp, base, &info, up, un);
    }

  }
}

unsafe extern "C-unwind" fn mpn_set_str_bits (rp: mp_ptr, sp: *const u8, mut sn: usize,
                                       bits: c_uint) -> mp_size_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut rn: mp_size_t;
  let mut limb: mp_limb_t;
  let mut shift: c_uint;

  limb = 0;
  rn = 0;
  shift = 0;
  while sn > 0
    {
      sn -= 1;
      limb |= (*sp.offset (sn as isize) as mp_limb_t) << shift;
      shift += bits;
      if shift as usize >= GMP_LIMB_BITS
	{
	  shift -= GMP_LIMB_BITS as c_uint;
	  *rp.offset (rn as isize) = limb;
	  rn += 1;
	  /* Next line is correct also if shift == 0,
	     bits == 8, and mp_limb_t == unsigned char. */
	  limb = ((*sp.offset (sn as isize)) as c_uint >> (bits - shift)) as mp_limb_t;
	}
    }
  if limb != 0
    { *rp.offset (rn as isize) = limb; rn += 1; }
  else
    { rn = mpn_normalized_size (rp, rn); }
  return rn;

  }
}

/* Result is usually normalized, except for all-zero input, in which
   case a single zero limb is written at *RP, and 1 is returned. */
unsafe extern "C-unwind" fn mpn_set_str_other (rp: mp_ptr, sp: *const u8, sn: usize,
                                        b: mp_limb_t, info: *const mpn_base_info)
  -> mp_size_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut rn: mp_size_t;
  let mut w: mp_limb_t;
  let mut k: c_uint;
  let mut j: usize;

  assert! (sn > 0);

  k = (1 + (sn - 1) % (*info).exp as usize) as c_uint;

  j = 0;
  w = *sp.offset (j as isize) as mp_limb_t;
  j += 1;
  while { k -= 1; k != 0 }
    { w = w.wrapping_mul (b).wrapping_add (*sp.offset (j as isize) as mp_limb_t); j += 1; }

  *rp = w;

  rn = 1;
  while j < sn
    {
      let mut cy: mp_limb_t;

      w = *sp.offset (j as isize) as mp_limb_t;
      j += 1;
      k = 1;
      while k < (*info).exp
	{ w = w.wrapping_mul (b).wrapping_add (*sp.offset (j as isize) as mp_limb_t); j += 1; k += 1; }

      cy = mpn_mul_1 (rp, rp, rn, (*info).bb);
      cy = cy.wrapping_add (mpn_add_1 (rp, rp, rn, w));
      if (cy > 0)
	{ *rp.offset (rn as isize) = cy; rn += 1; }
    }
  assert! (j == sn);

  return rn;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_set_str (rp: mp_ptr, sp: *const u8, sn: usize,
                                      base: c_int) -> mp_size_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut bits: c_uint;

  if (sn == 0)
    { return 0; }

  bits = mpn_base_power_of_two_p (base as c_uint);
  if bits != 0
    { return mpn_set_str_bits (rp, sp, sn, bits); }
  else
    {
      let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };

      mpn_get_base_info (&mut info, base as mp_limb_t);
      return mpn_set_str_other (rp, sp, sn, base as mp_limb_t, &info);
    }

  }
}

/* MPZ interface */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init (r: *mut __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  static dummy_limb: mp_limb_t = GMP_LIMB_MAX & 0xc1a0;

  (*r)._mp_alloc = 0;
  (*r)._mp_size = 0;
  (*r)._mp_d = &dummy_limb as *const mp_limb_t as mp_ptr;

  }
}

/* The utility of this function is a bit limited, since many functions
   assigns the result variable using mpz_swap. */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init2 (r: *mut __mpz_struct, mut bits: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let rn: mp_size_t;

  bits -= (bits != 0) as mp_bitcnt_t;		/* Round down, except if 0 */
  rn = (1 + bits as usize / GMP_LIMB_BITS) as mp_size_t;

  (*r)._mp_alloc = rn as c_int;
  (*r)._mp_size = 0;
  (*r)._mp_d = gmp_alloc_limbs (rn);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_clear (r: *mut __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if ((*r)._mp_alloc != 0)
    { gmp_free_limbs ((*r)._mp_d, (*r)._mp_alloc as mp_size_t); }

  }
}

unsafe extern "C-unwind" fn mpz_realloc (r: *mut __mpz_struct, size: mp_size_t) -> mp_ptr
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut size = size;
  size = GMP_MAX! (size, 1);

  if ((*r)._mp_alloc != 0)
    { (*r)._mp_d = gmp_realloc_limbs ((*r)._mp_d, (*r)._mp_alloc as mp_size_t, size); }
  else
    { (*r)._mp_d = gmp_alloc_limbs (size); }
  (*r)._mp_alloc = size as c_int;

  if (GMP_ABS! ((*r)._mp_size) as mp_size_t > size)
    { (*r)._mp_size = 0; }

  return (*r)._mp_d;

  }
}

/* Realloc for an mpz_t WHAT if it has less than NEEDED limbs.  */
/* #define MPZ_REALLOC(z,n) ((n) > (z)->_mp_alloc ? mpz_realloc(z,n) : (z)->_mp_d)  */
macro_rules! MPZ_REALLOC {
    ($z:expr, $n:expr) => {
        (if ($n) as mp_size_t > (*$z)._mp_alloc as mp_size_t
           { mpz_realloc ($z, ($n) as mp_size_t) }
         else { (*$z)._mp_d })
    };
}

/* MPZ assignment and basic conversions. */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set_si (r: *mut __mpz_struct, x: c_long)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if (x >= 0)
    { mpz_set_ui (r, x as c_ulong); }
  else /* (x < 0) */
    { if (GMP_LIMB_BITS < GMP_ULONG_BITS)
      {
	mpz_set_ui (r, GMP_NEG_CAST! (c_ulong, x));
	mpz_neg (r, r);
      }
    else
      {
	(*r)._mp_size = -1;
	*MPZ_REALLOC! (r, 1) = GMP_NEG_CAST! (c_ulong, x);
      } }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set_ui (r: *mut __mpz_struct, mut x: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if (x > 0)
    {
      (*r)._mp_size = 1;
      *MPZ_REALLOC! (r, 1) = x;
      if (GMP_LIMB_BITS < GMP_ULONG_BITS)
	{
	  let LOCAL_GMP_LIMB_BITS: c_int = GMP_LIMB_BITS as c_int;
	  loop
	    {
	      x = x.wrapping_shr (LOCAL_GMP_LIMB_BITS as u32);
	      if x == 0 { break; }
	      (*r)._mp_size += 1;
	      *MPZ_REALLOC! (r, (*r)._mp_size).offset ((*r)._mp_size as isize - 1) = x;
	    }
	}
    }
  else
    { (*r)._mp_size = 0; }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set (r: *mut __mpz_struct, x: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  /* Allow the NOP r == x */
  if (r as *const __mpz_struct != x)
    {
      let n: mp_size_t;
      let rp: mp_ptr;

      n = GMP_ABS! ((*x)._mp_size) as mp_size_t;
      rp = MPZ_REALLOC! (r, n);

      mpn_copyi (rp, (*x)._mp_d, n);
      (*r)._mp_size = (*x)._mp_size;
    }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init_set_si (r: *mut __mpz_struct, x: c_long)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_init (r);
  mpz_set_si (r, x);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init_set_ui (r: *mut __mpz_struct, x: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_init (r);
  mpz_set_ui (r, x);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init_set (r: *mut __mpz_struct, x: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_init (r);
  mpz_set (r, x);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fits_slong_p (u: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return (mpz_cmp_si (u, LONG_MAX) <= 0 && mpz_cmp_si (u, LONG_MIN) >= 0) as c_int;

  }
}

unsafe extern "C-unwind" fn mpn_absfits_ulong_p (up: mp_srcptr, un: mp_size_t) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let ulongsize: c_int = (GMP_ULONG_BITS / GMP_LIMB_BITS) as c_int;
  let mut ulongrem: mp_limb_t = 0;

  if (GMP_ULONG_BITS % GMP_LIMB_BITS != 0)
    { ulongrem = (ULONG_MAX >> (GMP_LIMB_BITS * ulongsize as usize)) as mp_limb_t + 1; }

  return (un <= ulongsize as mp_size_t
          || (*up.offset (ulongsize as isize) < ulongrem
              && un == ulongsize as mp_size_t + 1)) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fits_ulong_p (u: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let us: mp_size_t = (*u)._mp_size as mp_size_t;

  return (us >= 0 && mpn_absfits_ulong_p ((*u)._mp_d, us) != 0) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fits_sint_p (u: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return (mpz_cmp_si (u, INT_MAX as c_long) <= 0
          && mpz_cmp_si (u, INT_MIN as c_long) >= 0) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fits_uint_p (u: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return ((*u)._mp_size >= 0
          && mpz_cmpabs_ui (u, UINT_MAX as c_ulong) <= 0) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fits_sshort_p (u: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return (mpz_cmp_si (u, SHRT_MAX as c_long) <= 0
          && mpz_cmp_si (u, SHRT_MIN as c_long) >= 0) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fits_ushort_p (u: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return ((*u)._mp_size >= 0
          && mpz_cmpabs_ui (u, USHRT_MAX as c_ulong) <= 0) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_get_si (u: *const __mpz_struct) -> c_long
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let r: c_ulong = mpz_get_ui (u);
  let c: c_ulong = (-LONG_MAX - LONG_MIN) as c_ulong;

  if ((*u)._mp_size < 0)
    /* This expression is necessary to properly handle -LONG_MIN */
    { return (-(c as c_long)).wrapping_sub (
               (r.wrapping_sub (c) & LONG_MAX as c_ulong) as c_long); }
  else
    { return (r & LONG_MAX as c_ulong) as c_long; }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_get_ui (u: *const __mpz_struct) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if (GMP_LIMB_BITS < GMP_ULONG_BITS)
    {
      let LOCAL_GMP_LIMB_BITS: c_int = GMP_LIMB_BITS as c_int;
      let mut r: c_ulong = 0;
      let mut n: mp_size_t = GMP_ABS! ((*u)._mp_size) as mp_size_t;
      n = GMP_MIN! (n, (1 + (GMP_ULONG_BITS - 1) / GMP_LIMB_BITS) as mp_size_t);
      while { n -= 1; n >= 0 }
	{ r = r.wrapping_shl (LOCAL_GMP_LIMB_BITS as u32)
	        .wrapping_add (*(*u)._mp_d.offset (n as isize)); }
      return r;
    }

  return if (*u)._mp_size == 0 { 0 } else { *(*u)._mp_d };

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_size (u: *const __mpz_struct) -> usize
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return GMP_ABS! ((*u)._mp_size) as usize;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_getlimbn (u: *const __mpz_struct, n: mp_size_t)
  -> mp_limb_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if (n >= 0 && n < GMP_ABS! ((*u)._mp_size) as mp_size_t)
    { return *(*u)._mp_d.offset (n as isize); }
  else
    { return 0; }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_realloc2 (x: *mut __mpz_struct, n: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_realloc (x, (1 + (n - (n != 0) as mp_bitcnt_t) as usize / GMP_LIMB_BITS)
                  as mp_size_t);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_limbs_read (x: *const __mpz_struct) -> mp_srcptr
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return (*x)._mp_d as mp_srcptr;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_limbs_modify (x: *mut __mpz_struct, n: mp_size_t)
  -> mp_ptr
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  assert! (n > 0);
  return MPZ_REALLOC! (x, n);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_limbs_write (x: *mut __mpz_struct, n: mp_size_t)
  -> mp_ptr
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_limbs_modify (x, n);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_limbs_finish (x: *mut __mpz_struct, xs: mp_size_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let xn: mp_size_t;
  xn = mpn_normalized_size ((*x)._mp_d, GMP_ABS! (xs));
  (*x)._mp_size = (if xs < 0 { -xn } else { xn }) as c_int;

  }
}

unsafe extern "C-unwind" fn mpz_roinit_normal_n (x: *mut __mpz_struct, xp: mp_srcptr,
                                         xs: mp_size_t) -> mpz_srcptr
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  (*x)._mp_alloc = 0;
  (*x)._mp_d = xp as mp_ptr;
  (*x)._mp_size = xs as c_int;
  return x;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_roinit_n (x: *mut __mpz_struct, xp: mp_srcptr,
                                       xs: mp_size_t) -> mpz_srcptr
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_roinit_normal_n (x, xp, xs);
  mpz_limbs_finish (x, xs);
  return x;

  }
}

/* `#if MINI_GMP_ENABLE_FLOAT` — the frozen build enables it, so the block is ported.  */
/* Conversions and comparison to double. */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set_d (r: *mut __mpz_struct, mut x: c_double)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut sign: c_int;
  let mut rp: mp_ptr;
  let mut rn: mp_size_t;
  let mut i: mp_size_t;
  let mut B: c_double;
  let mut Bi: c_double;
  let mut f: mp_limb_t;

  /* x != x is true when x is a NaN, and x == x * 0.5 is true when x is
     zero or infinity. */
  if (x != x || x == x * 0.5)
    {
      (*r)._mp_size = 0;
      return;
    }

  sign = (x < 0.0) as c_int;
  if (sign != 0)
    { x = - x; }

  if (x < 1.0)
    {
      (*r)._mp_size = 0;
      return;
    }
  B = 4.0 * (GMP_LIMB_HIGHBIT >> 1) as c_double;
  Bi = 1.0 / B;
  rn = 1;
  while x >= B
    { x *= Bi; rn += 1; }

  rp = MPZ_REALLOC! (r, rn);

  f = x as mp_limb_t;
  x -= f as c_double;
  assert! (x < 1.0);
  i = rn-1;
  *rp.offset (i as isize) = f;
  while { i -= 1; i >= 0 }
    {
      x = B * x;
      f = x as mp_limb_t;
      x -= f as c_double;
      assert! (x < 1.0);
      *rp.offset (i as isize) = f;
    }

  (*r)._mp_size = (if sign != 0 { - rn } else { rn }) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init_set_d (r: *mut __mpz_struct, x: c_double)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_init (r);
  mpz_set_d (r, x);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_get_d (u: *const __mpz_struct) -> c_double
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut m: c_int;
  let mut l: mp_limb_t;
  let mut un: mp_size_t;
  let mut x: c_double;
  let B: c_double = 4.0 * (GMP_LIMB_HIGHBIT >> 1) as c_double;

  un = GMP_ABS! ((*u)._mp_size) as mp_size_t;

  if (un == 0)
    { return 0.0; }

  un -= 1;
  l = *(*u)._mp_d.offset (un as isize);
  /* C's `unsigned m` becomes an `int` here (its declaration is `int m`), and the C macro
     assigns an `unsigned`; the port takes the count as `unsigned` and converts once.  */
  let mut __clz: c_uint = 0;
  gmp_clz! (__clz, l);
  m = __clz as c_int;
  m = m + GMP_DBL_MANT_BITS - GMP_LIMB_BITS as c_int;
  if (m < 0)
    { l &= GMP_LIMB_MAX << -m as usize; }

  x = l as c_double;
  while { un -= 1; un >= 0 }
    {
      x = B*x;
      if (m > 0) {
	l = *(*u)._mp_d.offset (un as isize);
	m -= GMP_LIMB_BITS as c_int;
	if (m < 0)
	  { l &= GMP_LIMB_MAX << -m as usize; }
	x += l as c_double;
      }
    }

  if ((*u)._mp_size < 0)
    { x = -x; }

  return x;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmpabs_d (x: *const __mpz_struct, d: c_double) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut xn: mp_size_t;
  let mut B: c_double;
  let mut Bi: c_double;
  let mut i: mp_size_t;
  let mut d = d;

  xn = (*x)._mp_size as mp_size_t;
  d = GMP_ABS! (d);

  if (xn != 0)
    {
      xn = GMP_ABS! (xn);

      B = 4.0 * (GMP_LIMB_HIGHBIT >> 1) as c_double;
      Bi = 1.0 / B;

      /* Scale d so it can be compared with the top limb. */
      i = 1;
      while i < xn
	{ d *= Bi; i += 1; }

      if (d >= B)
	{ return -1; }

      /* Compare floor(d) to top limb, subtract and cancel when equal. */
      i = xn;
      while { i -= 1; i >= 0 } 
	{
	  let f: mp_limb_t;
	  let xl: mp_limb_t;

	  f = d as mp_limb_t;
	  xl = *(*x)._mp_d.offset (i as isize);
	  if (xl > f)
	    { return 1; }
	  else if (xl < f)
	    { return -1; }
	  d = B * (d - f as c_double);
	}
    }
  return -((d > 0.0) as c_int);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmp_d (x: *const __mpz_struct, d: c_double) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if ((*x)._mp_size < 0)
    {
      if (d >= 0.0)
	{ return -1; }
      else
	{ return -mpz_cmpabs_d (x, d); }
    }
  else
    {
      if (d < 0.0)
	{ return 1; }
      else
	{ return mpz_cmpabs_d (x, d); }
    }

  }
}

/* The port's spelling of a zero-initialized `mpz_t` local: C's `mpz_t x;` has no
   initializer, and Rust needs one before `x.as_mut_ptr ()` can be taken
   (PORTING.md 1.5); every such local is immediately `mpz_init`ed.  */
#[inline(always)]
fn zero_mpz () -> mpz_t
{ [__mpz_struct { _mp_alloc: 0, _mp_size: 0, _mp_d: std::ptr::null_mut () }; 1] }

/* MPZ comparisons and the like. */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sgn (u: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return GMP_CMP! ((*u)._mp_size, 0);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmp_si (u: *const __mpz_struct, v: c_long) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let usize_: mp_size_t = (*u)._mp_size as mp_size_t;

  if (v >= 0)
    { return mpz_cmp_ui (u, v as c_ulong); }
  else if (usize_ >= 0)
    { return 1; }
  else
    { return - mpz_cmpabs_ui (u, GMP_NEG_CAST! (c_ulong, v)); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmp_ui (u: *const __mpz_struct, v: c_ulong) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let usize_: mp_size_t = (*u)._mp_size as mp_size_t;

  if (usize_ < 0)
    { return -1; }
  else
    { return mpz_cmpabs_ui (u, v); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmp (a: *const __mpz_struct, b: *const __mpz_struct)
  -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let asize: mp_size_t = (*a)._mp_size as mp_size_t;
  let bsize: mp_size_t = (*b)._mp_size as mp_size_t;

  if (asize != bsize)
    { return if asize < bsize { -1 } else { 1 }; }
  else if (asize >= 0)
    { return mpn_cmp ((*a)._mp_d, (*b)._mp_d, asize); }
  else
    { return mpn_cmp ((*b)._mp_d, (*a)._mp_d, -asize); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmpabs_ui (u: *const __mpz_struct, v: c_ulong) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let un: mp_size_t = GMP_ABS! ((*u)._mp_size) as mp_size_t;

  if !(mpn_absfits_ulong_p ((*u)._mp_d, un) != 0)
    { return 1; }
  else
    {
      let uu: c_ulong = mpz_get_ui (u);
      return GMP_CMP! (uu, v);
    }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmpabs (u: *const __mpz_struct, v: *const __mpz_struct)
  -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpn_cmp4 ((*u)._mp_d, GMP_ABS! ((*u)._mp_size) as mp_size_t,
		   (*v)._mp_d, GMP_ABS! ((*v)._mp_size) as mp_size_t);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_abs (r: *mut __mpz_struct, u: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_set (r, u);
  (*r)._mp_size = GMP_ABS! ((*r)._mp_size);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_neg (r: *mut __mpz_struct, u: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_set (r, u);
  (*r)._mp_size = -(*r)._mp_size;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_swap (u: *mut __mpz_struct, v: *mut __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  MP_SIZE_T_SWAP! ((*u)._mp_alloc, (*v)._mp_alloc);
  MPN_PTR_SWAP! ((*u)._mp_d, (*u)._mp_size, (*v)._mp_d, (*v)._mp_size);

  }
}

/* MPZ addition and subtraction */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_add_ui (r: *mut __mpz_struct, a: *const __mpz_struct,
                                     b: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut bb: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                      _mp_d: std::ptr::null_mut () }; 1];
  mpz_init_set_ui (bb.as_mut_ptr (), b);
  mpz_add (r, a, bb.as_ptr ());
  mpz_clear (bb.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sub_ui (r: *mut __mpz_struct, a: *const __mpz_struct,
                                     b: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_ui_sub (r, b, a);
  mpz_neg (r, r);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_ui_sub (r: *mut __mpz_struct, a: c_ulong,
                                     b: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_neg (r, b);
  mpz_add_ui (r, r, a);

  }
}

unsafe extern "C-unwind" fn mpz_abs_add (r: *mut __mpz_struct, a: *const __mpz_struct,
                                  b: *const __mpz_struct) -> mp_size_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut an: mp_size_t = GMP_ABS! ((*a)._mp_size) as mp_size_t;
  let mut bn: mp_size_t = GMP_ABS! ((*b)._mp_size) as mp_size_t;
  let rp: mp_ptr;
  let cy: mp_limb_t;
  let mut a = a;
  let mut b = b;

  if (an < bn)
    {
      MPZ_SRCPTR_SWAP! (a, b);
      MP_SIZE_T_SWAP! (an, bn);
    }

  rp = MPZ_REALLOC! (r, an + 1);
  cy = mpn_add (rp, (*a)._mp_d, an, (*b)._mp_d, bn);

  *rp.offset (an as isize) = cy;

  return an + cy as mp_size_t;

  }
}

unsafe extern "C-unwind" fn mpz_abs_sub (r: *mut __mpz_struct, a: *const __mpz_struct,
                                  b: *const __mpz_struct) -> mp_size_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let an: mp_size_t = GMP_ABS! ((*a)._mp_size) as mp_size_t;
  let bn: mp_size_t = GMP_ABS! ((*b)._mp_size) as mp_size_t;
  let mut cmp: c_int;
  let mut rp: mp_ptr;

  cmp = mpn_cmp4 ((*a)._mp_d, an, (*b)._mp_d, bn);
  if (cmp > 0)
    {
      rp = MPZ_REALLOC! (r, an);
      gmp_assert_nocarry! (mpn_sub (rp, (*a)._mp_d, an, (*b)._mp_d, bn));
      return mpn_normalized_size (rp, an);
    }
  else if (cmp < 0)
    {
      rp = MPZ_REALLOC! (r, bn);
      gmp_assert_nocarry! (mpn_sub (rp, (*b)._mp_d, bn, (*a)._mp_d, an));
      return -mpn_normalized_size (rp, bn);
    }
  else
    { return 0; }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_add (r: *mut __mpz_struct, a: *const __mpz_struct,
                                  b: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let rn: mp_size_t;

  if ( ((*a)._mp_size ^ (*b)._mp_size) >= 0)
    { rn = mpz_abs_add (r, a, b); }
  else
    { rn = mpz_abs_sub (r, a, b); }

  (*r)._mp_size = (if (*a)._mp_size >= 0 { rn } else { - rn }) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sub (r: *mut __mpz_struct, a: *const __mpz_struct,
                                  b: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let rn: mp_size_t;

  if ( ((*a)._mp_size ^ (*b)._mp_size) >= 0)
    { rn = mpz_abs_sub (r, a, b); }
  else
    { rn = mpz_abs_add (r, a, b); }

  (*r)._mp_size = (if (*a)._mp_size >= 0 { rn } else { - rn }) as c_int;

  }
}

/* MPZ multiplication */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mul_si (r: *mut __mpz_struct, u: *const __mpz_struct,
                                     v: c_long)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if (v < 0)
    {
      mpz_mul_ui (r, u, GMP_NEG_CAST! (c_ulong, v));
      mpz_neg (r, r);
    }
  else
    { mpz_mul_ui (r, u, v as c_ulong); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mul_ui (r: *mut __mpz_struct, u: *const __mpz_struct,
                                     v: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut vv: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                      _mp_d: std::ptr::null_mut () }; 1];
  mpz_init_set_ui (vv.as_mut_ptr (), v);
  mpz_mul (r, u, vv.as_ptr ());
  mpz_clear (vv.as_mut_ptr ());
  return;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mul (r: *mut __mpz_struct, u: *const __mpz_struct,
                                  v: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut sign: c_int;
  let mut un: mp_size_t;
  let mut vn: mp_size_t;
  let mut rn: mp_size_t;
  let mut t: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                     _mp_d: std::ptr::null_mut () }; 1];
  let tp: mp_ptr;

  un = (*u)._mp_size as mp_size_t;
  vn = (*v)._mp_size as mp_size_t;

  if (un == 0 || vn == 0)
    {
      (*r)._mp_size = 0;
      return;
    }

  sign = ((un ^ vn) < 0) as c_int;

  un = GMP_ABS! (un);
  vn = GMP_ABS! (vn);

  mpz_init2 (t.as_mut_ptr (), ((un + vn) as usize * GMP_LIMB_BITS) as mp_bitcnt_t);

  tp = t[0]._mp_d;
  if (un >= vn)
    { mpn_mul (tp, (*u)._mp_d, un, (*v)._mp_d, vn); }
  else
    { mpn_mul (tp, (*v)._mp_d, vn, (*u)._mp_d, un); }

  rn = un + vn;
  rn -= (*tp.offset ((rn-1) as isize) == 0) as mp_size_t;

  t[0]._mp_size = (if sign != 0 { - rn } else { rn }) as c_int;
  mpz_swap (r, t.as_mut_ptr ());
  mpz_clear (t.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mul_2exp (r: *mut __mpz_struct, u: *const __mpz_struct,
                                       bits: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let un: mp_size_t;
  let mut rn: mp_size_t;
  let mut limbs: mp_size_t;
  let mut shift: c_uint;
  let mut rp: mp_ptr;

  un = GMP_ABS! ((*u)._mp_size) as mp_size_t;
  if (un == 0)
    {
      (*r)._mp_size = 0;
      return;
    }

  limbs = (bits / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;
  shift = (bits % GMP_LIMB_BITS as mp_bitcnt_t) as c_uint;

  rn = un + limbs + (shift > 0) as mp_size_t;
  rp = MPZ_REALLOC! (r, rn);
  if (shift > 0)
    {
      let cy: mp_limb_t = mpn_lshift (rp.offset (limbs as isize), (*u)._mp_d, un, shift);
      *rp.offset ((rn-1) as isize) = cy;
      rn -= (cy == 0) as mp_size_t;
    }
  else
    { mpn_copyd (rp.offset (limbs as isize), (*u)._mp_d, un); }

  mpn_zero (rp, limbs);

  (*r)._mp_size = (if (*u)._mp_size < 0 { - rn } else { rn }) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_addmul_ui (r: *mut __mpz_struct, u: *const __mpz_struct,
                                        v: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut t: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                     _mp_d: std::ptr::null_mut () }; 1];
  mpz_init_set_ui (t.as_mut_ptr (), v);
  mpz_mul (t.as_mut_ptr (), u, t.as_ptr ());
  mpz_add (r, r, t.as_ptr ());
  mpz_clear (t.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_submul_ui (r: *mut __mpz_struct, u: *const __mpz_struct,
                                        v: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut t: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                     _mp_d: std::ptr::null_mut () }; 1];
  mpz_init_set_ui (t.as_mut_ptr (), v);
  mpz_mul (t.as_mut_ptr (), u, t.as_ptr ());
  mpz_sub (r, r, t.as_ptr ());
  mpz_clear (t.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_addmul (r: *mut __mpz_struct, u: *const __mpz_struct,
                                     v: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut t: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                     _mp_d: std::ptr::null_mut () }; 1];
  mpz_init (t.as_mut_ptr ());
  mpz_mul (t.as_mut_ptr (), u, v);
  mpz_add (r, r, t.as_ptr ());
  mpz_clear (t.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_submul (r: *mut __mpz_struct, u: *const __mpz_struct,
                                     v: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut t: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                     _mp_d: std::ptr::null_mut () }; 1];
  mpz_init (t.as_mut_ptr ());
  mpz_mul (t.as_mut_ptr (), u, v);
  mpz_sub (r, r, t.as_ptr ());
  mpz_clear (t.as_mut_ptr ());

  }
}

/* MPZ division */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum mpz_div_round_mode { GMP_DIV_FLOOR, GMP_DIV_CEIL, GMP_DIV_TRUNC }

/* Allows q or r to be zero. Returns 1 iff remainder is non-zero. */
unsafe extern "C-unwind" fn mpz_div_qr (q: *mut __mpz_struct, r: *mut __mpz_struct,
                                 n: *const __mpz_struct, d: *const __mpz_struct,
                                 mode: mpz_div_round_mode) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut ns: mp_size_t;
  let mut ds: mp_size_t;
  let mut nn: mp_size_t;
  let mut dn: mp_size_t;
  let mut qs: mp_size_t;
  ns = (*n)._mp_size as mp_size_t;
  ds = (*d)._mp_size as mp_size_t;

  if (ds == 0)
    { gmp_die(c"mpz_div_qr: Divide by zero.".as_ptr ()); }

  if (ns == 0)
    {
      if !q.is_null ()
	{ (*q)._mp_size = 0; }
      if !r.is_null ()
	{ (*r)._mp_size = 0; }
      return 0;
    }

  nn = GMP_ABS! (ns);
  dn = GMP_ABS! (ds);

  qs = ds ^ ns;

  if (nn < dn)
    {
      if (mode == mpz_div_round_mode::GMP_DIV_CEIL && qs >= 0)
	{
	  /* q = 1, r = n - d */
	  if !r.is_null ()
	    { mpz_sub (r, n, d); }
	  if !q.is_null ()
	    { mpz_set_ui (q, 1); }
	}
      else if (mode == mpz_div_round_mode::GMP_DIV_FLOOR && qs < 0)
	{
	  /* q = -1, r = n + d */
	  if !r.is_null ()
	    { mpz_add (r, n, d); }
	  if !q.is_null ()
	    { mpz_set_si (q, -1); }
	}
      else
	{
	  /* q = 0, r = d */
	  if !r.is_null ()
	    { mpz_set (r, n); }
	  if !q.is_null ()
	    { (*q)._mp_size = 0; }
	}
      return 1;
    }
  else
    {
      let mut np: mp_ptr;
      let mut qp: mp_ptr;
      let mut qn: mp_size_t;
      let mut rn: mp_size_t;
      let mut tq: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                          _mp_d: std::ptr::null_mut () }; 1];
      let mut tr: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                          _mp_d: std::ptr::null_mut () }; 1];

      mpz_init_set (tr.as_mut_ptr (), n);
      np = tr[0]._mp_d;

      qn = nn - dn + 1;

      if !q.is_null ()
	{
	  mpz_init2 (tq.as_mut_ptr (), (qn as usize * GMP_LIMB_BITS) as mp_bitcnt_t);
	  qp = tq[0]._mp_d;
	}
      else
	{ qp = std::ptr::null_mut (); }

      mpn_div_qr (qp, np, nn, (*d)._mp_d, dn);

      if !qp.is_null ()
	{
	  qn -= (*qp.offset ((qn-1) as isize) == 0) as mp_size_t;

	  tq[0]._mp_size = (if qs < 0 { -qn } else { qn }) as c_int;
	}
      rn = mpn_normalized_size (np, dn);
      tr[0]._mp_size = (if ns < 0 { - rn } else { rn }) as c_int;

      if (mode == mpz_div_round_mode::GMP_DIV_FLOOR && qs < 0 && rn != 0)
	{
	  if !q.is_null ()
	    { mpz_sub_ui (tq.as_mut_ptr (), tq.as_ptr (), 1); }
	  if !r.is_null ()
	    { mpz_add (tr.as_mut_ptr (), tr.as_ptr (), d); }
	}
      else if (mode == mpz_div_round_mode::GMP_DIV_CEIL && qs >= 0 && rn != 0)
	{
	  if !q.is_null ()
	    { mpz_add_ui (tq.as_mut_ptr (), tq.as_ptr (), 1); }
	  if !r.is_null ()
	    { mpz_sub (tr.as_mut_ptr (), tr.as_ptr (), d); }
	}

      if !q.is_null ()
	{
	  mpz_swap (tq.as_mut_ptr (), q);
	  mpz_clear (tq.as_mut_ptr ());
	}
      if !r.is_null ()
	{ mpz_swap (tr.as_mut_ptr (), r); }

      mpz_clear (tr.as_mut_ptr ());

      return (rn != 0) as c_int;
    }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_qr (q: *mut __mpz_struct, r: *mut __mpz_struct,
                                      n: *const __mpz_struct, d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (q, r, n, d, mpz_div_round_mode::GMP_DIV_CEIL);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_qr (q: *mut __mpz_struct, r: *mut __mpz_struct,
                                      n: *const __mpz_struct, d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (q, r, n, d, mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_qr (q: *mut __mpz_struct, r: *mut __mpz_struct,
                                      n: *const __mpz_struct, d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (q, r, n, d, mpz_div_round_mode::GMP_DIV_TRUNC);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_q (q: *mut __mpz_struct, n: *const __mpz_struct,
                                     d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (q, std::ptr::null_mut (), n, d, mpz_div_round_mode::GMP_DIV_CEIL);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_q (q: *mut __mpz_struct, n: *const __mpz_struct,
                                     d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (q, std::ptr::null_mut (), n, d, mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_q (q: *mut __mpz_struct, n: *const __mpz_struct,
                                     d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (q, std::ptr::null_mut (), n, d, mpz_div_round_mode::GMP_DIV_TRUNC);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_r (r: *mut __mpz_struct, n: *const __mpz_struct,
                                     d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (std::ptr::null_mut (), r, n, d, mpz_div_round_mode::GMP_DIV_CEIL);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_r (r: *mut __mpz_struct, n: *const __mpz_struct,
                                     d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (std::ptr::null_mut (), r, n, d, mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_r (r: *mut __mpz_struct, n: *const __mpz_struct,
                                     d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (std::ptr::null_mut (), r, n, d, mpz_div_round_mode::GMP_DIV_TRUNC);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mod (r: *mut __mpz_struct, n: *const __mpz_struct,
                                  d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_qr (std::ptr::null_mut (), r, n, d,
              if (*d)._mp_size >= 0 { mpz_div_round_mode::GMP_DIV_FLOOR }
              else { mpz_div_round_mode::GMP_DIV_CEIL });

  }
}

unsafe extern "C-unwind" fn mpz_div_q_2exp (q: *mut __mpz_struct, u: *const __mpz_struct,
                                     mut bit_index: mp_bitcnt_t,
                                     mode: mpz_div_round_mode)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut un: mp_size_t;
  let mut qn: mp_size_t;
  let mut limb_cnt: mp_size_t;
  let mut qp: mp_ptr = std::ptr::null_mut ();
  let mut adjust: c_int;

  un = (*u)._mp_size as mp_size_t;
  if (un == 0)
    {
      (*q)._mp_size = 0;
      return;
    }
  limb_cnt = (bit_index / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;
  qn = GMP_ABS! (un) - limb_cnt;
  bit_index %= GMP_LIMB_BITS as mp_bitcnt_t;

  if (mode == (if un > 0 { mpz_div_round_mode::GMP_DIV_CEIL }
               else { mpz_div_round_mode::GMP_DIV_FLOOR })) /* un != 0 here. */
    /* Note: Below, the final indexing at limb_cnt is valid because at
       that point we have qn > 0. */
    { adjust = (qn <= 0
	      || mpn_zero_p ((*u)._mp_d, limb_cnt) == 0
	      || (*(*u)._mp_d.offset (limb_cnt as isize)
		  & (((1 as mp_limb_t) << bit_index).wrapping_sub (1))) != 0) as c_int; }
  else
    { adjust = 0; }

  if (qn <= 0)
    { qn = 0; }
  else
    {
      qp = MPZ_REALLOC! (q, qn);

      if (bit_index != 0)
	{
	  mpn_rshift (qp, (*u)._mp_d.offset (limb_cnt as isize), qn, bit_index as c_uint);
	  qn -= (*qp.offset ((qn - 1) as isize) == 0) as mp_size_t;
	}
      else
	{
	  mpn_copyi (qp, (*u)._mp_d.offset (limb_cnt as isize), qn);
	}
    }

  (*q)._mp_size = qn as c_int;

  if (adjust != 0)
    { mpz_add_ui (q, q, 1); }
  if (un < 0)
    { mpz_neg (q, q); }

  }
}

unsafe extern "C-unwind" fn mpz_div_r_2exp (r: *mut __mpz_struct, u: *const __mpz_struct,
                                     bit_index: mp_bitcnt_t,
                                     mode: mpz_div_round_mode)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut us: mp_size_t;
  let mut un: mp_size_t;
  let mut rn: mp_size_t;
  let mut rp: mp_ptr;
  let mut mask: mp_limb_t;

  us = (*u)._mp_size as mp_size_t;
  if (us == 0 || bit_index == 0)
    {
      (*r)._mp_size = 0;
      return;
    }
  rn = ((bit_index + GMP_LIMB_BITS as mp_bitcnt_t - 1)
        / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;
  assert! (rn > 0);

  rp = MPZ_REALLOC! (r, rn);
  un = GMP_ABS! (us);

  mask = GMP_LIMB_MAX >> (rn as usize * GMP_LIMB_BITS - bit_index as usize);

  if (rn > un)
    {
      /* Quotient (with truncation) is zero, and remainder is
	 non-zero */
      if (mode == (if us > 0 { mpz_div_round_mode::GMP_DIV_CEIL }
                   else { mpz_div_round_mode::GMP_DIV_FLOOR })) /* us != 0 here. */
	{
	  /* Have to negate and sign extend. */
	  let mut i: mp_size_t;

	  gmp_assert_nocarry! ((mpn_neg (rp, (*u)._mp_d, un) == 0) as mp_limb_t);
	  i = un;
	  while i < rn - 1
	    { *rp.offset (i as isize) = GMP_LIMB_MAX; i += 1; }

	  *rp.offset ((rn-1) as isize) = mask;
	  us = -us;
	}
      else
	{
	  /* Just copy */
	  if r as *const __mpz_struct != u
	    { mpn_copyi (rp, (*u)._mp_d, un); }

	  rn = un;
	}
    }
  else
    {
      if r as *const __mpz_struct != u
	{ mpn_copyi (rp, (*u)._mp_d, rn - 1); }

      *rp.offset ((rn-1) as isize)
        = *(*u)._mp_d.offset ((rn-1) as isize) & mask;

      if (mode == (if us > 0 { mpz_div_round_mode::GMP_DIV_CEIL }
                   else { mpz_div_round_mode::GMP_DIV_FLOOR })) /* us != 0 here. */
	{
	  /* If r != 0, compute 2^{bit_count} - r. */
	  mpn_neg (rp, rp, rn);

	  *rp.offset ((rn-1) as isize) &= mask;

	  /* us is not used for anything else, so we can modify it
	     here to indicate flipped sign. */
	  us = -us;
	}
    }
  rn = mpn_normalized_size (rp, rn);
  (*r)._mp_size = (if us < 0 { -rn } else { rn }) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_q_2exp (r: *mut __mpz_struct, u: *const __mpz_struct,
                                          cnt: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_q_2exp (r, u, cnt, mpz_div_round_mode::GMP_DIV_CEIL);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_q_2exp (r: *mut __mpz_struct, u: *const __mpz_struct,
                                          cnt: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_q_2exp (r, u, cnt, mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_q_2exp (r: *mut __mpz_struct, u: *const __mpz_struct,
                                          cnt: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_q_2exp (r, u, cnt, mpz_div_round_mode::GMP_DIV_TRUNC);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_r_2exp (r: *mut __mpz_struct, u: *const __mpz_struct,
                                          cnt: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_r_2exp (r, u, cnt, mpz_div_round_mode::GMP_DIV_CEIL);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_r_2exp (r: *mut __mpz_struct, u: *const __mpz_struct,
                                          cnt: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_r_2exp (r, u, cnt, mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_r_2exp (r: *mut __mpz_struct, u: *const __mpz_struct,
                                          cnt: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_div_r_2exp (r, u, cnt, mpz_div_round_mode::GMP_DIV_TRUNC);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_divexact (q: *mut __mpz_struct, n: *const __mpz_struct,
                                       d: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  gmp_assert_nocarry! (mpz_div_qr (q, std::ptr::null_mut (), n, d,
                                   mpz_div_round_mode::GMP_DIV_TRUNC) as mp_limb_t);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_divisible_p (n: *const __mpz_struct, d: *const __mpz_struct)
  -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return (mpz_div_qr (std::ptr::null_mut (), std::ptr::null_mut (), n, d,
                      mpz_div_round_mode::GMP_DIV_TRUNC) == 0) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_congruent_p (a: *const __mpz_struct, b: *const __mpz_struct,
                                          m: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut t: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                     _mp_d: std::ptr::null_mut () }; 1];
  let mut res: c_int;

  /* a == b (mod 0) iff a == b */
  if (mpz_sgn (m) == 0)
    { return (mpz_cmp (a, b) == 0) as c_int; }

  mpz_init (t.as_mut_ptr ());
  mpz_sub (t.as_mut_ptr (), a, b);
  res = mpz_divisible_p (t.as_ptr (), m);
  mpz_clear (t.as_mut_ptr ());

  return res;

  }
}

unsafe extern "C-unwind" fn mpz_div_qr_ui (q: *mut __mpz_struct, r: *mut __mpz_struct,
                                    n: *const __mpz_struct, d: c_ulong,
                                    mode: mpz_div_round_mode) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut ret: c_ulong;
  let mut rr: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                      _mp_d: std::ptr::null_mut () }; 1];
  let mut dd: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                      _mp_d: std::ptr::null_mut () }; 1];

  mpz_init (rr.as_mut_ptr ());
  mpz_init_set_ui (dd.as_mut_ptr (), d);
  mpz_div_qr (q, rr.as_mut_ptr (), n, dd.as_ptr (), mode);
  mpz_clear (dd.as_mut_ptr ());
  ret = mpz_get_ui (rr.as_ptr ());

  if !r.is_null ()
    { mpz_swap (r, rr.as_mut_ptr ()); }
  mpz_clear (rr.as_mut_ptr ());

  return ret;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_qr_ui (q: *mut __mpz_struct, r: *mut __mpz_struct,
                                         n: *const __mpz_struct, d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (q, r, n, d, mpz_div_round_mode::GMP_DIV_CEIL);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_qr_ui (q: *mut __mpz_struct, r: *mut __mpz_struct,
                                         n: *const __mpz_struct, d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (q, r, n, d, mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_qr_ui (q: *mut __mpz_struct, r: *mut __mpz_struct,
                                         n: *const __mpz_struct, d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (q, r, n, d, mpz_div_round_mode::GMP_DIV_TRUNC);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_q_ui (q: *mut __mpz_struct, n: *const __mpz_struct,
                                        d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (q, std::ptr::null_mut (), n, d,
                        mpz_div_round_mode::GMP_DIV_CEIL);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_q_ui (q: *mut __mpz_struct, n: *const __mpz_struct,
                                        d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (q, std::ptr::null_mut (), n, d,
                        mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_q_ui (q: *mut __mpz_struct, n: *const __mpz_struct,
                                        d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (q, std::ptr::null_mut (), n, d,
                        mpz_div_round_mode::GMP_DIV_TRUNC);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_r_ui (r: *mut __mpz_struct, n: *const __mpz_struct,
                                        d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (std::ptr::null_mut (), r, n, d,
                        mpz_div_round_mode::GMP_DIV_CEIL);

  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_r_ui (r: *mut __mpz_struct, n: *const __mpz_struct,
                                        d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (std::ptr::null_mut (), r, n, d,
                        mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_r_ui (r: *mut __mpz_struct, n: *const __mpz_struct,
                                        d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (std::ptr::null_mut (), r, n, d,
                        mpz_div_round_mode::GMP_DIV_TRUNC);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_ui (n: *const __mpz_struct, d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (std::ptr::null_mut (), std::ptr::null_mut (), n, d,
                        mpz_div_round_mode::GMP_DIV_CEIL);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_ui (n: *const __mpz_struct, d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (std::ptr::null_mut (), std::ptr::null_mut (), n, d,
                        mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_ui (n: *const __mpz_struct, d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (std::ptr::null_mut (), std::ptr::null_mut (), n, d,
                        mpz_div_round_mode::GMP_DIV_TRUNC);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mod_ui (r: *mut __mpz_struct, n: *const __mpz_struct,
                                     d: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_div_qr_ui (std::ptr::null_mut (), r, n, d,
                        mpz_div_round_mode::GMP_DIV_FLOOR);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_divexact_ui (q: *mut __mpz_struct, n: *const __mpz_struct,
                                          d: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  gmp_assert_nocarry! (mpz_div_qr_ui (q, std::ptr::null_mut (), n, d,
                                      mpz_div_round_mode::GMP_DIV_TRUNC));

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_divisible_ui_p (n: *const __mpz_struct, d: c_ulong)
  -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return (mpz_div_qr_ui (std::ptr::null_mut (), std::ptr::null_mut (), n, d,
                         mpz_div_round_mode::GMP_DIV_TRUNC) == 0) as c_int;

  }
}

/* GCD */
unsafe extern "C-unwind" fn mpn_gcd_11 (mut u: mp_limb_t, mut v: mp_limb_t) -> mp_limb_t
{
  let mut shift: c_uint = 0;

  assert! ( (u | v) > 0);

  if (u == 0)
    { return v; }
  else if (v == 0)
    { return u; }

  gmp_ctz! (shift, u | v);

  u >>= shift;
  v >>= shift;

  if ( (u & 1) == 0)
    { MP_LIMB_T_SWAP! (u, v); }

  while ( (v & 1) == 0)
    { v >>= 1; }

  while (u != v)
    {
      if (u > v)
	{
	  u = u.wrapping_sub (v);
	  loop
	    { u >>= 1; if !( (u & 1) == 0) { break; } }
	}
      else
	{
	  v = v.wrapping_sub (u);
	  loop
	    { v >>= 1; if !( (v & 1) == 0) { break; } }
	}
    }
  return u << shift;
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_gcd (rp: mp_ptr, up: mp_ptr, mut un: mp_size_t,
                                  vp: mp_ptr, mut vn: mp_size_t) -> mp_size_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  assert! (un >= vn);
  assert! (vn > 0);
  assert! (!GMP_MPN_OVERLAP_P! (up, un, vp, vn));
  assert! (*vp.offset ((vn-1) as isize) > 0);
  assert! ((*up | *vp) & 1 != 0);

  let mut up = up;
  let mut vp = vp;

  if (un > vn)
    { mpn_div_qr (std::ptr::null_mut (), up, un, vp, vn); }

  un = mpn_normalized_size (up, vn);
  if (un == 0)
    {
      mpn_copyi (rp, vp, vn);
      return vn;
    }

  if !(*vp & 1 != 0)
    { MPN_PTR_SWAP! (up, un, vp, vn); }

  while (un > 1 || vn > 1)
    {
      let mut shift: c_uint = 0;
      assert! (*vp & 1 != 0);

      while *up == 0
	{
	  up = up.add (1);
	  un -= 1;
	}
      gmp_ctz! (shift, *up);
      if (shift > 0)
	{
	  gmp_assert_nocarry! (mpn_rshift (up, up, un, shift));
	  un -= (*up.offset ((un-1) as isize) == 0) as mp_size_t;
	}

      if (un < vn)
	{ MPN_PTR_SWAP! (up, un, vp, vn); }
      else if (un == vn)
	{
	  let c: c_int = mpn_cmp (up, vp, un);
	  if (c == 0)
	    {
	      mpn_copyi (rp, up, un);
	      return un;
	    }
	  else if (c < 0)
	    { MP_PTR_SWAP! (up, vp); }
	}

      gmp_assert_nocarry! (mpn_sub (up, up, un, vp, vn));
      un = mpn_normalized_size (up, un);
    }
  *rp = mpn_gcd_11 (*up, *vp);
  return 1;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_gcd_ui (g: *mut __mpz_struct, u: *const __mpz_struct,
                                     mut v: c_ulong) -> c_ulong
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut t: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                     _mp_d: std::ptr::null_mut () }; 1];
  mpz_init_set_ui (t.as_mut_ptr (), v);
  mpz_gcd (t.as_mut_ptr (), u, t.as_ptr ());
  if (v > 0)
    { v = mpz_get_ui (t.as_ptr ()); }

  if !g.is_null ()
    { mpz_swap (t.as_mut_ptr (), g); }

  mpz_clear (t.as_mut_ptr ());

  return v;

  }
}

unsafe extern "C-unwind" fn mpz_make_odd (r: *mut __mpz_struct) -> mp_bitcnt_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let shift: mp_bitcnt_t;

  assert! ((*r)._mp_size > 0);
  /* Count trailing zeros, equivalent to mpn_scan1, because we know that there is a 1 */
  shift = mpn_scan1 ((*r)._mp_d, 0);
  mpz_tdiv_q_2exp (r, r, shift);

  return shift;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_gcd (g: *mut __mpz_struct, u: *const __mpz_struct,
                                  v: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut tu: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                      _mp_d: std::ptr::null_mut () }; 1];
  let mut tv: mpz_t = [__mpz_struct { _mp_alloc: 0, _mp_size: 0,
                                      _mp_d: std::ptr::null_mut () }; 1];
  let mut uz: mp_bitcnt_t;
  let mut vz: mp_bitcnt_t;
  let mut gz: mp_bitcnt_t;

  if ((*u)._mp_size == 0)
    {
      mpz_abs (g, v);
      return;
    }
  if ((*v)._mp_size == 0)
    {
      mpz_abs (g, u);
      return;
    }

  mpz_init (tu.as_mut_ptr ());
  mpz_init (tv.as_mut_ptr ());

  mpz_abs (tu.as_mut_ptr (), u);
  uz = mpz_make_odd (tu.as_mut_ptr ());
  mpz_abs (tv.as_mut_ptr (), v);
  vz = mpz_make_odd (tv.as_mut_ptr ());
  gz = GMP_MIN! (uz, vz);

  if (tu[0]._mp_size < tv[0]._mp_size)
    { mpz_swap (tu.as_mut_ptr (), tv.as_mut_ptr ()); }

  tu[0]._mp_size = mpn_gcd (tu[0]._mp_d, tu[0]._mp_d, tu[0]._mp_size as mp_size_t,
                            tv[0]._mp_d, tv[0]._mp_size as mp_size_t) as c_int;
  mpz_clear (tv.as_mut_ptr ());

  mpz_mul_2exp (g, tu.as_ptr (), gz);
  mpz_clear (tu.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_gcdext (g: *mut __mpz_struct, s: *mut __mpz_struct,
                                     t: *mut __mpz_struct, u: *const __mpz_struct,
                                     v: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut tu: mpz_t = zero_mpz ();
  let mut tv: mpz_t = zero_mpz ();
  let mut s0: mpz_t = zero_mpz ();
  let mut s1: mpz_t = zero_mpz ();
  let mut t0: mpz_t = zero_mpz ();
  let mut t1: mpz_t = zero_mpz ();
  let mut uz: mp_bitcnt_t;
  let mut vz: mp_bitcnt_t;
  let mut gz: mp_bitcnt_t;
  let mut power: mp_bitcnt_t;
  let mut cmp: c_int;
  let mut u = u;
  let mut v = v;
  let mut s = s;
  let mut t = t;

  if ((*u)._mp_size == 0)
    {
      /* g = 0 u + sgn(v) v */
      let sign: c_long = mpz_sgn (v) as c_long;
      mpz_abs (g, v);
      if !s.is_null ()
	{ (*s)._mp_size = 0; }
      if !t.is_null ()
	{ mpz_set_si (t, sign); }
      return;
    }

  if ((*v)._mp_size == 0)
    {
      /* g = sgn(u) u + 0 v */
      let sign: c_long = mpz_sgn (u) as c_long;
      mpz_abs (g, u);
      if !s.is_null ()
	{ mpz_set_si (s, sign); }
      if !t.is_null ()
	{ (*t)._mp_size = 0; }
      return;
    }

  mpz_init (tu.as_mut_ptr ());
  mpz_init (tv.as_mut_ptr ());
  mpz_init (s0.as_mut_ptr ());
  mpz_init (s1.as_mut_ptr ());
  mpz_init (t0.as_mut_ptr ());
  mpz_init (t1.as_mut_ptr ());

  mpz_abs (tu.as_mut_ptr (), u);
  uz = mpz_make_odd (tu.as_mut_ptr ());
  mpz_abs (tv.as_mut_ptr (), v);
  vz = mpz_make_odd (tv.as_mut_ptr ());
  gz = GMP_MIN! (uz, vz);

  uz -= gz;
  vz -= gz;

  /* Cofactors corresponding to odd gcd. gz handled later. */
  if (tu[0]._mp_size < tv[0]._mp_size)
    {
      mpz_swap (tu.as_mut_ptr (), tv.as_mut_ptr ());
      MPZ_SRCPTR_SWAP! (u, v);
      MPZ_PTR_SWAP! (s, t);
      MP_BITCNT_T_SWAP! (uz, vz);
    }

  /* Maintain
   *
   * u = t0 tu + t1 tv
   * v = s0 tu + s1 tv
   *
   * where u and v denote the inputs with common factors of two
   * eliminated, and det (s0, t0; s1, t1) = 2^p. Then
   *
   * 2^p tu =  s1 u - t1 v
   * 2^p tv = -s0 u + t0 v
   */

  /* After initial division, tu = q tv + tu', we have
   *
   * u = 2^uz (tu' + q tv)
   * v = 2^vz tv
   *
   * or
   *
   * t0 = 2^uz, t1 = 2^uz q
   * s0 = 0,    s1 = 2^vz
   */

  mpz_tdiv_qr (t1.as_mut_ptr (), tu.as_mut_ptr (), tu.as_ptr (), tv.as_ptr ());
  mpz_mul_2exp (t1.as_mut_ptr (), t1.as_ptr (), uz);

  mpz_setbit (s1.as_mut_ptr (), vz);
  power = uz + vz;

  if (tu[0]._mp_size > 0)
    {
      let mut shift: mp_bitcnt_t;
      shift = mpz_make_odd (tu.as_mut_ptr ());
      mpz_setbit (t0.as_mut_ptr (), uz + shift);
      power += shift;

      loop
	{
	  let mut c: c_int = 0;
	  c = mpz_cmp (tu.as_ptr (), tv.as_ptr ());
	  if (c == 0)
	    { break; }

	  if (c < 0)
	    {
	      /* tv = tv' + tu
	       *
	       * u = t0 tu + t1 (tv' + tu) = (t0 + t1) tu + t1 tv'
	       * v = s0 tu + s1 (tv' + tu) = (s0 + s1) tu + s1 tv' */

	      mpz_sub (tv.as_mut_ptr (), tv.as_ptr (), tu.as_ptr ());
	      mpz_add (t0.as_mut_ptr (), t0.as_ptr (), t1.as_ptr ());
	      mpz_add (s0.as_mut_ptr (), s0.as_ptr (), s1.as_ptr ());

	      shift = mpz_make_odd (tv.as_mut_ptr ());
	      mpz_mul_2exp (t1.as_mut_ptr (), t1.as_ptr (), shift);
	      mpz_mul_2exp (s1.as_mut_ptr (), s1.as_ptr (), shift);
	    }
	  else
	    {
	      mpz_sub (tu.as_mut_ptr (), tu.as_ptr (), tv.as_ptr ());
	      mpz_add (t1.as_mut_ptr (), t0.as_ptr (), t1.as_ptr ());
	      mpz_add (s1.as_mut_ptr (), s0.as_ptr (), s1.as_ptr ());

	      shift = mpz_make_odd (tu.as_mut_ptr ());
	      mpz_mul_2exp (t0.as_mut_ptr (), t0.as_ptr (), shift);
	      mpz_mul_2exp (s0.as_mut_ptr (), s0.as_ptr (), shift);
	    }
	  power += shift;
	}
    }
  else
    { mpz_setbit (t0.as_mut_ptr (), uz); }

  mpz_clear (tu.as_mut_ptr ());
  /* Now tv = odd part of gcd, and -s0 and t0 are corresponding
     cofactors. */

  mpz_mul_2exp (tv.as_mut_ptr (), tv.as_ptr (), gz);
  mpz_neg (s0.as_mut_ptr (), s0.as_ptr ());

  /* 2^p g = s0 u + t0 v. Eliminate one factor of two at a time. To
     adjust cofactors, we need u / g and v / g */

  mpz_divexact (s1.as_mut_ptr (), v, tv.as_ptr ());
  mpz_abs (s1.as_mut_ptr (), s1.as_ptr ());
  mpz_divexact (t1.as_mut_ptr (), u, tv.as_ptr ());
  mpz_abs (t1.as_mut_ptr (), t1.as_ptr ());

  while { let old = power; power -= 1; old > 0 }
    {
      /* s0 u + t0 v = (s0 - v/g) u - (t0 + u/g) v */
      if (mpz_odd_p! (s0.as_ptr ()) != 0 || mpz_odd_p! (t0.as_ptr ()) != 0)
	{
	  mpz_sub (s0.as_mut_ptr (), s0.as_ptr (), s1.as_ptr ());
	  mpz_add (t0.as_mut_ptr (), t0.as_ptr (), t1.as_ptr ());
	}
      assert! (mpz_even_p! (t0.as_ptr ()) != 0 && mpz_even_p! (s0.as_ptr ()) != 0);
      mpz_tdiv_q_2exp (s0.as_mut_ptr (), s0.as_ptr (), 1);
      mpz_tdiv_q_2exp (t0.as_mut_ptr (), t0.as_ptr (), 1);
    }

  /* Choose small cofactors (they should generally satify

       |s| < |u| / 2g and |t| < |v| / 2g,

     with some documented exceptions). Always choose the smallest s,
     if there are two choices for s with same absolute value, choose
     the one with smallest corresponding t (this asymmetric condition
     is needed to prefer s = 0, |t| = 1 when g = |a| = |b|). */
  mpz_add (s1.as_mut_ptr (), s0.as_ptr (), s1.as_ptr ());
  cmp = mpz_cmpabs (s0.as_ptr (), s1.as_ptr ());
  if (cmp >= 0)
    {
      mpz_sub (t1.as_mut_ptr (), t0.as_ptr (), t1.as_ptr ());
      if (cmp > 0 || mpz_cmpabs (t0.as_ptr (), t1.as_ptr ()) > 0)
      {
      mpz_swap (s0.as_mut_ptr (), s1.as_mut_ptr ());
      mpz_swap (t0.as_mut_ptr (), t1.as_mut_ptr ());
      }
    }
  mpz_clear (s1.as_mut_ptr ());
  mpz_clear (t1.as_mut_ptr ());

  if ((*u)._mp_size < 0)
    { mpz_neg (s0.as_mut_ptr (), s0.as_ptr ()); }
  if ((*v)._mp_size < 0)
    { mpz_neg (t0.as_mut_ptr (), t0.as_ptr ()); }

  mpz_swap (g, tv.as_mut_ptr ());
  if !s.is_null ()
    { mpz_swap (s, s0.as_mut_ptr ()); }
  if !t.is_null ()
    { mpz_swap (t, t0.as_mut_ptr ()); }

  mpz_clear (tv.as_mut_ptr ());
  mpz_clear (s0.as_mut_ptr ());
  mpz_clear (t0.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_lcm (r: *mut __mpz_struct, u: *const __mpz_struct,
                                  v: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut g: mpz_t = zero_mpz ();

  if ((*u)._mp_size == 0 || (*v)._mp_size == 0)
    {
      (*r)._mp_size = 0;
      return;
    }

  mpz_init (g.as_mut_ptr ());

  mpz_gcd (g.as_mut_ptr (), u, v);
  mpz_divexact (g.as_mut_ptr (), u, g.as_ptr ());
  mpz_mul (r, g.as_ptr (), v);

  mpz_clear (g.as_mut_ptr ());
  mpz_abs (r, r);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_lcm_ui (r: *mut __mpz_struct, u: *const __mpz_struct,
                                     mut v: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if (v == 0 || (*u)._mp_size == 0)
    {
      (*r)._mp_size = 0;
      return;
    }

  v /= mpz_gcd_ui (std::ptr::null_mut (), u, v);
  mpz_mul_ui (r, u, v);

  mpz_abs (r, r);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_invert (r: *mut __mpz_struct, u: *const __mpz_struct,
                                     m: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut g: mpz_t = zero_mpz ();
  let mut tr: mpz_t = zero_mpz ();
  let invertible: c_int;

  if ((*u)._mp_size == 0 || mpz_cmpabs_ui (m, 1) <= 0)
    { return 0; }

  mpz_init (g.as_mut_ptr ());
  mpz_init (tr.as_mut_ptr ());

  mpz_gcdext (g.as_mut_ptr (), tr.as_mut_ptr (), std::ptr::null_mut (), u, m);
  invertible = (mpz_cmp_ui (g.as_ptr (), 1) == 0) as c_int;
  mpz_clear (g.as_mut_ptr ());

  if (invertible != 0)
    {
      if (tr[0]._mp_size < 0)
	{
	  if ((*m)._mp_size >= 0)
	    { mpz_add (tr.as_mut_ptr (), tr.as_ptr (), m); }
	  else
	    { mpz_sub (tr.as_mut_ptr (), tr.as_ptr (), m); }
	}
      mpz_swap (r, tr.as_mut_ptr ());
    }

  mpz_clear (tr.as_mut_ptr ());
  return invertible;

  }
}

/* Higher level operations (sqrt, pow and root) */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_pow_ui (r: *mut __mpz_struct, b: *const __mpz_struct,
                                     e: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut bit: c_ulong;
  let mut tr: mpz_t = zero_mpz ();
  mpz_init_set_ui (tr.as_mut_ptr (), 1);

  bit = GMP_ULONG_HIGHBIT;
  loop
    {
      mpz_mul (tr.as_mut_ptr (), tr.as_ptr (), tr.as_ptr ());
      if (e & bit != 0)
	{ mpz_mul (tr.as_mut_ptr (), tr.as_ptr (), b); }
      bit >>= 1;
      if ! (bit > 0) { break; }
    }

  mpz_swap (r, tr.as_mut_ptr ());
  mpz_clear (tr.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_ui_pow_ui (r: *mut __mpz_struct, blimb: c_ulong, e: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut b: mpz_t = zero_mpz ();

  mpz_init_set_ui (b.as_mut_ptr (), blimb);
  mpz_pow_ui (r, b.as_ptr (), e);
  mpz_clear (b.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_powm (r: *mut __mpz_struct, b: *const __mpz_struct,
                                   e: *const __mpz_struct, m: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut tr: mpz_t = zero_mpz ();
  let mut base: mpz_t = zero_mpz ();
  let mut en: mp_size_t;
  let mn: mp_size_t;
  let mut mp: mp_srcptr;
  let mut minv: gmp_div_inverse = gmp_div_inverse { shift: 0, d1: 0, d0: 0, di: 0 };
  let shift: c_uint;
  let mut tp: mp_ptr = std::ptr::null_mut ();

  en = GMP_ABS! ((*e)._mp_size) as mp_size_t;
  mn = GMP_ABS! ((*m)._mp_size) as mp_size_t;
  if (mn == 0)
    { gmp_die (c"mpz_powm: Zero modulo.".as_ptr ()); }

  if (en == 0)
    {
      mpz_set_ui (r, mpz_cmpabs_ui (m, 1) as c_ulong);
      return;
    }

  mp = (*m)._mp_d;
  mpn_div_qr_invert (&mut minv, mp, mn);
  shift = minv.shift;

  if (shift > 0)
    {
      /* To avoid shifts, we do all our reductions, except the final
	 one, using a *normalized* m. */
      minv.shift = 0;

      tp = gmp_alloc_limbs (mn);
      gmp_assert_nocarry! (mpn_lshift (tp, mp, mn, shift));
      mp = tp;
    }

  mpz_init (base.as_mut_ptr ());

  if ((*e)._mp_size < 0)
    {
      if (mpz_invert (base.as_mut_ptr (), b, m) == 0)
	{ gmp_die (c"mpz_powm: Negative exponent and non-invertible base.".as_ptr ()); }
    }
  else
    {
      let mut bn: mp_size_t;
      mpz_abs (base.as_mut_ptr (), b);

      bn = base[0]._mp_size as mp_size_t;
      if (bn >= mn)
	{
	  mpn_div_qr_preinv (std::ptr::null_mut (), base[0]._mp_d,
	                     base[0]._mp_size as mp_size_t, mp, mn, &minv);
	  bn = mn;
	}

      /* We have reduced the absolute value. Now take care of the
	 sign. Note that we get zero represented non-canonically as
	 m. */
      if ((*b)._mp_size < 0)
	{
	  let bp: mp_ptr = MPZ_REALLOC! (base.as_mut_ptr (), mn);
	  gmp_assert_nocarry! (mpn_sub (bp, mp, mn, bp, bn));
	  bn = mn;
	}
      base[0]._mp_size = mpn_normalized_size (base[0]._mp_d, bn) as c_int;
    }
  mpz_init_set_ui (tr.as_mut_ptr (), 1);

  while { en -= 1; en >= 0 }
    {
      let w: mp_limb_t = *(*e)._mp_d.offset (en as isize);
      let mut bit: mp_limb_t;

      bit = GMP_LIMB_HIGHBIT;
      loop
	{
	  mpz_mul (tr.as_mut_ptr (), tr.as_ptr (), tr.as_ptr ());
	  if (w & bit != 0)
	    { mpz_mul (tr.as_mut_ptr (), tr.as_ptr (), base.as_ptr ()); }
	  if (tr[0]._mp_size as mp_size_t > mn)
	    {
	      mpn_div_qr_preinv (std::ptr::null_mut (), tr[0]._mp_d,
	                         tr[0]._mp_size as mp_size_t, mp, mn, &minv);
	      tr[0]._mp_size = mpn_normalized_size (tr[0]._mp_d, mn) as c_int;
	    }
	  bit >>= 1;
	  if ! (bit > 0) { break; }
	}
    }
  mpz_clear (base.as_mut_ptr ());

  /* Final reduction */
  if (tr[0]._mp_size as mp_size_t >= mn)
    {
      minv.shift = shift;
      mpn_div_qr_preinv (std::ptr::null_mut (), tr[0]._mp_d,
                         tr[0]._mp_size as mp_size_t, mp, mn, &minv);
      tr[0]._mp_size = mpn_normalized_size (tr[0]._mp_d, mn) as c_int;
    }
  if !tp.is_null ()
    { gmp_free_limbs (tp, mn); }

  mpz_swap (r, tr.as_mut_ptr ());
  mpz_clear (tr.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_powm_ui (r: *mut __mpz_struct, b: *const __mpz_struct,
                                      elimb: c_ulong, m: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut e: mpz_t = zero_mpz ();

  mpz_init_set_ui (e.as_mut_ptr (), elimb);
  mpz_powm (r, b, e.as_ptr (), m);
  mpz_clear (e.as_mut_ptr ());

  }
}

/* x=trunc(y^(1/z)), r=y-x^z */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_rootrem (x: *mut __mpz_struct, r: *mut __mpz_struct,
                                      y: *const __mpz_struct, z: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut sgn: c_int;
  let mut bc: mp_bitcnt_t;
  let mut t: mpz_t = zero_mpz ();
  let mut u: mpz_t = zero_mpz ();

  sgn = ((*y)._mp_size < 0) as c_int;
  if ((!z & sgn as c_ulong) != 0)
    { gmp_die (c"mpz_rootrem: Negative argument, with even root.".as_ptr ()); }
  if (z == 0)
    { gmp_die (c"mpz_rootrem: Zeroth root.".as_ptr ()); }

  if (mpz_cmpabs_ui (y, 1) <= 0) {
    if !x.is_null ()
      { mpz_set (x, y); }
    if !r.is_null ()
      { (*r)._mp_size = 0; }
    return;
  }

  mpz_init (u.as_mut_ptr ());
  bc = (mpz_sizeinbase (y, 2) as mp_bitcnt_t - 1) / z + 1;
  mpz_init_setbit (t.as_mut_ptr (), bc);

  if (z == 2) /* simplify sqrt loop: z-1 == 1 */
    { loop {
      mpz_swap (u.as_mut_ptr (), t.as_mut_ptr ());		/* u = x */
      mpz_tdiv_q (t.as_mut_ptr (), y, u.as_ptr ());		/* t = y/x */
      mpz_add (t.as_mut_ptr (), t.as_ptr (), u.as_ptr ());		/* t = y/x + x */
      mpz_tdiv_q_2exp (t.as_mut_ptr (), t.as_ptr (), 1);	/* x'= (y/x + x)/2 */
      if ! (mpz_cmpabs (t.as_ptr (), u.as_ptr ()) < 0) { break; }	/* |x'| < |x| */
    } }
  else /* z != 2 */ {
    let mut v: mpz_t = zero_mpz ();

    mpz_init (v.as_mut_ptr ());
    if (sgn != 0)
      { mpz_neg (t.as_mut_ptr (), t.as_ptr ()); }

    loop {
      mpz_swap (u.as_mut_ptr (), t.as_mut_ptr ());		/* u = x */
      mpz_pow_ui (t.as_mut_ptr (), u.as_ptr (), z - 1);		/* t = x^(z-1) */
      mpz_tdiv_q (t.as_mut_ptr (), y, t.as_ptr ());		/* t = y/x^(z-1) */
      mpz_mul_ui (v.as_mut_ptr (), u.as_ptr (), z - 1);		/* v = x*(z-1) */
      mpz_add (t.as_mut_ptr (), t.as_ptr (), v.as_ptr ());	/* t = y/x^(z-1) + x*(z-1) */
      mpz_tdiv_q_ui (t.as_mut_ptr (), t.as_ptr (), z);		/* x'=(y/x^(z-1) + x*(z-1))/z */
      if ! (mpz_cmpabs (t.as_ptr (), u.as_ptr ()) < 0) { break; }	/* |x'| < |x| */
    }

    mpz_clear (v.as_mut_ptr ());
  }

  if !r.is_null () {
    mpz_pow_ui (t.as_mut_ptr (), u.as_ptr (), z);
    mpz_sub (r, y, t.as_ptr ());
  }
  if !x.is_null ()
    { mpz_swap (x, u.as_mut_ptr ()); }
  mpz_clear (u.as_mut_ptr ());
  mpz_clear (t.as_mut_ptr ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_root (x: *mut __mpz_struct, y: *const __mpz_struct,
                                   z: c_ulong) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut res: c_int;
  let mut r: mpz_t = zero_mpz ();

  mpz_init (r.as_mut_ptr ());
  mpz_rootrem (x, r.as_mut_ptr (), y, z);
  res = (r[0]._mp_size == 0) as c_int;
  mpz_clear (r.as_mut_ptr ());

  return res;

  }
}

/* Compute s = floor(sqrt(u)) and r = u - s^2. Allows r == NULL */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sqrtrem (s: *mut __mpz_struct, r: *mut __mpz_struct,
                                      u: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_rootrem (s, r, u, 2);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sqrt (s: *mut __mpz_struct, u: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_rootrem (s, std::ptr::null_mut (), u, 2);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_perfect_square_root (r: *mut __mpz_struct,
                                                  u: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if ((*u)._mp_size <= 0)
    {
      let ret: c_int = ((*u)._mp_size == 0) as c_int;
      if r != std::ptr::null_mut ()
	{ (*r)._mp_size = 0; }
      return ret;
    }
  else
    { return mpz_root (r, u, 2); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_perfect_square_p (u: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return mpz_perfect_square_root (std::ptr::null_mut (), u);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_perfect_square_p (p: mp_srcptr, n: mp_size_t) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut t: mpz_t = zero_mpz ();

  assert! (n > 0);
  assert! (*p.offset ((n-1) as isize) != 0);
  return mpz_root (std::ptr::null_mut (), mpz_roinit_normal_n (t.as_mut_ptr (), p, n), 2);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_sqrtrem (sp: mp_ptr, rp: mp_ptr, p: mp_srcptr,
                                      n: mp_size_t) -> mp_size_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut s: mpz_t = zero_mpz ();
  let mut r: mpz_t = zero_mpz ();
  let mut u: mpz_t = zero_mpz ();
  let mut res: mp_size_t;

  assert! (n > 0);
  assert! (*p.offset ((n-1) as isize) != 0);

  mpz_init (r.as_mut_ptr ());
  mpz_init (s.as_mut_ptr ());
  mpz_rootrem (s.as_mut_ptr (), r.as_mut_ptr (),
               mpz_roinit_normal_n (u.as_mut_ptr (), p, n), 2);

  assert! (s[0]._mp_size as mp_size_t == (n+1)/2);
  mpn_copyd (sp, s[0]._mp_d, s[0]._mp_size as mp_size_t);
  mpz_clear (s.as_mut_ptr ());
  res = r[0]._mp_size as mp_size_t;
  if !rp.is_null ()
    { mpn_copyd (rp, r[0]._mp_d, res); }
  mpz_clear (r.as_mut_ptr ());
  return res;

  }
}

/* Combinatorics */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mfac_uiui (x: *mut __mpz_struct, mut n: c_ulong,
                                        m: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_set_ui (x, n + (n == 0) as c_ulong);
  if (m + 1 < 2) { return; }
  while (n > m + 1)
    { n -= m; mpz_mul_ui (x, x, n); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_2fac_ui (x: *mut __mpz_struct, n: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_mfac_uiui (x, n, 2);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fac_ui (x: *mut __mpz_struct, n: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_mfac_uiui (x, n, 1);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_bin_uiui (r: *mut __mpz_struct, mut n: c_ulong,
                                       mut k: c_ulong)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut t: mpz_t = zero_mpz ();

  mpz_set_ui (r, (k <= n) as c_ulong);

  if (k > (n >> 1))
    { k = if k <= n { n - k } else { 0 }; }

  mpz_init (t.as_mut_ptr ());
  mpz_fac_ui (t.as_mut_ptr (), k);

  while k > 0
    { mpz_mul_ui (r, r, n); n -= 1; k -= 1; }

  mpz_divexact (r, r, t.as_ptr ());
  mpz_clear (t.as_mut_ptr ());

  }
}

/* Computes Kronecker (a/b) with odd b, a!=0 and GCD(a,b) = 1 */
/* Adapted from JACOBI_BASE_METHOD==4 in mpn/generic/jacbase.c */
unsafe extern "C-unwind" fn gmp_jacobi_coprime (mut a: mp_limb_t, mut b: mp_limb_t) -> c_int
{
  let mut c: c_uint = 0;
  let mut bit: c_int = 0;

  assert! (b & 1 != 0);
  assert! (a != 0);
  /* assert (mpn_gcd_11 (a, b) == 1); */

  /* Below, we represent a and b shifted right so that the least
     significant one bit is implicit. */
  b >>= 1;

  gmp_ctz! (c, a);
  a >>= 1;

  loop
    {
      a >>= c;
      /* (2/b) = -1 if b = 3 or 5 mod 8 */
      bit ^= (c & (b ^ (b >> 1)) as c_uint) as c_int;
      if (a < b)
	{
	  if (a == 0)
	    { return if bit & 1 != 0 { -1 } else { 1 }; }
	  bit ^= (a & b) as c_int;
	  a = b - a;
	  b -= a;
	}
      else
	{
	  a = a.wrapping_sub (b);
	  assert! (a != 0);
	}

      gmp_ctz! (c, a);
      c += 1;
    }
}

unsafe extern "C-unwind" fn gmp_lucas_step_k_2k (V: *mut __mpz_struct, Qk: *mut __mpz_struct,
                                          n: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_mod (Qk, Qk, n);
  /* V_{2k} <- V_k ^ 2 - 2Q^k */
  mpz_mul (V, V, V);
  mpz_submul_ui (V, Qk, 2);
  mpz_tdiv_r (V, V, n);
  /* Q^{2k} = (Q^k)^2 */
  mpz_mul (Qk, Qk, Qk);

  }
}

/* Computes V_k, Q^k (mod n) for the Lucas' sequence */
/* with P=1, Q=Q; k = (n>>b0)|1. */
/* Requires an odd n > 4; b0 > 0; -2*Q must not overflow a long */
/* Returns (U_k == 0) and sets V=V_k and Qk=Q^k. */
unsafe extern "C-unwind" fn gmp_lucas_mod (V: *mut __mpz_struct, Qk: *mut __mpz_struct,
                                    Q: c_long, b0: mp_bitcnt_t,
                                    n: *const __mpz_struct) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut bs: mp_bitcnt_t;
  let mut U: mpz_t = zero_mpz ();
  let mut res: c_int;

  assert! (b0 > 0);
  assert! (Q <= - (LONG_MIN / 2));
  assert! (Q >= - (LONG_MAX / 2));
  assert! (mpz_cmp_ui (n, 4) > 0);
  assert! (mpz_odd_p! (n) != 0);

  mpz_init_set_ui (U.as_mut_ptr (), 1); /* U1 = 1 */
  mpz_set_ui (V, 1); /* V1 = 1 */
  mpz_set_si (Qk, Q);

  bs = mpz_sizeinbase (n, 2) as mp_bitcnt_t - 1;
  while { bs -= 1; bs >= b0 }
    {
      /* U_{2k} <- U_k * V_k */
      mpz_mul (U.as_mut_ptr (), U.as_ptr (), V);
      /* V_{2k} <- V_k ^ 2 - 2Q^k */
      /* Q^{2k} = (Q^k)^2 */
      gmp_lucas_step_k_2k (V, Qk, n);

      /* A step k->k+1 is performed if the bit in $n$ is 1	*/
      /* mpz_tstbit(n,bs) or the bit is 0 in $n$ but	*/
      /* should be 1 in $n+1$ (bs == b0)			*/
      if (b0 == bs || mpz_tstbit (n, bs) != 0)
	{
	  /* Q^{k+1} <- Q^k * Q */
	  mpz_mul_si (Qk, Qk, Q);
	  /* U_{k+1} <- (U_k + V_k) / 2 */
	  mpz_swap (U.as_mut_ptr (), V); /* Keep in V the old value of U_k */
	  mpz_add (U.as_mut_ptr (), U.as_ptr (), V);
	  /* We have to compute U/2, so we need an even value, */
	  /* equivalent (mod n) */
	  if (mpz_odd_p! (U.as_ptr ()) != 0)
	    { mpz_add (U.as_mut_ptr (), U.as_ptr (), n); }
	  mpz_tdiv_q_2exp (U.as_mut_ptr (), U.as_ptr (), 1);
	  /* V_{k+1} <-(D*U_k + V_k) / 2 =
			U_{k+1} + (D-1)/2*U_k = U_{k+1} - 2Q*U_k */
	  mpz_mul_si (V, V, -2*Q);
	  mpz_add (V, U.as_ptr (), V);
	  mpz_tdiv_r (V, V, n);
	}
      mpz_tdiv_r (U.as_mut_ptr (), U.as_ptr (), n);
    }

  res = (U[0]._mp_size == 0) as c_int;
  mpz_clear (U.as_mut_ptr ());
  return res;

  }
}

/* Performs strong Lucas' test on x, with parameters suggested */
/* for the BPSW test. Qk is only passed to recycle a variable. */
/* Requires GCD (x,6) = 1.*/
unsafe extern "C-unwind" fn gmp_stronglucas (x: *const __mpz_struct, Qk: *mut __mpz_struct)
  -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut b0: mp_bitcnt_t;
  let mut V: mpz_t = zero_mpz ();
  let mut n: mpz_t = zero_mpz ();
  let mut maxD: mp_limb_t;
  let mut D: mp_limb_t; /* The absolute value is stored. */
  let mut Q: c_long;
  let mut tl: mp_limb_t;

  /* Test on the absolute value. */
  mpz_roinit_normal_n (n.as_mut_ptr (), (*x)._mp_d, GMP_ABS! ((*x)._mp_size) as mp_size_t);

  assert! (mpz_odd_p! (n.as_ptr ()) != 0);
  /* assert (mpz_gcd_ui (NULL, n, 6) == 1); */
  if (mpz_root (Qk, n.as_ptr (), 2) != 0)
    { return 0; } /* A square is composite. */

  /* Check Ds up to square root (in case, n is prime)
     or avoid overflows */
  maxD = if (*Qk)._mp_size == 1 { *(*Qk)._mp_d - 1 } else { GMP_LIMB_MAX };

  D = 3;
  /* Search a D such that (D/n) = -1 in the sequence 5,-7,9,-11,.. */
  /* For those Ds we have (D/n) = (n/|D|) */
  loop
    {
      if (D >= maxD)
	{ return 1 + ((D != GMP_LIMB_MAX) as c_int); } /* (1 + ! ~ D) */
      D += 2;
      tl = mpz_tdiv_ui (n.as_ptr (), D as c_ulong);
      if (tl == 0)
	{ return 0; }
      if ! (gmp_jacobi_coprime (tl, D) == 1) { break; }
    }

  mpz_init (V.as_mut_ptr ());

  /* n-(D/n) = n+1 = d*2^{b0}, with d = (n>>b0) | 1 */
  b0 = mpn_common_scan (! *n[0]._mp_d, 0, n[0]._mp_d, n[0]._mp_size as mp_size_t,
                        GMP_LIMB_MAX);
  /* b0 = mpz_scan0 (n, 0); */

  /* D= P^2 - 4Q; P = 1; Q = (1-D)/4 */
  Q = if (D & 2) != 0 { ((D >> 2) as c_long) + 1 } else { -((D >> 2) as c_long) };

  if gmp_lucas_mod (V.as_mut_ptr (), Qk, Q, b0, n.as_ptr ()) == 0	/* If Ud != 0 */
    { while V[0]._mp_size != 0 && { b0 -= 1; b0 != 0 }	/* while Vk != 0 */
      /* V <- V ^ 2 - 2Q^k */
      /* Q^{2k} = (Q^k)^2 */
      { gmp_lucas_step_k_2k (V.as_mut_ptr (), Qk, n.as_ptr ()); } }

  mpz_clear (V.as_mut_ptr ());
  return (b0 != 0) as c_int;

  }
}

unsafe extern "C-unwind" fn gmp_millerrabin (n: *const __mpz_struct, nm1: *const __mpz_struct,
                                      y: *mut __mpz_struct, q: *const __mpz_struct,
                                      mut k: mp_bitcnt_t) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  assert! (k > 0);

  /* Caller must initialize y to the base. */
  mpz_powm (y, y, q, n);

  if (mpz_cmp_ui (y, 1) == 0 || mpz_cmp (y, nm1) == 0)
    { return 1; }

  while { k -= 1; k > 0 }
    {
      mpz_powm_ui (y, y, 2, n);
      if (mpz_cmp (y, nm1) == 0)
	{ return 1; }
    }
  return 0;

  }
}

/* This product is 0xc0cfd797, and fits in 32 bits. */
/* #define GMP_PRIME_PRODUCT (3UL*5UL*7UL*11UL*13UL*17UL*19UL*23UL*29UL)  */
pub const GMP_PRIME_PRODUCT: c_ulong
  = (3*5*7*11*13*17*19*23*29);

/* Bit (p+1)/2 is set, for each odd prime <= 61 */
/* #define GMP_PRIME_MASK 0xc96996dcUL  */
pub const GMP_PRIME_MASK: c_ulong = 0xc96996dc;

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_probab_prime_p (n: *const __mpz_struct, mut reps: c_int)
  -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut nm1: mpz_t = zero_mpz ();
  let mut q: mpz_t = zero_mpz ();
  let mut y: mpz_t = zero_mpz ();
  let mut k: mp_bitcnt_t;
  let mut is_prime: c_int;
  let mut j: c_int;

  /* Note that we use the absolute value of n only, for compatibility
     with the real GMP. */
  if (mpz_even_p! (n) != 0)
    { return if mpz_cmpabs_ui (n, 2) == 0 { 2 } else { 0 }; }

  /* Above test excludes n == 0 */
  assert! ((*n)._mp_size != 0);

  if (mpz_cmpabs_ui (n, 64) < 0)
    { return ((GMP_PRIME_MASK >> (*(*n)._mp_d >> 1)) & 2) as c_int; }

  if (mpz_gcd_ui (std::ptr::null_mut (), n, GMP_PRIME_PRODUCT) != 1)
    { return 0; }

  /* All prime factors are >= 31. */
  if (mpz_cmpabs_ui (n, 31*31) < 0)
    { return 2; }

  mpz_init (nm1.as_mut_ptr ());
  mpz_init (q.as_mut_ptr ());

  /* Find q and k, where q is odd and n = 1 + 2**k * q.  */
  mpz_abs (nm1.as_mut_ptr (), n);
  *nm1[0]._mp_d = (*nm1[0]._mp_d).wrapping_sub (1);
  /* Count trailing zeros, equivalent to mpn_scan1, because we know that there is a 1 */
  k = mpn_scan1 (nm1[0]._mp_d, 0);
  mpz_tdiv_q_2exp (q.as_mut_ptr (), nm1.as_ptr (), k);

  /* BPSW test */
  mpz_init_set_ui (y.as_mut_ptr (), 2);
  is_prime = (gmp_millerrabin (n, nm1.as_ptr (), y.as_mut_ptr (), q.as_ptr (), k) != 0
              && gmp_stronglucas (n, y.as_mut_ptr ()) != 0) as c_int;
  reps -= 24; /* skip the first 24 repetitions */

  /* Use Miller-Rabin, with a deterministic sequence of bases, a[j] =
     j^2 + j + 41 using Euler's polynomial. We potentially stop early,
     if a[j] >= n - 1. Since n >= 31*31, this can happen only if reps >
     30 (a[30] == 971 > 31*31 == 961). */

  j = 0;
  while (is_prime & ((j < reps) as c_int)) != 0
    {
      mpz_set_ui (y.as_mut_ptr (), (j*j+j+41) as c_ulong);
      if (mpz_cmp (y.as_ptr (), nm1.as_ptr ()) >= 0)
	{
	  /* Don't try any further bases. This "early" break does not affect
	     the result for any reasonable reps value (<=5000 was tested) */
	  assert! (j >= 30);
	  break;
	}
      is_prime = gmp_millerrabin (n, nm1.as_ptr (), y.as_mut_ptr (), q.as_ptr (), k);
      j += 1;
    }
  mpz_clear (nm1.as_mut_ptr ());
  mpz_clear (q.as_mut_ptr ());
  mpz_clear (y.as_mut_ptr ());

  return is_prime;

  }
}

/* Logical operations and bit manipulation. */

/* Numbers are treated as if represented in two's complement (and
   infinitely sign extended). For a negative values we get the two's
   complement from -x = ~x + 1, where ~ is bitwise complement.
   Negation transforms

     xxxx10...0

   into

     yyyy10...0

   where yyyy is the bitwise complement of xxxx. So least significant
   bits, up to and including the first one bit, are unchanged, and
   the more significant bits are all complemented.

   To change a bit from zero to one in a negative number, subtract the
   corresponding power of two from the absolute value. This can never
   underflow. To change a bit from one to zero, add the corresponding
   power of two, and this might overflow. E.g., if x = -001111, the
   two's complement is 110001. Clearing the least significant bit, we
   get two's complement 110000, and -010000. */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tstbit (d: *const __mpz_struct, bit_index: mp_bitcnt_t)
  -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut limb_index: mp_size_t;
  let mut shift: c_uint;
  let mut ds: mp_size_t;
  let mut dn: mp_size_t;
  let mut w: mp_limb_t;
  let mut bit: c_int;

  ds = (*d)._mp_size as mp_size_t;
  dn = GMP_ABS! (ds);
  limb_index = (bit_index / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;
  if (limb_index >= dn)
    { return (ds < 0) as c_int; }

  shift = (bit_index % GMP_LIMB_BITS as mp_bitcnt_t) as c_uint;
  w = *(*d)._mp_d.offset (limb_index as isize);
  bit = ((w >> shift) & 1) as c_int;

  if (ds < 0)
    {
      /* d < 0. Check if any of the bits below is set: If so, our bit
	 must be complemented. */
      if (shift > 0 && (w << (GMP_LIMB_BITS - shift as usize)) > 0)
	{ return bit ^ 1; }
      while { limb_index -= 1; limb_index >= 0 }
	{ if (*(*d)._mp_d.offset (limb_index as isize) > 0)
	    { return bit ^ 1; } }
    }
  return bit;

  }
}

unsafe extern "C-unwind" fn mpz_abs_add_bit (d: *mut __mpz_struct, bit_index: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut dn: mp_size_t;
  let mut limb_index: mp_size_t;
  let bit: mp_limb_t;
  let mut dp: mp_ptr;

  dn = GMP_ABS! ((*d)._mp_size) as mp_size_t;

  limb_index = (bit_index / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;
  bit = (1 as mp_limb_t) << (bit_index % GMP_LIMB_BITS as mp_bitcnt_t);

  if (limb_index >= dn)
    {
      let mut i: mp_size_t;
      /* The bit should be set outside of the end of the number.
	 We have to increase the size of the number. */
      dp = MPZ_REALLOC! (d, limb_index + 1);

      *dp.offset (limb_index as isize) = bit;
      i = dn;
      while i < limb_index
	{ *dp.offset (i as isize) = 0; i += 1; }
      dn = limb_index + 1;
    }
  else
    {
      let mut cy: mp_limb_t;

      dp = (*d)._mp_d;

      cy = mpn_add_1 (dp.offset (limb_index as isize), dp.offset (limb_index as isize),
	              dn - limb_index, bit);
      if (cy > 0)
	{
	  dp = MPZ_REALLOC! (d, dn + 1);
	  *dp.offset (dn as isize) = cy;
	  dn += 1;
	}
    }
  (*d)._mp_size = (if (*d)._mp_size < 0 { - dn } else { dn }) as c_int;

  }
}

unsafe extern "C-unwind" fn mpz_abs_sub_bit (d: *mut __mpz_struct, bit_index: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut dn: mp_size_t;
  let limb_index: mp_size_t;
  let dp: mp_ptr;
  let bit: mp_limb_t;

  dn = GMP_ABS! ((*d)._mp_size) as mp_size_t;
  dp = (*d)._mp_d;

  limb_index = (bit_index / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;
  bit = (1 as mp_limb_t) << (bit_index % GMP_LIMB_BITS as mp_bitcnt_t);

  assert! (limb_index < dn);

  gmp_assert_nocarry! (mpn_sub_1 (dp.offset (limb_index as isize),
				  dp.offset (limb_index as isize),
				  dn - limb_index, bit));
  dn = mpn_normalized_size (dp, dn);
  (*d)._mp_size = (if (*d)._mp_size < 0 { - dn } else { dn }) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_setbit (d: *mut __mpz_struct, bit_index: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if (mpz_tstbit (d, bit_index) == 0)
    {
      if ((*d)._mp_size >= 0)
	{ mpz_abs_add_bit (d, bit_index); }
      else
	{ mpz_abs_sub_bit (d, bit_index); }
    }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_clrbit (d: *mut __mpz_struct, bit_index: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if (mpz_tstbit (d, bit_index) != 0)
    {
      if ((*d)._mp_size >= 0)
	{ mpz_abs_sub_bit (d, bit_index); }
      else
	{ mpz_abs_add_bit (d, bit_index); }
    }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_combit (d: *mut __mpz_struct, bit_index: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  if (mpz_tstbit (d, bit_index) ^ (((*d)._mp_size < 0) as c_int)) != 0
    { mpz_abs_sub_bit (d, bit_index); }
  else
    { mpz_abs_add_bit (d, bit_index); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init_setbit (r: *mut __mpz_struct, b: mp_bitcnt_t)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_init (r);
  /* mpz_setbit (r, b); */
  mpz_abs_add_bit (r, b);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_com (r: *mut __mpz_struct, u: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_add_ui (r, u, 1);
  mpz_neg (r, r);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_and (r: *mut __mpz_struct, u: *const __mpz_struct,
                                 v: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut un: mp_size_t;
  let mut vn: mp_size_t;
  let mut rn: mp_size_t;
  let mut i: mp_size_t;
  let up: mp_ptr;
  let vp: mp_ptr;
  let mut rp: mp_ptr;

  let mut ux: mp_limb_t;
  let mut vx: mp_limb_t;
  let rx: mp_limb_t;
  let mut uc: mp_limb_t;
  let mut vc: mp_limb_t;
  let mut rc: mp_limb_t;
  let mut ul: mp_limb_t;
  let mut vl: mp_limb_t;
  let mut rl: mp_limb_t;
  let mut u = u;
  let mut v = v;

  un = GMP_ABS! ((*u)._mp_size) as mp_size_t;
  vn = GMP_ABS! ((*v)._mp_size) as mp_size_t;
  if (un < vn)
    {
      MPZ_SRCPTR_SWAP! (u, v);
      MP_SIZE_T_SWAP! (un, vn);
    }
  if (vn == 0)
    {
      (*r)._mp_size = 0;
      return;
    }

  uc = ((*u)._mp_size < 0) as mp_limb_t;
  vc = ((*v)._mp_size < 0) as mp_limb_t;
  rc = uc & vc;

  ux = uc.wrapping_neg ();
  vx = vc.wrapping_neg ();
  rx = rc.wrapping_neg ();

  /* If the smaller input is positive, higher limbs don't matter. */
  rn = if vx != 0 { un } else { vn };

  rp = MPZ_REALLOC! (r, rn + rc as mp_size_t);

  up = (*u)._mp_d;
  vp = (*v)._mp_d;

  i = 0;
  loop
    {
      ul = (*up.offset (i as isize) ^ ux).wrapping_add (uc);
      uc = (ul < uc) as mp_limb_t;

      vl = (*vp.offset (i as isize) ^ vx).wrapping_add (vc);
      vc = (vl < vc) as mp_limb_t;

      rl = ( (ul & vl) ^ rx).wrapping_add (rc);
      rc = (rl < rc) as mp_limb_t;
      *rp.offset (i as isize) = rl;
      i += 1;
      if ! (i < vn) { break; }
    }
  assert! (vc == 0);

  while i < rn
    {
      ul = (*up.offset (i as isize) ^ ux).wrapping_add (uc);
      uc = (ul < uc) as mp_limb_t;

      rl = ( (ul & vx) ^ rx).wrapping_add (rc);
      rc = (rl < rc) as mp_limb_t;
      *rp.offset (i as isize) = rl;
      i += 1;
    }
  if rc != 0
    { *rp.offset (rn as isize) = rc; rn += 1; }
  else
    { rn = mpn_normalized_size (rp, rn); }

  (*r)._mp_size = (if rx != 0 { -rn } else { rn }) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_ior (r: *mut __mpz_struct, u: *const __mpz_struct,
                                  v: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut un: mp_size_t;
  let mut vn: mp_size_t;
  let mut rn: mp_size_t;
  let mut i: mp_size_t;
  let up: mp_ptr;
  let vp: mp_ptr;
  let mut rp: mp_ptr;

  let mut ux: mp_limb_t;
  let mut vx: mp_limb_t;
  let rx: mp_limb_t;
  let mut uc: mp_limb_t;
  let mut vc: mp_limb_t;
  let mut rc: mp_limb_t;
  let mut ul: mp_limb_t;
  let mut vl: mp_limb_t;
  let mut rl: mp_limb_t;
  let mut u = u;
  let mut v = v;

  un = GMP_ABS! ((*u)._mp_size) as mp_size_t;
  vn = GMP_ABS! ((*v)._mp_size) as mp_size_t;
  if (un < vn)
    {
      MPZ_SRCPTR_SWAP! (u, v);
      MP_SIZE_T_SWAP! (un, vn);
    }
  if (vn == 0)
    {
      mpz_set (r, u);
      return;
    }

  uc = ((*u)._mp_size < 0) as mp_limb_t;
  vc = ((*v)._mp_size < 0) as mp_limb_t;
  rc = uc | vc;

  ux = uc.wrapping_neg ();
  vx = vc.wrapping_neg ();
  rx = rc.wrapping_neg ();

  /* If the smaller input is negative, by sign extension higher limbs
     don't matter. */
  rn = if vx != 0 { vn } else { un };

  rp = MPZ_REALLOC! (r, rn + rc as mp_size_t);

  up = (*u)._mp_d;
  vp = (*v)._mp_d;

  i = 0;
  loop
    {
      ul = (*up.offset (i as isize) ^ ux).wrapping_add (uc);
      uc = (ul < uc) as mp_limb_t;

      vl = (*vp.offset (i as isize) ^ vx).wrapping_add (vc);
      vc = (vl < vc) as mp_limb_t;

      rl = ( (ul | vl) ^ rx).wrapping_add (rc);
      rc = (rl < rc) as mp_limb_t;
      *rp.offset (i as isize) = rl;
      i += 1;
      if ! (i < vn) { break; }
    }
  assert! (vc == 0);

  while i < rn
    {
      ul = (*up.offset (i as isize) ^ ux).wrapping_add (uc);
      uc = (ul < uc) as mp_limb_t;

      rl = ( (ul | vx) ^ rx).wrapping_add (rc);
      rc = (rl < rc) as mp_limb_t;
      *rp.offset (i as isize) = rl;
      i += 1;
    }
  if rc != 0
    { *rp.offset (rn as isize) = rc; rn += 1; }
  else
    { rn = mpn_normalized_size (rp, rn); }

  (*r)._mp_size = (if rx != 0 { -rn } else { rn }) as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_xor (r: *mut __mpz_struct, u: *const __mpz_struct,
                                  v: *const __mpz_struct)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut un: mp_size_t;
  let mut vn: mp_size_t;
  let mut i: mp_size_t;
  let up: mp_ptr;
  let vp: mp_ptr;
  let mut rp: mp_ptr;

  let mut ux: mp_limb_t;
  let mut vx: mp_limb_t;
  let rx: mp_limb_t;
  let mut uc: mp_limb_t;
  let mut vc: mp_limb_t;
  let mut rc: mp_limb_t;
  let mut ul: mp_limb_t;
  let mut vl: mp_limb_t;
  let mut rl: mp_limb_t;
  let mut u = u;
  let mut v = v;

  un = GMP_ABS! ((*u)._mp_size) as mp_size_t;
  vn = GMP_ABS! ((*v)._mp_size) as mp_size_t;
  if (un < vn)
    {
      MPZ_SRCPTR_SWAP! (u, v);
      MP_SIZE_T_SWAP! (un, vn);
    }
  if (vn == 0)
    {
      mpz_set (r, u);
      return;
    }

  uc = ((*u)._mp_size < 0) as mp_limb_t;
  vc = ((*v)._mp_size < 0) as mp_limb_t;
  rc = uc ^ vc;

  ux = uc.wrapping_neg ();
  vx = vc.wrapping_neg ();
  rx = rc.wrapping_neg ();

  rp = MPZ_REALLOC! (r, un + rc as mp_size_t);

  up = (*u)._mp_d;
  vp = (*v)._mp_d;

  i = 0;
  loop
    {
      ul = (*up.offset (i as isize) ^ ux).wrapping_add (uc);
      uc = (ul < uc) as mp_limb_t;

      vl = (*vp.offset (i as isize) ^ vx).wrapping_add (vc);
      vc = (vl < vc) as mp_limb_t;

      rl = (ul ^ vl ^ rx).wrapping_add (rc);
      rc = (rl < rc) as mp_limb_t;
      *rp.offset (i as isize) = rl;
      i += 1;
      if ! (i < vn) { break; }
    }
  assert! (vc == 0);

  while i < un
    {
      ul = (*up.offset (i as isize) ^ ux).wrapping_add (uc);
      uc = (ul < uc) as mp_limb_t;

      rl = (ul ^ ux).wrapping_add (rc);
      rc = (rl < rc) as mp_limb_t;
      *rp.offset (i as isize) = rl;
      i += 1;
    }
  if rc != 0
    { *rp.offset (un as isize) = rc; un += 1; }
  else
    { un = mpn_normalized_size (rp, un); }

  (*r)._mp_size = (if rx != 0 { -un } else { un }) as c_int;

  }
}

unsafe extern "C-unwind" fn gmp_popcount_limb (mut x: mp_limb_t) -> c_uint
{
  let mut c: c_uint;

  /* Do 16 bits at a time, to avoid limb-sized constants. */
  let LOCAL_SHIFT_BITS: c_int = 16;
  c = 0;
  while x > 0
    {
      let mut w: c_uint = x.wrapping_sub ((x >> 1) & 0x5555) as c_uint;
      w = ((w >> 2) & 0x3333).wrapping_add (w & 0x3333);
      w =  (w >> 4).wrapping_add (w);
      w = ((w >> 8) & 0x000f).wrapping_add (w & 0x000f);
      c += w;
      if (GMP_LIMB_BITS > LOCAL_SHIFT_BITS as usize)
	{ x >>= LOCAL_SHIFT_BITS; }
      else
	{ x = 0; }
    }
  return c;
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpn_popcount (p: mp_srcptr, n: mp_size_t) -> mp_bitcnt_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut i: mp_size_t;
  let mut c: mp_bitcnt_t;

  c = 0;
  i = 0;
  while i < n
    { c += gmp_popcount_limb (*p.offset (i as isize)) as mp_bitcnt_t; i += 1; }

  return c;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_popcount (u: *const __mpz_struct) -> mp_bitcnt_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let un: mp_size_t;

  un = (*u)._mp_size as mp_size_t;

  if (un < 0)
    { return !(0 as mp_bitcnt_t); }

  return mpn_popcount ((*u)._mp_d, un);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_hamdist (u: *const __mpz_struct, v: *const __mpz_struct)
  -> mp_bitcnt_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut un: mp_size_t;
  let mut vn: mp_size_t;
  let mut i: mp_size_t;
  let mut uc: mp_limb_t;
  let mut vc: mp_limb_t;
  let mut ul: mp_limb_t;
  let mut vl: mp_limb_t;
  let comp: mp_limb_t;
  let mut up: mp_srcptr;
  let mut vp: mp_srcptr;
  let mut c: mp_bitcnt_t;

  un = (*u)._mp_size as mp_size_t;
  vn = (*v)._mp_size as mp_size_t;

  if ( (un ^ vn) < 0)
    { return !(0 as mp_bitcnt_t); }

  uc = ((un < 0) as mp_limb_t); vc = uc; comp = uc.wrapping_neg ();
  if (uc != 0)
    {
      assert! (vn < 0);
      un = -un;
      vn = -vn;
    }

  up = (*u)._mp_d;
  vp = (*v)._mp_d;

  if (un < vn)
    { MPN_SRCPTR_SWAP! (up, un, vp, vn); }

  i = 0;
  c = 0;
  while i < vn
    {
      ul = (*up.offset (i as isize) ^ comp).wrapping_add (uc);
      uc = (ul < uc) as mp_limb_t;

      vl = (*vp.offset (i as isize) ^ comp).wrapping_add (vc);
      vc = (vl < vc) as mp_limb_t;

      c += gmp_popcount_limb (ul ^ vl) as mp_bitcnt_t;
      i += 1;
    }
  assert! (vc == 0);

  while i < un
    {
      ul = (*up.offset (i as isize) ^ comp).wrapping_add (uc);
      uc = (ul < uc) as mp_limb_t;

      c += gmp_popcount_limb (ul ^ comp) as mp_bitcnt_t;
      i += 1;
    }

  return c;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_scan1 (u: *const __mpz_struct, starting_bit: mp_bitcnt_t)
  -> mp_bitcnt_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let up: mp_ptr;
  let mut us: mp_size_t;
  let un: mp_size_t;
  let mut i: mp_size_t;
  let mut limb: mp_limb_t;
  let mut ux: mp_limb_t;

  us = (*u)._mp_size as mp_size_t;
  un = GMP_ABS! (us);
  i = (starting_bit / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;

  /* Past the end there's no 1 bits for u>=0, or an immediate 1 bit
     for u<0. Notice this test picks up any u==0 too. */
  if (i >= un)
    { return if us >= 0 { !(0 as mp_bitcnt_t) } else { starting_bit }; }

  up = (*u)._mp_d;
  ux = 0;
  limb = *up.offset (i as isize);

  if (starting_bit != 0)
    {
      if (us < 0)
	{
	  ux = mpn_zero_p (up, i) as mp_limb_t;
	  limb = (! limb).wrapping_add (ux);
	  ux = ((limb >= ux) as mp_limb_t).wrapping_neg ();
	}

      /* Mask to 0 all bits before starting_bit, thus ignoring them. */
      limb &= GMP_LIMB_MAX << (starting_bit % GMP_LIMB_BITS as mp_bitcnt_t);
    }

  return mpn_common_scan (limb, i, up, un, ux);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_scan0 (u: *const __mpz_struct, starting_bit: mp_bitcnt_t)
  -> mp_bitcnt_t
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let up: mp_ptr;
  let us: mp_size_t;
  let un: mp_size_t;
  let i: mp_size_t;
  let mut limb: mp_limb_t;
  let ux: mp_limb_t;

  us = (*u)._mp_size as mp_size_t;
  ux = ((us >= 0) as mp_limb_t).wrapping_neg ();
  un = GMP_ABS! (us);
  i = (starting_bit / GMP_LIMB_BITS as mp_bitcnt_t) as mp_size_t;

  /* When past end, there's an immediate 0 bit for u>=0, or no 0 bits for
     u<0.  Notice this test picks up all cases of u==0 too. */
  if (i >= un)
    { return if ux != 0 { starting_bit } else { !(0 as mp_bitcnt_t) }; }

  up = (*u)._mp_d;
  limb = *up.offset (i as isize) ^ ux;

  if (ux == 0)
    { limb = limb.wrapping_sub (mpn_zero_p (up, i) as mp_limb_t); } /* limb = ~(~limb + zero_p) */

  /* Mask all bits before starting_bit, thus ignoring them. */
  limb &= GMP_LIMB_MAX << (starting_bit % GMP_LIMB_BITS as mp_bitcnt_t);

  return mpn_common_scan (limb, i, up, un, ux);

  }
}

/* MPZ base conversion. */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sizeinbase (u: *const __mpz_struct, base: c_int) -> usize
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut un: mp_size_t;
  let mut tn: mp_size_t;
  let mut up: mp_srcptr;
  let mut tp: mp_ptr;
  let mut bits: mp_bitcnt_t;
  let mut bi: gmp_div_inverse = gmp_div_inverse { shift: 0, d1: 0, d0: 0, di: 0 };
  let mut ndigits: usize;

  assert! (base >= 2);
  assert! (base <= 62);

  un = GMP_ABS! ((*u)._mp_size) as mp_size_t;
  if (un == 0)
    { return 1; }

  up = (*u)._mp_d;

  bits = ((un - 1) as usize * GMP_LIMB_BITS) as mp_bitcnt_t
         + mpn_limb_size_in_base_2 (*up.offset ((un-1) as isize));
  match base
    {
    2 => { return bits as usize; }
    4 => { return ((bits + 1) / 2) as usize; }
    8 => { return ((bits + 2) / 3) as usize; }
    16 => { return ((bits + 3) / 4) as usize; }
    32 => { return ((bits + 4) / 5) as usize; }
      /* FIXME: Do something more clever for the common case of base
	 10. */
    _ => {}
    }

  tp = gmp_alloc_limbs (un);
  mpn_copyi (tp, up, un);
  mpn_div_qr_1_invert (&mut bi, base as mp_limb_t);

  tn = un;
  ndigits = 0;
  loop
    {
      ndigits += 1;
      mpn_div_qr_1_preinv (tp, tp, tn, &bi);
      tn -= (*tp.offset ((tn-1) as isize) == 0) as mp_size_t;
      if ! (tn > 0) { break; }
    }

  gmp_free_limbs (tp, un);
  return ndigits;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_get_str (sp: *mut c_char, mut base: c_int,
                                      u: *const __mpz_struct) -> *mut c_char
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut bits: c_uint;
  let mut digits: *const c_char;
  let mut un: mp_size_t;
  let mut i: usize;
  let mut sn: usize;
  let mut osn: usize = 0;
  let mut sp = sp;

  digits = c"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_ptr ();
  if (base > 1)
    {
      if (base <= 36)
	{ digits = c"0123456789abcdefghijklmnopqrstuvwxyz".as_ptr (); }
      else if (base > 62)
	{ return std::ptr::null_mut (); }
    }
  else if (base >= -1)
    { base = 10; }
  else
    {
      base = -base;
      if (base > 36)
	{ return std::ptr::null_mut (); }
    }

  sn = 1 + mpz_sizeinbase (u, base);
  if sp.is_null ()
    {
      osn = 1 + sn;
      sp = gmp_alloc! (osn) as *mut c_char;
    }
  else
    { osn = 0; }
  un = GMP_ABS! ((*u)._mp_size) as mp_size_t;

  'ret: {
  if (un == 0)
    {
      *sp = '0' as c_char;
      sn = 1;
      break 'ret;
    }

  i = 0;

  if ((*u)._mp_size < 0)
    { *sp.offset (i as isize) = '-' as c_char; i += 1; }

  bits = mpn_base_power_of_two_p (base as c_uint);

  if bits != 0
    /* Not modified in this case. */
    { sn = i + mpn_get_str_bits (sp.offset (i as isize) as *mut u8, bits,
                                 (*u)._mp_d, un); }
  else
    {
      let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
      let mut tp: mp_ptr;

      mpn_get_base_info (&mut info, base as mp_limb_t);
      tp = gmp_alloc_limbs (un);
      mpn_copyi (tp, (*u)._mp_d, un);

      sn = i + mpn_get_str_other (sp.offset (i as isize) as *mut u8, base, &info, tp, un);
      gmp_free_limbs (tp, un);
    }

  while i < sn
    {
      *sp.offset (i as isize)
        = *digits.offset (*sp.offset (i as isize) as u8 as isize);
      i += 1;
    }
  }

  *sp.offset (sn as isize) = '\0' as c_char;
  if (osn != 0 && osn != sn + 1)
    { sp = gmp_realloc! (sp as *mut c_void, osn, sn + 1) as *mut c_char; }
  return sp;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set_str (r: *mut __mpz_struct, sp: *const c_char,
                                      mut base: c_int) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut bits: c_uint;
  let mut value_of_a: c_uint;
  let mut rn: mp_size_t;
  let mut alloc: mp_size_t;
  let mut rp: mp_ptr;
  let mut dn: usize;
  let sn: usize;
  let mut sign: c_int;
  let mut dp: *mut u8;
  let mut sp = sp;

  assert! (base == 0 || (base >= 2 && base <= 62));

  while isspace ( *sp as u8 as c_int) != 0
    { sp = sp.offset (1); }

  sign = (*sp == '-' as c_char) as c_int;
  sp = sp.offset (sign as isize);

  if (base == 0)
    {
      if (*sp == '0' as c_char)
	{
	  if (*sp.offset (1) == 'x' as c_char || *sp.offset (1) == 'X' as c_char)
	    {
	      base = 16;
	      sp = sp.offset (2);
	    }
	  else if (*sp.offset (1) == 'b' as c_char || *sp.offset (1) == 'B' as c_char)
	    {
	      base = 2;
	      sp = sp.offset (2);
	    }
	  else
	    { base = 8; }
	}
      else
	{ base = 10; }
    }

  if (*sp == 0)
    {
      (*r)._mp_size = 0;
      return -1;
    }
  sn = strlen (sp);
  dp = gmp_alloc! (sn) as *mut u8;

  value_of_a = (if base > 36 { 36 } else { 10 }) as c_uint;
  dn = 0;
  while (*sp) != 0
    {
      let mut digit: c_uint;

      if (isspace (*sp as u8 as c_int) != 0)
	{ sp = sp.offset (1); continue; }
      else if (*sp as u8) >= '0' as u8 && (*sp as u8) <= '9' as u8
	{ digit = (*sp as u8 - '0' as u8) as c_uint; }
      else if (*sp as u8) >= 'a' as u8 && (*sp as u8) <= 'z' as u8
	{ digit = (*sp as u8 - 'a' as u8) as c_uint + value_of_a; }
      else if (*sp as u8) >= 'A' as u8 && (*sp as u8) <= 'Z' as u8
	{ digit = (*sp as u8 - 'A' as u8) as c_uint + 10; }
      else
	{ digit = base as c_uint; } /* fail */

      if (digit >= base as c_uint)
	{
	  gmp_free! (dp as *mut c_void, sn);
	  (*r)._mp_size = 0;
	  return -1;
	}

      *dp.offset (dn as isize) = digit as u8;
      dn += 1;
      sp = sp.offset (1);
    }

  if (dn == 0)
    {
      gmp_free! (dp as *mut c_void, sn);
      (*r)._mp_size = 0;
      return -1;
    }
  bits = mpn_base_power_of_two_p (base as c_uint);

  if (bits > 0)
    {
      alloc = ((dn * bits as usize + GMP_LIMB_BITS - 1) / GMP_LIMB_BITS) as mp_size_t;
      rp = MPZ_REALLOC! (r, alloc);
      rn = mpn_set_str_bits (rp, dp, dn, bits);
    }
  else
    {
      let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
      mpn_get_base_info (&mut info, base as mp_limb_t);
      alloc = ((dn + info.exp as usize - 1) / info.exp as usize) as mp_size_t;
      rp = MPZ_REALLOC! (r, alloc);
      rn = mpn_set_str_other (rp, dp, dn, base as mp_limb_t, &info);
      /* Normalization, needed for all-zero input. */
      assert! (rn > 0);
      rn -= (*rp.offset ((rn-1) as isize) == 0) as mp_size_t;
    }
  assert! (rn <= alloc);
  gmp_free! (dp as *mut c_void, sn);

  (*r)._mp_size = (if sign != 0 { - rn } else { rn }) as c_int;

  return 0;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init_set_str (r: *mut __mpz_struct, sp: *const c_char,
                                           base: c_int) -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  mpz_init (r);
  return mpz_set_str (r, sp, base);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_out_str (stream: *mut FILE, base: c_int,
                                      x: *const __mpz_struct) -> usize
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut str_: *mut c_char;
  let len: usize;
  let n: usize;

  str_ = mpz_get_str (std::ptr::null_mut (), base, x);
  if str_.is_null ()
    { return 0; }
  len = strlen (str_);
  n = fwrite (str_ as *const c_void, 1, len, stream);
  gmp_free! (str_ as *mut c_void, len + 1);
  return n;

  }
}

unsafe extern "C-unwind" fn gmp_detect_endian () -> c_int
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  static i: c_int = 2;
  let p: *const u8 = &i as *const c_int as *const u8;
  return 1 - *p as c_int;

  }
}

/* Import and export. Does not support nails. */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_import (r: *mut __mpz_struct, mut count: usize,
                                     order: c_int, size: usize, mut endian: c_int,
                                     nails: usize, src: *const c_void)
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut p: *const u8;
  let mut word_step: isize;
  let rp: mp_ptr;
  let mut rn: mp_size_t;

  /* The current (partial) limb. */
  let mut limb: mp_limb_t;
  /* The number of bytes already copied to this limb (starting from
     the low end). */
  let mut bytes: usize;
  /* The index where the limb should be stored, when completed. */
  let mut i: mp_size_t;

  if (nails != 0)
    { gmp_die (c"mpz_import: Nails not supported.".as_ptr ()); }

  assert! (order == 1 || order == -1);
  assert! (endian >= -1 && endian <= 1);

  if (count == 0)
    {
      (*r)._mp_size = 0;
      return;
    }
  if (endian == 0)
    { endian = gmp_detect_endian (); }

  p = src as *const u8;

  word_step = if order != endian { (2 * size) as isize } else { 0 };

  /* Process bytes from the least significant end, so point p at the
     least significant word. */
  if (order == 1)
    {
      p = p.offset ((size * (count - 1)) as isize);
      word_step = - word_step;
    }

  /* And at least significant byte of that word. */
  if (endian == 1)
    { p = p.offset ((size - 1) as isize); }

  rn = ((size * count + std::mem::size_of::<mp_limb_t> () - 1)
        / std::mem::size_of::<mp_limb_t> ()) as mp_size_t;
  rp = MPZ_REALLOC! (r, rn);

  limb = 0; bytes = 0; i = 0;
  while count > 0
    {
      let mut j: usize;
      j = 0;
      while j < size
	{
	  limb |= (*p as mp_limb_t) << (bytes as usize * CHAR_BIT as usize);
	  bytes += 1;
	  if (bytes == std::mem::size_of::<mp_limb_t> ())
	    {
	      *rp.offset (i as isize) = limb;
	      i += 1;
	      bytes = 0;
	      limb = 0;
	    }
	  j += 1;
	  p = p.offset (-(endian as isize));
	}
      count -= 1;
      p = p.offset (word_step);
    }
  assert! (i + (bytes > 0) as mp_size_t == rn);
  if (limb != 0)
    { *rp.offset (i as isize) = limb; i += 1; }
  else
    { i = mpn_normalized_size (rp, i); }

  (*r)._mp_size = i as c_int;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_export (mut r: *mut c_void, countp: *mut usize,
                                     order: c_int, size: usize, mut endian: c_int,
                                     nails: usize, u: *const __mpz_struct)
  -> *mut c_void
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  let mut count: usize;
  let mut un: mp_size_t;

  if (nails != 0)
    { gmp_die (c"mpz_export: Nails not supported.".as_ptr ()); }

  assert! (order == 1 || order == -1);
  assert! (endian >= -1 && endian <= 1);
  assert! (size > 0 || (*u)._mp_size == 0);

  un = (*u)._mp_size as mp_size_t;
  count = 0;
  if (un != 0)
    {
      let mut k: usize;
      let mut p: *mut u8;
      let mut word_step: isize;
      /* The current (partial) limb. */
      let mut limb: mp_limb_t;
      /* The number of bytes left to do in this limb. */
      let mut bytes: usize;
      /* The index where the limb was read. */
      let mut i: mp_size_t;

      un = GMP_ABS! (un);

      /* Count bytes in top limb. */
      limb = *(*u)._mp_d.offset ((un-1) as isize);
      assert! (limb != 0);

      k = (GMP_LIMB_BITS <= CHAR_BIT as usize) as usize;
      if (k == 0)
	{
	  loop {
	    let LOCAL_CHAR_BIT: c_int = CHAR_BIT;
	    k += 1; limb >>= LOCAL_CHAR_BIT;
	    if ! (limb != 0) { break; }
	  }
	}
      /* else limb = 0; */

      count = (k + (un as usize-1) * std::mem::size_of::<mp_limb_t> () + size - 1) / size;

      if r.is_null ()
	{ r = gmp_alloc! (count * size); }

      if (endian == 0)
	{ endian = gmp_detect_endian (); }

      p = r as *mut u8;

      word_step = if order != endian { (2 * size) as isize } else { 0 };

      /* Process bytes from the least significant end, so point p at the
	 least significant word. */
      if (order == 1)
	{
	  p = p.offset ((size * (count - 1)) as isize);
	  word_step = - word_step;
	}

      /* And at least significant byte of that word. */
      if (endian == 1)
	{ p = p.offset ((size - 1) as isize); }

      bytes = 0; i = 0; k = 0;
      while k < count
	{
	  let mut j: usize;
	  j = 0;
	  while j < size
	    {
	      if (std::mem::size_of::<mp_limb_t> () == 1)
		{
		  if (i < un)
		    { *p = *(*u)._mp_d.offset (i as isize) as u8; i += 1; }
		  else
		    { *p = 0; }
		}
	      else
		{
		  let LOCAL_CHAR_BIT: c_int = CHAR_BIT;
		  if (bytes == 0)
		    {
		      if (i < un)
			{ limb = *(*u)._mp_d.offset (i as isize); i += 1; }
		      bytes = std::mem::size_of::<mp_limb_t> ();
		    }
		  *p = limb as u8;
		  limb >>= LOCAL_CHAR_BIT;
		  bytes -= 1;
		}
	      j += 1;
	      p = p.offset (-(endian as isize));
	    }
	  k += 1;
	  p = p.offset (word_step);
	}
      assert! (i == un);
      assert! (k == count);
    }

  if !countp.is_null ()
    { *countp = count; }

  return r;

  }
}

/// C: `realloc` as gnulib's generated `<stdlib.h>` leaves it for this configuration:
/// `#define realloc rpl_realloc`, `realloc (ptr, size ? size : 1)` (`lib/stdlib.in.h`
/// 1461-1482; the note in `lib/include/stdlib.rs`, status 4.787).
#[cfg (feature = "replace_realloc_for_realloc_posix")]
unsafe extern "C" fn realloc (ptr: *mut c_void, size: usize) -> *mut c_void
{
  // SAFETY: GMP/mini-gmp limb-array arithmetic boundary; caller guarantees valid limb pointers and non-negative limb counts matching `__mpz_struct` invariants.
  unsafe {
  return realloc_system (ptr, if size == 0 { 1 } else { size });

  }
}
