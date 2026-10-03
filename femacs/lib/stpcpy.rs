/* stpcpy.c -- copy a string and return pointer to end of new string
   Copyright (C) 1992, 1995, 1997-1998, 2006, 2009-2026 Free Software
   Foundation, Inc.

   NOTE: The canonical source of this file is maintained with the GNU C Library.
   Bugs can be reported to bug-glibc@prep.ai.mit.edu.

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
// femacs: port of lib/stpcpy.c @ fd7afced403 (GNU Emacs 32.0.50).  C 30-32's
// `#ifndef weak_alias / # define __stpcpy stpcpy / #endif` is the live arm (this target
// has no weak_alias), so the definition C writes as `__stpcpy` leaves the preprocessor as
// `stpcpy`: the port keeps C's source name on the item and emits the real symbol with
// `#[unsafe(export_name = ...)]` (PORTING.md 10.13, the mempcpy precedent).  C 26-28's
// `#ifdef _LIBC` arm and C 47-49's `#ifdef weak_alias` arm are dead.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/string.rs");     // <string.h>


/* C 30-32's rename macro, which the ledger compares (its body is C's `stpcpy`).  */
macro_rules! __stpcpy { () => { stpcpy }; }

/* Copy SRC to DEST, returning the address of the terminating '\0' in DEST.  */
#[unsafe(export_name = "stpcpy")]
pub unsafe extern "C" fn __stpcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char
{
  let mut d: *mut c_char = dest;
  let mut s: *const c_char = src;

  /* C 41-43's `do *d++ = *s; while (*s++ != '\0');`: the post-increments become the
     explicit steps C's `++` performs them in.  */
  loop
    {
      *d = *s;
      d = d.add (1);
      let c: c_char = *s;
      s = s.add (1);
      if ! (c != 0 as c_char)
        { break; }
    }

  return d.offset (-1);
}
