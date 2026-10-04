// femacs: port of lib/scratch_buffer.h @ fd7afced403.
//
// The reference header is the wrapper the units include: its C 62-102 text is gated on
// `#if 0` (the port's own declarations of the three allocation entries are dead text in
// C), it renames the glibc-imported functions to the `_gl_scratch_buffer_*` symbols
// (C 104-107), and it pulls the *definition* of `struct scratch_buffer` plus the inline
// `scratch_buffer_init`/`scratch_buffer_free`/`scratch_buffer_grow*`/`…_set_array_size`
// from the generated `<malloc/scratch_buffer.gl.h>` - the frozen build's
// `build/emacs-ref/lib/malloc/scratch_buffer.gl.h`, whose source is
// `lib/malloc/scratch_buffer.h`.  The port keeps that same split: the struct and the inline
// bodies below are the gl.h text, the `_gl_scratch_buffer_*` declarations are what the
// `__libc_…` macros name, and the header's six macros follow.
//
// `_GL_LIKELY`/`_GL_UNLIKELY` are `<builtin-expect>`'s hints (C 109-113): the port keeps
// them as the identity-plus-hint pair PORTING.md 2.6 allows, with the branch hint spelled
// once by `__builtin_expect`.  The macros come before their users, which PORTING.md 0.1-18
// requires of a mirror; the scalar names (`c_char`, `c_int`, `c_void`) come from the
// including unit's scope, as the other lib mirrors expect.

/// C: `__builtin_expect ((cond), 1)` (PORTING.md 2.6: a branch hint, not a layout hint).
macro_rules! __builtin_expect {
    ($cond:expr, $hint:expr) => { $cond };
}

/* Rely on __builtin_expect, as provided by the module 'builtin-expect'.  */
/* # define _GL_LIKELY(cond) __builtin_expect ((cond), 1)  */
macro_rules! _GL_LIKELY {
    ($cond:expr) => { __builtin_expect! ($cond, 1) };
}
/* # define _GL_UNLIKELY(cond) __builtin_expect ((cond), 0)  */
macro_rules! _GL_UNLIKELY {
    ($cond:expr) => { __builtin_expect! ($cond, 0) };
}

/* Avoid possible conflicts with symbols exported by the GNU libc.  */
/* #define __libc_scratch_buffer_grow _gl_scratch_buffer_grow  */
macro_rules! __libc_scratch_buffer_grow {
    ($buffer:expr) => { _gl_scratch_buffer_grow ($buffer) };
}
/* #define __libc_scratch_buffer_grow_preserve _gl_scratch_buffer_grow_preserve  */
macro_rules! __libc_scratch_buffer_grow_preserve {
    ($buffer:expr) => { _gl_scratch_buffer_grow_preserve ($buffer) };
}
/* #define __libc_scratch_buffer_set_array_size _gl_scratch_buffer_set_array_size  */
macro_rules! __libc_scratch_buffer_set_array_size {
    ($buffer:expr, $nelem:expr, $size:expr) => {
        _gl_scratch_buffer_set_array_size ($buffer, $nelem, $size)
    };
}

/* <malloc/scratch_buffer.gl.h>: Scratch buffer.  Must be initialized with
   scratch_buffer_init before its use.  */
/* struct scratch_buffer {
     void *data;
     size_t length;
     union { max_align_t __align; char __c[1024]; } __space;
   };  */
pub const SCRATCH_BUFFER_SPACE_SIZE: usize = 1024;

/// `<stddef.h>`'s `max_align_t` on the frozen target: `long double`, 8 bytes and 8-aligned
/// (`printf ("%zu %zu", sizeof (max_align_t), _Alignof (max_align_t))` -> `8 8`); only its
/// alignment matters to `__space`.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct MaxAlign {
  pub __opaque: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union scratch_buffer_space {
  /* max_align_t __align;  */
  pub __align: MaxAlign,
  /* char __c[1024];  */
  pub __c: [c_char; SCRATCH_BUFFER_SPACE_SIZE],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct scratch_buffer {
  /* void *data;  — Pointer to the beginning of the scratch area.  */
  pub data: *mut c_void,
  /* size_t length; — Allocated space at the data pointer, in bytes.  */
  pub length: usize,
  /* union { max_align_t __align; char __c[1024]; } __space;  */
  pub __space: scratch_buffer_space,
}

/* <stdlib.h>: the one entry scratch_buffer_free needs.  C's `free` is the generated
   `<stdlib.h>`'s renamed identifier (`build/emacs-ref/lib/stdlib.h` 883-884: `# undef free`
   / `# define free rpl_free`), and the frozen objects call it by that name
   (`nm build/emacs-ref/lib/malloc/scratch_buffer_grow.o` prints `U _rpl_free`), so the port
   spells `rpl_free` (PORTING.md 10.13, the rpl_mempcpy precedent).  */
unsafe extern "C" {
  /* void rpl_free (void *ptr);  */
  pub fn rpl_free (ptr: *mut c_void);
}

/* Grow *BUFFER by some arbitrary amount. … (the glibc-imported entry point)  */
/* bool __libc_scratch_buffer_grow (struct scratch_buffer *buffer);  */
unsafe extern "C" {
  pub fn _gl_scratch_buffer_grow (buffer: *mut scratch_buffer) -> bool;
}
/* Like scratch_buffer_grow, but preserve the old buffer contents on success.  */
unsafe extern "C" {
  pub fn _gl_scratch_buffer_grow_preserve (buffer: *mut scratch_buffer) -> bool;
}
/* Grow *BUFFER so that it can store at least NELEM elements of SIZE bytes.  */
unsafe extern "C" {
  pub fn _gl_scratch_buffer_set_array_size (buffer: *mut scratch_buffer, nelem: usize,
                                            size: usize) -> bool;
}

/* Initializes *BUFFER so that BUFFER->data points to BUFFER->__space
   and BUFFER->length reflects the available space.  */
#[inline(always)]
pub unsafe fn scratch_buffer_init (buffer: *mut scratch_buffer)
{
  (*buffer).data = &mut (*buffer).__space.__c as *mut [c_char] as *mut c_void;
  (*buffer).length = std::mem::size_of_val (&(*buffer).__space);
}

/* Deallocates *BUFFER (if it was heap-allocated).  */
#[inline(always)]
pub unsafe fn scratch_buffer_free (buffer: *mut scratch_buffer)
{
  if (*buffer).data != &mut (*buffer).__space.__c as *mut [c_char] as *mut c_void
    { rpl_free ((*buffer).data); }
}

/* Alias for __libc_scratch_buffer_grow.  */
#[inline(always)]
pub unsafe fn scratch_buffer_grow (buffer: *mut scratch_buffer) -> bool
{
  return _GL_LIKELY! (_gl_scratch_buffer_grow (buffer));
}

/* Alias for __libc_scratch_buffer_grow_preserve.  */
#[inline(always)]
pub unsafe fn scratch_buffer_grow_preserve (buffer: *mut scratch_buffer) -> bool
{
  return _GL_LIKELY! (_gl_scratch_buffer_grow_preserve (buffer));
}

/* Alias for __libc_scratch_set_array_size.  */
#[inline(always)]
pub unsafe fn scratch_buffer_set_array_size (buffer: *mut scratch_buffer, nelem: usize,
                                             size: usize) -> bool
{
  return _GL_LIKELY! (_gl_scratch_buffer_set_array_size (buffer, nelem, size));
}

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _GL_SCRATCH_BUFFER_H: bool = true;
