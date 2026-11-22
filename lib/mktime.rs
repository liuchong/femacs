/* The following macros influence what gets defined when this file is compiled:

   Macro/expression            Which gnulib module    This compilation unit
                                                      should define

   _LIBC                       (glibc proper)         mktime

   NEED_MKTIME_WORKING         mktime                 rpl_mktime
   || NEED_MKTIME_WINDOWS

   NEED_MKTIME_INTERNAL        mktime-internal        mktime_internal
 */

// femacs: port of lib/mktime.c @ fd7afced403 (gnulib).
//
// The frozen configuration resolves all 16 of this file's conditions
// (`tools/check-cond-eval.py lib/mktime.c`: 0 unknown, 0 disagreements), so only the live
// text is written: `_LIBC` is undefined, NEED_MKTIME_WORKING and NEED_MKTIME_INTERNAL are
// 1, and NEED_MKTIME_WINDOWS is undefined (the file's own `#ifndef` arm sets it 0).  The
// dead arms are the Windows body of `my_tzset` (C 74-99) and the `_LIBC`-only tail
// (C 557-581, which redefines `mktime` for a 32-bit `time_t`), each noted where C's `#if`
// stood.
//
// The two symbols this unit emits are the ones C 20-31's table names: `rpl_mktime`
// (`__mktime64`, renamed by the generated <time.h>'s `#define mktime rpl_mktime`, since
// REPLACE_MKTIME is 1) and `mktime_internal` (`__mktime_internal`, renamed by
// mktime-internal.h).  The port keeps C's identifiers and emits the symbols with
// `#[unsafe(export_name = ...)]` (PORTING.md 10.13①, the mempcpy precedent); the frozen
// build's own object confirms both: `nm -g build/emacs-ref/lib/mktime.o` prints
// `T _mktime_internal` and `T _rpl_mktime`, and its undefined symbols are the four
// `_gmtime_r`, `_localtime_r`, `_tzset` and `___error` this port reaches through the time
// and errno mirrors.
//
// Deviations (docs/ports/mktime.md): `struct tm tm;`/`struct tm oktm;`/`struct tm otm;` are
// zero-initialized (PORTING.md 1.5), C's `static` forward prototypes are dropped
// (§0.1-12), the `ckd_add`/`ckd_sub` calls are the `overflowing_*` family with the store
// written out (§2.6, P-15; the families in token-aliases.tsv), `TYPE_SIGNED`/
// `TYPE_MINIMUM`/`TYPE_MAXIMUM` are unit-local macros over intprops.h's `_Generic` tables,
// `static_assert` is `static_assert!` and `__set_errno` is the libc-config.h macro
// (§2.5, the nanosleep/canonicalize-lgpl precedents), and `goto offset_found` is the
// labelled block `'offset_found` (§2.2, the getdelim precedent).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/limits.in.rs imported via common
// [U1] include/errno.rs imported via common
// [U1] include/libc-config.rs imported via common
// [U1] include/time.in.rs imported via common
// [U1] include/mktime-internal.rs imported via common

/* C 37-42: `#ifndef LEAP_SECONDS_POSSIBLE` is live, so the file's own `# define … 1`
   applies.  C 55-63's three `#ifndef`s: the configuration defines NEED_MKTIME_INTERNAL and
   NEED_MKTIME_WORKING as 1 (their arms here are dead and are not written), and
   NEED_MKTIME_WINDOWS is undefined, so the file's 0 is the live one.  */
pub const LEAP_SECONDS_POSSIBLE: c_int = 1;
pub const NEED_MKTIME_WINDOWS: c_int = 0;

/* C 70-106: `#if !defined _LIBC && (NEED_MKTIME_WORKING || NEED_MKTIME_WINDOWS)` is live
   (NEED_MKTIME_WORKING is 1).  */
unsafe fn my_tzset ()
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* C 74-99's `#if NEED_MKTIME_WINDOWS` body - the `getenv`/`strchr`/`_putenv` dance that
     neutralizes a slash-bearing TZ - is dead: NEED_MKTIME_WINDOWS is 0.  */
  tzset ();

  }
}
/* C 104-105: `# undef tzset` then `# define tzset() my_tzset ()`.  The `tzset ()` the body
   above calls is the system function the time mirror declares (the macro is not yet
   defined at C 101, which is exactly why `my_tzset` wraps it).  */
macro_rules! tzset { () => { my_tzset () }; }

/* C 108's `#if defined _LIBC || NEED_MKTIME_WORKING || NEED_MKTIME_INTERNAL` is live.  */
/* <intprops.h> 43/73/88: the three `_Generic` tables, whose arms each reduce to the type's
   own limit for the types this file names (the nanosleep.rs TYPE_MAXIMUM precedent).  */
macro_rules! TYPE_SIGNED { ($t:ty) => { <$t>::MIN < 0 }; }
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }
/* PORTING.md 2.5: C's `static_assert` is `const _: () = core::assert! (cond);`.  */
macro_rules! static_assert { ($cond:expr) => { const _: () = core::assert! ($cond); }; }
/* <libc-config.h> 46: `# define __set_errno(val) (errno = (val))` is the
   `include/libc-config.rs` mirror's now (P-26, one landing point); the calls below keep C's
   spelling (the mirror's body parenthesises the value, this unit's local copy did not - the
   expansion is the same assignment either way).  */

/* C 124-128: `# if INT_MAX <= LONG_MAX / 4 / 366 / 24 / 60 / 60` holds on this target
   (a 64-bit `long`), so `long_int` is C's `long int`; the `#else` arm (`long long int`)
   is dead.  */
pub type long_int = c_long;
/* C 129.  */
static_assert! ((INT_MAX as long_int) <= TYPE_MAXIMUM! (long_int) / 4 / 366 / 24 / 60 / 60);

/* C 141-148.  */
unsafe fn shr (a: long_int, b: c_int) -> long_int
{
  let one: long_int = 1;
  return if -one >> 1 == -1
         { a >> b }
         else
           { (a + (a < 0) as long_int) / (one << b) - (a < 0) as long_int };
}

/* C 152-158: the bounds for the intersection of __time64_t and long_int.  */
static mktime_min: long_int =
  if TYPE_SIGNED! (__time64_t) && TYPE_MINIMUM! (__time64_t) < TYPE_MINIMUM! (long_int)
  { TYPE_MINIMUM! (long_int) } else { TYPE_MINIMUM! (__time64_t) };
static mktime_max: long_int =
  if TYPE_MAXIMUM! (long_int) < TYPE_MAXIMUM! (__time64_t)
  { TYPE_MAXIMUM! (long_int) } else { TYPE_MAXIMUM! (__time64_t) };

/* C 160-162.  */
pub const EPOCH_YEAR: c_int = 1970;
pub const TM_YEAR_BASE: c_int = 1900;
static_assert! (TM_YEAR_BASE % 100 == 0);

/* C 165-174.  */
unsafe fn leapyear (year: long_int) -> bool
{
  /* Don't add YEAR to TM_YEAR_BASE, as that might overflow.
     Also, work even if YEAR is negative.  */
  return (year & 3) == 0
         && (year % 100 != 0
             || ((year / 100) & 3) == ((- (TM_YEAR_BASE / 100) & 3) as long_int));
}

/* C 176-186.  */
/* How many days come before each month (0-12).  */
static __mon_yday: [[c_ushort; 13]; 2] =
  [
    /* Normal years.  */
    [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
    /* Leap years.  */
    [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366]
  ];

/* C 191-195.  */
unsafe fn isdst_differ (a: c_int, b: c_int) -> bool
{
  return (a == 0) != (b == 0) && (0 <= a) && (0 <= b);
}

/* C 207-230.  */
unsafe fn ydhms_diff (year1: long_int, yday1: long_int, hour1: c_int, min1: c_int,
                      sec1: c_int, year0: c_int, yday0: c_int, hour0: c_int, min0: c_int,
                      sec0: c_int) -> long_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  static_assert! (-1 / 2 == 0);

  /* Compute intervening leap days correctly even if year is negative.
     Take care to avoid integer overflow here.  */
  let a4: c_int = (shr (year1, 2) + shr (TM_YEAR_BASE as long_int, 2)
                   - ((year1 & 3) == 0) as long_int) as c_int;
  let b4: c_int = (shr (year0 as long_int, 2) + shr (TM_YEAR_BASE as long_int, 2)
                   - ((year0 & 3) == 0) as long_int) as c_int;
  let a100: c_int = (a4 + (a4 < 0) as c_int) / 25 - (a4 < 0) as c_int;
  let b100: c_int = (b4 + (b4 < 0) as c_int) / 25 - (b4 < 0) as c_int;
  let a400: c_int = shr (a100 as long_int, 2) as c_int;
  let b400: c_int = shr (b100 as long_int, 2) as c_int;
  let intervening_leap_days: c_int = (a4 - b4) - (a100 - b100) + (a400 - b400);

  /* Compute the desired time without overflowing.  */
  let years: long_int = year1 - year0 as long_int;
  let days: long_int = 365 * years + yday1 - yday0 as long_int
                       + intervening_leap_days as long_int;
  let hours: long_int = 24 * days + hour1 as long_int - hour0 as long_int;
  let minutes: long_int = 60 * hours + min1 as long_int - min0 as long_int;
  let seconds: long_int = 60 * minutes + sec1 as long_int - sec0 as long_int;
  return seconds;

  }
}

/* C 232-238.  */
unsafe fn long_int_avg (a: long_int, b: long_int) -> long_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  return shr (a, 1) + shr (b, 1) + ((a | b) & 1);

  }
}

/* C 240-254.  */
unsafe fn tm_diff (year: long_int, yday: long_int, hour: c_int, min: c_int, sec: c_int,
                   tp: *const tm) -> long_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  return ydhms_diff (year, yday, hour, min, sec,
                     (*tp).tm_year, (*tp).tm_yday,
                     (*tp).tm_hour, (*tp).tm_min, (*tp).tm_sec);

  }
}

/* C 256-270's `#ifndef _LIBC` arm is live.  */
unsafe fn convert_time (t: long_int, local: bool, tm: *mut tm) -> *mut tm
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let x: __time64_t = t;
  if local
    { return __localtime64_r! (&x, tm); }
  else
    { return __gmtime64_r! (&x, tm); }

  }
}
/* C 269: `# define __tz_convert convert_time`.  */
macro_rules! __tz_convert {
  ($t:expr, $local:expr, $tm:expr) => { convert_time ($t, $local, $tm) };
}

/* C 272-317.  */
unsafe fn ranged_convert (local: bool, t: *mut long_int, tp: *mut tm) -> *mut tm
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let t1: long_int = if *t < mktime_min
                     { mktime_min }
                     else if *t <= mktime_max { *t } else { mktime_max };
  let r: *mut tm = __tz_convert! (t1, local, tp);
  if ! r.is_null ()
    {
      *t = t1;
      return r;
    }
  if *errno () != EOVERFLOW
    { return std::ptr::null_mut (); }

  let mut bad: long_int = t1;
  let mut ok: long_int = 0;
  let mut oktm: tm = std::mem::zeroed ();   /* the C declaration, PORTING.md 1.5  */
  oktm.tm_sec = -1;

  /* BAD is a known out-of-range value, and OK is a known in-range one.
     Use binary search to narrow the range between BAD and OK until
     they differ by 1.  */
  while true
    {
      let mid: long_int = long_int_avg (ok, bad);
      if mid == ok || mid == bad
        { break; }
      if ! __tz_convert! (mid, local, tp).is_null ()
        {
          ok = mid;
          oktm = *tp;
        }
      else if *errno () != EOVERFLOW
        { return std::ptr::null_mut (); }
      else
        { bad = mid; }
    }

  if oktm.tm_sec < 0
    { return std::ptr::null_mut (); }
  *t = ok;
  *tp = oktm;
  return tp;

  }
}

/* C 319-531.  */
#[unsafe(export_name = "mktime_internal")]
pub unsafe extern "C" fn __mktime_internal (tp: *mut tm, local: bool,
                                            offset: *mut mktime_offset_t) -> __time64_t
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let mut tm: tm = std::mem::zeroed ();   /* the C declaration, PORTING.md 1.5  */

  /* The maximum number of probes should be enough to handle any
     combinations of time zone rule changes, solar time, leap seconds,
     and oscillations around a spring-forward gap.  POSIX.1 prohibits
     leap seconds, but some hosts have them anyway.  */
  let mut remaining_probes: c_int = 6;

  /* C 341-347's `#ifndef _LIBC` arm is live.  */
  /* Gnulib mktime doesn't lock the tz state, so it may need to probe
     more often if some other thread changes local time while
     __mktime_internal is probing.  Double the number of probes; this
     should suffice for practical cases that are at all likely.  */
  remaining_probes *= 2;

  /* Time requested.  Copy it in case gmtime/localtime modify *TP;
     this can occur if TP is localtime's returned value and CONVERT is
     localtime.  */
  let mut sec: c_int = (*tp).tm_sec;
  let min: c_int = (*tp).tm_min;
  let hour: c_int = (*tp).tm_hour;
  let mday: c_int = (*tp).tm_mday;
  let mon: c_int = (*tp).tm_mon;
  let year_requested: c_int = (*tp).tm_year;
  let isdst: c_int = (*tp).tm_isdst;

  /* True if the previous probe was DST.  */
  let mut dst2: bool = false;

  /* Ensure that mon is in range, and set year accordingly.  */
  let mon_remainder: c_int = mon % 12;
  let negative_mon_remainder: c_int = (mon_remainder < 0) as c_int;
  let mon_years: c_int = mon / 12 - negative_mon_remainder;
  let lyear_requested: long_int = year_requested as long_int;
  let year: long_int = lyear_requested + mon_years as long_int;

  /* The other values need not be in range:
     the remaining code handles overflows correctly.  */

  /* Calculate day of year from year, month, and day of month.
     The result need not be in range.  */
  let mon_yday: c_int =
    __mon_yday [leapyear (year) as usize]
               [(mon_remainder + 12 * negative_mon_remainder) as usize] as c_int - 1;
  let lmday: long_int = mday as long_int;
  let yday: long_int = mon_yday as long_int + lmday;

  let off: mktime_offset_t = *offset;
  let mut negative_offset_guess: c_int = 0;

  let sec_requested: c_int = sec;

  if LEAP_SECONDS_POSSIBLE != 0
    {
      /* Handle out-of-range seconds specially,
         since ydhms_diff assumes every minute has 60 seconds.  */
      if sec < 0
        { sec = 0; }
      if 59 < sec
        { sec = 59; }
    }

  /* Invert CONVERT by probing.  First assume the same offset as last
     time.  */

  /* C 399's `ckd_sub (&negative_offset_guess, 0, off)` stores the wrapped difference and
     its overflow result is discarded; the port spells the same three arguments as the
     associated function.  */
  let (sub, _overflowed) = mktime_offset_t::overflowing_sub (0, off);
  negative_offset_guess = sub as c_int;
  let t0: long_int = ydhms_diff (year, yday, hour, min, sec,
                                 EPOCH_YEAR - TM_YEAR_BASE, 0, 0, 0,
                                 negative_offset_guess);
  let mut t: long_int = t0;
  let mut t1: long_int = t0;
  let mut t2: long_int = t0;

  /* Repeatedly use the error to improve the guess.  */

  'offset_found: {
  while true
    {
      if ranged_convert (local, &mut t, &mut tm).is_null ()
        { return -1; }
      let dt: long_int = tm_diff (year, yday, hour, min, sec, &tm);
      if dt == 0
        { break; }

      if t == t1 && t != t2
         && (tm.tm_isdst < 0
             || (if isdst < 0
                 { dst2 as c_int <= (tm.tm_isdst != 0) as c_int }
                 else
                 { (isdst != 0) != (tm.tm_isdst != 0) }))
        /* We can't possibly find a match, as we are oscillating
           between two values.  The requested time probably falls
           within a spring-forward gap of size DT.  Follow the common
           practice in this case, which is to return a time that is DT
           away from the requested time, preferring a time whose
           tm_isdst differs from the requested value.  (If no tm_isdst
           was requested and only one of the two values has a nonzero
           tm_isdst, prefer that value.)  In practice, this is more
           useful than returning -1.  */
        { break 'offset_found; }

      remaining_probes -= 1;
      if remaining_probes == 0
        {
          __set_errno! (EOVERFLOW);
          return -1;
        }

      t1 = t2;
      t2 = t;
      t += dt;
      dst2 = tm.tm_isdst != 0;
    }

  /* We have a match.  Check whether tm.tm_isdst has the requested
     value, if any.  */
  if isdst_differ (isdst, tm.tm_isdst)
    {
      /* tm.tm_isdst has the wrong value.  Look for a neighboring
         time with the right value, and use its UTC offset.

         Heuristic: probe the adjacent timestamps in both directions,
         looking for the desired isdst.  If none is found within a
         reasonable duration bound, ignore the disagreement.
         This should work for all real time zone histories in the tz
         database.  */

      /* Distance between probes when looking for a DST boundary.  In
         tzdata2003a, the shortest period of DST is 601200 seconds
         (e.g., America/Recife starting 2000-10-08 01:00), and the
         shortest period of non-DST surrounded by DST is 694800
         seconds (Africa/Tunis starting 1943-04-17 01:00).  Use the
         minimum of these two values, so we don't miss these short
         periods when probing.  */
      let stride: c_int = 601200;

      /* Do not probe too far away from the requested time,
         by striding until at least a year has passed, but then giving up.
         This helps avoid unexpected results in (for example) Asia/Kolkata,
         for which today's users expect to see no DST even though it
         did observe DST long ago.  */
      let year_seconds_bound: c_int = 366 * 24 * 60 * 60 + 1;
      let delta_bound: c_int = year_seconds_bound + stride;

      /* Search in both directions, closest first.  */
      let mut delta: c_int = stride;
      while delta < delta_bound
        {
          let mut direction: c_int = -1;
          while direction <= 1
            {
              /* C 476's `ckd_add (&ot, t, delta * direction)` stores the wrapped sum in
                 OT and the test is its overflow result.  */
              let (mut ot, ot_overflowed) =
                t.overflowing_add ((delta * direction) as long_int);
              if ! ot_overflowed
                {
                  let mut otm: tm = std::mem::zeroed ();   /* the C declaration  */
                  if ! ranged_convert (local, &mut ot, &mut otm).is_null ()
                    {
                      if ! isdst_differ (isdst, otm.tm_isdst)
                        {
                          /* We found the desired tm_isdst.
                             Extrapolate back to the desired time.  */
                          let gt: long_int = ot + tm_diff (year, yday, hour, min, sec,
                                                           &otm);
                          if mktime_min <= gt && gt <= mktime_max
                            {
                              if ! __tz_convert! (gt, local, &mut tm).is_null ()
                                {
                                  t = gt;
                                  break 'offset_found;
                                }
                              if *errno () != EOVERFLOW
                                { return -1; }
                            }
                        }
                    }
                }
              direction += 2;
            }
          delta += stride;
        }

      /* No probe with the requested tm_isdst was found nearby.
         Ignore the requested tm_isdst.  */
    }
  }

  /* C 505: `offset_found:`.  */
  /* Set *OFFSET to the low-order bits of T - T0 - NEGATIVE_OFFSET_GUESS.
     This is just a heuristic to speed up the next mktime call, and
     correctness is unaffected if integer overflow occurs here.  */
  let (sub1, _overflowed1) = long_int::overflowing_sub (t, t0);
  *offset = sub1;
  let (sub2, _overflowed2) = mktime_offset_t::overflowing_sub (*offset,
                                                              negative_offset_guess
                                                              as mktime_offset_t);
  *offset = sub2;

  if LEAP_SECONDS_POSSIBLE != 0 && sec_requested != tm.tm_sec
    {
      /* Adjust time to reflect the tm_sec requested, not the normalized value.
         Also, repair any damage from a false match due to a leap second.  */
      let mut sec_adjustment: long_int = (sec == 0 && tm.tm_sec == 60) as long_int;
      sec_adjustment -= sec as long_int;
      sec_adjustment += sec_requested as long_int;
      let (t_add, t_overflowed) = t.overflowing_add (sec_adjustment);
      t = t_add;
      if t_overflowed || ! (mktime_min <= t && t <= mktime_max)
        {
          __set_errno! (EOVERFLOW);
          return -1;
        }
      if __tz_convert! (t, local, &mut tm).is_null ()
        { return -1; }
    }

  *tp = tm;
  return t;

  }
}

/* C 533's `#endif` closes the first live guard.  */
/* C 535-555: `#if defined _LIBC || NEED_MKTIME_WORKING || NEED_MKTIME_WINDOWS` is live.  */

/* Convert *TP to a __time64_t value.  */
#[unsafe(export_name = "rpl_mktime")]
pub unsafe extern "C" fn __mktime64 (tp: *mut tm) -> __time64_t
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  __libc_lock_lock! (__tzset_lock);
  __tzset_unlocked! ();

  /* C 544-550: `#if defined _LIBC || NEED_MKTIME_WORKING` is live (the `#else` arm's
     `mktime (tp)` belongs to the NEED_MKTIME_WINDOWS-only build).  */
  static localtime_offset: VmGlobal<mktime_offset_t> = VmGlobal::<mktime_offset_t>::new (0);
  let result: __time64_t = __mktime_internal (tp, true, localtime_offset.as_mut_ptr ());

  __libc_lock_unlock! (__tzset_lock);
  return result;

  }
}
/* C 557-581's `#if defined _LIBC && __TIMESIZE != 64` arm (the 32-bit `time_t` wrapper and
   its `libc_hidden_def`/`weak_alias`/`libc_hidden_weak` lines) is dead: `_LIBC` is
   undefined.  `weak_alias (mktime, timelocal)` (C 580) is a glibc attribute that gnulib's
   libc-config.h defines to nothing.  */
