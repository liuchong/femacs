// femacs: port of lib/byteswap.c @ fd7afced403.
use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/byteswap.in.rs");

/* lib/byteswap.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_BYTESWAP_INLINE: () = _GL_EXTERN_INLINE;

/* lib/byteswap.c is a gnulib `_GL_*_INLINE` wrapper: the bodies live in the header mirrored
   as lib/include/byteswap.in.rs and C's object exports them, so the unit re-exports them
   (PORTING.md P-81).  `super::` because the module's own item would shadow the mirror's
   and recurse.  */
mod byteswap_unit {
  use super::*;
  #[unsafe(export_name = "bswap_16")]
  pub unsafe extern "C" fn bswap_16 (x: uint_least16_t) -> uint_least16_t
  {
    return super::bswap_16 (x);
  }

  #[unsafe(export_name = "bswap_32")]
  pub unsafe extern "C" fn bswap_32 (x: uint_least32_t) -> uint_least32_t
  {
    return super::bswap_32 (x);
  }

  #[unsafe(export_name = "bswap_64")]
  pub unsafe extern "C" fn bswap_64 (x: uint_least64_t) -> uint_least64_t
  {
    return super::bswap_64 (x);
  }
}
