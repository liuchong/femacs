// femacs: port of lib/stdbit.c @ fd7afced403 (gnulib): C's LGPL header, the macro
// `_GL_STDBIT_INLINE = _GL_EXTERN_INLINE` and two includes - the C99 inline model's
// external-definition unit (PORTING.md 10.6).  The header's functions are Rust items in
// lib/include/stdbit.rs, so the macro is what survives.

/* stdbit_inline functions.
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
include!("include/gnulib_config.rs");

/* lib/stdbit.c: the qualifier this translation unit uses (PORTING.md 10.6).  */
pub const _GL_STDBIT_INLINE: () = _GL_EXTERN_INLINE;
