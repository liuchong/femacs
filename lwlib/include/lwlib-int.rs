// femacs: mirror of lwlib/lwlib-int.h @ fd7afced403.
//
// The back ends' shared internal records: `widget_instance` (one toolkit widget plus its
// `widget_info`), `widget_info` (the toolkit-independent description and its callback set),
// `widget_creation_function` and `widget_creation_entry` (the tables each back end exports).
// The frozen configuration builds no window system - `USE_CAIRO`, `HAVE_XFT` and
// `USE_X_TOOLKIT` are all `undef` in docs/baseline/config/darwin-arm64.macros.tsv - so no
// translation unit includes it and the mirror is never compiled; the text gate
// `tools/extract-abi.py --check` is its whole acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Widget`, `Boolean`, `XtPointer` - come from
// <X11/Intrinsic.h>; the port has no mirror for that header, so they keep C's spelling and
// are left for the unit that eventually uses this header to supply (PORTING.md P-26/P-72 (3)).

include!("lwlib.rs");   // lwlib-int.h 24: `#include "lwlib.h"`

// lwlib-int.h 21-22: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const LWLIB_INTERNAL_H: bool = true;

/// lwlib-int.h 26: `struct widget_xft_data;` - an incomplete type C only ever points at; the
/// port's stand-in is the zero-size unit struct (PORTING.md P-70).  **Gated off where the tag's
/// owner is in the same crate**: `lwlib/lwlib-Xaw.c` 62-70 defines the complete `struct
/// widget_xft_data`, and `lwlib/lwlib-Xaw.rs` includes this mirror - an ungated stand-in was
/// `E0428` there, which made that unit's cairo/Xft arms unbuildable (measured by the lane,
/// status 3.517).  The gate is the same condition the field below uses, so the stand-in exists
/// exactly when nothing defines the tag.
#[cfg (not (feature = "use_cairo_or_have_xft"))]
pub struct widget_xft_data;

/// lwlib-int.h 28-38: `typedef struct _widget_instance { ... } widget_instance;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _widget_instance
{
  pub widget: Widget,
  pub parent: Widget,
  pub pop_up_p: Boolean,
  /* C 33-35: `#if defined USE_CAIRO || defined HAVE_XFT` - dead in the frozen configuration for
     cairo and live for Xft, which is why the gate is the derived name and not `use_cairo` alone
     (the lane's D2: a `have_xft`-without-cairo build read `cannot find field xft_data`).  */
  #[cfg (feature = "use_cairo_or_have_xft")]
  pub xft_data: *mut widget_xft_data,
  pub info: *mut _widget_info,
  pub next: *mut _widget_instance,
}
/// lwlib-int.h 38: `typedef struct _widget_instance { ... } widget_instance;`
pub type widget_instance = _widget_instance;

/// lwlib-int.h 40-53: `typedef struct _widget_info { ... } widget_info;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _widget_info
{
  pub r#type: *mut c_char,
  pub name: *mut c_char,
  pub id: LWLIB_ID,
  pub val: *mut widget_value,
  pub busy: Boolean,
  pub pre_activate_cb: lw_callback,
  pub selection_cb: lw_callback,
  pub post_activate_cb: lw_callback,
  pub highlight_cb: lw_callback,
  pub instances: *mut _widget_instance,
  pub next: *mut _widget_info,
}
/// lwlib-int.h 53: `typedef struct _widget_info { ... } widget_info;`
pub type widget_info = _widget_info;

/// lwlib-int.h 55: `typedef Widget (*widget_creation_function) (widget_instance *instance);`
pub type widget_creation_function =
  Option<unsafe extern "C" fn (instance: *mut widget_instance) -> Widget>;

/// lwlib-int.h 57-61: `typedef struct _widget_creation_entry { ... } widget_creation_entry;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _widget_creation_entry
{
  pub r#type: *mut c_char,
  pub function: widget_creation_function,
}
/// lwlib-int.h 61: `typedef struct _widget_creation_entry { ... } widget_creation_entry;`
pub type widget_creation_entry = _widget_creation_entry;

/* The declarations live in `lwlib_int_decls` + a glob import (the `include/lwlib.rs` /
   `lwlib-Xlw.rs` shape, PORTING.md 0.1-10 / P-39): three of the four are *defined* by
   `lwlib/lwlib.c`'s port, and a Rust definition cannot share a scope with the foreign
   declaration of the same name (E0428) - an explicit item shadows a glob import instead.  */
pub mod lwlib_int_decls {
  use super::*;

  unsafe extern "C" {
    /* update all other instances of a widget.  Can be used in a callback when
       a widget has been used by the user */
    /// lwlib-int.h 65-66: `void lw_internal_update_other_instances (Widget, XtPointer, XtPointer);`
    pub fn lw_internal_update_other_instances (w: Widget, a: XtPointer, b: XtPointer);
    /* get the widget_value for a widget in a given instance */
    /// lwlib-int.h 69-70: `widget_value* lw_get_widget_value_for_widget (widget_instance *, Widget);`
    pub fn lw_get_widget_value_for_widget (instance: *mut widget_instance,
                                           w: Widget) -> *mut widget_value;
    /// lwlib-int.h 72: `widget_info *lw_get_widget_info (LWLIB_ID);`
    pub fn lw_get_widget_info (id: LWLIB_ID) -> *mut widget_info;
    /// lwlib-int.h 73: `widget_instance * lw_get_widget_instance (Widget);`
    pub fn lw_get_widget_instance (w: Widget) -> *mut widget_instance;
  }
}
pub use lwlib_int_decls::*;
