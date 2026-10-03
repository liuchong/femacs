/* Inline functions for <timespec.h>.

   Copyright (C) 2012-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong,
                c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/timespec.rs");

/* lib/timespec.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_TIMESPEC_INLINE: () = _GL_EXTERN_INLINE;

/* lib/timespec.c is a gnulib `_GL_*_INLINE` wrapper: its C text is the license, the
   `#define <qualifier> _GL_EXTERN_INLINE` line and the includes, i.e. the C99 inline model's
   "this translation unit emits the external definition" file.  The body lives in the header
   (mirrored as `lib/include/timespec.rs`) and C's object exports the symbol below; Rust's
   mirror keeps the body as an inline function, which emits nothing, so the unit re-exports it
   under C's own name (PORTING.md P-81).  */
mod timespec_unit {
  use super::*;

  #[unsafe(export_name = "make_timespec")]
  pub unsafe extern "C" fn make_timespec (s: time_t, ns: c_long) -> timespec
  {
    return make_timespec (s, ns);
  }

  #[unsafe(export_name = "timespec_cmp")]
  pub unsafe extern "C" fn timespec_cmp (a: timespec, b: timespec) -> c_int
  {
    return timespec_cmp (a, b);
  }

  #[unsafe(export_name = "timespec_sign")]
  pub unsafe extern "C" fn timespec_sign (a: timespec) -> c_int
  {
    return timespec_sign (a);
  }

  #[unsafe(export_name = "timespectod")]
  pub unsafe extern "C" fn timespectod (a: timespec) -> c_double
  {
    return timespectod (a);
  }
}
