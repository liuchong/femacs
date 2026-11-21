// femacs: port of lib/byteswap.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

pub const _GL_BYTESWAP_INLINE: () = _GL_EXTERN_INLINE;

mod byteswap_unit {
    use super::*;

    #[unsafe(export_name = "bswap_16")]
    pub extern "C" fn bswap_16(x: uint_least16_t) -> uint_least16_t {
        femacs_pure::bitops::bswap_16(x)
    }

    #[unsafe(export_name = "bswap_32")]
    pub extern "C" fn bswap_32(x: uint_least32_t) -> uint_least32_t {
        femacs_pure::bitops::bswap_32(x)
    }

    #[unsafe(export_name = "bswap_64")]
    pub extern "C" fn bswap_64(x: uint_least64_t) -> uint_least64_t {
        femacs_pure::bitops::bswap_64(x)
    }
}
