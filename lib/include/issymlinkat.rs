// femacs: mirror of lib/issymlinkat.h @ fd7afced403 ; PORTING.md 0.1.
//
// The header's one function is `_GL_ISSYMLINKAT_INLINE`-qualified (PORTING.md 10.6/10.12), so
// it is an ordinary Rust item here and the `.c` wrapper `lib/issymlinkat.rs` keeps the
// qualifier relationship.  `readlinkat`, `errno` and `EINVAL` come from the unistd and errno
// mirrors.

include!("sys_types.in.rs");   // <sys/types.h>
include!("unistd.in.rs");      // <unistd.h> (readlinkat)
include!("errno.rs");          // <errno.h> (errno, EINVAL)

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const _ISSYMLINKAT_H: bool = true;

/* C 31-33: `#ifndef _GL_ISSYMLINKAT_INLINE / # define _GL_ISSYMLINKAT_INLINE _GL_INLINE`.
   The wrapper `lib/issymlinkat.rs` defines the name before including this header (the
   acl-internal shape), so the mirror records it as C's `_GL_INLINE` is.  */
/* The name is not carried here: the wrapper `lib/issymlinkat.rs` defines it before
   including this header (C 20-21 do `#define _GL_ISSYMLINKAT_INLINE _GL_EXTERN_INLINE`), so a const in
   the mirror would be E0428 against it.  The gate records it in
   docs/ports/excluded-macros.txt instead (the `_GL_TIMESPEC_INLINE` shape).  */

/* Tests whether FILENAME represents a symbolic link.
   This function is more reliable than lstat() / fstatat() followed by S_ISLNK,
   because it avoids possible EOVERFLOW errors.
   If FILENAME is a relative file name, it is interpreted as relative to the
   directory referred to by FD (where FD = AT_FDCWD denotes the current
   directory).
   Returns
     1                      if FILENAME is a symbolic link,
     0                      if FILENAME exists and is not a symbolic link,
    -1 with errno set       if determination failed, in particular
    -1 with errno = ENOENT or ENOTDIR  if FILENAME does not exist.  */
#[inline(always)]
pub unsafe fn issymlinkat (fd: c_int, filename: *const c_char) -> c_int
{
  let mut linkbuf: [c_char; 1] = [0];
  if readlinkat (fd, filename, linkbuf.as_mut_ptr (), linkbuf.len ()) >= 0
    { return 1; }
  if *errno () == EINVAL
    { return 0; }
  else
    { return -1; }
}
