#![forbid(unsafe_code)]
//! Pure Safe Rust saturating `timespec` arithmetic (`lib/timespec*.c`, `lib/dtotimespec.c`).

pub const TIMESPEC_HZ: i64 = 1_000_000_000;
pub const TIMESPEC_HZ_I32: i32 = 1_000_000_000;

#[inline]
pub const fn cmp_i64(a: i64, b: i64) -> i32 {
    (a > b) as i32 - (a < b) as i32
}

#[inline]
pub const fn timespec_cmp(a_sec: i64, a_nsec: i64, b_sec: i64, b_nsec: i64) -> i32 {
    2 * cmp_i64(a_sec, b_sec) + cmp_i64(a_nsec, b_nsec)
}

#[inline]
pub const fn timespec_sign(sec: i64, nsec: i64) -> i32 {
    cmp_i64(sec | nsec, 0)
}

#[inline]
pub fn timespectod(sec: i64, nsec: i64) -> f64 {
    (sec as f64) + (nsec as f64) / 1e9
}

#[inline]
pub const fn timespec_add(a_sec: i64, a_nsec: i64, b_sec: i64, b_nsec: i64) -> (i64, i64) {
    let nssum = a_nsec.wrapping_add(b_nsec) as i32;
    let carry = (TIMESPEC_HZ_I32 <= nssum) as i32;
    let (mut rs, v) = a_sec.overflowing_add(b_sec);
    let (rs2, overflowed2) = rs.overflowing_add(carry as i64);
    rs = rs2;
    let rns = if v == overflowed2 {
        nssum - TIMESPEC_HZ_I32 * carry
    } else if (i64::MIN + i64::MAX) / 2 < rs {
        rs = i64::MIN;
        0
    } else {
        rs = i64::MAX;
        TIMESPEC_HZ_I32 - 1
    };
    (rs, rns as i64)
}

#[inline]
pub const fn timespec_sub(a_sec: i64, a_nsec: i64, b_sec: i64, b_nsec: i64) -> (i64, i64) {
    let nsdiff = a_nsec.wrapping_sub(b_nsec) as i32;
    let borrow = (nsdiff < 0) as i32;
    let (mut rs, v) = a_sec.overflowing_sub(b_sec);
    let (rs2, overflowed2) = rs.overflowing_sub(borrow as i64);
    rs = rs2;
    let rns = if v == overflowed2 {
        nsdiff + TIMESPEC_HZ_I32 * borrow
    } else if (i64::MIN + i64::MAX) / 2 < rs {
        rs = i64::MIN;
        0
    } else {
        rs = i64::MAX;
        TIMESPEC_HZ_I32 - 1
    };
    (rs, rns as i64)
}

#[inline]
pub fn dtotimespec(sec: f64) -> (i64, i64) {
    if !((i64::MIN as f64) < sec) {
        (i64::MIN, 0)
    } else if !(sec < 1.0 + (i64::MAX as f64)) {
        (i64::MAX, TIMESPEC_HZ - 1)
    } else {
        let mut s = sec as i64;
        let frac = (TIMESPEC_HZ as f64) * (sec - (s as f64));
        let mut ns = frac as i64;
        ns += i64::from((ns as f64) < frac);
        s += ns / TIMESPEC_HZ;
        ns %= TIMESPEC_HZ;
        if ns < 0 {
            s -= 1;
            ns += TIMESPEC_HZ;
        }
        (s, ns)
    }
}
