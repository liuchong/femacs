/* malloc() function that is glibc compatible.

   Copyright (C) 1997-1998, 2006-2007, 2009-2026 Free Software Foundation, Inc.

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

/* written by Jim Meyering and Bruno Haible */

/* emacs/lib/malloc.c: C 19-25's `#define _GL_USE_STDLIB_ALLOC 1` (before <config.h>) makes the
   system <stdlib.h> declare `malloc` unrenamed, which is what lets the replacement below call
   the system one; the port keeps the macro as the constant the ledger compares and declares
   the system entry itself.  C 27's `<errno.h>` and C 28's `<stdckdint.h>` are only used by the
   dead arms (docs/ports/macro-body-notes.tsv).  */
use std::ffi::{c_int, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");

pub const _GL_USE_STDLIB_ALLOC: c_int = 1;

/* C's `<stdlib.h>`: the system allocator `_GL_USE_STDLIB_ALLOC` selected.  */
unsafe extern "C" { pub fn malloc (n: usize) -> *mut c_void; }

/* Allocate an N-byte block of memory from the heap, even if N is 0.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_malloc (mut n: usize) -> *mut c_void
{
  /* C 30-34's `#if !HAVE_MALLOC_0_NONNULL` arm is dead in the frozen configuration
     (docs/baseline/config/darwin-arm64.macros.tsv 855 is 1); its text is kept under the
     feature that names the non-frozen side (P-51).  */
  #[cfg (feature = "no_malloc_0_nonnull")]
  {
    if n == 0
      { n = 1; }
  }

  /* C 41-47's `#if !HAVE_MALLOC_PTRDIFF` arm (macros.tsv 859 is 1).  */
  #[cfg (feature = "no_malloc_ptrdiff")]
  {
    let signed_n: isize;
    let (v, overflowed) = n.overflowing_add (0);
    signed_n = v as isize;
    if overflowed
      {
	*errno () = ENOMEM;
	return std::ptr::null_mut ();
      }
  }

  let result: *mut c_void = malloc (n);

  /* C 51-54's `#if !HAVE_MALLOC_POSIX` arm (macros.tsv 858 is 1).  */
  #[cfg (feature = "no_malloc_posix")]
  {
    if result.is_null ()
      { *errno () = ENOMEM; }
  }

  return result;
}
