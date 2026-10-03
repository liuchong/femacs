// femacs: mirror of lib/issymlink.h @ fd7afced403 ; PORTING.md 0.1.
//
// The header's one function is `_GL_ISSYMLINK_INLINE`-qualified (the C99 inline model,
// PORTING.md 10.6/10.12), so it is an ordinary Rust item here and the `.c` wrapper
// `femacs/lib/issymlink.rs` keeps the qualifier relationship.  `readlink`, `errno` and `EINVAL`
// come from the unistd and errno mirrors.

include!("sys_types.in.rs");   // <sys/types.h>
include!("unistd.in.rs");      // <unistd.h> (readlink)
include!("errno.rs");          // <errno.h> (errno, EINVAL)

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const _ISSYMLINK_H: bool = true;

// C's `#ifndef _GL_ISSYMLINK_INLINE / # define _GL_ISSYMLINK_INLINE _GL_INLINE` spells gnulib's inline model; the wrapper
// `femacs/lib/issymlink.rs` carries the qualifier (the timespec/stat-time shape), so its box stays
// unchecked in docs/ports/include-issymlink.md.

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
  let mut linkbuf: [c_char; 1] = [0];
  if readlink (filename, linkbuf.as_mut_ptr (), linkbuf.len ()) >= 0
    { return 1; }
  if *errno () == EINVAL
    { return 0; }
  else
    { return -1; }
}
