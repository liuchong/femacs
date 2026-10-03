// femacs: port of lib/careadlinkat.c @ fd7afced403.
/* Written by Paul Eggert, Bruno Haible, and Jim Meyering.  */

/* emacs/lib/careadlinkat.c (@ 173 lines): no `#if` decides any statement of the two
   functions, so the whole file is live text; the only conditional constructs are C 36-38's
   `#ifndef SIZE_MAX` fallback (dead: <stdint.h>'s SIZE_MAX is in scope through limits.in.rs),
   C 55-57's `#if _GL_GNUC_PREREQ (10, 1)` attribute (dropped, PORTING.md 0.3) and the header's
   `AT_FDCWD` arm.  C 21-31's includes are the mirrors below; C 40's `enum { STACK_BUF_SIZE =
   1024 }` is a constant, and `struct allocator`/`stdlib_allocator` come from the new allocator
   mirror.  */
use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/careadlinkat.rs");
include!("include/idx.rs");
include!("include/minmax.rs");
include!("include/limits.in.rs");
include!("include/string.rs");
include!("include/unistd.in.rs");
include!("include/errno.rs");
include!("include/allocator.rs");

/* <limits.h>'s `SSIZE_MAX` (LONG_MAX on this target, measured): the limits mirror does not
   carry it yet, so this unit declares the value C's <limits.h> would have given it (the
   getdelim.rs/regex_internal.rs shape; see docs/status.md 3.21).  */
pub const SSIZE_MAX: c_long = c_long::MAX;

/* C 36-38's `#ifndef SIZE_MAX / # define SIZE_MAX ((size_t) -1)`.  This file does not include
   <stdint.h>, and `tools/check-cond-eval.py` measures that the macro is *not* defined by the
   headers it does include (0 disagreements), so C's fallback is the live text and the port
   keeps it unconditionally - `((size_t) -1)` is `usize::MAX`.  */
pub const SIZE_MAX: usize = usize::MAX;

/* C 40's `enum { STACK_BUF_SIZE = 1024 };`.  */
pub const STACK_BUF_SIZE: usize = 1024;

/* Act like careadlinkat (see below), with an additional argument
   STACK_BUF that can be used as temporary storage.  */

/* C 58-143's `static char *readlink_stk (…)`.  */
unsafe fn readlink_stk (fd: c_int, filename: *const c_char,
                        mut buffer: *mut c_char, mut buffer_size: usize,
                        mut alloc: *const allocator,
                        preadlinkat: Option<unsafe extern "C" fn (c_int, *const c_char,
                                                                  *mut c_char, usize)
                                                       -> ssize_t>,
                        stack_buf: *mut c_char) -> *mut c_char
{
  if alloc.is_null ()
    { alloc = std::ptr::addr_of! (stdlib_allocator); }

  if buffer.is_null ()
    {
      buffer = stack_buf;
      buffer_size = STACK_BUF_SIZE;
    }

  let mut buf: *mut c_char = buffer;
  /* C's `MIN (IDX_MAX, MIN (SSIZE_MAX, SIZE_MAX))` is computed in the common type of the three
     operands: SIZE_MAX is `size_t` (unsigned), so the inner MIN is an unsigned comparison
     (SSIZE_MAX is the smaller of the two) and the outer one is signed (IDX_MAX is PTRDIFF_MAX,
     the same value), which is what the two stepwise conversions below spell.  */
  let size_limit: u64 = MIN! (SSIZE_MAX as u64, SIZE_MAX as u64);
  let buf_size_max: idx_t = MIN! (IDX_MAX, size_limit as idx_t);
  let mut buf_size: idx_t = MIN! (buffer_size, buf_size_max as usize) as idx_t;

  while !buf.is_null ()
    {
      /* Attempt to read the link into the current buffer.  */
      let link_length: idx_t = preadlinkat.unwrap () (fd, filename, buf, buf_size as usize)
        as idx_t;
      if link_length < 0
        {
          if buf != buffer
            {
              let readlinkat_errno: c_int = *errno ();
              (*alloc).free.unwrap () (buf as *mut c_void);
              *errno () = readlinkat_errno;
            }
          return std::ptr::null_mut ();
        }

      let mut link_size: idx_t = link_length;

      if (link_size as usize) < buf_size as usize
        {
          *buf.offset (link_size as isize) = b'\0' as c_char;
          link_size += 1;

          if buf == stack_buf
            {
              let b: *mut c_char = (*alloc).allocate.unwrap () (link_size as usize) as *mut c_char;
              buf_size = link_size;
              if b.is_null ()
                { break; }
              return memcpy (b as *mut c_void, buf as *const c_void, link_size as usize)
                as *mut c_char;
            }

          if (link_size as usize) < buf_size as usize && buf != buffer
            && (*alloc).reallocate.is_some ()
            {
              /* Shrink BUF before returning it.  */
              let b: *mut c_char = (*alloc).reallocate.unwrap () (buf as *mut c_void,
                                                                  link_size as usize)
                as *mut c_char;
              if !b.is_null ()
                { return b; }
            }

          return buf;
        }

      if buf != buffer
        { (*alloc).free.unwrap () (buf as *mut c_void); }

      if buf_size_max / 2 <= buf_size
        {
          *errno () = ENAMETOOLONG;
          return std::ptr::null_mut ();
        }

      buf_size = 2 * buf_size + 1;
      buf = (*alloc).allocate.unwrap () (buf_size as usize) as *mut c_char;
    }

  if (*alloc).die.is_some ()
    { (*alloc).die.unwrap () (buf_size as usize); }
  *errno () = ENOMEM;
  return std::ptr::null_mut ();
}

/* C 161-173's `char *careadlinkat (…)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn careadlinkat (fd: c_int, filename: *const c_char,
                                       buffer: *mut c_char, buffer_size: usize,
                                       alloc: *const allocator,
                                       preadlinkat: Option<unsafe extern "C" fn (c_int,
                                                                                 *const c_char,
                                                                                 *mut c_char,
                                                                                 usize)
                                                                            -> ssize_t>)
  -> *mut c_char
{
  /* Allocate the initial buffer on the stack.  This way, in the
     common case of a symlink of small size, we get away with a
     single small malloc instead of a big malloc followed by a
     shrinking realloc.  */
  let mut stack_buf: [c_char; STACK_BUF_SIZE] = [0; STACK_BUF_SIZE];
  return readlink_stk (fd, filename, buffer, buffer_size, alloc,
                       preadlinkat, stack_buf.as_mut_ptr ());
}
