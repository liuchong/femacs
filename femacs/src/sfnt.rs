// femacs: port of src/sfnt.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct field, macro, file-scope object, local), the same control flow and the same error model.
// The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers stay plain `unsafe fn`; C's `TEST_STATIC` (C 128/137) is spelled
// `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]`, which is what the macro's two arms
// select.
//
// **This file is P-86's largest unit (C 21,215 lines) and is being ported in slices.**  This slice
// is C 1-2407: slice 1 (C 1-1112, through `sfnt_read_cmap_table`) plus the glyph lookup routines
// (C 1114-1465), the table readers (C 1467-1906: `head`, `hhea`, both `loca` forms, `maxp`, `glyf`
// and the `HAVE_MMAP` pair) and the two glyph outline readers (C 1908-2407).  **The next live
// definition is C 2413's `sfnt_read_glyph`.**  The ledger `docs/ports/sfnt.md` carries the C
// line-range table for C 1-2407 plus the "the remainder" row, the `## Evidence` readings and the
// `## Deviations`.
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
