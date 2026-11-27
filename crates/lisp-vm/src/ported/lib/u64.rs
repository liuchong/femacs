// femacs: port of lib/u64.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::sys_common::*;

pub type dummy = c_int;
pub const _GL_U64_INLINE: () = _GL_EXTERN_INLINE;

mod u64_unit {
    use super::*;
    pub fn u64rol(x: u64, n: c_int) -> u64 {
        crate::sys::bitops::u64_rol(x, n)
    }
}
