// femacs: port of src/comp.h @ fd7afced403 (GNU Emacs 32.0.50).
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
