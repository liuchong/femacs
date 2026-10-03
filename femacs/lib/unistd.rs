// femacs: port of lib/unistd.c @ fd7afced403.
use std::ffi::c_int;
include!("include/gnulib_config.rs");
/* lib/unistd.c's `typedef int dummy;` (a link-time marker in gnulib).  */
pub type dummy = c_int;

/* lib/unistd.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_UNISTD_INLINE: () = _GL_EXTERN_INLINE;
