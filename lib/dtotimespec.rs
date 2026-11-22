// femacs: port of lib/dtotimespec.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;

pub fn dtotimespec(sec: c_double) -> timespec {
    super::common::dtotimespec(sec)
}
