// femacs: port of lib/stdint.in.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror: it carries only the width section (stdint.in.h 683-733) that the
// ported units need so far; the rest of the header is tracked in
// docs/ports/stdint.in.md.

// stdint.in.h 77 includes <limits.h> for the _GL_INTEGER_WIDTH helpers.
include!("limits.in.rs");

// gnulib defines PTRDIFF_MIN/PTRDIFF_MAX only when the system header lacks them
// (stdint.in.h 539-550); on the frozen target the C sees the system <stdint.h>
// values, which are the machine-word limits.
pub const PTRDIFF_MIN: isize = isize::MIN;
pub const PTRDIFF_MAX: isize = isize::MAX;

// stdint.in.h 539-550 covers SIZE_MAX the same way; the frozen target's
// <stdint.h> supplies it, and buffer.h's BUF_BYTES_MAX names it.
pub const SIZE_MAX: usize = usize::MAX;

// stdint.in.h 727
pub const PTRDIFF_WIDTH: usize = _GL_INTEGER_WIDTH (PTRDIFF_MIN, PTRDIFF_MAX);

// The fast integer types the reference names in its own headers (character.h
// uses int_fast64_t); the frozen target's widths come from the system header.
pub type int_fast64_t = i64;
