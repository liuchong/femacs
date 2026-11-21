// femacs: port of lib/stdc_bit_width.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

// [U1] include/gnulib_config.rs imported via common
// [U1] include/stdint.in.rs imported via common
// [U1] include/stdbit.in.rs imported via common

/* lib/stdc_bit_width.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STDC_BIT_WIDTH_INLINE: () = _GL_EXTERN_INLINE;

/* lib/stdc_bit_width.c is a gnulib `_GL_*_INLINE` wrapper: the bodies live in <stdbit.h>
   (mirrored as lib/include/stdbit.rs) and C's object exports the functions below, so the
   unit re-exports them (PORTING.md P-81).  The call is spelled `super::` because the
   module's own item would otherwise shadow the mirror's and recurse.  */
mod stdc_bit_width_unit {
  use super::*;

  #[unsafe(export_name = "stdc_bit_width_uc")]
  pub unsafe extern "C" fn stdc_bit_width_uc (n: c_uchar) -> c_uint
  {
    return super::stdc_bit_width_uc (n);
  }

  #[unsafe(export_name = "stdc_bit_width_us")]
  pub unsafe extern "C" fn stdc_bit_width_us (n: c_ushort) -> c_uint
  {
    return super::stdc_bit_width_us (n);
  }

  #[unsafe(export_name = "stdc_bit_width_ui")]
  pub unsafe extern "C" fn stdc_bit_width_ui (n: c_uint) -> c_uint
  {
    return super::stdc_bit_width_ui (n);
  }

  #[unsafe(export_name = "stdc_bit_width_ul")]
  pub unsafe extern "C" fn stdc_bit_width_ul (n: c_ulong) -> c_uint
  {
    return super::stdc_bit_width_ul (n);
  }

  #[unsafe(export_name = "stdc_bit_width_ull")]
  pub unsafe extern "C" fn stdc_bit_width_ull (n: c_ulonglong) -> c_uint
  {
    return super::stdc_bit_width_ull (n);
  }
}
