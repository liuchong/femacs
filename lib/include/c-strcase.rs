// femacs: port of lib/c-strcase.h @ fd7afced403 (gnulib's <c-strcase.h>; PORTING.md P-26).
//
// The header is not a template, so the criterion is the header itself.  Its conditions are
// all decided by the frozen configuration's compiler, and the extractor's item list is the
// guard alone (`## Functions defined — 0` counts definitions, not declarations):
//
//   * C 22-24's `#if !_GL_CONFIG_H_INCLUDED / #error` is dead: the configuration's
//     `config.h` defines `_GL_CONFIG_H_INCLUDED` (`docs/baseline/config/darwin-arm64.macros.tsv`),
//     which is what the check asks about.  The `#error` has no Rust counterpart.
//   * C 39-41's `extern "C" {` and C 56-58's `}` are dead (`__cplusplus` is undefined for a C
//     translation unit).
//   * C 26's `#include <stddef.h>` is live, and `size_t` is the one name the live text takes
//     from it.  PORTING.md 1.1 maps it to `usize`, which is how the other `lib/` mirrors spell
//     it in a declaration (`string.rs` 32/36, `unistd.in.rs` 39), so no `<stddef.h>` mirror is
//     needed and C's spelling stays in the `/// C:` line.
//
// `_GL_ATTRIBUTE_PURE` on both declarations is `attribute.rs`'s no-op: it is the
// `__attribute__ ((__pure__))` purity hint, whose port counterpart is the absence of the
// attribute (PORTING.md 8), so the token stays in each `/// C:` line and the declaration is
// plain.  The two declarations sit in a module with a glob import (PORTING.md 0.1-10/P-17/P-21)
// because the units that *define* them - `lib/c-strcasecmp.c` and `lib/c-strncasecmp.c` - are
// ported (`lib/c-strcasecmp.rs`, `lib/c-strncasecmp.rs`) and include this
// mirror: a definition in the unit then shadows the declaration instead of colliding with it.

/* C 18-19: the include guard has no Rust counterpart (PORTING.md 0.1-4).  */
pub const C_STRCASE_H: bool = true;

/* C 44-53: the two entry points.  C's parameters are named, so the port keeps C's names
   (`haiku_support.rs`'s `arg0`/`arg1` is for the unnamed ones).  */
pub mod c_strcase_decls {
  use super::*;

  unsafe extern "C" {
    /// C: extern int c_strcasecmp (const char *s1, const char *s2) _GL_ATTRIBUTE_PURE;
    pub fn c_strcasecmp (s1: *const c_char, s2: *const c_char) -> c_int;
    /// C: extern int c_strncasecmp (const char *s1, const char *s2, size_t n)
    ///      _GL_ATTRIBUTE_PURE;
    pub fn c_strncasecmp (s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
  }
}
use c_strcase_decls::*;

/* C 29-36's comment on the "C" locale and C 60's `#endif` have no Rust counterpart
   (PORTING.md 0.1).  */
