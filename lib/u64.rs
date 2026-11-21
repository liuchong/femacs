// femacs: port of lib/u64.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

pub type dummy = c_int;
pub const _GL_U64_INLINE: () = _GL_EXTERN_INLINE;

mod u64_unit {
    use super::*;

    #[unsafe(export_name = "u64rol")]
    pub extern "C" fn u64rol(x: u64, n: c_int) -> u64 {
        femacs_pure::bitops::u64_rol(x, n)
    }
}
