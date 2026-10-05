// femacs: port of lib/time.in.h @ fd7afced403 (gnulib).
//
// gnulib template.  The frozen target's generated header is build/emacs-ref/lib/time.h, and
// its substitutions decide which arms are live here:
//
//   TIME_H_DEFINES_STRUCT_TIMESPEC 1   -> the `struct timespec` arm (C 76-105) is dead
//   TIME_H_DEFINES_TIME_UTC        1   -> the TIME_UTC arm (C 119-125) is dead
//   GNULIB_TZNAME                  0   -> the tzname arm (C 127-142) is dead
//   GNULIB_TIMESPEC_GET/_GETRES    0   -> the timespec_get[_getres] arms (C 146-198) are dead
//   GNULIB_TIME                    0   -> the time arm (C 201-219) is dead
//   GNULIB_NANOSLEEP 1 / REPLACE_.. 1  -> the rename arm (C 224-243) is live
//   GNULIB_TZSET 0 / GNULIB_MDA_TZSET 1-> the system tzset (C 251-289)
//   GNULIB_MKTIME 1 / REPLACE_MKTIME 1 -> `mktime` is `rpl_mktime` (C 292-310)
//   GNULIB_TIME_R 1 / REPLACE_.. 0     -> the system gmtime_r/localtime_r (C 315-369)
//   GNULIB_LOCALTIME 0 / REPLACE_.. 0  -> the system localtime (C 374-394)
//   REPLACE_GMTIME 0                   -> the system gmtime (C 396-409)
//   GNULIB_STRPTIME 0                  -> the system strptime (C 414-430)
//   GNULIB_CTIME 0                     -> the system ctime (C 434-453)
//   GNULIB_STRFTIME 0                  -> the system strftime (C 457-482)
//   GNULIB_TIME_RZ 1 / HAVE_TZALLOC 0 / HAVE_TIMEZONE_T 0
//                                      -> the timezone_t typedef + tzalloc/tzfree/
//                                         localtime_rz/mktime_z are live (C 484-591)
//   GNULIB_TIMEGM 1 / REPLACE_TIMEGM 1 -> `timegm` is `rpl_timegm` (C 594-616)
//
// So the mirror's declarations are the *system* entries (the port has no `#include_next`),
// plus the two `rpl_` definitions the reference objects call, and the template's own names
// (the guard, the `GNULIB_defined_*` markers and the renames of the dead arms) are carried
// the way the endian/byteswap `.in.rs` mirrors carry theirs - the extractor reads this
// template blind, so both arms of every `@PLACEHOLDER@` block count as required names.
// `struct __time_t_must_be_integral` (C 107-117) is the one live block spelled as a *macro*
// marker here: it is C's compiler-level assertion that `time_t` is integral, which the
// port's `time_t = i64` satisfies by construction (its only member is a bitfield, which
// PORTING.md 1.3 packs explicitly).

/// The frozen target's `time_t` (a 64-bit signed integer).
pub type time_t = i64;

/// The frozen target's `struct timespec`; `buffer.h` embeds one (`modtime`), and
/// its size and the offsets inside `struct buffer` are asserted against the C
/// compiler's numbers.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timespec
{
  pub tv_sec: time_t,
  pub tv_nsec: c_long,
}

/// The frozen target's `struct tm`; strftime.c and mktime.c read and write it field by
/// field, and the `HAVE_STRUCT_TM_TM_GMTOFF`/`_ZONE` values (both 1 for this target)
/// make `tm_gmtoff`/`tm_zone` live.  The layout is the frozen target's, measured with
/// the frozen configuration:
///
///     $ cat > /tmp/tm.c <<'EOF'
///     #include <stddef.h>
///     #include <time.h>
///     #include <stdio.h>
///     int main (void) { printf ("%zu %zu %zu %zu\n", sizeof (struct tm),
///         offsetof (struct tm, tm_isdst), offsetof (struct tm, tm_gmtoff),
///         offsetof (struct tm, tm_zone)); return 0; }
///     EOF
///     $ clang -o /tmp/tm /tmp/tm.c && /tmp/tm
///     56 32 40 48
///
/// i.e. nine `int` members at 0..32 (with `tm_isdst` last), four bytes of padding, the
/// `long tm_gmtoff` at 40 and the `char *tm_zone` at 48.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tm
{
  pub tm_sec: c_int,
  pub tm_min: c_int,
  pub tm_hour: c_int,
  pub tm_mday: c_int,
  pub tm_mon: c_int,
  pub tm_year: c_int,
  pub tm_wday: c_int,
  pub tm_yday: c_int,
  pub tm_isdst: c_int,
  pub tm_gmtoff: c_long,
  pub tm_zone: *const c_char,
}

const _: () = core::assert! (std::mem::size_of::<tm> () == 56);
const _: () = core::assert! (std::mem::offset_of! (tm, tm_sec) == 0);
const _: () = core::assert! (std::mem::offset_of! (tm, tm_isdst) == 32);
const _: () = core::assert! (std::mem::offset_of! (tm, tm_gmtoff) == 40);
const _: () = core::assert! (std::mem::offset_of! (tm, tm_zone) == 48);

/// `time.in.h` 494's `typedef struct tm_zone *timezone_t;` - the template only
/// forward-declares `struct tm_zone` (its definition is `lib/time-internal.h`'s), so the
/// port spells the pointer as an opaque `*mut c_void` (PORTING.md 0.1-2); the cookie
/// `local_tz` is `(timezone_t) 1` in the time-internal mirror, which is never
/// dereferenced (time-internal.h 51-54).
pub type timezone_t = *mut c_void;

/* The declarations live in a uniquely-named module with a glob import: lib/time_r.c
   *defines* `gmtime_r`/`localtime_r`, and a definition at the unit's crate root would
   collide with a crate-root declaration (E0428, PORTING.md 0.1-10; the utimens.rs/
   stdio.rs shape).  `time_decls`, not `decls`: two mirrors in one unit must not share
   the module name (docs/status.md 3.18).  */
pub mod time_decls {
  use super::*;

  unsafe extern "C" {
    /* struct tm *gmtime (time_t const *);  */
    pub fn gmtime (t: *const time_t) -> *mut tm;
    /* struct tm *localtime (time_t const *);  */
    pub fn localtime (t: *const time_t) -> *mut tm;
    /* struct tm *gmtime_r (time_t const *restrict, struct tm *restrict);  */
    pub fn gmtime_r (t: *const time_t, tp: *mut tm) -> *mut tm;
    /* struct tm *localtime_r (time_t const *restrict, struct tm *restrict);  */
    pub fn localtime_r (t: *const time_t, tp: *mut tm) -> *mut tm;
    /* time_t mktime (struct tm *);  */
    pub fn mktime (tp: *mut tm) -> time_t;
    /* time_t timegm (struct tm *);  (HAVE_TIMEGM is 1 for this target)  */
    pub fn timegm (tp: *mut tm) -> time_t;
    /* void tzset (void);  */
    pub fn tzset ();
    /* size_t strftime (char *restrict, size_t, char const *restrict, struct tm const *restrict);  */
    pub fn strftime (s: *mut c_char, maxsize: usize, format: *const c_char, tp: *const tm)
      -> usize;
    /* time.h 1088-1120: the two entries the time_rz module adds (`REPLACE_LOCALTIME_RZ`
       and `REPLACE_MKTIME_Z` are undefined, so these are the sys spellings).  */
    pub fn localtime_rz (tz: timezone_t, t: *const time_t, tp: *mut tm) -> *mut tm;
    pub fn mktime_z (tz: timezone_t, tp: *mut tm) -> time_t;
    /* extern char *tzname[2];  */
    pub static mut tzname: [*mut c_char; 2];
  }

  /* time.h 841-842 / 1144-1145 (the frozen build's *generated* replacement header): gnulib's
     REPLACE_MKTIME/REPLACE_TIMEGM arms rename the two entries this port replaces --
     `mktime` becomes `rpl_mktime` and `timegm` becomes `rpl_timegm` -- and the reference
     objects call those symbols (`nm -g build/emacs-ref/lib/time_rz.o` lists `U _rpl_mktime`
     and `U _rpl_timegm`, and nstrftime.o lists `U _mktime_z`).  Appended as a second block
     rather than merged into the one above so the change stays a pure addition.  */
  unsafe extern "C" {
    /* time_t rpl_mktime (struct tm *);  */
    pub fn rpl_mktime (tp: *mut tm) -> time_t;
    /* time_t rpl_timegm (struct tm *);  -- the REPLACE_TIMEGM arm's name; the reference
       object calls it (`U _rpl_timegm`), and time_rz.rs's `timegm!` macro expands to it.  */
    pub fn rpl_timegm (tp: *mut tm) -> time_t;
    /* int nanosleep (const struct timespec *, struct timespec *);
       This is the *system* entry, not gnulib's replacement: nanosleep.c's `# undef
       nanosleep` (C 41, between the declarator and the body) is what makes its own definition
       `rpl_nanosleep` and its body's calls the system ones.  The port reaches it through this
       module, because the unit's own definition shadows the glob (PORTING.md 0.1-10, the
       pthread_sigmask.rs precedent).  */
    pub fn nanosleep (requested_delay: *const timespec, remaining_delay: *mut timespec)
      -> c_int;
  }

  /* The frozen generated <time.h> keeps gnulib's own blocks for `ctime`, `time`,
     `timespec_get` and `timespec_getres` dead (`GNULIB_CTIME`, `GNULIB_TIME`,
     `GNULIB_TIMESPEC_GET` and `GNULIB_TIMESPEC_GETRES` are all 0 there), so those four are
     the *system* entries - what the dead arms' `_GL_CXXALIAS_SYS` reaches - and the port
     declares them here.  `tzalloc`/`tzfree` are live (C 511-529, `HAVE_TZALLOC` is 0), and
     `lib/time_rz.rs` defines them; the declarations sit in this module so that definition
     shadows the glob import (PORTING.md 0.1-10, the `rpl_mktime` block above).  */
  unsafe extern "C" {
    /* char *ctime (time_t const *);  */
    pub fn ctime (t: *const time_t) -> *mut c_char;
    /* time_t time (time_t *);  */
    pub fn time (t: *mut time_t) -> time_t;
    /* int timespec_get (struct timespec *, int);  (C11)  */
    pub fn timespec_get (ts: *mut timespec, base: c_int) -> c_int;
    /* int timespec_getres (struct timespec *, int);  (C23)  */
    pub fn timespec_getres (ts: *mut timespec, base: c_int) -> c_int;
    /* timezone_t tzalloc (char const *);  */
    pub fn tzalloc (name: *const c_char) -> timezone_t;
    /* void tzfree (timezone_t);  */
    pub fn tzfree (tz: timezone_t);
  }
}
use time_decls::*;

/* ---- the template's own half (PORTING.md P-26, the endian.in.rs shape) ---------------- */

/* C 45: `# define _@GUARD_PREFIX@_TIME_H` - the frozen substitution is `_GL_TIME_H`
   (build/emacs-ref/lib/time.h 45).  The template's own spelling reads as `_`, which
   docs/ports/excluded-macros.txt records.  */
pub const _GL_TIME_H: c_int = 1;

/* C 49-53's `# if defined __MINGW32__` arm pins `_POSIX_THREAD_SAFE_FUNCTIONS` to 200112L.
   Dead on this target (the frozen compiler does not define __MINGW32__); the arm's two
   lines are kept under the feature that names the non-frozen side (P-51) with C's value,
   because the extractor reads this template blind and asks for both the name and the
   value.  */
#[cfg (feature = "mingw32")]
pub const _POSIX_THREAD_SAFE_FUNCTIONS: core::ffi::c_long = 200112;

/* C 76-105's `# if !@TIME_H_DEFINES_STRUCT_TIMESPEC@` arm: the frozen substitution is 1
   (the system <time.h> defines `struct timespec`), so the arm is dead (`#if ! 1`) and the
   struct is the system's (the `struct timespec` above).  Its rename
   `#    define timespec rpl_timespec` is kept under the feature naming the non-frozen side;
   the extractor asks for the name and compares the macro's body, whose single token is
   `rpl_timespec`.  */
#[cfg (feature = "no_time_h_defines_struct_timespec")]
macro_rules! timespec { () => { rpl_timespec } }
/* C 89-97: the arm's guard macro.  Dead here (see above), carried as the value C's
   preprocessor defines in that arm.  */
pub const GNULIB_defined_struct_timespec: c_int = 1;

/* C 107-117: the one block of this header that is live on the frozen target - the
   compiler-level assertion that `time_t` is an integral type.  C spells it as a struct
   whose only member is a one-bit bitfield of width `(time_t) 1`, which fails to compile
   when `time_t` is floating; the port's `time_t` is `i64` (the alias above), so the
   assertion holds by construction and PORTING.md 1.3's bitfield rule leaves no member to
   mirror.  The marker C sets is carried.  */
pub const GNULIB_defined_struct_time_t_must_be_integral: c_int = 1;

/* C 119-125's `# if ! @TIME_H_DEFINES_TIME_UTC@`: the frozen substitution is 1 (the system
   defines TIME_UTC), so the arm is dead; the name and its guard are carried because the
   extractor reads this template blind.  */
pub const TIME_UTC: c_int = 1;
pub const GNULIB_defined_TIME_UTC: c_int = 1;

/* C 127-142's `# if @GNULIB_TZNAME@` arm is dead (the frozen substitution is 0); the
   system <time.h> declares `tzname`, which this mirror declares as the static above.  The
   arm's Windows rename `#   define tzname _tzname` is kept under the feature naming the
   non-frozen side, because the extractor asks for the name.  */
#[cfg (feature = "cygwin_win32")]
macro_rules! tzname { () => { _tzname } }

/* C 484-501's `# if @GNULIB_TIME_RZ@` block is live (@GNULIB_TIME_RZ@ is 1), and inside it
   `#  if !@HAVE_TZALLOC@` is live too (@HAVE_TZALLOC@ is 0), so the frozen header declares
   `timezone_t` - the system's own type, since `#    if !@HAVE_TIMEZONE_T@` is true and
   `typedef struct tm_zone *timezone_t;` is the live arm (the `timezone_t` alias above).
   The `#    else` arm's `#     define timezone_t rpl_timezone_t` is dead and is kept under
   the feature naming the non-frozen side; the extractor asks for the name.  */
#[cfg (feature = "have_timezone_t")]
macro_rules! timezone_t { () => { rpl_timezone_t } }
pub const GNULIB_defined_timezone_t: c_int = 1;
