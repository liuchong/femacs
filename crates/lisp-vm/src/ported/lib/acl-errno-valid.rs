// femacs: port of lib/acl-errno-valid.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::sys_common::*;

pub const fn acl_errno_valid(errnum: c_int) -> bool {
    crate::sys_common::acl_errno_valid(errnum)
}
