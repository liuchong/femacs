// femacs: mirror of lib/stddef.in.h @ fd7afced403.
#![forbid(unsafe_code)]
//
// Header mirror (PORTING.md 0.1, P-26).  This is a gnulib *template*: its text carries 14
// `@PLACEHOLDER@` substitutions and 34 conditions, and the extractor cannot decide the
// placeholder ones (P-66).  `lib/stddef.in.h` is not in
// docs/baseline/config/cond-eval-verified.txt, and `python3 tools/check-cond-eval.py
// lib/stddef.in.h` prints `34 conditions, 7 unknown to the evaluator, 10 disagreements [NOT
// VERIFIED]`, so `tools/extract-abi.py` reads the file blind and requires all 12 macro names.
//
// **The criterion for the live arms is the generated header**, `build/emacs-ref/lib/stddef.h`
// (249 lines, header line 1 `/* DO NOT EDIT! GENERATED AUTOMATICALLY! */`), corroborated by
// the frozen clang:
//
//     $ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
//         -I emacs/lib -include config.h -include stddef.h -x c /dev/null \
//         | grep -E '#define (NULL|_GCC_NULLPTR_T|_GL_ATTRIBUTE_NOTHROW|…)\b'
//
// The generated substitutions are `@HAVE_MAX_ALIGN_T@` 1, `@REPLACE_NULL@` 0,
// `@NULLPTR_T_NEEDS_STDDEF@` 1, `@HAVE_C_UNREACHABLE@` 1, `@STDDEF_NOT_IDEMPOTENT@` 0 and
// `@GUARD_PREFIX@` `GL_`.  So, name by name: `_GCC_NULLPTR_T`'s arm is dead (its condition
// ends `&& !1`); the whole `max_align_t` block C 152-188 (and `_GL_STDDEF_ALIGNAS`,
// `max_align_t`, `GNULIB_defined_max_align_t`, `__CLANG_MAX_ALIGN_T_DEFINED` in it) is dead
// because `HAVE_MAX_ALIGN_T` is 1, and `_MAX_ALIGN_T`'s AIX arm C 76-90 is dead because the
// target is not AIX; the `NULL` block C 102-120 is dead (`@REPLACE_NULL@` is 0 - the surviving
// `NULL ((void*)0)` in the measurement is the *system* header's); `unreachable` C 237-244 is
// dead (`@HAVE_C_UNREACHABLE@` is 1); `_GL_ATTRIBUTE_NOTHROW` C 130-150 and
// `_GL_HAS_BUILTIN_UNREACHABLE` C 204-214 are dead because the configuration already defines
// both - measured `_GL_ATTRIBUTE_NOTHROW __attribute__ ((__nothrow__))` and
// `_GL_HAS_BUILTIN_UNREACHABLE __has_builtin (__builtin_unreachable)`, and the latter is why
// `gl_unreachable` (C 216-218) is the live `__builtin_unreachable ()` arm the measurement
// shows.  The dead arms and the one dead *type* (`rpl_max_align_t`, C 176-182) are recorded in
// docs/ports/include-stddef.in.md rather than transcribed; the frozen target's own
// `max_align_t` is the system's `long double` (8 bytes, 8-aligned - measured by
// `include/scratch_buffer.rs`).
//
// Two processor facts the ledger records in full: C 30-69 is GCC's "special invocation
// convention" (`__need_wchar_t`/`__need_size_t`/... plus `#include_next`), which no Rust
// translation unit has; and the extractor sees a macro named `_` at C 123, because
// `#   define _@GUARD_PREFIX@_STDDEF_H` is cut at the `@`.  `_` already owns a row in
// docs/ports/excluded-macros.txt (for `lib/getopt.c`'s message macro), which is what silences
// that artifact; the guard's real name (`_GL_STDDEF_H`, measured defined with an empty body)
// is what the mirror spells.
//
// The mirror carries no `use`: it spells its scalars with absolute `core::ffi::` paths and
// stands as its own crate root (`build-tu lib/stddef.in.h`).  The names whose C arms are dead
// are `#[cfg (feature = ...)]` text (P-51), each feature naming the condition that would make
// the arm live, so the frozen configuration compiles none of them.

// [U1-MODULE-MIRROR]
use super::*;
/* C 20-23: the POSIX/C23 `<stddef.h>` substitute's guard.  C 72's and C 122's
   `# ifndef _@GUARD_PREFIX@_STDDEF_H` are the split double-inclusion guard the `#include_next`
   needs (C 98-100): the first is entered, the second defines the name at C 123 and opens the
   body.  No Rust counterpart (PORTING.md 0.1-4); the name stays as its marker.  */
pub const _GL_STDDEF_H: bool = true;

/* C 130-150's `#ifndef _GL_ATTRIBUTE_NOTHROW` fallback.  Dead in C - the configuration
   defines the name as the attribute, measured above - and PORTING.md 8 gives an attribute no
   Rust counterpart, so the port's spelling is the empty macro its call sites do not need, the
   same two lines `include/attribute.rs` 184 and `include/sys_stat.in.rs` 54 carry.  A later
   `macro_rules!` of the same name shadows an earlier one (measured: two definitions plus one
   use print the second body), so a unit that pulls in two of the three still compiles.  */
macro_rules! _GL_ATTRIBUTE_NOTHROW {
  () => {};
}

pub mod stddef_decls {
  use super::*;

  /* C 204-214: `_GL_HAS_BUILTIN_UNREACHABLE`'s fallback chain.  Dead in C (the configuration
     defines it), and its live value is the configuration's `__has_builtin
     (__builtin_unreachable)`, which the frozen clang answers 1 for - that is why
     `gl_unreachable` below is the `__builtin_unreachable` arm.  The port's spelling is the
     measured integer, the `include/verify.rs` 159 shape.  */
  pub const _GL_HAS_BUILTIN_UNREACHABLE: core::ffi::c_int = 1;

  /* C 118: `#     define NULL ((void *) 0)` inside `#  if (@REPLACE_NULL@ && (...))`.  Dead
     (`@REPLACE_NULL@` is 0, so C 103's condition is `(0 && (...))`); the frozen `<stddef.h>`'s
     `NULL` is the system header's `((void*)0)` (measured), and the port's spelling of a null
     pointer is `core::ptr::null*`.  */
  #[cfg (feature = "replace_null")]
  pub const NULL: *mut core::ffi::c_void = core::ptr::null_mut ();

  /* C 92-96: `#  if !defined _GCC_NULLPTR_T && !@NULLPTR_T_NEEDS_STDDEF@ / #   define
     _GCC_NULLPTR_T` - the guard that suppresses GCC's unwanted `nullptr_t` typedef.  Dead: the
     generated header's condition ends `&& !1`, and the measurement shows the name undefined. */
  #[cfg (feature = "no_nullptr_t_needs_stddef")]
  macro_rules! _GCC_NULLPTR_T { () => {}; }

  /* C 78-86: `#    ifdef _MAX_ALIGN_T / #     define _MAX_ALIGN_T` - the AIX 7.2 xlc 64-bit
     workaround's marker (`typedef long max_align_t` under it).  Dead: `defined _AIX` is false. */
  #[cfg (feature = "aix")]
  macro_rules! _MAX_ALIGN_T { () => {}; }

  /* C 176-188: the `rpl_max_align_t` fallback's pieces.  Dead: `HAVE_MAX_ALIGN_T` is 1, so the
     generated header's `#  if ! (1 || (...))` is false.  C's union (C 176-182, four members
     each carrying `_GL_STDDEF_ALIGNAS`), its `typedef`, and the `max_align_t`/marker defines
     are recorded as text in the ledger; the three *names* the gate asks for are here.  */
  #[cfg (feature = "no_have_max_align_t")]
  pub const GNULIB_defined_max_align_t: core::ffi::c_int = 1;
  #[cfg (feature = "no_have_max_align_t")]
  pub const __CLANG_MAX_ALIGN_T_DEFINED: bool = true;

  /* C 174: `#    define _GL_STDDEF_ALIGNAS(type) /* */` - the empty arm of the alignment
     spelling (the other arm is `__attribute__ ((__aligned__ (__alignof__ (type))))`, C 171-172,
     and PORTING.md 8 maps Rust's own `#[repr(align(N))]` attribute here instead).  */
  #[cfg (feature = "no_have_max_align_t")]
  macro_rules! _GL_STDDEF_ALIGNAS { ($type:expr) => {}; }

  /* C 183: `#   define max_align_t rpl_max_align_t` - C spells the name as a macro, so the
     port does too.  The expansion names the dead union above, which is why nothing expands
     it: the frozen target's `max_align_t` is the system's.  */
  #[cfg (feature = "no_have_max_align_t")]
  macro_rules! max_align_t { () => { rpl_max_align_t }; }
}
pub use self::stddef_decls::*;

/* C 216-218: the live arm of the C23 `unreachable` machinery, `# define gl_unreachable()
   __builtin_unreachable ()` (`_GL_HAS_BUILTIN_UNREACHABLE` is 1, measured).
   `__builtin_unreachable` is PORTING.md 2.6's `core::hint::unreachable_unchecked`, the
   mapping `include/verify.rs`'s `assume` uses.  */
macro_rules! gl_unreachable {
  () => { core::unreachable! () };
}

/* C 237-244: `# if !defined __cplusplus && !@HAVE_C_UNREACHABLE@ / #  define unreachable()
   gl_unreachable ()`.  Dead: `@HAVE_C_UNREACHABLE@` is 1, so the generated condition is `!1`
   (the measurement's `unreachable() __builtin_unreachable()` is the *system* header's C23
   spelling, not this arm).  Nothing in the port expands it.  */
#[cfg (feature = "no_c_unreachable")]
macro_rules! unreachable {
  () => { gl_unreachable! () };
}
