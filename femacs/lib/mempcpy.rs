// femacs: port of lib/mempcpy.c @ fd7afced403.
use std::ffi::{c_char, c_int, c_void};
include!("../src/include/config.rs"); include!("include/gnulib_config.rs");

unsafe extern "C" { pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void; }

/* Copy N bytes of SRC to DEST, return pointer to bytes after the
   last written byte.  */
/* The frozen configuration has HAVE_MEMPCPY undefined, so C's `#if !HAVE_MEMPCPY` body is
   live; gnulib's <string.h> renames the identifier to `rpl_mempcpy` in this translation
   unit (which is the symbol doprnt.rs declares), so the port keeps C's function name for
   the ledger and emits C's real symbol through export_name (PORTING.md 10.13).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mempcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void
{
  return (memcpy (dest, src, n) as *mut c_char).offset (n as isize) as *mut c_void;
}
