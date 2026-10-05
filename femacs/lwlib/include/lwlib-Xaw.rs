// femacs: mirror of lwlib/lwlib-Xaw.h @ fd7afced403.
//
// The Athena back end's interface: the creation table `lwlib.c` looks up, the dialogs and
// the per-widget update/destroy/pop-up entry points.  The frozen configuration builds no
// window system - `USE_XAW`/`USE_X_TOOLKIT` and `HAVE_X11` are `undef` in
// docs/baseline/config/darwin-arm64.macros.tsv - so no translation unit includes it and the
// mirror is never compiled; the text gate `tools/extract-abi.py --check` is its whole
// acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Widget`, `Boolean`, `XEvent` - come from
// <X11/Intrinsic.h>; the port has no mirror for that header, so they keep C's spelling and
// are left for the unit that eventually uses this header to supply (PORTING.md P-26/P-72 (3)).

include!("lwlib-int.rs");   // lwlib-Xaw.h 4: `#include "lwlib-int.h"`

// lwlib-Xaw.h 1-2: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const LWLIB_XAW_H: bool = true;

/* The declarations live in `lwlib_Xaw_decls` + a glob import (the `include/lwlib-Xlw.rs` /
   `include/systhread.rs` shape, PORTING.md 0.1-10 / P-39): `lwlib/lwlib-Xaw.rs`'s port
   *defines* every function below (and `xaw_creation_table`), and a Rust definition cannot share
   a scope with the foreign declaration of the same name (E0428) - an explicit item shadows a
   glob import instead.  */
pub mod lwlib_Xaw_decls {
  use super::*;

  unsafe extern "C" {
  /// lwlib-Xaw.h 6: `extern widget_creation_entry xaw_creation_table [];` (an incomplete
  /// array; the port declares it with length 0, the lisp.h `my_edata` shape).
  pub static mut xaw_creation_table: [widget_creation_entry; 0];
  /// lwlib-Xaw.h 8-9: `Widget xaw_create_dialog (widget_instance*);`
  pub fn xaw_create_dialog (instance: *mut widget_instance) -> Widget;
  /// lwlib-Xaw.h 11-12: `Boolean lw_xaw_widget_p (Widget);`
  pub fn lw_xaw_widget_p (w: Widget) -> Boolean;
  /// lwlib-Xaw.h 14-15: `void xaw_update_one_widget (widget_instance *, Widget, widget_value *, Boolean);`
  pub fn xaw_update_one_widget (instance: *mut widget_instance, w: Widget,
                                val: *mut widget_value, deep_p: Boolean);
  /// lwlib-Xaw.h 17-18: `void xaw_update_one_value (widget_instance *, Widget, widget_value *);`
  pub fn xaw_update_one_value (instance: *mut widget_instance, w: Widget,
                               val: *mut widget_value);
  /// lwlib-Xaw.h 20-21: `void xaw_destroy_instance (widget_instance *);`
  pub fn xaw_destroy_instance (instance: *mut widget_instance);
  /// lwlib-Xaw.h 23-24: `void xaw_popup_menu (Widget, XEvent *);`
  pub fn xaw_popup_menu (w: Widget, event: *mut XEvent);
  /// lwlib-Xaw.h 26-27: `void xaw_pop_instance (widget_instance *, Boolean);`
  pub fn xaw_pop_instance (instance: *mut widget_instance, up: Boolean);
  }
}
use lwlib_Xaw_decls::*;
