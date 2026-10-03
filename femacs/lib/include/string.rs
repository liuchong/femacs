// femacs: mirror of the frozen target's <string.h> (system header, P-26).
//
// regex_internal.h 26 includes <string.h>; the regex units use the byte-memory
// family (`memcpy`/`memmove`/`memset`/`memcmp`) and `strlen`/`strcmp`/`strchr`.


/* gnulib's <string.h> template (lib/string.in.h 424-440): `memeq`, defined when the system
   header has not got it (it has not).  C's `_GL_MEMEQ_INLINE bool memeq (void const *__s1,
   void const *__s2, size_t __n)` returns `!memcmp (…)`; Rust's `!` on a `c_int` is bitwise,
   so the port writes the same value as `== 0`.  The frozen target's object exports
   `_memeq` (lib/memeq.rs re-exports this definition, PORTING.md P-81).  */
#[inline(always)]
pub unsafe fn memeq (s1: *const c_void, s2: *const c_void, n: usize) -> bool
{
  return memcmp (s1, s2, n) == 0;
}

/* gnulib's <string.h> template (lib/string.in.h 813-828): `streq`, defined when the system
   header has not got it (it has not).  C's `_GL_STREQ_INLINE bool streq (char const *__s1,
   char const *__s2)` returns `!strcmp (…)`, written as `== 0` for the same reason as
   `memeq`; the frozen target's object exports `_streq` (lib/streq.rs re-exports it, P-81).
   This was a `macro_rules!` before: C's frozen configuration defines the *function*, and
   the reference object's `T _streq` is the evidence (tools/check-symbols.py).  */
#[inline(always)]
pub unsafe fn streq (s1: *const c_char, s2: *const c_char) -> bool
{
  return strcmp (s1, s2) == 0;
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
  /* char *stpcpy (char *dest, const char *src); — the frozen configuration has
     HAVE_STPCPY=1 (docs/baseline/config/darwin-arm64.macros.tsv 609), so the system function is
     the one doc.c's `stpcpy (name, sibling_etc)` calls (lib/stpcpy.c is not compiled).  */
  pub fn stpcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  /* char *strchr (const char *s, int c);  */
  pub fn strchr (s: *const c_char, c: c_int) -> *mut c_char;
  /* size_t strspn (const char *s, const char *accept);  */
  pub fn strspn (s: *const c_char, accept: *const c_char) -> usize;
  /* char *strrchr (const char *s, int c);  */
  pub fn strrchr (s: *const c_char, c: c_int) -> *mut c_char;
  /* errno_t memset_s (void *s, rsize_t smax, int c, rsize_t n); — the frozen target's
     C11 Annex K entry, which lib/memset_explicit.c calls on the `HAVE_MEMSET_S` arm
     (`errno_t` is `int` and `rsize_t` is `size_t` there, measured with clang:
     `printf ("%zu %zu", sizeof (errno_t), sizeof (rsize_t))` -> `4 8`).  */
  pub fn memset_s (s: *mut c_void, smax: usize, c: c_int, n: usize) -> c_int;
  /* void *memchr (const void *s, int c, size_t n);  */
  pub fn memchr (s: *const c_void, c: c_int, n: usize) -> *mut c_void;
  /* void *memmem (const void *haystack, size_t haystack_len, const void *needle,
                   size_t needle_len); — the frozen configuration has HAVE_MEMMEM=1
     (macOS declares it under _DARWIN_C_SOURCE, which config.h defines), so the system
     function is the one lib-src/make-fingerprint.c's search loop calls; lib/memmem.c is not
     compiled on this target.  */
  pub fn memmem (haystack: *const c_void, haystack_len: usize, needle: *const c_void,
                 needle_len: usize) -> *mut c_void;
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
