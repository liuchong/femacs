// femacs: port of src/comp.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The twenty-nine `DEFUN`s use `DEFUN!` (PORTING.md 5.1); C's non-`static` definitions
// carry `#[unsafe(no_mangle)]` (PORTING.md 0.2) and the C `static` helpers stay plain `unsafe fn`.
//
// **This is a P-86 unit, ported by transcription.**  `HAVE_NATIVE_COMP` is `undef`
// (`docs/baseline/config/darwin-arm64.macros.tsv` 1473), so C 25-5578 and C 5596-5854 are the body
// of a false `#ifdef` and `tools/extract-abi.py`'s live-line filter (P-73) leaves this file with an
// empty checklist; P-86 rules that such a file is transcribed, **both arms of every `#if` carried**,
// each item under the gate that names C's own macro, and that its ledger
// (`docs/ports/comp.md`) records a C line-range table instead of ticked boxes:
//
//   * the file gate (`#ifdef HAVE_NATIVE_COMP`, C 25) is `feature = "have_native_comp"`, on every
//     item *and on C 27-41's include graph* - which is why the frozen configuration compiles only
//     the live text at the bottom of this file;
//   * C 48-442's `#ifdef WINDOWSNT` arm is `feature = "windowsnt"` (nested in the file gate), the
//     name `src/vm-limit.rs` 95 already uses for that configuration;
//   * the `LIBGCCJIT_HAVE_*` conditions (C 156, 186, 233, 273, 305, 349, 354, 357, 386, 417, 431,
//     436, 533, 569, 609, 738, 752, 797) are gated by the lowercase feature that names the macro -
//     `libgccjit_have_gcc_jit_global_set_initializer`, `libgccjit_have_gcc_jit_context_new_bitcast`,
//     `libgccjit_have_reflection`, `libgccjit_have_gcc_jit_version`, plus `have_zlib`,
//     `ns_self_contained`, `lisp_object_is_struct` and `have__setjmp`.  **None of those eight is
//     registered** in `femacs/xtask/Cargo.toml`'s `[features]` table (only `have_native_comp` and
//     `windowsnt` are), so their non-frozen arms are text no build selects; `docs/ports/comp.md`
//     records that gap rather than adding the features here.
//
// **The `libgccjit.h` gap.**  C 31 includes `<libgccjit.h>`, a third-party header with no file
// anywhere in the reference tree.  The two other units in that class (`decompress.rs`, `lcms.rs`)
// carry the declarations they use in the unit (PORTING.md P-88); this slice does **not**: every
// `gcc_jit_*` name below is C's own name, left undeclared, and `docs/ports/comp.md` lists the set.
// The consequence is measured, not asserted: the arm's compile reading stops in name resolution
// (see that ledger's `## Evidence`), so this slice's arm evidence is `text-only` (P-86 clause 3).
//
// **Slice 1 covers C 25-822, slice 2 is arriving from C 824 on.**  The file is 5,857 lines and is being ported in slices; this is the
// first (the dynamic loading of libgccjit, this unit's data structures, and the hashing helpers),
// and it stops at `hash_native_abi`'s closing brace because C 824's `freloc_check_fill` is the
// first place the file needs `FOR_EACH_TAIL`, one of the macros `femacs/src/include/w32common.h`
// 72-97's neighbours keep `deferred` in `docs/ports/excluded-macros.txt` (P-22).  C 824-5857 is
// slice 2's and is recorded in the ledger as not yet transcribed.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]

include!("include/config.rs");

// C 23: `#include "lisp.h"` - outside the file gate, as C has it (C 25 opens after it).
include!("include/lisp.rs");

/* ----------------------------------------------------------------------------------------------
   C 25-822: the file gate's text, in C's order.  C 25 is `#ifdef HAVE_NATIVE_COMP`.  */

/* C 27-41: C's includes, inside the file gate.  Per P-86 clause 8 the gate goes on the include
   graph too, or the frozen configuration would compile C's dead text through the mirrors.

   `<setjmp.h>`, `<stdio.h>`, `<stdlib.h>` and `<signal.h>` reach this unit through `lisp.h`
   (`FILE` is `lib/include/stdio-impl.rs` 76's `__sFILE`, which `md5.rs`'s `md5_stream` already
   names), and `<zlib.h>`/`<epaths.h>`/`<libgccjit.h>` have no mirror: C 41's `<zlib.h>` is used by
   no line of this slice (the `HAVE_ZLIB` blocks below call `md5_gz_stream`/`md5_stream`, both of
   which take a `FILE *`), C 32's `<epaths.h>` is used by no line of this file at all (the grep for
   `EPATHS` finds only the include), and C 31's `<libgccjit.h>` is the gap recorded above.  */
#[cfg (feature = "have_native_comp")]
include!("include/window.rs");                  // C 34: `#include "window.h"`
#[cfg (feature = "have_native_comp")]
include!("include/dynlib.rs");                  // C 35: `#include "dynlib.h"`
/* `character.h` reaches C through `buffer.h`, whose `CHAR_STRING`/`UNIBYTE_TO_CHAR` family lives
   in it (`decompress.rs` and `dired.rs` include the mirror in the same order, PORTING.md 0.1-4:
   the arm-flip compile of the first revision stopped on `UNIBYTE_TO_CHAR` in the `buffer.rs`
   copy, `docs/ports/comp.md`'s `## Evidence` ③).  */
#[cfg (feature = "have_native_comp")]
include!("include/character.rs");               // buffer.h: the CHAR_STRING family
#[cfg (feature = "have_native_comp")]
include!("../lib/include/string.rs");           // lisp.h 28: <string.h> (strcpy), md5.h's memcpy
#[cfg (feature = "have_native_comp")]
include!("../lib/include/stdio.rs");            // C 29: `#include <stdio.h>` (vsnprintf)
#[cfg (feature = "have_native_comp")]
include!("include/buffer.rs");                  // C 36: `#include "buffer.h"`
#[cfg (feature = "have_native_comp")]
include!("include/blockinput.rs");              // C 37: `#include "blockinput.h"`
#[cfg (feature = "have_native_comp")]
include!("include/coding.rs");                  // C 38: `#include "coding.h"`
#[cfg (feature = "have_native_comp")]
include!("../lib/include/md5.rs");              // C 39: `#include "md5.h"`
#[cfg (feature = "have_native_comp")]
include!("include/sysstdio.rs");                // C 40: `#include "sysstdio.h"`
/* lisp.h 28's `#include <stdcountof.h>` - C reaches it through lisp.h (C 23), and this unit is
   the first to need `countof` (slice 2's `freloc_check_fill`).  Gated like this file's other
   includes, so the frozen configuration compiles none of it.  */
#[cfg (feature = "have_native_comp")]
include!("../lib/include/stdcountof.rs");       // lisp.h 28: `<stdcountof.h>` (`countof`)

/* C 31: `#include <libgccjit.h>` - the reference tree has no such file (P-88: a third-party
   header, the class `decompress.rs`'s `<zlib.h>` and `lcms.rs`'s `<lcms2.h>` are the first two
   of), so this unit carries the declarations its own lines use.  The eleven below are the ones this
   slice names; the header's other tags (`gcc_jit_result` 59, `gcc_jit_function_type` 112,
   `gcc_jit_vector_type` 115, `gcc_jit_case` 157) are named by no line of it.  `gcc_jit_object`
   joined the ten when `gcc_jit_type_is_pointer`'s prototype (the bitcast arm's) named it.  Every one of the ten is an *incomplete* tag in that header (`typedef struct
   gcc_jit_type gcc_jit_type;`, libgccjit.h 100), so C holds one only through a pointer and there
   is no layout to measure: the reading is that `sizeof` is rejected for all ten, and

       clang -std=gnu23 -I/opt/homebrew/include -fsyntax-only <probe>   # probe: ## Evidence in
                                                                       # docs/ports/comp.md

   reports ten `invalid application of 'sizeof' to an incomplete type` errors, while
   `_Static_assert (sizeof (gcc_jit_type *) == 8)` holds.  The port's zero-sized marker is that
   same incomplete type - the shape `callproc.rs` 206-207 uses for `_posix_spawn_file_actions` -
   and it is what keeps the `have_native_comp` arm honest: with the types named, the arm's
   compile reading stops being "cannot find type" (the 81 `E0425`s `docs/ports/comp.md` records).  */
/// C: `typedef struct gcc_jit_context gcc_jit_context;`  (libgccjit.h 56)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_context { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_object gcc_jit_object;`  (libgccjit.h 84)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_object { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_location gcc_jit_location;`  (libgccjit.h 97)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_location { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_type gcc_jit_type;`  (libgccjit.h 100)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_type { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_field gcc_jit_field;`  (libgccjit.h 105)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_field { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_struct gcc_jit_struct;`  (libgccjit.h 109)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_struct { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_function gcc_jit_function;`  (libgccjit.h 120)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_function { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_block gcc_jit_block;`  (libgccjit.h 138)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_block { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_rvalue gcc_jit_rvalue;`  (libgccjit.h 141)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_rvalue { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_lvalue gcc_jit_lvalue;`  (libgccjit.h 146)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_lvalue { _private: [u8; 0] }
/// C: `typedef struct gcc_jit_param gcc_jit_param;`  (libgccjit.h 151)
#[cfg (feature = "have_native_comp")]
#[repr(C)] pub struct gcc_jit_param { _private: [u8; 0] }

/* ==============================================================================================
   C 44-46: the banner comment C writes over the libgccjit dynamic loader.
   ============================================================================================== */

/* ==============================================================================================
   C 48-442: `#ifdef WINDOWSNT` - the lazy loading of libgccjit from the DLL.  `WINDOWSNT` is
   `undef` here (macros.tsv 4010), so no line below is compiled on this target (P-51 names the
   non-frozen side), and this machine has no Windows cross toolchain, so this arm's evidence is
   `text-only` (P-86 clause 3 (2)); `docs/ports/comp.md` records what it does have.
   ============================================================================================== */

// C 49: `# include "w32common.h"`.
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
include!("include/w32common.rs");

/* C 51-114: the sixty-four `#undef`s of the libgccjit entry points, which cancel the header's own
   function-like macros so that the `#define`s at C 366-441 can rename each one to this unit's
   `fn_gcc_jit_*` pointer.  They have no Rust counterpart (there is nothing to undef: the port never
   writes the header's macros), which is the `docs/ports/comp.md` `## Deviations` row P-86's
   `decompress.rs` precedent uses for the same construct.  */

/// C: w32common.h 77-79: `#define DEF_DLL_FN(type, func, ...)  typedef type (CDECL *W32_PFN_##func)`
/// `__VA_ARGS__; static W32_PFN_##func fn_##func` - the macro `femacs/src/include/w32common.rs`
/// 175-183 keeps `deferred` (P-22).  `decompress.rs` (the first consumer) and `lcms.rs` (the
/// second) settled the long form, and this unit is its third: C pastes `W32_PFN_##func` and
/// `fn_##func` from one identifier, which Rust cannot do, so both generated names are parameters
/// here and C's `__VA_ARGS__` (the parameter list, with any trailing attributes) is the `$args`
/// fragment.  The two items the macro makes are C's, name for name.
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
macro_rules! DEF_DLL_FN {
  ($type:ty, $func:ident, $pfn:ident, $fnvar:ident, ($($arg:ty),* $(,)?)) => {
    pub type $pfn = Option<unsafe extern "C" fn ($($arg),*) -> $type>;
    static mut $fnvar: $pfn = None;
  };
}
/// C: w32common.h 82-89: `#define LOAD_DLL_FN(lib, func)       do { fn_##func = (W32_PFN_##func)`
/// `get_proc_addr (lib, #func); if (!fn_##func) return false; } while (false)`.  `do { … } while
/// (false)` is a block expression (PORTING.md 2.1); C's `#func` is the stringified name, a
/// parameter here for the same reason as above; the assignment's cast between two function pointer
/// types is C's, and Rust has no `as` cast to an `Option<fn>` type, so it is the `transmute` that
/// the equal widths make the same operation.  `get_proc_addr` is the mirror's own inline
/// definition (`femacs/src/include/w32common.rs` 165).
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
macro_rules! LOAD_DLL_FN {
  ($lib:expr, $func:ident, $pfn:ident, $fnvar:ident, $name:expr) => {
    {
      $fnvar = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
      if $fnvar.is_none () { return false; }
    }
  };
}
/// C: w32common.h 92-97: `#define LOAD_DLL_FN_OPT(lib, func)  do { fn_##func = (W32_PFN_##func)`
/// `get_proc_addr (lib, #func); } while (false)` - the loader without the failure arm.
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
macro_rules! LOAD_DLL_FN_OPT {
  ($lib:expr, $func:ident, $pfn:ident, $fnvar:ident, $name:expr) => {
    {
      $fnvar = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
    }
  };
}
/* C 116-276: the sixty-four `DEF_DLL_FN` uses ("In alphabetical order"), each expanding to the two
   names C's macro pastes - `W32_PFN_gcc_jit_…`/`fn_gcc_jit_…`.  C's `CDECL` is the `__cdecl`
   attribute, empty on this target's ABI, so each pointer type is the plain `extern "C"` one.  The
   parameter types use C's own spellings: `gcc_jit_*` are the undeclared header types recorded in
   the file note above and `enum gcc_jit_*` are the header's enums, also undeclared here (C's
   `enum ` keyword drops, as everywhere in the port).  */
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_rvalue_from_int, W32_PFN_gcc_jit_context_new_rvalue_from_int, fn_gcc_jit_context_new_rvalue_from_int,
             (*mut gcc_jit_context, *mut gcc_jit_type, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_lvalue_as_rvalue, W32_PFN_gcc_jit_lvalue_as_rvalue, fn_gcc_jit_lvalue_as_rvalue,
             (*mut gcc_jit_lvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_rvalue_access_field, W32_PFN_gcc_jit_rvalue_access_field, fn_gcc_jit_rvalue_access_field,
             (*mut gcc_jit_rvalue, *mut gcc_jit_location, *mut gcc_jit_field));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_block_add_comment, W32_PFN_gcc_jit_block_add_comment, fn_gcc_jit_block_add_comment,
             (*mut gcc_jit_block, *mut gcc_jit_location, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_release, W32_PFN_gcc_jit_context_release, fn_gcc_jit_context_release,
             (*mut gcc_jit_context));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*const c_char, gcc_jit_context_get_first_error, W32_PFN_gcc_jit_context_get_first_error, fn_gcc_jit_context_get_first_error,
             (*mut gcc_jit_context));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_block, gcc_jit_function_new_block, W32_PFN_gcc_jit_function_new_block, fn_gcc_jit_function_new_block,
             (*mut gcc_jit_function, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_context, gcc_jit_context_acquire, W32_PFN_gcc_jit_context_acquire, fn_gcc_jit_context_acquire,
             ());
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_add_command_line_option, W32_PFN_gcc_jit_context_add_command_line_option, fn_gcc_jit_context_add_command_line_option,
             (*mut gcc_jit_context, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_add_driver_option, W32_PFN_gcc_jit_context_add_driver_option, fn_gcc_jit_context_add_driver_option,
             (*mut gcc_jit_context, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_field, gcc_jit_context_new_field, W32_PFN_gcc_jit_context_new_field, fn_gcc_jit_context_new_field,
             (*mut gcc_jit_context, *mut gcc_jit_location, *mut gcc_jit_type, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_function, gcc_jit_context_get_builtin_function, W32_PFN_gcc_jit_context_get_builtin_function, fn_gcc_jit_context_get_builtin_function,
             (*mut gcc_jit_context, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_function, gcc_jit_context_new_function, W32_PFN_gcc_jit_context_new_function, fn_gcc_jit_context_new_function,
             (*mut gcc_jit_context, *mut gcc_jit_location, gcc_jit_function_kind, *mut gcc_jit_type, *const c_char, c_int, *mut *mut gcc_jit_param, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_lvalue, gcc_jit_context_new_array_access, W32_PFN_gcc_jit_context_new_array_access, fn_gcc_jit_context_new_array_access,
             (*mut gcc_jit_context, *mut gcc_jit_location, *mut gcc_jit_rvalue, *mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_lvalue, gcc_jit_context_new_global, W32_PFN_gcc_jit_context_new_global, fn_gcc_jit_context_new_global,
             (*mut gcc_jit_context, *mut gcc_jit_location, gcc_jit_global_kind, *mut gcc_jit_type, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_lvalue, gcc_jit_function_new_local, W32_PFN_gcc_jit_function_new_local, fn_gcc_jit_function_new_local,
             (*mut gcc_jit_function, *mut gcc_jit_location, *mut gcc_jit_type, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_global_set_initializer"))]
DEF_DLL_FN! (*mut gcc_jit_lvalue, gcc_jit_global_set_initializer, W32_PFN_gcc_jit_global_set_initializer, fn_gcc_jit_global_set_initializer,
             (*mut gcc_jit_lvalue, *const c_void, usize));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_lvalue, gcc_jit_lvalue_access_field, W32_PFN_gcc_jit_lvalue_access_field, fn_gcc_jit_lvalue_access_field,
             (*mut gcc_jit_lvalue, *mut gcc_jit_location, *mut gcc_jit_field));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_lvalue, gcc_jit_param_as_lvalue, W32_PFN_gcc_jit_param_as_lvalue, fn_gcc_jit_param_as_lvalue,
             (*mut gcc_jit_param));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_lvalue, gcc_jit_rvalue_dereference, W32_PFN_gcc_jit_rvalue_dereference, fn_gcc_jit_rvalue_dereference,
             (*mut gcc_jit_rvalue, *mut gcc_jit_location));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_lvalue, gcc_jit_rvalue_dereference_field, W32_PFN_gcc_jit_rvalue_dereference_field, fn_gcc_jit_rvalue_dereference_field,
             (*mut gcc_jit_rvalue, *mut gcc_jit_location, *mut gcc_jit_field));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_param, gcc_jit_context_new_param, W32_PFN_gcc_jit_context_new_param, fn_gcc_jit_context_new_param,
             (*mut gcc_jit_context, *mut gcc_jit_location, *mut gcc_jit_type, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_param, gcc_jit_function_get_param, W32_PFN_gcc_jit_function_get_param, fn_gcc_jit_function_get_param,
             (*mut gcc_jit_function, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_binary_op, W32_PFN_gcc_jit_context_new_binary_op, fn_gcc_jit_context_new_binary_op,
             (*mut gcc_jit_context, *mut gcc_jit_location, gcc_jit_binary_op, *mut gcc_jit_type, *mut gcc_jit_rvalue, *mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_call, W32_PFN_gcc_jit_context_new_call, fn_gcc_jit_context_new_call,
             (*mut gcc_jit_context, *mut gcc_jit_location, *mut gcc_jit_function, c_int, *mut *mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_call_through_ptr, W32_PFN_gcc_jit_context_new_call_through_ptr, fn_gcc_jit_context_new_call_through_ptr,
             (*mut gcc_jit_context, *mut gcc_jit_location, *mut gcc_jit_rvalue, c_int, *mut *mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_cast, W32_PFN_gcc_jit_context_new_cast, fn_gcc_jit_context_new_cast,
             (*mut gcc_jit_context, *mut gcc_jit_location, *mut gcc_jit_rvalue, *mut gcc_jit_type));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_bitcast, W32_PFN_gcc_jit_context_new_bitcast, fn_gcc_jit_context_new_bitcast,
             (*mut gcc_jit_context, *mut gcc_jit_location, *mut gcc_jit_rvalue, *mut gcc_jit_type));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_comparison, W32_PFN_gcc_jit_context_new_comparison, fn_gcc_jit_context_new_comparison,
             (*mut gcc_jit_context, *mut gcc_jit_location, gcc_jit_comparison, *mut gcc_jit_rvalue, *mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_rvalue_from_long, W32_PFN_gcc_jit_context_new_rvalue_from_long, fn_gcc_jit_context_new_rvalue_from_long,
             (*mut gcc_jit_context, *mut gcc_jit_type, c_long));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_rvalue_from_ptr, W32_PFN_gcc_jit_context_new_rvalue_from_ptr, fn_gcc_jit_context_new_rvalue_from_ptr,
             (*mut gcc_jit_context, *mut gcc_jit_type, *mut c_void));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_string_literal, W32_PFN_gcc_jit_context_new_string_literal, fn_gcc_jit_context_new_string_literal,
             (*mut gcc_jit_context, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_context_new_unary_op, W32_PFN_gcc_jit_context_new_unary_op, fn_gcc_jit_context_new_unary_op,
             (*mut gcc_jit_context, *mut gcc_jit_location, gcc_jit_unary_op, *mut gcc_jit_type, *mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_lvalue_get_address, W32_PFN_gcc_jit_lvalue_get_address, fn_gcc_jit_lvalue_get_address,
             (*mut gcc_jit_lvalue, *mut gcc_jit_location));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_rvalue, gcc_jit_param_as_rvalue, W32_PFN_gcc_jit_param_as_rvalue, fn_gcc_jit_param_as_rvalue,
             (*mut gcc_jit_param));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_struct, gcc_jit_context_new_opaque_struct, W32_PFN_gcc_jit_context_new_opaque_struct, fn_gcc_jit_context_new_opaque_struct,
             (*mut gcc_jit_context, *mut gcc_jit_location, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_struct, gcc_jit_context_new_struct_type, W32_PFN_gcc_jit_context_new_struct_type, fn_gcc_jit_context_new_struct_type,
             (*mut gcc_jit_context, *mut gcc_jit_location, *const c_char, c_int, *mut *mut gcc_jit_field));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_context_get_int_type, W32_PFN_gcc_jit_context_get_int_type, fn_gcc_jit_context_get_int_type,
             (*mut gcc_jit_context, c_int, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_context_get_type, W32_PFN_gcc_jit_context_get_type, fn_gcc_jit_context_get_type,
             (*mut gcc_jit_context, gcc_jit_types));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_context_new_array_type, W32_PFN_gcc_jit_context_new_array_type, fn_gcc_jit_context_new_array_type,
             (*mut gcc_jit_context, *mut gcc_jit_location, *mut gcc_jit_type, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_context_new_function_ptr_type, W32_PFN_gcc_jit_context_new_function_ptr_type, fn_gcc_jit_context_new_function_ptr_type,
             (*mut gcc_jit_context, *mut gcc_jit_location, *mut gcc_jit_type, c_int, *mut *mut gcc_jit_type, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_context_new_union_type, W32_PFN_gcc_jit_context_new_union_type, fn_gcc_jit_context_new_union_type,
             (*mut gcc_jit_context, *mut gcc_jit_location, *const c_char, c_int, *mut *mut gcc_jit_field));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_rvalue_get_type, W32_PFN_gcc_jit_rvalue_get_type, fn_gcc_jit_rvalue_get_type,
             (*mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_struct_as_type, W32_PFN_gcc_jit_struct_as_type, fn_gcc_jit_struct_as_type,
             (*mut gcc_jit_struct));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_type_get_const, W32_PFN_gcc_jit_type_get_const, fn_gcc_jit_type_get_const,
             (*mut gcc_jit_type));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_type_get_pointer, W32_PFN_gcc_jit_type_get_pointer, fn_gcc_jit_type_get_pointer,
             (*mut gcc_jit_type));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_reflection"))]
DEF_DLL_FN! (*mut gcc_jit_type, gcc_jit_type_is_pointer, W32_PFN_gcc_jit_type_is_pointer, fn_gcc_jit_type_is_pointer,
             (*mut gcc_jit_type));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_block_add_assignment, W32_PFN_gcc_jit_block_add_assignment, fn_gcc_jit_block_add_assignment,
             (*mut gcc_jit_block, *mut gcc_jit_location, *mut gcc_jit_lvalue, *mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_block_add_eval, W32_PFN_gcc_jit_block_add_eval, fn_gcc_jit_block_add_eval,
             (*mut gcc_jit_block, *mut gcc_jit_location, *mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_block_end_with_conditional, W32_PFN_gcc_jit_block_end_with_conditional, fn_gcc_jit_block_end_with_conditional,
             (*mut gcc_jit_block, *mut gcc_jit_location, *mut gcc_jit_rvalue, *mut gcc_jit_block, *mut gcc_jit_block));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_block_end_with_jump, W32_PFN_gcc_jit_block_end_with_jump, fn_gcc_jit_block_end_with_jump,
             (*mut gcc_jit_block, *mut gcc_jit_location, *mut gcc_jit_block));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_block_end_with_return, W32_PFN_gcc_jit_block_end_with_return, fn_gcc_jit_block_end_with_return,
             (*mut gcc_jit_block, *mut gcc_jit_location, *mut gcc_jit_rvalue));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_block_end_with_void_return, W32_PFN_gcc_jit_block_end_with_void_return, fn_gcc_jit_block_end_with_void_return,
             (*mut gcc_jit_block, *mut gcc_jit_location));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_compile_to_file, W32_PFN_gcc_jit_context_compile_to_file, fn_gcc_jit_context_compile_to_file,
             (*mut gcc_jit_context, gcc_jit_output_kind, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_dump_reproducer_to_file, W32_PFN_gcc_jit_context_dump_reproducer_to_file, fn_gcc_jit_context_dump_reproducer_to_file,
             (*mut gcc_jit_context, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_dump_to_file, W32_PFN_gcc_jit_context_dump_to_file, fn_gcc_jit_context_dump_to_file,
             (*mut gcc_jit_context, *const c_char, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_set_bool_option, W32_PFN_gcc_jit_context_set_bool_option, fn_gcc_jit_context_set_bool_option,
             (*mut gcc_jit_context, gcc_jit_bool_option, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_set_int_option, W32_PFN_gcc_jit_context_set_int_option, fn_gcc_jit_context_set_int_option,
             (*mut gcc_jit_context, gcc_jit_int_option, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_set_logfile, W32_PFN_gcc_jit_context_set_logfile, fn_gcc_jit_context_set_logfile,
             (*mut gcc_jit_context, *mut FILE, c_int, c_int));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_context_set_str_option, W32_PFN_gcc_jit_context_set_str_option, fn_gcc_jit_context_set_str_option,
             (*mut gcc_jit_context, gcc_jit_str_option, *const c_char));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
DEF_DLL_FN! ((), gcc_jit_struct_set_fields, W32_PFN_gcc_jit_struct_set_fields, fn_gcc_jit_struct_set_fields,
             (*mut gcc_jit_struct, *mut gcc_jit_location, c_int, *mut *mut gcc_jit_field));
#[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_version"))]
DEF_DLL_FN! (c_int, gcc_jit_version_major, W32_PFN_gcc_jit_version_major, fn_gcc_jit_version_major,
             ());
#[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_version"))]
DEF_DLL_FN! (c_int, gcc_jit_version_minor, W32_PFN_gcc_jit_version_minor, fn_gcc_jit_version_minor,
             ());
#[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_version"))]
DEF_DLL_FN! (c_int, gcc_jit_version_patchlevel, W32_PFN_gcc_jit_version_patchlevel, fn_gcc_jit_version_patchlevel,
             ());

/// C 279-364: `init_gccjit_functions`.  `HMODULE` is a pointer; `w32_delayed_load` is the
/// declaration `femacs/src/include/w32.rs` carries for `w32.h` 187 (`extern HMODULE
/// w32_delayed_load (Lisp_Object);`).
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
unsafe fn init_gccjit_functions () -> bool
{
  let library: *mut c_void = w32_delayed_load (Qgccjit ());

  if library.is_null ()
    { return false; }

  /* In alphabetical order */
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_block_add_assignment, W32_PFN_gcc_jit_block_add_assignment, fn_gcc_jit_block_add_assignment, c"gcc_jit_block_add_assignment");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_block_add_comment, W32_PFN_gcc_jit_block_add_comment, fn_gcc_jit_block_add_comment, c"gcc_jit_block_add_comment");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_block_add_eval, W32_PFN_gcc_jit_block_add_eval, fn_gcc_jit_block_add_eval, c"gcc_jit_block_add_eval");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_block_end_with_conditional, W32_PFN_gcc_jit_block_end_with_conditional, fn_gcc_jit_block_end_with_conditional, c"gcc_jit_block_end_with_conditional");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_block_end_with_jump, W32_PFN_gcc_jit_block_end_with_jump, fn_gcc_jit_block_end_with_jump, c"gcc_jit_block_end_with_jump");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_block_end_with_return, W32_PFN_gcc_jit_block_end_with_return, fn_gcc_jit_block_end_with_return, c"gcc_jit_block_end_with_return");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_block_end_with_void_return, W32_PFN_gcc_jit_block_end_with_void_return, fn_gcc_jit_block_end_with_void_return, c"gcc_jit_block_end_with_void_return");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_acquire, W32_PFN_gcc_jit_context_acquire, fn_gcc_jit_context_acquire, c"gcc_jit_context_acquire");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_compile_to_file, W32_PFN_gcc_jit_context_compile_to_file, fn_gcc_jit_context_compile_to_file, c"gcc_jit_context_compile_to_file");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_dump_reproducer_to_file, W32_PFN_gcc_jit_context_dump_reproducer_to_file, fn_gcc_jit_context_dump_reproducer_to_file, c"gcc_jit_context_dump_reproducer_to_file");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_dump_to_file, W32_PFN_gcc_jit_context_dump_to_file, fn_gcc_jit_context_dump_to_file, c"gcc_jit_context_dump_to_file");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_get_builtin_function, W32_PFN_gcc_jit_context_get_builtin_function, fn_gcc_jit_context_get_builtin_function, c"gcc_jit_context_get_builtin_function");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_get_first_error, W32_PFN_gcc_jit_context_get_first_error, fn_gcc_jit_context_get_first_error, c"gcc_jit_context_get_first_error");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_get_int_type, W32_PFN_gcc_jit_context_get_int_type, fn_gcc_jit_context_get_int_type, c"gcc_jit_context_get_int_type");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_get_type, W32_PFN_gcc_jit_context_get_type, fn_gcc_jit_context_get_type, c"gcc_jit_context_get_type");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_array_access, W32_PFN_gcc_jit_context_new_array_access, fn_gcc_jit_context_new_array_access, c"gcc_jit_context_new_array_access");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_array_type, W32_PFN_gcc_jit_context_new_array_type, fn_gcc_jit_context_new_array_type, c"gcc_jit_context_new_array_type");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_bitcast, W32_PFN_gcc_jit_context_new_bitcast, fn_gcc_jit_context_new_bitcast, c"gcc_jit_context_new_bitcast");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_binary_op, W32_PFN_gcc_jit_context_new_binary_op, fn_gcc_jit_context_new_binary_op, c"gcc_jit_context_new_binary_op");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_call, W32_PFN_gcc_jit_context_new_call, fn_gcc_jit_context_new_call, c"gcc_jit_context_new_call");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_call_through_ptr, W32_PFN_gcc_jit_context_new_call_through_ptr, fn_gcc_jit_context_new_call_through_ptr, c"gcc_jit_context_new_call_through_ptr");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_cast, W32_PFN_gcc_jit_context_new_cast, fn_gcc_jit_context_new_cast, c"gcc_jit_context_new_cast");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_comparison, W32_PFN_gcc_jit_context_new_comparison, fn_gcc_jit_context_new_comparison, c"gcc_jit_context_new_comparison");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_field, W32_PFN_gcc_jit_context_new_field, fn_gcc_jit_context_new_field, c"gcc_jit_context_new_field");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_function, W32_PFN_gcc_jit_context_new_function, fn_gcc_jit_context_new_function, c"gcc_jit_context_new_function");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_function_ptr_type, W32_PFN_gcc_jit_context_new_function_ptr_type, fn_gcc_jit_context_new_function_ptr_type, c"gcc_jit_context_new_function_ptr_type");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_global, W32_PFN_gcc_jit_context_new_global, fn_gcc_jit_context_new_global, c"gcc_jit_context_new_global");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_opaque_struct, W32_PFN_gcc_jit_context_new_opaque_struct, fn_gcc_jit_context_new_opaque_struct, c"gcc_jit_context_new_opaque_struct");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_param, W32_PFN_gcc_jit_context_new_param, fn_gcc_jit_context_new_param, c"gcc_jit_context_new_param");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_rvalue_from_int, W32_PFN_gcc_jit_context_new_rvalue_from_int, fn_gcc_jit_context_new_rvalue_from_int, c"gcc_jit_context_new_rvalue_from_int");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_rvalue_from_long, W32_PFN_gcc_jit_context_new_rvalue_from_long, fn_gcc_jit_context_new_rvalue_from_long, c"gcc_jit_context_new_rvalue_from_long");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_rvalue_from_ptr, W32_PFN_gcc_jit_context_new_rvalue_from_ptr, fn_gcc_jit_context_new_rvalue_from_ptr, c"gcc_jit_context_new_rvalue_from_ptr");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_string_literal, W32_PFN_gcc_jit_context_new_string_literal, fn_gcc_jit_context_new_string_literal, c"gcc_jit_context_new_string_literal");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_struct_type, W32_PFN_gcc_jit_context_new_struct_type, fn_gcc_jit_context_new_struct_type, c"gcc_jit_context_new_struct_type");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_unary_op, W32_PFN_gcc_jit_context_new_unary_op, fn_gcc_jit_context_new_unary_op, c"gcc_jit_context_new_unary_op");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_new_union_type, W32_PFN_gcc_jit_context_new_union_type, fn_gcc_jit_context_new_union_type, c"gcc_jit_context_new_union_type");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_release, W32_PFN_gcc_jit_context_release, fn_gcc_jit_context_release, c"gcc_jit_context_release");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_set_bool_option, W32_PFN_gcc_jit_context_set_bool_option, fn_gcc_jit_context_set_bool_option, c"gcc_jit_context_set_bool_option");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_set_int_option, W32_PFN_gcc_jit_context_set_int_option, fn_gcc_jit_context_set_int_option, c"gcc_jit_context_set_int_option");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_set_logfile, W32_PFN_gcc_jit_context_set_logfile, fn_gcc_jit_context_set_logfile, c"gcc_jit_context_set_logfile");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_context_set_str_option, W32_PFN_gcc_jit_context_set_str_option, fn_gcc_jit_context_set_str_option, c"gcc_jit_context_set_str_option");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_function_get_param, W32_PFN_gcc_jit_function_get_param, fn_gcc_jit_function_get_param, c"gcc_jit_function_get_param");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_function_new_block, W32_PFN_gcc_jit_function_new_block, fn_gcc_jit_function_new_block, c"gcc_jit_function_new_block");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_function_new_local, W32_PFN_gcc_jit_function_new_local, fn_gcc_jit_function_new_local, c"gcc_jit_function_new_local");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_lvalue_access_field, W32_PFN_gcc_jit_lvalue_access_field, fn_gcc_jit_lvalue_access_field, c"gcc_jit_lvalue_access_field");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_lvalue_as_rvalue, W32_PFN_gcc_jit_lvalue_as_rvalue, fn_gcc_jit_lvalue_as_rvalue, c"gcc_jit_lvalue_as_rvalue");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_lvalue_get_address, W32_PFN_gcc_jit_lvalue_get_address, fn_gcc_jit_lvalue_get_address, c"gcc_jit_lvalue_get_address");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_param_as_lvalue, W32_PFN_gcc_jit_param_as_lvalue, fn_gcc_jit_param_as_lvalue, c"gcc_jit_param_as_lvalue");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_param_as_rvalue, W32_PFN_gcc_jit_param_as_rvalue, fn_gcc_jit_param_as_rvalue, c"gcc_jit_param_as_rvalue");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_rvalue_access_field, W32_PFN_gcc_jit_rvalue_access_field, fn_gcc_jit_rvalue_access_field, c"gcc_jit_rvalue_access_field");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_rvalue_dereference, W32_PFN_gcc_jit_rvalue_dereference, fn_gcc_jit_rvalue_dereference, c"gcc_jit_rvalue_dereference");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_rvalue_dereference_field, W32_PFN_gcc_jit_rvalue_dereference_field, fn_gcc_jit_rvalue_dereference_field, c"gcc_jit_rvalue_dereference_field");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_rvalue_get_type, W32_PFN_gcc_jit_rvalue_get_type, fn_gcc_jit_rvalue_get_type, c"gcc_jit_rvalue_get_type");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_struct_as_type, W32_PFN_gcc_jit_struct_as_type, fn_gcc_jit_struct_as_type, c"gcc_jit_struct_as_type");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_struct_set_fields, W32_PFN_gcc_jit_struct_set_fields, fn_gcc_jit_struct_set_fields, c"gcc_jit_struct_set_fields");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_type_get_const, W32_PFN_gcc_jit_type_get_const, fn_gcc_jit_type_get_const, c"gcc_jit_type_get_const");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, gcc_jit_type_get_pointer, W32_PFN_gcc_jit_type_get_pointer, fn_gcc_jit_type_get_pointer, c"gcc_jit_type_get_pointer");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_reflection"))]
  LOAD_DLL_FN! (library, gcc_jit_type_is_pointer, W32_PFN_gcc_jit_type_is_pointer, fn_gcc_jit_type_is_pointer, c"gcc_jit_type_is_pointer");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN_OPT! (library, gcc_jit_context_add_command_line_option, W32_PFN_gcc_jit_context_add_command_line_option, fn_gcc_jit_context_add_command_line_option, c"gcc_jit_context_add_command_line_option");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
  LOAD_DLL_FN_OPT! (library, gcc_jit_context_add_driver_option, W32_PFN_gcc_jit_context_add_driver_option, fn_gcc_jit_context_add_driver_option, c"gcc_jit_context_add_driver_option");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_global_set_initializer"))]
  LOAD_DLL_FN_OPT! (library, gcc_jit_global_set_initializer, W32_PFN_gcc_jit_global_set_initializer, fn_gcc_jit_global_set_initializer, c"gcc_jit_global_set_initializer");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_version"))]
  LOAD_DLL_FN_OPT! (library, gcc_jit_version_major, W32_PFN_gcc_jit_version_major, fn_gcc_jit_version_major, c"gcc_jit_version_major");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_version"))]
  LOAD_DLL_FN_OPT! (library, gcc_jit_version_minor, W32_PFN_gcc_jit_version_minor, fn_gcc_jit_version_minor, c"gcc_jit_version_minor");
  #[cfg (all (feature = "have_native_comp", feature = "windowsnt", feature = "libgccjit_have_gcc_jit_version"))]
  LOAD_DLL_FN_OPT! (library, gcc_jit_version_patchlevel, W32_PFN_gcc_jit_version_patchlevel, fn_gcc_jit_version_patchlevel, c"gcc_jit_version_patchlevel");

  return true;
}

/* C 366-441: the `#define gcc_jit_x fn_gcc_jit_x` renames, sixty-four of them.  An object-like
   `#define` that renames an identifier has no Rust counterpart (PORTING.md 2.1): a `macro_rules!`
   resolves its names where it is defined, not where it is used, so no single definition can stand
   for `fn_gcc_jit_x` here and `gcc_jit_x` elsewhere.  `decompress.rs`'s `#define inflate
   fn_inflate` row is the precedent: the call sites are spelled the way C's preprocessor leaves
   them, and the two sides are the two arms of the `windowsnt` condition - on this gate
   `(fn_gcc_jit_x.unwrap ()) (…)`, outside it `gcc_jit_x (…)`.  No call site of this slice is under
   the gate, so nothing in this file carries the `unix` spelling yet; the slices that add them
   carry both.  `docs/ports/comp.md` records the row.  */

/* C 442: `#endif` - the `WINDOWSNT` arm ends here; C 444 begins the next function.  */

/// C 444-467: `load_gccjit_if_necessary` - C's `static bool load_gccjit_if_necessary (bool
/// mandatory)`, so the port keeps it a plain `unsafe fn` with no exported symbol (the `extract-abi`
/// reading of the first revision reported `#[unsafe(no_mangle)] but the reference does not define a
/// non-static function of that name`; `docs/ports/comp.md`'s `## Evidence` ④).  C's two arms are the
/// `windowsnt` condition's (C 447/464/466): the DLL path and the `return true` every other
/// configuration takes.
#[cfg (feature = "have_native_comp")]
unsafe fn load_gccjit_if_necessary (mandatory: bool) -> bool
{
  #[cfg (feature = "windowsnt")]
  {
    static mut tried_to_initialize_once: bool = false;
    static mut gccjit_initialized: bool = false;

    if !tried_to_initialize_once
      {
        tried_to_initialize_once = true;
        let mut status: Lisp_Object;
        gccjit_initialized = init_gccjit_functions ();
        status = if gccjit_initialized { Qt () } else { Qnil () };
        Vlibrary_cache = Fcons (Fcons (Qgccjit (), status), Vlibrary_cache);
      }

    if mandatory && !gccjit_initialized
      { xsignal1 (Qnative_compiler_error (), build_string (c"libgccjit not found".as_ptr ())); }

    return gccjit_initialized;
  }
  #[cfg (not (feature = "windowsnt"))]
  {
    return true;
  }
}

/* ==============================================================================================
   C 470-531: this file's own object-like macros and its relocation records.
   ============================================================================================== */

/* Increase this number to force a new Vcomp_abi_hash to be generated.  */
/// C 470: `#define ABI_VERSION "13"` - the string `hash_native_abi` (C 765) hands to `build_string`,
/// so the port keeps C's `const char *` shape (PORTING.md 2.1's string-macro rule).
#[cfg (feature = "have_native_comp")]
const ABI_VERSION: *const c_char = c"13".as_ptr ();

/* Length of the hashes used for eln file naming.  */
/// C 473: `#define HASH_LENGTH 8` - `make_fixnum`'s argument at C 727/762, hence `EMACS_INT`.
#[cfg (feature = "have_native_comp")]
const HASH_LENGTH: EMACS_INT = 8;

/* C symbols emitted for the load relocation mechanism.  */
/// C 476: `#define CURRENT_THREAD_RELOC_SYM "current_thread_reloc"`
#[cfg (feature = "have_native_comp")]
const CURRENT_THREAD_RELOC_SYM: *const c_char = c"current_thread_reloc".as_ptr ();
/// C 477: `#define F_SYMBOLS_WITH_POS_ENABLED_RELOC_SYM "f_symbols_with_pos_enabled_reloc"`
#[cfg (feature = "have_native_comp")]
const F_SYMBOLS_WITH_POS_ENABLED_RELOC_SYM: *const c_char = c"f_symbols_with_pos_enabled_reloc".as_ptr ();
/// C 478: `#define DATA_RELOC_SYM "d_reloc"`
#[cfg (feature = "have_native_comp")]
const DATA_RELOC_SYM: *const c_char = c"d_reloc".as_ptr ();
/// C 479: `#define DATA_RELOC_ADDR_SYM "d_reloc_addr"`
#[cfg (feature = "have_native_comp")]
const DATA_RELOC_ADDR_SYM: *const c_char = c"d_reloc_addr".as_ptr ();
/// C 480: `#define DATA_RELOC_EPHEMERAL_SYM "d_reloc_eph"`
#[cfg (feature = "have_native_comp")]
const DATA_RELOC_EPHEMERAL_SYM: *const c_char = c"d_reloc_eph".as_ptr ();
/// C 481: `#define DATA_RELOC_EPHEMERAL_ADDR_SYM "d_reloc_eph_addr"`
#[cfg (feature = "have_native_comp")]
const DATA_RELOC_EPHEMERAL_ADDR_SYM: *const c_char = c"d_reloc_eph_addr".as_ptr ();

/// C 483: `#define FUNC_LINK_TABLE_SYM "freloc_link_table"`
#[cfg (feature = "have_native_comp")]
const FUNC_LINK_TABLE_SYM: *const c_char = c"freloc_link_table".as_ptr ();
/// C 484: `#define LINK_TABLE_HASH_SYM "freloc_hash"`
#[cfg (feature = "have_native_comp")]
const LINK_TABLE_HASH_SYM: *const c_char = c"freloc_hash".as_ptr ();
/// C 485: `#define COMP_UNIT_SYM "comp_unit"`
#[cfg (feature = "have_native_comp")]
const COMP_UNIT_SYM: *const c_char = c"comp_unit".as_ptr ();
/// C 486: `#define TEXT_DATA_RELOC_SYM "text_data_reloc"`
#[cfg (feature = "have_native_comp")]
const TEXT_DATA_RELOC_SYM: *const c_char = c"text_data_reloc".as_ptr ();
/// C 487: `#define TEXT_DATA_RELOC_EPHEMERAL_SYM "text_data_reloc_eph"`
#[cfg (feature = "have_native_comp")]
const TEXT_DATA_RELOC_EPHEMERAL_SYM: *const c_char = c"text_data_reloc_eph".as_ptr ();

/// C 489: `#define TEXT_OPTIM_QLY_SYM "text_optim_qly"`
#[cfg (feature = "have_native_comp")]
const TEXT_OPTIM_QLY_SYM: *const c_char = c"text_optim_qly".as_ptr ();
/// C 490: `#define TEXT_FDOC_SYM "text_data_fdoc"`
#[cfg (feature = "have_native_comp")]
const TEXT_FDOC_SYM: *const c_char = c"text_data_fdoc".as_ptr ();

/// C 493-495: `#define STR_VALUE(s) #s` / `#define STR(s) STR_VALUE (s)` - C's stringification,
/// which the port writes with `stringify!` (PORTING.md 2.1).  `STR`'s extra level is C's (it is
/// what makes `STR (a_macro)` expand `a_macro` first); a `macro_rules!` expands its argument's
/// tokens the same way, so the two levels are kept as the two macros C has.
#[cfg (feature = "have_native_comp")]
macro_rules! STR_VALUE { ($($s:tt)+) => { stringify! ($($s)+) } }
#[cfg (feature = "have_native_comp")]
macro_rules! STR { ($($s:tt)+) => { STR_VALUE! ($($s)+) } }

/// C 497-504: the three list accessors C builds on `XCAR`/`XCDR`.
#[cfg (feature = "have_native_comp")]
macro_rules! FIRST { ($x:expr) => { XCAR ($x) } }
#[cfg (feature = "have_native_comp")]
macro_rules! SECOND { ($x:expr) => { XCAR (XCDR ($x)) } }
#[cfg (feature = "have_native_comp")]
macro_rules! THIRD { ($x:expr) => { XCAR (XCDR (XCDR ($x))) } }

/* Like calln but stringify and intern.  */
/// C 508-509: `#define CALLNI(fun, ...) calln (intern_c_string (STR (fun)), __VA_ARGS__)`.
/// **Deviation (recorded):** C's `STR (fun)` stringifies a *token sequence* - the only call site
/// in the file is C 884's `CALLNI (comp-mvar-slot, mvar)`, whose name lexes as five tokens - and
/// Rust's `stringify!` puts a space between each of them (`"comp - mvar - slot"`), so the call site
/// passes the string C's preprocessor produces and this macro takes a literal.  Recorded in
/// `docs/ports/comp.md`'s `## Deviations`; the same class as `decompress.rs`'s
/// `#define inflate fn_inflate` row.
#[cfg (feature = "have_native_comp")]
macro_rules! CALLNI {
  ($fun:literal $(, $arg:expr)* $(,)?) => {
    calln! (intern_c_string (concat! ($fun, "\0").as_ptr () as *const c_char) $(, $arg)*)
  };
}

/// C 511-513: `#define DECL_BLOCK(name, func) gcc_jit_block *(name) = gcc_jit_function_new_block
/// (func, STR (name))`.  C's `gcc_jit_block *` declarator becomes the `let` binding's type.  The
/// callee is one of the names C 366-441 renames on the `windowsnt` arm, and a `macro_rules!` body
/// resolves its names where it is *defined*, so the two arms need the two definitions below -
/// which is what C's preprocessor does to the one body (`decompress.rs`'s `inflateInit2` row).
#[cfg (all (feature = "have_native_comp", not (feature = "windowsnt")))]
macro_rules! DECL_BLOCK {
  ($name:ident, $func:expr) => {
    let $name: *mut gcc_jit_block = gcc_jit_function_new_block ($func,
      concat! (STR! ($name), "\0").as_ptr () as *const c_char);
  };
}
#[cfg (all (feature = "have_native_comp", feature = "windowsnt"))]
macro_rules! DECL_BLOCK {
  ($name:ident, $func:expr) => {
    let $name: *mut gcc_jit_block = (fn_gcc_jit_function_new_block.unwrap ()) ($func,
      concat! (STR! ($name), "\0").as_ptr () as *const c_char);
  };
}

/* C 515-526: the `SETJMP`/`SETJMP_NAME` macros.  The four-arm condition picks the setjmp
   implementation (C 517 `#ifndef WINDOWSNT` -> C 518 `#ifdef HAVE__SETJMP` -> `_setjmp` else
   `setjmp`; C 523 `#else` -> `_setjmp`) and C 526 binds `SETJMP_NAME` to it.  **No Rust
   counterpart:** like `decompress.rs`'s `#define inflate fn_inflate`, an object-like `#define` that
   renames an identifier has none - a `macro_rules!` resolves names where it is defined, not where
   it is used - so the call sites are spelled the way C's preprocessor leaves them
   (`_setjmp (…)`/`setjmp (…)`), and `SETJMP_NAME` is likewise a call-site spelling.  `HAVE__SETJMP`
   is `1` in the frozen configuration (macros.tsv 1722) and `WINDOWSNT` is `undef`, so the arm this
   target would take is C 519's `SETJMP` = `setjmp`; the two other arms are the `windowsnt` and
   `have__setjmp` features' text (P-51).  No line of this slice calls `SETJMP`, so no call site
   carries a spelling yet; `docs/ports/comp.md`'s `## Deviations` records the row.  */

/* Max number function importable by native-compiled code.  */
/// C 529: `#define F_RELOC_MAX_SIZE 1600` - the width of `f_reloc_t`'s array below.
#[cfg (feature = "have_native_comp")]
const F_RELOC_MAX_SIZE: usize = 1600;

/// C 531-534: `f_reloc_t` - `void *link_table[F_RELOC_MAX_SIZE]` and the `ptrdiff_t size` C fills
/// in as entries are added.
#[cfg (feature = "have_native_comp")]
#[repr(C)]
pub struct f_reloc_t
{
  pub link_table: [*mut c_void; F_RELOC_MAX_SIZE],
  pub size: ptrdiff_t,
}

/// C 536: `static f_reloc_t freloc;` - C's tentative definition, i.e. zeroed (PORTING.md 0.1-13).
#[cfg (feature = "have_native_comp")]
static mut freloc: f_reloc_t = f_reloc_t { link_table: [std::ptr::null_mut (); F_RELOC_MAX_SIZE],
                                           size: 0 };

/// C 538-540: `#ifndef LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast` / `# define NUM_CAST_TYPES 15`.
/// The macro is undefined in the frozen configuration's table (`tools/check-cond-eval.py` is the
/// measurement; the header that could define it is not included there at all), so the frozen side
/// is the C 542-657 fields that use it; the other side is the feature that names "defined".
#[cfg (all (feature = "have_native_comp", not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
const NUM_CAST_TYPES: usize = 15;

/// C 542-545: `reloc_array_t` - the length and the `gcc_jit_rvalue *` array this unit synthesizes
/// for data relocations.
#[cfg (feature = "have_native_comp")]
#[repr(C)]
#[derive(Clone, Copy)]   // C copies this record by value (`reloc.array = comp.data_relocs;`, C 917)
pub struct reloc_array_t
{
  pub len: EMACS_INT,
  pub r_val: *mut gcc_jit_rvalue,
}

/* C side of the compiler context.  */

/// C 547-657: `comp_t` - the port's C-side compiler context, field for field and in C's order.
/// Three of the fields C writes are inside C 610-617's `#ifndef
/// LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast` (five fields: the cast union, the 15x15 cast
/// function table, the two int/pointer cast functions and the 15 cast types), so they carry that
/// gate.  C 569-572's `#ifdef LISP_OBJECT_IS_STRUCT` pair is `undef` in the frozen configuration
/// (`lisp.h` 590's `#define` sits inside a condition this target leaves closed) and its two fields
/// are the `lisp_object_is_struct` feature's text - carried, as the other arms are.
///
/// **Layout: measured, not remembered.**  `clang` against this machine's header
/// (`/opt/homebrew/include/libgccjit.h` -> `Cellar/libgccjit/16.2.0/include/libgccjit.h`) reads
/// `sizeof (comp_t)` **2680**, `_Alignof` **8** with the cast fields present, and **736**/8 with
/// them absent - the two numbers the two `const _` asserts below pin.  The probe, its command and
/// its output are in `docs/ports/comp.md`'s `## Evidence`.
#[cfg (feature = "have_native_comp")]
#[repr(C)]
pub struct comp_t
{
  pub speed: EMACS_INT,
  pub debug: EMACS_INT,
  pub compiler_options: Lisp_Object,
  pub driver_options: Lisp_Object,
  pub ctxt: *mut gcc_jit_context,
  pub void_type: *mut gcc_jit_type,
  pub bool_type: *mut gcc_jit_type,
  pub char_type: *mut gcc_jit_type,
  pub int_type: *mut gcc_jit_type,
  pub unsigned_type: *mut gcc_jit_type,
  pub long_type: *mut gcc_jit_type,
  pub unsigned_long_type: *mut gcc_jit_type,
  pub long_long_type: *mut gcc_jit_type,
  pub unsigned_long_long_type: *mut gcc_jit_type,
  pub emacs_int_type: *mut gcc_jit_type,
  pub emacs_uint_type: *mut gcc_jit_type,
  pub void_ptr_type: *mut gcc_jit_type,
  pub bool_ptr_type: *mut gcc_jit_type,
  pub char_ptr_type: *mut gcc_jit_type,
  pub ptrdiff_type: *mut gcc_jit_type,
  pub uintptr_type: *mut gcc_jit_type,
  pub size_t_type: *mut gcc_jit_type,
  pub lisp_word_type: *mut gcc_jit_type,
  pub lisp_word_tag_type: *mut gcc_jit_type,
  /* C 569-572: `#ifdef LISP_OBJECT_IS_STRUCT` (dead on this target).  */
  #[cfg (feature = "lisp_object_is_struct")]
  pub lisp_obj_i: *mut gcc_jit_field,
  #[cfg (feature = "lisp_object_is_struct")]
  pub lisp_obj_s: *mut gcc_jit_struct,
  pub lisp_obj_type: *mut gcc_jit_type,
  pub lisp_obj_ptr_type: *mut gcc_jit_type,
  /* struct Lisp_Cons */
  pub lisp_cons_s: *mut gcc_jit_struct,
  pub lisp_cons_u: *mut gcc_jit_field,
  pub lisp_cons_u_s: *mut gcc_jit_field,
  pub lisp_cons_u_s_car: *mut gcc_jit_field,
  pub lisp_cons_u_s_u: *mut gcc_jit_field,
  pub lisp_cons_u_s_u_cdr: *mut gcc_jit_field,
  pub lisp_cons_type: *mut gcc_jit_type,
  pub lisp_cons_ptr_type: *mut gcc_jit_type,
  /* struct Lisp_Symbol_With_Position */
  pub f_symbols_with_pos_enabled_ref: *mut gcc_jit_rvalue,
  pub lisp_symbol_with_position: *mut gcc_jit_struct,
  pub lisp_symbol_with_position_header: *mut gcc_jit_field,
  pub lisp_symbol_with_position_sym: *mut gcc_jit_field,
  pub lisp_symbol_with_position_pos: *mut gcc_jit_field,
  pub lisp_symbol_with_position_type: *mut gcc_jit_type,
  pub lisp_symbol_with_position_ptr_type: *mut gcc_jit_type,
  pub get_symbol_with_position: *mut gcc_jit_function,
  pub symbol_with_pos_sym: *mut gcc_jit_function,
  /* struct jmp_buf.  */
  pub jmp_buf_s: *mut gcc_jit_struct,
  /* struct handler.  */
  pub handler_s: *mut gcc_jit_struct,
  pub handler_jmp_field: *mut gcc_jit_field,
  pub handler_val_field: *mut gcc_jit_field,
  pub handler_next_field: *mut gcc_jit_field,
  pub handler_ptr_type: *mut gcc_jit_type,
  pub loc_handler: *mut gcc_jit_lvalue,
  /* struct thread_state.  */
  pub thread_state_s: *mut gcc_jit_struct,
  pub m_handlerlist: *mut gcc_jit_field,
  pub thread_state_ptr_type: *mut gcc_jit_type,
  pub current_thread_ref: *mut gcc_jit_rvalue,
  /* Other globals.  */
  /* C 610-617: `#ifndef LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast`, the dead-cast workaround.  */
  #[cfg (not (feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
  pub cast_union_type: *mut gcc_jit_type,
  #[cfg (not (feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
  pub cast_functions_from_to: [[*mut gcc_jit_function; NUM_CAST_TYPES]; NUM_CAST_TYPES],
  #[cfg (not (feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
  pub cast_ptr_to_int: *mut gcc_jit_function,
  #[cfg (not (feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
  pub cast_int_to_ptr: *mut gcc_jit_function,
  #[cfg (not (feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
  pub cast_types: [*mut gcc_jit_type; NUM_CAST_TYPES],
  pub func: *mut gcc_jit_function,      /* Current function being compiled.  */
  pub func_has_non_local: bool,         /* From comp-func has-non-local slot.  */
  pub func_speed: EMACS_INT,            /* From comp-func speed slot.  */
  pub func_safety: EMACS_INT,           /* From comp-func safety slot.  */
  pub block: *mut gcc_jit_block,        /* Current basic block being compiled.  */
  pub scratch: *mut gcc_jit_lvalue,     /* Used as scratch slot for some code sequence (switch).  */
  pub frame_size: ptrdiff_t,            /* Size of the following array in elements. */
  pub frame: *mut *mut gcc_jit_lvalue,  /* Frame slot n -> gcc_jit_lvalue *.  */
  pub zero: *mut gcc_jit_rvalue,
  pub one: *mut gcc_jit_rvalue,
  pub inttypebits: *mut gcc_jit_rvalue,
  pub lisp_int0: *mut gcc_jit_rvalue,
  pub pseudovectorp: *mut gcc_jit_function,
  pub bool_to_lisp_obj: *mut gcc_jit_function,
  pub add1: *mut gcc_jit_function,
  pub sub1: *mut gcc_jit_function,
  pub negate: *mut gcc_jit_function,
  pub eq: *mut gcc_jit_function,
  pub car: *mut gcc_jit_function,
  pub cdr: *mut gcc_jit_function,
  pub setcar: *mut gcc_jit_function,
  pub setcdr: *mut gcc_jit_function,
  pub check_type: *mut gcc_jit_function,
  pub maybe_gc_or_quit: *mut gcc_jit_function,
  pub func_blocks_h: Lisp_Object,       /* blk_name -> gcc_block.  */
  pub exported_funcs_h: Lisp_Object,    /* c-func-name -> gcc_jit_function *.  */
  pub imported_funcs_h: Lisp_Object,    /* subr_name -> gcc_jit_field *reloc_field.  */
  pub emitter_dispatcher: Lisp_Object,
  /* Synthesized struct holding data relocs.  */
  pub data_relocs: reloc_array_t,
  /* Same as before but content does not survive load phase. */
  pub data_relocs_ephemeral: reloc_array_t,
  /* Global structure holding function relocations.  */
  pub func_relocs: *mut gcc_jit_lvalue,
  pub func_relocs_ptr_type: *mut gcc_jit_type,
  /* Pointer to this structure local to each function.  */
  pub func_relocs_local: *mut gcc_jit_lvalue,
  pub memcpy: *mut gcc_jit_function,
  pub d_default_idx: Lisp_Object,
  pub d_ephemeral_idx: Lisp_Object,
}
/* C 544-657's layout, as clang reads it (see the note above): the frozen configuration's side of
   C 610's condition is the one with the cast fields.  */
#[cfg (all (feature = "have_native_comp", not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
const _: () = assert! (std::mem::size_of::<comp_t> () == 2680);
#[cfg (all (feature = "have_native_comp", not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
const _: () = assert! (std::mem::align_of::<comp_t> () == 8);
#[cfg (all (feature = "have_native_comp", not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
const _: () = assert! (std::mem::offset_of! (comp_t, speed) == 0);
#[cfg (all (feature = "have_native_comp", not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
const _: () = assert! (std::mem::offset_of! (comp_t, cast_union_type) == 432);
#[cfg (all (feature = "have_native_comp", not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
const _: () = assert! (std::mem::offset_of! (comp_t, func_speed) == 2392);
#[cfg (all (feature = "have_native_comp", not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
const _: () = assert! (std::mem::offset_of! (comp_t, block) == 2408);
#[cfg (all (feature = "have_native_comp", not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
const _: () = assert! (std::mem::offset_of! (comp_t, data_relocs) == 2600);
#[cfg (all (feature = "have_native_comp", not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
const _: () = assert! (std::mem::offset_of! (comp_t, d_ephemeral_idx) == 2672);
/* The other side of the same condition, measured with the header's own state
   (`LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast` is what libgccjit 16.2.0 defines).  */
#[cfg (all (feature = "have_native_comp", feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
const _: () = assert! (std::mem::size_of::<comp_t> () == 736);
#[cfg (all (feature = "have_native_comp", feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
const _: () = assert! (std::mem::align_of::<comp_t> () == 8);
#[cfg (all (feature = "have_native_comp", feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
const _: () = assert! (std::mem::offset_of! (comp_t, func_speed) == 448);
#[cfg (all (feature = "have_native_comp", feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
const _: () = assert! (std::mem::offset_of! (comp_t, block) == 464);
#[cfg (all (feature = "have_native_comp", feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
const _: () = assert! (std::mem::offset_of! (comp_t, data_relocs) == 656);
#[cfg (all (feature = "have_native_comp", feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
const _: () = assert! (std::mem::offset_of! (comp_t, d_ephemeral_idx) == 728);

/// C 660: `static comp_t comp;` - C's tentative definition, i.e. zeroed (PORTING.md 0.1-13).
#[cfg (feature = "have_native_comp")]
static mut comp: comp_t = unsafe { std::mem::zeroed () };

/// C 662: `static FILE *logfile;` - C's tentative definition, i.e. a null pointer.
#[cfg (feature = "have_native_comp")]
static mut logfile: *mut FILE = std::ptr::null_mut ();

/// C 664-668: `static_obj_t` - a length and a flexible `char data[]` array (PORTING.md 1.3's
/// zero-length spelling, the `Lisp_Vector` family's in `lisp.rs`).
#[cfg (feature = "have_native_comp")]
#[repr(C)]
pub struct static_obj_t
{
  pub len: ptrdiff_t,
  pub data: [c_char; 0],               // FLEXIBLE_ARRAY_MEMBER
}

/// C 670-674: `imm_reloc_t` - a `reloc_array_t` and the index into it.
#[cfg (feature = "have_native_comp")]
#[repr(C)]
pub struct imm_reloc_t
{
  pub array: reloc_array_t,
  pub idx: *mut gcc_jit_rvalue,
}
/* The three records above, as clang reads them (the probe is in `docs/ports/comp.md`).  */
#[cfg (feature = "have_native_comp")]
const _: () = assert! (std::mem::size_of::<f_reloc_t> () == 12808);
#[cfg (feature = "have_native_comp")]
const _: () = assert! (std::mem::align_of::<f_reloc_t> () == 8);
#[cfg (feature = "have_native_comp")]
const _: () = assert! (std::mem::offset_of! (f_reloc_t, size) == 12800);
#[cfg (feature = "have_native_comp")]
const _: () = assert! (std::mem::size_of::<reloc_array_t> () == 16);
#[cfg (feature = "have_native_comp")]
const _: () = assert! (std::mem::offset_of! (reloc_array_t, r_val) == 8);
#[cfg (feature = "have_native_comp")]
const _: () = assert! (std::mem::size_of::<static_obj_t> () == 8);
#[cfg (feature = "have_native_comp")]
const _: () = assert! (std::mem::offset_of! (static_obj_t, data) == 8);
#[cfg (feature = "have_native_comp")]
const _: () = assert! (std::mem::size_of::<imm_reloc_t> () == 24);
#[cfg (feature = "have_native_comp")]
const _: () = assert! (std::mem::offset_of! (imm_reloc_t, idx) == 16);

/*
   Helper functions called by the run-time.
*/

/* C 680-687: C's forward declarations of this unit's own `static` helpers.  A C forward
   declaration has no Rust counterpart when the definition follows in the same unit (the
   `alloc.rs` 471-474 row), but here the definitions are slice 2's, and C 689-707 takes their
   addresses, so the declarations stand - in a module, per PORTING.md 0.10, so that the
   definitions that land in this unit's crate root later shadow them rather than colliding
   (`E0428`).  They are `extern "C"` because C takes their address and the native-compiled code
   calls them through that pointer, which is the C ABI (the definitions slice 2 adds must keep it).

   The three names after them are the frozen mirrors' gaps this slice reaches (P-72 rule 4's
   use-site declaration, the shape `femacs/src/emacs.rs` 193 and `femacs/src/pdumper.rs` 143 use
   for `hexbuf_digest`): `push_handler` and `record_unwind_protect_excursion` are declared in
   `lisp.h` 4812/4826 but not in `include/lisp.rs`, and `suffix_p`, `emacs_fopen`, `md5_gz_stream`
   and `hexbuf_digest` are `lisp.h` 4740/5322/5455/4247, carried by no mirror.  */
#[cfg (feature = "have_native_comp")]
pub mod comp_helpers
{
  use super::*;

  unsafe extern "C"
  {
  /// C: comp.c 680: `static void helper_unwind_protect (Lisp_Object);`
  pub fn helper_unwind_protect (arg: Lisp_Object);
  /// C: comp.c 681: `static Lisp_Object helper_unbind_n (Lisp_Object);`
  pub fn helper_unbind_n (n: Lisp_Object) -> Lisp_Object;
  /// C: comp.c 682: `static void helper_save_restriction (void);`
  pub fn helper_save_restriction ();
  /// C: comp.c 683: `static bool helper_PSEUDOVECTOR_TYPEP_XUNTAG (Lisp_Object, enum pvec_type);`
  pub fn helper_PSEUDOVECTOR_TYPEP_XUNTAG (a: Lisp_Object, type_: pvec_type) -> bool;
  /// C: comp.c 684-685: `static struct Lisp_Symbol_With_Pos *`
  /// `helper_GET_SYMBOL_WITH_POSITION (Lisp_Object);`
  pub fn helper_GET_SYMBOL_WITH_POSITION (a: Lisp_Object) -> *mut Lisp_Symbol_With_Pos;
  /// C: comp.c 686-687: `static Lisp_Object` / `helper_sanitizer_assert (Lisp_Object,
  /// Lisp_Object);`
  pub fn helper_sanitizer_assert (x: Lisp_Object, y: Lisp_Object) -> Lisp_Object;

  /// C: lisp.h 4812: `extern struct handler *push_handler (Lisp_Object, enum handlertype)`
  /// `ATTRIBUTE_MALLOC ATTRIBUTE_RETURNS_NONNULL;`
  pub fn push_handler (tag: Lisp_Object, type_: handlertype) -> *mut handler;
  /// C: lisp.h 4826: `extern void record_unwind_protect_excursion (void);`
  pub fn record_unwind_protect_excursion ();
  /// C: lisp.h 4740: `extern bool suffix_p (Lisp_Object, const char *);`
  pub fn suffix_p (string: Lisp_Object, suffix: *const c_char) -> bool;
  /// C: lisp.h 5322: `extern FILE *emacs_fopen (char const *, char const *)`
  /// `ATTRIBUTE_MALLOC ATTRIBUTE_DEALLOC (emacs_fclose, 1);`
  pub fn emacs_fopen (file: *const c_char, mode: *const c_char) -> *mut FILE;
  /// C: lisp.h 5455: `extern int md5_gz_stream (FILE *, void *);`
  pub fn md5_gz_stream (source: *mut FILE, resblock: *mut c_void) -> c_int;
  /// C: lisp.h 4247: `extern void hexbuf_digest (char *, void const *, int);`
  pub fn hexbuf_digest (buf: *mut c_char, data: *const c_void, size: c_int);
  /// C: lisp.h 4206: `extern void set_internal (Lisp_Object, Lisp_Object, Lisp_Object,`
  /// `enum set_internal_bind);`
  pub fn set_internal (symbol: Lisp_Object, newval: Lisp_Object, r#let: Lisp_Object,
                       bindflag: Set_Internal_Bind);
  }
}
#[cfg (feature = "have_native_comp")]
use comp_helpers::*;

/* Note: helper_link_table must match the list created by
   `declare_runtime_imported_funcs'.  */
/// C 689-707: `static void *helper_link_table[]` - the fifteen addresses, in C's order.
#[cfg (feature = "have_native_comp")]
static mut helper_link_table: [*mut c_void; 15] =
  [ wrong_type_argument as *mut c_void,
    helper_PSEUDOVECTOR_TYPEP_XUNTAG as *mut c_void,
    push_handler as *mut c_void,
    record_unwind_protect_excursion as *mut c_void,
    helper_unbind_n as *mut c_void,
    helper_save_restriction as *mut c_void,
    helper_GET_SYMBOL_WITH_POSITION as *mut c_void,
    slow_eq as *mut c_void,
    helper_sanitizer_assert as *mut c_void,
    record_unwind_current_buffer as *mut c_void,
    set_internal as *mut c_void,
    helper_unwind_protect as *mut c_void,
    specbind as *mut c_void,
    maybe_gc as *mut c_void,
    maybe_quit as *mut c_void ];

/* C 29's `<stdio.h>`: `lib/include/stdio.rs` is the frozen target's *system* header mirror and
   carries `FILE`/`fclose`/the md5-stream entries but not `vsnprintf`; the gnulib *template* mirror
   `lib/include/stdio.in.rs` 301 does carry it, but no unit reaches it (`rg -ln 'stdio.in.rs'
   femacs/src femacs/lib` is empty, `femacs/src/android.rs` 1347-1348's note), so the unit declares
   it here (the `android.rs` 1351-1358 / `sysdep.rs` in-unit shape).  Its `va_list` parameter is the
   `core::ffi::VaList<'_>` the tree's other variadic callees use (`pdumper.rs` 126).  */
#[cfg (feature = "have_native_comp")]
unsafe extern "C"
{
  /// C: `<stdio.h>`: `int vsnprintf (char *, size_t, const char *, va_list);`
  pub fn vsnprintf (s: *mut c_char, n: usize, format: *const c_char,
                    ap: core::ffi::VaList<'_>) -> c_int;
}

/// C 709-720: `format_string` - C's `static char * ATTRIBUTE_FORMAT_PRINTF (1, 2) format_string
/// (const char *format, ...)`.  `ATTRIBUTE_FORMAT_PRINTF` is a declaration attribute, not part of
/// the value, so it has no port counterpart; C's `va_list va; va_start (va, format); … va_end (va)`
/// have none either (PORTING.md P-04's row - the `...` parameter *is* the list, the
/// `femacs/src/alloc.rs` 2643-2650 `make_formatted_string` and `pdumper.rs` 330-336 `dump_trace`
/// shapes); `va_start` is the port's `let ap: core::ffi::VaList<'_> = args;` and `va_end` has none.  `scratch_area` is C's
/// function-static buffer, and C reads `res >= sizeof (scratch_area)` as a signed/unsigned
/// comparison of `int` against the array's size, so the port converts the same way.
#[cfg (feature = "have_native_comp")]
unsafe extern "C" fn format_string (format: *const c_char, mut args: ...) -> *mut c_char
{
  static mut scratch_area: [c_char; 512] = [0; 512];
  let ap: core::ffi::VaList<'_> = args;
  let res: c_int = vsnprintf (scratch_area.as_mut_ptr (),
                              std::mem::size_of_val (&scratch_area), format, ap);
  if res as usize >= std::mem::size_of_val (&scratch_area)
    { strcpy (scratch_area.as_mut_ptr ().offset (std::mem::size_of_val (&scratch_area) as isize - 4),
              c"...".as_ptr ()); }
  return scratch_area.as_mut_ptr ();
}

/// C 722-730: `comp_hash_string` - the md5 of a Lisp string, hex-encoded into a substring of the
/// first `HASH_LENGTH` characters.
#[cfg (feature = "have_native_comp")]
unsafe fn comp_hash_string (string: Lisp_Object) -> Lisp_Object
{
  let mut digest: Lisp_Object = make_uninit_string ((MD5_DIGEST_SIZE * 2) as EMACS_INT);
  md5_buffer (SSDATA (string), SBYTES (string) as usize, SSDATA (digest) as *mut c_void);
  hexbuf_digest (SSDATA (digest), SDATA (digest) as *const c_void, MD5_DIGEST_SIZE);

  return Fsubstring (digest, Qnil (), make_fixnum (HASH_LENGTH));
}

/// C 732-767: `comp_hash_source_file` - the same for a file's contents, with the two `HAVE_ZLIB`
/// blocks C 738-743 and C 752-758 (the compressed-file guard and the two hashing calls) carried
/// under the `have_zlib` feature that names the other side of `#ifdef HAVE_ZLIB` - `undef` in the
/// frozen configuration (macros.tsv 1721).
#[cfg (feature = "have_native_comp")]
unsafe fn comp_hash_source_file (filename: Lisp_Object) -> Lisp_Object
{
  /* Can't use Finsert_file_contents + Fbuffer_hash as this is called
     by Fcomp_el_to_eln_filename too early during bootstrap.  */
  let is_gz: bool = suffix_p (filename, c".gz".as_ptr ());
  #[cfg (not (feature = "have_zlib"))]
  {
    if is_gz
      { xsignal2 (Qfile_notify_error (), build_string (c"Cannot natively compile compressed *.el files without zlib support".as_ptr ()), filename); }
  }
  let encoded_filename: Lisp_Object = ENCODE_FILE (filename);
  let mut f: *mut FILE = emacs_fopen (SSDATA (encoded_filename),
                                      if is_gz { c"rb".as_ptr () } else { c"r".as_ptr () });

  if f.is_null ()
    { report_file_error (c"Opening source file".as_ptr (), filename); }

  let mut digest: Lisp_Object = make_uninit_string ((MD5_DIGEST_SIZE * 2) as EMACS_INT);

  /* C declares `int res` inside each arm of C 752's `#ifdef`, but a C declaration's scope is the
     enclosing block, not the arm - so the port declares it once here and the arms assign it
     (the arm-flip compile of the first revision reported `cannot find value res` at C 761;
     docs/ports/comp.md's `## Evidence` ③).  */
  let mut res: c_int = 0;
  #[cfg (feature = "have_zlib")]
  {
    res = if is_gz
      { md5_gz_stream (f, SSDATA (digest) as *mut c_void) }
      else
      { md5_stream (f, SSDATA (digest) as *mut c_void) };
  }
  #[cfg (not (feature = "have_zlib"))]
  {
    res = md5_stream (f, SSDATA (digest) as *mut c_void);
  }
  emacs_fclose! (f);

  if res != 0
    { xsignal2 (Qfile_notify_error (), build_string (c"hashing failed".as_ptr ()), filename); }

  hexbuf_digest (SSDATA (digest), SDATA (digest) as *const c_void, MD5_DIGEST_SIZE);

  return Fsubstring (digest, Qnil (), make_fixnum (HASH_LENGTH));
}

/* C 769-777: `DEFUN ("comp--subr-signature", …)`.  */
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp--subr-signature", Fcomp__subr_signature,
       Scomp__subr_signature, 1, 1, 0,
       doc: "Support function to hash_native_abi.\nFor internal use.",
       (subr: Lisp_Object),
       {
  return concat2 (Fsubr_name (subr),
		  Fprin1_to_string (Fsubr_arity (subr), Qnil (), Qnil ()));
});

/* Produce a key hashing Vcomp_subr_list.  */

/// C 781-822: `hash_native_abi`.  C 797-818's `#ifdef NS_SELF_CONTAINED` arm is dead on this
/// target (macros.tsv has no `NS_SELF_CONTAINED`; it is `emacs.c`'s own "self contained app bundle"
/// configuration) and is carried under the feature that names the other side; C 820-822's
/// `Vcomp_native_version_dir` assignment is outside the `#ifdef`, as C has it.
#[cfg (feature = "have_native_comp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hash_native_abi ()
{
  /* Check runs once.  */
  eassert! (NILP (Vcomp_abi_hash));

  Vcomp_abi_hash =
    comp_hash_string (
      concat3 (build_string (ABI_VERSION),
	       concat3 (Vemacs_version, Vsystem_configuration,
			Vsystem_configuration_options),
	       Fmapconcat (intern_c_string (c"comp--subr-signature".as_ptr ()),
			   Vcomp_subr_list, build_string (c"".as_ptr ()))));

  let mut version: Lisp_Object = Vemacs_version;

  #[cfg (feature = "ns_self_contained")]
  {
    /* MacOS self contained app bundles do not like having dots in the
       directory names under the Contents/Frameworks directory, so
       convert them to underscores.  */
    version = if STRING_MULTIBYTE (Vemacs_version)
      { make_uninit_multibyte_string (SCHARS (Vemacs_version), SBYTES (Vemacs_version)) }
      else
      { make_uninit_string (SBYTES (Vemacs_version)) };

    let from: *const c_uchar = SDATA (Vemacs_version);
    let mut to: *mut c_uchar = SDATA (version);

    while from < SDATA (Vemacs_version).offset (SBYTES (Vemacs_version))
      {
        let mut c: c_uchar = *from;
        from = from.offset (1);

        if c == b'.' as c_uchar
	  { c = b'_' as c_uchar; }

        *to = c;
        to = to.offset (1);
      }
  }

  Vcomp_native_version_dir =
    concat3 (version, build_string (c"-".as_ptr ()), Vcomp_abi_hash);
}

/* ==============================================================================================
   C 5579-5590: the first live text of this file - `Fnative_comp_available_p`, outside the file
   gate (the gate closed at C 5578).  Both arms of C 5585's `#ifdef` are carried: C 5586 is the
   arm this target does not compile, C 5588 the one it does.
   ============================================================================================== */
DEFUN! ("native-comp-available-p", Fnative_comp_available_p,
       Snative_comp_available_p, 0, 0, 0,
       doc: "Return non-nil if native compilation support is built-in.",
       (),
       {
  #[cfg (feature = "have_native_comp")]
  { return if load_gccjit_if_necessary (false) { Qt () } else { Qnil () }; }
  #[cfg (not (feature = "have_native_comp"))]
  { return Qnil (); }
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

/* ==============================================================================================
   C 824-5857: **slice 2**.  It starts here because C 824's `freloc_check_fill` is the file's first
   `FOR_EACH_TAIL` (the mirror's `FOR_EACH_TAIL!`, the `syntax` row of
   `docs/ports/excluded-macros.txt`) and the first `goto`; the slice is arriving in tranches and
   `docs/ports/comp.md` carries the ledger rows.
   ============================================================================================== */

/// C 504-505: `#define CALLNI(fun, ...) calln (intern_c_string (STR (fun)), __VA_ARGS__)` - this
/// file's own macro (C 507's `DECL_BLOCK` is written at its uses and `FIXNUM` is the mirror's).
/// C's `STR (fun)` stringifies the token; a Rust macro cannot, so the call sites pass the same
/// text as a `c"..."` literal.
macro_rules! CALLNI {
  ($fun:expr, $($args:expr),* $(,)?) => {
    calln! (intern_c_string ($fun), $($args),*)
  };
}

/* C 511-517's `#ifndef WINDOWSNT` arm takes these straight from `<libgccjit.h>`, while C 48-442's
   `WINDOWSNT` arm loads the same names into `fn_gcc_jit_*` pointers - so the declarations below
   are the half that is *not* `windowsnt`.  `<libgccjit.h>` is the third-party header P-88
   describes (`comp.rs`'s file note), and each prototype carries that header's own line.  */
#[cfg (all (feature = "have_native_comp", not (feature = "windowsnt")))]
unsafe extern "C" {
  /// C: libgccjit.h 644-645: `gcc_jit_type *gcc_jit_type_get_const (gcc_jit_type *type);`
  pub fn gcc_jit_type_get_const (type_: *mut gcc_jit_type) -> *mut gcc_jit_type;
  /// C: libgccjit.h 701-704: `gcc_jit_field *gcc_jit_context_new_field (gcc_jit_context *ctxt,
  /// gcc_jit_location *loc, gcc_jit_type *type, const char *name);`
  pub fn gcc_jit_context_new_field (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                    type_: *mut gcc_jit_type, name: *const c_char)
    -> *mut gcc_jit_field;
  /// C: libgccjit.h 771-776: `gcc_jit_type *gcc_jit_context_new_function_ptr_type
  /// (gcc_jit_context *ctxt, gcc_jit_location *loc, gcc_jit_type *return_type, int num_params,
  /// gcc_jit_type **param_types, int is_variadic);`
  pub fn gcc_jit_context_new_function_ptr_type (ctxt: *mut gcc_jit_context,
                                                loc: *mut gcc_jit_location,
                                                return_type: *mut gcc_jit_type,
                                                num_params: c_int,
                                                param_types: *mut *mut gcc_jit_type,
                                                is_variadic: c_int) -> *mut gcc_jit_type;
  /// C: libgccjit.h 874-875: `gcc_jit_block *gcc_jit_function_new_block (gcc_jit_function *func,
  /// const char *name);`
  pub fn gcc_jit_function_new_block (func: *mut gcc_jit_function, name: *const c_char)
    -> *mut gcc_jit_block;
  /// C: libgccjit.h 1112-1114: `gcc_jit_rvalue *gcc_jit_context_new_rvalue_from_int
  /// (gcc_jit_context *ctxt, gcc_jit_type *numeric_type, int value);`
  pub fn gcc_jit_context_new_rvalue_from_int (ctxt: *mut gcc_jit_context,
                                              numeric_type: *mut gcc_jit_type, value: c_int)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1495-1498: `gcc_jit_lvalue *gcc_jit_function_new_local (gcc_jit_function *func,
  /// gcc_jit_location *loc, gcc_jit_type *type, const char *name);`
  pub fn gcc_jit_function_new_local (func: *mut gcc_jit_function, loc: *mut gcc_jit_location,
                                     type_: *mut gcc_jit_type, name: *const c_char)
    -> *mut gcc_jit_lvalue;
  /// C: libgccjit.h 1561-1563: `void gcc_jit_block_add_comment (gcc_jit_block *block,
  /// gcc_jit_location *loc, const char *text);`
  pub fn gcc_jit_block_add_comment (block: *mut gcc_jit_block, loc: *mut gcc_jit_location,
                                    text: *const c_char);
  /// C: libgccjit.h 1102-1103: `gcc_jit_rvalue *gcc_jit_lvalue_as_rvalue (gcc_jit_lvalue *lvalue);`
  pub fn gcc_jit_lvalue_as_rvalue (lvalue: *mut gcc_jit_lvalue) -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1108-1109: `gcc_jit_type *gcc_jit_rvalue_get_type (gcc_jit_rvalue *rvalue);`
  pub fn gcc_jit_rvalue_get_type (rvalue: *mut gcc_jit_rvalue) -> *mut gcc_jit_type;
  /// C: libgccjit.h 1201-1206: `gcc_jit_rvalue *gcc_jit_context_new_unary_op
  /// (gcc_jit_context *ctxt, gcc_jit_location *loc, enum gcc_jit_unary_op op,
  /// gcc_jit_type *result_type, gcc_jit_rvalue *rvalue);`
  pub fn gcc_jit_context_new_unary_op (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                       op: gcc_jit_unary_op, result_type: *mut gcc_jit_type,
                                       rvalue: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1315-1319: `gcc_jit_rvalue *gcc_jit_context_new_call (gcc_jit_context *ctxt,
  /// gcc_jit_location *loc, gcc_jit_function *func, int numargs, gcc_jit_rvalue **args);`
  pub fn gcc_jit_context_new_call (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                   func: *mut gcc_jit_function, numargs: c_int,
                                   args: *mut *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1322-1326: `gcc_jit_rvalue *gcc_jit_context_new_call_through_ptr
  /// (gcc_jit_context *ctxt, gcc_jit_location *loc, gcc_jit_rvalue *fn_ptr, int numargs,
  /// gcc_jit_rvalue **args);`
  pub fn gcc_jit_context_new_call_through_ptr (ctxt: *mut gcc_jit_context,
                                               loc: *mut gcc_jit_location,
                                               fn_ptr: *mut gcc_jit_rvalue, numargs: c_int,
                                               args: *mut *mut gcc_jit_rvalue)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1439-1442: `gcc_jit_lvalue *gcc_jit_rvalue_dereference_field
  /// (gcc_jit_rvalue *ptr, gcc_jit_location *loc, gcc_jit_field *field);`
  pub fn gcc_jit_rvalue_dereference_field (ptr: *mut gcc_jit_rvalue, loc: *mut gcc_jit_location,
                                           field: *mut gcc_jit_field) -> *mut gcc_jit_lvalue;
  /// C: libgccjit.h 1454-1456: `gcc_jit_rvalue *gcc_jit_lvalue_get_address (gcc_jit_lvalue *lvalue,
  /// gcc_jit_location *loc);`
  pub fn gcc_jit_lvalue_get_address (lvalue: *mut gcc_jit_lvalue, loc: *mut gcc_jit_location)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1577-1582: `void gcc_jit_block_end_with_conditional (gcc_jit_block *block,
  /// gcc_jit_location *loc, gcc_jit_rvalue *boolval, gcc_jit_block *on_true,
  /// gcc_jit_block *on_false);`
  pub fn gcc_jit_block_end_with_conditional (block: *mut gcc_jit_block, loc: *mut gcc_jit_location,
                                             boolval: *mut gcc_jit_rvalue,
                                             on_true: *mut gcc_jit_block,
                                             on_false: *mut gcc_jit_block);
  /// C: libgccjit.h 1275-1280: `gcc_jit_rvalue *gcc_jit_context_new_binary_op
  /// (gcc_jit_context *ctxt, gcc_jit_location *loc, enum gcc_jit_binary_op op,
  /// gcc_jit_type *result_type, gcc_jit_rvalue *a, gcc_jit_rvalue *b);`
  pub fn gcc_jit_context_new_binary_op (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                        op: gcc_jit_binary_op, result_type: *mut gcc_jit_type,
                                        a: *mut gcc_jit_rvalue, b: *mut gcc_jit_rvalue)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1423-1426: `gcc_jit_lvalue *gcc_jit_lvalue_access_field
  /// (gcc_jit_lvalue *struct_or_union, gcc_jit_location *loc, gcc_jit_field *field);`
  pub fn gcc_jit_lvalue_access_field (struct_or_union: *mut gcc_jit_lvalue,
                                      loc: *mut gcc_jit_location, field: *mut gcc_jit_field)
    -> *mut gcc_jit_lvalue;
  /// C: libgccjit.h 1431-1434: `gcc_jit_rvalue *gcc_jit_rvalue_access_field
  /// (gcc_jit_rvalue *struct_or_union, gcc_jit_location *loc, gcc_jit_field *field);`
  pub fn gcc_jit_rvalue_access_field (struct_or_union: *mut gcc_jit_rvalue,
                                      loc: *mut gcc_jit_location, field: *mut gcc_jit_field)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1531-1535: `void gcc_jit_block_add_assignment (gcc_jit_block *block,
  /// gcc_jit_location *loc, gcc_jit_lvalue *lvalue, gcc_jit_rvalue *rvalue);`
  pub fn gcc_jit_block_add_assignment (block: *mut gcc_jit_block, loc: *mut gcc_jit_location,
                                       lvalue: *mut gcc_jit_lvalue, rvalue: *mut gcc_jit_rvalue);
  /* C 1155-1183's `#ifdef LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast` arm calls these three;
     the macro is undefined in the frozen configuration, so that arm is the one no flip reaches,
     and the declarations are here for the configuration that defines it.  */
  /// C: libgccjit.h 641-642: `gcc_jit_object *gcc_jit_type_is_pointer (gcc_jit_type *type);`
  pub fn gcc_jit_type_is_pointer (type_: *mut gcc_jit_type) -> *mut gcc_jit_object;
  /// C: libgccjit.h 1333-1337: `gcc_jit_rvalue *gcc_jit_context_new_cast (gcc_jit_context *ctxt,
  /// gcc_jit_location *loc, gcc_jit_rvalue *rvalue, gcc_jit_type *type);`
  pub fn gcc_jit_context_new_cast (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                   rvalue: *mut gcc_jit_rvalue, type_: *mut gcc_jit_type)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1907-1910: `gcc_jit_rvalue *gcc_jit_context_new_bitcast (gcc_jit_context *ctxt,
  /// gcc_jit_location *loc, gcc_jit_rvalue *rvalue, gcc_jit_type *type);`
  pub fn gcc_jit_context_new_bitcast (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                      rvalue: *mut gcc_jit_rvalue, type_: *mut gcc_jit_type)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 640-641: `gcc_jit_type *gcc_jit_type_get_pointer (gcc_jit_type *type);`
  pub fn gcc_jit_type_get_pointer (type_: *mut gcc_jit_type) -> *mut gcc_jit_type;
  /// C: libgccjit.h 741-742: `gcc_jit_type *gcc_jit_struct_as_type (gcc_jit_struct *struct_type);`
  pub fn gcc_jit_struct_as_type (struct_type: *mut gcc_jit_struct) -> *mut gcc_jit_type;
  /// C: libgccjit.h 1117-1120: `gcc_jit_rvalue *gcc_jit_context_new_rvalue_from_long
  /// (gcc_jit_context *ctxt, gcc_jit_type *numeric_type, long value);`
  pub fn gcc_jit_context_new_rvalue_from_long (ctxt: *mut gcc_jit_context,
                                               numeric_type: *mut gcc_jit_type, value: c_long)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1137-1140: `gcc_jit_rvalue *gcc_jit_context_new_rvalue_from_ptr
  /// (gcc_jit_context *ctxt, gcc_jit_type *pointer_type, void *value);`
  pub fn gcc_jit_context_new_rvalue_from_ptr (ctxt: *mut gcc_jit_context,
                                              pointer_type: *mut gcc_jit_type, value: *mut c_void)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1306-1310: `gcc_jit_rvalue *gcc_jit_context_new_comparison
  /// (gcc_jit_context *ctxt, gcc_jit_location *loc, enum gcc_jit_comparison op,
  /// gcc_jit_rvalue *a, gcc_jit_rvalue *b);`
  pub fn gcc_jit_context_new_comparison (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                         op: gcc_jit_comparison, a: *mut gcc_jit_rvalue,
                                         b: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1447-1449: `gcc_jit_lvalue *gcc_jit_rvalue_dereference (gcc_jit_rvalue *rvalue,
  /// gcc_jit_location *loc);`
  pub fn gcc_jit_rvalue_dereference (rvalue: *mut gcc_jit_rvalue, loc: *mut gcc_jit_location)
    -> *mut gcc_jit_lvalue;
  /// C: libgccjit.h 852-854: `gcc_jit_function *gcc_jit_context_get_builtin_function
  /// (gcc_jit_context *ctxt, const char *name);`
  pub fn gcc_jit_context_get_builtin_function (ctxt: *mut gcc_jit_context, name: *const c_char)
    -> *mut gcc_jit_function;
  /// C: libgccjit.h 1373-1377: `gcc_jit_lvalue *gcc_jit_context_new_array_access
  /// (gcc_jit_context *ctxt, gcc_jit_location *loc, gcc_jit_rvalue *ptr, gcc_jit_rvalue *index);`
  pub fn gcc_jit_context_new_array_access (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                           ptr: *mut gcc_jit_rvalue, index: *mut gcc_jit_rvalue)
    -> *mut gcc_jit_lvalue;
  /// C: libgccjit.h 1519-1522: `void gcc_jit_block_add_eval (gcc_jit_block *block,
  /// gcc_jit_location *loc, gcc_jit_rvalue *rvalue);`
  pub fn gcc_jit_block_add_eval (block: *mut gcc_jit_block, loc: *mut gcc_jit_location,
                                 rvalue: *mut gcc_jit_rvalue);
  /// C: libgccjit.h 678-682: `gcc_jit_type *gcc_jit_context_new_array_type (gcc_jit_context *ctxt,
  /// gcc_jit_location *loc, gcc_jit_type *element_type, int num_elements);`
  pub fn gcc_jit_context_new_array_type (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                         element_type: *mut gcc_jit_type, num_elements: c_int)
    -> *mut gcc_jit_type;
  /// C: libgccjit.h 783-787: `gcc_jit_param *gcc_jit_context_new_param (gcc_jit_context *ctxt,
  /// gcc_jit_location *loc, gcc_jit_type *type, const char *name);`
  pub fn gcc_jit_context_new_param (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                    type_: *mut gcc_jit_type, name: *const c_char)
    -> *mut gcc_jit_param;
  /// C: libgccjit.h 840-848: `gcc_jit_function *gcc_jit_context_new_function (gcc_jit_context *ctxt,
  /// gcc_jit_location *loc, enum gcc_jit_function_kind kind, gcc_jit_type *return_type,
  /// const char *name, int num_params, gcc_jit_param **params, int is_variadic);`
  pub fn gcc_jit_context_new_function (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                       kind: gcc_jit_function_kind,
                                       return_type: *mut gcc_jit_type, name: *const c_char,
                                       num_params: c_int, params: *mut *mut gcc_jit_param,
                                       is_variadic: c_int) -> *mut gcc_jit_function;
  /// C: libgccjit.h 794-795: `gcc_jit_lvalue *gcc_jit_param_as_lvalue (gcc_jit_param *param);`
  pub fn gcc_jit_param_as_lvalue (param: *mut gcc_jit_param) -> *mut gcc_jit_lvalue;
  /// C: libgccjit.h 798-799: `gcc_jit_rvalue *gcc_jit_param_as_rvalue (gcc_jit_param *param);`
  pub fn gcc_jit_param_as_rvalue (param: *mut gcc_jit_param) -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 861-862: `gcc_jit_param *gcc_jit_function_get_param (gcc_jit_function *func,
  /// int index);`
  pub fn gcc_jit_function_get_param (func: *mut gcc_jit_function, index: c_int)
    -> *mut gcc_jit_param;
  /// C: libgccjit.h 464-465: `const char *gcc_jit_context_get_first_error (gcc_jit_context *ctxt);`
  pub fn gcc_jit_context_get_first_error (ctxt: *mut gcc_jit_context) -> *const c_char;
  /// C: libgccjit.h 1590-1593: `void gcc_jit_block_end_with_jump (gcc_jit_block *block,
  /// gcc_jit_location *loc, gcc_jit_block *target);`
  pub fn gcc_jit_block_end_with_jump (block: *mut gcc_jit_block, loc: *mut gcc_jit_location,
                                      target: *mut gcc_jit_block);
  /// C: libgccjit.h 1597-1601: `void gcc_jit_block_end_with_return (gcc_jit_block *block,
  /// gcc_jit_location *loc, gcc_jit_rvalue *rvalue);`
  pub fn gcc_jit_block_end_with_return (block: *mut gcc_jit_block, loc: *mut gcc_jit_location,
                                        rvalue: *mut gcc_jit_rvalue);
}

/// C: libgccjit.h 1176-1198: `enum gcc_jit_unary_op { GCC_JIT_UNARY_OP_MINUS,
/// GCC_JIT_UNARY_OP_BITWISE_NEGATE, GCC_JIT_UNARY_OP_LOGICAL_NEGATE, GCC_JIT_UNARY_OP_ABS };` -
/// the port's alias for a C enum, plus the one enumerator this slice names.  Measured with clang:
/// `sizeof (enum gcc_jit_unary_op) == 4` and `__builtin_types_compatible_p (enum
/// gcc_jit_unary_op, unsigned int)` both hold, so the alias is `c_uint` (the reading is recorded
/// with the type readings in `docs/ports/comp.md`).
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_unary_op = c_uint;
/// C: libgccjit.h 1193: `GCC_JIT_UNARY_OP_LOGICAL_NEGATE,` - the third enumerator, i.e. 2.
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_UNARY_OP_LOGICAL_NEGATE: gcc_jit_unary_op = 2;

/// C: libgccjit.h 1211-1272: `enum gcc_jit_binary_op { GCC_JIT_BINARY_OP_PLUS, … };` - the alias,
/// measured the same way as `gcc_jit_unary_op` above (`sizeof` 4, underlying type `unsigned int`),
/// then the twelve enumerators, in C's order from 0.
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_binary_op = c_uint;
/// C: libgccjit.h 1214: `GCC_JIT_BINARY_OP_PLUS,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_PLUS: gcc_jit_binary_op = 0;
/// C: libgccjit.h 1219: `GCC_JIT_BINARY_OP_MINUS,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_MINUS: gcc_jit_binary_op = 1;
/// C: libgccjit.h 1224: `GCC_JIT_BINARY_OP_MULT,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_MULT: gcc_jit_binary_op = 2;
/// C: libgccjit.h 1232: `GCC_JIT_BINARY_OP_DIVIDE,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_DIVIDE: gcc_jit_binary_op = 3;
/// C: libgccjit.h 1237: `GCC_JIT_BINARY_OP_MODULO,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_MODULO: gcc_jit_binary_op = 4;
/// C: libgccjit.h 1242: `GCC_JIT_BINARY_OP_BITWISE_AND,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_BITWISE_AND: gcc_jit_binary_op = 5;
/// C: libgccjit.h 1247: `GCC_JIT_BINARY_OP_BITWISE_XOR,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_BITWISE_XOR: gcc_jit_binary_op = 6;
/// C: libgccjit.h 1252: `GCC_JIT_BINARY_OP_BITWISE_OR,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_BITWISE_OR: gcc_jit_binary_op = 7;
/// C: libgccjit.h 1257: `GCC_JIT_BINARY_OP_LOGICAL_AND,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_LOGICAL_AND: gcc_jit_binary_op = 8;
/// C: libgccjit.h 1262: `GCC_JIT_BINARY_OP_LOGICAL_OR,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_LOGICAL_OR: gcc_jit_binary_op = 9;
/// C: libgccjit.h 1267: `GCC_JIT_BINARY_OP_LSHIFT,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_LSHIFT: gcc_jit_binary_op = 10;
/// C: libgccjit.h 1272: `GCC_JIT_BINARY_OP_RSHIFT,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BINARY_OP_RSHIFT: gcc_jit_binary_op = 11;

/// C: libgccjit.h 802-836: `enum gcc_jit_function_kind { GCC_JIT_FUNCTION_EXPORTED, … };` - the
/// alias (measured like the others) and the two enumerators this slice names.
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_function_kind = c_uint;
/// C: libgccjit.h 806: `GCC_JIT_FUNCTION_EXPORTED,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_FUNCTION_EXPORTED: gcc_jit_function_kind = 0;
/// C: libgccjit.h 810: `GCC_JIT_FUNCTION_INTERNAL,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_FUNCTION_INTERNAL: gcc_jit_function_kind = 1;
/// C: libgccjit.h 815: `GCC_JIT_FUNCTION_IMPORTED,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_FUNCTION_IMPORTED: gcc_jit_function_kind = 2;

/// C: libgccjit.h 1284-1304: `enum gcc_jit_comparison { GCC_JIT_COMPARISON_EQ, … };` - the alias
/// (measured like the two above) and the one enumerator this slice names.
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_comparison = c_uint;
/// C: libgccjit.h 1288: `GCC_JIT_COMPARISON_EQ,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_COMPARISON_EQ: gcc_jit_comparison = 0;
/// C: libgccjit.h 1297: `GCC_JIT_COMPARISON_LE,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_COMPARISON_LE: gcc_jit_comparison = 3;

/// `fatal` is `sysdep.c` 2399's non-`static` definition; C declares it where it is used, and the
/// units that call it carry the variadic declaration themselves (`eval.rs` 100, `androidterm.rs`
/// 8695 - the same shape, since no mirror names it).
#[cfg (feature = "have_native_comp")]
unsafe extern "C" { pub fn fatal (msgid: *const c_char, ...) -> !; }

/// C 824-850: `static void freloc_check_fill (void)`.  C's `goto overflow` leaves the loop and
/// the following statements at once, so the body is a labelled block and the label's code sits
/// after it, where C's `overflow:` sits.
#[cfg (feature = "have_native_comp")]
unsafe fn freloc_check_fill ()
{
  if freloc.size != 0
    { return; }

  eassert! (! NILP (Vcomp_subr_list));

  'overflow: {
    if countof! (helper_link_table) > F_RELOC_MAX_SIZE
      { break 'overflow; }
    memcpy (freloc.link_table.as_mut_ptr () as *mut c_void,
            helper_link_table.as_ptr () as *const c_void,
            core::mem::size_of_val (&helper_link_table));
    freloc.size = countof! (helper_link_table) as ptrdiff_t;

    let mut subr_l: Lisp_Object = Vcomp_subr_list;
    FOR_EACH_TAIL! (subr_l, {
      if freloc.size == F_RELOC_MAX_SIZE as ptrdiff_t
        { break 'overflow; }
      let subr: *mut Lisp_Subr = XSUBR (XCAR (subr_l));
      freloc.link_table[freloc.size as usize] =
        (*subr).function.a0.map_or (std::ptr::null_mut (), |a0| a0 as *mut c_void);
      freloc.size += 1;
    });
    return;
  }

  fatal (c"Overflowing function relocation table, increase F_RELOC_MAX_SIZE".as_ptr ());
}

/// C 852-856: `static void bcall0 (Lisp_Object f)`.
#[cfg (feature = "have_native_comp")]
unsafe fn bcall0 (f: Lisp_Object)
{
  calln! (f);
}

/// C 858-867: `static gcc_jit_block *retrieve_block (Lisp_Object block_name)`.
#[cfg (feature = "have_native_comp")]
unsafe fn retrieve_block (block_name: Lisp_Object) -> *mut gcc_jit_block
{
  let value: Lisp_Object = Fgethash (block_name, comp.func_blocks_h, Qnil ());

  if NILP (value)
    { xsignal2 (Qnative_ice (), build_string (c"missing basic block".as_ptr ()), block_name); }

  return xmint_pointer (value) as *mut gcc_jit_block;
}

/// C 869-880: `static void declare_block (Lisp_Object block_name)`.
#[cfg (feature = "have_native_comp")]
unsafe fn declare_block (block_name: Lisp_Object)
{
  let name_str: *mut c_char = SSDATA (SYMBOL_NAME (block_name));
  let block: *mut gcc_jit_block = gcc_jit_function_new_block (comp.func, name_str);
  let value: Lisp_Object = make_mint_ptr (block as *mut c_void);

  if ! NILP (Fgethash (block_name, comp.func_blocks_h, Qnil ()))
    { xsignal1 (Qnative_ice (), build_string (c"double basic block declaration".as_ptr ())); }

  Fputhash (block_name, value, comp.func_blocks_h);
}

/// C 882-900: `static gcc_jit_lvalue *emit_mvar_lval (Lisp_Object mvar)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_mvar_lval (mvar: Lisp_Object) -> *mut gcc_jit_lvalue
{
  let mvar_slot: Lisp_Object = CALLNI! (c"comp-mvar-slot".as_ptr (), mvar);

  if EQ (mvar_slot, Qscratch ())
    {
      if comp.scratch.is_null ()
        { comp.scratch = gcc_jit_function_new_local (comp.func,
                                                     std::ptr::null_mut (),
                                                     comp.lisp_obj_type,
                                                     c"scratch".as_ptr ()); }
      return comp.scratch;
    }

  let slot_n: EMACS_INT = XFIXNUM (mvar_slot);
  eassert! (slot_n < comp.frame_size);
  return *comp.frame.offset (slot_n as isize);
}

/// C 902-907: `static void register_emitter (Lisp_Object key, void *func)`.
#[cfg (feature = "have_native_comp")]
unsafe fn register_emitter (key: Lisp_Object, func: *mut c_void)
{
  let value: Lisp_Object = make_mint_ptr (func);
  Fputhash (key, value, comp.emitter_dispatcher);
}

/// C 909-941: `static imm_reloc_t obj_to_reloc (Lisp_Object obj)`.  C declares `reloc` without an
/// initialiser and writes both fields on every path that reaches the return (PORTING.md 0.1-13's
/// zeroed reading for C's uninitialised storage); C's `goto found` jumps *forward* into the middle
/// of the body, so the statements from `found:` on sit in a labelled block.
#[cfg (feature = "have_native_comp")]
unsafe fn obj_to_reloc (obj: Lisp_Object) -> imm_reloc_t
{
  let mut reloc: imm_reloc_t = std::mem::zeroed ();
  let idx: Lisp_Object;

  'found: {
    let idx_first: Lisp_Object = Fgethash (obj, comp.d_default_idx, Qnil ());
    if ! NILP (idx_first)
      {
        reloc.array = comp.data_relocs;
        idx = idx_first;
        break 'found;
      }

    let idx_second: Lisp_Object = Fgethash (obj, comp.d_ephemeral_idx, Qnil ());
    if ! NILP (idx_second)
      {
        reloc.array = comp.data_relocs_ephemeral;
        idx = idx_second;
        break 'found;
      }

    xsignal1 (Qnative_ice (),
              build_string (c"can't find data in relocation containers".as_ptr ()));
    eassume! (false);
  }

  // found:
  eassert! (XFIXNUM (idx) < reloc.array.len);
  if ! FIXNUMP (idx)
    { xsignal1 (Qnative_ice (),
                build_string (c"inconsistent data relocation container".as_ptr ())); }
  reloc.idx = gcc_jit_context_new_rvalue_from_int (comp.ctxt,
                                                   comp.ptrdiff_type,
                                                   XFIXNUM (idx) as c_int);
  return reloc;
}

/* Should come with libgccjit.  */

/// C 1212-1254: `static gcc_jit_rvalue *emit_rvalue_from_long_long (gcc_jit_type *type,
/// long long n)`.  C's `(unsigned long long) n >> 32` is an unsigned shift, so the port casts
/// before shifting.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_rvalue_from_long_long (type_: *mut gcc_jit_type, n: c_longlong)
  -> *mut gcc_jit_rvalue
{
  emit_comment (format_string (c"emit long long: %lld".as_ptr (), n));

  let high: *mut gcc_jit_rvalue =
    gcc_jit_context_new_rvalue_from_long (comp.ctxt,
					  comp.unsigned_long_long_type,
					  ((n as c_ulonglong) >> 32) as c_long);
  let low: *mut gcc_jit_rvalue =
    emit_binary_op (GCC_JIT_BINARY_OP_RSHIFT,
		    comp.unsigned_long_long_type,
		    emit_binary_op (GCC_JIT_BINARY_OP_LSHIFT,
				    comp.unsigned_long_long_type,
				    gcc_jit_context_new_rvalue_from_long (
				      comp.ctxt,
				      comp.unsigned_long_long_type,
				      n as c_long),
				    gcc_jit_context_new_rvalue_from_int (
				      comp.ctxt,
				      comp.unsigned_long_long_type,
				      32)),
		    gcc_jit_context_new_rvalue_from_int (
		      comp.ctxt,
		      comp.unsigned_long_long_type,
		      32));

  return
    emit_coerce (type_,
      emit_binary_op (
	GCC_JIT_BINARY_OP_BITWISE_OR,
	comp.unsigned_long_long_type,
	emit_binary_op (
	  GCC_JIT_BINARY_OP_LSHIFT,
	  comp.unsigned_long_long_type,
	  high,
	  gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					       comp.unsigned_long_long_type,
					       32)),
	low));
}

/// C 1256-1266: `static gcc_jit_rvalue *emit_rvalue_from_emacs_uint (EMACS_UINT val)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_rvalue_from_emacs_uint (val: EMACS_UINT) -> *mut gcc_jit_rvalue
{
  #[cfg (feature = "wide_emacs_int")]
  {
    if val > ULONG_MAX
      { return emit_rvalue_from_long_long (comp.emacs_uint_type, val as c_longlong); }
  }
  return gcc_jit_context_new_rvalue_from_long (comp.ctxt,
					       comp.emacs_uint_type,
					       val as c_long);
}

/// C 1268-1276: `static gcc_jit_rvalue *emit_rvalue_from_emacs_int (EMACS_INT val)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_rvalue_from_emacs_int (val: EMACS_INT) -> *mut gcc_jit_rvalue
{
  if val > LONG_MAX as EMACS_INT || val < LONG_MIN as EMACS_INT
    { return emit_rvalue_from_long_long (comp.emacs_int_type, val as c_longlong); }
  else
    { return gcc_jit_context_new_rvalue_from_long (comp.ctxt,
						 comp.emacs_int_type, val as c_long); }
}

/// C 1278-1288: `static gcc_jit_rvalue *emit_rvalue_from_lisp_word_tag (Lisp_Word_tag val)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_rvalue_from_lisp_word_tag (val: Lisp_Word_tag) -> *mut gcc_jit_rvalue
{
  #[cfg (feature = "wide_emacs_int")]
  {
    if val > ULONG_MAX
      { return emit_rvalue_from_long_long (comp.lisp_word_tag_type, val as c_longlong); }
  }
  return gcc_jit_context_new_rvalue_from_long (comp.ctxt,
					       comp.lisp_word_tag_type,
					       val as c_long);
}

/// C 1290-1305: `static gcc_jit_rvalue *emit_rvalue_from_lisp_word (Lisp_Word val)`.  C's
/// `#if LISP_WORDS_ARE_POINTERS` reads a constant `lisp.h` defines, not a configuration macro, and
/// the constant is what decides which arm *exists*: with it true `Lisp_Word` is `struct Lisp_X *`,
/// so C's `#else` arm (`val > LONG_MAX`) is text the preprocessor never shows the compiler, and a
/// pointer comparison against `LONG_MAX` does not type-check in Rust either.  The arm is therefore
/// written against the integer view of the same word (a `usize` cast), which is the spelling
/// `Lisp_Word` itself has in that configuration; it is dead text, and both arms are carried.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_rvalue_from_lisp_word (val: Lisp_Word) -> *mut gcc_jit_rvalue
{
  if LISP_WORDS_ARE_POINTERS
    {
      return gcc_jit_context_new_rvalue_from_ptr (comp.ctxt,
                                                  comp.lisp_word_type,
                                                  val as *mut c_void);
    }
  else
    {
      /* C's `#else` arm compares `val` as an integer, which is the spelling `Lisp_Word` has in
	 *that* configuration (a word, not a pointer).  This one makes `Lisp_Word` a
	 `struct Lisp_X *`, so the arm reads the same bits through the integer view C's own
	 configuration would have given it - and the arm is never taken, because
	 `LISP_WORDS_ARE_POINTERS` is `true` (`include/lisp.rs` 3345).  */
      let word: usize = val as usize;
      if word > LONG_MAX as usize
	{ return emit_rvalue_from_long_long (comp.lisp_word_type,
					     word as c_longlong); }
      else
	{ return gcc_jit_context_new_rvalue_from_long (comp.ctxt,
						       comp.lisp_word_type,
						       word as c_long); }
    }
}

/// C 1307-1316: `static gcc_jit_rvalue *emit_rvalue_from_lisp_obj (Lisp_Object obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_rvalue_from_lisp_obj (obj: Lisp_Object) -> *mut gcc_jit_rvalue
{
  #[cfg (feature = "lisp_object_is_struct")]
  {
    return emit_coerce (comp.lisp_obj_type,
                        emit_rvalue_from_lisp_word (obj.i));
  }
  #[cfg (not (feature = "lisp_object_is_struct"))]
  {
    return emit_rvalue_from_lisp_word (obj);
  }
}

/*
   Emit the equivalent of:
   (typeof_ptr) ((uintptr) ptr + size_of_ptr_ref * i)
*/

/// C 1318-1346: `static gcc_jit_rvalue *emit_ptr_arithmetic (gcc_jit_rvalue *ptr,
/// gcc_jit_type *ptr_type, int size_of_ptr_ref, gcc_jit_rvalue *i)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_ptr_arithmetic (ptr: *mut gcc_jit_rvalue, ptr_type: *mut gcc_jit_type,
			       size_of_ptr_ref: c_int, i: *mut gcc_jit_rvalue)
  -> *mut gcc_jit_rvalue
{
  emit_comment (c"ptr_arithmetic".as_ptr ());

  let offset: *mut gcc_jit_rvalue =
    emit_binary_op (
      GCC_JIT_BINARY_OP_MULT,
      comp.uintptr_type,
      gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					   comp.uintptr_type,
					   size_of_ptr_ref),
      i);

  return
    emit_coerce (
      ptr_type,
      emit_binary_op (
	GCC_JIT_BINARY_OP_PLUS,
	comp.uintptr_type,
	ptr,
	offset));
}

/// C 1348-1353: `static gcc_jit_rvalue *emit_XLI (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_XLI (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"XLI".as_ptr ());
  return emit_coerce (comp.emacs_int_type, obj);
}

/// C 1355-1361: `static gcc_jit_rvalue *emit_XLP (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_XLP (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"XLP".as_ptr ());

  return emit_coerce (comp.void_ptr_type, obj);
}

/// C 1363-1377: `static gcc_jit_rvalue *emit_XUNTAG (gcc_jit_rvalue *a, gcc_jit_type *type,
/// Lisp_Word_tag lisp_word_tag)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_XUNTAG (a: *mut gcc_jit_rvalue, type_: *mut gcc_jit_type,
		       lisp_word_tag: Lisp_Word_tag) -> *mut gcc_jit_rvalue
{
  /* #define XUNTAG(a, type, ctype) ((ctype *)
     ((char *) XLP (a) - LISP_WORD_TAG (type))) */
  emit_comment (c"XUNTAG".as_ptr ());

  return emit_coerce (
	   gcc_jit_type_get_pointer (type_),
	   emit_binary_op (
	     GCC_JIT_BINARY_OP_MINUS,
	     comp.uintptr_type,
	     emit_XLP (a),
	     emit_rvalue_from_lisp_word_tag (lisp_word_tag)));
}

/// C 1379-1387: `static gcc_jit_rvalue *emit_XCONS (gcc_jit_rvalue *a)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_XCONS (a: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"XCONS".as_ptr ());

  return emit_XUNTAG (a,
		      gcc_jit_struct_as_type (comp.lisp_cons_s),
		      LISP_WORD_TAG (Lisp_Type::Lisp_Cons as Lisp_Word_tag));
}

/// C 1389-1400: `static gcc_jit_rvalue *emit_BASE_EQ (gcc_jit_rvalue *x, gcc_jit_rvalue *y)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_BASE_EQ (x: *mut gcc_jit_rvalue, y: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"BASE_EQ".as_ptr ());

  return gcc_jit_context_new_comparison (
	   comp.ctxt,
	   std::ptr::null_mut (),
	   GCC_JIT_COMPARISON_EQ,
	   emit_XLI (x),
	   emit_XLI (y));
}

/// C 1402-1412: `static gcc_jit_rvalue *emit_AND (gcc_jit_rvalue *x, gcc_jit_rvalue *y)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_AND (x: *mut gcc_jit_rvalue, y: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  return gcc_jit_context_new_binary_op (
    comp.ctxt,
    std::ptr::null_mut (),
    GCC_JIT_BINARY_OP_LOGICAL_AND,
    comp.bool_type,
    x,
    y);
}

/// C 1414-1424: `static gcc_jit_rvalue *emit_OR (gcc_jit_rvalue *x, gcc_jit_rvalue *y)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_OR (x: *mut gcc_jit_rvalue, y: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  return gcc_jit_context_new_binary_op (
    comp.ctxt,
    std::ptr::null_mut (),
    GCC_JIT_BINARY_OP_LOGICAL_OR,
    comp.bool_type,
    x,
    y);
}

/// C 1426-1469: `static gcc_jit_rvalue *emit_TAGGEDP (gcc_jit_rvalue *obj, Lisp_Word_tag tag)`.
/// C's `(USE_LSB_TAG ? 0 : VALBITS)` and `((1 << GCTYPEBITS) - 1)` are constants it evaluates at
/// compile time; the port reads the same two constants (`include/lisp.rs` 77 and lisp.h's widths).
#[cfg (feature = "have_native_comp")]
unsafe fn emit_TAGGEDP (obj: *mut gcc_jit_rvalue, tag: Lisp_Word_tag) -> *mut gcc_jit_rvalue
{
  /* (! (((unsigned) (XLI (a) >> (USE_LSB_TAG ? 0 : VALBITS)) 	- (unsigned) (tag)) 	& ((1 << GCTYPEBITS) - 1))) */
  emit_comment (c"TAGGEDP".as_ptr ());

  let sh_res: *mut gcc_jit_rvalue =
    emit_binary_op (
      GCC_JIT_BINARY_OP_RSHIFT,
      comp.emacs_int_type,
      emit_XLI (obj),
      gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					   comp.emacs_int_type,
					   (if USE_LSB_TAG { 0 } else { VALBITS as c_int })));

  let minus_res: *mut gcc_jit_rvalue =
    emit_binary_op (
      GCC_JIT_BINARY_OP_MINUS,
	   comp.unsigned_type,
	   sh_res,
	   gcc_jit_context_new_rvalue_from_int (
	     comp.ctxt,
	     comp.unsigned_type,
	     tag as c_int));

  let res: *mut gcc_jit_rvalue =
   gcc_jit_context_new_unary_op (
     comp.ctxt,
     std::ptr::null_mut (),
     GCC_JIT_UNARY_OP_LOGICAL_NEGATE,
     comp.int_type,
     emit_binary_op (
       GCC_JIT_BINARY_OP_BITWISE_AND,
       comp.unsigned_type,
       minus_res,
       gcc_jit_context_new_rvalue_from_int (
	 comp.ctxt,
	 comp.unsigned_type,
	 ((1usize << GCTYPEBITS) - 1) as c_int)));

  return res;
}

/// C 1471-1477: `static gcc_jit_rvalue *emit_VECTORLIKEP (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_VECTORLIKEP (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"VECTORLIKEP".as_ptr ());

  return emit_TAGGEDP (obj, Lisp_Type::Lisp_Vectorlike as Lisp_Word_tag);
}

/// C 1479-1485: `static gcc_jit_rvalue *emit_CONSP (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_CONSP (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"CONSP".as_ptr ());

  return emit_TAGGEDP (obj, Lisp_Type::Lisp_Cons as Lisp_Word_tag);
}

/// C 1487-1504: `static gcc_jit_rvalue *emit_SYMBOL_WITH_POS_P (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_SYMBOL_WITH_POS_P (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"SYMBOL_WITH_POS_P".as_ptr ());

  let args: [*mut gcc_jit_rvalue; 2] =
    [ obj,
      gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					   comp.int_type,
					   pvec_type::PVEC_SYMBOL_WITH_POS as c_int) ];

  return gcc_jit_context_new_call (comp.ctxt,
				   std::ptr::null_mut (),
				   comp.pseudovectorp,
				   2,
				   args.as_ptr () as *mut *mut gcc_jit_rvalue);
}

/// C 1698-1723: `static gcc_jit_rvalue *emit_make_fixnum_MSB_TAG (gcc_jit_rvalue *n)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_make_fixnum_MSB_TAG (n: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  /*
    n &= INTMASK;
    n += (int0 << VALBITS);
    return XIL (n);
  */

  let intmask: *mut gcc_jit_rvalue = emit_rvalue_from_emacs_uint (INTMASK as EMACS_UINT);

  let mut n: *mut gcc_jit_rvalue =
    emit_binary_op (GCC_JIT_BINARY_OP_BITWISE_AND,
		    comp.emacs_uint_type,
		    intmask, n);

  n =
    emit_binary_op (GCC_JIT_BINARY_OP_PLUS,
		    comp.emacs_uint_type,
		    emit_binary_op (GCC_JIT_BINARY_OP_LSHIFT,
				    comp.emacs_uint_type,
				    comp.lisp_int0,
                                    emit_rvalue_from_emacs_uint (VALBITS as EMACS_UINT)),
		    n);

  return emit_coerce (comp.lisp_obj_type, n);
}

/// C 1726-1733: `static gcc_jit_rvalue *emit_make_fixnum (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_make_fixnum (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"make_fixnum".as_ptr ());
  return if USE_LSB_TAG
    { emit_make_fixnum_LSB_TAG (obj) }
    else
    { emit_make_fixnum_MSB_TAG (obj) };
}

/// C 1735-1746: `static gcc_jit_lvalue *emit_lisp_obj_reloc_lval (Lisp_Object obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_lisp_obj_reloc_lval (obj: Lisp_Object) -> *mut gcc_jit_lvalue
{
  emit_comment (format_string (c"l-value for lisp obj: %s".as_ptr (),
			       SSDATA (Fprin1_to_string (obj, Qnil (), Qnil ()))));

  let reloc: imm_reloc_t = obj_to_reloc (obj);
  return gcc_jit_context_new_array_access (comp.ctxt,
					   std::ptr::null_mut (),
					   reloc.array.r_val,
					   reloc.idx);
}

/// C 1748-1762: `static gcc_jit_rvalue *emit_lisp_obj_rval (Lisp_Object obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_lisp_obj_rval (obj: Lisp_Object) -> *mut gcc_jit_rvalue
{
  emit_comment (format_string (c"const lisp obj: %s".as_ptr (),
			       SSDATA (Fprin1_to_string (obj, Qnil (), Qnil ()))));

  if NILP (obj)
    {
      let n: *mut gcc_jit_rvalue =
	emit_rvalue_from_lisp_word (iQnil as Lisp_Word);
      return emit_coerce (comp.lisp_obj_type, n);
    }

  return gcc_jit_lvalue_as_rvalue (emit_lisp_obj_reloc_lval (obj));
}

/// C 1764-1769: `static gcc_jit_rvalue *emit_NILP (gcc_jit_rvalue *x)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_NILP (x: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"NILP".as_ptr ());
  return emit_BASE_EQ (x, emit_lisp_obj_rval (Qnil ()));
}

/// C 1771-1791: `static gcc_jit_rvalue *emit_XCAR (gcc_jit_rvalue *c)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_XCAR (c: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"XCAR".as_ptr ());

  /* XCONS (c)->u.s.car */
  return
    gcc_jit_rvalue_access_field (
      /* XCONS (c)->u.s */
      gcc_jit_rvalue_access_field (
	/* XCONS (c)->u */
	gcc_jit_lvalue_as_rvalue (
	  gcc_jit_rvalue_dereference_field (
	    emit_XCONS (c),
	    std::ptr::null_mut (),
	    comp.lisp_cons_u)),
	std::ptr::null_mut (),
	comp.lisp_cons_u_s),
      std::ptr::null_mut (),
      comp.lisp_cons_u_s_car);
}

/// C 1793-1812: `static gcc_jit_lvalue *emit_lval_XCAR (gcc_jit_rvalue *c)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_lval_XCAR (c: *mut gcc_jit_rvalue) -> *mut gcc_jit_lvalue
{
  emit_comment (c"lval_XCAR".as_ptr ());

  /* XCONS (c)->u.s.car */
  return
    gcc_jit_lvalue_access_field (
      /* XCONS (c)->u.s */
      gcc_jit_lvalue_access_field (
	/* XCONS (c)->u */
	gcc_jit_rvalue_dereference_field (
	  emit_XCONS (c),
	  std::ptr::null_mut (),
	  comp.lisp_cons_u),
	std::ptr::null_mut (),
	comp.lisp_cons_u_s),
      std::ptr::null_mut (),
      comp.lisp_cons_u_s_car);
}

/// C 1814-1837: `static gcc_jit_rvalue *emit_XCDR (gcc_jit_rvalue *c)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_XCDR (c: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"XCDR".as_ptr ());
  /* XCONS (c)->u.s.u.cdr */
  return
    gcc_jit_rvalue_access_field (
      /* XCONS (c)->u.s.u */
      gcc_jit_rvalue_access_field (
	/* XCONS (c)->u.s */
	gcc_jit_rvalue_access_field (
	  /* XCONS (c)->u */
	  gcc_jit_lvalue_as_rvalue (
	    gcc_jit_rvalue_dereference_field (
	      emit_XCONS (c),
	      std::ptr::null_mut (),
	      comp.lisp_cons_u)),
	  std::ptr::null_mut (),
	  comp.lisp_cons_u_s),
	std::ptr::null_mut (),
	comp.lisp_cons_u_s_u),
      std::ptr::null_mut (),
      comp.lisp_cons_u_s_u_cdr);
}

/// C 1839-1862: `static gcc_jit_lvalue *emit_lval_XCDR (gcc_jit_rvalue *c)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_lval_XCDR (c: *mut gcc_jit_rvalue) -> *mut gcc_jit_lvalue
{
  emit_comment (c"lval_XCDR".as_ptr ());

  /* XCONS (c)->u.s.u.cdr */
  return
    gcc_jit_lvalue_access_field (
      /* XCONS (c)->u.s.u */
      gcc_jit_lvalue_access_field (
	/* XCONS (c)->u.s */
	gcc_jit_lvalue_access_field (
	  /* XCONS (c)->u */
	  gcc_jit_rvalue_dereference_field (
	    emit_XCONS (c),
	    std::ptr::null_mut (),
	    comp.lisp_cons_u),
	  std::ptr::null_mut (),
	  comp.lisp_cons_u_s),
	std::ptr::null_mut (),
	comp.lisp_cons_u_s_u),
      std::ptr::null_mut (),
      comp.lisp_cons_u_s_u_cdr);
}

/// C 1864-1882: `static void emit_CHECK_CONS (gcc_jit_rvalue *x)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_CHECK_CONS (x: *mut gcc_jit_rvalue)
{
  emit_comment (c"CHECK_CONS".as_ptr ());

  let args: [*mut gcc_jit_rvalue; 3] =
    [ emit_CONSP (x),
      emit_lisp_obj_rval (Qconsp ()),
      x ];

  gcc_jit_block_add_eval (
    comp.block,
    std::ptr::null_mut (),
    gcc_jit_context_new_call (comp.ctxt,
			      std::ptr::null_mut (),
			      comp.check_type,
			      3,
			      args.as_ptr () as *mut *mut gcc_jit_rvalue));
}

/// C 1884-1905: `static void emit_CHECK_SYMBOL_WITH_POS (gcc_jit_rvalue *x)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_CHECK_SYMBOL_WITH_POS (x: *mut gcc_jit_rvalue)
{
  emit_comment (c"CHECK_SYMBOL_WITH_POS".as_ptr ());

  let args: [*mut gcc_jit_rvalue; 3] =
    [ gcc_jit_context_new_cast (comp.ctxt,
				std::ptr::null_mut (),
				emit_SYMBOL_WITH_POS_P (x),
				comp.int_type),
      emit_lisp_obj_rval (Qsymbol_with_pos_p ()),
      x ];

  gcc_jit_block_add_eval (
    comp.block,
    std::ptr::null_mut (),
    gcc_jit_context_new_call (comp.ctxt,
			      std::ptr::null_mut (),
			      comp.check_type,
			      3,
			      args.as_ptr () as *mut *mut gcc_jit_rvalue));
}

/// C 1907-1913: `static gcc_jit_rvalue *emit_car_addr (gcc_jit_rvalue *c)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_car_addr (c: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"car_addr".as_ptr ());

  return gcc_jit_lvalue_get_address (emit_lval_XCAR (c), std::ptr::null_mut ());
}

/// C 1915-1921: `static gcc_jit_rvalue *emit_cdr_addr (gcc_jit_rvalue *c)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_cdr_addr (c: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"cdr_addr".as_ptr ());

  return gcc_jit_lvalue_get_address (emit_lval_XCDR (c), std::ptr::null_mut ());
}

/// C 1923-1935: `static void emit_XSETCAR (gcc_jit_rvalue *c, gcc_jit_rvalue *n)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_XSETCAR (c: *mut gcc_jit_rvalue, n: *mut gcc_jit_rvalue)
{
  emit_comment (c"XSETCAR".as_ptr ());

  gcc_jit_block_add_assignment (
    comp.block,
    std::ptr::null_mut (),
    gcc_jit_rvalue_dereference (
      emit_car_addr (c),
      std::ptr::null_mut ()),
    n);
}

/// C 1937-1949: `static void emit_XSETCDR (gcc_jit_rvalue *c, gcc_jit_rvalue *n)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_XSETCDR (c: *mut gcc_jit_rvalue, n: *mut gcc_jit_rvalue)
{
  emit_comment (c"XSETCDR".as_ptr ());

  gcc_jit_block_add_assignment (
    comp.block,
    std::ptr::null_mut (),
    gcc_jit_rvalue_dereference (
      emit_cdr_addr (c),
      std::ptr::null_mut ()),
    n);
}

/* ==============================================================================================
   Code emitted by LIMPLE statemes.  (C 1951-1953's banner, kept where C has it.)
   ============================================================================================== */

/* Emit an r-value from an mvar meta variable.
   In case this is a constant that was propagated return it otherwise load it
   from frame.  */

/// C 2079's function-pointer type, spelled once: `gcc_jit_rvalue *(*emitter_ptr) (Lisp_Object)`.
#[cfg (feature = "have_native_comp")]
pub type Llisp_emitter = unsafe extern "C" fn (Lisp_Object) -> *mut gcc_jit_rvalue;

/// C 1959-1996: `static gcc_jit_rvalue *emit_mvar_rval (Lisp_Object mvar)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_mvar_rval (mvar: Lisp_Object) -> *mut gcc_jit_rvalue
{
  let const_vld: Lisp_Object = CALLNI! (c"comp-cstr-imm-vld-p".as_ptr (), mvar);

  if ! NILP (const_vld)
    {
      let value: Lisp_Object = CALLNI! (c"comp-cstr-imm".as_ptr (), mvar);
      if NILP (Fgethash (value,
			  CALLNI! (c"comp-ctxt-non-materializable-objs-h".as_ptr (), Vcomp_ctxt),
			  Qnil ()))
	{
	  if comp.debug > 1
	    {
	      let func: Lisp_Object =
		Fgethash (value,
			  CALLNI! (c"comp-ctxt-byte-func-to-func-h".as_ptr (), Vcomp_ctxt),
			  Qnil ());

	      emit_comment (
	        SSDATA (
		  Fprin1_to_string (
		    if NILP (func) { value }
		    else { CALLNI! (c"comp-func-c-name".as_ptr (), func) },
		    Qnil (), Qnil ())));
	    }
	  if FIXNUMP (value)
	    {
	      /* We can still emit directly objects that are self-contained in a
		 word (read fixnums).  */
	      return emit_rvalue_from_lisp_obj (value);
	    }
	  /* Other const objects are fetched from the reloc array.  */
	  return emit_lisp_obj_rval (value);
	}
    }

  return gcc_jit_lvalue_as_rvalue (emit_mvar_lval (mvar));
}

/// C 1998-2007: `static void emit_frame_assignment (Lisp_Object dst_mvar,
/// gcc_jit_rvalue *val)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_frame_assignment (dst_mvar: Lisp_Object, val: *mut gcc_jit_rvalue)
{

  gcc_jit_block_add_assignment (
    comp.block,
    std::ptr::null_mut (),
    emit_mvar_lval (dst_mvar),
    val);
}

/// C 2009-2033: `static gcc_jit_rvalue *emit_set_internal (Lisp_Object args)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_set_internal (args_in: Lisp_Object) -> *mut gcc_jit_rvalue
{
  /*
    Ex: (set_internal #s(comp-mvar nil nil t comp-test-up-val nil nil)
                      #s(comp-mvar 1 4 t nil symbol nil)).
  */
  /* TODO: Inline the most common case.  */
  if list_length (args_in) != 3
    { xsignal2 (Qnative_ice (),
		build_string (c"unexpected arg length for insns".as_ptr ()),
		args_in); }

  let mut args: Lisp_Object = XCDR (args_in);
  let mut i: c_int = 0;
  let mut gcc_args: [*mut gcc_jit_rvalue; 4] = [std::ptr::null_mut (); 4];
  FOR_EACH_TAIL! (args, {
    gcc_args[i as usize] = emit_mvar_rval (XCAR (args));
    i += 1;
  });
  gcc_args[2] = emit_lisp_obj_rval (Qnil ());
  gcc_args[3] = gcc_jit_context_new_rvalue_from_int (comp.ctxt,
						     comp.int_type,
						     SET_INTERNAL_SET);
  return emit_call (intern_c_string (c"set_internal".as_ptr ()), comp.void_type, 4,
		    gcc_args.as_mut_ptr (), false);
}

/// C 2086-2143: `static gcc_jit_rvalue *emit_limple_call_ref (Lisp_Object insn, bool direct)`.
/// C's `static int i = 0;` is a function-scope static whose `i++` is a post-increment.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_limple_call_ref (insn: Lisp_Object, direct: bool) -> *mut gcc_jit_rvalue
{
  /* Ex: (funcall #s(comp-mvar 1 5 t eql symbol t)
                  #s(comp-mvar 2 6 nil nil nil t)
		  #s(comp-mvar 3 7 t 0 fixnum t)).  */
  static mut i: c_int = 0;
  let callee: Lisp_Object = FIRST! (insn);
  let nargs: EMACS_INT = XFIXNUM (Flength (CDR (insn)));

  if nargs == 0
    { return emit_call_ref (callee, 0, *comp.frame.offset (0), direct); }

  if comp.func_has_non_local || comp.func_speed == 0
    {
      /* FIXME: See bug#42360.  */
      let first_arg: Lisp_Object = SECOND! (insn);
      let first_slot: EMACS_INT = XFIXNUM (CALLNI! (c"comp-mvar-slot".as_ptr (), first_arg));
      return emit_call_ref (callee, nargs, *comp.frame.offset (first_slot as isize), direct);
    }

  let counter: c_int = i;
  i += 1;
  let tmp_arr: *mut gcc_jit_lvalue =
    gcc_jit_function_new_local (
      comp.func,
      std::ptr::null_mut (),
      gcc_jit_context_new_array_type (comp.ctxt,
				      std::ptr::null_mut (),
				      comp.lisp_obj_type,
				      nargs as c_int),
      format_string (c"call_arr_%d".as_ptr (), counter));

  let mut j: ptrdiff_t = 0;
  let mut arg: Lisp_Object = CDR (insn);
  FOR_EACH_TAIL! (arg, {
      gcc_jit_block_add_assignment (
        comp.block,
	std::ptr::null_mut (),
	gcc_jit_context_new_array_access (
	  comp.ctxt,
	  std::ptr::null_mut (),
	  gcc_jit_lvalue_as_rvalue (tmp_arr),
	  gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					       comp.int_type,
					       j as c_int)),
	emit_mvar_rval (XCAR (arg)));
      j += 1;
    });

  return emit_call_ref (
	   callee,
	   nargs,
	   gcc_jit_context_new_array_access (comp.ctxt,
					     std::ptr::null_mut (),
					     gcc_jit_lvalue_as_rvalue (tmp_arr),
					     comp.zero),
	   direct);
}

/// C 2145-2192: `static gcc_jit_rvalue *emit_setjmp (gcc_jit_rvalue *buf)`.  C's `#ifndef
/// WINDOWSNT` arm is the live one here; the `windowsnt` arm is carried as its feature.  C's
/// `STR (SETJMP_NAME)` is a stringified macro (`SETJMP_NAME` binds `SETJMP`), and the file note
/// above records that the port spells the name at the call site - `_setjmp` in the arm this
/// target takes (`HAVE__SETJMP` is 1 and `WINDOWSNT` is undef).
#[cfg (feature = "have_native_comp")]
unsafe fn emit_setjmp (buf: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  #[cfg (not (feature = "windowsnt"))]
  {
    let args: [*mut gcc_jit_rvalue; 1] = [buf];
    let params: [*mut gcc_jit_param; 1] =
    [
      gcc_jit_context_new_param (comp.ctxt, std::ptr::null_mut (), comp.void_ptr_type,
				 c"buf".as_ptr ()),
    ];
    /* Don't call setjmp through a function pointer (Bug#46824) */
    let f: *mut gcc_jit_function =
      gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				    GCC_JIT_FUNCTION_IMPORTED,
				    comp.int_type, c"_setjmp".as_ptr (),
				    countof! (params) as c_int,
				    params.as_ptr () as *mut *mut gcc_jit_param,
				    0);

    return gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (), f, 1,
				     args.as_ptr () as *mut *mut gcc_jit_rvalue);
  }
  #[cfg (feature = "windowsnt")]
  {
    /* _setjmp (buf, __builtin_frame_address (0)) */
    let params: [*mut gcc_jit_param; 2] =
    [
      gcc_jit_context_new_param (comp.ctxt, std::ptr::null_mut (), comp.void_ptr_type,
				 c"buf".as_ptr ()),
      gcc_jit_context_new_param (comp.ctxt, std::ptr::null_mut (), comp.void_ptr_type,
				 c"frame".as_ptr ()),
    ];
    let mut args: [*mut gcc_jit_rvalue; 2] = [std::ptr::null_mut (); 2];

    args[0] =
      gcc_jit_context_new_rvalue_from_int (comp.ctxt, comp.unsigned_type, 0);

    args[1] =
      gcc_jit_context_new_call (
	comp.ctxt,
	std::ptr::null_mut (),
	gcc_jit_context_get_builtin_function (comp.ctxt,
					      c"__builtin_frame_address".as_ptr ()),
	1, args.as_ptr () as *mut *mut gcc_jit_rvalue);
    args[0] = buf;
    let f: *mut gcc_jit_function =
      gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				    GCC_JIT_FUNCTION_IMPORTED,
				    comp.int_type, c"_setjmp".as_ptr (),
				    countof! (params) as c_int,
				    params.as_ptr () as *mut *mut gcc_jit_param,
				    0);

    return gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (), f, 2,
				     args.as_ptr () as *mut *mut gcc_jit_rvalue);
  }
}

/// C 2224-2521: `static void emit_limple_insn (Lisp_Object insn)`.  C's `Lisp_Object arg[6]` is a
/// six-slot stack array the `FOR_EACH_TAIL` below fills, and `res` is declared before the dispatch
/// (only the `set` arm writes and reads it, as in C).
#[cfg (feature = "have_native_comp")]
unsafe fn emit_limple_insn (insn: Lisp_Object)
{
  let op: Lisp_Object = XCAR (insn);
  let args: Lisp_Object = XCDR (insn);
  let mut res: *mut gcc_jit_rvalue;
  let mut arg: [Lisp_Object; 6] = [Qnil (); 6];

  let mut p: Lisp_Object = XCDR (insn);
  let mut i: ptrdiff_t = 0;
  FOR_EACH_TAIL! (p, {
      if i == countof! (arg) as ptrdiff_t
	{ break; }
      arg[i as usize] = XCAR (p);
      i += 1;
    });

  if EQ (op, Qjump ())
    {
      /* Unconditional branch.  */
      let target: *mut gcc_jit_block = retrieve_block (arg[0]);
      gcc_jit_block_end_with_jump (comp.block, std::ptr::null_mut (), target);
    }
  else if EQ (op, Qcond_jump ())
    {
      /* Conditional branch.  */
      let a: *mut gcc_jit_rvalue = emit_mvar_rval (arg[0]);
      let b: *mut gcc_jit_rvalue = emit_mvar_rval (arg[1]);
      let target1: *mut gcc_jit_block = retrieve_block (arg[2]);
      let target2: *mut gcc_jit_block = retrieve_block (arg[3]);

      if (! NILP (CALLNI! (c"comp-cstr-imm-vld-p".as_ptr (), arg[0]))
	   && NILP (CALLNI! (c"comp-cstr-imm".as_ptr (), arg[0])))
	  || (! NILP (CALLNI! (c"comp-cstr-imm-vld-p".as_ptr (), arg[1]))
	      && NILP (CALLNI! (c"comp-cstr-imm".as_ptr (), arg[1])))
	{ emit_cond_jump (emit_BASE_EQ (a, b), target1, target2); }
      else
	{ emit_cond_jump (emit_EQ (a, b), target1, target2); }
    }
  else if EQ (op, Qcond_jump_narg_leq ())
    {
      /*
	 Limple: (cond-jump-narg-less 2 entry_2 entry_fallback_2)
	 C: if (nargs < 2) goto entry2_fallback; else goto entry_2;
      */
      let nargs: *mut gcc_jit_lvalue =
	gcc_jit_param_as_lvalue (gcc_jit_function_get_param (comp.func, 0));
      eassert! (XFIXNUM (arg[0]) < INT_MAX as EMACS_INT);
      let n: *mut gcc_jit_rvalue =
	gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					     comp.ptrdiff_type,
					     XFIXNUM (arg[0]) as c_int);
      let target1: *mut gcc_jit_block = retrieve_block (arg[1]);
      let target2: *mut gcc_jit_block = retrieve_block (arg[2]);
      let test: *mut gcc_jit_rvalue = gcc_jit_context_new_comparison (
			       comp.ctxt,
			       std::ptr::null_mut (),
			       GCC_JIT_COMPARISON_LE,
			       gcc_jit_lvalue_as_rvalue (nargs),
			       n);
      emit_cond_jump (test, target1, target2);
    }
  else if EQ (op, Qphi ()) || EQ (op, Qassume ())
    {
      /* Nothing to do for phis or assumes in the backend.  */
    }
  else if EQ (op, Qpush_handler ())
    {
      /* (push-handler condition-case #s(comp-mvar 0 3 t (arith-error) cons nil) 1 bb_2 bb_1) 
      int h_num UNINIT; */
      let handler_spec: Lisp_Object = arg[0];
      let handler: *mut gcc_jit_rvalue = emit_mvar_rval (arg[1]);
      let mut h_num: handlertype;
      if EQ (handler_spec, Qcatcher ())
	{ h_num = CATCHER; }
      else if EQ (handler_spec, Qcondition_case ())
	{ h_num = CONDITION_CASE; }
      else
	{ xsignal2 (Qnative_ice (), build_string (c"incoherent insn".as_ptr ()), insn); }
      let handler_type: *mut gcc_jit_rvalue =
	gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					     comp.int_type,
					     h_num);
      let handler_bb: *mut gcc_jit_block = retrieve_block (arg[2]);
      let guarded_bb: *mut gcc_jit_block = retrieve_block (arg[3]);
      emit_limple_push_handler (handler, handler_type, handler_bb, guarded_bb,
				arg[0]);
    }
  else if EQ (op, Qpop_handler ())
    {
      /*
	C: current_thread->m_handlerlist =
	     current_thread->m_handlerlist->next;
      */
      let m_handlerlist: *mut gcc_jit_lvalue =
	gcc_jit_rvalue_dereference_field (
	  gcc_jit_lvalue_as_rvalue (
	    gcc_jit_rvalue_dereference (comp.current_thread_ref, std::ptr::null_mut ())),
	  std::ptr::null_mut (),
	  comp.m_handlerlist);

      gcc_jit_block_add_assignment (
	comp.block,
	std::ptr::null_mut (),
	m_handlerlist,
	gcc_jit_lvalue_as_rvalue (
	  gcc_jit_rvalue_dereference_field (
	    gcc_jit_lvalue_as_rvalue (m_handlerlist),
	    std::ptr::null_mut (),
	    comp.handler_next_field)));

    }
  else if EQ (op, Qfetch_handler ())
    {
      let m_handlerlist: *mut gcc_jit_lvalue =
	gcc_jit_rvalue_dereference_field (
	  gcc_jit_lvalue_as_rvalue (
	    gcc_jit_rvalue_dereference (comp.current_thread_ref, std::ptr::null_mut ())),
	  std::ptr::null_mut (),
	  comp.m_handlerlist);
      gcc_jit_block_add_assignment (comp.block,
				    std::ptr::null_mut (),
				    comp.loc_handler,
				    gcc_jit_lvalue_as_rvalue (m_handlerlist));

      gcc_jit_block_add_assignment (
	comp.block,
	std::ptr::null_mut (),
	m_handlerlist,
	gcc_jit_lvalue_as_rvalue (
	  gcc_jit_rvalue_dereference_field (
	    gcc_jit_lvalue_as_rvalue (comp.loc_handler),
	    std::ptr::null_mut (),
	    comp.handler_next_field)));
      emit_frame_assignment (
	arg[0],
	gcc_jit_lvalue_as_rvalue (
	  gcc_jit_rvalue_dereference_field (
	    gcc_jit_lvalue_as_rvalue (comp.loc_handler),
	    std::ptr::null_mut (),
	    comp.handler_val_field)));
    }
  else if EQ (op, Qcall ())
    {
      gcc_jit_block_add_eval (comp.block, std::ptr::null_mut (),
			      emit_limple_call (args));
    }
  else if EQ (op, Qcallref ())
    {
      gcc_jit_block_add_eval (comp.block, std::ptr::null_mut (),
			      emit_limple_call_ref (args, false));
    }
  else if EQ (op, Qdirect_call ())
    {
      gcc_jit_block_add_eval (
        comp.block, std::ptr::null_mut (),
	emit_simple_limple_call (XCDR (insn), comp.lisp_obj_type, true));
    }
  else if EQ (op, Qdirect_callref ())
    {
      gcc_jit_block_add_eval (comp.block, std::ptr::null_mut (),
			      emit_limple_call_ref (XCDR (insn), true));
    }
  else if EQ (op, Qset ())
    {
      let arg1: Lisp_Object = arg[1];

      if EQ (Fcl_type_of (arg1), Qcomp_mvar ())
	{ res = emit_mvar_rval (arg1); }
      else if EQ (FIRST! (arg1), Qcall ())
	{ res = emit_limple_call (XCDR (arg1)); }
      else if EQ (FIRST! (arg1), Qcallref ())
	{ res = emit_limple_call_ref (XCDR (arg1), false); }
      else if EQ (FIRST! (arg1), Qdirect_call ())
	{ res = emit_simple_limple_call (XCDR (arg1), comp.lisp_obj_type, true); }
      else if EQ (FIRST! (arg1), Qdirect_callref ())
	{ res = emit_limple_call_ref (XCDR (arg1), true); }
      else
	{ xsignal2 (Qnative_ice (),
		    build_string (c"LIMPLE inconsistent arg1 for insn".as_ptr ()),
		    insn); }

      if res.is_null ()
	{ xsignal1 (Qnative_ice (),
		    build_string (gcc_jit_context_get_first_error (comp.ctxt))); }

      emit_frame_assignment (arg[0], res);
    }
  else if EQ (op, Qset_par_to_local ())
    {
      /* Ex: (set-par-to-local #s(comp-mvar 0 3 nil nil nil nil) 0).  */
      let param_n: EMACS_INT = XFIXNUM (arg[1]);
      eassert! (param_n < INT_MAX as EMACS_INT);
      let param: *mut gcc_jit_rvalue =
	gcc_jit_param_as_rvalue (gcc_jit_function_get_param (comp.func,
							     param_n as c_int));
      emit_frame_assignment (arg[0], param);
    }
  else if EQ (op, Qset_args_to_local ())
    {
      /*
	Ex: (set-args-to-local #s(comp-mvar 1 6 nil nil nil nil))
	C: local[1] = *args;
      */
      let gcc_args: *mut gcc_jit_rvalue =
	gcc_jit_lvalue_as_rvalue (
	  gcc_jit_param_as_lvalue (gcc_jit_function_get_param (comp.func, 1)));

      let res: *mut gcc_jit_rvalue =
	gcc_jit_lvalue_as_rvalue (gcc_jit_rvalue_dereference (gcc_args,
							      std::ptr::null_mut ()));

      emit_frame_assignment (arg[0], res);
    }
  else if EQ (op, Qset_rest_args_to_local ())
    {
      /*
        Ex: (set-rest-args-to-local #s(comp-mvar 2 9 nil nil nil nil))
        C: local[2] = list (nargs - 2, args);
      */

      let slot_n: EMACS_INT = XFIXNUM (CALLNI! (c"comp-mvar-slot".as_ptr (), arg[0]));
      eassert! (slot_n < INT_MAX as EMACS_INT);
      let n: *mut gcc_jit_rvalue =
	gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					     comp.ptrdiff_type,
					     slot_n as c_int);
      let nargs: *mut gcc_jit_lvalue =
	gcc_jit_param_as_lvalue (gcc_jit_function_get_param (comp.func, 0));
      let args_: *mut gcc_jit_lvalue =
	gcc_jit_param_as_lvalue (gcc_jit_function_get_param (comp.func, 1));

      let list_args: [*mut gcc_jit_rvalue; 2] =
	[ emit_binary_op (GCC_JIT_BINARY_OP_MINUS,
			  comp.ptrdiff_type,
			  gcc_jit_lvalue_as_rvalue (nargs),
			  n),
	  gcc_jit_lvalue_as_rvalue (args_) ];

      res = emit_call (Qlist (), comp.lisp_obj_type, 2,
		       list_args.as_ptr () as *mut *mut gcc_jit_rvalue, false);

      emit_frame_assignment (arg[0], res);
    }
  else if EQ (op, Qinc_args ())
    {
      /*
	Ex: (inc-args)
	C: ++args;
      */
      let args_: *mut gcc_jit_lvalue =
	gcc_jit_param_as_lvalue (gcc_jit_function_get_param (comp.func, 1));

      gcc_jit_block_add_assignment (comp.block,
				    std::ptr::null_mut (),
				    args_,
				    emit_ptr_arithmetic (
				      gcc_jit_lvalue_as_rvalue (args_),
				      comp.lisp_obj_ptr_type,
				      core::mem::size_of::<Lisp_Object> () as c_int,
				      comp.one));
    }
  else if EQ (op, Qsetimm ())
    {
      /* Ex: (setimm #s(comp-mvar 9 1 t 3 nil) a).  */
      emit_comment (SSDATA (Fprin1_to_string (arg[1], Qnil (), Qnil ())));
      let reloc: imm_reloc_t = obj_to_reloc (arg[1]);
      emit_frame_assignment (
	arg[0],
	gcc_jit_lvalue_as_rvalue (
	  gcc_jit_context_new_array_access (comp.ctxt,
					    std::ptr::null_mut (),
					    reloc.array.r_val,
					    reloc.idx)));
    }
  else if EQ (op, Qcomment ())
    {
      /* Ex: (comment "Function: foo").  */
      emit_comment (SSDATA (arg[0]));
    }
  else if EQ (op, Qreturn ())
    {
      gcc_jit_block_end_with_return (comp.block,
				     std::ptr::null_mut (),
				     emit_mvar_rval (arg[0]));
    }
  else if EQ (op, Qunreachable ())
    {
      /* Libgccjit has no __builtin_unreachable.  */
      gcc_jit_block_end_with_return (comp.block,
				     std::ptr::null_mut (),
				     emit_lisp_obj_rval (Qnil ()));
    }
  else
    {
      xsignal2 (Qnative_ice (),
		build_string (c"LIMPLE op inconsistent".as_ptr ()),
		op);
    }
}

/* Inliners.  (C 2524-2526's banner, kept where C has it.)  */
/// C 2528-2542: `static gcc_jit_rvalue *emit_call_with_type_hint (gcc_jit_function *func,
/// Lisp_Object insn, Lisp_Object type)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_call_with_type_hint (func: *mut gcc_jit_function, insn: Lisp_Object,
				    type_: Lisp_Object) -> *mut gcc_jit_rvalue
{
  let hint_match: bool =
    comp.func_safety == 0
    && ! NILP (CALLNI! (c"comp-mvar-type-hint-match-p".as_ptr (), SECOND! (insn), type_));
  let args: [*mut gcc_jit_rvalue; 2] =
    [ emit_mvar_rval (SECOND! (insn)),
      gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					   comp.bool_type,
					   hint_match as c_int) ];

  return gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (), func, 2,
				   args.as_ptr () as *mut *mut gcc_jit_rvalue);
}

/* Same as before but with two args. The type hint is on the 2th.  */
/// C 2545-2560: `static gcc_jit_rvalue *emit_call2_with_type_hint (gcc_jit_function *func,
/// Lisp_Object insn, Lisp_Object type)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_call2_with_type_hint (func: *mut gcc_jit_function, insn: Lisp_Object,
				     type_: Lisp_Object) -> *mut gcc_jit_rvalue
{
  let hint_match: bool =
    comp.func_safety == 0
    && ! NILP (CALLNI! (c"comp-mvar-type-hint-match-p".as_ptr (), SECOND! (insn), type_));
  let args: [*mut gcc_jit_rvalue; 3] =
    [ emit_mvar_rval (SECOND! (insn)),
      emit_mvar_rval (THIRD! (insn)),
      gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					   comp.bool_type,
					   hint_match as c_int) ];

  return gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (), func, 3,
				   args.as_ptr () as *mut *mut gcc_jit_rvalue);
}

/// C 2563-2567: `static gcc_jit_rvalue *emit_add1 (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_add1 (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  return emit_call_with_type_hint (comp.add1, insn, Qfixnum ());
}

/// C 2569-2573: `static gcc_jit_rvalue *emit_sub1 (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_sub1 (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  return emit_call_with_type_hint (comp.sub1, insn, Qfixnum ());
}

/// C 2575-2579: `static gcc_jit_rvalue *emit_negate (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_negate (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  return emit_call_with_type_hint (comp.negate, insn, Qfixnum ());
}

/// C 2581-2591: `static gcc_jit_rvalue *emit_consp (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_consp (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  let x: *mut gcc_jit_rvalue = emit_mvar_rval (SECOND! (insn));
  let mut res: *mut gcc_jit_rvalue = emit_coerce (comp.bool_type,
						  emit_CONSP (x));
  return gcc_jit_context_new_call (comp.ctxt,
				   std::ptr::null_mut (),
				   comp.bool_to_lisp_obj,
				   1, &raw mut res);
}

/// C 2593-2605: `static gcc_jit_rvalue *emit_eq (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_eq (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  let x: *mut gcc_jit_rvalue = emit_mvar_rval (SECOND! (insn));
  let y: *mut gcc_jit_rvalue = emit_mvar_rval (THIRD! (insn));
  let mut res: *mut gcc_jit_rvalue = emit_EQ (x, y);
  res = emit_coerce (comp.bool_type, res);
  return gcc_jit_context_new_call (comp.ctxt,
                                   std::ptr::null_mut (),
                                   comp.bool_to_lisp_obj,
                                   1,
                                   &raw mut res);
}

/// C 2608-2612: `static gcc_jit_rvalue *emit_car (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_car (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  return emit_call_with_type_hint (comp.car, insn, Qcons ());
}

/// C 2614-2618: `static gcc_jit_rvalue *emit_cdr (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_cdr (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  return emit_call_with_type_hint (comp.cdr, insn, Qcons ());
}

/// C 2620-2624: `static gcc_jit_rvalue *emit_setcar (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_setcar (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  return emit_call2_with_type_hint (comp.setcar, insn, Qcons ());
}

/// C 2626-2630: `static gcc_jit_rvalue *emit_setcdr (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_setcdr (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  return emit_call2_with_type_hint (comp.setcdr, insn, Qcons ());
}

/// C 2632-2639: `static gcc_jit_rvalue *emit_numperp (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_numperp (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  let x: *mut gcc_jit_rvalue = emit_mvar_rval (SECOND! (insn));
  let mut res: *mut gcc_jit_rvalue = emit_NUMBERP (x);
  return gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (),
				   comp.bool_to_lisp_obj, 1,
				   &raw mut res);
}

/// C 2641-2648: `static gcc_jit_rvalue *emit_integerp (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_integerp (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  let x: *mut gcc_jit_rvalue = emit_mvar_rval (SECOND! (insn));
  let mut res: *mut gcc_jit_rvalue = emit_INTEGERP (x);
  return gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (),
				   comp.bool_to_lisp_obj, 1,
				   &raw mut res);
}

/// C 2650-2655: `static gcc_jit_rvalue *emit_maybe_gc_or_quit (Lisp_Object insn)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_maybe_gc_or_quit (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  return gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (),
				   comp.maybe_gc_or_quit, 0,
				   std::ptr::null_mut ());
}


/* Register an handler for a non local exit.  */

/// C 2196-2222: `static void emit_limple_push_handler (gcc_jit_rvalue *handler,
/// gcc_jit_rvalue *handler_type, gcc_jit_block *handler_bb, gcc_jit_block *guarded_bb,
/// Lisp_Object clobbered_mvar)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_limple_push_handler (handler: *mut gcc_jit_rvalue,
				    handler_type: *mut gcc_jit_rvalue,
				    handler_bb: *mut gcc_jit_block,
				    guarded_bb: *mut gcc_jit_block,
				    clobbered_mvar: Lisp_Object)
{
   /* struct handler *c = push_handler (POP, type);  */

  let mut args: [*mut gcc_jit_rvalue; 2] = [ handler, handler_type ];
  gcc_jit_block_add_assignment (
    comp.block,
    std::ptr::null_mut (),
    comp.loc_handler,
    emit_call (intern_c_string (c"push_handler".as_ptr ()),
	       comp.handler_ptr_type, 2, args.as_mut_ptr (), false));

  args[0] =
    gcc_jit_lvalue_get_address (
	gcc_jit_rvalue_dereference_field (
	  gcc_jit_lvalue_as_rvalue (comp.loc_handler),
	  std::ptr::null_mut (),
	  comp.handler_jmp_field),
	std::ptr::null_mut ());

  let res: *mut gcc_jit_rvalue = emit_setjmp (args[0]);
  emit_cond_jump (res, handler_bb, guarded_bb);
}

/* This is for a regular function with arguments as m-var.  */

/// C 2037-2052: `static gcc_jit_rvalue *emit_simple_limple_call (Lisp_Object args,
/// gcc_jit_type *ret_type, bool direct)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_simple_limple_call (args_in: Lisp_Object, ret_type: *mut gcc_jit_type,
				   direct: bool) -> *mut gcc_jit_rvalue
{
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let callee: Lisp_Object = FIRST! (args_in);
  let mut args: Lisp_Object = XCDR (args_in);
  let mut i: ptrdiff_t = 0;
  let nargs: ptrdiff_t = list_length (args);
  let mut gcc_args: *mut *mut gcc_jit_rvalue = std::ptr::null_mut ();
  SAFE_NALLOCA! (sa_avail, gcc_args, 1, nargs);
  FOR_EACH_TAIL! (args, {
    *gcc_args.offset (i) = emit_mvar_rval (XCAR (args));
    i += 1;
  });

  let res: *mut gcc_jit_rvalue = emit_call (callee, ret_type, nargs, gcc_args, direct);
  SAFE_FREE! (sa_count, sa_mark);
  return res;
}

/// C 2054-2061: `static gcc_jit_rvalue *emit_simple_limple_call_lisp_ret (Lisp_Object args)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_simple_limple_call_lisp_ret (args: Lisp_Object) -> *mut gcc_jit_rvalue
{
  /*
    Ex: (call Fcons #s(comp-mvar 3 0 t 1 nil) #s(comp-mvar 4 nil t nil nil)).
  */
  return emit_simple_limple_call (args, comp.lisp_obj_type, false);
}

/// C 2063-2067: `static gcc_jit_rvalue *emit_simple_limple_call_void_ret (Lisp_Object args)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_simple_limple_call_void_ret (args: Lisp_Object) -> *mut gcc_jit_rvalue
{
  return emit_simple_limple_call (args, comp.void_type, false);
}

/* Entry point to dispatch emitting (call fun ...).  */

/// C 2071-2084: `static gcc_jit_rvalue *emit_limple_call (Lisp_Object insn)`.  C's
/// `gcc_jit_rvalue * (*emitter_ptr) (Lisp_Object)` is a function pointer the emitter registry
/// hands back as `void *`; the port has the same alias and the same `xmint_pointer` recovery.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_limple_call (insn: Lisp_Object) -> *mut gcc_jit_rvalue
{
  let callee_sym: Lisp_Object = FIRST! (insn);
  let emitter: Lisp_Object = Fgethash (callee_sym, comp.emitter_dispatcher, Qnil ());

  if ! NILP (emitter)
    {
      let emitter_ptr: Llisp_emitter = std::mem::transmute (xmint_pointer (emitter));
      return emitter_ptr (insn);
    }

  return emit_simple_limple_call_lisp_ret (insn);
}

/* C 4932-4951's `DEFUN ("comp-libgccjit-version", …)` is transcribed **here**, out of C's order,
   because this slice's `emit_EQ` (C 1529) calls it and nothing between C 1520 and C 4932 defines
   it.  Rust resolves items regardless of position, so the body is C's; it moves to its own
   position when the C 4932 region is transcribed (`docs/ports/comp.md` records the move).
   `#pragma GCC diagnostic push`/`pop` and the ignored `-Waddress` around it are C's own
   diagnostics control and have no Rust counterpart.  */
/// C 4932-4951: `DEFUN ("comp-libgccjit-version", Fcomp_libgccjit_version, …)`.
/// `LIBGCCJIT_HAVE_gcc_jit_version` is undefined in the frozen configuration, so C 4948-4950 is
/// the live arm and C 4940-4947 is carried under the feature that names the macro.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp-libgccjit-version", Fcomp_libgccjit_version,
       Scomp_libgccjit_version, 0, 0, 0,
       doc: "Return libgccjit version in use.\n\nThe return value has the form (MAJOR MINOR PATCHLEVEL) or nil if\nunknown (before GCC version 10).",
       (),
       {
  #[cfg (feature = "libgccjit_have_gcc_jit_version")]
  {
    load_gccjit_if_necessary (true);

    return if *gcc_jit_version_major.as_ref ().unwrap () != std::ptr::null_mut ()
      { list3 (make_fixnum (gcc_jit_version_major () as EMACS_INT),
	       make_fixnum (gcc_jit_version_minor () as EMACS_INT),
	       make_fixnum (gcc_jit_version_patchlevel () as EMACS_INT)) }
      else
      { Qnil () };
  }
  #[cfg (not (feature = "libgccjit_have_gcc_jit_version"))]
  {
    return Qnil ();
  }
});

/// C 1520-1552: `static gcc_jit_rvalue *emit_EQ (gcc_jit_rvalue *x, gcc_jit_rvalue *y)`.  C's
/// `__builtin_expect` arm is guarded by a run-time version test (`bug#79722`), so both arms are
/// live code here, as in C.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_EQ (x: *mut gcc_jit_rvalue, y: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  let base_eq: *mut gcc_jit_rvalue = emit_BASE_EQ (x, y);
  let mut symbols_with_pos_enabled_rval: *mut gcc_jit_rvalue = gcc_jit_lvalue_as_rvalue (
    gcc_jit_rvalue_dereference (comp.f_symbols_with_pos_enabled_ref,
                                std::ptr::null_mut ()));

  /* Do not use '__builtin_expect' with libgccjit < 14 (bug#79722).  */
  let version: Lisp_Object = Fcomp_libgccjit_version ();
  if ! NILP (version) && XFIXNUM (XCAR (version)) >= 14
    {
      let expect_args: [*mut gcc_jit_rvalue; 2] =
        [ emit_coerce (comp.long_type, symbols_with_pos_enabled_rval),
          gcc_jit_context_new_rvalue_from_int (comp.ctxt,
                                               comp.long_type,
                                               0) ];

      symbols_with_pos_enabled_rval = emit_coerce (
        comp.bool_type,
        gcc_jit_context_new_call (
          comp.ctxt,
          std::ptr::null_mut (),
          gcc_jit_context_get_builtin_function (comp.ctxt,
                                                c"__builtin_expect".as_ptr ()),
          2,
          expect_args.as_ptr () as *mut *mut gcc_jit_rvalue));
    }

  return emit_OR (
    base_eq,
    emit_AND (symbols_with_pos_enabled_rval, emit_slow_eq (x, y)));
}

/// C 1554-1560: `static gcc_jit_rvalue *emit_FLOATP (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_FLOATP (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"FLOATP".as_ptr ());

  return emit_TAGGEDP (obj, Lisp_Type::Lisp_Float as Lisp_Word_tag);
}

/// C 1562-1579: `static gcc_jit_rvalue *emit_BIGNUMP (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_BIGNUMP (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  /* PSEUDOVECTORP (x, PVEC_BIGNUM); */
  emit_comment (c"BIGNUMP".as_ptr ());

  let args: [*mut gcc_jit_rvalue; 2] =
    [ obj,
      gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					   comp.int_type,
					   pvec_type::PVEC_BIGNUM as c_int) ];

  return gcc_jit_context_new_call (comp.ctxt,
				   std::ptr::null_mut (),
				   comp.pseudovectorp,
				   2,
				   args.as_ptr () as *mut *mut gcc_jit_rvalue);
}

/// C 1581-1625: `static gcc_jit_rvalue *emit_FIXNUMP (gcc_jit_rvalue *obj)`.  C's
/// `(Lisp_Int0 >> !USE_LSB_TAG)` and the `USE_LSB_TAG ? obj : …` selection are constants; the
/// port reads the same ones.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_FIXNUMP (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  /* (! (((unsigned) (XLI (x) >> (USE_LSB_TAG ? 0 : FIXNUM_BITS))
	- (unsigned) (Lisp_Int0 >> !USE_LSB_TAG))
	& ((1 << INTTYPEBITS) - 1)))  */
  emit_comment (c"FIXNUMP".as_ptr ());

  let sh_res: *mut gcc_jit_rvalue =
    if USE_LSB_TAG
    { obj }
    else
    { emit_binary_op (GCC_JIT_BINARY_OP_RSHIFT,
		      comp.emacs_int_type,
		      emit_XLI (obj),
		      gcc_jit_context_new_rvalue_from_int (
			comp.ctxt,
			comp.emacs_int_type,
			FIXNUM_BITS as c_int)) };

  let minus_res: *mut gcc_jit_rvalue =
    emit_binary_op (
      GCC_JIT_BINARY_OP_MINUS,
	   comp.unsigned_type,
	   sh_res,
	   gcc_jit_context_new_rvalue_from_int (
	     comp.ctxt,
	     comp.unsigned_type,
	     (Lisp_Type::Lisp_Int0 as c_int >> if ! USE_LSB_TAG { 1 } else { 0 })));

  let res: *mut gcc_jit_rvalue =
   gcc_jit_context_new_unary_op (
     comp.ctxt,
     std::ptr::null_mut (),
     GCC_JIT_UNARY_OP_LOGICAL_NEGATE,
     comp.int_type,
     emit_binary_op (
       GCC_JIT_BINARY_OP_BITWISE_AND,
       comp.unsigned_type,
       minus_res,
       gcc_jit_context_new_rvalue_from_int (
	 comp.ctxt,
	 comp.unsigned_type,
	 ((1usize << INTTYPEBITS) - 1) as c_int)));

  return res;
}

/// C 1627-1653: `static gcc_jit_rvalue *emit_XFIXNUM (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_XFIXNUM (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"XFIXNUM".as_ptr ());
  let mut i: *mut gcc_jit_rvalue = emit_coerce (comp.emacs_uint_type, emit_XLI (obj));

  /* FIXME: Implementation dependent (both RSHIFT are arithmetic).  */

  if ! USE_LSB_TAG
    {
      i = emit_binary_op (GCC_JIT_BINARY_OP_LSHIFT,
			  comp.emacs_uint_type,
			  i,
			  comp.inttypebits);

      return emit_binary_op (GCC_JIT_BINARY_OP_RSHIFT,
			     comp.emacs_int_type,
			     i,
			     comp.inttypebits);
    }
  else
    { return emit_coerce (comp.emacs_int_type,
			  emit_binary_op (GCC_JIT_BINARY_OP_RSHIFT,
					  comp.emacs_int_type,
					  i,
					  comp.inttypebits)); }
}

/// C 1655-1664: `static gcc_jit_rvalue *emit_INTEGERP (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_INTEGERP (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"INTEGERP".as_ptr ());

  return emit_binary_op (GCC_JIT_BINARY_OP_LOGICAL_OR,
			 comp.bool_type,
			 emit_FIXNUMP (obj),
			 emit_BIGNUMP (obj));
}

/// C 1666-1675: `static gcc_jit_rvalue *emit_NUMBERP (gcc_jit_rvalue *obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_NUMBERP (obj: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  emit_comment (c"NUMBERP".as_ptr ());

  return emit_binary_op (GCC_JIT_BINARY_OP_LOGICAL_OR,
			 comp.bool_type,
			 emit_INTEGERP (obj),
			 emit_FLOATP (obj));
}

/// C 1677-1696: `static gcc_jit_rvalue *emit_make_fixnum_LSB_TAG (gcc_jit_rvalue *n)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_make_fixnum_LSB_TAG (n: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  /*
    EMACS_UINT u = n;
    n = u << INTTYPEBITS;
    n += int0;
  */

  let mut tmp: *mut gcc_jit_rvalue =
    emit_binary_op (GCC_JIT_BINARY_OP_LSHIFT,
		    comp.emacs_int_type,
		    n, comp.inttypebits);

  tmp = emit_binary_op (GCC_JIT_BINARY_OP_PLUS,
			comp.emacs_int_type,
			tmp, comp.lisp_int0);

  return emit_coerce (comp.lisp_obj_type, tmp);
}

/// C 1506-1518: `static gcc_jit_rvalue *emit_slow_eq (gcc_jit_rvalue *x, gcc_jit_rvalue *y)`.
/// C's `countof (args)` is the array's length, a constant; the port passes the same 2.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_slow_eq (x: *mut gcc_jit_rvalue, y: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  let args: [*mut gcc_jit_rvalue; 2] =
    [ emit_coerce (comp.lisp_obj_type, x),
      emit_coerce (comp.lisp_obj_type, y) ];

  return emit_call (intern_c_string (c"slow_eq".as_ptr ()),
                    comp.bool_type,
		    countof! (args) as ptrdiff_t,
                    args.as_ptr () as *mut *mut gcc_jit_rvalue,
                    false);
}

/// C 1120-1197: `static gcc_jit_rvalue *emit_coerce (gcc_jit_type *new_type, gcc_jit_rvalue *obj)`.
/// C's two `#ifdef` arms are carried as the gates they name: `LISP_OBJECT_IS_STRUCT` (undefined in
/// the frozen configuration, and no flip turns it on) and
/// `LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast` (also undefined there, so C 1185-1196 is the live
/// side - the cast matrix `type_to_cast_index` indexes).
#[cfg (feature = "have_native_comp")]
unsafe fn emit_coerce (new_type: *mut gcc_jit_type, obj: *mut gcc_jit_rvalue)
  -> *mut gcc_jit_rvalue
{
  let old_type: *mut gcc_jit_type = gcc_jit_rvalue_get_type (obj);

  if new_type == old_type
    { return obj; }

  #[cfg (feature = "lisp_object_is_struct")]
  {
    if old_type == comp.lisp_obj_type
      {
	let lwordobj: *mut gcc_jit_rvalue =
	  gcc_jit_rvalue_access_field (obj, std::ptr::null_mut (), comp.lisp_obj_i);
	return emit_coerce (new_type, lwordobj);
      }

    if new_type == comp.lisp_obj_type
      {
	let lwordobj: *mut gcc_jit_rvalue =
	  emit_coerce (comp.lisp_word_type, obj);

	// C's `static ptrdiff_t i;`: a function-scope static, and C reads `i++` (the value first).
	static mut i: ptrdiff_t = 0;
	let counter: ptrdiff_t = i;
	i += 1;
	let tmp_s: *mut gcc_jit_lvalue =
	  gcc_jit_function_new_local (comp.func, std::ptr::null_mut (), comp.lisp_obj_type,
				      format_string (c"lisp_obj_%td".as_ptr (), counter));

	gcc_jit_block_add_assignment (
	  comp.block, std::ptr::null_mut (),
	  gcc_jit_lvalue_access_field (tmp_s, std::ptr::null_mut (),
				       comp.lisp_obj_i),
	  lwordobj);
	return gcc_jit_lvalue_as_rvalue (tmp_s);
      }
  }

  #[cfg (feature = "libgccjit_have_gcc_jit_context_new_bitcast")]
  {
    let old_is_ptr: bool = ! gcc_jit_type_is_pointer (old_type).is_null ();
    let new_is_ptr: bool = ! gcc_jit_type_is_pointer (new_type).is_null ();

    let mut tmp: *mut gcc_jit_rvalue = obj;

    /* `gcc_jit_context_new_bitcast` requires that the types being converted
       between have the same layout and as such, doesn't allow converting
       between an arbitrarily sized integer/boolean and a pointer. Casting it
       to a uintptr/void* is still necessary, to ensure that it can be bitcast
       into a (void *)/uintptr respectively.  */
    if old_is_ptr != new_is_ptr
      {
	if old_is_ptr
	  {
	    tmp = gcc_jit_context_new_cast (comp.ctxt, std::ptr::null_mut (), tmp,
					    comp.void_ptr_type);
	    tmp = gcc_jit_context_new_bitcast (comp.ctxt, std::ptr::null_mut (), tmp,
					       comp.uintptr_type);
	  }
	else
	  {
	    tmp = gcc_jit_context_new_cast (comp.ctxt, std::ptr::null_mut (), tmp,
					    comp.uintptr_type);
	    tmp = gcc_jit_context_new_bitcast (comp.ctxt, std::ptr::null_mut (), tmp,
					       comp.void_ptr_type);
	  }
      }
    return gcc_jit_context_new_cast (comp.ctxt, std::ptr::null_mut (), tmp, new_type);
  }

  /* C 1185-1196: `#else /* !LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast */`.  C passes `&obj` -
     the address of its own parameter - and the port spells that with a binding of its own.  */
  #[cfg (not (feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
  {
    let old_index: c_int = type_to_cast_index (old_type);
    let new_index: c_int = type_to_cast_index (new_type);

    /* Lookup the appropriate cast function in the cast matrix.  */
    let mut obj_: *mut gcc_jit_rvalue = obj;
    return gcc_jit_context_new_call (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.cast_functions_from_to[old_index as usize]
				       [new_index as usize],
				     1, &raw mut obj_);
  }
}

/// C 1199-1210: `static gcc_jit_rvalue *emit_binary_op (enum gcc_jit_binary_op op,
/// gcc_jit_type *result_type, gcc_jit_rvalue *a, gcc_jit_rvalue *b)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_binary_op (op: gcc_jit_binary_op, result_type: *mut gcc_jit_type,
			  a: *mut gcc_jit_rvalue, b: *mut gcc_jit_rvalue) -> *mut gcc_jit_rvalue
{
  /* FIXME Check here for possible UB.  */
  return gcc_jit_context_new_binary_op (comp.ctxt, std::ptr::null_mut (),
					op,
					result_type,
					emit_coerce (result_type, a),
					emit_coerce (result_type, b));
}

/// C 1012-1066: `static gcc_jit_rvalue *emit_call (Lisp_Object func, gcc_jit_type *ret_type,
/// ptrdiff_t nargs, gcc_jit_rvalue **args, bool direct)`.  `ret_type` is unused by C's own body
/// (the declaration keeps it), hence the file's `allow (unused_variables)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_call (func: Lisp_Object, ret_type: *mut gcc_jit_type, nargs: ptrdiff_t,
                     args: *mut *mut gcc_jit_rvalue, direct: bool) -> *mut gcc_jit_rvalue
{
  let gcc_func: Lisp_Object =
    Fgethash (func,
              if direct { comp.exported_funcs_h } else { comp.imported_funcs_h },
              Qnil ());

  if NILP (gcc_func)
    { xsignal2 (Qnative_ice (), build_string (c"missing function declaration".as_ptr ()), func); }

  if direct
    {
      emit_comment (format_string (c"direct call to: %s".as_ptr (), SSDATA (func)));
      return gcc_jit_context_new_call (comp.ctxt,
                                       std::ptr::null_mut (),
                                       xmint_pointer (gcc_func) as *mut gcc_jit_function,
                                       nargs as c_int,
                                       args);
    }
  else
    {
      /* Inline functions so far don't have a local variable for
	 function reloc table so we fall back to the global one.  Even
	 if this is not aesthetic calling into C from open-code is
	 always a fallback and therefore not be performance critical.
	 To fix this could think do the inline our-self without
	 relying on GCC. */
      let f_ptr: *mut gcc_jit_lvalue =
	gcc_jit_rvalue_dereference_field (
	  gcc_jit_lvalue_as_rvalue (if ! comp.func_relocs_local.is_null ()
				      { comp.func_relocs_local }
				      else
				      { comp.func_relocs }),
	  std::ptr::null_mut (),
	  xmint_pointer (gcc_func) as *mut gcc_jit_field);

      if f_ptr.is_null ()
	{ xsignal2 (Qnative_ice (),
		    build_string (c"missing function relocation".as_ptr ()),
		    func); }
      emit_comment (format_string (c"calling subr: %s".as_ptr (),
				   SSDATA (SYMBOL_NAME (func))));
      return gcc_jit_context_new_call_through_ptr (comp.ctxt,
						   std::ptr::null_mut (),
						   gcc_jit_lvalue_as_rvalue (f_ptr),
						   nargs as c_int,
						   args);
    }
}

/// C 1068-1078: `static gcc_jit_rvalue *emit_call_ref (Lisp_Object func, ptrdiff_t nargs,
/// gcc_jit_lvalue *base_arg, bool direct)`.  C's `gcc_jit_rvalue *args[]` is a two-element array
/// on the stack, passed as the pointer C's `emit_call` reads.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_call_ref (func: Lisp_Object, nargs: ptrdiff_t, base_arg: *mut gcc_jit_lvalue,
			 direct: bool) -> *mut gcc_jit_rvalue
{
  let args: [*mut gcc_jit_rvalue; 2] =
    [ gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					   comp.ptrdiff_type,
					   nargs as c_int),
      gcc_jit_lvalue_get_address (base_arg, std::ptr::null_mut ()) ];
  return emit_call (func, comp.lisp_obj_type, 2,
		    args.as_ptr () as *mut *mut gcc_jit_rvalue, direct);
}

/* Close current basic block emitting a conditional.  */

/// C 1080-1106: `static void emit_cond_jump (gcc_jit_rvalue *test, gcc_jit_block *then_target,
/// gcc_jit_block *else_target)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_cond_jump (test: *mut gcc_jit_rvalue, then_target: *mut gcc_jit_block,
			  else_target: *mut gcc_jit_block)
{
  if gcc_jit_rvalue_get_type (test) == comp.bool_type
    { gcc_jit_block_end_with_conditional (comp.block, std::ptr::null_mut (), test,
					  then_target, else_target); }
  else
    /* In case test is not bool we do a logical negation to obtain a bool as
       result.  */
    { gcc_jit_block_end_with_conditional (
	comp.block,
	std::ptr::null_mut (),
	gcc_jit_context_new_unary_op (comp.ctxt,
				      std::ptr::null_mut (),
				      GCC_JIT_UNARY_OP_LOGICAL_NEGATE,
				      comp.bool_type,
				      test),
	else_target,
	then_target); }
}

/// C 1108-1118: `#ifndef LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast` / `static int
/// type_to_cast_index (gcc_jit_type *type)`.  The gate is the same one `NUM_CAST_TYPES` (C
/// 538-540) carries: the macro is undefined in the frozen configuration, so this arm is the live
/// side there.
#[cfg (all (feature = "have_native_comp",
	    not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
unsafe fn type_to_cast_index (type_: *mut gcc_jit_type) -> c_int
{
  let mut i: c_int = 0;
  while i < NUM_CAST_TYPES as c_int
    {
      if type_ == comp.cast_types[i as usize]
	{ return i; }
      i += 1;
    }

  xsignal1 (Qnative_ice (), build_string (c"unsupported cast".as_ptr ()));
}

/// C 943-950: `static void emit_comment (const char *str)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_comment (str_: *const c_char)
{
  if comp.debug != 0
    { gcc_jit_block_add_comment (comp.block, std::ptr::null_mut (), str_); }
}

/// C 952-1010: `static gcc_jit_field *declare_imported_func (...)`.
#[cfg (feature = "have_native_comp")]
unsafe fn declare_imported_func (subr_sym: Lisp_Object,
                                                ret_type: *mut gcc_jit_type,
                                                nargs: c_int,
                                                types_in: *mut *mut gcc_jit_type)
  -> *mut gcc_jit_field
{
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let mut nargs: c_int = nargs;
  let mut types: *mut *mut gcc_jit_type = types_in;
  /* Don't want to declare the same function two times.  */
  if ! NILP (Fgethash (subr_sym, comp.imported_funcs_h, Qnil ()))
    { xsignal2 (Qnative_ice (),
                build_string (c"unexpected double function declaration".as_ptr ()),
                subr_sym); }

  if nargs as isize == MANY
    {
      nargs = 2;
      types = SAFE_ALLOCA! (sa_avail,
                            nargs as usize * core::mem::size_of::<*mut gcc_jit_type> ())
                as *mut *mut gcc_jit_type;
      *types.offset (0) = comp.ptrdiff_type;
      *types.offset (1) = comp.lisp_obj_ptr_type;
    }
  else if nargs as isize == UNEVALLED
    {
      nargs = 1;
      types = SAFE_ALLOCA! (sa_avail,
                            nargs as usize * core::mem::size_of::<*mut gcc_jit_type> ())
                as *mut *mut gcc_jit_type;
      *types.offset (0) = comp.lisp_obj_type;
    }
  else if types_in.is_null ()
    {
      SAFE_NALLOCA! (sa_avail, types, 1, nargs as usize);
      let mut i: ptrdiff_t = 0;
      while i < nargs as ptrdiff_t
        {
          *types.offset (i as isize) = comp.lisp_obj_type;
          i += 1;
        }
    }

  /* String containing the function ptr name.  */
  let f_ptr_name: Lisp_Object =
    calln! (intern_c_string (c"comp-c-func-name".as_ptr ()),
            subr_sym, make_string (c"R".as_ptr (), 1));

  let f_ptr_type: *mut gcc_jit_type =
    gcc_jit_type_get_const (
      gcc_jit_context_new_function_ptr_type (comp.ctxt,
                                             std::ptr::null_mut (),
                                             ret_type,
                                             nargs,
                                             types,
                                             0));
  let field: *mut gcc_jit_field =
    gcc_jit_context_new_field (comp.ctxt,
                               std::ptr::null_mut (),
                               f_ptr_type,
                               SSDATA (f_ptr_name));

  Fputhash (subr_sym, make_mint_ptr (field as *mut c_void), comp.imported_funcs_h);
  SAFE_FREE! (sa_count, sa_mark);
  return field;
}

/// C 5593-5857: `syms_of_comp`.  C 5596-5854's `#ifdef HAVE_NATIVE_COMP` region is this file's
/// other dead region (the `native-comp-jit-compilation` `DEFVAR_BOOL`, the `comp--#$`
/// `DEFVAR_LISP` and the fifty-odd `DEFSYM`s for the Limple opcodes and the compiler options);
/// it is **not yet transcribed** - it is outside this slice's C 25-822 and `docs/ports/comp.md`
/// records it as the remaining work.  C 5856's `defsubr` is outside the `#ifdef`, as C has it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_comp ()
{
  defsubr (&raw mut Snative_comp_available_p);
}
