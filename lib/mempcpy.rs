// femacs: port of lib/mempcpy.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_void};
// [U1] config.rs provided by common

unsafe extern "C" { pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void; }

/* Copy N bytes of SRC to DEST, return pointer to bytes after the
   last written byte.  */
/* The frozen configuration has HAVE_MEMPCPY undefined, so C's `#if !HAVE_MEMPCPY` body is live
   and the definition is C's own `mempcpy`: the generated `<string.h>`'s rename (`# undef
   mempcpy` / `# define mempcpy rpl_mempcpy`) sits inside `# if 0` (its line 1025), so this
   translation unit defines the unrenamed name - measured: `nm -g build/emacs-ref/lib/mempcpy.o`
   prints `T _mempcpy`.  (Status 3.112 read those two lines without the enclosing `#if 0` and
   made the port emit `rpl_mempcpy`; status 3.205 corrects it with `check-symbols.py` as the
   authority.)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mempcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void
{
  return (memcpy (dest, src, n) as *mut c_char).offset (n as isize) as *mut c_void;
}
