/* stdc_leading_zeros_* functions.
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

/* lib/stdc_leading_zeros.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STDC_LEADING_ZEROS_INLINE: () = _GL_EXTERN_INLINE;

/* lib/stdc_leading_zeros.c is a gnulib `_GL_*_INLINE` wrapper: the bodies live in <stdbit.h>
   (mirrored as lib/include/stdbit.rs) and C's object exports the functions below, so the
   unit re-exports them (PORTING.md P-81).  The call is spelled `super::` because the
   module's own item would otherwise shadow the mirror's and recurse.  */
mod stdc_leading_zeros_unit {
  use super::*;

  #[unsafe(export_name = "_gl_stdbit_clz")]
  pub unsafe extern "C" fn _gl_stdbit_clz (n: c_uint) -> c_int
  {
    return super::_gl_stdbit_clz (n);
  }

  #[unsafe(export_name = "_gl_stdbit_clzl")]
  pub unsafe extern "C" fn _gl_stdbit_clzl (n: c_ulong) -> c_int
  {
    return super::_gl_stdbit_clzl (n);
  }

  #[unsafe(export_name = "_gl_stdbit_clzll")]
  pub unsafe extern "C" fn _gl_stdbit_clzll (n: c_ulonglong) -> c_int
  {
    return super::_gl_stdbit_clzll (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_ui")]
  pub unsafe extern "C" fn stdc_leading_zeros_ui (n: c_uint) -> c_uint
  {
    return super::stdc_leading_zeros_ui (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_uc")]
  pub unsafe extern "C" fn stdc_leading_zeros_uc (n: c_uchar) -> c_uint
  {
    return super::stdc_leading_zeros_uc (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_us")]
  pub unsafe extern "C" fn stdc_leading_zeros_us (n: c_ushort) -> c_uint
  {
    return super::stdc_leading_zeros_us (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_ul")]
  pub unsafe extern "C" fn stdc_leading_zeros_ul (n: c_ulong) -> c_uint
  {
    return super::stdc_leading_zeros_ul (n);
  }

  #[unsafe(export_name = "stdc_leading_zeros_ull")]
  pub unsafe extern "C" fn stdc_leading_zeros_ull (n: c_ulonglong) -> c_uint
  {
    return super::stdc_leading_zeros_ull (n);
  }
}
