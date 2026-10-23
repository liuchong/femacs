// femacs: mirror of lwlib/xlwmenu.h @ fd7afced403.
//
// The lightweight menubar widget's public interface: the resource-name strings Xt's resource
// tables are built from, the widget/widget-class typedefs, the class pointer and the two
// entry points `xmenu.c` calls.  The frozen configuration builds no window system -
// `USE_LUCID`/`USE_X_TOOLKIT` and `HAVE_X11` are `undef` in
// docs/baseline/config/darwin-arm64.macros.tsv - so no translation unit includes it and the
// mirror is never compiled; the text gate `tools/extract-abi.py --check` is its whole
// acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Widget`, `WidgetClass`, `Window` - come from
// <X11/Intrinsic.h>; the port has no mirror for that header, so they keep C's spelling and
// are left for the unit that eventually uses this header to supply (PORTING.md P-26/P-72 (3)).
// The two tags `_XlwMenuRec` and `_XlwMenuClassRec` are defined in `xlwmenuP.h`, whose mirror
// is `xlwmenuP.rs` (the same batch).

include!("lwlib.rs");   // xlwmenu.h 30: `#include "lwlib.h"`

// xlwmenu.h 21-22: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _XlwMenu_h: bool = true;

/* Resource names used by the XlwMenu widget */
/// xlwmenu.h 33: `#define XtNdisabledForeground "disabledForeground"`
pub const XtNdisabledForeground: &core::ffi::CStr = c"disabledForeground";
/// xlwmenu.h 34: `#define XtCDisabledForeground "DisabledForeground"`
pub const XtCDisabledForeground: &core::ffi::CStr = c"DisabledForeground";
/// xlwmenu.h 35: `#define XtNbuttonForeground "buttonForeground"`
pub const XtNbuttonForeground: &core::ffi::CStr = c"buttonForeground";
/// xlwmenu.h 36: `#define XtCButtonForeground "ButtonForeground"`
pub const XtCButtonForeground: &core::ffi::CStr = c"ButtonForeground";
/// xlwmenu.h 37: `#define XtNmargin "margin"`
pub const XtNmargin: &core::ffi::CStr = c"margin";
/// xlwmenu.h 38: `#define XtNhorizontalSpacing "horizontalSpacing"`
pub const XtNhorizontalSpacing: &core::ffi::CStr = c"horizontalSpacing";
/// xlwmenu.h 39: `#define XtNverticalSpacing "verticalSpacing"`
pub const XtNverticalSpacing: &core::ffi::CStr = c"verticalSpacing";
/// xlwmenu.h 40: `#define XtNarrowSpacing "arrowSpacing"`
pub const XtNarrowSpacing: &core::ffi::CStr = c"arrowSpacing";
/// xlwmenu.h 41: `#define XtNmenu "menu"`
pub const XtNmenu: &core::ffi::CStr = c"menu";
/// xlwmenu.h 42: `#define XtCMenu "Menu"`
pub const XtCMenu: &core::ffi::CStr = c"Menu";
/// xlwmenu.h 43: `#define XtNopen "open"`
pub const XtNopen: &core::ffi::CStr = c"open";
/// xlwmenu.h 44: `#define XtNselect "select"`
pub const XtNselect: &core::ffi::CStr = c"select";
/// xlwmenu.h 45: `#define XtNhighlightCallback "highlightCallback"`
pub const XtNhighlightCallback: &core::ffi::CStr = c"highlightCallback";
/// xlwmenu.h 46: `#define XtNenterCallback "enterCallback"`
pub const XtNenterCallback: &core::ffi::CStr = c"enterCallback";
/// xlwmenu.h 47: `#define XtNleaveCallback "leaveCallback"`
pub const XtNleaveCallback: &core::ffi::CStr = c"leaveCallback";
/// xlwmenu.h 48: `#define XtNmenuBorderWidth "menuBorderWidth"`
pub const XtNmenuBorderWidth: &core::ffi::CStr = c"menuBorderWidth";
/// xlwmenu.h 49: `#define XtNhorizontal "horizontal"`
pub const XtNhorizontal: &core::ffi::CStr = c"horizontal";
/// xlwmenu.h 50: `#define XtCHorizontal "Horizontal"`
pub const XtCHorizontal: &core::ffi::CStr = c"Horizontal";
/// xlwmenu.h 51: `#define XtNcursor "cursor"`
pub const XtNcursor: &core::ffi::CStr = c"cursor";
/// xlwmenu.h 52: `#define XtNCursor "Cursor"`
pub const XtNCursor: &core::ffi::CStr = c"Cursor";
/// xlwmenu.h 53: `#define XtNshowGrip "showGrip"`
pub const XtNshowGrip: &core::ffi::CStr = c"showGrip";
/// xlwmenu.h 54: `#define XtCShowGrip "ShowGrip"`
pub const XtCShowGrip: &core::ffi::CStr = c"ShowGrip";
/// xlwmenu.h 55: `#define XtNresizeToPreferred "resizeToPreferred"`
pub const XtNresizeToPreferred: &core::ffi::CStr = c"resizeToPreferred";
/// xlwmenu.h 56: `#define XtCResizeToPreferred "ResizeToPreferred"`
pub const XtCResizeToPreferred: &core::ffi::CStr = c"ResizeToPreferred";
/// xlwmenu.h 57: `#define XtNallowResize "allowResize"`
pub const XtNallowResize: &core::ffi::CStr = c"allowResize";
/// xlwmenu.h 58: `#define XtCAllowResize "AllowResize"`
pub const XtCAllowResize: &core::ffi::CStr = c"AllowResize";
/// xlwmenu.h 59: `#define XtNborderThickness "borderThickness"`
pub const XtNborderThickness: &core::ffi::CStr = c"borderThickness";
/// xlwmenu.h 60: `#define XtCBorderThickness "BorderThickness"`
pub const XtCBorderThickness: &core::ffi::CStr = c"BorderThickness";
/// xlwmenu.h 61: `#define XtNhighlightForeground "highlightForeground"`
pub const XtNhighlightForeground: &core::ffi::CStr = c"highlightForeground";
/// xlwmenu.h 62: `#define XtCHighlightForeground "HighlightForeground"`
pub const XtCHighlightForeground: &core::ffi::CStr = c"HighlightForeground";
/// xlwmenu.h 63: `#define XtNhighlightBackground "highlightBackground"`
pub const XtNhighlightBackground: &core::ffi::CStr = c"highlightBackground";
/// xlwmenu.h 64: `#define XtCHighlightBackground "HighlightBackground"`
pub const XtCHighlightBackground: &core::ffi::CStr = c"HighlightBackground";

/* Motif-compatible resource names */
/// xlwmenu.h 67: `#define XmNshadowThickness "shadowThickness"`
pub const XmNshadowThickness: &core::ffi::CStr = c"shadowThickness";
/// xlwmenu.h 68: `#define XmCShadowThickness "ShadowThickness"`
pub const XmCShadowThickness: &core::ffi::CStr = c"ShadowThickness";
/// xlwmenu.h 69: `#define XmNtopShadowColor "topShadowColor"`
pub const XmNtopShadowColor: &core::ffi::CStr = c"topShadowColor";
/// xlwmenu.h 70: `#define XmCTopShadowColor "TopShadowColor"`
pub const XmCTopShadowColor: &core::ffi::CStr = c"TopShadowColor";
/// xlwmenu.h 71: `#define XmNbottomShadowColor "bottomShadowColor"`
pub const XmNbottomShadowColor: &core::ffi::CStr = c"bottomShadowColor";
/// xlwmenu.h 72: `#define XmCBottomShadowColor "BottomShadowColor"`
pub const XmCBottomShadowColor: &core::ffi::CStr = c"BottomShadowColor";
/// xlwmenu.h 73: `#define XmNtopShadowPixmap "topShadowPixmap"`
pub const XmNtopShadowPixmap: &core::ffi::CStr = c"topShadowPixmap";
/// xlwmenu.h 74: `#define XmCTopShadowPixmap "TopShadowPixmap"`
pub const XmCTopShadowPixmap: &core::ffi::CStr = c"TopShadowPixmap";
/// xlwmenu.h 75: `#define XmNbottomShadowPixmap "bottomShadowPixmap"`
pub const XmNbottomShadowPixmap: &core::ffi::CStr = c"bottomShadowPixmap";
/// xlwmenu.h 76: `#define XmCBottomShadowPixmap "BottomShadowPixmap"`
pub const XmCBottomShadowPixmap: &core::ffi::CStr = c"BottomShadowPixmap";
/// xlwmenu.h 77: `#define XmRHorizontalDimension "HorizontalDimension"`
pub const XmRHorizontalDimension: &core::ffi::CStr = c"HorizontalDimension";

/// xlwmenu.h 79: `typedef struct _XlwMenuRec *XlwMenuWidget;`
pub type XlwMenuWidget = *mut _XlwMenuRec;
/// xlwmenu.h 80: `typedef struct _XlwMenuClassRec *XlwMenuWidgetClass;`
pub type XlwMenuWidgetClass = *mut _XlwMenuClassRec;

unsafe extern "C" {
  /// xlwmenu.h 82: `extern WidgetClass xlwMenuWidgetClass;`
  pub static mut xlwMenuWidgetClass: WidgetClass;
  /// xlwmenu.h 84: `extern int xlwmenu_window_p (Widget w, Window window);`
  pub fn xlwmenu_window_p (w: Widget, window: Window) -> c_int;
  /// xlwmenu.h 85: `extern void xlwmenu_redisplay (Widget);`
  pub fn xlwmenu_redisplay (w: Widget);
}
