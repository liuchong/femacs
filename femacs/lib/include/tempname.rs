// femacs: WIP mirror of lib/tempname.h @ fd7afced403 (PORTING.md P-26).
//
// The header is the `GL_TEMPNAME_H` guard, the `GT_*` kind constants and the four
// prototypes.  C 26-33's `#ifdef __GT_FILE` arm is dead when the header is included by
// lib/tempname.c (the .c defines `__GT_FILE` only *after* C 20's `# include "tempname.h"`),
// so the `#else` arm - GT_FILE 0, GT_DIR 1, GT_NOCREATE 2 - is the live text, and its values
// are the ones lib/tempname.c's own `__GT_*` fallback carries (C 29-37's `#error` compares
// them).
//
// The prototypes live in `pub mod tempname_decls` + glob import (the stdio.rs shape): the
// unit that defines them has items of the same name, and an explicit item shadows a glob
// import (PORTING.md 0.1-10).  `try_tempname_len`'s `tryfunc` parameter is C's
// `int (*tryfunc) (char *, void *)`.

/* C 21-22's include guard `GL_TEMPNAME_H` (the idx.rs marker shape).  */
pub const GL_TEMPNAME_H: bool = true;

/* C 30-32 (the live arm): the KINDS gen_tempname takes.  */
pub const GT_FILE: c_int = 0;
pub const GT_DIR: c_int = 1;
pub const GT_NOCREATE: c_int = 2;

pub mod tempname_decls {
  use super::*;

  unsafe extern "C" {
    /* extern int gen_tempname (char *tmpl, int suffixlen, int flags, int kind);  */
    pub fn gen_tempname (tmpl: *mut c_char, suffixlen: c_int, flags: c_int, kind: c_int)
      -> c_int;
    /* extern int gen_tempname_len (char *tmpl, int suffixlen, int flags, int kind,
                                    size_t x_suffix_len);  */
    pub fn gen_tempname_len (tmpl: *mut c_char, suffixlen: c_int, flags: c_int, kind: c_int,
                             x_suffix_len: usize) -> c_int;
    /* extern int try_tempname (char *tmpl, int suffixlen, void *args,
                                int (*tryfunc) (char *, void *));  */
    pub fn try_tempname (tmpl: *mut c_char, suffixlen: c_int, args: *mut c_void,
                         tryfunc: unsafe extern "C" fn (*mut c_char, *mut c_void) -> c_int)
      -> c_int;
    /* extern int try_tempname_len (char *tmpl, int suffixlen, void *args,
                                    int (*tryfunc) (char *, void *), size_t x_suffix_len);  */
    pub fn try_tempname_len (tmpl: *mut c_char, suffixlen: c_int, args: *mut c_void,
                             tryfunc: unsafe extern "C" fn (*mut c_char, *mut c_void) -> c_int,
                             x_suffix_len: usize) -> c_int;
  }
}
use tempname_decls::*;
