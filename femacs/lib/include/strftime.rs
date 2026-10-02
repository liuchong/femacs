/* Declarations of functions for formatting time.

   Copyright (C) 2025-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

// femacs: port of lib/strftime.h @ fd7afced403 (gnulib; GNU Emacs 32.0.50).
//
// Header mirror (PORTING.md 0.1).  The header declares no macro and no type, only the two
// entry points over `timezone_t` (the time.h mirror) and `struct tm` (also the time.h
// mirror); its long `%N`/conversion documentation is the function comment of
// `nstrftime` below.  `nstrftime`/`c_nstrftime` are defined by lib/nstrftime.c, which is a
// separate unit (it `#include`s strftime.c with `my_strftime` renamed).

include!("time.in.rs");

unsafe extern "C" {
  /* ptrdiff_t nstrftime (char *restrict, size_t, char const *restrict,
                          struct tm const *restrict, timezone_t, int);  */
  pub fn nstrftime (s: *mut c_char, maxsize: usize, format: *const c_char, tp: *const tm,
                    tz: timezone_t, ns: c_int) -> isize;
  /* ptrdiff_t c_nstrftime (…same…);  */
  pub fn c_nstrftime (s: *mut c_char, maxsize: usize, format: *const c_char, tp: *const tm,
                      tz: timezone_t, ns: c_int) -> isize;
}
