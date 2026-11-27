// femacs: port of lib/unistd.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::sys_common::*;

pub type dummy = c_int;
pub const _GL_UNISTD_INLINE: () = _GL_EXTERN_INLINE;
