// femacs: port of lib/stdc_count_ones.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

// [U1] include/gnulib_config.rs imported via common
// [U1] include/stdint.in.rs imported via common
// [U1] include/stdbit.in.rs imported via common
// [U1] include/limits.in.rs imported via common
/* lib/stdc_count_ones.c's dead `#if 1500 <= _MSC_VER` block declares this object.  */
#[cfg(feature = "msvc")] #[unsafe(no_mangle)]
pub static mut _gl_stdbit_popcount_support: c_schar = 0;

/* lib/stdc_count_ones.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STDC_COUNT_ONES_INLINE: () = _GL_EXTERN_INLINE;

/* lib/stdc_count_ones.c is a gnulib `_GL_*_INLINE` wrapper: the bodies live in <stdbit.h>
   (mirrored as lib/include/stdbit.rs) and C's object exports the five typed entry points, so
   the unit re-exports them (PORTING.md P-81).  */
mod stdc_count_ones_unit {
  use super::*;

  #[unsafe(export_name = "stdc_count_ones_ui")]
  pub unsafe extern "C" fn stdc_count_ones_ui (n: c_uint) -> c_uint
  {
    return super::stdc_count_ones_ui (n);
  }

  #[unsafe(export_name = "stdc_count_ones_uc")]
  pub unsafe extern "C" fn stdc_count_ones_uc (n: c_uchar) -> c_uint
  {
    return super::stdc_count_ones_uc (n);
  }

  #[unsafe(export_name = "stdc_count_ones_us")]
  pub unsafe extern "C" fn stdc_count_ones_us (n: c_ushort) -> c_uint
  {
    return super::stdc_count_ones_us (n);
  }

  #[unsafe(export_name = "stdc_count_ones_ul")]
  pub unsafe extern "C" fn stdc_count_ones_ul (n: c_ulong) -> c_uint
  {
    return super::stdc_count_ones_ul (n);
  }

  #[unsafe(export_name = "stdc_count_ones_ull")]
  pub unsafe extern "C" fn stdc_count_ones_ull (n: c_ulonglong) -> c_uint
  {
    return super::stdc_count_ones_ull (n);
  }
}
