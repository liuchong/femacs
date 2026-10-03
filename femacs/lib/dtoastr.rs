// femacs: port of lib/dtoastr.c @ fd7afced403.
//
// C 18-19 instantiate gnulib's `ftoastr.c` template for `double`:
//
//     #define LENGTH 2
//     #include "ftoastr.c"
//
// so this file's own text is the `LENGTH` definition and the instantiation, per PORTING.md
// 10.13②'s inclusion-shim rule: the shared body lives in `lib/ftoastr.c`, which the port has
// not landed yet (`femacs/lib/ftoastr.rs` does not exist - it is the one template body
// `dtoastr.c`, `ldtoastr.c` and `nstrftime.c` share), so the instantiation's functions
// (`dtoastr`) are pending on it.  docs/ports/dtoastr.md records that dependency rather than
// inventing a body here; the inventory for this file is the macro alone.
//
// The other instantiation (`ldtoastr.c`'s `LENGTH 3`) is not compiled on this target.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Convert 'double' to accurate string.

   Copyright (C) 2010-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published
   by the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::c_int;

// C 18: `#define LENGTH 2` - the template's width selector for `double`.
pub const LENGTH: c_int = 2;

// C 19: `#include "ftoastr.c"` - the instantiation, pending on the shared body (see above).
