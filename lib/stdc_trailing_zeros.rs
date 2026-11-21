// femacs: port of lib/stdc_trailing_zeros.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

pub const _GL_STDC_TRAILING_ZEROS_INLINE: () = _GL_EXTERN_INLINE;

mod stdc_trailing_zeros_unit {
    use super::*;

    #[unsafe(export_name = "_gl_stdbit_ctz")]
    pub extern "C" fn _gl_stdbit_ctz(n: c_uint) -> c_int {
        super::_gl_stdbit_ctz(n)
    }
    #[unsafe(export_name = "_gl_stdbit_ctzl")]
    pub extern "C" fn _gl_stdbit_ctzl(n: c_ulong) -> c_int {
        super::_gl_stdbit_ctzl(n)
    }
    #[unsafe(export_name = "_gl_stdbit_ctzll")]
    pub extern "C" fn _gl_stdbit_ctzll(n: c_ulonglong) -> c_int {
        super::_gl_stdbit_ctzll(n)
    }
    #[unsafe(export_name = "stdc_trailing_zeros_ui")]
    pub extern "C" fn stdc_trailing_zeros_ui(n: c_uint) -> c_uint {
        super::stdc_trailing_zeros_ui(n)
    }
    #[unsafe(export_name = "stdc_trailing_zeros_uc")]
    pub extern "C" fn stdc_trailing_zeros_uc(n: c_uchar) -> c_uint {
        super::stdc_trailing_zeros_uc(n)
    }
    #[unsafe(export_name = "stdc_trailing_zeros_us")]
    pub extern "C" fn stdc_trailing_zeros_us(n: c_ushort) -> c_uint {
        super::stdc_trailing_zeros_us(n)
    }
    #[unsafe(export_name = "stdc_trailing_zeros_ul")]
    pub extern "C" fn stdc_trailing_zeros_ul(n: c_ulong) -> c_uint {
        super::stdc_trailing_zeros_ul(n)
    }
    #[unsafe(export_name = "stdc_trailing_zeros_ull")]
    pub extern "C" fn stdc_trailing_zeros_ull(n: c_ulonglong) -> c_uint {
        super::stdc_trailing_zeros_ull(n)
    }
}
