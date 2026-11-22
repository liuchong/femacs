// femacs: port of lib/malloc/scratch_buffer_grow_preserve.c @ fd7afced403.
//
// C 19-21's `#ifndef _LIBC` arm is live: `<libc-config.h>`'s `__set_errno` (below) is taken,
// C 67's `libc_hidden_def (…)` stays empty, and C 23's `<scratch_buffer.h>` is included below.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_void};
// [U1] ../include/errno.rs imported via common
// [U1] ../include/string.rs imported via common
// [U1] ../include/scratch_buffer.rs imported via common
/* C 19-21's `<libc-config.h>`: `# define __set_errno(val) (errno = (val))` (C 46).  */
macro_rules! __set_errno { ($val:expr) => { *errno () = $val }; }
/* `<sys/cdefs.h>`'s branch hints (PORTING.md 2.6), as lib/regcomp.rs spells the pair.  */
fn __glibc_likely<T> (cond: T) -> T { cond }
fn __glibc_unlikely<T> (cond: T) -> T { cond }
/* The entries the frozen object references besides `rpl_free`: `nm` prints `U _malloc`,
   `U _memcpy`, `U _realloc`, none of them renamed by the generated headers.  */
unsafe extern "C" {
  pub fn malloc (size: usize) -> *mut c_void;
  #[cfg (not (feature = "replace_realloc_for_realloc_posix"))]
  pub fn realloc (ptr: *mut c_void, size: usize) -> *mut c_void;
  #[cfg (feature = "replace_realloc_for_realloc_posix")]
  #[link_name = "realloc"]
  fn realloc_system (ptr: *mut c_void, size: usize) -> *mut c_void;
}
/* C: this file includes gnulib's `<stdlib.h>`, so `realloc` here is the redirected
   `rpl_realloc` - `realloc (ptr, size ? size : 1)`, the note in
   `lib/include/stdlib.rs` (status 4.787).  */
#[cfg (feature = "replace_realloc_for_realloc_posix")]
unsafe extern "C" fn realloc (ptr: *mut c_void, size: usize) -> *mut c_void
{
  // SAFETY: gnulib/OS raw buffer or dynamic-array boundary; caller upholds pointer validity, element capacity, and single-owner mutation invariants, returning error/null on overflow or OOM.
  unsafe {
  return realloc_system (ptr, if size == 0 { 1 } else { size });

  }
}

/* Like __libc_scratch_buffer_grow, but preserve the old buffer
   contents on success, as a prefix of the new buffer.  */
/* bool __libc_scratch_buffer_grow_preserve (struct scratch_buffer *buffer) -- the frozen
   object carries C's real name (`T __gl_scratch_buffer_grow_preserve`, PORTING.md 10.13).  */
#[unsafe(export_name = "_gl_scratch_buffer_grow_preserve")]
pub unsafe extern "C" fn __libc_scratch_buffer_grow_preserve (buffer: *mut scratch_buffer) -> bool
{
  // SAFETY: gnulib/OS raw buffer or dynamic-array boundary; caller upholds pointer validity, element capacity, and single-owner mutation invariants, returning error/null on overflow or OOM.
  unsafe {
  let new_length: usize = 2 * (*buffer).length;
  let mut new_ptr: *mut c_void;

  if (*buffer).data == &mut (*buffer).__space.__c as *mut [c_char] as *mut c_void
    {
      /* Move buffer to the heap.  No overflow is possible because
	 buffer->length describes a small buffer on the stack.  */
      new_ptr = malloc (new_length);
      if new_ptr.is_null ()
	{ return false; }
      memcpy (new_ptr, &mut (*buffer).__space.__c as *mut [c_char] as *mut c_void,
              (*buffer).length);
    }
  else
    {
      /* Buffer was already on the heap.  Check for overflow.  */
      if __glibc_likely (new_length >= (*buffer).length)
	{ new_ptr = realloc ((*buffer).data, new_length); }
      else
	{
	  __set_errno! (ENOMEM);
	  new_ptr = std::ptr::null_mut ();
	}

      if __glibc_unlikely (new_ptr.is_null ())
	{
	  /* Deallocate, but buffer must remain valid to free.  */
	  rpl_free ((*buffer).data);
	  scratch_buffer_init (buffer);
	  return false;
	}
    }

  /* Install new heap-based buffer.  */
  (*buffer).data = new_ptr;
  (*buffer).length = new_length;
  return true;

  }
}
/* C 67's `libc_hidden_def (__libc_scratch_buffer_grow_preserve)`: empty in this build.  */
