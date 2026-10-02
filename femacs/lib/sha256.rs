/* sha256.c - Functions to compute SHA256 and SHA224 message digest of files or
   memory blocks according to the NIST specification FIPS-180-2.

   Copyright (C) 2005-2006, 2008-2026 Free Software Foundation, Inc.

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

/* Written by David L. Jones <david@davidljones.name>
   Based on (and very similar to) the SHA1 implementation in
   Scott G. Miller's sha1.c
*/

/* emacs/lib/sha256.c: C 23's `<config.h>` is the generated config mirror, C 29's
   `"sha256.h"` is the header mirror, C 31-32's `<stdint.h>`/`<string.h>` are the stdint
   mirror plus the `memcpy` declaration below, and C 34's `<byteswap.h>` is the
   `bswap_32` C 38's live `SWAP` arm stands for (Rust's `swap_bytes`, PORTING.md 2.6).
   C 26-28's `#if HAVE_OPENSSL_SHA256` arm is dead on the frozen target and kept as text
   (P-51); C 41's `#if ! HAVE_OPENSSL_SHA256` is live and C 424's `#endif` closes it.  */
use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/stdint.in.rs");
include!("include/sha256.rs");

/* C 32's `<string.h>`: the one entry the bodies call.  */
unsafe extern "C" {
  pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/* C 27's `#define GL_OPENSSL_INLINE _GL_EXTERN_INLINE`, dead on this target and kept as
   text (P-51).  */
#[cfg (feature = "openssl_sha256")]
pub const GL_OPENSSL_INLINE: () = _GL_EXTERN_INLINE;

/* C 34-39's `<byteswap.h>` and the `SWAP` pair: the big-endian arm is `(n)` and the live
   little-endian arm is C's `bswap_32 (n)` = Rust's `swap_bytes` (PORTING.md 2.6).  */
/* The live arm first, so that the ledger's macro-body check reads the arm the frozen
   configuration compiles (extract-abi takes the first `macro_rules!` of a name); it is C's
   `bswap_32 (n)`, declared here as md5.rs declares it, because <byteswap.h>'s macro has no
   mirror yet.  */
pub fn bswap_32 (n: uint32_t) -> uint32_t { n.swap_bytes () }

macro_rules! SWAP { ($n:expr) => { bswap_32 ($n) } }
#[cfg (feature = "words_bigendian")]
macro_rules! SWAP { ($n:expr) => { $n } }

/* This array contains the bytes used to pad the buffer to the next
   64-byte boundary.  */
/* C: `static const unsigned char fillbuf[64] = { 0x80, 0, ..., };` - the initializer
   names 0x80 and leaves the rest of the array zero.  */
static fillbuf: [c_uchar; 64] = {
  let mut buf = [0; 64];
  buf[0] = 0x80;
  buf
};

/*
  Takes a pointer to a 256 bit block of data (eight 32 bit ints) and
  initializes it to the start constants of the SHA256 algorithm.  This
  must be called before using hash in the call to sha256_hash
*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_init_ctx (ctx: *mut sha256_ctx)
{
  (*ctx).state[0] = 0x6a09e667;
  (*ctx).state[1] = 0xbb67ae85;
  (*ctx).state[2] = 0x3c6ef372;
  (*ctx).state[3] = 0xa54ff53a;
  (*ctx).state[4] = 0x510e527f;
  (*ctx).state[5] = 0x9b05688c;
  (*ctx).state[6] = 0x1f83d9ab;
  (*ctx).state[7] = 0x5be0cd19;

  /* C's `ctx->total[0] = ctx->total[1] = 0;` is one statement.  */
  (*ctx).total[0] = { (*ctx).total[1] = 0; 0 };
  (*ctx).buflen = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha224_init_ctx (ctx: *mut sha256_ctx)
{
  (*ctx).state[0] = 0xc1059ed8;
  (*ctx).state[1] = 0x367cd507;
  (*ctx).state[2] = 0x3070dd17;
  (*ctx).state[3] = 0xf70e5939;
  (*ctx).state[4] = 0xffc00b31;
  (*ctx).state[5] = 0x68581511;
  (*ctx).state[6] = 0x64f98fa7;
  (*ctx).state[7] = 0xbefa4fa4;

  (*ctx).total[0] = { (*ctx).total[1] = 0; 0 };
  (*ctx).buflen = 0;
}

/* Copy the value from v into the memory location pointed to by *CP,
   If your architecture allows unaligned access, this is equivalent to
   * (__typeof__ (v) *) cp = v  */
unsafe fn set_uint32 (cp: *mut c_char, v: uint32_t)
{
  memcpy (cp as *mut c_void, &v as *const uint32_t as *const c_void, size_of::<uint32_t> ());
}

/* Put result from CTX in first 32 bytes following RESBUF.
   The result must be in little endian byte order.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_read_ctx (ctx: *const sha256_ctx, resbuf: *mut c_void)
  -> *mut c_void
{
  let r: *mut c_char = resbuf as *mut c_char;

  let mut i: c_int = 0;
  while i < 8
    {
      set_uint32 (r.add (i as usize * size_of::<uint32_t> ()),
                  SWAP! ((*ctx).state[i as usize]));
      i += 1;
    }

  return resbuf;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha224_read_ctx (ctx: *const sha256_ctx, resbuf: *mut c_void)
  -> *mut c_void
{
  let r: *mut c_char = resbuf as *mut c_char;

  let mut i: c_int = 0;
  while i < 7
    {
      set_uint32 (r.add (i as usize * size_of::<uint32_t> ()),
                  SWAP! ((*ctx).state[i as usize]));
      i += 1;
    }

  return resbuf;
}

/* Process the remaining bytes in the internal buffer and the usual
   prolog according to the standard and write the result to RESBUF.  */
unsafe fn sha256_conclude_ctx (ctx: *mut sha256_ctx)
{
  /* Take yet unprocessed bytes into account.  */
  let bytes: usize = (*ctx).buflen;
  let size: usize = if bytes < 56 { 64 / 4 } else { 64 * 2 / 4 };

  /* Now count remaining bytes.  */
  (*ctx).total[0] = (*ctx).total[0].wrapping_add (bytes as uint32_t);
  if ((*ctx).total[0] as usize) < bytes
    { (*ctx).total[1] = (*ctx).total[1].wrapping_add (1); }

  /* Put the 64-bit file length in *bits* at the end of the buffer.
     Use set_uint32 rather than a simple assignment, to avoid risk of
     unaligned access.  */
  set_uint32 ((*ctx).buffer.as_mut_ptr ().add (size - 2) as *mut c_char,
              SWAP! (((*ctx).total[1] << 3) | ((*ctx).total[0] >> 29)));
  set_uint32 ((*ctx).buffer.as_mut_ptr ().add (size - 1) as *mut c_char,
              SWAP! ((*ctx).total[0] << 3));

  memcpy ((*ctx).buffer.as_mut_ptr ().add (bytes) as *mut c_void,
          fillbuf.as_ptr () as *const c_void,
          (size - 2) * 4 - bytes);

  /* Process last bytes.  */
  sha256_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void, size * 4, ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_finish_ctx (ctx: *mut sha256_ctx, resbuf: *mut c_void)
  -> *mut c_void
{
  sha256_conclude_ctx (ctx);
  return sha256_read_ctx (ctx, resbuf);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha224_finish_ctx (ctx: *mut sha256_ctx, resbuf: *mut c_void)
  -> *mut c_void
{
  sha256_conclude_ctx (ctx);
  return sha224_read_ctx (ctx, resbuf);
}

/* Compute SHA256 message digest for LEN bytes beginning at BUFFER.  The
   result is always in little endian byte order, so that a byte-wise
   output yields to the wanted ASCII representation of the message
   digest.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_buffer (buffer: *const c_char, len: usize,
                                        resblock: *mut c_void) -> *mut c_void
{
  /* C's `struct sha256_ctx ctx;` (PORTING.md 1.5).  */
  let mut ctx: sha256_ctx = sha256_ctx { state: [0; 8], total: [0, 0], buflen: 0,
                                         buffer: [0; 32] };

  /* Initialize the computation context.  */
  sha256_init_ctx (&raw mut ctx);

  /* Process whole buffer but last len % 64 bytes.  */
  sha256_process_bytes (buffer as *const c_void, len, &raw mut ctx);

  /* Put result in desired memory area.  */
  return sha256_finish_ctx (&raw mut ctx, resblock);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha224_buffer (buffer: *const c_char, len: usize,
                                        resblock: *mut c_void) -> *mut c_void
{
  let mut ctx: sha256_ctx = sha256_ctx { state: [0; 8], total: [0, 0], buflen: 0,
                                         buffer: [0; 32] };

  /* Initialize the computation context.  */
  sha224_init_ctx (&raw mut ctx);

  /* Process whole buffer but last len % 64 bytes.  */
  sha256_process_bytes (buffer as *const c_void, len, &raw mut ctx);

  /* Put result in desired memory area.  */
  return sha224_finish_ctx (&raw mut ctx, resblock);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_process_bytes (mut buffer: *const c_void, mut len: usize,
                                               ctx: *mut sha256_ctx)
{
  /* When we already have some bits in our internal buffer concatenate
     both inputs first.  */
  if (*ctx).buflen != 0
    {
      let mut left_over: usize = (*ctx).buflen;
      let add: usize = if 128 - left_over > len { len } else { 128 - left_over };

      memcpy ((*ctx).buffer.as_mut_ptr ().add (left_over) as *mut c_void, buffer, add);
      (*ctx).buflen = (*ctx).buflen.wrapping_add (add);

      if (*ctx).buflen > 64
        {
          sha256_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void,
                                (*ctx).buflen & !63, ctx);

          (*ctx).buflen &= 63;
          /* The regions in the following copy operation cannot overlap,
             because ctx->buflen < 64 <= (left_over + add) & ~63.  */
          memcpy ((*ctx).buffer.as_mut_ptr () as *mut c_void,
                  (*ctx).buffer.as_mut_ptr ().add ((left_over + add) & !63) as *const c_void,
                  (*ctx).buflen);
        }

      buffer = (buffer as *const c_char).add (add) as *const c_void;
      len -= add;
    }

  /* Process available complete blocks.  */
  if len >= 64
    {
      /* C 227-228's `#if !(_STRING_ARCH_unaligned || _STRING_INLINE_unaligned)` arm is
         live for this target, and C defines `UNALIGNED_P` here; Rust's `macro_rules!`
         are textual, so the definition stays inside the same block as its uses.  */
      macro_rules! UNALIGNED_P { ($p:expr) => { ($p as usize) % align_of::<uint32_t> () != 0 } }
      if UNALIGNED_P! (buffer)
        { while len > 64
          {
            sha256_process_block (memcpy ((*ctx).buffer.as_mut_ptr () as *mut c_void, buffer, 64),
                                  64, ctx);
            buffer = (buffer as *const c_char).add (64) as *const c_void;
            len -= 64;
          } }
      else
        {
          sha256_process_block (buffer, len & !63, ctx);
          buffer = (buffer as *const c_char).add (len & !63) as *const c_void;
          len &= 63;
        }
    }

  /* Move remaining bytes in internal buffer.  */
  if len > 0
    {
      let mut left_over: usize = (*ctx).buflen;

      memcpy ((*ctx).buffer.as_mut_ptr ().add (left_over) as *mut c_void, buffer, len);
      left_over += len;
      if left_over >= 64
        {
          sha256_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void, 64, ctx);
          left_over -= 64;
          /* The regions in the following copy operation cannot overlap,
             because left_over <= 64.  */
          memcpy ((*ctx).buffer.as_mut_ptr () as *mut c_void,
                  (*ctx).buffer.as_mut_ptr ().add (16) as *const c_void, left_over);
        }
      (*ctx).buflen = left_over;
    }
}

/* --- Code below is the primary difference between sha1.c and sha256.c --- */

/* SHA256 round constants */
macro_rules! K { ($i:expr) => { sha256_round_constants[($i)] } }
static sha256_round_constants: [uint32_t; 64] = [
  0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
  0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
  0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
  0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
  0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
  0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
  0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
  0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
  0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
  0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
  0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
  0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
  0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
  0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
  0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
  0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/* Round functions.  */
macro_rules! F2 { ($A:expr, $B:expr, $C:expr) => { (($A & $B) | ($C & ($A | $B))) } }
macro_rules! F1 { ($E:expr, $F:expr, $G:expr) => { ($G ^ ($E & ($F ^ $G))) } }

/* Process LEN bytes of BUFFER, accumulating context into CTX.
   It is assumed that LEN % 64 == 0.
   Most of this code comes from GnuPG's cipher/sha1.c.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha256_process_block (buffer: *const c_void, len: usize,
                                               ctx: *mut sha256_ctx)
{
  let mut words: *const uint32_t = buffer as *const uint32_t;
  let nwords: usize = len / size_of::<uint32_t> ();
  let endp: *const uint32_t = words.add (nwords);
  let mut x: [uint32_t; 16] = [0; 16];
  let mut a: uint32_t = (*ctx).state[0];
  let mut b: uint32_t = (*ctx).state[1];
  let mut c: uint32_t = (*ctx).state[2];
  let mut d: uint32_t = (*ctx).state[3];
  let mut e: uint32_t = (*ctx).state[4];
  let mut f: uint32_t = (*ctx).state[5];
  let mut g: uint32_t = (*ctx).state[6];
  let mut h: uint32_t = (*ctx).state[7];
  let lolen: uint32_t = len as uint32_t;

  /* First increment the byte count.  FIPS PUB 180-2 specifies the possible
     length of the file up to 2^64 bits.  Here we only compute the
     number of bytes.  Do a double word increment.  */
  (*ctx).total[0] = (*ctx).total[0].wrapping_add (lolen);
  (*ctx).total[1] = (*ctx).total[1]
    .wrapping_add ((len >> 31 >> 1) as uint32_t)
    .wrapping_add (((*ctx).total[0] < lolen) as uint32_t);

  macro_rules! rol { ($x:expr, $n:expr) => { (($x) << ($n)) | (($x) >> (32 - ($n))) } }
  macro_rules! S0 { ($x:expr) => { (rol! ($x, 25) ^ rol! ($x, 14) ^ (($x) >> 3)) } }
  macro_rules! S1 { ($x:expr) => { (rol! ($x, 15) ^ rol! ($x, 13) ^ (($x) >> 10)) } }
  macro_rules! SS0 { ($x:expr) => { (rol! ($x, 30) ^ rol! ($x, 19) ^ rol! ($x, 10)) } }
  macro_rules! SS1 { ($x:expr) => { (rol! ($x, 26) ^ rol! ($x, 21) ^ (($x) >> 7)) } }

  /* C 325-327's `M(I)`: the comma operator's value is the stored word, and C's `tm` is
     the expansion's own local here (Rust macro hygiene; C assigns it before every read
     and never carries it between expansions - docs/ports/sha256.md).  */
  macro_rules! M {
    ($i:expr) => {
      {
        let tm: uint32_t = S1! (x[(($i) - 2) & 0x0f]).wrapping_add (x[(($i) - 7) & 0x0f])
          .wrapping_add (S0! (x[(($i) - 15) & 0x0f]))
          .wrapping_add (x[($i) & 0x0f]);
        x[($i) & 0x0f] = tm;
        tm
      }
    }
  }

  /* C 329-335's `R(A,…,K,M)`: C's `A`..`H` are the caller's locals, passed as the
     macro's first eight arguments exactly as C's call sites pass them; `t0`/`t1` are the
     expansion's own locals (C declares them inside the macro too).  `do { … } while (0)`
     is a block and C's unsigned `+=`/`+` are `wrapping_add`.  */
  macro_rules! R {
    ($A:ident, $B:ident, $C:ident, $D:ident, $E:ident, $F:ident, $G:ident, $H:ident,
     $K:expr, $M:expr) => {
      {
        let t0: uint32_t = SS0! ($A).wrapping_add (F2! ($A, $B, $C));
        let t1: uint32_t = $H.wrapping_add (SS1! ($E))
          .wrapping_add (F1! ($E, $F, $G))
          .wrapping_add ($K)
          .wrapping_add ($M);
        $D = $D.wrapping_add (t1);
        $H = t0.wrapping_add (t1);
      }
    }
  }

  while words < endp
    {
      /* FIXME: see sha1.c for a better implementation.  */
      let mut t: c_int = 0;
      while t < 16
        {
          x[t as usize] = SWAP! (*words);
          words = words.add (1);
          t += 1;
        }

      /* C's `uint32_t tm;` (C 346) is the `M!` expansion's own local above.  */

      R! (a, b, c, d, e, f, g, h, K!( 0), x[ 0]);
      R! (h, a, b, c, d, e, f, g, K!( 1), x[ 1]);
      R! (g, h, a, b, c, d, e, f, K!( 2), x[ 2]);
      R! (f, g, h, a, b, c, d, e, K!( 3), x[ 3]);
      R! (e, f, g, h, a, b, c, d, K!( 4), x[ 4]);
      R! (d, e, f, g, h, a, b, c, K!( 5), x[ 5]);
      R! (c, d, e, f, g, h, a, b, K!( 6), x[ 6]);
      R! (b, c, d, e, f, g, h, a, K!( 7), x[ 7]);
      R! (a, b, c, d, e, f, g, h, K!( 8), x[ 8]);
      R! (h, a, b, c, d, e, f, g, K!( 9), x[ 9]);
      R! (g, h, a, b, c, d, e, f, K!(10), x[10]);
      R! (f, g, h, a, b, c, d, e, K!(11), x[11]);
      R! (e, f, g, h, a, b, c, d, K!(12), x[12]);
      R! (d, e, f, g, h, a, b, c, K!(13), x[13]);
      R! (c, d, e, f, g, h, a, b, K!(14), x[14]);
      R! (b, c, d, e, f, g, h, a, K!(15), x[15]);
      R! (a, b, c, d, e, f, g, h, K!(16), M!(16));
      R! (h, a, b, c, d, e, f, g, K!(17), M!(17));
      R! (g, h, a, b, c, d, e, f, K!(18), M!(18));
      R! (f, g, h, a, b, c, d, e, K!(19), M!(19));
      R! (e, f, g, h, a, b, c, d, K!(20), M!(20));
      R! (d, e, f, g, h, a, b, c, K!(21), M!(21));
      R! (c, d, e, f, g, h, a, b, K!(22), M!(22));
      R! (b, c, d, e, f, g, h, a, K!(23), M!(23));
      R! (a, b, c, d, e, f, g, h, K!(24), M!(24));
      R! (h, a, b, c, d, e, f, g, K!(25), M!(25));
      R! (g, h, a, b, c, d, e, f, K!(26), M!(26));
      R! (f, g, h, a, b, c, d, e, K!(27), M!(27));
      R! (e, f, g, h, a, b, c, d, K!(28), M!(28));
      R! (d, e, f, g, h, a, b, c, K!(29), M!(29));
      R! (c, d, e, f, g, h, a, b, K!(30), M!(30));
      R! (b, c, d, e, f, g, h, a, K!(31), M!(31));
      R! (a, b, c, d, e, f, g, h, K!(32), M!(32));
      R! (h, a, b, c, d, e, f, g, K!(33), M!(33));
      R! (g, h, a, b, c, d, e, f, K!(34), M!(34));
      R! (f, g, h, a, b, c, d, e, K!(35), M!(35));
      R! (e, f, g, h, a, b, c, d, K!(36), M!(36));
      R! (d, e, f, g, h, a, b, c, K!(37), M!(37));
      R! (c, d, e, f, g, h, a, b, K!(38), M!(38));
      R! (b, c, d, e, f, g, h, a, K!(39), M!(39));
      R! (a, b, c, d, e, f, g, h, K!(40), M!(40));
      R! (h, a, b, c, d, e, f, g, K!(41), M!(41));
      R! (g, h, a, b, c, d, e, f, K!(42), M!(42));
      R! (f, g, h, a, b, c, d, e, K!(43), M!(43));
      R! (e, f, g, h, a, b, c, d, K!(44), M!(44));
      R! (d, e, f, g, h, a, b, c, K!(45), M!(45));
      R! (c, d, e, f, g, h, a, b, K!(46), M!(46));
      R! (b, c, d, e, f, g, h, a, K!(47), M!(47));
      R! (a, b, c, d, e, f, g, h, K!(48), M!(48));
      R! (h, a, b, c, d, e, f, g, K!(49), M!(49));
      R! (g, h, a, b, c, d, e, f, K!(50), M!(50));
      R! (f, g, h, a, b, c, d, e, K!(51), M!(51));
      R! (e, f, g, h, a, b, c, d, K!(52), M!(52));
      R! (d, e, f, g, h, a, b, c, K!(53), M!(53));
      R! (c, d, e, f, g, h, a, b, K!(54), M!(54));
      R! (b, c, d, e, f, g, h, a, K!(55), M!(55));
      R! (a, b, c, d, e, f, g, h, K!(56), M!(56));
      R! (h, a, b, c, d, e, f, g, K!(57), M!(57));
      R! (g, h, a, b, c, d, e, f, K!(58), M!(58));
      R! (f, g, h, a, b, c, d, e, K!(59), M!(59));
      R! (e, f, g, h, a, b, c, d, K!(60), M!(60));
      R! (d, e, f, g, h, a, b, c, K!(61), M!(61));
      R! (c, d, e, f, g, h, a, b, K!(62), M!(62));
      R! (b, c, d, e, f, g, h, a, K!(63), M!(63));

      /* C's `a = ctx->state[0] += a;` is one statement.  */
      a = { (*ctx).state[0] = (*ctx).state[0].wrapping_add (a); (*ctx).state[0] };
      b = { (*ctx).state[1] = (*ctx).state[1].wrapping_add (b); (*ctx).state[1] };
      c = { (*ctx).state[2] = (*ctx).state[2].wrapping_add (c); (*ctx).state[2] };
      d = { (*ctx).state[3] = (*ctx).state[3].wrapping_add (d); (*ctx).state[3] };
      e = { (*ctx).state[4] = (*ctx).state[4].wrapping_add (e); (*ctx).state[4] };
      f = { (*ctx).state[5] = (*ctx).state[5].wrapping_add (f); (*ctx).state[5] };
      g = { (*ctx).state[6] = (*ctx).state[6].wrapping_add (g); (*ctx).state[6] };
      h = { (*ctx).state[7] = (*ctx).state[7].wrapping_add (h); (*ctx).state[7] };
    }
}
