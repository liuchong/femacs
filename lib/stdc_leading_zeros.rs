// femacs: port of lib/stdc_leading_zeros.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

// [U1] include/gnulib_config.rs imported via common
// [U1] include/stdint.in.rs imported via common
// [U1] include/stdbit.in.rs imported via common

/* lib/stdc_leading_zeros.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STDC_LEADING_ZEROS_INLINE: () = _GL_EXTERN_INLINE;

/* lib/stdc_leading_zeros.c is a gnulib `_GL_*_INLINE` wrapper: the bodies live in <stdbit.h>
   (mirrored as lib/include/stdbit.rs) and C's object exports the functions below, so the
   unit re-exports them (PORTING.md P-81).  The call is spelled `super::` because the
   module's own item would otherwise shadow the mirror's and recurse.  */
mod stdc_leading_zeros_unit {
  use super::*;

  #[unsafe(export_name = "_gl_stdbit_clz")]
  pub unsafe extern "C" fn _gl_stdbit_clz (n: c_uint) -> c_int
  {
    return super::_gl_stdbit_clz (n);
  }

  #[unsafe(export_name = "_gl_stdbit_clzl")]
  pub unsafe extern "C" fn _gl_stdbit_clzl (n: c_ulong) -> c_int
  {
    return super::_gl_stdbit_clzl (n);
  }

  #[unsafe(export_name = "_gl_stdbit_clzll")]
  pub unsafe extern "C" fn _gl_stdbit_clzll (n: c_ulonglong) -> c_int
  {
    return super::_gl_stdbit_clzll (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_ui")]
  pub unsafe extern "C" fn stdc_leading_zeros_ui (n: c_uint) -> c_uint
  {
    return super::stdc_leading_zeros_ui (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_uc")]
  pub unsafe extern "C" fn stdc_leading_zeros_uc (n: c_uchar) -> c_uint
  {
    return super::stdc_leading_zeros_uc (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_us")]
  pub unsafe extern "C" fn stdc_leading_zeros_us (n: c_ushort) -> c_uint
  {
    return super::stdc_leading_zeros_us (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_ul")]
  pub unsafe extern "C" fn stdc_leading_zeros_ul (n: c_ulong) -> c_uint
  {
    return super::stdc_leading_zeros_ul (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_ull")]
  pub unsafe extern "C" fn stdc_leading_zeros_ull (n: c_ulonglong) -> c_uint
  {
    return super::stdc_leading_zeros_ull (n);
  }
}
