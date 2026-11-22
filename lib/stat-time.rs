// femacs: port of lib/stat-time.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::common::*;

pub const _GL_STAT_TIME_INLINE: () = _GL_EXTERN_INLINE;

mod stat_time_unit {
    use super::*;

    pub const fn get_stat_atime_ns(st: &stat) -> c_long {
        super::get_stat_atime_ns(st)
    }
    pub const fn get_stat_ctime_ns(st: &stat) -> c_long {
        super::get_stat_ctime_ns(st)
    }
    pub const fn get_stat_mtime_ns(st: &stat) -> c_long {
        super::get_stat_mtime_ns(st)
    }
    pub const fn get_stat_birthtime_ns(st: &stat) -> c_long {
        super::get_stat_birthtime_ns(st)
    }
    pub const fn get_stat_atime(st: &stat) -> timespec {
        super::get_stat_atime(st)
    }
    pub const fn get_stat_ctime(st: &stat) -> timespec {
        super::get_stat_ctime(st)
    }
    pub const fn get_stat_mtime(st: &stat) -> timespec {
        super::get_stat_mtime(st)
    }
    pub const fn get_stat_birthtime(st: &stat) -> timespec {
        super::get_stat_birthtime(st)
    }
    pub fn stat_time_normalize(result: c_int, st: &mut stat, err_out: &mut c_int) -> c_int {
        super::stat_time_normalize(result, st, err_out)
    }
}
