// femacs: port of lib/sha256.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating SHA-224/SHA-256 computation to `femacs_sys::sha256`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

#[cfg(feature = "openssl_sha256")]
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
pub unsafe extern "C" fn sha256_init_ctx(ctx: *mut sha256_ctx) {
    // SAFETY: Caller passes a valid writable `sha256_ctx` pointer.
    unsafe { (*ctx).init_256() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha224_init_ctx(ctx: *mut sha256_ctx) {
    // SAFETY: Caller passes a valid writable `sha256_ctx` pointer.
    unsafe { (*ctx).init_224() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_read_ctx(
    ctx: *const sha256_ctx,
    resbuf: *mut c_void,
) -> *mut c_void {
    let mut out = [0u8; 32];
    // SAFETY: Caller passes a valid readable `sha256_ctx` and 32-byte writable `resbuf`.
    unsafe {
        (*ctx).read_256_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 32);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha224_read_ctx(
    ctx: *const sha256_ctx,
    resbuf: *mut c_void,
) -> *mut c_void {
    let mut out = [0u8; 28];
    // SAFETY: Caller passes a valid readable `sha256_ctx` and 28-byte writable `resbuf`.
    unsafe {
        (*ctx).read_224_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 28);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_finish_ctx(
    ctx: *mut sha256_ctx,
    resbuf: *mut c_void,
) -> *mut c_void {
    let mut out = [0u8; 32];
    // SAFETY: Caller passes a valid mutable `sha256_ctx` and 32-byte writable `resbuf`.
    unsafe {
        (*ctx).finish_256_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 32);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha224_finish_ctx(
    ctx: *mut sha256_ctx,
    resbuf: *mut c_void,
) -> *mut c_void {
    let mut out = [0u8; 28];
    // SAFETY: Caller passes a valid mutable `sha256_ctx` and 28-byte writable `resbuf`.
    unsafe {
        (*ctx).finish_224_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 28);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0`.
    let data = unsafe { raw_bytes(buffer.cast::<c_void>(), len) };
    let out = ::femacs_sys::sha256::sha256_digest(data);
    // SAFETY: `resblock` points to at least 32 writable bytes.
    unsafe {
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 32);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha224_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0`.
    let data = unsafe { raw_bytes(buffer.cast::<c_void>(), len) };
    let out = ::femacs_sys::sha256::sha224_digest(data);
    // SAFETY: `resblock` points to at least 28 writable bytes.
    unsafe {
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 28);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_process_bytes(
    buffer: *const c_void,
    len: usize,
    ctx: *mut sha256_ctx,
) {
    // SAFETY: `[buffer, buffer + len)` is readable and `ctx` is a valid mutable context.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_bytes(data);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_process_block(
    buffer: *const c_void,
    len: usize,
    ctx: *mut sha256_ctx,
) {
    // SAFETY: `[buffer, buffer + len)` is readable (`len % 64 == 0`) and `ctx` is valid.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_block(data);
    }
}
