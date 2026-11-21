// femacs: port of lib/c-ctype.h @ fd7afced403.
#![forbid(unsafe_code)]
use super::*;

pub const C_CTYPE_ASCII: c_int = 1;
pub const C_CTYPE_H: bool = true;

macro_rules! _C_CTYPE_DIGIT { () => { 0x30..=0x39 }; }
macro_rules! _C_CTYPE_LOWER { () => { 0x61..=0x7a }; }
macro_rules! _C_CTYPE_UPPER { () => { 0x41..=0x5a }; }
macro_rules! _C_CTYPE_A_THRU_F { () => { 0x61..=0x66 | 0x41..=0x46 }; }
macro_rules! _C_CTYPE_PUNCT { () => { 0x21..=0x2f | 0x3a..=0x40 | 0x5b..=0x60 | 0x7b..=0x7e }; }
macro_rules! _C_CTYPE_CNTRL { () => { _C_CTYPE_OTHER_CNTRL! () | 0x07..=0x0d }; }
macro_rules! _C_CTYPE_OTHER_CNTRL { () => { 0x00..=0x06 | 0x0e..=0x1f | 0x7f }; }

#[inline(always)]
pub const fn c_isalnum(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isalnum(c)
}

#[inline(always)]
pub const fn c_isalpha(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isalpha(c)
}

#[inline(always)]
pub const fn c_isascii(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isascii(c)
}

#[inline(always)]
pub const fn c_isblank(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isblank(c)
}

#[inline(always)]
pub const fn c_iscntrl(c: c_int) -> bool {
    femacs_pure::c_ctype::c_iscntrl(c)
}

#[inline(always)]
pub const fn c_isdigit(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isdigit(c)
}

#[inline(always)]
pub const fn c_isgraph(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isgraph(c)
}

#[inline(always)]
pub const fn c_islower(c: c_int) -> bool {
    femacs_pure::c_ctype::c_islower(c)
}

#[inline(always)]
pub const fn c_isprint(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isprint(c)
}

#[inline(always)]
pub const fn c_ispunct(c: c_int) -> bool {
    femacs_pure::c_ctype::c_ispunct(c)
}

#[inline(always)]
pub const fn c_isspace(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isspace(c)
}

#[inline(always)]
pub const fn c_isupper(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isupper(c)
}

#[inline(always)]
pub const fn c_isxdigit(c: c_int) -> bool {
    femacs_pure::c_ctype::c_isxdigit(c)
}

#[inline(always)]
pub const fn c_tolower(c: c_int) -> c_int {
    femacs_pure::c_ctype::c_tolower(c)
}

#[inline(always)]
pub const fn c_toupper(c: c_int) -> c_int {
    femacs_pure::c_ctype::c_toupper(c)
}
