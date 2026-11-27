// femacs: port of lib/md5.h @ fd7afced403 (gnulib).
// [U1-MODULE-MIRROR]
use super::*;

pub const _MD5_H: c_int = 1;
const _: () = core::assert!(_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

#[cfg(feature = "openssl_md5")]
pub const OPENSSL_API_COMPAT: c_long = 0x10101000;
#[cfg(feature = "openssl_md5")]
pub const GL_OPENSSL_NAME: c_int = 5;

pub const MD5_DIGEST_SIZE: c_int = 16;
pub const MD5_BLOCK_SIZE: c_int = 64;

macro_rules! __GNUC_PREREQ {
    ($maj:expr, $min:expr) => {
        0
    };
}
pub const __THROW: () = ();

macro_rules! __md5_buffer { () => { md5_buffer } }
macro_rules! __md5_finish_ctx { () => { md5_finish_ctx } }
macro_rules! __md5_init_ctx { () => { md5_init_ctx } }
macro_rules! __md5_process_block { () => { md5_process_block } }
macro_rules! __md5_process_bytes { () => { md5_process_bytes } }
macro_rules! __md5_read_ctx { () => { md5_read_ctx } }
macro_rules! __md5_stream { () => { md5_stream } }

pub type md5_ctx = crate::sys::md5::Md5Ctx;

pub mod md5_decls {
    use super::*;

    unsafe extern "C" {
        pub fn md5_init_ctx(ctx: *mut md5_ctx);
        pub fn md5_process_block(buffer: *const c_void, len: usize, ctx: *mut md5_ctx);
        pub fn md5_process_bytes(buffer: *const c_void, len: usize, ctx: *mut md5_ctx);
        pub fn md5_finish_ctx(ctx: *mut md5_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn md5_read_ctx(ctx: *const md5_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn md5_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn md5_stream(stream: *mut FILE, resblock: *mut c_void) -> c_int;
    }
}
pub use self::md5_decls::*;
