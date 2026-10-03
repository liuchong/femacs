// femacs: port of lib/stat-time.c @ fd7afced403.
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong,
                c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/stat-time.rs");

/* lib/stat-time.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STAT_TIME_INLINE: () = _GL_EXTERN_INLINE;

/* lib/stat_time.c is a gnulib `_GL_*_INLINE` wrapper: its C text is the license, the
   `#define <qualifier> _GL_EXTERN_INLINE` line and the includes, i.e. the C99 inline model's
   "this translation unit emits the external definition" file.  The body lives in the header
   (mirrored as `lib/include/stat-time.rs`) and C's object exports the symbol below; Rust's
   mirror keeps the body as an inline function, which emits nothing, so the unit re-exports it
   under C's own name (PORTING.md P-81).  */
mod stat_time_unit {
  use super::*;

  #[unsafe(export_name = "get_stat_atime_ns")]
  pub unsafe extern "C" fn get_stat_atime_ns (st: *const stat) -> c_long
  {
    return super::get_stat_atime_ns (st);
  }

  #[unsafe(export_name = "get_stat_ctime_ns")]
  pub unsafe extern "C" fn get_stat_ctime_ns (st: *const stat) -> c_long
  {
    return super::get_stat_ctime_ns (st);
  }

  #[unsafe(export_name = "get_stat_mtime_ns")]
  pub unsafe extern "C" fn get_stat_mtime_ns (st: *const stat) -> c_long
  {
    return super::get_stat_mtime_ns (st);
  }

  #[unsafe(export_name = "get_stat_birthtime_ns")]
  pub unsafe extern "C" fn get_stat_birthtime_ns (st: *const stat) -> c_long
  {
    return super::get_stat_birthtime_ns (st);
  }

  #[unsafe(export_name = "get_stat_atime")]
  pub unsafe extern "C" fn get_stat_atime (st: *const stat) -> timespec
  {
    return super::get_stat_atime (st);
  }

  #[unsafe(export_name = "get_stat_ctime")]
  pub unsafe extern "C" fn get_stat_ctime (st: *const stat) -> timespec
  {
    return super::get_stat_ctime (st);
  }

  #[unsafe(export_name = "get_stat_mtime")]
  pub unsafe extern "C" fn get_stat_mtime (st: *const stat) -> timespec
  {
    return super::get_stat_mtime (st);
  }

  #[unsafe(export_name = "get_stat_birthtime")]
  pub unsafe extern "C" fn get_stat_birthtime (st: *const stat) -> timespec
  {
    return super::get_stat_birthtime (st);
  }

  #[unsafe(export_name = "stat_time_normalize")]
  pub unsafe extern "C" fn stat_time_normalize (result: c_int, st: *mut stat) -> c_int
  {
    return super::stat_time_normalize (result, st);
  }
}
