// femacs: port of src/systime.h @ fd7afced403 .
//
// Header mirror (PORTING.md 0.1): types, constants, the header's two `INLINE` functions and
// the declarations C spells `extern`.  C includes "lisp.h" (the includer's job, 0.1-14) and
// <timespec.h> (lib/include/timespec.rs, which pulls gnulib_config.rs); `<sys/time.h>` is
// the port's sys_time.rs.  The `<X11/X.h>` and `<support/SupportDefs.h>` includes belong to
// the dead branches of the `Time` definition below (P-51) and have no mirror: the port
// keeps the branch's effect (`Time` is `unsigned long` / `int64`) under its own feature.

/* systime.h - System-dependent definitions for time manipulations.
   Copyright (C) 1993-1994, 2002-2026 Free Software Foundation, Inc.

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

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_SYSTIME_H: bool = true;

include!("../../lib/include/timespec.rs");      // #include <timespec.h>

// C: `#ifdef HAVE_X_WINDOWS # include <X11/X.h> #elif defined HAVE_HAIKU
// # include <support/SupportDefs.h> typedef int64 Time; #else typedef unsigned long Time;
// #endif`.  The frozen configuration takes the `#else` (HAVE_X_WINDOWS and HAIKU are both
// 0, P-08); the other two branches keep C's text under their features (P-51), and X11's
// `Time` — which the port has no mirror for — is `unsigned long` on every supported X
// target, which is what the branch has to produce.
#[cfg(feature = "x_windows")]
pub type Time = c_ulong;                     // from <X11/X.h>: typedef unsigned long Time
#[cfg(all(feature = "haiku", not(feature = "x_windows")))]
pub type Time = i64;                         // typedef int64 Time
#[cfg(not(any(feature = "x_windows", feature = "haiku")))]
pub type Time = c_ulong;                     // typedef unsigned long Time
const _: () = assert! (!cfg! (feature = "x_windows") || HAVE_X_WINDOWS);
const _: () = assert! (!cfg! (feature = "haiku") || HAIKU);

/* On some configurations (hpux8.0, X11R4), sys/time.h and X11/Xos.h
   disagree about the name of the guard symbol.  */
// C: `#ifdef HPUX #ifdef _STRUCT_TIMEVAL #ifndef __TIMEVAL__ #define __TIMEVAL__ #endif
// #endif #endif`.  HPUX is not the frozen target, so the whole block is dead text; the
// inner `_STRUCT_TIMEVAL` is a system macro the port does not model (recorded in
// docs/ports/include-systime.md).
#[cfg(feature = "hpux")]
pub const __TIMEVAL__: bool = true;
const _: () = assert! (!cfg! (feature = "hpux"));

include!("../../lib/include/sys_time.rs");      // #include <sys/time.h> for `struct timeval`
// C: `#undef hz` (AIX's <sys/param.h> defines it) — there is nothing to undefine in the
// port, which never included the AIX header; recorded in docs/ports/include-systime.md.

/* Emacs uses struct timespec to represent nonnegative temporal intervals.  */
// The warning C spells here (tv_sec may be unsigned; do not use struct timespec as a
// general-purpose data type for A + B or A - B) is C's comment, kept above.

/* Return an invalid timespec.  */
#[inline(always)]
pub fn invalid_timespec () -> timespec
{
  make_timespec (0, -1)
}

/* Return true if TIME is a valid timespec.  This currently doesn't worry
   about whether tv_nsec is less than TIMESPEC_HZ; leap seconds might
   cause a problem if it did.  */
#[inline(always)]
pub fn timespec_valid_p (t: timespec) -> bool
{
  t.tv_nsec >= 0
}

/* Emacs uses the integer list (HI LO US PS) to represent the time
   (HI << LO_TIME_BITS) + LO + US / 1e6 + PS / 1e12.  */
pub const LO_TIME_BITS: c_int = 16;

unsafe extern "C" {
  // Parameter names come from the definitions (C's prototypes have none): `keyboard.c`'s
  // `set_waiting_for_input` and `timefns.c`'s seven.
  pub fn set_waiting_for_input (time_to_clear: *mut timespec);      // keyboard.c
  pub fn make_timeval (t: timespec) -> timeval;                     // timefns.c
  pub fn monotonic_coarse_timespec () -> timespec;                  // timefns.c
  pub fn make_lisp_time (t: timespec) -> Lisp_Object;               // timefns.c
  pub fn timespec_to_lisp (t: timespec) -> Lisp_Object;             // timefns.c
  pub fn list4_to_timespec (high: Lisp_Object, low: Lisp_Object,
                            usec: Lisp_Object, psec: Lisp_Object) -> timespec;  // timefns.c
  pub fn lisp_time_argument (specified_time: Lisp_Object) -> timespec;          // timefns.c
  pub fn float_time (specified_time: Lisp_Object) -> c_double;                  // timefns.c
  pub fn init_timefns ();                                                       // timefns.c
  pub fn syms_of_timefns ();                                                    // timefns.c
}
