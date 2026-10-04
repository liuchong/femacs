// femacs: port of src/comp.h @ fd7afced403.
//
// WIP mirror (P-66): `lisp.h` 2168 includes comp.h unconditionally, and the frozen
// configuration takes comp.h's `#else /* #ifdef HAVE_NATIVE_COMP */` branch, where the
// entry point data.c calls is a `static inline` no-op.  That branch is the whole of this
// file until comp.c is ported (batch G, docs/ports/include-comp.md).

// comp.h 22: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const COMP_H: bool = true;

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
/// Lisp_Native_Comp_Unit` - the field-carrying definition sits in the `#ifdef` branch the
/// frozen configuration does not compile (P-08), and the port's own copy lives in
/// `pdumper.rs` under the `native_comp` feature - so a pointer to a type this branch never
/// defines is spelled `*mut core::ffi::c_void` (PORTING.md P-72 3).
#[inline(always)]
pub fn unload_comp_unit (cu: *mut core::ffi::c_void) {
  let _ = cu;
}
