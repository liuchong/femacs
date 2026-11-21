// femacs: port of lib/endian.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/endian.in.rs imported via common

/* lib/endian.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_ENDIAN_INLINE: () = _GL_EXTERN_INLINE;

/* The twelve definitions below are C's `be16toh` … `htole64`, whose *names* the live
   `#define be16toh rpl_endian_be16toh` block in the header substitutes, so the object
   exports `rpl_endian_*` (check-symbols: `T _rpl_endian_be16toh` …).  */
/* lib/endian.c is a gnulib `_GL_*_INLINE` wrapper: the bodies live in the header mirrored
   as lib/include/endian.in.rs and C's object exports them, so the unit re-exports them
   (PORTING.md P-81).  `super::` because the module's own item would shadow the mirror's
   and recurse.  */
mod endian_unit {
  use super::*;
  #[unsafe(export_name = "rpl_endian_be16toh")]
  pub unsafe extern "C" fn be16toh (x: uint16_t) -> uint16_t
  {
    return super::be16toh (x);
  }

  #[unsafe(export_name = "rpl_endian_be32toh")]
  pub unsafe extern "C" fn be32toh (x: uint32_t) -> uint32_t
  {
    return super::be32toh (x);
  }

  #[unsafe(export_name = "rpl_endian_be64toh")]
  pub unsafe extern "C" fn be64toh (x: uint64_t) -> uint64_t
  {
    return super::be64toh (x);
  }

  #[unsafe(export_name = "rpl_endian_htobe16")]
  pub unsafe extern "C" fn htobe16 (x: uint16_t) -> uint16_t
  {
    return super::htobe16 (x);
  }

  #[unsafe(export_name = "rpl_endian_htobe32")]
  pub unsafe extern "C" fn htobe32 (x: uint32_t) -> uint32_t
  {
    return super::htobe32 (x);
  }

  #[unsafe(export_name = "rpl_endian_htobe64")]
  pub unsafe extern "C" fn htobe64 (x: uint64_t) -> uint64_t
  {
    return super::htobe64 (x);
  }

  #[unsafe(export_name = "rpl_endian_le16toh")]
  pub unsafe extern "C" fn le16toh (x: uint16_t) -> uint16_t
  {
    return super::le16toh (x);
  }

  #[unsafe(export_name = "rpl_endian_le32toh")]
  pub unsafe extern "C" fn le32toh (x: uint32_t) -> uint32_t
  {
    return super::le32toh (x);
  }

  #[unsafe(export_name = "rpl_endian_le64toh")]
  pub unsafe extern "C" fn le64toh (x: uint64_t) -> uint64_t
  {
    return super::le64toh (x);
  }

  #[unsafe(export_name = "rpl_endian_htole16")]
  pub unsafe extern "C" fn htole16 (x: uint16_t) -> uint16_t
  {
    return super::htole16 (x);
  }

  #[unsafe(export_name = "rpl_endian_htole32")]
  pub unsafe extern "C" fn htole32 (x: uint32_t) -> uint32_t
  {
    return super::htole32 (x);
  }

  #[unsafe(export_name = "rpl_endian_htole64")]
  pub unsafe extern "C" fn htole64 (x: uint64_t) -> uint64_t
  {
    return super::htole64 (x);
  }
}
