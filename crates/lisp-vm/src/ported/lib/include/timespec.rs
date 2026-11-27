// femacs: port of lib/timespec.h @ fd7afced403.
use super::*;

pub const TIMESPEC_H: bool = true;
const _: () = core::assert!(_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

pub const TIMESPEC_HZ: c_int = 1_000_000_000;
pub const LOG10_TIMESPEC_HZ: c_int = 9;
pub const TIMESPEC_RESOLUTION: c_int = TIMESPEC_HZ;
pub const LOG10_TIMESPEC_RESOLUTION: c_int = LOG10_TIMESPEC_HZ;

#[inline(always)]
pub const fn make_timespec(s: time_t, ns: c_long) -> timespec {
    timespec { tv_sec: s, tv_nsec: ns }
}

#[inline(always)]
pub const fn timespec_cmp(a: timespec, b: timespec) -> c_int {
    crate::sys::timespec::timespec_cmp(a.tv_sec, a.tv_nsec, b.tv_sec, b.tv_nsec)
}

#[inline(always)]
pub const fn timespec_sign(a: timespec) -> c_int {
    crate::sys::timespec::timespec_sign(a.tv_sec, a.tv_nsec)
}

#[inline(always)]
pub fn timespectod(a: timespec) -> c_double {
    crate::sys::timespec::timespectod(a.tv_sec, a.tv_nsec)
}

#[inline(always)]
pub fn timespec_add(a: timespec, b: timespec) -> timespec {
    let (s, ns) = crate::sys::timespec::timespec_add(a.tv_sec, a.tv_nsec, b.tv_sec, b.tv_nsec);
    make_timespec(s, ns as c_long)
}

#[inline(always)]
pub fn timespec_sub(a: timespec, b: timespec) -> timespec {
    let (s, ns) = crate::sys::timespec::timespec_sub(a.tv_sec, a.tv_nsec, b.tv_sec, b.tv_nsec);
    make_timespec(s, ns as c_long)
}

#[inline(always)]
pub fn dtotimespec(sec: c_double) -> timespec {
    let (s, ns) = crate::sys::timespec::dtotimespec(sec);
    make_timespec(s, ns as c_long)
}

pub mod timespec_decls {
    use super::*;

    unsafe extern "C" {
        pub fn gettime_res() -> c_long;
        pub fn current_timespec() -> timespec;
        pub fn gettime(ts: *mut timespec);
        pub fn settime(ts: *const timespec) -> c_int;
    }
}
pub use self::timespec_decls::*;
