/* Declarations of functions and data types used for SHA3 and SHAKE
   sum library functions.
   Copyright (C) 2022-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

// femacs: port of lib/sha3.h @ fd7afced403 (gnulib).
//
// Header mirror (PORTING.md 0.1).  C 26's `#include "u64.h"` is the u64 mirror;
// C 30-32's `<stddef.h>`/`<stdio.h>`/`<stdint.h>` are the stdio/stdint mirrors.
// C 39-41's `# if HAVE_OPENSSL_SHA3` arm (which includes <openssl/evp.h>) is dead on the
// frozen target: `HAVE_OPENSSL_SHA3` is not in the configuration, so the struct's live
// fields are C 57-61's and the `evp_ctx` field C 55 is kept as text under the feature
// naming the non-frozen side (P-51); the EVP API has no mirror, which the ledger records.
// The declarations live in the glob-imported `decls` module so sha3.rs's own `fn`s
// shadow them (PORTING.md 0.1-10).

include!("../src/include/config.rs");
include!("gnulib_config.rs");
include!("stdint.in.rs");
include!("stdio.rs");
include!("u64.rs");

/* C 20-21: `#ifndef SHA3_H` / `# define SHA3_H 1`.  */
pub const SHA3_H: c_int = 1;

/* C 43-48's eight enum constants (the sizes are C's `n / 8` expressions).  */
pub const SHA3_224_DIGEST_SIZE: c_int = 224 / 8;
pub const SHA3_256_DIGEST_SIZE: c_int = 256 / 8;
pub const SHA3_384_DIGEST_SIZE: c_int = 384 / 8;
pub const SHA3_512_DIGEST_SIZE: c_int = 512 / 8;
pub const SHA3_224_BLOCK_SIZE: c_int = 1152 / 8;
pub const SHA3_256_BLOCK_SIZE: c_int = 1088 / 8;
pub const SHA3_384_BLOCK_SIZE: c_int = 832 / 8;
pub const SHA3_512_BLOCK_SIZE: c_int = 576 / 8;

/* C 51-63: the state structure, both arms (C 53-55's `evp_ctx` is the OpenSSL arm's
   field, kept under its feature).  */
#[repr(C)]
pub struct sha3_ctx
{
  /* C 54-55: `EVP_MD_CTX is an incomplete type…`; the OpenSSL arm's pointer.  */
  #[cfg (feature = "openssl_sha3")]
  pub evp_ctx: *mut c_void,

  pub state: [u64; 25],
  pub buffer: [u8; 144], /* Up to BLOCKLEN in use.  */
  pub buflen: usize,       /* ≥ 0, ≤ BLOCKLEN  */
  pub digestlen: usize,    /* One of SHA3_{224,256,384,512}_DIGEST_SIZE.  */
  pub blocklen: usize,     /* One of SHA3_{224,256,384,512}_BLOCK_SIZE.  */
}

/* C 65-125: the declarations.  */
pub mod decls {
  use super::*;

  unsafe extern "C" {
    /* C 66-69.  */
    pub fn sha3_224_init_ctx (ctx: *mut sha3_ctx) -> bool;
    pub fn sha3_256_init_ctx (ctx: *mut sha3_ctx) -> bool;
    pub fn sha3_384_init_ctx (ctx: *mut sha3_ctx) -> bool;
    pub fn sha3_512_init_ctx (ctx: *mut sha3_ctx) -> bool;

    /* C 72.  */
    pub fn sha3_free_ctx (ctx: *mut sha3_ctx);

    /* C 79-80.  */
    pub fn sha3_process_block (buffer: *const c_void, len: usize, ctx: *mut sha3_ctx) -> bool;

    /* C 87-88.  */
    pub fn sha3_process_bytes (buffer: *const c_void, len: usize, ctx: *mut sha3_ctx) -> bool;

    /* C 94-95.  */
    pub fn sha3_finish_ctx (ctx: *mut sha3_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 101-102.  */
    pub fn sha3_read_ctx (ctx: *const sha3_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 109-116.  */
    pub fn sha3_224_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
      -> *mut c_void;
    pub fn sha3_256_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
      -> *mut c_void;
    pub fn sha3_384_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
      -> *mut c_void;
    pub fn sha3_512_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
      -> *mut c_void;

    /* C 121-124: sha3-stream.c's definitions.  */
    pub fn sha3_224_stream (stream: *mut FILE, resblock: *mut c_void) -> c_int;
    pub fn sha3_256_stream (stream: *mut FILE, resblock: *mut c_void) -> c_int;
    pub fn sha3_384_stream (stream: *mut FILE, resblock: *mut c_void) -> c_int;
    pub fn sha3_512_stream (stream: *mut FILE, resblock: *mut c_void) -> c_int;
  }
}
use decls::*;
