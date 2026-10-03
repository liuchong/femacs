/* Test whether a file is a symbolic link.
   Copyright (C) 2025-2026 Free Software Foundation, Inc.

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
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong,
                c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/issymlink.rs");

/* lib/issymlink.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_ISSYMLINK_INLINE: () = _GL_EXTERN_INLINE;

/* lib/issymlink.c is a gnulib `_GL_*_INLINE` wrapper: its C text is the license, the
   `#define <qualifier> _GL_EXTERN_INLINE` line and the includes, i.e. the C99 inline model's
   "this translation unit emits the external definition" file.  The body lives in the header
   (mirrored as `lib/include/issymlink.rs`) and C's object exports the symbol below; Rust's
   mirror keeps the body as an inline function, which emits nothing, so the unit re-exports it
   under C's own name (PORTING.md P-81).  */
mod issymlink_unit {
  use super::*;

  #[unsafe(export_name = "issymlink")]
  pub unsafe extern "C" fn issymlink (filename: *const c_char) -> c_int
  {
    return issymlink (filename);
  }
}
