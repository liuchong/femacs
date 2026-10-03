/* Reentrant time functions like localtime_r.

   Copyright (C) 2003, 2006-2007, 2010-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
// femacs: port of lib/time_r.c @ fd7afced403.  The file has no
// preprocessing conditions, and both definitions are exported symbols.  The unit defines
// `gmtime_r`/`localtime_r`, which <time.h>'s mirror declares: those declarations live in the
// mirror's `pub mod time_decls` + glob import, so the definitions here shadow the glob
// (PORTING.md 0.1-10).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/time.in.rs");    // <time.h> (`struct tm`, `time_t`, `gmtime`, `localtime`)


/* The replacement functions in this file are only used on native Windows.
   They are thread-safe, because the gmtime() and localtime() functions
   on native Windows — both in the ucrt and in the older MSVCRT — return a
   pointer to a 'struct tm' in thread-local memory.  */

/* static struct tm *copy_tm_result (struct tm *dest, struct tm const *src)  */
unsafe fn copy_tm_result (dest: *mut tm, src: *const tm) -> *mut tm
{
  if src.is_null ()
    { return 0 as *mut tm; }
  *dest = *src;
  return dest;
}

/* struct tm *gmtime_r (time_t const *restrict t, struct tm *restrict tp)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gmtime_r (t: *const time_t, tp: *mut tm) -> *mut tm
{
  return copy_tm_result (tp, gmtime (t));
}

/* struct tm *localtime_r (time_t const *restrict t, struct tm *restrict tp)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn localtime_r (t: *const time_t, tp: *mut tm) -> *mut tm
{
  return copy_tm_result (tp, localtime (t));
}
