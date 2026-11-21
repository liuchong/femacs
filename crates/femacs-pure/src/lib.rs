#![forbid(unsafe_code)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
//! Pure, stateless Safe Rust data tables and character/bit algorithms.

/// Adobe-Japan1 UVS data table (`src/include/macuvs.rs`).
#[path = "../../../src/include/macuvs.rs"]
pub mod macuvs;

/// Pure C-locale ASCII character classification and case conversion.
pub mod c_ctype {
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
}

/// Pure bit-level operations matching ISO C23 `<stdbit.h>` and `<byteswap.h>`.
pub mod bitops {
    #[inline]
    pub const fn bswap_16(x: u16) -> u16 {
        x.swap_bytes()
    }

    #[inline]
    pub const fn bswap_32(x: u32) -> u32 {
        x.swap_bytes()
    }

    #[inline]
    pub const fn bswap_64(x: u64) -> u64 {
        x.swap_bytes()
    }

    #[inline]
    pub const fn bit_width_u8(n: u8) -> u32 {
        u8::BITS - n.leading_zeros()
    }

    #[inline]
    pub const fn bit_width_u16(n: u16) -> u32 {
        u16::BITS - n.leading_zeros()
    }

    #[inline]
    pub const fn bit_width_u32(n: u32) -> u32 {
        u32::BITS - n.leading_zeros()
    }

    #[inline]
    pub const fn bit_width_u64(n: u64) -> u32 {
        u64::BITS - n.leading_zeros()
    }

    #[inline]
    pub const fn memreverse8u(n: u8) -> u8 {
        n.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_ctype_ascii_and_non_ascii_boundaries() {
        assert!(c_ctype::c_isascii(0));
        assert!(c_ctype::c_isascii(127));
        assert!(!c_ctype::c_isascii(-1));
        assert!(!c_ctype::c_isascii(128));
        assert!(c_ctype::c_isalnum(b'0' as i32));
        assert!(c_ctype::c_isalpha(b'Z' as i32));
        assert!(!c_ctype::c_isalpha(0xe9));
        assert_eq!(c_ctype::c_tolower(b'A' as i32), b'a' as i32);
        assert_eq!(c_ctype::c_toupper(b'z' as i32), b'Z' as i32);
        assert_eq!(c_ctype::c_tolower(0xc0), 0xc0);
    }

    #[test]
    fn bitops_match_c23_stdbit_invariants() {
        assert_eq!(bitops::bswap_16(0x1234), 0x3412);
        assert_eq!(bitops::bswap_32(0x1234_5678), 0x7856_3412);
        assert_eq!(bitops::bswap_64(0x0102_0304_0506_0708), 0x0807_0605_0403_0201);
        assert_eq!(bitops::bit_width_u8(0), 0);
        assert_eq!(bitops::bit_width_u8(1), 1);
        assert_eq!(bitops::bit_width_u32(0x8000_0000), 32);
        assert_eq!(bitops::memreverse8u(0b1011_0001), 0b1000_1101);
    }

    #[test]
    fn macuvs_table_remains_non_empty() {
        assert_eq!(macuvs::mac_uvs_table_adobe_japan1_bytes.len(), 73655);
    }
}
