// femacs: port of lib/sha1.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating SHA-1 computation to `#![forbid(unsafe_code)]` `femacs_sys::sha1`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

#[cfg(feature = "openssl_sha1")]
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
pub unsafe extern "C" fn sha1_init_ctx(ctx: *mut sha1_ctx) {
    // SAFETY: Caller passes a valid writable `sha1_ctx` pointer.
    unsafe { (*ctx).init() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_read_ctx(ctx: *const sha1_ctx, resbuf: *mut c_void) -> *mut c_void {
    let mut out = [0u8; 20];
    // SAFETY: Caller passes a valid readable `sha1_ctx` and 20-byte writable `resbuf`.
    unsafe {
        (*ctx).read_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 20);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_finish_ctx(ctx: *mut sha1_ctx, resbuf: *mut c_void) -> *mut c_void {
    let mut out = [0u8; 20];
    // SAFETY: Caller passes a valid mutable `sha1_ctx` and 20-byte writable `resbuf`.
    unsafe {
        (*ctx).finish_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 20);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0`.
    let data = unsafe { raw_bytes(buffer.cast::<c_void>(), len) };
    let out = ::femacs_sys::sha1::sha1_digest(data);
    // SAFETY: `resblock` points to at least `SHA1_DIGEST_SIZE` (20) writable bytes.
    unsafe {
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 20);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_process_bytes(buffer: *const c_void, len: usize, ctx: *mut sha1_ctx) {
    // SAFETY: `[buffer, buffer + len)` is readable and `ctx` is a valid mutable context.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_bytes(data);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_process_block(buffer: *const c_void, len: usize, ctx: *mut sha1_ctx) {
    // SAFETY: `[buffer, buffer + len)` is readable (`len % 64 == 0`) and `ctx` is valid.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_block(data);
    }
}
