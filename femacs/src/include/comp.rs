// femacs: port of src/comp.h @ fd7afced403.
//
// WIP mirror (P-66): `lisp.h` 2168 includes comp.h unconditionally, and the frozen
// configuration takes comp.h's `#else /* #ifdef HAVE_NATIVE_COMP */` branch, where the
// entry point data.c calls is a `static inline` no-op.  That branch is the whole of this
// file until comp.c is ported (batch G, docs/ports/include-comp.md).

// comp.h 23: `#include <dynlib.h>` - the record below names `dynlib_handle_ptr`, so the mirror is
// included exactly where C includes it (PORTING.md 0.1-4).
include!("dynlib.rs");

// comp.h 22: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const COMP_H: bool = true;

/* comp.h 25-51: the record itself.  It sits **before** the `#ifdef HAVE_NATIVE_COMP` at C 52, so
   it is live text on every configuration - an earlier revision of this mirror said the
   field-carrying definition was inside the dead branch, which is not what the reference says
   (status 3.153, found by the missing-type check).  Fields, order and types are C's; the
   `GCALIGNED_STRUCT` spelling is `#[repr(C, align(8))]` (PORTING.md 1.3 and
   docs/ports/excluded-macros.txt).  */
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Native_Comp_Unit
{
  pub header: vectorlike_header,
  pub file: Lisp_Object,
  pub optimize_qualities: Lisp_Object,
  pub lambda_gc_guard_h: Lisp_Object,
  pub lambda_c_name_idx_h: Lisp_Object,
  pub data_fdoc_v: Lisp_Object,
  pub data_vec: Lisp_Object,
  pub data_relocs: *mut Lisp_Object,
  pub loaded_once: bool,
  pub load_ongoing: bool,
  pub handle: dynlib_handle_ptr,
}

/* comp.h 90-95, the live `#else` branch:
     static inline void
     maybe_defer_native_compilation (Lisp_Object function_name,
				     Lisp_Object definition)
     {}                                                                 */
/// The empty body is the reference's own (comp.h 92-95), not a port stub: with
/// HAVE_NATIVE_COMP undefined this is the definition every caller links against; when that
/// macro *is* defined comp.h declares the real one and `src/comp.c` defines it, which is the
/// condition the gate below reproduces.
/* comp.h 39-41/47-48: when `HAVE_NATIVE_COMP` is defined the two are *declarations* of
   definitions in `src/comp.c`.  They sit in a module with a glob import so the defining unit's own
   items shadow the glob (P-39, the `include/window.rs` 895 shape) while every other unit that
   includes this mirror still sees the names.  */
#[cfg (feature = "have_native_comp")]
pub mod comp_h_decls {
  use super::*;

  unsafe extern "C" {
    pub fn maybe_defer_native_compilation (function_name: Lisp_Object,
                                           definition: Lisp_Object);
    pub fn unload_comp_unit (cu: *mut Lisp_Native_Comp_Unit);
  }
}
#[cfg (feature = "have_native_comp")]
use comp_h_decls::*;

#[cfg (not (feature = "have_native_comp"))]
#[inline(always)]
pub fn maybe_defer_native_compilation (function_name: Lisp_Object, definition: Lisp_Object)
{
  let _ = (function_name, definition);
}

/* comp.h 96-98, the same live `#else` branch:
     static inline
     void unload_comp_unit (struct Lisp_Native_Comp_Unit *cu)
     {}                                                                 */
/// The empty body is the reference's own (comp.h 97-98), not a port stub: with
/// HAVE_NATIVE_COMP undefined this is what `alloc.c` 3195 links against when its own
/// `#ifdef HAVE_NATIVE_COMP` arm is taken.  C's parameter is the *incomplete* `struct
/// Lisp_Native_Comp_Unit` - that record is above (comp.h 25-51, live text), so the port has the
/// type; C only ever passes it through by pointer here, which is why this signature keeps
/// `*mut c_void` (PORTING.md P-72 3) and the type stays a declaration the caller may ignore.
///
/// **Both this and the `maybe_defer_native_compilation` above are C's `#else` arms** - the empty
/// `static inline` definitions the reference has when `HAVE_NATIVE_COMP` is undefined.  When that
/// macro *is* defined, comp.h 39-41/47-48 declare the real ones and `src/comp.c` defines them, so
/// the port gates the two exactly as C's preprocessor does; without the gate the unit's
/// definitions and these would collide in the one crate root the materialiser builds.
#[cfg (not (feature = "have_native_comp"))]
#[inline(always)]
pub fn unload_comp_unit (cu: *mut core::ffi::c_void) {
  let _ = cu;
}

/* comp.h 55-67: the two accessors, inside the `#ifdef HAVE_NATIVE_COMP` that starts at C 52.
   They reach every unit the way C reaches them: `lisp.h` 2168 includes `comp.h`, and
   `include/lisp.rs` includes this mirror at its own 280 (status 3.223).  Before this they lived
   as a local copy in `src/pdumper.rs`, which is where the dead-arms gate found the last unit
   that needed them without the declaration (`src/print.rs`, C 2042).  */
#[cfg (feature = "have_native_comp")]
#[inline(always)]
pub fn NATIVE_COMP_UNITP (a: Lisp_Object) -> bool {
  PSEUDOVECTORP (a, pvec_type::PVEC_NATIVE_COMP_UNIT)
}
#[cfg (feature = "have_native_comp")]
#[inline(always)]
pub fn XNATIVE_COMP_UNIT (a: Lisp_Object) -> *mut Lisp_Native_Comp_Unit {
  eassert! (NATIVE_COMP_UNITP (a));
  return XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Native_Comp_Unit);
}
