// femacs: mirror of lwlib/lwlib.h @ fd7afced403.
//
// The Lucid Widget Library's toolkit-independent interface: the `LWLIB_ID` type, the
// `menu_separator` enum, the `lw_callback` type and the entry points the three back ends
// (`lwlib-Xaw.c`, `lwlib-Xlw.c`, `lwlib-Xm.c`) and the X menu code provide.  The frozen
// configuration builds no window system - `USE_X_TOOLKIT`, `USE_XAW`, `USE_LUCID`,
// `USE_MOTIF`, `USE_GTK`, `HAVE_X11` and `HAVE_X_WINDOWS` are all `undef` in
// docs/baseline/config/darwin-arm64.macros.tsv - so no translation unit includes it and the
// mirror is never compiled; the text gate `tools/extract-abi.py --check` is its whole
// acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Widget`, `Boolean`, `XEvent`, `Window` - come from
// <X11/Intrinsic.h>; the port has no mirror for that header, so they keep C's spelling and
// are left for the unit that eventually uses this header to supply (PORTING.md P-26/P-72 (3)).

include!("lwlib-widget.rs");   // lwlib.h 43: `#include "lwlib-widget.h"`

// lwlib.h 25-26: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const LWLIB_H: bool = true;

/*
** Widget values depend on the Widget type:
**
** widget:   (name value key enabled data contents/selected)
**
** label:    ("name" "string" NULL NULL NULL NULL)
** button:   ("name" "string" "key" T/F data <default-button-p>)
** button w/menu:
**           ("name" "string" "key" T/F data (label|button|button w/menu...))
** menubar:  ("name" NULL NULL T/F data (button w/menu))
** selectable thing:
**           ("name" "string" "key" T/F data T/F)
** checkbox: selectable thing
** radio:    ("name" NULL NULL T/F data (selectable thing...))
** strings:  ("name" NULL NULL T/F data (selectable thing...))
** text:     ("name" "string" <ign> T/F data)
** main:     ("name")
*/

// lwlib.h 62: `typedef unsigned int LWLIB_ID;`
pub type LWLIB_ID = c_uint;

/* Menu separator types.  */

/// lwlib.h 64-86: `enum menu_separator` - the port spells the C enum as `c_int` with one
/// constant per member (PORTING.md 1.3), in C's declaration order.
pub type menu_separator = c_int;
/* These values are Motif compatible.  */
/// lwlib.h 68: `SEPARATOR_NO_LINE` (the first member: 0)
pub const SEPARATOR_NO_LINE: menu_separator = 0;
/// lwlib.h 69: `SEPARATOR_SINGLE_LINE` (1)
pub const SEPARATOR_SINGLE_LINE: menu_separator = 1;
/// lwlib.h 70: `SEPARATOR_DOUBLE_LINE` (2)
pub const SEPARATOR_DOUBLE_LINE: menu_separator = 2;
/// lwlib.h 71: `SEPARATOR_SINGLE_DASHED_LINE` (3)
pub const SEPARATOR_SINGLE_DASHED_LINE: menu_separator = 3;
/// lwlib.h 72: `SEPARATOR_DOUBLE_DASHED_LINE` (4)
pub const SEPARATOR_DOUBLE_DASHED_LINE: menu_separator = 4;
/// lwlib.h 73: `SEPARATOR_SHADOW_ETCHED_IN` (5)
pub const SEPARATOR_SHADOW_ETCHED_IN: menu_separator = 5;
/// lwlib.h 74: `SEPARATOR_SHADOW_ETCHED_OUT` (6)
pub const SEPARATOR_SHADOW_ETCHED_OUT: menu_separator = 6;
/// lwlib.h 75: `SEPARATOR_SHADOW_ETCHED_IN_DASH` (7)
pub const SEPARATOR_SHADOW_ETCHED_IN_DASH: menu_separator = 7;
/// lwlib.h 76: `SEPARATOR_SHADOW_ETCHED_OUT_DASH` (8)
pub const SEPARATOR_SHADOW_ETCHED_OUT_DASH: menu_separator = 8;
/* The following are supported by Lucid menus.  */
/// lwlib.h 79: `SEPARATOR_SHADOW_DOUBLE_ETCHED_IN` (9)
pub const SEPARATOR_SHADOW_DOUBLE_ETCHED_IN: menu_separator = 9;
/// lwlib.h 80: `SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT` (10)
pub const SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT: menu_separator = 10;
/// lwlib.h 81: `SEPARATOR_SHADOW_DOUBLE_ETCHED_IN_DASH` (11)
pub const SEPARATOR_SHADOW_DOUBLE_ETCHED_IN_DASH: menu_separator = 11;
/// lwlib.h 82: `SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT_DASH` (12)
pub const SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT_DASH: menu_separator = 12;

/// lwlib.h 89: `typedef void (*lw_callback) (Widget w, LWLIB_ID id, void* data);` - the
/// pointer is nullable in C, so it is an `Option` of an `extern "C"` fn (P-27).
pub type lw_callback = Option<unsafe extern "C" fn (w: Widget, id: LWLIB_ID, data: *mut c_void)>;

unsafe extern "C" {
  /// lwlib.h 91-94: `void lw_register_widget (const char* type, const char* name, LWLIB_ID id,
  /// widget_value* val, lw_callback pre_activate_cb, lw_callback selection_cb,
  /// lw_callback post_activate_cb, lw_callback highlight_cb);`
  pub fn lw_register_widget (r#type: *const c_char, name: *const c_char, id: LWLIB_ID,
                             val: *mut widget_value, pre_activate_cb: lw_callback,
                             selection_cb: lw_callback, post_activate_cb: lw_callback,
                             highlight_cb: lw_callback);
  /// lwlib.h 95: `Widget lw_get_widget (LWLIB_ID id, Widget parent, Boolean pop_up_p);`
  pub fn lw_get_widget (id: LWLIB_ID, parent: Widget, pop_up_p: Boolean) -> Widget;
  /// lwlib.h 96: `Widget lw_make_widget (LWLIB_ID id, Widget parent, Boolean pop_up_p);`
  pub fn lw_make_widget (id: LWLIB_ID, parent: Widget, pop_up_p: Boolean) -> Widget;
  /// lwlib.h 97-102: `Widget lw_create_widget (const char* type, const char* name, LWLIB_ID id,
  /// widget_value* val, Widget parent, Boolean pop_up_p, lw_callback pre_activate_cb, ...);`
  pub fn lw_create_widget (r#type: *const c_char, name: *const c_char, id: LWLIB_ID,
                           val: *mut widget_value, parent: Widget, pop_up_p: Boolean,
                           pre_activate_cb: lw_callback, selection_cb: lw_callback,
                           post_activate_cb: lw_callback, highlight_cb: lw_callback) -> Widget;
  /// lwlib.h 103: `LWLIB_ID lw_get_widget_id (Widget w);`
  pub fn lw_get_widget_id (w: Widget) -> LWLIB_ID;
  /// lwlib.h 104: `int lw_modify_all_widgets (LWLIB_ID id, widget_value* val, Boolean deep_p);`
  pub fn lw_modify_all_widgets (id: LWLIB_ID, val: *mut widget_value, deep_p: Boolean) -> c_int;
  /// lwlib.h 105: `void lw_destroy_widget (Widget w);`
  pub fn lw_destroy_widget (w: Widget);
  /// lwlib.h 106: `void lw_destroy_all_widgets (LWLIB_ID id);`
  pub fn lw_destroy_all_widgets (id: LWLIB_ID);
  /// lwlib.h 107: `void lw_destroy_everything (void);`
  pub fn lw_destroy_everything ();
  /// lwlib.h 108: `void lw_destroy_all_pop_ups (void);`
  pub fn lw_destroy_all_pop_ups ();
  /// lwlib.h 109: `Widget lw_raise_all_pop_up_widgets (void);`
  pub fn lw_raise_all_pop_up_widgets () -> Widget;
  /// lwlib.h 110: `widget_value* lw_get_all_values (LWLIB_ID id);`
  pub fn lw_get_all_values (id: LWLIB_ID) -> *mut widget_value;
  /// lwlib.h 111: `Boolean lw_get_some_values (LWLIB_ID id, widget_value* val);`
  pub fn lw_get_some_values (id: LWLIB_ID, val: *mut widget_value) -> Boolean;
  /// lwlib.h 112: `void lw_pop_up_all_widgets (LWLIB_ID id);`
  pub fn lw_pop_up_all_widgets (id: LWLIB_ID);
  /// lwlib.h 113: `void lw_pop_down_all_widgets (LWLIB_ID id);`
  pub fn lw_pop_down_all_widgets (id: LWLIB_ID);
  /// lwlib.h 114: `void lw_popup_menu (Widget, XEvent *);`
  pub fn lw_popup_menu (w: Widget, event: *mut XEvent);
  /* Toolkit independent way of focusing on a Widget at the Xt level. */
  /// lwlib.h 117: `void lw_set_keyboard_focus (Widget parent, Widget w);`
  pub fn lw_set_keyboard_focus (parent: Widget, w: Widget);
  /* Silly Energize hack to invert the "sheet" button */
  /// lwlib.h 120: `void lw_show_busy (Widget w, Boolean busy);`
  pub fn lw_show_busy (w: Widget, busy: Boolean);
  /* Silly hack to assist with Lucid/Athena geometry management. */
  /// lwlib.h 123: `void lw_refigure_widget (Widget w, Boolean doit);`
  pub fn lw_refigure_widget (w: Widget, doit: Boolean);
  /* Toolkit independent way of determining if an event occurred on a
     menubar.  */
  /// lwlib.h 127: `Boolean lw_window_is_in_menubar (Window win, Widget menubar_widget);`
  pub fn lw_window_is_in_menubar (win: Window, menubar_widget: Widget) -> Boolean;
  /* Manage resizing: TRUE permits resizing widget w; FALSE disallows it. */
  /// lwlib.h 130: `void lw_allow_resizing (Widget w, Boolean flag);`
  pub fn lw_allow_resizing (w: Widget, flag: Boolean);
  /* Set up the main window. */
  /// lwlib.h 133-135: `void lw_set_main_areas (Widget parent, Widget menubar, Widget work_area);`
  pub fn lw_set_main_areas (parent: Widget, menubar: Widget, work_area: Widget);
  /* Value is non-zero if LABEL is a menu separator.  If it is, *TYPE is
     set to an appropriate enumerator of type enum menu_separator.
     MOTIF_P non-zero means map separator types not supported by Motif
     to similar ones that are supported.  */
  /// lwlib.h 142: `int lw_separator_p (const char *label, enum menu_separator *type, int motif_p);`
  pub fn lw_separator_p (label: *const c_char, r#type: *mut menu_separator,
                         motif_p: c_int) -> c_int;
}
