// femacs: port of lib/stat-time.h @ fd7afced403.
#![forbid(unsafe_code)]

use super::*;

const _: () = core::assert!(
    !cfg!(feature = "stat_time_stat_timespec_ns"),
    "the frozen configuration defines HAVE_STRUCT_STAT_ST_ATIMESPEC_TV_NSEC"
);
const _: () = core::assert!(
    !cfg!(feature = "stat_time_member_absent"),
    "the frozen configuration's struct stat carries the timespec members"
);

pub const STAT_TIME_H: c_int = 1;

#[cfg(not(feature = "have_android"))]
macro_rules! STAT_TIMESPEC {
    ($st:expr, st_atim) => {
        ($st).st_atimespec
    };
    ($st:expr, st_ctim) => {
        ($st).st_ctimespec
    };
    ($st:expr, st_mtim) => {
        ($st).st_mtimespec
    };
    ($st:expr, st_birthtim) => {
        ($st).st_birthtimespec
    };
}
#[cfg(feature = "have_android")]
macro_rules! STAT_TIMESPEC {
    ($st:expr, st_atim) => {
        ($st).st_atim
    };
    ($st:expr, st_ctim) => {
        ($st).st_ctim
    };
    ($st:expr, st_mtim) => {
        ($st).st_mtim
    };
}

#[cfg(not(feature = "have_android"))]
macro_rules! STAT_TIMESPEC_OFFSETOF {
    (st_atim) => {
        std::mem::offset_of!(stat, st_atimespec)
    };
    (st_mtim) => {
        std::mem::offset_of!(stat, st_mtimespec)
    };
    (st_ctim) => {
        std::mem::offset_of!(stat, st_ctimespec)
    };
}
#[cfg(feature = "have_android")]
macro_rules! STAT_TIMESPEC_OFFSETOF {
    (st_atim) => {
        std::mem::offset_of!(stat, st_atim)
    };
    (st_mtim) => {
        std::mem::offset_of!(stat, st_mtim)
    };
    (st_ctim) => {
        std::mem::offset_of!(stat, st_ctim)
    };
}

#[inline(always)]
pub const fn _gl_make_timespec(sec: time_t, nsec: c_long) -> timespec {
    timespec {
        tv_sec: sec,
        tv_nsec: nsec,
    }
}

#[inline(always)]
pub const fn get_stat_atime_ns(st: &stat) -> c_long {
    STAT_TIMESPEC!(st, st_atim).tv_nsec
}

#[inline(always)]
pub const fn get_stat_ctime_ns(st: &stat) -> c_long {
    STAT_TIMESPEC!(st, st_ctim).tv_nsec
}

#[inline(always)]
pub const fn get_stat_mtime_ns(st: &stat) -> c_long {
    STAT_TIMESPEC!(st, st_mtim).tv_nsec
}

#[inline(always)]
pub const fn get_stat_birthtime_ns(st: &stat) -> c_long {
    #[cfg(feature = "have_android")]
    {
        let _ = st;
        0
    }
    #[cfg(not(feature = "have_android"))]
    {
        STAT_TIMESPEC!(st, st_birthtim).tv_nsec
    }
}

#[inline(always)]
pub const fn get_stat_atime(st: &stat) -> timespec {
    STAT_TIMESPEC!(st, st_atim)
}

#[inline(always)]
pub const fn get_stat_ctime(st: &stat) -> timespec {
    STAT_TIMESPEC!(st, st_ctim)
}

#[inline(always)]
pub const fn get_stat_mtime(st: &stat) -> timespec {
    STAT_TIMESPEC!(st, st_mtim)
}

#[inline(always)]
pub const fn get_stat_birthtime(st: &stat) -> timespec {
    #[cfg(feature = "have_android")]
    let mut t: timespec = {
        let _ = st;
        _gl_make_timespec(-1, -1)
    };
    #[cfg(not(feature = "have_android"))]
    let mut t: timespec = STAT_TIMESPEC!(st, st_birthtim);

    if !(t.tv_sec != 0 && 0 <= t.tv_nsec && t.tv_nsec < 1_000_000_000) {
        t = _gl_make_timespec(-1, -1);
    }
    t
}

#[inline]
fn normalize_one_timespec(ts: &mut timespec) -> bool {
    const TIMESPEC_HZ: c_long = 1_000_000_000;
    let mut q: c_long = ts.tv_nsec / TIMESPEC_HZ;
    let mut r: c_long = ts.tv_nsec % TIMESPEC_HZ;
    if r < 0 {
        r += TIMESPEC_HZ;
        q -= 1;
    }
    ts.tv_nsec = r;
    let (sum, overflow) = q.overflowing_add(ts.tv_sec);
    ts.tv_sec = sum;
    !overflow
}

#[inline(always)]
pub fn stat_time_normalize(result: c_int, st: &mut stat, err_out: &mut c_int) -> c_int {
    if result == 0 {
        let ok = normalize_one_timespec(&mut STAT_TIMESPEC!(st, st_atim))
            && normalize_one_timespec(&mut STAT_TIMESPEC!(st, st_mtim))
            && normalize_one_timespec(&mut STAT_TIMESPEC!(st, st_ctim));
        if !ok {
            *err_out = EOVERFLOW;
            return -1;
        }
    }
    result
}
