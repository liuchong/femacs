// femacs: WIP mirror of lib/sys_random.in.h @ fd7afced403 (GNU Emacs 32.0.50); PORTING.md P-26.
//
// The reference is a gnulib template (`@PRAGMA_SYSTEM_HEADER@`, `@GUARD_PREFIX@`,
// `@HAVE_SYS_RANDOM_H@`, `@INCLUDE_NEXT@`), so `tools/check-cond-eval.py` cannot decide its
// conditions and it stays off the verified list; this mirror carries what the ported units
// name and the unchecked boxes of docs/ports/include-sys_random.in.md are the rest.

include!("sys_types.in.rs");   // <sys/types.h>

/* C 56-60's `#ifndef GRND_NONBLOCK / # define GRND_NONBLOCK 1 / # define GRND_RANDOM 2`:
   this target's system header has no <sys/random.h>, so gnulib's fallback is the live value
   (the same block defines both under one `#ifndef`, as C does).  */
pub const GRND_NONBLOCK: c_int = 1;
pub const GRND_RANDOM: c_int = 2;

/// C: ssize_t getrandom (void *buffer, size_t length, unsigned int flags)
pub mod sys_random_decls {
  use super::*;

  unsafe extern "C" {
    pub fn getrandom (buffer: *mut core::ffi::c_void, length: usize, flags: c_uint) -> ssize_t;
  }
}
use sys_random_decls::*;
