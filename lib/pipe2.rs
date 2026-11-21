// femacs: port of lib/pipe2.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
/* emacs/lib/pipe2.c (@ 166 lines) is on the verified list: HAVE_PIPE2 is undefined and
   GNULIB_defined_O_NONBLOCK is undefined on this target, so C 44-61's `#if HAVE_PIPE2` arm,
   C 66-103's `_WIN32` arm and the `nonblocking.h` include are dead by measurement, and the live
   text is the flag check, the Unix pipe+fcntl path, the dead `# if O_BINARY` arm (O_BINARY is
   0, so the port keeps C's O_BINARY/O_TEXT tokens in the flag check only) and C's `fail:` label
   block.  C 24's <errno.h> and C 25's <fcntl.h> are the errno and fcntl mirrors, C 27's
   "binary-io.h" the new binary-io mirror, and <unistd.h>'s `pipe`/`close` come from the unistd
   mirror.  */
use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/fcntl.in.rs imported via common
// [U1] include/binary-io.rs imported via common
// [U1] include/unistd.in.rs imported via common
// [U1] include/errno.rs imported via common

/* C 39-166's `int pipe2 (int fd[2], int flags)`.  C's three `goto fail` jumps become
   `break 'fail` out of the labelled block that wraps the live body (PORTING.md 2.2); the
   forward jump lands on the same cleanup C writes at its `fail:` label.  */
#[unsafe(export_name = "rpl_pipe2")]
pub unsafe extern "C" fn pipe2 (fd: *mut c_int, flags: c_int) -> c_int
{
  /* Mingw _pipe() corrupts fd on failure; also, if we succeed at
     creating the pipe but later fail at changing fcntl, we want
     to leave fd unchanged: https://austingroupbugs.net/view.php?id=467  */
  let mut tmp: [c_int; 2] = [0, 0];
  tmp[0] = *fd;
  tmp[1] = *fd.add (1);

  /* C 44-61's `#if HAVE_PIPE2` arm is dead (HAVE_PIPE2 is undefined here): the port does not
     carry its `static int have_pipe2_really` cache or its system `pipe2` call.  */

  /* Check the supported flags.  */
  if (flags & !(O_CLOEXEC | O_NONBLOCK | O_BINARY | O_TEXT)) != 0
    {
      *errno () = EINVAL;
      return -1;
    }

  /* C 66-103's `#if defined _WIN32 && ! defined __CYGWIN__` arm is dead on this target.  */

  'fail: {
    /* Unix API.  */
    if pipe (fd) < 0
      { return -1; }

    /* POSIX says that initially, the O_NONBLOCK and FD_CLOEXEC flags are cleared on
       both fd[0] and fd[1].  */

    /* O_NONBLOCK handling: on Unix platforms, O_NONBLOCK is defined by the system, so the
       port uses fcntl().  */
    if flags & O_NONBLOCK != 0
      {
        let mut fcntl_flags: c_int;

        fcntl_flags = fcntl (*fd.add (1), F_GETFL, 0);
        if fcntl_flags < 0
          || fcntl (*fd.add (1), F_SETFL, fcntl_flags | O_NONBLOCK) == -1
          { break 'fail; }
        fcntl_flags = fcntl (*fd, F_GETFL, 0);
        if fcntl_flags < 0
          || fcntl (*fd, F_SETFL, fcntl_flags | O_NONBLOCK) == -1
          { break 'fail; }
      }

    if flags & O_CLOEXEC != 0
      {
        let mut fcntl_flags: c_int;

        fcntl_flags = fcntl (*fd.add (1), F_GETFD, 0);
        if fcntl_flags < 0
          || fcntl (*fd.add (1), F_SETFD, fcntl_flags | FD_CLOEXEC) == -1
          { break 'fail; }
        fcntl_flags = fcntl (*fd, F_GETFD, 0);
        if fcntl_flags < 0
          || fcntl (*fd, F_SETFD, fcntl_flags | FD_CLOEXEC) == -1
          { break 'fail; }
      }

    /* C 146-156's `# if O_BINARY` arm is dead (O_BINARY is 0 here), so the port does not
       carry the two `set_binary_mode` calls it would make.  */

    return 0;
  }

  /* C 159-166's `fail:` label, live because GNULIB_defined_O_NONBLOCK is undefined and this
     is not a native Windows target.  */
  {
    let saved_errno: c_int = *errno ();
    close (*fd);
    close (*fd.add (1));
    *fd = tmp[0];
    *fd.add (1) = tmp[1];
    *errno () = saved_errno;
    return -1;
  }
}
