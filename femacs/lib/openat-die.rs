/* Respond to a save- or restore-cwd failure.
   This should never happen with Emacs.  */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::c_int;

include!("../src/include/config.rs");

/* C 4's `#include "openat.h"` declares the two functions below (and, per this file's own
   note, "should never happen with Emacs"); C 5's `#include <stdlib.h>` is the `abort`
   declaration, written here the way the other units declare it.  */
unsafe extern "C" { pub fn abort () -> !; }

/* C 6: `void openat_save_fail (int errnum) { abort (); }`.  `openat.h` is the system
   header's on this target, so the definitions are this file's own (the reference object
   exports `T _openat_save_fail`, tools/check-symbols.py).  `errnum` is unused in C too,
   which is why the file carries the unit's usual allow block.  */
#[unsafe(export_name = "openat_save_fail")]
pub unsafe extern "C" fn openat_save_fail (errnum: c_int)
{
  abort ();
}

/* C 7: `void openat_restore_fail (int errnum) { abort (); }`.  */
#[unsafe(export_name = "openat_restore_fail")]
pub unsafe extern "C" fn openat_restore_fail (errnum: c_int)
{
  abort ();
}
