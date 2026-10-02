/* filemode - describe a file's mode as an ls-style string

   Copyright (C) 1990-2026 Free Software Foundation, Inc.

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

/* emacs/lib/filemode.c: HAVE_DECL_STRMODE is 1 in the frozen configuration (macOS's
   <string.h> declares it), so C 23-107's `#if ! HAVE_DECL_STRMODE` arm - `ftypelet` and
   gnulib's own `strmode` - is dead text kept under the feature that names the frozen side
   (P-51, the getgroups.c shape) and C 154-166's `filemodestring` (which C 107's `#endif`
   leaves outside the arm) is the live one; its `strmode` call is the system entry.
   C 21's "filemode.h" is the filemode mirror, C 23's <sys/stat.h> the sys_stat mirror and
   the S_TYPEIS* tests it leaves as `0` on this target are what the port's macro_rules say.  */
use std::ffi::{c_char, c_int, c_long, c_short, c_uint};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");

/* C 42-106's dead arm, kept as text (P-51; dead: HAVE_DECL_STRMODE is 1).  */

/* Return a character indicating the type of file described by
   file mode BITS:
   '-' regular file
   'b' block special file
   'c' character special file
   'C' high performance ("contiguous data") file
   'd' directory
   'D' door
   'l' symbolic link
   'm' multiplexed file (7th edition Unix; obsolete)
   'n' network special file (HP-UX)
   'p' fifo (named pipe)
   'P' port
   's' socket
   'w' whiteout (4.4BSD)
   '?' some other file type  */
#[cfg (feature = "have_decl_strmode")]
unsafe fn ftypelet (bits: mode_t) -> c_char
{
  /* These are the most common, so test for them first.  */
  if S_ISREG! (bits) != 0
    { return b'-' as c_char; }
  if S_ISDIR! (bits) != 0
    { return b'd' as c_char; }

  /* Other letters standardized by POSIX 1003.1-2004.  */
  if S_ISBLK! (bits) != 0
    { return b'b' as c_char; }
  if S_ISCHR! (bits) != 0
    { return b'c' as c_char; }
  if S_ISLNK! (bits) != 0
    { return b'l' as c_char; }
  if S_ISFIFO! (bits) != 0
    { return b'p' as c_char; }

  /* Other file types (though not letters) standardized by POSIX.  */
  if S_ISSOCK! (bits) != 0
    { return b's' as c_char; }

  /* Nonstandard file types.  */
  if S_ISCTG! (bits) != 0
    { return b'C' as c_char; }
  if S_ISDOOR! (bits) != 0
    { return b'D' as c_char; }
  if S_ISMPB! (bits) != 0 || S_ISMPC! (bits) != 0 || S_ISMPX! (bits) != 0
    { return b'm' as c_char; }
  if S_ISNWK! (bits) != 0
    { return b'n' as c_char; }
  if S_ISPORT! (bits) != 0
    { return b'P' as c_char; }
  if S_ISWHT! (bits) != 0
    { return b'w' as c_char; }

  return b'?' as c_char;
}

/* Like filemodestring, but rely only on MODE.  */
#[cfg (feature = "have_decl_strmode")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strmode (mode: mode_t, str_: *mut c_char)
{
  *str_.offset (0) = ftypelet (mode);
  *str_.offset (1) = if mode & S_IRUSR != 0 { b'r' as c_char } else { b'-' as c_char };
  *str_.offset (2) = if mode & S_IWUSR != 0 { b'w' as c_char } else { b'-' as c_char };
  *str_.offset (3) = (if mode & S_ISUID != 0
                        { if mode & S_IXUSR != 0 { b's' as c_char } else { b'S' as c_char } }
                      else
                        { if mode & S_IXUSR != 0 { b'x' as c_char } else { b'-' as c_char } });
  *str_.offset (4) = if mode & S_IRGRP != 0 { b'r' as c_char } else { b'-' as c_char };
  *str_.offset (5) = if mode & S_IWGRP != 0 { b'w' as c_char } else { b'-' as c_char };
  *str_.offset (6) = (if mode & S_ISGID != 0
                        { if mode & S_IXGRP != 0 { b's' as c_char } else { b'S' as c_char } }
                      else
                        { if mode & S_IXGRP != 0 { b'x' as c_char } else { b'-' as c_char } });
  *str_.offset (7) = if mode & S_IROTH != 0 { b'r' as c_char } else { b'-' as c_char };
  *str_.offset (8) = if mode & S_IWOTH != 0 { b'w' as c_char } else { b'-' as c_char };
  *str_.offset (9) = (if mode & S_ISVTX != 0
                        { if mode & S_IXOTH != 0 { b't' as c_char } else { b'T' as c_char } }
                      else
                        { if mode & S_IXOTH != 0 { b'x' as c_char } else { b'-' as c_char } });
  *str_.offset (10) = b' ' as c_char;
  *str_.offset (11) = b'\0' as c_char;
}

/* C 107's `# define strmode`-less system entry: macOS's <string.h> declares `strmode`, which
   is what the live `filemodestring` calls (the header mirror records C's `#if
   !HAVE_DECL_STRMODE` arm as a cfg'd declaration of its own).  */
mod system {
  use super::*;

  unsafe extern "C" {
    pub fn strmode (mode: mode_t, str_: *mut c_char);
  }
}

/* filemodestring - fill in string STR with an ls-style ASCII
   representation of the st_mode field of file stats block STATP.
   12 characters are stored in STR.
   The characters stored in STR are:

   0    File type, as in ftypelet above, except that other letters are used
        for files whose type cannot be determined solely from st_mode:

            'F' semaphore
            'Q' message queue
            'S' shared memory object
            'T' typed memory object

   1    'r' if the owner may read, '-' otherwise.

   2    'w' if the owner may write, '-' otherwise.

   3    'x' if the owner may execute, 's' if the file is
        set-user-id, '-' otherwise.
        'S' if the file is set-user-id, but the execute
        bit isn't set.

   4    'r' if group members may read, '-' otherwise.

   5    'w' if group members may write, '-' otherwise.

   6    'x' if group members may execute, 's' if the file is
        set-group-id, '-' otherwise.
        'S' if it is set-group-id but not executable.

   7    'r' if any user may read, '-' otherwise.

   8    'w' if any user may write, '-' otherwise.

   9    'x' if any user may execute, 't' if the file is "sticky"
        (will be retained in swap space after execution), '-'
        otherwise.
        'T' if the file is sticky but not executable.

   10   ' ' for compatibility with 4.4BSD strmode,
        since this interface does not support ACLs.

   11   '\0'.  */

/* C 153-166's `void filemodestring (struct stat const *statp, char *str)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filemodestring (statp: *const stat, str_: *mut c_char)
{
  system::strmode ((*statp).st_mode, str_);

  if S_TYPEISSEM! (statp) != 0
    { *str_.offset (0) = b'F' as c_char; }
  else if S_TYPEISMQ! (statp) != 0
    { *str_.offset (0) = b'Q' as c_char; }
  else if S_TYPEISSHM! (statp) != 0
    { *str_.offset (0) = b'S' as c_char; }
  else if S_TYPEISTMO! (statp) != 0
    { *str_.offset (0) = b'T' as c_char; }
}

/* The dead arm above declares `c_int`/`c_uint` users only through the mirrors; keep the
   imports honest.  */
#[allow (unused)]
fn _unused (_: c_int, _: c_uint) {}
