// femacs: mirror of lib/save-cwd.h @ fd7afced403 ; PORTING.md 0.1.
//
// The reference header is `struct saved_cwd` and three prototypes; the definitions live in
// `femacs/lib/save-cwd.rs` (the reference's trimmed variant of the gnulib module, which drops
// the getcwd fallback).  `name` is a `char *` the C only ever sets to NULL.

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const SAVE_CWD_H: c_int = 1;

/// `struct saved_cwd { int desc; char *name; };`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct saved_cwd {
  pub desc: c_int,
  pub name: *mut c_char,
}

/// C: int save_cwd (struct saved_cwd *cwd)
/// C: int restore_cwd (const struct saved_cwd *cwd)
/// C: void free_cwd (struct saved_cwd *cwd)
/* The three prototypes live in a whole-import module: `femacs/lib/save-cwd.rs` defines the
   same three names, and a definition plus a same-named declaration in one namespace is E0428,
   while a whole-module `use` is the lowest-priority binding the definition shadows
   (PORTING.md 0.1-10, the lisp.rs::other_units shape).  */
pub mod save_cwd_decls {
  use super::*;

  unsafe extern "C" {
    pub fn save_cwd (cwd: *mut saved_cwd) -> c_int;
    pub fn restore_cwd (cwd: *const saved_cwd) -> c_int;
    pub fn free_cwd (cwd: *mut saved_cwd);
  }
}
use save_cwd_decls::*;
