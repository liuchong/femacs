/* provide consistent interface to getgroups for systems that don't allow N==0

   Copyright (C) 1996, 1999, 2003, 2006-2026 Free Software Foundation, Inc.

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

/* written by Jim Meyering */

/* emacs/lib/getgroups.c: the frozen configuration leaves HAVE_GETGROUPS undefined, so C 28's
   `#if !HAVE_GETGROUPS` arm - the stub - is the live text, and C 41-119's `#else` arm (the
   replacement that copes with systems whose getgroups rejects N==0) is dead: its text is kept
   under `#[cfg (feature = "have_getgroups")]`, the P-51 shape.  C 22's `<unistd.h>` supplies the
   groups entry, C 24's `<errno.h>` the mirror below, C 25's `<stdlib.h>` `malloc`/`free` and
   C 26's `<stdint.h>` `SIZE_MAX` (the stdint mirror).  */
use std::ffi::{c_int, c_long, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/stdint.in.rs");
include!("include/errno.rs");

unsafe extern "C" {
  pub fn malloc (n: usize) -> *mut c_void;
  pub fn free (p: *mut c_void);
}

/* C 41's `# undef getgroups`: the system entry the replacement arm calls (C 80 and friends).  */
mod system {
  use super::*;

  unsafe extern "C" {
    #[cfg (not (feature = "have_getgroups"))]
    pub fn getgroups (n: c_int, groups: *mut GETGROUPS_T) -> c_int;
    /* C 56-58's Apple arm declares it under the kernel's name:
       `int posix_getgroups (int, gid_t []) __asm ("_getgroups");`.  */
    #[cfg (feature = "have_getgroups")]
    #[link_name = "_getgroups"]
    pub fn getgroups (n: c_int, groups: *mut gid_t) -> c_int;
  }
}

/* C 58's `#  define getgroups posix_getgroups`: kept as a macro whose body holds C's
   expansion, for the ledger.  */
#[cfg (feature = "have_getgroups")]
macro_rules! getgroups { () => { posix_getgroups } }

/* Provide a stub that fails with ENOSYS, since there is no group
   information available on mingw.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getgroups (_n: c_int, _groups: *mut GETGROUPS_T) -> c_int
{
  *errno () = ENOSYS;
  return -1;
}

/* C 41-119's `#else` arm, kept as text (it never compiles on this target, but its tokens stay
   in the port for the ledger and for a future target where HAVE_GETGROUPS is undefined).  */

/* C 42-44's `# ifndef GETGROUPS_ZERO_BUG / #  define GETGROUPS_ZERO_BUG 0`.  */
#[cfg (feature = "have_getgroups")]
pub const GETGROUPS_ZERO_BUG: c_int = 0;

/* C 56-58's Apple arm: `int posix_getgroups (int, gid_t []) __asm ("_getgroups");` plus the
   `#define getgroups posix_getgroups` that follows it.  */
#[cfg (feature = "have_getgroups")]
unsafe extern "C" {
  #[link_name = "_getgroups"]
  pub fn posix_getgroups (n: c_int, groups: *mut gid_t) -> c_int;
}

#[cfg (feature = "have_getgroups")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_getgroups (mut n: c_int, group: *mut gid_t) -> c_int
{
  if n < 0
    {
      *errno () = EINVAL;
      return -1;
    }

  if n != 0 || GETGROUPS_ZERO_BUG == 0
    {
      if size_of::<*mut gid_t> () == size_of::<GETGROUPS_T> ()
        { return system::getgroups (n, group as *mut GETGROUPS_T); }

      if SIZE_MAX / size_of::<GETGROUPS_T> () <= n as usize
        {
          *errno () = ENOMEM;
          return -1;
        }
      let gbuf: *mut GETGROUPS_T
        = malloc (n as usize * size_of::<GETGROUPS_T> ()) as *mut GETGROUPS_T;
      if gbuf.is_null ()
        { return -1; }
      let mut result: c_int = system::getgroups (n, gbuf);
      if 0 <= result
        {
          n = result;
          while n != 0
            {
              n -= 1;
              *group.offset (n as isize) = *gbuf.offset (n as isize) as gid_t;
            }
        }
      free (gbuf as *mut c_void);
      return result;
    }

  n = 20;
  loop
    {
      /* No need to worry about address arithmetic overflow here,
         since the ancient systems that we're running on have low
         limits on the number of secondary groups.  */
      let gbuf: *mut GETGROUPS_T
	= malloc (n as usize * size_of::<GETGROUPS_T> ()) as *mut GETGROUPS_T;
      if gbuf.is_null ()
        { return -1; }
      let n_groups: c_int = system::getgroups (n, gbuf);
      if if n_groups == -1 { *errno () != EINVAL } else { n_groups < n }
        {
          free (gbuf as *mut c_void);
          return n_groups;
        }
      free (gbuf as *mut c_void);
      n *= 2;
    }
}
