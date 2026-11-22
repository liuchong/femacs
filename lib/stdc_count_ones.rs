// femacs: port of lib/stdc_count_ones.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;
pub const _GL_STDC_COUNT_ONES_INLINE: () = _GL_EXTERN_INLINE;

mod stdc_count_ones_unit {
    use super::*;
    pub fn stdc_count_ones_ui(n: c_uint) -> c_uint {
        super::stdc_count_ones_ui(n)
    }
    pub fn stdc_count_ones_uc(n: c_uchar) -> c_uint {
        super::stdc_count_ones_uc(n)
    }
    pub fn stdc_count_ones_us(n: c_ushort) -> c_uint {
        super::stdc_count_ones_us(n)
    }
    pub fn stdc_count_ones_ul(n: c_ulong) -> c_uint {
        super::stdc_count_ones_ul(n)
    }
    pub fn stdc_count_ones_ull(n: c_ulonglong) -> c_uint {
        super::stdc_count_ones_ull(n)
    }
}
