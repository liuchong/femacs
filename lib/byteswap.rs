// femacs: port of lib/byteswap.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;

pub const _GL_BYTESWAP_INLINE: () = _GL_EXTERN_INLINE;

mod byteswap_unit {
    use super::*;
    pub fn bswap_16(x: uint_least16_t) -> uint_least16_t {
        ::femacs_sys::bitops::bswap_16(x)
    }
    pub fn bswap_32(x: uint_least32_t) -> uint_least32_t {
        ::femacs_sys::bitops::bswap_32(x)
    }
    pub fn bswap_64(x: uint_least64_t) -> uint_least64_t {
        ::femacs_sys::bitops::bswap_64(x)
    }
}
