// femacs: port of lib/malloc/dynarray_emplace_enlarge.c @ fd7afced403.
//
// The frozen configuration leaves `_LIBC` undefined, so only C 20's `<libc-config.h>` include
// is conditional; the body has no `#if`.  The port's `__set_errno` is that header's
// `#define __set_errno(val) (errno = (val))`, and C 25's `<stdckdint.h>` is the `ckd_mul`
// alias family (PORTING.md 2.6/P-15).  C 23's `#include <dynarray.h>` is the gnulib wrapper,
// which reaches the shared header with no `DYNARRAY_*` defined (its `#else` arm).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// [U1] config.rs provided by common
// [U1] ../include/gnulib_config.rs imported via common
// [U1] ../include/errno.rs imported via common
// [U1] ../include/stdlib.rs imported via common
// [U1] ../include/string.rs imported via common
// [U1] ../include/dynarray.rs imported via common

/* C 19-21's `#ifndef _LIBC` arm includes `<libc-config.h>`; its `__set_errno` is the macro
   below (the tempname.rs/strtol.rs shape).  */
macro_rules! __set_errno {
  ($val:expr) => { *errno () = ($val) };
}

#[unsafe(export_name = "_gl_dynarray_emplace_enlarge")]
pub unsafe extern "C" fn __libc_dynarray_emplace_enlarge (list: *mut dynarray_header,
                                                          scratch: *mut c_void,
                                                          element_size: usize) -> bool
{
  // SAFETY: gnulib/OS raw buffer or dynamic-array boundary; caller upholds pointer validity, element capacity, and single-owner mutation invariants, returning error/null on overflow or OOM.
  unsafe {
  let mut new_allocated: usize;
  if (*list).allocated == 0
    {
      /* No scratch buffer provided.  Choose a reasonable default
         size.  */
      if element_size < 4
        { new_allocated = 16; }
      else if element_size < 8
        { new_allocated = 8; }
      else
        { new_allocated = 4; }
    }
  else
    /* Increase the allocated size, using an exponential growth
       policy.  */
    {
      new_allocated = (*list).allocated + (*list).allocated / 2 + 1;
      if new_allocated <= (*list).allocated
        {
          /* Overflow.  */
          __set_errno! (ENOMEM);
          return false;
        }
    }

  /* C 59's `if (ckd_mul (&new_size, new_allocated, element_size)) return false;`: the
     overflowing_mul pair, whose destination C writes only when the multiplication did not
     overflow (PORTING.md 2.6/P-15).  */
  let (new_size, overflow): (usize, bool) = new_allocated.overflowing_mul (element_size);
  if overflow
    { return false; }
  let new_array: *mut c_void;
  if (*list).array == scratch
    {
      /* The previous array was not heap-allocated.  */
      new_array = malloc (new_size);
      if !new_array.is_null () && !(*list).array.is_null ()
        { memcpy (new_array, (*list).array, (*list).used * element_size); }
    }
  else
    { new_array = realloc ((*list).array, new_size); }
  if new_array.is_null ()
    { return false; }
  (*list).array = new_array;
  (*list).allocated = new_allocated;
  return true;

  }
}
