// femacs: port of lib/sha3.h @ fd7afced403 (gnulib).
// [U1-MODULE-MIRROR]
use super::*;

pub const SHA3_H: c_int = 1;

pub const SHA3_224_DIGEST_SIZE: c_int = 224 / 8;
pub const SHA3_256_DIGEST_SIZE: c_int = 256 / 8;
pub const SHA3_384_DIGEST_SIZE: c_int = 384 / 8;
pub const SHA3_512_DIGEST_SIZE: c_int = 512 / 8;
pub const SHA3_224_BLOCK_SIZE: c_int = 1152 / 8;
pub const SHA3_256_BLOCK_SIZE: c_int = 1088 / 8;
pub const SHA3_384_BLOCK_SIZE: c_int = 832 / 8;
pub const SHA3_512_BLOCK_SIZE: c_int = 576 / 8;

#[cfg(not(feature = "openssl_sha3"))]
pub type sha3_ctx = ::femacs_sys::sha3::Sha3Ctx;

#[cfg(feature = "openssl_sha3")]
#[repr(C)]
pub struct sha3_ctx {
    pub evp_ctx: *mut c_void,
    pub state: [u64; 25],
    pub buffer: [u8; 144],
    pub buflen: usize,
    pub digestlen: usize,
    pub blocklen: usize,
}

pub mod sha3_decls {
    use super::*;

    unsafe extern "C" {
        pub fn sha3_224_init_ctx(ctx: *mut sha3_ctx) -> bool;
        pub fn sha3_256_init_ctx(ctx: *mut sha3_ctx) -> bool;
        pub fn sha3_384_init_ctx(ctx: *mut sha3_ctx) -> bool;
        pub fn sha3_512_init_ctx(ctx: *mut sha3_ctx) -> bool;
        pub fn sha3_free_ctx(ctx: *mut sha3_ctx);
        pub fn sha3_process_block(buffer: *const c_void, len: usize, ctx: *mut sha3_ctx) -> bool;
        pub fn sha3_process_bytes(buffer: *const c_void, len: usize, ctx: *mut sha3_ctx) -> bool;
        pub fn sha3_finish_ctx(ctx: *mut sha3_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha3_read_ctx(ctx: *const sha3_ctx, resbuf: *mut c_void) -> *mut c_void;
        pub fn sha3_224_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn sha3_256_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn sha3_384_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn sha3_512_buffer(buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;
        pub fn sha3_224_stream(stream: *mut FILE, resblock: *mut c_void) -> c_int;
        pub fn sha3_256_stream(stream: *mut FILE, resblock: *mut c_void) -> c_int;
        pub fn sha3_384_stream(stream: *mut FILE, resblock: *mut c_void) -> c_int;
        pub fn sha3_512_stream(stream: *mut FILE, resblock: *mut c_void) -> c_int;
    }
}
pub use self::sha3_decls::*;
