// femacs: port of lib/timespec.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;

pub const _GL_TIMESPEC_INLINE: () = _GL_EXTERN_INLINE;

mod timespec_unit {
    use super::*;

    pub fn make_timespec(s: time_t, ns: c_long) -> timespec {
        super::make_timespec(s, ns)
    }

    pub fn timespec_cmp(a: timespec, b: timespec) -> c_int {
        super::timespec_cmp(a, b)
    }

    pub fn timespec_sign(a: timespec) -> c_int {
        super::timespec_sign(a)
    }

    pub fn timespectod(a: timespec) -> c_double {
        super::timespectod(a)
    }
}
