// femacs: port of lib/timespec-sub.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;

pub fn timespec_sub(a: timespec, b: timespec) -> timespec {
    super::common::timespec_sub(a, b)
}
