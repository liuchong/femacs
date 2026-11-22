// femacs: port of lib/stdckdint.in.h @ fd7afced403 (gnulib's <stdckdint.h>; PORTING.md P-26).
#![forbid(unsafe_code)]
//
// This target generates no such header.  gnulib substitutes the template only when
// `GL_GENERATE_STDCKDINT_H_CONDITION` is non-empty and otherwise removes the file; the frozen
// configuration's value is empty, so the criterion is the system `<stdckdint.h>` that C 41's
// `# @INCLUDE_NEXT@ @NEXT_STDCKDINT_H@` reaches, and every name the template defines itself is
// dead text (the `errno.in.rs` situation, with the measurement here):
//
//     $ grep -n STDCKDINT build/emacs-ref/config.status build/emacs-ref/lib/gnulib.mk | head -3
//     build/emacs-ref/config.status:820:S["GL_GENERATE_STDCKDINT_H_CONDITION"]=""
//     build/emacs-ref/lib/gnulib.mk:374:GL_GENERATE_STDCKDINT_H_CONDITION =
//     $ ls build/emacs-ref/lib/stdckdint.h
//     ls: build/emacs-ref/lib/stdckdint.h: No such file or directory
//
// and the substitutions the template's three conditions would take, from the same file:
// `@HAVE_C_STDCKDINT_H@` = `@HAVE_WORKING_C_STDCKDINT_H@` = 1, `@HAVE_CXX_STDCKDINT_H@` =
// `@HAVE_WORKING_CXX_STDCKDINT_H@` = "" (C++ is not this configuration), so C 26's test is
// `#if 1` (the include_next is live) and C 34's is `#if !1` (C 36-54 is dead).  The frozen
// header itself, measured with the configuration's compiler:
//
//     $ printf '' > /tmp/empty.c
//     $ clang -std=gnu23 -E -dM -include stdckdint.h /tmp/empty.c | grep -E 'ckd_|STDC_VERSION_STDCKDINT'
//     #define __STDC_VERSION_STDCKDINT_H__ 202311L
//     #define ckd_add(R,A,B) __builtin_add_overflow((A), (B), (R))
//     #define ckd_sub(R,A,B) __builtin_sub_overflow((A), (B), (R))
//     #define ckd_mul(R,A,B) __builtin_mul_overflow((A), (B), (R))
//
// So this mirror is the guard plus the three dead definitions, each under the feature that
// names the condition that would take it (P-51), and the frozen arm of each name.  The port
// spells a `ckd_*` call with the `overflowing_*` pair and a write-back at the site - the
// mapping the ported units record for C 52-54's callers (`lib/timespec-add.rs` 33-38,
// `lib/mktime.rs` 36's note, `src/doprnt.rs` 83) - and no ported unit names
// `ckd_add`/`ckd_sub`/`ckd_mul` (measured: `rg -n "\bckd_(add|sub|mul)" ` finds only
// comments and this mirror), so the frozen arm is the marker that says so; the marker is a
// `const` because `rustc` compiles the ungated definition while the extractor compares the
// first one (the `gnulib_config.rs` `_GL_INLINE` marker shape).
//
// C's arms are dead text but their bodies are compared (each name is defined once in the
// template), so each names C's own `_GL_INT_*_WRAPV` - `lib/intprops-internal.h` (C 41's
// include) is still `todo` in `docs/port-inventory.md`, and that is the one name the arm's
// tokens need.

// [U1-MODULE-MIRROR]
use super::*;
/* C 30-31: `_@GUARD_PREFIX@_STDCKDINT_H`, whose substitution is `_GL_STDCKDINT_H`.  The
   template's own spelling reads as `_` to the extractor, which
   `docs/ports/excluded-macros.txt` records.  */
pub const _GL_STDCKDINT_H: c_int = 1;

/* C 36-41: `#  undef ckd_add`/`ckd_sub`/`ckd_mul` and `#  include "intprops-internal.h"` -
   the undefs are what C's own definitions below replace, and the include is the dead arm's
   dependency (recorded here; the header has no mirror yet).  */

/* C 52-54: the three definitions, dead on this target (C 34's `# if !@HAVE_WORKING_C_STDCKDINT_H@`
   is `#if !1`).  C's text is `((bool) _GL_INT_*_WRAPV (a, b, r))`: the gnulib macro stores the
   wrapped result and the `(bool)` carries the overflow flag out.  */
#[cfg (feature = "no_working_ckdint")]
macro_rules! ckd_add {
  ($r:expr, $a:expr, $b:expr) => { (_GL_INT_ADD_WRAPV! ($a, $b, $r)) as bool };
}
#[cfg (feature = "no_working_ckdint")]
macro_rules! ckd_sub {
  ($r:expr, $a:expr, $b:expr) => { (_GL_INT_SUBTRACT_WRAPV! ($a, $b, $r)) as bool };
}
#[cfg (feature = "no_working_ckdint")]
macro_rules! ckd_mul {
  ($r:expr, $a:expr, $b:expr) => { (_GL_INT_MULTIPLY_WRAPV! ($a, $b, $r)) as bool };
}

/* The frozen arm of each name: this target's `<stdckdint.h>` defines it as clang's
   `__builtin_*_overflow ((A), (B), (R))`, whose port counterpart is the `overflowing_*` pair
   with a write-back (P-15), spelled at each call site rather than through the macro.  */
/// `ckd_add (R, A, B)` is `__builtin_add_overflow ((A), (B), (R))` (frozen); the port writes
/// `overflowing_add` and the store at the site.
pub const ckd_add: () = ();
/// `ckd_sub (R, A, B)` is `__builtin_sub_overflow ((A), (B), (R))` (frozen); the port writes
/// `overflowing_sub` and the store at the site.
pub const ckd_sub: () = ();
/// `ckd_mul (R, A, B)` is `__builtin_mul_overflow ((A), (B), (R))` (frozen); the port writes
/// `overflowing_mul` and the store at the site.
pub const ckd_mul: () = ();

/* C 18-21's `__GNUC__ >= 3` test with the two pragma placeholders, C 23/56-57's guard pair and
   C 25-31's split-guard comment have no Rust counterpart (PORTING.md 0.1, P-08).  */
