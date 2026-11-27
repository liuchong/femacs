#![forbid(unsafe_code)]
//! Pure bit-level operations matching ISO C23 `<stdbit.h>`, `<byteswap.h>`, and `u64.h`.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitops_match_c23_stdbit_invariants() {
        assert_eq!(bswap_16(0x1234), 0x3412);
        assert_eq!(bswap_32(0x1234_5678), 0x7856_3412);
        assert_eq!(bswap_64(0x0102_0304_0506_0708), 0x0807_0605_0403_0201);
        assert_eq!(bit_width_u8(0), 0);
        assert_eq!(bit_width_u8(1), 1);
        assert_eq!(bit_width_u32(0x8000_0000), 32);
        assert_eq!(memreverse8u(0b1011_0001), 0b1000_1101);
        assert_eq!(u64_rol(1, 5), 32);
    }
}
