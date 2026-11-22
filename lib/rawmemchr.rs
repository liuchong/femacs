// femacs: port of lib/rawmemchr.c @ fd7afced403.
#![forbid(unsafe_code)]
//! Pure Safe Rust byte scan (`lib/rawmemchr.c`) backed by `femacs_pure::memmem::rawmemchr_find`.
//! No caller invokes `_rawmemchr` across C FFI, avoiding unaligned page-tail over-read UB.

use crate::common::*;

/// Safe slice counterpart to `rawmemchr`.
#[inline]
pub fn rawmemchr(s: &[u8], c_in: c_int) -> Option<usize> {
    femacs_pure::memmem::rawmemchr_find(s, c_in as u8)
}
