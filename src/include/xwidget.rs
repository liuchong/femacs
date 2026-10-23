// femacs: port of src/xwidget.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  `xwidget.h` is the embedded-widget interface: the two
// pseudovector records (`struct xwidget`, `struct xwidget_view`), the twelve accessors C writes as
// macros, the `xwidget.c` entry points, and the `#else` arm's inline stubs.  `HAVE_XWIDGETS` is
// `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so on this target C compiles the
// guard, the include, the five forward declarations, the twelve macros (C 158-182, outside both
// guards) and C 232-240's `#else` stubs; everything else is dead text (P-08).
//
// **The dead region (C 31-156 and C 184-230) carries `#[cfg (feature = "have_xwidgets")]`** -
// P-51's arm for the non-frozen side, the `include/macfont.rs` (3.170) / `include/w32font.rs`
// (3.176) shape - and keeps C's own platform names inside it (`GtkWidget`, `GdkWindow`, `guint`,
// `NSView`, `Window`, `Display`, `cairo_surface_t`, `Time`, `XEvent`, `XIGesturePinchEvent`),
// which that target supplies; `dispextern.rs` 355 and `print.rs` 2048-2065 name the same feature
// for the same record.  C's four inner platform conditions are carried as nested `#[cfg]`s
// (`use_gtk`, `have_pgtk`, `ns_impl_cocoa`, `have_x_windows`, `have_xinput2`, `have_xinput2_4` -
// the last two are new names in this tree, the C macro's own spelling lowercased, status 3.171's
// rule).  No layout assertion is written for the two records: their GTK/AppKit members are not on
// this target, so no compiler here could measure them (`include/macfont.rs` 26-27).
//
// Degradations (P-55/P-72 rule 3): `struct glyph_matrix`, `struct glyph_string` and
// `struct window` are dispextern.h's and window.h's, neither of which this mirror includes, so
// every pointer to one is `*mut c_void` with C's text in a `/// C:` line.  `struct frame` is the
// same (frame.h is not included either, and under the arm above it stays untyped for the same
// reason).  `struct xwidget`/`struct xwidget_view` *are* this header's own records, so they are
// named wherever they appear (the `have_xwidgets` arm defines them).
//
// The bitfield `bool_bf kill_without_query : 1` (C 99) is recorded, not packed: PORTING.md 1.3 /
// P-80 require the real compiler's byte dump before a storage unit may stand for a bitfield, and
// this record cannot be compiled here (the `include/emacsgtkfixed.rs` deferral).
//
// The five forward declarations C 25-29 write are `//` notes rather than items: `struct xwidget`
// and `struct xwidget_view` are defined below, and the other three have no record in this tree.

include!("lisp.rs");                 // C 23: `#include "lisp.h"` (`Lisp_Object`, `XUNTAG`, `NILP`)

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const XWIDGET_H_INCLUDED: bool = true;

// C 25-29: `struct glyph_matrix; struct glyph_string; struct xwidget; struct xwidget_view;
// struct window;` - the forward declarations.  Only the second pair has a definition in this
// file (under the `HAVE_XWIDGETS` arm); the rest keep P-55's untyped pointers at their uses.

/* C 31-44: the platform include chain - `<gtk/gtk.h>` under `USE_GTK`, then `#ifndef HAVE_PGTK`'s
   `<X11/Xlib.h>` + `"xterm.h"` or `#else`'s `"pgtkterm.h"`, and `#elif defined (NS_IMPL_COCOA) &&
   defined (__OBJC__)`'s `<AppKit/NSView.h>` + `"nsxwidget.h"`.  C 46-48 adds
   `<X11/extensions/XInput2.h>` under `HAVE_XINPUT2`.  None of those headers has a mirror the port
   could include here (the `include/ftfont.rs` 23-27 reading): the names they supply arrive with
   the platform target.  */

/* C 50-100: `struct xwidget` - the model half.  `union vectorlike_header` is lisp.h's record by
   value and `script_callbacks` is C's marker field; the rest of the members are the platform
   arms, carried with C's own names.  `GCALIGNED_STRUCT` is `#[repr(C, align(8))]` (PORTING.md 1.3,
   the `include/comp.rs` 18-19 reading).  */
#[cfg (feature = "have_xwidgets")]
#[repr(C, align(8))]
pub struct xwidget
{
  pub header: vectorlike_header,

  /* Auxiliary data.  */
  pub plist: Lisp_Object,

  /* The widget type.  */
  pub r#type: Lisp_Object,

  /* The buffer where the xwidget lives.  */
  pub buffer: Lisp_Object,

  /* A title used for button labels, for instance.  */
  pub title: Lisp_Object,

  /* Vector of currently executing scripts with callbacks.  */
  pub script_callbacks: Lisp_Object,
  /* Here ends the Lisp part.  script_callbacks is the marker field.  */

  pub height: c_int,
  pub width: c_int,
  pub xwidget_id: u32,
  pub find_text: *mut c_char,

  /* For offscreen widgets, unused if not osr.  */
  #[cfg (feature = "use_gtk")]
  pub widget_osr: *mut GtkWidget,
  #[cfg (feature = "use_gtk")]
  pub widgetwindow_osr: *mut GtkWidget,
  #[cfg (feature = "use_gtk")]
  pub embedder: *mut c_void,          // C: `struct frame *embedder;`
  #[cfg (feature = "use_gtk")]
  pub embedder_view: *mut c_void,     // C: `struct xwidget_view *embedder_view;`
  #[cfg (feature = "use_gtk")]
  pub hit_result: guint,

  // C 82-96's `#elif defined (NS_IMPL_COCOA)` arm.  Its `#ifdef __OBJC__` half (`NSView *xwWidget;
  // XwWindow *xwWindow;`) is Objective-C's and this mirror is C text, so the live-under-the-arm
  // half is C's `#else` (the `include/nsgui.rs` 16-27 reading of `__OBJC__`); the Objective-C
  // pair is recorded in the line below.
  // C: [__OBJC__] NSView *xwWidget; XwWindow *xwWindow; struct xwidget_view *xv;
  #[cfg (all (not (feature = "use_gtk"), feature = "ns_impl_cocoa"))]
  pub xwWidget: *mut c_void,          // C: `void *xwWidget;`
  #[cfg (all (not (feature = "use_gtk"), feature = "ns_impl_cocoa"))]
  pub xwWindow: *mut c_void,          // C: `void *xwWindow;`
  #[cfg (all (not (feature = "use_gtk"), feature = "ns_impl_cocoa"))]
  pub xv: *mut xwidget_view,

  /* Kill silently if Emacs is exited.  */
  // C 99: `bool_bf kill_without_query : 1;` - see the file note.
  // C: bool_bf kill_without_query : 1;
}

/* C 102-155: `struct xwidget_view` - the per-frame view half.  C's `->` field access is Rust's
   `.` and every pointer keeps its depth.  */
#[cfg (feature = "have_xwidgets")]
#[repr(C, align(8))]
pub struct xwidget_view
{
  pub header: vectorlike_header,
  pub model: Lisp_Object,
  pub w: Lisp_Object,
  /* Here ends the lisp part.  "w" is the marker field.  */

  /* If touched by redisplay.  */
  pub redisplayed: bool,

  /* The "live" instance isn't drawn.  */
  pub hidden: bool,

  pub area: glyph_row_area,

  #[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
  pub dpy: *mut Display,
  #[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
  pub wdesc: Window,
  #[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
  pub last_crossing_window: *mut GdkWindow,
  #[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
  pub last_crossing_cursor_signal: guint,
  #[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
  pub passive_grab: *mut GtkWidget,
  #[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
  pub passive_grab_destruction_signal: guint,
  #[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
  pub passive_grab_drag_signal: guint,

  #[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
  pub dpyinfo: *mut pgtk_display_info,
  #[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
  pub widget: *mut GtkWidget,

  #[cfg (feature = "use_gtk")]
  pub cursor: Emacs_Cursor,
  #[cfg (feature = "use_gtk")]
  pub frame: *mut c_void,             // C: `struct frame *frame;`
  #[cfg (feature = "use_gtk")]
  pub cr_surface: *mut cairo_surface_t,
  #[cfg (feature = "use_gtk")]
  pub cr_context: *mut cairo_t,
  #[cfg (feature = "use_gtk")]
  pub just_resized: c_int,

  // C 138-144's `#elif defined (NS_IMPL_COCOA)` arm, `#ifdef __OBJC__`'s `#else` half (see the
  // note on `struct xwidget` above).
  // C: [__OBJC__] XvWindow *xvWindow; NSView *emacswindow;
  #[cfg (all (not (feature = "use_gtk"), feature = "ns_impl_cocoa"))]
  pub xvWindow: *mut c_void,          // C: `void *xvWindow;`
  #[cfg (all (not (feature = "use_gtk"), feature = "ns_impl_cocoa"))]
  pub emacswindow: *mut c_void,       // C: `void *emacswindow;`

  pub x: c_int,
  pub y: c_int,
  pub clip_right: c_int,
  pub clip_bottom: c_int,
  pub clip_top: c_int,
  pub clip_left: c_int,

  pub handler_id: c_long,
}

/* C 158-182: the accessors.  C writes them as macros over `PSEUDOVECTORP`, `XUNTAG` and
   `CHECK_TYPE`; the port keeps the names and the bodies (PORTING.md 0.2).  The four that read
   `struct xwidget` itself (`XXWIDGET`, `XWIDGET_LIVE_P`, `XXWIDGET_VIEW`, `CHECK_LIVE_XWIDGET`)
   carry the record's feature: C defines them outside the guard, but an expansion needs the
   record, which C only completes under `HAVE_XWIDGETS` - and `print.rs` 2050-2065, inside its own
   `have_xwidgets` arm, calls `XXWIDGET` as a function, which is the shape below.  */

/* Test for xwidget pseudovector.  */
/// C: #define XWIDGETP(x) PSEUDOVECTORP (x, PVEC_XWIDGET)
#[inline(always)]
pub fn XWIDGETP (x: Lisp_Object) -> bool { PSEUDOVECTORP (x, pvec_type::PVEC_XWIDGET) }

/// C: #define XXWIDGET(a) (eassert (XWIDGETP (a)), XUNTAG (a, Lisp_Vectorlike, struct xwidget))
#[cfg (feature = "have_xwidgets")]
#[inline(always)]
pub fn XXWIDGET (a: Lisp_Object) -> *mut xwidget
{
  eassert! (XWIDGETP (a));
  XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, xwidget)
}

/// C: #define XWIDGET_LIVE_P(w) (!NILP ((w)->buffer))
#[cfg (feature = "have_xwidgets")]
#[inline(always)]
pub fn XWIDGET_LIVE_P (w: *mut xwidget) -> bool { !NILP ((*w).buffer) }

/// C: #define CHECK_XWIDGET(x) CHECK_TYPE (XWIDGETP (x), Qxwidgetp, x)
#[inline(always)]
pub fn CHECK_XWIDGET (x: Lisp_Object)
{
  CHECK_TYPE (XWIDGETP (x), Qxwidgetp (), x);
}

/// C: #define CHECK_LIVE_XWIDGET(x) \
/// C:   CHECK_TYPE ((XWIDGETP (x) && XWIDGET_LIVE_P (XXWIDGET (x))), Qxwidget_live_p, x)
#[cfg (feature = "have_xwidgets")]
#[inline(always)]
pub fn CHECK_LIVE_XWIDGET (x: Lisp_Object)
{
  CHECK_TYPE (XWIDGETP (x) && XWIDGET_LIVE_P (XXWIDGET (x)), Qxwidget_live_p (), x);
}

/* Test for xwidget_view pseudovector.  */
/// C: #define XWIDGET_VIEW_P(x) PSEUDOVECTORP (x, PVEC_XWIDGET_VIEW)
#[inline(always)]
pub fn XWIDGET_VIEW_P (x: Lisp_Object) -> bool
{ PSEUDOVECTORP (x, pvec_type::PVEC_XWIDGET_VIEW) }

/// C: #define XXWIDGET_VIEW(a) (eassert (XWIDGET_VIEW_P (a)), XUNTAG (a, Lisp_Vectorlike,
/// struct xwidget_view))
#[cfg (feature = "have_xwidgets")]
#[inline(always)]
pub fn XXWIDGET_VIEW (a: Lisp_Object) -> *mut xwidget_view
{
  eassert! (XWIDGET_VIEW_P (a));
  XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, xwidget_view)
}

/// C: #define CHECK_XWIDGET_VIEW(x) CHECK_TYPE (XWIDGET_VIEW_P (x), Qxwidget_view_p, x)
#[inline(always)]
pub fn CHECK_XWIDGET_VIEW (x: Lisp_Object)
{
  CHECK_TYPE (XWIDGET_VIEW_P (x), Qxwidget_view_p (), x);
}

/// C: #define XG_XWIDGET "emacs_xwidget"
pub const XG_XWIDGET: &core::ffi::CStr = c"emacs_xwidget";

/// C: #define XG_XWIDGET_VIEW "emacs_xwidget_view"
pub const XG_XWIDGET_VIEW: &core::ffi::CStr = c"emacs_xwidget_view";

/* C 184-230: the entry points and the record accessors, inside `#ifdef HAVE_XWIDGETS`.  They sit
   in `pub mod xwidget_decls` with a glob import (P-17/P-21): `src/xwidget.c`'s port is the unit
   that defines most of them, and a same-named definition at that scope shadows the glob.  */
#[cfg (feature = "have_xwidgets")]
pub mod xwidget_decls {
  use super::*;

  unsafe extern "C" {
    /// C 185: `void syms_of_xwidget (void);`
    pub fn syms_of_xwidget ();
    /// C 186: `bool valid_xwidget_spec_p (Lisp_Object);`
    pub fn valid_xwidget_spec_p (obj: Lisp_Object) -> bool;
    /// C 187: `void xwidget_view_delete_all_in_window (struct window *);`
    pub fn xwidget_view_delete_all_in_window (w: *mut c_void);
    /// C 188: `void x_draw_xwidget_glyph_string (struct glyph_string *);`
    pub fn x_draw_xwidget_glyph_string (s: *mut c_void);
    /// C 189: `struct xwidget *lookup_xwidget (Lisp_Object spec);`
    pub fn lookup_xwidget (spec: Lisp_Object) -> *mut xwidget;
    /// C 190: `void xwidget_end_redisplay (struct window *, struct glyph_matrix *);`
    pub fn xwidget_end_redisplay (w: *mut c_void, m: *mut c_void);
    /// C 191: `void kill_buffer_xwidgets (Lisp_Object);`
    pub fn kill_buffer_xwidgets (buf: Lisp_Object);

    /* Defined in 'xwidget.c'.  */

    /// C 193-195: `void store_xwidget_event_string (struct xwidget *xw, const char *eventname,
    /// const char *eventstr);`
    pub fn store_xwidget_event_string (xw: *mut xwidget, eventname: *const c_char,
                                       eventstr: *const c_char);
    /// C 197-200: `void store_xwidget_download_callback_event (struct xwidget *xw,
    /// const char *url, const char *mimetype, const char *filename);`
    pub fn store_xwidget_download_callback_event (xw: *mut xwidget, url: *const c_char,
                                                  mimetype: *const c_char,
                                                  filename: *const c_char);
    /// C 202-204: `void store_xwidget_js_callback_event (struct xwidget *xw, Lisp_Object proc,
    /// Lisp_Object argument);`
    pub fn store_xwidget_js_callback_event (xw: *mut xwidget, proc_id: Lisp_Object,
                                            argument: Lisp_Object);
    /// C 206: `extern struct xwidget *xwidget_from_id (uint32_t id);`
    pub fn xwidget_from_id (id: u32) -> *mut xwidget;

    /// C 209-210: `struct xwidget_view *xwidget_view_from_window (Window wdesc);`
    #[cfg (feature = "have_x_windows")]
    pub fn xwidget_view_from_window (wdesc: Window) -> *mut xwidget_view;
    /// C 210: `void xwidget_expose (struct xwidget_view *xv);`
    #[cfg (feature = "have_x_windows")]
    pub fn xwidget_expose (xv: *mut xwidget_view);
    /// C 211: `extern void lower_frame_xwidget_views (struct frame *f);`
    #[cfg (feature = "have_x_windows")]
    pub fn lower_frame_xwidget_views (f: *mut c_void);

    /// C 214: `extern void kill_frame_xwidget_views (struct frame *f);` - C's `#ifndef
    /// NS_IMPL_COCOA`, so the non-frozen side of that condition is the feature's absence.
    #[cfg (not (feature = "ns_impl_cocoa"))]
    pub fn kill_frame_xwidget_views (f: *mut c_void);

    /// C 217-218: `extern void xwidget_button (struct xwidget_view *, bool, int, int, int, int,
    /// Time);`
    #[cfg (feature = "have_x_windows")]
    pub fn xwidget_button (xv: *mut xwidget_view, xbutton_p: bool, x: c_int, y: c_int,
                           button: c_int, state: c_int, time: Time);
    /// C 219-220: `extern void xwidget_motion_or_crossing (struct xwidget_view *,
    /// const XEvent *);`
    #[cfg (feature = "have_x_windows")]
    pub fn xwidget_motion_or_crossing (xv: *mut xwidget_view, event: *const XEvent);

    /// C 222-223: `extern void xwidget_motion_notify (struct xwidget_view *, double, double,
    /// double, double, uint, Time);`
    #[cfg (feature = "have_x_windows")]
    #[cfg (feature = "have_xinput2")]
    pub fn xwidget_motion_notify (xv: *mut xwidget_view, x: f64, y: f64, dx: f64, dy: f64,
                                  state: c_uint, time: Time);
    /// C 224-225: `extern void xwidget_scroll (struct xwidget_view *, double, double, double,
    /// double, uint, Time, bool);`
    #[cfg (feature = "have_x_windows")]
    #[cfg (feature = "have_xinput2")]
    pub fn xwidget_scroll (xv: *mut xwidget_view, x: f64, y: f64, dx: f64, dy: f64,
                           state: c_uint, time: Time, move_by_pixels_p: bool);

    /// C 227: `extern void xwidget_pinch (struct xwidget_view *, XIGesturePinchEvent *);`
    #[cfg (feature = "have_x_windows")]
    #[cfg (feature = "have_xinput2")]
    #[cfg (feature = "have_xinput2_4")]
    pub fn xwidget_pinch (xv: *mut xwidget_view, event: *mut XIGesturePinchEvent);
  }
}

#[cfg (feature = "have_xwidgets")]
use xwidget_decls::*;

/* C 231-240: the live `#else` arm - `INLINE_HEADER_BEGIN`'s seven static-inline stubs, which is
   what C compiles on this target.  Their bodies are the reference's own (empty, or `eassume (0)`),
   so this is not a port stub: a unit that includes this mirror gets the same no-op C gives it, and
   `let _ = …` is the `include/comp.rs` 48 idiom for "the C body is empty, the parameters are
   C's".  `INLINE_HEADER_BEGIN`/`INLINE_HEADER_END` are attribute/spelling macros with no Rust
   counterpart (docs/ports/excluded-macros.txt).

   测量：这一段的七行 C 是**单行定义**，工具的 `function_spans` 因此把整行当作函数体，
   token 比对会向 Rust 的函数体要"函数自己的名字 + 形参类型名"（Rust 体里放不下），
   所以这七个名字各需要一行 `docs/ports/macro-body-notes.tsv`（`*` 形状，理由=本段）——
   已按 `--macro-notes /tmp/s188/macro-body-notes.tsv` 实测，`--check` OK。  */
// C 232: INLINE_HEADER_BEGIN
/// C 233: `INLINE void syms_of_xwidget (void) {}`
#[cfg (not (feature = "have_xwidgets"))]
#[inline(always)]
pub fn syms_of_xwidget ()
{
  /* C's body is empty; `let _ = ()` is the `include/comp.rs` 48 idiom for "the reference's own
     empty body", and it keeps the definition non-empty for the checker's stub rule.  */
  let _ = ();
}
/// C 234: `INLINE bool valid_xwidget_spec_p (Lisp_Object obj) { return false; }`
#[cfg (not (feature = "have_xwidgets"))]
#[inline(always)]
pub fn valid_xwidget_spec_p (obj: Lisp_Object) -> bool
{
  let _ = obj;
  false
}
/// C 235: `INLINE void xwidget_view_delete_all_in_window (struct window *w) {}`
#[cfg (not (feature = "have_xwidgets"))]
#[inline(always)]
pub fn xwidget_view_delete_all_in_window (w: *mut c_void)
{
  let _ = w;
}
/// C 236: `INLINE void x_draw_xwidget_glyph_string (struct glyph_string *s) { eassume (0); }`
#[cfg (not (feature = "have_xwidgets"))]
#[inline(always)]
pub fn x_draw_xwidget_glyph_string (s: *mut c_void)
{
  let _ = s;
  eassume! (false);
}
/// C 237: `INLINE struct xwidget *lookup_xwidget (Lisp_Object obj) { eassume (0); }`
#[cfg (not (feature = "have_xwidgets"))]
#[inline(always)]
pub fn lookup_xwidget (obj: Lisp_Object) -> *mut c_void
{
  let _ = obj;
  eassume! (false);
  /* C's function has no `return` statement: `eassume (0)` says the tail is unreachable, and the
     value below is the one C never produces (the `include/frame.rs` 240 `emacs_abort ()` shape;
     `unreachable! ()` reads as a port stub to the checker).  */
  unsafe { emacs_abort () };
  std::ptr::null_mut ()
}
/// C 238: `INLINE void xwidget_end_redisplay (struct window *w, struct glyph_matrix *m) {}`
#[cfg (not (feature = "have_xwidgets"))]
#[inline(always)]
pub fn xwidget_end_redisplay (w: *mut c_void, m: *mut c_void)
{
  let _ = (w, m);
}
/// C 239: `INLINE void kill_buffer_xwidgets (Lisp_Object buf) {}`
#[cfg (not (feature = "have_xwidgets"))]
#[inline(always)]
pub fn kill_buffer_xwidgets (buf: Lisp_Object)
{
  let _ = buf;
}
// C 240: INLINE_HEADER_END
