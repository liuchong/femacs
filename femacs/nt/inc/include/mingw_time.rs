// femacs: mirror of nt/inc/mingw_time.h @ fd7afced403.
//
// The Windows port's workaround for the MinGW runtime no longer maintaining its own version
// macros: the values are re-derived here once, so the rest of the tree can trust one set.  The
// file is not on `docs/baseline/config/cond-eval-verified.txt` and has no `#if` that this
// mirror has to choose between (see below), so the whole file is live text.  Nothing in the
// frozen configuration includes it (no `nt/` translation unit is compiled -
// docs/baseline/frozen-build-sources.txt's batch-F section - and no `src/` unit includes a
// `nt/inc/` header), so `xtask layout-check` never compiles this mirror; in a MinGW build
// `src/w32.c`, `src/w32proc.c` and `nt/inc/ms-w32.h` are the units whose
// `#include <mingw_time.h>` resolves here.
//
// `#include <_mingw.h>` (C 12-14, inside `#ifdef __MINGW32__`) has no mirror in the port: it is
// the MinGW runtime's own header, and the numbers below are what C re-derives from it.

// C 3: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _MINGW_TIME_H: bool = true;

/// C 19: `#define __MINGW_VERSION 3.17`  (`#ifndef __MINGW_VERSION`, and MinGW64 defines
/// `__MINGW64_VERSION_MAJOR` instead, so this whole block is the MinGW.org arm).
pub const __MINGW_VERSION: f64 = 3.17;
/// C 21: `#define __MINGW_MAJOR_VERSION 3`
pub const __MINGW_MAJOR_VERSION: c_int = 3;
/// C 23: `#define __MINGW_MINOR_VERSION 17`
pub const __MINGW_MINOR_VERSION: c_int = 17;
/// C 25: `#define __MINGW_PATCHLEVEL 0`
pub const __MINGW_PATCHLEVEL: c_int = 0;

/// C 29: `# define _USE_32BIT_TIME_T` under `#if __MINGW_MAJOR_VERSION >= 4` - the name the
/// runtime's 4.x headers read, which is why the four values above exist at all.
pub const _USE_32BIT_TIME_T: bool = true;
