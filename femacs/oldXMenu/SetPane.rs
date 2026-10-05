// femacs: port of oldXMenu/SetPane.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  C's non-`static` definition carries `#[unsafe(no_mangle)]` (PORTING.md 0.2).
//
// **The file gate is `feature = "have_x_windows"`** (P-86 clause 8).  The reference text carries no
// macro of its own for this file: `oldXMenu/Makefile.in`'s `OBJS` block lists the object
// unconditionally, and it is `configure.ac` 7287-7295 (`HAVE_X11 = yes`, `HAVE_GTK != yes`,
// `USE_X_TOOLKIT = none`) that decides whether the library is built at all.  `oldXMenu/Internal.rs`
// 8-16 records the same reading.
//
// `HAVE_X11`, `HAVE_X_WINDOWS` and `USE_X_TOOLKIT` are `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv`, so the frozen configuration compiles not one line
// of this unit and `xtask build-tu` leaves the empty object; this target has no X11 window system to
// run it against either.  This file carries no `#if` of its own, so there is no second arm to sweep.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_mut, unused_parens,
         unused_comparisons, clippy::all)]

/* C 1-32: the MIT notice and the file's own description - not transcribed (PORTING.md 0.1-19; the
   ledger's `## C line ranges` records the range).  */

#[cfg (feature = "have_x_windows")]
#[macro_use]
mod xmenu_int {
  use super::*;
  include!("include/XMenuInt.rs");             // C 34: `#include "XMenuInt.h"`
}
#[cfg (feature = "have_x_windows")]
use xmenu_int::*;

#[cfg (feature = "have_x_windows")]
include!("../src/include/xlib.rs");            // XMenuInt.h 46: `#include <X11/Xlib.h>`

/* The C fixed-width spellings.  Every unit that includes `lisp.h` gets them from that mirror's
   crate-root `use std::ffi::{…}`; XMenu's headers include no `lisp.h`, so this unit brings the
   whole set itself, the shape `oldXMenu/Internal.rs` 63-66 uses - and the `include!`d mirrors
   below (`xlib.rs`, `XMenu.rs`, `XMenuInt.rs`, `X10.rs`) read these names at the crate root.  */
#[cfg (feature = "have_x_windows")]
use std::ffi::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};

/* `False` and `NULL`: `include/xlib.rs` carries `Bool` but not the two `#define`s
   `<X11/Xlib.h>` 80-81 spells, and no mirror carries `NULL`.  So this unit declares the two names it
   reads, at the use site, with the header's own text (P-88 clause 1 / P-55 - the `oldXMenu/
   Internal.rs` 77-84 shape, and the ledger's `## Owed` names the move back into `include/xlib.rs`).
   C: Xlib.h 80-81: `#define True 1` / `#define False 0`.  */
/// C: Xlib.h 81: `#define False 0`
#[cfg (feature = "have_x_windows")]
pub const False: Bool = 0;

/// C 36-61: `int XMenuSetPane (register XMenu *menu, register int p_num, register int active)
/// { register XMPane *p_ptr; p_ptr = _XMGetPanePtr (menu, p_num); if (p_ptr == NULL)
/// return (XM_FAILURE); p_ptr->active = active; if (p_ptr->active == False) p_ptr->activated =
/// False; _XMErrorCode = XME_NO_ERROR; return (p_num); }` - the two `register`s are C's storage
/// hint (PORTING.md 1.2), `NULL` is the null test Rust spells the same way, and the four internal
/// names come from `XMenuInt.h`'s mirror.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XMenuSetPane (menu: *mut XMenu, p_num: c_int, active: c_int) -> c_int
{
  let p_ptr: *mut XMPane;                     /* XMPane pointer.  */

  /*
   * Find the right pane.
   */
  p_ptr = _XMGetPanePtr (menu, p_num);
  if p_ptr.is_null () { return XM_FAILURE; }

  /*
   * Set its active switch.
   */
  (*p_ptr).active = active;
  if (*p_ptr).active == False { (*p_ptr).activated = False; }

  /*
   * Return the pane number just set.
   */
  _XMErrorCode = XME_NO_ERROR;
  return p_num;
}
