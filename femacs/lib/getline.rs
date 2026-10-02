/* getline.c -- Replacement for GNU C library function getline
   Copyright (C) 2003, 2006, 2009-2026 Free Software Foundation, Inc.

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
use std::ffi::{c_char, c_int};
include!("include/gnulib_config.rs");
include!("../lib/include/stdio.rs");
unsafe extern "C" {
  pub fn getdelim (lineptr: *mut *mut c_char, n: *mut usize, delim: c_int, stream: *mut FILE) -> isize;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getline (lineptr: *mut *mut c_char, n: *mut usize, stream: *mut FILE) -> isize
{
  return getdelim (lineptr, n, b'\n' as c_int, stream);
}
