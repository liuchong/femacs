// femacs: mirror of the frozen target's <string.h> (system header, P-26).
//
// regex_internal.h 26 includes <string.h>; the regex units use the byte-memory
// family (`memcpy`/`memmove`/`memset`/`memcmp`) and `strlen`/`strcmp`/`strchr`.


/* gnulib's <string.h> template (lib/string.in.h 816-828) also defines `streq` when the system
   header has not got it (it has not): `#define streq(a,b) (strcmp ((a), (b)) == 0)`.  The port
   keeps it here rather than in a mirror of its own, because there is no separate reference
   header for it (lib/streq.c is the *unit*, already ported as femacs/lib/streq.rs).  */
macro_rules! streq {
  ($a:expr, $b:expr) => { strcmp ($a, $b) == 0 };
}

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
  /* int strncmp (const char *s1, const char *s2, size_t n);  */
  pub fn strncmp (s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
  /* char *strcpy (char *dest, const char *src);  */
  pub fn strcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  /* char *strchr (const char *s, int c);  */
  pub fn strchr (s: *const c_char, c: c_int) -> *mut c_char;
  /* char *strrchr (const char *s, int c);  */
  pub fn strrchr (s: *const c_char, c: c_int) -> *mut c_char;
  /* void *memchr (const void *s, int c, size_t n);  */
  pub fn memchr (s: *const c_void, c: c_int, n: usize) -> *mut c_void;
}

/* string.in.h 1250-1305: gnulib's `strnul` extension, whose definition is the header's
   `_GL_STRNUL_INLINE` function (the lib/strnul.c wrapper unit only carries the qualifier,
   PORTING.md 10.12).  */
/* _GL_STRNUL_INLINE const char *_gl_strnul (const char *string);  */
#[inline(always)]
pub unsafe fn _gl_strnul (string: *const c_char) -> *const c_char
{
  return string.add (strlen (string));
}

/* string.in.h 1302-1305: `# define strnul(s) _Generic (1 ? (s) : (void *) 99, void * :
   (char *) _gl_strnul (s), const void * : _gl_strnul (s))`.  The port writes the `void *`
   arm (the one a `char *` argument selects) with the cast C's arm spells; the const arm is
   the same call without it.  */
macro_rules! strnul {
  ($s:expr) => { _gl_strnul ($s) as *mut c_char };
}
