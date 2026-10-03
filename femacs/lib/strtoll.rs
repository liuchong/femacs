// femacs: port of lib/strtoll.c @ fd7afced403.
//
// C 19-21: `#define QUAD 1` + `#include <strtol.c>` - the QUAD instantiation of the template
// in femacs/lib/strtol.rs; provenance and the dead `_LIBC` block: docs/ports/strtoll.md.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_mut, unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Function to parse a 'long long int' from text.
   Copyright (C) 1995-1997, 1999, 2001, 2009-2026 Free Software Foundation,
   Inc.
   This file is part of the GNU C Library.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

include!("strtol.rs");

// C 19: `#define QUAD 1` - the `#ifdef QUAD` arm (`long long`, `LLONG_*`/`ULLONG_MAX`).
pub const QUAD: c_int = 1;

// C 21: `#include <strtol.c>` - the `QUAD == 1` instantiation.
strtol_body! (strtoll, "strtoll", c_longlong, c_ulonglong, c_longlong::MIN, c_longlong::MAX,
              c_ulonglong::MAX);
