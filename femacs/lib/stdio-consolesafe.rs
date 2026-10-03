/* fwrite, but robust against old mingw runtimes and against writing to a console.

   Copyright (C) 2024-2026 Free Software Foundation, Inc.

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

/* emacs/lib/stdio-consolesafe.c (@ 202 lines): the two functions at C 31-68 are live text and
   the `#if defined __MINGW32__ && __USE_MINGW_ANSI_STDIO` arm from C 70 on (with its
   `fseterr.h`/`vasprintf` fallbacks) is dead on this target.  C 22's <stdio.h>, C 26's
   <stdlib.h> and C 27's <string.h> are the stdio, stdlib and string mirrors; `ckd_mul` is the
   `overflowing_mul` pair (PORTING.md 2.6/P-15, the token-aliases family).  */
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/stdio.rs");
include!("include/stdlib.rs");
include!("include/string.rs");

/* Outputs N bytes starting at S to FP.
   These N bytes are known to be followed by a NUL.
   Finally frees the string at S.
   Returns the number of written bytes.  */

/* C 31-58's `static size_t workaround_fwrite0 (char *s, size_t n, FILE *fp)`.  */
unsafe fn workaround_fwrite0 (s: *mut c_char, mut n: usize, fp: *mut FILE) -> usize
{
  let mut ptr: *const c_char = s;
  /* Use fputs instead of fwrite, which is buggy in msvcrt.  */
  let mut written: usize = 0;
  while n > 0
    {
      let l: usize = strlen (ptr);   /* 0 <= l <= n */
      if l > 0
        {
          if fputs (ptr, fp) == EOF
            { break; }
          written += l;
          n -= l;
        }
      if n == 0
        { break; }
      if fputc (b'\0' as c_int, fp) == EOF
        { break; }
      written += 1;
      n -= 1;
      ptr = ptr.add (l + 1);
    }
  free (s as *mut c_void);
  return written;
}

/* C 60-68's `size_t _gl_consolesafe_fwrite (const void *ptr, size_t size, size_t nmemb,
   FILE *fp)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _gl_consolesafe_fwrite (ptr: *const c_void, size: usize,
                                                 nmemb: usize, fp: *mut FILE) -> usize
{
  /* C's `ckd_mul (&nbytes, size, nmemb) || nbytes == 0` is the overflowing_mul pair plus the
     zero test (PORTING.md 2.6).  */
  let (nbytes, overflow) = size.overflowing_mul (nmemb);
  if overflow || nbytes == 0
    /* Overflow, or nothing to do.  */
    { return 0; }
  let tmp: *mut c_char = malloc (nbytes + 1) as *mut c_char;
  if tmp.is_null ()
    { return 0; }
  memcpy (tmp as *mut c_void, ptr, nbytes);
  *tmp.add (nbytes) = b'\0' as c_char;
  let written: usize = workaround_fwrite0 (tmp, nbytes, fp);
  return written / size;
}
