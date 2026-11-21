// femacs: port of lib/endian.c @ fd7afced403.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

pub const _GL_ENDIAN_INLINE: () = _GL_EXTERN_INLINE;

mod endian_unit {
    use super::*;
    #[unsafe(export_name = "rpl_endian_be16toh")]
    pub extern "C" fn be16toh(x: uint16_t) -> uint16_t {
        super::be16toh(x)
    }
    #[unsafe(export_name = "rpl_endian_be32toh")]
    pub extern "C" fn be32toh(x: uint32_t) -> uint32_t {
        super::be32toh(x)
    }
    #[unsafe(export_name = "rpl_endian_be64toh")]
    pub extern "C" fn be64toh(x: uint64_t) -> uint64_t {
        super::be64toh(x)
    }
    #[unsafe(export_name = "rpl_endian_htobe16")]
    pub extern "C" fn htobe16(x: uint16_t) -> uint16_t {
        super::htobe16(x)
    }
    #[unsafe(export_name = "rpl_endian_htobe32")]
    pub extern "C" fn htobe32(x: uint32_t) -> uint32_t {
        super::htobe32(x)
    }
    #[unsafe(export_name = "rpl_endian_htobe64")]
    pub extern "C" fn htobe64(x: uint64_t) -> uint64_t {
        super::htobe64(x)
    }
    #[unsafe(export_name = "rpl_endian_le16toh")]
    pub extern "C" fn le16toh(x: uint16_t) -> uint16_t {
        super::le16toh(x)
    }
    #[unsafe(export_name = "rpl_endian_le32toh")]
    pub extern "C" fn le32toh(x: uint32_t) -> uint32_t {
        super::le32toh(x)
    }
    #[unsafe(export_name = "rpl_endian_le64toh")]
    pub extern "C" fn le64toh(x: uint64_t) -> uint64_t {
        super::le64toh(x)
    }
    #[unsafe(export_name = "rpl_endian_htole16")]
    pub extern "C" fn htole16(x: uint16_t) -> uint16_t {
        super::htole16(x)
    }
    #[unsafe(export_name = "rpl_endian_htole32")]
    pub extern "C" fn htole32(x: uint32_t) -> uint32_t {
        super::htole32(x)
    }
    #[unsafe(export_name = "rpl_endian_htole64")]
    pub extern "C" fn htole64(x: uint64_t) -> uint64_t {
        super::htole64(x)
    }
}
