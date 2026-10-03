/* The code is taken from glibc/manual/llio.texi  */

/* emacs/lib/cloexec.c: F_SETFD is defined by the frozen target's <fcntl.h>, so C 39-62's
   `#ifdef F_SETFD` arm - the `fcntl (desc, F_GETFD, 0)` dance - is the live text and C 64-79's
   `#else` arm (the dup2 punt for platforms whose <fcntl.h> has no F_SETFD) is dead; the dead
   arm is kept under the feature that names the non-frozen side (P-51).  C 26's <errno.h>,
   C 27's <fcntl.h> and C 28's <unistd.h> are the errno mirror, the fcntl mirror, and the
   unistd mirror's `dup2` declaration.  */
use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");
include!("include/cloexec.rs");
include!("include/fcntl.in.rs");
include!("include/unistd.in.rs");

/* Set the 'FD_CLOEXEC' flag of DESC if VALUE is true,
   or clear the flag if VALUE is false.
   Return 0 on success, or -1 on error with 'errno' set.

   Note that on MingW, this function does NOT protect DESC from being
   inherited into spawned children.  Instead, either use dup_cloexec
   followed by closing the original DESC, or use interfaces such as
   open or pipe2 that accept flags like O_CLOEXEC to create DESC
   non-inheritable in the first place.  */

/* C 38-80's `int set_cloexec_flag (int desc, bool value)`.  Both arms of C 39's
   `#ifdef F_SETFD` live inside the one function, the malloc.c shape: the frozen
   configuration defines F_SETFD, so the `fcntl` arm below is the live text.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_cloexec_flag (desc: c_int, value: bool) -> c_int
{
  #[cfg (not (feature = "no_f_setfd"))]
  {
    let flags: c_int = fcntl (desc, F_GETFD, 0);

    if 0 <= flags
      {
        let newflags: c_int = if value { flags | FD_CLOEXEC } else { flags & !FD_CLOEXEC };

        if flags == newflags
          || fcntl (desc, F_SETFD, newflags) != -1
          { return 0; }
      }

    return -1;
  }

  /* C 64-79's `#else` arm, kept as text (P-51; dead: F_SETFD is defined).  */

  /* Use dup2 to reject invalid file descriptors; the cloexec flag
     will be unaffected.  */
  #[cfg (feature = "no_f_setfd")]
  {
    if desc < 0
      {
        *errno () = EBADF;
        return -1;
      }
    if dup2 (desc, desc) < 0
      /* errno is EBADF here.  */
      { return -1; }

    /* There is nothing we can do on this kind of platform.  Punt.  */
    return 0;
  }
}

/* Duplicates a file handle FD, while marking the copy to be closed
   prior to exec or spawn.  Returns -1 and sets errno if FD could not
   be duplicated.  */

/* C 79-83's `int dup_cloexec (int fd)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dup_cloexec (fd: c_int) -> c_int
{
  return fcntl (fd, F_DUPFD_CLOEXEC, 0);
}
