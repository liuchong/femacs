// femacs: mirror of lib/issymlink.h @ fd7afced403 ; PORTING.md 0.1.
//
// The header's one function is `_GL_ISSYMLINK_INLINE`-qualified (the C99 inline model,
// PORTING.md 10.6/10.12), so it is an ordinary Rust item here and the `.c` wrapper
// `lib/issymlink.rs` keeps the qualifier relationship.  `readlink`, `errno` and `EINVAL`
// come from the unistd and errno mirrors.

// [U1-MODULE-MIRROR]
use super::*;
// [U1] sys_types.in.rs imported via super
// [U1] unistd.in.rs imported via super
// [U1] errno.rs imported via super

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const _ISSYMLINK_H: bool = true;

/* C 31-33: `#ifndef _GL_ISSYMLINK_INLINE / # define _GL_ISSYMLINK_INLINE _GL_INLINE`.  The
   wrapper `lib/issymlink.rs` defines the name before including this header (the
   acl-internal shape), so the mirror records it as C's `_GL_INLINE` is - the empty-value
   marker `gnulib_config.rs` carries.  */
/* The name is not carried here: the wrapper `lib/issymlink.rs` defines it before
   including this header (C 20-21 do `#define _GL_ISSYMLINK_INLINE _GL_EXTERN_INLINE`), so a const in
   the mirror would be E0428 against it.  The gate records it in
   docs/ports/excluded-macros.txt instead (the `_GL_TIMESPEC_INLINE` shape).  */

/* Tests whether FILENAME represents a symbolic link.
   This function is more reliable than lstat() / fstatat() followed by S_ISLNK,
   because it avoids possible EOVERFLOW errors.
   Returns
     1                      if FILENAME is a symbolic link,
     0                      if FILENAME exists and is not a symbolic link,
    -1 with errno set       if determination failed, in particular
    -1 with errno = ENOENT or ENOTDIR  if FILENAME does not exist.  */
#[inline(always)]
pub unsafe fn issymlink (filename: *const c_char) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let mut linkbuf: [c_char; 1] = [0];
  if readlink (filename, linkbuf.as_mut_ptr (), linkbuf.len ()) >= 0
    { return 1; }
  if *errno () == EINVAL
    { return 0; }
  else
    { return -1; }

  }
}
