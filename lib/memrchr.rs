// femacs: port of lib/memrchr.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating reverse byte search to `femacs_pure::memmem::memrchr_find`.
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]

use crate::common::*;

#[unsafe(export_name = "memrchr")]
pub unsafe extern "C" fn __memrchr(s: *const c_void, c_in: c_int, n: usize) -> *mut c_void {
    if n == 0 || s.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: Caller guarantees `[s, s + n)` is readable for `n` bytes per `memrchr` contract.
    let slice = unsafe { core::slice::from_raw_parts(s.cast::<u8>(), n) };
    match femacs_pure::memmem::memrchr_find(slice, c_in as u8) {
        // SAFETY: `idx < n`, so `s.cast::<u8>().add(idx)` stays within the caller's `[s, s + n)` allocation.
        Some(idx) => unsafe { s.cast::<u8>().add(idx).cast_mut().cast::<c_void>() },
        None => core::ptr::null_mut(),
    }
}
