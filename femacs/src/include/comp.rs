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
/// HAVE_NATIVE_COMP undefined this is the definition every caller links against.
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
#[inline(always)]
pub fn unload_comp_unit (cu: *mut core::ffi::c_void) {
  let _ = cu;
}
