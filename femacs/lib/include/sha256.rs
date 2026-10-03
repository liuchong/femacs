// femacs: port of lib/sha256.h @ fd7afced403 (gnulib).
//
// Header mirror (PORTING.md 0.1): the struct, the constants and the declarations.
// C 22-24's `#if !_GL_CONFIG_H_INCLUDED / #error …` is the generated config header's
// marker, C 26-27's `<stdio.h>`/`<stdint.h>` are the stdio/stdint mirrors, and C
// 57-62's `# if HAVE_OPENSSL_SHA256` arm (which includes `gl_openssl.h` twice, once
// for each digest length) is dead on the frozen target and kept as text under the
// feature naming the non-frozen side (P-51).  The declarations are in the
// glob-imported `decls` module so sha256.rs's own `fn`s shadow them (PORTING.md
// 0.1-10) instead of colliding with them.

include!("../src/include/config.rs");
include!("gnulib_config.rs");
include!("stdint.in.rs");
include!("stdio.rs");

/* C 18-19: `#ifndef SHA256_H` / `#define SHA256_H 1` - the include guard has no Rust
   counterpart (PORTING.md 0.1).  */
pub const SHA256_H: c_int = 1;

/* C 22-24's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.  */
const _: () = assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

/* C 30-32's `#ifndef OPENSSL_API_COMPAT / #define OPENSSL_API_COMPAT 0x10101000L` and
   C 58-61's two `#define GL_OPENSSL_NAME` (224 and 256) are inside the dead OpenSSL arm;
   the values are kept under its feature (C's `L` suffix is spelling -
   docs/ports/macro-body-notes.tsv).  */
#[cfg (feature = "openssl_sha256")]
pub const OPENSSL_API_COMPAT: c_long = 0x10101000;
#[cfg (feature = "openssl_sha256")]
pub const GL_OPENSSL_NAME: c_int = 256;

/* C 54-55's two enum constants (`224 / 8`, `256 / 8`).  */
pub const SHA224_DIGEST_SIZE: c_int = 224 / 8;
pub const SHA256_DIGEST_SIZE: c_int = 256 / 8;

/* C 63-71: the state structure (the `# else` arm of `# if HAVE_OPENSSL_SHA256`).  */
#[repr(C)]
pub struct sha256_ctx
{
  pub state: [uint32_t; 8],

  pub total: [uint32_t; 2],
  pub buflen: usize,       /* ≥ 0, ≤ 128 */
  pub buffer: [uint32_t; 32], /* 128 bytes; the first buflen bytes are in use */
}

/* C 73-117: the twelve functions sha256.c defines, and C 130's sha256_stream, which
   sha256-stream.c defines.  */
pub mod sha256_decls {
  use super::*;

  unsafe extern "C" {
    /* C 74-75.  */
    pub fn sha256_init_ctx (ctx: *mut sha256_ctx);
    pub fn sha224_init_ctx (ctx: *mut sha256_ctx);

    /* C 81-82.  */
    pub fn sha256_process_block (buffer: *const c_void, len: usize, ctx: *mut sha256_ctx);

    /* C 88-89.  */
    pub fn sha256_process_bytes (buffer: *const c_void, len: usize, ctx: *mut sha256_ctx);

    /* C 95-98.  */
    pub fn sha256_finish_ctx (ctx: *mut sha256_ctx, resbuf: *mut c_void) -> *mut c_void;
    pub fn sha224_finish_ctx (ctx: *mut sha256_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 104-107.  */
    pub fn sha256_read_ctx (ctx: *const sha256_ctx, resbuf: *mut c_void) -> *mut c_void;
    pub fn sha224_read_ctx (ctx: *const sha256_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 114-117.  */
    pub fn sha256_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
      -> *mut c_void;
    pub fn sha224_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
      -> *mut c_void;

    /* C 130: sha256-stream.c's definition.  */
    pub fn sha256_stream (stream: *mut FILE, resblock: *mut c_void) -> c_int;
  }
}
use sha256_decls::*;
