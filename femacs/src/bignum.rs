// femacs: port of src/bignum.c @ fd7afced403.
// One-to-one port (PORTING.md).  The system mirrors come from lib/include/ (P-26/P-49/
// P-50); `mpz_t` values pass as `…as_mut_ptr ()`/`…as_ptr ()` (P-49), and `mpz[5]`'s
// definition here shadows `include/bignum.rs`'s `bignum_externs` one (P-21/P-39).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("include/lisp.rs");
include!("../lib/include/mini-gmp.rs");
include!("include/bignum.rs");
include!("../lib/include/math.rs");
include!("../lib/include/stdlib.rs");
include!("../lib/include/stdcountof.rs");

/* mpz global temporaries.  Making them global saves the trouble of
   properly using mpz_init and mpz_clear on temporaries even when
   storage is exhausted.  Admittedly this is not ideal.  An mpz value
   in a temporary is made permanent by mpz_swapping it with a bignum's
   value.  Although typically at most two temporaries are needed,
   rounddiv_q and rounding_driver both need four and time_arith needs
   five.  */

#[unsafe(no_mangle)]
pub static mut mpz: [mpz_t; 5] =
  [[__mpz_struct { _mp_alloc: 0, _mp_size: 0, _mp_d: std::ptr::null_mut () }; 1]; 5];

unsafe extern "C" fn xrealloc_for_gmp (ptr: *mut c_void, _ignore: usize, size: usize)
				      -> *mut c_void
{
  return xrealloc (ptr, size);
}

unsafe extern "C" fn xfree_for_gmp (ptr: *mut c_void, _ignore: usize)
{
  xfree (ptr);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_bignum ()
{
  eassert! (mp_bits_per_limb == GMP_NUMB_BITS);
  integer_width = 1 << 16;

  /* FIXME: The Info node `(gmp) Custom Allocation' states: "No error
     return is allowed from any of these functions, if they return
     then they must have performed the specified operation. [...]
     There's currently no defined way for the allocation functions to
     recover from an error such as out of memory, they must terminate
     program execution.  A 'longjmp' or throwing a C++ exception will
     have undefined results."  But xmalloc and xrealloc do call
     'longjmp'.  */
  mp_set_memory_functions (xmalloc, xrealloc_for_gmp, xfree_for_gmp);

  let mut i = 0;
  while i < countof! (mpz)
    {
      mpz_init (mpz[i].as_mut_ptr ());
      i += 1;
    }
}

/* Return the value of the Lisp bignum N, as a double.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bignum_to_double (n: Lisp_Object) -> c_double
{
  return mpz_get_d_rounded (xbignum_val (n));
}

/* Return D, converted to a Lisp integer.  Discard any fraction.
   Signal an error if D cannot be converted.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn double_to_integer (d: c_double) -> Lisp_Object
{
  if !isfinite! (d)
    { overflow_error (); }
  mpz_set_d (mpz[0].as_mut_ptr (), d);
  return make_integer_mpz ();
}

/* Return a Lisp integer equal to mpz[0], which has BITS bits and which
   must not be in fixnum range.  Set mpz[0] to a junk value.  */
unsafe fn make_bignum_bits (bits: usize) -> Lisp_Object
{
  /* The documentation says integer-width should be nonnegative, so
     comparing it to BITS works even though BITS is unsigned.  Treat
     integer-width as if it were at least twice the machine integer width,
     so that timefns.c can safely use bignums for double-precision
     timestamps.  */
  if (integer_width as usize) < bits && 2 * max (INTMAX_WIDTH as isize, UINTMAX_WIDTH as isize) < bits as isize
    { overflow_error (); }

  let b: *mut Lisp_Bignum = ALLOCATE_PLAIN_PSEUDOVECTOR! (Lisp_Bignum, pvec_type::PVEC_BIGNUM);
  mpz_init ((*b).value.as_mut_ptr ());
  mpz_swap ((*b).value.as_mut_ptr (), mpz[0].as_mut_ptr ());
  return make_lisp_ptr (b as *mut c_void, Lisp_Type::Lisp_Vectorlike);
}

/* Return a Lisp integer equal to mpz[0], which must not be in fixnum range.
   Set mpz[0] to a junk value.  */
unsafe fn make_bignum () -> Lisp_Object
{
  return make_bignum_bits (mpz_sizeinbase (mpz[0].as_ptr (), 2));
}

/* Return a Lisp integer equal to N, which must not be in fixnum range.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_bigint (n: intmax_t) -> Lisp_Object
{
  eassert! (FIXNUM_OVERFLOW_P (n as EMACS_INT));
  mpz_set_intmax (mpz[0].as_mut_ptr (), n);
  return make_bignum ();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_biguint (n: uintmax_t) -> Lisp_Object
{
  eassert! (FIXNUM_OVERFLOW_P (n as EMACS_INT));
  mpz_set_uintmax (mpz[0].as_mut_ptr (), n);
  return make_bignum ();
}

/* Return a Lisp integer equal to -N, which must not be in fixnum range.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_neg_biguint (n: uintmax_t) -> Lisp_Object
{
  eassert! (((-MOST_NEGATIVE_FIXNUM) as uintmax_t) < n);
  mpz_set_uintmax (mpz[0].as_mut_ptr (), n);
  mpz_neg (mpz[0].as_mut_ptr (), mpz[0].as_ptr ());
  return make_bignum ();
}

/* Return a Lisp integer with value taken from mpz[0].
   Set mpz[0] to a junk value.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_integer_mpz () -> Lisp_Object
{
  if FASTER_BIGNUM != 0 && mpz_fits_slong_p (mpz[0].as_ptr ()) != 0
    {
      let v: c_long = mpz_get_si (mpz[0].as_ptr ());
      if !FIXNUM_OVERFLOW_P (v as EMACS_INT)
	{ return make_fixnum (v as EMACS_INT); }
    }

  let bits = mpz_sizeinbase (mpz[0].as_ptr (), 2);

  if ! (FASTER_BIGNUM != 0
	&& FIXNUM_OVERFLOW_P (LONG_MIN as EMACS_INT)
	&& FIXNUM_OVERFLOW_P (LONG_MAX as EMACS_INT))
      && bits <= FIXNUM_BITS as usize
    {
      let mut v: EMACS_INT = 0;
      let mut i: c_int = 0; let mut shift: c_int = 0;

      loop
	{
	  let limb: EMACS_INT = mpz_getlimbn (mpz[0].as_ptr (), i as mp_size_t) as EMACS_INT; i += 1;
	  v += limb << shift; shift += GMP_NUMB_BITS;
	  if shift >= bits as c_int { break; }
	}

      if mpz_sgn (mpz[0].as_ptr ()) < 0
	{ v = -v; }

      if !FIXNUM_OVERFLOW_P (v)
	{ return make_fixnum (v); }
    }

  return make_bignum_bits (bits);
}

/* Set RESULT to V.  This code is for when intmax_t is wider than long.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mpz_set_intmax_slow (result: *mut __mpz_struct, v: intmax_t)
{
  let maxlimbs: c_int = (INTMAX_WIDTH + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
  let limb: *mut mp_limb_t = mpz_limbs_write (result, maxlimbs as mp_size_t);
  let mut n: c_int = 0;
  let mut u: uintmax_t = v as uintmax_t;
  let negative = v < 0;
  if negative
    {
      let two: uintmax_t = 2;
      // C's subtraction wraps to UINTMAX_MAX (no mask); Rust rejects the constant overflow.
      u = u.wrapping_neg () & (two << (UINTMAX_WIDTH - 1)).wrapping_sub (1);
    }

  loop
    {
      *limb.offset (n as isize) = u as mp_limb_t;
      n += 1;
      u = if GMP_NUMB_BITS < UINTMAX_WIDTH { u >> GMP_NUMB_BITS } else { 0 };
      if u == 0 { break; }
    }

  mpz_limbs_finish (result, (if negative { -n } else { n }) as mp_size_t);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mpz_set_uintmax_slow (result: *mut __mpz_struct, mut v: uintmax_t)
{
  let maxlimbs: c_int = (UINTMAX_WIDTH + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
  let limb: *mut mp_limb_t = mpz_limbs_write (result, maxlimbs as mp_size_t);
  let mut n: c_int = 0;

  loop
    {
      *limb.offset (n as isize) = v as mp_limb_t;
      n += 1;
      v = if GMP_NUMB_BITS < INTMAX_WIDTH { v >> GMP_NUMB_BITS } else { 0 };
      if v == 0 { break; }
    }

  mpz_limbs_finish (result, n as mp_size_t);
}

/* If Z fits into *PI, store its value there and return true.
   Return false otherwise.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mpz_to_intmax (z: *const __mpz_struct, pi: *mut intmax_t) -> bool
{
  if FASTER_BIGNUM != 0
    {
      if mpz_fits_slong_p (z) != 0
	{
	  *pi = mpz_get_si (z);
	  return true;
	}
      if LONG_MIN <= INTMAX_MIN && INTMAX_MAX <= LONG_MAX
	{ return false; }
    }

  let bits: ptrdiff_t = mpz_sizeinbase (z, 2) as ptrdiff_t;
  let negative = mpz_sgn (z) < 0;

  if bits < INTMAX_WIDTH as ptrdiff_t
    {
      let mut v: intmax_t = 0; let mut i: c_int = 0; let mut shift: c_int = 0;

      loop
	{
	  let limb: intmax_t = mpz_getlimbn (z, i as mp_size_t) as intmax_t; i += 1;
	  v += limb << shift; shift += GMP_NUMB_BITS;
	  if shift >= bits as c_int { break; }
	}

      *pi = if negative { -v } else { v };
      return true;
    }
  if bits == INTMAX_WIDTH as ptrdiff_t && INTMAX_MIN < -INTMAX_MAX && negative
      && mpz_scan1 (z, 0) == (INTMAX_WIDTH - 1) as mp_bitcnt_t
    {
      *pi = INTMAX_MIN;
      return true;
    }
  return false;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mpz_to_uintmax (z: *const __mpz_struct, pi: *mut uintmax_t) -> bool
{
  if FASTER_BIGNUM != 0
    {
      if mpz_fits_ulong_p (z) != 0
	{
	  *pi = mpz_get_ui (z) as uintmax_t;
	  return true;
	}
      if UINTMAX_MAX <= ULONG_MAX as uintmax_t
	{ return false; }
    }

  if mpz_sgn (z) < 0
    { return false; }
  let bits: ptrdiff_t = mpz_sizeinbase (z, 2) as ptrdiff_t;
  if (UINTMAX_WIDTH as ptrdiff_t) < bits
    { return false; }

  let mut v: uintmax_t = 0; let mut i: c_int = 0; let mut shift: c_int = 0;

  loop
    {
      let limb: uintmax_t = mpz_getlimbn (z, i as mp_size_t) as uintmax_t; i += 1;
      v += limb << shift; shift += GMP_NUMB_BITS;
      if shift >= bits as c_int
	{ break; }
    }

  *pi = v;
  return true;
}

/* Return the value of the bignum X if it fits, 0 otherwise.
   A bignum cannot be zero, so 0 indicates failure reliably.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bignum_to_intmax (x: Lisp_Object) -> intmax_t
{
  let mut i: intmax_t = 0;
  return if mpz_to_intmax (xbignum_val (x), &mut i) { i } else { 0 };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bignum_to_uintmax (x: Lisp_Object) -> uintmax_t
{
  let mut i: uintmax_t = 0;
  return if mpz_to_uintmax (xbignum_val (x), &mut i) { i } else { 0 };
}

/* Multiply and exponentiate mpz_t values without aborting due to size
   limits.  */

/* GMP tests for this value and aborts (!) if it is exceeded.
   This is as of GMP 6.1.2 (2016); perhaps future versions will differ.  */
const fn min_const (a: isize, b: isize) -> isize { if a < b { a } else { b } }
const fn min_const_u (a: usize, b: usize) -> usize { if a < b { a } else { b } }

pub const GMP_NLIMBS_MAX: c_int =
  min_const (INT_MAX as isize, (ULONG_MAX / GMP_NUMB_BITS as c_ulong) as isize) as c_int;

/* An upper bound on limb counts, needed to prevent libgmp and/or
   Emacs from aborting or otherwise misbehaving.  This bound applies
   to estimates of mpz_t sizes before the mpz_t objects are created,
   as opposed to integer-width which operates on mpz_t values after
   creation and before conversion to Lisp bignums.  */
pub const NLIMBS_LIMIT: c_int =
  /* libgmp needs to store limb counts.  */
  min_const (min_const (GMP_NLIMBS_MAX as isize,
	    /* Size calculations need to work.  */
	    (min_const_u (PTRDIFF_MAX as usize, SIZE_MAX)
	     / std::mem::size_of::<mp_limb_t> ()) as isize),
       /* Emacs puts bit counts into fixnums.  */
       (MOST_POSITIVE_FIXNUM / GMP_NUMB_BITS as EMACS_INT) as isize) as c_int;

// C's inner `min (PTRDIFF_MAX, SIZE_MAX)` is evaluated in size_t, so the port does that
// level in `usize` (the signed form collapses the bound to 0 — review B's B-1, a P0).
const _: () = assert! (GMP_NLIMBS_MAX == 2147483647);
const _: () = assert! (NLIMBS_LIMIT == 2147483647);

/* Like mpz_size, but tell the compiler the result is a nonnegative int.  */

unsafe fn emacs_mpz_size (op: *const __mpz_struct) -> c_int
{
  let size: mp_size_t = mpz_size (op) as mp_size_t;
  eassume! (0 <= size && size <= INT_MAX as mp_size_t);
  return size as c_int;
}

/* Wrappers to work around GMP limitations.  As of GMP 6.1.2 (2016),
   the library code aborts when a number is too large.  These wrappers
   avoid the problem for functions that can return numbers much larger
   than their arguments.  For slowly-growing numbers, the integer
   width checks in bignum.c should suffice.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_mpz_mul (rop: *mut __mpz_struct, op1: *const __mpz_struct,
					op2: *const __mpz_struct)
{
  if NLIMBS_LIMIT - emacs_mpz_size (op1) < emacs_mpz_size (op2)
    { overflow_error (); }
  mpz_mul (rop, op1, op2);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_mpz_mul_2exp (rop: *mut __mpz_struct, op1: *const __mpz_struct,
					     op2: EMACS_INT)
{
  /* Fudge factor derived from GMP 6.1.2, to avoid an abort in
     mpz_mul_2exp (look for the '+ 1' in its source code).  */
  const mul_2exp_extra_limbs: c_int = 1;
  let lim: c_int = min (NLIMBS_LIMIT as isize,
			  (GMP_NLIMBS_MAX - mul_2exp_extra_limbs) as isize) as c_int;

  let op2limbs: EMACS_INT = op2 / GMP_NUMB_BITS as EMACS_INT;
  if ((lim as EMACS_INT) - emacs_mpz_size (op1) as EMACS_INT) < op2limbs
    { overflow_error (); }
  mpz_mul_2exp (rop, op1, op2 as mp_bitcnt_t);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_mpz_pow_ui (rop: *mut __mpz_struct, base: *const __mpz_struct,
					   exp: c_ulong)
{
  /* This fudge factor is derived from GMP 6.1.2, to avoid an abort in
     mpz_n_pow_ui (look for the '5' in its source code).  */
  const pow_ui_extra_limbs: c_int = 5;
  let lim: c_int = min (NLIMBS_LIMIT as isize,
			  (GMP_NLIMBS_MAX - pow_ui_extra_limbs) as isize) as c_int;

  let nbase: c_int = emacs_mpz_size (base);
  // C's ckd_mul multiplies in the operands' common type, so `exp` is not narrowed first.
  let (prod, mul_overflow) = (nbase as c_ulong).overflowing_mul (exp); let n = prod as c_int;
  if mul_overflow || (INT_MAX as c_ulong) < prod || lim < n { overflow_error (); }
  mpz_pow_ui (rop, base, exp);
}

/* Yield an upper bound on the buffer size needed to contain a C
   string representing the NUM in base BASE.  This includes any
   preceding '-' and the terminating null.  */
unsafe fn mpz_bufsize (num: *const __mpz_struct, base: c_int) -> ptrdiff_t
{
  return (mpz_sizeinbase (num, base) + 2) as ptrdiff_t;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bignum_bufsize (num: Lisp_Object, base: c_int) -> ptrdiff_t
{
  return mpz_bufsize (xbignum_val (num), base);
}

/* Convert NUM to a nearest double, as opposed to mpz_get_d which
   truncates toward zero.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mpz_get_d_rounded (num: *const __mpz_struct) -> c_double
{
  let size: ptrdiff_t = mpz_bufsize (num, 10);

  /* Use mpz_get_d as a shortcut for a bignum so small that rounding
     errors cannot occur, which is possible if EMACS_INT (not counting
     sign) has fewer bits than a double significand.  */
  if ! ((FLT_RADIX == 2 && DBL_MANT_DIG <= FIXNUM_BITS as c_int - 1)
	|| (FLT_RADIX == 16 && DBL_MANT_DIG * 4 <= FIXNUM_BITS as c_int - 1))
      && size <= DBL_DIG as ptrdiff_t + 2
    { return mpz_get_d (num); }

  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let buf = SAFE_ALLOCA! (sa_avail, size as usize) as *mut c_char;
  mpz_get_str (buf, 10, num);
  let result: c_double = strtod (buf, std::ptr::null_mut ());
  SAFE_FREE! (sa_count);
  return result;
}

/* Store into BUF (of size SIZE) the value of NUM as a base-BASE string.
   If BASE is negative, use upper-case digits in base -BASE.
   Return the string's length.
   SIZE must equal bignum_bufsize (NUM, abs (BASE)).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bignum_to_c_string (buf: *mut c_char, size: ptrdiff_t,
					     num: Lisp_Object, base: c_int) -> ptrdiff_t
{
  eassert! (bignum_bufsize (num, base.abs ()) == size);
  mpz_get_str (buf, base, xbignum_val (num));
  let n: ptrdiff_t = size - 2;
  return if *buf.offset (n - 1) == 0 { n - 1 }
    else { n + (*buf.offset (n) != 0) as ptrdiff_t };
}

/* Convert NUM to a base-BASE Lisp string.
   If BASE is negative, use upper-case digits in base -BASE.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bignum_to_string (num: Lisp_Object, base: c_int) -> Lisp_Object
{
  let size: ptrdiff_t = bignum_bufsize (num, base.abs ());
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let str_ = SAFE_ALLOCA! (sa_avail, size as usize) as *mut c_char;
  let len: ptrdiff_t = bignum_to_c_string (str_, size, num, base);
  let result: Lisp_Object = make_unibyte_string (str_, len);
  SAFE_FREE! (sa_count);
  return result;
}

/* Create a bignum by scanning NUM, with digits in BASE.
   NUM must consist of an optional '-', a nonempty sequence
   of base-BASE digits, and a terminating null byte, and
   the represented number must not be in fixnum range.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_bignum_str (num: *const c_char, base: c_int) -> Lisp_Object
{
  let b: *mut Lisp_Bignum = ALLOCATE_PLAIN_PSEUDOVECTOR! (Lisp_Bignum, pvec_type::PVEC_BIGNUM);
  mpz_init ((*b).value.as_mut_ptr ());
  let check: c_int = mpz_set_str ((*b).value.as_mut_ptr (), num, base);
  eassert! (check == 0);
  return make_lisp_ptr (b as *mut c_void, Lisp_Type::Lisp_Vectorlike);
}

/* Check that X is a Lisp integer in the range LO..HI.
   Return X's value as an intmax_t.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_integer_range (x: Lisp_Object, lo: intmax_t,
					      hi: intmax_t) -> intmax_t
{
  CHECK_INTEGER (x);
  let mut i: intmax_t = 0;
  if ! (integer_to_intmax (x, &mut i) && lo <= i && i <= hi)
    { args_out_of_range_3 (x, make_int (lo), make_int (hi)); }
  return i;
}

/* Check that X is a Lisp integer in the range 0..HI.
   Return X's value as an uintmax_t.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_uinteger_max (x: Lisp_Object, hi: uintmax_t) -> uintmax_t
{
  CHECK_INTEGER (x);
  let mut i: uintmax_t = 0;
  if ! (integer_to_uintmax (x, &mut i) && i <= hi)
    { args_out_of_range_3 (x, make_fixnum (0), make_uint (hi)); }
  return i;
}

/* Check that X is a Lisp integer no greater than INT_MAX,
   and return its value or zero, whichever is greater.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_int_nonnegative (x: Lisp_Object) -> c_int
{
  CHECK_INTEGER (x);
  return if NILP (Fnatnump (x)) { 0 } else { check_integer_range (x, 0, INT_MAX as intmax_t) as c_int };
}

/* Return a random mp_limb_t.  */

unsafe fn get_random_limb () -> mp_limb_t
{
  if GMP_NUMB_BITS <= ULONG_WIDTH
    { return get_random_ulong (); }

  /* Work around GCC -Wshift-count-overflow false alarm.  */
  let shift: c_int = if GMP_NUMB_BITS <= ULONG_WIDTH { 0 } else { ULONG_WIDTH };

  /* This is in case someone builds GMP with unusual definitions for
     MINI_GMP_LIMB_TYPE or _LONG_LONG_LIMB.  */
  let mut r: mp_limb_t = 0;
  let mut i: c_int = 0;
  while i < GMP_NUMB_BITS
    {
      r = (r << shift) | get_random_ulong ();
      i += ULONG_WIDTH;
    }
  return r;
}

/* Return a random mp_limb_t I in the range 0 <= I < LIM.
   If LIM is zero, simply return a random mp_limb_t.  */

unsafe fn get_random_limb_lim (lim: mp_limb_t) -> mp_limb_t
{
  /* Return the remainder of a random mp_limb_t R divided by LIM,
     except reject the rare case where R is so close to the maximum
     mp_limb_t that the remainder isn't random.  */
  let difflim: mp_limb_t = lim.wrapping_neg ();
  let mut diff: mp_limb_t = 0; let mut remainder: mp_limb_t = 0;
  loop
    {
      let r: mp_limb_t = get_random_limb ();
      if lim == 0
	{ return r; }
      remainder = r % lim;
      diff = r - remainder;
      if !(difflim < diff) { break; }
    }

  return remainder;
}

/* Return a random Lisp integer I in the range 0 <= I < LIMIT,
   where LIMIT is a positive bignum.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_random_bignum (limit: *const Lisp_Bignum) -> Lisp_Object
{
  let lim: *const __mpz_struct = bignum_val (limit);
  let nlimbs: mp_size_t = mpz_size (lim) as mp_size_t;
  eassume! (0 < nlimbs);
  let r_limb: *mut mp_limb_t = mpz_limbs_write (mpz[0].as_mut_ptr (), nlimbs);
  let lim_limb: *const mp_limb_t = mpz_limbs_read (lim);
  let limhi: mp_limb_t = *lim_limb.offset ((nlimbs - 1) as isize);
  eassert! (limhi != 0);
  let mut edgy: bool = false;

  loop
    {
      /* Generate the result one limb at a time, most significant first.
	 Choose the most significant limb RHI randomly from 0..LIMHI,
	 where LIMHI is the LIM's first limb, except choose from
	 0..(LIMHI-1) if there is just one limb.  RHI == LIMHI is an
	 unlucky edge case as later limbs might cause the result to
	 exceed or equal LIM; if this happens, it causes another
	 iteration in the outer loop.  */

      let rhi: mp_limb_t = get_random_limb_lim (limhi + (1 < nlimbs) as mp_limb_t);
      edgy = rhi == limhi;
      *r_limb.offset ((nlimbs - 1) as isize) = rhi;

      let mut i: mp_size_t = nlimbs - 1;
      while 0 < i
	{
	  i -= 1;
	  /* get_random_limb_lim (edgy ? limb_lim[i] + 1 : 0)
	     would be wrong here, as the full mp_limb_t range is
	     needed in later limbs for the edge case to have the
	     proper weighting.  */
	  let ri: mp_limb_t = get_random_limb ();
	  if edgy
	    {
	      if *lim_limb.offset (i as isize) < ri
		{ break; }
	      edgy = *lim_limb.offset (i as isize) == ri;
	    }
	  *r_limb.offset (i as isize) = ri;
	}
      if !edgy { break; }
    }

  mpz_limbs_finish (mpz[0].as_mut_ptr (), nlimbs);
  return make_integer_mpz ();
}
