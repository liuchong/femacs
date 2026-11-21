#![forbid(unsafe_code)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
//! Pure, stateless Safe Rust data tables, cryptographic digests, and string/bit/time algorithms.

/// Adobe-Japan1 UVS data table (`src/include/macuvs.rs`).
#[path = "../../../src/include/macuvs.rs"]
pub mod macuvs;

pub mod filemode;
pub mod filevercmp;
pub mod md5;
pub mod memmem;
pub mod sha1;
pub mod sha256;
pub mod sha3;
pub mod sha512;
pub mod strcase;
pub mod timespec;

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

/// Pure bit-level operations matching ISO C23 `<stdbit.h>`, `<byteswap.h>`, and `u64.h`.
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

    #[inline]
    pub const fn u64_rol(x: u64, n: i32) -> u64 {
        x.rotate_left(n as u32)
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
        assert_eq!(bitops::u64_rol(1, 5), 32);
    }

    #[test]
    fn macuvs_table_remains_non_empty() {
        assert_eq!(macuvs::mac_uvs_table_adobe_japan1_bytes.len(), 73655);
    }

    fn hex(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            use core::fmt::Write as _;
            let _ = write!(s, "{b:02x}");
        }
        s
    }

    #[test]
    fn md5_rfc1321_vectors_and_streaming_boundary_splits() {
        assert_eq!(hex(&md5::md5_digest(b"")), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(hex(&md5::md5_digest(b"abc")), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(
            hex(&md5::md5_digest(b"message digest")),
            "f96b697d7cb7938d525a2f31aaf161d0"
        );

        for len in [0usize, 1, 55, 56, 57, 63, 64, 65, 119, 120, 128, 257] {
            let payload: Vec<u8> = (0..len).map(|i| (i.wrapping_mul(37) ^ 0xa5) as u8).collect();
            let expected = md5::md5_digest(&payload);
            for step in [1usize, 7, 19, 64, 97] {
                let mut ctx = md5::Md5Ctx::new();
                for chunk in payload.chunks(step) {
                    ctx.process_bytes(chunk);
                }
                let mut got = [0u8; 16];
                ctx.finish_into(&mut got);
                assert_eq!(got, expected, "md5 split mismatch len={len} step={step}");
            }
        }
    }

    #[test]
    fn sha1_and_sha2_fips180_vectors_and_streaming_splits() {
        assert_eq!(
            hex(&sha1::sha1_digest(b"abc")),
            "a9993e364706816aba3e25717850c26c9cd0d89d"
        );
        assert_eq!(
            hex(&sha256::sha224_digest(b"abc")),
            "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7"
        );
        assert_eq!(
            hex(&sha256::sha256_digest(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            hex(&sha512::sha384_digest(b"abc")),
            "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"
        );
        assert_eq!(
            hex(&sha512::sha512_digest(b"abc")),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );

        for len in [55usize, 56, 64, 111, 112, 128, 300] {
            let payload: Vec<u8> = (0..len).map(|i| (i.wrapping_mul(13) ^ 0x5c) as u8).collect();
            let exp1 = sha1::sha1_digest(&payload);
            let exp256 = sha256::sha256_digest(&payload);
            let exp512 = sha512::sha512_digest(&payload);

            let mut c1 = sha1::Sha1Ctx::new();
            let mut c256 = sha256::Sha256Ctx::new_256();
            let mut c512 = sha512::Sha512Ctx::new_512();
            for chunk in payload.chunks(11) {
                c1.process_bytes(chunk);
                c256.process_bytes(chunk);
                c512.process_bytes(chunk);
            }
            let mut g1 = [0u8; 20];
            let mut g256 = [0u8; 32];
            let mut g512 = [0u8; 64];
            c1.finish_into(&mut g1);
            c256.finish_256_into(&mut g256);
            c512.finish_512_into(&mut g512);
            assert_eq!(g1, exp1);
            assert_eq!(g256, exp256);
            assert_eq!(g512, exp512);
        }
    }

    #[test]
    fn sha3_fips202_vectors_and_rate_boundary_splits() {
        assert_eq!(
            hex(&sha3::sha3_224_digest(b"abc")),
            "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf"
        );
        assert_eq!(
            hex(&sha3::sha3_256_digest(b"abc")),
            "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
        );
        assert_eq!(
            hex(&sha3::sha3_384_digest(b"abc")),
            "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25"
        );
        assert_eq!(
            hex(&sha3::sha3_512_digest(b"abc")),
            "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"
        );

        for len in [71usize, 72, 73, 103, 104, 135, 136, 143, 144, 289] {
            let payload: Vec<u8> = (0..len).map(|i| (i.wrapping_mul(29) ^ 0x33) as u8).collect();
            let exp = sha3::sha3_256_digest(&payload);
            let mut ctx = sha3::Sha3Ctx::new(sha3::SHA3_256_DIGEST_SIZE, sha3::SHA3_256_BLOCK_SIZE);
            for chunk in payload.chunks(9) {
                assert!(ctx.process_bytes(chunk));
            }
            let mut got = [0u8; 32];
            ctx.finish_into(&mut got);
            assert_eq!(got, exp, "sha3-256 split mismatch len={len}");
        }
    }

    #[test]
    fn filevercmp_filemode_memmem_strcase_and_timespec_boundaries() {
        // Debian Policy §5.6.12 version ordering
        assert!(filevercmp::filenvercmp_slice(b".", b"..") < 0);
        assert!(filevercmp::filenvercmp_slice(b"..", b".hidden") < 0);
        assert!(filevercmp::filenvercmp_slice(b".hidden", b"a") < 0);
        assert!(filevercmp::filenvercmp_slice(b"1.0~beta1", b"1.0~rc1") < 0);
        assert!(filevercmp::filenvercmp_slice(b"1.0~rc1", b"1.0") < 0);
        assert!(filevercmp::filenvercmp_slice(b"1.0", b"1.0a") < 0);
        assert!(filevercmp::filenvercmp_slice(b"foo-1.2.tar.gz", b"foo-1.10.tar.gz") < 0);

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
    }
}
