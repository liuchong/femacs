// femacs: port of lib/c-strcasecmp.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating ASCII case-insensitive comparison to `crate::sys::strcase`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::sys_common::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int {
    if core::ptr::eq(s1, s2) {
        return 0;
    }
    // SAFETY: `s1` and `s2` are valid NUL-terminated C strings per `c_strcasecmp` contract.
    let (b1, b2) = unsafe { (CStr::from_ptr(s1).to_bytes(), CStr::from_ptr(s2).to_bytes()) };
    crate::sys::strcase::c_strcasecmp_bytes(b1, b2)
}
