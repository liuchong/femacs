// femacs: port of lib/c-strncasecmp.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating bounded ASCII case-insensitive comparison to `crate::sys::strcase`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::sys_common::*;

#[inline]
unsafe fn bounded_c_bytes<'a>(ptr: *const c_char, max_len: usize) -> &'a [u8] {
    if max_len == 0 || ptr.is_null() {
        return &[];
    }
    let mut len = 0usize;
    // SAFETY: C `strncasecmp` contract permits reading up to the first NUL byte or `max_len` bytes.
    unsafe {
        while len < max_len && *ptr.add(len) != 0 {
            len += 1;
        }
        core::slice::from_raw_parts(ptr.cast::<u8>(), len)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    if core::ptr::eq(s1, s2) || n == 0 {
        return 0;
    }
    // SAFETY: `s1` and `s2` are readable up to NUL or `n` bytes per `c_strncasecmp` contract.
    let (b1, b2) = unsafe { (bounded_c_bytes(s1, n), bounded_c_bytes(s2, n)) };
    crate::sys::strcase::c_strncasecmp_bytes(b1, b2, n)
}
