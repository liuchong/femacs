// femacs: port of src/systime.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): types, constants, the header's two `INLINE` functions and
// the declarations C spells `extern`.  C includes "lisp.h" (the includer's job, 0.1-14) and
// <timespec.h> (lib/include/timespec.rs, which pulls gnulib_config.rs); `<sys/time.h>` is
// the port's sys_time.rs.  The `<X11/X.h>` and `<support/SupportDefs.h>` includes belong to
// the dead branches of the `Time` definition below (P-51) and have no mirror: the port
// keeps the branch's effect (`Time` is `unsigned long` / `int64`) under its own feature.

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_SYSTIME_H: bool = true;

include!("../../lib/include/timespec.rs");      // #include <timespec.h>

// C: `#ifdef HAVE_X_WINDOWS # include <X11/X.h> #elif defined HAVE_HAIKU
// # include <support/SupportDefs.h> typedef int64 Time; #else typedef unsigned long Time;
// #endif`.  The frozen configuration takes the `#else` (HAVE_X_WINDOWS and HAIKU are both
// 0, P-08); the other two branches keep C's text under their features (P-51), and X11's
// `Time` — which the port has no mirror for — is `unsigned long` on every supported X
// target, which is what the branch has to produce.
#[cfg(feature = "have_x_windows")]
pub type Time = c_ulong;                     // from <X11/X.h>: typedef unsigned long Time
#[cfg(all(feature = "have_haiku", not(feature = "have_x_windows")))]
pub type Time = i64;                         // typedef int64 Time
#[cfg(not(any(feature = "have_x_windows", feature = "have_haiku")))]
pub type Time = c_ulong;                     // typedef unsigned long Time
const _: () = assert! (!cfg! (feature = "have_x_windows") || HAVE_X_WINDOWS);
const _: () = assert! (!cfg! (feature = "have_haiku") || HAIKU);

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

/* The declarations live in a uniquely-named module with a glob import: `timefns.c`
   *defines* the seven functions systime.h declares for it (and `keyboard.c` defines the
   eighth), and a definition at the unit's crate root would collide with a crate-root
   declaration (E0428, PORTING.md 0.1-10; the same shape time.in.rs's `time_decls` uses).
   `systime_decls`, not `decls`: two mirrors in one unit must not share the module name
   (docs/status.md 3.18).  Parameter names come from the definitions (C's prototypes have
   none): `keyboard.c`'s `set_waiting_for_input` and `timefns.c`'s seven.  */
pub mod systime_decls {
  use super::*;

  unsafe extern "C" {
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
}
use systime_decls::*;
