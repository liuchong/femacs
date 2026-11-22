// femacs: port of lib/timespec-add.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;

pub fn timespec_add(a: timespec, b: timespec) -> timespec {
    super::common::timespec_add(a, b)
}
