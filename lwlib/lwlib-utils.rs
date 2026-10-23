// femacs: port of lwlib/lwlib-utils.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2).
//
// **The file gate is `feature = "use_lucid"`** (P-86 clause 8, the build's condition rather than
// one the C text spells): the reference builds `lwlib-utils.o` inside the widget library's own
// object list (`lwlib/Makefile.in` 58: `OBJS = lwlib.o $(TOOLKIT_OBJS) lwlib-utils.o`, whose
// toolkit lists are line 52's `LUCID_OBJS` and line 54's `MOTIF_OBJS`), and `configure.ac` 7231's
// `AC_DEFINE ([USE_LUCID], [1], [Define to 1 if using the Lucid X toolkit.])` is the condition
// this port's gate is written as.  `USE_LUCID`, `USE_X_TOOLKIT`, `HAVE_X11` and `USE_CAIRO` are
// `undef` in docs/baseline/config/darwin-arm64.macros.tsv, so the frozen configuration compiles
// none of this file and the port's frozen object is the empty one.
//
// **The X11 half is `text-only`** (P-86 clause 3(2)): this machine has no X11 header (no
// xorgproto and no XQuartz), so the widget records C 26-28 pull in are C's spelling and their
// widths are **UNVERIFIED** (`## Owed` 1).  The cairo half is not: `<cairo.h>` (1.18.4),
// `<cairo-ft.h>` and `<fontconfig/fontconfig.h>` (2.18.3) are installed under
// /opt/homebrew/include, so every cairo/fontconfig declaration below was **checked with clang
// against those headers** - each prototype by a function-pointer assignment and each record by a
// `sizeof`/`offsetof` probe (the readings are in the ledger's `## Evidence` ③).  What the round
// shows is that both arms **type-check** (`## Evidence`).
//
// C 141's `#ifdef USE_CAIRO` keeps a gate of its own (P-86 clause 1), written
// `all (feature = "use_lucid", feature = "use_cairo")`: the second half - the seven `crxft_*`
// renamings the Lucid back end's font code goes through - needs both features, and so do the
// `cairo_t` / `FcChar8` / `Display` / `Drawable` / `Visual` names the header mirror
// (`include/lwlib-utils.rs`) leaves to the unit that includes it.
//
// The `<X11/*>` surface this file reads is declared in this unit (P-88 clause 1: no mirror in
// this tree carries the three headers of C 26-28, and `lwlib/include/lwlib.rs`'s own comment
// leaves `Widget`, `Boolean` and `XEvent` to the unit that includes it), with the shapes
// **UNVERIFIED** - the `lwlib-Xlw.rs` reading of the same headers.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, unused_mut, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, clippy::all)]

include!("../src/include/config.rs");            // C 21: `#include <config.h>`
/* C 23's `#include <setjmp.h>` has no edge of its own: nothing in this file names a type that
   header declares, and the mirror that carries them is `lisp.rs` (its own note at 1060), which
   C 24 includes next.  */
#[cfg (feature = "use_lucid")]
include!("../src/include/lisp.rs");              // C 24: `#include <lisp.h>`
#[cfg (feature = "use_lucid")]
include!("../lib/include/string.rs");            // lisp.h 30: `#include <string.h>` (memcpy)
/* C 26-28's `<X11/Xatom.h>`, `<X11/IntrinsicP.h>` and `<X11/ObjectP.h>` are the surface below
   (P-88 clause 1: this unit is its only consumer in this tree).  */
#[cfg (feature = "use_lucid")]
include!("include/lwlib-utils.rs");              // C 29: `#include "lwlib-utils.h"`
#[cfg (feature = "use_lucid")]
include!("include/lwlib.rs");                    // C 30: `#include "lwlib.h"`

/* ------------------------------------------------------------------------------------------
   `<X11/IntrinsicP.h>`, `<X11/ObjectP.h>` and the `<X11/Intrinsic.h>` / `<X11/Xlib.h>` they
   pull in.  **C's spellings, not measurements** - no X11 header is on this machine, so every
   width and layout below is `UNVERIFIED` (P-86 clause 3(2)).
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "use_lucid")]
pub type XtPointer = *mut c_void;                     // C: `typedef char *XtPointer;`
#[cfg (feature = "use_lucid")]
pub type Boolean = c_int;                             // C: `typedef int Boolean;`
#[cfg (feature = "use_lucid")]
pub type Dimension = c_ushort;                        // C: X.h: `typedef unsigned short Dimension;`
#[cfg (feature = "use_lucid")]
pub type Cardinal = c_uint;                           // C: X.h: `typedef unsigned int Cardinal;`
/// C: Xlib.h: `typedef struct _XDisplay Display;` - the display the widget's screen belongs to.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Display { pub _private: [u8; 0] }
/// C: X.h: `typedef XID Window;` (`typedef unsigned long XID;`) - `lwlib/include/lwlib.rs`
/// names `Window` and leaves it to the unit that includes it (its own header comment).
#[cfg (feature = "use_lucid")]
pub type Window = c_ulong;
/// C: X.h: `typedef XID Drawable;`
#[cfg (feature = "use_lucid")]
pub type Drawable = c_ulong;
/// C: Xlib.h: `typedef struct _XVisual Visual;` - the visual `cairo_xlib_surface_create` takes.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Visual { pub _private: [u8; 0] }

/// C: X.h: `#define Expose 12` - the event code `XtNoClearRefreshWidget` refreshes with.
#[cfg (feature = "use_lucid")]
pub const Expose: c_int = 12;

/// C: Intrinsic.h: `typedef struct _WidgetRec *Widget;` - the record carries only the members
/// this file reads (`core.width`, `core.height`, `core.widget_class` and `core.being_destroyed`),
/// in C's own relative order, and its offsets are **UNVERIFIED**.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct WidgetRec
{
  pub core: CorePart,
}
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CorePart
{
  pub widget_class: WidgetClass,                // C: `WidgetClass widget_class;`
  pub width: Dimension,                         // C: `Dimension width;`
  pub height: Dimension,                        // C: `Dimension height;`
  pub being_destroyed: Boolean,                 // C: `Boolean being_destroyed;`
}
/// C: IntrinsicP.h: `typedef struct _WidgetClassRec { CoreClassPart core_class; … } *WidgetClass;`
/// - the class record's `core_class.expose` is the refresh callback the last line of
/// `XtNoClearRefreshWidget` calls, and its offset is **UNVERIFIED**.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct WidgetClassRec
{
  pub core_class: CoreClassPart,
}
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CoreClassPart
{
  pub expose: XtExposeProc,                     // C: `XtExposeProc expose;`
}
/// C: IntrinsicP.h: `typedef void (*XtExposeProc) (Widget, XEvent *, Region);`
#[cfg (feature = "use_lucid")]
pub type XtExposeProc = Option<unsafe extern "C" fn (Widget, *mut XEvent, Region)>;
#[cfg (feature = "use_lucid")]
pub type Widget = *mut WidgetRec;
#[cfg (feature = "use_lucid")]
pub type WidgetClass = *mut WidgetClassRec;

/// C: IntrinsicP.h: `typedef struct { WidgetList children; Cardinal num_children; … }
/// CompositePart;` - the members this file reads are `children` and `num_children`; the record's
/// offsets are **UNVERIFIED**.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CompositePart
{
  pub children: *mut Widget,                    // C: `WidgetList children;`
  pub num_children: Cardinal,                   // C: `Cardinal num_children;`
}
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CompositeRec
{
  pub core: CorePart,
  pub composite: CompositePart,
}
#[cfg (feature = "use_lucid")]
pub type CompositeWidget = *mut CompositeRec;

/// C: Xlib.h: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; int x, y; int width, height; int count; } XExposeEvent;` - the
/// members this file writes are carried in C's order, **UNVERIFIED**.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XExposeEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: Boolean,
  pub display: *mut Display,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub count: c_int,
}
/// C: Xlib.h: `typedef union _XEvent { int type; XAnyEvent xany; XExposeEvent xexpose; … }
/// XEvent;` - the port carries the union with the two members this file touches (`type`, which
/// C 38 writes, and `xexpose`, which C 39-47 fills) plus a padding member that keeps the union
/// the 192 bytes the frozen target's Xlib has; the members C does not name are **UNVERIFIED**
/// and owed (`## Owed`).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub union XEvent
{
  pub r#type: c_int,
  pub xexpose: XExposeEvent,
  pub _padding: [u8; 192],
}
/// C: Xlib.h: `typedef struct _XRegion *Region;` - `XtNoClearRefreshWidget` passes a null one.
#[cfg (feature = "use_lucid")]
pub type Region = *mut c_void;

#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  /// C: IntrinsicP.h: `extern WidgetClass compositeWidgetClass;` - the class C's
  /// `XtIsComposite (w)` macro compares against.
  pub static mut compositeWidgetClass: WidgetClass;
  /// C: IntrinsicP.h: `extern WidgetClass widgetClass;` - the class C's `XtIsWidget (w)` macro
  /// compares against.
  pub static mut widgetClass: WidgetClass;
  /* C: Intrinsic.h: `Display *XtDisplay (Widget);`
     C: Intrinsic.h: `Window XtWindow (Widget);`
     C: Intrinsic.h: `Boolean XtIsComposite (Widget);` (a macro in C: `XtIsSubclass (w,
     compositeWidgetClass)`, the `XtIsShell` shape `lwlib-Xlw.rs` declares)
     C: Intrinsic.h: `Boolean XtIsWidget (Widget);` (a macro in C: `XtIsSubclass (w, widgetClass)`)
     C: Intrinsic.h: `char *XtMalloc (Cardinal);`
     C: Xlib.h: `int XGetGeometry (Display *, Drawable, Window *, int *, int *, unsigned int *,
     unsigned int *, unsigned int *, unsigned int *);`
     C: Xlib.h: `char *XGetDefault (Display *, const char *, const char *);`
     C: Xlib.h: `#define DisplayHeight(dpy, scr) (ScreenOfDisplay (dpy, scr)->height)`
     C: Xlib.h: `#define DisplayHeightMM(dpy, scr) (ScreenOfDisplay (dpy, scr)->mheight)`  */
  pub fn XtDisplay (widget: Widget) -> *mut Display;
  pub fn XtWindow (widget: Widget) -> Window;
  pub fn XtIsComposite (widget: Widget) -> Boolean;
  pub fn XtIsWidget (widget: Widget) -> Boolean;
  pub fn XtMalloc (size: usize) -> *mut c_char;
  pub fn XGetGeometry (dpy: *mut Display, drawable: Drawable, root: *mut Window, x: *mut c_int,
                       y: *mut c_int, width: *mut c_uint, height: *mut c_uint,
                       border_width: *mut c_uint, depth: *mut c_uint) -> c_int;
  pub fn XGetDefault (dpy: *mut Display, program: *const c_char,
                      option: *const c_char) -> *mut c_char;
  pub fn DisplayHeight (dpy: *mut Display, screen: c_int) -> c_int;
  pub fn DisplayHeightMM (dpy: *mut Display, screen: c_int) -> c_int;
}

/* ------------------------------------------------------------------------------------------
   C 143-145's `<math.h>`, `<cairo-ft.h>` and `<cairo-xlib.h>`: the cairo/fontconfig surface
   (P-88 clause 1, no mirror in this tree carries it).  The cairo and fontconfig declarations are
   the headers' own text **measured with clang** (this machine's cairo 1.18.4 and fontconfig
   2.18.3; the probe's readings are in the ledger's `## Evidence` ③), and `cairo-xlib_surface_create`
   is cairo-xlib.h 49's prototype with the `<X11/Xlib.h>` half of its surface still UNVERIFIED.
   ------------------------------------------------------------------------------------------ */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
include!("../lib/include/math.rs");             // C 143: `#include <math.h>` (`ceil`; `lround` is
                                                // the other name read from it, declared below)
/* The two names the body below reads that the mirrors above do not carry, each at its use site
   with the reference header's line (P-72 rule 4, P-55), the `src/hbfont.rs` 585-590 shape.  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
unsafe extern "C" {
  /// C: math.h 514: `extern long int lround(double);`
  pub fn lround (x: c_double) -> c_long;
  /// C: _stdio.h 282: `int sscanf(const char * __restrict, const char * __restrict, ...);`
  pub fn sscanf (s: *const c_char, format: *const c_char, ...) -> c_int;
}

/// C: cairo.h: `typedef struct _cairo cairo_t;` - the drawing context `crxft_draw_create`
/// returns and every `crxft_draw_*` takes.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_t { pub _private: [u8; 0] }
/// C: cairo.h: `typedef struct _cairo_scaled_font cairo_scaled_font_t;`
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_scaled_font_t { pub _private: [u8; 0] }
/// C: cairo.h: `typedef struct _cairo_font_face cairo_font_face_t;`
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_font_face_t { pub _private: [u8; 0] }
/// C: cairo.h: `typedef struct _cairo_surface cairo_surface_t;`
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_surface_t { pub _private: [u8; 0] }
/// C: cairo.h: `typedef struct _cairo_font_options cairo_font_options_t;`
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_font_options_t { pub _private: [u8; 0] }
/// C: cairo.h: `typedef struct _cairo_matrix { double xx, yx, xy, yy, x0, y0; } cairo_matrix_t;`
/// - clang-measured: `sizeof` 48, align 8, the six doubles at 0/8/16/24/32/40.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_matrix_t
{
  pub xx: c_double,
  pub yx: c_double,
  pub xy: c_double,
  pub yy: c_double,
  pub x0: c_double,
  pub y0: c_double,
}
/// C: cairo.h: `typedef struct _cairo_font_extents { double ascent, descent, height,
/// max_x_advance, max_y_advance; } cairo_font_extents_t;` - clang-measured: `sizeof` 40,
/// ascent 0, descent 8, height 16, max_x_advance 24.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_font_extents_t
{
  pub ascent: c_double,
  pub descent: c_double,
  pub height: c_double,
  pub max_x_advance: c_double,
  pub max_y_advance: c_double,
}
/// C: cairo.h: `typedef struct _cairo_text_extents { double x_bearing, y_bearing, width, height,
/// x_advance, y_advance; } cairo_text_extents_t;` - clang-measured: `sizeof` 48, x_bearing 0,
/// y_bearing 8, width 16, height 24, x_advance 32, y_advance 40.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_text_extents_t
{
  pub x_bearing: c_double,
  pub y_bearing: c_double,
  pub width: c_double,
  pub height: c_double,
  pub x_advance: c_double,
  pub y_advance: c_double,
}

/// C: fontconfig.h: `typedef unsigned char FcChar8;` - clang-measured: `sizeof` 1.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
pub type FcChar8 = c_uchar;
/// C: fontconfig.h: `typedef int FcBool;` - clang-measured: `sizeof` 4.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
pub type FcBool = c_int;
/// C: fontconfig.h: `typedef struct _FcConfig FcConfig;`
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct FcConfig { pub _private: [u8; 0] }
/// C: fontconfig.h: `typedef struct _FcPattern FcPattern;`
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct FcPattern { pub _private: [u8; 0] }
/// C: fontconfig.h 279-285: `typedef enum _FcResult { FcResultMatch, FcResultNoMatch,
/// FcResultTypeMismatch, FcResultNoId, FcResultOutOfMemory } FcResult;` - the C enum is spelled
/// `c_int` with one constant per member (PORTING.md 1.3); clang-measured: `sizeof (FcResult)` 4,
/// `FcResultMatch` 0, `FcResultNoMatch` 1.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
pub type FcResult = c_int;
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
pub const FcResultMatch: FcResult = 0;
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
pub const FcResultNoMatch: FcResult = 1;
/// C: fontconfig.h 334-341: `typedef enum _FcMatchKind { FcMatchPattern, FcMatchFont,
/// FcMatchScan } FcMatchKind;` - the one member this file passes, same spelling as `FcResult`;
/// clang-measured: `FcMatchPattern` 0.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
pub const FcMatchPattern: c_int = 0;
/// C: fontconfig.h 116: `#define FC_DPI "dpi"` - a string macro, carried as the `&CStr` constant the
/// `xlwmenu.rs` mirror's `XtN*` names use (PORTING.md 2.1); clang-measured: `FC_DPI` is `"dpi"`.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
pub const FC_DPI: &core::ffi::CStr = c"dpi";
/// C: fontconfig.h 95: `#define FC_PIXEL_SIZE "pixelsize"` (clang-measured: `"pixelsize"`)
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
pub const FC_PIXEL_SIZE: &core::ffi::CStr = c"pixelsize";

#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
unsafe extern "C" {
  /// C: cairo-ft.h: `cairo_font_face_t *cairo_ft_font_face_create_for_pattern (FcPattern *);`
  pub fn cairo_ft_font_face_create_for_pattern (pattern: *mut FcPattern)
    -> *mut cairo_font_face_t;
  /// C: cairo.h: `cairo_scaled_font_t *cairo_scaled_font_create (cairo_font_face_t *,
  /// const cairo_matrix_t *, const cairo_matrix_t *, cairo_font_options_t *);`
  pub fn cairo_scaled_font_create (font_face: *mut cairo_font_face_t,
                                   font_matrix: *const cairo_matrix_t,
                                   ctm: *const cairo_matrix_t,
                                   options: *const cairo_font_options_t)
    -> *mut cairo_scaled_font_t;
  /// C: cairo.h: `void cairo_font_face_destroy (cairo_font_face_t *);`
  pub fn cairo_font_face_destroy (font_face: *mut cairo_font_face_t);
  /// C: cairo.h: `cairo_font_options_t *cairo_font_options_create (void);`
  pub fn cairo_font_options_create () -> *mut cairo_font_options_t;
  /// C: cairo.h: `void cairo_font_options_destroy (cairo_font_options_t *);`
  pub fn cairo_font_options_destroy (options: *mut cairo_font_options_t);
  /// C: cairo.h: `void cairo_scaled_font_extents (cairo_scaled_font_t *,
  /// cairo_font_extents_t *);`
  pub fn cairo_scaled_font_extents (scaled_font: *mut cairo_scaled_font_t,
                                    extents: *mut cairo_font_extents_t);
  /// C: cairo.h: `void cairo_scaled_font_destroy (cairo_scaled_font_t *);`
  pub fn cairo_scaled_font_destroy (scaled_font: *mut cairo_scaled_font_t);
  /// C: cairo.h: `void cairo_scaled_font_text_extents (cairo_scaled_font_t *, const char *,
  /// cairo_text_extents_t *);`
  pub fn cairo_scaled_font_text_extents (scaled_font: *mut cairo_scaled_font_t,
                                         utf8: *const c_char,
                                         extents: *mut cairo_text_extents_t);
  /// C: cairo-xlib.h 49-54: `cairo_surface_t *cairo_xlib_surface_create (Display *dpy,
  /// Drawable drawable, Visual *visual, int width, int height);`
  pub fn cairo_xlib_surface_create (dpy: *mut Display, drawable: Drawable, visual: *mut Visual,
                                    width: c_int, height: c_int) -> *mut cairo_surface_t;
  /// C: cairo.h: `cairo_t *cairo_create (cairo_surface_t *);`
  pub fn cairo_create (target: *mut cairo_surface_t) -> *mut cairo_t;
  /// C: cairo.h: `void cairo_surface_destroy (cairo_surface_t *);`
  pub fn cairo_surface_destroy (surface: *mut cairo_surface_t);
  /// C: cairo.h: `void cairo_destroy (cairo_t *);` (the header's `XftDrawDestroy`, C 51, is this
  /// function's `#define`)
  pub fn cairo_destroy (cr: *mut cairo_t);
  /// C: cairo.h: `void cairo_matrix_init_scale (cairo_matrix_t *, double, double);`
  pub fn cairo_matrix_init_scale (matrix: *mut cairo_matrix_t, sx: c_double, sy: c_double);
  /// C: cairo.h: `void cairo_matrix_init_identity (cairo_matrix_t *);`
  pub fn cairo_matrix_init_identity (matrix: *mut cairo_matrix_t);
  /// C: cairo-ft.h: `void cairo_ft_font_options_substitute (cairo_font_options_t *,
  /// const FcPattern *);`
  pub fn cairo_ft_font_options_substitute (options: *const cairo_font_options_t,
                                           pattern: *mut FcPattern);
  /// C: cairo.h: `void cairo_set_source_rgba (cairo_t *, double, double, double, double);`
  pub fn cairo_set_source_rgba (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double,
                                alpha: c_double);
  /// C: cairo.h: `void cairo_rectangle (cairo_t *, double, double, double, double);`
  pub fn cairo_rectangle (cr: *mut cairo_t, x: c_double, y: c_double, width: c_double,
                          height: c_double);
  /// C: cairo.h: `void cairo_fill (cairo_t *);`
  pub fn cairo_fill (cr: *mut cairo_t);
  /// C: cairo.h: `void cairo_set_scaled_font (cairo_t *, cairo_scaled_font_t *);`
  pub fn cairo_set_scaled_font (cr: *mut cairo_t, scaled_font: *const cairo_scaled_font_t);
  /// C: cairo.h: `void cairo_move_to (cairo_t *, double, double);`
  pub fn cairo_move_to (cr: *mut cairo_t, x: c_double, y: c_double);
  /// C: cairo.h: `void cairo_show_text (cairo_t *, const char *);`
  pub fn cairo_show_text (cr: *mut cairo_t, utf8: *const c_char);
  /// C: fontconfig.h: `FcPattern *FcNameParse (const FcChar8 *);`
  pub fn FcNameParse (name: *const FcChar8) -> *mut FcPattern;
  /// C: fontconfig.h: `void FcConfigSubstitute (FcConfig *, FcPattern *, FcMatchKind);`
  pub fn FcConfigSubstitute (config: *mut FcConfig, pattern: *mut FcPattern,
                             kind: c_int) -> FcBool;
  /// C: fontconfig.h: `void FcDefaultSubstitute (FcPattern *);`
  pub fn FcDefaultSubstitute (pattern: *mut FcPattern);
  /// C: fontconfig.h: `FcPattern *FcFontMatch (FcConfig *, FcPattern *, FcResult *);`
  pub fn FcFontMatch (config: *mut FcConfig, pattern: *mut FcPattern,
                      result: *mut FcResult) -> *mut FcPattern;
  /// C: fontconfig.h: `void FcPatternDestroy (FcPattern *);`
  pub fn FcPatternDestroy (pattern: *mut FcPattern);
  /// C: fontconfig.h: `FcResult FcPatternGetDouble (FcPattern *, const char *, int, double *);`
  pub fn FcPatternGetDouble (p: *const FcPattern, object: *const c_char, n: c_int,
                             d: *mut c_double) -> FcResult;
  /// C: fontconfig.h: `FcBool FcPatternAddDouble (FcPattern *, const char *, double);`
  pub fn FcPatternAddDouble (p: *mut FcPattern, object: *const c_char, d: c_double) -> FcBool;
}

/* ------------------------------------------------------------------------------------------
   The file's definitions, in C's order.
   ------------------------------------------------------------------------------------------ */

/* C 32-51: `XtNoClearRefreshWidget`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XtNoClearRefreshWidget (widget: Widget)
{
  /* C's `XEvent event;` is written member by member at C 38-47; Rust has no partial
     initialisation, so it starts at C's own zero value (PORTING.md 0.1-13).  */
  let mut event: XEvent = core::mem::zeroed ();
  let xexpose: *mut XExposeEvent = &raw mut event.xexpose;

  event.r#type = Expose;
  (*xexpose).serial = 0;
  (*xexpose).send_event = 0;
  (*xexpose).display = XtDisplay (widget);
  (*xexpose).window = XtWindow (widget);
  (*xexpose).x = 0;
  (*xexpose).y = 0;
  (*xexpose).width = (*widget).core.width as c_int;
  (*xexpose).height = (*widget).core.height as c_int;
  (*xexpose).count = 0;

  ((*(*widget).core.widget_class).core_class.expose.unwrap ())
    (widget, &raw mut event, core::ptr::null_mut::<c_void> () as Region);
}

/* C 54-81: `XtApplyToWidgets`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XtApplyToWidgets (w: Widget, proc: XtApplyToWidgetsProc,
                                           arg: XtPointer)
{
  if XtIsComposite (w) != 0
    {
      let cw: CompositeWidget = w as CompositeWidget;
      /* We have to copy the children list before mapping over it, because
	 the procedure might add/delete elements, which would lose badly.
	 */
      let nkids: c_int = (*cw).composite.num_children as c_int;
      let kids: *mut Widget =
        xmalloc (core::mem::size_of::<Widget> () * nkids as usize) as *mut Widget;
      let mut i: c_int;
      memcpy (kids as *mut c_void, (*cw).composite.children as *const c_void,
              core::mem::size_of::<Widget> () * nkids as usize);
      i = 0;
      while i < nkids
/* This prevent us from using gadgets, why is it here? */
/*	if (XtIsWidget (kids [i])) */
	{
	  /* do the kiddies first in case we're destroying */
	  XtApplyToWidgets (*kids.offset (i as isize), proc, arg);
	  (proc.unwrap ()) (*kids.offset (i as isize), arg);
          i += 1;
	}
      xfree (kids as *mut c_void);
    }
}

/* C 84-108: `XtApplyUntilToWidgets`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XtApplyUntilToWidgets (w: Widget, proc: XtApplyUntilToWidgetsProc,
                                                arg: XtPointer) -> *mut c_void
{
  let mut result: *mut c_void;
  if XtIsComposite (w) != 0
    {
      let cw: CompositeWidget = w as CompositeWidget;
      let mut i: c_int;
      i = 0;
      while i < (*cw).composite.num_children as c_int
	{
	  if XtIsWidget (*((*cw).composite.children).offset (i as isize)) != 0
	    {
	      result = (proc.unwrap ()) (*((*cw).composite.children).offset (i as isize), arg);
	      if !result.is_null ()
		{ return result; }
	      result = XtApplyUntilToWidgets (*((*cw).composite.children).offset (i as isize),
	                                      proc, arg);
	      if !result.is_null ()
		{ return result; }
	    }
          i += 1;
	}
    }
  return core::ptr::null_mut ();
}

/* C 111-133: `XtCompositeChildren`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XtCompositeChildren (widget: Widget, number: *mut c_uint) -> *mut Widget
{
  let cw: CompositeWidget = widget as CompositeWidget;
  let mut result: *mut Widget;
  let mut n: c_int;
  let mut i: c_int;

  if XtIsComposite (widget) == 0
    {
      *number = 0;
      return core::ptr::null_mut ();
    }
  n = (*cw).composite.num_children as c_int;
  result = XtMalloc (n as usize * core::mem::size_of::<Widget> ()) as *mut Widget;
  *number = n as c_uint;
  i = 0;
  while i < n
    {
      *result.offset (i as isize) = *((*cw).composite.children).offset (i as isize);
      i += 1;
    }
  return result;
}

/* C 135-139: `XtWidgetBeingDestroyedP`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XtWidgetBeingDestroyedP (widget: Widget) -> Boolean
{
  return (*widget).core.being_destroyed;
}

/* ------------------------------------------------------------------------------------------
   C 141: `#ifdef USE_CAIRO` ... C 291: `#endif /* USE_CAIRO */` - the second arm, carried as
   its own gate (P-86 clause 1).
   ------------------------------------------------------------------------------------------ */

/* C 141-145: `#ifdef USE_CAIRO`, then `<math.h>`, `<cairo-ft.h>` and `<cairo-xlib.h>`; the
   three headers have no mirror in this tree and the surface they declare is above.  */

/* C 147-208: `crxft_font_open_name` (the header's `XftFontOpenName`, C 44).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crxft_font_open_name (dpy: *mut Display, screen: c_int,
                                               name: *const c_char) -> *mut XftFont
{
  /* C's ten uninitialised locals (`dpi`, `result`, `pixel_size`, `font_matrix`, `ctm`,
     `extents`) start at C's own zero value (PORTING.md 0.1-13); C's `pub` is a Rust keyword,
     so the port's local is `pub_` (the `XftFontClose` metavariable's spelling, PORTING.md 0.2),
     and `match` is one too, spelled `r#match`.  */
  let mut pub_: *mut XftFont = core::ptr::null_mut ();
  let mut r#match: *mut FcPattern = core::ptr::null_mut ();
  let pattern: *mut FcPattern = FcNameParse (name as *const FcChar8);
  if !pattern.is_null ()
    {
      FcConfigSubstitute (core::ptr::null_mut (), pattern, FcMatchPattern);
      let mut dpi: c_double = 0.0;
      if FcPatternGetDouble (pattern, FC_DPI.as_ptr (), 0, &raw mut dpi) == FcResultNoMatch
	{
	  let v: *mut c_char = XGetDefault (dpy, c"Xft".as_ptr (), FC_DPI.as_ptr ());
	  if v.is_null () || sscanf (v, c"%lf".as_ptr (), &raw mut dpi) != 1
	    { dpi = ((DisplayHeight (dpy, screen) as c_double * 25.4)
		     / DisplayHeightMM (dpy, screen) as c_double); }
	  FcPatternAddDouble (pattern, FC_DPI.as_ptr (), dpi);
	}
      FcDefaultSubstitute (pattern);
      let mut result: FcResult = 0;
      r#match = FcFontMatch (core::ptr::null_mut (), pattern, &raw mut result);
      FcPatternDestroy (pattern);
    }
  if !r#match.is_null ()
    {
      let font_face: *mut cairo_font_face_t =
	cairo_ft_font_face_create_for_pattern (r#match);
      if !font_face.is_null ()
	{
	  let mut pixel_size: c_double = 0.0;
	  if (FcPatternGetDouble (r#match, FC_PIXEL_SIZE.as_ptr (), 0, &raw mut pixel_size)
	      != FcResultMatch)
	      || pixel_size < 1.0
	    { pixel_size = 10.0; }

	  pub_ = xmalloc (core::mem::size_of::<XftFont> ()) as *mut XftFont;
	  let mut font_matrix: cairo_matrix_t = core::mem::zeroed ();
	  let mut ctm: cairo_matrix_t = core::mem::zeroed ();
	  cairo_matrix_init_scale (&raw mut font_matrix, pixel_size, pixel_size);
	  cairo_matrix_init_identity (&raw mut ctm);
	  let options: *mut cairo_font_options_t = cairo_font_options_create ();
	  cairo_ft_font_options_substitute (options, r#match);
	  (*pub_).scaled_font = cairo_scaled_font_create (font_face, &raw mut font_matrix,
							  &raw mut ctm, options);
	  cairo_font_face_destroy (font_face);
	  cairo_font_options_destroy (options);

	  let mut extents: cairo_font_extents_t = core::mem::zeroed ();
	  cairo_scaled_font_extents ((*pub_).scaled_font, &raw mut extents);
	  (*pub_).ascent = lround (extents.ascent) as c_int;
	  (*pub_).descent = lround (extents.descent) as c_int;
	  (*pub_).height = lround (extents.height) as c_int;
	  (*pub_).max_advance_width = lround (extents.max_x_advance) as c_int;
	}
      FcPatternDestroy (r#match);
    }
  if !pub_.is_null () && (*pub_).height <= 0
    {
      crxft_font_close (pub_);
      pub_ = core::ptr::null_mut ();
    }
  return pub_;
}

/* C 210-215: `crxft_font_close` (the header's `XftFontClose`, C 46).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crxft_font_close (pub_: *mut XftFont)
{
  cairo_scaled_font_destroy ((*pub_).scaled_font);
  xfree (pub_ as *mut c_void);
}

/* C 217-238: `crxft_draw_create` (the header's `XftDrawCreate`, C 48-49).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crxft_draw_create (dpy: *mut Display, drawable: Drawable,
                                            visual: *mut Visual) -> *mut cairo_t
{
  /* The seven locals C 221-223 declares and `XGetGeometry` fills start at C's zero value
     (PORTING.md 0.1-13).  */
  let mut cr: *mut cairo_t = core::ptr::null_mut ();
  let mut root: Window = 0;
  let mut x: c_int = 0;
  let mut y: c_int = 0;
  let mut width: c_uint = 0;
  let mut height: c_uint = 0;
  let mut border_width: c_uint = 0;
  let mut depth: c_uint = 0;

  if XGetGeometry (dpy, drawable, &raw mut root, &raw mut x, &raw mut y, &raw mut width,
		   &raw mut height, &raw mut border_width, &raw mut depth) == 0
    { return core::ptr::null_mut (); }

  let surface: *mut cairo_surface_t =
    cairo_xlib_surface_create (dpy, drawable, visual, width as c_int, height as c_int);
  if !surface.is_null ()
    {
      cr = cairo_create (surface);
      cairo_surface_destroy (surface);
    }

  return cr;
}

/* C 240-247: `crxft_set_source_color` (static).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
unsafe extern "C" fn crxft_set_source_color (cr: *mut cairo_t, color: *const XftColor)
{
  cairo_set_source_rgba (cr, (*color).color.red as c_double / 65535.0,
			 (*color).color.green as c_double / 65535.0,
			 (*color).color.blue as c_double / 65535.0,
			 (*color).color.alpha as c_double / 65535.0);
}

/* C 249-256: `crxft_draw_rect` (the header's `XftDrawRect`, C 52).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crxft_draw_rect (cr: *mut cairo_t, color: *const XftColor, x: c_int,
                                          y: c_int, width: c_uint, height: c_uint)
{
  crxft_set_source_color (cr, color);
  cairo_rectangle (cr, x as c_double, y as c_double, width as c_double, height as c_double);
  cairo_fill (cr);
}

/* C 258-270: `crxft_draw_string` (the header's `XftDrawStringUtf8`, C 55).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crxft_draw_string (cr: *mut cairo_t, color: *const XftColor,
                                            pub_: *mut XftFont, x: c_int, y: c_int,
                                            string: *const FcChar8, len: c_int)
{
  let buf: *mut c_char = xmalloc (len as usize + 1) as *mut c_char;
  memcpy (buf as *mut c_void, string as *const c_void, len as usize);
  *buf.offset (len as isize) = 0;
  crxft_set_source_color (cr, color);
  cairo_set_scaled_font (cr, (*pub_).scaled_font);
  cairo_move_to (cr, x as c_double, y as c_double);
  cairo_show_text (cr, buf);
  xfree (buf as *mut c_void);
}

/* C 272-290: `crxft_text_extents` (the header's `XftTextExtentsUtf8`, C 58-59).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crxft_text_extents (pub_: *mut XftFont, string: *const FcChar8,
                                             len: c_int, extents: *mut XGlyphInfo)
{
  let buf: *mut c_char = xmalloc (len as usize + 1) as *mut c_char;
  memcpy (buf as *mut c_void, string as *const c_void, len as usize);
  *buf.offset (len as isize) = 0;
  let mut text_extents: cairo_text_extents_t = core::mem::zeroed ();
  cairo_scaled_font_text_extents ((*pub_).scaled_font, buf, &raw mut text_extents);
  xfree (buf as *mut c_void);
  (*extents).x = ceil (- text_extents.x_bearing) as c_short;
  (*extents).y = ceil (- text_extents.y_bearing) as c_short;
  (*extents).width =
    (ceil (text_extents.x_bearing + text_extents.width) + (*extents).x as c_double) as c_ushort;
  (*extents).height =
    (ceil (text_extents.y_bearing + text_extents.height) + (*extents).y as c_double) as c_ushort;
  (*extents).xOff = lround (text_extents.x_advance) as c_short;
  (*extents).yOff = lround (text_extents.y_advance) as c_short;
}
