// femacs: port of lib/sha1.c @ fd7afced403.
/* Written by Scott G. Miller
   Credits:
      Robert Klep <robert@ilse.nl>  -- Expansion function fix
*/

/* emacs/lib/sha1.c: C 24's `<config.h>` is the generated config mirror, C 30's
   `"sha1.h"` is the header mirror, C 32-33's `<stdint.h>`/`<string.h>` are the stdint
   mirror plus the `memcpy` declaration below, and C 35's `<byteswap.h>` is the
   `bswap_32` C 39's live `SWAP` arm stands for (Rust spells it `swap_bytes`,
   PORTING.md 2.6).  C 27-29's `#if HAVE_OPENSSL_SHA1` arm is dead on the frozen target
   (the macro is not in the configuration) and is kept as text (P-51).  C 42's
   `#if ! HAVE_OPENSSL_SHA1` is live, and C 355's `#endif` closes it.  */
use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/stdint.in.rs");
include!("include/sha1.rs");

/* C 33's `<string.h>`: the one entry the bodies call.  */
unsafe extern "C" {
  pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/* C 28's `#define GL_OPENSSL_INLINE _GL_EXTERN_INLINE`, dead on this target and kept as
   text (P-51).  */
#[cfg (feature = "openssl_sha1")]
pub const GL_OPENSSL_INLINE: () = _GL_EXTERN_INLINE;

/* C 35-40's `<byteswap.h>` and the `SWAP` pair.  This file's arms are the mirror image of
   md5.c's: the big-endian arm is `(n)` and the live little-endian arm is C's
   `bswap_32 (n)`, which is what Rust's `swap_bytes` is (the `<byteswap.h>` macro has no
   ported declaration; PORTING.md 2.6 maps `__builtin_bswap*` to `swap_bytes`).  */
/* The live arm first, so that the ledger's macro-body check reads the arm the frozen
   configuration compiles (extract-abi takes the first `macro_rules!` of a name); it is C's
   `bswap_32 (n)`, declared here as md5.rs declares it, because <byteswap.h>'s macro has no
   mirror yet.  */
pub fn bswap_32 (n: c_uint) -> c_uint { n.swap_bytes () }

macro_rules! SWAP { ($n:expr) => { bswap_32 ($n) } }
#[cfg (feature = "words_bigendian")]
macro_rules! SWAP { ($n:expr) => { $n } }

/* This array contains the bytes used to pad the buffer to the next
   64-byte boundary.  (RFC 1321, 3.1: Step 1)  */
/* C: `static const unsigned char fillbuf[64] = { 0x80, 0, ..., };` - the initializer
   names 0x80 and leaves the rest of the array zero.  */
static fillbuf: [c_uchar; 64] = {
  let mut buf = [0; 64];
  buf[0] = 0x80;
  buf
};

/* Take a pointer to a 160 bit block of data (five 32 bit ints) and
   initialize it to the start constants of the SHA1 algorithm.  This
   must be called before using hash in the call to sha1_hash.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_init_ctx (ctx: *mut sha1_ctx)
{
  (*ctx).A = 0x67452301;
  (*ctx).B = 0xefcdab89;
  (*ctx).C = 0x98badcfe;
  (*ctx).D = 0x10325476;
  (*ctx).E = 0xc3d2e1f0;

  /* C's `ctx->total[0] = ctx->total[1] = 0;` is one statement: the right-hand
     assignment is evaluated first and its value is the left-hand one's source.  */
  (*ctx).total[0] = { (*ctx).total[1] = 0; 0 };
  (*ctx).buflen = 0;
}

/* Copy the 4 byte value from v into the memory location pointed to by *cp,
   If your architecture allows unaligned access this is equivalent to
   * (uint32_t *) cp = v  */
unsafe fn set_uint32 (cp: *mut c_char, v: uint32_t)
{
  memcpy (cp as *mut c_void, &v as *const uint32_t as *const c_void, size_of::<uint32_t> ());
}

/* Put result from CTX in first 20 bytes following RESBUF.  The result
   must be in little endian byte order.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_read_ctx (ctx: *const sha1_ctx, resbuf: *mut c_void)
  -> *mut c_void
{
  let r: *mut c_char = resbuf as *mut c_char;
  set_uint32 (r.add (0 * size_of::<uint32_t> ()), SWAP! ((*ctx).A));
  set_uint32 (r.add (1 * size_of::<uint32_t> ()), SWAP! ((*ctx).B));
  set_uint32 (r.add (2 * size_of::<uint32_t> ()), SWAP! ((*ctx).C));
  set_uint32 (r.add (3 * size_of::<uint32_t> ()), SWAP! ((*ctx).D));
  set_uint32 (r.add (4 * size_of::<uint32_t> ()), SWAP! ((*ctx).E));

  return resbuf;
}

/* Process the remaining bytes in the internal buffer and the usual
   prolog according to the standard and write the result to RESBUF.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_finish_ctx (ctx: *mut sha1_ctx, resbuf: *mut c_void)
  -> *mut c_void
{
  /* Take yet unprocessed bytes into account.  */
  let bytes: uint32_t = (*ctx).buflen;
  let size: usize = if bytes < 56 { 64 / 4 } else { 64 * 2 / 4 };

  /* Now count remaining bytes.  */
  (*ctx).total[0] = (*ctx).total[0].wrapping_add (bytes);
  if (*ctx).total[0] < bytes
    { (*ctx).total[1] = (*ctx).total[1].wrapping_add (1); }

  /* Put the 64-bit file length in *bits* at the end of the buffer.  */
  (*ctx).buffer[size - 2] = SWAP! (((*ctx).total[1] << 3) | ((*ctx).total[0] >> 29));
  (*ctx).buffer[size - 1] = SWAP! ((*ctx).total[0] << 3);

  memcpy ((*ctx).buffer.as_mut_ptr ().add (bytes as usize) as *mut c_void,
          fillbuf.as_ptr () as *const c_void,
          (size - 2) * 4 - bytes as usize);

  /* Process last bytes.  */
  sha1_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void, size * 4, ctx);

  return sha1_read_ctx (ctx, resbuf);
}

/* Compute SHA1 message digest for LEN bytes beginning at BUFFER.  The
   result is always in little endian byte order, so that a byte-wise
   output yields to the wanted ASCII representation of the message
   digest.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
  -> *mut c_void
{
  /* C's `struct sha1_ctx ctx;` is filled by sha1_init_ctx below; a Rust local needs a
     value before its address can be taken (PORTING.md 1.5), so it starts zeroed.  */
  let mut ctx: sha1_ctx = sha1_ctx { A: 0, B: 0, C: 0, D: 0, E: 0, total: [0, 0], buflen: 0,
                                     buffer: [0; 32] };

  /* Initialize the computation context.  */
  sha1_init_ctx (&raw mut ctx);

  /* Process whole buffer but last len % 64 bytes.  */
  sha1_process_bytes (buffer as *const c_void, len, &raw mut ctx);

  /* Put result in desired memory area.  */
  return sha1_finish_ctx (&raw mut ctx, resblock);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_process_bytes (mut buffer: *const c_void, mut len: usize,
                                             ctx: *mut sha1_ctx)
{
  /* When we already have some bits in our internal buffer concatenate
     both inputs first.  */
  if (*ctx).buflen != 0
    {
      let mut left_over: usize = (*ctx).buflen as usize;
      let add: usize = if 128 - left_over > len { len } else { 128 - left_over };

      memcpy ((*ctx).buffer.as_mut_ptr ().add (left_over) as *mut c_void, buffer, add);
      (*ctx).buflen = (*ctx).buflen.wrapping_add (add as uint32_t);

      if (*ctx).buflen > 64
        {
          sha1_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void,
                              (*ctx).buflen as usize & !63, ctx);

          (*ctx).buflen &= 63;
          /* The regions in the following copy operation cannot overlap,
             because ctx->buflen < 64 <= (left_over + add) & ~63.  */
          memcpy ((*ctx).buffer.as_mut_ptr () as *mut c_void,
                  (*ctx).buffer.as_mut_ptr ().add ((left_over + add) & !63) as *const c_void,
                  (*ctx).buflen as usize);
        }

      buffer = (buffer as *const c_char).add (add) as *const c_void;
      len -= add;
    }

  /* Process available complete blocks.  */
  if len >= 64
    {
      /* C 167-168's `#if !(_STRING_ARCH_unaligned || _STRING_INLINE_unaligned)` arm is
         live for this target, and C defines `UNALIGNED_P` here; Rust's `macro_rules!`
         are textual, so the definition stays inside the same block as its uses.  */
      macro_rules! UNALIGNED_P { ($p:expr) => { ($p as usize) % align_of::<uint32_t> () != 0 } }
      if UNALIGNED_P! (buffer)
        { while len > 64
          {
            sha1_process_block (memcpy ((*ctx).buffer.as_mut_ptr () as *mut c_void, buffer, 64),
                                64, ctx);
            buffer = (buffer as *const c_char).add (64) as *const c_void;
            len -= 64;
          } }
      else
        {
          sha1_process_block (buffer, len & !63, ctx);
          buffer = (buffer as *const c_char).add (len & !63) as *const c_void;
          len &= 63;
        }
    }

  /* Move remaining bytes in internal buffer.  */
  if len > 0
    {
      let mut left_over: usize = (*ctx).buflen as usize;

      memcpy ((*ctx).buffer.as_mut_ptr ().add (left_over) as *mut c_void, buffer, len);
      left_over += len;
      if left_over >= 64
        {
          sha1_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void, 64, ctx);
          left_over -= 64;
          /* The regions in the following copy operation cannot overlap,
             because left_over <= 64.  */
          memcpy ((*ctx).buffer.as_mut_ptr () as *mut c_void,
                  (*ctx).buffer.as_mut_ptr ().add (16) as *const c_void, left_over);
        }
      (*ctx).buflen = left_over as uint32_t;
    }
}

/* --- Code below is the primary difference between md5.c and sha1.c --- */

/* SHA1 round constants */
pub const K1: uint32_t = 0x5a827999;
pub const K2: uint32_t = 0x6ed9eba1;
pub const K3: uint32_t = 0x8f1bbcdc;
pub const K4: uint32_t = 0xca62c1d6;

/* Round functions.  Note that F2 is the same as F4.  */
macro_rules! F1 { ($B:expr, $C:expr, $D:expr) => { ($D ^ ($B & ($C ^ $D))) } }
macro_rules! F2 { ($B:expr, $C:expr, $D:expr) => { ($B ^ $C ^ $D) } }
macro_rules! F3 { ($B:expr, $C:expr, $D:expr) => { (($B & $C) | ($D & ($B | $C))) } }
macro_rules! F4 { ($B:expr, $C:expr, $D:expr) => { ($B ^ $C ^ $D) } }

/* Process LEN bytes of BUFFER, accumulating context into CTX.
   It is assumed that LEN % 64 == 0.
   Most of this code comes from GnuPG's cipher/sha1.c.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1_process_block (buffer: *const c_void, len: usize,
                                             ctx: *mut sha1_ctx)
{
  let mut words: *const uint32_t = buffer as *const uint32_t;
  let nwords: usize = len / size_of::<uint32_t> ();
  let endp: *const uint32_t = words.add (nwords);
  let mut x: [uint32_t; 16] = [0; 16];
  let mut a: uint32_t = (*ctx).A;
  let mut b: uint32_t = (*ctx).B;
  let mut c: uint32_t = (*ctx).C;
  let mut d: uint32_t = (*ctx).D;
  let mut e: uint32_t = (*ctx).E;
  let lolen: uint32_t = len as uint32_t;

  /* First increment the byte count.  RFC 1321 specifies the possible
     length of the file up to 2^64 bits.  Here we only compute the
     number of bytes.  Do a double word increment.  */
  (*ctx).total[0] = (*ctx).total[0].wrapping_add (lolen);
  (*ctx).total[1] = (*ctx).total[1]
    .wrapping_add ((len >> 31 >> 1) as uint32_t)
    .wrapping_add (((*ctx).total[0] < lolen) as uint32_t);

  /* C 243's `rol(x, n)`: the compound literal `(uint32_t) {(x)}` is the cast to the
     rotated type, and C's unsigned `<<`/`|` are Rust's.  */
  macro_rules! rol {
    ($x:expr, $n:expr) => { (($x) << ($n)) | (($x as uint32_t) >> (32 - ($n))) }
  }

  /* C 245-247's `M(I)`: the comma operator's value is the stored word.  C writes the
     message schedule into its loop-local `tm`; the port's expansion has its own `tm`
     (Rust macro hygiene), which is the same value - C's slot is assigned before every
     read and never carries state between expansions (docs/ports/sha1.md).  */
  macro_rules! M {
    ($i:expr) => {
      {
        let tm: uint32_t = x[($i) & 0x0f] ^ x[(($i) - 14) & 0x0f]
                           ^ x[(($i) - 8) & 0x0f] ^ x[(($i) - 3) & 0x0f];
        x[($i) & 0x0f] = rol! (tm, 1);
        x[($i) & 0x0f]
      }
    }
  }

  /* C 249-254's `R(A,B,C,D,E,F,K,M)`: C's `A`..`E` are the caller's locals, which the
     port passes as the macro's first five arguments (C's call sites pass them by name
     too), so the invocations below are C's; `F`/`K`/`M` are the same
     expression/macro arguments C passes.  `do { … } while (0)` is a block and C's
     unsigned `+=` is `wrapping_add`.  */
  macro_rules! R {
    ($A:ident, $B:ident, $C:ident, $D:ident, $E:ident, $F:ident, $K:expr, $M:expr) => {
      {
        $E = $E.wrapping_add (rol! ($A, 5)
                              .wrapping_add ($F! ($B, $C, $D))
                              .wrapping_add ($K)
                              .wrapping_add ($M));
        $B = rol! ($B, 30);
      }
    }
  }

  while words < endp
    {
      let mut t: c_int = 0;
      while t < 16
        {
          x[t as usize] = SWAP! (*words);
          words = words.add (1);
          t += 1;
        }

      /* C's `uint32_t tm;` (C 264) is the `M!` expansion's own local above.  */

      R! (a, b, c, d, e, F1, K1, x[ 0]);
      R! (e, a, b, c, d, F1, K1, x[ 1]);
      R! (d, e, a, b, c, F1, K1, x[ 2]);
      R! (c, d, e, a, b, F1, K1, x[ 3]);
      R! (b, c, d, e, a, F1, K1, x[ 4]);
      R! (a, b, c, d, e, F1, K1, x[ 5]);
      R! (e, a, b, c, d, F1, K1, x[ 6]);
      R! (d, e, a, b, c, F1, K1, x[ 7]);
      R! (c, d, e, a, b, F1, K1, x[ 8]);
      R! (b, c, d, e, a, F1, K1, x[ 9]);
      R! (a, b, c, d, e, F1, K1, x[10]);
      R! (e, a, b, c, d, F1, K1, x[11]);
      R! (d, e, a, b, c, F1, K1, x[12]);
      R! (c, d, e, a, b, F1, K1, x[13]);
      R! (b, c, d, e, a, F1, K1, x[14]);
      R! (a, b, c, d, e, F1, K1, x[15]);
      R! (e, a, b, c, d, F1, K1, M!(16));
      R! (d, e, a, b, c, F1, K1, M!(17));
      R! (c, d, e, a, b, F1, K1, M!(18));
      R! (b, c, d, e, a, F1, K1, M!(19));
      R! (a, b, c, d, e, F2, K2, M!(20));
      R! (e, a, b, c, d, F2, K2, M!(21));
      R! (d, e, a, b, c, F2, K2, M!(22));
      R! (c, d, e, a, b, F2, K2, M!(23));
      R! (b, c, d, e, a, F2, K2, M!(24));
      R! (a, b, c, d, e, F2, K2, M!(25));
      R! (e, a, b, c, d, F2, K2, M!(26));
      R! (d, e, a, b, c, F2, K2, M!(27));
      R! (c, d, e, a, b, F2, K2, M!(28));
      R! (b, c, d, e, a, F2, K2, M!(29));
      R! (a, b, c, d, e, F2, K2, M!(30));
      R! (e, a, b, c, d, F2, K2, M!(31));
      R! (d, e, a, b, c, F2, K2, M!(32));
      R! (c, d, e, a, b, F2, K2, M!(33));
      R! (b, c, d, e, a, F2, K2, M!(34));
      R! (a, b, c, d, e, F2, K2, M!(35));
      R! (e, a, b, c, d, F2, K2, M!(36));
      R! (d, e, a, b, c, F2, K2, M!(37));
      R! (c, d, e, a, b, F2, K2, M!(38));
      R! (b, c, d, e, a, F2, K2, M!(39));
      R! (a, b, c, d, e, F3, K3, M!(40));
      R! (e, a, b, c, d, F3, K3, M!(41));
      R! (d, e, a, b, c, F3, K3, M!(42));
      R! (c, d, e, a, b, F3, K3, M!(43));
      R! (b, c, d, e, a, F3, K3, M!(44));
      R! (a, b, c, d, e, F3, K3, M!(45));
      R! (e, a, b, c, d, F3, K3, M!(46));
      R! (d, e, a, b, c, F3, K3, M!(47));
      R! (c, d, e, a, b, F3, K3, M!(48));
      R! (b, c, d, e, a, F3, K3, M!(49));
      R! (a, b, c, d, e, F3, K3, M!(50));
      R! (e, a, b, c, d, F3, K3, M!(51));
      R! (d, e, a, b, c, F3, K3, M!(52));
      R! (c, d, e, a, b, F3, K3, M!(53));
      R! (b, c, d, e, a, F3, K3, M!(54));
      R! (a, b, c, d, e, F3, K3, M!(55));
      R! (e, a, b, c, d, F3, K3, M!(56));
      R! (d, e, a, b, c, F3, K3, M!(57));
      R! (c, d, e, a, b, F3, K3, M!(58));
      R! (b, c, d, e, a, F3, K3, M!(59));
      R! (a, b, c, d, e, F4, K4, M!(60));
      R! (e, a, b, c, d, F4, K4, M!(61));
      R! (d, e, a, b, c, F4, K4, M!(62));
      R! (c, d, e, a, b, F4, K4, M!(63));
      R! (b, c, d, e, a, F4, K4, M!(64));
      R! (a, b, c, d, e, F4, K4, M!(65));
      R! (e, a, b, c, d, F4, K4, M!(66));
      R! (d, e, a, b, c, F4, K4, M!(67));
      R! (c, d, e, a, b, F4, K4, M!(68));
      R! (b, c, d, e, a, F4, K4, M!(69));
      R! (a, b, c, d, e, F4, K4, M!(70));
      R! (e, a, b, c, d, F4, K4, M!(71));
      R! (d, e, a, b, c, F4, K4, M!(72));
      R! (c, d, e, a, b, F4, K4, M!(73));
      R! (b, c, d, e, a, F4, K4, M!(74));
      R! (a, b, c, d, e, F4, K4, M!(75));
      R! (e, a, b, c, d, F4, K4, M!(76));
      R! (d, e, a, b, c, F4, K4, M!(77));
      R! (c, d, e, a, b, F4, K4, M!(78));
      R! (b, c, d, e, a, F4, K4, M!(79));

      /* C's `a = ctx->A += a;` is one statement.  */
      a = { (*ctx).A = (*ctx).A.wrapping_add (a); (*ctx).A };
      b = { (*ctx).B = (*ctx).B.wrapping_add (b); (*ctx).B };
      c = { (*ctx).C = (*ctx).C.wrapping_add (c); (*ctx).C };
      d = { (*ctx).D = (*ctx).D.wrapping_add (d); (*ctx).D };
      e = { (*ctx).E = (*ctx).E.wrapping_add (e); (*ctx).E };
    }
}
