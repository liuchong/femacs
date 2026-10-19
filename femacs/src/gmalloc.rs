// femacs: port of src/gmalloc.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local) and the same control flow.  This unit is
// one of P-86's class: **the frozen configuration compiles none of it**, so every item carries the
// file gate and the frozen build sees only `config.rs` (the reading is in the ledger's
// `## Evidence`).
//
//   * the **file gate** is `all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"))`
//     - the build's own condition, not an `#if`: `emacs/src/Makefile.in` 489-490 puts `gmalloc.o`
//     in `GMALLOC_OBJ` (`GMALLOC_OBJ=@GMALLOC_OBJ@`), and `configure.ac` 3308-3317 fills it only
//     in the `else` arm of `if test "${system_malloc}" = "yes"` and only there under
//     `test "$doug_lea_malloc" != "yes"`.  The frozen build's own generated
//     `build/emacs-ref/src/Makefile` 490 is `GMALLOC_OBJ=` (empty) and its generated `config.h`
//     2342 defines `SYSTEM_MALLOC` (1), so `docs/baseline/frozen-build-sources.txt` lists no
//     `gmalloc.o` for this target.  Both feature names are registered
//     (`femacs/xtask/Cargo.toml` 597 `no_system_malloc`, 86 `doug_lea_malloc`); together they are
//     C's `!SYSTEM_MALLOC && !DOUG_LEA_MALLOC`, and one alone would compile text C never compiles.
//   * the **sub-gates** are C's own `#if`s, each on the item it gates: C 24 `#ifdef HAVE_MALLOC_H`
//     (`have_malloc_h`), C 34 `#if defined HAVE_PTHREAD` (`have_pthread`), C 45/199/474/743/1163
//     `#ifdef USE_PTHREAD` (`use_pthread`, C's own alias defined at C 35), C 49/54's glibc arms
//     (`glibc_2_34`/`glibc_2_24`), C 90 `#ifdef MSDOS` (`msdos`), C 234/523/1693 `#ifdef GC_MCHECK`
//     (`gc_mcheck`), C 347 `GC_MALLOC_CHECK && GC_PROTECT_MALLOC_STATE` (`gc_malloc_check`,
//     `gc_protect_malloc_state`), C 740 `GC_MALLOC_CHECK`/`USE_PTHREAD`/neither, C 902
//     `#if !(defined (_LIBC))` (`libc`), C 1227 `#ifdef weak_alias` (`weak_alias`), C 1462
//     `#if !defined (__GNU_LIBRARY__) || defined (__UCLIBC__)` (`__gnu_library__`, `__uclibc__`),
//     C 1477 `#ifdef HAVE_SBRK` (`have_sbrk`), C 1640 `_SC_PAGESIZE`/`HAVE_GETPAGESIZE`
//     (`_sc_pagesize`, `have_getpagesize`), C 1693 `#ifdef GC_MCHECK` (`gc_mcheck`) and C 1872
//     `#ifdef __GNU_LIBRARY__` (`__gnu_library__`).  The frozen value of every one is in the
//     ledger's gate table.
//   * **the rename chain C 61-71 and its boundary at C 1657 are part of the text**: C `#undef`s the
//     `lisp.h` chain's names and `#define`s `malloc`->`gmalloc`, `realloc`->`grealloc`,
//     `calloc`->`gcalloc`, `aligned_alloc`->`galigned_alloc`, `free`->`gfree` and
//     `malloc_info`->`gmalloc_info`, so the definitions C 882-1404 emit are the `g*` symbols and
//     every call before C 1657 reaches them; C 1657-1661 `#undef`s the five function renames again
//     and C 1663-1691 define the real names over them.  The port spells the five renames as
//     `macro_rules!` (the `lib/include/stdlib.in.rs` 271 shape) and the call sites with the `!`
//     form, which is the same text and the same shadowing order C's preprocessor produces; the
//     union at C 113-146 is declared under C's post-rename name `gmalloc_info` and the
//     `malloc_info` name lives on as the marker macro at C 71.
//   * `#if 0` at C 693-696 and the `#pragma GCC diagnostic` at C 26 have no Rust item: they are
//     recorded in the ledger and dropped (`femacs/exec/trace.rs` 1605's precedent for the first;
//     the second is a compiler directive, `femacs/lib/strftime.rs` 269).
//   * `<malloc.h>` (C 28), `<pthread.h>` (C 46) and `<sys/mman.h>` (C 359) have no file in the
//     reference tree and no mirror.  Each stands in this unit at its own `#include` line (P-88
//     (1)), and its declarations carry the header's own line in a `/// C:` line; the layouts that
//     could be measured on this machine are in the `## Evidence` readings, the rest are marked
//     UNVERIFIED.  Nothing gmalloc.c names from `<malloc.h>` is used, so that arm carries no item.
//   * `min` (C 1371) and `abort` (C 383) are not on this unit's include graph; both stand at their
//     use site and are in the ledger's `## Owed` (the `femacs/src/w32term.rs` 1683 shape).  Two
//     measured conversions are spelled out: C's `PTRDIFF_MAX < size` (C 405) promotes `ptrdiff_t`
//     to `size_t`, and C's `- size` (C 1511) is the unsigned wrap (`wrapping_neg`).
//
// `docs/ports/gmalloc.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn,
         unused_assignments, unused_parens, unused_mut, unreachable_code, unused_labels, while_true,
         unused_comparisons, overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                  // C 22: `#include <config.h>`

/* C 24-29: `#ifdef HAVE_MALLOC_H` / `# if GNUC_PREREQ (4, 2, 0)` / `#  pragma GCC diagnostic
   ignored "-Wdeprecated-declarations"` / `# endif` / `# include <malloc.h>` / `#endif`.  The
   pragma is a compiler directive with no Rust item (recorded in the ledger), and the header
   declares the three hooks C 312-314 defines itself, so the arm carries no item of its own: the
   `__MALLOC_HOOK_VOLATILE` name it would define is the marker at C 30 below.  */

/* C 30-32: `#ifndef __MALLOC_HOOK_VOLATILE` / `# define __MALLOC_HOOK_VOLATILE volatile` /
   `#endif` - C's condition tests a macro nothing in this file's text defines, and Rust spells the
   qualifier at the access site, so the name lives on as this marker (the `vm-limit.rs` 42-44
   shape).  */
pub const __MALLOC_HOOK_VOLATILE: () = ();

/* C 34-36: `#if defined HAVE_PTHREAD` / `#define USE_PTHREAD` / `#endif` - C's own alias for
   HAVE_PTHREAD, which the `#ifdef USE_PTHREAD` sites below test.  The port keeps the name and
   gates the sites with `use_pthread` (the `alloc.rs` 286 `USE_VALGRIND` shape).  */
#[cfg (feature = "have_pthread")]
pub const USE_PTHREAD: () = ();

#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
include!("../lib/include/stddef.in.rs");        // C 38: `#include <stddef.h>`
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
include!("../lib/include/stdlib.rs");           // C 39: `#include <stdlib.h>`
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
include!("../lib/include/string.rs");           // C 40: `#include <string.h>`
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
include!("../lib/include/limits.in.rs");        // C 41: `#include <limits.h>` (INT_WIDTH)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
include!("../lib/include/stdint.in.rs");        // C 42: `#include <stdint.h>` (uintptr_t)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
include!("../lib/include/unistd.in.rs");        // C 43: `#include <unistd.h>` (getpagesize)

/* C 45-47: `#ifdef USE_PTHREAD` / `# include <pthread.h>`.  The record and its initializer are
   the system's and differ per target: bionic's `pthread_mutex_t` is 40 bytes (align 4) with an
   all-zero initializer, darwin's is 64 with the `0x32AAABA7` sentinel.  Using the host shape on
   Android made `_malloc_mutex` start out "destroyed" (status 4.805).  */
/* Both sizes are asserted, which is what turns them into rows of
   `docs/baseline/layout/android-arm64.tsv` that `tools/check-android-layout.py` compares with the
   NDK - the missing assertion is what let the host shape survive here (status 4.805).  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread", feature = "have_android"))]
const _: () = core::assert! (core::mem::size_of::<pthread_mutex_t> () == 40);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread", not (feature = "have_android")))]
const _: () = core::assert! (core::mem::size_of::<pthread_mutex_t> () == 64);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
unsafe extern "C" {
  /// C: pthread.h: `int pthread_mutex_init (pthread_mutex_t *, const pthread_mutexattr_t *);`
  /// C: pthread.h: `int pthread_mutex_lock (pthread_mutex_t *);`
  /// C: pthread.h: `int pthread_mutex_unlock (pthread_mutex_t *);`
  pub fn pthread_mutex_init (mutex: *mut pthread_mutex_t, attr: *const c_void) -> c_int;
  pub fn pthread_mutex_lock (mutex: *mut pthread_mutex_t) -> c_int;
  pub fn pthread_mutex_unlock (mutex: *mut pthread_mutex_t) -> c_int;
  /// C: pthread.h: `int pthread_atfork (void (*)(void), void (*)(void), void (*)(void));`
  pub fn pthread_atfork (prepare: Option<unsafe extern "C" fn ()>,
                         parent: Option<unsafe extern "C" fn ()>,
                         child: Option<unsafe extern "C" fn ()>) -> c_int;
}

/* C 49-53: `#if !defined HAVE_MALLOC_H || (__GLIBC__ > 2 || __GLIBC_MINOR__ >= 34)` /
   `extern void (*__MALLOC_HOOK_VOLATILE __after_morecore_hook) (void);` /
   `extern void *(*__morecore) (ptrdiff_t);` / `#endif`, and C 54-57's the same condition with
   `>= 24` over `extern void (*__MALLOC_HOOK_VOLATILE __malloc_initialize_hook) (void);`.  Both
   declare globals C 312-314 defines itself, so the port keeps them as the definitions there (a
   Rust `extern` declaration and a definition of one name in one crate root is `E0428`).
   `__GLIBC__`/`__GLIBC_MINOR__` are undefined on this target, so C's `0 > 2`/`0 >= 34` are false
   and only the `!defined HAVE_MALLOC_H` half is live; the two glibc feature names stand for the
   other half, whose values are UNVERIFIED (no glibc here).  */

/* C 59's `#include "lisp.h"`: the file gate is on the edge (P-86 ⑧), and `xtask`'s
   `materialize` only reads a gate above an `include!` when the whole attribute is on one line, so
   every gated edge in this unit is written that way.  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
include!("include/lisp.rs");                    // C 59: `#include "lisp.h"`

/* C 61-71: the renames.  C's five `#undef`s cancel the `lisp.h` chain's renames (the port's
   includes carry none of them) and the six `#define`s rename the *symbols* the rest of the file
   defines: C 882/1215/1394/1424/1503's definitions keep C's own text names and carry the
   preprocessor's `g*` symbol through `#[unsafe(export_name)]`, C 1657-1661's `#undef`s re-expose
   the five plain names over them, and C 1663-1691's definitions provide the plain symbols.  One
   Rust item per symbol means the five plain definitions cannot also be *named* with C's text name
   (`E0428`), so they carry C's name in their `export_name` and a `rpl_*` item name (the gnulib
   spelling for the replacement that provides the real symbol); the ledger records it.
   `malloc_info` (C 71) renames the union's typedef name, which the port declares as
   `gmalloc_info`, so the macro's name lives on as this marker.  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! malloc_info { () => { gmalloc_info }; }
/// C: #define malloc_info gmalloc_info
/* The five `g*` spellings C 1657-1661's `#undef`s leave free are what C 1663-1691's wrapper
   bodies call (they are written after the `#undef`s, so their callee is spelled `gmalloc` there);
   the port keeps those tokens as macros over the items above.  */
/// C: #define malloc gmalloc
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! gmalloc { ($size:expr) => { malloc ($size) }; }
/// C: #define realloc grealloc
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! grealloc { ($ptr:expr, $size:expr) => { realloc ($ptr, $size) }; }
/// C: #define calloc gcalloc
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! gcalloc { ($nmemb:expr, $size:expr) => { calloc ($nmemb, $size) }; }
/// C: #define aligned_alloc galigned_alloc
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! galigned_alloc { ($alignment:expr, $size:expr) => { aligned_alloc ($alignment, $size) }; }
/// C: #define free gfree
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! gfree { ($ptr:expr) => { free ($ptr) }; }

/* C 73-76 (`#ifdef __cplusplus` / `extern "C"` / `{` / `#endif`) and C 277-281 (`#undef extern`,
   the closing `}`): the port has no counterpart for C++ linkage guards.  */

/* C 78-93: the five allocator prototypes and C 90-93's `#ifdef MSDOS` pair.  Every one of them
   declares a function this unit defines below (C 882-882/1427/1590/1648/1663-1691), and Rust
   cannot repeat a declaration of a name it defines (`E0428`), so the declarations are recorded
   here rather than transcribed; C's `ATTRIBUTE_MALLOC_SIZE`/`ATTRIBUTE_ALLOC_SIZE` are gone with
   them.  */

/* C 95-99: the block/fragment design comment.  */
/// C: /* The allocator divides the heap into blocks of fixed size; large
/// C:    requests receive one or more whole blocks, and small requests
/// C:    receive a fragment of a block.  Fragment sizes are powers of two,
/// C:    and all fragments of a block are the same size.  When all the
/// C:    fragments in a block have been freed, the block itself is freed.  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! BLOCKLOG { () => { (if INT_WIDTH > 16 { 12 } else { 9 }) }; }
/// C: #define BLOCKLOG	(INT_WIDTH > 16 ? 12 : 9)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! BLOCKSIZE { () => { (1 << BLOCKLOG! ()) }; }
/// C: #define BLOCKSIZE	(1 << BLOCKLOG)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! BLOCKIFY { ($size:expr) => { ((($size) + BLOCKSIZE! () - 1) / BLOCKSIZE! ()) }; }
/// C: #define BLOCKIFY(SIZE)	(((SIZE) + BLOCKSIZE - 1) / BLOCKSIZE)

/* C 104-105: "Determine the amount of memory spanned by the initial heap table (not an absolute
   limit)."  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! HEAP { () => { (if INT_WIDTH > 16 { 4194304 } else { 65536 }) }; }
/// C: #define HEAP		(INT_WIDTH > 16 ? 4194304 : 65536)

/* C 108-109: "Number of contiguous free blocks allowed to build up at the end of memory before
   they will be returned to the system."  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
pub const FINAL_FREE_BLOCKS: c_int = 8;
/// C: #define FINAL_FREE_BLOCKS	8

/// C 112-146: `typedef union { … } malloc_info;` - the per-block information table, under C's
/// post-rename name (`C 71`'s `#define malloc_info gmalloc_info`).  The two members and the inner
/// union are C's anonymous aggregates, named here so the layout is spellable; the C field names
/// (`busy`/`free`, `info`, `frag`, `type`, `nfree`, `first`, `size`) are unchanged.
/// C: typedef union { struct { int type; union { struct { size_t nfree; size_t first; } frag;
/// C: ptrdiff_t size; } info; } busy; struct { size_t size; size_t next; size_t prev; } free; }
/// C: malloc_info;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[derive (Clone, Copy)]
#[repr (C)]
pub struct gmalloc_info_busy_info_frag
{
  pub nfree: size_t,
  pub first: size_t,
}
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[derive (Clone, Copy)]
#[repr (C)]
pub union gmalloc_info_busy_info
{
  pub frag: gmalloc_info_busy_info_frag,
  pub size: ptrdiff_t,
}
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[derive (Clone, Copy)]
#[repr (C)]
pub struct gmalloc_info_busy
{
  pub r#type: c_int,
  pub info: gmalloc_info_busy_info,
}
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[derive (Clone, Copy)]
#[repr (C)]
pub struct gmalloc_info_free
{
  pub size: size_t,
  pub next: size_t,
  pub prev: size_t,
}
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[derive (Clone, Copy)]
#[repr (C)]
pub union gmalloc_info
{
  pub busy: gmalloc_info_busy,
  pub free: gmalloc_info_free,
}
/* C's `busy.type` is a field name, and `type` is a Rust keyword: the port keeps the name with the
   raw identifier `r#type`, and the field access sites spell it that way.  */
/* Measured on this machine (clang, darwin-arm64): sizeof 24, align 8, `busy.type` 0,
   `busy.info` 8, `busy.info.frag.nfree` 8, `busy.info.frag.first` 16, `busy.info.size` 8,
   `free.size` 0, `free.next` 8, `free.prev` 16.  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::size_of::<gmalloc_info> () == 24);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::align_of::<gmalloc_info> () == 8);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (gmalloc_info, busy) == 0);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (gmalloc_info, free) == 0);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (gmalloc_info_busy, r#type) == 0);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (gmalloc_info_busy, info) == 8);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (gmalloc_info_busy_info_frag, nfree) == 0);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (gmalloc_info_busy_info_frag, first) == 8);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (gmalloc_info_free, next) == 8);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (gmalloc_info_free, prev) == 16);

/* C 148-152: `extern char *_heapbase;` and `extern malloc_info *_heapinfo;` - declarations of the
   globals C 317/320 define.  */

/* C 154: "Address to block number and vice versa."  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! BLOCK {
  ($a:expr) => { (($a) as *mut c_char).offset_from (_heapbase) as size_t / BLOCKSIZE! () + 1 };
}
/// C: #define BLOCK(A)	((size_t) ((char *) (A) - _heapbase) / BLOCKSIZE + 1)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
macro_rules! ADDRESS {
  ($b:expr) => { _heapbase.offset (((($b) - 1) * BLOCKSIZE! ()) as isize) as *mut c_void };
}
/// C: #define ADDRESS(B)	((void *) (((B) - 1) * BLOCKSIZE + _heapbase))

/* C 158-162: `extern size_t _heapindex;` and `extern size_t _heaplimit;` - the globals C 323/326
   define.  */

/* C 164-169: "Doubly linked lists of free fragments."  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[derive (Clone, Copy)]
#[repr (C)]
pub struct list
{
  pub next: *mut list,
  pub prev: *mut list,
}
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::size_of::<list> () == 16);          // clang: sizeof 16, align 8
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (list, prev) == 8);      // clang: next 0, prev 8

/// C 171-172: `static struct list _fraghead[BLOCKLOG];` - bss, so zeroed (tentative definition).
/// C: static struct list _fraghead[BLOCKLOG];
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
static mut _fraghead: [list; BLOCKLOG! ()] = unsafe { core::mem::zeroed () };

/// C 174-181: "List of blocks allocated with aligned_alloc and friends."
/// C: struct alignlist { struct alignlist *next; void *aligned; void *exact; };
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[derive (Clone, Copy)]
#[repr (C)]
pub struct alignlist
{
  pub next: *mut alignlist,
  pub aligned: *mut c_void,
  pub exact: *mut c_void,
}
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::size_of::<alignlist> () == 24);     // clang: sizeof 24, align 8
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (alignlist, aligned) == 8);   // clang: 0/8/16
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
const _: () = assert! (core::mem::offset_of! (alignlist, exact) == 16);

/* C 181: `extern struct alignlist *_aligned_blocks;` - defined at C 955.  C 183-187:
   `extern size_t _chunks_used;` / `_bytes_used` / `_chunks_free` / `_bytes_free` - defined at
   C 329-332.  C 189-197: the six internal prototypes - defined at C 679, 870, 959, 1205, 1266,
   1382.  */

/* C 199-227: the four locking macros.  C's `do { … } while (0)` is the port's block expression
   (PORTING.md 2.1); `pthread_mutex_lock (&_malloc_mutex)` takes the address of a static, which
   the 2024 edition spells `&raw mut`.  */
/// C: #define LOCK()					\
/// C:   do {						\
/// C:     if (_malloc_thread_enabled_p)		\
/// C:       pthread_mutex_lock (&_malloc_mutex);	\
/// C:   } while (0)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
macro_rules! LOCK {
  () => { if _malloc_thread_enabled_p != 0 { pthread_mutex_lock (&raw mut _malloc_mutex); } };
}
/// C: #define UNLOCK()				\
/// C:   do {						\
/// C:     if (_malloc_thread_enabled_p)		\
/// C:       pthread_mutex_unlock (&_malloc_mutex);	\
/// C:   } while (0)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
macro_rules! UNLOCK {
  () => { if _malloc_thread_enabled_p != 0 { pthread_mutex_unlock (&raw mut _malloc_mutex); } };
}
/// C: #define LOCK_ALIGNED_BLOCKS()				\
/// C:   do {							\
/// C:     if (_malloc_thread_enabled_p)			\
/// C:       pthread_mutex_lock (&_aligned_blocks_mutex);	\
/// C:   } while (0)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
macro_rules! LOCK_ALIGNED_BLOCKS {
  () => { if _malloc_thread_enabled_p != 0
            { pthread_mutex_lock (&raw mut _aligned_blocks_mutex); } };
}
/// C: #define UNLOCK_ALIGNED_BLOCKS()				\
/// C:   do {							\
/// C:     if (_malloc_thread_enabled_p)			\
/// C:       pthread_mutex_unlock (&_aligned_blocks_mutex);	\
/// C:   } while (0)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
macro_rules! UNLOCK_ALIGNED_BLOCKS {
  () => { if _malloc_thread_enabled_p != 0
            { pthread_mutex_unlock (&raw mut _aligned_blocks_mutex); } };
}
/* C 222-226's `#else` arm: the four macros are empty (C evaluates no argument).  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (feature = "use_pthread")))]
macro_rules! LOCK { () => { }; }
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (feature = "use_pthread")))]
macro_rules! UNLOCK { () => { }; }
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (feature = "use_pthread")))]
macro_rules! LOCK_ALIGNED_BLOCKS { () => { }; }
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (feature = "use_pthread")))]
macro_rules! UNLOCK_ALIGNED_BLOCKS { () => { }; }
/// C: #define LOCK()
/// C: #define UNLOCK()
/// C: #define LOCK_ALIGNED_BLOCKS()
/// C: #define UNLOCK_ALIGNED_BLOCKS()

/* C 200-201: `extern pthread_mutex_t _malloc_mutex, _aligned_blocks_mutex;` and
   `extern int _malloc_thread_enabled_p;` - the objects C 475-477 define.  */

/* C 229-232: `extern int __malloc_initialized;` and `extern int __malloc_initialize (void);` -
   the object is defined at C 335 and the function at C 551.  */

/* C 234-275: `#ifdef GC_MCHECK` - "Return values for `mprobe'" and the standard debugging-hook
   declarations.  `mcheck`/`mprobe`/`mtrace`/`muntrace`/`mstats` are the C library's (glibc's
   malloc-debug API), declared here and called at C 524; the port keeps the declarations under the
   same condition, with `struct mstats` and `enum mcheck_status` in full.  */
/// C: enum mcheck_status { MCHECK_DISABLED = -1, MCHECK_OK, MCHECK_FREE, MCHECK_HEAD,
/// C: MCHECK_TAIL };
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
#[derive (Clone, Copy, PartialEq, Eq)]
#[repr (C)]
pub enum mcheck_status
{
  MCHECK_DISABLED = -1,
  MCHECK_OK,
  MCHECK_FREE,
  MCHECK_HEAD,
  MCHECK_TAIL,
}
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
const _: () = assert! (core::mem::size_of::<mcheck_status> () == 4);   // clang: sizeof 4, align 4
/// C: struct mstats { size_t bytes_total; size_t chunks_used; size_t bytes_used;
/// C: size_t chunks_free; size_t bytes_free; };
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
#[derive (Clone, Copy)]
#[repr (C)]
pub struct mstats
{
  pub bytes_total: size_t,
  pub chunks_used: size_t,
  pub bytes_used: size_t,
  pub chunks_free: size_t,
  pub bytes_free: size_t,
}
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
const _: () = assert! (core::mem::size_of::<mstats> () == 40);          // clang: sizeof 40, align 8
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
unsafe extern "C" {
  /* C 252 and C 273's `mcheck`/`mprobe` declarations are the forward declarations of this file's
     own definitions (C 1882-1900, C 1902-1906), so the port keeps the definitions only.  */
  /// C: extern void mtrace (void);
  pub fn mtrace ();
  /// C: extern void muntrace (void);
  pub fn muntrace ();
  /// C: extern struct mstats mstats (void);
  pub fn mstats () -> mstats;
}

/* C 283's licence-block boundary is C 304 below; C's `#include <errno.h>` is C 304.  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
include!("../lib/include/errno.rs");            // C 304: `#include <errno.h>`

/// C 306-307: "Debugging hook for 'malloc'."
/// C: static void *(*__MALLOC_HOOK_VOLATILE gmalloc_hook) (size_t);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
static mut gmalloc_hook: Option<unsafe extern "C" fn (size_t) -> *mut c_void> = None;

/* C 309-314: the three hook objects.  C's "Platforms with these hooks all used relaxed ref/def, so
   it is OK to define them here too" are tentative definitions (no `extern`), i.e. real symbols
   when this file is the one that is compiled.  */
/// C: void (*__MALLOC_HOOK_VOLATILE __malloc_initialize_hook) (void);
/// C: void (*__MALLOC_HOOK_VOLATILE __after_morecore_hook) (void);
/// C: void (*__MALLOC_HOOK_VOLATILE __after_morecore_hook) (void);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut __malloc_initialize_hook: Option<unsafe extern "C" fn ()> = None;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut __after_morecore_hook: Option<unsafe extern "C" fn ()> = None;
/* C 314's `void *(*__morecore) (ptrdiff_t);` is the tentative definition whose initializer C 1485
   carries; the port keeps the one definition point there.  */

/// C 316-317: "Pointer to the base of the first block."
/// C: char *_heapbase;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut _heapbase: *mut c_char = core::ptr::null_mut ();

/// C 319-320: "Block information table.  Allocated with align/__free (not malloc/free)."
/// C: malloc_info *_heapinfo;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut _heapinfo: *mut gmalloc_info = core::ptr::null_mut ();

/// C 322-323: "Search index in the info table."
/// C: size_t _heapindex;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut _heapindex: size_t = 0;

/// C 325-326: "Limit of valid info table indices."
/// C: size_t _heaplimit;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut _heaplimit: size_t = 0;

/// C 328-332: "Instrumentation."
/// C: size_t _chunks_used; size_t _bytes_used; size_t _chunks_free; size_t _bytes_free;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut _chunks_used: size_t = 0;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut _bytes_used: size_t = 0;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut _chunks_free: size_t = 0;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut _bytes_free: size_t = 0;

/// C 334-335: "Are you experienced?"
/// C: int __malloc_initialized;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut __malloc_initialized: c_int = 0;

/* C 337-342: "Number of extra blocks to get each time we ask for more core."  The `#if` at
   C 339-341 decides the linkage: `static` when DOUG_LEA_MALLOC or SYSTEM_MALLOC holds (the
   file is then a libc-side helper), no `static` otherwise - the arm that exports the symbol.  */
/// C: #if defined DOUG_LEA_MALLOC || defined SYSTEM_MALLOC
/// C: static
/// C: #endif
/// C: size_t __malloc_extra_blocks;
/* C's `#if` has two halves and they are not symmetric here: `defined SYSTEM_MALLOC` is the file
   gate's own negation (`no_system_malloc` is what `!SYSTEM_MALLOC` is called), so its half of the
   condition can never hold inside the gate - C does not build this file when SYSTEM_MALLOC is
   defined either.  The `static` arm therefore carries the reachable half, `DOUG_LEA_MALLOC`, and
   the ledger records that the two arms cannot be compiled together.  */
#[cfg (feature = "doug_lea_malloc")]
static mut __malloc_extra_blocks: size_t = 0;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut __malloc_extra_blocks: size_t = 0;

/// C 344-345: "Number of info entries."
/// C: static size_t heapsize;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
static mut heapsize: size_t = 0;

/* C 347-391: `#if defined GC_MALLOC_CHECK && defined GC_PROTECT_MALLOC_STATE` - "Some code for
   hunting a bug writing into _heapinfo", the state-protection helper, and C 389-391's `#else`,
   which defines the macro empty (C evaluates no argument).  */
/// C: #include <sys/types.h>
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"), feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
include!("../lib/include/sys_types.in.rs");

/* C 359: `#include <sys/mman.h>` - no file in the reference tree and no mirror, so its surface
   stands here (P-88 (1)); measured with clang: `PROT_NONE` 0x00, `PROT_READ` 0x01,
   `PROT_WRITE` 0x02, `PROT_EXEC` 0x04, and `int mprotect(void *, size_t, int)`.  */
/// C: #include <sys/mman.h>
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
pub const PROT_NONE: c_int = 0x00;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
pub const PROT_READ: c_int = 0x01;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
pub const PROT_WRITE: c_int = 0x02;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
pub const PROT_EXEC: c_int = 0x04;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
unsafe extern "C" {
  /// C: sys/mman.h: `int mprotect(void *, size_t, int);`
  pub fn mprotect (addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
}

/// C: static int state_protected_p;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
static mut state_protected_p: c_int = 0;
/// C: static size_t last_state_size;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
static mut last_state_size: size_t = 0;
/// C: static malloc_info *last_heapinfo;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
static mut last_heapinfo: *mut gmalloc_info = core::ptr::null_mut ();

/// C 365-385: `protect_malloc_state`.
/// C: void protect_malloc_state (int protect_p);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn protect_malloc_state (protect_p: c_int)
{
  /* `abort` (C 383) is the C library's and is not on this unit's include graph (Owed).  */
  /// C: stdlib.h: `void abort(void) __attribute__((__cold__)) __attribute__((__noreturn__));`
  unsafe extern "C" { pub fn abort () -> !; }

  /* If _heapinfo has been relocated, make sure its old location
     isn't left read-only; it will be reused by malloc.  */
  if _heapinfo != last_heapinfo
     && !last_heapinfo.is_null ()
     && state_protected_p != 0
    { mprotect (last_heapinfo as *mut c_void, last_state_size, PROT_READ | PROT_WRITE); }

  last_state_size = _heaplimit * core::mem::size_of::<gmalloc_info> ();   // C 375: `sizeof *_heapinfo`
  last_heapinfo   = _heapinfo;

  if protect_p != state_protected_p
    {
      state_protected_p = protect_p;
      if mprotect (_heapinfo as *mut c_void, last_state_size,
                   if protect_p != 0 { PROT_READ } else { PROT_READ | PROT_WRITE }) != 0
        { abort (); }
    }
}

/// C 387: `#define PROTECT_MALLOC_STATE(PROT) protect_malloc_state (PROT)`
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))]
macro_rules! PROTECT_MALLOC_STATE { ($prot:expr) => { protect_malloc_state ($prot) }; }
/* C 389-391's `#else`: the macro is empty and its argument is not evaluated (PORTING.md 2.1's
   evaluation-count rule).  */
/// C: #define PROTECT_MALLOC_STATE(PROT)	/* empty */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (all (feature = "gc_malloc_check", feature = "gc_protect_malloc_state"))))]
macro_rules! PROTECT_MALLOC_STATE { ($prot:expr) => { }; }

/* Aligned allocation.  */
/// C 394-395: "Aligned allocation."
/// C: static void * align (size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
unsafe fn align (size: size_t) -> *mut c_void
{
  let mut result: *mut c_void = core::ptr::null_mut ();
  let mut adj: ptrdiff_t = 0;

  /* C 401-404: "align accepts an unsigned argument, but __morecore accepts a signed one.  This
     could lead to trouble if SIZE overflows the ptrdiff_t type accepted by __morecore.  We just
     punt in that case, since they are requesting a ludicrous amount anyway."  C's comparison
     converts PTRDIFF_MAX to `size_t`, and the port spells that conversion.  */
  if (PTRDIFF_MAX as size_t) < size
    { result = core::ptr::null_mut (); }
  else
    { result = (__morecore.unwrap ()) (size as ptrdiff_t); }
  adj = ((result as uintptr_t) % BLOCKSIZE! ()) as ptrdiff_t;
  if adj != 0
    {
      adj = BLOCKSIZE! () as ptrdiff_t - adj;
      (__morecore.unwrap ()) (adj);
      result = (result as *mut c_char).add (adj as usize) as *mut c_void;
    }

  if __after_morecore_hook.is_some ()
    { (__after_morecore_hook.unwrap ()) (); }

  return result;
}

/* Get SIZE bytes, if we can get them starting at END.
   Return the address of the space we got.
   If we cannot get space at END, fail and return 0.  */
/// C 423-426: "Get SIZE bytes, if we can get them starting at END. …"
/// C: static void * get_contiguous_space (ptrdiff_t size, void *position);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
unsafe fn get_contiguous_space (size: ptrdiff_t, position: *mut c_void) -> *mut c_void
{
  let mut before: *mut c_void = core::ptr::null_mut ();
  let mut after: *mut c_void = core::ptr::null_mut ();

  before = (__morecore.unwrap ()) (0);
  /* If we can tell in advance that the break is at the wrong place,
     fail now.  */
  if before != position
    { return core::ptr::null_mut (); }

  /* Allocate SIZE bytes and get the address of them.  */
  after = (__morecore.unwrap ()) (size);
  if after.is_null ()
    { return core::ptr::null_mut (); }

  /* It was not contiguous--reject it.  */
  if after != position
    {
      (__morecore.unwrap ()) ((size.wrapping_neg ()) as ptrdiff_t);
      return core::ptr::null_mut ();
    }

  return after;
}


/* This is called when `_heapinfo' and `heapsize' have just
   been set to describe a new info table.  Set up the table
   to describe itself and account for it in the statistics.  */
/// C 454-458: "This is called when `_heapinfo' and `heapsize' have just been set …"
/// C: static void register_heapinfo (void);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
unsafe fn register_heapinfo ()
{
  let mut block: size_t = 0;
  let mut blocks: size_t = 0;

  block = BLOCK! (_heapinfo);
  blocks = BLOCKIFY! (heapsize * core::mem::size_of::<gmalloc_info> ());

  /* Account for the _heapinfo block itself in the statistics.  */
  _bytes_used += blocks * BLOCKSIZE! ();
  _chunks_used += 1;

  /* Describe the heapinfo block itself in the heapinfo.  */
  (*_heapinfo.add (block)).busy.r#type = -1;
  (*_heapinfo.add (block)).busy.info.size = blocks as ptrdiff_t;
}

/* C 474-518: `#ifdef USE_PTHREAD` - the two mutexes, the thread flag, the three atfork handlers
   and `malloc_enable_thread`.  The three handlers' addresses are passed to `pthread_atfork`, so
   they are `unsafe extern "C" fn` (PORTING.md 1.4).  */
/// C 474-477: `pthread_mutex_t _malloc_mutex = PTHREAD_MUTEX_INITIALIZER;`
/// C: pthread_mutex_t _aligned_blocks_mutex = PTHREAD_MUTEX_INITIALIZER;
/// C: int _malloc_thread_enabled_p;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
#[unsafe(no_mangle)]
pub static mut _malloc_mutex: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
#[unsafe(no_mangle)]
pub static mut _aligned_blocks_mutex: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
#[unsafe(no_mangle)]
pub static mut _malloc_thread_enabled_p: c_int = 0;

/// C 479-484: `malloc_atfork_handler_prepare`.
/// C: static void malloc_atfork_handler_prepare (void);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
unsafe extern "C" fn malloc_atfork_handler_prepare ()
{
  LOCK! ();
  LOCK_ALIGNED_BLOCKS! ();
}

/// C 486-491: `malloc_atfork_handler_parent`.
/// C: static void malloc_atfork_handler_parent (void);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
unsafe extern "C" fn malloc_atfork_handler_parent ()
{
  UNLOCK_ALIGNED_BLOCKS! ();
  UNLOCK! ();
}

/// C 493-498: `malloc_atfork_handler_child`.
/// C: static void malloc_atfork_handler_child (void);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
unsafe extern "C" fn malloc_atfork_handler_child ()
{
  UNLOCK_ALIGNED_BLOCKS! ();
  UNLOCK! ();
}

/* Set up mutexes and make malloc etc. thread-safe.  */
/// C 500-502: "Set up mutexes and make malloc etc. thread-safe."
/// C: void malloc_enable_thread (void);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "use_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc_enable_thread ()
{
  if _malloc_thread_enabled_p != 0
    { return; }

  /* Some pthread implementations call malloc for statically
     initialized mutexes when they are used first.  To avoid such a
     situation, we initialize mutexes here while their use is
     disabled in malloc etc.  */
  pthread_mutex_init (&raw mut _malloc_mutex, core::ptr::null ());
  pthread_mutex_init (&raw mut _aligned_blocks_mutex, core::ptr::null ());
  pthread_atfork (Some (malloc_atfork_handler_prepare),
                  Some (malloc_atfork_handler_parent),
                  Some (malloc_atfork_handler_child));
  _malloc_thread_enabled_p = 1;
}

/// C 520-521: `malloc_initialize_1`.
/// C: static void malloc_initialize_1 (void);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
unsafe fn malloc_initialize_1 ()
{
  /* C 523-525's `#ifdef GC_MCHECK` / `mcheck (NULL);` / `#endif`.  */
  #[cfg (feature = "gc_mcheck")]
  { mcheck (None); }

  if __malloc_initialize_hook.is_some ()
    { (__malloc_initialize_hook.unwrap ()) (); }

  heapsize = HEAP! () / BLOCKSIZE! ();
  _heapinfo = align (heapsize * core::mem::size_of::<gmalloc_info> ()) as *mut gmalloc_info;
  if _heapinfo.is_null ()
    { return; }
  memset (_heapinfo as *mut c_void, 0, heapsize * core::mem::size_of::<gmalloc_info> ());
  (*_heapinfo).free.size = 0;
  (*_heapinfo).free.next = 0;
  (*_heapinfo).free.prev = 0;
  _heapindex = 0;
  _heapbase = _heapinfo as *mut c_char;
  _heaplimit = BLOCK! (_heapbase.add (heapsize * core::mem::size_of::<gmalloc_info> ()));

  register_heapinfo ();

  __malloc_initialized = 1;
  PROTECT_MALLOC_STATE! (1);
  return;
}

/* Set everything up and remember that we have.
   main will call malloc which calls this function.  That is before any threads
   or signal handlers has been set up, so we don't need thread protection.  */
/// C 548-552: "Set everything up and remember that we have. …"
/// C: int __malloc_initialize (void);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __malloc_initialize () -> c_int
{
  if __malloc_initialized != 0
    { return 0; }

  malloc_initialize_1 ();

  return __malloc_initialized;
}

/// C 562: `static int morecore_recursing;`
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
static mut morecore_recursing: c_int = 0;

/* Get neatly aligned memory, initializing or
   growing the heap info table as necessary. */
/// C 564-567: "Get neatly aligned memory, initializing or growing the heap info table as
/// necessary."
/// C: static void * morecore_nolock (size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
unsafe fn morecore_nolock (size: size_t) -> *mut c_void
{
  let mut result: *mut c_void = core::ptr::null_mut ();
  let mut newinfo: *mut gmalloc_info = core::ptr::null_mut ();
  let mut oldinfo: *mut gmalloc_info = core::ptr::null_mut ();
  let mut newsize: size_t = 0;

  if morecore_recursing != 0
    /* Avoid recursion.  The caller will know how to handle a null return.  */
    { return core::ptr::null_mut (); }

  result = align (size);
  if result.is_null ()
    { return core::ptr::null_mut (); }

  PROTECT_MALLOC_STATE! (0);

  /* Check if we need to grow the info table.  */
  /* C's `goto got_heap` (C 623) is the port's labeled-block break (PORTING.md 2.2) and the
     `got_heap:` tail (C 672-674) is the code after the block; the label keeps C's own name.  */
  'got_heap: {
  if heapsize < BLOCK! ((result as *mut c_char).add (size))
    {
      /* Calculate the new _heapinfo table size.  We do not account for the
	 added blocks in the table itself, as we hope to place them in
	 existing free space, which is already covered by part of the
	 existing table.  */
      newsize = heapsize;
      loop
	{
	  newsize *= 2;
	  if !(newsize < BLOCK! ((result as *mut c_char).add (size)))
	    { break; }
	}

      /* We must not reuse existing core for the new info table when called
	 from realloc in the case of growing a large block, because the
	 block being grown is momentarily marked as free.  In this case
	 _heaplimit is zero so we know not to reuse space for internal
	 allocation.  */
      if _heaplimit != 0
	{
	  /* First try to allocate the new info table in core we already
	     have, in the usual way using realloc.  If realloc cannot
	     extend it in place or relocate it to existing sufficient core,
	     we will get called again, and the code above will notice the
	     `morecore_recursing' flag and return null.  */
	  /* C's `errno` is `<errno.h>`'s `#define errno (*__error ())` on this target, so the
	     mirror's accessor is what the read and the write go through (`include/errno.rs` 14-17).  */
	  let mut save: c_int = *errno ();	/* Don't want to clobber errno with ENOMEM.  */
	  morecore_recursing = 1;
	  newinfo = _realloc_internal_nolock (_heapinfo as *mut c_void,
					      newsize * core::mem::size_of::<gmalloc_info> ())
	              as *mut gmalloc_info;
	  morecore_recursing = 0;
	  if newinfo.is_null ()
	    { *errno () = save; }
	  else
	    {
	      /* We found some space in core, and realloc has put the old
		 table's blocks on the free list.  Now zero the new part
		 of the table and install the new table location.  */
	      memset (newinfo.add (heapsize) as *mut c_void, 0,
		      (newsize - heapsize) * core::mem::size_of::<gmalloc_info> ());
	      _heapinfo = newinfo;
	      heapsize = newsize;
	      break 'got_heap;
	    }
	}

      /* Allocate new space for the malloc info table.  */
      loop
	{
	  newinfo = align (newsize * core::mem::size_of::<gmalloc_info> ()) as *mut gmalloc_info;

	  /* Did it fail?  */
	  if newinfo.is_null ()
	    {
	      (__morecore.unwrap ()) ((size.wrapping_neg ()) as ptrdiff_t);
	      return core::ptr::null_mut ();
	    }

	  /* Is it big enough to record status for its own space?
	     If so, we win.  */
	  if BLOCK! ((newinfo as *mut c_char).add (newsize * core::mem::size_of::<gmalloc_info> ()))
	     < newsize
	    { break; }

	  /* Must try again.  First give back most of what we just got.  */
	  (__morecore.unwrap ()) (((newsize * core::mem::size_of::<gmalloc_info> ()).wrapping_neg ()) as ptrdiff_t);
	  newsize *= 2;
	}

      /* Copy the old table to the beginning of the new,
	 and zero the rest of the new table.  */
      memcpy (newinfo as *mut c_void, _heapinfo as *const c_void,
	      heapsize * core::mem::size_of::<gmalloc_info> ());
      memset (newinfo.add (heapsize) as *mut c_void, 0,
	      (newsize - heapsize) * core::mem::size_of::<gmalloc_info> ());
      oldinfo = _heapinfo;
      _heapinfo = newinfo;
      heapsize = newsize;

      register_heapinfo ();

      /* Reset _heaplimit so _free_internal never decides
	 it can relocate or resize the info table.  */
      _heaplimit = 0;
      _free_internal_nolock (oldinfo as *mut c_void);
      PROTECT_MALLOC_STATE! (0);

      /* The new heap limit includes the new table just allocated.  */
      _heaplimit = BLOCK! ((newinfo as *mut c_char).add (heapsize * core::mem::size_of::<gmalloc_info> ()));
      return result;
    }
  }

 /* got_heap: */
  _heaplimit = BLOCK! ((result as *mut c_char).add (size));
  return result;
}

/* Allocate memory from the heap.  */
/// C 677-679: "Allocate memory from the heap."
/// C: void * _malloc_internal_nolock (size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _malloc_internal_nolock (size: size_t) -> *mut c_void
{
  let mut size = size;                          // C 679's `size` is assigned at C 701/709
  let mut result: *mut c_void = core::ptr::null_mut ();
  let mut block: size_t = 0;
  let mut blocks: size_t = 0;
  let mut lastblocks: size_t = 0;
  let mut start: size_t = 0;
  let mut i: size_t = 0;
  let mut next: *mut list = core::ptr::null_mut ();

  /* ANSI C allows `malloc (0)' to either return NULL, or to return a
     valid address you can realloc and free (though not dereference).

     It turns out that some extant code (sunrpc, at least Ultrix's version)
     expects `malloc (0)' to return non-NULL and breaks otherwise.
     Be compatible.  */

  /* C 693-696: `#if 0` / `if (size == 0) return NULL;` / `#endif` - dead in every configuration:
     dropped (the `femacs/exec/trace.rs` 1605 precedent).  */

  PROTECT_MALLOC_STATE! (0);

  'out: {
  if size < core::mem::size_of::<list> ()
    { size = core::mem::size_of::<list> (); }

  /* Determine the allocation policy based on the request size.  */
  if size <= BLOCKSIZE! () / 2
    {
      /* Small allocation to receive a fragment of a block.
	 Determine the logarithm to base two of the fragment size. */
      let mut log: size_t = 1;
      size -= 1;
      loop
	{
	  size /= 2;
	  if size == 0
	    { break; }
	  log += 1;
	}

      /* Look in the fragment lists for a
	 free fragment of the desired size. */
      next = _fraghead[log].next;
      if !next.is_null ()
	{
	  /* There are free fragments of this size.
	     Pop a fragment out of the fragment list and return it.
	     Update the block's nfree and first counters. */
	  result = next as *mut c_void;
	  (*(*next).prev).next = (*next).next;
	  if !(*next).next.is_null ()
	    { (*(*next).next).prev = (*next).prev; }
	  block = BLOCK! (result);
	  (*_heapinfo.add (block)).busy.info.frag.nfree -= 1;
	  if (*_heapinfo.add (block)).busy.info.frag.nfree != 0
	    { (*_heapinfo.add (block)).busy.info.frag.first
	        = ((*next).next as uintptr_t) % BLOCKSIZE! () >> log; }

	  /* Update the statistics.  */
	  _chunks_used += 1;
	  _bytes_used += 1 << log;
	  _chunks_free -= 1;
	  _bytes_free -= 1 << log;
	}
      else
	{
	  /* No free fragments of the desired size, so get a new block
	     and break it into fragments, returning the first.  */
	  #[cfg (feature = "gc_malloc_check")]
	  {
	    result = _malloc_internal_nolock (BLOCKSIZE! ());
	    PROTECT_MALLOC_STATE! (0);
	  }
	  #[cfg (all (not (feature = "gc_malloc_check"), feature = "use_pthread"))]
	  { result = _malloc_internal_nolock (BLOCKSIZE! ()); }
	  #[cfg (all (not (feature = "gc_malloc_check"), not (feature = "use_pthread")))]
	  { result = malloc (BLOCKSIZE! ()); }
	  if result.is_null ()
	    {
	      PROTECT_MALLOC_STATE! (1);
	      break 'out;
	    }

	  /* Link all fragments but the first into the free list.  */
	  next = (result as *mut c_char).add (1usize << log) as *mut list;
	  (*next).next = core::ptr::null_mut ();
	  (*next).prev = &raw mut _fraghead[log];
	  _fraghead[log].next = next;

	  i = 2;
	  while i < (BLOCKSIZE! () >> log) as size_t
	    {
	      next = (result as *mut c_char).add (i << log) as *mut list;
	      (*next).next = _fraghead[log].next;
	      (*next).prev = &raw mut _fraghead[log];
	      (*(*next).prev).next = next;
	      (*(*next).next).prev = next;
	      i += 1;
	    }

	  /* Initialize the nfree and first counters for this block.  */
	  block = BLOCK! (result);
	  (*_heapinfo.add (block)).busy.r#type = log as c_int;
	  (*_heapinfo.add (block)).busy.info.frag.nfree = i - 1;
	  (*_heapinfo.add (block)).busy.info.frag.first = i - 1;

	  _chunks_free += (BLOCKSIZE! () >> log) - 1;
	  _bytes_free += BLOCKSIZE! () - (1 << log);
	  _bytes_used -= BLOCKSIZE! () - (1 << log);
	}
    }
  else
    {
      /* Large allocation to receive one or more blocks.
	 Search the free list in a circle starting at the last place visited.
	 If we loop completely around without finding a large enough
	 space we will have to get more memory from the system.  */
      blocks = BLOCKIFY! (size);
      block = _heapindex;
      start = block;
      while (*_heapinfo.add (block)).free.size < blocks
	{
	  block = (*_heapinfo.add (block)).free.next;
	  if block == start
	    {
	      /* Need to get more from the system.  Get a little extra.  */
	      let mut wantblocks: size_t = blocks + __malloc_extra_blocks;
	      block = (*_heapinfo).free.prev;
	      lastblocks = (*_heapinfo.add (block)).free.size;
	      /* Check to see if the new core will be contiguous with the
		 final free block; if so we don't need to get as much.  */
	      if _heaplimit != 0 && block + lastblocks == _heaplimit &&
		  /* We can't do this if we will have to make the heap info
                     table bigger to accommodate the new space.  */
		  block + wantblocks <= heapsize &&
		  !get_contiguous_space (((wantblocks - lastblocks) * BLOCKSIZE! ()) as ptrdiff_t,
					  ADDRESS! (block + lastblocks)).is_null ()
		{
		  /* We got it contiguously.  Which block we are extending
		     (the `final free block' referred to above) might have
		     changed, if it got combined with a freed info table.  */
		  block = (*_heapinfo).free.prev;
		  (*_heapinfo.add (block)).free.size += (wantblocks - lastblocks);
		  _bytes_free += (wantblocks - lastblocks) * BLOCKSIZE! ();
		  _heaplimit += wantblocks - lastblocks;
		  continue;
		}
	      result = morecore_nolock (wantblocks * BLOCKSIZE! ());
	      if result.is_null ()
		{ break 'out; }
	      block = BLOCK! (result);
	      /* Put the new block at the end of the free list.  */
	      (*_heapinfo.add (block)).free.size = wantblocks;
	      (*_heapinfo.add (block)).free.prev = (*_heapinfo).free.prev;
	      (*_heapinfo.add (block)).free.next = 0;
	      (*_heapinfo).free.prev = block;
	      (*_heapinfo.add ((*_heapinfo.add (block)).free.prev)).free.next = block;
	      _chunks_free += 1;
	      /* Now loop to use some of that block for this allocation.  */
	    }
	}

      /* At this point we have found a suitable free list entry.
	 Figure out how to remove what we need from the list. */
      result = ADDRESS! (block);
      if (*_heapinfo.add (block)).free.size > blocks
	{
	  /* The block we found has a bit left over,
	     so relink the tail end back into the free list. */
	  (*_heapinfo.add (block + blocks)).free.size
	    = (*_heapinfo.add (block)).free.size - blocks;
	  (*_heapinfo.add (block + blocks)).free.next
	    = (*_heapinfo.add (block)).free.next;
	  (*_heapinfo.add (block + blocks)).free.prev
	    = (*_heapinfo.add (block)).free.prev;
	  /* C 843-845's chained assignment `_heapinfo[…prev].free.next = _heapinfo[…next].free.prev
	     = _heapindex = block + blocks` is three stores in C's right-to-left order, and both
	     index expressions are read from the state before the first store (the deviation is
	     recorded in the ledger).  */
	  let index: size_t = block + blocks;
	  let tail_next: size_t = (*_heapinfo.add (block)).free.next;
	  let tail_prev: size_t = (*_heapinfo.add (block)).free.prev;
	  _heapindex = index;
	  (*_heapinfo.add (tail_next)).free.prev = index;
	  (*_heapinfo.add (tail_prev)).free.next = index;
	}
      else
	{
	  /* The block exactly matches our requirements,
	     so just remove it from the list. */
	  (*_heapinfo.add ((*_heapinfo.add (block)).free.next)).free.prev
	    = (*_heapinfo.add (block)).free.prev;
	  /* C 853-854's chained assignment, read the same way as C 843-845 above.  */
	  let next_index: size_t = (*_heapinfo.add (block)).free.next;
	  let prev_index: size_t = (*_heapinfo.add (block)).free.prev;
	  _heapindex = next_index;
	  (*_heapinfo.add (prev_index)).free.next = next_index;
	  _chunks_free -= 1;
	}

      (*_heapinfo.add (block)).busy.r#type = -1;
      (*_heapinfo.add (block)).busy.info.size = blocks as ptrdiff_t;
      _chunks_used += 1;
      _bytes_used += blocks * BLOCKSIZE! ();
      _bytes_free -= blocks * BLOCKSIZE! ();
    }

  PROTECT_MALLOC_STATE! (1);
  }

 /* out: */
  return result;
}

/// C 870-871: `_malloc_internal`.
/// C: void * _malloc_internal (size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _malloc_internal (size: size_t) -> *mut c_void
{
  let mut result: *mut c_void = core::ptr::null_mut ();

  LOCK! ();
  result = _malloc_internal_nolock (size);
  UNLOCK! ();

  return result;
}

/// C 882-883: `malloc` - C's `#define malloc gmalloc` (C 66) makes this definition the `gmalloc`
/// symbol, which the port carries with `export_name` (PORTING.md 0.2's spelling for a definition
/// whose text name and symbol differ).
/// C: void * malloc (size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "gmalloc")]
pub unsafe extern "C" fn malloc (size: size_t) -> *mut c_void
{
  let mut hook: Option<unsafe extern "C" fn (size_t) -> *mut c_void> = None;

  if __malloc_initialized == 0 && __malloc_initialize () == 0
    { return core::ptr::null_mut (); }

  /* Copy the value of gmalloc_hook to an automatic variable in case
     gmalloc_hook is modified in another thread between its
     NULL-check and the use.

     Note: Strictly speaking, this is not a right solution.  We should
     use mutexes to access non-read-only variables that are shared
     among multiple threads.  We just leave it for compatibility with
     glibc malloc (i.e., assignments to gmalloc_hook) for now.  */
  hook = gmalloc_hook;
  return (if hook.is_some () { hook.unwrap () } else { _malloc_internal }) (size);
}

/* C 902-929: `#if !(defined (_LIBC))` - "On some ANSI C systems, some libc functions call
   _malloc, _free and _realloc.  Make them use the GNU functions."  C 907-909's three `extern`
   declarations are the forward declarations of the definitions below (dropped, as at C 78-93);
   the definitions themselves are non-`static`, so each keeps its symbol.  */
/// C 911-912: `_malloc`.
/// C: void * _malloc (size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (feature = "libc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _malloc (size: size_t) -> *mut c_void
{
  return malloc (size);
}

/// C 917-918: `_free`.
/// C: void _free (void *ptr);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (feature = "libc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _free (ptr: *mut c_void)
{
  free (ptr);
}

/// C 923-924: `_realloc`.
/// C: void * _realloc (void *ptr, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (feature = "libc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _realloc (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  return realloc (ptr, size);
}

/* C 951-952: "Debugging hook for free."  */
/// C: static void (*__MALLOC_HOOK_VOLATILE gfree_hook) (void *);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
static mut gfree_hook: Option<unsafe extern "C" fn (*mut c_void)> = None;

/// C 954-955: "List of blocks allocated by aligned_alloc."
/// C: struct alignlist *_aligned_blocks = NULL;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut _aligned_blocks: *mut alignlist = core::ptr::null_mut ();

/* Return memory to the heap.
   Like `_free_internal' but don't lock mutex.  */
/// C 957-960: "Return memory to the heap.  Like `_free_internal' but don't lock mutex."
/// C: void _free_internal_nolock (void *ptr);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _free_internal_nolock (ptr: *mut c_void)
{
  let mut ptr: *mut c_void = ptr;
  let mut r#type: c_int = 0;
  let mut block: size_t = 0;
  let mut blocks: size_t = 0;
  let mut i: size_t = 0;
  let mut prev: *mut list = core::ptr::null_mut ();
  let mut next: *mut list = core::ptr::null_mut ();
  let mut curbrk: *mut c_void = core::ptr::null_mut ();
  let lesscore_threshold: size_t
    /* Threshold of free space at which we will return some to the system.  */
    = FINAL_FREE_BLOCKS as size_t + 2 * __malloc_extra_blocks;

  let mut l: *mut alignlist = core::ptr::null_mut ();

  if ptr.is_null ()
    { return; }

  PROTECT_MALLOC_STATE! (0);

  LOCK_ALIGNED_BLOCKS! ();
  l = _aligned_blocks;
  while !l.is_null ()
    {
      if (*l).aligned == ptr
	{
	  (*l).aligned = core::ptr::null_mut ();	/* Mark the slot in the list as free.  */
	  ptr = (*l).exact;
	  break;
	}
      l = (*l).next;
    }
  UNLOCK_ALIGNED_BLOCKS! ();

  block = BLOCK! (ptr);

  r#type = (*_heapinfo.add (block)).busy.r#type;
  match r#type
    {
    -1 =>
      {
      /* Get as many statistics as early as we can.  */
      _chunks_used -= 1;
      _bytes_used -= ((*_heapinfo.add (block)).busy.info.size
		      * (BLOCKSIZE! () as ptrdiff_t)) as size_t;
      _bytes_free += ((*_heapinfo.add (block)).busy.info.size
		      * (BLOCKSIZE! () as ptrdiff_t)) as size_t;

      /* Find the free cluster previous to this one in the free list.
	 Start searching at the last block referenced; this may benefit
	 programs with locality of allocation.  */
      i = _heapindex;
      if i > block
	{
	  while i > block
	    { i = (*_heapinfo.add (i)).free.prev; }
	}
      else
	{
	  loop
	    {
	      i = (*_heapinfo.add (i)).free.next;
	      if !(i > 0 && i < block)
		{ break; }
	    }
	  i = (*_heapinfo.add (i)).free.prev;
	}

      /* Determine how to link this block into the free list.  */
      if block == i + (*_heapinfo.add (i)).free.size
	{
	  /* Coalesce this block with its predecessor.  */
	  (*_heapinfo.add (i)).free.size
	    += (*_heapinfo.add (block)).busy.info.size as size_t;
	  block = i;
	}
      else
	{
	  /* Really link this block back into the free list.  */
	  (*_heapinfo.add (block)).free.size
	    = (*_heapinfo.add (block)).busy.info.size as size_t;
	  (*_heapinfo.add (block)).free.next = (*_heapinfo.add (i)).free.next;
	  (*_heapinfo.add (block)).free.prev = i;
	  (*_heapinfo.add (i)).free.next = block;
	  (*_heapinfo.add ((*_heapinfo.add (block)).free.next)).free.prev = block;
	  _chunks_free += 1;
	}

      /* Now that the block is linked in, see if we can coalesce it
	 with its successor (by deleting its successor from the list
	 and adding in its size).  */
      if block + (*_heapinfo.add (block)).free.size == (*_heapinfo.add (block)).free.next
	{
	  (*_heapinfo.add (block)).free.size
	    += (*_heapinfo.add ((*_heapinfo.add (block)).free.next)).free.size;
	  (*_heapinfo.add (block)).free.next
	    = (*_heapinfo.add ((*_heapinfo.add (block)).free.next)).free.next;
	  (*_heapinfo.add ((*_heapinfo.add (block)).free.next)).free.prev = block;
	  _chunks_free -= 1;
	}

      /* How many trailing free blocks are there now?  */
      blocks = (*_heapinfo.add (block)).free.size;

      /* Where is the current end of accessible core?  */
      curbrk = (__morecore.unwrap ()) (0);

      if _heaplimit != 0 && curbrk == ADDRESS! (_heaplimit)
	{
	  /* The end of the malloc heap is at the end of accessible core.
	     It's possible that moving _heapinfo will allow us to
	     return some space to the system.  */

	  let info_block: size_t = BLOCK! (_heapinfo);
	  let info_blocks: size_t = (*_heapinfo.add (info_block)).busy.info.size as size_t;
	  let prev_block: size_t = (*_heapinfo.add (block)).free.prev;
	  let prev_blocks: size_t = (*_heapinfo.add (prev_block)).free.size;
	  let next_block: size_t = (*_heapinfo.add (block)).free.next;
	  let next_blocks: size_t = (*_heapinfo.add (next_block)).free.size;

	  if /* Win if this block being freed is last in core, the info table
		is just before it, the previous free block is just before the
		info table, and the two free blocks together form a useful
		amount to return to the system.  */
	     (block + blocks == _heaplimit &&
	      info_block + info_blocks == block &&
	      prev_block != 0 && prev_block + prev_blocks == info_block &&
	      blocks + prev_blocks >= lesscore_threshold) ||
	     /* Nope, not the case.  We can also win if this block being
		freed is just before the info table, and the table extends
		to the end of core or is followed only by a free block,
		and the total free space is worth returning to the system.  */
	     (block + blocks == info_block &&
	      ((info_block + info_blocks == _heaplimit &&
		blocks >= lesscore_threshold) ||
	       (info_block + info_blocks == next_block &&
		next_block + next_blocks == _heaplimit &&
		blocks + next_blocks >= lesscore_threshold)))
	    {
	      let mut newinfo: *mut gmalloc_info = core::ptr::null_mut ();
	      let oldlimit: size_t = _heaplimit;

	      /* Free the old info table, clearing _heaplimit to avoid
		 recursion into this code.  We don't want to return the
		 table's blocks to the system before we have copied them to
		 the new location.  */
	      _heaplimit = 0;
	      _free_internal_nolock (_heapinfo as *mut c_void);
	      _heaplimit = oldlimit;

	      /* Tell malloc to search from the beginning of the heap for
		 free blocks, so it doesn't reuse the ones just freed.  */
	      _heapindex = 0;

	      /* Allocate new space for the info table and move its data.  */
	      newinfo = _malloc_internal_nolock (info_blocks * BLOCKSIZE! ()) as *mut gmalloc_info;
	      PROTECT_MALLOC_STATE! (0);
	      memmove (newinfo as *mut c_void, _heapinfo as *const c_void,
		       info_blocks * BLOCKSIZE! ());
	      _heapinfo = newinfo;

	      /* We should now have coalesced the free block with the
		 blocks freed from the old info table.  Examine the entire
		 trailing free block to decide below whether to return some
		 to the system.  */
	      block = (*_heapinfo).free.prev;
	      blocks = (*_heapinfo.add (block)).free.size;
	    }

	  /* Now see if we can return stuff to the system.  */
	  if block + blocks == _heaplimit && blocks >= lesscore_threshold
	    {
	      let bytes: size_t = blocks * BLOCKSIZE! ();
	      _heaplimit -= blocks;
	      (__morecore.unwrap ()) ((bytes.wrapping_neg ()) as ptrdiff_t);
	      (*_heapinfo.add ((*_heapinfo.add (block)).free.prev)).free.next
		= (*_heapinfo.add (block)).free.next;
	      (*_heapinfo.add ((*_heapinfo.add (block)).free.next)).free.prev
		= (*_heapinfo.add (block)).free.prev;
	      block = (*_heapinfo.add (block)).free.prev;
	      _chunks_free -= 1;
	      _bytes_free -= bytes;
	    }
	}

      /* Set the next search to begin at this block.  */
      _heapindex = block;
      }

    _ =>
      {
      /* Do some of the statistics.  */
      _chunks_used -= 1;
      _bytes_used -= ((1 as c_int) << r#type) as size_t;
      _chunks_free += 1;
      _bytes_free += ((1 as c_int) << r#type) as size_t;

      /* Get the address of the first free fragment in this block.  */
      prev = (ADDRESS! (block) as *mut c_char)
	     .add (((*_heapinfo.add (block)).busy.info.frag.first << r#type) as usize)
	     as *mut list;

      if (*_heapinfo.add (block)).busy.info.frag.nfree
	 == ((BLOCKSIZE! () >> r#type) as size_t) - 1
	{
	  /* If all fragments of this block are free, remove them
	     from the fragment list and free the whole block.  */
	  next = prev;
	  i = 1;
	  while i < (BLOCKSIZE! () >> r#type) as size_t
	    {
	      next = (*next).next;
	      i += 1;
	    }
	  (*(*prev).prev).next = next;
	  if !next.is_null ()
	    { (*next).prev = (*prev).prev; }
	  (*_heapinfo.add (block)).busy.r#type = -1;
	  (*_heapinfo.add (block)).busy.info.size = 1;

	  /* Keep the statistics accurate.  */
	  _chunks_used += 1;
	  _bytes_used += BLOCKSIZE! ();
	  _chunks_free -= (BLOCKSIZE! () >> r#type) as size_t;
	  _bytes_free -= BLOCKSIZE! ();

	  #[cfg (any (feature = "gc_malloc_check", feature = "use_pthread"))]
	  { _free_internal_nolock (ADDRESS! (block)); }
	  #[cfg (not (any (feature = "gc_malloc_check", feature = "use_pthread")))]
	  { free (ADDRESS! (block)); }
	}
      else if (*_heapinfo.add (block)).busy.info.frag.nfree != 0
	{
	  /* If some fragments of this block are free, link this
	     fragment into the fragment list after the first free
	     fragment of this block. */
	  next = ptr as *mut list;
	  (*next).next = (*prev).next;
	  (*next).prev = prev;
	  (*prev).next = next;
	  if !(*next).next.is_null ()
	    { (*(*next).next).prev = next; }
	  (*_heapinfo.add (block)).busy.info.frag.nfree += 1;
	}
      else
	{
	  /* No fragments of this block are free, so link this
	     fragment into the fragment list and announce that
	     it is the first free fragment of this block. */
	  prev = ptr as *mut list;
	  (*_heapinfo.add (block)).busy.info.frag.nfree = 1;
	  (*_heapinfo.add (block)).busy.info.frag.first
	    = ((ptr as uintptr_t) % BLOCKSIZE! () >> r#type) as size_t;
	  (*prev).next = _fraghead[r#type as usize].next;
	  (*prev).prev = &raw mut _fraghead[r#type as usize];
	  (*(*prev).prev).next = prev;
	  if !(*prev).next.is_null ()
	    { (*(*prev).next).prev = prev; }
	}
      }
    }

  PROTECT_MALLOC_STATE! (1);
}

/* Return memory to the heap.
   Like 'free' but don't call a hook if there is one.  */
/// C 1203-1206: "Return memory to the heap.  Like 'free' but don't call a hook if there is one."
/// C: void _free_internal (void *ptr);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _free_internal (ptr: *mut c_void)
{
  LOCK! ();
  _free_internal_nolock (ptr);
  UNLOCK! ();
}

/* Return memory to the heap.  */
/// C 1213-1216: "Return memory to the heap." - `#define free gfree` (C 70) makes this the `gfree`
/// symbol.
/// C: void free (void *ptr);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "gfree")]
pub unsafe extern "C" fn free (ptr: *mut c_void)
{
  let hook: Option<unsafe extern "C" fn (*mut c_void)> = gfree_hook;

  if hook.is_some ()
    { (hook.unwrap ()) (ptr); }
  else
    { _free_internal (ptr); }
}

/* C 1226-1235: `#ifdef weak_alias` / `weak_alias (free, cfree)` / `#else` / the `cfree`
   definition / `#endif`.  C's `weak_alias` is glibc's alias directive; the port declares it the
   way `femacs/lib/euidaccess.rs` 197-204 does, and the `#else` arm - this target's - defines the
   real `cfree`.  */
/// C: #ifdef weak_alias
/// C: weak_alias (free, cfree)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "weak_alias"))]
unsafe extern "C" { pub fn weak_alias (old: *const c_char, new: *const c_char); }
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "weak_alias"))]
unsafe fn weak_alias_cfree () { weak_alias (c"free".as_ptr (), c"cfree".as_ptr ()); }
/// C 1230-1231: `cfree`.
/// C: void cfree (void *ptr);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (feature = "weak_alias")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfree (ptr: *mut c_void)
{
  free (ptr);
}

/* C 1257-1258: "Debugging hook for realloc."  */
/// C: static void *(*grealloc_hook) (void *, size_t);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
static mut grealloc_hook: Option<unsafe extern "C" fn (*mut c_void, size_t) -> *mut c_void> = None;

/* Resize the given region to the new size, returning a pointer
   to the (possibly moved) region.  This is optimized for speed;
   some benchmarks seem to indicate that greater compactness is
   achieved by unconditionally allocating and copying to a
   new region.  This module has incestuous knowledge of the
   internals of both free and malloc. */
/// C 1260-1267: "Resize the given region to the new size, returning a pointer to the (possibly
/// moved) region. …"
/// C: void * _realloc_internal_nolock (void *ptr, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _realloc_internal_nolock (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  let mut result: *mut c_void = core::ptr::null_mut ();
  let mut r#type: c_int = 0;
  let mut block: size_t = 0;
  let mut blocks: size_t = 0;
  let mut oldlimit: size_t = 0;

  'out: {
  if size == 0
    {
      _free_internal_nolock (ptr);
      return _malloc_internal_nolock (0);
    }
  else if ptr.is_null ()
    { return _malloc_internal_nolock (size); }

  block = BLOCK! (ptr);

  PROTECT_MALLOC_STATE! (0);

  r#type = (*_heapinfo.add (block)).busy.r#type;
  match r#type
    {
    -1 =>
      {
      /* Maybe reallocate a large block to a small fragment.  */
      if size <= BLOCKSIZE! () / 2
	{
	  result = _malloc_internal_nolock (size);
	  if !result.is_null ()
	    {
	      memcpy (result, ptr as *const c_void, size);
	      _free_internal_nolock (ptr);
	      break 'out;
	    }
	}

      /* The new size is a large allocation as well;
	 see if we can hold it in place. */
      blocks = BLOCKIFY! (size);
      if blocks < (*_heapinfo.add (block)).busy.info.size as size_t
	{
	  /* The new size is smaller; return
	     excess memory to the free list. */
	  (*_heapinfo.add (block + blocks)).busy.r#type = -1;
	  (*_heapinfo.add (block + blocks)).busy.info.size
	    = (*_heapinfo.add (block)).busy.info.size - blocks as ptrdiff_t;
	  (*_heapinfo.add (block)).busy.info.size = blocks as ptrdiff_t;
	  /* We have just created a new chunk by splitting a chunk in two.
	     Now we will free this chunk; increment the statistics counter
	     so it doesn't become wrong when _free_internal decrements it.  */
	  _chunks_used += 1;
	  _free_internal_nolock (ADDRESS! (block + blocks));
	  result = ptr;
	}
      else if blocks == (*_heapinfo.add (block)).busy.info.size as size_t
	/* No size change necessary.  */
	{ result = ptr; }
      else
	{
	  /* Won't fit, so allocate a new region that will.
	     Free the old region first in case there is sufficient
	     adjacent free space to grow without moving. */
	  blocks = (*_heapinfo.add (block)).busy.info.size as size_t;
	  /* Prevent free from actually returning memory to the system.  */
	  oldlimit = _heaplimit;
	  _heaplimit = 0;
	  _free_internal_nolock (ptr);
	  result = _malloc_internal_nolock (size);
	  PROTECT_MALLOC_STATE! (0);
	  if _heaplimit == 0
	    { _heaplimit = oldlimit; }
	  if result.is_null ()
	    {
	      /* Now we're really in trouble.  We have to unfree
		 the thing we just freed.  Unfortunately it might
		 have been coalesced with its neighbors.  */
	      if _heapindex == block
	        { let _ = _malloc_internal_nolock (blocks * BLOCKSIZE! ()); }
	      else
		{
		  let previous: *mut c_void
		    = _malloc_internal_nolock ((block - _heapindex) * BLOCKSIZE! ());
		  let _ = _malloc_internal_nolock (blocks * BLOCKSIZE! ());
		  _free_internal_nolock (previous);
		}
	      break 'out;
	    }
	  if ptr != result
	    { memmove (result, ptr as *const c_void, blocks * BLOCKSIZE! ()); }
	}
      }

    _ =>
      {
      /* Old size is a fragment; type is logarithm
	 to base two of the fragment size.  */
      /* C 1371's `min (size, (size_t) 1 << type)` is `lisp.h` 72's object-like macro, which
	 `include/lisp.rs` does not carry and no mirror on this unit's include graph has (Owed).  */
      /// C: #define min(a, b) ((a) < (b) ? (a) : (b))
      macro_rules! min { ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } }; }
      if size > ((1 as size_t) << (r#type - 1)) &&
	  size <= ((1 as size_t) << r#type)
	/* The new size is the same kind of fragment.  */
	{ result = ptr; }
      else
	{
	  /* The new size is different; allocate a new space,
	     and copy the lesser of the new size and the old. */
	  result = _malloc_internal_nolock (size);
	  if result.is_null ()
	    { break 'out; }
	  memcpy (result, ptr as *const c_void, min! (size, (1 as size_t) << r#type));
	  _free_internal_nolock (ptr);
	}
      }
    }

  PROTECT_MALLOC_STATE! (1);
  }

 /* out: */
  return result;
}

/// C 1382-1383: `_realloc_internal`.
/// C: void * _realloc_internal (void *ptr, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _realloc_internal (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  let mut result: *mut c_void = core::ptr::null_mut ();

  LOCK! ();
  result = _realloc_internal_nolock (ptr, size);
  UNLOCK! ();

  return result;
}

/// C 1394-1395: `realloc` - `#define realloc grealloc` (C 67) makes this the `grealloc` symbol.
/// C: void * realloc (void *ptr, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "grealloc")]
pub unsafe extern "C" fn realloc (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  let mut hook: Option<unsafe extern "C" fn (*mut c_void, size_t) -> *mut c_void> = None;

  if __malloc_initialized == 0 && __malloc_initialize () == 0
    { return core::ptr::null_mut (); }

  hook = grealloc_hook;
  return (if hook.is_some () { hook.unwrap () } else { _realloc_internal }) (ptr, size);
}

/* C 1424-1425: "Allocate an array of NMEMB elements each SIZE bytes long.  The entire array is
   initialized to zeros."  */
/// C 1424-1427: `calloc` - `#define calloc gcalloc` (C 68) makes this the `gcalloc` symbol.
/// C: void * calloc (size_t nmemb, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "gcalloc")]
pub unsafe extern "C" fn calloc (nmemb: size_t, size: size_t) -> *mut c_void
{
  let mut result: *mut c_void = core::ptr::null_mut ();
  let bytes: size_t = nmemb * size;

  if size != 0 && bytes / size != nmemb
    {
      *errno () = ENOMEM;
      return core::ptr::null_mut ();
    }

  result = malloc (bytes);
  if !result.is_null ()
    { return memset (result, 0, bytes); }
  return result;
}

/* C 1460-1469: "uClibc defines __GNU_LIBRARY__, but it is not completely compatible."  The
   `#if`'s live arm for this target is `#define __sbrk sbrk`; the other arm declares `__sbrk`
   itself and leaves its result uncast ("It is best not to declare this and cast its result on
   foreign operating systems with potentially hostile include files.").  */
/* `sbrk` (C 1478 reaches it through the `__sbrk` rename below): `<unistd.h>` declares it and no
   mirror on this unit's include graph carries it, so it stands at its use site (P-88 (1); Owed).  */
/// C: unistd.h: `void *sbrk (intptr_t increment);`
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            any (not (feature = "__gnu_library__"), feature = "__uclibc__")))]
unsafe extern "C" { pub fn sbrk (increment: ptrdiff_t) -> *mut c_void; }
/// C: #if !defined (__GNU_LIBRARY__) || defined (__UCLIBC__)
/// C: #define	__sbrk	sbrk
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            any (not (feature = "__gnu_library__"), feature = "__uclibc__")))]
macro_rules! __sbrk { ($increment:expr) => { sbrk ($increment) }; }
/// C: extern void *__sbrk (ptrdiff_t increment);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            not (any (not (feature = "__gnu_library__"), feature = "__uclibc__"))))]
unsafe extern "C" { pub fn __sbrk (increment: ptrdiff_t) -> *mut c_void; }

/* C 1471-1483: `gdefault_morecore` - the address C 1485 stores in `__morecore`, so it is
   `unsafe extern "C" fn` (PORTING.md 1.4).  */
/// C 1471-1475: "Allocate INCREMENT more bytes of data space, and return the start of data
/// space, or NULL on errors.  If INCREMENT is negative, shrink data space."
/// C: static void * gdefault_morecore (ptrdiff_t increment);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
unsafe extern "C" fn gdefault_morecore (increment: ptrdiff_t) -> *mut c_void
{
  #[cfg (feature = "have_sbrk")]
  {
    let result: *mut c_void = __sbrk! (increment);
    if result != (-1 as isize) as *mut c_void
      { return result; }
  }
  return core::ptr::null_mut ();
}

/* C 1485: `void *(*__morecore) (ptrdiff_t) = gdefault_morecore;` - C's definition-with-initializer
   of the object C 314 declares tentatively.  The port keeps the one definition point (Rust has no
   tentative definitions) and spells C's initializer.  */
/// C: void *(*__morecore) (ptrdiff_t) = gdefault_morecore;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub static mut __morecore: Option<unsafe extern "C" fn (ptrdiff_t) -> *mut c_void>
  = Some (gdefault_morecore);

/// C 1503-1504: `aligned_alloc` - `#define aligned_alloc galigned_alloc` (C 69) makes this the
/// `galigned_alloc` symbol.
/// C: void * aligned_alloc (size_t alignment, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "galigned_alloc")]
pub unsafe extern "C" fn aligned_alloc (alignment: size_t, size: size_t) -> *mut c_void
{
  let mut result: *mut c_void = core::ptr::null_mut ();
  let mut adj: size_t = 0;
  let mut lastadj: size_t = 0;

  /* Allocate a block with enough extra space to pad the block with up to
     (ALIGNMENT - 1) bytes if necessary.  */
  if size.wrapping_neg () < alignment
    {
      *errno () = ENOMEM;
      return core::ptr::null_mut ();
    }
  result = malloc (size + alignment - 1);
  if result.is_null ()
    { return core::ptr::null_mut (); }

  /* Figure out how much we will need to pad this particular block
     to achieve the required alignment.  */
  adj = alignment - (result as uintptr_t) % alignment;
  if adj == alignment
    { adj = 0; }

  if adj != alignment - 1
    {
      loop
	{
	  /* Reallocate the block with only as much excess as it
	     needs.  */
	  free (result);
	  result = malloc (size + adj);
	  if result.is_null ()	/* Impossible unless interrupted.  */
	    { return core::ptr::null_mut (); }

	  lastadj = adj;
	  adj = alignment - (result as uintptr_t) % alignment;
	  if adj == alignment
	    { adj = 0; }
	  /* It's conceivable we might have been so unlucky as to get
	     a different block with weaker alignment.  If so, this
	     block is too short to contain SIZE after alignment
	     correction.  So we must try again and get another block,
	     slightly larger.  */
	  if !(adj > lastadj)
	    { break; }
	}
    }

  if adj != 0
    {
      /* Record this block in the list of aligned blocks, so that `free'
	 can identify the pointer it is passed, which will be in the middle
	 of an allocated block.  */

      let mut l: *mut alignlist = core::ptr::null_mut ();
      LOCK_ALIGNED_BLOCKS! ();
      l = _aligned_blocks;
      while !l.is_null ()
	{
	  if (*l).aligned.is_null ()
	    /* This slot is free.  Use it.  */
	    { break; }
	  l = (*l).next;
	}
      if l.is_null ()
	{
	  l = malloc (core::mem::size_of::<alignlist> ()) as *mut alignlist;
	  if !l.is_null ()
	    {
	      (*l).next = _aligned_blocks;
	      _aligned_blocks = l;
	    }
	}
      if !l.is_null ()
	{
	  (*l).exact = result;
	  (*l).aligned = (result as *mut c_char).add (adj) as *mut c_void;
	  result = (*l).aligned;
	}
      UNLOCK_ALIGNED_BLOCKS! ();
      if l.is_null ()
	{
	  free (result);
	  result = core::ptr::null_mut ();
	}
    }

  return result;
}

/* Note that memalign and posix_memalign are not used in Emacs.  */
/* An obsolete alias for aligned_alloc, for any old libraries that use
   this alias.  */

/// C 1586-1591: "Note that memalign and posix_memalign are not used in Emacs.  …"
/// C: void * memalign (size_t alignment, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memalign (alignment: size_t, size: size_t) -> *mut c_void
{
  return aligned_alloc (alignment, size);
}

/// C 1596-1597: `posix_memalign`.
/// C: int posix_memalign (void **memptr, size_t alignment, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_memalign (memptr: *mut *mut c_void, alignment: size_t, size: size_t)
  -> c_int
{
  let mut mem: *mut c_void = core::ptr::null_mut ();

  if alignment == 0
     || alignment % core::mem::size_of::<*mut c_void> () != 0
     || (alignment & (alignment - 1)) != 0
    { return EINVAL; }

  mem = aligned_alloc (alignment, size);
  if mem.is_null ()
    { return ENOMEM; }

  *memptr = mem;

  return 0;
}

/* Allocate memory on a page boundary.
   (C 1615-1633 is this section's own opening block, which C 1635's `#ifndef HAVE_MALLOC_H`
   follows.)  */

/* C 1635-1638: `#ifndef HAVE_MALLOC_H` / `extern void *valloc (size_t);` / `#endif` - the
   forward declaration of the function C 1648 defines (dropped, as at C 78-93).  */
/* C 1640-1644: `#if defined _SC_PAGESIZE || !defined HAVE_GETPAGESIZE` /
   `# include "getpagesize.h"` / `#elif !defined getpagesize` / `extern int getpagesize (void);` /
   `#endif`.  The first arm is this target's (`_SC_PAGESIZE` is 29); its include has no live text
   here (the header's whole body is `#ifndef HAVE_GETPAGESIZE`, and HAVE_GETPAGESIZE is 1), and
   the `#elif` arm's declaration is the `<unistd.h>` mirror's (`include/unistd.in.rs`), which this
   unit already includes, so the arm is recorded rather than repeated.  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"), any (feature = "_sc_pagesize", not (feature = "have_getpagesize"))))]
include!("include/getpagesize.rs");             // C 1641: `# include "getpagesize.h"`

/// C 1646: `static size_t pagesize;`
/// C: static size_t pagesize;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
static mut pagesize: size_t = 0;

/// C 1648-1649: `valloc`.
/// C: void * valloc (size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn valloc (size: size_t) -> *mut c_void
{
  if pagesize == 0
    { pagesize = getpagesize () as size_t; }

  return aligned_alloc (pagesize, size);
}

/* C 1657-1661: `#undef malloc` / `#undef realloc` / `#undef calloc` / `#undef aligned_alloc` /
   `#undef free` - the boundary of C 61-66's rename chain.  The five definitions below are the
   real names; C 1663-1691's bodies reach the `g*` definitions above.  */

/// C 1663-1664: the `malloc` C 1657's `#undef` leaves free - the item carries C's text name in its
/// `export_name` (the port's `rpl_*` item name is the gnulib spelling for this replacement).
/// C: void * malloc (size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "malloc")]
pub unsafe extern "C" fn rpl_malloc (size: size_t) -> *mut c_void
{
  return gmalloc! (size);
}

/// C 1669-1670: the `calloc` C 1659's `#undef` leaves free.
/// C: void * calloc (size_t nmemb, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "calloc")]
pub unsafe extern "C" fn rpl_calloc (nmemb: size_t, size: size_t) -> *mut c_void
{
  return gcalloc! (nmemb, size);
}

/// C 1675-1676: the `free` C 1661's `#undef` leaves free.
/// C: void free (void *ptr);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "free")]
pub unsafe extern "C" fn rpl_free (ptr: *mut c_void)
{
  gfree! (ptr);
}

/// C 1681-1682: the `aligned_alloc` C 1660's `#undef` leaves free.
/// C: void * aligned_alloc (size_t alignment, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "aligned_alloc")]
pub unsafe extern "C" fn rpl_aligned_alloc (alignment: size_t, size: size_t) -> *mut c_void
{
  return galigned_alloc! (alignment, size);
}

/// C 1687-1688: the `realloc` C 1658's `#undef` leaves free.
/// C: void * realloc (void *ptr, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc")))]
#[unsafe(export_name = "realloc")]
pub unsafe extern "C" fn rpl_realloc (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  return grealloc! (ptr, size);
}

/* C 1693-1714: `#ifdef GC_MCHECK` and the section's opening block; C 1716's
   `#include <stdio.h>` follows.  */
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"), feature = "gc_mcheck"))]
include!("../lib/include/stdio.rs");            // C 1716: `#include <stdio.h>`

/* Old hook values.  */
/// C: static void (*old_free_hook) (void *ptr);
/// C: static void *(*old_malloc_hook) (size_t size);
/// C: static void *(*old_realloc_hook) (void *ptr, size_t size);
/// C: static void (*abortfunc) (enum mcheck_status);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
static mut old_free_hook: Option<unsafe extern "C" fn (*mut c_void)> = None;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
static mut old_malloc_hook: Option<unsafe extern "C" fn (size_t) -> *mut c_void> = None;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
static mut old_realloc_hook: Option<unsafe extern "C" fn (*mut c_void, size_t) -> *mut c_void> = None;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
static mut abortfunc: Option<unsafe extern "C" fn (mcheck_status)> = None;

/* Arbitrary magical numbers.  */
/// C: #define MAGICWORD	(SIZE_MAX / 11 ^ SIZE_MAX / 13 << 3)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
macro_rules! MAGICWORD { () => { (SIZE_MAX / 11 ^ SIZE_MAX / 13 << 3) }; }
/// C: #define MAGICFREE	(SIZE_MAX / 17 ^ SIZE_MAX / 19 << 4)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
macro_rules! MAGICFREE { () => { (SIZE_MAX / 17 ^ SIZE_MAX / 19 << 4) }; }
/// C: #define MAGICBYTE	((char) 0xd7)
/// C: #define MALLOCFLOOD	((char) 0x93)
/// C: #define FREEFLOOD	((char) 0x95)
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
macro_rules! MAGICBYTE { () => { 0xd7 as c_char }; }
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
macro_rules! MALLOCFLOOD { () => { 0x93 as c_char }; }
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
macro_rules! FREEFLOOD { () => { 0x95 as c_char }; }

/// C 1733-1737: `struct hdr`.
/// C: struct hdr { size_t size; size_t magic; };
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
#[derive (Clone, Copy)]
#[repr (C)]
pub struct hdr
{
  pub size: size_t,
  pub magic: size_t,
}
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
const _: () = assert! (core::mem::size_of::<hdr> () == 16);        // clang: sizeof 16, align 8
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
const _: () = assert! (core::mem::offset_of! (hdr, magic) == 8);   // clang: size 0, magic 8

/// C 1739-1740: `checkhdr`.
/// C: static enum mcheck_status checkhdr (const struct hdr *hdr);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
unsafe fn checkhdr (hdr: *const hdr) -> mcheck_status
{
  let mut status: mcheck_status = mcheck_status::MCHECK_OK;
  match (*hdr).magic
    {
    _ if (*hdr).magic == MAGICFREE! () => { status = mcheck_status::MCHECK_FREE; }
    _ if (*hdr).magic == MAGICWORD! () =>
      {
	if *((hdr as *const hdr).add (1) as *const c_char).add ((*hdr).size) != MAGICBYTE! ()
	  { status = mcheck_status::MCHECK_TAIL; }
	else
	  { status = mcheck_status::MCHECK_OK; }
      }
    _ => { status = mcheck_status::MCHECK_HEAD; }
    }
  if status != mcheck_status::MCHECK_OK
    { (abortfunc.unwrap ()) (status); }
  return status;
}

/// C 1763-1764: `freehook` (its address is stored in `gfree_hook`/`old_free_hook`).
/// C: static void freehook (void *ptr);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
unsafe extern "C" fn freehook (ptr: *mut c_void)
{
  let mut ptr: *mut c_void = ptr;
  let mut hdr_: *mut hdr = core::ptr::null_mut ();

  if !ptr.is_null ()
    {
      let mut l: *mut alignlist = core::ptr::null_mut ();

      /* If the block was allocated by aligned_alloc, its real pointer
	 to free is recorded in _aligned_blocks; find that.  */
      PROTECT_MALLOC_STATE! (0);
      LOCK_ALIGNED_BLOCKS! ();
      l = _aligned_blocks;
      while !l.is_null ()
	{
	  if (*l).aligned == ptr
	    {
	      (*l).aligned = core::ptr::null_mut ();	/* Mark the slot in the list as free.  */
	      ptr = (*l).exact;
	      break;
	    }
	  l = (*l).next;
	}
      UNLOCK_ALIGNED_BLOCKS! ();
      PROTECT_MALLOC_STATE! (1);

      hdr_ = (ptr as *mut hdr).offset (- 1);
      checkhdr (hdr_);
      (*hdr_).magic = MAGICFREE! ();
      memset (ptr, FREEFLOOD! () as c_int, (*hdr_).size);
    }
  else
    { hdr_ = core::ptr::null_mut (); }

  gfree_hook = old_free_hook;
  /* C 1657's `#undef free` is in effect here, so C 1795's `free` is the plain symbol the same
     file defines at C 1675 - the port's `rpl_free`.  */
  rpl_free (hdr_ as *mut c_void);
  gfree_hook = Some (freehook);
}

/// C 1799-1800: `mallochook` (its address is stored in `gmalloc_hook`/`old_malloc_hook`).
/// C: static void * mallochook (size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
unsafe extern "C" fn mallochook (size: size_t) -> *mut c_void
{
  let mut hdr_: *mut hdr = core::ptr::null_mut ();

  gmalloc_hook = old_malloc_hook;
  hdr_ = rpl_malloc (core::mem::size_of::<hdr> () + size + 1) as *mut hdr;   // C 1805 (after the `#undef`s)
  gmalloc_hook = Some (mallochook);
  if hdr_.is_null ()
    { return core::ptr::null_mut (); }

  (*hdr_).size = size;
  (*hdr_).magic = MAGICWORD! ();
  *((hdr_ as *mut hdr).add (1) as *mut c_char).add (size) = MAGICBYTE! ();
  return memset ((hdr_ as *mut hdr).add (1) as *mut c_void, MALLOCFLOOD! () as c_int, size);
}

/// C 1816-1817: `reallochook` (its address is stored in `grealloc_hook`/`old_realloc_hook`).
/// C: static void * reallochook (void *ptr, size_t size);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
unsafe extern "C" fn reallochook (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  let mut hdr_: *mut hdr = core::ptr::null_mut ();
  let mut osize: size_t = 0;

  if !ptr.is_null ()
    {
      hdr_ = (ptr as *mut hdr).offset (- 1);
      osize = (*hdr_).size;

      checkhdr (hdr_);
      if size < osize
	{ memset ((ptr as *mut c_char).add (size) as *mut c_void, FREEFLOOD! () as c_int,
		  osize - size); }
    }

  gfree_hook = old_free_hook;
  gmalloc_hook = old_malloc_hook;
  grealloc_hook = old_realloc_hook;
  hdr_ = rpl_realloc (hdr_ as *mut c_void, core::mem::size_of::<hdr> () + size + 1) as *mut hdr;   // C 1835
  gfree_hook = Some (freehook);
  gmalloc_hook = Some (mallochook);
  grealloc_hook = Some (reallochook);
  if hdr_.is_null ()
    { return core::ptr::null_mut (); }

  (*hdr_).size = size;
  (*hdr_).magic = MAGICWORD! ();
  *((hdr_ as *mut hdr).add (1) as *mut c_char).add (size) = MAGICBYTE! ();
  if size > osize
    { memset (((hdr_ as *mut hdr).add (1) as *mut c_char).add (osize) as *mut c_void,
	      MALLOCFLOOD! () as c_int, size - osize); }
  return (hdr_ as *mut hdr).add (1) as *mut c_void;
}

/// C 1850-1851: `mabort`.
/// C: static void mabort (enum mcheck_status status);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
unsafe extern "C" fn mabort (status: mcheck_status)
{
  let mut msg: *const c_char = core::ptr::null ();
  match status
    {
    mcheck_status::MCHECK_OK =>
      { msg = c"memory is consistent, library is buggy".as_ptr (); }
    mcheck_status::MCHECK_HEAD =>
      { msg = c"memory clobbered before allocated block".as_ptr (); }
    mcheck_status::MCHECK_TAIL =>
      { msg = c"memory clobbered past end of allocated block".as_ptr (); }
    mcheck_status::MCHECK_FREE =>
      { msg = c"block freed twice".as_ptr (); }
    _ =>
      { msg = c"bogus mcheck_status, library is buggy".as_ptr (); }
    }
  /* C 1872-1877's `#ifdef __GNU_LIBRARY__` arms.  `__libc_fatal` is glibc's and is declared at
     its use site (P-88 (1), the `femacs/lib/malloc/dynarray_at_failure.rs` 36-39 shape).  */
  #[cfg (feature = "__gnu_library__")]
  {
    /// C: stdio.h: `_Noreturn void __libc_fatal (const char *message);`
    unsafe extern "C" { pub fn __libc_fatal (message: *const c_char) -> !; }
    __libc_fatal (msg);
  }
  #[cfg (not (feature = "__gnu_library__"))]
  {
    fprintf (stderr, c"mcheck: %s\n".as_ptr (), msg);
    emacs_abort ();
  }
}

/// C 1880: `static int mcheck_used = 0;`
/// C: static int mcheck_used = 0;
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
static mut mcheck_used: c_int = 0;

/// C 1882-1883: `mcheck`.
/// C: int mcheck (void (*func) (enum mcheck_status));
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mcheck (func: Option<unsafe extern "C" fn (mcheck_status)>) -> c_int
{
  abortfunc = if func.is_some () { func } else { Some (mabort) };

  /* These hooks may not be safely inserted if malloc is already in use.  */
  if __malloc_initialized == 0 && mcheck_used == 0
    {
      old_free_hook = gfree_hook;
      gfree_hook = Some (freehook);
      old_malloc_hook = gmalloc_hook;
      gmalloc_hook = Some (mallochook);
      old_realloc_hook = grealloc_hook;
      grealloc_hook = Some (reallochook);
      mcheck_used = 1;
    }

  return if mcheck_used != 0 { 0 } else { -1 };
}

/// C 1902-1903: `mprobe`.
/// C: enum mcheck_status mprobe (void *ptr);
#[cfg (all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"),
            feature = "gc_mcheck"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mprobe (ptr: *mut c_void) -> mcheck_status
{
  return if mcheck_used != 0 { checkhdr (ptr as *const hdr) }
         else { mcheck_status::MCHECK_DISABLED };
}
