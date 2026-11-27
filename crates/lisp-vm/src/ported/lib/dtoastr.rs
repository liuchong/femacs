// femacs: port of lib/dtoastr.c @ fd7afced403.
//
// The whole of C 18-19 is an inclusion shim:
//
//     #define LENGTH 2
//     #include "ftoastr.c"
//
// so this file is the instantiation of gnulib's `ftoastr.c` template for `double`, per
// PORTING.md 10.13②.  The shared body landed first in `lib/ftoastr.rs` as the prelude
// `macro_rules! ftoastr_body` (that file's standalone-arm record and the parameterised body),
// and this shim instantiates it once with the `LENGTH == 2` arm's bindings.  The reference
// object `build/emacs-ref/lib/dtoastr.o` exports `dtoastr`, which is the `$export` below.  The
// inner `#![allow(…)]` lives here, not in `ftoastr.rs`: an attribute inside an included file
// is E0753.  The `LENGTH == 3` arm (`ldtoastr.c`) is not compiled on this target.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::sys_common::*;

include!("ftoastr.rs");

// C 18: `#define LENGTH 2` - the template's width selector for `double`.
pub const LENGTH: c_int = 2;

// C 19: `#include "ftoastr.c"` - the `LENGTH == 2` instantiation.
ftoastr_body! (2, dtoastr, "dtoastr", c_double, c_double, DBL_DIG, DBL_MIN,
               _GL_DBL_PREC_BOUND, strtod);
