// femacs: port of lib/dup2.c @ fd7afced403.
//
// `rpl_dup2` is the live function; the two arms above it are the non-frozen sides:
// C 30-98's `#if defined _WIN32 && ! defined __CYGWIN__` arm (the mingw32/Wine workaround)
// and C 100-173's `#elif defined __KLIBC__` arm (the OS/2 kLIBC workaround).  Both need
// headers this target has not got (`<windows.h>`, `<io.h>`, `<InnoTekLIBC/backend.h>`), so
// the port keeps their functions as `#[cfg]`-guarded text - the same name C gives them, one
// function each, never a renamed twin (the malloc.c/cloexec.c shape) - and
// docs/ports/macro-body-notes.tsv records the tokens of the statements those arms spell with
// the missing headers (the gettimeofday.c Windows-arm precedent).
//
// Deviations (docs/ports/dup2.md): the two `# define dup2 …` renames are `macro_rules!`
// twins (the live call `dup2 (fd, desired_fd)` names the system entry, which the unistd
// mirror declares), `#undef dup2` has no counterpart, and the `#ifdef F_GETFL` arm is live
// (the frozen <fcntl.h> defines it) while `HAVE_SETDTABLESIZE`/`REPLACE_FCHDIR` are not.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* written by Paul Eggert */

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/unistd.in.rs");
include!("include/fcntl.in.rs");
include!("include/errno.in.rs");

/* C 25's `#undef dup2`: the port's live call names the system entry directly.  */

/* `<errno.h>`'s `EMFILE`, which C 205's `*errno () = EMFILE` names, comes from the errno
   mirror (24, the value this unit's own comment recorded while it still declared the code
   itself; the copy was retired when the mirror grew it, status 3.80).  */

/* C 30-98's `#if defined _WIN32 && ! defined __CYGWIN__` arm (dead on this target).  */

/* <windows.h>'s `HANDLE`: a `void *` on the Windows targets the dead arm is for.  */
pub type handle = *mut c_void;

/* C 33's `# define WIN32_LEAN_AND_MEAN` is recorded in docs/ports/excluded-macros.txt.  */

/* C 49-64's `dup2_nothrow` (the `HAVE_MSVC_INVALID_PARAMETER_HANDLER` arm; the other arm,
   C 67, is `# define dup2_nothrow _dup2`).  */
#[cfg (feature = "windows_native")]
unsafe extern "C" fn dup2_nothrow (fd: c_int, desired_fd: c_int) -> c_int
{
  let result: c_int;

  /* TRY_MSVC_INVAL { DONE_MSVC_INVAL } */
  result = _dup2 (fd, desired_fd);

  if false
    {
      *errno () = EBADF;
      return -1;
    }

  return result;
}

/* C 71-95's `ms_windows_dup2`.  */
#[cfg (feature = "windows_native")]
unsafe extern "C" fn ms_windows_dup2 (fd: c_int, desired_fd: c_int) -> c_int
{
  /* If fd is closed, mingw hangs on dup2 (fd, fd).  If fd is open,
     dup2 (fd, fd) returns 0, but all further attempts to use fd in
     future dup2 calls will hang.  */
  if (fd == desired_fd)
    {
      if _get_osfhandle (fd) as handle == INVALID_HANDLE_VALUE
        {
          *errno () = EBADF;
          return -1;
        }
      return fd;
    }

  /* Wine 1.0.1 return 0 when desired_fd is negative but not -1:
     https://bugs.winehq.org/show_bug.cgi?id=21289 */
  if (desired_fd < 0)
    {
      *errno () = EBADF;
      return -1;
    }

  let mut result: c_int = dup2_nothrow (fd, desired_fd);

  if (result == 0)
    { result = desired_fd; }

  return result;
}

/* C 97's `# define dup2 ms_windows_dup2`.  */
#[cfg (feature = "windows_native")]
macro_rules! dup2 {
    ($fd:expr, $desired_fd:expr) => { ms_windows_dup2 ($fd, $desired_fd) };
}

/* C 100-173's `#elif defined __KLIBC__` arm (dead on this target).  */

/* C 109-153's `klibc_dup2dirfd`.  */
#[cfg (feature = "klibc")]
unsafe extern "C" fn klibc_dup2dirfd (fd: c_int, mut desired_fd: c_int) -> c_int
{
  let mut tempfd: c_int = open (c"NUL".as_ptr (), O_RDONLY);
  if (tempfd < 0)
    { return tempfd; }

  if (tempfd >= desired_fd)
    {
      close (tempfd);

      let mut path: [c_char; _MAX_PATH as usize] = [0; _MAX_PATH as usize];
      if (__libc_Back_ioFHToPath (fd, path.as_mut_ptr (), path.len ()) != 0)
        { return -1; }

      loop
        {
          close (desired_fd);

          let dupfd: c_int = open (path.as_ptr (), O_RDONLY);
          if (dupfd < 0)
            { return dupfd; }

          if (dupfd == desired_fd)
            { return dupfd; }

          /* If lower FD was closed by other threads, fill again.  */
          if (dupfd < desired_fd)
            {
              tempfd = dupfd;
              break;
            }

          /* desired_fd was opened by other threads. Try again.  */
          /* FIXME: Closing desired_fd opened by other threads may lead to
             unexpected behavior.  */
          close (dupfd);
        }
    }

  let dupfd: c_int = klibc_dup2dirfd (fd, desired_fd);

  close (tempfd);

  return dupfd;
}

/* C 156-170's `klibc_dup2`.  */
#[cfg (feature = "klibc")]
unsafe extern "C" fn klibc_dup2 (fd: c_int, desired_fd: c_int) -> c_int
{
  let dupfd: c_int = system::dup2 (fd, desired_fd);
  if (dupfd < 0 && *errno () == ENOTSUP)
    {
      let mut sbuf: stat = std::mem::zeroed ();
      if (!fstat (fd, &mut sbuf) && S_ISDIR (sbuf.st_mode))
        {
          close (desired_fd);

          return klibc_dup2dirfd (fd, desired_fd);
        }
    }

  return dupfd;
}

/* C 172's `# define dup2 klibc_dup2`.  */
#[cfg (feature = "klibc")]
macro_rules! dup2 {
    ($fd:expr, $desired_fd:expr) => { klibc_dup2 ($fd, $desired_fd) };
}

/* C 177-205's live function: the two arms above are dead, so its `dup2` call is the system
   entry the unistd mirror declares (through `system::dup2`, which keeps the C name).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_dup2 (mut fd: c_int, desired_fd: c_int) -> c_int
{
  /* #ifdef F_GETFL — live: the frozen <fcntl.h> defines it.  */
  {
  /* On Linux kernels 2.6.26-2.6.29, dup2 (fd, fd) returns -EBADF.
     On Cygwin 1.5.x, dup2 (1, 1) returns 0.
     On Cygwin 1.7.17, dup2 (1, -1) dumps core.
     On Cygwin 1.7.25, dup2 (1, 256) can dump core.
     On Haiku, dup2 (fd, fd) mistakenly clears FD_CLOEXEC.  */
  /* C 182-184's `# if HAVE_SETDTABLESIZE` arm, kept as text (P-51; dead).  */
  #[cfg (feature = "have_setdtablesize")]
  {
    setdtablesize (desired_fd + 1);
  }
  if (desired_fd < 0)
    { fd = desired_fd; }
  if (fd == desired_fd)
    { return if fcntl (fd, F_GETFL) == -1 { -1 } else { fd }; }
  }

  let mut result: c_int = system::dup2 (fd, desired_fd);

  /* Correct an errno value on FreeBSD 6.1 and Cygwin 1.5.x.  */
  if (result < 0 && *errno () == EMFILE)
    { *errno () = EBADF; }

  /* C 197-200's `#if REPLACE_FCHDIR` arm, kept as text (P-51; dead).  */
  #[cfg (feature = "replace_fchdir")]
  {
    if ! (result < 0 || fd == desired_fd)
      { result = _gl_register_dup (fd, result); }
  }
  return result;
}

/* C 20's <unistd.h> declares `dup2`; the file's own `#undef dup2` makes the live call name
   that entry, and the port needs it out of the way of the dead arms' `macro_rules! dup2`.  */
mod system {
  use super::*;

  unsafe extern "C" {
    pub fn dup2 (fd: c_int, fd2: c_int) -> c_int;
  }
}
