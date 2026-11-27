// femacs: port of lib/gettime.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::sys_common::*;
/* Written by Paul Eggert.  */

/* emacs/lib/gettime.c (@ 51 lines) is on the verified list: `CLOCK_REALTIME` is defined by this
   target's <time.h> and HAVE_CLOCK_GETTIME is 1, so C 30-32's `clock_gettime` arm is the live
   text and the `timespec_get`/`gettimeofday` arms are dead by measurement.  The port therefore
   declares only the two system entries that arm needs, in-unit: `CLOCK_REALTIME` (0, measured
   with clang) and `clock_gettime`.  C 23's "timespec.h" is the timespec mirror, which is also
   where the prototypes of both functions below live (`pub fn gettime (…)` and
   `pub fn current_timespec ()`).  */
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/timespec.rs imported via common

/* <time.h>: `CLOCK_REALTIME` is 0 on the frozen target and `clockid_t` is a 32-bit int
   (measured: `printf ("%d %zu", (int) CLOCK_REALTIME, sizeof (clockid_t))` -> `0 4`).  */
pub const CLOCK_REALTIME: c_int = 0;

unsafe extern "C" {
  pub fn clock_gettime (clock_id: c_int, tp: *mut timespec) -> c_int;
}

/* Get the system time into *TS.  */

/* C 28-36's `void gettime (struct timespec *ts)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gettime (ts: *mut timespec)
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* C 30-32's `#if defined CLOCK_REALTIME && HAVE_CLOCK_GETTIME` arm, the live one.  */
  clock_gettime (CLOCK_REALTIME, ts);

  }
}

/* Return the current system time as a struct timespec.  */

/* C 45-50's `struct timespec current_timespec (void)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn current_timespec () -> timespec
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let mut ts: timespec = std::mem::zeroed ();
  gettime (&mut ts);
  return ts;

  }
}
