// femacs: port of lib/stdc_memreverse8u.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::sys_common::*;

pub const _GL_STDC_MEMREVERSE8U_INLINE: () = _GL_EXTERN_INLINE;

mod stdc_memreverse8u_unit {
    use super::*;
    pub fn stdc_memreverse8u8(value: uint_least8_t) -> uint_least8_t {
        super::stdc_memreverse8u8(value)
    }
    pub fn stdc_memreverse8u16(value: uint_least16_t) -> uint_least16_t {
        super::stdc_memreverse8u16(value)
    }
    pub fn stdc_memreverse8u32(value: uint_least32_t) -> uint_least32_t {
        super::stdc_memreverse8u32(value)
    }
    pub fn stdc_memreverse8u64(value: uint_least64_t) -> uint_least64_t {
        super::stdc_memreverse8u64(value)
    }
}
