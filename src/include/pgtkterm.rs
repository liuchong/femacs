// femacs: port of src/pgtkterm.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1-2): declarations only - types, constants, `macro_rules!` and
// `unsafe extern "C"` items.  Nothing here produces a symbol.
//
// The whole body is one `#ifdef HAVE_PGTK` (C 29 at the top, `#endif /* HAVE_PGTK */` at C 680),
// and `HAVE_PGTK` is `undef` in the frozen configuration
// (`docs/baseline/config/darwin-arm64.macros.tsv`), so C 29-680 carries no compilation on this
// target.  P-86 ① transcribes it anyway - **both arms of every `#if`** - and `check-cond-eval.py
// src/pgtkterm.h` decides every condition it crosses: `8 conditions, 0 unknown, 0 disagreements
// [VERIFIED]`.
//
// The five `#include` lines at C 23-27 and the three cairo-surface probes at C 33-41 are carried
// as `include!` edges / gated arms below; `<gtk/gtk.h>` (C 31) reaches the port through the GTK
// mirror `include/gtk.rs`, which is the third-party surface's single home (P-90 ①②).
//
// **No GTK/GDK tree exists on this machine** (`ls -d /opt/homebrew/include/gtk-3.0
// /opt/homebrew/include/gtk-4.0` -> both absent; `pkg-config` is not installed; no `gtk/gtk.h` or
// `gdk/gdk.h` under `/opt/homebrew`, `/usr/local` or `/Library`), so every record below cites the
// header alone (no line, no probe) and **every size and offset of theirs is UNVERIFIED** - no
// layout assertion is written for `pgtk_bitmap_record`, `pgtk_device_t`, `pgtk_touch_point`,
// `scroll_bar`, `pgtk_display_info`, `pgtk_output` or `selection_input_event`.  That is the same
// machine limit `docs/ports/include-dispextern.md`'s 3.278 4 and `docs/ports/pgtkterm.md` record;
// the missing toolchain is named in `docs/ports/include-pgtkterm.md`'s `## Owed`.

// pgtkterm.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _PGTKTERM_H_: bool = true;

/* C 23-27: the five unconditional includes.  C's text is live (it sits above the `HAVE_PGTK`
   guard), but every name the *body* takes from them is inside it, so the edges carry the same
   gate the body does (P-86 ⑧).  `lisp.h`'s `Lisp_Object`/`vectorlike_header`/`ptrdiff_t`,
   `systime.h`'s `Time`, `termhooks.h`'s `event_kind`/`EVENT_KIND_WIDTH` and `<X11/Xresource.h>`'s
   `XrmDatabase` reach the body through those headers, so their mirrors are named here too
   (P-72 ④: a mirror may only name types it declares or reaches through its own `include!`s).  */
#[cfg (feature = "have_pgtk")]
include!("dispextern.rs");      // C 23: `#include "dispextern.h"`
#[cfg (feature = "have_pgtk")]
include!("frame.rs");           // C 24: `#include "frame.h"`
#[cfg (feature = "have_pgtk")]
include!("character.rs");       // C 25: `#include "character.h"`
#[cfg (feature = "have_pgtk")]
include!("font.rs");            // C 26: `#include "font.h"`
#[cfg (feature = "have_pgtk")]
include!("sysselect.rs");       // C 27: `#include "sysselect.h"`
#[cfg (feature = "have_pgtk")]
include!("lisp.rs");            // lisp.h: `Lisp_Object`, `vectorlike_header`, `ptrdiff_t`
#[cfg (feature = "have_pgtk")]
include!("systime.rs");         // systime.h: `Time`
#[cfg (feature = "have_pgtk")]
include!("termhooks.rs");       // termhooks.h: `event_kind`, `EVENT_KIND_WIDTH`
/* This edge is deliberately **ungated** while every other edge here carries `have_pgtk`: the
   materializer decides which `include!` edges to follow before the probe's `--cfg`s reach rustc
   (P-89's gated-edge trap), so a gated edge is dropped exactly in the arms that need it - measured:
   `Window`/`XrmDatabase`/`XRectangle` were absent from `build/probes/pgtkim/_root.rs` until this
   line lost its `#[cfg]`, and `include/xlib.rs`'s own items each carry their arm's gate
   (status 3.496).  */
include!("xlib.rs");            // <X11/Xresource.h>: `XrmDatabase`
/* C 31: `#include <gtk/gtk.h>` - the edge to the surface's mirror, `include/gtk.rs`, is **owed**:
   three landed units that already reach this mirror (`src/image.rs`, `src/xwidget.rs`,
   `src/pgtkfns.rs`) still carry their own copies of GTK/GDK/cairo names, and the edge would land
   both in one TU (measured: 30 `E0428`s in the `mirror-dupes` reading).  It goes in with their
   P-88 ⑤ move - `## Owed` of `docs/ports/include-pgtkterm.md`.  The two units this round owns
   reach `include/gtk.rs` through their own edges.  */

/* C 33-41: `#ifdef CAIRO_HAS_PDF_SURFACE` / `#include <cairo-pdf.h>` / `#endif` and its `_PS` and
   `_SVG` twins.  The three headers are cairo's (no file in the reference tree, no mirror), and the
   three features are **not registered** in `xtask/Cargo.toml`, so both arms are carried as
   C's text and the gap is named in `docs/ports/include-pgtkterm.md`'s `## Owed`.  */

/* C 43-49: `struct pgtk_bitmap_record` - `char *file; int refcount; int height, width, depth;
   cairo_pattern_t *pattern;`.  **UNVERIFIED**.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct pgtk_bitmap_record
{
  pub file: *mut c_char,              // C 45: `char *file;`
  pub refcount: c_int,                // C 46: `int refcount;`
  pub height: c_int,                  // C 47: `int height, width, depth;`
  pub width: c_int,
  pub depth: c_int,
  pub pattern: *mut cairo_pattern_t,  // C 48: `cairo_pattern_t *pattern;`
}

/* C 51-64: `struct pgtk_device_t`.  **UNVERIFIED**.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct pgtk_device_t
{
  pub name: Lisp_Object,                // C 54: `Lisp_Object name;`
  pub seat: *mut GdkSeat,               // C 57: `GdkSeat *seat;`
  pub device: *mut GdkDevice,           // C 60: `GdkDevice *device;`
  pub next: *mut pgtk_device_t,         // C 63: `struct pgtk_device_t *next;`
}

/* C 66-83: `struct pgtk_touch_point`.  **UNVERIFIED**.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct pgtk_touch_point
{
  pub local_detail: EMACS_INT,               // C 69: `EMACS_INT local_detail;`
  pub frame: *mut frame,                     // C 72: `struct frame *frame;`
  pub next: *mut pgtk_touch_point,           // C 75: `struct pgtk_touch_point *next;`
  pub number: *mut GdkEventSequence,         // C 79: `GdkEventSequence *number;`
  pub x: c_int,                              // C 82: `int x, y;`
  pub y: c_int,
}

/// C 85-91: the six colour macros; C's own bodies, kept verbatim (PORTING.md 2.1).  C 85-86 widen
/// with `<< 16`/`<< 8`, so the operands are C's `int`s: the port keeps C's expression and lets the
/// call site's type decide (`c_ulong` at `FRAME_BACKGROUND_PIXEL`'s uses).
#[cfg (feature = "have_pgtk")]
macro_rules! RGB_TO_ULONG {
  ($r:expr, $g:expr, $b:expr) => { ((($r) << 16) | (($g) << 8) | ($b)) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! ARGB_TO_ULONG {
  ($a:expr, $r:expr, $g:expr, $b:expr) => { ((($a) << 24) | (($r) << 16) | (($g) << 8) | ($b)) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! ALPHA_FROM_ULONG {
  ($color:expr) => { (($color) >> 24) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! RED_FROM_ULONG {
  ($color:expr) => { ((($color) >> 16) & 0xff) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! GREEN_FROM_ULONG {
  ($color:expr) => { ((($color) >> 8) & 0xff) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! BLUE_FROM_ULONG {
  ($color:expr) => { (($color) & 0xff) };
}

/* C 93-144: this header's own `struct scroll_bar` (a different record from `xterm.h`'s, which
   `include/xterm.rs` 1283 carries for the X arm; the two are never in one TU, because the arm
   gates are exclusive).  **UNVERIFIED**.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct scroll_bar
{
  /// C 96: `union vectorlike_header header;` - the shared vector-like header (P-67/P-80 spell it
  /// as `lisp.h`'s record, the `include/xterm.rs` 1285 shape).
  pub header: vectorlike_header,
  pub window: Lisp_Object,          // C 99: `Lisp_Object window;`
  pub next: Lisp_Object,            // C 102: `Lisp_Object next, prev;`
  pub prev: Lisp_Object,
  pub x_window: Window,             // C 107: `Window x_window;`
  pub top: c_int,                   // C 111: `int top, left, width, height;`
  pub left: c_int,
  pub width: c_int,
  pub height: c_int,
  pub start: c_int,                 // C 124: `int start, end;`
  pub end: c_int,
  pub dragging: c_int,              // C 130: `int dragging;`
  /// C 132-135: `#if defined (USE_TOOLKIT_SCROLL_BARS) && defined (USE_LUCID)` /
  /// `enum scroll_bar_part last_seen_part;` / `#endif`.  Both arms are carried; the feature names
  /// are unregistered (`## Owed`).
  #[cfg (all (feature = "use_toolkit_scroll_bars", feature = "use_lucid"))]
  pub last_seen_part: c_int,        // C: `enum scroll_bar_part last_seen_part;` (enum ABI, 1.3)
  /// C 137-140: `#if defined (USE_TOOLKIT_SCROLL_BARS) && !defined (USE_GTK)` / `int whole;` /
  /// `#endif`.
  #[cfg (all (feature = "use_toolkit_scroll_bars", not (feature = "use_gtk")))]
  pub whole: c_int,                 // C: `int whole;`
  pub horizontal: bool,             // C 143: `bool horizontal;`
}

/* C 146-281: `struct pgtk_display_info`.  **UNVERIFIED** - no GTK tree here, so no size or offset
   assertion is written and every member is C's own name.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct pgtk_display_info
{
  pub next: *mut pgtk_display_info,                 // C 149: `struct pgtk_display_info *next;`
  pub terminal: *mut terminal,                      // C 152: `struct terminal *terminal;`
  /// C 154-161: C's *anonymous* `union { GdkDisplay *gdpy; GdkDisplay *display; };` - both
  /// members are one pointer at offset 0, so the port keeps C's first member's name as the field
  /// and records the alias (`## Deviations` in `docs/ports/include-pgtkterm.md`).
  pub gdpy: *mut GdkDisplay,
  pub touchpoints: *mut pgtk_touch_point,           // C 164: `struct pgtk_touch_point *touchpoints;`
  pub name_list_element: Lisp_Object,               // C 167: `Lisp_Object name_list_element;`
  pub reference_count: c_int,                       // C 170: `int reference_count;`
  pub x_id: c_uint,                                 // C 173: `unsigned x_id;`
  pub x_id_name: *mut c_char,                       // C 176: `char *x_id_name;`
  pub n_fonts: c_int,                               // C 179: `int n_fonts;`
  pub smallest_char_width: c_int,                   // C 182: `int smallest_char_width;`
  pub smallest_font_height: c_int,                  // C 185: `int smallest_font_height;`
  pub bitmaps: *mut pgtk_bitmap_record,             // C 187: `struct pgtk_bitmap_record *bitmaps;`
  pub bitmaps_size: ptrdiff_t,                      // C 188: `ptrdiff_t bitmaps_size;`
  pub bitmaps_last: ptrdiff_t,                      // C 189: `ptrdiff_t bitmaps_last;`
  pub resx: c_double,                               // C 192: `double resx, resy;`
  pub resy: c_double,
  pub grabbed: c_int,                               // C 195: `int grabbed;`
  pub n_planes: c_int,                              // C 197: `int n_planes;`
  pub color_p: c_int,                               // C 199: `int color_p;`
  pub icon_bitmap_id: ptrdiff_t,                    // C 203: `ptrdiff_t icon_bitmap_id;`
  pub root_window: Window,                          // C 205: `Window root_window;`
  pub rdb: XrmDatabase,                             // C 208: `XrmDatabase rdb;`
  pub vertical_scroll_bar_cursor: Emacs_Cursor,     // C 211: `Emacs_Cursor vertical_scroll_bar_cursor;`
  pub horizontal_scroll_bar_cursor: Emacs_Cursor,   // C 214: `... horizontal_scroll_bar_cursor;`
  pub mouse_highlight: Mouse_HLInfo,                // C 218: `Mouse_HLInfo mouse_highlight;`
  pub highlight_frame: *mut frame,                  // C 220: `struct frame *highlight_frame;`
  pub x_focus_frame: *mut frame,                    // C 221: `struct frame *x_focus_frame;`
  pub x_focus_event_frame: *mut frame,              // C 227: `struct frame *x_focus_event_frame;`
  pub last_mouse_frame: *mut frame,                 // C 230: `struct frame *last_mouse_frame;`
  pub last_mouse_motion_frame: *mut frame,          // C 233: `struct frame *last_mouse_motion_frame;`
  pub last_mouse_motion_x: c_int,                   // C 237: `int last_mouse_motion_x;`
  pub last_mouse_motion_y: c_int,                   // C 238: `int last_mouse_motion_y;`
  pub last_mouse_glyph: XRectangle,                 // C 241: `XRectangle last_mouse_glyph;`
  pub last_mouse_movement_time: Time,               // C 244: `Time last_mouse_movement_time;`
  pub last_user_time: guint32,                      // C 247: `guint32 last_user_time;`
  pub last_mouse_scroll_bar: *mut c_void,           // C 250: `void *last_mouse_scroll_bar;`
  pub invisible_cursor: Emacs_Cursor,               // C 253: `Emacs_Cursor invisible_cursor;`
  pub xg_cursor: *mut GdkCursor,                    // C 256: `GdkCursor *xg_cursor;`
  pub devices: *mut pgtk_device_t,                  // C 259: `struct pgtk_device_t *devices;`
  pub last_mouse_glyph_frame: *mut frame,           // C 262: `struct frame *last_mouse_glyph_frame;`
  pub last_click_event: *mut GdkEvent,              // C 265: `GdkEvent *last_click_event;`
  /// C 268-272: the anonymous `struct { GtkIMContext *context; struct frame *focused_frame; } im;`
  /// - P-67 names it after its member.
  pub im: pgtk_display_info_im,
  /// C 274-278: the anonymous `struct { double acc_x, acc_y; double x_per_char, y_per_line; }
  /// scroll;`.
  pub scroll: pgtk_display_info_scroll,
  pub connection: c_int,                            // C 280: `int connection;`
}
/// C 268-272: the anonymous `struct { GtkIMContext *context; struct frame *focused_frame; } im;`.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct pgtk_display_info_im
{
  pub context: *mut GtkIMContext,   // C: `GtkIMContext *context;`
  pub focused_frame: *mut frame,    // C: `struct frame *focused_frame;`
}
/// C 274-278: the anonymous `struct { double acc_x, acc_y; double x_per_char, y_per_line; }`.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct pgtk_display_info_scroll
{
  pub acc_x: c_double,       // C: `double acc_x, acc_y;`
  pub acc_y: c_double,
  pub x_per_char: c_double,  // C: `double x_per_char, y_per_line;`
  pub y_per_line: c_double,
}

/// C 284: `extern struct pgtk_display_info *x_display_list;` - the *definition* is
/// `src/pgtkterm.rs`'s (C 81-82), so this mirror only declares it (PORTING.md 0.1-3).
#[cfg (feature = "have_pgtk")]
pub mod declarations
{
  use super::*;   // the TU's items: the module does not inherit the outer scope
  unsafe extern "C" {
    pub static mut x_display_list: *mut pgtk_display_info;
  }
}
#[cfg (feature = "have_pgtk")]
use declarations::*;

/* C 286-455: `struct pgtk_output` - the PGTK-side `struct x_output` (`src/gtkutil.rs`'s
   `xp_output`).  **UNVERIFIED**.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct pgtk_output
{
  pub foreground_color: c_ulong,        // C 288: `unsigned long foreground_color;`
  pub background_color: c_ulong,        // C 289: `unsigned long background_color;`
  pub toolbar: *mut c_void,             // C 290: `void *toolbar;`
  pub last_user_time: guint32,          // C 297: `guint32 last_user_time;`
  pub current_cursor: Emacs_Cursor,     // C 300-318: the sixteen `Emacs_Cursor` members
  pub text_cursor: Emacs_Cursor,
  pub nontext_cursor: Emacs_Cursor,
  pub modeline_cursor: Emacs_Cursor,
  pub hand_cursor: Emacs_Cursor,
  pub hourglass_cursor: Emacs_Cursor,
  pub horizontal_drag_cursor: Emacs_Cursor,
  pub vertical_drag_cursor: Emacs_Cursor,
  pub left_edge_cursor: Emacs_Cursor,
  pub top_left_corner_cursor: Emacs_Cursor,
  pub top_edge_cursor: Emacs_Cursor,
  pub top_right_corner_cursor: Emacs_Cursor,
  pub right_edge_cursor: Emacs_Cursor,
  pub bottom_right_corner_cursor: Emacs_Cursor,
  pub bottom_edge_cursor: Emacs_Cursor,
  pub bottom_left_corner_cursor: Emacs_Cursor,
  pub current_pointer: Emacs_Cursor,    // C 318: `Emacs_Cursor current_pointer;`
  pub border_pixel: c_ulong,            // C 321: `unsigned long border_pixel;`
  pub border_color_css_provider: *mut GtkCssProvider,        // C 322
  pub scrollbar_foreground_css_provider: *mut GtkCssProvider,  // C 325
  pub scrollbar_background_css_provider: *mut GtkCssProvider,  // C 326
  pub hourglass_widget: *mut GtkWidget,  // C 330: `GtkWidget *hourglass_widget;`
  pub cursor_xgcv: Emacs_GC,             // C 332: `Emacs_GC cursor_xgcv;`
  pub window_desc: Window,               // C 335: `Window window_desc, parent_desc;`
  pub parent_desc: Window,
  pub explicit_parent: c_char,           // C 336: `char explicit_parent;`
  pub icon_bitmap: ptrdiff_t,            // C 340: `ptrdiff_t icon_bitmap;`
  pub font: *mut font,                   // C 342: `struct font *font;`
  pub baseline_offset: c_int,            // C 343: `int baseline_offset;`
  pub fontset: c_int,                    // C 347: `int fontset;`
  pub mouse_color: c_ulong,              // C 349: `unsigned long mouse_color;`
  pub cursor_color: c_ulong,             // C 350: `unsigned long cursor_color;`
  pub cursor_foreground_color: c_ulong,  // C 351: `unsigned long cursor_foreground_color;`
  pub icon_top: c_int,                   // C 353: `int icon_top;`
  pub icon_left: c_int,                  // C 354: `int icon_left;`
  pub vertical_scroll_bar_extra: c_int,  // C 358
  pub titlebar_height: c_int,            // C 361: `int titlebar_height;`
  pub toolbar_height: c_int,             // C 364: `int toolbar_height;`
  pub display_info: *mut pgtk_display_info,  // C 367
  pub zooming: c_int,                    // C 370: `int zooming;`
  pub in_animation: c_int,               // C 373: `int in_animation;`
  pub size_hints: GdkGeometry,           // C 376: `GdkGeometry size_hints;`
  pub hint_flags: c_long,                // C 377: `long hint_flags;`
  pub preferred_width: c_int,            // C 378: `int preferred_width, preferred_height;`
  pub preferred_height: c_int,
  pub widget: *mut GtkWidget,            // C 381: `GtkWidget *widget;`
  pub edit_widget: *mut GtkWidget,       // C 384: `GtkWidget *edit_widget;`
  pub vbox_widget: *mut GtkWidget,       // C 386: `GtkWidget *vbox_widget;`
  pub hbox_widget: *mut GtkWidget,       // C 388: `GtkWidget *hbox_widget;`
  pub menubar_widget: *mut GtkWidget,    // C 390: `GtkWidget *menubar_widget;`
  pub toolbar_widget: *mut GtkWidget,    // C 392: `GtkWidget *toolbar_widget;`
  /// C 394/395: the record's two toolbar flags - carried in the P-80 ① storage unit below
  /// (2026-10-06: the members' first consumer arrived, so the unit is stored rather than
  /// recorded; `pgtk_output_bits!` reads and writes each by C's own member name).
  pub bits: c_uint,
  pub ttip_widget: *mut GtkTooltip,      // C 397: `GtkTooltip *ttip_widget;`
  pub ttip_lbl: *mut GtkWidget,          // C 398: `GtkWidget *ttip_lbl;`
  pub ttip_window: *mut GtkWindow,       // C 399: `GtkWindow *ttip_window;`
  pub menubar_height: c_int,             // C 403: `int menubar_height;`
  pub toolbar_top_height: c_int,         // C 408: `int toolbar_top_height, toolbar_bottom_height;`
  pub toolbar_bottom_height: c_int,
  pub toolbar_left_width: c_int,         // C 413: `int toolbar_left_width, toolbar_right_width;`
  pub toolbar_right_width: c_int,
  /* C 415-421: `#ifdef USE_CAIRO` - the four cairo contexts and the visible-bell surface.  Both
     arms are carried; the member names are C's.  */
  #[cfg (feature = "use_cairo")]
  pub cr_context: *mut cairo_t,          // C 417: `cairo_t *cr_context, *cr_active;`
  #[cfg (feature = "use_cairo")]
  pub cr_active: *mut cairo_t,
  #[cfg (feature = "use_cairo")]
  pub cr_surface_desired_width: c_int,   // C 418: `int cr_surface_desired_width, ...;`
  #[cfg (feature = "use_cairo")]
  pub cr_surface_desired_height: c_int,
  #[cfg (feature = "use_cairo")]
  pub cr_surface_visible_bell: *mut cairo_surface_t,  // C 420
  pub atimer_visible_bell: *mut c_void,  // C 422: `struct atimer *atimer_visible_bell;` (bare tag, 0.1-2)
  pub has_been_visible: c_int,           // C 424: `int has_been_visible;`
  pub black_relief: relief,              // C 427-432: `struct relief { Emacs_GC xgcv; unsigned
  pub white_relief: relief,              //  long pixel; } black_relief, white_relief;`
  pub relief_background: c_ulong,        // C 436: `unsigned long relief_background;`
  // C 440: `bool_bf relief_background_valid_p : 1;` - carried in the storage unit at C 394/395
  // above (P-80 ①: one unit per record's bit-field segment).
  pub focus_state: c_int,                // C 445: `int focus_state;`
  pub watched_scale_factor: c_double,    // C 451: `double watched_scale_factor;`
  pub scale_factor_atimer: *mut c_void,  // C 452: `struct atimer *scale_factor_atimer;` (bare tag, 0.1-2)
}
/// C 427-432: `struct relief { Emacs_GC xgcv; unsigned long pixel; };` (the record C declares
/// inside `pgtk_output`).  **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct relief
{
  pub xgcv: Emacs_GC,        // C: `Emacs_GC xgcv;`
  pub pixel: c_ulong,        // C: `unsigned long pixel;`
}

/// P-80 ①/②: `pgtk_output`'s three `bool_bf` members (C 394's `toolbar_in_hbox`, C 395's
/// `toolbar_is_packed`, C 440's `relief_background_valid_p`) read and written by C's own member
/// name through the record's storage unit - the `glyph_bits!`/`glyph_row_bits!` shape.  The unit
/// and the three masks are the port's storage assignment: no GTK toolchain here can measure this
/// record (P-88 ②, which is why the record as a whole is `UNVERIFIED`).
#[cfg (feature = "have_pgtk")]
pub const PGTK_OUTPUT_TOOLBAR_IN_HBOX: c_uint = 1 << 0;                  // C 394
#[cfg (feature = "have_pgtk")]
pub const PGTK_OUTPUT_TOOLBAR_IS_PACKED: c_uint = 1 << 1;                // C 395
#[cfg (feature = "have_pgtk")]
pub const PGTK_OUTPUT_RELIEF_BACKGROUND_VALID_P: c_uint = 1 << 2;        // C 440
#[cfg (feature = "have_pgtk")]
macro_rules! pgtk_output_bits {
  (toolbar_in_hbox, $o:expr) => { ((*$o).bits & PGTK_OUTPUT_TOOLBAR_IN_HBOX) != 0 };
  (toolbar_in_hbox, $o:expr, $v:expr) =>
    { (*$o).bits = if $v { (*$o).bits | PGTK_OUTPUT_TOOLBAR_IN_HBOX }
                   else { (*$o).bits & !PGTK_OUTPUT_TOOLBAR_IN_HBOX } };
  (toolbar_is_packed, $o:expr) => { ((*$o).bits & PGTK_OUTPUT_TOOLBAR_IS_PACKED) != 0 };
  (toolbar_is_packed, $o:expr, $v:expr) =>
    { (*$o).bits = if $v { (*$o).bits | PGTK_OUTPUT_TOOLBAR_IS_PACKED }
                   else { (*$o).bits & !PGTK_OUTPUT_TOOLBAR_IS_PACKED } };
  (relief_background_valid_p, $o:expr) =>
    { ((*$o).bits & PGTK_OUTPUT_RELIEF_BACKGROUND_VALID_P) != 0 };
  (relief_background_valid_p, $o:expr, $v:expr) =>
    { (*$o).bits = if $v { (*$o).bits | PGTK_OUTPUT_RELIEF_BACKGROUND_VALID_P }
                   else { (*$o).bits & !PGTK_OUTPUT_RELIEF_BACKGROUND_VALID_P } };
}

/* C 456-459: the PGTK arm's `struct x_output` (`/* Satisfy term.c. */`).  It is a *different*
   record from `xterm.h`'s, which `include/xterm.rs` 700-940 carries for the X arm; the two never
   meet, because `gtkutil.c` includes `xterm.h` under `#ifndef HAVE_PGTK` and this header under
   `#else`.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct x_output
{
  pub unused: c_int,         // C 458: `int unused;`
}

/* C 461-471: the anonymous enumeration whose members are `focus_state`'s values.  P-67/android's
   convention: the constants carry C's names, the enum type is unnamed in C, so the port declares
   the members as constants (the `include/androidterm.rs` 340-352 shape).  */
#[cfg (feature = "have_pgtk")]
pub const FOCUS_NONE: c_int = 0;       // C 468: `FOCUS_NONE = 0,`
#[cfg (feature = "have_pgtk")]
pub const FOCUS_IMPLICIT: c_int = 1;   // C 469: `FOCUS_IMPLICIT = 1,`
#[cfg (feature = "have_pgtk")]
pub const FOCUS_EXPLICIT: c_int = 2;   // C 470: `FOCUS_EXPLICIT = 2`

/* C 473-530: the accessor family.  Each keeps C's name; where C's macro is an l-value the port
   gives it the second (assignment) arm - the `src/pgtkterm.rs` 792-795 spelling of the same
   reference text, which is what those call sites need (`FRAME_GTK_OUTER_WIDGET (f) = wtop`).  */
/// C 474: `#define FRAME_X_OUTPUT(f)         ((f)->output_data.pgtk)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_X_OUTPUT {
  ($f:expr) => { ((*$f).output_data.pgtk as *mut pgtk_output) };
}
/// C 475: `#define FRAME_OUTPUT_DATA(f)      FRAME_X_OUTPUT (f)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_OUTPUT_DATA {
  ($f:expr) => { FRAME_X_OUTPUT! ($f) };
}
/// C 477: `#define FRAME_DISPLAY_INFO(f)     (FRAME_X_OUTPUT (f)->display_info)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_DISPLAY_INFO {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).display_info };
}
/// C 478/479/480: the three colour fields, which C writes (C 289-291 of `pgtkfns.c`).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_FOREGROUND_COLOR {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).foreground_color };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).foreground_color = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_BACKGROUND_COLOR {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).background_color };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).background_color = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CURSOR_COLOR {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).cursor_color };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).cursor_color = $v };
}
/// C 481: `#define FRAME_POINTER_TYPE(f)     (FRAME_X_OUTPUT (f)->current_pointer)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_POINTER_TYPE {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).current_pointer };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).current_pointer = $v };
}
/// C 482/514/516: `FRAME_FONT`, `FRAME_FONTSET` and `FRAME_BASELINE_OFFSET` (C writes all three).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_FONT {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).font };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).font = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_FONTSET {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).fontset };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).fontset = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_BASELINE_OFFSET {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).baseline_offset };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).baseline_offset = $v };
}
/// C 483/484: the two widget fields, which C writes (C 1669/1670 of `gtkutil.c`, C 1004-1011 of
/// `pgtkterm.c`).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_GTK_OUTER_WIDGET {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).widget };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).widget = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_GTK_WIDGET {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).edit_widget };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).edit_widget = $v };
}
/// C 485-487: `#define FRAME_WIDGET(f)           (FRAME_GTK_OUTER_WIDGET (f) \` / `? FRAME_GTK_OUTER_WIDGET (f) : FRAME_GTK_WIDGET (f))`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_WIDGET {
  ($f:expr) => {
    if ! FRAME_GTK_OUTER_WIDGET! ($f).is_null () { FRAME_GTK_OUTER_WIDGET! ($f) }
    else { FRAME_GTK_WIDGET! ($f) }
  };
}
/// C 489-495: `FRAME_PGTK_VIEW`, `FRAME_X_WINDOW`, `FRAME_NATIVE_WINDOW`, `FRAME_GDK_WINDOW` and
/// `FRAME_X_DISPLAY` (`->gdpy` is C's union member, which the record keeps as its field name).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_PGTK_VIEW {
  ($f:expr) => { FRAME_GTK_WIDGET! ($f) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_X_WINDOW {
  ($f:expr) => { FRAME_GTK_OUTER_WIDGET! ($f) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_NATIVE_WINDOW {
  ($f:expr) => { GTK_WINDOW! (FRAME_X_WINDOW! ($f)) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_GDK_WINDOW {
  ($f:expr) => { gtk_widget_get_window (GTK_WIDGET! (FRAME_X_WINDOW! ($f))) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_X_DISPLAY {
  ($f:expr) => { (*FRAME_DISPLAY_INFO! ($f)).gdpy };
}
/// C 497: `#define DEFAULT_GDK_DISPLAY() gdk_display_get_default ()`
#[cfg (feature = "have_pgtk")]
macro_rules! DEFAULT_GDK_DISPLAY {
  () => { gdk_display_get_default () };
}
/// C 500: `#define XSCROLL_BAR(vec) ((struct scroll_bar *) XVECTOR (vec))`
#[cfg (feature = "have_pgtk")]
macro_rules! XSCROLL_BAR {
  ($vec:expr) => { (XVECTOR ($vec) as *mut scroll_bar) };
}
/// C 502: `#define FRAME_DEFAULT_FACE(f) FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_DEFAULT_FACE {
  ($f:expr) => { FACE_FROM_ID_OR_NULL ($f, DEFAULT_FACE_ID) };
}
/// C 503/517/518: `FRAME_MENUBAR_HEIGHT`, `BLACK_PIX_DEFAULT` and `WHITE_PIX_DEFAULT`.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_MENUBAR_HEIGHT {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).menubar_height };
}
#[cfg (feature = "have_pgtk")]
macro_rules! BLACK_PIX_DEFAULT {
  ($f:expr) => { 0x000000 };
}
#[cfg (feature = "have_pgtk")]
macro_rules! WHITE_PIX_DEFAULT {
  ($f:expr) => { 0xFFFFFF };
}
/// C 504-513: the four toolbar measurements (C writes the two heights at C 330-331 of
/// `pgtkterm.c`).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TOOLBAR_TOP_HEIGHT {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).toolbar_top_height };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).toolbar_top_height = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TOOLBAR_BOTTOM_HEIGHT {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).toolbar_bottom_height };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).toolbar_bottom_height = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TOOLBAR_HEIGHT {
  ($f:expr) => {
    FRAME_TOOLBAR_TOP_HEIGHT! ($f) + FRAME_TOOLBAR_BOTTOM_HEIGHT! ($f) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TOOLBAR_LEFT_WIDTH {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).toolbar_left_width };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TOOLBAR_RIGHT_WIDTH {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).toolbar_right_width };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TOOLBAR_WIDTH {
  ($f:expr) => {
    FRAME_TOOLBAR_LEFT_WIDTH! ($f) + FRAME_TOOLBAR_RIGHT_WIDTH! ($f) };
}
/// C 522-524: `#define FIRST_CHAR_POSITION(f) (! (FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT (f)) ? 0 : FRAME_SCROLL_BAR_COLS (f))`
#[cfg (feature = "have_pgtk")]
macro_rules! FIRST_CHAR_POSITION {
  ($f:expr) => {
    if ! FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT ($f) { 0 } else { FRAME_SCROLL_BAR_COLS ($f) } };
}
/// C 526-529: the two cairo-surface desired sizes, which C writes (C 328-329 of `pgtkterm.c`).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CR_SURFACE_DESIRED_WIDTH {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).cr_surface_desired_width };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).cr_surface_desired_width = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CR_SURFACE_DESIRED_HEIGHT {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).cr_surface_desired_height };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).cr_surface_desired_height = $v };
}

/* C 539-547: `struct selection_input_event`.  Its first member is a bit-field
   (`ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH`), which the port carries as the storage unit
   `include/termhooks.rs`'s `input_event` uses for the same construct (P-80's member shape);
   **UNVERIFIED**.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct selection_input_event
{
  pub bits: c_uint,                 // C 541: `ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH;`
  pub dpyinfo: *mut pgtk_display_info,  // C 542: `struct pgtk_display_info *dpyinfo;`
  pub requestor: *mut GdkWindow,        // C 544: `GdkWindow *requestor;`
  pub selection: GdkAtom,               // C 545: `GdkAtom selection, target, property;`
  pub target: GdkAtom,
  pub property: GdkAtom,
  pub time: guint32,                    // C 546: `guint32 time;`
}

/// C 550-554: `INLINE GdkDisplay * SELECTION_EVENT_DISPLAY (struct selection_input_event *ev) {
/// return ev->dpyinfo->display; }` - C's own body, which is `INLINE` in the header, so the mirror
/// carries it as an `#[inline(always)]` function rather than a declaration (PORTING.md 0.1-6; the
/// `include/frame.rs` accessors' shape).  `->display` is the anonymous union's second member, the
/// field `pgtk_display_info` records as `gdpy`.
#[cfg (feature = "have_pgtk")]
#[inline (always)]
pub unsafe fn SELECTION_EVENT_DISPLAY (ev: *mut selection_input_event) -> *mut GdkDisplay
{
  return (*(*ev).dpyinfo).gdpy;
}
/// C 555-567: the six `SELECTION_EVENT_*` accessor macros, each keeping C's body.
#[cfg (feature = "have_pgtk")]
macro_rules! SELECTION_EVENT_DPYINFO {
  ($eventp:expr) => { (*$eventp).dpyinfo };
}
#[cfg (feature = "have_pgtk")]
macro_rules! SELECTION_EVENT_REQUESTOR {
  ($eventp:expr) => { (*$eventp).requestor };
}
#[cfg (feature = "have_pgtk")]
macro_rules! SELECTION_EVENT_SELECTION {
  ($eventp:expr) => { (*$eventp).selection };
}
#[cfg (feature = "have_pgtk")]
macro_rules! SELECTION_EVENT_TARGET {
  ($eventp:expr) => { (*$eventp).target };
}
#[cfg (feature = "have_pgtk")]
macro_rules! SELECTION_EVENT_PROPERTY {
  ($eventp:expr) => { (*$eventp).property };
}
#[cfg (feature = "have_pgtk")]
macro_rules! SELECTION_EVENT_TIME {
  ($eventp:expr) => { (*$eventp).time };
}

/* C 569-679: the prototype block, verbatim - each `extern` becomes a declaration in one
   `unsafe extern "C"` block (PORTING.md 0.1-2/0.1-3), with C's own line in a `/// C:` note.  */
#[cfg (feature = "have_pgtk")]
pub mod prototypes
{
  use super::*;   // the TU's items: the module does not inherit the outer scope
unsafe extern "C" {
  /// C 569: `extern void pgtk_handle_selection_event (struct selection_input_event *);`
  pub fn pgtk_handle_selection_event (event: *mut selection_input_event);
  /// C 570: `extern void pgtk_clear_frame_selections (struct frame *);`
  pub fn pgtk_clear_frame_selections (f: *mut frame);
  /// C 571: `extern void pgtk_handle_property_notify (GdkEventProperty *);`
  pub fn pgtk_handle_property_notify (event: *mut GdkEventProperty);
  /// C 572: `extern void pgtk_handle_selection_notify (GdkEventSelection *);`
  pub fn pgtk_handle_selection_notify (event: *mut GdkEventSelection);
  /// C 575: `extern struct pgtk_display_info *pgtk_term_init (Lisp_Object, char *);`
  pub fn pgtk_term_init (name: Lisp_Object, xrm: *mut c_char) -> *mut pgtk_display_info;
  /// C 576: `extern void pgtk_term_shutdown (int);`
  pub fn pgtk_term_shutdown (sig: c_int);
  /// C 579: `extern void pgtk_clear_frame (struct frame *);`
  pub fn pgtk_clear_frame (f: *mut frame);
  /// C 580: `extern char *pgtk_xlfd_to_fontname (const char *);`
  pub fn pgtk_xlfd_to_fontname (name: *const c_char) -> *mut c_char;
  /// C 583: `extern const char *pgtk_get_defaults_value (const char *);`
  pub fn pgtk_get_defaults_value (key: *const c_char) -> *const c_char;
  /// C 584: `extern const char *pgtk_get_string_resource (XrmDatabase, const char *, const char *);`
  pub fn pgtk_get_string_resource (rdb: XrmDatabase, name: *const c_char, class: *const c_char)
                                   -> *const c_char;
  /// C 585: `extern void pgtk_implicitly_set_name (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn pgtk_implicitly_set_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  /// C 588-590: `extern bool pgtk_defined_color (struct frame *, const char *, Emacs_Color *, int, bool);`
  pub fn pgtk_defined_color (f: *mut frame, color: *const c_char, color_def: *mut Emacs_Color,
                             alloc: c_int, makeindex: bool) -> bool;
  /// C 590: `extern void pgtk_query_color (struct frame *, Emacs_Color *);`
  pub fn pgtk_query_color (f: *mut frame, color: *mut Emacs_Color);
  /// C 591: `extern void pgtk_query_colors (struct frame *, Emacs_Color *, int);`
  pub fn pgtk_query_colors (f: *mut frame, colors: *mut Emacs_Color, n: c_int);
  /// C 592: `extern int pgtk_parse_color (struct frame *, const char *, Emacs_Color *);`
  pub fn pgtk_parse_color (f: *mut frame, color_name: *const c_char, color: *mut Emacs_Color)
                           -> c_int;
  /// C 595: `extern void pgtk_clear_area (struct frame *, int, int, int, int);`
  pub fn pgtk_clear_area (f: *mut frame, x: c_int, y: c_int, width: c_int, height: c_int);
  /// C 596: `extern int pgtk_gtk_to_emacs_modifiers (struct pgtk_display_info *, int);`
  pub fn pgtk_gtk_to_emacs_modifiers (dpyinfo: *mut pgtk_display_info, state: c_int) -> c_int;
  /// C 597: `extern void pgtk_clear_under_internal_border (struct frame *);`
  pub fn pgtk_clear_under_internal_border (f: *mut frame);
  /// C 598: `extern void pgtk_set_event_handler (struct frame *);`
  pub fn pgtk_set_event_handler (f: *mut frame);
  /// C 601: `extern int pgtk_display_pixel_height (struct pgtk_display_info *);`
  pub fn pgtk_display_pixel_height (dpyinfo: *mut pgtk_display_info) -> c_int;
  /// C 602: `extern int pgtk_display_pixel_width (struct pgtk_display_info *);`
  pub fn pgtk_display_pixel_width (dpyinfo: *mut pgtk_display_info) -> c_int;
  /// C 604: `extern void pgtk_destroy_window (struct frame *);`
  pub fn pgtk_destroy_window (f: *mut frame);
  /// C 605: `extern void pgtk_set_parent_frame (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn pgtk_set_parent_frame (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  /// C 606: `extern void pgtk_set_no_focus_on_map (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn pgtk_set_no_focus_on_map (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  /// C 607: `extern void pgtk_set_no_accept_focus (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn pgtk_set_no_accept_focus (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  /// C 608: `extern void pgtk_set_z_group (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn pgtk_set_z_group (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  /// C 611: `extern void pgtk_cr_update_surface_desired_size (struct frame *, int, int, bool);`
  pub fn pgtk_cr_update_surface_desired_size (f: *mut frame, width: c_int, height: c_int,
                                              force: bool);
  /// C 612: `extern cairo_t *pgtk_begin_cr_clip (struct frame *);`
  pub fn pgtk_begin_cr_clip (f: *mut frame) -> *mut cairo_t;
  /// C 613: `extern void pgtk_end_cr_clip (struct frame *);`
  pub fn pgtk_end_cr_clip (f: *mut frame);
  /// C 614: `extern void pgtk_set_cr_source_with_gc_foreground (struct frame *, Emacs_GC *, bool);`
  pub fn pgtk_set_cr_source_with_gc_foreground (f: *mut frame, gc: *mut Emacs_GC, respect_alpha: bool);
  /// C 615: `extern void pgtk_set_cr_source_with_gc_background (struct frame *, Emacs_GC *, bool);`
  pub fn pgtk_set_cr_source_with_gc_background (f: *mut frame, gc: *mut Emacs_GC, respect_alpha: bool);
  /// C 616: `extern void pgtk_set_cr_source_with_color (struct frame *, unsigned long, bool);`
  pub fn pgtk_set_cr_source_with_color (f: *mut frame, color: c_ulong, respect_alpha: bool);
  /// C 617: `extern void pgtk_cr_draw_frame (cairo_t *, struct frame *);`
  pub fn pgtk_cr_draw_frame (cr: *mut cairo_t, f: *mut frame);
  /// C 618: `extern void pgtk_cr_destroy_frame_context (struct frame *);`
  pub fn pgtk_cr_destroy_frame_context (f: *mut frame);
  /// C 619: `extern Lisp_Object pgtk_cr_export_frames (Lisp_Object , cairo_surface_type_t);`
  pub fn pgtk_cr_export_frames (frames: Lisp_Object, surface_type: cairo_surface_type_t)
                                -> Lisp_Object;
  /// C 622: `extern Lisp_Object pgtk_popup_dialog (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn pgtk_popup_dialog (f: *mut frame, header: Lisp_Object, contents: Lisp_Object) -> Lisp_Object;
  /// C 623-624: `extern Lisp_Object pgtk_dialog_show (struct frame *, Lisp_Object, Lisp_Object,
  /// Lisp_Object, Lisp_Object, Lisp_Object);`
  pub fn pgtk_dialog_show (f: *mut frame, title: Lisp_Object, header: Lisp_Object,
                           contents: Lisp_Object, button: Lisp_Object, text: Lisp_Object)
                           -> Lisp_Object;
  /// C 625: `extern void initialize_frame_menubar (struct frame *);`
  pub fn initialize_frame_menubar (f: *mut frame);
  /// C 629: `extern void syms_of_pgtkterm (void);`
  pub fn syms_of_pgtkterm ();
  /// C 630: `extern void syms_of_pgtkfns (void);`
  pub fn syms_of_pgtkfns ();
  /// C 631: `extern void syms_of_pgtkmenu (void);`
  pub fn syms_of_pgtkmenu ();
  /// C 632: `extern void syms_of_pgtkselect (void);`
  pub fn syms_of_pgtkselect ();
  /// C 633: `extern void syms_of_pgtkim (void);`
  pub fn syms_of_pgtkim ();
  /// C 636: `extern void mark_pgtkterm (void);`
  pub fn mark_pgtkterm ();
  /// C 637: `extern void pgtk_delete_terminal (struct terminal *);`
  pub fn pgtk_delete_terminal (terminal: *mut terminal);
  /// C 639: `extern void pgtk_make_frame_visible (struct frame *);`
  pub fn pgtk_make_frame_visible (f: *mut frame);
  /// C 640: `extern void pgtk_make_frame_invisible (struct frame *);`
  pub fn pgtk_make_frame_invisible (f: *mut frame);
  /// C 641: `extern void pgtk_free_frame_resources (struct frame *);`
  pub fn pgtk_free_frame_resources (f: *mut frame);
  /// C 642: `extern void pgtk_iconify_frame (struct frame *);`
  pub fn pgtk_iconify_frame (f: *mut frame);
  /// C 643: `extern void pgtk_focus_frame (struct frame *, bool);`
  pub fn pgtk_focus_frame (f: *mut frame, noactivate: bool);
  /// C 644: `extern void pgtk_set_scroll_bar_default_width (struct frame *);`
  pub fn pgtk_set_scroll_bar_default_width (f: *mut frame);
  /// C 645: `extern void pgtk_set_scroll_bar_default_height (struct frame *);`
  pub fn pgtk_set_scroll_bar_default_height (f: *mut frame);
  /// C 646: `extern Lisp_Object pgtk_get_focus_frame (struct frame *);`
  pub fn pgtk_get_focus_frame (f: *mut frame) -> Lisp_Object;
  /// C 648: `extern void pgtk_frame_rehighlight (struct pgtk_display_info *);`
  pub fn pgtk_frame_rehighlight (dpyinfo: *mut pgtk_display_info);
  /// C 650: `extern void pgtk_change_tab_bar_height (struct frame *, int);`
  pub fn pgtk_change_tab_bar_height (f: *mut frame, height: c_int);
  /// C 652: `extern struct pgtk_display_info *check_pgtk_display_info (Lisp_Object);`
  pub fn check_pgtk_display_info (object: Lisp_Object) -> *mut pgtk_display_info;
  /// C 654: `extern void pgtk_default_font_parameter (struct frame *, Lisp_Object);`
  pub fn pgtk_default_font_parameter (f: *mut frame, parm: Lisp_Object);
  /// C 656: `extern void pgtk_menu_set_in_use (bool);`
  pub fn pgtk_menu_set_in_use (in_use: bool);
  /// C 659: `extern void pgtk_update_drop_status (Lisp_Object, Lisp_Object);`
  pub fn pgtk_update_drop_status (action: Lisp_Object, result: Lisp_Object);
  /// C 660: `extern void pgtk_finish_drop (Lisp_Object, Lisp_Object, Lisp_Object);`
  pub fn pgtk_finish_drop (arg1: Lisp_Object, arg2: Lisp_Object, arg3: Lisp_Object);
  /// C 662: `extern void pgtk_enqueue_string (struct frame *, gchar *);`
  pub fn pgtk_enqueue_string (f: *mut frame, str: *mut gchar);
  /// C 663: `extern void pgtk_enqueue_preedit (struct frame *, Lisp_Object);`
  pub fn pgtk_enqueue_preedit (f: *mut frame, preedit: Lisp_Object);
  /// C 664: `extern void pgtk_im_focus_in (struct frame *);`
  pub fn pgtk_im_focus_in (f: *mut frame);
  /// C 665: `extern void pgtk_im_focus_out (struct frame *);`
  pub fn pgtk_im_focus_out (f: *mut frame);
  /// C 666: `extern bool pgtk_im_filter_keypress (struct frame *, GdkEventKey *);`
  pub fn pgtk_im_filter_keypress (f: *mut frame, event: *mut GdkEventKey) -> bool;
  /// C 667-668: `extern void pgtk_im_set_cursor_location (struct frame *, int, int, int, int);`
  /// - the record `src/pgtkim.c` 213-215 defines has C's five parameters, and this declaration
  /// used to carry `GdkRectangle` as its fourth (status 3.496 corrected it).
  pub fn pgtk_im_set_cursor_location (f: *mut frame, x: c_int, y: c_int,
                                      width: c_int, height: c_int);
  /// C 669: `extern void pgtk_im_init (struct pgtk_display_info *);`
  pub fn pgtk_im_init (dpyinfo: *mut pgtk_display_info);
  /// C 670: `extern void pgtk_im_finish (struct pgtk_display_info *);`
  pub fn pgtk_im_finish (dpyinfo: *mut pgtk_display_info);
  /// C 672: `extern bool xg_set_icon (struct frame *, Lisp_Object);`
  pub fn xg_set_icon (f: *mut frame, file: Lisp_Object) -> bool;
  /// C 673: `extern bool xg_set_icon_from_xpm_data (struct frame *, const char **);`
  pub fn xg_set_icon_from_xpm_data (f: *mut frame, data: *mut *const c_char) -> bool;
  /// C 675: `extern bool pgtk_text_icon (struct frame *, const char *);`
  pub fn pgtk_text_icon (f: *mut frame, icon_name: *const c_char) -> bool;
  /// C 677: `extern double pgtk_frame_scale_factor (struct frame *);`
  pub fn pgtk_frame_scale_factor (f: *mut frame) -> c_double;
  // C 678: `extern int pgtk_emacs_to_gtk_modifiers (struct pgtk_display_info *, int);` - **not
  // declared here yet**: `src/xwidget.rs` (a landed unit outside this round's write scope) still
  // carries its own copy of this prototype, and the two land in one TU through this mirror
  // (measured: `check-mirror-dupes.py` reads it as a duplicate).  The declaration lands with that
  // copy's P-88 clause-5 move - `## Owed` of `docs/ports/include-pgtkterm.md`.  Until then the
  // note is a plain comment: a `///` chain with no item after it is a syntax error, and it made
  // **every** `have_pgtk` TU unparsable (status 3.492/3.496).
}
}
#[cfg (feature = "have_pgtk")]
use prototypes::*;
