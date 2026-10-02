// femacs: mirror of the frozen target's <stdlib.h> (system header, P-26).
//
// gnulib has no `stdlib.in.h` wrapper in this build (the reference includes the system
// header), so this file carries what the ported units use.  So far that is `strtod`
// (bignum.c's `mpz_get_d_rounded`); the allocator family lives in lisp.rs, where lisp.h
// declares it.

unsafe extern "C" {
  /* double strtod (const char *nptr, char **endptr);  */
  pub fn strtod (nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
}
