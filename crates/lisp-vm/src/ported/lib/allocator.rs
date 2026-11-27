// femacs: port of lib/allocator.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::sys_common::*;
/* emacs/lib/allocator.c (@ 22 lines) is four lines of code in C: the `_GL_USE_STDLIB_ALLOC`
   marker, the allocator.h include, <stdlib.h>, and the one `struct allocator const
   stdlib_allocator = { malloc, realloc, free, NULL };` definition.  This is the unit
   docs/status.md 3.4 listed as blocked on the allocator.h mirror; that mirror is now in the
   tree (`lib/include/allocator.rs`), so the definition lands here.  C's `NULL` for
   `die` is Rust's `None`, and the three functions are the system entries of the stdlib mirror,
   which is what the C's `#define _GL_USE_STDLIB_ALLOC 1` selects.  */
use std::ffi::{c_char, c_double, c_int, c_long, c_ulong, c_void};
// [U1] include/gnulib_config.rs imported via common
// [U1] include/stdlib.rs imported via common
// [U1] include/allocator.rs imported via common

/* C 19's `#define _GL_USE_STDLIB_ALLOC 1`: it makes the system <stdlib.h> declare `malloc`
   unrenamed, which is what lets the three initializers below be the system functions.  */
pub const _GL_USE_STDLIB_ALLOC: c_int = 1;

unsafe extern "C-unwind" fn stdlib_allocate (size: usize) -> *mut c_void
{
  /* SAFETY: gnulib/OS raw buffer or dynamic-array boundary; caller upholds pointer validity, element capacity, and single-owner mutation invariants, returning error/null on overflow or OOM. */ unsafe { malloc (size) }
}

unsafe extern "C-unwind" fn stdlib_reallocate (ptr: *mut c_void, size: usize) -> *mut c_void
{
  /* SAFETY: gnulib/OS raw buffer or dynamic-array boundary; caller upholds pointer validity, element capacity, and single-owner mutation invariants, returning error/null on overflow or OOM. */ unsafe { realloc (ptr, size) }
}

unsafe extern "C-unwind" fn stdlib_free (ptr: *mut c_void)
{
  /* SAFETY: gnulib/OS raw buffer or dynamic-array boundary; caller upholds pointer validity, element capacity, and single-owner mutation invariants, returning error/null on overflow or OOM. */ unsafe { free (ptr) }
}

/* C 22's `struct allocator const stdlib_allocator = { malloc, realloc, free, NULL };`.  */
#[unsafe(no_mangle)]
pub static stdlib_allocator: allocator = allocator {
  allocate: Some (stdlib_allocate),
  reallocate: Some (stdlib_reallocate),
  free: Some (stdlib_free),
  die: None,
};
