// femacs: mirror of lib/sys-limits.h @ fd7afced403 ; PORTING.md 0.1.
#![forbid(unsafe_code)]
//
// The whole header is `enum { SYS_BUFSIZE_MAX = INT_MAX >> 20 << 20 };`, C's
// MAX_RW_COUNT approximation: `(INT_MAX >> 20) << 20` is 2047 << 20 = 2146435072.

// [U1-MODULE-MIRROR]
use super::*;
// [U1] limits.in.rs imported via super

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const _GL_SYS_LIMITS_H: bool = true;

/* enum { SYS_BUFSIZE_MAX = INT_MAX >> 20 << 20 };  */
pub const SYS_BUFSIZE_MAX: c_int = INT_MAX >> 20 << 20;
