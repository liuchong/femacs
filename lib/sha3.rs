// femacs: port of lib/sha3.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating SHA-3 (Keccak-f[1600]) computation to `femacs_pure::sha3`.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

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
pub unsafe extern "C" fn sha3_224_init_ctx(ctx: *mut sha3_ctx) -> bool {
    // SAFETY: Caller passes a valid writable `sha3_ctx` pointer.
    unsafe { (*ctx).init_224() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_256_init_ctx(ctx: *mut sha3_ctx) -> bool {
    // SAFETY: Caller passes a valid writable `sha3_ctx` pointer.
    unsafe { (*ctx).init_256() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_384_init_ctx(ctx: *mut sha3_ctx) -> bool {
    // SAFETY: Caller passes a valid writable `sha3_ctx` pointer.
    unsafe { (*ctx).init_384() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_512_init_ctx(ctx: *mut sha3_ctx) -> bool {
    // SAFETY: Caller passes a valid writable `sha3_ctx` pointer.
    unsafe { (*ctx).init_512() }
}

#[unsafe(no_mangle)]
pub extern "C" fn sha3_free_ctx(_ctx: *mut sha3_ctx) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_read_ctx(ctx: *const sha3_ctx, resbuf: *mut c_void) -> *mut c_void {
    let mut out = [0u8; 64];
    // SAFETY: Caller passes a valid initialized `sha3_ctx` and `resbuf` of `ctx->digestlen` bytes.
    unsafe {
        let dlen = (*ctx).digestlen.min(64);
        (*ctx).read_into(&mut out[..dlen]);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), dlen);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_finish_ctx(ctx: *mut sha3_ctx, resbuf: *mut c_void) -> *mut c_void {
    let mut out = [0u8; 64];
    // SAFETY: Caller passes a valid mutable `sha3_ctx` and `resbuf` of `ctx->digestlen` bytes.
    unsafe {
        let dlen = (*ctx).digestlen.min(64);
        (*ctx).finish_into(&mut out[..dlen]);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resbuf.cast::<u8>(), dlen);
    }
    resbuf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_224_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0` and `resblock` holds 28 bytes.
    unsafe {
        let data = raw_bytes(buffer.cast::<c_void>(), len);
        let out = femacs_pure::sha3::sha3_224_digest(data);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 28);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_256_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0` and `resblock` holds 32 bytes.
    unsafe {
        let data = raw_bytes(buffer.cast::<c_void>(), len);
        let out = femacs_pure::sha3::sha3_256_digest(data);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 32);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_384_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0` and `resblock` holds 48 bytes.
    unsafe {
        let data = raw_bytes(buffer.cast::<c_void>(), len);
        let out = femacs_pure::sha3::sha3_384_digest(data);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 48);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_512_buffer(
    buffer: *const c_char,
    len: usize,
    resblock: *mut c_void,
) -> *mut c_void {
    // SAFETY: `[buffer, buffer + len)` is readable when `len > 0` and `resblock` holds 64 bytes.
    unsafe {
        let data = raw_bytes(buffer.cast::<c_void>(), len);
        let out = femacs_pure::sha3::sha3_512_digest(data);
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 64);
    }
    resblock
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_process_bytes(
    buffer: *const c_void,
    len: usize,
    ctx: *mut sha3_ctx,
) -> bool {
    // SAFETY: `[buffer, buffer + len)` is readable and `ctx` is a valid mutable context.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_bytes(data)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_process_block(
    buffer: *const c_void,
    len: usize,
    ctx: *mut sha3_ctx,
) -> bool {
    // SAFETY: `[buffer, buffer + len)` is readable and `ctx` is a valid mutable context.
    unsafe {
        let data = raw_bytes(buffer, len);
        (*ctx).process_block(data)
    }
}
