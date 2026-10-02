/* Get the current time.

   Copyright (C) 2005, 2007, 2009-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Paul Eggert.  */

/* emacs/lib/gettime.c (@ 51 lines) is on the verified list: `CLOCK_REALTIME` is defined by this
   target's <time.h> and HAVE_CLOCK_GETTIME is 1, so C 30-32's `clock_gettime` arm is the live
   text and the `timespec_get`/`gettimeofday` arms are dead by measurement.  The port therefore
   declares only the two system entries that arm needs, in-unit: `CLOCK_REALTIME` (0, measured
   with clang) and `clock_gettime`.  C 23's "timespec.h" is the timespec mirror, which is also
   where the prototypes of both functions below live (`pub fn gettime (…)` and
   `pub fn current_timespec ()`).  */
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/timespec.rs");

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
  /* C 30-32's `#if defined CLOCK_REALTIME && HAVE_CLOCK_GETTIME` arm, the live one.  */
  clock_gettime (CLOCK_REALTIME, ts);
}

/* Return the current system time as a struct timespec.  */

/* C 45-50's `struct timespec current_timespec (void)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn current_timespec () -> timespec
{
  let mut ts: timespec = std::mem::zeroed ();
  gettime (&mut ts);
  return ts;
}
