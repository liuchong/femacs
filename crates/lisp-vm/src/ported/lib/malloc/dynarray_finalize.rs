// femacs: port of lib/malloc/dynarray_finalize.c @ fd7afced403.
//
// The frozen configuration leaves `_LIBC` undefined, so only C 20's `<libc-config.h>` include
// is conditional; the body has no `#if`.  C 23's `#include <dynarray.h>` is the gnulib
// wrapper, which reaches the shared header with no `DYNARRAY_*` defined (its `#else` arm) -
// that header's `__dynarray_error` inline is what C 32 calls.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::sys_common::*;

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr::null_mut;
// [U1] config.rs provided by common
// [U1] ../include/gnulib_config.rs imported via common
// [U1] ../include/stdlib.rs imported via common
// [U1] ../include/string.rs imported via common
// [U1] ../include/dynarray.rs imported via common

/* Internal function.  Copy the dynamically-allocated area to an
   explicitly-sized heap allocation.  SCRATCH is a pointer to the
   embedded scratch space.  ELEMENT_SIZE is the size, in bytes, of the
   element type.  On success, true is returned, and pointer and length
   are written to *RESULT.  On failure, false is returned.  The caller
   has to take care of some of the memory management; this function is
   expected to be called from dynarray-skeleton.c.  */

#[unsafe(export_name = "_gl_dynarray_finalize")]
pub unsafe extern "C" fn __libc_dynarray_finalize (list: *mut dynarray_header,
                                                   scratch: *mut c_void,
                                                   element_size: usize,
                                                   result: *mut dynarray_finalize_result)
  -> bool
{
  // SAFETY: gnulib/OS raw buffer or dynamic-array boundary; caller upholds pointer validity, element capacity, and single-owner mutation invariants, returning error/null on overflow or OOM.
  unsafe {
  if __dynarray_error (list)
    /* The caller will reported the deferred error.  */
    { return false; }

  let used: usize = (*list).used;

  /* Empty list.  */
  if used == 0
    {
      /* An empty list could still be backed by a heap-allocated
         array.  Free it if necessary.  */
      if (*list).array != scratch
        { free ((*list).array); }
      *result = dynarray_finalize_result { array: null_mut (), length: 0 };
      return true;
    }

  let allocation_size: usize = used * element_size;
  let heap_array: *mut c_void = malloc (allocation_size);
  if !heap_array.is_null ()
    {
      /* The new array takes ownership of the strings.  */
      if !(*list).array.is_null ()
        { memcpy (heap_array, (*list).array, allocation_size); }
      if (*list).array != scratch
        { free ((*list).array); }
      *result = dynarray_finalize_result { array: heap_array, length: used };
      return true;
    }
  else
    /* The caller will perform the freeing operation.  */
    { return false; }

  }
}
