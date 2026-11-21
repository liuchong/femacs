// femacs: port of lib/copy-file-range.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
/* emacs/lib/copy-file-range.c (@ 99 lines): the frozen configuration leaves
   HAVE_COPY_FILE_RANGE undefined, so C 19-40's Linux-only prelude (the `__linux__` guard, the
   <linux/version.h>/<sys/utsname.h> includes and the two CHECK_LINUX_KERNEL_VERSION arms) and
   the body's `#if HAVE_COPY_FILE_RANGE` block are all dead text, and the live body is C 95-97 -
   `errno = ENOSYS; return -1;`.  The dead Linux arm is kept as text under the feature that
   names the non-frozen side (P-51); its `utsname`/`uname`/LINUX_VERSION_CODE tokens cannot be
   expressed on this target and are recorded in docs/ports/macro-body-notes.tsv (the
   gettimeofday.c Windows-arm treatment).  C 22's <unistd.h>, C 24's <errno.h> and C 42's
   "sys-limits.h" are the unistd mirror, the errno mirror and the new sys-limits mirror.  */
use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/unistd.in.rs imported via common
// [U1] include/errno.rs imported via common
// [U1] include/sys-limits.rs imported via common

/* C 28-39's `#if defined __linux__ && HAVE_COPY_FILE_RANGE` arm, kept as text (P-51; dead:
   this is a darwin-arm64 target and HAVE_COPY_FILE_RANGE is undefined).  */
#[cfg (feature = "linux_copy_file_range")]
macro_rules! CHECK_LINUX_KERNEL_VERSION { () => { true } }
#[cfg (feature = "linux_copy_file_range_old_kernel")]
macro_rules! CHECK_LINUX_KERNEL_VERSION { () => { false } }

/* C 44-98's `ssize_t copy_file_range (int infd, off_t *pinoff, int outfd, off_t *poutoff,
   size_t length, unsigned int flags)`; C 48's `#undef copy_file_range` restores the system
   declaration for the call in the dead arm, which the port keeps in a `system` module.  */
mod system {
  use super::*;

  unsafe extern "C" {
    pub fn copy_file_range (infd: c_int, pinoff: *mut off_t, outfd: c_int,
                            poutoff: *mut off_t, length: usize, flags: c_uint) -> ssize_t;
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn copy_file_range (infd: c_int, pinoff: *mut off_t, outfd: c_int,
                                          poutoff: *mut off_t, length: usize,
                                          flags: c_uint) -> ssize_t
{
  /* C 50-92's `#if HAVE_COPY_FILE_RANGE` arm, kept as text (P-51; dead: the macro is
     undefined on this target).  The Linux-kernel prelude it opens with is noted rather than
     written, because `struct utsname` and `uname` have no mirror here.  */
  #[cfg (feature = "have_copy_file_range")]
  {
    let mut ok: bool = true;

    /* C 56-82's `# if CHECK_LINUX_KERNEL_VERSION` arm (dead even in the dead arm: the
       frozen table has no LINUX_VERSION_CODE).  Its tokens are in
       docs/ports/macro-body-notes.tsv.  */

    if ok
      {
        /* C 85-90's `# if defined __GLIBC__ && ! (2 < __GLIBC__ + (43 <= __GLIBC_MINOR__))`:
           the glibc 2.42 workaround, whose SYS_BUFSIZE_MAX is this target's own limit.  */
        #[cfg (feature = "glibc_copy_file_range_bug_33245")]
        {
          if SYS_BUFSIZE_MAX < length as c_int
            { length = SYS_BUFSIZE_MAX as usize; }
        }

        return system::copy_file_range (infd, pinoff, outfd, poutoff, length, flags);
      }
  }

  /* There is little need to emulate copy_file_range with read+write,
     since programs that use copy_file_range must fall back on
     read+write anyway.  */
  *errno () = ENOSYS;
  return -1;
}
