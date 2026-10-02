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

// stdint.in.h 728 (the TS 18661-1 block): SIZE_WIDTH is the one width macro this header
// owns; INT_WIDTH/UINT_WIDTH/LONG_WIDTH/ULONG_WIDTH come from <limits.h> (limits.in.h
// 112-122, already in scope through the include above) and PTRDIFF_WIDTH is at line 727.
// Measured with `printf ("%d", SIZE_WIDTH)` on the frozen build.
pub const SIZE_WIDTH: c_int = (8 * std::mem::size_of::<usize> ()) as c_int;

// stdint.in.h 539-550 covers SIZE_MAX the same way; the frozen target's
// <stdint.h> supplies it, and buffer.h's BUF_BYTES_MAX names it.
pub const SIZE_MAX: usize = usize::MAX;

// stdint.in.h 539-550 also spells INTMAX_MIN/INTMAX_MAX/UINTMAX_MAX for the wide
// typedefs; bignum.c's `mpz_to_intmax`/`mpz_to_uintmax`/`mpz_set_*_slow` read them, and
// the port derives them from the same `intmax_t`/`uintmax_t` aliases lisp.rs declares.
pub const INTMAX_MIN: i64 = i64::MIN;
pub const INTMAX_MAX: i64 = i64::MAX;
pub const UINTMAX_MAX: u64 = u64::MAX;

// stdint.in.h 727
pub const PTRDIFF_WIDTH: usize = _GL_INTEGER_WIDTH (PTRDIFF_MIN, PTRDIFF_MAX);

// The fast integer types the reference names in its own headers (character.h
// uses int_fast64_t); the frozen target's widths come from the system header.
pub type int_fast64_t = i64;

// The frozen target's <stdint.h> spells the fixed-width typedefs; a ported unit keeps
// C's spelling where a body's token check compares it (md5.c's `uint32_t`), the way the
// lisp.h mirror carries `intptr_t`/`uintmax_t`.
pub type uint32_t = u32;
pub type uint64_t = u64;

// <stdint.h>'s per-width maxima; lib/u64.h's native `u64getlo` masks with
// UINT32_MAX (the lisp.h mirror carries the machine-word ones).
pub const UINT32_MAX: u32 = u32::MAX;
