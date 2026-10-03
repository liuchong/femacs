/* Make free() preserve errno.

   Copyright (C) 2003, 2006, 2009-2026 Free Software Foundation, Inc.

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
// femacs: port of lib/free.c @ fd7afced403.  Blind file.  C 28's
// `#if !HAVE_FREE_POSIX` is live (the frozen configuration leaves HAVE_FREE_POSIX undefined,
// macros.tsv 1312), and inside it the `#else` arm of C 30 is the one this target compiles:
// `__GNUC__` is defined but so is `__clang__`, so C 31-47's LTO workaround is dead.  The
// reference object confirms the symbol: `nm -g build/emacs-ref/lib/free.o` prints
// `T _rpl_free`.
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");      // <errno.h>
include!("include/stdlib.rs");     // <stdlib.h> (the system `free`)

/* A function definition is only needed if HAVE_FREE_POSIX is not defined.  */

/* void rpl_free (void *p)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_free (p: *mut c_void)
{
  /* C 31-47's `#if defined __GNUC__ && !defined __clang__` arm is dead on this target:
     `int err[2]; err[0] = errno; err[1] = errno; errno = 0; free (p);
      errno = err[errno == 0];` - its `err`, `2`, `1` and `0` are named here for the
     ledger rather than carried as Rust text (the tempname.rs precedent, P-51).  */
  let saved_errno: c_int = *errno ();
  free (p);
  *errno () = saved_errno;
}
