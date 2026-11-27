// femacs: port of lib/md5.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating MD5 computation to `#![forbid(unsafe_code)]` `femacs_sys::md5`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

#[cfg(feature = "openssl_md5")]
pub const GL_OPENSSL_INLINE: () = _GL_EXTERN_INLINE;
#[cfg(feature = "words_bigendian")]
pub const WORDS_BIGENDIAN: c_int = 1;

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
pub unsafe extern "C" fn md5_init_ctx(ctx: *mut md5_ctx) {
    // SAFETY: Caller passes a valid writable `md5_ctx` pointer per C contract.
    unsafe { (*ctx).init() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_read_ctx(ctx: *const md5_ctx, resbuf: *mut c_void) -> *mut c_void {
    let mut out = [0u8; 16];
    // SAFETY: Caller passes a valid readable `md5_ctx` and 16-byte writable `resbuf`.
    unsafe {
        (*ctx).read_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 16);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_finish_ctx(ctx: *mut md5_ctx, resbuf: *mut c_void) -> *mut c_void {
    let mut out = [0u8; 16];
    // SAFETY: Caller passes a valid mutable `md5_ctx` and 16-byte writable `resbuf`.
    unsafe {
        (*ctx).finish_into(&mut out);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), 16);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0`, and `resblock` holds 16 bytes.
    let data = unsafe { raw_bytes(buffer.cast::<c_void>(), len) };
    let out = ::femacs_sys::md5::md5_digest(data);
    // SAFETY: `resblock` points to at least `MD5_DIGEST_SIZE` (16) writable bytes.
    unsafe {
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 16);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_process_bytes(buffer: *const c_void, len: usize, ctx: *mut md5_ctx) {
    // SAFETY: `[buffer, buffer + len)` is readable and `ctx` is a valid mutable context.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_bytes(data);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_process_block(buffer: *const c_void, len: usize, ctx: *mut md5_ctx) {
    // SAFETY: `[buffer, buffer + len)` is readable (`len % 64 == 0`) and `ctx` is valid.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_block(data);
    }
}
