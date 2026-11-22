// femacs: port of lib/malloc/scratch_buffer_grow.c @ fd7afced403.
//
// C 19-21's `#ifndef _LIBC` arm is live: `<libc-config.h>`'s `__set_errno` (below) is taken,
// C 56's `libc_hidden_def (…)` stays empty, and C 23's `<scratch_buffer.h>` is included below.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_void};
// [U1] ../include/errno.rs imported via common
// [U1] ../include/scratch_buffer.rs imported via common
/* C 19-21's `<libc-config.h>`: `# define __set_errno(val) (errno = (val))` (C 46).  */
macro_rules! __set_errno { ($val:expr) => { *errno () = $val }; }
/* `<sys/cdefs.h>`'s branch hints (PORTING.md 2.6), as lib/regcomp.rs spells the pair.  */
fn __glibc_likely<T> (cond: T) -> T { cond }
fn __glibc_unlikely<T> (cond: T) -> T { cond }
/* void *malloc (size_t size); -- the generated `<stdlib.h>` does not rename it (`nm` on the
   reference object prints `U _malloc`; its `free` is the mirror's `rpl_free`).  */
unsafe extern "C" { pub fn malloc (size: usize) -> *mut c_void; }

/* Grow *BUFFER by some arbitrary amount.  The buffer contents is NOT
   preserved.  Return true on success, false on allocation failure (in
   which case the old buffer is freed).  On success, the new buffer is
   larger than the previous size.  On failure, *BUFFER is deallocated,
   but remains in a free-able state, and errno is set.  */
/* bool __libc_scratch_buffer_grow (struct scratch_buffer *buffer) -- the frozen object carries
   C's real name (`nm …/scratch_buffer_grow.o` prints `T __gl_scratch_buffer_grow`, i.e.
   `_gl_scratch_buffer_grow`, C 23's wrapper rename, PORTING.md 10.13).  */
#[unsafe(export_name = "_gl_scratch_buffer_grow")]
pub unsafe extern "C" fn __libc_scratch_buffer_grow (buffer: *mut scratch_buffer) -> bool
{
  // SAFETY: gnulib/OS raw buffer or dynamic-array boundary; caller upholds pointer validity, element capacity, and single-owner mutation invariants, returning error/null on overflow or OOM.
  unsafe {
  let mut new_ptr: *mut c_void;
  let new_length: usize = (*buffer).length * 2;

  /* Discard old buffer.  */
  scratch_buffer_free (buffer);

  /* Check for overflow.  */
  if __glibc_likely (new_length >= (*buffer).length)
    { new_ptr = malloc (new_length); }
  else
    {
      __set_errno! (ENOMEM);
      new_ptr = std::ptr::null_mut ();
    }

  if __glibc_unlikely (new_ptr.is_null ())
    {
      /* Buffer must remain valid to free.  */
      scratch_buffer_init (buffer);
      return false;
    }

  /* Install new heap-based buffer.  */
  (*buffer).data = new_ptr;
  (*buffer).length = new_length;
  return true;

  }
}
/* C 56's `libc_hidden_def (__libc_scratch_buffer_grow)`: empty in this build.  */
