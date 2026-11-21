// femacs: mirror of src/conf_post.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  C's AH_BOTTOM post-configuration block: what the port keeps
// from it is the set of names the rest of the tree actually spells - `NDEBUG`, the compiler
// predicates, the attribute spellings, `VFORK`, `likely`, `UNINIT`, `bool_bf` - plus the three
// platform arms (MS-DOS, MS-Windows, Cygwin) whose text this target does not compile.
//
// **Liveness is measured, not read off the comments.**  `clang -std=gnu23 -E -dD
// -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib` on a probe that
// includes `<config.h>` prints every live definition below with its body, and
// docs/baseline/config/darwin-arm64.macros.tsv (generated from the same preprocessor run)
// supplies the predicates the `#if`s test: `MSDOS`, `WINDOWSNT`, `CYGWIN`, `HAVE_NTGUI`, `HPUX`,
// `NS_IMPL_GNUSTEP`, `EMACS_EXTERN_INLINE`, `__SANITIZE_ADDRESS__`, `__DJGPP__` and `vfork` are
// undefined, `GCC_LINT` is 1, `DARWIN_OS` is defined, and this clang has
// `__has_attribute`/`__has_feature` built in.
//
// **P-66.**  This file is not on docs/baseline/config/cond-eval-verified.txt:
// `tools/check-cond-eval.py src/conf_post.h` reports 3 disagreements, because
// `defined __has_attribute` (C 68) and `#ifdef __has_feature` (C 83) are clang builtins the
// macro table does not carry, so the evaluator answers "dead" where clang answers "live".  The
// extractor therefore inventories *both* arms and this mirror names all 66 macros: the dead arms
// are carried under `#[cfg (feature = "…")]`, each naming its condition's non-frozen side (P-51).
// They are parsed by layout-check; no gate enables those features (`xtask dead-arms` flips only
// its own six), which docs/ports/include-conf_post.md records.
//
// Ten names have no Rust item and are recorded in docs/ports/excluded-macros.txt:
// `HAS_ATTRIBUTE`/`HAS_FEATURE` (preprocessor builtins plus identifier pasting), their dead
// fallback table `HAS_ATTR_*`, `UNINIT`, `likely` and the dead `vfork` rename.

// The header includes nothing (its only `#include`s are the MS-Windows `ms-w32.h` at C 37-39 and
// the three stdio headers of the dead C 402-405 arm), so the mirror cannot take `c_int`/`c_char`
// from an included foundation the way the display mirrors take them from lisp.rs: a second
// `use std::ffi::…` at the including crate root is an E0252 (measured - lisp.rs already imports
// the names there for every unit).  The four live items that need them spell
// `core::ffi::c_int`/`core::ffi::c_char`, the absolute form the system mirrors
// (`lib/include/stdint.in.rs`, `ftoastr.rs`, `stdio-impl.rs`) use for the same reason, so this
// mirror compiles on its own (`xtask build-tu src/conf_post.h`).

// C 25-30: `#if !defined ENABLE_CHECKING && !defined NDEBUG` / `# define NDEBUG`.  ENABLE_CHECKING
// is `#undef` (macros.tsv 872) and nothing has defined NDEBUG before this point, so C defines it;
// it only steers `<assert.h>`, and the port's assertions are `eassert!`/`eassume!`, gated on
// `cfg (enable_checking)` (PORTING.md 2.5).
// [U1-MODULE-MIRROR]
use super::*;
pub const NDEBUG: bool = true;

// C 41-51: `GNUC_PREREQ (V, W, X)`.  The third arm is live (`__GNUC_MINOR__` is 2 at macros.tsv
// 6259 and `__GNUC_PATCHLEVEL__` is 1 at 6260, so neither `#ifndef` is taken), and C's predicate
// is a numeric comparison, so the port keeps it as a macro with the measured compiler version
// inlined (`__GNUC__` 4, macros.tsv 6264; a C `bool` counts as 0/1).  C's uses are all `#if`s,
// whose answers the frozen configuration already fixes (P-08).
macro_rules! GNUC_PREREQ {
  ($v:expr, $w:expr, $x:expr) => {
    ($v) < 4 + ((($w) < 2 + ((($x) <= 1) as core::ffi::c_int)) as core::ffi::c_int)
  };
}

// C 53-62: the `bool_bf` bitfield type.  `NS_IMPL_GNUSTEP` is `#undef` (macros.tsv 2997) and
// `__MINGW32__` is undefined, so C 61's `typedef bool bool_bf;` is live; the width is the
// measured one, and every mirror that packs a `bool_bf` run reproduces the same byte
// (PORTING.md 1.3).
pub type bool_bf = bool;
const _: () = core::assert! (std::mem::size_of::<bool_bf> () == 1);

// C 64-88: the two compiler *predicate* families.  Both are excluded by name (rows in
// docs/ports/excluded-macros.txt) and C's text is kept here for the reader:
//
//  * `HAS_ATTRIBUTE (a)` - C 71's arm is live (measured: clang defines `__has_attribute`, and
//    `3 < 21 + (5 <= 0)` holds), and its body is a preprocessor builtin applied to a pasted
//    identifier.  C 73-78's dead fallback arm is a second definition plus the five values below.
//  * `HAS_FEATURE (a)` - C 84's arm is live (clang defines `__has_feature`); its only consumer,
//    C 90's `ADDRESS_SANITIZER`, is spelled out below.
//
//   # define HAS_ATTRIBUTE(a) __has_attribute (__##a##__)
//   # define HAS_ATTRIBUTE(a) HAS_ATTR_##a
//   # define HAS_ATTR_cleanup GNUC_PREREQ (3, 4, 0)
//   # define HAS_ATTR_no_address_safety_analysis false
//   # define HAS_ATTR_no_sanitize false
//   # define HAS_ATTR_no_sanitize_address GNUC_PREREQ (4, 8, 0)
//   # define HAS_ATTR_no_sanitize_undefined GNUC_PREREQ (4, 9, 0)
//   # define HAS_FEATURE(a) __has_feature (a)
//   # define HAS_FEATURE(a) false
//
// The five `HAS_ATTR_*` names answer for `HAS_ATTRIBUTE (cleanup)` and its four neighbours;
// measured with the frozen clang: `cleanup`, `no_sanitize_address` and `no_sanitize` exist,
// `no_address_safety_analysis` and `no_sanitize_undefined` do not.

// C 89-94: `ADDRESS_SANITIZER`.  `__SANITIZE_ADDRESS__` is undefined and
// `HAS_FEATURE (address_sanitizer)` is false, so C 93's `false` is live.  Two units already
// spell it (`src/sysdep.rs` 769, `src/alloc.rs` 121); this mirror is its
// definition point and those copies go when they include it (P-26).
pub const ADDRESS_SANITIZER: bool = false;

// C 96-106: `#ifdef HPUX` - HPUX is `#undef` (macros.tsv 1732), so the block is dead text: four
// `#undef`s of `srandom`, `random`, `HAVE_RANDOM` and `HAVE_RINT`, the last of which the frozen
// configuration defines as 1.  Recorded, no item.

// C 108-158: the MS-DOS (djgpp) arm.  `MSDOS` is `#undef` (macros.tsv 2849) and `__DJGPP__` is
// undefined, so nothing in it compiles here; it is carried under `#[cfg (feature = "msdos")]`,
// and C 121's inner condition (djgpp newer than 2.3) under `#[cfg (feature = "djgpp_2_3")]`, the
// name src/msdos.h's mirror already uses for the 2.3-and-older arm.  Two names of the block are
// also live on this target with the same values, from the frozen config.h, and are carried below
// without a feature (`_REGEX_AVOID_UCHAR_H` 1, `HAVE_LSTAT` 1).  C 109-111 (`You lose;`) is a
// program that refuses to compile on a non-djgpp MS-DOS compiler: recorded, not carried.

// C 112: `# define _NAIVE_DOS_REGS`, the register-window switch dosfns.c tests with `#ifdef`.
#[cfg (feature = "msdos")]
pub const _NAIVE_DOS_REGS: bool = true;

// C 119: `#define strtold _strtold` - djgpp spells the long-double parser with an underscore.
#[cfg (feature = "msdos")]
macro_rules! strtold { ($($args:tt)*) => { _strtold ($($args)*) }; }

// C 121-122: the djgpp-2.4-and-later arm defines `HAVE_LSTAT` 1; the frozen config.h defines the
// same name to the same value (its `#define` at config.h's 1077), so nothing else spells it and
// this is carried live.
pub const HAVE_LSTAT: core::ffi::c_int = 1;

// C 124/126-127: the djgpp-2.3-and-older arm renames `lstat` and supplies the two values old
// libraries lack.  Both names have the port's system values elsewhere (`lib/include/errno.rs`,
// `lib/include/stdint.in.rs`), so the arm's values carry the feature rather than being spelled
// twice (the `errno.in.rs` shape).
#[cfg (feature = "msdos")]
#[cfg (feature = "djgpp_2_3")]
macro_rules! lstat { ($($args:tt)*) => { stat ($($args)*) }; }
#[cfg (feature = "msdos")]
#[cfg (feature = "djgpp_2_3")]
pub const EOVERFLOW: core::ffi::c_int = ERANGE;
#[cfg (feature = "msdos")]
#[cfg (feature = "djgpp_2_3")]
pub const SIZE_MAX: usize = 4294967295;

// C 130-140: the nine wide-character entries lib/reg* wants, which djgpp's libraries lack.
#[cfg (feature = "msdos")]
macro_rules! mbrtowc { ($pwc:expr, $s:expr, $n:expr, $ps:expr) => { mbtowc ($pwc, $s, $n) }; }
#[cfg (feature = "msdos")]
macro_rules! wcrtomb { ($s:expr, $wc:expr, $ps:expr) => { wctomb ($s, $wc) }; }
#[cfg (feature = "msdos")]
macro_rules! btowc { ($b:expr) => { $b as wchar_t }; }
#[cfg (feature = "msdos")]
macro_rules! towupper { ($chr:expr) => { toupper ($chr) }; }
#[cfg (feature = "msdos")]
macro_rules! towlower { ($chr:expr) => { tolower ($chr) }; }
#[cfg (feature = "msdos")]
macro_rules! iswalnum { ($chr:expr) => { isalnum ($chr) }; }
#[cfg (feature = "msdos")]
macro_rules! wctype { ($name:expr) => { 0 as wctype_t }; }
#[cfg (feature = "msdos")]
macro_rules! iswctype { ($wc:expr, $type:expr) => { false }; }
#[cfg (feature = "msdos")]
macro_rules! mbsinit { ($ps:expr) => { 1 }; }

// C 142-143: `#define GNULIB_SUPPORT_ONLY_AT_FDCWD` - the flag lib/at-func.c tests;
// `lib/at-func.rs` 67 records C 77-79's arm as dead on this target too.
#[cfg (feature = "msdos")]
pub const GNULIB_SUPPORT_ONLY_AT_FDCWD: bool = true;

// C 145-146: `#define EOPNOTSUPP EINVAL` (the name has the port's value in
// `lib/include/errno.rs`).
#[cfg (feature = "msdos")]
pub const EOPNOTSUPP: core::ffi::c_int = EINVAL;

// C 148-150: `#define opendir sys_opendir` - the MS-DOS build intercepts `opendir` to keep the
// directory name for `readlinkat` (msdos.c); the `<dirent.h>` mirrors own the real one.
#[cfg (feature = "msdos")]
macro_rules! opendir { ($($args:tt)*) => { sys_opendir ($($args)*) }; }

// C 154: `#define emacs_raise(sig) msdos_fatal_signal (sig)` - src/msdos.h's mirror declares
// `msdos_fatal_signal`.  On this target `emacs_raise` is src/syssignal.h's `raise (sig)` arm,
// which `include/syssignal.rs` 91 already spells.
#[cfg (feature = "msdos")]
macro_rules! emacs_raise { ($sig:expr) => { msdos_fatal_signal ($sig) }; }

// C 156-157: `#define DATA_START (&etext + 1)` - vm-limit.c's start-of-data address.
#[cfg (feature = "msdos")]
macro_rules! DATA_START { () => { unsafe { (&raw const etext).wrapping_add (1) } }; }

// C 160-167: `#if defined HAVE_NTGUI && !defined DebPrint` - `HAVE_NTGUI` is `#undef`
// (macros.tsv 1482), so both arms are dead.  The `EMACSDEBUG` one (C 163) expands the argument
// list into `_DebPrint`; the other, C 165's `((void) 0)`, is the last definition and the text the
// extractor compares - which is what the port carries.  `_DebPrint` itself (C 162) is MSVCRT's
// prototype: recorded, not declared.
#[cfg (feature = "have_ntgui")]
macro_rules! DebPrint { ($stuff:tt) => {}; }

// C 169-176: the Cygwin-with-NTGUI arm - `CYGWIN` is `#undef` (macros.tsv 731) and `HAVE_NTGUI`
// too, so all three of its definitions are dead.
#[cfg (feature = "cygwin")]
pub const NTGUI_UNICODE: bool = true;

// C 171: `# define _WIN32_WINNT 0x500` ("Win2k"); nt/inc's mirror has the Windows value 0x400.
#[cfg (feature = "cygwin")]
pub const _WIN32_WINNT: core::ffi::c_int = 0x500;

// C 173-175: `#ifndef strnicmp` / `#define strnicmp strncasecmp` - the name Cygwin's `<string.h>`
// carried before 1.7.33.
#[cfg (feature = "cygwin")]
macro_rules! strnicmp { ($($args:tt)*) => { strncasecmp ($($args)*) }; }

// C 178-183: the TZ getter/setter rename, live (the measured `#define getenv_TZ
// emacs_getenv_TZ`).  `lib/time_rz.rs` 75-76 already spells both; the definitions belong
// to this mirror, and that unit's copies go when it includes it (P-26).
macro_rules! getenv_TZ { () => { emacs_getenv_TZ () }; }
macro_rules! setenv_TZ { ($tz:expr) => { emacs_setenv_TZ ($tz) }; }

// C 185-186: `NO_INLINE` and `EXTERNALLY_VISIBLE`, live.  C splices them into declarations; Rust
// spells the effects as `#[inline(never)]` and `#[unsafe(no_mangle)] pub` on the item
// (PORTING.md 8), so each name is the no-op forward of what its C body names - the
// `lib/include/attribute.rs` pair for `ATTRIBUTE_NOINLINE`/`ATTRIBUTE_EXTERNALLY_VISIBLE`.
macro_rules! NO_INLINE { () => { _GL_ATTRIBUTE_NOINLINE! () }; }
macro_rules! EXTERNALLY_VISIBLE { () => { _GL_ATTRIBUTE_EXTERNALLY_VISIBLE! () }; }

// C 188-214: `PRINTF_ARCHETYPE`.  The live arm is C 213's `__printf__` (`__GLIBC_MINOR__` and
// `__MINGW32__` are undefined, so the first two arms are dead): a GCC format archetype token that
// only `__attribute__ ((format (…)))` can consume.  The port drops that attribute (PORTING.md 8),
// so the name is the no-op its one consumer needs (the `_GL_ATTRIBUTE_FORMAT` shape).
macro_rules! PRINTF_ARCHETYPE { () => {}; }

// C 215-216: `ATTRIBUTE_FORMAT_PRINTF (string_index, first_to_check)` forwards to
// `_GL_ATTRIBUTE_FORMAT ((PRINTF_ARCHETYPE, …))`, which the port carries as a no-op.
macro_rules! ATTRIBUTE_FORMAT_PRINTF {
  ($string_index:tt, $first_to_check:tt) => {
    _GL_ATTRIBUTE_FORMAT! ((PRINTF_ARCHETYPE! (), $string_index, $first_to_check))
  };
}

// C 218: `#define ARG_NONNULL _GL_ATTRIBUTE_NONNULL`, live.  PORTING.md 0.2's attribute rule
// drops it where C applies it; the forward keeps C's token (the `_GL_ARG_NONNULL` row of
// docs/ports/excluded-macros.txt is the same construct from the header that declares it).
macro_rules! ARG_NONNULL { ($args:tt) => { _GL_ATTRIBUTE_NONNULL! ($args) }; }

// C 220-226: `DECLARE_POINTER_ALIAS (name, type, addr)` declares `type *name = (type *) addr`
// with the may-alias attribute.  C's use sites (process.c's two, w32.c's two) write the
// declaration directly in the port, where `src/process.rs` 2878-2909 records the macro in
// a comment and spells `let sin: *mut sockaddr_in = sa as *mut sockaddr_in;`; the macro is
// carried because C's text is a declaration Rust can express, with C's initializer shape and the
// attribute as the no-op `_GL_ATTRIBUTE_MAY_ALIAS` (lib/include/attribute.rs 269).
macro_rules! DECLARE_POINTER_ALIAS {
  ($name:ident, $type:ty, $addr:expr) => {
    let $name: *mut $type = { _GL_ATTRIBUTE_MAY_ALIAS! (); $addr as *mut $type };
  };
}

// C 228-232: `ATTRIBUTE_SECTION (name)` is `__attribute__ ((section (name)))`.  The port's
// counterpart is `#[unsafe(link_section = "…")]` on the item itself (PORTING.md 8), no site keeps
// the macro - the `SUBR_SECTION_ATTRIBUTE` row of docs/ports/excluded-macros.txt records the same
// construct in src/lisp.h - so the name expands to nothing.
macro_rules! ATTRIBUTE_SECTION { () => {}; }

// C 234-235: `ATTRIBUTE_MALLOC_SIZE (args)` is the malloc plus alloc_size pair, both dropped in
// the port (PORTING.md 8); the forward keeps C's two tokens.
macro_rules! ATTRIBUTE_MALLOC_SIZE {
  ($args:tt) => { _GL_ATTRIBUTE_MALLOC! (); _GL_ATTRIBUTE_ALLOC_SIZE! ($args); };
}

// C 237-247: `ADDRESS_SANITIZER_WORKAROUND`.  The live arm is C 244's empty macro (measured:
// ADDRESS_SANITIZER is false, so the `! ADDRESS_SANITIZER` term holds); its only consumers are
// the two dropped `no_sanitize` attributes below.
macro_rules! ADDRESS_SANITIZER_WORKAROUND { () => {}; }

// C 249-260: `ATTRIBUTE_NO_SANITIZE_ADDRESS`.  The live arm is C 253-254's spelling (measured:
// `HAS_ATTRIBUTE (no_sanitize_address)` holds).  The attribute is dropped (PORTING.md P-13;
// `src/alloc.rs` 447 and 5392 record the drop at its two C use sites).
macro_rules! ATTRIBUTE_NO_SANITIZE_ADDRESS { () => {}; }

// C 262-271: `ATTRIBUTE_NO_SANITIZE_UNDEFINED`.  The live arm is C 267's
// `__attribute__ ((no_sanitize ("undefined")))` (measured: `HAS_ATTRIBUTE (no_sanitize_undefined)`
// is false and `HAS_ATTRIBUTE (no_sanitize)` holds).  Dropped like its neighbour.
macro_rules! ATTRIBUTE_NO_SANITIZE_UNDEFINED { () => {}; }

// C 273-278: `#if ADDRESS_SANITIZER && !defined vfork` / `# define vfork fork`.  ADDRESS_SANITIZER
// is false here, so the rename is dead text and has no carrier feature of its own: it is recorded
// in docs/ports/excluded-macros.txt, and the port's `vfork` is the system function that
// `src/sysdep.rs` 948 declares.
//   # define vfork fork

// C 280-291: `VFORK ()`.  `DARWIN_OS` is defined (macros.tsv 733), so C 284-288's
// `_Pragma`-wrapped `vfork ()` is live and C 290's arm is dead.  The `_Pragma`s silence a
// deprecation warning Rust does not have; `src/sysdep.rs` 961 already spells the macro
// this way, and that unit's copy goes when it includes this mirror.
macro_rules! VFORK { () => { vfork () }; }

// C 293-295: `#if ! (defined __FreeBSD__ || defined GNU_LINUX || defined __MINGW32__)` /
// `# undef PROFILING`.  All three are undefined here, so C undefines PROFILING - which nothing
// has defined (macros.tsv has no PROFILING row) - and the block is a no-op for the port.
// C 297-300's `#ifdef noinline` / `#undef noinline` is dead the same way.  Recorded, no item.

// C 302-364: the INLINE family.  `EMACS_EXTERN_INLINE` is undefined (nothing in the frozen build
// defines it), so C 352-362's `static inline` arm is live - and C 357's `#ifndef INLINE` is true
// (nothing has defined it yet), so C 358 is live too.  The port's mirrors write their functions
// as ordinary `#[inline]` items (P-12), so the four names are markers, exactly as
// `src/emacs.rs` 110-111 spells the `#define INLINE EXTERN_INLINE` that C's own emacs.c
// puts ahead of config.h.
pub const EXTERN_INLINE: () = ();
macro_rules! INLINE { () => { EXTERN_INLINE } }
macro_rules! INLINE_HEADER_BEGIN { () => {}; }
macro_rules! INLINE_HEADER_END { () => {}; }

// C 366-372: `UNINIT`.  `GCC_LINT` is 1 (macros.tsv 1162), so C 369's `= {0,}` is live and
// C 371's empty arm is dead.  It is a *declaration suffix*: `int id UNINIT;` declares a variable
// that the callee writes and that is zero-initialized, which the port spells at each site as
// `let mut id: c_int = 0;` (PORTING.md 1.5; coding.rs, data.rs, window.rs and the lib-src units
// all record the drop), so the name is recorded in docs/ports/excluded-macros.txt.
//   # define UNINIT = {0,}

// C 374-377: `likely (cond)` is `__builtin_expect (!!(cond), 1)`, a branch hint stable Rust has
// no equivalent for (PORTING.md 2.6 records the deviation).  The port's spelling is
// `src/include/lisp.rs` 704's `pub fn likely (cond: bool) -> bool` (the identity), so the
// name is recorded in docs/ports/excluded-macros.txt as having its counterpart there.

// C 379-382: the two strftime knobs, live with the measured `false` (they keep the port from
// using the non-Gregorian-calendar and AM/PM behaviors lib/strftime.c can be built with).
pub const REQUIRE_GNUISH_STRFTIME_AM_PM: bool = false;
pub const SUPPORT_NON_GREG_CALENDARS_IN_STRFTIME: bool = false;

// C 384-398: the second `#ifdef MSDOS` block - the two values lib/file-has-acl.c wants from
// headers the MS-DOS build does not generate, the `IFTTODT` fallback chain, and the `uchar.h`
// opt-out.
#[cfg (feature = "msdos")]
pub const _GL_DT_NOTDIR: core::ffi::c_int = 0x100;
#[cfg (feature = "msdos")]
pub const ENOTSUP: core::ffi::c_int = ENOSYS;

// C 389-393: `IFTTODT (mode)` as the djgpp chain; the port's live `IFTTODT` is the `<dirent.h>`
// shift form in `lib/include/dirent.rs` 38.
#[cfg (feature = "msdos")]
macro_rules! IFTODT {
  ($mode:expr) => {
    if S_ISREG ($mode) { DT_REG } else if S_ISDIR ($mode) { DT_DIR } else if S_ISLNK ($mode)
      { DT_LNK } else if S_ISBLK ($mode) { DT_BLK } else if S_ISCHR ($mode) { DT_CHR }
      else if S_ISFIFO ($mode) { DT_FIFO } else if S_ISSOCK ($mode) { DT_SOCK }
      else { DT_UNKNOWN }
  };
}

// C 395-397: `#define _REGEX_AVOID_UCHAR_H 1` - djgpp has no `<uchar.h>`; the frozen config.h
// defines the same name to 1 (its `#define` at config.h's 2675), so this is carried live.
pub const _REGEX_AVOID_UCHAR_H: core::ffi::c_int = 1;

// C 400-445: the MS-Windows console-safe stdio redirections - `WINDOWSNT` is `#undef`
// (macros.tsv 4010), so the whole block is dead and carries the feature that names it.  C 414-430's
// five `_gl_consolesafe_*` prototypes and C 438-443's `getdelim`/`getline` pairs are
// *declarations*: the port's `<stdio.h>` mirror owns those two names, and the extractor requires
// no function of this file, so only the five renames are carried.
#[cfg (feature = "windowsnt")]
macro_rules! fwrite { ($($args:tt)*) => { _gl_consolesafe_fwrite ($($args)*) }; }
#[cfg (feature = "windowsnt")]
macro_rules! fprintf { ($($args:tt)*) => { _gl_consolesafe_fprintf ($($args)*) }; }
#[cfg (feature = "windowsnt")]
macro_rules! printf { ($($args:tt)*) => { _gl_consolesafe_printf ($($args)*) }; }
#[cfg (feature = "windowsnt")]
macro_rules! vfprintf { ($($args:tt)*) => { _gl_consolesafe_vfprintf ($($args)*) }; }
#[cfg (feature = "windowsnt")]
macro_rules! vprintf { ($($args:tt)*) => { _gl_consolesafe_vprintf ($($args)*) }; }

// C 182-183: `emacs_getenv_TZ`/`emacs_setenv_TZ` are defined by src/timefns.c (the port's
// `src/timefns.rs` owns them), so this mirror declares them in a glob-imported module
// whose import the definition shadows (PORTING.md 0.1-10/P-21).  `lib/time_rz.rs` 69-72
// declares its own copy, which goes when that unit includes this mirror.
pub mod conf_post_decls {
  unsafe extern "C" {
    /* C 182: `extern char *emacs_getenv_TZ (void);`  */
    pub fn emacs_getenv_TZ () -> *mut core::ffi::c_char;
    /* C 183: `extern int emacs_setenv_TZ (char const *);`  */
    pub fn emacs_setenv_TZ (tz: *const core::ffi::c_char) -> core::ffi::c_int;
  }
}
pub use self::conf_post_decls::*;
