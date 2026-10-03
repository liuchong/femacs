/* Memory allocators with error handling.

   Copyright (C) 2020-2026 Free Software Foundation, Inc.

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

/* emacs/lib/allocator.c (@ 22 lines) is four lines of code in C: the `_GL_USE_STDLIB_ALLOC`
   marker, the allocator.h include, <stdlib.h>, and the one `struct allocator const
   stdlib_allocator = { malloc, realloc, free, NULL };` definition.  This is the unit
   docs/status.md 3.4 listed as blocked on the allocator.h mirror; that mirror is now in the
   tree (`femacs/lib/include/allocator.rs`), so the definition lands here.  C's `NULL` for
   `die` is Rust's `None`, and the three functions are the system entries of the stdlib mirror,
   which is what the C's `#define _GL_USE_STDLIB_ALLOC 1` selects.  */
use std::ffi::{c_char, c_double, c_int, c_long, c_ulong, c_void};
include!("include/gnulib_config.rs");
include!("include/stdlib.rs");
include!("include/allocator.rs");

/* C 19's `#define _GL_USE_STDLIB_ALLOC 1`: it makes the system <stdlib.h> declare `malloc`
   unrenamed, which is what lets the three initializers below be the system functions.  */
pub const _GL_USE_STDLIB_ALLOC: c_int = 1;

/* C 22's `struct allocator const stdlib_allocator = { malloc, realloc, free, NULL };`.  */
#[unsafe(no_mangle)]
pub static stdlib_allocator: allocator = allocator {
  allocate: Some (malloc),
  reallocate: Some (realloc),
  free: Some (free),
  die: None,
};
