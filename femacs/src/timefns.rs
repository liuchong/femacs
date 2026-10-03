// femacs: port of src/timefns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, union, enum, enum member, macro, local), same control flow, same error model.
// The fourteen `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's non-`static` definitions -
// `make_timeval`, `monotonic_coarse_timespec`, `init_timefns`, `make_lisp_time`,
// `timespec_to_lisp`, `float_time`, `list4_to_timespec`, `lisp_time_argument`,
// `emacs_getenv_TZ`, `emacs_setenv_TZ`, `syms_of_timefns` - carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn` (the ones that can
// signal are `-> !`, C's `AVOID`).
//
// **Carried arms.**  `src/timefns.c` is *not* in `docs/baseline/config/cond-eval-verified.txt`
// and `python3 tools/check-cond-eval.py --show-unknown src/timefns.c` reports
// `30 conditions, 7 unknown, 4 disagreements [NOT VERIFIED]`: the frozen configuration's
// macro table does not decide `FIXNUM_OVERFLOW_P (…)`/`TRIILLION <= ULONG_MAX` (a
// function-like macro that is not in the frozen `-dM` dump at all), so
// `tools/extract-abi.py`'s `blind_conditions` keeps *both* arms of every `#if` live and the
// per-function token check demands the tokens of C's text on both sides.  Per the
// integrator's 2026-10-03 decision this file therefore *carries* the text of the arms the
// frozen build does not compile, each under a `#[cfg (feature = "…")]` that names the C
// condition's non-frozen side (PORTING.md P-51; `femacs/lib/boot-time.rs` is the
// precedent).  None of these features is ever set: `xtask dead-arms` can only flip the keys
// `config.rs` carries and these are the compiler/preprocessor predicates of this file
// (docs/status.md 3.9), so the compiled build is exactly what clang compiles.  The arms:
//
//   * C 74-78 / 81-85: the `#else` objects `static Lisp_Object timespec_hz` / `trillion`
//     (features `no_fixnum_timespec_hz` / `no_fixnum_trillion`); the live arms are the two
//     object-like macros of C 75 / 82.
//   * C 86-93: `! (FASTER_TIMEFNS && TRILLION <= ULONG_MAX)` is false in the frozen build,
//     so clang compiles nothing there; both inner arms are carried (features
//     `ztrillion_bignum_val` for C 87-88 and `ztrillion_mpz` for C 89-91, the latter also
//     carrying `NEED_ZTRILLION_INIT`, `syms_of_timefns_for_pdumper` and the
//     `pdumper_do_now_and_after_load` call, C 2087-2094 / 2141-2143).
//   * C 148-154: the `CLOCK_MONOTONIC_COARSE` arm (feature `clock_monotonic_coarse`).
//   * C 47-52 (`HAVE_TIMEZONE_T`), C 202-211 and 328-330 (`WINDOWSNT`), C 1997-2001 and
//     2015-2084 (`MSDOS`): the frozen target compiles none of them (features
//     `have_timezone_t`, `windowsnt`, `msdos`).
//   * C 667-671 / 676-684 / 915-923: the `ztrillion` arms of the `TRILLION <= ULONG_MAX`
//     shortcut (feature `no_trillion_shortcut`).
//   * C 59-61 (`#ifndef HAVE_TM_GMTOFF`), C 2099-2102 (`#ifndef timespec_hz`) and
//     2103-2106 (`#ifndef trillion`): dead on the frozen target because `<config.h>` already
//     defines `HAVE_TM_GMTOFF 1` / the object macros are the live arm; carried under the
//     matching feature (the `HAVE_TM_GMTOFF` row is the ledger residue the integrator
//     authorized: C's own `#define` value is `false`, the frozen value is `1`).
//
// The `#include` list is C 20-41's through the mirrors those pull in (0.1-14's order:
// config, then lisp, then the rest).  C 27's `systime.h` is the mirror that declares the
// seven functions this unit defines (it gained the module + glob import that lets this unit
// define them without E0428 - see that mirror's note).  Names no mirror carries are declared
// at their use site (the terminal.rs/filelock.rs shape): `tzalloc`, `tzfree`,
// `clock_gettime`, `clock`, `synchronize_system_time_locale`, `xputenv`, `PRIdMAX`, `EDOM`.
//
// `#if` classification (PORTING.md 7.2): every condition here is a *constant* condition of
// the frozen configuration, except the `#include` selections (C 47/202/2051), which the
// port records in the ledger.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_parens, arithmetic_overflow)]

include!("include/config.rs");                 // C 20: `#include <config.h>`
include!("../lib/include/stdio.rs");           // C 40: `#include <stdio.h>`
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
include!("../lib/include/stdlib.rs");          // C 41: `#include <stdlib.h>`
include!("../lib/include/math.rs");            // C 39: `#include <math.h>`
include!("../lib/include/limits.in.rs");       // C 38: `#include <limits.h>`
include!("../lib/include/errno.rs");           // C 37: `#include <errno.h>`
include!("../lib/include/strftime.rs");        // C 35: `#include <strftime.h>`
include!("include/lisp.rs");                   // C 32: `#include "lisp.h"`
include!("../lib/include/mini-gmp.rs");        // bignum.h's `#include <gmp.h>`
include!("include/bignum.rs");                 // C 30: `#include "bignum.h"`
include!("include/composite.rs");              // coding.h 299-310 names its types
include!("include/coding.rs");                 // C 31: `#include "coding.h"`
include!("include/blockinput.rs");             // C 29: `#include "blockinput.h"`
include!("include/pdumper.rs");                // C 33: `#include "pdumper.h"`
include!("include/systime.rs");                // C 27: `#include "systime.h"`

/* C 23-25's `#if 10 <= __GNUC__` / `# pragma GCC diagnostic ignored
   "-Wanalyzer-null-dereference"` works around GCC bug 102671; clang's `__GNUC__` is 4, so
   the frozen build compiles the block dead, and a Rust port has no pragma (the
   floatfns.rs `__FINITE_MATH_ONLY__` precedent).  Recorded in docs/ports/timefns.md.  */

/* C 43-45's `#ifdef WINDOWSNT` declaration arm.  `WINDOWSNT` is undefined in the frozen
   configuration, so the arm is carried (P-51).  */
#[cfg (feature = "windowsnt")]
unsafe extern "C" { fn sys_clock () -> clock_t; }

/// <time.h> on the frozen target: `clock_t` is `unsigned long` (measured with clang:
/// `sizeof (clock_t) == 8`, `(clock_t) -1 < 0` is false), and `CLOCKS_PER_SEC` is
/// `((clock_t) 1000000)` (docs/baseline/config/darwin-arm64.macros.tsv 626).
type clock_t = c_ulong;
const CLOCKS_PER_SEC: clock_t = 1000000;

/// <inttypes.h>: `#define PRIdMAX "jd"` on the frozen target (`intmax_t` is `long long`;
/// measured with clang, the filelock.rs shape).
const PRIdMAX: &str = "jd";

/// <errno.h>: `#define EDOM 33` on the frozen target; `errno.rs` carries the other four
/// the file names.
const EDOM: c_int = 33;

/// <time.h>: the frozen target's `CLOCK_MONOTONIC` (6, measured with clang).
const CLOCK_MONOTONIC: c_int = 6;
/// `CLOCK_MONOTONIC_COARSE` does not exist on darwin; the port keeps the dead arm's name
/// with the Linux value (4) so its text exists (the arm is never compiled, P-51).
#[cfg (feature = "clock_monotonic_coarse")]
const CLOCK_MONOTONIC_COARSE: c_int = 4;

/* Declared at their use site; the mirrors this unit includes do not carry them.  */
unsafe extern "C" {
  /* char *tzalloc (char const *);  (time.in.h 512; gnulib's time_rz module)  */
  fn tzalloc (name: *const c_char) -> timezone_t;
  /* void tzfree (timezone_t);  (time.in.h 520)  */
  fn tzfree (tz: timezone_t);
  /* int clock_gettime (clockid_t, struct timespec *);  */
  fn clock_gettime (clockid: c_int, tp: *mut timespec) -> c_int;
  /* clock_t clock (void);  */
  fn clock () -> clock_t;
  /* void synchronize_system_time_locale (void);  (lisp.h 5143)  */
  fn synchronize_system_time_locale ();
  /* void xputenv (char const *);  (lisp.h 5579)  */
  fn xputenv (envvar: *const c_char);
}

/// C: gnulib intprops.h's `INT_STRLEN_BOUND (t)` - `((TYPE_WIDTH (t) - TYPE_SIGNED (t)) *
/// 146 + 484) / 485 + TYPE_SIGNED (t)` (the openat-proc.rs shape; the one use here is
/// `INT_STRLEN_BOUND (EMACS_INT)` = (63 * 146 + 484) / 485 + 1 = 20).
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => {
    ((std::mem::size_of::<$t> () * 8 - 1) * 146 + 484) / 485 + 1
  };
}

/// C: gnulib intprops.h's `TYPE_SIGNED (t)` (the mktime.rs/regcomp.rs shape): true when the
/// type is signed.  `time_t` is `i64` on the frozen target, so both uses are true.
macro_rules! TYPE_SIGNED {
  ($t:ty) => { <$t>::MIN < 0 };
}

/// C: gnulib intprops.h's `EXPR_SIGNED (e)`: the type of the expression, not its value, is
/// what it asks.  The one call site (`mpz_set_time`) passes a `time_t`, which is signed on
/// the frozen target; the `&` keeps the argument unevaluated exactly as C's does.
macro_rules! EXPR_SIGNED {
  ($e:expr) => { { let _ = &$e; true } };
}

/* C 59-61's `#ifndef HAVE_TM_GMTOFF / # define HAVE_TM_GMTOFF false / #endif` is dead on
   the frozen target (`<config.h>` already defines `HAVE_TM_GMTOFF 1`), but the evaluator is
   blind for this file and records C's own value, so the dead spelling is carried first (the
   ledger residue the integrator authorized; the extractor's `constants_of` reads the first
   occurrence).  The compiled arm is the frozen value.  */
#[cfg (feature = "no_have_tm_gmtoff")]
pub const HAVE_TM_GMTOFF: bool = false;
#[cfg (not (feature = "no_have_tm_gmtoff"))]
pub const HAVE_TM_GMTOFF: bool = true;

/* C 47-52: `#ifdef HAVE_TIMEZONE_T` (undefined) / `#include <sys/param.h>` / the NetBSD
   `HAVE_TZALLOC_BUG` arm.  The whole block is dead; the include has no mirror and the inner
   condition never holds (`__NetBSD_Version__` is undefined), so the port records it.  */
/* C 53-55: `#ifndef HAVE_TZALLOC_BUG / # define HAVE_TZALLOC_BUG false / #endif`.  */
pub const HAVE_TZALLOC_BUG: bool = false;

/* C 65-68: `#ifndef FASTER_TIMEFNS / # define FASTER_TIMEFNS 1 / #endif`.  */
pub const FASTER_TIMEFNS: c_int = 1;

/* C 70-73: `#ifndef CURRENT_TIME_LIST / enum { CURRENT_TIME_LIST = false }; / #endif`.  */
pub const CURRENT_TIME_LIST: bool = false;

/* C 57: `enum { TM_YEAR_BASE = 1900 };`.  */
pub const TM_YEAR_BASE: c_int = 1900;

/* C 74-78: the live arm is C 75's object-like macro; the `#else` object is carried.  */
#[cfg (feature = "no_fixnum_timespec_hz")]
static mut timespec_hz: Lisp_Object = lisp_h_Qnil ();
#[cfg (not (feature = "no_fixnum_timespec_hz"))]
macro_rules! timespec_hz { () => { make_fixnum (TIMESPEC_HZ as EMACS_INT) } }

/* C 80-85: `#define TRILLION 1000000000000`, then the live macro of C 82 / the carried
   `#else` object of C 84.  */
pub const TRILLION: i64 = 1000000000000;
#[cfg (feature = "no_fixnum_trillion")]
static mut trillion: Lisp_Object = lisp_h_Qnil ();
#[cfg (not (feature = "no_fixnum_trillion"))]
macro_rules! trillion { () => { make_fixnum (TRILLION as EMACS_INT) } }

/* C 86-93: `#if ! (FASTER_TIMEFNS && TRILLION <= ULONG_MAX)` is false in the frozen
   configuration, so clang compiles nothing here; the evaluator keeps the block live and the
   token check wants both inner arms, which are carried (P-51).  */
#[cfg (feature = "ztrillion_bignum_val")]
macro_rules! ztrillion { () => { xbignum_val (trillion! ()) } }
#[cfg (feature = "ztrillion_mpz")]
static mut ztrillion: mpz_t =
  [__mpz_struct { _mp_alloc: 0, _mp_size: 0, _mp_d: std::ptr::null_mut () }];
#[cfg (feature = "ztrillion_mpz")]
macro_rules! NEED_ZTRILLION_INIT { () => {} }

/* True if the nonzero Lisp integer HZ divides evenly into a trillion.  */
unsafe fn trillion_factor (hz: Lisp_Object) -> bool
{
  if FASTER_TIMEFNS != 0
    {
      if FIXNUMP (hz)
	{ return TRILLION % (XFIXNUM (hz) as i64) == 0; }
      if !FIXNUM_OVERFLOW_P (TRILLION as EMACS_INT)
	{ return false; }
    }
  static_assert! (TRILLION <= INTMAX_MAX);
  let mut ihz: intmax_t = 0;
  return integer_to_intmax (hz, &mut ihz) && TRILLION % ihz == 0;
}

/* Return a struct timeval that is roughly equivalent to T.
   Use the least timeval not less than T.
   Return an extremal value if the result would overflow.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_timeval (t: timespec) -> timeval
{
  let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
  tv.tv_sec = t.tv_sec;
  tv.tv_usec = (t.tv_nsec / 1000) as suseconds_t;

  if t.tv_nsec % 1000 != 0
    {
      if tv.tv_usec < 999999
	{ tv.tv_usec += 1; }
      else
	{
	  let mut s1: time_t = 0;
	  // C's `ckd_add (&s1, tv.tv_sec, 1)`: the wrapped sum and the overflow flag.
	  let (ckd_add_value, ckd_add_overflow) = tv.tv_sec.overflowing_add (1);
	  s1 = ckd_add_value;
	  if !ckd_add_overflow
	    {
	      tv.tv_sec = s1;
	      tv.tv_usec = 0;
	    }
	}
    }

  return tv;
}

/* Return the current time with an epoch specific to this Emacs instance
   (e.g., system boot).  The clock should be unaffected by changes to
   the system time, and should be cheap to access.  Its resolution
   should be appropriate for human time scales, e.g., better than 10 ms.
   Make do with realtime if such a clock is not available.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn monotonic_coarse_timespec () -> timespec
{
  let mut ts: timespec = std::mem::zeroed ();
  /* C 148-154: `#ifdef CLOCK_MONOTONIC_COARSE` is the Linux arm (dead here); the frozen
     target takes `#elif defined CLOCK_MONOTONIC`.  Both are carried (P-51).  */
  #[cfg (feature = "clock_monotonic_coarse")]
  if clock_gettime (CLOCK_MONOTONIC_COARSE, &mut ts) == 0
    { return ts; }
  #[cfg (not (feature = "clock_monotonic_coarse"))]
  if clock_gettime (CLOCK_MONOTONIC, &mut ts) == 0
    { return ts; }
  ts = current_timespec ();
  return ts;
}

/* Yield A's UTC offset, or an unspecified value if unknown.  */
unsafe fn tm_gmtoff (a: *mut tm) -> c_long
{
  /* C 163-168: the frozen `<config.h>` defines `HAVE_TM_GMTOFF` as 1; the `#else` arm is
     carried.  */
  #[cfg (not (feature = "no_have_tm_gmtoff"))]
  { return (*a).tm_gmtoff; }
  #[cfg (feature = "no_have_tm_gmtoff")]
  { return 0; }
}

/* Yield A - B, measured in seconds.
   This function is copied from the GNU C Library.  */
unsafe fn tm_diff (a: *mut tm, b: *mut tm) -> c_int
{
  /* Compute intervening leap days correctly even if year is negative.
     Take care to avoid int overflow in leap day calculations,
     but it's OK to assume that A and B are close to each other.  */
  let a4: c_int = ((*a).tm_year >> 2) + (TM_YEAR_BASE >> 2) - (! ((*a).tm_year & 3) as c_int);
  let b4: c_int = ((*b).tm_year >> 2) + (TM_YEAR_BASE >> 2) - (! ((*b).tm_year & 3) as c_int);
  let a100: c_int = a4 / 25 - ((a4 % 25 < 0) as c_int);
  let b100: c_int = b4 / 25 - ((b4 % 25 < 0) as c_int);
  let a400: c_int = a100 >> 2;
  let b400: c_int = b100 >> 2;
  let intervening_leap_days: c_int = (a4 - b4) - (a100 - b100) + (a400 - b400);
  let years: c_int = (*a).tm_year - (*b).tm_year;
  let days: c_int = (365 * years + intervening_leap_days
	      + ((*a).tm_yday - (*b).tm_yday));
  return (60 * (60 * (24 * days + ((*a).tm_hour - (*b).tm_hour))
		+ ((*a).tm_min - (*b).tm_min))
	  + ((*a).tm_sec - (*b).tm_sec));
}

/* C 193: `enum { tzeqlen = sizeof "TZ=" - 1 };` (`sizeof` is skipped by the token check).  */
pub const tzeqlen: c_int = (std::mem::size_of_val (&*b"TZ=") - 1) as c_int;

/* Time zones equivalent to current local time and to UTC, respectively.  */
static mut local_tz: timezone_t = std::ptr::null_mut ();
/* C: `static timezone_t const utc_tz = 0;` — the port's raw pointer cannot be a non-`mut`
   static (raw pointers are not `Sync`); the cell is never written, so the `mut` is Rust's
   spelling only.  The ledger does not list this object (cinventory omits `const`-initialised
   file-scope objects).  */
static mut utc_tz: timezone_t = std::ptr::null_mut ();

unsafe fn emacs_localtime_rz (tz: timezone_t, t: *const time_t, tm: *mut tm) -> *mut tm
{
  /* C 202-205's `#ifdef WINDOWSNT` arm (`tzset ()` / `w32_fix_tzset ()`), carried.  */
  #[cfg (feature = "windowsnt")]
  { tzset (); w32_fix_tzset (); }
  let tm = localtime_rz (tz, t, tm);
  if tm.is_null () && *errno () == ENOMEM
    { memory_full_up (); }
  return tm;
}

unsafe fn invalid_time_zone_specification (zone: Lisp_Object) -> !
{
  xsignal2 (Qerror (), build_string (c"Invalid time zone specification".as_ptr ()), zone);
}

/* Free a timezone, except do not free the time zone for local time.
   Freeing utc_tz is also a no-op.  */
unsafe fn xtzfree (tz: timezone_t)
{
  if tz != local_tz
    { tzfree (tz); }
}

/* Convert the Lisp time zone rule ZONE to a timezone_t object.
   The returned value either is 0, or is LOCAL_TZ, or is newly allocated.
   If SETTZ, set Emacs local time to the time zone rule; otherwise,
   the caller should eventually pass the returned value to xtzfree.  */
unsafe fn tzlookup (mut zone: Lisp_Object, settz: bool) -> timezone_t
{
  const tzbuf_format: [u8; 24] = *b"<%+.*ld>%s%ld:%02d:%02d\0";
  let trailing_tzbuf_format: *const c_char =
    (tzbuf_format.as_ptr () as *const c_char).add (std::mem::size_of_val (&*b"<%+.*ld") - 1);
  const TZBUF_LEN: usize = std::mem::size_of_val (&tzbuf_format)
    + 2 * INT_STRLEN_BOUND! (EMACS_INT);
  let mut tzbuf: [c_char; TZBUF_LEN] = [0; TZBUF_LEN];
  let mut zone_string: *const c_char;
  let mut new_tz: timezone_t;

  if NILP (zone)
    { return local_tz; }
  else if BASE_EQ (zone, make_fixnum (0)) || EQ (zone, Qt ())
    {
      zone_string = c"UTC0".as_ptr ();
      new_tz = utc_tz;
    }
  else
    {
      let plain_integer: bool = FIXNUMP (zone);

      if EQ (zone, Qwall ())
	{ zone_string = std::ptr::null (); }
      else if STRINGP (zone)
	{ zone_string = SSDATA (ENCODE_SYSTEM (zone)) as *const c_char; }
      else if plain_integer || (CONSP (zone) && FIXNUMP (XCAR (zone))
				 && CONSP (XCDR (zone)))
	{
	  let mut abbr: Lisp_Object = lisp_h_Qnil ();   /* C's `UNINIT`: assigned below.  */
	  if !plain_integer
	    {
	      abbr = XCAR (XCDR (zone));
	      zone = XCAR (zone);
	    }

	  let abszone: EMACS_INT = eabs! (XFIXNUM (zone));
	  let hour: EMACS_INT = abszone / (60 * 60);
	  let hour_remainder: c_int = (abszone % (60 * 60)) as c_int;
	  let min: c_int = hour_remainder / 60;
	  let sec: c_int = hour_remainder % 60;

	  if plain_integer
	    {
	      let mut prec: c_int = 2;
	      let mut numzone: EMACS_INT = hour;
	      if hour_remainder != 0
		{
		  prec += 2;
		  numzone = 100 * numzone + min as EMACS_INT;
		  if sec != 0
		    {
		      prec += 2;
		      numzone = 100 * numzone + sec as EMACS_INT;
		    }
		}
	      static MINUS: [u8; 2] = *b"-\0";
	      let sign: *const c_char =
		&MINUS[(XFIXNUM (zone) < 0) as usize] as *const u8 as *const c_char;
	      sprintf (tzbuf.as_mut_ptr (), tzbuf_format.as_ptr () as *const c_char, prec,
		       (if XFIXNUM (zone) < 0 { -numzone } else { numzone }) as c_long,
		       sign, hour as c_int, min, sec);
	      zone_string = tzbuf.as_ptr ();
	    }
	  else
	    {
	      AUTO_STRING! (leading, c"<".as_ptr ());
	      static MINUS2: [u8; 2] = *b"-\0";
	      let sign2: *const c_char =
		&MINUS2[(XFIXNUM (zone) < 0) as usize] as *const u8 as *const c_char;
	      let trailing_len = sprintf (tzbuf.as_mut_ptr (), trailing_tzbuf_format,
					  sign2, hour as c_int, min, sec);
	      AUTO_STRING_WITH_LEN! (trailing, tzbuf.as_ptr (), trailing_len as isize);
	      zone_string = SSDATA (concat3 (leading, ENCODE_SYSTEM (abbr),
					     trailing));
	    }
	}
      else
	{ invalid_time_zone_specification (zone); }

      new_tz = tzalloc (zone_string);

      if HAVE_TZALLOC_BUG && new_tz.is_null () && *errno () != ENOMEM && plain_integer
	  && XFIXNUM (zone) % (60 * 60) == 0
	{
	  /* tzalloc mishandles POSIX strings; fall back on tzdb if
	     possible (Bug#30738).  */
	  sprintf (tzbuf.as_mut_ptr (), c"Etc/GMT%+ld".as_ptr (),
		   - (XFIXNUM (zone) / (60 * 60)));
	  new_tz = tzalloc (zone_string);
	}

      if new_tz.is_null ()
	{
	  if *errno () == ENOMEM
	    { memory_full_up (); }
	  invalid_time_zone_specification (zone);
	}
    }

  if settz
    {
      block_input ();
      emacs_setenv_TZ (zone_string);
      tzset ();
      /* C 328-330's `#ifdef WINDOWSNT` arm, carried.  */
      #[cfg (feature = "windowsnt")]
      { w32_fix_tzset (); }
      let old_tz: timezone_t = local_tz;
      local_tz = new_tz;
      tzfree (old_tz);
      unblock_input ();
    }

  return new_tz;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_timefns ()
{
  let mut tz: *mut c_char = getenv (c"TZ".as_ptr ());

  /* Set the time zone rule now, so that the call to putenv is done
     before multiple threads are active.  */
  tzlookup (if !tz.is_null () { build_string (tz) } else { Qwall () }, true);
}

/* Report that a time value is out of range for Emacs.  */
unsafe fn time_overflow () -> !
{
  error (c"Specified time is not representable".as_ptr ());
}

unsafe fn time_spec_invalid () -> !
{
  error (c"Invalid time specification".as_ptr ());
}

unsafe fn time_error (err: c_int) -> !
{
  match err
    {
      ENOMEM => memory_full_up (),
      EOVERFLOW => time_overflow (),
      _ => time_spec_invalid (),
    }
    // C's `switch` falls through from ENOMEM and EOVERFLOW into the default; the arms are
    // `noreturn`, so the port calls them (PORTING.md 2.2).
}

unsafe fn invalid_hz (hz: Lisp_Object) -> !
{
  xsignal2 (Qerror (), build_string (c"Invalid time frequency".as_ptr ()), hz);
}

/* Return the upper part of the time T (everything but the bottom 16 bits).  */
unsafe fn hi_time (t: time_t) -> Lisp_Object
{
  return INT_TO_INTEGER ((t >> LO_TIME_BITS) as EMACS_INT);
}

/* Return the bottom bits of the time T.  */
unsafe fn lo_time (t: time_t) -> Lisp_Object
{
  return make_fixnum ((t & ((1 << LO_TIME_BITS) - 1)) as EMACS_INT);
}

/* When converting a double to a fraction TICKS / HZ, HZ is equal to
   FLT_RADIX * P where 0 <= P < FLT_RADIX_POWER_SIZE.  The tiniest
   nonzero double uses the maximum P.  */
pub const flt_radix_power_size: c_int = DBL_MANT_DIG - DBL_MIN_EXP + 1;

/* A integer vector of size flt_radix_power_size.  The Pth entry
   equals FLT_RADIX**P.  */
static mut flt_radix_power: Lisp_Object = lisp_h_Qnil ();

/* Return NUMERATOR / DENOMINATOR, rounded to the nearest double.
   Arguments must be Lisp integers, and DENOMINATOR must be positive.  */
unsafe fn frac_to_double (numerator: Lisp_Object, denominator: Lisp_Object) -> c_double
{
  let mut intmax_numerator: intmax_t = 0;
  let mut intmax_denominator: intmax_t = 0;
  if FASTER_TIMEFNS != 0
      && integer_to_intmax (numerator, &mut intmax_numerator)
      && integer_to_intmax (denominator, &mut intmax_denominator)
      && intmax_numerator % intmax_denominator == 0
    { return (intmax_numerator / intmax_denominator) as c_double; }

  /* Compute number of base-FLT_RADIX digits in numerator and denominator.  */
  let mut n: *const __mpz_struct = bignum_integer (&raw mut mpz[0], numerator);
  let mut d: *const __mpz_struct = bignum_integer (&raw mut mpz[1], denominator);
  let ndig: ptrdiff_t = mpz_sizeinbase (n, FLT_RADIX) as ptrdiff_t;
  let ddig: ptrdiff_t = mpz_sizeinbase (d, FLT_RADIX) as ptrdiff_t;

  /* Scale with SCALE when doing integer division.  That is, compute
     (N * FLT_RADIX**SCALE) / D [or, if SCALE is negative, N / (D *
     FLT_RADIX**-SCALE)] as a bignum, convert the bignum to double,
     then divide the double by FLT_RADIX**SCALE.  First scale N
     (or scale D, if SCALE is negative) ...  */
  let mut scale: ptrdiff_t = ddig - ndig + DBL_MANT_DIG as ptrdiff_t;
  if scale < 0
    {
      mpz_mul_2exp (mpz[1].as_mut_ptr (), d, (- (scale * LOG2_FLT_RADIX as ptrdiff_t)) as mp_bitcnt_t);
      d = mpz[1].as_ptr ();
    }
  else
    {
      /* min so we don't scale tiny numbers as if they were normalized.  */
      scale = min (scale, (flt_radix_power_size - 1) as ptrdiff_t);

      mpz_mul_2exp (mpz[0].as_mut_ptr (), n, (scale * LOG2_FLT_RADIX as ptrdiff_t) as mp_bitcnt_t);
      n = mpz[0].as_ptr ();
    }
  /* ... and then divide, with quotient Q and remainder R.  */
  let q: *mut mpz_t = &raw mut mpz[2];
  let r: *mut mpz_t = &raw mut mpz[3];
  mpz_tdiv_qr ((*q).as_mut_ptr (), (*r).as_mut_ptr (), n, d);

  /* The amount to add to the absolute value of Q so that truncating
     it to double will round correctly.  */
  let incr: c_int;

  /* Round the quotient before converting it to double.
     If the quotient is less than FLT_RADIX ** DBL_MANT_DIG,
     round to the nearest integer; otherwise, it is less than
     FLT_RADIX ** (DBL_MANT_DIG + 1) and round it to the nearest
     multiple of FLT_RADIX.  Break ties to even.  */
  if mpz_sizeinbase ((*q).as_ptr (), FLT_RADIX) as c_int <= DBL_MANT_DIG
    {
      /* Converting to double will use the whole quotient so add 1 to
	 its absolute value as per round-to-even; i.e., if the doubled
	 remainder exceeds the denominator, or exactly equals the
	 denominator and adding 1 would make the quotient even.  */
      mpz_mul_2exp ((*r).as_mut_ptr (), (*r).as_ptr (), 1);
      let cmp: c_int = mpz_cmpabs ((*r).as_ptr (), d);
      incr = if cmp > 0 || (cmp == 0 && (if FASTER_TIMEFNS != 0 && FLT_RADIX == 2
					 { mpz_odd_p! ((*q).as_ptr ()) != 0 }
					 else { (mpz_tdiv_ui ((*q).as_ptr (),
							      (FLT_RADIX * FLT_RADIX) as c_ulong) & 1) != 0 }))
	{ 1 } else { 0 };
    }
  else
    {
      /* Converting to double will discard the quotient's low-order digit,
	 so add FLT_RADIX to its absolute value as per round-to-even.  */
      let lo_2digits: c_int =
	mpz_tdiv_ui ((*q).as_ptr (), (FLT_RADIX * FLT_RADIX) as c_ulong) as c_int;
      eassume! (0 <= lo_2digits && lo_2digits < FLT_RADIX * FLT_RADIX);
      let lo_digit: c_int = lo_2digits % FLT_RADIX;
      incr = if lo_digit > FLT_RADIX / 2
	       || (lo_digit == FLT_RADIX / 2 && FLT_RADIX % 2 == 0
		   && (((lo_2digits / FLT_RADIX) & 1) != 0
		       || mpz_sgn ((*r).as_ptr ()) != 0))
	{ FLT_RADIX } else { 0 };
    }

  /* Increment the absolute value of the quotient by INCR.  */
  if FASTER_TIMEFNS == 0 || incr != 0
    {
      if mpz_sgn (n) < 0
	{ mpz_sub_ui ((*q).as_mut_ptr (), (*q).as_ptr (), incr as c_ulong); }
      else
	{ mpz_add_ui ((*q).as_mut_ptr (), (*q).as_ptr (), incr as c_ulong); }
    }

  /* Rescale the integer Q back to double.  This step does not round.  */
  return scalbn (mpz_get_d ((*q).as_ptr ()), (-scale) as c_int);
}

/* Convert Z to time_t, returning true if it fits.  */
unsafe fn mpz_time (z: *const __mpz_struct, t: *mut time_t) -> bool
{
  if TYPE_SIGNED! (time_t)
    {
      let mut i: intmax_t = 0;
      return mpz_to_intmax (z, &mut i) && {
	let (ckd_add_value, ckd_add_overflow) = i.overflowing_add (0);
	*t = ckd_add_value as time_t;
	!ckd_add_overflow
      };
    }
  else
    {
      let mut i: uintmax_t = 0;
      return mpz_to_uintmax (z, &mut i) && {
	let (ckd_add_value, ckd_add_overflow) = i.overflowing_add (0);
	*t = ckd_add_value as time_t;
	!ckd_add_overflow
      };
    }
}

/* Return a valid timespec (S, N) if S is in time_t range,
   an invalid timespec otherwise.  */
unsafe fn s_ns_to_timespec (s: intmax_t, ns: c_long) -> timespec
{
  let mut sec: time_t = 0;
  let (ckd_add_value, ckd_add_overflow) = s.overflowing_add (0);
  sec = ckd_add_value as time_t;
  let nsec: c_long = if ckd_add_overflow { -1 } else { ns };
  return make_timespec (sec, nsec);
}

/* Components of a Lisp timestamp (TICKS . HZ).  Using this C struct can
   avoid the consing overhead of creating (TICKS . HZ).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ticks_hz
{
  /* Clock count as a Lisp integer.  */
  pub ticks: Lisp_Object,

  /* Clock frequency (ticks per second) as a positive Lisp integer.  */
  pub hz: Lisp_Object,
}

/* Convert (TICKS . HZ) to struct timespec, returning an invalid
   timespec if the result would not fit.  */
unsafe fn ticks_hz_to_timespec (ticks: Lisp_Object, hz: Lisp_Object) -> timespec
{
  let mut ns: c_int = 0;
  let q: *mut mpz_t = &raw mut mpz[0];
  let mut qt: *const mpz_t = q;

  /* Floor-divide (TICKS * TIMESPEC_HZ) by HZ,
     yielding quotient Q (tv_sec) and remainder NS (tv_nsec).
     Return an invalid timespec if Q does not fit in time_t.
     For speed, prefer fixnum arithmetic if it works.  */
  if FASTER_TIMEFNS != 0 && BASE_EQ (hz, timespec_hz! ())
    {
      if FIXNUMP (ticks)
	{
	  let mut s: EMACS_INT = XFIXNUM (ticks) / TIMESPEC_HZ as EMACS_INT;
	  ns = (XFIXNUM (ticks) % TIMESPEC_HZ as EMACS_INT) as c_int;
	  if ns < 0
	    { s -= 1; ns += TIMESPEC_HZ; }
	  return s_ns_to_timespec (s as intmax_t, ns as c_long);
	}
      ns = mpz_fdiv_q_ui ((*q).as_mut_ptr (), xbignum_val (ticks),
			  TIMESPEC_HZ as c_ulong) as c_int;
    }
  else if FASTER_TIMEFNS != 0 && BASE_EQ (hz, make_fixnum (1))
    {
      ns = 0;
      if FIXNUMP (ticks)
	{ return s_ns_to_timespec (XFIXNUM (ticks) as intmax_t, ns as c_long); }
      qt = xbignum_val (ticks) as *const mpz_t;
    }
  else
    {
      mpz_mul_ui ((*q).as_mut_ptr (), bignum_integer (q, ticks), TIMESPEC_HZ as c_ulong);
      mpz_fdiv_q ((*q).as_mut_ptr (), (*q).as_ptr (),
		  bignum_integer (&raw mut mpz[1], hz));
      ns = mpz_fdiv_q_ui ((*q).as_mut_ptr (), (*q).as_ptr (),
			  TIMESPEC_HZ as c_ulong) as c_int;
    }

  /* Check that Q fits in time_t, not merely in RESULT.tv_sec.  With some MinGW
     versions, tv_sec is a 64-bit type, whereas time_t is a 32-bit type.  */
  let mut sec: time_t = 0;
  return if mpz_time ((*qt).as_ptr (), &mut sec) { make_timespec (sec, ns as c_long) }
         else { invalid_timespec () };
}

/* C timestamp forms.  This enum is passed to conversion functions to
   specify the desired C timestamp form.  */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum cform
  {
    CFORM_TICKS_HZ, /* struct ticks_hz */
    CFORM_TIMESPEC, /* struct timespec */
    CFORM_SECS_ONLY, /* struct timespec but tv_nsec irrelevant
			if timespec valid */
    CFORM_DOUBLE /* double */
  }
use cform::*;

/* A C timestamp in one of the forms specified by enum cform.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub union c_time
{
  pub th: ticks_hz,
  pub ts: timespec,
  pub d: c_double,
}

/* From a valid timestamp (TICKS . HZ), generate the corresponding
   time value in CFORM form.  */
unsafe fn decode_ticks_hz (ticks: Lisp_Object, hz: Lisp_Object, cform: cform) -> c_time
{
  match cform
    {
      CFORM_DOUBLE =>
	return c_time { d: frac_to_double (ticks, hz) },

      CFORM_TICKS_HZ =>
	return c_time { th: ticks_hz { ticks: ticks, hz: hz } },

      _ =>
	return c_time { ts: ticks_hz_to_timespec (ticks, hz) },
    }
}

/* Convert the finite number T into a C time of form CFORM, truncating
   toward minus infinity.  Signal an error if unsuccessful.  */
unsafe fn decode_float_time (t: c_double, cform: cform) -> c_time
{
  if FASTER_TIMEFNS != 0 && cform == CFORM_DOUBLE
    { return c_time { d: t }; }

  let mut ticks: Lisp_Object;
  let mut hz: Lisp_Object;
  if t == 0.0
    {
      ticks = make_fixnum (0);
      hz = make_fixnum (1);
    }
  else
    {
      let mut scale: c_int = double_integer_scale (t);
      /* Because SCALE treats trailing zeros in T as significant,
	 on typical platforms with IEEE floating point
	 (time-convert 3.5 t) yields (7881299347898368 . 2251799813685248),
	 a precision of 2**-51 s, not (7 . 2), a precision of 0.5 s.
	 Although numerically correct, this generates largish integers.
         On 64bit systems, this should not matter very much, tho.  */
      eassume! (scale < flt_radix_power_size);

      if scale < 0
	{
	 /* T is finite but so large that HZ would be less than 1 if
	    T's precision were represented exactly.  SCALE must be
	    nonnegative, as the (TICKS . HZ) representation requires
	    HZ to be at least 1.  So use SCALE = 0, which converts T to
	    (T . 1), which is the exact numeric value with too-large HZ,
	    which is typically better than signaling overflow.  */
	  scale = 0;
	}

      /* Compute TICKS, HZ such that TICKS / HZ exactly equals T, where HZ is
	 T's frequency or 1, whichever is greater.  Here, “frequency” means
	 1/precision.  Cache HZ values in flt_radix_power.  */
      let scaled: c_double = scalbn (t, scale);
      eassert! (trunc (scaled) == scaled);
      ticks = double_to_integer (scaled);
      hz = AREF (flt_radix_power, scale as isize);
      if NILP (hz)
	{
	  mpz_ui_pow_ui (mpz[0].as_mut_ptr (), FLT_RADIX as c_ulong, scale as c_ulong);
	  hz = make_integer_mpz ();
	  ASET (flt_radix_power, scale as isize, hz);
	}
    }
  return decode_ticks_hz (ticks, hz, cform);
}

/* Make a 4-element timestamp (HI LO US PS) from TICKS and HZ.
   Drop any excess precision.  */
unsafe fn ticks_hz_list4 (ticks: Lisp_Object, hz: Lisp_Object) -> Lisp_Object
{
  /* mpz[0] = floor ((ticks * trillion) / hz).  */
  let zticks: *const __mpz_struct = bignum_integer (&raw mut mpz[0], ticks);
  /* C 667-671: the frozen build takes the `TRILLION <= ULONG_MAX` arm; the `ztrillion`
     arm is carried (P-51).  */
  #[cfg (feature = "no_trillion_shortcut")]
  { mpz_mul (mpz[0].as_mut_ptr (), zticks, ztrillion.as_ptr ()); }
  #[cfg (not (feature = "no_trillion_shortcut"))]
  { mpz_mul_ui (mpz[0].as_mut_ptr (), zticks, TRILLION as c_ulong); }
  mpz_fdiv_q (mpz[0].as_mut_ptr (), mpz[0].as_ptr (),
	      bignum_integer (&raw mut mpz[1], hz));

  /* mpz[0] = floor (mpz[0] / trillion), with US = the high six digits of the
     12-digit remainder, and PS = the low six digits.  */
  let mut fullps: c_ulong = 0;
  let mut us: c_int = 0;
  let mut ps: c_int = 0;
  #[cfg (not (feature = "no_trillion_shortcut"))]
  {
    fullps = mpz_fdiv_q_ui (mpz[0].as_mut_ptr (), mpz[0].as_ptr (), TRILLION as c_ulong);
    us = (fullps / 1000000) as c_int;
    ps = (fullps % 1000000) as c_int;
  }
  #[cfg (feature = "no_trillion_shortcut")]
  {
    mpz_fdiv_qr (mpz[0].as_mut_ptr (), mpz[1].as_mut_ptr (), mpz[0].as_ptr (),
		 ztrillion.as_ptr ());
    ps = mpz_fdiv_q_ui (mpz[1].as_mut_ptr (), mpz[1].as_ptr (), 1000000) as c_int;
    us = mpz_get_ui (mpz[1].as_ptr ()) as c_int;
  }

  /* mpz[0] = floor (mpz[0] / (1 << LO_TIME_BITS)), with LO = remainder.  */
  let mut ulo: c_ulong = mpz_get_ui (mpz[0].as_ptr ());
  if mpz_sgn (mpz[0].as_ptr ()) < 0
    { ulo = ulo.wrapping_neg (); }
  let lo: c_int = (ulo & ((1 << LO_TIME_BITS) - 1)) as c_int;
  mpz_fdiv_q_2exp (mpz[0].as_mut_ptr (), mpz[0].as_ptr (), LO_TIME_BITS as mp_bitcnt_t);

  return list4 (make_integer_mpz (), make_fixnum (lo as EMACS_INT),
		make_fixnum (us as EMACS_INT), make_fixnum (ps as EMACS_INT));
}

/* Set ROP to T.  */
unsafe fn mpz_set_time (rop: *mut __mpz_struct, t: time_t)
{
  if EXPR_SIGNED! (t)
    { mpz_set_intmax (rop, t as intmax_t); }
  else
    { mpz_set_uintmax (rop, t as uintmax_t); }
}

/* Store into mpz[0] a clock tick count for T, assuming a
   TIMESPEC_HZ-frequency clock.  Use mpz[1] as a temp.  */
unsafe fn timespec_mpz (t: timespec)
{
  /* mpz[0] = sec * TIMESPEC_HZ + nsec.  */
  mpz_set_ui (mpz[0].as_mut_ptr (), t.tv_nsec as c_ulong);
  mpz_set_time (mpz[1].as_mut_ptr (), t.tv_sec);
  mpz_addmul_ui (mpz[0].as_mut_ptr (), mpz[1].as_ptr (), TIMESPEC_HZ as c_ulong);
}

/* Convert T to a Lisp integer counting TIMESPEC_HZ ticks.  */
unsafe fn timespec_ticks (t: timespec) -> Lisp_Object
{
  /* For speed, use intmax_t arithmetic if it will do.  */
  let mut accum: intmax_t = 0;
  if FASTER_TIMEFNS != 0
    {
      let (ckd_mul_value, ckd_mul_overflow) =
	(t.tv_sec as intmax_t).overflowing_mul (TIMESPEC_HZ as intmax_t);
      accum = ckd_mul_value;
      if !ckd_mul_overflow
	{
	  let (ckd_add_value, ckd_add_overflow) =
	    accum.overflowing_add (t.tv_nsec as intmax_t);
	  accum = ckd_add_value;
	  if !ckd_add_overflow
	    { return make_int (accum); }
	}
    }

  /* Fall back on bignum arithmetic.  */
  timespec_mpz (t);
  return make_integer_mpz ();
}

/* Return greatest common divisor of positive A and B.  */
unsafe fn emacs_gcd (mut a: EMACS_INT, mut b: EMACS_INT) -> EMACS_INT
{
  let mut r: EMACS_INT = 0;
  loop
    {
      r = a % b;
      if r == 0
	{ break; }
      a = b;
      b = r;
      continue;
    }
  return b;
}

/* Convert T to a Lisp integer counting HZ ticks, taking the floor.
   Assume T is valid, but check HZ.  */
unsafe fn ticks_hz_hz_ticks (mut t: ticks_hz, mut hz: Lisp_Object) -> Lisp_Object
{
  /* The idea is to return the floor of ((T.ticks * HZ) / T.hz).  */

  /* For speed, just return T.ticks if T.hz == HZ.  */
  if FASTER_TIMEFNS != 0 && BASE_EQ (t.hz, hz)
    { return t.ticks; }

  /* Check HZ for validity.  */
  if FIXNUMP (hz)
    {
      if XFIXNUM (hz) <= 0
	{ invalid_hz (hz); }

      /* Prefer non-bignum arithmetic to speed up common cases.  */
      if FASTER_TIMEFNS != 0 && FIXNUMP (t.hz)
	{
	  /* Reduce T.hz and HZ by their GCD, to avoid some intmax_t
	     overflows that would occur in T.ticks * HZ.  */
	  let mut ithz: EMACS_INT = XFIXNUM (t.hz);
	  let mut ihz: EMACS_INT = XFIXNUM (hz);
	  let d: EMACS_INT = emacs_gcd (ithz, ihz);
	  ithz /= d;
	  ihz /= d;

	  if FIXNUMP (t.ticks)
	    {
	      let mut ticks: intmax_t = 0;
	      let (ckd_mul_value, ckd_mul_overflow) =
		(XFIXNUM (t.ticks) as intmax_t).overflowing_mul (ihz as intmax_t);
	      ticks = ckd_mul_value;
	      if !ckd_mul_overflow
		{ return make_int (ticks / ithz as intmax_t
				   - ((ticks % (ithz as intmax_t) < 0) as intmax_t)); }
	    }

	  t.hz = make_fixnum (ithz);
	  hz = make_fixnum (ihz);
	}
    }
  else if ! (BIGNUMP (hz) && 0 < mpz_sgn (xbignum_val (hz)))
    { invalid_hz (hz); }

  /* Fall back on bignum arithmetic.  */
  let zticks: *const __mpz_struct = bignum_integer (&raw mut mpz[0], t.ticks);
  if FASTER_TIMEFNS != 0 && FIXNUMP (hz) && (XFIXNUM (hz) as c_ulong) <= ULONG_MAX
    { mpz_mul_ui (mpz[0].as_mut_ptr (), zticks, XFIXNUM (hz) as c_ulong); }
  else
    { mpz_mul (mpz[0].as_mut_ptr (), zticks, bignum_integer (&raw mut mpz[1], hz)); }
  if FASTER_TIMEFNS != 0 && FIXNUMP (t.hz) && (XFIXNUM (t.hz) as c_ulong) <= ULONG_MAX
    { mpz_fdiv_q_ui (mpz[0].as_mut_ptr (), mpz[0].as_ptr (), XFIXNUM (t.hz) as c_ulong); }
  else
    { mpz_fdiv_q (mpz[0].as_mut_ptr (), mpz[0].as_ptr (),
		  bignum_integer (&raw mut mpz[1], t.hz)); }
  return make_integer_mpz ();
}

/* Convert T to a Lisp integer counting seconds, taking the floor.  */
unsafe fn ticks_hz_seconds (t: ticks_hz) -> Lisp_Object
{
  /* The idea is to return the floor of T.ticks / T.hz.  */

  if FASTER_TIMEFNS == 0
    { return ticks_hz_hz_ticks (t, make_fixnum (1)); }

  /* For speed, use EMACS_INT arithmetic if it will do.  */
  if FIXNUMP (t.ticks) && FIXNUMP (t.hz)
    { return make_fixnum (XFIXNUM (t.ticks) / XFIXNUM (t.hz)
			  - ((XFIXNUM (t.ticks) % XFIXNUM (t.hz) < 0) as EMACS_INT)); }

  /* For speed, inline what ticks_hz_hz_ticks would do.  */
  mpz_fdiv_q (mpz[0].as_mut_ptr (),
	      bignum_integer (&raw mut mpz[0], t.ticks),
	      bignum_integer (&raw mut mpz[1], t.hz));
  return make_integer_mpz ();
}

/* Convert T to a Lisp timestamp.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_lisp_time (t: timespec) -> Lisp_Object
{
  if current_time_list
    {
      let s: time_t = t.tv_sec;
      let ns: c_int = t.tv_nsec as c_int;
      return list4 (hi_time (s), lo_time (s),
		    make_fixnum ((ns / 1000) as EMACS_INT),
		    make_fixnum ((ns % 1000 * 1000) as EMACS_INT));
    }
  else
    { return timespec_to_lisp (t); }
}

/* Return (TICKS . HZ) for time T.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timespec_to_lisp (t: timespec) -> Lisp_Object
{
  return Fcons (timespec_ticks (t), timespec_hz! ());
}

/* An (error number, C timestamp) pair.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct err_time
{
  pub err: c_int,
  pub time: c_time,
}

/* From the time components HIGH, LOW, USEC and PSEC and the timestamp
   resolution HZ, generate the corresponding time value in CFORM form.
   HZ should be either 1, 1000000, or 1000000000000.
   Return a (0, valid timestamp) pair if successful, an (error number,
   unspecified timestamp) pair otherwise.  */
unsafe fn decode_time_components (high: Lisp_Object, low: Lisp_Object,
				  usec: Lisp_Object, psec: Lisp_Object,
				  hz: Lisp_Object, cform: cform) -> err_time
{
  if !(FIXNUMP (usec) && FIXNUMP (psec))
    { return err_time { err: EINVAL, time: std::mem::zeroed () }; }

  let mut us: EMACS_INT = XFIXNUM (usec);
  let mut ps: EMACS_INT = XFIXNUM (psec);

  /* Normalize out-of-range lower-order components by carrying
     each overflow into the next higher-order component.  */
  us += ps / 1000000 - ((ps % 1000000 < 0) as EMACS_INT);
  let s_from_us_ps: EMACS_INT = us / 1000000 - ((us % 1000000 < 0) as EMACS_INT);
  ps = ps % 1000000 + 1000000 * ((ps % 1000000 < 0) as EMACS_INT);
  us = us % 1000000 + 1000000 * ((us % 1000000 < 0) as EMACS_INT);

  if FASTER_TIMEFNS != 0 && FIXNUMP (high) && FIXNUMP (low)
    {
      /* Use intmax_t arithmetic if the tick count fits.  */
      let mut iticks: intmax_t = 0;
      let mut v: bool = false;
      let (ckd_mul_value, ckd_mul_overflow) =
	(XFIXNUM (high) as intmax_t).overflowing_mul (1 << LO_TIME_BITS);
      iticks = ckd_mul_value;
      v = v || ckd_mul_overflow;
      let (ckd_add_value, ckd_add_overflow) =
	iticks.overflowing_add ((XFIXNUM (low) + s_from_us_ps) as intmax_t);
      iticks = ckd_add_value;
      v = v || ckd_add_overflow;
      if !v
	{
	  if cform == CFORM_TIMESPEC || cform == CFORM_SECS_ONLY
	    {
	      return err_time { err: 0, time: c_time { ts: s_ns_to_timespec (
		iticks, (us * 1000 + ps / 1000) as c_long) } };
	    }

	  if BASE_EQ (hz, trillion! ())
	    {
	      let million: int_fast64_t = 1000000;
	      let (ckd_mul_value, ckd_mul_overflow) =
		iticks.overflowing_mul (TRILLION as intmax_t);
	      iticks = ckd_mul_value;
	      v = v || ckd_mul_overflow;
	      let (ckd_add_value, ckd_add_overflow) =
		iticks.overflowing_add ((us * million as EMACS_INT + ps) as intmax_t);
	      iticks = ckd_add_value;
	      v = v || ckd_add_overflow;
	    }
	  else if BASE_EQ (hz, make_fixnum (1000000))
	    {
	      let (ckd_mul_value, ckd_mul_overflow) =
		iticks.overflowing_mul (1000000);
	      iticks = ckd_mul_value;
	      v = v || ckd_mul_overflow;
	      let (ckd_add_value, ckd_add_overflow) = iticks.overflowing_add (us as intmax_t);
	      iticks = ckd_add_value;
	      v = v || ckd_add_overflow;
	    }

	  if !v
	    {
	      return err_time { err: 0, time: decode_ticks_hz (make_int (iticks), hz, cform) };
	    }
	}
    }

  if ! (INTEGERP (high) && INTEGERP (low))
    { return err_time { err: EINVAL, time: std::mem::zeroed () }; }

  let s: *mut mpz_t = &raw mut mpz[1];
  mpz_set_intmax ((*s).as_mut_ptr (), s_from_us_ps as intmax_t);
  mpz_add ((*s).as_mut_ptr (), (*s).as_ptr (), bignum_integer (&raw mut mpz[0], low));
  mpz_addmul_ui ((*s).as_mut_ptr (), bignum_integer (&raw mut mpz[0], high),
		 (1 << LO_TIME_BITS) as c_ulong);

  if BASE_EQ (hz, trillion! ())
    {
      /* C 915-923: the frozen build takes the `TRILLION <= ULONG_MAX` arm; the `ztrillion`
	 arm is carried (P-51).  */
      #[cfg (not (feature = "no_trillion_shortcut"))]
      {
	let i: c_ulong = us as c_ulong;
	mpz_set_ui (mpz[0].as_mut_ptr (), i * 1000000 + ps as c_ulong);
	mpz_addmul_ui (mpz[0].as_mut_ptr (), (*s).as_ptr (), TRILLION as c_ulong);
      }
      #[cfg (feature = "no_trillion_shortcut")]
      {
	let i: intmax_t = us as intmax_t;
	mpz_set_intmax (mpz[0].as_mut_ptr (), i * 1000000 + ps as intmax_t);
	mpz_addmul (mpz[0].as_mut_ptr (), (*s).as_ptr (), ztrillion.as_ptr ());
      }
    }
  else if BASE_EQ (hz, make_fixnum (1000000))
    {
      mpz_set_ui (mpz[0].as_mut_ptr (), us as c_ulong);
      mpz_addmul_ui (mpz[0].as_mut_ptr (), (*s).as_ptr (), 1000000);
    }
  else
    { mpz_swap (mpz[0].as_mut_ptr (), (*s).as_mut_ptr ()); }

  let ticks: Lisp_Object = make_integer_mpz ();
  return err_time { err: 0, time: decode_ticks_hz (ticks, hz, cform) };
}

/* Current time (seconds since epoch) in form CFORM.  */
unsafe fn current_time_in_cform (cform: cform) -> c_time
{
  let now: timespec = current_timespec ();
  return if FASTER_TIMEFNS != 0
	   && (cform == CFORM_TIMESPEC || cform == CFORM_SECS_ONLY)
	 { c_time { ts: now } }
	 else { decode_ticks_hz (timespec_ticks (now), timespec_hz! (), cform) };
}

/* Decode a Lisp timestamp SPECIFIED_TIME that represents a time.

   Return a (form, time) pair that is the form of SPECIFIED-TIME
   and the resulting C timestamp in CFORM form.
   If CFORM == CFORM_SECS_ONLY, ignore and do not validate any sub-second
   components of an old-format SPECIFIED_TIME.

   Signal an error if unsuccessful.  */
unsafe fn decode_lisp_time (specified_time: Lisp_Object, cform: cform) -> c_time
{
  /* specified_time is one of:

     nil
       current time
     NUMBER
       that number of seconds
     (A . B)    ; A, B : integer, B>0
       A/B s
     (A B C D)  ; A, B : integer, C, D : fixnum
       (A * 2**16 + B + C / 10**6 + D / 10**12) s

     The following specified_time forms are also supported,
     for compatibility with older Emacs versions:

     (A B)
       like (A B 0 0)
     (A B . C)  ; C : fixnum
       like (A B C 0)
     (A B C)
       like (A B C 0)
  */

  if NILP (specified_time)
    { return current_time_in_cform (cform); }
  else if CONSP (specified_time)
    {
      let high: Lisp_Object = XCAR (specified_time);
      let mut low: Lisp_Object = XCDR (specified_time);
      let mut usec: Lisp_Object = make_fixnum (0);
      let mut psec: Lisp_Object = make_fixnum (0);
      if CONSP (low)
	{
	  let mut hz: Lisp_Object = make_fixnum (1);
	  let mut low_tail: Lisp_Object = XCDR (low);
	  low = XCAR (low);
	  if cform != CFORM_SECS_ONLY
	    {
	      if CONSP (low_tail)
		{
		  usec = XCAR (low_tail);
		  low_tail = XCDR (low_tail);
		  if CONSP (low_tail)
		    {
		      psec = XCAR (low_tail);
		      hz = trillion! ();
		    }
		  else
		    { hz = make_fixnum (1000000); }
		}
	      else if !NILP (low_tail)
		{
		  usec = low_tail;
		  hz = make_fixnum (1000000);
		}
	    }

	  let err_time: err_time
	    = decode_time_components (high, low, usec, psec, hz, cform);
	  if err_time.err != 0
	    { time_error (err_time.err); }
	  return err_time.time;
	}
      else
	{
	  /* (TICKS . HZ) */
	  if !(INTEGERP (high) && (if FIXNUMP (low) { XFIXNUM (low) > 0 }
				   else { !NILP (Fnatnump (low)) }))
	    { time_spec_invalid (); }
	  return decode_ticks_hz (high, low, cform);
	}
    }
  else if INTEGERP (specified_time)
    { return decode_ticks_hz (specified_time, make_fixnum (1), cform); }
  else if FLOATP (specified_time)
    {
      let d: c_double = XFLOAT_DATA (specified_time);
      if !isfinite! (d)
	{ time_error (if isnan! (d) { EDOM } else { EOVERFLOW }); }
      return decode_float_time (d, cform);
    }
  else
    { time_spec_invalid (); }
}

/* Convert a non-float Lisp timestamp SPECIFIED_TIME to double.
   Signal an error if unsuccessful.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn float_time (specified_time: Lisp_Object) -> c_double
{
  return decode_lisp_time (specified_time, CFORM_DOUBLE).d;
}

/* Convert (HIGH LOW USEC PSEC) to struct timespec.
   Return a valid timestamp if successful, an invalid one otherwise.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn list4_to_timespec (high: Lisp_Object, low: Lisp_Object,
					    usec: Lisp_Object, psec: Lisp_Object) -> timespec
{
  let err_time: err_time
    = decode_time_components (high, low, usec, psec, trillion! (), CFORM_TIMESPEC);
  return if err_time.err != 0 { invalid_timespec () } else { err_time.time.ts };
}

/* Decode a Lisp time value SPECIFIED_TIME that represents a time.
   Discard any low-order (sub-ns) resolution.
   If SPECIFIED_TIME is nil, use the current time.
   Signal an error if SPECIFIED_TIME does not represent a timespec.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lisp_time_argument (specified_time: Lisp_Object) -> timespec
{
  let t: timespec = decode_lisp_time (specified_time, CFORM_TIMESPEC).ts;
  if ! timespec_valid_p (t)
    { time_overflow (); }
  return t;
}

/* Like lisp_time_argument, except decode only the seconds part, and
   do not check the subseconds part.  */
unsafe fn lisp_seconds_argument (specified_time: Lisp_Object) -> time_t
{
  let t: timespec = decode_lisp_time (specified_time, CFORM_SECS_ONLY).ts;
  if ! timespec_valid_p (t)
    { time_overflow (); }
  return t.tv_sec;
}

/* Return the sum of the Lisp integers A and B.
   Subtract instead of adding if SUBTRACT.
   This function is tuned for small B.  */
unsafe fn lispint_arith (a: Lisp_Object, b: Lisp_Object, subtract: bool) -> Lisp_Object
{
  let mut mpz_done: bool = false;

  if FASTER_TIMEFNS != 0 && FIXNUMP (b)
    {
      if BASE_EQ (b, make_fixnum (0))
	{ return a; }

      /* For speed, use EMACS_INT arithmetic if it will do.  */
      if FIXNUMP (a)
	{ return make_int (if subtract { (XFIXNUM (a) - XFIXNUM (b)) as intmax_t }
			   else { (XFIXNUM (a) + XFIXNUM (b)) as intmax_t }); }

      /* For speed, use mpz_add_ui/mpz_sub_ui if it will do.  */
      if (eabs! (XFIXNUM (b)) as c_ulong) <= ULONG_MAX
	{
	  if (XFIXNUM (b) < 0) == subtract
	    { mpz_add_ui (mpz[0].as_mut_ptr (), xbignum_val (a), eabs! (XFIXNUM (b)) as c_ulong); }
	  else
	    { mpz_sub_ui (mpz[0].as_mut_ptr (), xbignum_val (a), eabs! (XFIXNUM (b)) as c_ulong); }
	  mpz_done = true;
	}
    }

  /* Fall back on bignum arithmetic if necessary.  */
  if !mpz_done
    {
      if subtract
	{ mpz_sub (mpz[0].as_mut_ptr (), bignum_integer (&raw mut mpz[0], a),
		   bignum_integer (&raw mut mpz[1], b)); }
      else
	{ mpz_add (mpz[0].as_mut_ptr (), bignum_integer (&raw mut mpz[0], a),
		   bignum_integer (&raw mut mpz[1], b)); }
    }
  return make_integer_mpz ();
}

/* Given Lisp operands A and B, add their values, and return the
   result as a Lisp timestamp.  Subtract instead of adding if SUBTRACT.  */
unsafe fn time_arith (a: Lisp_Object, b: Lisp_Object, subtract: bool) -> Lisp_Object
{
  let ta: ticks_hz = decode_lisp_time (a, CFORM_TICKS_HZ).th;
  let tb: ticks_hz = decode_lisp_time (b, CFORM_TICKS_HZ).th;
  let mut ticks: Lisp_Object;
  let mut hz: Lisp_Object;

  if FASTER_TIMEFNS != 0 && BASE_EQ (ta.hz, tb.hz)
    {
      hz = ta.hz;
      ticks = lispint_arith (ta.ticks, tb.ticks, subtract);
    }
  else
    {
      /* The plan is to decompose ta into na/da and tb into nb/db.
	 Start by computing da and db, their minimum (which will be
	 needed later) and the iticks temporary that will become
	 available once only their minimum is needed.  */
      let da: *const __mpz_struct = bignum_integer (&raw mut mpz[1], ta.hz);
      let db: *const __mpz_struct = bignum_integer (&raw mut mpz[2], tb.hz);
      let da_lt_db: bool = mpz_cmp (da, db) < 0;
      let hzmin: *const __mpz_struct = if da_lt_db { da } else { db };
      let iticks: *mut mpz_t = &raw mut mpz[if da_lt_db { 2 } else { 1 }];

      /* The plan is to compute (na * (db/g) + nb * (da/g)) / lcm (da, db)
	 where g = gcd (da, db).  Start by computing g.  */
      let g: *mut mpz_t = &raw mut mpz[3];
      mpz_gcd ((*g).as_mut_ptr (), da, db);

      /* fa = da/g, fb = db/g.  */
      let fa: *mut mpz_t = &raw mut mpz[4];
      let fb: *mut mpz_t = &raw mut mpz[3];
      mpz_divexact ((*fa).as_mut_ptr (), da, (*g).as_ptr ());
      mpz_divexact ((*fb).as_mut_ptr (), db, (*g).as_ptr ());

      /* ihz = fa * db.  This is equal to lcm (da, db).  */
      let ihz: *mut mpz_t = &raw mut mpz[0];
      mpz_mul ((*ihz).as_mut_ptr (), (*fa).as_ptr (), db);

      /* iticks = (fb * na) OP (fa * nb), where OP is + or -.  */
      let na: *const __mpz_struct = bignum_integer (iticks, ta.ticks);
      mpz_mul ((*iticks).as_mut_ptr (), (*fb).as_ptr (), na);
      let nb: *const __mpz_struct = bignum_integer (&raw mut mpz[3], tb.ticks);
      if subtract
	{ mpz_submul ((*iticks).as_mut_ptr (), (*fa).as_ptr (), nb); }
      else
	{ mpz_addmul ((*iticks).as_mut_ptr (), (*fa).as_ptr (), nb); }

      /* Normalize iticks/ihz by dividing both numerator and
	 denominator by ig = gcd (iticks, ihz).  For speed, though,
	 skip this division if ihz = 1.  */
      let ig: *mut mpz_t = &raw mut mpz[3];
      mpz_gcd ((*ig).as_mut_ptr (), (*iticks).as_ptr (), (*ihz).as_ptr ());
      if FASTER_TIMEFNS == 0 || mpz_cmp_ui ((*ig).as_ptr (), 1) > 0
	{
	  mpz_divexact ((*iticks).as_mut_ptr (), (*iticks).as_ptr (), (*ig).as_ptr ());
	  mpz_divexact ((*ihz).as_mut_ptr (), (*ihz).as_ptr (), (*ig).as_ptr ());

	  /* However, if dividing the denominator by ig would cause the
	     denominator to become less than hzmin, rescale the denominator
	     upwards by multiplying the normalized numerator and denominator
	     so that the resulting denominator becomes at least hzmin.
	     This rescaling avoids returning a timestamp that is less precise
	     than both a and b.  */
	  if FASTER_TIMEFNS == 0 || mpz_cmp ((*ihz).as_ptr (), hzmin) < 0
	    {
	      /* Rescale straightforwardly.  Although this might not
		 yield the minimal denominator that preserves numeric
		 value and is at least hzmin, calculating such a
		 denominator would be too expensive because it would
		 require testing multisets of factors of lcm (da, db).  */
	      let rescale: *mut mpz_t = &raw mut mpz[3];
	      mpz_cdiv_q ((*rescale).as_mut_ptr (), hzmin, (*ihz).as_ptr ());
	      mpz_mul ((*iticks).as_mut_ptr (), (*iticks).as_ptr (), (*rescale).as_ptr ());
	      mpz_mul ((*ihz).as_mut_ptr (), (*ihz).as_ptr (), (*rescale).as_ptr ());
	    }
	}

      /* mpz[0] and iticks now correspond to the (HZ . TICKS) pair.  */
      hz = make_integer_mpz ();
      mpz_swap (mpz[0].as_mut_ptr (), (*iticks).as_mut_ptr ());
      ticks = make_integer_mpz ();
    }

  /* Return an integer if the timestamp resolution is 1,
     otherwise the (TICKS . HZ) form if !current_time_list or if
     either input used (TICKS . HZ) form or the result can't be expressed
     exactly in (HI LO US PS) form, otherwise the (HI LO US PS) form
     for backward compatibility.  */
  return if BASE_EQ (hz, make_fixnum (1))
    { ticks }
    else if !current_time_list
	 || (CONSP (a) && !CONSP (XCDR (a)))
	 || (CONSP (b) && !CONSP (XCDR (b)))
	 || !trillion_factor (hz)
    { Fcons (ticks, hz) }
    else
    { ticks_hz_list4 (ticks, hz) };
}

DEFUN! ("time-add", Ftime_add, Stime_add, 2, 2, 0,
       doc: "Return the sum of two time values A and B, as a time value.\nSee `format-time-string' for the various forms of a time value.\nFor example, nil stands for the current time.  ",
       (a: Lisp_Object, b: Lisp_Object),
       {
  return time_arith (a, b, false);
});

DEFUN! ("time-subtract", Ftime_subtract, Stime_subtract, 2, 2, 0,
       doc: "Return the difference between two time values A and B, as a time value.\nYou can use `float-time' to convert the difference into elapsed seconds.\nSee `format-time-string' for the various forms of a time value.\nFor example, nil stands for the current time.  ",
       (a: Lisp_Object, b: Lisp_Object),
       {
  /* Subtract nil from nil correctly, and handle other eq values
     quicker while we're at it.  This means (time-subtract X X) does
     not signal an error if X is not a valid time value, but that's OK.  */
  if BASE_EQ (a, b)
    { return make_lisp_time (timespec { tv_sec: 0, tv_nsec: 0 }); }

  return time_arith (a, b, true);
});

/* Return negative, 0, positive if A < B, A == B, A > B respectively.
   A and B should be Lisp time values.  */
unsafe fn time_cmp (a: Lisp_Object, b: Lisp_Object) -> EMACS_INT
{
  /* Compare nil to nil correctly, and handle other eq values quicker
     while we're at it.  This means (time-equal-p X X) does not signal
     an error if X is not a valid time value, but that's OK.  */
  if BASE_EQ (a, b)
    { return 0; }

  /* Compare (X . Z) to (Y . Z) quickly if X and Y are fixnums.
     Do not inspect Z, as it is OK to not signal if A and B are invalid.
     Also, compare X to Y quickly if X and Y are fixnums.  */
  if FASTER_TIMEFNS != 0
    {
      let mut x: Lisp_Object = a;
      let mut y: Lisp_Object = b;
      if CONSP (a) && CONSP (b) && BASE_EQ (XCDR (a), XCDR (b))
	{ x = XCAR (a); y = XCAR (b); }
      if FIXNUMP (x) && FIXNUMP (y)
	{ return XFIXNUM (x) - XFIXNUM (y); }
    }

  /* Compare (ATICKS . AZ) to (BTICKS . BHZ) by comparing
     ATICKS * BHZ to BTICKS * AHZ.  */
  let ta: ticks_hz = decode_lisp_time (a, CFORM_TICKS_HZ).th;
  let tb: ticks_hz = decode_lisp_time (b, CFORM_TICKS_HZ).th;
  let mut za: *const __mpz_struct = bignum_integer (&raw mut mpz[0], ta.ticks);
  let mut zb: *const __mpz_struct = bignum_integer (&raw mut mpz[1], tb.ticks);
  if ! (FASTER_TIMEFNS != 0 && BASE_EQ (ta.hz, tb.hz))
    {
      /* This could be sped up by looking at the signs, sizes, and
	 number of bits of the two sides; see how GMP does mpq_cmp.
	 It may not be worth the trouble here, though.  */
      mpz_mul (mpz[0].as_mut_ptr (), za, bignum_integer (&raw mut mpz[2], tb.hz));
      mpz_mul (mpz[1].as_mut_ptr (), zb, bignum_integer (&raw mut mpz[2], ta.hz));
      za = mpz[0].as_ptr ();
      zb = mpz[1].as_ptr ();
    }
  return mpz_cmp (za, zb) as EMACS_INT;
}

DEFUN! ("time-less-p", Ftime_less_p, Stime_less_p, 2, 2, 0,
       doc: "Return non-nil if time value A is less than time value B.\nSee `format-time-string' for the various forms of a time value.\nFor example, nil stands for the current time.  ",
       (a: Lisp_Object, b: Lisp_Object),
       {
  return if time_cmp (a, b) < 0 { Qt () } else { Qnil () };
});

DEFUN! ("time-equal-p", Ftime_equal_p, Stime_equal_p, 2, 2, 0,
       doc: "Return non-nil if A and B are equal time values.\nSee `format-time-string' for the various forms of a time value.  ",
       (a: Lisp_Object, b: Lisp_Object),
       {
  /* A nil arg compares unequal to a non-nil arg.  This also saves the
     expense of current_timespec if either arg is nil.  */
  return if NILP (a) == NILP (b) && time_cmp (a, b) == 0 { Qt () } else { Qnil () };
});

DEFUN! ("float-time", Ffloat_time, Sfloat_time, 0, 1, 0,
       doc: "Return the current time, as a float number of seconds since the epoch.\nIf SPECIFIED-TIME is given, it is a time value to convert to float\ninstead of the current time.  See `format-time-string' for the various\nforms of a time value.\n\nWARNING: Since the result is floating point, it may not be exact.\nIf precise time stamps are required, use either `time-convert',\nor (if you need time as a string) `format-time-string'.  ",
       (specified_time: Lisp_Object),
       {
  return if FLOATP (specified_time) { specified_time }
	 else { make_float (float_time (specified_time)) };
});

/* Write information into buffer S of size MAXSIZE, according to the
   FORMAT of length FORMAT_LEN, using time information taken from *TP.
   FORMAT[FORMATLEN] must be NUL.
   Use the time zone specified by TZ.
   Use NS as the number of nanoseconds in the %N directive.
   Return the number of bytes written, not including the terminating NUL.
   On error return -1, setting errno and possibly writing some bytes.

   If S is NULL, nothing will be written anywhere; so to
   determine how many bytes would be written, use NULL for S and
   SIZE_MAX for MAXSIZE.

   This function behaves like nstrftime, except it allows null
   bytes in FORMAT.  */
unsafe fn emacs_nmemftime (mut s: *mut c_char, mut maxsize: usize, mut format: *const c_char,
			   mut format_len: usize, tp: *const tm, tz: timezone_t,
			   ns: c_int) -> ptrdiff_t
{
  let mut total: ptrdiff_t = 0;

  /* Loop through all the null-terminated strings in the format
     argument.  Normally there's just one null-terminated string, but
     there can be arbitrarily many, concatenated together, if the
     format contains '\0' bytes.  nstrftime stops at the first
     '\0' byte so we must invoke it separately for each such string.  */
  loop
    {
      let result: ptrdiff_t = nstrftime (s, maxsize, format, tp, tz, ns);
      if result < 0
	{ return result; }
      let len: usize = strlen (format);
      let (ckd_add_value, ckd_add_overflow) =
	total.overflowing_add (result + ((len != format_len) as ptrdiff_t));
      total = ckd_add_value;
      if ckd_add_overflow
	{
	  *errno () = ERANGE;
	  return -1;
	}
      if len == format_len
	{ break; }
      if !s.is_null ()
	{ s = s.add ((result + 1) as usize); }
      maxsize -= (result + 1) as usize;
      format = format.add (len + 1);
      format_len -= len + 1;
    }

  return total;
}

unsafe fn format_time_string (format: *const c_char, formatlen: ptrdiff_t,
			      t: timespec, zone: Lisp_Object,
			      mut tmp: *mut tm) -> Lisp_Object
{
  let mut buffer: [c_char; 4000] = [0; 4000];
  let mut buf: *mut c_char = buffer.as_mut_ptr ();
  let mut len: ptrdiff_t = -1;
  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  let tz: timezone_t = tzlookup (zone, false);
  /* On some systems, like 32-bit MinGW, tv_sec of struct timespec is
     a 64-bit type, but time_t is a 32-bit type.  emacs_localtime_rz
     expects a pointer to time_t value.  */
  let tsec: time_t = t.tv_sec;
  tmp = emacs_localtime_rz (tz, &tsec, tmp);
  if !tmp.is_null ()
    {
      synchronize_system_time_locale ();
      let ns: c_int = t.tv_nsec as c_int;
      len = emacs_nmemftime (buffer.as_mut_ptr (), std::mem::size_of_val (&buffer), format,
			     formatlen as usize, tmp, tz, ns);
      if len < 0 && *errno () == ERANGE
	{
	  /* Buffer was too small, so make it bigger and try again.  */
	  len = emacs_nmemftime (std::ptr::null_mut (), SIZE_MAX, format,
				 formatlen as usize, tmp, tz, ns);
	  if 0 <= len && len < STRING_BYTES_BOUND as ptrdiff_t
	    {
	      buf = SAFE_ALLOCA! (sa_avail, (len + 1) as usize) as *mut c_char;
	      len = emacs_nmemftime (buf, (len + 1) as usize, format, formatlen as usize,
				     tmp, tz, ns);
	    }
	}
    }

  xtzfree (tz);
  if len < 0
    { time_error (*errno ()); }
  AUTO_STRING_WITH_LEN! (bufstring, buf, len);
  let result: Lisp_Object = code_convert_string_norecord (bufstring,
							  Vlocale_coding_system, false);
  SAFE_FREE! (sa_count);
  return result;
}

DEFUN! ("format-time-string", Fformat_time_string, Sformat_time_string, 1, 3, 0,
       doc: "Use FORMAT-STRING to format the time value TIME.\nA time value that is omitted or nil stands for the current time,\na number stands for that many seconds, an integer pair (TICKS . HZ)\nstands for TICKS/HZ seconds, and an integer list (HI LO US PS) stands\nfor HI*2**16 + LO + US/10**6 + PS/10**12 seconds.  This function\ntreats seconds as time since the epoch of 1970-01-01 00:00:00 UTC.\n\nThe optional ZONE is omitted or nil for Emacs local time, t for\nUniversal Time, `wall' for system wall clock time, or a string as in\nthe TZ environment variable.  It can also be a list (as from\n`current-time-zone') or an integer (as from `decode-time') applied\nwithout consideration for daylight saving time.\n\nThe value is a copy of FORMAT-STRING, but with certain constructs replaced\nby text that describes the specified date and time in TIME:\n\n%Y is the year, %y year without century, %C the century.\n%G is the year corresponding to the ISO week, %g year corresponding\n to the ISO week, without century.\n%m is the numeric month.\n%b and %h are the locale's abbreviated month name, %B the full name.\n (%h is not supported on MS-Windows.)\n%d is the day of the month, zero-padded, %e is blank-padded.\n%u is the numeric day of week from 1 (Monday) to 7, %w from 0 (Sunday) to 6.\n%a is the locale's abbreviated name of the day of week, %A the full name.\n%U is the week number starting on Sunday, %W starting on Monday,\n %V the week number according to ISO 8601.\n%j is the day of the year.\n\n%H is the hour on a 24-hour clock, %I is on a 12-hour clock, %k is like %H\n only blank-padded, %l is like %I blank-padded.\n%p is the locale's equivalent of either AM or PM.\n%q is the calendar quarter (1–4).\n%M is the minute (00-59).\n%S is the second (00-59; 00-60 on platforms with leap seconds)\n%s is the number of seconds since 1970-01-01 00:00:00 +0000.\n%N is the nanosecond, %6N the microsecond, %3N the millisecond, etc.\n%Z is the time zone abbreviation, %z is the numeric form.\n\n%c is the locale's date and time format.\n%x is the locale's \"preferred\" date format.\n%D is like \"%m/%d/%y\".\n%F is the ISO 8601 date format (like \"%+4Y-%m-%d\").\n\n%R is like \"%H:%M\", %T is like \"%H:%M:%S\", %r is like \"%I:%M:%S %p\".\n%X is the locale's \"preferred\" time format.\n\nFinally, %n is a newline, %t is a tab, %% is a literal %, and\nunrecognized %-sequences stand for themselves.\n\nA %-sequence can contain optional flags, field width, and a modifier\n(in that order) after the `%'.  The flags are:\n\n`-' Do not pad the field.\n`_' Pad with spaces.\n`0' Pad with zeros.\n`+' Pad with zeros and put `+' before nonnegative year numbers with >4 digits.\n`^' Use upper case characters if possible.\n`#' Use opposite case characters if possible.\n\nA field width N is an unsigned decimal integer with a leading digit\nnonzero.  %NX is like %X, but takes up at least N positions.  The\nfield width is (on GNU/Linux and some other systems) in measured in\nbytes, not characters.  It depends on the locale what the width (in\ncharacters) %NX will end up being, especially when there are non-ASCII\ncharacters in %X.\n\nThe modifiers are:\n\n`E' Use the locale's alternative version.\n`O' Use the locale's number symbols.\n\nFor example, to produce full ISO 8601 format, use \"%FT%T%z\".\n\nusage: (format-time-string FORMAT-STRING &optional TIME ZONE)  ",
       (format_string: Lisp_Object, timeval: Lisp_Object, zone: Lisp_Object),
       {
  let t: timespec = lisp_time_argument (timeval);
  let mut tm: tm = std::mem::zeroed ();

  CHECK_STRING (format_string);
  /* C reassigns its parameter; the port shadows it (the DEFUN! macro's parameters are
     immutable).  */
  let format_string: Lisp_Object = code_convert_string_norecord (format_string,
								 Vlocale_coding_system, true);
  return format_time_string (SSDATA (format_string), SBYTES (format_string),
			     t, zone, &mut tm);
});

DEFUN! ("decode-time", Fdecode_time, Sdecode_time, 0, 3, 0,
       doc: "Decode a timestamp into (SEC MINUTE HOUR DAY MONTH YEAR DOW DST UTCOFF).\nThe optional TIME is the time value to convert.  See\n`format-time-string' for the various forms of a time value.\n\nThe optional ZONE is omitted or nil for Emacs local time, t for\nUniversal Time, `wall' for system wall clock time, or a string as in\nthe TZ environment variable.  It can also be a list (as from\n`current-time-zone') or an integer (the UTC offset in seconds) applied\nwithout consideration for daylight saving time.\n\nThe optional FORM specifies the form of the SEC member.  If `integer',\nSEC is an integer; if t, SEC is an integer or (TICKS . HZ) timestamp\nwith the same precision as TIME.  An omitted or nil FORM is currently\ntreated like `integer', but this may change in future Emacs versions.\n\nTo access (or alter) the elements in the time value, the\n`decoded-time-second', `decoded-time-minute', `decoded-time-hour',\n`decoded-time-day', `decoded-time-month', `decoded-time-year',\n`decoded-time-weekday', `decoded-time-dst' and `decoded-time-zone'\naccessors can be used.\n\nThe list has the following nine members: SEC is an integer or\nLisp timestamp representing a nonnegative value less than 60\n(or less than 61 if the operating system supports leap seconds).\nMINUTE is an integer between 0 and 59.  HOUR is an integer\nbetween 0 and 23.  DAY is an integer between 1 and 31.  MONTH is an\ninteger between 1 and 12.  YEAR is the year number, an integer; 0\nrepresents 1 BC.  DOW is the day of week, an integer between 0 and 6,\nwhere 0 is Sunday.  DST is t if daylight saving time is in effect,\nnil if it is not in effect, and -1 if daylight saving information is\nnot available.  UTCOFF is an integer indicating the UTC offset in\nseconds, i.e., the number of seconds east of Greenwich.  (Note that\nCommon Lisp has different meanings for DOW and UTCOFF, and its\nSEC is always an integer between 0 and 59.)\n\nusage: (decode-time &optional TIME ZONE FORM)  ",
       (specified_time: Lisp_Object, zone: Lisp_Object, form: Lisp_Object),
       {
  /* Convert SPECIFIED_TIME to TIME_SPEC and HZ;
     if HZ != 1 also set TH.ticks.  */
  let mut time_spec: time_t = 0;
  let mut hz: Lisp_Object = lisp_h_Qnil ();
  let mut th: ticks_hz = ticks_hz { ticks: lisp_h_Qnil (), hz: lisp_h_Qnil () };  /* C's UNINIT */
  if EQ (form, Qt ())
    {
      th = decode_lisp_time (specified_time, CFORM_TICKS_HZ).th;
      let ts: timespec = ticks_hz_to_timespec (th.ticks, th.hz);
      if ! timespec_valid_p (ts)
	{ time_overflow (); }
      time_spec = ts.tv_sec;
      hz = th.hz;
    }
  else
    {
      time_spec = lisp_seconds_argument (specified_time);
      hz = make_fixnum (1);
    }

  /* Compute broken-down local time LOCAL_TM from TIME_SPEC and ZONE.  */
  let mut local_tm: tm = std::mem::zeroed ();
  let mut gmt_tm: tm = std::mem::zeroed ();
  let tz: timezone_t = tzlookup (zone, false);
  let tm: *mut tm = emacs_localtime_rz (tz, &time_spec, &raw mut local_tm);
  xtzfree (tz);

  if tm.is_null ()
    { time_error (*errno ()); }

  /* Let YEAR = LOCAL_TM.tm_year + TM_YEAR_BASE.  */
  let year: Lisp_Object;
  if FASTER_TIMEFNS != 0
      && (MOST_NEGATIVE_FIXNUM - TM_YEAR_BASE as EMACS_INT) <= local_tm.tm_year as EMACS_INT
      && local_tm.tm_year as EMACS_INT <= (MOST_POSITIVE_FIXNUM - TM_YEAR_BASE as EMACS_INT)
    {
      /* Avoid overflow when INT_MAX - TM_YEAR_BASE < local_tm.tm_year.  */
      let tm_year_base: EMACS_INT = TM_YEAR_BASE as EMACS_INT;
      year = make_fixnum ((local_tm.tm_year as EMACS_INT + tm_year_base));
    }
  else
    {
      mpz_set_si (mpz[0].as_mut_ptr (), local_tm.tm_year as c_long);
      mpz_add_ui (mpz[0].as_mut_ptr (), mpz[0].as_ptr (), TM_YEAR_BASE as c_ulong);
      year = make_integer_mpz ();
    }

  /* Compute SEC from LOCAL_TM.tm_sec and HZ.  */
  let sec: Lisp_Object;
  if BASE_EQ (hz, make_fixnum (1))
    { sec = make_fixnum (local_tm.tm_sec as EMACS_INT); }
  else
    {
      /* Let TICKS = HZ * LOCAL_TM.tm_sec + mod (TH.ticks, HZ)
	 and SEC = (TICKS . HZ).  */
      let ticks: Lisp_Object;
      let mut n: intmax_t = 0;
      let mut ticks_fast: bool = false;
      if FASTER_TIMEFNS != 0 && FIXNUMP (th.ticks) && FIXNUMP (hz)
	{
	  let (ckd_mul_value, ckd_mul_overflow) =
	    (XFIXNUM (hz) as intmax_t).overflowing_mul (local_tm.tm_sec as intmax_t);
	  n = ckd_mul_value;
	  if !ckd_mul_overflow
	    {
	      let (ckd_add_value, ckd_add_overflow) = n.overflowing_add ((
		XFIXNUM (th.ticks) % XFIXNUM (hz)
		+ (if XFIXNUM (th.ticks) % XFIXNUM (hz) < 0 { XFIXNUM (hz) } else { 0 })) as intmax_t);
	      n = ckd_add_value;
	      if !ckd_add_overflow
		{ ticks_fast = true; }
	    }
	}
      if ticks_fast
	{ ticks = make_int (n); }
      else
	{
	  mpz_fdiv_r (mpz[0].as_mut_ptr (),
		      bignum_integer (&raw mut mpz[0], th.ticks),
		      bignum_integer (&raw mut mpz[1], hz));
	  mpz_addmul_ui (mpz[0].as_mut_ptr (), bignum_integer (&raw mut mpz[1], hz),
			 local_tm.tm_sec as c_ulong);
	  ticks = make_integer_mpz ();
	}
      sec = Fcons (ticks, hz);
    }

  return CALLN! (Flist,
		 sec,
		 make_fixnum (local_tm.tm_min as EMACS_INT),
		 make_fixnum (local_tm.tm_hour as EMACS_INT),
		 make_fixnum (local_tm.tm_mday as EMACS_INT),
		 make_fixnum ((local_tm.tm_mon + 1) as EMACS_INT),
		 year,
		 make_fixnum (local_tm.tm_wday as EMACS_INT),
		 (if local_tm.tm_isdst < 0 { make_fixnum (-1) }
		  else if local_tm.tm_isdst == 0 { Qnil () } else { Qt () }),
		 (if HAVE_TM_GMTOFF { make_fixnum (tm_gmtoff (&raw mut local_tm) as EMACS_INT) }
		  else if ! gmtime_r (&time_spec, &raw mut gmt_tm).is_null ()
		  { make_fixnum (tm_diff (&raw mut local_tm, &raw mut gmt_tm) as EMACS_INT) }
		  else { Qnil () }));
});

/* Return OBJ - OFFSET, checking that OBJ is a valid integer and that
   the result is representable as an int.  0 <= OFFSET <= TM_YEAR_BASE.  */
unsafe fn check_tm_member (obj: Lisp_Object, offset: c_int) -> c_int
{
  if FASTER_TIMEFNS != 0 && INT_MAX as EMACS_INT <= MOST_POSITIVE_FIXNUM - TM_YEAR_BASE as EMACS_INT
    {
      CHECK_FIXNUM (obj);
      let n: EMACS_INT = XFIXNUM (obj);
      let i: c_int;
      /* C's `ckd_sub (&i, n, offset)`: the difference is computed in EMACS_INT and
	 stored in `int` when it fits (gnulib intprops' rule).  */
      let (ckd_sub_value, ckd_sub_overflow) = n.overflowing_sub (offset as EMACS_INT);
      if ckd_sub_overflow || ckd_sub_value < (c_int::MIN as EMACS_INT)
	 || (c_int::MAX as EMACS_INT) < ckd_sub_value
	{ time_overflow (); }
      i = ckd_sub_value as c_int;
      return i;
    }
  else
    {
      CHECK_INTEGER (obj);
      mpz_sub_ui (mpz[0].as_mut_ptr (), bignum_integer (&raw mut mpz[0], obj),
		  offset as c_ulong);
      if mpz_fits_sint_p (mpz[0].as_ptr ()) == 0
	{ time_overflow (); }
      return mpz_get_si (mpz[0].as_ptr ()) as c_int;
    }
}

DEFUN! ("encode-time", Fencode_time, Sencode_time, 1, MANY, 0,
       doc: "Convert TIME to a timestamp.\n\nTIME is a list (SECOND MINUTE HOUR DAY MONTH YEAR IGNORED DST ZONE)\nin the style of `decode-time', so that (encode-time (decode-time ...)) works.\nIn this list, ZONE can be nil for Emacs local time, t for Universal\nTime, `wall' for system wall clock time, or a string as in the TZ\nenvironment variable.  ZONE can also be a list (as from\n`current-time-zone') or an integer (as from `decode-time') applied\nwithout consideration for daylight saving time.  If ZONE specifies a\ntime zone with daylight-saving transitions, DST is t for daylight\nsaving time, nil for standard time, and -1 to cause the daylight\nsaving flag to be guessed.\n\nTIME can also be a list (SECOND MINUTE HOUR DAY MONTH YEAR), which is\nequivalent to (SECOND MINUTE HOUR DAY MONTH YEAR nil -1 nil).\n\nAs an obsolescent calling convention, if this function is called with\n6 or more arguments, the first 6 arguments are SECOND, MINUTE, HOUR,\nDAY, MONTH, and YEAR, and specify the components of a decoded time.\nIf there are more than 6 arguments the *last* argument is used as ZONE\nand any other extra arguments are ignored, so that (apply\n#\\='encode-time (decode-time ...)) works.  In this obsolescent\nconvention, DST is -1 and ZONE defaults to nil.\n\nThe range of supported years is at least 1970 to the near future.\nOut-of-range values for SECOND through MONTH are brought into range\nvia date arithmetic.  This can be tricky especially when combined with\nDST; see Info node `(elisp)Time Conversion' for details and caveats.\n\nusage: (encode-time TIME &rest OBSOLESCENT-ARGUMENTS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut tm: tm = std::mem::zeroed ();
  let mut zone: Lisp_Object = Qnil ();
  let mut a: Lisp_Object = *args;
  let secarg: Lisp_Object;
  let minarg: Lisp_Object;
  let hourarg: Lisp_Object;
  let mdayarg: Lisp_Object;
  let monarg: Lisp_Object;
  let yeararg: Lisp_Object;
  tm.tm_isdst = -1;

  if nargs == 1
    {
      let mut tail: Lisp_Object = a;
      let mut i: c_int = 0;
      loop
	{
	  if i >= 6
	    { break; }
	  CHECK_CONS (tail);
	  i += 1;
	  tail = XCDR (tail);
	}
      secarg = XCAR (a); a = XCDR (a);
      minarg = XCAR (a); a = XCDR (a);
      hourarg = XCAR (a); a = XCDR (a);
      mdayarg = XCAR (a); a = XCDR (a);
      monarg = XCAR (a); a = XCDR (a);
      yeararg = XCAR (a); a = XCDR (a);
      if ! NILP (a)
	{
	  CHECK_CONS (a);
	  a = XCDR (a);
	  CHECK_CONS (a);
	  let dstflag: Lisp_Object = XCAR (a); a = XCDR (a);
	  CHECK_CONS (a);
	  zone = XCAR (a);
	  if SYMBOLP (dstflag) && !FIXNUMP (zone) && !CONSP (zone)
	    { tm.tm_isdst = (!NILP (dstflag)) as c_int; }
	}
    }
  else if nargs < 6
    { xsignal2 (Qwrong_number_of_arguments (), Qencode_time (), make_fixnum (nargs as EMACS_INT)); }
  else
    {
      if 6 < nargs
	{ zone = *args.add ((nargs - 1) as usize); }
      secarg = a;
      minarg = *args.add (1);
      hourarg = *args.add (2);
      mdayarg = *args.add (3);
      monarg = *args.add (4);
      yeararg = *args.add (5);
    }

  /* Let SEC = floor (TH.ticks / HZ), with SUBSECTICKS the remainder.  */
  let th: ticks_hz = decode_lisp_time (secarg, CFORM_TICKS_HZ).th;
  let hz: Lisp_Object = th.hz;
  let sec: Lisp_Object;
  let subsecticks: Lisp_Object;
  if FASTER_TIMEFNS != 0 && BASE_EQ (hz, make_fixnum (1))
    {
      sec = th.ticks;
      subsecticks = make_fixnum (0);
    }
  else
    {
      mpz_fdiv_qr (mpz[0].as_mut_ptr (), mpz[1].as_mut_ptr (),
		   bignum_integer (&raw mut mpz[0], th.ticks),
		   bignum_integer (&raw mut mpz[1], hz));
      sec = make_integer_mpz ();
      mpz_swap (mpz[0].as_mut_ptr (), mpz[1].as_mut_ptr ());
      subsecticks = make_integer_mpz ();
    }
  tm.tm_sec  = check_tm_member (sec, 0);
  tm.tm_min  = check_tm_member (minarg, 0);
  tm.tm_hour = check_tm_member (hourarg, 0);
  tm.tm_mday = check_tm_member (mdayarg, 0);
  tm.tm_mon  = check_tm_member (monarg, 1);
  tm.tm_year = check_tm_member (yeararg, TM_YEAR_BASE);

  let tz: timezone_t = tzlookup (zone, false);
  tm.tm_wday = -1;
  let value: time_t = mktime_z (tz, &mut tm);
  xtzfree (tz);

  if tm.tm_wday < 0
    { time_error (*errno ()); }

  if BASE_EQ (hz, make_fixnum (1))
    { return if current_time_list { list2 (hi_time (value), lo_time (value)) }
	     else { INT_TO_INTEGER (value as EMACS_INT) }; }
  else
    {
      let val1: ticks_hz = ticks_hz { ticks: INT_TO_INTEGER (value as EMACS_INT),
				      hz: make_fixnum (1) };
      let secticks: Lisp_Object = ticks_hz_hz_ticks (val1, hz);
      let ticks: Lisp_Object = lispint_arith (secticks, subsecticks, false);
      return Fcons (ticks, hz);
    }
});

DEFUN! ("time-convert", Ftime_convert, Stime_convert, 1, 2, 0,
       doc: "Convert TIME value to a Lisp timestamp of the given FORM.\nTruncate the returned value toward minus infinity.\n\nIf FORM is a positive integer, return a pair of integers (TICKS . FORM),\nwhere TICKS is the number of clock ticks and FORM is the clock frequency\nin ticks per second.\n\nIf FORM is t, return (TICKS . PHZ), where PHZ is a suitable clock\nfrequency in ticks per second.\n\nIf FORM is `integer', return an integer count of seconds.\n\nIf FORM is `list', return an integer list (HIGH LOW USEC PSEC), where\nHIGH has the most significant bits of the seconds, LOW has the least\nsignificant 16 bits, and USEC and PSEC are the microsecond and\npicosecond counts.\n\nIf FORM is nil, the behavior depends on `current-time-list',\nbut new code should not rely on it.  ",
       (time: Lisp_Object, form: Lisp_Object),
       {
  /* FIXME: Any reason why we don't offer a `float` output format option as
     well, since we accept it as input?  */
  let t: ticks_hz = decode_lisp_time (time, CFORM_TICKS_HZ).th;
  let mut form: Lisp_Object = if !NILP (form) { maybe_remove_pos_from_symbol (form) }
			      else if current_time_list { Qlist () } else { Qt () };
  if BASE_EQ (form, Qlist ())
    { return ticks_hz_list4 (t.ticks, t.hz); }
  if BASE_EQ (form, Qinteger ())
    { return if FASTER_TIMEFNS != 0 && INTEGERP (time) { time } else { ticks_hz_seconds (t) }; }
  if BASE_EQ (form, Qt ())
    { form = t.hz; }
  if FASTER_TIMEFNS != 0 && CONSP (time) && BASE_EQ (form, XCDR (time))
    { return time; }
  return Fcons (ticks_hz_hz_ticks (t, form), form);
});

DEFUN! ("current-time", Fcurrent_time, Scurrent_time, 0, 0, 0,
       doc: "Return the current time, as the number of seconds since 1970-01-01 00:00:00.\nIf the variable `current-time-list' is nil, the time is returned as a\npair of integers (TICKS . HZ), where TICKS counts clock ticks and HZ\nis the clock ticks per second.  Otherwise, the time is returned as a\nlist of integers (HIGH LOW USEC PSEC) where HIGH has the most\nsignificant bits of the seconds, LOW has the least significant 16\nbits, and USEC and PSEC are the microsecond and picosecond counts.\n\nYou can use `time-convert' to get a particular timestamp form\nregardless of the value of `current-time-list'.  ",
       (),
       {
  return make_lisp_time (current_timespec ());
});

/* C 1832-1842: `#ifdef CLOCKS_PER_SEC` (defined in the frozen `<config.h>`) keeps this
   subr live.  */
DEFUN! ("current-cpu-time", Fcurrent_cpu_time, Scurrent_cpu_time, 0, 0, 0,
       doc: "Return the current CPU time along with its resolution.\nThe return value is a pair (CPU-TICKS . TICKS-PER-SEC).\nThe CPU-TICKS counter can wrap around, so values cannot be meaningfully\ncompared if too much time has passed between them.  ",
       (),
       {
  return Fcons (make_int (clock () as intmax_t), make_int (CLOCKS_PER_SEC as intmax_t));
});

DEFUN! ("current-time-string", Fcurrent_time_string, Scurrent_time_string, 0, 2, 0,
       doc: "Return the current local time, as a human-readable string.\nPrograms can use this function to decode a time,\nsince the number of columns in each field is fixed\nif the year is in the range 1000-9999.\nThe format is `Sun Sep 16 01:03:52 1973'.\nHowever, see also the functions `decode-time' and `format-time-string'\nwhich provide a much more powerful and general facility.\n\nIf SPECIFIED-TIME is given, it is the time value to format instead of\nthe current time.  See `format-time-string' for the various forms of a\ntime value.\n\nThe optional ZONE is omitted or nil for Emacs local time, t for\nUniversal Time, `wall' for system wall clock time, or a string as in\nthe TZ environment variable.  It can also be a list (as from\n`current-time-zone') or an integer (as from `decode-time') applied\nwithout consideration for daylight saving time.  ",
       (specified_time: Lisp_Object, zone: Lisp_Object),
       {
  let value: time_t = lisp_seconds_argument (specified_time);
  let tz: timezone_t = tzlookup (zone, false);

  /* Convert to a string in ctime format, except without the trailing
     newline, and without the 4-digit year limit.  Don't use asctime
     or ctime, as they might dump core if the year is outside the
     range -999 .. 9999.  */
  let mut tm: tm = std::mem::zeroed ();
  let tmp: *mut tm = emacs_localtime_rz (tz, &value, &raw mut tm);
  xtzfree (tz);
  if tmp.is_null ()
    { time_error (*errno ()); }

  static wday_name: [[u8; 4]; 7] =
    [ *b"Sun\0", *b"Mon\0", *b"Tue\0", *b"Wed\0", *b"Thu\0", *b"Fri\0", *b"Sat\0" ];
  static mon_name: [[u8; 4]; 12] =
    [ *b"Jan\0", *b"Feb\0", *b"Mar\0", *b"Apr\0", *b"May\0", *b"Jun\0",
      *b"Jul\0", *b"Aug\0", *b"Sep\0", *b"Oct\0", *b"Nov\0", *b"Dec\0" ];
  let year_base: intmax_t = TM_YEAR_BASE as intmax_t;
  const BUF_LEN: usize = std::mem::size_of_val (&*b"Mon Apr 30 12:49:17 ")
    + INT_STRLEN_BOUND! (c_int) + 1;
  let mut buf: [c_char; BUF_LEN] = [0; BUF_LEN];
  let len: c_int = sprintf (buf.as_mut_ptr (), c"%s %s%3d %02d:%02d:%02d %jd".as_ptr (),
			    wday_name[tm.tm_wday as usize].as_ptr () as *const c_char,
			    mon_name[tm.tm_mon as usize].as_ptr () as *const c_char,
			    tm.tm_mday as c_int, tm.tm_hour as c_int, tm.tm_min as c_int,
			    tm.tm_sec as c_int, tm.tm_year as intmax_t + year_base);

  return make_unibyte_string (buf.as_ptr (), len as ptrdiff_t);
});

DEFUN! ("current-time-zone", Fcurrent_time_zone, Scurrent_time_zone, 0, 2, 0,
       doc: "Return the offset and name for the local time zone.\nThis returns a list of the form (OFFSET NAME).\nOFFSET is an integer number of seconds ahead of UTC (east of Greenwich).\n    A negative value means west of Greenwich.\nNAME is a string giving the name of the time zone.\nIf SPECIFIED-TIME is given, the time zone offset is determined from it\ninstead of using the current time.  The argument should be a Lisp\ntime value; see `format-time-string' for the various forms of a time\nvalue.\n\nThe optional ZONE is omitted or nil for Emacs local time, t for\nUniversal Time, `wall' for system wall clock time, or a string as in\nthe TZ environment variable.  It can also be a list (as from\n`current-time-zone') or an integer (as from `decode-time') applied\nwithout consideration for daylight saving time.\n\nSome operating systems cannot provide all this information to Emacs;\nin this case, `current-time-zone' returns a list containing nil for\nthe data it can't find.  ",
       (specified_time: Lisp_Object, zone: Lisp_Object),
       {
  let value: timespec;
  let mut local_tm: tm = std::mem::zeroed ();
  let mut gmt_tm: tm = std::mem::zeroed ();
  let mut zone_offset: Lisp_Object;
  let mut zone_name: Lisp_Object;

  zone_offset = Qnil ();
  value = make_timespec (lisp_seconds_argument (specified_time), 0);
  const ZONE_FMT: &[u8; 3] = b"%Z\0";
  zone_name = format_time_string (ZONE_FMT.as_ptr () as *const c_char,
				  (std::mem::size_of_val (ZONE_FMT) - 1) as ptrdiff_t,
				  value, zone, &raw mut local_tm);

  /* gmtime_r expects a pointer to time_t, but tv_sec of struct
     timespec on some systems (MinGW) is a 64-bit field.  */
  let tsec: time_t = value.tv_sec;
  if HAVE_TM_GMTOFF || ! gmtime_r (&tsec, &raw mut gmt_tm).is_null ()
    {
      let offset: c_long = if HAVE_TM_GMTOFF
			   { tm_gmtoff (&raw mut local_tm) }
			   else { tm_diff (&raw mut local_tm, &raw mut gmt_tm) as c_long };
      zone_offset = make_fixnum (offset as EMACS_INT);
      if SCHARS (zone_name) == 0
	{
	  /* No local time zone name is available; use numeric zone instead.  */
	  let hour: c_long = offset / 3600;
	  let min_sec: c_int = (offset % 3600) as c_int;
	  const BUF2_LEN: usize = INT_STRLEN_BOUND! (c_long)
	    + std::mem::size_of_val (&*b"5959");
	  let mut buf: [c_char; BUF2_LEN] = [0; BUF2_LEN];
	  let mut buflen: c_int = 0;
	  buf[buflen as usize] = (if offset < 0 { b'-' } else { b'+' }) as c_char;
	  buflen += 1;
	  buflen += sprintf (buf.as_mut_ptr ().add (buflen as usize),
			     c"%.2ld".as_ptr (), eabs! (hour));
	  if min_sec != 0
	    {
	      let amin_sec: c_int = eabs! (min_sec);
	      let min: c_int = amin_sec / 60;
	      let sec: c_int = amin_sec % 60;
	      buf[buflen as usize] = ((b'0' + (min / 10) as u8)) as c_char;
	      buflen += 1;
	      buf[buflen as usize] = ((b'0' + (min % 10) as u8)) as c_char;
	      buflen += 1;
	      if sec != 0
		{
		  buf[buflen as usize] = ((b'0' + (sec / 10) as u8)) as c_char;
		  buflen += 1;
		  buf[buflen as usize] = ((b'0' + (sec % 10) as u8)) as c_char;
		  buflen += 1;
		}
	    }
	  zone_name = make_string (buf.as_ptr (), buflen as ptrdiff_t);
	}
    }

  return list2 (zone_offset, zone_name);
});

DEFUN! ("set-time-zone-rule", Fset_time_zone_rule, Sset_time_zone_rule, 1, 1, 0,
       doc: "Set the Emacs local time zone using TZ, a string specifying a time zone rule.\nIf TZ is nil or `wall', use system wall clock time; this differs from\nthe usual Emacs convention where nil means current local time.  If TZ\nis t, use Universal Time.  If TZ is a list (as from\n`current-time-zone') or an integer (as from `decode-time'), use the\nspecified time zone without consideration for daylight saving time.\n\nInstead of calling this function, you typically want something else.\nTo temporarily use a different time zone rule for just one invocation\nof `decode-time', `encode-time', or `format-time-string', pass the\nfunction a ZONE argument.  To change local time consistently\nthroughout Emacs, call (setenv \"TZ\" TZ): this changes both the\nenvironment of the Emacs process and the variable\n`process-environment', whereas `set-time-zone-rule' affects only the\nformer.  ",
       (tz: Lisp_Object),
       {
  tzlookup (if NILP (tz) { Qwall () } else { tz }, true);
  return Qnil ();
});

/* C 1985-1991: `#ifndef MSDOS` (MSDOS is undefined, so this is the live text).  */
/* A buffer holding a string of the form "TZ=value", intended
   to be part of the environment.  If TZ is supposed to be unset,
   the buffer string is "tZ=".  */
static mut tzvalbuf: *mut c_char = std::ptr::null_mut ();

/* Get the local time zone rule.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_getenv_TZ () -> *mut c_char
{
  /* C 1997-2001: the frozen build takes `#ifndef MSDOS`; the `#else` arm is carried.  */
  #[cfg (not (feature = "msdos"))]
  { return if *tzvalbuf == b'T' as c_char { tzvalbuf.add (tzeqlen as usize) }
	   else { std::ptr::null_mut () }; }
  #[cfg (feature = "msdos")]
  { return getenv (c"TZ".as_ptr ()); }
}

/* Set the local time zone rule to TZSTRING, which can be null to
   denote wall clock time.  Do not record the setting in LOCAL_TZ.

   This function is not thread-safe, in theory because putenv is not,
   but mostly because of the static storage it updates.  Other threads
   that invoke localtime etc. may be adversely affected while this
   function is executing.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_setenv_TZ (tzstring: *const c_char) -> c_int
{
  /* C 2015-2084: the frozen build takes the `#ifndef MSDOS` arm; the `#else` arm is
     carried.  */
  #[cfg (not (feature = "msdos"))]
  {
    static mut tzvalbufsize: ptrdiff_t = 0;
    let tzstringlen: ptrdiff_t = if !tzstring.is_null () { strlen (tzstring) as ptrdiff_t } else { 0 };
    let mut tzval: *mut c_char = tzvalbuf;
    let new_tzvalbuf: bool = tzvalbufsize <= (tzeqlen as ptrdiff_t) + tzstringlen;

    if new_tzvalbuf
      {
	/* Do not attempt to free the old tzvalbuf, since another thread
	   may be using it.  In practice, the first allocation is large
	   enough and memory does not leak.  */
	tzval = xpalloc (std::ptr::null_mut (), &raw mut tzvalbufsize,
			 (tzeqlen as ptrdiff_t) + tzstringlen - tzvalbufsize + 1,
			 -1, 1) as *mut c_char;
	tzvalbuf = tzval;
	*tzval.add (1) = b'Z' as c_char;
	*tzval.add (2) = b'=' as c_char;
      }

    if !tzstring.is_null ()
      {
	/* Modify TZVAL in place.  Although this is dicey in a
	   multithreaded environment, we know of no portable alternative.
	   Calling putenv or setenv could crash some other thread.  */
	*tzval = b'T' as c_char;
	strcpy (tzval.add (tzeqlen as usize), tzstring);
      }
    else
      {
	/* Turn 'TZ=whatever' into an empty environment variable 'tZ='.
	   Although this is also dicey, calling unsetenv here can crash Emacs.
	   See Bug#8705.  */
	*tzval = b't' as c_char;
	*tzval.add (tzeqlen as usize) = 0;
      }


    /* C 2051-2065: the frozen build takes `#ifndef WINDOWSNT` (`new_tzvalbuf`); the
       `#else` arm (`true`) is carried.  */
    #[cfg (not (feature = "windowsnt"))]
    /* Modifying *TZVAL merely requires calling tzset (which is the
       caller's responsibility).  However, modifying TZVAL requires
       calling putenv; although this is not thread-safe, in practice this
       runs only on startup when there is only one thread.  */
    let need_putenv: bool = new_tzvalbuf;
    #[cfg (feature = "windowsnt")]
    /* MS-Windows 'putenv' copies the argument string into a block it
       allocates, so modifying *TZVAL will not change the environment.
       However, the other threads run by Emacs on MS-Windows never call
       'xputenv' or 'putenv' or 'unsetenv', so the original cause for the
       dicey in-place modification technique doesn't exist there in the
       first place.  */
    let need_putenv: bool = true;
    if need_putenv
      { xputenv (tzval); }

    return 0;
  }
  #[cfg (feature = "msdos")]
  {
    /* Happily, there are no threads on MS-DOS that might be contending
       with Emacs for access to TZ.  Call putenv to modify TZ: the code
       above is not only unnecessary but results in modifications being
       omitted in consequence of an internal environment counter
       remaining unchanged despite DJGPP being all too ready to reuse
       preexisting environment storage.  */
    USE_SAFE_ALLOCA! (sa_avail, sa_count);
    let buf: *mut c_char = SAFE_ALLOCA! (sa_avail, (tzeqlen as usize) + strlen (tzstring) + 1) as *mut c_char;
    strcpy (buf, c"TZ=".as_ptr ());
    strcpy (buf.add (tzeqlen as usize), tzstring);
    xputenv (buf);
    SAFE_FREE! (sa_count);
    return 0;
  }
}

#[cfg (feature = "ztrillion_mpz")]
unsafe extern "C" fn syms_of_timefns_for_pdumper ()
{
  mpz_init_set_ui (ztrillion.as_mut_ptr (), 1000000);
  mpz_mul_ui (ztrillion.as_mut_ptr (), ztrillion.as_ptr (), 1000000);
}

/* C's DEFVAR_BOOL variable lives in the generated globals.h in C; the port's globals.rs
   mirrors it as a declaration and the owning unit defines it (P-21/§3.29).  */
#[unsafe(no_mangle)]
pub static mut current_time_list: bool = false;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_timefns ()
{
  /* C 2099-2106: the frozen build compiles neither `#ifndef` (the object macros are the
     live arm); both are carried (P-51).  */
  #[cfg (feature = "no_fixnum_timespec_hz")]
  {
    timespec_hz = make_int (TIMESPEC_HZ as intmax_t);
    staticpro (&raw const timespec_hz);
  }
  #[cfg (feature = "no_fixnum_trillion")]
  {
    trillion = make_int (1000000000000);
    staticpro (&raw const trillion);
  }

  DEFSYM! (Qencode_time, c"encode-time");

  DEFVAR_BOOL! (c"current-time-list", current_time_list,
		"Whether `current-time' should return list or (TICKS . HZ) form.\n\nThis boolean variable is a transition aid.  If nil (the default),\n`current-time' and related functions return timestamps in (TICKS . HZ)\nform.  If t, these functions return in list form, typically (HIGH LOW\nUSEC PSEC), for behavior compatible with previous Emacs versions.  As it\nis merely a transition aid, this variable is planned to be removed in\nsome future Emacs version.  ");
  current_time_list = CURRENT_TIME_LIST;

  defsubr (&raw mut Scurrent_time);
  defsubr (&raw mut Scurrent_cpu_time);
  defsubr (&raw mut Stime_convert);
  defsubr (&raw mut Stime_add);
  defsubr (&raw mut Stime_subtract);
  defsubr (&raw mut Stime_less_p);
  defsubr (&raw mut Stime_equal_p);
  defsubr (&raw mut Sformat_time_string);
  defsubr (&raw mut Sfloat_time);
  defsubr (&raw mut Sdecode_time);
  defsubr (&raw mut Sencode_time);
  defsubr (&raw mut Scurrent_time_string);
  defsubr (&raw mut Scurrent_time_zone);
  defsubr (&raw mut Sset_time_zone_rule);

  flt_radix_power = make_nil_vector (flt_radix_power_size as isize);
  staticpro (&raw const flt_radix_power);

  #[cfg (feature = "ztrillion_mpz")]
  pdumper_do_now_and_after_load (Some (syms_of_timefns_for_pdumper));
}
