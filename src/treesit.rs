// femacs: port of src/treesit.c; P430 rebases behavior to stable a360712c.
// Historical fd7 range notes below identify the prior feature-disabled slice.
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
// P430 restores the complete stable feature by symbol. Older source-range notes
// below describe the initial disabled-feature port. The SDK/header owners and
// macOS build binding now live in include/treesit.rs and xtask/src/treesit.rs;
// actual ABI and runtime acceptance are recorded separately, not inferred from
// this transcription. The inactive development-only cache is retired in CLN132.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe, unused_assignments, unused_parens, unused_mut, unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals, unused_must_use, clippy::all)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] include/lisp.rs imported via common

/* C 24-27's `buffer.h`, `coding.h` and `treesit.h`.  C compiles the three includes in *every*
   configuration - they precede the file gate at C 29 - but each mirror carries only names the
   *gated* text below names, so the port hangs them on the same feature the gate uses (P-86 ⑧: put
   the gate on the include graph, or the frozen configuration starts compiling names it should not
   see; measured for the sibling unit in docs/ports/decompress.md ③).  */
#[cfg (feature = "have_tree_sitter")]
const _: () = ();
#[cfg (feature = "have_tree_sitter")]
const _: () = ();
#[cfg(feature = "have_tree_sitter")]
const _: () = ();
/* `dynlib.h`'s five names (`dynlib_handle_ptr` and the four functions) are used by C 864-941
   below, and this snapshot's `treesit.c` carries no include that declares them - the region is
   never compiled in the frozen configuration, so C tolerates it, and a tree-sitter
   configuration would need `#include "dynlib.h"` here.  The port takes the names from the
   mirror that owns them (PORTING.md 0.1-3), recorded in docs/ports/treesit.md.  */
#[cfg (feature = "have_tree_sitter")]
const _: () = ();

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
const _: () = ();

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
    static $fnvar: VmGlobal<$pfn> = VmGlobal::new(None);
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
      (*$fnvar.as_mut_ptr ()) = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
      if (*$fnvar.as_mut_ptr ()).is_none () { return false; }
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
             (*fn_ts_language_abi_version.as_mut_ptr ()), (const TSLanguage *));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt",
            not (feature = "treesit_language_version_ge_15")))]
DEF_DLL_FN! (u32, ts_language_version, W32_PFN_ts_language_version,
             (*fn_ts_language_version.as_mut_ptr ()), (const TSLanguage *));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_child, W32_PFN_ts_node_child, (*fn_ts_node_child.as_mut_ptr ()), (TSNode, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_child_by_field_name, W32_PFN_ts_node_child_by_field_name,
             (*fn_ts_node_child_by_field_name.as_mut_ptr ()), (TSNode, *const c_char, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_node_child_count, W32_PFN_ts_node_child_count, (*fn_ts_node_child_count.as_mut_ptr ()),
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_descendant_for_byte_range,
             W32_PFN_ts_node_descendant_for_byte_range, (*fn_ts_node_descendant_for_byte_range.as_mut_ptr ()),
             (TSNode, u32, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_node_end_byte, W32_PFN_ts_node_end_byte, (*fn_ts_node_end_byte.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_eq, W32_PFN_ts_node_eq, (*fn_ts_node_eq.as_mut_ptr ()), (TSNode, TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, ts_node_field_name_for_child,
             W32_PFN_ts_node_field_name_for_child, (*fn_ts_node_field_name_for_child.as_mut_ptr ()),
             (TSNode, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_has_error, W32_PFN_ts_node_has_error, (*fn_ts_node_has_error.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_is_extra, W32_PFN_ts_node_is_extra, (*fn_ts_node_is_extra.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_is_missing, W32_PFN_ts_node_is_missing, (*fn_ts_node_is_missing.as_mut_ptr ()),
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_is_named, W32_PFN_ts_node_is_named, (*fn_ts_node_is_named.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_node_is_null, W32_PFN_ts_node_is_null, (*fn_ts_node_is_null.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_named_child, W32_PFN_ts_node_named_child, (*fn_ts_node_named_child.as_mut_ptr ()),
             (TSNode, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_node_named_child_count, W32_PFN_ts_node_named_child_count,
             (*fn_ts_node_named_child_count.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_named_descendant_for_byte_range,
             W32_PFN_ts_node_named_descendant_for_byte_range,
             (*fn_ts_node_named_descendant_for_byte_range.as_mut_ptr ()), (TSNode, u32, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_next_named_sibling, W32_PFN_ts_node_next_named_sibling,
             (*fn_ts_node_next_named_sibling.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_next_sibling, W32_PFN_ts_node_next_sibling, (*fn_ts_node_next_sibling.as_mut_ptr ()),
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_prev_named_sibling, W32_PFN_ts_node_prev_named_sibling,
             (*fn_ts_node_prev_named_sibling.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_node_prev_sibling, W32_PFN_ts_node_prev_sibling, (*fn_ts_node_prev_sibling.as_mut_ptr ()),
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_node_start_byte, W32_PFN_ts_node_start_byte, (*fn_ts_node_start_byte.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut c_char, ts_node_string, W32_PFN_ts_node_string, (*fn_ts_node_string.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, ts_node_type, W32_PFN_ts_node_type, (*fn_ts_node_type.as_mut_ptr ()), (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_parser_delete, W32_PFN_ts_parser_delete, (*fn_ts_parser_delete.as_mut_ptr ()), (*mut TSParser));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const TSRange, ts_parser_included_ranges, W32_PFN_ts_parser_included_ranges,
             (*fn_ts_parser_included_ranges.as_mut_ptr ()), (*const TSParser, *mut u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const TSLanguage, ts_parser_language, W32_PFN_ts_parser_language,
             (*fn_ts_parser_language.as_mut_ptr ()), (*const TSParser));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSParser, ts_parser_new, W32_PFN_ts_parser_new, (*fn_ts_parser_new.as_mut_ptr ()), ());
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSTree, ts_parser_parse, W32_PFN_ts_parser_parse, (*fn_ts_parser_parse.as_mut_ptr ()),
             (*mut TSParser, *const TSTree, TSInput));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_parser_set_included_ranges, W32_PFN_ts_parser_set_included_ranges,
             (*fn_ts_parser_set_included_ranges.as_mut_ptr ()), (*mut TSParser, *const TSRange, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_parser_set_language, W32_PFN_ts_parser_set_language,
             (*fn_ts_parser_set_language.as_mut_ptr ()), (*mut TSParser, *const TSLanguage));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, ts_query_capture_name_for_id,
             W32_PFN_ts_query_capture_name_for_id, (*fn_ts_query_capture_name_for_id.as_mut_ptr ()),
             (*const TSQuery, u32, *mut u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_query_cursor_delete, W32_PFN_ts_query_cursor_delete,
             (*fn_ts_query_cursor_delete.as_mut_ptr ()), (*mut TSQueryCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_query_cursor_exec, W32_PFN_ts_query_cursor_exec, (*fn_ts_query_cursor_exec.as_mut_ptr ()),
             (*mut TSQueryCursor, *const TSQuery, TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSQueryCursor, ts_query_cursor_new, W32_PFN_ts_query_cursor_new,
             (*fn_ts_query_cursor_new.as_mut_ptr ()), ());
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_query_cursor_next_match, W32_PFN_ts_query_cursor_next_match,
             (*fn_ts_query_cursor_next_match.as_mut_ptr ()), (*mut TSQueryCursor, *mut TSQueryMatch));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_query_cursor_set_byte_range, W32_PFN_ts_query_cursor_set_byte_range,
             (*fn_ts_query_cursor_set_byte_range.as_mut_ptr ()), (*mut TSQueryCursor, u32, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_query_delete, W32_PFN_ts_query_delete, (*fn_ts_query_delete.as_mut_ptr ()), (*mut TSQuery));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSQuery, ts_query_new, W32_PFN_ts_query_new, (*fn_ts_query_new.as_mut_ptr ()),
             (*const TSLanguage, *const c_char, u32, *mut u32, *mut TSQueryError));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (u32, ts_query_pattern_count, W32_PFN_ts_query_pattern_count,
             (*fn_ts_query_pattern_count.as_mut_ptr ()), (*const TSQuery));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const TSQueryPredicateStep, ts_query_predicates_for_pattern,
             W32_PFN_ts_query_predicates_for_pattern, (*fn_ts_query_predicates_for_pattern.as_mut_ptr ()),
             (*const TSQuery, u32, *mut u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, ts_query_string_value_for_id,
             W32_PFN_ts_query_string_value_for_id, (*fn_ts_query_string_value_for_id.as_mut_ptr ()),
             (*const TSQuery, u32, *mut u32));
/* C 158-159: C's four allocator parameters are the four function-pointer types below, in that
   order (C writes `void *(*)(size_t)`, `void *(*)(size_t, size_t)`, `void *(*)(void *, size_t)`
   and `void (*)(void *)`).  */
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_set_allocator, W32_PFN_ts_set_allocator, (*fn_ts_set_allocator.as_mut_ptr ()),
             (Option<unsafe extern "C-unwind" fn (size_t) -> *mut c_void>,
              Option<unsafe extern "C-unwind" fn (size_t, size_t) -> *mut c_void>,
              Option<unsafe extern "C-unwind" fn (*mut c_void, size_t) -> *mut c_void>,
              Option<unsafe extern "C-unwind" fn (*mut c_void)>));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_tree_cursor_current_node, W32_PFN_ts_tree_cursor_current_node,
             (*fn_ts_tree_cursor_current_node.as_mut_ptr ()), (*const TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_tree_cursor_delete, W32_PFN_ts_tree_cursor_delete, (*fn_ts_tree_cursor_delete.as_mut_ptr ()),
             (*const TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_tree_cursor_goto_first_child, W32_PFN_ts_tree_cursor_goto_first_child,
             (*fn_ts_tree_cursor_goto_first_child.as_mut_ptr ()), (*mut TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (i64, ts_tree_cursor_goto_first_child_for_byte,
             W32_PFN_ts_tree_cursor_goto_first_child_for_byte,
             (*fn_ts_tree_cursor_goto_first_child_for_byte.as_mut_ptr ()), (*mut TSTreeCursor, u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_tree_cursor_goto_next_sibling, W32_PFN_ts_tree_cursor_goto_next_sibling,
             (*fn_ts_tree_cursor_goto_next_sibling.as_mut_ptr ()), (*mut TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt",
            feature = "have_ts_tree_cursor_goto_previous_sibling"))]
DEF_DLL_FN! (bool, ts_tree_cursor_goto_previous_sibling,
             W32_PFN_ts_tree_cursor_goto_previous_sibling, (*fn_ts_tree_cursor_goto_previous_sibling.as_mut_ptr ()),
             (*mut TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt",
            not (feature = "have_ts_tree_cursor_goto_previous_sibling")))]
DEF_DLL_FN! (TSTreeCursor, ts_tree_cursor_copy, W32_PFN_ts_tree_cursor_copy, (*fn_ts_tree_cursor_copy.as_mut_ptr ()),
             (*const TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (bool, ts_tree_cursor_goto_parent, W32_PFN_ts_tree_cursor_goto_parent,
             (*fn_ts_tree_cursor_goto_parent.as_mut_ptr ()), (*mut TSTreeCursor));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSTreeCursor, ts_tree_cursor_new, W32_PFN_ts_tree_cursor_new, (*fn_ts_tree_cursor_new.as_mut_ptr ()),
             (TSNode));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_tree_delete, W32_PFN_ts_tree_delete, (*fn_ts_tree_delete.as_mut_ptr ()), (*mut TSTree));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! ((), ts_tree_edit, W32_PFN_ts_tree_edit, (*fn_ts_tree_edit.as_mut_ptr ()),
             (*mut TSTree, *const TSInputEdit));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (*mut TSRange, ts_tree_get_changed_ranges, W32_PFN_ts_tree_get_changed_ranges,
             (*fn_ts_tree_get_changed_ranges.as_mut_ptr ()), (*const TSTree, *const TSTree, *mut u32));
#[cfg (all (feature = "have_tree_sitter", feature = "windowsnt"))]
DEF_DLL_FN! (TSNode, ts_tree_root_node, W32_PFN_ts_tree_root_node, (*fn_ts_tree_root_node.as_mut_ptr ()),
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
                (*fn_ts_language_abi_version.as_mut_ptr ()), c"ts_language_abi_version");
  #[cfg (not (feature = "treesit_language_version_ge_15"))]
  LOAD_DLL_FN! (library, ts_language_version, W32_PFN_ts_language_version,
                (*fn_ts_language_version.as_mut_ptr ()), c"ts_language_version");
  LOAD_DLL_FN! (library, ts_node_child, W32_PFN_ts_node_child, (*fn_ts_node_child.as_mut_ptr ()),
                c"ts_node_child");
  LOAD_DLL_FN! (library, ts_node_child_by_field_name, W32_PFN_ts_node_child_by_field_name,
                (*fn_ts_node_child_by_field_name.as_mut_ptr ()), c"ts_node_child_by_field_name");
  LOAD_DLL_FN! (library, ts_node_child_count, W32_PFN_ts_node_child_count,
                (*fn_ts_node_child_count.as_mut_ptr ()), c"ts_node_child_count");
  LOAD_DLL_FN! (library, ts_node_descendant_for_byte_range,
                W32_PFN_ts_node_descendant_for_byte_range,
                (*fn_ts_node_descendant_for_byte_range.as_mut_ptr ()), c"ts_node_descendant_for_byte_range");
  LOAD_DLL_FN! (library, ts_node_end_byte, W32_PFN_ts_node_end_byte, (*fn_ts_node_end_byte.as_mut_ptr ()),
                c"ts_node_end_byte");
  LOAD_DLL_FN! (library, ts_node_eq, W32_PFN_ts_node_eq, (*fn_ts_node_eq.as_mut_ptr ()), c"ts_node_eq");
  LOAD_DLL_FN! (library, ts_node_field_name_for_child,
                W32_PFN_ts_node_field_name_for_child, (*fn_ts_node_field_name_for_child.as_mut_ptr ()),
                c"ts_node_field_name_for_child");
  LOAD_DLL_FN! (library, ts_node_has_error, W32_PFN_ts_node_has_error, (*fn_ts_node_has_error.as_mut_ptr ()),
                c"ts_node_has_error");
  LOAD_DLL_FN! (library, ts_node_is_extra, W32_PFN_ts_node_is_extra, (*fn_ts_node_is_extra.as_mut_ptr ()),
                c"ts_node_is_extra");
  LOAD_DLL_FN! (library, ts_node_is_missing, W32_PFN_ts_node_is_missing, (*fn_ts_node_is_missing.as_mut_ptr ()),
                c"ts_node_is_missing");
  LOAD_DLL_FN! (library, ts_node_is_named, W32_PFN_ts_node_is_named, (*fn_ts_node_is_named.as_mut_ptr ()),
                c"ts_node_is_named");
  LOAD_DLL_FN! (library, ts_node_is_null, W32_PFN_ts_node_is_null, (*fn_ts_node_is_null.as_mut_ptr ()),
                c"ts_node_is_null");
  LOAD_DLL_FN! (library, ts_node_named_child, W32_PFN_ts_node_named_child,
                (*fn_ts_node_named_child.as_mut_ptr ()), c"ts_node_named_child");
  LOAD_DLL_FN! (library, ts_node_named_child_count, W32_PFN_ts_node_named_child_count,
                (*fn_ts_node_named_child_count.as_mut_ptr ()), c"ts_node_named_child_count");
  LOAD_DLL_FN! (library, ts_node_named_descendant_for_byte_range,
                W32_PFN_ts_node_named_descendant_for_byte_range,
                (*fn_ts_node_named_descendant_for_byte_range.as_mut_ptr ()),
                c"ts_node_named_descendant_for_byte_range");
  LOAD_DLL_FN! (library, ts_node_next_named_sibling, W32_PFN_ts_node_next_named_sibling,
                (*fn_ts_node_next_named_sibling.as_mut_ptr ()), c"ts_node_next_named_sibling");
  LOAD_DLL_FN! (library, ts_node_next_sibling, W32_PFN_ts_node_next_sibling,
                (*fn_ts_node_next_sibling.as_mut_ptr ()), c"ts_node_next_sibling");
  LOAD_DLL_FN! (library, ts_node_prev_named_sibling, W32_PFN_ts_node_prev_named_sibling,
                (*fn_ts_node_prev_named_sibling.as_mut_ptr ()), c"ts_node_prev_named_sibling");
  LOAD_DLL_FN! (library, ts_node_prev_sibling, W32_PFN_ts_node_prev_sibling,
                (*fn_ts_node_prev_sibling.as_mut_ptr ()), c"ts_node_prev_sibling");
  LOAD_DLL_FN! (library, ts_node_start_byte, W32_PFN_ts_node_start_byte, (*fn_ts_node_start_byte.as_mut_ptr ()),
                c"ts_node_start_byte");
  LOAD_DLL_FN! (library, ts_node_string, W32_PFN_ts_node_string, (*fn_ts_node_string.as_mut_ptr ()),
                c"ts_node_string");
  LOAD_DLL_FN! (library, ts_node_type, W32_PFN_ts_node_type, (*fn_ts_node_type.as_mut_ptr ()), c"ts_node_type");
  LOAD_DLL_FN! (library, ts_parser_delete, W32_PFN_ts_parser_delete, (*fn_ts_parser_delete.as_mut_ptr ()),
                c"ts_parser_delete");
  LOAD_DLL_FN! (library, ts_parser_included_ranges, W32_PFN_ts_parser_included_ranges,
                (*fn_ts_parser_included_ranges.as_mut_ptr ()), c"ts_parser_included_ranges");
  LOAD_DLL_FN! (library, ts_parser_language, W32_PFN_ts_parser_language, (*fn_ts_parser_language.as_mut_ptr ()),
                c"ts_parser_language");
  LOAD_DLL_FN! (library, ts_parser_new, W32_PFN_ts_parser_new, (*fn_ts_parser_new.as_mut_ptr ()), c"ts_parser_new");
  LOAD_DLL_FN! (library, ts_parser_parse, W32_PFN_ts_parser_parse, (*fn_ts_parser_parse.as_mut_ptr ()),
                c"ts_parser_parse");
  LOAD_DLL_FN! (library, ts_parser_set_included_ranges,
                W32_PFN_ts_parser_set_included_ranges, (*fn_ts_parser_set_included_ranges.as_mut_ptr ()),
                c"ts_parser_set_included_ranges");
  LOAD_DLL_FN! (library, ts_parser_set_language, W32_PFN_ts_parser_set_language,
                (*fn_ts_parser_set_language.as_mut_ptr ()), c"ts_parser_set_language");
  LOAD_DLL_FN! (library, ts_query_capture_name_for_id,
                W32_PFN_ts_query_capture_name_for_id, (*fn_ts_query_capture_name_for_id.as_mut_ptr ()),
                c"ts_query_capture_name_for_id");
  LOAD_DLL_FN! (library, ts_query_cursor_delete, W32_PFN_ts_query_cursor_delete,
                (*fn_ts_query_cursor_delete.as_mut_ptr ()), c"ts_query_cursor_delete");
  LOAD_DLL_FN! (library, ts_query_cursor_exec, W32_PFN_ts_query_cursor_exec,
                (*fn_ts_query_cursor_exec.as_mut_ptr ()), c"ts_query_cursor_exec");
  LOAD_DLL_FN! (library, ts_query_cursor_new, W32_PFN_ts_query_cursor_new, (*fn_ts_query_cursor_new.as_mut_ptr ()),
                c"ts_query_cursor_new");
  LOAD_DLL_FN! (library, ts_query_cursor_next_match, W32_PFN_ts_query_cursor_next_match,
                (*fn_ts_query_cursor_next_match.as_mut_ptr ()), c"ts_query_cursor_next_match");
  LOAD_DLL_FN! (library, ts_query_cursor_set_byte_range,
                W32_PFN_ts_query_cursor_set_byte_range, (*fn_ts_query_cursor_set_byte_range.as_mut_ptr ()),
                c"ts_query_cursor_set_byte_range");
  LOAD_DLL_FN! (library, ts_query_delete, W32_PFN_ts_query_delete, (*fn_ts_query_delete.as_mut_ptr ()),
                c"ts_query_delete");
  LOAD_DLL_FN! (library, ts_query_new, W32_PFN_ts_query_new, (*fn_ts_query_new.as_mut_ptr ()), c"ts_query_new");
  LOAD_DLL_FN! (library, ts_query_pattern_count, W32_PFN_ts_query_pattern_count,
                (*fn_ts_query_pattern_count.as_mut_ptr ()), c"ts_query_pattern_count");
  LOAD_DLL_FN! (library, ts_query_predicates_for_pattern,
                W32_PFN_ts_query_predicates_for_pattern, (*fn_ts_query_predicates_for_pattern.as_mut_ptr ()),
                c"ts_query_predicates_for_pattern");
  LOAD_DLL_FN! (library, ts_query_string_value_for_id,
                W32_PFN_ts_query_string_value_for_id, (*fn_ts_query_string_value_for_id.as_mut_ptr ()),
                c"ts_query_string_value_for_id");
  LOAD_DLL_FN! (library, ts_set_allocator, W32_PFN_ts_set_allocator, (*fn_ts_set_allocator.as_mut_ptr ()),
                c"ts_set_allocator");
  LOAD_DLL_FN! (library, ts_tree_cursor_current_node, W32_PFN_ts_tree_cursor_current_node,
                (*fn_ts_tree_cursor_current_node.as_mut_ptr ()), c"ts_tree_cursor_current_node");
  LOAD_DLL_FN! (library, ts_tree_cursor_delete, W32_PFN_ts_tree_cursor_delete,
                (*fn_ts_tree_cursor_delete.as_mut_ptr ()), c"ts_tree_cursor_delete");
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_first_child,
                W32_PFN_ts_tree_cursor_goto_first_child, (*fn_ts_tree_cursor_goto_first_child.as_mut_ptr ()),
                c"ts_tree_cursor_goto_first_child");
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_first_child_for_byte,
                W32_PFN_ts_tree_cursor_goto_first_child_for_byte,
                (*fn_ts_tree_cursor_goto_first_child_for_byte.as_mut_ptr ()),
                c"ts_tree_cursor_goto_first_child_for_byte");
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_next_sibling,
                W32_PFN_ts_tree_cursor_goto_next_sibling, (*fn_ts_tree_cursor_goto_next_sibling.as_mut_ptr ()),
                c"ts_tree_cursor_goto_next_sibling");
  #[cfg (feature = "have_ts_tree_cursor_goto_previous_sibling")]
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_previous_sibling,
                W32_PFN_ts_tree_cursor_goto_previous_sibling,
                (*fn_ts_tree_cursor_goto_previous_sibling.as_mut_ptr ()),
                c"ts_tree_cursor_goto_previous_sibling");
  #[cfg (not (feature = "have_ts_tree_cursor_goto_previous_sibling"))]
  LOAD_DLL_FN! (library, ts_tree_cursor_copy, W32_PFN_ts_tree_cursor_copy,
                (*fn_ts_tree_cursor_copy.as_mut_ptr ()), c"ts_tree_cursor_copy");
  LOAD_DLL_FN! (library, ts_tree_cursor_goto_parent, W32_PFN_ts_tree_cursor_goto_parent,
                (*fn_ts_tree_cursor_goto_parent.as_mut_ptr ()), c"ts_tree_cursor_goto_parent");
  LOAD_DLL_FN! (library, ts_tree_cursor_new, W32_PFN_ts_tree_cursor_new, (*fn_ts_tree_cursor_new.as_mut_ptr ()),
                c"ts_tree_cursor_new");
  LOAD_DLL_FN! (library, ts_tree_delete, W32_PFN_ts_tree_delete, (*fn_ts_tree_delete.as_mut_ptr ()),
                c"ts_tree_delete");
  LOAD_DLL_FN! (library, ts_tree_edit, W32_PFN_ts_tree_edit, (*fn_ts_tree_edit.as_mut_ptr ()), c"ts_tree_edit");
  LOAD_DLL_FN! (library, ts_tree_get_changed_ranges, W32_PFN_ts_tree_get_changed_ranges,
                (*fn_ts_tree_get_changed_ranges.as_mut_ptr ()), c"ts_tree_get_changed_ranges");
  LOAD_DLL_FN! (library, ts_tree_root_node, W32_PFN_ts_tree_root_node, (*fn_ts_tree_root_node.as_mut_ptr ()),
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
static Vtreesit_str_libtree_sitter: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_tree_sitter: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (all (feature = "have_tree_sitter", not (feature = "windowsnt")))]
static Vtreesit_str_dot_0: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_dot: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_question_mark: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_star: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_plus: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_pound_eq_question_mark: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_pound_match_question_mark: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_pound_pred_question_mark: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_open_bracket: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_close_bracket: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_open_paren: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_close_paren: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_space: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_eq_question_mark: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_match_question_mark: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_pred_question_mark: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[cfg (feature = "have_tree_sitter")]
static Vtreesit_str_empty: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

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
static treesit_initialized: VmGlobal<bool> = VmGlobal::<bool>::new (false);

/* C 535-561: `load_tree_sitter_if_necessary`.  C's two function-local `static bool`s are the
   block-scoped `static mut`s below, and the `#else` arm (C 559-560) is the non-Windows one.  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn load_tree_sitter_if_necessary (required: bool) -> bool
{
  #[cfg (feature = "windowsnt")]
  {
    static tried_to_initialize_once: VmGlobal<bool> = VmGlobal::<bool>::new (false);
    static tree_sitter_initialized: VmGlobal<bool> = VmGlobal::<bool>::new (false);

    if !(*tried_to_initialize_once.as_mut_ptr ())
      {
        let status: Lisp_Object;

        (*tried_to_initialize_once.as_mut_ptr ()) = true;
        (*tree_sitter_initialized.as_mut_ptr ()) = init_treesit_functions ();
        status = if (*tree_sitter_initialized.as_mut_ptr ()) { Qt () } else { Qnil () };
        (*Vlibrary_cache.as_mut_ptr ()) = Fcons (Fcons (Qtree_sitter (), status), (*Vlibrary_cache.as_mut_ptr ()));
      }

    if required && !(*tree_sitter_initialized.as_mut_ptr ())
      { xsignal1 (Qtreesit_error (),
		  build_string (c"tree-sitter library not found or failed to load".as_ptr ())); }

    return (*tree_sitter_initialized.as_mut_ptr ());
  }
  #[cfg (not (feature = "windowsnt"))]
  {
    return true;
  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe extern "C" fn treesit_malloc_wrapper(size: usize) -> *mut c_void {
  // SAFETY: `ts_set_allocator` callbacks are invoked from C (`libtree-sitter`); catch VM OOM
  // `NonLocalExit` at the FFI boundary and return NULL so no Rust unwind crosses C frames.
  crate::nonlocal::catch_vm_nonlocal(|| unsafe { xmalloc(size) }).unwrap_or(core::ptr::null_mut())
}

// Stable 31.1 allocator callback, before the development branch used xcalloc.
#[cfg(feature = "have_tree_sitter")]
unsafe extern "C" fn treesit_calloc_wrapper(n: usize, size: usize) -> *mut c_void {
  // SAFETY: Same C FFI boundary guard as `treesit_malloc_wrapper`.
  crate::nonlocal::catch_vm_nonlocal(|| unsafe { xzalloc(n.wrapping_mul(size)) })
    .unwrap_or(core::ptr::null_mut())
}

#[cfg(feature = "have_tree_sitter")]
unsafe extern "C" fn treesit_realloc_wrapper(ptr: *mut c_void, size: usize) -> *mut c_void {
  // SAFETY: Same C FFI boundary guard as `treesit_malloc_wrapper`.
  crate::nonlocal::catch_vm_nonlocal(|| unsafe { xrealloc(ptr, size) })
    .unwrap_or(core::ptr::null_mut())
}

#[cfg(feature = "have_tree_sitter")]
unsafe extern "C" fn treesit_free_wrapper(ptr: *mut c_void) {
  // SAFETY: `xfree` does not signal, wrapped in `extern "C"` for `ts_set_allocator`.
  let _ = crate::nonlocal::catch_vm_nonlocal(|| unsafe { xfree(ptr) });
}

/* C 563-574: `treesit_initialize`.  `ts_set_allocator`'s four arguments are C's four allocator
   names; the `windowsnt` arm calls the loaded pointer instead (the `#define` run above).  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_initialize ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if !(*treesit_initialized.as_mut_ptr ())
    {
      load_tree_sitter_if_necessary (true);
      #[cfg (not (feature = "windowsnt"))]
      { ts_set_allocator (Some (treesit_malloc_wrapper), Some (treesit_calloc_wrapper), Some (treesit_realloc_wrapper), Some (treesit_free_wrapper)); }
      #[cfg (feature = "windowsnt")]
      { ((*fn_ts_set_allocator.as_mut_ptr ()).unwrap ()) (Some (treesit_malloc_wrapper), Some (treesit_calloc_wrapper), Some (treesit_realloc_wrapper), Some (treesit_free_wrapper)); }
      (*treesit_initialized.as_mut_ptr ()) = true;
    }

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
pub unsafe extern "C-unwind" fn treesit_debug_print_parser_list (msg: *mut c_char, parser: Lisp_Object)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let buf: *mut buffer = XBUFFER ((*XTS_PARSER (parser)).buffer);
  let buf_name: *mut c_char = SSDATA (BVAR! (buf, name_));
  printf (c"%s (%s) [%s] <%s>: %td(%td)-(%td)%td {\n".as_ptr (),
	  if msg.is_null () { c"".as_ptr () } else { msg },
	  SSDATA (SYMBOL_NAME ((*Vthis_command.as_mut_ptr ()))),
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
	= ((*fn_ts_parser_included_ranges.as_mut_ptr ()).unwrap ()) ((*parser).parser, &raw mut len);

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

/* Translate a symbol treesit-<lang> to a C name treesit_<lang>.  */
/* C 687-698: C's comparison is `int idx < size_t len`, i.e. the int is converted to `size_t`; the
   port writes that conversion.  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn treesit_symbol_to_c_name (symbol_name: *mut c_char)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let len: size_t = strlen (symbol_name);
  let mut idx: c_int = 0;
  while (idx as size_t) < len
    {
      if *symbol_name.offset (idx as isize) == b'-' as c_char
	{ *symbol_name.offset (idx as isize) = b'_' as c_char; }
      idx += 1;
    }

  }
}

/* Resolve language symbol LANG according to
   treesit-language-remap-alist.  */
/* C 700-708: `resolve_language_symbol`.  */
#[cfg (feature = "have_tree_sitter")]
unsafe fn resolve_language_symbol (lang: Lisp_Object) -> Lisp_Object
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let res: Lisp_Object = Fassoc (lang, (*Vtreesit_language_remap_alist.as_mut_ptr ()), Qeq ());
  if NILP (res)
    { return lang; }
  return Fcdr (res);

  }
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
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  CHECK_LIST ((*Vtreesit_load_name_override_list.as_mut_ptr ()));
  let mut tail: Lisp_Object = (*Vtreesit_load_name_override_list.as_mut_ptr ());

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

  CHECK_LIST_END (tail, (*Vtreesit_load_name_override_list.as_mut_ptr ()));

  return false;

  }
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
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut suffixes: Lisp_Object;

  suffixes = (*Vdynamic_library_suffixes.as_mut_ptr ());

  FOR_EACH_TAIL! (suffixes, {
      let candidate1: Lisp_Object = concat2 (lib_base_name, XCAR (suffixes));
      #[cfg (not (feature = "windowsnt"))]
      {
	/* On Posix hosts, support libraries named with ABI version
	   numbers.  Originally tree-sitter grammars are always versioned
	   at 0.0, so we first try that.  For more details, see
	   https://lists.gnu.org/archive/html/emacs-devel/2023-04/msg00386.html.  */
	let candidate2: Lisp_Object = concat2 (candidate1, (*Vtreesit_str_dot_0.as_mut_ptr ()));
	let candidate3: Lisp_Object = concat2 (candidate2, (*Vtreesit_str_dot_0.as_mut_ptr ()));

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
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  #[cfg (all (feature = "treesit_language_version_ge_15", not (feature = "windowsnt")))]
  { return ts_language_abi_version (ts_lang); }
  #[cfg (all (feature = "treesit_language_version_ge_15", feature = "windowsnt"))]
  { return ((*fn_ts_language_abi_version.as_mut_ptr ()).unwrap ()) (ts_lang); }
  #[cfg (all (not (feature = "treesit_language_version_ge_15"), not (feature = "windowsnt")))]
  { return ts_language_version (ts_lang); }
  #[cfg (all (not (feature = "treesit_language_version_ge_15"), feature = "windowsnt"))]
  { return ((*fn_ts_language_version.as_mut_ptr ()).unwrap ()) (ts_lang); }

  }
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
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let symbol_name: Lisp_Object = Fsymbol_name (language_symbol);

  CHECK_LIST ((*Vtreesit_extra_load_path.as_mut_ptr ()));

  /* Figure out the library name and C name.  */
  let mut lib_base_name: Lisp_Object = concat2 ((*Vtreesit_str_libtree_sitter.as_mut_ptr ()), symbol_name);
  let base_name: Lisp_Object = concat2 ((*Vtreesit_str_tree_sitter.as_mut_ptr ()), symbol_name);

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

  tail = Freverse ((*Vtreesit_extra_load_path.as_mut_ptr ()));

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
  let parser: *mut TSParser = ((*fn_ts_parser_new.as_mut_ptr ()).unwrap ()) ();
  #[cfg (not (feature = "windowsnt"))]
  let success: bool = ts_parser_set_language (parser, lang);
  #[cfg (feature = "windowsnt")]
  let success: bool = ((*fn_ts_parser_set_language.as_mut_ptr ()).unwrap ()) (parser, lang);
  #[cfg (not (feature = "windowsnt"))]
  { ts_parser_delete (parser); }
  #[cfg (feature = "windowsnt")]
  { ((*fn_ts_parser_delete.as_mut_ptr ()).unwrap ()) (parser); }
  if !success
    {
      let fmt: Lisp_Object
	= build_string (c"%s's ABI version is %d, but supported versions are %d-%d".as_ptr ());
      let formatted_msg: Lisp_Object
	= CALLN! (Fformat_message, fmt, loaded_lib,
		  make_fixnum (treesit_language_abi_version (lang) as EMACS_INT),
		  make_fixnum (TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION as EMACS_INT),
		  make_fixnum (TREE_SITTER_LANGUAGE_VERSION as EMACS_INT));
      *signal_symbol = Qtreesit_load_language_error ();
      *signal_data = list2 (Qlang_version_mismatch (), formatted_msg);
      return loaded_lang;
    }

  let mut sym: *const c_char = std::ptr::null ();
  dynlib_addr (std::mem::transmute::<Option<unsafe extern "C" fn () -> *mut TSLanguage>,
				     Option<unsafe extern "C" fn ()>> (langfn),
	       &raw mut loaded_lang.filename, &raw mut sym);

  loaded_lang.lang = lang;
  return loaded_lang;

  }
}

#[cfg(feature = "have_tree_sitter")]
const TREESIT_DEBUG_LINECOL:bool=false;

#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-language-available-p", Ftreesit_language_available_p, Streesit_language_available_p, 1, 2, 0,
  doc: "Return non-nil if LANGUAGE exists and is loadable.\n\nIf DETAIL is non-nil, return (t . nil) when LANGUAGE is available,\n(nil . DATA) when unavailable.  DATA is the signal data of\n`treesit-load-language-error'.",
  (language: Lisp_Object, detail: Lisp_Object),
  {
CHECK_SYMBOL(language); treesit_initialize();
let mut signal_symbol=Qnil(); let mut signal_data=Qnil();
let loaded_lang=treesit_load_language(language,&raw mut signal_symbol,&raw mut signal_data);
if loaded_lang.lang.is_null() {
 return if NILP(detail) { Qnil() } else { Fcons(Qnil(),signal_data) };
}
return if NILP(detail) { Qt() } else { Fcons(Qt(),Qnil()) };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-library-abi-version", Ftreesit_library_abi_version, Streesit_library_abi_version, 0, 1, 0,
  doc: "Return the language ABI version of the tree-sitter library.\n\nBy default, report the latest ABI version supported by the library for\nloading language support modules.  The library is backward-compatible\nwith language modules which use older ABI versions; if MIN-COMPATIBLE\nis non-nil, return the oldest compatible ABI version.",
  (min_compatible: Lisp_Object),
  {
return make_fixnum(if NILP(min_compatible) { TREE_SITTER_LANGUAGE_VERSION as EMACS_INT }
                  else { TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION as EMACS_INT });
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-language-abi-version", Ftreesit_language_abi_version, Streesit_language_abi_version, 0, 1, 0,
  doc: "Return the ABI version of the tree-sitter grammar for LANGUAGE.\nReturn nil if a grammar library for LANGUAGE is not available.",
  (language: Lisp_Object),
  {
if NILP(Ftreesit_language_available_p(language,Qnil())) { return Qnil(); }
let mut signal_symbol=Qnil(); let mut signal_data=Qnil();
let lang=treesit_load_language(language,&raw mut signal_symbol,&raw mut signal_data);
let ts_language=lang.lang;
if ts_language.is_null() { return Qnil(); }
let version=treesit_language_abi_version(ts_language);
return make_fixnum(version as EMACS_INT);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-grammar-location", Ftreesit_grammar_location, Streesit_grammar_location, 1, 1, 0,
  doc: "Return the absolute file name of the grammar file for LANGUAGE.\n\nIf LANGUAGE isn't loaded yet, load it first.  If the language can't be\nloaded or the file name couldn't be determined, return nil.",
  (language: Lisp_Object),
  {
CHECK_SYMBOL(language);
let mut signal_symbol=Qnil(); let mut signal_data=Qnil();
let lang=treesit_load_language(language,&raw mut signal_symbol,&raw mut signal_data);
if lang.lang.is_null() || lang.filename.is_null() { return Qnil(); }
return DECODE_FILE(make_unibyte_string(lang.filename,strlen(lang.filename) as ptrdiff_t));
});


#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn treesit_debug_print_linecol(linecol:ts_linecol) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 printf(c"{ line=%td col=%td bytepos=%td }\n".as_ptr(),linecol.line,linecol.col,linecol.bytepos);

  }
}


#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_buf_tracks_linecol_p(buf:*mut buffer)->bool {
 return BUF_TS_LINECOL_BEGV(buf).bytepos != 0;
}


#[cfg(feature = "have_tree_sitter")]
unsafe extern "C-unwind" fn restore_restriction_and_selective_display(record:Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 save_restriction_restore(Fcar(record));
 BVAR!(current_buffer!(),selective_display_)=Fcdr(record);

  }
}


#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_count_lines(start_byte:ptrdiff_t,limit_byte:ptrdiff_t,count:ptrdiff_t,
                             byte_pos_ptr:*mut ptrdiff_t)->ptrdiff_t {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 let record=Fcons(save_restriction_save(),BVAR!(current_buffer!(),selective_display_));
 let pdl_count=SPECPDL_INDEX();
 record_unwind_protect(Some(restore_restriction_and_selective_display),record);
 BVAR!(current_buffer!(),selective_display_)=Qnil();
 labeled_restrictions_remove_in_current_buffer(); Fwiden();
 let mut counted=display_count_lines(start_byte,limit_byte,count,byte_pos_ptr);
 unbind_to(pdl_count,Qnil());
 if count < 0 && limit_byte != *byte_pos_ptr { counted += 1; *byte_pos_ptr -= 1; }
 return counted;

  }
}


#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_debug_validate_linecol(linecol:ts_linecol) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 eassert!(linecol.bytepos <= Z_BYTE!());
 let mut unused=0;
 let true_line_count=treesit_count_lines(BEG_BYTE as ptrdiff_t,linecol.bytepos,Z_BYTE!(),&raw mut unused)+1;
 eassert!(true_line_count==linecol.line);

  }
}


#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_linecol_of_pos(target_bytepos:ptrdiff_t,cache:ts_linecol)->ts_linecol {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 if TREESIT_DEBUG_LINECOL { treesit_debug_validate_linecol(cache); }
 let mut byte_pos_1=0; let mut byte_pos_2=0; let mut line_delta=0;
 if target_bytepos==cache.bytepos { return cache; }
 if cache.bytepos < target_bytepos {
  byte_pos_2=cache.bytepos;
  while byte_pos_2 < target_bytepos {
   let counted=treesit_count_lines(byte_pos_2,target_bytepos,1,&raw mut byte_pos_2);
   if counted>0 { byte_pos_1=byte_pos_2; }
   line_delta += counted;
  }
  eassert!(byte_pos_2==target_bytepos);
  let target_linecol=ts_linecol { bytepos:target_bytepos,line:cache.line+line_delta,
   col:if line_delta>0 { target_bytepos-byte_pos_1 } else { target_bytepos-cache.bytepos+cache.col } };
  if TREESIT_DEBUG_LINECOL { treesit_debug_validate_linecol(target_linecol); }
  return target_linecol;
 }
 byte_pos_2=cache.bytepos;
 while byte_pos_2 > target_bytepos {
  let counted=treesit_count_lines(byte_pos_2,target_bytepos,-1,&raw mut byte_pos_2);
  line_delta -= counted;
 }
 eassert!(byte_pos_2==target_bytepos);
 let mut target_linecol=ts_linecol { bytepos:target_bytepos,line:cache.line+line_delta,col:0 };
 eassert!(cache.line+line_delta>0);
 if line_delta==0 { target_linecol.col=cache.col-(cache.bytepos-target_bytepos); }
 else {
  let counted=treesit_count_lines(byte_pos_2,BEG_BYTE as ptrdiff_t,-1,&raw mut byte_pos_2);
  // Preserve the selected C expression, including its conditional precedence.
  target_linecol.col=target_bytepos-if byte_pos_2+counted==1 { 1 } else { 0 };
 }
 if TREESIT_DEBUG_LINECOL { treesit_debug_validate_linecol(target_linecol); }
 return target_linecol;

  }
}


#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_make_ts_point(visible_beg:ts_linecol,pos:ts_linecol)->TSPoint {
 let point=if visible_beg.line==pos.line {
  let point=TSPoint { row:0,column:(pos.col-visible_beg.col) as u32 };
  eassert!(point.column>=0);point
 } else {
  let point=TSPoint { row:(pos.line-visible_beg.line) as u32,column:pos.col as u32 };
  eassert!(point.row>0);point
 };
 return point;
}


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-tracking-line-column-p", Ftreesit_tracking_line_column_p, Streesit_tracking_line_column_p, 0, 1, 0,
  doc: "Return non-nil if BUFFER is tracking line and column.\n\nReturn nil otherwise.  BUFFER defaults to the current buffer.",
  (buffer: Lisp_Object),
  {
let mut buf=current_buffer!();
if !NILP(buffer) { CHECK_BUFFER(buffer);buf=XBUFFER(buffer); }
return if treesit_buf_tracks_linecol_p(buf) { Qt() } else { Qnil() };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-tracking-line-column-p", Ftreesit_parser_tracking_line_column_p, Streesit_parser_tracking_line_column_p, 1, 1, 0,
  doc: "Return non-nil if PARSER is tracking line and column.\n\nReturn nil otherwise.",
  (parser: Lisp_Object),
  {
CHECK_TS_PARSER(parser);
return if (*XTS_PARSER(parser)).visi_beg_linecol.bytepos==0 { Qnil() } else { Qt() };
});

#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub static Vtreesit_load_name_override_list: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil());
#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub static Vtreesit_extra_load_path: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil());
#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub static Vtreesit_thing_settings: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil());
#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub static Vtreesit_language_remap_alist: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil());
#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub static Vtreesit_languages_require_line_column_tracking: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil());
#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub static Vtreesit_major_mode_remap_alist: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil());

// P430 stable parsing/edit path. A tree records the parser's previous visible
// interval; edits update it before the next lazy parse reconciles narrowing.
#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_check_parser(obj: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    CHECK_TS_PARSER(obj);
    if (*XTS_PARSER(obj)).deleted { xsignal1(Qtreesit_parser_deleted(), obj); }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_tree_edit_1(tree: *mut TSTree, start_byte: ptrdiff_t,
    old_end_byte: ptrdiff_t, new_end_byte: ptrdiff_t,
    start_point: TSPoint, old_end_point: TSPoint, new_end_point: TSPoint) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    eassert!(start_byte >= 0);
    eassert!(start_byte <= old_end_byte);
    eassert!(start_byte <= new_end_byte);
    eassert!(start_byte <= u32::MAX as ptrdiff_t);
    eassert!(old_end_byte <= u32::MAX as ptrdiff_t);
    eassert!(new_end_byte <= u32::MAX as ptrdiff_t);
    let edit = TSInputEdit { start_byte: start_byte as u32,
        old_end_byte: old_end_byte as u32, new_end_byte: new_end_byte as u32,
        start_point, old_end_point, new_end_point };
    ts_tree_edit(tree, &edit);

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn compute_new_linecol_by_change(pos_linecol: ts_linecol,
    start_linecol: ts_linecol, old_end_linecol: ts_linecol,
    new_end_linecol: ts_linecol, target_bytepos: ptrdiff_t) -> ts_linecol {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut new_linecol = ts_linecol { bytepos: 0, line: 0, col: 0 };
    if start_linecol.bytepos >= pos_linecol.bytepos {
        new_linecol = pos_linecol;
    } else if old_end_linecol.bytepos <= pos_linecol.bytepos {
        let line_delta = new_end_linecol.line - old_end_linecol.line;
        new_linecol.line = pos_linecol.line + line_delta;
        new_linecol.bytepos = pos_linecol.bytepos + new_end_linecol.bytepos - old_end_linecol.bytepos;
        if old_end_linecol.line == pos_linecol.line {
            eassert!(old_end_linecol.col <= pos_linecol.col);
            let old_end_to_pos = pos_linecol.col - old_end_linecol.col;
            new_linecol.col = new_end_linecol.col + old_end_to_pos;
        } else { new_linecol.col = pos_linecol.col; }
    } else if target_bytepos - start_linecol.bytepos < (target_bytepos - new_end_linecol.bytepos).abs() {
        new_linecol = treesit_linecol_of_pos(target_bytepos, start_linecol);
    } else { new_linecol = treesit_linecol_of_pos(target_bytepos, new_end_linecol); }
    if new_linecol.bytepos != target_bytepos {
        new_linecol = treesit_linecol_of_pos(target_bytepos, new_linecol);
    }
    if TREESIT_DEBUG_LINECOL { treesit_debug_validate_linecol(new_linecol); }
    new_linecol

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_record_change_1(start_byte: ptrdiff_t, old_end_byte: ptrdiff_t,
    new_end_byte: ptrdiff_t, start_linecol: ts_linecol,
    old_end_linecol: ts_linecol, new_end_linecol: ts_linecol) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut base_buffer = current_buffer!();
    if !(*current_buffer!()).base_buffer.is_null() { base_buffer = (*current_buffer!()).base_buffer; }
    let mut parser_list = BVAR!(base_buffer, ts_parser_list_);
    let buf_tracks_linecol = start_linecol.bytepos != 0;
    FOR_EACH_TAIL_SAFE!(parser_list, {
        CHECK_CONS(parser_list);
        let lisp_parser = XCAR(parser_list);
        treesit_check_parser(lisp_parser);
        let p = XTS_PARSER(lisp_parser);
        let tree = (*p).tree;
        let parser_tracks_linecol = (*p).visi_beg_linecol.bytepos != 0;
        if !tree.is_null() {
            eassert!(start_byte <= old_end_byte);
            eassert!(start_byte <= new_end_byte);
            let visible_beg = (*p).visible_beg;
            let visible_end = (*p).visible_end;
            eassert!(visible_beg >= 0);
            eassert!(visible_beg <= visible_end);
            let start_offset = visible_end.min(visible_beg.max(start_byte)) - visible_beg;
            let old_end_offset = visible_end.min(visible_beg.max(old_end_byte)) - visible_beg;
            // The inserted tail grows the visible interval; do not clip it at its old end.
            let new_end_offset = visible_beg.max(new_end_byte) - visible_beg;
            eassert!(start_offset <= old_end_offset);
            eassert!(start_offset <= new_end_offset);
            let visi_beg_delta = if old_end_byte > new_end_byte {
                visible_beg.min(new_end_byte) - visible_beg.min(old_end_byte)
            } else if old_end_byte < visible_beg { new_end_byte - old_end_byte } else { 0 };
            let new_visible_beg = visible_beg + visi_beg_delta;
            let new_visible_end = visible_end + visi_beg_delta + new_end_offset - old_end_offset;
            (*p).visible_beg = new_visible_beg;
            (*p).visible_end = new_visible_end;
            eassert!(BEG_BYTE as ptrdiff_t <= new_visible_beg);
            eassert!(new_visible_beg <= new_visible_end);
            eassert!(new_visible_end <= Z_BYTE!());
            let mut start_point = TREESIT_TS_POINT_1_0;
            let mut old_end_point = TREESIT_TS_POINT_1_0;
            let mut new_end_point = TREESIT_TS_POINT_1_0;
            if parser_tracks_linecol {
                eassert!(buf_tracks_linecol);
                let old_visi_beg_linecol = (*p).visi_beg_linecol;
                let old_visi_end_linecol = (*p).visi_end_linecol;
                let new_visi_beg_linecol = compute_new_linecol_by_change(old_visi_beg_linecol,
                    start_linecol, old_end_linecol, new_end_linecol, new_visible_beg);
                let new_visi_end_linecol = compute_new_linecol_by_change(old_visi_end_linecol,
                    start_linecol, old_end_linecol, new_end_linecol, new_visible_end);
                (*p).visi_beg_linecol = new_visi_beg_linecol;
                (*p).visi_end_linecol = new_visi_end_linecol;
                let new_begv_linecol = (*p).visi_beg_linecol;
                old_end_point = treesit_make_ts_point(old_visi_beg_linecol, old_end_linecol);
                start_point = treesit_make_ts_point(new_begv_linecol, start_linecol);
                new_end_point = treesit_make_ts_point(new_begv_linecol, new_end_linecol);
            }
            treesit_tree_edit_1(tree, start_offset, old_end_offset, new_end_offset,
                start_point, old_end_point, new_end_point);
            (*p).need_reparse = true;
        }
    });

  }
}

#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn treesit_linecol_maybe(_pos: ptrdiff_t, pos_byte: ptrdiff_t,
    cache: ts_linecol) -> ts_linecol {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if NILP(BVAR!(current_buffer!(), ts_parser_list_)) || !treesit_buf_tracks_linecol_p(current_buffer!()) {
        return TREESIT_EMPTY_LINECOL;
    }
    treesit_linecol_of_pos(pos_byte, cache)

  }
}

#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn treesit_record_change(start_byte: ptrdiff_t, old_end_byte: ptrdiff_t,
    new_end_byte: ptrdiff_t, start_linecol: ts_linecol,
    old_end_linecol: ts_linecol, new_end: ptrdiff_t) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let new_end_linecol = treesit_linecol_maybe(new_end, new_end_byte, start_linecol);
    treesit_record_change_1(start_byte, old_end_byte, new_end_byte,
        start_linecol, old_end_linecol, new_end_linecol);
    if new_end_linecol.bytepos != 0 {
        let new_begv_linecol = compute_new_linecol_by_change(BUF_TS_LINECOL_BEGV(current_buffer!()),
            start_linecol, old_end_linecol, new_end_linecol, BEGV_BYTE!());
        let new_zv_linecol = compute_new_linecol_by_change(BUF_TS_LINECOL_ZV(current_buffer!()),
            start_linecol, old_end_linecol, new_end_linecol, ZV_BYTE!());
        SET_BUF_TS_LINECOL_BEGV(current_buffer!(), new_begv_linecol);
        SET_BUF_TS_LINECOL_POINT(current_buffer!(), new_end_linecol);
        SET_BUF_TS_LINECOL_ZV(current_buffer!(), new_zv_linecol);
    }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_sync_visible_region(parser: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let p = XTS_PARSER(parser);
    let tree = (*p).tree;
    let buffer = XBUFFER((*p).buffer);
    let track_linecol = treesit_buf_tracks_linecol_p(buffer);
    if tree.is_null() {
        (*p).visible_beg = BUF_BEGV_BYTE(buffer);
        (*p).visible_end = BUF_ZV_BYTE(buffer);
        return;
    }
    let mut visible_beg = (*p).visible_beg;
    let mut visible_end = (*p).visible_end;
    eassert!(0 <= visible_beg);
    eassert!(visible_beg <= visible_end);
    eassert!(BUF_BEGV_BYTE(buffer) <= u32::MAX as ptrdiff_t);
    eassert!(BUF_ZV_BYTE(buffer) <= u32::MAX as ptrdiff_t);
    if visible_beg != BUF_BEGV_BYTE(buffer) || visible_end != BUF_ZV_BYTE(buffer) {
        (*p).need_reparse = true;
    }
    let mut visi_beg_linecol = if track_linecol { (*p).visi_beg_linecol } else { TREESIT_EMPTY_LINECOL };
    let mut visi_end_linecol = if track_linecol { (*p).visi_end_linecol } else { TREESIT_EMPTY_LINECOL };
    let buffer_begv_linecol = if track_linecol {
        treesit_linecol_of_pos(BUF_BEGV_BYTE(buffer), BUF_TS_LINECOL_BEGV(buffer))
    } else { TREESIT_EMPTY_LINECOL };
    let buffer_zv_linecol = if track_linecol {
        treesit_linecol_of_pos(BUF_ZV_BYTE(buffer), BUF_TS_LINECOL_ZV(buffer))
    } else { TREESIT_EMPTY_LINECOL };
    if track_linecol { eassert!(visi_beg_linecol.bytepos == visible_beg); }
    if visible_beg > BUF_BEGV_BYTE(buffer) {
        let point_new_end = if track_linecol {
            treesit_make_ts_point(buffer_begv_linecol, visi_beg_linecol)
        } else { TREESIT_TS_POINT_1_0 };
        treesit_tree_edit_1(tree, 0, 0, visible_beg - BUF_BEGV_BYTE(buffer),
            TREESIT_TS_POINT_1_0, TREESIT_TS_POINT_1_0, point_new_end);
        visible_beg = BUF_BEGV_BYTE(buffer);
        visi_beg_linecol = buffer_begv_linecol;
        eassert!(visible_beg <= visible_end);
    }
    if visible_end < BUF_ZV_BYTE(buffer) {
        let point_start = if track_linecol { treesit_make_ts_point(visi_beg_linecol, visi_end_linecol) }
            else { TREESIT_TS_POINT_1_0 };
        let point_new_end = if track_linecol { treesit_make_ts_point(visi_beg_linecol, buffer_zv_linecol) }
            else { TREESIT_TS_POINT_1_0 };
        treesit_tree_edit_1(tree, visible_end-visible_beg, visible_end-visible_beg,
            BUF_ZV_BYTE(buffer)-visible_beg, point_start, point_start, point_new_end);
        visible_end = BUF_ZV_BYTE(buffer);
        visi_end_linecol = buffer_zv_linecol;
        eassert!(visible_beg <= visible_end);
    } else if visible_end > BUF_ZV_BYTE(buffer) {
        let point_start = if track_linecol { treesit_make_ts_point(visi_beg_linecol, buffer_zv_linecol) }
            else { TREESIT_TS_POINT_1_0 };
        let point_old_end = if track_linecol { treesit_make_ts_point(visi_beg_linecol, visi_end_linecol) }
            else { TREESIT_TS_POINT_1_0 };
        treesit_tree_edit_1(tree, BUF_ZV_BYTE(buffer)-visible_beg, visible_end-visible_beg,
            BUF_ZV_BYTE(buffer)-visible_beg, point_start, point_old_end, point_start);
        visible_end = BUF_ZV_BYTE(buffer);
        visi_end_linecol = buffer_zv_linecol;
        eassert!(visible_beg <= visible_end);
    }
    if visible_beg < BUF_BEGV_BYTE(buffer) {
        let point_old_end = if track_linecol { treesit_make_ts_point(visi_beg_linecol, buffer_begv_linecol) }
            else { TREESIT_TS_POINT_1_0 };
        treesit_tree_edit_1(tree, 0, BUF_BEGV_BYTE(buffer)-visible_beg, 0,
            TREESIT_TS_POINT_1_0, point_old_end, TREESIT_TS_POINT_1_0);
        visible_beg = BUF_BEGV_BYTE(buffer);
        visi_beg_linecol = buffer_begv_linecol;
        eassert!(visible_beg <= visible_end);
    }
    eassert!(0 <= visible_beg);
    eassert!(visible_beg <= visible_end);
    eassert!(visible_beg == BUF_BEGV_BYTE(buffer));
    eassert!(visible_end == BUF_ZV_BYTE(buffer));
    (*p).visible_beg = visible_beg;
    (*p).visible_end = visible_end;
    (*p).visi_beg_linecol = visi_beg_linecol;
    (*p).visi_end_linecol = visi_end_linecol;
    if track_linecol {
        eassert!(visi_beg_linecol.bytepos == visible_beg);
        eassert!(visi_end_linecol.bytepos == visible_end);
    }
    // Clip character positions before converting to bytes: an old byte range
    // could now bisect a multibyte character after the user's edit.
    let mut lisp_ranges = (*p).last_set_ranges;
    if NILP(lisp_ranges) { return; }
    let mut new_ranges_head = lisp_ranges;
    let mut prev_cons = Qnil();
    FOR_EACH_TAIL_SAFE!(lisp_ranges, {
        let range = XCAR(lisp_ranges);
        let beg = XFIXNUM(XCAR(range));
        let end = XFIXNUM(XCDR(range));
        if end <= BUF_BEGV(buffer) {
            new_ranges_head = XCDR(new_ranges_head);
        } else if beg >= BUF_ZV(buffer) {
            if NILP(prev_cons) { new_ranges_head = Qnil(); }
            else { XSETCDR(prev_cons, Qnil()); }
            break;
        } else {
            if beg < BUF_BEGV(buffer) { XSETCAR(range, make_fixnum(BUF_BEGV(buffer))); }
            if end > BUF_ZV(buffer) { XSETCDR(range, make_fixnum(BUF_ZV(buffer))); }
        }
        prev_cons = lisp_ranges;
    });
    if NILP(new_ranges_head) {
        new_ranges_head = Fcons(Fcons(make_fixnum(BUF_BEGV(buffer)), make_fixnum(BUF_BEGV(buffer))), Qnil());
    }
    (*p).last_set_ranges = new_ranges_head;
    let mut len = 0;
    let ts_ranges = treesit_make_ts_ranges(new_ranges_head, parser, &raw mut len);
    let success = ts_parser_set_included_ranges((*p).parser, ts_ranges, len);
    xfree(ts_ranges.cast());
    eassert!(success);

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_check_buffer_size(buffer: *mut buffer) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let buffer_size_bytes = BUF_Z_BYTE!(buffer) - BUF_BEG_BYTE(buffer);
    if buffer_size_bytes > u32::MAX as ptrdiff_t {
        xsignal2(Qtreesit_buffer_too_large(), build_string(c"Buffer size cannot be larger than 4GB".as_ptr()),
            make_fixnum(buffer_size_bytes));
    }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_get_affected_ranges(old_tree: *mut TSTree, new_tree: *mut TSTree,
    parser: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let buf = XBUFFER((*XTS_PARSER(parser)).buffer);
    if !old_tree.is_null() {
        let mut len = 0;
        let ranges = ts_tree_get_changed_ranges(old_tree, new_tree, &raw mut len);
        let lisp_ranges = treesit_make_ranges(ranges, len, parser, buf);
        xfree(ranges.cast());
        lisp_ranges
    } else {
        let oldbuf = current_buffer!();
        set_buffer_internal(buf);
        let lisp_ranges = Fcons(Fcons(Fpoint_min(), Fpoint_max()), Qnil());
        set_buffer_internal(oldbuf);
        lisp_ranges
    }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_call_after_change_functions(parser: Lisp_Object, ranges: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let count = SPECPDL_INDEX();
    let mut functions = (*XTS_PARSER(parser)).after_change_functions;
    FOR_EACH_TAIL!(functions, { safe_calln!(XCAR(functions), ranges, parser); });
    unbind_to(count, Qnil());

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_ensure_parsed(parser: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let p = XTS_PARSER(parser);
    if (*p).within_reparse { return Qnil(); }
    (*p).within_reparse = true;
    let buffer = XBUFFER((*p).buffer);
    treesit_check_buffer_size(buffer);
    treesit_sync_visible_region(parser);
    if !(*p).need_reparse {
        (*p).within_reparse = false;
        return Qnil();
    }
    let treesit_parser = (*p).parser;
    let tree = (*p).tree;
    let input = (*p).input;
    let new_tree = ts_parser_parse(treesit_parser, tree, input);
    if new_tree.is_null() {
        let mut buf = Qnil();
        XSETBUFFER!(buf, buffer);
        xsignal1(Qtreesit_parse_error(), buf);
    }
    (*p).tree = new_tree;
    (*p).need_reparse = false;
    (*p).timestamp = (*p).timestamp.wrapping_add(1);
    let ranges = treesit_get_affected_ranges(tree, new_tree, parser);
    treesit_call_after_change_functions(parser, ranges);
    ts_tree_delete(tree);
    (*p).within_reparse = false;
    ranges

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe extern "C" fn treesit_read_buffer(parser: *mut c_void, byte_index: u32,
    _position: TSPoint, bytes_read: *mut u32) -> *const c_char {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let parser = parser.cast::<Lisp_TS_Parser>();
    let buffer = XBUFFER((*parser).buffer);
    let mut visible_beg = (*parser).visible_beg;
    let mut visible_end = (*parser).visible_end;
    let byte_pos = byte_index as ptrdiff_t + visible_beg;
    // The original checking assertions contain assignments. Their side effects
    // occur only when checking actually evaluates the assertion condition.
    eassert!({ visible_beg = BUF_BEGV_BYTE(buffer); visible_beg != 0 });
    eassert!({ visible_end = BUF_ZV_BYTE(buffer); visible_end != 0 });
    let (beg, len) = if !BUFFER_LIVE_P(buffer) {
        (core::ptr::null_mut(), 0)
    } else if byte_pos >= visible_end {
        (core::ptr::null_mut(), 0)
    } else {
        let beg = BUF_BYTE_ADDRESS(buffer, byte_pos).cast::<c_char>();
        (beg, BYTES_BY_CHAR_HEAD(*beg as c_int))
    };
    eassert!((len as u64) < u32::MAX as u64);
    *bytes_read = len as u32;
    beg

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn make_treesit_parser(buffer: Lisp_Object, parser: *mut TSParser,
    tree: *mut TSTree, language_symbol: Lisp_Object, tag: Lisp_Object,
    tracks_linecol: bool) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let lisp_parser = ALLOCATE_PSEUDOVECTOR!(Lisp_TS_Parser, buffer, pvec_type::PVEC_TS_PARSER);
    (*lisp_parser).language_symbol = language_symbol;
    (*lisp_parser).after_change_functions = Qnil();
    (*lisp_parser).tag = tag;
    (*lisp_parser).last_set_ranges = Qnil();
    (*lisp_parser).embed_level = Qnil();
    (*lisp_parser).buffer = buffer;
    (*lisp_parser).parser = parser;
    (*lisp_parser).tree = tree;
    // The stable three-field initializer zeroes SDK extensions as C does.
    (*lisp_parser).input = TSInput { payload: lisp_parser.cast(), read: Some(treesit_read_buffer),
        encoding: TSInputEncodingUTF8, decode: None };
    (*lisp_parser).need_reparse = true;
    (*lisp_parser).visible_beg = BUF_BEGV_BYTE(XBUFFER(buffer));
    (*lisp_parser).visible_end = BUF_ZV_BYTE(XBUFFER(buffer));
    (*lisp_parser).timestamp = 0;
    (*lisp_parser).deleted = false;
    (*lisp_parser).need_to_gc_buffer = false;
    (*lisp_parser).within_reparse = false;
    eassert!((*lisp_parser).visible_beg <= (*lisp_parser).visible_end);
    if tracks_linecol {
        let old_buf = current_buffer!();
        set_buffer_internal(XBUFFER(buffer));
        (*lisp_parser).visi_beg_linecol = treesit_linecol_of_pos(BEGV_BYTE!(), TREESIT_BOB_LINECOL);
        (*lisp_parser).visi_end_linecol = treesit_linecol_of_pos(ZV_BYTE!(), (*lisp_parser).visi_beg_linecol);
        set_buffer_internal(old_buf);
    } else {
        (*lisp_parser).visi_beg_linecol = TREESIT_EMPTY_LINECOL;
        (*lisp_parser).visi_end_linecol = TREESIT_EMPTY_LINECOL;
    }
    make_lisp_ptr(lisp_parser.cast(), Lisp_Type::Lisp_Vectorlike)

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn make_treesit_node(parser: Lisp_Object, node: TSNode) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let lisp_node = ALLOCATE_PSEUDOVECTOR!(Lisp_TS_Node, parser, pvec_type::PVEC_TS_NODE);
    (*lisp_node).parser = parser;
    (*lisp_node).node = node;
    (*lisp_node).timestamp = (*XTS_PARSER(parser)).timestamp;
    make_lisp_ptr(lisp_node.cast(), Lisp_Type::Lisp_Vectorlike)

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn make_treesit_query(query: Lisp_Object, language: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let lisp_query = ALLOCATE_PSEUDOVECTOR!(Lisp_TS_Query, source, pvec_type::PVEC_TS_COMPILED_QUERY);
    (*lisp_query).language = language;
    (*lisp_query).source = query;
    (*lisp_query).query = core::ptr::null_mut();
    (*lisp_query).cursor = core::ptr::null_mut();
    make_lisp_ptr(lisp_query.cast(), Lisp_Type::Lisp_Vectorlike)

  }
}

#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn treesit_delete_parser(lisp_parser: *mut Lisp_TS_Parser) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if (*lisp_parser).need_to_gc_buffer { Fkill_buffer((*lisp_parser).buffer); }
    ts_tree_delete((*lisp_parser).tree);
    ts_parser_delete((*lisp_parser).parser);

  }
}

#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn treesit_delete_query(lisp_query: *mut Lisp_TS_Query) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if !(*lisp_query).query.is_null() { ts_query_delete((*lisp_query).query); }
    if !(*lisp_query).cursor.is_null() { ts_query_cursor_delete((*lisp_query).cursor); }

  }
}

#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn treesit_named_node_p(node: TSNode) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { ts_node_is_named(node) 
  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_query_error_to_string(error: TSQueryError) -> *const c_char {
    match error {
        TSQueryErrorNone => c"None".as_ptr(),
        TSQueryErrorSyntax => c"Syntax error at".as_ptr(),
        TSQueryErrorNodeType => c"Node type error at".as_ptr(),
        TSQueryErrorField => c"Field error at".as_ptr(),
        TSQueryErrorCapture => c"Capture error at".as_ptr(),
        TSQueryErrorStructure => c"Structure error at".as_ptr(),
        _ => c"Unknown error".as_ptr(),
    }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_compose_query_signal_data(error_offset: u32, error_type: TSQueryError,
    query_source: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    list4(build_string(treesit_query_error_to_string(error_type)),
        make_fixnum(error_offset.wrapping_add(1) as EMACS_INT), query_source,
        build_string(c"Debug the query with `treesit-query-validate'".as_ptr()))

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_ensure_query_cursor(query: Lisp_Object) -> *mut TSQueryCursor {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let q = XTS_COMPILED_QUERY(query);
    if (*q).cursor.is_null() { (*q).cursor = ts_query_cursor_new(); }
    (*q).cursor

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_ensure_query_compiled(query: Lisp_Object, signal_symbol: *mut Lisp_Object,
    signal_data: *mut Lisp_Object) -> *mut TSQuery {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut treesit_query = (*XTS_COMPILED_QUERY(query)).query;
    if !treesit_query.is_null() { return treesit_query; }
    let mut source = (*XTS_COMPILED_QUERY(query)).source;
    let language = (*XTS_COMPILED_QUERY(query)).language;
    let remapped_lang = resolve_language_symbol(language);
    if !SYMBOLP(remapped_lang) {
        *signal_symbol = Qtreesit_query_error();
        *signal_data = list2(build_string(c"Invalid language symbol".as_ptr()), remapped_lang);
        return core::ptr::null_mut();
    }
    let lang = treesit_load_language(remapped_lang, signal_symbol, signal_data);
    let treesit_lang = lang.lang;
    if treesit_lang.is_null() { return core::ptr::null_mut(); }
    if CONSP(source) { source = Ftreesit_query_expand(source); }
    let mut error_offset = 0;
    let mut error_type = TSQueryErrorNone;
    treesit_query = ts_query_new(treesit_lang, SSDATA(source), SBYTES(source) as u32,
        &raw mut error_offset, &raw mut error_type);
    if treesit_query.is_null() {
        *signal_symbol = Qtreesit_query_error();
        *signal_data = treesit_compose_query_signal_data(error_offset, error_type, source);
    }
    (*XTS_COMPILED_QUERY(query)).query = treesit_query;
    treesit_query

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_ensure_query_compiled_signal(lisp_query: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut signal_symbol = Qnil();
    let mut signal_data = Qnil();
    let treesit_query = treesit_ensure_query_compiled(lisp_query, &raw mut signal_symbol, &raw mut signal_data);
    if treesit_query.is_null() { xsignal(signal_symbol, signal_data); }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_pred_with_guard(pred: Lisp_Object, node: Lisp_Object, parser: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let pre_timestamp = (*XTS_PARSER(parser)).timestamp;
    let res = calln!(pred, node);
    if (*XTS_PARSER(parser)).timestamp != pre_timestamp {
        xsignal(Qtreesit_buffer_changed(), list1(pred));
    }
    res

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_parser_live_p(parser: Lisp_Object) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    CHECK_TS_PARSER(parser);
    !(*XTS_PARSER(parser)).deleted && !NILP(Fbuffer_live_p((*XTS_PARSER(parser)).buffer))

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_check_range_argument(ranges: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let buffer = current_buffer!();
    let point_min = BUF_BEGV(buffer);
    let point_max = BUF_ZV(buffer);
    let mut last_point = point_min;
    let mut tail = ranges;
    CHECK_LIST(tail);
    FOR_EACH_TAIL!(tail, {
        CHECK_CONS(tail);
        let range = XCAR(tail);
        CHECK_CONS(range);
        CHECK_FIXNUM(XCAR(range));
        CHECK_FIXNUM(XCDR(range));
        let beg = XFIXNUM(XCAR(range));
        let end = XFIXNUM(XCDR(range));
        if !(last_point <= beg && beg <= end && end <= point_max) {
            xsignal2(Qtreesit_range_invalid(),
                build_string(c"RANGE is either overlapping, out-of-order or out-of-range".as_ptr()), ranges);
        }
        last_point = end;
    });
    CHECK_LIST_END(tail, ranges);

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_make_ranges(ranges: *const TSRange, len: u32, parser: Lisp_Object,
    buffer: *mut buffer) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut list = Qnil();
    let mut idx: c_int = 0;
    while (idx as u32) < len {
        let range = *ranges.offset(idx as isize);
        let beg_byte = (range.start_byte as ptrdiff_t + (*XTS_PARSER(parser)).visible_beg) as u32;
        let end_byte = (range.end_byte as ptrdiff_t + (*XTS_PARSER(parser)).visible_beg) as u32;
        eassert!(BUF_BEGV_BYTE(buffer) <= beg_byte as ptrdiff_t);
        eassert!(beg_byte <= end_byte);
        eassert!((end_byte as ptrdiff_t) <= BUF_ZV_BYTE(buffer));
        let lisp_range = Fcons(make_fixnum(buf_bytepos_to_charpos(buffer.cast(), beg_byte as ptrdiff_t)),
            make_fixnum(buf_bytepos_to_charpos(buffer.cast(), end_byte as ptrdiff_t)));
        list = Fcons(lisp_range, list);
        idx = idx.wrapping_add(1);
    }
    Fnreverse(list)

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_make_ts_ranges(mut ranges: Lisp_Object, parser: Lisp_Object, len: *mut u32) -> *mut TSRange {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let ranges_len = list_length(ranges);
    if ranges_len > u32::MAX as ptrdiff_t {
        xsignal(Qargs_out_of_range(), list2(ranges, Flength(ranges)));
    }
    *len = ranges_len as u32;
    let treesit_ranges = xmalloc(core::mem::size_of::<TSRange>() * ranges_len as usize).cast::<TSRange>();
    let buffer = XBUFFER((*XTS_PARSER(parser)).buffer);
    let mut idx: c_int = 0;
    while (idx as ptrdiff_t) < ranges_len {
        let range = XCAR(ranges);
        let beg_byte = buf_charpos_to_bytepos(buffer.cast(), XFIXNUM(XCAR(range)));
        let end_byte = buf_charpos_to_bytepos(buffer.cast(), XFIXNUM(XCDR(range)));
        eassert!(beg_byte - BUF_BEGV_BYTE(buffer) <= u32::MAX as ptrdiff_t);
        eassert!(end_byte - BUF_BEGV_BYTE(buffer) <= u32::MAX as ptrdiff_t);
        let rg = TSRange { start_point: TSPoint { row: 0, column: 0 },
            end_point: TSPoint { row: 0, column: 0 },
            start_byte: (beg_byte as u32 as ptrdiff_t - (*XTS_PARSER(parser)).visible_beg) as u32,
            end_byte: (end_byte as u32 as ptrdiff_t - (*XTS_PARSER(parser)).visible_beg) as u32 };
        *treesit_ranges.offset(idx as isize) = rg;
        idx = idx.wrapping_add(1);
        ranges = XCDR(ranges);
    }
    treesit_ranges

  }
}


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-p", Ftreesit_parser_p, Streesit_parser_p, 1, 1, 0,
  doc: "Return t if OBJECT is a tree-sitter parser.",
  (object: Lisp_Object),
  {
return if TS_PARSERP(object) { Qt() } else { Qnil() };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-p", Ftreesit_node_p, Streesit_node_p, 1, 1, 0,
  doc: "Return t if OBJECT is a tree-sitter node.",
  (object: Lisp_Object),
  {
return if TS_NODEP(object) { Qt() } else { Qnil() };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-compiled-query-p", Ftreesit_compiled_query_p, Streesit_compiled_query_p, 1, 1, 0,
  doc: "Return t if OBJECT is a compiled tree-sitter query.",
  (object: Lisp_Object),
  {
return if TS_COMPILED_QUERY_P(object) { Qt() } else { Qnil() };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-query-p", Ftreesit_query_p, Streesit_query_p, 1, 1, 0,
  doc: "Return t if OBJECT is a generic tree-sitter query.",
  (object: Lisp_Object),
  {
return if TS_COMPILED_QUERY_P(object) || CONSP(object) || STRINGP(object) { Qt() } else { Qnil() };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-query-eagerly-compiled-p", Ftreesit_query_eagerly_compiled_p, Streesit_query_eagerly_compiled_p, 1, 1, 0,
  doc: "Return non-nil if QUERY is eagerly compiled.\n\nQUERY has to be a compiled query.  Compiled queries are lazily compiled\nby default, meaning they are not actually compiled until first used.\nReturn non-nil if QUERY is actually compiled (either by passing the\nEAGER flag to `treesit-query-compile' or due to the fact that it was\nalready used).",
  (query: Lisp_Object),
  {
CHECK_TS_COMPILED_QUERY(query);return if (*XTS_COMPILED_QUERY(query)).query.is_null() { Qnil() } else { Qt() };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-query-language", Ftreesit_query_language, Streesit_query_language, 1, 1, 0,
  doc: "Return the language of QUERY.\nQUERY has to be a compiled query.",
  (query: Lisp_Object),
  {
CHECK_TS_COMPILED_QUERY(query);return (*XTS_COMPILED_QUERY(query)).language;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-query-source", Ftreesit_query_source, Streesit_query_source, 1, 1, 0,
  doc: "Return the (string or sexp) source of QUERY.\nQUERY has to be a compiled query.",
  (query: Lisp_Object),
  {
CHECK_TS_COMPILED_QUERY(query);return (*XTS_COMPILED_QUERY(query)).source;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-parser", Ftreesit_node_parser, Streesit_node_parser, 1, 1, 0,
  doc: "Return the parser to which NODE belongs.",
  (node: Lisp_Object),
  {
CHECK_TS_NODE(node);return (*XTS_NODE(node)).parser;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-delete", Ftreesit_parser_delete, Streesit_parser_delete, 1, 1, 0,
  doc: "Delete PARSER from its buffer's parser list.\nSee `treesit-parser-list' for the buffer's parser list.",
  (parser: Lisp_Object),
  {
treesit_check_parser(parser);
let buffer=(*XTS_PARSER(parser)).buffer;let buf=XBUFFER(buffer);
BVAR!(buf,ts_parser_list_)=Fdelete(parser,BVAR!(buf,ts_parser_list_));
(*XTS_PARSER(parser)).deleted=true;return Qnil();
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-buffer", Ftreesit_parser_buffer, Streesit_parser_buffer, 1, 1, 0,
  doc: "Return the buffer of PARSER.",
  (parser: Lisp_Object),
  {
treesit_check_parser(parser);let mut buf=Qnil();
XSETBUFFER!(buf,XBUFFER((*XTS_PARSER(parser)).buffer));return buf;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-language", Ftreesit_parser_language, Streesit_parser_language, 1, 1, 0,
  doc: "Return PARSER's language symbol.\nThis symbol is the one used to create the parser.",
  (parser: Lisp_Object),
  {
treesit_check_parser(parser);return (*XTS_PARSER(parser)).language_symbol;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-tag", Ftreesit_parser_tag, Streesit_parser_tag, 1, 1, 0,
  doc: "Return PARSER's tag.",
  (parser: Lisp_Object),
  {
treesit_check_parser(parser);return (*XTS_PARSER(parser)).tag;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-embed-level", Ftreesit_parser_embed_level, Streesit_parser_embed_level, 1, 1, 0,
  doc: "Return PARSER's embed level.\n\nThe embed level can be either nil or a non-negative integer.  A value of\nnil means the parser isn't part of the embedded parser tree.  The\nprimary parser has embed level 0, and each additional layer of parser\nembedding increments the embed level by 1.",
  (parser: Lisp_Object),
  {
treesit_check_parser(parser);return (*XTS_PARSER(parser)).embed_level;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-set-embed-level", Ftreesit_parser_set_embed_level, Streesit_parser_set_embed_level, 2, 2, 0,
  doc: "Set the embed level for PARSER to LEVEL.\nLEVEL can be nil, for a parser that is not part of an embedded parser\ntree; otherwise it must be a non-negative integer.",
  (parser: Lisp_Object, level: Lisp_Object),
  {
treesit_check_parser(parser);
if !NILP(level) { CHECK_NUMBER(level);if XFIXNUM(level)<0 { xsignal(Qargs_out_of_range(),list1(level)); } }
(*XTS_PARSER(parser)).embed_level=level;return level;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-root-node", Ftreesit_parser_root_node, Streesit_parser_root_node, 1, 1, 0,
  doc: "Return the root node of PARSER.",
  (parser: Lisp_Object),
  {
treesit_check_parser(parser);treesit_initialize();treesit_ensure_parsed(parser);
let root_node=ts_tree_root_node((*XTS_PARSER(parser)).tree);return make_treesit_node(parser,root_node);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-create", Ftreesit_parser_create, Streesit_parser_create, 1, 4, 0,
  doc: "Create and return a parser in BUFFER for LANGUAGE with TAG.\n\nThe parser is automatically added to BUFFER's parser list, as returned\nby `treesit-parser-list'.  LANGUAGE is a language symbol.  If BUFFER\nis nil or omitted, it defaults to the current buffer.  If BUFFER\nalready has a parser for LANGUAGE with TAG, return that parser, but if\nNO-REUSE is non-nil, always create a new parser.\n\nTAG can be any symbol except t, and defaults to nil.  Different\nparsers can have the same tag.\n\nIf that buffer is an indirect buffer, its base buffer is used instead.\nThat is, indirect buffers use their base buffer's parsers.  Lisp\nprograms should widen as necessary should they want to use a parser in\nan indirect buffer.",
  (language: Lisp_Object, buffer: Lisp_Object, no_reuse: Lisp_Object, tag: Lisp_Object),
  {
treesit_initialize();CHECK_SYMBOL(language);CHECK_SYMBOL(tag);
let mut buf;let mut buf_orig=Qnil();
if NILP(buffer) { buf=current_buffer!();XSETBUFFER!(buf_orig,current_buffer!()); }
else { CHECK_BUFFER(buffer);buf=XBUFFER(buffer);buf_orig=buffer; }
if !(*buf).base_buffer.is_null() { buf=(*buf).base_buffer; }
if EQ(tag,Qt()) { wrong_type_argument(list2(Qnot(),Qt()),Qt()); }
treesit_check_buffer_size(buf);
let remapped_lang=resolve_language_symbol(language);CHECK_SYMBOL(remapped_lang);
if NILP(no_reuse) {
 let mut tail=BVAR!(buf,ts_parser_list_);
 FOR_EACH_TAIL!(tail, {
  let parser=XTS_PARSER(XCAR(tail));
  if EQ((*parser).tag,tag) && EQ((*parser).language_symbol,language) && EQ((*parser).buffer,buf_orig) { return XCAR(tail); }
 });
}
let mut signal_symbol=Qnil();let mut signal_data=Qnil();let parser=ts_parser_new();
let loaded_lang=treesit_load_language(remapped_lang,&raw mut signal_symbol,&raw mut signal_data);
let lang=loaded_lang.lang;if lang.is_null() { xsignal(signal_symbol,signal_data); }
ts_parser_set_language(parser,lang);
let lang_need_linecol_tracking=!NILP(Fmemq(remapped_lang,(*Vtreesit_languages_require_line_column_tracking.as_mut_ptr ())));
let lisp_parser=make_treesit_parser(buf_orig,parser,core::ptr::null_mut(),language,tag,lang_need_linecol_tracking);
if lang_need_linecol_tracking && !treesit_buf_tracks_linecol_p(buf) {
 SET_BUF_TS_LINECOL_BEGV(buf,TREESIT_BOB_LINECOL);SET_BUF_TS_LINECOL_POINT(buf,TREESIT_BOB_LINECOL);SET_BUF_TS_LINECOL_ZV(buf,TREESIT_BOB_LINECOL);
}
BVAR!(buf,ts_parser_list_)=Fcons(lisp_parser,BVAR!(buf,ts_parser_list_));return lisp_parser;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-list", Ftreesit_parser_list, Streesit_parser_list, 0, 3, 0,
  doc: "Return BUFFER's parser list, filtered by LANGUAGE and TAG.\n\nBUFFER defaults to the current buffer.  If that buffer is an indirect\nbuffer, its base buffer is used instead.  That is, indirect buffers\nuse their base buffer's parsers.\n\nIf LANGUAGE is non-nil, only return parsers for that language.\n\nThe returned list only contain parsers with TAG.  TAG defaults to nil.\nIf TAG is t, include parsers in the returned list regardless of their\ntag.",
  (buffer: Lisp_Object, language: Lisp_Object, tag: Lisp_Object),
  {
let mut buf;let mut buf_orig=Qnil();
if NILP(buffer) { buf=current_buffer!();XSETBUFFER!(buf_orig,current_buffer!()); }
else { CHECK_BUFFER(buffer);buf=XBUFFER(buffer);buf_orig=buffer; }
if !(*buf).base_buffer.is_null() { buf=(*buf).base_buffer; }
let mut return_list=Qnil();let mut tail=BVAR!(buf,ts_parser_list_);
FOR_EACH_TAIL!(tail, {
 let parser=XTS_PARSER(XCAR(tail));
 if (NILP(language)||EQ(language,(*parser).language_symbol)) && (EQ(tag,Qt())||EQ(tag,(*parser).tag)) && EQ((*parser).buffer,buf_orig) { return_list=Fcons(XCAR(tail),return_list); }
});return Freverse(return_list);
});


#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_check_node(obj:Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 CHECK_TS_NODE(obj);
 if !treesit_node_uptodate_p(obj) { xsignal1(Qtreesit_node_outdated(),obj); }
 if !treesit_node_buffer_live_p(obj) { xsignal1(Qtreesit_node_buffer_killed(),obj); }

  }
}


#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_check_position(obj:Lisp_Object,buf:*mut buffer) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 let pos=fix_position(obj);if pos<BUF_BEGV(buf)||pos>BUF_ZV(buf) { xsignal1(Qargs_out_of_range(),obj); }

  }
}


#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn treesit_node_uptodate_p(obj:Lisp_Object)->bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 let lisp_parser=(*XTS_NODE(obj)).parser;
 return (*XTS_NODE(obj)).timestamp==(*XTS_PARSER(lisp_parser)).timestamp;

  }
}


#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn treesit_node_buffer_live_p(obj:Lisp_Object)->bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 let buffer=XBUFFER((*XTS_PARSER((*XTS_NODE(obj)).parser)).buffer);return BUFFER_LIVE_P(buffer);

  }
}


#[cfg(feature = "have_tree_sitter")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn treesit_node_eq(node1:Lisp_Object,node2:Lisp_Object)->bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 treesit_initialize();let treesit_node_1=(*XTS_NODE(node1)).node;let treesit_node_2=(*XTS_NODE(node2)).node;
 return ts_node_eq(treesit_node_1,treesit_node_2);

  }
}


#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_cursor_first_child_for_byte(cursor:*mut TSTreeCursor,pos:ptrdiff_t,named:bool)->bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
 if ts_tree_cursor_goto_first_child_for_byte(cursor,pos as u32)==-1 && !ts_tree_cursor_goto_first_child(cursor) { return false; }
 let mut node=ts_tree_cursor_current_node(cursor);
 while (ts_node_end_byte(node) as ptrdiff_t) <= pos {
  if ts_tree_cursor_goto_next_sibling(cursor) { node=ts_tree_cursor_current_node(cursor); } else { return false; }
 }
 while named && !ts_node_is_named(node) {
  if ts_tree_cursor_goto_next_sibling(cursor) { node=ts_tree_cursor_current_node(cursor); } else { return false; }
 }
 return true;

  }
}


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-set-included-ranges", Ftreesit_parser_set_included_ranges, Streesit_parser_set_included_ranges, 2, 2, 0,
  doc: "Limit PARSER to RANGES.\n\nRANGES is a list of (BEG . END), each (BEG . END) defines a region in\nwhich the parser should operate.  Regions must not overlap, and the\nregions should come in order in the list.  Signal\n`treesit-set-range-error' if the argument is invalid, or something\nelse went wrong.  If RANGES is nil, the PARSER is to parse the whole\nbuffer.\n\nDO NOT modify RANGES after passing it to this function, as RANGES is\nsaved to PARSER internally.",
  (parser: Lisp_Object, ranges: Lisp_Object),
  {
treesit_check_parser(parser);if !NILP(ranges) { CHECK_CONS(ranges); }
if !NILP(Fequal((*XTS_PARSER(parser)).last_set_ranges,ranges)) { return Qnil(); }
treesit_check_range_argument(ranges);treesit_initialize();
treesit_check_buffer_size(XBUFFER((*XTS_PARSER(parser)).buffer));treesit_sync_visible_region(parser);
(*XTS_PARSER(parser)).last_set_ranges=ranges;
let success;
if NILP(ranges) { success=ts_parser_set_included_ranges((*XTS_PARSER(parser)).parser,core::ptr::null(),0); }
else { let mut len=0;let treesit_ranges=treesit_make_ts_ranges(ranges,parser,&raw mut len);
 success=ts_parser_set_included_ranges((*XTS_PARSER(parser)).parser,treesit_ranges,len);xfree(treesit_ranges.cast()); }
if !success { xsignal2(Qtreesit_range_invalid(),build_string(c"Something went wrong when setting ranges".as_ptr()),ranges); }
(*XTS_PARSER(parser)).need_reparse=true;return Qnil();
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-included-ranges", Ftreesit_parser_included_ranges, Streesit_parser_included_ranges, 1, 1, 0,
  doc: "Return the ranges set for PARSER.\nIf no ranges are set for PARSER, return nil.\nSee also `treesit-parser-set-included-ranges'.",
  (parser: Lisp_Object),
  {
treesit_check_parser(parser);treesit_initialize();treesit_sync_visible_region(parser);return (*XTS_PARSER(parser)).last_set_ranges;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-notifiers", Ftreesit_parser_notifiers, Streesit_parser_notifiers, 1, 1, 0,
  doc: "Return the list of after-change notifier functions for PARSER.",
  (parser: Lisp_Object),
  {
treesit_check_parser(parser);let mut return_list=Qnil();let mut tail=(*XTS_PARSER(parser)).after_change_functions;
FOR_EACH_TAIL!(tail, { return_list=Fcons(XCAR(tail),return_list); });return return_list;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-add-notifier", Ftreesit_parser_add_notifier, Streesit_parser_add_notifier, 2, 2, 0,
  doc: "Add FUNCTION to the list of PARSER's after-change notifiers.\nFUNCTION must be a function symbol, rather than a lambda form.\nFUNCTION should take 2 arguments, RANGES and PARSER.  RANGES is a list\nof cons cells of the form (START . END), where START and END are buffer\npositions.  PARSER is the parser issuing the notification.",
  (parser: Lisp_Object, function: Lisp_Object),
  {
treesit_check_parser(parser);CHECK_SYMBOL(function);
let functions=(*XTS_PARSER(parser)).after_change_functions;
if NILP(Fmemq(function,functions)) { (*XTS_PARSER(parser)).after_change_functions=Fcons(function,functions); }return Qnil();
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-remove-notifier", Ftreesit_parser_remove_notifier, Streesit_parser_remove_notifier, 2, 2, 0,
  doc: "Remove FUNCTION from the list of PARSER's after-change notifiers.\n  FUNCTION must be a function symbol, rather than a lambda form.\nFUNCTION should take 2 arguments, RANGES and PARSER.  RANGES is a list\nof cons of the form (START . END), where START and END are buffer\npositions.  PARSER is the parser issuing the notification.",
  (parser: Lisp_Object, function: Lisp_Object),
  {
treesit_check_parser(parser);CHECK_SYMBOL(function);
let functions=(*XTS_PARSER(parser)).after_change_functions;
if !NILP(Fmemq(function,functions)) { (*XTS_PARSER(parser)).after_change_functions=Fdelq(function,functions); }return Qnil();
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parse-string", Ftreesit_parse_string, Streesit_parse_string, 2, 2, 0,
  doc: "Parse STRING using a parser for LANGUAGE.\n\nReturn the root node of the result parse tree.  DO NOT use this function\nin a loop: this function is intended for one-off use and isn't\noptimized; for heavy workload, use a temporary buffer instead.",
  (string: Lisp_Object, language: Lisp_Object),
  {
CHECK_SYMBOL(language);CHECK_STRING(string);
let name_str=build_string(c" *treesit-parse-string*".as_ptr());let buffer_name=Fgenerate_new_buffer_name(name_str,Qnil());
let buffer=Fget_buffer_create(buffer_name,Qnil());let old_buffer=current_buffer!();
set_buffer_internal(XBUFFER(buffer));insert1(string);set_buffer_internal(old_buffer);
let parser=Ftreesit_parser_create(language,buffer,Qt(),Qnil());(*XTS_PARSER(parser)).need_to_gc_buffer=true;
BVAR!(XBUFFER(buffer),ts_parser_list_)=Qnil();return Ftreesit_parser_root_node(parser);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-parser-changed-regions", Ftreesit_parser_changed_regions, Streesit_parser_changed_regions, 1, 1, 0,
  doc: "Force PARSER to re-parse and return the affected regions.\n\nReturn ranges as a list of (BEG . END).  If there's no need to re-parse\nor no affected ranges, return nil.",
  (parser: Lisp_Object),
  {
treesit_check_parser(parser);treesit_initialize();return treesit_ensure_parsed(parser);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-type", Ftreesit_node_type, Streesit_node_type, 1, 1, 0,
  doc: "Return the NODE's type as a string.\nIf NODE is nil, return nil.",
  (node: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;let type_=ts_node_type(treesit_node);
return if type_.is_null() { (*Vtreesit_str_empty.as_mut_ptr ()) } else { build_string(type_) };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-string", Ftreesit_node_string, Streesit_node_string, 1, 1, 0,
  doc: "Return the string representation of NODE.\nIf NODE is nil, return nil.",
  (node: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;let string=ts_node_string(treesit_node);return build_string(string);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-parent", Ftreesit_node_parent, Streesit_node_parent, 1, 1, 0,
  doc: "Return the immediate parent of NODE.\nReturn nil if NODE has no parent.  If NODE is nil, return nil.",
  (node: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);treesit_initialize();
let mut return_value=Qnil();let treesit_node=(*XTS_NODE(node)).node;let parser=(*XTS_NODE(node)).parser;
let mut cursor=core::mem::MaybeUninit::<TSTreeCursor>::uninit();
if !treesit_cursor_helper(cursor.as_mut_ptr(),treesit_node,parser) { return return_value; }
let mut cursor=cursor.assume_init();
if ts_tree_cursor_goto_parent(&raw mut cursor) { let parent=ts_tree_cursor_current_node(&raw mut cursor);return_value=make_treesit_node(parser,parent); }
ts_tree_cursor_delete(&raw mut cursor);return return_value;
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-child", Ftreesit_node_child, Streesit_node_child, 2, 3, 0,
  doc: "Return the Nth child of NODE.\n\nReturn nil if there is no Nth child.  If NAMED is non-nil, look for\nnamed child only.  NAMED defaults to nil.  If NODE is nil, return\nnil.\n\nN could be negative, e.g., -1 represents the last child.",
  (node: Lisp_Object, n: Lisp_Object, named: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);CHECK_INTEGER(n);let mut idx=XFIXNUM(n);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;
if idx<0 { idx=if NILP(named) { ts_node_child_count(treesit_node) as EMACS_INT+idx } else { ts_node_named_child_count(treesit_node) as EMACS_INT+idx }; }
if idx<0 { return Qnil(); }if idx>u32::MAX as EMACS_INT { xsignal1(Qargs_out_of_range(),n); }
let child=if NILP(named) { ts_node_child(treesit_node,idx as u32) } else { ts_node_named_child(treesit_node,idx as u32) };
if ts_node_is_null(child) { return Qnil(); }return make_treesit_node((*XTS_NODE(node)).parser,child);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-check", Ftreesit_node_check, Streesit_node_check, 2, 2, 0,
  doc: "Return non-nil if NODE has PROPERTY, nil otherwise.\n\nPROPERTY could be `named', `missing', `extra', `outdated',\n`has-error', or `live'.\n\nNamed nodes correspond to named rules in the language definition,\nwhereas \"anonymous\" nodes correspond to string literals in the\nlanguage definition.\n\nMissing nodes are inserted by the parser in order to recover from\ncertain kinds of syntax errors, i.e., should be there but not there.\n\nExtra nodes represent things like comments, which are not required the\nlanguage definition, but can appear anywhere.\n\nA node is \"outdated\" if the parser has reparsed at least once after\nthe node was created.\n\nA node \"has error\" if itself is a syntax error or contains any syntax\nerrors.\n\nA node is \"live\" if its parser is not deleted and its buffer is\nlive.",
  (node: Lisp_Object, property: Lisp_Object),
  {
if NILP(node) { return Qnil(); }CHECK_TS_NODE(node);CHECK_SYMBOL(property);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;
if BASE_EQ(property,Qoutdated()) { return if treesit_node_uptodate_p(node) { Qnil() } else { Qt() }; }
treesit_check_node(node);
let result=if BASE_EQ(property,Qnamed()) { ts_node_is_named(treesit_node) }
else if BASE_EQ(property,Qmissing()) { ts_node_is_missing(treesit_node) }
else if BASE_EQ(property,Qextra()) { ts_node_is_extra(treesit_node) }
else if BASE_EQ(property,Qhas_error()) { ts_node_has_error(treesit_node) }
else if BASE_EQ(property,Qlive()) { treesit_parser_live_p((*XTS_NODE(node)).parser) }
else { signal_error(c"Expecting `named', `missing', `extra', `outdated', `has-error', or `live', but got".as_ptr(),property); };
return if result { Qt() } else { Qnil() };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-field-name-for-child", Ftreesit_node_field_name_for_child, Streesit_node_field_name_for_child, 2, 2, 0,
  doc: "Return the field name of the Nth child of NODE.\n\nReturn nil if there's no Nth child, or if it has no field.\nIf NODE is nil, return nil.\n\nN counts all children, i.e., named ones and anonymous ones.\n\nN could be negative, e.g., -1 represents the last child.",
  (node: Lisp_Object, n: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);CHECK_INTEGER(n);let mut idx=XFIXNUM(n);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;if idx<0 { idx=ts_node_child_count(treesit_node) as EMACS_INT+idx; }
if idx<0 { return Qnil(); }if idx>u32::MAX as EMACS_INT { xsignal1(Qargs_out_of_range(),n); }
let name=ts_node_field_name_for_child(treesit_node,idx as u32);if name.is_null() { return Qnil(); }return build_string(name);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-child-count", Ftreesit_node_child_count, Streesit_node_child_count, 1, 2, 0,
  doc: "Return the number of children of NODE.\n\nIf NAMED is non-nil, count named children only.  NAMED defaults to\nnil.  If NODE is nil, return nil.",
  (node: Lisp_Object, named: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);treesit_initialize();let treesit_node=(*XTS_NODE(node)).node;
let count=if NILP(named) { ts_node_child_count(treesit_node) } else { ts_node_named_child_count(treesit_node) };return make_fixnum(count as EMACS_INT);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-child-by-field-name", Ftreesit_node_child_by_field_name, Streesit_node_child_by_field_name, 2, 2, 0,
  doc: "Return the child of NODE with FIELD-NAME (a string).\nReturn nil if there is no such child.  If NODE is nil, return nil.",
  (node: Lisp_Object, field_name: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);CHECK_STRING(field_name);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;let child=ts_node_child_by_field_name(treesit_node,SSDATA(field_name),SBYTES(field_name) as u32);
if ts_node_is_null(child) { return Qnil(); }return make_treesit_node((*XTS_NODE(node)).parser,child);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-first-child-for-pos", Ftreesit_node_first_child_for_pos, Streesit_node_first_child_for_pos, 2, 3, 0,
  doc: "Return the first child of NODE for buffer position POS.\n\nSpecifically, return the first child that extends beyond POS.\nReturn nil if there is no such child.\nIf NAMED is non-nil, look for named children only.  NAMED defaults to nil.\nNote that this function returns an immediate child, not the smallest\n(grand)child.  If NODE is nil, return nil.",
  (node: Lisp_Object, pos: Lisp_Object, named: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);
let buf=XBUFFER((*XTS_PARSER((*XTS_NODE(node)).parser)).buffer);let visible_beg=(*XTS_PARSER((*XTS_NODE(node)).parser)).visible_beg;
treesit_check_position(pos,buf);treesit_initialize();let byte_pos=buf_charpos_to_bytepos(buf.cast(),fix_position(pos));
let treesit_node=(*XTS_NODE(node)).node;let mut cursor=ts_tree_cursor_new(treesit_node);let treesit_pos=byte_pos-visible_beg;
let success=treesit_cursor_first_child_for_byte(&raw mut cursor,treesit_pos,!NILP(named));let child=ts_tree_cursor_current_node(&raw mut cursor);
ts_tree_cursor_delete(&raw mut cursor);if !success { return Qnil(); }return make_treesit_node((*XTS_NODE(node)).parser,child);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-descendant-for-range", Ftreesit_node_descendant_for_range, Streesit_node_descendant_for_range, 3, 4, 0,
  doc: "Return the smallest node that covers buffer positions BEG to END.\n\nThe returned node is a descendant of NODE.\nReturn nil if there is no such node.\nIf NAMED is non-nil, look for named child only.  NAMED defaults to nil.\nIf NODE is nil, return nil.",
  (node: Lisp_Object, beg: Lisp_Object, end: Lisp_Object, named: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);
let buf=XBUFFER((*XTS_PARSER((*XTS_NODE(node)).parser)).buffer);let visible_beg=(*XTS_PARSER((*XTS_NODE(node)).parser)).visible_beg;
treesit_check_position(beg,buf);treesit_check_position(end,buf);treesit_initialize();
let byte_beg=buf_charpos_to_bytepos(buf.cast(),fix_position(beg));let byte_end=buf_charpos_to_bytepos(buf.cast(),fix_position(end));let treesit_node=(*XTS_NODE(node)).node;
let child=if NILP(named) { ts_node_descendant_for_byte_range(treesit_node,(byte_beg-visible_beg) as u32,(byte_end-visible_beg) as u32) }
else { ts_node_named_descendant_for_byte_range(treesit_node,(byte_beg-visible_beg) as u32,(byte_end-visible_beg) as u32) };
if ts_node_is_null(child) { return Qnil(); }return make_treesit_node((*XTS_NODE(node)).parser,child);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-eq", Ftreesit_node_eq, Streesit_node_eq, 2, 2, 0,
  doc: "Return non-nil if NODE1 and NODE2 refer to the same node.\nIf any one of NODE1 and NODE2 is nil, return nil.\nThis function uses the same equivalence metric as `equal', and returns\nnon-nil if NODE1 and NODE2 refer to the same node in a syntax tree\nproduced by tree-sitter.",
  (node1: Lisp_Object, node2: Lisp_Object),
  {
if NILP(node1)||NILP(node2) { return Qnil(); }CHECK_TS_NODE(node1);CHECK_TS_NODE(node2);
let same_node=treesit_node_eq(node1,node2);return if same_node { Qt() } else { Qnil() };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-start", Ftreesit_node_start, Streesit_node_start, 1, 1, 0,
  doc: "Return the NODE's start position in its buffer.\nIf NODE is nil, return nil.",
  (node: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;let visible_beg=(*XTS_PARSER((*XTS_NODE(node)).parser)).visible_beg;
let byte_offset=ts_node_start_byte(treesit_node);let buffer=XBUFFER((*XTS_PARSER((*XTS_NODE(node)).parser)).buffer);
let pos=buf_bytepos_to_charpos(buffer.cast(),byte_offset as ptrdiff_t+visible_beg);return make_fixnum(pos);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-end", Ftreesit_node_end, Streesit_node_end, 1, 1, 0,
  doc: "Return the NODE's end position in its buffer.\nIf NODE is nil, return nil.",
  (node: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;let visible_beg=(*XTS_PARSER((*XTS_NODE(node)).parser)).visible_beg;
let byte_offset=ts_node_end_byte(treesit_node);let buffer=XBUFFER((*XTS_PARSER((*XTS_NODE(node)).parser)).buffer);
let pos=buf_bytepos_to_charpos(buffer.cast(),byte_offset as ptrdiff_t+visible_beg);return make_fixnum(pos);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-next-sibling", Ftreesit_node_next_sibling, Streesit_node_next_sibling, 1, 2, 0,
  doc: "Return the next sibling of NODE.\n\nReturn nil if there is no next sibling.  If NAMED is non-nil, look for named\nsiblings only.  NAMED defaults to nil.  If NODE is nil, return nil.",
  (node: Lisp_Object, named: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;let sibling=if NILP(named) { ts_node_next_sibling(treesit_node) } else { ts_node_next_named_sibling(treesit_node) };
if ts_node_is_null(sibling) { return Qnil(); }return make_treesit_node((*XTS_NODE(node)).parser,sibling);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-prev-sibling", Ftreesit_node_prev_sibling, Streesit_node_prev_sibling, 1, 2, 0,
  doc: "Return the previous sibling of NODE.\n\nReturn nil if there is no previous sibling.  If NAMED is non-nil, look\nfor named siblings only.  NAMED defaults to nil.  If NODE is nil,\nreturn nil.",
  (node: Lisp_Object, named: Lisp_Object),
  {
if NILP(node) { return Qnil(); }treesit_check_node(node);treesit_initialize();
let treesit_node=(*XTS_NODE(node)).node;let sibling=if NILP(named) { ts_node_prev_sibling(treesit_node) } else { ts_node_prev_named_sibling(treesit_node) };
if ts_node_is_null(sibling) { return Qnil(); }return make_treesit_node((*XTS_NODE(node)).parser,sibling);
});

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_query_string_string(str_: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let nbytes = SBYTES(str_);
    let mut escapes: ptrdiff_t = 0;
    for i in 0..nbytes {
        let c = SREF(str_, i as usize);
        escapes += (c == 0 || c == b'\n' || c == b'\r' || c == b'\t' || c == b'"' || c == b'\\') as ptrdiff_t;
    }
    let nchars = SCHARS(str_);
    let extra = escapes + 2;
    let dst = if STRING_MULTIBYTE(str_) { make_uninit_multibyte_string(nchars+extra, nbytes+extra) }
        else { make_uninit_string(nbytes+extra) };
    let mut d = SDATA(dst);
    *d = b'"'; d = d.add(1);
    for i in 0..nbytes {
        let c = SREF(str_, i as usize);
        match c {
            0 | b'\n' | b'\r' | b'\t' => {
                *d = b'\\'; d = d.add(1);
                *d = match c { 0 => b'0', b'\n' => b'n', b'\r' => b'r', _ => b't' }; d = d.add(1);
            }
            b'"' | b'\\' => { *d = b'\\'; d = d.add(1); *d = c; d = d.add(1); }
            _ => { *d = c; d = d.add(1); }
        }
    }
    *d = b'"'; d = d.add(1);
    eassert!(d == SDATA(dst).offset(SBYTES(dst)));
    dst

  }
}

#[cfg(feature = "have_tree_sitter")]
#[repr(C)]
#[derive(Clone, Copy)]
struct capture_range { start: Lisp_Object, end: Lisp_Object }

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_predicates_for_pattern(query: *mut TSQuery, pattern_index: u32) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut len = 0;
    let predicate_list = ts_query_predicates_for_pattern(query, pattern_index, &raw mut len);
    let mut result = Qnil();
    let mut predicate = Qnil();
    let mut idx: c_int = 0;
    while (idx as u32) < len {
        let step = *predicate_list.offset(idx as isize);
        match step.r#type {
            TSQueryPredicateStepTypeCapture => {
                let mut str_len = 0;
                let str_ = ts_query_capture_name_for_id(query, step.value_id, &raw mut str_len);
                predicate = Fcons(intern_c_string_1(str_, str_len as ptrdiff_t), predicate);
            }
            TSQueryPredicateStepTypeString => {
                let mut str_len = 0;
                let str_ = ts_query_string_value_for_id(query, step.value_id, &raw mut str_len);
                predicate = Fcons(make_string(str_, str_len as ptrdiff_t), predicate);
            }
            TSQueryPredicateStepTypeDone => { result = Fcons(Fnreverse(predicate), result); predicate = Qnil(); }
            _ => {}
        }
        idx = idx.wrapping_add(1);
    }
    Fnreverse(result)

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_predicate_capture_name_to_node(name: Lisp_Object, captures: capture_range,
    node: *mut Lisp_Object, signal_data: *mut Lisp_Object) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    *node = Qnil();
    let mut tail = captures.start;
    while !EQ(tail, captures.end) {
        if EQ(XCAR(XCAR(tail)), name) { *node = XCDR(XCAR(tail)); break; }
        tail = XCDR(tail);
    }
    if NILP(*node) {
        *signal_data = list3(build_string(c"Cannot find captured node".as_ptr()), name,
            build_string(c"A predicate can only refer to captured nodes in the same pattern".as_ptr()));
        return false;
    }
    true

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_predicate_capture_name_to_text(name: Lisp_Object, captures: capture_range,
    text: *mut Lisp_Object, signal_data: *mut Lisp_Object) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut node = Qnil();
    if !treesit_predicate_capture_name_to_node(name, captures, &raw mut node, signal_data) { return false; }
    let old_buffer = current_buffer!();
    set_buffer_internal(XBUFFER((*XTS_PARSER((*XTS_NODE(node)).parser)).buffer));
    *text = Fbuffer_substring(Ftreesit_node_start(node), Ftreesit_node_end(node));
    set_buffer_internal(old_buffer);
    true

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_predicate_equal(args: Lisp_Object, captures: capture_range,
    signal_data: *mut Lisp_Object) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if list_length(args) != 2 {
        *signal_data = list2(build_string(c"Predicate `equal' requires two arguments but got".as_ptr()), Flength(args));
        return false;
    }
    let arg1 = XCAR(args);
    let arg2 = XCAR(XCDR(args));
    let mut text1 = arg1;
    let mut text2 = arg2;
    if SYMBOLP(arg1) && !treesit_predicate_capture_name_to_text(arg1, captures, &raw mut text1, signal_data) { return false; }
    if SYMBOLP(arg2) && !treesit_predicate_capture_name_to_text(arg2, captures, &raw mut text2, signal_data) { return false; }
    !NILP(Fstring_equal(text1, text2))

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe extern "C-unwind" {
    fn assq_no_signal(key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object;
    fn FUNCTIONP(object: Lisp_Object) -> bool;
    fn display_count_lines(start_byte: ptrdiff_t, limit_byte: ptrdiff_t,
        count: ptrdiff_t, byte_pos_ptr: *mut ptrdiff_t) -> ptrdiff_t;
    fn search_buffer(string: Lisp_Object, pos: ptrdiff_t, pos_byte: ptrdiff_t,
        lim: ptrdiff_t, lim_byte: ptrdiff_t, n: EMACS_INT, re: bool,
        trt: Lisp_Object, inverse_trt: Lisp_Object, posix: bool) -> EMACS_INT;
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_predicate_match(args: Lisp_Object, captures: capture_range,
    signal_data: *mut Lisp_Object) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if list_length(args) != 2 {
        *signal_data = list2(build_string(c"Predicate `match?' requires two arguments but got".as_ptr()), Flength(args));
        return false;
    }
    let arg1 = XCAR(args);
    let arg2 = XCAR(XCDR(args));
    let regexp = if SYMBOLP(arg2) { arg1 } else { arg2 };
    let capture_name = if SYMBOLP(arg2) { arg2 } else { arg1 };
    if !STRINGP(regexp) || !SYMBOLP(capture_name) {
        *signal_data = list2(build_string(c"Predicate `match?' takes a regexp and a node capture (order doesn't matter), but got".as_ptr()), Flength(args));
        return false;
    }
    let mut node = Qnil();
    if !treesit_predicate_capture_name_to_node(capture_name, captures, &raw mut node, signal_data) { return false; }
    let treesit_node = (*XTS_NODE(node)).node;
    let visible_beg = (*XTS_PARSER((*XTS_NODE(node)).parser)).visible_beg;
    let start_byte_offset = ts_node_start_byte(treesit_node);
    let end_byte_offset = ts_node_end_byte(treesit_node);
    let start_byte = visible_beg + start_byte_offset as ptrdiff_t;
    let end_byte = visible_beg + end_byte_offset as ptrdiff_t;
    let start_pos = BYTE_TO_CHAR(start_byte);
    let end_pos = BYTE_TO_CHAR(end_byte);
    let old_begv = BEGV!(); let old_begv_byte = BEGV_BYTE!();
    let old_zv = ZV!(); let old_zv_byte = ZV_BYTE!();
    BEGV!() = start_pos; BEGV_BYTE!() = start_byte;
    ZV!() = end_pos; ZV_BYTE!() = end_byte;
    let val = search_buffer(regexp, start_pos, start_byte, end_pos, end_byte,
        1, true, Qnil(), Qnil(), false);
    BEGV!() = old_begv; BEGV_BYTE!() = old_begv_byte;
    ZV!() = old_zv; ZV_BYTE!() = old_zv_byte;
    val > 0

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_predicate_pred(args: Lisp_Object, captures: capture_range,
    signal_data: *mut Lisp_Object, parser: Lisp_Object) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if list_length(args) < 2 {
        *signal_data = list2(build_string(c"Predicate `pred' requires at least two arguments, but only got".as_ptr()), Flength(args));
        return false;
    }
    let fn_ = Fintern(XCAR(args), Qnil());
    let mut nodes = Qnil();
    let mut tail = XCDR(args);
    FOR_EACH_TAIL!(tail, {
        let mut node = Qnil();
        if !treesit_predicate_capture_name_to_node(XCAR(tail), captures, &raw mut node, signal_data) { return false; }
        nodes = Fcons(node, nodes);
    });
    nodes = Fnreverse(nodes);
    let ts = (*XTS_PARSER(parser)).timestamp;
    let val = CALLN!(Fapply, fn_, nodes);
    if (*XTS_PARSER(parser)).timestamp != ts { *signal_data = list1(fn_); return false; }
    !NILP(val)

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_eval_predicates(captures: capture_range, predicates: Lisp_Object,
    signal_data: *mut Lisp_Object, parser: Lisp_Object) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut pass = true;
    let mut tail = predicates;
    while pass && !NILP(tail) {
        let predicate = XCAR(tail);
        let fn_ = XCAR(predicate);
        let args = XCDR(predicate);
        if !NILP(Fstring_equal(fn_, (*Vtreesit_str_eq_question_mark.as_mut_ptr ()))) {
            pass &= treesit_predicate_equal(args, captures, signal_data);
        } else if !NILP(Fstring_equal(fn_, (*Vtreesit_str_match_question_mark.as_mut_ptr ()))) {
            pass &= treesit_predicate_match(args, captures, signal_data);
        } else if !NILP(Fstring_equal(fn_, (*Vtreesit_str_pred_question_mark.as_mut_ptr ()))) {
            pass &= treesit_predicate_pred(args, captures, signal_data, parser);
        } else {
            *signal_data = list3(build_string(c"Invalid predicate".as_ptr()), fn_,
                build_string(c"Currently Emacs only supports `equal', `match', and `pred' predicates".as_ptr()));
            pass = false;
        }
        tail = XCDR(tail);
    }
    pass

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_resolve_node(obj: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if TS_NODEP(obj) { treesit_check_node(obj); obj }
    else if TS_PARSERP(obj) { treesit_check_parser(obj); Ftreesit_parser_root_node(obj) }
    else if SYMBOLP(obj) {
        let parser = Ftreesit_parser_create(obj, Fcurrent_buffer(), Qnil(), Qnil());
        Ftreesit_parser_root_node(parser)
    } else { wrong_type_argument(list4(Qor(), Qtreesit_node_p(), Qtreesit_parser_p(), Qsymbolp()), obj) }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_initialize_query(mut query: Lisp_Object, lang: *const TSLanguage,
    ts_query: *mut *mut TSQuery, cursor: *mut *mut TSQueryCursor, need_free: *mut bool,
    signal_symbol: *mut Lisp_Object, signal_data: *mut Lisp_Object) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if TS_COMPILED_QUERY_P(query) {
        *ts_query = treesit_ensure_query_compiled(query, signal_symbol, signal_data);
        *cursor = treesit_ensure_query_cursor(query);
        *need_free = false;
        !(*ts_query).is_null()
    } else {
        if CONSP(query) { query = Ftreesit_query_expand(query); }
        let mut error_offset = 0;
        let mut error_type = TSQueryErrorNone;
        *ts_query = ts_query_new(lang, SSDATA(query), SBYTES(query) as u32,
            &raw mut error_offset, &raw mut error_type);
        if (*ts_query).is_null() {
            *signal_symbol = Qtreesit_query_error();
            *signal_data = treesit_compose_query_signal_data(error_offset, error_type, query);
            false
        } else { *cursor = ts_query_cursor_new(); *need_free = true; true }
    }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn query_capture_remove_capture_name(start: Lisp_Object, end: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut tail = start;
    FOR_EACH_TAIL!(tail, {
        let cell = CAR(tail);
        CHECK_CONS(cell);
        XSETCAR(tail, CDR(cell));
        if EQ(CDR(tail), end) { return; }
    });

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_assume_true(val: bool) { eassert!(val == true); }

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_cursor_helper_1(cursor: *mut TSTreeCursor, target: *const TSNode,
    start_pos: u32, end_pos: u32, limit: ptrdiff_t) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if limit <= 0 { return false; }
    let mut cursor_node = ts_tree_cursor_current_node(cursor);
    if ts_node_eq(cursor_node, *target) { return true; }
    if !((start_pos != end_pos && ts_tree_cursor_goto_first_child_for_byte(cursor, start_pos) != -1)
        || ts_tree_cursor_goto_first_child(cursor)) { return false; }
    // Keep the selected source's cursor_node update order.
    while ts_node_start_byte(cursor_node) <= end_pos {
        if ts_node_end_byte(cursor_node) >= end_pos
            && treesit_cursor_helper_1(cursor, target, start_pos, end_pos, limit - 1) { return true; }
        if !ts_tree_cursor_goto_next_sibling(cursor) { break; }
        cursor_node = ts_tree_cursor_current_node(cursor);
    }
    treesit_assume_true(ts_tree_cursor_goto_parent(cursor));
    false

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_cursor_helper(cursor: *mut TSTreeCursor, node: TSNode, parser: Lisp_Object) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let start_pos = ts_node_start_byte(node);
    let end_pos = ts_node_end_byte(node);
    let root = ts_tree_root_node((*XTS_PARSER(parser)).tree);
    *cursor = ts_tree_cursor_new(root);
    let success = treesit_cursor_helper_1(cursor, &node, start_pos, end_pos, TREESIT_RECURSION_LIMIT as ptrdiff_t);
    if !success { ts_tree_cursor_delete(cursor); }
    success

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_traverse_sibling_helper(cursor: *mut TSTreeCursor, forward: bool, named: bool) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if forward {
        if !named { return ts_tree_cursor_goto_next_sibling(cursor); }
        while ts_tree_cursor_goto_next_sibling(cursor) {
            if ts_node_is_named(ts_tree_cursor_current_node(cursor)) { return true; }
        }
        false
    } else {
        #[cfg(feature = "have_ts_tree_cursor_goto_previous_sibling")]
        {
            if !named { return ts_tree_cursor_goto_previous_sibling(cursor); }
            while ts_tree_cursor_goto_previous_sibling(cursor) {
                if ts_node_is_named(ts_tree_cursor_current_node(cursor)) { return true; }
            }
            false
        }
        #[cfg(not(feature = "have_ts_tree_cursor_goto_previous_sibling"))]
        {
            let start = ts_tree_cursor_current_node(cursor);
            if !ts_tree_cursor_goto_parent(cursor) { return false; }
            treesit_assume_true(ts_tree_cursor_goto_first_child(cursor));
            let first_child = ts_tree_cursor_current_node(cursor);
            if ts_node_eq(first_child, start) { return false; }
            let mut probe = ts_tree_cursor_copy(cursor);
            let mut delta: ptrdiff_t = 0;
            while ts_tree_cursor_goto_next_sibling(&raw mut probe) {
                delta += 1;
                let probe_node = ts_tree_cursor_current_node(&raw mut probe);
                if ts_node_eq(probe_node, start) {
                    ts_tree_cursor_delete(&raw mut probe);
                    let cursor_node = ts_tree_cursor_current_node(cursor);
                    ts_tree_cursor_delete(&raw mut probe);
                    return !named || (named && ts_node_is_named(cursor_node));
                }
                if !named || (named && ts_node_is_named(probe_node)) {
                    while delta > 0 {
                        treesit_assume_true(ts_tree_cursor_goto_next_sibling(cursor));
                        delta -= 1;
                    }
                }
            }
            ts_tree_cursor_delete(&raw mut probe);
            false
        }
    }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_traverse_child_helper(cursor: *mut TSTreeCursor, forward: bool, named: bool) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if forward {
        if !named { return ts_tree_cursor_goto_first_child(cursor); }
        if !ts_tree_cursor_goto_first_child(cursor) { return false; }
        let first_child = ts_tree_cursor_current_node(cursor);
        if ts_node_is_named(first_child) { return true; }
        if treesit_traverse_sibling_helper(cursor, true, true) { true }
        else { treesit_assume_true(ts_tree_cursor_goto_parent(cursor)); false }
    } else {
        if !ts_tree_cursor_goto_first_child(cursor) { return false; }
        while ts_tree_cursor_goto_next_sibling(cursor) {}
        if !named || (named && ts_node_is_named(ts_tree_cursor_current_node(cursor))) { return true; }
        if treesit_traverse_sibling_helper(cursor, false, true) { true }
        else { treesit_assume_true(ts_tree_cursor_goto_parent(cursor)); false }
    }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_traverse_get_predicate(thing: Lisp_Object, language: Lisp_Object) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let cons = assq_no_signal(language, (*Vtreesit_thing_settings.as_mut_ptr ()));
    if NILP(cons) { return Qnil(); }
    let definitions = XCDR(cons);
    let entry = assq_no_signal(thing, definitions);
    if NILP(entry) { return Qnil(); }
    let cdr = XCDR(entry);
    if !CONSP(cdr) { return Qnil(); }
    XCAR(cdr)

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_traverse_validate_predicate(pred: Lisp_Object, language: Lisp_Object,
    signal_data: *mut Lisp_Object, recursion_level: ptrdiff_t) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if recursion_level > 99 {
        *signal_data = list2(Qtreesit_invalid_predicate(),
            build_string(c"Predicate recursion level exceeded: it must not exceed 100 levels".as_ptr()));
        return false;
    }
    if STRINGP(pred) { return true; }
    else if FUNCTIONP(pred) && !(SYMBOLP(pred) && !NILP(Fget(pred, Qtreesit_thing_symbol()))) { return true; }
    else if SYMBOLP(pred) {
        if BASE_EQ(pred, Qnamed()) || BASE_EQ(pred, Qanonymous()) { return true; }
        let definition = treesit_traverse_get_predicate(pred, language);
        if NILP(definition) {
            *signal_data = list3(Qtreesit_predicate_not_found(),
                build_string(c"Cannot find the definition of the predicate in `treesit-thing-settings'".as_ptr()), pred);
            return false;
        }
        return treesit_traverse_validate_predicate(definition, language, signal_data, recursion_level+1);
    } else if CONSP(pred) {
        let car = XCAR(pred);
        let mut cdr = XCDR(pred);
        if BASE_EQ(car, Qnot()) {
            if !CONSP(cdr) {
                *signal_data = list3(Qtreesit_invalid_predicate(), build_string(c"Invalid `not' predicate".as_ptr()), pred);
                return false;
            }
            if XFIXNUM(Flength(cdr)) != 1 {
                *signal_data = list3(Qtreesit_invalid_predicate(), build_string(c"`not' can only have one argument".as_ptr()), pred);
                return false;
            }
            return treesit_traverse_validate_predicate(XCAR(cdr), language, signal_data, recursion_level+1);
        } else if BASE_EQ(car, Qor()) || BASE_EQ(car, Qand()) {
            if !CONSP(cdr) || NILP(cdr) {
                *signal_data = list3(Qtreesit_invalid_predicate(),
                    build_string(c"`or' or `and' must have a list of patterns as arguments ".as_ptr()), pred);
                return false;
            }
            FOR_EACH_TAIL!(cdr, {
                if !treesit_traverse_validate_predicate(XCAR(cdr), language, signal_data, recursion_level+1) { return false; }
            });
            return true;
        } else if STRINGP(car) && FUNCTIONP(cdr) { return true; }
    }
    *signal_data = list3(Qtreesit_invalid_predicate(),
        build_string(c"Invalid predicate, see `treesit-thing-settings' for valid forms of predicate".as_ptr()), pred);
    false

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_traverse_match_predicate(cursor: *mut TSTreeCursor, pred: Lisp_Object,
    parser: Lisp_Object, named: bool) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let node = ts_tree_cursor_current_node(cursor);
    if named && !ts_node_is_named(node) { return false; }
    if STRINGP(pred) {
        let type_ = ts_node_type(node);
        if type_.is_null() { return false; }
        return fast_c_string_match(pred, type_, strlen(type_)) >= 0;
    } else if FUNCTIONP(pred) && !(SYMBOLP(pred) && !NILP(Fget(pred, Qtreesit_thing_symbol()))) {
        let lisp_node = make_treesit_node(parser, node);
        let res = treesit_pred_with_guard(pred, lisp_node, parser);
        return !NILP(res);
    } else if SYMBOLP(pred) && BASE_EQ(pred, Qnamed()) { return ts_node_is_named(node); }
    else if SYMBOLP(pred) && BASE_EQ(pred, Qanonymous()) { return !ts_node_is_named(node); }
    else if SYMBOLP(pred) {
        let language = (*XTS_PARSER(parser)).language_symbol;
        let definition = treesit_traverse_get_predicate(pred, language);
        return treesit_traverse_match_predicate(cursor, definition, parser, named);
    } else if CONSP(pred) {
        let car = XCAR(pred);
        let mut cdr = XCDR(pred);
        if BASE_EQ(car, Qnot()) {
            return !treesit_traverse_match_predicate(cursor, XCAR(cdr), parser, named);
        } else if BASE_EQ(car, Qor()) {
            FOR_EACH_TAIL!(cdr, {
                if treesit_traverse_match_predicate(cursor, XCAR(cdr), parser, named) { return true; }
            });
            return false;
        } else if BASE_EQ(car, Qand()) {
            FOR_EACH_TAIL!(cdr, {
                if !treesit_traverse_match_predicate(cursor, XCAR(cdr), parser, named) { return false; }
            });
            return true;
        } else if STRINGP(car) && FUNCTIONP(cdr) {
            let type_ = ts_node_type(node);
            if type_.is_null() { return false; }
            if !(fast_c_string_match(car, type_, strlen(type_)) >= 0) { return false; }
            let lisp_node = make_treesit_node(parser, node);
            if NILP(calln!(cdr, lisp_node)) { return false; }
            return true;
        }
    }
    false

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_search_dfs(cursor: *mut TSTreeCursor, pred: Lisp_Object,
    parser: Lisp_Object, forward: bool, named: bool, limit: ptrdiff_t, skip_root: bool) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if !skip_root && treesit_traverse_match_predicate(cursor, pred, parser, named) { return true; }
    if limit == 0 { return false; }
    if !treesit_traverse_child_helper(cursor, forward, named) { return false; }
    loop {
        if treesit_search_dfs(cursor, pred, parser, forward, named, limit-1, false) { return true; }
        if !treesit_traverse_sibling_helper(cursor, forward, false) { break; }
    }
    treesit_assume_true(ts_tree_cursor_goto_parent(cursor));
    false

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_search_forward(cursor: *mut TSTreeCursor, pred: Lisp_Object,
    parser: Lisp_Object, forward: bool, named: bool) -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut initial = true;
    loop {
        if !initial && treesit_traverse_match_predicate(cursor, pred, parser, named) { return true; }
        initial = false;
        while !treesit_traverse_sibling_helper(cursor, forward, named) {
            if !ts_tree_cursor_goto_parent(cursor) { return false; }
            if treesit_traverse_match_predicate(cursor, pred, parser, named) { return true; }
        }
        while treesit_traverse_child_helper(cursor, forward, false) {}
    }

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe extern "C-unwind" fn treesit_traverse_cleanup_cursor(cursor: *mut c_void) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    ts_tree_cursor_delete(cursor.cast());

  }
}

#[cfg(feature = "have_tree_sitter")]
unsafe fn treesit_build_sparse_tree(cursor: *mut TSTreeCursor, mut parent: Lisp_Object,
    pred: Lisp_Object, process_fn: Lisp_Object, limit: ptrdiff_t, parser: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let match_ = treesit_traverse_match_predicate(cursor, pred, parser, false);
    if match_ {
        let node = ts_tree_cursor_current_node(cursor);
        let mut lisp_node = make_treesit_node(parser, node);
        if !NILP(process_fn) { lisp_node = treesit_pred_with_guard(process_fn, lisp_node, parser); }
        let this = Fcons(lisp_node, Qnil());
        Fsetcdr(parent, Fcons(this, Fcdr(parent)));
        parent = this;
    }
    if limit > 0 && ts_tree_cursor_goto_first_child(cursor) {
        loop {
            treesit_build_sparse_tree(cursor, parent, pred, process_fn, limit-1, parser);
            if !ts_tree_cursor_goto_next_sibling(cursor) { break; }
        }
        ts_tree_cursor_goto_parent(cursor);
    }
    if match_ { Fsetcdr(parent, Fnreverse(Fcdr(parent))); }

  }
}


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-pattern-expand", Ftreesit_pattern_expand, Streesit_pattern_expand, 1, 1, 0,
  doc: "Expand PATTERN to its string form.\n\nPATTERN can be\n\n    :anchor\n    :?\n    :*\n    :+\n    :equal\n    :match\n    (TYPE PATTERN...)\n    [PATTERN...]\n    FIELD-NAME:\n    @CAPTURE-NAME\n    (_)\n    _\n    \"TYPE\"\n\nSee Info node `(elisp)Pattern Matching' for detailed explanation.",
  (pattern: Lisp_Object),
  {
if BASE_EQ(pattern,QCanchor()) { return (*Vtreesit_str_dot.as_mut_ptr ()); }
if BASE_EQ(pattern,QCquestion()) { return (*Vtreesit_str_question_mark.as_mut_ptr ()); }
if BASE_EQ(pattern,QCstar()) { return (*Vtreesit_str_star.as_mut_ptr ()); }
if BASE_EQ(pattern,QCplus()) { return (*Vtreesit_str_plus.as_mut_ptr ()); }
if BASE_EQ(pattern,QCequal())||BASE_EQ(pattern,QCeq_q()) { return (*Vtreesit_str_pound_eq_question_mark.as_mut_ptr ()); }
if BASE_EQ(pattern,QCmatch())||BASE_EQ(pattern,QCmatch_q()) { return (*Vtreesit_str_pound_match_question_mark.as_mut_ptr ()); }
if BASE_EQ(pattern,QCpred())||BASE_EQ(pattern,QCpred_q()) { return (*Vtreesit_str_pound_pred_question_mark.as_mut_ptr ()); }
let opening_delimeter=if VECTORP(pattern) { (*Vtreesit_str_open_bracket.as_mut_ptr ()) } else { (*Vtreesit_str_open_paren.as_mut_ptr ()) };
let closing_delimiter=if VECTORP(pattern) { (*Vtreesit_str_close_bracket.as_mut_ptr ()) } else { (*Vtreesit_str_close_paren.as_mut_ptr ()) };
if VECTORP(pattern)||CONSP(pattern) { return concat3(opening_delimeter,Fmapconcat(Qtreesit_pattern_expand(),pattern,(*Vtreesit_str_space.as_mut_ptr ())),closing_delimiter); }
if STRINGP(pattern) { return treesit_query_string_string(pattern); }return Fprin1_to_string(pattern,Qnil(),Qt());
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-query-expand", Ftreesit_query_expand, Streesit_query_expand, 1, 1, 0,
  doc: "Expand sexp QUERY to its string form.\n\nA PATTERN in QUERY can be\n\n    :anchor\n    :?\n    :*\n    :+\n    :equal\n    :eq?\n    :match\n    :match?\n    (TYPE PATTERN...)\n    [PATTERN...]\n    FIELD-NAME:\n    @CAPTURE-NAME\n    (_)\n    _\n    \"TYPE\"\n\nSee Info node `(elisp)Pattern Matching' for detailed explanation.",
  (query: Lisp_Object),
  {
return Fmapconcat(Qtreesit_pattern_expand(),query,(*Vtreesit_str_space.as_mut_ptr ()));
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-query-compile", Ftreesit_query_compile, Streesit_query_compile, 2, 3, 0,
  doc: "Compile QUERY to a compiled query.\n\nQuerying with a compiled query is much faster than an uncompiled one.\nSo it's a good idea to use compiled query in tight loops, etc.\n\nLANGUAGE is the language this query is for.\n\nIf EAGER is non-nil, immediately load LANGUAGE and compile the query.\nOtherwise defer the compilation until the query is first used.\n\nSignal `treesit-query-error' if QUERY is malformed or something else\ngoes wrong.  (This only happens if EAGER is non-nil.)\nYou can use `treesit-query-validate' to validate and debug a query.",
  (language: Lisp_Object, query: Lisp_Object, eager: Lisp_Object),
  {
if NILP(Ftreesit_query_p(query)) { wrong_type_argument(Qtreesit_query_p(),query); }CHECK_SYMBOL(language);treesit_initialize();
if TS_COMPILED_QUERY_P(query) { if NILP(eager) { return query; }treesit_ensure_query_compiled_signal(query);return query; }
let lisp_query=make_treesit_query(query,language);if NILP(eager) { return lisp_query; }
else { treesit_ensure_query_compiled_signal(lisp_query);return lisp_query; }
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-query-capture", Ftreesit_query_capture, Streesit_query_capture, 2, 6, 0,
  doc: "Query NODE with patterns in QUERY.\n\nReturn a list of (CAPTURE_NAME . NODE).  CAPTURE_NAME is the name\nassigned to the node in PATTERN.  NODE is the captured node.\n\nQUERY is either a string query, a sexp query, or a compiled query.\nSee Info node `(elisp)Pattern Matching' for how to write a query in\neither string or sexp form.  When using repeatedly, a compiled query\nis much faster than a string or sexp one, so it is recommend to\ncompile your query if it will be used repeatedly.\n\nBEG and END, if both non-nil, specify the region of buffer positions\nin which the query is executed.  Any matching node whose span overlaps\nwith the region between BEG and END are captured, it doesn't have to\nbe completely in the region.\n\nIf GROUPED is non-nil, ther function groups the returned list of\ncaptures into matches and return a list of MATCH, where each MATCH is\na list of the form (CAPTURE_NAME . NODE).\n\nIf NODE-ONLY is non-nil, return nodes only, and don't include\nCAPTURE_NAME.\n\nBesides a node, NODE can be a parser, in which case the root node of\nthat parser is used.  NODE can also be a language symbol, in which case\nthe root node of a parser for that language is used.  If such a parser\ndoesn't exist, it is created.\n\nSignal `treesit-query-error' if QUERY is malformed or something else\ngoes wrong.  You can use `treesit-query-validate' to validate and debug\nthe query.",
  (node: Lisp_Object, query: Lisp_Object, beg: Lisp_Object, end: Lisp_Object, node_only: Lisp_Object, grouped: Lisp_Object),
  {
if !(TS_COMPILED_QUERY_P(query)||CONSP(query)||STRINGP(query)) { wrong_type_argument(Qtreesit_query_p(),query); }
treesit_initialize();let lisp_node=treesit_resolve_node(node);treesit_check_node(lisp_node);
let treesit_node=(*XTS_NODE(lisp_node)).node;let lisp_parser=(*XTS_NODE(lisp_node)).parser;
let lang=ts_parser_language((*XTS_PARSER(lisp_parser)).parser);
let buf=XBUFFER((*XTS_PARSER(lisp_parser)).buffer);
if !NILP(beg) { treesit_check_position(beg,buf); }if !NILP(end) { treesit_check_position(end,buf); }
let mut treesit_query=core::ptr::null_mut();let mut cursor=core::ptr::null_mut();let mut needs_to_free_query_and_cursor=false;
let mut signal_symbol=Qnil();let mut signal_data=Qnil();
if !treesit_initialize_query(query,lang,&raw mut treesit_query,&raw mut cursor,&raw mut needs_to_free_query_and_cursor,&raw mut signal_symbol,&raw mut signal_data) { xsignal(signal_symbol,signal_data); }
if !NILP(beg)&&!NILP(end) {
 let visible_beg=(*XTS_PARSER((*XTS_NODE(lisp_node)).parser)).visible_beg;
 let beg_byte=CHAR_TO_BYTE(fix_position(beg));let end_byte=CHAR_TO_BYTE(fix_position(end));
 if beg_byte==visible_beg&&end_byte==visible_beg { return Qnil(); }
 eassert!(beg_byte-visible_beg<=u32::MAX as ptrdiff_t);eassert!(end_byte-visible_beg<=u32::MAX as ptrdiff_t);
 ts_query_cursor_set_byte_range(cursor,(beg_byte-visible_beg) as u32,(end_byte-visible_beg) as u32);
} else { ts_query_cursor_set_byte_range(cursor,0,u32::MAX); }
ts_query_cursor_exec(cursor,treesit_query,treesit_node);
let mut match_storage=core::mem::MaybeUninit::<TSQueryMatch>::uninit();
let patterns_count=ts_query_pattern_count(treesit_query);let mut result=Qnil();let mut prev_result=result;
let predicates_table=make_vector(patterns_count as ptrdiff_t,Qt());let mut predicate_signal_data=Qnil();
let old_buf=current_buffer!();set_buffer_internal(buf);
while ts_query_cursor_next_match(cursor,match_storage.as_mut_ptr()) {
 let query_match=match_storage.assume_init();prev_result=result;let mut match_group=Qnil();let captures=query_match.captures;
 let mut idx:c_int=0;
 while idx<query_match.capture_count as c_int {
  let mut capture_name_len=0;let capture=*captures.offset(idx as isize);let captured_node=make_treesit_node(lisp_parser,capture.node);
  let capture_name=ts_query_capture_name_for_id(treesit_query,capture.index,&raw mut capture_name_len);
  let cap=Fcons(intern_c_string_1(capture_name,capture_name_len as ptrdiff_t),captured_node);
  if NILP(grouped) { result=Fcons(cap,result); }else { match_group=Fcons(cap,match_group); }idx+=1;
 }
 let mut predicates=AREF(predicates_table,query_match.pattern_index as ptrdiff_t);
 if BASE_EQ(predicates,Qt()) { predicates=treesit_predicates_for_pattern(treesit_query,query_match.pattern_index as u32);ASET(predicates_table,query_match.pattern_index as ptrdiff_t,predicates); }
 let mut captures_range=capture_range { start:result,end:prev_result };
 if !NILP(grouped) { captures_range.start=match_group;captures_range.end=Qnil(); }
 let match_=treesit_eval_predicates(captures_range,predicates,&raw mut predicate_signal_data,lisp_parser);
 if !NILP(predicate_signal_data) { break; }
 if NILP(grouped) { if !match_ { result=prev_result; }else if !NILP(node_only) { query_capture_remove_capture_name(result,prev_result); } }
 if match_&&!NILP(grouped) { match_group=Fnreverse(match_group);if !NILP(node_only) { query_capture_remove_capture_name(match_group,Qnil()); }result=Fcons(match_group,result); }
}
if needs_to_free_query_and_cursor { ts_query_delete(treesit_query);ts_query_cursor_delete(cursor); }set_buffer_internal(old_buf);
if !NILP(predicate_signal_data) { xsignal(Qtreesit_query_error(),predicate_signal_data); }return Fnreverse(result);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-search-subtree", Ftreesit_search_subtree, Streesit_search_subtree, 2, 5, 0,
  doc: "Traverse the parse tree of NODE depth-first using PREDICATE.\n\nTraverse the subtree of NODE, and match PREDICATE with each node along\nthe way.\n\nPREDICATE can be a regexp string that matches against each node's\ntype, a predicate function, and more.  See `treesit-thing-settings'\nfor the possible predicates.  PREDICATE can also be a thing defined in\n`treesit-thing-settings'.  Using an undefined thing doesn't raise an\nerror.\n\nBy default, only traverse named nodes, but if ALL is non-nil, traverse\nall nodes.  If BACKWARD is non-nil, traverse backwards.  If DEPTH is\nnon-nil, only traverse nodes up to that number of levels down in the\ntree.  If DEPTH is nil, default to 1000.\n\nReturn the first matched node, or nil if none matches.",
  (node: Lisp_Object, predicate: Lisp_Object, backward: Lisp_Object, all: Lisp_Object, depth: Lisp_Object),
  {
CHECK_TS_NODE(node);CHECK_SYMBOL(all);CHECK_SYMBOL(backward);
let mut the_limit=TREESIT_RECURSION_LIMIT as ptrdiff_t;if !NILP(depth) { CHECK_FIXNUM(depth);the_limit=XFIXNUM(depth); }
treesit_initialize();let parser=(*XTS_NODE(node)).parser;let language=(*XTS_PARSER(parser)).language_symbol;
let mut signal_data=Qnil();
if !treesit_traverse_validate_predicate(predicate,language,&raw mut signal_data,0) {
 let err_symbol=XCAR(signal_data);let data=XCDR(signal_data);
 if EQ(err_symbol,Qtreesit_predicate_not_found()) { return Qnil(); }xsignal1(err_symbol,data);
}
let mut return_value=Qnil();let mut cursor_storage=core::mem::MaybeUninit::<TSTreeCursor>::uninit();
if !treesit_cursor_helper(cursor_storage.as_mut_ptr(),(*XTS_NODE(node)).node,parser) { return return_value; }
let mut cursor=cursor_storage.assume_init();let count=SPECPDL_INDEX();
record_unwind_protect_ptr(Some(treesit_traverse_cleanup_cursor),(&raw mut cursor).cast());
if treesit_search_dfs(&raw mut cursor,predicate,parser,NILP(backward),NILP(all),the_limit,false) { let node=ts_tree_cursor_current_node(&raw mut cursor);return_value=make_treesit_node(parser,node); }
return unbind_to(count,return_value);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-search-forward", Ftreesit_search_forward, Streesit_search_forward, 2, 4, 0,
  doc: "Search for node matching PREDICATE in the parse tree of START.\n\nStart traversing the tree from node START, and match PREDICATE with\neach node (except START itself) along the way.\n\nPREDICATE can be a regexp string that matches against each node's\ntype, a predicate function, and more.  See `treesit-thing-settings'\nfor the possible predicates.  PREDICATE can also be a thing defined in\n`treesit-thing-settings'.  Using an undefined thing doesn't raise an\nerror.\n\nBy default, only search for named nodes, but if ALL is non-nil, search\nfor all nodes.  If BACKWARD is non-nil, search backwards.\n\nReturn the first matched node, or nil if none matches.\n\nFor a tree like below, where START is marked by S, traverse as\nnumbered from 1 to 12:\n\n                12\n                |\n       S--------3----------11\n       |        |          |\n  o--o-+--o  1--+--2    6--+-----10\n  |  |                  |        |\n  o  o                +-+-+   +--+--+\n                      |   |   |  |  |\n                      4   5   7  8  9\n\nNote that this function doesn't traverse the subtree of START, and it\nalways traverse leaf nodes first, then upwards.",
  (start: Lisp_Object, predicate: Lisp_Object, backward: Lisp_Object, all: Lisp_Object),
  {
CHECK_TS_NODE(start);CHECK_SYMBOL(all);CHECK_SYMBOL(backward);
treesit_initialize();let parser=(*XTS_NODE(start)).parser;let language=(*XTS_PARSER(parser)).language_symbol;
let mut signal_data=Qnil();
if !treesit_traverse_validate_predicate(predicate,language,&raw mut signal_data,0) {
 let err_symbol=XCAR(signal_data);let data=XCDR(signal_data);
 if EQ(err_symbol,Qtreesit_predicate_not_found()) { return Qnil(); }xsignal1(err_symbol,data);
}
let mut return_value=Qnil();let mut cursor_storage=core::mem::MaybeUninit::<TSTreeCursor>::uninit();
if !treesit_cursor_helper(cursor_storage.as_mut_ptr(),(*XTS_NODE(start)).node,parser) { return return_value; }
let mut cursor=cursor_storage.assume_init();let count=SPECPDL_INDEX();
record_unwind_protect_ptr(Some(treesit_traverse_cleanup_cursor),(&raw mut cursor).cast());
if treesit_search_forward(&raw mut cursor,predicate,parser,NILP(backward),NILP(all)) { let node=ts_tree_cursor_current_node(&raw mut cursor);return_value=make_treesit_node(parser,node); }
return unbind_to(count,return_value);
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-induce-sparse-tree", Ftreesit_induce_sparse_tree, Streesit_induce_sparse_tree, 2, 4, 0,
  doc: "Create a sparse tree of ROOT's subtree.\n\nThis takes the subtree under ROOT, and combs it so only the nodes that\nmatch PREDICATE are left, like picking out grapes on the vine.\n\nPREDICATE can be a regexp string that matches against each node's\ntype, a predicate function, and more.  See `treesit-thing-settings'\nfor the possible predicates.  PREDICATE can also be a thing defined in\n`treesit-thing-settings'.  Using an undefined thing doesn't raise an\nerror.\n\nFor a subtree on the left that consist of both numbers and letters, if\nPREDICATE is \"is letter\", the returned tree is the one on the right.\n\n\ta                 a              a\n\t|                 |              |\n    +---+---+         +---+---+      +---+---+\n    |   |   |         |   |   |      |   |   |\n    b   1   2         b   |   |      b   c   d\n\t|   |     =>      |   |  =>      |\n\tc   +--+          c   +          e\n\t|   |  |          |   |\n     +--+   d  4       +--+   d\n     |  |              |\n     e  5              e\n\nIf PROCESS-FN is non-nil, it should be a function of one argument.  In\nthat case, instead of returning the matched nodes, pass each node to\nPROCESS-FN, and use its return value instead.\n\nIf non-nil, DEPTH is the number of levels to go down the tree from\nROOT.  If DEPTH is nil or omitted, it defaults to 1000.\n\nEach node in the returned tree looks like (NODE . (CHILD ...)).  The\nroot of this tree might be nil, if ROOT doesn't match PREDICATE.\n\nIf no node matches PREDICATE, return nil.",
  (root: Lisp_Object, predicate: Lisp_Object, process_fn: Lisp_Object, depth: Lisp_Object),
  {
CHECK_TS_NODE(root);if !NILP(process_fn) { CHECK_TYPE(FUNCTIONP(process_fn),Qfunctionp(),process_fn); }
let mut the_limit=TREESIT_RECURSION_LIMIT as ptrdiff_t;if !NILP(depth) { CHECK_FIXNUM(depth);the_limit=XFIXNUM(depth); }
treesit_initialize();let parser=(*XTS_NODE(root)).parser;let language=(*XTS_PARSER(parser)).language_symbol;
let mut signal_data=Qnil();
if !treesit_traverse_validate_predicate(predicate,language,&raw mut signal_data,0) {
 let err_symbol=XCAR(signal_data);let data=XCDR(signal_data);
 if EQ(err_symbol,Qtreesit_predicate_not_found()) { return Qnil(); }xsignal1(err_symbol,data);
}
let parent=Fcons(Qnil(),Qnil());let mut cursor=ts_tree_cursor_new((*XTS_NODE(root)).node);
let count=SPECPDL_INDEX();record_unwind_protect_ptr(Some(treesit_traverse_cleanup_cursor),(&raw mut cursor).cast());
treesit_build_sparse_tree(&raw mut cursor,parent,predicate,process_fn,the_limit,parser);unbind_to(count,Qnil());
Fsetcdr(parent,Fnreverse(Fcdr(parent)));return if NILP(Fcdr(parent)) { Qnil() } else { parent };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-node-match-p", Ftreesit_node_match_p, Streesit_node_match_p, 2, 3, 0,
  doc: "Check whether NODE matches PREDICATE.\n\nPREDICATE can be a symbol representing a thing in\n`treesit-thing-settings', or a predicate, like regexp matching node\ntype, etc.  See `treesit-thing-settings' for more details.\n\nReturn non-nil if NODE matches PREDICATE, nil otherwise.  If NODE is\nnil, return nil.\n\nSignals `treesit-invalid-predicate' if there's no definition of THING\nin `treesit-thing-settings', or if PREDICATE is malformed.  If\nIGNORE-MISSING is non-nil, don't signal an error for missing THING\ndefinition, but still signal for malformed PREDICATE.",
  (node: Lisp_Object, predicate: Lisp_Object, ignore_missing: Lisp_Object),
  {
if NILP(node) { return Qnil(); }CHECK_TS_NODE(node);
let parser=(*XTS_NODE(node)).parser;let language=(*XTS_PARSER(parser)).language_symbol;let mut signal_data=Qnil();
if !treesit_traverse_validate_predicate(predicate,language,&raw mut signal_data,0) {
 let err_symbol=XCAR(signal_data);let data=XCDR(signal_data);
 if !NILP(ignore_missing)&&EQ(err_symbol,Qtreesit_predicate_not_found()) { return Qnil(); }xsignal1(err_symbol,data);
}
let mut cursor=ts_tree_cursor_new((*XTS_NODE(node)).node);let count=SPECPDL_INDEX();
record_unwind_protect_ptr(Some(treesit_traverse_cleanup_cursor),(&raw mut cursor).cast());
let match_=treesit_traverse_match_predicate(&raw mut cursor,predicate,parser,false);unbind_to(count,Qnil());
return if match_ { Qt() } else { Qnil() };
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit-subtree-stat", Ftreesit_subtree_stat, Streesit_subtree_stat, 1, 1, 0,
  doc: "Return information about the subtree of NODE.\n\nReturn a list (MAX-DEPTH MAX-WIDTH COUNT), where MAX-DEPTH is the\nmaximum depth of the subtree, MAX-WIDTH is the maximum number of\ndirect children of nodes in the subtree, and COUNT is the number of\nnodes in the subtree, including NODE.",
  (node: Lisp_Object),
  {
CHECK_TS_NODE(node);treesit_initialize();let mut cursor=ts_tree_cursor_new((*XTS_NODE(node)).node);
let mut max_depth:ptrdiff_t=1;let mut max_width:ptrdiff_t=0;let mut count:ptrdiff_t=0;let mut current_depth:ptrdiff_t=0;
loop {
 count+=1;
 while ts_tree_cursor_goto_first_child(&raw mut cursor) {
  current_depth+=1;count+=1;let mut width_count=1;
  while ts_tree_cursor_goto_next_sibling(&raw mut cursor) { width_count+=1; }max_width=max_width.max(width_count);
  treesit_assume_true(ts_tree_cursor_goto_parent(&raw mut cursor));treesit_assume_true(ts_tree_cursor_goto_first_child(&raw mut cursor));
 }
 max_depth=max_depth.max(current_depth);
 while !ts_tree_cursor_goto_next_sibling(&raw mut cursor) {
  if ts_tree_cursor_goto_parent(&raw mut cursor) { current_depth-=1; }
  else { ts_tree_cursor_delete(&raw mut cursor);return list3(make_fixnum(max_depth),make_fixnum(max_width),make_fixnum(count)); }
 }
}
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit--linecol-at", Ftreesit__linecol_at, Streesit__linecol_at, 1, 1, 0,
  doc: "Test buffer-local linecol cache.\n\nCalculate the line and column at POS using the buffer-local cache,\nreturn the line and column in the form of\n\n  (LINE . COL)\n\nThis is used for internal testing and debugging ONLY.",
  (pos: Lisp_Object),
  {
treesit_check_position(pos,current_buffer!());
let pos_linecol=treesit_linecol_of_pos(CHAR_TO_BYTE(fix_position(pos)),BUF_TS_LINECOL_POINT(current_buffer!()));
return Fcons(make_fixnum(pos_linecol.line),make_fixnum(pos_linecol.col));
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit--linecol-cache-set", Ftreesit__linecol_cache_set, Streesit__linecol_cache_set, 3, 3, 0,
  doc: "Set the linecol cache for the current buffer.\n\nThis is used for internal testing and debugging ONLY.",
  (line: Lisp_Object, col: Lisp_Object, bytepos: Lisp_Object),
  {
CHECK_FIXNUM(line);CHECK_FIXNUM(col);CHECK_FIXNUM(bytepos);
let linecol=ts_linecol { line:XFIXNUM(line),col:XFIXNUM(col),bytepos:XFIXNUM(bytepos) };
SET_BUF_TS_LINECOL_POINT(current_buffer!(),linecol);return Qnil();
});


#[cfg(feature = "have_tree_sitter")]
DEFUN! ("treesit--linecol-cache", Ftreesit__linecol_cache, Streesit__linecol_cache, 0, 0, 0,
  doc: "Return the buffer-local linecol cache for debugging.\n\nReturn a plist (:line LINE :col COL :pos POS :bytepos BYTEPOS).  This is\nused for internal testing and debugging ONLY.",
  (),
  {
let cache=BUF_TS_LINECOL_POINT(current_buffer!());
let mut plist=list4(QCcol(),make_fixnum(cache.col),QCbytepos(),make_fixnum(cache.bytepos));
plist=Fcons(make_fixnum(cache.line),plist);plist=Fcons(QCline(),plist);return plist;
});

DEFUN! ("treesit-available-p", Ftreesit_available_p,
       Streesit_available_p, 0, 0, 0,
       doc: "Return non-nil if tree-sitter support is built-in and available.",
       (),
       {
  #[cfg(feature = "have_tree_sitter")]
  { return if load_tree_sitter_if_necessary(false) { Qt() } else { Qnil() }; }
  #[cfg(not(feature = "have_tree_sitter"))]
  { return Qnil(); }
});

/*** Initialization  */

/* Initialize the tree-sitter routines.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_treesit ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  #[cfg(feature = "have_tree_sitter")]
  {

  DEFSYM!(Qtreesit_parser_p, "treesit-parser-p");
  DEFSYM!(Qtreesit_node_p, "treesit-node-p");
  DEFSYM!(Qtreesit_compiled_query_p, "treesit-compiled-query-p");
  DEFSYM!(Qtreesit_query_p, "treesit-query-p");
  DEFSYM!(Qnamed, "named");
  DEFSYM!(Qanonymous, "anonymous");
  DEFSYM!(Qmissing, "missing");
  DEFSYM!(Qextra, "extra");
  DEFSYM!(Qoutdated, "outdated");
  DEFSYM!(Qhas_error, "has-error");
  DEFSYM!(Qlive, "live");
  DEFSYM!(Qnot, "not");

  DEFSYM!(QCanchor, ":anchor");
  DEFSYM!(QCquestion, ":?");
  DEFSYM!(QCstar, ":*");
  DEFSYM!(QCplus, ":+");
  DEFSYM!(QCequal, ":equal");
  DEFSYM!(QCeq_q, ":eq?");
  DEFSYM!(QCmatch, ":match");
  DEFSYM!(QCmatch_q, ":match?");
  DEFSYM!(QCpred, ":pred");
  DEFSYM!(QCpred_q, ":pred?");
  DEFSYM!(QCline, ":line");
  DEFSYM!(QCcol, ":col");
  DEFSYM!(QCpos, ":pos");
  DEFSYM!(QCbytepos, ":bytepos");


  DEFSYM!(Qnot_found, "not-found");
  DEFSYM!(Qsymbol_error, "symbol-error");
  DEFSYM!(Qlang_version_mismatch, "language-grammar-version-mismatch");

  DEFSYM!(Qtreesit_error, "treesit-error");
  DEFSYM!(Qtreesit_query_error, "treesit-query-error");
  DEFSYM!(Qtreesit_parse_error, "treesit-parse-error");
  DEFSYM!(Qtreesit_range_invalid, "treesit-range-invalid");
  DEFSYM!(Qtreesit_buffer_too_large,
	  "treesit-buffer-too-large");
  DEFSYM!(Qtreesit_load_language_error,
	  "treesit-load-language-error");
  DEFSYM!(Qtreesit_node_outdated,
	  "treesit-node-outdated");
  DEFSYM!(Qtreesit_buffer_changed,
	  "treesit-buffer_changed");
  DEFSYM!(Qtreesit_node_buffer_killed,
	  "treesit-node-buffer-killed");
  DEFSYM!(Quser_emacs_directory,
	  "user-emacs-directory");
  DEFSYM!(Qtreesit_parser_deleted, "treesit-parser-deleted");
  DEFSYM!(Qtreesit_pattern_expand, "treesit-pattern-expand");
  DEFSYM!(Qtreesit_invalid_predicate, "treesit-invalid-predicate");
  DEFSYM!(Qtreesit_predicate_not_found, "treesit-predicate-not-found");

  DEFSYM!(Qtreesit_thing_symbol, "treesit-thing-symbol");

  DEFSYM!(Qor, "or");
  DEFSYM!(Qand, "and");

  DEFSYM!(Qhaskell, "haskell");

#[cfg(feature = "windowsnt")]
  DEFSYM!(Qtree_sitter, "tree-sitter");


  define_error(Qtreesit_error(), c"Generic tree-sitter error".as_ptr(), Qerror());
  define_error(Qtreesit_query_error(), c"Query pattern is malformed".as_ptr(), Qtreesit_error());

  define_error(Qtreesit_parse_error(), c"Parse failed".as_ptr(), Qtreesit_error());
  define_error(Qtreesit_range_invalid(), c"RANGES are invalid: they have to be ordered and should not overlap".as_ptr(), Qtreesit_error());
  define_error(Qtreesit_buffer_too_large(), c"Buffer too large (> 4GiB)".as_ptr(), Qtreesit_error());
  define_error(Qtreesit_load_language_error(), c"Cannot load language definition".as_ptr(), Qtreesit_error());
  define_error(Qtreesit_node_outdated(), c"This node is outdated, please retrieve a new one".as_ptr(), Qtreesit_error());
  define_error(Qtreesit_buffer_changed(), c"Buffer content changed, please don't edit buffer in predicate function, etc".as_ptr(), Qtreesit_error());
  define_error(Qtreesit_node_buffer_killed(), c"The buffer associated with this node is killed".as_ptr(), Qtreesit_error());
  define_error(Qtreesit_parser_deleted(), c"This parser is deleted and cannot be used".as_ptr(), Qtreesit_error());
  define_error(Qtreesit_invalid_predicate(), c"Invalid predicate, see `treesit-thing-settings' for valid forms for a predicate".as_ptr(), Qtreesit_error());

  DEFVAR_LISP!(c"treesit-load-name-override-list", Vtreesit_load_name_override_list, "An override list for unconventional tree-sitter libraries.\n\nBy default, Emacs assumes the dynamic library for LANG is\nlibtree-sitter-LANG.EXT, where EXT is the OS specific extension for\ndynamic libraries.  Emacs also assumes that the name of the C function\nthe library provides is tree_sitter_LANG.  If that is not the case,\nyou can add an entry\n\n    (LANG LIBRARY-BASE-NAME FUNCTION-NAME)\n\nto this list, where LIBRARY-BASE-NAME is the filename of the dynamic\nlibrary without the file-name extension, and FUNCTION-NAME is the\nfunction provided by the library.");
  (*Vtreesit_load_name_override_list.as_mut_ptr ()) = Qnil();

  DEFVAR_LISP!(c"treesit-extra-load-path", Vtreesit_extra_load_path, "Additional directories to look for tree-sitter language definitions.\nThe value should be a list of directories.\nWhen trying to load a tree-sitter language definition,\nEmacs first looks in the directories mentioned in this variable,\nthen in the `tree-sitter' subdirectory of `user-emacs-directory', and\nthen in the system default locations for dynamic libraries, in that order.\nThe first writeable directory in the list is special: it's used as the\ndefault directory when automatically installing the language grammar\nusing `treesit-ensure-installed'.");
  (*Vtreesit_extra_load_path.as_mut_ptr ()) = Qnil();

  DEFVAR_LISP!(c"treesit-thing-settings", Vtreesit_thing_settings, "A list defining things.\n\nThe value should be defined by the major mode, and should be an alist\nof the form (LANGUAGE . DEFINITIONS), where LANGUAGE is a language\nsymbol and DEFINITIONS is a list whose elements are of the form\n\n    (THING PRED)\n\nTHING is a symbol representing the thing, like `defun', `defclass',\n`sexp', `sentence', `comment', or any other symbol that is meaningful\nfor the major mode; PRED defines what kind of node can be qualified\nas THING.\n\nPRED can be a regexp string that matches the type of the node; it can\nbe a predicate function that takes the node as the sole argument and\nreturns t if the node is the thing, and nil otherwise; it can be a\ncons (REGEXP . FN), which is a combination of a regexp and a predicate\nfunction, and the node has to match both to qualify as the thing.\n\nPRED can also be recursively defined.  It can be:\n\n (or PRED...), meaning satisfying any of the inner PREDs qualifies the node;\n (and PRED...) meaning satisfying all of the inner PREDs qualifies the node;\n (not PRED), meaning not satisfying the inner PRED qualifies the node.\n\nThere are two pre-defined predicates, `named' and `anonymous'.  They\nmatch named nodes and anonymous nodes, respectively.\n\nFinally, PRED can refer to other THINGs defined in this list by using\nthe symbol of that THING.  For example, (or sexp sentence).");
  (*Vtreesit_thing_settings.as_mut_ptr ()) = Qnil();

  DEFVAR_LISP!(c"treesit-language-remap-alist", Vtreesit_language_remap_alist, "An alist remapping language symbols.\n\nThe value should be an alist of (LANGUAGE-A . LANGUAGE-B).  When such\npair exists in the alist, creating a parser for LANGUAGE-A actually\ncreates a parser for LANGUAGE-B.  Basically, anything that requires or\napplies to LANGUAGE-A will be redirected to LANGUAGE-B instead.");
  (*Vtreesit_language_remap_alist.as_mut_ptr ()) = Qnil();
  DEFSYM!(Qtreesit_language_remap_alist, "treesit-language-remap-alist");
  Fmake_variable_buffer_local (Qtreesit_language_remap_alist());

  DEFVAR_LISP!(c"treesit-languages-require-line-column-tracking", Vtreesit_languages_require_line_column_tracking, "A list of languages that need line-column tracking.\n\nMost tree-sitter language grammars don't require line and column\ntracking to work, but some languages do.  When creating a parser, if the\nlanguage is in this list, Emacs enables line-column tracking for the\nbuffer.");
  (*Vtreesit_languages_require_line_column_tracking.as_mut_ptr ()) = list1 (Qhaskell());

  DEFVAR_LISP!(c"treesit-major-mode-remap-alist", Vtreesit_major_mode_remap_alist, "Alist mapping file-specified modes to ts-modes.\n\nThe value should be an alist of (MODE . TS-MODE).\nThis alist is used to modify the value of `major-mode-remap-alist'\ndepending on customization of `treesit-enabled-modes'.");
  (*Vtreesit_major_mode_remap_alist.as_mut_ptr ()) = Qnil();

  staticpro(Vtreesit_str_libtree_sitter.as_mut_ptr ());
  (*Vtreesit_str_libtree_sitter.as_mut_ptr ()) = build_string(c"libtree-sitter-".as_ptr());
  staticpro(Vtreesit_str_tree_sitter.as_mut_ptr ());
  (*Vtreesit_str_tree_sitter.as_mut_ptr ()) = build_string(c"tree-sitter-".as_ptr());
#[cfg(not(feature = "windowsnt"))]
  staticpro(Vtreesit_str_dot_0.as_mut_ptr ());
  (*Vtreesit_str_dot_0.as_mut_ptr ()) = build_string(c".0".as_ptr());

  staticpro(Vtreesit_str_dot.as_mut_ptr ());
  (*Vtreesit_str_dot.as_mut_ptr ()) = build_string(c".".as_ptr());
  staticpro(Vtreesit_str_question_mark.as_mut_ptr ());
  (*Vtreesit_str_question_mark.as_mut_ptr ()) = build_string(c"?".as_ptr());
  staticpro(Vtreesit_str_star.as_mut_ptr ());
  (*Vtreesit_str_star.as_mut_ptr ()) = build_string(c"*".as_ptr());
  staticpro(Vtreesit_str_plus.as_mut_ptr ());
  (*Vtreesit_str_plus.as_mut_ptr ()) = build_string(c"+".as_ptr());
  staticpro(Vtreesit_str_pound_eq_question_mark.as_mut_ptr ());
  (*Vtreesit_str_pound_eq_question_mark.as_mut_ptr ()) = build_string(c"#eq?".as_ptr());
  staticpro(Vtreesit_str_pound_match_question_mark.as_mut_ptr ());
  (*Vtreesit_str_pound_match_question_mark.as_mut_ptr ()) = build_string(c"#match?".as_ptr());
  staticpro(Vtreesit_str_pound_pred_question_mark.as_mut_ptr ());
  (*Vtreesit_str_pound_pred_question_mark.as_mut_ptr ()) = build_string(c"#pred?".as_ptr());
  staticpro(Vtreesit_str_open_bracket.as_mut_ptr ());
  (*Vtreesit_str_open_bracket.as_mut_ptr ()) = build_string(c"[".as_ptr());
  staticpro(Vtreesit_str_close_bracket.as_mut_ptr ());
  (*Vtreesit_str_close_bracket.as_mut_ptr ()) = build_string(c"]".as_ptr());
  staticpro(Vtreesit_str_open_paren.as_mut_ptr ());
  (*Vtreesit_str_open_paren.as_mut_ptr ()) = build_string(c"(".as_ptr());
  staticpro(Vtreesit_str_close_paren.as_mut_ptr ());
  (*Vtreesit_str_close_paren.as_mut_ptr ()) = build_string(c")".as_ptr());
  staticpro(Vtreesit_str_space.as_mut_ptr ());
  (*Vtreesit_str_space.as_mut_ptr ()) = build_string(c" ".as_ptr());
  staticpro(Vtreesit_str_eq_question_mark.as_mut_ptr ());
  (*Vtreesit_str_eq_question_mark.as_mut_ptr ()) = build_string(c"eq?".as_ptr());
  staticpro(Vtreesit_str_match_question_mark.as_mut_ptr ());
  (*Vtreesit_str_match_question_mark.as_mut_ptr ()) = build_string(c"match?".as_ptr());
  staticpro(Vtreesit_str_pred_question_mark.as_mut_ptr ());
  (*Vtreesit_str_pred_question_mark.as_mut_ptr ()) = build_string(c"pred?".as_ptr());
  staticpro(Vtreesit_str_empty.as_mut_ptr ());
  (*Vtreesit_str_empty.as_mut_ptr ()) = build_string(c"".as_ptr());

  defsubr(Streesit_language_available_p.as_mut_ptr ());
  defsubr(Streesit_library_abi_version.as_mut_ptr ());
  defsubr(Streesit_language_abi_version.as_mut_ptr ());
  defsubr(Streesit_grammar_location.as_mut_ptr ());

  defsubr(Streesit_parser_tracking_line_column_p.as_mut_ptr ());
  defsubr(Streesit_tracking_line_column_p.as_mut_ptr ());

  defsubr(Streesit_parser_p.as_mut_ptr ());
  defsubr(Streesit_node_p.as_mut_ptr ());
  defsubr(Streesit_compiled_query_p.as_mut_ptr ());
  defsubr(Streesit_query_p.as_mut_ptr ());
  defsubr(Streesit_query_eagerly_compiled_p.as_mut_ptr ());
  defsubr(Streesit_query_language.as_mut_ptr ());
  defsubr(Streesit_query_source.as_mut_ptr ());

  defsubr(Streesit_node_parser.as_mut_ptr ());

  defsubr(Streesit_parser_create.as_mut_ptr ());
  defsubr(Streesit_parser_delete.as_mut_ptr ());
  defsubr(Streesit_parser_list.as_mut_ptr ());
  defsubr(Streesit_parser_buffer.as_mut_ptr ());
  defsubr(Streesit_parser_language.as_mut_ptr ());
  defsubr(Streesit_parser_tag.as_mut_ptr ());
  defsubr(Streesit_parser_embed_level.as_mut_ptr ());
  defsubr(Streesit_parser_set_embed_level.as_mut_ptr ());
  defsubr(Streesit_parser_changed_regions.as_mut_ptr ());

  defsubr(Streesit_parser_root_node.as_mut_ptr ());
  defsubr(Streesit_parse_string.as_mut_ptr ());

  defsubr(Streesit_parser_set_included_ranges.as_mut_ptr ());
  defsubr(Streesit_parser_included_ranges.as_mut_ptr ());

  defsubr(Streesit_parser_notifiers.as_mut_ptr ());
  defsubr(Streesit_parser_add_notifier.as_mut_ptr ());
  defsubr(Streesit_parser_remove_notifier.as_mut_ptr ());

  defsubr(Streesit_node_type.as_mut_ptr ());
  defsubr(Streesit_node_start.as_mut_ptr ());
  defsubr(Streesit_node_end.as_mut_ptr ());
  defsubr(Streesit_node_string.as_mut_ptr ());
  defsubr(Streesit_node_parent.as_mut_ptr ());
  defsubr(Streesit_node_child.as_mut_ptr ());
  defsubr(Streesit_node_check.as_mut_ptr ());
  defsubr(Streesit_node_field_name_for_child.as_mut_ptr ());
  defsubr(Streesit_node_child_count.as_mut_ptr ());
  defsubr(Streesit_node_child_by_field_name.as_mut_ptr ());
  defsubr(Streesit_node_next_sibling.as_mut_ptr ());
  defsubr(Streesit_node_prev_sibling.as_mut_ptr ());
  defsubr(Streesit_node_first_child_for_pos.as_mut_ptr ());
  defsubr(Streesit_node_descendant_for_range.as_mut_ptr ());
  defsubr(Streesit_node_eq.as_mut_ptr ());

  defsubr(Streesit_pattern_expand.as_mut_ptr ());
  defsubr(Streesit_query_expand.as_mut_ptr ());
  defsubr(Streesit_query_compile.as_mut_ptr ());
  defsubr(Streesit_query_capture.as_mut_ptr ());

  defsubr(Streesit_search_subtree.as_mut_ptr ());
  defsubr(Streesit_search_forward.as_mut_ptr ());
  defsubr(Streesit_induce_sparse_tree.as_mut_ptr ());
  defsubr(Streesit_node_match_p.as_mut_ptr ());
  defsubr(Streesit_subtree_stat.as_mut_ptr ());

  defsubr(Streesit__linecol_at.as_mut_ptr ());
  defsubr(Streesit__linecol_cache.as_mut_ptr ());
  defsubr(Streesit__linecol_cache_set.as_mut_ptr ());

  }
  defsubr(Streesit_available_p.as_mut_ptr ());
  #[cfg(feature = "windowsnt")]
  {
    DEFSYM!(Qtree_sitter__library_abi, "tree-sitter--library-abi");
    #[cfg(feature = "have_tree_sitter")]
    Fset(Qtree_sitter__library_abi(), make_fixnum(TREE_SITTER_LANGUAGE_VERSION as EMACS_INT));
    #[cfg(not(feature = "have_tree_sitter"))]
    Fset(Qtree_sitter__library_abi(), make_fixnum(-1));
  }

  }
}
