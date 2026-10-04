// femacs: port of lib/malloc/dynarray_resize.c @ fd7afced403.
//
// The frozen configuration leaves `_LIBC` undefined, so only C 20's `<libc-config.h>` include
// is conditional; the body has no `#if`.  The port's `__set_errno` is that header's
// `#define __set_errno(val) (errno = (val))`, and C 25's `<stdckdint.h>` is the `ckd_mul`
// alias family (PORTING.md 2.6/P-15).  C 23's `#include <dynarray.h>` is the gnulib wrapper,
// which reaches the shared header with no `DYNARRAY_*` defined (its `#else` arm).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
include!("../../src/include/config.rs");
include!("../include/gnulib_config.rs");
include!("../include/errno.rs");     // C 24's <errno.h>
include!("../include/stdlib.rs");    // C 26's <stdlib.h>
include!("../include/string.rs");    // C 27's <string.h>
include!("../include/dynarray.rs");  // C 23's <dynarray.h>

/* C 19-21's `#ifndef _LIBC` arm includes `<libc-config.h>`; its `__set_errno` is the macro
   below (the tempname.rs/strtol.rs shape).  */
macro_rules! __set_errno {
  ($val:expr) => { *errno () = ($val) };
}

#[unsafe(export_name = "_gl_dynarray_resize")]
pub unsafe extern "C" fn __libc_dynarray_resize (list: *mut dynarray_header, size: usize,
                                                 scratch: *mut c_void,
                                                 element_size: usize) -> bool
{
  /* The existing allocation provides sufficient room.  */
  if size <= (*list).allocated
    {
      (*list).used = size;
      return true;
    }

  /* Otherwise, use size as the new allocation size.  The caller is
     expected to provide the final size of the array, so there is no
     over-allocation here.  */

  /* C 45's `if (ckd_mul (&new_size_bytes, size, element_size)) { /* Overflow.  */
     __set_errno (ENOMEM); return false; }`: the overflowing_mul pair plus C's errno arm
     (PORTING.md 2.6/P-15).  */
  let (new_size_bytes, overflow): (usize, bool) = size.overflowing_mul (element_size);
  if overflow
    {
      /* Overflow.  */
      __set_errno! (ENOMEM);
      return false;
    }
  let new_array: *mut c_void;
  if (*list).array == scratch
    {
      /* The previous array was not heap-allocated.  */
      new_array = malloc (new_size_bytes);
      if !new_array.is_null () && !(*list).array.is_null ()
        { memcpy (new_array, (*list).array, (*list).used * element_size); }
    }
  else
    { new_array = realloc ((*list).array, new_size_bytes); }
  if new_array.is_null ()
    { return false; }
  (*list).array = new_array;
  (*list).allocated = size;
  (*list).used = size;
  return true;
}
