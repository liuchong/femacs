// femacs: port of lib/timespec.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

pub const _GL_TIMESPEC_INLINE: () = _GL_EXTERN_INLINE;

mod timespec_unit {
    use super::*;

    #[unsafe(export_name = "make_timespec")]
    pub extern "C" fn make_timespec(s: time_t, ns: c_long) -> timespec {
        super::make_timespec(s, ns)
    }

    #[unsafe(export_name = "timespec_cmp")]
    pub extern "C" fn timespec_cmp(a: timespec, b: timespec) -> c_int {
        super::timespec_cmp(a, b)
    }

    #[unsafe(export_name = "timespec_sign")]
    pub extern "C" fn timespec_sign(a: timespec) -> c_int {
        super::timespec_sign(a)
    }

    #[unsafe(export_name = "timespectod")]
    pub extern "C" fn timespectod(a: timespec) -> c_double {
        super::timespectod(a)
    }
}
