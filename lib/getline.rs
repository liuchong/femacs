// femacs: port of lib/getline.c @ fd7afced403.
use std::ffi::{c_char, c_int, c_void};
include!("include/gnulib_config.rs");
include!("../lib/include/stdio.rs");
unsafe extern "C" {
  pub fn getdelim (lineptr: *mut *mut c_char, n: *mut usize, delim: c_int, stream: *mut FILE) -> isize;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getline (lineptr: *mut *mut c_char, n: *mut usize, stream: *mut FILE) -> isize
{
  return getdelim (lineptr, n, b'\n' as c_int, stream);
}
