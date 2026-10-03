// femacs: port of lib/md5.c @ fd7afced403.
/* Written by Ulrich Drepper <drepper@gnu.ai.mit.edu>, 1995.  */

/* emacs/lib/md5.c: C 22's `<config.h>` is the generated config mirror, C 28's `"md5.h"`
   is the header mirror (which carries `struct md5_ctx`, `SWAP`'s companion constants and
   the declarations), C 29-31's `<stdint.h>`/`<string.h>`/`<sys/types.h>` are the stdint
   mirror plus the `memcpy` declaration below (the mempcpy.rs shape), and C 49's
   `<byteswap.h>` is only needed by the big-endian `SWAP` arm.
   C 24-27's `#if HAVE_OPENSSL_MD5` arm is dead on the frozen target (the macro is not in
   the configuration, so the condition is 0) and C 33-46's `#ifdef _LIBC` arm is dead
   (`_LIBC` is undefined); both are kept as text under the features that name the
   non-frozen sides (P-51), and the six `#define`s' renames are recorded in
   docs/ports/excluded-macros.txt.  C 56's `#if ! HAVE_OPENSSL_MD5` is live (the macro is
   undefined, so the body below is C's gnulib implementation) and C 387's `#endif`
   closes it.  */
use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/stdint.in.rs");
include!("include/md5.rs");

/* C 31's `<string.h>`: the one entry the bodies call.  */
unsafe extern "C" {
  pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/* C 25-26's `#if HAVE_OPENSSL_MD5 / # define GL_OPENSSL_INLINE _GL_EXTERN_INLINE`: the
   OpenSSL build's qualifier, dead on this target and kept as text (P-51).  */
#[cfg (feature = "openssl_md5")]
pub const GL_OPENSSL_INLINE: () = _GL_EXTERN_INLINE;

/* C 34-36's `#ifdef _LIBC / # include <endian.h> / # if __BYTE_ORDER == __BIG_ENDIAN /
   #  define WORDS_BIGENDIAN 1` is dead (`_LIBC` is undefined); the name and value are
   kept under the feature the `SWAP` pair below uses (P-51).  */
#[cfg (feature = "words_bigendian")]
pub const WORDS_BIGENDIAN: c_int = 1;

/* C 49-54's `#include <byteswap.h>` and the `SWAP` pair: the `#ifdef WORDS_BIGENDIAN`
   arm is the one this target does not take, so its `bswap_32` (a glibc macro the frozen
   target has no declaration of) is kept as text under the feature naming the other side,
   and the live arm is C's `#else` body.  */
#[cfg (feature = "words_bigendian")]
unsafe extern "C" {
  pub fn bswap_32 (n: uint32_t) -> uint32_t;
}
#[cfg (feature = "words_bigendian")]
macro_rules! SWAP { ($n:expr) => { bswap_32 ($n) } }
#[cfg (not (feature = "words_bigendian"))]
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

/* Initialize structure containing state of computation.
   (RFC 1321, 3.3: Step 3)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_init_ctx (ctx: *mut md5_ctx)
{
  (*ctx).A = 0x67452301;
  (*ctx).B = 0xefcdab89;
  (*ctx).C = 0x98badcfe;
  (*ctx).D = 0x10325476;

  /* C's `ctx->total[0] = ctx->total[1] = 0;` is one statement: the right-hand assignment
     is evaluated first and its value is the left-hand one's source.  */
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

/* Put result from CTX in first 16 bytes following RESBUF.  The result
   must be in little endian byte order.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_read_ctx (ctx: *const md5_ctx, resbuf: *mut c_void) -> *mut c_void
{
  let r: *mut c_char = resbuf as *mut c_char;
  set_uint32 (r.add (0 * size_of::<uint32_t> ()), SWAP! ((*ctx).A));
  set_uint32 (r.add (1 * size_of::<uint32_t> ()), SWAP! ((*ctx).B));
  set_uint32 (r.add (2 * size_of::<uint32_t> ()), SWAP! ((*ctx).C));
  set_uint32 (r.add (3 * size_of::<uint32_t> ()), SWAP! ((*ctx).D));

  return resbuf;
}

/* Process the remaining bytes in the internal buffer and the usual
   prolog according to the standard and write the result to RESBUF.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_finish_ctx (ctx: *mut md5_ctx, resbuf: *mut c_void) -> *mut c_void
{
  /* Take yet unprocessed bytes into account.  */
  let bytes: uint32_t = (*ctx).buflen;
  let size: usize = if bytes < 56 { 64 / 4 } else { 64 * 2 / 4 };

  /* Now count remaining bytes.  */
  (*ctx).total[0] = (*ctx).total[0].wrapping_add (bytes);
  if (*ctx).total[0] < bytes
    { (*ctx).total[1] = (*ctx).total[1].wrapping_add (1); }

  /* Put the 64-bit file length in *bits* at the end of the buffer.  */
  (*ctx).buffer[size - 2] = SWAP! ((*ctx).total[0] << 3);
  (*ctx).buffer[size - 1] = SWAP! (((*ctx).total[1] << 3) | ((*ctx).total[0] >> 29));

  memcpy ((*ctx).buffer.as_mut_ptr ().add (bytes as usize) as *mut c_void,
          fillbuf.as_ptr () as *const c_void,
          (size - 2) * 4 - bytes as usize);

  /* Process last bytes.  */
  md5_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void, size * 4, ctx);

  return md5_read_ctx (ctx, resbuf);
}

/* Compute MD5 message digest for LEN bytes beginning at BUFFER.  The
   result is always in little endian byte order, so that a byte-wise
   output yields to the wanted ASCII representation of the message
   digest.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_buffer (buffer: *const c_char, len: usize, resblock: *mut c_void)
  -> *mut c_void
{
  /* C's `struct md5_ctx ctx;` is filled by md5_init_ctx below; a Rust local needs a value
     before its address can be taken (PORTING.md 1.5), so it starts zeroed.  */
  let mut ctx: md5_ctx = md5_ctx { A: 0, B: 0, C: 0, D: 0, total: [0, 0], buflen: 0,
                                   buffer: [0; 32] };

  /* Initialize the computation context.  */
  md5_init_ctx (&raw mut ctx);

  /* Process whole buffer but last len % 64 bytes.  */
  md5_process_bytes (buffer as *const c_void, len, &raw mut ctx);

  /* Put result in desired memory area.  */
  return md5_finish_ctx (&raw mut ctx, resblock);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_process_bytes (mut buffer: *const c_void, mut len: usize,
                                            ctx: *mut md5_ctx)
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
          md5_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void,
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
      /* C 178-179's `#if !(_STRING_ARCH_unaligned || _STRING_INLINE_unaligned)` arm is
         live for this target (neither macro is defined by the frozen configuration's
         system `<string.h>`), and C defines `UNALIGNED_P` right here, inside the block;
         Rust's `macro_rules!` are textual, so the definition stays inside the same block
         as its uses.  */
      macro_rules! UNALIGNED_P { ($p:expr) => { ($p as usize) % align_of::<uint32_t> () != 0 } }
      if UNALIGNED_P! (buffer)
        { while len > 64
          {
            md5_process_block (memcpy ((*ctx).buffer.as_mut_ptr () as *mut c_void, buffer, 64),
                               64, ctx);
            buffer = (buffer as *const c_char).add (64) as *const c_void;
            len -= 64;
          } }
      else
        {
          md5_process_block (buffer, len & !63, ctx);
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
          md5_process_block ((*ctx).buffer.as_mut_ptr () as *const c_void, 64, ctx);
          left_over -= 64;
          /* The regions in the following copy operation cannot overlap,
             because left_over <= 64.  */
          memcpy ((*ctx).buffer.as_mut_ptr () as *mut c_void,
                  (*ctx).buffer.as_mut_ptr ().add (16) as *const c_void, left_over);
        }
      (*ctx).buflen = left_over as uint32_t;
    }
}

/* These are the four functions used in the four steps of the MD5 algorithm
   and defined in the RFC 1321.  The first function is a little bit optimized
   (as found in Colin Plumbs public domain implementation).  */
/* #define FF(b, c, d) ((b & c) | (~b & d)) */
macro_rules! FF { ($b:expr, $c:expr, $d:expr) => { ($d ^ ($b & ($c ^ $d))) } }
macro_rules! FG { ($b:expr, $c:expr, $d:expr) => { FF! ($d, $b, $c) } }
macro_rules! FH { ($b:expr, $c:expr, $d:expr) => { ($b ^ $c ^ $d) } }
macro_rules! FI { ($b:expr, $c:expr, $d:expr) => { ($c ^ ($b | !$d)) } }

/* Process LEN bytes of BUFFER, accumulating context into CTX.
   It is assumed that LEN % 64 == 0.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_process_block (buffer: *const c_void, len: usize, ctx: *mut md5_ctx)
{
  let mut correct_words: [uint32_t; 16] = [0; 16];
  let mut words: *const uint32_t = buffer as *const uint32_t;
  let nwords: usize = len / size_of::<uint32_t> ();
  let endp: *const uint32_t = words.add (nwords);
  let mut A: uint32_t = (*ctx).A;
  let mut B: uint32_t = (*ctx).B;
  let mut C: uint32_t = (*ctx).C;
  let mut D: uint32_t = (*ctx).D;
  let lolen: uint32_t = len as uint32_t;

  /* First increment the byte count.  RFC 1321 specifies the possible
     length of the file up to 2^64 bits.  Here we only compute the
     number of bytes.  Do a double word increment.  */
  (*ctx).total[0] = (*ctx).total[0].wrapping_add (lolen);
  (*ctx).total[1] = (*ctx).total[1]
    .wrapping_add ((len >> 31 >> 1) as uint32_t)
    .wrapping_add (((*ctx).total[0] < lolen) as uint32_t);

  /* Process all bytes in the buffer with 64 bytes in each round of
     the loop.  */
  while words < endp
    {
      let mut cwp: *mut uint32_t = correct_words.as_mut_ptr ();
      let A_save: uint32_t = A;
      let B_save: uint32_t = B;
      let C_save: uint32_t = C;
      let D_save: uint32_t = D;

      /* First round: using the given function, the context and a constant
         the next context is computed.  Because the algorithms processing
         unit is a 32-bit word and it is determined to work on words in
         little endian byte order we perhaps have to change the byte order
         before the computation.  To reduce the work for the next steps
         we store the swapped words in the array CORRECT_WORDS.  */

      /* C defines CYCLIC at C 276, after the first OP; Rust's `macro_rules!` are
         textual, so the port defines it first (the bodies are C's).  */
      macro_rules! CYCLIC { ($w:ident, $s:expr) => { $w = ($w << $s) | ($w >> (32 - $s)) } }

      /* C 264-272 and C 310-317 define OP twice, with `#undef OP` between them: the first
         shape is round 1's, the second the remaining rounds'.  A `macro_rules!` cannot be
         redefined in one scope, so the two are one macro with C's two arms, dispatched by
         arity.  C's `cwp`, `words` and `correct_words` are the caller's locals, which a
         macro cannot name (hygiene), so they are passed as trailing arguments - the
         port's own deviation, recorded in docs/ports/md5.md.  `do { ... } while (0)` is a
         block, and C's unsigned `+=` is `wrapping_add`.  */
      macro_rules! OP {
        /* C 264-272.  */
        ($a:ident, $b:expr, $c:expr, $d:expr, $s:expr, $t:expr,
         $cwp:ident, $words:ident) => {
          {
            $a = $a.wrapping_add (FF! ($b, $c, $d)
                                  .wrapping_add ({ let w = SWAP! (*$words);
                                                   *$cwp = w;
                                                   $cwp = $cwp.add (1);
                                                   w })
                                  .wrapping_add ($t));
            $words = $words.add (1);
            CYCLIC! ($a, $s);
            $a = $a.wrapping_add ($b);
          }
        };
        /* C 310-317.  */
        ($f:ident, $a:ident, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $t:expr,
         $cw:ident) => {
          {
            $a = $a.wrapping_add ($f! ($b, $c, $d).wrapping_add ($cw[$k]).wrapping_add ($t));
            CYCLIC! ($a, $s);
            $a = $a.wrapping_add ($b);
          }
        };
      }

      /* Before we start, one word to the strange constants.
         They are defined in RFC 1321 as

         T[i] = (int) (4294967296.0 * fabs (sin (i))), i=1..64

         Here is an equivalent invocation using Perl:

         perl -e 'foreach(1..64){printf "0x%08x\n", int (4294967296 * abs (sin $_))}'
       */

      /* Round 1.  */
      OP! (A, B, C, D, 7, 0xd76aa478, cwp, words);
      OP! (D, A, B, C, 12, 0xe8c7b756, cwp, words);
      OP! (C, D, A, B, 17, 0x242070db, cwp, words);
      OP! (B, C, D, A, 22, 0xc1bdceee, cwp, words);
      OP! (A, B, C, D, 7, 0xf57c0faf, cwp, words);
      OP! (D, A, B, C, 12, 0x4787c62a, cwp, words);
      OP! (C, D, A, B, 17, 0xa8304613, cwp, words);
      OP! (B, C, D, A, 22, 0xfd469501, cwp, words);
      OP! (A, B, C, D, 7, 0x698098d8, cwp, words);
      OP! (D, A, B, C, 12, 0x8b44f7af, cwp, words);
      OP! (C, D, A, B, 17, 0xffff5bb1, cwp, words);
      OP! (B, C, D, A, 22, 0x895cd7be, cwp, words);
      OP! (A, B, C, D, 7, 0x6b901122, cwp, words);
      OP! (D, A, B, C, 12, 0xfd987193, cwp, words);
      OP! (C, D, A, B, 17, 0xa679438e, cwp, words);
      OP! (B, C, D, A, 22, 0x49b40821, cwp, words);

      /* For the second to fourth round we have the possibly swapped words
         in CORRECT_WORDS.  Redefine the macro to take an additional first
         argument specifying the function to use.  */

      /* Round 2.  */
      OP! (FG, A, B, C, D, 1, 5, 0xf61e2562, correct_words);
      OP! (FG, D, A, B, C, 6, 9, 0xc040b340, correct_words);
      OP! (FG, C, D, A, B, 11, 14, 0x265e5a51, correct_words);
      OP! (FG, B, C, D, A, 0, 20, 0xe9b6c7aa, correct_words);
      OP! (FG, A, B, C, D, 5, 5, 0xd62f105d, correct_words);
      OP! (FG, D, A, B, C, 10, 9, 0x02441453, correct_words);
      OP! (FG, C, D, A, B, 15, 14, 0xd8a1e681, correct_words);
      OP! (FG, B, C, D, A, 4, 20, 0xe7d3fbc8, correct_words);
      OP! (FG, A, B, C, D, 9, 5, 0x21e1cde6, correct_words);
      OP! (FG, D, A, B, C, 14, 9, 0xc33707d6, correct_words);
      OP! (FG, C, D, A, B, 3, 14, 0xf4d50d87, correct_words);
      OP! (FG, B, C, D, A, 8, 20, 0x455a14ed, correct_words);
      OP! (FG, A, B, C, D, 13, 5, 0xa9e3e905, correct_words);
      OP! (FG, D, A, B, C, 2, 9, 0xfcefa3f8, correct_words);
      OP! (FG, C, D, A, B, 7, 14, 0x676f02d9, correct_words);
      OP! (FG, B, C, D, A, 12, 20, 0x8d2a4c8a, correct_words);

      /* Round 3.  */
      OP! (FH, A, B, C, D, 5, 4, 0xfffa3942, correct_words);
      OP! (FH, D, A, B, C, 8, 11, 0x8771f681, correct_words);
      OP! (FH, C, D, A, B, 11, 16, 0x6d9d6122, correct_words);
      OP! (FH, B, C, D, A, 14, 23, 0xfde5380c, correct_words);
      OP! (FH, A, B, C, D, 1, 4, 0xa4beea44, correct_words);
      OP! (FH, D, A, B, C, 4, 11, 0x4bdecfa9, correct_words);
      OP! (FH, C, D, A, B, 7, 16, 0xf6bb4b60, correct_words);
      OP! (FH, B, C, D, A, 10, 23, 0xbebfbc70, correct_words);
      OP! (FH, A, B, C, D, 13, 4, 0x289b7ec6, correct_words);
      OP! (FH, D, A, B, C, 0, 11, 0xeaa127fa, correct_words);
      OP! (FH, C, D, A, B, 3, 16, 0xd4ef3085, correct_words);
      OP! (FH, B, C, D, A, 6, 23, 0x04881d05, correct_words);
      OP! (FH, A, B, C, D, 9, 4, 0xd9d4d039, correct_words);
      OP! (FH, D, A, B, C, 12, 11, 0xe6db99e5, correct_words);
      OP! (FH, C, D, A, B, 15, 16, 0x1fa27cf8, correct_words);
      OP! (FH, B, C, D, A, 2, 23, 0xc4ac5665, correct_words);

      /* Round 4.  */
      OP! (FI, A, B, C, D, 0, 6, 0xf4292244, correct_words);
      OP! (FI, D, A, B, C, 7, 10, 0x432aff97, correct_words);
      OP! (FI, C, D, A, B, 14, 15, 0xab9423a7, correct_words);
      OP! (FI, B, C, D, A, 5, 21, 0xfc93a039, correct_words);
      OP! (FI, A, B, C, D, 12, 6, 0x655b59c3, correct_words);
      OP! (FI, D, A, B, C, 3, 10, 0x8f0ccc92, correct_words);
      OP! (FI, C, D, A, B, 10, 15, 0xffeff47d, correct_words);
      OP! (FI, B, C, D, A, 1, 21, 0x85845dd1, correct_words);
      OP! (FI, A, B, C, D, 8, 6, 0x6fa87e4f, correct_words);
      OP! (FI, D, A, B, C, 15, 10, 0xfe2ce6e0, correct_words);
      OP! (FI, C, D, A, B, 6, 15, 0xa3014314, correct_words);
      OP! (FI, B, C, D, A, 13, 21, 0x4e0811a1, correct_words);
      OP! (FI, A, B, C, D, 4, 6, 0xf7537e82, correct_words);
      OP! (FI, D, A, B, C, 11, 10, 0xbd3af235, correct_words);
      OP! (FI, C, D, A, B, 2, 15, 0x2ad7d2bb, correct_words);
      OP! (FI, B, C, D, A, 9, 21, 0xeb86d391, correct_words);

      /* Add the starting values of the context.  */
      A = A.wrapping_add (A_save);
      B = B.wrapping_add (B_save);
      C = C.wrapping_add (C_save);
      D = D.wrapping_add (D_save);
    }

  /* Put checksum in context given as argument.  */
  (*ctx).A = A;
  (*ctx).B = B;
  (*ctx).C = C;
  (*ctx).D = D;
}
