/* stdc_bit_width_* functions.
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

use std::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};

include!("include/gnulib_config.rs");
include!("include/stdint.in.rs");
include!("include/stdbit.rs");

/* lib/stdc_bit_width.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STDC_BIT_WIDTH_INLINE: () = _GL_EXTERN_INLINE;

/* lib/stdc_bit_width.c is a gnulib `_GL_*_INLINE` wrapper: the bodies live in <stdbit.h>
   (mirrored as lib/include/stdbit.rs) and C's object exports the functions below, so the
   unit re-exports them (PORTING.md P-81).  The call is spelled `super::` because the
   module's own item would otherwise shadow the mirror's and recurse.  */
mod stdc_bit_width_unit {
  use super::*;

  #[unsafe(export_name = "stdc_bit_width_uc")]
  pub unsafe extern "C" fn stdc_bit_width_uc (n: c_uchar) -> c_uint
  {
    return super::stdc_bit_width_uc (n);
  }

  #[unsafe(export_name = "stdc_bit_width_us")]
  pub unsafe extern "C" fn stdc_bit_width_us (n: c_ushort) -> c_uint
  {
    return super::stdc_bit_width_us (n);
  }

  #[unsafe(export_name = "stdc_bit_width_ui")]
  pub unsafe extern "C" fn stdc_bit_width_ui (n: c_uint) -> c_uint
  {
    return super::stdc_bit_width_ui (n);
  }

  #[unsafe(export_name = "stdc_bit_width_ul")]
  pub unsafe extern "C" fn stdc_bit_width_ul (n: c_ulong) -> c_uint
  {
    return super::stdc_bit_width_ul (n);
  }

  #[unsafe(export_name = "stdc_bit_width_ull")]
  pub unsafe extern "C" fn stdc_bit_width_ull (n: c_ulonglong) -> c_uint
  {
    return super::stdc_bit_width_ull (n);
  }
}
