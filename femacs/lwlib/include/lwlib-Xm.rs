// femacs: mirror of lwlib/lwlib-Xm.h @ fd7afced403.
//
// The Motif back end's interface: the creation table, the dialog creator, the per-widget
// update/destroy/pop-up entry points and the three Motif-only hooks (keyboard focus, main
// areas, resizing).  The frozen configuration builds no window system - `USE_MOTIF`/
// `USE_X_TOOLKIT` and `HAVE_X11` are `undef` in docs/baseline/config/darwin-arm64.macros.tsv
// - so no translation unit includes it and the mirror is never compiled; the text gate
// `tools/extract-abi.py --check` is its whole acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Widget`, `Boolean`, `XEvent` - come from
// <X11/Intrinsic.h>; the port has no mirror for that header, so they keep C's spelling and
// are left for the unit that eventually uses this header to supply (PORTING.md P-26/P-72 (3)).

include!("lwlib-int.rs");   // lwlib-Xm.h 22: `#include "lwlib-int.h"`

// lwlib-Xm.h 19-20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const LWLIB_XM_H: bool = true;

unsafe extern "C" {
  /// lwlib-Xm.h 24: `extern widget_creation_entry xm_creation_table [];` (an incomplete
  /// array; the port declares it with length 0, the lisp.h `my_edata` shape).
  pub static mut xm_creation_table: [widget_creation_entry; 0];
  /// lwlib-Xm.h 26-27: `Widget xm_create_dialog (widget_instance* instance);`
  pub fn xm_create_dialog (instance: *mut widget_instance) -> Widget;
  /// lwlib-Xm.h 29-30: `Boolean lw_motif_widget_p (Widget widget);`
  pub fn lw_motif_widget_p (widget: Widget) -> Boolean;
  /// lwlib-Xm.h 32-34: `void xm_update_one_widget (widget_instance* instance, Widget widget,
  /// widget_value* val, Boolean deep_p);`
  pub fn xm_update_one_widget (instance: *mut widget_instance, widget: Widget,
                               val: *mut widget_value, deep_p: Boolean);
  /// lwlib-Xm.h 36-38: `void xm_update_one_value (widget_instance* instance, Widget widget,
  /// widget_value* val);`
  pub fn xm_update_one_value (instance: *mut widget_instance, widget: Widget,
                              val: *mut widget_value);
  /// lwlib-Xm.h 40-41: `void xm_destroy_instance (widget_instance* instance);`
  pub fn xm_destroy_instance (instance: *mut widget_instance);
  /// lwlib-Xm.h 43-44: `void xm_set_keyboard_focus (Widget parent, Widget w);`
  pub fn xm_set_keyboard_focus (parent: Widget, w: Widget);
  /// lwlib-Xm.h 46-47: `void xm_popup_menu (Widget widget, XEvent *event);`
  pub fn xm_popup_menu (widget: Widget, event: *mut XEvent);
  /// lwlib-Xm.h 49-50: `void xm_pop_instance (widget_instance* instance, Boolean up);`
  pub fn xm_pop_instance (instance: *mut widget_instance, up: Boolean);
  /// lwlib-Xm.h 52-53: `void xm_set_main_areas (Widget parent, Widget menubar, Widget work_area);`
  pub fn xm_set_main_areas (parent: Widget, menubar: Widget, work_area: Widget);
  /// lwlib-Xm.h 55-56: `void xm_manage_resizing (Widget w, Boolean flag);`
  pub fn xm_manage_resizing (w: Widget, flag: Boolean);
}
