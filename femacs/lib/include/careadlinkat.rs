// femacs: mirror of lib/careadlinkat.h @ fd7afced403 ; PORTING.md 0.1.
//
// The header is `struct allocator`'s forward declaration, one prototype and the
// `AT_FDCWD` fallback; the definition is `femacs/lib/careadlinkat.rs`, so the prototype sits
// in a whole-import module the definition shadows (PORTING.md 0.1-10).

include!("sys_types.in.rs");   // <sys/types.h> (ssize_t)
include!("fcntl.in.rs");       // <fcntl.h> (AT_FDCWD)

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const _GL_CAREADLINKAT_H: bool = true;

/// C: char *careadlinkat (int fd, char const *filename, char *restrict buffer,
///                        size_t buffer_size, struct allocator const *alloc,
///                        ssize_t (*preadlinkat) (int, char const *, char *, size_t))
pub mod careadlinkat_decls {
  use super::*;

  unsafe extern "C" {
    pub fn careadlinkat (fd: c_int, filename: *const c_char, buffer: *mut c_char,
                         buffer_size: usize, alloc: *const allocator,
                         preadlinkat: Option<unsafe extern "C" fn (c_int, *const c_char,
                                                                   *mut c_char, usize)
                                                       -> ssize_t>)
      -> *mut c_char;
  }
}
use careadlinkat_decls::*;

include!("allocator.rs");      // struct allocator (the prototype's fifth parameter)

/* C 72-80's `#if HAVE_READLINKAT … #else … #endif`: live here (HAVE_READLINKAT is 1, so
   AT_FDCWD comes from <fcntl.h>), and the dead arm's value is kept as the constant C would
   have defined (P-51; this header is not on the verified list, so the checker still wants the
   token).  */
#[cfg (feature = "no_readlinkat")]
pub const AT_FDCWD: c_int = -3041965;
