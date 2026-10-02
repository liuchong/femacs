// femacs: mirror of the frozen target's <string.h> (system header, P-26).
//
// regex_internal.h 26 includes <string.h>; the regex units use the byte-memory
// family (`memcpy`/`memmove`/`memset`/`memcmp`) and `strlen`/`strcmp`/`strchr`.


unsafe extern "C" {
  /* void *memcpy (void *dest, const void *src, size_t n);  */
  pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
  /* void *memmove (void *dest, const void *src, size_t n);  */
  pub fn memmove (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
  /* void *memset (void *s, int c, size_t n);  */
  pub fn memset (s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
  /* int memcmp (const void *s1, const void *s2, size_t n);  */
  pub fn memcmp (s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
  /* size_t strlen (const char *s);  */
  pub fn strlen (s: *const c_char) -> usize;
  /* int strcmp (const char *s1, const char *s2);  */
  pub fn strcmp (s1: *const c_char, s2: *const c_char) -> c_int;
  /* char *strchr (const char *s, int c);  */
  pub fn strchr (s: *const c_char, c: c_int) -> *mut c_char;
  /* char *strrchr (const char *s, int c);  */
  pub fn strrchr (s: *const c_char, c: c_int) -> *mut c_char;
  /* void *memchr (const void *s, int c, size_t n);  */
  pub fn memchr (s: *const c_void, c: c_int, n: usize) -> *mut c_void;
}
