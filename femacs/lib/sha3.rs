/* sha3.c - Functions to calculate SHA-3 hashes as specified by FIPS-202.
   Copyright (C) 2025-2026 Free Software Foundation, Inc.

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

/* Written by Collin Funk <collin.funk1@gmail.com>, 2025.  */

/* emacs/lib/sha3.c: C 19's `<config.h>` is the generated config mirror, C 22's
   `"sha3.h"` is the header mirror (which brings `u64` and the `u64*` operations from the
   u64 mirror), C 24-25's `<stdint.h>`/`<stdlib.h>` and C 26's `<string.h>` are the
   stdint mirror plus the two declarations below, and C 28's `<byteswap.h>` is what the
   big-endian `SWAP` arm's `u64bswap` stands for (Rust's `swap_bytes`, PORTING.md 2.6).
   C 32's `#if ! HAVE_OPENSSL_SHA3` is live (the macro is not in the configuration) and
   C 442's `#endif` closes it; C 334-441's `#else` arm is the EVP implementation, whose
   OpenSSL API has no mirror (the ledger records it, and C 336-337's `<errno.h>` is only
   used there).
   C 53-61's `DEFINE_SHA3_INIT_CTX(SIZE)` and C 122-131's `DEFINE_SHA3_BUFFER(SIZE)` paste
   `sha3_##SIZE##_*` names; Rust has no token pasting, so the port's macros take the four
   generated names as arguments and C's numbers are the constants they pass (the
   ITREE_FOREACH decision, P-33) - the call sites below say which expansion is which.  */
use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/stdint.in.rs");
include!("include/sha3.rs");

/* C 26's `<string.h>`: the two entries the bodies call.  */
unsafe extern "C" {
  pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
  pub fn memset (dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

/* C 34-39's `<byteswap.h>` and the `SWAP` pair: this file's arms are the reverse of
   sha512.c's - the big-endian arm is `u64bswap (n)` and the live little-endian arm is
   C's `(n)`.  */
#[cfg (feature = "words_bigendian")]
macro_rules! SWAP { ($n:expr) => { u64bswap! ($n) } }
#[cfg (not (feature = "words_bigendian"))]
macro_rules! SWAP { ($n:expr) => { $n } }

static rc: [u64; 24] = [
  u64init! (0x00000000, 0x00000001), u64init! (0x00000000, 0x00008082),
  u64init! (0x80000000, 0x0000808A), u64init! (0x80000000, 0x80008000),
  u64init! (0x00000000, 0x0000808B), u64init! (0x00000000, 0x80000001),
  u64init! (0x80000000, 0x80008081), u64init! (0x80000000, 0x00008009),
  u64init! (0x00000000, 0x0000008A), u64init! (0x00000000, 0x00000088),
  u64init! (0x00000000, 0x80008009), u64init! (0x00000000, 0x8000000A),
  u64init! (0x00000000, 0x8000808B), u64init! (0x80000000, 0x0000008B),
  u64init! (0x80000000, 0x00008089), u64init! (0x80000000, 0x00008003),
  u64init! (0x80000000, 0x00008002), u64init! (0x80000000, 0x00000080),
  u64init! (0x00000000, 0x0000800A), u64init! (0x80000000, 0x8000000A),
  u64init! (0x80000000, 0x80008081), u64init! (0x80000000, 0x00008080),
  u64init! (0x00000000, 0x80000001), u64init! (0x80000000, 0x80008008),
];

/* C 53-61: `bool sha3_##SIZE##_init_ctx (struct sha3_ctx *ctx)`, with C's body.  */
macro_rules! DEFINE_SHA3_INIT_CTX {
  ($name:ident, $digest:expr, $block:expr) => {
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn $name (ctx: *mut sha3_ctx) -> bool
    {
      memset ((*ctx).state.as_mut_ptr () as *mut c_void, b'\0' as c_int,
              size_of::<[u64; 25]> ());
      (*ctx).buflen = 0;
      (*ctx).digestlen = $digest as usize;
      (*ctx).blocklen = $block as usize;
      return true;
    }
  };
}

DEFINE_SHA3_INIT_CTX! (sha3_224_init_ctx, SHA3_224_DIGEST_SIZE, SHA3_224_BLOCK_SIZE);
DEFINE_SHA3_INIT_CTX! (sha3_256_init_ctx, SHA3_256_DIGEST_SIZE, SHA3_256_BLOCK_SIZE);
DEFINE_SHA3_INIT_CTX! (sha3_384_init_ctx, SHA3_384_DIGEST_SIZE, SHA3_384_BLOCK_SIZE);
DEFINE_SHA3_INIT_CTX! (sha3_512_init_ctx, SHA3_512_DIGEST_SIZE, SHA3_512_BLOCK_SIZE);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_free_ctx (_ctx: *mut sha3_ctx)
{
  /* Do nothing.  */
}

/* Copy the value from V into the memory location pointed to by *CP,
   If your architecture allows unaligned access, this is equivalent to
   * (__typeof__ (v) *) cp = v  */
unsafe fn set_uint64 (cp: *mut c_char, v: u64)
{
  memcpy (cp as *mut c_void, &v as *const u64 as *const c_void, size_of::<u64> ());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_read_ctx (ctx: *const sha3_ctx, resbuf: *mut c_void)
  -> *mut c_void
{
  let mut r: *mut c_char = resbuf as *mut c_char;
  let words: usize = (*ctx).digestlen / size_of::<u64> ();
  let mut bytes: usize = (*ctx).digestlen % size_of::<u64> ();

  let mut i: c_int = 0;
  while i < words as c_int
    {
      set_uint64 (r, SWAP! ((*ctx).state[i as usize]));
      r = r.add (size_of::<u64> ());
      i += 1;
    }
  if bytes != 0
    {
      let mut word: u64 = (*ctx).state[i as usize];
      loop
        {
          *r = (u64getlo! (word) & 0xFF) as c_char;
          r = r.add (1);
          word = u64shr! (word, 8);
          bytes -= 1;
          if bytes == 0
            { break; }
        }
    }
  return resbuf;
}

unsafe fn sha3_conclude_ctx (ctx: *mut sha3_ctx)
{
  /* C's `ctx->buffer[ctx->buflen++] = 0x06;` is one statement: the index is the old
     value and the increment happens after it is read.  */
  (*ctx).buffer[(*ctx).buflen] = 0x06;
  (*ctx).buflen += 1;
  memset ((*ctx).buffer.as_mut_ptr ().add ((*ctx).buflen) as *mut c_void, b'\0' as c_int,
          (*ctx).blocklen - (*ctx).buflen);
  (*ctx).buffer[(*ctx).blocklen - 1] |= 0x80;
  sha3_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void, (*ctx).blocklen, ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_finish_ctx (ctx: *mut sha3_ctx, resbuf: *mut c_void)
  -> *mut c_void
{
  sha3_conclude_ctx (ctx);
  return sha3_read_ctx (ctx, resbuf);
}

/* C 122-131: `void *sha3_##SIZE##_buffer (...)`, with C's body.  */
macro_rules! DEFINE_SHA3_BUFFER {
  ($name:ident, $init:ident) => {
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn $name (buffer: *const c_char, len: usize,
                                    resblock: *mut c_void) -> *mut c_void
    {
      /* C's `struct sha3_ctx ctx;` (PORTING.md 1.5).  */
      let mut ctx: sha3_ctx = sha3_ctx { state: [0; 25], buffer: [0; 144], buflen: 0,
                                         digestlen: 0, blocklen: 0 };
      $init (&raw mut ctx);
      sha3_process_bytes (buffer as *const c_void, len, &raw mut ctx);
      return sha3_finish_ctx (&raw mut ctx, resblock);
    }
  };
}

DEFINE_SHA3_BUFFER! (sha3_224_buffer, sha3_224_init_ctx);
DEFINE_SHA3_BUFFER! (sha3_256_buffer, sha3_256_init_ctx);
DEFINE_SHA3_BUFFER! (sha3_384_buffer, sha3_384_init_ctx);
DEFINE_SHA3_BUFFER! (sha3_512_buffer, sha3_512_init_ctx);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_process_bytes (mut buffer: *const c_void, mut len: usize,
                                             ctx: *mut sha3_ctx) -> bool
{
  let mut buf: *const c_char = buffer as *const c_char;

  if 0 < (*ctx).buflen
    {
      let left: usize = (*ctx).blocklen - (*ctx).buflen;
      if len < left
        {
          /* Not enough to fill a full block.  */
          memcpy ((*ctx).buffer.as_mut_ptr ().add ((*ctx).buflen) as *mut c_void, buf as *const c_void, len);
          (*ctx).buflen += len;
          return true;
        }
      /* Process the block that already had bytes buffered.  */
      memcpy ((*ctx).buffer.as_mut_ptr ().add ((*ctx).buflen) as *mut c_void, buf as *const c_void, left);
      buf = buf.add (left);
      len -= left;
      sha3_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void, (*ctx).blocklen, ctx);
    }

  /* Process as many complete blocks as possible.  */
  let full_len: usize = len - len % (*ctx).blocklen;
  sha3_process_block (buf as *const c_void, full_len, ctx);
  buf = buf.add (full_len);
  len -= full_len;

  memcpy ((*ctx).buffer.as_mut_ptr () as *mut c_void, buf as *const c_void, len);
  (*ctx).buflen = len;
  return true;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha3_process_block (buffer: *const c_void, len: usize,
                                             ctx: *mut sha3_ctx) -> bool
{
  let a: *mut u64 = (*ctx).state.as_mut_ptr ();
  let mut words: *const u64 = buffer as *const u64;
  let nwords: usize = len / size_of::<u64> ();
  let endp: *const u64 = words.add (nwords);

  while words < endp
    {
      let mut i: usize = 0;
      while i < (*ctx).blocklen / size_of::<u64> ()
        {
          (*ctx).state[i] = u64xor! ((*ctx).state[i], SWAP! (*words));
          words = words.add (1);
          i += 1;
        }
      let mut i: c_int = 0;
      while i < 24
        {
          let mut c: [u64; 5] = [0; 5];
          let mut d: [u64; 5] = [0; 5];
          let mut t1: u64 = 0;
          let mut t2: u64 = 0;

          /* Theta step 1.  */
          c[0] = u64xor! (u64xor! (u64xor! (u64xor! (*a.add (0), *a.add (5)), *a.add (10)),
                                   *a.add (15)), *a.add (20));
          c[1] = u64xor! (u64xor! (u64xor! (u64xor! (*a.add (1), *a.add (6)), *a.add (11)),
                                   *a.add (16)), *a.add (21));
          c[2] = u64xor! (u64xor! (u64xor! (u64xor! (*a.add (2), *a.add (7)), *a.add (12)),
                                   *a.add (17)), *a.add (22));
          c[3] = u64xor! (u64xor! (u64xor! (u64xor! (*a.add (3), *a.add (8)), *a.add (13)),
                                   *a.add (18)), *a.add (23));
          c[4] = u64xor! (u64xor! (u64xor! (u64xor! (*a.add (4), *a.add (9)), *a.add (14)),
                                   *a.add (19)), *a.add (24));

          /* Theta step 2.  */
          d[0] = u64xor! (c[4], u64rol (c[1], 1));
          d[1] = u64xor! (c[0], u64rol (c[2], 1));
          d[2] = u64xor! (c[1], u64rol (c[3], 1));
          d[3] = u64xor! (c[2], u64rol (c[4], 1));
          d[4] = u64xor! (c[3], u64rol (c[0], 1));

          /* Theta step 3.  */
          *a.add (0) = u64xor! (*a.add (0), d[0]);
          *a.add (5) = u64xor! (*a.add (5), d[0]);
          *a.add (10) = u64xor! (*a.add (10), d[0]);
          *a.add (15) = u64xor! (*a.add (15), d[0]);
          *a.add (20) = u64xor! (*a.add (20), d[0]);
          *a.add (1) = u64xor! (*a.add (1), d[1]);
          *a.add (6) = u64xor! (*a.add (6), d[1]);
          *a.add (11) = u64xor! (*a.add (11), d[1]);
          *a.add (16) = u64xor! (*a.add (16), d[1]);
          *a.add (21) = u64xor! (*a.add (21), d[1]);
          *a.add (2) = u64xor! (*a.add (2), d[2]);
          *a.add (7) = u64xor! (*a.add (7), d[2]);
          *a.add (12) = u64xor! (*a.add (12), d[2]);
          *a.add (17) = u64xor! (*a.add (17), d[2]);
          *a.add (22) = u64xor! (*a.add (22), d[2]);
          *a.add (3) = u64xor! (*a.add (3), d[3]);
          *a.add (8) = u64xor! (*a.add (8), d[3]);
          *a.add (13) = u64xor! (*a.add (13), d[3]);
          *a.add (18) = u64xor! (*a.add (18), d[3]);
          *a.add (23) = u64xor! (*a.add (23), d[3]);
          *a.add (4) = u64xor! (*a.add (4), d[4]);
          *a.add (9) = u64xor! (*a.add (9), d[4]);
          *a.add (14) = u64xor! (*a.add (14), d[4]);
          *a.add (19) = u64xor! (*a.add (19), d[4]);
          *a.add (24) = u64xor! (*a.add (24), d[4]);

          /* Rho and Pi.  */
          t1 = *a.add (1);
          t2 = u64rol (t1, 1);
          t1 = *a.add (10);
          *a.add (10) = t2;
          t2 = u64rol (t1, 3);
          t1 = *a.add (7);
          *a.add (7) = t2;
          t2 = u64rol (t1, 6);
          t1 = *a.add (11);
          *a.add (11) = t2;
          t2 = u64rol (t1, 10);
          t1 = *a.add (17);
          *a.add (17) = t2;
          t2 = u64rol (t1, 15);
          t1 = *a.add (18);
          *a.add (18) = t2;
          t2 = u64rol (t1, 21);
          t1 = *a.add (3);
          *a.add (3) = t2;
          t2 = u64rol (t1, 28);
          t1 = *a.add (5);
          *a.add (5) = t2;
          t2 = u64rol (t1, 36);
          t1 = *a.add (16);
          *a.add (16) = t2;
          t2 = u64rol (t1, 45);
          t1 = *a.add (8);
          *a.add (8) = t2;
          t2 = u64rol (t1, 55);
          t1 = *a.add (21);
          *a.add (21) = t2;
          t2 = u64rol (t1, 2);
          t1 = *a.add (24);
          *a.add (24) = t2;
          t2 = u64rol (t1, 14);
          t1 = *a.add (4);
          *a.add (4) = t2;
          t2 = u64rol (t1, 27);
          t1 = *a.add (15);
          *a.add (15) = t2;
          t2 = u64rol (t1, 41);
          t1 = *a.add (23);
          *a.add (23) = t2;
          t2 = u64rol (t1, 56);
          t1 = *a.add (19);
          *a.add (19) = t2;
          t2 = u64rol (t1, 8);
          t1 = *a.add (13);
          *a.add (13) = t2;
          t2 = u64rol (t1, 25);
          t1 = *a.add (12);
          *a.add (12) = t2;
          t2 = u64rol (t1, 43);
          t1 = *a.add (2);
          *a.add (2) = t2;
          t2 = u64rol (t1, 62);
          t1 = *a.add (20);
          *a.add (20) = t2;
          t2 = u64rol (t1, 18);
          t1 = *a.add (14);
          *a.add (14) = t2;
          t2 = u64rol (t1, 39);
          t1 = *a.add (22);
          *a.add (22) = t2;
          t2 = u64rol (t1, 61);
          t1 = *a.add (9);
          *a.add (9) = t2;
          t2 = u64rol (t1, 20);
          t1 = *a.add (6);
          *a.add (6) = t2;
          t2 = u64rol (t1, 44);
          t1 = *a.add (1);
          *a.add (1) = t2;

          /* Chi.  */
          let mut j: c_int = 0;
          while j < 25
            {
              t1 = *a.add (j as usize);
              t2 = *a.add (j as usize + 1);
              *a.add (j as usize) = u64xor! (*a.add (j as usize),
                                             u64and! (u64not! (*a.add (j as usize + 1)),
                                                      *a.add (j as usize + 2)));
              *a.add (j as usize + 1) = u64xor! (*a.add (j as usize + 1),
                                                 u64and! (u64not! (*a.add (j as usize + 2)),
                                                          *a.add (j as usize + 3)));
              *a.add (j as usize + 2) = u64xor! (*a.add (j as usize + 2),
                                                 u64and! (u64not! (*a.add (j as usize + 3)),
                                                          *a.add (j as usize + 4)));
              *a.add (j as usize + 3) = u64xor! (*a.add (j as usize + 3),
                                                 u64and! (u64not! (*a.add (j as usize + 4)), t1));
              *a.add (j as usize + 4) = u64xor! (*a.add (j as usize + 4),
                                                 u64and! (u64not! (t1), t2));
              j += 5;
            }

          /* Iota.  */
          *a.add (0) = u64xor! (*a.add (0), rc[i as usize]);
          i += 1;
        }
    }
  return true;
}
