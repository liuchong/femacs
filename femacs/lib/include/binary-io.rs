// femacs: mirror of lib/binary-io.h @ fd7afced403 ; PORTING.md 0.1.
//
// `O_BINARY` is 0 on this target (gnulib's <fcntl.h> fallback), so the header's
// `#if O_BINARY` arm is dead and the live text is the two `_GL_INLINE` functions it gives
// instead: `_gl_set_fd_mode` (returning O_BINARY) and `set_binary_mode`, which is the function
// lib/pipe2.c's own `# if O_BINARY` arm would call.  The `SET_BINARY` macro comes with them.

include!("fcntl.in.rs");       // <fcntl.h> (O_BINARY, O_TEXT)
include!("stdio.rs");          // <stdio.h> (the MSVC7 ordering note; FILE is not used here)

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const _BINARY_H: bool = true;

// C 32-34's `#ifndef BINARY_IO_INLINE / # define BINARY_IO_INLINE _GL_INLINE`.
pub const BINARY_IO_INLINE: () = _GL_INLINE;

/* C 41-46's `# if defined __EMX__ || defined __DJGPP__ || defined __CYGWIN__ … # else # define
   _gl_set_fd_mode _setmode # undef fileno # define fileno _fileno … # endif` arm, kept as text
   (P-51; dead: O_BINARY is 0 on this target, so the header's `#else` arm above is the live
   one).  The `fileno` rename is a macro_rules whose body keeps C's `_fileno` (the getgroups.c
   shape).  */
#[cfg (feature = "o_binary")]
macro_rules! _gl_set_fd_mode { ($fd:expr, $mode:expr) => { _setmode ($fd, $mode) } }
#[cfg (feature = "o_binary")]
macro_rules! fileno { ($f:expr) => { _fileno ($f) } }

/* C 45-50's `BINARY_IO_INLINE int _gl_set_fd_mode (int fd, int mode) { return O_BINARY; }`.  */
#[inline(always)]
pub unsafe fn _gl_set_fd_mode (_fd: c_int, _mode: c_int) -> c_int
{
  return O_BINARY;
}

/* C 72-78's `BINARY_IO_INLINE int set_binary_mode (int fd, int mode)`.  */
#[inline(always)]
pub unsafe fn set_binary_mode (fd: c_int, mode: c_int) -> c_int
{
  return _gl_set_fd_mode (fd, mode);
}

/* C 81-82's `#define SET_BINARY(fd) ((void) set_binary_mode (fd, O_BINARY))`.  */
macro_rules! SET_BINARY {
  ($fd:expr) => { { let _ = set_binary_mode ($fd, O_BINARY); } };
}
