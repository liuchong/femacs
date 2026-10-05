// femacs: port of oldXMenu/SetFrz.c @ fd7afced403.
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
// run it against either.  The `<X11/*>` names the file reaches through `XMenuInt.h` come from
// `include/xlib.rs` where that mirror carries them and are declared at the use site where it does
// not (P-88 clause 1; the ledger's `## Owed` names the move back).  This file carries no `#if` of
// its own, so there is no second arm to sweep.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_mut, unused_parens,
         unused_comparisons, clippy::all)]

/* C 1-34: the MIT notice and the file's own description - not transcribed (PORTING.md 0.1-19; the
   ledger's `## C line ranges` records the range).  */

#[cfg (feature = "have_x_windows")]
#[macro_use]
mod xmenu_int {
  use super::*;
  include!("include/XMenuInt.rs");             // C 36: `#include "XMenuInt.h"`
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

/// C 38-47: `void XMenuSetFreeze (register XMenu *menu, register int freeze)
/// { /* Set the freeze mode switch.  */ menu->freeze = freeze; }` - the parameter is spelled
/// `freeze`, which is Rust's name of `core::mem::freeze` (a 2025-edition intrinsic); a parameter
/// shadows a path, so the port keeps C's name (PORTING.md 1.2, `include/buffer.rs`'s same shape).
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XMenuSetFreeze (menu: *mut XMenu, freeze: c_int)
{
  /*
   * Set the freeze mode switch.
   */
  (*menu).freeze = freeze;
}
