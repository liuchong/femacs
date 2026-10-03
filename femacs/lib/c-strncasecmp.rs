/* c-strncasecmp.c -- case insensitive string comparator in C locale
   Copyright (C) 1998-1999, 2005-2006, 2009-2026 Free Software Foundation, Inc.

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
// femacs: port of lib/c-strncasecmp.c @ fd7afced403 (GNU Emacs 32.0.50).  The same shape as
// femacs/lib/c-strcasecmp.rs: blind file, `UCHAR_MAX <= INT_MAX` live, C's `else` arm
// carried so the checker sees `_GL_CMP`, and C 40's `--n` written as the wrapping step C
// performs.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");   // _GL_CMP
include!("include/limits.in.rs");       // <limits.h> (UCHAR_MAX, INT_MAX)
include!("include/c-ctype.rs");         // "c-ctype.h" (c_tolower)


/* int c_strncasecmp (const char *s1, const char *s2, size_t n)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_strncasecmp (s1: *const c_char, s2: *const c_char, mut n: usize)
  -> c_int
{
  let mut p1: *const u8 = s1 as *const u8;
  let mut p2: *const u8 = s2 as *const u8;

  if p1 == p2 || n == 0
    { return 0; }

  loop
    {
      let c1: u8 = c_tolower (*p1 as c_int) as u8;
      let c2: u8 = c_tolower (*p2 as c_int) as u8;

      n = n.wrapping_sub (1);
      if n == 0 || c1 == 0 || c1 != c2
        {
          if UCHAR_MAX <= INT_MAX
            { return c1 as c_int - c2 as c_int; }
          else
            /* On machines where 'char' and 'int' are types of the same size,
               the difference of two 'unsigned char' values - including the
               sign bit - doesn't fit in an 'int'.  */
            { return _GL_CMP! (c1, c2); }
        }
      p1 = p1.add (1);
      p2 = p2.add (1);
    }
}
