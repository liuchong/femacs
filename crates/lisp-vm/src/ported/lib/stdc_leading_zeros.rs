// femacs: port of lib/stdc_leading_zeros.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::sys_common::*;

pub const _GL_STDC_LEADING_ZEROS_INLINE: () = _GL_EXTERN_INLINE;

mod stdc_leading_zeros_unit {
    use super::*;
    pub fn _gl_stdbit_clz(n: c_uint) -> c_int {
        super::_gl_stdbit_clz(n)
    }
    pub fn _gl_stdbit_clzl(n: c_ulong) -> c_int {
        super::_gl_stdbit_clzl(n)
    }
    pub fn _gl_stdbit_clzll(n: c_ulonglong) -> c_int {
        super::_gl_stdbit_clzll(n)
    }
    pub fn stdc_leading_zeros_ui(n: c_uint) -> c_uint {
        super::stdc_leading_zeros_ui(n)
    }
    pub fn stdc_leading_zeros_uc(n: c_uchar) -> c_uint {
        super::stdc_leading_zeros_uc(n)
    }
    pub fn stdc_leading_zeros_us(n: c_ushort) -> c_uint {
        super::stdc_leading_zeros_us(n)
    }
    pub fn stdc_leading_zeros_ul(n: c_ulong) -> c_uint {
        super::stdc_leading_zeros_ul(n)
    }
    pub fn stdc_leading_zeros_ull(n: c_ulonglong) -> c_uint {
        super::stdc_leading_zeros_ull(n)
    }
}
