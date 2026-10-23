/* Written by Paul Eggert.  */

// femacs: port of lib/time-internal.h @ fd7afced403 (gnulib).
//
// Header mirror (PORTING.md 0.1): gnulib's time-zone rule and the two entry points the
// time_rz module exports, plus the `local_tz` cookie.  `timezone_t` (and `struct tm`,
// which the "Only the address matters" comment below does not use) come from the time.h
// mirror, which is what C's `#include <time.h>` chain supplies; `bool` is the prelude's.

include!("time.in.rs");

/* A time zone rule.  */
#[repr(C)]
pub struct tm_zone
{
  /* More abbreviations, should they be needed.  Their TZ_IS_SET
     members are zero.  */
  pub next: *mut tm_zone,

  /* If nonzero, the rule represents the TZ environment variable set
     to the first "abbreviation" (this may be the empty string).
     Otherwise, it represents an unset TZ.  */
  pub tz_is_set: c_char,

  /* A sequence of null-terminated strings packed next to each other.
     The strings are followed by an extra null byte.  If TZ_IS_SET,
     there must be at least one string and the first string (which is
     actually a TZ environment value) may be empty.  Otherwise all
     strings must be nonempty.

     Abbreviations are stored here even on platforms with tm_zone, because
     otherwise tm_zone values would be dead after changing TZ and calling
     tzset.  Abbreviations never move once allocated, and are live
     until tzfree is called.  */
  pub abbrs: [c_char; 0],  // FLEXIBLE_ARRAY_MEMBER
}

unsafe extern "C" {
  /* timezone_t set_tz (timezone_t);  */
  pub fn set_tz (tz: timezone_t) -> timezone_t;
  /* bool revert_tz (timezone_t);  */
  pub fn revert_tz (tz: timezone_t) -> bool;
}

/* Magic cookie timezone_t value, for local time.  It differs from
   NULL and from all other timezone_t values.  Only the address
   matters; the pointer is never dereferenced.  */
macro_rules! local_tz { () => { 1 as timezone_t } }
