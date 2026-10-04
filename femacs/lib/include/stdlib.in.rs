// femacs: port of lib/stdlib.in.h @ fd7afced403 (gnulib's <stdlib.h>; PORTING.md P-26).
//
// **The criterion is the generated header, and it exists.**  `build/emacs-ref/lib/stdlib.h`
// (94,510 bytes, 2,581 lines) is generated from this template by `gnulib.mk`'s rule, and it is
// not pre-digested: every `@PLACEHOLDER@` becomes a literal number there, which is what decides
// each arm.  The diff that names them all is
//
//     $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/stdlib.in.h) build/emacs-ref/lib/stdlib.h
//     (1,258 lines; most of it is the `c++defs.h`/`arg-nonnull.h`/`warn-on-use.h` text the build
//      inlines, and the `@GNULIB_FOO@` lines near the end are the build's own legend)
//
// `tools/extract-abi.py` cannot read those numbers, so its item list is the union of every arm
// (P-66: `check-cond-eval.py lib/stdlib.in.h` -> `393 conditions, 169 unknown, 218
// disagreements [NOT VERIFIED]`).  The arms themselves are decided by the **frozen
// preprocessor**, measured on the generated header:
//
//     $ clang -E -dM -std=gnu23 -include build/emacs-ref/src/config.h \
//         -I build/emacs-ref/lib -I build/emacs-ref/src -I emacs/src -I emacs/lib \
//         build/emacs-ref/lib/stdlib.h | grep '^#define '
//     #define free rpl_free            <-- the only two live `#define X rpl_X` renames
//     #define realpath rpl_realpath
//     #define MB_CUR_MAX __mb_cur_max  #define RAND_MAX 0x7fffffff  (the system's spellings)
//
//     $ clang -E -std=gnu23 … build/emacs-ref/lib/stdlib.h | grep -c '\bX\b'   (per function name)
//     -> 42 of the 45 function names are *declared* by the frozen translation unit (the SDK's
//        `<stdlib.h>`/`<_stdlib.h>` or gnulib's own `_GL_FUNCDECL_SYS`), and 7 template names
//        are absent from it altogether (see the ledger's table).
//
// **The system half is `stdlib.rs`** - C 38/50's `# @INCLUDE_NEXT@ @NEXT_STDLIB_H@`, i.e. the
// frozen `<stdlib.h>` - and `attribute.rs` carries the `_GL_ATTRIBUTE_*` names C 142-215's
// fallback guards define (each guarded by `#ifndef`, so `lib/attribute.h`'s own text wins when a
// unit includes it first).  Both are `include!`s, the `dirent.in.rs`/`errno.in.rs` relation.
//
// **What this mirror carries.**  The template adds to the system header: the two exit constants
// (`stdlib.rs` answers `EXIT_SUCCESS`/`EXIT_FAILURE`; this mirror does not repeat them), gnulib's
// `RAND_MAX` value, the `MB_CUR_MAX` macro (whose template arm is dead - see below), the three
// `_GL_*_INLINE`/marker constants, the twelve `GNULIB_defined_*` witnesses (carried as the values
// C's own arms set, the sys_types.in.rs/endian.in.rs shape), the `free`/`realpath` **renames**
// (live, so a unit that includes this header calls `rpl_free`/`rpl_realpath` - the port spells
// each as a redirect macro over the renamed declaration, PORTING.md P-81), and the live
// declarations of the 34 function names the system half does not already declare.
//
// **What it does not carry, and why (the ledger's table has the line numbers).**  Eight names
// live in arms the frozen configuration does not take, so the frozen header declares nothing for
// them and neither does this mirror: `reallocarray`, `initstate_r`, `random_r`, `srandom_r`,
// `setstate_r` (all inside `#if @GNULIB_RANDOM_R@`/`@GNULIB_REALLOCARRAY@` = 0, and no macOS
// header or libSystem export has them either - measured with `nm -gU /usr/lib/libSystem.B.dylib`
// and a grep of the SDK), `print_stack_trace` (C 1650-1661's `#if @GNULIB_STACK_TRACE@` = 0),
// and the template's own two definitions `_gl_MB_CUR_MAX` (C 761) and `rpl_realloc` (C 1468),
// whose gates `@REPLACE_MB_CUR_MAX@`/`@REPLACE_REALLOC_FOR_REALLOC_POSIX@` are 0.  The `struct
// random_data` type (C 84-92) is in the same `#if 0` arm.  P-66 rule 2 asks for the boundary and
// the line numbers instead of a transcription, so those eight names are recorded in
// `docs/ports/excluded-macros.txt` (the ledger lists the rows to add) and the type is the one
// unchecked box this ledger keeps.

include!("stdlib.rs");        // C 38/50: `# @INCLUDE_NEXT@ @NEXT_STDLIB_H@` - the system half
include!("attribute.rs");     // C 142-215: the `_GL_ATTRIBUTE_*` spellings

/// C 45: `#define _@GUARD_PREFIX@_STDLIB_H` - the split include guard, whose substitution is
/// `_GL_STDLIB_H`; the template's own spelling reads as `_` to the extractor, which
/// `docs/ports/excluded-macros.txt` already records (the `string.in.rs` row).
pub const _GL_STDLIB_H: c_int = 1;

/// C 1210: `# define RAND_MAX 0x7fffffff` - gnulib's own value, and the same one the frozen
/// `<stdlib.h>` spells (`_stdlib.h` 125, measured with `clang -dM`).
/// C: #  define RAND_MAX 0x7fffffff
pub const RAND_MAX: c_int = 0x7fffffff;

/* ---- C 755-780: `MB_CUR_MAX` ---------------------------------------------------------------
   The template's own arm is dead: its gate is C 758's `#if @REPLACE_MB_CUR_MAX@`, which the
   generated header substitutes to `#if 0` (line 1348), and the definition of `_gl_MB_CUR_MAX`
   (C 761-775) with it.  The *live* macro is the system's, measured with the frozen preprocessor:
       $ clang -E -std=gnu23 … #include <stdlib.h> + `MB_CUR_MAX`  ->  `__mb_cur_max`
   i.e. the libc variable `_stdlib.h` 138 declares as `extern int __mb_cur_max;` (the
   `_USE_EXTENDED_LOCALES_` arm's `___mb_cur_max ()` call is *not* the frozen one).  The port
   declares that variable and spells the macro C's own `__mb_cur_max`; the one
   `docs/ports/macro-body-notes.tsv` row this needs (the template's recorded body is the dead
   arm's `_gl_MB_CUR_MAX ()`) is in the ledger.  */
unsafe extern "C" {
  /// C: `<stdlib.h>` (`_stdlib.h` 138): `extern int __mb_cur_max;`
  pub static mut __mb_cur_max: c_int;
}
/// C: `#define MB_CUR_MAX __mb_cur_max` - the frozen `<stdlib.h>`'s live spelling of the name.
macro_rules! MB_CUR_MAX { () => { unsafe { __mb_cur_max } }; }

/// C 129: `# define _GL_STDLIB_INLINE _GL_INLINE` - the C99-inline qualifier.  The port spells
/// the qualifier `#[inline(always)]` on the members that use it and keeps the name as the
/// *macro* (the `_GL_STDC_*_INLINE` shape, `stdbit.in.rs`): `lib/stdlib.rs`, the unit that emits
/// the external definition, holds the same C name as a `const`, and a macro and a const are
/// different namespaces, so the two coexist where two consts would be `E0428`.
/// C: # define _GL_STDLIB_INLINE _GL_INLINE
macro_rules! _GL_STDLIB_INLINE { () => { _GL_INLINE }; }
/// C 132: `# define _GL_REALLOC_INLINE _GL_INLINE` - the same qualifier for `rpl_realloc`.
/// C: # define _GL_REALLOC_INLINE _GL_INLINE
macro_rules! _GL_REALLOC_INLINE { () => { _GL_INLINE }; }

/* ---- C 452-1935: the `GNULIB_defined_*` witnesses ------------------------------------------
   Each is a marker gnulib's own `# ifndef GNULIB_defined_*` guards read.  Eleven of them (and
   C 1463's `_GL_INLINE_RPL_REALLOC`) sit in arms the frozen configuration does not take (`#if 0`
   in the generated header), and the port carries each as the value C's preprocessor defines in
   that arm - the value is a fact about the header, and the port has no second inclusion for the
   guard to protect against (`sys_types.in.rs` 103-132 and `endian.in.rs` 36-49 carry theirs the
   same way).  Only `GNULIB_defined_canonicalize_file_name` (C 478) is live.  */
/// C 773: `#   define GNULIB_defined_MB_CUR_MAX 1` (in the dead `@REPLACE_MB_CUR_MAX@` arm).
/// C: #  define GNULIB_defined_MB_CUR_MAX 1
pub const GNULIB_defined_MB_CUR_MAX: c_int = 1;
/// C 1661: `#   define GNULIB_defined_print_stack_trace 1` (dead `@GNULIB_STACK_TRACE@` arm).
/// C: #   define GNULIB_defined_print_stack_trace 1
pub const GNULIB_defined_print_stack_trace: c_int = 1;
/// C 1167: `#  define GNULIB_defined_qsort_r_fn_types 1` (dead `@GNULIB_QSORT_R@` arm).
/// C: #  define GNULIB_defined_qsort_r_fn_types 1
pub const GNULIB_defined_qsort_r_fn_types: c_int = 1;
/// C 1669: `#  define GNULIB_defined_strtod_function 1` (dead `@REPLACE_STRTOD@` arm).
/// C: #  define GNULIB_defined_strtod_function 1
pub const GNULIB_defined_strtod_function: c_int = 1;
/// C 1700: `#  define GNULIB_defined_strtof_function 1` (dead `@REPLACE_STRTOF@` arm).
/// C: #  define GNULIB_defined_strtof_function 1
pub const GNULIB_defined_strtof_function: c_int = 1;
/// C 1767: `#  define GNULIB_defined_strtol_function 1` (dead `@REPLACE_STRTOL@` arm).
/// C: #  define GNULIB_defined_strtol_function 1
pub const GNULIB_defined_strtol_function: c_int = 1;
/// C 1731: `#  define GNULIB_defined_strtold_function 1` (dead `@REPLACE_STRTOLD@` arm).
/// C: #  define GNULIB_defined_strtold_function 1
pub const GNULIB_defined_strtold_function: c_int = 1;
/// C 1809: `#  define GNULIB_defined_strtoll_function 1` (dead `@REPLACE_STRTOLL@` arm).
/// C: #  define GNULIB_defined_strtoll_function 1
pub const GNULIB_defined_strtoll_function: c_int = 1;
/// C 1848: `#  define GNULIB_defined_strtoul_function 1` (dead `@REPLACE_STRTOUL@` arm).
/// C: #  define GNULIB_defined_strtoul_function 1
pub const GNULIB_defined_strtoul_function: c_int = 1;
/// C 1890: `#  define GNULIB_defined_strtoull_function 1` (dead `@REPLACE_STRTOULL@` arm).
/// C: #  define GNULIB_defined_strtoull_function 1
pub const GNULIB_defined_strtoull_function: c_int = 1;
/// C 101: `#   define GNULIB_defined_struct_random_data 1` (dead `@GNULIB_RANDOM_R@` arm).
/// C: #   define GNULIB_defined_struct_random_data 1
pub const GNULIB_defined_struct_random_data: c_int = 1;
/// C 1463: `#   define _GL_INLINE_RPL_REALLOC 1` (dead `@REPLACE_REALLOC_FOR_REALLOC_POSIX@` arm);
/// the marker the `rpl_realloc` definition below it is guarded by.
/// C: #   define _GL_INLINE_RPL_REALLOC 1
pub const _GL_INLINE_RPL_REALLOC: c_int = 1;

/// C 478: `#  define GNULIB_defined_canonicalize_file_name (!@HAVE_CANONICALIZE_FILE_NAME@ ||
/// @REPLACE_CANONICALIZE_FILE_NAME@)` - **live** (measured: the frozen macro table carries it as
/// `(!0 || 0)`, because `HAVE_CANONICALIZE_FILE_NAME` is `undef` and `REPLACE_*` is 0), and the
/// one marker of this file whose body C writes as an expression of two *placeholder* names.  The
/// port carries both readings: the macro keeps the template's own tokens - the
/// `sys_types.in.rs` 109 `_GL_WINDOWS_STAT_INODES` shape, a body nothing expands - and the const
/// beside it is the frozen value, which a unit can actually read (a macro and a const are
/// different namespaces, the `_GL_MEMEQ_INLINE` rule).
/// C: #  define GNULIB_defined_canonicalize_file_name (!@HAVE_CANONICALIZE_FILE_NAME@ || @REPLACE_CANONICALIZE_FILE_NAME@)
macro_rules! GNULIB_defined_canonicalize_file_name {
  () => { (! HAVE_CANONICALIZE_FILE_NAME || REPLACE_CANONICALIZE_FILE_NAME) };
}
/// `GNULIB_defined_canonicalize_file_name`'s frozen value: `(!0 || 0)` is 1.
pub const GNULIB_defined_canonicalize_file_name: c_int = 1;

/* ---- C 280-1935: the live declarations -----------------------------------------------------
   Every name here is *declared* by the frozen translation unit, measured by preprocesssing the
   generated header (`clang -E … | grep -c '\bX\b'`) and by dumping the declarations of that
   translation unit (`clang -std=gnu23 -Xclang -ast-dump`) - the signature in each `/// C:` line is
   the one that dump prints, with `int` as `c_int`, `size_t` as `usize`, `wchar_t` (4 bytes on
   this target, `wchar.rs` 10 says `c_int`) as `c_int` and `long double` (8 bytes, measured) as
   `c_double`.  The names `stdlib.rs` already declares (`malloc`, `realloc`, `calloc`, `free`,
   `strtod`, `strtol`, `strtoul`, `getenv`) are *not* repeated; a whole-import module carries these
   (PORTING.md 0.1-10, the `stdio.rs` `stdio_decls` shape) so that a unit which declares one of
   them itself - `lib/getloadavg.rs`, `lib/mkostemp.rs`, `lib/fstatat.rs` and others do - shadows
   the glob instead of colliding with it (`E0428`).  */
pub mod stdlib_decls {
  use super::*;

  unsafe extern "C" {
    /* C 249-269: `_Exit` and `abort`, both `_Noreturn`.  */
    /// C 253: `_GL_FUNCDECL_SYS (_Exit, void, (int status), ) _GL_ATTRIBUTE_NORETURN;`
    pub fn _Exit (status: c_int) -> !;
    /// C 285: `_GL_FUNCDECL_RPL (abort, _Noreturn void, (void), );` (the `@GNULIB_ABORT_DEBUG@` arm).
    pub fn abort () -> !;
    /// C 322: `_GL_FUNCDECL_RPL (aligned_alloc, void *, (size_t alignment, size_t size), …);`
    pub fn aligned_alloc (alignment: usize, size: usize) -> *mut c_void;
    /// C 455: `_GL_FUNCDECL_SYS (canonicalize_file_name, char *, (const char *name), );`
    pub fn canonicalize_file_name (name: *const c_char) -> *mut c_char;
    /// C 519: `_GL_FUNCDECL_SYS (ecvt, char *, (double value, int ndigit, int *decpt, int *sign), );`
    pub fn ecvt (value: c_double, ndigit: c_int, decpt: *mut c_int, sign: *mut c_int)
      -> *mut c_char;
    /// C 542: `_GL_FUNCDECL_SYS (fcvt, char *, (double value, int ndigit, int *decpt, int *sign), );`
    pub fn fcvt (value: c_double, ndigit: c_int, decpt: *mut c_int, sign: *mut c_int)
      -> *mut c_char;
    /// C 565: `_GL_FUNCDECL_SYS (gcvt, char *, (double value, int ndigit, char *buf), );`
    pub fn gcvt (value: c_double, ndigit: c_int, buf: *mut c_char) -> *mut c_char;
    /// C 586: `_GL_FUNCDECL_SYS (getloadavg, int, (double loadavg[], int nelem), );`
    pub fn getloadavg (loadavg: *mut c_double, nelem: c_int) -> c_int;
    /// C 619: `_GL_FUNCDECL_SYS (getprogname, const char *, (void), );`
    pub fn getprogname () -> *const c_char;
    /// C 664: `_GL_FUNCDECL_SYS (getsubopt, int, (char **optionp, char *const *tokens, char **valuep), );`
    pub fn getsubopt (optionp: *mut *mut c_char, tokens: *const *mut c_char,
                      valuep: *mut *mut c_char) -> c_int;
    /// C 1290: `_GL_FUNCDECL_SYS (initstate, char *, (unsigned int seed, char *buf, size_t buf_size), …);`
    pub fn initstate (seed: c_uint, state: *mut c_char, n: usize) -> *mut c_char;
    /// C 787: `_GL_FUNCDECL_SYS (mbstowcs, size_t, (wchar_t *dest, const char *src, size_t len), );`
    pub fn mbstowcs (dest: *mut c_int, src: *const c_char, len: usize) -> usize;
    /// C 816: `_GL_FUNCDECL_SYS (mbtowc, int, (wchar_t *pwc, const char *s, size_t n), );`
    pub fn mbtowc (pwc: *mut c_int, s: *const c_char, n: usize) -> c_int;
    /// C 875: `_GL_FUNCDECL_SYS (mkostemp, int, (char * /*template*/, int /*flags*/), );`
    pub fn mkostemp (template: *mut c_char, flags: c_int) -> c_int;
    /// C 914: `_GL_FUNCDECL_SYS (mkostemps, int, (char * /*template*/, int /*suffixlen*/, int /*flags*/), );`
    pub fn mkostemps (template: *mut c_char, suffixlen: c_int, flags: c_int) -> c_int;
    /// C 952: `_GL_FUNCDECL_SYS (mkstemp, int, (char * /*template*/), );`
    pub fn mkstemp (template: *mut c_char) -> c_int;
    /// C 1003: `_GL_FUNCDECL_SYS (mktemp, char *, (char * /*template*/), );`
    pub fn mktemp (template: *mut c_char) -> *mut c_char;
    /// C 1017: `_GL_FUNCDECL_SYS (posix_memalign, int, (void **memptr, size_t alignment, size_t size), );`
    pub fn posix_memalign (memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
    /// C 1046: `_GL_FUNCDECL_SYS (posix_openpt, int, (int flags), );`
    pub fn posix_openpt (flags: c_int) -> c_int;
    /// C 1075: `_GL_FUNCDECL_SYS (ptsname, char *, (int fd), );`
    pub fn ptsname (fd: c_int) -> *mut c_char;
    /// C 1100: `_GL_FUNCDECL_SYS (ptsname_r, int, (int fd, char *buf, size_t buflen), );`
    pub fn ptsname_r (fd: c_int, buf: *mut c_char, buflen: usize) -> c_int;
    /// C 1155: `_GL_FUNCDECL_SYS (putenv, int, (char *string), );` (the `REPLACE_PUTENV` arm is
    /// not the frozen one, and the name has three arms, so no body is compared).
    pub fn putenv (string: *mut c_char) -> c_int;
    /// C 1180: `_GL_FUNCDECL_SYS (qsort_r, void, (void *base, size_t nmemb, size_t size, …), );`
    /// - the frozen `<stdlib.h>`'s BSD-signature comparator.
    pub fn qsort_r (base: *mut c_void, nmemb: usize, size: usize, thunk: *mut c_void,
                    compar: unsafe extern "C" fn (*mut c_void, *const c_void, *const c_void)
                            -> c_int);
    /// C 1223: `_GL_FUNCDECL_SYS (rand, int, (void), );`
    pub fn rand () -> c_int;
    /// C 1240: `_GL_FUNCDECL_SYS (random, long, (void), );`
    pub fn random () -> c_long;
    /// C 1305: `_GL_FUNCDECL_SYS (setstate, char *, (const char *state), );`
    pub fn setstate (state: *const c_char) -> *mut c_char;
    /// C 1266: `_GL_FUNCDECL_SYS (srandom, void, (unsigned int seed), );`
    pub fn srandom (seed: c_uint);
    /// C 1700: `_GL_FUNCDECL_SYS (strtof, float, (const char *restrict str, char **restrict endp), …);`
    pub fn strtof (nptr: *const c_char, endptr: *mut *mut c_char) -> c_float;
    /// C 1731: `_GL_FUNCDECL_SYS (strtold, long double, (const char *restrict str, char **restrict endp), …);`
    /// (`long double` is 8 bytes on this target, measured, so the port spells `c_double`.)
    pub fn strtold (nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    /// C 1809: `_GL_FUNCDECL_SYS (strtoll, long long int, (const char *restrict str, char **restrict endp, int base), …);`
    pub fn strtoll (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_longlong;
    /// C 1890: `_GL_FUNCDECL_SYS (strtoull, unsigned long long int, (const char *restrict str, char **restrict endp, int base), …);`
    pub fn strtoull (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    /// C 1942: `_GL_FUNCDECL_SYS (unsetenv, int, (const char *name), );`
    pub fn unsetenv (name: *const c_char) -> c_int;
    /// C 1967: `_GL_FUNCDECL_SYS (wctomb, int, (char *s, wchar_t wc), );`
    pub fn wctomb (s: *mut c_char, wc: c_int) -> c_int;
    /// C 1628: `_GL_FUNCDECL_SYS (setenv, int, (const char *name, const char *value, int replace), );`
    pub fn setenv (name: *const c_char, value: *const c_char, replace: c_int) -> c_int;

    /* ---- the two live renames --------------------------------------------------------------
       C 293-297 and C 2148-2152: the generated header substitutes `@REPLACE_FREE@` and
       `@REPLACE_REALPATH@` to 1 (measured: `#if 1` at lines 880/881 and 2149/2150 of the
       generated header), so the frozen `<stdlib.h>` really does `# undef free` +
       `# define free rpl_free` and declares the *renamed* functions with `_GL_FUNCDECL_RPL`.
       The port keeps C's two names exactly as C writes them: the declaration is the renamed
       symbol - which the unit (`lib/free.rs`, the port of gnulib's `free.c`) emits with
       `#[unsafe(export_name = "rpl_free")]` (PORTING.md P-81) - and the macro is C's own
       redirect, so a unit that includes this header and writes `free! (p)` reaches it.  */
    /// C 297: `_GL_FUNCDECL_RPL (free, void, (void *ptr), );`
    pub fn rpl_free (ptr: *mut c_void);
    /// C 2152: `_GL_FUNCDECL_RPL (realpath, char *, (const char *restrict name, char *restrict resolved), …);`
    pub fn rpl_realpath (name: *const c_char, resolved: *mut c_char) -> *mut c_char;
  }
}
use stdlib_decls::*;

/// C 294: `#   define free rpl_free` - **live** (the generated header's 880-884).  C's own
/// redirect, which is why the port spells it as a macro over the renamed declaration instead of
/// a second `fn` (`stdlib.rs`'s `free` is the *system* symbol and stays).
/// C: #   define free rpl_free
macro_rules! free { ($ptr:expr) => { rpl_free ($ptr) }; }
/// C 2151: `#   define realpath rpl_realpath` - **live** (the generated header's 2149-2153).
/// C: #   define realpath rpl_realpath
macro_rules! realpath { ($name:expr, $resolved:expr) => { rpl_realpath ($name, $resolved) }; }
