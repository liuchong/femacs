// femacs: WIP mirror of lib/getopt-cdefs.in.h @ fd7afced403 (the generated
#![forbid(unsafe_code)]
// `build/emacs-ref/lib/getopt-cdefs.h`; PORTING.md P-26).
//
// The header exists so that getopt-core.h and getopt-ext.h get the internal macros that
// glibc's <sys/cdefs.h> would supply.  Every one of them is a spelling macro with no Rust
// body: `__BEGIN_DECLS` / `__END_DECLS` are `extern "C" {` / `}`, and `__THROW` /
// `__GNUC_PREREQ` are attributes, which PORTING.md 0.3 drops.  `HAVE_SYS_CDEFS_H` is
// undefined in the frozen configuration, so the generated header's `#include <sys/cdefs.h>`
// arm is dead (`#if 0` in build/emacs-ref/lib/getopt-cdefs.h).
//
// The file therefore carries the record and no items; docs/ports/include-getopt-cdefs.md
// tracks the header's four macros as the rest of the mirror.

// [U1-MODULE-MIRROR]
use super::*;
/* C 21: the include guard, whose C body is `1`.  */
pub const _GETOPT_CDEFS_H: core::ffi::c_int = 1;
