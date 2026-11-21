// femacs: port of lib/unistd.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::c_int;
// [U1] include/gnulib_config.rs imported via common
/* lib/unistd.c's `typedef int dummy;` (a link-time marker in gnulib).  */
pub type dummy = c_int;

/* lib/unistd.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_UNISTD_INLINE: () = _GL_EXTERN_INLINE;
