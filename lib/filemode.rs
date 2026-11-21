// femacs: port of lib/filemode.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating POSIX mode formatting to `femacs_pure::filemode`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

#[cfg(feature = "not_have_decl_strmode")]
pub unsafe extern "C" fn strmode(mode: mode_t, str_: *mut c_char) {
    let bytes = femacs_pure::filemode::strmode_bytes(u32::from(mode));
    // SAFETY: `strmode` contract requires `str_` to point to at least 12 writable bytes.
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), str_.cast::<u8>(), 12);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filemodestring(statp: *const stat, str_: *mut c_char) {
    // SAFETY: `statp` is a valid readable `struct stat` and `str_` points to 12 writable bytes.
    let mode = unsafe { u32::from((*statp).st_mode) };
    let mut bytes = femacs_pure::filemode::strmode_bytes(mode);

    if S_TYPEISSEM!(statp) != 0 {
        bytes[0] = b'F';
    } else if S_TYPEISMQ!(statp) != 0 {
        bytes[0] = b'Q';
    } else if S_TYPEISSHM!(statp) != 0 {
        bytes[0] = b'S';
    } else if S_TYPEISTMO!(statp) != 0 {
        bytes[0] = b'T';
    }

    // SAFETY: `str_` points to at least 12 writable bytes per `filemodestring` contract.
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), str_.cast::<u8>(), 12);
    }
}
