// femacs: port of lib/memmem.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating Two-Way substring search to `femacs_sys::memmem`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

#[unsafe(export_name = "rpl_memmem")]
pub unsafe extern "C" fn memmem(
    haystack_start: *const c_void,
    haystack_len: usize,
    needle_start: *const c_void,
    needle_len: usize,
) -> *mut c_void {
    if needle_len == 0 {
        return haystack_start as *mut c_void;
    }
    if haystack_len < needle_len || haystack_start.is_null() || needle_start.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: `needle_len > 0` and `haystack_len >= needle_len`; caller guarantees both byte ranges are readable.
    let (haystack, needle) = unsafe {
        (
            core::slice::from_raw_parts(haystack_start.cast::<u8>(), haystack_len),
            core::slice::from_raw_parts(needle_start.cast::<u8>(), needle_len),
        )
    };
    match ::femacs_sys::memmem::memmem_find(haystack, needle) {
        // SAFETY: `offset <= haystack_len` stays within the caller's `haystack_start` object.
        Some(offset) => unsafe { haystack_start.cast::<u8>().add(offset) as *mut c_void },
        None => core::ptr::null_mut(),
    }
}
