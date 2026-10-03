// femacs: port of lib/strnlen.c @ fd7afced403.
use std::ffi::c_char;
include!("include/gnulib_config.rs");

/* C's `<string.h>` include has no counterpart: the body below calls nothing from it (that
   is the point of the comment it carries).  */
/* Find the length of S, but scan at most MAXLEN bytes.
   S must be a string if it starts with fewer than MAXLEN initialized bytes.
   If no '\0' terminator is found in that many bytes, return MAXLEN.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strnlen (s: *const c_char, maxlen: usize) -> usize
{
  /* Do not use memchr, because on some platforms memchr has
     undefined behavior if MAXLEN exceeds the number of bytes in S.  */
  let mut i: usize = 0;
  while i < maxlen && *s.offset (i as isize) != 0
    { i += 1; }
  return i;
}
