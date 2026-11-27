// femacs: port of lib/u64.h @ fd7afced403 (gnulib).
#![forbid(unsafe_code)]
use super::*;

pub const U64_H: c_int = 1;
const _: () = core::assert!(_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

#[cfg(feature = "no_uint64")]
pub const _GL_U64_MASK32: c_ulong = 0xffffffff;

pub type u64 = core::primitive::u64;

macro_rules! bswap_64 { ($x:expr) => { ($x).swap_bytes () } }
macro_rules! u64hilo { ($hi:expr, $lo:expr) => { ((($hi) as u64) << 32).wrapping_add (($lo) as u64) } }
macro_rules! u64init { ($hi:expr, $lo:expr) => { ((($hi) as u64) << 32).wrapping_add (($lo) as u64) } }
macro_rules! u64lo { ($x:expr) => { ($x) as u64 } }
macro_rules! u64getlo { ($x:expr) => { (($x) as u64 & (UINT32_MAX as u64)) as uint32_t } }
macro_rules! u64size { ($x:expr) => { u64lo! ($x) } }
macro_rules! u64not { ($x:expr) => { !($x) } }
macro_rules! u64lt { ($x:expr, $y:expr) => { ($x) < ($y) } }
macro_rules! u64and { ($x:expr, $y:expr) => { ($x) & ($y) } }
macro_rules! u64or { ($x:expr, $y:expr) => { ($x) | ($y) } }
macro_rules! u64xor { ($x:expr, $y:expr) => { ($x) ^ ($y) } }
macro_rules! u64plus { ($x:expr, $y:expr) => { ($x).wrapping_add ($y) } }
macro_rules! u64shl { ($x:expr, $n:expr) => { ($x) << ($n) } }
macro_rules! u64shr { ($x:expr, $n:expr) => { ($x) >> ($n) } }
macro_rules! u64bswap { ($x:expr) => { bswap_64! ($x) } }

#[inline(always)]
pub const fn u64rol(x: u64, n: c_int) -> u64 {
    ::femacs_sys::bitops::u64_rol(x, n)
}
