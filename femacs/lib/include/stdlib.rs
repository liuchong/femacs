// femacs: mirror of the frozen target's <stdlib.h> (system header, P-26).
//
// gnulib has no `stdlib.in.h` wrapper in this build (the reference includes the system
// header), so this file carries what the ported units use.  So far that is `strtod`
// (bignum.c's `mpz_get_d_rounded`); the allocator family lives in lisp.rs, where lisp.h
// declares it.

unsafe extern "C" {
  /* long int strtol (const char *nptr, char **endptr, int base);  */
  pub fn strtol (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
  /* double strtod (const char *nptr, char **endptr);  */
  pub fn strtod (nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
  /* void *malloc (size_t size);  */
  pub fn malloc (size: usize) -> *mut c_void;
  /* void *realloc (void *ptr, size_t size);  */
  pub fn realloc (ptr: *mut c_void, size: usize) -> *mut c_void;
  /* void *calloc (size_t nmemb, size_t size);  */
  pub fn calloc (nmemb: usize, size: usize) -> *mut c_void;
  /* void free (void *ptr);  */
  pub fn free (ptr: *mut c_void);
}
