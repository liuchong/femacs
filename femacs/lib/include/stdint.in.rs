// femacs: port of lib/stdint.in.h @ fd7afced403.
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
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

// stdint.in.h 324 spells the pointer-width typedefs alongside the fixed-width ones; the
// lisp.h mirror carries `intptr_t` where C's lisp.h 1113 uses it, and this one is here
// because pdumper.h's `struct pdumper_loaded_dump` names it (pdumper.h 154-158).
pub type uintptr_t = usize;

// <stdint.h>'s per-width maxima; lib/u64.h's native `u64getlo` masks with
// UINT32_MAX (the lisp.h mirror carries the machine-word ones).
pub const UINT32_MAX: u32 = u32::MAX;

// The fast integer types temname.c names: `typedef uint_fast64_t random_value;` and
// `#define RANDOM_VALUE_MAX UINT_FAST64_MAX` (lib/tempname.c 64-65); the frozen target's
// widths come from the system header (uint_fast64_t is a 64-bit unsigned type, measured:
// `printf ("%zu", sizeof (uint_fast64_t))` -> 8).
pub type uint_fast64_t = u64;
pub const UINT_FAST64_MAX: u64 = u64::MAX;

// The least-width integer types, which stdbit.in.h's memreverse section names: ISO C
// defines `stdc_memreverse8u*` with `uintN_t` and gnulib generalises them to
// `uint_leastN_t`, which the system header maps to the same types on this target
// (`clang -dM`: __UINT_LEAST8_TYPE__ is `unsigned char`, __UINT_LEAST16_TYPE__
// `unsigned short`, __UINT_LEAST32_TYPE__ `unsigned int`, __UINT_LEAST64_TYPE__
// `long long unsigned int`).
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;

// The `#ifdef UINT_LEAST64_MAX` that guards byteswap.in.h's bswap_64: the system header
// defines it, so the third wrapper function is live (lib/include/byteswap.in.rs).
pub const UINT_LEAST64_MAX: u64 = u64::MAX;
// `#ifdef UINT64_MAX` (endian.in.h 149, 192, 235, 278): the system header defines it, so
// the three 64-bit host/big/little wrappers are live.
pub const UINT64_MAX: u64 = u64::MAX;

// stdint.in.h 481-521's `int_fast32_t`/`uint_fast32_t`: regcomp.c's bracket parser takes
// its `nrules` and `table_size` in these two (`build_range_exp`, `parse_bracket_exp`), and
// regex_internal.h 674-690 declares the same pair for the regex cluster.  Measured with
// the frozen target's compiler: both are 4 bytes (arm64 darwin widens only the 64-bit
// fast types).
pub type int_fast32_t = i32;
pub type uint_fast32_t = u32;
