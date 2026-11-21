// femacs: port of lib/string.in.h @ fd7afced403 (gnulib's <string.h>; PORTING.md P-26).
//
// **The criterion is the generated header, and it exists.**  `string.h` is in the `string`
// module's `BUILT_SOURCES` and its rule is unconditional - there is no `GL_GENERATE_STRING_H_*`
// variable at all (so a `grep GL_GENERATE_STRING_H` finds nothing here, unlike the `stdbit.h`
// case) - measured:
//
//     $ ls -la build/emacs-ref/lib/string.h
//     -rw-r--r-- 1 liu staff 92367 Oct  3 04:44 build/emacs-ref/lib/string.h     (2,399 lines)
//     $ grep -n 'string.h: string.in.h' build/emacs-ref/lib/gnulib.mk
//     3838:string.h: string.in.h $(top_builddir)/config.status $(CXXDEFS_H) $(ARG_NONNULL_H) $(WARN_ON_USE_H)
//     $ grep -c '^GL_GENERATE_STRING_H' build/emacs-ref/lib/gnulib.mk
//     0
//
// The template's 111 distinct `@PLACEHOLDER@` names are substituted with *expressions* (the
// generated header is not pre-digested).  Three of them carry literal replacements
// (`GUARD_PREFIX` -> `GL`, `PRAGMA_SYSTEM_HEADER` -> `#pragma GCC system_header`,
// `PRAGMA_COLUMNS` -> empty) and the other 108 are `$(VAR)` references whose values live in
// `build/emacs-ref/lib/gnulib.mk`; `tools/extract-abi.py` cannot resolve them, so the
// extractor's item list is the union of every arm (P-66).  Resolved from the rule (the table
// and the command are in docs/ports/include-string.in.md), **36 of the 108 are `1`** and 70 are
// `0`/empty, and the live `@GNULIB_*@`/`@REPLACE_*@` selectors are **16**: `MDA_MEMCCPY`,
// `MDA_STRDUP`, `MEMEQ`, `MEMMEM`, `MEMPCPY`, `MEMRCHR`, `MEMSET_EXPLICIT`, `RAWMEMCHR`,
// `SIGDESCR_NP`, `STPCPY`, `STREQ`, `STRNLEN`, `STRNUL`, `FREE_POSIX`, `REPLACE_MEMMEM` and
// `REPLACE_FREE`.  The template has 77 `GNULIB_*`/`REPLACE_*` selectors in all, so the other
// **61 are `0`/`0 && ...`** - most of the template's own definitions are dead text (P-51).
//
// **The system half is `string.rs`.**  The template's live arms are `_GL_FUNCDECL_SYS`/
// `_GL_CXXALIAS_SYS` machinery - i.e. *declarations* of the system's functions - and the port's
// `lib/include/string.rs` (the frozen `<string.h>` mirror) carries the ones the units use, so
// this mirror `include!`s it and records the rest.  `attribute.rs` is included for the five
// `_GL_ATTRIBUTE_*` names the template's own fallback blocks guard (the frozen values come from
// `lib/attribute.h` through `config.h`).
//
// **Ownership: the template's own two function *definitions* live here** (the ruling
// `stdbit.in.rs` states, docs/status.md 3.181).  `tools/extract-abi.py` reads a *function body*
// out of a mirror's own text only - `macros`/`constants`/`statics` follow the `include!` chain,
// bodies do not - and two definitions of one name in one crate is `E0428`, so `memeq` (C 425-431)
// and `streq` (C 817-822) moved out of `string.rs` into this file, and the eight units that call
// them now include the owner: `lib/{memeq,streq}.rs`, `lib/time_rz.rs`, `lib/sig2str.rs`,
// `lib/boot-time.rs`, `lib-src/{ebrowse,etags,make-docfile}.rs`.
//
// **What this mirror carries, and why each shape.**  The 45 names `tools/extract-abi.py`
// inventories are: the guard (`_GL_STRING_H`, whose C spelling reads as `_`), the three
// `_GL_*_INLINE` qualifier spellings (live: `_GL_INLINE`, and written as `macro_rules!` rather
// than `const` - `lib/memeq.rs`/`lib/streq.rs`/`lib/strnul.rs` define the *same* C name as a
// `_GL_EXTERN_INLINE` marker const, and a macro and a const are different namespaces, so the two
// spellings coexist where two consts would be `E0428`; the `_GL_STDC_*_INLINE` shape), the four
// `GNULIB_defined_*` witnesses, the five `_GL_ATTRIBUTE_*` names (from `attribute.rs`), the two
// function definitions above, and **31 rename macros** like `#define memchr rpl_memchr` - every
// one of which sits inside a `#if @REPLACE_X@` arm that the frozen configuration does not take
// (so the name is *not* a macro here: measured, `clang -dM` on the generated header has none of
// them except the live `memmem` below).  Each is therefore a
// `#[cfg (feature = "gnulib_<name>")] macro_rules!` carrying C's own rename tokens, and the live
// one gets the frozen spelling as the ungated macro after it:
//
//   * `memmem`: `REPLACE_MEMMEM` is 1, so the frozen `<string.h>` really does `#define memmem
//     rpl_memmem`; the port keeps that macro here (its realization is `string.rs`'s
//     `memmem_decl`, whose `#[link_name = "rpl_memmem"]` declaration is the symbol, and the
//     replacement function is `lib/memmem.rs`'s).
//   * `strnul`: the frozen header's `strnul(s)` is the `_Generic` wrapper over `_gl_strnul`.  It
//     is **not** written here - `macro_rules!` shadow by textual order and this mirror
//     `include!`s `string.rs` first, so a second `macro_rules! strnul` would shadow `string.rs`'s
//     `strnul!` and break its users (`lib/time_rz.rs` 198); the C macro's name is answered
//     through the include chain, which the extractor follows for macros.
//
// Not carried (PORTING.md 0.1; the "not carried" paragraph of docs/ports/include-string.in.md):
// C 20-26's pragma prologue, C 28-45's double-inclusion guard machinery, C 52-54's
// `_GL_CONFIG_H_INCLUDED` check, C 56's `<stddef.h>`, C 58-62's `<locale.h>` arm, the
// `#ifdef __cplusplus` arms, the `_GL_*`-machinery text the build inlines from `c++defs.h`
// (`_GL_FUNCDECL_*`/`_GL_CXXALIAS_*`/`_GL_CXXALIASWARN*`, which the port spells as ordinary
// declarations in `string.rs`), and the live arms' prototypes of the functions the port's units
// reach through `string.rs`.

// [U1-MODULE-MIRROR]
use super::*;
// [U1] string.rs imported via super
// [U1] attribute.rs imported via super

/* C 45: `#define _@GUARD_PREFIX@_STRING_H` - the split include guard, whose substitution is `_GL_STRING_H`.
   The template's own spelling reads as `_` to the extractor, which
   docs/ports/excluded-macros.txt records.  */
pub const _GL_STRING_H: c_int = 1;

/// C 85: `# define _GL_MEMEQ_INLINE _GL_INLINE` - the C99-inline qualifier, guarded by
/// `#ifndef _GL_MEMEQ_INLINE` so that `lib/memeq.c`'s own `_GL_EXTERN_INLINE` spelling wins in
/// that unit.  The port writes the *macro* (not a `const`): `macro_rules!` shadow by textual
/// order and a macro and a `const` are different namespaces, so `lib/memeq.rs` can keep the
/// wrapper's `pub const _GL_MEMEQ_INLINE: () = _GL_EXTERN_INLINE;` beside this name without the
/// `E0428` a second `const` would be (the `_GL_STDC_*_INLINE` shape, `stdbit.in.rs`).
/// C: # define _GL_MEMEQ_INLINE _GL_INLINE
macro_rules! _GL_MEMEQ_INLINE { () => { _GL_INLINE }; }

/// C 88: `# define _GL_STREQ_INLINE _GL_INLINE` - the `memeq` arm's sibling; see C 85 above.
/// C: # define _GL_STREQ_INLINE _GL_INLINE
macro_rules! _GL_STREQ_INLINE { () => { _GL_INLINE }; }

/// C 91: `# define _GL_STRNUL_INLINE _GL_INLINE` - the `memeq` arm's sibling; see C 85 above.
/// C: # define _GL_STRNUL_INLINE _GL_INLINE
macro_rules! _GL_STRNUL_INLINE { () => { _GL_INLINE }; }

/* C 346: `#   define ffsll rpl_ffsll` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_ffsll")]
macro_rules! ffsll {
  () => { rpl_ffsll };
}

/* C 371: `#   define memccpy _memccpy` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_memccpy")]
macro_rules! memccpy {
  () => { _memccpy };
}

/* C 388: `#   define memchr rpl_memchr` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_memchr")]
macro_rules! memchr {
  () => { rpl_memchr };
}

/* C 425-431: `_GL_MEMEQ_INLINE bool memeq (void const *__s1, void const *__s2, size_t __n)
   { return !memcmp (__s1, __s2, __n); }` - the template's **own** definition, and live: the
   arm is `@GNULIB_MEMEQ@ && !@HAVE_DECL_MEMEQ@`, and this target has neither `memeq` nor
   `GNULIB_defined_memeq` (the generated header 976 carries the definition).  It sits here
   rather than in the system half because the inventory gate compares a function body against
   the mirror's *own* text (`string.rs` records the ruling).  Rust's `!` on a `c_int` is
   bitwise, so `!memcmp (…)` is written as `== 0`, and C's own parameter names are kept -
   `lib/include/getopt*.rs` keeps theirs.  The frozen object exports `_memeq` (lib/memeq.rs
   re-exports this definition, PORTING.md P-81).  */
#[inline(always)]
pub unsafe fn memeq (__s1: *const c_void, __s2: *const c_void, __n: usize) -> bool
{
  return memcmp (__s1, __s2, __n) == 0;
}

/* C 436: `#  define GNULIB_defined_memeq 1` - this arm is live in the frozen configuration.  */
pub const GNULIB_defined_memeq: c_int = 1;

/* C 444: `#   define memmem rpl_memmem` - live: the frozen `<string.h>` defines it.  C's replacement arm
   (the `#[cfg]` one below) is dead, and the ungated macro is the frozen
   spelling.  */
#[cfg (feature = "gnulib_memmem")]
macro_rules! memmem {
  () => { rpl_memmem };
}
/// `memmem` is `memmem rpl_memmem` (frozen; `docs/ports/include-string.in.md` records the measurement).
macro_rules! memmem {
  () => { ( memmem, rpl_memmem, ) };
}

/* C 481: `#   define mempcpy rpl_mempcpy` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_mempcpy")]
macro_rules! mempcpy {
  () => { rpl_mempcpy };
}

/* C 551: `#   define memset_explicit rpl_memset_explicit` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_memset_explicit")]
macro_rules! memset_explicit {
  () => { rpl_memset_explicit };
}

/* C 614: `#   define stpcpy rpl_stpcpy` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_stpcpy")]
macro_rules! stpcpy {
  () => { rpl_stpcpy };
}

/* C 646: `#   define stpncpy rpl_stpncpy` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_stpncpy")]
macro_rules! stpncpy {
  () => { rpl_stpncpy };
}

/* C 693: `#   define strchrnul rpl_strchrnul` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strchrnul")]
macro_rules! strchrnul {
  () => { rpl_strchrnul };
}

/* C 801: `#    define strdup _strdup` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strdup")]
macro_rules! strdup {
  () => { _strdup };
}

/* C 817-822: `_GL_STREQ_INLINE bool streq (char const *__s1, char const *__s2)
   { return !strcmp (__s1, __s2); }` - the same live arm as `memeq` (generated header 1368),
   with the same `== 0` spelling for `!` and the same use of C's parameter names.  The frozen
   object exports `_streq` (lib/streq.rs re-exports this definition, P-81; the reference
   object's `T _streq` is the evidence, tools/check-symbols.py).  */
#[inline(always)]
pub unsafe fn streq (__s1: *const c_char, __s2: *const c_char) -> bool
{
  return strcmp (__s1, __s2) == 0;
}

/* C 828: `#  define GNULIB_defined_streq 1` - this arm is live in the frozen configuration.  */
pub const GNULIB_defined_streq: c_int = 1;

/* C 837: `#   define strncat rpl_strncat` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strncat")]
macro_rules! strncat {
  () => { rpl_strncat };
}

/* C 864: `#   define strncpy rpl_strncpy` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strncpy")]
macro_rules! strncpy {
  () => { rpl_strncpy };
}

/* C 894: `#   define strndup rpl_strndup` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strndup")]
macro_rules! strndup {
  () => { rpl_strndup };
}

/* C 951: `#   define strnlen rpl_strnlen` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strnlen")]
macro_rules! strnlen {
  () => { rpl_strnlen };
}

/* C 1090: `#   define strstr rpl_strstr` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strstr")]
macro_rules! strstr {
  () => { rpl_strstr };
}

/* C 1137: `#   define strcasestr rpl_strcasestr` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strcasestr")]
macro_rules! strcasestr {
  () => { rpl_strcasestr };
}

/* C 1210: `#   define strtok_r rpl_strtok_r` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strtok_r")]
macro_rules! strtok_r {
  () => { rpl_strtok_r };
}

/* C 1302-1305: `# define strnul(s) _Generic (1 ? (s) : (void *) 99, void * : (char *)
   _gl_strnul (s), const void * : _gl_strnul (s))` - live, and **deliberately not written
   here**.  `macro_rules!` shadow each other by textual order, and this mirror `include!`s
   `string.rs`, whose `strnul!` (the `void *` arm the port spells) and `_gl_strnul`
   (PORTING.md checklist 12's `_GL_STRNUL_INLINE` function) are the frozen spellings; a second
   `macro_rules! strnul` at this point would shadow them and break every `strnul! (s)` use
   (`lib/time_rz.rs` 198).  The C macro's name is answered through the include chain - the
   extractor reads Rust macro names from the file *and* the files it includes.  */

/* C 1311: `#  define GNULIB_defined_strnul 1` - this arm is live in the frozen configuration.  */
pub const GNULIB_defined_strnul: c_int = 1;

/* C 1339: `#   define mbslen rpl_mbslen` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_mbslen")]
macro_rules! mbslen {
  () => { rpl_mbslen };
}

/* C 1371: `#   define mbschr rpl_mbschr /* avoid collision with HP-UX function */` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_mbschr")]
macro_rules! mbschr {
  () => { rpl_mbschr };
}

/* C 1393: `#   define mbsrchr rpl_mbsrchr /* avoid collision with system function */` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_mbsrchr")]
macro_rules! mbsrchr {
  () => { rpl_mbsrchr };
}

/* C 1436: `#    define mbsstr(h,n) \` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_mbsstr")]
macro_rules! mbsstr {
  () => { ( _Generic, mbsstr, r#default, mbsstr, ) };
}

/* C 1505: `#    define mbspcasecmp(s,p) \` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_mbspcasecmp")]
macro_rules! mbspcasecmp {
  () => { ( _Generic, mbspcasecmp, r#default, mbspcasecmp, ) };
}

/* C 1543: `#    define mbscasestr(h,n) \` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_mbscasestr")]
macro_rules! mbscasestr {
  () => { ( _Generic, mbscasestr, r#default, mbscasestr, ) };
}

/* C 1570: `#   define mbspbrk rpl_mbspbrk /* avoid collision with HP-UX function */` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_mbspbrk")]
macro_rules! mbspbrk {
  () => { rpl_mbspbrk };
}

/* C 1663: `#   define GNULIB_defined_strerror 1` - dead: the enclosing block needs a replacement this target
   does not use.  */
#[cfg (feature = "gnulib_gnulib_defined_strerror")]
pub const GNULIB_defined_strerror: c_int = 1;

/* C 1670: `#   define strerror(...) ((const char *) strerror (__VA_ARGS__))` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strerror")]
macro_rules! strerror {
  () => { ( strerror, __VA_ARGS__, ) };
}

/* C 1688: `#   define strerror_r rpl_strerror_r` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strerror_r")]
macro_rules! strerror_r {
  () => { rpl_strerror_r };
}

/* C 1716: `#   define strerror_l rpl_strerror_l` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strerror_l")]
macro_rules! strerror_l {
  () => { rpl_strerror_l };
}

/* C 1752: `#   define strerrorname_np rpl_strerrorname_np` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strerrorname_np")]
macro_rules! strerrorname_np {
  () => { rpl_strerrorname_np };
}

/* C 1803: `#   define strsignal rpl_strsignal` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strsignal")]
macro_rules! strsignal {
  () => { rpl_strsignal };
}

/* C 1826: `#   define strverscmp rpl_strverscmp` - dead on the frozen target (its `REPLACE_*` selector is 0 and the
   live arm only declares the system's function, which `string.rs` carries).
   The gated macro keeps C's rename text.  */
#[cfg (feature = "gnulib_strverscmp")]
macro_rules! strverscmp {
  () => { rpl_strverscmp };
}
