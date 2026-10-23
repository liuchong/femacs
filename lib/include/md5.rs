// femacs: port of lib/md5.h @ fd7afced403 (gnulib).
//
// Header mirror (PORTING.md 0.1): the struct, the constants, the `__md5_*` name
// aliases and the declarations C spells `extern`.  `tools/extract-abi.py --check
// lib/md5.h lib/include/md5.rs` compares it to the C header.
//
// C 24-26's `#if !_GL_CONFIG_H_INCLUDED / #error … / #endif` is the generated
// config header's marker; `_GL_CONFIG_H_INCLUDED` comes from src/config.rs (the
// timespec.rs precedent).
//
// C 31-50's `#if HAVE_OPENSSL_MD5` arm is dead on the frozen target (the macro is not
// in the configuration at all, so `#if HAVE_OPENSSL_MD5` is 0) and is kept as text
// under the feature that names the non-frozen side (P-51).  Its OpenSSL headers and
// the `gl_openssl.h` body have no mirror; the arm needs a configured OpenSSL, which
// this target does not have, so the ledger records it as unported text.
//
// C 55-78's `__GNUC_PREREQ`/`__THROW` are C++-and-attribute spellings; the port keeps
// the names C's text uses (the `_GL_CMP`/`__THROW` shape).  C 80-88's `#ifndef _LIBC`
// renames are *live* (`_LIBC` is undefined): every `__md5_*` token in the header's
// declarations is the name `md5_*`, which is what the declarations below spell, and
// the macro rows keep C's spelling for the ledger (`docs/ports/include-md5.md`).

include!("../src/include/config.rs");
include!("gnulib_config.rs");
include!("stdint.in.rs");
include!("stdio.rs");

/* C 20-21: `#ifndef _MD5_H` / `#define _MD5_H 1`; an include guard has no Rust
   counterpart (PORTING.md 0.1), so it is the constant the ledger compares.  */
pub const _MD5_H: c_int = 1;

/* C 24-26's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.  */
const _: () = core::assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

/* C 32-34 (`#ifndef OPENSSL_API_COMPAT / #define OPENSSL_API_COMPAT 0x10101000L`) and
   C 95's `#define GL_OPENSSL_NAME 5` are inside the dead OpenSSL arm (C 31-50, C 94-96);
   the values are kept under its feature.  C's `L` suffix is spelling, the value is the
   same (docs/ports/macro-body-notes.tsv).  */
#[cfg (feature = "openssl_md5")]
pub const OPENSSL_API_COMPAT: c_long = 0x10101000;
#[cfg (feature = "openssl_md5")]
pub const GL_OPENSSL_NAME: c_int = 5;

/* C 52-53.  */
pub const MD5_DIGEST_SIZE: c_int = 16;
pub const MD5_BLOCK_SIZE: c_int = 64;

/* C 55-66: the compiler-version query the `__THROW` condition below uses; C's body is
   `((maj) < __GNUC__ + ((min) <= __GNUC_MINOR__))` for GCC and `0` otherwise, and the
   frozen target's clang takes the `**0**` arm (C 64).  Nothing in the port evaluates
   it: the conditions it decides are C++/attribute spellings, so the macro is kept for
   the ledger (the `_GL_CMP` shape).  */
macro_rules! __GNUC_PREREQ { ($maj:expr, $min:expr) => { 0 } }

/* C 68-78: `__THROW` is empty for a C build (the `noexcept (true)`/`throw ()` arms need
   C++); the port keeps the name as an empty-value marker like `_GL_EXTERN_INLINE`.  */
pub const __THROW: () = ();

/* C 80-88: `#ifndef _LIBC # define __md5_buffer md5_buffer … #endif`.  `_LIBC` is
   undefined, so these are live in C and every `__md5_*` token below means the name
   without the underscores; the declarations use that name and the macro rows keep C's
   spelling (`docs/ports/include-md5.md`).  */
macro_rules! __md5_buffer { () => { md5_buffer } }
macro_rules! __md5_finish_ctx { () => { md5_finish_ctx } }
macro_rules! __md5_init_ctx { () => { md5_init_ctx } }
macro_rules! __md5_process_block { () => { md5_process_block } }
macro_rules! __md5_process_bytes { () => { md5_process_bytes } }
macro_rules! __md5_read_ctx { () => { md5_read_ctx } }
macro_rules! __md5_stream { () => { md5_stream } }

/* C 98-109: the state structure (`# else` arm of `# if HAVE_OPENSSL_MD5`; the OpenSSL
   arm's `struct md5_ctx` is <openssl/md5.h>'s and has no mirror).  C's `≥ 0, ≤ 128`
   comment on `buflen` and `128 bytes; the first buflen bytes are in use` on `buffer`
   are kept.  */
#[repr(C)]
pub struct md5_ctx
{
  pub A: uint32_t,
  pub B: uint32_t,
  pub C: uint32_t,
  pub D: uint32_t,

  pub total: [uint32_t; 2],
  pub buflen: uint32_t,     /* ≥ 0, ≤ 128 */
  pub buffer: [uint32_t; 32], /* 128 bytes; the first buflen bytes are in use */
}

/* C 116-156: the six functions md5.c defines and md5-stream.c's md5_stream.  The
   definitions live in md5.rs (md5_*) and md5-stream.rs (md5_stream); C's `__md5_*`
   spelling is the `#ifndef _LIBC` alias above, so the declarations spell the name the
   frozen configuration gives them.  They are one glob-imported module so that the
   defining unit's own `fn` shadows the declaration (PORTING.md 0.1-10, the
   lisp.rs `other_units` shape) - the include of this mirror by md5.rs would otherwise
   be an E0428 "defined multiple times".  */
pub mod md5_decls {
  use super::*;

  unsafe extern "C" {
    /* C 118: `extern void __md5_init_ctx (struct md5_ctx *ctx) __THROW;`  */
    pub fn md5_init_ctx (ctx: *mut md5_ctx);

    /* C 124-125.  */
    pub fn md5_process_block (buffer: *const c_void, len: usize, ctx: *mut md5_ctx);

    /* C 131-132.  */
    pub fn md5_process_bytes (buffer: *const c_void, len: usize, ctx: *mut md5_ctx);

    /* C 138-140.  */
    pub fn md5_finish_ctx (ctx: *mut md5_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 146-148.  */
    pub fn md5_read_ctx (ctx: *const md5_ctx, resbuf: *mut c_void) -> *mut c_void;

    /* C 155-156.  */
    pub fn md5_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void) -> *mut c_void;

    /* C 166-167: md5-stream.c's definition.  */
    pub fn md5_stream (stream: *mut FILE, resblock: *mut c_void) -> c_int;
  }
}
use md5_decls::*;
