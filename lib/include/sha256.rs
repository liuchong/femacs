// femacs: port of lib/sha256.h @ fd7afced403 (gnulib).
// [U1-MODULE-MIRROR]
use super::*;

pub const SHA256_H: c_int = 1;
const _: () = core::assert!(_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

#[cfg(feature = "openssl_sha256")]
pub const OPENSSL_API_COMPAT: c_long = 0x10101000;
#[cfg(feature = "openssl_sha256")]
pub const GL_OPENSSL_NAME: c_int = 256;

pub const SHA224_DIGEST_SIZE: c_int = 224 / 8;
pub const SHA256_DIGEST_SIZE: c_int = 256 / 8;

pub type sha256_ctx = femacs_pure::sha256::Sha256Ctx;

pub mod sha256_decls {
    use super::*;

    unsafe extern "C" {
        pub fn sha256_init_ctx(ctx: *mut sha256_ctx);
        pub fn sha224_init_ctx(ctx: *mut sha256_ctx);
        pub fn sha256_process_block(buffer: *const c_void, len: usize, ctx: *mut sha256_ctx);
        pub fn sha256_process_bytes(buffer: *const c_void, len: usize, ctx: *mut sha256_ctx);
        pub fn sha256_finish_ctx(ctx: *mut sha256_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha224_finish_ctx(ctx: *mut sha256_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha256_read_ctx(ctx: *const sha256_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha224_read_ctx(ctx: *const sha256_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha256_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn sha224_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn sha256_stream(stream: *mut FILE, resblock: *mut c_void) -> c_int;
    }
}
pub use self::sha256_decls::*;
