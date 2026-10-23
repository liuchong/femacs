// femacs: port of oldXMenu/insque.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The two non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2), and
// the file-local `struct qelem` is a Rust record under C's own name and field names (PORTING.md 1.3).
//
// **The file gate is `feature = "have_x_windows"`** (P-86 clause 8).  The reference text carries no
// macro of its own for this file: `oldXMenu/Makefile.in`'s `OBJS` block lists the object
// unconditionally, and it is `configure.ac` 7287-7295 (`HAVE_X11 = yes`, `HAVE_GTK != yes`,
// `USE_X_TOOLKIT = none`) that decides whether the library is built at all.  `oldXMenu/Internal.rs`
// 8-16 records the same reading; C 17-20's own comment (the `emacs_mumble` renaming note) is carried
// below as prose, because it is the reason these two functions exist at all.
//
// `HAVE_X11`, `HAVE_X_WINDOWS` and `USE_X_TOOLKIT` are `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv`, so the frozen configuration compiles not one line
// of this unit and `xtask build-tu` leaves the empty object.  This file carries no `#if` of its own,
// so there is no second arm to sweep.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_mut, unused_parens,
         unused_comparisons, clippy::all)]

/* C 1-15: the file's opening block - not transcribed (PORTING.md 0.1-19; the ledger's
   `## C line ranges` records the range).  */

/* C 17-20: "This file implements the emacs_insque and emacs_remque functions, clones of the insque
   and remque functions of BSD.  They and all their callers have been renamed to emacs_mumble to
   allow us to include this file in the menu library on all systems."  */

#[cfg (feature = "have_x_windows")]
#[macro_use]
mod xmenu_int {
  use super::*;
  include!("include/XMenuInt.rs");             // C 22: `#include "XMenuInt.h"`
}
#[cfg (feature = "have_x_windows")]
use xmenu_int::*;

/* The C fixed-width spellings.  Every unit that includes `lisp.h` gets them from that mirror's
   crate-root `use std::ffi::{…}`; XMenu's headers include no `lisp.h`, so this unit brings the
   whole set itself, the shape `oldXMenu/Internal.rs` 63-66 uses - and the `include!`d mirrors
   below (`xlib.rs`, `XMenu.rs`, `XMenuInt.rs`, `X10.rs`) read these names at the crate root.  */
#[cfg (feature = "have_x_windows")]
use std::ffi::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};

/// C 24-28: `struct qelem { struct qelem *q_forw; struct qelem *q_back; char q_data[1]; };` -
/// C's file-local tag, so the port declares it in this unit under C's own name and field names
/// (PORTING.md 1.3).  `char q_data[1]` is an array of length one, spelled with the constant.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
pub struct qelem
{
  pub q_forw: *mut qelem,
  pub q_back: *mut qelem,
  pub q_data: [c_char; 1],
}

/// C 30-43: `void emacs_insque (void *velem, void *vprev) { struct qelem *elem = velem;
/// struct qelem *prev = vprev; struct qelem *next = prev->q_forw; prev->q_forw = elem;
/// if (next) next->q_back = elem; elem->q_forw = next; elem->q_back = prev; }`.  C's
/// `void *` to `struct qelem *` conversions are implicit; Rust spells them as the casts C's
/// assignment performs (PORTING.md 1.4), and C's `if (next)` is the null test it is.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_insque (velem: *mut c_void, vprev: *mut c_void)
{
  let elem: *mut qelem = velem as *mut qelem;
  let prev: *mut qelem = vprev as *mut qelem;
  let next: *mut qelem = (*prev).q_forw;
  (*prev).q_forw = elem;
  if !next.is_null ()
    {
      (*next).q_back = elem;
    }
  (*elem).q_forw = next;
  (*elem).q_back = prev;
}

/// C 45-57: `void emacs_remque (void *velem) { struct qelem *elem = velem;
/// struct qelem *next = elem->q_forw; struct qelem *prev = elem->q_back; if (next)
/// next->q_back = prev; if (prev) prev->q_forw = next; }`
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_remque (velem: *mut c_void)
{
  let elem: *mut qelem = velem as *mut qelem;
  let next: *mut qelem = (*elem).q_forw;
  let prev: *mut qelem = (*elem).q_back;
  if !next.is_null ()
    {
      (*next).q_back = prev;
    }
  if !prev.is_null ()
    {
      (*prev).q_forw = next;
    }
}
