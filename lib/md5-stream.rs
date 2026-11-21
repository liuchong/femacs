// femacs: port of lib/md5-stream.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
/* Written by Simon Josefsson.  */

/* emacs/lib/md5-stream.c (@ 134 lines): C 29-31's `#if USE_UNLOCKED_IO` arm and C 33-35's
   `#if GNULIB_AF_ALG` arm are dead in the frozen configuration (neither macro is defined), and
   so is C 37-49's `#ifdef _LIBC` rename block (`_LIBC` is undefined, which is why the port's
   calls keep the source names `md5_init_ctx` and friends); the `#if GNULIB_AF_ALG` arm inside
   md5_stream's body needs "af_alg.h" and `afalg_stream`, which this target has not got, so its
   tokens are recorded in docs/ports/macro-body-notes.tsv (the gettimeofday.c Windows-arm
   treatment).  C 25's "md5.h" is the md5 mirror (which is where `struct md5_ctx`, the
   `md5_*` prototypes and `MD5_DIGEST_SIZE` come from), C 27's <stdlib.h> the in-unit
   `malloc`/`free` declarations and C 22's <stdio.h> the stdio mirror (`FILE`, `feof`,
   `ferror`, `fread`).  The two `goto process_partial_block` jumps become `break` out of a
   labelled block (PORTING.md 2.2).  */
use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/stdio.rs imported via common
// [U1] include/md5.rs imported via common

/* C 27's <stdlib.h>: the two entries the buffer handling needs (stdlib.in.h has no mirror
   yet, so this is the getgroups.c shape).  */
unsafe extern "C" {
  pub fn malloc (n: usize) -> *mut c_void;
  pub fn free (p: *mut c_void);
}

/* C 37-49's `#ifdef _LIBC` block, kept as text (P-51; dead: _LIBC is undefined, which is why
   the port's own calls keep the source names).  C 40's `# define WORDS_BIGENDIAN 1` is inside
   C 38-42's `#include <endian.h>` arm.  */
#[cfg (feature = "libc")]
pub const WORDS_BIGENDIAN: c_int = 1;
#[cfg (feature = "libc")]
macro_rules! md5_stream { () => { __md5_stream } }

/* C 51-54's `#define BLOCKSIZE 32768` and its `#if BLOCKSIZE % 64 != 0 / # error` guard.  */
pub const BLOCKSIZE: usize = 32768;
const _: () = core::assert! (BLOCKSIZE % 64 == 0, "invalid BLOCKSIZE");

/* Compute MD5 message digest for bytes read from STREAM.  The
   resulting message digest number will be written into the 16 bytes
   beginning at RESBLOCK.  */

/* C 59-131's `int md5_stream (FILE *stream, void *resblock)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_stream (stream: *mut FILE, resblock: *mut c_void) -> c_int
{
  /* C 61-66's `#if GNULIB_AF_ALG` arm, kept as text (P-51; dead: the macro is undefined).  Its
     `afalg_stream` call and `-EIO`/`MD5_DIGEST_SIZE` tokens are in
     docs/ports/macro-body-notes.tsv.  */

  let buffer: *mut c_char = malloc (BLOCKSIZE) as *mut c_char;
  if buffer.is_null ()
    { return 1; }

  let mut ctx: md5_ctx = std::mem::zeroed ();
  md5_init_ctx (&mut ctx);

  /* Iterate over full file contents.  */
  let mut sum: usize;
  'process_partial_block: {
    loop
      {
        /* We read the file in blocks of BLOCKSIZE bytes.  One call of the
           computation function processes the whole buffer so that with the
           next round of the loop another block can be read.  */
        sum = 0;

        /* Read block.  Take care for partial reads.  */
        loop
          {
            /* Either process a partial fread() from this loop,
               or the fread() in afalg_stream may have gotten EOF.
               We need to avoid a subsequent fread() as EOF may
               not be sticky.  For details of such systems, see:
               https://sourceware.org/PR1190  */
            if feof (stream) != 0
              { break 'process_partial_block; }

            let n: usize = fread (buffer.add (sum) as *mut c_void, 1, BLOCKSIZE - sum, stream);

            sum += n;

            if sum == BLOCKSIZE
              { break; }

            if n == 0
              {
                /* Check for the error flag IFF N == 0, so that we don't
                   exit the loop after a partial read due to e.g., EAGAIN
                   or EWOULDBLOCK.  */
                if ferror (stream) != 0
                  {
                    free (buffer as *mut c_void);
                    return 1;
                  }
                break 'process_partial_block;
              }
          }

        /* Process buffer with BLOCKSIZE bytes.  Note that
           BLOCKSIZE % 64 == 0
         */
        md5_process_block (buffer as *const c_void, BLOCKSIZE, &mut ctx);
      }
  }

  /* Process any remaining bytes.  */
  if sum > 0
    { md5_process_bytes (buffer as *const c_void, sum, &mut ctx); }

  /* Construct result in desired memory.  */
  md5_finish_ctx (&mut ctx, resblock);
  free (buffer as *mut c_void);
  return 0;
}
