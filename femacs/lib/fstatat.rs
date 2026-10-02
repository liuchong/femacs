/* Work around an fstatat bug on Solaris 9.

   Copyright (C) 2006, 2009-2026 Free Software Foundation, Inc.

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

/* Written by Paul Eggert and Jim Meyering.  */

/* emacs/lib/fstatat.c: HAVE_FSTATAT and HAVE_WORKING_FSTATAT_ZERO_FLAG are both 1 in the
   frozen configuration, so C 53-106's arm - `orig_fstatat`, `normal_fstatat` and
   `rpl_fstatat` - is the live text, and C 108-150's `#else` arm is dead; the dead arm's
   `stat_func` and its `AT_FUNC_*` macros are kept under the feature that names the non-frozen
   side (P-51), while the `#include "at-func.c"` expansion inside it needs save-cwd.h,
   openat.h and openat-priv.h, which this target has not got: its tokens are recorded in
   docs/ports/macro-body-notes.tsv (the gettimeofday.c Windows-arm treatment).
   C 23's `# define __need_system_sys_stat_h` before <config.h> is the lstat.c shape, C 38's
   <sys/stat.h> is the sys_stat mirror, C 42's stat-time.h the stat-time mirror, and C 44-47's
   <errno.h>, <fcntl.h>, <stdlib.h> and <string.h> the errno mirror, the fcntl mirror and the
   `malloc`/`free`/`strlen` declarations below (stdlib.in.h/string.in.h have no mirror yet).  */
use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/fcntl.in.rs");
include!("include/stat-time.rs");
include!("include/errno.rs");

/* C 23's `#define __need_system_sys_stat_h` (empty body; the port keeps the name).  */
pub const __need_system_sys_stat_h: () = ();

/* C 46-47's <stdlib.h> and <string.h> entries (the mirrors of those headers land with their
   consumers).  */
unsafe extern "C" {
  pub fn malloc (n: usize) -> *mut c_void;
  pub fn free (p: *mut c_void);
  pub fn strlen (s: *const c_char) -> usize;
}

/* C 49-51's `#ifndef AT_EMPTY_PATH / # define AT_EMPTY_PATH 0`: this target's <fcntl.h> does
   not define the Linux-only name, so the fallback is live.  */
pub const AT_EMPTY_PATH: c_int = 0;

/* C 31-37's `#if HAVE_FSTATAT && HAVE_WORKING_FSTATAT_ZERO_FLAG` arm (the live one).  */

/* C 55-57's `#ifndef LSTAT_FOLLOWS_SLASHED_SYMLINK / # define ... 0`, live on this target.  */
pub const LSTAT_FOLLOWS_SLASHED_SYMLINK: c_int = 0;

/* C 32-36's `static int orig_fstatat (int fd, char const *filename, struct stat *buf,
   int flags)`.  */
unsafe fn orig_fstatat (fd: c_int, filename: *const c_char, buf: *mut stat, flags: c_int)
  -> c_int
{
  return fstatat (fd, filename, buf, flags);
}

/* C 59-63's `static int normal_fstatat (int fd, char const *file, struct stat *st, int flag)`.  */
unsafe fn normal_fstatat (fd: c_int, file: *const c_char, st: *mut stat, flag: c_int) -> c_int
{
  return stat_time_normalize (orig_fstatat (fd, file, st, flag), st);
}

/* fstatat should always follow symbolic links that end in /, but on
   Solaris 9 it doesn't if AT_SYMLINK_NOFOLLOW is specified.
   Likewise, trailing slash on a non-directory should be an error.
   These are the same problems that lstat.c and stat.c address, so
   solve it in a similar way.

   AIX 7.1 fstatat (AT_FDCWD, ..., 0) always fails, which is a bug.
   Work around this bug if FSTATAT_AT_FDCWD_0_BROKEN is nonzero.  */

/* C 74-106's `int rpl_fstatat (int fd, char const *file, struct stat *st, int flags)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_fstatat (fd: c_int, file: *const c_char, st: *mut stat,
                                      flags: c_int) -> c_int
{
  /* Implement Linux kernel 6.11+ behavior on platforms that have
     AT_EMPTY_PATH but do not support it on null pointers.  */
  let mut file = file;
  if flags & AT_EMPTY_PATH != 0 && file.is_null ()
    { file = c"".as_ptr (); }

  let mut result: c_int = normal_fstatat (fd, file, st, flags);
  if LSTAT_FOLLOWS_SLASHED_SYMLINK != 0 || result != 0
    { return result; }

  let len: usize = strlen (file);
  if flags & AT_SYMLINK_NOFOLLOW != 0
    {
      /* Fix lstat behavior.  */
      if *file.add (len - 1) != b'/' as c_char || S_ISDIR! ((*st).st_mode)
        { return 0; }
      if !S_ISLNK! ((*st).st_mode)
        {
          *errno () = ENOTDIR;
          return -1;
        }
      result = normal_fstatat (fd, file, st, flags & !AT_SYMLINK_NOFOLLOW);
    }
  /* Fix stat behavior.  */
  if result == 0 && !S_ISDIR! ((*st).st_mode) && *file.add (len - 1) == b'/' as c_char
    {
      *errno () = ENOTDIR;
      return -1;
    }
  return result;
}

/* C 108-150's `#else` arm, kept as text (P-51; dead: both macros are 1).  */

/* On mingw, the gnulib <sys/stat.h> defines 'stat' as a function-like
   macro; but using it in AT_FUNC_F2 causes compilation failure
   because the preprocessor sees a use of a macro that requires two
   arguments but is only given one.  Hence, we need an inline
   forwarder to get past the preprocessor.  */
#[cfg (feature = "no_working_fstatat_zero_flag")]
unsafe fn stat_func (name: *const c_char, st: *mut stat) -> c_int
{
  return stat (name, st);
}

/* Likewise, if there is no native 'lstat', then the gnulib
   <sys/stat.h> defined it as stat, which also needs adjustment.  */
#[cfg (feature = "no_working_fstatat_zero_flag")]
macro_rules! lstat { () => { stat_func } }

/* C 136-141's `# define AT_FUNC_*` block: the at-func.c template's parameters.  */
#[cfg (feature = "no_working_fstatat_zero_flag")]
macro_rules! AT_FUNC_NAME { () => { fstatat } }
#[cfg (feature = "no_working_fstatat_zero_flag")]
macro_rules! AT_FUNC_F1 { () => { lstat } }
#[cfg (feature = "no_working_fstatat_zero_flag")]
macro_rules! AT_FUNC_F2 { () => { stat_func } }
#[cfg (feature = "no_working_fstatat_zero_flag")]
macro_rules! AT_FUNC_USE_F1_COND { () => { AT_SYMLINK_NOFOLLOW } }
#[cfg (feature = "no_working_fstatat_zero_flag")]
macro_rules! AT_FUNC_POST_FILE_PARAM_DECLS { () => { , struct stat *st, int flag } }
#[cfg (feature = "no_working_fstatat_zero_flag")]
macro_rules! AT_FUNC_POST_FILE_ARGS { () => { , st } }

/* C 143's `#include "at-func.c"` expansion: the /proc/self/fd simulation built on
   save_cwd/fchdir/restore_cwd, whose headers this target has not got; its tokens are recorded
   in docs/ports/macro-body-notes.tsv (the gettimeofday.c Windows-arm treatment) instead of
   being written out.  */
