// femacs: port of src/widget.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header belongs to the Xaw/Lucid widget build (it is
// `<X11/IntrinsicP.h>`'s client for C's own `EmacsFrame` widget), and nothing on this
// target compiles it: `USE_X_TOOLKIT` and `USE_LUCID` are undefined (`docs/baseline/config/`),
// `src/widget.c` is not in the frozen build's source list, and the types it names (`Widget`,
// `WidgetClass`, `XtN*`) are Xaw's, which has no mirror here.
//
// The split follows `include/macfont.rs` (3.170), `include/w32font.rs` (3.176) and
// `include/ftfont.rs` (3.178): the resource-name **strings** are frozen-visible (they are
// plain C string literals), while everything Xaw-typed lives under `#[cfg (feature =
// "use_x_toolkit")]` - P-51's arm for the non-frozen side, spelled with C's own names,
// because that target supplies them.  (`use_lucid` is the other half of the same build; the
// port declares the toolkit feature and the ledger records the relation.)

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const _EmacsFrame_h: bool = true;

/// C 23-24: `#include <X11/IntrinsicP.h>` - Xaw's own header, which has no mirror; the arm
/// below names its types because that target supplies them.

/// C: `#define XtNminibuffer "minibuffer"`
pub const XtNminibuffer: &core::ffi::CStr = c"minibuffer";

/// C: `#define XtCMinibuffer "Minibuffer"`
pub const XtCMinibuffer: &core::ffi::CStr = c"Minibuffer";

/// C: `#define XtNunsplittable "unsplittable"`
pub const XtNunsplittable: &core::ffi::CStr = c"unsplittable";

/// C: `#define XtCUnsplittable "Unsplittable"`
pub const XtCUnsplittable: &core::ffi::CStr = c"Unsplittable";

/// C: `#define XtNinternalBorderWidth "internalBorderWidth"`
pub const XtNinternalBorderWidth: &core::ffi::CStr = c"internalBorderWidth";

/// C: `#define XtCInternalBorderWidth "InternalBorderWidth"`
pub const XtCInternalBorderWidth: &core::ffi::CStr = c"InternalBorderWidth";

/// C: `#define XtNinterline "interline"`
pub const XtNinterline: &core::ffi::CStr = c"interline";

/// C: `#define XtCInterline "Interline"`
pub const XtCInterline: &core::ffi::CStr = c"Interline";

/// C: `#ifndef XtNfont` / `#define XtNfont "font"` / `#endif` - the guard is there
/// because older Xaw headers may define the name already (C 34-46).
pub const XtNfont: &core::ffi::CStr = c"font";

/// C: `#ifndef XtCFont` / `#define XtCFont "Font"` / `#endif` - the guard is there
/// because older Xaw headers may define the name already (C 34-46).
pub const XtCFont: &core::ffi::CStr = c"Font";

/// C: `#ifndef XtNforeground` / `#define XtNforeground "foreground"` / `#endif` - the guard is there
/// because older Xaw headers may define the name already (C 34-46).
pub const XtNforeground: &core::ffi::CStr = c"foreground";

/// C: `#ifndef XtCForeground` / `#define XtCForeground "Foreground"` / `#endif` - the guard is there
/// because older Xaw headers may define the name already (C 34-46).
pub const XtCForeground: &core::ffi::CStr = c"Foreground";

/// C: `#define XtNcursorColor "cursorColor"`
pub const XtNcursorColor: &core::ffi::CStr = c"cursorColor";

/// C: `#define XtCCursorColor "CursorColor"`
pub const XtCCursorColor: &core::ffi::CStr = c"CursorColor";

/// C: `#define XtNbarCursor "barCursor"`
pub const XtNbarCursor: &core::ffi::CStr = c"barCursor";

/// C: `#define XtCBarCursor "BarCursor"`
pub const XtCBarCursor: &core::ffi::CStr = c"BarCursor";

/// C: `#define XtNvisualBell "visualBell"`
pub const XtNvisualBell: &core::ffi::CStr = c"visualBell";

/// C: `#define XtCVisualBell "VisualBell"`
pub const XtCVisualBell: &core::ffi::CStr = c"VisualBell";

/// C: `#define XtNbellVolume "bellVolume"`
pub const XtNbellVolume: &core::ffi::CStr = c"bellVolume";

/// C: `#define XtCBellVolume "BellVolume"`
pub const XtCBellVolume: &core::ffi::CStr = c"BellVolume";

/// C: `#define XtNpointerBackground "pointerBackground"`
pub const XtNpointerBackground: &core::ffi::CStr = c"pointerBackground";

/// C: `#define XtNpointerColor "pointerColor"`
pub const XtNpointerColor: &core::ffi::CStr = c"pointerColor";

/// C: `#define XtNtextPointer "textPointer"`
pub const XtNtextPointer: &core::ffi::CStr = c"textPointer";

/// C: `#define XtNspacePointer "spacePointer"`
pub const XtNspacePointer: &core::ffi::CStr = c"spacePointer";

/// C: `#define XtNmodeLinePointer "modePointer"`
pub const XtNmodeLinePointer: &core::ffi::CStr = c"modePointer";

/// C: `#define XtNgcPointer "gcPointer"`
pub const XtNgcPointer: &core::ffi::CStr = c"gcPointer";

/// C: `#define XtNemacsFrame "emacsFrame"`
pub const XtNemacsFrame: &core::ffi::CStr = c"emacsFrame";

/// C: `#define XtCEmacsFrame "EmacsFrame"`
pub const XtCEmacsFrame: &core::ffi::CStr = c"EmacsFrame";

/// C: `#ifndef XtNgeometry` / `#define XtNgeometry "geometry"` / `#endif` - the guard is there
/// because older Xaw headers may define the name already (C 34-46).
pub const XtNgeometry: &core::ffi::CStr = c"geometry";

/// C: `#ifndef XtCGeometry` / `#define XtCGeometry "Geometry"` / `#endif` - the guard is there
/// because older Xaw headers may define the name already (C 34-46).
pub const XtCGeometry: &core::ffi::CStr = c"Geometry";

/// C: `#ifndef XtNshowGrip` / `#define XtNshowGrip "showGrip"` / `#endif` - the guard is there
/// because older Xaw headers may define the name already (C 34-46).
pub const XtNshowGrip: &core::ffi::CStr = c"showGrip";

/// C: `#ifndef XtNallowResize` / `#define XtNallowResize "allowResize"` / `#endif` - the guard is there
/// because older Xaw headers may define the name already (C 34-46).
pub const XtNallowResize: &core::ffi::CStr = c"allowResize";

/// C: `#ifndef XtNresizeToPreferred` / `#define XtNresizeToPreferred "resizeToPreferred"` / `#endif` - the guard is there
/// because older Xaw headers may define the name already (C 34-46).
pub const XtNresizeToPreferred: &core::ffi::CStr = c"resizeToPreferred";

/// C: `#define XtNinitialGeometry "initialGeometry"`
pub const XtNinitialGeometry: &core::ffi::CStr = c"initialGeometry";

/// C: `#define XtCInitialGeometry "InitialGeometry"`
pub const XtCInitialGeometry: &core::ffi::CStr = c"InitialGeometry";

/// C 92: `extern struct _DisplayContext *display_context;` - the record is `src/widget.c`'s
/// own (C 92 forward-declares it through the pointer), so the object keeps C's pointer depth
/// with the pointee untyped (P-55).
/// C: extern struct _DisplayContext *display_context;
unsafe extern "C" {
  pub static mut display_context: *mut c_void;
}

/// C 88-90: the Xaw half - the two opaque widget typedefs, the class accessor and the three
/// entry points `src/widget.c` defines.  `Widget`/`WidgetClass` are Xaw's names (the target
/// supplies them), and the two typedefs stay pointers with the pointee untyped because
/// `struct _EmacsFrameRec` / `struct _EmacsFrameClassRec` are Xaw's own records.
/// C: typedef struct _EmacsFrameRec *EmacsFrame;
/// C: typedef struct _EmacsFrameClassRec *EmacsFrameClass;
/// C: extern WidgetClass emacsFrameClass (void);
/// C: void EmacsFrameSetCharSize (Widget, int, int);
/// C: void widget_store_internal_border (Widget widget);
/// C: bool widget_update_wm_size_hints (Widget widget, Widget frame);
#[cfg (feature = "use_x_toolkit")]
pub type EmacsFrame = *mut c_void;
#[cfg (feature = "use_x_toolkit")]
pub type EmacsFrameClass = *mut c_void;

/* C 88-90 declares four entry points that `src/widget.c` **defines** (C 146-514), so a copy at
   this mirror's top level is `E0428` in that unit - measured: four of them on `src/widget.rs`'s
   flip (status 3.504).  They sit in a module the mirror glob-imports afterwards (P-39, the
   `include/lwlib-Xlw.rs` 3.500 and `include/emacsgtkfixed.rs` 3.496 shapes): a *glob* import is
   shadowed by a definition in the including unit, while an `unsafe extern` item is not, which is
   exactly the difference this class needs.  */
#[cfg (feature = "use_x_toolkit")]
pub mod widget_decls {
  use super::*;

  unsafe extern "C" {
    pub fn emacsFrameClass () -> WidgetClass;
    pub fn EmacsFrameSetCharSize (w: Widget, columns: c_int, rows: c_int);
    pub fn widget_store_internal_border (widget: Widget);
    pub fn widget_update_wm_size_hints (widget: Widget, frame: Widget) -> bool;
  }
}
#[cfg (feature = "use_x_toolkit")]
use widget_decls::*;
