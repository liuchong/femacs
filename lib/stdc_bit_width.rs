// femacs: port of lib/stdc_bit_width.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;

pub const _GL_STDC_BIT_WIDTH_INLINE: () = _GL_EXTERN_INLINE;

mod stdc_bit_width_unit {
    use super::*;
    pub fn stdc_bit_width_uc(n: c_uchar) -> c_uint {
        super::stdc_bit_width_uc(n)
    }
    pub fn stdc_bit_width_us(n: c_ushort) -> c_uint {
        super::stdc_bit_width_us(n)
    }
    pub fn stdc_bit_width_ui(n: c_uint) -> c_uint {
        super::stdc_bit_width_ui(n)
    }
    pub fn stdc_bit_width_ul(n: c_ulong) -> c_uint {
        super::stdc_bit_width_ul(n)
    }
    pub fn stdc_bit_width_ull(n: c_ulonglong) -> c_uint {
        super::stdc_bit_width_ull(n)
    }
}
