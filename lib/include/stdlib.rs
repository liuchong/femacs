// femacs: mirror of the frozen target's <stdlib.h> (system header, P-26).
//
// gnulib has no `stdlib.in.h` wrapper in this build (the reference includes the system
// header), so this file carries what the ported units use.  So far that is `strtod`
// (bignum.c's `mpz_get_d_rounded`); the allocator family lives in lisp.rs, where lisp.h
// declares it.

// [U1-MODULE-MIRROR]
use super::*;
/* `<stdlib.h>`'s `#define EXIT_SUCCESS 0` / `EXIT_FAILURE 1` (measured with clang).  */
pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;

unsafe extern "C" {
  /* long int strtol (const char *nptr, char **endptr, int base);  */
  pub fn strtol (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
  /* double strtod (const char *nptr, char **endptr);  */
  pub fn strtod (nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
  /* void *malloc (size_t size);  */
  pub fn malloc (size: usize) -> *mut c_void;
  /* void *realloc (void *ptr, size_t size);  -  the entry gnulib's generated header leaves the
     plain `realloc` name on when it does not redirect the allocator (below).  */
  #[cfg (not (feature = "replace_realloc_for_realloc_posix"))]
  pub fn realloc (ptr: *mut c_void, size: usize) -> *mut c_void;
  /* void *realloc (void *ptr, size_t size);  -  the system entry the redirected `realloc`
     below calls; the generated header's `#undef realloc` is why the two names differ.  */
  #[cfg (feature = "replace_realloc_for_realloc_posix")]
  #[link_name = "realloc"]
  pub fn realloc_system (ptr: *mut c_void, size: usize) -> *mut c_void;
  /* void *calloc (size_t nmemb, size_t size);  */
  pub fn calloc (nmemb: usize, size: usize) -> *mut c_void;
  /* void free (void *ptr);  */
  pub fn free (ptr: *mut c_void);
  /* int posix_memalign (void **memptr, size_t alignment, size_t size);  - the entry alloc.c's
     `#elif defined HAVE_POSIX_MEMALIGN` arm calls (C 982); bionic's header 74 declares it and
     every API level has it, while `aligned_alloc` only arrived in API 28 (status 4.713).  */
  pub fn posix_memalign (memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
}

/* The two entries lib/nproc.c needs, added to this mirror (`getenv` for the OMP variables,
   `strtoul` for `parse_omp_threads`).  The declaration lives here and nowhere else: a unit
   that includes this mirror must not also declare these names - that is a second
   declaration of one name in one compilation unit (E0428), which is what lib/getopt.rs's
   own `getenv` was (removed when this entry landed, PORTING.md 0.1-3's single home for the
   system-header declarations, the sys_stat.in.rs `chmod` precedent).  */
unsafe extern "C" {
  /* char *getenv (const char *name);  */
  pub fn getenv (name: *const c_char) -> *mut c_char;
  /* unsigned long int strtoul (const char *nptr, char **endptr, int base);  */
  pub fn strtoul (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
}

/* C: gnulib's generated `<stdlib.h>` 1461-1482.  With `@REPLACE_REALLOC_FOR_REALLOC_POSIX@`
   nonzero it defines `_GL_INLINE_RPL_REALLOC`, then `#undef realloc` / `#define realloc
   rpl_realloc`, whose body is `realloc (ptr, size ? size : 1)`; `lib/realloc.c` 60-62's
   non-inline twin sets `n1 = 1` for the same reason.  The frozen table has
   `HAVE_REALLOC_POSIX` = 1 with `HAVE_REALLOC_0_NONNULL` undefined, and the device measured the
   difference: `realloc (NULL, 0)` hands back a pointer while `realloc (p, 0)` on a live block
   returns NULL, so `shrink_regexp_cache`'s `xrealloc (cp->buf.buffer, cp->buf.used)` reached
   `memory_full (0)` in the first GC of `loadup.el` (status 4.787).  */
#[cfg (feature = "replace_realloc_for_realloc_posix")]
pub unsafe extern "C" fn realloc (ptr: *mut c_void, size: usize) -> *mut c_void
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  return realloc_system (ptr, if size == 0 { 1 } else { size });

  }
}
