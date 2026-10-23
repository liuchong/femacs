// femacs: mirror of src/sysstdio.h @ fd7afced403.
//
// Live text on the frozen target: the include guard, three extern declarations and the `#else`
// arm of `#if O_BINARY` - `O_BINARY` is 0 in the frozen config.h, so both stream-mode strings
// are empty (C 39-40, the arms `tools/extract-abi.py` lists as the live macros).  The
// `#ifndef PIPE_BUF` fallback (C 43-45) is dead: the frozen `<limits.h>` defines PIPE_BUF
// (512, docs/baseline/config/darwin-arm64.macros.tsv).
//
// The three declarations are declarations only - C defines `errputc`, `errwrite` and
// `close_output_streams` in src/sysdep.c, which is ported (`src/sysdep.rs` 4147, 4161,
// 4168), so the unit that owns them stays the single definition point (PORTING.md 0.1-3).
// C 29's `#include <unlocked-io.h>` is `lib/include/unlocked-io.rs` (batch L landed the
// mirror): C's renames of the stdio entries live there as macros, both arms and the measured
// criterion for the live one included, so a unit that spells one of those names invokes the
// macro at its own use site rather than inventing a declaration for it.
//
// C 22: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_SYSSTDIO_H: bool = true;

include!("../../lib/include/stdio.rs");        // C 24: `#include <stdio.h>`
include!("../../lib/include/fcntl.in.rs");     // C 25: `#include <fcntl.h>` (`O_BINARY`)
include!("../../lib/include/limits.in.rs");    // C 26: `#include <limits.h>` (`PIPE_BUF`)
include!("../../lib/include/attribute.rs");    // C 28: `#include <attribute.h>`
include!("../../lib/include/unlocked-io.rs");   // C 29: `#include <unlocked-io.h>`

// C 27's `#include <stdarg.h>` has no mirror: Rust spells a C variadic list as `VaList`
// (the `sysdep.rs`/`print.rs` precedent), so nothing is included here for it.

pub mod sysstdio_decls {
  use super::*;

  unsafe extern "C" {
    /* C 32: `extern void errputc (int);`  */
    pub fn errputc (c: c_int);
    /* C 33: `extern void errwrite (void const *, ptrdiff_t);`  */
    pub fn errwrite (buf: *const c_void, nbuf: ptrdiff_t);
    /* C 34: `extern void close_output_streams (void);`  */
    pub fn close_output_streams ();
  }
}
use sysstdio_decls::*;

/// C 39: `# define FOPEN_BINARY ""` - the live `#else` arm of `#if O_BINARY`.  The extractor
/// compares a string-literal macro body as a constant, so the mirror keeps it one (PORTING.md
/// 2.1's rule: a string macro is a `&CStr` constant unless a use site needs it inside `concat!`
/// - `src/xfaces.rs` 572 does, and records that use-site `macro_rules!` there).
pub const FOPEN_BINARY: &core::ffi::CStr = c"";

/// C 40: `# define FOPEN_TEXT ""` - the same arm.
pub const FOPEN_TEXT: &core::ffi::CStr = c"";
