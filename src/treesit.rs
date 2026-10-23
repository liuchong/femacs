// femacs: port of src/treesit.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow.  `HAVE_TREE_SITTER`
// is `undef` in the frozen configuration (docs/baseline/config/darwin-arm64.macros.tsv 1656), so C
// compiles nothing out of C 29-5293 and nothing out of C 5314-5621 on this target: this is a P-86
// unit, and **both arms of every `#if` are carried**, each item under the feature that names C's own
// macro:
//
//   * the file gate (`#if HAVE_TREE_SITTER`, C 29) is `feature = "have_tree_sitter"` (the name
//     `xtask/Cargo.toml` 161 registers for that macro), on every item *and on C 24-27's
//     include graph* - which is why the frozen configuration still compiles nothing from here;
//   * C 34-315's `#ifdef WINDOWSNT` arm is `feature = "windowsnt"` (nested inside the file gate),
//     the name `src/decompress.rs` and `src/vm-limit.rs` already use for that configuration;
//   * C 38-42, 101-105, 165-169, 186-190, 237-241, 252-256 and 303-307's
//     `TREE_SITTER_LANGUAGE_VERSION >= 15` and `HAVE_TS_TREE_CURSOR_GOTO_PREVIOUS_SIBLING` pairs are
//     `feature = "treesit_language_version_ge_15"` and
//     `feature = "have_ts_tree_cursor_goto_previous_sibling"` - **two names this tree has not
//     registered** (docs/ports/treesit.md `## Deviations`; the tree has no tree-sitter version gate
//     yet, and this unit does not add one to a shared manifest).
//
// This round transcribes C 29-946: the file gate, the Windows dynamic-loading arm, the file's own
// commentary, the constants, the language cache and `treesit_load_language`, ending at that
// function's closing brace.  C 948-5634 - the DEFUNs from `treesit-language-available-p` to
// `treesit-subtree-stat`, the linecol and query/predicate engines, and `syms_of_treesit`'s
// `#if HAVE_TREE_SITTER` region - is not transcribed this round and is recorded in
// docs/ports/treesit.md's C line-range table.  The text outside the gate (C 5295-5312's
// `Ftreesit_available_p` and `syms_of_treesit`) is unchanged below.
//
// Two names the transcribed text needs are not written anywhere in the port, and this unit does not
// invent them: `struct Lisp_TS_Parser` with its `XTS_PARSER` accessor sits inside the `src/treesit.h`
// mirror's own recorded (not transcribed) `#ifdef HAVE_TREE_SITTER` region
// (docs/ports/include-treesit.md, P-08), and the `TS*` types come from `<tree_sitter/api.h>`, a
// third-party header that has no file in the reference tree and no copy on this host - P-88's shape
// for such a header asks for a clang-measured layout, which cannot be taken here, so those
// declarations are a *named gap* (docs/ports/treesit.md `## Deviations`) rather than an invention.
// The frozen reading of this unit is unaffected either way (docs/ports/treesit.md `## Evidence`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]

include!("include/config.rs");                  // C 22: `#include <config.h>`
include!("include/lisp.rs");                    // C 23: `#include "lisp.h"`

/* C 24-27's `buffer.h`, `coding.h` and `treesit.h`.  C compiles the three includes in *every*
   configuration - they precede the file gate at C 29 - but each mirror carries only names the
   *gated* text below names, so the port hangs them on the same feature the gate uses (P-86 ⑧: put
   the gate on the include graph, or the frozen configuration starts compiling names it should not
   see; measured for the sibling unit in docs/ports/decompress.md ③).  */
#[cfg (feature = "have_tree_sitter")]
include!("include/buffer.rs");                  // C 24: `#include "buffer.h"`
#[cfg (feature = "have_tree_sitter")]
include!("include/coding.rs");                  // C 25: `#include "coding.h"`
/* C 27's `#include "treesit.h"` is *not* an `include!` edge here.  The mirror carries two names -
   its guard constant and `unsafe extern "C" { pub fn syms_of_treesit (); }` - and this unit
   defines `syms_of_treesit` below, which is `E0428` the moment both are in one crate (PORTING.md
   0.1-10: a declaration the defining unit also defines collides; measured - the flip reading in
   docs/ports/treesit.md `## Evidence` was 39 errors with one of them exactly this, until the edge
   was removed).  No name this slice's text needs comes from the mirror either: the records it
   would carry (`struct Lisp_TS_Parser`, `XTS_PARSER`) are inside its own recorded region
   (docs/ports/include-treesit.md, P-08), so they stay the named gap the file note describes.  */
/* `dynlib.h`'s five names (`dynlib_handle_ptr` and the four functions) are used by C 864-941
   below, and this snapshot's `treesit.c` carries no include that declares them - the region is
   never compiled in the frozen configuration, so C tolerates it, and a tree-sitter
   configuration would need `#include "dynlib.h"` here.  The port takes the names from the
   mirror that owns them (PORTING.md 0.1-3), recorded in docs/ports/treesit.md.  */
#[cfg (feature = "have_tree_sitter")]
include!("include/dynlib.rs");                  // treesit.c 864-941: the `dynlib_*` names

/* ----------------------------------------------------------------------------------------------
   C 29: `#if HAVE_TREE_SITTER` - the file gate.  Everything from C 31 to C 5293 is inside it, and
   so is C 5314-5621's region of `syms_of_treesit`; none of it is compiled on this target (P-51
   names the other side).  The gate's `#endif` is C 5293; the two DEFUNs it does not cover are
   below.  */

/* C 31-32: the banner the loader arm opens with.  */
/* Dynamic loading of libtree-sitter.  */

/* ----------------------------------------------------------------------------------------------
   C 34-315: `#ifdef WINDOWSNT`.  `WINDOWSNT` is `undef` here, so no line below is compiled on this
   target, and this machine has no Windows cross toolchain (no `*-w64-mingw32-gcc`), so this arm's
   evidence is `text-only` (P-86 clause 3 ⑵): there is no compile reading of it.  */

#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
include!("include/w32common.rs");               // C 35: `# include "w32common.h"`

/* C 37-99: C 37's own banner ("In alphabetical order.") and the run of `#undef`s that opens the arm - one per
   `ts_*` entry point the DLL loader re-declares below, with C 38-42's version pair choosing
   `ts_language_abi_version` or `ts_language_version` and C 88-92's
   `HAVE_TS_TREE_CURSOR_GOTO_PREVIOUS_SIBLING` pair choosing `ts_tree_cursor_goto_previous_sibling`
   or `ts_tree_cursor_copy`.  An `#undef` has no counterpart in Rust - there is no macro table to
   remove a name from, and the `ts_*` names these lines undef are only ever the `fn_ts_*` objects
   C 101-176 declares - so the run is recorded here rather than spelled out, the shape
   `src/decompress.rs` used for its four (docs/ports/treesit.md `## Deviations`).  */

/* C: w32common.h 77-79: `#define DEF_DLL_FN(type, func, ...)  typedef type (CDECL *W32_PFN_##func)`
   `__VA_ARGS__; static W32_PFN_##func fn_##func` - the macro `src/include/w32common.rs`
   keeps `deferred` (P-22).  This unit writes the expansion in long form, the shape
   `src/decompress.rs` established as its first consumer: C pastes `W32_PFN_##func` and `fn_##func`
   together, which Rust cannot do, so both generated names are parameters here and C's `__VA_ARGS__`
   (the parameter list, with any trailing attributes) is the `$args` fragment.  The two items the
   macro makes are C's, name for name.  */
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
macro_rules! DEF_DLL_FN {
  ($type:ty, $func:ident, $pfn:ident, $fnvar:ident, ($($arg:ty),* $(,)?)) => {
    pub type $pfn = Option<unsafe extern "C" fn ($($arg),*) -> $type>;
    static mut $fnvar: $pfn = None;
  };
}
/* C: w32common.h 82-89: `#define LOAD_DLL_FN(lib, func)       do { fn_##func = (W32_PFN_##func)`
   `get_proc_addr (lib, #func); if (!fn_##func) return false; } while (false)`.  `do { … } while
   (false)` is a block expression (PORTING.md 2.1); C's `#func` is the stringified name, which is
   also a parameter here for the same reason as above; the assignment's cast between two function
   pointer types is C's, and Rust has no `as` cast to an `Option<fn>` type, so it is the
   `transmute` that the equal widths make the same operation.  `get_proc_addr` is the mirror's own
   inline definition (`src/include/w32common.rs` 143-147).  */
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
macro_rules! LOAD_DLL_FN {
  ($lib:expr, $func:ident, $pfn:ident, $fnvar:ident, $name:expr) => {
    {
      $fnvar = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
      if $fnvar.is_none () { return false; }
    }
  };
}

/* C 101-176: the `DEF_DLL_FN` uses, each expanding to the two names C's macro pastes -
   `W32_PFN_<func>` and `fn_<func>`.  C's `CDECL` is the `__cdecl` attribute, empty on this
   target's ABI, so the pointer type is the plain `extern "C"` one; `uint32_t`/`int64_t` are
   PORTING.md 1.1's `u32`/`i64` and `bool` is C's `bool`.  C 101-105 and 165-169's pairs are the two
   version gates (see the file note).  */
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt",
            feature = "treesit_language_version_ge_15"))]
DEF_DLL_FN! (u32, ts_language_abi_version, W32_PFN_ts_language_abi_version,
             fn_ts_language_abi_version, (const TSLanguage *));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt",
            not (feature = "treesit_language_version_ge_15")))]
DEF_DLL_FN! (u32, ts_language_version, W32_PFN_ts_language_version,
             fn_ts_language_version, (const TSLanguage *));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_child, W32_PFN_ts_node_child, fn_ts_node_child, (TSNode, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_child_by_field_name, W32_PFN_ts_node_child_by_field_name,
             fn_ts_node_child_by_field_name, (TSNode, *const c_char, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_node_child_count, W32_PFN_ts_node_child_count, fn_ts_node_child_count,
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_descendant_for_byte_range,
             W32_PFN_ts_node_descendant_for_byte_range, fn_ts_node_descendant_for_byte_range,
             (TSNode, u32, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_node_end_byte, W32_PFN_ts_node_end_byte, fn_ts_node_end_byte, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_eq, W32_PFN_ts_node_eq, fn_ts_node_eq, (TSNode, TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, ts_node_field_name_for_child,
             W32_PFN_ts_node_field_name_for_child, fn_ts_node_field_name_for_child,
             (TSNode, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_has_error, W32_PFN_ts_node_has_error, fn_ts_node_has_error, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_is_extra, W32_PFN_ts_node_is_extra, fn_ts_node_is_extra, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_is_missing, W32_PFN_ts_node_is_missing, fn_ts_node_is_missing,
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_is_named, W32_PFN_ts_node_is_named, fn_ts_node_is_named, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_is_null, W32_PFN_ts_node_is_null, fn_ts_node_is_null, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_named_child, W32_PFN_ts_node_named_child, fn_ts_node_named_child,
             (TSNode, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_node_named_child_count, W32_PFN_ts_node_named_child_count,
             fn_ts_node_named_child_count, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_named_descendant_for_byte_range,
             W32_PFN_ts_node_named_descendant_for_byte_range,
             fn_ts_node_named_descendant_for_byte_range, (TSNode, u32, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_next_named_sibling, W32_PFN_ts_node_next_named_sibling,
             fn_ts_node_next_named_sibling, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_next_sibling, W32_PFN_ts_node_next_sibling, fn_ts_node_next_sibling,
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_prev_named_sibling, W32_PFN_ts_node_prev_named_sibling,
             fn_ts_node_prev_named_sibling, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_prev_sibling, W32_PFN_ts_node_prev_sibling, fn_ts_node_prev_sibling,
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_node_start_byte, W32_PFN_ts_node_start_byte, fn_ts_node_start_byte, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut c_char, ts_node_string, W32_PFN_ts_node_string, fn_ts_node_string, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, ts_node_type, W32_PFN_ts_node_type, fn_ts_node_type, (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_parser_delete, W32_PFN_ts_parser_delete, fn_ts_parser_delete, (*mut TSParser));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const TSRange, ts_parser_included_ranges, W32_PFN_ts_parser_included_ranges,
             fn_ts_parser_included_ranges, (*const TSParser, *mut u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const TSLanguage, ts_parser_language, W32_PFN_ts_parser_language,
             fn_ts_parser_language, (*const TSParser));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSParser, ts_parser_new, W32_PFN_ts_parser_new, fn_ts_parser_new, ());
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSTree, ts_parser_parse, W32_PFN_ts_parser_parse, fn_ts_parser_parse,
             (*mut TSParser, *const TSTree, TSInput));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_parser_set_included_ranges, W32_PFN_ts_parser_set_included_ranges,
             fn_ts_parser_set_included_ranges, (*mut TSParser, *const TSRange, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_parser_set_language, W32_PFN_ts_parser_set_language,
             fn_ts_parser_set_language, (*mut TSParser, *const TSLanguage));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, ts_query_capture_name_for_id,
             W32_PFN_ts_query_capture_name_for_id, fn_ts_query_capture_name_for_id,
             (*const TSQuery, u32, *mut u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_query_cursor_delete, W32_PFN_ts_query_cursor_delete,
             fn_ts_query_cursor_delete, (*mut TSQueryCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_query_cursor_exec, W32_PFN_ts_query_cursor_exec, fn_ts_query_cursor_exec,
             (*mut TSQueryCursor, *const TSQuery, TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSQueryCursor, ts_query_cursor_new, W32_PFN_ts_query_cursor_new,
             fn_ts_query_cursor_new, ());
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_query_cursor_next_match, W32_PFN_ts_query_cursor_next_match,
             fn_ts_query_cursor_next_match, (*mut TSQueryCursor, *mut TSQueryMatch));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_query_cursor_set_byte_range, W32_PFN_ts_query_cursor_set_byte_range,
             fn_ts_query_cursor_set_byte_range, (*mut TSQueryCursor, u32, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_query_delete, W32_PFN_ts_query_delete, fn_ts_query_delete, (*mut TSQuery));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSQuery, ts_query_new, W32_PFN_ts_query_new, fn_ts_query_new,
             (*const TSLanguage, *const c_char, u32, *mut u32, *mut TSQueryError));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_query_pattern_count, W32_PFN_ts_query_pattern_count,
             fn_ts_query_pattern_count, (*const TSQuery));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const TSQueryPredicateStep, ts_query_predicates_for_pattern,
             W32_PFN_ts_query_predicates_for_pattern, fn_ts_query_predicates_for_pattern,
             (*const TSQuery, u32, *mut u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, ts_query_string_value_for_id,
             W32_PFN_ts_query_string_value_for_id, fn_ts_query_string_value_for_id,
             (*const TSQuery, u32, *mut u32));
/* C 158-159: C's four allocator parameters are the four function-pointer types below, in that
   order (C writes `void *(*)(size_t)`, `void *(*)(size_t, size_t)`, `void *(*)(void *, size_t)`
   and `void (*)(void *)`).  */
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_set_allocator, W32_PFN_ts_set_allocator, fn_ts_set_allocator,
             (Option<unsafe extern "C" fn (size_t) -> *mut c_void>,
              Option<unsafe extern "C" fn (size_t, size_t) -> *mut c_void>,
              Option<unsafe extern "C" fn (*mut c_void, size_t) -> *mut c_void>,
              Option<unsafe extern "C" fn (*mut c_void)>));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_tree_cursor_current_node, W32_PFN_ts_tree_cursor_current_node,
             fn_ts_tree_cursor_current_node, (*const TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_tree_cursor_delete, W32_PFN_ts_tree_cursor_delete, fn_ts_tree_cursor_delete,
             (*const TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_tree_cursor_goto_first_child, W32_PFN_ts_tree_cursor_goto_first_child,
             fn_ts_tree_cursor_goto_first_child, (*mut TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (i64, ts_tree_cursor_goto_first_child_for_byte,
             W32_PFN_ts_tree_cursor_goto_first_child_for_byte,
             fn_ts_tree_cursor_goto_first_child_for_byte, (*mut TSTreeCursor, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_tree_cursor_goto_next_sibling, W32_PFN_ts_tree_cursor_goto_next_sibling,
             fn_ts_tree_cursor_goto_next_sibling, (*mut TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt",
            feature = "have_ts_tree_cursor_goto_previous_sibling"))]
DEF_DLL_FN! (bool, ts_tree_cursor_goto_previous_sibling,
             W32_PFN_ts_tree_cursor_goto_previous_sibling, fn_ts_tree_cursor_goto_previous_sibling,
             (*mut TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt",
            not (feature = "have_ts_tree_cursor_goto_previous_sibling")))]
DEF_DLL_FN! (TSTreeCursor, ts_tree_cursor_copy, W32_PFN_ts_tree_cursor_copy, fn_ts_tree_cursor_copy,
             (*const TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_tree_cursor_goto_parent, W32_PFN_ts_tree_cursor_goto_parent,
             fn_ts_tree_cursor_goto_parent, (*mut TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSTreeCursor, ts_tree_cursor_new, W32_PFN_ts_tree_cursor_new, fn_ts_tree_cursor_new,
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_tree_delete, W32_PFN_ts_tree_delete, fn_ts_tree_delete, (*mut TSTree));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_tree_edit, W32_PFN_ts_tree_edit, fn_ts_tree_edit,
             (*mut TSTree, *const TSInputEdit));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSRange, ts_tree_get_changed_ranges, W32_PFN_ts_tree_get_changed_ranges,
             fn_ts_tree_get_changed_ranges, (*const TSTree, *const TSTree, *mut u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_tree_root_node, W32_PFN_ts_tree_root_node, fn_ts_tree_root_node,
             (*const TSTree));

/* C 178-250: `init_treesit_functions`.  `HMODULE` is the platform's module handle, a pointer at
   the ABI, so `library` is `*mut c_void` (`src/include/w32common.rs` 137 spells it that way
   for `hbfont_init_w32_funcs`), and `w32_delayed_load` is the declaration
   `src/include/w32.rs` carries for `w32.h` 197.  The `LOAD_DLL_FN` uses below are C's, one
   per `DEF_DLL_FN` above, each passing the stringified name C's macro pastes.  */
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
unsafe fn init_treesit_functions () -> bool
{
  let library: *mut c_void = w32_delayed_load (Qtree_sitter ());

  if library.is_null ()
    { return false; }

  #[cfg (feature = "treesit_language_version_ge_15")]
  LOAD_DLL_FN! (library, ts_language_abi_version, W32_PFN_ts_language_abi_version,
                fn_ts_language_abi_version, c"ts_language_abi_version");
  #[cfg (not (feature = "treesit_language_version_ge_15"))]
  LOAD_DLL_FN! (library, ts_language_version, W32_PFN_ts_language_version,
                fn_ts_language_version, c"ts_language_version");
  LOAD_DLL_FN! (library, ts_node_child, W32_PFN_ts_node_child, fn_ts_node_child,
                c"ts_node_child");
  LOAD_DLL_FN! (library, ts_node_child_by_field_name, W32_PFN_ts_node_child_by_field_name,
                fn_ts_node_child_by_field_name, c"ts_node_child_by_field_name");
  LOAD_DLL_FN! (library, ts_node_child_count, W32_PFN_ts_node_child_count,
                fn_ts_node_child_count, c"ts_node_child_count");
  LOAD_DLL_FN! (library, ts_node_descendant_for_byte_range,
                W32_PFN_ts_node_descendant_for_byte_range,
                fn_ts_node_descendant_for_byte_range, c"ts_node_descendant_for_byte_range");
  LOAD_DLL_FN! (library, ts_node_end_byte, W32_PFN_ts_node_end_byte, fn_ts_node_end_byte,
                c"ts_node_end_byte");
  LOAD_DLL_FN! (library, ts_node_eq, W32_PFN_ts_node_eq, fn_ts_node_eq, c"ts_node_eq");
  LOAD_DLL_FN! (library, ts_node_field_name_for_child,
                W32_PFN_ts_node_field_name_for_child, fn_ts_node_field_name_for_child,
                c"ts_node_field_name_for_child");
  LOAD_DLL_FN! (library, ts_node_has_error, W32_PFN_ts_node_has_error, fn_ts_node_has_error,
                c"ts_node_has_error");
  LOAD_DLL_FN! (library, ts_node_is_extra, W32_PFN_ts_node_is_extra, fn_ts_node_is_extra,
                c"ts_node_is_extra");
  LOAD_DLL_FN! (library, ts_node_is_missing, W32_PFN_ts_node_is_missing, fn_ts_node_is_missing,
                c"ts_node_is_missing");
  LOAD_DLL_FN! (library, ts_node_is_named, W32_PFN_ts_node_is_named, fn_ts_node_is_named,
                c"ts_node_is_named");
  LOAD_DLL_FN! (library, ts_node_is_null, W32_PFN_ts_node_is_null, fn_ts_node_is_null,
                c"ts_node_is_null");
  LOAD_DLL_FN! (library, ts_node_named_child, W32_PFN_ts_node_named_child,
                fn_ts_node_named_child, c"ts_node_named_child");
  LOAD_DLL_FN! (library, ts_node_named_child_count, W32_PFN_ts_node_named_child_count,
                fn_ts_node_named_child_count, c"ts_node_named_child_count");
  LOAD_DLL_FN! (library, ts_node_named_descendant_for_byte_range,
                W32_PFN_ts_node_named_descendant_for_byte_range,
                fn_ts_node_named_descendant_for_byte_range,
                c"ts_node_named_descendant_for_byte_range");
  LOAD_DLL_FN! (library, ts_node_next_named_sibling, W32_PFN_ts_node_next_named_sibling,
                fn_ts_node_next_named_sibling, c"ts_node_next_named_sibling");
  LOAD_DLL_FN! (library, ts_node_next_sibling, W32_PFN_ts_node_next_sibling,
                fn_ts_node_next_sibling, c"ts_node_next_sibling");
  LOAD_DLL_FN! (library, ts_node_prev_named_sibling, W32_PFN_ts_node_prev_named_sibling,
                fn_ts_node_prev_named_sibling, c"ts_node_prev_named_sibling");
  LOAD_DLL_FN! (library, ts_node_prev_sibling, W32_PFN_ts_node_prev_sibling,
                fn_ts_node_prev_sibling, c"ts_node_prev_sibling");
  LOAD_DLL_FN! (library, ts_node_start_byte, W32_PFN_ts_node_start_byte, fn_ts_node_start_byte,
                c"ts_node_start_byte");
  LOAD_DLL_FN! (library, ts_node_string, W32_PFN_ts_node_string, fn_ts_node_string,
                c"ts_node_string");
  LOAD_DLL_FN! (library, ts_node_type, W32_PFN_ts_node_type, fn_ts_node_type, c"ts_node_type");
  LOAD_DLL_FN! (library, ts_parser_delete, W32_PFN_ts_parser_delete, fn_ts_parser_delete,
                c"ts_parser_delete");
  LOAD_DLL_FN! (library, ts_parser_included_ranges, W32_PFN_ts_parser_included_ranges,
                fn_ts_parser_included_ranges, c"ts_parser_included_ranges");
  LOAD_DLL_FN! (library, ts_parser_language, W32_PFN_ts_parser_language, fn_ts_parser_language,
                c"ts_parser_language");
  LOAD_DLL_FN! (library, ts_parser_new, W32_PFN_ts_parser_new, fn_ts_parser_new, c"ts_parser_new");
  LOAD_DLL_FN! (library, ts_parser_parse, W32_PFN_ts_parser_parse, fn_ts_parser_parse,
                c"ts_parser_parse");
  LOAD_DLL_FN! (library, ts_parser_set_included_ranges,
                W32_PFN_ts_parser_set_included_ranges, fn_ts_parser_set_included_ranges,
                c"ts_parser_set_included_ranges");
  LOAD_DLL_FN! (library, ts_parser_set_language, W32_PFN_ts_parser_set_language,
                fn_ts_parser_set_language, c"ts_parser_set_language");
  LOAD_DLL_FN! (library, ts_query_capture_name_for_id,
                W32_PFN_ts_query_capture_name_for_id, fn_ts_query_capture_name_for_id,
                c"ts_query_capture_name_for_id");
  LOAD_DLL_FN! (library, ts_query_cursor_delete, W32_PFN_ts_query_cursor_delete,
                fn_ts_query_cursor_delete, c"ts_query_cursor_delete");
  LOAD_DLL_FN! (library, ts_query_cursor_exec, W32_PFN_ts_query_cursor_exec,
                fn_ts_query_cursor_exec, c"ts_query_cursor_exec");
  LOAD_DLL_FN! (library, ts_query_cursor_new, W32_PFN_ts_query_cursor_new, fn_ts_query_cursor_new,
                c"ts_query_cursor_new");
  LOAD_DLL_FN! (library, ts_query_cursor_next_match, W32_PFN_ts_query_cursor_next_match,
                fn_ts_query_cursor_next_match, c"ts_query_cursor_next_match");
  LOAD_DLL_FN! (library, ts_query_cursor_set_byte_range,
                W32_PFN_ts_query_cursor_set_byte_range, fn_ts_query_cursor_set_byte_range,
                c"ts_query_cursor_set_byte_range");
  LOAD_DLL_FN! (library, ts_query_delete, W32_PFN_ts_query_delete, fn_ts_query_delete,
                c"ts_query_delete");
  LOAD_DLL_FN! (library, ts_query_new, W32_PFN_ts_query_new, fn_ts_query_new, c"ts_query_new");
  LOAD_DLL_FN! (library, ts_query_pattern_count, W32_PFN_ts_query_pattern_count,
                fn_ts_query_pattern_count, c"ts_query_pattern_count");
  LOAD_DLL_FN! (library, ts_query_predicates_for_pattern,
                W32_PFN_ts_query_predicates_for_pattern, fn_ts_query_predicates_for_pattern,
                c"ts_query_predicates_for_pattern");
  LOAD_DLL_FN! (library, ts_query_string_value_for_id,
                W32_PFN_ts_query_string_value_for_id, fn_ts_query_string_value_for_id,
                c"ts_query_string_value_for_id");
  LOAD_DLL_FN! (library, ts_set_allocator, W32_PFN_ts_set_allocator, fn_ts_set_allocator,
                c"ts_set_allocator");
  LOAD_DLL_FN! (library, ts_tree_cursor_current_node, W32_PFN_ts_tree_cursor_current_node,
                fn_ts_tree_cursor_current_node, c"ts_tree_cursor_current_node");
  LOAD_DLL_FN! (library, ts_tree_cursor_delete, W32_PFN_ts_tree_cursor_delete,
                fn_ts_tree_cursor_delete, c"ts_tree_cursor_delete");
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_first_child,
                W32_PFN_ts_tree_cursor_goto_first_child, fn_ts_tree_cursor_goto_first_child,
                c"ts_tree_cursor_goto_first_child");
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_first_child_for_byte,
                W32_PFN_ts_tree_cursor_goto_first_child_for_byte,
                fn_ts_tree_cursor_goto_first_child_for_byte,
                c"ts_tree_cursor_goto_first_child_for_byte");
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_next_sibling,
                W32_PFN_ts_tree_cursor_goto_next_sibling, fn_ts_tree_cursor_goto_next_sibling,
                c"ts_tree_cursor_goto_next_sibling");
  #[cfg (feature = "have_ts_tree_cursor_goto_previous_sibling")]
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_previous_sibling,
                W32_PFN_ts_tree_cursor_goto_previous_sibling,
                fn_ts_tree_cursor_goto_previous_sibling,
                c"ts_tree_cursor_goto_previous_sibling");
  #[cfg (not (feature = "have_ts_tree_cursor_goto_previous_sibling"))]
  LOAD_DLL_FN! (library, ts_tree_cursor_copy, W32_PFN_ts_tree_cursor_copy,
                fn_ts_tree_cursor_copy, c"ts_tree_cursor_copy");
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_parent, W32_PFN_ts_tree_cursor_goto_parent,
                fn_ts_tree_cursor_goto_parent, c"ts_tree_cursor_goto_parent");
  LOAD_DLL_FN! (library, ts_tree_cursor_new, W32_PFN_ts_tree_cursor_new, fn_ts_tree_cursor_new,
                c"ts_tree_cursor_new");
  LOAD_DLL_FN! (library, ts_tree_delete, W32_PFN_ts_tree_delete, fn_ts_tree_delete,
                c"ts_tree_delete");
  LOAD_DLL_FN! (library, ts_tree_edit, W32_PFN_ts_tree_edit, fn_ts_tree_edit, c"ts_tree_edit");
  LOAD_DLL_FN! (library, ts_tree_get_changed_ranges, W32_PFN_ts_tree_get_changed_ranges,
                fn_ts_tree_get_changed_ranges, c"ts_tree_get_changed_ranges");
  LOAD_DLL_FN! (library, ts_tree_root_node, W32_PFN_ts_tree_root_node, fn_ts_tree_root_node,
                c"ts_tree_root_node");

  return true;
}

/* C 252-313: the `#define` run that closes the arm - one `#define ts_<func> fn_ts_<func>` per
   entry point above, with C 252-256's version pair and C 303-307's
   `HAVE_TS_TREE_CURSOR_GOTO_PREVIOUS_SIBLING` pair.  An object-like `#define` has no counterpart
   in Rust (it renames an identifier), so the arms spell the call sites the way C's preprocessor
   leaves them - `fn_ts_<func> (...)` under this gate, `ts_<func> (...)` otherwise, the rule
   `src/decompress.rs` records for the same construct - and the run itself is recorded here
   (docs/ports/treesit.md `## Deviations`).  */

/* C 315: the arm's own `#endif` closes WINDOWSNT.  */

/* ----------------------------------------------------------------------------------------------
   C 317-486: the file's own commentary ("The Emacs wrapper of tree-sitter does not expose
   everything the C API provides …"), carried as text.  */

/* Commentary

   The Emacs wrapper of tree-sitter does not expose everything the C
   API provides, most notably:

   - It doesn't expose a syntax tree.  The syntax tree is part of the
     parser object, and updating the tree is handled on the C level.

   - It doesn't expose the tree cursor, either.  Presumably, Lisp is
     slow enough to make insignificant any performance advantages from
     using the cursor.  Not exposing the cursor also minimizes the
     number of new types this adds to Emacs Lisp; currently, this adds
     only the parser, node, and compiled query types.

   - Because updating the change is handled on the C level as each
     change is made in the buffer, there is no way for Lisp to update
     a node.  But since we can just retrieve a new node, it shouldn't
     be a limitation.

   - I didn't expose setting timeout and cancellation flag for a
     parser, mainly because I don't think they are really necessary
     in Emacs's use cases.

   - Many tree-sitter functions take a TSPoint, which is basically a
     line and column.  Emacs uses a gap buffer and does not keep
     information about the line and column positions in a buffer, so
     it's hard for us to pass it to tree-sitter.  Instead we just give
     it dummy values.  But there are certain languages that does need
     the line and column positions to work right, like Haskell.  So we
     added optional line and column tracking.  See the linecol section
     below.

   treesit.h has some commentary on the two main data structure for
   the parser and node.  treesit_sync_visible_region has some
   commentary on how we make tree-sitter play well with narrowing (the
   tree-sitter parser only sees the visible region, so we need to
   translate positions back and forth).  Most action happens in
   treesit_ensure_parsed, treesit_read_buffer and
   treesit_record_change.

   A complete correspondence list between tree-sitter functions and
   exposed Lisp functions can be found in the manual node (elisp)API
   Correspondence.

   Placement of CHECK_xxx functions: call CHECK_xxx before using any
   unchecked Lisp values; these include arguments of Lisp functions,
   the return value of Fsymbol_value, and that of Fcar or Fcdr on
   user-specified conses.

   Initializing tree-sitter: there are two entry points to tree-sitter
   functions: 'treesit-parser-create' and
   'treesit-language-available-p'.  Technically we only need to call
   initialization function in those two functions, but in reality we
   check at the beginning of every Lisp function.  That should be more
   fool-proof.

   Tree-sitter offset (0-based) and buffer position (1-based):
     tree-sitter offset + buffer position = buffer position
     buffer position - buffer position = tree-sitter offset

   Tree-sitter-related code in other files:
   - src/alloc.c for gc for parser and node
   - src/casefiddle.c, src/insdel.c, src/editfns.c for notifying
     tree-sitter parser of buffer changes.
   - lisp/emacs-lisp/cl-preloaded.el & data.c & lisp.h for parser and
     node type.
   - print.c for printing tree-sitter objects (node, parser, query).

   Regarding signals: only raise signals in Lisp functions.

   Casts from EMACS_INT and ptrdiff_t to uint32_t: We install checks
   for buffer size and range and thus able to assume these casts never
   overflow.

   We don't parse at every keystroke.  Instead we only record the
   changes at each keystroke, and only parse when requested.  It is
   possible that lazy parsing is worse: instead of dispersed little
   pauses, now you have less frequent but larger pauses.  I doubt
   there will be any perceived difference, as the lazy parsing is
   going to be pretty frequent anyway.  Also this (lazy parsing) is
   what the mailing list guys wanted.

   Because it is pretty slow (comparing to other tree-sitter
   operations) for tree-sitter to parse the query and produce a query
   object, it is very wasteful to reparse the query every time
   treesit-query-capture is called, and it completely kills the
   performance of querying in a loop for a moderate amount of times
   (hundreds of queries takes seconds rather than milliseconds to
   complete).  Therefore we want some caching.  We can either use a
   search.c style transparent caching, or simply expose a new type,
   compiled-ts-query and let the user to manually compile AOT.  I
   believe AOT compiling gives users more control, makes the
   performance stable and easy to understand (compiled -> fast,
   uncompiled -> slow), and avoids some edge cases transparent cache
   could have (see below).  So I implemented the AOT compilation.

   Problems a transparent cache could have: Suppose we store cache
   entries in a fixed-length linked-list, and compare with EQ.  1)
   One-off query could kick out useful cache.  2) if the user messed
   up and the query doesn't EQ to the cache anymore, the performance
   mysteriously drops.  3) what if a user uses so many stuff that the
   default cache size (20) is not enough and we end up thrashing?
   These are all imaginary scenarios but they are not impossible
   :-)

   Parsers in indirect buffers: We make indirect buffers share the
   parser list of their base buffer.  Indirect buffers and their base
   buffer share the same buffer content but not other buffer attributes.
   If they had separate parser lists, changes made in an indirect
   buffer would only update parsers of that indirect buffer, and not
   parsers in the base buffer or other indirect buffers, and vice versa.
   For that reason, the base buffer and all of its indirect buffers
   share a single parser list.  But each parser in this shared parser
   list still points to their own buffer.  On top of that,
   treesit-parser-list only returns parsers that belong to the calling
   buffer.  So ultimately, from the user's POV, each buffer, regardless
   of whether indirect or not, appears to have its own parser list.  A
   discussion can be found in bug#59693.  Note that the discussion led
   to an earlier design, which is different from the current one.

   Line and column reporting to tree-sitter: technically we had to send
   tree-sitter the line and column position of each edit.  But in
   practice we just send it dummy values, because tree-sitter doesn't
   use it for parsing and mostly just carries the line and column
   positions around and return it when e.g. reporting node positions[1].
   This has been working fine until we encountered grammars that
   actually utilizes the line and column information for parsing
   (Haskell)[2].

   [1] https://github.com/tree-sitter/tree-sitter/issues/445
   [2] https://github.com/tree-sitter/tree-sitter/issues/4001

   So now we have to keep track of line and column positions and pass
   valid values to tree-sitter.  (It adds quite some complexity, but
   only linearly; one can ignore all the linecol stuff when trying to
   understand treesit code and then come back to it later.)  Eli
   convinced me to disable tracking by default, and only enable it for
   languages that needs it.  So the buffer starts out not tracking
   linecol.  And when a parser is created, if the language is in
   treesit-languages-require-line-column-tracking, we enable tracking in
   the buffer, and enable tracking for the parser.  To simplify things,
   once a buffer starts tracking linecol, it never disables tracking,
   even if parsers that need tracking are all deleted; and for parsers,
   tracking is determined at creation time, if it starts out
   tracking/non-tracking, it stays that way, regardless of later changes
   to treesit-languages-require-line-column-tracking.

   To make calculating line/column positions fast, we store linecol
   caches for begv, point, and zv in the buffer
   (buf->ts_linecol_cache_xxx); and in the parser object, we store
   linecol cache for visible beg/end of that parser.

   In buffer editing functions, we need the linecol for
   start/old_end/new_end, those can be calculated by scanning newlines
   (treesit_linecol_of_pos) from the buffer point cache, which should be
   always near the point.  And we usually set the calculated linecol of
   new_end back to the buffer point cache.

   We also need to calculate linecol for the visible_beg/end for each
   parser, and linecol for the buffer's begv/zv, these positions are
   usually far from point, so we have caches for all of them (in either
   the parser object or the buffer).  These positions are far from
   point, so it's inefficient to scan newlines from point to there to
   get up-to-date linecol for them; but in the same time, because
   they're far and outside the changed region, we can calculate their
   change in line and column number by simply counting how much newlines
   are added/removed in the changed region
   (compute_new_linecol_by_change).  */

/* C 488: the Constants banner.  */

/* A linecol_cache that points to BOB, this is always valid.  */
#[cfg (feature = "have_tree_sitter")]
static TREESIT_BOB_LINECOL: ts_linecol = ts_linecol { bytepos: 1, line: 1, col: 0 };
/* An uninitialized linecol.  */
#[cfg (feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub static TREESIT_EMPTY_LINECOL: ts_linecol = ts_linecol { bytepos: 0, line: 0, col: 0 };
/* C 494's `static TSPoint const TREESIT_TS_POINT_1_0 = { 1, 0 };` is a `TSPoint`, i.e. a structure
   of `<tree_sitter/api.h>`, whose fields C names `row` and `column` at C 1237-1245 (and whose
   declaration is the named gap the file note records).  */
#[cfg (feature = "have_tree_sitter")]
static TREESIT_TS_POINT_1_0: TSPoint = TSPoint { row: 1, column: 0 };

/* C 498: the Initialization banner.  */

/* C 500-520: the `static Lisp_Object Vtreesit_str_*` objects, C's tentative definitions (zeroed;
   the port's `lisp_h_Qnil ()` shape, PORTING.md 1.5).  C 502-504's `#ifndef WINDOWSNT` guards the
   one of them only the Posix arm builds.  */
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_libtree_sitter: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_tree_sitter: Lisp_Object = lisp_h_Qnil ();
#[cfg (all (feature = "have_tree_sitter", not (feature = "windowsnt")))]
static mut Vtreesit_str_dot_0: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_dot: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_question_mark: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_star: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_plus: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_pound_eq_question_mark: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_pound_match_question_mark: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_pound_pred_question_mark: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_open_bracket: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_close_bracket: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_open_paren: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_close_paren: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_space: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_eq_question_mark: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_match_question_mark: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_pred_question_mark: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_tree_sitter")]
static mut Vtreesit_str_empty: Lisp_Object = lisp_h_Qnil ();

/* This is the limit on recursion levels for some tree-sitter
   functions.  Remember to update docstrings when changing this value.

   If we think of programs and AST, it is very rare for any program to
   have a very deep AST.  For example, you would need 1000+ levels of
   nested if-statements, or a struct somehow nested for 1000+ levels.
   It's hard for me to imagine any hand-written or machine generated
   program to be like that.  So I think 1000 is already generous.  If
   we look at xdisp.c, its AST only have 30 levels.  */
#[cfg (feature = "have_tree_sitter")]
const TREESIT_RECURSION_LIMIT: c_int = 1000;

#[cfg (feature = "have_tree_sitter")]
static mut treesit_initialized: bool = false;

/* C 535-561: `load_tree_sitter_if_necessary`.  C's two function-local `static bool`s are the
   block-scoped `static mut`s below, and the `#else` arm (C 559-560) is the non-Windows one.  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn load_tree_sitter_if_necessary (required: bool) -> bool
{
  #[cfg (feature = "windowsnt")]
  {
    static mut tried_to_initialize_once: bool = false;
    static mut tree_sitter_initialized: bool = false;

    if !tried_to_initialize_once
      {
        let status: Lisp_Object;

        tried_to_initialize_once = true;
        tree_sitter_initialized = init_treesit_functions ();
        status = if tree_sitter_initialized { Qt () } else { Qnil () };
        Vlibrary_cache = Fcons (Fcons (Qtree_sitter (), status), Vlibrary_cache);
      }

    if required && !tree_sitter_initialized
      { xsignal1 (Qtreesit_error (),
		  build_string (c"tree-sitter library not found or failed to load".as_ptr ())); }

    return tree_sitter_initialized;
  }
  #[cfg (not (feature = "windowsnt"))]
  {
    return true;
  }
}

/* C 563-574: `treesit_initialize`.  `ts_set_allocator`'s four arguments are C's four allocator
   names; the `windowsnt` arm calls the loaded pointer instead (the `#define` run above).  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_initialize ()
{
  if !treesit_initialized
    {
      load_tree_sitter_if_necessary (true);
      #[cfg (not (feature = "windowsnt"))]
      { ts_set_allocator (Some (xmalloc), Some (xcalloc), Some (xrealloc), Some (xfree)); }
      #[cfg (feature = "windowsnt")]
      { (fn_ts_set_allocator.unwrap ()) (Some (xmalloc), Some (xcalloc), Some (xrealloc), Some (xfree)); }
      treesit_initialized = true;
    }
}

/* C 576: `void treesit_debug_print_parser_list (char *, Lisp_Object) EXTERNALLY_VISIBLE;` - C's own
   forward declaration of the definition below (PORTING.md 0.1-11: the prototype/definition pair
   collapses to the one definition).  */

/* C 578-621: `treesit_debug_print_parser_list`.  The three `%td` arguments are `ptrdiff_t`s, C's
   `char *` arguments are `*mut c_char`, and C 606-614's block is the range printer with C 611-613's
   commented-out `eassert`s kept as C wrote them.  C's `printf` reaches this unit through
   `lisp.h`'s `<stdio.h>` (the `alloc.rs` 63 shape).  */
#[cfg (feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn treesit_debug_print_parser_list (msg: *mut c_char, parser: Lisp_Object)
{
  let buf: *mut buffer = XBUFFER ((*XTS_PARSER (parser)).buffer);
  let buf_name: *mut c_char = SSDATA (BVAR! (buf, name_));
  printf (c"%s (%s) [%s] <%s>: %td(%td)-(%td)%td {\n".as_ptr (),
	  if msg.is_null () { c"".as_ptr () } else { msg },
	  SSDATA (SYMBOL_NAME (Vthis_command)),
	  SSDATA (SYMBOL_NAME ((*XTS_PARSER (parser)).language_symbol)),
	  buf_name, BUF_BEG (buf),
	  BUF_BEGV (buf), BUF_ZV (buf), BUF_Z! (buf));
  let mut tail: Lisp_Object = BVAR! (buf, ts_parser_list_);

  FOR_EACH_TAIL! (tail, {
      let parser: *mut Lisp_TS_Parser = XTS_PARSER (XCAR (tail));
      printf (c"[%s %s %s %td-%td T:%td]\n".as_ptr (),
	      SSDATA (SYMBOL_NAME ((*parser).language_symbol)),
	      SSDATA (SYMBOL_NAME ((*parser).tag)),
	      if (*parser).need_reparse { c"NEED-R".as_ptr () } else { c"NONEED".as_ptr () },
	      (*parser).visible_beg, (*parser).visible_end,
	      (*parser).timestamp);
      /* Print ranges. */
      let mut len: u32 = 0;
      #[cfg (not (feature = "windowsnt"))]
      let ranges: *const TSRange = ts_parser_included_ranges ((*parser).parser, &raw mut len);
      #[cfg (feature = "windowsnt")]
      let ranges: *const TSRange
	= (fn_ts_parser_included_ranges.unwrap ()) ((*parser).parser, &raw mut len);

      /* C's `-1` in this `uint32_t` comparison is the all-ones value; Rust spells the conversion
	 `u32::MAX` (a bare `-1` is not even a `u32` expression).  */
      if ! (len == 1 && (*ranges).start_byte == 0 && (*ranges).end_byte == u32::MAX)
	{
	  let mut idx: c_int = 0;
	  while (idx as u32) < len
	    {
	      let range: TSRange = *ranges.offset (idx as isize);
	      /* C 609's `%"PRIu32"` pair concatenates to the two `%u`s below; `PRIu32` is `c"u"` in
		 the frozen `<inttypes.h>` (`lib/include/inttypes.in.rs` 201).  */
	      printf (c" [%u, %u)".as_ptr (), range.start_byte, range.end_byte);

	      /* if (!parser->need_reparse) */
	      /* 	{ */
	      /* 	  eassert (BUF_BEGV_BYTE (buf) <= range.start_byte + parser->visible_beg); */
	      /* 	  eassert (range.end_byte + parser->visible_beg <= BUF_ZV_BYTE (buf)); */
	      /* 	} */
	      idx += 1;
	    }
	  printf (c"\n".as_ptr ());
	}
  });
  printf (c"}\n\n".as_ptr ());
}

/* C 624: the Loading language library banner.  */

/* C 626-633: `struct treesit_loaded_lang`.  Both members are pointers, and the two `const _`s below
   are the measured layout (`clang` probe in docs/ports/treesit.md `## Evidence`: size 16, align 8,
   `lang` at 0, `filename` at 8).  */
#[cfg (feature = "have_tree_sitter")]
#[repr(C)]
pub struct treesit_loaded_lang
{
  /* The language object, or NULL if the language failed to load.  */
  pub lang: *mut TSLanguage,
  /* The absolute file name of the shared library, or NULL if access
     failed.  */
  pub filename: *const c_char,
}
#[cfg (feature = "have_tree_sitter")]
const _: () = core::assert! (std::mem::size_of::<treesit_loaded_lang> () == 16);
#[cfg (feature = "have_tree_sitter")]
const _: () = core::assert! (std::mem::offset_of! (treesit_loaded_lang, lang) == 0);
#[cfg (feature = "have_tree_sitter")]
const _: () = core::assert! (std::mem::offset_of! (treesit_loaded_lang, filename) == 8);

/* C 635-649: `struct treesit_lang_cache_entry` (measured: size 24, align 8, `symbol` at 0, `lang` at
   8 - `Lisp_Object` is the port's word-sized tagged pointer, `lisp.rs` 82-83).  */
#[cfg (feature = "have_tree_sitter")]
#[repr(C)]
pub struct treesit_lang_cache_entry
{
  /* Symbol for the cached language, never gc'ed.  */
  pub symbol: Lisp_Object,
  /* The cached language object.  */
  pub lang: treesit_loaded_lang,
}
#[cfg (feature = "have_tree_sitter")]
const _: () = core::assert! (std::mem::size_of::<treesit_lang_cache_entry> () == 24);
#[cfg (feature = "have_tree_sitter")]
const _: () = core::assert! (std::mem::offset_of! (treesit_lang_cache_entry, symbol) == 0);
#[cfg (feature = "have_tree_sitter")]
const _: () = core::assert! (std::mem::offset_of! (treesit_lang_cache_entry, lang) == 8);

/* C 651-656: the three cache objects.  */
/* The cache, a monotonically growing array on the heap.  */
#[cfg (feature = "have_tree_sitter")]
static mut treesit_lang_cache: *mut treesit_lang_cache_entry = std::ptr::null_mut ();
/* Number of available slots in the allocated space.  */
#[cfg (feature = "have_tree_sitter")]
static mut treesit_lang_cache_size: ptrdiff_t = 0;
/* Number of existing cache entry in the array.  */
#[cfg (feature = "have_tree_sitter")]
static mut treesit_lang_cache_used: ptrdiff_t = 0;

/* Return the cached language object for LANGUAGE_SYMBOL, or {NULL,
   NULL} if there is none.  */
/* C 661-668: the `for` loop is the port's `while` (PORTING.md 0.3's shape for C's index loops).  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_lang_cache_get (language_symbol: Lisp_Object) -> treesit_loaded_lang
{
  let mut i: ptrdiff_t = 0;
  while i < treesit_lang_cache_used
    {
      if EQ ((*treesit_lang_cache.offset (i as isize)).symbol, language_symbol)
	{ return (*treesit_lang_cache.offset (i as isize)).lang; }
      i += 1;
    }
  return treesit_loaded_lang { lang: std::ptr::null_mut (), filename: std::ptr::null () };
}

/* Cache LOADED_LANG keyed by LANGUAGE_SYMBOL.  */
/* C 670-685: `xpalloc`'s size argument is C's `sizeof *treesit_lang_cache`, which the port asks for
   by pointee type (`lisp.rs`'s `pointee_size` note, PORTING.md 4.x).  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_lang_cache_put (language_symbol: Lisp_Object,
				  loaded_lang: treesit_loaded_lang)
{
  if treesit_lang_cache_used == treesit_lang_cache_size
    {
      treesit_lang_cache
	= xpalloc (treesit_lang_cache as *mut c_void, &raw mut treesit_lang_cache_size, 1, -1,
		   std::mem::size_of::<treesit_lang_cache_entry> () as ptrdiff_t) as *mut treesit_lang_cache_entry;
    }
  (*treesit_lang_cache.offset (treesit_lang_cache_used as isize)).symbol = language_symbol;
  (*treesit_lang_cache.offset (treesit_lang_cache_used as isize)).lang = loaded_lang;
  treesit_lang_cache_used += 1;
}

/* Translate a symbol treesit-<lang> to a C name treesit_<lang>.  */
/* C 687-698: C's comparison is `int idx < size_t len`, i.e. the int is converted to `size_t`; the
   port writes that conversion.  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_symbol_to_c_name (symbol_name: *mut c_char)
{
  let len: size_t = strlen (symbol_name);
  let mut idx: c_int = 0;
  while (idx as size_t) < len
    {
      if *symbol_name.offset (idx as isize) == b'-' as c_char
	{ *symbol_name.offset (idx as isize) = b'_' as c_char; }
      idx += 1;
    }
}

/* Resolve language symbol LANG according to
   treesit-language-remap-alist.  */
/* C 700-708: `resolve_language_symbol`.  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn resolve_language_symbol (lang: Lisp_Object) -> Lisp_Object
{
  let res: Lisp_Object = Fassoc (lang, Vtreesit_language_remap_alist, Qeq ());
  if NILP (res)
    { return lang; }
  return Fcdr (res);
}

/* Find the override name for LANGUAGE_SYMBOL in
   treesit-load-name-override-list.  Set NAME and C_SYMBOL to the
   override name, and return true if there exists one, otherwise
   return false.

   This function may signal if treesit-load-name-override-list is
   malformed.  */
/* C 710-756: `treesit_find_override_name`.  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_find_override_name (language_symbol: Lisp_Object, name: *mut Lisp_Object,
				      c_symbol: *mut Lisp_Object) -> bool
{
  CHECK_LIST (Vtreesit_load_name_override_list);
  let mut tail: Lisp_Object = Vtreesit_load_name_override_list;

  FOR_EACH_TAIL! (tail, {
      let entry: Lisp_Object = XCAR (tail);
      CHECK_LIST (entry);
      let lang: Lisp_Object = XCAR (entry);
      CHECK_SYMBOL (lang);

      if EQ (lang, language_symbol)
	{
	  *name = Fnth (make_fixnum (1), entry);
	  CHECK_STRING (*name);
	  *c_symbol = Fnth (make_fixnum (2), entry);
	  CHECK_STRING (*c_symbol);

	  return true;
	}
  });

  CHECK_LIST_END (tail, Vtreesit_load_name_override_list);

  return false;
}

/* For example, if Vdynamic_library_suffixes is (".so", ".dylib"),
   this function pushes "lib_base_name.so" and "lib_base_name.dylib"
   into *path_candidates.  Obviously path_candidates should be a Lisp
   list of Lisp strings.  */
/* C 758-791: `treesit_load_language_push_for_each_suffix`.  C's version loop's bounds are
   `<tree_sitter/api.h>`'s two version macros, which this host has no copy of (the named gap the
   file note records); C 786's `ext` is a local C leaves uninitialized, which the port zeroes
   (PORTING.md 1.5).  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_load_language_push_for_each_suffix (lib_base_name: Lisp_Object,
						      path_candidates: *mut Lisp_Object)
{
  let mut suffixes: Lisp_Object;

  suffixes = Vdynamic_library_suffixes;

  FOR_EACH_TAIL! (suffixes, {
      let candidate1: Lisp_Object = concat2 (lib_base_name, XCAR (suffixes));
      #[cfg (not (feature = "windowsnt"))]
      {
	/* On Posix hosts, support libraries named with ABI version
	   numbers.  Originally tree-sitter grammars are always versioned
	   at 0.0, so we first try that.  For more details, see
	   https://lists.gnu.org/archive/html/emacs-devel/2023-04/msg00386.html.  */
	let candidate2: Lisp_Object = concat2 (candidate1, Vtreesit_str_dot_0);
	let candidate3: Lisp_Object = concat2 (candidate2, Vtreesit_str_dot_0);

	*path_candidates = Fcons (candidate3, *path_candidates);
	*path_candidates = Fcons (candidate2, *path_candidates);

	/* Since 2025, tree-sitter grammars use their supported
	   TREE_SITTER_LANGUAGE_VERSION as the major version.  So we need
	   to try all the version supported by the tree-sitter library
	   too.  (See bug#78754)  */
	let mut version: c_int = TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION;
	while version <= TREE_SITTER_LANGUAGE_VERSION
	  {
	    let mut ext: [c_char; 16] = std::mem::zeroed (); // 16 should be enough until the end of universe.
	    snprintf (&raw mut ext as *mut c_char, 16, c".%d.0".as_ptr (), version);
	    let versioned_candidate: Lisp_Object = concat2 (candidate1,
							    build_string (&raw mut ext as *mut c_char));
	    *path_candidates = Fcons (versioned_candidate, *path_candidates);
	    version += 1;
	  }
      }
      *path_candidates = Fcons (candidate1, *path_candidates);
  });
}

/* This function is a compatibility shim.  Tree-sitter 0.25 introduced
   ts_language_abi_version as a replacement for ts_language_version, and
   tree-sitter 0.26 removed ts_language_version.  Here we use the fact
   that 0.25 bumped TREE_SITTER_LANGUAGE_VERSION to 15, to use the new
   function instead of the old one, when Emacs is compiled against
   tree-sitter version 0.25 or newer.  */
/* C 799-805: the two arms call the `ts_*` entry point the way the `#define` run leaves the name on
   each arm; the non-Windows one is `<tree_sitter/api.h>`'s own declaration (the named gap).  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_language_abi_version (ts_lang: *const TSLanguage) -> u32
{
  #[cfg (all (feature = "treesit_language_version_ge_15", not (feature = "windowsnt")))]
  { return ts_language_abi_version (ts_lang); }
  #[cfg (all (feature = "treesit_language_version_ge_15", feature = "windowsnt"))]
  { return (fn_ts_language_abi_version.unwrap ()) (ts_lang); }
  #[cfg (all (not (feature = "treesit_language_version_ge_15"), not (feature = "windowsnt")))]
  { return ts_language_version (ts_lang); }
  #[cfg (all (not (feature = "treesit_language_version_ge_15"), feature = "windowsnt"))]
  { return (fn_ts_language_version.unwrap ()) (ts_lang); }
}

/* Load the dynamic library of LANGUAGE_SYMBOL and return the pointer
   to the language definition.

   If error occurs, return NULL and fill SIGNAL_SYMBOL and SIGNAL_DATA
   with values suitable for xsignal.  */
/* C 806-946: `treesit_load_language`, the last definition of this slice.  The four `ts_*` call
   sites in its body (C 893-895's `ts_parser_new`/`ts_parser_set_language`/`ts_parser_delete` and
   C 900's `treesit_language_abi_version`, which is the unit's own shim) are spelled the way the
   `#define` run leaves each name on each arm (the rule `src/decompress.rs` records); C 901's
   `langfn = dynlib_sym (...)` converts `void *` to a function pointer, which is the `transmute`
   the two equal-width pointer types make the same operation, and C 941's `(void (*)) langfn` is
   the same conversion in the other direction.  `dynlib_*` reaches C through no include in this
   snapshot (a finding docs/ports/treesit.md records): this unit takes them from the mirror that
   owns them.  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_load_language (language_symbol: Lisp_Object, signal_symbol: *mut Lisp_Object,
				 signal_data: *mut Lisp_Object) -> treesit_loaded_lang
{
  let cached: treesit_loaded_lang = treesit_lang_cache_get (language_symbol);
  if !cached.lang.is_null ()
    { return cached; }

  let symbol_name: Lisp_Object = Fsymbol_name (language_symbol);

  CHECK_LIST (Vtreesit_extra_load_path);

  /* Figure out the library name and C name.  */
  let mut lib_base_name: Lisp_Object = concat2 (Vtreesit_str_libtree_sitter, symbol_name);
  let base_name: Lisp_Object = concat2 (Vtreesit_str_tree_sitter, symbol_name);

  /* Override the library name and C name, if appropriate.  */
  let mut override_name: Lisp_Object = Qnil ();
  let mut override_c_name: Lisp_Object = Qnil ();
  let found_override: bool = treesit_find_override_name (language_symbol,
							 &raw mut override_name,
							 &raw mut override_c_name);
  if found_override
    { lib_base_name = override_name; }

  /* Now we generate a list of possible library paths.  */
  let mut path_candidates: Lisp_Object = Qnil ();
  /* First push just the filenames to the candidate list, which will
     make dynlib_open look under standard system load paths.  */
  treesit_load_language_push_for_each_suffix (lib_base_name, &raw mut path_candidates);
  /* Then push ~/.emacs.d/tree-sitter paths.  */
  let lib_name: Lisp_Object
    = Fexpand_file_name (concat2 (build_string (c"tree-sitter/".as_ptr ()), lib_base_name),
			 Fsymbol_value (Quser_emacs_directory ()));
  treesit_load_language_push_for_each_suffix (lib_name, &raw mut path_candidates);
  /* Then push paths from treesit-extra-load-path.  */
  let mut tail: Lisp_Object;

  tail = Freverse (Vtreesit_extra_load_path);

  FOR_EACH_TAIL! (tail, {
      let expanded_lib: Lisp_Object = Fexpand_file_name (lib_base_name, XCAR (tail));
      treesit_load_language_push_for_each_suffix (expanded_lib, &raw mut path_candidates);
  });

  /* Try loading the dynamic library by each path candidate.  Stop
     when succeed, record the error message and try the next one when
     fail.  */
  let mut handle: dynlib_handle_ptr = std::ptr::null_mut ();
  let mut error: *const c_char = std::ptr::null ();
  let mut error_list: Lisp_Object = Qnil ();
  let mut loaded_lang: treesit_loaded_lang = treesit_loaded_lang { lang: std::ptr::null_mut (),
								   filename: std::ptr::null () };

  tail = path_candidates;
  error = std::ptr::null ();
  handle = std::ptr::null_mut ();

  let mut loaded_lib: Lisp_Object = Qnil ();
  FOR_EACH_TAIL! (tail, {
      let library_name: *mut c_char = SSDATA (XCAR (tail));
      dynlib_error ();
      handle = dynlib_open (library_name);
      error = dynlib_error ();
      if error.is_null ()
	{
	  loaded_lib = XCAR (tail);
	  break;
	}
      else
	{ error_list = Fcons (build_string (error), error_list); }
  });

  if !error.is_null ()
    {
      /* Yes, the error message list gets a bit verbose, but those
	 messages will be helpful for certain errors like libc version
	 mismatch.  */
      *signal_symbol = Qtreesit_load_language_error ();
      *signal_data = Fcons (Qnot_found (), Fnreverse (error_list));
      return loaded_lang;
    }

  /* Load TSLanguage.  */
  eassume! (!handle.is_null ());
  dynlib_error ();
  let mut langfn: Option<unsafe extern "C" fn () -> *mut TSLanguage> = None;
  let mut c_name: *mut c_char;
  if found_override
    { c_name = xstrdup (SSDATA (override_c_name) as *const c_char); }
  else
    {
      c_name = xstrdup (SSDATA (base_name) as *const c_char);
      treesit_symbol_to_c_name (c_name);
    }
  langfn = std::mem::transmute::<*mut c_void, Option<unsafe extern "C" fn () -> *mut TSLanguage>>
	     (dynlib_sym (handle, c_name));
  xfree (c_name as *mut c_void);
  error = dynlib_error ();
  if !error.is_null ()
    {
      *signal_symbol = Qtreesit_load_language_error ();
      *signal_data = list2 (Qsymbol_error (), build_string (error));
      return loaded_lang;
    }
  let lang: *mut TSLanguage = (langfn.unwrap ()) ();

  /* Check if language version matches tree-sitter version.  */
  #[cfg (not (feature = "windowsnt"))]
  let parser: *mut TSParser = ts_parser_new ();
  #[cfg (feature = "windowsnt")]
  let parser: *mut TSParser = (fn_ts_parser_new.unwrap ()) ();
  #[cfg (not (feature = "windowsnt"))]
  let success: bool = ts_parser_set_language (parser, lang);
  #[cfg (feature = "windowsnt")]
  let success: bool = (fn_ts_parser_set_language.unwrap ()) (parser, lang);
  #[cfg (not (feature = "windowsnt"))]
  { ts_parser_delete (parser); }
  #[cfg (feature = "windowsnt")]
  { (fn_ts_parser_delete.unwrap ()) (parser); }
  if !success
    {
      let fmt: Lisp_Object
	= build_string (c"%s's ABI version is %d, but supported versions are %d-%d".as_ptr ());
      let formatted_msg: Lisp_Object
	= CALLN! (Fformat_message, fmt, loaded_lib,
		  make_fixnum (treesit_language_abi_version (lang) as EMACS_INT),
		  make_fixnum (TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION),
		  make_fixnum (TREE_SITTER_LANGUAGE_VERSION));
      *signal_symbol = Qtreesit_load_language_error ();
      *signal_data = list2 (Qlang_version_mismatch (), formatted_msg);
      return loaded_lang;
    }

  let mut sym: *const c_char = std::ptr::null ();
  dynlib_addr (std::mem::transmute::<Option<unsafe extern "C" fn () -> *mut TSLanguage>,
				     Option<unsafe extern "C" fn ()>> (langfn),
	       &raw mut loaded_lang.filename, &raw mut sym);

  loaded_lang.lang = lang;
  treesit_lang_cache_put (language_symbol, loaded_lang);
  return loaded_lang;
}
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
