// femacs: port of lib/filevercmp.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating version comparison to `crate::sys::filevercmp`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::sys_common::*;

#[inline]
unsafe fn version_slice<'a>(ptr: *const c_char, len: isize) -> &'a [u8] {
    if len == 0 || ptr.is_null() {
        &[]
    } else if len < 0 {
        // SAFETY: Negative length specifies a NUL-terminated C string per `filenvercmp` contract.
        unsafe { CStr::from_ptr(ptr).to_bytes() }
    } else {
        // SAFETY: Non-negative `len` specifies `[ptr, ptr + len)` readable bytes.
        unsafe { core::slice::from_raw_parts(ptr.cast::<u8>(), len as usize) }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filevercmp(s1: *const c_char, s2: *const c_char) -> c_int {
    // SAFETY: `s1` and `s2` are NUL-terminated C strings per `filevercmp` contract.
    unsafe { filenvercmp(s1, -1, s2, -1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filenvercmp(
    a: *const c_char,
    alen: isize,
    b: *const c_char,
    blen: isize,
) -> c_int {
    // SAFETY: Caller guarantees `(a, alen)` and `(b, blen)` obey `filenvercmp` pointer/length rules.
    let (sa, sb) = unsafe { (version_slice(a, alen), version_slice(b, blen)) };
    crate::sys::filevercmp::filenvercmp_slice(sa, sb)
}
