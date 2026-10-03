/* Copy memory area and return pointer after last written byte.
   Copyright (C) 2003, 2012-2026 Free Software Foundation, Inc.

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

use std::ffi::{c_char, c_int, c_void};
include!("../src/include/config.rs"); include!("include/gnulib_config.rs");

unsafe extern "C" { pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void; }

/* Copy N bytes of SRC to DEST, return pointer to bytes after the
   last written byte.  */
/* The frozen configuration has HAVE_MEMPCPY undefined, so C's `#if !HAVE_MEMPCPY` body is
   live; gnulib's <string.h> renames the identifier to `rpl_mempcpy` in this translation
   unit (which is the symbol doprnt.rs declares), so the port keeps C's function name for
   the ledger and emits C's real symbol through export_name (PORTING.md 10.13).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mempcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void
{
  return (memcpy (dest, src, n) as *mut c_char).offset (n as isize) as *mut c_void;
}
