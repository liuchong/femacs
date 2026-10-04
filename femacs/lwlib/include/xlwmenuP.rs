// femacs: mirror of lwlib/xlwmenuP.h @ fd7afced403.
//
// The lightweight menubar widget's private records: the per-window state the pop-up stacks
// are made of (`window_state`), the instance record (`XlwMenuPart` inside `XlwMenuRec`) and
// the class record.  The frozen configuration builds no window system and no i18n - 
// `USE_CAIRO`, `HAVE_XFT`, `HAVE_X_I18N`, `USE_LUCID`/`USE_X_TOOLKIT` and `HAVE_X11` are all
// `undef` in docs/baseline/config/darwin-arm64.macros.tsv - so no translation unit includes
// it and the mirror is never compiled; the text gate `tools/extract-abi.py --check` is its
// whole acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Widget`, `Window`, `Pixmap`, `Position`, `Dimension`,
// `String`, `XFontStruct`, `Pixel`, `Cursor`, `XtCallbackList`, `GC`, `CorePart`,
// `CoreClassPart`, `XFontSet`, `XFontSetExtents` - come from <X11/Intrinsic.h>,
// <X11/CoreP.h> and <X11/Xlib.h>, and the cairo/Xft ones (`XftDraw`, `XftFont`, `XftColor`)
// from "lwlib-utils.h" and <X11/Xft/Xft.h>; the port has no mirror for any of those headers,
// so they keep C's spelling and are left for the unit that eventually uses this header to
// supply (PORTING.md P-26/P-72 (3)).  `String` is X11's `char *`, and the name collides with
// the Rust prelude's `std::string::String`: the unit that uses this header must define the
// alias, because a module item shadows the prelude and a missing alias would silently widen
// `fontName` from a pointer to a 24-byte `String`.
//
// C 26-32's two `#if` regions are dead in the frozen configuration and are carried as
// `#[cfg (feature = ...)]` text (PORTING.md P-51): the cairo/Xft arm, and the `HAVE_X_I18N`
// fields of `XlwMenuPart`.

include!("xlwmenu.rs");   // xlwmenuP.h 24: `#include "xlwmenu.h"`

/* C 25: `#include <X11/CoreP.h>` - the port has no mirror for it (X11).  */

// xlwmenuP.h 21-22: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _XlwMenuP_h: bool = true;

/* Elements in the stack arrays. */
/// xlwmenuP.h 35-52: `typedef struct _window_state { ... } window_state;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _window_state
{
  pub w: Widget,
  pub window: Window,
  pub pixmap: Pixmap,
  pub x: Position,
  pub y: Position,
  pub width: Dimension,
  pub height: Dimension,
  pub label_width: Dimension,
  pub max_rest_width: c_int,

  /* Width of toggle buttons or radio buttons.  */
  pub button_width: Dimension,
  /* C 49-51: `#if defined USE_CAIRO || defined HAVE_XFT` - dead in the frozen
     configuration, carried.  */
  #[cfg (feature = "use_cairo_or_have_xft")]
  pub xft_draw: *mut XftDraw,
}
/// xlwmenuP.h 52: `typedef struct _window_state { ... } window_state;`
pub type window_state = _window_state;

/* New fields for the XlwMenu widget instance record */
/// xlwmenuP.h 56-133: `typedef struct _XlwMenu_part { ... } XlwMenuPart;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _XlwMenu_part
{
  /* slots set by the resources */
  /* C 59-62: `#ifdef HAVE_X_I18N` - dead in the frozen configuration, carried.  */
  #[cfg (feature = "have_x_i18n")]
  pub fontSet: XFontSet,
  #[cfg (feature = "have_x_i18n")]
  pub font_extents: *mut XFontSetExtents,
  /* C 63-67: `#if defined USE_CAIRO || defined HAVE_XFT` - dead in the frozen
     configuration, carried.  */
  #[cfg (feature = "use_cairo_or_have_xft")]
  pub default_face: c_int,
  #[cfg (feature = "use_cairo_or_have_xft")]
  pub xft_font: *mut XftFont,
  #[cfg (feature = "use_cairo_or_have_xft")]
  pub xft_fg: XftColor,
  #[cfg (feature = "use_cairo_or_have_xft")]
  pub xft_bg: XftColor,
  #[cfg (feature = "use_cairo_or_have_xft")]
  pub xft_disabled_fg: XftColor,
  #[cfg (feature = "use_cairo_or_have_xft")]
  pub xft_highlight_fg: XftColor,
  pub fontName: String,
  pub font: *mut XFontStruct,
  pub foreground: Pixel,
  pub disabled_foreground: Pixel,
  pub button_foreground: Pixel,
  pub highlight_foreground: Pixel,
  pub highlight_background: Pixel,
  pub margin: Dimension,
  pub horizontal_spacing: Dimension,
  pub vertical_spacing: Dimension,
  pub arrow_spacing: Dimension,
  pub shadow_thickness: Dimension,
  pub border_thickness: Dimension,
  pub top_shadow_color: Pixel,
  pub bottom_shadow_color: Pixel,
  pub top_shadow_pixmap: Pixmap,
  pub bottom_shadow_pixmap: Pixmap,
  pub top_highlight_shadow_color: Pixel,
  pub bottom_highlight_shadow_color: Pixel,
  pub top_highlight_shadow_pixmap: Pixmap,
  pub bottom_highlight_shadow_pixmap: Pixmap,
  pub cursor_shape: Cursor,
  pub open: XtCallbackList,
  pub select: XtCallbackList,
  pub highlight: XtCallbackList,
  pub enter: XtCallbackList,
  pub leave: XtCallbackList,
  pub contents: *mut widget_value,
  pub horizontal: c_int,

  /* True means top_shadow_color and/or bottom_shadow_color must be freed.  */
  pub free_top_shadow_color_p: Boolean,
  pub free_bottom_shadow_color_p: Boolean,
  pub free_top_highlight_shadow_color_p: Boolean,
  pub free_bottom_highlight_shadow_color_p: Boolean,

  /* State of the XlwMenu */
  pub top_depth: c_int,
  pub old_depth: c_int,
  pub old_stack: *mut *mut widget_value,
  pub old_stack_length: c_int,
  pub inside_entry: *mut widget_value,

  /* New state after the user moved */
  pub new_depth: c_int,
  pub new_stack: *mut *mut widget_value,
  pub new_stack_length: c_int,

  /* Window resources */
  pub windows: *mut window_state,
  pub windows_length: c_int,

  /* Internal part, set by the XlwMenu */
  pub foreground_gc: GC,
  pub button_gc: GC,
  pub background_gc: GC,
  pub disabled_gc: GC,
  pub highlight_foreground_gc: GC,
  pub highlight_background_gc: GC,
  pub inactive_button_gc: GC,
  pub shadow_top_gc: GC,
  pub shadow_bottom_gc: GC,
  pub highlight_shadow_top_gc: GC,
  pub highlight_shadow_bottom_gc: GC,
  pub cursor: Cursor,
  pub popped_up: Boolean,
  pub gray_pixmap: Pixmap,
}
/// xlwmenuP.h 133: `typedef struct _XlwMenu_part { ... } XlwMenuPart;`
pub type XlwMenuPart = _XlwMenu_part;

/* Full instance record declaration */
/// xlwmenuP.h 136-140: `typedef struct _XlwMenuRec { ... } XlwMenuRec;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _XlwMenuRec
{
  pub core: CorePart,
  pub menu: XlwMenuPart,
}
/// xlwmenuP.h 140: `typedef struct _XlwMenuRec { ... } XlwMenuRec;`
pub type XlwMenuRec = _XlwMenuRec;

/* New fields for the XlwMenu widget class record */
/// xlwmenuP.h 143-146: `typedef struct { int dummy; } XlwMenuClassPart;` (anonymous, so the
/// port names the record after its typedef).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XlwMenuClassPart
{
  pub dummy: c_int,
}

/* Full class record declaration. */
/// xlwmenuP.h 149-153: `typedef struct _XlwMenuClassRec { ... } XlwMenuClassRec;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _XlwMenuClassRec
{
  pub core_class: CoreClassPart,
  pub menu_class: XlwMenuClassPart,
}
/// xlwmenuP.h 153: `typedef struct _XlwMenuClassRec { ... } XlwMenuClassRec;`
pub type XlwMenuClassRec = _XlwMenuClassRec;

unsafe extern "C" {
  /* Class pointer. */
  /// xlwmenuP.h 156: `extern XlwMenuClassRec xlwMenuClassRec;`
  pub static mut xlwMenuClassRec: XlwMenuClassRec;
}
