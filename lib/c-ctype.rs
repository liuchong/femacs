// femacs: port of lib/c-ctype.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

// [U1] include/gnulib_config.rs imported via common
// [U1] include/c-ctype.rs imported via common

/* lib/c-ctype.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const C_CTYPE_INLINE: () = _GL_EXTERN_INLINE;

/* lib/c-ctype.c is a gnulib `C_CTYPE_INLINE` wrapper: the fifteen bodies live in
   lib/c-ctype.h (mirrored as lib/include/c-ctype.rs) and C's object exports them
   (`T _c_isalnum` … `T _c_toupper`, tools/check-symbols.py), so the unit re-exports the
   definitions (PORTING.md P-81).  `super::` because the module's own item would shadow the
   mirror's and recurse.  */
mod c_ctype_unit {
  use super::*;

  #[unsafe(export_name = "c_isalnum")]
  pub unsafe extern "C" fn c_isalnum (c: c_int) -> bool
  {
    return super::c_isalnum (c);
  }

  #[unsafe(export_name = "c_isalpha")]
  pub unsafe extern "C" fn c_isalpha (c: c_int) -> bool
  {
    return super::c_isalpha (c);
  }

  #[unsafe(export_name = "c_isascii")]
  pub unsafe extern "C" fn c_isascii (c: c_int) -> bool
  {
    return super::c_isascii (c);
  }

  #[unsafe(export_name = "c_isblank")]
  pub unsafe extern "C" fn c_isblank (c: c_int) -> bool
  {
    return super::c_isblank (c);
  }

  #[unsafe(export_name = "c_iscntrl")]
  pub unsafe extern "C" fn c_iscntrl (c: c_int) -> bool
  {
    return super::c_iscntrl (c);
  }

  #[unsafe(export_name = "c_isdigit")]
  pub unsafe extern "C" fn c_isdigit (c: c_int) -> bool
  {
    return super::c_isdigit (c);
  }

  #[unsafe(export_name = "c_isgraph")]
  pub unsafe extern "C" fn c_isgraph (c: c_int) -> bool
  {
    return super::c_isgraph (c);
  }

  #[unsafe(export_name = "c_islower")]
  pub unsafe extern "C" fn c_islower (c: c_int) -> bool
  {
    return super::c_islower (c);
  }

  #[unsafe(export_name = "c_isprint")]
  pub unsafe extern "C" fn c_isprint (c: c_int) -> bool
  {
    return super::c_isprint (c);
  }

  #[unsafe(export_name = "c_ispunct")]
  pub unsafe extern "C" fn c_ispunct (c: c_int) -> bool
  {
    return super::c_ispunct (c);
  }

  #[unsafe(export_name = "c_isspace")]
  pub unsafe extern "C" fn c_isspace (c: c_int) -> bool
  {
    return super::c_isspace (c);
  }

  #[unsafe(export_name = "c_isupper")]
  pub unsafe extern "C" fn c_isupper (c: c_int) -> bool
  {
    return super::c_isupper (c);
  }

  #[unsafe(export_name = "c_isxdigit")]
  pub unsafe extern "C" fn c_isxdigit (c: c_int) -> bool
  {
    return super::c_isxdigit (c);
  }

  #[unsafe(export_name = "c_tolower")]
  pub unsafe extern "C" fn c_tolower (c: c_int) -> c_int
  {
    return super::c_tolower (c);
  }

  #[unsafe(export_name = "c_toupper")]
  pub unsafe extern "C" fn c_toupper (c: c_int) -> c_int
  {
    return super::c_toupper (c);
  }
}
