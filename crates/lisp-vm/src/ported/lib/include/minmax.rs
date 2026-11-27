// femacs: mirror of lib/minmax.h @ fd7afced403 ; PORTING.md 0.1.
#![forbid(unsafe_code)]
//
// The header is `MAX`/`MIN` and nothing else.  C's `#ifndef MAX` guards are dead here (the
// system headers of this target do not define them), and the C bodies' arguments are each
// evaluated twice - which the port's macro_rules keeps, exactly like C, because these are
// value macros whose callers pass plain variables.

// The include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const _MINMAX_H: bool = true;

/* MAX(a,b) returns the maximum of A and B.  */
macro_rules! MAX { ($a:expr, $b:expr) => { if ($a) > ($b) { $a } else { $b } }; }

/* MIN(a,b) returns the minimum of A and B.  */
macro_rules! MIN { ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } }; }
