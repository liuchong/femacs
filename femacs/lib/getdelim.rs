// femacs: port of lib/getdelim.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// The file is one function plus one dead-arm helper.  C 28-42's three arms select the
// locking behaviour: `USE_UNLOCKED_IO`'s `GNULIB_STDIO_SINGLE_THREAD` is not defined, and
// the frozen configuration has neither `HAVE_FLOCKFILE` nor `HAVE_FUNLOCKFILE`, so the
// `#elif` arm is the live text (`flockfile`/`funlockfile` become no-ops and
// `getc_maybe_unlocked (fp)` is `getc (fp)`); the C 29-32 `USE_UNLOCKED_IO` arm and the
// `getc_unlocked` spelling are the non-frozen sides.  C 47-55's `alloc_failed` is empty on
// this target (its only statement is the Windows arm's `errno = ENOMEM`).
//
// Deviations (docs/ports/getdelim.md): `_GL_ARG_NONNULL` is a syntax-only attribute
// (docs/ports/excluded-macros.txt), the two `goto unlock_return` jumps are the labelled
// block `'unlock_return` (PORTING.md 2.2), `ssize_t result;` is zero-initialized
// (PORTING.md 1.5), and `getc`/`EOF`/`SSIZE_MAX` are declared below with their C
// prototypes (the md5.rs shape) because the stdio/limits mirrors do not carry them yet.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Ported from glibc by Simon Josefsson.  */

/* Don't use __attribute__ __nonnull__ in this compilation unit.  Otherwise gcc
   optimizes away the lineptr == NULL || n == NULL || fp == NULL tests below.  */
/* #define _GL_ARG_NONNULL(params)  — no Rust counterpart
   (docs/ports/excluded-macros.txt).  */

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/sys_types.in.rs");   // ssize_t (and time_t family)
include!("include/stdint.in.rs");
include!("include/stdlib.rs");
include!("include/stdio.rs");
include!("include/errno.rs");

/* The three entries this unit needs beyond the mirrors (the md5.rs shape).  */
unsafe extern "C" {
  /* int getc (FILE *stream);  */
  pub fn getc (stream: *mut FILE) -> c_int;
}
/* <stdio.h>'s `#define EOF (-1)`.  */
/* <sys/types.h>'s `ssize_t` (the sys_types mirror in this build spells it
   through its own block, so the port repeats the alias the C header gives).  */
pub type ssize_t = isize;
/* <limits.h>'s `SSIZE_MAX` (the regex mirror spells its own; the port keeps this one local
   so that no other mirror has to change).  */
pub const SSIZE_MAX: isize = isize::MAX;

/* C 34-42's live `#elif` arm.  */
/* # define flockfile(x) ((void) 0)  */
macro_rules! flockfile {
    ($x:expr) => { { let _ = ($x, 0); } };
}
/* # define funlockfile(x) ((void) 0)  */
macro_rules! funlockfile {
    ($x:expr) => { { let _ = ($x, 0); } };
}
/* # define getc_maybe_unlocked(fp)        getc(fp)  */
macro_rules! getc_maybe_unlocked {
    ($fp:expr) => { getc ($fp) };
}

unsafe extern "C" fn alloc_failed ()
{
  /* C 49-53's `#if defined _WIN32 && ! defined __CYGWIN__` arm, kept as text (P-51;
     dead: this is a darwin-arm64 target).  */
  #[cfg (feature = "windows_native")]
  {
    /* Avoid errno problem without using the realloc module; see:
       https://lists.gnu.org/r/bug-gnulib/2016-08/msg00025.html  */
    *errno () = ENOMEM;
  }
}

/* Read up to (and including) a DELIMITER from FP into *LINEPTR (and
   NUL-terminate it).  *LINEPTR is a pointer returned from malloc (or
   NULL), pointing to *N characters of space.  It is realloc'ed as
   necessary.  Returns the number of characters read (not including
   the null terminator), or -1 on error or EOF.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn getdelim (lineptr: *mut *mut c_char, n: *mut usize,
                                   delimiter: c_int, fp: *mut FILE) -> ssize_t
{
  let mut result: ssize_t = 0;

  if lineptr.is_null () || n.is_null ()
      /* glibc already declares this function as __nonnull ((4)).
         Avoid a gcc warning "'nonnull' argument 'fp' compared to NULL".  */
      /* C 69-71's `#if !(__GLIBC__ >= 2)` guard: the frozen target is not glibc, so the
         comparison stays in the condition.  */
      || fp.is_null ()
    {
      *errno () = EINVAL;
      return -1;
    }

  flockfile! (fp);

  'unlock_return: {

  if *lineptr == std::ptr::null_mut () || *n == 0
    {
      *n = 120;
      let new_lineptr: *mut c_char = realloc (*lineptr as *mut c_void, *n) as *mut c_char;
      if new_lineptr == std::ptr::null_mut ()
        {
          alloc_failed ();
          result = -1;
          break 'unlock_return;
        }
      *lineptr = new_lineptr;
    }

  {
    let mut cur_len: usize = 0;
    loop
      {
        let mut i: c_int;

        i = getc_maybe_unlocked! (fp);
        if (i == EOF)
          {
            result = -1;
            break;
          }

        /* Make enough space for len+1 (for final NUL) bytes.  */
        if (cur_len + 1 >= *n)
          {
            let needed_max: usize =
              if (SSIZE_MAX as usize) < SIZE_MAX { SSIZE_MAX as usize + 1 } else { SIZE_MAX };
            let mut needed: usize = (2 * *n).wrapping_add (1);   /* Be generous. */

            if (needed_max < needed)
              { needed = needed_max; }
            if (cur_len + 1 >= needed)
              {
                result = -1;
                *errno () = EOVERFLOW;
                break 'unlock_return;
              }

            let new_lineptr: *mut c_char
              = realloc (*lineptr as *mut c_void, needed) as *mut c_char;
            if new_lineptr == std::ptr::null_mut ()
              {
                alloc_failed ();
                result = -1;
                break 'unlock_return;
              }

            *lineptr = new_lineptr;
            *n = needed;
          }

        *(*lineptr).offset (cur_len as isize) = i as c_char;
        cur_len += 1;

        if (i == delimiter)
          { break; }
      }
    *(*lineptr).offset (cur_len as isize) = 0;
    result = if cur_len != 0 { cur_len as ssize_t } else { result };
  }

  } /* unlock_return: */

  funlockfile! (fp); /* doesn't set errno */

  return result;
}
