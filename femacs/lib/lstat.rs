// femacs: port of lib/lstat.c @ fd7afced403.
/* written by Jim Meyering */

/* emacs/lib/lstat.c: C 23's `#define __need_system_sys_stat_h` before <config.h> makes gnulib's
   <sys/stat.h> include only the system header, so `orig_lstat` below calls the real `lstat`
   rather than a replacement macro; the mirror declares the system entry itself, and the macro
   name is kept as a constant for the ledger.  C 26-30's `#if !HAVE_LSTAT` arm (the
   `typedef int dummy;` that keeps the file from being empty) is dead in the frozen
   configuration - HAVE_LSTAT is 1 - so its text is kept under the feature that names the
   non-frozen side (P-51).  C 34-50's includes are the mirrors below; <string.h> has no mirror
   yet, so `strlen` is declared here as C's <string.h> declares it.  */
use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/stat-time.rs");
include!("include/errno.rs");

/* C 23's `#define __need_system_sys_stat_h` (empty body; the port keeps the name).  */
pub const __need_system_sys_stat_h: () = ();

/* C 49's <string.h>: `strlen`, declared here until string.in.h's mirror lands.  */
unsafe extern "C" { pub fn strlen (s: *const c_char) -> usize; }

/* C 26-30's `#if !HAVE_LSTAT` arm, kept as text (P-51; dead: HAVE_LSTAT is 1).  */
#[cfg (feature = "no_lstat")]
pub type dummy = c_int;

/* Get the original definition of lstat.  It might be defined as a macro.  */

/* C 38-42's `static int orig_lstat (const char *filename, struct stat *buf)`.  */
unsafe fn orig_lstat (filename: *const c_char, buf: *mut stat) -> c_int
{
  return lstat (filename, buf);
}

/* lstat works differently on Linux and Solaris systems.  POSIX (see
   "pathname resolution" in the glossary) requires that programs like
   'ls' take into consideration the fact that FILE has a trailing slash
   when FILE is a symbolic link.  On Linux and Solaris 10 systems, the
   lstat function already has the desired semantics (in treating
   'lstat ("symlink/", sbuf)' just like 'lstat ("symlink/.", sbuf)',
   but on Solaris 9 and earlier it does not.

   If FILE has a trailing slash and specifies a symbolic link,
   then use stat() to get more info on the referent of FILE.
   If the referent is a non-directory, then set errno to ENOTDIR
   and return -1.  Otherwise, return stat's result.  */

/* C 65-95's `int rpl_lstat (const char *file, struct stat *sbuf)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_lstat (file: *const c_char, sbuf: *mut stat) -> c_int
{
  let mut result: c_int = orig_lstat (file, sbuf);

  /* This replacement file can blindly check against '/' rather than
     using the ISSLASH macro, because all platforms with '\\' either
     lack symlinks (mingw) or have working lstat (cygwin) and thus do
     not compile this file.  0 len should have already been filtered
     out above, with a failure return of ENOENT.  */
  if result == 0
    {
      if S_ISDIR! ((*sbuf).st_mode) || *file.add (strlen (file) - 1) != b'/' as c_char
        { result = stat_time_normalize (result, sbuf); }
      else
        {
          /* At this point, a trailing slash is permitted only on
             symlink-to-dir; but it should have found information on the
             directory, not the symlink.  Call 'stat' to get info about the
             link's referent.  Our replacement stat guarantees valid results,
             even if the symlink is not pointing to a directory.  */
          if !S_ISLNK! ((*sbuf).st_mode)
            {
              *errno () = ENOTDIR;
              return -1;
            }
          result = stat (file, sbuf);
        }
    }
  return result;
}
