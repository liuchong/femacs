// femacs: port of lib/readlinkat.c @ fd7afced403.
/* written by Eric Blake */

/* emacs/lib/readlinkat.c: HAVE_READLINKAT is 1 in the frozen configuration, so C 25-91's
   `#if HAVE_READLINKAT` arm (`rpl_readlinkat`) is the live text and C 93-123's `#else` arm -
   the `#include "at-func.c"` instantiation that falls back to readlink plus save_cwd/fchdir -
   is dead; the dead arm is kept under the feature that names the non-frozen side (P-51), with
   its five AT_FUNC_* macros kept as macro_rules (their bodies are the token lists C passes to
   the template).  Inside the live arm, READLINK_TRAILING_SLASH_BUG, READLINK_TRUNCATE_BUG and
   `__CYGWIN__` are all undefined, so those three arms are dead as well (same shape as
   lib/readlink.c).  C 22's <unistd.h>, C 24's <errno.h>, C 25's <stdlib.h>, C 26's <string.h>
   and C 27's <sys/stat.h> are the unistd mirror, the errno mirror, the `malloc`/`free`
   declarations, the `strlen`/`memset`/`memcpy`/`strncmp` declarations and the sys_stat mirror.  */
use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/fcntl.in.rs");
include!("include/unistd.in.rs");
include!("include/errno.rs");

/* C 24 and 26's <stdlib.h>/<string.h> entries the dead arms name.  */
unsafe extern "C" {
  pub fn malloc (n: usize) -> *mut c_void;
  pub fn free (p: *mut c_void);
  pub fn strlen (s: *const c_char) -> usize;
  pub fn strncmp (a: *const c_char, b: *const c_char, n: usize) -> c_int;
  pub fn memset (s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
  pub fn memcpy (d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
}

/* C 28's `# undef fstatat` / `# undef readlinkat`: the system entries the live wrapper calls.  */
mod system {
  use super::*;

  unsafe extern "C" {
    pub fn fstatat (fd: c_int, file: *const c_char, buf: *mut stat, flag: c_int) -> c_int;
    pub fn readlinkat (fd: c_int, file: *const c_char, buf: *mut c_char, bufsize: usize)
      -> ssize_t;
  }
}

/* C 34-91's `ssize_t rpl_readlinkat (int fd, char const *file, char *buf, size_t bufsize)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_readlinkat (fd: c_int, file: *const c_char, buf: *mut c_char,
                                         bufsize: usize) -> ssize_t
{
  /* C 36-49's `# if READLINK_TRAILING_SLASH_BUG` arm, kept as text (P-51; dead).  */
  #[cfg (feature = "readlink_trailing_slash_bug")]
  {
    let file_len: usize = strlen (file);
    if file_len != 0 && *file.add (file_len - 1) == b'/' as c_char
      {
        /* Even if FILE without the slash is a symlink to a directory,
           both lstat() and stat() must resolve the trailing slash to
           the directory rather than the symlink.  We can therefore
           safely use fstatat(..., 0) to distinguish between EINVAL and
           ENOTDIR/ENOENT, avoiding extra overhead of rpl_fstatat().  */
        let mut st: stat = std::mem::zeroed ();
        if system::fstatat (fd, file, &mut st, 0) == 0 || *errno () == EOVERFLOW
          { *errno () = EINVAL; }
        return -1;
      }
  }

  let mut r: ssize_t = system::readlinkat (fd, file, buf, bufsize);

  /* C 53-78's `# if READLINK_TRUNCATE_BUG` arm, kept as text (P-51; dead).  */
  #[cfg (feature = "readlink_truncate_bug")]
  {
    if r < 0 && *errno () == ERANGE
      {
        /* Try again with a bigger buffer.  This is just for test cases;
           real code invariably discards short reads.  */
        let mut stackbuf: [c_char; 4032] = [0; 4032];
        r = system::readlinkat (fd, file, stackbuf.as_mut_ptr (), stackbuf.len ());
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

  /* C 80-88's `# if defined __CYGWIN__` arm, kept as text (P-51; dead on this target).  */
  #[cfg (feature = "cygwin")]
  {
    /* On Cygwin 3.3.6, readlinkat(AT_FDCWD,"/dev/null") returns "\\Device\\Null",
       which is unusable.  Better fail with EINVAL.  */
    if r > 0 && strncmp (file, c"/dev/".as_ptr (), 5) == 0 && *buf == b'\\' as c_char
      {
        *errno () = EINVAL;
        return -1;
      }
  }

  return r;
}

/* C 93-123's `#else` arm, kept as text (P-51; dead: HAVE_READLINKAT is 1).  */

/* Gnulib provides a readlink stub for mingw; use it for distinction
   between EINVAL and ENOENT, rather than always failing with ENOSYS.  */

/* POSIX 2008 says that unlike readlink, readlinkat returns 0 for
   success instead of the buffer length.  But this would render
   readlinkat worthless since readlink does not guarantee a
   NUL-terminated buffer.  Assume this was a bug in POSIX.  */

/* Read the contents of symlink FILE into buffer BUF of size BUFSIZE, in the
   directory open on descriptor FD.  If possible, do it without changing
   the working directory.  Otherwise, resort to using save_cwd/fchdir,
   then readlink/restore_cwd.  If either the save_cwd or the restore_cwd
   fails, then give a diagnostic and exit nonzero.  */

#[cfg (feature = "no_readlinkat")]
macro_rules! AT_FUNC_NAME { () => { readlinkat } }
#[cfg (feature = "no_readlinkat")]
macro_rules! AT_FUNC_F1 { () => { readlink } }
#[cfg (feature = "no_readlinkat")]
macro_rules! AT_FUNC_POST_FILE_PARAM_DECLS { () => { , char *buf, size_t bufsize } }
#[cfg (feature = "no_readlinkat")]
macro_rules! AT_FUNC_POST_FILE_ARGS { () => { , buf, bufsize } }
#[cfg (feature = "no_readlinkat")]
macro_rules! AT_FUNC_RESULT { () => { ssize_t } }

/* C 116's `#include "at-func.c"` expansion: the /proc/self/fd simulation over
   save-cwd.h/openat.h/openat-priv.h, which this target has not got; its tokens are recorded in
   docs/ports/macro-body-notes.tsv (the gettimeofday.c Windows-arm treatment).  */
