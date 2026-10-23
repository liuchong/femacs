// femacs: port of lib/timespec.c @ fd7afced403.
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong,
                c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/timespec.rs");

/* lib/timespec.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_TIMESPEC_INLINE: () = _GL_EXTERN_INLINE;

/* lib/timespec.c is a gnulib `_GL_*_INLINE` wrapper: its C text carries no code — only the
   `#define <qualifier> _GL_EXTERN_INLINE` line and the includes, i.e. the C99 inline model's
   "this translation unit emits the external definition" file.  The body lives in the header
   (mirrored as `lib/include/timespec.rs`) and C's object exports the symbol below; Rust's
   mirror keeps the body as an inline function, which emits nothing, so the unit re-exports it
   under C's own name (PORTING.md P-81).  */
mod timespec_unit {
  use super::*;

  #[unsafe(export_name = "make_timespec")]
  pub unsafe extern "C" fn make_timespec (s: time_t, ns: c_long) -> timespec
  {
    return super::make_timespec (s, ns);
  }

  #[unsafe(export_name = "timespec_cmp")]
  pub unsafe extern "C" fn timespec_cmp (a: timespec, b: timespec) -> c_int
  {
    return super::timespec_cmp (a, b);
  }

  #[unsafe(export_name = "timespec_sign")]
  pub unsafe extern "C" fn timespec_sign (a: timespec) -> c_int
  {
    return super::timespec_sign (a);
  }

  #[unsafe(export_name = "timespectod")]
  pub unsafe extern "C" fn timespectod (a: timespec) -> c_double
  {
    return super::timespectod (a);
  }
}
