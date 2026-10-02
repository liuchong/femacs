/* Declarations of functions and data types used for SHA512 and SHA384 sum
   library functions.
   Copyright (C) 2000-2001, 2003-2006, 2008-2026 Free Software Foundation, Inc.

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

// femacs: port of lib/sha512.h @ fd7afced403 (gnulib; GNU Emacs 32.0.50).
//
// Header mirror (PORTING.md 0.1): the struct, the constants and the declarations.
// `u64` is `lib/u64.h`'s typedef, so this mirror includes the u64 mirror, which is
// also what C 27's `#include <stdint.h>` / C 24's `<u64.h>` chain supplies.  C 30-48's
// `# if HAVE_OPENSSL_SHA512` arm (which includes `gl_openssl.h` twice) is dead on the
// frozen target and kept as text under the feature naming the non-frozen side (P-51).
// The declarations are in the glob-imported `decls` module so sha512.rs's own `fn`s
// shadow them (PORTING.md 0.1-10).

include!("../src/include/config.rs");
include!("gnulib_config.rs");
include!("stdint.in.rs");
include!("stdio.rs");
include!("u64.rs");

/* C 17-18: `#ifndef SHA512_H` / `#define SHA512_H 1`.  */
pub const SHA512_H: c_int = 1;

/* C 21-23's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.  */
const _: () = assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

/* C 29-31's `#ifndef OPENSSL_API_COMPAT / #define OPENSSL_API_COMPAT 0x10101000L` and
   C 55-58's two `#define GL_OPENSSL_NAME` (384 and 512) are inside the dead OpenSSL arm.  */
#[cfg (feature = "openssl_sha512")]
pub const OPENSSL_API_COMPAT: c_long = 0x10101000;
#[cfg (feature = "openssl_sha512")]
pub const GL_OPENSSL_NAME: c_int = 512;

/* C 51-52's two enum constants (`384 / 8`, `512 / 8`).  */
pub const SHA384_DIGEST_SIZE: c_int = 384 / 8;
pub const SHA512_DIGEST_SIZE: c_int = 512 / 8;

/* C 60-71: the state structure (the `# else` arm of `# if HAVE_OPENSSL_SHA512`).  */
#[repr(C)]
pub struct sha512_ctx
{
  pub state: [u64; 8],

  pub total: [u64; 2],
  pub buflen: usize,  /* ≥ 0, ≤ 256 */
  pub buffer: [u64; 32], /* 256 bytes; the first buflen bytes are in use */
}

/* C 73-127: the twelve functions sha512.c defines, and C 140's sha512_stream, which
   sha512-stream.c defines.  */
pub mod decls {
  use super::*;

  unsafe extern "C" {
    /* C 74-75.  */
    pub fn sha512_init_ctx (ctx: *mut sha512_ctx);
    pub fn sha384_init_ctx (ctx: *mut sha512_ctx);

    /* C 81-82.  */
    pub fn sha512_process_block (buffer: *const c_void, len: usize, ctx: *mut sha512_ctx);

    /* C 88-89.  */
    pub fn sha512_process_bytes (buffer: *const c_void, len: usize, ctx: *mut sha512_ctx);

    /* C 95-98.  */
    pub fn sha512_finish_ctx (ctx: *mut sha512_ctx, resbuf: *mut c_void) -> *mut c_void;
    pub fn sha384_finish_ctx (ctx: *mut sha512_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 107-110.  */
    pub fn sha512_read_ctx (ctx: *const sha512_ctx, resbuf: *mut c_void) -> *mut c_void;
    pub fn sha384_read_ctx (ctx: *const sha512_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 117-120.  */
    pub fn sha512_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
      -> *mut c_void;
    pub fn sha384_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
      -> *mut c_void;

    /* C 140: sha512-stream.c's definition.  */
    pub fn sha512_stream (stream: *mut FILE, resblock: *mut c_void) -> c_int;
  }
}
use decls::*;
