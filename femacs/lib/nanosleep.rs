// femacs: port of lib/nanosleep.c @ fd7afced403 .
//
// Three arms define one function.  The frozen configuration *does* set
// `HAVE_BUG_BIG_NANOSLEEP` (the reference build's config.h records it), so C 39-80's arm is
// the live text: the replacement splits a long sleep into 24-day chunks.  C 82-177's
// `#elif defined _WIN32 && ! defined __CYGWIN__` arm (the PerformanceCounter busy-loop) and
// C 180-191's `#else` arm (the `pselect` fallback) are the non-frozen sides and are kept as
// text under features that name them (P-51).
//
// Deviations (docs/ports/nanosleep.md): `static_assert`/`TYPE_MAXIMUM` are the port's
// `static_assert!` and a unit-local `TYPE_MAXIMUM!` (PORTING.md 2.5), the `# undef
// nanosleep` before the definition has no counterpart (Rust's own name is the entry point),
// and `pselect` is declared locally with `void *` for the three `fd_set *` parameters, which
// only the dead arm passes `NULL` (the P-55 shape).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* nanosleep.c -- a work around for a bug in nanosleep ().
   Copyright (C) 2009-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/sys_types.in.rs");
include!("include/time.in.rs");     // struct timespec
include!("include/sys_time.rs");
include!("include/stdint.in.rs");
include!("include/stdio.rs");
include!("include/errno.rs");

/// PORTING.md 2.5: C's `static_assert` is `const _: () = assert! (cond);`.
macro_rules! static_assert {
    ($cond:expr) => { const _: () = assert! ($cond); };
}
/// intprops.h's `TYPE_MAXIMUM (t)`: the largest value of the type `t`.
macro_rules! TYPE_MAXIMUM {
    ($t:ty) => { <$t>::MAX };
}

pub const BILLION: c_int = 1000 * 1000 * 1000;

/* the `#else` arm's one call; the three `fd_set *` parameters are only ever NULL there,
   which is why the port spells them `*mut c_void` (PORTING.md P-55).  */
unsafe extern "C" {
  /* int pselect (int nfds, fd_set *readfds, fd_set *writefds, fd_set *exceptfds,
                 const struct timespec *timeout, const sigset_t *sigmask);  */
  pub fn pselect (nfds: c_int, readfds: *mut c_void, writefds: *mut c_void,
                  exceptfds: *mut c_void, timeout: *const timespec,
                  sigmask: *const c_void) -> c_int;
}

/* C 39's live arm (HAVE_BUG_BIG_NANOSLEEP is 1).  */
/* int nanosleep (const struct timespec *requested_delay,
                  struct timespec *remaining_delay)  */
/* The generated <time.h> gives `#define nanosleep rpl_nanosleep` when the replacement exists,
   and C 41's `# undef nanosleep` (between the declarator and the body) undoes it: the
   *definition* is `rpl_nanosleep`, while the body's `nanosleep (...)` calls are the system
   entry.  The port emits the renamed symbol and keeps C's source name on the item (PORTING.md
   10.13①, the pthread_sigmask.rs shape); the system calls below go through `time_decls`.  */
#[unsafe(export_name = "rpl_nanosleep")]
pub unsafe extern "C" fn nanosleep (requested_delay: *const timespec,
                                   remaining_delay: *mut timespec) -> c_int
{
  /* C 82-177's `#elif defined _WIN32 && ! defined __CYGWIN__` arm (the Windows
     implementation, whose `WIN32_LEAN_AND_MEAN` is recorded in
     docs/ports/excluded-macros.txt) is dead text on this target: the tokens it names are
     recorded in docs/ports/macro-body-notes.tsv, as the gettimeofday.c Windows arm's are,
     because the arm needs <windows.h>'s types.  The two statics it declares are what the
     port keeps in text.  */
  #[cfg (feature = "windows_native")]
  {
    /* C 92-95's `static bool initialized;` and `static double
       ticks_per_nanosecond;` (Number of performance counter increments per nanosecond, or
       zero if it could not be determined).  */
    static mut initialized: bool = false;
    static mut ticks_per_nanosecond: c_double = 0.0;
    let _ = (initialized, ticks_per_nanosecond);
  }

  /* C 180-191's `#else` arm, kept as text (P-51; dead: HAVE_BUG_BIG_NANOSLEEP is 1).  */

  /* Other platforms lacking nanosleep.
     It's not clear whether these are still practical porting targets.
     For now, just fall back on pselect.  */

  /* Suspend execution for at least *REQUESTED_DELAY seconds.  The
     *REMAINING_DELAY part isn't implemented yet.  */
  #[cfg (feature = "no_bug_big_nanosleep")]
  {
    return pselect (0, std::ptr::null_mut (), std::ptr::null_mut (),
                    std::ptr::null_mut (), requested_delay, std::ptr::null ());
  }

  if ((*requested_delay).tv_nsec < 0 || BILLION as c_long <= (*requested_delay).tv_nsec)
    {
      *errno () = EINVAL;
      return -1;
    }

  {
    /* Verify that time_t is large enough.  */
    static_assert! ((TYPE_MAXIMUM! (time_t) / 24 / 24 / 60 / 60) != 0);
    let limit: time_t = 24 * 24 * 60 * 60;
    let mut seconds: time_t = (*requested_delay).tv_sec;
    let mut intermediate: timespec = *requested_delay;

    while (limit < seconds)
      {
        intermediate.tv_sec = limit;
        let result: c_int = time_decls::nanosleep (&intermediate as *const timespec, remaining_delay);
        seconds -= limit;
        if (result != 0)
          {
            if !remaining_delay.is_null ()
              { (*remaining_delay).tv_sec += seconds; }
            return result;
          }
        intermediate.tv_nsec = 0;
      }
    intermediate.tv_sec = seconds;
    return time_decls::nanosleep (&intermediate as *const timespec, remaining_delay);
  }
}
