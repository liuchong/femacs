// femacs: port of lib/strtoll.c @ fd7afced403.
//
// C 19-21: `#define QUAD 1` + `#include <strtol.c>` - the QUAD instantiation of the template
// in lib/strtol.rs; provenance and the dead `_LIBC` block: docs/ports/strtoll.md.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_mut, unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn)]

include!("strtol.rs");

// C 19: `#define QUAD 1` - the `#ifdef QUAD` arm (`long long`, `LLONG_*`/`ULLONG_MAX`).
pub const QUAD: c_int = 1;

// C 21: `#include <strtol.c>` - the `QUAD == 1` instantiation.
strtol_body! (strtoll, "strtoll", c_longlong, c_ulonglong, c_longlong::MIN, c_longlong::MAX,
              c_ulonglong::MAX);
