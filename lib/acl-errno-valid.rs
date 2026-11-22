// femacs: port of lib/acl-errno-valid.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;

pub const fn acl_errno_valid(errnum: c_int) -> bool {
    crate::common::acl_errno_valid(errnum)
}
