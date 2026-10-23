// femacs: port of src/ralloc.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`, except `r_alloc_sbrk`, whose address C stores in the
// `__morecore` hook and compares there (C 1162-1163, C 1003) - a C-ABI pointer position, so it is
// `unsafe extern "C" fn` (PORTING.md 1.4, the `androidmenu.rs` shape).
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**
// `emacs/src/Makefile.in` 495 puts `ralloc.o` in `RALLOC_OBJ` (`RALLOC_OBJ=@RALLOC_OBJ@`), whose
// value `configure.ac` 7320-7326 fills only when `REL_ALLOC = yes` *and* `system_malloc != yes`
// (`configure.ac` 3355-3368 decides `REL_ALLOC`; the frozen build's generated `config.h` leaves it
// undefined, and its `SYSTEM_MALLOC` is 1).  `docs/baseline/frozen-build-sources.txt` therefore
// lists no `ralloc.o` for this target, `extract-abi.py`'s checklist for the file is empty
// (P-86 ②), and the ledger `docs/ports/ralloc.md` carries that clause's substitute: the
// **C line-range table**, the `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `all (feature = "rel_alloc", feature = "no_system_malloc")` - C's
//     `REL_ALLOC && !SYSTEM_MALLOC`, the two features registered at `xtask/Cargo.toml` 586
//     and 534 - on every item *and on C 27-33's include graph*, which is why the frozen
//     configuration compiles nothing here;
//   * the sub-gates are C's own `#if`s, each on the item it gates: C 80 `#ifdef HAVE_MALLOC_H`
//     (`feature = "have_malloc_h"`), C 83 `#ifndef DOUG_LEA_MALLOC`
//     (`not (feature = "doug_lea_malloc")`), C 995 `#ifdef DOUG_LEA_MALLOC`, C 1013 `#ifdef DEBUG`
//     (`feature = "debug"`), and C 1161/1180/1193's `#if !defined SYSTEM_MALLOC`
//     (`feature = "no_system_malloc"`, nested in the file gate).  The frozen values of all five
//     are in the ledger's gate table.
//   * C 27's `<stddef.h>`, C 31's `<unistd.h>` and C 80's `<malloc.h>` are system headers with no
//     file in the reference tree.  The first two have port mirrors (`lib/include/stddef.in.rs`,
//     `lib/include/unistd.in.rs` - the second is also where `getpagesize` comes from, because
//     C 33's `getpagesize.h` has no live text on this target); C 80's surface is declared at its
//     `#include` line in this unit (P-88 ①, see the section before the `#ifndef DOUG_LEA_MALLOC`
//     one).  `sbrk`, which C 1028 uses and `<unistd.h>` declares, is declared in C 1013's section
//     (the only site that names it).
//
// `docs/ports/ralloc.md` carries the C line ranges, the acceptance readings, every deviation (the
// `<malloc.h>` stand-in section, the explicit null-pointer casts, the `assert`/`assert!` spelling,
// the line-count tolerance this file needs) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn,
         unused_assignments, unused_parens, unused_mut, unreachable_code, unused_labels, while_true,
         unused_comparisons, overflowing_literals, unused_must_use, clippy::all)]

/* NOTES:

   Only relocate the blocs necessary for SIZE in r_alloc_sbrk,
   rather than all of them.  This means allowing for a possible
   hole between the first bloc and the end of malloc storage.  */

include!("include/config.rs");                  // C 25: `#include <config.h>`

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
include!("../lib/include/stddef.in.rs");        // C 27: `#include <stddef.h>`
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
include!("include/lisp.rs");                    // C 29: `#include "lisp.h"`
/* C reaches `<stdlib.h>` and `<string.h>` through lisp.h's own include list (lisp.h 30 is
   `#include <string.h>`; the frozen preprocessor's `stdlib.h` marker is what declares `malloc`
   and `free`).  The port's `include/lisp.rs` carries neither mirror yet, so they are included
   here, where C's chain reaches them (the `buffer.rs` 38/41 reading).  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
include!("../lib/include/stdlib.rs");           // lisp.h's chain: `malloc`, `free`
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
include!("../lib/include/string.rs");           // lisp.h 30: `#include <string.h>`
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
include!("include/blockinput.rs");              // C 30: `#include "blockinput.h"`
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
include!("../lib/include/unistd.in.rs");        // C 31: `#include <unistd.h>` (getpagesize)
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
include!("include/getpagesize.rs");             // C 33: `#include "getpagesize.h"`

/* A flag to indicate whether we have initialized ralloc yet.  For
   Emacs's sake, please do not make this local to malloc_init; on some
   machines, the dumping procedure makes all static variables
   read-only.  On these machines, the word static is #defined to be
   the empty string, meaning that r_alloc_initialized becomes an
   automatic variable, and loses its value each time Emacs is started
   up.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut r_alloc_initialized: c_int = 0;

/* C 45: `static void r_alloc_init (void);` - a forward declaration, which has no Rust counterpart
   (a function item is visible for the whole module regardless of position; the definition is
   C 1153-1214 below).  */

/* Declarations for working with the malloc, ralloc, and system breaks.  */

/* Function to set the real break value.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
#[unsafe(no_mangle)]
pub static mut real_morecore: Option<unsafe extern "C" fn (ptrdiff_t) -> *mut c_void> = None;

/* The break value, as seen by malloc.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut virtual_break_value: *mut c_void = core::ptr::null_mut ();

/* The address of the end of the last data in use by ralloc,
   including relocatable blocs as well as malloc data.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut break_value: *mut c_void = core::ptr::null_mut ();

/* This is the size of a page.  We round memory requests to this boundary.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut page_size: c_int = 0;

/* Whenever we get memory from the system, get this many extra bytes.  This
   must be a multiple of page_size.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut extra_bytes: c_int = 0;

/* Macros for rounding.  Note that rounding to any value is possible
   by changing the definition of PAGE.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
macro_rules! PAGE { () => { getpagesize () }; }
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
macro_rules! PAGE_ROUNDUP {
  ($size:expr) => { ((($size) as size_t + page_size as size_t - 1)
		     & !(page_size as size_t - 1)) };
}

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
macro_rules! MEM_ALIGN { () => { core::mem::size_of::<c_double> () }; }
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
macro_rules! MEM_ROUNDUP {
  ($addr:expr) => { ((($addr) as size_t + MEM_ALIGN! () - 1)
		     & !(MEM_ALIGN! () - 1)) };
}

/* The hook `malloc' uses for the function which gets more space
   from the system.  */

/* C 80-82: `#ifdef HAVE_MALLOC_H` / `# include <malloc.h>` / `#endif`.  The header has no file in
   the reference tree (P-88 ①: the declaration site is C's own `#include` line, and no file is
   added under `src/include/`), so the surface this file reaches through it is declared
   here: `mallopt` and `M_TOP_PAD`, both used by C 1176's arm.  C's arm is dead text on the frozen
   target (`HAVE_MALLOC_H` is undefined there), and the two names are reachable only when either
   C's `HAVE_MALLOC_H` or its `DOUG_LEA_MALLOC` holds (C 1187-1188 records that the second implies
   the first on every system that has it), so this one declaration site cannot become a duplicate
   item (`E0428`) under a configuration that sets both features.  Both names are glibc's; this
   machine has no `<malloc.h>` to measure them against, so `M_TOP_PAD`'s value is **UNVERIFIED**.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc",
            any (feature = "have_malloc_h", feature = "doug_lea_malloc")))]
unsafe extern "C" {
  /// C 81's `<malloc.h>`: `extern int mallopt (int __param, int __val) __THROW;`
  /// C: extern int mallopt (int __param, int __val) __THROW;
  pub fn mallopt (param: c_int, value: c_int) -> c_int;
}

/// C 81's `<malloc.h>`: `#define M_TOP_PAD -2` (glibc's value; UNVERIFIED on this machine).
/// C: #define M_TOP_PAD -2
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc",
            any (feature = "have_malloc_h", feature = "doug_lea_malloc")))]
pub const M_TOP_PAD: c_int = -2;

/* C 83-85: `#ifndef DOUG_LEA_MALLOC` / `extern void *(*__morecore) (ptrdiff_t);` / `#endif` -
   the declaration of the hook variable (the `libc`'s).  C has no `#else`, so the arm is empty
   when `DOUG_LEA_MALLOC` holds and the name comes from `<malloc.h>`'s `_G_config.h` chain, which
   `lib.rs`'s `__morecore` site is (PORTING.md P-88 ①, the `vm-limit.rs` 44-48 shape).  Rust
   cannot declare a freed-standing `extern` item twice, so the declaration is here, at C's own
   line, under the frozen arm only.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc",
            not (feature = "doug_lea_malloc")))]
unsafe extern "C" {
  /// C 84: `extern void *(*__morecore) (ptrdiff_t);`
  /// C: extern void *(*__morecore) (ptrdiff_t);
  pub static mut __morecore: Option<unsafe extern "C" fn (ptrdiff_t) -> *mut c_void>;
}

/***********************************************************************
		      Implementation using sbrk
 ***********************************************************************/

/* Data structures of heaps and blocs.  */

/* The relocatable objects, or blocs, and the malloc data
   both reside within one or more heaps.
   Each heap contains malloc data, running from `start' to `bloc_start',
   and relocatable objects, running from `bloc_start' to `free'.

   Relocatable objects may relocate within the same heap
   or may move into another heap; the heaps themselves may grow
   but they never move.

   We try to make just one heap and make it larger as necessary.
   But sometimes we can't do that, because we can't get contiguous
   space to add onto the heap.  When that happens, we start a new heap.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
#[repr(C)]
pub struct heap
{
  pub next: *mut heap,
  pub prev: *mut heap,
  /* Start of memory range of this heap.  */
  pub start: *mut c_void,
  /* End of memory range of this heap.  */
  pub end: *mut c_void,
  /* Start of relocatable data in this heap.  */
  pub bloc_start: *mut c_void,
  /* Start of unused space in this heap.  */
  pub free: *mut c_void,
  /* First bloc in this heap.  */
  pub first_bloc: *mut bp,
  /* Last bloc in this heap.  */
  pub last_bloc: *mut bp,
}
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
pub type heap_ptr = *mut heap;

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
macro_rules! NIL_HEAP { () => { 0 as heap_ptr }; }

/* This is the first heap object.
   If we need additional heap objects, each one resides at the beginning of
   the space it covers.   */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut heap_base: heap = unsafe { core::mem::zeroed () };   // C 131 (tentative)

/* Head and tail of the list of heaps.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut first_heap: heap_ptr = core::ptr::null_mut ();
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut last_heap: heap_ptr = core::ptr::null_mut ();

/* These structures are allocated in the malloc arena.
   The linked list is kept in order of increasing '.data' members.
   The data blocks abut each other; if b->next is non-nil, then
   b->data + b->size == b->next->data.

   An element with variable==NULL denotes a freed block, which has not yet
   been collected.  They may only appear while r_alloc_freeze_level > 0,
   and will be freed when the arena is thawed.  Currently, these blocs are
   not reusable, while the arena is frozen.  Very inefficient.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
#[repr(C)]
pub struct bp
{
  pub next: *mut bp,
  pub prev: *mut bp,
  pub variable: *mut *mut c_void,
  pub data: *mut c_void,
  pub size: size_t,
  pub new_data: *mut c_void,		/* temporarily used for relocation */
  pub heap: *mut heap,			/* Heap this bloc is in.  */
}
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
pub type bloc_ptr = *mut bp;

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
macro_rules! NIL_BLOC { () => { 0 as bloc_ptr }; }
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
macro_rules! BLOC_PTR_SIZE { () => { core::mem::size_of::<bp> () }; }

/* Head and tail of the list of relocatable blocs.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut first_bloc: bloc_ptr = core::ptr::null_mut ();
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut last_bloc: bloc_ptr = core::ptr::null_mut ();

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut use_relocatable_buffers: c_int = 0;

/* If >0, no relocation whatsoever takes place.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
static mut r_alloc_freeze_level: c_int = 0;

/* Functions to get and return memory from the system.  */

/* Find the heap that ADDRESS falls within.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn find_heap (address: *mut c_void) -> heap_ptr
{
  let mut heap: heap_ptr;

  heap = last_heap;
  loop
    {
      if (*heap).start <= address && address <= (*heap).end
	{
	  return heap;
	}
      heap = (*heap).prev;
    }
}

/* Find SIZE bytes of space in a heap.
   Try to get them at ADDRESS (which must fall within some heap's range)
   if we can get that many within one heap.

   If enough space is not presently available in our reserve, this means
   getting more page-aligned space from the system.  If the returned space
   is not contiguous to the last heap, allocate a new heap, and append it
   to the heap list.

   obtain does not try to keep track of whether space is in use or not
   in use.  It just returns the address of SIZE bytes that fall within a
   single heap.  If you call obtain twice in a row with the same arguments,
   you typically get the same value.  It's the caller's responsibility to
   keep track of what space is in use.

   Return the address of the space if all went well, or zero if we couldn't
   allocate the memory.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn obtain (mut address: *mut c_void, size: size_t) -> *mut c_void
{
  let mut heap: heap_ptr;
  let mut already_available: size_t;

  /* Find the heap that ADDRESS falls within.  */
  heap = last_heap;
  while !heap.is_null ()
    {
      if (*heap).start <= address && address <= (*heap).end
	{
	  break;
	}
      heap = (*heap).prev;
    }

  if heap.is_null ()
    {
      emacs_abort ();
    }

  /* If we can't fit SIZE bytes in that heap,
     try successive later heaps.  */
  while !heap.is_null ()
    && (address as *mut c_char).add (size) > (*heap).end as *mut c_char
    {
      heap = (*heap).next;
      if heap == NIL_HEAP! ()
	{
	  break;
	}
      address = (*heap).bloc_start;
    }

  /* If we can't fit them within any existing heap,
     get more space.  */
  if heap == NIL_HEAP! ()
    {
      let new = real_morecore.unwrap () (0);
      let mut get: size_t;

      already_available = ((*last_heap).end as *mut c_char).offset_from (address as *mut c_char)
	as size_t;

      if new != (*last_heap).end
	{
	  /* Someone else called sbrk.  Make a new heap.  */

	  let new_heap: heap_ptr = MEM_ROUNDUP! (new as size_t) as heap_ptr;
	  /* C 244: `(void *) MEM_ROUNDUP ((void *) (new_heap + 1))` - `new_heap + 1` is C's own
	     pointer arithmetic on `struct heap *`, which Rust spells `.offset (1)`.  */
	  let bloc_start: *mut c_void
	    = MEM_ROUNDUP! (new_heap.offset (1)) as *mut c_void;

	  if real_morecore.unwrap () ((bloc_start as *mut c_char)
				      .offset_from (new as *mut c_char) as ptrdiff_t) != new
	    {
	      return 0 as *mut c_void;
	    }

	  (*new_heap).start = new;
	  (*new_heap).end = bloc_start;
	  (*new_heap).bloc_start = bloc_start;
	  (*new_heap).free = bloc_start;
	  (*new_heap).next = NIL_HEAP! ();
	  (*new_heap).prev = last_heap;
	  (*new_heap).first_bloc = NIL_BLOC! ();
	  (*new_heap).last_bloc = NIL_BLOC! ();
	  (*last_heap).next = new_heap;
	  last_heap = new_heap;

	  address = bloc_start;
	  already_available = 0;
	}

      /* Add space to the last heap (which we may have just created).
	 Get some extra, so we can come here less often.  */

      get = size + extra_bytes as size_t - already_available;
      get = PAGE_ROUNDUP! (((*last_heap).end as *mut c_char).add (get))
	- ((*last_heap).end as size_t);

      if real_morecore.unwrap () (get as ptrdiff_t) != (*last_heap).end
	{
	  return 0 as *mut c_void;
	}

      (*last_heap).end = ((*last_heap).end as *mut c_char).add (get) as *mut c_void;
    }

  return address;
}

/* Return unused heap space to the system
   if there is a lot of unused space now.
   This can make the last heap smaller;
   it can also eliminate the last heap entirely.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn relinquish ()
{
  let mut h: heap_ptr;
  let mut excess: ptrdiff_t = 0;

  /* Add the amount of space beyond break_value
     in all heaps which have extend beyond break_value at all.  */

  h = last_heap;
  while !h.is_null () && break_value < (*h).end
    {
      excess += ((*h).end as *mut c_char).offset_from
	((if break_value < (*h).bloc_start { (*h).bloc_start } else { break_value })
	 as *mut c_char) as ptrdiff_t;
      h = (*h).prev;
    }

  if excess > extra_bytes as ptrdiff_t * 2 && real_morecore.unwrap () (0) == (*last_heap).end
    {
      /* Keep extra_bytes worth of empty space.
	 And don't free anything unless we can free at least extra_bytes.  */
      excess -= extra_bytes as ptrdiff_t;

      if ((*last_heap).end as *mut c_char).offset_from ((*last_heap).bloc_start as *mut c_char)
	<= excess
	{
	  /* This heap should have no blocs in it.  If it does, we
	     cannot return it to the system.  */
	  if (*last_heap).first_bloc != NIL_BLOC! () || (*last_heap).last_bloc != NIL_BLOC! ()
	    {
	      return;
	    }

	  /* Return the last heap, with its header, to the system.  */
	  excess = ((*last_heap).end as *mut c_char)
	    .offset_from ((*last_heap).start as *mut c_char) as ptrdiff_t;
	  let lh_prev = (*last_heap).prev;
	  /* If the system doesn't want that much memory back, leave
	     last_heap unaltered to reflect that.  This can occur if
	     break_value is still within the original data segment.  */
	  if real_morecore.unwrap () (- excess) != 0 as *mut c_void
	    {
	      last_heap = lh_prev;
	      (*last_heap).next = NIL_HEAP! ();
	    }
	}
      else
	{
	  excess = ((*last_heap).end as *mut c_char).offset_from
	    (PAGE_ROUNDUP! (((*last_heap).end as size_t) - (excess as size_t)) as *mut c_char)
	    as ptrdiff_t;
	  /* If the system doesn't want that much memory back, leave
	     the end of the last heap unchanged to reflect that.  This
	     can occur if break_value is still within the original
	     data segment.  */
	  if real_morecore.unwrap () (- excess) != 0 as *mut c_void
	    {
	      (*last_heap).end = ((*last_heap).end as *mut c_char).offset (- excess) as *mut c_void;
	    }
	}
    }
}

/* The meat - allocating, freeing, and relocating blocs.  */

/* Find the bloc referenced by the address in PTR.  Returns a pointer
   to that block.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn find_bloc (ptr: *mut *mut c_void) -> bloc_ptr
{
  let mut p: bloc_ptr = first_bloc;

  while p != NIL_BLOC! ()
    {
      /* Consistency check. Don't return inconsistent blocs.
	 Don't abort here, as callers might be expecting this, but
	 callers that always expect a bloc to be returned should abort
	 if one isn't to avoid a memory corruption bug that is
	 difficult to track down.  */
      if (*p).variable == ptr && (*p).data == *ptr
	{
	  return p;
	}

      p = (*p).next;
    }

  return p;
}

/* Allocate a bloc of SIZE bytes and append it to the chain of blocs.
   Returns a pointer to the new bloc, or zero if we couldn't allocate
   memory for the new block.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn get_bloc (size: size_t) -> bloc_ptr
{
  let mut new_bloc: bloc_ptr;
  let mut heap: heap_ptr;

  new_bloc = malloc (BLOC_PTR_SIZE! ()) as bloc_ptr;
  if !new_bloc.is_null ()
    {
      (*new_bloc).data = obtain (break_value, size);
    }
  if new_bloc.is_null () || (*new_bloc).data.is_null ()
    {
      free (new_bloc as *mut c_void);

      return 0 as bloc_ptr;
    }

  break_value = ((*new_bloc).data as *mut c_char).add (size) as *mut c_void;

  (*new_bloc).size = size;
  (*new_bloc).next = NIL_BLOC! ();
  (*new_bloc).variable = core::ptr::null_mut ();
  (*new_bloc).new_data = 0 as *mut c_void;

  /* Record in the heap that this space is in use.  */
  heap = find_heap ((*new_bloc).data);
  (*heap).free = break_value;

  /* Maintain the correspondence between heaps and blocs.  */
  (*new_bloc).heap = heap;
  (*heap).last_bloc = new_bloc;
  if (*heap).first_bloc == NIL_BLOC! ()
    {
      (*heap).first_bloc = new_bloc;
    }

  /* Put this bloc on the doubly-linked list of blocs.  */
  if !first_bloc.is_null ()
    {
      (*new_bloc).prev = last_bloc;
      (*last_bloc).next = new_bloc;
      last_bloc = new_bloc;
    }
  else
    {
      first_bloc = new_bloc;
      last_bloc = new_bloc;
      (*new_bloc).prev = NIL_BLOC! ();
    }

  return new_bloc;
}

/* Calculate new locations of blocs in the list beginning with BLOC,
   relocating it to start at ADDRESS, in heap HEAP.  If enough space is
   not presently available in our reserve, call obtain for
   more space.

   Store the new location of each bloc in its new_data field.
   Do not touch the contents of blocs or break_value.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn relocate_blocs (bloc: bloc_ptr, heap: heap_ptr, address: *mut c_void) -> c_int
{
  let mut b: bloc_ptr = bloc;
  let mut heap = heap;
  let mut address = address;

  /* No need to ever call this if arena is frozen, bug somewhere!  */
  if r_alloc_freeze_level != 0
    {
      emacs_abort ();
    }

  while !b.is_null ()
    {
      /* If bloc B won't fit within HEAP,
	 move to the next heap and try again.  */
      while !heap.is_null () && (address as *mut c_char).add ((*b).size) > (*heap).end as *mut c_char
	{
	  heap = (*heap).next;
	  if heap == NIL_HEAP! ()
	    {
	      break;
	    }
	  address = (*heap).bloc_start;
	}

      /* If BLOC won't fit in any heap,
	 get enough new space to hold BLOC and all following blocs.  */
      if heap == NIL_HEAP! ()
	{
	  let mut tb: bloc_ptr = b;
	  let mut s: size_t = 0;

	  /* Add up the size of all the following blocs.  */
	  while tb != NIL_BLOC! ()
	    {
	      if !(*tb).variable.is_null ()
		{
		  s += (*tb).size;
		}

	      tb = (*tb).next;
	    }

	  /* Get that space.  */
	  address = obtain (address, s);
	  if address.is_null ()
	    {
	      return 0;
	    }

	  heap = last_heap;
	}

      /* Record the new address of this bloc
	 and update where the next bloc can start.  */
      (*b).new_data = address;
      if !(*b).variable.is_null ()
	{
	  address = (address as *mut c_char).add ((*b).size) as *mut c_void;
	}
      b = (*b).next;
    }

  return 1;
}

/* Update the records of which heaps contain which blocs, starting
   with heap HEAP and bloc BLOC.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn update_heap_bloc_correspondence (bloc: bloc_ptr, heap: heap_ptr)
{
  let mut b: bloc_ptr;
  let mut heap = heap;

  /* Initialize HEAP's status to reflect blocs before BLOC.  */
  if bloc != NIL_BLOC! () && (*bloc).prev != NIL_BLOC! () && (*(*bloc).prev).heap == heap
    {
      /* The previous bloc is in HEAP.  */
      (*heap).last_bloc = (*bloc).prev;
      (*heap).free = ((*(*bloc).prev).data as *mut c_char).add ((*(*bloc).prev).size) as *mut c_void;
    }
  else
    {
      /* HEAP contains no blocs before BLOC.  */
      (*heap).first_bloc = NIL_BLOC! ();
      (*heap).last_bloc = NIL_BLOC! ();
      (*heap).free = (*heap).bloc_start;
    }

  /* Advance through blocs one by one.  */
  b = bloc;
  while b != NIL_BLOC! ()
    {
      /* Advance through heaps, marking them empty,
	 till we get to the one that B is in.  */
      while !((*heap).bloc_start <= (*b).data && (*b).data <= (*heap).end)
	{
	  heap = (*heap).next;
	  /* We know HEAP is not null now,
	     because there has to be space for bloc B.  */
	  (*heap).first_bloc = NIL_BLOC! ();
	  (*heap).last_bloc = NIL_BLOC! ();
	  (*heap).free = (*heap).bloc_start;
	}

      /* Update HEAP's status for bloc B.  */
      (*heap).free = ((*b).data as *mut c_char).add ((*b).size) as *mut c_void;
      (*heap).last_bloc = b;
      if (*heap).first_bloc == NIL_BLOC! ()
	{
	  (*heap).first_bloc = b;
	}

      /* Record that B is in HEAP.  */
      (*b).heap = heap;

      b = (*b).next;
    }

  /* If there are any remaining heaps and no blocs left,
     mark those heaps as empty.  */
  heap = (*heap).next;
  while !heap.is_null ()
    {
      (*heap).first_bloc = NIL_BLOC! ();
      (*heap).last_bloc = NIL_BLOC! ();
      (*heap).free = (*heap).bloc_start;
      heap = (*heap).next;
    }
}

/* Resize BLOC to SIZE bytes.  This relocates the blocs
   that come after BLOC in memory.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn resize_bloc (bloc: bloc_ptr, size: size_t) -> c_int
{
  let mut b: bloc_ptr;
  let mut heap: heap_ptr;
  let mut address: *mut c_void;
  let mut old_size: size_t;

  /* No need to ever call this if arena is frozen, bug somewhere!  */
  if r_alloc_freeze_level != 0
    {
      emacs_abort ();
    }

  if bloc == NIL_BLOC! () || size == (*bloc).size
    {
      return 1;
    }

  heap = first_heap;
  while heap != NIL_HEAP! ()
    {
      if (*heap).bloc_start <= (*bloc).data && (*bloc).data <= (*heap).end
	{
	  break;
	}
      heap = (*heap).next;
    }

  if heap == NIL_HEAP! ()
    {
      emacs_abort ();
    }

  old_size = (*bloc).size;
  (*bloc).size = size;

  /* Note that bloc could be moved into the previous heap.  */
  address = if !(*bloc).prev.is_null ()
    {
      ((*(*bloc).prev).data as *mut c_char).add ((*(*bloc).prev).size) as *mut c_void
    }
  else
    {
      (*first_heap).bloc_start
    };
  while !heap.is_null ()
    {
      if (*heap).bloc_start <= address && address <= (*heap).end
	{
	  break;
	}
      heap = (*heap).prev;
    }

  if relocate_blocs (bloc, heap, address) == 0
    {
      (*bloc).size = old_size;
      return 0;
    }

  if size > old_size
    {
      b = last_bloc;
      while b != bloc
	{
	  if (*b).variable.is_null ()
	    {
	      (*b).size = 0;
	      (*b).data = (*b).new_data;
	    }
	  else
	    {
	      if (*b).new_data != (*b).data
		{
		  memmove ((*b).new_data, (*b).data, (*b).size);
		}
	      (*b).data = (*b).new_data;
	      *(*b).variable = (*b).data;
	    }
	  b = (*b).prev;
	}
      if (*bloc).variable.is_null ()
	{
	  (*bloc).size = 0;
	  (*bloc).data = (*bloc).new_data;
	}
      else
	{
	  if (*bloc).new_data != (*bloc).data
	    {
	      memmove ((*bloc).new_data, (*bloc).data, old_size);
	    }
	  memset (((*bloc).new_data as *mut c_char).add (old_size) as *mut c_void, 0,
		  size - old_size);
	  (*bloc).data = (*bloc).new_data;
	  *(*bloc).variable = (*bloc).data;
	}
    }
  else
    {
      b = bloc;
      while b != NIL_BLOC! ()
	{
	  if (*b).variable.is_null ()
	    {
	      (*b).size = 0;
	      (*b).data = (*b).new_data;
	    }
	  else
	    {
	      if (*b).new_data != (*b).data
		{
		  memmove ((*b).new_data, (*b).data, (*b).size);
		}
	      (*b).data = (*b).new_data;
	      *(*b).variable = (*b).data;
	    }
	  b = (*b).next;
	}
    }

  update_heap_bloc_correspondence (bloc, heap);

  break_value = if !last_bloc.is_null ()
    {
      ((*last_bloc).data as *mut c_char).add ((*last_bloc).size) as *mut c_void
    }
  else
    {
      (*first_heap).bloc_start
    };
  return 1;
}

/* Free BLOC from the chain of blocs, relocating any blocs above it.
   This may return space to the system.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn free_bloc (bloc: bloc_ptr)
{
  let mut heap: heap_ptr = (*bloc).heap;
  let mut h: heap_ptr;

  if r_alloc_freeze_level != 0
    {
      (*bloc).variable = core::ptr::null_mut ();
      return;
    }

  resize_bloc (bloc, 0);

  if bloc == first_bloc && bloc == last_bloc
    {
      first_bloc = NIL_BLOC! ();
      last_bloc = NIL_BLOC! ();
    }
  else if bloc == last_bloc
    {
      last_bloc = (*bloc).prev;
      (*last_bloc).next = NIL_BLOC! ();
    }
  else if bloc == first_bloc
    {
      first_bloc = (*bloc).next;
      (*first_bloc).prev = NIL_BLOC! ();
    }
  else
    {
      (*(*bloc).next).prev = (*bloc).prev;
      (*(*bloc).prev).next = (*bloc).next;
    }

  /* Sometimes, 'heap' obtained from bloc->heap above is not really a
     'heap' structure.  It can even be beyond the current break point,
     which will cause crashes when we dereference it below (see
     bug#12242).  Evidently, the reason is bloc allocations done while
     use_relocatable_buffers was non-positive, because additional
     memory we get then is not recorded in the heaps we manage.  If
     bloc->heap records such a "heap", we cannot (and don't need to)
     update its records.  So we validate the 'heap' value by making
     sure it is one of the heaps we manage via the heaps linked list,
     and don't touch a 'heap' that isn't found there.  This avoids
     accessing memory we know nothing about.  */
  h = first_heap;
  while h != NIL_HEAP! ()
    {
      if heap == h
	{
	  break;
	}
      h = (*h).next;
    }

  if !h.is_null ()
    {
      /* Update the records of which blocs are in HEAP.  */
      if (*heap).first_bloc == bloc
	{
	  if !(*bloc).next.is_null () && (*(*bloc).next).heap == heap
	    {
	      (*heap).first_bloc = (*bloc).next;
	    }
	  else
	    {
	      (*heap).first_bloc = NIL_BLOC! ();
	      (*heap).last_bloc = NIL_BLOC! ();
	    }
	}
      if (*heap).last_bloc == bloc
	{
	  if !(*bloc).prev.is_null () && (*(*bloc).prev).heap == heap
	    {
	      (*heap).last_bloc = (*bloc).prev;
	    }
	  else
	    {
	      (*heap).first_bloc = NIL_BLOC! ();
	      (*heap).last_bloc = NIL_BLOC! ();
	    }
	}
    }

  relinquish ();
  free (bloc as *mut c_void);
}

/* Interface routines.  */

/* Obtain SIZE bytes of storage from the free pool, or the system, as
   necessary.  If relocatable blocs are in use, this means relocating
   them.  This function gets plugged into the GNU malloc's __morecore
   hook.

   We provide hysteresis, never relocating by less than extra_bytes.

   If we're out of memory, we should return zero, to imitate the other
   __morecore hook values - in particular, __default_morecore in the
   GNU malloc package.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe extern "C" fn r_alloc_sbrk (size: ptrdiff_t) -> *mut c_void
{
  let mut b: bloc_ptr;
  let mut address: *mut c_void;

  if r_alloc_initialized == 0
    {
      r_alloc_init ();
    }

  if use_relocatable_buffers <= 0
    {
      return real_morecore.unwrap () (size);
    }

  if size == 0
    {
      return virtual_break_value;
    }

  if size > 0
    {
      /* Allocate a page-aligned space.  GNU malloc would reclaim an
	 extra space if we passed an unaligned one.  But we could
	 not always find a space which is contiguous to the previous.  */
      let mut new_bloc_start: *mut c_void;
      let mut h: heap_ptr = first_heap;
      let mut get: size_t = PAGE_ROUNDUP! (size);

      address = PAGE_ROUNDUP! (virtual_break_value) as *mut c_void;

      /* Search the list upward for a heap which is large enough.  */
      while ((*h).end as *mut c_char) < (MEM_ROUNDUP! ((address as *mut c_char).add (get))
					  as *mut c_char)
	{
	  h = (*h).next;
	  if h == NIL_HEAP! ()
	    {
	      break;
	    }
	  address = PAGE_ROUNDUP! ((*h).start) as *mut c_void;
	}

      /* If not found, obtain more space.  */
      if h == NIL_HEAP! ()
	{
	  get += (extra_bytes + page_size) as size_t;

	  if obtain (address, get).is_null ()
	    {
	      return 0 as *mut c_void;
	    }

	  if first_heap == last_heap
	    {
	      address = PAGE_ROUNDUP! (virtual_break_value) as *mut c_void;
	    }
	  else
	    {
	      address = PAGE_ROUNDUP! ((*last_heap).start) as *mut c_void;
	    }
	  h = last_heap;
	}

      new_bloc_start = MEM_ROUNDUP! ((address as *mut c_char).add (get)) as *mut c_void;

      if (*first_heap).bloc_start < new_bloc_start
	{
	  /* This is no clean solution - no idea how to do it better.  */
	  if r_alloc_freeze_level != 0
	    {
	      return core::ptr::null_mut ();
	    }

	  /* There is a bug here: if the above obtain call succeeded, but the
	     relocate_blocs call below does not succeed, we need to free
	     the memory that we got with obtain.  */

	  /* Move all blocs upward.  */
	  if relocate_blocs (first_bloc, h, new_bloc_start) == 0
	    {
	      return 0 as *mut c_void;
	    }

	  /* Note that (char *) (h + 1) <= (char *) new_bloc_start since
	     get >= page_size, so the following does not destroy the heap
	     header.  */
	  b = last_bloc;
	  while b != NIL_BLOC! ()
	    {
	      if (*b).new_data != (*b).data
		{
		  memmove ((*b).new_data, (*b).data, (*b).size);
		}
	      (*b).data = (*b).new_data;
	      *(*b).variable = (*b).data;
	      b = (*b).prev;
	    }

	  (*h).bloc_start = new_bloc_start;

	  update_heap_bloc_correspondence (first_bloc, h);
	}
      if h != first_heap
	{
	  /* Give up managing heaps below the one the new
	     virtual_break_value points to.  */
	  (*first_heap).prev = NIL_HEAP! ();
	  (*first_heap).next = (*h).next;
	  (*first_heap).start = (*h).start;
	  (*first_heap).end = (*h).end;
	  (*first_heap).free = (*h).free;
	  (*first_heap).first_bloc = (*h).first_bloc;
	  (*first_heap).last_bloc = (*h).last_bloc;
	  (*first_heap).bloc_start = (*h).bloc_start;

	  if !(*first_heap).next.is_null ()
	    {
	      (*(*first_heap).next).prev = first_heap;
	    }
	  else
	    {
	      last_heap = first_heap;
	    }
	}

      memset (address, 0, size as size_t);
    }
  else /* size < 0 */
    {
      let mut excess: size_t
	= ((*first_heap).bloc_start as *mut c_char)
	.offset_from ((virtual_break_value as *mut c_char).offset (size)) as size_t;

      address = virtual_break_value;

      if r_alloc_freeze_level == 0 && excess > (2 * extra_bytes) as size_t
	{
	  excess -= extra_bytes as size_t;
	  (*first_heap).bloc_start
	    = MEM_ROUNDUP! (((*first_heap).bloc_start as *mut c_char)
			    .offset (- (excess as isize))) as *mut c_void;

	  relocate_blocs (first_bloc, first_heap, (*first_heap).bloc_start);

	  b = first_bloc;
	  while b != NIL_BLOC! ()
	    {
	      if (*b).new_data != (*b).data
		{
		  memmove ((*b).new_data, (*b).data, (*b).size);
		}
	      (*b).data = (*b).new_data;
	      *(*b).variable = (*b).data;
	      b = (*b).next;
	    }
	}

      if (virtual_break_value as *mut c_char).offset (size) < (*first_heap).start as *mut c_char
	{
	  /* We found an additional space below the first heap */
	  (*first_heap).start = (virtual_break_value as *mut c_char).offset (size) as *mut c_void;
	}
    }

  virtual_break_value = (address as *mut c_char).offset (size) as *mut c_void;
  break_value = if !last_bloc.is_null ()
    {
      ((*last_bloc).data as *mut c_char).add ((*last_bloc).size) as *mut c_void
    }
  else
    {
      (*first_heap).bloc_start
    };
  if size < 0
    {
      relinquish ();
    }

  return address;
}


/* Allocate a relocatable bloc of storage of size SIZE.  A pointer to
   the data is returned in *PTR.  PTR is thus the address of some variable
   which will use the data area.

   The allocation of 0 bytes is valid.
   In case r_alloc_freeze_level is set, a best fit of unused blocs could be
   done before allocating a new area.  Not yet done.

   If we can't allocate the necessary memory, set *PTR to zero, and
   return zero.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn r_alloc (ptr: *mut *mut c_void, size: size_t) -> *mut c_void
{
  let mut new_bloc: bloc_ptr;

  if r_alloc_initialized == 0
    {
      r_alloc_init ();
    }

  new_bloc = get_bloc (MEM_ROUNDUP! (size));
  if !new_bloc.is_null ()
    {
      (*new_bloc).variable = ptr;
      *ptr = (*new_bloc).data;
    }
  else
    {
      *ptr = 0 as *mut c_void;
    }

  return *ptr;
}

/* lisp.h 4437-4439: `#if defined REL_ALLOC && !defined SYSTEM_MALLOC` / `extern void
   refill_memory_reserve (void);` / `#endif` - C's own condition, which is this file's gate.  The
   port's `include/lisp.rs` does not carry the prototype (it sits inside that `#if`, so only this
   unit reaches it), so the unit declares it where C 934 calls it (P-72 rule 4 / P-88 ①, the
   `androidmenu.rs` shape).  `alloc.rs` 4557 defines it as a plain `unsafe fn` - a C `static`, so
   no symbol (its own comment) - which means a `rel_alloc` configuration's link would need that
   definition and this declaration to agree; the ledger's `## Owed` names it.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe extern "C" {
  pub fn refill_memory_reserve ();
}

/* Free a bloc of relocatable storage whose data is pointed to by PTR.
   Store 0 in *PTR to show there's no block allocated.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn r_alloc_free (ptr: *mut *mut c_void)
{
  let mut dead_bloc: bloc_ptr;

  if r_alloc_initialized == 0
    {
      r_alloc_init ();
    }

  dead_bloc = find_bloc (ptr);
  if dead_bloc == NIL_BLOC! ()
    {
      emacs_abort (); /* Double free? PTR not originally used to allocate?  */
    }

  free_bloc (dead_bloc);
  *ptr = 0 as *mut c_void;

  refill_memory_reserve ();
}

/* Given a pointer at address PTR to relocatable data, resize it to SIZE.
   Do this by shifting all blocks above this one up in memory, unless
   SIZE is less than or equal to the current bloc size, in which case
   do nothing.

   In case r_alloc_freeze_level is set, a new bloc is allocated, and the
   memory copied to it.  Not very efficient.  We could traverse the
   bloc_list for a best fit of free blocs first.

   Change *PTR to reflect the new bloc, and return this value.

   If more memory cannot be allocated, then leave *PTR unchanged, and
   return zero.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn r_re_alloc (ptr: *mut *mut c_void, size: size_t) -> *mut c_void
{
  let mut bloc: bloc_ptr;

  if r_alloc_initialized == 0
    {
      r_alloc_init ();
    }

  if (*ptr).is_null ()
    {
      return r_alloc (ptr, size);
    }
  if size == 0
    {
      r_alloc_free (ptr);
      return r_alloc (ptr, 0);
    }

  bloc = find_bloc (ptr);
  if bloc == NIL_BLOC! ()
    {
      emacs_abort (); /* Already freed? PTR not originally used to allocate?  */
    }

  if size < (*bloc).size
    {
      /* Wouldn't it be useful to actually resize the bloc here?  */
      /* I think so too, but not if it's too expensive...  */
      if (*bloc).size - MEM_ROUNDUP! (size) >= page_size as size_t
	  && r_alloc_freeze_level == 0
	{
	  resize_bloc (bloc, MEM_ROUNDUP! (size));
	  /* Never mind if this fails, just do nothing...  */
	  /* It *should* be infallible!  */
	}
    }
  else if size > (*bloc).size
    {
      if r_alloc_freeze_level != 0
	{
	  let mut new_bloc: bloc_ptr;
	  new_bloc = get_bloc (MEM_ROUNDUP! (size));
	  if !new_bloc.is_null ()
	    {
	      (*new_bloc).variable = ptr;
	      *ptr = (*new_bloc).data;
	      (*bloc).variable = core::ptr::null_mut ();
	    }
	  else
	    {
	      return core::ptr::null_mut ();
	    }
	}
      else
	{
	  if resize_bloc (bloc, MEM_ROUNDUP! (size)) == 0
	    {
	      return core::ptr::null_mut ();
	    }
	}
    }
  return *ptr;
}


/* C 995-1011: `#ifdef DOUG_LEA_MALLOC` ... `#endif /* emacs && DOUG_LEA_MALLOC */`.  C reaches
   `__morecore` through `<malloc.h>` in this arm (C 83's `#ifndef` declares it only when
   `DOUG_LEA_MALLOC` is undefined), so the arm declares it here; the two arms sit on opposite
   sides of one `#cfg`, so the crate root never carries both.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc",
	    feature = "doug_lea_malloc"))]
unsafe extern "C" {
  /// C 1003's `__morecore`, from `<malloc.h>` (glibc's).
  /// C: extern void *(*__morecore) (ptrdiff_t);
  pub static mut __morecore: Option<unsafe extern "C" fn (ptrdiff_t) -> *mut c_void>;
}

/* Reinitialize the morecore hook variables after restarting a dumped
   Emacs.  This is needed when using Doug Lea's malloc from GNU libc.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc",
	    feature = "doug_lea_malloc"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn r_alloc_reinit ()
{
  /* Only do this if the hook has been reset, so that we don't get an
     infinite loop, in case Emacs was linked statically.  */
  if __morecore != Some (r_alloc_sbrk)
    {
      real_morecore = __morecore;
      __morecore = Some (r_alloc_sbrk);
    }
}


/* C 1013-1105: `#ifdef DEBUG` ... `#endif /* DEBUG */`.  C 1015's `#include <assert.h>` has no
   mirror and nothing to declare (`assert` is a macro): the port spells every site with Rust's
   `assert!`, whose failure path is `abort` here (`-C panic=abort`, PORTING.md 2.5's neighbour
   rule).  `sbrk`, which C 1028 is the file's only use of, is declared at that site (P-88 ①).  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc", feature = "debug"))]
unsafe extern "C" {
  /// C 31's `<unistd.h>` declares it: `void *sbrk (intptr_t __delta);`
  /// C: void *sbrk (intptr_t __delta);
  pub fn sbrk (delta: ptrdiff_t) -> *mut c_void;
}

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc", feature = "debug"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn r_alloc_check ()
{
  let mut found: c_int = 0;
  let mut h: heap_ptr;
  let mut ph: heap_ptr = 0 as heap_ptr;
  let mut b: bloc_ptr;
  let mut pb: bloc_ptr = 0 as bloc_ptr;

  if r_alloc_initialized == 0
    {
      return;
    }

  assert! (!first_heap.is_null ());
  assert! ((*last_heap).end <= sbrk (0));
  assert! (first_heap as *mut c_void < (*first_heap).start);
  assert! ((*first_heap).start <= virtual_break_value);
  assert! (virtual_break_value <= (*first_heap).end);

  h = first_heap;
  while !h.is_null ()
    {
      assert! ((*h).prev == ph);
      assert! (PAGE_ROUNDUP! ((*h).end) as *mut c_void == (*h).end);
      /* C 1037-1041 is an `#if 0` region (its comment asks whether the heap start should
	 carry any sort of alignment) around
	 `assert ((void *) MEM_ROUNDUP (h->start) == h->start);` - dead text in every
	 configuration, recorded here instead of transcribed (the `alloc.rs` 176-186 reading;
	 `tools/extract-abi.py`'s `dead_branch_map` reads the region as dead).  */
      assert! (MEM_ROUNDUP! ((*h).bloc_start) as *mut c_void == (*h).bloc_start);
      assert! ((*h).start <= (*h).bloc_start && (*h).bloc_start <= (*h).end);

      if !ph.is_null ()
	{
	  assert! ((*ph).end < (*h).start);
	  assert! ((*h).start <= h as *mut c_void
		   && (h as *mut c_void).add (core::mem::size_of::<heap> ()) <= (*h).bloc_start);
	}

      if (*h).bloc_start <= break_value && break_value <= (*h).end
	{
	  found = 1;
	}

      ph = h;
      h = (*h).next;
    }

  assert! (found != 0);
  assert! (last_heap == ph);

  b = first_bloc;
  while !b.is_null ()
    {
      assert! ((*b).prev == pb);
      assert! (MEM_ROUNDUP! ((*b).data) as *mut c_void == (*b).data);
      assert! (MEM_ROUNDUP! ((*b).size) as size_t == (*b).size);

      ph = 0 as heap_ptr;
      h = first_heap;
      while !h.is_null ()
	{
	  if (*h).bloc_start <= (*b).data
	    && ((*b).data as *mut c_char).add ((*b).size) <= (*h).end as *mut c_char
	    {
	      break;
	    }
	  ph = h;
	  h = (*h).next;
	}

      assert! (!h.is_null ());

      if !pb.is_null ()
	&& ((*pb).data as *mut c_char).add ((*pb).size) != (*b).data as *mut c_char
	{
	  assert! (!ph.is_null () && (*b).data == (*h).bloc_start);
	  while !ph.is_null ()
	    {
	      if (*ph).bloc_start <= (*pb).data
		&& ((*pb).data as *mut c_char).add ((*pb).size) <= (*ph).end as *mut c_char
		{
		  assert! ((((*pb).data as *mut c_char).add ((*pb).size)).add ((*b).size)
			   > (*ph).end as *mut c_char);
		  break;
		}
	      else
		{
		  assert! (((*ph).bloc_start as *mut c_char).add ((*b).size)
			   > (*ph).end as *mut c_char);
		}
	      ph = (*ph).prev;
	    }
	}
      pb = b;
      b = (*b).next;
    }

  assert! (last_bloc == pb);

  if !last_bloc.is_null ()
    {
      assert! (((*last_bloc).data as *mut c_char).add ((*last_bloc).size) == break_value);
    }
  else
    {
      assert! ((*first_heap).bloc_start == break_value);
    }
}

/* Update the internal record of which variable points to some data to NEW.
   Used by buffer-swap-text in Emacs to restore consistency after it
   swaps the buffer text between two buffer objects.  The OLD pointer
   is checked to ensure that memory corruption does not occur due to
   misuse.  */
#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn r_alloc_reset_variable (old: *mut *mut c_void, new: *mut *mut c_void)
{
  let mut bloc: bloc_ptr = first_bloc;

  /* Find the bloc that corresponds to the data pointed to by pointer.
     find_bloc cannot be used, as it has internal consistency checks
     which fail when the variable needs resetting.  */
  while bloc != NIL_BLOC! ()
    {
      if (*bloc).data == *new
	{
	  break;
	}

      bloc = (*bloc).next;
    }

  if bloc == NIL_BLOC! () || (*bloc).variable != old
    {
      emacs_abort (); /* Already freed? OLD not originally used to allocate?  */
    }

  /* Update variable to point to the new location.  */
  (*bloc).variable = new;
}

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn r_alloc_inhibit_buffer_relocation (inhibit: c_int)
{
  if use_relocatable_buffers > 1
    {
      use_relocatable_buffers = 1;
    }
  if inhibit != 0
    {
      use_relocatable_buffers -= 1;
    }
  else if use_relocatable_buffers < 1
    {
      use_relocatable_buffers += 1;
    }
}


/***********************************************************************
			    Initialization
 ***********************************************************************/

/* Initialize various things for memory allocation.  */

#[cfg (all (feature = "rel_alloc", feature = "no_system_malloc"))]
unsafe fn r_alloc_init ()
{
  if r_alloc_initialized != 0
    {
      return;
    }
  r_alloc_initialized = 1;

  page_size = PAGE! ();
  #[cfg (feature = "no_system_malloc")]
  {
    real_morecore = __morecore;
    __morecore = Some (r_alloc_sbrk);

    last_heap = &raw mut heap_base;
    first_heap = last_heap;
    (*first_heap).prev = NIL_HEAP! ();
    (*first_heap).next = (*first_heap).prev;
    break_value = real_morecore.unwrap () (0);
    virtual_break_value = break_value;
    (*first_heap).bloc_start = virtual_break_value;
    (*first_heap).start = (*first_heap).bloc_start;
    if break_value == core::ptr::null_mut ()
      {
	emacs_abort ();
      }

    extra_bytes = PAGE_ROUNDUP! (50000) as c_int;
  }

  #[cfg (feature = "doug_lea_malloc")]
  {
    block_input ();
    mallopt (M_TOP_PAD, 64 * 4096);
    unblock_input ();
  }
  #[cfg (not (feature = "doug_lea_malloc"))]
  {
    #[cfg (feature = "no_system_malloc")]
    {
      /* Give GNU malloc's morecore some hysteresis so that we move all
	 the relocatable blocks much less often.  The number used to be
	 64, but alloc.c would override that with 32 in code that was
	 removed when SYNC_INPUT became the only input handling mode.
	 That code was conditioned on !DOUG_LEA_MALLOC, so the call to
	 mallopt above is left unchanged.  (Actually, I think there's no
	 system nowadays that uses DOUG_LEA_MALLOC and also uses
	 REL_ALLOC.)  */
      __malloc_extra_blocks = 32;
    }
  }

  #[cfg (feature = "no_system_malloc")]
  {
    (*first_heap).end = PAGE_ROUNDUP! ((*first_heap).start) as *mut c_void;

    /* The extra call to real_morecore guarantees that the end of the
       address space is a multiple of page_size, even if page_size is
       not really the page size of the system running the binary in
       which page_size is stored.  This allows a binary to be built on a
       system with one page size and run on a system with a smaller page
       size.  */
    real_morecore.unwrap () (((*first_heap).end as *mut c_char)
			     .offset_from ((*first_heap).start as *mut c_char));

    /* Clear the rest of the last page; this memory is in our address space
       even though it is after the sbrk value.  */
    /* Doubly true, with the additional call that explicitly adds the
       rest of that page to the address space.  */
    memset ((*first_heap).start, 0, ((*first_heap).end as *mut c_char)
	    .offset_from ((*first_heap).start as *mut c_char) as size_t);
    (*first_heap).bloc_start = (*first_heap).end;
    break_value = (*first_heap).bloc_start;
    virtual_break_value = break_value;
  }

  use_relocatable_buffers = 1;
}
