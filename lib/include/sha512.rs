// femacs: port of lib/sha512.h @ fd7afced403 (gnulib).
// [U1-MODULE-MIRROR]
use super::*;

pub const SHA512_H: c_int = 1;
const _: () = core::assert!(_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

#[cfg(feature = "openssl_sha512")]
pub const OPENSSL_API_COMPAT: c_long = 0x10101000;
#[cfg(feature = "openssl_sha512")]
pub const GL_OPENSSL_NAME: c_int = 512;

pub const SHA384_DIGEST_SIZE: c_int = 384 / 8;
pub const SHA512_DIGEST_SIZE: c_int = 512 / 8;

pub type sha512_ctx = femacs_pure::sha512::Sha512Ctx;

pub mod sha512_decls {
    use super::*;

    unsafe extern "C" {
        pub fn sha512_init_ctx(ctx: *mut sha512_ctx);
        pub fn sha384_init_ctx(ctx: *mut sha512_ctx);
        pub fn sha512_process_block(buffer: *const c_void, len: usize, ctx: *mut sha512_ctx);
        pub fn sha512_process_bytes(buffer: *const c_void, len: usize, ctx: *mut sha512_ctx);
        pub fn sha512_finish_ctx(ctx: *mut sha512_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha384_finish_ctx(ctx: *mut sha512_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha512_read_ctx(ctx: *const sha512_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha384_read_ctx(ctx: *const sha512_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha512_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn sha384_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn sha512_stream(stream: *mut FILE, resblock: *mut c_void) -> c_int;
    }
}
pub use self::sha512_decls::*;
