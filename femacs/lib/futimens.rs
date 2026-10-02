/* futimens (fd, times) function: set file access and modification times.
   Copyright (C) 2009-2026 Free Software Foundation, Inc.

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

/* written by Eric Blake */

use std::ffi::{c_char, c_double, c_int, c_long};
include!("../src/include/config.rs"); include!("include/gnulib_config.rs");
include!("../lib/include/timespec.rs");

/* utimens.h declares fdutimens; its own mirror lands with lib/utimens.rs.  */
unsafe extern "C" { pub fn fdutimens (fd: c_int, file: *const c_char, times: *const timespec) -> c_int; }

/* Set the access and modification timestamps of FD to be
   TIMESPEC[0] and TIMESPEC[1], respectively.
   Fail with ENOSYS on systems without futimes (or equivalent).
   If TIMESPEC is null, set the timestamps to the current time.
   Return 0 on success, -1 (setting errno) on failure.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn futimens (fd: c_int, times: *const timespec) -> c_int
{
  /* fdutimens also works around bugs in native futimens, when running
     with glibc compiled against newer headers but on a Linux kernel
     older than 2.6.32.  */
  return fdutimens (fd, std::ptr::null (), times);
}
