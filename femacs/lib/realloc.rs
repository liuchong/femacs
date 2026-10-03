// femacs: port of lib/realloc.c @ fd7afced403.
/* written by Jim Meyering and Bruno Haible */

/* emacs/lib/realloc.c: the two `#define`s C 21-23 puts before <config.h> are the constants
   below (the timespec.rs `_GL_*_INLINE` shape), and `malloc`/`realloc` are declared here
   because `_GL_USE_STDLIB_ALLOC` is what makes C's <stdlib.h> name the system ones.  C 25's
   `<errno.h>` and `<stdckdint.h>` are only used by the dead arms.  */
use std::ffi::{c_int, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");

pub const _GL_USE_STDLIB_ALLOC: c_int = 1;
pub const _GL_REALLOC_INLINE: () = _GL_EXTERN_INLINE;

/* C's `<stdlib.h>`: the system allocator `_GL_USE_STDLIB_ALLOC` selected.  */
unsafe extern "C" {
  pub fn malloc (n: usize) -> *mut c_void;
  pub fn realloc (p: *mut c_void, n: usize) -> *mut c_void;
  /* C's `<stdlib.h>` declares `abort` too; only the dead arm below calls it.  */
  pub fn abort () -> !;
}

/* Change the size of an allocated block of memory P to N bytes,
   with error checking.  If P is NULL, use malloc.  Otherwise if N is zero,
   free P and return NULL.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_realloc (p: *mut c_void, n: usize) -> *mut c_void
{
  let mut n1: usize = n;

  if n == 0
    {
      /* C 30-45's `#if NEED_SANITIZED_REALLOC` arm is dead in the frozen configuration
	 (docs/baseline/config/darwin-arm64.macros.tsv 1404 undef); the text is kept under the
	 feature that names the non-frozen side (P-51).  */
      #[cfg (feature = "need_sanitized_realloc")]
      {
	if !p.is_null ()
	  { abort (); }
      }

      /* C 60-62's `# if !HAVE_REALLOC_0_NONNULL` arm (macros.tsv 919 is 1).  */
      #[cfg (feature = "no_realloc_0_nonnull")]
      { n1 = 1; }
    }

  /* C 64-71's `# if !HAVE_MALLOC_PTRDIFF` arm (macros.tsv 859 is 1).  */
  #[cfg (feature = "no_malloc_ptrdiff")]
  {
    let signed_n: isize;
    let (v, overflowed) = n.overflowing_add (0);
    signed_n = v as isize;
    if overflowed
      {
	*errno () = ENOMEM;
	return std::ptr::null_mut ();
      }
  }

  let result: *mut c_void = realloc (p, n1);

  /* C 73-77's `# if !HAVE_REALLOC_POSIX` arm (macros.tsv 920 is 1).  */
  #[cfg (feature = "no_realloc_posix")]
  {
    if result.is_null ()
      { *errno () = ENOMEM; }
  }

  return result;
}
