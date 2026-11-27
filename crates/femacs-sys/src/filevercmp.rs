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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{filemode, memmem, signal_names, strcase, timespec};

    #[test]
    fn filevercmp_filemode_memmem_strcase_and_timespec_boundaries() {
        // Debian Policy §5.6.12 version ordering
        assert!(filenvercmp_slice(b".", b"..") < 0);
        assert!(filenvercmp_slice(b"..", b".hidden") < 0);
        assert!(filenvercmp_slice(b".hidden", b"a") < 0);
        assert!(filenvercmp_slice(b"1.0~beta1", b"1.0~rc1") < 0);
        assert!(filenvercmp_slice(b"1.0~rc1", b"1.0") < 0);
        assert!(filenvercmp_slice(b"1.0", b"1.0a") < 0);
        assert!(filenvercmp_slice(b"foo-1.2.tar.gz", b"foo-1.10.tar.gz") < 0);

        // POSIX/BSD strmode special bits
        assert_eq!(&filemode::strmode_bytes(0o104755)[..11], b"-rwsr-xr-x ");
        assert_eq!(&filemode::strmode_bytes(0o104644)[..11], b"-rwSr--r-- ");
        assert_eq!(&filemode::strmode_bytes(0o042750)[..11], b"drwxr-s--- ");
        assert_eq!(&filemode::strmode_bytes(0o041777)[..11], b"drwxrwxrwt ");
        assert_eq!(&filemode::strmode_bytes(0o041776)[..11], b"drwxrwxrwT ");

        // Two-Way substring search (periodic and non-periodic critical factorizations)
        assert_eq!(memmem::memmem_find(b"anything", b""), Some(0));
        assert_eq!(memmem::memmem_find(b"short", b"longer-needle"), None);
        assert_eq!(
            memmem::memmem_find(b"abababacabababab", b"abababab"),
            Some(8)
        );
        assert_eq!(
            memmem::memmem_find(b"GCATCGCAGAGAGTATACAGTACG", b"GCAGAGAG"),
            Some(5)
        );

        // C-locale ASCII case comparison
        assert_eq!(strcase::c_strcasecmp_bytes(b"FeMaCs\0ignored", b"femacs\0other"), 0);
        assert_eq!(strcase::c_strncasecmp_bytes(b"AbCd", b"abXY", 2), 0);
        assert!(strcase::c_strcasecmp_bytes(b"\xc0", b"\xe0") < 0);

        // Saturating timespec arithmetic & extremal float conversion
        assert_eq!(timespec::timespec_add(1, 600_000_000, 2, 500_000_000), (4, 100_000_000));
        assert_eq!(timespec::timespec_sub(4, 100_000_000, 1, 600_000_000), (2, 500_000_000));
        assert_eq!(timespec::timespec_add(i64::MAX, 999_999_999, 0, 1), (i64::MAX, 999_999_999));
        assert_eq!(timespec::timespec_sub(i64::MIN, 0, 0, 1), (i64::MIN, 0));
        assert_eq!(timespec::dtotimespec(f64::NAN), (i64::MIN, 0));
        assert_eq!(timespec::dtotimespec(f64::INFINITY), (i64::MAX, 999_999_999));

        // Reverse/forward byte search & signal table conversion
        assert_eq!(memmem::memrchr_find(b"ab/cd/ef", b'/'), Some(5));
        assert_eq!(memmem::memrchr_find(b"abcdef", b'/'), None);
        assert_eq!(memmem::rawmemchr_find(b"ab/cd/ef", b'/'), Some(2));

        let sig_tbl: &[(i32, &[u8])] = &[(1, b"HUP"), (2, b"INT"), (9, b"KILL"), (0, b"EXIT")];
        assert_eq!(signal_names::str2signum_bytes(b"KILL", sig_tbl, 31, 0, -1), 9);
        assert_eq!(signal_names::str2signum_bytes(b"15", sig_tbl, 31, 0, -1), 15);
        assert_eq!(signal_names::str2signum_bytes(b"99", sig_tbl, 31, 0, -1), -1);
        assert_eq!(signal_names::str2signum_bytes(b"RTMIN+3", sig_tbl, 64, 34, 64), 37);
        assert_eq!(signal_names::str2signum_bytes(b"RTMAX-2", sig_tbl, 64, 34, 64), 62);
        let mut out = [0u8; 16];
        assert!(signal_names::sig2str_bytes(9, sig_tbl, 0, -1, &mut out));
        assert_eq!(&out[..5], b"KILL\0");
        assert!(signal_names::sig2str_bytes(37, sig_tbl, 34, 64, &mut out));
        assert_eq!(&out[..8], b"RTMIN+3\0");
        assert_eq!(
            signal_names::sigdescr_lookup(9, &[(9, c"Killed")]),
            Some(c"Killed")
        );
    }
}
