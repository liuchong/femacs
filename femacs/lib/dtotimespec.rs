/* Convert double to timespec.

   Copyright (C) 2011-2026 Free Software Foundation, Inc.

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
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/dtotimespec.c @ fd7afced403.  `time_t` is
// `i64`, so `<intprops.h>`'s `TYPE_MINIMUM`/`TYPE_MAXIMUM` reduce to the type's own limits
// (the nanosleep.rs/mktime.rs precedent, P-26); `make_timespec`/`TIMESPEC_HZ` come from the
// timespec.h mirror, whose declarations are in `pub mod timespec_decls` + glob import, so
// this unit's own `dtotimespec` definition shadows the declaration (PORTING.md 0.1-10).
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/timespec.rs");   // "timespec.h", <stdckdint.h>, "intprops.h"

macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }

/* Convert the double value SEC to a struct timespec.  Round toward
   positive infinity.  On overflow, return an extremal value.  */

/* struct timespec dtotimespec (double sec)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dtotimespec (sec: c_double) -> timespec
{
  if ! ((TYPE_MINIMUM! (time_t) as c_double) < sec)
    { return make_timespec (TYPE_MINIMUM! (time_t), 0); }
  else if ! (sec < 1.0 + TYPE_MAXIMUM! (time_t) as c_double)
    { return make_timespec (TYPE_MAXIMUM! (time_t), TIMESPEC_HZ as c_long - 1); }
  else
    {
      /* C 38-40: `time_t s = sec;` (the C conversion truncates toward zero), then
         `double frac = TIMESPEC_HZ * (sec - s);` and `long ns = frac;`.  */
      let mut s: time_t = sec as time_t;
      let frac: c_double = TIMESPEC_HZ as c_double * (sec - s as c_double);
      let mut ns: c_long = frac as c_long;
      /* C 41's `ns += ns < frac;`: the comparison promotes `ns` to double and its
         bool result converts to 0 or 1.  */
      ns += ((ns as c_double) < frac) as c_long;
      s += ns / TIMESPEC_HZ as c_long;
      ns %= TIMESPEC_HZ as c_long;

      if ns < 0
        {
          s -= 1;
          ns += TIMESPEC_HZ as c_long;
        }

      return make_timespec (s, ns);
    }
}
