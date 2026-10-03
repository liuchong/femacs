#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/timespec-sub.c @ fd7afced403.  The same shape as
// femacs/lib/timespec-add.rs with `ckd_sub`/`overflowing_sub` and C's `borrow`; the
// reference object confirms the name (`nm -g build/emacs-ref/lib/timespec-sub.o` is empty
// because the file is not in the frozen build's object list, but C's non-`static`
// definition exports `timespec_sub`).
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/timespec.rs");   // "timespec.h", <stdckdint.h>, "intprops.h"

macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }

/* Written by Paul Eggert.  */

/* Return the difference between two timespec values A and B.  On
   overflow, return an extremal value.  This assumes 0 <= tv_nsec <
   TIMESPEC_HZ.  */

/* struct timespec timespec_sub (struct timespec a, struct timespec b)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timespec_sub (a: timespec, b: timespec) -> timespec
{
  let nsdiff: c_int = (a.tv_nsec - b.tv_nsec) as c_int;
  let borrow: c_int = (nsdiff < 0) as c_int;
  let mut rs: time_t;
  /* C 19: `bool v = ckd_sub (&rs, a.tv_sec, b.tv_sec);`.  */
  let (diff, overflowed) = a.tv_sec.overflowing_sub (b.tv_sec);
  rs = diff;
  let v: bool = overflowed;
  let mut rns: c_int;
  /* C 21: `if (v == ckd_sub (&rs, rs, borrow))`.  */
  let (diff2, overflowed2) = rs.overflowing_sub (borrow as time_t);
  rs = diff2;
  if v == overflowed2
    { rns = nsdiff + TIMESPEC_HZ * borrow; }
  else
    {
      if (TYPE_MINIMUM! (time_t) + TYPE_MAXIMUM! (time_t)) / 2 < rs
        {
          rs = TYPE_MINIMUM! (time_t);
          rns = 0;
        }
      else
        {
          rs = TYPE_MAXIMUM! (time_t);
          rns = TIMESPEC_HZ - 1;
        }
    }

  return make_timespec (rs, rns as c_long);
}
