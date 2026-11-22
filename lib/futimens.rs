// femacs: port of lib/futimens.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
/* written by Eric Blake */

use std::ffi::{c_char, c_double, c_int, c_long, c_void};
// [U1] config.rs provided by common
// [U1] ../lib/include/timespec.rs imported via common

/* utimens.h declares fdutimens; its own mirror lands with lib/utimens.rs.  */
unsafe extern "C" { pub fn fdutimens (fd: c_int, file: *const c_char, times: *const timespec) -> c_int; }

/* Set the access and modification timestamps of FD to be
   TIMESPEC[0] and TIMESPEC[1], respectively.
   Fail with ENOSYS on systems without futimes (or equivalent).
   If TIMESPEC is null, set the timestamps to the current time.
   Return 0 on success, -1 (setting errno) on failure.  */
#[unsafe(export_name = "rpl_futimens")]
pub unsafe extern "C" fn futimens (fd: c_int, times: *const timespec) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* fdutimens also works around bugs in native futimens, when running
     with glibc compiled against newer headers but on a Linux kernel
     older than 2.6.32.  */
  return fdutimens (fd, std::ptr::null (), times);

  }
}
