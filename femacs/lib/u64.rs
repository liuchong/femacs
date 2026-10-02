/* uint64_t-like operations that work even on hosts lacking uint64_t

   Copyright (C) 2012-2026 Free Software Foundation, Inc.

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
use std::ffi::c_int;
include!("include/gnulib_config.rs");
/* lib/u64.c's `typedef int dummy;` (a link-time marker in gnulib).  */
pub type dummy = c_int;

/* lib/u64.c: the qualifier this translation unit uses (PORTING.md 10.12).  */
pub const _GL_U64_INLINE: () = _GL_EXTERN_INLINE;
