#![forbid(unsafe_code)]
//! Pure Safe Rust signal-name conversion (`lib/sig2str.c`) and description lookup (`lib/sigdescr_np.c`).

use core::ffi::CStr;

/// Parse a base-10 integer matching C `strtol(s, &endp, 10)` where `*endp == '\0'`.
/// Note: in C `strtol("", &endp, 10)` sets `endp = s` (`*endp == '\0'`) and returns `0`.
fn c_strtol_exhaustive(bytes: &[u8]) -> Option<i64> {
    if bytes.is_empty() {
        return Some(0);
    }
    let mut idx = 0usize;
    while idx < bytes.len() && matches!(bytes[idx], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        idx += 1;
    }
    if idx == bytes.len() {
        return Some(0);
    }
    let mut neg = false;
    if bytes[idx] == b'+' {
        idx += 1;
    } else if bytes[idx] == b'-' {
        neg = true;
        idx += 1;
    }
    if idx == bytes.len() || !bytes[idx].is_ascii_digit() {
        return None;
    }
    let mut acc: i128 = 0;
    while idx < bytes.len() {
        let b = bytes[idx];
        if !b.is_ascii_digit() {
            return None;
        }
        let d = (b - b'0') as i128;
        acc = acc.saturating_mul(10).saturating_add(d);
        idx += 1;
    }
    let signed = if neg { -acc } else { acc };
    let clamped = signed.clamp(i64::MIN as i128, i64::MAX as i128) as i64;
    Some(clamped)
}

/// Pure implementation of `str2signum` (`lib/sig2str.c`).
/// Returns the signal number on match, or `-1` on invalid input.
pub fn str2signum_bytes(
    signame: &[u8],
    table: &[(i32, &[u8])],
    signum_bound: i32,
    rtmin: i32,
    rtmax: i32,
) -> i32 {
    if let Some(&first) = signame.first() {
        if first.is_ascii_digit() {
            if let Some(n) = c_strtol_exhaustive(signame) {
                if n >= 0 && n <= signum_bound as i64 {
                    return n as i32;
                }
            }
            return -1;
        }
    } else {
        return -1;
    }

    for &(num, name) in table {
        if name == signame {
            return num;
        }
    }

    if rtmin > 0 && signame.starts_with(b"RTMIN") {
        if let Some(n) = c_strtol_exhaustive(&signame[5..]) {
            if n >= 0 && n <= (rtmax - rtmin) as i64 {
                return rtmin + (n as i32);
            }
        }
    } else if rtmax > 0 && signame.starts_with(b"RTMAX") {
        if let Some(n) = c_strtol_exhaustive(&signame[5..]) {
            if n >= (rtmin - rtmax) as i64 && n <= 0 {
                return rtmax + (n as i32);
            }
        }
    }

    -1
}

fn write_decimal_u32(out: &mut [u8], mut pos: usize, mut val: u32) -> usize {
    let mut digits = [0u8; 10];
    let mut dlen = 0usize;
    if val == 0 {
        digits[0] = b'0';
        dlen = 1;
    } else {
        while val > 0 {
            digits[dlen] = b'0' + (val % 10) as u8;
            val /= 10;
            dlen += 1;
        }
    }
    while dlen > 0 {
        dlen -= 1;
        out[pos] = digits[dlen];
        pos += 1;
    }
    pos
}

/// Pure implementation of `sig2str` (`lib/sig2str.c`), writing a NUL-terminated ASCII name into `out` (`len >= 16`).
/// Returns `true` (`0` in C) on success, `false` (`-1` in C) when `signum` is unrecognized.
pub fn sig2str_bytes(
    signum: i32,
    table: &[(i32, &[u8])],
    rtmin: i32,
    rtmax: i32,
    out: &mut [u8; 16],
) -> bool {
    for &(num, name) in table {
        if num == signum {
            let n = name.len().min(15);
            out[..n].copy_from_slice(&name[..n]);
            out[n] = 0;
            return true;
        }
    }

    if !(rtmin <= signum && signum <= rtmax) {
        return false;
    }

    let base = if signum <= rtmin + (rtmax - rtmin) / 2 {
        out[..5].copy_from_slice(b"RTMIN");
        rtmin
    } else {
        out[..5].copy_from_slice(b"RTMAX");
        rtmax
    };

    let delta = signum - base;
    let mut pos = 5usize;
    if delta != 0 {
        if delta > 0 {
            out[pos] = b'+';
            pos += 1;
            pos = write_decimal_u32(out, pos, delta as u32);
        } else {
            out[pos] = b'-';
            pos += 1;
            pos = write_decimal_u32(out, pos, delta.unsigned_abs());
        }
    }
    out[pos] = 0;
    true
}

/// Lookup a static C-string signal description from an explicit `(signum, CStr)` table (`lib/sigdescr_np.c`).
#[inline]
pub fn sigdescr_lookup<'a>(sig: i32, table: &'a [(i32, &'static CStr)]) -> Option<&'static CStr> {
    for &(num, desc) in table {
        if num == sig {
            return Some(desc);
        }
    }
    None
}
