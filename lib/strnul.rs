// femacs: port of lib/strnul.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong,
                c_void};

// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/string.rs imported via common

/* lib/strnul.c: the qualifier this translation unit uses (PORTING.md 10.6).  */
pub const _GL_STRNUL_INLINE: () = _GL_EXTERN_INLINE;

/* lib/strnul.c is a gnulib `_GL_*_INLINE` wrapper: its C text carries no code — only the
   `#define <qualifier> _GL_EXTERN_INLINE` line and the includes, i.e. the C99 inline model's
   "this translation unit emits the external definition" file.  The body lives in the header
   (mirrored as `lib/include/string.rs`) and C's object exports the symbol below; Rust's
   mirror keeps the body as an inline function, which emits nothing, so the unit re-exports it
   under C's own name (PORTING.md P-81).  */
mod strnul_unit {
  use super::*;

  #[unsafe(export_name = "_gl_strnul")]
  pub unsafe extern "C" fn _gl_strnul (string: *const c_char) -> *const c_char
  {
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
    return super::_gl_strnul (string);
  
  }
}
}
