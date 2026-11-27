#![forbid(unsafe_code)]
//! Pure C-locale ASCII character classification and case conversion (`lib/c-ctype.h`).

#[inline]
pub const fn c_isalnum(c: i32) -> bool {
    matches!(c, 48..=57 | 65..=90 | 97..=122)
}

#[inline]
pub const fn c_isalpha(c: i32) -> bool {
    matches!(c, 65..=90 | 97..=122)
}

#[inline]
pub const fn c_isascii(c: i32) -> bool {
    matches!(c, 0..=127)
}

#[inline]
pub const fn c_isblank(c: i32) -> bool {
    c == 32 || c == 9
}

#[inline]
pub const fn c_iscntrl(c: i32) -> bool {
    matches!(c, 0..=31 | 127)
}

#[inline]
pub const fn c_isdigit(c: i32) -> bool {
    matches!(c, 48..=57)
}

#[inline]
pub const fn c_isgraph(c: i32) -> bool {
    matches!(c, 33..=126)
}

#[inline]
pub const fn c_islower(c: i32) -> bool {
    matches!(c, 97..=122)
}

#[inline]
pub const fn c_isprint(c: i32) -> bool {
    matches!(c, 32..=126)
}

#[inline]
pub const fn c_ispunct(c: i32) -> bool {
    matches!(c, 33..=47 | 58..=64 | 91..=96 | 123..=126)
}

#[inline]
pub const fn c_isspace(c: i32) -> bool {
    matches!(c, 32 | 9 | 10 | 11 | 12 | 13)
}

#[inline]
pub const fn c_isupper(c: i32) -> bool {
    matches!(c, 65..=90)
}

#[inline]
pub const fn c_isxdigit(c: i32) -> bool {
    matches!(c, 48..=57 | 65..=70 | 97..=102)
}

#[inline]
pub const fn c_tolower(c: i32) -> i32 {
    if c >= 65 && c <= 90 {
        c - 65 + 97
    } else {
        c
    }
}

#[inline]
pub const fn c_toupper(c: i32) -> i32 {
    if c >= 97 && c <= 122 {
        c - 97 + 65
    } else {
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_ctype_ascii_and_non_ascii_boundaries() {
        assert!(c_isascii(0));
        assert!(c_isascii(127));
        assert!(!c_isascii(-1));
        assert!(!c_isascii(128));
        assert!(c_isalnum(b'0' as i32));
        assert!(c_isalpha(b'Z' as i32));
        assert!(!c_isalpha(0xe9));
        assert_eq!(c_tolower(b'A' as i32), b'a' as i32);
        assert_eq!(c_toupper(b'z' as i32), b'Z' as i32);
        assert_eq!(c_tolower(0xc0), 0xc0);
    }
}
