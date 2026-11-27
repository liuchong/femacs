// femacs: port of lib/timespec-sub.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::sys_common::*;

pub fn timespec_sub(a: timespec, b: timespec) -> timespec {
    super::sys_common::timespec_sub(a, b)
}
