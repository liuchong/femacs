// femacs: port of lib/endian.in.h @ fd7afced403 (gnulib).
//
// gnulib template; the frozen target's generated header is build/emacs-ref/lib/endian.h,
// whose substitutions are `_GL_ENDIAN_H` (guard), `HAVE_ENDIAN_H` = 0,
// `HAVE_SYS_ENDIAN_H` = 1 (`#include <sys/endian.h>` is live) and
// `ENDIAN_H_JUST_MISSING_STDINT` = 0 (gnulib's own macros and functions are the live arm).
// The port keeps the live arm only: the system header's `be16toh` family is not mirrored
// (P-26, the port has its own), and the `#if @HAVE_ENDIAN_H@ || @HAVE_SYS_ENDIAN_H@` block
// that `#undef`s and renames the twelve identifiers is live, which is why the *unit*
// exports them as `rpl_endian_*` (lib/endian.rs).
//
// `_GL_ENDIAN_INLINE` is defined by the unit (lib/endian.rs 23); the header's `#ifndef`
// fallback spells `_GL_INLINE` and has no Rust counterpart
// (docs/ports/excluded-macros.txt, kind=syntax).
//
// Definitions are header-inline in C, so `#[inline(always)]` here (P-12) with no symbol;
// lib/endian.rs re-exports them as the renamed symbols (P-81).
include!("gnulib_config.rs");
include!("stdint.in.rs");
include!("byteswap.in.rs");

/* C 50-52's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.  */
const _: () = core::assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

/* C 20-47: the guard `_@GUARD_PREFIX@_ENDIAN_H` (the frozen target substitutes
   `_GL_ENDIAN_H`), and the three byte-order constants C defines itself.  */
pub const _GL_ENDIAN_H: c_int = 1;
pub const LITTLE_ENDIAN: c_int = 1234;
pub const BIG_ENDIAN: c_int = 4321;
pub const PDP_ENDIAN: c_int = 3412;

/* C 70-74: `#ifdef WORDS_BIGENDIAN` - this target is little-endian, so C's `#else` line is
   the value and the macro keeps C's spelling.  */
pub const BYTE_ORDER: c_int = LITTLE_ENDIAN;

/* C 121-291's `# define GNULIB_defined_<name> 1` lines: each header writes them so that a
   second inclusion of gnulib's <endian.h> does not redefine the wrapper.  The port defines
   each wrapper once, so the marker is carried as the constant C's preprocessor has.  */
pub const GNULIB_defined_be16toh: c_int = 1;
pub const GNULIB_defined_be32toh: c_int = 1;
pub const GNULIB_defined_be64toh: c_int = 1;
pub const GNULIB_defined_htobe16: c_int = 1;
pub const GNULIB_defined_htobe32: c_int = 1;
pub const GNULIB_defined_htobe64: c_int = 1;
pub const GNULIB_defined_le16toh: c_int = 1;
pub const GNULIB_defined_le32toh: c_int = 1;
pub const GNULIB_defined_le64toh: c_int = 1;
pub const GNULIB_defined_htole16: c_int = 1;
pub const GNULIB_defined_htole32: c_int = 1;
pub const GNULIB_defined_htole64: c_int = 1;

/* C 76-106: the live `#if @HAVE_ENDIAN_H@ || @HAVE_SYS_ENDIAN_H@` block `#undef`s the
   system's twelve identifiers and renames them to gnulib's `rpl_endian_*`; the port's
   definitions keep C's spelling and the *unit* exports them under the renamed C symbol
   (PORTING.md P-81).  */

/* C 121-291: the twelve wrappers, in C's order (big-endian group, host-to-big, little-to-
   host, host-to-little).  */

/* C endian.in.h be16toh: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn be16toh (x: uint16_t) -> uint16_t
{
  if BYTE_ORDER == BIG_ENDIAN { return x; }
  else { return bswap_16 (x); }
}

/* C endian.in.h be32toh: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn be32toh (x: uint32_t) -> uint32_t
{
  if BYTE_ORDER == BIG_ENDIAN { return x; }
  else { return bswap_32 (x); }
}

/* C endian.in.h be64toh: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn be64toh (x: uint64_t) -> uint64_t
{
  if BYTE_ORDER == BIG_ENDIAN { return x; }
  else { return bswap_64 (x); }
}

/* C endian.in.h htobe16: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn htobe16 (x: uint16_t) -> uint16_t
{
  if BYTE_ORDER == BIG_ENDIAN { return x; }
  else { return bswap_16 (x); }
}

/* C endian.in.h htobe32: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn htobe32 (x: uint32_t) -> uint32_t
{
  if BYTE_ORDER == BIG_ENDIAN { return x; }
  else { return bswap_32 (x); }
}

/* C endian.in.h htobe64: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn htobe64 (x: uint64_t) -> uint64_t
{
  if BYTE_ORDER == BIG_ENDIAN { return x; }
  else { return bswap_64 (x); }
}

/* C endian.in.h le16toh: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn le16toh (x: uint16_t) -> uint16_t
{
  if BYTE_ORDER == BIG_ENDIAN { return bswap_16 (x); }
  else { return x; }
}

/* C endian.in.h le32toh: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn le32toh (x: uint32_t) -> uint32_t
{
  if BYTE_ORDER == BIG_ENDIAN { return bswap_32 (x); }
  else { return x; }
}

/* C endian.in.h le64toh: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn le64toh (x: uint64_t) -> uint64_t
{
  if BYTE_ORDER == BIG_ENDIAN { return bswap_64 (x); }
  else { return x; }
}

/* C endian.in.h htole16: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn htole16 (x: uint16_t) -> uint16_t
{
  if BYTE_ORDER == BIG_ENDIAN { return bswap_16 (x); }
  else { return x; }
}

/* C endian.in.h htole32: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn htole32 (x: uint32_t) -> uint32_t
{
  if BYTE_ORDER == BIG_ENDIAN { return bswap_32 (x); }
  else { return x; }
}

/* C endian.in.h htole64: `#if BYTE_ORDER == BIG_ENDIAN` is the frozen target's false arm
   (darwin is little-endian), so the body is C's `#else` line; the comparison is kept
   because the port spells both arms of C's `#if` (P-51).  */
#[inline(always)]
pub fn htole64 (x: uint64_t) -> uint64_t
{
  if BYTE_ORDER == BIG_ENDIAN { return bswap_64 (x); }
  else { return x; }
}

