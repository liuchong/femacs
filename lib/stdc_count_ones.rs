// femacs: port of lib/stdc_count_ones.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

#[cfg(feature = "msvc")]
#[unsafe(no_mangle)]
pub static mut _gl_stdbit_popcount_support: c_schar = 0;

pub const _GL_STDC_COUNT_ONES_INLINE: () = _GL_EXTERN_INLINE;

mod stdc_count_ones_unit {
    use super::*;

    #[unsafe(export_name = "stdc_count_ones_ui")]
    pub extern "C" fn stdc_count_ones_ui(n: c_uint) -> c_uint {
        super::stdc_count_ones_ui(n)
    }
    #[unsafe(export_name = "stdc_count_ones_uc")]
    pub extern "C" fn stdc_count_ones_uc(n: c_uchar) -> c_uint {
        super::stdc_count_ones_uc(n)
    }
    #[unsafe(export_name = "stdc_count_ones_us")]
    pub extern "C" fn stdc_count_ones_us(n: c_ushort) -> c_uint {
        super::stdc_count_ones_us(n)
    }
    #[unsafe(export_name = "stdc_count_ones_ul")]
    pub extern "C" fn stdc_count_ones_ul(n: c_ulong) -> c_uint {
        super::stdc_count_ones_ul(n)
    }
    #[unsafe(export_name = "stdc_count_ones_ull")]
    pub extern "C" fn stdc_count_ones_ull(n: c_ulonglong) -> c_uint {
        super::stdc_count_ones_ull(n)
    }
}
