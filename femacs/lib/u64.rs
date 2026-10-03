// femacs: port of lib/u64.c @ fd7afced403.
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong,
                c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/u64.rs");
/* lib/u64.c's `typedef int dummy;` (a link-time marker in gnulib).  */
pub type dummy = c_int;

/* lib/u64.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_U64_INLINE: () = _GL_EXTERN_INLINE;

/* lib/u64.c is a gnulib `_GL_*_INLINE` wrapper: its C text is the license, the
   `#define <qualifier> _GL_EXTERN_INLINE` line and the includes, i.e. the C99 inline model's
   "this translation unit emits the external definition" file.  The body lives in the header
   (mirrored as `lib/include/u64.rs`) and C's object exports the symbol below; Rust's
   mirror keeps the body as an inline function, which emits nothing, so the unit re-exports it
   under C's own name (PORTING.md P-81).  */
mod u64_unit {
  use super::*;

  #[unsafe(export_name = "u64rol")]
  pub unsafe extern "C" fn u64rol (x: u64, n: c_int) -> u64
  {
    return super::u64rol (x, n);
  }
}
