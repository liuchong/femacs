/* Copyright (C) 1991-1994, 1996-1998, 2000, 2004, 2007-2026 Free Software
   Foundation, Inc.
   This file is part of the GNU C Library.

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
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/memmem.c @ fd7afced403 .  `_LIBC` is undefined, so
// C 20-22's `<config.h>` include is live; the file is in cond-eval-verified.txt, so its
// conditions are the frozen configuration's.  C 27-29's three lines are the port's own shape:
// `RETURN_TYPE` and `AVAILABLE` are `macro_rules!` *before* the `include!` of the header
// mirror, because the header's functions are written in terms of them and Rust's macros are
// text-scoped (PORTING.md 0.1-5/P-57) — exactly C's requirement that they be defined before
// C 29.  The reference object confirms the symbol: `nm build/emacs-ref/lib/memmem.o` prints
// `T _rpl_memmem` (gnulib's generated <string.h> renames `memmem` under REPLACE_MEMMEM), so the
// port keeps C's name and emits the real symbol with `#[unsafe(export_name = "rpl_memmem")]`
// (PORTING.md 10.13①, the mempcpy precedent).
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");   // <limits.h> (CHAR_BIT, for the header's shift table)
include!("include/stdint.in.rs");   // <stdint.h> (SIZE_MAX, for the header's sentinel)
include!("include/string.rs");      // <string.h> (`memchr`, `memcmp`)

/* C 27's `# define RETURN_TYPE void *`.  */
macro_rules! RETURN_TYPE {
    () => { *mut c_void };
}

/* C 28's `# define AVAILABLE(h, h_l, j, n_l) ((j) <= (h_l) - (n_l))`.  */
macro_rules! AVAILABLE {
    ($h:expr, $h_l:expr, $j:expr, $n_l:expr) => { $j <= $h_l - $n_l };
}

/* PORTING.md 2.6: `__builtin_expect (cond, expected)` is kept as the branch-hint helper the C
   spells; the hint itself has no Rust counterpart (the deviation is in docs/ports/memmem.md),
   and the expression's value is what C's macro yields.  */
macro_rules! __builtin_expect {
    ($cond:expr, $expected:expr) => { $cond };
}

include!("include/str-two-way.rs");

/* Return the first occurrence of NEEDLE in HAYSTACK.  Return HAYSTACK
   if NEEDLE_LEN is 0, otherwise NULL if NEEDLE is not found in
   HAYSTACK.  */
#[unsafe(export_name = "rpl_memmem")]
pub unsafe extern "C" fn memmem (haystack_start: *const c_void, mut haystack_len: usize,
                                 needle_start: *const c_void, needle_len: usize) -> *mut c_void
{
  /* Abstract memory is considered to be an array of 'unsigned char' values,
     not an array of 'char' values.  See ISO C 99 section 6.2.6.1.  */
  let mut haystack: *const u8 = haystack_start as *const u8;
  let needle: *const u8 = needle_start as *const u8;

  if needle_len == 0
    /* The first occurrence of the empty string is deemed to occur at
       the beginning of the string.  */
    { return haystack as *mut c_void; }

  /* Sanity check, otherwise the loop might search through the whole
     memory.  */
  if __builtin_expect! (haystack_len < needle_len, 0)
    { return std::ptr::null_mut (); }

  /* Use optimizations in memchr when possible, to reduce the search
     size of haystack using a linear algorithm with a smaller
     coefficient.  However, avoid memchr for long needles, since we
     can often achieve sublinear performance.  */
  if needle_len < LONG_NEEDLE_THRESHOLD as usize
    {
      haystack = memchr (haystack as *const c_void, *needle as c_int, haystack_len) as *const u8;
      if haystack.is_null () || __builtin_expect! (needle_len == 1, 0)
        { return haystack as *mut c_void; }
      haystack_len -= (haystack as isize - haystack_start as isize) as usize;
      if haystack_len < needle_len
        { return std::ptr::null_mut (); }
      return two_way_short_needle (haystack, haystack_len, needle, needle_len);
    }
  else
    { return two_way_long_needle (haystack, haystack_len, needle, needle_len); }
}

/* C 71's `#undef LONG_NEEDLE_THRESHOLD`: the port's constant belongs to the header mirror and
   has no textual scope to undo.  */
