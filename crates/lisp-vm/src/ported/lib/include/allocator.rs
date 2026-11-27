// femacs: mirror of lib/allocator.h @ fd7afced403 ; PORTING.md 0.1.
//
// `struct allocator` is four function pointers (the C marks none of them with GCC's `malloc`
// attribute, which is why the port keeps plain `Option<unsafe extern "C" fn …>` fields) and one
// `extern` object whose definition is `lib/allocator.rs`.  That definition shadows the
// declaration through the whole-import module below (PORTING.md 0.1-10), which is what keeps a
// unit that includes both from seeing two `stdlib_allocator`s.

// The include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const _GL_ALLOCATOR_H: bool = true;

/// An object describing a memory allocator family.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct allocator
{
  /* Call ALLOCATE to allocate memory, like 'malloc'.  On failure ALLOCATE
     should return NULL, though not necessarily set errno.  When given
     a zero size it may return NULL even if successful.  */
  pub allocate: Option<unsafe extern "C-unwind" fn (size: usize) -> *mut core::ffi::c_void>,

  /* If nonnull, call REALLOCATE to reallocate memory, like 'realloc'.
     On failure REALLOCATE should return NULL, though not necessarily set
     errno.  */
  pub reallocate: Option<unsafe extern "C-unwind" fn (ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>,

  /* Call FREE to free memory, like 'free'.  */
  pub free: Option<unsafe extern "C-unwind" fn (ptr: *mut core::ffi::c_void)>,

  /* If nonnull, call DIE (SIZE) if MALLOC (SIZE) or REALLOC (…, SIZE) fails.  DIE should not
     return.  */
  pub die: Option<unsafe extern "C-unwind" fn (size: usize)>,
}

/// C: extern struct allocator const stdlib_allocator
pub mod allocator_decls {
  use super::*;

  unsafe extern "C-unwind" {
    pub static stdlib_allocator: allocator;
  }
}
pub use self::allocator_decls::*;
