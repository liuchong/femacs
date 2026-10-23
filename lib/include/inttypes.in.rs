// femacs: port of lib/inttypes.in.h @ fd7afced403 (gnulib's <inttypes.h>; PORTING.md P-26).
//
// **The criterion is the generated header, and it is this target's system header that answers.**
// gnulib's `inttypes-h` rule substitutes the template when its condition is non-empty, and here
// it is: `build/emacs-ref/lib/inttypes.h` exists (46,752 bytes, `/* DO NOT EDIT! GENERATED
// AUTOMATICALLY! */` on line 1).  Its substitutions, from the same build:
//
//     $ grep -n 'INTTYPES\|PRIPTR_PREFIX\|INT64_MAX_EQ_LONG_MAX\|APPLE_UNIVERSAL_BUILD' \
//         build/emacs-ref/config.status | head
//     S["HAVE_INTTYPES_H"]="1"          S["APPLE_UNIVERSAL_BUILD"]="0"
//     S["INT64_MAX_EQ_LONG_MAX"]="defined _LP64"
//     S["UINT64_MAX_EQ_ULONG_MAX"]="defined _LP64"
//     S["INT32_MAX_LT_INTMAX_MAX"]="1"  S["UINT32_MAX_LT_UINTMAX_MAX"]="1"
//     S["PRIPTR_PREFIX"]="__PRIPTR_PREFIX"
//     S["REPLACE_IMAXABS"]="0" ... S["HAVE_DECL_IMAXABS"]="1" ...   (the `GNULIB_*` are unset)
//     $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/inttypes.in.h) build/emacs-ref/lib/inttypes.h
//     (the head: `# if @HAVE_INTTYPES_H@` -> `# if 1`, `#  @INCLUDE_NEXT@ @NEXT_INTTYPES_H@` ->
//      `#  include_next <inttypes.h>`, plus the 368 lines the template's `/* … copied here.  */`
//      comments stand for - `c++defs.h`, `arg-nonnull.h`, `warn-on-use.h`, `stdint.h`)
//
// **Every name the template defines is dead text on this target, and the measurement is the
// include order.**  `inttypes.h` is reached with `build/emacs-ref/lib` first on the path, so its
// C 41 `# include_next <inttypes.h>` pulls the *system* header, whose own `#include <stdint.h>`
// resolves to gnulib's generated `stdint.h`; that file defines `_GL_JUST_INCLUDE_SYSTEM_INTTYPES_H`
// (its C 35) before it includes `<inttypes.h>`, and the generated header's own body is guarded by
// C 47's `#if ! defined INTTYPES_H && ! defined _GL_JUST_INCLUDE_SYSTEM_INTTYPES_H` - so the whole
// C 47-1574 body is skipped, `INTTYPES_H` is never defined, and the names come from the system
// `<inttypes.h>`:
//
//     $ printf '' > /tmp/empty.c
//     $ clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib \
//         -include config.h -include inttypes.h -E -dM /tmp/empty.c | grep -E 'PREFIX|INTTYPES_H'
//     #define _GL_FINISHED_INCLUDING_SYSTEM_INTTYPES_H
//     #define _GL_JUST_INCLUDE_SYSTEM_INTTYPES_H
//     (no `INTTYPES_H`, no `_PRI64_PREFIX`, no `_LONG_LONG_FORMAT_PREFIX`)
//
// The values below are measured the same way, one name per line, `__STR`-wrapped so an empty or
// undefined expansion is still one line (`/tmp` transcript in `docs/ports/include-inttypes.in.md`).
//
// **The four shapes the mirror uses, and why.**  Each of the template's 167 names is dead here, so
// each is P-51 text; the frozen value is written next to it:
//
//   * `same` (53 names): C's fallback and the frozen value agree, so one ungated constant answers
//     for both arms (the `fcntl.in.rs` `FD_CLOEXEC` shape).
//   * `diff/literal` (57): they differ and C's body is a literal.  C's value is a *gated constant*
//     - `tools/extract-abi.py` compares a constant's value against C's, and the ungated frozen
//     value is a `macro_rules!` of the same name, i.e. the other namespace, so the two do not
//     collide and `rustc` compiles the frozen spelling (macro_rules shadowing is textual, and
//     the frozen macro is last).
//   * `diff/expression` (45): they differ and C's body is an expression (`_PRI64_PREFIX "d"`,
//     `PRId64`, `@PRIPTR_PREFIX@ "d"`).  The gated arm is a `macro_rules!` naming C's tokens
//     (the checker compares the *first* definition, so it sees C's text) and the frozen arm is
//     the second, ungated one.
//   * `undefined` (11): the frozen `<inttypes.h>` defines no such name at all, so only the gated
//     arm exists; each is named in the ledger with its measurement.
//
// The features are named after the name they carry (`no_prid8`, `no_pri64_prefix`, ...), the
// `no_<name>` convention `include-dirent.in.md`/`include-fcntl.in.md` use for the `#ifndef` arms;
// the probe below asserts every one of them off (P-51).
//
// **Not carried** (PORTING.md 0.1; the `include-dirent.in.md` "Not carried" paragraph): C 23-25's
// `__GNUC__ >= 3` test with the two pragma placeholders, C 32-45's and C 47's guard pair, C 50-52's
// `_GL_CONFIG_H_INCLUDED` check, C 56-58's `<stdint.h>`, C 60's `<limits.h>`, C 62-64's
// `_WIN32`-only `<stdio.h>` and C 66-69's `int`-width check have no Rust counterpart; the
// `_GL_*`/`_GL_CXXALIAS_*`/`_GL_FUNCDECL_*`/`_GL_WARN_ON_USE` text the build inlines from
// `c++defs.h`/`arg-nonnull.h`/`warn-on-use.h` is those headers' own mirrors' (the port's
// `c++defs.rs`, `arg-nonnull.rs`); and C 900-1029's four function blocks
// (`imaxabs`/`imaxdiv`/`strtoimax`/`strtoumax`) define no name of their own beyond the four
// rename macros carried below - their live arms are the system declarations
// (`imaxdiv_t`, `imaxabs`, `imaxdiv`, `strtoimax`, `strtoumax`), which the port's units reach
// through their own declarations (`lib/strtoimax.rs`, `lib-src/etags.rs` 2257).

// C 37: `#   define __STDC_FORMAT_MACROS 1` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_stdc_format_macros")]
pub const __STDC_FORMAT_MACROS: c_int = 1;

// C 42: `#  define _GL_FINISHED_INCLUDING_SYSTEM_INTTYPES_H` - the live arm of C 32-44's
// `# include_next` block; the definition is empty and so is the frozen one, so one ungated
// empty macro answers for both.
macro_rules! _GL_FINISHED_INCLUDING_SYSTEM_INTTYPES_H {
  () => {};
}

// C 47: `#define INTTYPES_H` - the split guard.
pub const INTTYPES_H: bool = true;

// C 83: `# define _LONG_LONG_FORMAT_PREFIX "ll"` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_long_long_format_prefix")]
pub const _LONG_LONG_FORMAT_PREFIX: &core::ffi::CStr = c"ll";

// C 88: `#  define PRId8 "d"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prid8")]
pub const PRId8: &core::ffi::CStr = c"d";
/// `PRId8` is `"hh" "d"` (frozen; measured).
macro_rules! PRId8 {
  () => { concat! ("hh", "d") };
}

// C 93: `#  define PRIi8 "i"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prii8")]
pub const PRIi8: &core::ffi::CStr = c"i";
/// `PRIi8` is `"hh" "i"` (frozen; measured).
macro_rules! PRIi8 {
  () => { concat! ("hh", "i") };
}

// C 98: `#  define PRIo8 "o"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prio8")]
pub const PRIo8: &core::ffi::CStr = c"o";
/// `PRIo8` is `"hh" "o"` (frozen; measured).
macro_rules! PRIo8 {
  () => { concat! ("hh", "o") };
}

// C 103: `#  define PRIu8 "u"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priu8")]
pub const PRIu8: &core::ffi::CStr = c"u";
/// `PRIu8` is `"hh" "u"` (frozen; measured).
macro_rules! PRIu8 {
  () => { concat! ("hh", "u") };
}

// C 108: `#  define PRIx8 "x"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prix8")]
pub const PRIx8: &core::ffi::CStr = c"x";
/// `PRIx8` is `"hh" "x"` (frozen; measured).
macro_rules! PRIx8 {
  () => { concat! ("hh", "x") };
}

// C 113: `#  define PRIX8 "X"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prix8")]
pub const PRIX8: &core::ffi::CStr = c"X";
/// `PRIX8` is `"hh" "X"` (frozen; measured).
macro_rules! PRIX8 {
  () => { concat! ("hh", "X") };
}

// C 118: `#  define PRId16 "d"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prid16")]
pub const PRId16: &core::ffi::CStr = c"d";
/// `PRId16` is `"hd"` (frozen; measured).
macro_rules! PRId16 {
  () => { concat! ("hd") };
}

// C 123: `#  define PRIi16 "i"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prii16")]
pub const PRIi16: &core::ffi::CStr = c"i";
/// `PRIi16` is `"hi"` (frozen; measured).
macro_rules! PRIi16 {
  () => { concat! ("hi") };
}

// C 128: `#  define PRIo16 "o"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prio16")]
pub const PRIo16: &core::ffi::CStr = c"o";
/// `PRIo16` is `"ho"` (frozen; measured).
macro_rules! PRIo16 {
  () => { concat! ("ho") };
}

// C 133: `#  define PRIu16 "u"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priu16")]
pub const PRIu16: &core::ffi::CStr = c"u";
/// `PRIu16` is `"hu"` (frozen; measured).
macro_rules! PRIu16 {
  () => { concat! ("hu") };
}

// C 138: `#  define PRIx16 "x"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prix16")]
pub const PRIx16: &core::ffi::CStr = c"x";
/// `PRIx16` is `"hx"` (frozen; measured).
macro_rules! PRIx16 {
  () => { concat! ("hx") };
}

// C 143: `#  define PRIX16 "X"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prix16")]
pub const PRIX16: &core::ffi::CStr = c"X";
/// `PRIX16` is `"hX"` (frozen; measured).
macro_rules! PRIX16 {
  () => { concat! ("hX") };
}

// C 148: `#  define PRId32 "d"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRId32` is `c"d"` (frozen; measured).
pub const PRId32: &core::ffi::CStr = c"d";

// C 153: `#  define PRIi32 "i"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIi32` is `c"i"` (frozen; measured).
pub const PRIi32: &core::ffi::CStr = c"i";

// C 158: `#  define PRIo32 "o"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIo32` is `c"o"` (frozen; measured).
pub const PRIo32: &core::ffi::CStr = c"o";

// C 163: `#  define PRIu32 "u"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIu32` is `c"u"` (frozen; measured).
pub const PRIu32: &core::ffi::CStr = c"u";

// C 168: `#  define PRIx32 "x"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIx32` is `c"x"` (frozen; measured).
pub const PRIx32: &core::ffi::CStr = c"x";

// C 173: `#  define PRIX32 "X"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIX32` is `c"X"` (frozen; measured).
pub const PRIX32: &core::ffi::CStr = c"X";

// C 182: `#  define _PRI64_PREFIX _LONG_LONG_FORMAT_PREFIX` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_pri64_prefix")]
macro_rules! _PRI64_PREFIX {
  () => { _LONG_LONG_FORMAT_PREFIX! () };
}

// C 185: `#  define PRId64 _PRI64_PREFIX "d"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prid64")]
macro_rules! PRId64 {
  () => { concat! (_PRI64_PREFIX! (), "d") };
}
/// `PRId64` is `"ll" "d"` (frozen; measured).
macro_rules! PRId64 {
  () => { concat! ("ll", "d") };
}

// C 188: `#  define PRIi64 _PRI64_PREFIX "i"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prii64")]
macro_rules! PRIi64 {
  () => { concat! (_PRI64_PREFIX! (), "i") };
}
/// `PRIi64` is `"ll" "i"` (frozen; measured).
macro_rules! PRIi64 {
  () => { concat! ("ll", "i") };
}

// C 197: `#  define _PRIu64_PREFIX _LONG_LONG_FORMAT_PREFIX` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_priu64_prefix")]
macro_rules! _PRIu64_PREFIX {
  () => { _LONG_LONG_FORMAT_PREFIX! () };
}

// C 200: `#  define PRIo64 _PRIu64_PREFIX "o"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prio64")]
macro_rules! PRIo64 {
  () => { concat! (_PRIu64_PREFIX! (), "o") };
}
/// `PRIo64` is `"ll" "o"` (frozen; measured).
macro_rules! PRIo64 {
  () => { concat! ("ll", "o") };
}

// C 203: `#  define PRIu64 _PRIu64_PREFIX "u"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_priu64")]
macro_rules! PRIu64 {
  () => { concat! (_PRIu64_PREFIX! (), "u") };
}
/// `PRIu64` is `"ll" "u"` (frozen; measured).
macro_rules! PRIu64 {
  () => { concat! ("ll", "u") };
}

// C 206: `#  define PRIx64 _PRIu64_PREFIX "x"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prix64")]
macro_rules! PRIx64 {
  () => { concat! (_PRIu64_PREFIX! (), "x") };
}
/// `PRIx64` is `"ll" "x"` (frozen; measured).
macro_rules! PRIx64 {
  () => { concat! ("ll", "x") };
}

// C 209: `#  define PRIX64 _PRIu64_PREFIX "X"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prix64")]
macro_rules! PRIX64 {
  () => { concat! (_PRIu64_PREFIX! (), "X") };
}
/// `PRIX64` is `"ll" "X"` (frozen; measured).
macro_rules! PRIX64 {
  () => { concat! ("ll", "X") };
}

// C 214: `# define PRIdLEAST8 "d"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_pridleast8")]
pub const PRIdLEAST8: &core::ffi::CStr = c"d";
/// `PRIdLEAST8` is `"hh" "d"` (frozen; measured).
macro_rules! PRIdLEAST8 {
  () => { concat! ("hh", "d") };
}

// C 217: `# define PRIiLEAST8 "i"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priileast8")]
pub const PRIiLEAST8: &core::ffi::CStr = c"i";
/// `PRIiLEAST8` is `"hh" "i"` (frozen; measured).
macro_rules! PRIiLEAST8 {
  () => { concat! ("hh", "i") };
}

// C 220: `# define PRIoLEAST8 "o"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prioleast8")]
pub const PRIoLEAST8: &core::ffi::CStr = c"o";
/// `PRIoLEAST8` is `"hh" "o"` (frozen; measured).
macro_rules! PRIoLEAST8 {
  () => { concat! ("hh", "o") };
}

// C 223: `# define PRIuLEAST8 "u"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priuleast8")]
pub const PRIuLEAST8: &core::ffi::CStr = c"u";
/// `PRIuLEAST8` is `"hh" "u"` (frozen; measured).
macro_rules! PRIuLEAST8 {
  () => { concat! ("hh", "u") };
}

// C 226: `# define PRIxLEAST8 "x"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixleast8")]
pub const PRIxLEAST8: &core::ffi::CStr = c"x";
/// `PRIxLEAST8` is `"hh" "x"` (frozen; measured).
macro_rules! PRIxLEAST8 {
  () => { concat! ("hh", "x") };
}

// C 229: `# define PRIXLEAST8 "X"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixleast8")]
pub const PRIXLEAST8: &core::ffi::CStr = c"X";
/// `PRIXLEAST8` is `"hh" "X"` (frozen; measured).
macro_rules! PRIXLEAST8 {
  () => { concat! ("hh", "X") };
}

// C 232: `# define PRIdLEAST16 "d"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_pridleast16")]
pub const PRIdLEAST16: &core::ffi::CStr = c"d";
/// `PRIdLEAST16` is `"hd"` (frozen; measured).
macro_rules! PRIdLEAST16 {
  () => { concat! ("hd") };
}

// C 235: `# define PRIiLEAST16 "i"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priileast16")]
pub const PRIiLEAST16: &core::ffi::CStr = c"i";
/// `PRIiLEAST16` is `"hi"` (frozen; measured).
macro_rules! PRIiLEAST16 {
  () => { concat! ("hi") };
}

// C 238: `# define PRIoLEAST16 "o"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prioleast16")]
pub const PRIoLEAST16: &core::ffi::CStr = c"o";
/// `PRIoLEAST16` is `"ho"` (frozen; measured).
macro_rules! PRIoLEAST16 {
  () => { concat! ("ho") };
}

// C 241: `# define PRIuLEAST16 "u"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priuleast16")]
pub const PRIuLEAST16: &core::ffi::CStr = c"u";
/// `PRIuLEAST16` is `"hu"` (frozen; measured).
macro_rules! PRIuLEAST16 {
  () => { concat! ("hu") };
}

// C 244: `# define PRIxLEAST16 "x"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixleast16")]
pub const PRIxLEAST16: &core::ffi::CStr = c"x";
/// `PRIxLEAST16` is `"hx"` (frozen; measured).
macro_rules! PRIxLEAST16 {
  () => { concat! ("hx") };
}

// C 247: `# define PRIXLEAST16 "X"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixleast16")]
pub const PRIXLEAST16: &core::ffi::CStr = c"X";
/// `PRIXLEAST16` is `"hX"` (frozen; measured).
macro_rules! PRIXLEAST16 {
  () => { concat! ("hX") };
}

// C 250: `# define PRIdLEAST32 "d"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIdLEAST32` is `c"d"` (frozen; measured).
pub const PRIdLEAST32: &core::ffi::CStr = c"d";

// C 253: `# define PRIiLEAST32 "i"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIiLEAST32` is `c"i"` (frozen; measured).
pub const PRIiLEAST32: &core::ffi::CStr = c"i";

// C 256: `# define PRIoLEAST32 "o"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIoLEAST32` is `c"o"` (frozen; measured).
pub const PRIoLEAST32: &core::ffi::CStr = c"o";

// C 259: `# define PRIuLEAST32 "u"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIuLEAST32` is `c"u"` (frozen; measured).
pub const PRIuLEAST32: &core::ffi::CStr = c"u";

// C 262: `# define PRIxLEAST32 "x"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIxLEAST32` is `c"x"` (frozen; measured).
pub const PRIxLEAST32: &core::ffi::CStr = c"x";

// C 265: `# define PRIXLEAST32 "X"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIXLEAST32` is `c"X"` (frozen; measured).
pub const PRIXLEAST32: &core::ffi::CStr = c"X";

// C 269: `#  define PRIdLEAST64 PRId64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_pridleast64")]
macro_rules! PRIdLEAST64 {
  () => { PRId64! () };
}
/// `PRIdLEAST64` is `"ll" "d"` (frozen; measured).
macro_rules! PRIdLEAST64 {
  () => { concat! ("ll", "d") };
}

// C 272: `#  define PRIiLEAST64 PRIi64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_priileast64")]
macro_rules! PRIiLEAST64 {
  () => { PRIi64! () };
}
/// `PRIiLEAST64` is `"ll" "i"` (frozen; measured).
macro_rules! PRIiLEAST64 {
  () => { concat! ("ll", "i") };
}

// C 277: `#  define PRIoLEAST64 PRIo64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prioleast64")]
macro_rules! PRIoLEAST64 {
  () => { PRIo64! () };
}
/// `PRIoLEAST64` is `"ll" "o"` (frozen; measured).
macro_rules! PRIoLEAST64 {
  () => { concat! ("ll", "o") };
}

// C 280: `#  define PRIuLEAST64 PRIu64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_priuleast64")]
macro_rules! PRIuLEAST64 {
  () => { PRIu64! () };
}
/// `PRIuLEAST64` is `"ll" "u"` (frozen; measured).
macro_rules! PRIuLEAST64 {
  () => { concat! ("ll", "u") };
}

// C 283: `#  define PRIxLEAST64 PRIx64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prixleast64")]
macro_rules! PRIxLEAST64 {
  () => { PRIx64! () };
}
/// `PRIxLEAST64` is `"ll" "x"` (frozen; measured).
macro_rules! PRIxLEAST64 {
  () => { concat! ("ll", "x") };
}

// C 286: `#  define PRIXLEAST64 PRIX64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prixleast64")]
macro_rules! PRIXLEAST64 {
  () => { PRIX64! () };
}
/// `PRIXLEAST64` is `"ll" "X"` (frozen; measured).
macro_rules! PRIXLEAST64 {
  () => { concat! ("ll", "X") };
}

// C 294: `#  define PRIdFAST8 "d"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_pridfast8")]
pub const PRIdFAST8: &core::ffi::CStr = c"d";
/// `PRIdFAST8` is `"hh" "d"` (frozen; measured).
macro_rules! PRIdFAST8 {
  () => { concat! ("hh", "d") };
}

// C 301: `#  define PRIiFAST8 "i"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priifast8")]
pub const PRIiFAST8: &core::ffi::CStr = c"i";
/// `PRIiFAST8` is `"hh" "i"` (frozen; measured).
macro_rules! PRIiFAST8 {
  () => { concat! ("hh", "i") };
}

// C 308: `#  define PRIoFAST8 "o"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priofast8")]
pub const PRIoFAST8: &core::ffi::CStr = c"o";
/// `PRIoFAST8` is `"hh" "o"` (frozen; measured).
macro_rules! PRIoFAST8 {
  () => { concat! ("hh", "o") };
}

// C 315: `#  define PRIuFAST8 "u"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priufast8")]
pub const PRIuFAST8: &core::ffi::CStr = c"u";
/// `PRIuFAST8` is `"hh" "u"` (frozen; measured).
macro_rules! PRIuFAST8 {
  () => { concat! ("hh", "u") };
}

// C 322: `#  define PRIxFAST8 "x"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixfast8")]
pub const PRIxFAST8: &core::ffi::CStr = c"x";
/// `PRIxFAST8` is `"hh" "x"` (frozen; measured).
macro_rules! PRIxFAST8 {
  () => { concat! ("hh", "x") };
}

// C 329: `#  define PRIXFAST8 "X"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixfast8")]
pub const PRIXFAST8: &core::ffi::CStr = c"X";
/// `PRIXFAST8` is `"hh" "X"` (frozen; measured).
macro_rules! PRIXFAST8 {
  () => { concat! ("hh", "X") };
}

// C 336: `#  define PRIdFAST16 "d"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_pridfast16")]
pub const PRIdFAST16: &core::ffi::CStr = c"d";
/// `PRIdFAST16` is `"hd"` (frozen; measured).
macro_rules! PRIdFAST16 {
  () => { concat! ("hd") };
}

// C 343: `#  define PRIiFAST16 "i"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priifast16")]
pub const PRIiFAST16: &core::ffi::CStr = c"i";
/// `PRIiFAST16` is `"hi"` (frozen; measured).
macro_rules! PRIiFAST16 {
  () => { concat! ("hi") };
}

// C 350: `#  define PRIoFAST16 "o"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priofast16")]
pub const PRIoFAST16: &core::ffi::CStr = c"o";
/// `PRIoFAST16` is `"ho"` (frozen; measured).
macro_rules! PRIoFAST16 {
  () => { concat! ("ho") };
}

// C 357: `#  define PRIuFAST16 "u"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priufast16")]
pub const PRIuFAST16: &core::ffi::CStr = c"u";
/// `PRIuFAST16` is `"hu"` (frozen; measured).
macro_rules! PRIuFAST16 {
  () => { concat! ("hu") };
}

// C 364: `#  define PRIxFAST16 "x"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixfast16")]
pub const PRIxFAST16: &core::ffi::CStr = c"x";
/// `PRIxFAST16` is `"hx"` (frozen; measured).
macro_rules! PRIxFAST16 {
  () => { concat! ("hx") };
}

// C 371: `#  define PRIXFAST16 "X"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixfast16")]
pub const PRIXFAST16: &core::ffi::CStr = c"X";
/// `PRIXFAST16` is `"hX"` (frozen; measured).
macro_rules! PRIXFAST16 {
  () => { concat! ("hX") };
}

// C 378: `#  define PRIdFAST32 "d"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIdFAST32` is `c"d"` (frozen; measured).
pub const PRIdFAST32: &core::ffi::CStr = c"d";

// C 385: `#  define PRIiFAST32 "i"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIiFAST32` is `c"i"` (frozen; measured).
pub const PRIiFAST32: &core::ffi::CStr = c"i";

// C 392: `#  define PRIoFAST32 "o"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIoFAST32` is `c"o"` (frozen; measured).
pub const PRIoFAST32: &core::ffi::CStr = c"o";

// C 399: `#  define PRIuFAST32 "u"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIuFAST32` is `c"u"` (frozen; measured).
pub const PRIuFAST32: &core::ffi::CStr = c"u";

// C 406: `#  define PRIxFAST32 "x"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIxFAST32` is `c"x"` (frozen; measured).
pub const PRIxFAST32: &core::ffi::CStr = c"x";

// C 413: `#  define PRIXFAST32 "X"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `PRIXFAST32` is `c"X"` (frozen; measured).
pub const PRIXFAST32: &core::ffi::CStr = c"X";

// C 418: `#  define PRIdFAST64 PRId64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_pridfast64")]
macro_rules! PRIdFAST64 {
  () => { PRId64! () };
}
/// `PRIdFAST64` is `"ll" "d"` (frozen; measured).
macro_rules! PRIdFAST64 {
  () => { concat! ("ll", "d") };
}

// C 421: `#  define PRIiFAST64 PRIi64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_priifast64")]
macro_rules! PRIiFAST64 {
  () => { PRIi64! () };
}
/// `PRIiFAST64` is `"ll" "i"` (frozen; measured).
macro_rules! PRIiFAST64 {
  () => { concat! ("ll", "i") };
}

// C 426: `#  define PRIoFAST64 PRIo64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_priofast64")]
macro_rules! PRIoFAST64 {
  () => { PRIo64! () };
}
/// `PRIoFAST64` is `"ll" "o"` (frozen; measured).
macro_rules! PRIoFAST64 {
  () => { concat! ("ll", "o") };
}

// C 429: `#  define PRIuFAST64 PRIu64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_priufast64")]
macro_rules! PRIuFAST64 {
  () => { PRIu64! () };
}
/// `PRIuFAST64` is `"ll" "u"` (frozen; measured).
macro_rules! PRIuFAST64 {
  () => { concat! ("ll", "u") };
}

// C 432: `#  define PRIxFAST64 PRIx64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prixfast64")]
macro_rules! PRIxFAST64 {
  () => { PRIx64! () };
}
/// `PRIxFAST64` is `"ll" "x"` (frozen; measured).
macro_rules! PRIxFAST64 {
  () => { concat! ("ll", "x") };
}

// C 435: `#  define PRIXFAST64 PRIX64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prixfast64")]
macro_rules! PRIXFAST64 {
  () => { PRIX64! () };
}
/// `PRIXFAST64` is `"ll" "X"` (frozen; measured).
macro_rules! PRIXFAST64 {
  () => { concat! ("ll", "X") };
}

// C 443: `#  define PRIdMAX "ld"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_pridmax")]
pub const PRIdMAX: &core::ffi::CStr = c"ld";
/// `PRIdMAX` is `"j" "d"` (frozen; measured).
macro_rules! PRIdMAX {
  () => { concat! ("j", "d") };
}

// C 450: `#  define PRIiMAX "li"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priimax")]
pub const PRIiMAX: &core::ffi::CStr = c"li";
/// `PRIiMAX` is `"j" "i"` (frozen; measured).
macro_rules! PRIiMAX {
  () => { concat! ("j", "i") };
}

// C 457: `#  define PRIoMAX "lo"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priomax")]
pub const PRIoMAX: &core::ffi::CStr = c"lo";
/// `PRIoMAX` is `"j" "o"` (frozen; measured).
macro_rules! PRIoMAX {
  () => { concat! ("j", "o") };
}

// C 464: `#  define PRIuMAX "lu"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_priumax")]
pub const PRIuMAX: &core::ffi::CStr = c"lu";
/// `PRIuMAX` is `"j" "u"` (frozen; measured).
macro_rules! PRIuMAX {
  () => { concat! ("j", "u") };
}

// C 471: `#  define PRIxMAX "lx"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixmax")]
pub const PRIxMAX: &core::ffi::CStr = c"lx";
/// `PRIxMAX` is `"j" "x"` (frozen; measured).
macro_rules! PRIxMAX {
  () => { concat! ("j", "x") };
}

// C 478: `#  define PRIXMAX "lX"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_prixmax")]
pub const PRIXMAX: &core::ffi::CStr = c"lX";
/// `PRIXMAX` is `"j" "X"` (frozen; measured).
macro_rules! PRIXMAX {
  () => { concat! ("j", "X") };
}

// C 484: `#  define PRIdPTR @PRIPTR_PREFIX@ "d"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_pridptr")]
macro_rules! PRIdPTR {
  () => { concat! (PRIPTR_PREFIX! (), "d") };
}
/// `PRIdPTR` is `"ld"` (frozen; measured).
macro_rules! PRIdPTR {
  () => { concat! ("ld") };
}

// C 489: `#  define PRIiPTR @PRIPTR_PREFIX@ "i"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_priiptr")]
macro_rules! PRIiPTR {
  () => { concat! (PRIPTR_PREFIX! (), "i") };
}
/// `PRIiPTR` is `"li"` (frozen; measured).
macro_rules! PRIiPTR {
  () => { concat! ("li") };
}

// C 494: `#  define PRIoPTR @PRIPTR_PREFIX@ "o"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prioptr")]
macro_rules! PRIoPTR {
  () => { concat! (PRIPTR_PREFIX! (), "o") };
}
/// `PRIoPTR` is `"lo"` (frozen; measured).
macro_rules! PRIoPTR {
  () => { concat! ("lo") };
}

// C 499: `#  define PRIuPTR @PRIPTR_PREFIX@ "u"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_priuptr")]
macro_rules! PRIuPTR {
  () => { concat! (PRIPTR_PREFIX! (), "u") };
}
/// `PRIuPTR` is `"lu"` (frozen; measured).
macro_rules! PRIuPTR {
  () => { concat! ("lu") };
}

// C 504: `#  define PRIxPTR @PRIPTR_PREFIX@ "x"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prixptr")]
macro_rules! PRIxPTR {
  () => { concat! (PRIPTR_PREFIX! (), "x") };
}
/// `PRIxPTR` is `"lx"` (frozen; measured).
macro_rules! PRIxPTR {
  () => { concat! ("lx") };
}

// C 509: `#  define PRIXPTR @PRIPTR_PREFIX@ "X"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_prixptr")]
macro_rules! PRIXPTR {
  () => { concat! (PRIPTR_PREFIX! (), "X") };
}
/// `PRIXPTR` is `"lX"` (frozen; measured).
macro_rules! PRIXPTR {
  () => { concat! ("lX") };
}

// C 515: `#  define SCNd8 "hhd"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNd8` is `c"hhd"` (frozen; measured).
pub const SCNd8: &core::ffi::CStr = c"hhd";

// C 520: `#  define SCNi8 "hhi"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNi8` is `c"hhi"` (frozen; measured).
pub const SCNi8: &core::ffi::CStr = c"hhi";

// C 525: `#  define SCNo8 "hho"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNo8` is `c"hho"` (frozen; measured).
pub const SCNo8: &core::ffi::CStr = c"hho";

// C 530: `#  define SCNu8 "hhu"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNu8` is `c"hhu"` (frozen; measured).
pub const SCNu8: &core::ffi::CStr = c"hhu";

// C 535: `#  define SCNx8 "hhx"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNx8` is `c"hhx"` (frozen; measured).
pub const SCNx8: &core::ffi::CStr = c"hhx";

// C 540: `#  define SCNd16 "hd"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNd16` is `c"hd"` (frozen; measured).
pub const SCNd16: &core::ffi::CStr = c"hd";

// C 545: `#  define SCNi16 "hi"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNi16` is `c"hi"` (frozen; measured).
pub const SCNi16: &core::ffi::CStr = c"hi";

// C 550: `#  define SCNo16 "ho"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNo16` is `c"ho"` (frozen; measured).
pub const SCNo16: &core::ffi::CStr = c"ho";

// C 555: `#  define SCNu16 "hu"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNu16` is `c"hu"` (frozen; measured).
pub const SCNu16: &core::ffi::CStr = c"hu";

// C 560: `#  define SCNx16 "hx"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNx16` is `c"hx"` (frozen; measured).
pub const SCNx16: &core::ffi::CStr = c"hx";

// C 565: `#  define SCNd32 "d"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNd32` is `c"d"` (frozen; measured).
pub const SCNd32: &core::ffi::CStr = c"d";

// C 570: `#  define SCNi32 "i"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNi32` is `c"i"` (frozen; measured).
pub const SCNi32: &core::ffi::CStr = c"i";

// C 575: `#  define SCNo32 "o"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNo32` is `c"o"` (frozen; measured).
pub const SCNo32: &core::ffi::CStr = c"o";

// C 580: `#  define SCNu32 "u"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNu32` is `c"u"` (frozen; measured).
pub const SCNu32: &core::ffi::CStr = c"u";

// C 585: `#  define SCNx32 "x"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNx32` is `c"x"` (frozen; measured).
pub const SCNx32: &core::ffi::CStr = c"x";

// C 594: `#  define _SCN64_PREFIX _LONG_LONG_FORMAT_PREFIX` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_scn64_prefix")]
macro_rules! _SCN64_PREFIX {
  () => { _LONG_LONG_FORMAT_PREFIX! () };
}

// C 597: `#  define SCNd64 _SCN64_PREFIX "d"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnd64")]
macro_rules! SCNd64 {
  () => { concat! (_SCN64_PREFIX! (), "d") };
}
/// `SCNd64` is `"ll" "d"` (frozen; measured).
macro_rules! SCNd64 {
  () => { concat! ("ll", "d") };
}

// C 600: `#  define SCNi64 _SCN64_PREFIX "i"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scni64")]
macro_rules! SCNi64 {
  () => { concat! (_SCN64_PREFIX! (), "i") };
}
/// `SCNi64` is `"ll" "i"` (frozen; measured).
macro_rules! SCNi64 {
  () => { concat! ("ll", "i") };
}

// C 609: `#  define _SCNu64_PREFIX _LONG_LONG_FORMAT_PREFIX` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_scnu64_prefix")]
macro_rules! _SCNu64_PREFIX {
  () => { _LONG_LONG_FORMAT_PREFIX! () };
}

// C 612: `#  define SCNo64 _SCNu64_PREFIX "o"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scno64")]
macro_rules! SCNo64 {
  () => { concat! (_SCNu64_PREFIX! (), "o") };
}
/// `SCNo64` is `"ll" "o"` (frozen; measured).
macro_rules! SCNo64 {
  () => { concat! ("ll", "o") };
}

// C 615: `#  define SCNu64 _SCNu64_PREFIX "u"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnu64")]
macro_rules! SCNu64 {
  () => { concat! (_SCNu64_PREFIX! (), "u") };
}
/// `SCNu64` is `"ll" "u"` (frozen; measured).
macro_rules! SCNu64 {
  () => { concat! ("ll", "u") };
}

// C 618: `#  define SCNx64 _SCNu64_PREFIX "x"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnx64")]
macro_rules! SCNx64 {
  () => { concat! (_SCNu64_PREFIX! (), "x") };
}
/// `SCNx64` is `"ll" "x"` (frozen; measured).
macro_rules! SCNx64 {
  () => { concat! ("ll", "x") };
}

// C 623: `# define SCNdLEAST8 "hhd"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNdLEAST8` is `c"hhd"` (frozen; measured).
pub const SCNdLEAST8: &core::ffi::CStr = c"hhd";

// C 626: `# define SCNiLEAST8 "hhi"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNiLEAST8` is `c"hhi"` (frozen; measured).
pub const SCNiLEAST8: &core::ffi::CStr = c"hhi";

// C 629: `# define SCNoLEAST8 "hho"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNoLEAST8` is `c"hho"` (frozen; measured).
pub const SCNoLEAST8: &core::ffi::CStr = c"hho";

// C 632: `# define SCNuLEAST8 "hhu"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNuLEAST8` is `c"hhu"` (frozen; measured).
pub const SCNuLEAST8: &core::ffi::CStr = c"hhu";

// C 635: `# define SCNxLEAST8 "hhx"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNxLEAST8` is `c"hhx"` (frozen; measured).
pub const SCNxLEAST8: &core::ffi::CStr = c"hhx";

// C 638: `# define SCNdLEAST16 "hd"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNdLEAST16` is `c"hd"` (frozen; measured).
pub const SCNdLEAST16: &core::ffi::CStr = c"hd";

// C 641: `# define SCNiLEAST16 "hi"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNiLEAST16` is `c"hi"` (frozen; measured).
pub const SCNiLEAST16: &core::ffi::CStr = c"hi";

// C 644: `# define SCNoLEAST16 "ho"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNoLEAST16` is `c"ho"` (frozen; measured).
pub const SCNoLEAST16: &core::ffi::CStr = c"ho";

// C 647: `# define SCNuLEAST16 "hu"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNuLEAST16` is `c"hu"` (frozen; measured).
pub const SCNuLEAST16: &core::ffi::CStr = c"hu";

// C 650: `# define SCNxLEAST16 "hx"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNxLEAST16` is `c"hx"` (frozen; measured).
pub const SCNxLEAST16: &core::ffi::CStr = c"hx";

// C 653: `# define SCNdLEAST32 "d"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNdLEAST32` is `c"d"` (frozen; measured).
pub const SCNdLEAST32: &core::ffi::CStr = c"d";

// C 656: `# define SCNiLEAST32 "i"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNiLEAST32` is `c"i"` (frozen; measured).
pub const SCNiLEAST32: &core::ffi::CStr = c"i";

// C 659: `# define SCNoLEAST32 "o"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNoLEAST32` is `c"o"` (frozen; measured).
pub const SCNoLEAST32: &core::ffi::CStr = c"o";

// C 662: `# define SCNuLEAST32 "u"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNuLEAST32` is `c"u"` (frozen; measured).
pub const SCNuLEAST32: &core::ffi::CStr = c"u";

// C 665: `# define SCNxLEAST32 "x"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNxLEAST32` is `c"x"` (frozen; measured).
pub const SCNxLEAST32: &core::ffi::CStr = c"x";

// C 669: `#  define SCNdLEAST64 SCNd64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scndleast64")]
macro_rules! SCNdLEAST64 {
  () => { SCNd64! () };
}
/// `SCNdLEAST64` is `"ll" "d"` (frozen; measured).
macro_rules! SCNdLEAST64 {
  () => { concat! ("ll", "d") };
}

// C 672: `#  define SCNiLEAST64 SCNi64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnileast64")]
macro_rules! SCNiLEAST64 {
  () => { SCNi64! () };
}
/// `SCNiLEAST64` is `"ll" "i"` (frozen; measured).
macro_rules! SCNiLEAST64 {
  () => { concat! ("ll", "i") };
}

// C 677: `#  define SCNoLEAST64 SCNo64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnoleast64")]
macro_rules! SCNoLEAST64 {
  () => { SCNo64! () };
}
/// `SCNoLEAST64` is `"ll" "o"` (frozen; measured).
macro_rules! SCNoLEAST64 {
  () => { concat! ("ll", "o") };
}

// C 680: `#  define SCNuLEAST64 SCNu64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnuleast64")]
macro_rules! SCNuLEAST64 {
  () => { SCNu64! () };
}
/// `SCNuLEAST64` is `"ll" "u"` (frozen; measured).
macro_rules! SCNuLEAST64 {
  () => { concat! ("ll", "u") };
}

// C 683: `#  define SCNxLEAST64 SCNx64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnxleast64")]
macro_rules! SCNxLEAST64 {
  () => { SCNx64! () };
}
/// `SCNxLEAST64` is `"ll" "x"` (frozen; measured).
macro_rules! SCNxLEAST64 {
  () => { concat! ("ll", "x") };
}

// C 695: `#  define SCNdFAST8 "d"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scndfast8")]
pub const SCNdFAST8: &core::ffi::CStr = c"d";
/// `SCNdFAST8` is `"hh" "d"` (frozen; measured).
macro_rules! SCNdFAST8 {
  () => { concat! ("hh", "d") };
}

// C 706: `#  define SCNiFAST8 "i"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnifast8")]
pub const SCNiFAST8: &core::ffi::CStr = c"i";
/// `SCNiFAST8` is `"hh" "i"` (frozen; measured).
macro_rules! SCNiFAST8 {
  () => { concat! ("hh", "i") };
}

// C 717: `#  define SCNoFAST8 "o"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnofast8")]
pub const SCNoFAST8: &core::ffi::CStr = c"o";
/// `SCNoFAST8` is `"hh" "o"` (frozen; measured).
macro_rules! SCNoFAST8 {
  () => { concat! ("hh", "o") };
}

// C 728: `#  define SCNuFAST8 "u"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnufast8")]
pub const SCNuFAST8: &core::ffi::CStr = c"u";
/// `SCNuFAST8` is `"hh" "u"` (frozen; measured).
macro_rules! SCNuFAST8 {
  () => { concat! ("hh", "u") };
}

// C 739: `#  define SCNxFAST8 "x"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnxfast8")]
pub const SCNxFAST8: &core::ffi::CStr = c"x";
/// `SCNxFAST8` is `"hh" "x"` (frozen; measured).
macro_rules! SCNxFAST8 {
  () => { concat! ("hh", "x") };
}

// C 748: `#  define SCNdFAST16 "d"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scndfast16")]
pub const SCNdFAST16: &core::ffi::CStr = c"d";
/// `SCNdFAST16` is `"hd"` (frozen; measured).
macro_rules! SCNdFAST16 {
  () => { concat! ("hd") };
}

// C 757: `#  define SCNiFAST16 "i"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnifast16")]
pub const SCNiFAST16: &core::ffi::CStr = c"i";
/// `SCNiFAST16` is `"hi"` (frozen; measured).
macro_rules! SCNiFAST16 {
  () => { concat! ("hi") };
}

// C 766: `#  define SCNoFAST16 "o"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnofast16")]
pub const SCNoFAST16: &core::ffi::CStr = c"o";
/// `SCNoFAST16` is `"ho"` (frozen; measured).
macro_rules! SCNoFAST16 {
  () => { concat! ("ho") };
}

// C 775: `#  define SCNuFAST16 "u"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnufast16")]
pub const SCNuFAST16: &core::ffi::CStr = c"u";
/// `SCNuFAST16` is `"hu"` (frozen; measured).
macro_rules! SCNuFAST16 {
  () => { concat! ("hu") };
}

// C 784: `#  define SCNxFAST16 "x"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnxfast16")]
pub const SCNxFAST16: &core::ffi::CStr = c"x";
/// `SCNxFAST16` is `"hx"` (frozen; measured).
macro_rules! SCNxFAST16 {
  () => { concat! ("hx") };
}

// C 791: `#  define SCNdFAST32 "d"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNdFAST32` is `c"d"` (frozen; measured).
pub const SCNdFAST32: &core::ffi::CStr = c"d";

// C 798: `#  define SCNiFAST32 "i"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNiFAST32` is `c"i"` (frozen; measured).
pub const SCNiFAST32: &core::ffi::CStr = c"i";

// C 805: `#  define SCNoFAST32 "o"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNoFAST32` is `c"o"` (frozen; measured).
pub const SCNoFAST32: &core::ffi::CStr = c"o";

// C 812: `#  define SCNuFAST32 "u"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNuFAST32` is `c"u"` (frozen; measured).
pub const SCNuFAST32: &core::ffi::CStr = c"u";

// C 819: `#  define SCNxFAST32 "x"` - the frozen value is the same, so one ungated constant answers for both arms.
/// `SCNxFAST32` is `c"x"` (frozen; measured).
pub const SCNxFAST32: &core::ffi::CStr = c"x";

// C 824: `#  define SCNdFAST64 SCNd64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scndfast64")]
macro_rules! SCNdFAST64 {
  () => { SCNd64! () };
}
/// `SCNdFAST64` is `"ll" "d"` (frozen; measured).
macro_rules! SCNdFAST64 {
  () => { concat! ("ll", "d") };
}

// C 827: `#  define SCNiFAST64 SCNi64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnifast64")]
macro_rules! SCNiFAST64 {
  () => { SCNi64! () };
}
/// `SCNiFAST64` is `"ll" "i"` (frozen; measured).
macro_rules! SCNiFAST64 {
  () => { concat! ("ll", "i") };
}

// C 832: `#  define SCNoFAST64 SCNo64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnofast64")]
macro_rules! SCNoFAST64 {
  () => { SCNo64! () };
}
/// `SCNoFAST64` is `"ll" "o"` (frozen; measured).
macro_rules! SCNoFAST64 {
  () => { concat! ("ll", "o") };
}

// C 835: `#  define SCNuFAST64 SCNu64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnufast64")]
macro_rules! SCNuFAST64 {
  () => { SCNu64! () };
}
/// `SCNuFAST64` is `"ll" "u"` (frozen; measured).
macro_rules! SCNuFAST64 {
  () => { concat! ("ll", "u") };
}

// C 838: `#  define SCNxFAST64 SCNx64` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnxfast64")]
macro_rules! SCNxFAST64 {
  () => { SCNx64! () };
}
/// `SCNxFAST64` is `"ll" "x"` (frozen; measured).
macro_rules! SCNxFAST64 {
  () => { concat! ("ll", "x") };
}

// C 846: `#  define SCNdMAX "ld"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scndmax")]
pub const SCNdMAX: &core::ffi::CStr = c"ld";
/// `SCNdMAX` is `"j" "d"` (frozen; measured).
macro_rules! SCNdMAX {
  () => { concat! ("j", "d") };
}

// C 853: `#  define SCNiMAX "li"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnimax")]
pub const SCNiMAX: &core::ffi::CStr = c"li";
/// `SCNiMAX` is `"j" "i"` (frozen; measured).
macro_rules! SCNiMAX {
  () => { concat! ("j", "i") };
}

// C 860: `#  define SCNoMAX "lo"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnomax")]
pub const SCNoMAX: &core::ffi::CStr = c"lo";
/// `SCNoMAX` is `"j" "o"` (frozen; measured).
macro_rules! SCNoMAX {
  () => { concat! ("j", "o") };
}

// C 867: `#  define SCNuMAX "lu"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnumax")]
pub const SCNuMAX: &core::ffi::CStr = c"lu";
/// `SCNuMAX` is `"j" "u"` (frozen; measured).
macro_rules! SCNuMAX {
  () => { concat! ("j", "u") };
}

// C 874: `#  define SCNxMAX "lx"` - the template's fallback; C's value is the gated constant, the frozen spelling the macro.
#[cfg (feature = "no_scnxmax")]
pub const SCNxMAX: &core::ffi::CStr = c"lx";
/// `SCNxMAX` is `"j" "x"` (frozen; measured).
macro_rules! SCNxMAX {
  () => { concat! ("j", "x") };
}

// C 880: `#  define SCNdPTR @PRIPTR_PREFIX@ "d"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scndptr")]
macro_rules! SCNdPTR {
  () => { concat! (PRIPTR_PREFIX! (), "d") };
}
/// `SCNdPTR` is `"ld"` (frozen; measured).
macro_rules! SCNdPTR {
  () => { concat! ("ld") };
}

// C 885: `#  define SCNiPTR @PRIPTR_PREFIX@ "i"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scniptr")]
macro_rules! SCNiPTR {
  () => { concat! (PRIPTR_PREFIX! (), "i") };
}
/// `SCNiPTR` is `"li"` (frozen; measured).
macro_rules! SCNiPTR {
  () => { concat! ("li") };
}

// C 890: `#  define SCNoPTR @PRIPTR_PREFIX@ "o"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnoptr")]
macro_rules! SCNoPTR {
  () => { concat! (PRIPTR_PREFIX! (), "o") };
}
/// `SCNoPTR` is `"lo"` (frozen; measured).
macro_rules! SCNoPTR {
  () => { concat! ("lo") };
}

// C 895: `#  define SCNuPTR @PRIPTR_PREFIX@ "u"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnuptr")]
macro_rules! SCNuPTR {
  () => { concat! (PRIPTR_PREFIX! (), "u") };
}
/// `SCNuPTR` is `"lu"` (frozen; measured).
macro_rules! SCNuPTR {
  () => { concat! ("lu") };
}

// C 900: `#  define SCNxPTR @PRIPTR_PREFIX@ "x"` - the template's fallback; C's body is the gated macro, the frozen spelling the macro.
#[cfg (feature = "no_scnxptr")]
macro_rules! SCNxPTR {
  () => { concat! (PRIPTR_PREFIX! (), "x") };
}
/// `SCNxPTR` is `"lx"` (frozen; measured).
macro_rules! SCNxPTR {
  () => { concat! ("lx") };
}

// C 914: `#   define imaxabs rpl_imaxabs` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_imaxabs")]
macro_rules! imaxabs {
  () => { rpl_imaxabs! () };
}

// C 938: `#   define GNULIB_defined_imaxdiv_t 1` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_gnulib_defined_imaxdiv_t")]
pub const GNULIB_defined_imaxdiv_t: c_int = 1;

// C 944: `#   define imaxdiv rpl_imaxdiv` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_imaxdiv")]
macro_rules! imaxdiv {
  () => { rpl_imaxdiv! () };
}

// C 968: `#   define strtoimax rpl_strtoimax` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_strtoimax")]
macro_rules! strtoimax {
  () => { rpl_strtoimax! () };
}

// C 997: `#   define strtoumax rpl_strtoumax` - dead, and the frozen `<inttypes.h>` does not define it either.
#[cfg (feature = "no_strtoumax")]
macro_rules! strtoumax {
  () => { rpl_strtoumax! () };
}
