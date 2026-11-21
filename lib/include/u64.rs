// [U1-MODULE-MIRROR]
use super::*;
/* Written by Paul Eggert.  */

// femacs: port of lib/u64.h @ fd7afced403 (gnulib).
//
// Header mirror (PORTING.md 0.1).  C 44's `#if defined UINT64_MAX && INT_MAX <
// UINT64_MAX` arm is the live one on the frozen target (uint64_t exists), so `u64` is
// `uint64_t` and the fourteen operations are C's macros; C 64-236's `#else` arm - a
// `struct u64` plus the same operations as `_GL_U64_INLINE` *functions* - is the
// text for a host without uint64_t, and it is recorded in
// docs/ports/excluded-macros.txt (kind=dead-text) and docs/ports/include-u64.md,
// because the frozen target never compiles it.  C 33-35's `<limits.h>`/`<stddef.h>`/
// `<stdint.h>` are the limits/stdint mirrors; C 37's `<byteswap.h>` is the `bswap_64`
// macro below (PORTING.md 2.6: C's `bswap_64` is Rust's `swap_bytes`).
//
// The `#ifdef __cplusplus extern "C" {` and `_GL_INLINE_HEADER_BEGIN/END` lines have no
// Rust counterpart (PORTING.md 0.1; the `_GL_INLINE_HEADER_BEGIN` row is in
// excluded-macros.txt).

// config.rs via common root
// [U1] gnulib_config.rs imported via super
// [U1] limits.in.rs imported via super
// [U1] stdint.in.rs imported via super

/* C 22-23: `#ifndef U64_H` / `#define U64_H 1`.  */
pub const U64_H: c_int = 1;

/* C 26-28's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.  */
const _: () = core::assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

/* C 71's `# define _GL_U64_MASK32 0xfffffffful` (its comment reads `2**32 - 1.`)
   is the dead arm's mask; the value is kept under the feature naming the non-frozen
   side (P-51) and its `ul` suffix is spelling (docs/ports/macro-body-notes.tsv).  */
#[cfg (feature = "no_uint64")]
pub const _GL_U64_MASK32: c_ulong = 0xffffffff;

/* C 48's `typedef uint64_t u64;`.  `uint64_t` is itself an alias of the primitive,
   and naming this alias `u64` shadows the primitive, so the right-hand side is spelled
   through `core::primitive` (the stdint mirror's `uint64_t` would be a cycle).  */
pub type u64 = core::primitive::u64;

/* C 37's `<byteswap.h>`: `bswap_64`, which C spells as `__builtin_bswap64`.  */
macro_rules! bswap_64 { ($x:expr) => { ($x).swap_bytes () } }

/* C 49-62: the native operations.  Each macro's body is C's, with C's casts and C's
   compound-literal spelling `(T) {(x)}` written as the same cast; C's unsigned `+` is
   `wrapping_add` (the port's unit builds carry debug assertions).  */
macro_rules! u64hilo { ($hi:expr, $lo:expr) => { ((($hi) as u64) << 32).wrapping_add (($lo) as u64) } }
macro_rules! u64init { ($hi:expr, $lo:expr) => { ((($hi) as u64) << 32).wrapping_add (($lo) as u64) } }
macro_rules! u64lo { ($x:expr) => { ($x) as u64 } }
macro_rules! u64getlo { ($x:expr) => { (($x) as u64 & (UINT32_MAX as u64)) as uint32_t } }
macro_rules! u64size { ($x:expr) => { u64lo! ($x) } }
macro_rules! u64not { ($x:expr) => { !($x) } }
macro_rules! u64lt { ($x:expr, $y:expr) => { ($x) < ($y) } }
macro_rules! u64and { ($x:expr, $y:expr) => { ($x) & ($y) } }
macro_rules! u64or { ($x:expr, $y:expr) => { ($x) | ($y) } }
macro_rules! u64xor { ($x:expr, $y:expr) => { ($x) ^ ($y) } }
macro_rules! u64plus { ($x:expr, $y:expr) => { ($x).wrapping_add ($y) } }
macro_rules! u64shl { ($x:expr, $n:expr) => { ($x) << ($n) } }
macro_rules! u64shr { ($x:expr, $n:expr) => { ($x) >> ($n) } }
macro_rules! u64bswap { ($x:expr) => { bswap_64! ($x) } }

/* C 223-227: the rotate.  The reference defines it in the `#else` arm (the one this
   target does not compile) while sha512.c C 345-348 and sha3.c C 205-306 call it in the
   frozen configuration, so the C reference needs it here too; the port carries C's body
   unconditionally (docs/ports/include-u64.md records the reading), which is what makes
   those two units compile.  */
pub fn u64rol (x: u64, n: c_int) -> u64
{
  return u64or! (u64shl! (x, n), u64shr! (x, 64 - n));
}
