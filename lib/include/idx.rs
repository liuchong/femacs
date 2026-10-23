// femacs: port of lib/idx.h @ fd7afced403.
//
// The header is a typedef guarded by `__PTRDIFF_TYPE__` plus one macro; the frozen
// target's clang defines the former as `long int`, so the port spells `idx_t` as the
// reference's own 1:1 type (PORTING.md 1.1: `ptrdiff_t`/`size_t` are `isize`/`usize`).
//
// C 22's `#ifndef __PTRDIFF_TYPE__ / # include <stddef.h> / #endif` and C 33's fallback
// `typedef ptrdiff_t idx_t;` are the compiler-guarded arms; the frozen target takes the
// `__PTRDIFF_TYPE__` arm (C 91), which is the definition below.

/// `__PTRDIFF_TYPE__` (the compiler's builtin, `long int` on the frozen target).
pub const __PTRDIFF_TYPE__: idx_t = 0;

/* typedef __PTRDIFF_TYPE__ idx_t;  */
pub type idx_t = isize;

/// `__PTRDIFF_MAX__` (the compiler's builtin, `PTRDIFF_MAX` of the target).
pub const __PTRDIFF_MAX__: idx_t = isize::MAX;

/* IDX_MAX is the maximum value of an idx_t.  */
/* # define IDX_MAX __PTRDIFF_MAX__  */
pub const IDX_MAX: idx_t = __PTRDIFF_MAX__;

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _IDX_H: bool = true;
