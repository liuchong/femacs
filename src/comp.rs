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
//     registered** in `xtask/Cargo.toml`'s `[features]` table (only `have_native_comp` and
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
// first place the file needs `FOR_EACH_TAIL`, one of the macros `src/include/w32common.h`
// 72-97's neighbours keep `deferred` in `docs/ports/excluded-macros.txt` (P-22).  C 824-5857 is
// slice 2's and is recorded in the ledger as not yet transcribed.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]

include!("include/config.rs");
include!("include/epaths.rs");                    // C: `<epaths.h>` (generated; see epaths.in's android arm)

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

/* C 22's `<string.h>` reaches this unit through `lisp.h`, and the mirror
   (`lib/include/string.rs`) carries neither of these two names: `strnlen` and `mempcpy` are
   declared here the way `doprnt.rs` 69 and `androidvfs.rs` 244 declare them (P-55's use-site
   shape, which P-26 leaves for a name the shared mirror does not have).  */
#[cfg (feature = "have_native_comp")]
unsafe extern "C" {
  pub fn strnlen (s: *const c_char, maxlen: size_t) -> size_t;
  pub fn mempcpy (dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

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
/// `__VA_ARGS__; static W32_PFN_##func fn_##func` - the macro `src/include/w32common.rs`
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
/// definition (`src/include/w32common.rs` 165).
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
/// declaration `src/include/w32.rs` carries for `w32.h` 187 (`extern HMODULE
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
   use-site declaration, the shape `src/emacs.rs` 193 and `src/pdumper.rs` 143 use
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
   src lib` is empty, `src/android.rs` 1347-1348's note), so the unit declares
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
/// `src/alloc.rs` 2643-2650 `make_formatted_string` and `pdumper.rs` 330-336 `dump_trace`
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

/// C 5533-5576: `DEFUN ("native-elisp-load", Fnative_elisp_load, …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("native-elisp-load", Fnative_elisp_load, Snative_elisp_load, 1, 2, 0,
       doc: "Load native elisp code FILENAME.\nLATE-LOAD has to be non-nil when loading for deferred compilation.",
       (filename: Lisp_Object, late_load: Lisp_Object),
       {
  CHECK_STRING (filename);
  if NILP (Ffile_exists_p (filename))
    { xsignal2 (Qnative_lisp_load_failed (), build_string (c"file does not exists".as_ptr ()),
	      filename); }
  let comp_u: *mut Lisp_Native_Comp_Unit = allocate_native_comp_unit ();
  let encoded_filename: Lisp_Object = ENCODE_FILE (filename);

  if ! NILP (Fgethash (filename, Vcomp_loaded_comp_units_h, Qnil ()))
      && ! file_in_eln_sys_dir (filename)
      && ! NILP (Ffile_writable_p (filename))
    {
      /* If in this session there was ever a file loaded with this
	 name, rename it before loading, to make sure we always get a
	 new handle!  */
      let tmp_filename: Lisp_Object =
	Fmake_temp_file_internal (filename, Qnil (), build_string (c".eln.tmp".as_ptr ()),
				  Qnil ());
      if NILP (Ffile_writable_p (tmp_filename))
	{ (*comp_u).handle = dynlib_open_for_eln (SSDATA (encoded_filename)); }
      else
	{
	  Frename_file (filename, tmp_filename, Qt ());
	  (*comp_u).handle = dynlib_open_for_eln (SSDATA (ENCODE_FILE (tmp_filename)));
	  Frename_file (tmp_filename, filename, Qnil ());
	}
    }
  else
    { (*comp_u).handle = dynlib_open_for_eln (SSDATA (encoded_filename)); }

  if (*comp_u).handle.is_null ()
    { xsignal2 (Qnative_lisp_load_failed (), filename,
	      build_string (dynlib_error ())); }
  (*comp_u).file = filename;
  (*comp_u).data_vec = Qnil ();
  (*comp_u).lambda_gc_guard_h = CALLN! (Fmake_hash_table, QCtest (), Qeq ());
  (*comp_u).lambda_c_name_idx_h = CALLN! (Fmake_hash_table, QCtest (), Qequal ());
  return load_comp_unit (comp_u, false, ! NILP (late_load));
});

/* C 5578: `#endif /* HAVE_NATIVE_COMP */` - the file gate's end.  */

/// C 5369-5381: `void unload_comp_unit (struct Lisp_Native_Comp_Unit *cu)`.
#[cfg (feature = "have_native_comp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unload_comp_unit (cu: *mut Lisp_Native_Comp_Unit)
{
  if (*cu).handle.is_null ()
    { return; }

  let saved_cu: *mut Lisp_Object =
    dynlib_sym ((*cu).handle, COMP_UNIT_SYM) as *mut Lisp_Object;
  let mut this_cu: Lisp_Object = Qnil ();
  XSETPSEUDOVECTOR! (this_cu, cu, pvec_type::PVEC_NATIVE_COMP_UNIT);
  if EQ (this_cu, *saved_cu)
    { *saved_cu = Qnil (); }
  dynlib_close ((*cu).handle);
}

/// C 5383-5398: `Lisp_Object native_function_doc (Lisp_Object function)`.
#[cfg (feature = "have_native_comp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn native_function_doc (function: Lisp_Object) -> Lisp_Object
{
  let cu: *mut Lisp_Native_Comp_Unit =
    XNATIVE_COMP_UNIT (Fsubr_native_comp_unit (function));

  if NILP ((*cu).data_fdoc_v)
    { (*cu).data_fdoc_v = load_static_obj (cu, TEXT_FDOC_SYM); }
  if ! VECTORP ((*cu).data_fdoc_v)
    { xsignal2 (Qnative_lisp_file_inconsistent (), (*cu).file,
	      build_string (c"missing documentation vector".as_ptr ())); }
  let doc: EMACS_INT = (*XSUBR (function)).doc as EMACS_INT;
  if doc < 0
    { return AREF ((*cu).data_fdoc_v, -doc - 1); }
  return make_fixnum (doc);
}

/// C 5400-5449: `static Lisp_Object make_subr (...)`.
#[cfg (feature = "have_native_comp")]
unsafe fn make_subr (symbol_name: Lisp_Object, minarg: Lisp_Object, maxarg_in: Lisp_Object,
	   c_name: Lisp_Object, type_: Lisp_Object, doc_idx: Lisp_Object,
	   intspec: Lisp_Object, command_modes: Lisp_Object, comp_u: Lisp_Object)
  -> Lisp_Object
{
  let cu: *mut Lisp_Native_Comp_Unit = XNATIVE_COMP_UNIT (comp_u);
  let handle: dynlib_handle_ptr = (*cu).handle;
  if handle.is_null ()
    { xsignal0 (Qwrong_register_subr_call ()); }

  let func: *mut c_void = dynlib_sym (handle, SSDATA (c_name));
  eassert! (! func.is_null ());
  let x: *mut Aligned_Lisp_Subr =
    allocate_pseudovector (
			  VECSIZE (core::mem::size_of::<Aligned_Lisp_Subr> ()) as c_int,
			  0, VECSIZE (core::mem::size_of::<Aligned_Lisp_Subr> ()) as c_int,
			  pvec_type::PVEC_SUBR) as *mut Aligned_Lisp_Subr;
  let mut minarg: Lisp_Object = minarg;
  let mut maxarg: Lisp_Object = maxarg_in;
  if CONSP (minarg)
    {
      /* Dynamic code.  */
      (*x).s.lambda_list = maxarg;
      maxarg = XCDR (minarg);
      minarg = XCAR (minarg);
    }
  else
    {
      (*x).s.lambda_list = Qnil ();
    }
  (*x).s.function.a0 = Some (std::mem::transmute (func));
  (*x).s.min_args = XFIXNUM (minarg) as i16;   // `Lisp_Subr.min_args` is a `short`
  (*x).s.max_args = if FIXNUMP (maxarg) { XFIXNUM (maxarg) as c_short }
                    else { MANY as c_short };
  (*x).s.symbol_name = xstrdup (SSDATA (symbol_name));
  (*x).s.intspec.native = intspec;
  (*x).s.command_modes = command_modes;
  (*x).s.doc = -XFIXNUM (doc_idx) - 1;        // `Lisp_Subr.doc` is an `EMACS_INT`
  eassert! ((*x).s.doc < 0);
  (*x).s.native_comp_u = comp_u;
  (*x).s.native_c_name = xstrdup (SSDATA (c_name));
  (*x).s.r#type = type_;
  let mut tem: Lisp_Object = Qnil ();
  XSETSUBR! (tem, &raw mut (*x).s);

  return tem;
}

/// C 5451-5482: `DEFUN ("comp--register-lambda", Fcomp__register_lambda, …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp--register-lambda", Fcomp__register_lambda, Scomp__register_lambda,
       7, 7, 0,
       doc: "Register anonymous lambda.\nThis gets called by top_level_run during the load phase.",
       (reloc_idx: Lisp_Object, c_name: Lisp_Object, minarg: Lisp_Object,
        maxarg: Lisp_Object, type_: Lisp_Object, rest: Lisp_Object,
        comp_u: Lisp_Object),
       {
  let doc_idx: Lisp_Object = FIRST! (rest);
  let intspec: Lisp_Object = SECOND! (rest);
  let command_modes: Lisp_Object = THIRD! (rest);

  let cu: *mut Lisp_Native_Comp_Unit = XNATIVE_COMP_UNIT (comp_u);
  if (*cu).loaded_once
    { return Qnil (); }

  let tem: Lisp_Object =
    make_subr (c_name, minarg, maxarg, c_name, type_, doc_idx, intspec,
	       command_modes, comp_u);

  /* We must protect it against GC because the function is not
     reachable through symbols.  */
  Fputhash (tem, Qt (), (*cu).lambda_gc_guard_h);
  /* This is for fixing up the value in d_reloc while resurrecting
     from dump.  See 'dump_do_dump_relocation'.  */
  eassert! (NILP (Fgethash (c_name, (*cu).lambda_c_name_idx_h, Qnil ())));
  Fputhash (c_name, reloc_idx, (*cu).lambda_c_name_idx_h);
  /* Do the real relocation fixup.  */
  *((*cu).data_relocs).offset (XFIXNUM (reloc_idx) as isize) = tem;

  return tem;
});

/// C 5484-5503: `DEFUN ("comp--register-subr", Fcomp__register_subr, …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp--register-subr", Fcomp__register_subr, Scomp__register_subr,
       7, 7, 0,
       doc: "Register exported subr.\nThis gets called by top_level_run during the load phase.",
       (name: Lisp_Object, c_name: Lisp_Object, minarg: Lisp_Object,
        maxarg: Lisp_Object, type_: Lisp_Object, rest: Lisp_Object,
        comp_u: Lisp_Object),
       {
  let doc_idx: Lisp_Object = FIRST! (rest);
  let intspec: Lisp_Object = SECOND! (rest);
  let command_modes: Lisp_Object = THIRD! (rest);

  let tem: Lisp_Object =
    make_subr (SYMBOL_NAME (name), minarg, maxarg, c_name, type_, doc_idx,
	       intspec, command_modes, comp_u);

  /* C's `defalias (name, tem)` is `Fdefalias (name, tem, Qnil)` in lisp.h.  */
  Fdefalias (name, tem, Qnil ());

  return tem;
});

/// C 5505-5518: `DEFUN ("comp--late-register-subr", Fcomp__late_register_subr, …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp--late-register-subr", Fcomp__late_register_subr,
       Scomp__late_register_subr, 7, 7, 0,
       doc: "Register exported subr.\nThis gets called by late_top_level_run during the load phase.",
       (name: Lisp_Object, c_name: Lisp_Object, minarg: Lisp_Object,
        maxarg: Lisp_Object, type_: Lisp_Object, rest: Lisp_Object,
        comp_u: Lisp_Object),
       {
  if ! NILP (Fequal (Fsymbol_function (name),
		     Fgethash (name, Vcomp_deferred_pending_h, Qnil ())))
    { Fcomp__register_subr (name, c_name, minarg, maxarg, type_, rest, comp_u); }
  Fremhash (name, Vcomp_deferred_pending_h);
  return Qnil ();
});

/// C 5520-5531: `static bool file_in_eln_sys_dir (Lisp_Object filename)`.
#[cfg (feature = "have_native_comp")]
unsafe fn file_in_eln_sys_dir (filename: Lisp_Object) -> bool
{
  let mut eln_sys_dir: Lisp_Object = Qnil ();
  let mut tmp: Lisp_Object = Vnative_comp_eln_load_path;
  FOR_EACH_TAIL! (tmp, {
    eln_sys_dir = XCAR (tmp);
  });
  return ! NILP (Fstring_match (Fregexp_quote (Fexpand_file_name (eln_sys_dir,
								 Qnil ())),
			       Fexpand_file_name (filename, Qnil ()),
			       Qnil (), Qnil ()));
}

/// C 5279-5281's `Lisp_Object (*top_level_run)(Lisp_Object)` - the entry point the compiled unit
/// exports, cast out of `dynlib_sym`'s `void *`.
#[cfg (feature = "have_native_comp")]
pub type LLtop_level_fn = Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>;

/// C 5232-5367: `Lisp_Object load_comp_unit (struct Lisp_Native_Comp_Unit *comp_u,
/// bool loading_dump, bool late_load)`.  C's `XSETNATIVE_COMP_UNIT (a, b)` is
/// `XSETPSEUDOVECTOR (a, b, PVEC_NATIVE_COMP_UNIT)` and the mirror has that macro.
#[cfg (feature = "have_native_comp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_comp_unit (comp_u_in: *mut Lisp_Native_Comp_Unit,
					 loading_dump: bool,
					 late_load: bool) -> Lisp_Object
{
  let mut res: Lisp_Object = Qnil ();
  let handle: dynlib_handle_ptr = (*comp_u_in).handle;
  let mut comp_u: *mut Lisp_Native_Comp_Unit = comp_u_in;
  let mut comp_u_lisp_obj: Lisp_Object = Qnil ();
  XSETPSEUDOVECTOR! (comp_u_lisp_obj, comp_u, pvec_type::PVEC_NATIVE_COMP_UNIT);

  let saved_cu: *mut Lisp_Object =
    dynlib_sym (handle, COMP_UNIT_SYM) as *mut Lisp_Object;
  if saved_cu.is_null ()
    { xsignal1 (Qnative_lisp_file_inconsistent (), (*comp_u).file); }
  (*comp_u).loaded_once = ! NILP (*saved_cu);
  let data_eph_relocs: *mut Lisp_Object
    = find_relocs (handle, DATA_RELOC_EPHEMERAL_ADDR_SYM);
  eassume! (! data_eph_relocs.is_null ());

  /* While resurrecting from an image dump loading more than once the
     same compilation unit does not make any sense.  */
  eassert! (! (loading_dump && (*comp_u).loaded_once));

  if (*comp_u).loaded_once
    /* 'dlopen' returns the same handle when trying to load two times
       the same shared.  In this case touching 'd_reloc' etc leads to
       fails in case a frame with a reference to it in a live reg is
       active (native-comp-speed > 0).

       We must *never* mess with static pointers in an already loaded
       eln.  */
    {
      comp_u_lisp_obj = *saved_cu;
      comp_u = XNATIVE_COMP_UNIT (comp_u_lisp_obj);
      (*comp_u).loaded_once = true;
    }
  else
    { *saved_cu = comp_u_lisp_obj; }

  /* Once we are sure to have the right compilation unit we want to
     identify is we have at least another load active on it.  */
  let recursive_load: bool = (*comp_u).load_ongoing;
  (*comp_u).load_ongoing = true;
  let count: specpdl_ref = SPECPDL_INDEX ();
  if ! recursive_load
    { record_unwind_protect (Some (std::mem::transmute (unset_cu_load_ongoing as *const c_void)), comp_u_lisp_obj); }

  freloc_check_fill ();

  let top_level_run: LLtop_level_fn =
    std::mem::transmute (dynlib_sym (handle,
		  if late_load { c"late_top_level_run".as_ptr () }
		  else { c"top_level_run".as_ptr () }));

  /* Always set data_imp_relocs pointer in the compilation unit (in can be
     used in 'dump_do_dump_relocation').  */
  (*comp_u).data_relocs = find_relocs (handle, DATA_RELOC_ADDR_SYM);
  eassert! (! (*comp_u).data_relocs.is_null ());

  if ! (*comp_u).loaded_once
    {
      let current_thread_reloc: *mut *mut c_void =
	dynlib_sym (handle, CURRENT_THREAD_RELOC_SYM) as *mut *mut c_void;
      let f_symbols_with_pos_enabled_reloc: *mut *mut c_void =
	dynlib_sym (handle, F_SYMBOLS_WITH_POS_ENABLED_RELOC_SYM) as *mut *mut c_void;
      let data_relocs: *mut Lisp_Object = (*comp_u).data_relocs;
      let freloc_link_table: *mut *mut c_void =
	dynlib_sym (handle, FUNC_LINK_TABLE_SYM) as *mut *mut c_void;

      if ! (! current_thread_reloc.is_null ()
	    && ! f_symbols_with_pos_enabled_reloc.is_null ()
	    && ! data_relocs.is_null ()
	    && ! data_eph_relocs.is_null ()
	    && ! freloc_link_table.is_null ()
	    && top_level_run.is_some ())
	  || NILP (Fstring_equal (load_static_obj (comp_u, LINK_TABLE_HASH_SYM),
				  Vcomp_abi_hash))
	{ xsignal1 (Qnative_lisp_file_inconsistent (), (*comp_u).file); }

      *current_thread_reloc = &raw mut current_thread as *mut c_void;
      *f_symbols_with_pos_enabled_reloc = &raw mut symbols_with_pos_enabled as *mut c_void;

      /* Imported functions.  */
      *freloc_link_table = freloc.link_table.as_mut_ptr () as *mut c_void;

      /* Imported data.  */
      if ! loading_dump
	{
	  (*comp_u).optimize_qualities =
	    load_static_obj (comp_u, TEXT_OPTIM_QLY_SYM);
	  (*comp_u).data_vec = load_static_obj (comp_u, TEXT_DATA_RELOC_SYM);
	}

      let d_vec_len: EMACS_INT = XFIXNUM (Flength ((*comp_u).data_vec));
      let mut i: EMACS_INT = 0;
      while i < d_vec_len
	{
	  *data_relocs.offset (i as isize) = AREF ((*comp_u).data_vec, i);
	  i += 1;
	}
    }

  if ! loading_dump
    {
      /* Note: data_ephemeral_vec is not GC protected except than by
	 this function frame.  After this functions will be
	 deactivated GC will be free to collect it, but it MUST
	 survive till 'top_level_run' has finished his job.  We store
	 into the ephemeral allocation class only objects that we know
	 are necessary exclusively during the first load.  Once these
	 are collected we don't have to maintain them in the heap
	 forever.  */
      let mut data_ephemeral_vec: Lisp_Object = Qnil ();
      /* In case another load of the same CU is active on the stack
	 all ephemeral data is hold by that frame.  Re-writing
	 'data_ephemeral_vec' would be not only a waste of cycles but
	 more importantly would lead to crashes if the contained data
	 is not cons hashed.  */
      if ! recursive_load
	{
	  data_ephemeral_vec =
	    load_static_obj (comp_u, TEXT_DATA_RELOC_EPHEMERAL_SYM);

	  let d_vec_len: EMACS_INT = XFIXNUM (Flength (data_ephemeral_vec));
	  let mut i: EMACS_INT = 0;
	  while i < d_vec_len
	    {
	      *data_eph_relocs.offset (i as isize) = AREF (data_ephemeral_vec, i);
	      i += 1;
	    }
	}
      /* Executing this will perform all the expected environment
	 modifications.  */
      res = top_level_run.unwrap () (comp_u_lisp_obj);
      /* Make sure data_ephemeral_vec still exists after top_level_run has run.
	 Guard against sibling call optimization (or any other).  */
      data_ephemeral_vec = data_ephemeral_vec;
      eassert! (check_comp_unit_relocs (comp_u));
    }

  if ! recursive_load
    /* Clean-up the load ongoing flag in case.  */
    { unbind_to (count, Qnil ()); }

  register_native_comp_unit (comp_u_lisp_obj);

  return res;
}

/* Queue an asynchronous compilation for the source file defining
   FUNCTION_NAME and perform a late load.

   NOTE: ideally would be nice to move its call simply into Fload but
   we need DEFINITION to guard against function redefinition while
   async compilation happen.  */

/// C 5076-5128: `void maybe_defer_native_compilation (Lisp_Object function_name,
/// Lisp_Object definition)`.  C 5080-5100 is an `#if 0` block of logging code: text the compiler
/// never sees, carried here as a comment so the file's shape stays readable.
#[cfg (feature = "have_native_comp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn maybe_defer_native_compilation (function_name: Lisp_Object,
							 definition: Lisp_Object)
{
  /* C 5080-5100: `#if 0` - `#include <sys/types.h>`, `#include <unistd.h>`, the `log_%d` file and
     the `fprintf`/`fflush` calls.  Dead text from C's own source; no port counterpart.  */
  if ! load_gccjit_if_necessary (false)
    { return; }

  if native_comp_jit_compilation == false
      || noninteractive
      || ! NILP (Vpurify_flag)
      || ! CLOSUREP (definition)
      || ! STRINGP (Vload_true_file_name)
      || ! suffix_p (Vload_true_file_name, c".elc".as_ptr ())
      || ! NILP (Fgethash (Vload_true_file_name, V_comp_no_native_file_h, Qnil ()))
    { return; }

  let mut src: Lisp_Object =
    concat2 (CALLNI! ("file-name-sans-extension", Vload_true_file_name),
	     build_string (c".el".as_ptr ()));
  if NILP (Ffile_exists_p (src))
    {
      src = concat2 (src, build_string (c".gz".as_ptr ()));
      if NILP (Ffile_exists_p (src))
	{ return; }
    }

  Fputhash (function_name, definition, Vcomp_deferred_pending_h);

  pending_funcalls
    = Fcons (list (Qnative__compile_async (), src, Qnil (), Qlate ()),
             pending_funcalls);
}

/**************************************/
/* Functions used to load eln files.  */
/**************************************/

/* Fixup the system eln-cache directory, which is the last entry in
   `native-comp-eln-load-path'.  Argument is a .eln file in that directory.  */
/// C 5137-5158: `void fixup_eln_load_path (Lisp_Object eln_filename)`.
#[cfg (feature = "have_native_comp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixup_eln_load_path (eln_filename: Lisp_Object)
{
  let mut last_cell: Lisp_Object = Qnil ();
  let mut tem: Lisp_Object = Vnative_comp_eln_load_path;
  FOR_EACH_TAIL! (tem, {
    if CONSP (tem)
      { last_cell = tem; }
  });

  let preloaded: [u8; 12] = *b"/preloaded/\0";   // C's `const char preloaded[] = "/preloaded/"`;
  let mut eln_cache_sys: Lisp_Object = Ffile_name_directory (eln_filename);
  let p_preloaded: *const c_char =
    SSDATA (eln_cache_sys).offset (SBYTES (eln_cache_sys)
				   - core::mem::size_of_val (&preloaded) as ptrdiff_t + 1);
  let preloaded_p: bool = strcmp (p_preloaded, preloaded.as_ptr () as *const c_char) == 0;

  /* One or two directories up...  */
  let mut i: c_int = 0;
  while i < (if preloaded_p { 2 } else { 1 })
    {
      eln_cache_sys =
	Ffile_name_directory (Fsubstring_no_properties (eln_cache_sys, Qnil (),
						      make_fixnum (-1)));
      i += 1;
    }
  Fsetcar (last_cell, eln_cache_sys);
}

/// C 5179's `static_obj_t *(*f) (void)` and C 5191's `Lisp_Object *(*fun) (void)` - the two
/// function-pointer types `dynlib_sym` hands back as `void *`.
#[cfg (feature = "have_native_comp")]
pub type LLstatic_obj_fn = Option<unsafe extern "C" fn () -> *mut static_obj_t>;
#[cfg (feature = "have_native_comp")]
pub type LLreloc_fn = Option<unsafe extern "C" fn () -> *mut Lisp_Object>;

/// C 5160: `typedef char *(*comp_lit_str_func) (void);` - the type C names and never uses beyond
/// the typedef itself.
#[cfg (feature = "have_native_comp")]
pub type comp_lit_str_func = unsafe extern "C" fn () -> *mut c_char;

/* Deserialize read and return static object.  */
/// C 5162-5186: `static Lisp_Object load_static_obj (struct Lisp_Native_Comp_Unit *comp_u,
/// const char *name)`.
#[cfg (feature = "have_native_comp")]
unsafe fn load_static_obj (comp_u: *mut Lisp_Native_Comp_Unit, name: *const c_char)
  -> Lisp_Object
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut blob: *mut static_obj_t =
    dynlib_sym ((*comp_u).handle, format_string (c"%s_blob".as_ptr (), name))
      as *mut static_obj_t;
  /* Special value so we can recognize #$, which is used for entries in
     the static vector that must be overwritten at load time.  This is a
     specific string that contains "#$", which is not EQ to any
     legitimate object returned by Fread.  */
  specbind (intern_c_string (c"load-file-name".as_ptr ()),
	    Vcomp__hashdollar);
  if ! blob.is_null ()
    /* New blob format.  */
    { return unbind_to (count, Fread (make_string ((*blob).data.as_ptr (), (*blob).len))); }

  let f: LLstatic_obj_fn = std::mem::transmute (dynlib_sym ((*comp_u).handle, name));
  if f.is_none ()
    { xsignal1 (Qnative_lisp_file_inconsistent (), (*comp_u).file); }

  blob = f.unwrap () ();
  return unbind_to (count, Fread (make_string ((*blob).data.as_ptr (), (*blob).len)));

}

/// C 5188-5195: `static Lisp_Object *find_relocs (dynlib_handle_ptr handle, const char *fun_sym)`.
#[cfg (feature = "have_native_comp")]
unsafe fn find_relocs (handle: dynlib_handle_ptr, fun_sym: *const c_char) -> *mut Lisp_Object
{
  let fun: LLreloc_fn = std::mem::transmute (dynlib_sym (handle, fun_sym));
  if fun.is_none ()
    { return std::ptr::null_mut (); }
  return fun.unwrap () ();
}

/* Return false when something is wrong or true otherwise.  */

/// C 5197-5224: `static bool check_comp_unit_relocs (struct Lisp_Native_Comp_Unit *comp_u)`.
#[cfg (feature = "have_native_comp")]
unsafe fn check_comp_unit_relocs (comp_u: *mut Lisp_Native_Comp_Unit) -> bool
{
  let handle: dynlib_handle_ptr = (*comp_u).handle;
  let data_relocs: *mut Lisp_Object
    = find_relocs (handle, DATA_RELOC_ADDR_SYM);
  if data_relocs.is_null ()
    { return false; }

  let d_vec_len: EMACS_INT = XFIXNUM (Flength ((*comp_u).data_vec));

  let mut i: ptrdiff_t = 0;
  while i < d_vec_len
    {
      let x: Lisp_Object = *data_relocs.offset (i as isize);
      if EQ (x, Vcomp__hashdollar)
	{ return false; }
      else if NATIVE_COMP_FUNCTIONP (x)
	{
	  if NILP (Fgethash (x, (*comp_u).lambda_gc_guard_h, Qnil ()))
	    { return false; }
	}
      else if ! EQ (x, AREF ((*comp_u).data_vec, i))
	{ return false; }
      i += 1;
    }
  return true;
}

/// C 5226-5230: `static void unset_cu_load_ongoing (Lisp_Object comp_u)`.
#[cfg (feature = "have_native_comp")]
unsafe fn unset_cu_load_ongoing (comp_u: Lisp_Object)
{
  (*XNATIVE_COMP_UNIT (comp_u)).load_ongoing = false;
}

/* The helper entry points `declare_runtime_imported_funcs` names for the compiled code.  C's
   `bcall0`, `prog_ignore`, `save_restriction_save`/`_restore` and `internal_delete_file` are other
   units' definitions; the ones no mirror names are declared where they are used.  */
#[cfg (feature = "have_native_comp")]
unsafe extern "C" {


  pub fn internal_delete_file (file: Lisp_Object) -> Lisp_Object;
}

/// C 4961-4967: `static void helper_unwind_protect (Lisp_Object handler)`.
#[cfg (feature = "have_native_comp")]
unsafe fn helper_unwind_protect (handler: Lisp_Object)
{
  /* Support for a function here is new in 24.4.  */
  /* C's `FUNCTIONP (handler) ? bcall0 : prog_ignore` is a function-pointer choice; the port's
     `record_unwind_protect` takes an `Option<fn (Lisp_Object)>`, so the two items are cast to
     that type and the condition is C's own test written against the mirror's predicate.  */
  let fnptr: Option<unsafe extern "C" fn (Lisp_Object)> =
    if FUNCTIONP (handler) { Some (std::mem::transmute (bcall0 as *const c_void)) }
    else { Some (std::mem::transmute (prog_ignore as *const c_void)) };
  record_unwind_protect (fnptr, handler);
}

/// C 4969-4973: `static Lisp_Object helper_unbind_n (Lisp_Object n)`.
#[cfg (feature = "have_native_comp")]
unsafe fn helper_unbind_n (n: Lisp_Object) -> Lisp_Object
{
  return unbind_to (specpdl_ref_add (SPECPDL_INDEX (), -XFIXNUM (n)), Qnil ());
}

/// C 4975-4980: `static void helper_save_restriction (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn helper_save_restriction ()
{
  record_unwind_protect (Some (std::mem::transmute (save_restriction_restore as *const c_void)),
			 save_restriction_save ());
}

/// C 4982-4988: `static bool helper_PSEUDOVECTOR_TYPEP_XUNTAG (Lisp_Object a,
/// enum pvec_type code)`.
#[cfg (feature = "have_native_comp")]
unsafe fn helper_PSEUDOVECTOR_TYPEP_XUNTAG (a: Lisp_Object, code: pvec_type) -> bool
{
  return PSEUDOVECTOR_TYPEP (XUNTAG! (a,
				      LISP_WORD_TAG (Lisp_Type::Lisp_Vectorlike as Lisp_Word_tag),
				      vectorlike_header),
			     code);
}

/// C 4990-4995: `static struct Lisp_Symbol_With_Pos *helper_GET_SYMBOL_WITH_POSITION
/// (Lisp_Object a)`.
#[cfg (feature = "have_native_comp")]
unsafe fn helper_GET_SYMBOL_WITH_POSITION (a: Lisp_Object) -> *mut Lisp_Symbol_With_Pos
{
  CHECK_TYPE (SYMBOL_WITH_POS_P (a), Qsymbol_with_pos_p (), a);
  return XUNTAG! (a, LISP_WORD_TAG (Lisp_Type::Lisp_Vectorlike as Lisp_Word_tag),
		  Lisp_Symbol_With_Pos);
}

/// C 4997-5010: `static Lisp_Object helper_sanitizer_assert (Lisp_Object val,
/// Lisp_Object type)`.
#[cfg (feature = "have_native_comp")]
unsafe fn helper_sanitizer_assert (val: Lisp_Object, type_: Lisp_Object) -> Lisp_Object
{
  if ! comp_sanitizer_active
      || ! NILP ((CALLNI! ("cl-typep", val, type_)))
    { return Qnil (); }

  AUTO_STRING! (format, c"Comp sanitizer FAIL for %s with type %s".as_ptr ());
  CALLN! (Fmessage, format, val, type_);
  CALLNI! ("backtrace");
  xsignal2 (Qcomp_sanitizer_error (), val, type_);

  return Qnil ();
}

/* `native-comp-eln-load-path' clean-up support code.  */

#[cfg(all (feature = "have_native_comp", feature = "windowsnt"))]
unsafe fn return_nil (arg: Lisp_Object) -> Lisp_Object
{
  return Qnil ();
}

#[cfg(all (feature = "have_native_comp", feature = "windowsnt"))]
unsafe fn directory_files_matching (name: Lisp_Object, match_: Lisp_Object) -> Lisp_Object
{
  return Fdirectory_files (name, Qt (), match_, Qnil (), Qnil ());
}

/* Windows does not let us delete a .eln file that is currently loaded
   by a process.  The strategy is to rename .eln files into .old.eln
   instead of removing them when this is not possible and clean-up
   `native-comp-eln-load-path' when exiting.

   Any error is ignored because it may be due to the file being loaded
   in another Emacs instance.  */
/// C 5036-5053: `void eln_load_path_final_clean_up (void)`.
#[cfg (feature = "have_native_comp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn eln_load_path_final_clean_up ()
{
  #[cfg (feature = "windowsnt")]
  {
    let mut dir_tail: Lisp_Object = Vnative_comp_eln_load_path;
    FOR_EACH_TAIL! (dir_tail, {
	let files_in_dir: Lisp_Object =
	  internal_condition_case_2 (directory_files_matching as *mut c_void,
				     Fexpand_file_name (Vcomp_native_version_dir,
							XCAR (dir_tail)),
				     build_string (c"\\.eln\\.old\\'".as_ptr ()),
				     Qt (), return_nil as *mut c_void);
	let mut files_in_dir: Lisp_Object = files_in_dir;
	FOR_EACH_TAIL! (files_in_dir, {
	    internal_delete_file (XCAR (files_in_dir));
	  });
      });
  }
}

/* This function puts the compilation unit in the
  `Vcomp_loaded_comp_units_h` hashmap.  */
/// C 5057-5062: `static void register_native_comp_unit (Lisp_Object comp_u)`.
#[cfg (feature = "have_native_comp")]
unsafe fn register_native_comp_unit (comp_u: Lisp_Object)
{
  Fputhash (
    (*XNATIVE_COMP_UNIT (comp_u)).file, comp_u, Vcomp_loaded_comp_units_h);
}

/***********************************/
/* Deferred compilation mechanism. */
/***********************************/

/// C 4778-4928: `DEFUN ("comp--compile-ctxt-to-file0", …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp--compile-ctxt-to-file0", Fcomp__compile_ctxt_to_file0,
       Scomp__compile_ctxt_to_file0,
       1, 1, 0,
       doc: "Compile the current context as native code to file FILENAME.",
       (filename: Lisp_Object),
       {
  load_gccjit_if_necessary (true);

  CHECK_STRING (filename);
  let base_name: Lisp_Object = Fsubstring (filename, Qnil (), make_fixnum (-4));
  let ebase_name: Lisp_Object = ENCODE_FILE (base_name);

  comp.func_relocs_local = std::ptr::null_mut ();

  #[cfg (feature = "windowsnt")]
  {
    let mut ebase_name: Lisp_Object = ansi_encode_filename (ebase_name);
    /* Tell libgccjit the actual file name of the loaded DLL, otherwise
       it will use 'libgccjit.so', which is not useful.  */
    let libgccjit_loaded_from: Lisp_Object = Fget (Qgccjit (), QCloaded_from ());

    if CONSP (libgccjit_loaded_from)
      {
        /* Use the absolute file name if available, otherwise the name
	   we looked for in w32_delayed_load.  */
        let mut libgccjit_fname: Lisp_Object = XCDR (libgccjit_loaded_from);
        if NILP (libgccjit_fname)
	  { libgccjit_fname = XCAR (libgccjit_loaded_from); }
        /* Must encode to ANSI, as libgccjit will not be able to handle
	   UTF-8 encoded file names.  */
        libgccjit_fname = ENCODE_FILE (libgccjit_fname);
        libgccjit_fname = ansi_encode_filename (libgccjit_fname);
        gcc_jit_context_set_str_option (comp.ctxt, GCC_JIT_STR_OPTION_PROGNAME,
				      SSDATA (libgccjit_fname));
      }
    else	/* this should never happen */
      { gcc_jit_context_set_str_option (comp.ctxt, GCC_JIT_STR_OPTION_PROGNAME,
				    c"libgccjit-0.dll".as_ptr ()); }
  }

  comp.speed = XFIXNUM (CALLNI! ("comp-ctxt-speed", Vcomp_ctxt));
  eassert! (comp.speed < INT_MAX as EMACS_INT);
  comp.debug = XFIXNUM (CALLNI! ("comp-ctxt-debug", Vcomp_ctxt));
  eassert! (comp.debug < INT_MAX as EMACS_INT);
  comp.driver_options = CALLNI! ("comp-ctxt-driver-options", Vcomp_ctxt);
  comp.compiler_options = CALLNI! ("comp-ctxt-compiler-options", Vcomp_ctxt);

  if comp.debug != 0
    { gcc_jit_context_set_bool_option (comp.ctxt,
				       GCC_JIT_BOOL_OPTION_DEBUGINFO,
				       1); }
  if comp.debug >= 3
    {
      logfile = emacs_fopen (c"libgccjit.log".as_ptr (), c"w".as_ptr ());
      gcc_jit_context_set_logfile (comp.ctxt,
				   logfile,
				   0, 0);
      gcc_jit_context_set_bool_option (comp.ctxt,
				       GCC_JIT_BOOL_OPTION_KEEP_INTERMEDIATES,
				       1);
      gcc_jit_context_set_bool_option (comp.ctxt,
				       GCC_JIT_BOOL_OPTION_DUMP_EVERYTHING,
				       1);
    }

  gcc_jit_context_set_int_option (comp.ctxt,
				  GCC_JIT_INT_OPTION_OPTIMIZATION_LEVEL,
				  if comp.speed < 0 { 0 }
				  else if comp.speed > 3 { 3 }
				  else { comp.speed as c_int });

  /* On MacOS set a unique dylib ID.  */
  #[cfg (all (feature = "libgccjit_have_gcc_jit_context_add_driver_option",
	      target_os = "macos"))]
  {
    gcc_jit_context_add_driver_option (comp.ctxt, c"-install_name".as_ptr ());
    gcc_jit_context_add_driver_option (
           comp.ctxt, SSDATA (Ffile_name_nondirectory (filename)));
  }

  comp.d_default_idx =
    CALLNI! ("comp-data-container-idx", CALLNI! ("comp-ctxt-d-default", Vcomp_ctxt));
  comp.d_ephemeral_idx =
    CALLNI! ("comp-data-container-idx", CALLNI! ("comp-ctxt-d-ephemeral", Vcomp_ctxt));

  emit_ctxt_code ();

  /* Define inline functions.  */
  define_CAR_CDR ();
  define_PSEUDOVECTORP ();
  define_GET_SYMBOL_WITH_POSITION ();
  define_CHECK_TYPE ();
  define_SYMBOL_WITH_POS_SYM ();
  define_bool_to_lisp_obj ();
  define_setcar_setcdr ();
  define_add1_sub1 ();
  define_negate ();
  define_maybe_gc_or_quit ();

  let func_h: *mut Lisp_Hash_Table =
    XHASH_TABLE (CALLNI! ("comp-ctxt-funcs-h", Vcomp_ctxt));
  DOHASH! (func_h, k, function, { declare_function (function); });
  /* Compile all functions. Can't be done before because the
     relocation structs has to be already defined.  */
  DOHASH! (func_h, k, function, { compile_function (function); });

  /* Work around bug#46495 (GCC PR99126). */
  #[cfg (all (feature = "wide_emacs_int",
	      feature = "libgccjit_have_gcc_jit_context_add_command_line_option"))]
  {
    let version: Lisp_Object = Fcomp_libgccjit_version ();
    if NILP (version)
	|| XFIXNUM (XCAR (version)) < 11
      { gcc_jit_context_add_command_line_option (comp.ctxt,
					     c"-fdisable-tree-isolate-paths".as_ptr ()); }
  }

  add_compiler_options ();
  add_driver_options ();

  if comp.debug > 1
    { gcc_jit_context_dump_to_file (comp.ctxt,
				    format_string (c"%s.c".as_ptr (), SSDATA (ebase_name)),
				    1); }
  if ! NILP (Fsymbol_value (Qcomp_libgccjit_reproducer ()))
    { gcc_jit_context_dump_reproducer_to_file (
      comp.ctxt,
      format_string (c"%s_libgccjit_repro.c".as_ptr (), SSDATA (ebase_name))); }

  let tmp_file: Lisp_Object =
    CALLNI! ("make-temp-file", base_name, Qnil (), build_string (c".eln.tmp".as_ptr ()), Qnil ());

  let mut encoded_tmp_file: Lisp_Object = ENCODE_FILE (tmp_file);
  #[cfg (feature = "windowsnt")]
  {
    encoded_tmp_file = ansi_encode_filename (encoded_tmp_file);
  }
  gcc_jit_context_compile_to_file (comp.ctxt,
				   GCC_JIT_OUTPUT_KIND_DYNAMIC_LIBRARY,
				   SSDATA (encoded_tmp_file));

  let err: *const c_char = gcc_jit_context_get_first_error (comp.ctxt);
  if ! err.is_null ()
    { xsignal3 (Qnative_ice (),
	      build_string (c"failed to compile".as_ptr ()),
	      filename,
	      build_string (err)); }

  CALLNI! ("comp-clean-up-stale-eln", filename);
  CALLNI! ("comp-delete-or-replace-file", filename, tmp_file);

  return filename;
});

/// C 4704-4739: `static void add_driver_options (void)`.  Both gated arms are C's
/// `#if defined (LIBGCCJIT_HAVE_gcc_jit_context_add_driver_option)`; the feature that names the
/// macro is off in the frozen configuration, so the two `#endif` bodies are the live text and the
/// whole block reduces to the two `CONSP` checks - carried exactly as C spells them.
#[cfg (feature = "have_native_comp")]
unsafe fn add_driver_options ()
{
  let mut options: Lisp_Object = Fsymbol_value (Qnative_comp_driver_options ());

  #[cfg (feature = "libgccjit_have_gcc_jit_context_add_driver_option")]
  {
    load_gccjit_if_necessary (true);
    if ! NILP (Fcomp_native_driver_options_effective_p ())
      {
	let mut options: Lisp_Object = options;
	FOR_EACH_TAIL! (options, {
	    gcc_jit_context_add_driver_option (comp.ctxt,
					 /* FIXME: Need to encode
					    this, but how? either
					    ENCODE_FILE or
					    ENCODE_SYSTEM.  */
					 SSDATA (XCAR (options)));
	  });
      }
  }
  if CONSP (options)
    { xsignal1 (Qnative_compiler_error (),
	      build_string (c"Customizing native compiler options via `comp-native-driver-options' is only available on libgccjit version 9 and above.".as_ptr ())); }

  /* Captured `comp-native-driver-options' because file-local.  */
  #[cfg (feature = "libgccjit_have_gcc_jit_context_add_driver_option")]
  {
    options = comp.driver_options;
    if ! NILP (Fcomp_native_driver_options_effective_p ())
      {
	let mut options: Lisp_Object = options;
	FOR_EACH_TAIL! (options, {
	    gcc_jit_context_add_driver_option (comp.ctxt,
					 /* FIXME: Need to encode
					    this, but how? either
					    ENCODE_FILE or
					    ENCODE_SYSTEM.  */
					 SSDATA (XCAR (options)));
	  });
      }
  }
}

/// C 4741-4776: `static void add_compiler_options (void)` - the same shape with the
/// command-line-option macro.
#[cfg (feature = "have_native_comp")]
unsafe fn add_compiler_options ()
{
  let mut options: Lisp_Object = Fsymbol_value (Qnative_comp_compiler_options ());

  #[cfg (feature = "libgccjit_have_gcc_jit_context_add_command_line_option")]
  {
    load_gccjit_if_necessary (true);
    if ! NILP (Fcomp_native_compiler_options_effective_p ())
      {
	let mut options: Lisp_Object = options;
	FOR_EACH_TAIL! (options, {
	    gcc_jit_context_add_command_line_option (comp.ctxt,
                                                 /* FIXME: Need to encode
                                                    this, but how? either
                                                    ENCODE_FILE or
                                                    ENCODE_SYSTEM.  */
                                                 SSDATA (XCAR (options)));
	  });
      }
  }
  if CONSP (options)
    { xsignal1 (Qnative_compiler_error (),
	      build_string (c"Customizing native compiler options via `comp-native-compiler-options' is only available on libgccjit version 9 and above.".as_ptr ())); }

  /* Captured `comp-native-compiler-options' because file-local.  */
  #[cfg (feature = "libgccjit_have_gcc_jit_context_add_command_line_option")]
  {
    options = comp.compiler_options;
    if ! NILP (Fcomp_native_compiler_options_effective_p ())
      {
	let mut options: Lisp_Object = options;
	FOR_EACH_TAIL! (options, {
	    gcc_jit_context_add_command_line_option (comp.ctxt,
                                               /* FIXME: Need to encode
                                                  this, but how? either
                                                  ENCODE_FILE or
                                                  ENCODE_SYSTEM.  */
                                               SSDATA (XCAR (options)));
	  });
      }
  }
}

/// C 4472-4504: `DEFUN ("comp--install-trampoline", Fcomp__install_trampoline, …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp--install-trampoline", Fcomp__install_trampoline,
       Scomp__install_trampoline, 2, 2, 0,
       doc: "Install a TRAMPOLINE for primitive SUBR-NAME.",
       (subr_name: Lisp_Object, trampoline: Lisp_Object),
       {
  CHECK_SYMBOL (subr_name);
  CHECK_SUBR (trampoline);
  let orig_subr: Lisp_Object = Fsymbol_function (subr_name);
  CHECK_SUBR (orig_subr);

  /* FIXME: add a post dump load trampoline machinery to remove this
     check.  */
  if will_dump_p ()
    { signal_error (c"Trying to advice unexpected primitive before dumping".as_ptr (),
		  subr_name); }

  let mut subr_l: Lisp_Object = Vcomp_subr_list;
  let mut i: ptrdiff_t = countof! (helper_link_table) as ptrdiff_t;
  FOR_EACH_TAIL! (subr_l, {
      let subr: Lisp_Object = XCAR (subr_l);
      if EQ (subr, orig_subr)
	{
	  freloc.link_table[i as usize] =
	    (*XSUBR (trampoline)).function.a0.map_or (std::ptr::null_mut (),
						      |a0| a0 as *mut c_void);
	  Fputhash (subr_name, trampoline, Vcomp_installed_trampolines_h);
	  return Qt ();
	}
      i += 1;
    });
    signal_error (c"Trying to install trampoline for non existent subr".as_ptr (),
		  subr_name);
    return Qnil ();
});

/// C 4506-4651: `DEFUN ("comp--init-ctxt", Fcomp__init_ctxt, …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp--init-ctxt", Fcomp__init_ctxt, Scomp__init_ctxt,
       0, 0, 0,
       doc: "Initialize the native compiler context.\nReturn t on success.",
       (),
       {
  load_gccjit_if_necessary (true);

  if ! comp.ctxt.is_null ()
    {
      xsignal1 (Qnative_ice (),
		build_string (c"compiler context already taken".as_ptr ()));
      return Qnil ();
    }

  if NILP (comp.emitter_dispatcher)
    {
      /* Move this into syms_of_comp the day will be dumpable.  */
      comp.emitter_dispatcher = Fmake_hash_table (0, std::ptr::null_mut ());
      register_emitter (Qset_internal (), emit_set_internal as *mut c_void);
      register_emitter (Qhelper_unbind_n (), emit_simple_limple_call_lisp_ret as *mut c_void);
      register_emitter (Qhelper_unwind_protect (),
			emit_simple_limple_call_void_ret as *mut c_void);
      register_emitter (Qrecord_unwind_current_buffer (),
			emit_simple_limple_call_lisp_ret as *mut c_void);
      register_emitter (Qrecord_unwind_protect_excursion (),
			emit_simple_limple_call_void_ret as *mut c_void);
      register_emitter (Qhelper_save_restriction (),
			emit_simple_limple_call_void_ret as *mut c_void);
      register_emitter (Qhelper_sanitizer_assert (),
			emit_simple_limple_call_lisp_ret as *mut c_void);
      /* Inliners.  */
      register_emitter (Qadd1 (), emit_add1 as *mut c_void);
      register_emitter (Qsub1 (), emit_sub1 as *mut c_void);
      register_emitter (Qconsp (), emit_consp as *mut c_void);
      register_emitter (Qeq (), emit_eq as *mut c_void);
      register_emitter (Qcar (), emit_car as *mut c_void);
      register_emitter (Qcdr (), emit_cdr as *mut c_void);
      register_emitter (Qsetcar (), emit_setcar as *mut c_void);
      register_emitter (Qsetcdr (), emit_setcdr as *mut c_void);
      register_emitter (Qnegate (), emit_negate as *mut c_void);
      register_emitter (Qnumberp (), emit_numperp as *mut c_void);
      register_emitter (Qintegerp (), emit_integerp as *mut c_void);
      register_emitter (Qcomp_maybe_gc_or_quit (), emit_maybe_gc_or_quit as *mut c_void);
    }

  comp.ctxt = gcc_jit_context_acquire ();

  comp.void_type = gcc_jit_context_get_type (comp.ctxt, GCC_JIT_TYPE_VOID);
  comp.void_ptr_type =
    gcc_jit_context_get_type (comp.ctxt, GCC_JIT_TYPE_VOID_PTR);
  comp.bool_type = gcc_jit_context_get_type (comp.ctxt, GCC_JIT_TYPE_BOOL);
  comp.char_type = gcc_jit_context_get_type (comp.ctxt, GCC_JIT_TYPE_CHAR);
  comp.int_type = gcc_jit_context_get_type (comp.ctxt, GCC_JIT_TYPE_INT);
  comp.unsigned_type = gcc_jit_context_get_type (comp.ctxt,
						 GCC_JIT_TYPE_UNSIGNED_INT);
  comp.long_type = gcc_jit_context_get_type (comp.ctxt, GCC_JIT_TYPE_LONG);
  comp.unsigned_long_type =
    gcc_jit_context_get_type (comp.ctxt, GCC_JIT_TYPE_UNSIGNED_LONG);
  comp.long_long_type =
    gcc_jit_context_get_type (comp.ctxt, GCC_JIT_TYPE_LONG_LONG);
  comp.unsigned_long_long_type =
    gcc_jit_context_get_type (comp.ctxt, GCC_JIT_TYPE_UNSIGNED_LONG_LONG);
  comp.bool_ptr_type = gcc_jit_type_get_pointer (comp.bool_type);
  comp.char_ptr_type = gcc_jit_type_get_pointer (comp.char_type);
  comp.emacs_int_type = gcc_jit_context_get_int_type (comp.ctxt,
						      core::mem::size_of::<EMACS_INT> () as c_int,
						      1);
  comp.emacs_uint_type = gcc_jit_context_get_int_type (comp.ctxt,
						       core::mem::size_of::<EMACS_UINT> () as c_int,
						       0);
  if LISP_WORDS_ARE_POINTERS
    {
      comp.lisp_word_type =
	gcc_jit_type_get_pointer (
	  gcc_jit_struct_as_type (
	    gcc_jit_context_new_opaque_struct (comp.ctxt,
					   std::ptr::null_mut (),
					   c"Lisp_X".as_ptr ())));
    }
  else
    { comp.lisp_word_type = comp.emacs_int_type; }
  comp.lisp_word_tag_type
    = gcc_jit_context_get_int_type (comp.ctxt,
				    core::mem::size_of::<Lisp_Word_tag> () as c_int, 0);
  #[cfg (feature = "lisp_object_is_struct")]
  {
    comp.lisp_obj_i = gcc_jit_context_new_field (comp.ctxt,
                                               std::ptr::null_mut (),
                                               comp.lisp_word_type,
                                               c"i".as_ptr ());
    comp.lisp_obj_s = gcc_jit_context_new_struct_type (comp.ctxt,
                                                     std::ptr::null_mut (),
                                                     c"Lisp_Object".as_ptr (),
                                                     1,
                                                     &raw mut comp.lisp_obj_i);
    comp.lisp_obj_type = gcc_jit_struct_as_type (comp.lisp_obj_s);
  }
  #[cfg (not (feature = "lisp_object_is_struct"))]
  {
    comp.lisp_obj_type = comp.lisp_word_type;
  }
  comp.lisp_obj_ptr_type = gcc_jit_type_get_pointer (comp.lisp_obj_type);
  comp.zero =
    gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					 comp.emacs_int_type,
					 0);
  comp.one =
    gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					 comp.emacs_int_type,
					 1);
  comp.inttypebits =
    gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					 comp.emacs_uint_type,
					 INTTYPEBITS as c_int);
  comp.lisp_int0 =
    gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					 comp.emacs_int_type,
					 Lisp_Type::Lisp_Int0 as c_int);
  comp.ptrdiff_type = gcc_jit_context_get_int_type (comp.ctxt,
						    core::mem::size_of::<*mut c_void> () as c_int,
						    1);
  comp.uintptr_type = gcc_jit_context_get_int_type (comp.ctxt,
						    core::mem::size_of::<*mut c_void> () as c_int,
						    0);
  comp.size_t_type = gcc_jit_context_get_int_type (comp.ctxt,
						   core::mem::size_of::<size_t> () as c_int,
						   0);

  comp.exported_funcs_h = CALLN! (Fmake_hash_table, QCtest (), Qequal ());
  /*
    Always reinitialize this cause old function definitions are garbage
    collected by libgccjit when the ctxt is released.
  */
  comp.imported_funcs_h = Fmake_hash_table (0, std::ptr::null_mut ());

  define_memcpy ();

  /* Define data structures.  */

  define_lisp_cons ();
  define_lisp_symbol_with_position ();
  define_jmp_buf ();
  define_handler_struct ();
  define_thread_state_struct ();
  #[cfg (not (feature = "libgccjit_have_gcc_jit_context_new_bitcast"))]
  {
    define_cast_functions ();
  }

  return Qt ();
});

/// C 4653-4668: `DEFUN ("comp--release-ctxt", Fcomp__release_ctxt, …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp--release-ctxt", Fcomp__release_ctxt, Scomp__release_ctxt,
       0, 0, 0,
       doc: "Release the native compiler context.",
       (),
       {
  load_gccjit_if_necessary (true);

  if ! comp.ctxt.is_null ()
    { gcc_jit_context_release (comp.ctxt); }

  if ! logfile.is_null ()
    { emacs_fclose (logfile); }
  comp.ctxt = std::ptr::null_mut ();

  return Qt ();
});

/// C 4672-4684: `DEFUN ("comp-native-driver-options-effective-p", …)`.  C's
/// `#if defined (LIBGCCJIT_HAVE_gcc_jit_context_add_driver_option)` arm tests the loaded symbol.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp-native-driver-options-effective-p",
       Fcomp_native_driver_options_effective_p,
       Scomp_native_driver_options_effective_p,
       0, 0, 0,
       doc: "Return t if `comp-native-driver-options' is effective.",
       (),
       {
  #[cfg (feature = "libgccjit_have_gcc_jit_context_add_driver_option")]
  {
    if ! fn_gcc_jit_context_add_driver_option.is_none ()
      { return Qt (); }
  }
  return Qnil ();
});

/// C 4689-4701: `DEFUN ("comp-native-compiler-options-effective-p", …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp-native-compiler-options-effective-p",
       Fcomp_native_compiler_options_effective_p,
       Scomp_native_compiler_options_effective_p,
       0, 0, 0,
       doc: "Return t if `comp-native-compiler-options' is effective.",
       (),
       {
  #[cfg (feature = "libgccjit_have_gcc_jit_context_add_command_line_option")]
  {
    if ! fn_gcc_jit_context_add_command_line_option.is_none ()
      { return Qt (); }
  }
  return Qnil ();
});

/* `<epaths.h>` has no mirror (it is a configure product, the `callproc.rs` 284 / `lread.rs` 74
   shape): C 4367/4369 read `PATH_REL_LOADSEARCH` and `PATH_DUMPLOADSEARCH` from it, and the two
   values below are the frozen build's own (`build/emacs-ref/src/epaths.h` 34 and 51, read
   together with the frozen `configure` that produced them).  */
/// `emacs_fclose` is `sysdep.c`'s non-`static` definition; C 4664 declares it where it is used
/// and the units that call it carry the declaration themselves (`charset.rs` 117).
#[cfg (feature = "have_native_comp")]
unsafe extern "C" { pub fn emacs_fclose (stream: *mut FILE) -> c_int; }

#[cfg (feature = "have_native_comp")]
const PATH_REL_LOADSEARCH: *const c_char = c"32.0.50/lisp".as_ptr ();
/* C 4369's `PATH_DUMPLOADSEARCH` and its `"/"`-joined form are the `include/epaths.rs` mirror's;
   C's adjacent-literal concatenation is the mirror's `PATH_DUMPLOADSEARCH_SLASH ()` (per
   target, `epaths.in` 85-99).  */

/* In use by Fcomp_el_to_eln_filename.  */
/// C 4291: `static Lisp_Object loadsearch_re_list;` - C's tentative definition (zeroed, i.e.
/// `nil`, PORTING.md 0.1-13).
#[cfg (feature = "have_native_comp")]
static mut loadsearch_re_list: Lisp_Object = lisp_h_Qnil ();

/// C 4293-4298: `static Lisp_Object make_directory_wrapper (Lisp_Object directory)`.
#[cfg (feature = "have_native_comp")]
unsafe extern "C" fn make_directory_wrapper (directory: Lisp_Object) -> Lisp_Object
{
  CALLNI! ("make-directory", directory, Qt ());
  return Qnil ();
}

/// C 4300-4304: `static Lisp_Object make_directory_wrapper_1 (Lisp_Object ignore)`.
#[cfg (feature = "have_native_comp")]
unsafe extern "C" fn make_directory_wrapper_1 (ignore: Lisp_Object) -> Lisp_Object
{
  return Qt ();
}

/// C 4306-4395: `DEFUN ("comp-el-to-eln-rel-filename", Fcomp_el_to_eln_rel_filename, …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp-el-to-eln-rel-filename", Fcomp_el_to_eln_rel_filename,
       Scomp_el_to_eln_rel_filename, 1, 1, 0,
       doc: "Return the relative name of the .eln file for FILENAME.\nFILENAME must exist, and if it's a symlink, the target must exist.\nIf FILENAME is compressed, it must have the \".gz\" extension,\nand Emacs must have been compiled with zlib; the file will be\nuncompressed on the fly to hash its contents.\nValue includes the original base name, followed by 2 hash values,\none for the file name and another for its contents, followed by .eln.",
       (filename: Lisp_Object),
       {
  CHECK_STRING (filename);

  /* Resolve possible symlinks in FILENAME, so that path_hash below
     always compares equal. (Bug#44701).  */
  let mut filename: Lisp_Object = Fexpand_file_name (filename, Qnil ());
  let file_normalized: *mut c_char =
    realpath (SSDATA (ENCODE_FILE (filename)), std::ptr::null_mut ());
  if ! file_normalized.is_null ()
    {
      filename = DECODE_FILE (make_unibyte_string (file_normalized,
						   strlen (file_normalized) as ptrdiff_t));
      xfree (file_normalized as *mut c_void);
    }

  if NILP (Ffile_exists_p (filename))
    { xsignal1 (Qfile_missing (), filename); }

  #[cfg (feature = "windowsnt")]
  {
    filename = Fw32_long_file_name (filename);
  }

  let content_hash: Lisp_Object = comp_hash_source_file (filename);

  if suffix_p (filename, c".gz".as_ptr ())
    { filename = Fsubstring (filename, Qnil (), make_fixnum (-3)); }

  /* We create eln filenames with an hash in order to look-up these
     starting from the source filename, IOW have a relation

     /absolute/path/filename.el + content ->
     eln-cache/filename-path_hash-content_hash.eln.

     'dlopen' can return the same handle if two shared with the same
     filename are loaded in two different times (even if the first was
     deleted!).  To prevent this scenario the source file content is
     included in the hashing algorithm.

     As at any point in time no more then one file can exist with the
     same filename, should be possible to clean up all
     filename-path_hash-* except the most recent one (or the new one
     being recompiled).

     As installing .eln files compiled during the build changes their
     absolute path we need an hashing mechanism that is not sensitive
     to that.  For this we replace if match PATH_DUMPLOADSEARCH or
     *PATH_REL_LOADSEARCH have been replaced with `'//'` before computing the hash.  */

  if NILP (loadsearch_re_list)
    {
      let sys_re: Lisp_Object =
	concat2 (build_string (c"\\`[[:ascii:]]+".as_ptr ()),
		 Fregexp_quote (build_string (PATH_REL_LOADSEARCH)));
      let mut dump_load_search: Lisp_Object =
	Fexpand_file_name (build_string (PATH_DUMPLOADSEARCH_SLASH ()),
			   Qnil ());
      #[cfg (feature = "windowsnt")]
      {
	dump_load_search = Fw32_long_file_name (dump_load_search);
      }
      loadsearch_re_list = list2 (sys_re, Fregexp_quote (dump_load_search));
    }

  let mut lds_re_tail: Lisp_Object = loadsearch_re_list;
  FOR_EACH_TAIL! (lds_re_tail, {
      let match_idx: Lisp_Object =
	Fstring_match (XCAR (lds_re_tail), filename, Qnil (), Qnil ());
      if BASE_EQ (match_idx, make_fixnum (0))
	{
	  filename =
	    Freplace_match (build_string (c"//".as_ptr ()), Qt (), Qt (), filename, Qnil ());
	  break;
	}
    });
  let separator: Lisp_Object = build_string (c"-".as_ptr ());
  let path_hash: Lisp_Object = comp_hash_string (filename);
  filename = concat2 (Ffile_name_nondirectory (Fsubstring (filename, Qnil (),
							   make_fixnum (-3))),
		      separator);
  let hash: Lisp_Object = concat3 (path_hash, separator, content_hash);
  return concat3 (filename, hash, build_string (NATIVE_ELISP_SUFFIX.as_ptr ()));
});

/// C 4397-4470: `DEFUN ("comp-el-to-eln-filename", Fcomp_el_to_eln_filename, …)`.
#[cfg (feature = "have_native_comp")]
DEFUN! ("comp-el-to-eln-filename", Fcomp_el_to_eln_filename,
       Scomp_el_to_eln_filename, 1, 2, 0,
       doc: "Return the absolute .eln file name for source FILENAME.\nThe resulting .eln file name is intended to be used for natively\ncompiling FILENAME.  FILENAME must exist and be readable, but other\nthan that, its leading directories are ignored when constructing\nthe name of the .eln file.\nIf BASE-DIR is non-nil, use it as the directory for the .eln file;\nnon-absolute BASE-DIR is interpreted as relative to `invocation-directory'.\nIf BASE-DIR is omitted or nil, look for the first writable directory\nin `native-comp-eln-load-path', and use as BASE-DIR its subdirectory\nwhose name is given by `comp-native-version-dir'.\nIf FILENAME specifies a preloaded file, the directory for the .eln\nfile is the \"preloaded/\" subdirectory of the directory determined\nas described above.  FILENAME is considered to be a preloaded file if\nthe value of `comp-file-preloaded-p' is non-nil, or if FILENAME\nappears in the value of the environment variable LISP_PRELOADED;\nthe latter is supposed to be used by the Emacs build procedure.",
       (filename: Lisp_Object, base_dir: Lisp_Object),
       {
  let source_filename: Lisp_Object = filename;
  let mut filename: Lisp_Object = Fcomp_el_to_eln_rel_filename (filename);
  let mut base_dir: Lisp_Object = base_dir;

  /* If base_dir was not specified search inside Vnative_comp_eln_load_path
     for the first directory where we have write access.  */
  if NILP (base_dir)
    {
      let mut eln_load_paths: Lisp_Object = Vnative_comp_eln_load_path;
      FOR_EACH_TAIL! (eln_load_paths, {
	  let dir: Lisp_Object = XCAR (eln_load_paths);
	  if ! NILP (Ffile_exists_p (dir))
	    {
	      if ! NILP (Ffile_writable_p (dir))
		{
		  base_dir = dir;
		  break;
		}
	    }
	  else
	    {
	      /* Try to create the directory and if succeeds use it.  */
	      if NILP (internal_condition_case_1 (Some (make_directory_wrapper),
						   dir, Qt (),
						   Some (make_directory_wrapper_1)))
		{
		  base_dir = dir;
		  break;
		}
	    }
	});
      if NILP (base_dir)
	{ error (c"Cannot find suitable directory for output in `native-comp-eln-load-path'.".as_ptr ()); }
    }

  if ! file_name_absolute_p (SSDATA (base_dir))
    { base_dir = Fexpand_file_name (base_dir, Vinvocation_directory); }

  /* In case the file being compiled is found in 'LISP_PRELOADED' or
     `comp-file-preloaded-p' is non-nil target for output the
     'preloaded' subfolder.  */
  let lisp_preloaded: Lisp_Object =
    Fgetenv_internal (build_string (c"LISP_PRELOADED".as_ptr ()), Qnil ());
  base_dir = Fexpand_file_name (Vcomp_native_version_dir, base_dir);
  if comp_file_preloaded_p
      || (! NILP (lisp_preloaded)
	  && ! NILP (Fmember (CALLNI! ("file-name-base", source_filename),
			     Fmapcar (intern_c_string (c"file-name-base".as_ptr ()),
				      CALLNI! ("split-string", lisp_preloaded)))))
    { base_dir = Fexpand_file_name (build_string (c"preloaded".as_ptr ()), base_dir); }

  return Fexpand_file_name (filename, base_dir);
});

/* ==============================================================================================
   C 824-5857: **slice 2**.  It starts here because C 824's `freloc_check_fill` is the file's first
   `FOR_EACH_TAIL` (the mirror's `FOR_EACH_TAIL!`, the `syntax` row of
   `docs/ports/excluded-macros.txt`) and the first `goto`; the slice is arriving in tranches and
   `docs/ports/comp.md` carries the ledger rows.
   ============================================================================================== */

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
  /// C: libgccjit.h 1601-1604: `void gcc_jit_block_end_with_void_return (gcc_jit_block *block,
  /// gcc_jit_location *loc);`
  pub fn gcc_jit_block_end_with_void_return (block: *mut gcc_jit_block,
                                             loc: *mut gcc_jit_location);
  /// C: libgccjit.h 727-732: `gcc_jit_struct *gcc_jit_context_new_struct_type
  /// (gcc_jit_context *ctxt, gcc_jit_location *loc, const char *name, int num_fields,
  /// gcc_jit_field **fields);`
  pub fn gcc_jit_context_new_struct_type (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                          name: *const c_char, num_fields: c_int,
                                          fields: *mut *mut gcc_jit_field) -> *mut gcc_jit_struct;
  /// C: libgccjit.h 905-910: `gcc_jit_lvalue *gcc_jit_context_new_global (gcc_jit_context *ctxt,
  /// gcc_jit_location *loc, enum gcc_jit_global_kind kind, gcc_jit_type *type,
  /// const char *name);`
  pub fn gcc_jit_context_new_global (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                     kind: gcc_jit_global_kind, type_: *mut gcc_jit_type,
                                     name: *const c_char) -> *mut gcc_jit_lvalue;
  /// C: libgccjit.h 1172-1174: `gcc_jit_rvalue *gcc_jit_context_new_string_literal
  /// (gcc_jit_context *ctxt, const char *value);`
  pub fn gcc_jit_context_new_string_literal (ctxt: *mut gcc_jit_context, value: *const c_char)
    -> *mut gcc_jit_rvalue;
  /// C: libgccjit.h 1092-1094: `gcc_jit_lvalue *gcc_jit_global_set_initializer
  /// (gcc_jit_lvalue *global, const void *blob, size_t num_bytes);`
  pub fn gcc_jit_global_set_initializer (global: *mut gcc_jit_lvalue, blob: *const c_void,
                                         num_bytes: size_t) -> *mut gcc_jit_lvalue;
  /// C: libgccjit.h 736-738: `gcc_jit_struct *gcc_jit_context_new_opaque_struct
  /// (gcc_jit_context *ctxt, gcc_jit_location *loc, const char *name);`
  pub fn gcc_jit_context_new_opaque_struct (ctxt: *mut gcc_jit_context,
                                            loc: *mut gcc_jit_location, name: *const c_char)
    -> *mut gcc_jit_struct;
  /// C: libgccjit.h 747-750: `void gcc_jit_struct_set_fields (gcc_jit_struct *struct_type,
  /// gcc_jit_location *loc, int num_fields, gcc_jit_field **fields);`
  pub fn gcc_jit_struct_set_fields (struct_type: *mut gcc_jit_struct,
                                    loc: *mut gcc_jit_location, num_fields: c_int,
                                    fields: *mut *mut gcc_jit_field);
  /// C: libgccjit.h 166-167: `gcc_jit_context *gcc_jit_context_acquire (void);`
  pub fn gcc_jit_context_acquire () -> *mut gcc_jit_context;
  /// C: libgccjit.h 171-172: `void gcc_jit_context_release (gcc_jit_context *ctxt);`
  pub fn gcc_jit_context_release (ctxt: *mut gcc_jit_context);
  /// C: libgccjit.h 628-630: `gcc_jit_type *gcc_jit_context_get_type (gcc_jit_context *ctxt,
  /// enum gcc_jit_types type_);`
  pub fn gcc_jit_context_get_type (ctxt: *mut gcc_jit_context, type_: gcc_jit_types)
    -> *mut gcc_jit_type;
  /// C: libgccjit.h 633-635: `gcc_jit_type *gcc_jit_context_get_int_type (gcc_jit_context *ctxt,
  /// int num_bytes, int is_signed);`
  pub fn gcc_jit_context_get_int_type (ctxt: *mut gcc_jit_context, num_bytes: c_int,
                                       is_signed: c_int) -> *mut gcc_jit_type;
  /// C: libgccjit.h 265-267: `void gcc_jit_context_set_str_option (gcc_jit_context *ctxt,
  /// enum gcc_jit_str_option opt, const char *value);`
  pub fn gcc_jit_context_set_str_option (ctxt: *mut gcc_jit_context, opt: gcc_jit_str_option,
                                         value: *const c_char);
  /// C: libgccjit.h 271-273: `void gcc_jit_context_set_int_option (gcc_jit_context *ctxt,
  /// enum gcc_jit_int_option opt, int value);`
  pub fn gcc_jit_context_set_int_option (ctxt: *mut gcc_jit_context, opt: gcc_jit_int_option,
                                         value: c_int);
  /// C: libgccjit.h 279-281: `void gcc_jit_context_set_bool_option (gcc_jit_context *ctxt,
  /// enum gcc_jit_bool_option opt, int value);`
  pub fn gcc_jit_context_set_bool_option (ctxt: *mut gcc_jit_context, opt: gcc_jit_bool_option,
                                          value: c_int);
  /// C: libgccjit.h 427-430: `void gcc_jit_context_compile_to_file (gcc_jit_context *ctxt,
  /// enum gcc_jit_output_kind output_kind, const char *output_path);`
  pub fn gcc_jit_context_compile_to_file (ctxt: *mut gcc_jit_context,
                                          output_kind: gcc_jit_output_kind,
                                          output_path: *const c_char);
  /// C: libgccjit.h 440-442: `void gcc_jit_context_dump_to_file (gcc_jit_context *ctxt,
  /// const char *path, int update_locations);`
  pub fn gcc_jit_context_dump_to_file (ctxt: *mut gcc_jit_context, path: *const c_char,
                                       update_locations: c_int);
  /// C: libgccjit.h 452-455: `void gcc_jit_context_set_logfile (gcc_jit_context *ctxt,
  /// FILE *logfile, int flags, int verbosity);`
  pub fn gcc_jit_context_set_logfile (ctxt: *mut gcc_jit_context, logfile: *mut FILE,
                                      flags: c_int, verbosity: c_int);
  /// C: libgccjit.h 1745-1746: `void gcc_jit_context_dump_reproducer_to_file
  /// (gcc_jit_context *ctxt, const char *path);`
  pub fn gcc_jit_context_dump_reproducer_to_file (ctxt: *mut gcc_jit_context,
                                                  path: *const c_char);
  /// C: libgccjit.h 763-767: `gcc_jit_type *gcc_jit_context_new_union_type
  /// (gcc_jit_context *ctxt, gcc_jit_location *loc, const char *name, int num_fields,
  /// gcc_jit_field **fields);`
  pub fn gcc_jit_context_new_union_type (ctxt: *mut gcc_jit_context, loc: *mut gcc_jit_location,
                                         name: *const c_char, num_fields: c_int,
                                         fields: *mut *mut gcc_jit_field) -> *mut gcc_jit_type;
}

/// C: libgccjit.h 1176-1198: `enum gcc_jit_unary_op { GCC_JIT_UNARY_OP_MINUS,
/// GCC_JIT_UNARY_OP_BITWISE_NEGATE, GCC_JIT_UNARY_OP_LOGICAL_NEGATE, GCC_JIT_UNARY_OP_ABS };` -
/// the port's alias for a C enum, plus the one enumerator this slice names.  Measured with clang:
/// `sizeof (enum gcc_jit_unary_op) == 4` and `__builtin_types_compatible_p (enum
/// gcc_jit_unary_op, unsigned int)` both hold, so the alias is `c_uint` (the reading is recorded
/// with the type readings in `docs/ports/comp.md`).
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_unary_op = c_uint;
/// C: libgccjit.h 1181: `GCC_JIT_UNARY_OP_MINUS,` - the first enumerator, i.e. 0.
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_UNARY_OP_MINUS: gcc_jit_unary_op = 0;
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

/// C: libgccjit.h 178-188: `enum gcc_jit_str_option { GCC_JIT_STR_OPTION_PROGNAME, … };` - the
/// alias (measured like the others) and the one enumerator this file names.
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_str_option = c_uint;
/// C: libgccjit.h 182: `GCC_JIT_STR_OPTION_PROGNAME,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_STR_OPTION_PROGNAME: gcc_jit_str_option = 0;

/// C: libgccjit.h 191-201: `enum gcc_jit_int_option { GCC_JIT_INT_OPTION_OPTIMIZATION_LEVEL };`
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_int_option = c_uint;
/// C: libgccjit.h 198: `GCC_JIT_INT_OPTION_OPTIMIZATION_LEVEL,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_INT_OPTION_OPTIMIZATION_LEVEL: gcc_jit_int_option = 0;

/// C: libgccjit.h 205-260: `enum gcc_jit_bool_option { GCC_JIT_BOOL_OPTION_DEBUGINFO, … };` - the
/// three enumerators C 4826-4840 names, at the positions they hold there (DEBUGINFO first,
/// DUMP_EVERYTHING fifth, KEEP_INTERMEDIATES sixth).
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_bool_option = c_uint;
/// C: libgccjit.h 214: `GCC_JIT_BOOL_OPTION_DEBUGINFO,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BOOL_OPTION_DEBUGINFO: gcc_jit_bool_option = 0;
/// C: libgccjit.h 243: `GCC_JIT_BOOL_OPTION_DUMP_EVERYTHING,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BOOL_OPTION_DUMP_EVERYTHING: gcc_jit_bool_option = 5;
/// C: libgccjit.h 254: `GCC_JIT_BOOL_OPTION_KEEP_INTERMEDIATES,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_BOOL_OPTION_KEEP_INTERMEDIATES: gcc_jit_bool_option = 6;

/// C: libgccjit.h 406-419: `enum gcc_jit_output_kind { GCC_JIT_OUTPUT_KIND_ASSEMBLER, … };`
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_output_kind = c_uint;
/// C: libgccjit.h 415: `GCC_JIT_OUTPUT_KIND_DYNAMIC_LIBRARY,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_OUTPUT_KIND_DYNAMIC_LIBRARY: gcc_jit_output_kind = 2;

/// C: libgccjit.h 553-620: `enum gcc_jit_types { GCC_JIT_TYPE_VOID, … };` - the alias (measured
/// like the others) and the ten enumerators this slice names, in C's order from 0.
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_types = c_uint;
/// C: libgccjit.h 556: `GCC_JIT_TYPE_VOID,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_VOID: gcc_jit_types = 0;
/// C: libgccjit.h 559: `GCC_JIT_TYPE_VOID_PTR,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_VOID_PTR: gcc_jit_types = 1;
/// C: libgccjit.h 563: `GCC_JIT_TYPE_BOOL,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_BOOL: gcc_jit_types = 2;
/// C: libgccjit.h 569: `GCC_JIT_TYPE_CHAR,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_CHAR: gcc_jit_types = 3;
/// C: libgccjit.h 578: `GCC_JIT_TYPE_INT,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_INT: gcc_jit_types = 8;
/// C: libgccjit.h 579: `GCC_JIT_TYPE_UNSIGNED_INT,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_UNSIGNED_INT: gcc_jit_types = 9;
/// C: libgccjit.h 582: `GCC_JIT_TYPE_LONG,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_LONG: gcc_jit_types = 10;
/// C: libgccjit.h 583: `GCC_JIT_TYPE_UNSIGNED_LONG,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_UNSIGNED_LONG: gcc_jit_types = 11;
/// C: libgccjit.h 586: `GCC_JIT_TYPE_LONG_LONG,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_LONG_LONG: gcc_jit_types = 12;
/// C: libgccjit.h 587: `GCC_JIT_TYPE_UNSIGNED_LONG_LONG,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_TYPE_UNSIGNED_LONG_LONG: gcc_jit_types = 13;

/// C: libgccjit.h 888-901: `enum gcc_jit_global_kind { GCC_JIT_GLOBAL_EXPORTED, … };` - the alias
/// (measured like the others) and the two enumerators this slice names.
#[cfg (feature = "have_native_comp")]
pub type gcc_jit_global_kind = c_uint;
/// C: libgccjit.h 893: `GCC_JIT_GLOBAL_EXPORTED,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_GLOBAL_EXPORTED: gcc_jit_global_kind = 0;
/// C: libgccjit.h 897: `GCC_JIT_GLOBAL_INTERNAL,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_GLOBAL_INTERNAL: gcc_jit_global_kind = 1;

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
/// C: libgccjit.h 1291: `GCC_JIT_COMPARISON_NE,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_COMPARISON_NE: gcc_jit_comparison = 1;
/// C: libgccjit.h 1297: `GCC_JIT_COMPARISON_LE,`
#[cfg (feature = "have_native_comp")]
pub const GCC_JIT_COMPARISON_LE: gcc_jit_comparison = 3;

/// Names the reference's own headers declare and no mirror carries: `list` and `prog_ignore` are
/// `lisp.h`'s, `realpath` and `allocate_native_comp_unit` are `<stdlib.h>`/`comp.c`'s, and
/// `internal_condition_case_1`/`file_name_absolute_p` are declared where they are used elsewhere
/// in the tree (`eval.rs` 1687, `fileio.rs` 2161).  Each is the P-55 use-site shape.
#[cfg (feature = "have_native_comp")]
unsafe extern "C" {
  /// C: lisp.h's `extern Lisp_Object list (Lisp_Object, ...)` - variadic from the first object,
  /// not from a count.  `check-mirror-dupes.py` reports this as a duplicate of a mirror
  /// declaration; the ratchet row in `docs/baseline/mirror-dupes.tsv` records why it stays: with
  /// `have_native_comp` flipped on (the `dead-arms` configuration) that mirror declaration is not
  /// in scope, and the unit does not compile without its own copy (status 4.733).
  pub fn list (arg: Lisp_Object, ...) -> Lisp_Object;

  pub fn realpath (file_name: *const c_char, resolved_name: *mut c_char) -> *mut c_char;
  pub fn allocate_native_comp_unit () -> *mut Lisp_Native_Comp_Unit;
  pub fn internal_condition_case_1 (
    bfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>, arg: Lisp_Object,
    handlers: Lisp_Object,
    hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>) -> Lisp_Object;
  pub fn file_name_absolute_p (filename: *const c_char) -> bool;
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);` - a real (non-inline) definition the
  /// mirror does not carry.
  pub fn FUNCTIONP (a: Lisp_Object) -> bool;
}

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
  let mvar_slot: Lisp_Object = CALLNI! ("comp-mvar-slot", mvar);

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

/* Inline function definition and lisp data structure follows.  (C 3071-3073's banner.)  */

/* struct Lisp_Cons definition.  */

/// C 3077-3184: `static void define_lisp_cons (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_lisp_cons ()
{
  /*
    union cdr_u
    {
      Lisp_Object cdr;
      struct Lisp_Cons *chain;
    };

    struct cons_s
    {
      Lisp_Object car;
      union cdr_u u;
    };

    union cons_u
    {
      struct cons_s s;
      char align_pad[sizeof (struct Lisp_Cons)];
    };

    struct Lisp_Cons
    {
      union cons_u u;
    };
  */

  comp.lisp_cons_s =
    gcc_jit_context_new_opaque_struct (comp.ctxt,
				       std::ptr::null_mut (),
				       c"comp_Lisp_Cons".as_ptr ());
  comp.lisp_cons_type =
    gcc_jit_struct_as_type (comp.lisp_cons_s);
  comp.lisp_cons_ptr_type =
    gcc_jit_type_get_pointer (comp.lisp_cons_type);

  comp.lisp_cons_u_s_u_cdr =
    gcc_jit_context_new_field (comp.ctxt,
			       std::ptr::null_mut (),
			       comp.lisp_obj_type,
			       c"cdr".as_ptr ());

  let cdr_u_fields: [*mut gcc_jit_field; 2] =
    [ comp.lisp_cons_u_s_u_cdr,
      gcc_jit_context_new_field (comp.ctxt,
				 std::ptr::null_mut (),
				 comp.lisp_cons_ptr_type,
				 c"chain".as_ptr ()) ];

  let cdr_u: *mut gcc_jit_type =
    gcc_jit_context_new_union_type (comp.ctxt,
				    std::ptr::null_mut (),
				    c"comp_cdr_u".as_ptr (),
				    countof! (cdr_u_fields) as c_int,
				    cdr_u_fields.as_ptr () as *mut *mut gcc_jit_field);

  comp.lisp_cons_u_s_car = gcc_jit_context_new_field (comp.ctxt,
					    std::ptr::null_mut (),
					    comp.lisp_obj_type,
					    c"car".as_ptr ());
  comp.lisp_cons_u_s_u = gcc_jit_context_new_field (comp.ctxt,
						    std::ptr::null_mut (),
						    cdr_u,
						    c"u".as_ptr ());
  let cons_s_fields: [*mut gcc_jit_field; 2] =
    [ comp.lisp_cons_u_s_car,
      comp.lisp_cons_u_s_u ];

  let cons_s: *mut gcc_jit_struct =
    gcc_jit_context_new_struct_type (comp.ctxt,
				     std::ptr::null_mut (),
				     c"comp_cons_s".as_ptr (),
				     countof! (cons_s_fields) as c_int,
				     cons_s_fields.as_ptr () as *mut *mut gcc_jit_field);

  comp.lisp_cons_u_s = gcc_jit_context_new_field (comp.ctxt,
				 std::ptr::null_mut (),
				 gcc_jit_struct_as_type (cons_s),
				 c"s".as_ptr ());

  let cons_u_fields: [*mut gcc_jit_field; 2] =
    [ comp.lisp_cons_u_s,
      gcc_jit_context_new_field (
	comp.ctxt,
	std::ptr::null_mut (),
	gcc_jit_context_new_array_type (comp.ctxt,
					std::ptr::null_mut (),
					comp.char_type,
					core::mem::size_of::<Lisp_Cons> () as c_int),
	c"align_pad".as_ptr ()) ];

  let lisp_cons_u_type: *mut gcc_jit_type =
    gcc_jit_context_new_union_type (comp.ctxt,
				    std::ptr::null_mut (),
				    c"comp_cons_u".as_ptr (),
				    countof! (cons_u_fields) as c_int,
				    cons_u_fields.as_ptr () as *mut *mut gcc_jit_field);

  comp.lisp_cons_u =
    gcc_jit_context_new_field (comp.ctxt,
			       std::ptr::null_mut (),
			       lisp_cons_u_type,
			       c"u".as_ptr ());
  gcc_jit_struct_set_fields (comp.lisp_cons_s,
			     std::ptr::null_mut (), 1, &raw mut comp.lisp_cons_u);

}

/// C 3186-3217: `static void define_lisp_symbol_with_position (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_lisp_symbol_with_position ()
{
  comp.lisp_symbol_with_position_header =
    gcc_jit_context_new_field (comp.ctxt,
			       std::ptr::null_mut (),
			       comp.ptrdiff_type,
			       c"header".as_ptr ());
  comp.lisp_symbol_with_position_sym =
    gcc_jit_context_new_field (comp.ctxt,
			       std::ptr::null_mut (),
			       comp.lisp_obj_type,
			       c"sym".as_ptr ());
  comp.lisp_symbol_with_position_pos =
    gcc_jit_context_new_field (comp.ctxt,
			       std::ptr::null_mut (),
			       comp.lisp_obj_type,
			       c"pos".as_ptr ());
  let fields: [*mut gcc_jit_field; 3] = [comp.lisp_symbol_with_position_header,
			       comp.lisp_symbol_with_position_sym,
			       comp.lisp_symbol_with_position_pos];
  comp.lisp_symbol_with_position =
    gcc_jit_context_new_struct_type (comp.ctxt,
				     std::ptr::null_mut (),
				     c"comp_lisp_symbol_with_position".as_ptr (),
				     3,
				     fields.as_ptr () as *mut *mut gcc_jit_field);
  comp.lisp_symbol_with_position_type =
    gcc_jit_struct_as_type (comp.lisp_symbol_with_position);
  comp.lisp_symbol_with_position_ptr_type =
    gcc_jit_type_get_pointer (comp.lisp_symbol_with_position_type);
}

/* Opaque jmp_buf definition.  */

/// C 3221-3238: `static void define_jmp_buf (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_jmp_buf ()
{
  let field: *mut gcc_jit_field =
    gcc_jit_context_new_field (
      comp.ctxt,
      std::ptr::null_mut (),
      gcc_jit_context_new_array_type (comp.ctxt,
				      std::ptr::null_mut (),
				      comp.char_type,
				      core::mem::size_of::<sys_jmp_buf> () as c_int),
      c"stuff".as_ptr ());
  comp.jmp_buf_s =
    gcc_jit_context_new_struct_type (comp.ctxt,
				     std::ptr::null_mut (),
				     c"comp_jmp_buf".as_ptr (),
				     1, &raw const field as *mut *mut gcc_jit_field);
}

/// C 3240-3253: `static void define_memcpy (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_memcpy ()
{

  let params: [*mut gcc_jit_param; 3] =
    [ gcc_jit_context_new_param (comp.ctxt, std::ptr::null_mut (), comp.void_ptr_type,
				 c"dest".as_ptr ()),
      gcc_jit_context_new_param (comp.ctxt, std::ptr::null_mut (), comp.void_ptr_type,
				 c"src".as_ptr ()),
      gcc_jit_context_new_param (comp.ctxt, std::ptr::null_mut (), comp.size_t_type,
				 c"n".as_ptr ()) ];

  comp.memcpy =
    gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (), GCC_JIT_FUNCTION_IMPORTED,
				  comp.void_ptr_type, c"memcpy".as_ptr (),
				  countof! (params) as c_int,
				  params.as_ptr () as *mut *mut gcc_jit_param, 0);
}

/* struct handler definition  */

/// C 3257-3315: `static void define_handler_struct (void)`.  C's four `offsetof`/`sizeof`
/// expressions against `struct handler` are the mirror's record and `core::mem`'s operators.
#[cfg(feature = "have_native_comp")]
unsafe fn define_handler_struct ()
{
  comp.handler_s =
    gcc_jit_context_new_opaque_struct (comp.ctxt, std::ptr::null_mut (),
				       c"comp_handler".as_ptr ());
  comp.handler_ptr_type =
    gcc_jit_type_get_pointer (gcc_jit_struct_as_type (comp.handler_s));

  comp.handler_jmp_field = gcc_jit_context_new_field (comp.ctxt,
						      std::ptr::null_mut (),
						      gcc_jit_struct_as_type (
							comp.jmp_buf_s),
						      c"jmp".as_ptr ());
  comp.handler_val_field = gcc_jit_context_new_field (comp.ctxt,
						      std::ptr::null_mut (),
						      comp.lisp_obj_type,
						      c"val".as_ptr ());
  comp.handler_next_field = gcc_jit_context_new_field (comp.ctxt,
						       std::ptr::null_mut (),
						       comp.handler_ptr_type,
						       c"next".as_ptr ());
  let fields: [*mut gcc_jit_field; 6] =
    [ gcc_jit_context_new_field (
	comp.ctxt,
	std::ptr::null_mut (),
	gcc_jit_context_new_array_type (comp.ctxt,
					std::ptr::null_mut (),
					comp.char_type,
					core::mem::offset_of! (handler, val) as c_int),
	c"pad0".as_ptr ()),
      comp.handler_val_field,
      comp.handler_next_field,
      gcc_jit_context_new_field (
	comp.ctxt,
	std::ptr::null_mut (),
	gcc_jit_context_new_array_type (comp.ctxt,
					std::ptr::null_mut (),
					comp.char_type,
					(core::mem::offset_of! (handler, jmp)
					 - core::mem::offset_of! (handler, next)
					 - core::mem::size_of::<*mut handler> ()) as c_int),
	c"pad1".as_ptr ()),
      comp.handler_jmp_field,
      gcc_jit_context_new_field (
	comp.ctxt,
	std::ptr::null_mut (),
	gcc_jit_context_new_array_type (comp.ctxt,
					std::ptr::null_mut (),
					comp.char_type,
					(core::mem::size_of::<handler> ()
					 - core::mem::offset_of! (handler, jmp)
					 - core::mem::size_of::<sys_jmp_buf> ()) as c_int),
	c"pad2".as_ptr ()) ];
  gcc_jit_struct_set_fields (comp.handler_s,
			     std::ptr::null_mut (),
			     countof! (fields) as c_int,
			     fields.as_ptr () as *mut *mut gcc_jit_field);

}

/// C 3508-3558: `static void define_CHECK_TYPE (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_CHECK_TYPE ()
{
  let param: [*mut gcc_jit_param; 3] =
    [ gcc_jit_context_new_param (comp.ctxt,
				 std::ptr::null_mut (),
				 comp.int_type,
				 c"ok".as_ptr ()),
      gcc_jit_context_new_param (comp.ctxt,
				 std::ptr::null_mut (),
				 comp.lisp_obj_type,
				 c"predicate".as_ptr ()),
      gcc_jit_context_new_param (comp.ctxt,
				 std::ptr::null_mut (),
				 comp.lisp_obj_type,
				 c"x".as_ptr ()) ];
  comp.check_type =
    gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				  GCC_JIT_FUNCTION_INTERNAL,
				  comp.void_type,
				  c"CHECK_TYPE".as_ptr (),
				  3,
				  param.as_ptr () as *mut *mut gcc_jit_param,
				  0);
  let ok: *mut gcc_jit_rvalue = gcc_jit_param_as_rvalue (param[0]);
  let predicate: *mut gcc_jit_rvalue = gcc_jit_param_as_rvalue (param[1]);
  let x: *mut gcc_jit_rvalue = gcc_jit_param_as_rvalue (param[2]);

  DECL_BLOCK! (entry_block, comp.check_type);
  DECL_BLOCK! (ok_block, comp.check_type);
  DECL_BLOCK! (not_ok_block, comp.check_type);

  comp.block = entry_block;
  comp.func = comp.check_type;

  emit_cond_jump (ok, ok_block, not_ok_block);

  gcc_jit_block_end_with_void_return (ok_block, std::ptr::null_mut ());

  comp.block = not_ok_block;

  let wrong_type_args: [*mut gcc_jit_rvalue; 2] = [ predicate, x ];

  gcc_jit_block_add_eval (comp.block,
			  std::ptr::null_mut (),
			  emit_call (intern_c_string (c"wrong_type_argument".as_ptr ()),
				     comp.void_type, 2,
				     wrong_type_args.as_ptr () as *mut *mut gcc_jit_rvalue,
				     false));

  gcc_jit_block_end_with_void_return (not_ok_block, std::ptr::null_mut ());
}

/* Define a substitute for CAR as always inlined function.  */

/// C 3562-3635: `static void define_CAR_CDR (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_CAR_CDR ()
{
  let mut func: [*mut gcc_jit_function; 2] = [std::ptr::null_mut (); 2];
  let f_name: [*const c_char; 2] = [ c"CAR".as_ptr (), c"CDR".as_ptr () ];
  let mut i: c_int = 0;
  while i < 2
    {
      let param: [*mut gcc_jit_param; 2] =
	[ gcc_jit_context_new_param (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.lisp_obj_type,
				     c"c".as_ptr ()),
	  gcc_jit_context_new_param (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.bool_type,
				     c"cert_cons".as_ptr ()) ];
      /* TODO: understand why after ipa-prop pass gcc is less keen on inlining
	 and as consequence can refuse to compile these. (see dhrystone.el)
	 Flag this and all the one involved in ipa-prop as
	 GCC_JIT_FUNCTION_INTERNAL not to fail compilation in case.
	 This seems at least to have no perf downside.  */
      func[i as usize] =
	gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				      GCC_JIT_FUNCTION_INTERNAL,
				      comp.lisp_obj_type,
				      f_name[i as usize],
				      2, param.as_ptr () as *mut *mut gcc_jit_param, 0);

      let c: *mut gcc_jit_rvalue = gcc_jit_param_as_rvalue (param[0]);
      DECL_BLOCK! (entry_block, func[i as usize]);
      DECL_BLOCK! (is_cons_b, func[i as usize]);
      DECL_BLOCK! (not_a_cons_b, func[i as usize]);
      comp.block = entry_block;
      comp.func = func[i as usize];
      emit_cond_jump (emit_binary_op (GCC_JIT_BINARY_OP_LOGICAL_OR,
				      comp.bool_type,
				      gcc_jit_param_as_rvalue (param[1]),
				      emit_CONSP (c)),
		      is_cons_b,
		      not_a_cons_b);
      comp.block = is_cons_b;
      if i == 0
	{ gcc_jit_block_end_with_return (comp.block, std::ptr::null_mut (), emit_XCAR (c)); }
      else
	{ gcc_jit_block_end_with_return (comp.block, std::ptr::null_mut (), emit_XCDR (c)); }

      comp.block = not_a_cons_b;

      DECL_BLOCK! (is_nil_b, func[i as usize]);
      DECL_BLOCK! (not_nil_b, func[i as usize]);

      emit_cond_jump (emit_NILP (c), is_nil_b, not_nil_b);

      comp.block = is_nil_b;
      gcc_jit_block_end_with_return (comp.block,
				     std::ptr::null_mut (),
				     emit_lisp_obj_rval (Qnil ()));

      comp.block = not_nil_b;
      let wrong_type_args: [*mut gcc_jit_rvalue; 2] =
	[ emit_lisp_obj_rval (Qlistp ()), c ];

      gcc_jit_block_add_eval (comp.block,
			      std::ptr::null_mut (),
			      emit_call (intern_c_string (c"wrong_type_argument".as_ptr ()),
					 comp.void_type, 2,
					 wrong_type_args.as_ptr () as *mut *mut gcc_jit_rvalue,
					 false));
      gcc_jit_block_end_with_return (comp.block,
				     std::ptr::null_mut (),
				     emit_lisp_obj_rval (Qnil ()));
      i += 1;
    }
  comp.car = func[0];
  comp.cdr = func[1];
}

/// C 3637-3690: `static void define_setcar_setcdr (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_setcar_setcdr ()
{
  let f_name: [*const c_char; 2] = [ c"setcar".as_ptr (), c"setcdr".as_ptr () ];
  let par_name: [*const c_char; 2] = [ c"new_car".as_ptr (), c"new_cdr".as_ptr () ];

  let mut i: c_int = 0;
  while i < 2
    {
      let cell: *mut gcc_jit_param =
	gcc_jit_context_new_param (comp.ctxt,
				   std::ptr::null_mut (),
				   comp.lisp_obj_type,
				   c"cell".as_ptr ());
      let new_el: *mut gcc_jit_param =
	gcc_jit_context_new_param (comp.ctxt,
				   std::ptr::null_mut (),
				   comp.lisp_obj_type,
				   par_name[i as usize]);

      let param: [*mut gcc_jit_param; 3] =
	[ cell,
	  new_el,
	  gcc_jit_context_new_param (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.bool_type,
				     c"cert_cons".as_ptr ()) ];

      /* C's `gcc_jit_function **f_ref = !i ? &comp.setcar : &comp.setcdr;` - the address of the
	 field the loop writes; the port writes the field through the same two names.  */
      let f: *mut gcc_jit_function = gcc_jit_context_new_function (comp.ctxt,
					     std::ptr::null_mut (),
					     GCC_JIT_FUNCTION_INTERNAL,
					     comp.lisp_obj_type,
					     f_name[i as usize],
					     3, param.as_ptr () as *mut *mut gcc_jit_param, 0);
      if i == 0 { comp.setcar = f; } else { comp.setcdr = f; }
      DECL_BLOCK! (entry_block, f);
      comp.func = f;
      comp.block = entry_block;

      /* CHECK_CONS (cell);  */
      emit_CHECK_CONS (gcc_jit_param_as_rvalue (cell));

      /* XSETCDR (cell, newel);  */
      if i == 0
	{ emit_XSETCAR (gcc_jit_param_as_rvalue (cell),
		      gcc_jit_param_as_rvalue (new_el)); }
      else
	{ emit_XSETCDR (gcc_jit_param_as_rvalue (cell),
		      gcc_jit_param_as_rvalue (new_el)); }

      /* return newel;  */
      gcc_jit_block_end_with_return (entry_block,
				     std::ptr::null_mut (),
				     gcc_jit_param_as_rvalue (new_el));
      i += 1;
    }
}

/*
   Define a substitute for Fadd1 Fsub1.
   Currently expose just fixnum arithmetic.
*/

/* Define a function to convert boolean into t or nil */

/// C 4072-4107: `static void define_bool_to_lisp_obj (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_bool_to_lisp_obj ()
{
  /* x ? Qt : Qnil */
  let param: *mut gcc_jit_param = gcc_jit_context_new_param (comp.ctxt,
						    std::ptr::null_mut (),
						    comp.bool_type,
						    c"x".as_ptr ());
  comp.bool_to_lisp_obj =
    gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				  GCC_JIT_FUNCTION_INTERNAL,
				  comp.lisp_obj_type,
				  c"bool_to_lisp_obj".as_ptr (),
				  1,
				  &raw const param as *mut *mut gcc_jit_param,
				  0);
  DECL_BLOCK! (entry_block, comp.bool_to_lisp_obj);
  DECL_BLOCK! (ret_t_block, comp.bool_to_lisp_obj);
  DECL_BLOCK! (ret_nil_block, comp.bool_to_lisp_obj);
  comp.block = entry_block;
  comp.func = comp.bool_to_lisp_obj;

  emit_cond_jump (gcc_jit_param_as_rvalue (param),
		  ret_t_block,
		  ret_nil_block);

  comp.block = ret_t_block;
  gcc_jit_block_end_with_return (ret_t_block,
				 std::ptr::null_mut (),
				 emit_lisp_obj_rval (Qt ()));

  comp.block = ret_nil_block;
  gcc_jit_block_end_with_return (ret_nil_block,
				 std::ptr::null_mut (),
				 emit_lisp_obj_rval (Qnil ()));
}

/// C 4109-4163: `static gcc_jit_function *declare_lex_function (Lisp_Object func)`.
#[cfg (feature = "have_native_comp")]
unsafe fn declare_lex_function (func: Lisp_Object) -> *mut gcc_jit_function
{
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let c_name: Lisp_Object = CALLNI! ("comp-func-c-name", func);
  let args: Lisp_Object = CALLNI! ("comp-func-l-args", func);
  let nargs: bool = ! NILP (CALLNI! ("comp-nargs-p", args));

  let res: *mut gcc_jit_function;
  if ! nargs
    {
      let max_args: EMACS_INT = XFIXNUM (CALLNI! ("comp-args-max", args));
      eassert! (max_args < INT_MAX as EMACS_INT);
      let mut type_: *mut *mut gcc_jit_type = std::ptr::null_mut ();
      SAFE_NALLOCA! (sa_avail, type_, 1, max_args as usize);
      let mut i: ptrdiff_t = 0;
      while i < max_args
	{
	  *type_.offset (i as isize) = comp.lisp_obj_type;
	  i += 1;
	}

      let mut params: *mut *mut gcc_jit_param = std::ptr::null_mut ();
      SAFE_NALLOCA! (sa_avail, params, 1, max_args as usize);
      let mut i: c_int = 0;
      while (i as EMACS_INT) < max_args
	{
	  *params.offset (i as isize) = gcc_jit_context_new_param (comp.ctxt,
					      std::ptr::null_mut (),
					      *type_.offset (i as isize),
					      format_string (c"par_%d".as_ptr (), i));
	  i += 1;
	}
      res = gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
					  GCC_JIT_FUNCTION_EXPORTED,
					  comp.lisp_obj_type,
					  SSDATA (c_name),
					  max_args as c_int,
					  params,
					  0);
    }
  else
    {
      let params: [*mut gcc_jit_param; 2] =
	[ gcc_jit_context_new_param (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.ptrdiff_type,
				     c"nargs".as_ptr ()),
	  gcc_jit_context_new_param (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.lisp_obj_ptr_type,
				     c"args".as_ptr ()) ];
      res = gcc_jit_context_new_function (comp.ctxt,
				      std::ptr::null_mut (),
				      GCC_JIT_FUNCTION_EXPORTED,
				      comp.lisp_obj_type,
				      SSDATA (c_name),
				      countof! (params) as c_int,
				      params.as_ptr () as *mut *mut gcc_jit_param, 0);
    }
  SAFE_FREE! (sa_count, sa_mark);
  return res;
}

/* Declare a function being compiled and add it to comp.exported_funcs_h.  */

/// C 4167-4182: `static void declare_function (Lisp_Object func)`.
#[cfg (feature = "have_native_comp")]
unsafe fn declare_function (func: Lisp_Object)
{
  let gcc_func: *mut gcc_jit_function =
    if ! NILP (CALLNI! ("comp-func-l-p", func))
    { declare_lex_function (func) }
    else
    { gcc_jit_context_new_function (comp.ctxt,
				    std::ptr::null_mut (),
				    GCC_JIT_FUNCTION_EXPORTED,
				    comp.lisp_obj_type,
				    SSDATA (CALLNI! ("comp-func-c-name", func)),
				    0, std::ptr::null_mut (), 0) };
  Fputhash (CALLNI! ("comp-func-c-name", func),
	    make_mint_ptr (gcc_func as *mut c_void),
	    comp.exported_funcs_h);
}

/// C 4184-4283: `static void compile_function (Lisp_Object func)`.
#[cfg (feature = "have_native_comp")]
unsafe fn compile_function (func: Lisp_Object)
{
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  comp.frame_size = XFIXNUM (CALLNI! ("comp-func-frame-size", func));
  eassert! (comp.frame_size < INT_MAX as EMACS_INT);

  comp.func = xmint_pointer (Fgethash (CALLNI! ("comp-func-c-name", func),
				       comp.exported_funcs_h, Qnil ()))
    as *mut gcc_jit_function;

  comp.func_has_non_local = ! NILP (CALLNI! ("comp-func-has-non-local", func));
  comp.func_speed = XFIXNUM (CALLNI! ("comp-func-speed", func));
  comp.func_safety = XFIXNUM (CALLNI! ("comp-func-safety", func));

  comp.func_relocs_local =
    gcc_jit_function_new_local (comp.func,
				std::ptr::null_mut (),
				comp.func_relocs_ptr_type,
				c"freloc".as_ptr ());

  /* C's `SAFE_NALLOCA (comp.frame, 1, comp.frame_size)` names a struct member; the mirror's
     macro takes an identifier (it assigns the buffer), so the member is carried in and back.  */
  let mut frame_buf: *mut *mut gcc_jit_lvalue = comp.frame;
  SAFE_NALLOCA! (sa_avail, frame_buf, 1, comp.frame_size as usize);
  comp.frame = frame_buf;
  if comp.func_has_non_local || comp.func_speed == 0
    {
      /* FIXME: See bug#42360.  */
      let arr: *mut gcc_jit_lvalue =
        gcc_jit_function_new_local (
          comp.func,
          std::ptr::null_mut (),
          gcc_jit_context_new_array_type (comp.ctxt,
                                          std::ptr::null_mut (),
                                          comp.lisp_obj_type,
                                          comp.frame_size as c_int),
          c"frame".as_ptr ());

      let mut i: ptrdiff_t = 0;
      while i < comp.frame_size
	{
	  *comp.frame.offset (i as isize) =
          gcc_jit_context_new_array_access (
            comp.ctxt,
            std::ptr::null_mut (),
            gcc_jit_lvalue_as_rvalue (arr),
            gcc_jit_context_new_rvalue_from_int (comp.ctxt,
                                                 comp.int_type,
                                                 i as c_int));
	  i += 1;
	}
    }
  else
    {
      let mut i: ptrdiff_t = 0;
      while i < comp.frame_size
	{
	  *comp.frame.offset (i as isize) =
	    gcc_jit_function_new_local (comp.func,
				    std::ptr::null_mut (),
				    comp.lisp_obj_type,
				    format_string (c"slot_%td".as_ptr (), i));
	  i += 1;
	}
    }

  comp.scratch = std::ptr::null_mut ();

  comp.loc_handler = gcc_jit_function_new_local (comp.func,
						  std::ptr::null_mut (),
						  comp.handler_ptr_type,
						  c"c".as_ptr ());

  comp.func_blocks_h = Fmake_hash_table (0, std::ptr::null_mut ());

  /* Pre-declare all basic blocks to gcc.
     The "entry" block must be declared as first.  */
  declare_block (Qentry ());
  let ht: *mut Lisp_Hash_Table = XHASH_TABLE (CALLNI! ("comp-func-blocks", func));
  DOHASH! (ht, block_name, block, {
      if ! EQ (block_name, Qentry ())
	{ declare_block (block_name); }
    });

  gcc_jit_block_add_assignment (retrieve_block (Qentry ()),
				std::ptr::null_mut (),
				comp.func_relocs_local,
				gcc_jit_lvalue_as_rvalue (comp.func_relocs));


  DOHASH! (ht, block_name, block, {
      let insns: Lisp_Object = CALLNI! ("comp-block-insns", block);
      if NILP (block) || NILP (insns)
	{ xsignal1 (Qnative_ice (),
		  build_string (c"basic block is missing or empty".as_ptr ())); }

      comp.block = retrieve_block (block_name);
      let mut insns: Lisp_Object = insns;
      while CONSP (insns)
	{
	  let insn: Lisp_Object = XCAR (insns);
	  emit_limple_insn (insn);
	  insns = XCDR (insns);
	}
    });
  let err: *const c_char = gcc_jit_context_get_first_error (comp.ctxt);
  if ! err.is_null ()
    { xsignal3 (Qnative_ice (),
	      build_string (c"failing to compile function".as_ptr ()),
	      CALLNI! ("comp-func-name", func),
	      build_string (err)); }
  SAFE_FREE! (sa_count, sa_mark);
}

/**********************************/
/* Entry points exposed to lisp.  */
/**********************************/

/// C 3778-3847: `static void define_negate (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_negate ()
{
  let bb_orig: *mut gcc_jit_block = comp.block;
  let param: [*mut gcc_jit_param; 2] =
	[ gcc_jit_context_new_param (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.lisp_obj_type,
				     c"n".as_ptr ()),
	  gcc_jit_context_new_param (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.bool_type,
				     c"cert_fixnum".as_ptr ()) ];

  let negate: *mut gcc_jit_function = gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				  GCC_JIT_FUNCTION_INTERNAL,
				  comp.lisp_obj_type,
				  c"negate".as_ptr (),
				  2, param.as_ptr () as *mut *mut gcc_jit_param, 0);
  comp.negate = negate;
  comp.func = negate;

  DECL_BLOCK! (entry_block, negate);
  DECL_BLOCK! (inline_block, negate);
  DECL_BLOCK! (fcall_block, negate);

  comp.block = entry_block;

  /* (cert_fixnum || FIXNUMP (TOP)) && XFIXNUM (TOP) != MOST_NEGATIVE_FIXNUM
     ? make_fixnum (- XFIXNUM (TOP)) : Fminus (1, &TOP))  */

  let n: *mut gcc_jit_lvalue = gcc_jit_param_as_lvalue (param[0]);
  let n_fixnum: *mut gcc_jit_rvalue = emit_XFIXNUM (gcc_jit_lvalue_as_rvalue (n));
  let sure_fixnum: *mut gcc_jit_rvalue =
    emit_binary_op (GCC_JIT_BINARY_OP_LOGICAL_OR,
		    comp.bool_type,
		    gcc_jit_param_as_rvalue (param[1]),
		    emit_FIXNUMP (gcc_jit_lvalue_as_rvalue (n)));

  emit_cond_jump (emit_binary_op (GCC_JIT_BINARY_OP_LOGICAL_AND,
				  comp.bool_type,
				  sure_fixnum,
				  gcc_jit_context_new_comparison (
				    comp.ctxt,
				    std::ptr::null_mut (),
				    GCC_JIT_COMPARISON_NE,
				    n_fixnum,
				    emit_rvalue_from_emacs_int (
                                      MOST_NEGATIVE_FIXNUM))),
		  inline_block,
		  fcall_block);

  comp.block = inline_block;
  let inline_res: *mut gcc_jit_rvalue =
    gcc_jit_context_new_unary_op (comp.ctxt,
				  std::ptr::null_mut (),
				  GCC_JIT_UNARY_OP_MINUS,
				  comp.emacs_int_type,
				  n_fixnum);

  gcc_jit_block_end_with_return (inline_block,
				 std::ptr::null_mut (),
				 emit_make_fixnum (inline_res));

  comp.block = fcall_block;
  let call_res: *mut gcc_jit_rvalue = emit_call_ref (Qminus (), 1, n, false);
  gcc_jit_block_end_with_return (fcall_block,
				 std::ptr::null_mut (),
				 call_res);
  comp.block = bb_orig;
}

/* Define a substitute for PSEUDOVECTORP as always inlined function.  */

/// C 3851-3902: `static void define_PSEUDOVECTORP (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_PSEUDOVECTORP ()
{
  let param: [*mut gcc_jit_param; 2] =
    [ gcc_jit_context_new_param (comp.ctxt,
				 std::ptr::null_mut (),
				 comp.lisp_obj_type,
				 c"a".as_ptr ()),
      gcc_jit_context_new_param (comp.ctxt,
				 std::ptr::null_mut (),
				 comp.int_type,
				 c"code".as_ptr ()) ];

  comp.pseudovectorp =
    gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				  GCC_JIT_FUNCTION_INTERNAL,
				  comp.bool_type,
				  c"PSEUDOVECTORP".as_ptr (),
				  2,
				  param.as_ptr () as *mut *mut gcc_jit_param,
				  0);

  DECL_BLOCK! (entry_block, comp.pseudovectorp);
  DECL_BLOCK! (ret_false_b, comp.pseudovectorp);
  DECL_BLOCK! (call_pseudovector_typep_b, comp.pseudovectorp);

  comp.block = entry_block;
  comp.func = comp.pseudovectorp;

  emit_cond_jump (emit_VECTORLIKEP (gcc_jit_param_as_rvalue (param[0])),
		  call_pseudovector_typep_b,
		  ret_false_b);

  comp.block = ret_false_b;
  gcc_jit_block_end_with_return (ret_false_b,
				 std::ptr::null_mut (),
				 gcc_jit_context_new_rvalue_from_int (
				   comp.ctxt,
				   comp.bool_type,
				   0));

  let args: [*mut gcc_jit_rvalue; 2] =
    [ gcc_jit_param_as_rvalue (param[0]),
      gcc_jit_param_as_rvalue (param[1]) ];
  comp.block = call_pseudovector_typep_b;
  /* FIXME use XUNTAG now that's available.  */
  gcc_jit_block_end_with_return (
    call_pseudovector_typep_b,
    std::ptr::null_mut (),
    emit_call (intern_c_string (c"helper_PSEUDOVECTOR_TYPEP_XUNTAG".as_ptr ()),
	       comp.bool_type, 2, args.as_ptr () as *mut *mut gcc_jit_rvalue, false));
}

/// C 3904-3936: `static void define_GET_SYMBOL_WITH_POSITION (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_GET_SYMBOL_WITH_POSITION ()
{
  let param: [*mut gcc_jit_param; 1] =
    [ gcc_jit_context_new_param (comp.ctxt,
				 std::ptr::null_mut (),
				 comp.lisp_obj_type,
				 c"a".as_ptr ()) ];

  comp.get_symbol_with_position =
    gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				  GCC_JIT_FUNCTION_INTERNAL,
				  comp.lisp_symbol_with_position_ptr_type,
				  c"GET_SYMBOL_WITH_POSITION".as_ptr (),
				  1,
				  param.as_ptr () as *mut *mut gcc_jit_param,
				  0);

  DECL_BLOCK! (entry_block, comp.get_symbol_with_position);

  comp.block = entry_block;
  comp.func = comp.get_symbol_with_position;

  let args: [*mut gcc_jit_rvalue; 1] =
    [ gcc_jit_param_as_rvalue (param[0]) ];
  /* FIXME use XUNTAG now that's available.  */
  gcc_jit_block_end_with_return (
    entry_block,
    std::ptr::null_mut (),
    emit_call (intern_c_string (c"helper_GET_SYMBOL_WITH_POSITION".as_ptr ()),
	       comp.lisp_symbol_with_position_ptr_type,
	       1, args.as_ptr () as *mut *mut gcc_jit_rvalue, false));
}

/// C 3938-3978: `static void define_SYMBOL_WITH_POS_SYM (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_SYMBOL_WITH_POS_SYM ()
{
  let param: [*mut gcc_jit_param; 1] =
    [ gcc_jit_context_new_param (comp.ctxt,
				 std::ptr::null_mut (),
				 comp.lisp_obj_type,
				 c"a".as_ptr ()) ];
  comp.symbol_with_pos_sym =
    gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				  GCC_JIT_FUNCTION_INTERNAL,
				  comp.lisp_obj_type,
				  c"SYMBOL_WITH_POS_SYM".as_ptr (),
				  1,
				  param.as_ptr () as *mut *mut gcc_jit_param,
				  0);

  DECL_BLOCK! (entry_block, comp.symbol_with_pos_sym);
  comp.func = comp.symbol_with_pos_sym;
  comp.block = entry_block;

  emit_CHECK_SYMBOL_WITH_POS (gcc_jit_param_as_rvalue (param [0]));

  let args: [*mut gcc_jit_rvalue; 1] = [ gcc_jit_param_as_rvalue (param [0]) ];

  let swp: *mut gcc_jit_rvalue = gcc_jit_context_new_call (comp.ctxt,
				  std::ptr::null_mut (),
				  comp.get_symbol_with_position,
				  1,
				  args.as_ptr () as *mut *mut gcc_jit_rvalue);
  let tmpl: *mut gcc_jit_lvalue = gcc_jit_rvalue_dereference (swp, std::ptr::null_mut ());
  let tmpr: *mut gcc_jit_rvalue = gcc_jit_lvalue_as_rvalue (tmpl);
  gcc_jit_block_end_with_return (entry_block,
				 std::ptr::null_mut (),
				 gcc_jit_rvalue_access_field (
				   tmpr,
				   std::ptr::null_mut (),
				   comp.lisp_symbol_with_position_sym));
}

/// C 3980-4068: `static void define_maybe_gc_or_quit (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_maybe_gc_or_quit ()
{

  /*
    void
    maybe_gc_or_quit (void)
    {
      static unsigned quitcounter;
     inc:
      quitcounter++;
      if (quitcounter >> 14) goto maybe_do_it else goto pass;
     maybe_do_it:
          quitcounter = 0;
          maybe_gc ();
          maybe_quit ();
          return;
     pass:
          return;
    }
  */

  let bb_orig: *mut gcc_jit_block = comp.block;

  let quitcounter: *mut gcc_jit_lvalue =
    gcc_jit_context_new_global (
      comp.ctxt,
      std::ptr::null_mut (),
      GCC_JIT_GLOBAL_INTERNAL,
      comp.unsigned_type,
      c"quitcounter".as_ptr ());

  let f: *mut gcc_jit_function = gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				  GCC_JIT_FUNCTION_INTERNAL,
				  comp.void_type,
				  c"maybe_gc_quit".as_ptr (),
				  0, std::ptr::null_mut (), 0);
  comp.maybe_gc_or_quit = f;
  comp.func = f;
  DECL_BLOCK! (increment_block, f);
  DECL_BLOCK! (maybe_do_it_block, f);
  DECL_BLOCK! (pass_block, f);

  comp.block = increment_block;

  gcc_jit_block_add_assignment (
    comp.block,
    std::ptr::null_mut (),
    quitcounter,
    emit_binary_op (GCC_JIT_BINARY_OP_PLUS,
		    comp.unsigned_type,
		    gcc_jit_lvalue_as_rvalue (quitcounter),
		    gcc_jit_context_new_rvalue_from_int (comp.ctxt,
							 comp.unsigned_type,
							 1)));
  emit_cond_jump (
    emit_binary_op (GCC_JIT_BINARY_OP_RSHIFT,
		    comp.unsigned_type,
		    gcc_jit_lvalue_as_rvalue (quitcounter),
		    gcc_jit_context_new_rvalue_from_int (comp.ctxt,
							 comp.unsigned_type,
							 9)),
    /* 9 translates into checking for GC or quit every 512 calls to
       'maybe_gc_quit'.  This is the smallest value I could find with
       no performance impact running elisp-banechmarks and the same
       used by the byte interpreter (see 'exec_byte_code').  */
    maybe_do_it_block,
    pass_block);

  comp.block = maybe_do_it_block;

  gcc_jit_block_add_assignment (
    comp.block,
    std::ptr::null_mut (),
    quitcounter,
    gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					 comp.unsigned_type,
					 0));
  gcc_jit_block_add_eval (comp.block, std::ptr::null_mut (),
			  emit_call (intern_c_string (c"maybe_gc".as_ptr ()),
				     comp.void_type, 0, std::ptr::null_mut (), false));
  gcc_jit_block_add_eval (comp.block, std::ptr::null_mut (),
			  emit_call (intern_c_string (c"maybe_quit".as_ptr ()),
				     comp.void_type, 0, std::ptr::null_mut (), false));
  gcc_jit_block_end_with_void_return (comp.block, std::ptr::null_mut ());

  gcc_jit_block_end_with_void_return (pass_block, std::ptr::null_mut ());

  comp.block = bb_orig;
}

/// C 3697-3776: `static void define_add1_sub1 (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn define_add1_sub1 ()
{
  let bb_orig: *mut gcc_jit_block = comp.block;
  let mut func: [*mut gcc_jit_function; 2] = [std::ptr::null_mut (); 2];
  let f_name: [*const c_char; 2] = [ c"add1".as_ptr (), c"sub1".as_ptr () ];
  let fall_back_func: [*const c_char; 2] = [ c"1+".as_ptr (), c"1-".as_ptr () ];
  let op: [gcc_jit_binary_op; 2] =
    [ GCC_JIT_BINARY_OP_PLUS, GCC_JIT_BINARY_OP_MINUS ];
  let mut i: ptrdiff_t = 0;
  while i < 2
    {
      let param: [*mut gcc_jit_param; 2] =
	[ gcc_jit_context_new_param (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.lisp_obj_type,
				     c"n".as_ptr ()),
	  gcc_jit_context_new_param (comp.ctxt,
				     std::ptr::null_mut (),
				     comp.bool_type,
				     c"cert_fixnum".as_ptr ()) ];
      func[i as usize] = gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				      GCC_JIT_FUNCTION_INTERNAL,
				      comp.lisp_obj_type,
				      f_name[i as usize],
				      2,
				      param.as_ptr () as *mut *mut gcc_jit_param, 0);
      comp.func = func[i as usize];
      DECL_BLOCK! (entry_block, func[i as usize]);
      DECL_BLOCK! (inline_block, func[i as usize]);
      DECL_BLOCK! (fcall_block, func[i as usize]);

      comp.block = entry_block;

      /* cert_fixnum ||
	 ((FIXNUMP (n) && XFIXNUM (n) != MOST_POSITIVE_FIXNUM
	 ? (XFIXNUM (n) + 1)
	 : Fadd1 (n)) */

      let n: *mut gcc_jit_rvalue = gcc_jit_param_as_rvalue (param[0]);
      let n_fixnum: *mut gcc_jit_rvalue = emit_XFIXNUM (n);
      let sure_fixnum: *mut gcc_jit_rvalue =
	emit_binary_op (GCC_JIT_BINARY_OP_LOGICAL_OR,
			comp.bool_type,
			gcc_jit_param_as_rvalue (param[1]),
			emit_FIXNUMP (n));
      emit_cond_jump (
	emit_binary_op (
	  GCC_JIT_BINARY_OP_LOGICAL_AND,
	  comp.bool_type,
	  sure_fixnum,
	  gcc_jit_context_new_comparison (
	    comp.ctxt,
	    std::ptr::null_mut (),
	    GCC_JIT_COMPARISON_NE,
	    n_fixnum,
	    if i == 0
	    { emit_rvalue_from_emacs_int (MOST_POSITIVE_FIXNUM) }
	    else
	    { emit_rvalue_from_emacs_int (MOST_NEGATIVE_FIXNUM) })),
	inline_block,
	fcall_block);

      comp.block = inline_block;
      let inline_res: *mut gcc_jit_rvalue =
	emit_binary_op (op[i as usize], comp.emacs_int_type, n_fixnum, comp.one);

      gcc_jit_block_end_with_return (inline_block,
				     std::ptr::null_mut (),
				     emit_make_fixnum (inline_res));

      comp.block = fcall_block;
      let call_res: *mut gcc_jit_rvalue =
	emit_call (intern_c_string (fall_back_func[i as usize]),
		   comp.lisp_obj_type, 1,
		   &raw const n as *mut *mut gcc_jit_rvalue, false);
      gcc_jit_block_end_with_return (fcall_block,
				     std::ptr::null_mut (),
				     call_res);
      i += 1;
    }
  comp.block = bb_orig;
  comp.add1 = func[0];
  comp.sub1 = func[1];
}

/// C 3401-3406: `struct cast_type { gcc_jit_type *type; const char *name; bool is_ptr; };` - the
/// record the cast matrix is built from, declared in C between the two functions that use it.
#[cfg (all (feature = "have_native_comp",
	    not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cast_type
{
  pub r#type: *mut gcc_jit_type,
  pub name: *const c_char,
  pub is_ptr: bool,
}

/// C 3363-3399: `static gcc_jit_function *define_type_punning (const char *name,
/// gcc_jit_type *from, gcc_jit_field *from_field, gcc_jit_type *to, gcc_jit_field *to_field)`.
/// C 3379's `DECL_BLOCK (entry_block, result)` is the file's `DECL_BLOCK!` macro (C 507-509).
#[cfg (all (feature = "have_native_comp",
	    not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
unsafe fn define_type_punning (name: *const c_char,
			       from: *mut gcc_jit_type, from_field: *mut gcc_jit_field,
			       to: *mut gcc_jit_type, to_field: *mut gcc_jit_field)
  -> *mut gcc_jit_function
{
  let param: *mut gcc_jit_param = gcc_jit_context_new_param (comp.ctxt, std::ptr::null_mut (),
                                                    from, c"arg".as_ptr ());
  let result: *mut gcc_jit_function = gcc_jit_context_new_function (comp.ctxt,
                               std::ptr::null_mut (),
                               GCC_JIT_FUNCTION_INTERNAL,
                               to,
                               name,
                               1,
                               &raw const param as *mut *mut gcc_jit_param,
                               0);

  let mut entry_block: *mut gcc_jit_block =
    gcc_jit_function_new_block (result, c"entry_block".as_ptr ());

  let tmp_union: *mut gcc_jit_lvalue
    = gcc_jit_function_new_local (result,
                                  std::ptr::null_mut (),
                                  comp.cast_union_type,
                                  c"union_cast".as_ptr ());

  gcc_jit_block_add_assignment (entry_block, std::ptr::null_mut (),
                                gcc_jit_lvalue_access_field (tmp_union, std::ptr::null_mut (),
							     from_field),
                                gcc_jit_param_as_rvalue (param));

  gcc_jit_block_end_with_return (entry_block,
                                 std::ptr::null_mut (),
                                 gcc_jit_rvalue_access_field (
                                   gcc_jit_lvalue_as_rvalue (tmp_union),
                                   std::ptr::null_mut (), to_field));

  return result;
}

/// C 3408-3446: `static gcc_jit_function *define_cast_from_to (struct cast_type from,
/// struct cast_type to)`.
#[cfg (all (feature = "have_native_comp",
	    not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
unsafe fn define_cast_from_to (from: cast_type, to: cast_type) -> *mut gcc_jit_function
{
  let name: *mut c_char = format_string (c"cast_from_%s_to_%s".as_ptr (), from.name, to.name);
  let param: *mut gcc_jit_param = gcc_jit_context_new_param (comp.ctxt, std::ptr::null_mut (),
						    from.r#type, c"arg".as_ptr ());
  let result: *mut gcc_jit_function
    = gcc_jit_context_new_function (comp.ctxt,
				    std::ptr::null_mut (),
				    GCC_JIT_FUNCTION_INTERNAL,
				    to.r#type, name,
				    1, &raw const param as *mut *mut gcc_jit_param, 0);
  let mut entry_block: *mut gcc_jit_block =
    gcc_jit_function_new_block (result, c"entry_block".as_ptr ());

  let mut tmp: *mut gcc_jit_rvalue = gcc_jit_param_as_rvalue (param);
  if from.is_ptr != to.is_ptr
    {
      if from.is_ptr
	{
	  tmp = gcc_jit_context_new_cast (comp.ctxt, std::ptr::null_mut (),
					  tmp, comp.void_ptr_type);
	  tmp = gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (),
					  comp.cast_ptr_to_int, 1,
					  &raw mut tmp as *mut *mut gcc_jit_rvalue);
	}
      else
	{
	  tmp = gcc_jit_context_new_cast (comp.ctxt, std::ptr::null_mut (),
					  tmp, comp.uintptr_type);
	  tmp = gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (),
					  comp.cast_int_to_ptr, 1,
					  &raw mut tmp as *mut *mut gcc_jit_rvalue);
	}
    }

  tmp = gcc_jit_context_new_cast (comp.ctxt, std::ptr::null_mut (), tmp, to.r#type);

  gcc_jit_block_end_with_return (entry_block, std::ptr::null_mut (), tmp);

  return result;
}

/// C 3448-3505: `static void define_cast_functions (void)`.  C's `struct cast_type
/// cast_types[NUM_CAST_TYPES]` initialiser is a constant table here; the two loops fill
/// `comp.cast_types` and the `cast_functions_from_to` matrix as C's do.
#[cfg (all (feature = "have_native_comp",
	    not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")))]
unsafe fn define_cast_functions ()
{
  let cast_types: [cast_type; NUM_CAST_TYPES]
    = [ cast_type { r#type: comp.bool_type, name: c"bool".as_ptr (), is_ptr: false },
        cast_type { r#type: comp.char_ptr_type, name: c"char_ptr".as_ptr (), is_ptr: true },
        cast_type { r#type: comp.int_type, name: c"int".as_ptr (), is_ptr: false },
        cast_type { r#type: comp.lisp_cons_ptr_type, name: c"lisp_cons_ptr".as_ptr (),
		    is_ptr: true },
        cast_type { r#type: comp.lisp_obj_ptr_type, name: c"lisp_obj_ptr".as_ptr (),
		    is_ptr: true },
        cast_type { r#type: comp.lisp_word_tag_type, name: c"lisp_word_tag".as_ptr (),
		    is_ptr: false },
        cast_type { r#type: comp.lisp_word_type, name: c"lisp_word".as_ptr (),
		    is_ptr: LISP_WORDS_ARE_POINTERS },
        cast_type { r#type: comp.long_long_type, name: c"long_long".as_ptr (), is_ptr: false },
        cast_type { r#type: comp.long_type, name: c"long".as_ptr (), is_ptr: false },
        cast_type { r#type: comp.ptrdiff_type, name: c"ptrdiff".as_ptr (), is_ptr: false },
        cast_type { r#type: comp.uintptr_type, name: c"uintptr".as_ptr (), is_ptr: false },
        cast_type { r#type: comp.unsigned_long_long_type, name: c"unsigned_long_long".as_ptr (),
		    is_ptr: false },
        cast_type { r#type: comp.unsigned_long_type, name: c"unsigned_long".as_ptr (),
		    is_ptr: false },
        cast_type { r#type: comp.unsigned_type, name: c"unsigned".as_ptr (), is_ptr: false },
        cast_type { r#type: comp.void_ptr_type, name: c"void_ptr".as_ptr (), is_ptr: true } ];
  let mut cast_union_fields: [*mut gcc_jit_field; 2] = [std::ptr::null_mut (); 2];

  /* Define the union used for type punning.  */
  cast_union_fields[0] = gcc_jit_context_new_field (comp.ctxt,
						    std::ptr::null_mut (),
						    comp.void_ptr_type,
						    c"void_ptr".as_ptr ());
  cast_union_fields[1] = gcc_jit_context_new_field (comp.ctxt,
						    std::ptr::null_mut (),
						    comp.uintptr_type,
						    c"uintptr".as_ptr ());

  comp.cast_union_type
    = gcc_jit_context_new_union_type (comp.ctxt,
				      std::ptr::null_mut (),
				      c"cast_union".as_ptr (),
				      2, cast_union_fields.as_ptr () as *mut *mut gcc_jit_field);

  comp.cast_ptr_to_int = define_type_punning (c"cast_pointer_to_uintptr_t".as_ptr (),
					      comp.void_ptr_type,
					      cast_union_fields[0],
					      comp.uintptr_type,
					      cast_union_fields[1]);
  comp.cast_int_to_ptr = define_type_punning (c"cast_uintptr_t_to_pointer".as_ptr (),
					      comp.uintptr_type,
					      cast_union_fields[1],
					      comp.void_ptr_type,
					      cast_union_fields[0]);


  let mut i: usize = 0;
  while i < NUM_CAST_TYPES
    {
      comp.cast_types[i] = cast_types[i].r#type;
      i += 1;
    }

  /* Define the cast functions using a matrix.  */
  let mut i: usize = 0;
  while i < NUM_CAST_TYPES
    {
      let mut j: usize = 0;
      while j < NUM_CAST_TYPES
        {
          comp.cast_functions_from_to[i][j] =
            define_cast_from_to (cast_types[i], cast_types[j]);
          j += 1;
        }
      i += 1;
    }
}

/// C 3317-3360: `static void define_thread_state_struct (void)`.
#[cfg(feature = "have_native_comp")]
unsafe fn define_thread_state_struct ()
{
  /* Partially opaque definition for `thread_state'.
     Because we need to access just m_handlerlist hopefully this is requires
     less manutention then the full deifnition.	 */

  comp.m_handlerlist = gcc_jit_context_new_field (comp.ctxt,
						  std::ptr::null_mut (),
						  comp.handler_ptr_type,
						  c"m_handlerlist".as_ptr ());
  let fields: [*mut gcc_jit_field; 3] =
    [ gcc_jit_context_new_field (
	comp.ctxt,
	std::ptr::null_mut (),
	gcc_jit_context_new_array_type (comp.ctxt,
					std::ptr::null_mut (),
					comp.char_type,
					core::mem::offset_of! (thread_state,
							  m_handlerlist) as c_int),
	c"pad0".as_ptr ()),
      comp.m_handlerlist,
      gcc_jit_context_new_field (
	comp.ctxt,
	std::ptr::null_mut (),
	gcc_jit_context_new_array_type (
	  comp.ctxt,
	  std::ptr::null_mut (),
	  comp.char_type,
	  (core::mem::size_of::<thread_state> ()
	  - core::mem::offset_of! (thread_state,
		      m_handlerlist)
	  - core::mem::size_of::<*mut handler> ()) as c_int),
	c"pad1".as_ptr ()) ];

  comp.thread_state_s =
    gcc_jit_context_new_struct_type (comp.ctxt,
				     std::ptr::null_mut (),
				     c"comp_thread_state".as_ptr (),
				     countof! (fields) as c_int,
				     fields.as_ptr () as *mut *mut gcc_jit_field);
  comp.thread_state_ptr_type =
    gcc_jit_type_get_pointer (gcc_jit_struct_as_type (comp.thread_state_s));
}

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
  let const_vld: Lisp_Object = CALLNI! ("comp-cstr-imm-vld-p", mvar);

  if ! NILP (const_vld)
    {
      let value: Lisp_Object = CALLNI! ("comp-cstr-imm", mvar);
      if NILP (Fgethash (value,
			  CALLNI! ("comp-ctxt-non-materializable-objs-h", Vcomp_ctxt),
			  Qnil ()))
	{
	  if comp.debug > 1
	    {
	      let func: Lisp_Object =
		Fgethash (value,
			  CALLNI! ("comp-ctxt-byte-func-to-func-h", Vcomp_ctxt),
			  Qnil ());

	      emit_comment (
	        SSDATA (
		  Fprin1_to_string (
		    if NILP (func) { value }
		    else { CALLNI! ("comp-func-c-name", func) },
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
      let first_slot: EMACS_INT = XFIXNUM (CALLNI! ("comp-mvar-slot", first_arg));
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

      if (! NILP (CALLNI! ("comp-cstr-imm-vld-p", arg[0]))
	   && NILP (CALLNI! ("comp-cstr-imm", arg[0])))
	  || (! NILP (CALLNI! ("comp-cstr-imm-vld-p", arg[1]))
	      && NILP (CALLNI! ("comp-cstr-imm", arg[1])))
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

      let slot_n: EMACS_INT = XFIXNUM (CALLNI! ("comp-mvar-slot", arg[0]));
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
    && ! NILP (CALLNI! ("comp-mvar-type-hint-match-p", SECOND! (insn), type_));
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
    && ! NILP (CALLNI! ("comp-mvar-type-hint-match-p", SECOND! (insn), type_));
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

/* This is in charge of serializing an object and export a function to
   retrieve it at load time.  */
/* `#pragma GCC diagnostic push`/`pop` and the ignored `-Waddress` around C 2661-2852 are C's own
   diagnostics control and have no Rust counterpart.  */
/// C 2661-2851: `static void emit_static_object (const char *name, Lisp_Object obj)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_static_object (name: *const c_char, obj: Lisp_Object)
{
  /* libgccjit has no support for initialized static data.
     The mechanism below is certainly not aesthetic but I assume the bottle neck
     in terms of performance at load time will still be the reader.
     NOTE: we can not rely on libgccjit even for valid NULL terminated C
     strings cause of this funny bug that will affect all pre gcc10 era gccs:
     https://gcc.gnu.org/ml/jit/2019-q3/msg00013.html  */

  let count: specpdl_ref = SPECPDL_INDEX ();
  /* Preserve uninterned symbols, this is specifically necessary for
     CL macro expansion in dynamic scope code (bug#42088).  See
     `byte-compile-output-file-form'.  */
  specbind (intern_c_string (c"print-escape-newlines".as_ptr ()), Qt ());
  specbind (intern_c_string (c"print-length".as_ptr ()), Qnil ());
  specbind (intern_c_string (c"print-level".as_ptr ()), Qnil ());
  specbind (intern_c_string (c"print-quoted".as_ptr ()), Qt ());
  specbind (intern_c_string (c"print-gensym".as_ptr ()), Qt ());
  specbind (intern_c_string (c"print-circle".as_ptr ()), Qt ());
  /* Bind print-number-table and print-continuous-numbering so comp--#$
     prints as #$.  */
  let print_number_table: Lisp_Object =
    CALLN! (Fmake_hash_table, QCtest (), Qeq ());
  Fputhash (Vcomp__hashdollar, build_string (c"#$".as_ptr ()) , print_number_table);
  specbind (intern_c_string (c"print-number-table".as_ptr ()), print_number_table);
  specbind (intern_c_string (c"print-continuous-numbering".as_ptr ()), Qt ());
  let str_: Lisp_Object = Fprin1_to_string (obj, Qnil (), Qnil ());
  unbind_to (count, Qnil ());

  let len: ptrdiff_t = SBYTES (str_);
  let mut p: *const c_char = SSDATA (str_);

  #[cfg (feature = "libgccjit_have_gcc_jit_global_set_initializer")]
  {
    if fn_gcc_jit_global_set_initializer.is_some () || ! cfg! (feature = "windowsnt")
      {
	let str_size: ptrdiff_t = len + 1;
	let size: ptrdiff_t = core::mem::size_of::<static_obj_t> () as ptrdiff_t + str_size;
	let static_obj: *mut static_obj_t = xmalloc (size as usize) as *mut static_obj_t;
	(*static_obj).len = str_size;
	memcpy ((*static_obj).data.as_mut_ptr () as *mut c_void, p as *const c_void,
		str_size as usize);
	let blob: *mut gcc_jit_lvalue =
	  gcc_jit_context_new_global (
	    comp.ctxt,
	    std::ptr::null_mut (),
	    GCC_JIT_GLOBAL_EXPORTED,
	    gcc_jit_context_new_array_type (comp.ctxt, std::ptr::null_mut (),
					    comp.char_type,
					    size as c_int),
	    format_string (c"%s_blob".as_ptr (), name));
	gcc_jit_global_set_initializer (blob, static_obj as *const c_void, size as size_t);
	xfree (static_obj as *mut c_void);

	return;
      }
  }

  let a_type: *mut gcc_jit_type =
    gcc_jit_context_new_array_type (comp.ctxt,
				    std::ptr::null_mut (),
				    comp.char_type,
				    (len + 1) as c_int);
  let fields: [*mut gcc_jit_field; 2] =
    [ gcc_jit_context_new_field (comp.ctxt,
				 std::ptr::null_mut (),
				 comp.ptrdiff_type,
				 c"len".as_ptr ()),
      gcc_jit_context_new_field (comp.ctxt,
				 std::ptr::null_mut (),
				 a_type,
				 c"data".as_ptr ()) ];

  let data_struct_t: *mut gcc_jit_type =
    gcc_jit_struct_as_type (
      gcc_jit_context_new_struct_type (comp.ctxt,
				       std::ptr::null_mut (),
				       format_string (c"%s_struct".as_ptr (), name),
				       countof! (fields) as c_int,
				       fields.as_ptr () as *mut *mut gcc_jit_field));

  let data_struct: *mut gcc_jit_lvalue =
    gcc_jit_context_new_global (comp.ctxt,
				std::ptr::null_mut (),
				GCC_JIT_GLOBAL_INTERNAL,
				data_struct_t,
				format_string (c"%s_s".as_ptr (), name));

  let f: *mut gcc_jit_function =
    gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				  GCC_JIT_FUNCTION_EXPORTED,
				  gcc_jit_type_get_pointer (data_struct_t),
				  name,
				  0, std::ptr::null_mut (), 0);
  let mut block: *mut gcc_jit_block = gcc_jit_function_new_block (f, std::ptr::null_mut ());

  if comp.debug > 1
    {
      let comment: *mut c_char = memcpy (xmalloc (len as usize), p as *const c_void,
					 len as usize) as *mut c_char;
      let mut i: ptrdiff_t = 0;
      while i < len - 1
	{
	  if *comment.offset (i as isize) == 0
	    { *comment.offset (i as isize) = b'\n' as c_char; }
	  i += 1;
	}
      gcc_jit_block_add_comment (block, std::ptr::null_mut (), comment);
      xfree (comment as *mut c_void);
    }

  let arr: *mut gcc_jit_lvalue =
      gcc_jit_lvalue_access_field (data_struct, std::ptr::null_mut (), fields[1]);

  let ptrvar: *mut gcc_jit_lvalue = gcc_jit_function_new_local (f, std::ptr::null_mut (),
                                                       comp.char_ptr_type,
                                                       c"ptr".as_ptr ());

  gcc_jit_block_add_assignment (
    block,
    std::ptr::null_mut (),
    ptrvar,
    gcc_jit_lvalue_get_address (
      gcc_jit_context_new_array_access (
        comp.ctxt,
        std::ptr::null_mut (),
        gcc_jit_lvalue_as_rvalue (arr),
        gcc_jit_context_new_rvalue_from_int (comp.ctxt, comp.int_type, 0)),
      std::ptr::null_mut ()));

  /* We can't use always string literals longer that 200 bytes because
     they cause a crash in pre GCC 10 libgccjit.
     <https://gcc.gnu.org/ml/jit/2019-q3/msg00013.html>.

     Adjust if possible to reduce the number of function calls.  */
  let mut buff: [c_char; 1024] = [0; 1024];
  let chunk_size: c_int = if NILP (Fcomp_libgccjit_version ()) { 200 }
                          else { core::mem::size_of_val (&buff) as c_int };
  let mut i: ptrdiff_t = 0;
  while i < len
    {
      let l: c_int = strnlen (p, (chunk_size - 1) as size_t) as c_int;

      if l != 0
        {
	  let buff_end: *mut c_void = mempcpy (buff.as_mut_ptr () as *mut c_void,
					      p as *const c_void, l as size_t);
	  *(buff_end as *mut c_char) = 0;

          p = p.offset (l as isize);
          i += l as ptrdiff_t;

          let args: [*mut gcc_jit_rvalue; 3] =
	    [ gcc_jit_lvalue_as_rvalue (ptrvar),
	      gcc_jit_context_new_string_literal (comp.ctxt, buff.as_ptr ()),
	      gcc_jit_context_new_rvalue_from_int (comp.ctxt,
						   comp.size_t_type,
						   l) ];

          gcc_jit_block_add_eval (block, std::ptr::null_mut (),
                                  gcc_jit_context_new_call (comp.ctxt, std::ptr::null_mut (),
                                                            comp.memcpy,
							    countof! (args) as c_int,
							    args.as_ptr () as *mut *mut gcc_jit_rvalue));
          gcc_jit_block_add_assignment (block, std::ptr::null_mut (), ptrvar,
            gcc_jit_lvalue_get_address (
              gcc_jit_context_new_array_access (comp.ctxt, std::ptr::null_mut (),
                gcc_jit_lvalue_as_rvalue (ptrvar),
                gcc_jit_context_new_rvalue_from_int (comp.ctxt,
                                                     comp.uintptr_type,
                                                     l)),
              std::ptr::null_mut ()));
        }
      else
        {
          /* If strlen returned 0 that means that the static object
             contains a NULL byte.  In that case just move over to the
             next block.  We can rely on the byte being zero because
             the dynamic linker cleared it.  */
          p = p.offset (1);
          i += 1;
          gcc_jit_block_add_assignment (
            block, std::ptr::null_mut (), ptrvar,
            gcc_jit_lvalue_get_address (
              gcc_jit_context_new_array_access (
                comp.ctxt, std::ptr::null_mut (), gcc_jit_lvalue_as_rvalue (ptrvar),
                gcc_jit_context_new_rvalue_from_int (comp.ctxt,
                                                     comp.uintptr_type, 1)),
              std::ptr::null_mut ()));
        }
    }

  gcc_jit_block_add_assignment (
	block,
	std::ptr::null_mut (),
	gcc_jit_lvalue_access_field (data_struct, std::ptr::null_mut (), fields[0]),
	gcc_jit_context_new_rvalue_from_int (comp.ctxt,
					     comp.ptrdiff_type,
					     len as c_int));
  let res: *mut gcc_jit_rvalue = gcc_jit_lvalue_get_address (data_struct, std::ptr::null_mut ());
  gcc_jit_block_end_with_return (block, std::ptr::null_mut (), res);
}

/// C 2894-2908: `static void declare_imported_data (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn declare_imported_data ()
{
  /* Imported objects.  */
  comp.data_relocs =
    declare_imported_data_relocs (CALLNI! ("comp-ctxt-d-default", Vcomp_ctxt),
				  DATA_RELOC_SYM,
				  DATA_RELOC_ADDR_SYM,
				  TEXT_DATA_RELOC_SYM);
  comp.data_relocs_ephemeral =
    declare_imported_data_relocs (CALLNI! ("comp-ctxt-d-ephemeral", Vcomp_ctxt),
				  DATA_RELOC_EPHEMERAL_SYM,
				  DATA_RELOC_EPHEMERAL_ADDR_SYM,
				  TEXT_DATA_RELOC_EPHEMERAL_SYM);
}

/*
  Declare as imported all the functions that are requested from the runtime.
  These are either subrs or not.  Note that the list created here must match
  the array `helper_link_table'.
*/
/// C 2920-2927: `#define ADD_IMPORTED(f_name, ret_type, nargs, args)` - C's `STR (f_name)`
/// stringifies the token, which a Rust macro cannot, so the call sites pass the same text as a
/// `c"..."` literal (the `CALLNI!` shape above).  C's macro also writes the enclosing
/// `field_list` by name; a `macro_rules!` is hygienic, so that variable is a parameter here.
#[cfg (feature = "have_native_comp")]
macro_rules! ADD_IMPORTED {
  ($field_list:ident, $f_name:expr, $ret_type:expr, $nargs:expr, $args:expr) => {{
    let name: Lisp_Object = intern_c_string ($f_name);
    let field: Lisp_Object =
      make_mint_ptr (declare_imported_func (name, $ret_type, $nargs, $args) as *mut c_void);
    let el: Lisp_Object = Fcons (name, field);
    $field_list = Fcons (el, $field_list);
  }};
}

/// C 2915-2979: `static Lisp_Object declare_runtime_imported_funcs (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn declare_runtime_imported_funcs () -> Lisp_Object
{
  let mut field_list: Lisp_Object = Qnil ();

  let mut args: [*mut gcc_jit_type; 4] = [std::ptr::null_mut (); 4];

  ADD_IMPORTED! (field_list, c"wrong_type_argument".as_ptr (), comp.void_type, 2, std::ptr::null_mut ());

  args[0] = comp.lisp_obj_type;
  args[1] = comp.int_type;
  ADD_IMPORTED! (field_list, c"helper_PSEUDOVECTOR_TYPEP_XUNTAG".as_ptr (), comp.bool_type, 2,
		 args.as_mut_ptr ());

  args[0] = comp.lisp_obj_type;
  args[1] = comp.int_type;
  ADD_IMPORTED! (field_list, c"push_handler".as_ptr (), comp.handler_ptr_type, 2, args.as_mut_ptr ());

  ADD_IMPORTED! (field_list, c"record_unwind_protect_excursion".as_ptr (), comp.void_type, 0,
		 std::ptr::null_mut ());

  args[0] = comp.lisp_obj_type;
  ADD_IMPORTED! (field_list, c"helper_unbind_n".as_ptr (), comp.lisp_obj_type, 1, args.as_mut_ptr ());

  ADD_IMPORTED! (field_list, c"helper_save_restriction".as_ptr (), comp.void_type, 0,
		 std::ptr::null_mut ());

  args[0] = comp.lisp_obj_type;
  ADD_IMPORTED! (field_list, c"helper_GET_SYMBOL_WITH_POSITION".as_ptr (),
		 comp.lisp_symbol_with_position_ptr_type,
		 1, args.as_mut_ptr ());

  args[0] = comp.lisp_obj_type;
  args[1] = comp.lisp_obj_type;
  ADD_IMPORTED! (field_list, c"slow_eq".as_ptr (), comp.bool_type, 2, args.as_mut_ptr ());

  args[0] = comp.lisp_obj_type;
  args[1] = comp.lisp_obj_type;
  ADD_IMPORTED! (field_list, c"helper_sanitizer_assert".as_ptr (), comp.lisp_obj_type, 2,
		 args.as_mut_ptr ());

  ADD_IMPORTED! (field_list, c"record_unwind_current_buffer".as_ptr (), comp.void_type, 0,
		 std::ptr::null_mut ());

  args[0] = comp.lisp_obj_type;
  args[1] = comp.lisp_obj_type;
  args[2] = comp.lisp_obj_type;
  args[3] = comp.int_type;
  ADD_IMPORTED! (field_list, c"set_internal".as_ptr (), comp.void_type, 4, args.as_mut_ptr ());

  args[0] = comp.lisp_obj_type;
  ADD_IMPORTED! (field_list, c"helper_unwind_protect".as_ptr (), comp.void_type, 1, args.as_mut_ptr ());

  args[0] = comp.lisp_obj_type;
  args[1] = comp.lisp_obj_type;
  ADD_IMPORTED! (field_list, c"specbind".as_ptr (), comp.void_type, 2, args.as_mut_ptr ());

  ADD_IMPORTED! (field_list, c"maybe_gc".as_ptr (), comp.void_type, 0, std::ptr::null_mut ());

  ADD_IMPORTED! (field_list, c"maybe_quit".as_ptr (), comp.void_type, 0, std::ptr::null_mut ());

  return Freverse (field_list);
}

/*
  This emit the code needed by every compilation unit to be loaded.
*/
/// C 2984-3068: `static void emit_ctxt_code (void)`.
#[cfg (feature = "have_native_comp")]
unsafe fn emit_ctxt_code ()
{
  /* Emit optimize qualities.  */
  let opt_qly: [Lisp_Object; 3] =
    [ Fcons (Qnative_comp_speed (), make_fixnum (comp.speed)),
      Fcons (Qnative_comp_debug (), make_fixnum (comp.debug)),
      Fcons (Qgccjit (),
	     Fcomp_libgccjit_version ()) ];
  emit_static_object (TEXT_OPTIM_QLY_SYM,
		      Flist (countof! (opt_qly) as ptrdiff_t,
			     opt_qly.as_ptr () as *mut Lisp_Object));

  emit_static_object (TEXT_FDOC_SYM,
		      CALLNI! ("comp-ctxt-function-docs", Vcomp_ctxt));

  comp.current_thread_ref =
    gcc_jit_lvalue_as_rvalue (
      gcc_jit_context_new_global (
	comp.ctxt,
	std::ptr::null_mut (),
	GCC_JIT_GLOBAL_EXPORTED,
	gcc_jit_type_get_pointer (comp.thread_state_ptr_type),
	CURRENT_THREAD_RELOC_SYM));

  comp.f_symbols_with_pos_enabled_ref =
    gcc_jit_lvalue_as_rvalue (
      gcc_jit_context_new_global (
	comp.ctxt,
	std::ptr::null_mut (),
	GCC_JIT_GLOBAL_EXPORTED,
	comp.bool_ptr_type,
	F_SYMBOLS_WITH_POS_ENABLED_RELOC_SYM));

  gcc_jit_context_new_global (
	comp.ctxt,
	std::ptr::null_mut (),
	GCC_JIT_GLOBAL_EXPORTED,
	comp.lisp_obj_type,
	COMP_UNIT_SYM);

  declare_imported_data ();

  /* Functions imported from Lisp code.	 */
  freloc_check_fill ();
  let mut fields: *mut *mut gcc_jit_field =
    xmalloc ((freloc.size as usize) * core::mem::size_of::<*mut gcc_jit_field> ())
      as *mut *mut gcc_jit_field;
  let mut n_frelocs: ptrdiff_t = 0;
  let mut f_runtime: Lisp_Object = declare_runtime_imported_funcs ();
  FOR_EACH_TAIL! (f_runtime, {
      let el: Lisp_Object = XCAR (f_runtime);
      eassert! (n_frelocs < freloc.size);
      *fields.offset (n_frelocs as isize) =
	xmint_pointer (XCDR (el)) as *mut gcc_jit_field;
      n_frelocs += 1;
    });

  /* Sign the .eln for the exposed ABI it expects at load.  */
  eassert! (! NILP (Vcomp_abi_hash));
  emit_static_object (LINK_TABLE_HASH_SYM, Vcomp_abi_hash);

  let mut subr_l: Lisp_Object = Vcomp_subr_list;
  FOR_EACH_TAIL! (subr_l, {
      let subr: *mut Lisp_Subr = XSUBR (XCAR (subr_l));
      let subr_sym: Lisp_Object = intern_c_string ((*subr).symbol_name);
      eassert! (n_frelocs < freloc.size);
      *fields.offset (n_frelocs as isize) =
	declare_imported_func (subr_sym, comp.lisp_obj_type,
			       (*subr).max_args as c_int,   // C's `short` -> `int` widening
			       std::ptr::null_mut ());
      n_frelocs += 1;
    });

  let f_reloc_struct: *mut gcc_jit_struct =
    gcc_jit_context_new_struct_type (comp.ctxt,
				     std::ptr::null_mut (),
				     c"freloc_link_table".as_ptr (),
				     n_frelocs as c_int, fields);
  comp.func_relocs_ptr_type =
    gcc_jit_type_get_pointer (
      gcc_jit_struct_as_type (f_reloc_struct));

  comp.func_relocs =
    gcc_jit_context_new_global (comp.ctxt,
				std::ptr::null_mut (),
				GCC_JIT_GLOBAL_EXPORTED,
				comp.func_relocs_ptr_type,
				FUNC_LINK_TABLE_SYM);

  xfree (fields as *mut c_void);
}

/// C 2854-2892: `static reloc_array_t declare_imported_data_relocs (Lisp_Object container,
/// const char *code_symbol, const char *addr_fun_symbol, const char *text_symbol)`.
#[cfg (feature = "have_native_comp")]
unsafe fn declare_imported_data_relocs (container: Lisp_Object,
					code_symbol: *const c_char,
					addr_fun_symbol: *const c_char,
					text_symbol: *const c_char) -> reloc_array_t
{
  /* Imported objects.  */
  let len: EMACS_INT =
    XFIXNUM (CALLNI! ("hash-table-count",
		     CALLNI! ("comp-data-container-idx", container)));
  let mut d_reloc: Lisp_Object = CALLNI! ("comp-data-container-l", container);
  d_reloc = Fvconcat (1, &raw mut d_reloc);

  emit_static_object (text_symbol, d_reloc);

  let d_reloc_type: *mut gcc_jit_type
    = gcc_jit_context_new_array_type (comp.ctxt, std::ptr::null_mut (),
				      comp.lisp_obj_type, len as c_int);
  let d_reloc_lval: *mut gcc_jit_lvalue
    = gcc_jit_context_new_global (comp.ctxt, std::ptr::null_mut (),
				  GCC_JIT_GLOBAL_INTERNAL,
				  d_reloc_type, code_symbol);
  let d_reloc_rval: *mut gcc_jit_rvalue
    = gcc_jit_lvalue_as_rvalue (d_reloc_lval);
  let addr_rval: *mut gcc_jit_rvalue
    = gcc_jit_lvalue_get_address (d_reloc_lval, std::ptr::null_mut ());
  let addr_rval_type: *mut gcc_jit_type = gcc_jit_rvalue_get_type (addr_rval);
  let get_addr_fun: *mut gcc_jit_function
    = gcc_jit_context_new_function (comp.ctxt, std::ptr::null_mut (),
				    GCC_JIT_FUNCTION_EXPORTED,
				    addr_rval_type, addr_fun_symbol,
				    0, std::ptr::null_mut (), 0);
  let block: *mut gcc_jit_block
    = gcc_jit_function_new_block (get_addr_fun, std::ptr::null_mut ());
  gcc_jit_block_end_with_return (block, std::ptr::null_mut (), addr_rval);

  let res: reloc_array_t = reloc_array_t { len: len, r_val: d_reloc_rval };
  return res;
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
/* C's `DEFVAR_*` in `syms_of_comp` expands to the definition of each global plus its
   registration; the port's macros only register, and `check-globals.py` requires the definition to
   live in the unit that registers it (P-313).  Types and initial values are C's: `LISP` starts as
   nil, `BOOL` as false - `syms_of_comp` assigns the rest.  */

#[unsafe(no_mangle)]
pub static mut native_comp_jit_compilation: bool = false;

#[unsafe(no_mangle)]
pub static mut Vcomp_ctxt: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut Vcomp_subr_list: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut Vcomp_abi_hash: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut Vcomp_native_version_dir: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut Vcomp_deferred_pending_h: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut Vnative_comp_eln_load_path: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut Vnative_comp_enable_subr_trampolines: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut Vcomp_installed_trampolines_h: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut V_comp_no_native_file_h: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut comp_file_preloaded_p: bool = false;

#[unsafe(no_mangle)]
pub static mut Vcomp_loaded_comp_units_h: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut Vcomp_subr_arities_h: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut comp_sanitizer_active: bool = false;

#[unsafe(no_mangle)]
pub static mut Vcomp__hashdollar: Lisp_Object = lisp_h_Qnil ();

}

/// C 5593-5857: `syms_of_comp`.  C 5596-5854's `#ifdef HAVE_NATIVE_COMP` region is this file's
/// other dead region (the `native-comp-jit-compilation` `DEFVAR_BOOL`, the `comp--#$`
/// `DEFVAR_LISP` and the fifty-odd `DEFSYM`s for the Limple opcodes and the compiler options);
/// it is **not yet transcribed** - it is outside this slice's C 25-822 and `docs/ports/comp.md`
/// records it as the remaining work.  C 5856's `defsubr` is outside the `#ifdef`, as C has it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_comp ()
{
  /* C 5596-5854: `#ifdef HAVE_NATIVE_COMP`, the file gate's `syms_of` region.  */
  #[cfg (feature = "have_native_comp")]
  {
  DEFVAR_BOOL! (c"native-comp-jit-compilation", native_comp_jit_compilation,
    doc: "If non-nil, compile loaded .elc files asynchronously.\n\nAfter compilation, each function definition is updated to use the\nnatively compiled one.");
  native_comp_jit_compilation = true;

  DEFSYM! (Qnative_comp_speed, c"native-comp-speed");
  DEFSYM! (Qnative_comp_debug, c"native-comp-debug");
  DEFSYM! (Qnative_comp_driver_options, c"native-comp-driver-options");
  DEFSYM! (Qnative_comp_compiler_options, c"native-comp-compiler-options");
  DEFSYM! (Qcomp_libgccjit_reproducer, c"comp-libgccjit-reproducer");

  /* Limple instruction set.  */
  DEFSYM! (Qcomment, c"comment");
  DEFSYM! (Qjump, c"jump");
  DEFSYM! (Qcall, c"call");
  DEFSYM! (Qcallref, c"callref");
  DEFSYM! (Qdirect_call, c"direct-call");
  DEFSYM! (Qdirect_callref, c"direct-callref");
  DEFSYM! (Qassume, c"assume");
  DEFSYM! (Qsetimm, c"setimm");
  DEFSYM! (Qreturn, c"return");
  DEFSYM! (Qunreachable, c"unreachable");
  DEFSYM! (Qcomp_mvar, c"comp-mvar");
  DEFSYM! (Qcond_jump, c"cond-jump");
  DEFSYM! (Qphi, c"phi");
  /* Ops in use for prologue emission.  */
  DEFSYM! (Qset_par_to_local, c"set-par-to-local");
  DEFSYM! (Qset_args_to_local, c"set-args-to-local");
  DEFSYM! (Qset_rest_args_to_local, c"set-rest-args-to-local");
  DEFSYM! (Qinc_args, c"inc-args");
  DEFSYM! (Qcond_jump_narg_leq, c"cond-jump-narg-leq");
  /* Others.  */
  DEFSYM! (Qpush_handler, c"push-handler");
  DEFSYM! (Qpop_handler, c"pop-handler");
  DEFSYM! (Qfetch_handler, c"fetch-handler");
  DEFSYM! (Qcondition_case, c"condition-case");
  /* call operands.  */
  DEFSYM! (Qcatcher, c"catcher");
  DEFSYM! (Qentry, c"entry");
  DEFSYM! (Qset_internal, c"set_internal");
  DEFSYM! (Qrecord_unwind_current_buffer, c"record_unwind_current_buffer");
  DEFSYM! (Qrecord_unwind_protect_excursion, c"record_unwind_protect_excursion");
  DEFSYM! (Qhelper_unbind_n, c"helper_unbind_n");
  DEFSYM! (Qhelper_unwind_protect, c"helper_unwind_protect");
  DEFSYM! (Qhelper_save_restriction, c"helper_save_restriction");
  DEFSYM! (Qhelper_sanitizer_assert, c"helper_sanitizer_assert");
  /* Inliners.  */
  DEFSYM! (Qadd1, c"1+");
  DEFSYM! (Qsub1, c"1-");
  DEFSYM! (Qconsp, c"consp");
  DEFSYM! (Qcar, c"car");
  DEFSYM! (Qcdr, c"cdr");
  DEFSYM! (Qsetcar, c"setcar");
  DEFSYM! (Qsetcdr, c"setcdr");
  DEFSYM! (Qnegate, c"negate");
  DEFSYM! (Qnumberp, c"numberp");
  DEFSYM! (Qintegerp, c"integerp");
  DEFSYM! (Qcomp_maybe_gc_or_quit, c"comp-maybe-gc-or-quit");
  DEFSYM! (Qsymbol_with_pos_p, c"symbol-with-pos-p");

  /* Allocation classes. */
  DEFSYM! (Qd_default, c"d-default");
  DEFSYM! (Qd_ephemeral, c"d-ephemeral");

  /* Others.  */
  DEFSYM! (Qnative_compiler, c"native-compiler");
  DEFSYM! (Qfixnum, c"fixnum");
  DEFSYM! (Qscratch, c"scratch");
  DEFSYM! (Qlate, c"late");
  DEFSYM! (Qgccjit, c"gccjit");
  DEFSYM! (Qcomp_subr_trampoline_install, c"comp-subr-trampoline-install");
  DEFSYM! (Qnative_comp_warning_on_missing_source,
	  c"native-comp-warning-on-missing-source");

  /* To be signaled by the compiler.  */
  DEFSYM! (Qnative_compiler_error, c"native-compiler-error");
  Fput (Qnative_compiler_error (), Qerror_conditions (),
	list (Qnative_compiler_error (), Qerror ()));
  Fput (Qnative_compiler_error (), Qerror_message (),
        build_string (c"Native compiler error".as_ptr ()));

  DEFSYM! (Qnative_ice, c"native-ice");
  Fput (Qnative_ice (), Qerror_conditions (),
	list (Qnative_ice (), Qnative_compiler_error (), Qerror ()));
  Fput (Qnative_ice (), Qerror_message (),
        build_string (c"Internal native compiler error".as_ptr ()));

  /* By the load machinery.  */
  DEFSYM! (Qnative_lisp_load_failed, c"native-lisp-load-failed");
  Fput (Qnative_lisp_load_failed (), Qerror_conditions (),
	list (Qnative_lisp_load_failed (), Qerror ()));
  Fput (Qnative_lisp_load_failed (), Qerror_message (),
        build_string (c"Native elisp load failed".as_ptr ()));

  DEFSYM! (Qnative_lisp_wrong_reloc, c"native-lisp-wrong-reloc");
  Fput (Qnative_lisp_wrong_reloc (), Qerror_conditions (),
	list (Qnative_lisp_wrong_reloc (), Qnative_lisp_load_failed (), Qerror ()));
  Fput (Qnative_lisp_wrong_reloc (), Qerror_message (),
        build_string (c"Primitive redefined or wrong relocation".as_ptr ()));

  DEFSYM! (Qwrong_register_subr_call, c"wrong-register-subr-call");
  Fput (Qwrong_register_subr_call (), Qerror_conditions (),
	list (Qwrong_register_subr_call (), Qnative_lisp_load_failed (), Qerror ()));
  Fput (Qwrong_register_subr_call (), Qerror_message (),
        build_string (c"comp--register-subr can only be called during native lisp load phase.".as_ptr ()));

  DEFSYM! (Qnative_lisp_file_inconsistent, c"native-lisp-file-inconsistent");
  Fput (Qnative_lisp_file_inconsistent (), Qerror_conditions (),
	list (Qnative_lisp_file_inconsistent (), Qnative_lisp_load_failed (), Qerror ()));
  Fput (Qnative_lisp_file_inconsistent (), Qerror_message (),
        build_string (c"eln file inconsistent with current runtime configuration, please recompile".as_ptr ()));

  DEFSYM! (Qcomp_sanitizer_error, c"comp-sanitizer-error");
  Fput (Qcomp_sanitizer_error (), Qerror_conditions (),
	list (Qcomp_sanitizer_error (), Qerror ()));
  Fput (Qcomp_sanitizer_error (), Qerror_message (),
        build_string (c"Native code sanitizer runtime error".as_ptr ()));

  DEFSYM! (Qnative__compile_async, c"native--compile-async");

  defsubr (&raw mut Scomp__subr_signature);
  defsubr (&raw mut Scomp_el_to_eln_rel_filename);
  defsubr (&raw mut Scomp_el_to_eln_filename);
  defsubr (&raw mut Scomp_native_driver_options_effective_p);
  defsubr (&raw mut Scomp_native_compiler_options_effective_p);
  defsubr (&raw mut Scomp__install_trampoline);
  defsubr (&raw mut Scomp__init_ctxt);
  defsubr (&raw mut Scomp__release_ctxt);
  defsubr (&raw mut Scomp__compile_ctxt_to_file0);
  defsubr (&raw mut Scomp_libgccjit_version);
  defsubr (&raw mut Scomp__register_lambda);
  defsubr (&raw mut Scomp__register_subr);
  defsubr (&raw mut Scomp__late_register_subr);
  defsubr (&raw mut Snative_elisp_load);

  staticpro (&raw mut comp.exported_funcs_h);
  comp.exported_funcs_h = Qnil ();
  staticpro (&raw mut comp.imported_funcs_h);
  comp.imported_funcs_h = Qnil ();
  staticpro (&raw mut comp.func_blocks_h);
  staticpro (&raw mut comp.emitter_dispatcher);
  comp.emitter_dispatcher = Qnil ();
  staticpro (&raw mut loadsearch_re_list);
  loadsearch_re_list = Qnil ();

  DEFVAR_LISP! (c"comp-ctxt", Vcomp_ctxt,
	       doc: "The compiler context.");
  Vcomp_ctxt = Qnil ();

  /* FIXME should be initialized but not here...  Plus this don't have
     to be necessarily exposed to lisp but can easy debug for now.  */
  DEFVAR_LISP! (c"comp-subr-list", Vcomp_subr_list,
    doc: "List of all defined subrs.");
  DEFVAR_LISP! (c"comp-abi-hash", Vcomp_abi_hash,
    doc: "String signing the .eln files ABI.");
  Vcomp_abi_hash = Qnil ();
  DEFVAR_LISP! (c"comp-native-version-dir", Vcomp_native_version_dir,
    doc: "Directory in use to disambiguate eln compatibility.");
  Vcomp_native_version_dir = Qnil ();

  DEFVAR_LISP! (c"comp-deferred-pending-h", Vcomp_deferred_pending_h,
    doc: "Hash table symbol-name -> function-value.\nFor internal use.");
  Vcomp_deferred_pending_h = CALLN! (Fmake_hash_table, QCtest (), Qeq ());

  DEFVAR_LISP! (c"comp-eln-to-el-h", Vcomp_eln_to_el_h,
    doc: "Hash table eln-filename -> el-filename.");
  Vcomp_eln_to_el_h = CALLN! (Fmake_hash_table, QCtest (), Qequal ());

  DEFVAR_LISP! (c"native-comp-eln-load-path", Vnative_comp_eln_load_path,
    doc: "List of directories to look for native-compiled *.eln files.\n\nThe *.eln files are actually looked for in a version-specific\nsubdirectory of each directory in this list.  That subdirectory\nis determined by the value of `comp-native-version-dir'.\nIf the name of a directory in this list is not absolute, it is\nassumed to be relative to `invocation-directory'.\nThe last directory of this list is assumed to be the one holding\nthe system *.eln files, which are the files produced when building\nEmacs.");

  /* Temporary value in use for bootstrap.  We can't do better as
     `invocation-directory' is still unset, will be fixed up during
     dump reload.  */
  Vnative_comp_eln_load_path = Fcons (build_string (c"../native-lisp/".as_ptr ()), Qnil ());

  DEFVAR_LISP! (c"native-comp-enable-subr-trampolines",
	       Vnative_comp_enable_subr_trampolines,
    doc: "If non-nil, enable generation of trampolines for calling primitives.\nTrampolines are needed so that Emacs respects redefinition or advice of\nprimitive functions when they are called from native-compiled Lisp code\nat `native-comp-speed' of 2.\n\nBy default, the value is t, and when Emacs sees a redefined or advised\nprimitive called from native-compiled Lisp, it generates a trampoline\nfor it on-the-fly.\n\nIf the value is a file name (a string), it specifies the directory in\nwhich to deposit the generated trampolines, overriding the directories\nin `native-comp-eln-load-path'.\n\nWhen this variable is nil, generation of trampolines is disabled.\n\nDisabling the generation of trampolines, when a trampoline for a redefined\nor advised primitive is not already available from previous compilations,\nmeans that such redefinition or advice will not have effect when calling\nprimitives from native-compiled Lisp code.  That is, calls to primitives\nwithout existing trampolines from native-compiled Lisp will behave as if\nthe primitive was called directly from C, and will ignore its redefinition\nand advice.");

  DEFVAR_LISP! (c"comp-installed-trampolines-h", Vcomp_installed_trampolines_h,
    doc: "Hash table subr-name -> installed trampoline.\nThis is used to prevent double trampoline instantiation, and also to\nprotect the trampolines against GC.");
  Vcomp_installed_trampolines_h = Fmake_hash_table (0, std::ptr::null_mut ());

  DEFVAR_LISP! (c"comp-no-native-file-h", V_comp_no_native_file_h,
    doc: "Files for which no deferred compilation should be performed.\nThese files' compilation should not be deferred because the bytecode\nversion was explicitly requested by the user during load.\nFor internal use.");
  V_comp_no_native_file_h = CALLN! (Fmake_hash_table, QCtest (), Qequal ());

  DEFVAR_BOOL! (c"comp-file-preloaded-p", comp_file_preloaded_p,
    doc: "When non-nil, assume the file being compiled to be preloaded.");

  DEFVAR_LISP! (c"comp-loaded-comp-units-h", Vcomp_loaded_comp_units_h,
    doc: "Hash table recording all loaded compilation units, file -> CU.");
  Vcomp_loaded_comp_units_h =
    CALLN! (Fmake_hash_table, QCweakness (), Qvalue (), QCtest (), Qequal ());

  DEFVAR_LISP! (c"comp-subr-arities-h", Vcomp_subr_arities_h,
    doc: "Hash table recording the arity of Lisp primitives.\nThis is in case they are redefined so the compiler still knows how to\ncompile calls to them.\nsubr-name -> arity\nFor internal use.");
  Vcomp_subr_arities_h = CALLN! (Fmake_hash_table, QCtest (), Qequal ());

  DEFVAR_BOOL! (c"comp-sanitizer-active", comp_sanitizer_active,
    doc: "If non-nil, enable runtime execution of native-compiler sanitizer.\nFor this to be effective, Lisp code must be compiled\nwith `comp-sanitizer-emit' non-nil.\nThis is intended to be used only for development and\nverification of the native compiler.");
  comp_sanitizer_active = false;

  DEFVAR_LISP! (c"comp--#$", Vcomp__hashdollar,
    doc: "Special value which will print as \"#$\".");
  Vcomp__hashdollar = build_string (c"#$".as_ptr ());

  Fprovide (intern_c_string (c"native-compile".as_ptr ()), Qnil ());
  }

  defsubr (&raw mut Snative_comp_available_p);
}
