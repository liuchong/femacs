/* Read the contents of a symbolic link.
   Copyright (C) 2003-2007, 2009-2026 Free Software Foundation, Inc.

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

/* emacs/lib/readlink.c: HAVE_READLINK is 1 in the frozen configuration, so C 26-43's
   `#if !HAVE_READLINK` arm (the DJGPP/mingw32 substitute, whose `readlink` only reports
   EINVAL) is dead and C 45-114's `#else` arm - `rpl_readlink` - is the live text; the dead
   arm's text is kept under the feature that names the non-frozen side (P-51).  Inside the live
   arm both READLINK_TRAILING_SLASH_BUG and READLINK_TRUNCATE_BUG are undefined on this target,
   so those two arms are dead as well, and so is the `# if defined __CYGWIN__` arm (this is a
   darwin-arm64 target).  C 20's <unistd.h> supplies the system `readlink` (unistd.in.rs),
   C 22's <errno.h> the errno mirror and C 24's <sys/stat.h> `struct stat` for the arms.  */
use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/unistd.in.rs");
include!("include/errno.rs");

/* C 26-43's `#if !HAVE_READLINK` arm, kept as text (P-51; dead: HAVE_READLINK is 1).  */

/* readlink() substitute for systems that don't have a readlink() function,
   such as DJGPP 2.03 and mingw32.  */
#[cfg (feature = "no_readlink")]
pub unsafe extern "C" fn readlink (file: *const c_char, _buf: *mut c_char, _bufsize: usize)
  -> ssize_t
{
  /* C declares `struct stat statbuf;` uninitialized; Rust needs a value, and C's own comment
     below says the contents are never used.  */
  let mut statbuf: stat = std::mem::zeroed ();

  /* In general we should use lstat() here, not stat().  But on platforms
     without symbolic links, lstat() - if it exists - would be equivalent to
     stat(), therefore we can use stat().  This saves us a configure check.  */
  if stat (file, &mut statbuf) >= 0
    { *errno () = EINVAL; }
  return -1;
}

/* C 47's `# undef readlink`: the system entry the wrapper calls.  */
mod system {
  use super::*;

  unsafe extern "C" {
    pub fn readlink (file: *const c_char, buf: *mut c_char, bufsize: usize) -> ssize_t;
  }
}

/* readlink() wrapper that uses correct types, for systems like cygwin
   1.5.x where readlink returns int, and which rejects trailing slash,
   for Solaris 9.  */

/* C 53-112's `ssize_t rpl_readlink (char const *file, char *buf, size_t bufsize)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_readlink (file: *const c_char, buf: *mut c_char, bufsize: usize)
  -> ssize_t
{
  /* C 56-70's `# if READLINK_TRAILING_SLASH_BUG` arm, kept as text (P-51; dead).  */
  #[cfg (feature = "readlink_trailing_slash_bug")]
  {
    let file_len: usize = strlen (file);
    if file_len != 0 && *file.add (file_len - 1) == b'/' as c_char
      {
        /* Even if FILE without the slash is a symlink to a directory,
           both lstat() and stat() must resolve the trailing slash to
           the directory rather than the symlink.  We can therefore
           safely use stat() to distinguish between EINVAL and
           ENOTDIR/ENOENT, avoiding extra overhead of rpl_lstat().  */
        let mut st: stat = std::mem::zeroed ();
        if stat (file, &mut st) == 0 || *errno () == EOVERFLOW
          { *errno () = EINVAL; }
        return -1;
      }
  }

  let mut r: ssize_t = system::readlink (file, buf, bufsize);

  /* C 74-99's `# if READLINK_TRUNCATE_BUG` arm, kept as text (P-51; dead).  */
  #[cfg (feature = "readlink_truncate_bug")]
  {
    if r < 0 && *errno () == ERANGE
      {
        /* Try again with a bigger buffer.  This is just for test cases;
           real code invariably discards short reads.  */
        let mut stackbuf: [c_char; 4032] = [0; 4032];
        r = system::readlink (file, stackbuf.as_mut_ptr (), stackbuf.len ());
        if r < 0
          {
            if *errno () == ERANGE
              {
                /* Clear the buffer, which is good enough for real code.
                   Thankfully, no test cases try short reads of enormous
                   symlinks and what would be the point anyway?  */
                r = bufsize as ssize_t;
                memset (buf as *mut c_void, 0, bufsize);
              }
          }
        else
          {
            if bufsize < r as usize
              { r = bufsize as ssize_t; }
            memcpy (buf as *mut c_void, stackbuf.as_ptr () as *const c_void, r as usize);
          }
      }
  }

  /* C 101-109's `# if defined __CYGWIN__` arm, kept as text (P-51; dead on this target).  */
  #[cfg (feature = "cygwin")]
  {
    /* On Cygwin 3.3.6, readlink("/dev/null") returns "\\Device\\Null", which
       is unusable.  Better fail with EINVAL.  */
    if r > 0 && strncmp (file, c"/dev/".as_ptr (), 5) == 0 && *buf == b'\\' as c_char
      {
        *errno () = EINVAL;
        return -1;
      }
  }

  return r;
}

/* <string.h>: the two entries the dead arms above name (declared here until string.in.h's
   mirror lands).  */
unsafe extern "C" {
  pub fn strlen (s: *const c_char) -> usize;
  pub fn strncmp (a: *const c_char, b: *const c_char, n: usize) -> c_int;
  pub fn memset (s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
  pub fn memcpy (d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
}
