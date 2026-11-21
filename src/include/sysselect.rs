// femacs: mirror of src/sysselect.h @ fd7afced403.
//
// Live text on the frozen target (`HAVE_SELECT` is 1, and neither `DOS_NT`, `WINDOWSNT` nor
// `MSDOS` is defined - docs/baseline/config/darwin-arm64.macros.tsv):
//
//  * C 24: `#include <sys/select.h>` - the port's system mirror carries `fd_set`,
//    `FD_SETSIZE` and the four accessors (`FD_ISSET`/`FD_SET`/`FD_CLR`/`FD_ZERO`, measured
//    against this target's SDK headers); C 33-44's `#define FD_SETSIZE 32` / `typedef int
//    fd_set` fallback is dead because the system header supplies `FD_SET`, which is what the
//    extractor's live text shows;
//  * C 62-93: the `#ifndef WINDOWSNT` block - the three bounds-checked `INLINE` functions and
//    the three macros that `#undef` the system names and re-point them at those functions.
//
// Dead: `#if !defined (HAVE_SELECT)` (C 46-48, `#define select sys_select`) and the
// `#ifdef MSDOS` arm (C 50-58) - both are recorded in docs/ports/include-sysselect.md.
//
// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
// (C writes the guard as `#define SYSSELECT_H 1`, so the constant carries C's `1`; the
// other mirrors' guards are empty `#define X_H` and are `bool = true`.)
// [U1-MODULE-MIRROR]
use super::*;
pub const SYSSELECT_H: c_int = 1;

// [U1] ../../lib/include/sys_select.rs imported via super

/// C 67-71: `INLINE void fd_CLR (int fd, fd_set *set)`.
#[inline(always)]
pub unsafe fn fd_CLR (fd: c_int, set: *mut fd_set)
{
  eassume! (0 <= fd && fd < FD_SETSIZE);
  FD_CLR (fd, set);
}

/// C 74-78: `INLINE bool fd_ISSET (int fd, fd_set *set)`.
#[inline(always)]
pub unsafe fn fd_ISSET (fd: c_int, set: *const fd_set) -> bool
{
  eassume! (0 <= fd && fd < FD_SETSIZE);
  return FD_ISSET (fd, set) as c_int != 0;
}

/// C 81-84: `INLINE void fd_SET (int fd, fd_set *set)`.
#[inline(always)]
pub unsafe fn fd_SET (fd: c_int, set: *mut fd_set)
{
  eassume! (0 <= fd && fd < FD_SETSIZE);
  FD_SET (fd, set);
}

/// C 90-93: C `#undef`s the three system macros and re-points the names at the bounds-checked
/// functions above, so a caller that writes `FD_CLR (fd, set)` after including this header gets
/// the checked version.  A Rust `macro_rules!` carries the same re-pointing (macro expansion
/// happens before name resolution, so it shadows the system mirror's function of that name).
macro_rules! FD_CLR { ($fd:expr, $set:expr) => { fd_CLR ($fd, $set) }; }
macro_rules! FD_ISSET { ($fd:expr, $set:expr) => { fd_ISSET ($fd, $set) }; }
macro_rules! FD_SET { ($fd:expr, $set:expr) => { fd_SET ($fd, $set) }; }
