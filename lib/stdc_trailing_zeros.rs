// femacs: port of lib/stdc_trailing_zeros.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;

pub const _GL_STDC_TRAILING_ZEROS_INLINE: () = _GL_EXTERN_INLINE;

mod stdc_trailing_zeros_unit {
    use super::*;
    pub fn _gl_stdbit_ctz(n: c_uint) -> c_int {
        super::_gl_stdbit_ctz(n)
    }
    pub fn _gl_stdbit_ctzl(n: c_ulong) -> c_int {
        super::_gl_stdbit_ctzl(n)
    }
    pub fn _gl_stdbit_ctzll(n: c_ulonglong) -> c_int {
        super::_gl_stdbit_ctzll(n)
    }
    pub fn stdc_trailing_zeros_ui(n: c_uint) -> c_uint {
        super::stdc_trailing_zeros_ui(n)
    }
    pub fn stdc_trailing_zeros_uc(n: c_uchar) -> c_uint {
        super::stdc_trailing_zeros_uc(n)
    }
    pub fn stdc_trailing_zeros_us(n: c_ushort) -> c_uint {
        super::stdc_trailing_zeros_us(n)
    }
    pub fn stdc_trailing_zeros_ul(n: c_ulong) -> c_uint {
        super::stdc_trailing_zeros_ul(n)
    }
    pub fn stdc_trailing_zeros_ull(n: c_ulonglong) -> c_uint {
        super::stdc_trailing_zeros_ull(n)
    }
}
