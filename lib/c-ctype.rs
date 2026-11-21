// femacs: port of lib/c-ctype.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

pub const C_CTYPE_INLINE: () = _GL_EXTERN_INLINE;

mod c_ctype_unit {
    use super::*;

    #[unsafe(export_name = "c_isalnum")]
    pub extern "C" fn c_isalnum(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isalnum(c)
    }

    #[unsafe(export_name = "c_isalpha")]
    pub extern "C" fn c_isalpha(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isalpha(c)
    }

    #[unsafe(export_name = "c_isascii")]
    pub extern "C" fn c_isascii(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isascii(c)
    }

    #[unsafe(export_name = "c_isblank")]
    pub extern "C" fn c_isblank(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isblank(c)
    }

    #[unsafe(export_name = "c_iscntrl")]
    pub extern "C" fn c_iscntrl(c: c_int) -> bool {
        femacs_pure::c_ctype::c_iscntrl(c)
    }

    #[unsafe(export_name = "c_isdigit")]
    pub extern "C" fn c_isdigit(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isdigit(c)
    }

    #[unsafe(export_name = "c_isgraph")]
    pub extern "C" fn c_isgraph(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isgraph(c)
    }

    #[unsafe(export_name = "c_islower")]
    pub extern "C" fn c_islower(c: c_int) -> bool {
        femacs_pure::c_ctype::c_islower(c)
    }

    #[unsafe(export_name = "c_isprint")]
    pub extern "C" fn c_isprint(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isprint(c)
    }

    #[unsafe(export_name = "c_ispunct")]
    pub extern "C" fn c_ispunct(c: c_int) -> bool {
        femacs_pure::c_ctype::c_ispunct(c)
    }

    #[unsafe(export_name = "c_isspace")]
    pub extern "C" fn c_isspace(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isspace(c)
    }

    #[unsafe(export_name = "c_isupper")]
    pub extern "C" fn c_isupper(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isupper(c)
    }

    #[unsafe(export_name = "c_isxdigit")]
    pub extern "C" fn c_isxdigit(c: c_int) -> bool {
        femacs_pure::c_ctype::c_isxdigit(c)
    }

    #[unsafe(export_name = "c_tolower")]
    pub extern "C" fn c_tolower(c: c_int) -> c_int {
        femacs_pure::c_ctype::c_tolower(c)
    }

    #[unsafe(export_name = "c_toupper")]
    pub extern "C" fn c_toupper(c: c_int) -> c_int {
        femacs_pure::c_ctype::c_toupper(c)
    }
}
