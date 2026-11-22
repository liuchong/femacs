// femacs: port of lib/malloc/scratch_buffer_set_array_size.c @ fd7afced403.
//
// C 19-21's `#ifndef _LIBC` arm is live: `<libc-config.h>`'s `__set_errno` (below) is taken,
// C 64's `libc_hidden_def (…)` stays empty, and C 23's `<scratch_buffer.h>` is included below.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_uint, c_long, c_ulong, c_void};
// [U1] ../include/errno.rs imported via common
// [U1] ../include/limits.in.rs imported via common
// [U1] ../include/scratch_buffer.rs imported via common
/* C 19-21's `<libc-config.h>`: `# define __set_errno(val) (errno = (val))` (C 46).  */
macro_rules! __set_errno { ($val:expr) => { *errno () = $val }; }
/* void *malloc (size_t size); -- the generated `<stdlib.h>` does not rename it (`U _malloc`).  */
unsafe extern "C" { pub fn malloc (size: usize) -> *mut c_void; }

/* Grow *BUFFER so that it can store at least NELEM elements of SIZE
   bytes.  The buffer contents are NOT preserved.  Both NELEM and SIZE
   can be zero.  Return true on success, false on allocation failure
   (in which case the old buffer is freed, but *BUFFER remains in a
   free-able state, and errno is set).  It is unspecified whether this
   function can reduce the array size.  */
/* bool __libc_scratch_buffer_set_array_size (struct scratch_buffer *buffer, size_t nelem,
   size_t size) -- the frozen object carries C's real name (`T
   __gl_scratch_buffer_set_array_size`, C 23's wrapper rename, PORTING.md 10.13).  */
#[unsafe(export_name = "_gl_scratch_buffer_set_array_size")]
pub unsafe extern "C" fn __libc_scratch_buffer_set_array_size (buffer: *mut scratch_buffer,
                                                               nelem: usize, size: usize) -> bool
{
  // SAFETY: gnulib/OS raw buffer or dynamic-array boundary; caller upholds pointer validity, element capacity, and single-owner mutation invariants, returning error/null on overflow or OOM.
  unsafe {
  let new_length: usize = nelem * size;

  /* Avoid overflow check if both values are small. */
  if (nelem | size) >> (std::mem::size_of::<usize> () * CHAR_BIT as usize / 2) != 0
      && nelem != 0 && size != new_length / nelem
    {
      /* Overflow.  Discard the old buffer, but it must remain valid
	 to free.  */
      scratch_buffer_free (buffer);
      scratch_buffer_init (buffer);
      __set_errno! (ENOMEM);
      return false;
    }

  if new_length <= (*buffer).length
    { return true; }

  /* Discard old buffer.  */
  scratch_buffer_free (buffer);

  let new_ptr: *mut c_char = malloc (new_length) as *mut c_char;
  if new_ptr.is_null ()
    {
      /* Buffer must remain valid to free.  */
      scratch_buffer_init (buffer);
      return false;
    }

  /* Install new heap-based buffer.  */
  (*buffer).data = new_ptr as *mut c_void;
  (*buffer).length = new_length;
  return true;

  }
}
/* C 64's `libc_hidden_def (__libc_scratch_buffer_set_array_size)`: empty in this build.  */
