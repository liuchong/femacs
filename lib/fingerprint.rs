// femacs: port of lib/fingerprint.c @ fd7afced403.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;
use std::ffi::c_uchar;

// [U1] config.rs provided by common

/* C 20's `#include "fingerprint.h"`: that header only declares
   `extern volatile unsigned char fingerprint[32];` - the object this very unit defines -
   so it has no Rust counterpart (the region-cache.h reading, PORTING.md 0.1-12): a mirror
   would declare the same name the definition below binds (E0428), and no ported unit
   includes it yet.  docs/ports/include-fingerprint.md is not created for the same reason.  */

/* C 33-66: `volatile unsigned char fingerprint[] = { … };`, the placeholder fingerprint a
   real build replaces (lib-src/make-fingerprint.c rewrites this file).  The reference
   object exports it as 32 bytes of data (`D _fingerprint`), which is what the definition
   below is; C's `volatile` qualifies every *access*, and a Rust reader spells that
   `read_volatile` at the access (no object-level qualifier exists).  */
#[used]
#[unsafe(no_mangle)]
pub static mut fingerprint: [c_uchar; 32] =
  [
   0xDE, 0x86, 0xBB, 0x99,
   0xFF, 0xF5, 0x46, 0x9A,
   0x9E, 0x3F, 0x9F, 0x5D,
   0x9A, 0xDF, 0xF0, 0x91,
   0xBD, 0xCD, 0xC1, 0xE8,
   0x0C, 0x16, 0x1E, 0xAF,
   0xB8, 0x6C, 0xE2, 0x2B,
   0xB1, 0x24, 0xCE, 0xB0,
  ];
