// femacs: port of src/pdumper.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): the `EMACS_PDUMPER_H` guard, the `PDUMPER_*` macros, the
// two records, the `pdumper_hook` typedef, the load-result enum and the inline predicates.
//
// The declarations live in `pdumper_decls` + glob import (the itree.rs / tempname.rs shape):
// `pdumper.c`'s port defines `pdumper_load`, `pdumper_record_wd`, `init_pdumper_once`,
// `syms_of_pdumper` and the four `*_impl` functions, and an explicit item shadows a glob
// import (PORTING.md 0.1-10).  The `INLINE` bodies are ordinary `#[inline(always)] pub fn`
// in this file, the way C's `INLINE` (P-17) gives every translation unit its own copy.
//
// `HAVE_PDUMPER` is 1 in the frozen configuration (docs/baseline/config/darwin-arm64.macros.tsv),
// so every `#ifdef HAVE_PDUMPER` arm is live and each `#else` arm is recorded in
// docs/ports/include-pdumper.md rather than transcribed.

// pdumper.h 20-21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_PDUMPER_H: bool = true;

include!("../../lib/include/stdio.rs");        // pdumper.h 23: `#include <stdio.h>` (`FILE`)
include!("../../lib/include/fingerprint.rs");  // pdumper.h 23: `#include "fingerprint.h"`
// pdumper.h 24's `#include "lisp.h"` reaches config.h (lisp.h 27), which the lisp.rs mirror
// reads for `HAVE_PDUMPER`/`HAVE_NATIVE_COMP` (PORTING.md 0.1-14: config.rs first).
include!("config.rs");
include!("lisp.rs");

pub const PDUMPER_NO_OBJECT: c_int = -1;       // pdumper.h 26

/* Indicate in source code that we're deliberately relying on pdumper
   not preserving the given value.  Compiles to nothing --- for humans
   only.  */
// pdumper.h 31: `((void) &(thing))`.  Rust has no `(void)` cast; the block takes the
// address and discards it, which is the same non-evaluation of `thing`'s value.
macro_rules! PDUMPER_IGNORE {
    ($thing:expr) => { { let _ = &$thing; } };
}

/* The portable dumper automatically preserves the Lisp heap and any C
   variables to which the Lisp heap points.  It doesn't know anything
   about other C variables.  The functions below allow code from other
   parts of Emacs to tell the portable dumper about other bits of
   information to preserve in dump files.

   These memory-records are themselves preserved in the dump, so call
   the functions below only on the !initialized init path, just
   like staticpro.

   There are no special functions to preserve a global Lisp_Object.
   You should just staticpro these.  */

/* Remember the value of THING in dump files.  THING must not
   contain any pointers or Lisp_Object variables: these values are not
   valid across dump and load.  */
// pdumper.h 46-47: `pdumper_remember_scalar (&(thing), sizeof (thing))`.  `sizeof` of a C
// expression is its *unevaluated* size, which `size_of_val` on the taken address gives;
// `addr_of_mut!` keeps C's "the argument is an lvalue" requirement (P-79).
macro_rules! PDUMPER_REMEMBER_SCALAR {
    ($thing:expr) => {
        pdumper_remember_scalar (core::ptr::addr_of_mut! ($thing) as *mut c_void,
                                 core::mem::size_of_val (&$thing) as ptrdiff_t)
    };
}

pub mod pdumper_decls {
  use super::*;

  unsafe extern "C" {
    pub fn dump_fingerprint (output: *mut FILE, label: *const c_char,
                             fingerp: *const c_uchar);              // pdumper.h 49-50
    pub fn pdumper_remember_scalar_impl (data: *mut c_void,
                                         nbytes: ptrdiff_t);        // pdumper.h 52
    pub fn pdumper_remember_lv_ptr_raw_impl (ptr: *mut c_void,
                                             type_: Lisp_Type);     // pdumper.h 62
    pub fn pdumper_do_now_and_after_load_impl (hook: pdumper_hook);        // pdumper.h 75
    pub fn pdumper_do_now_and_after_late_load_impl (hook: pdumper_hook);   // pdumper.h 76
    pub fn pdumper_load (dump_filename: *const c_char, argv0: *mut c_char)
      -> c_int;                                                     // pdumper.h 146
    pub static mut dump_public: pdumper_loaded_dump;                // pdumper.h 154
    pub fn pdumper_cold_object_p_impl (obj: *const c_void) -> bool;         // pdumper.h 168
    pub fn pdumper_find_object_type_impl (obj: *const c_void) -> c_int;     // pdumper.h 180
    pub fn pdumper_marked_p_impl (obj: *const c_void) -> bool;              // pdumper.h 213
    pub fn pdumper_set_marked_impl (obj: *const c_void);                    // pdumper.h 227
    pub fn pdumper_clear_marks_impl ();                                     // pdumper.h 238
    pub fn pdumper_record_wd (arg0: *const c_char);                         // pdumper.h 248
    pub fn init_pdumper_once ();                                            // pdumper.h 250
    pub fn syms_of_pdumper ();                                              // pdumper.h 251
  }
}

use pdumper_decls::*;

// pdumper.h 81: `typedef void (*pdumper_hook)(void);`.  The pointer is nullable in C, so
// the port spells it `Option<unsafe extern "C" fn ()>` (P-27, the atimer_callback shape).
pub type pdumper_hook = Option<unsafe extern "C" fn ()>;

// pdumper.h 92: `INLINE void pdumper_remember_scalar (void *data, ptrdiff_t nbytes)`.
#[inline(always)]
pub fn pdumper_remember_scalar (data: *mut c_void, nbytes: ptrdiff_t) {
  // HAVE_PDUMPER is 1 in the frozen configuration, so C's `#ifdef` arm is the live text.
  unsafe { pdumper_remember_scalar_impl (data, nbytes); }
}

/* Remember the pointer at *PTR.  *PTR must be null or point to a Lisp
   object.  TYPE is the rough type of Lisp object to which *PTR
   points.  */
#[inline(always)]
pub fn pdumper_remember_lv_ptr_raw (ptr: *mut c_void, type_: Lisp_Type) {
  unsafe { pdumper_remember_lv_ptr_raw_impl (ptr, type_); }
}

#[inline(always)]
pub fn pdumper_do_now_and_after_load (hook: pdumper_hook) {
  unsafe { pdumper_do_now_and_after_load_impl (hook); }
}

/* Same as 'pdumper_do_now_and_after_load' but for hooks running code
   that can call into Lisp.  */
#[inline(always)]
pub fn pdumper_do_now_and_after_late_load (hook: pdumper_hook) {
  unsafe { pdumper_do_now_and_after_late_load_impl (hook); }
}

/* Macros useful in pdumper callback functions.  Assign a value if
   we're loading a dump and the value needs to be reset to its
   original value, and if we're initializing for the first time,
   assert that the value has the expected original value.  */

// pdumper.h 113-119: `do { ... } while (0)`; `$variable` must stay an assignable place,
// which is why the fragment is used bare on the assignment's left (P-79).
macro_rules! PDUMPER_RESET {
    ($variable:expr, $value:expr) => {
        if dumped_with_pdumper_p () { $variable = $value; }
        else { eassert! (($variable) == ($value)); }
    };
}

macro_rules! PDUMPER_RESET_LV {
    ($variable:expr, $value:expr) => {
        if dumped_with_pdumper_p () { $variable = $value; }
        else { eassert! (EQ ($variable, $value)); }
    };
}

/* Actually load a dump.  */

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum pdumper_load_result
  {
    PDUMPER_LOAD_SUCCESS,
    PDUMPER_NOT_LOADED /* Not returned: useful for callers */,
    PDUMPER_LOAD_FILE_NOT_FOUND,
    PDUMPER_LOAD_BAD_FILE_TYPE,
    PDUMPER_LOAD_FAILED_DUMP,
    PDUMPER_LOAD_OOM,
    PDUMPER_LOAD_VERSION_MISMATCH,
    PDUMPER_LOAD_ERROR /* Must be last, as errno may be added.  */
  }

#[repr(C)]
pub struct pdumper_loaded_dump
{
  pub start: uintptr_t,
  pub end: uintptr_t,
}

/* Return whether the OBJ points somewhere into the loaded dump image.
   Works even when we have no dump loaded --- in this case, it just
   returns false.  */
#[inline(always)]
pub fn pdumper_object_p (obj: *const c_void) -> bool {
  let obj_addr = obj as uintptr_t;
  unsafe { dump_public.start <= obj_addr && obj_addr < dump_public.end }
}

/* Return whether the OBJ is in the cold section of the dump.
   Only bool-vectors and floats should end up there.
   pdumper_object_p() and pdumper_object_p_precise() must have
   returned true for OBJ before calling this function.  */
#[inline(always)]
pub fn pdumper_cold_object_p (obj: *const c_void) -> bool {
  unsafe { pdumper_cold_object_p_impl (obj) }
}

/* Return the type of the dumped object that starts at OBJ.  It is a
   programming error to call this routine for an OBJ for which
   pdumper_object_p would return false.  */
#[inline(always)]
pub fn pdumper_find_object_type (obj: *const c_void) -> c_int {
  unsafe { pdumper_find_object_type_impl (obj) }
}

/* Return true if TYPE is that of a Lisp object.
   PDUMPER_NO_OBJECT is invalid.  */
#[inline(always)]
pub fn pdumper_valid_object_type_p (type_: c_int) -> bool {
  0 <= type_
}

/* Return whether OBJ points exactly to the start of some object in
   the loaded dump image.  It is a programming error to call this
   routine for an OBJ for which pdumper_object_p would return
   false.  */
#[inline(always)]
pub fn pdumper_object_p_precise (obj: *const c_void) -> bool {
  pdumper_valid_object_type_p (pdumper_find_object_type (obj))
}

/* Return whether OBJ is marked according to the portable dumper.
   It is an error to call this routine for an OBJ for which
   pdumper_object_p_precise would return false.  */
#[inline(always)]
pub fn pdumper_marked_p (obj: *const c_void) -> bool {
  unsafe { pdumper_marked_p_impl (obj) }
}

/* Set the pdumper mark bit for OBJ.  It is a programming error to
   call this function with an OBJ for which pdumper_object_p_precise
   would return false.  */
#[inline(always)]
pub fn pdumper_set_marked (obj: *const c_void) {
  unsafe { pdumper_set_marked_impl (obj) }
}

/* Clear all the mark bits for pdumper objects.  */
#[inline(always)]
pub fn pdumper_clear_marks () {
  unsafe { pdumper_clear_marks_impl (); }
}
