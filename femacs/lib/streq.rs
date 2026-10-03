/* streq function.
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

use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

include!("include/gnulib_config.rs");
include!("include/string.rs");

/* lib/streq.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STREQ_INLINE: () = _GL_EXTERN_INLINE;

/* lib/streq.c is a gnulib `_GL_*_INLINE` wrapper: the body lives in <string.h> (mirrored
   as lib/include/string.rs) and C's object exports it, so the unit re-exports the
   definition (PORTING.md P-81).  `super::` because the module's own item would shadow the
   mirror's and recurse.  */
mod streq_unit {
  use super::*;

  #[unsafe(export_name = "streq")]
  pub unsafe extern "C" fn streq (s1: *const c_char, s2: *const c_char) -> bool
  {
    return super::streq (s1, s2);
  }
}
