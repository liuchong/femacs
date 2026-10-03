/* stdc_count_ones_* functions.
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
include!("include/stdbit.rs");
include!("include/limits.in.rs");
/* lib/stdc_count_ones.c's dead `#if 1500 <= _MSC_VER` block declares this object.  */
#[cfg(feature = "msvc")] #[unsafe(no_mangle)]
pub static mut _gl_stdbit_popcount_support: c_schar = 0;

/* lib/stdc_count_ones.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_STDC_COUNT_ONES_INLINE: () = _GL_EXTERN_INLINE;

/* lib/stdc_count_ones.c is a gnulib `_GL_*_INLINE` wrapper: the bodies live in <stdbit.h>
   (mirrored as lib/include/stdbit.rs) and C's object exports the five typed entry points, so
   the unit re-exports them (PORTING.md P-81).  */
mod stdc_count_ones_unit {
  use super::*;

  #[unsafe(export_name = "stdc_count_ones_ui")]
  pub unsafe extern "C" fn stdc_count_ones_ui (n: c_uint) -> c_uint
  {
    return stdc_count_ones_ui (n);
  }

  #[unsafe(export_name = "stdc_count_ones_uc")]
  pub unsafe extern "C" fn stdc_count_ones_uc (n: c_uchar) -> c_uint
  {
    return stdc_count_ones_uc (n);
  }

  #[unsafe(export_name = "stdc_count_ones_us")]
  pub unsafe extern "C" fn stdc_count_ones_us (n: c_ushort) -> c_uint
  {
    return stdc_count_ones_us (n);
  }

  #[unsafe(export_name = "stdc_count_ones_ul")]
  pub unsafe extern "C" fn stdc_count_ones_ul (n: c_ulong) -> c_uint
  {
    return stdc_count_ones_ul (n);
  }

  #[unsafe(export_name = "stdc_count_ones_ull")]
  pub unsafe extern "C" fn stdc_count_ones_ull (n: c_ulonglong) -> c_uint
  {
    return stdc_count_ones_ull (n);
  }
}
