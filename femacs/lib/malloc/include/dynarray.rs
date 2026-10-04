// femacs: mirror of lib/malloc/dynarray.h @ fd7afced403.
//
// The reference header is the shared definition of the dynarray facility.  Two readings of
// the gnulib wrapper `lib/dynarray.h` reach it, and they reach the *same* text: the wrapper's
// `#if defined DYNARRAY_STRUCT || ...` arm includes the generated `malloc/dynarray.gl.h`,
// which is this header plus a generated-marker line and minus the five `libc_hidden_proto`
// lines (verified with `diff` against build/emacs-ref/lib/malloc/dynarray.gl.h), and its
// `#else` arm includes this header directly.  The port therefore keeps the text here once
// and `femacs/lib/include/dynarray.rs` includes it for both arms.
//
// The five entries are declared under C's own names inside a module that this mirror
// glob-imports.  A unit that *defines* one of them (`lib/malloc/dynarray_*.c`) then shadows
// the import, which is C's "prototype + definition" and PORTING.md 0.1-10's module shape
// (a same-name item at the crate root would be E0428 for each defining unit).
//
// The generated header renames each entry (`__libc_dynarray_resize` -> `_gl_dynarray_resize`,
// lib/dynarray.h 252-256), and the frozen build's reference objects export the renamed
// symbols (`nm -g build/emacs-ref/lib/malloc/dynarray_resize.o` prints
// `T __gl_dynarray_resize`, which is the Mach-O spelling of the C symbol `_gl_dynarray_resize`),
// so each declaration carries C's renamed identifier through `#[link_name]`.  The defining
// units emit the same symbol through `#[unsafe(export_name = "_gl_dynarray_...")]`.
//
// `<stddef.h>` and `<string.h>` (C 97-98) need no mirror text here: the header names no
// function of its own, and the scalar aliases are the module below.  It is glob-imported so
// that the mirror is a compilable root on its own and an including unit's own
// `use std::ffi::…` shadows it (PORTING.md 0.1-10's rule for declarations; a unit that
// imports the same name explicitly and one that imports it through the glob are both valid).
mod stddef {
  pub use std::ffi::c_void;
}
use stddef::*;

/* struct dynarray_header { size_t used; size_t allocated; void *array; }  (C 100-105).
   The layout numbers are the frozen target's clang values (a probe in /tmp; there is no
   baseline row for this type in docs/baseline/layout, so they are recorded as measured):
   sizeof 24, alignof 8, used at 0, allocated at 8, array at 16.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dynarray_header {
  pub used: usize,
  pub allocated: usize,
  pub array: *mut c_void,
}
const _: () = assert! (std::mem::size_of::<dynarray_header> () == 24);
const _: () = assert! (std::mem::offset_of! (dynarray_header, allocated) == 8);
const _: () = assert! (std::mem::offset_of! (dynarray_header, array) == 16);

/* Marker used in the allocated member to indicate that an error was
   encountered.  (C 108-112).  */
#[inline(always)]
pub unsafe fn __dynarray_error_marker () -> usize
{
  return -1isize as usize;
}

/* Internal function.  See the has_failed function in
   dynarray-skeleton.c.  (C 114-121).  */
#[inline(always)]
pub unsafe fn __dynarray_error (list: *mut dynarray_header) -> bool
{
  return (*list).allocated == __dynarray_error_marker ();
}

/* struct dynarray_finalize_result { void *array; size_t length; } (C 145-150); the measured
   layout is sizeof 16, alignof 8, array at 0, length at 8.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dynarray_finalize_result {
  pub array: *mut c_void,
  pub length: usize,
}
const _: () = assert! (std::mem::size_of::<dynarray_finalize_result> () == 16);
const _: () = assert! (std::mem::offset_of! (dynarray_finalize_result, length) == 8);

/* C 122-166: the five internal entries.  Each keeps C's own name and carries the renamed
   symbol C's `#define`s give it (see the mirror note above).  `_Noreturn` (C 168) is the
   `-> !` of `__libc_dynarray_at_failure`.  */
pub mod libc_dynarray {
  use super::*;

  /* bool __libc_dynarray_emplace_enlarge (struct dynarray_header *, void *scratch,
                                          size_t element_size);  */
  unsafe extern "C" {
    #[link_name = "_gl_dynarray_emplace_enlarge"]
    pub fn __libc_dynarray_emplace_enlarge (list: *mut dynarray_header,
                                            scratch: *mut c_void,
                                            element_size: usize) -> bool;
  }
  /* bool __libc_dynarray_resize (struct dynarray_header *, size_t size, void *scratch,
                                  size_t element_size);  */
  unsafe extern "C" {
    #[link_name = "_gl_dynarray_resize"]
    pub fn __libc_dynarray_resize (list: *mut dynarray_header, size: usize,
                                   scratch: *mut c_void, element_size: usize) -> bool;
  }
  /* bool __libc_dynarray_resize_clear (struct dynarray_header *, size_t size,
                                        void *scratch, size_t element_size);  */
  unsafe extern "C" {
    #[link_name = "_gl_dynarray_resize_clear"]
    pub fn __libc_dynarray_resize_clear (list: *mut dynarray_header, size: usize,
                                         scratch: *mut c_void, element_size: usize) -> bool;
  }
  /* bool __libc_dynarray_finalize (struct dynarray_header *list, void *scratch,
                                    size_t element_size,
                                    struct dynarray_finalize_result *result);  */
  unsafe extern "C" {
    #[link_name = "_gl_dynarray_finalize"]
    pub fn __libc_dynarray_finalize (list: *mut dynarray_header, scratch: *mut c_void,
                                     element_size: usize,
                                     result: *mut dynarray_finalize_result) -> bool;
  }
  /* _Noreturn void __libc_dynarray_at_failure (size_t size, size_t index);  */
  unsafe extern "C" {
    #[link_name = "_gl_dynarray_at_failure"]
    pub fn __libc_dynarray_at_failure (size: usize, index: usize) -> !;
  }
}
use libc_dynarray::*;

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _DYNARRAY_H: bool = true;
