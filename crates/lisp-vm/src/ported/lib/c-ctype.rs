// femacs: port of lib/c-ctype.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::sys_common::*;

pub const C_CTYPE_INLINE: () = _GL_EXTERN_INLINE;

mod c_ctype_unit {
    use super::*;
    pub fn c_isalnum(c: c_int) -> bool {
        crate::sys::c_ctype::c_isalnum(c)
    }
    pub fn c_isalpha(c: c_int) -> bool {
        crate::sys::c_ctype::c_isalpha(c)
    }
    pub fn c_isascii(c: c_int) -> bool {
        crate::sys::c_ctype::c_isascii(c)
    }
    pub fn c_isblank(c: c_int) -> bool {
        crate::sys::c_ctype::c_isblank(c)
    }
    pub fn c_iscntrl(c: c_int) -> bool {
        crate::sys::c_ctype::c_iscntrl(c)
    }
    pub fn c_isdigit(c: c_int) -> bool {
        crate::sys::c_ctype::c_isdigit(c)
    }
    pub fn c_isgraph(c: c_int) -> bool {
        crate::sys::c_ctype::c_isgraph(c)
    }
    pub fn c_islower(c: c_int) -> bool {
        crate::sys::c_ctype::c_islower(c)
    }
    pub fn c_isprint(c: c_int) -> bool {
        crate::sys::c_ctype::c_isprint(c)
    }
    pub fn c_ispunct(c: c_int) -> bool {
        crate::sys::c_ctype::c_ispunct(c)
    }
    pub fn c_isspace(c: c_int) -> bool {
        crate::sys::c_ctype::c_isspace(c)
    }
    pub fn c_isupper(c: c_int) -> bool {
        crate::sys::c_ctype::c_isupper(c)
    }
    pub fn c_isxdigit(c: c_int) -> bool {
        crate::sys::c_ctype::c_isxdigit(c)
    }
    pub fn c_tolower(c: c_int) -> c_int {
        crate::sys::c_ctype::c_tolower(c)
    }
    pub fn c_toupper(c: c_int) -> c_int {
        crate::sys::c_ctype::c_toupper(c)
    }
}
