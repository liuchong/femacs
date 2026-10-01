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

// stdint.in.h 727
pub const PTRDIFF_WIDTH: usize = _GL_INTEGER_WIDTH (PTRDIFF_MIN, PTRDIFF_MAX);
