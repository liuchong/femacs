// femacs: port of lib/fpending.h @ fd7afced403 ; PORTING.md 0.1.
//
// The header is the config assertion, two includes and one declaration guarded by
// `#if !HAVE_DECL___FPENDING` (undefined on this target: macros.tsv 1288), so the
// declaration is live.  `HAVE_STDIO_EXT_H` is undefined too, so C 32-34's `<stdio_ext.h>`
// include is dead; `_GL_ATTRIBUTE_PURE` is an attribute the port drops (PORTING.md 0.3).
//
// The declaration lives in `pub mod fpending_decls` + glob import, because lib/fpending.c
// *defines* `__fpending` (E0428 with a crate-root declaration; PORTING.md 0.1-10, the
// stdio.rs shape).
include!("gnulib_config.rs");
include!("stdio.rs");

/* C 23-26's `#if !_GL_CONFIG_H_INCLUDED / # error "Please include config.h first." / #endif`.  */
const _: () = core::assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

pub mod fpending_decls {
  use super::*;

  unsafe extern "C" {
    /* size_t __fpending (FILE *);  */
    pub fn __fpending (fp: *mut FILE) -> usize;
  }
}
use fpending_decls::*;
