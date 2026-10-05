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
// slices so far are C 1-6306: slice 1 (C 1-1112, through `sfnt_read_cmap_table`), slice 2 (the
// glyph lookup routines, the table readers and the two glyph outline readers, C 1113-2407),
// slice 3 (the glyph outline decomposition, C 2408-3594), slice 4 (the outline builders and the
// "ragged" rasterizer, C 3595-4543), slice 5 (the "exact coverage scaler", C 4544-5451) and
// slice 6 (the metrics, name and meta readers, the TrueType collection header and the table
// readers and fixed-point helpers the interpreter needs, C 5455-6306).  **The next live
// definition is C 6316's `sfnt_init_graphics_state`** - and it is *blocked*: its only callee,
// `sfnt_validate_gs` (declared C 6310), is defined at C 10538 in the remainder, which is the
// ledger's Owed 12.  The ledger `docs/ports/sfnt.md` carries the C line-range table for C 1-6306
// plus the "the remainder" row, the `## Evidence` readings and the `## Deviations`.
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
   C 1-1112 names neither.  */
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
