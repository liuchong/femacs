// femacs: port of lib/rawmemchr.c @ fd7afced403.
/* emacs/lib/rawmemchr.c: C's `<config.h>`, `<string.h>`, `<limits.h>` and `<stdint.h>`
   includes have no counterpart beyond the mirrors below (PORTING.md 0.1).  rawmemchr.c is
   not one of the files in docs/baseline/config/cond-eval-verified.txt, so the condition
   table is not applied to it (P-66); each arm records what the frozen configuration says
   about its condition (docs/baseline/config/darwin-arm64.macros.tsv).  */
use std::ffi::{c_int, c_long, c_uint, c_ulong, c_void};
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");

/* C 23's `#if !HAVE_RAWMEMCHR`: the frozen configuration leaves HAVE_RAWMEMCHR undefined
   (macros.tsv 916), so the whole body below is live text and no `cfg` guard is needed.

   C 28-30's `#ifndef __has_feature / #define __has_feature(a) 0 / #endif` is the
   compiler-builtin fallback the aligned arm's condition uses; the builtin is absent from
   the frozen table as well, so C's own zero is what the arm evaluated
   (docs/ports/excluded-macros.txt carries the row).

   C 37's `#ifdef __CHERI_PURE_CAPABILITY__` arm is dead: the macro is not in the frozen
   table, and the arm calls builtins this target has not got.  Its text is recorded in
   docs/ports/rawmemchr.md.

   C 56's `#if ! (defined __SANITIZE_ADDRESS__ || __has_feature (address_sanitizer))` is
   live: __SANITIZE_ADDRESS__ is undefined and __has_feature is 0, so the negation holds.
   C 147's `#endif` closes it.  */

/* Find the first occurrence of C in S.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rawmemchr (mut s: *const c_void, c_in: c_int) -> *mut c_void
{
  /* C 37's `#ifdef __CHERI_PURE_CAPABILITY__` arm is dead in the frozen configuration
     (the macro is not in the table, and the arm calls builtins this target has not got);
     its text is recorded in docs/ports/rawmemchr.md.  */

  /* C 59's `typedef uintptr_t longword _GL_ATTRIBUTE_MAY_ALIAS;`: uintptr_t is the
     prelude's usize, and the attribute is a spelling with no Rust counterpart (the class
     of C's other `_GL_ATTRIBUTE_*` uses, docs/ports/excluded-macros.txt).  */
  type longword = usize;

  /* C 61's `static_assert (UINTPTR_WIDTH == UCHAR_WIDTH * sizeof (uintptr_t));` keeps its
     place in the body; the two macros are the configuration's own expressions
     (macros.tsv 1825, 1852), whose frozen values are recorded here the way a mirror
     records a configuration constant (PORTING.md 0.1).  */
  const UINTPTR_WIDTH: usize = 64;
  const UCHAR_WIDTH: usize = 8;
  const _: () = assert! (UINTPTR_WIDTH == UCHAR_WIDTH * size_of::<usize> ());

  let c: u8 = c_in as u8;

  {
    let mut char_ptr: *const u8 = s as *const u8;

    /* Handle the first few bytes by reading one byte at a time.
       Do this until CHAR_PTR is aligned on a natural longword boundary,
       as using alignof (longword) might be slower.  */
    while (char_ptr as usize) % size_of::<longword> () != 0
      {
        if *char_ptr == c
          { return char_ptr as *mut c_void; }
        char_ptr = char_ptr.add (1);
      }

    s = char_ptr as *const c_void;
  }

  {
    let mut longword_ptr: *const longword = s as *const longword;

    /* Compute auxiliary longword values:
       repeated_one is a value which has a 1 in every byte.
       repeated_c has c in every byte.  */
    let repeated_one: longword = (longword::MAX) / (UCHAR_MAX as longword);
    let repeated_c: longword = repeated_one * (c as longword);
    let repeated_hibit: longword =
      repeated_one * ((UCHAR_MAX as longword) / 2 + 1);

    /* Instead of the traditional loop which tests each byte, we will
       test a longword at a time.  The tricky part is testing if any of
       the bytes in the longword in question are equal to
       c.  We first use an xor with repeated_c.  This reduces the task
       to testing whether any of the bytes in longword1 is zero.

       We compute tmp =
         ((longword1 - repeated_one) & ~longword1) & (repeated_one * 0x80).
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

       The test whether any byte in longword1 is zero is equivalent
       to testing whether tmp is nonzero.

       This test can read beyond the end of a string, depending on where
       C_IN is encountered.  However, this is considered safe since the
       initialization phase ensured that the read will be aligned,
       therefore, the read will not cross page boundaries and will not
       cause a fault.  */

    loop
      {
        let longword1: longword = *longword_ptr ^ repeated_c;

        if ((((longword1 - repeated_one) & !longword1) & repeated_hibit) != 0)
          { break; }
        longword_ptr = longword_ptr.add (1);
      }

    /* At this point, we know that one of the sizeof (longword) bytes
       starting at char_ptr is == c.  If we knew endianness, we
       could determine the first such byte without any further memory
       accesses, just by looking at the tmp result from the last loop
       iteration.  However, the following simple and portable code does
       not attempt this potential optimization.  */

    s = longword_ptr as *const c_void;
  }

  {
    let mut char_ptr: *const u8 = s as *const u8;
    while *char_ptr != c
      { char_ptr = char_ptr.add (1); }
    return char_ptr as *mut c_void;
  }
}
