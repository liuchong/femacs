#![forbid(unsafe_code)]
//! Pure Safe Rust Debian version-sort comparison (`lib/filevercmp.c`).

use crate::c_ctype::{c_isalnum, c_isalpha, c_isdigit};

const UCHAR_MAX_I32: i32 = 255;

#[inline]
fn file_prefixlen(s: &[u8]) -> usize {
    let n = s.len();
    let mut prefixlen = 0usize;
    let mut i = 0usize;
    loop {
        if i == n {
            return prefixlen;
        }
        i += 1;
        prefixlen = i;
        while i + 1 < n
            && s[i] == b'.'
            && (c_isalpha(i32::from(s[i + 1])) || s[i + 1] == b'~')
        {
            i += 2;
            while i < n && (c_isalnum(i32::from(s[i])) || s[i] == b'~') {
                i += 1;
            }
        }
    }
}

#[inline]
fn order(s: &[u8], pos: usize) -> i32 {
    if pos == s.len() {
        return -1;
    }
    let c = s[pos];
    let ci = i32::from(c);
    if c_isdigit(ci) {
        0
    } else if c_isalpha(ci) {
        ci
    } else if c == b'~' {
        -2
    } else {
        ci + UCHAR_MAX_I32 + 1
    }
}

pub fn verrevcmp(s1: &[u8], s2: &[u8]) -> i32 {
    let s1_len = s1.len();
    let s2_len = s2.len();
    let mut s1_pos = 0usize;
    let mut s2_pos = 0usize;

    while s1_pos < s1_len || s2_pos < s2_len {
        let mut first_diff = 0i32;
        while (s1_pos < s1_len && !c_isdigit(i32::from(s1[s1_pos])))
            || (s2_pos < s2_len && !c_isdigit(i32::from(s2[s2_pos])))
        {
            let s1_c = order(s1, s1_pos);
            let s2_c = order(s2, s2_pos);
            if s1_c != s2_c {
                return s1_c - s2_c;
            }
            s1_pos += 1;
            s2_pos += 1;
        }
        while s1_pos < s1_len && s1[s1_pos] == b'0' {
            s1_pos += 1;
        }
        while s2_pos < s2_len && s2[s2_pos] == b'0' {
            s2_pos += 1;
        }
        while s1_pos < s1_len
            && s2_pos < s2_len
            && c_isdigit(i32::from(s1[s1_pos]))
            && c_isdigit(i32::from(s2[s2_pos]))
        {
            if first_diff == 0 {
                first_diff = i32::from(s1[s1_pos]) - i32::from(s2[s2_pos]);
            }
            s1_pos += 1;
            s2_pos += 1;
        }
        if s1_pos < s1_len && c_isdigit(i32::from(s1[s1_pos])) {
            return 1;
        }
        if s2_pos < s2_len && c_isdigit(i32::from(s2[s2_pos])) {
            return -1;
        }
        if first_diff != 0 {
            return first_diff;
        }
    }
    0
}

pub fn filenvercmp_slice(a: &[u8], b: &[u8]) -> i32 {
    if a.is_empty() {
        return -i32::from(!b.is_empty());
    }
    if b.is_empty() {
        return 1;
    }

    if a[0] == b'.' {
        if b[0] != b'.' {
            return -1;
        }
        let adot = a.len() == 1;
        let bdot = b.len() == 1;
        if adot {
            return -i32::from(!bdot);
        }
        if bdot {
            return 1;
        }
        let adotdot = a == b"..";
        let bdotdot = b == b"..";
        if adotdot {
            return -i32::from(!bdotdot);
        }
        if bdotdot {
            return 1;
        }
    } else if b[0] == b'.' {
        return 1;
    }

    let aprefixlen = file_prefixlen(a);
    let bprefixlen = file_prefixlen(b);
    let one_pass_only = aprefixlen == a.len() && bprefixlen == b.len();

    let result = verrevcmp(&a[..aprefixlen], &b[..bprefixlen]);
    if result != 0 || one_pass_only {
        result
    } else {
        verrevcmp(a, b)
    }
}
