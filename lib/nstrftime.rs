// femacs: port of lib/nstrftime.c @ fd7afced403.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;
include!("strftime.rs");
macro_rules! my_strftime { () => { nstrftime } }
strftime_body! (my_strftime, "nstrftime");
