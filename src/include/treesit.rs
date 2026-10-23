// femacs: port of src/treesit.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  `HAVE_TREE_SITTER` is undefined in the frozen
// configuration (docs/baseline/config/darwin-arm64.macros.tsv 1656) and
// `tools/check-cond-eval.py src/treesit.h` agrees with clang on both of this header's
// conditions, so C 25-253's `#ifdef HAVE_TREE_SITTER` region is dead text (P-08): the three
// pseudo-vector record types (`struct Lisp_TS_Parser`/`Lisp_TS_Node`/`Lisp_TS_Query`, with
// their `<tree_sitter/api.h>` members `TSParser`/`TSTree`/`TSInput`/`TSNode`/`TSQuery`/
// `TSQueryCursor` and the `bool` flags C keeps after `timestamp`), the nine inline accessors
// and checkers (`TS_PARSERP`/`XTS_PARSER`, `TS_NODEP`/`XTS_NODE`,
// `TS_COMPILED_QUERY_P`/`XTS_COMPILED_QUERY`, `CHECK_TS_PARSER`/`CHECK_TS_NODE`/
// `CHECK_TS_COMPILED_QUERY`), the `TREESIT_EMPTY_LINECOL` object and the ten `extern`
// functions (`linecol_offset`, `treesit_linecol_maybe`, `treesit_record_change`,
// `treesit_node_uptodate_p`, `treesit_node_buffer_live_p`, `treesit_delete_parser`,
// `treesit_delete_query`, `treesit_named_node_p`, `treesit_node_eq`).  They are recorded in
// docs/ports/include-treesit.md, not transcribed; `src/treesit.c`'s own ledger
// (docs/ports/treesit.md) records the same region from the unit's side.
//
// What is left is the one declaration outside the guard, and it is live: C 147-149 of
// `src/emacs.c` includes this header **without** a guard, with the comment that `treesit.o`
// is always compiled so that `treesit-available-p` exists even when the library is not, and
// C 2347 calls this function unconditionally for the same reason.  The other units that
// name this header (`buffer.c` 51-52, `print.c` 50-51, `alloc.c` 59-60, `fns.c` 40-41,
// `editfns.c` 63-64, `casefiddle.c` 33-34, `insdel.c`) do it inside `#ifdef HAVE_TREE_SITTER`
// and so take nothing from it here.

/* The C 20-21 include guard carries its name the way the other mirrors do (PORTING.md 0.1-4:
   the guard itself has no Rust counterpart, the include graph's dedup does the same job).  */
pub const EMACS_TREESIT_H: bool = true;

/// C: treesit.h 255: `extern void syms_of_treesit (void);`
unsafe extern "C" { pub fn syms_of_treesit (); }
