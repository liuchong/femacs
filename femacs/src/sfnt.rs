// femacs: port of src/sfnt.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct field, macro, file-scope object, local), the same control flow and the same error model.
// The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers stay plain `unsafe fn`; C's `TEST_STATIC` (C 128/137) is spelled
// `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]`, which is what the macro's two arms
// select.
//
// **This file is P-86's largest unit (C 21,215 lines) and is being ported in slices.**  The
// slices so far are C 1-6306 plus slice 7's two out-of-order ranges and slice 8: slice 1
// (C 1-1112, through
// `sfnt_read_cmap_table`), slice 2 (the glyph lookup routines, the table readers and the two glyph
// outline readers, C 1113-2407), slice 3 (the glyph outline decomposition, C 2408-3594), slice 4
// (the outline builders and the "ragged" rasterizer, C 3595-4543), slice 5 (the "exact coverage
// scaler", C 4544-5451), slice 6 (the metrics, name and meta readers, the TrueType collection
// header and the table readers and fixed-point helpers the interpreter needs, C 5455-6306) and
// slice 7 (C 10225-10663, the graphics-state validation family, and C 13918-14554, the glyph
// variation readers - two ranges at their own C positions, chosen by PORTING.md **P-91 ①**'s
// dependency closure rather than by walking C's order, because Rust has no prototype for a
// `static` function: C 6316's `sfnt_init_graphics_state` calls `sfnt_validate_gs`, whose
// definition C 10538 had to land first), and slice 8 (C 14555-16447: the rest of the
// glyph-variation group - `sfnt_read_packed_deltas`, `sfnt_read_cvar_table`, the three blend
// helpers, the `gvar` glyph header record, `sfnt_compute_tuple_scale`, the three interpolators,
// the two appliers, `sfnt_vary_interpreter` and the `OS/2`/`post` readers - which is this file's
// **last non-TEST block**: C 16448's `#ifdef TEST` opens the test programme and runs to C 21215),
// and slice 9 (C 6312-6557: `sfnt_init_graphics_state`, `sfnt_make_interpreter`, the
// `sfnt_interpreter_run_context` enum and `sfnt_interpret_trap`; and C 8161-10224: the whole
// interpreter-helpers block - the zone addressing/storing/moving family, the projection and
// rounding helpers, `sfnt_interpret_utp` through `sfnt_interpret_else` - with the C 6559-8159
// macro region left to `sfnt_interpret_run`'s slice, above).
// **Slice 10 takes C 10664-11101** and **slice 11 C 11103-11395** - the interpreter's shift,
// interpolate and reference-point instructions: `sfnt_interpret_shz`, `sfnt_interpret_shc`,
// `sfnt_interpret_shp`, the four point-access macros with `IUP_SINGLE_PAIR`,
// `sfnt_interpret_iup_1`, `sfnt_interpret_iup`, `sfnt_interpret_mirp`,
// `sfnt_project_zp1_zp0_org` and `sfnt_interpret_mdrp`.
// **Slice 12 takes C 13370-13915** (the variation-selector and table-mapping block, out of
// order: the two UVS readers, the three comparators, `sfnt_create_uvs_context`/`sfnt_free_uvs_context`,
// `sfnt_variation_glyph_for_char`, `sfnt_is_character_default` and the mapping trio
// `sfnt_map_table`/`sfnt_unmap_table`/`sfnt_read_table`, the last three of which `src/sfntfont.rs`
// already calls).
// **Slice 13 takes C 12116-12295 and C 12495-12566** - the instructed-outline helpers
// (`sfnt_decompose_instructed_outline`, the `TEST_STATIC` builder `sfnt_build_instructed_outline`,
// `sfnt_compute_phantom_points`) and `sfnt_transform_f26dot6`, the 26.6 twin of
// `sfnt_transform_coordinates`.  The four functions between them (`sfnt_interpret_simple_glyph`,
// `sfnt_transform_f26dot6`'s three callers) wait for the interpreter.
// **Slice 14 takes C 11396-11983** - `sfnt_interpret_run` itself, C's dispatcher, *and* the
// C 6559-8159 instruction macros as its in-function copies (P-57: their bodies name this caller's
// `interpreter` and `is_prep`, and the eight that also jump to its own labels -
// `ELSE`/`JMPR`/`FDEF`/`IF`/`JROT`/`JROF`/`IDEF` to `skip_step`, `CALL` to `next_instruction` - are
// defined inside the labelled blocks that declare those labels, because a `macro_rules!` resolves
// a label at its *definition* site, measured).  C 11420's `switch (opcode)` is the port's `match`
// (146 arms in C's order plus C 11949's `default:`), and C's `break` - which leaves the switch -
// is the dispatch's `loop` (`next_instruction`/`skip_step` are the two labelled blocks around it).
// **Slice 15 takes C 11984-12115** - the two programme entries, `sfnt_interpret_font_program`
// (C 11995-12011) and `sfnt_interpret_control_value_program` (C 12023-12082): both are
// `TEST_STATIC` (this file's `cfg_attr`), both open with C's plain `setjmp` (the declaration
// stands at C 37's site above; on this target the C name `setjmp` is the symbol `nm -u` prints as
// `_setjmp`, re-measured), and the second carries C 12061-12077's graphics-state reversion block
// whole.  This is also where C 9832's `sfnt_interpret_call` call is satisfied end to end: slice 14
// landed the `sfnt_interpret_run` its `E0425` named, and this slice lands its two callers.
// **What is left after slice 15** is C 12305-12494's `sfnt_interpret_simple_glyph`, C 12591-13369's
// three compound-glyph interpreters, and C 16448-21215, the TEST programme (the ledger's Owed 21).
// The ledger
// `docs/ports/sfnt.md` carries the C line-range table for the rows (up to C 21215), the
// `## Evidence` readings and the `## Deviations`.
//
// **Slice 9 takes C 6312-6557 and C 8161-10224** - the two halves of the C 6307-10224 block that
// stand *outside* C 6559-8159 - and leaves that middle region to the lane that ports
// `sfnt_interpret_run` (C 11396), for the reason P-57 gives: C's ~130 interpreter-instruction
// macros are file scope in C but their bodies name the *caller's* `interpreter` (and `is_prep`,
// and `sfnt_interpret_run`'s own labels - `ELSE`'s `goto skip_step`, `CALL`'s
// `goto next_instruction`, the six `goto deltac1_start`-family jumps of the DELTA instructions),
// which a `macro_rules!` resolves at its **definition** site.  The tree's answer (PORTING.md
// P-57, `src/coding.rs` / `src/ccl.rs`) is one in-function copy per using function, and the only
// four of those macros the two ranges here *use* are `TRAP` (C 6562), `POP` (C 6597, C's live
// `#ifndef TEST` arm), `PROJECT` (C 7777) and `DUAL_PROJECT` (C 7780) - each is defined inside
// every function that calls it, and every call site keeps C's own spelling.  The rest of
// C 6559-8159 is slice 14's in-function copies inside `sfnt_interpret_run`, at that position.
//
// The frozen configuration compiles no `sfnt.o`: `configure.ac` 2645 adds `sfnt.o` to
// `ANDROID_OBJ` only inside the `XCONFIGURE = android` branch (the `REALLY_ANDROID` one,
// 2620-2654) - `android.o` is added one branch earlier, at 2598 - and
// `docs/baseline/frozen-build-sources.txt` lists no `sfnt.o`.  Unlike `src/android.c`, though,
// this file **is** in `docs/baseline/config/cond-eval-verified.txt` (353), so `extract-abi.py`
// reads its `#if`s and its inventory is live: the tool clears it only for a port with no `fn` at
// all (`tools/extract-abi.py` 2225-2227), which is why the 11-line placeholder read `OK` and this
// slice does not.  `--check` is therefore *not* vacuous here, and its reading (the ledger's
// Evidence ③) counts out what the remaining slices owe.  P-86 ① still governs the port:
// transcription, both arms of every `#if`, every item behind the file gate, and the ledger records
// P-86 ②'s C line-range table.  The gates, in C's order:
//
//   * the file gate (`HAVE_ANDROID && !defined ANDROID_STUBIFY`, i.e. `REALLY_ANDROID`) is
//     `all (feature = "have_android", feature = "not_android_stubify")` - on every item and on
//     C 22-39's include graph, which is why the frozen configuration compiles nothing here.
//     (`androidselect.rs`'s gate is the same pair, from 2647-2648; `androidmenu.rs`'s is
//     `have_android` alone because 2597 lists it in both Android branches.)
//   * C 49's `#ifdef TEST` is `feature = "test"`: this slice carries *both* arms - the file's own
//     `xmalloc` family, `TEST_STATIC`, `eassert` and `AVOID` (C 51-134), and C 136-138's `#else`
//     arm with `#include "lisp.h"`.
//   * C 41-43's `#ifdef HAVE_MMAP` is live on this target (`HAVE_MMAP` is 1 in
//     docs/baseline/config/darwin-arm64.macros.tsv 1469), so `<sys/mman.h>` is included ungated;
//     the name has no mirror here, and this slice is the first to use what it provides, so the
//     unit carries C's four prototypes and the five constants (the P-88 ① use-site block above
//     C 49).  C 1827's `#if defined HAVE_MMAP && !defined TEST` therefore reduces to C's
//     `TEST` half alone - no `have_mmap` feature exists in docs/baseline/feature-coverage.tsv,
//     and slice 1 already recorded that ruling for C 41 (the ledger's Owed 4).
//   * C 1871-1876's `#ifdef HAVE_POSIX_MADVISE` is 1 in the frozen configuration
//     (docs/baseline/config/darwin-arm64.macros.tsv 1492), so the `posix_madvise` arm is live;
//     `#elif defined HAVE_MADVISE` (also 1, 1445) is carried under the registered
//     `no_have_posix_madvise` feature, the `pdumper.rs` 5281-5284 shape.
//   * C 45-47's `#if defined __GNUC__ && !defined __clang__` guards `#pragma GCC diagnostic
//     ignored "-Wstringop-overflow"`; there is no Rust counterpart and it is not carried.
//   * C 224/232's `#ifndef WORDS_BIGENDIAN` is `not (feature = "words_bigendian")`; the other arm
//     of each swap helper is C's empty body - the big-endian byte order needs no swap.
//   * C 4322's `#ifndef NDEBUG` (slice 4: `sfnt_fill_span`'s `row_end` and its four `eassert`s)
//     is `feature = "no_ndebug"`: the frozen configuration *defines* `NDEBUG`, so this is dead
//     text and the feature names the non-frozen side (P-51's `no_signbit`/`no_rint` rule, the
//     ledger's Deviation 24).  The name is not registered yet - the ledger's Owed 8.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 20: `#include <config.h>`

/* C 22-39: the file's own include list, in C's order.  It sits outside C 49's `#ifdef TEST`, so
   the whole graph is on the file gate alone - no translation unit the frozen configuration builds
   reaches any of it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/sfnt.rs");                          // C 22: `#include "sfnt.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/attribute.rs");               // C 24: `#include <attribute.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/byteswap.in.rs");             // C 25: `#include <byteswap.h>` (bswap_16/32)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/fcntl.in.rs");                // C 26: `#include <fcntl.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/intprops.rs");                // C 27: `#include <intprops.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/inttypes.in.rs");             // C 28: `#include <inttypes.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdbit.in.rs");               // C 29: `#include <stdbit.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdckdint.in.rs");            // C 30: `#include <stdckdint.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdcountof.rs");              // C 31: `#include <stdcountof.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdint.in.rs");               // C 32: `#include <stdint.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdio.rs");                   // C 33: `#include <stdio.h>` (SEEK_SET)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdlib.rs");                  // C 34: `#include <stdlib.h>` (malloc, abort)

/* `<stdlib.h>`: C's one use in this slice is C 2451's `abort` (`sfnt_read_glyph`'s
   neither-`loca` arm).  `lib/include/stdlib.rs` carries `malloc` and friends but not
   `abort` (the `androidvfs.rs` 228-234 reading of the same header).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `_Noreturn void abort (void);`
  pub fn abort () -> !;
}
/* `<stdlib.h>`: slice 4 is the first text here to call `abs` (`sfnt_multiply_divide_signed`, C
   3712-3713, `sfnt_curve_is_flat`, C 3812, and `sfnt_build_outline_edges`, C 4134), and neither
   `lib/include/stdlib.rs` nor any other mirror in this unit's include graph carries it, so the
   prototype stands here beside `abort` (the same P-88 ① reading of C 34).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `int abs (int);`
  pub fn abs (i: c_int) -> c_int;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/string.rs");                  // C 35: `#include <string.h>` (memcpy)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/unistd.in.rs");               // C 36: `#include <unistd.h>` (read, lseek)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/sys_types.in.rs");            // unistd.h 40: `#include <sys/types.h>` (off_t)
/* C 37's `#include <setjmp.h>`: this tree has no setjmp mirror, and the member it exists for
   (`struct sfnt_interpreter`'s `trap`, sfnt.h 2010) is lisp.rs 1058's 192-byte array type.
   C 1-1112 names neither; C 6556's `longjmp` is this tree's first *plain* one (image.c's two go
   through C's own `sys_longjmp` spelling) and C 11999's `setjmp` is the first of that name, so
   both stand here with C's own prototypes (P-88 ①).  **Measured, not assumed**: a three-line
   programme calling `setjmp`/`longjmp` on this target links `_setjmp`/`_longjmp` in `nm -u`'s
   notation - which is Mach-O's leading underscore for the C names `setjmp`/`longjmp`, i.e. the
   two plain functions and *not* the `sigsetjmp` pair (`3.702` re-measured both names; the
   earlier reading of this paragraph stands) - and `sizeof (jmp_buf)` prints **192**, which is
   the array width lisp.rs 1068 declares.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `<setjmp.h>`'s `void longjmp (jmp_buf, int) __attribute__((noreturn));`
  pub fn longjmp (env: *mut jmp_buf, val: c_int) -> !;
  /// C: `<setjmp.h>`'s `extern int setjmp (jmp_buf);` - the declaration C 11999 binds.
  pub fn setjmp (env: *mut jmp_buf) -> c_int;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/errno.in.rs");                // C 38: `#include <errno.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/alloca.in.rs");               // C 39: `#include <alloca.h>`
/* C 41-43: `#ifdef HAVE_MMAP` / `#include <sys/mman.h>` - live on this target (`HAVE_MMAP` is 1
   in docs/baseline/config/darwin-arm64.macros.tsv 1469), and this slice is the first to name
   what the include provides (`sfnt_map_glyf_table`/`sfnt_unmap_glyf_table`, C 1827-1906).  The
   ref tree has no `sys/mman.h`, so the names stand here with C's own prototypes (the P-88 ①
   shape; `pdumper.rs` 102-166 is the same class of use-site block for `<sys/mman.h>`,
   `<stdlib.h>`'s `qsort` and `<string.h>`'s `strcat`).  The four constants are the values
   `pdumper.rs` measured with `clang -E -dM -include sys/mman.h` on the frozen target.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `<sys/mman.h>`: `void *mmap (void *, size_t, int, int, int, off_t);`
  pub fn mmap (addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int,
	       offset: off_t) -> *mut c_void;
  /// C: `<sys/mman.h>`: `int munmap (void *, size_t);`
  pub fn munmap (addr: *mut c_void, len: size_t) -> c_int;
  /// C: `<sys/mman.h>`: `int posix_madvise (void *, size_t, int);`
  pub fn posix_madvise (addr: *mut c_void, len: size_t, advice: c_int) -> c_int;
  /// C: `<sys/mman.h>`: `int madvise (void *, size_t, int);`
  pub fn madvise (addr: *mut c_void, len: size_t, advice: c_int) -> c_int;
}
/// C: `<sys/mman.h>`: `#define PROT_READ 0x1`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const PROT_READ: c_int = 0x01;
/// C: `<sys/mman.h>`: `#define MAP_PRIVATE 0x0002`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const MAP_PRIVATE: c_int = 0x0002;
/// C: `<sys/mman.h>`: `#define MAP_FAILED ((void *) -1)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;
/// C: `<sys/mman.h>`: `#define POSIX_MADV_WILLNEED 3`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const POSIX_MADV_WILLNEED: c_int = 3;
/// C: `<sys/mman.h>`: `#define MADV_WILLNEED 3`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const MADV_WILLNEED: c_int = 3;

/* C 138's `#include "lisp.h"` reaches `thread.h` 29-31's `<time.h>` arm (dead: inside
   `#ifdef MSDOS`) and `thread.h` 40's `sysselect.h`, so the *live* prototypes those mirrors
   carry name `time_t` and `struct timespec`; C gets both from the system `<sys/select.h>`/
   `<sys/time.h>` (measured: `<sys/select.h>` alone compiles a `time_t` declaration on this
   target), and the port takes them from the `<time.h>` mirror (P-56's class, the
   `intervals.rs` 80 and `sys_select.rs` 28 ordering).  This edge has no C line of its own -
   C's own `<time.h>` (C 165) is inside `#ifdef TEST` - so it is written here, at the include
   that needs it; the `test` arm's copy then collapses into the materializer's dedupe marker.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/time.in.rs");               // lisp.h -> thread.h 29/40: `time_t`,
                                                     // `struct timespec`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* C 49-56: `#ifdef TEST`, the file's own test harness.  Both arms are carried.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
include!("../lib/include/assert.in.rs");               // C 51: `#include <assert.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
include!("../lib/include/time.in.rs");                 // C 52: `#include <time.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
include!("../lib/include/timespec.rs");                // C 53: `#include <timespec.h>`
/* C 54's `#include <sys/wait.h>`, C 57-58's `<X11/Xlib.h>` and
   `<X11/extensions/Xrender.h>`: no mirror in this tree.  The TEST arm's X11 harness is in the
   remainder (`sfnt_test_*`), and C 1-1112 names no Xlib type.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
unsafe fn memory_full_up ()                           // C 58-64
{
  abort ();
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
unsafe fn xmalloc (size: size_t) -> *mut c_void       // C 66-77
{
  let mut ptr: *mut c_void;

  ptr = malloc (size);

  if ptr.is_null ()
    { memory_full_up (); }

  return ptr;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
unsafe fn xcalloc (n: size_t, s: size_t) -> *mut c_void   // C 79-90
{
  let mut ptr: *mut c_void;

  ptr = calloc (n, s);

  if ptr.is_null ()
    { memory_full_up (); }

  return ptr;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
unsafe fn xzalloc (size: size_t) -> *mut c_void       // C 92-96
{
  return xcalloc (1, size);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
unsafe fn xnmalloc (n: size_t, s: size_t) -> *mut c_void   // C 98-105
{
  let mut size: size_t;
  let (ckd_size, ckd_overflow) = n.overflowing_mul (s);
  size = ckd_size;
  if ckd_overflow
    { memory_full_up (); }
  return xmalloc (size);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
unsafe fn xrealloc (ptr: *mut c_void, size: size_t) -> *mut c_void   // C 107-118
{
  let mut new_ptr: *mut c_void;

  new_ptr = realloc (ptr, size);

  if new_ptr.is_null ()
    { memory_full_up (); }

  return new_ptr;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
unsafe fn xfree (ptr: *mut c_void)                    // C 120-124
{
  free (ptr);
}

/* C 126-128: `#define TEST_STATIC static` - `static` in the TEST arm, empty in the other one.
   Which arm is live decides whether the two readers this slice defines for other units
   (`sfnt_read_table_directory`, C 248-249, and `sfnt_read_cmap_table`, C 1019-1020) are internal
   or external, and the port spells exactly that bit with
   `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]`.  */

/* C 130-131: `#define eassert(expr) assert (expr)` - the TEST arm's own spelling of lisp.h's
   `eassert` (lisp.rs 387's `macro_rules!`), which C 136-138's arm reaches through `lisp.h`.
   C 1-1112 names `eassert` nowhere.  */

/* C 133-134: `#define AVOID _Noreturn ATTRIBUTE_COLD void` - the TEST arm's spelling of the
   return type of `memory_full_up` and friends; the port writes `-> !`
   (`docs/ports/excluded-macros.txt`'s `AVOID` row, also lisp.rs 4116).  C 1-1112 names `AVOID`
   nowhere.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify",
            not (feature = "test")))]
include!("include/lisp.rs");                          // C 138: `#include "lisp.h"`

/* C 141-148.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn xaddmalloc (base: size_t, additional: size_t) -> *mut c_void
{
  let mut size: size_t;
  let (ckd_size, ckd_overflow) = additional.overflowing_add (base);
  size = ckd_size;
  if ckd_overflow
    { memory_full_up (); }
  return xmalloc (size);
}

/* C 150-157.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn xaddrealloc (ptr: *mut c_void, base: size_t, additional: size_t)
  -> *mut c_void
{
  let mut size: size_t;
  let (ckd_size, ckd_overflow) = additional.overflowing_add (base);
  size = ckd_size;
  if ckd_overflow
    { memory_full_up (); }
  return xrealloc (ptr, size);
}

/* C 159-160: sfnt.c's own `MIN`/`MAX`, whose bodies are `minmax.h`'s.  The port's macro_rules
   keep both of C's evaluations of each argument (minmax.rs 12/15's note).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! MIN { ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } }; }
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! MAX { ($a:expr, $b:expr) => { if ($a) > ($b) { $a } else { $b } }; }

/* This file provides generic support for reading most TrueType fonts,
   and some OpenType fonts with TrueType outlines, along with glyph
   lookup, outline decomposition, and alpha mask generation from those
   glyphs.  It is intended to be used on any platform where proper
   libraries such as FreeType are not easily available, and the native
   font library is too limited for Emacs to support properly.

   Unlike most popular libraries for handling fonts, no ``font'' or
   ``face'' type is provided.  Instead, routines and structure
   definitions for accessing and making use of individual tables in a
   font file are exported, which allows for flexibility in the rest of
   Emacs.

   Try not to keep this file too dependent on Emacs.  Everything Lisp
   related goes in sfntfont.c.  The author wants to keep using it for
   some other (free) software.

   The source of reference is the TrueType Reference Manual, published
   by Apple Computer, which is currently found at:

     https://developer.apple.com/fonts/TrueType-Reference-Manual/

   Apple's TrueType implementation is notably missing features
   provided by Microsoft's extended OpenType scaler, such as the two
   additional phantom points on the Y axis, and also behaves
   differently, especially when it comes to considering phantom points
   as anchors in compound glyphs.

   As a result, do not expect this scaler to work well with Microsoft
   fonts such as Arial.  */

/* Mapping between sfnt table names and their identifiers.  */

/* C 197-219: `static uint32_t sfnt_table_names[] = { [SFNT_TABLE_CMAP] = … }` - every one of the
   enum's 18 values is designated, so the array C's initialiser produces has 18 elements and no
   implicit zero.  Rust has no designated initialiser, so the port fills the same 18 slots by index
   in C's order (keyboard.rs 5607's `lispy_function_keys` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_table_names: [uint32_t; 18] = [
  0x636d6170,/* [SFNT_TABLE_CMAP] = 0x636d6170 */
  0x676c7966,/* [SFNT_TABLE_GLYF] = 0x676c7966 */
  0x68656164,/* [SFNT_TABLE_HEAD] = 0x68656164 */
  0x68686561,/* [SFNT_TABLE_HHEA] = 0x68686561 */
  0x686d7478,/* [SFNT_TABLE_HMTX] = 0x686d7478 */
  0x6c6f6361,/* [SFNT_TABLE_LOCA] = 0x6c6f6361 */
  0x6d617870,/* [SFNT_TABLE_MAXP] = 0x6d617870 */
  0x6e616d65,/* [SFNT_TABLE_NAME] = 0x6e616d65 */
  0x6d657461,/* [SFNT_TABLE_META] = 0x6d657461 */
  0x63767420,/* [SFNT_TABLE_CVT] = 0x63767420 */
  0x6670676d,/* [SFNT_TABLE_FPGM] = 0x6670676d */
  0x70726570,/* [SFNT_TABLE_PREP] = 0x70726570 */
  0x66766172,/* [SFNT_TABLE_FVAR] = 0x66766172 */
  0x67766172,/* [SFNT_TABLE_GVAR] = 0x67766172 */
  0x63766172,/* [SFNT_TABLE_CVAR] = 0x63766172 */
  0x61766172,/* [SFNT_TABLE_AVAR] = 0x61766172 */
  0x4f532f32,/* [SFNT_TABLE_OS_2] = 0x4f532f32 */
  0x706f7374,/* [SFNT_TABLE_POST] = 0x706f7374 */
];


/* Swap values from TrueType to system byte order.  */

/* C 221-226.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_swap16_1 (value: *mut uint16_t)
{
  #[cfg (not (feature = "words_bigendian"))]
  { *value = bswap_16 (*value); }
}

/* C 229-234.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_swap32_1 (value: *mut uint32_t)
{
  #[cfg (not (feature = "words_bigendian"))]
  { *value = bswap_32 (*value); }
}

/* C 237-238: `#define sfnt_swap16(what) (sfnt_swap16_1 ((uint16_t *) (what)))` and its 32-bit
   twin.  C's cast is a no-op at most call sites - the argument is already a `uint16_t *` - and is
   what widens `&format4->id_delta[i]` (an `int16_t *`) at the others; Rust needs the pointee cast
   spelled in two steps (`&mut i16 as *mut i16 as *mut u16`), so the port's macro keeps C's one
   cast as `as *mut _ as *mut uint16_t`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! sfnt_swap16 { ($what:expr) => { sfnt_swap16_1 (($what) as *mut _ as *mut uint16_t) }; }
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! sfnt_swap32 { ($what:expr) => { sfnt_swap32_1 (($what) as *mut _ as *mut uint32_t) }; }

/* Read the table directory from the file FD.  FD must currently be at
   the start of the file (or an offset defined in the TTC header, if
   applicable), and must be seekable.  Return the table directory upon
   success, else NULL.

   Value is NULL upon failure, and the offset subtable upon success.
   If FD is actually a TrueType collection file, value is -1.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_table_directory (fd: c_int)
  -> *mut sfnt_offset_subtable                              // C 249-323
{
  let mut subtable: *mut sfnt_offset_subtable;
  let mut rc: ssize_t;
  let mut offset: size_t;
  let mut subtable_size: size_t;
  let mut i: c_int;

  subtable = xmalloc (std::mem::size_of::<sfnt_offset_subtable> ())
    as *mut sfnt_offset_subtable;
  offset = SFNT_ENDOF! (sfnt_offset_subtable, range_shift, uint16_t);
  rc = read (fd, subtable as *mut c_void, offset);

  if rc == -1 || (rc as size_t) < offset
    {
      if rc != -1 && (rc as size_t) >= std::mem::size_of::<uint32_t> ()
	{
	  /* Detect a TTC file.  In that case, the first long will be
	     ``ttcf''.  */
	  sfnt_swap32! (&mut (*subtable).scaler_type);

	  if (*subtable).scaler_type == SFNT_TTC_TTCF as uint32_t
	    {
	      xfree (subtable as *mut c_void);
	      return (usize::MAX as *mut sfnt_offset_subtable);
	    }
	}

      xfree (subtable as *mut c_void);
      return std::ptr::null_mut ();
    }

  sfnt_swap32! (&mut (*subtable).scaler_type);

  /* Bail out early if this font is actually a TrueType collection
     file.  */

  if (*subtable).scaler_type == SFNT_TTC_TTCF as uint32_t
    {
      xfree (subtable as *mut c_void);
      return (usize::MAX as *mut sfnt_offset_subtable);
    }

  sfnt_swap16! (&mut (*subtable).num_tables);
  sfnt_swap16! (&mut (*subtable).search_range);
  sfnt_swap16! (&mut (*subtable).entry_selector);
  sfnt_swap16! (&mut (*subtable).range_shift);

  /* Figure out how many more tables have to be read, and read each
     one of them.  */
  subtable_size = ((*subtable).num_tables as size_t
		   * std::mem::size_of::<sfnt_table_directory> ());
  subtable = xrealloc (subtable as *mut c_void,
		       std::mem::size_of::<sfnt_offset_subtable> () + subtable_size)
    as *mut sfnt_offset_subtable;
  (*subtable).subtables = subtable.offset (1) as *mut sfnt_table_directory;

  rc = read (fd, (*subtable).subtables as *mut c_void, subtable_size);

  if rc == -1 || (rc as size_t) < subtable_size
    {
      xfree (subtable as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap each of the subtables.  */

  i = 0;
  while i < (*subtable).num_tables as c_int
    {
      sfnt_swap32! (&mut (*((*subtable).subtables).offset (i as isize)).tag);
      sfnt_swap32! (&mut (*((*subtable).subtables).offset (i as isize)).checksum);
      sfnt_swap32! (&mut (*((*subtable).subtables).offset (i as isize)).offset);
      sfnt_swap32! (&mut (*((*subtable).subtables).offset (i as isize)).length);
      i += 1;
    }

  return subtable;
}

/* Return a pointer to the table directory entry for TABLE in
   SUBTABLE, or NULL if it was not found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_find_table (subtable: *mut sfnt_offset_subtable, table: sfnt_table)
  -> *mut sfnt_table_directory                             // C 328-341
{
  let mut i: c_int;

  i = 0;
  while i < (*subtable).num_tables as c_int
    {
      if (*((*subtable).subtables).offset (i as isize)).tag
	== sfnt_table_names[table as usize]
	{ return &mut (*((*subtable).subtables).offset (i as isize)); }
      i += 1;
    }

  return std::ptr::null_mut ();
}

/* Character mapping routines.  */

/* Read a format 0 cmap subtable from FD.  HEADER has already been
   read.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_cmap_format_0 (fd: c_int,
				  header: *mut sfnt_cmap_encoding_subtable_data)
  -> *mut sfnt_cmap_format_0                              // C 350-379
{
  let mut format0: *mut sfnt_cmap_format_0;
  let mut rc: ssize_t;
  let mut wanted_size: size_t;

  format0 = xmalloc (std::mem::size_of::<sfnt_cmap_format_0> ())
    as *mut sfnt_cmap_format_0;

  /* Fill in fields that have already been read.  */
  (*format0).format = (*header).format;
  (*format0).length = (*header).length;

  /* Read the rest.  */
  wanted_size = (std::mem::size_of::<sfnt_cmap_format_0> ()
		 - std::mem::offset_of! (sfnt_cmap_format_0, language));
  rc = read (fd, &mut (*format0).language as *mut uint16_t as *mut c_void,
	     wanted_size);

  if rc == -1 || (rc as size_t) < wanted_size
    {
      xfree (format0 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_0);
    }

  /* Swap fields and return.  */
  sfnt_swap16! (&mut (*format0).language);
  return format0;
}

/* Read a format 2 cmap subtable from FD.  HEADER has already been
   read.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_cmap_format_2 (fd: c_int,
				  header: *mut sfnt_cmap_encoding_subtable_data)
  -> *mut sfnt_cmap_format_2                              // C 384-483
{
  let mut format2: *mut sfnt_cmap_format_2;
  let mut rc: ssize_t;
  let mut min_bytes: size_t;
  let mut i: c_int;
  let mut nsub: c_int;

  /* Reject contents that are too small.  */
  min_bytes = SFNT_ENDOF! (sfnt_cmap_format_2, sub_header_keys, [uint16_t; 256]);
  if ((*header).length as size_t) < min_bytes
    { return std::ptr::null_mut (); }

  /* Add enough bytes at the end to fit the two variable length
     pointers.  */
  format2 = xmalloc ((*header).length as size_t
		     + std::mem::size_of::<sfnt_cmap_format_2> ())
    as *mut sfnt_cmap_format_2;
  (*format2).format = (*header).format;
  (*format2).length = (*header).length;

  /* Read the part before the variable length data.  */
  min_bytes -= std::mem::offset_of! (sfnt_cmap_format_2, language);
  rc = read (fd, &mut (*format2).language as *mut uint16_t as *mut c_void,
	     min_bytes);
  if rc == -1 || (rc as size_t) < min_bytes
    {
      xfree (format2 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_2);
    }

  /* Swap the fields now.  */

  sfnt_swap16! (&mut (*format2).language);

  /* At the same time, look for the largest value in sub_header_keys.
     That will be the number of subheaders and elements in the glyph
     index array.  */

  nsub = 0;

  i = 0;
  while i < 256
    {
      /* Values in sub_header_keys are actually offsets from the end of
	 that array.  Since the language of the spec is such as to imply
	 that they must be divisible by eight, divide them by the
	 same.  */
      sfnt_swap16! (&mut (*format2).sub_header_keys[i as usize]);

      if (*format2).sub_header_keys[i as usize] as c_int > nsub
	{ nsub = (*format2).sub_header_keys[i as usize] as c_int / 8; }
      i += 1;
    }

  /* There always exists a subheader at index zero.  */
  nsub += 1;

  /* Otherwise, read the rest of the variable length data to the end
     of format2.  */
  min_bytes = ((*format2).length as size_t
	       - SFNT_ENDOF! (sfnt_cmap_format_2, sub_header_keys, [uint16_t; 256]));
  rc = read (fd, format2.offset (1) as *mut c_void, min_bytes);
  if rc == -1 || (rc as size_t) < min_bytes
    {
      xfree (format2 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_2);
    }

  /* Check whether or not the data is of the correct size.  */
  if min_bytes < (nsub as size_t
		  * std::mem::size_of::<sfnt_cmap_format_2_subheader> ())
    {
      xfree (format2 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_2);
    }

  /* Point the data pointers to the right location, swap everything,
     and return.  */

  (*format2).subheaders = format2.offset (1) as *mut sfnt_cmap_format_2_subheader;
  (*format2).glyph_index_array
    = (*format2).subheaders.offset (nsub as isize) as *mut uint16_t;

  i = 0;
  while i < nsub
    {
      sfnt_swap16! (&mut (*((*format2).subheaders).offset (i as isize)).first_code);
      sfnt_swap16! (&mut (*((*format2).subheaders).offset (i as isize)).entry_count);
      sfnt_swap16! (&mut (*((*format2).subheaders).offset (i as isize)).id_delta);
      sfnt_swap16! (&mut (*((*format2).subheaders).offset (i as isize)).id_range_offset);
      i += 1;
    }

  /* Figure out how big the glyph index array is, and swap everything
     there.  */
  (*format2).num_glyphs
    = ((min_bytes - (nsub as size_t
		     * std::mem::size_of::<sfnt_cmap_format_2_subheader> ())) / 2)
    as uint16_t;

  i = 0;
  while i < (*format2).num_glyphs as c_int
    {
      sfnt_swap16! (&mut (*((*format2).glyph_index_array).offset (i as isize)));
      i += 1;
    }

  return format2;
}

/* Read a format 4 cmap subtable from FD.  HEADER has already been
   read.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_cmap_format_4 (fd: c_int,
				  header: *mut sfnt_cmap_encoding_subtable_data)
  -> *mut sfnt_cmap_format_4                              // C 488-589
{
  let mut format4: *mut sfnt_cmap_format_4;
  let mut min_bytes: size_t;
  let mut variable_size: size_t;
  let mut rc: ssize_t;
  let mut bytes_minus_format4: size_t;
  let mut seg_count: c_int;
  let mut i: c_int;

  min_bytes = SFNT_ENDOF! (sfnt_cmap_format_4, range_shift, uint16_t);

  /* Check that the length is at least min_bytes.  */
  if ((*header).length as size_t) < min_bytes
    { return std::ptr::null_mut (); }

  /* Allocate the format4 buffer, making it the size of the buffer
     itself plus that of the data.  */
  format4 = xmalloc ((*header).length as size_t
		     + std::mem::size_of::<sfnt_cmap_format_4> ())
    as *mut sfnt_cmap_format_4;

  /* Copy over fields that have already been read.  */
  (*format4).format = (*header).format;
  (*format4).length = (*header).length;

  /* Read the initial data.  */
  min_bytes -= std::mem::offset_of! (sfnt_cmap_format_4, language);
  rc = read (fd, &mut (*format4).language as *mut uint16_t as *mut c_void,
	     min_bytes);
  if rc == -1 || (rc as size_t) < min_bytes
    {
      xfree (format4 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_4);
    }

  /* Swap fields that have been read.  */
  sfnt_swap16! (&mut (*format4).language);
  sfnt_swap16! (&mut (*format4).seg_count_x2);
  sfnt_swap16! (&mut (*format4).search_range);
  sfnt_swap16! (&mut (*format4).entry_selector);
  sfnt_swap16! (&mut (*format4).range_shift);

  /* Get the number of segments to read.  */
  seg_count = (*format4).seg_count_x2 as c_int / 2;

  /* Now calculate whether or not the size is sufficiently large.  */
  bytes_minus_format4
    = (*format4).length as size_t
    - SFNT_ENDOF! (sfnt_cmap_format_4, range_shift, uint16_t);
  variable_size = (seg_count as size_t * std::mem::size_of::<uint16_t> ()
		   + std::mem::size_of::<uint16_t> ()
		   + seg_count as size_t * std::mem::size_of::<uint16_t> ()
		   + seg_count as size_t * std::mem::size_of::<int16_t> ()
		   + seg_count as size_t * std::mem::size_of::<int16_t> ());

  if bytes_minus_format4 < variable_size
    {
      /* Not enough bytes to fit the entire implied table
	 contents.  */
      xfree (format4 as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Read the rest of the bytes to the end of format4.  */
  rc = read (fd, format4.offset (1) as *mut c_void, bytes_minus_format4);
  if rc == -1 || (rc as size_t) < bytes_minus_format4
    {
      xfree (format4 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_4);
    }

  /* Set data pointers to the right locations.  */
  (*format4).end_code = format4.offset (1) as *mut uint16_t;
  (*format4).reserved_pad = (*format4).end_code.offset (seg_count as isize);
  (*format4).start_code = (*format4).reserved_pad.offset (1);
  (*format4).id_delta
    = (*format4).start_code.offset (seg_count as isize) as *mut int16_t;
  (*format4).id_range_offset = (*format4).id_delta.offset (seg_count as isize);
  (*format4).glyph_index_array
    = (*format4).id_range_offset.offset (seg_count as isize) as *mut uint16_t;

  /* N.B. that the number of elements in glyph_index_array is
     (bytes_minus_format4 - variable_size) / 2.  Swap all the
     data.  */

  sfnt_swap16! ((*format4).reserved_pad);

  i = 0;
  while i < seg_count
    {
      sfnt_swap16! (&mut (*((*format4).end_code).offset (i as isize)));
      sfnt_swap16! (&mut (*((*format4).start_code).offset (i as isize)));
      sfnt_swap16! (&mut (*((*format4).id_delta).offset (i as isize)));
      sfnt_swap16! (&mut (*((*format4).id_range_offset).offset (i as isize)));
      i += 1;
    }

  (*format4).glyph_index_size
    = (bytes_minus_format4 - variable_size) / 2;

  i = 0;
  while i < (*format4).glyph_index_size as c_int
    {
      sfnt_swap16! (&mut (*((*format4).glyph_index_array).offset (i as isize)));
      i += 1;
    }

  /* Done.  Return the format 4 character map.  */
  return format4;
}

/* Read a format 6 cmap subtable from FD.  HEADER has already been
   read.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_cmap_format_6 (fd: c_int,
				  header: *mut sfnt_cmap_encoding_subtable_data)
  -> *mut sfnt_cmap_format_6                              // C 594-659
{
  let mut format6: *mut sfnt_cmap_format_6;
  let mut min_size: size_t;
  let mut rc: ssize_t;
  let mut i: uint16_t;

  min_size = SFNT_ENDOF! (sfnt_cmap_format_6, entry_count, uint16_t);

  /* See if header->length is big enough.  */
  if ((*header).length as size_t) < min_size
    { return std::ptr::null_mut (); }

  /* Allocate the buffer to hold header->size and enough for at least
     the glyph index array pointer.  */
  format6 = xmalloc ((*header).length as size_t
		     + std::mem::size_of::<sfnt_cmap_format_6> ())
    as *mut sfnt_cmap_format_6;

  /* Fill in data that has already been read.  */
  (*format6).format = (*header).format;
  (*format6).length = (*header).length;

  /* Read the fixed size data.  */
  min_size -= std::mem::offset_of! (sfnt_cmap_format_6, language);
  rc = read (fd, &mut (*format6).language as *mut uint16_t as *mut c_void,
	     min_size);
  if rc == -1 || (rc as size_t) < min_size
    {
      xfree (format6 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_6);
    }

  /* Swap what was read.  */
  sfnt_swap16! (&mut (*format6).language);
  sfnt_swap16! (&mut (*format6).first_code);
  sfnt_swap16! (&mut (*format6).entry_count);

  /* Figure out whether or not header->length is sufficient to hold
     the variable length data.  */
  if ((*header).length as size_t)
      < (*format6).entry_count as size_t * std::mem::size_of::<uint16_t> ()
    {
      xfree (format6 as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Read the variable length data.  */
  rc = read (fd, format6.offset (1) as *mut c_void,
	     ((*format6).entry_count as size_t * std::mem::size_of::<uint16_t> ()));
  if rc == -1 || ((rc as size_t) < ((*format6).entry_count as size_t
				    * std::mem::size_of::<uint16_t> ()))
    {
      xfree (format6 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_6);
    }

  /* Set the data pointer and swap everything.  */
  (*format6).glyph_index_array = format6.offset (1) as *mut uint16_t;
  i = 0;
  while i < (*format6).entry_count
    {
      sfnt_swap16! (&mut (*((*format6).glyph_index_array).offset (i as isize)));
      i += 1;
    }

  /* All done! */
  return format6;
}

/* Read a format 8 cmap subtable from FD.  HEADER has already been
   read.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_cmap_format_8 (fd: c_int,
				  header: *mut sfnt_cmap_encoding_subtable_data)
  -> *mut sfnt_cmap_format_8                              // C 664-750
{
  let mut format8: *mut sfnt_cmap_format_8;
  let mut min_size: size_t;
  let mut temp: size_t;
  let mut rc: ssize_t;
  let mut length: uint32_t = 0;
  let mut i: uint32_t;

  /* Read the 32-bit length field.  */
  if (read (fd, &mut length as *mut uint32_t as *mut c_void,
	    std::mem::size_of::<uint32_t> ())
      < std::mem::size_of::<uint32_t> () as ssize_t)
    { return (usize::MAX as *mut sfnt_cmap_format_8); }

  /* Swap the 32-bit length field.  */
  sfnt_swap32! (&mut length);

  min_size = SFNT_ENDOF! (sfnt_cmap_format_8, num_groups, uint32_t);

  /* Make sure the header is at least as large as min_size.  */
  if (length as size_t) < min_size
    { return std::ptr::null_mut (); }

  /* Allocate a buffer of sufficient size.  */
  format8 = xmalloc (length as size_t + std::mem::size_of::<sfnt_cmap_format_8> ())
    as *mut sfnt_cmap_format_8;
  (*format8).format = (*header).format;
  (*format8).reserved = (*header).length;
  (*format8).length = length;

  /* Read the fixed length data.  */
  min_size -= std::mem::offset_of! (sfnt_cmap_format_8, language);
  rc = read (fd, &mut (*format8).language as *mut uint32_t as *mut c_void,
	     min_size);
  if rc == -1 || (rc as size_t) < min_size
    {
      xfree (format8 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_8);
    }

  /* Swap what was read.  */
  sfnt_swap32! (&mut (*format8).language);
  sfnt_swap32! (&mut (*format8).num_groups);

  /* See if the size is sufficient to read the variable length
     data.  */
  min_size = SFNT_ENDOF! (sfnt_cmap_format_8, num_groups, uint32_t);

  let (ckd_temp, mul_overflow)
    = ((*format8).num_groups as size_t)
    .overflowing_mul (std::mem::size_of::<sfnt_cmap_format_8_or_12_group> ());
  temp = ckd_temp;
  if mul_overflow
    {
      xfree (format8 as *mut c_void);
      return std::ptr::null_mut ();
    }

  let (ckd_min_size, add_overflow) = min_size.overflowing_add (temp);
  min_size = ckd_min_size;
  if add_overflow
    {
      xfree (format8 as *mut c_void);
      return std::ptr::null_mut ();
    }

  if (length as size_t) < min_size
    {
      xfree (format8 as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Now read the variable length data.  */
  rc = read (fd, format8.offset (1) as *mut c_void, temp);
  if rc == -1 || (rc as size_t) < temp
    {
      xfree (format8 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_8);
    }

  /* Set the pointer to the variable length data.  */
  (*format8).groups = format8.offset (1) as *mut sfnt_cmap_format_8_or_12_group;

  i = 0;
  while i < (*format8).num_groups
    {
      sfnt_swap32! (&mut (*((*format8).groups).offset (i as isize)).start_char_code);
      sfnt_swap32! (&mut (*((*format8).groups).offset (i as isize)).end_char_code);
      sfnt_swap32! (&mut (*((*format8).groups).offset (i as isize)).start_glyph_code);
      i += 1;
    }

  /* All done.  */
  return format8;
}

/* Read a format 12 cmap subtable from FD.  HEADER has already been
   read.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_cmap_format_12 (fd: c_int,
				   header: *mut sfnt_cmap_encoding_subtable_data)
  -> *mut sfnt_cmap_format_12                             // C 755-841
{
  let mut format12: *mut sfnt_cmap_format_12;
  let mut min_size: size_t;
  let mut temp: size_t;
  let mut rc: ssize_t;
  let mut length: uint32_t = 0;
  let mut i: uint32_t;

  /* Read the 32-bit length field.  */
  if (read (fd, &mut length as *mut uint32_t as *mut c_void,
	    std::mem::size_of::<uint32_t> ())
      < std::mem::size_of::<uint32_t> () as ssize_t)
    { return (usize::MAX as *mut sfnt_cmap_format_12); }

  /* Swap the 32-bit length field.  */
  sfnt_swap32! (&mut length);

  min_size = SFNT_ENDOF! (sfnt_cmap_format_12, num_groups, uint32_t);

  /* Make sure the header is at least as large as min_size.  */
  if (length as size_t) < min_size
    { return std::ptr::null_mut (); }

  /* Allocate a buffer of sufficient size.  */
  format12 = xaddmalloc (std::mem::size_of::<sfnt_cmap_format_12> (),
			 length as size_t) as *mut sfnt_cmap_format_12;
  (*format12).format = (*header).format;
  (*format12).reserved = (*header).length;
  (*format12).length = length;

  /* Read the fixed length data.  */
  min_size -= std::mem::offset_of! (sfnt_cmap_format_12, language);
  rc = read (fd, &mut (*format12).language as *mut uint32_t as *mut c_void,
	     min_size);
  if rc == -1 || (rc as size_t) < min_size
    {
      xfree (format12 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_12);
    }

  /* Swap what was read.  */
  sfnt_swap32! (&mut (*format12).language);
  sfnt_swap32! (&mut (*format12).num_groups);

  /* See if the size is sufficient to read the variable length
     data.  */
  min_size = SFNT_ENDOF! (sfnt_cmap_format_12, num_groups, uint32_t);

  let (ckd_temp, mul_overflow)
    = ((*format12).num_groups as size_t)
    .overflowing_mul (std::mem::size_of::<sfnt_cmap_format_8_or_12_group> ());
  temp = ckd_temp;
  if mul_overflow
    {
      xfree (format12 as *mut c_void);
      return std::ptr::null_mut ();
    }

  let (ckd_min_size, add_overflow) = min_size.overflowing_add (temp);
  min_size = ckd_min_size;
  if add_overflow
    {
      xfree (format12 as *mut c_void);
      return std::ptr::null_mut ();
    }

  if (length as size_t) < min_size
    {
      xfree (format12 as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Now read the variable length data.  */
  rc = read (fd, format12.offset (1) as *mut c_void, temp);
  if rc == -1 || (rc as size_t) < temp
    {
      xfree (format12 as *mut c_void);
      return (usize::MAX as *mut sfnt_cmap_format_12);
    }

  /* Set the pointer to the variable length data.  */
  (*format12).groups = format12.offset (1) as *mut sfnt_cmap_format_8_or_12_group;

  i = 0;
  while i < (*format12).num_groups
    {
      sfnt_swap32! (&mut (*((*format12).groups).offset (i as isize)).start_char_code);
      sfnt_swap32! (&mut (*((*format12).groups).offset (i as isize)).end_char_code);
      sfnt_swap32! (&mut (*((*format12).groups).offset (i as isize)).start_glyph_code);
      i += 1;
    }

  /* All done.  */
  return format12;
}

/* Read a 3-byte big endian number from BYTES.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_24 (bytes: *mut c_uchar) -> c_uint    // C 845-849
{
  return ((*bytes.offset (0) as c_uint) << 16u32)
    | ((*bytes.offset (1) as c_uint) << 8u32)
    | (*bytes.offset (2) as c_uint);
}

/* Read a format 14 cmap table from FD.  HEADER->format will be 14 and
   HEADER->length will be 0; the 16-bit length field is not read.
   OFFSET is the offset of the table's header in the font file.

   Only variation selector records will be read.  UVS tables will
   not.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_cmap_format_14 (fd: c_int,
				   header: *mut sfnt_cmap_encoding_subtable_data,
				   offset: off_t)
  -> *mut sfnt_cmap_format_14                             // C 858-932
{
  let mut format14: *mut sfnt_cmap_format_14;
  let mut length: uint32_t = 0;
  let mut num_records: uint32_t;
  let mut buffer1: [uint32_t; 2] = [0; 2];
  let mut size: size_t;
  let mut temp: size_t;
  let mut buffer: [c_char; 3 + 4 + 4] = [0; 3 + 4 + 4];
  let mut i: uint32_t;

  /* Read the length field and number of variation selector
     records.  */

  if read (fd, buffer1.as_mut_ptr () as *mut c_void,
	   std::mem::size_of::<[uint32_t; 2]> ())
      < std::mem::size_of::<[uint32_t; 2]> () as ssize_t
    { return std::ptr::null_mut (); }

  length = buffer1[0];
  num_records = buffer1[1];

  sfnt_swap32! (&mut length);
  sfnt_swap32! (&mut num_records);

  /* Now, the number of records present is known.  Allocate the format
     14 cmap table.  */

  size = std::mem::size_of::<sfnt_cmap_format_14> ();
  /* C 887-888 is `if (ckd_mul (&temp, num_records, sizeof *format14->records)
     || ckd_add (&size, size, temp))`: glibc's two `__builtin_*_overflow` calls write their
     result through the pointer and return the overflow flag, so the short-circuit `||` keeps
     `size` unchanged when the multiply overflows.  The port writes the two halves out at the
     site (P-15's spelling, buffer.rs 3602) and returns early, which is the same control flow.  */
  let (ckd_temp, mul_overflow)
    = (num_records as size_t).overflowing_mul (
	std::mem::size_of::<*mut sfnt_variation_selector_record> ());
  temp = ckd_temp;
  if mul_overflow
    { return std::ptr::null_mut (); }
  let (ckd_size, add_overflow) = size.overflowing_add (temp);
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  format14 = xmalloc (size) as *mut sfnt_cmap_format_14;

  /* Fill in the data already read.  */
  (*format14).format = (*header).format;
  (*format14).length = length;
  (*format14).num_var_selector_records = num_records as uint16_t;
  (*format14).offset = offset;

  /* Set the pointer to the remaining record data.  */
  (*format14).records
    = format14.offset (1) as *mut sfnt_variation_selector_record;

  /* Read each variation selector record.  */

  i = 0;
  while i < num_records
    {
      if read (fd, buffer.as_mut_ptr () as *mut c_void,
	       std::mem::size_of::<[c_char; 3 + 4 + 4]> ())
	  < std::mem::size_of::<[c_char; 3 + 4 + 4]> () as ssize_t
	{
	  xfree (format14 as *mut c_void);
	  return std::ptr::null_mut ();
	}

      /* First, read the 24 bit variation selector.  */
      (*((*format14).records).offset (i as isize)).var_selector
	= sfnt_read_24 (buffer.as_mut_ptr () as *mut c_uchar);

      /* Next, read the two unaligned longs.  */
      memcpy ((&raw mut (*((*format14).records).offset (i as isize)).default_uvs_offset)
	      as *mut uint32_t as *mut c_void,
	      buffer.as_ptr ().offset (3) as *const c_void,
	      std::mem::size_of::<uint32_t> ());
      memcpy ((&raw mut (*((*format14).records).offset (i as isize)).nondefault_uvs_offset)
	      as *mut uint32_t as *mut c_void,
	      buffer.as_ptr ().offset (7) as *const c_void,
	      std::mem::size_of::<uint32_t> ());

      /* And swap them.  */
      sfnt_swap32! (&mut (*((*format14).records).offset (i as isize)).default_uvs_offset);
      sfnt_swap32! (&mut (*((*format14).records).offset (i as isize)).nondefault_uvs_offset);
      i += 1;
    }

  /* Return the format 14 character mapping table.  */
  return format14;
}

/* Read the CMAP subtable data from a given file FD at TABLE_OFFSET
   bytes from DIRECTORY_OFFSET.  Return the subtable data if it is
   supported.  Else, value is NULL if the format is unsupported, or -1
   upon an IO error.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_cmap_table_1 (fd: c_int, directory_offset: uint32_t,
				  table_offset: uint32_t)
  -> *mut sfnt_cmap_encoding_subtable_data                // C 939-1011
{
  let mut offset: off_t;
  let mut header: sfnt_cmap_encoding_subtable_data = std::mem::zeroed ();

  let (ckd_offset, add_overflow)
    = (directory_offset as off_t).overflowing_add (table_offset as off_t);
  offset = ckd_offset;
  if add_overflow
    { return (usize::MAX as *mut sfnt_cmap_encoding_subtable_data); }

  if lseek (fd, offset, SEEK_SET) == (-1 as off_t)
    { return (usize::MAX as *mut sfnt_cmap_encoding_subtable_data); }

  if read (fd, &raw mut header.format as *mut uint16_t as *mut c_void,
	   std::mem::size_of::<uint16_t> ())
      < std::mem::size_of::<uint16_t> () as ssize_t
    { return (usize::MAX as *mut sfnt_cmap_encoding_subtable_data); }

  sfnt_swap16! (&mut header.format);

  /* Format 14 tables are rather special: they do not have a 16-bit
     `length' field.  When these tables are encountered, leave reading
     the rest of the header to `sfnt_read_cmap_table_14'.  */

  if header.format != 14
    {
      if read (fd, &raw mut header.length as *mut uint16_t as *mut c_void,
	       std::mem::size_of::<uint16_t> ())
	  < std::mem::size_of::<uint16_t> () as ssize_t
	{ return (usize::MAX as *mut sfnt_cmap_encoding_subtable_data); }

      sfnt_swap16! (&mut header.length);
    }
  else
    { header.length = 0; }

  match header.format
    {
    0 =>						/* C 975: `case 0:`.  */
      {
	/* If the length changes, then something has changed to the
	   format.  */
	if header.length != 262
	  { return std::ptr::null_mut (); }

	return (sfnt_read_cmap_format_0 (fd, &raw mut header)
		as *mut sfnt_cmap_encoding_subtable_data);
      }

    2 =>						/* C 984: `case 2:`.  */
      return (sfnt_read_cmap_format_2 (fd, &raw mut header)
	      as *mut sfnt_cmap_encoding_subtable_data),

    4 =>						/* C 988: `case 4:`.  */
      return (sfnt_read_cmap_format_4 (fd, &raw mut header)
	      as *mut sfnt_cmap_encoding_subtable_data),

    6 =>						/* C 992: `case 6:`.  */
      return (sfnt_read_cmap_format_6 (fd, &raw mut header)
	      as *mut sfnt_cmap_encoding_subtable_data),

    8 =>						/* C 996: `case 8:`.  */
      return (sfnt_read_cmap_format_8 (fd, &raw mut header)
	      as *mut sfnt_cmap_encoding_subtable_data),

    12 =>						/* C 1000: `case 12:`.  */
      return (sfnt_read_cmap_format_12 (fd, &raw mut header)
	      as *mut sfnt_cmap_encoding_subtable_data),

    14 =>						/* C 1004: `case 14:`.  */
      return (sfnt_read_cmap_format_14 (fd, &raw mut header, offset)
	      as *mut sfnt_cmap_encoding_subtable_data),

    _ =>						/* C 1008: `default:`.  */
      return std::ptr::null_mut (),
    }
}

/* Read the CMAP table of a given font from the file FD.  Use the
   table directory specified in SUBTABLE.

   Return the CMAP table and a list of encoding subtables in
   *SUBTABLES and *DATA upon success, else NULL.  If DATA is NULL, do
   not read the subtable data.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_cmap_table
  (fd: c_int, subtable: *mut sfnt_offset_subtable,
   subtables: *mut *mut sfnt_cmap_encoding_subtable,
   data: *mut *mut *mut sfnt_cmap_encoding_subtable_data)
  -> *mut sfnt_cmap_table                                // C 1020-1112
{
  let mut directory: *mut sfnt_table_directory;
  let mut cmap: *mut sfnt_cmap_table;
  let mut rc: ssize_t;
  let mut i: c_int;
  let mut j: c_int;

  /* Find the CMAP table in the table directory.  */
  directory = sfnt_find_table (subtable, SFNT_TABLE_CMAP);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the start of the CMAP table.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Read the table header.  */
  cmap = xmalloc (std::mem::size_of::<sfnt_cmap_table> ()) as *mut sfnt_cmap_table;
  rc = read (fd, cmap as *mut c_void, std::mem::size_of::<sfnt_cmap_table> ());

  if rc < std::mem::size_of::<sfnt_cmap_table> () as ssize_t
    {
      xfree (cmap as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap the header data.  */
  sfnt_swap16! (&mut (*cmap).version);
  sfnt_swap16! (&mut (*cmap).num_subtables);

  if (*cmap).version != 0
    {
      xfree (cmap as *mut c_void);
      return std::ptr::null_mut ();
    }

  *subtables = xmalloc ((*cmap).num_subtables as size_t
			* std::mem::size_of::<sfnt_cmap_encoding_subtable> ())
    as *mut sfnt_cmap_encoding_subtable;


  /* First, read the common parts of each encoding subtable.  */

  i = 0;
  while i < (*cmap).num_subtables as c_int
    {
      /* Read the common part of the new subtable.  */
      rc = read (fd, (&raw mut (*((*subtables)).offset (i as isize)))
		 as *mut sfnt_cmap_encoding_subtable as *mut c_void,
		 std::mem::size_of::<sfnt_cmap_encoding_subtable> ());

      if rc < std::mem::size_of::<sfnt_cmap_encoding_subtable> () as ssize_t
	{
	  xfree (cmap as *mut c_void);
	  xfree (*subtables as *mut c_void);
	  return std::ptr::null_mut ();
	}

      sfnt_swap16! (&mut (*((*subtables)).offset (i as isize)).platform_id);
      sfnt_swap16! (&mut (*((*subtables)).offset (i as isize)).platform_specific_id);
      sfnt_swap32! (&mut (*((*subtables)).offset (i as isize)).offset);
      i += 1;
    }

  /* If data is NULL, the caller only wants the table headers.  */

  if data.is_null ()
    { return cmap; }

  /* Second, read each encoding subtable itself.  */
  *data = xmalloc ((*cmap).num_subtables as size_t
		   * std::mem::size_of::<*mut *mut sfnt_cmap_encoding_subtable_data> ())
    as *mut *mut sfnt_cmap_encoding_subtable_data;

  i = 0;
  while i < (*cmap).num_subtables as c_int
    {
      (*((*data)).offset (i as isize)) = sfnt_read_cmap_table_1 (fd, (*directory).offset,
						    (*((*subtables)).offset (i as isize)).offset);

      if (*((*data)).offset (i as isize)) == (usize::MAX as *mut sfnt_cmap_encoding_subtable_data)
	{
	  /* An IO error occurred (as opposed to the subtable format
	     being unsupported.)  Return now.  */

	  j = 0;
	  while j < i
	    {
	      xfree ((*((*data)).offset (j as isize)) as *mut c_void);
	      j += 1;
	    }

	  xfree (*data as *mut c_void);
	  xfree (*subtables as *mut c_void);
	  xfree (cmap as *mut c_void);
	  return std::ptr::null_mut ();
	}
      i += 1;
    }

  return cmap;
}

/* C 1114-1465: the glyph lookup routines.  Six per-format readers behind `sfnt_lookup_glyph`,
   C's dispatcher; three of the seven are `static`, the helpers the others call
   (`sfnt_bsearch_above` and its two comparators) are `static` too, and `sfnt_lookup_glyph`
   itself is `TEST_STATIC` (the `cfg_attr` pair, C 1433).  */

/* Look up the glyph corresponding to CHARACTER in the format 0 cmap
   FORMAT0.  Return 0 if no glyph was found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_lookup_glyph_0 (character: sfnt_char,
			       format0: *mut sfnt_cmap_format_0)
  -> sfnt_glyph_index                                      // C 1117-1125
{
  if character >= 256
    { return 0; }

  return (*format0).glyph_index_array[character as usize] as sfnt_glyph_index;
}

/* Look up the glyph corresponding to CHARACTER in the format 2 cmap
   FORMAT2.  Return 0 if no glyph was found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_lookup_glyph_2 (character: sfnt_char,
			       format2: *mut sfnt_cmap_format_2)
  -> sfnt_glyph_index                                      // C 1131-1181
{
  let mut i: c_uchar;
  let mut k: c_uchar;
  let mut j: c_uchar;
  let mut subheader: *mut sfnt_cmap_format_2_subheader;
  let mut slice: *mut c_uchar;
  let mut glyph: uint16_t = 0;

  if character > 65535
    { return 0; }

  i = (character >> 8) as c_uchar;
  j = (character & 0xff) as c_uchar;
  k = ((*format2).sub_header_keys[i as usize] / 8) as c_uchar;

  if k != 0
    {
      subheader = (*format2).subheaders.offset (k as isize);

      if (*subheader).first_code as c_int <= j as c_int
	 && j as c_int <= ((*subheader).first_code as c_int
			   + (*subheader).entry_count as c_int)
	{
	  /* id_range_offset is actually the number of bytes past itself
	     containing the uint16_t ``slice''.  Whether this may be
	     unaligned is not stated in the specification, but I doubt
	     it, for that would render values meaningless if the array
	     were byte swapped when read.  */
	  slice = &raw mut (*subheader).id_range_offset as *mut c_uchar;
	  slice = slice.offset ((*subheader).id_range_offset as isize);
	  slice = slice.offset (((j as c_int - (*subheader).first_code as c_int)
				 * std::mem::size_of::<uint16_t> () as c_int) as isize);

	  if slice < (*format2).glyph_index_array as *mut c_uchar
	    || (slice.offset (1)
		> ((*format2).glyph_index_array.offset ((*format2).num_glyphs as isize)
		   as *mut c_uchar))
	    /* The character is out of bounds.  */
	    { return 0; }

	  memcpy (&raw mut glyph as *mut uint16_t as *mut c_void,
		  slice as *const c_void,
		  std::mem::size_of::<uint16_t> ());
	  return (((glyph as c_int) + ((*subheader).id_delta as c_int)) % 65536)
	    as sfnt_glyph_index;
	}
      else
	{ return 0; }
    }

  /* k is 0, so glyph_index_array[j] is the glyph.  */
  return (if (j as c_int) < ((*format2).num_glyphs as c_int)
	  { *((*format2).glyph_index_array.offset (j as isize)) as sfnt_glyph_index }
	  else
	  { 0 });
}

/* Like `bsearch', but return the element ordered exactly above KEY if
   one exists and KEY itself cannot be located.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_bsearch_above (key: *const c_void, base: *const c_void,
			      nmemb: size_t, size: size_t,
			      compar: Option<unsafe extern "C" fn (*const c_void,
								   *const c_void)
					     -> c_int>)
  -> *mut c_void                                           // C 1186-1220
{
  let mut bytes: *const c_uchar;
  let mut sample: *const c_uchar;
  let mut low: size_t;
  let mut high: size_t;
  let mut mid: size_t;

  bytes = base as *const c_uchar;
  low = 0;
  high = nmemb - 1;

  if nmemb == 0
    { return std::ptr::null_mut (); }

  while low != high
    {
      mid = low + (high - low) / 2;
      sample = bytes.offset ((mid * size) as isize);

      if (compar.unwrap ()) (key, sample as *const c_void) > 0
	{ low = mid + 1; }
      else
	{ high = mid; }
    }

  sample = bytes.offset ((low * size) as isize);

  if low == nmemb - 1
    && (compar.unwrap ()) (key, sample as *const c_void) > 0
    { return std::ptr::null_mut (); }

  return bytes.offset ((low * size) as isize) as *mut c_void;
}

/* Compare two uint16_t's.  Used to bisect through a format 4
   table.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_compare_uint16 (a: *const c_void, b: *const c_void) -> c_int
{                                                          // C 1225-1229
  return (*(a as *const uint16_t) as c_int) - (*(b as *const uint16_t) as c_int);
}

/* Look up the glyph corresponding to CODE in the format 4 cmap
   FORMAT4, using the table segment SEGMENT.  Value is 0 if no glyph
   was found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_lookup_glyph_4_1 (code: uint16_t, segment: uint16_t,
				 format4: *mut sfnt_cmap_format_4)
  -> sfnt_glyph_index                                      // C 1235-1262
{
  let mut index: *mut uint16_t;

  if (*((*format4).id_range_offset).offset (segment as isize)) != 0
    {
      /* id_range_offset is not 0, so the glyph mapping depends on
	 it.  */
      index = ((*format4).id_range_offset.offset (segment as isize)
	       .offset (((*((*format4).id_range_offset).offset (segment as isize)) as c_int / 2)
			as isize)
	       .offset ((code as c_int
			 - (*((*format4).start_code).offset (segment as isize)) as c_int)
			as isize)) as *mut uint16_t;

      /* Check that index is not out of bounds.  */
      if index >= (*format4).glyph_index_array.offset ((*format4).glyph_index_size as isize)
	 || index < (*format4).glyph_index_array
	{ return 0; }

      /* Return what is in index.  */
      return (if *index != 0
	      { (((*((*format4).id_delta).offset (segment as isize)) as c_int
		  + (*index as c_int)) % 65536) as sfnt_glyph_index }
	      else
	      { 0 });
    }

  /* Otherwise, just add id_delta.  */
  return (((*((*format4).id_delta).offset (segment as isize)) as c_int
	   + code as c_int) % 65536) as sfnt_glyph_index;
}

/* Look up the glyph corresponding to CHARACTER in the format 4 cmap
   FORMAT4.  Return 0 if no glyph was found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_lookup_glyph_4 (character: sfnt_char,
			       format4: *mut sfnt_cmap_format_4)
  -> sfnt_glyph_index                                      // C 1267-1298
{
  let mut segment_address: *mut uint16_t;
  let mut code: uint16_t;
  let mut segment: uint16_t;
  let mut glyph: sfnt_glyph_index;

  if character > 65535
    { return 0; }

  code = character as uint16_t;

  /* Find the segment ending above or at CHARACTER.  */
  segment_address = sfnt_bsearch_above (&raw const code as *const c_void,
					(*format4).end_code as *const c_void,
					((*format4).seg_count_x2 / 2) as size_t,
					std::mem::size_of::<uint16_t> (),
					Some (sfnt_compare_uint16)) as *mut uint16_t;
  segment = ((segment_address as isize - (*format4).end_code as isize)
	     / std::mem::size_of::<uint16_t> () as isize) as uint16_t;

  /* If the segment starts too late, return 0.  */
  if segment_address.is_null ()
    || (*((*format4).start_code).offset (segment as isize)) as sfnt_char > character
    { return 0; }

  glyph = sfnt_lookup_glyph_4_1 (character as uint16_t, segment, format4);

  if glyph != 0
    { return glyph; }

  /* Fail.  */
  return 0;
}

/* Look up the glyph corresponding to CHARACTER in the format 6 cmap
   FORMAT6.  Return 0 if no glyph was found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_lookup_glyph_6 (character: sfnt_char,
			       format6: *mut sfnt_cmap_format_6)
  -> sfnt_glyph_index                                      // C 1303-1313
{
  if character < (*format6).first_code as sfnt_char
    || character >= (((*format6).first_code as c_int
		      + (*format6).entry_count as c_int) as sfnt_char)
    { return 0; }

  return *((*format6).glyph_index_array
	   .offset ((character - (*format6).first_code as sfnt_char) as isize))
    as sfnt_glyph_index;
}

/* Compare the sfnt_char A with B's end code.  Employed to bisect
   through a format 8 or 12 table.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_compare_char (a: *const c_void, b: *const c_void) -> c_int
{                                                          // C 1318-1326
  let mut group: *mut sfnt_cmap_format_8_or_12_group;

  group = b as *mut sfnt_cmap_format_8_or_12_group;

  return (*(a as *const sfnt_char) as c_int) - (*group).end_char_code as c_int;
}

/* Look up the glyph corresponding to CHARACTER in the format 8 cmap
   FORMAT8.  Return 0 if no glyph was found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_lookup_glyph_8 (character: sfnt_char,
			       format8: *mut sfnt_cmap_format_8)
  -> sfnt_glyph_index                                      // C 1331-1375
{
  let mut i: uint32_t;
  let mut group: *mut sfnt_cmap_format_8_or_12_group;

  if character > 0xffffffff
    { return 0; }

  if (*format8).num_groups > 64
    {
      /* This table is large, likely supplied by a CJK or similar
	 font.  Perform a binary search.  */

      /* Find the group whose END_CHAR_CODE is greater than or equal
	 to CHARACTER.  */

      group = sfnt_bsearch_above (&raw const character as *const c_void,
				  (*format8).groups as *const c_void,
				  (*format8).num_groups as size_t,
				  std::mem::size_of::<sfnt_cmap_format_8_or_12_group> (),
				  Some (sfnt_compare_char))
	as *mut sfnt_cmap_format_8_or_12_group;

      if group.is_null () || (*group).start_char_code > character
	/* No glyph matches this group.  */
	{ return 0; }

      /* Otherwise, use this group to map the character to a
	 glyph.  */
      return (*group).start_glyph_code
	+ character
	- (*group).start_char_code;
    }

  i = 0;
  while i < (*format8).num_groups
    {
      if (*((*format8).groups.offset (i as isize))).start_char_code <= character
	&& (*((*format8).groups.offset (i as isize))).end_char_code >= character
	{
	  return (*((*format8).groups.offset (i as isize))).start_glyph_code
	    + (character
	       - (*((*format8).groups.offset (i as isize))).start_char_code);
	}
      i += 1;
    }

  return 0;
}

/* Look up the glyph corresponding to CHARACTER in the format 12 cmap
   FORMAT12.  Return 0 if no glyph was found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_lookup_glyph_12 (character: sfnt_char,
				format12: *mut sfnt_cmap_format_12)
  -> sfnt_glyph_index                                      // C 1380-1424
{
  let mut i: uint32_t;
  let mut group: *mut sfnt_cmap_format_8_or_12_group;

  if character > 0xffffffff
    { return 0; }

  if (*format12).num_groups > 64
    {
      /* This table is large, likely supplied by a CJK or similar
	 font.  Perform a binary search.  */

      /* Find the group whose END_CHAR_CODE is greater than or equal
	 to CHARACTER.  */

      group = sfnt_bsearch_above (&raw const character as *const c_void,
				  (*format12).groups as *const c_void,
				  (*format12).num_groups as size_t,
				  std::mem::size_of::<sfnt_cmap_format_8_or_12_group> (),
				  Some (sfnt_compare_char))
	as *mut sfnt_cmap_format_8_or_12_group;

      if group.is_null () || (*group).start_char_code > character
	/* No glyph matches this group.  */
	{ return 0; }

      /* Otherwise, use this group to map the character to a
	 glyph.  */
      return (*group).start_glyph_code
	+ character
	- (*group).start_char_code;
    }

  i = 0;
  while i < (*format12).num_groups
    {
      if (*((*format12).groups.offset (i as isize))).start_char_code <= character
	&& (*((*format12).groups.offset (i as isize))).end_char_code >= character
	{
	  return (*((*format12).groups.offset (i as isize))).start_glyph_code
	    + (character
	       - (*((*format12).groups.offset (i as isize))).start_char_code);
	}
      i += 1;
    }

  return 0;
}

/* Look up the glyph index corresponding to the character CHARACTER,
   which must be in the correct encoding for the cmap table pointed to
   by DATA.

   DATA must be either a format 0, 2, 4, 6, 8 or 12 cmap table, else
   behavior is undefined.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_lookup_glyph (character: sfnt_char,
					    data: *mut sfnt_cmap_encoding_subtable_data)
  -> sfnt_glyph_index                                      // C 1433-1465
{
  match (*data).format
    {
    0 =>							/* C 1439: `case 0:`.  */
      return (sfnt_lookup_glyph_0 (character,
				   data as *mut sfnt_cmap_format_0)),

    2 =>							/* C 1443: `case 2:`.  */
      return (sfnt_lookup_glyph_2 (character,
				   data as *mut sfnt_cmap_format_2)),

    4 =>							/* C 1447: `case 4:`.  */
      return (sfnt_lookup_glyph_4 (character,
				   data as *mut sfnt_cmap_format_4)),

    6 =>							/* C 1451: `case 6:`.  */
      return (sfnt_lookup_glyph_6 (character,
				   data as *mut sfnt_cmap_format_6)),

    8 =>							/* C 1455: `case 8:`.  */
      return (sfnt_lookup_glyph_8 (character,
				   data as *mut sfnt_cmap_format_8)),

    12 =>							/* C 1459: `case 12:`.  */
      return (sfnt_lookup_glyph_12 (character,
				    data as *mut sfnt_cmap_format_12)),

    _ =>							/* C 1462: the `switch` falls out.  */
      {}
    }

  return 0;
}

/* C 1467-1469: the form feed and the banner comment C opens this group with ("Header reading
   routines."); the port keeps the comment text and drops the page separator, exactly as it did
   for C 343-345.  */

/* Header reading routines.  */

/* Read the head table of a given font FD.  Use the table directory
   specified in SUBTABLE.

   Return the head table upon success, else NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_head_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_head_table                                 // C 1476-1541
{
  let mut directory: *mut sfnt_table_directory;
  let mut head: *mut sfnt_head_table;
  let mut rc: ssize_t;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_HEAD);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Read the entire table.  */
  head = xmalloc (std::mem::size_of::<sfnt_head_table> ()) as *mut sfnt_head_table;
  rc = read (fd, head as *mut c_void, std::mem::size_of::<sfnt_head_table> ());

  if rc < (std::mem::size_of::<sfnt_head_table> () as c_int) as ssize_t
    {
      xfree (head as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap the header data.  */
  sfnt_swap32! (&mut (*head).version);
  sfnt_swap32! (&mut (*head).revision);

  if (*head).version != 0x00010000
    {
      xfree (head as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap the rest of the data.  */
  sfnt_swap32! (&mut (*head).checksum_adjustment);
  sfnt_swap32! (&mut (*head).magic);

  if (*head).magic != 0x5f0f3cf5
    {
      xfree (head as *mut c_void);
      return std::ptr::null_mut ();
    }

  sfnt_swap16! (&mut (*head).flags);
  sfnt_swap16! (&mut (*head).units_per_em);
  sfnt_swap32! (&mut (*head).created_high);
  sfnt_swap32! (&mut (*head).created_low);
  sfnt_swap32! (&mut (*head).modified_high);
  sfnt_swap32! (&mut (*head).modified_low);
  sfnt_swap16! (&mut (*head).xmin);
  sfnt_swap16! (&mut (*head).xmax);
  sfnt_swap16! (&mut (*head).ymin);
  sfnt_swap16! (&mut (*head).ymax);
  sfnt_swap16! (&mut (*head).mac_style);
  sfnt_swap16! (&mut (*head).lowest_rec_ppem);
  sfnt_swap16! (&mut (*head).font_direction_hint);
  sfnt_swap16! (&mut (*head).index_to_loc_format);
  sfnt_swap16! (&mut (*head).glyph_data_format);

  return head;
}

/* Read the hhea table of a given font FD.  Use the table directory
   specified in SUBTABLE.

   Return the head table upon success, else NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_hhea_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_hhea_table                                 // C 1548-1607
{
  let mut directory: *mut sfnt_table_directory;
  let mut hhea: *mut sfnt_hhea_table;
  let mut rc: ssize_t;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_HHEA);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Check the length is right.  */
  if (*directory).length as size_t != std::mem::size_of::<sfnt_hhea_table> ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Read the entire table.  */
  hhea = xmalloc (std::mem::size_of::<sfnt_hhea_table> ()) as *mut sfnt_hhea_table;
  rc = read (fd, hhea as *mut c_void, std::mem::size_of::<sfnt_hhea_table> ());

  if rc < (std::mem::size_of::<sfnt_hhea_table> () as c_int) as ssize_t
    {
      xfree (hhea as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap the header data.  */
  sfnt_swap32! (&mut (*hhea).version);

  if (*hhea).version != 0x00010000
    {
      xfree (hhea as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap the rest of the data.  */
  sfnt_swap16! (&mut (*hhea).ascent);
  sfnt_swap16! (&mut (*hhea).descent);
  sfnt_swap16! (&mut (*hhea).line_gap);
  sfnt_swap16! (&mut (*hhea).advance_width_max);
  sfnt_swap16! (&mut (*hhea).min_left_side_bearing);
  sfnt_swap16! (&mut (*hhea).min_right_side_bearing);
  sfnt_swap16! (&mut (*hhea).x_max_extent);
  sfnt_swap16! (&mut (*hhea).caret_slope_rise);
  sfnt_swap16! (&mut (*hhea).caret_slope_run);
  sfnt_swap16! (&mut (*hhea).reserved1);
  sfnt_swap16! (&mut (*hhea).reserved2);
  sfnt_swap16! (&mut (*hhea).reserved3);
  sfnt_swap16! (&mut (*hhea).reserved4);
  sfnt_swap16! (&mut (*hhea).metric_data_format);
  sfnt_swap16! (&mut (*hhea).num_of_long_hor_metrics);

  return hhea;
}

/* Read a short loca table from the given font FD.  Use the table
   directory specified in SUBTABLE.

   Return the short table upon success, else NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_loca_table_short (fd: c_int,
						     subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_loca_table_short                           // C 1614-1652
{
  let mut directory: *mut sfnt_table_directory;
  let mut loca: *mut sfnt_loca_table_short;
  let mut rc: ssize_t;
  let mut i: c_int;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_LOCA);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Figure out how many glyphs there are based on the length.  */
  loca = xaddmalloc (std::mem::size_of::<sfnt_loca_table_short> (),
		     (*directory).length as size_t) as *mut sfnt_loca_table_short;
  (*loca).offsets = loca.offset (1) as *mut uint16_t;
  (*loca).num_offsets = ((*directory).length / 2) as size_t;

  /* Read the variable-length table data.  */
  rc = read (fd, (*loca).offsets as *mut c_void, (*directory).length as size_t);
  if rc < (*directory).length as ssize_t
    {
      xfree (loca as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap each of the offsets.  */
  i = 0;
  while (i as size_t) < (*loca).num_offsets
    {
      sfnt_swap16! (&mut *((*loca).offsets.offset (i as isize)));
      i += 1;
    }

  /* Return the table.  */
  return loca;
}

/* Read a long loca table from the given font FD.  Use the table
   directory specified in SUBTABLE.

   Return the long table upon success, else NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_loca_table_long (fd: c_int,
						    subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_loca_table_long                            // C 1659-1697
{
  let mut directory: *mut sfnt_table_directory;
  let mut loca: *mut sfnt_loca_table_long;
  let mut rc: ssize_t;
  let mut i: c_int;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_LOCA);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Figure out how many glyphs there are based on the length.  */
  loca = xaddmalloc (std::mem::size_of::<sfnt_loca_table_long> (),
		     (*directory).length as size_t) as *mut sfnt_loca_table_long;
  (*loca).offsets = loca.offset (1) as *mut uint32_t;
  (*loca).num_offsets = ((*directory).length / 4) as size_t;

  /* Read the variable-length table data.  */
  rc = read (fd, (*loca).offsets as *mut c_void, (*directory).length as size_t);
  if rc < (*directory).length as ssize_t
    {
      xfree (loca as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap each of the offsets.  */
  i = 0;
  while (i as size_t) < (*loca).num_offsets
    {
      sfnt_swap32! (&mut *((*loca).offsets.offset (i as isize)));
      i += 1;
    }

  /* Return the table.  */
  return loca;
}

/* Read the maxp table from the given font FD.  Use the table
   directory specified in SUBTABLE.

   Return the maxp table upon success, else NULL.  If the version is
   0.5, fields past num_glyphs will not be populated.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_maxp_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_maxp_table                                 // C 1705-1781
{
  let mut directory: *mut sfnt_table_directory;
  let mut maxp: *mut sfnt_maxp_table;
  let mut size: size_t;
  let mut rc: ssize_t;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_MAXP);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* If directory->length is not big enough for version 0.5, punt.  */
  if ((*directory).length as size_t)
      < SFNT_ENDOF! (sfnt_maxp_table, num_glyphs, uint16_t)
    { return std::ptr::null_mut (); }

  /* Allocate the buffer to hold the data.  Then, read
     directory->length or sizeof *maxp bytes into it, whichever is
     smaller.  */

  maxp = xmalloc (std::mem::size_of::<sfnt_maxp_table> ()) as *mut sfnt_maxp_table;
  size = MIN! ((*directory).length as size_t, std::mem::size_of::<sfnt_maxp_table> ());
  rc = read (fd, maxp as *mut c_void, size);

  if rc == -1 || (rc as size_t) < size
    {
      xfree (maxp as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Now, swap version and num_glyphs.  */
  sfnt_swap32! (&mut (*maxp).version);
  sfnt_swap16! (&mut (*maxp).num_glyphs);

  /* Reject version 1.0 tables that are too small.  */
  if (*maxp).version > 0x00005000 && size < std::mem::size_of::<sfnt_maxp_table> ()
    {
      xfree (maxp as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* If the table is version 0.5, then this function is done.  */
  if (*maxp).version == 0x00005000
    { return maxp; }
  else if (*maxp).version != 0x00010000
    {
      /* Reject invalid versions.  */
      xfree (maxp as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Otherwise, swap the rest of the fields.  */
  sfnt_swap16! (&mut (*maxp).max_points);
  sfnt_swap16! (&mut (*maxp).max_contours);
  sfnt_swap16! (&mut (*maxp).max_composite_points);
  sfnt_swap16! (&mut (*maxp).max_composite_contours);
  sfnt_swap16! (&mut (*maxp).max_zones);
  sfnt_swap16! (&mut (*maxp).max_twilight_points);
  sfnt_swap16! (&mut (*maxp).max_storage);
  sfnt_swap16! (&mut (*maxp).max_function_defs);
  sfnt_swap16! (&mut (*maxp).max_instruction_defs);
  sfnt_swap16! (&mut (*maxp).max_stack_elements);
  sfnt_swap16! (&mut (*maxp).max_size_of_instructions);
  sfnt_swap16! (&mut (*maxp).max_component_elements);
  sfnt_swap16! (&mut (*maxp).max_component_depth);

  /* All done.  */
  return maxp;
}

/* C 1783-1785: the form feed and the banner comment C opens the next group with ("Glyph
   outlining generation.").  */

/* Glyph outlining generation.  */

/* Read a glyf table from the given font FD.  Use the table directory
   specified in SUBTABLE.  The glyph data is not swapped.

   Return the glyf table upon success, else NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_glyf_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_glyf_table                                 // C 1792-1825
{
  let mut directory: *mut sfnt_table_directory;
  let mut glyf: *mut sfnt_glyf_table;
  let mut rc: ssize_t;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_GLYF);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Allocate enough to hold everything.  */
  glyf = xaddmalloc (std::mem::size_of::<sfnt_glyf_table> (),
		     (*directory).length as size_t) as *mut sfnt_glyf_table;
  (*glyf).size = (*directory).length as size_t;
  (*glyf).glyphs = glyf.offset (1) as *mut c_uchar;

  /* Read the glyph data.  */
  rc = read (fd, (*glyf).glyphs as *mut c_void, (*glyf).size);
  if rc == -1 || (rc as size_t) < (*glyf).size
    {
      xfree (glyf as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Return the table.  */
  return glyf;
}

/* C 1827-1906: `#if defined HAVE_MMAP && !defined TEST`, the mmap'ed glyf pair.  `HAVE_MMAP`
   is 1 in the frozen configuration (docs/baseline/config/darwin-arm64.macros.tsv 1469) and
   docs/baseline/feature-coverage.tsv carries no `have_mmap` row, so the port keeps that half
   live - the ruling slice 1 recorded for C 41 - and gates C's `!defined TEST` half with
   `not (feature = "test")`.  C's `#if` has no `#else` arm, so the other arm has no text.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	    not (feature = "test")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfnt_map_glyf_table (fd: c_int,
					      subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_glyf_table                                 // C 1837-1885
{
  let mut directory: *mut sfnt_table_directory;
  let mut glyf: *mut sfnt_glyf_table;
  let mut glyphs: *mut c_void;
  let mut offset: size_t;
  let mut page: size_t;
  let mut map_offset: size_t;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_GLYF);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Now try to map the glyph data.  Make sure offset is a multiple of
     the page size.  */

  page = getpagesize () as size_t;
  offset = (*directory).offset as size_t & !(page - 1);

  /* Figure out how much larger the mapping should be.  */
  map_offset = (*directory).offset as size_t - offset;

  /* Do the mmap.  */
  glyphs = mmap (std::ptr::null_mut (), (*directory).length as size_t + map_offset,
		 PROT_READ, MAP_PRIVATE, fd, offset as off_t);

  if glyphs == MAP_FAILED
    { return std::ptr::null_mut (); }

  /* An observation is that glyphs tend to be accessed in sequential
     order and immediately after the font's glyph table is loaded.  */

  /* C 1871-1876: `#ifdef HAVE_POSIX_MADVISE` is 1 in the frozen configuration
     (docs/baseline/config/darwin-arm64.macros.tsv 1492), so the `posix_madvise` arm is live;
     `#elif defined HAVE_MADVISE` (also 1, 1445) is carried under the feature the tree
     registers for `!HAVE_POSIX_MADVISE` (the `pdumper.rs` 5281-5284 shape), and the arm where
     neither macro is defined has no text in C.  */
  #[cfg (not (feature = "no_have_posix_madvise"))]
  {
    posix_madvise (glyphs, (*directory).length as size_t,
		   POSIX_MADV_WILLNEED);
  }
  #[cfg (feature = "no_have_posix_madvise")]
  {
    if cfg! (feature = "have_madvise")
      { madvise (glyphs, (*directory).length as size_t, MADV_WILLNEED); }
  }

  /* Allocate the glyf table.  */
  glyf = xmalloc (std::mem::size_of::<sfnt_glyf_table> ()) as *mut sfnt_glyf_table;
  (*glyf).size = (*directory).length as size_t;
  (*glyf).glyphs = (glyphs as *mut c_uchar).offset (map_offset as isize);
  (*glyf).start = glyphs as *mut c_uchar;

  return glyf;
}

/* Unmap the mmap'ed glyf table GLYF, then free its associated data.
   Value is 0 upon success, else 1, in which case GLYF is still freed
   all the same.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	    not (feature = "test")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfnt_unmap_glyf_table (glyf: *mut sfnt_glyf_table) -> c_int
{                                                          // C 1891-1904
  let mut rc: c_int;
  let mut size: size_t;

  /* Calculate the size of the mapping.  */
  size = (*glyf).size + (((*glyf).glyphs as isize) - ((*glyf).start as isize)) as size_t;

  rc = munmap ((*glyf).start as *mut c_void, size);
  xfree (glyf as *mut c_void);

  return (rc != 0) as c_int;
}

/* C 1906: `#endif /* HAVE_MMAP */`.  */

/* Read the simple glyph outline from the glyph GLYPH from the
   specified glyf table at the given offset.  Set GLYPH->simple to a
   non-NULL value upon success, else set it to NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_simple_glyph (glyph: *mut sfnt_glyph,
				  glyf: *mut sfnt_glyf_table,
				  offset: size_t)          // C 1912-2169
{
  let mut simple: *mut sfnt_simple_glyph;
  let mut min_size: ssize_t;
  let mut min_size_2: ssize_t;
  let mut i: c_int;
  let mut number_of_points: c_int;
  let mut repeat_count: c_int;
  let mut instructions_start: *mut c_uchar;
  let mut flags_start: *mut c_uchar;
  let mut flags_end: *mut c_uchar;
  let mut vec_start: *mut c_uchar;
  let mut delta: int16_t;
  let mut x: int16_t;
  let mut y: int16_t;

  (*glyph).simple = std::ptr::null_mut ();

  /* Calculate the minimum size of the glyph data.  This is the size
     of the instruction length field followed by
     glyph->number_of_contours * sizeof (uint16_t).  */

  min_size = ((*glyph).number_of_contours as c_int as size_t
	      * std::mem::size_of::<uint16_t> ()
	      + std::mem::size_of::<uint16_t> ()) as ssize_t;

  /* Check that the size is big enough.  */
  if (*glyf).size < (min_size as size_t) || (*glyf).size - (min_size as size_t) < offset
    { return; }

  /* Allocate enough to read at least that.  */
  simple = xaddmalloc (std::mem::size_of::<sfnt_simple_glyph> (),
		       min_size as size_t) as *mut sfnt_simple_glyph;
  (*simple).end_pts_of_contours = simple.offset (1) as *mut uint16_t;
  memcpy ((*simple).end_pts_of_contours as *mut c_void,
	  (*glyf).glyphs.offset (offset as isize) as *const c_void,
	  min_size as size_t);

  /* This is not really an index into simple->end_pts_of_contours.
     Rather, it is reading the first word past it.  */
  (*simple).instruction_length
    = *((*simple).end_pts_of_contours.offset ((*glyph).number_of_contours as isize));

  /* Swap the contour end point indices and the instruction
     length.  */

  i = 0;
  while i < (*glyph).number_of_contours as c_int
    {
      sfnt_swap16! (&mut *((*simple).end_pts_of_contours.offset (i as isize)));
      i += 1;
    }

  sfnt_swap16! (&mut (*simple).instruction_length);

  /* Based on those values, calculate the maximum size of the
     following data.  This is the instruction length + the last
     contour point + the last contour point * uint16_t * 2.  */

  if (*glyph).number_of_contours != 0
    {
      number_of_points
	= *((*simple).end_pts_of_contours
	    .offset ((*glyph).number_of_contours as isize - 1)) as c_int + 1;
    }
  else
    { number_of_points = 0; }

  (*simple).number_of_points = number_of_points as size_t;

  /* Make simple big enough.  */
  let mut size: size_t;
  let (ckd_min_size_2, mul_overflow)
    = (number_of_points as size_t)
      .overflowing_mul (std::mem::size_of::<uint16_t> () * 2 + 1);
  min_size_2 = ckd_min_size_2 as ssize_t;
  if mul_overflow
    { memory_full_up (); }

  let (ckd_min_size_2, add_overflow) = min_size_2.overflowing_add ((*simple).instruction_length as ssize_t);
  min_size_2 = ckd_min_size_2;
  if add_overflow
    { memory_full_up (); }

  let (ckd_size, add_overflow) = min_size.overflowing_add (min_size_2);
  size = ckd_size as size_t;
  if add_overflow
    { memory_full_up (); }

  simple = xaddrealloc (simple as *mut c_void, std::mem::size_of::<sfnt_simple_glyph> (),
			size) as *mut sfnt_simple_glyph;
  (*simple).end_pts_of_contours = simple.offset (1) as *mut uint16_t;

  /* Set the instruction data pointer and other pointers.
     simple->instructions comes one word past number_of_contours,
     because end_pts_of_contours also contains the instruction
     length.  */

  (*simple).x_coordinates = (*simple).end_pts_of_contours
    .offset ((*glyph).number_of_contours as isize + 1) as *mut int16_t;
  (*simple).y_coordinates = (*simple).x_coordinates.offset (number_of_points as isize);
  (*simple).instructions = (*simple).y_coordinates.offset (number_of_points as isize)
    as *mut uint8_t;
  (*simple).flags = (*simple).instructions.offset ((*simple).instruction_length as isize);

  /* Read instructions into the glyph.  */
  instructions_start = (*glyf).glyphs.offset (offset as isize).offset (min_size as isize);

  if instructions_start >= (*glyf).glyphs.offset ((*glyf).size as isize)
    || (instructions_start.offset ((*simple).instruction_length as isize)
	>= (*glyf).glyphs.offset ((*glyf).size as isize))
    {
      xfree (simple as *mut c_void);
      return;
    }

  memcpy ((*simple).instructions as *mut c_void,
	  instructions_start as *const c_void,
	  (*simple).instruction_length as size_t);

  /* Start reading flags.  */
  flags_start = (*glyf).glyphs.offset (offset as isize).offset (min_size as isize)
    .offset ((*simple).instruction_length as isize);
  flags_end = flags_start.offset (number_of_points as isize);

  if flags_start >= (*glyf).glyphs.offset ((*glyf).size as isize)
    {
      xfree (simple as *mut c_void);
      return;
    }

  i = 0;

  while flags_start < flags_end
    {
      if i == number_of_points
	{ break; }

      if flags_start >= (*glyf).glyphs.offset ((*glyf).size as isize)
	{ break; }

      *(*simple).flags.offset (i as isize) = *flags_start;
      i += 1;

      if (*flags_start as c_int & 0o10) != 0		/* REPEAT_FLAG */
	{
	  /* The next byte specifies how many times this byte is to be
	     repeated.  Check that it is in range.  */

	  if flags_start.offset (1) >= (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree (simple as *mut c_void);
	      return;
	    }

	  /* Repeat the current flag until
	     glyph->number_of_points.  */

	  repeat_count = *flags_start.offset (1) as c_int;

	  while i < number_of_points && repeat_count != 0
	    {
	      *(*simple).flags.offset (i as isize) = *flags_start;
	      i += 1;
	      repeat_count -= 1;
	    }

	  /* Skip one byte in flags_start.  */
	  flags_start = flags_start.offset (1);
	}

      flags_start = flags_start.offset (1);
    }

  /* If an insufficient number of flags have been read, then the
     outline is invalid.  */

  if i != number_of_points
    {
      xfree (simple as *mut c_void);
      return;
    }

  /* Now that the flags have been decoded, start decoding the
     vectors.  */
  vec_start = flags_start;
  i = 0;
  x = 0;

  /* flags_start is now repurposed to act as a pointer to the flags
     for the current vector! */
  flags_start = (*simple).flags;

  while i < number_of_points
    {
      delta = 0;

      if ((*flags_start as c_int) & 0o2) != 0		/* X_SHORT_VECTOR */
	{
	  /* The next byte is a delta to apply to the previous
	     value.  Make sure it is in bounds.  */

	  if vec_start.offset (1) > (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree (simple as *mut c_void);
	      return;
	    }

	  delta = *vec_start as int16_t;
	  vec_start = vec_start.offset (1);

	  if ((*flags_start as c_int) & 0o20) == 0	/* SAME_X */
	    { delta = -delta; }
	}
      else if ((*flags_start as c_int) & 0o20) == 0	/* SAME_X */
	{
	  /* The next word is a delta to apply to the previous value.
	     Make sure it is in bounds.  */

	  if vec_start.offset (2) > (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree (simple as *mut c_void);
	      return;
	    }

	  /* Read the unaligned word and swap it.  */
	  memcpy (&raw mut delta as *mut int16_t as *mut c_void,
		  vec_start as *const c_void,
		  std::mem::size_of::<int16_t> ());
	  sfnt_swap16! (&mut delta);
	  vec_start = vec_start.offset (2);
	}

      /* Apply the delta and set the X value.  */
      x += delta;
      *(*simple).x_coordinates.offset (i as isize) = x;
      i += 1;
      flags_start = flags_start.offset (1);
    }

  /* Decode the Y vector.  flags_start is again repurposed to act as a
     pointer to the flags for the current vector.  */
  flags_start = (*simple).flags;
  y = 0;
  i = 0;

  while i < number_of_points
    {
      delta = 0;

      if ((*flags_start as c_int) & 0o4) != 0		/* Y_SHORT_VECTOR */
	{
	  /* The next byte is a delta to apply to the previous
	     value.  Make sure it is in bounds.  */

	  if vec_start.offset (1) > (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree (simple as *mut c_void);
	      return;
	    }

	  delta = *vec_start as int16_t;
	  vec_start = vec_start.offset (1);

	  if ((*flags_start as c_int) & 0o40) == 0	/* SAME_Y */
	    { delta = -delta; }
	}
      else if ((*flags_start as c_int) & 0o40) == 0	/* SAME_Y */
	{
	  /* The next word is a delta to apply to the previous value.
	     Make sure it is in bounds.  */

	  if vec_start.offset (2) > (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree (simple as *mut c_void);
	      return;
	    }

	  /* Read the unaligned word and swap it.  */
	  memcpy (&raw mut delta as *mut int16_t as *mut c_void,
		  vec_start as *const c_void,
		  std::mem::size_of::<int16_t> ());
	  sfnt_swap16! (&mut delta);
	  vec_start = vec_start.offset (2);
	}

      /* Apply the delta and set the X value.  */
      y += delta;
      *(*simple).y_coordinates.offset (i as isize) = y;
      i += 1;
      flags_start = flags_start.offset (1);
    }

  /* All done.  */
  (*simple).y_coordinates_end = (*simple).y_coordinates.offset (i as isize);
  (*glyph).simple = simple;
}

/* Read the compound glyph outline from the glyph GLYPH from the
   specified glyf table at the given offset.  Set GLYPH->compound to a
   non-NULL value upon success, else set it to NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_compound_glyph (glyph: *mut sfnt_glyph,
				    glyf: *mut sfnt_glyf_table,
				    offset: size_t)        // C 2175-2407
{
  let mut flags: uint16_t = 0;
  let mut instruction_length: uint16_t = 0;
  let mut words: [uint16_t; 2] = [0; 2];
  let mut words4: [uint16_t; 4] = [0; 4];
  let mut required_bytes: size_t;
  let mut num_components: size_t;
  let mut i: size_t;
  let mut data: *mut c_uchar;
  let mut instruction_base: *mut c_uchar;

  /* Assume failure for now.  Figure out how many bytes have to be
     allocated by reading the compound data.  */
  (*glyph).compound = std::ptr::null_mut ();
  required_bytes = 0;
  num_components = 0;
  data = (*glyf).glyphs.offset (offset as isize);

  /* Offset could be unaligned.  */
  loop
    {
      if data.offset (2) > (*glyf).glyphs.offset ((*glyf).size as isize)
	{ return; }

      memcpy (&raw mut flags as *mut uint16_t as *mut c_void,
	      data as *const c_void,
	      std::mem::size_of::<uint16_t> ());
      sfnt_swap16! (&mut flags);
      data = data.offset (std::mem::size_of::<uint16_t> () as isize);

      /* Require at least one structure to hold this data.  */
      required_bytes += std::mem::size_of::<sfnt_compound_glyph_component> ();
      num_components += 1;

      /* Skip past unused data.  */
      data = data.offset (2);

      if (flags as c_int & 0o1) != 0			/* ARG_1_AND_2_ARE_WORDS */
	{ data = data.offset ((std::mem::size_of::<int16_t> () * 2) as isize); }
      else
	{ data = data.offset ((std::mem::size_of::<int8_t> () * 2) as isize); }

      if (flags as c_int & 0o10) != 0			/* WE_HAVE_A_SCALE */
	{ data = data.offset (std::mem::size_of::<uint16_t> () as isize); }
      else if (flags as c_int & 0o100) != 0		/* WE_HAVE_AN_X_AND_Y_SCALE */
	{ data = data.offset ((std::mem::size_of::<uint16_t> () * 2) as isize); }
      else if (flags as c_int & 0o200) != 0		/* WE_HAVE_A_TWO_BY_TWO */
	{ data = data.offset ((std::mem::size_of::<uint16_t> () * 4) as isize); }

      if (flags as c_int & 0o40) == 0			/* MORE_COMPONENTS */
	{ break; }
    }

  if (flags as c_int & 0o400) != 0			/* WE_HAVE_INSTRUCTIONS */
    {
      /* Figure out the instruction length.  */
      if data.offset (2) > (*glyf).glyphs.offset ((*glyf).size as isize)
	{ return; }

      /* Now see how much is required to hold the instruction
	 data.  */
      memcpy (&raw mut instruction_length as *mut uint16_t as *mut c_void,
	      data as *const c_void,
	      std::mem::size_of::<uint16_t> ());
      sfnt_swap16! (&mut instruction_length);
      required_bytes += instruction_length as size_t;
      data = data.offset (std::mem::size_of::<*mut c_uchar> () as isize
			  + instruction_length as isize);
    }

  /* Now allocate the buffer to hold all the glyph data.  */
  (*glyph).compound = xaddmalloc (std::mem::size_of::<sfnt_compound_glyph> (),
				  required_bytes) as *mut sfnt_compound_glyph;
  (*(*glyph).compound).components
    = (*glyph).compound.offset (1) as *mut sfnt_compound_glyph_component;
  (*(*glyph).compound).num_components = num_components;

  /* Figure out where instruction data starts.  It comes after
     glyph->compound->components ends.  */
  instruction_base
    = (*(*glyph).compound).components
      .offset ((*(*glyph).compound).num_components as isize) as *mut c_uchar;

  /* Start reading.  */
  i = 0;
  data = (*glyf).glyphs.offset (offset as isize);
  loop
    {
      if data.offset (4) > (*glyf).glyphs.offset ((*glyf).size as isize)
	{
	  xfree ((*glyph).compound as *mut c_void);
	  (*glyph).compound = std::ptr::null_mut ();
	  return;
	}

      memcpy (&raw mut flags as *mut uint16_t as *mut c_void,
	      data as *const c_void,
	      std::mem::size_of::<uint16_t> ());
      sfnt_swap16! (&mut flags);
      data = data.offset (std::mem::size_of::<uint16_t> () as isize);
      (*((*(*glyph).compound).components.offset (i as isize))).flags = flags;

      memcpy (&raw mut (*((*(*glyph).compound).components.offset (i as isize))).glyph_index
	      as *mut uint16_t as *mut c_void,
	      data as *const c_void,
	      std::mem::size_of::<uint16_t> ());
      sfnt_swap16! (&mut (*((*(*glyph).compound).components.offset (i as isize))).glyph_index);
      data = data.offset (std::mem::size_of::<uint16_t> () as isize);

      if (flags as c_int & 0o1) != 0			/* ARG_1_AND_2_ARE_WORDS.  */
	{
	  if data.offset (4) > (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree ((*glyph).compound as *mut c_void);
	      (*glyph).compound = std::ptr::null_mut ();
	      return;
	    }

	  /* Read two words into arg1 and arg2.  */
	  memcpy (words.as_mut_ptr () as *mut c_void,
		  data as *const c_void,
		  std::mem::size_of::<[uint16_t; 2]> ());
	  sfnt_swap16! (&mut words[0]);
	  sfnt_swap16! (&mut words[1]);

	  (*((*(*glyph).compound).components.offset (i as isize))).argument1.c = words[0];
	  (*((*(*glyph).compound).components.offset (i as isize))).argument2.c = words[1];
	  data = data.offset (std::mem::size_of::<[uint16_t; 2]> () as isize);
	}
      else
	{
	  if data.offset (2) > (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree ((*glyph).compound as *mut c_void);
	      (*glyph).compound = std::ptr::null_mut ();
	      return;
	    }

	  /* Read two bytes into arg1 and arg2.  */
	  (*((*(*glyph).compound).components.offset (i as isize))).argument1.a = *data;
	  (*((*(*glyph).compound).components.offset (i as isize))).argument2.a
	    = *data.offset (1);
	  data = data.offset (2);
	}

      if (flags as c_int & 0o10) != 0			/* WE_HAVE_A_SCALE */
	{
	  if data.offset (2) > (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree ((*glyph).compound as *mut c_void);
	      (*glyph).compound = std::ptr::null_mut ();
	      return;
	    }

	  /* Read one word into scale.  */
	  memcpy (&raw mut (*((*(*glyph).compound).components.offset (i as isize))).u.scale
		  as *mut int16_t as *mut c_void,
		  data as *const c_void,
		  std::mem::size_of::<int16_t> ());
	  sfnt_swap16! (&mut (*((*(*glyph).compound).components.offset (i as isize))).u.scale);
	  data = data.offset (std::mem::size_of::<int16_t> () as isize);
	}
      else if (flags as c_int & 0o100) != 0		/* WE_HAVE_AN_X_AND_Y_SCALE.  */
	{
	  if data.offset (4) > (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree ((*glyph).compound as *mut c_void);
	      (*glyph).compound = std::ptr::null_mut ();
	      return;
	    }

	  /* Read two words into xscale and yscale.  */
	  memcpy (words.as_mut_ptr () as *mut c_void,
		  data as *const c_void,
		  std::mem::size_of::<[uint16_t; 2]> ());
	  sfnt_swap16! (&mut words[0]);
	  sfnt_swap16! (&mut words[1]);

	  (*((*(*glyph).compound).components.offset (i as isize))).u.a.xscale
	    = words[0] as int16_t;
	  (*((*(*glyph).compound).components.offset (i as isize))).u.a.yscale
	    = words[1] as int16_t;
	  data = data.offset (std::mem::size_of::<[uint16_t; 2]> () as isize);
	}
      else if (flags as c_int & 0o200) != 0		/* WE_HAVE_A_TWO_BY_TWO */
	{
	  if data.offset (8) > (*glyf).glyphs.offset ((*glyf).size as isize)
	    {
	      xfree ((*glyph).compound as *mut c_void);
	      (*glyph).compound = std::ptr::null_mut ();
	      return;
	    }

	  /* Read 4 words into the transformation matrix.  */
	  memcpy (words4.as_mut_ptr () as *mut c_void,
		  data as *const c_void,
		  std::mem::size_of::<[uint16_t; 4]> ());
	  sfnt_swap16! (&mut words4[0]);
	  sfnt_swap16! (&mut words4[1]);
	  sfnt_swap16! (&mut words4[2]);
	  sfnt_swap16! (&mut words4[3]);

	  (*((*(*glyph).compound).components.offset (i as isize))).u.b.xscale
	    = words4[0] as int16_t;
	  (*((*(*glyph).compound).components.offset (i as isize))).u.b.scale01
	    = words4[1] as int16_t;
	  (*((*(*glyph).compound).components.offset (i as isize))).u.b.scale10
	    = words4[2] as int16_t;
	  (*((*(*glyph).compound).components.offset (i as isize))).u.b.yscale
	    = words4[3] as int16_t;
	  data = data.offset (std::mem::size_of::<[uint16_t; 4]> () as isize);
	}

      /* Record the component flags.  */
      (*((*(*glyph).compound).components.offset (i as isize))).flags = flags;

      i += 1;

      if (flags as c_int & 0o40) == 0			/* MORE_COMPONENTS */
	{ break; }
    }

  if (flags as c_int & 0o400) != 0			/* WE_HAVE_INSTR */
    {
      /* Figure out the instruction length.  */
      if data.offset (2) > (*glyf).glyphs.offset ((*glyf).size as isize)
	{
	  xfree ((*glyph).compound as *mut c_void);
	  (*glyph).compound = std::ptr::null_mut ();
	  return;
	}

      /* Now see how much is required to hold the instruction
	 data.  */
      memcpy (&raw mut (*(*glyph).compound).instruction_length
	      as *mut uint16_t as *mut c_void,
	      data as *const c_void,
	      std::mem::size_of::<uint16_t> ());
      sfnt_swap16! (&mut (*(*glyph).compound).instruction_length);
      data = data.offset (2);

      /* Read the instructions.  */
      (*(*glyph).compound).instructions = instruction_base;

      if data.offset ((*(*glyph).compound).instruction_length as isize)
	  > (*glyf).glyphs.offset ((*glyf).size as isize)
	{
	  xfree ((*glyph).compound as *mut c_void);
	  (*glyph).compound = std::ptr::null_mut ();
	  return;
	}

      memcpy (instruction_base as *mut c_void,
	      data as *const c_void,
	      (*(*glyph).compound).instruction_length as size_t);
    }
  else
    {
      (*(*glyph).compound).instructions = std::ptr::null_mut ();
      (*(*glyph).compound).instruction_length = 0;
    }

  /* Data read successfully.  */
  return;
}

/* Read the description of the glyph GLYPH_CODE from the specified
   glyf table, using the offsets of LOCA_SHORT or LOCA_LONG, depending
   on which is non-NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_glyph (glyph_code: sfnt_glyph_index,
					   glyf: *mut sfnt_glyf_table,
					   loca_short: *mut sfnt_loca_table_short,
					   loca_long: *mut sfnt_loca_table_long)
  -> *mut sfnt_glyph                                // C 2413-2525
{
  let mut glyph: sfnt_glyph = std::mem::zeroed ();
  let mut memory: *mut sfnt_glyph;
  let mut offset: size_t;
  let mut next_offset: size_t;

  /* Check the glyph code is within bounds.  */
  if glyph_code > 65535
    { return std::ptr::null_mut (); }

  if !loca_short.is_null ()
    {
      /* Check that the glyph is within bounds.  glyph_code + 1 is the
	 entry in the table which defines the length of the glyph.  */
      if ((glyph_code + 1) as size_t) >= (*loca_short).num_offsets
	{ return std::ptr::null_mut (); }

      offset = ((*(*loca_short).offsets.offset (glyph_code as isize)) as c_int * 2)
	as size_t;
      next_offset
	= ((*(*loca_short).offsets.offset ((glyph_code + 1) as isize)) as c_int * 2)
	  as size_t;
    }
  else if !loca_long.is_null ()
    {
      if ((glyph_code + 1) as size_t) >= (*loca_long).num_offsets
	{ return std::ptr::null_mut (); }

      offset = *(*loca_long).offsets.offset (glyph_code as isize) as size_t;
      next_offset
	= *(*loca_long).offsets.offset ((glyph_code + 1) as isize) as size_t;
    }
  else
    { abort (); }

  /* If offset - next_offset is 0, then the glyph is empty.  Its
     horizontal advance may still be provided by the hmtx table.  */

  if offset == next_offset
    {
      glyph.number_of_contours = 0;
      glyph.xmin = 0;
      glyph.ymin = 0;
      glyph.xmax = 0;
      glyph.ymax = 0;
      glyph.advance_distortion = 0;
      glyph.origin_distortion = 0;
      glyph.simple
	= xzalloc (std::mem::size_of::<sfnt_simple_glyph> ()) as *mut sfnt_simple_glyph;
      glyph.compound = std::ptr::null_mut ();
      memory = xmalloc (std::mem::size_of::<sfnt_glyph> ()) as *mut sfnt_glyph;
      *memory = glyph;
      return memory;
    }

  /* Verify that GLYF is big enough to hold a glyph at OFFSET.  */
  if (*glyf).size < offset + SFNT_ENDOF! (sfnt_glyph, ymax, sfnt_fword)
    { return std::ptr::null_mut (); }

  /* Copy over the glyph data.  */
  memcpy (&raw mut glyph as *mut c_void,
	  (*glyf).glyphs.offset (offset as isize) as *const c_void,
	  SFNT_ENDOF! (sfnt_glyph, ymax, sfnt_fword));

  /* Swap the glyph data.  */
  sfnt_swap16! (&mut glyph.number_of_contours);
  sfnt_swap16! (&mut glyph.xmin);
  sfnt_swap16! (&mut glyph.ymin);
  sfnt_swap16! (&mut glyph.xmax);
  sfnt_swap16! (&mut glyph.ymax);

  /* This is set later on after `sfnt_vary_X_glyph'.  */
  glyph.advance_distortion = 0;
  glyph.origin_distortion = 0;

  /* Figure out what needs to be read based on
     glyph.number_of_contours.  */
  if glyph.number_of_contours >= 0
    {
      /* Read the simple glyph.  */

      glyph.compound = std::ptr::null_mut ();
      sfnt_read_simple_glyph (&raw mut glyph, glyf,
			      offset + SFNT_ENDOF! (sfnt_glyph, ymax, sfnt_fword));

      if !glyph.simple.is_null ()
	{
	  memory = xmalloc (std::mem::size_of::<sfnt_glyph> ()) as *mut sfnt_glyph;
	  *memory = glyph;

	  return memory;
	}
    }
  else
    {
      /* Read the compound glyph.  */

      glyph.simple = std::ptr::null_mut ();
      sfnt_read_compound_glyph (&raw mut glyph, glyf,
				offset + SFNT_ENDOF! (sfnt_glyph, ymax, sfnt_fword));

      if !glyph.compound.is_null ()
	{
	  memory = xmalloc (std::mem::size_of::<sfnt_glyph> ()) as *mut sfnt_glyph;
	  *memory = glyph;

	  return memory;
	}
    }

  return std::ptr::null_mut ();
}

/* Free a glyph returned from sfnt_read_glyph.  GLYPH may be NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_free_glyph (glyph: *mut sfnt_glyph)   // C 2529-2538
{
  if glyph.is_null ()
    { return; }

  xfree ((*glyph).simple as *mut c_void);
  xfree ((*glyph).compound as *mut c_void);
  xfree (glyph as *mut c_void);
}


/* Glyph outline decomposition.  */

/* Apply the transform in the compound glyph component COMPONENT to
   the array of points of length NUM_COORDINATES given as X and Y.

   Also, apply the fixed point offsets X_OFF and Y_OFF to each X and Y
   coordinate after transforms within COMPONENT are effected.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_transform_coordinates (component: *mut sfnt_compound_glyph_component,
				      x: *mut sfnt_fixed, y: *mut sfnt_fixed,
				      num_coordinates: size_t,
				      x_off: sfnt_fixed, y_off: sfnt_fixed)
                                                          // C 2550-2618
{
  let mut m1: f64;
  let mut m2: f64;
  let mut m3: f64;
  let mut m4: f64;
  let mut m5: f64;
  let mut m6: f64;
  let mut i: size_t;

  if ((*component).flags as c_int & 0o10) != 0		/* WE_HAVE_A_SCALE */
    {
      m1 = (*component).u.scale as f64 / 16384.0;
      m2 = 0.0; m3 = 0.0; m4 = 0.0;
      m5 = (*component).u.scale as f64 / 16384.0;
      m6 = 0.0;
    }
  else if ((*component).flags as c_int & 0o100) != 0	/* WE_HAVE_AN_X_AND_Y_SCALE */
    {
      m1 = (*component).u.a.xscale as f64 / 16384.0;
      m2 = 0.0; m3 = 0.0; m4 = 0.0;
      m5 = (*component).u.a.yscale as f64 / 16384.0;
      m6 = 0.0;
    }
  else if ((*component).flags as c_int & 0o200) != 0	/* WE_HAVE_A_TWO_BY_TWO */
    {
      m1 = (*component).u.b.xscale as f64 / 16384.0;
      m2 = (*component).u.b.scale01 as f64 / 16384.0;
      m3 = 0.0;
      m4 = (*component).u.b.scale10 as f64 / 16384.0;
      m5 = (*component).u.b.yscale as f64 / 16384.0;
      m6 = 0.0;
    }
  else							/* No scale, just apply x_off and y_off.  */
    {
      i = 0;
      while i < num_coordinates
	{
	  *(x.offset (i as isize)) += x_off;
	  *(y.offset (i as isize)) += y_off;
	  i += 1;
	}

      return;
    }

  m3 = x_off as f64;
  m6 = y_off as f64;

  /* Apply the specified affine transformation.
     A transform looks like:

     M1 M2 M3     X
     M4 M5 M6   * Y

     =

     M1*X + M2*Y + M3*1 = X1
     M4*X + M5*Y + M6*1 = Y1

     (In most transforms, there is another row at the bottom for
     mathematical reasons.  Since Z1 is always 1.0, the row is simply
     implied to be 0 0 1, because 0 * x + 0 * y + 1 * 1 = 1.0.  See
     the definition of matrix3x3 in image.c for some more explanations
     about this.) */

  i = 0;
  while i < num_coordinates
    {
      let xi: sfnt_fixed = (m1 * (*(x.offset (i as isize)) as f64)
			    + m2 * (*(y.offset (i as isize)) as f64)
			    + m3 * 1.0) as sfnt_fixed;
      let yi: sfnt_fixed = (m4 * (*(x.offset (i as isize)) as f64)
			    + m5 * (*(y.offset (i as isize)) as f64)
			    + m6 * 1.0) as sfnt_fixed;
      *(x.offset (i as isize)) = xi;
      *(y.offset (i as isize)) = yi;
      i += 1;
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct sfnt_compound_glyph_context             // C 2620-2647
{
  /* Arrays of points.  The underlying type is actually sfnt_f26dot6
     when instructing a compound glyph.  */
  pub x_coordinates: *mut sfnt_fixed,
  pub y_coordinates: *mut sfnt_fixed,

  /* Array of flags for the points.  */
  pub flags: *mut c_uchar,

  /* Number of points in that array, and the size of that array.  */
  pub num_points: size_t,
  pub points_size: size_t,

  /* Array of contour end points.  */
  pub contour_end_points: *mut size_t,

  /* Number of elements in and the size of that array.  */
  pub num_end_points: size_t,
  pub end_points_size: size_t,

  /* The X positions of two phantom points marking this glyph's origin
     and advance position, only used while interpreting the glyph.  */
  pub phantom_point_1_x: sfnt_f26dot6,
  pub phantom_point_2_x: sfnt_f26dot6,

  /* Y positions.  */
  pub phantom_point_1_y: sfnt_f26dot6,
  pub phantom_point_2_y: sfnt_f26dot6,

  /* Unrounded X positions.  */
  pub phantom_point_1_s: sfnt_f26dot6,
  pub phantom_point_2_s: sfnt_f26dot6,
}

/* Extend the arrays inside the compound glyph decomposition context
   CONTEXT.  NUMBER_OF_CONTOURS is the number of contours to add.
   NUMBER_OF_POINTS is the number of points to add.

   Return pointers to the beginning of the extension in *X_BASE,
   *Y_BASE, *FLAGS_BASE and *CONTOUR_BASE.  Value zero upon success,
   and something else on failure.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_expand_compound_glyph_context (context: *mut sfnt_compound_glyph_context,
					      number_of_contours: size_t,
					      number_of_points: size_t,
					      x_base: *mut *mut sfnt_fixed,
					      y_base: *mut *mut sfnt_fixed,
					      flags_base: *mut *mut c_uchar,
					      contour_base: *mut *mut size_t)
  -> c_int                                        // C 2657-2723
{
  let mut size_bytes: size_t;

  /* Add each field while checking for overflow.  */
  let (ckd_num_end_points, add_overflow)
    = number_of_contours.overflowing_add ((*context).num_end_points);
  (*context).num_end_points = ckd_num_end_points;
  if add_overflow
    { return 1; }

  let (ckd_num_points, add_overflow)
    = number_of_points.overflowing_add ((*context).num_points);
  (*context).num_points = ckd_num_points;
  if add_overflow
    { return 1; }

  /* Reallocate each array to the new size if necessary.  */
  if (*context).points_size < (*context).num_points
    {
      let (ckd_points_size, mul_overflow)
	= (*context).num_points.overflowing_mul (2);
      (*context).points_size = ckd_points_size;
      if mul_overflow
	{ (*context).points_size = (*context).num_points; }

      let (ckd_size_bytes, mul_overflow)
	= (*context).points_size.overflowing_mul (
	    std::mem::size_of_val (&*(*context).x_coordinates));
      size_bytes = ckd_size_bytes;
      if mul_overflow
	{ return 1; }

      (*context).x_coordinates
	= xrealloc ((*context).x_coordinates as *mut c_void, size_bytes)
	  as *mut sfnt_fixed;
      (*context).y_coordinates
	= xrealloc ((*context).y_coordinates as *mut c_void, size_bytes)
	  as *mut sfnt_fixed;
      (*context).flags
	= xrealloc ((*context).flags as *mut c_void, (*context).points_size)
	  as *mut c_uchar;
    }

  /* Set x_base and y_base.  */
  *x_base = (*context).x_coordinates
    .offset ((*context).num_points as isize)
    .offset (-(number_of_points as isize));
  *y_base = (*context).y_coordinates
    .offset ((*context).num_points as isize)
    .offset (-(number_of_points as isize));
  *flags_base = (*context).flags
    .offset ((*context).num_points as isize)
    .offset (-(number_of_points as isize));

  if (*context).end_points_size < (*context).num_end_points
    {
      let (ckd_end_points_size, mul_overflow)
	= (*context).num_end_points.overflowing_mul (2);
      (*context).end_points_size = ckd_end_points_size;
      if mul_overflow
	{ (*context).end_points_size = (*context).num_end_points; }

      let (ckd_size_bytes, mul_overflow)
	= (*context).end_points_size.overflowing_mul (
	    std::mem::size_of_val (&*(*context).contour_end_points));
      size_bytes = ckd_size_bytes;
      if mul_overflow
	{ return 1; }

      (*context).contour_end_points
	= xrealloc ((*context).contour_end_points as *mut c_void, size_bytes)
	  as *mut size_t;
    }

  /* Set contour_base.  */
  *contour_base = (*context).contour_end_points
    .offset ((*context).num_end_points as isize)
    .offset (-(number_of_contours as isize));
  return 0;
}

/* Round the 16.16 fixed point number NUMBER to the nearest integral
   value.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_round_fixed (mut number: int32_t) -> int32_t  // C 2728-2736
{
  /* Add 0.5... */
  number += 1 << 15;

  /* Remove the fractional.  */
  return number & !0xffff;
}

/* Decompose GLYPH, a compound glyph, into an array of points and
   contours.

   CONTEXT should be zeroed and put on the stack. RECURSION_COUNT
   should be initialized to 0.  GET_GLYPH, FREE_GLYPH, and
   GET_METRICS, along with DCONTEXT, mean the same as in
   sfnt_decompose_glyph.

   If it has been arranged that a component's metrics (or those of an
   innermore component also with the flag set) replace the metrics of
   GLYPH, set *METRICS_RETURN to those metrics.  Mind that such
   metrics are not scaled in any manner.

   Value is 1 upon failure, else 0.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_decompose_compound_glyph (glyph: *mut sfnt_glyph,
					 context: *mut sfnt_compound_glyph_context,
					 metrics_return: *mut sfnt_glyph_metrics,
					 get_glyph: sfnt_get_glyph_proc,
					 free_glyph: sfnt_free_glyph_proc,
					 get_metrics: sfnt_get_metrics_proc,
					 recursion_count: c_int,
					 dcontext: *mut c_void)
  -> c_int                                        // C 2753-3145
{
  let mut subglyph: *mut sfnt_glyph;
  let mut i: c_int;
  let mut j: c_int;
  let mut rc: c_int;
  let mut need_free: bool = false;
  let mut component: *mut sfnt_compound_glyph_component;
  let mut x: sfnt_fixed = 0;
  let mut y: sfnt_fixed = 0;
  let mut xtemp: sfnt_fixed;
  let mut ytemp: sfnt_fixed;
  let mut point: size_t = 0;
  let mut point2: size_t = 0;
  let mut index: size_t;
  let mut last_point: uint16_t;
  let mut number_of_contours: uint16_t;
  let mut x_base: *mut sfnt_fixed = std::ptr::null_mut ();
  let mut y_base: *mut sfnt_fixed = std::ptr::null_mut ();
  let mut contour_base: *mut size_t = std::ptr::null_mut ();
  let mut flags_base: *mut c_uchar = std::ptr::null_mut ();
  let mut base_index: size_t;
  let mut contour_start: size_t;
  let mut defer_offsets: bool;
  let mut sub_metrics: sfnt_glyph_metrics = std::mem::zeroed ();
  let mut f1: sfnt_fixed;
  let mut f2: sfnt_fixed;

  /* Set up the base index.  This is the index from where on point
     renumbering starts.

     In other words, point 0 in this glyph will be 0 + base_index,
     point 1 will be 1 + base_index, and so on.  */
  base_index = (*context).num_points;

  /* Prevent infinite loops.  Simply limit the level of nesting to the
     maximum valid value of `max_component_depth', which is 16.  */

  if recursion_count > 16
    { return 1; }

  j = 0;
  while (j as size_t) < (*(*glyph).compound).num_components
    {
      /* Look up the associated subglyph.  */
      component = (*(*glyph).compound).components.offset (j as isize);
      subglyph = get_glyph.unwrap () ((*component).glyph_index as sfnt_glyph_index,
				      dcontext, &raw mut need_free);

      if subglyph.is_null ()
	{ return 1; }

      /* Don't defer offsets.  This variable is set if the component
	 glyph is a compound glyph that is anchored to a previously
	 decomposed point, and needs its coordinates adjusted after
	 decomposition completes.  */
      defer_offsets = false;

      /* Record the size of the point array before expansion.  This
	 will be the base to apply to all points coming from this
	 subglyph.  */
      contour_start = (*context).num_points;

      /* Compute the offset for the component.  */
      if ((*component).flags as c_int & 0o2) != 0	/* ARGS_ARE_XY_VALUES */
	{
	  /* Component offsets are X/Y values as opposed to points
	     GLYPH.  */

	  if ((*component).flags as c_int & 0o1) == 0	/* ARG_1_AND_2_ARE_WORDS */
	    {
	      /* X and Y are signed bytes.  */
	      x = ((*component).argument1.b as c_int) * 65536;
	      y = ((*component).argument2.b as c_int) * 65536;
	    }
	  else
	    {
	      /* X and Y are signed words.  */
	      x = ((*component).argument1.d as c_int) * 65536;
	      y = ((*component).argument2.d as c_int) * 65536;
	    }

	  /* If there is some kind of scale and component offsets are
	     scaled, then apply the transform to the offset.  */
	  if ((*component).flags as c_int & 0o4000) != 0	/* SCALED_COMPONENT_OFFSET */
	    { sfnt_transform_coordinates (component, &raw mut x, &raw mut y, 1, 0, 0); }
	}
      else
	{
	  /* The offset is determined by matching a point location in
	     a preceding component with a point location in the
	     current component.  The index of the point in the
	     previous component can be determined by adding
	     component->argument1.a or component->argument1.c to
	     point.  argument2 contains the index of the point in the
	     current component.  */

	  if ((*component).flags as c_int & 0o1) == 0	/* ARG_1_AND_2_ARE_WORDS */
	    {
	      point = base_index + (*component).argument1.a as size_t;
	      point2 = (*component).argument2.a as size_t;
	    }
	  else
	    {
	      point = base_index + (*component).argument1.c as size_t;
	      point2 = (*component).argument2.c as size_t;
	    }

	  /* Now, check that the anchor point specified lies inside
	     the glyph.  */

	  if point >= contour_start
	    {
	      if need_free
		{ free_glyph.unwrap () (subglyph, dcontext); }

	      return 1;
	    }

	  if (*subglyph).compound.is_null ()
	    {
	      'skip_computation: {
		if point2 >= (*(*subglyph).simple).number_of_points
		  {
		    if point2 < (*(*subglyph).simple).number_of_points + 2
		      {
			/* POINT2 is one of SUBGLYPH's phantom points.
			   Retrieve the glyph's metrics.  */

			if get_metrics.unwrap () ((*component).glyph_index as sfnt_glyph_index,
						  &raw mut sub_metrics,
						  dcontext) != 0
			  {
			    if need_free
			      { free_glyph.unwrap () (subglyph, dcontext); }

			    return 1;
			  }

			/* Derive the phantom points from those metrics.  */
			f1 = (*glyph).xmin as sfnt_fixed - sub_metrics.lbearing;
			f2 = f1 + sub_metrics.advance;

			/* Apply the metrics distortion.  */
			f1 += (*glyph).origin_distortion as sfnt_fixed;
			f2 += (*glyph).advance_distortion as sfnt_fixed;

			/* Get the points and use them to compute the offsets.  */

			if (point2 - (*(*subglyph).simple).number_of_points) == 0
			  { x = f1 * 65536; }
			else
			  { x = f2 * 65536; }

			x = *((*context).x_coordinates.offset (point as isize)) - x;
			y = *((*context).y_coordinates.offset (point as isize));

			/* X and Y offsets have been ascertained.  */
			break 'skip_computation;
		      }

		    if need_free
		      { free_glyph.unwrap () (subglyph, dcontext); }

		    return 1;
		  }

		/* Get the points and use them to compute the offsets.  */
		xtemp = *((*context).x_coordinates.offset (point as isize));
		ytemp = *((*context).y_coordinates.offset (point as isize));
		x = xtemp
		  - ((*(*(*subglyph).simple).x_coordinates.offset (point2 as isize)) as c_int
		     * 65536);
		y = ytemp
		  - ((*(*(*subglyph).simple).y_coordinates.offset (point2 as isize)) as c_int
		     * 65536);
	      }
	      /* skip_computation: */
	    }
	  else
	    {
	      /* First, set offsets to 0, because it is not yet
		 possible to determine the position of the anchor
		 point in the child.  */
	      x = 0;
	      y = 0;

	      /* Set a flag which indicates that offsets must be
		 resolved from the child glyph after it is loaded, but
		 before it is incorporated into the parent glyph.  */
	      defer_offsets = true;
	    }
	}

      if !(*subglyph).simple.is_null ()
	{
	  /* Simple subglyph.  Copy over the points and contours, and
	     transform them.  */
	  if (*subglyph).number_of_contours != 0
	    {
	      index = ((*subglyph).number_of_contours - 1) as size_t;
	      last_point
		= *((*(*subglyph).simple).end_pts_of_contours.offset (index as isize));
	      number_of_contours = (*subglyph).number_of_contours as uint16_t;


	      /* Grow various arrays.  */
	      rc = sfnt_expand_compound_glyph_context (context,
						       /* Number of
							  new contours
							  required.  */
						       number_of_contours as size_t,
						       /* Number of new
							  points
							  required.  */
						       (last_point as c_int + 1) as size_t,
						       &raw mut x_base,
						       &raw mut y_base,
						       &raw mut flags_base,
						       &raw mut contour_base);
	      if rc != 0
		{
		  if need_free
		    { free_glyph.unwrap () (subglyph, dcontext); }

		  return 1;
		}

	      i = 0;
	      while i <= (last_point as c_int)
		{
		  *(x_base.offset (i as isize))
		    = ((*(*(*subglyph).simple).x_coordinates.offset (i as isize)) as c_int
		       * 65536);
		  *(y_base.offset (i as isize))
		    = ((*(*(*subglyph).simple).y_coordinates.offset (i as isize)) as c_int
		       * 65536);
		  *(flags_base.offset (i as isize))
		    = *(*(*subglyph).simple).flags.offset (i as isize);
		  i += 1;
		}

	      /* Apply the transform to the points.  */
	      sfnt_transform_coordinates (component, x_base, y_base,
					  (last_point as c_int + 1) as size_t, x, y);

	      /* Copy over the contours.  */
	      i = 0;
	      while i < (number_of_contours as c_int)
		{
		  *(contour_base.offset (i as isize))
		    = contour_start
		      + (*(*(*subglyph).simple).end_pts_of_contours.offset (i as isize))
			as size_t;
		  i += 1;
		}

	      /* If USE_MY_METRICS is present within this component,
		 save its metrics within *METRICS_RETURN.  */

	      if ((*component).flags as c_int & 0o1000) != 0	/* USE_MY_METRICS */
		{
		  if get_metrics.unwrap () ((*component).glyph_index as sfnt_glyph_index,
					    metrics_return, dcontext) != 0
		    {
		      if need_free
			{ free_glyph.unwrap () (subglyph, dcontext); }

		      return 1;
		    }

		  /* Refer to the comment above sfnt_decompose_glyph
		     for reasons and manner in which these offsets are
		     applied.  */
		  (*metrics_return).lbearing
		    -= (*subglyph).origin_distortion as sfnt_fixed;
		  (*metrics_return).advance
		    += (*subglyph).advance_distortion as sfnt_fixed;
		}
	    }
	}
      else
	{
	  /* If USE_MY_METRICS, save this subglyph's metrics within
	     sub_metrics; they might be overwritten by metrics for
	     subglyphs of this compound subglyph in turn.  */

	  if ((*component).flags as c_int & 0o1000) != 0	/* USE_MY_METRICS */
	    {
	      if get_metrics.unwrap () ((*component).glyph_index as sfnt_glyph_index,
					&raw mut sub_metrics, dcontext) != 0
		{
		  if need_free
		    { free_glyph.unwrap () (subglyph, dcontext); }

		  return 1;
		}

	      /* Refer to the comment above sfnt_decompose_glyph for
		 reasons and manner in which these offsets are
		 applied.  */
	      sub_metrics.lbearing -= (*subglyph).origin_distortion as sfnt_fixed;
	      sub_metrics.advance += (*subglyph).advance_distortion as sfnt_fixed;
	    }

	  /* Compound subglyph.  Decompose the glyph recursively, and
	     then apply the transform.  */
	  rc = sfnt_decompose_compound_glyph (subglyph,
					      context,
					      &raw mut sub_metrics,
					      get_glyph,
					      free_glyph,
					      get_metrics,
					      recursion_count + 1,
					      dcontext);

	  if rc != 0
	    {
	      if need_free
		{ free_glyph.unwrap () (subglyph, dcontext); }

	      return 1;
	    }

	  if ((*component).flags as c_int & 0o1000) != 0	/* USE_MY_METRICS */
	    /* Save sub_metrics inside *metrics_return as stated
	       above.  */
	    { *metrics_return = sub_metrics; }

	  /* When an anchor point is being used to translate the
	     glyph, and the subglyph in question is actually a
	     compound glyph, it is impossible to know which offset to
	     use until the compound subglyph has actually been loaded.

	     defer_offsets is set to true if these conditions apply,
	     whereupon the offset is calculated here, using the points
	     in the loaded child compound glyph.  */

	  if defer_offsets
	    {
	      'skip_computation_from_defer_offsets: {
		/* Renumber the non renumbered point2 to point into the
		   decomposed component.  */
		point2 += contour_start;

		/* Next, check that the non-renumbered point being
		   anchored lies inside the glyph data that was
		   decomposed.  */

		if point2 >= (*context).num_points
		  {
		    /* POINT2 might fall within the phantom points of
		       that glyph.  */

		    'error_in_defer_offsets: {
		      if point2 - (*context).num_points < 2
			{
			  if get_metrics.unwrap () (
				 (*component).glyph_index as sfnt_glyph_index,
				 &raw mut sub_metrics, dcontext) != 0
			    { break 'error_in_defer_offsets; }

			  /* Derive the phantom points from those metrics.  */
			  f1 = (*glyph).xmin as sfnt_fixed - sub_metrics.lbearing;
			  f2 = f1 + sub_metrics.advance;

			  /* Apply the metrics distortion.  */
			  f1 += (*glyph).origin_distortion as sfnt_fixed;
			  f2 += (*glyph).advance_distortion as sfnt_fixed;

			  /* Get the points and use them to compute the offsets.  */

			  if (point2 - (*context).num_points) == 0
			    { x = f1 * 65536; }
			  else
			    { x = f2 * 65536; }

			  x = *((*context).x_coordinates.offset (point as isize)) - x;
			  y = *((*context).y_coordinates.offset (point as isize));

			  /* X and Y offsets have been ascertained.  */
			  break 'skip_computation_from_defer_offsets;
			}
		    }

		    /* error_in_defer_offsets: */
		    if need_free
		      { free_glyph.unwrap () (subglyph, dcontext); }

		    return 1;
		  }

		/* Get the points and use them to compute the
		   offsets.  */

		xtemp = *((*context).x_coordinates.offset (point as isize));
		ytemp = *((*context).y_coordinates.offset (point as isize));
		x = xtemp - *((*context).x_coordinates.offset (point2 as isize));
		y = ytemp - *((*context).y_coordinates.offset (point2 as isize));
	      }
	      /* skip_computation_from_defer_offsets: */
	    }

	  sfnt_transform_coordinates (component,
				      (*context).x_coordinates.offset (contour_start as isize),
				      (*context).y_coordinates.offset (contour_start as isize),
				      (*context).num_points - contour_start,
				      x, y);
	}

      if need_free
	{ free_glyph.unwrap () (subglyph, dcontext); }

      j += 1;
    }

  /* Decomposition is complete.  CONTEXT now contains the adjusted
     outlines of the entire compound glyph.  */
  return 0;
}

/* Linear-interpolate to a point halfway between the points specified
   by CONTROL1 and CONTROL2.  Put the result in RESULT.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_lerp_half (control1: *mut sfnt_point, control2: *mut sfnt_point,
			  result: *mut sfnt_point)               // C 3150-3156
{
  (*result).x = (*control1).x + (((*control2).x - (*control1).x) / 2);
  (*result).y = (*control1).y + (((*control2).y - (*control1).y) / 2);
}

/* Decompose contour data inside X, Y and FLAGS, between the indices
   HERE and LAST.  Call LINE_TO, CURVE_TO and MOVE_TO as appropriate,
   with DCONTEXT as an argument.  Apply SCALE to each point; SCALE
   should be the factor necessary to turn points into 16.16 fixed
   point.

   Value is 1 upon failure, else 0.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_decompose_glyph_1 (mut here: size_t, mut last: size_t,
				  move_to: sfnt_move_to_proc,
				  line_to: sfnt_line_to_proc,
				  curve_to: sfnt_curve_to_proc,
				  dcontext: *mut c_void,
				  x: *mut sfnt_fword,
				  y: *mut sfnt_fword, flags: *mut c_uchar,
				  scale: c_int)
  -> c_int                                        // C 3166-3293
{
  let mut control1: sfnt_point = std::mem::zeroed ();
  let mut control2: sfnt_point = std::mem::zeroed ();
  let mut start: sfnt_point = std::mem::zeroed ();
  let mut mid: sfnt_point = std::mem::zeroed ();
  let mut i: size_t;

  /* The contour is empty.  */

  if here == last
    /* An empty contour, if redundant, is not necessarily invalid.  */
    { return 0; }

  /* Move the pen to the start of the contour.  Apparently some fonts
     have off the curve points as the start of a contour, so when that
     happens lerp between the first and last points.  */

  if (*flags.offset (here as isize) as c_int & 0o1) != 0	/* On Curve */
    {
      control1.x = (*(x.offset (here as isize)) as c_int) * scale;
      control1.y = (*(y.offset (here as isize)) as c_int) * scale;
      start = control1;
    }
  else if (*flags.offset (last as isize) as c_int & 0o1) != 0
    {
      /* Start at the last point if it is on the curve.  Here, the
         start really becomes the middle of a spline.  */
      control1.x = (*(x.offset (last as isize)) as c_int) * scale;
      control1.y = (*(y.offset (last as isize)) as c_int) * scale;
      start = control1;

      /* Curve back one point early.  */
      last = last.wrapping_sub (1);
      here = here.wrapping_sub (1);
    }
  else
    {
      /* Lerp between the start and the end.  */
      control1.x = (*(x.offset (here as isize)) as c_int) * scale;
      control1.y = (*(y.offset (here as isize)) as c_int) * scale;
      control2.x = (*(x.offset (last as isize)) as c_int) * scale;
      control2.y = (*(y.offset (last as isize)) as c_int) * scale;
      sfnt_lerp_half (&raw mut control1, &raw mut control2, &raw mut start);

      /* In either of these cases, start iterating from just here as
	 opposed to here + 1, since logically the contour now starts
	 from the last curve.  */
      here = here.wrapping_sub (1);
    }

  /* Move to the start.  */
  move_to.unwrap () (start, dcontext);

  /* Now handle each point between here + 1 and last.  */

  'exit: {
    i = here;
    'continue_loop: while { i = i.wrapping_add (1); i <= last }
      {
	/* If the point is on the curve, then draw a line here from the
	   last control point.  */

	if (*flags.offset (i as isize) as c_int & 0o1) != 0
	  {
	    control1.x = (*(x.offset (i as isize)) as c_int) * scale;
	    control1.y = (*(y.offset (i as isize)) as c_int) * scale;

	    line_to.unwrap () (control1, dcontext);

	    /* Move to the next point.  */
	    continue;
	  }

	/* Off the curve points are more interesting.  They are handled
	   one by one, with points in between being interpolated, until
	   either the last point is reached or an on-curve point is
	   processed.  First, load the initial control points.  */

	control1.x = (*(x.offset (i as isize)) as c_int) * scale;
	control1.y = (*(y.offset (i as isize)) as c_int) * scale;

	while { i = i.wrapping_add (1); i <= last }
	  {
	    /* Load this point.  */
	    control2.x = (*(x.offset (i as isize)) as c_int) * scale;
	    control2.y = (*(y.offset (i as isize)) as c_int) * scale;

	    /* If this point is on the curve, curve directly to this
	       point.  */

	    if (*flags.offset (i as isize) as c_int & 0o1) != 0
	      {
		curve_to.unwrap () (control1, control2, dcontext);
		continue 'continue_loop;    /* C's `goto continue_loop` */
	      }

	    /* Calculate the point between here and the previous
	       point.  */
	    sfnt_lerp_half (&raw mut control1, &raw mut control2, &raw mut mid);

	    /* Curve over there.  */
	    curve_to.unwrap () (control1, mid, dcontext);

	    /* Reload the control point.  */
	    control1 = control2;
	  }

	/* Close the contour by curving back to start.  */
	curve_to.unwrap () (control1, start, dcontext);

	/* Don't close the contour twice.  */
	break 'exit;                    /* C's `goto exit` */
      }
  } /* 'exit (C 3291) */

  /* Close the contour with a line back to start.  */
  line_to.unwrap () (start, dcontext);

  /* exit: */
  return 0;
}

/* Decompose contour data inside X, Y and FLAGS, between the indices
   HERE and LAST.  Call LINE_TO, CURVE_TO and MOVE_TO as appropriate,
   with DCONTEXT as an argument.  Apply SCALE to each point; SCALE
   should be the factor necessary to turn points into 16.16 fixed
   point.

   This is the version of sfnt_decompose_glyph_1 which takes
   sfnt_fixed (or sfnt_f26dot6) as opposed to sfnt_fword.

   Value is 1 upon failure, else 0.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_decompose_glyph_2 (mut here: size_t, mut last: size_t,
				  move_to: sfnt_move_to_proc,
				  line_to: sfnt_line_to_proc,
				  curve_to: sfnt_curve_to_proc,
				  dcontext: *mut c_void,
				  x: *mut sfnt_fixed,
				  y: *mut sfnt_fixed, flags: *mut c_uchar,
				  scale: c_int)
  -> c_int                                        // C 3306-3433
{
  let mut control1: sfnt_point = std::mem::zeroed ();
  let mut control2: sfnt_point = std::mem::zeroed ();
  let mut start: sfnt_point = std::mem::zeroed ();
  let mut mid: sfnt_point = std::mem::zeroed ();
  let mut i: size_t;

  /* The contour is empty.  */

  if here == last
    /* An empty contour, if redundant, is not necessarily invalid.  */
    { return 0; }

  /* Move the pen to the start of the contour.  Apparently some fonts
     have off the curve points as the start of a contour, so when that
     happens lerp between the first and last points.  */

  if (*flags.offset (here as isize) as c_int & 0o1) != 0	/* On Curve */
    {
      control1.x = *(x.offset (here as isize)) * scale;
      control1.y = *(y.offset (here as isize)) * scale;
      start = control1;
    }
  else if (*flags.offset (last as isize) as c_int & 0o1) != 0
    {
      /* Start at the last point if it is on the curve.  Here, the
         start really becomes the middle of a spline.  */
      control1.x = *(x.offset (last as isize)) * scale;
      control1.y = *(y.offset (last as isize)) * scale;
      start = control1;

      /* Curve back one point early.  */
      last = last.wrapping_sub (1);
      here = here.wrapping_sub (1);
    }
  else
    {
      /* Lerp between the start and the end.  */
      control1.x = *(x.offset (here as isize)) * scale;
      control1.y = *(y.offset (here as isize)) * scale;
      control2.x = *(x.offset (last as isize)) * scale;
      control2.y = *(y.offset (last as isize)) * scale;
      sfnt_lerp_half (&raw mut control1, &raw mut control2, &raw mut start);

      /* In either of these cases, start iterating from just here as
	 opposed to here + 1, since logically the contour now starts
	 from the last curve.  */
      here = here.wrapping_sub (1);
    }

  /* Move to the start.  */
  move_to.unwrap () (start, dcontext);

  /* Now handle each point between here + 1 and last.  */

  'exit: {
    i = here;
    'continue_loop: while { i = i.wrapping_add (1); i <= last }
      {
	/* If the point is on the curve, then draw a line here from the
	   last control point.  */

	if (*flags.offset (i as isize) as c_int & 0o1) != 0
	  {
	    control1.x = *(x.offset (i as isize)) * scale;
	    control1.y = *(y.offset (i as isize)) * scale;

	    line_to.unwrap () (control1, dcontext);

	    /* Move to the next point.  */
	    continue;
	  }

	/* Off the curve points are more interesting.  They are handled
	   one by one, with points in between being interpolated, until
	   either the last point is reached or an on-curve point is
	   processed.  First, load the initial control points.  */

	control1.x = *(x.offset (i as isize)) * scale;
	control1.y = *(y.offset (i as isize)) * scale;

	while { i = i.wrapping_add (1); i <= last }
	  {
	    /* Load this point.  */
	    control2.x = *(x.offset (i as isize)) * scale;
	    control2.y = *(y.offset (i as isize)) * scale;

	    /* If this point is on the curve, curve directly to this
	       point.  */

	    if (*flags.offset (i as isize) as c_int & 0o1) != 0
	      {
		curve_to.unwrap () (control1, control2, dcontext);
		continue 'continue_loop;    /* C's `goto continue_loop` */
	      }

	    /* Calculate the point between here and the previous
	       point.  */
	    sfnt_lerp_half (&raw mut control1, &raw mut control2, &raw mut mid);

	    /* Curve over there.  */
	    curve_to.unwrap () (control1, mid, dcontext);

	    /* Reload the control point.  */
	    control1 = control2;
	  }

	/* Close the contour by curving back to start.  */
	curve_to.unwrap () (control1, start, dcontext);

	/* Don't close the contour twice.  */
	break 'exit;                    /* C's `goto exit` */
      }
  } /* 'exit (C 3431) */

  /* Close the contour with a line back to start.  */
  line_to.unwrap () (start, dcontext);

  /* exit: */
  return 0;
}

/* Decompose GLYPH into its individual components.  Call MOVE_TO to
   move to a specific location.  For each line encountered, call
   LINE_TO to draw a line to that location.  For each spline
   encountered, call CURVE_TO to draw the curves comprising the
   spline.

   If GLYPH is compound, use GET_GLYPH to obtain subglyphs.  PROC must
   return whether or not FREE_GLYPH will be called with the glyph
   after sfnt_decompose_glyph is done with it.  If GLYPH moreover
   incorporates components whose anchor points are phantom points, use
   GET_METRICS to obtain glyph metrics prerequisite for establishing
   their coordinates.

   When glyphs originate from a GX font with an active set of
   transforms, the correct manner of applying such transforms is to
   apply them within GET_GLYPH, while returning unaltered metrics from
   GET_METRICS.

   If there is a component glyph within GLYPH whose metrics have been
   indicated as replacing those of its parent glyph, the variable
   *METRICS_RETURN will be set to its metrics with GX-induced offsets
   applied.

   *METRICS_RETURN must initially hold metrics with GX offsets
   applied, if any.

   All functions will be called with DCONTEXT as an argument.

   The winding rule used to fill the resulting lines is described in
   chapter 2 of the TrueType reference manual, under the heading
   "distinguishing the inside from the outside of a glyph."

   Value is 0 upon success, or some non-zero value upon failure, which
   can happen if the glyph is invalid.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_decompose_glyph (glyph: *mut sfnt_glyph,
				metrics_return: *mut sfnt_glyph_metrics,
				move_to: sfnt_move_to_proc,
				line_to: sfnt_line_to_proc,
				curve_to: sfnt_curve_to_proc,
				get_glyph: sfnt_get_glyph_proc,
				free_glyph: sfnt_free_glyph_proc,
				get_metrics: sfnt_get_metrics_proc,
				dcontext: *mut c_void)
  -> c_int                                        // C 3470-3594
{
  let mut here: size_t;
  let mut last: size_t;
  let mut n: size_t;
  let mut context: sfnt_compound_glyph_context = std::mem::zeroed ();
  let mut compound_metrics: sfnt_glyph_metrics;

  if !(*glyph).simple.is_null ()
    {
      if (*glyph).number_of_contours == 0
	/* No contours.  Nothing needs to be decomposed.  */
	{ return 0; }

      here = 0;

      n = 0;
      while n < (*glyph).number_of_contours as size_t
	{
	  /* here is the first index into the glyph's point arrays
	     belonging to the contour in question.  last is the index
	     of the last point in the contour.  */
	  last = *((*(*glyph).simple).end_pts_of_contours.offset (n as isize)) as size_t;

	  /* Make sure here and last make sense.  */

	  if here > last || last >= (*(*glyph).simple).number_of_points
	    { return 1; }

	  /* Now perform the decomposition.  */
	  if sfnt_decompose_glyph_1 (here, last, move_to,
				     line_to, curve_to,
				     dcontext,
				     (*(*glyph).simple).x_coordinates,
				     (*(*glyph).simple).y_coordinates,
				     (*(*glyph).simple).flags,
				     65536) != 0
	    { return 1; }

	  /* Move forward to the start of the next contour.  */
	  here = last + 1;
	  n += 1;
	}

      return 0;
    }

  /* Decompose the specified compound glyph.  */
  std::ptr::write_bytes (&raw mut context as *mut u8, 0,
			 std::mem::size_of_val (&context));

  /* Rather than handing METRICS_RETURN over to
     sfnt_decompose_compound_glyph, save metrics within a temporary
     variable and postpone returning them until it is certain the
     decomposition has succeeded.  */

  compound_metrics = *metrics_return;

  'early: {
    if sfnt_decompose_compound_glyph (glyph, &raw mut context,
				      &raw mut compound_metrics,
				      get_glyph, free_glyph,
				      get_metrics, 0,
				      dcontext) != 0
      {
	xfree (context.x_coordinates as *mut c_void);
	xfree (context.y_coordinates as *mut c_void);
	xfree (context.flags as *mut c_void);
	xfree (context.contour_end_points as *mut c_void);

	return 1;
      }

    *metrics_return = compound_metrics;

    /* Now, generate the outlines.  */

    if context.num_end_points == 0
      /* No contours.  */
      { break 'early; }

    here = 0;

    'fail: {
      n = 0;
      while n < context.num_end_points
	{
	  /* here is the first index into the glyph's point arrays
	     belonging to the contour in question.  last is the index
	     of the last point in the contour.  */
	  last = *context.contour_end_points.offset (n as isize);

	  /* Make sure here and last make sense.  */

	  if here > last || last >= context.num_points
	    { break 'fail; }

	  /* Now perform the decomposition.  */
	  if sfnt_decompose_glyph_2 (here, last, move_to,
				     line_to, curve_to,
				     dcontext,
				     context.x_coordinates,
				     context.y_coordinates,
				     context.flags, 1) != 0
	    { break 'fail; }

	  /* Move forward.  */
	  here = last + 1;
	  n += 1;
	}
    }
  } /* 'early (C 3581) */

  /* early: */
  xfree (context.x_coordinates as *mut c_void);
  xfree (context.y_coordinates as *mut c_void);
  xfree (context.flags as *mut c_void);
  xfree (context.contour_end_points as *mut c_void);
  return 0;

  /* fail: */
  xfree (context.x_coordinates as *mut c_void);
  xfree (context.y_coordinates as *mut c_void);
  xfree (context.flags as *mut c_void);
  xfree (context.contour_end_points as *mut c_void);
  return 1;
}

/* C 3595-3610: the record the outline builders communicate through, and the global state object
   `build_outline_context` that `extract-abi.py`'s inventory still listed as missing at the end of
   slice 3.  This opens the section the ledger's prose calls "the outline builders and the
   rasterizer" (C 3644-6426).  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct sfnt_build_glyph_outline_context
{
  /* The outline being built.  */
  pub outline: *mut sfnt_glyph_outline,

  /* Factor to multiply positions by to get the pixel width.  */
  pub factor: sfnt_fixed,

  /* The position of the pen in 16.16 fixed point format.  */
  pub x: sfnt_fixed,
  pub y: sfnt_fixed,
}

/* Global state for sfnt_build_glyph_outline and related
   functions.  */
/* C 3610's definition carries no initialiser - a tentative definition, i.e. zero - so the port
   spells it `zeroed ()` (the android.rs 458 reading of the same C shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut build_outline_context: sfnt_build_glyph_outline_context
  = unsafe { std::mem::zeroed () };

/* Append the given three words FLAGS, X, and Y to the outline
   currently being built.  Value is the new pointer to outline
   memory.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_build_append (flags: c_int, x: sfnt_fixed, y: sfnt_fixed)
  -> *mut sfnt_glyph_outline                       // C 3616-3674
{
  let mut outline: *mut sfnt_glyph_outline;

  outline = build_outline_context.outline;

  if x == build_outline_context.x
      && y == build_outline_context.y
      /* If the outline is presently empty, the first move_to must be
	 recorded even if its X and Y are set to origin.  Without this
	 initial vertex, edges will be generated from the next vertex
	 onward, and thus be misaligned.  */
      && (*outline).outline_used != 0
    /* Ignore redundant motion.  */
    { return build_outline_context.outline; }

  (*outline).outline_used += 1;

  /* See if the outline has to be extended.  */

  if (*outline).outline_used > (*outline).outline_size
    {
      /* This can't overflow, as the old value did not overflow when
	 multiplied by sizeof *outline->outline.  */
      (*outline).outline_size = (*outline).outline_used * 2;

      /* Extend the outline to some size past the new size.  */
      let mut size: size_t;
      let (ckd_size, mul_overflow)
	= (*outline).outline_size
	  .overflowing_mul (std::mem::size_of::<sfnt_glyph_outline_command> ());
      size = ckd_size;
      if mul_overflow
	{ memory_full_up (); }
      outline = xaddrealloc (outline as *mut c_void,
			     std::mem::size_of::<sfnt_glyph_outline> (),
			     size) as *mut sfnt_glyph_outline;
      (*outline).outline
	= outline.offset (1) as *mut sfnt_glyph_outline_command;
    }

  /* Write the outline data.  */
  (*(*outline).outline.offset (((*outline).outline_used - 1) as isize)).flags = flags;
  (*(*outline).outline.offset (((*outline).outline_used - 1) as isize)).x = x;
  (*(*outline).outline.offset (((*outline).outline_used - 1) as isize)).y = y;

  /* Extend outline bounding box.  */

  if (*outline).outline_used == 1
    {
      /* These are the first points in the outline.  */
      (*outline).xmax = x;
      (*outline).xmin = x;
      (*outline).ymax = y;
      (*outline).ymin = y;
    }
  else
    {
      (*outline).xmin = MIN! (x as sfnt_fixed, (*outline).xmin);
      (*outline).ymin = MIN! (y as sfnt_fixed, (*outline).ymin);
      (*outline).xmax = MAX! (x as sfnt_fixed, (*outline).xmax);
      (*outline).ymax = MAX! (y as sfnt_fixed, (*outline).ymax);
    }

  return outline;
}

/* Calculate (A * B) / C with no rounding and return the result.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_multiply_divide (a: c_uint, b: c_uint, c: c_uint) -> c_uint
{                                                          // C 3678-3682
  return (a as uint64_t * b as uint64_t / c as uint64_t) as c_uint;
}

/* Calculate (A * B) / C with rounding and return the result.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_multiply_divide_rounded (a: c_uint, b: c_uint, c: c_uint) -> c_uint
{                                                          // C 3686-3691
  return ((a as uint64_t * b as uint64_t + (c / 2) as uint64_t)
	  / c as uint64_t) as c_uint;
}

/* The same as sfnt_multiply_divide_rounded, but handle signed values
   instead.  */
/* C 3696's `MAYBE_UNUSED` is `attribute.h`'s attribute, a no-op the port drops (the fileio.rs 273
   reading of the same attribute); the `macro-body-notes.tsv` row it owes is named in the ledger's
   Owed 3.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_multiply_divide_signed (a: c_int, b: c_int, c: c_int) -> c_int
{                                                          // C 3696-3714
  let mut sign: c_int;

  sign = 1;

  if a < 0
    { sign = -sign; }

  if b < 0
    { sign = -sign; }

  if c < 0
    { sign = -sign; }

  return sfnt_multiply_divide_rounded (abs (a) as c_uint, abs (b) as c_uint,
				       abs (c) as c_uint) as c_int * sign;
}

/* Multiply the two 16.16 fixed point numbers X and Y.  Return the
   result regardless of overflow.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_mul_fixed (x: sfnt_fixed, y: sfnt_fixed) -> sfnt_fixed
{                                                          // C 3719-3723
  return (x as int64_t * y as int64_t / ((1 << 16) as int64_t)) as sfnt_fixed;
}

/* Multiply the two 16.16 fixed point numbers X and Y, with rounding
   of the result.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_mul_fixed_round (x: sfnt_fixed, y: sfnt_fixed) -> sfnt_fixed
{                                                          // C 3728-3733
  let mut product: int64_t = x as int64_t * y as int64_t;
  return ((product + (if product < 0 { -(1 << 15) } else { 1 << 15 }) as int64_t)
	  / ((1 << 16) as int64_t)) as sfnt_fixed;
}

/* Set the pen size to the specified point and return.  POINT will be
   scaled up to the pixel size.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_move_to_and_build (point: sfnt_point, dcontext: *mut c_void)
{                                                          // C 3738-3749
  let mut x: sfnt_fixed;
  let mut y: sfnt_fixed;

  x = sfnt_mul_fixed (build_outline_context.factor, point.x);
  y = sfnt_mul_fixed (build_outline_context.factor, point.y);

  build_outline_context.outline = sfnt_build_append (0, x, y);
  build_outline_context.x = x;
  build_outline_context.y = y;
}

/* Record a line to the specified point and return.  POINT will be
   scaled up to the pixel size.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_line_to_and_build (point: sfnt_point, dcontext: *mut c_void)
{                                                          // C 3754-3767
  let mut x: sfnt_fixed;
  let mut y: sfnt_fixed;

  x = sfnt_mul_fixed (build_outline_context.factor, point.x);
  y = sfnt_mul_fixed (build_outline_context.factor, point.y);

  build_outline_context.outline
    = sfnt_build_append (SFNT_GLYPH_OUTLINE_LINETO,
			 x, y);
  build_outline_context.x = x;
  build_outline_context.y = y;
}

/* Divide the two 16.16 fixed point numbers X and Y.  Return the
   result regardless of overflow.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_div_fixed (x: sfnt_fixed, y: sfnt_fixed) -> sfnt_fixed
{                                                          // C 3772-3776
  return (x as int64_t * ((1 << 16) as int64_t) / y as int64_t) as sfnt_fixed;
}

/* Return the ceiling value of the specified fixed point number X.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_ceil_fixed (x: sfnt_fixed) -> sfnt_fixed
{                                                          // C 3780-3784
  return ((x + 0o177777) as uint32_t & 0o37777600000) as sfnt_fixed;
}

/* Return the floor value of the specified fixed point number X.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_floor_fixed (x: sfnt_fixed) -> sfnt_fixed
{                                                          // C 3788-3792
  return (x as uint32_t & 0o37777600000) as sfnt_fixed;
}

/* Given a curve consisting of three points CONTROL0, CONTROL1 and
   ENDPOINT, return whether or not the curve is sufficiently small to
   be approximated by a line between CONTROL0 and ENDPOINT.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_curve_is_flat (control0: sfnt_point, control1: sfnt_point,
			      endpoint: sfnt_point)
  -> bool                                                  // C 3798-3815
{
  let mut g: sfnt_point = std::mem::zeroed ();
  let mut h: sfnt_point = std::mem::zeroed ();

  g.x = control1.x - control0.x;
  g.y = control1.y - control0.y;
  h.x = endpoint.x - control0.x;
  h.y = endpoint.y - control0.y;

  /* 1.0 is a constant representing the area covered at which point
     the curve is considered ``flat''.  */
  return (abs (sfnt_mul_fixed (g.x, h.y)
	       - sfnt_mul_fixed (g.y, h.x))
	  <= 0o200000);
}

/* Recursively split the splines in the bezier curve formed from
   CONTROL0, CONTROL1 and ENDPOINT until the area between the curve's
   two ends is small enough to be considered ``flat''.  Then, turn
   those ``flat'' curves into lines.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_curve_to_and_build_1 (mut control0: sfnt_point, mut control1: sfnt_point,
				     mut endpoint: sfnt_point)
{                                                          // C 3822-3854
  let mut ab: sfnt_point = std::mem::zeroed ();
  let mut bc: sfnt_point = std::mem::zeroed ();
  let mut abbc: sfnt_point = std::mem::zeroed ();

  /* control0, control and endpoint make up the spline.  Figure out
     its distance from a line.  */
  if sfnt_curve_is_flat (control0, control1, endpoint)
    {
      /* Draw a line to endpoint.  */
      build_outline_context.outline
	= sfnt_build_append (SFNT_GLYPH_OUTLINE_LINETO,
			     endpoint.x, endpoint.y);
      build_outline_context.x = endpoint.x;
      build_outline_context.y = endpoint.y;
    }
  else
    {
      /* Calculate new control points.
	 Maybe apply a recursion limit here? */
      sfnt_lerp_half (&raw mut control0, &raw mut control1, &raw mut ab);
      sfnt_lerp_half (&raw mut control1, &raw mut endpoint, &raw mut bc);
      sfnt_lerp_half (&raw mut ab, &raw mut bc, &raw mut abbc);

      /* Keep splitting until a flat enough spline results.  */
      sfnt_curve_to_and_build_1 (control0, ab, abbc);

      /* Then go on with the spline between control1 and endpoint.  */
      sfnt_curve_to_and_build_1 (abbc, bc, endpoint);
    }
}

/* Scale and decompose the specified bezier curve into individual
   lines.  Then, record each of those lines into the outline being
   built.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_curve_to_and_build (mut control: sfnt_point,
					      mut endpoint: sfnt_point,
					      dcontext: *mut c_void)
{                                                          // C 3860-3879
  let mut control0: sfnt_point = std::mem::zeroed ();

  control0.x = build_outline_context.x;
  control0.y = build_outline_context.y;
  control.x = sfnt_mul_fixed (control.x,
			      build_outline_context.factor);
  control.y = sfnt_mul_fixed (control.y,
			      build_outline_context.factor);
  endpoint.x = sfnt_mul_fixed (endpoint.x,
			       build_outline_context.factor);
  endpoint.y = sfnt_mul_fixed (endpoint.y,
			       build_outline_context.factor);

  sfnt_curve_to_and_build_1 (control0, control, endpoint);
}

/* Non-reentrantly build the outline for the specified GLYPH at the
   given scale factor.  Return the outline data with a refcount of 0
   upon success, or NULL upon failure.

   SCALE is a scale factor that converts between em space and device
   space.

   Use the unscaled glyph METRICS to determine the origin point of the
   outline, or those of compound glyph components within *GLYPH
   configured to replace their parents', which if existent are
   returned in *METRICS.  METRICS should not be altered by GX-derived
   offsets, as they will be applied to *METRICS if present, following
   this formula:

     LBEARING = LBEARING - GLYPH->origin_distortion
     ADVANCE = ADVANCE + GLYPH->advance_distortion

   Call GET_GLYPH and FREE_GLYPH with the specified DCONTEXT to obtain
   glyphs for compound glyph subcomponents, and GET_METRICS with the
   provided DCONTEXT for unscaled glyph metrics.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_build_glyph_outline (glyph: *mut sfnt_glyph,
						   scale: sfnt_fixed,
						   metrics: *mut sfnt_glyph_metrics,
						   get_glyph: sfnt_get_glyph_proc,
						   free_glyph: sfnt_free_glyph_proc,
						   get_metrics: sfnt_get_metrics_proc,
						   dcontext: *mut c_void)
  -> *mut sfnt_glyph_outline                              // C 3902-3971
{
  let mut outline: *mut sfnt_glyph_outline;
  let mut rc: c_int;
  let mut origin: sfnt_fword;

  std::ptr::write_bytes (&raw mut build_outline_context as *mut u8, 0,
			 std::mem::size_of_val (&build_outline_context));

  /* Allocate the outline now with enough for 44 words at the end.  */
  outline = xmalloc (std::mem::size_of::<sfnt_glyph_outline> ()
		     + 40 * std::mem::size_of::<sfnt_glyph_outline_command> ())
    as *mut sfnt_glyph_outline;
  (*outline).outline_size = 40;
  (*outline).outline_used = 0;
  (*outline).refcount = 0;
  (*outline).outline
    = outline.offset (1) as *mut sfnt_glyph_outline_command;

  /* DCONTEXT will be passed to GET_GLYPH and FREE_GLYPH, so global
     variables must be used to communicate with the decomposition
     functions.  */
  build_outline_context.outline = outline;

  /* Clear outline bounding box.  */
  (*outline).xmin = 0;
  (*outline).ymin = 0;
  (*outline).xmax = 0;
  (*outline).ymax = 0;

  /* Set the scale factor.  */
  build_outline_context.factor = scale;

  /* Apply the glyph's advance and origin distortion to METRICS in
     advance of constructing the glyph outline, which might replace
     METRICS with the metrics of a compound subglyph.  */
  (*metrics).lbearing -= (*glyph).origin_distortion as sfnt_fixed;
  (*metrics).advance += (*glyph).advance_distortion as sfnt_fixed;

  /* Decompose the outline.  */
  rc = sfnt_decompose_glyph (glyph, metrics,
			     Some (sfnt_move_to_and_build),
			     Some (sfnt_line_to_and_build),
			     Some (sfnt_curve_to_and_build),
			     get_glyph, free_glyph, get_metrics,
			     dcontext);

  /* Synchronize the outline object with what might have changed
     inside sfnt_decompose_glyph.  */
  outline = build_outline_context.outline;

  if rc != 0
    {
      xfree (outline as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Compute the origin position.  Note that the original glyph xmin
     is first used to calculate the origin point, and the origin
     distortion is applied to it to get the distorted origin.  */

  origin = ((*glyph).xmin as sfnt_fixed - (*metrics).lbearing) as sfnt_fword;
  (*outline).origin = sfnt_mul_fixed (origin as sfnt_fixed, scale);

  return outline;
}


/* Glyph rasterization.  The algorithm used here is fairly simple.
   Each contour is decomposed into lines, which turn into a polygon.
   Then, a bog standard edge filler is used to turn them into
   spans.  */

/* Coverage table.  This is a four dimensional array indiced by the Y,
   then X axis fractional, shifted down to 2 bits.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static sfnt_poly_coverage: [[c_uchar; 9]; 8] = [
  [0, 4, 8, 12, 16, 20, 24, 28, 32,],
  [0, 4, 8, 12, 16, 20, 24, 28, 32,],
  [0, 4, 8, 12, 16, 20, 24, 28, 32,],
  [0, 3, 7, 11, 15, 19, 23, 27, 31,],
  [0, 4, 8, 12, 16, 20, 24, 28, 32,],
  [0, 4, 8, 12, 16, 20, 24, 28, 32,],
  [0, 4, 8, 12, 16, 20, 24, 28, 32,],
  [0, 4, 8, 12, 16, 20, 24, 28, 32,],
];                                                         // C 3983-3993

/* Return the nearest coordinate on the sample grid no less than
   F.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_poly_grid_ceil (f: sfnt_fixed) -> sfnt_fixed
{                                                          // C 3998-4003
  return (((f + (SFNT_POLY_START - 1))
	   & !(SFNT_POLY_STEP - 1)) + SFNT_POLY_START);
}

/* C 4005-4008: C's anonymous enum, so the port declares the same one constant (`c_int`); it is
   the third of the five constants the checker listed as missing after slice 3.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SFNT_POLY_ALIGNMENT: c_int = 4;

/* Initialize the specified RASTER in preparation for displaying spans
   for OUTLINE, and set RASTER->refcount to 0.  The caller must then
   set RASTER->cells to a zeroed array of size RASTER->stride *
   RASTER->height, aligned to RASTER.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_prepare_raster (raster: *mut sfnt_raster,
					      outline: *mut sfnt_glyph_outline)
{                                                          // C 4015-4038
  (*raster).width
    = ((sfnt_ceil_fixed ((*outline).xmax)
	- sfnt_floor_fixed ((*outline).xmin)) / 65536) as c_ushort;
  (*raster).height
    = ((sfnt_ceil_fixed ((*outline).ymax)
	- sfnt_floor_fixed ((*outline).ymin)) / 65536) as c_ushort;
  (*raster).refcount = 0;

  /* Align the raster to a SFNT_POLY_ALIGNMENT byte boundary.  */
  (*raster).stride
    = (((*raster).width as c_int
	+ (SFNT_POLY_ALIGNMENT - 1))
       & !(SFNT_POLY_ALIGNMENT - 1)) as c_ushort;

  /* Apply outline->origin.  This is 0 by convention in most fonts.
     However, variable fonts typically change this as variations are
     applied.  */
  (*raster).offx
    = (sfnt_floor_fixed ((*outline).xmin
			 - (*outline).origin) / 65536) as c_short;
  (*raster).offy
    = (sfnt_floor_fixed ((*outline).ymin) / 65536) as c_short;
}

/* C 4040-4042: C's two function-pointer typedefs for the edge and span consumers.  The port's
   bare-pointer-to-`Option` spelling is P-27's shape (the mirror's `sfnt_move_to_proc` family).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type sfnt_edge_proc =
  Option<unsafe extern "C" fn (*mut sfnt_edge, size_t, *mut c_void)>;   // C 4040-4041
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type sfnt_span_proc =
  Option<unsafe extern "C" fn (*mut sfnt_edge, sfnt_fixed, *mut c_void)>; // C 4042

/* Move EDGE->x forward, assuming that the scanline has moved upwards
   by SFNT_POLY_STEP.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_step_edge (edge: *mut sfnt_edge)
{                                                          // C 4047-4052
  /* Add step.  */
  (*edge).x += (*edge).step_x;
}

/* Build a list of edges for each contour in OUTLINE, applying xmin
   and ymin as the offset to each edge.  Call EDGE_PROC with DCONTEXT
   and the resulting edges as arguments.  It is OK to modify the edges
   given to EDGE_PROC.  Align all edges to the sub-pixel grid.  */

/* C 4068's `void *edges_alloc = NULL;` and C 4070's `alloca`: the port has no `alloca`, so the
   request goes to the stack pool `AVAIL_ALLOCA!`/`SAFE_ALLOCA!` allocate from (PORTING.md P-05,
   the alloc.rs 7936 / pdumper.rs 2241 shape); C's `edges_alloc` stays `NULL` in that arm, so
   C 4166's `xfree (edges_alloc)` is the same no-op.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_build_outline_edges (outline: *mut sfnt_glyph_outline,
				    edge_proc: sfnt_edge_proc, dcontext: *mut c_void)
{                                                          // C 4059-4167
  let mut edges: *mut sfnt_edge;
  let mut i: size_t;
  let mut edge: size_t;
  let mut next_vertex: size_t;
  let mut dx: sfnt_fixed;
  let mut dy: sfnt_fixed;
  let mut bot: sfnt_fixed;
  let mut step_x: sfnt_fixed;
  let mut ymin: sfnt_fixed;
  let mut xmin: sfnt_fixed;
  let mut top: size_t;
  let mut bottom: size_t;
  let mut y: size_t;

  let mut edges_alloc: *mut c_void = std::ptr::null_mut ();
  if (*outline).outline_used < 1024 * 16 / std::mem::size_of::<sfnt_edge> ()
    { edges = sa_stack_alloc ((*outline).outline_used
			      * std::mem::size_of::<sfnt_edge> ()) as *mut sfnt_edge; }
  else
    {
      edges_alloc = xnmalloc ((*outline).outline_used as isize,
			      std::mem::size_of::<sfnt_edge> () as isize);
      edges = edges_alloc as *mut sfnt_edge;
    }

  edge = 0;

  /* ymin and xmin must be the same as the offset used to set offy and
     offx in rasters.  */
  ymin = sfnt_floor_fixed ((*outline).ymin);
  xmin = sfnt_floor_fixed ((*outline).xmin);

  i = 0;
  while i < (*outline).outline_used
    {
      /* Set NEXT_VERTEX to the next point (vertex) in this contour.

	 If i is past the end of the contour, then don't build edges
	 for this point.  */
      next_vertex = i + 1;

      if next_vertex == (*outline).outline_used
	  || ((*(*outline).outline.offset (next_vertex as isize)).flags
	      & SFNT_GLYPH_OUTLINE_LINETO) == 0
	{ i += 1; continue; }

      /* Skip past horizontal vertices.  */
      if (*(*outline).outline.offset (next_vertex as isize)).y
	== (*(*outline).outline.offset (i as isize)).y
	{ i += 1; continue; }

      /* Figure out the winding direction.  */
      if (*(*outline).outline.offset (next_vertex as isize)).y
	< (*(*outline).outline.offset (i as isize)).y
	/* Vector will cross imaginary ray from its bottom from the
	   left of the ray.  Winding is thus 1.  */
	{ (*(edges.offset (edge as isize))).winding = 1; }
      else
	/* Moving clockwise.  Winding is thus -1.  */
	{ (*(edges.offset (edge as isize))).winding = -1; }

      /* Figure out the top and bottom values of this edge.  If the
	 next edge is below, top is here and bot is the edge below.
	 If the next edge is above, then top is there and this is the
	 bottom.  */

      if (*(*outline).outline.offset (next_vertex as isize)).y
	< (*(*outline).outline.offset (i as isize)).y
	{
	  /* End of edge is below this one (keep in mind this is a
	     cartesian coordinate system, so smaller values are below
	     larger ones.) */
	  top = i;
	  bottom = next_vertex;
	}
      else
	{
	  /* End of edge is above this one.  */
	  bottom = i;
	  top = next_vertex;
	}

      bot = ((*(*outline).outline.offset (bottom as isize)).y - ymin);
      (*(edges.offset (edge as isize))).top
	= ((*(*outline).outline.offset (top as isize)).y - ymin);

      /* Record the edge.  Rasterization happens from bottom to
	 up, so record the X at the bottom.  */
      (*(edges.offset (edge as isize))).x
	= ((*(*outline).outline.offset (bottom as isize)).x - xmin);
      dx = ((*(*outline).outline.offset (top as isize)).x
	    - (*(*outline).outline.offset (bottom as isize)).x);
      dy = abs ((*(*outline).outline.offset (top as isize)).y
		- (*(*outline).outline.offset (bottom as isize)).y);

      /* Step to first grid point.  */
      y = sfnt_poly_grid_ceil (bot) as size_t;

      /* If rounding would make the edge not cover any area, skip this
	 edge. */

      if y >= (*(edges.offset (edge as isize))).top as size_t
	{ i += 1; continue; }

      /* Compute the step X.  This is how much X changes for each
	 increase in Y.  */
      step_x = sfnt_div_fixed (dx, dy);
      (*(edges.offset (edge as isize))).next = std::ptr::null_mut ();

      /* Compute the step X scaled to the poly step.  */
      (*(edges.offset (edge as isize))).step_x
	= sfnt_mul_fixed (step_x, SFNT_POLY_STEP);

      /* Step to the grid point.  */
      (*(edges.offset (edge as isize))).x
	+= sfnt_mul_fixed (step_x, bot.wrapping_sub (y as sfnt_fixed));

      /* Set the bottom position.  */
      (*(edges.offset (edge as isize))).bottom = y as sfnt_fixed;

      edge += 1;
      i += 1;
    }

  if edge != 0
    { edge_proc.unwrap () (edges, edge, dcontext); }
  xfree (edges_alloc);
}

/* Sort an array of SIZE edges to increase by bottom Y position, in
   preparation for building spans.

   Insertion sort is used because there are usually not very many
   edges, and anything larger would bloat up the code.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_edge_sort (edges: *mut sfnt_edge, size: size_t)
{                                                          // C 4175-4194
  let mut i: ssize_t;
  let mut j: ssize_t;
  let mut edge: sfnt_edge;

  i = 1;
  while (i as size_t) < size
    {
      edge = *(edges.offset (i as isize));
      j = i - 1;

      while j >= 0
	&& (*(edges.offset (j as isize))).bottom > edge.bottom
	{
	  *(edges.offset ((j + 1) as isize)) = *(edges.offset (j as isize));
	  j -= 1;
	}

      *(edges.offset ((j + 1) as isize)) = edge;
      i += 1;
    }
}

/* Draw EDGES, an unsorted array of polygon edges of size SIZE.  For
   each scanline, call SPAN_FUNC with a list of active edges and
   coverage information, and DCONTEXT.

   Sort each edge in ascending order by the bottommost Y coordinate to
   which it applies.  Start a loop on the Y coordinate, which starts
   out at that of the bottommost edge.  For each iteration, add edges
   that now overlap with Y, keeping them sorted by X.  Poly those
   edges through SPAN_FUNC.  Then, move upwards by SFNT_POLY_STEP,
   remove edges that no longer apply, and interpolate the remaining
   edges' X coordinates.  Repeat until all the edges have been polyed.

   Or alternatively, think of this as such: each edge is actually a
   vector from its bottom position towards its top most position.
   Every time Y moves upwards, the position of each edge intersecting
   with Y is interpolated and added to a list of spans along with
   winding information that is then given to EDGE_FUNC.

   Anti-aliasing is performed using a coverage map for fractional
   coordinates, and incrementing the Y axis by SFNT_POLY_STEP instead
   of 1.  SFNT_POLY_STEP is chosen to always keep Y aligned to a grid
   placed such that there are always 1 << SFNT_POLY_SHIFT positions
   available for each integral pixel coordinate.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_poly_edges (edges: *mut sfnt_edge, size: size_t,
			   span_func: sfnt_span_proc, dcontext: *mut c_void)
{                                                          // C 4220-4296
  let mut y: sfnt_fixed;
  let mut e: size_t;
  let mut active: *mut sfnt_edge;
  let mut prev: *mut *mut sfnt_edge;
  let mut a: *mut sfnt_edge;
  let mut n: *mut sfnt_edge = std::ptr::null_mut ();

  if size == 0
    { return; }

  /* Sort edges to ascend by Y-order.  Once again, remember: cartesian
     coordinates.  */
  sfnt_edge_sort (edges, size);

  /* Step down line by line.  Find active edges.  */

  y = (*(edges.offset (0))).bottom;
  active = std::ptr::null_mut ();
  e = 0;

  while true
    {
      /* Add in new edges keeping them sorted.  */
      while e < size && (*(edges.offset (e as isize))).bottom <= y
	{
	  /* Find where to place this edge.  */
	  prev = &raw mut active;
	  while { a = *prev; !a.is_null () }
	    {
	      if (*a).x > (*(edges.offset (e as isize))).x
		{ break; }
	      prev = &raw mut (*a).next;
	    }

	  (*(edges.offset (e as isize))).next = *prev;
	  *prev = edges.offset (e as isize);
	  e += 1;
	}

      /* Draw this span at the current position.  Y axis antialiasing
	 is expected to be handled by SPAN_FUNC.  */
      span_func.unwrap () (active, y, dcontext);

      /* Compute the next Y position.  */
      y += SFNT_POLY_STEP;

      /* Strip out edges that no longer have effect.  */

      prev = &raw mut active;
      while { a = *prev; !a.is_null () }
	{
	  if (*a).top <= y
	    { *prev = (*a).next; }
	  else
	    { prev = &raw mut (*a).next; }
	}

      /* Break if all is done.  */
      if active.is_null () && e == size
	{ break; }

      /* Step all edges.  */
      a = active;
      while !a.is_null ()
	{
	  sfnt_step_edge (a);
	  a = (*a).next;
	}

      /* Resort on X axis.  */
      prev = &raw mut active;
      while { a = *prev; !a.is_null () && { n = (*a).next; !n.is_null () } }
	{
	  if (*a).x > (*n).x
	    {
	      (*a).next = (*n).next;
	      (*n).next = a;
	      *prev = n;
	      prev = &raw mut active;
	    }
	  else
	    { prev = &raw mut (*a).next; }
	}
    }
}

/* Saturate and convert the given unsigned short value X to an
   unsigned char.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_saturate_short (x: c_ushort) -> c_uchar
{                                                          // C 4301-4308
  if x > 255
    { return 255; }

  return x as c_uchar;
}

/* Fill a single span of pixels between X0 and X1 at Y, a raster
   coordinate, onto RASTER.  */

/* C 4322-4324's `#ifndef NDEBUG` block (`row_end` and every `eassert`) is carried under
   `feature = "no_ndebug"`: the frozen configuration defines `NDEBUG`, so that arm is dead text
   (P-51's rule that the feature names the non-frozen side - the `no_signbit`/`no_rint` shape); the
   feature needs registering, which is the ledger's Owed.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_fill_span (raster: *mut sfnt_raster, y: sfnt_fixed,
			  mut x0: sfnt_fixed, mut x1: sfnt_fixed)
{                                                          // C 4313-4435
  let mut start: *mut u8;
  let mut coverage: *const u8;
  let mut left: sfnt_fixed;
  let mut right: sfnt_fixed;
  let mut end: sfnt_fixed;
  let mut w: c_ushort;
  let mut a: c_ushort;
  let mut row: c_int;
  #[cfg (feature = "no_ndebug")]
  let mut row_end: *mut u8 = std::ptr::null_mut ();

  /* Clip bounds to pixmap.  */

  if x0 < 0
    { x0 = 0; }

  /* If x1 is greater than the raster width, make sure the last pixel
     is filled and no more after that.  */

  if x1 > (*raster).width as c_int * 65536
    { x1 = (*raster).width as c_int * 65536; }

  /* Check for empty spans.  */
  if x1 <= x0
    { return; }

  /* Figure out coverage based on Y axis fractional.  */
  coverage = sfnt_poly_coverage[((y >> (16 - SFNT_POLY_SHIFT))
				 & SFNT_POLY_MASK) as usize].as_ptr ();
  row = y >> 16;

  /* Don't fill out of bounds rows.  */
  if row < 0 || row >= (*raster).height as c_int
    { return; }

  /* Set start, then start filling according to coverage.  left and
     right are now .3.  */
  left = x0 >> (16 - SFNT_POLY_SHIFT);
  right = x1 >> (16 - SFNT_POLY_SHIFT);
  start = (*raster).cells.offset ((row * (*raster).stride as c_int) as isize);
  #[cfg (feature = "no_ndebug")]
  { row_end = start.offset ((*raster).width as isize); }
  start = start.offset ((left >> SFNT_POLY_SHIFT) as isize);

  /* If left and right actually lie in the same pixel, just fill with
     the coverage of both and return.  */

  if (left & !SFNT_POLY_MASK) == (right & !SFNT_POLY_MASK)
    {
      #[cfg (feature = "no_ndebug")]
      /* Assert that start does not exceed the end of the row.  */
      { eassert! (start <= row_end); }

      w = *(coverage.offset ((right - left) as isize)) as c_ushort;
      a = (*start as c_int + w as c_int) as c_ushort;

      *start = sfnt_saturate_short (a);
      return;
    }

  /* Compute coverage for first pixel, then poly.  The code from here
     onwards assumes that left and right are on two different
     pixels.  */

  if (left & SFNT_POLY_MASK) != 0
    {
      #[cfg (feature = "no_ndebug")]
      /* Assert that start does not exceed the end of the row.  */
      { eassert! (start <= row_end); }

      /* Compute the coverage for the first pixel, and move left past
	 it.  The coverage is a number from 1 to 7 describing how
	 ``partially'' covered this pixel is.  */

      end = (left + SFNT_POLY_SAMPLE - 1) & !SFNT_POLY_MASK;
      end = MIN! (right, end);

      w = *(coverage.offset ((end - left) as isize)) as c_ushort;
      a = (*start as c_int + w as c_int) as c_ushort;

      /* Now move left past.  */
      left = end;
      *start = sfnt_saturate_short (a);
      start = start.offset (1);
    }

  /* Clear coverage info for first pixel.  Compute coverage for center
     pixels.  Note that SFNT_POLY_SAMPLE is used and not
     SFNT_POLY_MASK, because coverage has a blank column at the
     start.  */
  w = *(coverage.offset (SFNT_POLY_SAMPLE as isize)) as c_ushort;

  /* Fill pixels between left and right.  */
  while left + SFNT_POLY_MASK < right
    {
      #[cfg (feature = "no_ndebug")]
      /* Assert that start does not exceed the end of the row.  */
      { eassert! (start <= row_end); }

      a = (*start as c_int + w as c_int) as c_ushort;
      *start = sfnt_saturate_short (a);
      start = start.offset (1);
      left += SFNT_POLY_SAMPLE;
    }

  /* Fill rightmost pixel with any partial coverage.  */

  if (right & SFNT_POLY_MASK) != 0
    {
      #[cfg (feature = "no_ndebug")]
      /* Assert that start does not exceed the end of the row.  */
      { eassert! (start <= row_end); }

      w = *(coverage.offset ((right - left) as isize)) as c_ushort;
      a = (*start as c_int + w as c_int) as c_ushort;
      *start = sfnt_saturate_short (a);
    }
}

/* Poly each span starting from START onto RASTER, at position Y.  Y
   here is still a cartesian coordinate, where the bottom of the
   raster is 0.  But that is no longer true by the time sfnt_span_fill
   is called.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_poly_span (start: *mut sfnt_edge, y: sfnt_fixed,
			  raster: *mut sfnt_raster)
{                                                          // C 4442-4489
  let mut edge: *mut sfnt_edge;
  let mut winding: c_int;
  let mut x0: sfnt_fixed;
  let mut x1: sfnt_fixed;

  /* Pacify -Wmaybe-uninitialized; x1 and x0 are only used when edge
     != start, at which point x0 has already been set.  */
  x1 = 0;
  x0 = 0;

  /* Generate the X axis coverage map.  Then poly it onto RASTER.
     winding on each edge determines the winding direction: when it is
     positive, winding is 1.  When it is negative, winding is -1.

     Fill each consecutive stretch of spans that are inside the glyph;
     otherwise, coverage will overlap for some spans, but not
     others.

     The spans must be terminated with an edge that causes an
     off-transition, or some spans will not be filled.  */

  winding = 0;

  edge = start;
  while !edge.is_null ()
    {
      if winding == 0
	{
	  if edge != start && x0 != x1
	    /* Draw this section of spans that are on.  */
	    { sfnt_fill_span (raster, (((*raster).height as c_int) << 16) - y,
			      x0, x1); }

	  x1 = (*edge).x;
	  x0 = (*edge).x;
	}
      else
	{ x1 = (*edge).x; }

      winding += (*edge).winding;
      edge = (*edge).next;
    }

  /* Draw the last span following the last off-transition.  */

  if winding == 0 && edge != start && x0 != x1
    { sfnt_fill_span (raster, (((*raster).height as c_int) << 16) - y,
		      x0, x1); }
}


/* Main entry point for outline rasterization.  */

/* Raster the spans between START and its end to the raster specified
   as DCONTEXT.  The span's position is Y.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_raster_span (start: *mut sfnt_edge, y: sfnt_fixed,
				       dcontext: *mut c_void)
{                                                          // C 4498-4503
  sfnt_poly_span (start, y, dcontext as *mut sfnt_raster);
}

/* Generate and poly each span in EDGES onto the raster specified as
   DCONTEXT.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_raster_edge (edges: *mut sfnt_edge, num_edges: size_t,
				       dcontext: *mut c_void)
{                                                          // C 4508-4514
  sfnt_poly_edges (edges, num_edges, Some (sfnt_raster_span),
		   dcontext);
}

/* Generate an alpha mask for the glyph outline OUTLINE.  Value is the
   alpha mask upon success, NULL upon failure.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_raster_glyph_outline (outline: *mut sfnt_glyph_outline)
  -> *mut sfnt_raster                                     // C 4519-4543
{
  let mut raster: sfnt_raster = std::mem::zeroed ();
  let mut data: *mut sfnt_raster;

  /* Get the raster parameters.  */
  sfnt_prepare_raster (&raw mut raster, outline);

  /* Allocate the raster data.  Clear the raster; it is easier to use
     xzalloc and clear everything.  */
  let mut size: size_t;
  let (ckd_size, mul_overflow) = (raster.stride as size_t)
    .overflowing_mul (raster.height as size_t);
  size = ckd_size;
  if mul_overflow
    { memory_full_up (); }
  let (ckd_size, add_overflow) = size
    .overflowing_add (std::mem::size_of::<sfnt_raster> ());
  size = ckd_size;
  if add_overflow
    { memory_full_up (); }
  data = xzalloc (size) as *mut sfnt_raster;
  *data = raster;
  (*data).cells = data.offset (1) as *mut u8;

  /* Generate edges for the outline, polying each array of edges to
     the raster.  */
  sfnt_build_outline_edges (outline, Some (sfnt_raster_edge),
			    data as *mut c_void);

  /* All done.  */
  return data;
}

/* C 4545-4554: the three wrapping-arithmetic macros the interpreter section uses.  C's casts are
   the port's `as`, and the `unsigned int` arithmetic C relies on is `wrapping_*` (PORTING.md
   1.1); they stand here in C's order and stay unused until the interpreter lands (the crate
   allows `unused_macros`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! sfnt_add {
  ($a:expr, $b:expr) => { ((($a) as c_uint).wrapping_add (($b) as c_uint) as c_int) };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! sfnt_sub {
  ($a:expr, $b:expr) => { ((($a) as c_uint).wrapping_sub (($b) as c_uint) as c_int) };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! sfnt_mul {
  ($a:expr, $b:expr) => { ((($a) as c_uint).wrapping_mul (($b) as c_uint) as c_int) };
}

/* Exact coverage scaler.

   The foregoing routines calculate partial coverage for each pixel by
   increasing each span in increments finer than a single pixel, then
   merging active spans into the raster.

   Experience has proven this yields imperfect display results,
   particularly when combined with glyph instruction code which aligns
   points in a certain and as yet undetermined manner.

   The scaler implemented in this page attains greater precision,
   generating at length an array of scanlines, in which each is
   represented by a list of steps.  Each step holds an X coordinate
   and a coverage value, which contributes to the coverage of each
   pixel within the scanline rightwards or equal to the pixel with its
   X coordinate.

   Such a coverage value can be positive or negative; when the winding
   direction of the span it derives from is positive, so is the
   coverage value, that the pixels to its right (thus further into the
   polygon it demarcates) might be painted in.  In the other case, the
   value is negative, thus negating the effect of preceding steps and
   marking the outer boundary of the section of the polygon's
   intersection with the scanline.

   The procedure for producing this array of scanlines is largely an
   adaptation of that which sfnt_poly_edges implements; in particular
   the process of sorting and filtering edges remains untouched.

   Rather than advancing through the edges SFNT_POLY_STEP at a time,
   the edges are iterated over scanline-by-scanline.  Every edge
   overlapping with a particular scanline is considered piecemeal to
   generate its array of steps.

   An edge might overlap pixels within the scanline in one of four
   fashions; each is illustrated with a graphic below:

   +--------ee-----+------------------------------------------------+ (I)
   |      ee.......|................................................|
   |    ee.........|................................................|
   |  ee...........|................................................|
   |ee.............|................................................|
  ee---------------+------------------------------------------------+

   In this instance, the edge partially overlaps its first pixel, but
   the remainder all receive complete coverage.

   +---------------+---------eeeeee+--------------------------------+ (II)
   |               |   eeeeee......|................................|
   |   	          eeeee............|................................|
   |        eeeeee.|...............|................................|
   |  eeeeee.......|...............|................................|
   eee-------------+---------------+--------------------------------+

   In this instance, the edge partially overlaps two or more pixels on
   this scanline.  These pixels are referred to as a run.

   +---------------+---------------+----------------+---------------+ (III)
   |       eeeeeee.|...............|................|...............|
   |   	 eeeeeeee..|...............|................|...............|
   |   eeeeeeee....|...............|................|...............|
   |  eeeeeee......|...............|................|...............|
   +---------------+---------------+----------------+---------------+

   This instance is much like the first instance, save that the
   covered vertical area does not span the entire scanline.

   +---------------+---------------+----------------+---------------+ (IV)
   |               |               |                | eeeeeeeeeee...|
   |   	           |              eeeeeeeeeeeeeeeeeeeeeeeeeeeeeee...|
   |          eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee|...............|
   |eeeeeeeeeeeeeeeeeeeeeee........|................|...............|
   +---------------+---------------+----------------+---------------+

   And this the second, again with the same distinction therefrom.

   In each of these instances, a trapezoid is formed within every
   pixel of the scanline, between:

     - The point of the span's entry into the first pixel, either that
       point itself or, for subsequent pixels, its projection onto
       those pixels.

     - The point of the span's exit or its termination.

     - Both those points projected into the outer boundary of the
       pertinent pixel.

   The proportion formed by the area of this trapezoid and that of the
   pixel then constitutes the coverage value to be recorded.  */

/* Structure representing a step, as above.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct sfnt_step
{
  /* The next step in this list.  */
  pub next: *mut sfnt_step,

  /* X coordinate of the step.  This value affects all pixels at and
     beyond this X coordinate.  */
  pub x: c_int,

  /* Coverage value between -1 and 1.  */
  pub coverage: f32,
}

/* Structure representing an array of steps, one for each
   scanline.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct sfnt_step_raster
{
  /* Number of scanlines within this raster.  */
  pub scanlines: size_t,

  /* Array of steps with one element for each scanline.  */
  pub steps: *mut *mut sfnt_step,

  /* Linked list of chunks of steps allocated for this raster.  */
  pub chunks: *mut sfnt_step_chunk,
}

/* C 4679-4682: C's anonymous enum, so the port declares the same one constant (`c_int`); it is
   the last of the four constants the checker listed as missing after slice 4.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SFNT_BLOCK_STEPS: c_int = 128;

/* Structure representing a block of steps, which are allocated
   SFNT_BLOCK_STEPS at a time.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct sfnt_step_chunk
{
  /* The next chunk in this list, or NULL.  */
  pub next: *mut sfnt_step_chunk,

  /* Number of steps used within this chunk thus far.  */
  pub nused: size_t,

  /* The steps themselves.  */
  pub steps: [sfnt_step; SFNT_BLOCK_STEPS as usize],
}

/* Structure representing an edge as consumed by the exact coverage
   scaler.  This structure is much like struct sfnt_edge, albeit with
   all fractionals replaced by floating point numbers and an extra
   field holding a Y delta.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct sfnt_fedge
{
  /* Next edge in this chain.  */
  pub next: *mut sfnt_fedge,

  /* Winding direction.  1 if clockwise, -1 if counterclockwise.  */
  pub winding: c_int,

  /* X position, top and bottom of edges.  */
  /* C's `float x, top, bottom;` and `float step_x, step_y;` are one declaration per line here:
     `extract-abi.py`'s field check reads `pub name: type,` lines, so a shared line is three
     "missing field" rows (the layout is the same either way).  */
  pub x: f32,
  pub top: f32,
  pub bottom: f32,

  /* Amount to move X by upon each change of Y, and vice versa.  */
  pub step_x: f32,
  pub step_y: f32,
}

/* C 4719-4720: C's function-pointer typedef for the edge consumer (P-27's `Option` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type sfnt_fedge_proc =
  Option<unsafe extern "C" fn (*mut sfnt_fedge, size_t, *mut c_void)>;

/* Build a list of edges for each contour in OUTLINE, displacing each
   edge by xmin and ymin.  Call EDGE_PROC with DCONTEXT and the edges
   produced as arguments.  */

/* C 4735-4739's `edges_alloc`/`alloca` pair is slice 4's C 4068-4072 in the `fedge` flavour:
   the port has no `alloca`, so the small-outline arm goes to the stack pool (PORTING.md P-05,
   Deviation 23), C's `edges_alloc` stays `NULL` there and C 4825's `xfree` is the same no-op.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_build_outline_fedges (outline: *mut sfnt_glyph_outline,
				     edge_proc: sfnt_fedge_proc, dcontext: *mut c_void)
{                                                          // C 4726-4826
  let mut edges: *mut sfnt_fedge;
  let mut i: size_t;
  let mut edge: size_t;
  let mut next_vertex: size_t;
  let mut dx: sfnt_fixed;
  let mut dy: sfnt_fixed;
  let mut step_x: sfnt_fixed;
  let mut step_y: sfnt_fixed;
  let mut ymin: sfnt_fixed;
  let mut xmin: sfnt_fixed;
  let mut top: size_t;
  let mut bottom: size_t;

  let mut edges_alloc: *mut c_void = std::ptr::null_mut ();
  if (*outline).outline_used < 1024 * 16 / std::mem::size_of::<sfnt_fedge> ()
    { edges = sa_stack_alloc ((*outline).outline_used
			      * std::mem::size_of::<sfnt_fedge> ()) as *mut sfnt_fedge; }
  else
    {
      edges_alloc = xnmalloc ((*outline).outline_used as isize,
			      std::mem::size_of::<sfnt_fedge> () as isize);
      edges = edges_alloc as *mut sfnt_fedge;
    }

  edge = 0;

  /* ymin and xmin must be the same as the offset used to set offy and
     offx in rasters.  */
  ymin = sfnt_floor_fixed ((*outline).ymin);
  xmin = sfnt_floor_fixed ((*outline).xmin);

  i = 0;
  while i < (*outline).outline_used
    {
      /* Set NEXT_VERTEX to the next point (vertex) in this contour.

	 If i is past the end of the contour, then don't build edges
	 for this point.  */
      next_vertex = i + 1;

      if next_vertex == (*outline).outline_used
	  || ((*(*outline).outline.offset (next_vertex as isize)).flags
	      & SFNT_GLYPH_OUTLINE_LINETO) == 0
	{ i += 1; continue; }

      /* Skip past horizontal vertices.  */
      if (*(*outline).outline.offset (next_vertex as isize)).y
	== (*(*outline).outline.offset (i as isize)).y
	{ i += 1; continue; }

      /* Figure out the winding direction.  */
      if (*(*outline).outline.offset (next_vertex as isize)).y
	< (*(*outline).outline.offset (i as isize)).y
	/* Vector will cross imaginary ray from its bottom from the
	   left of the ray.  Winding is thus 1.  */
	{ (*(edges.offset (edge as isize))).winding = 1; }
      else
	/* Moving clockwise.  Winding is thus -1.  */
	{ (*(edges.offset (edge as isize))).winding = -1; }

      /* Figure out the top and bottom values of this edge.  If the
	 next edge is below, top is here and bot is the edge below.
	 If the next edge is above, then top is there and this is the
	 bottom.  */

      if (*(*outline).outline.offset (next_vertex as isize)).y
	< (*(*outline).outline.offset (i as isize)).y
	{
	  /* End of edge is below this one (keep in mind this is a
	     cartesian coordinate system, so smaller values are below
	     larger ones.) */
	  top = i;
	  bottom = next_vertex;
	}
      else
	{
	  /* End of edge is above this one.  */
	  bottom = i;
	  top = next_vertex;
	}

      /* Record the edge.  Rasterization happens from bottom to
	 up, so record the X at the bottom.  */
      dx = ((*(*outline).outline.offset (top as isize)).x
	    - (*(*outline).outline.offset (bottom as isize)).x);
      dy = abs ((*(*outline).outline.offset (top as isize)).y
		- (*(*outline).outline.offset (bottom as isize)).y);

      /* Compute the step X.  This is how much X changes for each
	 increase in Y.  */
      step_x = sfnt_div_fixed (dx, dy);

      /* And the step Y, which is the amount of movement to Y an
	 increase in X will incur.  */
      step_y = if dx != 0 { sfnt_div_fixed (dy, dx) } else { 0 };

      /* Save information computed above into the edge.  */
      (*(edges.offset (edge as isize))).top
	= sfnt_fixed_float ((*(*outline).outline.offset (top as isize)).y - ymin);
      (*(edges.offset (edge as isize))).bottom
	= sfnt_fixed_float ((*(*outline).outline.offset (bottom as isize)).y - ymin);
      (*(edges.offset (edge as isize))).x
	= sfnt_fixed_float ((*(*outline).outline.offset (bottom as isize)).x - xmin);
      (*(edges.offset (edge as isize))).step_x = sfnt_fixed_float (step_x);
      (*(edges.offset (edge as isize))).step_y = sfnt_fixed_float (step_y);
      (*(edges.offset (edge as isize))).next = std::ptr::null_mut ();

      /* Increment the edge index.  */
      edge += 1;
      i += 1;
    }

  if edge != 0
    { edge_proc.unwrap () (edges, edge, dcontext); }
  xfree (edges_alloc);
}

/* C 4828: C's function-pointer typedef for the step consumer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type sfnt_step_raster_proc =
  Option<unsafe extern "C" fn (*mut sfnt_step_raster, *mut c_void)>;

/* Append a step with the supplied COVERAGE at X to the sorted list of
   scanline steps within the container RASTER.  Y is the scanline to
   append to.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_insert_raster_step (raster: *mut sfnt_step_raster,
				   mut x: c_int, coverage: f32, scanline: size_t)
{                                                          // C 4834-4883
  let mut chunk: *mut sfnt_step_chunk;
  let mut step: *mut sfnt_step;
  let mut p_next: *mut *mut sfnt_step;

  if scanline >= (*raster).scanlines
    { return; }

  if x < 0
    { x = 0; }

  /* Search within RASTER->steps[scanline] for a step at X.  */

  'found: {
    p_next = (*raster).steps.offset (scanline as isize);

    while { step = *p_next; !step.is_null () }
      {
        if (*step).x > x
	  { break; }

        if (*step).x == x
	  { break 'found; }

        p_next = &raw mut (*step).next;
      }

    if (*raster).chunks.is_null ()
        || (*(*raster).chunks).nused == SFNT_BLOCK_STEPS as size_t
      {
        /* All chunks have been consumed, and consequently a new chunk
	   must be allocated.  */
        chunk = xmalloc (std::mem::size_of::<sfnt_step_chunk> ()) as *mut sfnt_step_chunk;
        (*chunk).next = (*raster).chunks;
        (*chunk).nused = 0;
        (*raster).chunks = chunk;
      }
    else
      { chunk = (*raster).chunks; }

    step		 = &raw mut (*chunk).steps[(*chunk).nused];
    (*chunk).nused	+= 1;
    (*step).next	 = *p_next;
    *p_next		 = step;
    (*step).x	 = x;
    (*step).coverage = 0.0;
  } /* 'found (C 4881) */

  /* found: */
  (*step).coverage += coverage;
}

/* Draw EDGES, an unsorted array of polygon edges of size NEDGES.

   Transform EDGES into an array of steps representing a raster with
   HEIGHT scanlines, then call POLY_FUNC with DCONTEXT and the
   resulting struct sfnt_step_raster to transfer it onto an actual
   raster.

   WIDTH must be the width of the raster.  Although there is no
   guarantee that no steps generated extend past WIDTH, steps starting
   after width might be omitted, and as such it must be accurate.  */

/* C 4903's `sentinel` is an uninitialised local used only through its address (the active list's
   terminator), so the port zeroes it (PORTING.md 1.5, the Deviation 30 class).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_poly_edges_exact (edges: *mut sfnt_fedge, nedges: size_t,
				 height: size_t, width: size_t,
				 proc: sfnt_step_raster_proc, dcontext: *mut c_void)
{                                                          // C 4896-5303
  let mut y: c_int;
  let mut e: size_t;
  let mut edges_processed: size_t;
  let mut active: *mut sfnt_fedge;
  let mut prev: *mut *mut sfnt_fedge;
  let mut a: *mut sfnt_fedge;
  let mut sentinel: sfnt_fedge = std::mem::zeroed ();
  let mut raster: sfnt_step_raster = std::mem::zeroed ();
  let mut next: *mut sfnt_step_chunk;
  let mut last: *mut sfnt_step_chunk;

  if height == 0
    { return; }

  /* Step down line by line.  Find active edges.  */

  y = sfnt_floor_fixed ((MAX! (0.0f32, (*(edges.offset (0))).bottom)) as sfnt_fixed);
  edges_processed = 0;
  e = 0;
  active = &raw mut sentinel;

  /* Allocate the array of edges.  */

  raster.scanlines = height;
  raster.chunks    = std::ptr::null_mut ();
  raster.steps = xcalloc (height,
			  std::mem::size_of::<*mut sfnt_step> ()) as *mut *mut sfnt_step;

  while (y as size_t) != height
    {
      /* Run over the whole array on each iteration of this loop;
	 experiments demonstrate this is faster for the majority of
	 glyphs.  */
      e = 0;
      while e < nedges
	{
	  /* Although edges is unsorted, edges which have already been
	     processed will see their next fields set, and can thus be
	     disregarded.  */
	  if (*(edges.offset (e as isize))).next.is_null ()
	      && ((*(edges.offset (e as isize))).bottom < (y + 1) as f32)
	      && ((*(edges.offset (e as isize))).top > y as f32)
	    {
	      /* As steps generated from each edge are sorted at the
		 time of their insertion, sorting the list of active
		 edges itself is redundant.  */
	      (*(edges.offset (e as isize))).next = active;
	      active = edges.offset (e as isize);

	      /* Increment the counter recording the number of edges
		 processed, which is used to terminate this loop early
		 once all have been processed.  */
	      edges_processed += 1;
	    }
	  e += 1;
	}

      /* Iterate through each active edge, appending steps for it, and
	 removing it if it does not overlap with the next
	 scanline.  */

      prev = &raw mut active;
      while { a = *prev; a != &raw mut sentinel }
	{
	  'next: {
	    let mut x_top: f32;
	    let mut x_bot: f32;
	    let mut x_min: f32;
	    let mut x_max: f32;
	    let mut y_top: f32;
	    let mut y_bot: f32;
	    let mut x_pixel_min: c_int;
	    let mut x_pixel_max: c_int;

	    macro_rules! APPEND_STEP {
	      ($x:expr, $coverage:expr) => {
	        sfnt_insert_raster_step (&raw mut raster, $x as c_int, $coverage, y as size_t);
	      };
	    }

	    /* Calculate several values to establish which overlap
	       category this edge falls into.  */

	    y_top = (y + 1) as f32; /* Topmost coordinate covered by this
					edge in this scanline.  */
	    y_bot = y as f32;     /* Bottom-most coordinate covered by this
				   edge in this scanline.  */

	    /* III or IV?  If the edge terminates before the next
	       scanline, make its terminus y_top.  */

	    if y_top > (*a).top
	      { y_top = (*a).top; }

	    /* Same goes for y_bottom.  */

	    if (*a).bottom > y_bot
	      { y_bot = (*a).bottom; }

	    /* y_top should never equal y_bottom, but check to be on the
	       safe side.  */
	    if y_top == y_bot
	      { break 'next; }
	    /* x_top and x_bot are the X positions where the edge enters
	       and exits this scanline.  */

	    /*
	      (x_top)
     +--------ee-----+------------------------------------------------+ (y_top)
     |      ee.......|................................................|
     |    ee.........|................................................|
     |  ee...........|................................................|
     |ee.............|................................................|
    ee---------------+------------------------------------------------+ (y_bot)
  (x_bot)
              (y_bot might be further below.)
	    */
	    x_top = (y_top - (*a).bottom) * (*a).step_x + (*a).x;
	    x_bot = (y_bot - (*a).bottom) * (*a).step_x + (*a).x;

	    x_min = MIN! (x_top, x_bot);
	    x_max = MAX! (x_top, x_bot);

	    /* Pixels containing x_bot and x_top respectively.  */
	    x_pixel_min = x_min as c_int;
	    x_pixel_max = x_max as c_int;

	    macro_rules! TRAPEZOID_AREA {
	      ($height:expr, $top_start:expr, $top_end:expr, $bot_start:expr,
	       $bot_end:expr) => {
	      (((($top_end) as f32 - ($top_start) as f32)
		+ (($bot_end) as f32 - ($bot_start) as f32))
	       / 2.0f32 * ($height) as f32)
	      };
	    }

	    /* I, III?  These two instances' criteria are that the edge
	       enters and exits within one pixel.  */

	    if x_pixel_min == x_pixel_max
	      {
	        let mut xmin: f32;
	        let mut xmax: f32;
	        let mut ytop: f32;
	        let mut ybot: f32;
	        let mut height: f32;
	        let mut coverage: f32;
	        let mut delta: f32;

	        /* Partial coverage for the first pixel.  */

	        xmin = x_min;
	        xmax = x_max;
	        ytop = y_top;
	        ybot = y_bot;
	        height = ytop - ybot;
	        /* The trapezoid here is one of the following two:

    ytop+------xmax--+-----------+---------------------------------------------+
        |     /................................................................|
        |    /.......|...........|.............................................|
        |   /........|...........|.............................................|
        |  /.........|...........|.............................................|
        | / ...................................................................|
    xmin+------------+-----------+---------------------------------------------+
        ybot
    ytop+------------+-----------+---------------------------------------------+
        |\ xmin................................................................|
        | \..........|...........|.............................................|
        |  \.........|...........|.............................................|
        |   \........|...........|.............................................|
        |    \.......|...........|.............................................|
        |     \................................................................|
        +------xmax--+-----------+---------------------------------------------+

                   In either situation, the first pixel's coverage is
                   the space occupied by a trapezoid whose corners are
                   xmin and x_pixel_min + 1 and xmax and x_pixel_min +
                   1, and whose height is ytop - ybot.  The coverage for
                   the remainder is the height alone.  */
	        coverage = (TRAPEZOID_AREA! (height,
					    xmin, xmin as c_int + 1,
					    xmax, xmax as c_int + 1)
			    * (*a).winding as f32);
	        APPEND_STEP! (x_pixel_min, coverage);

	        /* Then if the next pixel isn't beyond the raster,
		   append complete coverage for it.  */

	        if ((x_pixel_min + 1) as size_t) < width
		  {
		    delta = (y_top - y_bot) * (*a).winding as f32;
		    APPEND_STEP! (x_pixel_max + 1, delta - coverage);
		  }
	      }
	    else
	      {
	        let mut dy: f32;
	        let mut y_crossing: f32;
	        let mut coverage: f32;
	        let mut ytop: f32;
	        let mut ybot: f32;
	        let mut xtop: f32;
	        let mut xbot: f32;
	        let mut increment: f32;
	        let mut x: f32;
	        let mut last: f32;
	        let mut here: f32;

	        ytop = y_top;
	        ybot = y_bot;
	        xtop = x_top;
	        xbot = x_bot;

	        macro_rules! TRIANGLE_AREA {
		  ($width:expr, $height:expr) => { (($width) * ($height) / 2.0f32) };
	        }

	        /* II, IV.  Coverage must be computed for each pixel
		   from x_pixel_min to x_pixel_max, with the latter
		   treated much as in I or III.  */

	        if x_bot < x_top
		  {
		    /*


    y_top                                                         x_top
         +-----------+-----------+-----------+------------+-------/------+-------------------------------+
         |           |           |           |            |    /--.......|...............................|
         |x_pixel_min|           |           |            | /--..........|...............................|
         |           |           |           |           /+-y_crossing...|...............................|
         |           |           |           |        /--.|..............|...............................|
         |           |           |           |      /-....|..............|...............................|
         |           |           |           |   /--......|..x_pixel_max.|...............................|
         |           |           |           |/--.........|..............|...............................|
         |           |           |         /-+............|..............|...............................|
         |           |           |      /--..|............|..............|...............................|
         |           |           |   /--.....|............|..............|...............................|
         |           |           |/--........|............|..............|...............................|
         |           |         /-+...........|............|..............|...............................|
         |           |      /--..|...........|............|..............|...............................|
         |           |   /--.....|...........|............|..............|...............................|
         |           | /-........|...........|............|..............|...............................|
         |          /+-y_crossing|...........|............|..............|...............................|
         |       /--.|...........|...........|............|..............|...............................|
         |    /--....|...........|...........|............|..............|...............................|
         | /--.......|...........|...........|............|..............|...............................|
         +-----------+-----------+-----------+------------+--------------+-------------------------------+
    y_bot x_bot


  The purpose of this code is to calculate the area occupied by dots of
  each pixel in between x_pixel_min and x_pixel_max + 1.

  The area occupied in the first pixel is a triangle comprising [x_bot,
  y_bot], [x_bot + 1, y_bot], and [x_bot + 1, y_crossing].

  The area occupied in the second pixel through x_pixel_max - 1 is that
  of a rectangle comprising [y_bot, pixel], [the previous rectangle's
  y_crossing, pixel], [the previous rectangle's y_crossing, pixel + 1],
  and [pixel + 1, y_bot] summed with the area the remaining triangle.

  The area occupied in the last pixel is a trapezoid proper.

  Thus the procedure is roughly as follows: dy is computed, which is the
  increase to the Y of the edge for each increase in scanline X.  */
		    dy = (*a).step_y;

		    /* As is y_crossing for the first pixel.  */
		    y_crossing = ybot + dy * (xbot as c_int + 1) as f32 - xbot;

		    /* And the area of the first triangle.

		       The width is (int) xbot + 1 - xbot, and the
		       height is y_crossing - ybot.  */
		    last = (TRIANGLE_AREA! (y_crossing - ybot,
					    (xbot as c_int + 1) as f32 - xbot)
			    * (*a).winding as f32);
		    APPEND_STEP! (x_pixel_min, last);

		    /* Coverage value for subsequent rectangles.  The
		       value set here is for the next pixel, which is
		       filled from ybot to y_crossing.  */

		    coverage = (y_crossing - ybot) * (*a).winding as f32;
		    increment = dy * (*a).winding as f32;

		    x = (x_pixel_min + 1) as f32;
		    while x < x_pixel_max as f32
		      {
		        here = coverage + increment / 2.0f32;
		        APPEND_STEP! (x, here - last);
		        last = here;
		        coverage += increment;
		        x += 1.0;
		      }

		    /* The y_crossing for the last pixel.  */
		    y_crossing = ybot + dy * (xtop as c_int) as f32 - xbot;

		    /* And calculate the area of the trapezoid in the
		       last pixel.  */

		    coverage += (*a).winding as f32
		      * TRAPEZOID_AREA! (ytop - y_crossing,
				         xtop,
				         xtop as c_int + 1,
				         xtop as c_int,
				         xtop as c_int + 1);
		    here = coverage;
		    APPEND_STEP! (x_pixel_max, here - last);
		    last = here;

		    /* Fill the remainder of the scanline with
		       height-derived coverage.  */

		    if ((x_pixel_max + 1) as size_t) < width
		      { APPEND_STEP! (x_pixel_max + 1, ((y_top - y_bot) * (*a).winding as f32 - last)); }
		  }
	        else /* if (x_bot > x_top) */
		  {
		    /*

    y_top   x_top
      +----------------+----------------+-----------------+-----------------+-----------------------------+
      |     \--........|................|.................|.................|.............................|
      |        \--.....|................|.................|.................|.............................|
      |           \--..|................|.................|.................|.............................|
      |              \-+.y_crossing.....|.................|.................|.............................|
      |                |\--.............|.................|.................|.............................|
      |                |   \--..........|.................|.................|.............................|
      |  x_pixel_min   |      \---......|.................|.................|.............................|
      |                |          \--...|.................|.................|.............................|
      |                |             \--|y_crossing.......|.................|.............................|
      |                |                \--...............|.................|.............................|
      |                |                |  \--............|.................|.............................|
      |                |                |     \--.........|.................|.............................|
      |                |                |        \--......|.................|.............................|
      +----------------+----------------+-----------\-----+-----------------+-----------------------------+
    y_bot                                        x_bot

  Whereas in this situation the trapezoid is inverted, and the code must
  be as well.  */
		    /* The edge's Y decreases as the edge's X increases,
		       yielding a negative a->step_x.  */
		    dy = (*a).step_y;

		    /* Calculate y_crossing for the first pixel.  */
		    y_crossing = ytop + dy * (xtop as c_int + 1) as f32 - xtop;

		    /* And the area of the first triangle.  */
		    last = (TRIANGLE_AREA! ((xtop as c_int + 1) as f32 - xtop,
					    ytop - y_crossing)
			    * (*a).winding as f32);
		    APPEND_STEP! (x_pixel_min, last);

		    /* Coverage value for subsequent rectangles.  The
		       value set here is for the next pixel, which is
		       filled from ytop to y_crossing.  */
		    coverage = (ytop - y_crossing) * (*a).winding as f32;
		    increment = -dy * (*a).winding as f32;

		    x = (x_pixel_min + 1) as f32;
		    while x < x_pixel_max as f32
		      {
		        here = coverage + increment / 2.0f32;
		        APPEND_STEP! (x, here - last);
		        last = here;
		        coverage += increment;
		        x += 1.0;
		      }

		    /* The y_crossing for the last pixel.  */
		    y_crossing = ytop + dy * (xbot as c_int) as f32 - xtop;

		    /* And calculate the area of the trapezoid in the
		       last pixel.  */

		    coverage += (*a).winding as f32
		      * TRAPEZOID_AREA! (y_crossing - ybot,
				         xbot as c_int,
				         xbot as c_int + 1,
				         xbot,
				         xbot as c_int + 1);
		    here = coverage;
		    APPEND_STEP! (x_pixel_max, here - last);
		    last = here;

		    /* Fill the remainder of the scanline with
		       height-derived coverage.  */

		    if ((x_pixel_max + 1) as size_t) < width
		      { APPEND_STEP! (x_pixel_max + 1, ((y_top - y_bot) * (*a).winding as f32 - last)); }
		  }
	      } /* C's `#undef TRIANGLE_AREA` (C 5262) has no Rust text: `TRIANGLE_AREA` is a
		   `macro_rules!` scoped to this block, so it dies at the block's end.  */
	  /* C's `#undef APPEND_STEP`/`#undef TRAPEZOID_AREA` (C 5265-5266) are the same
	     block-scope ending.  */

	    /* When an edge is created, its a->bottom (and by extension
	       a->y) is not aligned to a->x.  Since this iteration can
	       only affect the scan line Y, align a to the next
	       scanline, that the next iteration of this loop to
	       consider it might consider its entire intersection.  */
	    (*a).x += (*a).step_x * ((y + 1) as f32 - (*a).bottom);
	  (*a).bottom = (y + 1) as f32;
	  } /* 'next (C 5275) */

	/* next: */
	if (*a).top < (y + 1) as f32
	  { *prev = (*a).next; }
	else
	    /* This edge doesn't intersect with the next scanline;
	       remove it from the list.  After the edge at hand is so
	       deleted from the list, its next field remains set,
	       excluding it from future consideration.  */
	  { prev = &raw mut (*a).next; }
	}

      /* Break if all is done.  */
      if active == &raw mut sentinel && edges_processed == nedges
	{ break; }

      y += 1;
    }

  proc.unwrap () (&raw mut raster, dcontext);
  xfree (raster.steps as *mut c_void);

  /* Free each block of steps allocated.  */
  next = raster.chunks;
  while !next.is_null ()
    {
      last = next;
      next = (*next).next;
      xfree (last as *mut c_void);
    }
}

/* Apply winding rule to the coverage value VALUE.  Convert VALUE to a
   number between 0 and 255.  If VALUE is negative, invert it.  If it
   exceeds 255 afterwards, truncate it to 255.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_compute_fill (mut value: f32) -> c_int
{                                                          // C 5309-5316
  if value < 0.0f32
    { value = -value; }

  return (MIN! (value * 255.0f32, 255.0f32)) as c_int;
}

/* Set N pixels at DATA to the value VALUE.  If N is large, call
   memset; otherwise set this by hand.  */

/* C 5327-5354's `switch (n)` is a Duff's-device ladder: every `case` falls through into the next,
   so each `match` arm carries the statements of the arms below it and keeps C's `FALLTHROUGH;`
   token at its site (the `regcomp.rs` 2044/2080 shape - the token survives because the shared
   statements follow it).  `case 0:'s `break` is the arm's end, and `default:` is `_`.  The
   `switch`/`case`/`default`/`break`/`fallthrough` row this owes `docs/ports/macro-body-notes.tsv`
   is the ledger's Owed 10.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_poly_set_steps (data: *mut c_uchar, value: c_int, n: c_int)
{                                                          // C 5321-5355
  let mut p: *mut c_uchar;

  p = data;
  match n
    {
      7 =>
	{
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	}
      6 =>
	{
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	}
      5 =>
	{
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	}
      4 =>
	{
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	}
      3 =>
	{
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	}
      2 =>
	{
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	  *p = value as c_uchar; p = p.offset (1);
	}
      1 =>
	{
	  *p = value as c_uchar; p = p.offset (1);
	  FALLTHROUGH! ();
	}
      0 => {}
      _ =>
	{ memset (data as *mut c_void, value, n as size_t); }
    }
}

/* Transfer steps generated by sfnt_poly_edges_exact from STEPS to the
   provided raster RASTER.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_poly_steps (steps: *mut sfnt_step_raster, raster: *mut sfnt_raster)
{                                                          // C 5360-5396
  let mut y: c_int;
  let mut data: *mut c_uchar;
  let mut x: c_int;
  let mut xend: c_int;
  let mut fill: c_int;
  let mut total: f32;
  let mut step: *mut sfnt_step;

  y = 0; /* This y is an X-style coordinate in RASTER's space.

	    Its counterpart array of steps is STEPS->steps[
	    raster->height - y - 1].  */
  data = (*raster).cells;

  y = 0;
  while y < (*raster).height as c_int
    {
      x = 0;
      total = 0.0;
      fill = 0;

      step = *(*steps).steps.offset (((*raster).height as c_int - y - 1) as isize);
      while !step.is_null () && x < (*raster).width as c_int
	{
	  xend = MIN! ((*step).x, (*raster).width as c_int);

	  if fill != 0
	    { sfnt_poly_set_steps (data.offset (x as isize), fill, xend - x); }

	  total += (*step).coverage;
	  fill = sfnt_compute_fill (total);
	  x = xend;
	  step = (*step).next;
	}

      if x < (*raster).width as c_int
	{ sfnt_poly_set_steps (data.offset (x as isize), fill,
			       (*raster).width as c_int - x); }
      y += 1;
      data = data.offset ((*raster).stride as isize);
    }
}

/* Poly each edge in EDGES onto the raster supplied in DCONTEXT.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_raster_steps (steps: *mut sfnt_step_raster, dcontext: *mut c_void)
{                                                          // C 5400-5404
  sfnt_poly_steps (steps, dcontext as *mut sfnt_raster);
}

/* Call sfnt_poly_edges_exact with suitable arguments for polying
   EDGES onto DCONTEXT, a raster structure.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_raster_edges_exact (edges: *mut sfnt_fedge, size: size_t,
					      dcontext: *mut c_void)
{                                                          // C 5409-5419
  let mut raster: *mut sfnt_raster;

  raster = dcontext as *mut sfnt_raster;
  sfnt_poly_edges_exact (edges, size, (*raster).height as size_t,
			 (*raster).width as size_t, Some (sfnt_raster_steps),
			 dcontext);
}

/* Generate an alpha mask for the glyph outline OUTLINE by means of
   the exact coverage scaler.  Value is the alpha mask upon success,
   NULL upon failure.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_raster_glyph_outline_exact (outline: *mut sfnt_glyph_outline)
  -> *mut sfnt_raster                                     // C 5425-5451
{
  let mut raster: sfnt_raster = std::mem::zeroed ();
  let mut data: *mut sfnt_raster;
  let mut size: size_t;

  /* Get the raster parameters.  */
  sfnt_prepare_raster (&raw mut raster, outline);

  /* Allocate the raster data.  Clear the raster; it is easier to use
     xcalloc and clear everything.  */
  let (ckd_size, mul_overflow) = (raster.stride as size_t)
    .overflowing_mul (raster.height as size_t);
  size = ckd_size;
  if mul_overflow
    { return std::ptr::null_mut (); }
  let (ckd_size, add_overflow) = size
    .overflowing_add (std::mem::size_of::<sfnt_raster> ());
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }
  data = xcalloc (1, size) as *mut sfnt_raster;
  if data.is_null ()
    { return std::ptr::null_mut (); }
  *data = raster;
  (*data).cells = data.offset (1) as *mut u8;

  /* Generate edges for the outline, polying each array of edges to
     the raster.  */
  sfnt_build_outline_fedges (outline, Some (sfnt_raster_edges_exact),
			     data as *mut c_void);

  /* All done.  */
  return data;
}
/* Glyph metrics computation.  */

/* Read an hmtx table from the font FD, using the table directory
   specified as SUBTABLE, the maxp table MAXP, and the hhea table
   HHEA.

   Return NULL upon failure, and the hmtx table otherwise.
   HHEA->num_of_long_hor_metrics determines the number of horizontal
   metrics present, and MAXP->num_glyphs -
   HHEA->num_of_long_hor_metrics determines the number of left-side
   bearings present.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_hmtx_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable,
					       hhea: *mut sfnt_hhea_table,
					       maxp: *mut sfnt_maxp_table)
  -> *mut sfnt_hmtx_table                                 // C 5467-5532
{
  let mut directory: *mut sfnt_table_directory;
  let mut hmtx: *mut sfnt_hmtx_table;
  let mut size: size_t;
  let mut rc: ssize_t;
  let mut i: c_int;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_HMTX);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Figure out how many bytes are required.  */
  size = ((*hhea).num_of_long_hor_metrics as size_t
	  * std::mem::size_of::<sfnt_long_hor_metric> ())
    + (MAX! (0 as c_int, ((*maxp).num_glyphs as c_int
			  - (*hhea).num_of_long_hor_metrics as c_int)) as size_t
       * std::mem::size_of::<int16_t> ());

  /* Check the length matches exactly.  */
  if (*directory).length as size_t != size
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Now allocate enough to hold all of that along with the table
     directory structure.  */

  hmtx = xaddmalloc (std::mem::size_of::<sfnt_hmtx_table> (), size)
    as *mut sfnt_hmtx_table;

  /* Read into hmtx + 1.  */
  rc = read (fd, hmtx.offset (1) as *mut c_void, size);
  if rc == -1 || (rc as size_t) < size
    {
      xfree (hmtx as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Set pointers to data.  */
  (*hmtx).h_metrics = hmtx.offset (1) as *mut sfnt_long_hor_metric;
  (*hmtx).left_side_bearing
    = (*hmtx).h_metrics.offset ((*hhea).num_of_long_hor_metrics as isize) as *mut int16_t;

  /* Swap what was read.  */

  i = 0;
  while i < (*hhea).num_of_long_hor_metrics as c_int
    {
      sfnt_swap16! (&raw mut (*(*hmtx).h_metrics.offset (i as isize)).advance_width);
      sfnt_swap16! (&raw mut (*(*hmtx).h_metrics.offset (i as isize)).left_side_bearing);
      i += 1;
    }

  while i < (*maxp).num_glyphs as c_int
    {
      sfnt_swap16! (&raw mut *(*hmtx).left_side_bearing
		    .offset ((i - (*hhea).num_of_long_hor_metrics as c_int) as isize));
      i += 1;
    }

  /* All done.  */
  return hmtx;
}

/* Obtain unscaled glyph metrics for the glyph indexed by GLYPH.
   Return 0 and the metrics in *METRICS if metrics could be found,
   else 1.

   HMTX, HHEA and MAXP should be the hmtx, hhea, head, and maxp tables
   of the font respectively.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_lookup_glyph_metrics (glyph: sfnt_glyph_index,
						    metrics: *mut sfnt_glyph_metrics,
						    hmtx: *mut sfnt_hmtx_table,
						    hhea: *mut sfnt_hhea_table,
						    maxp: *mut sfnt_maxp_table)
  -> c_int                                                // C 5541-5575
{
  let mut lbearing: c_short;
  let mut advance: c_ushort;

  if glyph < (*hhea).num_of_long_hor_metrics as c_uint
    {
      /* There is a long entry in the hmtx table.  */
      lbearing = (*(*hmtx).h_metrics.offset (glyph as isize)).left_side_bearing;
      advance = (*(*hmtx).h_metrics.offset (glyph as isize)).advance_width;
    }
  else if (*hhea).num_of_long_hor_metrics != 0
	   && glyph < (*maxp).num_glyphs as c_uint
    {
      /* There is a short entry in the hmtx table.  */
      lbearing
	= *(*hmtx).left_side_bearing
	  .offset ((glyph - (*hhea).num_of_long_hor_metrics as c_uint) as isize);
      advance
	= (*(*hmtx).h_metrics
	   .offset (((*hhea).num_of_long_hor_metrics as c_int - 1) as isize)).advance_width;
    }
  else
    /* No entry corresponds to the glyph.  */
    { return 1; }

  /* Return unscaled metrics.  */
  (*metrics).lbearing = lbearing as sfnt_fixed;
  (*metrics).advance = advance as sfnt_fixed;
  return 0;
}

/* Scale the specified glyph metrics by FACTOR.  Set METRICS->lbearing
   and METRICS->advance to their current values times factor; take the
   floor of the left bearing and round the advance width.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_scale_metrics (metrics: *mut sfnt_glyph_metrics,
					     factor: sfnt_fixed)
{                                                          // C 5581-5592
  let mut lbearing: sfnt_fixed;
  let mut advance: sfnt_fixed;

  lbearing = sfnt_mul_fixed ((*metrics).lbearing * 65536, factor);
  advance = sfnt_mul_fixed ((*metrics).advance * 65536, factor);

  (*metrics).lbearing = sfnt_floor_fixed (lbearing);
  (*metrics).advance = sfnt_round_fixed (advance);
}

/* Calculate the factor used to convert em space to device space for a
   font with the specified HEAD table and PPEM value.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_get_scale (head: *mut sfnt_head_table, ppem: c_int)
  -> sfnt_fixed                                           // C 5597-5612
{
  /* Figure out how to convert from font unit-space to pixel space.
     To turn one unit to its corresponding pixel size given a ppem of
     1, the unit must be divided by head->units_per_em.  Then, it must
     be multiplied by the ppem.  So,

       PIXEL = UNIT / UPEM * PPEM

     which means:

       PIXEL = UNIT * PPEM / UPEM  */
  return sfnt_div_fixed (ppem, (*head).units_per_em as sfnt_fixed);
}

/* Font style parsing.  */

/* Read the name table from the given font FD, using the table
   directory specified as SUBTABLE.  Perform validation on the offsets
   in the name records.  Return NULL upon failure, else the name
   table.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_name_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_name_table                                 // C 5623-5768
{
  let mut directory: *mut sfnt_table_directory;
  let mut name: *mut sfnt_name_table;
  let mut required: size_t;
  let mut rc: ssize_t;
  let mut i: c_int;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_NAME);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Figure out the minimum that has to be read.  */
  required = SFNT_ENDOF! (sfnt_name_table, string_offset, uint16_t);

  if ((*directory).length as size_t) < required
    { return std::ptr::null_mut (); }

  /* Allocate enough to hold the name table and variable length
     data.  */
  name = xaddmalloc (std::mem::size_of::<sfnt_name_table> (),
		     (*directory).length as size_t) as *mut sfnt_name_table;

  /* Read the fixed length data.  */
  rc = read (fd, name as *mut c_void, required);
  if rc == -1 || (rc as size_t) < required
    {
      xfree (name as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap what was read.  */
  sfnt_swap16! (&raw mut (*name).format);
  sfnt_swap16! (&raw mut (*name).count);
  sfnt_swap16! (&raw mut (*name).string_offset);

  /* Reject unsupported formats.  */
  if (*name).format != 0
    {
      xfree (name as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Set the pointer to the start of the variable length data.  */
  (*name).name_records = name.offset (1) as *mut sfnt_name_record;

  /* Check there is enough for the name records.  */
  required = (*directory).length as size_t - required;
  if required < (*name).count as size_t * std::mem::size_of::<sfnt_name_record> ()
    {
      xfree (name as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Read the variable length data.  First, read the name records.  */
  rc = read (fd, (*name).name_records as *mut c_void,
	     (*name).count as size_t * std::mem::size_of::<sfnt_name_record> ());
  if rc == -1 || (rc as size_t) < ((*name).count as size_t
				   * std::mem::size_of::<sfnt_name_record> ())
    {
      xfree (name as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap each of the name records.  */
  i = 0;
  while i < (*name).count as c_int
    {
      sfnt_swap16! (&raw mut (*(*name).name_records.offset (i as isize)).platform_id);
      sfnt_swap16! (&raw mut (*(*name).name_records.offset (i as isize)).platform_specific_id);
      sfnt_swap16! (&raw mut (*(*name).name_records.offset (i as isize)).language_id);
      sfnt_swap16! (&raw mut (*(*name).name_records.offset (i as isize)).name_id);
      sfnt_swap16! (&raw mut (*(*name).name_records.offset (i as isize)).length);
      sfnt_swap16! (&raw mut (*(*name).name_records.offset (i as isize)).offset);
      i += 1;
    }

  /* Now, read the name data.  */

  if ((*name).string_offset as size_t) > (*directory).length as size_t
    {
      xfree (name as *mut c_void);
      return std::ptr::null_mut ();
    }

  required = (*directory).length as size_t - (*name).string_offset as size_t;

  /* It can happen that the string offset comes before the name
     records, and as a result exceeds the number of bytes
     previously allocated.  Extend name if that is the case.  */

  if required > ((*directory).length as size_t
		 - ((*name).count as size_t
		    * std::mem::size_of::<sfnt_name_record> ()))
    {
      let mut size: size_t;
      let (ckd_size, mul_overflow) = ((*name).count as size_t)
	.overflowing_mul (std::mem::size_of::<sfnt_name_record> ());
      size = ckd_size;
      if mul_overflow
	{ memory_full_up (); }
      let (ckd_size, add_overflow) = size.overflowing_add (required);
      size = ckd_size;
      if add_overflow
	{ memory_full_up (); }
      name = xaddrealloc (name as *mut c_void,
			  std::mem::size_of::<sfnt_name_table> (),
			  size) as *mut sfnt_name_table;
      (*name).name_records = name.offset (1) as *mut sfnt_name_record;
    }

  /* There is enough space past name->name_records to hold REQUIRED
     bytes.  Seek to the right offset.  */

  if lseek (fd, (*directory).offset as off_t + (*name).string_offset as off_t,
	    SEEK_SET) == (-1 as off_t)
    {
      xfree (name as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Read REQUIRED bytes into the string data.  */
  (*name).data = (*name).name_records.offset ((*name).count as isize) as *mut u8;
  rc = read (fd, (*name).data as *mut c_void, required);
  if rc == -1 || (rc as size_t) < required
    {
      xfree (name as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Now validate each of the name records.  */
  i = 0;
  while i < (*name).count as c_int
    {
      if (((*(*name).name_records.offset (i as isize)).offset as c_int
	   + (*(*name).name_records.offset (i as isize)).length as c_int) as size_t)
	> required
	{
	  /* The name is out of bounds! */
	  xfree (name as *mut c_void);
	  return std::ptr::null_mut ();
	}
      i += 1;
    }

  /* Return the name table.  */
  return name;
}

/* Return a pointer to the name data corresponding with CODE under the
   name table NAME.  Return the start of the data and the name record
   under *RECORD upon success, and NULL otherwise.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_find_name (name: *mut sfnt_name_table,
					 code: sfnt_name_identifier_code,
					 record: *mut sfnt_name_record)
  -> *mut c_uchar                                          // C 5774-5792
{
  let mut i: c_int;

  i = 0;
  while i < (*name).count as c_int
    {
      if (*(*name).name_records.offset (i as isize)).name_id as c_int == code
	{
	  /* The offsets within have already been validated.  */
	  *record = *(*name).name_records.offset (i as isize);
	  return (*name).data.offset ((*record).offset as isize);
	}
      i += 1;
    }

  return std::ptr::null_mut ();
}

/* Read the meta table from the given font FD, using the table
   directory specified as SUBTABLE.  Perform validation on the offsets
   in each metadata record.  Return NULL upon failure, else the meta
   table.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_meta_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_meta_table                                 // C 5799-5916
{
  let mut directory: *mut sfnt_table_directory;
  let mut meta: *mut sfnt_meta_table;
  let mut required: size_t;
  let mut i: size_t;
  let mut data_size: size_t;
  let mut map_size: size_t;
  let mut offset: size_t;
  let mut rc: ssize_t;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_META);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Figure out the minimum that has to be read.  */
  required = SFNT_ENDOF! (sfnt_meta_table, num_data_maps, uint32_t);

  if ((*directory).length as size_t) < required
    { return std::ptr::null_mut (); }

  /* Allocate enough to hold it.  */
  meta = xmalloc (std::mem::size_of::<sfnt_meta_table> ()) as *mut sfnt_meta_table;

  /* Read the header.  */
  rc = read (fd, meta as *mut c_void, required);
  if rc == -1 || (rc as size_t) < required
    {
      xfree (meta as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap what has been read so far.  */
  sfnt_swap32! (&raw mut (*meta).version);
  sfnt_swap32! (&raw mut (*meta).flags);
  sfnt_swap32! (&raw mut (*meta).data_offset);
  sfnt_swap32! (&raw mut (*meta).num_data_maps);

  /* Make sure the meta is supported.  */
  if (*meta).version != 1
    {
      xfree (meta as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Reallocate the table to hold sizeof *meta + meta->num_data_maps
     times sizeof meta->data_maps + directory->length bytes.  This is
     because it is ok for metadata to point into the data map itself,
     so an unswapped copy of the whole meta contents must be
     retained.  */

  let (ckd_map_size, mul_overflow) = std::mem::size_of::<sfnt_meta_data_map> ()
    .overflowing_mul ((*meta).num_data_maps as size_t);
  map_size = ckd_map_size;
  if mul_overflow
      /* Do so while checking for overflow from bad sfnt files.  */
    {
      xfree (meta as *mut c_void);
      return std::ptr::null_mut ();
    }
  if (*directory).length as size_t - required < map_size
    {
      xfree (meta as *mut c_void);
      return std::ptr::null_mut ();
    }
  let (ckd_data_size, add_overflow) = map_size
    .overflowing_add ((*directory).length as size_t);
  data_size = ckd_data_size;
  if add_overflow
    {
      xfree (meta as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Do the reallocation.  */
  meta = xaddrealloc (meta as *mut c_void, std::mem::size_of::<sfnt_meta_table> (),
		      data_size) as *mut sfnt_meta_table;

  /* Set pointers to data_maps and data.  */
  (*meta).data_maps = meta.offset (1) as *mut sfnt_meta_data_map;
  (*meta).data = (*meta).data_maps.offset ((*meta).num_data_maps as isize) as *mut u8;

  /* Now, seek back.  Read the entire table into meta->data.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    {
      xfree (meta as *mut c_void);
      return std::ptr::null_mut ();
    }

  rc = read (fd, (*meta).data as *mut c_void, (*directory).length as size_t);
  if rc < (*directory).length as ssize_t
    {
      xfree (meta as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Copy the data maps into meta->data_maps and swap them one by
     one.  */
  memcpy ((*meta).data_maps as *mut c_void,
	  (*meta).data.offset (required as isize) as *const c_void,
	  map_size);

  i = 0;
  while i < (*meta).num_data_maps as size_t
    {
      sfnt_swap32! (&raw mut (*(*meta).data_maps.offset (i as isize)).tag);
      sfnt_swap32! (&raw mut (*(*meta).data_maps.offset (i as isize)).data_offset);
      sfnt_swap32! (&raw mut (*(*meta).data_maps.offset (i as isize)).data_length);

      /* Verify the data offsets.  Overflow checking is particularly
	 important here.  */

      let (ckd_offset, add_overflow)
	= ((*(*meta).data_maps.offset (i as isize)).data_offset as size_t)
	  .overflowing_add ((*(*meta).data_maps.offset (i as isize)).data_length as size_t);
      offset = ckd_offset;
      if add_overflow
	{
	  xfree (meta as *mut c_void);
	  return std::ptr::null_mut ();
	}

      if offset > (*directory).length as size_t
	{
	  xfree (meta as *mut c_void);
	  return std::ptr::null_mut ();
	}
      i += 1;
    }

  /* All done.  */
  return meta;
}

/* Return a pointer to the metadata corresponding to TAG under the
   meta table META.  Return the start of the data and the metadata map
   under *MAP upon success, and NULL otherwise.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_find_metadata (meta: *mut sfnt_meta_table,
					     tag: sfnt_meta_data_tag,
					     map: *mut sfnt_meta_data_map)
  -> *mut c_char                                           // C 5922-5939
{
  let mut i: c_int;

  i = 0;
  while i < (*meta).num_data_maps as c_int
    {
      if (*(*meta).data_maps.offset (i as isize)).tag == tag as uint32_t
	{
	  *map = *(*meta).data_maps.offset (i as isize);
	  return ((*meta).data as *mut c_char).offset ((*map).data_offset as isize);
	}
      i += 1;
    }

  return std::ptr::null_mut ();
}

/* TrueType collection format support.  */

/* Read a TrueType collection header from the font file FD.
   FD must currently at the start of the file.

   Value is the header upon success, else NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_ttc_header (fd: c_int) -> *mut sfnt_ttc_header
{                                                          // C 5950-6032
  let mut ttc: *mut sfnt_ttc_header;
  let mut size: size_t;
  let mut i: size_t;
  let mut rc: ssize_t;

  /* First, allocate only as much as required.  */

  ttc = xmalloc (std::mem::size_of::<sfnt_ttc_header> ()) as *mut sfnt_ttc_header;

  /* Read the version 1.0 data.  */

  size = SFNT_ENDOF! (sfnt_ttc_header, num_fonts, uint32_t);
  rc = read (fd, ttc as *mut c_void, size);
  if rc == -1 || (rc as size_t) < size
    {
      xfree (ttc as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Now swap what was read.  */
  sfnt_swap32! (&raw mut (*ttc).ttctag);
  sfnt_swap32! (&raw mut (*ttc).version);
  sfnt_swap32! (&raw mut (*ttc).num_fonts);

  /* Verify that the tag is as expected.  */
  if (*ttc).ttctag != SFNT_TTC_TTCF as uint32_t
    {
      xfree (ttc as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Now, read the variable length data.  Make sure to check for
     overflow.  */

  let (ckd_size, mul_overflow) = ((*ttc).num_fonts as size_t)
    .overflowing_mul (std::mem::size_of::<uint32_t> ());
  size = ckd_size;
  if mul_overflow
    {
      xfree (ttc as *mut c_void);
      return std::ptr::null_mut ();
    }

  ttc = xaddrealloc (ttc as *mut c_void, std::mem::size_of::<sfnt_ttc_header> (),
		     size) as *mut sfnt_ttc_header;
  (*ttc).offset_table = ttc.offset (1) as *mut uint32_t;
  rc = read (fd, (*ttc).offset_table as *mut c_void, size);
  if rc == -1 || (rc as size_t) < size
    {
      xfree (ttc as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap each of the offsets read.  */
  i = 0;
  while i < (*ttc).num_fonts as size_t
    {
      sfnt_swap32! (&raw mut *(*ttc).offset_table.offset (i as isize));
      i += 1;
    }

  /* Now, look at the version.  If it is earlier than 2.0, then
     reading is finished.  */

  if (*ttc).version < 0x00020000
    { return ttc; }

  /* If it is 2.0 or later, then continue to read ul_dsig_tag to
     ul_dsig_offset.  */

  size = (SFNT_ENDOF! (sfnt_ttc_header, ul_dsig_offset, uint32_t)
	  - std::mem::offset_of! (sfnt_ttc_header, ul_dsig_tag));
  rc = read (fd, &raw mut (*ttc).ul_dsig_tag as *mut c_void, size);
  if rc == -1 || (rc as size_t) < size
    {
      xfree (ttc as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap what was read.  */
  sfnt_swap32! (&raw mut (*ttc).ul_dsig_tag);
  sfnt_swap32! (&raw mut (*ttc).ul_dsig_length);
  sfnt_swap32! (&raw mut (*ttc).ul_dsig_offset);

  /* All done.  */
  return ttc;
}

/* TrueType hinting support.

   If you do not read the code in this section in conjunction with
   Apple's TrueType Reference Manual, you will get very confused!

   TrueType fonts don't provide simple hinting meta data, unlike Type
   2 or CFF fonts.

   Instead, they come with a ``font program'', a bytecode program
   which is executed upon loading the font, a ``control value
   program'', executed upon font metrics changing, and then a ``glyph
   program'' for each glyph, which is run to fit its glyph after
   scaling.

   The virtual machine which runs this bytecode is arranged as such:

   Firstly, there is a set of registers known as the ``graphics
   state''.  Each time the point size of a font changes, the ``control
   value program'' is run to establish the default values of the
   ``graphics state''.  Then, before each glyph program is run, the
   ``graphics state'' is set back to the default values.

   Secondly, there is an address space which contains all instructions
   being run for the current program, which is addressed by the
   interpreter through its program counter and also by the
   instructions which push data on to the stack.

   Thirdly, there is a single stack, from which most instructions take
   their operands and store data.

   Then, there is some memory set aside for each font, the ``storage
   area'', which is addressed through the RS[] and WS[] instructions,
   and a ``control value table'', which is the `cvt ' table of the
   font.

   And finally, there is a ``glyph zone'' which holds points from a
   scaled glyph outline, and a ``twilight zone'', which holds points
   used by the font program itself.  Both are addressed indirectly
   through one of three ``zone pointer'' registers, and are accessible
   only when a program is being run on behalf of a glyph.  */


/* Functions for reading tables used by the TrueType interpreter.  */

/* Read the cvt table (control value table) from the given font FD,
   using the table directory specified as SUBTABLE.  Swap all values
   in the control value table.  Return NULL upon failure, else the cvt
   table.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_cvt_table (fd: c_int,
					      subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_cvt_table                                  // C 6086-6131
{
  let mut directory: *mut sfnt_table_directory;
  let mut required: size_t;
  let mut i: size_t;
  let mut rc: ssize_t;
  let mut cvt: *mut sfnt_cvt_table;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_CVT);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Figure out the minimum amount that has to be read.  */
  let (ckd_required, add_overflow) = ((*directory).length as size_t)
    .overflowing_add (std::mem::size_of::<sfnt_cvt_table> ());
  required = ckd_required;
  if add_overflow
    { return std::ptr::null_mut (); }

  /* Allocate enough for that much data.  */
  cvt = xmalloc (required) as *mut sfnt_cvt_table;

  /* Now set cvt->num_elements as appropriate, and make cvt->values
     point into the values.  */
  (*cvt).num_elements = (*directory).length as size_t / 2;
  (*cvt).values = cvt.offset (1) as *mut sfnt_fword;

  /* Read into cvt.  */
  rc = read (fd, (*cvt).values as *mut c_void, (*directory).length as size_t);
  if rc != (*directory).length as ssize_t
    {
      xfree (cvt as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap each element in the control value table.  */
  i = 0;
  while i < (*cvt).num_elements
    {
      sfnt_swap16! (&raw mut *(*cvt).values.offset (i as isize));
      i += 1;
    }

  /* All done.  */
  return cvt;
}

/* Read the fpgm table from the given font FD, using the table
   directory specified as SUBTABLE.  Value is NULL upon failure, else
   the fpgm table.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_fpgm_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_fpgm_table                                 // C 6137-6179
{
  let mut directory: *mut sfnt_table_directory;
  let mut required: size_t;
  let mut rc: ssize_t;
  let mut fpgm: *mut sfnt_fpgm_table;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_FPGM);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Figure out the minimum amount that has to be read.  */
  let (ckd_required, add_overflow) = ((*directory).length as size_t)
    .overflowing_add (std::mem::size_of::<sfnt_fpgm_table> ());
  required = ckd_required;
  if add_overflow
    { return std::ptr::null_mut (); }

  /* Allocate enough for that much data.  */
  fpgm = xaddmalloc (std::mem::size_of::<sfnt_fpgm_table> (),
		     (*directory).length as size_t) as *mut sfnt_fpgm_table;

  /* Now set fpgm->num_instructions as appropriate, and make
     fpgm->instructions point to the right place.  */

  (*fpgm).num_instructions = (*directory).length as size_t;
  (*fpgm).instructions = fpgm.offset (1) as *mut u8;

  /* Read into fpgm.  */
  rc = read (fd, (*fpgm).instructions as *mut c_void, (*directory).length as size_t);
  if rc != (*directory).length as ssize_t
    {
      xfree (fpgm as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* All done.  */
  return fpgm;
}

/* Read the prep table from the given font FD, using the table
   directory specified as SUBTABLE.  Value is NULL upon failure, else
   the prep table.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_prep_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_prep_table                                 // C 6185-6227
{
  let mut directory: *mut sfnt_table_directory;
  let mut required: size_t;
  let mut rc: ssize_t;
  let mut prep: *mut sfnt_prep_table;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_PREP);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Figure out the minimum amount that has to be read.  */
  let (ckd_required, add_overflow) = ((*directory).length as size_t)
    .overflowing_add (std::mem::size_of::<sfnt_prep_table> ());
  required = ckd_required;
  if add_overflow
    { return std::ptr::null_mut (); }

  /* Allocate enough for that much data.  */
  prep = xaddmalloc (std::mem::size_of::<sfnt_prep_table> (),
		     (*directory).length as size_t) as *mut sfnt_prep_table;

  /* Now set prep->num_instructions as appropriate, and make
     prep->instructions point to the right place.  */

  (*prep).num_instructions = (*directory).length as size_t;
  (*prep).instructions = prep.offset (1) as *mut u8;

  /* Read into prep.  */
  rc = read (fd, (*prep).instructions as *mut c_void, (*directory).length as size_t);
  if rc != (*directory).length as ssize_t
    {
      xfree (prep as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* All done.  */
  return prep;
}

/* Interpreter execution environment.  */

/* Divide the specified two 26.6 fixed point numbers X and Y.
   Return the result.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_div_f26dot6 (x: sfnt_f26dot6, y: sfnt_f26dot6) -> sfnt_f26dot6
{                                                          // C 6236-6240
  return (x as int64_t * ((1 << 6) as int64_t) / y as int64_t) as sfnt_f26dot6;
}

/* Multiply the specified two 26.6 fixed point numbers A and B.
   Return the result, or an undefined value upon overflow.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_mul_f26dot6 (a: sfnt_f26dot6, b: sfnt_f26dot6) -> sfnt_f26dot6
{                                                          // C 6245-6249
  return (a as int64_t * b as int64_t / ((1 << 6) as int64_t)) as sfnt_f26dot6;
}

/* Multiply the specified two 26.6 fixed point numbers A and B, with
   rounding.  Return the result, or an undefined value upon
   overflow.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_mul_f26dot6_round (a: sfnt_f26dot6, b: sfnt_f26dot6) -> sfnt_f26dot6
{                                                          // C 6255-6259
  return ((a as int64_t * b as int64_t + ((1 << 5) as int64_t))
    / ((1 << 6) as int64_t)) as sfnt_f26dot6;
}

/* Multiply the specified 2.14 number with another signed 32 bit
   number.  Return the result as a signed 32 bit number.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_mul_f2dot14 (a: sfnt_f2dot14, b: int32_t) -> int32_t
{                                                          // C 6264-6268
  return (a as int64_t * b as int64_t / ((1 << 14) as int64_t)) as int32_t;
}

/* Multiply the specified 26.6 fixed point number X by the specified
   16.16 fixed point number Y with rounding.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_mul_f26dot6_fixed (x: sfnt_f26dot6, y: sfnt_fixed) -> sfnt_f26dot6
{                                                          // C 6273-6277
  return sfnt_mul_fixed_round (x, y);
}

/* Return the floor of the specified 26.6 fixed point value X.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_floor_f26dot6 (x: sfnt_f26dot6) -> sfnt_f26dot6
{                                                          // C 6281-6285
  return (x as uint32_t & 0o37777777700) as sfnt_f26dot6;
}

/* Return the ceiling of the specified 26.6 fixed point value X.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_ceil_f26dot6 (x: sfnt_f26dot6) -> sfnt_f26dot6
{                                                          // C 6289-6293
  return (x + 0o77) & !0o77;
}

/* Return the 26.6 fixed point value X rounded to the nearest integer
   value.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_round_f26dot6 (mut x: sfnt_f26dot6) -> sfnt_f26dot6
{                                                          // C 6298-6306
  /* Add 0.5.  */
  x += 0o40;

  /* Remove the fractional.  */
  return x & !0o77;
}

/* C 6307's blank line, then C 6308-6309's comment "Needed by sfnt_init_graphics_state." and
   C 6310's `static void sfnt_validate_gs (struct sfnt_graphics_state *);`.  Rust has no
   prototype for a `static` function (PORTING.md 0.1-12 deletes C's forward declarations), and
   the definition this prototype names stands in this unit at C 10538's own position (slice 7,
   P-91 ①) - which is why the two items below can call it.  */

/* Set up default values for the interpreter graphics state.  Return
   them in STATE.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_init_graphics_state (state: *mut sfnt_graphics_state)
{                                                          // C 6316-6344
  (*state).auto_flip = true;
  (*state).cvt_cut_in = 0o104;
  (*state).delta_base = 9;
  (*state).delta_shift = 3;
  (*state).dual_projection_vector.x = 0o40000; /* 1.0 */
  (*state).dual_projection_vector.y = 0;
  (*state).freedom_vector.x = 0o40000; /* 1.0 */
  (*state).freedom_vector.y = 0;
  (*state).instruct_control = 0;
  (*state).r#loop = 1;
  (*state).minimum_distance = 0o100;
  (*state).projection_vector.x = 0o40000; /* 1.0 */
  (*state).projection_vector.y = 0;
  (*state).round_state = 1;
  (*state).rp0 = 0;
  (*state).rp1 = 0;
  (*state).rp2 = 0;
  (*state).scan_control = 0;
  (*state).sw_cut_in = 0;
  (*state).single_width_value = 0;
  (*state).zp0 = 1;
  (*state).zp1 = 1;
  (*state).zp2 = 1;

  /* Validate the graphics state.  */
  sfnt_validate_gs (state);
}

/* Set up an interpreter to be used with a font.  Use the resource
   limits specified in the MAXP table, the values specified in the
   CVT, HEAD and FVAR tables, the pixel size PIXEL_SIZE, and the point
   size POINT_SIZE.  CVT may be NULL, in which case the interpreter
   will not have access to a control value table.

   POINT_SIZE should be PIXEL_SIZE, converted to 1/72ths of an inch.

   Value is the interpreter, with all state initialized to default
   values, or NULL upon failure.  */

/* C 6427-6431's `#ifdef TEST` arm assigns `interpreter->run_hook`/`push_hook`/`pop_hook` - the
   three members `include/sfnt.rs` 2153-2157 deliberately does not transcribe (C 2026-2038's
   `#ifdef TEST` is dead text there).  The arm has no Rust text until the mirror carries the
   members; the ledger's Owed 12(b) records the ruling and this lane's `## Deviations`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_make_interpreter (maxp: *mut sfnt_maxp_table,
						cvt: *mut sfnt_cvt_table,
						head: *mut sfnt_head_table,
						fvar: *mut sfnt_fvar_table,
						pixel_size: c_int, point_size: c_int)
  -> *mut sfnt_interpreter                                // C 6358-6522
{
  let mut size: size_t;
  let mut temp: size_t;
  let mut i: size_t;
  let mut storage_size: size_t;
  let mut pad: size_t;
  let mut interpreter: *mut sfnt_interpreter;

  /* Detect CFF maxp tables.  */
  if (*maxp).version != 0x00010000
    { return std::ptr::null_mut (); }

  /* Use the contents of the MAXP table to determine the size of the
     interpreter structure.  */
  size = std::mem::size_of::<sfnt_interpreter> ();

  /* Add program stack.  */
  let (ckd_size, add_overflow)
    = size.overflowing_add ((*maxp).max_stack_elements as size_t
			    * std::mem::size_of::<uint32_t> ());
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  /* Add twilight zone.  */

  let (ckd_size, add_overflow)
    = size.overflowing_add ((*maxp).max_twilight_points as size_t
			    * std::mem::size_of::<sfnt_f26dot6> ());
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  let (ckd_size, add_overflow)
    = size.overflowing_add ((*maxp).max_twilight_points as size_t
			    * std::mem::size_of::<sfnt_f26dot6> ());
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  let (ckd_size, add_overflow)
    = size.overflowing_add ((*maxp).max_twilight_points as size_t
			    * std::mem::size_of::<sfnt_f26dot6> ());
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  let (ckd_size, add_overflow)
    = size.overflowing_add ((*maxp).max_twilight_points as size_t
			    * std::mem::size_of::<sfnt_f26dot6> ());
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  /* Add the storage area.  */
  storage_size = (*maxp).max_storage as size_t * std::mem::size_of::<uint32_t> ();
  let (ckd_size, add_overflow) = size.overflowing_add (storage_size);
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  /* Add padding for the storage area.  */
  pad = std::mem::align_of::<sfnt_interpreter_definition> ();
  pad -= size & (pad - 1);
  let (ckd_size, add_overflow) = size.overflowing_add (pad);
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  /* Add function and instruction definitions.  */
  let (ckd_size, add_overflow)
    = size.overflowing_add ((((*maxp).max_instruction_defs as c_int
			      + (*maxp).max_function_defs as c_int) as size_t)
			    * std::mem::size_of::<sfnt_interpreter_definition> ());
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  /* Add control value table.  */

  if !cvt.is_null ()
    {
      let (ckd_temp, mul_overflow)
	= (*cvt).num_elements.overflowing_mul (std::mem::size_of::<sfnt_f26dot6> ());
      temp = ckd_temp;
      if mul_overflow
	{ return std::ptr::null_mut (); }
      let (ckd_size, add_overflow) = size.overflowing_add (temp);
      size = ckd_size;
      if add_overflow
	{ return std::ptr::null_mut (); }
    }

  /* Allocate the interpreter.  */
  interpreter = xmalloc (size) as *mut sfnt_interpreter;

  /* Fill in pointers and default values.  */
  (*interpreter).max_stack_elements = (*maxp).max_stack_elements;
  (*interpreter).num_instructions = 0;
  (*interpreter).IP = 0;
  (*interpreter).storage_size = (*maxp).max_storage;
  (*interpreter).function_defs_size = (*maxp).max_function_defs;
  (*interpreter).instruction_defs_size = (*maxp).max_instruction_defs;
  (*interpreter).twilight_zone_size = (*maxp).max_twilight_points;
  (*interpreter).scale = 0; /* This should be set later.  */

  (*interpreter).stack = interpreter.offset (1) as *mut uint32_t;
  (*interpreter).SP = (*interpreter).stack;
  (*interpreter).instructions = std::ptr::null_mut ();
  (*interpreter).twilight_x
    = (*interpreter).stack.offset ((*maxp).max_stack_elements as isize)
      as *mut sfnt_f26dot6;
  (*interpreter).twilight_y = (*interpreter).twilight_x
			      .offset ((*maxp).max_twilight_points as isize);
  (*interpreter).twilight_original_x = (*interpreter).twilight_y
				      .offset ((*maxp).max_twilight_points as isize);
  (*interpreter).twilight_original_y
    = (*interpreter).twilight_original_x
      .offset ((*maxp).max_twilight_points as isize);
  (*interpreter).glyph_zone = std::ptr::null_mut ();
  (*interpreter).advance_width = 0;
  (*interpreter).storage
    = (*interpreter).twilight_original_y
      .offset ((*maxp).max_twilight_points as isize) as *mut uint32_t;
  (*interpreter).function_defs
    = (*interpreter).storage.offset ((*maxp).max_storage as isize)
      as *mut sfnt_interpreter_definition;
  (*interpreter).function_defs
    = ((*interpreter).function_defs as *mut u8).offset (pad as isize)
      as *mut sfnt_interpreter_definition;
  (*interpreter).instruction_defs = (*interpreter).function_defs
				   .offset ((*maxp).max_function_defs as isize);
  (*interpreter).cvt
    = (*interpreter).instruction_defs
      .offset ((*maxp).max_instruction_defs as isize) as *mut sfnt_f26dot6;

  if !cvt.is_null ()
    { (*interpreter).cvt_size = (*cvt).num_elements as usize; }
  else
    { (*interpreter).cvt_size = 0; }

  /* Now compute the scale.  Then, scale up the control value table
     values.  */
  (*interpreter).scale
    = sfnt_div_fixed (pixel_size * 64, (*head).units_per_em as sfnt_fixed);

  /* Set the PPEM.  */
  (*interpreter).ppem = pixel_size;
  (*interpreter).point_size = point_size;

  /* Zero out the interpreter state from the stack to the end of the
     instruction definitions.  */
  memset ((*interpreter).stack as *mut c_void, 0,
	  size - std::mem::size_of::<sfnt_interpreter> ());

  /* Initialize the interpreter graphics state.  */
  sfnt_init_graphics_state (&raw mut (*interpreter).state);

  /* Load the control value table.  */
  i = 0;
  while i < (*interpreter).cvt_size
    {
      *(*interpreter).cvt.offset (i as isize)
	= sfnt_mul_f26dot6_fixed (*(*cvt).values.offset (i as isize) as sfnt_f26dot6,
				  (*interpreter).scale);
      i += 1;
    }

  /* Fill in the default values for phase, period and threshold.  */
  (*interpreter).period = 64;
  (*interpreter).phase = 0;
  (*interpreter).threshold = 0;

  /* Fill in the current call depth.  */
  (*interpreter).call_depth = 0;

  /* Clear variation axes.  They will be set upon a call to
     `sfnt_vary_interpreter'.  */
  (*interpreter).n_axis = 0;
  (*interpreter).norm_coords = std::ptr::null_mut ();

  /* Set n_axis now if a fvar table was provided.  This way, GXAXIS
     pushes the correct number of values even if no blend is
     provided.  */

  if !fvar.is_null ()
    { (*interpreter).n_axis = (*fvar).axis_count as c_int; }

  /* Return the interpreter.  */
  return interpreter;
}

/* These enums are used to determine why the interpreter is being
   run.  They have the following meanings:

     - The interpreter is being run to interpret a font program.
     - The interpreter is being run to interpret the control
       value program.
     - The interpreter is being run to fit a glyph.
     - The interpreter is being run as part of an automated test.  */

/// C 6533-6541: C's `enum sfnt_interpreter_run_context`, both arms; the
/// `SFNT_RUN_CONTEXT_TEST` member sits behind C 6538's `#ifdef TEST`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type sfnt_interpreter_run_context = c_int;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SFNT_RUN_CONTEXT_FONT_PROGRAM: sfnt_interpreter_run_context = 0;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SFNT_RUN_CONTEXT_CONTROL_VALUE_PROGRAM: sfnt_interpreter_run_context = 1;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SFNT_RUN_CONTEXT_GLYPH_PROGRAM: sfnt_interpreter_run_context = 2;
#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "test"))]
pub const SFNT_RUN_CONTEXT_TEST: sfnt_interpreter_run_context = 3;

/* Cancel execution of the program in INTERPRETER with the specified
   error REASON and reset the loop counter to 1.

   After this is called, it is probably okay to reuse INTERPRETER.
   However, instructions must always be reloaded.  */

/* C 6549's `static AVOID` is `_Noreturn ATTRIBUTE_COLD void` (lisp.h 153; the port writes
   `-> !`, `docs/ports/excluded-macros.txt`'s `AVOID` row).  C 6556's `longjmp` is the first
   plain `longjmp` in this tree: `<setjmp.h>` has no mirror, so the name is declared in this
   unit at C 37's site above (P-88 ①) - the call keeps C's name and C's argument.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_trap (interpreter: *mut sfnt_interpreter,
			       reason: *const c_char) -> !
{                                                          // C 6550-6557
  (*interpreter).trap_reason = reason;
  (*interpreter).call_depth = 0;
  (*interpreter).state.r#loop = 1;
  longjmp (&raw mut (*interpreter).trap, 1);
}

/* Multiply the specified MAGNITUDE by the contents of INTERPRETER's
   freedom vector and return the result in *DX and *DY.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_scale_by_freedom_vector (interpreter: *mut sfnt_interpreter,
					magnitude: sfnt_f26dot6, dx: *mut sfnt_f26dot6,
					dy: *mut sfnt_f26dot6)
{                                                          // C 8162-8171
  let mut vector: *mut sfnt_unit_vector;

  vector = &raw mut (*interpreter).state.freedom_vector;
  *dx = sfnt_mul_f2dot14 ((*vector).x, magnitude);
  *dy = sfnt_mul_f2dot14 ((*vector).y, magnitude);
}

/* Interpret a UTP instruction with the point P in INTERPRETER.
   Unset any ``touched'' flag inside the point P, relative to the
   zone in INTERPRETER's ZP0 register.

   Trap upon encountering an out of bounds point.  */

/* C 6562's `#define TRAP(why) sfnt_interpret_trap (interpreter, why)` is a `macro_rules!`
   *here*, not at C's file scope: its body names the `interpreter` parameter of this function, and
   a `macro_rules!` resolves local names at its **definition** site (PORTING.md P-57, the
   `coding.rs`/`ccl.rs` shape).  Every using function carries its own copy and every call site
   keeps C's own `TRAP (…)` spelling (with the `!` every `macro_rules!` call needs).  C's argument
   is a string literal that reaches the `const char *` parameter through C's array decay, so each
   site writes `c"…".as_ptr ()` - the tree's spelling for that decay (`alloc.rs` 2342).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_utp (interpreter: *mut sfnt_interpreter,
			      p: uint32_t)
{                                                          // C 8180-8223
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut mask: c_uchar;

  if (*interpreter).state.zp0 == 0
    {
      if p >= (*interpreter).twilight_zone_size as uint32_t
	{ TRAP! (c"UTP[] p lies outside twilight zone".as_ptr ()); }

      /* There are no flags in the twilight zone.  */
      return;
    }

  if (*interpreter).glyph_zone.is_null ()
      || (p as usize) >= (*(*interpreter).glyph_zone).num_points
    { TRAP! (c"UTP[] p lies outside glyph zone".as_ptr ()); }

  /* The flags unset by UTP are subject to which axes in the freedom
     vector are significant, as stated in the TrueType reference
     manual by this needless mouthful:

       A point may be touched in the x-direction, the y-direction, or
       in both the x and y-directions.  The position of the freedom
       vector determines whether the point is untouched in the
       x-direction, the y-direction, or both.  If the vector is set to
       the x-axis, the point will be untouched in the x-direction.  If
       the vector is set to the y-axis, the point will be untouched in
       the y-direction.  Otherwise the point will be untouched in both
       directions.

       A points that is marked as untouched will be moved by an IUP[]
       instruction even if the point was previously touched.  */

  mask = 0xff;

  if (*interpreter).state.freedom_vector.x != 0
    { mask &= !(SFNT_POINT_TOUCHED_X as c_uchar); }

  if (*interpreter).state.freedom_vector.y != 0
    { mask &= !(SFNT_POINT_TOUCHED_Y as c_uchar); }

  *(*(*interpreter).glyph_zone).flags.offset (p as isize) &= mask;
}

/* Save the specified unit VECTOR into INTERPRETER's graphics state as
   both the projection and the dual projection vectors.

   If not DUAL_ONLY, set VECTOR as both the projection and dual
   projection vectors.  Otherwise, only set VECTOR as the dual
   projection vector.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_save_projection_vector (interpreter: *mut sfnt_interpreter,
				       vector: *mut sfnt_unit_vector,
				       dual_only: bool)
{                                                          // C 8233-8243
  if !dual_only
    { (*interpreter).state.projection_vector = *vector; }

  (*interpreter).state.dual_projection_vector = *vector;

  sfnt_validate_gs (&raw mut (*interpreter).state);
}

/* Save the specified unit VECTOR into INTERPRETER's graphics
   state.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_save_freedom_vector (interpreter: *mut sfnt_interpreter,
				    vector: *mut sfnt_unit_vector)
{                                                          // C 8249-8255
  (*interpreter).state.freedom_vector = *vector;

  sfnt_validate_gs (&raw mut (*interpreter).state);
}

/* Return the values of the point NUMBER in the zone pointed to by
   INTERPRETER's ZP2 register.

   If X_ORG and Y_ORG are set, return the original values (prior to
   any instruction interpretations) in those two locations.

   Trap if NUMBER is out of bounds or the zone is inaccessible.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_address_zp2 (interpreter: *mut sfnt_interpreter,
			    number: uint32_t,
			    x: *mut sfnt_f26dot6, y: *mut sfnt_f26dot6,
			    x_org: *mut sfnt_f26dot6, y_org: *mut sfnt_f26dot6)
{                                                          // C 8266-8313
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  if (*interpreter).state.zp2 == 0
    {
      /* Address the twilight zone.  */
      if number >= (*interpreter).twilight_zone_size as uint32_t
	{ TRAP! (c"address to ZP2 (twilight zone) out of bounds".as_ptr ()); }

      'next: {
	if !x.is_null () && !y.is_null ()
	  {
	    *x = *(*interpreter).twilight_x.offset (number as isize);
	    *y = *(*interpreter).twilight_y.offset (number as isize);
	  }
      }

      if x_org.is_null () || y_org.is_null ()
	{ return; }

      /* The twilight zone is initially all zero, but initial
	 positions can still be changed.  */
      *x_org = *(*interpreter).twilight_original_x.offset (number as isize);
      *y_org = *(*interpreter).twilight_original_y.offset (number as isize);
      return;
    }

  /* Address the glyph zone.  */
  if (*interpreter).glyph_zone.is_null ()
    { TRAP! (c"address to ZP2 (glyph zone) points into unset zone".as_ptr ()); }

  if (number as usize) >= (*(*interpreter).glyph_zone).num_points
    { TRAP! (c"address to ZP2 (glyph zone) out of bounds".as_ptr ()); }

  if !x.is_null () && !y.is_null ()
    {
      *x = *(*(*interpreter).glyph_zone).x_current.offset (number as isize);
      *y = *(*(*interpreter).glyph_zone).y_current.offset (number as isize);
    }

  if !x_org.is_null () && !y_org.is_null ()
    {
      *x_org = *(*(*interpreter).glyph_zone).x_points.offset (number as isize);
      *y_org = *(*(*interpreter).glyph_zone).y_points.offset (number as isize);
    }
}

/* Return the values of the point NUMBER in the zone pointed to by
   INTERPRETER's ZP1 register.

   Trap if NUMBER is out of bounds or the zone is inaccessible.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_address_zp1 (interpreter: *mut sfnt_interpreter,
			    number: uint32_t,
			    x: *mut sfnt_f26dot6, y: *mut sfnt_f26dot6,
			    x_org: *mut sfnt_f26dot6, y_org: *mut sfnt_f26dot6)
{                                                          // C 8321-8368
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  if (*interpreter).state.zp1 == 0
    {
      /* Address the twilight zone.  */
      if number >= (*interpreter).twilight_zone_size as uint32_t
	{ TRAP! (c"address to ZP1 (twilight zone) out of bounds".as_ptr ()); }

      'next: {
	if !x.is_null () && !y.is_null ()
	  {
	    *x = *(*interpreter).twilight_x.offset (number as isize);
	    *y = *(*interpreter).twilight_y.offset (number as isize);
	  }
      }

      if x_org.is_null () || y_org.is_null ()
	{ return; }

      /* The twilight zone is initially all zero, but initial
	 positions can still be changed.  */
      *x_org = *(*interpreter).twilight_original_x.offset (number as isize);
      *y_org = *(*interpreter).twilight_original_y.offset (number as isize);
      return;
    }

  /* Address the glyph zone.  */
  if (*interpreter).glyph_zone.is_null ()
    { TRAP! (c"address to ZP1 (glyph zone) points into unset zone".as_ptr ()); }

  if (number as usize) >= (*(*interpreter).glyph_zone).num_points
    { TRAP! (c"address to ZP1 (glyph zone) out of bounds".as_ptr ()); }

  if !x.is_null () && !y.is_null ()
    {
      *x = *(*(*interpreter).glyph_zone).x_current.offset (number as isize);
      *y = *(*(*interpreter).glyph_zone).y_current.offset (number as isize);
    }

  if !x_org.is_null () && !y_org.is_null ()
    {
      *x_org = *(*(*interpreter).glyph_zone).x_points.offset (number as isize);
      *y_org = *(*(*interpreter).glyph_zone).y_points.offset (number as isize);
    }
}

/* Return the values of the point NUMBER in the zone pointed to by
   INTERPRETER's ZP0 register.

   Trap if NUMBER is out of bounds or the zone is inaccessible.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_address_zp0 (interpreter: *mut sfnt_interpreter,
			    number: uint32_t,
			    x: *mut sfnt_f26dot6, y: *mut sfnt_f26dot6,
			    x_org: *mut sfnt_f26dot6, y_org: *mut sfnt_f26dot6)
{                                                          // C 8376-8423
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  if (*interpreter).state.zp0 == 0
    {
      /* Address the twilight zone.  */
      if number >= (*interpreter).twilight_zone_size as uint32_t
	{ TRAP! (c"address to ZP0 (twilight zone) out of bounds".as_ptr ()); }

      'next: {
	if !x.is_null () && !y.is_null ()
	  {
	    *x = *(*interpreter).twilight_x.offset (number as isize);
	    *y = *(*interpreter).twilight_y.offset (number as isize);
	  }
      }

      if x_org.is_null () || y_org.is_null ()
	{ return; }

      /* The twilight zone is initially all zero, but initial
	 positions can still be changed.  */
      *x_org = *(*interpreter).twilight_original_x.offset (number as isize);
      *y_org = *(*interpreter).twilight_original_y.offset (number as isize);
      return;
    }

  /* Address the glyph zone.  */
  if (*interpreter).glyph_zone.is_null ()
    { TRAP! (c"address to ZP0 (glyph zone) points into unset zone".as_ptr ()); }

  if (number as usize) >= (*(*interpreter).glyph_zone).num_points
    { TRAP! (c"address to ZP0 (glyph zone) out of bounds".as_ptr ()); }

  if !x.is_null () && !y.is_null ()
    {
      *x = *(*(*interpreter).glyph_zone).x_current.offset (number as isize);
      *y = *(*(*interpreter).glyph_zone).y_current.offset (number as isize);
    }

  if !x_org.is_null () && !y_org.is_null ()
    {
      *x_org = *(*(*interpreter).glyph_zone).x_points.offset (number as isize);
      *y_org = *(*(*interpreter).glyph_zone).y_points.offset (number as isize);
    }
}

/* Set the point NUMBER in the zone referenced by INTERPRETER's ZP2
   register to the specified X and Y.

   Apply FLAGS to NUMBER's flags in that zone.  Trap if NUMBER is out
   of bounds.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_store_zp2 (interpreter: *mut sfnt_interpreter,
			  number: uint32_t, x: sfnt_f26dot6, y: sfnt_f26dot6,
			  flags: c_int)
{                                                          // C 8432-8458
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  if (*interpreter).state.zp2 == 0
    {
      /* Address the twilight zone.  */
      if number >= (*interpreter).twilight_zone_size as uint32_t
	{ TRAP! (c"address to ZP2 (twilight zone) out of bounds".as_ptr ()); }

      *(*interpreter).twilight_x.offset (number as isize) = x;
      *(*interpreter).twilight_y.offset (number as isize) = y;
      return;
    }

  /* Address the glyph zone.  */
  if (*interpreter).glyph_zone.is_null ()
    { TRAP! (c"address to ZP0 (glyph zone) points into unset zone".as_ptr ()); }

  if (number as usize) >= (*(*interpreter).glyph_zone).num_points
    { TRAP! (c"address to ZP0 (glyph zone) out of bounds".as_ptr ()); }

  *(*(*interpreter).glyph_zone).x_current.offset (number as isize) = x;
  *(*(*interpreter).glyph_zone).y_current.offset (number as isize) = y;
  *(*(*interpreter).glyph_zone).flags.offset (number as isize) |= flags as u8;
}

/* C 8460-8487: `#if 0` around `sfnt_line_to_standard_form` - dead in every configuration, so the
   port records the region instead of carrying it (P-86 ①; the `android.md` 1125-1360 /
   `androidfns.md` 1015-1022 precedent).  `tools/cinventory.py` skips `#if 0` unconditionally.  */

/* Check that the specified POINT lies within the zone addressed by
   INTERPRETER's ZP2 register.  Trap if it does not.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_check_zp2 (interpreter: *mut sfnt_interpreter, point: uint32_t)
{                                                          // C 8492-8502
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  if (*interpreter).state.zp2 == 0
    {
      if point >= (*interpreter).twilight_zone_size as uint32_t
	{ TRAP! (c"point lies outside twilight zone (ZP2)".as_ptr ()); }
    }
  else if (*interpreter).glyph_zone.is_null ()
	   || (point as usize) >= (*(*interpreter).glyph_zone).num_points
    { TRAP! (c"point lies outside glyph zone (ZP2)".as_ptr ()); }
}

/* Check that the specified POINT lies within the zone addressed by
   INTERPRETER's ZP0 register.  Trap if it does not.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_check_zp0 (interpreter: *mut sfnt_interpreter, point: uint32_t)
{                                                          // C 8508-8518
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  if (*interpreter).state.zp0 == 0
    {
      if point >= (*interpreter).twilight_zone_size as uint32_t
	{ TRAP! (c"point lies outside twilight zone (ZP0)".as_ptr ()); }
    }
  else if (*interpreter).glyph_zone.is_null ()
	   || (point as usize) >= (*(*interpreter).glyph_zone).num_points
    { TRAP! (c"point lies outside glyph zone (ZP0)".as_ptr ()); }
}

/* Check that the specified POINT lies within the zone addressed by
   INTERPRETER's ZP1 register.  Trap if it does not.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_check_zp1 (interpreter: *mut sfnt_interpreter, point: uint32_t)
{                                                          // C 8524-8534
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  if (*interpreter).state.zp1 == 0
    {
      if point >= (*interpreter).twilight_zone_size as uint32_t
	{ TRAP! (c"point lies outside twilight zone (ZP0)".as_ptr ()); }
    }
  else if (*interpreter).glyph_zone.is_null ()
	   || (point as usize) >= (*(*interpreter).glyph_zone).num_points
    { TRAP! (c"point lies outside glyph zone (ZP0)".as_ptr ()); }
}

/* Move N points starting from the specified POINT in the zone
   addressed by INTERPRETER's ZP0 register by the given DISTANCE along
   the freedom vector.

   No checking is done to ensure that POINT lies inside the zone, or
   even that the zone exists at all.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_move_zp0 (interpreter: *mut sfnt_interpreter, point: uint32_t,
			 n: size_t, distance: sfnt_f26dot6)
{                                                          // C 8544-8556
  if (*interpreter).state.zp0 == 0
    { (*interpreter).state.r#move.unwrap () (
	&raw mut *(*interpreter).twilight_x.offset (point as isize),
	&raw mut *(*interpreter).twilight_y.offset (point as isize),
	n, interpreter, distance, std::ptr::null_mut ()); }
  else
    { (*interpreter).state.r#move.unwrap () (
	&raw mut *(*(*interpreter).glyph_zone).x_current.offset (point as isize),
	&raw mut *(*(*interpreter).glyph_zone).y_current.offset (point as isize),
	n, interpreter, distance,
	&raw mut *(*(*interpreter).glyph_zone).flags.offset (point as isize)); }
}

/* Move N points starting from the specified POINT in the zone
   addressed by INTERPRETER's ZP1 register by the given DISTANCE along
   the freedom vector.

   No checking is done to ensure that POINT lies inside the zone, or
   even that the zone exists at all.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_move_zp1 (interpreter: *mut sfnt_interpreter, point: uint32_t,
			 n: size_t, distance: sfnt_f26dot6)
{                                                          // C 8566-8578
  if (*interpreter).state.zp1 == 0
    { (*interpreter).state.r#move.unwrap () (
	&raw mut *(*interpreter).twilight_x.offset (point as isize),
	&raw mut *(*interpreter).twilight_y.offset (point as isize),
	n, interpreter, distance, std::ptr::null_mut ()); }
  else
    { (*interpreter).state.r#move.unwrap () (
	&raw mut *(*(*interpreter).glyph_zone).x_current.offset (point as isize),
	&raw mut *(*(*interpreter).glyph_zone).y_current.offset (point as isize),
	n, interpreter, distance,
	&raw mut *(*(*interpreter).glyph_zone).flags.offset (point as isize)); }
}

/* Move N points starting from the specified POINT in the zone
   addressed by INTERPRETER's ZP2 register by the given DISTANCE along
   the freedom vector.

   No checking is done to ensure that POINT lies inside the zone, or
   even that the zone exists at all.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_move_zp2 (interpreter: *mut sfnt_interpreter, point: uint32_t,
			 n: size_t, distance: sfnt_f26dot6)
{                                                          // C 8588-8600
  if (*interpreter).state.zp2 == 0
    { (*interpreter).state.r#move.unwrap () (
	&raw mut *(*interpreter).twilight_x.offset (point as isize),
	&raw mut *(*interpreter).twilight_y.offset (point as isize),
	n, interpreter, distance, std::ptr::null_mut ()); }
  else
    { (*interpreter).state.r#move.unwrap () (
	&raw mut *(*(*interpreter).glyph_zone).x_current.offset (point as isize),
	&raw mut *(*(*interpreter).glyph_zone).y_current.offset (point as isize),
	n, interpreter, distance,
	&raw mut *(*(*interpreter).glyph_zone).flags.offset (point as isize)); }
}

/* Move N points from the specified POINT in INTERPRETER's glyph zone
   by the given DISTANCE along the freedom vector.

   Do not touch the points that are moved.

   No checking is done to ensure that POINT lies inside the zone, or
   even that the zone exists at all.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_move_glyph_zone (interpreter: *mut sfnt_interpreter, point: uint32_t,
				n: size_t, distance: sfnt_f26dot6)
{                                                          // C 8611-8617
  (*interpreter).state.r#move.unwrap () (
    &raw mut *(*(*interpreter).glyph_zone).x_current.offset (point as isize),
    &raw mut *(*(*interpreter).glyph_zone).y_current.offset (point as isize),
    n, interpreter, distance, std::ptr::null_mut ());
}

/* Move N points from the specified POINT in INTERPRETER's twilight
   zone by the given DISTANCE along the freedom vector.

   Do not touch the points that are moved.

   No checking is done to ensure that POINT lies inside the zone, or
   even that the zone exists at all.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_move_twilight_zone (interpreter: *mut sfnt_interpreter, point: uint32_t,
				   n: size_t, distance: sfnt_f26dot6)
{                                                          // C 8628-8634
  (*interpreter).state.r#move.unwrap () (
    &raw mut *(*interpreter).twilight_x.offset (point as isize),
    &raw mut *(*interpreter).twilight_y.offset (point as isize),
    n, interpreter, distance, std::ptr::null_mut ());
}

/* Move the point P in the zone pointed to by the ZP2 register in
   INTERPRETER's graphics state by DX, and DY.

   Touch the point P in the directions of the movement.

   Check that P is valid; if not, trap.  Else, perform the move
   directly without converting it from the projection vector or to the
   freedom vector.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_direct_move_zp2 (interpreter: *mut sfnt_interpreter, p: uint32_t,
				dx: sfnt_f26dot6, dy: sfnt_f26dot6)
{                                                          // C 8646-8676
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  if (*interpreter).state.zp2 == 0
    {
      if p >= (*interpreter).twilight_zone_size as uint32_t
	{ TRAP! (c"point out of bounds".as_ptr ()); }

      *(*interpreter).twilight_x.offset (p as isize)
	= sfnt_add! (*(*interpreter).twilight_x.offset (p as isize), dx);
      *(*interpreter).twilight_y.offset (p as isize)
	= sfnt_add! (*(*interpreter).twilight_y.offset (p as isize), dy);
    }
  else
    {
      if (*interpreter).glyph_zone.is_null ()
	  || (p as usize) >= (*(*interpreter).glyph_zone).num_points
	{ TRAP! (c"point out of bounds".as_ptr ()); }

      *(*(*interpreter).glyph_zone).x_current.offset (p as isize)
	= sfnt_add! (*(*(*interpreter).glyph_zone).x_current.offset (p as isize), dx);
      *(*(*interpreter).glyph_zone).y_current.offset (p as isize)
	= sfnt_add! (*(*(*interpreter).glyph_zone).y_current.offset (p as isize), dy);

      if dx != 0
	{ *(*(*interpreter).glyph_zone).flags.offset (p as isize)
	    |= SFNT_POINT_TOUCHED_X as u8; }

      if dy != 0
	{ *(*(*interpreter).glyph_zone).flags.offset (p as isize)
	    |= SFNT_POINT_TOUCHED_Y as u8; }
    }
}

/* Project the vector VX, VY onto INTERPRETER's projection vector.
   Return the magnitude of the projection.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_project_vector (interpreter: *mut sfnt_interpreter,
			       vx: sfnt_f26dot6, vy: sfnt_f26dot6)
  -> sfnt_f26dot6                                        // C 8682-8686
{
  return (*interpreter).state.project.unwrap () (vx, vy, interpreter);
}

/* Project the vector VX, VY onto INTERPRETER's dual projection
   vector.  Return the magnitude of the projection.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_dual_project_vector (interpreter: *mut sfnt_interpreter,
				    vx: sfnt_f26dot6, vy: sfnt_f26dot6)
  -> sfnt_f26dot6                                        // C 8692-8696
{
  return (*interpreter).state.dual_project.unwrap () (vx, vy, interpreter);
}

/* Interpret a FLIPRGOFF instruction in INTERPRTER.  Make each point
   in ZP0 between L and H an off-curve point.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_fliprgoff (interpreter: *mut sfnt_interpreter,
				    h: uint32_t, l: uint32_t)
{                                                          // C 8702-8715
  let mut i: uint32_t;

  sfnt_check_zp0 (interpreter, l);
  sfnt_check_zp0 (interpreter, h);

  if (*interpreter).state.zp0 == 0
    { return; }

  i = l;
  while i <= h
    {
      *(*(*interpreter).glyph_zone).flags.offset (i as isize) &= !0o1;
      i += 1;
    }
}

/* Interpret a FLIPRGON instruction in INTERPRTER.  Make each point in
   ZP0 between L and H an on-curve point.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_fliprgon (interpreter: *mut sfnt_interpreter,
				   h: uint32_t, l: uint32_t)
{                                                          // C 8721-8734
  let mut i: uint32_t;

  sfnt_check_zp0 (interpreter, l);
  sfnt_check_zp0 (interpreter, h);

  if (*interpreter).state.zp0 == 0
    { return; }

  i = l;
  while i <= h
    {
      *(*(*interpreter).glyph_zone).flags.offset (i as isize) |= 0o1;
      i += 1;
    }
}

/* Interpret a FLIPPT instruction in INTERPRETER.  For loop times, pop
   a point in ZP0.  If it is an on-curve point, make it an off-curve
   one, and vice versa.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_flippt (interpreter: *mut sfnt_interpreter)
{                                                          // C 8741-8768
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }
  /* C 6597-6601's `#define POP()` - the live `#ifndef TEST` arm; the `#else` arm's body names
     the TEST-only `pop_hook` member (see this file's header).  */
  macro_rules! POP { () => {
    if (*interpreter).SP == (*interpreter).stack
      { TRAP! (c"stack underflow".as_ptr ()); 0 }
    else
      { (*interpreter).SP = (*interpreter).SP.offset (-1); *(*interpreter).SP }
  } }

  let mut point: uint32_t;

  while { (*interpreter).state.r#loop = (*interpreter).state.r#loop.wrapping_sub (1);
	  (*interpreter).state.r#loop != 0xffff }
    {
      point = POP! ();

      /* There are no flags in the twilight zone.
         But first check that the point is within bounds.  */

      sfnt_check_zp0 (interpreter, point);

      if (*interpreter).state.zp0 == 0
	{ continue; }

      /* If POINT is on the curve, make it off the curve and vice
	 versa.  */

      if (*(*(*interpreter).glyph_zone).flags.offset (point as isize) & 0o1) != 0
	{ *(*(*interpreter).glyph_zone).flags.offset (point as isize) &= !0o1; }
      else
	{ *(*(*interpreter).glyph_zone).flags.offset (point as isize) |= 0o1; }
    }

  /* Restore loop.  */
  (*interpreter).state.r#loop = 1;
}

/* Interpret an SCFS instruction.
   Move P in ZP2 along the freedom vector until its projection is
   equal to C.

   If ZP2 is the twilight zone, ``create'' P by setting its original
   position to the projection.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_scfs (interpreter: *mut sfnt_interpreter,
			       p: uint32_t, c: sfnt_f26dot6)
{                                                          // C 8778-8792
  /* C 7777-7778's `#define PROJECT(x, y)`: the P-57 in-function copy.  */
  macro_rules! PROJECT { ($x:expr, $y:expr) => {
    sfnt_project_vector (interpreter, $x, $y)
  } }

  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut distance: sfnt_f26dot6;

  sfnt_address_zp2 (interpreter, p, &raw mut x, &raw mut y,
		    std::ptr::null_mut (), std::ptr::null_mut ());
  distance = PROJECT! (x, y);
  sfnt_move_zp2 (interpreter, p, 1, sfnt_sub! (c, distance));

  if (*interpreter).state.zp2 == 0
    {
      *(*interpreter).twilight_original_x.offset (p as isize)
	= *(*interpreter).twilight_x.offset (p as isize);
      *(*interpreter).twilight_original_y.offset (p as isize)
	= *(*interpreter).twilight_y.offset (p as isize);
    }
}

/* Symmetrically round the 26.6 fixed point value X using the rounding
   mode in INTERPRETER.  Return the result.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_round_symmetric (interpreter: *mut sfnt_interpreter, mut x: sfnt_f26dot6)
  -> sfnt_f26dot6                                        // C 8798-8811
{
  let mut sign: c_int;

  sign = 1;

  if x < 0
    {
      sign = -1;
      x = -x;
    }

  return (*interpreter).state.round.unwrap () (x, interpreter) * sign;
}

/* Interpret an MIAP (``Move Indirect Absolute Point'') instruction
   using INTERPRETER.

   Move P in ZP0 along the freedom vector until its projection on the
   projection vector is equal to CVT units in the projection vector.

   Finally, set RP0 and RP1 to P.

   If ZP0 is the twilight zone, then first create that point in the
   twilight zone by setting its ``original position'' to the
   projection of the value.

   If OPCODE is 0x3f, then in addition check the CVT value against the
   control value cut-in, and round the magnitudes of the movement.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_miap (interpreter: *mut sfnt_interpreter,
			       cvt: uint32_t, p: uint32_t, opcode: c_uchar)
{                                                          // C 8829-8888
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut distance: sfnt_f26dot6;
  let mut value: sfnt_f26dot6;
  let mut delta: sfnt_f26dot6;

  /* Read the cvt value.  */

  if (cvt as usize) >= (*interpreter).cvt_size
    { TRAP! (c"out of bounds read to cvt".as_ptr ()); }

  value = *(*interpreter).cvt.offset (cvt as isize);

  /* Now load the point.  */
  sfnt_address_zp0 (interpreter, p, &raw mut x, &raw mut y,
		    std::ptr::null_mut (), std::ptr::null_mut ());

  /* Create the twilight zone point if necessary.
     Note that the value used is not rounded.  */

  if (*interpreter).state.zp0 == 0
    {
      *(*interpreter).twilight_original_x.offset (p as isize)
	= sfnt_mul_f2dot14 ((*interpreter).state.projection_vector.x,
			    value);
      *(*interpreter).twilight_x.offset (p as isize)
	= *(*interpreter).twilight_original_x.offset (p as isize);
      x = *(*interpreter).twilight_x.offset (p as isize);

      *(*interpreter).twilight_original_y.offset (p as isize)
	= sfnt_mul_f2dot14 ((*interpreter).state.projection_vector.y,
			    value);
      *(*interpreter).twilight_y.offset (p as isize)
	= *(*interpreter).twilight_original_y.offset (p as isize);
      y = *(*interpreter).twilight_y.offset (p as isize);
    }

  /* Obtain the original distance.  */
  distance = sfnt_project_vector (interpreter, x, y);

  /* Round the distance and apply the cvt cut in if necessary.  */

  if opcode == 0x3f
    {
      delta = sfnt_sub! (value, distance);

      if delta < 0
	{ delta = -delta; }

      /* If delta is more than the cvt cut in (more aptly named ``cut
	 out''), use the original distance.  */

      if delta > (*interpreter).state.cvt_cut_in
	{ value = distance; }

      /* Round value.  */
      value = sfnt_round_symmetric (interpreter, value);
    }

  /* Move the point by the distance.  */
  sfnt_move_zp0 (interpreter, p, 1, sfnt_sub! (value, distance));

  /* Set reference points.  */
  (*interpreter).state.rp0 = p as uint16_t;
  (*interpreter).state.rp1 = p as uint16_t;
}

/* Perform a single iteration of sfnt_interpret_alignrp.  RP0X and
   RP0Y should be the position of the reference point RP0 in ZP0.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_alignrp_1 (interpreter: *mut sfnt_interpreter,
				    rp0x: sfnt_f26dot6, rp0y: sfnt_f26dot6)
{                                                          // C 8894-8911
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }
  macro_rules! POP { () => {
    if (*interpreter).SP == (*interpreter).stack
      { TRAP! (c"stack underflow".as_ptr ()); 0 }
    else
      { (*interpreter).SP = (*interpreter).SP.offset (-1); *(*interpreter).SP }
  } }

  let mut distance: sfnt_f26dot6;
  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut point: uint32_t;

  point = POP! ();

  /* Load this point.  */
  sfnt_address_zp1 (interpreter, point, &raw mut x, &raw mut y,
		    std::ptr::null_mut (), std::ptr::null_mut ());

  /* Measure the distance from here to rp0.  */
  distance = sfnt_project_vector (interpreter, sfnt_sub! (x, rp0x),
				  sfnt_sub! (y, rp0y));

  /* Move by the opposite.  */
  sfnt_move_zp1 (interpreter, point, 1, -distance);
}

/* For loop times, pop a point in ZP1 and align it to RP0 in ZP0 by
   moving it along the freedom vector until its projected distance
   from RP0 becomes 0.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_alignrp (interpreter: *mut sfnt_interpreter)
{                                                          // C 8918-8936
  let mut rp0x: sfnt_f26dot6 = 0;
  let mut rp0y: sfnt_f26dot6 = 0;

  sfnt_address_zp0 (interpreter, (*interpreter).state.rp0 as uint32_t,
		    &raw mut rp0x, &raw mut rp0y,
		    std::ptr::null_mut (), std::ptr::null_mut ());

  while { (*interpreter).state.r#loop = (*interpreter).state.r#loop.wrapping_sub (1);
	  (*interpreter).state.r#loop != 0xffff }
    {
      sfnt_interpret_alignrp_1 (interpreter, rp0x, rp0y);

      /* Reload RP0 if it is in the same zone as ZP1.  */
      if (*interpreter).state.zp0 == (*interpreter).state.zp1
	{ sfnt_address_zp0 (interpreter, (*interpreter).state.rp0 as uint32_t,
			    &raw mut rp0x, &raw mut rp0y,
			    std::ptr::null_mut (), std::ptr::null_mut ()); }
    }

  (*interpreter).state.r#loop = 1;
}

/* Align the two points P1 and P2 relative to the projection vector.
   P1 is addressed relative to ZP0, and P2 is addressed relative to
   ZP1.

   Move both points along the freedom vector by half the magnitude of
   the projection of a vector formed by P1.x - P2.x, P1.y - P2.y,
   upon the projection vector.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_alignpts (interpreter: *mut sfnt_interpreter,
				   p1: uint32_t, p2: uint32_t)
{                                                          // C 8947-8964
  let mut p1x: sfnt_f26dot6 = 0;
  let mut p1y: sfnt_f26dot6 = 0;
  let mut p2x: sfnt_f26dot6 = 0;
  let mut p2y: sfnt_f26dot6 = 0;
  let mut magnitude: sfnt_f26dot6;

  sfnt_address_zp0 (interpreter, p1, &raw mut p1x, &raw mut p1y,
		    std::ptr::null_mut (), std::ptr::null_mut ());
  sfnt_address_zp1 (interpreter, p2, &raw mut p2x, &raw mut p2y,
		    std::ptr::null_mut (), std::ptr::null_mut ());

  magnitude = sfnt_project_vector (interpreter,
				   sfnt_sub! (p1x, p2x),
				   sfnt_sub! (p1y, p2y));
  magnitude = magnitude / 2;

  /* Now move both points along the freedom vector.  */
  sfnt_move_zp0 (interpreter, p1, 1, magnitude);
  sfnt_move_zp1 (interpreter, p2, 1, -magnitude);
}

/* Set the point P in the zone referenced in INTERPRETER's ZP2
   register to the intersection between the line formed by the points
   POINT_A0 to POINT_A1 in ZP0 and another line formed by POINT_B0 to
   POINT_B1 in ZP1.

   Touch the point P.  */

/* C 8983-8990's `#if 0` (the `determinant`/`a0`/`b0`/`a1`/`b1`/`c0`/`c1`/`px`/`py` declaration
   run) is dead text, so the port carries the `#else` arm's declarations alone.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_isect (interpreter: *mut sfnt_interpreter,
				point_a0: uint32_t, point_a1: uint32_t,
				point_b0: uint32_t, point_b1: uint32_t,
				p: uint32_t)
{                                                          // C 8974-9120
  let mut a0x: sfnt_f26dot6 = 0;
  let mut a0y: sfnt_f26dot6 = 0;
  let mut a1x: sfnt_f26dot6 = 0;
  let mut a1y: sfnt_f26dot6 = 0;
  let mut b0x: sfnt_f26dot6 = 0;
  let mut b0y: sfnt_f26dot6 = 0;
  let mut b1x: sfnt_f26dot6 = 0;
  let mut b1y: sfnt_f26dot6 = 0;
  let mut dx: sfnt_f26dot6;
  let mut dy: sfnt_f26dot6;
  let mut dax: sfnt_f26dot6;
  let mut day: sfnt_f26dot6;
  let mut dbx: sfnt_f26dot6;
  let mut dby: sfnt_f26dot6;
  let mut discriminant: sfnt_f26dot6;
  let mut val: sfnt_f26dot6;
  let mut dot_product: sfnt_f26dot6;
  let mut px: sfnt_f26dot6;
  let mut py: sfnt_f26dot6;

  /* Load points.  */
  sfnt_address_zp0 (interpreter, point_a0, &raw mut a0x, &raw mut a0y,
		    std::ptr::null_mut (), std::ptr::null_mut ());
  sfnt_address_zp0 (interpreter, point_a1, &raw mut a1x, &raw mut a1y,
		    std::ptr::null_mut (), std::ptr::null_mut ());
  sfnt_address_zp1 (interpreter, point_b0, &raw mut b0x, &raw mut b0y,
		    std::ptr::null_mut (), std::ptr::null_mut ());
  sfnt_address_zp1 (interpreter, point_b1, &raw mut b1x, &raw mut b1y,
		    std::ptr::null_mut (), std::ptr::null_mut ());

  /* C 8992-9034's `#if 0` arm (the Cramer's-rule block with
     `sfnt_line_to_standard_form`) is dead text: recorded, not carried.  */

  /* The algorithm above would work with floating point, but overflows
     too easily with fixed point numbers.

     Instead, use the modified vector projection algorithm found in
     FreeType.  */

  'degenerate_case: {
    dbx = sfnt_sub! (b1x, b0x);
    dby = sfnt_sub! (b1y, b0y);
    dax = sfnt_sub! (a1x, a0x);
    day = sfnt_sub! (a1y, a0y);

    /* Compute vector cross product.  */
    discriminant = sfnt_add! (sfnt_mul_f26dot6 (dax, -dby),
			      sfnt_mul_f26dot6 (day, dbx));
    dot_product = sfnt_add! (sfnt_mul_f26dot6 (dax, dbx),
			     sfnt_mul_f26dot6 (day, dby));

    /* Reject any non-intersections and grazing intersections.  */
    if !(sfnt_mul! (19, abs (discriminant)) > abs (dot_product))
      { return; }

    /* Reject any non-intersections.  */
    if discriminant == 0
      { break 'degenerate_case; }

    dx = sfnt_sub! (b0x, a0x);
    dy = sfnt_sub! (b0y, a0y);
    val = sfnt_add! (sfnt_mul_f26dot6 (dx, -dby),
		     sfnt_mul_f26dot6 (dy, dbx));

    /* Project according to these values.  */
    dx = sfnt_add! (a0x, sfnt_multiply_divide_signed (val, dax,
						      discriminant));
    dy = sfnt_add! (a0y, sfnt_multiply_divide_signed (val, day,
						      discriminant));

    sfnt_store_zp2 (interpreter, p,
		    /* C 9065-9071's `#if 0` (`sfnt_div_fixed (dx, determinant)`,
		       `sfnt_div_fixed (dy, determinant)`) is dead text.  */
		    dx, dy,
		    SFNT_POINT_TOUCHED_BOTH);
    return;
  }

  /* degenerate_case: */

  /* Apple says that in this case:

     Px = (a0x + a1x) / 2 + (b0x + b1x) / 2
          ---------------------------------
	                  2
     Py = (a0y + a1y) / 2 + (b0y + b1y) / 2
          ---------------------------------
	                  2  */

  px = (sfnt_add! (a0x, a1x) / 2 + sfnt_add! (b0x, b1x) / 2) / 2;
  py = (sfnt_add! (a0y, a1y) / 2 + sfnt_add! (b0y, b1y) / 2) / 2;
  sfnt_store_zp2 (interpreter, p, px, py,
		  SFNT_POINT_TOUCHED_BOTH);
}

/* Compute the square root of the 16.16 fixed point number N.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_sqrt_fixed (n: sfnt_fixed) -> sfnt_fixed
{                                                          // C 9125-9155
  let mut count: c_int;
  let mut root: c_uint;
  let mut rem_hi: c_uint;
  let mut rem_lo: c_uint;
  let mut possible: c_uint;

  root = 0;

  if n > 0
    {
      rem_hi = 0;
      rem_lo = n as c_uint;
      count = 24;

      loop
	{
	  rem_hi = (rem_hi << 2) | (rem_lo >> 30);
	  rem_lo <<= 2;
	  root <<= 1;
	  possible = (root << 1) + 1;

	  if rem_hi >= possible
	    {
	      rem_hi -= possible;
	      root += 1;
	    }

	  count -= 1;
	  if count == 0
	    { break; }
	}
    }

  return root as sfnt_fixed;
}

/* Compute a unit vector describing a vector VX, VY.  Return the value
   in *VECTOR.  */

/* C 9180-9183's `fail:` tail stands after this `'fail` block rather than inside C's first `if`:
   C 9208's `goto fail` jumps *into* that block, which a Rust labelled block cannot be entered
   from (PORTING.md 2.2 spells the forward jump as `break 'fail`); the success path keeps C's
   implicit fall-off-the-end as `return`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_normalize_vector (mut vx: sfnt_f26dot6, mut vy: sfnt_f26dot6,
				 vector: *mut sfnt_unit_vector)
{                                                          // C 9161-9213
  let mut x_squared: sfnt_f26dot6;
  let mut y_squared: sfnt_f26dot6;
  let mut n: sfnt_fixed;
  let mut magnitude: sfnt_fixed;

  'fail: {
    if vx == 0 && vy == 0
      { break 'fail; }

    /* Scale vx and vy up if they won't at least make 1.  */

    while !(vx < -32 || vx > 32) && !(vy < -32 || vy > 32)
      {
	vx = vx * 2;
	vy = vy * 2;
      }

    /* Compute the magnitude of this vector.  */
    x_squared = sfnt_mul_f26dot6 (vx, vx);
    y_squared = sfnt_mul_f26dot6 (vy, vy);

    /* x_squared and y_squared can end up too large to fit in a 16.16
       fixed.  Scale both values down until they fit.  */

    while x_squared > 0x200000 || y_squared > 0x200000
	  || x_squared < -0x200000 || y_squared < -0x200000
      {
	x_squared /= 2;
	y_squared /= 2;
      }

    /* Convert to 16.16 for greater precision.  */
    n = sfnt_add! (x_squared, y_squared) * 1024;

    /* Get hypotenuse of the triangle from vx, 0, to 0, vy.  */
    magnitude = sfnt_sqrt_fixed (n);

    /* Avoid division by zero.  */
    if magnitude == 0
      { break 'fail; }

    /* Long division.. eek! */
    (*vector).x = (sfnt_div_fixed (vx * 1024, magnitude) / 4) as sfnt_f2dot14;
    (*vector).y = (sfnt_div_fixed (vy * 1024, magnitude) / 4) as sfnt_f2dot14;
    return;
  }

  /* fail: */
  (*vector).x = 0o4000;
  (*vector).y = 0;
}

/* Compute a unit vector describing the direction of a line from the
   point P2 to the point P1.  Save the result in *VECTOR.

   P2 is the address of a point in the zone specified in the ZP2
   register.  P1 is the address of a point in the zone specified in
   the ZP1 register.  Take the values of both registers from the
   specified INTERPRETER's graphics state.

   If PERPENDICULAR, then *VECTOR will be rotated 90 degrees
   counter-clockwise.  Else, *VECTOR will be parallel to the line.

   If ORIGINAL, then the coordinates used to calculate the line will
   be those prior to instructing.  Otherwise, the current coordinates
   will be used.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_line_to_vector (interpreter: *mut sfnt_interpreter,
			       p2: uint32_t, p1: uint32_t,
			       vector: *mut sfnt_unit_vector,
			       perpendicular: bool, original: bool)
{                                                          // C 9231-9270
  let mut x2: sfnt_f26dot6 = 0;
  let mut y2: sfnt_f26dot6 = 0;
  let mut original_x2: sfnt_f26dot6 = 0;
  let mut original_y2: sfnt_f26dot6 = 0;
  let mut x1: sfnt_f26dot6 = 0;
  let mut y1: sfnt_f26dot6 = 0;
  let mut original_x1: sfnt_f26dot6 = 0;
  let mut original_y1: sfnt_f26dot6 = 0;
  let mut a: sfnt_f26dot6;
  let mut b: sfnt_f26dot6;
  let mut temp: sfnt_f26dot6;

  sfnt_address_zp2 (interpreter, p2, &raw mut x2, &raw mut y2, &raw mut original_x2,
		    &raw mut original_y2);
  sfnt_address_zp1 (interpreter, p1, &raw mut x1, &raw mut y1, &raw mut original_x1,
		    &raw mut original_y1);

  /* Use original coordinates if specified.  */

  if original
    {
      x2 = original_x2;
      y2 = original_y2;
      x1 = original_x1;
      y1 = original_y1;
    }

  /* Calculate the vector between X2, Y2, and X1, Y1.  */
  a = sfnt_sub! (x1, x2);
  b = sfnt_sub! (y1, y2);

  /* Rotate counterclockwise if necessary.  */

  if perpendicular
    {
      temp = b;
      b = a;
      a = -temp;
    }

  /* Normalize this vector, turning it into a unit vector.  */
  sfnt_normalize_vector (a, b, vector);
}

/* Measure the distance between P1 in ZP0 and P2 in ZP1,
   relative to the projection or dual projection vector.

   Return the distance of P1 and P2 relative to their original
   un-instructed positions should OPCODE be 0x4A, and to their
   instructed positions should OPCODE be 0x49.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_measure_distance (interpreter: *mut sfnt_interpreter,
				 p1: uint32_t, p2: uint32_t,
				 opcode: c_uchar)
  -> sfnt_f26dot6                                        // C 9280-9301
{
  /* C 7777/7780's `PROJECT`/`DUAL_PROJECT` - the P-57 in-function copies.  */
  macro_rules! PROJECT { ($x:expr, $y:expr) => {
    sfnt_project_vector (interpreter, $x, $y)
  } }
  macro_rules! DUAL_PROJECT { ($x:expr, $y:expr) => {
    sfnt_dual_project_vector (interpreter, $x, $y)
  } }

  let mut p1x: sfnt_f26dot6 = 0;
  let mut p1y: sfnt_f26dot6 = 0;
  let mut p1_original_x: sfnt_f26dot6 = 0;
  let mut p1_original_y: sfnt_f26dot6 = 0;
  let mut p2x: sfnt_f26dot6 = 0;
  let mut p2y: sfnt_f26dot6 = 0;
  let mut p2_original_x: sfnt_f26dot6 = 0;
  let mut p2_original_y: sfnt_f26dot6 = 0;

  /* P1 is relative to ZP0 and P2 is relative to ZP1.
     Apple's manual says this, Microsoft's does not.  */

  sfnt_address_zp0 (interpreter, p1, &raw mut p1x, &raw mut p1y,
		    &raw mut p1_original_x, &raw mut p1_original_y);
  sfnt_address_zp1 (interpreter, p2, &raw mut p2x, &raw mut p2y,
		    &raw mut p2_original_x, &raw mut p2_original_y);

  if opcode == 0x4A
    { return DUAL_PROJECT! (sfnt_sub! (p1_original_x, p2_original_x),
			    sfnt_sub! (p1_original_y, p2_original_y)); }

  return PROJECT! (sfnt_sub! (p1x, p2x),
		   sfnt_sub! (p1y, p2y));
}

/* Interpret an MSIRP instruction in INTERPRETER.
   Take a point P, and make the distance between P in ZP1 and the
   current position of RP0 in ZP0 equal to D.

   If ZP1 is the twilight zone, then create the point P by setting its
   position and relative positions.

   Then, if OPCODE is equal to 0x3b, make P RP0.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_msirp (interpreter: *mut sfnt_interpreter,
				d: sfnt_f26dot6, p: uint32_t, opcode: c_uchar)
{                                                          // C 9313-9361
  let mut rp0x: sfnt_f26dot6 = 0;
  let mut rp0y: sfnt_f26dot6 = 0;
  let mut rp0_original_x: sfnt_f26dot6 = 0;
  let mut rp0_original_y: sfnt_f26dot6 = 0;
  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut old_distance: sfnt_f26dot6;
  let mut temp: sfnt_f26dot6;

  sfnt_address_zp0 (interpreter, (*interpreter).state.rp0 as uint32_t,
		    &raw mut rp0x, &raw mut rp0y, &raw mut rp0_original_x,
		    &raw mut rp0_original_y);
  sfnt_address_zp1 (interpreter, p, &raw mut x, &raw mut y,
		    std::ptr::null_mut (), std::ptr::null_mut ());

  if (*interpreter).state.zp1 == 0
    {
      /* Create this point in the twilight zone at RP0.  */

      *(*interpreter).twilight_x.offset (p as isize) = rp0x;
      x = *(*interpreter).twilight_x.offset (p as isize);
      *(*interpreter).twilight_y.offset (p as isize) = rp0y;
      y = *(*interpreter).twilight_y.offset (p as isize);

      /* Now set the original positions to the projected difference
         from rp0.  This makes sense once you think about it.  */
      temp = sfnt_mul_f2dot14 ((*interpreter).state.projection_vector.x, d);
      temp = sfnt_add! (temp, rp0_original_x);
      *(*interpreter).twilight_original_x.offset (p as isize) = temp;

      temp = sfnt_mul_f2dot14 ((*interpreter).state.projection_vector.y, d);
      temp = sfnt_add! (temp, rp0_original_y);
      *(*interpreter).twilight_original_y.offset (p as isize) = temp;
    }

  /* Compute the original distance.  */
  old_distance = sfnt_project_vector (interpreter,
				      sfnt_sub! (x, rp0x),
				      sfnt_sub! (y, rp0y));

  /* Move the point.  */
  sfnt_move_zp1 (interpreter, p, 1, sfnt_sub! (d, old_distance));

  /* Nothing in the TrueType reference manual says directly that this
     instruction should change rp1 and rp2.  However, it says this
     instruction is ``very similar to the MIRP[] instruction
     except...'', and FreeType seems to do this, so do it as well.  */

  (*interpreter).state.rp1 = (*interpreter).state.rp0;
  (*interpreter).state.rp2 = p as uint16_t;

  if opcode == 0x3b
    { (*interpreter).state.rp0 = p as uint16_t; }
}

/* Interpret an IP instruction in INTERPRETER.  For loop times, pop a
   single point in ZP2, and interpolate it so that its original
   relationship to the points RP1 in ZP0 and RP2 in ZP1 as measured
   along the dual projection vector continues to hold true.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_ip (interpreter: *mut sfnt_interpreter)
{                                                          // C 9369-9527
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }
  macro_rules! POP { () => {
    if (*interpreter).SP == (*interpreter).stack
      { TRAP! (c"stack underflow".as_ptr ()); 0 }
    else
      { (*interpreter).SP = (*interpreter).SP.offset (-1); *(*interpreter).SP }
  } }

  let mut rp1x: sfnt_f26dot6 = 0;
  let mut rp1y: sfnt_f26dot6 = 0;
  let mut rp1_original_x: sfnt_f26dot6 = 0;
  let mut rp1_original_y: sfnt_f26dot6 = 0;
  let mut rp2x: sfnt_f26dot6 = 0;
  let mut rp2y: sfnt_f26dot6 = 0;
  let mut rp2_original_x: sfnt_f26dot6 = 0;
  let mut rp2_original_y: sfnt_f26dot6 = 0;
  let mut range: sfnt_f26dot6;
  let mut new_range: sfnt_f26dot6;
  let mut org_distance: sfnt_f26dot6;
  let mut cur_distance: sfnt_f26dot6;
  let mut new_distance: sfnt_f26dot6;
  let mut p: uint32_t;
  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut original_x: sfnt_f26dot6 = 0;
  let mut original_y: sfnt_f26dot6 = 0;
  let mut zone: *mut sfnt_interpreter_zone;
  let mut scale: bool;

  /* First load both reference points.  */
  sfnt_address_zp0 (interpreter, (*interpreter).state.rp1 as uint32_t,
		    &raw mut rp1x, &raw mut rp1y, &raw mut rp1_original_x,
		    &raw mut rp1_original_y);
  sfnt_address_zp1 (interpreter, (*interpreter).state.rp2 as uint32_t,
		    &raw mut rp2x, &raw mut rp2y, &raw mut rp2_original_x,
		    &raw mut rp2_original_y);

  /* If RP1, RP2, and all arguments all fall within the glyph zone and
     a simple glyph is loaded, replace their original coordinates as
     loaded here with coordinates from the unscaled glyph outline.  */

  zone = (*interpreter).glyph_zone;
  scale = false;

  if !zone.is_null () && !(*zone).simple.is_null ()
      && (*interpreter).state.zp0 != 0
      && (*interpreter).state.zp1 != 0
      && (*interpreter).state.zp2 != 0
    {
      p = (*interpreter).state.rp1 as uint32_t;

      /* If P is a phantom point... */
      if (p as usize) >= (*(*zone).simple).number_of_points
	{
	  /* ...scale the phantom point to the size of the original
	     outline.  */
	  rp1_original_x = sfnt_div_fixed (rp1_original_x,
					   (*interpreter).scale);
	  rp1_original_y = sfnt_div_fixed (rp1_original_y,
					   (*interpreter).scale);
	}
      else
	{
	  rp1_original_x = *(*(*zone).simple).x_coordinates.offset (p as isize)
			   as sfnt_f26dot6;
	  rp1_original_y = *(*(*zone).simple).y_coordinates.offset (p as isize)
			   as sfnt_f26dot6;
	}

      p = (*interpreter).state.rp2 as uint32_t;

      /* If P is a phantom point... */
      if (p as usize) >= (*(*zone).simple).number_of_points
	{
	  /* ...scale the phantom point to the size of the original
	     outline.  */
	  rp2_original_x = sfnt_div_fixed (rp2_original_x,
					   (*interpreter).scale);
	  rp2_original_y = sfnt_div_fixed (rp2_original_y,
					   (*interpreter).scale);
	}
      else
	{
	  rp2_original_x = *(*(*zone).simple).x_coordinates.offset (p as isize)
			   as sfnt_f26dot6;
	  rp2_original_y = *(*(*zone).simple).y_coordinates.offset (p as isize)
			   as sfnt_f26dot6;
	}

      scale = true;
    }

  /* Get the original distance between of RP1 and RP2 measured
     relative to the dual projection vector.  */
  range = sfnt_dual_project_vector (interpreter,
				    sfnt_sub! (rp2_original_x,
					       rp1_original_x),
				    sfnt_sub! (rp2_original_y,
					       rp1_original_y));

  if scale
    { range = sfnt_mul_fixed_round (range, (*interpreter).scale); }

  /* Get the new distance.  */
  new_range = sfnt_dual_project_vector (interpreter,
					sfnt_sub! (rp2x, rp1x),
					sfnt_sub! (rp2y, rp1y));

  while { (*interpreter).state.r#loop = (*interpreter).state.r#loop.wrapping_sub (1);
	  (*interpreter).state.r#loop != 0xffff }
    {
      p = POP! ();

      /* Load this point relative to zp2.  */
      sfnt_address_zp2 (interpreter, p, &raw mut x, &raw mut y, &raw mut original_x,
			&raw mut original_y);

      if scale
	{
	  /* If P is a phantom point... */
	  if (p as usize) >= (*(*zone).simple).number_of_points
	    {
	      /* ...scale the phantom point to the size of the original
		 outline.  */
	      original_x = sfnt_div_fixed (original_x,
					   (*interpreter).scale);
	      original_y = sfnt_div_fixed (original_y,
					   (*interpreter).scale);
	    }
	  else
	    {
	      original_x = *(*(*zone).simple).x_coordinates.offset (p as isize)
			   as sfnt_f26dot6;
	      original_y = *(*(*zone).simple).y_coordinates.offset (p as isize)
			   as sfnt_f26dot6;
	    }
	}

      /* Now compute the old distance from this point to rp1.  */
      org_distance
	= sfnt_dual_project_vector (interpreter,
				    sfnt_sub! (original_x,
					       rp1_original_x),
				    sfnt_sub! (original_y,
					       rp1_original_y));

      if scale
	{ org_distance = sfnt_mul_fixed_round (org_distance,
					       (*interpreter).scale); }

      /* And the current distance from this point to rp1, so
         how much to move can be determined.  */
      cur_distance
	= sfnt_project_vector (interpreter,
			       sfnt_sub! (x, rp1x),
			       sfnt_sub! (y, rp1y));

      /* Finally, apply the ratio of the new distance between RP1 and
	 RP2 to that of the old distance between the two reference
	 points to org_distance, making new_distance.

         If both reference points occupy the same position on the dual
         projection vector, then simply use the old distance.  */

      if org_distance != 0
	{
	  if range != 0
	    { new_distance
		= sfnt_multiply_divide_signed (org_distance,
					       new_range, range); }
	  else
	    { new_distance = org_distance; }
	}
      else
	{ new_distance = 0; }

      /* And move the point along the freedom vector to reflect the
	 change in distance.  */
      sfnt_move_zp2 (interpreter, p, 1,
		     sfnt_sub! (new_distance, cur_distance));
    }

  (*interpreter).state.r#loop = 1;
}

/* Apply the delta specified by OPERAND to the control value table
   entry at INDEX currently loaded inside INTERPRETER.

   Trap if INDEX is out of bounds.

   NUMBER is the number of the specific DELTAC instruction this delta
   is being applied on behalf of.  It must be between 1 and 3.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_deltac (number: c_int, interpreter: *mut sfnt_interpreter,
		       index: c_uint, operand: c_uchar)
{                                                          // C 9538-9656
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut ppem: c_int;
  let mut delta: c_int;

  /* Make sure INDEX is a valid cvt entry.  */

  if (index as usize) >= (*interpreter).cvt_size
    { TRAP! (c"DELTACn instruction out of bounds".as_ptr ()); }

  /* operand is an 8 bit number.  The most significant 4 bits
     represent a specific PPEM size at which to apply the delta
     specified in the low 4 bits, summed with an instruction specific
     delta, and the current delta base.  */

  ppem = (operand >> 4) as c_int + (*interpreter).state.delta_base as c_int;

  match number
    {
      1 => {}
      2 => { ppem += 16; }
      3 => { ppem += 32; }
      _ => {}
    }

  /* Don't apply the delta if the ppem size doesn't match.  */

  if (*interpreter).ppem != ppem
    { return; }

  /* Now, determine the delta using the low 4 bits.  The low 4 bits
     actually specify a ``magnitude'' to apply to the delta, and do
     not have an encoding for the delta 0.  */

  match operand & 0xf
    {
      0 => { delta = -8; }
      1 => { delta = -7; }
      2 => { delta = -6; }
      3 => { delta = -5; }
      4 => { delta = -4; }
      5 => { delta = -3; }
      6 => { delta = -2; }
      7 => { delta = -1; }
      8 => { delta = 1; }
      9 => { delta = 2; }
      10 => { delta = 3; }
      11 => { delta = 4; }
      12 => { delta = 5; }
      13 => { delta = 6; }
      14 => { delta = 7; }
      15 => { delta = 8; }

      /* To pacify -fanalyzer.  */
      _ => { abort (); }
    }

  /* Now, scale up the delta by the step size, which is determined by
     the delta shift.  */
  delta = (delta as c_long
	   * ((1 as c_long) << (6 - (*interpreter).state.delta_shift as c_int)))
	  as c_int;

  /* Finally, apply the delta to the CVT entry.  */
  *(*interpreter).cvt.offset (index as isize)
    = sfnt_add! (*(*interpreter).cvt.offset (index as isize),
		 delta);
}

/* Interpret an MDAP (Move Direct Absolute Point) instruction with the
   opcode OPCODE and the operand P in INTERPRETER.

   Touch the point P (within the zone specified in zp0) in the
   directions specified in the freedom vector.  Then, if OPCODE is
   0x2f, round the point and move it the rounded distance along the
   freedom vector.

   Finally, set the RP0 and RP1 registers to P.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_mdap (interpreter: *mut sfnt_interpreter,
			       p: uint32_t, opcode: uint32_t)
{                                                          // C 9669-9694
  let mut here: sfnt_f26dot6;
  let mut distance: sfnt_f26dot6;
  let mut px: sfnt_f26dot6 = 0;
  let mut py: sfnt_f26dot6 = 0;

  sfnt_address_zp0 (interpreter, p, &raw mut px, &raw mut py,
		    std::ptr::null_mut (), std::ptr::null_mut ());

  /* Measure the current distance.  */
  here = sfnt_project_vector (interpreter, px, py);

  if opcode == 0x2f
    {
      /* Measure distance, round, then move to the distance.  */
      distance = sfnt_project_vector (interpreter, px, py);
      distance = sfnt_round_symmetric (interpreter, distance);
      distance = sfnt_sub! (distance, here);
    }
  else
    /* Don't move.  Just touch the point.  */
    { distance = 0; }

  sfnt_move_zp0 (interpreter, p, 1, distance);

  (*interpreter).state.rp0 = p as uint16_t;
  (*interpreter).state.rp1 = p as uint16_t;
}

/* Apply the delta specified by OPERAND to the point P in ZP0
   currently loaded inside INTERPRETER.

   Trap if P is out of bounds.

   NUMBER is the number of the specific DELTAP instruction this delta
   is being applied on behalf of.  It must be between 1 and 3.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_deltap (number: c_int, interpreter: *mut sfnt_interpreter,
		       operand: c_uchar, p: c_uint)
{                                                          // C 9705-9815
  let mut ppem: c_int;
  let mut delta: c_int;

  /* Extract the ppem from OPERAND.  The format is the same as in
     sfnt_deltac.  */

  ppem = (operand >> 4) as c_int + (*interpreter).state.delta_base as c_int;

  match number
    {
      1 => {}
      2 => { ppem += 16; }
      3 => { ppem += 32; }
      _ => {}
    }

  /* Don't apply the delta if the ppem size doesn't match.  */

  if (*interpreter).ppem != ppem
    { return; }

  /* Now, determine the magnitude of the movement and find the
     delta.  */

  match operand & 0xf
    {
      0 => { delta = -8; }
      1 => { delta = -7; }
      2 => { delta = -6; }
      3 => { delta = -5; }
      4 => { delta = -4; }
      5 => { delta = -3; }
      6 => { delta = -2; }
      7 => { delta = -1; }
      8 => { delta = 1; }
      9 => { delta = 2; }
      10 => { delta = 3; }
      11 => { delta = 4; }
      12 => { delta = 5; }
      13 => { delta = 6; }
      14 => { delta = 7; }
      15 => { delta = 8; }

      /* To pacify -fanalyzer.  */
      _ => { abort (); }
    }

  /* Now, scale up the delta by the step size, which is determined by
     the delta shift.  */
  delta = (delta as c_long
	   * ((1 as c_long) << (6 - (*interpreter).state.delta_shift as c_int)))
	  as c_int;

  /* Move the point.  */
  sfnt_check_zp0 (interpreter, p);
  sfnt_move_zp0 (interpreter, p, 1, delta);
}

/* C 9817-9819's `/* Needed by sfnt_interpret_call.  */` and its
   `static void sfnt_interpret_run (struct sfnt_interpreter *, enum sfnt_interpreter_run_context);`
   - the forward declaration 0.1-12 deletes, and the definition is C 11396, still unported: P-91 ①
   and ④ name this call site and its C line in the ledger.  */

/* Call DEFINITION inside INTERPRETER.

   Save INTERPRETER->IP, INTERPRETER->instructions, and
   INTERPRETER->num_instructions onto the C stack.

   Then, load the instructions in DEFINITION, and run the interpreter
   again with the context CONTEXT.

   Finally, restore all values.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_call (definition: *mut sfnt_interpreter_definition,
			       interpreter: *mut sfnt_interpreter,
			       context: sfnt_interpreter_run_context)
{                                                          // C 9832-9860
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut num_instructions: uint16_t;
  let mut IP: c_int;
  let mut instructions: *mut c_uchar;

  /* Check that no recursion is going on.  C's `call_depth++` compares the value before the
     increment (the `bidi.rs` `n_old` shape).  */
  if { let call_depth_old: c_int = (*interpreter).call_depth;
       (*interpreter).call_depth += 1;
       call_depth_old } >= 128
    { TRAP! (c"CALL called CALL more than 127 times".as_ptr ()); }

  /* Save the old IP, instructions and number of instructions.  */
  num_instructions = (*interpreter).num_instructions;
  IP = (*interpreter).IP;
  instructions = (*interpreter).instructions;

  /* Load and run the definition.  */
  (*interpreter).num_instructions = (*definition).instruction_count;
  (*interpreter).instructions = (*definition).instructions;
  (*interpreter).IP = 0;
  sfnt_interpret_run (interpreter, context);

  /* Restore the old values.  */
  (*interpreter).num_instructions = num_instructions;
  (*interpreter).IP = IP;
  (*interpreter).instructions = instructions;
  (*interpreter).call_depth -= 1;
}

/* Set the detailed rounding state in interpreter, on behalf of either
   an SROUND or S45ROUND instruction that has been given the operand
   OPERAND.

   Use the specified GRID_PERIOD to determine the period.  It is is a
   18.14 fixed point number, but the rounding state set will be a 26.6
   fixed point number.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_set_srounding_state (interpreter: *mut sfnt_interpreter,
				    operand: uint32_t, grid_period: sfnt_f18dot14)
{                                                          // C 9871-9932
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut period: sfnt_f18dot14;
  let mut phase: sfnt_f18dot14;
  let mut threshold: sfnt_f18dot14;

  /* The most significant 2 bits in the 8 bit OPERAND determine the
     period.  */

  match (operand & 0xc0) >> 6
    {
      0 => { period = grid_period / 2; }
      1 => { period = grid_period; }
      2 => { period = grid_period * 2; }
      _ => { TRAP! (c"reserved period given to SROUND".as_ptr ()); }
    }

  /* The next two bits determine the phase.  */

  match (operand & 0x30) >> 4
    {
      0 => { phase = 0; }
      1 => { phase = period / 4; }
      2 => { phase = period / 2; }
      _ => { phase = period * 3 / 2; }
    }

  /* And the least significant 4 bits determine the threshold.  */

  if (operand & 0x0f) != 0
    { threshold = (((operand & 0x0f) as c_int - 4)
		   * period / 8); }
  else
    { threshold = period - 1; }

  /* Now extend these values to 26.6 format and set them.  */
  (*interpreter).period = period >> 8;
  (*interpreter).phase = phase >> 8;
  (*interpreter).threshold = threshold >> 8;
}

/* Move to the next opcode in INTERPRETER's instruction stream.
   Value is the opcode originally at INTERPRETER->IP.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_skip_code (interpreter: *mut sfnt_interpreter) -> c_uchar
{                                                          // C 9938-9989
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut opcode: c_uchar;
  let mut nbytes: c_int;

  if (*interpreter).IP == (*interpreter).num_instructions as c_int
    { TRAP! (c"IP at end of instruction stream".as_ptr ()); }

  /* Load opcode at IP.  */
  opcode = *(*interpreter).instructions.offset ((*interpreter).IP as isize);

  if opcode == 0x40 || opcode == 0x41
    {
      if (*interpreter).IP + 1 >= (*interpreter).num_instructions as c_int
	{ TRAP! (c"Missing arg to NPUSHB or NPUSHW".as_ptr ()); }

      /* Figure out how many bytes or words to push.  */

      nbytes = *(*interpreter).instructions.offset (((*interpreter).IP + 1) as isize)
	       as c_int;

      if opcode == 0x41
	{ nbytes *= 2; }

      if (*interpreter).IP + 2 + nbytes > (*interpreter).num_instructions as c_int
	{ TRAP! (c"args to NPUSH instruction lie outside IS".as_ptr ()); }

      /* Increment IP by so much.  */
      (*interpreter).IP += 2 + nbytes;
    }
  else if opcode >= 0xb0 && opcode <= 0xb7
    {
      nbytes = opcode as c_int - 0xb0 + 1;

      if (*interpreter).IP + 1 + nbytes > (*interpreter).num_instructions as c_int
	{ TRAP! (c"args to PUSHB instruction lie outide IS".as_ptr ()); }

      (*interpreter).IP += 1 + nbytes;
    }
  else if opcode >= 0xb8 && opcode <= 0xbf
    {
      nbytes = (opcode as c_int - 0xb8 + 1) * 2;

      if (*interpreter).IP + 1 + nbytes > (*interpreter).num_instructions as c_int
	{ TRAP! (c"args to PUSHW instruction lie outide IS".as_ptr ()); }

      (*interpreter).IP += 1 + nbytes;
    }
  else
    { (*interpreter).IP += 1; }

  return opcode;
}

/* Interpret the unimplemented operation OPCODE using INTERPRETER, and
   the context WHY.  If there is no instruction definition named
   OPCODE, trap.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_unimplemented (interpreter: *mut sfnt_interpreter,
					opcode: c_uchar,
					why: sfnt_interpreter_run_context)
{                                                          // C 9996-10019
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut i: uint32_t;
  let mut def: *mut sfnt_interpreter_definition;

  i = 0;
  while i < (*interpreter).instruction_defs_size as uint32_t
    {
      def = (*interpreter).instruction_defs.offset (i as isize);

      if (*def).opcode as uint32_t == opcode as uint32_t
	{
	  if (*def).instructions.is_null ()
	    { TRAP! (c"** ERROR ** malformed internal instruction definition".as_ptr ()); }

	  sfnt_interpret_call (def, interpreter, why);
	  return;
	}

      i += 1;
    }

  TRAP! (c"invalid instruction".as_ptr ());
}

/* Start a function definition in INTERPRETER, with the function
   opcode OPCODE.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_fdef (interpreter: *mut sfnt_interpreter,
			       opcode: uint32_t)
{                                                          // C 10025-10074
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut i: size_t;
  let mut num_fdefs: size_t;
  let mut IP: c_int;
  let mut instruction: c_uchar;

  IP = (*interpreter).IP + 1;
  num_fdefs = 0;

  /* Now find an ENDF.  */

  while { instruction = sfnt_skip_code (interpreter); instruction != 0x2d }
    {
      if (*interpreter).IP >= (*interpreter).num_instructions as c_int
	{ TRAP! (c"missing ENDF".as_ptr ()); }

      /* If this is an FDEF or IDEF instruction, increment num_fdefs.
	 Prohibit nested FDEFs or IDEFS.  */
      if instruction == 0x2c || instruction == 0x89
	{ num_fdefs += 1; }

      if num_fdefs > 1
	{ TRAP! (c"IDEF or FDEF before ENDF".as_ptr ()); }
    }

  /* ENDF has been found.  Now save the function definition.  Try to
     find an existing function definition with this opcode.  If that
     fails, make i the first available function definition.  */

  i = 0;
  while i < (*interpreter).function_defs_size as size_t
    {
      if (*(*interpreter).function_defs.offset (i as isize)).opcode as uint32_t == opcode
	  || (*(*interpreter).function_defs.offset (i as isize)).instructions.is_null ()
	{ break; }

      i += 1;
    }

  if i == (*interpreter).function_defs_size as size_t
    { TRAP! (c"number of fdefs exceeded maxp->max_function_defs".as_ptr ()); }

  /* Save the opcode of this function definition.  */
  (*(*interpreter).function_defs.offset (i as isize)).opcode = opcode as uint16_t;

  /* Make sure to ignore the trailing ENDF instruction.  */
  (*(*interpreter).function_defs.offset (i as isize)).instruction_count
    = ((*interpreter).IP - IP - 1) as uint16_t;

  /* Now save a pointer to the instructions.  */
  (*(*interpreter).function_defs.offset (i as isize)).instructions
    = (*interpreter).instructions.offset (IP as isize);
}

/* Start an instruction definition in INTERPRETER, with the
   instruction opcode OPCODE.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_idef (interpreter: *mut sfnt_interpreter,
			       opcode: uint32_t)
{                                                          // C 10080-10130
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut i: size_t;
  let mut num_fdefs: size_t;
  let mut IP: c_int;
  let mut instruction: c_uchar;

  IP = (*interpreter).IP + 1;
  num_fdefs = 0;

  /* Now find an ENDF.  */

  while { instruction = sfnt_skip_code (interpreter); instruction != 0x2d }
    {
      if (*interpreter).IP >= (*interpreter).num_instructions as c_int
	{ TRAP! (c"missing ENDF".as_ptr ()); }

      /* If this is an FDEF or IDEF instruction, increment num_fdefs.
	 Prohibit nested FDEFs or IDEFS.  */
      if instruction == 0x2c || instruction == 0x89
	{ num_fdefs += 1; }

      if num_fdefs > 1
	{ TRAP! (c"IDEF or FDEF before ENDF".as_ptr ()); }
    }

  /* ENDF has been found.  Now save the instruction definition.  Try to
     find an existing instruction definition with this opcode.  If that
     fails, make i the first available instruction definition.  */

  i = 0;
  while i < (*interpreter).instruction_defs_size as size_t
    {
      if (*(*interpreter).instruction_defs.offset (i as isize)).opcode as uint32_t == opcode
	  || (*(*interpreter).instruction_defs.offset (i as isize)).instructions.is_null ()
	{ break; }

      i += 1;
    }

  if i == (*interpreter).instruction_defs_size as size_t
    { TRAP! (c"number of defs exceeded maxp->max_instruction_defs".as_ptr ()); }

  /* Save the opcode of this instruction definition.  */
  (*(*interpreter).instruction_defs.offset (i as isize)).opcode = opcode as uint16_t;

  /* Make sure to ignore the trailing ENDF instruction.  */
  (*(*interpreter).instruction_defs.offset (i as isize)).instruction_count
    = ((*interpreter).IP - IP - 1) as uint16_t;

  /* Now save a pointer to the instructions.  */
  (*(*interpreter).instruction_defs.offset (i as isize)).instructions
    = (*interpreter).instructions.offset (IP as isize);
}

/* Interpret the specified conditional at INTERPRETER->IP.
   If CONDITION, evaluate this branch up until the next ELSE or ENDIF.
   Else, evaluate the branch from a matching ELSE condition, if
   one exists.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_if (interpreter: *mut sfnt_interpreter,
			     condition: bool)
{                                                          // C 10138-10185
  let mut nifs: c_int;
  let mut need_break: bool;
  let mut opcode: c_uchar;

  if condition
    {
      (*interpreter).IP += 1;
      return;
    }

  /* Number of ifs.  */
  nifs = 0;
  need_break = false;

  /* Break past the matching else condition.  */
  loop
    {
      /* Load the current opcode, then increase IP.  */
      opcode = sfnt_skip_code (interpreter);

      if (*interpreter).IP >= (*interpreter).num_instructions as c_int
	{ break; }

      match opcode
	{
	  0x58 =>                                          /* IF */
	    { nifs += 1; }

	  0x1B =>                                          /* ELSE */
	    {
	      if nifs == 1
		{ need_break = true; }
	    }

	  0x59 =>                                          /* EIF */
	    {
	      nifs -= 1;
	      if nifs == 0
		{ need_break = true; }
	    }

	  _ => {}
	}

      /* C's `while (!need_break)` test, inverted into a break.  */
      if need_break
	{ break; }
    }
}

/* Interpret the specified ELSE branch at INTERPRETER->IP.
   Evaluate starting from a matching ENDIF instruction.

   If IF has set INTERPRETER->IP to a code within an ELSE branch, this
   will not be called.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_else (interpreter: *mut sfnt_interpreter)
{                                                          // C 10194-10224
  let mut nifs: c_int;
  let mut opcode: c_uchar;

  /* Number of ifs.  */
  nifs = 1;

  /* Break past the matching ENDIF condition.  */
  loop
    {
      /* Load the current opcode, then increase IP.  */
      opcode = sfnt_skip_code (interpreter);

      if (*interpreter).IP >= (*interpreter).num_instructions as c_int
	{ break; }

      match opcode
	{
	  0x58 =>                                          /* IF */
	    { nifs += 1; }

	  0x59 =>                                          /* EIF */
	    {
	      nifs -= 1;
	    }

	  _ => {}
	}

      if !(nifs > 0)
	{ break; }
    }
}
/* C 10225-10663: the graphics-state validation family.  It is ported **here**, ahead of the
   C 6307-10224 block, because of P-91 ①: `sfnt_init_graphics_state` (C 6316) calls
   `sfnt_validate_gs`, whose definition is C 10538, and Rust has no prototype for a `static`
   function (PORTING.md 0.1-12).  The text stands in C's own position (P-91 ②) - after slice 6's
   C 6306 text and before the C 13918 group below.  Every callee is already in this unit
   (`sfnt_round_f26dot6` C 6298, `sfnt_floor_f26dot6` C 6281, `sfnt_ceil_f26dot6` C 6289, all
   slice 6, and `sfnt_multiply_divide_signed` C 3696-3714, slice 5), so the group is closed and
   needs none of P-91 ④'s allowances.  */

/* ``Add engine compensation to X''.  Since engine compensation is not
   implemented here, this simply returns X.  INTERPRETER is
   unused.  */

/* C's `static sfnt_f26dot6 sfnt_round_none (…)` is one of the eight `round` callbacks - the
   type of `sfnt_graphics_state.round` (mirror C 1797) - so it is `unsafe extern "C"` here
   (PORTING.md 1.4), and the seven below it are the same.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_round_none (x: sfnt_f26dot6, interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10230-10234
{
  return x;
}

/* Round X to the grid after adding engine compensation.  Return the
   result.  INTERPRETER is unused.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_round_to_grid (x: sfnt_f26dot6, interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10239-10243
{
  return sfnt_round_f26dot6 (x);
}

/* Round X to the nearest half integer or integer and return the
   result.  INTERPRETER is unused.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_round_to_double_grid (x: sfnt_f26dot6,
						interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10248-10253
{
  return (x + 0o20) & !0o37;
}

/* Take the floor of X and return the result.  INTERPRETER is
   unused.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_round_down_to_grid (x: sfnt_f26dot6,
					      interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10258-10263
{
  return sfnt_floor_f26dot6 (x);
}

/* Take the ceiling of X and return the result.  INTERPRETER is
   unused.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_round_up_to_grid (x: sfnt_f26dot6,
					    interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10268-10273
{
  return sfnt_ceil_f26dot6 (x);
}

/* Round X to only the nearest half integer and return the result.
   INTERPRETER is unused.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_round_to_half_grid (x: sfnt_f26dot6,
					      interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10278-10283
{
  return sfnt_floor_f26dot6 (x) + 32;
}

/* Round X using the detailed rounding information ``super rounding
   state'' in INTERPRETER.  Value is the result.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_round_super (x: sfnt_f26dot6, interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10288-10305
{
  let mut value: sfnt_f26dot6;

  /* Compute the rounded value.  */
  value = sfnt_add! ((*interpreter).threshold
		     - (*interpreter).phase, x);
  value = sfnt_add! (value & -(*interpreter).period,
		     (*interpreter).phase);

  /* Remember that since the phase is specified by font instructions,
     it is possible for the sign to be changed.  In that case, return
     the phase itself.  */

  return if value < 0 { (*interpreter).phase } else { value };
}

/* Round X using the detailed rounding information ``super rounding
   state'' in INTERPRETER, but suitably for values that are multiples
   of the sqrt of 2.  Value is the result.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_round_super45 (x: sfnt_f26dot6, interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10311-10330
{
  let mut value: sfnt_f26dot6;

  /* Compute the rounded value.  */

  value = ((sfnt_add! (x, ((*interpreter).threshold
			   - (*interpreter).phase))
	    / (*interpreter).period)
	   * (*interpreter).period);
  value = sfnt_add! (value, (*interpreter).phase);

  /* Remember that since the phase is specified by font instructions,
     it is possible for the sign to be changed.  In that case, return
     the phase itself.  */

  return if value < 0 { (*interpreter).phase } else { value };
}

/* Project the specified vector VX and VY onto the unit vector that is
   INTERPRETER's projection vector, assuming that INTERPRETER's
   projection vector is on the X axis.

   Value is the magnitude of the projected vector.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_project_onto_x_axis_vector (vx: sfnt_f26dot6, vy: sfnt_f26dot6,
						      interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10338-10343
{
  return vx;
}

/* Project the specified vector VX and VY onto the unit vector that is
   INTERPRETER's projection vector, assuming that INTERPRETER's
   projection vector is on the Y axis.

   Value is the magnitude of the projected vector.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_project_onto_y_axis_vector (vx: sfnt_f26dot6, vy: sfnt_f26dot6,
						      interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10351-10356
{
  return vy;
}

/* Calculate AX * BX + AY * BY divided by 16384.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_dot_fix_14 (ax: int32_t, ay: int32_t, bx: c_int, by: c_int) -> int32_t
{                                                          // C 10360-10376
  let mut xx: int64_t = ax as int64_t * bx as int64_t;
  let mut yy: int64_t = ay as int64_t * by as int64_t;
  xx += yy;
  yy = xx >> 63;
  xx += 0x2000 + yy;

  /* TrueType fonts rely on "division" here truncating towards
     negative infinity, so compute the arithmetic right shift in place
     of division.  */
  let mut temp: int64_t = -((xx < 0) as int64_t);
  temp = (temp ^ xx) >> 14 ^ temp;
  return temp as int32_t;
}

/* Project the specified vector VX and VY onto the unit vector that is
   INTERPRETER's projection vector, making only the assumption that the
   projection vector is a valid unit vector.

   Value is the magnitude of the projected vector.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_project_onto_any_vector (vx: sfnt_f26dot6, vy: sfnt_f26dot6,
						   interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10384-10391
{
  return sfnt_dot_fix_14 (vx, vy,
			  (*interpreter).state.projection_vector.x as c_int,
			  (*interpreter).state.projection_vector.y as c_int);
}

/* Project the specified vector VX and VY onto the unit vector that is
   INTERPRETER's dual projection vector, making only the assumption
   that the dual projection vector is a valid unit vector.

   The dual projection vector is a vector that is normally the
   projection vector, but can be set using the original unscaled
   coordinates of two points as well.

   Value is the magnitude of the projected vector.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_dual_project_onto_any_vector (vx: sfnt_f26dot6, vy: sfnt_f26dot6,
							interpreter: *mut sfnt_interpreter)
  -> sfnt_f26dot6                                        // C 10403-10410
{
  return sfnt_dot_fix_14 (vx, vy,
			  (*interpreter).state.dual_projection_vector.x as c_int,
			  (*interpreter).state.dual_projection_vector.y as c_int);
}

/* Move N points at *X, *Y by DISTANCE along INTERPRETER's freedom
   vector.  Set N flags in *FLAGS where appropriate and when non-NULL.

   Assume both vectors are aligned to the X axis.  */

/* C's two `restrict`-qualified `sfnt_f26dot6 *` parameters (`x` and `y`) drop the qualifier -
   Rust has no spelling for it - and `flags` is `*mut u8`, C's `unsigned char *`, which is the
   type `sfnt_graphics_state.move` carries (mirror C 1814-1817).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_move_x (mut x: *mut sfnt_f26dot6, y: *mut sfnt_f26dot6,
				  mut n: size_t, interpreter: *mut sfnt_interpreter,
				  distance: sfnt_f26dot6, mut flags: *mut u8)
{                                                          // C 10417-10430
  while n != 0
    {
      *x = sfnt_add! (*x, distance);
      x = x.offset (1);
      n -= 1;

      if !flags.is_null ()
	{ *flags |= SFNT_POINT_TOUCHED_X as c_uchar; flags = flags.offset (1); }
    }
}

/* Move N points at *X, *Y by DISTANCE along INTERPRETER's freedom
   vector.  Set N flags in *FLAGS where appropriate and when non-NULL.

   Assume both vectors are aligned to the Y axis.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_move_y (x: *mut sfnt_f26dot6, mut y: *mut sfnt_f26dot6,
				  mut n: size_t, interpreter: *mut sfnt_interpreter,
				  distance: sfnt_f26dot6, mut flags: *mut u8)
{                                                          // C 10437-10450
  while n != 0
    {
      *y = sfnt_add! (*y, distance);
      y = y.offset (1);
      n -= 1;

      if !flags.is_null ()
	{ *flags |= SFNT_POINT_TOUCHED_Y as c_uchar; flags = flags.offset (1); }
    }
}

/* Move N points at *X, *Y by DISTANCE along INTERPRETER's freedom
   vector.  Set N flags in *FLAGS where appropriate and when
   non-NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_move (mut x: *mut sfnt_f26dot6, mut y: *mut sfnt_f26dot6,
				n: size_t, interpreter: *mut sfnt_interpreter,
				distance: sfnt_f26dot6, mut flags: *mut u8)
{                                                          // C 10456-10521
  let mut versor: sfnt_f26dot6;
  let mut k: sfnt_f26dot6;
  let mut dot_product: sfnt_f2dot14;
  let mut num: size_t;
  let mut flags_start: *mut u8;

  dot_product = (*interpreter).state.vector_dot_product;

  /* If the vectors are orthogonal, it is impossible to move anywhere,
     so simply return.  */
  if dot_product == 0
    { return; }

  /* Not actually 26.6, but the multiply-divisions below cancel each
     other out, so the result is 26.6.  */
  versor = (*interpreter).state.freedom_vector.x as sfnt_f26dot6;

  /* Save flags that it may be restored for the second Y axis
     loop.  */
  flags_start = flags;

  if versor != 0
    {
      /* Move along X axis, converting the distance to the freedom
	 vector.  */
      num = n;
      k = sfnt_multiply_divide_signed (distance, versor,
				       dot_product as c_int) as sfnt_f26dot6;

      while num != 0
	{
	  *x = sfnt_add! (*x, k);
	  x = x.offset (1);
	  num -= 1;

	  if !flags.is_null ()
	    { *flags |= SFNT_POINT_TOUCHED_X as c_uchar; flags = flags.offset (1); }
	}
    }

  flags = flags_start;
  versor = (*interpreter).state.freedom_vector.y as sfnt_f26dot6;

  if versor != 0
    {
      /* Move along Y axis, converting the distance to the freedom
	 vector.  */
      num = n;
      k = sfnt_multiply_divide_signed (distance, versor,
				       dot_product as c_int) as sfnt_f26dot6;

      while num != 0
	{
	  *y = sfnt_add! (*y, k);
	  y = y.offset (1);
	  num -= 1;

	  if !flags.is_null ()
	    { *flags |= SFNT_POINT_TOUCHED_Y as c_uchar; flags = flags.offset (1); }
	}
    }
}

/* Compute the dot product of the two versors A and B with
   rounding.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_short_frac_dot (a: sfnt_f2dot14, b: sfnt_f2dot14) -> sfnt_f2dot14
{                                                          // C 10526-10530
  return (((a as c_long * b as c_long) + 8192) / 16384) as sfnt_f2dot14;
}

/* Validate the graphics state GS.
   Establish function pointers for rounding and projection.
   Establish dot product used to convert vector distances between
   each other.  */

/* C 10543's `switch (gs->round_state)` is C's arm order kept as the port's `match`; C has **no
   `default:`**, so C's fall-through-to-nothing tail is the port's `_ => {}` (PORTING.md 2.2, the
   slice-1 `sfnt_read_cmap_table_1` precedent).  The `switch`/`case` row that owes is Owed 14.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_validate_gs (gs: *mut sfnt_graphics_state)
{                                                          // C 10537-10633
  /* Establish the function used for rounding based on the round
     state.  */

  match (*gs).round_state
    {
      5 =>                                               /* Rounding off.  */
	{ (*gs).round = Some (sfnt_round_none); }
      0 =>                                               /* Round to half grid.  */
	{ (*gs).round = Some (sfnt_round_to_half_grid); }
      1 =>                                               /* Round to grid.  */
	{ (*gs).round = Some (sfnt_round_to_grid); }
      2 =>                                               /* Round to double grid.  */
	{ (*gs).round = Some (sfnt_round_to_double_grid); }
      4 =>                                               /* Round up to grid.  */
	{ (*gs).round = Some (sfnt_round_up_to_grid); }
      3 =>                                               /* Round down to grid.  */
	{ (*gs).round = Some (sfnt_round_down_to_grid); }
      6 =>                                               /* Fine grained rounding.  */
	{ (*gs).round = Some (sfnt_round_super); }
      7 =>                                               /* Fine grained rounding 45 degree variant.  */
	{ (*gs).round = Some (sfnt_round_super45); }
      _ => {}
    }

  /* Establish the function used for vector projection.
     When the projection vector is an axis vector, a fast
     version can be used.  */

  if (*gs).projection_vector.x == 0o40000
    { (*gs).project = Some (sfnt_project_onto_x_axis_vector); }
  else if (*gs).projection_vector.y == 0o40000
    { (*gs).project = Some (sfnt_project_onto_y_axis_vector); }
  else
    { (*gs).project = Some (sfnt_project_onto_any_vector); }

  /* Do the same for the dual projection vector.  */

  if (*gs).dual_projection_vector.x == 0o40000
    { (*gs).dual_project = Some (sfnt_project_onto_x_axis_vector); }
  else if (*gs).dual_projection_vector.y == 0o40000
    { (*gs).dual_project = Some (sfnt_project_onto_y_axis_vector); }
  else
    { (*gs).dual_project = Some (sfnt_dual_project_onto_any_vector); }

  /* Compute dot product of the freedom and projection vectors.
     Handle the common case where the freedom vector is aligned
     to an axis.  */

  if (*gs).freedom_vector.x == 0o40000
    { (*gs).vector_dot_product = (*gs).projection_vector.x; }
  else if (*gs).freedom_vector.y == 0o40000
    { (*gs).vector_dot_product = (*gs).projection_vector.y; }
  else
    /* Actually calculate the dot product.  */
    {
      (*gs).vector_dot_product
	= (sfnt_short_frac_dot ((*gs).projection_vector.x,
				(*gs).freedom_vector.x) as c_int
	   + sfnt_short_frac_dot ((*gs).projection_vector.y,
				  (*gs).freedom_vector.y) as c_int) as sfnt_f2dot14;
    }

  /* If the product is less than 1/16th of a vector, prevent overflow
     by resetting it to 1.  */

  if (*gs).vector_dot_product > -0x400
      && (*gs).vector_dot_product < 0x400
    {
      (*gs).vector_dot_product = (if (*gs).vector_dot_product < 0
				  { -0x4000 } else { 0x4000 }) as sfnt_f2dot14;
    }

  /* Now figure out which function to use to move distances.  Handle
     the common case where both the freedom and projection vectors are
     aligned to an axis.  */

  if (*gs).freedom_vector.x == 0o40000
      && (*gs).projection_vector.x == 0o40000
    { (*gs).r#move = Some (sfnt_move_x); }
  else if (*gs).freedom_vector.y == 0o40000
	   && (*gs).projection_vector.y == 0o40000
    { (*gs).r#move = Some (sfnt_move_y); }
  else
    { (*gs).r#move = Some (sfnt_move); }
}

/* Set the X and Y versors of the freedom vector of INTERPRETER's
   graphics state to the specified X and Y, in 2.14 fixed point
   format.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_set_freedom_vector (interpreter: *mut sfnt_interpreter,
				   x: sfnt_f2dot14, y: sfnt_f2dot14)
{                                                          // C 10639-10647
  (*interpreter).state.freedom_vector.x = x;
  (*interpreter).state.freedom_vector.y = y;

  sfnt_validate_gs (&raw mut (*interpreter).state);
}

/* Set the X and Y versors of the projection vector of INTERPRETER's
   graphics state to the specified X and Y, in 2.14 fixed point
   format.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_set_projection_vector (interpreter: *mut sfnt_interpreter,
				      x: sfnt_f2dot14, y: sfnt_f2dot14)
{                                                          // C 10653-10663
  (*interpreter).state.projection_vector.x = x;
  (*interpreter).state.projection_vector.y = y;
  (*interpreter).state.dual_projection_vector.x = x;
  (*interpreter).state.dual_projection_vector.y = y;

  sfnt_validate_gs (&raw mut (*interpreter).state);
}

/* C 13918-14554: the glyph-variation readers, ported at their own C position (P-91 ②).  C 13919's
   form feed is dropped the way C 343-345's and C 1467-1469's were (the banner text is kept).
   Every name these four readers call is already in this unit - `sfnt_find_table`, `xmalloc`,
   `xrealloc`, `xfree`, `xaddrealloc`, `read`, `lseek`, `memcpy`,
   `sfnt_swap16!`/`sfnt_swap32!`, `SFNT_ENDOF!` and P-05's `sa_stack_alloc` for C 14096's `alloca`
   - so, like the C 10225 group above, this range needs none of P-91 ④'s allowances.  */

/* Glyph variations.  Instead of defining separate fonts for each
   combination of weight, width and slant (bold, condensed, italic,
   etc), some fonts specify a list of ``variation axes'', which are
   options that accept values consisting of numbers on scales
   governing deltas applied to select points in their glyphs.

   Particular styles within the font are then supplied as sets of
   values on these scales to which their respective axes are set,
   termed ``instances''.

   This optional information is specified in the `fvar' (font
   variation), `gvar' (glyph variation) and `cvar' (CVT variation)
   tables in a font file.  */

/* Read an fvar table from the given font FD.  Use the table directory
   specified in SUBTABLE.

   Return the fvar table upon success, else NULL.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_fvar_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_fvar_table                                // C 13940-14155
{
  let mut directory: *mut sfnt_table_directory;
  let mut fvar: *mut sfnt_fvar_table;
  let mut rc: ssize_t;
  let mut min_bytes: size_t;
  let mut ps_size: size_t;
  let mut non_ps_size: size_t;
  let mut temp: size_t;
  let mut pad: size_t;
  let mut offset: off_t;
  let mut i: c_int;
  let mut j: c_int;
  let mut buffer: *mut c_char;
  let mut coords: *mut sfnt_fixed;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_FVAR);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  min_bytes = SFNT_ENDOF! (sfnt_fvar_table,
			   instance_size, uint16_t);

  /* Check that the length is at least min_bytes.  */
  if ((*directory).length as size_t) < min_bytes
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Allocate enough to hold the fvar table header.  */
  fvar = xmalloc (std::mem::size_of::<sfnt_fvar_table> ()) as *mut sfnt_fvar_table;

  /* Read the fvar table header.  */
  buffer = std::ptr::null_mut ();

  'bail: {
    rc = read (fd, fvar as *mut c_void, min_bytes);
    if rc == -1 || rc != min_bytes as ssize_t
      { break 'bail; }

    /* Swap what was read.  */
    sfnt_swap16! (&raw mut (*fvar).major_version);
    sfnt_swap16! (&raw mut (*fvar).minor_version);
    sfnt_swap16! (&raw mut (*fvar).offset_to_data);
    sfnt_swap16! (&raw mut (*fvar).count_size_pairs);
    sfnt_swap16! (&raw mut (*fvar).axis_count);
    sfnt_swap16! (&raw mut (*fvar).axis_size);
    sfnt_swap16! (&raw mut (*fvar).instance_count);
    sfnt_swap16! (&raw mut (*fvar).instance_size);

    /* major_version should be 1, and minor_version 0.  */

    if (*fvar).major_version != 1 || (*fvar).minor_version != 0
      { break 'bail; }

    /* count_size_pairs should be more than 2.  */

    if (*fvar).count_size_pairs < 2
      { break 'bail; }

    /* Don't try to read tables where the axis format differs.  */

    if (*fvar).axis_size != 20
      { break 'bail; }

    /* The instance size must either be 2 * sizeof (uint16_t) +
       axisCount * sizeof (sfnt_fixed), meaning there is no PostScript
       name identifier, or 3 * sizeof (uint16_t) + axisCount * sizeof
       (sfnt_fixed), meaning there is.  */

    let (ckd_temp, mul_overflow) = ((*fvar).axis_count as size_t)
      .overflowing_mul (std::mem::size_of::<sfnt_fixed> ());
    temp = ckd_temp;
    if mul_overflow
      { break 'bail; }
    let (ckd_non_ps_size, add_overflow) = temp
      .overflowing_add (2 * std::mem::size_of::<uint16_t> ());
    non_ps_size = ckd_non_ps_size;
    if add_overflow
      { break 'bail; }

    let (ckd_temp, mul_overflow) = ((*fvar).axis_count as size_t)
      .overflowing_mul (std::mem::size_of::<sfnt_fixed> ());
    temp = ckd_temp;
    if mul_overflow
      { break 'bail; }
    let (ckd_ps_size, add_overflow) = temp
      .overflowing_add (3 * std::mem::size_of::<uint16_t> ());
    ps_size = ckd_ps_size;
    if add_overflow
      { break 'bail; }

    if (*fvar).instance_size as size_t != non_ps_size
	&& (*fvar).instance_size as size_t != ps_size
      { break 'bail; }

    /* Now compute the offset of the axis data from the start of the
       font file.  */

    let (ckd_offset, add_overflow) = ((*fvar).offset_to_data as off_t)
      .overflowing_add ((*directory).offset as off_t);
    offset = ckd_offset;
    if add_overflow
      { break 'bail; }

    /* Seek there.  */

    if lseek (fd, offset, SEEK_SET) != offset
      { break 'bail; }

    min_bytes = std::mem::size_of::<sfnt_fvar_table> ();

    /* Now, read each axis and instance.  Calculate how much extra data
       needs to be allocated for the axes and instances: this is
       fvar->axis_count * sizeof (struct sfnt_variation_axis), some
       padding, and finally fvar->instance_count * sizeof (struct
       sfnt_instance) + sizeof (sfnt_fixed) * fvar->instance_count *
       fvar->axis_count.  */

    let (ckd_temp, mul_overflow) = ((*fvar).axis_count as size_t)
      .overflowing_mul (std::mem::size_of::<sfnt_variation_axis> ());
    temp = ckd_temp;
    if mul_overflow
      { break 'bail; }
    let (ckd_min_bytes, add_overflow) = min_bytes.overflowing_add (temp);
    min_bytes = ckd_min_bytes;
    if add_overflow
      { break 'bail; }

    pad = std::mem::align_of::<sfnt_instance> ();
    pad -= min_bytes & (pad - 1);

    let (ckd_min_bytes, add_overflow) = min_bytes.overflowing_add (pad);
    min_bytes = ckd_min_bytes;
    if add_overflow
      { break 'bail; }

    let (ckd_temp, mul_overflow) = ((*fvar).instance_count as size_t)
      .overflowing_mul (std::mem::size_of::<sfnt_instance> ());
    temp = ckd_temp;
    if mul_overflow
      { break 'bail; }
    let (ckd_min_bytes, add_overflow) = min_bytes.overflowing_add (temp);
    min_bytes = ckd_min_bytes;
    if add_overflow
      { break 'bail; }

    let (ckd_temp, mul_overflow) = ((*fvar).instance_count as size_t)
      .overflowing_mul (std::mem::size_of::<sfnt_fixed> ());
    temp = ckd_temp;
    if mul_overflow
      { break 'bail; }
    let (ckd_temp, mul_overflow) = temp.overflowing_mul ((*fvar).axis_count as size_t);
    temp = ckd_temp;
    if mul_overflow
      { break 'bail; }
    let (ckd_min_bytes, add_overflow) = min_bytes.overflowing_add (temp);
    min_bytes = ckd_min_bytes;
    if add_overflow
      { break 'bail; }

    /* Reallocate fvar.  */
    fvar = xrealloc (fvar as *mut c_void, min_bytes) as *mut sfnt_fvar_table;

    /* Fill in offsets.  */
    (*fvar).axis = fvar.offset (1) as *mut sfnt_variation_axis;
    (*fvar).instance
      = ((*fvar).axis.offset ((*fvar).axis_count as isize) as *mut c_char)
	.offset (pad as isize) as *mut sfnt_instance;

    /* Read axes.  */

    if ((*directory).length as size_t
	- SFNT_ENDOF! (sfnt_fvar_table, instance_size, uint16_t)
	< std::mem::size_of::<sfnt_variation_axis> () * ((*fvar).axis_count as size_t))
      { break 'bail; }

    rc = read (fd, (*fvar).axis as *mut c_void,
	       std::mem::size_of::<sfnt_variation_axis> () * ((*fvar).axis_count as size_t));
    if rc == -1
	|| rc != (std::mem::size_of::<sfnt_variation_axis> ()
		  * ((*fvar).axis_count as size_t)) as ssize_t
      { break 'bail; }

    /* Swap each axis.  */

    i = 0;
    while i < (*fvar).axis_count as c_int
      {
	sfnt_swap32! (&raw mut (*(*fvar).axis.offset (i as isize)).axis_tag);
	sfnt_swap32! (&raw mut (*(*fvar).axis.offset (i as isize)).min_value);
	sfnt_swap32! (&raw mut (*(*fvar).axis.offset (i as isize)).default_value);
	sfnt_swap32! (&raw mut (*(*fvar).axis.offset (i as isize)).max_value);
	sfnt_swap16! (&raw mut (*(*fvar).axis.offset (i as isize)).flags);
	sfnt_swap16! (&raw mut (*(*fvar).axis.offset (i as isize)).name_id);
	i += 1;
      }

    /* Read each instance.  */

    if (*fvar).instance_size < 1024 * 16
      { buffer = sa_stack_alloc ((*fvar).instance_size as size_t) as *mut c_char; }
    else
      { buffer = xmalloc ((*fvar).instance_size as size_t) as *mut c_char; }

    coords = (*fvar).instance.offset ((*fvar).instance_count as isize) as *mut sfnt_fixed;

    i = 0;
    while i < (*fvar).instance_count as c_int
      {
	rc = read (fd, buffer as *mut c_void, (*fvar).instance_size as size_t);
	if rc != (*fvar).instance_size as ssize_t
	  { break 'bail; }

	/* Fill in various fields.  */

	(*(*fvar).instance.offset (i as isize)).name_id
	  = *(buffer as *mut uint16_t);
	(*(*fvar).instance.offset (i as isize)).flags
	  = *(buffer as *mut uint16_t).offset (1);
	(*(*fvar).instance.offset (i as isize)).ps_name_id = 0;

	sfnt_swap16! (&raw mut (*(*fvar).instance.offset (i as isize)).name_id);
	sfnt_swap16! (&raw mut (*(*fvar).instance.offset (i as isize)).flags);

	/* Read coordinates.  */

	(*(*fvar).instance.offset (i as isize)).coords = coords;
	coords = coords.offset ((*fvar).axis_count as isize);

	memcpy ((*(*fvar).instance.offset (i as isize)).coords as *mut c_void,
		buffer.offset (4) as *const c_void,
		std::mem::size_of::<sfnt_fixed> () * ((*fvar).axis_count as size_t));

	/* Swap coordinates.  */

	j = 0;
	while j < (*fvar).axis_count as c_int
	  {
	    sfnt_swap32! (&raw mut *(*(*fvar).instance.offset (i as isize)).coords
			  .offset (j as isize));
	    j += 1;
	  }

	/* Read the PostScript name ID if necessary.  If not, set it to
	   nil.  */

	if (*fvar).instance_size as size_t == ps_size
	  {
	    (*(*fvar).instance.offset (i as isize)).ps_name_id
	      = *(buffer.offset ((4 + (std::mem::size_of::<sfnt_fixed> ()
				      * ((*fvar).axis_count as size_t))) as isize)
		  as *mut uint16_t);
	    sfnt_swap16! (&raw mut (*(*fvar).instance.offset (i as isize)).ps_name_id);
	  }
	i += 1;
      }

    /* Free the temporary buffer.  */
    if !buffer.is_null () && (*fvar).instance_size >= 1024 * 16
      { xfree (buffer as *mut c_void); }

    /* Return the fvar table.  */
    return fvar;
  } /* 'bail (C 14149) */

  /* bail: */
  if !buffer.is_null () && (*fvar).instance_size >= 1024 * 16
    { xfree (buffer as *mut c_void); }

  xfree (fvar as *mut c_void);
  return std::ptr::null_mut ();
}

/* Read a gvar table from the given font FD.  Use the table directory
   specified in SUBTABLE.

   Return the gvar table upon success, else NULL.  */

/* C 14157's form feed is dropped (the C 13919 reading above).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_gvar_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_gvar_table                                // C 14164-14318
{
  let mut directory: *mut sfnt_table_directory;
  let mut gvar: *mut sfnt_gvar_table;
  let mut rc: ssize_t;
  let mut min_bytes: size_t;
  let mut off_size: size_t;
  let mut coordinate_size: size_t;
  let mut data_size: size_t;
  let mut i: c_int;
  let mut offset: off_t;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_GVAR);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  min_bytes = SFNT_ENDOF! (sfnt_gvar_table,
			   offset_to_data, uint32_t);

  /* Check that the length is at least min_bytes.  */
  if ((*directory).length as size_t) < min_bytes
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Allocate enough to hold the gvar table header.  */
  gvar = xmalloc (std::mem::size_of::<sfnt_gvar_table> ()) as *mut sfnt_gvar_table;

  /* Read the gvar table header.  */
  'bail: {
    rc = read (fd, gvar as *mut c_void, min_bytes);
    if rc == -1 || rc != min_bytes as ssize_t
      { break 'bail; }

    /* Swap what was read.  */
    sfnt_swap16! (&raw mut (*gvar).version);
    sfnt_swap16! (&raw mut (*gvar).reserved);
    sfnt_swap16! (&raw mut (*gvar).axis_count);
    sfnt_swap16! (&raw mut (*gvar).shared_coord_count);
    sfnt_swap32! (&raw mut (*gvar).offset_to_coord);
    sfnt_swap16! (&raw mut (*gvar).glyph_count);
    sfnt_swap16! (&raw mut (*gvar).flags);
    sfnt_swap32! (&raw mut (*gvar).offset_to_data);

    if (*gvar).version != 1
      { break 'bail; }

    if (*gvar).offset_to_data > (*directory).length
      { break 'bail; }

    /* Figure out the size required for the offset array.  Note that
       there is one extra offset at the end of the array to mark the
       size of the last glyph.  */

    if ((*gvar).flags & 1) != 0
      /* Offsets are long words.  */
      { off_size = std::mem::size_of::<uint32_t> () * ((*gvar).glyph_count as size_t + 1); }
    else
      /* Offsets are words.  */
      { off_size = std::mem::size_of::<uint16_t> () * ((*gvar).glyph_count as size_t + 1); }

    /* Now figure out the size of the shared coordinates.  */
    coordinate_size = ((*gvar).shared_coord_count as size_t
		       * ((*gvar).axis_count as size_t)
		       * std::mem::size_of::<uint16_t> ());

    /* And the size of the glyph variation data.  */
    data_size = (*directory).length as size_t - ((*gvar).offset_to_data as size_t);

    /* Wraparound.  */
    if data_size > (*directory).length as size_t
      { break 'bail; }

    /* Figure out how big gvar needs to be.  */
    let (ckd_min_bytes, add_overflow) = coordinate_size.overflowing_add (off_size);
    min_bytes = ckd_min_bytes;
    if add_overflow
      { break 'bail; }
    let (ckd_min_bytes, add_overflow) = min_bytes.overflowing_add (data_size);
    min_bytes = ckd_min_bytes;
    if add_overflow
      { break 'bail; }

    /* Now allocate enough for all of this extra data.  */
    gvar = xaddrealloc (gvar as *mut c_void, std::mem::size_of::<sfnt_gvar_table> (),
			min_bytes) as *mut sfnt_gvar_table;

    /* Start reading offsets.  */

    if ((*gvar).flags & 1) == 0
      {
	(*gvar).u.offset_word = gvar.offset (1) as *mut uint16_t;
	rc = read (fd, (*gvar).u.offset_word as *mut c_void, off_size);
	if rc != off_size as ssize_t
	  { break 'bail; }

	i = 0;
	while i <= (*gvar).glyph_count as c_int
	  {
	    sfnt_swap16! (&raw mut *(*gvar).u.offset_word.offset (i as isize));
	    i += 1;
	  }
      }
    else
      {
	(*gvar).u.offset_long = gvar.offset (1) as *mut uint32_t;
	rc = read (fd, (*gvar).u.offset_long as *mut c_void, off_size);
	if rc == -1 || rc != off_size as ssize_t
	  { break 'bail; }

	i = 0;
	while i <= (*gvar).glyph_count as c_int
	  {
	    sfnt_swap32! (&raw mut *(*gvar).u.offset_long.offset (i as isize));
	    i += 1;
	  }
      }

    /* Start reading shared coordinates.  */

    (*gvar).global_coords
      = (gvar.offset (1) as *mut c_char).offset (off_size as isize) as *mut sfnt_f2dot14;

    if (*gvar).shared_coord_count != 0
      {
	let (ckd_offset, add_overflow) = ((*gvar).offset_to_coord as off_t)
	  .overflowing_add ((*directory).offset as off_t);
	offset = ckd_offset;
	if add_overflow
	  { break 'bail; }

	if lseek (fd, offset, SEEK_SET) != offset
	  { break 'bail; }

	rc = read (fd, (*gvar).global_coords as *mut c_void, coordinate_size);

	if rc == -1 || rc != coordinate_size as ssize_t
	  { break 'bail; }

	i = 0;
	while i < (coordinate_size / std::mem::size_of::<sfnt_f2dot14> ()) as c_int
	  {
	    sfnt_swap16! (&raw mut *(*gvar).global_coords.offset (i as isize));
	    i += 1;
	  }
      }

    /* Finally, read the rest of the glyph variation data.  */
    (*gvar).data_size = data_size;
    (*gvar).glyph_variation_data
      = (*gvar).global_coords.offset ((coordinate_size
				       / std::mem::size_of::<sfnt_f2dot14> ()) as isize) as *mut u8;

    if (*gvar).data_size != 0
      {
	let (ckd_offset, add_overflow) = ((*gvar).offset_to_data as off_t)
	  .overflowing_add ((*directory).offset as off_t);
	offset = ckd_offset;
	if add_overflow
	  { break 'bail; }

	if lseek (fd, offset, SEEK_SET) != offset
	  { break 'bail; }

	rc = read (fd, (*gvar).glyph_variation_data as *mut c_void, (*gvar).data_size);

	if rc == -1 || rc != (*gvar).data_size as ssize_t
	  { break 'bail; }
      }

    /* Return the read gvar table.  */
    return gvar;
  } /* 'bail (C 14315) */

  /* bail: */
  xfree (gvar as *mut c_void);
  return std::ptr::null_mut ();
}

/* Read an avar table from the given font FD.  Use the table directory
   specified in SUBTABLE.

   Return the avar table upon success, else NULL.  */

/* C 14320's form feed is dropped.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_avar_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_avar_table                                // C 14327-14443
{
  let mut directory: *mut sfnt_table_directory;
  let mut avar: *mut sfnt_avar_table;
  let mut rc: ssize_t;
  let mut min_size: size_t;
  let mut size: size_t;
  let mut i: size_t;
  let mut k: size_t;
  let mut j: size_t;
  let mut buffer: *mut uint16_t;
  let mut correspondences: *mut sfnt_short_frac_correspondence;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_AVAR);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  min_size = SFNT_ENDOF! (sfnt_avar_table, axis_count, uint32_t);

  /* Check that the length is at least min_size.  */
  if ((*directory).length as size_t) < min_size
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Allocate enough to hold the avar table header.  */
  avar = xmalloc (std::mem::size_of::<sfnt_avar_table> ()) as *mut sfnt_avar_table;

  /* Read the avar table header.  */
  'bail: {
    rc = read (fd, avar as *mut c_void, min_size);
    if rc == -1 || rc != min_size as ssize_t
      { break 'bail; }

    /* Swap what was read.  */
    sfnt_swap32! (&raw mut (*avar).version);
    sfnt_swap32! (&raw mut (*avar).axis_count);

    if (*avar).version != 0x00010000
      { break 'bail; }

    if (*avar).axis_count < 0
      { break 'bail; }

    /* Allocate a buffer that holds the rest of the data.  */
    size = (*directory).length as size_t - min_size;
    buffer = xmalloc (size) as *mut uint16_t;

    'bail1: {
      rc = read (fd, buffer as *mut c_void, size);
      if rc == -1 || rc != size as ssize_t
	{ break 'bail1; }

      /* Swap each word.  */
      i = 0;
      while i < size / std::mem::size_of::<uint16_t> ()
	{
	  sfnt_swap16! (&raw mut *buffer.offset (i as isize));
	  i += 1;
	}

      /* Now, determine how big the resulting data needs to be.  Each
	 struct has a pointer field, and that should be its alignment.  */

      k = 0;
      min_size = std::mem::size_of::<sfnt_avar_table> ();
      i = 0;
      while i < (*avar).axis_count as size_t
	{
	  /* Check that k remains within bounds.  */
	  if k >= size / std::mem::size_of::<uint16_t> ()
	    { break 'bail1; }

	  /* Now add one struct sfnt_short_frac_segment for each axis and
	     each of its correspondences.  */
	  let (ckd_min_size, add_overflow) = min_size
	    .overflowing_add (std::mem::size_of::<sfnt_short_frac_segment> ());
	  min_size = ckd_min_size;
	  if add_overflow
	    { break 'bail1; }
	  let (ckd_min_size, add_overflow) = min_size
	    .overflowing_add (std::mem::size_of::<sfnt_short_frac_correspondence> ()
			      * (*buffer.offset (k as isize) as size_t));
	  min_size = ckd_min_size;
	  if add_overflow
	    { break 'bail1; }

	  /* Verify that words from here to buffer[1 + buffer[k] * 2], the
	     next pairCount field, are within bounds.  */
	  j = k + 1 + (*buffer.offset (k as isize) as size_t) * 2;
	  if j > size / std::mem::size_of::<uint16_t> ()
	    { break 'bail1; }

	  /* Move to the next pairCount field.  */
	  k = j;

	  i += 1;
	}

      /* Resize avar to min_size and start filling in various
	 pointers.  */
      avar = xrealloc (avar as *mut c_void, min_size) as *mut sfnt_avar_table;
      (*avar).segments = avar.offset (1) as *mut sfnt_short_frac_segment;
      correspondences
	= (*avar).segments.offset ((*avar).axis_count as isize)
	as *mut sfnt_short_frac_correspondence;

      k = 0;
      i = 0;
      while i < (*avar).axis_count as size_t
	{
	  (*(*avar).segments.offset (i as isize)).pair_count
	    = *buffer.offset (k as isize);
	  k += 1;
	  (*(*avar).segments.offset (i as isize)).correspondence = correspondences;

	  j = 0;
	  while j < (*(*avar).segments.offset (i as isize)).pair_count as size_t
	    {
	      (*correspondences).from_coord = *buffer.offset (k as isize) as sfnt_f2dot14;
	      k += 1;
	      (*correspondences).to_coord = *buffer.offset (k as isize) as sfnt_f2dot14;
	      k += 1;
	      correspondences = correspondences.offset (1);

	      j += 1;
	    }

	  i += 1;
	}

      /* Return the read avar table.  Free buffer.  */
      xfree (buffer as *mut c_void);
      return avar;
    } /* 'bail1 (C 14438) */

    /* bail1: */
    xfree (buffer as *mut c_void);

    /* C's `bail1:` tail falls through into `bail:`; the port's labelled blocks put `bail:`'s
       tail after this block, so the fall-through is this `break 'bail`.  */
    break 'bail;
  } /* 'bail (C 14440) */

  /* bail: */
  xfree (avar as *mut c_void);
  return std::ptr::null_mut ();
}

/* Read a sequence of packed points starting from DATA.  Return the
   number of points read in *NPOINTS_RETURN and the array of unpacked
   points, or NULL upon failure.

   If non-NULL, set LOCATION to DATA plus the number of bytes read
   upon success.

   Return (uint16_t *) -1 if there are no points at all.
   In this case, deltas will apply to all points in the glyph,
   and *NPOINTS_RETURN will be UINT16_MAX.

   END is one byte past the last byte in DATA.  */

/* C 14445's form feed is dropped.  C's three `unsigned char *restrict` parameters drop the
   `restrict` (no Rust spelling); `(uint16_t *) -1` is the port's `usize::MAX as *mut uint16_t`
   (the `token-aliases.tsv` row the ledger owes).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_packed_points (mut data: *mut c_uchar, npoints_return: *mut uint16_t,
				   end: *mut c_uchar, location: *mut *mut c_uchar)
  -> *mut uint16_t                                       // C 14460-14554
{
  let mut npoints: c_int;
  let mut points: *mut uint16_t;
  let mut i: c_int;
  let mut first: c_int;
  let mut control: c_int;

  points = std::ptr::null_mut ();
  npoints = 0;

  if data >= end
    { return std::ptr::null_mut (); }

  /* Load the control byte.  */
  control = *data as c_int;
  data = data.offset (1);

  if control == 0
    {
      *npoints_return = UINT16_MAX as uint16_t;
      *location = data;
      return usize::MAX as *mut uint16_t;
    }

  /* Now figure out the number of points within.  */

  if (control & 0x80) != 0
    {
      npoints = control & 0x7f;
      npoints <<= 8;

      if data >= end
	{ return std::ptr::null_mut (); }

      npoints |= *data as c_int;
      data = data.offset (1);
    }
  else
    { npoints = control; }

  /* Start reading points.  */
  first = 0;
  i = 0;
  points = xmalloc (std::mem::size_of::<uint16_t> () * npoints as size_t) as *mut uint16_t;

  'bail: {
    while i < npoints
      {
	if data >= end
	  { break 'bail; }

	control = *data as c_int;
	data = data.offset (1);

	if (control & 0x80) != 0
	  {
	    /* Next control & 0x7f words are points.  */

	    control &= 0x7f;

	    while control != -1 && i < npoints
	      {
		if data >= end || data.offset (1) >= end
		  { break 'bail; }

		first += ((*data as c_uint) << 8u32) as c_int;
		data = data.offset (1);
		first += *data as c_int;
		data = data.offset (1);
		*points.offset (i as isize) = first as uint16_t;
		control -= 1;
		i += 1;
	      }
	  }
	else
	  {
	    /* Next control bytes are points.  */

	    while control != -1 && i < npoints
	      {
		if data >= end
		  { break 'bail; }

		first += *data as c_int;
		data = data.offset (1);
		*points.offset (i as isize) = first as uint16_t;
		control -= 1;
		i += 1;
	      }
	  }
      }

    /* Return the points read.  */
    *npoints_return = npoints as uint16_t;
    *location = data;
    return points;
  } /* 'bail (C 14551) */

  /* bail: */
  xfree (points as *mut c_void);
  return std::ptr::null_mut ();
}


/* C 14555's blank line and C 14556-14560's comment stand above.  C's two `unsigned char
   *restrict` parameters drop the `restrict` (no Rust spelling, the C 14445 reading); C's `fail:`
   label is the port's `'fail: { … }` block (PORTING.md 2.2).  */

/* Read and return N packed deltas from DATA.  Set *DATA_RETURN to
   DATA plus the number of bytes read.

   END is the end of the glyph variation data.  Value is an array of N
   deltas upon success, and NULL upon failure.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_packed_deltas (mut data: *mut c_uchar, end: *mut c_uchar,
				   n: c_int, data_return: *mut *mut c_uchar)
  -> *mut sfnt_fword                                     // C 14562-14618
{
  let mut deltas: *mut sfnt_fword;
  let mut i: c_int;
  let mut count: c_int;
  let mut control: c_uchar;
  let mut value: uint16_t;

  if data >= end
    { return std::ptr::null_mut (); }

  deltas = xnmalloc (n as isize, std::mem::size_of::<sfnt_fword> () as isize)
    as *mut sfnt_fword;
  i = 0;

  'fail: {
    while i < n
      {
	if data >= end
	  { break 'fail; }

	control = *data;
	data = data.offset (1);
	count = (control as c_int) & 0x3f;

	while count != -1 && i < n
	  {
	    if ((control as c_int) & 0x80) != 0
	      { *deltas.offset (i as isize) = 0; i += 1; }
	    else if ((control as c_int) & 0x40) != 0
	      {
		if data.offset (1) >= end
		  { break 'fail; }

		value = ((*data as c_uint) << 8u32) as uint16_t;
		data = data.offset (1);
		value |= *data as uint16_t;
		data = data.offset (1);
		*deltas.offset (i as isize) = value as sfnt_fword;
		i += 1;
	      }
	    else
	      {
		if data >= end
		  { break 'fail; }

		*deltas.offset (i as isize) = *data as int8_t as sfnt_fword;
		data = data.offset (1);
		i += 1;
	      }

	    count -= 1;
	  }
      }

    *data_return = data;
    return deltas;
  } /* 'fail (C 14615) */

  /* fail: */
  xfree (deltas as *mut c_void);
  return std::ptr::null_mut ();
}

/* Read a cvar table from the given font FD.  Use the table directory
   specified in SUBTABLE, axis information provided in the fvar table
   FVAR, and CVT information provided in the cvt table CVT.

   Return the cvar table upon success, else NULL.  */

/* The `bail`/`bail1`/`bail2` cascade is three nested blocks, so each `break` lands on the same
   cleanup text C's three labels do (PORTING.md 2.2).  `while (ntuples--)` keeps C's
   decrement-before-test order in the `while { … }` form.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_cvar_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable,
					       fvar: *mut sfnt_fvar_table,
					       cvt: *mut sfnt_cvt_table)
  -> *mut sfnt_cvar_table                                // C 14626-14950
{
  let mut directory: *mut sfnt_table_directory;
  let mut cvar: *mut sfnt_cvar_table;
  let mut rc: ssize_t;
  let mut ntuples: size_t;
  let mut size: size_t;
  let mut i: c_int;
  let mut j: c_int;
  let mut coords: *mut sfnt_f2dot14;
  let mut local: *mut uint16_t;
  let mut points: *mut uint16_t;
  let mut npoints: uint16_t;
  let mut data_size: uint16_t = 0;
  let mut min_size: uint16_t;
  let mut index: uint16_t = 0;
  let mut buffer: *mut c_uchar;
  let mut data: *mut c_uchar;
  let mut end: *mut c_uchar;
  let mut tuple: *mut c_uchar;
  let mut data_offset: ptrdiff_t;
  let mut deltas: *mut sfnt_fword;

  /* Find the table in the directory.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_CVAR);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  min_size = SFNT_ENDOF! (sfnt_cvar_table, data_offset, uint16_t) as uint16_t;

  /* Check that the length is at least min_size.  */
  if ((*directory).length as size_t) < min_size as size_t
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  /* Allocate enough to hold the cvar table header.  */
  cvar = xmalloc (std::mem::size_of::<sfnt_cvar_table> ()) as *mut sfnt_cvar_table;

  /* Read the cvar table header.  */
  'bail: {
    rc = read (fd, cvar as *mut c_void, min_size as size_t);
    if rc != min_size as ssize_t
      { break 'bail; }

    /* Swap what was read.  */
    sfnt_swap32! (&raw mut (*cvar).version);
    sfnt_swap16! (&raw mut (*cvar).tuple_count);
    sfnt_swap16! (&raw mut (*cvar).data_offset);

    /* Read the rest of the table.  */
    size = (*directory).length as size_t - min_size as size_t;
    buffer = xmalloc (size) as *mut c_uchar;
    rc = read (fd, buffer as *mut c_void, size);
    if rc == -1 || rc != size as ssize_t
      { break 'bail; }

    /* Now figure out how large cvar must be by reading the tuples.  */

    ntuples = ((*cvar).tuple_count & 0x0fff) as size_t;
    data_offset = ((*cvar).data_offset as ptrdiff_t) - (min_size as ptrdiff_t);
    end = buffer.offset (size as isize);

    'bail1: {
      if data_offset < 0
	{ break 'bail1; }

      /* See if there are shared points, and read them if there are.  */

      data = buffer.offset (data_offset);
      tuple = buffer;
      points = std::ptr::null_mut ();

      /* Initialize `npoints' to zero.  The specification doesn't say what
	 should happen with tuples using shared point numbers if it is not
	 set later on; simply assume no points at all apply to such a
	 tuple.  */

      npoints = 0;

      /* Initialize `size' to 0.  */
      size = 0;

      if ((*cvar).tuple_count & 0x8000) != 0
	{
	  points = sfnt_read_packed_points (data, &raw mut npoints, end,
					    &raw mut tuple);
	  if points.is_null ()
	    { break 'bail1; }

	  /* Add npoints words to the size.  If npoints is UINT16_MAX, no
	     coordinates will actually be allocated.  */

	  if npoints != UINT16_MAX as uint16_t
	    { size = (npoints as size_t) * std::mem::size_of::<uint16_t> (); }
	}

      'bail2: {
	while { ntuples = ntuples.wrapping_sub (1); ntuples != SIZE_MAX }
	  {
	    data = buffer.offset (data_offset);

	    /* Read the tuple.  */
	    if tuple.offset (3) >= end
	      { break 'bail2; }

	    memcpy (&raw mut data_size as *mut c_void,
		    tuple as *const c_void,
		    std::mem::size_of::<uint16_t> ());
	    tuple = tuple.offset (std::mem::size_of::<uint16_t> () as isize);
	    memcpy (&raw mut index as *mut c_void,
		    tuple as *const c_void,
		    std::mem::size_of::<uint16_t> ());
	    tuple = tuple.offset (std::mem::size_of::<uint16_t> () as isize);
	    sfnt_swap16! (&raw mut data_size);
	    sfnt_swap16! (&raw mut index);

	    /* Increment the offset to the data by the data size specified
	       here.  */
	    data_offset += data_size as ptrdiff_t;

	    if (index & 0x8000) != 0
	      {
		/* Embedded coordinates are present.  Read each coordinate
		   and add it to the size.  */

		if tuple.offset (((*fvar).axis_count as isize)
				 * (std::mem::size_of::<sfnt_f2dot14> () as isize)
				 - 1) >= end
		  { break 'bail2; }

		tuple = tuple.offset ((std::mem::size_of::<sfnt_f2dot14> ()
				       * (*fvar).axis_count as usize) as isize);
		let (ckd_size, add_overflow) = size.overflowing_add (
		  std::mem::size_of::<sfnt_f2dot14> () * (*fvar).axis_count as usize);
		size = ckd_size;
		if add_overflow
		  { break 'bail2; }
	      }
	    else
	      /* This table is invalid, as cvar tables don't have global
		 coordinates.  */
	      { break 'bail2; }

	    /* Now read indeterminate tuples if required.  */
	    if (index & 0x4000) != 0
	      {
		tuple = tuple.offset (((*fvar).axis_count * 4) as isize);
		let (ckd_size, add_overflow)
		  = size.overflowing_add (((*fvar).axis_count * 4) as usize);
		size = ckd_size;
		if add_overflow
		  { break 'bail2; }
	      }

	    /* Add one point and one delta for each CVT element.  */
	    let (ckd_size, add_overflow)
	      = size.overflowing_add ((*cvt).num_elements * 4);
	    size = ckd_size;
	    if add_overflow
	      { break 'bail2; }

	    /* Now add the size of the tuple.  */
	    let (ckd_size, add_overflow)
	      = size.overflowing_add (std::mem::size_of::<sfnt_tuple_variation> ());
	    size = ckd_size;
	    if add_overflow
	      { break 'bail2; }
	  }

	let (ckd_size, add_overflow)
	  = size.overflowing_add (std::mem::size_of::<sfnt_cvar_table> ());
	size = ckd_size;
	if add_overflow
	  { break 'bail2; }

	/* Reallocate cvar.  */
	cvar = xrealloc (cvar as *mut c_void, size) as *mut sfnt_cvar_table;
	ntuples = ((*cvar).tuple_count & 0x0fff) as size_t;
	(*cvar).variation = cvar.offset (1) as *mut sfnt_tuple_variation;
	coords = (*cvar).variation.offset (ntuples as isize) as *mut sfnt_f2dot14;
	tuple = buffer;

	data_offset = ((*cvar).data_offset as ptrdiff_t) - (min_size as ptrdiff_t);

	/* Start reading the tuples into cvar.  */
	i = 0;
	while (i as size_t) < ntuples
	  {
	    data = buffer.offset (data_offset);

	    /* Read the tuple.  */
	    if tuple.offset (3) >= end
	      { break 'bail2; }

	    memcpy (&raw mut data_size as *mut c_void,
		    tuple as *const c_void,
		    std::mem::size_of::<uint16_t> ());
	    tuple = tuple.offset (std::mem::size_of::<uint16_t> () as isize);
	    memcpy (&raw mut index as *mut c_void,
		    tuple as *const c_void,
		    std::mem::size_of::<uint16_t> ());
	    tuple = tuple.offset (std::mem::size_of::<uint16_t> () as isize);
	    sfnt_swap16! (&raw mut data_size);
	    sfnt_swap16! (&raw mut index);

	    /* Increment the offset to the data by the data size specified
	       here.  */
	    data_offset += data_size as ptrdiff_t;

	    (*(*cvar).variation.offset (i as isize)).intermediate_start
	      = std::ptr::null_mut ();
	    (*(*cvar).variation.offset (i as isize)).intermediate_end
	      = std::ptr::null_mut ();

	    if (index & 0x8000) != 0
	      {
		/* Embedded coordinates are present.  Read each
		   coordinate.  */
		(*(*cvar).variation.offset (i as isize)).coordinates = coords;

		j = 0;
		while j < (*fvar).axis_count as c_int
		  {
		    if tuple.offset (1) >= end
		      { break 'bail2; }

		    memcpy (coords as *mut c_void, tuple as *const c_void,
			    std::mem::size_of::<sfnt_f2dot14> ());
		    coords = coords.offset (1);
		    tuple = tuple.offset (std::mem::size_of::<sfnt_f2dot14> () as isize);
		    sfnt_swap16! (coords);
		    j += 1;
		  }
	      }
	    else
	      { break 'bail2; }

	    /* Now read indeterminate tuples if required.  */
	    if (index & 0x4000) != 0
	      {
		(*(*cvar).variation.offset (i as isize)).intermediate_start = coords;

		j = 0;
		while j < (*fvar).axis_count as c_int
		  {
		    if tuple.offset (1) >= end
		      { break 'bail2; }

		    memcpy (coords as *mut c_void, tuple as *const c_void,
			    std::mem::size_of::<sfnt_f2dot14> ());
		    coords = coords.offset (1);
		    tuple = tuple.offset (std::mem::size_of::<sfnt_f2dot14> () as isize);
		    sfnt_swap16! (coords);
		    j += 1;
		  }

		(*(*cvar).variation.offset (i as isize)).intermediate_end = coords;

		j = 0;
		while j < (*fvar).axis_count as c_int
		  {
		    if tuple.offset (1) >= end
		      { break 'bail2; }

		    memcpy (coords as *mut c_void, tuple as *const c_void,
			    std::mem::size_of::<sfnt_f2dot14> ());
		    coords = coords.offset (1);
		    tuple = tuple.offset (std::mem::size_of::<sfnt_f2dot14> () as isize);
		    sfnt_swap16! (coords);
		    j += 1;
		  }
	      }

	    /* Finally, read private ``point'' numbers.  If this flag is not
	       set, use shared point numbers previously read.

	       Read at most CVT->num_elements points, as that is all the
	       storage allocated.  */

	    if (index & 0x2000) != 0
	      {
		local = sfnt_read_packed_points (
		  data,
		  &raw mut (*(*cvar).variation.offset (i as isize)).num_points,
		  end, &raw mut data);
		if local.is_null ()
		  { break 'bail2; }

		/* If points apply to all CVT indices, skip this part.  */

		if (*(*cvar).variation.offset (i as isize)).num_points
		  != UINT16_MAX as uint16_t
		  {
		    if ((*(*cvar).variation.offset (i as isize)).num_points as size_t)
		      > (*cvt).num_elements
		      {
			(*(*cvar).variation.offset (i as isize)).num_points
			  = (*cvt).num_elements as uint16_t;
		      }

		    (*(*cvar).variation.offset (i as isize)).points
		      = coords as *mut uint16_t;
		    j = 0;
		    while j < (*(*cvar).variation.offset (i as isize)).num_points as c_int
		      {
			*coords = *local.offset (j as isize) as sfnt_f2dot14;
			coords = coords.offset (1);
			j += 1;
		      }
		    xfree (local as *mut c_void);
		  }
		else
		  { (*(*cvar).variation.offset (i as isize)).points = std::ptr::null_mut (); }
	      }
	    else
	      {
		/* Copy in the shared point numbers instead.  */
		(*(*cvar).variation.offset (i as isize)).num_points = npoints;

		if !points.is_null () && npoints != UINT16_MAX as uint16_t
		  {
		    if ((*(*cvar).variation.offset (i as isize)).num_points as size_t)
		      > (*cvt).num_elements
		      {
			(*(*cvar).variation.offset (i as isize)).num_points
			  = (*cvt).num_elements as uint16_t;
		      }

		    (*(*cvar).variation.offset (i as isize)).points
		      = coords as *mut uint16_t;
		    j = 0;
		    while j < (*(*cvar).variation.offset (i as isize)).num_points as c_int
		      {
			*coords = *points.offset (j as isize) as sfnt_f2dot14;
			coords = coords.offset (1);
			j += 1;
		      }
		  }
		else
		  { (*(*cvar).variation.offset (i as isize)).points = std::ptr::null_mut (); }
	      }

	    /* And read packed deltas.  If cvar->variation[i].num_points is
	       UINT16_MAX, then there is one delta for each CVT entry.
	       Otherwise, there are that many deltas.  */

	    if (*(*cvar).variation.offset (i as isize)).num_points
	      == UINT16_MAX as uint16_t
	      {
		deltas = sfnt_read_packed_deltas (data, end,
						  (*cvt).num_elements as c_int,
						  &raw mut data);

		if deltas.is_null ()
		  { break 'bail2; }

		(*(*cvar).variation.offset (i as isize)).deltas = coords;

		j = 0;
		while j < (*cvt).num_elements as c_int
		  {
		    *coords = *deltas.offset (j as isize);
		    coords = coords.offset (1);
		    j += 1;
		  }
		xfree (deltas as *mut c_void);
	      }
	    else
	      {
		deltas = sfnt_read_packed_deltas (
		  data, end,
		  (*(*cvar).variation.offset (i as isize)).num_points as c_int,
		  &raw mut data);
		if deltas.is_null ()
		  { break 'bail2; }

		(*(*cvar).variation.offset (i as isize)).deltas = coords;

		j = 0;
		while j < (*(*cvar).variation.offset (i as isize)).num_points as c_int
		  {
		    *coords = *deltas.offset (j as isize);
		    coords = coords.offset (1);
		    j += 1;
		  }
		xfree (deltas as *mut c_void);
	      }

	    i += 1;
	  }

	/* Free data and return the read cvar table.  */
	if points != (usize::MAX as *mut uint16_t)
	  { xfree (points as *mut c_void); }
	xfree (buffer as *mut c_void);
	return cvar;
      } /* 'bail2 (C 14942) */

      if points != (usize::MAX as *mut uint16_t)
	{ xfree (points as *mut c_void); }
    } /* 'bail1 (C 14945) */

    xfree (buffer as *mut c_void);
  } /* 'bail (C 14947) */

  /* bail: */
  xfree (cvar as *mut c_void);
  return std::ptr::null_mut ();
}

/* Initialize the specified BLEND with the given FVAR and GVAR tables.
   If non-NULL, adjust normalized coordinates using the axis variation
   table AVAR; similarly, adjust interpreter CVT values using CVAR, if
   specified.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_init_blend (blend: *mut sfnt_blend,
					  fvar: *mut sfnt_fvar_table,
					  gvar: *mut sfnt_gvar_table,
					  avar: *mut sfnt_avar_table,
					  cvar: *mut sfnt_cvar_table)
{                                                          // C 14959-14975
  let mut size: size_t;

  (*blend).fvar = fvar;
  (*blend).gvar = gvar;
  (*blend).avar = avar;
  (*blend).cvar = cvar;

  /* Allocate a single array to hold both coords and norm_coords.  */
  size = ((*fvar).axis_count as size_t) * std::mem::size_of::<sfnt_fixed> () * 2;
  (*blend).coords = xmalloc (size) as *mut sfnt_fixed;
  (*blend).norm_coords
    = (*blend).coords.offset ((*fvar).axis_count as isize);
}

/* Free what was initialized in the specified BLEND.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_free_blend (blend: *mut sfnt_blend)
{                                                          // C 14979-14983
  xfree ((*blend).coords as *mut c_void);
}

/* Normalize BLEND->fvar->axis_count coordinates in BLEND->coords and
   place the result in BLEND->norm_coords.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_normalize_blend (blend: *mut sfnt_blend)
{                                                          // C 14988-15074
  let mut axis: *mut sfnt_variation_axis;
  let mut i: c_int;
  let mut j: c_int;
  let mut from: sfnt_fixed;
  let mut coord: sfnt_fixed;
  let mut j0: sfnt_fixed;
  let mut j1: sfnt_fixed;
  let mut j2: sfnt_fixed;
  let mut from_last: sfnt_fixed;
  let mut coord_last: sfnt_fixed;
  let mut segment: *mut sfnt_short_frac_segment;

  /* For each axis... */
  i = 0;
  while i < (*((*blend).fvar)).axis_count as c_int
    {
      /* Normalize based on [min, default, max], into [-1, 0, 1].  */
      axis = (*((*blend).fvar)).axis.offset (i as isize);

      /* Load the current design coordinate.  */
      coord = *(*blend).coords.offset (i as isize);

      /* Keep it within bounds.  */

      if coord > (*axis).max_value
	{ coord = (*axis).max_value; }
      else if coord < (*axis).min_value
	{ coord = (*axis).min_value; }

      if coord > (*axis).default_value
	{
	  /* Avoid division by 0.  */
	  if (*axis).max_value != (*axis).default_value
	    {
	      *(*blend).norm_coords.offset (i as isize)
		= sfnt_div_fixed (sfnt_sub! (coord, (*axis).default_value),
				  sfnt_sub! ((*axis).max_value,
					     (*axis).default_value));
	    }
	  else
	    { *(*blend).norm_coords.offset (i as isize) = 0; }
	}
      else if coord < (*axis).default_value
	{
	  if (*axis).default_value != (*axis).min_value
	    {
	      *(*blend).norm_coords.offset (i as isize)
		= sfnt_div_fixed (sfnt_sub! (coord, (*axis).default_value),
				  sfnt_sub! ((*axis).default_value,
					     (*axis).min_value));
	    }
	  else
	    { *(*blend).norm_coords.offset (i as isize) = 0; }
	}
      else
	{ *(*blend).norm_coords.offset (i as isize) = 0; }

      i += 1;
    }

  /* Now, apply axis variations, but only if the avar table has the
     right number of axes.  */

  if !(*blend).avar.is_null ()
    && ((*((*blend).fvar)).axis_count as c_int == (*((*blend).avar)).axis_count)
    {
      i = 0;
      while i < (*((*blend).fvar)).axis_count as c_int
	{
	  segment = (*((*blend).avar)).segments.offset (i as isize);

	  /* Search for a correspondence record above the normalized
	     coordinate of this axis.  */

	  j = 1;
	  while j < (*segment).pair_count as c_int
	    {
	      from = (*(*segment).correspondence.offset (j as isize)).from_coord
		as sfnt_fixed * 4;
	      coord = (*(*segment).correspondence.offset (j as isize)).to_coord
		as sfnt_fixed * 4;

	      if *(*blend).norm_coords.offset (i as isize) < from
		{
		  from_last
		    = (*(*segment).correspondence.offset (j as isize - 1)).from_coord
		    as sfnt_fixed * 4;
		  coord_last
		    = (*(*segment).correspondence.offset (j as isize - 1)).to_coord
		    as sfnt_fixed * 4;

		  j0 = *(*blend).norm_coords.offset (i as isize) - from_last;
		  j1 = coord - coord_last;
		  j2 = from - from_last;

		  *(*blend).norm_coords.offset (i as isize)
		    = (sfnt_multiply_divide_signed (j0, j1, j2) + coord_last);
		  break;
		}

	      j += 1;
	    }

	  i += 1;
	}
    }
}

/* C 15076's form feed is dropped.  */

/* C 15078-15088 defines this record in the .c file (it is not in sfnt.h), so the port defines it
   here rather than in the mirror.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfnt_gvar_glyph_header
{
  /* A packed field. The high 4 bits are flags and the low 12 bits are
     the number of tuples for this glyph.  The number of tuples can be
     any number between 1 and 4095.  */
  pub tuple_count: uint16_t,

  /* Offset from the start of the GlyphVariationData table to the
     serialized data.  */
  pub data_offset: uint16_t,
}

/* Given a BLEND containing normalized coordinates, an array of
   BLEND->gvar->axis_count tuple coordinates, and, if INTERMEDIATE_P,
   a range of tuple coordinates from INTERMEDIATE_START to
   INTERMEDIATE_END, return the scaling factor to apply to deltas for
   each corresponding point.  */

/* C 15103's `start UNINIT, end UNINIT` are zeroed (PORTING.md 1.5); they are read only when
   `intermediate_p` is set, which is the same guard that writes them.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_compute_tuple_scale (blend: *mut sfnt_blend, intermediate_p: bool,
				    coords: *mut sfnt_f2dot14,
				    intermediate_start: *mut sfnt_f2dot14,
				    intermediate_end: *mut sfnt_f2dot14)
  -> sfnt_fixed                                          // C 15096-15174
{
  let mut i: c_int;
  let mut coord: sfnt_fixed;
  let mut start: sfnt_fixed = 0;
  let mut end: sfnt_fixed = 0;
  let mut scale: sfnt_fixed;

  /* scale is initially 1.0.  */
  scale = 0o200000;

  i = 0;
  while i < (*((*blend).gvar)).axis_count as c_int
    {
      /* Load values for this axis, scaled up to sfnt_fixed.  */
      coord = *coords.offset (i as isize) as sfnt_fixed * 4;

      /* GCC warns about start and end being used when uninitialized,
	 but they are used only if intermediate_p.  */

      if intermediate_p
	{
	  start = *intermediate_start.offset (i as isize) as sfnt_fixed * 4;
	  end = *intermediate_end.offset (i as isize) as sfnt_fixed * 4;
	}

      /* Ignore tuples that can be skipped.  */

      if coord == 0
	{ i += 1; continue; }

      /* If the coordinate is set to 0, then deltas should not be
	 applied.  Return 0.  */

      if *(*blend).norm_coords.offset (i as isize) == 0
	{ return 0; }

      /* If no scaling need take place, continue.  */

      if *(*blend).norm_coords.offset (i as isize) == coord
	{ i += 1; continue; }

      if !intermediate_p
	{
	  /* Not an intermediate tuple; if coord is less than 0 and
	     blend->norm_coords[i] < coord, or coord is more than 0
	     and blend->norm_coords[i] > coord, then it doesn't fit,
	     so return.  */

	  if *(*blend).norm_coords.offset (i as isize) < MIN! (0, coord)
	      || *(*blend).norm_coords.offset (i as isize) > MAX! (0, coord)
	    { return 0; }

	  scale = sfnt_multiply_divide_signed (scale,
					       *(*blend).norm_coords.offset (i as isize),
					       coord);
	}
      else
	{
	  /* Otherwise, renormalize between start and end.  */

	  if *(*blend).norm_coords.offset (i as isize) < start
	      || *(*blend).norm_coords.offset (i as isize) > end
	    { return 0; }

	  if *(*blend).norm_coords.offset (i as isize) < coord
	    {
	      scale = sfnt_multiply_divide (
		scale as c_uint,
		(*(*blend).norm_coords.offset (i as isize) - start) as c_uint,
		(coord - start) as c_uint) as sfnt_fixed;
	    }
	  else
	    {
	      scale = sfnt_multiply_divide (
		scale as c_uint,
		(end - *(*blend).norm_coords.offset (i as isize)) as c_uint,
		(end - coord) as c_uint) as sfnt_fixed;
	    }
	}

      i += 1;
    }

  return scale;
}

/* Move each point in the simple glyph GLYPH between PAIR_START and
   PAIR_END to agree with the positions of those two anchor points as
   compared with their initial positions recorded within the arrays X
   and Y.

   The range formed between PAIR_START and PAIR_END may encompass the
   upper extreme of the contour between START and END.  */

/* C's two `goto` labels (`consider_y` and `next`) are the port's `'consider_y`/`'next` blocks
   (PORTING.md 2.2); the X-axis body's `break 'consider_y` is C's `goto consider_y`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_infer_deltas_2 (glyph: *mut sfnt_glyph, pair_start: size_t,
			       pair_end: size_t, start: size_t, end: size_t,
			       x: *mut sfnt_fword, y: *mut sfnt_fword)
							   // C 15184-15356
{
  let mut j: size_t;
  let mut min_pos: sfnt_fword;
  let mut max_pos: sfnt_fword;
  let mut position: sfnt_fword;
  let mut d1: sfnt_fword;
  let mut d2: sfnt_fword;
  let mut ratio: sfnt_fixed;
  let mut delta: sfnt_fixed;

  j = pair_start + 1;

  while j != pair_end
    {
      /* Reset j to the contour's start position if it is about to
	 overrun this contour.  */

      if j > end
	{
	  /* The start of the contour might also be the end of this
	     reference point.  */
	  if start == pair_end
	    { return; }

	  j = start;
	}

      /* Consider the X axis.  Set min_pos and max_pos to the
	 smallest and greatest values along that axis.  */
      min_pos = MIN! (*x.offset (pair_start as isize),
		      *x.offset (pair_end as isize));
      max_pos = MAX! (*x.offset (pair_start as isize),
		      *x.offset (pair_end as isize));

      /* Now see if the current point lies between min and
	 max...

	 GX interpolation differs from IUP in one important detail:
	 points are shifted to follow the movement of their reference
	 points if their positions are identical to those of any of
	 their reference points, whereas IUP considers such points to
	 fall within their reference points.  */
      'consider_y: {
	if *x.offset (j as isize) > min_pos && *x.offset (j as isize) < max_pos
	  {
	    /* Interpolate between min_pos and max_pos.  */
	    ratio = sfnt_div_fixed ((sfnt_sub! (*x.offset (j as isize), min_pos)
				     * 65536),
				    (sfnt_sub! (max_pos, min_pos)
				     * 65536));

	    /* Load the current positions of pair_start and pair_end
	       along this axis.  */
	    min_pos = MIN! (*(*(*glyph).simple).x_coordinates.offset (pair_start as isize),
			    *(*(*glyph).simple).x_coordinates.offset (pair_end as isize));
	    max_pos = MAX! (*(*(*glyph).simple).x_coordinates.offset (pair_start as isize),
			    *(*(*glyph).simple).x_coordinates.offset (pair_end as isize));

	    /* Lerp in between.  */
	    delta = sfnt_sub! (max_pos, min_pos);
	    delta = sfnt_mul_fixed (ratio, delta);
	    *(*(*glyph).simple).x_coordinates.offset (j as isize)
	      = (min_pos as sfnt_fixed + delta) as int16_t;
	  }
	else
	  {
	    /* ... otherwise, move point j by the delta of the
	       nearest touched point.  */

	    /* If min_pos and max_pos are the same, apply
	       pair_start's delta if it is identical to that of
	       pair_end, or apply nothing at all otherwise.  */

	    if min_pos == max_pos
	      {
		d1 = (*(*(*glyph).simple).x_coordinates.offset (pair_start as isize)
		      as c_int - *x.offset (pair_start as isize) as c_int) as sfnt_fword;
		d2 = (*(*(*glyph).simple).x_coordinates.offset (pair_end as isize)
		      as c_int - *x.offset (pair_end as isize) as c_int) as sfnt_fword;

		if d1 == d2
		  {
		    *(*(*glyph).simple).x_coordinates.offset (j as isize)
		      = (*(*(*glyph).simple).x_coordinates.offset (j as isize)
			 as c_int + d1 as c_int) as int16_t;
		  }

		break 'consider_y;
	      }

	    if *x.offset (j as isize) >= max_pos
	      {
		position = MAX! (*(*(*glyph).simple).x_coordinates.offset (pair_start as isize),
				 *(*(*glyph).simple).x_coordinates.offset (pair_end as isize));
		delta = position as sfnt_fixed - max_pos as sfnt_fixed;
	      }
	    else
	      {
		position = MIN! (*(*(*glyph).simple).x_coordinates.offset (pair_start as isize),
				 *(*(*glyph).simple).x_coordinates.offset (pair_end as isize));
		delta = position as sfnt_fixed - min_pos as sfnt_fixed;
	      }

	    *(*(*glyph).simple).x_coordinates.offset (j as isize)
	      = (*x.offset (j as isize) as sfnt_fixed + delta) as int16_t;
	  }
      } /* 'consider_y (C 15281) */

      /* consider_y: */

      /* Now, consider the Y axis.  */
      'next: {
	min_pos = MIN! (*y.offset (pair_start as isize),
			*y.offset (pair_end as isize));
	max_pos = MAX! (*y.offset (pair_start as isize),
			*y.offset (pair_end as isize));

	if *y.offset (j as isize) > min_pos && *y.offset (j as isize) < max_pos
	  {
	    /* Interpolate between min_pos and max_pos.  */
	    ratio = sfnt_div_fixed ((sfnt_sub! (*y.offset (j as isize), min_pos)
				     * 65536),
				    (sfnt_sub! (max_pos, min_pos)
				     * 65536));

	    /* Load the current positions of pair_start and pair_end
	       along this axis.  */
	    min_pos = MIN! (*(*(*glyph).simple).y_coordinates.offset (pair_start as isize),
			    *(*(*glyph).simple).y_coordinates.offset (pair_end as isize));
	    max_pos = MAX! (*(*(*glyph).simple).y_coordinates.offset (pair_start as isize),
			    *(*(*glyph).simple).y_coordinates.offset (pair_end as isize));

	    /* Lerp in between.  */
	    delta = sfnt_sub! (max_pos, min_pos);
	    delta = sfnt_mul_fixed (ratio, delta);
	    *(*(*glyph).simple).y_coordinates.offset (j as isize)
	      = (min_pos as sfnt_fixed + delta) as int16_t;
	  }
	else
	  {
	    /* ... otherwise, move point j by the delta of the
	       nearest touched point.  */

	    /* If min_pos and max_pos are the same, apply
	       pair_start's delta if it is identical to that of
	       pair_end, or apply nothing at all otherwise.  */

	    if min_pos == max_pos
	      {
		d1 = (*(*(*glyph).simple).y_coordinates.offset (pair_start as isize)
		      as c_int - *y.offset (pair_start as isize) as c_int) as sfnt_fword;
		d2 = (*(*(*glyph).simple).y_coordinates.offset (pair_end as isize)
		      as c_int - *y.offset (pair_end as isize) as c_int) as sfnt_fword;

		if d1 == d2
		  {
		    *(*(*glyph).simple).y_coordinates.offset (j as isize)
		      = (*(*(*glyph).simple).y_coordinates.offset (j as isize)
			 as c_int + d1 as c_int) as int16_t;
		  }

		break 'next;
	      }

	    if *y.offset (j as isize) >= max_pos
	      {
		position = MAX! (*(*(*glyph).simple).y_coordinates.offset (pair_start as isize),
				 *(*(*glyph).simple).y_coordinates.offset (pair_end as isize));
		delta = position as sfnt_fixed - max_pos as sfnt_fixed;
	      }
	    else
	      {
		position = MIN! (*(*(*glyph).simple).y_coordinates.offset (pair_start as isize),
				 *(*(*glyph).simple).y_coordinates.offset (pair_end as isize));
		delta = position as sfnt_fixed - min_pos as sfnt_fixed;
	      }

	    *(*(*glyph).simple).y_coordinates.offset (j as isize)
	      = (*y.offset (j as isize) as sfnt_fixed + delta) as int16_t;
	  }
      } /* 'next (C 15353) */

      /* next: */
      j += 1;
    }
}

/* Infer point positions for points that have been partially moved
   within the contour in GLYPH denoted by START and END.  */

/* C's `goto next` is the port's `break 'next` (PORTING.md 2.2); the `continue` carries the
   `for`-loop increment C's `++i` would skip (the ledger's Deviation 27 reading).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_infer_deltas_1 (glyph: *mut sfnt_glyph, start: size_t,
			       end: size_t, touched: *mut bool,
			       x: *mut sfnt_fword, y: *mut sfnt_fword)
							   // C 15361-15406
{
  let mut i: size_t;
  let mut pair_start: size_t;
  let mut pair_end: size_t;
  let mut pair_first: size_t;

  pair_first = SIZE_MAX;
  pair_start = SIZE_MAX;

  /* Look for pairs of touched points.  */

  i = start;
  while i <= end
    {
      if !*touched.offset (i as isize)
	{ i += 1; continue; }

      'next: {
	if pair_start == SIZE_MAX
	  {
	    pair_first = i;
	    break 'next;
	  }

	pair_end = i;

	/* pair_start to pair_end are now a pair of points whose
	   intermediates should be interpolated.  */
	sfnt_infer_deltas_2 (glyph, pair_start, pair_end,
			     start, end, x, y);
      } /* 'next (C 15390) */

      /* next: */
      pair_start = i;

      i += 1;
    }

  /* If pair_start is set, then lerp points between it and
     pair_first.  */

  if pair_start != (SIZE_MAX as size_t)
    {
      pair_end = pair_first;

      /* pair_start to pair_end are now a pair of points whose
	 intermediates should be interpolated.  */
      sfnt_infer_deltas_2 (glyph, pair_start, pair_end,
			   start, end, x, y);
    }
}

/* Infer point positions for contours that have been partially moved
   by variation.  For each contour in GLYPH, find pairs of points
   which have had deltas applied.  For each such pair, interpolate
   points between the first point in the pair and the second by
   considering each point along every one of the two axes (X and Y)
   like so:

     - For each point that lies between the first point and the last
       on the axis currently being considered, interpolate its
       position in that axis so that the ratio formed by its position
       relative to the first and last points of the pair in the
       original outline still holds.

     - For each point that lies to the left or top of the first point
       on the axis being considered, use the delta of the first point.

     - And finally, for each point that lies to the right or bottom of
       the last point on that axis, use the delta of the last
       point.

   X and Y contain the original positions of each point.
   TOUCHED contains whether or not each point within GLYPH has been
   changed through variation.

   Apply the inferred deltas back to GLYPH.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_infer_deltas (glyph: *mut sfnt_glyph, touched: *mut bool,
			     x: *mut sfnt_fword, y: *mut sfnt_fword)
							   // C 15434-15457
{
  let mut i: size_t;
  let mut point: c_int;
  let mut first: c_int;
  let mut end: c_int;

  point = 0;
  i = 0;
  while i < (*glyph).number_of_contours as usize
    {
      first = point;
      end = *(*(*glyph).simple).end_pts_of_contours.offset (i as isize) as c_int;

      /* Return if the glyph is invalid.  */

      if (first as size_t) >= (*(*glyph).simple).number_of_points
	  || (end as size_t) >= (*(*glyph).simple).number_of_points
	  || first > end
	{ return; }

      sfnt_infer_deltas_1 (glyph, first as size_t, end as size_t, touched, x, y);
      point = end + 1;

      i += 1;
    }
}

/* Read the glyph variation data for the specified glyph ID from
   BLEND's gvar table.  Apply the offsets to each point in the
   specified simple GLYPH, based on the specified BLEND.

   Value is 0 upon success, else 1.

   The glyph variation data consists of a number of elements, each of
   which has its own associated point numbers and deltas, and a list
   of one or two coordinates for each axis.  Each such list is
   referred to as a ``tuple''.

   The deltas, one for each point, are multiplied by the normalized
   value of each axis and applied to those points for each tuple that
   is found to be applicable.

   Each element of the glyph variation data is applicable to an axis
   if its list of coordinates:

     - contains one element for each axis, and its axis has a value
       between 0 and that element.

     - contains two elements for each axis, and its axis has a value
       between the first element and the second.

   Return the deltas that would normally be applied to the two phantom
   points describing horizontal bounds in *DISTORTION.  Do not
   transform the outline to reflect adjustments to the origin
   point.  */

/* C 15494's `struct sfnt_gvar_glyph_header header;` is `memcpy`ed into whole, and the port
   zeroes it first (PORTING.md 1.5, the C 1180 `sfnt_cmap_encoding_subtable_data` shape);
   the three `alloca`s are P-05's `sa_stack_alloc` (the ledger's Deviation 47).  C's `fail3:`
   and `fail1:` are the port's nested `'fail3`/`'fail1` blocks, so `break 'fail3` runs the
   three frees and then the `fail1` tail while `break 'fail1` skips them (PORTING.md 2.2).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_vary_simple_glyph (blend: *mut sfnt_blend,
						 id: sfnt_glyph_index,
						 glyph: *mut sfnt_glyph,
						 distortion: *mut sfnt_metrics_distortion)
  -> c_int                                                // C 15488-15851
{
  let mut offset: uint32_t;
  let mut header: sfnt_gvar_glyph_header = std::mem::zeroed ();
  let mut points: *mut uint16_t;
  let mut npoints: uint16_t;
  let mut i: c_int;
  let mut ntuples: c_int;
  let mut j: c_int;
  let mut point_count: c_int;
  let mut tuple: *mut c_uchar = std::ptr::null_mut ();
  let mut end: *mut c_uchar;
  let mut data: *mut c_uchar;
  let mut data_size: uint16_t = 0;
  let mut index: uint16_t = 0;
  let mut glyph_points: *mut uint16_t;
  let mut coords: *mut sfnt_f2dot14;
  let mut intermediate_start: *mut sfnt_f2dot14;
  let mut intermediate_end: *mut sfnt_f2dot14;
  let mut dx: *mut sfnt_fword;
  let mut dy: *mut sfnt_fword;
  let mut fword: sfnt_fword;
  let mut gvar: *mut sfnt_gvar_table;
  let mut local_points: *mut uint16_t;
  let mut n_local_points: uint16_t = 0;
  let mut scale: sfnt_fixed;
  let mut data_offset: ptrdiff_t;
  let mut touched: *mut bool;
  let mut original_x: *mut sfnt_fword;
  let mut original_y: *mut sfnt_fword;

  gvar = (*blend).gvar;

  if (*gvar).axis_count as c_int != (*((*blend).fvar)).axis_count as c_int
    { return 1; }

  if ((*gvar).glyph_count as sfnt_glyph_index) <= id
    { return 1; }

  if ((*gvar).flags & 1) != 0
    { offset = *(*gvar).u.offset_long.offset (id as isize); }
  else
    { offset = (*(*gvar).u.offset_word.offset (id as isize) as uint32_t) * 2u32; }

  if (offset as size_t) >= (*gvar).data_size
    { return 1; }

  end = (*gvar).glyph_variation_data.offset ((*gvar).data_size as isize);

  /* Start reading the header.  */

  if (offset as size_t) + std::mem::size_of::<sfnt_gvar_glyph_header> ()
      > (*gvar).data_size
    { return 1; }

  /* Clear the distortion.  */
  (*distortion).origin = 0;
  (*distortion).advance = 0;

  memcpy (&raw mut header as *mut c_void,
	  (*gvar).glyph_variation_data.offset (offset as isize) as *const c_void,
	  std::mem::size_of::<sfnt_gvar_glyph_header> ());

  /* Swap the header.  */
  sfnt_swap16! (&raw mut header.tuple_count);
  sfnt_swap16! (&raw mut header.data_offset);

  /* Prepare to read each tuple.  */
  ntuples = (header.tuple_count & 0x0fff) as c_int;

  /* Initialize the data offset.  This is incremented with each tuple
     read.  */
  data_offset = header.data_offset as ptrdiff_t;

  /* If gvar->flags & tuples_share_point_numbers, read the shared
     point numbers.  Initialize `npoints' to zero.  The specification
     doesn't say what should happen with tuples using shared point
     numbers if it is not set later on; simply assume no points at all
     apply to such a tuple.  */

  npoints = 0;

  if (header.tuple_count & 0x8000) != 0
    {
      data = (*gvar).glyph_variation_data
	.offset ((offset as isize) + data_offset);
      points = sfnt_read_packed_points (data, &raw mut npoints, end,
					&raw mut tuple);

      if points.is_null ()
	{ return 1; }

      /* Shared point numbers are part of the data after the tuple
	 array.  Thus, increment data_offset by tuple - data.  `tuple'
	 here holds no relation to a pointer to the current part of
	 the tuple array that is being read later on.  */
      data_offset += (tuple as isize) - (data as isize);
    }
  else
    { points = std::ptr::null_mut (); }

  /* Start reading each tuple.  */
  tuple = (*gvar).glyph_variation_data
    .offset ((offset as isize) + (std::mem::size_of::<sfnt_gvar_glyph_header> () as isize));

  let mut coords_alloc: *mut c_void = std::ptr::null_mut ();
  if ((*gvar).axis_count as size_t)
      < 1024 * 16 / (3 * std::mem::size_of::<sfnt_f2dot14> ())
    {
      coords = sa_stack_alloc ((*gvar).axis_count as size_t
			       * std::mem::size_of::<sfnt_f2dot14> () * 3)
	as *mut sfnt_f2dot14;
    }
  else
    {
      coords_alloc = xnmalloc ((*gvar).axis_count as isize,
			       (3 * std::mem::size_of::<sfnt_f2dot14> ()) as isize);
      coords = coords_alloc as *mut sfnt_f2dot14;
    }

  intermediate_start = coords.offset ((*gvar).axis_count as isize);
  intermediate_end = intermediate_start.offset ((*gvar).axis_count as isize);

  /* Allocate arrays of booleans and fwords to keep track of which
     points have been touched.  */
  touched = std::ptr::null_mut ();
  original_x = std::ptr::null_mut ();
  original_y = std::ptr::null_mut ();
  let mut touched_alloc: *mut c_void = std::ptr::null_mut ();
  let mut original_x_alloc: *mut c_void = std::ptr::null_mut ();

  'fail1: {
    'fail3: {
      while { ntuples = ntuples.wrapping_sub (1); ntuples != -1 }
	{
	  data = (*gvar).glyph_variation_data
	    .offset ((offset as isize) + data_offset);

	  if tuple.offset (3) >= end
	    { break 'fail1; }

	  memcpy (&raw mut data_size as *mut c_void,
		  tuple as *const c_void,
		  std::mem::size_of::<uint16_t> ());
	  tuple = tuple.offset (std::mem::size_of::<uint16_t> () as isize);
	  memcpy (&raw mut index as *mut c_void,
		  tuple as *const c_void,
		  std::mem::size_of::<uint16_t> ());
	  tuple = tuple.offset (std::mem::size_of::<uint16_t> () as isize);
	  sfnt_swap16! (&raw mut data_size);
	  sfnt_swap16! (&raw mut index);

	  /* Increment the offset to the data by the data size specified
	     here.  */
	  data_offset += data_size as ptrdiff_t;

	  if (index & 0x8000) != 0
	    {
	      /* Embedded coordinates are present.  Read each
		 coordinate and add it to the tuple.  */
	      j = 0;
	      while j < (*gvar).axis_count as c_int
		{
		  if tuple.offset (1) >= end
		    { break 'fail1; }

		  memcpy (coords.offset (j as isize) as *mut c_void,
			  tuple as *const c_void,
			  std::mem::size_of::<sfnt_f2dot14> ());
		  tuple = tuple.offset (std::mem::size_of::<sfnt_f2dot14> () as isize);
		  sfnt_swap16! (coords.offset (j as isize));
		  j += 1;
		}
	    }
	  else if (*gvar).shared_coord_count <= (index & 0x0fff)
	    /* The index is too large.  */
	    { break 'fail1; }
	  else
	    /* index points into gvar->axis_count coordinates making up
	       the tuple.  */
	    {
	      memcpy (coords as *mut c_void,
		      (*gvar).global_coords.offset (
			(((index & 0x0fff) as isize)
			 * ((*gvar).axis_count as isize))) as *const c_void,
		      ((*gvar).axis_count as usize)
		      * std::mem::size_of::<sfnt_f2dot14> ());
	    }

	  /* Now read indeterminate tuples if required.  */
	  if (index & 0x4000) != 0
	    {
	      j = 0;
	      while j < (*gvar).axis_count as c_int
		{
		  if tuple.offset (1) >= end
		    { break 'fail1; }

		  memcpy (intermediate_start.offset (j as isize) as *mut c_void,
			  tuple as *const c_void,
			  std::mem::size_of::<sfnt_f2dot14> ());
		  tuple = tuple.offset (std::mem::size_of::<sfnt_f2dot14> () as isize);
		  sfnt_swap16! (intermediate_start.offset (j as isize));
		  j += 1;
		}

	      j = 0;
	      while j < (*gvar).axis_count as c_int
		{
		  if tuple.offset (1) >= end
		    { break 'fail1; }

		  memcpy (intermediate_end.offset (j as isize) as *mut c_void,
			  tuple as *const c_void,
			  std::mem::size_of::<sfnt_f2dot14> ());
		  tuple = tuple.offset (std::mem::size_of::<sfnt_f2dot14> () as isize);
		  sfnt_swap16! (intermediate_end.offset (j as isize));
		  j += 1;
		}
	    }

	  /* See whether or not the tuple applies to the current variation
	     configuration, and how much to scale them by.  */

	  scale = sfnt_compute_tuple_scale (blend, (index & 0x4000) != 0,
					    coords, intermediate_start,
					    intermediate_end);

	  if scale == 0
	    { continue; }

	  local_points = std::ptr::null_mut ();

	  /* Finally, read private point numbers.
	     Set local_points to those numbers; it will be freed
	     once the loop ends.  */

	  if (index & 0x2000) != 0
	    {
	      local_points = sfnt_read_packed_points (data, &raw mut n_local_points,
						      end, &raw mut data);
	      if local_points.is_null ()
		{ break 'fail1; }

	      point_count = n_local_points as c_int;
	      glyph_points = local_points;
	    }
	  else
	    {
	      /* If there are no private point numbers, use global
		 points.  */
	      point_count = npoints as c_int;
	      glyph_points = points;
	    }

	  /* Now, read packed deltas.  */

	  dx = std::ptr::null_mut ();
	  dy = std::ptr::null_mut ();

	  match point_count
	    {
	      UINT16_MAX =>
		{
		  /* Deltas are provided for all points in the glyph.
		     No glyph should have more than 65535 points.  */

		  /* Add 4 phantom points to each end.  */
		  dx = sfnt_read_packed_deltas (
		    data, end,
		    ((*(*glyph).simple).number_of_points + 4) as c_int,
		    &raw mut data);
		  dy = sfnt_read_packed_deltas (
		    data, end,
		    ((*(*glyph).simple).number_of_points + 4) as c_int,
		    &raw mut data);

		  if dx.is_null () || dy.is_null ()
		    { break 'fail3; }

		  /* Apply each delta to the simple glyph.  */

		  i = 0;
		  while i < (*(*glyph).simple).number_of_points as c_int
		    {
		      fword = sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
						    scale) as sfnt_fword;
		      *(*(*glyph).simple).x_coordinates.offset (i as isize)
			= (*(*(*glyph).simple).x_coordinates.offset (i as isize)
			   as c_int + fword as c_int) as int16_t;
		      fword = sfnt_mul_fixed_round (*dy.offset (i as isize) as sfnt_fixed,
						    scale) as sfnt_fword;
		      *(*(*glyph).simple).y_coordinates.offset (i as isize)
			= (*(*(*glyph).simple).y_coordinates.offset (i as isize)
			   as c_int + fword as c_int) as int16_t;
		      i += 1;
		    }

		  /* Apply the deltas for the two phantom points.  */
		  (*distortion).origin
		    = ((*distortion).origin as c_int).wrapping_add (
			sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
					      scale)) as sfnt_fword;
		  i += 1;
		  (*distortion).advance
		    = ((*distortion).advance as c_int).wrapping_add (
			sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
					      scale)) as sfnt_fword;
		}

	      _ =>
		{
		  dx = sfnt_read_packed_deltas (data, end, point_count, &raw mut data);
		  dy = sfnt_read_packed_deltas (data, end, point_count, &raw mut data);

		  if dx.is_null () || dy.is_null ()
		    { break 'fail3; }

		  /* Deltas are only applied for each point number read.  */

		  if original_x.is_null ()
		    {
		      if (*(*glyph).simple).number_of_points
			  < 1024 * 16 / std::mem::size_of::<bool> ()
			{
			  touched = sa_stack_alloc (
			    std::mem::size_of::<bool> ()
			    * (*(*glyph).simple).number_of_points) as *mut bool;
			}
		      else
			{
			  touched_alloc = xnmalloc (
			    (*(*glyph).simple).number_of_points as isize,
			    std::mem::size_of::<bool> () as isize);
			  touched = touched_alloc as *mut bool;
			}

		      if (*(*glyph).simple).number_of_points
			  < 1024 * 16 / (2 * std::mem::size_of::<sfnt_fword> ())
			{
			  original_x = sa_stack_alloc (
			    std::mem::size_of::<sfnt_fword> () * 2
			    * (*(*glyph).simple).number_of_points) as *mut sfnt_fword;
			}
		      else
			{
			  original_x_alloc = xnmalloc (
			    (*(*glyph).simple).number_of_points as isize,
			    (2 * std::mem::size_of::<sfnt_fword> ()) as isize);
			  original_x = original_x_alloc as *mut sfnt_fword;
			}

		      original_y
			= original_x.offset ((*(*glyph).simple).number_of_points as isize);
		    }

		  /* The array of original coordinates should reflect the
		     state of the glyph immediately before deltas from this
		     tuple are applied, in contrast to the state before any
		     deltas are applied.  */

		  memcpy (original_x as *mut c_void,
			  (*(*glyph).simple).x_coordinates as *const c_void,
			  std::mem::size_of::<sfnt_fword> ()
			  * (*(*glyph).simple).number_of_points);
		  memcpy (original_y as *mut c_void,
			  (*(*glyph).simple).y_coordinates as *const c_void,
			  std::mem::size_of::<sfnt_fword> ()
			  * (*(*glyph).simple).number_of_points);

		  memset (touched as *mut c_void, 0,
			  std::mem::size_of::<bool> ()
			  * (*(*glyph).simple).number_of_points);

		  i = 0;
		  while i < point_count
		    {
		      /* Apply deltas to phantom points.  */

		      if *glyph_points.offset (i as isize) as size_t
			  == (*(*glyph).simple).number_of_points
			{
			  (*distortion).origin
			    = ((*distortion).origin as c_int).wrapping_add (
				sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
						      scale)) as sfnt_fword;
			  i += 1;
			  continue;
			}

		      if *glyph_points.offset (i as isize) as size_t
			  == (*(*glyph).simple).number_of_points + 1
			{
			  (*distortion).advance
			    = ((*distortion).advance as c_int).wrapping_add (
				sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
						      scale)) as sfnt_fword;
			  i += 1;
			  continue;
			}

		      /* Make sure the point doesn't end up out of bounds.  */
		      if *glyph_points.offset (i as isize) as size_t
			  >= (*(*glyph).simple).number_of_points
			{ i += 1; continue; }

		      fword = sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
						    scale) as sfnt_fword;
		      *(*(*glyph).simple).x_coordinates.offset (
			  *glyph_points.offset (i as isize) as isize)
			= (*(*(*glyph).simple).x_coordinates.offset (
			     *glyph_points.offset (i as isize) as isize)
			   as c_int + fword as c_int) as int16_t;
		      fword = sfnt_mul_fixed_round (*dy.offset (i as isize) as sfnt_fixed,
						    scale) as sfnt_fword;
		      *(*(*glyph).simple).y_coordinates.offset (
			  *glyph_points.offset (i as isize) as isize)
			= (*(*(*glyph).simple).y_coordinates.offset (
			     *glyph_points.offset (i as isize) as isize)
			   as c_int + fword as c_int) as int16_t;
		      *touched.offset (*glyph_points.offset (i as isize) as isize) = true;

		      i += 1;
		    }

		  sfnt_infer_deltas (glyph, touched, original_x,
				     original_y);
		}
	    }

	  xfree (dx as *mut c_void);
	  xfree (dy as *mut c_void);

	  if local_points != (usize::MAX as *mut uint16_t)
	    { xfree (local_points as *mut c_void); }
	}

      /* Return success.  */

      xfree (touched_alloc);
      xfree (coords_alloc);
      xfree (original_x_alloc);

      if points != (usize::MAX as *mut uint16_t)
	{ xfree (points as *mut c_void); }

      /* Set the glyph metrics distortion as well.  */
      (*glyph).advance_distortion = (*distortion).advance;
      (*glyph).origin_distortion = (*distortion).origin;

      return 0;
    } /* 'fail3 (C 15837) */

    /* fail3: */
    xfree (dx as *mut c_void);
    xfree (dy as *mut c_void);
    xfree (local_points as *mut c_void);
  } /* 'fail1 (C 15841) */

  /* fail1: */

  xfree (touched_alloc);
  xfree (coords_alloc);
  xfree (original_x_alloc);

  if points != (usize::MAX as *mut uint16_t)
    { xfree (points as *mut c_void); }

  return 1;
}

/* Read the glyph variation data for the specified glyph ID from
   BLEND's gvar table.  Apply the deltas specified within to each
   component with offsets in the specified compound GLYPH, based on
   the specified BLEND.  Return distortions to phantom points in
   *DISTORTION.

   Value is 0 upon success, 1 otherwise.  */

/* Same cleanup shape as `sfnt_vary_simple_glyph` above: `'fail3` holds the loop, `'fail1`
   wraps it, and `break 'fail3`/`break 'fail1` are C's two labels (PORTING.md 2.2).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_vary_compound_glyph (blend: *mut sfnt_blend,
						   id: sfnt_glyph_index,
						   glyph: *mut sfnt_glyph,
						   distortion: *mut sfnt_metrics_distortion)
  -> c_int                                                // C 15861-16212
{
  let mut offset: uint32_t;
  let mut header: sfnt_gvar_glyph_header = std::mem::zeroed ();
  let mut points: *mut uint16_t;
  let mut npoints: uint16_t;
  let mut i: c_int;
  let mut ntuples: c_int;
  let mut j: c_int;
  let mut point_count: c_int;
  let mut tuple: *mut c_uchar = std::ptr::null_mut ();
  let mut end: *mut c_uchar;
  let mut data: *mut c_uchar;
  let mut data_size: uint16_t = 0;
  let mut index: uint16_t = 0;
  let mut glyph_points: *mut uint16_t;
  let mut coords: *mut sfnt_f2dot14;
  let mut intermediate_start: *mut sfnt_f2dot14;
  let mut intermediate_end: *mut sfnt_f2dot14;
  let mut dx: *mut sfnt_fword;
  let mut dy: *mut sfnt_fword;
  let mut fword: sfnt_fword;
  let mut word: sfnt_fword;
  let mut gvar: *mut sfnt_gvar_table;
  let mut local_points: *mut uint16_t;
  let mut n_local_points: uint16_t = 0;
  let mut scale: sfnt_fixed;
  let mut data_offset: ptrdiff_t;
  let mut component: *mut sfnt_compound_glyph_component;

  gvar = (*blend).gvar;

  if (*gvar).axis_count as c_int != (*((*blend).fvar)).axis_count as c_int
    { return 1; }

  if ((*gvar).glyph_count as sfnt_glyph_index) <= id
    { return 1; }

  if ((*gvar).flags & 1) != 0
    { offset = *(*gvar).u.offset_long.offset (id as isize); }
  else
    { offset = (*(*gvar).u.offset_word.offset (id as isize) as uint32_t) * 2u32; }

  if (offset as size_t) >= (*gvar).data_size
    { return 1; }

  end = (*gvar).glyph_variation_data.offset ((*gvar).data_size as isize);

  /* Start reading the header.  */

  if (offset as size_t) + std::mem::size_of::<sfnt_gvar_glyph_header> ()
      > (*gvar).data_size
    { return 1; }

  /* Clear the distortion.  */
  (*distortion).origin = 0;
  (*distortion).advance = 0;

  memcpy (&raw mut header as *mut c_void,
	  (*gvar).glyph_variation_data.offset (offset as isize) as *const c_void,
	  std::mem::size_of::<sfnt_gvar_glyph_header> ());

  /* Swap the header.  */
  sfnt_swap16! (&raw mut header.tuple_count);
  sfnt_swap16! (&raw mut header.data_offset);

  /* Prepare to read each tuple.  */
  ntuples = (header.tuple_count & 0x0fff) as c_int;

  /* Initialize the data offset.  This is incremented with each tuple
     read.  */
  data_offset = header.data_offset as ptrdiff_t;

  /* If gvar->flags & tuples_share_point_numbers, read the shared
     point numbers.  */

  npoints = 0;

  if (header.tuple_count & 0x8000) != 0
    {
      data = (*gvar).glyph_variation_data
	.offset ((offset as isize) + data_offset);
      points = sfnt_read_packed_points (data, &raw mut npoints, end,
					&raw mut tuple);

      if points.is_null ()
	{ return 1; }

      /* Shared point numbers are part of the data after the tuple
	 array.  Thus, increment data_offset by tuple - data.  `tuple'
	 here holds no relation to a pointer to the current part of
	 the tuple array that is being read later on.  */
      data_offset += (tuple as isize) - (data as isize);
    }
  else
    { points = std::ptr::null_mut (); }

  /* Start reading each tuple.  */
  tuple = (*gvar).glyph_variation_data
    .offset ((offset as isize) + (std::mem::size_of::<sfnt_gvar_glyph_header> () as isize));

  let mut coords_alloc: *mut c_void = std::ptr::null_mut ();
  if ((*gvar).axis_count as size_t)
      < 1024 * 16 / (3 * std::mem::size_of::<sfnt_f2dot14> ())
    {
      coords = sa_stack_alloc ((*gvar).axis_count as size_t
			       * std::mem::size_of::<sfnt_f2dot14> () * 3)
	as *mut sfnt_f2dot14;
    }
  else
    {
      coords_alloc = xnmalloc ((*gvar).axis_count as isize,
			       (3 * std::mem::size_of::<sfnt_f2dot14> ()) as isize);
      coords = coords_alloc as *mut sfnt_f2dot14;
    }

  intermediate_start = coords.offset ((*gvar).axis_count as isize);
  intermediate_end = intermediate_start.offset ((*gvar).axis_count as isize);

  'fail1: {
    'fail3: {
      while { ntuples = ntuples.wrapping_sub (1); ntuples != -1 }
	{
	  data = (*gvar).glyph_variation_data
	    .offset ((offset as isize) + data_offset);

	  if tuple.offset (3) >= end
	    { break 'fail1; }

	  memcpy (&raw mut data_size as *mut c_void,
		  tuple as *const c_void,
		  std::mem::size_of::<uint16_t> ());
	  tuple = tuple.offset (std::mem::size_of::<uint16_t> () as isize);
	  memcpy (&raw mut index as *mut c_void,
		  tuple as *const c_void,
		  std::mem::size_of::<uint16_t> ());
	  tuple = tuple.offset (std::mem::size_of::<uint16_t> () as isize);
	  sfnt_swap16! (&raw mut data_size);
	  sfnt_swap16! (&raw mut index);

	  /* Increment the offset to the data by the data size specified
	     here.  */
	  data_offset += data_size as ptrdiff_t;

	  if (index & 0x8000) != 0
	    {
	      /* Embedded coordinates are present.  Read each
		 coordinate and add it to the tuple.  */
	      j = 0;
	      while j < (*gvar).axis_count as c_int
		{
		  if tuple.offset (1) >= end
		    { break 'fail1; }

		  memcpy (coords.offset (j as isize) as *mut c_void,
			  tuple as *const c_void,
			  std::mem::size_of::<sfnt_f2dot14> ());
		  tuple = tuple.offset (std::mem::size_of::<sfnt_f2dot14> () as isize);
		  sfnt_swap16! (coords.offset (j as isize));
		  j += 1;
		}
	    }
	  else if (*gvar).shared_coord_count <= (index & 0x0fff)
	    /* The index is too large.  */
	    { break 'fail1; }
	  else
	    /* index points into gvar->axis_count coordinates making up
	       the tuple.  */
	    {
	      memcpy (coords as *mut c_void,
		      (*gvar).global_coords.offset (
			(((index & 0x0fff) as isize)
			 * ((*gvar).axis_count as isize))) as *const c_void,
		      ((*gvar).axis_count as usize)
		      * std::mem::size_of::<sfnt_f2dot14> ());
	    }

	  /* Now read indeterminate tuples if required.  */
	  if (index & 0x4000) != 0
	    {
	      j = 0;
	      while j < (*gvar).axis_count as c_int
		{
		  if tuple.offset (1) >= end
		    { break 'fail1; }

		  memcpy (intermediate_start.offset (j as isize) as *mut c_void,
			  tuple as *const c_void,
			  std::mem::size_of::<sfnt_f2dot14> ());
		  tuple = tuple.offset (std::mem::size_of::<sfnt_f2dot14> () as isize);
		  sfnt_swap16! (intermediate_start.offset (j as isize));
		  j += 1;
		}

	      j = 0;
	      while j < (*gvar).axis_count as c_int
		{
		  if tuple.offset (1) >= end
		    { break 'fail1; }

		  memcpy (intermediate_end.offset (j as isize) as *mut c_void,
			  tuple as *const c_void,
			  std::mem::size_of::<sfnt_f2dot14> ());
		  tuple = tuple.offset (std::mem::size_of::<sfnt_f2dot14> () as isize);
		  sfnt_swap16! (intermediate_end.offset (j as isize));
		  j += 1;
		}
	    }

	  /* See whether or not the tuple applies to the current variation
	     configuration, and how much to scale them by.  */

	  scale = sfnt_compute_tuple_scale (blend, (index & 0x4000) != 0,
					    coords, intermediate_start,
					    intermediate_end);

	  if scale == 0
	    { continue; }

	  local_points = std::ptr::null_mut ();

	  /* Finally, read private point numbers.
	     Set local_points to those numbers; it will be freed
	     once the loop ends.  */

	  if (index & 0x2000) != 0
	    {
	      local_points = sfnt_read_packed_points (data, &raw mut n_local_points,
						      end, &raw mut data);
	      if local_points.is_null ()
		{ break 'fail1; }

	      point_count = n_local_points as c_int;
	      glyph_points = local_points;
	    }
	  else
	    {
	      /* If there are no private point numbers, use global
		 points.  */
	      point_count = npoints as c_int;
	      glyph_points = points;
	    }

	  /* Now, read packed deltas.  */

	  dx = std::ptr::null_mut ();
	  dy = std::ptr::null_mut ();

	  match point_count
	    {
	      UINT16_MAX =>
		{
		  /* Deltas are provided for all components in the glyph.  */

		  /* Add 4 phantom points to each end.  */
		  dx = sfnt_read_packed_deltas (
		    data, end,
		    ((*(*glyph).compound).num_components + 4) as c_int,
		    &raw mut data);
		  dy = sfnt_read_packed_deltas (
		    data, end,
		    ((*(*glyph).compound).num_components + 4) as c_int,
		    &raw mut data);

		  if dx.is_null () || dy.is_null ()
		    { break 'fail3; }

		  /* Apply each delta to the compound glyph.  */

		  i = 0;
		  while i < (*(*glyph).compound).num_components as c_int
		    {
		      component = (*(*glyph).compound).components.offset (i as isize);

		      /* Check if the component uses deltas at all.  */
		      if ((*component).flags & 0o2) == 0
			{ i += 1; continue; }

		      /* Vary the X offset.  */

		      if ((*component).flags & 0o1) == 0
			{ word = (*component).argument1.b as sfnt_fword; }
		      else
			{ word = (*component).argument1.d; }

		      fword = sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
						    scale) as sfnt_fword;
		      (*component).argument1.d
			= (word as c_int + fword as c_int) as int16_t;

		      /* Vary the Y offset.  */

		      if ((*component).flags & 0o1) == 0
			{ word = (*component).argument2.b as sfnt_fword; }
		      else
			{ word = (*component).argument2.d; }

		      fword = sfnt_mul_fixed_round (*dy.offset (i as isize) as sfnt_fixed,
						    scale) as sfnt_fword;

		      /* Set the flag that says offsets are words.  */
		      (*component).flags |= 0o1;
		      (*component).argument2.d
			= (word as c_int + fword as c_int) as int16_t;

		      i += 1;
		    }

		  /* Apply the deltas for the two phantom points.  */
		  (*distortion).origin
		    = ((*distortion).origin as c_int).wrapping_add (
			sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
					      scale)) as sfnt_fword;
		  i += 1;
		  (*distortion).advance
		    = ((*distortion).advance as c_int).wrapping_add (
			sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
					      scale)) as sfnt_fword;
		}

	      _ =>
		{
		  dx = sfnt_read_packed_deltas (data, end, point_count, &raw mut data);
		  dy = sfnt_read_packed_deltas (data, end, point_count, &raw mut data);

		  if dx.is_null () || dy.is_null ()
		    { break 'fail3; }

		  /* Deltas are only applied for each point number read.  */

		  i = 0;
		  while i < point_count
		    {
		      /* Apply deltas to phantom points.  */

		      if *glyph_points.offset (i as isize) as size_t
			  == (*(*glyph).compound).num_components
			{
			  (*distortion).origin
			    = ((*distortion).origin as c_int).wrapping_add (
				sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
						      scale)) as sfnt_fword;
			  i += 1;
			  continue;
			}

		      if *glyph_points.offset (i as isize) as size_t
			  == (*(*glyph).compound).num_components + 1
			{
			  (*distortion).advance
			    = ((*distortion).advance as c_int).wrapping_add (
				sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
						      scale)) as sfnt_fword;
			  i += 1;
			  continue;
			}

		      /* Make sure the point doesn't end up out of bounds.  */
		      if *glyph_points.offset (i as isize) as size_t
			  >= (*(*glyph).compound).num_components
			{ i += 1; continue; }

		      component = (*(*glyph).compound).components.offset (
			*glyph_points.offset (i as isize) as isize);

		      /* Check if the component uses deltas at all.  */
		      if ((*component).flags & 0o2) == 0
			{ i += 1; continue; }

		      /* Vary the X offset.  */

		      if ((*component).flags & 0o1) == 0
			{ word = (*component).argument1.b as sfnt_fword; }
		      else
			{ word = (*component).argument1.d; }

		      fword = sfnt_mul_fixed_round (*dx.offset (i as isize) as sfnt_fixed,
						    scale) as sfnt_fword;
		      (*component).argument1.d
			= (word as c_int + fword as c_int) as int16_t;

		      /* Vary the Y offset.  */

		      if ((*component).flags & 0o1) == 0
			{ word = (*component).argument2.b as sfnt_fword; }
		      else
			{ word = (*component).argument2.d; }

		      fword = sfnt_mul_fixed_round (*dy.offset (i as isize) as sfnt_fixed,
						    scale) as sfnt_fword;

		      /* Set the flag that says offsets are words.  */
		      (*component).flags |= 0o1;
		      (*component).argument2.d
			= (word as c_int + fword as c_int) as int16_t;

		      i += 1;
		    }
		}
	    }

	  xfree (dx as *mut c_void);
	  xfree (dy as *mut c_void);

	  if local_points != (usize::MAX as *mut uint16_t)
	    { xfree (local_points as *mut c_void); }
	}

      /* Return success.  */

      xfree (coords_alloc);
      if points != (usize::MAX as *mut uint16_t)
	{ xfree (points as *mut c_void); }

      /* Set the glyph metrics distortion as well.  */
      (*glyph).advance_distortion = (*distortion).advance;
      (*glyph).origin_distortion = (*distortion).origin;

      return 0;
    } /* 'fail3 (C 16201) */

    /* fail3: */
    xfree (dx as *mut c_void);
    xfree (dy as *mut c_void);
    xfree (local_points as *mut c_void);
  } /* 'fail1 (C 16205) */

  /* fail1: */

  xfree (coords_alloc);
  if points != (usize::MAX as *mut uint16_t)
    { xfree (points as *mut c_void); }

  return 1;
}

/* Vary the specified INTERPRETER's control value table using the
   variations in BLEND's CVT variations table, then record the blend's
   normalized coordinates and axis count in the interpreter.

   The CVT table used to create INTERPRETER must be the same used
   to read BLEND->cvar.  If not, behavior is undefined.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_vary_interpreter (interpreter: *mut sfnt_interpreter,
						blend: *mut sfnt_blend)
{                                                          // C 16221-16277
  let mut scale: sfnt_fixed;
  let mut i: c_int;
  let mut variation: *mut sfnt_tuple_variation;
  let mut ndeltas: size_t;
  let mut j: size_t;
  let mut index: size_t;
  let mut delta: sfnt_f26dot6;

  /* Return if there's no cvar table.  */
  if (*blend).cvar.is_null ()
    { return; }

  /* For each tuple in the cvar table... */
  i = 0;
  while i < ((*((*blend).cvar)).tuple_count & 0x0fff) as c_int
    {
      /* See if the tuple applies.  */
      variation = (*((*blend).cvar)).variation.offset (i as isize);
      scale = sfnt_compute_tuple_scale (blend,
					!(*variation).intermediate_start.is_null (),
					(*variation).coordinates,
					(*variation).intermediate_start,
					(*variation).intermediate_end);
      if scale == 0
	{ i += 1; continue; }

      /* Figure out how many deltas there are.  If variation->points,
	 there are num_points deltas.  Otherwise, there are
	 interpreter->cvt_size deltas.  */

      ndeltas = if !(*variation).points.is_null ()
	{ (*variation).num_points as size_t }
      else
	{ (*interpreter).cvt_size };

      j = 0;
      while j < ndeltas
	{
	  /* Figure out which CVT entry this applies to.  */
	  index = if !(*variation).points.is_null ()
	    { *(*variation).points.offset (j as isize) as size_t }
	  else
	    { j };

	  if index > (*interpreter).cvt_size
	    { j += 1; continue; }

	  /* Multiply the delta by the interpreter scale factor and
	     then the tuple scale factor.  */
	  delta = sfnt_mul_f26dot6_fixed (
	    *(*variation).deltas.offset (j as isize) as sfnt_f26dot6,
	    (*interpreter).scale);
	  delta = sfnt_mul_fixed_round (delta, scale);

	  /* Apply the delta to the control value table.  */
	  *(*interpreter).cvt.offset (i as isize)
	    = (*(*interpreter).cvt.offset (i as isize)).wrapping_add (delta);

	  j += 1;
	}

      i += 1;
    }

  (*interpreter).n_axis = (*((*blend).fvar)).axis_count as c_int;
  (*interpreter).norm_coords = (*blend).norm_coords;
}

/* OS/2 metadata retrieval.

   A font's `OS/2' table incorporates some miscellaneous information
   that is consulted by the font scaler on MS-Windows.  Emacs requires
   one fragment of this information: the font foundry name.  */

/* Read an OS/2 table from the given font FD.  Use the table directory
   provided in SUBTABLE.

   Return the OS/2 table if successful, NULL otherwise.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_OS_2_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_OS_2_table                                  // C 16292-16381
{
  let mut OS_2: *mut sfnt_OS_2_table;
  let mut directory: *mut sfnt_table_directory;
  let mut rc: ssize_t;
  let mut minimum: size_t;
  let mut wanted: size_t;

  /* Search for the OS/2 table within SUBTABLE.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_OS_2);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Calculate how large the table must be.  The field `panose' is the
     last field aligned to natural boundaries, and thus contents must
     be read twice: once to populate the table with information up to
     `panose', and once again to retrieve the information
     afterwards.  */

  minimum = (SFNT_ENDOF! (sfnt_OS_2_table, panose, [u8; 10])
	     + SFNT_ENDOF! (sfnt_OS_2_table, fs_last_char_index, uint16_t)
	     - std::mem::offset_of! (sfnt_OS_2_table, ul_unicode_range));

  /* If the table is too short, return.  */
  if ((*directory).length as size_t) < minimum
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  OS_2 = xmalloc (std::mem::size_of::<sfnt_OS_2_table> ()) as *mut sfnt_OS_2_table;

  /* Read data up to the end of `panose'.  */

  wanted = SFNT_ENDOF! (sfnt_OS_2_table, panose, [u8; 10]);
  rc = read (fd, OS_2 as *mut c_void, wanted);

  if rc == -1 || rc != wanted as ssize_t
    {
      xfree (OS_2 as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Byte swap that data.  */

  sfnt_swap16! (&raw mut (*OS_2).version);
  sfnt_swap16! (&raw mut (*OS_2).x_avg_char_width);
  sfnt_swap16! (&raw mut (*OS_2).us_weight_class);
  sfnt_swap16! (&raw mut (*OS_2).us_width_class);
  sfnt_swap16! (&raw mut (*OS_2).fs_type);
  sfnt_swap16! (&raw mut (*OS_2).y_subscript_x_size);
  sfnt_swap16! (&raw mut (*OS_2).y_subscript_y_size);
  sfnt_swap16! (&raw mut (*OS_2).y_subscript_x_offset);
  sfnt_swap16! (&raw mut (*OS_2).y_subscript_y_offset);
  sfnt_swap16! (&raw mut (*OS_2).y_superscript_x_size);
  sfnt_swap16! (&raw mut (*OS_2).y_superscript_y_size);
  sfnt_swap16! (&raw mut (*OS_2).y_superscript_x_offset);
  sfnt_swap16! (&raw mut (*OS_2).y_superscript_y_offset);
  sfnt_swap16! (&raw mut (*OS_2).y_strikeout_size);
  sfnt_swap16! (&raw mut (*OS_2).y_strikeout_position);
  sfnt_swap16! (&raw mut (*OS_2).s_family_class);

  /* Read fields between ul_unicode_range and fs_last_char_index.  */
  wanted = (SFNT_ENDOF! (sfnt_OS_2_table, fs_last_char_index, uint16_t)
	    - std::mem::offset_of! (sfnt_OS_2_table, ul_unicode_range));
  rc = read (fd, (&raw mut (*OS_2).ul_unicode_range) as *mut c_void, wanted);

  if rc == -1 || rc != wanted as ssize_t
    {
      xfree (OS_2 as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Swap the remainder and return the table.  */
  sfnt_swap32! (&raw mut (*OS_2).ul_unicode_range[0]);
  sfnt_swap32! (&raw mut (*OS_2).ul_unicode_range[1]);
  sfnt_swap32! (&raw mut (*OS_2).ul_unicode_range[2]);
  sfnt_swap32! (&raw mut (*OS_2).ul_unicode_range[3]);
  sfnt_swap16! (&raw mut (*OS_2).fs_selection);
  sfnt_swap16! (&raw mut (*OS_2).fs_first_char_index);
  sfnt_swap16! (&raw mut (*OS_2).fs_last_char_index);
  return OS_2;
}

/* C 16383's form feed is dropped.  */

/* PostScript metadata retrieval.

   TrueType fonts electively incorporate a table of miscellaneous
   information concerning such matters as the underline position or
   whether the font is fixed pitch.  This table also assigns
   human-readable names to glyphs, subject to the table format, but
   these names are not read by the functions defined below.  */

/* Read the header of a post table from the given font FD.  Refer to
   the table directory SUBTABLE for its location.

   Return the post table header if successful, NULL otherwise.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_read_post_table (fd: c_int,
					       subtable: *mut sfnt_offset_subtable)
  -> *mut sfnt_post_table                                  // C 16398-16444
{
  let mut post: *mut sfnt_post_table;
  let mut directory: *mut sfnt_table_directory;
  let mut rc: ssize_t;

  /* Search for the post table within SUBTABLE.  */

  directory = sfnt_find_table (subtable, SFNT_TABLE_POST);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  /* Although the size of the table is affected by its format, this
     function is meant to read only its header; guarantee that the
     directory is that large.  */

  if ((*directory).length as size_t) < std::mem::size_of::<sfnt_post_table> ()
    { return std::ptr::null_mut (); }

  /* Seek to the location given in the directory.  */
  if lseek (fd, (*directory).offset as off_t, SEEK_SET) == (-1 as off_t)
    { return std::ptr::null_mut (); }

  post = xmalloc (std::mem::size_of::<sfnt_post_table> ()) as *mut sfnt_post_table;
  rc = read (fd, post as *mut c_void, std::mem::size_of::<sfnt_post_table> ());

  if rc == -1 || rc != std::mem::size_of::<sfnt_post_table> () as ssize_t
    {
      xfree (post as *mut c_void);
      return std::ptr::null_mut ();
    }

  /* Byte swap the data retrieved.  */
  sfnt_swap32! (&raw mut (*post).format);
  sfnt_swap32! (&raw mut (*post).italic_angle);
  sfnt_swap16! (&raw mut (*post).underline_position);
  sfnt_swap16! (&raw mut (*post).underline_thickness);
  sfnt_swap32! (&raw mut (*post).is_fixed_pitch);
  sfnt_swap32! (&raw mut (*post).min_mem_type_42);
  sfnt_swap32! (&raw mut (*post).max_mem_type_42);
  sfnt_swap32! (&raw mut (*post).min_mem_type_1);
  sfnt_swap32! (&raw mut (*post).max_mem_type_1);

  return post;
}

/* ----------------------------------------------------------------------------------------------
   C 10664-11101: the interpreter's shift and interpolate instructions, in C's order - C 10665-10667's
   comment with `sfnt_interpret_shz` (C 10669-10699), C 10701-10703's comment with
   `sfnt_interpret_shc` (C 10705-10780), C 10782-10788's comment with `sfnt_interpret_shp`
   (C 10790-10825), C 10827-10845's four point-access macros with C 10847-10947's
   `IUP_SINGLE_PAIR ()`, and C 10949-10952's comment with `sfnt_interpret_iup_1` (C 10955-11046)
   and `sfnt_interpret_iup` (C 11058-11101).  **The next live definition is C 11113's
   `sfnt_interpret_mirp`**, and the ledger's `## C line ranges` names both boundaries.

   C's four macros and `IUP_SINGLE_PAIR ()` are in-function `macro_rules!`s (P-57): their bodies name
   this function's `opcode` and its locals, and a `macro_rules!` resolves those at its definition
   site, which is why C's `#undef`s (C 11048-11051) are the same statement.  `TRAP` and `POP` are
   the same shape, each copied into the function that uses it (`sfnt_interpret_flippt`'s reading).
   ---------------------------------------------------------------------------------------------- */
/// C 10669-10699: `sfnt_interpret_shz` - C's `static`, called by name only.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_shz (interpreter: *mut sfnt_interpreter,
			      zone: uint32_t, opcode: c_uint)
{                                                          // C 10669-10699
  /* C's four out-parameters of `sfnt_address_zp0`/`zp1` (their addresses are taken and C reads
     them after the call); zero-initialised so Rust's analysis sees what C's flow guarantees
     (PORTING.md 1.5, Deviation 10).  */
  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut original_x: sfnt_f26dot6 = 0;
  let mut original_y: sfnt_f26dot6 = 0;
  let mut magnitude: sfnt_f26dot6;

  if zone != 0 && (*interpreter).glyph_zone.is_null ()        // C 10676
    /* There are no points in the glyph zone.  */
    { return; }                                            // C 10678

  if opcode == 0x37                                        // C 10680
    { sfnt_address_zp0 (interpreter, (*interpreter).state.rp1 as uint32_t,
			&raw mut x, &raw mut y, &raw mut original_x, &raw mut original_y); }
  else
    { sfnt_address_zp1 (interpreter, (*interpreter).state.rp2 as uint32_t,
			&raw mut x, &raw mut y, &raw mut original_x, &raw mut original_y); }   // C 10684-10685

  magnitude = sfnt_project_vector (interpreter,
				   sfnt_sub! (x, original_x),
				   sfnt_sub! (y, original_y));   // C 10687-10689

  if zone == 0                                             // C 10691
    { sfnt_move_twilight_zone (interpreter, 0,
			       (*interpreter).twilight_zone_size as size_t,
			       magnitude); }               // C 10692-10694
  else
    { sfnt_move_glyph_zone (interpreter, 0,
			    (*(*interpreter).glyph_zone).num_points,
			    magnitude); }                  // C 10696-10698
}

/// C 10705-10780: `sfnt_interpret_shc` - C's `static`, called by name only.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_shc (interpreter: *mut sfnt_interpreter,
			      contour: uint32_t, opcode: c_uint)
{                                                          // C 10705-10780
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  /* C's four out-parameters of `sfnt_address_zp0`/`zp1` (their addresses are taken and C reads
     them after the call); zero-initialised so Rust's analysis sees what C's flow guarantees
     (PORTING.md 1.5, Deviation 10).  */
  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut original_x: sfnt_f26dot6 = 0;
  let mut original_y: sfnt_f26dot6 = 0;
  let mut magnitude: sfnt_f26dot6;
  let mut reference_point: uint16_t;
  let mut start: size_t;
  let mut end: size_t;
  let mut start1: size_t;
  let mut end1: size_t;
  let mut n: size_t;

  if (*interpreter).glyph_zone.is_null ()                  // C 10714
    { TRAP! (c"SHC without glyph zone".as_ptr ()); }       // C 10715

  /* Check that the contour is within bounds.  */
  if contour as usize >= (*(*interpreter).glyph_zone).num_contours   // C 10718
    { TRAP! (c"contour out of bounds".as_ptr ()); }        // C 10719

  /* Figure out the magnitude of the change, measured from the
     projection vector.  */

  if opcode == 0x35                                        // C 10724
    { sfnt_address_zp0 (interpreter,
			{ reference_point = (*interpreter).state.rp1; reference_point } as uint32_t,
			&raw mut x, &raw mut y, &raw mut original_x, &raw mut original_y); }
  else
    { sfnt_address_zp1 (interpreter,
			{ reference_point = (*interpreter).state.rp2; reference_point } as uint32_t,
			&raw mut x, &raw mut y, &raw mut original_x, &raw mut original_y); }   // C 10729-10731

  magnitude = sfnt_project_vector (interpreter,
				   sfnt_sub! (x, original_x),
				   sfnt_sub! (y, original_y));   // C 10733-10735

  /* Now obtain the start and end of the contour.
     Verify that both are valid.  */

  if contour != 0                                          // C 10740
    { start = *((*(*interpreter).glyph_zone).contour_end_points)
	.offset (contour as isize - 1) + 1; }              // C 10741
  else
    { start = 0; }                                         // C 10743

  end = *((*(*interpreter).glyph_zone).contour_end_points).offset (contour as isize);   // C 10745

  if start > end || end >= (*(*interpreter).glyph_zone).num_points   // C 10747
    { TRAP! (c"invalid contour data in glyph".as_ptr ()); }   // C 10748

  /* If the reference point falls between end and start, split the
     range formed by end and start at the reference point and keep the
     latter intact.  */

  if start <= reference_point as usize && reference_point as usize <= end   // C 10754
    {
      /* Do the points between start and rpN.  */
      start1 = start;                                      // C 10757
      end1 = reference_point as usize - 1;                 // C 10758

      if start1 <= end1                                    // C 10760
	{ sfnt_move_glyph_zone (interpreter, start1 as uint32_t,
				end1 - start1 + 1, magnitude); }   // C 10761-10762

      /* Now the points between rpN + 1 and end.  */
      start1 = reference_point as usize + 1;               // C 10765
      end1 = end;                                          // C 10766

      if start1 <= end1                                    // C 10768
	{ sfnt_move_glyph_zone (interpreter, start1 as uint32_t,
				end1 - start1 + 1, magnitude); }   // C 10769-10770

      return;                                              // C 10772
    }

  /* Compute the number of points to move.  */
  n = end - start + 1;                                     // C 10776

  /* Move that many points.  */
  sfnt_move_glyph_zone (interpreter, start as uint32_t, n, magnitude);   // C 10779
}

/// C 10790-10825: `sfnt_interpret_shp` - C's `static`, called by name only.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_shp (interpreter: *mut sfnt_interpreter, opcode: c_uint)
{                                                          // C 10790-10825
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }
  /* C 6597-6601's `#define POP()` - the live `#ifndef TEST` arm; the `#else` arm's body names
     the TEST-only `pop_hook` member (see this file's header).  */
  macro_rules! POP { () => {
    if (*interpreter).SP == (*interpreter).stack
      { TRAP! (c"stack underflow".as_ptr ()); 0 }
    else
      { (*interpreter).SP = (*interpreter).SP.offset (-1); *(*interpreter).SP }
  } }

  /* C's four out-parameters of `sfnt_address_zp0`/`zp1` (their addresses are taken and C reads
     them after the call); zero-initialised so Rust's analysis sees what C's flow guarantees
     (PORTING.md 1.5, Deviation 10).  */
  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut original_x: sfnt_f26dot6 = 0;
  let mut original_y: sfnt_f26dot6 = 0;
  let mut magnitude: sfnt_f26dot6;
  let mut point: uint32_t;

  /* Figure out the magnitude of the change, measured from the
     projection vector.  */

  if opcode == 0x33                                        // C 10801
    { sfnt_address_zp0 (interpreter, (*interpreter).state.rp1 as uint32_t,
			&raw mut x, &raw mut y, &raw mut original_x, &raw mut original_y); }
  else
    { sfnt_address_zp1 (interpreter, (*interpreter).state.rp2 as uint32_t,
			&raw mut x, &raw mut y, &raw mut original_x, &raw mut original_y); }   // C 10805-10806

  magnitude = sfnt_project_vector (interpreter,
				   sfnt_sub! (x, original_x),
				   sfnt_sub! (y, original_y));   // C 10808-10810

  /* Now project it onto the freedom vector and move the point that
     much for loop variable times.  */

  while { (*interpreter).state.r#loop = (*interpreter).state.r#loop.wrapping_sub (1);
	  (*interpreter).state.r#loop != 0xffff }          // C 10815
    {
      point = POP! ();                                     // C 10817

      sfnt_check_zp2 (interpreter, point);                 // C 10819
      sfnt_move_zp2 (interpreter, point, 1, magnitude);    // C 10820
    }

  /* Restore interpreter->state.loop to 1.  */
  (*interpreter).state.r#loop = 1;                         // C 10824
}

/// C 10955-11046: `sfnt_interpret_iup_1` - C's `static`, called by name only.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_iup_1 (interpreter: *mut sfnt_interpreter,
				start: size_t, end: size_t,
				opcode: c_uchar, mask: c_int)
{                                                          // C 10955-11046
  let mut point: size_t;
  let mut touch_start: size_t;
  let mut touch_end: size_t;
  let mut first_point: size_t;
  let mut point_min: size_t;
  let mut point_max: size_t;
  let mut i: size_t;
  let mut position: sfnt_f26dot6;
  let mut min_pos: sfnt_f26dot6;
  let mut max_pos: sfnt_f26dot6;
  let mut delta: sfnt_f26dot6;
  let mut ratio: sfnt_f26dot6;
  let mut original_max_pos: sfnt_f26dot6;
  let mut org_max_pos: sfnt_f26dot6;
  let mut original_min_pos: sfnt_f26dot6;
  let mut org_min_pos: sfnt_f26dot6;

  /* C 10827-10845's four point-access macros: each body selects on this function's `opcode` and
     reads this zone, so each is a `macro_rules!` here (P-57, the `TRAP` shape).  `load_unscaled`
     reads the simple glyph's `int16_t` coordinates, which C promotes to `sfnt_f26dot6` - the
     explicit `as` is that promotion.  */
  macro_rules! load_point { ($p:expr) => {
    if opcode == 0x31
      { *((*(*interpreter).glyph_zone).x_current).offset ($p as isize) }
    else
      { *((*(*interpreter).glyph_zone).y_current).offset ($p as isize) }
  } }
  macro_rules! store_point { ($p:expr, $val:expr) => {
    if opcode == 0x31
      { *((*(*interpreter).glyph_zone).x_current).offset ($p as isize) = $val; }
    else
      { *((*(*interpreter).glyph_zone).y_current).offset ($p as isize) = $val; }
  } }
  macro_rules! load_original { ($p:expr) => {
    if opcode == 0x31
      { *((*(*interpreter).glyph_zone).x_points).offset ($p as isize) }
    else
      { *((*(*interpreter).glyph_zone).y_points).offset ($p as isize) }
  } }
  macro_rules! load_unscaled { ($p:expr) => {
    if opcode == 0x31
      { *((*(*(*interpreter).glyph_zone).simple).x_coordinates).offset ($p as isize)
	as sfnt_f26dot6 }
    else
      { *((*(*(*interpreter).glyph_zone).simple).y_coordinates).offset ($p as isize)
	as sfnt_f26dot6 }
  } }
  /* C 10847-10947's `IUP_SINGLE_PAIR ()` - the interpolation itself, used at C 11023 and
     C 11040.  */
  macro_rules! IUP_SINGLE_PAIR { () => {
    /* Now make touch_start the first point before, i.e. the first
       touched point in this pair.  */

    if touch_start == start
      { touch_start = end; }
    else
      { touch_start = touch_start - 1; }

    /* Set point_min and point_max based on which glyph is at a
       lower value.  */

    if load_original! (touch_start) < load_original! (touch_end)
      {
	point_min = touch_start;
	point_max = touch_end;
      }
    else
      {
	point_max = touch_start;
	point_min = touch_end;
      }

    min_pos = load_point! (point_min);
    max_pos = load_point! (point_max);

    /* This is needed for interpolation.  */
    original_max_pos = load_original! (point_max);
    original_min_pos = load_original! (point_min);

    /* Now process points between touch_start and touch_end.  */

    i = touch_start + 1;

    /* touch_start might be the last point in the contour.  */

    if i > end
      { i = start; }

    while i != touch_end
      {
	/* Movement is always relative to the original position of
	   the point.  */
	position = load_original! (i);

	/* If i is in between touch_start and touch_end...  */
	if position >= original_min_pos
	   && position <= original_max_pos
	  {
	    /* Compute the ratio between the two touched point positions
	       and the original position of the point being touched with
	       positions from the unscaled outline, if at all
	       possible.  */

	    if !(*(*interpreter).glyph_zone).simple.is_null ()
	      {
		org_max_pos = load_unscaled! (point_max);
		org_min_pos = load_unscaled! (point_min);
		position = load_unscaled! (i);
	      }
	    else
	      {
		org_max_pos = original_max_pos;
		org_min_pos = original_min_pos;
	      }

	    /* Handle the degenerate case where original_min_pos and
	       original_max_pos have not changed by placing the point in
	       the middle.  */
	    if org_min_pos == org_max_pos
	      { ratio = 0o77777; }
	    else
	      /* ... preserve the ratio of i between min_pos and
		 max_pos...  */
	      { ratio = sfnt_div_fixed (sfnt_sub! (position, org_min_pos) * 1024,
					sfnt_sub! (org_max_pos, org_min_pos) * 1024); }

	    delta = sfnt_sub! (max_pos, min_pos);
	    delta = sfnt_mul_fixed_round (ratio, delta);
	    store_point! (i, sfnt_add! (min_pos, delta));
	  }
	else
	  {
	    /* ... otherwise, move i by how much the nearest touched
	       point moved.  */

	    if position >= original_max_pos
	      { delta = sfnt_sub! (max_pos, original_max_pos); }
	    else
	      { delta = sfnt_sub! (min_pos, original_min_pos); }

	    store_point! (i, sfnt_add! (position, delta));
	  }

	i += 1;
	if i > end
	  { i = start; }
      }
  } }

  /* Find the first touched point.  If none is found, simply
     return.  */

  /* C 10978's `touched:` is entered by C 10973's `goto`; C 11028's `wraparound:` and C 11035's
     `wraparound_1:` are entered by C 11002 and C 11016, and C 11035 falls out of C 11033's
     `wraparound:` body.  Three blocks carry them: `'skip33` encloses the body and C 11033's
     statement, so C 11016's jump (`break 'skip33`) skips it and lands on C 11039; `'body` ends at
     C 11033, so C 11002's jump (`break 'body`) runs it; `'untouched` encloses all of it and is
     where C 10989/11026's `goto untouched` goes.  */
  'untouched: {
   'skip33: {
    'body: {
      point = start;                                       // C 10980
      while !((*((*(*interpreter).glyph_zone).flags).offset (point as isize) as c_int) & mask != 0)
	{
	  point += 1;                                      // C 10985

	  /* There are no touched points.  */
	  if point > end                                   // C 10988
	    { break 'untouched; }                          // C 10989
	}

      first_point = point;                                 // C 10992

      while point <= end                                   // C 10994
	{
	  /* Find the next untouched point.  */
	  while ((*((*(*interpreter).glyph_zone).flags).offset (point as isize) as c_int) & mask) != 0
	    {
	      point += 1;                                  // C 10999

	      if point > end                               // C 11001
		{ break 'body; }                           // C 11002: `goto wraparound` (C 11033)
	    }

	  /* touch_start is now the first untouched point.  */
	  touch_start = point;                             // C 11006

	  /* Find the next touched point.  */
	  while !(((*((*(*interpreter).glyph_zone).flags).offset (point as isize) as c_int) & mask) != 0)
	    {
	      point += 1;                                  // C 11011

	      /* Move back to start if point has gone past the end of the
		 contour.  */
	      if point > end                               // C 11015
		{ break 'skip33; }                         // C 11016: `goto wraparound_1` (C 11039)
	    }

	  /* touch_end is now the next touched point.  */
	  touch_end = point;                               // C 11020

	  /* Do the interpolation.  */
	  IUP_SINGLE_PAIR! ();                             // C 11023
	}

      break 'untouched;                                    // C 11026
    }

    /* C 11028: `wraparound:`.  */
    /* This is like wraparound_1, except that no untouched points have
       yet to be found.

       This means the first untouched point is start.  */
    touch_start = start;                                   // C 11033
   }

   /* C 11035: `wraparound_1:` - reached by the fall-through from C 11033 and by C 11016's jump.  */
   /* If point > end, wrap around.  Here, touch_start is set
      properly, so touch_end must be first_point.  */

   touch_end = first_point;                                // C 11039
   IUP_SINGLE_PAIR! ();                                    // C 11040
  }
  /* C 11042: `untouched:` - the function returns immediately.  */
  return;                                                  // C 11045
}

/// C 11058-11101: `sfnt_interpret_iup` - C's `static`, called by name only.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_iup (interpreter: *mut sfnt_interpreter, opcode: c_uchar)
{                                                          // C 11058-11101
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }

  let mut mask: c_int;
  let mut i: size_t;
  let mut point: size_t;
  let mut end: size_t;
  let mut first_point: size_t;

  /* Check that the zone is the glyph zone.  */

  if (*interpreter).state.zp2 == 0                         // C 11066
    { TRAP! (c"trying to iup in twilight zone".as_ptr ()); }   // C 11067

  if (*interpreter).glyph_zone.is_null ()                  // C 11069
    { TRAP! (c"iup without loaded glyph!".as_ptr ()); }    // C 11070

  /* Figure out what axis to interpolate in based on the opcode.  */
  if opcode == 0x30                                        // C 11073
    { mask = SFNT_POINT_TOUCHED_Y; }
  else
    { mask = SFNT_POINT_TOUCHED_X; }                       // C 11076

  /* Now, for each contour, interpolate untouched points.  */
  point = 0;                                               // C 11079
  i = 0;                                                   // C 11080
  while i < (*(*interpreter).glyph_zone).num_contours
    {
      first_point = point;                                 // C 11082
      end = *((*(*interpreter).glyph_zone).contour_end_points).offset (i as isize);   // C 11083

      if point >= (*(*interpreter).glyph_zone).num_points
	 || end >= (*(*interpreter).glyph_zone).num_points   // C 11085-11086
	{ TRAP! (c"glyph contains out of bounds contour end point data!".as_ptr ()); }   // C 11087-11088

      sfnt_interpret_iup_1 (interpreter, first_point, end,
			    opcode, mask);                 // C 11090-11091
      point = end + 1;                                     // C 11092

      /* Skip the subsequent phantom points, which may end up
	 intermixed with contours inside a compound glyph.  */

      while point < (*(*interpreter).glyph_zone).num_points
	 && (((*((*(*interpreter).glyph_zone).flags).offset (point as isize) as c_int)
	      & SFNT_POINT_PHANTOM) != 0)
	{ point += 1; }                                    // C 11099
      i += 1;
    }
}

/* ----------------------------------------------------------------------------------------------
   C 11103-11395: the reference-point instructions, in C's order - C 11103-11110's comment with
   `sfnt_interpret_mirp` (C 11113-11235), C 11237-11255's comment with `sfnt_project_zp1_zp0_org`
   (C 11257-11293, whose one `goto project_normally` is a labelled block) and C 11295-11302's
   comment with `sfnt_interpret_mdrp` (C 11305-11382).  **The next live definition is C 11396's
   `sfnt_interpret_run`** (the comment is C 11384-11394), and the ledger's `## C line ranges` names
   both boundaries.  `TRAP`, `POP`, `PROJECT` and `DUAL_PROJECT` are in-function copies (P-57), and
   every out-parameter of `sfnt_address_zp0`/`zp1` is zero-initialised (Deviation 10's class).
   ---------------------------------------------------------------------------------------------- */
/// C 11113-11235: `sfnt_interpret_mirp` - C's `static`, called by name only.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_mirp (interpreter: *mut sfnt_interpreter, opcode: uint32_t)
{                                                          // C 11113-11235
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }
  /* C 6597-6601's `#define POP()` - the live `#ifndef TEST` arm; the `#else` arm's body names
     the TEST-only `pop_hook` member (see this file's header).  */
  macro_rules! POP { () => {
    if (*interpreter).SP == (*interpreter).stack
      { TRAP! (c"stack underflow".as_ptr ()); 0 }
    else
      { (*interpreter).SP = (*interpreter).SP.offset (-1); *(*interpreter).SP }
  } }
  macro_rules! PROJECT { ($x:expr, $y:expr) => {
    sfnt_project_vector (interpreter, $x, $y)
  } }
  macro_rules! DUAL_PROJECT { ($x:expr, $y:expr) => {
    sfnt_dual_project_vector (interpreter, $x, $y)
  } }

  let mut n: uint32_t;
  let mut p: uint32_t;
  let mut distance: sfnt_f26dot6;
  let mut delta: sfnt_f26dot6;
  let mut temp: sfnt_f26dot6;
  let mut current_projection: sfnt_f26dot6;
  let mut original_projection: sfnt_f26dot6;
  /* The eight out-parameters of `sfnt_address_zp0` (C 11155) and `sfnt_address_zp1` (C 11181);
     both calls take their addresses and C reads them afterwards.  */
  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut org_x: sfnt_f26dot6 = 0;
  let mut org_y: sfnt_f26dot6 = 0;
  let mut rx: sfnt_f26dot6 = 0;
  let mut ry: sfnt_f26dot6 = 0;
  let mut org_rx: sfnt_f26dot6 = 0;
  let mut org_ry: sfnt_f26dot6 = 0;

  /* CVT index.  */
  n = POP! ();                                             // C 11124

  /* Point number.  */
  p = POP! ();                                             // C 11127

  /* Now get the distance from the CVT.  */
  if n as usize >= (*interpreter).cvt_size                 // C 11130
    { TRAP! (c"cvt index out of bounds".as_ptr ()); }      // C 11131

  distance = *(*interpreter).cvt.offset (n as isize);      // C 11133

  /* Test against the single width value.  */

  delta = sfnt_sub! (distance,
		     (*interpreter).state.single_width_value);   // C 11137-11138

  if delta < 0                                             // C 11140
    { delta = -delta; }                                    // C 11141

  if delta < (*interpreter).state.sw_cut_in                // C 11143
    {
      /* Use the single width instead, as the CVT entry is too
	 small.  */

      if distance >= 0                                     // C 11148
	{ distance = (*interpreter).state.single_width_value; }   // C 11149
      else
	{ distance = -(*interpreter).state.single_width_value; }   // C 11151
    }

  /* Load the reference point.  */
  sfnt_address_zp0 (interpreter, (*interpreter).state.rp0 as uint32_t,
		    &raw mut rx, &raw mut ry, &raw mut org_rx, &raw mut org_ry);   // C 11155-11156

  /* Create the point in the twilight zone, should that be ZP1.  */

  if (*interpreter).state.zp1 == 0                         // C 11160
    {
      /* Since P hasn't been loaded yet, whether or not it is valid is
	 not known.  */
      sfnt_check_zp1 (interpreter, p);                     // C 11164

      *(*interpreter).twilight_x.offset (p as isize) = rx;   // C 11166
      *(*interpreter).twilight_y.offset (p as isize) = ry;   // C 11167

      temp = sfnt_mul_f2dot14 ((*interpreter).state.projection_vector.x,
			       distance);                  // C 11169-11170
      temp = sfnt_add! (temp, org_rx);                     // C 11171
      *(*interpreter).twilight_original_x.offset (p as isize) = temp;   // C 11172

      temp = sfnt_mul_f2dot14 ((*interpreter).state.projection_vector.y,
			       distance);                  // C 11174-11175
      temp = sfnt_add! (temp, org_ry);                     // C 11176
      *(*interpreter).twilight_original_y.offset (p as isize) = temp;   // C 11177
    }

  /* Load P.  */
  sfnt_address_zp1 (interpreter, p, &raw mut x, &raw mut y,
		    &raw mut org_x, &raw mut org_y);       // C 11181

  /* If distance would be negative and auto_flip is on, flip it.  */

  original_projection = DUAL_PROJECT! (org_x - org_rx,
				       org_y - org_ry);    // C 11185-11186
  current_projection = PROJECT! (x - rx, y - ry);          // C 11187

  if (*interpreter).state.auto_flip                         // C 11189
    {
      if (original_projection ^ distance) < 0              // C 11191
	{ distance = -distance; }                          // C 11192
    }

  /* Flag B means look at the cvt cut in and round the
     distance.  */

  if (opcode & 4) != 0                                     // C 11198
    {
      delta = sfnt_sub! (distance, original_projection);   // C 11200

      if delta < 0                                         // C 11202
	{ delta = -delta; }                                // C 11203

      if delta > (*interpreter).state.cvt_cut_in           // C 11205
	{ distance = original_projection; }                // C 11206

      /* Now, round the distance.  */
      distance = sfnt_round_symmetric (interpreter, distance);   // C 11209
    }

  /* Flag C means look at the minimum distance.  */

  if (opcode & 8) != 0                                     // C 11214
    {
      if original_projection >= 0                          // C 11216
	 && distance < (*interpreter).state.minimum_distance
	{ distance = (*interpreter).state.minimum_distance; }   // C 11218
      else if original_projection < 0                      // C 11219
	       && distance > -(*interpreter).state.minimum_distance
	{ distance = -(*interpreter).state.minimum_distance; }   // C 11221
    }

  /* Finally, move the point.  */
  sfnt_move_zp1 (interpreter, p, 1,
		 sfnt_sub! (distance, current_projection));   // C 11225-11226

  /* Set RP1 to RP0 and RP2 to the point.  If flag 3 is set, also make
     it RP0.  */
  (*interpreter).state.rp1 = (*interpreter).state.rp0;     // C 11230
  (*interpreter).state.rp2 = p as uint16_t;                // C 11231

  if (opcode & 16) != 0                                    // C 11233
    { (*interpreter).state.rp0 = p as uint16_t; }          // C 11234
}

/// C 11257-11293: `sfnt_project_zp1_zp0_org` - C's `static`, called by `sfnt_interpret_mdrp`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_project_zp1_zp0_org (interpreter: *mut sfnt_interpreter,
				    p1: uint32_t, p2: uint32_t) -> sfnt_f26dot6
{                                                          // C 11257-11293
  macro_rules! DUAL_PROJECT { ($x:expr, $y:expr) => {
    sfnt_dual_project_vector (interpreter, $x, $y)
  } }

  let mut x1: sfnt_fword;
  let mut y1: sfnt_fword;
  let mut x2: sfnt_fword;
  let mut y2: sfnt_fword;
  let mut projection: sfnt_fword;
  let mut simple: *mut sfnt_simple_glyph;
  /* The four out-parameters of the two calls below (C 11290-11291); `sfnt_address_zp1`/`zp0` take
     their addresses with the x/y pair NULL.  */
  let mut org_x1: sfnt_f26dot6 = 0;
  let mut org_y1: sfnt_f26dot6 = 0;
  let mut org_x2: sfnt_f26dot6 = 0;
  let mut org_y2: sfnt_f26dot6 = 0;

  /* Addressing the twilight zone, perhaps only partially.  */
  /* C 11275's `goto project_normally` skips the simple-glyph path; the block ends at C 11289,
     where the target is.  */
  'project_normally: {
    if (*interpreter).state.zp0 == 0                     // C 11266
       || (*interpreter).state.zp1 == 0                  // C 11267
       /* Not interpreting a glyph.  */
       || (*interpreter).glyph_zone.is_null ()           // C 11269
       /* Not interpreting a simple glyph.  */
       || (*(*interpreter).glyph_zone).simple.is_null ()   // C 11271
       /* P1 or P2 are phantom points.  */
       || p1 as usize >= (*(*(*interpreter).glyph_zone).simple).number_of_points   // C 11273
       || p2 as usize >= (*(*(*interpreter).glyph_zone).simple).number_of_points   // C 11274
      { break 'project_normally; }                       // C 11275

    simple = (*(*interpreter).glyph_zone).simple;        // C 11277
    x1 = *(*simple).x_coordinates.offset (p1 as isize);  // C 11278
    y1 = *(*simple).y_coordinates.offset (p1 as isize);  // C 11279
    x2 = *(*simple).x_coordinates.offset (p2 as isize);  // C 11280
    y2 = *(*simple).y_coordinates.offset (p2 as isize);  // C 11281

    /* Compute the projection.  */
    /* C's `projection` is an `sfnt_fword` and the call returns an `sfnt_f26dot6`, so the
       assignment narrows - C's implicit conversion, written out.  */
    projection = DUAL_PROJECT! (x1 as sfnt_f26dot6 - x2 as sfnt_f26dot6,
				y1 as sfnt_f26dot6 - y2 as sfnt_f26dot6) as sfnt_fword;   // C 11284

    /* Return the projection, scaled with rounding.  */
    return sfnt_mul_fixed_round (projection as sfnt_fixed,
				 (*interpreter).scale);   // C 11287
  }

  /* C 11289: `project_normally:`.  */
  sfnt_address_zp1 (interpreter, p1, std::ptr::null_mut (), std::ptr::null_mut (),
		    &raw mut org_x1, &raw mut org_y1);   // C 11290
  sfnt_address_zp0 (interpreter, p2, std::ptr::null_mut (), std::ptr::null_mut (),
		    &raw mut org_x2, &raw mut org_y2);   // C 11291
  return DUAL_PROJECT! (org_x1 - org_x2, org_y1 - org_y2);   // C 11292
}

/// C 11305-11382: `sfnt_interpret_mdrp` - C's `static`, called by name only.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_mdrp (interpreter: *mut sfnt_interpreter, opcode: uint32_t)
{                                                          // C 11305-11382
  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }
  /* C 6597-6601's `#define POP()` - the live `#ifndef TEST` arm; the `#else` arm's body names
     the TEST-only `pop_hook` member (see this file's header).  */
  macro_rules! POP { () => {
    if (*interpreter).SP == (*interpreter).stack
      { TRAP! (c"stack underflow".as_ptr ()); 0 }
    else
      { (*interpreter).SP = (*interpreter).SP.offset (-1); *(*interpreter).SP }
  } }
  macro_rules! PROJECT { ($x:expr, $y:expr) => {
    sfnt_project_vector (interpreter, $x, $y)
  } }

  let mut p: uint32_t;
  let mut distance: sfnt_f26dot6;
  let mut applied: sfnt_f26dot6;
  let mut current_projection: sfnt_f26dot6;
  /* The four out-parameters of the two calls below (C 11317-11319, both with the original pair
     NULL).  */
  let mut x: sfnt_f26dot6 = 0;
  let mut y: sfnt_f26dot6 = 0;
  let mut rx: sfnt_f26dot6 = 0;
  let mut ry: sfnt_f26dot6 = 0;

  /* Point number.  */
  p = POP! ();                                             // C 11314

  /* Load the points.  */
  sfnt_address_zp1 (interpreter, p, &raw mut x, &raw mut y,
		    std::ptr::null_mut (), std::ptr::null_mut ());   // C 11317
  sfnt_address_zp0 (interpreter, (*interpreter).state.rp0 as uint32_t,
		    &raw mut rx, &raw mut ry,
		    std::ptr::null_mut (), std::ptr::null_mut ());   // C 11318-11319

  /* Calculate the distance between P and rp0 prior to hinting.  */
  distance = sfnt_project_zp1_zp0_org (interpreter, p,
				       (*interpreter).state.rp0 as uint32_t);   // C 11322-11323

  /* Calculate the distance between P and rp0 as of now in the hinting
     process.  */
  current_projection = PROJECT! (x - rx, y - ry);          // C 11327

  /* Test against the single width value.  */

  if (*interpreter).state.sw_cut_in > 0                    // C 11331
     && distance < ((*interpreter).state.single_width_value
		    + (*interpreter).state.sw_cut_in)      // C 11332-11333
     && distance > ((*interpreter).state.single_width_value
		    - (*interpreter).state.sw_cut_in)      // C 11334-11335
    {
      /* Use the single width instead, as the CVT entry is too
	 small.  */

      if distance >= 0                                     // C 11340
	{ distance = (*interpreter).state.single_width_value; }   // C 11341
      else
	{ distance = -(*interpreter).state.single_width_value; }   // C 11343
    }

  /* Flag B implies that the distance should be rounded.  The CVT cut
     in is not taken into account by MDRP, contrary to earlier
     presumptions.  */

  if (opcode & 4) != 0                                     // C 11350
    { applied = sfnt_round_symmetric (interpreter, distance); }   // C 11351
  else
    { applied = distance; }                                // C 11353

  /* Flag C means look at the minimum distance.  */

  if (opcode & 8) != 0                                     // C 11357
    {
      /* Test the sign of the initial distance, but compare the
	 distance that will be applied in reality against the minimum
	 distance.  */

      if distance >= 0                                     // C 11363
	 && applied < (*interpreter).state.minimum_distance
	{ applied = (*interpreter).state.minimum_distance; }   // C 11365
      else if distance < 0                                 // C 11366
	       && applied > -(*interpreter).state.minimum_distance
	{ applied = -(*interpreter).state.minimum_distance; }   // C 11368
    }

  /* Finally, move the point.  */
  sfnt_move_zp1 (interpreter, p, 1,
		 sfnt_sub! (applied, current_projection));   // C 11372-11373

  /* Set RP1 to RP0 and RP2 to the point.  If flag 3 is set, also make
     it RP0.  */
  (*interpreter).state.rp1 = (*interpreter).state.rp0;     // C 11377
  (*interpreter).state.rp2 = p as uint16_t;                // C 11378

  if (opcode & 16) != 0                                    // C 11380
    { (*interpreter).state.rp0 = p as uint16_t; }          // C 11381
}

/* ----------------------------------------------------------------------------------------------
   C 13370-13915: the variation-selector and table-mapping block, in C's order - C 13370-13372's
   comment with `sfnt_read_default_uvs_table` (C 13375-13438), C 13440-13442's with
   `sfnt_read_nondefault_uvs_table` (C 13445-13512), C 13514's with `sfnt_compare_table_offsets`
   (C 13517-13530), C 13532-13540's with `sfnt_create_uvs_context` (C 13543-13701),
   C 13703's with `sfnt_free_uvs_context` (C 13706-13717), C 13719-13720's with
   `sfnt_compare_uvs_mapping` (C 13723-13737), C 13739's with `sfnt_compare_unicode_value_range`
   (C 13742-13757), C 13759-13763's with `sfnt_variation_glyph_for_char` (C 13765-13776),
   C 13778-13779's with `sfnt_is_character_default` (C 13782-13791), and C 13797-13899's comments
   with the mapping trio under C 13795's `#if defined HAVE_MMAP && !defined TEST` and C 13871's
   `#ifndef TEST`: `sfnt_map_table` (C 13815-13856), `sfnt_unmap_table` (C 13862-13865) and
   `sfnt_read_table` (C 13880-13915).  **The next live definition is C 13918's glyph-metrics
   block** (`sfnt_scale_metrics`, ported in slice 7), and the ledger's `## C line ranges` names both
   boundaries.

   `qsort`/`bsearch` are declared at their use site (`<stdlib.h>`, no mirror carries them; the
   `buffer.rs` 129-134 `qsort` shape), and the three comparators are `unsafe extern "C" fn` because
   C takes their addresses (PORTING.md 1.4).  C's `TEST_STATIC` (C 128/137) is
   `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]`, and the three functions C puts
   inside `#ifndef TEST` are gated `not (feature = "test")` on top of the file gate.
   ---------------------------------------------------------------------------------------------- */
/// C 13375-13438: `sfnt_read_default_uvs_table` - C's `static`, called by `sfnt_create_uvs_context`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_default_uvs_table (fd: c_int, offset: off_t) -> *mut sfnt_default_uvs_table
{                                                          // C 13375-13438
  let mut uvs: *mut sfnt_default_uvs_table;
  /* C 13378's counters are filled in through `read` (their addresses are taken); zero-initialised
     so Rust's analysis sees what C's flow guarantees (PORTING.md 1.5, Deviation 10's class).  */
  let mut num_ranges: uint32_t = 0;
  let mut i: uint32_t;
  let mut j: uint32_t;
  let mut size: ssize_t;
  let mut temp: ssize_t;
  /* C 13380's `char data[512]` is written by `read` before it is read; zero-initialised
     (PORTING.md 1.5).  */
  let mut data: [c_char; 512] = [0; 512];

  /* First, seek to the given offset.  */

  if lseek (fd, offset, SEEK_SET) != offset             // C 13384
    { return std::ptr::null_mut (); }                    // C 13385

  /* Next, read the number of ranges present.  */

  if read (fd, &raw mut num_ranges as *mut c_void, std::mem::size_of::<uint32_t> ())
     != std::mem::size_of::<uint32_t> () as ssize_t      // C 13389-13390
    { return std::ptr::null_mut (); }                    // C 13391

  /* Swap the number of ranges present.  */
  sfnt_swap32! (&raw mut num_ranges);                     // C 13394

  /* Now, allocate enough to hold the UVS table.  */

  size = std::mem::size_of::<sfnt_default_uvs_table> () as ssize_t;   // C 13398
  /* C 13399-13401's `ckd_mul (&temp, num_ranges, sizeof *uvs->ranges) || ckd_add (&size, size,
     temp)`: the two `__builtin_*_overflow` calls write their result through the pointer and return
     the flag, so the port writes the two halves out and returns early (P-15's spelling, the
     `sfnt_read_cmap_table` site).  */
  let (ckd_temp, mul_overflow)
    = (num_ranges as ssize_t).overflowing_mul (std::mem::size_of::<sfnt_unicode_value_range> ()
					       as ssize_t);
  temp = ckd_temp;
  if mul_overflow
    { return std::ptr::null_mut (); }
  let (ckd_size, add_overflow) = size.overflowing_add (temp);
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  uvs = xmalloc (size as usize) as *mut sfnt_default_uvs_table;   // C 13403

  /* Fill in the data which was already read.  */
  (*uvs).num_unicode_value_ranges = num_ranges;            // C 13406

  /* Fill in the pointer to the ranges.  */
  (*uvs).ranges = uvs.offset (1) as *mut sfnt_unicode_value_range;   // C 13409
  i = 0;                                                   // C 13410

  /* Read each default UVS range in multiples of 512 bytes.  Then,
     fill in uvs->ranges.  */

  while num_ranges != 0                                    // C 13415
    {
      size = (if num_ranges > 128 { 512 } else { num_ranges * 4 }) as ssize_t;   // C 13417

      if read (fd, data.as_mut_ptr () as *mut c_void, size as usize) != size   // C 13419
	{
	  xfree (uvs as *mut c_void);                      // C 13421
	  return std::ptr::null_mut ();                    // C 13422
	}

      j = 0;                                               // C 13425
      while j < (size / 4) as uint32_t
	{
	  (*(*uvs).ranges.offset ((i + j) as isize)).start_unicode_value
	    = sfnt_read_24 ((data.as_mut_ptr () as *mut c_uchar).offset ((j * 4) as isize));   // C 13427-13428
	  (*(*uvs).ranges.offset ((i + j) as isize)).additional_count
	    = data[(j * 4 + 1) as usize] as u8;            // C 13429
	  j += 1;
	}

      i += j;                                              // C 13432
      num_ranges -= (size / 4) as uint32_t;                // C 13433
    }

  /* Return the resulting default UVS table.  */
  return uvs;                                              // C 13437
}

/// C 13445-13512: `sfnt_read_nondefault_uvs_table` - C's `static`, called by
/// `sfnt_create_uvs_context`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_read_nondefault_uvs_table (fd: c_int, offset: off_t)
					   -> *mut sfnt_nondefault_uvs_table
{                                                          // C 13445-13512
  let mut uvs: *mut sfnt_nondefault_uvs_table;
  /* The same class as `sfnt_read_default_uvs_table`'s counters.  */
  let mut num_mappings: uint32_t = 0;
  let mut i: uint32_t;
  let mut j: uint32_t;
  let mut size: ssize_t;
  let mut temp: ssize_t;
  /* C 13450's `char data[500]`, the same class as the 512-byte one above.  */
  let mut data: [c_char; 500] = [0; 500];

  /* First, seek to the given offset.  */

  if lseek (fd, offset, SEEK_SET) != offset             // C 13454
    { return std::ptr::null_mut (); }                    // C 13455

  /* Next, read the number of mappings present.  */

  if read (fd, &raw mut num_mappings as *mut c_void, std::mem::size_of::<uint32_t> ())
     != std::mem::size_of::<uint32_t> () as ssize_t      // C 13459-13460
    { return std::ptr::null_mut (); }                    // C 13461

  /* Swap the number of mappings present.  */
  sfnt_swap32! (&raw mut num_mappings);                   // C 13464

  /* Now, allocate enough to hold the UVS table.  */

  size = std::mem::size_of::<sfnt_nondefault_uvs_table> () as ssize_t;   // C 13468
  let (ckd_temp, mul_overflow)
    = (num_mappings as ssize_t).overflowing_mul (std::mem::size_of::<sfnt_uvs_mapping> ()
						 as ssize_t);   // C 13469
  temp = ckd_temp;
  if mul_overflow
    { return std::ptr::null_mut (); }
  let (ckd_size, add_overflow) = size.overflowing_add (temp);   // C 13470
  size = ckd_size;
  if add_overflow
    { return std::ptr::null_mut (); }

  uvs = xmalloc (size as usize) as *mut sfnt_nondefault_uvs_table;   // C 13473

  /* Fill in the data which was already read.  */
  (*uvs).num_uvs_mappings = num_mappings;                  // C 13476

  /* Fill in the pointer to the mappings.  */
  (*uvs).mappings = uvs.offset (1) as *mut sfnt_uvs_mapping;   // C 13479

  i = 0;                                                   // C 13481

  /* Read each nondefault UVS mapping in multiples of 500 bytes.
     Then, fill in uvs->ranges.  */

  while num_mappings != 0                                  // C 13486
    {
      size = (if num_mappings > 100 { 500 } else { num_mappings * 5 }) as ssize_t;   // C 13488

      if read (fd, data.as_mut_ptr () as *mut c_void, size as usize) != size   // C 13490
	{
	  xfree (uvs as *mut c_void);                      // C 13492
	  return std::ptr::null_mut ();                    // C 13493
	}

      j = 0;                                               // C 13496
      while j < (size / 5) as uint32_t
	{
	  (*(*uvs).mappings.offset ((i + j) as isize)).unicode_value
	    = sfnt_read_24 ((data.as_mut_ptr () as *mut c_uchar).offset ((j * 5) as isize));   // C 13498-13499
	  memcpy (
	    &raw mut (*(*uvs).mappings.offset ((i + j) as isize)).base_character_value
	    as *mut uint16_t as *mut c_void,
	    (data.as_mut_ptr () as *mut c_char).offset ((j * 5 + 3) as isize) as *const c_void,
	    std::mem::size_of::<uint16_t> ());             // C 13500-13502
	  sfnt_swap16! (&raw mut (*(*uvs).mappings.offset ((i + j) as isize))
			.base_character_value);            // C 13503
	  j += 1;
	}

      i += j;                                              // C 13506
      num_mappings -= (size / 5) as uint32_t;              // C 13507
    }

  /* Return the nondefault UVS table.  */
  return uvs;                                              // C 13511
}

/// C 13517-13530: `sfnt_compare_table_offsets` - C's `static`, whose address `qsort`/`bsearch`
/// take (PORTING.md 1.4).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_compare_table_offsets (a: *const c_void, b: *const c_void) -> c_int
{                                                          // C 13517-13530
  let mut rec_a: *const sfnt_table_offset_rec;
  let mut rec_b: *const sfnt_table_offset_rec;

  rec_a = a as *const sfnt_table_offset_rec;               // C 13521
  rec_b = b as *const sfnt_table_offset_rec;               // C 13522

  if (*rec_a).offset < (*rec_b).offset                     // C 13524
    { return -1; }                                         // C 13525
  else if (*rec_a).offset > (*rec_b).offset                // C 13526
    { return 1; }                                          // C 13527

  return 0;                                                // C 13529
}

/* C: <stdlib.h>: `void qsort (void *, size_t, size_t, int (*) (const void *, const void *));` and
   `void *bsearch (const void *, const void *, size_t, size_t,
   int (*) (const void *, const void *));` - no mirror carries either, so they are declared at the
   use site (P-72 rule 3 / P-55; the `buffer.rs` 129-134 `qsort` shape), and `bsearch`'s result is
   the untyped pointer C assigns to `struct sfnt_table_offset_rec *`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub fn qsort (base: *mut c_void, nmemb: usize, size: usize,
		compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
  pub fn bsearch (key: *const c_void, base: *const c_void, nmemb: usize, size: usize,
		  compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>)
		  -> *mut c_void;
}

/// C 13543-13701: `sfnt_create_uvs_context` - C's `TEST_STATIC` (C 128/137), so the port spells
/// the macro's bit with the attribute.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_create_uvs_context (cmap: *mut sfnt_cmap_format_14,
						  fd: c_int) -> *mut sfnt_uvs_context
{                                                          // C 13543-13701
  let mut table_offsets: *mut sfnt_table_offset_rec;
  let mut rec: *mut sfnt_table_offset_rec;
  /* C 13545's `template` is only ever given its `offset` field, which is what the comparator
     reads; zero-initialised (PORTING.md 1.5, Deviation 10's class).  */
  let mut template: sfnt_table_offset_rec = std::mem::zeroed ();
  let mut i: size_t;
  let mut nmemb: size_t;
  let mut j: size_t;
  let mut offset: off_t;
  let mut context: *mut sfnt_uvs_context;

  context = std::ptr::null_mut ();                         // C 13550

  /* First, record and sort the UVS and nondefault UVS table offsets
     in ascending order.  */

  table_offsets = xcalloc ((*cmap).num_var_selector_records as usize,
			   std::mem::size_of::<sfnt_table_offset_rec> ())
    as *mut sfnt_table_offset_rec;                         // C 13555-13556
  nmemb = (*cmap).num_var_selector_records as usize * 2;   // C 13557
  j = 0;                                                   // C 13558

  /* C 13569/13578's two `goto bail`s leave the scan and land at C 13685; the labelled block is
     that region (the `sfnt_open_tables` reading).  */
  'bail: {
    i = 0;                                                 // C 13560
    while i < (*cmap).num_var_selector_records as usize
      {
	/* Note that either offset may be 0, meaning there is no such
	   table.  */

	if (*(*cmap).records.offset (i as isize)).default_uvs_offset != 0   // C 13565
	  {
	    let (o, overflow)
	      = (*cmap).offset.overflowing_add (
		  (*(*cmap).records.offset (i as isize)).default_uvs_offset as off_t);   // C 13567-13568
	    (*table_offsets.offset (j as isize)).offset = o;
	    if overflow
	      { break 'bail; }                             // C 13569

	    (*table_offsets.offset (j as isize)).is_nondefault_table = false;   // C 13571
	    j += 1;
	  }

	if (*(*cmap).records.offset (i as isize)).nondefault_uvs_offset != 0   // C 13574
	  {
	    let (o, overflow)
	      = (*cmap).offset.overflowing_add (
		  (*(*cmap).records.offset (i as isize)).nondefault_uvs_offset as off_t);   // C 13576-13577
	    (*table_offsets.offset (j as isize)).offset = o;
	    if overflow
	      { break 'bail; }                             // C 13578

	    (*table_offsets.offset (j as isize)).is_nondefault_table = true;   // C 13580
	    j += 1;
	  }

	i += 1;
      }

    /* Make nmemb the number of offsets actually looked up.  */
    nmemb = j;                                             // C 13585

    qsort (table_offsets as *mut c_void, nmemb,
	   std::mem::size_of::<sfnt_table_offset_rec> (),
	   Some (sfnt_compare_table_offsets));             // C 13587-13588

    /* Now go through table_offsets, and read everything.  nmemb is the
       number of elements in table_offsets[i]; it is kept up to date
       when duplicate members are removed.  */
    offset = -1;                                           // C 13593

    i = 0;                                                 // C 13595
    while i < nmemb
      {
	/* Skip past duplicate tables.  */

	while (*(table_offsets.offset (i as isize))).offset == offset && i < nmemb   // C 13599
	  {
	    nmemb -= 1;                                    // C 13601
	    *(table_offsets.offset (i as isize))
	      = *(table_offsets.offset ((i + 1) as isize));   // C 13602
	  }

	/* If the last element of the array is a duplicate, break out of
	   the loop.  */

	if i == nmemb                                       // C 13608
	  { break; }                                       // C 13609

	/* Read the correct type of table depending on
	   table_offsets[i].is_nondefault_table.  Then skip past
	   duplicate tables.  Don't handle the case where two different
	   kind of tables share the same offset, because that is not
	   possible in a valid variation selector record.  */

	offset = (*(table_offsets.offset (i as isize))).offset;   // C 13617

	if (*(table_offsets.offset (i as isize))).is_nondefault_table   // C 13619
	  { (*(table_offsets.offset (i as isize))).table
	      = sfnt_read_nondefault_uvs_table (fd, offset) as *mut c_void; }   // C 13620-13621
	else
	  { (*(table_offsets.offset (i as isize))).table
	      = sfnt_read_default_uvs_table (fd, offset) as *mut c_void; }   // C 13623-13624

	i += 1;
      }

    /* Now make the context.  C 13628's `sizeof *context` and C 13631's `sizeof
       *context->records` are the `sizeof`-over-a-place spelling that keeps the registered
       `context` family token (`docs/ports/token-aliases.tsv` 30; Deviation 17's ruling, the
       C 2694/2709 sites above): `size_of_val` sizes the place and never reads it, so the NULL
       `context` of C 13550 and the not-yet-assigned `records` are the operands C has too.  */
    context = xmalloc (std::mem::size_of_val (&(*context)) as usize)
      as *mut sfnt_uvs_context;                            // C 13628
    (*context).num_records = (*cmap).num_var_selector_records as usize;   // C 13629
    (*context).nmemb = nmemb;                              // C 13630
    (*context).records = xmalloc (std::mem::size_of_val (&(*(*context).records))
				  * (*cmap).num_var_selector_records as usize)
      as *mut sfnt_mapped_variation_selector_record;       // C 13631-13632

    i = 0;                                                 // C 13634
    while i < (*cmap).num_var_selector_records as usize
      {
	(*(*context).records.offset (i as isize)).selector
	  = (*(*cmap).records.offset (i as isize)).var_selector;   // C 13636

	/* Either offset may be 0, meaning no such table exists.  Also,
	   the code below will lose if more than one kind of table
	   shares the same offset, because that is impossible.  */

	if (*(*cmap).records.offset (i as isize)).default_uvs_offset != 0   // C 13642
	  {
	    /* Resolve the default table.  */
	    template.offset
	      = (*(*cmap).records.offset (i as isize)).default_uvs_offset as off_t
		+ (*cmap).offset;                          // C 13645-13646
	    rec = bsearch (&raw const template as *const c_void,
			   table_offsets as *const c_void,
			   nmemb, std::mem::size_of::<sfnt_table_offset_rec> (),
			   Some (sfnt_compare_table_offsets)) as *mut sfnt_table_offset_rec;   // C 13647-13649

	    /* Make sure this record is the right type.  */
	    if rec.is_null () || (*rec).is_nondefault_table || (*rec).table.is_null ()   // C 13652
	      { break 'bail; }                             // C 13653

	    (*(*context).records.offset (i as isize)).default_uvs
	      = (*rec).table as *mut sfnt_default_uvs_table;   // C 13655
	  }
	else
	  { (*(*context).records.offset (i as isize)).default_uvs = std::ptr::null_mut (); }   // C 13658

	if (*(*cmap).records.offset (i as isize)).nondefault_uvs_offset != 0   // C 13660
	  {
	    /* Resolve the nondefault table.  */
	    template.offset
	      = (*(*cmap).records.offset (i as isize)).nondefault_uvs_offset as off_t
		+ (*cmap).offset;                          // C 13663-13664
	    rec = bsearch (&raw const template as *const c_void,
			   table_offsets as *const c_void,
			   nmemb, std::mem::size_of::<sfnt_table_offset_rec> (),
			   Some (sfnt_compare_table_offsets)) as *mut sfnt_table_offset_rec;   // C 13665-13667

	    if rec.is_null ()                              // C 13669
	      { break 'bail; }                             // C 13670

	    /* Make sure this record is the right type.  */
	    if rec.is_null () || !(*rec).is_nondefault_table || (*rec).table.is_null ()   // C 13673
	      { break 'bail; }                             // C 13674

	    (*(*context).records.offset (i as isize)).nondefault_uvs
	      = (*rec).table as *mut sfnt_nondefault_uvs_table;   // C 13676
	  }
	else
	  { (*(*context).records.offset (i as isize)).nondefault_uvs = std::ptr::null_mut (); }   // C 13679

	i += 1;
      }

    (*context).tables = table_offsets;                     // C 13682
    return context;                                        // C 13683
  }

  /* C 13685: `bail:`.  */

  if !context.is_null ()                                   // C 13687
    {
      xfree ((*context).records as *mut c_void);           // C 13689
      xfree (context as *mut c_void);                      // C 13690
    }

  /* Loop through and free any tables that might have been read
     already.  */

  i = 0;                                                   // C 13696
  while i < nmemb
    {
      xfree ((*(table_offsets.offset (i as isize))).table);   // C 13697
      i += 1;
    }

  xfree (table_offsets as *mut c_void);                    // C 13699
  return std::ptr::null_mut ();                            // C 13700
}

/// C 13706-13717: `sfnt_free_uvs_context` - C's `TEST_STATIC`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_free_uvs_context (c: *mut sfnt_uvs_context)
{                                                          // C 13706-13717
  let mut i: size_t;

  xfree ((*c).records as *mut c_void);                     // C 13710

  i = 0;                                                   // C 13712
  while i < (*c).nmemb
    {
      xfree ((*(*c).tables.offset (i as isize)).table);    // C 13713
      i += 1;
    }

  xfree ((*c).tables as *mut c_void);                      // C 13715
  xfree (c as *mut c_void);                                // C 13716
}

/// C 13723-13737: `sfnt_compare_uvs_mapping` - C's `static`, whose address `bsearch` takes.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_compare_uvs_mapping (k: *const c_void, v: *const c_void) -> c_int
{                                                          // C 13723-13737
  let mut key: *const sfnt_char;
  let mut value: *const sfnt_uvs_mapping;

  key = k as *const sfnt_char;                             // C 13728
  value = v as *const sfnt_uvs_mapping;                    // C 13729

  if *key < (*value).unicode_value                          // C 13731
    { return -1; }                                         // C 13732
  else if *key == (*value).unicode_value                    // C 13733
    { return 0; }                                          // C 13734

  return 1;                                                // C 13736
}

/// C 13742-13757: `sfnt_compare_unicode_value_range` - C's `static`, whose address `bsearch` takes.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_compare_unicode_value_range (k: *const c_void, v: *const c_void)
						       -> c_int
{                                                          // C 13742-13757
  let mut key: *const sfnt_char;
  let mut value: *const sfnt_unicode_value_range;

  key = k as *const sfnt_char;                             // C 13747
  value = v as *const sfnt_unicode_value_range;            // C 13748

  if *key < (*value).start_unicode_value                   // C 13750
    { return -1; }                                         // C 13751
  else if ((*key - (*value).start_unicode_value) as uint32_t
	   <= (*value).additional_count as uint32_t)       // C 13752-13753
    { return 0; }                                          // C 13754

  return 1;                                                // C 13756
}

/// C 13765-13776: `sfnt_variation_glyph_for_char` - C's `TEST_STATIC`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_variation_glyph_for_char (uvs: *mut sfnt_nondefault_uvs_table,
							c: sfnt_char) -> sfnt_glyph_index
{                                                          // C 13765-13776
  /* C 13765's return type is the `sfnt_glyph` *scalar* typedef, which this port spells
     `sfnt_glyph_index` because C's `struct sfnt_glyph` owns the other name (`include/sfnt.rs`
     208).  */
  let mut mapping: *mut sfnt_uvs_mapping;

  mapping = bsearch (&raw const c as *const c_void, (*uvs).mappings as *const c_void,
		     (*uvs).num_uvs_mappings as usize, std::mem::size_of::<sfnt_uvs_mapping> (),
		     Some (sfnt_compare_uvs_mapping)) as *mut sfnt_uvs_mapping;   // C 13771-13773

  return if !mapping.is_null () { (*mapping).base_character_value as sfnt_glyph_index } else { 0 };   // C 13775
}

/// C 13782-13791: `sfnt_is_character_default` - C's `TEST_STATIC`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_is_character_default (uvs: *mut sfnt_default_uvs_table,
						    c: sfnt_char) -> bool
{                                                          // C 13782-13791
  /* UVS->ranges comprises ranges of characters sorted in increasing
     order; these ranges cannot overlap.  */

  return !bsearch (&raw const c as *const c_void, (*uvs).ranges as *const c_void,
		   (*uvs).num_unicode_value_ranges as usize,
		   std::mem::size_of::<sfnt_unicode_value_range> (),
		   Some (sfnt_compare_unicode_value_range)).is_null ();   // C 13788-13790
}

/// C 13815-13856: `sfnt_map_table` - C's non-`static` definition, in C 13795's live
/// `#if defined HAVE_MMAP && !defined TEST` arm.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	    not (feature = "test")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfnt_map_table (fd: c_int, subtable: *mut sfnt_offset_subtable,
					 tag: uint32_t, table: *mut sfnt_mapped_table) -> c_int
{                                                          // C 13815-13856
  let mut directory: *mut sfnt_table_directory;
  let mut offset: size_t;
  let mut page: size_t;
  let mut map_offset: size_t;
  let mut data: *mut c_void;
  let mut i: c_int;

  /* Find the table in the directory.  */

  i = 0;                                                   // C 13825
  loop
    {
      if !(i < (*subtable).num_tables as c_int)             // C 13827
	{ return 1; }                                      // C 13828
      directory = &raw mut *(*subtable).subtables.offset (i as isize);   // C 13829
      if (*directory).tag == tag                            // C 13830
	{ break; }                                         // C 13831
      i += 1;
    }

  /* Now try to map the glyph data.  Make sure offset is a multiple of
     the page size.  */

  page = getpagesize () as size_t;                          // C 13837
  offset = (*directory).offset as size_t & !(page - 1);     // C 13838

  /* Figure out how much larger the mapping should be.  */
  map_offset = (*directory).offset as size_t - offset;      // C 13841

  /* Do the mmap.  */
  data = mmap (std::ptr::null_mut (), (*directory).length as size_t + map_offset,
	       PROT_READ, MAP_PRIVATE, fd, offset as off_t);   // C 13844-13845

  if data == MAP_FAILED                                     // C 13847
    { return 1; }                                          // C 13848

  /* Fill in *TABLE.  */
  (*table).data = (data as *mut u8).offset (map_offset as isize) as *mut c_void;   // C 13851
  (*table).mapping = data;                                  // C 13852
  (*table).length = (*directory).length as usize;           // C 13853
  (*table).size = (*directory).length as size_t + map_offset;   // C 13854
  return 0;                                                 // C 13855
}

/// C 13862-13865: `sfnt_unmap_table` - C's non-`static` definition in the same arm.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	    not (feature = "test")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfnt_unmap_table (table: *mut sfnt_mapped_table) -> c_int
{                                                          // C 13862-13865
  return (munmap ((*table).mapping, (*table).size) != 0) as c_int;   // C 13864
}

/// C 13880-13915: `sfnt_read_table` - C's non-`static` definition, in C 13871's live
/// `#ifndef TEST` arm.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	    not (feature = "test")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfnt_read_table (fd: c_int, subtable: *mut sfnt_offset_subtable,
					  tag: uint32_t, length: *mut size_t) -> *mut c_void
{                                                          // C 13880-13915
  let mut directory: *mut sfnt_table_directory;
  let mut data: *mut c_void;
  let mut i: c_int;

  /* Find the table in the directory.  */

  i = 0;                                                   // C 13889
  loop
    {
      if !(i < (*subtable).num_tables as c_int)             // C 13891
	{ return std::ptr::null_mut (); }                   // C 13892
      directory = &raw mut *(*subtable).subtables.offset (i as isize);   // C 13893
      if (*directory).tag == tag                            // C 13894
	{ break; }                                         // C 13895
      i += 1;
    }

  /* Seek to the table.  */

  if lseek (fd, (*directory).offset as off_t, SEEK_SET) != (*directory).offset as off_t   // C 13900
    { return std::ptr::null_mut (); }                      // C 13901

  /* Now allocate enough to hold the data and read into it.  */

  data = xmalloc ((*directory).length as usize);           // C 13905
  if read (fd, data, (*directory).length as usize) != (*directory).length as ssize_t   // C 13906
    {
      xfree (data);                                        // C 13908
      return std::ptr::null_mut ();                        // C 13909
    }

  /* Return the length and table data.  */
  *length = (*directory).length as usize;                  // C 13913
  return data;                                             // C 13914
}

/* ----------------------------------------------------------------------------------------------
   C 12116-12295 and C 12495-12566: the instructed-outline helpers, in C's order - C 12116's
   `sfnt_decompose_instructed_outline` (C 12117-12166, whose two `goto fail`s are one labelled
   block), C 12168-12173's comment with the `TEST_STATIC` builder `sfnt_build_instructed_outline`
   (C 12176-12243), C 12247-12255's comment with `sfnt_compute_phantom_points` (C 12258-12295),
   and `sfnt_transform_f26dot6` (C 12496-12566), the 26.6 twin of the already-ported
   `sfnt_transform_coordinates` (C 2551-2618) that C 12861 calls.  **The next live definition is
   C 12591's `sfnt_interpret_compound_glyph_2`** (it and the three items that follow call
   `sfnt_interpret_run`, C 11396), and the ledger's `## C line ranges` names the boundaries.
   C 12305-12494's `sfnt_interpret_simple_glyph` and C 11996-12115's two program entries sit
   between the two ranges and wait for the interpreter.
   ---------------------------------------------------------------------------------------------- */
/// C 12117-12166: `sfnt_decompose_instructed_outline` - C's `static`, called by the builder below.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_decompose_instructed_outline (outline: *mut sfnt_instructed_outline,
					     move_to: sfnt_move_to_proc,
					     line_to: sfnt_line_to_proc,
					     curve_to: sfnt_curve_to_proc,
					     dcontext: *mut c_void) -> c_int
{                                                          // C 12117-12166
  let mut here: size_t;
  let mut last: size_t;
  let mut n: size_t;

  if (*outline).num_contours == 0                         // C 12125
    { return 0; }                                          // C 12126

  /* C 12140/12147's two `goto fail`s land at C 12164's `fail:`, which is the `return 1`
     below; the labelled block is the body they leave.  */
  'fail: {
    here = 0;                                              // C 12128

    n = 0;                                                 // C 12130
    while n < (*outline).num_contours
      {
	/* here is the first index into the glyph's point arrays
	   belonging to the contour in question.  last is the index
	   of the last point in the contour.  */
	last = *(*outline).contour_end_points.offset (n as isize);   // C 12135

	/* Make sure here and last make sense.  */

	if here > last || last >= (*outline).num_points        // C 12139
	  { break 'fail; }                                     // C 12140

	if sfnt_decompose_glyph_2 (here, last, move_to,
				   line_to, curve_to, dcontext,
				   (*outline).x_points,
				   (*outline).y_points,
				   (*outline).flags, 1024) != 0   // C 12142-12146
	  { break 'fail; }                                     // C 12147

	/* Move forward to the start of the next contour.  */
	here = last + 1;                                       // C 12150

	/* here may be a phantom point when outlining a compound glyph,
	   as they can have phantom points mixed in with contours.

	   When that happens, skip past all the phantom points.  */

	while here < (*outline).num_points
	   && ((*(*outline).flags.offset (here as isize) as c_int)
	       & SFNT_POINT_PHANTOM) != 0                      // C 12157-12158
	  { here += 1; }                                       // C 12159

	n += 1;
      }

    return 0;                                              // C 12162
  }

  /* C 12164: `fail:`.  */
  return 1;                                                // C 12165
}

/// C 12176-12243: `sfnt_build_instructed_outline` - C's `TEST_STATIC`, so the port spells the
/// macro's bit with the attribute.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_build_instructed_outline (instructed: *mut sfnt_instructed_outline,
							advance_width: *mut sfnt_fixed)
							-> *mut sfnt_glyph_outline
{                                                          // C 12176-12243
  let mut outline: *mut sfnt_glyph_outline;
  let mut rc: c_int;
  let mut x1: sfnt_f26dot6;
  let mut x2: sfnt_f26dot6;

  std::ptr::write_bytes (&raw mut build_outline_context as *mut u8, 0,
			 std::mem::size_of_val (&build_outline_context));   // C 12183

  /* Allocate the outline now with enough for 44 words at the end.  */
  outline = xmalloc (std::mem::size_of::<sfnt_glyph_outline> ()
		     + 40 * std::mem::size_of::<sfnt_glyph_outline_command> ())
    as *mut sfnt_glyph_outline;                            // C 12186
  (*outline).outline_size = 40;                            // C 12187
  (*outline).outline_used = 0;                             // C 12188
  (*outline).refcount = 0;                                 // C 12189
  (*outline).outline = outline.offset (1) as *mut sfnt_glyph_outline_command;   // C 12190-12191

  /* Clear outline bounding box.  */
  (*outline).xmin = 0;                                     // C 12194
  (*outline).ymin = 0;                                     // C 12195
  (*outline).xmax = 0;                                     // C 12196
  (*outline).ymax = 0;                                     // C 12197

  /* Set up the context.  */
  build_outline_context.outline = outline;                 // C 12200
  build_outline_context.factor = 0o177777;                 // C 12201

  /* Start decomposing.  */
  rc = sfnt_decompose_instructed_outline (instructed,
					  Some (sfnt_move_to_and_build),
					  Some (sfnt_line_to_and_build),
					  Some (sfnt_curve_to_and_build),
					  std::ptr::null_mut ());   // C 12204-12208

  /* Synchronize the outline object with what might have changed
     inside sfnt_decompose_glyph.  */
  outline = build_outline_context.outline;                 // C 12212

  /* Finally, obtain the origin point of the glyph after it has been
     instructed.  */

  if (*instructed).num_points > 1                          // C 12217
    {
      x1 = *(*instructed).x_points.offset (((*instructed).num_points - 2) as isize);   // C 12219
      x2 = *(*instructed).x_points.offset (((*instructed).num_points - 1) as isize);   // C 12220

      /* Convert the origin point to a 16.16 fixed point number.  */
      (*outline).origin = x1 * 1024;                       // C 12223

      /* Do the same for the advance width.  */
      *advance_width = (x2 - x1) * 1024;                   // C 12226
    }
  else
    {
      /* Phantom points are absent from this outline, which is
	 impossible.  */
      *advance_width = 0;                                  // C 12232
      (*outline).origin = 0;                               // C 12233
    }

  if rc != 0                                               // C 12236
    {
      xfree (outline as *mut c_void);                      // C 12238
      return std::ptr::null_mut ();                        // C 12239
    }

  return outline;                                          // C 12242
}

/// C 12258-12295: `sfnt_compute_phantom_points` - C's `static`, called by the simple-glyph
/// interpreter and the compound-glyph helper.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_compute_phantom_points (glyph: *mut sfnt_glyph,
				       metrics: *mut sfnt_glyph_metrics,
				       scale: sfnt_fixed,
				       x1: *mut sfnt_f26dot6, y1: *mut sfnt_f26dot6,
				       x2: *mut sfnt_f26dot6, y2: *mut sfnt_f26dot6,
				       s1: *mut sfnt_f26dot6, s2: *mut sfnt_f26dot6)
{                                                          // C 12258-12295
  let mut f1: sfnt_fword;
  let mut f2: sfnt_fword;

  /* Two ``phantom points'' are appended to each outline by the scaler
     prior to instruction interpretation.  One of these points
     represents the left-side bearing distance from xmin, while the
     other represents the advance width.  Both are then used after the
     hinting process to ensure that the reported glyph metrics are
     consistent with the instructed outline.  */

  /* First compute both values in fwords.  */
  /* C's `sfnt_fword f1 = sfnt_fword (glyph->xmin - metrics->lbearing)` narrows an `int`
     expression; the port writes the two conversions out (Deviation 16's class).  */
  f1 = ((*glyph).xmin as sfnt_fixed - (*metrics).lbearing) as sfnt_fword;   // C 12275
  f2 = (f1 as sfnt_fixed + (*metrics).advance) as sfnt_fword;   // C 12276

  /* Apply the metrics distortion.  */
  f1 += (*glyph).origin_distortion;                        // C 12279
  f2 += (*glyph).advance_distortion;                       // C 12280

  /* Next, scale both up.  */
  *s1 = sfnt_mul_f26dot6_fixed (f1 as sfnt_f26dot6, scale);   // C 12283
  *s2 = sfnt_mul_f26dot6_fixed (f2 as sfnt_f26dot6, scale);   // C 12284

  /* While not expressly provided in the manual, the phantom points
     (at times termed the advance and origin points) represent pixel
     coordinates within the raster, and are therefore rounded.  */
  *x1 = sfnt_round_f26dot6 (*s1);                          // C 12289
  *x2 = sfnt_round_f26dot6 (*s2);                          // C 12290

  /* Clear y1 and y2.  */
  *y1 = 0;                                                 // C 12293
  *y2 = 0;                                                 // C 12294
}

/// C 12496-12566: `sfnt_transform_f26dot6` - C's `static`, called by
/// `sfnt_interpret_compound_glyph_2` (C 12861).  It is the 26.6 twin of `sfnt_transform_coordinates`
/// (C 2551-2618), and its body is that function's with `sfnt_fixed` replaced.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_transform_f26dot6 (component: *mut sfnt_compound_glyph_component,
				  x: *mut sfnt_f26dot6, y: *mut sfnt_f26dot6,
				  num_coordinates: size_t,
				  x_off: sfnt_f26dot6, y_off: sfnt_f26dot6)
{                                                          // C 12496-12566
  let mut m1: f64;
  let mut m2: f64;
  let mut m3: f64;
  let mut m4: f64;
  let mut m5: f64;
  let mut m6: f64;
  let mut i: size_t;

  if ((*component).flags as c_int & 0o10) != 0		/* WE_HAVE_A_SCALE */
    {
      m1 = (*component).u.scale as f64 / 16384.0;          // C 12507
      m2 = 0.0; m3 = 0.0; m4 = 0.0;                        // C 12508
      m5 = (*component).u.scale as f64 / 16384.0;          // C 12509
      m6 = 0.0;                                            // C 12510
    }
  else if ((*component).flags as c_int & 0o100) != 0	/* WE_HAVE_AN_X_AND_Y_SCALE */
    {
      m1 = (*component).u.a.xscale as f64 / 16384.0;       // C 12514
      m2 = 0.0; m3 = 0.0; m4 = 0.0;                        // C 12515
      m5 = (*component).u.a.yscale as f64 / 16384.0;       // C 12516
      m6 = 0.0;                                            // C 12517
    }
  else if ((*component).flags as c_int & 0o200) != 0	/* WE_HAVE_A_TWO_BY_TWO */
    {
      m1 = (*component).u.b.xscale as f64 / 16384.0;       // C 12521
      m2 = (*component).u.b.scale01 as f64 / 16384.0;      // C 12522
      m3 = 0.0;                                            // C 12523
      m4 = (*component).u.b.scale10 as f64 / 16384.0;      // C 12524
      m5 = (*component).u.b.yscale as f64 / 16384.0;       // C 12525
      m6 = 0.0;                                            // C 12526
    }
  else							/* No scale, just apply x_off and y_off.  */
    {
      if x_off != 0 || y_off != 0                          // C 12530
	{
	  i = 0;                                           // C 12532
	  while i < num_coordinates
	    {
	      *(x.offset (i as isize)) += x_off;           // C 12533
	      *(y.offset (i as isize)) += y_off;
	      i += 1;
	    }
	}

      return;                                              // C 12536
    }

  m3 = x_off as f64;                                       // C 12539
  m6 = y_off as f64;                                       // C 12540

  /* Apply the specified affine transformation.
     A transform looks like:

     M1 M2 M3     X
     M4 M5 M6   * Y

     =

     M1*X + M2*Y + M3*1 = X1
     M4*X + M5*Y + M6*1 = Y1

     (In most transforms, there is another row at the bottom for
     mathematical reasons.  Since Z1 is always 1.0, the row is simply
     implied to be 0 0 1, because 0 * x + 0 * y + 1 * 1 = 1.0.  See
     the definition of matrix3x3 in image.c for some more explanations
     about this.) */

  i = 0;                                                   // C 12559
  while i < num_coordinates
    {
      let xi: sfnt_f26dot6 = (m1 * (*(x.offset (i as isize)) as f64)
			      + m2 * (*(y.offset (i as isize)) as f64)
			      + m3 * 1.0) as sfnt_f26dot6;   // C 12561
      let yi: sfnt_f26dot6 = (m4 * (*(x.offset (i as isize)) as f64)
			      + m5 * (*(y.offset (i as isize)) as f64)
			      + m6 * 1.0) as sfnt_f26dot6;   // C 12562
      *(x.offset (i as isize)) = xi;                       // C 12563
      *(y.offset (i as isize)) = yi;                       // C 12564
      i += 1;
    }
}

/* ----------------------------------------------------------------------------------------------
   C 11384-11983: `sfnt_interpret_run` - C's `static`, the interpreter's dispatch, and with it
   C 6559-8159's instruction macros.  **The next live definition is C 11996's
   `sfnt_interpret_font_program`**, and the ledger's `## C line ranges` names both boundaries.

   C's 146 macros are P-57 `macro_rules!`s *inside* this function, because their bodies name the
   caller's `interpreter` and `is_prep`; the eight that also jump into this function's own labels
   (`ELSE`/`JMPR`/`FDEF`/`IF`/`JROT`/`JROF`/`IDEF` -> `skip_step`, `CALL` -> `next_instruction`) are
   defined inside the labelled blocks that declare those labels, because a `macro_rules!` resolves a
   label at its definition site (measured: a body's `break 'label` does not see a label declared
   later in the caller).  C 6595/6625/6643's four `#else` arms name the TEST-only `pop_hook`/
   `push_hook` members the mirror omits (Owed 23), so the `#ifndef TEST` arms are what the port
   carries, and C 7195-7198's `EIF`, C 7599-7609's three round macros and the rest are C's own text.
   ---------------------------------------------------------------------------------------------- */
/// C 11396-11983: `sfnt_interpret_run`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_interpret_run (interpreter: *mut sfnt_interpreter,
			      why: sfnt_interpreter_run_context)
{                                                          // C 11396-11983
  let mut opcode: c_uchar;
  let mut is_prep: bool;

  /* Determine whether or not this is the control value program.  */
  is_prep = why == SFNT_RUN_CONTEXT_CONTROL_VALUE_PROGRAM;   // C 11403

  /* C 11405-11409's `#ifdef TEST` arm allows testing control value program instructions as well.  */
  #[cfg (feature = "test")]
  {
    if why == SFNT_RUN_CONTEXT_TEST
      { is_prep = true; }
  }

  while (*interpreter).IP < (*interpreter).num_instructions as c_int   // C 11411
    {
      opcode = *(*interpreter).instructions.offset ((*interpreter).IP as isize);   // C 11413

      /* C 11415-11418's `#ifdef TEST`: `if (interpreter->run_hook) interpreter->run_hook
	 (interpreter);` - the mirror carries neither the member nor the hook.  */

      /* C 11980's `skip_step:` is the loop's tail (its end is the `continue`) and C 11970's
	 `next_instruction:` is this inner block's end (C 11975's `IP++`); the eight jumping macros
	 below are inside both blocks, so their `break`s resolve to these two labels.  */
      'skip_step: {
	'next_instruction: {
	  /* --- C 6559-8159: the instruction macros, in C's order --------------------------- */

	  /* C 6559-6560: `STACKSIZE ()`.  */
	  macro_rules! STACKSIZE { () => {
	    (*interpreter).SP.offset_from ((*interpreter).stack)
	  } }
	  /* C 6562-6563: `TRAP (why)`.  C's argument is a string literal that reaches the
	     `const char *` parameter through C's array decay, so each call site writes
	     `c"…".as_ptr ()` (the tree's spelling for that decay).  */
	  macro_rules! TRAP { ($why:expr) => { sfnt_interpret_trap (interpreter, $why) } }
	  /* C 6565-6566: `MOVE (a, b, n)` - `memmove (a, b, (n) * sizeof (uint32_t))`.  */
	  macro_rules! MOVE { ($a:expr, $b:expr, $n:expr) => {
	    memmove ($a as *mut c_void, $b as *const c_void,
		     ($n) as size_t * std::mem::size_of::<uint32_t> ())
	  } }
	  /* C 6568-6573: `CHECK_STACK_ELEMENTS (n)`.  */
	  macro_rules! CHECK_STACK_ELEMENTS { ($n:expr) => {
	    {
	      if ((*interpreter).SP.offset_from ((*interpreter).stack) as isize) < ($n)
		{ TRAP! (c"stack underflow".as_ptr ()); }
	    }
	  } }
	  /* C 6575-6584: `CHECK_STACK_AVAILABLE (n)` - C's `char *stack_end = (char *)
	     interpreter->twilight_x;` and the pointer comparison above it.  */
	  macro_rules! CHECK_STACK_AVAILABLE { ($n:expr) => {
	    {
	      let stack_end: *mut c_char = (*interpreter).twilight_x as *mut c_char;
	      if ((*interpreter).SP.offset (($n) as isize) as *mut c_char) > stack_end
		{ TRAP! (c"stack overflow".as_ptr ()); }
	    }
	  } }
	  /* C 6586-6590: `CHECK_PREP ()` - C's `if (!is_prep) TRAP ("instruction executed not
	     valid" " outside control value program")` (the two-literal argument is one string).  */
	  macro_rules! CHECK_PREP { () => {
	    if !is_prep
	      { TRAP! (c"instruction executed not valid outside control value program".as_ptr ()); }
	  } }
	  /* C 6597-6600: `POP ()` - the live `#ifndef TEST` arm (the `#else` arm's body names the
	     TEST-only `pop_hook` member, Owed 23).  */
	  macro_rules! POP { () => {
	    if (*interpreter).SP == (*interpreter).stack
	      { TRAP! (c"stack underflow".as_ptr ()); 0 }
	    else
	      { (*interpreter).SP = (*interpreter).SP.offset (-1); *(*interpreter).SP }
	  } }
	  /* C 6602: `POP_UNCHECKED ()`.  */
	  macro_rules! POP_UNCHECKED { () => {
	    { (*interpreter).SP = (*interpreter).SP.offset (-1); *(*interpreter).SP }
	  } }
	  /* C 6620-6623: `LOOK ()`.  */
	  macro_rules! LOOK { () => {
	    if (*interpreter).SP == (*interpreter).stack
	      { TRAP! (c"stack underflow".as_ptr ()); 0 }
	    else
	      { *(*interpreter).SP.offset (-1) }
	  } }
	  /* C 6627-6635: `PUSH (value)` - the live `#ifndef TEST` arm (the `#else` arm names the
	     TEST-only `push_hook` member, Owed 23).  */
	  macro_rules! PUSH { ($value:expr) => {
	    {
	      if ((*interpreter).SP.offset (1) as *mut c_char)
		 > (*interpreter).twilight_x as *mut c_char
		{ TRAP! (c"stack overflow".as_ptr ()); }

	      *(*interpreter).SP = $value;
	      (*interpreter).SP = (*interpreter).SP.offset (1);
	    }
	  } }
	  /* C 6637-6641: `PUSH_UNCHECKED (value)`.  */
	  macro_rules! PUSH_UNCHECKED { ($value:expr) => {
	    {
	      *(*interpreter).SP = $value;
	      (*interpreter).SP = (*interpreter).SP.offset (1);
	    }
	  } }
	  /* C 6663-6670: `PUSH2_UNCHECKED (high, low)` - C's `int16_t word = (((uint8_t) high)
	     << 8 | low);` then `PUSH_UNCHECKED (word)`.  */
	  macro_rules! PUSH2_UNCHECKED { ($high:expr, $low:expr) => {
	    {
	      let word: int16_t = (((($high) as uint8_t as c_int) << 8)
				   | ($low) as c_int) as int16_t;
	      PUSH_UNCHECKED! (word as uint32_t);
	    }
	  } }

	  /* --- C 6671-8159: the instruction macros (continued below) ------------------------ */

	  /* C 6671-6677: `SRP0 ()`.  */
	  macro_rules! SRP0 { () => {
	    {
	      let mut p: uint32_t;

	      p = POP! ();
	      (*interpreter).state.rp0 = p as uint16_t;
	    }
	  } }
	  /* C 6679-6685: `SRP1 ()`.  */
	  macro_rules! SRP1 { () => {
	    {
	      let mut p: uint32_t;

	      p = POP! ();
	      (*interpreter).state.rp1 = p as uint16_t;
	    }
	  } }
	  /* C 6687-6693: `SRP2 ()`.  */
	  macro_rules! SRP2 { () => {
	    {
	      let mut p: uint32_t;

	      p = POP! ();
	      (*interpreter).state.rp2 = p as uint16_t;
	    }
	  } }
	  /* C 6695-6705: `SZP0 ()`.  */
	  macro_rules! SZP0 { () => {
	    {
	      let mut zone: uint32_t;

	      zone = POP! ();

	      if zone > 1
		{ TRAP! (c"invalid zone".as_ptr ()); }

	      (*interpreter).state.zp0 = zone as c_int;
	    }
	  } }
	  /* C 6707-6717: `SZP1 ()`.  */
	  macro_rules! SZP1 { () => {
	    {
	      let mut zone: uint32_t;

	      zone = POP! ();

	      if zone > 1
		{ TRAP! (c"invalid zone".as_ptr ()); }

	      (*interpreter).state.zp1 = zone as c_int;
	    }
	  } }
	  /* C 6719-6729: `SZP2 ()`.  */
	  macro_rules! SZP2 { () => {
	    {
	      let mut zone: uint32_t;

	      zone = POP! ();

	      if zone > 1
		{ TRAP! (c"invalid zone".as_ptr ()); }

	      (*interpreter).state.zp2 = zone as c_int;
	    }
	  } }
	  /* C 6731-6743: `SZPS ()`.  */
	  macro_rules! SZPS { () => {
	    {
	      let mut zone: uint32_t;

	      zone = POP! ();

	      if zone > 1
		{ TRAP! (c"invalid zone".as_ptr ()); }

	      (*interpreter).state.zp0 = zone as c_int;
	      (*interpreter).state.zp1 = zone as c_int;
	      (*interpreter).state.zp2 = zone as c_int;
	    }
	  } }
	  /* C 6745-6759: `SLOOP ()` - C's `MIN (65535, loop)`.  */
	  macro_rules! SLOOP { () => {
	    {
	      let mut r#loop: int32_t;

	      r#loop = POP! () as int32_t;

	      if r#loop < 0
		{ TRAP! (c"loop set to invalid value".as_ptr ()); }

	      /* N.B. loop might be greater than 65535,
		 but no reasonable font should define
		 such values.  */
	      (*interpreter).state.r#loop = MIN! (65535, r#loop) as c_ushort;
	    }
	  } }
	  /* C 6761-6768: `SMD ()`.  */
	  macro_rules! SMD { () => {
	    {
	      let mut md: sfnt_f26dot6;

	      md = POP! () as sfnt_f26dot6;

	      (*interpreter).state.minimum_distance = md;
	    }
	  } }
	  /* C 6770-6774: `ELSE ()` - C's `goto skip_step`.  */
	  macro_rules! ELSE { () => {
	    {
	      sfnt_interpret_else (interpreter);
	      break 'skip_step;
	    }
	  } }
	  /* C 6776-6789: `JMPR ()` - C's `goto skip_step`.  */
	  macro_rules! JMPR { () => {
	    {
	      let mut offset: int32_t;

	      offset = POP! () as int32_t;

	      if ((*interpreter).IP + offset) < 0
		 || ((*interpreter).IP + offset
		     > (*interpreter).num_instructions as c_int)
		{ TRAP! (c"JMPR out of bounds".as_ptr ()); }

	      (*interpreter).IP += offset;
	      break 'skip_step;
	    }
	  } }
	  /* C 6791-6798: `SCVTCI ()`.  */
	  macro_rules! SCVTCI { () => {
	    {
	      let mut cutin: sfnt_f26dot6;

	      cutin = POP! () as sfnt_f26dot6;

	      (*interpreter).state.cvt_cut_in = cutin;
	    }
	  } }
	  /* C 6800-6807: `SSWCI ()`.  */
	  macro_rules! SSWCI { () => {
	    {
	      let mut cutin: sfnt_f26dot6;

	      cutin = POP! () as sfnt_f26dot6;

	      (*interpreter).state.sw_cut_in = cutin;
	    }
	  } }
	  /* C 6809-6818: `SSW ()`.  */
	  macro_rules! SSW { () => {
	    {
	      let mut single_width: int32_t;

	      single_width = POP! () as int32_t;

	      (*interpreter).state.single_width_value
		= sfnt_mul_fixed (single_width, (*interpreter).scale);
	    }
	  } }
	  /* C 6820-6826: `DUP ()`.  */
	  macro_rules! DUP { () => {
	    {
	      let mut value: uint32_t;

	      value = LOOK! ();
	      PUSH! (value);
	    }
	  } }
	  /* C 6828-6831: `CLEAR ()`.  */
	  macro_rules! CLEAR { () => {
	    {
	      (*interpreter).SP = (*interpreter).stack;
	    }
	  } }
	  /* C 6833-6842: `SWAP ()`.  */
	  macro_rules! SWAP { () => {
	    {
	      let mut a: uint32_t;
	      let mut b: uint32_t;

	      a = POP! ();
	      b = POP! ();

	      PUSH_UNCHECKED! (a);
	      PUSH_UNCHECKED! (b);
	    }
	  } }
	  /* C 6844-6851: `DEPTH ()`.  */
	  macro_rules! DEPTH { () => {
	    {
	      let mut diff: ptrdiff_t;

	      diff = (*interpreter).SP.offset_from ((*interpreter).stack);
	      PUSH! (diff as uint32_t);
	    }
	  } }
	  /* C 6853-6864: `CINDEX ()`.  */
	  macro_rules! CINDEX { () => {
	    {
	      let mut index: int32_t;

	      index = POP! () as int32_t;

	      if index <= 0 || index as isize > STACKSIZE! ()
		{ TRAP! (c"stack overflow".as_ptr ()); }

	      PUSH_UNCHECKED! (*(*interpreter).SP.offset (-(index as isize)));
	    }
	  } }
	  /* C 6866-6880: `MINDEX ()`.  */
	  macro_rules! MINDEX { () => {
	    {
	      let mut index: int32_t;
	      let mut what: int32_t;

	      index = POP! () as int32_t;

	      if index <= 0 || index as isize > STACKSIZE! ()
		{ TRAP! (c"stack overflow".as_ptr ()); }

	      what = *(*interpreter).SP.offset (-(index as isize)) as int32_t;
	      MOVE! ((*interpreter).SP.offset (-(index as isize)),
		     (*interpreter).SP.offset (-(index as isize) + 1),
		     (index - 1) as size_t);
	      *(*interpreter).SP.offset (-1) = what as uint32_t;
	    }
	  } }
	  /* C 6882-6888: `RAW ()`.  */
	  macro_rules! RAW { () => {
	    {
	      if why != SFNT_RUN_CONTEXT_GLYPH_PROGRAM
		{ TRAP! (c"Read Advance Width without loaded glyph".as_ptr ()); }
	      PUSH! ((*interpreter).advance_width as uint32_t);
	    }
	  } }
	  /* C 6890-6916: `CALL ()` - C's `goto next_instruction`.  */
	  macro_rules! CALL { () => {
	    {
	      let mut id: uint32_t;
	      let mut i: uint32_t;
	      let mut def: *mut sfnt_interpreter_definition;

	      id = POP! ();

	      i = 0;
	      while i < (*interpreter).function_defs_size as uint32_t
		{
		  def = &raw mut *(*interpreter).function_defs.offset (i as isize);

		  if (*def).instructions.is_null ()
		    { TRAP! (c"invalid function".as_ptr ()); }

		  if (*def).opcode as uint32_t == id
		    {
		      sfnt_interpret_call (def, interpreter, why);
		      break 'next_instruction;
		    }

		  i += 1;
		}

	      TRAP! (c"invalid function".as_ptr ());
	    }
	  } }
	  /* C 6918-6953: `LOOPCALL ()` - C's `loopcall_begin:` is the macro's own label, and its
	     C 6941 `goto` leaves the search loop for it.  */
	  macro_rules! LOOPCALL { () => {
	    {
	      let mut id: uint32_t;
	      let mut n: int32_t;
	      let mut i: c_int;
	      let mut def: *mut sfnt_interpreter_definition;

	      id = POP! ();
	      n = POP! () as int32_t;

	      if n > 65535
		{ TRAP! (c"invalid LOOPCALL count".as_ptr ()); }

	      /* C 6941's `goto loopcall_begin` lands on the caller loop below, so the search is a
		 labelled block the match breaks out of.  */
	      'search: {
		i = 0;                                           // C 6932
		while i < (*interpreter).function_defs_size as c_int
		  {
		    def = &raw mut *(*interpreter).function_defs.offset (i as isize);   // C 6935

		    if (*def).instructions.is_null ()            // C 6937
		      { TRAP! (c"invalid function".as_ptr ()); }   // C 6938

		    if (*def).opcode as uint32_t == id           // C 6940
		      { break 'search; }                         // C 6941
		    i += 1;
		  }

		TRAP! (c"invalid function".as_ptr ());           // C 6944
	      }

	      /* C 6946: `loopcall_begin:`.  */
	      'loopcall_begin: loop
		{
		  /* C's `n-- <= 0` compares before the decrement (the `bidi.rs` `n_old` shape).  */
		  let n_old: int32_t = n;
		  n -= 1;
		  if n_old <= 0                                  // C 6947
		    { break; }                                   // C 6948

		  sfnt_interpret_call (def, interpreter, why);   // C 6950-6951
		  continue 'loopcall_begin;                      // C 6952
		}
	    }
	  } }
	  /* C 6955-6962: `FDEF ()` - C's `goto skip_step`.  */
	  macro_rules! FDEF { () => {
	    {
	      if why == SFNT_RUN_CONTEXT_GLYPH_PROGRAM
		{ TRAP! (c"FDEF inside glyph program".as_ptr ()); }

	      sfnt_interpret_fdef (interpreter, POP! ());
	      break 'skip_step;
	    }
	  } }
	  /* C 6964-6967: `ENDF ()`.  */
	  macro_rules! ENDF { () => {
	    {
	      TRAP! (c"stray ENDF".as_ptr ());
	    }
	  } }
	  /* C 6969-6991: `NPUSHB ()`.  */
	  macro_rules! NPUSHB { () => {
	    {
	      let mut b: c_int;
	      let mut nbytes: c_int;
	      let mut IP: c_int;
	      let mut ip: *mut c_uchar;

	      IP = (*interpreter).IP + 1;                    // C 6974
	      if IP >= (*interpreter).num_instructions as c_int   // C 6975
		{ TRAP! (c"Missing arg to NPUSHB".as_ptr ()); }   // C 6976

	      nbytes = *(*interpreter).instructions.offset (IP as isize) as c_int;   // C 6978-6979

	      if IP + 1 + nbytes > (*interpreter).num_instructions as c_int   // C 6981-6982
		{ TRAP! (c"args to NPUSHB lie outside IS".as_ptr ()); }   // C 6983

	      CHECK_STACK_AVAILABLE! (nbytes);               // C 6985
	      ip = (*interpreter).instructions;              // C 6986
	      b = IP + 1;                                    // C 6987
	      while b < IP + 1 + nbytes
		{
		  PUSH_UNCHECKED! (*ip.offset (b as isize) as uint32_t);   // C 6988
		  b += 1;
		}

	      (*interpreter).IP += nbytes + 1;               // C 6990
	    }
	  } }
	  /* C 6993-7016: `NPUSHW ()`.  */
	  macro_rules! NPUSHW { () => {
	    {
	      let mut b: c_int;
	      let mut nbytes: c_int;
	      let mut IP: c_int;
	      let mut ip: *mut c_uchar;

	      IP = (*interpreter).IP + 1;                    // C 6998
	      if IP >= (*interpreter).num_instructions as c_int   // C 6999
		{ TRAP! (c"Missing arg to NPUSHW".as_ptr ()); }   // C 7000

	      nbytes = *(*interpreter).instructions.offset (IP as isize) as c_int * 2;   // C 7002-7003

	      if IP + 1 + nbytes > (*interpreter).num_instructions as c_int   // C 7005-7006
		{ TRAP! (c"args to NPUSHW lie outside IS".as_ptr ()); }   // C 7007

	      CHECK_STACK_AVAILABLE! (nbytes / 2);           // C 7009
	      ip = (*interpreter).instructions;              // C 7010
	      b = IP + 1;                                    // C 7011
	      while b < IP + 1 + nbytes
		{
		  PUSH2_UNCHECKED! (*ip.offset (b as isize), *ip.offset ((b + 1) as isize));   // C 7013
		  b += 2;
		}

	      (*interpreter).IP += nbytes + 1;               // C 7015
	    }
	  } }
	  /* C 7018-7029: `WS ()`.  */
	  macro_rules! WS { () => {
	    {
	      let mut address: uint32_t;
	      let mut value: uint32_t;

	      value = POP! ();
	      address = POP! ();

	      if address >= (*interpreter).storage_size as uint32_t   // C 7025
		{ TRAP! (c"invalid WS".as_ptr ()); }         // C 7026

	      *(*interpreter).storage.offset (address as isize) = value;   // C 7028
	    }
	  } }
	  /* C 7031-7042: `RS ()`.  */
	  macro_rules! RS { () => {
	    {
	      let mut address: uint32_t;
	      let mut value: uint32_t;

	      address = POP! ();

	      if address >= (*interpreter).storage_size as uint32_t   // C 7037
		{ TRAP! (c"invalid RS".as_ptr ()); }         // C 7038

	      value = *(*interpreter).storage.offset (address as isize);   // C 7040
	      PUSH_UNCHECKED! (value);                       // C 7041
	    }
	  } }
	  /* C 7044-7056: `WCVTP ()`.  */
	  macro_rules! WCVTP { () => {
	    {
	      let mut value: sfnt_f26dot6;
	      let mut location: uint32_t;

	      value = POP! () as sfnt_f26dot6;
	      location = POP! ();

	      if location as usize >= (*interpreter).cvt_size   // C 7052
		{ TRAP! (c"WCVTP out of bounds".as_ptr ()); }   // C 7053

	      *(*interpreter).cvt.offset (location as isize) = value;   // C 7055
	    }
	  } }
	  /* C 7058-7070: `RCVT ()`.  */
	  macro_rules! RCVT { () => {
	    {
	      let mut value: sfnt_f26dot6;
	      let mut location: uint32_t;

	      location = POP! ();

	      if location as usize >= (*interpreter).cvt_size   // C 7065
		{ TRAP! (c"out of bounds RCVT".as_ptr ()); }   // C 7066

	      value = *(*interpreter).cvt.offset (location as isize);   // C 7068
	      PUSH_UNCHECKED! (value as uint32_t);           // C 7069
	    }
	  } }
	  /* C 7072-7075: `MPPEM ()`.  */
	  macro_rules! MPPEM { () => {
	    {
	      PUSH! ((*interpreter).ppem as uint32_t);
	    }
	  } }
	  /* C 7077-7080: `MPS ()`.  */
	  macro_rules! MPS { () => {
	    {
	      PUSH! ((*interpreter).point_size as uint32_t);
	    }
	  } }
	  /* C 7082-7085: `FLIPON ()`.  */
	  macro_rules! FLIPON { () => {
	    {
	      (*interpreter).state.auto_flip = true;
	    }
	  } }
	  /* C 7087-7090: `FLIPOFF ()`.  */
	  macro_rules! FLIPOFF { () => {
	    {
	      (*interpreter).state.auto_flip = false;
	    }
	  } }
	  /* C 7092-7095: `DEBUG ()`.  */
	  macro_rules! DEBUG { () => {
	    {
	      POP! ();  /* Value is ignored.  */
	    }
	  } }
	  /* C 7097-7105: `LT ()`.  */
	  macro_rules! LT { () => {
	    {
	      let mut e1: int32_t;
	      let mut e2: int32_t;

	      e2 = POP! () as int32_t;
	      e1 = POP! () as int32_t;

	      PUSH_UNCHECKED! (if e1 < e2 { 1 } else { 0 });
	    }
	  } }
	  /* C 7107-7115: `LTEQ ()`.  */
	  macro_rules! LTEQ { () => {
	    {
	      let mut e1: int32_t;
	      let mut e2: int32_t;

	      e2 = POP! () as int32_t;
	      e1 = POP! () as int32_t;

	      PUSH_UNCHECKED! (if e1 <= e2 { 1 } else { 0 });
	    }
	  } }
	  /* C 7117-7125: `GT ()`.  */
	  macro_rules! GT { () => {
	    {
	      let mut e1: int32_t;
	      let mut e2: int32_t;

	      e2 = POP! () as int32_t;
	      e1 = POP! () as int32_t;

	      PUSH_UNCHECKED! (if e1 > e2 { 1 } else { 0 });
	    }
	  } }
	  /* C 7127-7135: `GTEQ ()`.  */
	  macro_rules! GTEQ { () => {
	    {
	      let mut e1: int32_t;
	      let mut e2: int32_t;

	      e2 = POP! () as int32_t;
	      e1 = POP! () as int32_t;

	      PUSH_UNCHECKED! (if e1 >= e2 { 1 } else { 0 });
	    }
	  } }
	  /* C 7137-7145: `EQ ()`.  */
	  macro_rules! EQ { () => {
	    {
	      let mut e1: uint32_t;
	      let mut e2: uint32_t;

	      e1 = POP! ();
	      e2 = POP! ();

	      PUSH_UNCHECKED! (if e1 == e2 { 1 } else { 0 });
	    }
	  } }
	  /* C 7147-7155: `NEQ ()`.  */
	  macro_rules! NEQ { () => {
	    {
	      let mut e1: uint32_t;
	      let mut e2: uint32_t;

	      e1 = POP! ();
	      e2 = POP! ();

	      PUSH_UNCHECKED! (if e1 != e2 { 1 } else { 0 });
	    }
	  } }
	  /* C 7157-7169: `ODD ()` - C's `interpreter->state.round (result, interpreter)` through the
	     `Option` the mirror declares it as.  */
	  macro_rules! ODD { () => {
	    {
	      let mut e1: sfnt_f26dot6;
	      let mut result: sfnt_f26dot6;

	      e1 = POP! () as sfnt_f26dot6;
	      result = abs (e1) as sfnt_f26dot6;

	      result = (*interpreter).state.round.unwrap () (result, interpreter);
	      PUSH_UNCHECKED! (if (result & 127) == 64 { 1 } else { 0 });
	    }
	  } }
	  /* C 7171-7184: `EVEN ()`.  */
	  macro_rules! EVEN { () => {
	    {
	      let mut e1: sfnt_f26dot6;
	      let mut result: sfnt_f26dot6;
	      let mut value: uint32_t;

	      e1 = POP! () as sfnt_f26dot6;
	      result = abs (e1) as sfnt_f26dot6;

	      result = (*interpreter).state.round.unwrap () (result, interpreter);
	      value = if (result & 127) == 64 { 0 } else { 1 };
	      PUSH_UNCHECKED! (value);
	    }
	  } }
	  /* C 7186-7193: `IF ()` - C's `goto skip_step`.  */
	  macro_rules! IF { () => {
	    {
	      let mut condition: uint32_t;

	      condition = POP! ();
	      sfnt_interpret_if (interpreter, condition != 0);   // C's `uint32_t` -> `bool`
	      break 'skip_step;
	    }
	  } }
	  /* C 7195-7198: `EIF ()` - C's body is empty.  */
	  macro_rules! EIF { () => {
	    {
	    }
	  } }
	  /* C 7200-7208: `AND ()`.  */
	  macro_rules! AND { () => {
	    {
	      let mut e1: uint32_t;
	      let mut e2: uint32_t;

	      e1 = POP! ();
	      e2 = POP! ();

	      PUSH_UNCHECKED! (if e1 != 0 && e2 != 0 { 1 } else { 0 });
	    }
	  } }
	  /* C 7210-7218: `OR ()`.  */
	  macro_rules! OR { () => {
	    {
	      let mut e1: uint32_t;
	      let mut e2: uint32_t;

	      e1 = POP! ();
	      e2 = POP! ();

	      PUSH_UNCHECKED! (if e1 != 0 || e2 != 0 { 1 } else { 0 });
	    }
	  } }
	  /* C 7220-7227: `NOT ()`.  */
	  macro_rules! NOT { () => {
	    {
	      let mut e1: uint32_t;

	      e1 = POP! ();

	      PUSH_UNCHECKED! (if e1 == 0 { 1 } else { 0 });
	    }
	  } }
	  /* C 7229-7236: `SDB ()`.  */
	  macro_rules! SDB { () => {
	    {
	      let mut base: uint32_t;

	      base = POP! ();

	      (*interpreter).state.delta_base = base as c_ushort;
	    }
	  } }
	  /* C 7238-7248: `SDS ()`.  */
	  macro_rules! SDS { () => {
	    {
	      let mut shift: uint32_t;

	      shift = POP! ();

	      if shift > 6
		{ TRAP! (c"invalid delta shift".as_ptr ()); }

	      (*interpreter).state.delta_shift = shift as c_ushort;
	    }
	  } }
	  /* C 7250-7258: `ADD ()`.  */
	  macro_rules! ADD { () => {
	    {
	      let mut n1: sfnt_f26dot6;
	      let mut n2: sfnt_f26dot6;

	      n1 = POP! () as sfnt_f26dot6;
	      n2 = POP! () as sfnt_f26dot6;

	      PUSH_UNCHECKED! (sfnt_add! (n1, n2) as uint32_t);
	    }
	  } }
	  /* C 7260-7268: `SUB ()`.  */
	  macro_rules! SUB { () => {
	    {
	      let mut n2: sfnt_f26dot6;
	      let mut n1: sfnt_f26dot6;

	      n2 = POP! () as sfnt_f26dot6;
	      n1 = POP! () as sfnt_f26dot6;

	      PUSH_UNCHECKED! (sfnt_sub! (n1, n2) as uint32_t);
	    }
	  } }
	  /* C 7270-7281: `DIV ()`.  */
	  macro_rules! DIV { () => {
	    {
	      let mut n2: sfnt_f26dot6;
	      let mut n1: sfnt_f26dot6;

	      n2 = POP! () as sfnt_f26dot6;
	      n1 = POP! () as sfnt_f26dot6;

	      if n2 == 0
		{ TRAP! (c"DIV by 0".as_ptr ()); }

	      PUSH_UNCHECKED! (sfnt_div_f26dot6 (n1, n2) as uint32_t);
	    }
	  } }
	  /* C 7283-7292: `MUL ()`.  */
	  macro_rules! MUL { () => {
	    {
	      let mut n2: sfnt_f26dot6;
	      let mut n1: sfnt_f26dot6;
	      let mut r: sfnt_f26dot6;

	      n2 = POP! () as sfnt_f26dot6;
	      n1 = POP! () as sfnt_f26dot6;

	      r = sfnt_mul_f26dot6_round (n2, n1);
	      PUSH_UNCHECKED! (r as uint32_t);
	    }
	  } }
	  /* C 7294-7304: `ABS ()`.  */
	  macro_rules! ABS { () => {
	    {
	      let mut n: sfnt_f26dot6;

	      n = POP! () as sfnt_f26dot6;

	      if n == INT32_MIN
		{ PUSH_UNCHECKED! (0); }
	      else
		{ PUSH_UNCHECKED! (if n < 0 { -n } else { n } as uint32_t); }
	    }
	  } }
	  /* C 7306-7316: `NEG ()`.  */
	  macro_rules! NEG { () => {
	    {
	      let mut n: sfnt_f26dot6;

	      n = POP! () as sfnt_f26dot6;

	      if n == INT32_MIN
		{ PUSH_UNCHECKED! (0); }
	      else
		{ PUSH_UNCHECKED! (-n as uint32_t); }
	    }
	  } }
	  /* C 7318-7324: `FLOOR ()`.  */
	  macro_rules! FLOOR { () => {
	    {
	      let mut n: sfnt_f26dot6;

	      n = POP! () as sfnt_f26dot6;
	      PUSH_UNCHECKED! (sfnt_floor_f26dot6 (n) as uint32_t);
	    }
	  } }
	  /* C 7326-7332: `CEILING ()`.  */
	  macro_rules! CEILING { () => {
	    {
	      let mut n: sfnt_f26dot6;

	      n = POP! () as sfnt_f26dot6;
	      PUSH_UNCHECKED! (sfnt_ceil_f26dot6 (n) as uint32_t);
	    }
	  } }
	  /* C 7334-7348: `WCVTF ()`.  */
	  macro_rules! WCVTF { () => {
	    {
	      let mut value: int32_t;
	      let mut location: uint32_t;

	      value = POP! () as int32_t;
	      location = POP! ();

	      if location as usize >= (*interpreter).cvt_size   // C 7342
		{ TRAP! (c"WCVTF out of bounds".as_ptr ()); }   // C 7343

	      *(*interpreter).cvt.offset (location as isize)
		= sfnt_mul_fixed (value, (*interpreter).scale);   // C 7345-7347
	    }
	  } }
	  /* C 7350-7368: `JROT ()` - C's `break` leaves the switch, which is the dispatch loop
	     (C 11970's `next_instruction:` is what a switch `break` falls to).  */
	  macro_rules! JROT { () => {
	    {
	      let mut e: uint32_t;
	      let mut offset: int32_t;

	      e = POP! ();
	      offset = POP! () as int32_t;

	      if e == 0
		{ break; }                                   // C 7359

	      if ((*interpreter).IP + offset) < 0
		 || ((*interpreter).IP + offset
		     > (*interpreter).num_instructions as c_int)   // C 7361-7363
		{ TRAP! (c"JMPR out of bounds".as_ptr ()); }   // C 7364

	      (*interpreter).IP += offset;                   // C 7366
	      break 'skip_step;                              // C 7367
	    }
	  } }
	  /* C 7370-7388: `JROF ()`.  */
	  macro_rules! JROF { () => {
	    {
	      let mut e: uint32_t;
	      let mut offset: int32_t;

	      e = POP! ();
	      offset = POP! () as int32_t;

	      if e != 0
		{ break; }                                   // C 7379

	      if ((*interpreter).IP + offset) < 0
		 || ((*interpreter).IP + offset
		     > (*interpreter).num_instructions as c_int)   // C 7381-7383
		{ TRAP! (c"JMPR out of bounds".as_ptr ()); }   // C 7384

	      (*interpreter).IP += offset;                   // C 7386
	      break 'skip_step;                              // C 7387
	    }
	  } }
	  /* C 7390-7393: `ILLEGAL_INSTRUCTION ()`.  */
	  macro_rules! ILLEGAL_INSTRUCTION { () => {
	    {
	      TRAP! (c"MS reserved illegal instruction".as_ptr ());
	    }
	  } }
	  /* C 7395-7401: `SCANCTRL ()`.  */
	  macro_rules! SCANCTRL { () => {
	    {
	      let mut value: uint32_t;

	      value = POP! ();
	      (*interpreter).state.scan_control = value as c_int;
	    }
	  } }
	  /* C 7410-7429: `GETINFO ()`.  */
	  macro_rules! GETINFO { () => {
	    {
	      let mut selector: uint32_t;
	      let mut k: uint32_t;

	      selector = POP! ();

	      k = 0;                                         // C 7416

	      if (selector & 1) != 0                         // C 7418
		{ k |= 0o2; }

	      if (selector & 8) != 0 && !(*interpreter).norm_coords.is_null()   // C 7421-7422
		{ k |= 0o2000; }                             // C 7423

	      if (selector & 32) != 0                        // C 7425
		{ k |= 0o10000; }                            // C 7426

	      PUSH_UNCHECKED! (k);                           // C 7428
	    }
	  } }
	  /* C 7431-7438: `IDEF ()` - C's `goto skip_step`.  */
	  macro_rules! IDEF { () => {
	    {
	      if why == SFNT_RUN_CONTEXT_GLYPH_PROGRAM
		{ TRAP! (c"IDEF inside glyph program".as_ptr ()); }

	      sfnt_interpret_idef (interpreter, POP! ());
	      break 'skip_step;
	    }
	  } }
	  /* C 7440-7453: `ROLL ()`.  */
	  macro_rules! ROLL { () => {
	    {
	      let mut a: uint32_t;
	      let mut b: uint32_t;
	      let mut c: uint32_t;

	      CHECK_STACK_ELEMENTS! (3);                     // C 7444

	      a = POP_UNCHECKED! ();                         // C 7446
	      b = POP_UNCHECKED! ();                         // C 7447
	      c = POP_UNCHECKED! ();                         // C 7448

	      PUSH_UNCHECKED! (b);                           // C 7450
	      PUSH_UNCHECKED! (a);                           // C 7451
	      PUSH_UNCHECKED! (c);                           // C 7452
	    }
	  } }
	  /* C 7455-7463: `INS_MAX ()`.  */
	  macro_rules! INS_MAX { () => {
	    {
	      let mut e1: int32_t;
	      let mut e2: int32_t;

	      e1 = POP! () as int32_t;
	      e2 = POP! () as int32_t;

	      PUSH_UNCHECKED! (MAX! (e1, e2) as uint32_t);
	    }
	  } }
	  /* C 7465-7473: `INS_MIN ()`.  */
	  macro_rules! INS_MIN { () => {
	    {
	      let mut e1: int32_t;
	      let mut e2: int32_t;

	      e1 = POP! () as int32_t;
	      e2 = POP! () as int32_t;

	      PUSH_UNCHECKED! (MIN! (e1, e2) as uint32_t);
	    }
	  } }
	  /* C 7475-7478: `SCANTYPE ()` - C's body is the one `POP ()`.  */
	  macro_rules! SCANTYPE { () => {
	    {
	      POP! ();
	    }
	  } }
	  /* C 7480-7497: `INSTCTRL ()` - C's `break` leaves the switch (the dispatch loop).  */
	  macro_rules! INSTCTRL { () => {
	    {
	      let mut s: uint32_t;
	      let mut v: uint32_t;

	      CHECK_PREP! ();                                // C 7484
	      s = POP! ();                                   // C 7485
	      v = POP! ();                                   // C 7486

	      if s == 0 || s > 2                            // C 7488
		{ break; }                                   // C 7489

	      (*interpreter).state.instruct_control
		&= !((1u32 << s) as u8);                     // C 7491-7492

	      if v != 0                                     // C 7494
		{ (*interpreter).state.instruct_control
		    |= (1u32 << s) as u8; }                  // C 7495-7496
	    }
	  } }
	  /* C 7502-7519: `GXAXIS ()` - C's `norm_coords[i] / 4` promotes the `sfnt_fixed` element
	     and then converts to `uint32_t`.  */
	  macro_rules! GXAXIS { () => {
	    {
	      let mut v: uint32_t;
	      let mut i: c_int;
	      let mut naxis: c_int;

	      naxis = (*interpreter).n_axis;                 // C 7507
	      CHECK_STACK_AVAILABLE! (naxis);                // C 7508

	      i = 0;                                         // C 7510
	      while i < naxis
		{
		  if !(*interpreter).norm_coords.is_null ()  // C 7512
		    { v = ((*(*interpreter).norm_coords.offset (i as isize)) as c_int / 4) as uint32_t; }   // C 7513
		  else
		    { v = 0; }                               // C 7515

		  PUSH_UNCHECKED! (v);                       // C 7517
		  i += 1;
		}
	    }
	  } }
	  /* C 7521-7539: `PUSHB ()` - C's `opcode` here is the dispatch's own opcode.  */
	  macro_rules! PUSHB { () => {
	    {
	      let mut b: c_int;
	      let mut nbytes: c_int;
	      let mut IP: c_int;
	      let mut ip: *mut c_uchar;

	      IP = (*interpreter).IP;                        // C 7526
	      nbytes = opcode as c_int - 0xb0 + 1;           // C 7527

	      if IP + nbytes + 1 > (*interpreter).num_instructions as c_int   // C 7529-7530
		{ TRAP! (c"args to PUSHB lie outside IS".as_ptr ()); }   // C 7531

	      CHECK_STACK_AVAILABLE! (nbytes);               // C 7533
	      ip = (*interpreter).instructions;              // C 7534
	      b = IP + 1;                                    // C 7535
	      while b < IP + nbytes + 1
		{
		  PUSH_UNCHECKED! (*ip.offset (b as isize) as uint32_t);   // C 7536
		  b += 1;
		}

	      (*interpreter).IP += nbytes;                   // C 7538
	    }
	  } }
	  /* C 7541-7560: `PUSHW ()`.  */
	  macro_rules! PUSHW { () => {
	    {
	      let mut b: c_int;
	      let mut nbytes: c_int;
	      let mut IP: c_int;
	      let mut ip: *mut c_uchar;

	      IP = (*interpreter).IP;                        // C 7546
	      nbytes = (opcode as c_int - 0xb8 + 1) * 2;     // C 7547

	      if IP + 1 + nbytes > (*interpreter).num_instructions as c_int   // C 7549-7550
		{ TRAP! (c"args to PUSHW lie outside IS".as_ptr ()); }   // C 7551

	      CHECK_STACK_AVAILABLE! (nbytes / 2);           // C 7553
	      ip = (*interpreter).instructions;              // C 7554
	      b = IP + 1;                                    // C 7555
	      while b < IP + nbytes + 1
		{
		  PUSH2_UNCHECKED! (*ip.offset (b as isize), *ip.offset ((b + 1) as isize));   // C 7557
		  b += 2;
		}

	      (*interpreter).IP += nbytes;                   // C 7559
	    }
	  } }
	  /* C 7566-7577: `ROUND ()`.  */
	  macro_rules! ROUND { () => {
	    {
	      let mut n: sfnt_f26dot6;
	      let mut result: sfnt_f26dot6;

	      n = POP! () as sfnt_f26dot6;
	      result = abs (n) as sfnt_f26dot6;

	      result = (*interpreter).state.round.unwrap () (result, interpreter);
	      PUSH_UNCHECKED! (if n < 0 { -result } else { result } as uint32_t);
	    }
	  } }
	  /* C 7579-7585: `NROUND ()`.  */
	  macro_rules! NROUND { () => {
	    {
	      let mut n: sfnt_f26dot6;

	      n = POP! () as sfnt_f26dot6;
	      PUSH_UNCHECKED! (n as uint32_t);
	    }
	  } }
	  /* C 7587-7591: `ROFF ()`.  */
	  macro_rules! ROFF { () => {
	    {
	      (*interpreter).state.round_state = 5;
	      sfnt_validate_gs (&raw mut (*interpreter).state);
	    }
	  } }
	  /* C 7593-7597: `RUTG ()`.  */
	  macro_rules! RUTG { () => {
	    {
	      (*interpreter).state.round_state = 4;
	      sfnt_validate_gs (&raw mut (*interpreter).state);
	    }
	  } }
	  /* C 7599-7603: `RDTG ()`.  */
	  macro_rules! RDTG { () => {
	    {
	      (*interpreter).state.round_state = 3;
	      sfnt_validate_gs (&raw mut (*interpreter).state);
	    }
	  } }
	  /* C 7605-7609: `RTG ()`.  */
	  macro_rules! RTG { () => {
	    {
	      (*interpreter).state.round_state = 1;
	      sfnt_validate_gs (&raw mut (*interpreter).state);
	    }
	  } }
	  /* C 7611-7615: `RTHG ()`.  */
	  macro_rules! RTHG { () => {
	    {
	      (*interpreter).state.round_state = 0;
	      sfnt_validate_gs (&raw mut (*interpreter).state);
	    }
	  } }
	  /* C 7617-7621: `RTDG ()`.  */
	  macro_rules! RTDG { () => {
	    {
	      (*interpreter).state.round_state = 2;
	      sfnt_validate_gs (&raw mut (*interpreter).state);
	    }
	  } }
	  /* C 7623-7633: `SROUND ()`.  */
	  macro_rules! SROUND { () => {
	    {
	      let mut operand: uint32_t;

	      operand = POP! ();
	      sfnt_set_srounding_state (interpreter, operand, 0x4000);   // C 7628-7630
	      (*interpreter).state.round_state = 6;          // C 7631
	      sfnt_validate_gs (&raw mut (*interpreter).state);   // C 7632
	    }
	  } }
	  /* C 7635-7645: `S45ROUND ()`.  */
	  macro_rules! S45ROUND { () => {
	    {
	      let mut operand: uint32_t;

	      operand = POP! ();
	      sfnt_set_srounding_state (interpreter, operand, 0x5a82);   // C 7640-7642
	      (*interpreter).state.round_state = 7;          // C 7643
	      sfnt_validate_gs (&raw mut (*interpreter).state);   // C 7644
	    }
	  } }
	  /* C 7654-7670: `DELTAC1 ()` - C's `deltac1_start:` is the loop's head, its `goto` is that
	     loop's `continue`, and its `break` leaves the switch (the dispatch loop).  */
	  macro_rules! DELTAC1 { () => {
	    {
	      let mut operand1: uint32_t;
	      let mut operand2: uint32_t;
	      let mut n: uint32_t;

	      n = POP! ();                                   // C 7658

	      loop
		{
		  if n == 0                                  // C 7661
		    { break; }                               // C 7662

		  operand1 = POP! ();                        // C 7664
		  operand2 = POP! ();                        // C 7665
		  sfnt_deltac (1, interpreter, operand1, operand2 as c_uchar);   // C 7666-7667
		  n -= 1;                                    // C 7668
		}
	    }
	  } }
	  /* C 7672-7688: `DELTAC2 ()`.  */
	  macro_rules! DELTAC2 { () => {
	    {
	      let mut operand1: uint32_t;
	      let mut operand2: uint32_t;
	      let mut n: uint32_t;

	      n = POP! ();

	      loop
		{
		  if n == 0
		    { break; }

		  operand1 = POP! ();
		  operand2 = POP! ();
		  sfnt_deltac (2, interpreter, operand1, operand2 as c_uchar);
		  n -= 1;
		}
	    }
	  } }
	  /* C 7690-7706: `DELTAC3 ()`.  */
	  macro_rules! DELTAC3 { () => {
	    {
	      let mut operand1: uint32_t;
	      let mut operand2: uint32_t;
	      let mut n: uint32_t;

	      n = POP! ();

	      loop
		{
		  if n == 0
		    { break; }

		  operand1 = POP! ();
		  operand2 = POP! ();
		  sfnt_deltac (3, interpreter, operand1, operand2 as c_uchar);
		  n -= 1;
		}
	    }
	  } }
	  /* C 7708-7723: `DELTAP1 ()`.  */
	  macro_rules! DELTAP1 { () => {
	    {
	      let mut n: uint32_t;
	      let mut argn: uint32_t;
	      let mut pn: uint32_t;

	      n = POP! ();

	      loop
		{
		  if n == 0
		    { break; }

		  pn = POP! ();
		  argn = POP! ();
		  sfnt_deltap (1, interpreter, argn as c_uchar, pn);
		  n -= 1;
		}
	    }
	  } }
	  /* C 7725-7740: `DELTAP2 ()`.  */
	  macro_rules! DELTAP2 { () => {
	    {
	      let mut n: uint32_t;
	      let mut argn: uint32_t;
	      let mut pn: uint32_t;

	      n = POP! ();

	      loop
		{
		  if n == 0
		    { break; }

		  pn = POP! ();
		  argn = POP! ();
		  sfnt_deltap (2, interpreter, argn as c_uchar, pn);
		  n -= 1;
		}
	    }
	  } }
	  /* C 7742-7757: `DELTAP3 ()`.  */
	  macro_rules! DELTAP3 { () => {
	    {
	      let mut n: uint32_t;
	      let mut argn: uint32_t;
	      let mut pn: uint32_t;

	      n = POP! ();

	      loop
		{
		  if n == 0
		    { break; }

		  pn = POP! ();
		  argn = POP! ();
		  sfnt_deltap (3, interpreter, argn as c_uchar, pn);
		  n -= 1;
		}
	    }
	  } }
	  /* C 7763-7766: `AA ()` - C's body is the one `POP ()`.  */
	  macro_rules! AA { () => {
	    {
	      POP! ();
	    }
	  } }
	  /* C 7768-7771: `SANGW ()` - C's body is the one `POP ()`.  */
	  macro_rules! SANGW { () => {
	    {
	      POP! ();
	    }
	  } }
	  /* C 7777-7778: `PROJECT (x, y)` - C's argument order is `(x, y)`.  */
	  macro_rules! PROJECT { ($x:expr, $y:expr) => {
	    sfnt_project_vector (interpreter, $x, $y)
	  } }
	  /* C 7780-7781: `DUAL_PROJECT (x, y)`.  */
	  macro_rules! DUAL_PROJECT { ($x:expr, $y:expr) => {
	    sfnt_dual_project_vector (interpreter, $x, $y)
	  } }
	  /* C 7783-7789: `SVTCAy ()`.  */
	  macro_rules! SVTCAy { () => {
	    {
	      sfnt_set_freedom_vector (interpreter, 0, 0o40000 as sfnt_f2dot14);
	      sfnt_set_projection_vector (interpreter, 0, 0o40000 as sfnt_f2dot14);
	    }
	  } }
	  /* C 7791-7797: `SVTCAx ()`.  */
	  macro_rules! SVTCAx { () => {
	    {
	      sfnt_set_freedom_vector (interpreter, 0o40000 as sfnt_f2dot14, 0);
	      sfnt_set_projection_vector (interpreter, 0o40000 as sfnt_f2dot14, 0);
	    }
	  } }
	  /* C 7799-7803: `SPvTCAy ()`.  */
	  macro_rules! SPvTCAy { () => {
	    {
	      sfnt_set_projection_vector (interpreter, 0, 0o40000 as sfnt_f2dot14);
	    }
	  } }
	  /* C 7805-7809: `SPvTCAx ()`.  */
	  macro_rules! SPvTCAx { () => {
	    {
	      sfnt_set_projection_vector (interpreter, 0o40000 as sfnt_f2dot14, 0);
	    }
	  } }
	  /* C 7811-7815: `SFvTCAy ()`.  */
	  macro_rules! SFvTCAy { () => {
	    {
	      sfnt_set_freedom_vector (interpreter, 0, 0o40000 as sfnt_f2dot14);
	    }
	  } }
	  /* C 7817-7821: `SFvTCAx ()`.  */
	  macro_rules! SFvTCAx { () => {
	    {
	      sfnt_set_freedom_vector (interpreter, 0o40000 as sfnt_f2dot14, 0);
	    }
	  } }
	  /* C 7823-7839: `SPVTL ()`.  */
	  macro_rules! SPVTL { () => {
	    {
	      let mut vector: sfnt_unit_vector = std::mem::zeroed ();
	      let mut p2: uint32_t;
	      let mut p1: uint32_t;

	      p2 = POP! ();
	      p1 = POP! ();

	      sfnt_line_to_vector (interpreter, p2, p1, &raw mut vector,
				   opcode == 0x07, false);   // C 7831-7834

	      sfnt_save_projection_vector (interpreter, &raw mut vector, false);   // C 7836-7838
	    }
	  } }
	  /* C 7841-7856: `SFVTL ()`.  */
	  macro_rules! SFVTL { () => {
	    {
	      let mut vector: sfnt_unit_vector = std::mem::zeroed ();
	      let mut p2: uint32_t;
	      let mut p1: uint32_t;

	      p2 = POP! ();
	      p1 = POP! ();

	      sfnt_line_to_vector (interpreter, p2, p1, &raw mut vector,
				   opcode == 0x09, false);   // C 7849-7852

	      sfnt_save_freedom_vector (interpreter, &raw mut vector);   // C 7854-7855
	    }
	  } }
	  /* C 7858-7867: `SPVFS ()`.  */
	  macro_rules! SPVFS { () => {
	    {
	      let mut y: uint32_t;
	      let mut x: uint32_t;

	      y = POP! ();
	      x = POP! ();

	      sfnt_set_projection_vector (interpreter, x as sfnt_f2dot14, y as sfnt_f2dot14);   // C 7865-7866
	    }
	  } }
	  /* C 7869-7878: `SFVFS ()` - C's two locals are `uint16_t`s.  */
	  macro_rules! SFVFS { () => {
	    {
	      let mut y: uint16_t;
	      let mut x: uint16_t;

	      y = POP! () as uint16_t;
	      x = POP! () as uint16_t;

	      sfnt_set_freedom_vector (interpreter, x as sfnt_f2dot14, y as sfnt_f2dot14);   // C 7876-7877
	    }
	  } }
	  /* C 7880-7889: `GPV ()`.  */
	  macro_rules! GPV { () => {
	    {
	      let mut vector: sfnt_unit_vector;

	      vector = (*interpreter).state.projection_vector;   // C 7884-7885

	      PUSH! (vector.x as int32_t as uint32_t);       // C 7887
	      PUSH! (vector.y as int32_t as uint32_t);       // C 7888
	    }
	  } }
	  /* C 7891-7900: `GFV ()`.  */
	  macro_rules! GFV { () => {
	    {
	      let mut vector: sfnt_unit_vector;

	      vector = (*interpreter).state.freedom_vector;   // C 7895-7896

	      PUSH! (vector.x as int32_t as uint32_t);       // C 7898
	      PUSH! (vector.y as int32_t as uint32_t);       // C 7899
	    }
	  } }
	  /* C 7902-7908: `SFVTPV ()`.  */
	  macro_rules! SFVTPV { () => {
	    {
	      (*interpreter).state.freedom_vector
		= (*interpreter).state.projection_vector;    // C 7904-7905

	      sfnt_validate_gs (&raw mut (*interpreter).state);   // C 7907
	    }
	  } }
	  /* C 7910-7924: `ISECT ()`.  */
	  macro_rules! ISECT { () => {
	    {
	      let mut a0: uint32_t;
	      let mut a1: uint32_t;
	      let mut b0: uint32_t;
	      let mut b1: uint32_t;
	      let mut p: uint32_t;

	      CHECK_STACK_ELEMENTS! (5);                     // C 7914

	      a0 = POP_UNCHECKED! ();                        // C 7916
	      a1 = POP_UNCHECKED! ();                        // C 7917
	      b0 = POP_UNCHECKED! ();                        // C 7918
	      b1 = POP_UNCHECKED! ();                        // C 7919
	      p = POP_UNCHECKED! ();                         // C 7920

	      sfnt_interpret_isect (interpreter, a0, a1, b0, b1, p);   // C 7922-7923
	    }
	  } }
	  /* C 7926-7935: `ALIGNPTS ()`.  */
	  macro_rules! ALIGNPTS { () => {
	    {
	      let mut p1: uint32_t;
	      let mut p2: uint32_t;

	      p1 = POP! ();
	      p2 = POP! ();

	      sfnt_interpret_alignpts (interpreter, p1, p2);   // C 7933-7934
	    }
	  } }
	  /* C 7937-7943: `UTP ()`.  */
	  macro_rules! UTP { () => {
	    {
	      let mut p: uint32_t;

	      p = POP! ();
	      sfnt_interpret_utp (interpreter, p);
	    }
	  } }
	  /* C 7945-7952: `MDAP ()`.  */
	  macro_rules! MDAP { () => {
	    {
	      let mut p: uint32_t;

	      p = POP! ();
	      sfnt_interpret_mdap (interpreter, p, opcode as uint32_t);   // C 7950-7951
	    }
	  } }
	  /* C 7954-7957: `IUP ()`.  */
	  macro_rules! IUP { () => {
	    {
	      sfnt_interpret_iup (interpreter, opcode);
	    }
	  } }
	  /* C 7959-7962: `SHP ()`.  */
	  macro_rules! SHP { () => {
	    {
	      sfnt_interpret_shp (interpreter, opcode as uint32_t);
	    }
	  } }
	  /* C 7964-7972: `SHC ()`.  */
	  macro_rules! SHC { () => {
	    {
	      let mut contour: uint32_t;

	      contour = POP! ();

	      sfnt_interpret_shc (interpreter, contour, opcode as uint32_t);   // C 7970-7971
	    }
	  } }
	  /* C 7974-7985: `SHZ ()`.  */
	  macro_rules! SHZ { () => {
	    {
	      let mut e: uint32_t;

	      e = POP! ();

	      if e > 1
		{ TRAP! (c"invalid zone!".as_ptr ()); }

	      sfnt_interpret_shz (interpreter, e, opcode as uint32_t);   // C 7983-7984
	    }
	  } }
	  /* C 7987-8005: `SHPIX ()` - C's `while (interpreter->state.loop--)`.  */
	  macro_rules! SHPIX { () => {
	    {
	      let mut pixels: sfnt_f26dot6;
	      let mut dx: sfnt_f26dot6 = 0;
	      let mut dy: sfnt_f26dot6 = 0;
	      let mut p: uint32_t;

	      pixels = POP! () as sfnt_f26dot6;
	      sfnt_scale_by_freedom_vector (interpreter, pixels, &raw mut dx,
					    &raw mut dy);   // C 7993-7995

	      while { let old = (*interpreter).state.r#loop;
		      (*interpreter).state.r#loop = (*interpreter).state.r#loop.wrapping_sub (1);
		      old != 0 }                           // C 7997
		{
		  p = POP! ();                             // C 7999
		  sfnt_direct_move_zp2 (interpreter, p, dx, dy);   // C 8000-8001
		}

	      (*interpreter).state.r#loop = 1;               // C 8004
	    }
	  } }
	  /* C 8007-8010: `IP ()` - C's body is the one `sfnt_interpret_ip` call.  */
	  macro_rules! IP { () => {
	    {
	      sfnt_interpret_ip (interpreter);
	    }
	  } }
	  /* C 8012-8022: `MSIRP ()`.  */
	  macro_rules! MSIRP { () => {
	    {
	      let mut d: sfnt_f26dot6;
	      let mut p: uint32_t;

	      d = POP! () as sfnt_f26dot6;
	      p = POP! ();

	      sfnt_interpret_msirp (interpreter, d, p, opcode);   // C 8020-8021
	    }
	  } }
	  /* C 8024-8027: `ALIGNRP ()`.  */
	  macro_rules! ALIGNRP { () => {
	    {
	      sfnt_interpret_alignrp (interpreter);
	    }
	  } }
	  /* C 8029-8039: `MIAP ()`.  */
	  macro_rules! MIAP { () => {
	    {
	      let mut cvt: uint32_t;
	      let mut p: uint32_t;

	      cvt = POP! ();
	      p = POP! ();

	      sfnt_interpret_miap (interpreter, cvt, p, opcode);   // C 8037-8038
	    }
	  } }
	  /* C 8041-8058: `GC ()`.  */
	  macro_rules! GC { () => {
	    {
	      let mut p: uint32_t;
	      let mut x: sfnt_f26dot6 = 0;
	      let mut y: sfnt_f26dot6 = 0;
	      let mut value: sfnt_f26dot6;
	      let mut org_x: sfnt_f26dot6 = 0;
	      let mut org_y: sfnt_f26dot6 = 0;

	      p = POP! ();                                   // C 8047

	      sfnt_address_zp2 (interpreter, p, &raw mut x, &raw mut y,
				&raw mut org_x, &raw mut org_y);   // C 8049-8050

	      if opcode == 0x47                              // C 8052
		{ value = DUAL_PROJECT! (org_x, org_y); }    // C 8053
	      else
		{ value = PROJECT! (x, y); }                 // C 8055

	      PUSH_UNCHECKED! (value as uint32_t);           // C 8057
	    }
	  } }
	  /* C 8060-8069: `SCFS ()`.  */
	  macro_rules! SCFS { () => {
	    {
	      let mut p: uint32_t;
	      let mut c: sfnt_f26dot6;

	      c = POP! () as sfnt_f26dot6;
	      p = POP! ();

	      sfnt_interpret_scfs (interpreter, p, c);       // C 8068
	    }
	  } }
	  /* C 8071-8084: `MD ()`.  */
	  macro_rules! MD { () => {
	    {
	      let mut p1: uint32_t;
	      let mut p2: uint32_t;
	      let mut distance: sfnt_f26dot6;

	      p2 = POP! ();                                  // C 8076
	      p1 = POP! ();                                  // C 8077

	      distance = sfnt_measure_distance (interpreter, p1, p2, opcode);   // C 8079-8082
	      PUSH_UNCHECKED! (distance as uint32_t);        // C 8083
	    }
	  } }
	  /* C 8086-8089: `FLIPPT ()`.  */
	  macro_rules! FLIPPT { () => {
	    {
	      sfnt_interpret_flippt (interpreter);
	    }
	  } }
	  /* C 8091-8100: `FLIPRGOFF ()`.  */
	  macro_rules! FLIPRGOFF { () => {
	    {
	      let mut h: uint32_t;
	      let mut l: uint32_t;

	      h = POP! ();
	      l = POP! ();

	      sfnt_interpret_fliprgoff (interpreter, h, l);   // C 8098-8099
	    }
	  } }
	  /* C 8102-8111: `FLIPRGON ()`.  */
	  macro_rules! FLIPRGON { () => {
	    {
	      let mut h: uint32_t;
	      let mut l: uint32_t;

	      h = POP! ();
	      l = POP! ();

	      sfnt_interpret_fliprgon (interpreter, h, l);   // C 8109-8110
	    }
	  } }
	  /* C 8113-8138: `SDPVTL ()`.  */
	  macro_rules! SDPVTL { () => {
	    {
	      let mut vector: sfnt_unit_vector = std::mem::zeroed ();
	      let mut p2: uint32_t;
	      let mut p1: uint32_t;

	      p2 = POP! ();
	      p1 = POP! ();

	      sfnt_line_to_vector (interpreter, p2, p1, &raw mut vector,
				   opcode == 0x87, false);   // C 8121-8124

	      sfnt_save_projection_vector (interpreter, &raw mut vector, false);   // C 8126-8128

	      sfnt_line_to_vector (interpreter, p2, p1, &raw mut vector,
				   opcode == 0x87, true);    // C 8130-8133

	      sfnt_save_projection_vector (interpreter, &raw mut vector, true);   // C 8135-8137
	    }
	  } }
	  /* C 8140-8143: `MIRP ()`.  */
	  macro_rules! MIRP { () => {
	    {
	      sfnt_interpret_mirp (interpreter, opcode as uint32_t);
	    }
	  } }
	  /* C 8145-8148: `MDRP ()`.  */
	  macro_rules! MDRP { () => {
	    {
	      sfnt_interpret_mdrp (interpreter, opcode as uint32_t);
	    }
	  } }
	  /* C 8152-8154: `NOT_IMPLEMENTED ()` - C's body has no braces, so its call sites' `;`
	     follows the expansion (the port keeps the same shape).  */
	  macro_rules! NOT_IMPLEMENTED { () => {
	    sfnt_interpret_unimplemented (interpreter, opcode, why)
	  } }
	  /* C 11420's `switch (opcode)`: the dispatch is a `match` with C's cases as arms (146 of
	     them, plus C 11949's `default:`).  The `loop` is what a `break` inside a macro body
	     leaves: C's `break` leaves the *switch*, and C 11970's `next_instruction:` is where
	     both that `break` and this `loop`'s `break` land (the `IP++` below).  */
	  loop
	    {
	      match opcode
		{
	      0x00 => { SVTCAy! (); }		/* SVTCA y  */
	      0x01 => { SVTCAx! (); }		/* SVTCA x  */
	      0x02 => { SPvTCAy! (); }		/* SPvTCA y */
	      0x03 => { SPvTCAx! (); }		/* SPvTCA x */
	      0x04 => { SFvTCAy! (); }		/* SFvTCA y */
	      0x05 => { SFvTCAx! (); }		/* SFvTCA x */
	      0x06 => { SPVTL! (); }		/* SPvTL // */
	      0x07 => { SPVTL! (); }		/* SPvTL +  */
	      0x08 => { SFVTL! (); }		/* SFvTL // */
	      0x09 => { SFVTL! (); }		/* SFvTL +  */
	      0x0A => { SPVFS! (); }		/* SPvFS */
	      0x0B => { SFVFS! (); }		/* SFvFS */
	      0x0C => { GPV! (); }		/* GPv */
	      0x0D => { GFV! (); }		/* GFv */
	      0x0E => { SFVTPV! (); }		/* SFvTPv */
	      0x0F => { ISECT! (); }		/* ISECT  */
	      0x10 => { SRP0! (); }		/* SRP0 */
	      0x11 => { SRP1! (); }		/* SRP1 */
	      0x12 => { SRP2! (); }		/* SRP2 */
	      0x13 => { SZP0! (); }		/* SZP0 */
	      0x14 => { SZP1! (); }		/* SZP1 */
	      0x15 => { SZP2! (); }		/* SZP2 */
	      0x16 => { SZPS! (); }		/* SZPS */
	      0x17 => { SLOOP! (); }		/* SLOOP */
	      0x18 => { RTG! (); }		/* RTG */
	      0x19 => { RTHG! (); }		/* RTHG */
	      0x1A => { SMD! (); }		/* SMD */
	      0x1B => { ELSE! (); }		/* ELSE */
	      0x1C => { JMPR! (); }		/* JMPR */
	      0x1D => { SCVTCI! (); }		/* SCVTCI */
	      0x1E => { SSWCI! (); }		/* SSWCI */
	      0x1F => { SSW! (); }		/* SSW */
	      0x20 => { DUP! (); }		/* DUP */
	      0x21 => { POP! (); }		/* POP */
	      0x22 => { CLEAR! (); }		/* CLEAR */
	      0x23 => { SWAP! (); }		/* SWAP */
	      0x24 => { DEPTH! (); }		/* DEPTH */
	      0x25 => { CINDEX! (); }		/* CINDEX */
	      0x26 => { MINDEX! (); }		/* MINDEX */
	      0x27 => { ALIGNPTS! (); }		/* ALIGNPTS */
	      0x28 => { RAW! (); }		/* RAW */
	      0x29 => { UTP! (); }		/* UTP */
	      0x2A => { LOOPCALL! (); }		/* LOOPCALL */
	      0x2B => { CALL! (); }		/* CALL */
	      0x2C => { FDEF! (); }		/* FDEF */
	      0x2D => { ENDF! (); }		/* ENDF */
	      0x2E => { MDAP! (); }		/* MDAP */
	      0x2F => { MDAP! (); }		/* MDAP */
	      0x30 => { IUP! (); }		/* IUP */
	      0x31 => { IUP! (); }		/* IUP */
	      0x32 => { SHP! (); }		/* SHP */
	      0x33 => { SHP! (); }		/* SHP */
	      0x34 => { SHC! (); }		/* SHC */
	      0x35 => { SHC! (); }		/* SHC */
	      0x36 => { SHZ! (); }		/* SHZ */
	      0x37 => { SHZ! (); }		/* SHZ */
	      0x38 => { SHPIX! (); }		/* SHPIX */
	      0x39 => { IP! (); }		/* IP    */
	      0x3A => { MSIRP! (); }		/* MSIRP */
	      0x3B => { MSIRP! (); }		/* MSIRP */
	      0x3C => { ALIGNRP! (); }		/* ALIGNRP */
	      0x3D => { RTDG! (); }		/* RTDG */
	      0x3E => { MIAP! (); }		/* MIAP */
	      0x3F => { MIAP! (); }		/* MIAP */
	      0x40 => { NPUSHB! (); }		/* NPUSHB */
	      0x41 => { NPUSHW! (); }		/* NPUSHW */
	      0x42 => { WS! (); }		/* WS */
	      0x43 => { RS! (); }		/* RS */
	      0x44 => { WCVTP! (); }		/* WCVTP */
	      0x45 => { RCVT! (); }		/* RCVT */
	      0x46 => { GC! (); }		/* GC */
	      0x47 => { GC! (); }		/* GC */
	      0x48 => { SCFS! (); }		/* SCFS */
	      0x49 => { MD! (); }		/* MD */
	      0x4A => { MD! (); }		/* MD */
	      0x4B => { MPPEM! (); }		/* MPPEM */
	      0x4C => { MPS! (); }		/* MPS */
	      0x4D => { FLIPON! (); }		/* FLIPON */
	      0x4E => { FLIPOFF! (); }		/* FLIPOFF */
	      0x4F => { DEBUG! (); }		/* DEBUG */
	      0x50 => { LT! (); }		/* LT */
	      0x51 => { LTEQ! (); }		/* LTEQ */
	      0x52 => { GT! (); }		/* GT */
	      0x53 => { GTEQ! (); }		/* GTEQ */
	      0x54 => { EQ! (); }		/* EQ */
	      0x55 => { NEQ! (); }		/* NEQ */
	      0x56 => { ODD! (); }		/* ODD */
	      0x57 => { EVEN! (); }		/* EVEN */
	      0x58 => { IF! (); }		/* IF */
	      0x59 => { EIF! (); }		/* EIF */
	      0x5A => { AND! (); }		/* AND */
	      0x5B => { OR! (); }		/* OR */
	      0x5C => { NOT! (); }		/* NOT */
	      0x5D => { DELTAP1! (); }		/* DELTAP1 */
	      0x5E => { SDB! (); }		/* SDB */
	      0x5F => { SDS! (); }		/* SDS */
	      0x60 => { ADD! (); }		/* ADD */
	      0x61 => { SUB! (); }		/* SUB */
	      0x62 => { DIV! (); }		/* DIV */
	      0x63 => { MUL! (); }		/* MUL */
	      0x64 => { ABS! (); }		/* ABS */
	      0x65 => { NEG! (); }		/* NEG */
	      0x66 => { FLOOR! (); }		/* FLOOR */
	      0x67 => { CEILING! (); }		/* CEILING */
	      0x68 => { ROUND! (); }		/* ROUND */
	      0x69 => { ROUND! (); }		/* ROUND */
	      0x6A => { ROUND! (); }		/* ROUND */
	      0x6B => { ROUND! (); }		/* ROUND */
	      0x6C => { NROUND! (); }		/* NROUND */
	      0x6D => { NROUND! (); }		/* NROUND */
	      0x6E => { NROUND! (); }		/* NRRUND */
	      0x6F => { NROUND! (); }		/* NROUND */
	      0x70 => { WCVTF! (); }		/* WCVTF */
	      0x71 => { DELTAP2! (); }		/* DELTAP2 */
	      0x72 => { DELTAP3! (); }		/* DELTAP3 */
	      0x73 => { DELTAC1! (); }		/* DELTAC1 */
	      0x74 => { DELTAC2! (); }		/* DELTAC2 */
	      0x75 => { DELTAC3! (); }		/* DELTAC3 */
	      0x76 => { SROUND! (); }		/* SROUND */
	      0x77 => { S45ROUND! (); }		/* S45Round */
	      0x78 => { JROT! (); }		/* JROT */
	      0x79 => { JROF! (); }		/* JROF */
	      0x7A => { ROFF! (); }		/* ROFF */
	      0x7B => { ILLEGAL_INSTRUCTION! (); }		/* ILLEGAL_INSTRUCTION */
	      0x7C => { RUTG! (); }		/* RUTG */
	      0x7D => { RDTG! (); }		/* RDTG */
	      0x7E => { SANGW! (); }		/* SANGW */
	      0x7F => { AA! (); }		/* AA */
	      0x80 => { FLIPPT! (); }		/* FLIPPT */
	      0x81 => { FLIPRGON! (); }		/* FLIPRGON */
	      0x82 => { FLIPRGOFF! (); }		/* FLIPRGOFF */
	      0x83 => { NOT_IMPLEMENTED! (); }		/* RMVT */
	      0x84 => { NOT_IMPLEMENTED! (); }		/* WMVT */
	      0x85 => { SCANCTRL! (); }		/* SCANCTRL */
	      0x86 => { SDPVTL! (); }		/* SDPVTL */
	      0x87 => { SDPVTL! (); }		/* SDPVTL */
	      0x88 => { GETINFO! (); }		/* GETINFO */
	      0x89 => { IDEF! (); }		/* IDEF */
	      0x8A => { ROLL! (); }		/* ROLL */
	      0x8B => { INS_MAX! (); }		/* MAX */
	      0x8C => { INS_MIN! (); }		/* MIN */
	      0x8D => { SCANTYPE! (); }		/* SCANTYPE */
	      0x8E => { INSTCTRL! (); }		/* INSTCTRL */
	      0x8F => { NOT_IMPLEMENTED! (); }		/* ADJUST */
	      0x90 => { NOT_IMPLEMENTED! (); }		/* ADJUST */
	      0x91 => { GXAXIS! (); }		/* GXAXIS */
		  _ =>
		    {
		      if opcode >= 0xE0  /* MIRP */
			{ MIRP! (); }
		      else if opcode >= 0xC0  /* MDRP */
			{ MDRP! (); }
		      else if opcode >= 0xB8  /* PUSHW */
			{ PUSHW! (); }
		      else if opcode >= 0xB0  /* PUSHB */
			{ PUSHB! (); }
		      else
			{ NOT_IMPLEMENTED! (); }
		    }
		}

	      break;
	    }
	}
	(*interpreter).IP += 1;
      }
      continue;
    }
}

/* Execute the font program FPGM using INTERPRETER.
   This must only be called once per interpreter, else behavior is
   undefined.

   Value is NULL upon success, else it is a string describing the
   reason for failure.

   The caller must save the graphics state after interpreting the font
   program and restore it prior to instructing each glyph.  */

/* C 11995's `TEST_STATIC const char *` is the file's `cfg_attr` shape, and C 11999 is the unit's
   first `setjmp` call: the declaration stands at C 37's site above with its own note.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_interpret_font_program (interpreter: *mut sfnt_interpreter,
						      fpgm: *mut sfnt_fpgm_table)
  -> *const c_char                                       // C 11995-12011
{
  if setjmp (&raw mut (*interpreter).trap) != 0           // C 11999
    { return (*interpreter).trap_reason; }               // C 12000

  /* Set up the interpreter to evaluate the font program.  */
  (*interpreter).IP = 0;                                 // C 12003
  (*interpreter).SP = (*interpreter).stack;              // C 12004
  (*interpreter).instructions = (*fpgm).instructions;    // C 12005
  (*interpreter).num_instructions = (*fpgm).num_instructions as uint16_t;   // C 12006
  (*interpreter).glyph_zone = std::ptr::null_mut ();     // C 12007

  sfnt_interpret_run (interpreter, SFNT_RUN_CONTEXT_FONT_PROGRAM);   // C 12009
  return std::ptr::null ();                              // C 12010
}

/* Execute the control value program PREP using INTERPRETER.

   Return NULL and the graphics state after the execution of the
   program in *STATE, or a string describing the reason for a failure
   to interpret the program.

   The caller must save the graphics state after interpreting the
   control value program and restore it prior to instructing each
   glyph.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]
pub unsafe extern "C" fn sfnt_interpret_control_value_program (interpreter: *mut sfnt_interpreter,
							       prep: *mut sfnt_prep_table,
							       state: *mut sfnt_graphics_state)
  -> *const c_char                                       // C 12023-12082
{
  if setjmp (&raw mut (*interpreter).trap) != 0           // C 12028
    { return (*interpreter).trap_reason; }               // C 12029

  /* Set up the interpreter to evaluate the control value program.  */
  (*interpreter).IP = 0;                                 // C 12032
  (*interpreter).SP = (*interpreter).stack;              // C 12033
  (*interpreter).instructions = (*prep).instructions;    // C 12034
  (*interpreter).num_instructions = (*prep).num_instructions as uint16_t;   // C 12035
  (*interpreter).glyph_zone = std::ptr::null_mut ();     // C 12036

  sfnt_interpret_run (interpreter,                       // C 12038-12039
		      SFNT_RUN_CONTEXT_CONTROL_VALUE_PROGRAM);

  /* If instruct_control & 2, then changes to the graphics state made
     in this program should be reverted.  */

  if ((*interpreter).state.instruct_control & 2) != 0    // C 12044
    { sfnt_init_graphics_state (&raw mut (*interpreter).state); }   // C 12045
  else
    {
      /* And even if not, reset the following graphics state
	 variables, to which both the Apple and MS scalers don't
	 permit modifications from the preprogram.

         Not only is such reversion undocumented, it is also
         inefficient, for modern fonts at large only move points on
         the Y axis.  As such, these fonts must issue a redundant
         SVTCA[Y] instruction within each glyph program, in place of
         initializing the projection and freedom vectors once and for
         all in prep.  Unfortunately many fonts which do instruct on
         the X axis now rely on this ill-conceived behavior, so Emacs
         must, reluctantly, follow suit.  */

      (*interpreter).state.dual_projection_vector.x = 0o40000; /* 1.0 */
      (*interpreter).state.dual_projection_vector.y = 0;         // C 12062
      (*interpreter).state.freedom_vector.x = 0o40000; /* 1.0 */
      (*interpreter).state.freedom_vector.y = 0;                 // C 12064
      (*interpreter).state.projection_vector.x = 0o40000; /* 1.0 */
      (*interpreter).state.projection_vector.y = 0;              // C 12066
      (*interpreter).state.rp0 = 0;                              // C 12067
      (*interpreter).state.rp1 = 0;                              // C 12068
      (*interpreter).state.rp2 = 0;                              // C 12069
      (*interpreter).state.zp0 = 1;                              // C 12070
      (*interpreter).state.zp1 = 1;                              // C 12071
      (*interpreter).state.zp2 = 1;                              // C 12072
      (*interpreter).state.r#loop = 1;                           // C 12073

      /* Validate the graphics state.  */
      sfnt_validate_gs (&raw mut (*interpreter).state);          // C 12076
    }

  /* Save the graphics state upon success.  */
  memcpy (state as *mut c_void,                          // C 12080
	  &raw mut (*interpreter).state as *const c_void,
	  std::mem::size_of_val (&*state));
  return std::ptr::null ();                              // C 12081
}
