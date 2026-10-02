/* Stub for symlink().
   Copyright (C) 2009-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* emacs/lib/symlink.c: HAVE_SYMLINK is 1 in the frozen configuration, so C 26-44's
   `#if HAVE_SYMLINK` arm (`rpl_symlink`, which rejects a trailing slash) is the live text and
   C 46-57's `#else` arm - the ENOSYS stub for systems without symlinks - is dead; the dead
   arm is kept under the feature that names the non-frozen side (P-51).  C 20's <unistd.h>
   supplies the system `symlink` (unistd.in.rs), C 22-24's <errno.h>, <string.h> and
   <sys/stat.h> the errno mirror, `strlen` (declared here until string.in.h's mirror lands)
   and `struct stat`.  */
use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/unistd.in.rs");
include!("include/errno.rs");

/* <string.h>: `strlen`, declared here until string.in.h's mirror lands.  */
unsafe extern "C" { pub fn strlen (s: *const c_char) -> usize; }

/* C 29's `# undef symlink`: the system entry `rpl_symlink` calls.  */
mod system {
  use super::*;

  unsafe extern "C" {
    pub fn symlink (contents: *const c_char, name: *const c_char) -> c_int;
  }
}

/* Create a symlink, but reject trailing slash.  */

/* C 32-44's `int rpl_symlink (char const *contents, char const *name)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_symlink (contents: *const c_char, name: *const c_char) -> c_int
{
  let len: usize = strlen (name);
  if len != 0 && *name.add (len - 1) == b'/' as c_char
    {
      let mut st: stat = std::mem::zeroed ();
      if lstat (name, &mut st) == 0 || *errno () == EOVERFLOW
        { *errno () = EEXIST; }
      return -1;
    }
  return system::symlink (contents, name);
}

/* C 46-57's `#else` arm, kept as text (P-51; dead: HAVE_SYMLINK is 1).  */

/* The system does not support symlinks.  */
#[cfg (feature = "no_symlink")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn symlink (_contents: *const c_char, _name: *const c_char) -> c_int
{
  *errno () = ENOSYS;
  return -1;
}
