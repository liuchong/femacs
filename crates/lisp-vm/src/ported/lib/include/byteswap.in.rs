// femacs: port of lib/byteswap.in.h @ fd7afced403 (gnulib).
#![forbid(unsafe_code)]
use super::*;

const _: () = core::assert!(_GL_CONFIG_H_INCLUDED, "Please include config.h first.");
pub const _GL_BYTESWAP_H: c_int = 1;

#[inline(always)]
pub const fn bswap_16(x: uint_least16_t) -> uint_least16_t {
    crate::sys::bitops::bswap_16(x)
}

#[inline(always)]
pub const fn bswap_32(x: uint_least32_t) -> uint_least32_t {
    crate::sys::bitops::bswap_32(x)
}

#[inline(always)]
pub const fn bswap_64(x: uint_least64_t) -> uint_least64_t {
    crate::sys::bitops::bswap_64(x)
}
