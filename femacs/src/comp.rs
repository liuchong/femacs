// femacs: port of src/comp.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.
//
// `HAVE_NATIVE_COMP` is undefined (docs/baseline/config/darwin-arm64.macros.tsv 10), so all
// but 45 lines of this file are dead text: C 25-5578's `#ifdef HAVE_NATIVE_COMP` region (the
// `<dynlib.h>` include, the `comp` struct and its `XCOMP`/`COMP_UNIT` accessors, the Limple
// opcode and cast enums, the GIMPLE-to-Limple pass, `load_comp_unit` and the rest of the
// compiler interface) and C 5596-5854's `#ifdef HAVE_NATIVE_COMP` region inside `syms_of_comp`
// (the `native-comp-jit-compilation` DEFVAR_BOOL, the `comp--#\$` DEFVAR_LISP and the
// fifty-odd DEFSYMs).  They are recorded in docs/ports/comp.md, not transcribed (P-08).
//
// The live text is C 21-23's two includes, `Fnative_comp_available_p` (whose `#ifdef` arm is
// dead, leaving C 5588's `return Qnil;`) and `syms_of_comp`'s one `defsubr`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

include!("include/config.rs");
include!("include/lisp.rs");

DEFUN! ("native-comp-available-p", Fnative_comp_available_p,
       Snative_comp_available_p, 0, 0, 0,
       doc: "Return non-nil if native compilation support is built-in.",
       (),
       {
  // C 5586-5589: `return load_gccjit_if_necessary (false) ? Qt : Qnil;` is the dead arm; the
  // live body is C 5588's `return Qnil;`.
  return Qnil ();
});

// C 5766's `DEFVAR_LISP ("comp-eln-to-el-h", Vcomp_eln_to_el_h, ...)` is dead text on this
// target, but the storage it names is not: `globals.f_Vcomp_eln_to_el_h` is a field of the
// generated `struct emacs_globals`, which exists in every configuration, and the *live*
// `src/lread.c` 1019/1632 reads it.  The port's model gives a member its storage in the unit
// that owns it (status 3.29), so the storage stands here even though the registration does
// not.  The first real link found this name referenced by `lread.rs` and defined nowhere
// (docs/ports/link.md, reading three).
#[unsafe(no_mangle)]
pub static mut Vcomp_eln_to_el_h: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_comp ()
{
  // C 5596-5854's `#ifdef HAVE_NATIVE_COMP` region is dead text (docs/ports/comp.md).
  defsubr (&raw mut Snative_comp_available_p);
}
