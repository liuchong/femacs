/* lchmod -- change the mode of a file, without following symlinks

   Copyright (C) 2023-2026 Free Software Foundation, Inc.

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

/* written by Paul Eggert */

/* emacs/lib/lchmod.c (@ 97 lines): O_PATH is not defined on this target (measured), so C 30-68's
   `#ifdef O_PATH` arm - the open("/proc/self/fd/…") dance with its `INT_BUFSIZE_BOUND` buffer -
   is dead text kept under the feature that names the non-frozen side (P-51); its
   INT_BUFSIZE_BOUND token has no mirror here and is recorded in
   docs/ports/macro-body-notes.tsv.  The live text is C 70-96.  C 20's <sys/stat.h> is the
   sys_stat mirror, C 22-27's <errno.h>, <fcntl.h>, <stdio.h>, <string.h> and <unistd.h> are the
   errno, fcntl, stdio, string (`strlen`) and unistd mirrors, and C 25-26's "issymlink.h"/
   "issymlinkat.h" are the two new mirrors whose `issymlink`/`issymlinkat` the port calls.  */
use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/fcntl.in.rs");
include!("include/stdio.rs");
include!("include/string.rs");     // <string.h> (strlen)
include!("include/unistd.in.rs");
include!("include/errno.rs");
include!("include/issymlink.rs");
include!("include/issymlinkat.rs");

/* C 26's <stdio.h> entry the dead O_PATH arm uses on its /proc path: `sprintf` is declared
   here rather than in the stdio mirror, because src/doprnt.c declares it in its own module and
   a mirror-level declaration would be a second definition in that unit (E0428).  */
unsafe extern "C" {
  pub fn sprintf (s: *mut c_char, format: *const c_char, ...) -> c_int;
}

/* Work like chmod, except when FILE is a symbolic link.
   In that case, on systems where permissions on symbolic links are unsupported
   (such as Linux), set errno to EOPNOTSUPP and return -1.  */

/* C 34-96's `int lchmod (char const *file, mode_t mode)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lchmod (file: *const c_char, mode: mode_t) -> c_int
{
  /* C 36-68's `#ifdef O_PATH` arm, kept as text (P-51; dead: this target's <fcntl.h> has no
     O_PATH).  */
  #[cfg (feature = "o_path")]
  {
    /* Open a file descriptor with O_NOFOLLOW, to make sure we don't
       follow symbolic links, if /proc is mounted.  O_PATH is used to
       avoid a failure if the file is not readable.
       Cf. <https://sourceware.org/PR14578>  */
    let fd: c_int = open (file, O_PATH | O_NOFOLLOW | O_CLOEXEC);
    if fd < 0
      { return fd; }

    let err: c_int;
    {
      let ret: c_int = issymlinkat (fd, c"".as_ptr ());
      if ret > 0
        { err = EOPNOTSUPP; }
      else if ret == 0
        {
          let fmt: [c_char; 15] = *b"/proc/self/fd/%d\0";
          /* C declares `char buf[sizeof fmt - sizeof "%d" + INT_BUFSIZE_BOUND (int)]`; the
             INT_BUFSIZE_BOUND token has no mirror on this target and is in the notes.  */
          let mut buf: [c_char; 15 - 2 + 12] = [0; 15 - 2 + 12];
          sprintf (buf.as_mut_ptr (), fmt.as_ptr (), fd);
          err = if chmod (buf.as_ptr (), mode) == 0 { 0 } else if *errno () == ENOENT { -1 } else { *errno () };
        }
      else
        { err = if *errno () == ENOENT { -1 } else { *errno () }; }
    }

    close (fd);

    *errno () = err;
    if 0 <= err
      { return if err == 0 { 0 } else { -1 }; }
  }

  let len: usize = strlen (file);
  if len != 0 && *file.add (len - 1) == b'/' as c_char
    {
      let mut st: stat = std::mem::zeroed ();
      if lstat (file, &mut st) < 0
        { return -1; }
      if !S_ISDIR! (st.st_mode)
        {
          *errno () = ENOTDIR;
          return -1;
        }
    }

  /* O_PATH + /proc is not supported.  */

  if issymlink (file) > 0
    {
      *errno () = EOPNOTSUPP;
      return -1;
    }

  /* Fall back on chmod, despite a possible race.  */
  return chmod (file, mode);
}
