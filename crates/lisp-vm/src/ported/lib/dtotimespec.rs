// femacs: port of lib/dtotimespec.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::sys_common::*;

pub fn dtotimespec(sec: c_double) -> timespec {
    super::sys_common::dtotimespec(sec)
}
