// femacs: port of lib/stdc_leading_zeros.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

pub const _GL_STDC_LEADING_ZEROS_INLINE: () = _GL_EXTERN_INLINE;

mod stdc_leading_zeros_unit {
    use super::*;

    #[unsafe(export_name = "_gl_stdbit_clz")]
    pub extern "C" fn _gl_stdbit_clz(n: c_uint) -> c_int {
        super::_gl_stdbit_clz(n)
    }
    #[unsafe(export_name = "_gl_stdbit_clzl")]
    pub extern "C" fn _gl_stdbit_clzl(n: c_ulong) -> c_int {
        super::_gl_stdbit_clzl(n)
    }
    #[unsafe(export_name = "_gl_stdbit_clzll")]
    pub extern "C" fn _gl_stdbit_clzll(n: c_ulonglong) -> c_int {
        super::_gl_stdbit_clzll(n)
    }
    #[unsafe(export_name = "stdc_leading_zeros_ui")]
    pub extern "C" fn stdc_leading_zeros_ui(n: c_uint) -> c_uint {
        super::stdc_leading_zeros_ui(n)
    }
    #[unsafe(export_name = "stdc_leading_zeros_uc")]
    pub extern "C" fn stdc_leading_zeros_uc(n: c_uchar) -> c_uint {
        super::stdc_leading_zeros_uc(n)
    }
    #[unsafe(export_name = "stdc_leading_zeros_us")]
    pub extern "C" fn stdc_leading_zeros_us(n: c_ushort) -> c_uint {
        super::stdc_leading_zeros_us(n)
    }
    #[unsafe(export_name = "stdc_leading_zeros_ul")]
    pub extern "C" fn stdc_leading_zeros_ul(n: c_ulong) -> c_uint {
        super::stdc_leading_zeros_ul(n)
    }
    #[unsafe(export_name = "stdc_leading_zeros_ull")]
    pub extern "C" fn stdc_leading_zeros_ull(n: c_ulonglong) -> c_uint {
        super::stdc_leading_zeros_ull(n)
    }
}
