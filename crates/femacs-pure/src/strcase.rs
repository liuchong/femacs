#![forbid(unsafe_code)]
//! Pure Safe Rust C-locale ASCII case-insensitive byte comparisons (`lib/c-strcasecmp.c`, `lib/c-strncasecmp.c`).

use crate::c_ctype::c_tolower;

/// Compare two NUL-terminated or slice-bounded byte sequences ASCII case-insensitively.
#[inline]
pub fn c_strcasecmp_bytes(s1: &[u8], s2: &[u8]) -> i32 {
    let mut i = 0usize;
    loop {
        let b1 = s1.get(i).copied().unwrap_or(0);
        let b2 = s2.get(i).copied().unwrap_or(0);
        let c1 = c_tolower(i32::from(b1)) as u8;
        let c2 = c_tolower(i32::from(b2)) as u8;
        if c1 == 0 || c1 != c2 {
            return i32::from(c1) - i32::from(c2);
        }
        i += 1;
    }
}

/// Compare at most `n` bytes of two NUL-terminated or slice-bounded byte sequences ASCII case-insensitively.
#[inline]
pub fn c_strncasecmp_bytes(s1: &[u8], s2: &[u8], mut n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut i = 0usize;
    loop {
        let b1 = s1.get(i).copied().unwrap_or(0);
        let b2 = s2.get(i).copied().unwrap_or(0);
        let c1 = c_tolower(i32::from(b1)) as u8;
        let c2 = c_tolower(i32::from(b2)) as u8;
        n -= 1;
        if n == 0 || c1 == 0 || c1 != c2 {
            return i32::from(c1) - i32::from(c2);
        }
        i += 1;
    }
}
