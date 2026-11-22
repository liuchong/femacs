// femacs: port of lib/save-cwd.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
/* Gnulib needs to save and restore the current working directory to
   fully emulate functions like fstatat.  But Emacs doesn't care what
   the current working directory is; it always uses absolute file
   names.  This module replaces the Gnulib module by omitting the code
   that Emacs does not need.  */

/* emacs/lib/save-cwd.c: this is the reference's trimmed variant of the gnulib module, so the file has
   no `#if` at all except C 46-48's `#if !defined HAVE_FCHDIR && !defined fchdir /
   # define fchdir(fd) (-1)` - dead here, since HAVE_FCHDIR is 1 (the fchdir the restore path
   calls is the real one) - which the port keeps as a cfg'd macro (P-51).  C 28's
   "save-cwd.h" is the new save-cwd mirror, C 30's <fcntl.h> the fcntl mirror (`open`,
   `O_SEARCH`, `O_CLOEXEC`) and C 31's <unistd.h> the unistd mirror (`fchdir`, `close`).  */
use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/save-cwd.rs imported via common
// [U1] include/fcntl.in.rs imported via common
// [U1] include/unistd.in.rs imported via common

/* C 46-48's `#if !defined HAVE_FCHDIR && !defined fchdir` arm, kept as text (P-51; dead:
   HAVE_FCHDIR is 1).  */
#[cfg (feature = "no_fchdir")]
macro_rules! fchdir { ($fd:expr) => { -1 }; }

/* Record the location of the current working directory in CWD so that
   the program may change to other directories and later use restore_cwd
   to return to the recorded location.  This function may allocate
   space using malloc (via getcwd) or leave a file descriptor open;
   use free_cwd to perform the necessary free or close.  Upon failure,
   no memory is allocated, any locally opened file descriptors are
   closed;  return non-zero -- in that case, free_cwd need not be
   called, but doing so is ok.  Otherwise, return zero.

   The _raison d'etre_ for this interface is that the working directory
   is sometimes inaccessible, and getcwd is not robust or as efficient.
   So, we prefer to use the open/fchdir approach, but fall back on
   getcwd if necessary.  This module works for most cases with just
   the getcwd-lgpl module, but to be truly robust, use the getcwd module.

   Some systems lack fchdir altogether: e.g., OS/2, pre-2001 Cygwin,
   SCO Xenix.  Also, SunOS 4 provides the function, yet it doesn't work
   for partitions on which auditing is enabled.  If you're still using
   an obsolete system with these problems, please send email to the
   maintainer of this code.  */

/* C 56-66's `int save_cwd (struct saved_cwd *cwd)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_cwd (cwd: *mut saved_cwd) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* The 'name' member is present only to minimize differences from
     gnulib.  Initialize it to NULL, if only to simplify debugging.  */
  (*cwd).name = std::ptr::null_mut ();

  (*cwd).desc = open (c".".as_ptr (), O_SEARCH | O_CLOEXEC);
  return 0;

  }
}

/* Change to recorded location, CWD, in directory hierarchy.
   Upon failure, return -1 (errno is set by chdir or fchdir).
   Upon success, return zero.  */

/* C 71-83's `int restore_cwd (const struct saved_cwd *cwd)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_cwd (cwd: *const saved_cwd) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* Restore the previous directory if possible, to avoid tying down
     the file system of the new directory (Bug#18232).
     Don't worry if fchdir fails, as Emacs doesn't care what the
     working directory is.  The fchdir call is inside an 'if' merely to
     pacify compilers that complain if fchdir's return value is ignored.  */
  if fchdir ((*cwd).desc) == 0
    { return 0; }

  return 0;

  }
}

/* C 85-89's `void free_cwd (struct saved_cwd *cwd)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_cwd (cwd: *mut saved_cwd)
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  close ((*cwd).desc);

  }
}
