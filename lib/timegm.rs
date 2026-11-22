#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/timegm.c @ fd7afced403.  `_LIBC` is undefined,
// so C 20-22's `<libc-config.h>` include is live and C 36-57's `#if defined _LIBC &&
// __TIMESIZE != 64` arm (the 32-bit `timegm` wrapper) is dead.  mktime-internal.h renames
// the definition twice on this target: `__timegm64` -> `timegm` (mktime-internal.h 53) and
// `timegm` -> `rpl_timegm` (the generated <time.h>'s REPLACE_TIMEGM arm), which is the
// symbol the reference object carries: `nm -g build/emacs-ref/lib/timegm.o` prints
// `T _rpl_timegm`.  The port keeps C's source name and emits the real symbol with
// `#[unsafe(export_name = ...)]` (PORTING.md 10.13①, the mempcpy precedent).
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/errno.rs imported via common
// [U1] include/mktime-internal.rs imported via common

/* __time64_t __timegm64 (struct tm *tmp)  */
#[unsafe(export_name = "rpl_timegm")]
pub unsafe extern "C" fn __timegm64 (tmp: *mut tm) -> __time64_t
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* C 32: `static mktime_offset_t gmtime_offset;` - C zero-initializes it.  */
  static gmtime_offset: VmGlobal<mktime_offset_t> = VmGlobal::<mktime_offset_t>::new (0);
  return __mktime_internal! (tmp, false, gmtime_offset.as_mut_ptr ());

  }
}
