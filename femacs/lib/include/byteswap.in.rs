// femacs: port of lib/byteswap.in.h @ fd7afced403 (gnulib).
//
// gnulib template; the frozen target's generated header is build/emacs-ref/lib/byteswap.h,
// and its substitutions are `_GL_BYTESWAP_H` for the guard and nothing else.  C's
// `_GL_BYTESWAP_INLINE` is defined by the *unit* (lib/byteswap.rs 18), so the header's
// `#ifndef` fallback is dead text (docs/ports/excluded-macros.txt, kind=syntax) and the
// three wrappers below are the live `_GL_EXTERN_INLINE` definitions the object carries.
//
// `#include <stdbit.h>` is the stdbit mirror (the wrappers are `stdc_memreverse8u*`
// calls) and `#include <stdint.h>` is the stdint mirror for `uint_leastN_t` and the
// `#ifdef UINT_LEAST64_MAX` that guards bswap_64 (live on this target).
//
// Definitions are header-inline in C, so they are `#[inline(always)]` here (P-12) and
// emit no symbol; lib/byteswap.rs re-exports them (P-81).
include!("gnulib_config.rs");
include!("stdint.in.rs");
include!("stdbit.in.rs");

/* C 22-24's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.  */
const _: () = assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

/* C 18-19: `#ifndef _GL_BYTESWAP_H` / `#define _GL_BYTESWAP_H 1`.  */
pub const _GL_BYTESWAP_H: c_int = 1;

/* C 41-47: `bswap_16`, the 16-bit byte reversal (C's comment).  */
#[inline(always)]
pub fn bswap_16 (x: uint_least16_t) -> uint_least16_t { return stdc_memreverse8u16 (x); }

/* C 49-55.  */
#[inline(always)]
pub fn bswap_32 (x: uint_least32_t) -> uint_least32_t { return stdc_memreverse8u32 (x); }

/* C 57-65, inside `#ifdef UINT_LEAST64_MAX`.  */
#[inline(always)]
pub fn bswap_64 (x: uint_least64_t) -> uint_least64_t { return stdc_memreverse8u64 (x); }
