// femacs: port of lib/streq.c @ fd7afced403.
use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

include!("include/gnulib_config.rs");
include!("include/string.rs");

/* lib/streq.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STREQ_INLINE: () = _GL_EXTERN_INLINE;

/* lib/streq.c is a gnulib `_GL_*_INLINE` wrapper: the body lives in <string.h> (mirrored
   as lib/include/string.rs) and C's object exports it, so the unit re-exports the
   definition (PORTING.md P-81).  `super::` because the module's own item would shadow the
   mirror's and recurse.  */
mod streq_unit {
  use super::*;

  #[unsafe(export_name = "streq")]
  pub unsafe extern "C" fn streq (s1: *const c_char, s2: *const c_char) -> bool
  {
    return super::streq (s1, s2);
  }
}
