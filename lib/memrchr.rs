// femacs: port of lib/memrchr.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
/* emacs/lib/memrchr.c: C's `#if defined _LIBC` / `#else` pair (C 25-30) is decided by the
   frozen configuration: _LIBC is not defined, so the live arm is `#include <config.h>` /
   `#define reg_char char`, whose two effects are the config mirror below and the `reg_char`
   alias the body uses.  C 32-33's `<string.h>` and `<limits.h>` have no counterpart beyond
   the limits mirror (PORTING.md 0.1).  C 35's `#undef __memrchr`, C 36-38's `#ifdef _LIBC`
   and C 156-158's `#ifdef weak_alias` arm are dead text (the ledger records them).
   C 40-42's `#ifndef weak_alias / #define __memrchr memrchr / #endif` is live: the frozen
   configuration has no weak_alias, so the function C 45-46 declares as `__memrchr` is
   emitted under the name `memrchr`, which is what gnulib's <string.h> declares and what the
   port's export_name carries (the mempcpy precedent, PORTING.md 10.13).  */
use std::ffi::{c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
// [U1] include/gnulib_config.rs imported via common
// [U1] include/limits.in.rs imported via common

/* C 29's `#define reg_char char`: the type the body spells `reg_char` is the unsigned byte
   the cast to `unsigned char` produces.  */
type reg_char = c_uchar;

/* Search no more than N bytes of S for C.  */
#[unsafe(export_name = "memrchr")]
pub unsafe extern "C" fn __memrchr (s: *const c_void, c_in: c_int, mut n: usize) -> *mut c_void
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* On 32-bit hardware, choosing longword to be a 32-bit unsigned
     long instead of a 64-bit uintmax_t tends to give better
     performance.  On 64-bit hardware, unsigned long is generally 64
     bits already.  Change this typedef to experiment with
     performance.  */
  type longword = c_ulong;

  let c: reg_char = c_in as c_uchar;

  let mut longword_ptr: *const longword;

  /* Handle the last few bytes by reading one byte at a time.
     Do this until CHAR_PTR is aligned on a longword boundary.  */
  {
    let mut char_ptr: *const u8 = (s as *const u8).add (n);
    while n > 0 && (char_ptr as usize) % size_of::<longword> () != 0
      {
        char_ptr = char_ptr.sub (1);
        if *char_ptr == c
          { return char_ptr as *mut c_void; }
        n -= 1;
      }

    longword_ptr = char_ptr as *const longword;
  }

  /* All these elucidatory comments refer to 4-byte longwords,
     but the theory applies equally well to any size longwords.  */
  {
    /* Compute auxiliary longword values:
       repeated_one is a value which has a 1 in every byte.
       repeated_c has c in every byte.  */
    let mut repeated_one: longword = 0x01010101;
    let mut repeated_c: longword = (c as longword) | ((c as longword) << 8);
    repeated_c |= repeated_c << 16;
    if (0xffffffffu32 as longword) < (longword::MAX)
      {
        repeated_one |= repeated_one << 31 << 1;
        repeated_c |= repeated_c << 31 << 1;
        if 8 < size_of::<longword> ()
          {
            let mut i: usize = 64;
            while i < size_of::<longword> () * 8
              {
                repeated_one |= repeated_one << i;
                repeated_c |= repeated_c << i;
                i *= 2;
              }
          }
      }

    /* Instead of the traditional loop which tests each byte, we will test a
       longword at a time.  The tricky part is testing if *any of the four*
       bytes in the longword in question are equal to c.  We first use an xor
       with repeated_c.  This reduces the task to testing whether *any of the
       four* bytes in longword1 is zero.

       We compute tmp =
         ((longword1 - repeated_one) & ~longword1) & (repeated_one << 7).
       That is, we perform the following operations:
         1. Subtract repeated_one.
         2. & ~longword1.
         3. & a mask consisting of 0x80 in every byte.
       Consider what happens in each byte:
         - If a byte of longword1 is zero, step 1 and 2 transform it into 0xff,
           and step 3 transforms it into 0x80.  A carry can also be propagated
           to more significant bytes.
         - If a byte of longword1 is nonzero, let its lowest 1 bit be at
           position k (0 <= k <= 7); so the lowest k bits are 0.  After step 1,
           the byte ends in a single bit of value 0 and k bits of value 1.
           After step 2, the result is just k bits of value 1: 2^k - 1.  After
           step 3, the result is 0.  And no carry is produced.
       So, if longword1 has only non-zero bytes, tmp is zero.
       Whereas if longword1 has a zero byte, call j the position of the least
       significant zero byte.  Then the result has a zero at positions 0, ...,
       j-1 and a 0x80 at position j.  We cannot predict the result at the more
       significant bytes (positions j+1..3), but it does not matter since we
       already have a non-zero bit at position 8*j+7.

     So, the test whether any byte in longword1 is zero is equivalent to
     testing whether tmp is nonzero.  */

    while n >= size_of::<longword> ()
      {
        longword_ptr = longword_ptr.sub (1);
        let longword1: longword = *longword_ptr ^ repeated_c;

        /* C's `longword1 - repeated_one` *is* a wrapping subtraction (the haszero trick), and
           PORTING.md 1.1 requires C's wrap-around to be spelled out: in a debug build Rust's
           `-` panics on underflow, which is what stopped the first `message` (status 3.617).  */
        if ((((longword1.wrapping_sub (repeated_one)) & !longword1)
             & (repeated_one << 7)) != 0)
          {
            longword_ptr = longword_ptr.add (1);
            break;
          }
        n -= size_of::<longword> ();
      }
  }

  {
    let mut char_ptr: *const u8 = longword_ptr as *const u8;

    /* At this point, we know that either n < sizeof (longword), or one of the
       sizeof (longword) bytes starting at char_ptr is == c.  On little-endian
       machines, we could determine the first such byte without any further
       memory accesses, just by looking at the tmp result from the last loop
       iteration.  But this does not work on big-endian machines.  Choose code
       that works in both cases.  */
    while n > 0
      {
        n -= 1;
        char_ptr = char_ptr.sub (1);
        if *char_ptr == c
          { return char_ptr as *mut c_void; }
      }
  }

  return std::ptr::null_mut ();

  }
}
