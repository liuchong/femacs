/* Get the number of file descriptors available to the process.

   Copyright (C) 2004-2026 Free Software Foundation, Inc.

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

/* emacs/lib/getdtablesize.c (@ 121 lines) is on the verified list, so the `_WIN32` arm (C 20-85:
   the `_setmaxstdio` probe, its cache and its own `getdtablesize`) and the `__CYGWIN__` arm
   (C 102-106) are dead by measurement and only C 88-120 is required: the two `#ifndef
   RLIM_SAVED_*` fallbacks (live: this target's <sys/resource.h> does not define them) and the
   one function.  C 89-90's <limits.h> and <sys/resource.h> are the limits and sys_resource
   mirrors; the `RLIM_SAVED_*` values are C's own fallback, so the port keeps them as constants
   and the tolerance row records the line-count relation.  */
use std::ffi::{c_int, c_long, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/sys_resource.rs");

/* C 92-97's `#ifndef RLIM_SAVED_CUR / # define RLIM_SAVED_CUR RLIM_INFINITY` and its MAX twin:
   live on this target, so the port keeps C's values.  */
pub const RLIM_SAVED_CUR: rlim_t = RLIM_INFINITY;
pub const RLIM_SAVED_MAX: rlim_t = RLIM_INFINITY;

/* C 99-106's `# ifdef __CYGWIN__ / # define rlim_cur rlim_max` arm, kept as text (P-51; dead
   on darwin-arm64: the port's `rlim_cur` for this target is the struct's own field).  */
#[cfg (feature = "cygwin")]
macro_rules! rlim_cur { () => { rlim_max } }

/* C 107-120's `int getdtablesize (void)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getdtablesize () -> c_int
{
  let mut lim: rlimit = std::mem::zeroed ();

  if getrlimit (RLIMIT_NOFILE, &mut lim) == 0
    && 0 <= lim.rlim_cur && lim.rlim_cur <= INT_MAX as rlim_t
    && lim.rlim_cur != RLIM_INFINITY
    && lim.rlim_cur != RLIM_SAVED_CUR
    && lim.rlim_cur != RLIM_SAVED_MAX
    { return lim.rlim_cur as c_int; }

  return INT_MAX;
}
