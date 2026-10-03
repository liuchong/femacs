// femacs: port of lib/filevercmp.h @ fd7afced403 .
//
// Two prototypes.  C 22-24's `#if !_GL_CONFIG_H_INCLUDED / #error …` is the generated config
// header's marker (the md5.rs shape); `_GL_ATTRIBUTE_PURE` is a syntax-only attribute
// (docs/ports/excluded-macros.txt), so the declarations carry no qualifier.
//
// The declarations live in a module with a glob import, because the unit that defines them
// (filevercmp.c) must not collide with them (the P-19/P-21/P-49 shape, as in
// include/mini-gmp.rs).

/* C 22-24's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.  */
const _: () = assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

pub mod decls {
  use super::*;

  unsafe extern "C" {
    /* int filevercmp (char const *a, char const *b) _GL_ATTRIBUTE_PURE;  */
    pub fn filevercmp (a: *const c_char, b: *const c_char) -> c_int;
    /* int filenvercmp (char const *a, ptrdiff_t alen, char const *b, ptrdiff_t blen)
         _GL_ATTRIBUTE_PURE;  */
    pub fn filenvercmp (a: *const c_char, alen: isize, b: *const c_char, blen: isize)
      -> c_int;
  }
}
use decls::*;

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const FILEVERCMP_H: bool = true;
