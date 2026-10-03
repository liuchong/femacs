// femacs: port of lib/sha1.h @ fd7afced403 (gnulib).
//
// Header mirror (PORTING.md 0.1): the struct, the constants and the declarations.
// C 23-25's `#if !_GL_CONFIG_H_INCLUDED / #error …` is the generated config
// header's marker (the timespec.rs shape), C 27-28's `<stdio.h>`/`<stdint.h>` are
// the stdio/stdint mirrors, and C 30-49's `#if HAVE_OPENSSL_SHA1` arm is dead on
// the frozen target (the macro is not in the configuration) and is kept as text
// under the feature naming the non-frozen side (P-51); its OpenSSL headers have no
// mirror, which the ledger records.
//
// The declarations live in the glob-imported `decls` module so sha1.rs's own `fn`s
// shadow them (PORTING.md 0.1-10, the lisp.rs `other_units` shape) instead of
// colliding with them (E0428).

include!("../src/include/config.rs");
include!("gnulib_config.rs");
include!("stdint.in.rs");
include!("stdio.rs");

/* C 19-20: `#ifndef SHA1_H` / `#define SHA1_H 1` - the include guard has no Rust
   counterpart (PORTING.md 0.1).  */
pub const SHA1_H: c_int = 1;

/* C 23-25's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.  */
const _: () = assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

/* C 31-33's `#ifndef OPENSSL_API_COMPAT / #define OPENSSL_API_COMPAT 0x10101000L` and
   C 58's `#define GL_OPENSSL_NAME 1` are inside the dead OpenSSL arm; the values are
   kept under its feature (C's `L` suffix is spelling - docs/ports/macro-body-notes.tsv).  */
#[cfg (feature = "openssl_sha1")]
pub const OPENSSL_API_COMPAT: c_long = 0x10101000;
#[cfg (feature = "openssl_sha1")]
pub const GL_OPENSSL_NAME: c_int = 1;

/* C 55.  */
pub const SHA1_DIGEST_SIZE: c_int = 20;

/* C 74-86: the state structure (the `# else` arm of `# if HAVE_OPENSSL_SHA1`; the OpenSSL
   arm's `SHA_CTX` is <openssl/sha.h>'s and has no mirror).  */
#[repr(C)]
pub struct sha1_ctx
{
  pub A: uint32_t,
  pub B: uint32_t,
  pub C: uint32_t,
  pub D: uint32_t,
  pub E: uint32_t,

  pub total: [uint32_t; 2],
  pub buflen: uint32_t,     /* ≥ 0, ≤ 128 */
  pub buffer: [uint32_t; 32], /* 128 bytes; the first buflen bytes are in use */
}

/* C 90-112: the six functions sha1.c defines, and C 122's sha1_stream, which
   sha1-stream.c defines.  */
pub mod decls {
  use super::*;

  unsafe extern "C" {
    /* C 90.  */
    pub fn sha1_init_ctx (ctx: *mut sha1_ctx);

    /* C 96-97.  */
    pub fn sha1_process_block (buffer: *const c_void, len: usize, ctx: *mut sha1_ctx);

    /* C 103-104.  */
    pub fn sha1_process_bytes (buffer: *const c_void, len: usize, ctx: *mut sha1_ctx);

    /* C 110-111.  */
    pub fn sha1_finish_ctx (ctx: *mut sha1_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 117-118.  */
    pub fn sha1_read_ctx (ctx: *const sha1_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 125-126.  */
    pub fn sha1_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;

    /* C 122: sha1-stream.c's definition.  */
    pub fn sha1_stream (stream: *mut FILE, resblock: *mut c_void) -> c_int;
  }
}
use decls::*;
