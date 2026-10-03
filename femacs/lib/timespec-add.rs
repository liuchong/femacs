#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/timespec-add.c @ fd7afced403.  `ckd_add` is
// today's C23 spelling of the overflowing pair: the port writes the wrapped sum and the
// overflow flag out (PORTING.md 2.6/P-15, the families in docs/ports/token-aliases.tsv;
// mktime.rs is the precedent).  `TYPE_MINIMUM`/`TYPE_MAXIMUM` are unit-local macros over
// `<intprops.h>`'s `_Generic` tables (P-26).  `timespec_add` is declared in the timespec.h
// mirror's `pub mod timespec_decls` + glob import, so this definition shadows it
// (PORTING.md 0.1-10).  The reference object confirms the name:
// `nm -g build/emacs-ref/lib/timespec-add.o` prints `T _timespec_add`.
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/timespec.rs");   // "timespec.h", <stdckdint.h>, "intprops.h"

macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }

/* Written by Paul Eggert.  */

/* Return the sum of two timespec values A and B.  On overflow, return
   an extremal value.  This assumes 0 <= tv_nsec < TIMESPEC_HZ.  */

/* struct timespec timespec_add (struct timespec a, struct timespec b)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timespec_add (a: timespec, b: timespec) -> timespec
{
  /* C 15: the `long`/`long` sum truncates to `int` on assignment.  */
  let nssum: c_int = (a.tv_nsec + b.tv_nsec) as c_int;
  let carry: c_int = (TIMESPEC_HZ <= nssum) as c_int;
  let mut rs: time_t;
  /* C 18: `bool v = ckd_add (&rs, a.tv_sec, b.tv_sec);`.  */
  let (sum, overflowed) = a.tv_sec.overflowing_add (b.tv_sec);
  rs = sum;
  let v: bool = overflowed;
  let mut rns: c_int;
  /* C 20: `if (v == ckd_add (&rs, rs, carry))` - the second call reads what the first
     wrote and writes it again.  */
  let (sum2, overflowed2) = rs.overflowing_add (carry as time_t);
  rs = sum2;
  if v == overflowed2
    { rns = nssum - TIMESPEC_HZ * carry; }
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

  /* C 36: `make_timespec (rs, rns)` converts `rns` (an `int`) to the `long` parameter.  */
  return make_timespec (rs, rns as c_long);
}
