// femacs: port of lib/binary-io.c @ fd7afced403.
//
// One of P-81's "extern-inline carriers": C 17 defines `BINARY_IO_INLINE` to
// `_GL_EXTERN_INLINE` before including `binary-io.h`, so the two `BINARY_IO_INLINE` bodies the
// header defines (`_gl_set_fd_mode` and `set_binary_mode`) are this *object's* external
// definitions.  The mirror keeps the bodies as `#[inline(always)] pub fn` (P-12, no symbol) and
// this unit re-exports them with C's names (P-81), which is why the object carries
// `_gl_set_fd_mode` and `set_binary_mode` (the same symbol set as
// `build/emacs-ref/lib/binary-io.o`, whose `nm -g` shows exactly those two).
//
// C 21-38's `#if defined __DJGPP__ || defined __EMX__` region (the `<unistd.h>` include and
// `set_binary_mode`'s `isatty`/`O_TEXT`/`_gl_set_fd_mode` body) is dead on this target - the
// evaluator decides it (`__DJGPP__`/`__EMX__` are not defined by the frozen compiler, so the
// `#if` is false; the inventory's function list for this file is empty) - and is recorded in
// docs/ports/binary-io.md rather than transcribed (P-08).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Binary mode I/O.
   Copyright 2017-2026 Free Software Foundation, Inc.

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

use std::ffi::{c_char, c_int, c_uint};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/binary-io.rs");

/* lib/binary-io.c 17: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const BINARY_IO_INLINE: () = _GL_EXTERN_INLINE;

/* The two `BINARY_IO_INLINE` definitions the header gives this object.  `super::` because the
   module's own item would shadow the mirror's and recurse (the byteswap.rs shape).  */
mod binary_io_unit {
  use super::*;

  #[unsafe(export_name = "_gl_set_fd_mode")]
  pub unsafe extern "C" fn _gl_set_fd_mode (fd: c_int, mode: c_int) -> c_int
  {
    return super::_gl_set_fd_mode (fd, mode);
  }

  #[unsafe(export_name = "set_binary_mode")]
  pub unsafe extern "C" fn set_binary_mode (fd: c_int, mode: c_int) -> c_int
  {
    return super::set_binary_mode (fd, mode);
  }
}
