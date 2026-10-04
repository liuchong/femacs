// femacs: mirror of lwlib/lwlib-utils.h @ fd7afced403.
//
// The widget-tree helpers (`XtApplyToWidgets`, `XtCompositeChildren`, ...) plus, under
// `USE_CAIRO`, the cairo/fontconfig renamings the Lucid back end draws its fonts with.  The
// frozen configuration builds no window system and no cairo: `USE_CAIRO` and `HAVE_XRENDER`
// are `undef` in docs/baseline/config/darwin-arm64.macros.tsv, so no translation unit
// includes this header and the mirror is never compiled; the text gate
// `tools/extract-abi.py --check` is its whole acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Widget`, `XtPointer`, `Display`, `Drawable`, `Visual` -
// come from <X11/Intrinsic.h> and <X11/Xlib.h>, and the cairo/fontconfig names
// (`cairo_t`, `cairo_scaled_font_t`, `FcChar8`) from <cairo.h> and <fontconfig/fontconfig.h>;
// the port has no mirror for any of them, so they keep C's spelling and are left for the unit
// that eventually uses this header to supply (PORTING.md P-26/P-72 (3)).
//
// C 18-62's `#ifdef USE_CAIRO` region is dead in the frozen configuration; it is carried as
// `#[cfg (feature = "use_cairo")]` text (PORTING.md P-51), and the C 35-42 `HAVE_XRENDER`
// pair inside it keeps the `#else` arm's own `XGlyphInfo` (the arm this target would take).

// C 1-2: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _LWLIB_UTILS_H_: bool = true;

unsafe extern "C" {
  /// C 4: `void XtNoClearRefreshWidget (Widget);`
  pub fn XtNoClearRefreshWidget (w: Widget);
}

/// C 6: `typedef void (*XtApplyToWidgetsProc) (Widget, XtPointer);` - nullable in C, so it
/// is an `Option` of an `extern "C"` fn (P-27).
pub type XtApplyToWidgetsProc = Option<unsafe extern "C" fn (w: Widget, client: XtPointer)>;
/// C 7: `typedef void* (*XtApplyUntilToWidgetsProc) (Widget, XtPointer);`
pub type XtApplyUntilToWidgetsProc =
  Option<unsafe extern "C" fn (w: Widget, client: XtPointer) -> *mut c_void>;

unsafe extern "C" {
  /// C 9: `void XtApplyToWidgets (Widget, XtApplyToWidgetsProc, XtPointer);`
  pub fn XtApplyToWidgets (w: Widget, proc: XtApplyToWidgetsProc, client: XtPointer);
  /// C 10: `void *XtApplyUntilToWidgets (Widget, XtApplyUntilToWidgetsProc, XtPointer);`
  pub fn XtApplyUntilToWidgets (w: Widget, proc: XtApplyUntilToWidgetsProc,
                                client: XtPointer) -> *mut c_void;
  /// C 12: `Widget *XtCompositeChildren (Widget, unsigned int *);`
  pub fn XtCompositeChildren (w: Widget, nchildren: *mut c_uint) -> *mut Widget;
  /// C 15-16: `Boolean XtWidgetBeingDestroyedP (Widget widget);` (the C comment above it:
  /// "returns True is the widget is being destroyed, False otherwise").
  pub fn XtWidgetBeingDestroyedP (widget: Widget) -> Boolean;
}

/* C 18: `#ifdef USE_CAIRO` - dead in the frozen configuration, carried.  */
/* C 20-21: `#include <cairo.h>` and `#include <fontconfig/fontconfig.h>`; the port has no
   mirror for either header, and the four types below are the renamings C builds with them.  */

/// C 23-26: `typedef struct { cairo_scaled_font_t *scaled_font; int ascent, descent, height,
/// max_advance_width; } XftFont;` (anonymous, so the port names the record after its typedef).
#[cfg (feature = "use_cairo")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XftFont
{
  pub scaled_font: *mut cairo_scaled_font_t,
  pub ascent: c_int,
  pub descent: c_int,
  pub height: c_int,
  pub max_advance_width: c_int,
}

/// C 28: `typedef cairo_t XftDraw;`
#[cfg (feature = "use_cairo")]
pub type XftDraw = cairo_t;

/// C 30-33: `typedef struct { unsigned long pixel; struct {unsigned short red, green, blue,
/// alpha;} color; } XftColor;` - the nested anonymous record is named `<type>_<member>`
/// (PORTING.md 1.3/P-67).
#[cfg (feature = "use_cairo")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XftColor
{
  pub pixel: c_ulong,
  pub color: XftColor_color,
}
/// C 32: the anonymous `struct {unsigned short red, green, blue, alpha;} color;` member of
/// `XftColor`.
#[cfg (feature = "use_cairo")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XftColor_color
{
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
  pub alpha: c_ushort,
}

/* C 35: `#ifdef HAVE_XRENDER` - dead in the frozen configuration, and `<X11/extensions/
   Xrender.h>` has no mirror; the `#else` arm's own `XGlyphInfo` is the one carried.  */
/// C 38-41: `typedef struct { unsigned short width, height; short x, y, xOff, yOff; }
/// XGlyphInfo;`
#[cfg (feature = "use_cairo")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XGlyphInfo
{
  pub width: c_ushort,
  pub height: c_ushort,
  pub x: c_short,
  pub y: c_short,
  pub xOff: c_short,
  pub yOff: c_short,
}

/* C 44-60: the seven cairo renamings.  Each C macro is a call the port mirrors as a
   `macro_rules!` (PORTING.md 2.1), and the `crxft_*` entry each one forwards to keeps C's
   name as a declaration (the definition lives in `lwlib-utils.c`'s cairo arm, which the
   frozen configuration does not build).  */

/// C 44: `#define XftFontOpenName crxft_font_open_name`
#[cfg (feature = "use_cairo")]
macro_rules! XftFontOpenName {
  ($dpy:expr, $screen:expr, $name:expr) => { crxft_font_open_name ($dpy, $screen, $name) };
}
/// C 46: `#define XftFontClose(dpy, pub) crxft_font_close (pub)` - C's second parameter is
/// spelled `pub`, so the port's metavariable is `pub_` (a Rust keyword cannot be a
/// metavariable name; the token compares as `pub` after the trailing underscore, PORTING.md 0.2).
#[cfg (feature = "use_cairo")]
macro_rules! XftFontClose {
  ($dpy:expr, $pub_:expr) => { crxft_font_close ($pub_) };
}
/// C 48-49: `#define XftDrawCreate(dpy, drawable, visual, colormap) crxft_draw_create (dpy,
/// drawable, visual)`
#[cfg (feature = "use_cairo")]
macro_rules! XftDrawCreate {
  ($dpy:expr, $drawable:expr, $visual:expr, $colormap:expr) => {
    crxft_draw_create ($dpy, $drawable, $visual)
  };
}
/// C 51: `#define XftDrawDestroy cairo_destroy`
#[cfg (feature = "use_cairo")]
macro_rules! XftDrawDestroy {
  ($draw:expr) => { cairo_destroy ($draw) };
}
/// C 52: `#define XftDrawRect crxft_draw_rect`
#[cfg (feature = "use_cairo")]
macro_rules! XftDrawRect {
  ($dpy:expr, $color:expr, $x:expr, $y:expr, $width:expr, $height:expr) => {
    crxft_draw_rect ($dpy, $color, $x, $y, $width, $height)
  };
}
/// C 55: `#define XftDrawStringUtf8 crxft_draw_string`
#[cfg (feature = "use_cairo")]
macro_rules! XftDrawStringUtf8 {
  ($dpy:expr, $color:expr, $font:expr, $x:expr, $y:expr, $string:expr, $len:expr) => {
    crxft_draw_string ($dpy, $color, $font, $x, $y, $string, $len)
  };
}
/// C 58-59: `#define XftTextExtentsUtf8(dpy, pub, string, len, extents) crxft_text_extents
/// (pub, string, len, extents)`
#[cfg (feature = "use_cairo")]
macro_rules! XftTextExtentsUtf8 {
  ($dpy:expr, $pub_:expr, $string:expr, $len:expr, $extents:expr) => {
    crxft_text_extents ($pub_, $string, $len, $extents)
  };
}

#[cfg (feature = "use_cairo")]
unsafe extern "C" {
  /// C 45: `extern XftFont *crxft_font_open_name (Display *, int, const char *);`
  pub fn crxft_font_open_name (dpy: *mut Display, screen: c_int,
                               name: *const c_char) -> *mut XftFont;
  /// C 47: `extern void crxft_font_close (XftFont *);`
  pub fn crxft_font_close (font: *mut XftFont);
  /// C 50: `extern cairo_t *crxft_draw_create (Display *, Drawable, Visual *);`
  pub fn crxft_draw_create (dpy: *mut Display, drawable: Drawable,
                            visual: *mut Visual) -> *mut cairo_t;
  /// C 53-54: `extern void crxft_draw_rect (cairo_t *, const XftColor *, int, int,
  /// unsigned int, unsigned int);`
  pub fn crxft_draw_rect (draw: *mut cairo_t, color: *const XftColor, x: c_int, y: c_int,
                          width: c_uint, height: c_uint);
  /// C 56-57: `extern void crxft_draw_string (cairo_t *, const XftColor *, XftFont *, int,
  /// int, const FcChar8 *, int);`
  pub fn crxft_draw_string (draw: *mut cairo_t, color: *const XftColor, font: *mut XftFont,
                            x: c_int, y: c_int, string: *const FcChar8, len: c_int);
  /// C 60: `extern void crxft_text_extents (XftFont *, const FcChar8 *, int, XGlyphInfo *);`
  pub fn crxft_text_extents (font: *mut XftFont, string: *const FcChar8, len: c_int,
                             extents: *mut XGlyphInfo);
}
