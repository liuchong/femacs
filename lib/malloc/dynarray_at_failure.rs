// femacs: port of lib/malloc/dynarray_at_failure.c @ fd7afced403.
//
// The frozen configuration leaves `_LIBC` undefined, so C 30-38's arm (the `__snprintf`
// diagnostic plus `__libc_fatal`) is dead and C's `#else` (`abort ()`) is the live text.
// The file is not in `docs/baseline/config/cond-eval-verified.txt`, so the ledger's body
// check sees both arms: the dead one is kept as the non-frozen side's text under the feature
// that names it (P-51, the md5-stream.rs shape).  C 24's `#include <dynarray.h>` is the gnulib
// wrapper, which reaches the shared header with no `DYNARRAY_*` defined (its `#else` arm).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// [U1] config.rs provided by common
// [U1] ../include/gnulib_config.rs imported via common
// [U1] ../include/stdlib.rs imported via common
// [U1] ../include/dynarray.rs imported via common
/* C 25's <stdio.h> is only used by the dead `_LIBC` arm's `__snprintf`.  */

/* C 19-22's `#ifndef _LIBC` arm includes `<libc-config.h>` and `<stdlib.h>`; the port's
   configuration header is the former's value half.  `abort` is a system entry no mirror
   carries, so it is declared at its use site (P-26/P-72(3), the openat-die.rs shape).  */
unsafe extern "C" {
  /* void abort (void);  */
  pub fn abort () -> !;
}

/* C 27-39.  `_LIBC` is undefined, so the live reading is `abort ();`.  */
#[unsafe(export_name = "_gl_dynarray_at_failure")]
pub unsafe extern "C" fn __libc_dynarray_at_failure (size: usize, index: usize) -> !
{
  /* C 30-35's `#ifdef _LIBC` arm, kept as text even though nothing compiles it here.  */
  #[cfg(feature = "libc")]
  {
    unsafe extern "C" {
      /* int __snprintf (char *s, size_t maxlen, const char *format, ...);  */
      pub fn __snprintf (s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
      /* _Noreturn void __libc_fatal (const char *message);  */
      pub fn __libc_fatal (message: *const c_char) -> !;
    }
    let mut buf: [c_char; 200] = [0; 200];
    __snprintf (buf.as_mut_ptr (), std::mem::size_of_val (&buf),
                c"Fatal glibc error: array index %zu not less than array length %zu\n"
                  .as_ptr (),
                index, size);
    __libc_fatal (buf.as_ptr ());
  }
  /* C 36-38's `#else` arm.  */
  abort ();
}
