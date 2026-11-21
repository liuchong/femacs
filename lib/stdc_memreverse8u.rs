// femacs: port of lib/stdc_memreverse8u.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

// [U1] include/gnulib_config.rs imported via common
// [U1] include/stdint.in.rs imported via common
// [U1] include/stdbit.in.rs imported via common

/* lib/stdc_memreverse8u.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STDC_MEMREVERSE8U_INLINE: () = _GL_EXTERN_INLINE;

/* lib/stdc_memreverse8u.c is a gnulib `_GL_*_INLINE` wrapper: the bodies live in <stdbit.h>
   (mirrored as lib/include/stdbit.rs) and C's object exports the functions below, so the
   unit re-exports them (PORTING.md P-81).  The call is spelled `super::` because the
   module's own item would otherwise shadow the mirror's and recurse.  */
mod stdc_memreverse8u_unit {
  use super::*;

  #[unsafe(export_name = "stdc_memreverse8u8")]
  pub unsafe extern "C" fn stdc_memreverse8u8 (value: uint_least8_t) -> uint_least8_t
  {
    return super::stdc_memreverse8u8 (value);
  }

  #[unsafe(export_name = "stdc_memreverse8u16")]
  pub unsafe extern "C" fn stdc_memreverse8u16 (value: uint_least16_t) -> uint_least16_t
  {
    return super::stdc_memreverse8u16 (value);
  }

  #[unsafe(export_name = "stdc_memreverse8u32")]
  pub unsafe extern "C" fn stdc_memreverse8u32 (value: uint_least32_t) -> uint_least32_t
  {
    return super::stdc_memreverse8u32 (value);
  }

  #[unsafe(export_name = "stdc_memreverse8u64")]
  pub unsafe extern "C" fn stdc_memreverse8u64 (value: uint_least64_t) -> uint_least64_t
  {
    return super::stdc_memreverse8u64 (value);
  }
}
