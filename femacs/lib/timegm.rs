/* Convert UTC calendar time to simple time.  Like mktime but assumes UTC.

   Copyright (C) 1994-2026 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/timegm.c @ fd7afced403.  `_LIBC` is undefined,
// so C 20-22's `<libc-config.h>` include is live and C 36-57's `#if defined _LIBC &&
// __TIMESIZE != 64` arm (the 32-bit `timegm` wrapper) is dead.  mktime-internal.h renames
// the definition twice on this target: `__timegm64` -> `timegm` (mktime-internal.h 53) and
// `timegm` -> `rpl_timegm` (the generated <time.h>'s REPLACE_TIMEGM arm), which is the
// symbol the reference object carries: `nm -g build/emacs-ref/lib/timegm.o` prints
// `T _rpl_timegm`.  The port keeps C's source name and emits the real symbol with
// `#[unsafe(export_name = ...)]` (PORTING.md 10.13①, the mempcpy precedent).
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");            // <errno.h>
include!("include/mktime-internal.rs");  // "mktime-internal.h"

/* __time64_t __timegm64 (struct tm *tmp)  */
#[unsafe(export_name = "rpl_timegm")]
pub unsafe extern "C" fn __timegm64 (tmp: *mut tm) -> __time64_t
{
  /* C 32: `static mktime_offset_t gmtime_offset;` - C zero-initializes it.  */
  static mut gmtime_offset: mktime_offset_t = 0;
  return __mktime_internal! (tmp, false, &raw mut gmtime_offset);
}
