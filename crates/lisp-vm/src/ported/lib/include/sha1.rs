// femacs: port of lib/sha1.h @ fd7afced403 (gnulib).
// [U1-MODULE-MIRROR]
use super::*;

pub const SHA1_H: c_int = 1;
const _: () = core::assert!(_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

#[cfg(feature = "openssl_sha1")]
pub const OPENSSL_API_COMPAT: c_long = 0x10101000;
#[cfg(feature = "openssl_sha1")]
pub const GL_OPENSSL_NAME: c_int = 1;

pub const SHA1_DIGEST_SIZE: c_int = 20;

pub type sha1_ctx = crate::sys::sha1::Sha1Ctx;

pub mod sha1_decls {
    use super::*;

    unsafe extern "C" {
        pub fn sha1_init_ctx(ctx: *mut sha1_ctx);
        pub fn sha1_process_block(buffer: *const c_void, len: usize, ctx: *mut sha1_ctx);
        pub fn sha1_process_bytes(buffer: *const c_void, len: usize, ctx: *mut sha1_ctx);
        pub fn sha1_finish_ctx(ctx: *mut sha1_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha1_read_ctx(ctx: *const sha1_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha1_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn sha1_stream(stream: *mut FILE, resblock: *mut c_void) -> c_int;
    }
}
pub use self::sha1_decls::*;
