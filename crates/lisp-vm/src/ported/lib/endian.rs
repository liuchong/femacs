// femacs: port of lib/endian.c @ fd7afced403.
#![forbid(unsafe_code)]
use crate::sys_common::*;

pub const _GL_ENDIAN_INLINE: () = _GL_EXTERN_INLINE;

mod endian_unit {
    use super::*;
    pub fn be16toh(x: uint16_t) -> uint16_t {
        super::be16toh(x)
    }
    pub fn be32toh(x: uint32_t) -> uint32_t {
        super::be32toh(x)
    }
    pub fn be64toh(x: uint64_t) -> uint64_t {
        super::be64toh(x)
    }
    pub fn htobe16(x: uint16_t) -> uint16_t {
        super::htobe16(x)
    }
    pub fn htobe32(x: uint32_t) -> uint32_t {
        super::htobe32(x)
    }
    pub fn htobe64(x: uint64_t) -> uint64_t {
        super::htobe64(x)
    }
    pub fn le16toh(x: uint16_t) -> uint16_t {
        super::le16toh(x)
    }
    pub fn le32toh(x: uint32_t) -> uint32_t {
        super::le32toh(x)
    }
    pub fn le64toh(x: uint64_t) -> uint64_t {
        super::le64toh(x)
    }
    pub fn htole16(x: uint16_t) -> uint16_t {
        super::htole16(x)
    }
    pub fn htole32(x: uint32_t) -> uint32_t {
        super::htole32(x)
    }
    pub fn htole64(x: uint64_t) -> uint64_t {
        super::htole64(x)
    }
}
