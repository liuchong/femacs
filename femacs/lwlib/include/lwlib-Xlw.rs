// femacs: mirror of lwlib/lwlib-Xlw.h @ fd7afced403.
//
// The Lucid back end's interface: the creation table, the dialog creator it names by its
// own function-pointer typedef, and the per-widget update/destroy/pop-up entry points.  The
// frozen configuration builds no window system - `USE_LUCID`/`USE_X_TOOLKIT` and `HAVE_X11`
// are `undef` in docs/baseline/config/darwin-arm64.macros.tsv - so no translation unit
// includes it and the mirror is never compiled; the text gate `tools/extract-abi.py --check`
// is its whole acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Widget`, `Boolean`, `XEvent` - come from
// <X11/Intrinsic.h>; the port has no mirror for that header, so they keep C's spelling and
// are left for the unit that eventually uses this header to supply (PORTING.md P-26/P-72 (3)).

include!("lwlib-int.rs");   // lwlib-Xlw.h 4: `#include "lwlib-int.h"`

// lwlib-Xlw.h 1-2: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const LWLIB_XLW_H: bool = true;

/* The declarations live in `lwlib_Xlw_decls` + a glob import (the `include/systhread.rs`
   shape, PORTING.md 0.1-10 / P-39): `lwlib/lwlib-Xlw.c`'s port *defines* every function below
   (and `xlw_creation_table`), and a Rust definition cannot share a scope with the foreign
   declaration of the same name (E0428) - an explicit item shadows a glob import instead.  */
pub mod lwlib_Xlw_decls {
  use super::*;

  unsafe extern "C" {
  /// lwlib-Xlw.h 6: `extern widget_creation_entry xlw_creation_table [];` (an incomplete
  /// array; the port declares it with length 0, the lisp.h `my_edata` shape).
  pub static mut xlw_creation_table: [widget_creation_entry; 0];
  /// lwlib-Xlw.h 7: `extern widget_creation_function xlw_create_dialog;`
  pub static mut xlw_create_dialog: widget_creation_function;
  /// lwlib-Xlw.h 9-10: `Boolean lw_lucid_widget_p (Widget widget);`
  pub fn lw_lucid_widget_p (widget: Widget) -> Boolean;
  /// lwlib-Xlw.h 12-14: `void xlw_update_one_widget (widget_instance* instance, Widget widget,
  /// widget_value* val, Boolean deep_p);`
  pub fn xlw_update_one_widget (instance: *mut widget_instance, widget: Widget,
                                val: *mut widget_value, deep_p: Boolean);
  /// lwlib-Xlw.h 16-18: `void xlw_update_one_value (widget_instance* instance, Widget widget,
  /// widget_value* val);`
  pub fn xlw_update_one_value (instance: *mut widget_instance, widget: Widget,
                               val: *mut widget_value);
  /// lwlib-Xlw.h 20-21: `void xlw_destroy_instance (widget_instance* instance);`
  pub fn xlw_destroy_instance (instance: *mut widget_instance);
  /// lwlib-Xlw.h 23-24: `void xlw_pop_instance (widget_instance* instance, Boolean up);`
  pub fn xlw_pop_instance (instance: *mut widget_instance, up: Boolean);
  /// lwlib-Xlw.h 26-27: `void xlw_popup_menu (Widget widget, XEvent * event);`
  pub fn xlw_popup_menu (widget: Widget, event: *mut XEvent);
  }
}
use lwlib_Xlw_decls::*;
