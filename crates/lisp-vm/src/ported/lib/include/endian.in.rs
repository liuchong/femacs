// femacs: port of lib/endian.in.h @ fd7afced403 (gnulib).
#![forbid(unsafe_code)]
use super::*;

const _: () = core::assert!(_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

pub const _GL_ENDIAN_H: c_int = 1;
pub const LITTLE_ENDIAN: c_int = 1234;
pub const BIG_ENDIAN: c_int = 4321;
pub const PDP_ENDIAN: c_int = 3412;
pub const BYTE_ORDER: c_int = LITTLE_ENDIAN;

pub const GNULIB_defined_be16toh: c_int = 1;
pub const GNULIB_defined_be32toh: c_int = 1;
pub const GNULIB_defined_be64toh: c_int = 1;
pub const GNULIB_defined_htobe16: c_int = 1;
pub const GNULIB_defined_htobe32: c_int = 1;
pub const GNULIB_defined_htobe64: c_int = 1;
pub const GNULIB_defined_le16toh: c_int = 1;
pub const GNULIB_defined_le32toh: c_int = 1;
pub const GNULIB_defined_le64toh: c_int = 1;
pub const GNULIB_defined_htole16: c_int = 1;
pub const GNULIB_defined_htole32: c_int = 1;
pub const GNULIB_defined_htole64: c_int = 1;

#[inline(always)]
pub const fn be16toh(x: uint16_t) -> uint16_t {
    u16::from_be(x)
}
#[inline(always)]
pub const fn be32toh(x: uint32_t) -> uint32_t {
    u32::from_be(x)
}
#[inline(always)]
pub const fn be64toh(x: uint64_t) -> uint64_t {
    u64::from_be(x)
}
#[inline(always)]
pub const fn htobe16(x: uint16_t) -> uint16_t {
    x.to_be()
}
#[inline(always)]
pub const fn htobe32(x: uint32_t) -> uint32_t {
    x.to_be()
}
#[inline(always)]
pub const fn htobe64(x: uint64_t) -> uint64_t {
    x.to_be()
}
#[inline(always)]
pub const fn le16toh(x: uint16_t) -> uint16_t {
    u16::from_le(x)
}
#[inline(always)]
pub const fn le32toh(x: uint32_t) -> uint32_t {
    u32::from_le(x)
}
#[inline(always)]
pub const fn le64toh(x: uint64_t) -> uint64_t {
    u64::from_le(x)
}
#[inline(always)]
pub const fn htole16(x: uint16_t) -> uint16_t {
    x.to_le()
}
#[inline(always)]
pub const fn htole32(x: uint32_t) -> uint32_t {
    x.to_le()
}
#[inline(always)]
pub const fn htole64(x: uint64_t) -> uint64_t {
    x.to_le()
}
