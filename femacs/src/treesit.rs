// femacs: port of src/treesit.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.
//
// `HAVE_TREE_SITTER` is undefined (docs/baseline/config/darwin-arm64.macros.tsv 1625), so all
// but 53 lines are dead text: C 29-5293's `#ifdef HAVE_TREE_SITTER` region (the
// `<tree_sitter/api.h>` and `<tree_sitter/parser.h>` includes, the parser/node/query records
// and their accessors, the cursor and predicate engines, `treesit_load_language` and the
// ninety-odd DEFUNs) and C 5314-5621's region inside `syms_of_treesit` (the `DEFSYM`s, the
// `Vtreesit_*` variables and the `defsubr`s).  They are recorded in docs/ports/treesit.md,
// not transcribed (P-08).
//
// The live text is C 22-27's five includes, `Ftreesit_available_p` (whose `#if HAVE_TREE_SITTER`
// arm is dead, leaving C 5303's `return Qnil;`) and `syms_of_treesit`'s one `defsubr`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

include!("include/config.rs");
include!("include/lisp.rs");

// C 24-27's `buffer.h`, `coding.h` and `treesit.h` are not pulled in: every name the live
// text could take from them (`struct buffer`, the coding-system accessors, `struct
// Lisp_TS_*`) sits inside the dead region (the atimer.rs precedent for an include the live
// text never names; docs/ports/treesit.md).

DEFUN! ("treesit-available-p", Ftreesit_available_p,
       Streesit_available_p, 0, 0, 0,
       doc: "Return non-nil if tree-sitter support is built-in and available.",
       (),
       {
  // C 5300-5304: `return load_tree_sitter_if_necessary (false) ? Qt : Qnil;` is the dead arm;
  // the live body is C 5303's `return Qnil;`.
  return Qnil ();
});

/*** Initialization  */

/* Initialize the tree-sitter routines.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_treesit ()
{
  // C 5314-5621's `#if HAVE_TREE_SITTER` region is dead text (docs/ports/treesit.md).
  defsubr (&raw mut Streesit_available_p);
  // C 5623-5631's `#ifdef WINDOWSNT` block (`Qtree_sitter__library_abi` and the
  // `tree-sitter--library-abi` value) is dead too: WINDOWSNT is undefined.
}
