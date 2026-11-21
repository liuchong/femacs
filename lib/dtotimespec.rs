// femacs: port of lib/dtotimespec.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

#[unsafe(no_mangle)]
pub extern "C" fn dtotimespec(sec: c_double) -> timespec {
    let (s, ns) = femacs_pure::timespec::dtotimespec(sec);
    make_timespec(s, ns as c_long)
}
