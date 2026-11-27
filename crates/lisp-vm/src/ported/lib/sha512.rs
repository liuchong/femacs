// femacs: port of lib/sha512.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating SHA-384/SHA-512 computation to `crate::sys::sha512`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::sys_common::*;

#[cfg(feature = "openssl_sha512")]
pub const GL_OPENSSL_INLINE: () = _GL_EXTERN_INLINE;

#[inline]
unsafe fn raw_bytes<'a>(ptr: *const c_void, len: usize) -> &'a [u8] {
    if len == 0 || ptr.is_null() {
        &[]
    } else {
        // SAFETY: Caller guarantees `[ptr, ptr + len)` is readable for the call duration.
        unsafe { core::slice::from_raw_parts(ptr.cast::<u8>(), len) }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha512_init_ctx(ctx: *mut sha512_ctx) {
    // SAFETY: Caller passes a valid writable `sha512_ctx` pointer.
    unsafe { (*ctx).init_512() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha384_init_ctx(ctx: *mut sha512_ctx) {
    // SAFETY: Caller passes a valid writable `sha512_ctx` pointer.
    unsafe { (*ctx).init_384() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha512_read_ctx(
    ctx: *const sha512_ctx,
    resbuf: *mut c_void,
) -> *mut c_void {
    let mut out = [0u8; 64];
    // SAFETY: Caller passes a valid readable `sha512_ctx` and 64-byte writable `resbuf`.
    unsafe {
        (*ctx).read_512_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 64);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha384_read_ctx(
    ctx: *const sha512_ctx,
    resbuf: *mut c_void,
) -> *mut c_void {
    let mut out = [0u8; 48];
    // SAFETY: Caller passes a valid readable `sha512_ctx` and 48-byte writable `resbuf`.
    unsafe {
        (*ctx).read_384_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 48);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha512_finish_ctx(
    ctx: *mut sha512_ctx,
    resbuf: *mut c_void,
) -> *mut c_void {
    let mut out = [0u8; 64];
    // SAFETY: Caller passes a valid mutable `sha512_ctx` and 64-byte writable `resbuf`.
    unsafe {
        (*ctx).finish_512_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 64);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha384_finish_ctx(
    ctx: *mut sha512_ctx,
    resbuf: *mut c_void,
) -> *mut c_void {
    let mut out = [0u8; 48];
    // SAFETY: Caller passes a valid mutable `sha512_ctx` and 48-byte writable `resbuf`.
    unsafe {
        (*ctx).finish_384_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 48);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha512_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0`.
    let data = unsafe { raw_bytes(buffer.cast::<c_void>(), len) };
    let out = crate::sys::sha512::sha512_digest(data);
    // SAFETY: `resblock` points to at least 64 writable bytes.
    unsafe {
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 64);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha384_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0`.
    let data = unsafe { raw_bytes(buffer.cast::<c_void>(), len) };
    let out = crate::sys::sha512::sha384_digest(data);
    // SAFETY: `resblock` points to at least 48 writable bytes.
    unsafe {
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 48);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha512_process_bytes(
    buffer: *const c_void,
    len: usize,
    ctx: *mut sha512_ctx,
) {
    // SAFETY: `[buffer, buffer + len)` is readable and `ctx` is a valid mutable context.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_bytes(data);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha512_process_block(
    buffer: *const c_void,
    len: usize,
    ctx: *mut sha512_ctx,
) {
    // SAFETY: `[buffer, buffer + len)` is readable (`len % 128 == 0`) and `ctx` is valid.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_block(data);
    }
}
