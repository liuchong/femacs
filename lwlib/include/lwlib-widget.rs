// femacs: mirror of lwlib/lwlib-widget.h @ fd7afced403.
//
// The toolkit-independent half of the widget protocol: `change_type`, `button_type` and the
// `widget_value` tree the menus are described with.  C keeps it apart from `lwlib.h` because
// it needs no X (C 20-21), and `src/xmenu.c` and the menu-bar code include it on every build.
// The frozen configuration builds no window system, so no translation unit in it includes
// this header and the mirror is never compiled; `tools/extract-abi.py --check` is its whole
// acceptance (docs/status.md 3.31).
//
// Two `#ifdef`s sit inside `struct _widget_value`: `HAVE_NTGUI` (C 70-73) and `USE_X_TOOLKIT`
// (C 87-101).  Both are `undef` in docs/baseline/config/darwin-arm64.macros.tsv, so the
// frozen `widget_value` has neither the `title` field nor the toolkit-data tail; the dead
// arms are still carried as `#[cfg (feature = ...)]` text (PORTING.md P-51), which is what
// keeps C's tokens in the mirror.

// lwlib-widget.h 26's `#include "../src/lisp.h"` is the including unit's
// `include!("../../src/include/lisp.rs")`; this header names `Lisp_Object`, which that mirror
// carries (the disptab.h precedent, docs/ports/include-lwlib-widget.md).

// lwlib-widget.h 23-24: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const LWLIB_WIDGET_H: bool = true;

/// lwlib-widget.h 28-34: `typedef enum { NO_CHANGE = 0, INVISIBLE_CHANGE = 1,
/// VISIBLE_CHANGE = 2, STRUCTURAL_CHANGE = 3 } change_type;` - one constant per member
/// (PORTING.md 1.3).
pub type change_type = c_int;
/// lwlib-widget.h 30: `NO_CHANGE = 0`
pub const NO_CHANGE: change_type = 0;
/// lwlib-widget.h 31: `INVISIBLE_CHANGE = 1`
pub const INVISIBLE_CHANGE: change_type = 1;
/// lwlib-widget.h 32: `VISIBLE_CHANGE = 2`
pub const VISIBLE_CHANGE: change_type = 2;
/// lwlib-widget.h 33: `STRUCTURAL_CHANGE = 3`
pub const STRUCTURAL_CHANGE: change_type = 3;

/// lwlib-widget.h 36-41: `enum button_type` (members `BUTTON_TYPE_NONE` = 0,
/// `BUTTON_TYPE_TOGGLE` = 1, `BUTTON_TYPE_RADIO` = 2).
pub type button_type = c_int;
/// lwlib-widget.h 38: `BUTTON_TYPE_NONE` (the first member: 0)
pub const BUTTON_TYPE_NONE: button_type = 0;
/// lwlib-widget.h 39: `BUTTON_TYPE_TOGGLE` (1)
pub const BUTTON_TYPE_TOGGLE: button_type = 1;
/// lwlib-widget.h 40: `BUTTON_TYPE_RADIO` (2)
pub const BUTTON_TYPE_RADIO: button_type = 2;

/// lwlib-widget.h 43-103: `typedef struct _widget_value { ... } widget_value;`.  C's
/// `Lisp_Object` and `bool` are lisp.rs's and the port's own; the two dead arms below carry
/// C's conditional fields.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _widget_value
{
  /* Name of widget.  */
  pub lname: Lisp_Object,
  pub name: *mut c_char,

  /* Value (meaning depend on widget type).  */
  pub value: *mut c_char,

  /* Keyboard equivalent. no implications for XtTranslations.  */
  pub lkey: Lisp_Object,
  pub key: *mut c_char,

  /* Help string or nil if none.
     GC finds this string through the frame's menu_bar_vector
     or through menu_items.  */
  pub help: Lisp_Object,

  /* True if enabled.  */
  pub enabled: bool,

  /* True if selected.  */
  pub selected: bool,

  /* True if was edited (maintained by get_value).  */
  pub edited: bool,

  /* C 70-73: `#ifdef HAVE_NTGUI` - dead in the frozen configuration, carried.  */
  #[cfg (feature = "have_ntgui")]
  /* True if menu title.  */
  pub title: bool,

  /* The type of a button.  */
  pub button_type: button_type,

  /* Contents of the sub-widgets, also selected slot for checkbox.  */
  pub contents: *mut _widget_value,

  /* Data passed to callback.  */
  pub call_data: *mut c_void,

  /* Next one in the list.  */
  pub next: *mut _widget_value,

  /* C 87-101: `#ifdef USE_X_TOOLKIT` - dead in the frozen configuration, carried.  */
  #[cfg (feature = "use_x_toolkit")]
  /* Type of change (maintained by lw library).  */
  pub change: change_type,
  #[cfg (feature = "use_x_toolkit")]
  /* Type of this widget's change, but not counting the other widgets
     found in the `next' field.  */
  pub this_one_change: change_type,
  #[cfg (feature = "use_x_toolkit")]
  /* Slot for the toolkit dependent part.  Always initialize to NULL.  */
  pub toolkit_data: *mut c_void,
  #[cfg (feature = "use_x_toolkit")]
  /* Whether we should free the toolkit data slot when freeing the
     widget_value itself.  */
  pub free_toolkit_data: bool,
}

/// lwlib-widget.h 103: `typedef struct _widget_value { ... } widget_value;`
pub type widget_value = _widget_value;

/* This part is separate from lwlib.h because it does not need X,
   and thus can be used by non-X code in Emacs proper.  */
