// femacs: port of lib/malloc/dynarray_resize_clear.c @ fd7afced403.
//
// The frozen configuration leaves `_LIBC` undefined, so only C 20's `<libc-config.h>` include
// is conditional; the body has no `#if`.  C 23's `#include <dynarray.h>` is the gnulib
// wrapper, which reaches the shared header with no `DYNARRAY_*` defined (its `#else` arm),
// and the `__libc_dynarray_resize` it declares is the renamed `_gl_dynarray_resize` symbol
// that lib/malloc/dynarray_resize.c defines.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
include!("../../src/include/config.rs");
include!("../include/gnulib_config.rs");
include!("../include/string.rs");    // C 24's <string.h>
include!("../include/dynarray.rs");  // C 23's <dynarray.h>

#[unsafe(export_name = "_gl_dynarray_resize_clear")]
pub unsafe extern "C" fn __libc_dynarray_resize_clear (list: *mut dynarray_header,
                                                       size: usize, scratch: *mut c_void,
                                                       element_size: usize) -> bool
{
  let old_size: usize = (*list).used;
  if !__libc_dynarray_resize (list, size, scratch, element_size)
    { return false; }
  /* __libc_dynarray_resize already checked for overflow.  */
  let array: *mut c_char = (*list).array as *mut c_char;
  memset (array.add (old_size * element_size) as *mut c_void, 0,
          (size - old_size) * element_size);
  return true;
}
