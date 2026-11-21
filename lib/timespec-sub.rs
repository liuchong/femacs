// femacs: port of lib/timespec-sub.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

#[unsafe(no_mangle)]
pub extern "C" fn timespec_sub(a: timespec, b: timespec) -> timespec {
    let (s, ns) = femacs_pure::timespec::timespec_sub(a.tv_sec, a.tv_nsec, b.tv_sec, b.tv_nsec);
    make_timespec(s, ns as c_long)
}
