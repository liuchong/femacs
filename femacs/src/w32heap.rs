// femacs: port of src/w32heap.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32heap.o`
// sits in `W32_OBJ` (`emacs/src/Makefile.in` 482, `W32_OBJ=@W32_OBJ@`, filled by the non-Cygwin
// MinGW arm of `configure.ac` 3051's `HAVE_W32` split - the build whose `nt/inc/ms-w32.h` 25-27
// `#define`s `WINDOWSNT`), and `docs/baseline/frozen-build-sources.txt` lists no `w32heap.o` for
// this target, so `extract-abi.py`'s checklist for the file is empty (P-86 ②) and the ledger
// `docs/ports/w32heap.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "windowsnt"`, on every item *and on the include graph*,
//     which is why the frozen configuration compiles nothing here.  It is `WINDOWSNT`, not
//     `HAVE_NTGUI`: `w32heap.o` is in `W32_OBJ` on both arms of the Cygwin/non-Cygwin split, and
//     the file's only dependence on the GUI half is `DebPrint`, which it reaches through
//     `config.h`'s `conf_post.h` 160-166 (the arm flip therefore sets both, the `src/w32proc.rs`
//     reading);
//   * the file's own `#if`s are the two `#ifndef MINGW_W64` (C 93 and C 119), carried as
//     `not (feature = "mingw_w64")` (the `src/w32.rs` 636 spelling).  `tools/check-cond-eval.py
//     src/w32heap.c` reads `2 conditions, 0 unknown, 0 disagreements [VERIFIED]` - those two and
//     nothing else - so no other arm is conditional;
//   * the `<windows.h>` section below stands where the header enters this translation unit: C 45
//     `#include "w32common.h"` is the file's first reach of it (`w32common.h` 24
//     `#include <windows.h>`, which the port's `include/w32common.rs` spells as its one system
//     header).  The reference tree has no file for `<windows.h>` (P-88 ①: the declaration site is
//     C's own `#include` line, and no file is added under `femacs/src/include/`), so the names this
//     file uses are declared there, each with the SDK's own text above it.  The spellings repeat
//     `src/w32proc.rs`'s and `src/w32.rs`'s sections (`DWORD` is the 32-bit Windows unsigned,
//     `HANDLE`/`HMODULE`/`LPVOID`/`PVOID` are `void *`, `BOOL` is C's `int`, `SIZE_T` is `usize`),
//     so that a `windowsnt` translation unit sees one set of names.  `DWORD_PTR` and the
//     `ROUND_UP`/`get_page_size`/`get_allocation_unit` macros belong to `include/w32common.rs` and
//     are not repeated here.
//
// **Acquisition note - this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no
// mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
// Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h` anywhere), so there is no
// `<windows.h>` to compile or measure against: **every record below is `UNVERIFIED`** (its widths
// are the documented Windows ones, not measurements) and the layout assertions are owed.  What the
// round does show is that the `windowsnt` arm type-checks under this machine's `rustc` - the
// ledger's `## Evidence` carries that reading and names the errors that belong to the include
// graph rather than to this unit.
//
// `docs/ports/w32heap.md` carries the C line ranges, the acceptance readings, every deviation (the
// `<windows.h>` stand-in section, the spelled implicit conversions, the two `match`es that stand
// for C's `switch`es, the `FREEABLE_P` macro, the `HexPrint`-shaped `%I64x` casts) and the items the
// integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, unused_mut,
         unused_assignments, unused_parens, unused_comparisons, static_mut_refs,
         unsafe_op_in_unsafe_fn, clippy::all)]

/* C 1-17 is the file's declaration block, dropped (PORTING.md 0.1-19).  C 19-37 is its two
   attribution lines and its design comment, carried as C's own text at its C position.  */

/*
  Geoff Voelker (voelker@cs.washington.edu)                          7-29-94
*/

/*
  Heavily modified by Fabrice Popineau (fabrice.popineau@gmail.com) 28-02-2014
*/

/*
  Memory allocation scheme for w32/w64:

  - Buffers are mmap'ed using a very simple emulation of mmap/munmap
  - During the emacs phase, or always if pdumper is used:
    * we create a private heap for new memory blocks
    * we make sure that we never free a block that has been dumped.
      Freeing a dumped block could work in principle, but may prove
      unreliable if we distribute binaries of emacs.exe: MS does not
      guarantee that the heap data structures are the same across all
      versions of their OS, even though the API is available since XP.  */

include!("include/config.rs");                  // C 39: `#include <config.h>`
include!("include/conf_post.rs");               // C 39's AH_BOTTOM `#include <conf_post.h>` (DebPrint, C 131)

/* C 39's `config.h` -> `conf_post.h` 33-38 chain reaches `<ms-w32.h>`: `malloc_fn`/`realloc_fn`/
   `free_fn` (C 61-63) and the `*_after_dump` prototypes come from there, and that mirror's
   declarations sit in its glob-imported `ms_w32_decls` module, so this file's own definitions of
   those names shadow them instead of colliding with them (PORTING.md P-39's shape).  */
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/ms-w32.rs");        // C 39 -> `conf_post.h` 38: `#include <ms-w32.h>`

/* C 40-48: the include graph.  Every header sits inside the file gate, as C's own build has it (no
   translation unit the frozen configuration builds reaches this file; P-86 ⑧).  Where the reference
   tree carries *two* headers of one name, the one `nt/inc` overrides is the one included - that is
   what `<sys/mman.h>` and `<sys/resource.h>` resolve to in the MinGW build.  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdio.rs");            // C 40: `#include <stdio.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/errno.rs");            // C 41: `#include <errno.h>` (`ENOMEM`, `EPERM`, `ENOSYS`, `EINVAL`)
#[cfg (feature = "windowsnt")]
include!("../nt/inc/sys/include/mman.rs");      // C 43: `#include <sys/mman.h>`
#[cfg (feature = "windowsnt")]
include!("../nt/inc/sys/include/resource.rs");  // C 44: `#include <sys/resource.h>` (`rlimit`, `rlimit_resource_t`, `RLIMIT_*`)
#[cfg (feature = "windowsnt")]
include!("include/w32common.rs");               // C 45: `#include "w32common.h"`

/* ----------------------------------------------------------------------------------------------
   C 45's `#include "w32common.h"` reaches `<windows.h>` (that header's line 24).  No file in the
   reference tree carries the SDK header (P-88 ①: the declaration site is C's own `#include` line,
   and no file is added under `femacs/src/include/`).  What follows is the names *this file* uses,
   each with the SDK's own text above it.

   **No measurement was possible**: this machine has no `<windows.h>` and no mingw toolchain (see
   the acquisition note), so every scalar is spelled from PORTING.md 1.1's fixed widths (`DWORD` and
   `BOOL` are the Windows ones whatever this target's `c_long`/`int` widths are, and the handle
   typedefs are `void *`), and `MEMORY_BASIC_INFORMATION` is **UNVERIFIED** - its layout assertion
   is owed.
   ---------------------------------------------------------------------------------------------- */

/// C 45's `<windows.h>`: the scalar types and the handle typedefs this file's declarations use.
/// C: typedef unsigned long DWORD; typedef unsigned long long ULONG_PTR; typedef ULONG_PTR SIZE_T;
/// C: typedef int BOOL; typedef void *HANDLE, *HMODULE, *HINSTANCE, *LPVOID, *PVOID;
/// C: typedef const void *LPCVOID; typedef const char *LPCSTR;
#[cfg (feature = "windowsnt")]
pub type DWORD = u32;
#[cfg (feature = "windowsnt")]
pub type SIZE_T = usize;
#[cfg (feature = "windowsnt")]
pub type BOOL = c_int;
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HMODULE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type PVOID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPCVOID = *const c_void;
#[cfg (feature = "windowsnt")]
pub type LPCSTR = *const c_char;

/// C 45's `<windows.h>`: `#define WINAPI __stdcall` - the ABI Rust spells `extern "system"`, which
/// the entry-point typedef C 98 and the `HeapSetInformation` declaration below use.
/// C: #define WINAPI __stdcall
#[cfg (feature = "windowsnt")]
pub const WINAPI: bool = true;

/// C 45's `<windows.h>`: the allocation, mapping and error entry points this file calls, in C's
/// own parameter order and with C's own names (`HeapReAlloc`'s `dwFlags` is 0 at both call sites;
/// `VirtualQuery`'s buffer parameter is `PMEMORY_BASIC_INFORMATION`, the record declared below).
/// C: WINBASEAPI HANDLE WINAPI HeapCreate (DWORD flOptions, SIZE_T dwInitialSize, SIZE_T dwMaximumSize);
/// C: WINBASEAPI LPVOID WINAPI HeapAlloc (HANDLE hHeap, DWORD dwFlags, SIZE_T dwBytes);
/// C: WINBASEAPI LPVOID WINAPI HeapReAlloc (HANDLE hHeap, DWORD dwFlags, LPVOID lpMem, SIZE_T dwBytes);
/// C: WINBASEAPI BOOL WINAPI HeapFree (HANDLE hHeap, DWORD dwFlags, LPVOID lpMem);
/// C: WINBASEAPI BOOL WINAPI HeapSetInformation (HANDLE, HEAP_INFORMATION_CLASS, PVOID, SIZE_T);
/// C: LPVOID WINAPI VirtualAlloc (LPVOID lpAddress, SIZE_T dwSize, DWORD flAllocationType, DWORD flProtect);
/// C: WINBASEAPI BOOL WINAPI VirtualFree (LPVOID lpAddress, SIZE_T dwSize, DWORD dwFreeType);
/// C: SIZE_T WINAPI VirtualQuery (LPCVOID lpAddress, PMEMORY_BASIC_INFORMATION lpBuffer, SIZE_T dwLength);
/// C: WINBASEAPI DWORD WINAPI GetLastError (void);
/// C: WINBASEAPI HMODULE WINAPI LoadLibraryA (LPCSTR lpLibFileName);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn HeapCreate (flOptions: DWORD, dwInitialSize: SIZE_T, dwMaximumSize: SIZE_T) -> HANDLE;
  pub fn HeapAlloc (hHeap: HANDLE, dwFlags: DWORD, dwBytes: SIZE_T) -> LPVOID;
  pub fn HeapReAlloc (hHeap: HANDLE, dwFlags: DWORD, lpMem: LPVOID, dwBytes: SIZE_T) -> LPVOID;
  pub fn HeapFree (hHeap: HANDLE, dwFlags: DWORD, lpMem: LPVOID) -> BOOL;
  /// C 119-134's arm is the one caller: `HeapSetInformation`'s second parameter is the file's own
  /// `HEAP_INFORMATION_CLASS` (C 94-96), so this declaration carries the same gate that enum does.
  #[cfg (not (feature = "mingw_w64"))]
  pub fn HeapSetInformation (HeapHandle: HANDLE, HeapInformationClass: HEAP_INFORMATION_CLASS,
			     HeapInformation: PVOID, HeapInformationLength: SIZE_T) -> BOOL;
  pub fn VirtualAlloc (lpAddress: LPVOID, dwSize: SIZE_T, flAllocationType: DWORD,
		       flProtect: DWORD) -> LPVOID;
  pub fn VirtualFree (lpAddress: LPVOID, dwSize: SIZE_T, dwFreeType: DWORD) -> BOOL;
  pub fn VirtualQuery (lpAddress: LPCVOID, lpBuffer: *mut MEMORY_BASIC_INFORMATION,
		       dwLength: SIZE_T) -> SIZE_T;
  pub fn GetLastError () -> DWORD;
  /// C 122 calls the `…A` alias by its generic name (`LoadLibrary`), the `src/w32.rs` 404-409
  /// reading; the declaration is the SDK's `LoadLibraryA`.
  pub fn LoadLibrary (lpLibFileName: LPCSTR) -> HMODULE;
}

/// C 45's `<windows.h>`: `#define CopyMemory RtlCopyMemory` and `#define MoveMemory RtlMoveMemory`
/// (winnt.h's renames; MinGW-w64's own header then renames both on to the CRT entries the rename
/// chain ends at).  The port keeps the C *source* names this file's text uses and spells the
/// renames as `macro_rules!`s over the CRT entries (the `nt/inc/include/ms-w32.rs` 275-278 shape);
/// declaring the `memcpy`/`memmove` symbols directly is refused by the compiler's own runtime-symbol
/// check (measured).  **UNVERIFIED**: the chain's end is only as good as the MinGW header, and no
/// such header is on this machine.
/// C: #define CopyMemory RtlCopyMemory
/// C: #define MoveMemory RtlMoveMemory
/// C: VOID RtlCopyMemory (PVOID Destination, const VOID *Source, SIZE_T Length);
/// C: VOID RtlMoveMemory (PVOID Destination, const VOID *Source, SIZE_T Length);
#[cfg (feature = "windowsnt")]
macro_rules! CopyMemory { ($dest:expr, $src:expr, $len:expr) => { memcpy ($dest, $src, $len) }; }
#[cfg (feature = "windowsnt")]
macro_rules! MoveMemory { ($dest:expr, $src:expr, $len:expr) => { memmove ($dest, $src, $len) }; }

/// C 45's `<windows.h>`: the two allocation types and the one error code this file compares
/// against.  `ERROR_NOT_ENOUGH_MEMORY` is the documented 8L; the SDK header that states it is not
/// on this machine (acquisition note), so the row is `UNVERIFIED` like the records.
/// C: #define MEM_COMMIT 0x1000
/// C: #define MEM_RESERVE 0x2000
/// C: #define MEM_DECOMMIT 0x4000
/// C: #define MEM_RELEASE 0x8000
/// C: #define PAGE_READWRITE 0x04
/// C: #define ERROR_NOT_ENOUGH_MEMORY 8L
#[cfg (feature = "windowsnt")]
pub const MEM_COMMIT: DWORD = 0x1000;
#[cfg (feature = "windowsnt")]
pub const MEM_RESERVE: DWORD = 0x2000;
#[cfg (feature = "windowsnt")]
pub const MEM_DECOMMIT: DWORD = 0x4000;
#[cfg (feature = "windowsnt")]
pub const MEM_RELEASE: DWORD = 0x8000;
#[cfg (feature = "windowsnt")]
pub const PAGE_READWRITE: DWORD = 0x04;
#[cfg (feature = "windowsnt")]
pub const ERROR_NOT_ENOUGH_MEMORY: DWORD = 8;

/// C 45's `<windows.h>`: `typedef struct _MEMORY_BASIC_INFORMATION { … } MEMORY_BASIC_INFORMATION;`
/// - the record `VirtualQuery` fills and this file reads five members of.  **All seven members are
/// carried** (not just the five C's text names) because the two `VirtualQuery` calls hand it
/// `sizeof` bytes of buffer: a shortened record would move every field the file reads.  **The
/// layout is UNVERIFIED** - the widths are the documented ones (pointer, pointer, DWORD, SIZE_T,
/// DWORD, DWORD, DWORD) and the offsets a 64-bit compiler gives them, and no assertion is written
/// here; the ledger's owed list names the measurement.
/// C: typedef struct _MEMORY_BASIC_INFORMATION {
/// C:   PVOID  BaseAddress;
/// C:   PVOID  AllocationBase;
/// C:   DWORD  AllocationProtect;
/// C:   SIZE_T RegionSize;
/// C:   DWORD  State;
/// C:   DWORD  Protect;
/// C:   DWORD  Type;
/// C: } MEMORY_BASIC_INFORMATION;
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct MEMORY_BASIC_INFORMATION
{
  pub BaseAddress: PVOID,
  pub AllocationBase: PVOID,
  pub AllocationProtect: DWORD,
  pub RegionSize: SIZE_T,
  pub State: DWORD,
  pub Protect: DWORD,
  pub Type: DWORD,
}

#[cfg (feature = "windowsnt")]
include!("include/w32heap.rs");                 // C 46: `#include "w32heap.h"`
#[cfg (feature = "windowsnt")]
include!("include/lisp.rs");                    // C 47: `#include "lisp.h"` (`eassert`, `PTRDIFF_MAX`, `memset`'s header)
/* C 47's `lisp.h` reaches `<string.h>` at its line 30, which is where `memset` (C 290, C 392,
   C 399) is declared; the mirror that carries it is named here (P-72 clause 4).  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/string.rs");           // C 47's `lisp.h`: `#include <string.h>` (`memset`)
#[cfg (feature = "windowsnt")]
include!("include/w32.rs");                     // C 48: `#include "w32.h"`	/* for FD_SETSIZE */

/* Info for keeping track of our dynamic heap used after dumping. */
#[cfg (feature = "windowsnt")]
static mut data_region_base: *mut u8 = core::ptr::null_mut ();
#[cfg (feature = "windowsnt")]
static mut data_region_end: *mut u8 = core::ptr::null_mut ();

/* Handle for the private heap:
    - inside the dumped_data[] array before dump with unexec,
    - outside of it after dump, or always if pdumper is used.
*/
#[cfg (feature = "windowsnt")]
static mut heap: HANDLE = core::ptr::null_mut ();

/* We redirect the standard allocation functions.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut the_malloc_fn: malloc_fn = None;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut the_realloc_fn: realloc_fn = None;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut the_free_fn: free_fn = None;

/// C 65-72: `static void *heap_alloc (size_t size)` - `size | !size` is C's `|` of the size with
/// the *logical* not of the size (`!`, not `~`), i.e. 1 exactly when the size is 0; the port
/// spells that operand `(size == 0) as size_t` and records it in the ledger.  C's comparison
/// `size <= PTRDIFF_MAX` needs the widening the port spells as `PTRDIFF_MAX as size_t`.
#[cfg (feature = "windowsnt")]
unsafe fn heap_alloc (size: size_t) -> *mut c_void
{
  let mut p: *mut c_void = if size <= PTRDIFF_MAX as size_t
    { HeapAlloc (heap, 0, size | (size == 0) as size_t) }
    else { core::ptr::null_mut () };
  if p.is_null ()
    { *errno () = ENOMEM; }
  return p;
}

/// C 74-83: `static void *heap_realloc (void *ptr, size_t size)`.  The same two spellings as
/// `heap_alloc` above.
#[cfg (feature = "windowsnt")]
unsafe fn heap_realloc (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  let mut p: *mut c_void = if size <= PTRDIFF_MAX as size_t
    { HeapReAlloc (heap, 0, ptr, size | (size == 0) as size_t) }
    else { core::ptr::null_mut () };
  if p.is_null ()
    { *errno () = ENOMEM; }
  return p;
}

/* C 85-87: "It doesn't seem to be useful to allocate from a file mapping.  It would be if the
   memory was shared.  <URL>" - C's comment, carried at its position.  */

/* Heap creation.  */

/* We want to turn on Low Fragmentation Heap for XP and older systems.
   MinGW32 lacks those definitions.  */
#[cfg (all (feature = "windowsnt", not (feature = "mingw_w64")))]
pub type HEAP_INFORMATION_CLASS = c_int;

/// C 94-96: `typedef enum _HEAP_INFORMATION_CLASS { HeapCompatibilityInformation } HEAP_INFORMATION_CLASS;`
/// - PORTING.md 1.3's shape for a C enum (the C type plus one constant per member, the
/// `nt/inc/sys/include/resource.rs` spelling), which keeps C 128's bare member name intact.
/// C: typedef enum _HEAP_INFORMATION_CLASS {
/// C:   HeapCompatibilityInformation
/// C: } HEAP_INFORMATION_CLASS;
#[cfg (all (feature = "windowsnt", not (feature = "mingw_w64")))]
pub const HeapCompatibilityInformation: HEAP_INFORMATION_CLASS = 0;

/// C 98: `typedef WINBASEAPI BOOL (WINAPI * HeapSetInformation_Proc)(HANDLE,HEAP_INFORMATION_CLASS,PVOID,SIZE_T);`
/// - the entry-point type C 123 casts `get_proc_addr`'s result to.  `WINBASEAPI`/`WINAPI` are the
/// SDK's declaration and calling-convention markers; the ABI is `extern "system"` (the
/// `include/w32common.rs` `VOIDFNPTR` row spells the `WINAPI`-less one).
/// C: typedef WINBASEAPI BOOL (WINAPI * HeapSetInformation_Proc)(HANDLE,HEAP_INFORMATION_CLASS,PVOID,SIZE_T);
#[cfg (all (feature = "windowsnt", not (feature = "mingw_w64")))]
pub type HeapSetInformation_Proc
  = Option<unsafe extern "system" fn (HANDLE, HEAP_INFORMATION_CLASS, PVOID, SIZE_T) -> BOOL>;

/// C 101-151: `init_heap` - the private heap's creation, the pre-Vista low-fragmentation-heap
/// request (C 119-134, the file's `#ifndef MINGW_W64` arm) and the allocator switch.
/// C: void init_heap (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_heap ()
{
  /* After dumping, use a new private heap.  We explicitly enable
     the low fragmentation heap (LFH) here, for the sake of pre
     Vista versions.  Note: this will harmlessly fail on Vista and
     later, where the low-fragmentation heap is enabled by
     default.  It will also fail on pre-Vista versions when Emacs
     is run under a debugger; set _NO_DEBUG_HEAP=1 in the
     environment before starting GDB to get low fragmentation heap
     on XP and older systems, for the price of losing "certain
     heap debug options"; for the details see
     https://msdn.microsoft.com/en-us/library/windows/desktop/aa366705%28v=vs.85%29.aspx.  */
  data_region_end = data_region_base;

  /* Create the private heap.  */
  heap = HeapCreate (0, 0, 0);

  /* C 119-134: `#ifndef MINGW_W64` - on the MinGW-w64 SDK this arm is C's dead text (its
     `_mingw.h` is what defines `MINGW_W64`), so the port carries it under the feature that names
     that side (PORTING.md P-51).  C 123's `(HeapSetInformation_Proc)` cast is not a Rust `as`
     cast pair, so the port spells the pointer operation the `DEF_DLL_FN` expansions use
     (`src/w32proc.rs` 620-625's reading).  */
  #[cfg (not (feature = "mingw_w64"))]
  {
    let mut enable_lfh: c_ulong = 2;
    /* Set the low-fragmentation heap for OS before Vista.  */
    let hm_kernel32dll: HMODULE = LoadLibrary (c"kernel32.dll".as_ptr ());
    let s_pfn_Heap_Set_Information: HeapSetInformation_Proc =
      core::mem::transmute::<VOIDFNPTR, HeapSetInformation_Proc>
	(get_proc_addr (hm_kernel32dll, c"HeapSetInformation".as_ptr ()));
    if s_pfn_Heap_Set_Information.is_some ()
      {
	if s_pfn_Heap_Set_Information.unwrap () (heap as PVOID,
						HeapCompatibilityInformation,
						(&raw mut enable_lfh) as PVOID,
						core::mem::size_of_val (&enable_lfh)) == 0
	  { DebPrint! (("Enabling Low Fragmentation Heap failed: error %ld\n",
			GetLastError ())); }
      }
  }

  if os_subtype == OS_SUBTYPE_9X
    {
      the_malloc_fn = Some (malloc_after_dump_9x);
      the_realloc_fn = Some (realloc_after_dump_9x);
      the_free_fn = Some (free_after_dump_9x);
    }
  else
    {
      the_malloc_fn = Some (malloc_after_dump);
      the_realloc_fn = Some (realloc_after_dump);
      the_free_fn = Some (free_after_dump);
    }

  /* Update system version information to match current system.  */
  cache_system_info ();
}

/* C 153's form feed and C 154's banner, carried (the form feed itself is not text).  */

/* malloc, realloc, free.  */

/* C 156-158: `#undef malloc` / `#undef realloc` / `#undef free` - the three cancels of the
   `nt/inc/ms-w32.h` 443-445 renames.  The mirror spells those renames as `macro_rules!`s, which
   fire on the `name! (…)` form only, so the port's plain calls and its definitions above reach the
   entries C's `#undef` selects by construction.  */

/* FREEABLE_P checks if the block can be safely freed.  */
/// C 161-162: `#define FREEABLE_P(addr) ((DWORD_PTR)(unsigned char *)(addr) > 0)` - the body's
/// cast is the port's `as *mut u8 as DWORD_PTR` and the comparison is C's own.
/// C: #define FREEABLE_P(addr)						\
/// C:   ((DWORD_PTR)(unsigned char *)(addr) > 0)
#[cfg (feature = "windowsnt")]
macro_rules! FREEABLE_P { ($addr:expr) => { ($addr) as *mut u8 as DWORD_PTR > 0 }; }

/// C 164-182: `malloc_after_dump`.
/// C: void * malloc_after_dump (size_t size);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc_after_dump (size: size_t) -> *mut c_void
{
  /* Use the new private heap.  */
  let mut p: *mut c_void = heap_alloc (size);

  /* After dump, keep track of the "brk value" for sbrk(0).  */
  if !p.is_null ()
    {
      let new_brk: *mut u8 = (p as *mut u8).add (size);

      if new_brk > data_region_end
	{ data_region_end = new_brk; }
    }
  return p;
}

/// C 184-211: `realloc_after_dump`.  C 199's `if (p && ptr)` is the port's two null tests, in
/// C's order.
/// C: void * realloc_after_dump (void *ptr, size_t size);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc_after_dump (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  let mut p: *mut c_void;

  /* After dumping.  */
  if FREEABLE_P! (ptr)
    {
      /* Reallocate the block since it lies in the new heap.  */
      p = heap_realloc (ptr, size);
    }
  else
    {
      /* If the block lies in the dumped data, do not free it.  Only
         allocate a new one.  */
      p = heap_alloc (size);
      if !p.is_null () && !ptr.is_null ()
	{ CopyMemory! (p, ptr, size); }
    }
  /* After dump, keep track of the "brk value" for sbrk(0).  */
  if !p.is_null ()
    {
      let new_brk: *mut u8 = (p as *mut u8).add (size);

      if new_brk > data_region_end
	{ data_region_end = new_brk; }
    }
  return p;
}

/// C 213-223: `free_after_dump`.
/// C: void free_after_dump (void *ptr);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_after_dump (ptr: *mut c_void)
{
  /* After dumping.  */
  if FREEABLE_P! (ptr)
    {
      /* Free the block if it is in the new private heap.  */
      HeapFree (heap, 0, ptr);
    }
}

/* On Windows 9X, HeapAlloc may return pointers that are not aligned
   on 8-byte boundary, alignment which is required by the Lisp memory
   management.  To circumvent this problem, manually enforce alignment
   on Windows 9X.  */

/// C 230-240: `malloc_after_dump_9x`.  C 237's `((intptr_t)p + 8) & ~7` is the port's
/// `(p as intptr_t + 8) & !7`, and C 238's `*((void**)pa-1) = p` the pointer step below it.
/// C: void * malloc_after_dump_9x (size_t size);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc_after_dump_9x (size: size_t) -> *mut c_void
{
  let mut p: *mut c_void = malloc_after_dump (size + 8);
  let mut pa: *mut c_void;
  if p.is_null ()
    { return p; }
  pa = ((p as intptr_t + 8) & !7) as *mut c_void;
  *((pa as *mut *mut c_void).sub (1)) = p;
  return pa;
}

/// C 242-273: `realloc_after_dump_9x`.  C 255's pointer difference is the port's `offset_from`
/// pair (both differences are of `char *`s, i.e. bytes), and C 259's
/// `(char *)p + ((char *)ptr - (char *)po)` the `.add` of that difference.
/// C: void * realloc_after_dump_9x (void *ptr, size_t size);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc_after_dump_9x (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  if FREEABLE_P! (ptr)
    {
      let po: *mut c_void = *((ptr as *mut *mut c_void).sub (1));
      let mut p: *mut c_void;
      let mut pa: *mut c_void;
      p = realloc_after_dump (po, size + 8);
      if p.is_null ()
	{ return p; }
      pa = ((p as intptr_t + 8) & !7) as *mut c_void;
      if !ptr.is_null ()
	  && (pa as *mut c_char).offset_from (p as *mut c_char)
	     != (ptr as *mut c_char).offset_from (po as *mut c_char)
	{
	  /* Handle the case where alignment in pre-realloc and
	     post-realloc blocks does not match.  */
	  MoveMemory! (pa, ((p as *mut c_char).add ((ptr as *mut c_char).offset_from (po as *mut c_char)
						   as usize)) as *mut c_void, size);
	}
      *((pa as *mut *mut c_void).sub (1)) = p;
      return pa;
    }
  else
    {
      /* Non-freeable pointers have no alignment-enforcing header
         (since dumping is not allowed on Windows 9X).  */
      let p: *mut c_void = malloc_after_dump_9x (size);
      if !p.is_null ()
	{ CopyMemory! (p, ptr, size); }
      return p;
    }
}

/// C 275-282: `free_after_dump_9x`.
/// C: void free_after_dump_9x (void *ptr);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_after_dump_9x (ptr: *mut c_void)
{
  if FREEABLE_P! (ptr)
    {
      free_after_dump (*((ptr as *mut *mut c_void).sub (1)));
    }
}

/// C 284-292: `sys_calloc` - C 288's `(*the_malloc_fn) (nbytes)` is the port's `.unwrap ()` call
/// (the `src/w32proc.rs` reading of the same indirection: the CRT's function-pointer object is a
/// nullable pointer, and the port's `Option<extern fn>` is that pointer).
/// C: void * sys_calloc (size_t number, size_t size);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_calloc (number: size_t, size: size_t) -> *mut c_void
{
  let nbytes: size_t = number * size;
  let mut ptr: *mut c_void = (the_malloc_fn.unwrap ()) (nbytes);
  if !ptr.is_null ()
    { memset (ptr, 0, nbytes); }
  return ptr;
}

/// C 294-299: `getpagesize` - the emulation `nt/inc/ms-w32.h` 472 declares.  `sysinfo_cache` and
/// its `dwPageSize` belong to `include/w32common.rs`; C's `DWORD`->`int` conversion is spelled.
/// C: int getpagesize (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getpagesize () -> c_int
{
  return sysinfo_cache.dwPageSize as c_int;
}

/// C 301-310: `sbrk` - the one value of the parameter C 308 asserts on.  `ptrdiff_t` is the
/// port's `isize` (PORTING.md 1.1).
/// C: void * sbrk (ptrdiff_t increment);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sbrk (increment: isize) -> *mut c_void
{
  /* data_region_end is the address beyond the last allocated byte.
     The sbrk() function is not emulated at all, except for a 0 value
     of its parameter.  This is needed by the Emacs Lisp function
     `memory-limit'.  */
  eassert! (increment == 0);
  return data_region_end as *mut c_void;
}

/* C 311-312: the blank and the form feed that separate the two slices.  */

/* MMAP allocation for buffers.  */

/// C 316: `#define MAX_BUFFER_SIZE (512 * 1024 * 1024)` - C's `int` constant; the comparison
/// C 331 makes against a `size_t` is the port's `as size_t`.
/// C: #define MAX_BUFFER_SIZE (512 * 1024 * 1024)
#[cfg (feature = "windowsnt")]
pub const MAX_BUFFER_SIZE: c_int = (512 * 1024 * 1024);

/// C 318-363: `mmap_alloc` - the amortized allocation C 323-330 describes.  `ROUND_UP` is
/// `include/w32common.rs` 45-49's macro; its `align` operand is used uncast in the first term
/// there, where C's own macro relies on the usual arithmetic conversions, so the port passes
/// `get_allocation_unit () as DWORD_PTR` - the conversion C performs, spelled.  C 362's
/// `return *var = p;` assigns and returns; the port writes the assignment and the return.
/// C: void * mmap_alloc (void **var, size_t nbytes);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mmap_alloc (var: *mut *mut c_void, nbytes: size_t) -> *mut c_void
{
  let mut p: *mut c_void = core::ptr::null_mut ();

  /* We implement amortized allocation.  We start by reserving twice
     the size requested and commit only the size requested.  Then
     realloc could proceed and use the reserved pages, reallocating
     only if needed.  Buffer shrink would happen only so that we stay
     in the 2x range.  This is a big win when visiting compressed
     files, where the final size of the buffer is not known in
     advance, and the buffer is enlarged several times as the data is
     decompressed on the fly.  */
  if nbytes < MAX_BUFFER_SIZE as size_t
    { p = VirtualAlloc (core::ptr::null_mut (),
			ROUND_UP! (nbytes * 2, get_allocation_unit! () as DWORD_PTR),
			MEM_RESERVE, PAGE_READWRITE); }

  /* If it fails, or if the request is above 512MB, try with the
     requested size.  */
  if p.is_null ()
    { p = VirtualAlloc (core::ptr::null_mut (),
			ROUND_UP! (nbytes, get_allocation_unit! () as DWORD_PTR),
			MEM_RESERVE, PAGE_READWRITE); }

  if !p.is_null ()
    {
      /* Now, commit pages for NBYTES.  */
      *var = VirtualAlloc (p, nbytes, MEM_COMMIT, PAGE_READWRITE);
      if (*var).is_null ()
	{ p = *var; }
    }

  if p.is_null ()
    {
      let e: DWORD = GetLastError ();

      if e == ERROR_NOT_ENOUGH_MEMORY
	{ *errno () = ENOMEM; }
      else
	{
	  /* C 357's report.  */
	  DebPrint! (("mmap_alloc: error %ld\n", e));
	  *errno () = EINVAL;
	}
    }

  *var = p;
  return p;
}

/// C 365-374: `mmap_free`.
/// C: void mmap_free (void **var);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mmap_free (var: *mut *mut c_void)
{
  if !(*var).is_null ()
    {
      if VirtualFree (*var, 0, MEM_RELEASE) == 0
	{ DebPrint! (("mmap_free: error %ld\n", GetLastError ())); }
      *var = core::ptr::null_mut ();
    }
}

/// C 376-479: `mmap_realloc` - the grow, the shrink and the in-place decommit.  C leaves the two
/// records and `old_ptr` unset on the paths that use them; the port writes C's own zero values
/// (PORTING.md 0.1-13 / 1.5).  `getpagesize ()` is this file's own function (C 294) and
/// `get_page_size ()` is `include/w32common.rs`'s macro (C 470-471); both are `int`/`DWORD` in C
/// and the port spells the `size_t` arithmetic's conversions.
/// C: void * mmap_realloc (void **var, size_t nbytes);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mmap_realloc (var: *mut *mut c_void, nbytes: size_t) -> *mut c_void
{
  let mut memInfo: MEMORY_BASIC_INFORMATION = core::mem::zeroed ();
  let mut m2: MEMORY_BASIC_INFORMATION = core::mem::zeroed ();
  let mut old_ptr: *mut c_void = core::ptr::null_mut ();

  if (*var).is_null ()
    { return mmap_alloc (var, nbytes); }

  /* This case happens in init_buffer().  */
  if nbytes == 0
    {
      mmap_free (var);
      return mmap_alloc (var, nbytes);
    }

  memset (&raw mut memInfo as *mut c_void, 0, core::mem::size_of_val (&memInfo));
  if VirtualQuery ((*var) as LPCVOID, &raw mut memInfo,
		   core::mem::size_of_val (&memInfo)) == 0
    { DebPrint! (("mmap_realloc: VirtualQuery error = %ld\n", GetLastError ())); }

  /* We need to enlarge the block.  */
  if memInfo.RegionSize < nbytes
    {
      memset (&raw mut m2 as *mut c_void, 0, core::mem::size_of_val (&m2));
      if VirtualQuery (((*var) as *mut c_char).add (memInfo.RegionSize) as LPCVOID,
		       &raw mut m2, core::mem::size_of_val (&m2)) == 0
	{ DebPrint! (("mmap_realloc: VirtualQuery error = %ld\n",
		      GetLastError ())); }
      /* If there is enough room in the current reserved area, then
	 commit more pages as needed.  */
      if m2.State == MEM_RESERVE
	  && m2.AllocationBase == memInfo.AllocationBase
	  && nbytes <= memInfo.RegionSize + m2.RegionSize
	{
	  let mut p: *mut c_void;

	  p = VirtualAlloc (*var, nbytes, MEM_COMMIT, PAGE_READWRITE);
	  if p.is_null () /* && GetLastError() != ERROR_NOT_ENOUGH_MEMORY */
	    {
	      /* C 414-420's two reports use MSVC's `%I64x` and C's `(uint64_t)` casts; the port
		 keeps the casts (`u64`) and the format text C writes.  */
	      DebPrint! (("realloc enlarge: VirtualAlloc (%p + %I64x, %I64x) error %ld\n",
			  *var, memInfo.RegionSize as u64,
			  (nbytes - memInfo.RegionSize) as u64,
			  GetLastError ()));
	      DebPrint! (("next region: %p %p %I64x %x\n", m2.BaseAddress,
			  m2.AllocationBase, m2.RegionSize as u64,
			  m2.AllocationProtect));
	    }
	  else
	    { return *var; }
	}
      /* Else we must actually enlarge the block by allocating a new
	 one and copying previous contents from the old to the new one.  */
      old_ptr = *var;

      if !mmap_alloc (var, nbytes).is_null ()
	{
	  CopyMemory! (*var, old_ptr, memInfo.RegionSize);
	  mmap_free (&raw mut old_ptr);
	  return *var;
	}
      else
	{
	  /* We failed to reallocate the buffer.  */
	  *var = old_ptr;
	  return core::ptr::null_mut ();
	}
    }

  /* If we are shrinking by more than one page...  */
  if memInfo.RegionSize > nbytes + getpagesize () as size_t
    {
      /* If we are shrinking a lot...  */
      if (memInfo.RegionSize / 2) > nbytes
	{
	  /* Let's give some memory back to the system and release
	     some pages.  */
	  old_ptr = *var;

	  if !mmap_alloc (var, nbytes).is_null ()
	    {
	      CopyMemory! (*var, old_ptr, nbytes);
	      mmap_free (&raw mut old_ptr);
	      return *var;
	    }
	  else
	    {
	      /* In case we fail to shrink, try to go on with the old block.
		 But that means there is a lot of memory pressure.
		 We could also decommit pages.  */
	      *var = old_ptr;
	      return *var;
	    }
	}

      /* We still can decommit pages.  */
      if VirtualFree ((((*var) as *mut c_char).add (nbytes + get_page_size! () as size_t)) as LPVOID,
		      memInfo.RegionSize - nbytes - get_page_size! () as size_t,
		      MEM_DECOMMIT) == 0
	{ DebPrint! (("mmap_realloc: VirtualFree error %ld\n", GetLastError ())); }
      return *var;
    }

  /* Not enlarging, not shrinking by more than one page.  */
  return *var;
}

/* C 481's form feed and C 482's banner, carried (the form feed itself is not text).  */

/* Emulation of getrlimit and setrlimit.  */

/// C 484-537: `getrlimit`.  C's `switch`/`case RLIMIT_STACK`/`case RLIMIT_NOFILE`/`default` is
/// PORTING.md 2.2's `match`; the `RLIMIT_*` labels are the constants
/// `nt/inc/sys/include/resource.rs` declares.  C 506-509's `DWORD_PTR` casts of the record's
/// members and of `&m` are the port's `as DWORD_PTR`; C 493's record is one of the locals C leaves
/// unset, so the port zeroes it (PORTING.md 1.5) before the `VirtualQuery` that fills it.
/// C: int getrlimit (rlimit_resource_t rltype, struct rlimit *rlp);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getrlimit (rltype: rlimit_resource_t, rlp: *mut rlimit) -> c_int
{
  let mut retval: c_int = -1;

  match rltype
    {
      RLIMIT_STACK =>
	{
	  let mut m: MEMORY_BASIC_INFORMATION = core::mem::zeroed ();
	  /* Implementation note: Posix says that RLIMIT_STACK returns
	     information about the stack size for the main thread.  The
	     implementation below returns the stack size for the calling
	     thread, so it's more like pthread_attr_getstacksize.  But
	     Emacs clearly wants the latter, given how it uses the
	     results, so the implementation below is more future-proof,
	     if what's now the main thread will become some other thread
	     at some future point.  */
	  if VirtualQuery ((&raw const m) as LPCVOID, &raw mut m,
			   core::mem::size_of_val (&m)) == 0
	    { *errno () = EPERM; }
	  else
	    {
	      (*rlp).rlim_cur = (((&raw const m) as DWORD_PTR) - (m.AllocationBase as DWORD_PTR)) as rlim_t;
	      (*rlp).rlim_max =
		(m.BaseAddress as DWORD_PTR + m.RegionSize as DWORD_PTR
		 - m.AllocationBase as DWORD_PTR) as rlim_t;

	      /* The last page is the guard page, so subtract that.  */
	      (*rlp).rlim_cur -= getpagesize () as rlim_t;
	      (*rlp).rlim_max -= getpagesize () as rlim_t;
	      retval = 0;
	    }
	}
      RLIMIT_NOFILE =>
	{
	  /* Implementation note: The real value is returned by
	     _getmaxstdio.  But our FD_SETSIZE is smaller, to cater to
	     Windows 9X, and process.c includes some logic that's based on
	     the assumption that the handle resource is inherited to child
	     processes.  We want to avoid that logic, so we tell process.c
	     our current limit is already equal to FD_SETSIZE.  */
	  (*rlp).rlim_cur = FD_SETSIZE as rlim_t;
	  (*rlp).rlim_max = 2048;	/* see _setmaxstdio documentation */
	  retval = 0;
	}
      _ =>
	{
	  /* Note: we could return meaningful results for other RLIMIT_*
	     requests, but Emacs doesn't currently need that, so we just
	     punt for them.  */
	  *errno () = ENOSYS;
	}
    }
  return retval;
}

/// C 539-554: `setrlimit` - C 544-545's two `case` labels share one arm.  `rlp` is C's unused
/// parameter (the two limits cannot be changed).
/// C: int setrlimit (rlimit_resource_t rltype, const struct rlimit *rlp);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setrlimit (rltype: rlimit_resource_t, rlp: *const rlimit) -> c_int
{
  match rltype
    {
      RLIMIT_STACK | RLIMIT_NOFILE =>
	{
	  /* We cannot modify these limits, so we always fail.  */
	  *errno () = EPERM;
	}
      _ =>
	{
	  *errno () = ENOSYS;
	}
    }
  return -1;
}
