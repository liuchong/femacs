// femacs: port of src/xfns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **`src/xfns.c` is a P-86 unit and is ported in slices; this is the first: C 1-802.**  The frozen
// configuration compiles nothing from this file.  Its object belongs to `WINDOW_SYSTEM_OBJ`, which
// `configure.ac` 7521 fills only under `HAVE_WINDOW_SYSTEM`, `HAVE_WINDOW_SYSTEM` is `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv`, and `docs/baseline/frozen-build-sources.txt`
// lists no `xfns.o`.  P-86 ① therefore applies: this is *transcription*, both arms of every `#if`
// are carried, and the file gate `have_x_windows` goes on every item **and on the include graph**
// (P-86 ⑧ - an include outside the gate fails the frozen build at once).  The `extract-abi.py`
// checklist for this file is empty (P-86 ②), so P-86 ②'s substitute is in `docs/ports/xfns.md`:
// the C line-range table, the `## Evidence` readings and the per-`#if` `## Deviations`.
//
// **This slice ends at the `}` of `x_set_alpha_background` (C 802); the next live definition is
// `x_set_tool_bar_position` (C 805)** (C 804 is its `static void` line).  The nested gates are
// C's own macros lower-cased: `use_xcb`, `have_x_i18n`, `glyph_debug`, `use_motif`, `use_gtk`,
// `have_gtk3`, `have_xrender`, `use_x_toolkit`, `use_lucid`, `have_xaw3d`, `have_xdbe`,
// `have_xinput2`, `have_xrandr`, `have_xinerama`, `usg`.  Three of those names (`use_motif`,
// `have_xrandr`, `have_xinerama`) and one more this slice's heads name (`have_xaw3d`) are the
// C macros this file's `#if`s test but the tree's feature table does not register yet; they stay
// unregistered and are recorded in `docs/ports/xfns.md` ("Owed"), not added here.
//
// Names this slice calls that the mirrors it includes do not carry are declared at the use site
// with C's own line in a `/// C:` comment (PORTING.md P-55/P-72 rule 3, the `xterm.rs` /
// `androidmenu.rs` shape); the Xlib and XCB entry points keep the header and line of the same
// measurement base `include/xlib.rs` cites (libx11 1.8.13 and xorgproto 2025.1 under
// `/opt/homebrew/include`).  `docs/ports/xfns.md` names each as a mirror gap and names the file it
// belongs in.  Names reached only through a *mirror* macro body (`BlackPixel`/`WhitePixel`/
// `XScreenNumberOfScreen`, in `include/xterm.rs`'s `BLACK_PIX_DEFAULT`/`WHITE_PIX_DEFAULT` and
// `FRAME_X_SCREEN_NUMBER`) are recorded as that mirror's owed rows rather than redeclared here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 20: `#include <config.h>` (every unit includes it)

/* C 21-126: the file's own include list.  It sits outside every `#if` of the file body, so with
   the file gate on it (P-86 ⑧) the frozen configuration compiles not one line of this unit - the
   reading `docs/ports/xfns.md`'s Evidence ① records (a 336-byte empty object).  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/stdio.rs");           // C 21: `#include <stdio.h>`
#[cfg (feature = "have_x_windows")]
include!("../lib/include/stdlib.rs");          // C 22: `#include <stdlib.h>` (free)
#[cfg (feature = "have_x_windows")]
include!("../lib/include/math.rs");            // C 23: `#include <math.h>` (pow)
#[cfg (feature = "have_x_windows")]
include!("../lib/include/unistd.in.rs");       // C 24: `#include <unistd.h>`
#[cfg (feature = "have_x_windows")]
include!("include/lisp.rs");                   // C 26: `#include "lisp.h"`
include!("../lib/include/stdcountof.rs");   // lisp.h 28: `#include <stdcountof.h>` (countof)
#[cfg (feature = "have_x_windows")]
include!("../lib/include/limits.in.rs");      // lisp.h 33: `#include <limits.h>` (LONG_WIDTH)
#[cfg (feature = "have_x_windows")]
include!("include/character.rs");              // C 27: `#include "character.h"`
#[cfg (feature = "have_x_windows")]
include!("include/xterm.rs");                  // C 28: `#include "xterm.h"` (brings `xlib.rs`)
#[cfg (feature = "have_x_windows")]
include!("include/frame.rs");                  // C 29: `#include "frame.h"`
#[cfg (feature = "have_x_windows")]
include!("include/window.rs");                 // C 30: `#include "window.h"`
#[cfg (feature = "have_x_windows")]
include!("include/buffer.rs");                 // C 31: `#include "buffer.h"`
#[cfg (feature = "have_x_windows")]
include!("include/dispextern.rs");             // C 32: `#include "dispextern.h"` (Emacs_Color)
#[cfg (feature = "have_x_windows")]
include!("include/keyboard.rs");               // C 33: `#include "keyboard.h"`
#[cfg (feature = "have_x_windows")]
include!("include/blockinput.rs");             // C 34: `#include "blockinput.h"`
#[cfg (feature = "have_x_windows")]
include!("include/charset.rs");                // C 35: `#include "charset.h"`
#[cfg (feature = "have_x_windows")]
include!("include/coding.rs");                 // C 36: `#include "coding.h"`
#[cfg (feature = "have_x_windows")]
include!("include/termhooks.rs");              // C 37: `#include "termhooks.h"`
#[cfg (feature = "have_x_windows")]
include!("include/font.rs");                   // C 38: `#include "font.h"`
/* C 40-42: `#ifdef HAVE_X_I18N / #include "textconv.h" / #endif`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
include!("include/textconv.rs");               // C 41: `#include "textconv.h"`
#[cfg (feature = "have_x_windows")]
include!("../lib/include/sys_types.in.rs");    // C 44: `#include <sys/types.h>`
#[cfg (feature = "have_x_windows")]
include!("../lib/include/sys_stat.in.rs");     // C 45: `#include <sys/stat.h>`

/* C 47-50: `#ifdef USE_XCB / #include <xcb/xcb.h> / #include <xcb/xproto.h> / #endif`.  The two
   headers are the X11 toolchain's own (`include/xlib.rs` is this tree's declaration of the surface
   the gated arms name), so the C lines stand as this comment and the xcb names this slice uses -
   `xcb_query_tree` and its reply record, which the mirror does not carry yet - are declared at
   their use site below (P-88 ①).  */

/* C 52: `#include "bitmaps/gray.xbm"`.  An XBM image, not a header: C's `#include` is the
   preprocessor spelling of "read this data file's two constants" (`gray_width`, `gray_height`,
   `gray_bits`).  The reference tree file has no Rust counterpart; `docs/ports/xfns.md` records it
   and the names' only consumer is beyond this slice.  */
#[cfg (feature = "have_x_windows")]
include!("include/xsettings.rs");              // C 53: `#include "xsettings.h"`

/* C 55-57 and 58-60: `<X11/extensions/Xrandr.h>` (HAVE_XRANDR) and `<X11/extensions/Xinerama.h>`
   (HAVE_XINERAMA).  The X11 extensions' own headers; the mirror that would carry them does not
   exist in this tree, and this slice takes no name from either, so the C lines stand as this
   comment (P-88 ①, `docs/ports/xfns.md`'s Owed).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
include!("include/gtkutil.rs");                // C 63: `#include "gtkutil.h"`
/* C 63's chain: `"gtkutil.h"` 26's `<gtk/gtk.h>` - the SDK surface's one
   mirror (P-90 ②).  The in-unit segment that declared `GtkWidget`,
   `GtkWidgetClass` and `gtk_widget_set_app_paintable` is deleted in the same
   round (status 3.484).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
include!("include/gtk.rs");

/* C 66-72: `<X11/extensions/Xdbe.h>` (HAVE_XDBE, C 67) and `<X11/extensions/XInput2.h>`
   (HAVE_XINPUT2, C 71) - the X11 extensions' own headers again, with no mirror in this tree; the
   two lines stand as this comment (P-88 ①).  */

/* C 87-95: the `#ifdef USG` dance around `<X11/Xos.h>` - `#undef USG`, the include, `#define
   USG`, then an empty `#ifdef USG` block that exists only to pacify `gcc -Wunused-macros`.  Both
   arms include the same header, so the port has one include and the marker itself is C's empty
   definition (the `__MALLOC_HOOK_VOLATILE`/`GC_SETJMP_WORKS` shape in alloc.rs).  */
macro_rules! USG { () => { }; }

/* C 74-112: the `USE_X_TOOLKIT` block - `<X11/Shell.h>`, `<X11/Xaw3d/{Paned,Label}.h>` or
   `<X11/Xaw/{Paned,Label}.h>` (HAVE_XAW3D), `<X11/Xos.h>` (with the `USG` dance at C 87-95),
   `"widget.h"` (C 97), `"../lwlib/lwlib.h"` (C 99), the six `<Xm/…>` headers (`Xm/Xm.h`,
   `Xm/DialogS.h`, `Xm/FileSB.h`, `Xm/List.h`, `Xm/TextF.h`, `Xm/MwmUtil.h`; USE_MOTIF, C 102-107) and `"../lwlib/xlwmenu.h"` (USE_LUCID, C 111).  The two C-side headers have
   mirrors and are included; the X11/Xaw/Xm headers are the toolchain's own and have none, so the
   C lines stand as this comment (P-88 ①).  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
include!("include/widget.rs");                 // C 97: `#include "widget.h"`
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
include!("../lwlib/include/lwlib.rs");         // C 99: `#include "../lwlib/lwlib.h"`
#[cfg (all (feature = "have_x_windows", feature = "use_lucid"))]
include!("../lwlib/include/xlwmenu.rs");       // C 111: `#include "../lwlib/xlwmenu.h"`

/* C 114-116: "Unique id counter for widgets created by the Lucid Widget Library."  C 116 is a
   bare `extern` declaration of `lwlib.h` 89's object, which `lwlib.c` defines; no mirror carries
   it yet, so it is declared here (P-72 rule 3).  */
/// C: xfns.c 116: `extern LWLIB_ID widget_id_tick;`
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe extern "C" {
  pub static mut widget_id_tick: LWLIB_ID;
}

/* C 118-126: two empty `#ifdef` blocks (`USE_MOTIF`, then `USE_GTK`) holding nothing but their
   own comments - carried as this comment, with no Rust item (there is nothing to gate).  */

/* C 128: C's macro for Xlib's request-size query.  */
/// C: xfns.c 128: `#define MAXREQUEST(dpy) (XMaxRequestSize (dpy))`
#[cfg (feature = "have_x_windows")]
macro_rules! MAXREQUEST {
  ($dpy:expr) => { XMaxRequestSize ($dpy) };
}

/* C 130-132: the reference count a `GLYPH_DEBUG` build checks `struct x_display_info`'s own
   against (C 4808 is the assertion, beyond this slice).  */
#[cfg (all (feature = "have_x_windows", feature = "glyph_debug"))]
static mut dpyinfo_refcount: c_int = 0;   // C: `static int dpyinfo_refcount;`

/* C 134-161: the Motif window-manager hint constants, the record that carries them and its element
   count, all three under C's own `#ifndef USE_MOTIF` / `#ifndef USE_GTK` pair (C 134-135,
   160-161).  The two commented-out definitions C leaves in the block (C 136, 138-139, 142-147) are
   comments in the reference too and stay comments here.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
/// C: xfns.c 137: `#define MWM_HINTS_DECORATIONS   (1L << 1)`
pub const MWM_HINTS_DECORATIONS: c_long = 1 << 1;
#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
/// C: xfns.c 141: `#define MWM_DECOR_ALL           (1L << 0)`
pub const MWM_DECOR_ALL: c_long = 1 << 0;

/// C: xfns.c 149-157: `typedef struct { unsigned long flags; unsigned long functions; unsigned
/// long decorations; long input_mode; unsigned long status; } PropMotifWmHints;`
#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct PropMotifWmHints
{
  pub flags: c_ulong,          // C: unsigned long flags;
  pub functions: c_ulong,      // C: unsigned long functions;
  pub decorations: c_ulong,    // C: unsigned long decorations;
  pub input_mode: c_long,      // C: long input_mode;
  pub status: c_ulong,         // C: unsigned long status;
}
/* Measured on this machine with the record's own text (`clang -o mwm mwm.c && ./mwm`, 2026-10-05):
   `sizeof=40 align=8`, offsets `flags 0 functions 8 decorations 16 input_mode 24 status 32`.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
const _: () = core::assert! (core::mem::size_of::<PropMotifWmHints> () == 40);
#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
const _: () = core::assert! (core::mem::align_of::<PropMotifWmHints> () == 8);
#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
const _: () = core::assert! (core::mem::offset_of! (PropMotifWmHints, functions) == 8);
#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
const _: () = core::assert! (core::mem::offset_of! (PropMotifWmHints, decorations) == 16);
#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
const _: () = core::assert! (core::mem::offset_of! (PropMotifWmHints, input_mode) == 24);
#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
const _: () = core::assert! (core::mem::offset_of! (PropMotifWmHints, status) == 32);

#[cfg (all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk")))]
/// C: xfns.c 159: `#define PROP_MOTIF_WM_HINTS_ELEMENTS 5`
pub const PROP_MOTIF_WM_HINTS_ELEMENTS: c_int = 5;

/* C 101-108: the six `<Xm/…>` includes C takes under `USE_MOTIF` (`Xm/Xm.h`, `Xm/DialogS.h`,
   `Xm/FileSB.h`, `Xm/List.h`, `Xm/TextF.h`, `Xm/MwmUtil.h`).  None of them has a mirror in this
   tree, so the `USE_MOTIF` arm's names are the use-site declarations this file already carries
   for the other toolchain headers (P-88 ①).  C's block above is the file's *own* copy of the
   record `<Xm/MwmUtil.h>` defines; the block below is the header's, which is what a `USE_MOTIF`
   build sees instead, and it is what this file's `USE_MOTIF` arm reads them from.  */
/// C: Xm/MwmUtil.h: `typedef struct { unsigned long flags; unsigned long functions; unsigned
/// long decorations; long input_mode; unsigned long status; } PropMotifWmHints;` - the same
/// record as C 151-157's copy, with the layout that copy was measured at (`sizeof=40`, `flags`
/// 0, `functions` 8, `decorations` 16, `input_mode` 24, `status` 32).  **UNVERIFIED** against the
/// header itself (no Motif headers on this machine).
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct PropMotifWmHints
{
  pub flags: c_ulong,          // C: unsigned long flags;
  pub functions: c_ulong,      // C: unsigned long functions;
  pub decorations: c_ulong,    // C: unsigned long decorations;
  pub input_mode: c_long,      // C: long input_mode;
  pub status: c_ulong,         // C: unsigned long status;
}
/// C: Xm/MwmUtil.h: `#define MWM_HINTS_DECORATIONS (1L << 1)` - C 908's read.  **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const MWM_HINTS_DECORATIONS: c_long = 1 << 1;
/// C: Xm/MwmUtil.h: `#define MWM_DECOR_ALL (1L << 0)` - C 909's read.  **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const MWM_DECOR_ALL: c_long = 1 << 0;
/// C: Xm/MwmUtil.h: `#define PROP_MOTIF_WM_HINTS_ELEMENTS 5` - C 918's read.  **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const PROP_MOTIF_WM_HINTS_ELEMENTS: c_int = 5;

/* C 163-164 drop C's two forward declarations (PORTING.md 0.1-12: Rust has no forward
   declarations, and both functions are defined later in this same unit - `set_up_x_back_buffer`
   at C 3972 and `x_display_info_for_name` at C 7613, both in the remainder):

     static struct x_display_info *x_display_info_for_name (Lisp_Object);
     static void set_up_x_back_buffer (struct frame *f);

   `x_display_info_for_name` is *called* by this slice (C 198), so the call is kept and the
   definition is owed to the remainder slice; `docs/ports/xfns.md` records it.  */

/* Xlib entry points this slice calls that `include/xlib.rs` does not carry yet (it declares the
   ten its own arms needed).  Each keeps C's prototype and the header line, from the same
   measurement base the mirror cites.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: Xlib.h 2526-2528: `extern int XFree (void *);`
  pub fn XFree (data: *mut c_void) -> c_int;
  /// C: Xlib.h 2630-2640: `extern Status XGetGeometry (Display *, Drawable, Window *, int *,
  /// int *, unsigned int *, unsigned int *, unsigned int *, unsigned int *);`
  pub fn XGetGeometry (display: *mut Display, d: Drawable, root_return: *mut Window,
                       x_return: *mut c_int, y_return: *mut c_int,
                       width_return: *mut c_uint, height_return: *mut c_uint,
                       border_width_return: *mut c_uint, depth_return: *mut c_uint) -> c_int;
  /// C: Xlib.h 2686-2698: `extern int XGetWindowProperty (Display *, Window, Atom, long, long,
  /// Bool, Atom, Atom *, int *, unsigned long *, unsigned long *, unsigned char **);`
  pub fn XGetWindowProperty (display: *mut Display, w: Window, property: Atom,
                             long_offset: c_long, long_length: c_long, delete: Bool,
                             req_type: Atom, actual_type_return: *mut Atom,
                             actual_format_return: *mut c_int, nitems_return: *mut c_ulong,
                             bytes_after_return: *mut c_ulong,
                             prop_return: *mut *mut c_uchar) -> c_int;
  /// C: Xlib.h 2078-2087: `extern int XChangeProperty (Display *, Window, Atom, Atom, int, int,
  /// _Xconst unsigned char *, int);`
  pub fn XChangeProperty (display: *mut Display, w: Window, property: Atom, r#type: Atom,
                          format: c_int, mode: c_int, data: *const c_uchar,
                          nelements: c_int) -> c_int;
  /// C: Xlib.h 1709-1711: `extern long XMaxRequestSize (Display *);`
  pub fn XMaxRequestSize (display: *mut Display) -> c_long;
  /// C: Xlib.h 3027-3034: `extern Status XQueryTree (Display *, Window, Window *, Window *,
  /// Window **, unsigned int *);`
  pub fn XQueryTree (display: *mut Display, w: Window, root_return: *mut Window,
                     parent_return: *mut Window, children_return: *mut *mut Window,
                     nchildren_return: *mut c_uint) -> c_int;
  /// C: Xlib.h 3456-3465: `extern Bool XTranslateCoordinates (Display *, Window, Window, int,
  /// int, int *, int *, Window *);`
  pub fn XTranslateCoordinates (display: *mut Display, src_w: Window, dest_w: Window,
                                src_x: c_int, src_y: c_int, dest_x_return: *mut c_int,
                                dest_y_return: *mut c_int,
                                child_return: *mut Window) -> Bool;
}

/* The XCB entry points of C 47-50's arm that `include/xlib.rs`'s xcb section does not carry yet
   (it carries the property, window-attribute, translate-coordinates and geometry families; the
   query-tree pair this slice's loop needs is owed).  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
unsafe extern "C" {
  /// C: xcb/xproto.h 6605-6607: `xcb_query_tree_cookie_t xcb_query_tree (xcb_connection_t *,
  /// xcb_window_t);`
  /// C: xcb/xproto.h 6650-6653: `xcb_query_tree_reply_t *xcb_query_tree_reply
  /// (xcb_connection_t *, xcb_query_tree_cookie_t, xcb_generic_error_t **);`
  pub fn xcb_query_tree (c: *mut xcb_connection_t, window: xcb_window_t)
                         -> xcb_query_tree_cookie_t;
  pub fn xcb_query_tree_reply (c: *mut xcb_connection_t, cookie: xcb_query_tree_cookie_t,
                               e: *mut xcb_generic_error_t) -> *mut xcb_query_tree_reply_t;
}

/// C: xcb/xproto.h 1820-1822: `typedef struct xcb_query_tree_cookie_t { unsigned int
/// sequence; } xcb_query_tree_cookie_t;`
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct xcb_query_tree_cookie_t
{
  pub sequence: c_uint,
}
/// C: xcb/xproto.h 1840-1849 - the record's eight members, of which this slice reads exactly
/// C's `root` and `parent`:
/// ```
/// typedef struct {
///     uint8_t      response_type;   uint8_t      pad0;        uint16_t    sequence;
///     uint32_t     length;          xcb_window_t root;        xcb_window_t parent;
///     uint16_t     children_len;    uint8_t      pad1[14];
/// } xcb_query_tree_reply_t;
/// ```
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct xcb_query_tree_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub root: xcb_window_t,
  pub parent: xcb_window_t,
  pub children_len: u16,
  pub pad1: [u8; 14],
}

/* C's own entry points this slice calls, from headers whose mirrors do not carry them.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: frame.h 1474-1475: `extern WINDOW_SYSTEM_RETURN struct frame *decode_window_system_frame (Lisp_Object);`
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
  /// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);`
  pub fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal;
  /// C: xterm.h 1752: `extern void x_catch_errors (Display *);`
  pub fn x_catch_errors (dpy: *mut Display);
  /// C: xterm.h 1760: `extern void x_check_errors (Display *, const char *);` - C 7435's call
  /// (`src/xselect.rs` 495-496 declares the same row).
  pub fn x_check_errors (dpy: *mut Display, format: *const c_char);
  /// C: xterm.h 1762: `extern bool x_had_errors_p (Display *);`
  pub fn x_had_errors_p (dpy: *mut Display) -> bool;
  /// C: xterm.h 1764: `extern void x_uncatch_errors (void);`
  pub fn x_uncatch_errors ();
  /// C: xterm.h 1839-1840: `extern Bool x_query_pointer (Display *, Window, Window *, Window *,
  /// int *, int *, int *, int *, unsigned int *);`
  pub fn x_query_pointer (dpy: *mut Display, w: Window, root_return: *mut Window,
                          child_return: *mut Window, root_x_return: *mut c_int,
                          root_y_return: *mut c_int, win_x_return: *mut c_int,
                          win_y_return: *mut c_int,
                          mask_return: *mut c_uint) -> Bool;
  /// C: xterm.h 391: `extern Status x_parse_color (struct frame *, const char *, XColor *);`
  pub fn x_parse_color (f: *mut frame, color_name: *const c_char,
                        color: *mut XColor) -> c_int;
  /// C: xterm.h 1796: `extern bool x_alloc_nearest_color (struct frame *, Colormap, XColor *);`
  pub fn x_alloc_nearest_color (f: *mut frame, cmap: Colormap, color: *mut XColor) -> bool;

}

/* The two GTK-side names this slice's `x_defined_color` and `x_set_alpha_background` call.
   `include/gtkutil.rs` is the dead-by-construction mirror (`USE_GTK` is undef in the frozen
   configuration, so C 24-244 of the header is not compiled), so the names are declared here under
   C's own gate (P-88 ①).  The `<gtk/gtk.h>` types themselves are the SDK's and are named in
   `docs/ports/xfns.md`'s Owed.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  /// C: gtkutil.h 184-186: `extern bool xg_check_special_colors (struct frame *f,
  /// const char *color_name, Emacs_Color *color);`
  pub fn xg_check_special_colors (f: *mut frame, color_name: *const c_char,
                                  color: *mut Emacs_Color) -> bool;
}

/* Let the user specify an X display with a Lisp object.
   OBJECT may be nil, a frame or a terminal object.
   nil stands for the selected frame--or, if that is not an X frame,
   the first X display on the list.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_x_display_info (object: Lisp_Object) -> *mut x_display_info
{
  let mut dpyinfo: *mut x_display_info = std::ptr::null_mut ();

  if NILP (object)
    {
      let sf: *mut frame = XFRAME (unsafe { selected_frame });

      if FRAME_X_P (sf) && FRAME_LIVE_P (sf)
	{ dpyinfo = FRAME_DISPLAY_INFO! (sf); }
      else if ! unsafe { x_display_list }.is_null ()
	{ dpyinfo = unsafe { x_display_list }; }
      else
	{ error (c"X windows are not in use or not initialized".as_ptr ()); }
    }
  else if TERMINALP (object)
    {
      let t: *mut terminal = decode_live_terminal (object);

      if (*t).r#type != output_x_window
        { error (c"Terminal %d is not an X display".as_ptr (), (*t).id); }

      // C: `dpyinfo = t->display_info.x;` - the union member keeps P-72 rule 3's untyped
      // stand-in (`include/termhooks.rs` 165), so the consumer converts at the call site.
      dpyinfo = (*t).display_info.x as *mut x_display_info;
    }
  else if STRINGP (object)
    { dpyinfo = x_display_info_for_name (object); }
  else
    {
      let f: *mut frame = decode_window_system_frame (object);
      dpyinfo = FRAME_DISPLAY_INFO! (f);
    }

  dpyinfo
}

/* Return the screen positions and offsets of frame F.
   Store the offsets between FRAME_OUTER_WINDOW and the containing
   window manager window into LEFT_OFFSET_X, RIGHT_OFFSET_X,
   TOP_OFFSET_Y and BOTTOM_OFFSET_Y.
   Store the offsets between FRAME_X_WINDOW and the containing
   window manager window into X_PIXELS_DIFF and Y_PIXELS_DIFF.
   Store the screen positions of frame F into XPTR and YPTR.
   These are the positions of the containing window manager window,
   not Emacs's own window.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_real_pos_and_offsets (f: *mut frame,
                                  left_offset_x: *mut c_int,
                                  right_offset_x: *mut c_int,
                                  top_offset_y: *mut c_int,
                                  bottom_offset_y: *mut c_int,
                                  x_pixels_diff: *mut c_int,
                                  y_pixels_diff: *mut c_int,
                                  xptr: *mut c_int,
                                  yptr: *mut c_int,
                                  outer_border: *mut c_int)
{
  use crate::x11_constants::{PropModeReplace, Success, XA_CARDINAL};

  let mut win_x: c_int = 0;
  let mut win_y: c_int = 0;
  let mut outer_x: c_int = 0;
  let mut outer_y: c_int = 0;
  let mut real_x: c_int = 0;
  let mut real_y: c_int = 0;
  let mut had_errors: bool = false;
  let parent_frame: *mut frame = FRAME_PARENT_FRAME (f);
  let mut win: Window = if ! parent_frame.is_null ()
    { FRAME_X_WINDOW! (parent_frame) }
    else { (*((*f).output_data.x as *mut x_output)).parent_desc };
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let max_len: c_long = 400;
  let target_type: Atom = XA_CARDINAL;
  let mut ow: c_uint = 0;
  let mut oh: c_uint = 0;
  let mut fw: c_uint = 0;
  let mut fh: c_uint = 0;
  let mut bw: c_uint = 0;

  /* We resort to XCB if possible because there are several X calls
     here which require responses from the server but do not have data
     dependencies between them.  Using XCB lets us pipeline requests,
     whereas with Xlib we must wait for each answer before sending the
     next request.

     For a non-local display, the round-trip time could be a few tens
     of milliseconds, depending on the network distance.  It doesn't
     take a lot of those to add up to a noticeable hesitation in
     responding to user actions.  */
  /* C 251-262: the two arms' declarations.  C leaves `prop_cookie`,
     `outer_geom_cookie`, `sent_requests`, `actual_type` and `tmp_data` unassigned here and writes
     them before their guarded reads; the port zero-initialises each (PORTING.md 1.5).  */
  #[cfg (feature = "use_xcb")]
  let xcb_conn: *mut xcb_connection_t = (*dpyinfo).xcb_connection;
  #[cfg (feature = "use_xcb")]
  let mut prop_cookie: xcb_get_property_cookie_t = std::mem::zeroed ();
  #[cfg (feature = "use_xcb")]
  let mut outer_geom_cookie: xcb_get_geometry_cookie_t = std::mem::zeroed ();
  #[cfg (feature = "use_xcb")]
  let mut sent_requests: bool = false;
  #[cfg (not (feature = "use_xcb"))]
  let mut actual_type: Atom = 0;
  #[cfg (not (feature = "use_xcb"))]
  let mut actual_size: c_ulong = 0;
  #[cfg (not (feature = "use_xcb"))]
  let mut bytes_remaining: c_ulong = 0;
  #[cfg (not (feature = "use_xcb"))]
  let mut rc: c_int = 0;
  #[cfg (not (feature = "use_xcb"))]
  let mut actual_format: c_int = 0;
  #[cfg (not (feature = "use_xcb"))]
  let dpy: *mut Display = FRAME_X_DISPLAY! (f);
  #[cfg (not (feature = "use_xcb"))]
  let mut tmp_data: *mut c_uchar = std::ptr::null_mut ();

  if ! x_pixels_diff.is_null () { *x_pixels_diff = 0; }
  if ! y_pixels_diff.is_null () { *y_pixels_diff = 0; }
  if ! left_offset_x.is_null () { *left_offset_x = 0; }
  if ! top_offset_y.is_null () { *top_offset_y = 0; }
  if ! right_offset_x.is_null () { *right_offset_x = 0; }
  if ! bottom_offset_y.is_null () { *bottom_offset_y = 0; }
  if ! xptr.is_null () { *xptr = 0; }
  if ! yptr.is_null () { *yptr = 0; }
  if ! outer_border.is_null () { *outer_border = 0; }

  if win == (*dpyinfo).root_window
    { win = FRAME_OUTER_WINDOW! (f); }

  block_input ();

  #[cfg (not (feature = "use_xcb"))]
  {
    /* If we're using XCB, all errors are checked for on each call.  */
    x_catch_errors (dpy);
  }

  /* This loop traverses up the containment tree until we hit the root
     window.  Window managers may intersect many windows between our window
     and the root window.  The window we find just before the root window
     should be the outer WM window. */
  loop
    {
      // C: `Window wm_window UNINIT, rootw UNINIT;` (PORTING.md 1.5: zero-initialised).
      let mut wm_window: Window = 0;
      let mut rootw: Window = 0;

      #[cfg (feature = "use_xcb")]
      {
        let query_tree_cookie: xcb_query_tree_cookie_t;
        let query_tree: *mut xcb_query_tree_reply_t;

        query_tree_cookie = xcb_query_tree (xcb_conn, win as xcb_window_t);
        query_tree = xcb_query_tree_reply (xcb_conn, query_tree_cookie, std::ptr::null_mut ());
        if query_tree.is_null ()
          { had_errors = true; }
        else
          {
            wm_window = (*query_tree).parent as Window;
            rootw = (*query_tree).root as Window;
            free (query_tree as *mut c_void);
          }
      }
      #[cfg (not (feature = "use_xcb"))]
      {
        let mut tmp_children: *mut Window = std::ptr::null_mut ();
        let mut tmp_nchildren: c_uint = 0;
        let success: c_int;

        success = XQueryTree (dpy, win, &raw mut rootw,
                              &raw mut wm_window, &raw mut tmp_children, &raw mut tmp_nchildren);

        had_errors = x_had_errors_p (dpy);

        /* Don't free tmp_children if XQueryTree failed.  */
        if success == 0
          { break; }

        XFree (tmp_children as *mut c_void);
      }

      if had_errors || wm_window == rootw
        { break; }

      win = wm_window;
    }

  if ! had_errors
    {
      #[cfg (feature = "use_xcb")]
      let geom_cookie: xcb_get_geometry_cookie_t;
      #[cfg (feature = "use_xcb")]
      let trans_cookie: xcb_translate_coordinates_cookie_t;
      /* C: `xcb_translate_coordinates_cookie_t outer_trans_cookie;` - written only when
         `FRAME_X_WINDOW (f) != FRAME_OUTER_WINDOW (f)` and read only in the arm that tests so;
         the port zero-initialises it (PORTING.md 1.5), the same treatment `prop_cookie` and
         `outer_geom_cookie` get above.  */
      #[cfg (feature = "use_xcb")]
      let mut outer_trans_cookie: xcb_translate_coordinates_cookie_t = std::mem::zeroed ();
      #[cfg (feature = "use_xcb")]
      let trans: *mut xcb_translate_coordinates_reply_t;
      #[cfg (feature = "use_xcb")]
      let geom: *mut xcb_get_geometry_reply_t;
      #[cfg (not (feature = "use_xcb"))]
      let mut child: Window = 0;
      #[cfg (not (feature = "use_xcb"))]
      let mut rootw: Window = 0;
      #[cfg (not (feature = "use_xcb"))]
      let mut ign: c_uint = 0;

      #[cfg (feature = "use_xcb")]
      {
        /* Fire off the requests that don't have data dependencies.

           Once we've done this, we must collect the results for each
           one before returning, even if other errors are detected,
           making the other responses moot.  */
        geom_cookie = xcb_get_geometry (xcb_conn, win as xcb_window_t);

        trans_cookie =
          xcb_translate_coordinates (xcb_conn,
                                     /* From-window, to-window.  */
                                     (*FRAME_DISPLAY_INFO! (f)).root_window as xcb_window_t,
                                     FRAME_X_WINDOW! (f) as xcb_window_t,

                                     /* From-position.  */
                                     0, 0);
        if FRAME_X_WINDOW! (f) != FRAME_OUTER_WINDOW! (f)
          { outer_trans_cookie =
              xcb_translate_coordinates (xcb_conn,
                                         /* From-window, to-window.  */
                                         (*FRAME_DISPLAY_INFO! (f)).root_window as xcb_window_t,
                                         FRAME_OUTER_WINDOW! (f) as xcb_window_t,

                                         /* From-position.  */
                                         0, 0); }
        if ! right_offset_x.is_null () || ! bottom_offset_y.is_null ()
          { outer_geom_cookie = xcb_get_geometry (xcb_conn,
                                                  FRAME_OUTER_WINDOW! (f) as xcb_window_t); }

        if parent_frame.is_null ()
            && (*dpyinfo).root_window
               == (*((*f).output_data.x as *mut x_output)).parent_desc
          /* Try _NET_FRAME_EXTENTS if our parent is the root window.  */
          { prop_cookie = xcb_get_property (xcb_conn, 0, win as xcb_window_t,
                                            (*dpyinfo).Xatom_net_frame_extents as xcb_atom_t,
                                            target_type as xcb_atom_t, 0, max_len as u32); }

        sent_requests = true;
      }

      /* Get the real coordinates for the WM window upper left corner */
      #[cfg (feature = "use_xcb")]
      {
        geom = xcb_get_geometry_reply (xcb_conn, geom_cookie, std::ptr::null_mut ());
        if ! geom.is_null ()
          {
            real_x = (*geom).x as c_int;
            real_y = (*geom).y as c_int;
            ow = (*geom).width as c_uint;
            oh = (*geom).height as c_uint;
            bw = (*geom).border_width as c_uint;
            free (geom as *mut c_void);
          }
        else
          { had_errors = true; }
      }
      #[cfg (not (feature = "use_xcb"))]
      {
        XGetGeometry (dpy, win,
                      &raw mut rootw, &raw mut real_x, &raw mut real_y,
                      &raw mut ow, &raw mut oh, &raw mut bw, &raw mut ign);
      }

      /* Translate real coordinates to coordinates relative to our
         window.  For our window, the upper left corner is 0, 0.
         Since the upper left corner of the WM window is outside
         our window, win_x and win_y will be negative:

         ------------------          ---> x
         |      title                |
         | -----------------         v y
         | |  our window

         Since we don't care about the child window corresponding to
         the actual coordinates, we can send zero to get the offsets
         and compute the resulting coordinates below.  This reduces
         the data dependencies between calls and lets us pipeline the
         requests better in the XCB case.  */
      #[cfg (feature = "use_xcb")]
      {
        trans = xcb_translate_coordinates_reply (xcb_conn, trans_cookie, std::ptr::null_mut ());
        if ! trans.is_null ()
          {
            win_x = (*trans).dst_x as c_int;
            win_y = (*trans).dst_y as c_int;
            free (trans as *mut c_void);
          }
        else
          { had_errors = true; }
      }
      #[cfg (not (feature = "use_xcb"))]
      {
        XTranslateCoordinates (dpy,

                               /* From-window, to-window.  */
                               (*FRAME_DISPLAY_INFO! (f)).root_window,
                               FRAME_X_WINDOW! (f),

                               /* From-position, to-position.  */
                               0, 0, &raw mut win_x, &raw mut win_y,

                               /* Child of win.  */
                               &raw mut child);
      }

      win_x += real_x;
      win_y += real_y;

      if FRAME_X_WINDOW! (f) == FRAME_OUTER_WINDOW! (f)
	{
          outer_x = win_x;
          outer_y = win_y;
	}
      else
        {
          #[cfg (feature = "use_xcb")]
          let outer_trans: *mut xcb_translate_coordinates_reply_t;

          #[cfg (feature = "use_xcb")]
          {
            outer_trans = xcb_translate_coordinates_reply (xcb_conn,
                                                           outer_trans_cookie,
                                                           std::ptr::null_mut ());
            if ! outer_trans.is_null ()
              {
                outer_x = (*outer_trans).dst_x as c_int;
                outer_y = (*outer_trans).dst_y as c_int;
                free (outer_trans as *mut c_void);
              }
            else
              { had_errors = true; }
          }
          #[cfg (not (feature = "use_xcb"))]
          {
            XTranslateCoordinates (dpy,

                                   /* From-window, to-window.  */
                                   (*FRAME_DISPLAY_INFO! (f)).root_window,
                                   FRAME_OUTER_WINDOW! (f),

                                   /* From-position, to-position.  */
                                   0, 0, &raw mut outer_x, &raw mut outer_y,

                                   /* Child of win.  */
                                   &raw mut child);
          }

	  outer_x += real_x;
	  outer_y += real_y;
	}

      #[cfg (not (feature = "use_xcb"))]
      { had_errors = x_had_errors_p (dpy); }
    }

  if parent_frame.is_null ()
      && (*dpyinfo).root_window == (*((*f).output_data.x as *mut x_output)).parent_desc
    {
      /* Try _NET_FRAME_EXTENTS if our parent is the root window.  */
      #[cfg (feature = "use_xcb")]
      {
        /* Make sure we didn't get an X error early and skip sending the
           request.  */
        if sent_requests
          {
            let prop: *mut xcb_get_property_reply_t;

            prop = xcb_get_property_reply (xcb_conn, prop_cookie, std::ptr::null_mut ());
            if ! prop.is_null ()
              {
                if (*prop).r#type == target_type as xcb_atom_t
                    && (*prop).format == 32
                    && (xcb_get_property_value_length (prop)
                        == 4 * core::mem::size_of::<i32> () as c_int)
                  {
                    // C: `int32_t *fe = xcb_get_property_value (prop);` - P-72 rule 3's
                    // untyped return (`include/xlib.rs` 736), converted at the call site.
                    let fe: *mut i32 = xcb_get_property_value (prop) as *mut i32;

                    outer_x = -(*fe.offset (0));
                    outer_y = -(*fe.offset (2));
                    real_x -= *fe.offset (0);
                    real_y -= *fe.offset (2);
                  }
                free (prop as *mut c_void);
              }
            /* Xlib version doesn't set had_errors here.  Intentional or bug?  */
          }
      }
      #[cfg (not (feature = "use_xcb"))]
      {
        rc = XGetWindowProperty (dpy, win, (*dpyinfo).Xatom_net_frame_extents,
                                 0, max_len, 0, target_type,
                                 &raw mut actual_type, &raw mut actual_format, &raw mut actual_size,
                                 &raw mut bytes_remaining, &raw mut tmp_data);

        if rc == Success && actual_type == target_type && ! x_had_errors_p (dpy)
            && actual_size == 4 && actual_format == 32
          {
            let fe: *mut c_long = tmp_data as *mut c_long;

            /* C: `long *fe = (long *) tmp_data;` then `outer_x = -fe[0]; ... real_x -= fe[0];` -
               C's implicit `long` -> `int` conversions, spelled out here.  */
            outer_x = (-(*fe.offset (0))) as c_int;
            outer_y = (-(*fe.offset (2))) as c_int;
            real_x -= *fe.offset (0) as c_int;
            real_y -= *fe.offset (2) as c_int;
          }

        if ! tmp_data.is_null () { XFree (tmp_data as *mut c_void); }
      }
    }

  if ! right_offset_x.is_null () || ! bottom_offset_y.is_null ()
    {
      #[cfg (feature = "use_xcb")]
      {
        /* Make sure we didn't get an X error early and skip sending the
           request.  */
        if sent_requests
          {
            let outer_geom: *mut xcb_get_geometry_reply_t;

            outer_geom = xcb_get_geometry_reply (xcb_conn, outer_geom_cookie,
                                                 std::ptr::null_mut ());
            if ! outer_geom.is_null ()
              {
                fw = (*outer_geom).width as c_uint;
                fh = (*outer_geom).height as c_uint;
                free (outer_geom as *mut c_void);
              }
            else
              { had_errors = true; }
          }
      }
      #[cfg (not (feature = "use_xcb"))]
      {
        let mut xy_ign: c_int = 0;
        let mut ign: c_uint = 0;
        let mut rootw: Window = 0;

        XGetGeometry (dpy, FRAME_OUTER_WINDOW! (f),
                      &raw mut rootw, &raw mut xy_ign, &raw mut xy_ign,
                      &raw mut fw, &raw mut fh, &raw mut ign, &raw mut ign);
      }
    }

  #[cfg (not (feature = "use_xcb"))]
  { x_uncatch_errors (); }

  unblock_input ();

  if had_errors { return; }

  if ! x_pixels_diff.is_null () { *x_pixels_diff = -win_x; }
  if ! y_pixels_diff.is_null () { *y_pixels_diff = -win_y; }

  if ! left_offset_x.is_null () { *left_offset_x = -outer_x; }
  if ! top_offset_y.is_null () { *top_offset_y = -outer_y; }

  if ! xptr.is_null () { *xptr = real_x; }
  if ! yptr.is_null () { *yptr = real_y; }

  if ! outer_border.is_null () { *outer_border = bw as c_int; }

  if ! right_offset_x.is_null () { *right_offset_x = ow.wrapping_sub (fw).wrapping_add (outer_x as c_uint) as c_int; }
  if ! bottom_offset_y.is_null () { *bottom_offset_y = oh.wrapping_sub (fh).wrapping_add (outer_y as c_uint) as c_int; }
}

/* Store the screen positions of frame F into XPTR and YPTR.
   These are the positions of the containing window manager window,
   not Emacs's own window.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_real_positions (f: *mut frame, xptr: *mut c_int, yptr: *mut c_int)
{
  x_real_pos_and_offsets (f, std::ptr::null_mut (), std::ptr::null_mut (),
                          std::ptr::null_mut (), std::ptr::null_mut (),
                          std::ptr::null_mut (), std::ptr::null_mut (), xptr, yptr,
                          std::ptr::null_mut ());
}


/* Get the mouse position in frame relative coordinates.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_relative_mouse_position (f: *mut frame, x: *mut c_int, y: *mut c_int)
{
  let mut root: Window = 0;
  let mut dummy_window: Window = 0;
  let mut dummy: c_int = 0;

  eassert! (FRAME_X_P (f));

  block_input ();

  x_query_pointer (FRAME_X_DISPLAY! (f),
		   (*FRAME_DISPLAY_INFO! (f)).root_window,

		   /* The root window which contains the pointer.  */
		   &raw mut root,

		   /* Window pointer is on, not used  */
		   &raw mut dummy_window,

		   /* The position on that root window.  */
		   x, y,

		   /* x/y in dummy_window coordinates, not used.  */
		   &raw mut dummy, &raw mut dummy,

		   /* Modifier keys and pointer buttons, about which
		      we don't care.  */
		   &raw mut dummy as *mut c_uint);

  XTranslateCoordinates (FRAME_X_DISPLAY! (f),

                         /* From-window, to-window.  */
                         (*FRAME_DISPLAY_INFO! (f)).root_window,
                         FRAME_X_WINDOW! (f),

                         /* From-position, to-position.  */
                         *x, *y, x, y,

                         /* Child of win.  */
                         &raw mut dummy_window);

  unblock_input ();
}

/* Gamma-correct COLOR on frame F.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gamma_correct (f: *mut frame, color: *mut XColor)
{
  if (*f).gamma != 0.0
    {
      (*color).red = (pow ((*color).red as c_double / 65535.0, (*f).gamma) * 65535.0 + 0.5)
        as c_ushort;
      (*color).green = (pow ((*color).green as c_double / 65535.0, (*f).gamma) * 65535.0 + 0.5)
        as c_ushort;
      (*color).blue = (pow ((*color).blue as c_double / 65535.0, (*f).gamma) * 65535.0 + 0.5)
        as c_ushort;
    }
}


/* Decide if color named COLOR_NAME is valid for use on frame F.  If
   so, return the RGB values in COLOR.  If ALLOC_P,
   allocate the color.  Value is false if COLOR_NAME is invalid, or
   no color could be allocated.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_defined_color (f: *mut frame, color_name: *const c_char,
                                          color: *mut Emacs_Color, alloc_p: bool,
                                          _makeIndex: bool) -> bool
{
  let mut success_p: bool = false;
  let cmap: Colormap = FRAME_X_COLORMAP! (f);

  block_input ();
  #[cfg (feature = "use_gtk")]
  { success_p = xg_check_special_colors (f, color_name, color); }
  /* C: `success_p = x_parse_color (f, color_name, color) != 0;` - C's two argument types are the
     same one: dispextern.h 38's x11 arm is `typedef XColor Emacs_Color;` (C 27-45).  The mirror
     `include/dispextern.rs` carries the `#else` arm's record instead (its ledger
     `docs/ports/include-dispextern.md` tracks the WIP), so this unit keeps C's `XColor`
     prototype and converts at the call site (P-72 rule 3; `docs/ports/xfns.md` records the owed
     mirror row `pub type Emacs_Color = XColor;` under `have_x_windows`).  */
  if ! success_p
    { success_p = x_parse_color (f, color_name, color as *mut XColor) != 0; }
  if success_p && alloc_p
    { success_p = x_alloc_nearest_color (f, cmap, color as *mut XColor); }
  unblock_input ();

  success_p
}


/* Return the pixel color value for color COLOR_NAME on frame F.  If F
   is a monochrome frame, return MONO_COLOR regardless of what ARG says.
   Signal an error if color can't be allocated.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_decode_color (f: *mut frame, color_name: Lisp_Object, mono_color: c_int) -> c_ulong
{
  let mut cdef: XColor = std::mem::zeroed ();

  CHECK_STRING (color_name);

  /* C 699-706 is `#if false` - C's own dead text ("Don't do this.  It's wrong when we're not
     using the default colormap, ..."), which no configuration compiles.  `character.rs` /
     `cm.rs`'s `#if 0` precedent (docs/status.md 3.55): recorded in `docs/ports/xfns.md`, not
     transcribed.  */

  /* Return MONO_COLOR for monochrome frames.  */
  if (*FRAME_DISPLAY_INFO! (f)).n_planes == 1
    { return mono_color as c_ulong; }

  /* x_defined_color is responsible for coping with failures
     by looking for a near-miss.  */
  /* C: `XColor cdef;` (C 695) passed to `Emacs_Color *` - the same type in C through
     dispextern.h 38's alias; the mirror `include/dispextern.rs` carries the `#else` arm's record
     instead, so the conversion is at the call site (P-72 rule 3; the owed mirror row is the one
     `docs/ports/xfns.md` records).  */
  if x_defined_color (f, SSDATA (color_name), &raw mut cdef as *mut Emacs_Color, true, false)
    { return cdef.pixel; }

  signal_error (c"Undefined color".as_ptr (), color_name);
}


/* Change the `wait-for-wm' frame parameter of frame F.  OLD_VALUE is
   the previous value of that parameter, NEW_VALUE is the new value.
   See also the comment of wait_for_wm in struct x_output.  */

#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_wait_for_wm (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  /* C: `f->output_data.x->wait_for_wm = !NILP (new_value);`.  `wait_for_wm` is a `bool_bf`
     member of `struct x_output` (xterm.h 1226 and the six-member group at 1209-1230), and
     `include/xterm.rs`'s record carries that group as comments rather than as storage (its own
     ledger's "Bitfields are recorded, not packed"), so the member has no field to receive this
     write until the mirror gains the four-byte unit P-80 requires - measured
     `x11 wait_for_wm 0 1 4 1` in docs/baseline/x_output-bits.tsv (unit 1, bit 4, width 1).
     The C statement keeps C's own member name (the `xterm.rs` `buffered_input_event.sie`
     precedent for a member the mirror does not carry yet, whose resulting `E0609 no field` is
     this unit's flip-arm reading in docs/ports/xfns.md), and the owed mirror row is recorded
     there as well; P-80's member-dispatch accessor cannot stand here because the storage unit it
     dispatches on is the mirror's and is not in this tree.  */
  (*((*f).output_data.x as *mut x_output)).wait_for_wm = ! NILP (new_value);
}

/* C 124-126 and 736-802: `x_set_alpha_background`'s GTK-side names.  GTK's headers are the
   toolchain's own and this machine has no GTK tree (P-86 ③②'s second tier: `text-only`):
   the `GtkWidget`/`GtkWidgetClass`/`gtk_widget_set_app_paintable` declarations are the SDK
   mirror's now (`include/gtk.rs`, reached through C 63's chain above; P-90 ②'s
   one-surface-one-mirror), and what stays here is the two GLib cast macros below - their
   names and result types the port keeps, their cast chains stay owed to a GTK toolchain.
   `GtkWidget` is also the type `include/xterm.rs`'s `FRAME_GTK_OUTER_WIDGET` names bare
   (xterm.rs 378 lists it among that mirror's un-mirrored names); the edge above supplies
   it.  */
/// C: `<gobject/gobject.h>`: `#define G_OBJECT_GET_CLASS(object) (G_TYPE_INSTANCE_GET_CLASS
/// ((object), G_TYPE_OBJECT, GObjectClass))`, whose own body is `((GObjectClass *)
/// g_type_check_instance_cast ((GTypeInstance *) (instance), (g_type)))`.  The port keeps the
/// macro's name and its result type; the GLib checked-cast chain (`G_TYPE_INSTANCE_GET_CLASS`,
/// `g_type_check_instance_cast`, `G_TYPE_OBJECT`) is owed to a GTK toolchain, so the body is the
/// conversion that chain ends in - `docs/ports/xfns.md` records the deviation.
#[cfg (all (feature = "have_x_windows", feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! G_OBJECT_GET_CLASS {
  ($object:expr) => { ($object) as *mut GObjectClass };
}
/// C: gtk/gtkwidget.h: `#define GTK_WIDGET_CLASS(klass) (G_TYPE_CHECK_CLASS_CAST ((klass),
/// GTK_TYPE_WIDGET, GtkWidgetClass))` - the same treatment: the name and the result type are
/// kept, the GLib cast chain is owed.
#[cfg (all (feature = "have_x_windows", feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_WIDGET_CLASS {
  ($klass:expr) => { ($klass) as *mut GtkWidgetClass };
}
/// C: gobject/gobject.h: `typedef struct _GObjectClass GObjectClass;`
#[cfg (all (feature = "have_x_windows", feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct GObjectClass
{
  _private: [u8; 0],
}

/* C 732-802.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_alpha_background (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  use crate::x11_constants::{PropModeReplace, XA_CARDINAL};

  let mut opaque_region: [c_ulong; 4] = [0, 0, FRAME_PIXEL_WIDTH (f) as c_ulong,
                                         FRAME_PIXEL_HEIGHT (f) as c_ulong];
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  let mut object_class: *mut GObjectClass = std::ptr::null_mut ();
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  let mut class: *mut GtkWidgetClass = std::ptr::null_mut ();

  gui_set_alpha_background (f, arg, oldval);

  #[cfg (feature = "have_xrender")]
  {
    /* Setting `alpha_background' to something other than opaque on a
       display that doesn't support the required features leads to
       confusing results.  */
    if (*f).alpha_background < 1.0
        && (*FRAME_DISPLAY_INFO! (f)).alpha_bits == 0
        && ! FRAME_CHECK_XR_VERSION! (f, 0, 2)
      { (*f).alpha_background = 1.0; }
  }
  #[cfg (not (feature = "have_xrender"))]
  { (*f).alpha_background = 1.0; }

  #[cfg (feature = "use_gtk")]
  {
    /* This prevents GTK from painting the window's background, which
       interferes with transparent background in some environments */

    if ! FRAME_TOOLTIP_P (f)
      /* C: `gtk_widget_set_app_paintable (FRAME_GTK_OUTER_WIDGET (f),
         f->alpha_background != 1.0);` - C's `bool` argument widens to GObject's `gboolean`
         (`int`), which the port spells with the conversion.  */
      { gtk_widget_set_app_paintable (FRAME_GTK_OUTER_WIDGET! (f),
                                      ((*f).alpha_background != 1.0) as c_int); }
  }

  if (*FRAME_DISPLAY_INFO! (f)).alpha_bits == 0
    { return; }

  if (*f).alpha_background != 1.0
    {
      XChangeProperty (FRAME_X_DISPLAY! (f),
		       FRAME_X_WINDOW! (f),
		       (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_opaque_region,
		       XA_CARDINAL, 32, PropModeReplace,
		       std::ptr::null (), 0);
    }
  else
    {
      /* C 776-801: the `#else` arm C writes between this branch and the `HAVE_GTK3` one; the two
         arms of C's `#ifndef HAVE_GTK3` are the two `#[cfg]` blocks below, C's own order kept. */
      #[cfg (not (feature = "have_gtk3"))]
      { XChangeProperty (FRAME_X_DISPLAY! (f),
		     FRAME_X_WINDOW! (f),
		     (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_opaque_region,
		     XA_CARDINAL, 32, PropModeReplace,
		     &raw mut opaque_region as *mut c_uchar, 4); }

      #[cfg (feature = "have_gtk3")]
      {
	if FRAME_TOOLTIP_P (f)
	  { XChangeProperty (FRAME_X_DISPLAY! (f),
			 FRAME_X_WINDOW! (f),
			 (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_opaque_region,
			 XA_CARDINAL, 32, PropModeReplace,
			 &raw mut opaque_region as *mut c_uchar, 4); }
	else
	  {
	    object_class = G_OBJECT_GET_CLASS! (FRAME_GTK_OUTER_WIDGET! (f));
	    class = GTK_WIDGET_CLASS! (object_class);

	    if (*class).style_updated.is_some ()
	      { ((*class).style_updated.unwrap_unchecked ()) (FRAME_GTK_OUTER_WIDGET! (f)); }
	  }
      }
    }
}

/* ==========================================================================================
   The second slice: C 803-3341.  It opens at C 803 (the blank line after slice 1's `}`) and
   closes at C 3341, the `}` of `xic_set_statusarea`, the nearest whole-function boundary to
   3,000 C lines past the slice's start; the next definition is `x_xic_to_frame` (C 3342).
   Both arms of every `#if` are carried, and the gate mapping and per-item citations are in
   `docs/ports/xfns.md`.
   ========================================================================================== */

/* The X11 entry points this slice's plain-X11 arms call and `include/xlib.rs` does not carry
   yet (it declares the Xrm family, the screen/resource accessors and the seven `image.c`
   reads).  Each keeps C's prototype and cites the header and line of the same measurement base
   `include/xlib.rs` uses - libx11 1.8.13 and xorgproto 2025.1 under `/opt/homebrew/include` -
   and `docs/ports/xfns.md` names them as `include/xlib.rs` rows.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: Xlib.h 1572: `extern Cursor XCreateFontCursor (Display *, unsigned int);`
  pub fn XCreateFontCursor (display: *mut Display, shape: c_uint) -> Cursor;
  /// C: Xlib.h 2241: `extern int XDefineCursor (Display *, Window, Cursor);`
  pub fn XDefineCursor (display: *mut Display, w: Window, cursor: Cursor) -> c_int;
  /// C: Xlib.h 2247: `extern int XDeleteProperty (Display *, Window, Atom);`
  pub fn XDeleteProperty (display: *mut Display, w: Window, property: Atom) -> c_int;
  /// C: Xlib.h 2517: `extern int XFlush (Display *);`
  pub fn XFlush (display: *mut Display) -> c_int;
  /* C: Xlib.h 2526's `XFree` is already in `include/xlib.rs`'s extern block.  */
  /// C: Xlib.h 2543: `extern int XFreeCursor (Display *, Cursor);`
  pub fn XFreeCursor (display: *mut Display, cursor: Cursor) -> c_int;
  /// C: Xlib.h 2967: `extern int XQueryColor (Display *, Colormap, XColor *);`
  pub fn XQueryColor (display: *mut Display, cmap: Colormap, def: *mut XColor) -> c_int;
  /// C: Xlib.h 3070: `extern int XRecolorCursor (Display *, Cursor, XColor *, XColor *);`
  pub fn XRecolorCursor (display: *mut Display, cursor: Cursor, foreground: *mut XColor,
                         background: *mut XColor) -> c_int;
  /// C: Xlib.h 3097: `extern int XReparentWindow (Display *, Window, Window, int, int);`
  pub fn XReparentWindow (display: *mut Display, w: Window, parent_w: Window, x: c_int,
                          y: c_int) -> c_int;
  /// C: Xlib.h 3164: `extern int XSetBackground (Display *, GC, unsigned long);`
  pub fn XSetBackground (display: *mut Display, gc: GC, background: c_ulong) -> c_int;
  /// C: Xlib.h 3237: `extern int XSetForeground (Display *, GC, unsigned long);`
  pub fn XSetForeground (display: *mut Display, gc: GC, foreground: c_ulong) -> c_int;
  /// C: Xlib.h 3343: `extern int XSetWindowBackground (Display *, Window, unsigned long);`
  pub fn XSetWindowBackground (display: *mut Display, w: Window,
                               background_pixel: c_ulong) -> c_int;
  /// C: Xlib.h 3355: `extern int XSetWindowBorder (Display *, Window, unsigned long);`
  pub fn XSetWindowBorder (display: *mut Display, w: Window, border_pixel: c_ulong) -> c_int;
  /// C: Xlib.h 2095: `extern int XChangeWindowAttributes (Display *, Window, unsigned long,
  /// XSetWindowAttributes *);`
  pub fn XChangeWindowAttributes (display: *mut Display, w: Window,
                                  valuemask: c_ulong,
                                  attributes: *mut XSetWindowAttributes) -> c_int;
  /// C: Xutil.h 641: `extern int XSetWMHints (Display *, Window, XWMHints *);`
  pub fn XSetWMHints (display: *mut Display, w: Window, wm_hints: *mut XWMHints) -> c_int;
  /// C: Xutil.h 647: `extern void XSetWMIconName (Display *, Window, XTextProperty *);`
  pub fn XSetWMIconName (display: *mut Display, w: Window, text_prop: *mut XTextProperty);
  /// C: Xutil.h 653: `extern void XSetWMName (Display *, Window, XTextProperty *);`
  pub fn XSetWMName (display: *mut Display, w: Window, text_prop: *mut XTextProperty);
}

/// C: X11/Intrinsic.h: `typedef struct { String name; XtArgVal value; } Arg, *ArgList;` with
/// `typedef char *String;` and `typedef long XtArgVal;`.  **No libXt on this machine** (there is
/// no `<X11/Intrinsic.h>` under `/opt/homebrew`, `/usr/local` or `/Library`), so the record's
/// declarations name the header alone and carry no layout reading - the `include/gtk.rs` rule for
/// the GDK/GTK surface, and `docs/ports/xfns.md` names its rows for a real toolkit's dump.  The
/// three call sites are the `USE_X_TOOLKIT` arms of `x_set_no_accept_focus` and `x_set_name`/
/// `x_set_icon_name`; none of them is compiled by the plain-X11 or the GTK arm.
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
#[allow(non_camel_case_types)]
pub type String = *mut c_char;
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
#[allow(non_camel_case_types)]
pub type XtArgVal = c_long;
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
#[repr(C)]
pub struct Arg
{
  pub name: String,      // C: String name;
  pub value: XtArgVal,   // C: XtArgVal value;
}
/// C: X11/Intrinsic.h: `#define XtSetArg(arg, n, d) ((void)((arg).name = (n), (arg).value =
/// (XtArgVal)(d)))` - C's comma expression, both members written in C's order.
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
macro_rules! XtSetArg {
  ($arg:expr, $n:expr, $d:expr) => {
    { $arg.name = $n; $arg.value = $d as XtArgVal; } };
}
/// C: X11/Intrinsic.h: `extern void XtSetValues (Widget, ArgList, Cardinal);` and
/// `extern void XtVaSetValues (Widget, ...);`
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe extern "C" {
  pub fn XtSetValues (widget: Widget, args: *mut Arg, num_args: c_uint);
  pub fn XtVaSetValues (widget: Widget, ...);
  /// C: X11/Intrinsic.h: `#define XtDisplay(w) ((w)->core.screen->display)` /
  /// `#define XtWindow(w) ((w)->core.window)`
  pub fn XtDisplay (widget: Widget) -> *mut Display;
  pub fn XtWindow (widget: Widget) -> Window;
}

/* The X11 records and constants this slice's arms name that no mirror carries yet.  Each is
   measured with clang against the installed headers (libx11 1.8.13, xorgproto 2025.1 under
   `/opt/homebrew/include`); `docs/ports/xfns.md` names the `include/xlib.rs` rows they are owed
   to.  */
/// C: Xlib.h 283-302: `typedef struct { Pixmap background_pixmap; unsigned long
/// background_pixel; Pixmap border_pixmap; unsigned long border_pixel; int bit_gravity; int
/// win_gravity; int backing_store; unsigned long backing_planes; unsigned long backing_pixel;
/// Bool save_under; long event_mask; long do_not_propagate_mask; Bool override_redirect;
/// Colormap colormap; Cursor cursor; } XSetWindowAttributes;` - this slice writes exactly
/// `override_redirect` (C 1096).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XSetWindowAttributes
{
  pub background_pixmap: Pixmap,
  pub background_pixel: c_ulong,
  pub border_pixmap: Pixmap,
  pub border_pixel: c_ulong,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: Bool,
  pub event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool,
  pub colormap: Colormap,
  pub cursor: Cursor,
}
/* Measured on this machine (`clang -I/opt/homebrew/include` dump of the header's own record,
   2026-10-05): `sizeof=112 align=8`, `offsetof override_redirect=88`.  */
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XSetWindowAttributes> () == 112);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XSetWindowAttributes> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XSetWindowAttributes, override_redirect) == 88);

/// C: Xutil.h 163-168: `typedef struct { unsigned char *value; Atom encoding; int format;
/// unsigned long nitems; } XTextProperty;` - `x_set_name_internal` (C 2203) reads all four.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XTextProperty
{
  pub value: *mut c_uchar,   // C: unsigned char *value;
  pub encoding: Atom,        // C: Atom encoding;
  pub format: c_int,         // C: int format;
  pub nitems: c_ulong,       // C: unsigned long nitems;
}
/* Measured the same way: `sizeof=32 align=8`, offsets `value 0 encoding 8 format 16 nitems 24`.  */
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XTextProperty> () == 32);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XTextProperty, encoding) == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XTextProperty, nitems) == 24);

/// C: X11/X.h 139: `#define CurrentTime 0L /* special Time */` - the one X11 constant this
/// slice needs that `include/xlib.rs`'s `x11_constants` module does not carry (the module is the
/// integrator's; `docs/ports/xfns.md` names the owed row).
#[cfg (feature = "have_x_windows")]
pub const CurrentTime: Time = 0;

/* frame.h's flag-run members this slice *writes* - `undecorated` (C 917), `no_focus_on_map`
   (C 1031), `no_accept_focus` (C 1065) and `override_redirect` (C 1098).  `include/frame.rs`
   carries each as a read-only getter, so the port gives each the two-arm form `src/pgtkfns.rs`
   466-560 uses: the read arm is the mirror's getter, the write arm is the mirror's own flag
   constants through `frame_set_flag`.  */
/// C: frame.h 1239: `#define FRAME_UNDECORATED(f) ((f)->undecorated)`
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_UNDECORATED {
  ($f:expr) => { FRAME_UNDECORATED ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_UNDECORATED_UNIT, FRAME_UNDECORATED_SHIFT, ($v) as u32) };
}
/// C: frame.h 1266: `#define FRAME_NO_FOCUS_ON_MAP(f) ((f)->no_focus_on_map)`
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_NO_FOCUS_ON_MAP {
  ($f:expr) => { FRAME_NO_FOCUS_ON_MAP ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_FOCUS_ON_MAP_UNIT, FRAME_NO_FOCUS_ON_MAP_SHIFT, ($v) as u32) };
}
/// C: frame.h 1263: `#define FRAME_NO_ACCEPT_FOCUS(f) ((f)->no_accept_focus)`
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_NO_ACCEPT_FOCUS {
  ($f:expr) => { FRAME_NO_ACCEPT_FOCUS ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_ACCEPT_FOCUS_UNIT, FRAME_NO_ACCEPT_FOCUS_SHIFT, ($v) as u32) };
}
/// C: frame.h 1249: `#define FRAME_OVERRIDE_REDIRECT(f) ((f)->override_redirect)` - its unit is
/// 2, the first of the two the mirror's own flag run uses.
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_OVERRIDE_REDIRECT {
  ($f:expr) => { FRAME_OVERRIDE_REDIRECT ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_OVERRIDE_REDIRECT_FLAG_UNIT, FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT,
                    ($v) as u32) };
}

/* C's own entry points the arms above call, from headers whose mirrors do not carry them
   (P-72 rule 4).  Each keeps C's prototype and names the C header and line it came from.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: lisp.h 5117: `extern void store_frame_param (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
  /// C: frame.h 774: `fset_parent_frame (struct frame *f, Lisp_Object val)` (the C `INLINE`
  /// setter).
  pub fn fset_parent_frame (f: *mut frame, val: Lisp_Object);
  /// C: dispextern.h 3901: `extern void adjust_frame_glyphs (struct frame *);`
  pub fn adjust_frame_glyphs (f: *mut frame);

  /// C: xterm.h 1900: `extern void x_clear_under_internal_border (struct frame *f);`
  pub fn x_clear_under_internal_border (f: *mut frame);
  /// C: xterm.h 1773/1774: `extern void x_make_frame_visible (struct frame *);` / `extern void
  /// x_make_frame_invisible (struct frame *);`
  pub fn x_make_frame_visible (f: *mut frame);
  pub fn x_make_frame_invisible (f: *mut frame);
  /// C: dispextern.h 3748: `extern Lisp_Object image_find_image_file (Lisp_Object);`
  
  /// C: gtkutil.h 160: `extern void xg_change_toolbar_position (struct frame *f, Lisp_Object pos);`
  pub fn xg_change_toolbar_position (f: *mut frame, pos: Lisp_Object);
  /// C: gtkutil.h 200/203/204/205: `extern void xg_set_undecorated (struct frame *f, Lisp_Object
  /// undecorated);` / `xg_set_no_focus_on_map (…)` / `xg_set_no_accept_focus (…)` /
  /// `xg_set_override_redirect (…)`
  pub fn xg_set_undecorated (f: *mut frame, undecorated: Lisp_Object);
  pub fn xg_set_no_focus_on_map (f: *mut frame, no_focus_on_map: Lisp_Object);
  pub fn xg_set_no_accept_focus (f: *mut frame, no_accept_focus: Lisp_Object);
  pub fn xg_set_override_redirect (f: *mut frame, override_redirect: Lisp_Object);
}

/* The two GTK names the `use_gtk` arms of `x_set_parent_frame` need and `include/gtk.rs` does
   not carry yet (no GTK tree on this machine, so the citations name the header alone - the same
   rule that mirror's own GDK/GTK rows follow; `docs/ports/xfns.md` names them as its owed rows).
   `GTK_RESIZE_IMMEDIATE`/`GTK_RESIZE_QUEUE` are `gtk/gtkcontainer.h`'s enumerators and their
   values are **not measurable here**, so they stand as placeholders rather than invented
   behaviour (`src/xwidget.rs` 495-520's rule).  */
/// C: gtk/gtkcontainer.h: `void gtk_container_set_resize_mode (GtkContainer *container,
/// GtkResizeMode resize_mode);`
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn gtk_container_set_resize_mode (container: *mut GtkContainer,
                                        resize_mode: GtkResizeMode);
}
/// C: gtk/gtkcontainer.h: `typedef enum { GTK_RESIZE_PARENT, GTK_RESIZE_QUEUE,
/// GTK_RESIZE_IMMEDIATE } GtkResizeMode;` - placeholder values, not GTK's.
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub type GtkResizeMode = c_int;
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub const GTK_RESIZE_PARENT: GtkResizeMode = 0;        // placeholder, not GTK's value
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub const GTK_RESIZE_QUEUE: GtkResizeMode = 1;         // placeholder, not GTK's value
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub const GTK_RESIZE_IMMEDIATE: GtkResizeMode = 2;     // placeholder, not GTK's value
/// C: gdk/gdkx.h: `void gdk_x11_window_set_frame_sync_enabled (GdkWindow *window, gboolean
/// enabled);`
#[cfg (all (feature = "have_x_windows", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn gdk_x11_window_set_frame_sync_enabled (window: *mut GdkWindow, enabled: c_int);
}
/// C: X11/StringDefs.h: `#define XtNinput "input"` / `#define XtNborderColor "borderColor"` - no
/// libXt on this machine, so the strings' values cannot be read here and the rows name the
/// header alone.
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNinput () -> String { c"input".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNborderColor () -> String { c"borderColor".as_ptr () as *mut c_char }

/// C: X11/Intrinsic.h 37-47: `typedef struct _WidgetRec *Widget;` - the X toolkit's opaque
/// widget handle, again with no libXt here (the same header-alone rule as the record above).
/// This unit's `USE_X_TOOLKIT` arm names it, and so does `include/xterm.rs`'s `struct x_output`,
/// which is why the two `Widget` failures under `use_x_toolkit` are this unit's and the mirror's
/// at once; `docs/ports/xfns.md` records the row `include/xlib.rs` is owed.
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type Widget = *mut c_void;

/// C: X11/X.h 253: `typedef unsigned long Pixel;`
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type Pixel = c_ulong;


/* C 804-849.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_tool_bar_position (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  #[cfg (feature = "use_gtk")]
  let mut choice: Lisp_Object = std::ptr::null_mut ();

  #[cfg (feature = "use_gtk")]
  {
    choice = list4 (Qleft (), Qright (), Qtop (), Qbottom ());

    if !NILP (Fmemq (new_value, choice))
      {
        if !EQ (new_value, old_value)
          {
            xg_change_toolbar_position (f, new_value);
            fset_tool_bar_position (f, new_value);
          }
      }
    else
      { wrong_choice (choice, new_value); }
  }

  #[cfg (not (feature = "use_gtk"))]
  {
    if !EQ (new_value, Qtop ()) && !EQ (new_value, Qbottom ())
      { error (c"Tool bar position must be either `top' or `bottom'".as_ptr ()); }

    if EQ (new_value, old_value)
      { return; }

    /* Set the tool bar position.  */
    fset_tool_bar_position (f, new_value);

    /* Now reconfigure frame glyphs to place the tool bar at the
       bottom.  While the inner height has not changed, call
       `resize_frame_windows' to place each of the windows at its
       new position.  */

    adjust_frame_size (f, -1, -1, 3, false, Qtool_bar_position ());
    adjust_frame_glyphs (f);
    SET_FRAME_GARBAGED! (f);

    if FRAME_X_WINDOW! (f) != 0
      { x_clear_under_internal_border (f); }
  }
}

/* C 850-880: `#ifdef HAVE_XDBE`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xdbe"))]

unsafe extern "C" fn x_set_inhibit_double_buffering (f: *mut frame, new_value: Lisp_Object,
                                          old_value: Lisp_Object)
{
  let mut want_double_buffering: bool = false;
  let mut was_double_buffered: bool = false;

  if FRAME_X_WINDOW! (f) != 0 && !EQ (new_value, old_value)
    {
      want_double_buffering = NILP (new_value);
      was_double_buffered = FRAME_X_DOUBLE_BUFFERED_P! (f);

      block_input ();
      if want_double_buffering != was_double_buffered
        {
          /* Force XftDraw etc to be recreated with the new double
             buffered drawable.  */
          font_drop_xrender_surfaces (f);
        }
      if FRAME_X_DOUBLE_BUFFERED_P! (f) && !want_double_buffering
        { tear_down_x_back_buffer (f); }
      else if !FRAME_X_DOUBLE_BUFFERED_P! (f) && want_double_buffering
        { set_up_x_back_buffer (f); }
      if FRAME_X_DOUBLE_BUFFERED_P! (f) != was_double_buffered
        {
          SET_FRAME_GARBAGED! (f);
          font_drop_xrender_surfaces (f);
        }
      unblock_input ();
    }
}

/* C 882-921: the doc comment C 882-894 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_undecorated (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  use crate::x11_constants::PropModeReplace;

  if !EQ (new_value, old_value)
    {
      FRAME_UNDECORATED! (f, if NILP (new_value) { false } else { true });
      #[cfg (feature = "use_gtk")]
      { xg_set_undecorated (f, new_value); }
      #[cfg (not (feature = "use_gtk"))]
      {
        let dpy: *mut Display = FRAME_X_DISPLAY! (f);
        let mut hints: PropMotifWmHints = std::mem::zeroed ();
        let prop: Atom = (*FRAME_DISPLAY_INFO! (f)).Xatom_MOTIF_WM_HINTS;

        memset (&raw mut hints as *mut PropMotifWmHints as *mut c_void, 0,
                std::mem::size_of_val (&hints));
        hints.flags = MWM_HINTS_DECORATIONS as c_ulong;
        hints.decorations = if NILP (new_value) { MWM_DECOR_ALL as c_ulong } else { 0 };

        block_input ();
        /* For some reason the third and fourth arguments in the following
           call must be identical: In the corresponding XGetWindowProperty
           call in getMotifHints, xfwm has the third and seventh args both
           display_info->atoms[MOTIF_WM_HINTS].  Obviously, YMMV.   */
        XChangeProperty (dpy, FRAME_OUTER_WINDOW! (f), prop, prop, 32,
                         PropModeReplace, &raw mut hints as *mut c_uchar,
                         PROP_MOTIF_WM_HINTS_ELEMENTS);
        unblock_input ();
      }
    }
}

/* C 923-1003: the doc comment C 923-953 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_parent_frame (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  let mut p: *mut frame = std::ptr::null_mut ();
  #[cfg (feature = "have_gtk3")]
  let mut window: *mut GdkWindow = std::ptr::null_mut ();

  if !NILP (new_value)
      && (!FRAMEP (new_value)
          || { p = XFRAME (new_value); !FRAME_LIVE_P (p) }
          || !FRAME_X_P (p))
    {
      store_frame_param (f, Qparent_frame (), old_value);
      error (c"Invalid specification of `parent-frame'".as_ptr ());
    }

  if p != FRAME_PARENT_FRAME (f)
    {
      block_input ();
      XReparentWindow (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),
                       if !p.is_null () { FRAME_X_WINDOW! (p) }
                       else { (*FRAME_DISPLAY_INFO! (f)).root_window },
                       (*f).left_pos, (*f).top_pos);
      #[cfg (feature = "use_gtk")]
      {
        if EQ (x_gtk_resize_child_frames, Qresize_mode ())
          {
            gtk_container_set_resize_mode (GTK_CONTAINER! (FRAME_GTK_OUTER_WIDGET! (f)),
                                           if !p.is_null () { GTK_RESIZE_IMMEDIATE }
                                           else { GTK_RESIZE_QUEUE });
          }
      }

      #[cfg (feature = "have_gtk3")]
      {
        if !p.is_null ()
          {
            window = gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f));
            gdk_x11_window_set_frame_sync_enabled (window, 0);   /* C: FALSE */
          }
      }

      #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
                  feature = "have_clock_gettime"))]
      {
        /* Frame synchronization can't be used in child frames since
           they are not directly managed by the compositing manager.
           Re-enabling vsync in former child frames also leads to
           inconsistent display.  In addition, they can only be updated
           outside of a toplevel frame.  */
        (*(FRAME_X_OUTPUT! (f) as *mut x_output)).use_vsync_p = false;
        FRAME_X_WAITING_FOR_DRAW! (f) = false;
      }
      unblock_input ();

      fset_parent_frame (f, new_value);
    }
}

/* C 1005-1033: the doc comment C 1005-1017 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_no_focus_on_map (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  use crate::x11_constants::{PropModeReplace, XA_CARDINAL};

  if !EQ (new_value, old_value)
    {
      #[cfg (feature = "use_gtk")]
      { xg_set_no_focus_on_map (f, new_value); }
      #[cfg (not (feature = "use_gtk"))]
      {
        let dpy: *mut Display = FRAME_X_DISPLAY! (f);
        let prop: Atom = (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_user_time;
        let timestamp: Time = if NILP (new_value) { CurrentTime } else { 0 };

        XChangeProperty (dpy, FRAME_OUTER_WINDOW! (f), prop,
                         XA_CARDINAL, 32, PropModeReplace,
                         &raw const timestamp as *mut c_uchar, 1);
      }
      /* C's member lives inside frame.h's `#ifdef HAVE_WINDOW_SYSTEM` (C 452-479), so the write
         carries the macro the mirror's own flag constants carry.  */
      #[cfg (feature = "have_window_system")]
      FRAME_NO_FOCUS_ON_MAP! (f, !NILP (new_value));
    }
}

/* C 1035-1069: the doc comment C 1035-1044 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_no_accept_focus (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  if !EQ (new_value, old_value)
    {
      #[cfg (feature = "use_gtk")]
      { xg_set_no_accept_focus (f, new_value); }
      #[cfg (all (not (feature = "use_gtk"), feature = "use_x_toolkit"))]
      {
        let mut al: [Arg; 1] = [Arg { name: std::ptr::null_mut (), value: 0 }];

        XtSetArg! (al[0], XtNinput (), if NILP (new_value) { True } else { False });
        XtSetValues ((*((*f).output_data.x as *mut x_output)).widget, al.as_mut_ptr (), 1);
      }
      #[cfg (all (not (feature = "use_gtk"), not (feature = "use_x_toolkit")))]
      {
        let window: Window = FRAME_X_WINDOW! (f);

        (*((*f).output_data.x as *mut x_output)).wm_hints.input
          = if NILP (new_value) { True } else { False };
        XSetWMHints (FRAME_X_DISPLAY! (f), window,
                     &raw mut (*((*f).output_data.x as *mut x_output)).wm_hints);
      }
      FRAME_NO_ACCEPT_FOCUS! (f, !NILP (new_value));
    }
}

/* C 1071-1100: the doc comment C 1071-1079 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_override_redirect (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  use crate::x11_constants::CWOverrideRedirect;

  if !EQ (new_value, old_value)
    {
      /* Here (xfwm) override_redirect can be changed for invisible
         frames only.  */
      x_make_frame_invisible (f);

      #[cfg (feature = "use_gtk")]
      { xg_set_override_redirect (f, new_value); }
      #[cfg (not (feature = "use_gtk"))]
      {
        let mut attributes: XSetWindowAttributes = std::mem::zeroed ();

        attributes.override_redirect = if NILP (new_value) { False } else { True };
        XChangeWindowAttributes (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),
                                 CWOverrideRedirect as c_ulong, &raw mut attributes);
      }
      x_make_frame_visible (f);
      /* C's member lives inside frame.h's `#ifdef HAVE_WINDOW_SYSTEM` (C 452-479); see the note
         on `x_set_no_focus_on_map`.  */
      #[cfg (feature = "have_window_system")]
      FRAME_OVERRIDE_REDIRECT! (f, !NILP (new_value));
    }
}

/* C 1102-1177: `#ifdef USE_GTK`, the two global icon setters.  Their Xlib counterparts live in
   `xterm.c`; these are `xg_`-prefixed because they are the GTK build's.  */
/// C: gtkutil.h 143: `extern bool xg_set_icon (struct frame *f, Lisp_Object file);`
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_icon (f: *mut frame, file: Lisp_Object) -> bool
{
  let mut result: bool = false;
  let mut found: Lisp_Object = std::ptr::null_mut ();

  found = image_find_image_file (file);

  if !NILP (found)
    {
      let pixbuf: *mut GdkPixbuf;
      let mut err: *mut GError = std::ptr::null_mut ();
      let filename: *mut c_char = SSDATA (ENCODE_FILE (found));
      block_input ();

      pixbuf = gdk_pixbuf_new_from_file (filename, &raw mut err);

      if !pixbuf.is_null ()
        {
          gtk_window_set_icon (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)), pixbuf);
          g_object_unref (pixbuf as gpointer);

          result = true;
        }
      else
        { g_error_free (err); }

      unblock_input ();
    }

  result
}

/// C: gtkutil.h 144: `extern bool xg_set_icon_from_xpm_data (struct frame *f, const char **data);`
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_icon_from_xpm_data (f: *mut frame,
                                                    data: *mut *const c_char) -> bool
{
  /* gdk-pixbuf 2.44 deprecated gdk_pixbuf_new_from_xpm_data.
     Emacs should convert assets to PNG and use gdk_pixbuf_new_from_stream
     with a GMemoryInputStream, or transition to PNG via GResource.
     Pacify GCC for now.  */

  let pixbuf: *mut GdkPixbuf = gdk_pixbuf_new_from_xpm_data (data);

  if pixbuf.is_null ()
    { return false; }

  gtk_window_set_icon (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)), pixbuf);
  g_object_unref (pixbuf as gpointer);
  true
}

/* frame.h's two pixel accessors C *writes* (C 1194/1230); the read arm is the mirror's getter.
   C: frame.h 1234/1239: `#define FRAME_FOREGROUND_PIXEL(f) ((f)->foreground_pixel)` /
   `#define FRAME_BACKGROUND_PIXEL(f) ((f)->background_pixel)`  */
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_FOREGROUND_PIXEL {
  ($f:expr) => { FRAME_FOREGROUND_PIXEL ($f) };
  ($f:expr, $v:expr) => { (*$f).foreground_pixel = $v };
}
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_BACKGROUND_PIXEL {
  ($f:expr) => { FRAME_BACKGROUND_PIXEL ($f) };
  ($f:expr, $v:expr) => { (*$f).background_pixel = $v };
}

/* C 1179-1184: C's own comment (the family's contract).  Carried as this note; the three
   functions it describes follow in C's order.  */

/* C 1186-1220.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_foreground_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let x: *mut x_output = (*f).output_data.x as *mut x_output;
  let fg: c_ulong;
  let old_fg: c_ulong;

  fg = x_decode_color (f, arg, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;
  old_fg = FRAME_FOREGROUND_PIXEL (f);
  FRAME_FOREGROUND_PIXEL! (f, fg);

  if FRAME_X_WINDOW! (f) != 0
    {
      let dpy: *mut Display = FRAME_X_DISPLAY! (f);

      block_input ();
      XSetForeground (dpy, (*x).normal_gc, fg);
      XSetBackground (dpy, (*x).reverse_gc, fg);

      if (*x).cursor_pixel == old_fg
        {
          unload_color (f, (*x).cursor_pixel);
          (*x).cursor_pixel = x_copy_color (f, fg);
          XSetBackground (dpy, (*x).cursor_gc, (*x).cursor_pixel);
        }

      unblock_input ();

      update_face_from_frame_parameter (f, Qforeground_color (), arg);

      if FRAME_VISIBLE_P (f)
        { redraw_frame (f); }
    }

  unload_color (f, old_fg);
}

/* C 1222-1252.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_background_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let x: *mut x_output = (*f).output_data.x as *mut x_output;
  let bg: c_ulong;

  bg = x_decode_color (f, arg, WHITE_PIX_DEFAULT! (f) as c_int) as c_ulong;
  unload_color (f, FRAME_BACKGROUND_PIXEL (f));
  FRAME_BACKGROUND_PIXEL! (f, bg);

  if FRAME_X_WINDOW! (f) != 0
    {
      let dpy: *mut Display = FRAME_X_DISPLAY! (f);

      block_input ();
      XSetBackground (dpy, (*x).normal_gc, bg);
      XSetForeground (dpy, (*x).reverse_gc, bg);
      XSetWindowBackground (dpy, FRAME_X_WINDOW! (f), bg);
      XSetForeground (dpy, (*x).cursor_gc, bg);

      #[cfg (feature = "use_gtk")]
      { xg_set_background_color (f, bg); }

      unblock_input ();
      update_face_from_frame_parameter (f, Qbackground_color (), arg);

      if FRAME_VISIBLE_P (f)
        { redraw_frame (f); }
    }
}

/* C 1254-1272: `enum mouse_cursor`.  C's enum is `c_int` (PORTING.md 1.3) and the enumerators
   keep C's names.  */
#[cfg (feature = "have_x_windows")]
pub type mouse_cursor = c_int;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_text: mouse_cursor = 0;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_nontext: mouse_cursor = 1;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_hourglass: mouse_cursor = 2;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_mode: mouse_cursor = 3;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_hand: mouse_cursor = 4;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_horizontal_drag: mouse_cursor = 5;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_vertical_drag: mouse_cursor = 6;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_left_edge: mouse_cursor = 7;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_top_left_corner: mouse_cursor = 8;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_top_edge: mouse_cursor = 9;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_top_right_corner: mouse_cursor = 10;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_right_edge: mouse_cursor = 11;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_bottom_right_corner: mouse_cursor = 12;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_bottom_edge: mouse_cursor = 13;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_bottom_left_corner: mouse_cursor = 14;
#[cfg (feature = "have_x_windows")]
pub const mouse_cursor_max: mouse_cursor = 15;

/* C 1274-1286.  */
/// C: xfns.c 1274-1286: `struct mouse_cursor_types { const char *name; Lisp_Object
/// *shape_var_ptr; int default_shape; };`
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct mouse_cursor_types
{
  pub name: *const c_char,               // C: const char *name;
  pub shape_var_ptr: *mut Lisp_Object,   // C: Lisp_Object *shape_var_ptr;
  pub default_shape: c_int,              // C: int default_shape;
}

/* C 1288-1306: the table, one row per `enum mouse_cursor` member.  C's `static const struct
   mouse_cursor_types mouse_cursor_types[]` is 15 rows and C's own comment says it must stay in
   sync with the enum above.  The `XC_*` names are `<X11/cursorfont.h>`'s, reached in C through
   `xterm.h` 40 - the mirror that would carry them does not, so they are declared below with the
   values measured from the installed header.  */
#[cfg (feature = "have_x_windows")]
static mut mouse_cursor_types: [mouse_cursor_types; 15] =
  [
    mouse_cursor_types { name: c"text".as_ptr (), shape_var_ptr: &raw mut Vx_pointer_shape,
                         default_shape: XC_xterm },
    mouse_cursor_types { name: c"nontext".as_ptr (),
                         shape_var_ptr: &raw mut Vx_nontext_pointer_shape,
                         default_shape: XC_left_ptr },
    mouse_cursor_types { name: c"hourglass".as_ptr (),
                         shape_var_ptr: &raw mut Vx_hourglass_pointer_shape,
                         default_shape: XC_watch },
    mouse_cursor_types { name: c"modeline".as_ptr (),
                         shape_var_ptr: &raw mut Vx_mode_pointer_shape,
                         default_shape: XC_xterm },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_sensitive_text_pointer_shape,
                         default_shape: XC_hand2 },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_horizontal_drag_shape,
                         default_shape: XC_sb_h_double_arrow },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_vertical_drag_shape,
                         default_shape: XC_sb_v_double_arrow },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_left_edge_shape,
                         default_shape: XC_left_side },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_top_left_corner_shape,
                         default_shape: XC_top_left_corner },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_top_edge_shape,
                         default_shape: XC_top_side },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_top_right_corner_shape,
                         default_shape: XC_top_right_corner },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_right_edge_shape,
                         default_shape: XC_right_side },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_bottom_right_corner_shape,
                         default_shape: XC_bottom_right_corner },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_bottom_edge_shape,
                         default_shape: XC_bottom_side },
    mouse_cursor_types { name: std::ptr::null (), shape_var_ptr: &raw mut Vx_window_bottom_left_corner_shape,
                         default_shape: XC_bottom_left_corner },
  ];

/* C 1308-1331.  */
/// C: xfns.c 1308-1331: `struct mouse_cursor_data` - C's six members, in C's order; the three
/// arrays are `mouse_cursor_max` long, which is C's own length expression.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct mouse_cursor_data
{
  pub last_cursor_create_request: c_int,   // C: int last_cursor_create_request;
  pub error_cursor: c_int,                 // C: int error_cursor;
  pub cursor_num: [c_uint; mouse_cursor_max as usize],          // C: unsigned int cursor_num[mouse_cursor_max];
  pub cursor: [Cursor; mouse_cursor_max as usize],              // C: Cursor cursor[mouse_cursor_max];
  pub x_request_serial: [c_ulong; mouse_cursor_max as usize],   // C: unsigned long x_request_serial[mouse_cursor_max];
  pub error_string: *mut c_char,           // C: char *error_string;
}

/* C 1333-1350.  */
/// C: xterm.h 1753-1755: `extern void x_catch_errors_with_handler (Display *, x_special_error_handler,
/// void *);`
#[cfg (feature = "have_x_windows")]
pub type x_special_error_handler = Option<unsafe extern "C" fn (dpy: *mut Display,
                                                                event: *mut XErrorEvent,
                                                                error_string: *mut c_char,
                                                                data: *mut c_void)>;
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_catch_errors_with_handler (dpy: *mut Display, handler: x_special_error_handler,
                                      data: *mut c_void);
  /// C: xterm.h 1785: `extern Cursor x_create_font_cursor (struct x_display_info *, int);`
  pub fn x_create_font_cursor (dpyinfo: *mut x_display_info, num: c_int) -> Cursor;
  /// C: xterm.h 1786: `extern unsigned long x_copy_color (struct frame *, unsigned long);`
  pub fn x_copy_color (f: *mut frame, color: c_ulong) -> c_ulong;
  /// C: dispextern.h 3790/3791: `void unload_color (struct frame *, unsigned long);` /
  /// `void x_free_colors (struct frame *, unsigned long *, int);`
  pub fn unload_color (f: *mut frame, pixel: c_ulong);
  pub fn x_free_colors (f: *mut frame, pixels: *mut c_ulong, npixels: c_int);
  /// C: xterm.h 1797: `extern void x_query_colors (struct frame *f, XColor *, int);`
  pub fn x_query_colors (f: *mut frame, colors: *mut XColor, n: c_int);
  /// C: xterm.h 1765/1766/1768: `extern void x_uncatch_errors_after_check (void);` / `extern void
  /// x_ignore_errors_for_next_request (struct x_display_info *, unsigned int);` / `extern void
  /// x_stop_ignoring_errors (struct x_display_info *);`
  pub fn x_uncatch_errors_after_check ();
  pub fn x_ignore_errors_for_next_request (dpyinfo: *mut x_display_info, serial: c_uint);
  pub fn x_stop_ignoring_errors (dpyinfo: *mut x_display_info);
  /// C: dispextern.h 3794: `void update_face_from_frame_parameter (struct frame *, Lisp_Object,
  /// Lisp_Object);`
  pub fn update_face_from_frame_parameter (f: *mut frame, prop: Lisp_Object,
                                           newval: Lisp_Object);
  /// C: dispextern.h 3896: `extern void redraw_frame (struct frame *);`
  pub fn redraw_frame (f: *mut frame);
  /// C: dispextern.h 3693: `extern void gui_update_cursor (struct frame *, bool);`
  pub fn gui_update_cursor (f: *mut frame, on_p: bool);
  /// C: lisp.h 4409: `void set_frame_cursor_types (struct frame *, Lisp_Object);`
  pub fn set_frame_cursor_types (f: *mut frame, arg: Lisp_Object);

  /// C: gtkutil.h 183: `extern void xg_set_background_color (struct frame *f, unsigned long bg);`
  pub fn xg_set_background_color (f: *mut frame, bg: c_ulong);
  /* C: Xlib.h 1806's `extern unsigned long XNextRequest (Display *);` moved to
     `include/xlib.rs` in the sixty-sixth `xterm` slice (P-90 5's move-back): xfns.rs reaches that
     mirror, and the row's second copy was one of `check-mirror-dupes.py`'s two failures.  */
  /// C: Xlib.h 1839-1842: `extern int XScreenNumberOfScreen (Screen *);` - Xlib's own row, which
  /// stands only where `configure` found the function (`src/config.in` 1905's template); the
  /// substitute C 7473-7491 defines takes the other arm (status 3.927).
  #[cfg (feature = "have_xscreennumberofscreen")]
  pub fn XScreenNumberOfScreen (screen: *mut Screen) -> c_int;
  /// C: Xlib.h 93: `#define BlackPixel(dpy, scr) (ScreenOfDisplay(dpy,scr)->black_pixel)`
  pub fn BlackPixel (dpy: *mut Display, scr: c_int) -> c_ulong;
  /// C: Xlib.h 94: `#define WhitePixel(dpy, scr) (ScreenOfDisplay(dpy,scr)->white_pixel)`
  pub fn WhitePixel (dpy: *mut Display, scr: c_int) -> c_ulong;
}

/// C: xterm.h 1126/1130: `#define FRAME_X_DISPLAY(f) (FRAME_DISPLAY_INFO (f)->display)` /
/// `#define FRAME_X_SCREEN_NUMBER(f) (XScreenNumberOfScreen (FRAME_X_SCREEN (f)))`.  The mirror
/// carries both as `macro_rules!`; these two functions exist because the same mirror's
/// `BLACK_PIX_DEFAULT`/`WHITE_PIX_DEFAULT` bodies spell them as *calls*, which only a function
/// can satisfy (a `macro_rules!` needs its `!`).  Each body is the mirror's own expansion, and
/// `docs/ports/xfns.md` records the two `include/xterm.rs` rows the mirror still owes.
#[cfg (feature = "have_x_windows")]
pub unsafe fn FRAME_X_DISPLAY (f: *mut frame) -> *mut Display { FRAME_X_DISPLAY! (f) }
#[cfg (feature = "have_x_windows")]
pub unsafe fn FRAME_X_SCREEN_NUMBER (f: *mut frame) -> c_int { FRAME_X_SCREEN_NUMBER! (f) }

/// C: Xlib.h 920-928: `typedef struct { int type; Display *display; XID resourceid; unsigned
/// long serial; unsigned char error_code; unsigned char request_code; unsigned char minor_code;
/// } XErrorEvent;` - `x_set_mouse_color_handler` (C 1334) reads `serial`.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XErrorEvent
{
  pub r#type: c_int,        // C: int type;
  pub display: *mut Display, // C: Display *display;
  pub resourceid: XID,      // C: XID resourceid;
  pub serial: c_ulong,      // C: unsigned long serial;
  pub error_code: c_uchar,  // C: unsigned char error_code;
  pub request_code: c_uchar, // C: unsigned char request_code;
  pub minor_code: c_uchar,  // C: unsigned char minor_code;
}
/* Measured on this machine (`clang -I/opt/homebrew/include`, 2026-10-05): `sizeof=40 align=8`,
   `offsetof serial=24`, `offsetof error_code=32`.  */
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XErrorEvent> () == 40);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XErrorEvent, serial) == 24);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XErrorEvent, error_code) == 32);

/* `<X11/cursorfont.h>`'s thirteen shape numbers, reached in C through `xterm.h` 40 (the mirror
   `include/xterm.rs` does not carry the edge).  Every value is measured from the installed
   header (`/opt/homebrew/include/X11/cursorfont.h`, xorgproto 2025.1); `docs/ports/xfns.md`
   names the row `include/xlib.rs` is owed.  */
#[cfg (feature = "have_x_windows")]
pub const XC_xterm: c_int = 152;                 // C: cursorfont.h 109
#[cfg (feature = "have_x_windows")]
pub const XC_left_ptr: c_int = 68;                // C: cursorfont.h 67
#[cfg (feature = "have_x_windows")]
pub const XC_watch: c_int = 150;                  // C: cursorfont.h 108
#[cfg (feature = "have_x_windows")]
pub const XC_hand2: c_int = 60;                   // C: cursorfont.h 63
#[cfg (feature = "have_x_windows")]
pub const XC_sb_h_double_arrow: c_int = 108;      // C: cursorfont.h 87
#[cfg (feature = "have_x_windows")]
pub const XC_sb_v_double_arrow: c_int = 116;      // C: cursorfont.h 91
#[cfg (feature = "have_x_windows")]
pub const XC_left_side: c_int = 70;               // C: cursorfont.h 68
#[cfg (feature = "have_x_windows")]
pub const XC_top_left_corner: c_int = 134;        // C: cursorfont.h 100
#[cfg (feature = "have_x_windows")]
pub const XC_top_side: c_int = 138;               // C: cursorfont.h 102
#[cfg (feature = "have_x_windows")]
pub const XC_top_right_corner: c_int = 136;       // C: cursorfont.h 101
#[cfg (feature = "have_x_windows")]
pub const XC_right_side: c_int = 96;              // C: cursorfont.h 81
#[cfg (feature = "have_x_windows")]
pub const XC_bottom_right_corner: c_int = 14;     // C: cursorfont.h 40
#[cfg (feature = "have_x_windows")]
pub const XC_bottom_side: c_int = 16;             // C: cursorfont.h 41
#[cfg (feature = "have_x_windows")]
pub const XC_bottom_left_corner: c_int = 12;      // C: cursorfont.h 39

/* frame.h's `tab_bar_resized`/`tool_bar_resized`/`tool_bar_redisplayed`/`tab_bar_redisplayed`
   run and the tool-bar line/height pair: the same two-arm shape.  Each `_RESIZED` write is C's
   own assignment (C 1847, 1879, 1916, 1950) through the mirror's flag storage.  */
/// C: frame.h 504: `bool_bf tab_bar_resized : 1;`
#[cfg (feature = "have_x_windows")]
macro_rules! TAB_BAR_RESIZED {
  ($f:expr) => { frame_flag ($f, FRAME_TAB_BAR_RESIZED_UNIT, FRAME_TAB_BAR_RESIZED_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TAB_BAR_RESIZED_UNIT, FRAME_TAB_BAR_RESIZED_SHIFT, ($v) as u32) };
}
/// C: frame.h 503/504: `bool_bf tab_bar_redisplayed : 1;` - read at C 1847.
#[cfg (feature = "have_x_windows")]
macro_rules! TAB_BAR_REDISPLAYED {
  ($f:expr) => { frame_flag ($f, FRAME_TAB_BAR_REDISPLAYED_UNIT, FRAME_TAB_BAR_REDISPLAYED_SHIFT) };
}
/// C: frame.h 514: `bool_bf tool_bar_resized : 1;`
#[cfg (feature = "have_x_windows")]
macro_rules! TOOL_BAR_RESIZED {
  ($f:expr) => { frame_flag ($f, FRAME_TOOL_BAR_RESIZED_UNIT, FRAME_TOOL_BAR_RESIZED_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TOOL_BAR_RESIZED_UNIT, FRAME_TOOL_BAR_RESIZED_SHIFT, ($v) as u32) };
}
/// C: frame.h 513: `bool_bf tool_bar_redisplayed : 1;` - read at C 1950.
#[cfg (feature = "have_x_windows")]
macro_rules! TOOL_BAR_REDISPLAYED {
  ($f:expr) => {
    frame_flag ($f, FRAME_TOOL_BAR_REDISPLAYED_UNIT, FRAME_TOOL_BAR_REDISPLAYED_SHIFT) };
}
/// C: frame.h 1090/1091: `#define FRAME_TAB_BAR_LINES(f) (f)->tab_bar_lines` /
/// `#define FRAME_TAB_BAR_HEIGHT(f) (f)->tab_bar_height` - written at C 1822/1823.
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_TAB_BAR_LINES {
  ($f:expr) => { FRAME_TAB_BAR_LINES ($f) };
  ($f:expr, $v:expr) => { (*$f).tab_bar_lines = $v };
}
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr) => { FRAME_TAB_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).tab_bar_height = $v };
}
/// C: frame.h 1107/1108: `#define FRAME_TOOL_BAR_LINES(f) (f)->tool_bar_lines` /
/// `#define FRAME_TOOL_BAR_HEIGHT(f) (f)->tool_bar_height` - written at C 1899/1900/1927/1928.
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_TOOL_BAR_LINES {
  ($f:expr) => { FRAME_TOOL_BAR_LINES ($f) };
  ($f:expr, $v:expr) => { (*$f).tool_bar_lines = $v };
}
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_TOOL_BAR_HEIGHT {
  ($f:expr) => { FRAME_TOOL_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).tool_bar_height = $v };
}
/// C: frame.h 1093-1098: `#ifdef HAVE_EXT_TOOL_BAR` / `#define FRAME_EXTERNAL_TOOL_BAR(f)
/// (f)->external_tool_bar` / `#else` / `#define FRAME_EXTERNAL_TOOL_BAR(f) false` - written at
/// C 1903/1913.
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_EXTERNAL_TOOL_BAR {
  ($f:expr) => { FRAME_EXTERNAL_TOOL_BAR ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXTERNAL_TOOL_BAR_FLAG_UNIT, FRAME_EXTERNAL_TOOL_BAR_FLAG_SHIFT,
                    ($v) as u32) };
}

/* frame.h's four members C 1683-1766 *writes*, again as two-arm macros over the mirror's own
   getters and flag storage.  `FRAME_EXTERNAL_MENU_BAR`'s write arm names the mirror's flag
   constant, which exists only under `have_ext_menu_bar` (the arm C's own lisp.h 3931-3936 gives
   `USE_GTK`), so it carries that gate the way `src/pgtkfns.rs` 543-560 does.  */
/// C: frame.h 1086/1087: `#define FRAME_MENU_BAR_LINES(f) (f)->menu_bar_lines` /
/// `#define FRAME_MENU_BAR_HEIGHT(f) (f)->menu_bar_height` - written at C 1706/1707/1724/1725.
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_MENU_BAR_LINES {
  ($f:expr) => { FRAME_MENU_BAR_LINES ($f) };
  ($f:expr, $v:expr) => { (*$f).menu_bar_lines = $v };
}
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_MENU_BAR_HEIGHT {
  ($f:expr) => { FRAME_MENU_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).menu_bar_height = $v };
}
/// C: frame.h 1078-1083: `#ifdef HAVE_EXT_MENU_BAR` / `#define FRAME_EXTERNAL_MENU_BAR(f)
/// (f)->external_menu_bar` / `#else` / `#define FRAME_EXTERNAL_MENU_BAR(f) false` - written at
/// C 1710/1719.
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_EXTERNAL_MENU_BAR {
  ($f:expr) => { FRAME_EXTERNAL_MENU_BAR ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXTERNAL_MENU_BAR_FLAG_UNIT, FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT,
                    ($v) as u32) };
}

/* C's own entry points this batch calls (P-72 rule 4).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: xterm.h 1751: `extern bool x_text_icon (struct frame *, const char *);`
  pub fn x_text_icon (f: *mut frame, name: *const c_char) -> bool;
  /// C: xterm.h 1798: `extern void x_clear_area (struct frame *f, int, int, int, int);`
  pub fn x_clear_area (f: *mut frame, x: c_int, y: c_int, w: c_int, h: c_int);

  /// C: dispextern.h 3906: `void clear_glyph_matrix (struct glyph_matrix *);`
  
  /* `fset_redisplay` (window.h 1168) is already declared by `include/window.rs`.  */
  /// C: xterm.h 1876: `extern void font_drop_xrender_surfaces (struct frame *f);`
  pub fn font_drop_xrender_surfaces (f: *mut frame);
  /// C: lisp.h 5120: `extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);`
  pub fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;
  /// C: dispextern.h 3940: `extern void clear_frame (struct frame *);`
  pub fn clear_frame (f: *mut frame);
  /// C: dispextern.h 3907: `void clear_current_matrices (struct frame *f);`
  pub fn clear_current_matrices (f: *mut frame);

  /// C: gtkutil.h 158/159: `extern void update_frame_tool_bar (struct frame *f);` / `extern void
  /// free_frame_tool_bar (struct frame *f);`
  pub fn update_frame_tool_bar (f: *mut frame);
  pub fn free_frame_tool_bar (f: *mut frame);
  /* `gtk_css_provider_load_from_data` is already in `include/gtk.rs` (gtkutil.h 243's own
     prototype), so the two scroll-bar setters use that copy.  */
}

/* C 1333-1350.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_set_mouse_color_handler (dpy: *mut Display, event: *mut XErrorEvent,
                                                error_string: *mut c_char, data: *mut c_void)
{
  let cursor_data: *mut mouse_cursor_data = data as *mut mouse_cursor_data;
  let mut i: c_int;

  (*cursor_data).error_cursor = -1;
  (*cursor_data).error_string = error_string;
  i = 0;
  while i < (*cursor_data).last_cursor_create_request
    {
      if (*event).serial >= (*cursor_data).x_request_serial[i as usize]
        { (*cursor_data).error_cursor = i; }
      i += 1;
    }
  if (*cursor_data).error_cursor >= 0
    {
      /* If we failed to allocate it, don't try to free it.  */
      (*cursor_data).cursor[(*cursor_data).error_cursor as usize] = 0;
    }
}

/* C 1352-1478.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_mouse_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let x: *mut x_output = (*f).output_data.x as *mut x_output;
  let dpy: *mut Display = FRAME_X_DISPLAY! (f);
  /* C's `struct mouse_cursor_data cursor_data = { -1, -1 };` - C's aggregate initialisation
     writes the first two members and zero-fills the rest.  */
  let mut cursor_data: mouse_cursor_data
    = mouse_cursor_data { last_cursor_create_request: -1, error_cursor: -1,
                          cursor_num: [0; mouse_cursor_max as usize],
                          cursor: [0; mouse_cursor_max as usize],
                          x_request_serial: [0; mouse_cursor_max as usize],
                          error_string: std::ptr::null_mut () };
  let mut pixel: c_ulong = x_decode_color (f, arg, BLACK_PIX_DEFAULT! (f) as c_int);
  let mask_color: c_ulong = FRAME_BACKGROUND_PIXEL (f);
  let mut i: c_int;

  /* Don't let pointers be invisible.  */
  if mask_color == pixel
    {
      x_free_colors (f, &raw mut pixel, 1);
      pixel = x_copy_color (f, FRAME_FOREGROUND_PIXEL (f));
    }

  unload_color (f, (*x).mouse_pixel);
  (*x).mouse_pixel = pixel;

  i = 0;
  while i < mouse_cursor_max
    {
      let shape_var: Lisp_Object = *mouse_cursor_types[i as usize].shape_var_ptr;
      cursor_data.cursor_num[i as usize]
        = (if !NILP (shape_var)
             { check_uinteger_max (shape_var, UINT_MAX as uintmax_t) }
           else
             { mouse_cursor_types[i as usize].default_shape as uintmax_t }) as c_uint;
      i += 1;
    }

  block_input ();

  /* It's not okay to crash if the user selects a screwy cursor.  */
  x_catch_errors_with_handler (dpy, Some (x_set_mouse_color_handler),
                               &raw mut cursor_data as *mut c_void);

  i = 0;
  while i < mouse_cursor_max
    {
      cursor_data.x_request_serial[i as usize] = XNextRequest (dpy);
      cursor_data.last_cursor_create_request = i;

      cursor_data.cursor[i as usize]
        = x_create_font_cursor (FRAME_DISPLAY_INFO! (f),
                                cursor_data.cursor_num[i as usize] as c_int);
      i += 1;
    }

  /* Now sync up and process all received errors from cursor
     creation.  */
  if x_had_errors_p (dpy)
    {
      let mut bad_cursor_name: *const c_char = std::ptr::null ();
      let mut xmessage: [c_char; X_ERROR_MESSAGE_SIZE as usize]
        = [0; X_ERROR_MESSAGE_SIZE as usize];
      eassert! (strlen (cursor_data.error_string) < std::mem::size_of_val (&xmessage));
      strcpy (xmessage.as_mut_ptr (), cursor_data.error_string);

      x_uncatch_errors_after_check ();

      /* XFreeCursor can generate BadCursor errors, because
	 XCreateFontCursor is not a request that waits for a reply,
	 and as such can return IDs that will not actually be used by
	 the server.  */
      x_ignore_errors_for_next_request (FRAME_DISPLAY_INFO! (f), 0);

      /* Free any successfully created cursors.  */
      i = 0;
      while i < mouse_cursor_max
        {
          if cursor_data.cursor[i as usize] != 0
            { XFreeCursor (dpy, cursor_data.cursor[i as usize]); }
          i += 1;
        }

      x_stop_ignoring_errors (FRAME_DISPLAY_INFO! (f));

      /* This should only be able to fail if the server's serial
	 number tracking is broken.  */
      if cursor_data.error_cursor >= 0
        { bad_cursor_name = mouse_cursor_types[cursor_data.error_cursor as usize].name; }
      if !bad_cursor_name.is_null ()
        { error (c"Bad %s pointer cursor: %s".as_ptr (), bad_cursor_name, xmessage.as_ptr ()); }
      else
        { error (c"Can't set cursor shape: %s".as_ptr (), xmessage.as_ptr ()); }
    }

  x_uncatch_errors_after_check ();

  {
    let mut colors: [XColor; 2] = [std::mem::zeroed (); 2]; /* 0=foreground, 1=background */

    colors[0].pixel = (*x).mouse_pixel;
    colors[1].pixel = mask_color;
    x_query_colors (f, colors.as_mut_ptr (), 2);

    i = 0;
    while i < mouse_cursor_max
      {
        XRecolorCursor (dpy, cursor_data.cursor[i as usize], &raw mut colors[0],
                        &raw mut colors[1]);
        i += 1;
      }
  }

  if FRAME_X_WINDOW! (f) != 0
    {
      (*((*f).output_data.x as *mut x_output)).current_cursor
        = cursor_data.cursor[mouse_cursor_text as usize];
      XDefineCursor (dpy, FRAME_X_WINDOW! (f),
                     (*((*f).output_data.x as *mut x_output)).current_cursor);
    }

  /* C 1450-1454's `INSTALL_CURSOR(FIELD, SHORT_INDEX)`: C's `##` paste has no Rust equivalent,
     so the port's macro takes the pasted name as its own argument.  The four statements and
     their order are C's.  */
  macro_rules! INSTALL_CURSOR {
    ($x:expr, $cursor_data:expr, $field:ident, $index:expr) => {
      eassert! ((*($x)).$field != $cursor_data.cursor[$index]);
      if (*($x)).$field != 0
        { XFreeCursor (dpy, (*($x)).$field); }
      (*($x)).$field = $cursor_data.cursor[$index];
    };
  }

  INSTALL_CURSOR! (x, cursor_data, text_cursor, mouse_cursor_text as usize);
  INSTALL_CURSOR! (x, cursor_data, nontext_cursor, mouse_cursor_nontext as usize);
  INSTALL_CURSOR! (x, cursor_data, hourglass_cursor, mouse_cursor_hourglass as usize);
  INSTALL_CURSOR! (x, cursor_data, modeline_cursor, mouse_cursor_mode as usize);
  INSTALL_CURSOR! (x, cursor_data, hand_cursor, mouse_cursor_hand as usize);
  INSTALL_CURSOR! (x, cursor_data, horizontal_drag_cursor,
                    mouse_cursor_horizontal_drag as usize);
  INSTALL_CURSOR! (x, cursor_data, vertical_drag_cursor, mouse_cursor_vertical_drag as usize);
  INSTALL_CURSOR! (x, cursor_data, left_edge_cursor, mouse_cursor_left_edge as usize);
  INSTALL_CURSOR! (x, cursor_data, top_left_corner_cursor,
                    mouse_cursor_top_left_corner as usize);
  INSTALL_CURSOR! (x, cursor_data, top_edge_cursor, mouse_cursor_top_edge as usize);
  INSTALL_CURSOR! (x, cursor_data, top_right_corner_cursor,
                    mouse_cursor_top_right_corner as usize);
  INSTALL_CURSOR! (x, cursor_data, right_edge_cursor, mouse_cursor_right_edge as usize);
  INSTALL_CURSOR! (x, cursor_data, bottom_right_corner_cursor,
                    mouse_cursor_bottom_right_corner as usize);
  INSTALL_CURSOR! (x, cursor_data, bottom_edge_cursor, mouse_cursor_bottom_edge as usize);
  INSTALL_CURSOR! (x, cursor_data, bottom_left_corner_cursor,
                    mouse_cursor_bottom_left_corner as usize);

  XFlush (dpy);
  unblock_input ();

  update_face_from_frame_parameter (f, Qmouse_color (), arg);
}

/* C 1480-1545.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_cursor_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut fore_pixel: c_ulong;
  let mut pixel: c_ulong;
  let mut fore_pixel_allocated_p: bool = false;
  let mut pixel_allocated_p: bool = false;
  let x: *mut x_output = (*f).output_data.x as *mut x_output;

  if !NILP (Vx_cursor_fore_pixel)
    {
      fore_pixel = x_decode_color (f, Vx_cursor_fore_pixel,
                                   WHITE_PIX_DEFAULT! (f) as c_int);
      fore_pixel_allocated_p = true;
    }
  else
    { fore_pixel = FRAME_BACKGROUND_PIXEL (f); }

  pixel = x_decode_color (f, arg, BLACK_PIX_DEFAULT! (f) as c_int);
  pixel_allocated_p = true;

  /* Make sure that the cursor color differs from the background color.  */
  if pixel == FRAME_BACKGROUND_PIXEL (f)
    {
      if pixel_allocated_p
        {
          x_free_colors (f, &raw mut pixel, 1);
          pixel_allocated_p = false;
        }

      pixel = (*x).mouse_pixel;
      if pixel == fore_pixel
        {
          if fore_pixel_allocated_p
            {
              x_free_colors (f, &raw mut fore_pixel, 1);
              fore_pixel_allocated_p = false;
            }
          fore_pixel = FRAME_BACKGROUND_PIXEL (f);
        }
    }

  unload_color (f, (*x).cursor_foreground_pixel);
  if !fore_pixel_allocated_p
    { fore_pixel = x_copy_color (f, fore_pixel); }
  (*x).cursor_foreground_pixel = fore_pixel;

  unload_color (f, (*x).cursor_pixel);
  if !pixel_allocated_p
    { pixel = x_copy_color (f, pixel); }
  (*x).cursor_pixel = pixel;

  if FRAME_X_WINDOW! (f) != 0
    {
      block_input ();
      XSetBackground (FRAME_X_DISPLAY! (f), (*x).cursor_gc, (*x).cursor_pixel);
      XSetForeground (FRAME_X_DISPLAY! (f), (*x).cursor_gc, fore_pixel);
      unblock_input ();

      if FRAME_VISIBLE_P (f)
        {
          gui_update_cursor (f, false);
          gui_update_cursor (f, true);
        }
    }

  update_face_from_frame_parameter (f, Qcursor_color (), arg);
}

/* C 1547-1581: the comment C 1547-1550 and the function.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_border_pixel (f: *mut frame, pix: c_ulong)
{
  unload_color (f, (*((*f).output_data.x as *mut x_output)).border_pixel);
  (*((*f).output_data.x as *mut x_output)).border_pixel = pix;

  #[cfg (feature = "use_x_toolkit")]
  {
    if !(*((*f).output_data.x as *mut x_output)).widget.is_null () && (*f).border_width > 0
      {
        block_input ();
        XtVaSetValues ((*((*f).output_data.x as *mut x_output)).widget, XtNborderColor (),
                       pix as Pixel, std::ptr::null_mut::<c_void> ());
        unblock_input ();

        if FRAME_VISIBLE_P (f)
          { redraw_frame (f); }

        return;
      }
  }

  if FRAME_X_WINDOW! (f) != 0 && (*f).border_width > 0
    {
      block_input ();
      XSetWindowBorder (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), pix);
      unblock_input ();

      if FRAME_VISIBLE_P (f)
        { redraw_frame (f); }
    }
}

/* C 1583-1603: the comment C 1583-1592 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_border_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let pix: c_ulong;

  CHECK_STRING (arg);
  pix = x_decode_color (f, arg, BLACK_PIX_DEFAULT! (f) as c_int);
  x_set_border_pixel (f, pix);
  update_face_from_frame_parameter (f, Qborder_color (), arg);
}

/* C 1606-1610.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_cursor_type (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  set_frame_cursor_types (f, arg);
}

/* C 1612-1642.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_icon_type (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut result: bool;

  if STRINGP (arg)
    {
      if STRINGP (oldval) && BASE_EQ (Fstring_equal (oldval, arg), Qt ())
        { return; }
    }
  else if !STRINGP (oldval) && NILP (oldval) == NILP (arg)
    { return; }

  block_input ();
  if NILP (arg)
    {
      result = x_text_icon (f,
                            SSDATA (if !NILP ((*f).icon_name) { (*f).icon_name }
                                    else { (*f).name }));
    }
  else
    {
      result = ((*FRAME_TERMINAL! (f)).set_bitmap_icon_hook.unwrap_unchecked ())
                 (f as *mut c_void, arg);
    }

  if result
    {
      unblock_input ();
      error (c"No icon window available".as_ptr ());
    }

  XFlush (FRAME_X_DISPLAY! (f));
  unblock_input ();
}

/* C 1644-1679.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_icon_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut result: bool;

  if STRINGP (arg)
    {
      if STRINGP (oldval) && BASE_EQ (Fstring_equal (oldval, arg), Qt ())
        { return; }
    }
  else if !NILP (arg) || NILP (oldval)
    { return; }

  fset_icon_name (f, arg);

  if (*((*f).output_data.x as *mut x_output)).icon_bitmap != 0
    { return; }

  block_input ();

  result = x_text_icon (f,
                        SSDATA (if !NILP ((*f).icon_name) { (*f).icon_name }
                                else if !NILP ((*f).title) { (*f).title }
                                else { (*f).name }));

  if result
    {
      unblock_input ();
      error (c"No icon window available".as_ptr ());
    }

  XFlush (FRAME_X_DISPLAY! (f));
  unblock_input ();
}

/* C 1682-1766.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_menu_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let mut nlines: c_int;
  #[cfg (all (not (feature = "use_x_toolkit"), not (feature = "use_gtk")))]
  let mut olines: c_int = FRAME_MENU_BAR_LINES (f);

  /* Right now, menu bars don't work properly in minibuf-only frames;
     most of the commands try to apply themselves to the minibuffer
     frame itself, and get an error because you can't switch buffers
     in or split the minibuffer window.  */
  if FRAME_MINIBUF_ONLY_P (f) || FRAME_PARENT_FRAME (f) != std::ptr::null_mut ()
    { return; }

  if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNUM (value) as c_int; }
  else
    { nlines = 0; }

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay (f as _);

  #[cfg (any (feature = "use_x_toolkit", feature = "use_gtk"))]
  {
    FRAME_MENU_BAR_LINES! (f, 0);
    FRAME_MENU_BAR_HEIGHT! (f, 0);
    if nlines != 0
      {
        /* C's member and the mirror's flag constants live under frame.h's
           `#ifdef HAVE_WINDOW_SYSTEM` (C 382-390).  */
        #[cfg (feature = "have_window_system")]
        FRAME_EXTERNAL_MENU_BAR! (f, 1);
        if FRAME_X_P (f) && (*((*f).output_data.x as *mut x_output)).menubar_widget.is_null ()
          {
            /* Make sure next redisplay shows the menu bar.  */
            (*XWINDOW (FRAME_SELECTED_WINDOW (f))).update_mode_line = true;
          }
      }
    else
      {
        if (FRAME_EXTERNAL_MENU_BAR (f) as c_int) == 1
          { free_frame_menubar (f); }
        #[cfg (feature = "have_window_system")]
        FRAME_EXTERNAL_MENU_BAR! (f, 0);
        if FRAME_X_P (f)
          { (*((*f).output_data.x as *mut x_output)).menubar_widget = std::ptr::null_mut (); }
      }
  }
  #[cfg (all (not (feature = "use_x_toolkit"), not (feature = "use_gtk")))]
  {
    FRAME_MENU_BAR_LINES! (f, nlines);
    FRAME_MENU_BAR_HEIGHT! (f, nlines * FRAME_LINE_HEIGHT (f));
    if FRAME_X_WINDOW! (f) != 0
      { x_clear_under_internal_border (f); }

    /* If the menu bar height gets changed, the internal border below
       the top margin has to be cleared.  Also, if the menu bar gets
       larger, the area for the added lines has to be cleared except for
       the first menu bar line that is to be drawn later.  */
    if nlines != olines
      {
        let mut height: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);
        let width: c_int = FRAME_PIXEL_WIDTH (f);
        let mut y: c_int;

        adjust_frame_size (f, -1, -1, 3, true, Qmenu_bar_lines ());

        /* height can be zero here. */
        if FRAME_X_WINDOW! (f) != 0 && height > 0 && width > 0
          {
            y = FRAME_TOP_MARGIN_HEIGHT (f);

            block_input ();
            x_clear_area (f, 0, y, width, height);
            unblock_input ();
          }

        if nlines > 1 && nlines > olines
          {
            y = (if olines == 0 { 1 } else { olines }) * FRAME_LINE_HEIGHT (f);
            height = nlines * FRAME_LINE_HEIGHT (f) - y;

            block_input ();
            x_clear_area (f, 0, y, width, height);
            unblock_input ();
          }

        #[cfg (all (feature = "have_window_system", not (feature = "have_ext_menu_bar")))]
        if nlines == 0 && WINDOWP ((*f).menu_bar_window)
          {
            clear_glyph_matrix ((*XWINDOW ((*f).menu_bar_window)).current_matrix
                                as *mut glyph_matrix);
          }
      }
  }
  adjust_frame_glyphs (f);
}

/* C 1769-1794: the comment C 1769-1774 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_tab_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let olines: c_int = FRAME_TAB_BAR_LINES (f);
  let mut nlines: c_int;

  /* Treat tab bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    { return; }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP (0, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  if nlines != olines && (olines == 0 || nlines == 0)
    { x_change_tab_bar_height (f, nlines * FRAME_LINE_HEIGHT (f)); }
}

/* C 1797-1859: the comment C 1797 and the function.  C's `void x_change_tab_bar_height` is not
   `static`, so the port keeps the linkage (PORTING.md 0.2).  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_change_tab_bar_height (f: *mut frame, height: c_int)
{
  let unit: c_int = FRAME_LINE_HEIGHT (f);
  let old_height: c_int = FRAME_TAB_BAR_HEIGHT (f);

  /* This differs from the tool bar code in that the tab bar height is
     not rounded up.  Otherwise, if redisplay_tab_bar decides to grow
     the tab bar by even 1 pixel, FRAME_TAB_BAR_LINES will be changed,
     leading to the tab bar height being incorrectly set upon the next
     call to x_set_font.  (bug#59285) */
  let mut lines: c_int = height / unit;

  /* Even so, HEIGHT might be less than unit if the tab bar face is
     not so tall as the frame's font height; which if true lines will
     be set to 0 and the tab bar will thus vanish.  */

  if lines == 0 && height != 0
    { lines = 1; }

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay (f as _);

  /* Recalculate tab bar and frame text sizes.  */
  FRAME_TAB_BAR_HEIGHT! (f, height);
  FRAME_TAB_BAR_LINES! (f, lines);
  store_frame_param (f, Qtab_bar_lines (), make_fixnum (lines as EMACS_INT));

  if FRAME_X_WINDOW! (f) != 0 && FRAME_TAB_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  #[cfg (feature = "have_window_system")]
  if height < old_height && WINDOWP ((*f).tab_bar_window)
    {
      clear_glyph_matrix ((*XWINDOW ((*f).tab_bar_window)).current_matrix
                          as *mut glyph_matrix);
    }

  if !TAB_BAR_RESIZED! (f)
    {
      let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

      /* As long as tab_bar_resized is false, effectively try to change
	 F's native height.  */
      if NILP (fullscreen) || EQ (fullscreen, Qfullwidth ())
        {
          adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
                             1, false, Qtab_bar_lines ());
        }
      else
        { adjust_frame_size (f, -1, -1, 4, false, Qtab_bar_lines ()); }

      TAB_BAR_RESIZED! (f, TAB_BAR_REDISPLAYED! (f));
    }
  else
    {
      /* Any other change may leave the native size of F alone.  */
      adjust_frame_size (f, -1, -1, 3, false, Qtab_bar_lines ());
    }

  /* adjust_frame_size might not have done anything, garbage frame
     here.  */
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);
  if FRAME_X_WINDOW! (f) != 0
    { x_clear_under_internal_border (f); }
}

/* C 1862-1891: the comment C 1862-1866 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_tool_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let nlines: c_int;

  /* Treat tool bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    {
      #[cfg (feature = "use_gtk")]
      {
        /* Make sure implied resizing of minibuffer-only frames can be
	   inhibited too.  */
        TOOL_BAR_RESIZED! (f, true);
      }
      return;
    }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP (0, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  x_change_tool_bar_height (f, nlines * FRAME_LINE_HEIGHT (f));
}

/* C 1894-1964: the comment C 1894 and the function.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_change_tool_bar_height (f: *mut frame, height: c_int)
{
  #[cfg (feature = "use_gtk")]
  {
    FRAME_TOOL_BAR_LINES! (f, 0);
    FRAME_TOOL_BAR_HEIGHT! (f, 0);
    if height != 0
      {
        /* C's member and the mirror's flag constants live under frame.h's
           `#ifdef HAVE_WINDOW_SYSTEM` (C 390-397).  */
        #[cfg (feature = "have_window_system")]
        FRAME_EXTERNAL_TOOL_BAR! (f, true);
        if FRAME_X_P (f)
            && (*((*f).output_data.x as *mut x_output)).toolbar_widget.is_null ()
          {
            /* Make sure next redisplay shows the tool bar.  */
            (*XWINDOW (FRAME_SELECTED_WINDOW (f))).update_mode_line = true;
          }
        update_frame_tool_bar (f);
      }
    else
      {
        if FRAME_EXTERNAL_TOOL_BAR (f)
          { free_frame_tool_bar (f); }
        #[cfg (feature = "have_window_system")]
        FRAME_EXTERNAL_TOOL_BAR! (f, false);
        /* Make sure implied resizing of frames without initial tool bar
	   can be inhibited too.  */
        TOOL_BAR_RESIZED! (f, true);
      }
  }
  #[cfg (not (feature = "use_gtk"))]
  {
    let unit: c_int = FRAME_LINE_HEIGHT (f);
    let old_height: c_int = FRAME_TOOL_BAR_HEIGHT (f);
    let lines: c_int = (height + unit - 1) / unit;
    let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

    /* Make sure we redisplay all windows in this frame.  */
    fset_redisplay (f as _);

    FRAME_TOOL_BAR_HEIGHT! (f, height);
    FRAME_TOOL_BAR_LINES! (f, lines);
    store_frame_param (f, Qtool_bar_lines (), make_fixnum (lines as EMACS_INT));

    if FRAME_X_WINDOW! (f) != 0 && FRAME_TOOL_BAR_HEIGHT (f) == 0
      {
        clear_frame (f);
        clear_current_matrices (f);
      }

    #[cfg (feature = "have_window_system")]
    if height < old_height && WINDOWP ((*f).tool_bar_window)
      {
        clear_glyph_matrix ((*XWINDOW ((*f).tool_bar_window)).current_matrix
                            as *mut glyph_matrix);
      }

    if !TOOL_BAR_RESIZED! (f)
      {
        /* As long as tool_bar_resized is false, effectively try to change
	   F's native height.  */
        if NILP (fullscreen) || EQ (fullscreen, Qfullwidth ())
          {
            adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
                               1, false, Qtool_bar_lines ());
          }
        else
          { adjust_frame_size (f, -1, -1, 4, false, Qtool_bar_lines ()); }

        TOOL_BAR_RESIZED! (f, TOOL_BAR_REDISPLAYED! (f));
      }
    else
      {
        /* Any other change may leave the native size of F alone.  */
        adjust_frame_size (f, -1, -1, 3, false, Qtool_bar_lines ());
      }

    /* adjust_frame_size might not have done anything, garbage frame
       here.  */
    adjust_frame_glyphs (f);
    SET_FRAME_GARBAGED! (f);
    if FRAME_X_WINDOW! (f) != 0
      { x_clear_under_internal_border (f); }
  }
}

/* C 1966-1994.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_child_frame_border_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let border: c_int;

  if NILP (arg)
    { border = -1; }
  else if RANGED_FIXNUMP (0, arg, INT_MAX as EMACS_INT)
    { border = XFIXNAT (arg) as c_int; }
  else
    { signal_error (c"Invalid child frame border width".as_ptr (), arg); }

  if border != FRAME_CHILD_FRAME_BORDER_WIDTH (f)
    {
      (*f).child_frame_border_width = border;

      #[cfg (feature = "use_x_toolkit")]
      {
        if !(*(FRAME_X_OUTPUT! (f) as *mut x_output)).edit_widget.is_null ()
          { widget_store_internal_border ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).edit_widget); }
      }

      if FRAME_X_WINDOW! (f) != 0
        {
          adjust_frame_size (f, -1, -1, 3, false, Qchild_frame_border_width ());
          x_clear_under_internal_border (f);
        }
    }
}

/* C 1996-2017.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_internal_border_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let border: c_int = check_int_nonnegative (arg);

  if border != FRAME_INTERNAL_BORDER_WIDTH (f)
    {
      (*f).internal_border_width = border;

      #[cfg (feature = "use_x_toolkit")]
      {
        if !(*(FRAME_X_OUTPUT! (f) as *mut x_output)).edit_widget.is_null ()
          { widget_store_internal_border ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).edit_widget); }
      }

      if FRAME_X_WINDOW! (f) != 0
        {
          adjust_frame_size (f, -1, -1, 3, false, Qinternal_border_width ());
          x_clear_under_internal_border (f);
        }
    }
}

/* C 2020-2076: the comment C 2020-2023 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_scroll_bar_foreground (f: *mut frame, value: Lisp_Object,
                                       oldval: Lisp_Object)
{
  let mut pixel: c_ulong;
  #[cfg (feature = "have_gtk3")]
  let mut color: XColor = std::mem::zeroed ();
  #[cfg (feature = "have_gtk3")]
  let mut css: [c_char; 64] = [0; 64];

  if STRINGP (value)
    { pixel = x_decode_color (f, value, BLACK_PIX_DEFAULT! (f) as c_int); }
  else
    { pixel = (-1 as c_int) as c_ulong; }                 /* C: `unsigned long pixel = -1;` */

  if (*((*f).output_data.x as *mut x_output)).scroll_bar_foreground_pixel != (-1 as c_int) as c_ulong
    { unload_color (f, (*((*f).output_data.x as *mut x_output)).scroll_bar_foreground_pixel); }

  (*((*f).output_data.x as *mut x_output)).scroll_bar_foreground_pixel = pixel;
  if FRAME_X_WINDOW! (f) != 0 && FRAME_VISIBLE_P (f)
    {
      /* Remove all scroll bars because they have wrong colors.  */
      if ((*FRAME_TERMINAL! (f)).condemn_scroll_bars_hook.is_some ()
          && FRAME_X_WINDOW! (f) != 0)
        {
          ((*FRAME_TERMINAL! (f)).condemn_scroll_bars_hook.unwrap_unchecked ()) (f as *mut c_void);
        }
      if ((*FRAME_TERMINAL! (f)).judge_scroll_bars_hook.is_some ())
        {
          ((*FRAME_TERMINAL! (f)).judge_scroll_bars_hook.unwrap_unchecked ()) (f as *mut c_void);
        }

      update_face_from_frame_parameter (f, Qscroll_bar_foreground (), value);
      redraw_frame (f);
    }

  #[cfg (feature = "have_gtk3")]
  {
    if !FRAME_TOOLTIP_P (f)
      {
        if pixel != (-1 as c_int) as c_ulong
          {
            color.pixel = pixel;

            XQueryColor (FRAME_X_DISPLAY! (f),
                         FRAME_X_COLORMAP! (f),
                         &raw mut color);

            sprintf (css.as_mut_ptr (),
                     c"scrollbar slider { background-color: #%02x%02x%02x; }".as_ptr (),
                     (color.red >> 8) as c_int, (color.green >> 8) as c_int,
                     (color.blue >> 8) as c_int);
            gtk_css_provider_load_from_data (
              (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scrollbar_foreground_css_provider,
              css.as_ptr (), -1, std::ptr::null_mut ());
          }
        else
          {
            gtk_css_provider_load_from_data (
              (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scrollbar_foreground_css_provider,
              c"".as_ptr (), -1, std::ptr::null_mut ());
          }
      }
  }
}

/* C 2079-2149: the comment C 2079-2082 and the function.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_scroll_bar_background (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let mut pixel: c_ulong;
  #[cfg (feature = "have_gtk3")]
  let mut color: XColor = std::mem::zeroed ();
  #[cfg (feature = "have_gtk3")]
  let mut css: [c_char; 64] = [0; 64];

  if STRINGP (value)
    { pixel = x_decode_color (f, value, WHITE_PIX_DEFAULT! (f) as c_int); }
  else
    { pixel = (-1 as c_int) as c_ulong; }                 /* C: `unsigned long pixel = -1;` */

  if (*((*f).output_data.x as *mut x_output)).scroll_bar_background_pixel != (-1 as c_int) as c_ulong
    {
      unload_color (f,
                    (*((*f).output_data.x as *mut x_output)).scroll_bar_background_pixel);
    }

  #[cfg (all (feature = "use_lucid", feature = "use_toolkit_scroll_bars"))]
  {
    /* Scrollbar shadow colors.  */
    if (*((*f).output_data.x as *mut x_output)).scroll_bar_top_shadow_pixel != -1
      {
        unload_color (f,
                      (*((*f).output_data.x as *mut x_output)).scroll_bar_top_shadow_pixel);
        (*((*f).output_data.x as *mut x_output)).scroll_bar_top_shadow_pixel = -1;
      }
    if (*((*f).output_data.x as *mut x_output)).scroll_bar_bottom_shadow_pixel != -1
      {
        unload_color (f,
                      (*((*f).output_data.x as *mut x_output)).scroll_bar_bottom_shadow_pixel);
        (*((*f).output_data.x as *mut x_output)).scroll_bar_bottom_shadow_pixel = -1;
      }
  }

  (*((*f).output_data.x as *mut x_output)).scroll_bar_background_pixel = pixel;
  if FRAME_X_WINDOW! (f) != 0 && FRAME_VISIBLE_P (f)
    {
      /* Remove all scroll bars because they have wrong colors.  */
      if ((*FRAME_TERMINAL! (f)).condemn_scroll_bars_hook.is_some ()
          && FRAME_X_WINDOW! (f) != 0)
        {
          ((*FRAME_TERMINAL! (f)).condemn_scroll_bars_hook.unwrap_unchecked ()) (f as *mut c_void);
        }
      if ((*FRAME_TERMINAL! (f)).judge_scroll_bars_hook.is_some ())
        {
          ((*FRAME_TERMINAL! (f)).judge_scroll_bars_hook.unwrap_unchecked ()) (f as *mut c_void);
        }

      update_face_from_frame_parameter (f, Qscroll_bar_background (), value);
      redraw_frame (f);
    }

  #[cfg (feature = "have_gtk3")]
  {
    if !FRAME_TOOLTIP_P (f)
      {
        if pixel != (-1 as c_int) as c_ulong
          {
            color.pixel = pixel;

            XQueryColor (FRAME_X_DISPLAY! (f),
                         FRAME_X_COLORMAP! (f),
                         &raw mut color);

            sprintf (css.as_mut_ptr (),
                     c"scrollbar trough { background-color: #%02x%02x%02x; }".as_ptr (),
                     (color.red >> 8) as c_int, (color.green >> 8) as c_int,
                     (color.blue >> 8) as c_int);
            gtk_css_provider_load_from_data (
              (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scrollbar_background_css_provider,
              css.as_ptr (), -1, std::ptr::null_mut ());
          }
        else
          {
            gtk_css_provider_load_from_data (
              (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scrollbar_background_css_provider,
              c"".as_ptr (), -1, std::ptr::null_mut ());
          }
      }
  }
}

/* =====================================================================================
   C 2150-3811: the slice that follows slice 2.  It opens at C 2150 (the blank after
   `x_set_scroll_bar_background`'s `}`) and closes at C 3811, the `}` of
   `xic_preedit_draw_callback` - the first whole function past the 1,500 C lines this
   slice's brief asked for.  The boundary after it is C 3813's `void` / C 3814
   `xic_set_xfontset`, which is where the next slice begins.

   Everything here carries the file gate `have_x_windows` (P-86 ⑧), and every nested
   `#if` carries C's own macro lower-cased, both arms where C has two.  Names the
   mirrors this unit's include graph does not carry are declared at their use site with
   C's own header line (P-88 ①); `docs/ports/xfns.md` names each as an owed mirror row.
   ===================================================================================== */

/* C 2150-2195: the blank at 2150, the form feed at 2151, C 2152-2165's comment, and
   `x_encode_text`.  `setup_coding_system` (called at C 2177) is `coding.h` 686's
   prototype, which `include/coding.rs` does not carry, so it is declared here (P-88 ①).  */
/// C: coding.h 686: `extern void setup_coding_system (Lisp_Object, struct coding_system *);`
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {

}

/* Encode Lisp string STRING as a text in a format appropriate for
   the ICCCM (Inter Client Communication Conventions Manual).

   If STRING contains only ASCII characters, do no conversion and
   return the string data of STRING.  Otherwise, encode the text by
   CODING_SYSTEM, and return a newly allocated memory area which
   should be freed by `xfree' by a caller.

   Store the byte length of resulting text in *TEXT_BYTES.

   If the text contains only ASCII and Latin-1, store true in *STRING_P,
   which means that the `encoding' of the result can be `STRING'.
   Otherwise store false in *STRINGP, which means that the `encoding' of
   the result should be `COMPOUND_TEXT'.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_encode_text (string: Lisp_Object, coding_system: Lisp_Object,
                         text_bytes: *mut ptrdiff_t, stringp: *mut bool,
                         freep: *mut bool) -> *mut c_uchar
{
  let result: c_int = string_xstring_p (string);
  /* C: `struct coding_system coding;` - `setup_coding_system` writes it through the
     by-address parameter, so the port zero-initialises it (PORTING.md 1.5).  */
  let mut coding: coding_system = std::mem::zeroed ();

  if result == 0
    {
      /* No multibyte character in OBJ.  We need not encode it.  */
      *text_bytes = SBYTES (string);
      *stringp = true;
      *freep = false;
      return SDATA (string);
    }

  setup_coding_system (coding_system, &raw mut coding);
  /* C's `coding.mode |= (CODING_MODE_SAFE_ENCODING | CODING_MODE_LAST_BLOCK);` - `mode` is
     the 5-bit member of the mirror's one storage word (its shifts/masks are in
     `include/coding.rs`).  */
  coding.bits |= ((CODING_MODE_SAFE_ENCODING | CODING_MODE_LAST_BLOCK) as u32)
                 << CODING_SYSTEM_MODE_SHIFT;
  /* We suppress producing escape sequences for composition.  */
  /* C's `coding.common_flags &= ~CODING_ANNOTATION_MASK;` - the same word's low members.  */
  coding.bits &= !(CODING_ANNOTATION_MASK << CODING_SYSTEM_COMMON_FLAGS_SHIFT);
  coding.destination = xnmalloc (SCHARS (string), 2) as *mut u8;
  coding.dst_bytes = SCHARS (string) * 2;
  encode_coding_object (&raw mut coding, string, 0, 0,
                        SCHARS (string), SBYTES (string), Qnil ());
  *text_bytes = coding.produced;
  *stringp = (result == 1 || !EQ (coding_system, Qcompound_text ()));
  *freep = true;
  return coding.destination;
}

/* C 2196-2294: the blank at 2196, C 2197-2199's comment and `x_set_name_internal`.
   `XA_STRING` (used at C 2214/2229) is `<X11/Xatom.h>` 39's atom, which
   `include/xlib.rs`'s `x11_constants` module does not carry (it has `XA_ATOM`,
   `XA_CARDINAL`, `XA_PRIMARY` and `XA_WINDOW`), so it stands here as a use-site copy
   (P-88 ①; `docs/ports/xfns.md` records the owed row).  */
/// C: X11/Xatom.h 39: `#define XA_STRING ((Atom) 31)`
#[cfg (feature = "have_x_windows")]
pub const XA_STRING: Atom = 31;

#[cfg (feature = "have_x_windows")]
unsafe fn x_set_name_internal (f: *mut frame, name: Lisp_Object)
{
  use crate::x11_constants::PropModeReplace;   // C 2224/2231's `PropModeReplace` (X.h 475)

  if FRAME_X_WINDOW! (f) != 0
    {
      block_input ();
      {
        let mut text: XTextProperty = std::mem::zeroed ();
        let mut icon: XTextProperty = std::mem::zeroed ();
        let mut bytes: ptrdiff_t = 0;
        let mut stringp: bool = false;
        let mut do_free_icon_value: bool = false;
        let mut do_free_text_value: bool = false;
        let mut coding_system: Lisp_Object = std::ptr::null_mut ();
        let mut encoded_name: Lisp_Object;
        let mut encoded_icon_name: Lisp_Object;

        /* As ENCODE_UTF_8 may cause GC and relocation of string data,
           we use it before x_encode_text that may return string data.  */
        encoded_name = ENCODE_UTF_8 (name);

        coding_system = Qcompound_text ();
        /* Note: Encoding strategy

           We encode NAME by compound-text and use "COMPOUND-TEXT" in
           text.encoding.  But, there are non-internationalized window
           managers which don't support that encoding.  So, if NAME
           contains only ASCII and 8859-1 characters, encode it by
           iso-latin-1, and use "STRING" in text.encoding hoping that
           such window managers at least analyze this format correctly,
           i.e. treat 8-bit bytes as 8859-1 characters.

           We may also be able to use "UTF8_STRING" in text.encoding
           in the future which can encode all Unicode characters.
           But, for the moment, there's no way to know that the
           current window manager supports it or not.

           Either way, we also set the _NET_WM_NAME and _NET_WM_ICON_NAME
           properties.  Per the EWMH specification, those two properties
           are always UTF8_STRING.  This matches what gtk_window_set_title()
           does in the USE_GTK case. */
        text.value = x_encode_text (name, coding_system, &raw mut bytes,
                                    &raw mut stringp, &raw mut do_free_text_value);
        text.encoding = (if stringp { XA_STRING }
                         else { (*FRAME_DISPLAY_INFO! (f)).Xatom_COMPOUND_TEXT });
        text.format = 8;
        text.nitems = bytes as c_ulong;

        if !STRINGP ((*f).icon_name)
          {
            icon = text;
            encoded_icon_name = encoded_name;
          }
        else
          {
            /* See the above comment "Note: Encoding strategy".  */
            icon.value = x_encode_text ((*f).icon_name, coding_system, &raw mut bytes,
                                        &raw mut stringp, &raw mut do_free_icon_value);
            icon.encoding = (if stringp { XA_STRING }
                             else { (*FRAME_DISPLAY_INFO! (f)).Xatom_COMPOUND_TEXT });
            icon.format = 8;
            icon.nitems = bytes as c_ulong;

            encoded_icon_name = ENCODE_UTF_8 ((*f).icon_name);
          }

        #[cfg (feature = "use_gtk")]
        {
          gtk_window_set_title (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
                                SSDATA (encoded_name));
        }
        #[cfg (not (feature = "use_gtk"))]
        {
          XSetWMName (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f), &raw mut text);
          XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),
                           (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_name,
                           (*FRAME_DISPLAY_INFO! (f)).Xatom_UTF8_STRING,
                           8, PropModeReplace,
                           SDATA (encoded_name),
                           SBYTES (encoded_name) as c_int);
        }

        XSetWMIconName (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f), &raw mut icon);
        XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),
                         (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_icon_name,
                         (*FRAME_DISPLAY_INFO! (f)).Xatom_UTF8_STRING,
                         8, PropModeReplace,
                         SDATA (encoded_icon_name),
                         SBYTES (encoded_icon_name) as c_int);

        if do_free_icon_value
          { xfree (icon.value as *mut c_void); }
        if do_free_text_value
          { xfree (text.value as *mut c_void); }
      }
      unblock_input ();
    }
}

/* C 2296-2349: C 2296-2306's comment and `x_set_name`.  `explicit_name` (C 2316/2320) is
   frame.h 417's flag, which `include/frame.rs` records as a comment only, so the two accesses
   carry C's own name in a use-site macro over the mirror's flag word (the `src/androidfns.rs`
   209-214 shape; `docs/ports/xfns.md` records the mirror row).  */
/// C: frame.h 417: `bool_bf explicit_name : 1;`
#[cfg (feature = "have_x_windows")]
macro_rules! explicit_name {
  ($f:expr) => { frame_flag ($f, 1, FRAME_EXPLICIT_NAME_SHIFT) };
  ($f:expr, $v:expr) => { frame_set_flag ($f, 1, FRAME_EXPLICIT_NAME_SHIFT, $v as u32) };
}

/* Change the name of frame F to NAME.  If NAME is nil, set F's name to
       x_id_name.

   If EXPLICIT is true, that indicates that lisp code is setting the
       name; if NAME is a string, set F's name to NAME and set
       F->explicit_name; if NAME is Qnil, then clear F->explicit_name.

   If EXPLICIT is false, that indicates that Emacs redisplay code is
       suggesting a new name, which lisp code should override; if
       F->explicit_name is set, ignore the new name; otherwise, set it.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_set_name (f: *mut frame, mut name: Lisp_Object, explicit: bool)
{
  /* Make sure that requests from lisp code override requests from
     Emacs redisplay code.  */
  if explicit
    {
      /* If we're switching from explicit to implicit, we had better
	 update the mode lines and thereby update the title.  */
      if explicit_name! (f) && NILP (name)
	{ update_mode_lines = 37; }

      explicit_name! (f, ! NILP (name));
    }
  else if explicit_name! (f)
    { return; }

  /* If NAME is nil, set the name to the x_id_name.  */
  if NILP (name)
    {
      /* Check for no change needed in this very common case
	 before we do any consing.  */
      if strcmp ((*FRAME_DISPLAY_INFO! (f)).x_id_name,
                 SSDATA ((*f).name)) == 0
	{ return; }
      name = build_string ((*FRAME_DISPLAY_INFO! (f)).x_id_name);
    }
  else
    { CHECK_STRING (name); }

  /* Don't change the name if it's already NAME.  */
  if ! NILP (Fstring_equal (name, (*f).name))
    { return; }

  fset_name (f, name);

  /* For setting the frame title, the title parameter should override
     the name parameter.  */
  if ! NILP ((*f).title)
    { name = (*f).title; }

  x_set_name_internal (f, name);
}

/* This function should be called when the user's lisp code has
   specified a name for the frame; the name will override any set by the
   redisplay code.  */

/* C 2354-2358.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_explicitly_set_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  x_set_name (f, arg, true);
}

/* This function should be called by Emacs redisplay code to set the
   name; names set this way will never override names set by the user's
   lisp code.  */

/* C 2363-2367.  C's definition has no `static`, so the port keeps it exported (PORTING.md 0.2).  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_implicitly_set_name (f: *mut frame, arg: Lisp_Object,
                                                oldval: Lisp_Object)
{
  x_set_name (f, arg, false);
}

/* Change the title of frame F to NAME.
   If NAME is nil, use the frame name as the title.  */

/* C 2372-2389.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_title (f: *mut frame, mut name: Lisp_Object, old_name: Lisp_Object)
{
  /* Don't change the title if it's already NAME.  */
  if EQ (name, (*f).title)
    { return; }

  update_mode_lines = 38;

  fset_title (f, name);

  if NILP (name)
    { name = (*f).name; }
  else
    { CHECK_STRING (name); }

  x_set_name_internal (f, name);
}

/* C 2391-2410.  `FRAME_CONFIG_SCROLL_BAR_*` are frame.h 1291-1301's four macros; the mirror
   `include/frame.rs` carries each as a *getter function*, so a write needs C's own macro form
   (the `src/androidfns.rs` 338-352 shape; the same four names, same gates).  */
/// C: frame.h 1293: `#define FRAME_CONFIG_SCROLL_BAR_WIDTH(f) ((f)->config_scroll_bar_width)`
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_WIDTH {
  ($f:expr) => { (*$f).config_scroll_bar_width };
}
/// C: frame.h 1296: `#define FRAME_CONFIG_SCROLL_BAR_COLS(f) ((f)->config_scroll_bar_cols)`
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr) => { (*$f).config_scroll_bar_cols };
}
/// C: frame.h 1299: `#define FRAME_CONFIG_SCROLL_BAR_HEIGHT(f) ((f)->config_scroll_bar_height)`
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_HEIGHT {
  ($f:expr) => { (*$f).config_scroll_bar_height };
}
/// C: frame.h 1301: `#define FRAME_CONFIG_SCROLL_BAR_LINES(f) ((f)->config_scroll_bar_lines)`
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_LINES {
  ($f:expr) => { (*$f).config_scroll_bar_lines };
}

/* `xg_get_default_scrollbar_width`/`_height` are `gtkutil.h`'s, which `include/gtkutil.rs`
   does not carry (`USE_GTK` is `undef` in the frozen configuration, so the header's body is
   dead there); the two are declared here under C's own gate, as this unit's other GTK-side
   names are (P-88 ①).  */
/// C: gtkutil.h 206: `extern int xg_get_default_scrollbar_width (struct frame *f);`
/// C: gtkutil.h 207: `extern int xg_get_default_scrollbar_height (struct frame *f);`
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn xg_get_default_scrollbar_width (f: *mut frame) -> c_int;
  pub fn xg_get_default_scrollbar_height (f: *mut frame) -> c_int;
}

/* C 2391-2410.  C's definition has no `static`, so the port keeps it exported (PORTING.md 0.2).  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_set_scroll_bar_default_width (f: *mut frame)
{
  let unit: c_int = FRAME_COLUMN_WIDTH (f);
  #[cfg (feature = "use_toolkit_scroll_bars")]
  {
    #[cfg (feature = "use_gtk")]
    let mut minw: c_int = xg_get_default_scrollbar_width (f);
    #[cfg (not (feature = "use_gtk"))]
    let mut minw: c_int = 16;
    /* A minimum width of 14 doesn't look good for toolkit scroll bars.  */
    FRAME_CONFIG_SCROLL_BAR_COLS! (f) = (minw + unit - 1) / unit;
    FRAME_CONFIG_SCROLL_BAR_WIDTH! (f) = minw;
  }
  #[cfg (not (feature = "use_toolkit_scroll_bars"))]
  {
    /* The width of a non-toolkit scrollbar is 14 pixels.  */
    FRAME_CONFIG_SCROLL_BAR_COLS! (f) = (14 + unit - 1) / unit;
    FRAME_CONFIG_SCROLL_BAR_WIDTH! (f)
      = FRAME_CONFIG_SCROLL_BAR_COLS! (f) * unit;
  }
}

/* C 2412-2433.  C's definition has no `static`, so the port keeps it exported (PORTING.md 0.2).  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_set_scroll_bar_default_height (f: *mut frame)
{
  let height: c_int = FRAME_LINE_HEIGHT (f);
  #[cfg (feature = "use_toolkit_scroll_bars")]
  {
    #[cfg (feature = "use_gtk")]
    let mut min_height: c_int = xg_get_default_scrollbar_height (f);
    #[cfg (not (feature = "use_gtk"))]
    let mut min_height: c_int = 16;
    /* A minimum height of 14 doesn't look good for toolkit scroll bars.  */
    FRAME_CONFIG_SCROLL_BAR_HEIGHT! (f) = min_height;
    FRAME_CONFIG_SCROLL_BAR_LINES! (f) = (min_height + height - 1) / height;
  }
  #[cfg (not (feature = "use_toolkit_scroll_bars"))]
  {
    /* The height of a non-toolkit scrollbar is 14 pixels.  */
    FRAME_CONFIG_SCROLL_BAR_LINES! (f) = (14 + height - 1) / height;

    /* Use all of that space (aside from required margins) for the
       scroll bar.  */
    FRAME_CONFIG_SCROLL_BAR_HEIGHT! (f) = 14;
  }
}

/* C 2435-2490.  */
#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_alpha (f: *mut frame, mut arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut alpha: c_double = 1.0;
  let mut newval: [c_double; 2] = [0.0; 2];
  let mut i: c_int;
  let mut item: Lisp_Object;
  let mut alpha_identical_p: bool;

  alpha_identical_p = true;

  /* C: `for (i = 0; i < 2; i++)`.  */
  i = 0;
  while i < 2
    {
      newval[i as usize] = 1.0;
      if CONSP (arg)
        {
          item = CAR (arg);
          arg  = CDR (arg);

	  alpha_identical_p = false;
        }
      else
        { item = arg; }

      if NILP (item)
	{ alpha = - 1.0; }
      else if FLOATP (item)
	{
	  alpha = XFLOAT_DATA (item);
	  if ! (0.0 <= alpha && alpha <= 1.0)
	    { args_out_of_range (make_float (0.0), make_float (1.0)); }
	}
      else if FIXNUMP (item)
	{
	  let ialpha: EMACS_INT = XFIXNUM (item);
	  if ! (0 <= ialpha && ialpha <= 100)
	    { args_out_of_range (make_fixnum (0), make_fixnum (100)); }
	  alpha = ialpha as c_double / 100.0;
	}
      else
	{ wrong_type_argument (Qnumberp (), item); }
      newval[i as usize] = alpha;
      i += 1;
    }

  /* C: `for (i = 0; i < 2; i++) f->alpha[i] = newval[i];`.  */
  i = 0;
  while i < 2
    {
      (*f).alpha[i as usize] = newval[i as usize];
      i += 1;
    }

  /* C: `FRAME_X_OUTPUT (f)->alpha_identical_p = alpha_identical_p;` - the member is
     `include/xterm.rs`'s `// C:` row (a `bool_bf` it records as a comment), so this statement
     keeps C's own member name; `docs/ports/xfns.md` records the reading that follows.  */
  (*(FRAME_X_OUTPUT! (f) as *mut x_output)).alpha_identical_p = alpha_identical_p;

  if (*FRAME_TERMINAL! (f)).set_frame_alpha_hook.is_some ()
    {
      block_input ();
      ((*FRAME_TERMINAL! (f)).set_frame_alpha_hook.unwrap_unchecked ()) (f as *mut c_void);
      unblock_input ();
    }
}

/* C 2492-2531.  `x_wm_supports` is `xterm.h` 1897's prototype, which no mirror in this unit's
   graph carries, so it is declared here under C's own gate (P-88 ①).  */
/// C: xterm.h 1897: `extern bool x_wm_supports (struct frame *, Atom);`
#[cfg (all (feature = "have_x_windows", feature = "have_xsync",
            not (feature = "use_gtk"), feature = "have_clock_gettime"))]
unsafe extern "C" {
  pub fn x_wm_supports (f: *mut frame, property: Atom) -> bool;
}

#[cfg (feature = "have_x_windows")]

unsafe extern "C" fn x_set_use_frame_synchronization (f: *mut frame, arg: Lisp_Object,
                                           oldval: Lisp_Object)
{
  use crate::x11_constants::{PropModeReplace, XA_CARDINAL};   // C 2510/2512 (X.h 477, Xatom.h 14)

  #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
              feature = "have_clock_gettime"))]
  {
    let mut dpyinfo: *mut x_display_info;
    let mut bypass_compositor: c_ulong;

    dpyinfo = FRAME_DISPLAY_INFO! (f);

    if !NILP (arg) && FRAME_X_EXTENDED_COUNTER! (f) != 0
      {
        (*(FRAME_X_OUTPUT! (f) as *mut x_output)).use_vsync_p
          = x_wm_supports (f, (*dpyinfo).Xatom_net_wm_frame_drawn);

        /* At the same time, write the bypass compositor property to the
           outer window.  2 means to never bypass the compositor, as we
           need its cooperation for frame synchronization.  */
        bypass_compositor = 2;
        XChangeProperty ((*dpyinfo).display, FRAME_OUTER_WINDOW! (f),
                         (*dpyinfo).Xatom_net_wm_bypass_compositor,
                         XA_CARDINAL, 32, PropModeReplace,
                         &raw const bypass_compositor as *const c_uchar, 1);
      }
    else
      {
        (*(FRAME_X_OUTPUT! (f) as *mut x_output)).use_vsync_p = false;

        /* Remove the compositor bypass property from the outer
           window.  */
        XDeleteProperty ((*dpyinfo).display, FRAME_OUTER_WINDOW! (f),
                         (*dpyinfo).Xatom_net_wm_bypass_compositor);
      }

    store_frame_param (f, Quse_frame_synchronization (),
                       if (*(FRAME_X_OUTPUT! (f) as *mut x_output)).use_vsync_p
                         { Qt () } else { Qnil () });
  }
  #[cfg (not (all (feature = "have_xsync", not (feature = "use_gtk"),
                   feature = "have_clock_gettime")))]
  {
    store_frame_param (f, Quse_frame_synchronization (), Qnil ());
  }
}

/* C 2533-2584: C 2533-2538's comment and `x_default_scroll_bar_color_parameter`.  The three
   frame.h/dispextern.h prototypes it calls - `gui_display_get_arg`, `gui_set_frame_parameters`
   and `gui_display_get_resource` - are not in any mirror of this unit's include graph, so they
   are declared here with C's own lines (P-88 ①; `src/androidfns.rs` 122-128 is the same
   transcript for the first), and `enum resource_types` with them.  */
/// C: dispextern.h 3994: `extern Lisp_Object gui_display_get_arg (Display_Info *, Lisp_Object,`
/// C: `Lisp_Object, const char *, const char *, enum resource_types);`
/// (C's `Display_Info` is `frame.h`'s alias of the X arm's `struct x_display_info`.)
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn gui_display_get_arg (dpyinfo: *mut x_display_info, alist: Lisp_Object,
                              param: Lisp_Object, name: *const c_char, x_name: *const c_char,
                              r#type: resource_types) -> Lisp_Object;

  /// C: frame.h 1875: `extern Lisp_Object gui_display_get_resource (Display_Info *,
  /// Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object);`
  pub fn gui_display_get_resource (dpyinfo: *mut x_display_info, attribute: Lisp_Object,
                                   class: Lisp_Object, component: Lisp_Object,
                                   subclass: Lisp_Object) -> Lisp_Object;
}

/* C: dispextern.h 3983: `enum resource_types { RES_TYPE_NUMBER, RES_TYPE_FLOAT,
   RES_TYPE_BOOLEAN, RES_TYPE_STRING, RES_TYPE_SYMBOL, RES_TYPE_BOOLEAN_NUMBER };` - a
   WIP-slice gap of `include/dispextern.rs` (`src/androidfns.rs` 164-178 carries the same
   copy).  C's enum ABI is `c_int` (PORTING.md 1.3).  */
#[cfg (feature = "have_x_windows")]
pub type resource_types = c_int;
#[cfg (feature = "have_x_windows")]
pub const RES_TYPE_NUMBER: resource_types = 0;          // C 3985
#[cfg (feature = "have_x_windows")]
pub const RES_TYPE_FLOAT: resource_types = 1;           // C 3986
#[cfg (feature = "have_x_windows")]
pub const RES_TYPE_BOOLEAN: resource_types = 2;         // C 3987
#[cfg (feature = "have_x_windows")]
pub const RES_TYPE_STRING: resource_types = 3;          // C 3988
#[cfg (feature = "have_x_windows")]
pub const RES_TYPE_SYMBOL: resource_types = 4;          // C 3989
#[cfg (feature = "have_x_windows")]
pub const RES_TYPE_BOOLEAN_NUMBER: resource_types = 5;  // C 3990

/* Record in frame F the specified or default value according to ALIST
   of the parameter named PROP (a Lisp symbol).  If no value is
   specified for PROP, look for an X default for XPROP on the frame
   named NAME.  If that is not found either, use the value DEFLT.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_default_scroll_bar_color_parameter (f: *mut frame, alist: Lisp_Object,
                                                prop: Lisp_Object, xprop: *const c_char,
                                                xclass: *const c_char, foreground_p: bool)
                                                -> Lisp_Object
{
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let mut tem: Lisp_Object;

  tem = gui_display_get_arg (dpyinfo, alist, prop, xprop, xclass,
                             RES_TYPE_STRING);
  if BASE_EQ (tem, Qunbound ())
    {
      #[cfg (feature = "use_toolkit_scroll_bars")]
      {
        /* See if an X resource for the scroll bar color has been
	   specified.  */
        AUTO_STRING! (foreground, c"foreground".as_ptr ());
        AUTO_STRING! (background, c"foreground".as_ptr ());
        AUTO_STRING! (verticalScrollBar, c"verticalScrollBar".as_ptr ());
        tem = gui_display_get_resource (
                dpyinfo, if foreground_p { foreground } else { background },
                empty_unibyte_string,
                verticalScrollBar,
                empty_unibyte_string);
        if !STRINGP (tem)
          {
            /* If nothing has been specified, scroll bars will use a
	       toolkit-dependent default.  Because these defaults are
	       difficult to get at without actually creating a scroll
	       bar, use nil to indicate that no color has been
	       specified.  */
            tem = Qnil ();
          }
      }
      #[cfg (not (feature = "use_toolkit_scroll_bars"))]
      {
        tem = Qnil ();
      }
    }

  AUTO_FRAME_ARG! (arg, prop, tem);
  gui_set_frame_parameters (f, arg);
  return tem;
}

/* C 2585-2654: C 2585-2588's comment, the `#ifdef USE_X_TOOLKIT` at C 2589, `hack_wm_protocols`
   and the `#endif` at C 2655.  */

/* If the WM_PROTOCOLS property does not already contain WM_TAKE_FOCUS,
   WM_DELETE_WINDOW, and WM_SAVE_YOURSELF, then add them.  (They may
   already be present because of the toolkit (Motif adds some of them,
   for example, but Xt doesn't).  */

#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe fn hack_wm_protocols (f: *mut frame, widget: Widget)
{
  use crate::x11_constants::{PropModeAppend, Success, XA_ATOM};   // C 2614/2620/2650 (X.h/Xatom.h)

  let dpy: *mut Display = XtDisplay (widget);
  let w: Window = XtWindow (widget);
  let mut need_delete: bool = true;
  let mut need_focus: bool = true;
  let mut need_save: bool = true;

  block_input ();
  {
    let mut r#type: Atom = 0;
    let mut catoms: *mut c_uchar = std::ptr::null_mut ();
    let mut format: c_int = 0;
    let mut nitems: c_ulong = 0;
    let mut bytes_after: c_ulong = 0;

    if (XGetWindowProperty (dpy, w,
			     (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_protocols,
			     0, 100, False, XA_ATOM,
			     &raw mut r#type, &raw mut format, &raw mut nitems, &raw mut bytes_after,
			     &raw mut catoms)
	 == Success)
	&& format == 32 && r#type == XA_ATOM
      {
	let atoms: *mut Atom = catoms as *mut Atom;
	while nitems > 0
	  {
	    nitems -= 1;
	    if *atoms.offset (nitems as isize)
		== (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_delete_window
	      { need_delete = false; }
	    else if *atoms.offset (nitems as isize)
		     == (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_take_focus
	      { need_focus = false; }
	    else if *atoms.offset (nitems as isize)
		     == (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_save_yourself
	      { need_save = false; }
	  }
      }
    if !catoms.is_null ()
      { XFree (catoms as *mut c_void); }
  }
  {
    let mut props: [Atom; 10] = [0; 10];
    let mut count: c_int = 0;
    /* C's `props[count++] = …`: the store first, the index step second.  */
    if need_delete
      {
        props[count as usize] = (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_delete_window;
        count += 1;
      }
    if need_focus
      {
        props[count as usize] = (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_take_focus;
        count += 1;
      }
    if need_save
      {
        props[count as usize] = (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_save_yourself;
        count += 1;
      }
    if count != 0
      {
        XChangeProperty (dpy, w, (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_protocols,
			 XA_ATOM, 32, PropModeAppend,
			 props.as_ptr () as *const c_uchar, count);
      }
  }
  unblock_input ();
}

/* C 2657-2722.  */
#[cfg (feature = "have_x_windows")]
unsafe fn append_wm_protocols (dpyinfo: *mut x_display_info, f: *mut frame)
{
  use crate::x11_constants::{PropModeAppend, Success, XA_ATOM};   // C 2678/2680/2719 (X.h/Xatom.h)

  let mut existing: *mut c_uchar = std::ptr::null_mut ();
  let mut format: c_int = 0;
  let mut nitems: c_ulong = 0;
  let mut r#type: Atom = 0;
  let mut existing_protocols: *mut Atom = std::ptr::null_mut ();
  let mut protos: [Atom; 10] = [0; 10];
  let mut num_protos: c_int = 0;
  let mut found_wm_ping: bool = false;
  #[cfg (all (not (feature = "have_gtk3"), feature = "have_xsync"))]
  let mut found_wm_sync_request: bool = false;
  let mut bytes_after: c_ulong = 0;

  block_input ();
  if (XGetWindowProperty ((*dpyinfo).display, FRAME_OUTER_WINDOW! (f),
			   (*dpyinfo).Xatom_wm_protocols,
			   0, 100, False, XA_ATOM, &raw mut r#type, &raw mut format,
			   &raw mut nitems, &raw mut bytes_after,
			   &raw mut existing) == Success)
      && format == 32 && r#type == XA_ATOM
    {
      existing_protocols = existing as *mut Atom;

      while nitems != 0
	{
	  nitems -= 1;

	  if *existing_protocols.offset (nitems as isize)
	      == (*dpyinfo).Xatom_net_wm_ping
	    { found_wm_ping = true; }
	  else
	    {
	      /* C's `#if !defined HAVE_GTK3 && defined HAVE_XSYNC` around the `else if`
		 branch: the gated arm's test is the else's body, so the chain is the
		 same one C compiles.  */
	      #[cfg (all (not (feature = "have_gtk3"), feature = "have_xsync"))]
	      if *existing_protocols.offset (nitems as isize)
		  == (*dpyinfo).Xatom_net_wm_sync_request
		{ found_wm_sync_request = true; }
	    }
	}
    }

  if !existing.is_null ()
    { XFree (existing as *mut c_void); }

  if !(*dpyinfo).untrusted
    {
      /* Untrusted clients cannot use these protocols which require
	 communicating with the window manager.  */

      /* C's `protos[num_protos++] = …`: the store first, the index step second.  */
      if !found_wm_ping
        {
          protos[num_protos as usize] = (*dpyinfo).Xatom_net_wm_ping;
          num_protos += 1;
        }
      #[cfg (all (not (feature = "have_gtk3"), feature = "have_xsync"))]
      {
        if !found_wm_sync_request && (*dpyinfo).xsync_supported_p
          {
            protos[num_protos as usize] = (*dpyinfo).Xatom_net_wm_sync_request;
            num_protos += 1;
          }
      }
    }

  if num_protos != 0
    {
      XChangeProperty ((*dpyinfo).display,
		       FRAME_OUTER_WINDOW! (f),
		       (*dpyinfo).Xatom_wm_protocols,
		       XA_ATOM, 32, PropModeAppend,
		       protos.as_ptr () as *const c_uchar,
		       num_protos);
    }
  unblock_input ();
}

/* =====================================================================================
   C 2724-3956: the XIC (X Input Context) block.

   C 2724-2726 is its comment ("Support routines for XIC (X Input Context).") and C 2728 the
   `#ifdef HAVE_X_I18N` that holds everything through C 3956.  C 2729-2736 are five `static`
   forward declarations, which PORTING.md 0.1-12 drops (Rust has no prototypes; every definition
   they name is an item of this file; the last of them, `xic_string_conversion_callback`
   (C 3838), is this slice's last function).

   The XIM/XIC surface: `<X11/Xlib.h>` 1090-1346's types, the `XIMCaretDirection` enumerators,
   the nine preedit/status masks of Xlib.h 1169-1177, the sixteen `XN*` attribute-name strings
   of Xlib.h 1179-1215, and the eight entry points below.  No mirror of this unit's include graph
   declares any of them (`include/xterm.rs` *names* XIM/XIC/XFontSet/XIMStyles/XIMStyle in its
   `x_display_info`/`x_output` rows and its `STYLE_*` constants use the style masks, but it
   declares neither the types nor the macros), so each stands here at the use site under C's own
   gate with the header line it comes from, measured against the installed headers (libx11 1.8.13
   and xorgproto 2025.1 under `/opt/homebrew/include`); `docs/ports/xfns.md` names the
   `include/xterm.rs`/`include/xlib.rs` rows they are owed.  */

/// C: Xlib.h 1090: `typedef struct _XOC *XOC, *XFontSet;`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XFontSet = *mut c_void;
/// C: Xlib.h 1141: `typedef struct _XIM *XIM;`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIM = *mut c_void;
/// C: Xlib.h 1142: `typedef struct _XIC *XIC;`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIC = *mut c_void;
/// C: Xlib.h 1162: `typedef unsigned long XIMStyle;`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIMStyle = c_ulong;
/// C: Xlib.h 1164-1167: `typedef struct { unsigned short count_styles; XIMStyle
/// *supported_styles; } XIMStyles;` - measured `sizeof 16 align 8`, offsets `count_styles 0
/// supported_styles 8`.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIMStyles
{
  pub count_styles: c_ushort,             // C: unsigned short count_styles;
  pub supported_styles: *mut XIMStyle,    // C: XIMStyle *supported_styles;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::size_of::<XIMStyles> () == 16);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMStyles, supported_styles) == 8);

/// C: Xlib.h 1144-1148: `typedef void (*XIMProc) (XIM, XPointer, XPointer);`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIMProc = Option<unsafe extern "C" fn (XIM, XPointer, XPointer)>;
/// C: Xlib.h 1150-1154: `typedef Bool (*XICProc) (XIC, XPointer, XPointer);`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XICProc = Option<unsafe extern "C" fn (XIC, XPointer, XPointer) -> Bool>;
/// C: Xlib.h 1229: `typedef void *XVaNestedList;`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XVaNestedList = *mut c_void;
/// C: Xlib.h 1231-1234: `typedef struct { XPointer client_data; XIMProc callback; }
/// XIMCallback;` - measured `sizeof 16 align 8`, offsets `client_data 0 callback 8`.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIMCallback
{
  pub client_data: XPointer,              // C: XPointer client_data;
  pub callback: XIMProc,                  // C: XIMProc callback;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::size_of::<XIMCallback> () == 16);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMCallback, callback) == 8);
/// C: Xlib.h 1236-1239: `typedef struct { XPointer client_data; XICProc callback; }
/// XICCallback;` - the same two words, with `XICProc`'s `Bool` result.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XICCallback
{
  pub client_data: XPointer,              // C: XPointer client_data;
  pub callback: XICProc,                  // C: XICProc callback;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::size_of::<XICCallback> () == 16);

/// C: Xlib.h 1235: `typedef unsigned long XIMFeedback;`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIMFeedback = c_ulong;
/// C: Xlib.h 1253-1261's inner union, `union { char *multi_byte; wchar_t *wide_char; } string;`
/// - anonymous in C, so the port names it after its owner and member the way
/// `lisp.rs`'s `Lisp_String_s`/`Lisp_String_u` are named (PORTING.md 1.3).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub union XIMText_string
{
  pub multi_byte: *mut c_char,            // C: char *multi_byte;
  pub wide_char: *mut c_int,              // C: wchar_t *wide_char; (`wchar_t` is `int` here)
}
/// C: Xlib.h 1253-1261: `typedef struct _XIMText { unsigned short length; XIMFeedback
/// *feedback; Bool encoding_is_wchar; union { char *multi_byte; wchar_t *wide_char; } string;
/// } XIMText;` - measured `sizeof 32 align 8`, offsets `length 0 feedback 8 encoding_is_wchar 16
/// string 24`.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIMText
{
  pub length: c_ushort,                   // C: unsigned short length;
  pub feedback: *mut XIMFeedback,         // C: XIMFeedback *feedback;
  pub encoding_is_wchar: Bool,            // C: Bool encoding_is_wchar;
  pub string: XIMText_string,             // C: union { … } string;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::size_of::<XIMText> () == 32);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMText, feedback) == 8);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMText, encoding_is_wchar) == 16);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMText, string) == 24);
/// C: Xlib.h 1278: `typedef unsigned long XIMStringConversionFeedback;`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIMStringConversionFeedback = c_ulong;
/// C: Xlib.h 1287-1295's inner union, `union { char *mbs; wchar_t *wcs; } string;` - anonymous in
/// C, so the port names it after its owner the way `lisp.rs`'s `Lisp_String_s`/`Lisp_String_u`
/// are named (PORTING.md 1.3).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub union XIMStringConversionText_string
{
  pub mbs: *mut c_char,                   // C: char *mbs;
  pub wcs: *mut c_int,                    // C: wchar_t *wcs; (`wchar_t` is `int` here)
}
/// C: Xlib.h 1287-1295: `typedef struct _XIMStringConversionText { unsigned short length;
/// XIMStringConversionFeedback *feedback; Bool encoding_is_wchar; union { char *mbs; wchar_t *wcs;
/// } string; } XIMStringConversionText;` - measured `sizeof 32 align 8`, offsets `length 0 feedback
/// 8 encoding_is_wchar 16 string 24`.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIMStringConversionText
{
  pub length: c_ushort,                   // C: unsigned short length;
  pub feedback: *mut XIMStringConversionFeedback,  // C: XIMStringConversionFeedback *feedback;
  pub encoding_is_wchar: Bool,            // C: Bool encoding_is_wchar;
  pub string: XIMStringConversionText_string,      // C: union { … } string;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::size_of::<XIMStringConversionText> () == 32);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMStringConversionText, feedback) == 8);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMStringConversionText, encoding_is_wchar) == 16);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMStringConversionText, string) == 24);
/// C: Xlib.h 1297: `typedef unsigned short XIMStringConversionPosition;`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIMStringConversionPosition = c_ushort;
/// C: Xlib.h 1306: `typedef unsigned short XIMStringConversionOperation;`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIMStringConversionOperation = c_ushort;
/// C: Xlib.h 1308-1309: `#define XIMStringConversionSubstitution (0x0001)` and `#define
/// XIMStringConversionRetrieval (0x0002)` (measured `1` and `2`).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMStringConversionSubstitution: XIMStringConversionOperation = 0x0001;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMStringConversionRetrieval: XIMStringConversionOperation = 0x0002;

/// C: Xlib.h 1305-1319: `typedef enum { XIMForwardChar, XIMBackwardChar, XIMForwardWord,
/// XIMBackwardWord, XIMCaretUp, XIMCaretDown, XIMNextLine, XIMPreviousLine, XIMLineStart,
/// XIMLineEnd, XIMAbsolutePosition, XIMDontChange } XIMCaretDirection;` - C's members carry no
/// initialiser, so the values are the declaration order (measured with clang).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIMCaretDirection = c_int;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMForwardChar: XIMCaretDirection = 0;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMBackwardChar: XIMCaretDirection = 1;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMForwardWord: XIMCaretDirection = 2;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMBackwardWord: XIMCaretDirection = 3;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMCaretUp: XIMCaretDirection = 4;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMCaretDown: XIMCaretDirection = 5;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMNextLine: XIMCaretDirection = 6;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreviousLine: XIMCaretDirection = 7;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMLineStart: XIMCaretDirection = 8;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMLineEnd: XIMCaretDirection = 9;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMAbsolutePosition: XIMCaretDirection = 10;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMDontChange: XIMCaretDirection = 11;

/// C: Xlib.h 1321-1327: `typedef struct _XIMStringConversionCallbackStruct {
/// XIMStringConversionPosition position; XIMCaretDirection direction;
/// XIMStringConversionOperation operation; unsigned short factor;
/// XIMStringConversionText *text; } XIMStringConversionCallbackStruct;` - measured `sizeof 24
/// align 8`, offsets `position 0 direction 4 operation 8 factor 10 text 16`.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIMStringConversionCallbackStruct
{
  pub position: XIMStringConversionPosition,  // C: XIMStringConversionPosition position;
  pub direction: XIMCaretDirection,       // C: XIMCaretDirection direction;
  pub operation: XIMStringConversionOperation,  // C: XIMStringConversionOperation operation;
  pub factor: c_ushort,                   // C: unsigned short factor;
  pub text: *mut XIMStringConversionText, // C: XIMStringConversionText *text;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::size_of::<XIMStringConversionCallbackStruct> () == 24);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMStringConversionCallbackStruct, direction) == 4);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMStringConversionCallbackStruct, operation) == 8);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMStringConversionCallbackStruct, factor) == 10);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMStringConversionCallbackStruct, text) == 16);

/// C: Xlib.h 1336-1340: `typedef enum { XIMIsInvisible, XIMIsPrimary, XIMIsSecondary }
/// XIMCaretStyle;` (measured `0/1/2`).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIMCaretStyle = c_int;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMIsInvisible: XIMCaretStyle = 0;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMIsPrimary: XIMCaretStyle = 1;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMIsSecondary: XIMCaretStyle = 2;

/// C: Xlib.h 1329-1334: `typedef struct _XIMPreeditDrawCallbackStruct { int caret; int
/// chg_first; int chg_length; XIMText *text; } XIMPreeditDrawCallbackStruct;` - measured
/// `sizeof 24 align 8`, offsets `caret 0 chg_first 4 chg_length 8 text 16`.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIMPreeditDrawCallbackStruct
{
  pub caret: c_int,                       // C: int caret;
  pub chg_first: c_int,                   // C: int chg_first;
  pub chg_length: c_int,                  // C: int chg_length;
  pub text: *mut XIMText,                 // C: XIMText *text;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::size_of::<XIMPreeditDrawCallbackStruct> () == 24);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMPreeditDrawCallbackStruct, chg_first) == 4);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMPreeditDrawCallbackStruct, text) == 16);

/// C: Xlib.h 1342-1346: `typedef struct _XIMPreeditCaretCallbackStruct { int position;
/// XIMCaretDirection direction; XIMCaretStyle style; } XIMPreeditCaretCallbackStruct;` -
/// measured `sizeof 12 align 4`, offsets `position 0 direction 4 style 8`.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIMPreeditCaretCallbackStruct
{
  pub position: c_int,                    // C: int position;
  pub direction: XIMCaretDirection,       // C: XIMCaretDirection direction;
  pub style: XIMCaretStyle,               // C: XIMCaretStyle style;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::size_of::<XIMPreeditCaretCallbackStruct> () == 12);
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::offset_of! (XIMPreeditCaretCallbackStruct, direction) == 4);

/// C: Xlib.h 422-426: `typedef struct { short x, y; } XPoint;` - measured `sizeof 4 align 2`.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XPoint
{
  pub x: c_short,                         // C: short x;
  pub y: c_short,                         // C: short y;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
const _: () = core::assert! (core::mem::size_of::<XPoint> () == 4);

/* C: Xlib.h 1169-1177: the nine preedit/status masks (`#define XIMPreeditArea 0x0001L`,
   `XIMPreeditCallbacks 0x0002L`, `XIMPreeditPosition 0x0004L`, `XIMPreeditNothing 0x0008L`,
   `XIMPreeditNone 0x0010L`, `XIMStatusArea 0x0100L`, `XIMStatusCallbacks 0x0200L`,
   `XIMStatusNothing 0x0400L`, `XIMStatusNone 0x0800L`) - measured with clang, and the six
   `include/xterm.rs`'s `STYLE_*` constants (its own C 2003-2009) are already written with.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreeditArea: XIMStyle = 0x0001;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreeditCallbacks: XIMStyle = 0x0002;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreeditPosition: XIMStyle = 0x0004;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreeditNothing: XIMStyle = 0x0008;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreeditNone: XIMStyle = 0x0010;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMStatusArea: XIMStyle = 0x0100;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMStatusCallbacks: XIMStyle = 0x0200;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMStatusNothing: XIMStyle = 0x0400;
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMStatusNone: XIMStyle = 0x0800;

/* C: Xlib.h 1179-1215's `XN*` attribute-name strings - C's `#define XNFontSet "fontSet"` family.
   Each is the `src/xfns.rs` 1274-1283 `XtNinput`/`XtNborderColor` shape: a function returning
   the string's own bytes, because every use is a variadic argument that needs `char *`.  */
/// C: Xlib.h 1183: `#define XNClientWindow "clientWindow"`
/// C: Xlib.h 1184: `#define XNInputStyle "inputStyle"` / 1185 `XNFocusWindow "focusWindow"`
/// C: Xlib.h 1190: `#define XNPreeditStartCallback "preeditStartCallback"` / 1191
/// `XNPreeditDoneCallback "preeditDoneCallback"` / 1192 `XNPreeditDrawCallback
/// "preeditDrawCallback"` / 1193 `XNPreeditCaretCallback "preeditCaretCallback"`
/// C: Xlib.h 1195: `#define XNPreeditAttributes "preeditAttributes"` / 1200
/// `XNStatusAttributes "statusAttributes"` / 1201 `XNArea "area"` / 1202
/// `XNAreaNeeded "areaNeeded"` / 1203 `XNSpotLocation "spotLocation"` / 1206
/// `XNForeground "foreground"` / 1207 `XNBackground "background"` / 1209 `XNFontSet "fontSet"`
/// C: Xlib.h 1214: `#define XNStringConversionCallback "stringConversionCallback"`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNClientWindow () -> *mut c_char { c"clientWindow".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNInputStyle () -> *mut c_char { c"inputStyle".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNFocusWindow () -> *mut c_char { c"focusWindow".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNPreeditStartCallback () -> *mut c_char
{ c"preeditStartCallback".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNPreeditDoneCallback () -> *mut c_char
{ c"preeditDoneCallback".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNPreeditDrawCallback () -> *mut c_char
{ c"preeditDrawCallback".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNPreeditCaretCallback () -> *mut c_char
{ c"preeditCaretCallback".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNPreeditAttributes () -> *mut c_char
{ c"preeditAttributes".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNStatusAttributes () -> *mut c_char
{ c"statusAttributes".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNArea () -> *mut c_char { c"area".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNAreaNeeded () -> *mut c_char { c"areaNeeded".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNSpotLocation () -> *mut c_char { c"spotLocation".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNForeground () -> *mut c_char { c"foreground".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNBackground () -> *mut c_char { c"background".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNFontSet () -> *mut c_char { c"fontSet".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub fn XNStringConversionCallback () -> *mut c_char
{ c"stringConversionCallback".as_ptr () as *mut c_char }

/// C: Xlib.h 3619-3625: `extern XFontSet XCreateFontSet (Display *, _Xconst char *, char ***,
/// int *, char **);`
/// C: Xlib.h 1932-1934: `extern void XFreeStringList (char **);`
/// C: Xlib.h 3627-3630: `extern void XFreeFontSet (Display *, XFontSet);`
/// C: Xlib.h 3863-3865: `extern XIC XCreateIC (XIM, ...);`
/// C: Xlib.h 3867-3869: `extern void XDestroyIC (XIC);`
/// C: Xlib.h 3891-3893: `extern char *XSetICValues (XIC, ...);`
/// C: Xlib.h 3895-3897: `extern char *XGetICValues (XIC, ...);`
/// C: Xlib.h 3935-3937: `extern XVaNestedList XVaCreateNestedList (int, ...);`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" {
  pub fn XCreateFontSet (display: *mut Display, base_font_name_list: *const c_char,
                         missing_charset_list: *mut *mut *mut c_char,
                         missing_charset_count: *mut c_int,
                         def_string: *mut *mut c_char) -> XFontSet;
  pub fn XFreeStringList (list: *mut *mut c_char);
  pub fn XFreeFontSet (display: *mut Display, font_set: XFontSet);
  pub fn XCreateIC (im: XIM, ...) -> XIC;
  pub fn XDestroyIC (ic: XIC);
  pub fn XSetICValues (ic: XIC, ...) -> *mut c_char;
  pub fn XGetICValues (ic: XIC, ...) -> *mut c_char;
  pub fn XVaCreateNestedList (unused: c_int, ...) -> XVaNestedList;
}

/* The `use_gtk` arm of `xic_set_preeditarea` (C 3277-3295) calls this one; `include/gtk.rs`
   does not carry it, so it stands here under that gate (P-88 ①).  */
/// C: gtk/gtkimcontext.h: `void gtk_im_context_set_cursor_location (GtkIMContext *context,
/// const GdkRectangle *area);`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn gtk_im_context_set_cursor_location (context: *mut GtkIMContext,
                                             area: *const GdkRectangle);
}

/* C 2740-2776: the five callback records `create_frame_xic` hands to `XCreateIC` (the sixth at
   C 2778-2786, `Xxic_string_conversion_callback`, is in the list below).  C's `static XIMCallback
   X… = { NULL, (XIMProc) f };` is a file-scope object, so the port keeps `static mut` and C's
   cast.  **C's cast reinterprets the function pointer, and Rust refuses a signature-changing
   `as` cast between fn pointers (`error: non-primitive cast`, measured here), so the port
   spells each of the four casts that change a signature as the item address plus a pointer
   transmute - the `src/xwidget.rs` 213-222 `G_CALLBACK` shape, whose note records the same
   limitation; every callback C passes through it is declared `unsafe extern "C" fn`
   (PORTING.md 1.4).**  `docs/ports/xfns.md` records the deviation.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
static mut Xxic_preedit_draw_callback: XIMCallback =
  XIMCallback {
    client_data: std::ptr::null_mut (),
    callback: Some (unsafe { core::mem::transmute::<*const (),
                             unsafe extern "C" fn (XIM, XPointer, XPointer)> (
                               xic_preedit_draw_callback as *const ()) }),
  };

#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
static mut Xxic_preedit_caret_callback: XIMCallback =
  XIMCallback {
    client_data: std::ptr::null_mut (),
    callback: Some (unsafe { core::mem::transmute::<*const (),
                             unsafe extern "C" fn (XIM, XPointer, XPointer)> (
                               xic_preedit_caret_callback as *const ()) }),
  };

#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
static mut Xxic_preedit_done_callback: XIMCallback =
  XIMCallback {
    client_data: std::ptr::null_mut (),
    callback: Some (xic_preedit_done_callback
                    as unsafe extern "C" fn (XIM, XPointer, XPointer)),
  };

/* C 2757-2769: the `#ifdef HAVE_XICCALLBACK_CALLBACK` pair - the record is an `XICCallback`
   whose `callback` is an `XICProc` when the configuration has that callback, and an `XIMCallback`
   with C's `(XIMProc)` cast otherwise.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n",
            feature = "have_xiccallback_callback"))]
static mut Xxic_preedit_start_callback: XICCallback =
  XICCallback {
    client_data: std::ptr::null_mut (),
    callback: Some (xic_preedit_start_callback
                    as unsafe extern "C" fn (XIC, XPointer, XPointer) -> Bool),
  };
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n",
            not (feature = "have_xiccallback_callback")))]
static mut Xxic_preedit_start_callback: XIMCallback =
  XIMCallback {
    client_data: std::ptr::null_mut (),
    callback: Some (unsafe { core::mem::transmute::<*const (),
                             unsafe extern "C" fn (XIM, XPointer, XPointer)> (
                               xic_preedit_start_callback as *const ()) }),
  };

/* C 2771-2776.  C's initialiser casts `xic_string_conversion_callback`, whose definition is C
   3838 - this file's last function, below this record in the same unit.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
static mut Xxic_string_conversion_callback: XIMCallback =
  XIMCallback {
    /* This is actually an XICCallback! */
    client_data: std::ptr::null_mut (),
    callback: Some (unsafe { core::mem::transmute::<*const (),
                             unsafe extern "C" fn (XIM, XPointer, XPointer)> (
                               xic_string_conversion_callback as *const ()) }),
  };

/* C 2778-2903: the `#if defined HAVE_X_WINDOWS && defined USE_X_TOOLKIT` block - the default
   fontset name, `xic_create_fontsetname` and the `#endif`.  With the file gate already on, C's
   pair reduces to `USE_X_TOOLKIT`, so the port's gate is `use_x_toolkit`.  */

/* Create an X fontset on frame F with base font name BASE_FONTNAME.  */

/// C: xfns.c 2780: `static const char xic_default_fontset[] =
/// "-*-*-*-r-normal--14-*-*-*-*-*-*-*";` - an array with a stable address, which
/// `xic_create_fontsetname` (C 2794) compares against its argument's *pointer*, so the port
/// keeps it a `&CStr` static (`src/emacs.rs` 437's shape).
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
static xic_default_fontset: &core::ffi::CStr = c"-*-*-*-r-normal--14-*-*-*-*-*-*-*";

/* Create an Xt fontset spec from the name of a base font.
   If `motif' is True use the Motif syntax.  */

#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xic_create_fontsetname (mut base_fontname: *const c_char,
                                                 motif: bool) -> *mut c_char
{
  let sep: *const c_char = if motif { c";".as_ptr () } else { c",".as_ptr () };
  let mut fontsetname: *mut c_char;
  let mut z: *mut c_char;

  /* Make a fontset name from the base font name.  */
  if std::ptr::eq (xic_default_fontset.as_ptr (), base_fontname)
    {
      /* There is no base font name, use the default.  */
      fontsetname = xmalloc (strlen (base_fontname) + 2) as *mut c_char;
      z = stpcpy (fontsetname, base_fontname);
    }
  else
    {
      /* Make a fontset name from the base font name.
	 The font set will be made of the following elements:
	 - the base font.
	 - the base font where the charset spec is replaced by -*-*.
	 - the same but with the family also replaced with -*-*-.  */
      let mut p: *const c_char = base_fontname;
      let mut i: ptrdiff_t;

      /* C: `for (i = 0; *p; p++) if (*p == '-') i++;`.  */
      i = 0;
      while *p != 0
	{
	  if *p == b'-' as c_char
	    { i += 1; }
	  p = p.offset (1);
	}
      if i != 14
	{
	  /* As the font name doesn't conform to XLFD, we can't
	     modify it to generalize it to allcs and allfamilies.
	     Use the specified font plus the default.  */
	  fontsetname = xmalloc (strlen (base_fontname)
				 + strlen (xic_default_fontset.as_ptr ()) + 3) as *mut c_char;
	  z = stpcpy (fontsetname, base_fontname);
	  z = stpcpy (z, sep);
	  z = stpcpy (z, xic_default_fontset.as_ptr ());
	}
      else
	{
	  let mut len: ptrdiff_t;
	  let mut p1: *const c_char = std::ptr::null ();
	  let mut p2: *const c_char = std::ptr::null ();
	  let mut p3: *const c_char = std::ptr::null ();
	  let mut font_allcs: *mut c_char = std::ptr::null_mut ();
	  let mut font_allfamilies: *mut c_char = std::ptr::null_mut ();
	  let mut font_all: *mut c_char = std::ptr::null_mut ();
	  let allcs: *const c_char = c"*-*-*-*-*-*-*".as_ptr ();
	  let allfamilies: *const c_char = c"-*-*-".as_ptr ();
	  let all: *const c_char = c"*-*-*-*-".as_ptr ();
	  let mut base: *mut c_char;

	  /* C: `for (i = 0, p = base_fontname; i < 8; p++) { … }`.  */
	  i = 0;
	  p = base_fontname;
	  while i < 8
	    {
	      if *p == b'-' as c_char
		{
		  i += 1;
		  if i == 3
		    { p1 = p.offset (1); }
		  else if i == 7
		    { p2 = p.offset (1); }
		  else if i == 6
		    { p3 = p.offset (1); }
		}
	      p = p.offset (1);
	    }
	  /* If base_fontname specifies ADSTYLE, make it a
	     wildcard.  */
	  if *p3 != b'*' as c_char
	    {
	      let diff: ptrdiff_t = p2.offset_from (p3) - 2;

	      /* C's bare `alloca` is the port's stack pool (PORTING.md P-05/3.2).  */
	      base = sa_stack_alloc (strlen (base_fontname) + 1) as *mut c_char;
	      memcpy (base as *mut c_void, base_fontname as *const c_void,
		      p3.offset_from (base_fontname) as usize);
	      *base.offset (p3.offset_from (base_fontname)) = b'*' as c_char;
	      *base.offset (p3.offset_from (base_fontname) + 1) = b'-' as c_char;
	      strcpy (base.offset (p3.offset_from (base_fontname) + 2), p2);
	      p = base.offset (p.offset_from (base_fontname) - diff);
	      p1 = base.offset (p1.offset_from (base_fontname));
	      p2 = base.offset (p2.offset_from (base_fontname) - diff);
	      base_fontname = base;
	    }

	  /* Build the font spec that matches all charsets.  */
	  len = p.offset_from (base_fontname) + strlen (allcs) as ptrdiff_t + 1;
	  font_allcs = sa_stack_alloc (len as usize) as *mut c_char;
	  memcpy (font_allcs as *mut c_void, base_fontname as *const c_void,
		  p.offset_from (base_fontname) as usize);
	  strcpy (font_allcs.offset (p.offset_from (base_fontname)), allcs);

	  /* Build the font spec that matches all families and
	     add-styles.  */
	  len = p.offset_from (p1) + strlen (allcs) as ptrdiff_t
		+ strlen (allfamilies) as ptrdiff_t + 1;
	  font_allfamilies = sa_stack_alloc (len as usize) as *mut c_char;
	  strcpy (font_allfamilies, allfamilies);
	  memcpy (font_allfamilies.offset (strlen (allfamilies) as isize) as *mut c_void,
		  p1 as *const c_void, p.offset_from (p1) as usize);
	  strcpy (font_allfamilies.offset (strlen (allfamilies) as isize
					   + p.offset_from (p1)), allcs);

	  /* Build the font spec that matches all.  */
	  len = p.offset_from (p2) + strlen (allcs) as ptrdiff_t + strlen (all) as ptrdiff_t
		+ strlen (allfamilies) as ptrdiff_t + 1;
	  font_all = sa_stack_alloc (len as usize) as *mut c_char;
	  z = stpcpy (font_all, allfamilies);
	  z = stpcpy (z, all);
	  memcpy (z as *mut c_void, p2 as *const c_void, p.offset_from (p2) as usize);
	  strcpy (z.offset (p.offset_from (p2)), allcs);

	  /* Build the actual font set name.  */
	  len = strlen (base_fontname) as ptrdiff_t + strlen (font_allcs) as ptrdiff_t
		+ strlen (font_allfamilies) as ptrdiff_t + strlen (font_all) as ptrdiff_t + 5;
	  fontsetname = xmalloc (len as usize) as *mut c_char;
	  z = stpcpy (fontsetname, base_fontname);
	  z = stpcpy (z, sep);
	  z = stpcpy (z, font_allcs);
	  z = stpcpy (z, sep);
	  z = stpcpy (z, font_allfamilies);
	  z = stpcpy (z, sep);
	  z = stpcpy (z, font_all);
	}
    }
  if motif
    { strcpy (z, c":".as_ptr ()); }
  return fontsetname;
}

/* C 2905-2923: the `#ifdef DEBUG_XIC_FONTSET` block and `print_fontset_result` - a debug
   build's reporter, which never compiles in the frozen configuration.  */
#[cfg (all (feature = "have_x_windows", feature = "debug_xic_fontset"))]
unsafe fn print_fontset_result (xfs: XFontSet, name: *mut c_char,
				mut missing_list: *mut *mut c_char, mut missing_count: c_int)
{
  if !xfs.is_null ()
    { fprintf (stderr, c"XIC Fontset created: %s\n".as_ptr (), name); }
  else
    {
      fprintf (stderr, c"XIC Fontset failed: %s\n".as_ptr (), name);
      /* C's `while (missing_count-- > 0)`: the decrement happens on the test that fails too,
	 so the port keeps it inside the condition (the `src/keyboard.rs` 3889 shape).  */
      while { missing_count -= 1; missing_count + 1 > 0 }
	{
	  fprintf (stderr, c"  missing: %s\n".as_ptr (), *missing_list);
	  missing_list = missing_list.offset (1);
	}
    }
}

/* C 2925-3020.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe fn xic_create_xfontset (f: *mut frame) -> XFontSet
{
  let mut xfs: XFontSet = std::ptr::null_mut ();
  let font: *mut font = FRAME_FONT! (f);
  let pixel_size: c_int = (*font).pixel_size;
  let mut rest: Lisp_Object;
  let mut frame: Lisp_Object;

  /* See if there is another frame already using same fontset.  */
  FOR_EACH_FRAME! (rest, frame, {
    let cf: *mut frame = XFRAME (frame);

    if !std::ptr::eq (cf, f) && FRAME_LIVE_P (f) && FRAME_X_P (cf)
	&& FRAME_DISPLAY_INFO! (cf) == FRAME_DISPLAY_INFO! (f)
	&& !FRAME_FONT! (f).is_null ()
	&& (*FRAME_FONT! (f)).pixel_size == pixel_size
      {
	xfs = FRAME_XIC_FONTSET! (cf);
	break;
      }
  });

  if xfs.is_null ()
    {
      let mut buf: [c_char; 256] = [0; 256];
      let mut missing_list: *mut *mut c_char = std::ptr::null_mut ();
      let mut missing_count: c_int = 0;
      let mut def_string: *mut c_char = std::ptr::null_mut ();
      let xlfd_format: *const c_char = c"-*-*-medium-r-normal--%d-*-*-*-*-*".as_ptr ();

      sprintf (buf.as_mut_ptr (), xlfd_format, pixel_size);
      missing_list = std::ptr::null_mut ();
      xfs = XCreateFontSet (FRAME_X_DISPLAY! (f), buf.as_ptr (),
			    &raw mut missing_list, &raw mut missing_count,
			    &raw mut def_string);
      #[cfg (feature = "debug_xic_fontset")]
      { print_fontset_result (xfs, buf.as_mut_ptr (), missing_list, missing_count); }
      if !missing_list.is_null ()
	{ XFreeStringList (missing_list); }
      if xfs.is_null ()
	{
	  /* List of pixel sizes most likely available.  Find one that
	     is closest to pixel_size.  */
	  let mut sizes: [c_int; 13] = [0, 8, 10, 11, 12, 14, 17, 18, 20, 24, 26, 34, 0];
	  let mut smaller: *mut c_int;
	  let mut larger: *mut c_int;

	  /* C: `for (smaller = sizes; smaller[1]; smaller++) if (smaller[1] >= pixel_size)
	     break;`.  */
	  smaller = sizes.as_mut_ptr ();
	  while *smaller.offset (1) != 0
	    {
	      if *smaller.offset (1) >= pixel_size
		{ break; }
	      smaller = smaller.offset (1);
	    }
	  larger = smaller.offset (1);
	  if *larger == pixel_size
	    { larger = larger.offset (1); }
	  while *smaller != 0 || *larger != 0
	    {
	      let mut this_size: c_int;

	      /* C's `*smaller--`/`*larger++`: the read first, the pointer step second.  */
	      if *larger == 0
		{
		  this_size = *smaller;
		  smaller = smaller.offset (-1);
		}
	      else if *smaller == 0
		{
		  this_size = *larger;
		  larger = larger.offset (1);
		}
	      else if pixel_size - *smaller < *larger - pixel_size
		{
		  this_size = *smaller;
		  smaller = smaller.offset (-1);
		}
	      else
		{
		  this_size = *larger;
		  larger = larger.offset (1);
		}
	      sprintf (buf.as_mut_ptr (), xlfd_format, this_size);
	      missing_list = std::ptr::null_mut ();
	      xfs = XCreateFontSet (FRAME_X_DISPLAY! (f), buf.as_ptr (),
				    &raw mut missing_list, &raw mut missing_count,
				    &raw mut def_string);
	      #[cfg (feature = "debug_xic_fontset")]
	      { print_fontset_result (xfs, buf.as_mut_ptr (), missing_list, missing_count); }
	      if !missing_list.is_null ()
		{ XFreeStringList (missing_list); }
	      if !xfs.is_null ()
		{ break; }
	    }
	}
      if xfs.is_null ()
	{
	  let last_resort: *const c_char = c"-*-*-*-r-normal--*-*-*-*-*-*".as_ptr ();

	  missing_list = std::ptr::null_mut ();
	  xfs = XCreateFontSet (FRAME_X_DISPLAY! (f), last_resort,
				&raw mut missing_list, &raw mut missing_count,
				&raw mut def_string);
	  #[cfg (feature = "debug_xic_fontset")]
	  { print_fontset_result (xfs, last_resort as *mut c_char, missing_list,
				  missing_count); }
	  if !missing_list.is_null ()
	    { XFreeStringList (missing_list); }
	}

    }

  return xfs;
}

/* Free the X fontset of frame F if it is the last frame using it.  */

/* C 3025-3051.  C's definition has no `static`, so the port keeps it exported (PORTING.md 0.2).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xic_free_xfontset (f: *mut frame)
{
  let mut rest: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut shared_p: bool = false;

  if FRAME_XIC_FONTSET! (f).is_null ()
    { return; }

  /* See if there is another frame sharing the same fontset.  */
  FOR_EACH_FRAME! (rest, frame, {
    let cf: *mut frame = XFRAME (frame);
    if !std::ptr::eq (cf, f) && FRAME_LIVE_P (f) && FRAME_X_P (cf)
	&& FRAME_DISPLAY_INFO! (cf) == FRAME_DISPLAY_INFO! (f)
	&& FRAME_XIC_FONTSET! (cf) == FRAME_XIC_FONTSET! (f)
      {
	shared_p = true;
	break;
      }
  });

  if !shared_p
    /* The fontset is not used anymore.  It is safe to free it.  */
    { XFreeFontSet (FRAME_X_DISPLAY! (f), FRAME_XIC_FONTSET! (f)); }

  FRAME_XIC_FONTSET! (f) = std::ptr::null_mut ();
}

/* Create XIC for frame F. */

/* C 3067-3068.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
static supported_xim_styles: [XIMStyle; 5] =
  [
    STYLE_NONE,
    STYLE_CALLBACK,
    STYLE_OVERTHESPOT,
    STYLE_OFFTHESPOT,
    STYLE_ROOT
  ];

/* Value is the best input style, given user preferences USER (already
   checked to be supported by Emacs), and styles supported by the
   input method XIM.  */

/* C 3069-3085.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe fn best_xim_style (dpyinfo: *mut x_display_info, xim: *mut XIMStyles) -> XIMStyle
{
  let mut i: c_int;
  let mut j: c_int;
  let nr_supported: c_int = countof! (supported_xim_styles) as c_int;

  if (*dpyinfo).preferred_xim_style != 0
    { return (*dpyinfo).preferred_xim_style; }

  /* C: `for (i = 0; i < nr_supported; ++i) for (j = 0; j < xim->count_styles; ++j) …`.  */
  i = 0;
  while i < nr_supported
    {
      j = 0;
      while j < (*xim).count_styles as c_int
	{
	  if supported_xim_styles[i as usize] == *(*xim).supported_styles.offset (j as isize)
	    { return supported_xim_styles[i as usize]; }
	  j += 1;
	}
      i += 1;
    }

  /* Return the default style.  */
  return XIMPreeditNothing | XIMStatusNothing;
}

/* =====================================================================================
   C 3086-3811: the XIC block's second half.  It opens at C 3086 (the blank after
   `best_xim_style`'s `}`) and closes at C 3811, `xic_preedit_draw_callback`'s `}`.
   ===================================================================================== */

/* `input_event::kind` is termhooks.h 373-412's 16-bit field, which `include/termhooks.rs`
   shares with `part : 16` in one storage word (PORTING.md 1.3), so C's `ie.kind = x;` keeps
   C's field name in the per-unit macro `src/androidterm.rs` 199-209 and `src/dbusbind.rs`
   2505-2515 carry, body-identical (P-80's class).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* `xg_get_scale` is `gtkutil.h`'s (the same dead-in-the-frozen-configuration mirror as the two
   scroll-bar defaults above), declared here under C's own gate (P-88 ①).  */
/// C: gtkutil.h 181: `extern int xg_get_scale (struct frame *f);`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn xg_get_scale (f: *mut frame) -> c_int;
}

/* Create XIC for frame F. */

/* C 3089-3229.  C's definition has no `static`, so the port keeps it exported (PORTING.md 0.2).
   Its `goto out;` is C's forward jump to the cleanup at C 3220, which the port carries as a
   labelled block (PORTING.md 2.2).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_frame_xic (f: *mut frame)
{
  let mut xim: XIM = std::ptr::null_mut ();
  let mut xic: XIC = std::ptr::null_mut ();
  let mut xfs: XFontSet = std::ptr::null_mut ();
  let mut status_attr: XVaNestedList = std::ptr::null_mut ();
  let mut preedit_attr: XVaNestedList = std::ptr::null_mut ();
  let mut s_area: XRectangle = std::mem::zeroed ();
  let mut spot: XPoint = std::mem::zeroed ();
  let mut xic_style: XIMStyle;

  'out: {
    if !FRAME_XIC! (f).is_null ()
      { break 'out; }

    xim = FRAME_X_XIM! (f);
    if xim.is_null () || FRAME_X_XIM_STYLES! (f).is_null ()
      { break 'out; }

    /* Determine XIC style.  */
    xic_style = best_xim_style (FRAME_DISPLAY_INFO! (f),
				FRAME_X_XIM_STYLES! (f));

    /* Create X fontset. */
    if xic_style & (XIMPreeditPosition | XIMStatusArea) != 0
      {
	xfs = xic_create_xfontset (f);
	if xfs.is_null ()
	  { break 'out; }

	FRAME_XIC_FONTSET! (f) = xfs;
      }

    if xic_style & XIMPreeditPosition != 0
      {
	spot.x = 0; spot.y = 1;
	preedit_attr = XVaCreateNestedList (0,
					    XNFontSet (), xfs,
					    XNForeground (),
					    FRAME_FOREGROUND_PIXEL! (f),
					    XNBackground (),
					    FRAME_BACKGROUND_PIXEL! (f),
					    (if xic_style & XIMPreeditPosition != 0
					       { XNSpotLocation () }
					     else { std::ptr::null_mut::<c_char> () }),
					    &raw mut spot,
					    std::ptr::null_mut::<c_void> ());

	if preedit_attr.is_null ()
	  { break 'out; }
      }

    if xic_style & XIMStatusArea != 0
      {
	s_area.x = 0; s_area.y = 0; s_area.width = 1; s_area.height = 1;
	status_attr = XVaCreateNestedList (0,
					   XNArea (),
					   &raw mut s_area,
					   XNFontSet (),
					   xfs,
					   XNForeground (),
					   FRAME_FOREGROUND_PIXEL! (f),
					   XNBackground (),
					   FRAME_BACKGROUND_PIXEL! (f),
					   std::ptr::null_mut::<c_void> ());

	if status_attr.is_null ()
	  { break 'out; }
      }

    if xic_style & XIMPreeditCallbacks != 0
      {
	spot.x = 0;
	spot.y = 0;
	preedit_attr = XVaCreateNestedList (0,
					    XNSpotLocation (), &raw mut spot,
					    XNPreeditStartCallback (),
					    &raw mut Xxic_preedit_start_callback,
					    XNPreeditDoneCallback (),
					    &raw mut Xxic_preedit_done_callback,
					    XNPreeditDrawCallback (),
					    &raw mut Xxic_preedit_draw_callback,
					    XNPreeditCaretCallback (),
					    &raw mut Xxic_preedit_caret_callback,
					    std::ptr::null_mut::<c_void> ());

	if preedit_attr.is_null ()
	  { break 'out; }
      }

    if !preedit_attr.is_null () && !status_attr.is_null ()
      {
	xic = XCreateIC (xim,
			 XNInputStyle (), xic_style,
			 XNClientWindow (), FRAME_X_WINDOW! (f),
			 XNFocusWindow (), FRAME_X_WINDOW! (f),
			 XNStatusAttributes (), status_attr,
			 XNPreeditAttributes (), preedit_attr,
			 XNStringConversionCallback (),
			 &raw mut Xxic_string_conversion_callback,
			 std::ptr::null_mut::<c_void> ());
      }
    else if !preedit_attr.is_null ()
      {
	xic = XCreateIC (xim,
			 XNInputStyle (), xic_style,
			 XNClientWindow (), FRAME_X_WINDOW! (f),
			 XNFocusWindow (), FRAME_X_WINDOW! (f),
			 XNPreeditAttributes (), preedit_attr,
			 XNStringConversionCallback (),
			 &raw mut Xxic_string_conversion_callback,
			 std::ptr::null_mut::<c_void> ());
      }
    else if !status_attr.is_null ()
      {
	xic = XCreateIC (xim,
			 XNInputStyle (), xic_style,
			 XNClientWindow (), FRAME_X_WINDOW! (f),
			 XNFocusWindow (), FRAME_X_WINDOW! (f),
			 XNStatusAttributes (), status_attr,
			 XNStringConversionCallback (),
			 &raw mut Xxic_string_conversion_callback,
			 std::ptr::null_mut::<c_void> ());
      }
    else
      {
	xic = XCreateIC (xim,
			 XNInputStyle (), xic_style,
			 XNClientWindow (), FRAME_X_WINDOW! (f),
			 XNFocusWindow (), FRAME_X_WINDOW! (f),
			 XNStringConversionCallback (),
			 &raw mut Xxic_string_conversion_callback,
			 std::ptr::null_mut::<c_void> ());
      }

    if xic.is_null ()
      { break 'out; }

    FRAME_XIC! (f) = xic;
    FRAME_XIC_STYLE! (f) = xic_style;
    xfs = std::ptr::null_mut (); /* Don't free below.  */
  }

  /* out: */

  if !xfs.is_null ()
    { free_frame_xic (f); }

  if !preedit_attr.is_null ()
    { XFree (preedit_attr); }

  if !status_attr.is_null ()
    { XFree (status_attr); }
}

/* Destroy XIC and free XIC fontset of frame F, if any. */

/* C 3234-3244.  C's definition has no `static`, so the port keeps it exported (PORTING.md 0.2).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_frame_xic (f: *mut frame)
{
  if FRAME_XIC! (f).is_null ()
    { return; }

  XDestroyIC (FRAME_XIC! (f));
  xic_free_xfontset (f);

  FRAME_XIC! (f) = std::ptr::null_mut ();
}

/* Place preedit area for XIC of window W's frame to specified
   pixel position X/Y.  X and Y are relative to window W.  */

/* C 3250-3296.  C's definition has no `static`, so the port keeps it exported (PORTING.md 0.2).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xic_set_preeditarea (w: *mut window, x: c_int, y: c_int)
{
  let f: *mut frame = WINDOW_XFRAME! (w);
  let mut attr: XVaNestedList;
  let mut spot: XPoint = std::mem::zeroed ();

  if !FRAME_XIC! (f).is_null ()
    {
      spot.x = (WINDOW_TO_FRAME_PIXEL_X! (w, x)
		+ WINDOW_LEFT_FRINGE_WIDTH! (w)
		+ WINDOW_LEFT_MARGIN_WIDTH! (w)) as c_short;
      spot.y = (WINDOW_TO_FRAME_PIXEL_Y! (w, y)
		+ (*w).phys_cursor_height) as c_short;

      if FRAME_XIC_STYLE! (f) & XIMPreeditCallbacks != 0
	{
	  attr = XVaCreateNestedList (0, XNSpotLocation (), &raw mut spot,
				      XNPreeditStartCallback (),
				      &raw mut Xxic_preedit_start_callback,
				      XNPreeditDoneCallback (),
				      &raw mut Xxic_preedit_done_callback,
				      XNPreeditDrawCallback (),
				      &raw mut Xxic_preedit_draw_callback,
				      XNPreeditCaretCallback (),
				      &raw mut Xxic_preedit_caret_callback,
				      std::ptr::null_mut::<c_void> ());
	}
      else
	{
	  attr = XVaCreateNestedList (0, XNSpotLocation (), &raw mut spot,
				      std::ptr::null_mut::<c_void> ());
	}
      XSetICValues (FRAME_XIC! (f), XNPreeditAttributes (), attr,
		    std::ptr::null_mut::<c_void> ());
      XFree (attr);
    }
  #[cfg (feature = "use_gtk")]
  {
    /* C's `f->tooltip`: the member is `include/frame.rs`'s `// C:` row (frame.h 457's
       `bool_bf tooltip : 1`), and the mirror's reader for it is `FRAME_TOOLTIP_P`.  */
    if FRAME_TOOLTIP_P (f)
      { return; }

    let mut rect: GdkRectangle = std::mem::zeroed ();
    let scale: c_int = xg_get_scale (f);

    rect.x = (WINDOW_TO_FRAME_PIXEL_X! (w, x)
	      + WINDOW_LEFT_FRINGE_WIDTH! (w)
	      + WINDOW_LEFT_MARGIN_WIDTH! (w)) / scale;
    rect.y = (WINDOW_TO_FRAME_PIXEL_Y! (w, y)
	      + FRAME_TOOLBAR_HEIGHT! (f)
	      + FRAME_MENUBAR_HEIGHT! (f)) / scale;
    rect.width = (*w).phys_cursor_width / scale;
    rect.height = (*w).phys_cursor_height / scale;

    gtk_im_context_set_cursor_location ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).im_context,
					&raw const rect);
  }
}

/* Place status area for XIC in bottom right corner of frame F.. */

/* C 3301-3339.  C's definition has no `static`, so the port keeps it exported (PORTING.md 0.2).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xic_set_statusarea (f: *mut frame)
{
  let xic: XIC = FRAME_XIC! (f);
  let mut attr: XVaNestedList;
  let mut area: XRectangle = std::mem::zeroed ();
  let mut needed: *mut XRectangle = std::ptr::null_mut ();

  /* Negotiate geometry of status area.  If input method has existing
     status area, use its current size.  */
  area.x = 0; area.y = 0; area.width = 0; area.height = 0;
  attr = XVaCreateNestedList (0, XNAreaNeeded (), &raw mut area,
			      std::ptr::null_mut::<c_void> ());
  XSetICValues (xic, XNStatusAttributes (), attr, std::ptr::null_mut::<c_void> ());
  XFree (attr);

  attr = XVaCreateNestedList (0, XNAreaNeeded (), &raw mut needed,
			      std::ptr::null_mut::<c_void> ());
  XGetICValues (xic, XNStatusAttributes (), attr, std::ptr::null_mut::<c_void> ());
  XFree (attr);

  if (*needed).width == 0 /* Use XNArea instead of XNAreaNeeded */
    {
      attr = XVaCreateNestedList (0, XNArea (), &raw mut needed,
				  std::ptr::null_mut::<c_void> ());
      XGetICValues (xic, XNStatusAttributes (), attr, std::ptr::null_mut::<c_void> ());
      XFree (attr);
    }

  area.width  = (*needed).width;
  area.height = (*needed).height;
  area.x = (FRAME_PIXEL_WIDTH (f) - area.width as c_int
	    - FRAME_INTERNAL_BORDER_WIDTH (f)) as c_short;
  area.y = (FRAME_PIXEL_HEIGHT (f) - area.height as c_int
	    - FRAME_MENUBAR_HEIGHT! (f)
	    - FRAME_TOOLBAR_TOP_HEIGHT! (f)
	    - FRAME_INTERNAL_BORDER_WIDTH (f)) as c_short;
  XFree (needed as *mut c_void);

  attr = XVaCreateNestedList (0, XNArea (), &raw mut area,
			      std::ptr::null_mut::<c_void> ());
  XSetICValues (xic, XNStatusAttributes (), attr, std::ptr::null_mut::<c_void> ());
  XFree (attr);
}

/* C 3341-3356.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe fn x_xic_to_frame (xic: XIC) -> *mut frame
{
  let mut tail: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut f: *mut frame;

  FOR_EACH_FRAME! (tail, tem, {
    f = XFRAME (tem);

    if FRAME_X_P (f) && FRAME_XIC! (f) == xic
      { return f; }
  });

  return std::ptr::null_mut ();
}

/* C 3358-3380.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" fn xic_preedit_start_callback (xic: XIC, client_data: XPointer,
					      call_data: XPointer) -> c_int
{
  let f: *mut frame = x_xic_to_frame (xic);
  let mut output: *mut x_output;

  if !f.is_null ()
    {
      output = FRAME_X_OUTPUT! (f) as *mut x_output;

      (*output).preedit_size = 0;
      (*output).preedit_active = true;
      (*output).preedit_caret = 0;

      if !(*output).preedit_chars.is_null ()
	{ xfree ((*output).preedit_chars as *mut c_void); }

      (*output).preedit_chars = std::ptr::null_mut ();
    }

  return -1;
}

/* C 3382-3435.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" fn xic_preedit_caret_callback (xic: XIC, client_data: XPointer,
					      call_data: *mut XIMPreeditCaretCallbackStruct)
{
  let f: *mut frame = x_xic_to_frame (xic);
  let mut output: *mut x_output;
  let mut ie: input_event = std::mem::zeroed ();
  EVENT_INIT (&raw mut ie);

  if !f.is_null ()
    {
      output = FRAME_X_OUTPUT! (f) as *mut x_output;

      if !(*output).preedit_active
	{ return; }

      match (*call_data).direction
	{
	  XIMAbsolutePosition =>
	    { (*output).preedit_caret = (*call_data).position; }
	  XIMForwardChar | XIMForwardWord =>
	    {
	      /* C's `output->preedit_caret++`: the read first, the increment second.  */
	      (*call_data).position = (*output).preedit_caret;
	      (*output).preedit_caret += 1;
	    }
	  XIMBackwardChar | XIMBackwardWord =>
	    {
	      /* C's `max (0, output->preedit_caret--)`.  */
	      (*call_data).position = max (0, (*output).preedit_caret as isize) as c_int;
	      (*output).preedit_caret -= 1;
	    }
	  _ =>
	    { (*call_data).position = (*output).preedit_caret; }
	}

      if !(*output).preedit_chars.is_null ()
	{
	  input_event_bits! (kind, &raw mut ie, PREEDIT_TEXT_EVENT);
	  XSETFRAME! (ie.frame_or_window, f);
	  ie.arg = make_string_from_utf8 ((*output).preedit_chars,
					  (*output).preedit_size);

	  if SCHARS (ie.arg) != 0
	    {
	      Fput_text_property (make_fixnum (min (SCHARS (ie.arg) - 1,
						    max (0, (*output).preedit_caret as isize))),
				  make_fixnum (max (SCHARS (ie.arg),
						    max (0, (*output).preedit_caret as isize) + 1)),
				  Qcursor (), Qt (), ie.arg);
	    }

	  XSETINT! (ie.x, 0);
	  XSETINT! (ie.y, 0);

	  kbd_buffer_store_event (&raw mut ie as *mut c_void);
	}
    }
}

/* C 3438-3466.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" fn xic_preedit_done_callback (xic: XIC, client_data: XPointer,
					     call_data: XPointer)
{
  let f: *mut frame = x_xic_to_frame (xic);
  let mut output: *mut x_output;
  let mut ie: input_event = std::mem::zeroed ();
  EVENT_INIT (&raw mut ie);

  if !f.is_null ()
    {
      input_event_bits! (kind, &raw mut ie, PREEDIT_TEXT_EVENT);
      ie.arg = Qnil ();
      XSETFRAME! (ie.frame_or_window, f);
      XSETINT! (ie.x, 0);
      XSETINT! (ie.y, 0);
      kbd_buffer_store_event (&raw mut ie as *mut c_void);

      output = FRAME_X_OUTPUT! (f) as *mut x_output;

      if !(*output).preedit_chars.is_null ()
	{ xfree ((*output).preedit_chars as *mut c_void); }

      (*output).preedit_size = 0;
      (*output).preedit_active = false;
      (*output).preedit_chars = std::ptr::null_mut ();
      (*output).preedit_caret = 0;
    }
}

/* C 3468-3475.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
pub struct x_xim_text_conversion_data
{
  pub coding: *mut coding_system,          // C: struct coding_system *coding;
  pub source: *mut c_char,                 // C: char *source;
  pub dpyinfo: *mut x_display_info,        // C: struct x_display_info *dpyinfo;
  pub size: ptrdiff_t,                     // C: ptrdiff_t size;
}

/* `internal_condition_case_n` is lisp.h 4808's prototype, which no mirror of this unit's graph
   carries; the definition is `src/eval.rs` 1741's, and this is the declaration
   `src/keyboard.rs` 560-566 carries (P-88 ①).  */
/// C: lisp.h 4808: `extern Lisp_Object internal_condition_case_n
///   (Lisp_Object (*) (ptrdiff_t, Lisp_Object *), ptrdiff_t, Lisp_Object *,
///    Lisp_Object, Lisp_Object (*) (Lisp_Object, ptrdiff_t, Lisp_Object *));`
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" {
  pub fn internal_condition_case_n (bfun: unsafe extern "C" fn (ptrdiff_t, *mut Lisp_Object)
                                                                   -> Lisp_Object,
                                    nargs: ptrdiff_t, args: *mut Lisp_Object,
                                    handlers: Lisp_Object,
                                    hfun: unsafe extern "C" fn (Lisp_Object, ptrdiff_t,
                                                                *mut Lisp_Object)
                                                                   -> Lisp_Object)
                                    -> Lisp_Object;
}

/* C 3477-3506.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" fn x_xim_text_to_utf8_unix_1 (nargs: ptrdiff_t,
					      args: *mut Lisp_Object) -> Lisp_Object
{
  let mut data: *mut x_xim_text_conversion_data;
  let mut nbytes: ptrdiff_t;
  let mut coding_system: Lisp_Object;

  data = xmint_pointer (*args.offset (0)) as *mut x_xim_text_conversion_data;

  if SYMBOLP (Vx_input_coding_system)
    { coding_system = Vx_input_coding_system; }
  else if !NILP ((*(*data).dpyinfo).xim_coding)
    { coding_system = (*(*data).dpyinfo).xim_coding; }
  else
    { coding_system = Vlocale_coding_system; }

  nbytes = strlen ((*data).source) as ptrdiff_t;

  (*(*data).coding).destination = std::ptr::null_mut ();

  setup_coding_system (coding_system, (*data).coding);
  /* C's `data->coding->mode |= (CODING_MODE_LAST_BLOCK | CODING_MODE_SAFE_ENCODING);` - the
     `mode` member of the mirror's one storage word (see `x_encode_text` above).  */
  (*(*data).coding).bits
    |= ((CODING_MODE_LAST_BLOCK | CODING_MODE_SAFE_ENCODING) as u32) << CODING_SYSTEM_MODE_SHIFT;
  (*(*data).coding).source = (*data).source as *const u8;
  (*(*data).coding).dst_bytes = 2048;
  (*(*data).coding).destination = xmalloc (2048) as *mut u8;
  decode_coding_object ((*data).coding, Qnil (), 0, 0,
			nbytes, nbytes, Qnil ());

  return Qnil ();
}

/* C 3509-3538.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" fn x_encode_xim_text_1 (nargs: ptrdiff_t,
					  args: *mut Lisp_Object) -> Lisp_Object
{
  let mut data: *mut x_xim_text_conversion_data;
  let mut nbytes: ptrdiff_t;
  let mut coding_system: Lisp_Object;

  data = xmint_pointer (*args.offset (0)) as *mut x_xim_text_conversion_data;

  if SYMBOLP (Vx_input_coding_system)
    { coding_system = Vx_input_coding_system; }
  else if !NILP ((*(*data).dpyinfo).xim_coding)
    { coding_system = (*(*data).dpyinfo).xim_coding; }
  else
    { coding_system = Vlocale_coding_system; }

  nbytes = (*data).size;

  (*(*data).coding).destination = std::ptr::null_mut ();

  setup_coding_system (coding_system, (*data).coding);
  /* C's `data->coding->mode |= (CODING_MODE_LAST_BLOCK | CODING_MODE_SAFE_ENCODING);`.  */
  (*(*data).coding).bits
    |= ((CODING_MODE_LAST_BLOCK | CODING_MODE_SAFE_ENCODING) as u32) << CODING_SYSTEM_MODE_SHIFT;
  (*(*data).coding).source = (*data).source as *const u8;
  (*(*data).coding).dst_bytes = 2048;
  (*(*data).coding).destination = xmalloc (2048) as *mut u8;
  encode_coding_object ((*data).coding, Qnil (), 0, 0,
			nbytes, nbytes, Qnil ());

  return Qnil ();
}

/* C 3541-3554.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" fn x_xim_text_to_utf8_unix_2 (val: Lisp_Object, nargs: ptrdiff_t,
					     args: *mut Lisp_Object) -> Lisp_Object
{
  let mut data: *mut x_xim_text_conversion_data;

  data = xmint_pointer (*args.offset (0)) as *mut x_xim_text_conversion_data;

  if !(*(*data).coding).destination.is_null ()
    { xfree ((*(*data).coding).destination as *mut c_void); }

  (*(*data).coding).destination = std::ptr::null_mut ();

  return Qnil ();
}

/* The string returned is not null-terminated.  */
/* C 3557-3595.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe fn x_xim_text_to_utf8_unix (dpyinfo: *mut x_display_info, text: *mut XIMText,
				   length: *mut ptrdiff_t) -> *mut c_char
{
  let mut wchar_buf: *mut c_uchar;
  let mut wchar_actual_length: ptrdiff_t;
  let mut i: ptrdiff_t;
  let mut coding: coding_system = std::mem::zeroed ();
  let mut data: x_xim_text_conversion_data = std::mem::zeroed ();
  let mut was_waiting_for_input_p: bool;
  let mut arg: Lisp_Object;

  if (*text).encoding_is_wchar != 0
    {
      /* C's `(text->length + 1) * MAX_MULTIBYTE_LENGTH` - C promotes the `unsigned short` to
	 `int` for the sum and the product, then hands the result to `size_t`.  */
      wchar_buf = xmalloc ((((*text).length as c_int) + 1) as usize
			   * MAX_MULTIBYTE_LENGTH as usize) as *mut c_uchar;
      wchar_actual_length = 0;

      /* C: `for (i = 0; i < text->length; ++i) …`.  */
      i = 0;
      while i < (*text).length as ptrdiff_t
	{
	  /* `text->string.wide_char[i]` is the union's member read at C's index.  */
	  wchar_actual_length += CHAR_STRING (*(*text).string.wide_char.offset (i),
					      wchar_buf.offset (wchar_actual_length)) as ptrdiff_t;
	  i += 1;
	}
      *length = wchar_actual_length;

      return wchar_buf as *mut c_char;
    }

  data.coding = &raw mut coding;
  data.source = (*text).string.multi_byte;
  data.dpyinfo = dpyinfo;

  was_waiting_for_input_p = waiting_for_input! ();
  /* Otherwise Fsignal will crash.  */
  waiting_for_input! () = false;
  arg = make_mint_ptr (&raw mut data as *mut c_void);
  internal_condition_case_n (x_xim_text_to_utf8_unix_1, 1, &raw mut arg,
			     Qt (), x_xim_text_to_utf8_unix_2);
  waiting_for_input! () = was_waiting_for_input_p;

  *length = coding.produced;
  return coding.destination as *mut c_char;
}

/* Convert SIZE bytes of the specified text from Emacs's internal
   coding system to the input method coding system.  Return the
   result, its byte length in *LENGTH, and its character length in
   *CHARS, or NULL.

   The string returned is not NULL terminated.  */

/* C 3604-3635.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe fn x_encode_xim_text (dpyinfo: *mut x_display_info, text: *mut c_char,
			     size: ptrdiff_t, length: *mut ptrdiff_t,
			     chars: *mut ptrdiff_t) -> *mut c_char
{
  let mut coding: coding_system = std::mem::zeroed ();
  let mut data: x_xim_text_conversion_data = std::mem::zeroed ();
  let mut arg: Lisp_Object;
  let mut was_waiting_for_input_p: bool;

  data.coding = &raw mut coding;
  data.source = text;
  data.dpyinfo = dpyinfo;
  data.size = size;

  was_waiting_for_input_p = waiting_for_input! ();
  /* Otherwise Fsignal will crash.  */
  waiting_for_input! () = false;

  arg = make_mint_ptr (&raw mut data as *mut c_void);
  internal_condition_case_n (x_encode_xim_text_1, 1, &raw mut arg,
			     Qt (), x_xim_text_to_utf8_unix_2);
  waiting_for_input! () = was_waiting_for_input_p;

  if !length.is_null ()
    { *length = coding.produced; }

  if !chars.is_null ()
    { *chars = coding.produced_char; }

  return coding.destination as *mut c_char;
}

/* C 3637-3811.  The `goto im_abort` sites (C 3668, 3680, 3694, 3709, 3717, 3726, 3738 and 3764)
   jump to C 3802's label, which sits after C 3799's `return;`; the port carries it as a labelled
   block whose `break` leaves the work and whose end falls into the cleanup, with C's `return;`
   kept at the end of the block (PORTING.md 2.2).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" fn xic_preedit_draw_callback (xic: XIC, client_data: XPointer,
					     call_data: *mut XIMPreeditDrawCallbackStruct)
{
  let mut f: *mut frame;
  let mut output: *mut x_output;
  let mut text_length: ptrdiff_t;
  let mut charpos: ptrdiff_t;
  let mut original_size: ptrdiff_t;
  let mut text: *mut c_char;
  let mut chg_start: *mut c_char;
  let mut chg_end: *mut c_char;
  let mut ie: input_event = std::mem::zeroed ();

  f = x_xic_to_frame (xic);
  EVENT_INIT (&raw mut ie);

  if !f.is_null ()
    {
      text_length = 0;
      output = FRAME_X_OUTPUT! (f) as *mut x_output;

      if !(*output).preedit_active
	{ return; }

      'im_abort: {
	if !(*call_data).text.is_null ()
	  {
	    text = x_xim_text_to_utf8_unix (FRAME_DISPLAY_INFO! (f),
					    (*call_data).text, &raw mut text_length);

	    if text.is_null ()
	      /* Decoding the IM text failed.  */
	      { break 'im_abort; }
	  }
	else
	  { text = std::ptr::null_mut (); }

	original_size = (*output).preedit_size;

	/* This is an ordinary insertion: reallocate the buffer to hold
	   enough for TEXT.  */
	if (*call_data).chg_length == 0
	  {
	    if text.is_null ()
	      { break 'im_abort; }

	    if !(*output).preedit_chars.is_null ()
	      {
		/* C's `output->preedit_chars = xrealloc (output->preedit_chars,
		   output->preedit_size += text_length);`: the size step first, then the call
		   with the new value, then C's store.  */
		(*output).preedit_size += text_length;
		(*output).preedit_chars = xrealloc ((*output).preedit_chars as *mut c_void,
						    (*output).preedit_size as usize) as *mut c_char;
	      }
	    else
	      {
		(*output).preedit_size += text_length;
		(*output).preedit_chars
		  = xmalloc ((*output).preedit_size as usize) as *mut c_char;
	      }
	  }

	chg_start = (*output).preedit_chars;

	/* The IM sent bad data: the buffer is empty, but the change
	   position is more than 0.  */
	if (*output).preedit_chars.is_null () && (*call_data).chg_first != 0
	  { break 'im_abort; }

	/* Find the byte position for the character position where the
	   first change is to be made.  */
	if (*call_data).chg_first != 0
	  {
	    charpos = 0;

	    while charpos < (*call_data).chg_first as ptrdiff_t
	      {
		chg_start = chg_start.offset (BYTES_BY_CHAR_HEAD (*chg_start as c_int) as isize);

		if chg_start.offset_from ((*output).preedit_chars) > (*output).preedit_size
		  /* The IM sent bad data: chg_start is larger than the
		     current buffer.  */
		  { break 'im_abort; }
		charpos += 1;
	      }
	  }

	if (*call_data).chg_length == 0
	  {
	    if text.is_null ()
	      { break 'im_abort; }

	    memmove (chg_start.offset (text_length) as *mut c_void,
		     chg_start as *const c_void,
		     (original_size - chg_start.offset_from ((*output).preedit_chars)) as usize);
	    memcpy (chg_start as *mut c_void, text as *const c_void, text_length as usize);
	  }
	else
	  {
	    if (*call_data).chg_length < 1
	      { break 'im_abort; }

	    charpos = 0;
	    chg_end = chg_start;

	    while charpos < (*call_data).chg_length as ptrdiff_t
	      {
		chg_end = chg_end.offset (BYTES_BY_CHAR_HEAD (*chg_end as c_int) as isize);

		if chg_end.offset_from ((*output).preedit_chars) > (*output).preedit_size
		  /* The IM sent bad data: chg_end ends someplace outside
		     the current buffer.  */
		  { break 'im_abort; }
		charpos += 1;
	      }

	    memmove (chg_start as *mut c_void, chg_end as *const c_void,
		     ((*output).preedit_chars.offset ((*output).preedit_size))
		       .offset_from (chg_end) as usize);
	    (*output).preedit_size -= chg_end.offset_from (chg_start);

	    if !text.is_null ()
	      {
		original_size = (*output).preedit_size;
		(*output).preedit_size += text_length;
		(*output).preedit_chars = xrealloc ((*output).preedit_chars as *mut c_void,
						    (*output).preedit_size as usize) as *mut c_char;

		/* Find chg_start again, since preedit_chars was reallocated.  */

		chg_start = (*output).preedit_chars;
		charpos = 0;

		while charpos < (*call_data).chg_first as ptrdiff_t
		  {
		    chg_start = chg_start.offset (BYTES_BY_CHAR_HEAD (*chg_start as c_int) as isize);

		    if chg_start.offset_from ((*output).preedit_chars) > (*output).preedit_size
		      /* The IM sent bad data: chg_start is larger than the
			 current buffer.  */
		      { break 'im_abort; }
		    charpos += 1;
		  }

		memmove (chg_start.offset (text_length) as *mut c_void,
			 chg_start as *const c_void,
			 (original_size - chg_start.offset_from ((*output).preedit_chars)) as usize);
		memcpy (chg_start as *mut c_void, text as *const c_void, text_length as usize);
	      }
	  }

	if !text.is_null ()
	  { xfree (text as *mut c_void); }

	(*output).preedit_caret = (*call_data).caret;

	/* This is okay because this callback is called from the big XIM
	   event filter, which runs inside XTread_socket.  */

	input_event_bits! (kind, &raw mut ie, PREEDIT_TEXT_EVENT);
	XSETFRAME! (ie.frame_or_window, f);
	ie.arg = make_string_from_utf8 ((*output).preedit_chars,
					(*output).preedit_size);

	if SCHARS (ie.arg) != 0
	  {
	    Fput_text_property (make_fixnum (min (SCHARS (ie.arg) - 1,
						  max (0, (*output).preedit_caret as isize))),
				make_fixnum (min (SCHARS (ie.arg),
						  max (0, (*output).preedit_caret as isize) + 1)),
				Qcursor (), Qt (), ie.arg);
	  }

	XSETINT! (ie.x, 0);
	XSETINT! (ie.y, 0);

	kbd_buffer_store_event (&raw mut ie as *mut c_void);

	/* C's `return;` at C 3799, before the label.  */
	return;
      }

      /* im_abort: */
      if !text.is_null ()
	{ xfree (text as *mut c_void); }
      if !(*output).preedit_chars.is_null ()
	{ xfree ((*output).preedit_chars as *mut c_void); }
      (*output).preedit_chars = std::ptr::null_mut ();
      (*output).preedit_size = 0;
      (*output).preedit_active = false;
      (*output).preedit_caret = 0;
    }
}

/* C 3812-3831.  C's definition has no `static`, so the port keeps it exported (PORTING.md 0.2).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xic_set_xfontset (f: *mut frame, base_fontname: *const c_char)
{
  let mut attr: XVaNestedList;
  let mut xfs: XFontSet;

  xic_free_xfontset (f);

  xfs = xic_create_xfontset (f);

  attr = XVaCreateNestedList (0, XNFontSet (), xfs, std::ptr::null_mut::<c_void> ());
  if FRAME_XIC_STYLE! (f) & XIMPreeditPosition != 0
    { XSetICValues (FRAME_XIC! (f), XNPreeditAttributes (), attr, std::ptr::null_mut::<c_void> ()); }
  if FRAME_XIC_STYLE! (f) & XIMStatusArea != 0
    { XSetICValues (FRAME_XIC! (f), XNStatusAttributes (), attr, std::ptr::null_mut::<c_void> ()); }
  XFree (attr);

  FRAME_XIC_FONTSET! (f) = xfs;
}

/* String conversion support.  See textconv.c for more details.  */

/* C 3837-3954.  C's four `goto failure` (C 3850, 3907, 3924 and 3943) jump to C 3947's label,
   which sits after C 3945's `return;`; the port carries it as C 3947's `failure:` label over a
   labelled block whose handler follows the block (the `'out:` shape above, PORTING.md 2.2).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
use textconv_decls::*;   // C 3919's `textconv_query` (textconv.h 125-126's prototype)

#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe extern "C" fn xic_string_conversion_callback (ic: XIC, client_data: XPointer,
						     call_data: *mut XIMStringConversionCallbackStruct)
{
  let mut request: textconv_callback_struct = std::mem::zeroed ();
  let mut length: ptrdiff_t = 0;
  let mut f: *mut frame;
  let mut rc: c_int;

  'failure: {
    /* Find the frame associated with this IC.  */
    f = x_xic_to_frame (ic);

    if f.is_null ()
      { break 'failure; }

    /* Fill in CALL_DATA as early as possible.  */
    (*(*call_data).text).feedback = std::ptr::null_mut ();
    (*(*call_data).text).encoding_is_wchar = False;

    /* Now translate the conversion request to the format understood by
       textconv.c.  */
    request.position = (*call_data).position as EMACS_INT;

    match (*call_data).direction
      {
	XIMForwardChar =>
	  { request.direction = TEXTCONV_FORWARD_CHAR; }
	XIMBackwardChar =>
	  { request.direction = TEXTCONV_BACKWARD_CHAR; }
	XIMForwardWord =>
	  { request.direction = TEXTCONV_FORWARD_WORD; }
	XIMBackwardWord =>
	  { request.direction = TEXTCONV_BACKWARD_WORD; }
	XIMCaretUp =>
	  { request.direction = TEXTCONV_CARET_UP; }
	XIMCaretDown =>
	  { request.direction = TEXTCONV_CARET_DOWN; }
	XIMNextLine =>
	  { request.direction = TEXTCONV_NEXT_LINE; }
	XIMPreviousLine =>
	  { request.direction = TEXTCONV_PREVIOUS_LINE; }
	XIMLineStart =>
	  { request.direction = TEXTCONV_LINE_START; }
	XIMLineEnd =>
	  { request.direction = TEXTCONV_LINE_END; }
	XIMAbsolutePosition =>
	  { request.direction = TEXTCONV_ABSOLUTE_POSITION; }
	_ =>
	  { break 'failure; }
      }

    /* factor is signed in call_data but is actually a CARD16.  */
    request.factor = (*call_data).factor;

    if (*call_data).operation == XIMStringConversionSubstitution
      { request.operation = TEXTCONV_SUBSTITUTION; }
    else
      { request.operation = TEXTCONV_RETRIEVAL; }

    /* Now perform the string conversion.  */
    rc = textconv_query (f, &raw mut request, 0);

    if rc != 0
      {
	xfree (request.text.text as *mut c_void);
	break 'failure;
      }

    /* Encode the text in the locale coding system and give it back to
       the input method.  */
    request.text.text = std::ptr::null_mut ();
    (*(*call_data).text).string.mbs
      = x_encode_xim_text (FRAME_DISPLAY_INFO! (f),
			   request.text.text,
			   request.text.bytes, std::ptr::null_mut (),
			   &raw mut length);
    (*(*call_data).text).length = length as c_ushort;

    /* Free the encoded text.  This is always set to something
       valid.  */
    xfree (request.text.text as *mut c_void);

    /* Detect failure.  */
    if (*(*call_data).text).string.mbs.is_null ()
      { break 'failure; }

    return;
  }

  /* failure: */
  /* Return a string of length 0 using the C library malloc (1)
     (not malloc (0), to pacify gcc -Walloc-size).  This
     assumes XFree is able to free data allocated with our malloc
     wrapper.  */
  (*(*call_data).text).length = 0;
  (*(*call_data).text).string.mbs = malloc (1) as *mut c_char;
}

/* C 3955-3956: the blank and the `#endif` that closes the C 2728 `#ifdef` over this whole XIM
   block.  Every item of that block carries the gate in its own `#[cfg]`, so the `#endif` is not
   an item of its own.  */

/* C 3961-4168: `x_mark_frame_dirty`, `set_up_x_back_buffer`, `tear_down_x_back_buffer`,
   `initial_set_up_x_back_buffer` and the `HAVE_XINPUT2` block's `setup_xi_event_mask`.  The
   Xdbe/XRender/XInput2 vocabulary these five read has no mirror in this unit's include graph, so
   it is declared here at the use site with the header's own prototypes and values, which on this
   machine *are* measurable (P-72 rules 3-4; `docs/ports/xterm.md`'s slice 10 declares the same
   family in its own use-site block).  Each member carries the guard its C call site has, so a
   name resolves exactly in the arms C gives it to.  */

/// C: X11/extensions/Xdbe.h 55: `typedef XID XdbeBackBuffer;` - Xlib's `XID`, so this port's
/// `c_ulong` (P-72 rule 3); C 4000-4002's and C 4035-4036's calls read it.
#[cfg (all (feature = "have_x_windows", feature = "have_xdbe"))]
pub type XdbeBackBuffer = c_ulong;
/// C: X11/extensions/Xdbe.h 63: `#define XdbeCopied 1` - C 4002's swap action (measured).
#[cfg (all (feature = "have_x_windows", feature = "have_xdbe"))]
pub const XdbeCopied: c_int = 1;
/// C: X11/extensions/Xrender.h 810: `typedef XID Picture;` - Xlib's `XID`; no mirror of this
/// unit's graph declares the name (`include/xterm.rs` 840 reads it in a commented field), so it
/// stands here under the guard its only consumer has (P-72 rule 3).
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
pub type Picture = c_ulong;
/// C: X11/extensions/XInput2.h 911-915: `typedef struct { int deviceid; int mask_len; unsigned
/// char *mask; } XIEventMask;` - the record C 4062/4068-4073/4078/4082/4136 write.  No mirror of
/// this unit's graph declares it, so it is carried at the use site with the header's published
/// field order (P-88 rule 1); the layout is **UNVERIFIED** - the record is only ever read
/// through these three fields.
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
#[repr (C)]
pub struct XIEventMask
{
  pub deviceid: c_int,
  pub mask_len: c_int,
  pub mask: *mut c_uchar,
}

/* C: X11/extensions/Xdbe.h 114/121: `XdbeBackBuffer XdbeAllocateBackBufferName (Display *,
   Window, XdbeSwapAction);` and `Status XdbeDeallocateBackBufferName (Display *, XdbeBackBuffer);`.
   C: X11/extensions/Xrender.h 816: `void XRenderFreePicture (Display *, Picture);`.
   C: X11/extensions/XInput2.h 951: `void XISelectEvents (Display *, Window, XIEventMask *, int);`.
   The three prototypes stand in their own blocks so each carries the guard its call site has; a
   doc comment cannot sit on an `extern` block without the `unused doc comment` warning this file's
   baseline already counts, so they are block comments (`docs/ports/xfns.md`, slice 5).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xdbe"))]
unsafe extern "C" {
  pub fn XdbeAllocateBackBufferName (dpy: *mut Display, window: Window, swap_action: c_int)
				     -> XdbeBackBuffer;
  pub fn XdbeDeallocateBackBufferName (dpy: *mut Display, buffer: XdbeBackBuffer) -> c_int;
}
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
unsafe extern "C" {
  pub fn XRenderFreePicture (dpy: *mut Display, picture: Picture);
}
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe extern "C" {
  pub fn XISelectEvents (dpy: *mut Display, win: Window, masks: *mut XIEventMask,
			 num_masks: c_int) -> c_int;
}

/// C: X11/extensions/XI2.h 187-223, every value measured on this machine's header:
/// `XIAllDevices 0`, `XIAllMasterDevices 1`, `XI_KeyPress 2`, `XI_KeyRelease 3`,
/// `XI_ButtonPress 4`, `XI_ButtonRelease 5`, `XI_Motion 6`, `XI_Enter 7`, `XI_Leave 8`,
/// `XI_FocusIn 9`, `XI_FocusOut 10`, `XI_TouchBegin 18`, `XI_TouchUpdate 19`, `XI_TouchEnd 20`,
/// `XI_TouchOwnership 21`, `XI_GesturePinchBegin 27`, `XI_GesturePinchUpdate 28`,
/// `XI_GesturePinchEnd 29` and `XI_LASTEVENT XI_GestureSwipeEnd` (33) - the masks
/// C 4063-4164 builds.
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XIAllDevices: c_int = 0;             // XI2.h 187
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XIAllMasterDevices: c_int = 1;       // XI2.h 188
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_KeyPress: c_int = 2;              // XI2.h 192
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_KeyRelease: c_int = 3;            // XI2.h 193
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_ButtonPress: c_int = 4;           // XI2.h 194
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_ButtonRelease: c_int = 5;         // XI2.h 195
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_Motion: c_int = 6;                // XI2.h 196
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_Enter: c_int = 7;                 // XI2.h 197
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_Leave: c_int = 8;                 // XI2.h 198
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_FocusIn: c_int = 9;               // XI2.h 199
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_FocusOut: c_int = 10;             // XI2.h 200
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_TouchBegin: c_int = 18;           // XI2.h 208
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_TouchUpdate: c_int = 19;          // XI2.h 209
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_TouchEnd: c_int = 20;             // XI2.h 210
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_TouchOwnership: c_int = 21;       // XI2.h 211
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_GesturePinchBegin: c_int = 27;    // XI2.h 217
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_GesturePinchUpdate: c_int = 28;   // XI2.h 218
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_GesturePinchEnd: c_int = 29;      // XI2.h 219
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_LASTEVENT: c_int = 33;            // XI2.h 223: `XI_GestureSwipeEnd`

/// C: X11/extensions/XInput2.h 939: `#define XIMaskLen(event) (((event) >> 3) + 1)` and
/// `#define XISetMask(ptr, event) (((unsigned char *) (ptr))[((event) >> 3)] |= (1 << ((event) & 7)))`
/// - C 4063's `XIMaskLen (XI_LASTEVENT)` and C 4084-4153's set calls, with the header's own
/// bodies (measured; the port spells macros with the `!` PORTING.md 1.7/D-1 gives them).
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
macro_rules! XIMaskLen { ($event:expr) => { (($event as c_int) >> 3) + 1 }; }
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
macro_rules! XISetMask {
  ($ptr:expr, $event:expr) =>
    { *($ptr).add (($event as usize) >> 3) |= 1u8 << (($event as usize) & 7) };
}

/* Mark the frame as needing a buffer flip the next time it is redrawn.  */

/// C 3961-3969: `void x_mark_frame_dirty (struct frame *f)` - not `static`, so
/// `#[unsafe(no_mangle)] pub unsafe extern "C" fn` (PORTING.md 0.2/1.4).  Its body is empty where
/// `HAVE_XDBE` is not defined, which is C's own shape.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_mark_frame_dirty (f: *mut frame)
{
#[cfg (feature = "have_xdbe")]
  if FRAME_X_DOUBLE_BUFFERED_P! (f)                                     // C 3965
      && !FRAME_X_NEED_BUFFER_FLIP! (f)                                 // C 3966
    { FRAME_X_NEED_BUFFER_FLIP! (f) = true; }                           // C 3967
}

/// C 3971-4010: `static void set_up_x_back_buffer (struct frame *f)` - C's `static`, so a plain
/// `unsafe fn`; C's two `#ifdef`s are the port's gates and C 3999's chained assignment is written
/// where C writes it.
#[cfg (feature = "have_x_windows")]
unsafe fn set_up_x_back_buffer (f: *mut frame)
{
#[cfg (feature = "have_xrender")]
  {
    block_input ();                                                     // C 3975
    if FRAME_X_PICTURE! (f) != None                                     // C 3976
      {
	XRenderFreePicture (FRAME_X_DISPLAY! (f),                       // C 3978-3979
			    FRAME_X_PICTURE! (f));
	FRAME_X_PICTURE! (f) = None;                                    // C 3980
      }
    unblock_input ();                                                   // C 3982
  }

#[cfg (feature = "have_xdbe")]
  {
    block_input ();                                                     // C 3986
    if FRAME_X_WINDOW! (f) != 0 && !FRAME_X_DOUBLE_BUFFERED_P! (f)       // C 3987
      {
#[cfg (feature = "use_cairo")]
	x_cr_destroy_frame_context (f);                                 // C 3990
	FRAME_X_RAW_DRAWABLE! (f) = FRAME_X_WINDOW! (f);                // C 3992
	if (*FRAME_DISPLAY_INFO! (f)).supports_xdbe                     // C 3993
	  {
	    /* If allocating a back buffer fails, either because the
	       server ran out of memory or we don't have the right kind
	       of visual, just use single-buffered rendering.  */
	    x_catch_errors (FRAME_X_DISPLAY! (f));                      // C 3998
	    FRAME_X_RAW_DRAWABLE! (f)                                   // C 3999-4002
	      = XdbeAllocateBackBufferName (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
					    XdbeCopied);
	    if x_had_errors_p (FRAME_X_DISPLAY! (f))                    // C 4003
	      { FRAME_X_RAW_DRAWABLE! (f) = FRAME_X_WINDOW! (f); }      // C 4004
	    x_uncatch_errors_after_check ();                            // C 4005
	  }
      }
    unblock_input ();                                                   // C 4008
  }
}

/// C 4012-4042: `void tear_down_x_back_buffer (struct frame *f)` - not `static`, so exported.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tear_down_x_back_buffer (f: *mut frame)
{
#[cfg (feature = "have_xrender")]
  {
    block_input ();                                                     // C 4016
    if FRAME_X_PICTURE! (f) != None                                     // C 4017
      {
	XRenderFreePicture (FRAME_X_DISPLAY! (f),                       // C 4019-4020
			    FRAME_X_PICTURE! (f));
	FRAME_X_PICTURE! (f) = None;                                    // C 4021
      }
    unblock_input ();                                                   // C 4023
  }

#[cfg (feature = "have_xdbe")]
  {
    block_input ();                                                     // C 4027
    if FRAME_X_WINDOW! (f) != 0 && FRAME_X_DOUBLE_BUFFERED_P! (f)        // C 4028
      {
	if FRAME_X_DOUBLE_BUFFERED_P! (f)                               // C 4030
	  {
#[cfg (feature = "use_cairo")]
	    x_cr_destroy_frame_context (f);                             // C 4033
	    XdbeDeallocateBackBufferName (FRAME_X_DISPLAY! (f),         // C 4035-4036
					  FRAME_X_DRAWABLE! (f));
	    FRAME_X_RAW_DRAWABLE! (f) = FRAME_X_WINDOW! (f);            // C 4037
	  }
      }
    unblock_input ();                                                   // C 4040
  }
}

/* Set up double buffering if the frame parameters don't prohibit
   it.  */

/// C 4046-4055: `void initial_set_up_x_back_buffer (struct frame *f)` - not `static`, so
/// exported; C 4052's `f->param_alist` is the mirror's field and `Qinhibit_double_buffering` is
/// frame.rs's DEFSYM accessor (the `Qnil ()` form).
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initial_set_up_x_back_buffer (f: *mut frame)
{
  eassert! (FRAME_X_WINDOW! (f));                                       // C 4049
  FRAME_X_RAW_DRAWABLE! (f) = FRAME_X_WINDOW! (f);                      // C 4050

  if NILP (CDR (Fassq (Qinhibit_double_buffering (), (*f).param_alist)))  // C 4052-4053
    { set_up_x_back_buffer (f); }                                       // C 4054
}

/// C 4059-4168: `static void setup_xi_event_mask (struct frame *f)`, inside C 4057's
/// `#if defined HAVE_XINPUT2`.  C 4076's `alloca` is the port's stack pool (P-05, the
/// `xi_select_hierarchy_events` shape at `src/xterm.rs` 19884-19890) and the token it leaves
/// behind has its `docs/ports/macro-body-notes.tsv` row; C 4071's
/// `((unsigned char *) selected) + sizeof *selected` is the port's `.offset`.
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe fn setup_xi_event_mask (f: *mut frame)
{
  let mut mask: XIEventMask;                                            // C 4062
  let l: ptrdiff_t = XIMaskLen! (XI_LASTEVENT) as ptrdiff_t;            // C 4063
  let mut m: *mut c_uchar;                                              // C 4064
#[cfg (not (feature = "have_xinput2_1"))]
  let mut selected: *mut XIEventMask;                                   // C 4068

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
#[cfg (not (feature = "have_xinput2_1"))]
  {
    /* Set up fallback values, since XIGetSelectedEvents doesn't work
       with this version of libXi.  */
    selected = xzalloc (std::mem::size_of::<XIEventMask> () + l as usize)  // C 4070
      as *mut XIEventMask;
    (*selected).mask                                                    // C 4071
      = (selected as *mut c_uchar).offset (std::mem::size_of::<XIEventMask> () as isize);
    (*selected).mask_len = l;                                           // C 4072
    (*selected).deviceid = XIAllMasterDevices;                          // C 4073
  }

  m = SAFE_ALLOCA! (sa_avail, l as usize) as *mut c_uchar;              // C 4076 (C's `mask.mask = m = alloca (l)`)
  mask.mask = m;
  memset (m as *mut c_void, 0, l as usize);                             // C 4077
  mask.mask_len = l;                                                    // C 4078

  block_input ();                                                       // C 4080
#[cfg (not (feature = "have_gtk3"))]
  {
    mask.deviceid = XIAllMasterDevices;                                 // C 4082

    XISetMask! (m, XI_ButtonPress);                                     // C 4084
    XISetMask! (m, XI_ButtonRelease);                                   // C 4085
    XISetMask! (m, XI_Motion);                                          // C 4086
    XISetMask! (m, XI_Enter);                                           // C 4087
    XISetMask! (m, XI_Leave);                                           // C 4088
#[cfg (not (feature = "use_gtk"))]
    {
      XISetMask! (m, XI_FocusIn);                                       // C 4090
      XISetMask! (m, XI_FocusOut);                                      // C 4091
      XISetMask! (m, XI_KeyPress);                                      // C 4092
      XISetMask! (m, XI_KeyRelease);                                    // C 4093
    }
#[cfg (feature = "have_xinput2_4")]
    if (*FRAME_DISPLAY_INFO! (f)).xi2_version >= 4                      // C 4096
      {
	/* Select for gesture events.  Since this configuration doesn't
	   use GTK 3, Emacs is the only code that can change the XI
	   event mask, and can safely select for gesture events on
	   master pointers only.  */
	XISetMask! (m, XI_GesturePinchBegin);                           // C 4102
	XISetMask! (m, XI_GesturePinchUpdate);                          // C 4103
	XISetMask! (m, XI_GesturePinchEnd);                             // C 4104
      }
    XISelectEvents (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),          // C 4107-4108
		    &raw mut mask, 1);

    /* Fortunately `xi_masks' isn't used on GTK 3, where we really have
       to get the event mask from the X server.  */
#[cfg (not (feature = "have_xinput2_1"))]
    memcpy ((*selected).mask as *mut c_void, m as *const c_void, l as usize);  // C 4113

    memset (m as *mut c_void, 0, l as usize);                           // C 4116
  }

#[cfg (feature = "use_x_toolkit")]
  {
    XISetMask! (m, XI_KeyPress);                                        // C 4120
    XISetMask! (m, XI_KeyRelease);                                      // C 4121

    XISelectEvents (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),      // C 4123-4124
		    &raw mut mask, 1);
    memset (m as *mut c_void, 0, l as usize);                           // C 4125
  }

#[cfg (feature = "have_xinput2_2")]
  if (*FRAME_DISPLAY_INFO! (f)).xi2_version >= 2                        // C 4129
    {
      /* Select for touch events from all devices.

         Emacs will only process touch events originating
         from slave devices, as master pointers may also
         represent dependent touch devices.  */
      mask.deviceid = XIAllDevices;                                     // C 4136

      XISetMask! (m, XI_TouchBegin);                                    // C 4138
      XISetMask! (m, XI_TouchUpdate);                                   // C 4139
      XISetMask! (m, XI_TouchEnd);                                      // C 4140
      XISetMask! (m, XI_TouchOwnership);                                // C 4141

#[cfg (all (feature = "have_xinput2_4", feature = "use_gtk3"))]
      if (*FRAME_DISPLAY_INFO! (f)).xi2_version >= 4                    // C 4144
	{
	  /* Now select for gesture events from all pointer devices.
	     Emacs will only handle gesture events from the master
	     pointer, but cannot afford to overwrite the event mask
	     set by GDK.  */

	  XISetMask! (m, XI_GesturePinchBegin);                         // C 4151
	  XISetMask! (m, XI_GesturePinchUpdate);                        // C 4152
	  XISetMask! (m, XI_GesturePinchEnd);                           // C 4153
	}

      XISelectEvents (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),        // C 4157-4158
		      &raw mut mask, 1);
    }

#[cfg (not (feature = "have_xinput2_1"))]
  {
    (*FRAME_X_OUTPUT! (f)).xi_masks = selected;                         // C 4163
    (*FRAME_X_OUTPUT! (f)).num_xi_masks = 1;                            // C 4164
  }

  unblock_input ();                                                     // C 4167
  SAFE_FREE! (sa_count, sa_mark);
}

/* C 4430-4621: the two `#else` arms of `x_window` - the GTK one (C 4431-4474) and the plain-X one
   (C 4476-4619).  The Xt arm (C 4172-4428) is outside this slice and opens the C-range table's
   next row.  `create_frame_xic` (C 2740's block) and `append_wm_protocols` (C 1573) are this
   file's own definitions, `setup_xi_event_mask` is slice 5's, and the four Xlib entry points
   neither the file nor the mirror declares stand in the use-site blocks below, each under the
   guard its arm has.  */

/// C: Xutil.h 189-192: `typedef struct { char *res_name; char *res_class; } XClassHint;` - no
/// mirror of this unit's graph declares it, so it stands here with the header's field order
/// (P-88 rule 1); C 4482/4552-4554 write the two fields, and the Xt arm's C 4179/4331-4333 do
/// the same through `XSetClassHint`, so the guard is the whole `have_x_windows` family.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
pub struct XClassHint
{
  pub res_name: *mut c_char,
  pub res_class: *mut c_char,
}

/// C: Xlib.h 304-330: the `XWindowAttributes` record, fields in the header's own order and the
/// header's own types (measured on this machine) - no mirror of this unit's graph declares it,
/// and C 4453/4458 reads `your_event_mask` out of it (P-88 rule 1; the layout is **UNVERIFIED**
/// beyond that field, which is the only one this file touches).
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit")))]
#[repr (C)]
pub struct XWindowAttributes
{
  pub x: c_int, pub y: c_int, pub width: c_int, pub height: c_int, pub border_width: c_int,
  pub depth: c_int, pub visual: *mut Visual, pub root: Window, pub class: c_int,
  pub bit_gravity: c_int, pub win_gravity: c_int, pub backing_store: c_int,
  pub backing_planes: c_ulong, pub backing_pixel: c_ulong, pub save_under: Bool,
  pub colormap: Colormap, pub map_installed: Bool, pub map_state: c_int,
  pub all_event_masks: c_long, pub your_event_mask: c_long, pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool, pub screen: *mut Screen,
}

/// C: Xlib.h 1188: `#define XNFilterEvents "filterEvents"` - the atom name C 4448/4535 hand to
/// `XGetICValues` (measured on this machine's header).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XNFilterEvents: &core::ffi::CStr = c"filterEvents";

/// C: X11/X.h 387, 393, 395, 396, 405, 431, 439 and 568, plus X11/Xutil.h 74, 75, 102 and 134,
/// every value measured on this machine's headers: `InputOutput 1`, `CWBackPixel (1L<<1)`,
/// `CWBorderPixel (1L<<3)`, `CWBitGravity (1L<<4)`, `CWColormap (1L<<13)`, `StaticGravity 10`,
/// `NotUseful 0`, `GCLineWidth (1L<<4)`, `XNegative 0x0010`, `YNegative 0x0020`,
/// `USPosition (1L<<0)` and `InputHint (1L<<0)` - the plain-X arm (C 4486-4562) reads ten of
/// them and the Xt arm four (`XNegative`/`YNegative` at C 4283-4296, `USPosition` at C 4299,
/// `InputHint` at C 4342), so the guard is the whole `have_x_windows` family (the five names
/// `include/xlib.rs`'s `x11_constants` module carries - `CWEventMask`, `CWOverrideRedirect`,
/// `GCForeground`, `GCBackground` and `PropModeReplace` - are read from there instead).
#[cfg (feature = "have_x_windows")]
pub const InputOutput: c_uint = 1;                        // X11/X.h 387
#[cfg (feature = "have_x_windows")]
pub const CWBackPixel: c_long = 1 << 1;                 // X11/X.h 393
#[cfg (feature = "have_x_windows")]
pub const CWBorderPixel: c_long = 1 << 3;               // X11/X.h 395
#[cfg (feature = "have_x_windows")]
pub const CWBitGravity: c_long = 1 << 4;                // X11/X.h 396
#[cfg (feature = "have_x_windows")]
pub const CWColormap: c_long = 1 << 13;                 // X11/X.h 405
#[cfg (feature = "have_x_windows")]
pub const StaticGravity: c_int = 10;                     // X11/X.h 431
#[cfg (feature = "have_x_windows")]
pub const NotUseful: c_int = 0;                          // X11/X.h 439
#[cfg (feature = "have_x_windows")]
pub const XNegative: c_int = 0x0010;                    // X11/Xutil.h 74
#[cfg (feature = "have_x_windows")]
pub const YNegative: c_int = 0x0020;                    // X11/Xutil.h 75
#[cfg (feature = "have_x_windows")]
pub const InputHint: c_long = 1 << 0;                    // X11/Xutil.h 134
#[cfg (feature = "have_x_windows")]
pub const USPosition: c_long = 1 << 0;                   // X11/Xutil.h 102

/* C: Xlib.h 2995-3003: `extern Window XCreateWindow (Display *, Window, int, int, unsigned int,
   unsigned int, unsigned int, int, unsigned int, Visual *, unsigned long,
   XSetWindowAttributes *);` - C 4512-4521's call.
   C: Xutil.h 642: `extern void XSetClassHint (Display *, Window, XClassHint *);` - C 4554's.
   C: Xlib.h 2896: `extern Status XSetWMProtocols (Display *, Window, Atom *, int);` - C 4572's.
   C: Xlib.h 1424: `extern Status XGetWindowAttributes (Display *, Window, XWindowAttributes *);`
   - C 4456-4457's.  Block comments, because a doc comment on an `extern` block is the
   `unused doc comment` warning the file's baseline already counts (slice 5's note).
   C: Xutil.h 642: `extern void XSetClassHint (Display *, Window, XClassHint *);` is not
   here: both `x_window` arms call it (C 4333 and C 4554) and the other three entry points
   name the plain-X-only records, so it stands in its own block below.  */
/* The gate widened from `not (use_x_toolkit)` to the file gate in slice 34: the tip-frame
   body's C 8609 calls `XCreateWindow` in every arm, not only the plain-X one.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XCreateWindow (dpy: *mut Display, parent: Window, x: c_int, y: c_int, width: c_uint,
			height: c_uint, border_width: c_uint, depth: c_int, class: c_uint,
			visual: *mut Visual, valuemask: c_ulong,
			attributes: *mut XSetWindowAttributes) -> Window;
  pub fn XSetWMProtocols (dpy: *mut Display, w: Window, protocols: *mut Atom, count: c_int)
			  -> c_int;
  pub fn XGetWindowAttributes (dpy: *mut Display, w: Window, attr: *mut XWindowAttributes)
			       -> c_int;
}

/* C: Xutil.h 642: `extern void XSetClassHint (Display *, Window, XClassHint *);` - C 4333 (the
   Xt arm) and C 4554 (the plain-X arm) both call it, and it names `XClassHint` alone, so it
   stands under the whole family's guard rather than in the plain-X-only block above.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XSetClassHint (dpy: *mut Display, w: Window, class_hints: *mut XClassHint) -> c_int;
}

/* C: src/gtkutil.h 78: `extern bool xg_create_frame_widgets (struct frame *f);` - C 4435's call.
   (gtkutil.rs defines it; this unit declares it at the use site, P-72 rule 4.)  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"), feature = "use_gtk"))]
unsafe extern "C" {
  pub fn xg_create_frame_widgets (f: *mut frame) -> bool;
}

/// C 4432-4474: `static void x_window (struct frame *f)`, the `USE_GTK` arm of C 4430's
/// `#else /* not USE_X_TOOLKIT */`.  C 4470-4473's `HAVE_XINPUT2` pair calls slice 5's
/// `setup_xi_event_mask`; `XNFilterEvents` is Xlib.h 3895's atom name and `NoEventMask`/`CWEventMask`
/// are the mirror's `x11_constants` members.
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"), feature = "use_gtk"))]
unsafe fn x_window (f: *mut frame)
{
  use crate::x11_constants::{CWEventMask, NoEventMask};

  let res: bool = xg_create_frame_widgets (f);                          // C 4435
  if res == false
    { error (c"Unable to create window".as_ptr ()); }                   // C 4436

#[cfg (feature = "have_x_i18n")]
  {
    FRAME_XIC! (f) = std::ptr::null_mut ();                             // C 4439
    if (*FRAME_DISPLAY_INFO! (f)).use_xim                              // C 4440
      {
	block_input ();                                                // C 4442
	create_frame_xic (f);                                          // C 4443
	if !FRAME_XIC! (f).is_null ()                                  // C 4444
	  {
	    /* XIM server might require some X events. */
	    let mut fevent: c_long = NoEventMask;                      // C 4447 (C: unsigned long)
	    XGetICValues (FRAME_XIC! (f), XNFilterEvents.as_ptr (), &raw mut fevent,
			  std::ptr::null_mut::<c_void> ());            // C 4448

	    if fevent != NoEventMask                                   // C 4450
	      {
		let mut attributes: XSetWindowAttributes = std::mem::zeroed ();   // C 4452
		let mut wattr: XWindowAttributes = std::mem::zeroed ();           // C 4453
		let attribute_mask: c_long;                            // C 4454 (C: unsigned long; the mirror's CW* words are c_long)

		XGetWindowAttributes (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
				      &raw mut wattr);                     // C 4456-4457
		attributes.event_mask = wattr.your_event_mask | fevent;    // C 4458
		attribute_mask = CWEventMask;                          // C 4459
		XChangeWindowAttributes (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
					 attribute_mask as c_ulong, &raw mut attributes);  // C 4460-4461
	      }
	  }
	unblock_input ();                                              // C 4464
      }
  }

  append_wm_protocols (FRAME_DISPLAY_INFO! (f), f);                    // C 4468

#[cfg (feature = "have_xinput2")]
  if (*FRAME_DISPLAY_INFO! (f)).supports_xi2                           // C 4471
    { setup_xi_event_mask (f); }                                       // C 4472
}

/* Create and set up the X window for frame F.  */

/// C 4479-4619: `static void x_window (struct frame *f)`, the plain-X arm (C 4476's
/// `#else /*! USE_GTK */`).  `STANDARD_EVENT_SET` is the mirror's own constant (xterm.h 146),
/// `XNegative`/`YNegative` are Xlib's `Xutil.h 417-418`, and the `MWM_*` trio plus
/// `PropMotifWmHints` are this file's C 137-170 block, already carried by slice 1.
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
unsafe fn x_window (f: *mut frame)
{
  use crate::x11_constants::{CWEventMask, CWOverrideRedirect, GCForeground, GCBackground,
			     PropModeReplace, NoEventMask};

  let mut class_hints: XClassHint = std::mem::zeroed ();               // C 4482 (the fields at C 4552-4553)
  let mut attributes: XSetWindowAttributes = std::mem::zeroed ();       // C 4483 (the fields at C 4486-4541)
  let mut attribute_mask: c_long;                                      // C 4484 (C: unsigned long; the mirror's CW* words are c_long)

  attributes.background_pixel = FRAME_BACKGROUND_PIXEL! (f);           // C 4486
  attributes.border_pixel = (*((*f).output_data.x as *mut x_output)).border_pixel;       // C 4487
  attributes.bit_gravity = StaticGravity;                              // C 4488
  attributes.backing_store = NotUseful;                                // C 4489
  attributes.save_under = True;                                        // C 4490
  attributes.event_mask = STANDARD_EVENT_SET;                          // C 4491
  attributes.colormap = FRAME_X_COLORMAP! (f);                         // C 4492
  attributes.override_redirect = FRAME_OVERRIDE_REDIRECT! (f);         // C 4493
  attribute_mask = CWBackPixel | CWBorderPixel | CWBitGravity | CWEventMask  // C 4494-4495
    | CWOverrideRedirect | CWColormap;

  if !FRAME_PARENT_FRAME (f).is_null ()                                // C 4497
    {
      if (*f).size_hint_flags & XNegative != 0                          // C 4499
	{ (*f).left_pos = FRAME_PIXEL_WIDTH (FRAME_PARENT_FRAME (f))  // C 4500-4501
	    - FRAME_PIXEL_WIDTH (f) + (*f).left_pos; }

      if (*f).size_hint_flags & YNegative != 0                          // C 4503
	{ (*f).top_pos = FRAME_PIXEL_HEIGHT (FRAME_PARENT_FRAME (f))  // C 4504-4505
	    - FRAME_PIXEL_HEIGHT (f) + (*f).top_pos; }

      (*f).size_hint_flags &= !(XNegative | YNegative);                 // C 4507
    }

  block_input ();                                                       // C 4510
  FRAME_X_WINDOW! (f)                                                   // C 4511-4521
    = XCreateWindow (FRAME_X_DISPLAY! (f), (*FRAME_DISPLAY_INFO! (f)).root_window,
		     (*f).left_pos, (*f).top_pos, FRAME_PIXEL_WIDTH (f) as c_uint,
		     FRAME_PIXEL_HEIGHT (f) as c_uint, (*f).border_width as c_uint,
		     (*FRAME_DISPLAY_INFO! (f)).n_planes, InputOutput,
		     FRAME_X_VISUAL! (f), attribute_mask as c_ulong, &raw mut attributes);
  initial_set_up_x_back_buffer (f);                                     // C 4522

#[cfg (feature = "have_x_i18n")]
  {
    if (*FRAME_DISPLAY_INFO! (f)).use_xim                               // C 4525
      {
	create_frame_xic (f);                                           // C 4527
	if !FRAME_XIC! (f).is_null ()                                   // C 4528
	  {
	    /* XIM server might require some X events. */
	    let mut fevent: c_ulong = NoEventMask;                      // C 4531

	    if fevent != 0                                              // C 4533
	      {
		XGetICValues (FRAME_XIC! (f), XNFilterEvents.as_ptr (), &raw mut fevent,
			      std::ptr::null_mut::<c_void> ());             // C 4535
		attributes.event_mask |= fevent;                        // C 4536
		attribute_mask = CWEventMask;                           // C 4537
		XChangeWindowAttributes (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
					 attribute_mask as c_ulong, &raw mut attributes);  // C 4538-4539
	      }
	  }
      }
  }

#[cfg (feature = "have_xinput2")]
  if (*FRAME_DISPLAY_INFO! (f)).supports_xi2                            // C 4546
    { setup_xi_event_mask (f); }                                        // C 4547

  validate_x_resource_name ();                                          // C 4550

  class_hints.res_name = SSDATA (Vx_resource_name);                     // C 4552
  class_hints.res_class = SSDATA (Vx_resource_class);                   // C 4553
  XSetClassHint (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), &raw mut class_hints);  // C 4554

  /* This indicates that we use the "Passive Input" input model.
     Unless we do this, we don't get the Focus{In,Out} events that we
     need to draw the cursor correctly.  Accursed bureaucrats.
   XWhipsAndChains (FRAME_X_DISPLAY (f), IronMaiden, &TheRack);  */

  (*((*f).output_data.x as *mut x_output)).wm_hints.input = True;                         // C 4561
  (*((*f).output_data.x as *mut x_output)).wm_hints.flags |= InputHint;                   // C 4562
  XSetWMHints (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),               // C 4563-4564
	       &raw mut (*((*f).output_data.x as *mut x_output)).wm_hints);
  (*((*f).output_data.x as *mut x_output)).wm_hints.icon_pixmap = 0;  /* C's `None` */    // C 4565

  /* Request "save yourself" and "delete window" commands from wm.  */
  {
    let mut protocols: [Atom; 2] = std::mem::zeroed ();                  // C 4569 (the two members at C 4573-4578)
    protocols[0] = (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_delete_window;   // C 4570
    protocols[1] = (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_save_yourself;    // C 4571
    XSetWMProtocols (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), protocols.as_mut_ptr (), 2);  // C 4572
  }

  append_wm_protocols (FRAME_DISPLAY_INFO! (f), f);                     // C 4575

  /* x_set_name normally ignores requests to set the name if the
     requested name is the same as the current name.  This is the one
     place where that assumption isn't correct; f->name is set, but
     the X server hasn't been told.  */
  {
    let name: Lisp_Object;                                              // C 4582
    /* C's `f->explicit_name` is one of frame.h 417's bitfields; the mirror reads and writes
       those through `frame_flag`/`frame_set_flag` with the target's own unit/shift pair.  */
    let explicit: bool                                                  // C 4583
      = frame_flag (f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT);

    frame_set_flag (f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT, 0);  // C 4585
    name = (*f).name;                                                   // C 4586
    fset_name (f, Qnil ());                                             // C 4587
    x_set_name (f, name, explicit);                                     // C 4588
  }

  if FRAME_UNDECORATED! (f) != 0                                        // C 4591
    {
      let dpy: *mut Display = FRAME_X_DISPLAY! (f);                     // C 4593
      let mut hints: PropMotifWmHints = std::mem::zeroed ();            // C 4594
      let prop: Atom = (*FRAME_DISPLAY_INFO! (f)).Xatom_MOTIF_WM_HINTS;  // C 4595

      memset (&raw mut hints as *mut c_void, 0, std::mem::size_of::<PropMotifWmHints> ());  // C 4597
      hints.flags = MWM_HINTS_DECORATIONS as c_ulong;                   // C 4598
      hints.decorations = 0;                                            // C 4599

      /* For some reason the third and fourth arguments in the following
	 call must be identical: ...  */
      XChangeProperty (dpy, FRAME_OUTER_WINDOW! (f), prop, prop, 32, PropModeReplace,
		       &raw mut hints as *mut c_uchar,
		       PROP_MOTIF_WM_HINTS_ELEMENTS);                       // C 4605-4607
    }


  /* C passes an assignment expression; Rust has no such expression, so the assignment is its
     own statement and the value it leaves is passed (PORTING.md 2.2's statement class).  */
  (*((*f).output_data.x as *mut x_output)).current_cursor                 // C 4612-4613
    = (*((*f).output_data.x as *mut x_output)).text_cursor;
  XDefineCursor (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),              // C 4611
		 (*((*f).output_data.x as *mut x_output)).current_cursor);

  unblock_input ();                                                     // C 4615

  if FRAME_X_WINDOW! (f) == 0                                           // C 4617
    { error (c"Unable to create window".as_ptr ()); }                   // C 4618
}

/* C 4624-4789: the four definitions between this slice's `x_window` halves and
   `unwind_create_frame` - `x_icon_verify` (C 4626-4642), `x_icon` (C 4648-4691), `x_make_gc`
   (C 4697-4745) and `x_free_gcs` (C 4750-4782, the only one of the four C does not declare
   `static`).  `gray_bits`, `gray_width` and `gray_height` are `src/bitmaps/gray.xbm`'s
   data-file constants - the gap this file's own header note (97-98) and `docs/ports/xfns.md`
   already record - so they are read bare and the arms' `E0425` rows count them.  */

/* C: frame.h 1886: `extern void x_wm_set_icon_position (struct frame *, int, int);` - C 4672's.
   C: frame.h 1651-1655: `extern Lisp_Object gui_frame_get_and_record_arg (struct frame *,
   Lisp_Object, Lisp_Object, const char *, const char *, enum resource_types);` - C 4633/4634 and
   C 4654/4656's.
   C: Xlib.h 2258: `extern int DefaultDepth (Display *, int);` - C 4742's.
   `check_integer_range` (C 4665/4666), `XCreateGC` (C 4711), `XFreeGC` (C 4759) and `XFreePixmap`
   (C 4777) are already declared by mirrors in this unit's include graph
   (`include/lisp.rs` and `include/xlib.rs`), so nothing of those four is repeated here
   (`check-mirror-dupes.py` is the gate that says so).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {

  pub fn gui_frame_get_and_record_arg (f: *mut frame, alist: Lisp_Object, param: Lisp_Object,
				       attribute: *const c_char, class: *const c_char,
				       r#type: resource_types) -> Lisp_Object;
  pub fn DefaultDepth (dpy: *mut Display, screen_number: c_int) -> c_int;
  /// C: Xlib.h 2507-2509: `extern Pixmap XCreatePixmapFromBitmapData (Display *, Drawable, char *,
  /// unsigned int, unsigned int, unsigned long, unsigned long, unsigned int);` - C 4737-4742's.
  pub fn XCreatePixmapFromBitmapData (dpy: *mut Display, d: Drawable, data: *mut c_char,
				      width: c_uint, height: c_uint, fg: c_ulong, bg: c_ulong,
				      depth: c_uint) -> Pixmap;
}

/// C: X11/X.h 568: `#define GCLineWidth (1L<<4)` - read by `x_make_gc`, which every arm
/// compiles, so it stands under the file gate alone (measured on this machine's header).
#[cfg (feature = "have_x_windows")]
pub const GCLineWidth: c_int = 1 << 4;

/// C: Xlib.h 177-203: the `XGCValues` record, fields in the header's own order and with the
/// header's own types (measured on this machine); C 4707-4731 writes `foreground`, `background`
/// and `line_width` and hands the record to `XCreateGC` (P-88 rule 1; the layout is
/// **UNVERIFIED** beyond those three fields).
#[cfg (feature = "have_x_windows")]
#[repr (C)]
pub struct XGCValues
{
  pub function: c_int, pub plane_mask: c_ulong, pub foreground: c_ulong, pub background: c_ulong,
  pub line_width: c_int, pub line_style: c_int, pub cap_style: c_int, pub join_style: c_int,
  pub fill_style: c_int, pub fill_rule: c_int, pub arc_mode: c_int, pub tile: Pixmap,
  pub stipple: Pixmap, pub ts_x_origin: c_int, pub ts_y_origin: c_int, pub font: Font,
  pub subwindow_mode: c_int, pub graphics_exposures: Bool, pub clip_x_origin: c_int,
  pub clip_y_origin: c_int, pub clip_mask: Pixmap, pub dash_offset: c_int, pub dashes: c_char,
}

/// C 4626-4642: `static void x_icon_verify (struct frame *f, Lisp_Object parms)`.
#[cfg (feature = "have_x_windows")]
unsafe fn x_icon_verify (f: *mut frame, parms: Lisp_Object)
{
  let icon_x: Lisp_Object;                                              // C 4629
  let icon_y: Lisp_Object;                                              // C 4629

  /* Set the position of the icon.  Note that twm groups all
     icons in an icon window.  */
  icon_x = gui_frame_get_and_record_arg (f, parms, Qicon_left (),       // C 4633
					 0 as *const c_char,
					 0 as *const c_char, RES_TYPE_NUMBER);
  icon_y = gui_frame_get_and_record_arg (f, parms, Qicon_top (),        // C 4634
					 0 as *const c_char,
					 0 as *const c_char, RES_TYPE_NUMBER);
  if !BASE_EQ (icon_x, Qunbound ()) && !BASE_EQ (icon_y, Qunbound ())   // C 4635
    {
      CHECK_FIXNUM (icon_x);                                            // C 4637
      CHECK_FIXNUM (icon_y);                                            // C 4638
    }
  else if !BASE_EQ (icon_x, Qunbound ()) || !BASE_EQ (icon_y, Qunbound ())  // C 4640
    { error (c"Both left and top icon corners of icon must be specified".as_ptr ()); }  // C 4641
}

/* Handle the icon stuff for this window.  Perhaps later we might
   want an x_set_icon_position which can be called interactively as
   well.  */

/// C 4648-4691: `static void x_icon (struct frame *f, Lisp_Object parms)`.  C 4674-4684's
/// `#if false` block (the `x_wm_set_window_state` arm) is C's own dead text; the port carries it
/// as the comment below rather than inventing a `cfg` for it.
#[cfg (feature = "have_x_windows")]
unsafe fn x_icon (f: *mut frame, parms: Lisp_Object)
{
  /* Set the position of the icon.  Note that twm groups all
     icons in an icon window.  */
  let icon_x: Lisp_Object                                              // C 4653-4654
    = gui_frame_get_and_record_arg (f, parms, Qicon_left (), std::ptr::null::<c_char> (),
				    std::ptr::null::<c_char> (), RES_TYPE_NUMBER);
  let icon_y: Lisp_Object                                              // C 4655-4656
    = gui_frame_get_and_record_arg (f, parms, Qicon_top (), std::ptr::null::<c_char> (),
				    std::ptr::null::<c_char> (), RES_TYPE_NUMBER);
  let mut icon_xval: c_int = 0;                                         // C 4657 (Deviation 12)
  let mut icon_yval: c_int = 0;                                         // C 4657 (Deviation 12)

  let xgiven: bool = !BASE_EQ (icon_x, Qunbound ());                    // C 4659
  let ygiven: bool = !BASE_EQ (icon_y, Qunbound ());                    // C 4660
  if xgiven != ygiven                                                   // C 4661
    { error (c"Both left and top icon corners of icon must be specified".as_ptr ()); }  // C 4662
  if xgiven                                                             // C 4663
    {
      icon_xval = check_integer_range (icon_x, INT_MIN as intmax_t,  // C 4665
				       INT_MAX as intmax_t) as c_int;
      icon_yval = check_integer_range (icon_y, INT_MIN as intmax_t,  // C 4666
				       INT_MAX as intmax_t) as c_int;
    }

  block_input ();                                                       // C 4669

  if xgiven                                                             // C 4671
    { x_wm_set_icon_position (f, icon_xval, icon_yval); }               // C 4672

  /* C 4674-4684: `#if false` - the `x_wm_set_window_state (f, EQ (gui_display_get_arg (…)) ? IconicState
     : NormalState)` arm, which C's own comment says `gui_display_get_arg` makes unusable.  */

  x_text_icon (f, SSDATA (if !NILP ((*f).icon_name)                   // C 4686-4688
			  { (*f).icon_name }
			else
			  { (*f).name }));

  unblock_input ();                                                     // C 4690
}

/* Make the GCs needed for this window, setting the
   background, border and mouse colors; also create the
   mouse cursor and the gray border tile.  */

/// C 4697-4745: `static void x_make_gc (struct frame *f)`.  C 4700's `XGCValues gc_values;` is
/// C's uninitialised local (Deviation 12's zeroed form), and the four `f->output_data.x->…`
/// fields are the mirror's `x_output` members.
#[cfg (feature = "have_x_windows")]
unsafe fn x_make_gc (f: *mut frame)
{
  use crate::x11_constants::{GCBackground, GCForeground};

  let mut gc_values: XGCValues = std::mem::zeroed ();                   // C 4700 (Deviation 12)

  block_input ();                                                       // C 4702

  /* Create the GCs of this frame.
     Note that many default values are used.  */

  gc_values.foreground = FRAME_FOREGROUND_PIXEL (f);                    // C 4707
  gc_values.background = FRAME_BACKGROUND_PIXEL! (f);                   // C 4708
  gc_values.line_width = 1;                                             // C 4709
  (*((*f).output_data.x as *mut x_output)).normal_gc                    // C 4710-4714
    = XCreateGC (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
		 (GCLineWidth | GCForeground | GCBackground) as c_ulong, &raw mut gc_values as *mut c_void);

  /* Reverse video style.  */
  gc_values.foreground = FRAME_BACKGROUND_PIXEL! (f);                   // C 4717
  gc_values.background = FRAME_FOREGROUND_PIXEL (f);                    // C 4718
  (*((*f).output_data.x as *mut x_output)).reverse_gc                   // C 4719-4723
    = XCreateGC (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
		 (GCForeground | GCBackground | GCLineWidth) as c_ulong, &raw mut gc_values as *mut c_void);

  /* Cursor has cursor-color background, background-color foreground.  */
  gc_values.foreground = FRAME_BACKGROUND_PIXEL! (f);                   // C 4726
  gc_values.background = (*((*f).output_data.x as *mut x_output)).cursor_pixel;  // C 4727
  (*((*f).output_data.x as *mut x_output)).cursor_gc                    // C 4728-4731
    = XCreateGC (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
		 (GCForeground | GCBackground | GCLineWidth) as c_ulong, &raw mut gc_values as *mut c_void);

  /* Create the gray border tile used when the pointer is not in
     the frame.  Since this depends on the frame's pixel values,
     this must be done on a per-frame basis.  */
  (*((*f).output_data.x as *mut x_output)).border_tile                  // C 4736-4742
    = XCreatePixmapFromBitmapData (FRAME_X_DISPLAY! (f),
				   (*FRAME_DISPLAY_INFO! (f)).root_window,
				   gray_bits.as_ptr (), gray_width, gray_height,
				   FRAME_FOREGROUND_PIXEL (f), FRAME_BACKGROUND_PIXEL! (f),
				   DefaultDepth (FRAME_X_DISPLAY! (f),
						 FRAME_X_SCREEN_NUMBER! (f)));

  unblock_input ();                                                     // C 4744
}

/* Free what was allocated in x_make_gc.  */

/// C 4750-4782: `void x_free_gcs (struct frame *f)` - no `static`, so exported.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_free_gcs (f: *mut frame)
{
  let dpy: *mut Display = FRAME_X_DISPLAY! (f);                         // C 4753

  block_input ();                                                       // C 4755

  if !(*((*f).output_data.x as *mut x_output)).normal_gc.is_null ()     // C 4757
    {
      XFreeGC (dpy, (*((*f).output_data.x as *mut x_output)).normal_gc);  // C 4759
      (*((*f).output_data.x as *mut x_output)).normal_gc = std::ptr::null_mut ();  // C 4760
    }

  if !(*((*f).output_data.x as *mut x_output)).reverse_gc.is_null ()    // C 4763
    {
      XFreeGC (dpy, (*((*f).output_data.x as *mut x_output)).reverse_gc);  // C 4765
      (*((*f).output_data.x as *mut x_output)).reverse_gc = std::ptr::null_mut ();  // C 4766
    }

  if !(*((*f).output_data.x as *mut x_output)).cursor_gc.is_null ()     // C 4769
    {
      XFreeGC (dpy, (*((*f).output_data.x as *mut x_output)).cursor_gc);  // C 4771
      (*((*f).output_data.x as *mut x_output)).cursor_gc = std::ptr::null_mut ();  // C 4772
    }

  if (*((*f).output_data.x as *mut x_output)).border_tile != 0          // C 4775
    {
      XFreePixmap (dpy, (*((*f).output_data.x as *mut x_output)).border_tile);  // C 4777
      (*((*f).output_data.x as *mut x_output)).border_tile = 0;         // C 4778
    }

  unblock_input ();                                                     // C 4781
}

/* C 4785-4916: the comment that documents the unwind handler, `unwind_create_frame`
   (C 4789-4814), `do_unwind_create_frame` (C 4816-4820), `x_default_font_parameter`
   (C 4822-4882), the file's first `DEFUN` - `x-wm-set-size-hint` (C 4885-4898) - and
   `set_machine_and_pid_properties` (C 4900-4916).  Everything between slice 7's `x_free_gcs`
   and the giant `Fx_create_frame` at C 4918.  The names the mirrors of this unit's include graph
   do not carry stand in the use-site block below (P-72 rule 4).  */

/* C: lisp.h 3699: `extern Lisp_Object Fmemq (Lisp_Object, Lisp_Object);` - C 4801's.
   C: dispextern.h 3902: `extern void free_glyphs (struct frame *);` - C 4804's.
   C: dispextern.h 4017-4020: `extern void gui_default_parameter (struct frame *, Lisp_Object,
   Lisp_Object, Lisp_Object, const char *, const char *, enum resource_types);` - C 4881's.
   C: font.h 883: `extern Lisp_Object font_open_by_name (struct frame *, Lisp_Object);` - C 4839/4872's.
   C: xterm.h 1782-1784: `extern void x_wm_set_size_hint (struct frame *, long, bool);` - C 4895's.
   C: Xlib.h 2300-2303: `extern void XSetWMProperties (Display *, Window, XTextProperty *,
   XTextProperty *, char **, int, XSizeHints *, XWMHints *, XClassHint *);` - C 4904's.
   Seven other names this slice reads - `XFRAME`, `Vframe_list`, `Fmemq`,
   `gui_display_get_arg`, `decode_window_system_frame`, `xsettings_get_system_font` and `getpid` -
   are already declared by this unit or by a mirror in its include graph, so nothing of those
   seven is repeated (`E0428` from the compiler, a `check-mirror-dupes` failure from the gate).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn free_glyphs (f: *mut frame);
  pub fn x_free_frame_resources (f: *mut frame);
  pub fn gui_default_parameter (f: *mut frame, alist: Lisp_Object, prop: Lisp_Object,
				default_value: Lisp_Object, attribute: *const c_char,
				class: *const c_char, r#type: resource_types);
  pub fn font_open_by_name (f: *mut frame, name: Lisp_Object) -> Lisp_Object;
  pub fn x_wm_set_size_hint (f: *mut frame, flags: c_long, user_position: bool);
  pub fn XSetWMProperties (dpy: *mut Display, w: Window, window_name: *mut XTextProperty,
			   icon_name: *mut XTextProperty, argv: *mut *mut c_char, argc: c_int,
			   normal_hints: *mut XSizeHints, wm_hints: *mut XWMHints,
			   class_hints: *mut XClassHint) -> c_int;
}

/* Handler for signals raised during x_create_frame and
   Fx_create_tip_frame.  FRAME is the frame which is partially
   constructed.  */

/// C 4789-4814: `static Lisp_Object unwind_create_frame (Lisp_Object frame)` - C's `static`, so a
/// plain `unsafe fn`.  C 4797's `FRAME_LIVE_P` is frame.h 1097's `((f)->terminal != 0)` written as
/// the field test the mirror's `frame` has, and C 4805-4809's `GLYPH_DEBUG && ENABLE_CHECKING`
/// arm is the port's pair of feature gates.
#[cfg (feature = "have_x_windows")]
unsafe fn unwind_create_frame (frame: Lisp_Object) -> Lisp_Object
{
  let f: *mut frame = XFRAME (frame);                                   // C 4792

  /* If frame is already dead, nothing to do.  This can happen if the
     display is disconnected after the frame has become official, but
     before Fx_create_frame removes the unwind protect.  */
  if !FRAME_LIVE_P (f)                                                  // C 4797
    { return Qnil (); }                                                 // C 4798

  /* If frame is ``official'', nothing to do.  */
  if NILP (Fmemq (frame, Vframe_list))                                  // C 4801
    {
      x_free_frame_resources (f);                                       // C 4803
      free_glyphs (f);                                                  // C 4804
#[cfg (all (feature = "glyph_debug", feature = "enable_checking"))]
      {
	/* Check that reference counts are indeed correct.  */
	let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);      // C 4807
	eassert! ((*dpyinfo).reference_count == dpyinfo_refcount);       // C 4808
      }
      return Qt ();                                                     // C 4810
    }

  Qnil ()                                                               // C 4813
}

/// C 4816-4820: `static void do_unwind_create_frame (Lisp_Object frame)` - the handler C's
/// `record_unwind_protect` takes.
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn do_unwind_create_frame (frame: Lisp_Object)
{
  unwind_create_frame (frame);                                          // C 4819
}

/* C 4851-4867's `names[]`: C's `#if defined USE_CAIRO || defined HAVE_XFT` is one *element* of
   the initialiser, and an attribute cannot sit on an array element, so the rows live in this
   macro and each of the two arms instantiates it (the `def_x_atom_refs!` shape, `src/xterm.rs`'s
   slice 108).  It is a block comment because a doc comment cannot sit on a `macro_rules!`
   without the `unused doc comment` warning this file's baseline already counts.  */
#[cfg (feature = "have_x_windows")]
macro_rules! default_font_names {
  ($($xft:expr),*) => {
    [ $($xft,)*
      c"-adobe-courier-medium-r-*-*-*-120-*-*-*-*-iso8859-1".as_ptr () as *mut c_char,    // C 4857
      c"-misc-fixed-medium-r-normal-*-*-140-*-*-c-*-iso8859-1".as_ptr () as *mut c_char,  // C 4858
      c"-*-*-medium-r-normal-*-*-140-*-*-c-*-iso8859-1".as_ptr () as *mut c_char,         // C 4859
      c"-*-*-medium-r-*-*-*-*-*-*-c-*-iso8859-1".as_ptr () as *mut c_char,                // C 4862
      c"-*-fixed-*-*-*-*-*-140-*-*-c-*-iso8859-1".as_ptr () as *mut c_char,               // C 4865
      c"fixed".as_ptr () as *mut c_char,                                                 // C 4866
      std::ptr::null_mut () ] };
}

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_default_font_parameter (f: *mut frame, parms: Lisp_Object)
{
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);           // C 4825
  let mut font_param: Lisp_Object                                       // C 4826-4827
    = gui_display_get_arg (dpyinfo, parms, Qfont (), std::ptr::null (),
			   std::ptr::null (), RES_TYPE_STRING);
  let mut font: Lisp_Object = Qnil ();                                  // C 4828
  if BASE_EQ (font_param, Qunbound ())                                  // C 4829
    { font_param = Qnil (); }                                           // C 4830

  if NILP (font_param)                                                  // C 4832
    {
      /* System font should take precedence over X resources.  We suggest this
	 regardless of font-use-system-font because .emacs may not have been
	 read yet.  */
      let system_font: *const c_char = xsettings_get_system_font ();    // C 4837
      if !system_font.is_null ()                                        // C 4838
	{ font = font_open_by_name (f, build_unibyte_string (system_font)); }  // C 4839
    }

  if NILP (font)                                                        // C 4842
    { font = if !NILP (font_param)                                      // C 4843-4847
	{ font_param }
      else
	{ gui_display_get_arg (dpyinfo, parms, Qfont (), c"font".as_ptr (),
			       c"Font".as_ptr (), RES_TYPE_STRING) }; }

  if !FONTP (font) && !STRINGP (font)                                   // C 4849
    {
      #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
      let names: [*mut c_char; 8] = default_font_names! (c"monospace-10".as_ptr () as *mut c_char);  // C 4855
      #[cfg (not (any (feature = "use_cairo", feature = "have_xft")))]
      let names: [*mut c_char; 7] = default_font_names! ();             // C 4851-4867
      let mut i: c_int;                                                 // C 4868

      i = 0;
      while !names[i as usize].is_null ()                               // C 4870
	{
	  font = font_open_by_name (f, build_unibyte_string (names[i as usize]));  // C 4872
	  if !NILP (font)                                               // C 4873
	    { break; }                                                  // C 4874
	  i += 1;
	}
      if NILP (font)                                                    // C 4876
	{ error (c"No suitable font was found".as_ptr ()); }            // C 4877
    }

  /* This call will make X resources override any system font setting.  */
  gui_default_parameter (f, parms, Qfont (), font, c"font".as_ptr (),    // C 4881
			 c"Font".as_ptr (), RES_TYPE_STRING);
}

/* C 4885-4898: `DEFUN ("x-wm-set-size-hint", Fx_wm_set_size_hint, Sx_wm_set_size_hint, 0, 1, 0,
   doc: …)` - this file's first subr, whose registration belongs to the still-missing
   `syms_of_xfns` (`--check`'s `missing defsubr` class); a macro invocation carries no doc
   comment, so the note is a block comment.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-wm-set-size-hint", Fx_wm_set_size_hint, Sx_wm_set_size_hint, 0, 1, 0,
       doc: "Send the size hints for frame FRAME to the window manager.\nIf FRAME is omitted or nil, use the selected frame.\nSignal error if FRAME is not an X frame.",
       (frame: Lisp_Object),
       {
  let f: *mut frame = decode_window_system_frame (frame);               // C 4892

  block_input ();                                                       // C 4894
  x_wm_set_size_hint (f, 0, false);                                     // C 4895
  unblock_input ();                                                     // C 4896
  Qnil ()                                                               // C 4897
});

/// C 4900-4916: `static void set_machine_and_pid_properties (struct frame *f)`.
#[cfg (feature = "have_x_windows")]
unsafe fn set_machine_and_pid_properties (f: *mut frame)
{
  use crate::x11_constants::{PropModeReplace, XA_CARDINAL};

  /* This will set WM_CLIENT_MACHINE and WM_LOCALE_NAME.  */
  XSetWMProperties (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),       // C 4904-4905
		    std::ptr::null_mut (), std::ptr::null_mut (), std::ptr::null_mut (),
		    0, std::ptr::null_mut (), std::ptr::null_mut (), std::ptr::null_mut ());
  let pid: pid_t = getpid ();                                           // C 4906
  if pid as c_uint <= 0xffff_ffff                                       // C 4907
    {
      let mut xpid: c_ulong = pid as c_ulong;                           // C 4909
      XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),   // C 4910-4914
		       (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_pid,
		       XA_CARDINAL as Atom, 32, PropModeReplace,
		       &raw mut xpid as *mut c_uchar, 1);
    }
}

/* C 4172-4428: the `USE_X_TOOLKIT` arm of `x_window` (C 4176 the `static void`, C 4177 the name) -
   the third and last body of that function, and the one slice 6 recorded as the C-range table's
   next-item row.  It is the only arm that builds its widgets through libXt's `XtSetArg`
   vocabulary, so the resource names and the entry points no declaration of this unit or its
   mirrors carries stand in the blocks below (P-72 rule 4).  */

/// C: X11/StringDefs.h's resource names C 4198-4236 and C 4316-4320 hand to `XtSetArg`.  This
/// file's own shape for them (its `XtNinput`/`XtNborderColor` pair at 1382/1384): a function
/// returning `String`, which is Xt's `char *`.  The six names `src/widget.h` itself defines -
/// `XtNshowGrip`, `XtNallowResize`, `XtNresizeToPreferred`, `XtNemacsFrame`,
/// `XtNinternalBorderWidth` and `XtNgeometry` - do **not** stand here: this unit includes that
/// header's mirror (`include/widget.rs` 35/105/112/120/124/128), so the call sites read the
/// mirror's own `&CStr` rows (`XtNgeometry.as_ptr () as String`) - `check-mirror-dupes.py`'s
/// rule that a name a mirror in the include graph carries is not declared a second time.
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNallowShellResize () -> String { c"allowShellResize".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNmappedWhenManaged () -> String { c"mappedWhenManaged".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNborderWidth () -> String { c"borderWidth".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNvisual () -> String { c"visual".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNdepth () -> String { c"depth".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNcolormap () -> String { c"colormap".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNx () -> String { c"x".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub fn XtNy () -> String { c"y".as_ptr () as *mut c_char }

/// C: <intprops.h> 305-312's `INT_STRLEN_BOUND (t)`, which this tree records as a blind mirror
/// (`lib/include/intprops.rs`) that no unit includes - the `src/xterm.rs` 18944 shape.  C's
/// `<intprops.h>` is in every configuration's include graph, and its three callers in this file
/// (`x_create_tip_frame`'s two and C 10550/10563's two version buffers, status 3.952) sit under
/// the file gate, so the row does too.
#[cfg (feature = "have_x_windows")]
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => { ((std::mem::size_of::<$t> () * 8 - 1) * 146 + 484) / 485 + 1 };
}

/* C: X11/Intrinsic.h's entry points and the three lwlib/this-file shims C 4212/4346/4422 call.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe extern "C" {
  pub fn XtAppCreateShell (application_name: String, application_class: *const c_char,
			   widget_class: WidgetClass, display: *mut Display, args: *mut Arg,
			   num_args: c_uint) -> Widget;
  pub fn XtCreateWidget (name: String, widget_class: WidgetClass, parent: Widget,
			 args: *mut Arg, num_args: c_uint) -> Widget;
  pub fn XtManageChild (widget: Widget);
  pub fn XtRealizeWidget (widget: Widget);
  pub fn XtMapWidget (widget: Widget);
  pub fn XtVaGetValues (widget: Widget, ...);
  pub fn XtAddEventHandler (widget: Widget, event_mask: c_ulong, nonmaskable: Bool,
			    proc_: XtEventHandler, client_data: XtPointer);
  pub fn lw_create_widget (r#type: *const c_char, name: *const c_char, id: LWLIB_ID,
			   val: *mut widget_value, parent: Widget, pop_up_p: Boolean,
			   pre_activate_cb: lw_callback, selection_cb: lw_callback,
			   post_activate_cb: lw_callback, highlight_cb: lw_callback) -> Widget;
  pub fn lw_set_main_areas (parent: Widget, menubar: Widget, work_area: Widget);
  pub static mut applicationShellWidgetClass: WidgetClass;
}

/* C: lisp.h 5553: `extern char *xlispstrdup (Lisp_Object string)` - `alloc.c` 842 defines it
   and no mirror of this unit's graph carries the row, so it stands here (P-72 rule 4; the
   `docs/ports/xfns.md` owed row names `include/lisp.rs`).  C's row carries no guard of its own
   (lisp.h is included in every configuration), and its callers are the `USE_X_TOOLKIT` arm of
   C 4892 and the `USE_GTK`+`HAVE_FREETYPE` arm of C 9767/9772, so it stands under the file
   gate - slice 9's narrower `use_x_toolkit` gate is superseded.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {

}

/* C: `frame.c` 6180 defines `validate_x_resource_name` and no header declares it - the
   reference calls it from xfns.c 4329/4550, pgtkfns.c 156, xsmfns.c 377 and frame.c 6263 the
   same way.  Both arms of this file call it, so the row stands under the whole family's guard
   (P-72 rule 4; the owed row for the declaring header is in `docs/ports/xfns.md`).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {

}

/* C: X11/Intrinsic.h: `typedef int Boolean;`, `typedef char *XtPointer;` and `typedef struct
   _WidgetClassRec *WidgetClass;` - the X toolkit's own types, none of them mirrored in this
   tree (the same header-alone rule as `Widget` at 1386).  The block below names all three, and
   so do the two mirrors its include graph pulls in - `include/widget.rs`'s `emacsFrameClass`
   and `../lwlib/include/lwlib.rs`'s nine `lw_*` prototypes - which is why the three rows are
   owed to `include/xlib.rs` in `docs/ports/xfns.md` (the `Widget` row's twin).  `emacsFrameClass`
   is **not** declared here: `include/widget.rs` 165-177 puts it in `widget_decls` and glob-imports
   that module in this arm, and a definition in the including unit shadows a glob import - which is
   why the compiler stayed silent while `check-mirror-dupes.py` read the duplicate.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type Boolean = c_int;
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type XtPointer = *mut c_void;
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type WidgetClass = *mut c_void;
/// C: X.h: `typedef unsigned short Dimension;` - the `CorePart` members below are
/// `Dimension`s (the `lwlib/lwlib-utils.rs` 64 spelling of the same typedef).
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type Dimension = c_ushort;
/// C: IntrinsicP.h's `WidgetRec`/`CorePart` - the record carries only the members C 4251-4252
/// reads (`core.height`, `core.border_width`), in C's own relative order, and its offsets are
/// **UNVERIFIED** (the `lwlib/lwlib-utils.rs` 89-108 shape for the same record).
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct WidgetRec
{
  pub core: CorePart,
}
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CorePart
{
  pub height: Dimension,                       // C: `Dimension height;`
  pub border_width: Dimension,                 // C: `Dimension border_width;`
}

/// C: X11/Intrinsic.h: `typedef void (*XtEventHandler) (Widget, XtPointer, XEvent *, Boolean *);`
/// - the header's own event-handler type, whose four parameter types are the three above and
/// `XEvent` (the mirror's).
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type XtEventHandler =
  Option<unsafe extern "C" fn (Widget, XtPointer, *mut XEvent, *mut Boolean)>;

/// C 4176-4428: `static void x_window (struct frame *f, long window_prompting)`, the
/// `USE_X_TOOLKIT` arm.  C 4245-4322's geometry block is C's own scope; the two `#if false`
/// blocks (C 4255-4263 and C 4674's twin in the other arm) are C's dead text and are carried as
/// comments.
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe fn x_window (f: *mut frame, mut window_prompting: c_long)
{
  use crate::x11_constants::{CWEventMask, CWOverrideRedirect, PropModeAppend, PropModeReplace,
			     XA_ATOM};

  let mut class_hints: XClassHint = std::mem::zeroed ();                // C 4179 (the fields at C 4331-4332)
  let mut attributes: XSetWindowAttributes = std::mem::zeroed ();       // C 4180 (the fields at C 4360-4372)
  let mut attribute_mask: c_long;                                       // C 4181
  let shell_widget: Widget;                                             // C 4182
  let pane_widget: Widget;                                              // C 4183
  let frame_widget: Widget;                                             // C 4184
  let mut al: [Arg; 25] = std::array::from_fn (|_| std::mem::zeroed ());  // C 4185 (C: `Arg al[25];`)
  let mut ac: c_int;                                                    // C 4186

  block_input ();                                                       // C 4188

  /* Use the resource name as the top-level widget name
     for looking up resources.  Make a non-Lisp copy
     for the window manager, so GC relocation won't bother it.

     Elsewhere we specify the window name for the window manager.  */
  (*f).namebuf = xlispstrdup (Vx_resource_name);                        // C 4195

  ac = 0;                                                               // C 4197
  XtSetArg! (al[ac as usize], XtNallowShellResize (), 1); ac += 1;      // C 4198
  XtSetArg! (al[ac as usize], XtNinput (), 1); ac += 1;                 // C 4199
  XtSetArg! (al[ac as usize], XtNmappedWhenManaged (), 0); ac += 1;     // C 4200
  XtSetArg! (al[ac as usize], XtNborderWidth (), (*f).border_width); ac += 1;      // C 4201
  XtSetArg! (al[ac as usize], XtNvisual (), FRAME_X_VISUAL! (f)); ac += 1;         // C 4202
  XtSetArg! (al[ac as usize], XtNdepth (), (*FRAME_DISPLAY_INFO! (f)).n_planes); ac += 1;  // C 4203
  XtSetArg! (al[ac as usize], XtNcolormap (), FRAME_X_COLORMAP! (f)); ac += 1;     // C 4204
  /* frame.h 1840's `EMACS_CLASS`: the toolkit resource class C 4206 hands to
     `XtAppCreateShell`.  An *external* name (the X server and the user's own resource file
     read it), so it keeps C's bytes (PORTING.md 0.1-19's kept list, status 3.648); the
     `src/xterm.rs` 19974-19978 shape.  */
  let EMACS_CLASS: &core::ffi::CStr = c"Emacs";
  shell_widget = XtAppCreateShell ((*f).namebuf, EMACS_CLASS.as_ptr (),     // C 4205-4207
				   applicationShellWidgetClass, FRAME_X_DISPLAY! (f),
				   al.as_mut_ptr (), ac as c_uint);

  (*((*f).output_data.x as *mut x_output)).widget = shell_widget;        // C 4209
  /* maybe_set_screen_title_format (shell_widget); */                   // C 4210 (C's own comment)

  pane_widget = lw_create_widget (c"main".as_ptr (), c"pane".as_ptr (),   // C 4212-4214
				  widget_id_tick, std::ptr::null_mut (), shell_widget, 0,
				  None, None, None, None);
  widget_id_tick += 1;

  ac = 0;                                                               // C 4216
  XtSetArg! (al[ac as usize], XtNvisual (), FRAME_X_VISUAL! (f)); ac += 1;         // C 4217
  XtSetArg! (al[ac as usize], XtNdepth (), (*FRAME_DISPLAY_INFO! (f)).n_planes); ac += 1;  // C 4218
  XtSetArg! (al[ac as usize], XtNcolormap (), FRAME_X_COLORMAP! (f)); ac += 1;     // C 4219
  XtSetArg! (al[ac as usize], XtNborderWidth (), 0); ac += 1;           // C 4220
  XtSetValues (pane_widget, al.as_mut_ptr (), ac as c_uint);            // C 4221
  (*((*f).output_data.x as *mut x_output)).column_widget = pane_widget;  // C 4222
  /* mappedWhenManaged to false tells to the paned window to not map/unmap
     the emacs screen when changing menubar.  This reduces flickering.  */

  ac = 0;                                                               // C 4227
  XtSetArg! (al[ac as usize], XtNmappedWhenManaged (), 0); ac += 1;     // C 4228
  XtSetArg! (al[ac as usize], XtNshowGrip.as_ptr () as String, 0); ac += 1; // C 4229
  XtSetArg! (al[ac as usize], XtNallowResize.as_ptr () as String, 1); ac += 1; // C 4230
  XtSetArg! (al[ac as usize], XtNresizeToPreferred.as_ptr () as String, 1); ac += 1; // C 4231
  XtSetArg! (al[ac as usize], XtNemacsFrame.as_ptr () as String, f); ac += 1; // C 4232
  XtSetArg! (al[ac as usize], XtNvisual (), FRAME_X_VISUAL! (f)); ac += 1;         // C 4233
  XtSetArg! (al[ac as usize], XtNdepth (), (*FRAME_DISPLAY_INFO! (f)).n_planes); ac += 1;  // C 4234
  XtSetArg! (al[ac as usize], XtNcolormap (), FRAME_X_COLORMAP! (f)); ac += 1;     // C 4235
  XtSetArg! (al[ac as usize], XtNborderWidth (), 0); ac += 1;           // C 4236
  frame_widget = XtCreateWidget ((*f).namebuf, emacsFrameClass (), pane_widget,  // C 4237-4238
				 al.as_mut_ptr (), ac as c_uint);

  (*((*f).output_data.x as *mut x_output)).edit_widget = frame_widget;   // C 4240

  XtManageChild (frame_widget);                                         // C 4242

  /* Do some needed geometry management.  */
  {
    let mut gal: [Arg; 3] = std::array::from_fn (|_| std::mem::zeroed ());  // C 4246
    let mut gac: c_int = 0;                                             // C 4247
    let mut extra_borders: c_int = 0;                                   // C 4248
    let menubar_size: c_int                                             // C 4249-4253
      = if !(*((*f).output_data.x as *mut x_output)).menubar_widget.is_null ()
	{
	  /* C's `menubar_widget->core.height + menubar_widget->core.border_width`: the widget's
	     `core` is libXt's own record, which no mirror in this tree carries, so the port reads it
	     through the two fields C's expression names (P-72 rule 3).  */
	  (*((*((*f).output_data.x as *mut x_output)).menubar_widget as *mut WidgetRec)).core.height
	      as c_int
	    + (*((*((*f).output_data.x as *mut x_output)).menubar_widget as *mut WidgetRec)).core.border_width
	      as c_int
	}
      else
	{ 0 };

    /* C 4255-4263: `#if false` - the `FRAME_EXTERNAL_MENU_BAR` arm C's own comment (24 Aug 96,
       rms) disables; carried as this comment.  */

    /* C 4265: `FRAME_MENUBAR_HEIGHT (f) = menubar_size;`.  The mirror carries only C xterm.h
       1463-1464's `#else` variant (`include/xterm.rs` 1177-1181: `((void) f, 0)`, which is not
       an lvalue), while C 1461-1462's `USE_X_TOOLKIT || USE_GTK` variant is
       `((f)->output_data.x->menubar_height)`; the assignment is therefore spelled through that
       member (P-72 rule 3's use-site form, the `FRAME_LIVE_P` shape at C 4797).  The missing
       mirror row is owed and named in `docs/ports/xfns.md` - its other face is the two reads at
       4929/4976, which the non-toolkit body answers with 0 in the GTK arm.  */
    (*((*f).output_data.x as *mut x_output)).menubar_height = menubar_size;  // C 4265

#[cfg (not (feature = "use_lucid"))]
    {
      /* Motif seems to need this amount added to the sizes
	 specified for the shell widget.  The Athena/Lucid widgets don't.
	 Both conclusions reached experimentally.  -- rms.  */
      XtVaGetValues ((*((*f).output_data.x as *mut x_output)).edit_widget,  // C 4271-4272
		     XtNinternalBorderWidth.as_ptr () as String, &raw mut extra_borders,
		     std::ptr::null_mut::<c_void> ());
      extra_borders *= 2;                                               // C 4273
    }

    (*f).shell_position                                                // C 4276
      = xmalloc (4 + 4 * INT_STRLEN_BOUND! (c_int) as usize) as *mut c_char;

    /* Convert our geometry parameters into a geometry string
       and specify it.
       Note that we do not specify here whether the position
       is a user-specified or program-specified one.
       We pass that information later, in x_wm_set_size_hint.  */
    let xneg: bool = (window_prompting & XNegative as c_long) != 0;     // C 4283
    let yneg: bool = (window_prompting & YNegative as c_long) != 0;     // C 4284

    if !FRAME_PARENT_FRAME (f).is_null ()                               // C 4286
      {
	if window_prompting & XNegative as c_long != 0                  // C 4288
	  { (*f).left_pos = FRAME_PIXEL_WIDTH (FRAME_PARENT_FRAME (f))  // C 4289-4290
	      - FRAME_PIXEL_WIDTH (f) + (*f).left_pos; }

	if window_prompting & YNegative as c_long != 0                  // C 4292
	  { (*f).top_pos = FRAME_PIXEL_HEIGHT (FRAME_PARENT_FRAME (f))  // C 4293-4294
	      - FRAME_PIXEL_HEIGHT (f) + (*f).top_pos; }

	window_prompting &= !(XNegative as c_long | YNegative as c_long);  // C 4296
      }

    if window_prompting & USPosition as c_long != 0                     // C 4299
      { sprintf ((*f).shell_position, c"=%dx%d%c%d%c%d".as_ptr (),      // C 4300-4304
		 FRAME_PIXEL_WIDTH (f) + extra_borders,
		 FRAME_PIXEL_HEIGHT (f) + menubar_size + extra_borders,
		 if xneg { b'-' as c_int } else { b'+' as c_int }, (*f).left_pos,
		 if yneg { b'-' as c_int } else { b'+' as c_int }, (*f).top_pos); }
    else
      {
	sprintf ((*f).shell_position, c"=%dx%d".as_ptr (),              // C 4307-4309
		 FRAME_PIXEL_WIDTH (f) + extra_borders,
		 FRAME_PIXEL_HEIGHT (f) + menubar_size + extra_borders);

	/* Setting x and y when the position is not specified in
	   the geometry string will set program position in the WM hints.
	   If Emacs had just one program position, we could set it in
	   fallback resources, but since each make-frame call can specify
	   different program positions, this is easier.  */
	XtSetArg! (gal[gac as usize], XtNx (), (*f).left_pos); gac += 1;    // C 4316
	XtSetArg! (gal[gac as usize], XtNy (), (*f).top_pos); gac += 1;     // C 4317
      }

    XtSetArg! (gal[gac as usize], XtNgeometry.as_ptr () as String, (*f).shell_position); // C 4320
    gac += 1;
    XtSetValues (shell_widget, gal.as_mut_ptr (), gac as c_uint);        // C 4321
  }

  XtManageChild (pane_widget);                                          // C 4324
  XtRealizeWidget (shell_widget);                                       // C 4325

  FRAME_X_WINDOW! (f) = XtWindow (frame_widget);                        // C 4327
  initial_set_up_x_back_buffer (f);                                     // C 4328
  validate_x_resource_name ();                                          // C 4329

  class_hints.res_name = SSDATA (Vx_resource_name);                     // C 4331
  class_hints.res_class = SSDATA (Vx_resource_class);                   // C 4332
  XSetClassHint (FRAME_X_DISPLAY! (f), XtWindow (shell_widget),          // C 4333
		 &raw mut class_hints);

#[cfg (feature = "have_x_i18n")]
  {
    FRAME_XIC! (f) = std::ptr::null_mut ();                             // C 4336
    if (*FRAME_DISPLAY_INFO! (f)).use_xim                              // C 4337
      { create_frame_xic (f); }                                        // C 4338
  }

  (*((*f).output_data.x as *mut x_output)).wm_hints.input = True;        // C 4341
  (*((*f).output_data.x as *mut x_output)).wm_hints.flags |= InputHint;  // C 4342
  XSetWMHints (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),                // C 4343-4344
	       &raw mut (*((*f).output_data.x as *mut x_output)).wm_hints);

  hack_wm_protocols (f, shell_widget);                                  // C 4346
  append_wm_protocols (FRAME_DISPLAY_INFO! (f), f);                     // C 4347

#[cfg (feature = "x_toolkit_editres")]
  XtAddEventHandler (shell_widget, 0, 1, Some (_XEditResCheckMessages),  // C 4350
		     std::ptr::null_mut ());

  /* Do a stupid property change to force the server to generate a
     PropertyNotify event so that the event_stream server timestamp will
     be initialized to something relevant to the time we created the window.
     */
  XChangeProperty (XtDisplay (frame_widget), XtWindow (frame_widget),    // C 4357-4359
		   (*FRAME_DISPLAY_INFO! (f)).Xatom_wm_protocols, XA_ATOM as Atom, 32,
		   PropModeAppend, std::ptr::null_mut (), 0);

  /* Make all the standard events reach the Emacs frame.  */
  attributes.event_mask = STANDARD_EVENT_SET;                           // C 4362

#[cfg (feature = "have_x_i18n")]
  if !FRAME_XIC! (f).is_null ()                                         // C 4365
    {
      /* XIM server might require some X events. */
      let mut fevent: c_long = NoEventMask;                             // C 4368
      XGetICValues (FRAME_XIC! (f), XNFilterEvents.as_ptr (), &raw mut fevent,  // C 4369
		    std::ptr::null_mut::<c_void> ());
      attributes.event_mask |= fevent;                                  // C 4370
    }

  attributes.override_redirect = FRAME_OVERRIDE_REDIRECT! (f);           // C 4374
  attribute_mask = CWEventMask | CWOverrideRedirect;                     // C 4375
  XChangeWindowAttributes (XtDisplay (shell_widget), XtWindow (shell_widget),  // C 4376-4377
			   attribute_mask as c_ulong, &raw mut attributes);

  XtMapWidget (frame_widget);                                           // C 4379

  /* x_set_name normally ignores requests to set the name if the
     requested name is the same as the current name.  This is the one
     place where that assumption isn't correct; f->name is set, but
     the X server hasn't been told.  */
  {
    let name: Lisp_Object;                                              // C 4386
    let explicit: bool = explicit_name! (f);                            // C 4387

    explicit_name! (f, false);                                          // C 4389
    name = (*f).name;                                                   // C 4390
    fset_name (f, Qnil ());                                             // C 4391
    x_set_name (f, name, explicit);                                     // C 4392
  }

  if FRAME_UNDECORATED! (f) != 0                                        // C 4395
    {
      let dpy: *mut Display = FRAME_X_DISPLAY! (f);                     // C 4397
      let mut hints: PropMotifWmHints = std::mem::zeroed ();            // C 4398
      let prop: Atom = (*FRAME_DISPLAY_INFO! (f)).Xatom_MOTIF_WM_HINTS;  // C 4399

      memset (&raw mut hints as *mut c_void, 0, std::mem::size_of::<PropMotifWmHints> ());  // C 4401
      hints.flags = MWM_HINTS_DECORATIONS as c_ulong;                   // C 4402
      hints.decorations = 0;                                            // C 4403

      /* For some reason the third and fourth arguments in the following
	 call must be identical: ...  */
      XChangeProperty (dpy, FRAME_OUTER_WINDOW! (f), prop, prop, 32, PropModeReplace,
		       &raw mut hints as *mut c_uchar,
		       PROP_MOTIF_WM_HINTS_ELEMENTS);                       // C 4409-4411
    }

  /* C passes an assignment expression; Rust has no such expression, so the assignment is its own
     statement and the value it leaves is passed (slice 6's note).  */
  (*((*f).output_data.x as *mut x_output)).current_cursor               // C 4415-4416
    = (*((*f).output_data.x as *mut x_output)).text_cursor;
  XDefineCursor (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),              // C 4414
		 (*((*f).output_data.x as *mut x_output)).current_cursor);

  unblock_input ();                                                     // C 4418

  /* This is a no-op, except under Motif.  Make sure main areas are
     set to something reasonable, in case we get an error later.  */
  lw_set_main_areas (pane_widget, std::ptr::null_mut (), frame_widget);  // C 4422

#[cfg (feature = "have_xinput2")]
  if (*FRAME_DISPLAY_INFO! (f)).supports_xi2                            // C 4425
    { setup_xi_event_mask (f); }                                        // C 4426
}

/* frame.h's packed flags the body of C 4918-5582 writes (`no_split` at C 5311,
   `can_set_window_size` at C 5414 and `was_invisible` at C 5460-5478): `include/frame.rs` records
   them as `// C:` lines, so each unit whose bodies touch them defines its own writers over the
   mirror's `bits1`/`bits2` (PORTING.md 1.3; the `src/androidfns.rs` 186-250 shape, and the
   `explicit_name!` macro above for the fourth).  */
#[cfg (feature = "have_x_windows")]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
#[cfg (feature = "have_x_windows")]
macro_rules! set_frame_flag2 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
/// C: frame.h 411: `bool_bf no_split : 1;` (read and written at C 5311)
#[cfg (feature = "have_x_windows")]
macro_rules! no_split {
  ($f:expr) => { (*$f).bits1 >> FRAME_NO_SPLIT_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_NO_SPLIT_SHIFT, $v) };
}
/// C: frame.h 491: `bool_bf can_set_window_size : 1;` (written at C 5414)
#[cfg (feature = "have_x_windows")]
macro_rules! can_set_window_size {
  ($f:expr) => { (*$f).bits1 >> FRAME_CAN_SET_WINDOW_SIZE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_CAN_SET_WINDOW_SIZE_SHIFT, $v) };
}
/// C: frame.h 530: `bool_bf was_invisible : 1;` (read and written at C 5460-5478)
#[cfg (feature = "have_x_windows")]
macro_rules! was_invisible {
  ($f:expr) => { (*$f).bits2 >> FRAME_WAS_INVISIBLE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_WAS_INVISIBLE_SHIFT, $v) };
}

/* C: frame.h 471: `bool_bf override_redirect : 1;` - `include/frame.rs` 624-627 emits its
   unit/shift pair only under `have_window_system`, while the measured numbering is the same for
   every configuration (`// A 2:7; B 2:7; C 2:7`), so the arms without that feature read the two
   constants from here and the mirror row is owed (`docs/ports/xfns.md`).  The file's own
   `FRAME_OVERRIDE_REDIRECT!` at 1306-1312 is the writer this slice first expands.  */
#[cfg (all (feature = "have_x_windows", not (feature = "have_window_system")))]
pub const FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT: u32 = 7;
#[cfg (all (feature = "have_x_windows", not (feature = "have_window_system")))]
pub const FRAME_OVERRIDE_REDIRECT_FLAG_UNIT: u32 = 2;

/* The two arguments C 5184-5188 and C 5213-5217 pick per arm: an attribute cannot sit on a call's
   argument, so each pair is a macro the two arms instantiate (the slice 8 `default_font_names!`
   shape).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
macro_rules! internal_border_default { () => { make_fixnum (0) } }      // C 5185
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk")))]
macro_rules! internal_border_default { () => { make_fixnum (1) } }      // C 5187
#[cfg (all (feature = "have_x_windows", feature = "use_gtk", feature = "use_toolkit_scroll_bars"))]
macro_rules! vertical_scroll_bars_default { () => { Qright () } }       // C 5214
#[cfg (all (feature = "have_x_windows",
	     not (all (feature = "use_gtk", feature = "use_toolkit_scroll_bars"))))]
macro_rules! vertical_scroll_bars_default { () => { Qleft () } }        // C 5216

/* The names C 5132-5543 reads that no mirror of this unit's graph carries: the five font-driver
   objects (ftcrfont.c, xftfont.c and xfont.c define them), `register_font_driver` (font.h 896),
   and the frame, face, Cairo and XSync entry points.  `sysdep.h`'s two XSync types stand with
   them.  Each row is owed to `docs/ports/xfns.md` (P-72 rule 4).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn register_font_driver (driver: *const c_void, f: *mut frame);
  pub fn init_frame_faces (f: *mut frame);
  pub fn initialize_frame_menubar (f: *mut frame);
  pub fn x_embed_frame (dpyinfo: *mut x_display_info, f: *mut frame) -> bool;


  pub fn x_iconify_frame (f: *mut frame);
  pub fn x_cr_destroy_frame_context (f: *mut frame);                          // xterm.h 1811
  pub fn x_cr_update_surface_desired_size (f: *mut frame, width: c_int, height: c_int);
  pub fn x_sync_init_fences (f: *mut frame);
  pub fn XIGetSelectedEvents (dpy: *mut Display, win: Window, count: *mut c_int)
			      -> *mut c_void;
  pub fn XSyncIntToValue (value: *mut XSyncValue, i: c_int);
  pub fn XSyncCreateCounter (dpy: *mut Display, value: XSyncValue) -> XSyncCounter;
  pub static mut ftcrfont_driver: font_driver;
  pub static mut ftcrhbfont_driver: font_driver;
  pub static mut xftfont_driver: font_driver;
  pub static mut xfthbfont_driver: font_driver;
  pub static mut xfont_driver: font_driver;
}
/// C: sysdep.h's X11/extensions/sync.h types the block above names (they have no mirror either).
#[cfg (feature = "have_x_windows")]
#[repr (C)] #[derive (Copy, Clone)]
pub struct XSyncValue
{
  pub hi: c_int,
  pub lo: c_uint,
}
/// C: `<X11/extensions/sync.h>`: `typedef XID XSyncCounter;`
#[cfg (feature = "have_x_windows")]
pub type XSyncCounter = XID;

/* C 4918-5582: `DEFUN ("x-create-frame", Fx_create_frame, Sx_create_frame, 1, 1, 0, …)` - the
   file's largest function (665 C lines), the item slice 8 named as the next one.  C's own `#ifdef`
   arms are the font-driver block (C 5131-5146), the GTK3 CSS providers (C 5243-5248), the `USE_GTK`
   arms at C 5184-5188/5213-5217/5322-5326/5378-5386, the `USE_X_TOOLKIT` `x_window` call
   (C 5316-5320), the toolkit/GTK menu bar (C 5395-5411), the Cairo surface size (C 5432-5441) and
   the XSync block (C 5500-5546).  */
/// C: frame.h 1145's `ENUM_BF (output_method)` is the four-bit field in the middle of `bits1`; the
/// mirror's reader is private to `include/frame.rs`, so the writer C spells
/// (`f->output_method = …`, C 5046) stands here (the `src/androidfns.rs` 285-297 shape).
#[cfg (feature = "have_x_windows")]
macro_rules! set_output_method {
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
                  | ((($v as u32) & FRAME_OUTPUT_METHOD_MASK) << FRAME_OUTPUT_METHOD_SHIFT)
  };
}
#[cfg (feature = "have_x_windows")]
macro_rules! output_method {
  ($f:expr) => { output_method ($f) };
  ($f:expr, $v:expr) => { set_output_method! ($f, $v) };
}

/// C: X11/X.h 289: `#define VisibilityFullyObscured 2` (measured on this machine's header) - the
/// value C 5058 stores and xterm.h 1500's `FRAME_X_VISIBLE` tests - no mirror carries the
/// constant: `include/xterm.rs` 1264 names it with no definition, and the same mirror's
/// 1236 cites the macro as 2035, a wrong row of that mirror (owed).
#[cfg (feature = "have_x_windows")]
pub const VisibilityFullyObscured: c_int = 2;

/* C 4929-4943's four declarations the include graph does not carry.  frame.h 1479/1489/1491's
   `make_frame`, `make_minibuffer_frame` and `make_frame_without_minibuffer` are frame.c's
   definitions, and frame.h 1430's `frame_set_id_from_params` is frame.c's; the rows are owed to
   `include/frame.rs` in `docs/ports/xfns.md` (the `src/androidfns.rs` 100-111 shape).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {


  pub fn make_frame_without_minibuffer (mini_window: Lisp_Object, kb: *mut KBOARD,
					display: Lisp_Object) -> *mut frame;
  pub fn frame_set_id_from_params (f: *mut frame, params: Lisp_Object) -> EMACS_UINT;
}

/* C 4918-5582: the DEFUN and the whole body (C 4930-4939's locals, C 4945-5130 the display,
   name, parent and colour setup).  The `USE_GTK` declaration C 4941-4943 is the cfg'd local
   below, and the token rows this body leaves are recorded in `docs/ports/xfns.md`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-create-frame", Fx_create_frame, Sx_create_frame,
	1, 1, 0,
	doc: "Make a new X window, which is called a \"frame\" in Emacs terms.\nReturn an Emacs frame object.  PARMS is an alist of frame parameters.\nIf the parameters specify that the frame should not have a minibuffer,\nand do not specify a specific minibuffer window to use, then\n`default-minibuffer-frame' must be a frame whose minibuffer can be\nshared by the new frame.\n\nThis function is an internal primitive--use `make-frame' instead.  ",
	(parms: Lisp_Object),
	{
  let f: *mut frame;                                                    // C 4930
  let frame: Lisp_Object;                                               // C 4931
  let mut tem: Lisp_Object;                                             // C 4931
  let name: Lisp_Object;                                                // C 4932
  let mut minibuffer_only: bool = false;                                // C 4933
  let mut undecorated: bool = false;                                    // C 4934
  let mut override_redirect: bool = false;                              // C 4934
  let mut window_prompting: c_long = 0;                                 // C 4935
  let count: specpdl_ref = SPECPDL_INDEX ();                            // C 4936
  let mut display: Lisp_Object;                                         // C 4937
  let mut dpyinfo: *mut x_display_info = std::ptr::null_mut ();         // C 4938
  let mut parent: Lisp_Object;                                          // C 4939
  let mut parent_frame: Lisp_Object;                                    // C 4939
  let mut kb: *mut KBOARD;                                              // C 4940
  use crate::x11_constants::{PropModeReplace, XA_CARDINAL, XA_WINDOW};
#[cfg (feature = "have_gtk3")]
  let mut gwin: *mut GdkWindow;                                         // C 4942
  let mut parms = parms;                          /* C 4945 reassigns the parameter.  */

  parms = Fcopy_alist (parms);                                          // C 4945

  /* Use this general default value to start with
     until we know if this frame has a specified name.  */
  Vx_resource_name = Vinvocation_name;                                  // C 4949

  display = gui_display_get_arg (dpyinfo, parms, Qterminal (), std::ptr::null (),
				 std::ptr::null (), RES_TYPE_NUMBER);   // C 4951-4952
  if BASE_EQ (display, Qunbound ())                                     // C 4953
    { display = gui_display_get_arg (dpyinfo, parms, Qdisplay (), std::ptr::null (),
				     std::ptr::null (), RES_TYPE_STRING); }
  if BASE_EQ (display, Qunbound ())                                     // C 4956
    { display = Qnil (); }
  dpyinfo = check_x_display_info (display);                             // C 4958
  /* C 4959: `dpyinfo->terminal->kboard` - the mirror's `terminal.kboard` is `*mut c_void`
     (`struct kboard`'s record is keyboard.h's, P-72 rule 3), so the port casts it.  */
  kb = (*((*dpyinfo).terminal)).kboard as *mut KBOARD;

  if (*((*dpyinfo).terminal)).name.is_null ()                           // C 4961
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }

  /* C 5016/5028 put the call inside the `!NILP (…)` test; the assignment is a statement of its
     own in Rust (the `src/androidfns.rs` 1187-1190 note for the same C).  */
  name = gui_display_get_arg (dpyinfo, parms, Qname (), c"name".as_ptr (),
			      c"Name".as_ptr (), RES_TYPE_STRING);      // C 4964-4965
  if !STRINGP (name)
      && ! BASE_EQ (name, Qunbound ())
      && ! NILP (name)
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }   // C 4966-4969

  if STRINGP (name)                                                     // C 4971
    { Vx_resource_name = name; }

  /* See if parent window is specified.  */
  parent = gui_display_get_arg (dpyinfo, parms, Qparent_id (), std::ptr::null (),
				std::ptr::null (), RES_TYPE_NUMBER);    // C 4975-4976
  if BASE_EQ (parent, Qunbound ())                                      // C 4977
    { parent = Qnil (); }
  if ! NILP (parent)                                                    // C 4979
    { CHECK_FIXNUM (parent); }

  frame = Qnil ();                                                      // C 4982
  tem = gui_display_get_arg (dpyinfo,
			     parms, Qminibuffer (), c"minibuffer".as_ptr (),
			     c"Minibuffer".as_ptr (),
			     RES_TYPE_SYMBOL);                          // C 4983-4985
  if EQ (tem, Qnone ()) || NILP (tem)                                   // C 4986
    { f = make_frame_without_minibuffer (Qnil (), kb, display); }
  else if EQ (tem, Qonly ())                                            // C 4988
    {
      f = make_minibuffer_frame ();                                     // C 4990
      minibuffer_only = true;                                           // C 4991
    }
  else if WINDOWP (tem)                                                 // C 4993
    { f = make_frame_without_minibuffer (tem, kb, display); }           // C 4994
  else
    { f = make_frame (true); }                                          // C 4996

  parent_frame = gui_display_get_arg (dpyinfo,
				      parms,
				      Qparent_frame (),
				      std::ptr::null (),
				      std::ptr::null (),
				      RES_TYPE_SYMBOL);                     // C 4998-5003
  /* Accept parent-frame iff parent-id was not specified.  */
  if !NILP (parent)                                                     // C 5005
      || BASE_EQ (parent_frame, Qunbound ())
      || NILP (parent_frame)
      || !FRAMEP (parent_frame)
      || !FRAME_LIVE_P (XFRAME (parent_frame))
      || !FRAME_X_P (XFRAME (parent_frame))
    { parent_frame = Qnil (); }                                         // C 5011

  fset_parent_frame (f, parent_frame);                                  // C 5013
  store_frame_param (f, Qparent_frame (), parent_frame);                // C 5014

  /* C 5016-5023's assignment the `!NILP (…)` tests is a statement of its own here.  */
  tem = gui_display_get_arg (dpyinfo,
			     parms,
			     Qundecorated (),
			     std::ptr::null (),
			     std::ptr::null (),
			     RES_TYPE_BOOLEAN);                             // C 5016-5021
  if !NILP (tem)
      && !(BASE_EQ (tem, Qunbound ()))                                  // C 5022
    { undecorated = true; }                                             // C 5023

  FRAME_UNDECORATED! (f, undecorated);                                  // C 5025
  store_frame_param (f, Qundecorated (), if undecorated { Qt () } else { Qnil () });  // C 5026

  tem = gui_display_get_arg (dpyinfo,
			     parms,
			     Qoverride_redirect (),
			     std::ptr::null (),
			     std::ptr::null (),
			     RES_TYPE_BOOLEAN);                             // C 5028-5033
  if !NILP (tem)
      && !(BASE_EQ (tem, Qunbound ()))                                  // C 5034
    { override_redirect = true; }                                       // C 5035

  FRAME_OVERRIDE_REDIRECT! (f, override_redirect);                       // C 5037
  store_frame_param (f, Qoverride_redirect (), if override_redirect { Qt () } else { Qnil () });  // C 5038

  XSETFRAME! (frame, f);                                                // C 5040

  frame_set_id_from_params (f, parms);                                  // C 5042

  (*f).terminal = (*dpyinfo).terminal;                                  // C 5044

  output_method! (f, output_x_window);                                  // C 5046
  (*f).output_data.x = xzalloc (std::mem::size_of::<x_output> ());      // C 5047
  (*((*f).output_data.x as *mut x_output)).icon_bitmap = -1;            // C 5048
  /* C 5049: `FRAME_FONTSET (f) = -1;` - C's macro is an l-value and the mirror carries the read
     arm only (`E0428` forbids a second `FRAME_FONTSET!`), so the write is C's own member path
     (the `src/androidfns.rs` 1216-1219 shape).  */
  (*((*f).output_data.x as *mut x_output)).fontset = -1;                 // C 5049 (C: `c_int`)
  (*((*f).output_data.x as *mut x_output)).scroll_bar_foreground_pixel = (-1 as c_long) as c_ulong;  // C 5050
  (*((*f).output_data.x as *mut x_output)).scroll_bar_background_pixel = (-1 as c_long) as c_ulong;  // C 5051
#[cfg (all (feature = "use_lucid", feature = "use_toolkit_scroll_bars"))]
  {
    (*((*f).output_data.x as *mut x_output)).scroll_bar_top_shadow_pixel = (-1 as c_long) as c_ulong;     // C 5053
    (*((*f).output_data.x as *mut x_output)).scroll_bar_bottom_shadow_pixel = (-1 as c_long) as c_ulong;  // C 5054
  }
  (*((*f).output_data.x as *mut x_output)).white_relief.pixel = (-1 as c_long) as c_ulong;   // C 5056
  (*((*f).output_data.x as *mut x_output)).black_relief.pixel = (-1 as c_long) as c_ulong;   // C 5057
  (*((*f).output_data.x as *mut x_output)).visibility_state = VisibilityFullyObscured;       // C 5058

  fset_icon_name (f, gui_display_get_arg (dpyinfo,
					  parms,
					  Qicon_name (),
					  c"iconName".as_ptr (),
					  c"Title".as_ptr (),
					  RES_TYPE_STRING));            // C 5060-5065
  if ! STRINGP ((*f).icon_name)                                         // C 5066
    { fset_icon_name (f, Qnil ()); }                                    // C 5067

  /* C 5069: `FRAME_DISPLAY_INFO (f) = dpyinfo;` - the same l-value case as C 5049.  */
  (*((*f).output_data.x as *mut x_output)).display_info = dpyinfo;       // C 5069

  /* With FRAME_DISPLAY_INFO set up, this unwind-protect is safe.  */
  record_unwind_protect (Some (do_unwind_create_frame), frame);         // C 5072

  /* These colors will be set anyway later, but it's important
     to get the color reference counts right, so initialize them!  */
  {
    let black: Lisp_Object;

    /* Function x_decode_color can signal an error.  Make
       sure to initialize color slots so that we won't try
       to free colors we haven't allocated.  */
    (*f).foreground_pixel = (-1 as c_long) as c_ulong;                  // C 5082 (the macro is an l-value)
    (*f).background_pixel = (-1 as c_long) as c_ulong;                  // C 5083 (the macro is an l-value)
    (*((*f).output_data.x as *mut x_output)).cursor_pixel = (-1 as c_long) as c_ulong;      // C 5084
    (*((*f).output_data.x as *mut x_output)).cursor_foreground_pixel = (-1 as c_long) as c_ulong;  // C 5085
    (*((*f).output_data.x as *mut x_output)).border_pixel = (-1 as c_long) as c_ulong;      // C 5086
    (*((*f).output_data.x as *mut x_output)).mouse_pixel = (-1 as c_long) as c_ulong;       // C 5087

    black = build_string (c"black".as_ptr ());                          // C 5089
    (*f).foreground_pixel
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 5090-5091
    (*f).background_pixel
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 5092-5093
    (*((*f).output_data.x as *mut x_output)).cursor_pixel
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 5094-5095
    (*((*f).output_data.x as *mut x_output)).cursor_foreground_pixel
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 5096-5097
    (*((*f).output_data.x as *mut x_output)).border_pixel
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 5098-5099
    (*((*f).output_data.x as *mut x_output)).mouse_pixel
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 5100-5101
  }

  /* Specify the parent under which to make this X window.  */
  if !NILP (parent)                                                     // C 5105
    {
      (*((*f).output_data.x as *mut x_output)).parent_desc = XFIXNAT (parent) as Window;  // C 5107
      (*((*f).output_data.x as *mut x_output)).explicit_parent = true;   // C 5108 (C: `bool_bf` - the mirror's comment row)
    }
  else
    {
      (*((*f).output_data.x as *mut x_output)).parent_desc
	= (*FRAME_DISPLAY_INFO! (f)).root_window;                       // C 5112
      (*((*f).output_data.x as *mut x_output)).explicit_parent = false;  // C 5113
    }

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if BASE_EQ (name, Qunbound ()) || NILP (name)                         // C 5118
    {
      fset_name (f, build_string ((*dpyinfo).x_id_name));               // C 5120
      explicit_name! (f, false);                                        // C 5121
    }
  else
    {
      fset_name (f, name);                                              // C 5125
      explicit_name! (f, true);                                         // C 5126
      /* Use the frame's title when getting resources for this frame.  */
      specbind (Qx_resource_name (), name);                             // C 5128
    }

  /* C 5131-5146: the font drivers.  C's `#ifdef USE_CAIRO` / `#else` pair nests three more
     `#ifdef`s; each arm registers the same kind of object, so every arm keeps C's own call and the
     guards follow C's.  The driver objects are the other units' definitions (ftcrfont.c,
     xftfont.c, xfont.c), declared at this use site because no mirror carries them.  */
#[cfg (feature = "use_cairo")]
  {
    register_font_driver (&raw mut ftcrfont_driver as *const c_void, f);     // C 5132
#[cfg (feature = "harfbuzz")]
    { register_font_driver (&raw mut ftcrhbfont_driver as *const c_void, f); }  // C 5134
  }
#[cfg (not (feature = "use_cairo"))]
  {
#[cfg (feature = "freetype")]
    {
#[cfg (feature = "have_xft")]
      {
	register_font_driver (&raw mut xftfont_driver as *const c_void, f);   // C 5139
#[cfg (feature = "harfbuzz")]
	{ register_font_driver (&raw mut xfthbfont_driver as *const c_void, f); }  // C 5141
      }
    }
  }
  register_font_driver (&raw mut xfont_driver as *const c_void, f);          // C 5146
#[cfg (feature = "glyph_debug")]
  { dpyinfo_refcount = (*dpyinfo).reference_count; }                        // C 5148

  gui_default_parameter (f, parms, Qfont_backend (), Qnil (),
			 c"fontBackend".as_ptr (), c"FontBackend".as_ptr (),
			 RES_TYPE_STRING);                              // C 5151-5152

  /* Extract the window parameters from the supplied values
     that are needed to determine window geometry.  */
  x_default_font_parameter (f, parms);                                      // C 5156
  if FRAME_FONT! (f).is_null ()                                             // C 5157
    {
      delete_frame (frame, Qnoelisp ());                                    // C 5159
      error (c"Invalid frame font".as_ptr ());                              // C 5160
    }

  /* Frame contents get displaced if an embedded X window has a border.  */
  if ! FRAME_X_EMBEDDED_P! (f)                                              // C 5164
    { gui_default_parameter (f, parms, Qborder_width (), make_fixnum (0),
			     c"borderWidth".as_ptr (), c"BorderWidth".as_ptr (),
			     RES_TYPE_NUMBER); }                        // C 5165-5166

  /* This defaults to 1 in order to match xterm.  We recognize either
     internalBorderWidth or internalBorder (which is what xterm calls
     it).  */
  if NILP (Fassq (Qinternal_border_width (), parms))                        // C 5171
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo, parms, Qinternal_border_width (),
				   c"internalBorder".as_ptr (), c"internalBorder".as_ptr (),
				   RES_TYPE_NUMBER);                    // C 5175-5177
      if ! BASE_EQ (value, Qunbound ())                                     // C 5178
	{ parms = Fcons (Fcons (Qinternal_border_width (), value), parms); }  // C 5179-5180
    }

  gui_default_parameter (f, parms, Qinternal_border_width (),
			 internal_border_default! (),                   // C 5184-5188
			 c"internalBorderWidth".as_ptr (), c"internalBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5189-5190

  /* Same for child frames.  */
  if NILP (Fassq (Qchild_frame_border_width (), parms))                     // C 5193
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo, parms, Qchild_frame_border_width (),
				   c"childFrameBorder".as_ptr (),
				   c"childFrameBorder".as_ptr (),
				   RES_TYPE_NUMBER);                    // C 5197-5199
      if ! BASE_EQ (value, Qunbound ())                                     // C 5200
	{ parms = Fcons (Fcons (Qchild_frame_border_width (), value), parms); }  // C 5201-5202
    }

  gui_default_parameter (f, parms, Qchild_frame_border_width (), Qnil (),
			 c"childFrameBorderWidth".as_ptr (),
			 c"childFrameBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5205-5207
  gui_default_parameter (f, parms, Qright_divider_width (), make_fixnum (0),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);   // C 5208-5209
  gui_default_parameter (f, parms, Qbottom_divider_width (), make_fixnum (0),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);   // C 5210-5211
  gui_default_parameter (f, parms, Qvertical_scroll_bars (),
			 vertical_scroll_bars_default! (),              // C 5213-5217
			 c"verticalScrollBars".as_ptr (), c"ScrollBars".as_ptr (),
			 RES_TYPE_SYMBOL);                              // C 5218-5219
  gui_default_parameter (f, parms, Qhorizontal_scroll_bars (), Qnil (),
			 c"horizontalScrollBars".as_ptr (), c"ScrollBars".as_ptr (),
			 RES_TYPE_SYMBOL);                              // C 5220-5222
  /* Also do the stuff which must be set before the window exists.  */
  gui_default_parameter (f, parms, Qforeground_color (), build_string (c"black".as_ptr ()),
			 c"foreground".as_ptr (), c"Foreground".as_ptr (),
			 RES_TYPE_STRING);                              // C 5224-5225
  gui_default_parameter (f, parms, Qbackground_color (), build_string (c"white".as_ptr ()),
			 c"background".as_ptr (), c"Background".as_ptr (),
			 RES_TYPE_STRING);                              // C 5226-5227
  gui_default_parameter (f, parms, Qmouse_color (), build_string (c"black".as_ptr ()),
			 c"pointerColor".as_ptr (), c"Foreground".as_ptr (),
			 RES_TYPE_STRING);                              // C 5228-5229
  gui_default_parameter (f, parms, Qborder_color (), build_string (c"black".as_ptr ()),
			 c"borderColor".as_ptr (), c"BorderColor".as_ptr (),
			 RES_TYPE_STRING);                              // C 5230-5231
  gui_default_parameter (f, parms, Qscreen_gamma (), Qnil (),
			 c"screenGamma".as_ptr (), c"ScreenGamma".as_ptr (),
			 RES_TYPE_FLOAT);                               // C 5232-5233
  gui_default_parameter (f, parms, Qline_spacing (), Qnil (),
			 c"lineSpacing".as_ptr (), c"LineSpacing".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5234-5235
  gui_default_parameter (f, parms, Qleft_fringe (), Qnil (),
			 c"leftFringe".as_ptr (), c"LeftFringe".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5236-5237
  gui_default_parameter (f, parms, Qright_fringe (), Qnil (),
			 c"rightFringe".as_ptr (), c"RightFringe".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5238-5239
  gui_default_parameter (f, parms, Qno_special_glyphs (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);   // C 5240-5241

#[cfg (feature = "have_gtk3")]
  {
    (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scrollbar_background_css_provider
      = gtk_css_provider_new ();                                        // C 5244-5245
    (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scrollbar_foreground_css_provider
      = gtk_css_provider_new ();                                        // C 5246-5247
  }

  x_default_scroll_bar_color_parameter (f, parms, Qscroll_bar_foreground (),
					c"scrollBarForeground".as_ptr (),
					c"ScrollBarForeground".as_ptr (), true);   // C 5250-5252
  x_default_scroll_bar_color_parameter (f, parms, Qscroll_bar_background (),
					c"scrollBarBackground".as_ptr (),
					c"ScrollBarBackground".as_ptr (), false);  // C 5253-5255

  /* Init faces before gui_default_parameter is called for the
     scroll-bar-width parameter because otherwise we end up in
     init_iterator with a null face cache, which should not happen.  */
  init_frame_faces (f);                                                     // C 5260

  tem = gui_display_get_arg (dpyinfo, parms, Qmin_width (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_NUMBER);       // C 5262-5263
  if FIXNUMP (tem)                                                          // C 5264
    { store_frame_param (f, Qmin_width (), tem); }                          // C 5265
  tem = gui_display_get_arg (dpyinfo, parms, Qmin_height (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_NUMBER);       // C 5266-5267
  if FIXNUMP (tem)                                                          // C 5268
    { store_frame_param (f, Qmin_height (), tem); }                         // C 5269

  adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),
		     FRAME_LINES (f) * FRAME_LINE_HEIGHT (f), 5, true,
		     Qx_create_frame_1 ());                             // C 5271-5273

  /* Set the menu-bar-lines and tool-bar-lines parameters.  We don't
     look up the X resources controlling the menu-bar and tool-bar
     here; they are processed specially at startup, and reflected in
     the values of the mode variables.  */

  gui_default_parameter (f, parms, Qmenu_bar_lines (),
			 if NILP (Vmenu_bar_mode) { make_fixnum (0) } else { make_fixnum (1) },
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);  // C 5280-5283
  gui_default_parameter (f, parms, Qtab_bar_lines (),
			 if NILP (Vtab_bar_mode) { make_fixnum (0) } else { make_fixnum (1) },
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);  // C 5284-5287
  gui_default_parameter (f, parms, Qtool_bar_lines (),
			 if NILP (Vtool_bar_mode) { make_fixnum (0) } else { make_fixnum (1) },
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);  // C 5288-5291

  gui_default_parameter (f, parms, Qbuffer_predicate (), Qnil (),
			 c"bufferPredicate".as_ptr (), c"BufferPredicate".as_ptr (),
			 RES_TYPE_SYMBOL);                              // C 5293-5295
  gui_default_parameter (f, parms, Qtitle (), Qnil (),
			 c"title".as_ptr (), c"Title".as_ptr (),
			 RES_TYPE_STRING);                              // C 5296-5297
  gui_default_parameter (f, parms, Qwait_for_wm (), Qt (),
			 c"waitForWM".as_ptr (), c"WaitForWM".as_ptr (),
			 RES_TYPE_BOOLEAN);                             // C 5298-5299
  gui_default_parameter (f, parms, Qtool_bar_position (),
			 FRAME_TOOL_BAR_POSITION (f), std::ptr::null (), std::ptr::null (),
			 RES_TYPE_SYMBOL);                              // C 5300-5301
  gui_default_parameter (f, parms, Qinhibit_double_buffering (), Qnil (),
			 c"inhibitDoubleBuffering".as_ptr (),
			 c"InhibitDoubleBuffering".as_ptr (),
			 RES_TYPE_BOOLEAN);                             // C 5302-5304

  /* Compute the size of the X window.  */
  window_prompting = gui_figure_window_size (f, parms, true, true);         // C 5307

  tem = gui_display_get_arg (dpyinfo, parms, Qunsplittable (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_BOOLEAN);      // C 5309-5310
  no_split! (f, minibuffer_only || EQ (tem, Qt ()));                        // C 5311

  x_icon_verify (f, parms);                                                 // C 5313

  /* Create the X widget or window.  */
#[cfg (feature = "use_x_toolkit")]
  { x_window (f, window_prompting); }                                       // C 5317
#[cfg (not (feature = "use_x_toolkit"))]
  { x_window (f); }                                                         // C 5319

#[cfg (not (feature = "use_gtk"))]
  {
    if FRAME_X_EMBEDDED_P! (f)
	&& !x_embed_frame (dpyinfo, f)                                  // C 5323-5324
      { error (c"The frame could not be embedded; does the embedder exist?".as_ptr ()); }  // C 5325
  }

  x_icon (f, parms);                                                        // C 5328
  x_make_gc (f);                                                            // C 5329

  /* While this function is present in versions of libXi that only
     support 2.0, it does not release the display lock after
     finishing, leading to a deadlock.  */
#[cfg (all (feature = "have_xinput2", feature = "have_xinput2_1"))]
  {
    if (*dpyinfo).supports_xi2                                            // C 5335
      {
	(*((*f).output_data.x as *mut x_output)).xi_masks
	  = XIGetSelectedEvents ((*dpyinfo).display, FRAME_X_WINDOW! (f),
				 &raw mut (*((*f).output_data.x as *mut x_output)).num_xi_masks);  // C 5336-5338
      }
  }

  /* Now consider the frame official.  */
  (*((*f).terminal)).reference_count += 1;                                  // C 5342
  (*FRAME_DISPLAY_INFO! (f)).reference_count += 1;                          // C 5343
  Vframe_list = Fcons (frame, Vframe_list);                                 // C 5344

  /* We need to do this after creating the X window, so that the
     icon-creation functions can say whose icon they're describing.  */
  gui_default_parameter (f, parms, Qicon_type (), Qt (),
			 c"bitmapIcon".as_ptr (), c"BitmapIcon".as_ptr (),
			 RES_TYPE_BOOLEAN);                             // C 5348-5349

  gui_default_parameter (f, parms, Qauto_raise (), Qnil (),
			 c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (),
			 RES_TYPE_BOOLEAN);                             // C 5351-5352
  gui_default_parameter (f, parms, Qauto_lower (), Qnil (),
			 c"autoLower".as_ptr (), c"AutoRaiseLower".as_ptr (),
			 RES_TYPE_BOOLEAN);                             // C 5353-5354
  gui_default_parameter (f, parms, Qcursor_type (), Qbox (),
			 c"cursorType".as_ptr (), c"CursorType".as_ptr (),
			 RES_TYPE_SYMBOL);                              // C 5355-5356
  gui_default_parameter (f, parms, Qscroll_bar_width (), Qnil (),
			 c"scrollBarWidth".as_ptr (), c"ScrollBarWidth".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5357-5359
  gui_default_parameter (f, parms, Qscroll_bar_height (), Qnil (),
			 c"scrollBarHeight".as_ptr (), c"ScrollBarHeight".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5360-5362
  gui_default_parameter (f, parms, Qalpha (), Qnil (),
			 c"alpha".as_ptr (), c"Alpha".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5363-5364
  gui_default_parameter (f, parms, Qalpha_background (), Qnil (),
			 c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5365-5366
  gui_default_parameter (f, parms, Qborders_respect_alpha_background (), Qnil (),
			 c"bordersRespectAlphaBackground".as_ptr (),
			 c"BordersRespectAlphaBackground".as_ptr (),
			 RES_TYPE_NUMBER);                              // C 5367-5369

  if !NILP (parent_frame)                                                   // C 5371
    {
      let p: *mut frame = XFRAME (parent_frame);                            // C 5373

      block_input ();                                                       // C 5375
      XReparentWindow (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),
		       FRAME_X_WINDOW! (p), (*f).left_pos, (*f).top_pos);  // C 5376-5377
#[cfg (feature = "use_gtk")]
      {
	if EQ (x_gtk_resize_child_frames, Qresize_mode ())              // C 5379
	  { gtk_container_set_resize_mode
	      (GTK_CONTAINER! (FRAME_GTK_OUTER_WIDGET! (f)), GTK_RESIZE_IMMEDIATE); }  // C 5380-5381
      }
#[cfg (feature = "have_gtk3")]
      {
	gwin = gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f));      // C 5384
	gdk_x11_window_set_frame_sync_enabled (gwin, 0);                 // C 5385
      }
      unblock_input ();                                                     // C 5387
    }

  gui_default_parameter (f, parms, Qno_focus_on_map (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);   // C 5390-5391
  gui_default_parameter (f, parms, Qno_accept_focus (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);   // C 5392-5393

  /* C 5395-5411: the menu bar, under C's `USE_X_TOOLKIT || USE_GTK`.  */
#[cfg (any (feature = "use_x_toolkit", feature = "use_gtk"))]
  {
    if !minibuffer_only && FRAME_EXTERNAL_MENU_BAR (f)                     // C 5397
      {
	/* If this signals an error, we haven't set size hints for the
	   frame and we didn't make it visible.  */
	initialize_frame_menubar (f);                                      // C 5401

#[cfg (not (feature = "use_gtk"))]
	{
	  /* This is a no-op, except under Motif where it arranges the
	     main window for the widgets on it.  */
	  lw_set_main_areas ((*((*f).output_data.x as *mut x_output)).column_widget,
			     (*((*f).output_data.x as *mut x_output)).menubar_widget,
			     (*((*f).output_data.x as *mut x_output)).edit_widget);   // C 5406-5408
	}
      }
  }

  /* Consider frame official, now.  */
  can_set_window_size! (f, true);                                           // C 5414

  /* Tell the server what size and position, etc, we want, and how
     badly we want them.  This should be done after we have the menu
     bar so that its size can be taken into account.  */
  block_input ();                                                           // C 5419
  x_wm_set_size_hint (f, window_prompting, false);                          // C 5420
  unblock_input ();                                                         // C 5421

  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
		     0, true, Qx_create_frame_2 ());                       // C 5423-5424

  /* Process fullscreen parameter here in the hope that normalizing a
     fullheight/fullwidth frame will produce the size set by the last
     adjust_frame_size call.  */
  gui_default_parameter (f, parms, Qfullscreen (), Qnil (),
			 c"fullscreen".as_ptr (), c"Fullscreen".as_ptr (),
			 RES_TYPE_SYMBOL);                              // C 5429-5430

#[cfg (feature = "use_cairo")]
  { x_cr_update_surface_desired_size (f, FRAME_PIXEL_WIDTH (f),
				      FRAME_PIXEL_HEIGHT (f)); }         // C 5439-5440

  /* Make the window appear on the frame and enable display, unless
     the caller says not to.  However, with explicit parent, Emacs
     cannot control visibility, so don't try.  */
  if !(*((*f).output_data.x as *mut x_output)).explicit_parent               // C 5446
    {
      /* When called from `x-create-frame-with-faces' visibility is
	 always explicitly nil.  */
      let mut visibility: Lisp_Object
	= gui_display_get_arg (dpyinfo, parms, Qvisibility (), std::ptr::null (),
			       std::ptr::null (),
			       RES_TYPE_SYMBOL);                        // C 5450-5452
      let height: Lisp_Object
	= gui_display_get_arg (dpyinfo, parms, Qheight (), std::ptr::null (),
			       std::ptr::null (),
			       RES_TYPE_NUMBER);                        // C 5453-5454
      let width: Lisp_Object
	= gui_display_get_arg (dpyinfo, parms, Qwidth (), std::ptr::null (),
			       std::ptr::null (),
			       RES_TYPE_NUMBER);                        // C 5455-5456

      if EQ (visibility, Qicon ())                                          // C 5458
	{
	  was_invisible! (f, true);                                         // C 5460
	  x_iconify_frame (f);                                              // C 5461
	}
      else
	{
	  if BASE_EQ (visibility, Qunbound ())                              // C 5465
	    { visibility = Qt (); }                                         // C 5466

	  if !NILP (visibility)                                             // C 5468
	    { x_make_frame_visible (f); }                                   // C 5469
	  else
	    { was_invisible! (f, true); }                                   // C 5471
	}

      /* Leave f->was_invisible true only if height or width were
	 specified too.  This takes effect only when we are not called
	 from `x-create-frame-with-faces' (see above comment).  */
      was_invisible! (f, was_invisible! (f)
			 && (!BASE_EQ (height, Qunbound ()) || !BASE_EQ (width, Qunbound ())));  // C 5477-5479

      store_frame_param (f, Qvisibility (), visibility);                    // C 5481
    }

  block_input ();                                                           // C 5484

  /* Set machine name and pid for the purpose of window managers.  */
  set_machine_and_pid_properties (f);                                       // C 5487

  /* Set the WM leader property.  GTK does this itself, so this is not
     needed when using GTK.  */
  if (*dpyinfo).client_leader_window != 0                                  // C 5491
    {
      XChangeProperty (FRAME_X_DISPLAY! (f),
		       FRAME_OUTER_WINDOW! (f),
		       (*dpyinfo).Xatom_wm_client_leader,
		       XA_WINDOW as Atom, 32, PropModeReplace,
		       &raw mut (*dpyinfo).client_leader_window as *mut c_uchar, 1);  // C 5493-5497
    }

#[cfg (feature = "have_xsync")]
  {
    if (*dpyinfo).xsync_supported_p                                       // C 5501
	/* Frame synchronization isn't supported in child frames.  */
	&& NILP (parent_frame)                                            // C 5503
	&& !(*((*f).output_data.x as *mut x_output)).explicit_parent      // C 5504
      {
#[cfg (not (feature = "have_gtk3"))]
	{
	  let mut initial_value: XSyncValue;
	  let mut counters: [XSyncCounter; 2];

	  AUTO_STRING! (synchronizeResize, c"synchronizeResize");          // C 5510
	  AUTO_STRING! (SynchronizeResize, c"SynchronizeResize");          // C 5511

	  let value: Lisp_Object = gui_display_get_resource (dpyinfo,
							     synchronizeResize,
							     SynchronizeResize,
							     Qnil (), Qnil ());  // C 5513-5516

	  XSyncIntToValue (&raw mut initial_value, 0);                     // C 5518
	  counters[0] = FRAME_X_BASIC_COUNTER! (f)
	    = XSyncCreateCounter (FRAME_X_DISPLAY! (f),
				  initial_value);                          // C 5519-5522

	  if STRINGP (value) && strcmp (SSDATA (value), c"extended".as_ptr ()) == 0
	    { counters[1] = FRAME_X_EXTENDED_COUNTER! (f)
		= XSyncCreateCounter (FRAME_X_DISPLAY! (f),
				      initial_value); }                    // C 5524-5528

	  (*((*f).output_data.x as *mut x_output)).current_extended_counter_value
	    = initial_value;                                              // C 5530-5531

	  XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),
			   (*dpyinfo).Xatom_net_wm_sync_request_counter,
			   XA_CARDINAL as Atom, 32, PropModeReplace,
			   &raw mut counters as *mut c_uchar,
			   (if STRINGP (value)
			       && strcmp (SSDATA (value), c"extended".as_ptr ()) == 0
			     { 2 } else { 1 }));                          // C 5533-5538

#[cfg (all (feature = "have_xsynctriggerfence", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
	  { x_sync_init_fences (f); }                                     // C 5542
	}
      }
  }

  unblock_input ();                                                         // C 5548

  /* Set whether or not frame synchronization is enabled.  */
  gui_default_parameter (f, parms, Quse_frame_synchronization (), Qt (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);  // C 5551-5552

  /* Works iff frame has been already mapped.  */
  gui_default_parameter (f, parms, Qskip_taskbar (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);  // C 5555-5556
  /* The `z-group' parameter works only for visible frames.  */
  gui_default_parameter (f, parms, Qz_group (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_SYMBOL);   // C 5558-5559

  /* Initialize `default-minibuffer-frame' in case this is the first
     frame on this terminal.  */
  /* C 5564-5565: `KVAR (kb, Vdefault_minibuffer_frame)` is keyboard.h 38's token-pasting
     macro (`(kboard)->field ## _`), which Rust cannot spell; keyboard.h 160's member is
     `Vdefault_minibuffer_frame_`, so the port writes the member (the `src/androidfns.rs`
     1550-1554 shape; `KVAR` is in `docs/ports/excluded-macros.txt`).  */
  if FRAME_HAS_MINIBUF_P (f)                                                // C 5563
      && (!FRAMEP ((*kb).Vdefault_minibuffer_frame_)
	  || !FRAME_LIVE_P (XFRAME ((*kb).Vdefault_minibuffer_frame_)))
    { kset_default_minibuffer_frame (kb, frame); }                          // C 5566

  /* All remaining specified parameters, which have not been "used" by
     gui_display_get_arg and friends, now go in the misc. alist of the
     frame.  */
  tem = parms;                                                              // C 5571
  while CONSP (tem)
    {
      if CONSP (XCAR (tem)) && !NILP (XCAR (XCAR (tem)))                    // C 5572
	{ fset_param_alist (f, Fcons (XCAR (tem), (*f).param_alist)); }     // C 5573
      tem = XCDR (tem);                                                     // C 5571
    }

  /* Make sure windows on this frame appear in calls to next-window
     and similar functions.  */
  Vwindow_list = Qnil ();                                                   // C 5577

  unbind_to (count, frame)                                                  // C 5579
});

/* C 5583-5758: the first nine subrs of the remainder - `xw-color-defined-p`, `xw-color-values`,
   `xw-display-color-p`, `x-display-grayscale-p`, `x-display-pixel-width`,
   `x-display-pixel-height`, `x-display-planes`, `x-display-color-cells` and
   `x-server-max-request-size`.  All nine are `DEFUN`s of this file whose doc strings stay
   verbatim; `x_display_pixel_width`/`x_display_pixel_height` are xterm.c 18491/18482's
   definitions, declared at this use site below.  */
/* C: xterm.h 2011/2004: `extern int x_display_pixel_width (struct x_display_info *)` and
   `… x_display_pixel_height (…)` - the two rows `docs/ports/xfns.md` owes `include/xterm.rs`
   (the `src/xfns.rs` use-site block's shape, P-72 rule 4).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_display_pixel_width (dpyinfo: *mut x_display_info) -> c_int;
  pub fn x_display_pixel_height (dpyinfo: *mut x_display_info) -> c_int;
}

/* C 5583-5597: `DEFUN ("xw-color-defined-p", Fxw_color_defined_p, Sxw_color_defined_p, 1, 2, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("xw-color-defined-p", Fxw_color_defined_p, Sxw_color_defined_p,
	1, 2, 0,
	doc: "Internal function called by `color-defined-p'.\n(Note that the Nextstep version of this function ignores FRAME.)  ",
	(color: Lisp_Object, frame: Lisp_Object),
	{
  let mut foo: XColor = std::mem::zeroed ();                            // C 5588 (the callee fills it)
  let f: *mut frame = decode_window_system_frame (frame);               // C 5589

  CHECK_STRING (color);                                                 // C 5591

  /* C's `Emacs_Color *` parameter is dispextern.h 38's `typedef XColor Emacs_Color` for this arm,
     so the local is C's `XColor` and the conversion is at the call site (this file's 969-974
     shape; the owed mirror row is in `docs/ports/xfns.md`).  */
  if x_defined_color (f, SSDATA (color), &raw mut foo as *mut Emacs_Color, false, false)  // C 5593
    { return Qt (); }                                                   // C 5594
  else
    { return Qnil (); }                                                 // C 5596
});

/* C 5599-5613: `DEFUN ("xw-color-values", Fxw_color_values, Sxw_color_values, 1, 2, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("xw-color-values", Fxw_color_values, Sxw_color_values,
	1, 2, 0,
	doc: "Internal function called by `color-values'.\n(Note that the Nextstep version of this function ignores FRAME.)  ",
	(color: Lisp_Object, frame: Lisp_Object),
	{
  let mut foo: XColor = std::mem::zeroed ();                            // C 5604 (the callee fills it)
  let f: *mut frame = decode_window_system_frame (frame);               // C 5605

  CHECK_STRING (color);                                                 // C 5607

  if x_defined_color (f, SSDATA (color), &raw mut foo as *mut Emacs_Color, false, false)  // C 5609
    { return list3i (foo.red as EMACS_INT, foo.green as EMACS_INT,      // C 5610
		     foo.blue as EMACS_INT); }
  else
    { return Qnil (); }                                                 // C 5612
});

/* C 5615-5635: `DEFUN ("xw-display-color-p", Fxw_display_color_p, Sxw_display_color_p, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("xw-display-color-p", Fxw_display_color_p, Sxw_display_color_p,
	0, 1, 0,
	doc: "Internal function called by `display-color-p'.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5619

  if (*dpyinfo).n_planes <= 2                                           // C 5621
    { return Qnil (); }                                                 // C 5622

  use crate::x11_constants::{DirectColor, PseudoColor, StaticColor, TrueColor};

  match (*dpyinfo).visual_info.class                                    // C 5624
    {
      StaticColor                                                       // C 5626
	| PseudoColor                                                   // C 5627
	| TrueColor                                                     // C 5628
	| DirectColor                                                   // C 5629
	=> { return Qt (); }                                            // C 5630

      _                                                                 // C 5632
	=> { return Qnil (); }                                          // C 5633
    }
});

/* C 5637-5664: `DEFUN ("x-display-grayscale-p", Fx_display_grayscale_p, Sx_display_grayscale_p, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-grayscale-p", Fx_display_grayscale_p, Sx_display_grayscale_p,
	0, 1, 0,
	doc: "Return t if the X display supports shades of gray.\nNote that color displays do support shades of gray.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5646

  if (*dpyinfo).n_planes <= 1                                           // C 5648
    { return Qnil (); }                                                 // C 5649

  use crate::x11_constants::{DirectColor, GrayScale, PseudoColor, StaticColor, StaticGray,
					     TrueColor};

  match (*dpyinfo).visual_info.class                                    // C 5651
    {
      StaticColor                                                       // C 5653
	| PseudoColor                                                   // C 5654
	| TrueColor                                                     // C 5655
	| DirectColor                                                   // C 5656
	| StaticGray                                                    // C 5657
	| GrayScale                                                     // C 5658
	=> { return Qt (); }                                            // C 5659

      _                                                                 // C 5661
	=> { return Qnil (); }                                          // C 5662
    }
});

/* C 5666-5682: `DEFUN ("x-display-pixel-width", Fx_display_pixel_width, Sx_display_pixel_width, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-pixel-width", Fx_display_pixel_width, Sx_display_pixel_width,
	0, 1, 0,
	doc: "Return the width in pixels of the X display TERMINAL.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n(On MS Windows, this function does not accept terminal objects.)\n\nOn \"multi-monitor\" setups this refers to the pixel width for all\nphysical monitors associated with TERMINAL.  To get information for\neach physical monitor, use `display-monitor-attributes-list'.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5679

  return make_fixnum (x_display_pixel_width (dpyinfo) as EMACS_INT);    // C 5681
});

/* C 5684-5700: `DEFUN ("x-display-pixel-height", Fx_display_pixel_height, Sx_display_pixel_height, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-pixel-height", Fx_display_pixel_height, Sx_display_pixel_height,
	0, 1, 0,
	doc: "Return the height in pixels of the X display TERMINAL.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n(On MS Windows, this function does not accept terminal objects.)\n\nOn \"multi-monitor\" setups this refers to the pixel height for all\nphysical monitors associated with TERMINAL.  To get information for\neach physical monitor, use `display-monitor-attributes-list'.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5697

  return make_fixnum (x_display_pixel_height (dpyinfo) as EMACS_INT);   // C 5699
});

/* C 5702-5714: `DEFUN ("x-display-planes", Fx_display_planes, Sx_display_planes, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-planes", Fx_display_planes, Sx_display_planes,
	0, 1, 0,
	doc: "Return the number of bitplanes of the X display TERMINAL.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n(On MS Windows, this function does not accept terminal objects.)  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5711

  return make_fixnum ((*dpyinfo).n_planes as EMACS_INT);                // C 5713
});

/* C 5716-5741: `DEFUN ("x-display-color-cells", Fx_display_color_cells, Sx_display_color_cells, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-color-cells", Fx_display_color_cells, Sx_display_color_cells,
	0, 1, 0,
	doc: "Return the number of color cells of the X display TERMINAL.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n(On MS Windows, this function does not accept terminal objects.)  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5725

  use crate::x11_constants::{DirectColor, TrueColor};

  if (*dpyinfo).visual_info.class != TrueColor                          // C 5727
      && (*dpyinfo).visual_info.class != DirectColor                    // C 5728
    { return make_fixnum ((*dpyinfo).visual_info.colormap_size as EMACS_INT); }  // C 5729

  let mut nr_planes: c_int = (*dpyinfo).n_planes;                       // C 5731

  /* Truncate nr_planes to 24 to avoid integer overflow.  Some
     displays says 32, but only 24 bits are actually significant.
     There are only very few and rare video cards that have more than
     24 significant bits.  Also 24 bits is more than 16 million
     colors, it "should be enough for everyone".  */
  if nr_planes > 24 { nr_planes = 24; }                                 // C 5738

  return make_fixnum ((1 << nr_planes) as EMACS_INT);                   // C 5740
});

/* C 5743-5758: `DEFUN ("x-server-max-request-size", Fx_server_max_request_size, Sx_server_max_request_size, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-server-max-request-size", Fx_server_max_request_size, Sx_server_max_request_size,
	0, 1, 0,
	doc: "Return the maximum request size of the X server of display TERMINAL.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nOn MS Windows, this function just returns 1.\nOn Nextstep and PGTK, this function just returns nil.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5755

  return make_fixnum (MAXREQUEST! ((*dpyinfo).display) as EMACS_INT);   // C 5757
});

/* The seven Xlib names C 5776-5888 reads.  Xlib.h defines six of them as *macros* over private
   `_XDisplay` fields (Xlib.h 103-107, 131-132) and the mirror's `_XDisplay` is opaque
   (`include/xlib.rs` 1046), so the port calls the header's own function forms through a
   `macro_rules!` that keeps C's name (the `MAXREQUEST`/`XMaxRequestSize` shape at 152-156); the
   two `Screen` macros are spelled through the mirror's own members.  Two rows are owed here:
   `include/xlib.rs`'s six `extern` rows, and the two `macros.rs`-style define bodies.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XServerVendor (dpy: *mut Display) -> *mut c_char;              // Xlib.h 1812
  pub fn XProtocolVersion (dpy: *mut Display) -> c_int;                 // Xlib.h 2903
  pub fn XProtocolRevision (dpy: *mut Display) -> c_int;                // Xlib.h 2899
  pub fn XVendorRelease (dpy: *mut Display) -> c_int;                   // Xlib.h 3520
  pub fn XScreenCount (dpy: *mut Display) -> c_int;                     // Xlib.h 3135
}
#[cfg (feature = "have_x_windows")]
macro_rules! ServerVendor { ($dpy:expr) => { XServerVendor ($dpy) }; }          // Xlib.h 104
#[cfg (feature = "have_x_windows")]
macro_rules! ProtocolVersion { ($dpy:expr) => { XProtocolVersion ($dpy) }; }    // Xlib.h 105
#[cfg (feature = "have_x_windows")]
macro_rules! ProtocolRevision { ($dpy:expr) => { XProtocolRevision ($dpy) }; }  // Xlib.h 106
#[cfg (feature = "have_x_windows")]
macro_rules! VendorRelease { ($dpy:expr) => { XVendorRelease ($dpy) }; }        // Xlib.h 107
#[cfg (feature = "have_x_windows")]
macro_rules! ScreenCount { ($dpy:expr) => { XScreenCount ($dpy) }; }            // Xlib.h 103
/* Xlib.h 131-132: `#define WidthMMOfScreen(s) ((s)->mwidth)` /
   `#define HeightMMOfScreen(s) ((s)->mheight)` - the mirror's `Screen` carries both members
   (`include/xlib.rs` 1130-1131), so the two macros keep C's name and read them.  */
#[cfg (feature = "have_x_windows")]
macro_rules! WidthMMOfScreen { ($s:expr) => { (*$s).mwidth }; }                 // Xlib.h 131
#[cfg (feature = "have_x_windows")]
macro_rules! HeightMMOfScreen { ($s:expr) => { (*$s).mheight }; }               // Xlib.h 132

/* C 5760-5889: the next six subrs of the remainder - `x-server-vendor`, `x-server-version`,
   `x-server-input-extension-version`, `x-display-screens`, `x-display-mm-height` and
   `x-display-mm-width` - all `DEFUN`s of this file whose doc strings stay verbatim (decoded per
   `lib-src/make-docfile.c`'s rule).  */

/* C 5760-5781: `DEFUN ("x-server-vendor", Fx_server_vendor, Sx_server_vendor, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-server-vendor", Fx_server_vendor, Sx_server_vendor,
	0, 1, 0,
	doc: "Return the \"vendor ID\" string of the GUI software on TERMINAL.\n\n(Labeling every distributor as a \"vendor\" embodies the false assumption\nthat operating systems cannot be developed and distributed noncommercially.)\nThe optional argument TERMINAL specifies which display to ask about.\n\nFor GNU and Unix systems, this queries the X server software.\nFor Android systems, value is the manufacturer who developed the Android\nsystem that is being used.\nFor MS Windows and Nextstep the result is hard-coded.\n\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5776
  let mut vendor: *const c_char = ServerVendor! ((*dpyinfo).display);    // C 5777

  if vendor.is_null () { vendor = c"".as_ptr (); }                      // C 5779
  return build_string (vendor);                                         // C 5780
});

/* C 5783-5808: `DEFUN ("x-server-version", Fx_server_version, Sx_server_version, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-server-version", Fx_server_version, Sx_server_version,
	0, 1, 0,
	doc: "Return the version numbers of the GUI software on TERMINAL.\nThe value is a list of three integers specifying the version of the GUI\nsoftware in use.\n\nFor GNU and Unix system, the first 2 numbers are the version of the X\nProtocol used on TERMINAL and the 3rd number is the distributor-specific\nrelease number.  For MS Windows, the 3 numbers report the OS major and\nminor version and build number.  For Nextstep, the first 2 numbers are\nhard-coded and the 3rd represents the OS version.  For Haiku, all 3\nnumbers are hard-coded.  For Android, the first number represents the\nAndroid API level, and the next two numbers are all zero.\n\nSee also the function `x-server-vendor'.\n\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5803
  let dpy: *mut Display = (*dpyinfo).display;                           // C 5804

  return list3i (ProtocolVersion! (dpy) as EMACS_INT,                    // C 5806
		 ProtocolRevision! (dpy) as EMACS_INT,
		 VendorRelease! (dpy) as EMACS_INT);                       // C 5807
});

/* C 5810-5829: `DEFUN ("x-server-input-extension-version", Fx_server_input_extension_version, Sx_server_input_extension_version, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-server-input-extension-version", Fx_server_input_extension_version, Sx_server_input_extension_version,
	0, 1, 0,
	doc: "Return the version of the X Input Extension supported by TERMINAL.\nThe value is nil if TERMINAL's X server doesn't support the X Input\nExtension extension, or if Emacs doesn't support the version present\non that server.  Otherwise, the return value is a list of the major\nand minor versions of the X Input Extension extension running on that\nserver.  ",
	(terminal: Lisp_Object),
	{
#[cfg (feature = "have_xinput2")]
  {
    let dpyinfo: *mut x_display_info = check_x_display_info (terminal);  // C 5821

    return if (*dpyinfo).supports_xi2                                    // C 5823
      { list2i (2, (*dpyinfo).xi2_version as EMACS_INT) }                // C 5824
    else
      { Qnil () };                                                       // C 5825
  }
#[cfg (not (feature = "have_xinput2"))]
  { return Qnil (); }                                                    // C 5827
});

/* C 5831-5849: `DEFUN ("x-display-screens", Fx_display_screens, Sx_display_screens, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-screens", Fx_display_screens, Sx_display_screens,
	0, 1, 0,
	doc: "Return the number of screens on the X server of display TERMINAL.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nOn PGTK and Nextstep, \"screen\" is in X terminology, not that of Wayland\nand Nextstep, respectively.\n\nOn MS Windows, this function just returns 1.\n\nFor the number of physical monitors, use `(length\n(display-monitor-attributes-list TERMINAL))' instead.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5846

  return make_fixnum (ScreenCount! ((*dpyinfo).display) as EMACS_INT);   // C 5848
});

/* C 5851-5869: `DEFUN ("x-display-mm-height", Fx_display_mm_height, Sx_display_mm_height, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-mm-height", Fx_display_mm_height, Sx_display_mm_height,
	0, 1, 0,
	doc: "Return the height in millimeters of the X display TERMINAL.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n(On MS Windows, this function does not accept terminal objects.)\n\nOn \"multi-monitor\" setups this refers to the height in millimeters for\nall physical monitors associated with TERMINAL.  To get information\nfor each physical monitor, use `display-monitor-attributes-list'.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5863

  if (*dpyinfo).screen_mm_height != 0                                   // C 5865
    { return make_fixnum ((*dpyinfo).screen_mm_height as EMACS_INT); }   // C 5866

  return make_fixnum (HeightMMOfScreen! ((*dpyinfo).screen) as EMACS_INT);  // C 5868
});

/* C 5871-5889: `DEFUN ("x-display-mm-width", Fx_display_mm_width, Sx_display_mm_width, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-mm-width", Fx_display_mm_width, Sx_display_mm_width,
	0, 1, 0,
	doc: "Return the width in millimeters of the X display TERMINAL.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n(On MS Windows, this function does not accept terminal objects.)\n\nOn \"multi-monitor\" setups this refers to the width in millimeters for\nall physical monitors associated with TERMINAL.  To get information\nfor each physical monitor, use `display-monitor-attributes-list'.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);   // C 5883

  if (*dpyinfo).screen_mm_width != 0                                    // C 5885
    { return make_fixnum ((*dpyinfo).screen_mm_width as EMACS_INT); }    // C 5886

  return make_fixnum (WidthMMOfScreen! ((*dpyinfo).screen) as EMACS_INT);  // C 5888
});

/* C 5891-5991: the next three subrs of the remainder - `x-display-backing-store` (C 5891-5928),
   `x-display-visual-class` (C 5930-5973) and `x-display-save-under` (C 5975-5991).  The two
   `switch` bodies are the port's `match` arms plus `_` (PORTING.md 2.2), whose
   `break`/`case`/`default`/`switch` tokens `docs/ports/macro-body-notes.tsv` answers.  */
/* X11/X.h 440-441: `#define WhenMapped 1` / `#define Always 2` (measured on this machine's
   header) - the two `BackingStore` values C 5909-5920 compares against; their partner
   `NotUseful 0` already stands at 6077.  No mirror carries the three.  Xlib.h 269-270's two
   `Screen` macros keep C's name and read the mirror's members (`include/xlib.rs` 1142-1143).  */
#[cfg (feature = "have_x_windows")]
pub const WhenMapped: c_int = 1;
#[cfg (feature = "have_x_windows")]
pub const Always: c_int = 2;
#[cfg (feature = "have_x_windows")]
macro_rules! DoesBackingStore { ($s:expr) => { (*$s).backing_store }; }   // Xlib.h 269
#[cfg (feature = "have_x_windows")]
macro_rules! DoesSaveUnders { ($s:expr) => { (*$s).save_unders }; }       // Xlib.h 270

/* C 5891-5928: `DEFUN ("x-display-backing-store", Fx_display_backing_store, Sx_display_backing_store, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-backing-store", Fx_display_backing_store, Sx_display_backing_store,
	0, 1, 0,
	doc: "Return an indication of whether X display TERMINAL does backing store.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nThe value may be `always', `when-mapped', or `not-useful'.\n\nOn Nextstep and PGTK, the value may be `buffered', `retained', or\n`non-retained'.\n\nOn MS Windows, this returns nothing useful.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);    // C 5906
  let result: Lisp_Object;                                               // C 5907

  match DoesBackingStore! ((*dpyinfo).screen)                            // C 5909
    {
      Always                                                             // C 5911
	=> { result = Qalways (); }                                          // C 5912
      WhenMapped                                                         // C 5915
	=> { result = Qwhen_mapped (); }                                     // C 5916
      NotUseful                                                          // C 5919
	=> { result = Qnot_useful (); }                                      // C 5920

      _                                                                  // C 5923
	=> { error (c"Strange value for BackingStore parameter of screen".as_ptr ()); }  // C 5924
    }

  return result;                                                         // C 5927
});

/* C 5930-5973: `DEFUN ("x-display-visual-class", Fx_display_visual_class, Sx_display_visual_class, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-visual-class", Fx_display_visual_class, Sx_display_visual_class,
	0, 1, 0,
	doc: "Return the visual class of the X display TERMINAL.\nThe value is one of the symbols `static-gray', `gray-scale',\n`static-color', `pseudo-color', `true-color', or `direct-color'.\n(On MS Windows, the second and last result above are not possible.)\n\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n(On MS Windows, this function does not accept terminal objects.)\n\nOn PGTK, always return `true-color'.  ",
	(terminal: Lisp_Object),
	{
  use crate::x11_constants::{DirectColor, GrayScale, PseudoColor, StaticColor, StaticGray,
				     TrueColor};

  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);    // C 5945
  let result: Lisp_Object;                                               // C 5946

  match (*dpyinfo).visual_info.class                                     // C 5948
    {
      StaticGray                                                         // C 5950
	=> { result = Qstatic_gray (); }                                     // C 5951
      GrayScale                                                          // C 5953
	=> { result = Qgray_scale (); }                                      // C 5954
      StaticColor                                                        // C 5956
	=> { result = Qstatic_color (); }                                    // C 5957
      PseudoColor                                                        // C 5959
	=> { result = Qpseudo_color (); }                                    // C 5960
      TrueColor                                                          // C 5962
	=> { result = Qtrue_color (); }                                      // C 5963
      DirectColor                                                        // C 5965
	=> { result = Qdirect_color (); }                                    // C 5966

      _                                                                  // C 5968
	=> { error (c"Display has an unknown visual class".as_ptr ()); }     // C 5969
    }

  return result;                                                         // C 5972
});

/* C 5975-5991: `DEFUN ("x-display-save-under", Fx_display_save_under, Sx_display_save_under, 0, 1, 0, …)`.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-save-under", Fx_display_save_under, Sx_display_save_under,
	0, 1, 0,
	doc: "Return t if the X display TERMINAL supports the save-under feature.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nOn MS Windows, this just returns nil.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);    // C 5985

  if DoesSaveUnders! ((*dpyinfo).screen) == True                         // C 5987
    { return Qt (); }
  else
    { return Qnil (); }                                                   // C 5990
});

/* C 5993-6123: `#if !(defined USE_GTK && defined HAVE_GTK3)` ... `static bool
   x_get_net_workarea (struct x_display_info *dpyinfo, XRectangle *rect)` ... `#endif` - the two
   arms of the `_NET_WORKAREA` query (C's own `#ifndef USE_XCB` pair).  The declarations it needs
   are this unit's already (`x_catch_errors`/`x_had_errors_p`/`x_uncatch_errors` at 313-317,
   `XGetWindowProperty`/`XFree` at 222/231, the xcb family at 260-275 and `include/xlib.rs`
   575-908); `False` is `include/xterm.rs` 79's constant.  */
/// C 5995-5997: "Store the geometry of the workarea on display DPYINFO into *RECT.  Return false
/// if and only if the workarea information cannot be obtained via the _NET_WORKAREA root window
/// property."  A `static` function whose address never leaves the unit, so a plain `unsafe fn`
/// (the file's shape for its other statics).
#[cfg (all (feature = "have_x_windows", not (all (feature = "use_gtk", feature = "have_gtk3"))))]
unsafe fn x_get_net_workarea (dpyinfo: *mut x_display_info, rect: *mut XRectangle) -> bool
{
#[cfg (not (feature = "use_xcb"))]
  {
    use crate::x11_constants::{Success, XA_CARDINAL};

    let dpy: *mut Display = (*dpyinfo).display;                           // C 6003
    let mut offset: c_long;                                               // C 6004
    let mut max_len: c_long;
    let mut target_type: Atom;                                            // C 6005
    let mut actual_type: Atom = 0;                 /* XGetWindowProperty fills these four.  */
    let mut actual_size: c_ulong = 0;                                     // C 6006
    let mut bytes_remaining: c_ulong = 0;
    let mut rc: c_int;                                                    // C 6007
    let mut actual_format: c_int = 0;
    let mut tmp_data: *mut c_uchar = std::ptr::null_mut ();               // C 6008
    let mut result: bool = false;                                         // C 6009

    x_catch_errors (dpy);                                                 // C 6011
    offset = 0;                                                           // C 6012
    max_len = 1;                                                          // C 6013
    target_type = XA_CARDINAL;                                            // C 6014
    rc = XGetWindowProperty (dpy, (*dpyinfo).root_window,                  // C 6015-6019
			     (*dpyinfo).Xatom_net_current_desktop,
			     offset, max_len, False, target_type,
			     &raw mut actual_type, &raw mut actual_format, &raw mut actual_size,
			     &raw mut bytes_remaining, &raw mut tmp_data);
    if rc == Success && actual_type == target_type && !x_had_errors_p (dpy)   // C 6020
	&& actual_format == 32 && actual_size == max_len as c_ulong       // C 6021
      {
	let current_desktop: c_long = *tmp_data.cast::<c_long> ();        // C 6023

	XFree (tmp_data as *mut c_void);                                  // C 6025
	tmp_data = std::ptr::null_mut ();                                 // C 6026

	offset = 4 * current_desktop;                                     // C 6028
	max_len = 4;                                                      // C 6029
	rc = XGetWindowProperty (dpy, (*dpyinfo).root_window,              // C 6030-6034
				 (*dpyinfo).Xatom_net_workarea,
				 offset, max_len, False, target_type,
				 &raw mut actual_type, &raw mut actual_format,
				 &raw mut actual_size, &raw mut bytes_remaining,
				 &raw mut tmp_data);
	if rc == Success && actual_type == target_type && !x_had_errors_p (dpy)   // C 6035
	    && actual_format == 32 && actual_size == max_len as c_ulong       // C 6036
	  {
	    let values: *mut c_long = tmp_data.cast::<c_long> ();          // C 6038

	    (*rect).x = *values.offset (0) as c_short;                     // C 6040
	    (*rect).y = *values.offset (1) as c_short;                     // C 6041
	    (*rect).width = *values.offset (2) as c_ushort;                // C 6042
	    (*rect).height = *values.offset (3) as c_ushort;               // C 6043

	    XFree (tmp_data as *mut c_void);                               // C 6045
	    tmp_data = std::ptr::null_mut ();                              // C 6046

	    result = true;                                                 // C 6048
	  }
      }
    if !tmp_data.is_null ()                                               // C 6051
      { XFree (tmp_data as *mut c_void); }
    x_uncatch_errors ();                                                  // C 6053

    return result;                                                        // C 6055
  }
#[cfg (feature = "use_xcb")]
  {
    use crate::x11_constants::XA_CARDINAL;

    let mut current_desktop_cookie: xcb_get_property_cookie_t;            // C 6057
    let mut workarea_cookie: xcb_get_property_cookie_t;                   // C 6058
    let mut reply: *mut xcb_get_property_reply_t;                         // C 6059
    let mut error: *mut xcb_generic_error_t = std::ptr::null_mut ();      // C 6060 (the callee sets it)
    let mut rc: bool;                                                     // C 6061
    let mut current_workspace: u32 = 0;                                   // C 6062 (read only when rc)
    let mut values: *mut u32;

    current_desktop_cookie
      = xcb_get_property ((*dpyinfo).xcb_connection, 0,                    // C 6064-6068
			  (*dpyinfo).root_window as xcb_window_t,
			  (*dpyinfo).Xatom_net_current_desktop as xcb_atom_t,
			  XA_CARDINAL as xcb_atom_t, 0, 1);

    workarea_cookie
      = xcb_get_property ((*dpyinfo).xcb_connection, 0,                    // C 6070-6074
			  (*dpyinfo).root_window as xcb_window_t,
			  (*dpyinfo).Xatom_net_workarea as xcb_atom_t,
			  XA_CARDINAL as xcb_atom_t, 0, UINT32_MAX);

    reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,             // C 6076-6077
				    current_desktop_cookie, &raw mut error);
    rc = true;                                                             // C 6078

    if reply.is_null ()                                                    // C 6080
      { free (error as *mut c_void); rc = false; }                         // C 6081
    else
      {
	if xcb_get_property_value_length (reply) != 4                      // C 6084
	    || (*reply).r#type != XA_CARDINAL as xcb_atom_t
	    || (*reply).format != 32
	  { rc = false; }                                                  // C 6086
	else
	  { current_workspace = *(xcb_get_property_value (reply) as *mut u32); }  // C 6088

	free (reply as *mut c_void);                                       // C 6090
      }

    reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,             // C 6093-6094
				    workarea_cookie, &raw mut error);

    if reply.is_null ()                                                    // C 6096
      { free (error as *mut c_void); rc = false; }                         // C 6097
    else
      {
	let value_count: usize = xcb_get_property_value_length (reply) as usize  // C 6100-6101
	  / std::mem::size_of::<u32> ();

	if rc && (*reply).r#type == XA_CARDINAL as xcb_atom_t && (*reply).format == 32   // C 6103
	    && (current_workspace as usize) < value_count / 4              // C 6104
	  {
	    let offset: usize = 4 * current_workspace as usize;            // C 6106
	    values = xcb_get_property_value (reply) as *mut u32;           // C 6107

	    (*rect).x = *values.offset (offset as isize) as c_short;       // C 6109
	    (*rect).y = *values.offset ((offset + 1) as isize) as c_short;  // C 6110
	    (*rect).width = *values.offset ((offset + 2) as isize) as c_ushort;   // C 6111
	    (*rect).height = *values.offset ((offset + 3) as isize) as c_ushort;  // C 6112
	  }
	else
	  { rc = false; }                                                  // C 6115

	free (reply as *mut c_void);                                       // C 6117
      }

    return rc;                                                             // C 6120
  }
}

/* C 6125-6215: `#ifndef USE_GTK` and the two statics under it - `x_get_monitor_for_frame`
   (C 6127-6188) and `x_make_monitor_attribute_list` (C 6190-6215).  Both name `struct
   MonitorInfo`, which frame.h 1926 guards with `#ifdef HAVE_WINDOW_SYSTEM` (the mirror row is
   `include/frame.rs` 1937), so C's own `#ifndef USE_GTK` is *implicitly* narrowed to that
   feature; the guard below spells the pair out, which is why neither function lands in any of the
   four arms measured so far (`have_window_system` is absent from all four).  */
/// C 6127: `/* Return monitor number where F is "most" or closest to.  */`
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system"))]
unsafe fn x_get_monitor_for_frame (f: *mut frame, monitors: *mut MonitorInfo,
				   n_monitors: c_int) -> c_int
{
  let mut frect: XRectangle = std::mem::zeroed ();                        // C 6133
  let mut area: c_int = 0;                                                // C 6134
  let mut dist: c_int = -1;
  let mut best_area: c_int = -1;                                          // C 6135
  let mut best_dist: c_int = -1;
  let mut i: c_int;                                                       // C 6136

  if n_monitors == 1 { return 0; }                                        // C 6138
  frect.x = (*f).left_pos as c_short;                                     // C 6139
  frect.y = (*f).top_pos as c_short;                                      // C 6140
  frect.width = FRAME_PIXEL_WIDTH (f) as c_ushort;                        // C 6141
  frect.height = FRAME_PIXEL_HEIGHT (f) as c_ushort;                      // C 6142

  i = 0;
  while i < n_monitors                                                   // C 6144
    {
      let mi: *mut MonitorInfo = monitors.offset (i as isize);            // C 6146
      let mut res: XRectangle = std::mem::zeroed ();                      // C 6147
      let mut a: c_int = 0;                                               // C 6148

      if (*mi).geom.width == 0 { i += 1; continue; }                      // C 6150

      /* `gui_intersect_rectangles`'s three parameters are C's `Emacs_Rectangle *`
	 (dispextern.h 3676); in the x11 arm that name is dispextern.h 38's `typedef XRectangle
	 Emacs_Rectangle`, and the mirror carries the other arm's record, so the two `XRectangle`
	 locals convert at the call site (the owed row `docs/ports/xfns.md` names).  */
      if gui_intersect_rectangles (&raw const (*mi).geom,                   // C 6152
				   &raw const frect as *const Emacs_Rectangle,
				   &raw mut res as *mut Emacs_Rectangle)
	{
	  a = res.width as c_int * res.height as c_int;                     // C 6154
	  if a > area                                                       // C 6155
	    {
	      area = a;                                                     // C 6157
	      best_area = i;                                                // C 6158
	    }
	}

      if a == 0 && area == 0                                            // C 6162
	{
	  let mut dx: c_int;                                                // C 6164
	  let mut dy: c_int;
	  let mut d: c_int;
	  /* C's operands are an `XRectangle`'s `short`/`unsigned short` members and an
	     `Emacs_Rectangle`'s `int`s; the port writes the promotions the C compilers imply.  */
	  if (frect.x as c_int + frect.width as c_int) < (*mi).geom.x as c_int   // C 6165
	    { dx = ((*mi).geom.x as c_int - frect.x as c_int) + frect.width as c_int; }
	  else if frect.x as c_int > ((*mi).geom.x as c_int + (*mi).geom.width as c_int)  // C 6167
	    { dx = (frect.x as c_int - (*mi).geom.x as c_int) + (*mi).geom.width as c_int; }
	  else
	    { dx = 0; }                                                     // C 6170
	  if (frect.y as c_int + frect.height as c_int) < (*mi).geom.y as c_int  // C 6171
	    { dy = ((*mi).geom.y as c_int - frect.y as c_int) + frect.height as c_int; }
	  else if frect.y as c_int > ((*mi).geom.y as c_int + (*mi).geom.height as c_int)  // C 6173
	    { dy = (frect.y as c_int - (*mi).geom.y as c_int) + (*mi).geom.height as c_int; }
	  else
	    { dy = 0; }                                                     // C 6176

	  d = dx * dx + dy * dy;                                            // C 6178
	  if dist == -1 || dist > d                                         // C 6179
	    {
	      dist = d;                                                     // C 6181
	      best_dist = i;                                                // C 6182
	    }
	}

      i += 1;                                                           // C 6144's `++i`
    }

  return if best_area != -1 { best_area }                               // C 6187
    else if best_dist != -1 { best_dist }
    else { 0 };
}

#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system"))]
unsafe fn x_make_monitor_attribute_list (monitors: *mut MonitorInfo, n_monitors: c_int,
					 primary_monitor: c_int,
					 dpyinfo: *mut x_display_info,
					 source: *const c_char) -> Lisp_Object
{
  let monitor_frames: Lisp_Object = make_nil_vector (n_monitors as isize);   // C 6197
  let mut frame: Lisp_Object;                                                // C 6198
  let mut rest: Lisp_Object;

  FOR_EACH_FRAME! (rest, frame, {                                          // C 6200
    let f: *mut frame = XFRAME (frame);                                    // C 6202

    if FRAME_X_P (f)                                                       // C 6204
	&& FRAME_DISPLAY_INFO! (f) == dpyinfo                             // C 6205
	&& !FRAME_TOOLTIP_P (f)                                           // C 6206
      {
	let i: c_int = x_get_monitor_for_frame (f, monitors, n_monitors);  // C 6208
	ASET (monitor_frames, i as isize,                                  // C 6209
	      Fcons (frame, AREF (monitor_frames, i as isize)));
      }
  });

  return make_monitor_attribute_list (monitors, n_monitors, primary_monitor,   // C 6213-6214
				      monitor_frames, source);
}

/* C 6217-6297: `x_get_monitor_attributes_fallback` (C 6217-6238) and C 6241-6297's
   `#ifdef HAVE_XINERAMA` with `x_get_monitor_attributes_xinerama` (C 6242-6296).  Both sit under
   C 6125's `#ifndef USE_GTK` and name `struct MonitorInfo` (frame.h 1926's
   `HAVE_WINDOW_SYSTEM`), so their guard is the slice 15 pair; the Xinerama one adds C 6241's own
   gate.  */
/* X11/extensions/Xinerama.h's record and entry point - the header is the toolchain's and this
   machine has no `<X11/extensions/Xinerama.h>` (`/opt/homebrew/include/X11/extensions/` carries
   XKB/XI/XRes/… and no Xinerama), so no mirror carries them, the rows are owed
   (`docs/ports/xfns.md`) and the record's layout is **UNVERIFIED**: it carries the members the
   C reads (`int screen_number; short x_org, y_org; unsigned short width, height;`) in the
   header's own order, and the size/align pair is the integrator's to measure.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xinerama"))]
#[repr (C)]
pub struct XineramaScreenInfo
{
  pub screen_number: c_int,
  pub x_org: c_short,
  pub y_org: c_short,
  pub width: c_ushort,
  pub height: c_ushort,
}
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xinerama"))]
unsafe extern "C" {
  /// C: `<X11/extensions/Xinerama.h>`: `extern XineramaScreenInfo *XineramaQueryScreens
  /// (Display *, int *);`
  pub fn XineramaQueryScreens (dpy: *mut Display, number: *mut c_int) -> *mut XineramaScreenInfo;
  /// C: `<X11/extensions/Xinerama.h>`: `extern Bool XineramaIsActive (Display *);` - C 6535's
  /// truth test, the dispatcher's `have_xinerama` arm (the same header, so the same home).
  pub fn XineramaIsActive (dpy: *mut Display) -> Bool;
}

#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system"))]
unsafe fn x_get_monitor_attributes_fallback (dpyinfo: *mut x_display_info) -> Lisp_Object
{
  let mut monitor: MonitorInfo = std::mem::zeroed ();                      // C 6220
  let mut workarea_r: XRectangle = std::mem::zeroed ();                    // C 6221

  /* Fallback: treat (possibly) multiple physical monitors as if they
     formed a single monitor as a whole.  This should provide a
     consistent result at least on single monitor environments.  */
  monitor.geom.x = 0;                                                      // C 6226 (C's chain splits)
  monitor.geom.y = 0;
  monitor.geom.width = x_display_pixel_width (dpyinfo) as c_ushort;        // C 6227
  monitor.geom.height = x_display_pixel_height (dpyinfo) as c_ushort;      // C 6228
  monitor.mm_width = WidthMMOfScreen! ((*dpyinfo).screen);                 // C 6229
  monitor.mm_height = HeightMMOfScreen! ((*dpyinfo).screen);               // C 6230
  monitor.name = xstrdup (c"combined screen".as_ptr ());                   // C 6231

  if x_get_net_workarea (dpyinfo, &raw mut workarea_r)                     // C 6233
    { monitor.work = workarea_r; }                                         // C 6234
  else
    { monitor.work = monitor.geom; }                                       // C 6236
  return x_make_monitor_attribute_list (&raw mut monitor, 1, 0, dpyinfo,    // C 6237
					c"fallback".as_ptr ());
}

#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xinerama"))]
unsafe fn x_get_monitor_attributes_xinerama (dpyinfo: *mut x_display_info) -> Lisp_Object
{
  let mut n_monitors: c_int = 0;                                           // C 6245
  let mut i: c_int = 0;
  let mut attributes_list: Lisp_Object = Qnil ();                          // C 6246
  let dpy: *mut Display = (*dpyinfo).display;                              // C 6247
  let info: *mut XineramaScreenInfo                                      // C 6248
    = XineramaQueryScreens (dpy, &raw mut n_monitors);
  let monitors: *mut MonitorInfo;                                          // C 6249
  let mm_width_per_pixel: c_double;                                        // C 6250
  let mm_height_per_pixel: c_double;

  if info.is_null () || n_monitors == 0                                    // C 6252
    {
      if !info.is_null () { XFree (info as *mut c_void); }                 // C 6254-6255
      return attributes_list;                                              // C 6256
    }

  mm_width_per_pixel = WidthMMOfScreen! ((*dpyinfo).screen) as c_double    // C 6259-6260
    / x_display_pixel_width (dpyinfo) as c_double;
  mm_height_per_pixel = HeightMMOfScreen! ((*dpyinfo).screen) as c_double  // C 6261-6262
    / x_display_pixel_height (dpyinfo) as c_double;
  monitors = xcalloc (n_monitors as usize, std::mem::size_of::<MonitorInfo> ())  // C 6263
    as *mut MonitorInfo;
  while i < n_monitors                                                     // C 6264
    {
      let mi: *mut MonitorInfo = monitors.offset (i as isize);             // C 6266
      let mut workarea_r: XRectangle = std::mem::zeroed ();                // C 6267

      (*mi).geom.x = (*info.offset (i as isize)).x_org;                    // C 6269
      (*mi).geom.y = (*info.offset (i as isize)).y_org;                    // C 6270
      (*mi).geom.width = (*info.offset (i as isize)).width;                // C 6271
      (*mi).geom.height = (*info.offset (i as isize)).height;              // C 6272
      (*mi).mm_width                                                       // C 6273
	= ((*mi).geom.width as c_double * mm_width_per_pixel + 0.5) as c_int;
      (*mi).mm_height                                                      // C 6274
	= ((*mi).geom.height as c_double * mm_height_per_pixel + 0.5) as c_int;
      (*mi).name = std::ptr::null_mut ();                                  // C 6275

      /* Xinerama usually have primary monitor first, just use that.  */
      if i == 0 && x_get_net_workarea (dpyinfo, &raw mut workarea_r)       // C 6278
	{
	  (*mi).work = workarea_r;                                         // C 6280
	  if !gui_intersect_rectangles (&raw const (*mi).geom,               // C 6281
				       &raw const (*mi).work,
				       &raw mut (*mi).work)
	    { (*mi).work = (*mi).geom; }                                   // C 6282
	}
      else
	{ (*mi).work = (*mi).geom; }                                       // C 6285
      i += 1;                                                              // C 6264's `++i`
    }
  XFree (info as *mut c_void);                                             // C 6287

  attributes_list = x_make_monitor_attribute_list (monitors,               // C 6289-6293
						   n_monitors,
						   0,
						   dpyinfo,
						   c"Xinerama".as_ptr ());
  free_monitors (monitors, n_monitors);                                    // C 6294
  return attributes_list;                                                  // C 6295
}

/* C 6300-6513: `#ifdef HAVE_XRANDR` and `static Lisp_Object x_get_monitor_attributes_xrandr
   (struct x_display_info *dpyinfo)`.  Its condition has three layers: the outer `HAVE_XRANDR`,
   C 6312's `#if RANDR_MAJOR > 1 || (RANDR_MAJOR == 1 && RANDR_MINOR >= 5)` (the derived
   `have_xrandr_1_5`), C 6314's `#ifdef USE_XCB` inside that, and C 6413-6414's `RANDR13_LIBRARY`
   (the derived `have_xrandr_1_3`).  **Every RandR record and entry point below is UNVERIFIED**:
   this machine has no `<X11/extensions/Xrandr.h>` (`/opt/homebrew/include/X11/extensions/` carries
   XKB/XI/XRes/Xdbe/… and no Xrandr), so each record carries only the members C reads, in the
   header's own order, and the layouts are the integrator's to measure.  C's two `goto`s are Rust
   labelled blocks (the file's 4714 shape): `goto fallback` is `break 'fallback`, `goto out` is
   `break 'out`; C's bare `alloca` at C 6339 is the port's stack pool (PORTING.md P-05, the
   `src/eval.rs` 1579 shape).  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr"))]
pub type RROutput = XID;                    /* C: Xrandr.h: `typedef XID RROutput;`  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr"))]
pub type RRCrtc = XID;                      /* C: Xrandr.h: `typedef XID RRCrtc;`  */
/// C: Xrandr.h's `Connection`: `typedef enum { RR_Connected, RR_Disconnected,
/// RR_UnknownConnection } Connection;` - the value C 6461 compares against.
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr"))]
pub const RR_Disconnected: c_int = 1;
/// C: Xrandr.h's `XRRScreenResources` - only the two members C 6429/6435/6448/6481 read, in the
/// header's own order; the layout is **UNVERIFIED** (no Xrandr.h on this machine).
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr"))]
#[repr (C)]
pub struct XRRScreenResources
{
  pub noutput: c_int,                       /* C: `int noutput;`  */
  pub outputs: *mut RROutput,               /* C: `RROutput *outputs;`  */
}
/// C: Xrandr.h's `XRRMonitorInfo` - the eight members C 6344-6369 read, in the header's own
/// order; the layout is **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr", feature = "have_xrandr_1_5"))]
#[repr (C)]
pub struct XRRMonitorInfo
{
  pub name: Atom,                           /* C: `Atom name;`  */
  pub primary: Bool,                        /* C: `Bool primary, automatic;`  */
  pub x: c_int,                             /* C: `int x, y, width, height;`  */
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub mwidth: c_int,                        /* C: `int mwidth, mheight;`  */
  pub mheight: c_int,
}
/* C: Xrandr.h: `extern XRRMonitorInfo *XRRGetMonitors (Display *, Window, Bool, int *);` and
   `extern void XRRFreeMonitors (XRRMonitorInfo *);` - C 6331 and C 6405's calls, which only
   the `have_xrandr_1_5` arm makes (so they stand under the record's own guard).  Block
   comment, because a doc comment on an `extern` block is the `unused doc comment` warning
   this file's baseline counts.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr", feature = "have_xrandr_1_5"))]
unsafe extern "C" {
  pub fn XRRGetMonitors (dpy: *mut Display, window: Window, get_active: Bool,
			 nmonitors: *mut c_int) -> *mut XRRMonitorInfo;
  pub fn XRRFreeMonitors (monitors: *mut XRRMonitorInfo);
}
/// C: Xrandr.h's `XRROutputInfo` - the five members C 6452-6479 read, in the header's own order;
/// the layout is **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr"))]
#[repr (C)]
pub struct XRROutputInfo
{
  pub crtc: RRCrtc,                         /* C: `RRCrtc crtc;` (C 6461/6463 read it)  */
  pub mm_width: c_int,                      /* C: `int mm_width, mm_height;`  */
  pub mm_height: c_int,
  pub connection: c_int,                    /* C: `Connection connection;` (an enum: `int`)  */
  pub name: *mut c_char,                    /* C: `char *name;`  */
}
/// C: Xrandr.h's `XRRCrtcInfo` - the four members C 6473-6476 read, in the header's own order;
/// the layout is **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr"))]
#[repr (C)]
pub struct XRRCrtcInfo
{
  pub x: c_int,                             /* C: `int x, y;`  */
  pub y: c_int,
  pub width: c_uint,                        /* C: `unsigned int width, height;`  */
  pub height: c_uint,
}
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr"))]
unsafe extern "C" {
  /// C: Xrandr.h: `extern XRRScreenResources *XRRGetScreenResourcesCurrent (Display *, Window);`
  /// - C 6423's call.
  pub fn XRRGetScreenResourcesCurrent (dpy: *mut Display, window: Window)
				       -> *mut XRRScreenResources;
  /// C: Xrandr.h: `extern XRRScreenResources *XRRGetScreenResources (Display *, Window);` - C
  /// 6425/6427's call.
  pub fn XRRGetScreenResources (dpy: *mut Display, window: Window) -> *mut XRRScreenResources;
  /// C: Xrandr.h: `extern void XRRFreeScreenResources (XRRScreenResources *);` - C 6432/6456/6499.
  pub fn XRRFreeScreenResources (resources: *mut XRRScreenResources);
  /// C: Xrandr.h: `extern RROutput XRRGetOutputPrimary (Display *, Window);` - C 6440's call.
  pub fn XRRGetOutputPrimary (dpy: *mut Display, window: Window) -> RROutput;
  /// C: Xrandr.h: `extern XRROutputInfo *XRRGetOutputInfo (Display *, XRRScreenResources *,
  /// RROutput);` - C 6447's call.
  pub fn XRRGetOutputInfo (dpy: *mut Display, resources: *mut XRRScreenResources,
			   output: RROutput) -> *mut XRROutputInfo;
  /// C: Xrandr.h: `extern void XRRFreeOutputInfo (XRROutputInfo *);` - C 6455/6469/6497's calls.
  pub fn XRRFreeOutputInfo (output_info: *mut XRROutputInfo);
  /// C: Xrandr.h: `extern XRRCrtcInfo *XRRGetCrtcInfo (Display *, XRRScreenResources *, RRCrtc);`
  /// - C 6463's call.
  pub fn XRRGetCrtcInfo (dpy: *mut Display, resources: *mut XRRScreenResources,
			 crtc: RRCrtc) -> *mut XRRCrtcInfo;
  /// C: Xrandr.h: `extern void XRRFreeCrtcInfo (XRRCrtcInfo *);` - C 6495's call.
  pub fn XRRFreeCrtcInfo (crtc_info: *mut XRRCrtcInfo);
  /// C: Xlib.h 2319: `extern char *XGetAtomName (Display *, Atom);` - C 6352's call (the non-XCB
  /// side of the 1.5 arm); its row is owed to `include/xlib.rs`.
  pub fn XGetAtomName (dpy: *mut Display, atom: Atom) -> *mut c_char;
}
/// C: xproto.h 10593/10596: `xcb_get_atom_name_cookie_t xcb_get_atom_name (xcb_connection_t *,
/// xcb_atom_t);` and `xcb_get_atom_name_reply_t *xcb_get_atom_name_reply (xcb_connection_t *,
/// xcb_get_atom_name_cookie_t, xcb_generic_error_t **);` plus the two accessors C 6395/6397 name -
/// the four rows are owed to `include/xlib.rs` (it carries the property/geometry families).
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr", feature = "have_xrandr_1_5", feature = "use_xcb"))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct xcb_get_atom_name_cookie_t
{
  pub sequence: c_uint,
}
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr", feature = "have_xrandr_1_5", feature = "use_xcb"))]
#[repr (C)]
pub struct xcb_get_atom_name_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub name_len: u16,
  pub pad1: [u8; 22],
}
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr", feature = "have_xrandr_1_5", feature = "use_xcb"))]
unsafe extern "C" {
  pub fn xcb_get_atom_name (c: *mut xcb_connection_t, name: xcb_atom_t)
			    -> xcb_get_atom_name_cookie_t;
  pub fn xcb_get_atom_name_reply (c: *mut xcb_connection_t, cookie: xcb_get_atom_name_cookie_t,
				  e: *mut *mut xcb_generic_error_t)
				  -> *mut xcb_get_atom_name_reply_t;
  pub fn xcb_get_atom_name_name_length (r: *const xcb_get_atom_name_reply_t) -> c_int;
  pub fn xcb_get_atom_name_name (r: *const xcb_get_atom_name_reply_t) -> *mut c_char;
}

#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system",
	    feature = "have_xrandr"))]
unsafe fn x_get_monitor_attributes_xrandr (dpyinfo: *mut x_display_info) -> Lisp_Object
{
  let mut attributes_list: Lisp_Object = Qnil ();                          // C 6304
  let mut resources: *mut XRRScreenResources;                              // C 6305
  let dpy: *mut Display = (*dpyinfo).display;                              // C 6306
  let mut i: c_int;                                                        // C 6307
  let mut n_monitors: c_int = 0;
  let mut primary: c_int = -1;
  let mut pxid: RROutput = 0;                                              // C 6308 (C: `None`)
  let mut monitors: *mut MonitorInfo = std::ptr::null_mut ();              // C 6309
  let mut randr15_p: bool = false;                                         // C 6310
#[cfg (feature = "have_xrandr_1_3")]
  /* Check if the display supports 1.3 too.  (C declares this at C 6418-6420, inside its
     RANDR13 section; C's scope is the whole function, so the port's declaration stands with
     the other locals.)  */
  let randr13_avail: bool = (*dpyinfo).xrandr_major_version > 1
    || ((*dpyinfo).xrandr_major_version == 1 && (*dpyinfo).xrandr_minor_version >= 3);

  'out: {
#[cfg (feature = "have_xrandr_1_5")]
    {
      let mut rr_monitors: *mut XRRMonitorInfo;                            // C 6313
#[cfg (feature = "use_xcb")]
      let mut atom_name_cookies: *mut xcb_get_atom_name_cookie_t;          // C 6315
#[cfg (feature = "use_xcb")]
      let mut reply: *mut xcb_get_atom_name_reply_t;                       // C 6316
#[cfg (feature = "use_xcb")]
      let mut error: *mut xcb_generic_error_t = std::ptr::null_mut ();     // C 6317
#[cfg (feature = "use_xcb")]
      let mut length: c_int;                                               // C 6318

      'fallback: {
	/* If RandR 1.5 or later is available, use that instead, as some
	   video drivers don't report correct dimensions via other versions
	   of RandR.  */
	if (*dpyinfo).xrandr_major_version > 1                                  // C 6324
	    || ((*dpyinfo).xrandr_major_version == 1
		&& (*dpyinfo).xrandr_minor_version >= 5)                        // C 6326
	  {
	    let mut workarea: XRectangle = std::mem::zeroed ();                 // C 6328
	    let mut name: *mut c_char;                                          // C 6329

	    rr_monitors = XRRGetMonitors ((*dpyinfo).display,                   // C 6331-6333
					  (*dpyinfo).root_window,
					  True, &raw mut n_monitors);
	    if rr_monitors.is_null () { break 'fallback; }                      // C 6334-6335

	    monitors = xcalloc (n_monitors as usize,                            // C 6337
				std::mem::size_of::<MonitorInfo> ()) as *mut MonitorInfo;
#[cfg (feature = "use_xcb")]
	    /* C's bare `alloca` is the port's stack pool (PORTING.md P-05's decision).  */
	    { atom_name_cookies = sa_stack_alloc (n_monitors as usize
						  * std::mem::size_of::<xcb_get_atom_name_cookie_t> ())
		as *mut xcb_get_atom_name_cookie_t; }                           // C 6339

	    i = 0;
	    while i < n_monitors                                               // C 6342
	      {
		(*monitors.offset (i as isize)).geom.x
		  = (*rr_monitors.offset (i as isize)).x as c_short;                  // C 6344
		(*monitors.offset (i as isize)).geom.y
		  = (*rr_monitors.offset (i as isize)).y as c_short;                  // C 6345
		(*monitors.offset (i as isize)).geom.width
		  = (*rr_monitors.offset (i as isize)).width as c_ushort;       // C 6346
		(*monitors.offset (i as isize)).geom.height
		  = (*rr_monitors.offset (i as isize)).height as c_ushort;      // C 6347
		(*monitors.offset (i as isize)).mm_width
		  = (*rr_monitors.offset (i as isize)).mwidth;                  // C 6348
		(*monitors.offset (i as isize)).mm_height
		  = (*rr_monitors.offset (i as isize)).mheight;                 // C 6349

#[cfg (not (feature = "use_xcb"))]
		{
		  name = XGetAtomName ((*dpyinfo).display,                     // C 6352
				       (*rr_monitors.offset (i as isize)).name);
		  if !name.is_null ()                                          // C 6353
		    {
		      (*monitors.offset (i as isize)).name = xstrdup (name);    // C 6355
		      XFree (name as *mut c_void);                              // C 6356
		    }
		  else
		    { (*monitors.offset (i as isize)).name
			= xstrdup (c"Unknown Monitor".as_ptr ()); }             // C 6359
		}
#[cfg (feature = "use_xcb")]
		{
		  *atom_name_cookies.offset (i as isize)
		    = xcb_get_atom_name ((*dpyinfo).xcb_connection,             // C 6361-6363
					 (*rr_monitors.offset (i as isize)).name as xcb_atom_t);
		}

		if (*rr_monitors.offset (i as isize)).primary != 0              // C 6366
		  { primary = i; }                                             // C 6367

		if (*rr_monitors.offset (i as isize)).primary != 0              // C 6369
		    && x_get_net_workarea (dpyinfo, &raw mut workarea)          // C 6370
		  {
		    (*monitors.offset (i as isize)).work = workarea;             // C 6372
		    if !gui_intersect_rectangles (&raw const (*monitors.offset (i as isize)).geom,   // C 6373
						  &raw const (*monitors.offset (i as isize)).work,
						  &raw mut (*monitors.offset (i as isize)).work)  // C 6375
		      { (*monitors.offset (i as isize)).work
			  = (*monitors.offset (i as isize)).geom; }             // C 6376
		  }
		else
		  { (*monitors.offset (i as isize)).work
		      = (*monitors.offset (i as isize)).geom; }                 // C 6379
		i += 1;                                                         // C 6342's `++i`
	      }

#[cfg (feature = "use_xcb")]
	    {
	      i = 0;
	      while i < n_monitors                                             // C 6383
		{
		  reply = xcb_get_atom_name_reply ((*dpyinfo).xcb_connection,   // C 6385-6386
						   *atom_name_cookies.offset (i as isize),
						   &raw mut error);

		  if reply.is_null ()                                           // C 6388
		    {
		      (*monitors.offset (i as isize)).name
			= xstrdup (c"Unknown monitor".as_ptr ());               // C 6390
		      free (error as *mut c_void);                              // C 6391
		    }
		  else
		    {
		      length = xcb_get_atom_name_name_length (reply);           // C 6395
		      name = xmalloc (length as usize + 1) as *mut c_char;      // C 6396
		      memcpy (name as *mut c_void,                              // C 6397
			      xcb_get_atom_name_name (reply) as *const c_void,
			      length as usize);
		      *name.offset (length as isize) = 0;                       // C 6398
		      (*monitors.offset (i as isize)).name = name;              // C 6399
		      free (reply as *mut c_void);                              // C 6400
		    }
		  i += 1;                                                       // C 6383's `++i`
		}
	    }

	    XRRFreeMonitors (rr_monitors);                                      // C 6405
	    randr15_p = true;                                                   // C 6406
	    break 'out;                                                         // C 6407's `goto out`
	  }
      }
    }

    /* C 6413-6414's `RANDR13_LIBRARY`: the library is at least 1.3 (the derived feature).  */
#[cfg (feature = "have_xrandr_1_3")]
    {
      if randr13_avail
	{ resources = XRRGetScreenResourcesCurrent (dpy, (*dpyinfo).root_window); }   // C 6422-6423
      else
	{ resources = XRRGetScreenResources (dpy, (*dpyinfo).root_window); }          // C 6425
    }
#[cfg (not (feature = "have_xrandr_1_3"))]
    { resources = XRRGetScreenResources (dpy, (*dpyinfo).root_window); }       // C 6427

    if resources.is_null () || (*resources).noutput == 0                        // C 6429
      {
	if !resources.is_null () { XRRFreeScreenResources (resources); }         // C 6431-6432
	return Qnil ();                                                         // C 6433
      }
    n_monitors = (*resources).noutput;                                          // C 6435
    monitors = xcalloc (n_monitors as usize,                                    // C 6436
			std::mem::size_of::<MonitorInfo> ()) as *mut MonitorInfo;

#[cfg (feature = "have_xrandr_1_3")]
    {
      if randr13_avail
	{ pxid = XRRGetOutputPrimary (dpy, (*dpyinfo).root_window); }           // C 6439-6440
    }

    i = 0;
    while i < n_monitors                                                        // C 6445
      {
	let info: *mut XRROutputInfo = XRRGetOutputInfo (dpy, resources,         // C 6447-6448
							 *(*resources).outputs.offset (i as isize));
	if info.is_null () { i += 1; continue; }                                // C 6449-6450

	if strcmp ((*info).name, c"default".as_ptr ()) == 0                     // C 6452
	  {
	    /* Non XRandr 1.2 driver, does not give useful data.  */
	    XRRFreeOutputInfo (info);                                           // C 6455
	    XRRFreeScreenResources (resources);                                 // C 6456
	    free_monitors (monitors, n_monitors);                               // C 6457
	    return Qnil ();                                                     // C 6458
	  }

	if (*info).connection != RR_Disconnected                            // C 6461
	    && (*info).crtc != 0                                            // C 6461 (C: `None`)
	  {
	    let crtc: *mut XRRCrtcInfo = XRRGetCrtcInfo (dpy, resources, (*info).crtc);  // C 6463
	    let mi: *mut MonitorInfo = monitors.offset (i as isize);        // C 6464
	    let mut workarea_r: XRectangle = std::mem::zeroed ();           // C 6465

	    if crtc.is_null ()                                              // C 6467
	      {
		XRRFreeOutputInfo (info);                                   // C 6469
		i += 1; continue;                                           // C 6470
	      }

	    (*mi).geom.x = (*crtc).x as c_short;                            // C 6473
	    (*mi).geom.y = (*crtc).y as c_short;                            // C 6474
	    (*mi).geom.width = (*crtc).width as c_ushort;                   // C 6475
	    (*mi).geom.height = (*crtc).height as c_ushort;                 // C 6476
	    (*mi).mm_width = (*info).mm_width;                              // C 6477
	    (*mi).mm_height = (*info).mm_height;                            // C 6478
	    (*mi).name = xstrdup ((*info).name);                            // C 6479

	    if pxid != 0 && pxid == *(*resources).outputs.offset (i as isize)   // C 6481
	      { primary = i; }
	    else if primary == -1 && strcmp ((*info).name, c"LVDS".as_ptr ()) == 0   // C 6483
	      { primary = i; }                                              // C 6484

	    if i == primary && x_get_net_workarea (dpyinfo, &raw mut workarea_r)   // C 6486
	      {
		(*mi).work = workarea_r;                                    // C 6488
		if ! gui_intersect_rectangles (&raw const (*mi).geom,        // C 6489
					       &raw const (*mi).work,
					       &raw mut (*mi).work)
		  { (*mi).work = (*mi).geom; }                              // C 6490
	      }
	    else
	      { (*mi).work = (*mi).geom; }                                  // C 6493

	    XRRFreeCrtcInfo (crtc);                                         // C 6495
	  }
	XRRFreeOutputInfo (info);                                           // C 6497
	i += 1;                                                             // C 6445's `++i`
      }
    XRRFreeScreenResources (resources);                                     // C 6499
  }

  attributes_list = x_make_monitor_attribute_list (monitors,                // C 6503-6506
						   n_monitors,
						   primary,
						   dpyinfo,
						   if randr15_p { c"XRandR 1.5".as_ptr () }  // C 6507-6509
						   else { c"XRandr".as_ptr () });
  free_monitors (monitors, n_monitors);                                     // C 6510
  return attributes_list;                                                   // C 6511
}

/* -----------------------------------------------------------------------------------------
   C 6515-6601: `x_get_monitor_attributes` - the dispatcher the subr at C 6771 and the Lucid
   helper below both call - and, under C 6548's `#ifdef USE_LUCID`, `xlw_monitor_dimensions_at_pos_1`
   with `xlw_monitor_dimensions_at_pos` (`lwlib/xlwmenu.c` 1475/1479/2822 are their three
   callers).  C 6546's `#endif /* !USE_GTK */` closes the region the monitor functions from
   C 6125 on live in, and the dispatcher keeps the slice 15 guard pair (C's `#ifndef USE_GTK`
   narrowed by frame.h 1926's `#ifdef HAVE_WINDOW_SYSTEM`, where `struct MonitorInfo` and the
   three callees live).  C's own gate for the two Lucid functions is `USE_LUCID` alone, but
   their callee is the pair's, so the ledger's Lucid arm carries the pair (the WSX arm's shape,
   slice 16).
   -------------------------------------------------------------------------------------- */

/* Xlib.h 129-130: `#define WidthOfScreen(s) ((s)->width)` / `#define HeightOfScreen(s)
   ((s)->height)` - the mirror's `Screen` carries both members (`include/xlib.rs` 1124-1125),
   so the two macros keep C's name and read them (the `WidthMMOfScreen` pair at 8194-8200 is
   the shape).  C 6585-6586's calls are the only two.  */
#[cfg (feature = "have_x_windows")]
macro_rules! WidthOfScreen { ($s:expr) => { (*$s).width }; }                     // Xlib.h 129
#[cfg (feature = "have_x_windows")]
macro_rules! HeightOfScreen { ($s:expr) => { (*$s).height }; }                   // Xlib.h 130

/* C: xterm.h 1001/1913: `extern struct x_display_info *x_dpyinfo (Display *) ATTRIBUTE_MALLOC;`
   and `extern void xlw_monitor_dimensions_at_pos (Display *, Screen *, int, int, int *, int *,
   int *, int *);` - the first is `src/xterm.c` 6883's definition (the port's `src/xterm.rs`
   7543 carries C's linkage) and this unit's include graph declares neither, so it stands at the
   use site (PORTING.md P-55/P-72 rule 3); the second is this unit's own export, so the
   definition below is its home.  C 6593's call is the only use of the first.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_dpyinfo (dpy: *mut Display) -> *mut x_display_info;
}

#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk"), feature = "have_window_system"))]
unsafe fn x_get_monitor_attributes (dpyinfo: *mut x_display_info) -> Lisp_Object
{
  let mut attributes_list: Lisp_Object = Qnil ();                       // C 6518
  let dpy: *mut Display = (*dpyinfo).display;                           // C 6519

  let _ = dpy;  /* C 6521: `(void) dpy;` - "Suppress unused variable warning."  */

  #[cfg (feature = "have_xrandr")]                                      // C 6523
  {
    let xrr_ok: bool = ((*dpyinfo).xrandr_major_version == 1           // C 6524
			&& (*dpyinfo).xrandr_minor_version >= 2         // C 6525
			|| (*dpyinfo).xrandr_major_version > 1);        // C 6526

    if xrr_ok
      { attributes_list = x_get_monitor_attributes_xrandr (dpyinfo); }  // C 6528-6529
  }

  #[cfg (feature = "have_xinerama")]                                    // C 6532
  {
    if NILP (attributes_list)                                           // C 6533
      {
	if (*dpyinfo).xinerama_supported_p && XineramaIsActive (dpy) != 0   // C 6535
	  { attributes_list = x_get_monitor_attributes_xinerama (dpyinfo); }  // C 6536
      }
  }

  if NILP (attributes_list)                                             // C 6540
    { attributes_list = x_get_monitor_attributes_fallback (dpyinfo); }  // C 6541

  return attributes_list;                                               // C 6543
}

/* C 6549-6550: "This is used by the Lucid menu widget, but it's defined here so we can make use
   of a great deal of existing code."  */
#[cfg (feature = "use_lucid")]
unsafe fn xlw_monitor_dimensions_at_pos_1 (dpyinfo: *mut x_display_info,        // C 6551-6554
					   screen: *mut Screen, src_x: c_int, src_y: c_int,
					   x: *mut c_int, y: *mut c_int, width: *mut c_int,
					   height: *mut c_int)
{
  let attrs: Lisp_Object = x_get_monitor_attributes (dpyinfo);          // C 6556/6558
  let mut tem: Lisp_Object = attrs;                                     // C 6560
  let mut val: Lisp_Object = Qnil ();                                   // C 6556

  while CONSP (tem)                                                     // C 6560
    {
      let mut sx: c_int = 0;                                            // C 6562
      let mut sy: c_int = 0;
      let mut swidth: c_int = 0;
      let mut sheight: c_int = 0;
      val = assq_no_quit (Qworkarea (), XCAR (tem));                    // C 6563
      if ! NILP (val)                                                   // C 6564
	{
	  sx = XFIXNUM (XCAR (XCDR (val))) as c_int;                    // C 6566
	  sy = XFIXNUM (XCAR (XCDR (XCDR (val)))) as c_int;             // C 6567
	  swidth = XFIXNUM (XCAR (XCDR (XCDR (XCDR (val))))) as c_int;  // C 6568
	  sheight = XFIXNUM (XCAR (XCDR (XCDR (XCDR (XCDR (val)))))) as c_int;  /* C 6569 */

	  if sx <= src_x && src_x < (sx + swidth)                       // C 6571
	    && sy <= src_y && src_y < (sy + swidth)                     // C 6572
	    {
	      *x = sx;                                                  // C 6574
	      *y = sy;                                                  // C 6575
	      *width = swidth;                                          // C 6576
	      *height = sheight;                                        // C 6577
	      return;                                                   // C 6578
	    }
	}
      tem = XCDR (tem);                                                 // C 6560: `tem = XCDR (tem)`
    }

  *x = 0;                                                               // C 6583
  *y = 0;                                                               // C 6584
  *width = WidthOfScreen! (screen);                                     // C 6585
  *height = HeightOfScreen! (screen);                                   // C 6586
}

#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe fn xlw_monitor_dimensions_at_pos (dpy: *mut Display,         // C 6589-6591
					     screen: *mut Screen, src_x: c_int, src_y: c_int,
					     x: *mut c_int, y: *mut c_int, width: *mut c_int,
					     height: *mut c_int)
{
  let dpyinfo: *mut x_display_info = x_dpyinfo (dpy);                   // C 6593

  block_input ();                                                       // C 6595
  xlw_monitor_dimensions_at_pos_1 (dpyinfo, screen, src_x, src_y,       // C 6596-6597
				   x, y, width, height);

  unblock_input ();                                                     // C 6599
}

/* -----------------------------------------------------------------------------------------
   C 6604-6777: the DEFUN `x-display-monitor-attributes-list` - the GTK monitor walk
   (C 6627-6767, whose 3.22 arms choose between the `gdk_monitor_*` family and the pre-3.22
   `gdk_screen_*` one) and the plain-X arm (C 6768-6773, the slice 18 dispatcher between
   `block_input` and `unblock_input`).
   -------------------------------------------------------------------------------------- */

/* C: gdk/gdkscreen.h's nine pre-3.22 `GdkScreen` entry points C 6643-6757 reads, plus
   gdk/gdkrectangle.h's intersection C 6720 reads: no mirror carries them (`include/gtk.rs`
   carries the 3.22 *monitor* family instead, under `#[cfg (not (have_pgtk))]`), so they stand at
   the use site under C's own sub-arm gate (P-55/P-72 rule 3).  This machine has no GTK headers
   (`include/gtk.rs`'s own rows name the header and the signature only), so the rows are owed to
   a `gdk.rs`-style mirror and the `Owed` list names them.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk",
	    not (feature = "gtk_check_version_ge_3_22")))]
unsafe extern "C" {
  pub fn gdk_screen_get_n_monitors (screen: *mut GdkScreen) -> gint;                // C 6644
  pub fn gdk_screen_get_primary_monitor (screen: *mut GdkScreen) -> gint;           // C 6645
  pub fn gdk_screen_get_monitor_at_window (screen: *mut GdkScreen,                  // C 6671
					   window: *mut GdkWindow) -> gint;
  pub fn gdk_screen_get_monitor_geometry (screen: *mut GdkScreen, monitor_num: gint,  // C 6691
					  dest: *mut GdkRectangle);
  pub fn gdk_screen_get_monitor_width_mm (screen: *mut GdkScreen,                   // C 6698
					  monitor_num: gint) -> gint;
  pub fn gdk_screen_get_monitor_height_mm (screen: *mut GdkScreen,                  // C 6699
					   monitor_num: gint) -> gint;
  pub fn gdk_screen_get_monitor_workarea (screen: *mut GdkScreen, monitor_num: gint,  // C 6708
					  dest: *mut GdkRectangle);
  pub fn gdk_screen_get_monitor_scale_factor (screen: *mut GdkScreen,               // C 6732
					      monitor_num: gint) -> gint;
  pub fn gdk_screen_get_monitor_plug_name (screen: *mut GdkScreen,                  // C 6757
					   monitor_num: gint) -> *mut c_char;
  pub fn gdk_rectangle_intersect (src1: *const GdkRectangle, src2: *const GdkRectangle,  // C 6720
				  dest: *mut GdkRectangle) -> gboolean;
}

/* C: lisp.h 5555: `extern void dupstring (char **, char const *);` - C 6755's call; no mirror
   carries it (`pgtkfns.rs` 743-745 declares the same row at its own use site).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn dupstring (dest: *mut *mut c_char, src: *const c_char);                    // C 6755
}

/* C 6604-6777.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-monitor-attributes-list", Fx_display_monitor_attributes_list,
	Sx_display_monitor_attributes_list,
	0, 1, 0,
	doc: "Return a list of physical monitor attributes on the X display TERMINAL.\n\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nIn addition to the standard attribute keys listed in\n`display-monitor-attributes-list', the following keys are contained in\nthe attributes:\n\n source -- String describing the source from which multi-monitor\n\t   information is obtained, one of \"Gdk\", \"XRandR 1.5\",\n\t   \"XRandr\", \"Xinerama\", or \"fallback\"\n\nInternal use only, use `display-monitor-attributes-list' instead.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);    // C 6624
  let mut attributes_list: Lisp_Object = Qnil ();                        // C 6625

  #[cfg (feature = "use_gtk")]                                           // C 6627
  {
    let mut gdpy: *mut GdkDisplay;                                       // C 6628
    #[cfg (not (feature = "gtk_check_version_ge_3_22"))]                 // C 6629
    let mut mm_width_per_pixel: c_double = 0.0;                          // C 6630
    #[cfg (not (feature = "gtk_check_version_ge_3_22"))]
    let mut mm_height_per_pixel: c_double = 0.0;
    #[cfg (not (feature = "gtk_check_version_ge_3_22"))]
    let mut gscreen: *mut GdkScreen = std::ptr::null_mut ();             // C 6631
    let mut primary_monitor: gint = 0;                                   // C 6633
    let mut n_monitors: gint;
    let mut i: gint;
    let mut monitor_frames: Lisp_Object;                                 // C 6634
    let mut rest: Lisp_Object;
    let mut frame: Lisp_Object;
    /* C 6635: `static const char *source = "Gdk";` - C's function-scope static, whose storage
       duration is the program's; the port writes Rust's function-scope `static mut` with C's own
       value (`pgtkfns.rs` 3707-3710's shape).  */
    static mut source: *const c_char = c"Gdk".as_ptr ();
    let mut monitors: *mut MonitorInfo;                                  // C 6636

    block_input ();                                                      // C 6638
    gdpy = gdk_x11_lookup_xdisplay ((*dpyinfo).display);                 // C 6639
    #[cfg (feature = "gtk_check_version_ge_3_22")]                       // C 6640
    { n_monitors = gdk_display_get_n_monitors (gdpy); }                  // C 6641
    #[cfg (not (feature = "gtk_check_version_ge_3_22"))]                 // C 6642
    {
      gscreen = gdk_display_get_default_screen (gdpy);                   // C 6643
      n_monitors = gdk_screen_get_n_monitors (gscreen);                  // C 6644
      primary_monitor = gdk_screen_get_primary_monitor (gscreen);        // C 6645
      /* C 6646: "Fallback if gdk_screen_get_monitor_{width,height}_mm fail".  */
      mm_width_per_pixel = (WidthMMOfScreen! ((*dpyinfo).screen) as c_double)   // C 6647-6648
			    / x_display_pixel_width (dpyinfo) as c_double;
      mm_height_per_pixel = (HeightMMOfScreen! ((*dpyinfo).screen) as c_double)  // C 6649-6650
			     / x_display_pixel_height (dpyinfo) as c_double;	/* the pair is declared at 7989-7996 */
    }
    monitor_frames = make_nil_vector (n_monitors as isize);              // C 6652
    monitors = xcalloc (n_monitors as size_t,                            // C 6653
			std::mem::size_of::<MonitorInfo> ()) as *mut MonitorInfo;

    FOR_EACH_FRAME! (rest, frame, {                                      // C 6655
	let f: *mut frame = XFRAME (frame);                              // C 6657

	if FRAME_X_P (f)                                                 // C 6659
	  && FRAME_DISPLAY_INFO! (f) == dpyinfo
	  && !FRAME_TOOLTIP_P (f)
	  {
	    let gwin: *mut GdkWindow = gtk_widget_get_window (FRAME_GTK_WIDGET! (f));  // C 6663

	    #[cfg (feature = "gtk_check_version_ge_3_22")]               // C 6665
	    {
	      i = 0;                                                     // C 6666
	      while i < n_monitors
		{
		  if gdk_display_get_monitor_at_window (gdpy, gwin)      // C 6667-6668
		     == gdk_display_get_monitor (gdpy, i)
		    { break; }                                           // C 6669
		  i += 1;
		}
	    }
	    #[cfg (not (feature = "gtk_check_version_ge_3_22"))]         // C 6670
	    { i = gdk_screen_get_monitor_at_window (gscreen, gwin); }    // C 6671

	    if 0 <= i && i < n_monitors                                  // C 6673
	      { ASET (monitor_frames, i as isize,                        // C 6674
		      Fcons (frame, AREF (monitor_frames, i as isize))); }
	  }
    });

    i = 0;                                                               // C 6678
    while i < n_monitors
      {
	let mut width_mm: gint = 0;                                      // C 6680
	let mut height_mm: gint = 0;
	let mut rec: GdkRectangle = std::mem::zeroed ();                 // C 6681
	let mut work: GdkRectangle = std::mem::zeroed ();
	/* C 6682's `struct MonitorInfo *mi = &monitors[i];` - the port's element address.  */
	let mi: *mut MonitorInfo = monitors.offset (i as isize);
	let mut scale: c_int = 1;                                        // C 6683
	/* C 6686's `GdkMonitor *monitor` is declared inside its 3.22 arm but read by four more of
	   them (C 6695/6696, 6706, 6730, 6755), so the port hoists the declaration to the loop's own
	   locals under the same gate (the `randr13_avail` shape, slice 17's deviation 3).  */
	#[cfg (feature = "gtk_check_version_ge_3_22")]
	let mut monitor: *mut GdkMonitor = std::ptr::null_mut ();

	#[cfg (feature = "gtk_check_version_ge_3_22")]                   // C 6685
	{
	  monitor = gdk_display_get_monitor (gdpy, i);                   // C 6686
	  if gdk_monitor_is_primary (monitor) != 0                       // C 6687
	    { primary_monitor = i; }                                     // C 6688
	  gdk_monitor_get_geometry (monitor, &raw mut rec);              // C 6689
	}
	#[cfg (not (feature = "gtk_check_version_ge_3_22"))]             // C 6690
	{ gdk_screen_get_monitor_geometry (gscreen, i, &raw mut rec); }  // C 6691

	#[cfg (feature = "gtk_check_version_ge_3_22")]                   // C 6694
	{
	  width_mm = gdk_monitor_get_width_mm (monitor);                 // C 6695
	  height_mm = gdk_monitor_get_height_mm (monitor);               // C 6696
	}
	#[cfg (not (feature = "gtk_check_version_ge_3_22"))]             // C 6697
	{
	  width_mm = gdk_screen_get_monitor_width_mm (gscreen, i);       // C 6698
	  height_mm = gdk_screen_get_monitor_height_mm (gscreen, i);     // C 6699
	  if width_mm < 0                                                // C 6700
	    { width_mm = (rec.width as c_double * mm_width_per_pixel + 0.5) as gint; }   // C 6701
	  if height_mm < 0                                               // C 6702
	    { height_mm = (rec.height as c_double * mm_height_per_pixel + 0.5) as gint; }  // C 6703
	}

	#[cfg (feature = "gtk_check_version_ge_3_22")]                   // C 6705
	{ gdk_monitor_get_workarea (monitor, &raw mut work); }           // C 6706
	#[cfg (all (not (feature = "gtk_check_version_ge_3_22"),
		    feature = "have_gtk3"))]                             // C 6707
	{ gdk_screen_get_monitor_workarea (gscreen, i, &raw mut work); }  // C 6708
	#[cfg (all (not (feature = "gtk_check_version_ge_3_22"),
		    not (feature = "have_gtk3")))]                       // C 6709
	{
	  /* Emulate the behavior of GTK+ 3.4.  */                       // C 6710
	  let mut workarea_r: XRectangle = std::mem::zeroed ();          // C 6712

	  if i == primary_monitor && x_get_net_workarea (dpyinfo, &raw mut workarea_r)  // C 6714
	    {
	      work.x = workarea_r.x as c_int;                            // C 6716
	      work.y = workarea_r.y as c_int;                            // C 6717
	      work.width = workarea_r.width as c_int;                    // C 6718
	      work.height = workarea_r.height as c_int;                  // C 6719
	      if ! gdk_rectangle_intersect (&raw const rec, &raw const work, &raw mut work)  // C 6720
		{ work = rec; }                                          // C 6721
	    }
	  else
	    { work = rec; }                                              // C 6724
	}

	/* GTK returns scaled sizes for the workareas.  */               // C 6728
	#[cfg (feature = "gtk_check_version_ge_3_22")]                   // C 6729
	{ scale = gdk_monitor_get_scale_factor (monitor); }              // C 6730
	#[cfg (all (not (feature = "gtk_check_version_ge_3_22"),
		    feature = "have_gtk3"))]                             // C 6731
	{ scale = gdk_screen_get_monitor_scale_factor (gscreen, i); }    // C 6732
	rec.x *= scale;                                                  // C 6734
	rec.y *= scale;                                                  // C 6735
	rec.width *= scale;                                              // C 6736
	rec.height *= scale;                                             // C 6737
	work.x *= scale;                                                 // C 6738
	work.y *= scale;                                                 // C 6739
	work.width *= scale;                                             // C 6740
	work.height *= scale;                                            // C 6741

	/* C's `struct MonitorInfo` rectangles are `Emacs_Rectangle` (`frame.h` 1929), which the X
	   build reads as `XRectangle` (`dispextern.h` 38's alias: `short`/`unsigned short`), so the
	   eight implicit gint -> short/unsigned short conversions C writes are written out.  */
	(*mi).geom.x = rec.x as c_short;                                 // C 6743
	(*mi).geom.y = rec.y as c_short;                                 // C 6744
	(*mi).geom.width = rec.width as c_ushort;                        // C 6745
	(*mi).geom.height = rec.height as c_ushort;                      // C 6746
	(*mi).work.x = work.x as c_short;                                // C 6747
	(*mi).work.y = work.y as c_short;                                // C 6748
	(*mi).work.width = work.width as c_ushort;                       // C 6749
	(*mi).work.height = work.height as c_ushort;                     // C 6750
	(*mi).mm_width = width_mm;                                       // C 6751
	(*mi).mm_height = height_mm;                                     // C 6752

	#[cfg (feature = "gtk_check_version_ge_3_22")]                   // C 6754
	{ dupstring (&raw mut (*mi).name, gdk_monitor_get_model (monitor)); }  // C 6755
	#[cfg (not (feature = "gtk_check_version_ge_3_22"))]             // C 6756
	{ (*mi).name = gdk_screen_get_monitor_plug_name (gscreen, i); }  // C 6757

	i += 1;                                                          // C 6678's `++i`
      }

    attributes_list = make_monitor_attribute_list (monitors,             // C 6761-6765
						   n_monitors,
						   primary_monitor,
						   monitor_frames,
						   source);
    free_monitors (monitors, n_monitors);                                // C 6766
    unblock_input ();                                                    // C 6767
  }

  #[cfg (not (feature = "use_gtk"))]                                     // C 6768
  {
    block_input ();                                                      // C 6770
    /* C's `#ifndef USE_GTK` arm is reachable only where its callee is: `HAVE_X_WINDOWS`
       implies `HAVE_WINDOW_SYSTEM` (configure.ac 7517-7518 defines the latter whenever
       `$window_system != "none"`), and the callee's own gate is the slice 15 pair (frame.h
       1926's `#ifdef HAVE_WINDOW_SYSTEM`, where `struct MonitorInfo` lives).  The statement
       carries the callee's existence condition, so the `have_x_windows`-only reading arms - a
       combination C cannot reach - read no phantom `E0425`; the ledger records the deviation.  */
    #[cfg (feature = "have_window_system")]
    { attributes_list = x_get_monitor_attributes (dpyinfo); }            // C 6771
    unblock_input ();                                                    // C 6772
  }

  return attributes_list;                                               // C 6776
});

/* C: frame.h 1476: `extern struct frame *decode_live_frame (Lisp_Object);` - `frame.c` defines it
   (the port's `src/frame.rs`) and this unit's include graph carries no declaration, so it stands
   at the use site (PORTING.md P-55/P-72 rule 3; `src/dispnew.rs` 270-271 declares the same row).
   C 6788's call, the only one here.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {

}

/* -----------------------------------------------------------------------------------------
   C 6785-7035: `frame_geometry` (its comment is C 6779-6783) and the two subrs it backs,
   `x-frame-geometry` (C 6964-7011) and `x-frame-edges` (C 7013-7035).  The `HAVE_EXT_MENU_BAR`
   and `HAVE_EXT_TOOL_BAR` arms C 6860-6922 carries decide where the menu and tool bars sit.
   -------------------------------------------------------------------------------------- */

/* C 6779-6783: "Return geometric attributes of FRAME.  According to the value of ATTRIBUTES
   return the outer edges of FRAME (Qouter_edges), the native edges of FRAME (Qnative_edges), or
   the inner edges of frame (Qinner_edges).  Any other value means to return the geometry as
   returned by Fx_frame_geometry."  */
#[cfg (feature = "have_x_windows")]
unsafe fn frame_geometry (frame: Lisp_Object, attribute: Lisp_Object) -> Lisp_Object
{
  let f: *mut frame = decode_live_frame (frame);                          // C 6788
  /* C 6789: `/**   XWindowAttributes atts; **/` - the file's own commented-out declaration.  */
  let mut rootw: Window = 0;                                              // C 6790
  let mut ign: c_uint = 0;                                                // C 6791
  let mut native_width: c_uint = 0;
  let mut native_height: c_uint = 0;
  let mut x_border_width: c_uint = 0;
  let mut x_native: c_int = 0;                                            // C 6792
  let mut y_native: c_int = 0;
  let mut xptr: c_int = 0;
  let mut yptr: c_int = 0;
  let mut left_off: c_int = 0;                                            // C 6793
  let mut right_off: c_int = 0;
  let mut top_off: c_int = 0;
  let mut bottom_off: c_int = 0;
  let mut outer_left: c_int;                                              // C 6794
  let mut outer_top: c_int;
  let mut outer_right: c_int;
  let mut outer_bottom: c_int;
  let mut native_left: c_int;                                             // C 6795
  let mut native_top: c_int;
  let mut native_right: c_int;
  let mut native_bottom: c_int;
  let mut inner_left: c_int;                                              // C 6796
  let mut inner_top: c_int;
  let mut inner_right: c_int;
  let mut inner_bottom: c_int;
  let mut internal_border_width: c_int;                                   // C 6797
  let mut menu_bar_external: bool = false;                                // C 6798
  let mut tool_bar_external: bool = false;
  let mut menu_bar_height: c_int = 0;                                     // C 6799
  let mut menu_bar_width: c_int = 0;
  let mut tab_bar_height: c_int = 0;                                      // C 6800
  let mut tab_bar_width: c_int = 0;
  let mut tool_bar_height: c_int = 0;                                     // C 6801
  let mut tool_bar_width: c_int = 0;

  if FRAME_INITIAL_P (f) || !FRAME_X_P (f) || FRAME_OUTER_WINDOW! (f) == 0   // C 6803
    { return Qnil (); }                                                    // C 6804

  block_input ();                                                          // C 6806
  XGetGeometry (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),             // C 6807-6809
		&raw mut rootw, &raw mut x_native, &raw mut y_native,
		&raw mut native_width, &raw mut native_height,
		&raw mut x_border_width, &raw mut ign);
  /* C 6810: `/**   XGetWindowAttributes (FRAME_X_DISPLAY (f), FRAME_OUTER_WINDOW (f), &atts); **/`  */
  if FRAME_PARENT_FRAME (f).is_null ()                                     // C 6811
    { x_real_pos_and_offsets (f, &raw mut left_off, &raw mut right_off,      // C 6812-6813
			      &raw mut top_off, &raw mut bottom_off,
			      std::ptr::null_mut (), std::ptr::null_mut (),
			      &raw mut xptr, &raw mut yptr, std::ptr::null_mut ()); }
  unblock_input ();                                                        // C 6814

  /* C 6816-6817: `/**   native_width = atts.width; **/` and `/**   native_height = atts.height; **/`  */

  if !FRAME_PARENT_FRAME (f).is_null ()                                    // C 6819
    {
      let mut parent: Lisp_Object = Qnil ();                               // C 6821
      let mut edges: Lisp_Object = Qnil ();

      XSETFRAME! (parent, FRAME_PARENT_FRAME (f));                          // C 6823
      edges = Fx_frame_edges (parent, Qnative_edges ());                    // C 6824
      if !NILP (edges)                                                      // C 6825
	{
	  x_native += XFIXNUM (Fnth (make_fixnum (0), edges)) as c_int;      // C 6827
	  y_native += XFIXNUM (Fnth (make_fixnum (1), edges)) as c_int;      // C 6828
	}

      outer_left = x_native;                                                // C 6831
      outer_top = y_native;                                                 // C 6832
      outer_right = outer_left + native_width as c_int + 2 * (x_border_width as c_int);   // C 6833
      outer_bottom = outer_top + native_height as c_int + 2 * (x_border_width as c_int);  // C 6834

      native_left = x_native + x_border_width as c_int;                     // C 6836
      native_top = y_native + x_border_width as c_int;                      // C 6837
      native_right = native_left + native_width as c_int;                   // C 6838
      native_bottom = native_top + native_height as c_int;                  // C 6839
    }
  else
    {
      outer_left = xptr;                                                    // C 6843
      outer_top = yptr;                                                     // C 6844
      outer_right = outer_left + left_off + native_width as c_int + right_off;    // C 6845
      outer_bottom = outer_top + top_off + native_height as c_int + bottom_off;   // C 6846

      native_left = outer_left + left_off;                                  // C 6848
      native_top = outer_top + top_off;                                     // C 6849
      native_right = native_left + native_width as c_int;                   // C 6850
      native_bottom = native_top + native_height as c_int;                  // C 6851
    }

  internal_border_width = FRAME_INTERNAL_BORDER_WIDTH (f);                  // C 6854
  inner_left = native_left + internal_border_width;                         // C 6855
  inner_top = native_top + internal_border_width;                           // C 6856
  inner_right = native_right - internal_border_width;                       // C 6857
  inner_bottom = native_bottom - internal_border_width;                     // C 6858

  #[cfg (feature = "have_ext_menu_bar")]                                    // C 6860
  {
    menu_bar_external = true;                                               // C 6861
    menu_bar_height = FRAME_MENUBAR_HEIGHT! (f);                            // C 6862
    native_top += menu_bar_height;                                          // C 6863
    inner_top += menu_bar_height;                                           // C 6864
  }
  #[cfg (not (feature = "have_ext_menu_bar"))]                              // C 6865
  {
    menu_bar_height = FRAME_MENU_BAR_HEIGHT (f);                            // C 6866
    inner_top += menu_bar_height;                                           // C 6867
  }
  menu_bar_width = if menu_bar_height != 0 { native_width as c_int } else { 0 };   // C 6869

  tab_bar_height = FRAME_TAB_BAR_HEIGHT (f);                                // C 6871
  tab_bar_width = if tab_bar_height != 0                                    // C 6872-6874
    { native_width as c_int - 2 * internal_border_width } else { 0 };
  inner_top += tab_bar_height;                                              // C 6875

  #[cfg (feature = "have_ext_tool_bar")]                                    // C 6877
  {
    tool_bar_external = true;                                               // C 6878
    if EQ (FRAME_TOOL_BAR_POSITION (f), Qleft ())                           // C 6879
      {
	tool_bar_width = FRAME_TOOLBAR_WIDTH! (f);                          // C 6881
	native_left += tool_bar_width;                                      // C 6882
	inner_left += tool_bar_width;                                       // C 6883
	tool_bar_height = if tool_bar_width != 0                            // C 6884-6885
	  { native_height as c_int - menu_bar_height } else { 0 };
      }
    else if EQ (FRAME_TOOL_BAR_POSITION (f), Qtop ())                       // C 6887
      {
	tool_bar_height = FRAME_TOOLBAR_HEIGHT! (f);                        // C 6889
	native_top += tool_bar_height;                                      // C 6890
	inner_top += tool_bar_height;                                       // C 6891
	tool_bar_width = if tool_bar_height != 0 { native_width as c_int } else { 0 };   // C 6892
      }
    else if EQ (FRAME_TOOL_BAR_POSITION (f), Qright ())                     // C 6894
      {
	tool_bar_width = FRAME_TOOLBAR_WIDTH! (f);                          // C 6896
	native_right -= tool_bar_width;                                     // C 6897
	inner_right -= tool_bar_width;                                      // C 6898
	tool_bar_height = if tool_bar_width != 0                            // C 6899-6900
	  { native_height as c_int - menu_bar_height } else { 0 };
      }
    else
      {
	tool_bar_height = FRAME_TOOLBAR_HEIGHT! (f);                        // C 6904
	native_bottom -= tool_bar_height;                                   // C 6905
	inner_bottom -= tool_bar_height;                                    // C 6906
	tool_bar_width = if tool_bar_height != 0 { native_width as c_int } else { 0 };  // C 6907
      }
  }
  #[cfg (not (feature = "have_ext_tool_bar"))]                              // C 6909
  {
    tool_bar_height = FRAME_TOOL_BAR_HEIGHT (f);                            // C 6910
    tool_bar_width = if tool_bar_height != 0                                // C 6911-6913
      { native_width as c_int - 2 * internal_border_width } else { 0 };

    /* Subtract or add to the inner dimensions based on the tool bar position.  */   // C 6915-6916

    if EQ (FRAME_TOOL_BAR_POSITION (f), Qtop ())                            // C 6918
      { inner_top += tool_bar_height; }                                     // C 6919
    else
      { inner_bottom -= tool_bar_height; }                                  // C 6921
  }

  /* Construct list.  */                                                    // C 6924
  if EQ (attribute, Qouter_edges ())                                        // C 6925
    { return list4i (outer_left as EMACS_INT, outer_top as EMACS_INT,       // C 6926
		     outer_right as EMACS_INT, outer_bottom as EMACS_INT); }
  else if EQ (attribute, Qnative_edges ())                                  // C 6927
    { return list4i (native_left as EMACS_INT, native_top as EMACS_INT,     // C 6928
		     native_right as EMACS_INT, native_bottom as EMACS_INT); }
  else if EQ (attribute, Qinner_edges ())                                   // C 6929
    { return list4i (inner_left as EMACS_INT, inner_top as EMACS_INT,       // C 6930
		     inner_right as EMACS_INT, inner_bottom as EMACS_INT); }
  else
    { return                                                                // C 6932
      list! (Fcons (Qouter_position (),                                     // C 6933-6935
		    Fcons (make_fixnum (outer_left as EMACS_INT),
			   make_fixnum (outer_top as EMACS_INT))),
	     Fcons (Qouter_size (),                                         // C 6936-6938
		    Fcons (make_fixnum ((outer_right - outer_left) as EMACS_INT),
			   make_fixnum ((outer_bottom - outer_top) as EMACS_INT))),
	     /* Approximate.  */                                            // C 6939
	     Fcons (Qexternal_border_size (),                               // C 6940-6942
		    Fcons (make_fixnum (right_off as EMACS_INT),
			   make_fixnum (bottom_off as EMACS_INT))),
	     Fcons (Qouter_border_width (), make_fixnum (x_border_width as EMACS_INT)),  // C 6943
	     /* Approximate.  */                                            // C 6944
	     Fcons (Qtitle_bar_size (),                                     // C 6945-6947
		    Fcons (make_fixnum (0),
			   make_fixnum ((top_off - bottom_off) as EMACS_INT))),
	     Fcons (Qmenu_bar_external (),                                  // C 6948
		    if menu_bar_external { Qt () } else { Qnil () }),
	     Fcons (Qmenu_bar_size (),                                      // C 6949-6951
		    Fcons (make_fixnum (menu_bar_width as EMACS_INT),
			   make_fixnum (menu_bar_height as EMACS_INT))),
	     Fcons (Qtab_bar_size (),                                       // C 6952-6954
		    Fcons (make_fixnum (tab_bar_width as EMACS_INT),
			   make_fixnum (tab_bar_height as EMACS_INT))),
	     Fcons (Qtool_bar_external (),                                  // C 6955
		    if tool_bar_external { Qt () } else { Qnil () }),
	     Fcons (Qtool_bar_position (), FRAME_TOOL_BAR_POSITION (f)),    // C 6956
	     Fcons (Qtool_bar_size (),                                      // C 6957-6959
		    Fcons (make_fixnum (tool_bar_width as EMACS_INT),
			   make_fixnum (tool_bar_height as EMACS_INT))),
	     Fcons (Qinternal_border_width (),                              // C 6960-6961
		    make_fixnum (internal_border_width as EMACS_INT)));
    }
}

/* C 6964-7011.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-frame-geometry", Fx_frame_geometry, Sx_frame_geometry,
	0, 1, 0,
	doc: "Return geometric attributes of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is an association list of the attributes listed below.  All height\nand width values are in pixels.\n\n`outer-position' is a cons of the outer left and top edges of FRAME\n  relative to the origin - the position (0, 0) - of FRAME's display.\n\n`outer-size' is a cons of the outer width and height of FRAME.  The\n  outer size includes the title bar and the external borders as well as\n  any menu and/or tool bar of frame.  For a child frame the value\n  includes FRAME's X borders, if any.\n\n`external-border-size' is a cons of the horizontal and vertical width of\n  FRAME's external borders as supplied by the window manager.\n\n`title-bar-size' is a cons of the width and height of the title bar of\n  FRAME as supplied by the window manager.  If both of them are zero,\n  FRAME has no title bar.  If only the width is zero, Emacs was not\n  able to retrieve the width information.\n\n`menu-bar-external', if non-nil, means the menu bar is external (never\n  included in the inner edges of FRAME).\n\n`menu-bar-size' is a cons of the width and height of the menu bar of\n  FRAME.\n\n`tool-bar-external', if non-nil, means the tool bar is external (never\n  included in the inner edges of FRAME).\n\n`tool-bar-position' tells on which side the tool bar on FRAME is and can\n  be one of `left', `top', `right' or `bottom'.  If this is nil, FRAME\n  has no tool bar.\n\n`tool-bar-size' is a cons of the width and height of the tool bar of\n  FRAME.\n\n`internal-border-width' is the width of the internal border of\n  FRAME.\n\n`outer-border-width' is the width of the X border of FRAME.  The X\n  border is usually shown only for frames without window manager\n  decorations, such as child and tooltip frames.  ",
	(frame: Lisp_Object),
	{
  return frame_geometry (frame, Qnil ());                                   // C 7010
});

/* C 7013-7035.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-frame-edges", Fx_frame_edges, Sx_frame_edges,
	0, 2, 0,
	doc: "Return edge coordinates of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is a list of the form (LEFT, TOP, RIGHT, BOTTOM).  All values are\nin pixels relative to the origin - the position (0, 0) - of FRAME's\ndisplay.\n\nIf optional argument TYPE is the symbol `outer-edges', return the outer\nedges of FRAME.  The outer edges comprise the decorations of the window\nmanager (like the title bar or external borders) as well as any external\nmenu or tool bar of FRAME.  If optional argument TYPE is the symbol\n`native-edges' or nil, return the native edges of FRAME.  The native\nedges exclude the decorations of the window manager and any external\nmenu or tool bar of FRAME.  If TYPE is the symbol `inner-edges', return\nthe inner edges of FRAME.  These edges exclude title bar, any borders,\nmenu bar or tool bar of FRAME.  ",
	(frame: Lisp_Object, r#type: Lisp_Object),
	{
  return frame_geometry (frame, if EQ (r#type, Qouter_edges ())             // C 7031-7034
				   || EQ (r#type, Qinner_edges ())
				 { r#type } else { Qnative_edges () });
});

/* -----------------------------------------------------------------------------------------
   C 7037-7207: `x_frame_list_z_order` (the comment C 7037-7043), its subr `x-frame-list-z-order`
   (C 7125-7148), `x_frame_restack` (the comment C 7150-7157) and its subr `x-frame-restack`
   (C 7182-7207).  The restack helper's two arms are C 7161's `#ifdef USE_GTK` pair: the GTK one
   calls gtkutil.c's `xg_frame_restack`, the `#else` configures the window manager window.
   -------------------------------------------------------------------------------------- */

/* C: xterm.h 1898: `extern bool x_wm_supports_1 (struct x_display_info *, Atom);` - `src/xterm.c`
   27728 defines it (the port's `src/xterm.rs` 15912, which carries C's linkage) and this unit's
   include graph does not carry the row, so it stands at the use site (PORTING.md P-55/P-72 rule 3).
   C 7064's call.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_wm_supports_1 (dpyinfo: *mut x_display_info, want_atom: Atom) -> bool;
}

/* C: xterm.h 1004: `extern struct frame *x_top_window_to_frame (struct x_display_info *, int);` -
   `src/xterm.c` defines it (the port's `src/xterm.rs` 23137/23204, C's linkage) and no mirror
   carries the row, so it stands at the use site (`src/emacsgtkfixed.rs` 177 declares the same
   row).  C 7085's call, whose `Window` argument C narrows to the header's `int`.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_top_window_to_frame (dpyinfo: *mut x_display_info, w: c_int) -> *mut frame;
}

/* C: Xlib.h 395-401: `typedef struct { int x, y; int width, height; int border_width; Window
   sibling; int stack_mode; } XWindowChanges;` - C 7168's record.  This machine's `<X11/Xlib.h>`
   gives the layout, so the members stand in the header's own order; the row is owed to
   `include/xlib.rs`, whose 148 already names the record.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk")))]
#[repr (C)]
pub struct XWindowChanges
{
  pub x: c_int,                              /* C: `int x, y;`  */
  pub y: c_int,
  pub width: c_int,                          /* C: `int width, height;`  */
  pub height: c_int,
  pub border_width: c_int,                   /* C: `int border_width;`  */
  pub sibling: Window,                       /* C: `Window sibling;`  */
  pub stack_mode: c_int,                     /* C: `int stack_mode;`  */
}

/* C: X11/X.h 415/416/462/463: `#define CWSibling (1<<5)` / `#define CWStackMode (1<<6)` /
   `#define Above 0` / `#define Below 1` - C 7169/7172's four names.  The mirror's
   `x11_constants` module (`include/xlib.rs` 61) carries none of them (`xlib.rs` 148's comment
   names the mask C 7169 needs), so they stand at the use site and are owed to that module; the
   values are this machine's header's.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk")))]
const CWSibling: c_int = 1 << 5;                                          // X.h 415
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk")))]
const CWStackMode: c_int = 1 << 6;                                        // X.h 416
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk")))]
const Above: c_int = 0;                                                   // X.h 462
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk")))]
const Below: c_int = 1;                                                   // X.h 463

/* C: Xlib.h 1885-1890: `extern Status XReconfigureWMWindow (Display *, Window, int, unsigned int,
   XWindowChanges *);` - C 7176's call, on the `not (use_gtk)` arm only (the GTK arm calls
   `xg_frame_restack` instead), and no mirror carries this Xlib entry.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk")))]
unsafe extern "C" {
  pub fn XReconfigureWMWindow (display: *mut Display, w: Window, screen_number: c_int,
			       value_mask: c_uint, changes: *mut XWindowChanges) -> c_int;
}

/* C: gtkutil.h 201: `extern void xg_frame_restack (struct frame *f1, struct frame *f2, bool
   above);` - C 7163's call (`src/gtkutil.rs` 2687 defines it; `src/pgtkfns.rs` 720-722 declares
   the same row).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn xg_frame_restack (f1: *mut frame, f2: *mut frame, above: bool);
}

/* C 7037-7043: "Recursively add list of all frames on the display specified via DPYINFO and whose
   window-system window's parent is specified by WINDOW to FRAMES and return FRAMES."  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_frame_list_z_order (dpyinfo: *mut x_display_info, window: Window) -> Lisp_Object
{
  use crate::x11_constants::{Success, XA_WINDOW};                        // C 7072/7075

  let dpy: *mut Display;                                                 // C 7047
  let mut root: Window = 0;                                              // C 7048
  let mut parent: Window = 0;
  let mut children: *mut Window = std::ptr::null_mut ();
  let mut nchildren: c_uint = 0;                                         // C 7049
  let mut i: c_ulong;                                                    // C 7050
  let mut frames: Lisp_Object = Qnil ();                                 // C 7051
  let mut val: Lisp_Object = Qnil ();
  let mut r#type: Atom = 0;                                              // C 7052
  let toplevels: *mut Window;                                            // C 7053
  let mut format: c_int = 0;                                             // C 7054
  let mut rc: c_int = 0;
  let mut nitems: c_ulong = 0;                                           // C 7055
  let mut bytes_after: c_ulong = 0;
  let mut data: *mut c_uchar = std::ptr::null_mut ();                    // C 7056
  let mut f: *mut frame;                                                 // C 7057

  dpy = (*dpyinfo).display;                                              // C 7059
  data = std::ptr::null_mut ();                                          // C 7060
  frames = Qnil ();                                                      // C 7061

  if window == (*dpyinfo).root_window                                    // C 7063
    && x_wm_supports_1 (dpyinfo, (*dpyinfo).Xatom_net_client_list_stacking)   // C 7064-7065
    {
      rc = XGetWindowProperty ((*dpyinfo).display, (*dpyinfo).root_window,     // C 7067-7070
			       (*dpyinfo).Xatom_net_client_list_stacking,
			       0, LONG_MAX, false as Bool, XA_WINDOW, &raw mut r#type,
			       &raw mut format, &raw mut nitems, &raw mut bytes_after,
			       &raw mut data);

      if rc != Success                                                   // C 7072
	{ return Qnil (); }                                              // C 7073

      if format != 32 || r#type != XA_WINDOW                             // C 7075
	{
	  XFree (data as *mut c_void);                                   // C 7077
	  return Qnil ();                                                // C 7078
	}

      toplevels = data as *mut Window;                                   // C 7081

      i = 0;                                                             // C 7083
      while i < nitems
	{
	  f = x_top_window_to_frame (dpyinfo,                              // C 7085
				     *toplevels.offset (i as isize) as c_int);

	  if !f.is_null ()                                               // C 7087
	    {
	      XSETFRAME! (val, f);                                       // C 7089
	      frames = Fcons (val, frames);                              // C 7090
	    }
	  i += 1;
	}

      XFree (data as *mut c_void);                                       // C 7094
      return frames;                                                     // C 7095
    }

  if XQueryTree (dpy, window, &raw mut root, &raw mut parent, &raw mut children,   // C 7098
		 &raw mut nchildren) != 0
    {
      i = 0;                                                             // C 7100
      while i < nchildren as c_ulong
	{
	  let mut frame: Lisp_Object = Qnil ();                          // C 7102
	  let mut tail: Lisp_Object = Qnil ();

	  FOR_EACH_FRAME! (tail, frame, {                                // C 7104
	      let cf: *mut frame = XFRAME (frame);                       // C 7106
	      /* With a reparenting window manager the parent_desc
		 field usually specifies the topmost windows of our
		 frames.  Otherwise FRAME_OUTER_WINDOW should do.  */    // C 7107-7109
	      if FRAME_X_P (cf)                                          // C 7110
		  && ((*(((*cf).output_data.x) as *mut x_output)).parent_desc == *children.offset (i as isize)   // C 7111
		      || FRAME_OUTER_WINDOW! (cf) == *children.offset (i as isize))   // C 7112
		{ frames = Fcons (frame, frames); }                      // C 7113
	  });
	  i += 1;
	}

      if !children.is_null ()                                            // C 7117
	{ XFree (children as *mut c_void); }                             // C 7118
    }

  return frames;                                                         // C 7121
}

/* C 7125-7148.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-frame-list-z-order", Fx_frame_list_z_order, Sx_frame_list_z_order,
	0, 1, 0,
	doc: "Return list of Emacs's frames, in Z (stacking) order.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be either a frame or a display name (a string).  If\nomitted or nil, that stands for the selected frame's display.  Return\nnil if TERMINAL contains no Emacs frame.\n\nAs a special case, if TERMINAL is non-nil and specifies a live frame,\nreturn the child frames of that frame in Z (stacking) order.\n\nFrames are listed from topmost (first) to bottommost (last).  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);    // C 7139
  let window: Window;                                                    // C 7140

  if FRAMEP (terminal) && FRAME_LIVE_P (XFRAME (terminal))                // C 7142
    { window = FRAME_X_WINDOW! (XFRAME (terminal)); }                     // C 7143
  else
    { window = (*dpyinfo).root_window; }                                  // C 7145

  return x_frame_list_z_order (dpyinfo, window);                          // C 7147
});

/* C 7150-7157: "Restack frame F1 below frame F2, above if ABOVE_FLAG is non-nil.  In practice
   this is a two-step action: The first step removes F1's window-system window from the display.
   The second step reinserts F1's window below (above if ABOVE_FLAG is true) that of F2."  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_frame_restack (f1: *mut frame, f2: *mut frame, above_flag: bool)
{
  #[cfg (feature = "use_gtk")]                                           // C 7161
  {
    block_input ();                                                       // C 7162
    xg_frame_restack (f1, f2, above_flag);                                // C 7163
    unblock_input ();                                                     // C 7164
  }
  #[cfg (not (feature = "use_gtk"))]                                     // C 7165
  {
    let dpy: *mut Display = FRAME_X_DISPLAY! (f1);                        // C 7166
    let window1: Window = FRAME_OUTER_WINDOW! (f1);                       // C 7167
    let mut wc: XWindowChanges = std::mem::zeroed ();                     // C 7168
    let mask: c_ulong = (CWSibling | CWStackMode) as c_ulong;             // C 7169

    wc.sibling = FRAME_OUTER_WINDOW! (f2);                                // C 7171
    wc.stack_mode = if above_flag { Above } else { Below };               // C 7172
    block_input ();                                                       // C 7173
    /* Configure the window manager window (a normal XConfigureWindow
       won't cut it).  This should also work for child frames.  */        // C 7174-7175
    XReconfigureWMWindow (dpy, window1, FRAME_X_SCREEN_NUMBER! (f1),      // C 7176
			  mask as c_uint, &raw mut wc);
    unblock_input ();                                                     // C 7177
  }
}

/* C 7182-7207.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-frame-restack", Fx_frame_restack, Sx_frame_restack,
	2, 3, 0,
	doc: "Restack FRAME1 below FRAME2.\nThis means that if both frames are visible and the display areas of\nthese frames overlap, FRAME2 (partially) obscures FRAME1.  If optional\nthird argument ABOVE is non-nil, restack FRAME1 above FRAME2.  This\nmeans that if both frames are visible and the display areas of these\nframes overlap, FRAME1 (partially) obscures FRAME2.\n\nThis may be thought of as an atomic action performed in two steps: The\nfirst step removes FRAME1's window-step window from the display.  The\nsecond step reinserts FRAME1's window below (above if ABOVE is true)\nthat of FRAME2.  Hence the position of FRAME2 in its display's Z\n(stacking) order relative to all other frames excluding FRAME1 remains\nunaltered.\n\nSome window managers may refuse to restack windows.  ",
	(frame1: Lisp_Object, frame2: Lisp_Object, above: Lisp_Object),
	{
  let f1: *mut frame = decode_live_frame (frame1);                        // C 7200
  let f2: *mut frame = decode_live_frame (frame2);                        // C 7201

  if ! (FRAME_OUTER_WINDOW! (f1) != 0 && FRAME_OUTER_WINDOW! (f2) != 0)   // C 7203
    { error (c"Cannot restack frames".as_ptr ()); }                       // C 7204
  x_frame_restack (f1, f2, !NILP (above));                                // C 7205
  return Qt ();                                                           // C 7206
});

/* -----------------------------------------------------------------------------------------
   C 7210-7273: the two mouse subrs - `x-mouse-absolute-pixel-position` (C 7210-7233) and
   `x-set-mouse-absolute-pixel-position` (C 7235-7273, whose C 7251-7266 `HAVE_XINPUT2` pair
   chooses `XIWarpPointer` on the client pointer device or `XWarpPointer`).
   -------------------------------------------------------------------------------------- */

/* C: Xlib.h 3524-3534: `extern int XWarpPointer (Display *, Window, Window, int, int, unsigned
   int, unsigned int, int, int);` - C 7267/7269's call, and the header is on this machine.  The
   file's posture for Xlib entries is the use-site row (slices 9 and 21's `XGetGeometry` and
   `XReconfigureWMWindow`), so this closes the row `src/xterm.rs` 17421-17422 predicts for this
   file rather than keeping it bare (that comment is corrected in this slice).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XWarpPointer (display: *mut Display, src_w: Window, dest_w: Window, src_x: c_int,
		       src_y: c_int, src_width: c_uint, src_height: c_uint, dest_x: c_int,
		       dest_y: c_int) -> c_int;
}

/* C: `<X11/extensions/XInput2.h>`: `extern int XIWarpPointer (Display *, int, Window, Window, int,
   int, int, int, int, int);` - C 7260's call.  This machine has no XInput2.h
   (`/opt/homebrew/include/X11/extensions/` carries `XI2.h`, the protocol header, and no
   declaration), so the row names the header and the signature and is **UNVERIFIED**; it is owed
   to an XInput2 mirror.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe extern "C" {
  pub fn XIWarpPointer (display: *mut Display, deviceid: c_int, src_w: Window, dest_w: Window,
			src_x: c_int, src_y: c_int, src_width: c_int, src_height: c_int,
			dest_x: c_int, dest_y: c_int) -> c_int;
}

/* C: xterm.h 1755/1756/1759: `extern void x_catch_errors_for_lisp (struct x_display_info *);` /
   `… x_uncatch_errors_for_lisp (…);` / `extern void x_check_errors_for_lisp (struct
   x_display_info *, const char *);` - C 7259/7263's pair uses the XInput2 gate's arm, and
   C 7935/7944/7946's uses stand under the file gate (status 3.930 widened the row's gate and
   added the third name).  `src/xterm.rs` 21835/21858 define the pair; `src/xselect.rs` 497-502
   declares the same rows.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_catch_errors_for_lisp (dpyinfo: *mut x_display_info);
  pub fn x_check_errors_for_lisp (dpyinfo: *mut x_display_info, format: *const c_char);
  pub fn x_uncatch_errors_for_lisp (dpyinfo: *mut x_display_info);
}

/* C 7210-7233.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-mouse-absolute-pixel-position", Fx_mouse_absolute_pixel_position,
	Sx_mouse_absolute_pixel_position,
	0, 0, 0,
	doc: "Return absolute position of mouse cursor in pixels.\nThe position is returned as a cons cell (X . Y) of the coordinates of\nthe mouse cursor position in pixels relative to a position (0, 0) of the\nselected frame's display.  ",
	(),
	{
  let f: *mut frame = SELECTED_FRAME ();                                  // C 7218
  let mut root: Window = 0;                                               // C 7219
  let mut dummy_window: Window = 0;
  let mut x: c_int = 0;                                                   // C 7220
  let mut y: c_int = 0;
  let mut dummy: c_int = 0;

  if FRAME_INITIAL_P (f) || !FRAME_X_P (f)                                 // C 7222
    { return Qnil (); }                                                   // C 7223

  block_input ();                                                         // C 7225
  x_query_pointer (FRAME_X_DISPLAY! (f),                                  // C 7226
		   (*FRAME_DISPLAY_INFO! (f)).root_window,                 // C 7227
		   &raw mut root, &raw mut dummy_window,                   // C 7228
		   &raw mut x, &raw mut y,
		   &raw mut dummy, &raw mut dummy,                         // C 7229
		   &raw mut dummy as *mut c_uint);
  unblock_input ();                                                       // C 7230

  return Fcons (make_fixnum (x as EMACS_INT),                             // C 7232
		make_fixnum (y as EMACS_INT));
});

/* C 7235-7273.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-set-mouse-absolute-pixel-position", Fx_set_mouse_absolute_pixel_position,
	Sx_set_mouse_absolute_pixel_position,
	2, 2, 0,
	doc: "Move mouse pointer to absolute pixel position (X, Y).\nThe coordinates X and Y are interpreted in pixels relative to a position\n(0, 0) of the selected frame's display.  ",
	(x: Lisp_Object, y: Lisp_Object),
	{
  let f: *mut frame = SELECTED_FRAME ();                                  // C 7242

  if FRAME_INITIAL_P (f) || !FRAME_X_P (f)                                 // C 7244
    { return Qnil (); }                                                   // C 7245

  let xval: c_int = check_integer_range (x, INT_MIN as intmax_t,          // C 7247
					 INT_MAX as intmax_t) as c_int;
  let yval: c_int = check_integer_range (y, INT_MIN as intmax_t,          // C 7248
					 INT_MAX as intmax_t) as c_int;

  block_input ();                                                         // C 7250
  #[cfg (feature = "have_xinput2")]                                       // C 7251
  {
    let mut deviceid: c_int;                                              // C 7252

    deviceid = (*FRAME_DISPLAY_INFO! (f)).client_pointer_device;          // C 7254

    if (*FRAME_DISPLAY_INFO! (f)).supports_xi2 && deviceid != -1          // C 7256-7257
      {
	use crate::x11_constants::None;                                   // X.h 115's `None`
	x_catch_errors_for_lisp (FRAME_DISPLAY_INFO! (f));                // C 7259
	XIWarpPointer (FRAME_X_DISPLAY! (f), deviceid, None as Window,    // C 7260-7262
		       (*FRAME_DISPLAY_INFO! (f)).root_window,
		       0, 0, 0, 0, xval, yval);
	x_uncatch_errors_for_lisp (FRAME_DISPLAY_INFO! (f));              // C 7263
      }
    else
      {
	use crate::x11_constants::None;
	XWarpPointer (FRAME_X_DISPLAY! (f), None as Window,               // C 7267-7269
		      (*FRAME_DISPLAY_INFO! (f)).root_window,
		      0, 0, 0, 0, xval, yval);
      }
  }
  #[cfg (not (feature = "have_xinput2"))]                                 // C 7266's `#endif`
  {
    use crate::x11_constants::None;
    XWarpPointer (FRAME_X_DISPLAY! (f), None as Window,                   // C 7267-7269
		  (*FRAME_DISPLAY_INFO! (f)).root_window,
		  0, 0, 0, 0, xval, yval);
  }
  unblock_input ();                                                       // C 7270

  return Qnil ();                                                         // C 7272
});

/* -----------------------------------------------------------------------------------------
   C 7275-7447: the `x-begin-drag` subr - the target list walk (C 7350-7368), the action dispatch
   (C 7370-7426), the atom interning under C 7430-7437's error trap and C 7439-7443's
   `x_dnd_begin_drag_and_drop` call, all inside C 7348/7445's `USE_SAFE_ALLOCA`/`SAFE_FREE` pair.
   -------------------------------------------------------------------------------------- */

/* C: xterm.h 1927: `extern Atom symbol_to_x_atom (struct x_display_info *, Lisp_Object);` -
   C 7384's call (`src/xselect.rs` 540 defines it; `src/xselect.rs` 505-506 declares the
   `x_intern_atoms` row below the same way).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn symbol_to_x_atom (dpyinfo: *mut x_display_info, sym: Lisp_Object) -> Atom;
}

/* C: xterm.h 1843 and 1859-1862: `extern void x_intern_atoms (struct x_display_info *, char **,
   int, Atom *);` and `extern Lisp_Object x_dnd_begin_drag_and_drop (struct frame *, Time, Atom,
   Lisp_Object, Atom *, const char **, int, bool, Atom *, int, Lisp_Object, bool);` - C 7433 and
   C 7439's calls.  Both are `src/xterm.c`'s (the port's `src/xterm.rs` 19211 and 23502).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_intern_atoms (dpyinfo: *mut x_display_info, names: *mut *mut c_char, n: c_int,
			 atoms: *mut Atom);
  pub fn x_dnd_begin_drag_and_drop (f: *mut frame, time: Time, xaction: Atom,
				    return_frame: Lisp_Object, ask_action_list: *mut Atom,
				    ask_action_names: *const *const c_char, n_ask_actions: c_int,
				    allow_current_frame: bool, target_atoms: *mut Atom,
				    ntargets: c_int, selection_target_list: Lisp_Object,
				    follow_tooltip: bool) -> Lisp_Object;
}

/* C 7275-7447.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-begin-drag", Fx_begin_drag, Sx_begin_drag,
	1, 6, 0,
	doc: "Begin dragging contents on FRAME, with targets TARGETS.\nTARGETS is a list of strings, which defines the X selection targets\nthat will be available to the drop target.  Block until the mouse\nbuttons are released, then return the action chosen by the target, or\n`nil' if the drop was not accepted by the drop target.  Dragging\nstarts when the mouse is pressed on FRAME, and the contents of the\nselection `XdndSelection' will be sent to the X window underneath the\nmouse pointer (the drop target) when the mouse button is released.\n\nACTION is a symbol which tells the target what it should do, and can\nbe one of the following:\n\n - `XdndActionCopy', which means to copy the contents from the drag\n   source (FRAME) to the drop target.\n\n - `XdndActionMove', which means to first take the contents of\n   `XdndSelection', and to delete whatever was saved into that\n   selection afterwards.\n\n`XdndActionPrivate' is also a valid return value, and means that the\ndrop target chose to perform an unspecified or unknown action.\n\nThe source is also expected to cooperate with the target to perform\nthe action chosen by the target.  For example, callers should delete\nthe buffer text that was dragged if `XdndActionMove' is returned.\n\nThere are also some other valid values of ACTION that depend on\ndetails of both the drop target's implementation details and that of\nEmacs.  For that reason, they are not mentioned here.  Consult\n\"Drag-and-Drop Protocol for the X Window System\" for more details:\nhttps://freedesktop.org/wiki/Specifications/XDND/.\n\nIf RETURN-FRAME is non-nil, this function will return the frame if the\nmouse pointer moves onto an Emacs frame, after first moving out of\nFRAME.  (This is not guaranteed to work on some systems.)  If\nRETURN-FRAME is the symbol `now', any frame underneath the mouse\npointer will be returned immediately.\n\nIf ACTION is a list and not nil, its elements are assumed to be a cons\nof (ITEM . STRING), where ITEM is the name of an action, and STRING is\na string describing ITEM to the user.  The drop target is expected to\nprompt the user to choose between any of the actions in the list.\n\nIf ACTION is not specified or nil, `XdndActionCopy' is used\ninstead.\n\nIf ALLOW-CURRENT-FRAME is not specified or nil, then the drop target\nis allowed to be FRAME.  Otherwise, no action will be taken if the\nmouse buttons are released on top of FRAME.\n\nIf FOLLOW-TOOLTIP is non-nil, any tooltip currently being displayed\nwill be moved to follow the mouse pointer while the drag is in\nprogress.  Note that this does not work with system tooltips (tooltips\ncreated when `use-system-tooltips' is non-nil).\n\nThis function will sometimes return immediately if no mouse buttons\nare currently held down.  It should only be called when it is known\nthat mouse buttons are being held down, such as immediately after a\n`down-mouse-1' (or similar) event.  ",
	(targets: Lisp_Object, action: Lisp_Object, frame: Lisp_Object,
	 return_frame: Lisp_Object, allow_current_frame: Lisp_Object,
	 follow_tooltip: Lisp_Object),
	{
  let f: *mut frame = decode_window_system_frame (frame);                 // C 7339
  let mut ntargets: c_int = 0;                                           // C 7340
  let mut nnames: c_int = 0;
  let mut target_names: [*mut c_char; 2048] = [std::ptr::null_mut (); 2048];   // C 7341
  let mut target_atoms: *mut Atom = std::ptr::null_mut ();               // C 7342
  let mut lval: Lisp_Object = Qnil ();                                   // C 7343
  let mut original: Lisp_Object = Qnil ();
  let mut targets_arg: Lisp_Object = Qnil ();
  let mut tem: Lisp_Object = Qnil ();
  let mut t1: Lisp_Object = Qnil ();
  let mut t2: Lisp_Object = Qnil ();
  let mut xaction: Atom;                                                 // C 7344
  let mut action_list: [Atom; 2048] = [0; 2048];                         // C 7345
  let mut name_list: [*mut c_char; 2048] = [std::ptr::null_mut (); 2048];   // C 7346

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                                 // C 7348

  /* C's two `FOR_EACH_TAIL`s walk their own parameters (the macro assigns the parameter back
     to itself), and a DEFUN! argument carries no `mut` (the macro takes `$arg:ident`), so
     each walk reads a mutable copy of the same name (`androidfns.rs` 581's shape).  */
  let mut targets = targets;                                             // C 7354
  let mut action = action;                                               // C 7391

  CHECK_LIST (targets);                                                  // C 7350
  original = targets;                                                    // C 7351
  targets_arg = targets;                                                 // C 7352

  FOR_EACH_TAIL! (targets, {                                             // C 7354
      CHECK_STRING (XCAR (targets));                                     // C 7356

      if ntargets < 2048                                                 // C 7358
	{
	  /* C's `SAFE_ALLOCA_STRING (target_names[ntargets], XCAR (targets));` - the mirror's
	     macro takes a `$ptr:ident` and C passes an array-element lvalue, so the expansion
	     stands here (`image.rs` 8899-8906's shape, `fileio.rs` 1034 the ident form).  */
	  target_names[ntargets as usize]
	    = SAFE_ALLOCA! (sa_avail, SBYTES (XCAR (targets)) + 1) as *mut c_char;
	  std::ptr::copy_nonoverlapping (SDATA (XCAR (targets)),
					 target_names[ntargets as usize] as *mut u8,
					 (SBYTES (XCAR (targets)) + 1) as usize);
	  ntargets += 1;                                                     // C 7362
	}
      else
	{ error (c"Too many targets".as_ptr ()); }                           // C 7365
  });

  CHECK_LIST_END (targets, original);                                    // C 7368

  if NILP (action) || EQ (action, QXdndActionCopy ())                    // C 7370
    { xaction = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionCopy; }       // C 7371
  else if EQ (action, QXdndActionMove ())                                // C 7372
    { xaction = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionMove; }       // C 7373
  else if EQ (action, QXdndActionLink ())                                // C 7374
    { xaction = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionLink; }       // C 7375
  else if EQ (action, QXdndActionPrivate ())                             // C 7376
    { xaction = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionPrivate; }    // C 7377
  else if EQ (action, QXdndActionAsk ())                                 // C 7378
    { xaction = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionAsk; }        // C 7379
  else if SYMBOLP (action)                                               // C 7380
    /* This is to accommodate non-standard DND protocols such as XDS
       that are explicitly implemented by Emacs, and is not documented
       for that reason.  */                                             // C 7381-7383
    { xaction = symbol_to_x_atom (FRAME_DISPLAY_INFO! (f), action); }    // C 7384
  else if CONSP (action)                                                 // C 7385
    {
      xaction = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionAsk;          // C 7387
      original = action;                                                 // C 7388

      CHECK_LIST (action);                                               // C 7390
      FOR_EACH_TAIL! (action, {                                          // C 7391
	  tem = XCAR (action);                                           // C 7393
	  CHECK_CONS (tem);                                              // C 7394
	  t1 = XCAR (tem);                                               // C 7395
	  t2 = XCDR (tem);                                               // C 7396
	  CHECK_SYMBOL (t1);                                             // C 7397
	  CHECK_STRING (t2);                                             // C 7398

	  if nnames < 2048                                               // C 7400
	    {
	      if EQ (t1, QXdndActionCopy ())                             // C 7402
		{ action_list[nnames as usize] = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionCopy; }    // C 7403
	      else if EQ (t1, QXdndActionMove ())                        // C 7404
		{ action_list[nnames as usize] = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionMove; }    // C 7405
	      else if EQ (t1, QXdndActionLink ())                        // C 7406
		{ action_list[nnames as usize] = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionLink; }    // C 7407
	      else if EQ (t1, QXdndActionAsk ())                         // C 7408
		{ action_list[nnames as usize] = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionAsk; }     // C 7409
	      else if EQ (t1, QXdndActionPrivate ())                     // C 7410
		{ action_list[nnames as usize] = (*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionPrivate; }  // C 7411
	      else
		{ signal_error (c"Invalid drag-and-drop action".as_ptr (), tem); }   // C 7413

	      /* C's `SAFE_ALLOCA_STRING (name_list[nnames], ENCODE_SYSTEM (t2));` - the same
		 array-element expansion as above, and C's macro evaluates its string argument
		 three times, which `ENCODE_SYSTEM`'s three calls here mirror.  */
	      name_list[nnames as usize]
		= SAFE_ALLOCA! (sa_avail, SBYTES (ENCODE_SYSTEM (t2)) + 1) as *mut c_char;
	      std::ptr::copy_nonoverlapping (SDATA (ENCODE_SYSTEM (t2)),
					     name_list[nnames as usize] as *mut u8,
					     (SBYTES (ENCODE_SYSTEM (t2)) + 1) as usize);

	      nnames += 1;                                                   // C 7418
	    }
	  else
	    { error (c"Too many actions".as_ptr ()); }                       // C 7421
      });
      CHECK_LIST_END (action, original);                                 // C 7423
    }
  else
    { signal_error (c"Invalid drag-and-drop action".as_ptr (), action); }   // C 7426

  SAFE_NALLOCA! (sa_avail, target_atoms, 1, ntargets);                   // C 7428

  /* Catch errors since interning lots of targets can potentially
     generate a BadAlloc error.  */                                     // C 7430-7431
  x_catch_errors (FRAME_X_DISPLAY! (f));                                 // C 7432
  x_intern_atoms (FRAME_DISPLAY_INFO! (f), target_names.as_mut_ptr (),   // C 7433-7434
		  ntargets, target_atoms);
  x_check_errors (FRAME_X_DISPLAY! (f),                                  // C 7435-7436
		  c"Failed to intern target atoms: %s".as_ptr ());
  x_uncatch_errors_after_check ();                                       // C 7437

  lval = x_dnd_begin_drag_and_drop (f,                                    // C 7439-7443
				    (*FRAME_DISPLAY_INFO! (f)).last_user_time,
				    xaction, return_frame, action_list.as_mut_ptr (),
				    name_list.as_ptr () as *const *const c_char, nnames,
				    !NILP (allow_current_frame), target_atoms, ntargets,
				    targets_arg, !NILP (follow_tooltip));

  SAFE_FREE! (sa_count, sa_mark);                                                 // C 7445
  return lval;                                                          // C 7446
});

/* -----------------------------------------------------------------------------------------
   C 7449-7491: the `X Displays` section (C 7449-7451), C 7454-7470's visual-name table and
   C 7473-7491's `XScreenNumberOfScreen` substitute.  `select_visual` (C 7494-7606) follows in the
   next slice and is the table's only reader.
   -------------------------------------------------------------------------------------- */

/* C: X11/X.h 704-709: `#define StaticGray 0` / `#define GrayScale 1` / `#define StaticColor 2` /
   `#define PseudoColor 3` / `#define TrueColor 4` / `#define DirectColor 5` - the six class names
   C 7463-7468's table initialises `class` with.  All six are the mirror's `x11_constants` rows
   (`include/xlib.rs` 180-185, with the same header lines) and `include/xterm.rs` 114-125 already
   imports three of them (`StaticColor`, `StaticGray`, `TrueColor`) at the crate root this
   `include!` graph builds, so only the other three are imported here - the file's own shape
   (412's `use crate::x11_constants::{…}`), and no glob (which would bring the module's `None`
   into the crate root, as `include/xterm.rs` 109-111 records).  */
#[cfg (feature = "have_x_windows")]
use crate::x11_constants::{GrayScale, PseudoColor, DirectColor};

/* C 7454-7470: "Mapping visual names to visuals."  C's record and its table.  The table's only
   reader is `select_visual` (C 7532-7535), the next slice's first item, so it carries the
   `dead_code` allowance it needed before `select_visual` landed is gone: the reads are by value
   (the record is `Copy`), which `static mut` allows.  */
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct visual_class
{
  pub name: *const c_char,                 /* C: `const char *name;`  */
  pub class: c_int,                        /* C: `int class;`  */
}
#[cfg (feature = "have_x_windows")]
static mut visual_classes: [visual_class; 7] =
  [
    visual_class { name: c"StaticGray".as_ptr (), class: StaticGray },     // C 7463
    visual_class { name: c"GrayScale".as_ptr (), class: GrayScale },       // C 7464
    visual_class { name: c"StaticColor".as_ptr (), class: StaticColor },   // C 7465
    visual_class { name: c"PseudoColor".as_ptr (), class: PseudoColor },   // C 7466
    visual_class { name: c"TrueColor".as_ptr (), class: TrueColor },       // C 7467
    visual_class { name: c"DirectColor".as_ptr (), class: DirectColor },   // C 7468
    visual_class { name: std::ptr::null (), class: 0 },                    // C 7469's sentinel
  ];

/* C 7473-7491: `#ifndef HAVE_XSCREENNUMBEROFSCREEN` … `#endif /* not HAVE_XSCREENNUMBEROFSCREEN */`
   - "Value is the screen number of screen SCR.  This is a substitute for the X function with the
   same name when that doesn't exist."  `configure` tests the macro (`src/config.in` 1905's
   template) and this tree's feature table does not register it, so this is the live arm and the
   file's `extern` row (1960-1961) stands under the feature instead (P-86 ①: both arms carried,
   the gate written as the lowercase feature).  C's definition is non-`static` and Xlib.h declares
   it, so it carries C's linkage (PORTING.md 0.2/1.4) - which retires `--check`'s standing row
   ("function XScreenNumberOfScreen is not static in C but has no #[unsafe(no_mangle)] in the
   port").  The two members C reads (`dpy->nscreens`, `dpy->screens`) are inside `_XDisplay`'s
   body, which this machine's Xlib.h keeps behind `XLIB_ILLEGAL_ACCESS`, so the mirror's
   `_XDisplay` stays opaque (`include/xlib.rs` 1043) and those two reads are that class's rows.  */
#[cfg (all (feature = "have_x_windows", not (feature = "have_xscreennumberofscreen")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XScreenNumberOfScreen (scr: *mut Screen) -> c_int
{
  let dpy: *mut Display = (*scr).display;                                 // C 7481
  let mut i: c_int;                                                       // C 7482

  i = 0;                                                                  // C 7484
  while i < (*dpy).nscreens
    {
      if scr == (*dpy).screens.offset (i as isize)                        // C 7485
	{ break; }                                                        // C 7486
      i += 1;
    }

  return i;                                                               // C 7488
}

/* -----------------------------------------------------------------------------------------
   C 7494-7606: `select_visual` - "Select the visual that should be used on display DPYINFO.  Set
   members of DPYINFO appropriately.  Called from x_term_init."  It is `visual_classes`' only
   reader (C 7532-7535), whose `dead_code` allowance is therefore gone.
   -------------------------------------------------------------------------------------- */

/* C: Xutil.h 305/306: `#define VisualIDMask 0x1` / `#define VisualScreenMask 0x2` - C 7569 and
   C 7598's masks.  The mirror's `x11_constants` module does not carry them, so they stand at the
   use site and are owed to it; the values are this machine's header's.  */
#[cfg (feature = "have_x_windows")]
const VisualIDMask: c_int = 0x1;                                          // Xutil.h 305
#[cfg (feature = "have_x_windows")]
const VisualScreenMask: c_int = 0x2;                                      // Xutil.h 306

/* C: Xutil.h 471-476 and 542-548: `extern XVisualInfo *XGetVisualInfo (Display *, long,
   XVisualInfo *, int *);` and `extern Status XMatchVisualInfo (Display *, int, int, int,
   XVisualInfo *);` - C 7569/7598's and C 7541's calls; no mirror carries either.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XGetVisualInfo (display: *mut Display, vinfo_mask: c_long,
			 vinfo_template: *mut XVisualInfo, nitems_return: *mut c_int)
			 -> *mut XVisualInfo;
  pub fn XMatchVisualInfo (display: *mut Display, screen: c_int, depth: c_int, class: c_int,
			   vinfo_return: *mut XVisualInfo) -> c_int;
}

/* C: Xrender.h 57-74 and 87-98: `XRenderDirectFormat` (`short` red, redMask, green, greenMask,
   blue, blueMask, alpha, alphaMask) and `XRenderPictFormat` (`PictFormat id; int type; int depth;
   XRenderDirectFormat direct; Colormap colormap;`), with 482-484's
   `XRenderPictFormat *XRenderFindVisualFormat (Display *, Visual *);` and render.h 89-90's
   `#define PictTypeIndexed 0` / `#define PictTypeDirect 1` - C 7559/7574/7575/7576/7577's reads.
   Both headers are on this machine, so the members' order and the constants' values are theirs;
   render.h's `PictFormat` is `typedef XID PictFormat;`, which this graph has no name for, so the
   field keeps `XID`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
const PictTypeDirect: c_int = 1;                                          // render.h 90
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XRenderDirectFormat
{
  pub red: c_short,                        /* C: `short red;`  */
  pub redMask: c_short,                    /* C: `short redMask;`  */
  pub green: c_short,
  pub greenMask: c_short,
  pub blue: c_short,
  pub blueMask: c_short,
  pub alpha: c_short,                      /* C: `short alpha;` (C 7577's `alphaMask`)  */
  pub alphaMask: c_short,
}
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XRenderPictFormat
{
  pub id: XID,                             /* C: `PictFormat id;` (render.h's `typedef XID`)  */
  pub r#type: c_int,                       /* C: `int type;` (C 7576 reads it)  */
  pub depth: c_int,                        /* C: `int depth;`  */
  pub direct: XRenderDirectFormat,          /* C: `XRenderDirectFormat direct;`  */
  pub colormap: Colormap,                  /* C: `Colormap colormap;`  */
}
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
unsafe extern "C" {
  pub fn XRenderFindVisualFormat (dpy: *mut Display, visual: *mut Visual) -> *mut XRenderPictFormat;
}

/* C: Xlib.h 128: `#define DefaultVisualOfScreen(s)((s)->root_visual)` - C 7596's call; the
   mirror's `Screen` carries `root_visual` (`include/xlib.rs` 1128), so the macro keeps C's name
   and reads it (the `WidthMMOfScreen` shape at 8194-8200).  */
#[cfg (feature = "have_x_windows")]
macro_rules! DefaultVisualOfScreen { ($s:expr) => { (*$s).root_visual }; }     // Xlib.h 128

/* C: Xlib.h 1724-1726: `extern VisualID XVisualIDFromVisual (Visual *);` - C 7597's call; the
   name stands in `include/gtk.rs` only inside comments (1402-1404), so this is the use-site row
   (the Xutil.h pair above, same header group).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XVisualIDFromVisual (visual: *mut Visual) -> VisualID;
}

/* C: dispextern.h 104: `extern int xstrcasecmp (const char *, const char *);` - C 7533's call
   (`src/font.rs` 298 defines it; `src/fontset.rs` 108 declares the same row).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn xstrcasecmp (a: *const c_char, b: *const c_char) -> c_int;
}

/* C: `<stdlib.h>`: `int atoi (const char *);` - C 7523's call (the `lib/include/stdlib.rs` mirror
   carries `free` and `malloc` only; `src/font.rs` 386-387 declares the same row).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn atoi (nptr: *const c_char) -> c_int;
}

/* C: lisp.h 5371: `extern AVOID fatal (const char *msgid, ...);` - C 7543/7601's calls; no
   mirror carries it, so it stands at the use site (`src/eval.rs` 100's shape).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn fatal (format: *const c_char, ...) -> !;
}

/* C 7497-7606.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn select_visual (dpyinfo: *mut x_display_info)
{
  let dpy: *mut Display = (*dpyinfo).display;                             // C 7500
  let screen: *mut Screen = (*dpyinfo).screen;                            // C 7501

  /* See if a visual is specified.  */                                    // C 7503
  AUTO_STRING! (visualClass, c"visualClass".as_ptr ());                   // C 7504
  AUTO_STRING! (VisualClass, c"VisualClass".as_ptr ());                   // C 7505
  let mut value: Lisp_Object = gui_display_get_resource (dpyinfo, visualClass,   // C 7506-7507
							 VisualClass, Qnil (), Qnil ());

  if STRINGP (value)                                                      // C 7509
    {
      /* VALUE should be of the form CLASS-DEPTH, where CLASS is one
	 of `PseudoColor', `TrueColor' etc. and DEPTH is the color
	 depth, a decimal number.  NAME is compared with case ignored.  */   // C 7511-7513
      /* C's bare `alloca` is the port's stack pool (PORTING.md P-05/3.2).  */
      let s: *mut c_char = sa_stack_alloc ((SBYTES (value) + 1) as usize) as *mut c_char;   // C 7514
      let mut dash: *mut c_char;                                          // C 7515
      let mut i: c_int = 0;                                               // C 7516
      let mut class: c_int = -1;
      let mut vinfo: XVisualInfo = std::mem::zeroed ();                   // C 7517

      lispstpcpy (s, value);                                              // C 7519
      dash = strchr (s, b'-' as c_int);                                   // C 7520
      if !dash.is_null ()                                                 // C 7521
	{
	  (*dpyinfo).n_planes = atoi (dash.offset (1));                      // C 7523
	  *dash = b'\0' as c_char;                                          // C 7524
	}
      else
	/* We won't find a matching visual with depth 0, so that
	   an error will be printed below.  */                            // C 7527-7528
	{ (*dpyinfo).n_planes = 0; }                                      // C 7529

      /* Determine the visual class.  */                                  // C 7531
      i = 0;                                                              // C 7532
      while !(visual_classes[i as usize].name).is_null ()                 // C 7532's loop test
	{
	  if xstrcasecmp (s, visual_classes[i as usize].name) == 0         // C 7533
	    {
	      class = visual_classes[i as usize].class;                    // C 7535
	      break;                                                      // C 7536
	    }
	  i += 1;
	}

      /* Look up a matching visual for the specified class.  */           // C 7539
      if class == -1                                                      // C 7540
	|| XMatchVisualInfo (dpy, XScreenNumberOfScreen (screen),          // C 7541-7542
			     (*dpyinfo).n_planes, class, &raw mut vinfo) == 0
	{ fatal (c"Invalid visual specification '%s'".as_ptr (),           // C 7543-7544
		 SSDATA (ENCODE_SYSTEM (value))); }

      (*dpyinfo).visual = vinfo.visual;                                   // C 7546
      (*dpyinfo).visual_info = vinfo;                                     // C 7547
    }
  else
    {
      let mut n_visuals: c_int = 0;                                       // C 7551
      let mut vinfo: *mut XVisualInfo;                                    // C 7552
      let mut vinfo_template: XVisualInfo = std::mem::zeroed ();

      vinfo_template.screen = XScreenNumberOfScreen (screen);             // C 7554

      #[cfg (all (not (feature = "use_x_toolkit"),
		  not (all (feature = "use_gtk", not (feature = "have_gtk3"))),
		  feature = "have_xrender"))]                             // C 7556-7557
      {
	let mut i: c_int;                                                 // C 7558
	let mut format: *mut XRenderPictFormat;                           // C 7559

	/* First attempt to find a visual with an alpha mask if
	   available.  That information is only available when the
	   render extension is present, and we cannot do much with such
	   a visual if it isn't.  */                                      // C 7561-7564

	if (*dpyinfo).xrender_supported_p                                 // C 7566
	  {
	    vinfo = XGetVisualInfo (dpy, VisualScreenMask as c_long,      // C 7569-7570
				    &raw mut vinfo_template, &raw mut n_visuals);

	    i = 0;                                                        // C 7572
	    while i < n_visuals
	      {
		format = XRenderFindVisualFormat (dpy,                    // C 7574
						  (*vinfo.offset (i as isize)).visual);

		if !format.is_null ()                                     // C 7576
		    && (*format).r#type == PictTypeDirect                  // C 7576
		    && (*format).direct.alphaMask != 0                     // C 7577
		  {
		    (*dpyinfo).n_planes = (*vinfo.offset (i as isize)).depth;      // C 7579
		    (*dpyinfo).visual = (*vinfo.offset (i as isize)).visual;       // C 7580
		    (*dpyinfo).visual_info = *vinfo.offset (i as isize);           // C 7581
		    (*dpyinfo).pict_format = format as *mut c_void;                // C 7582

		    XFree (vinfo as *mut c_void);                              // C 7584
		    return;                                                   // C 7585
		  }
		i += 1;
	      }

	    if !vinfo.is_null ()                                          // C 7589
	      { XFree (vinfo as *mut c_void); }                           // C 7590
	  }
      }

      /* Visual with alpha channel (or the Render extension) not
	 available, fallback to default visual.  */                      // C 7594-7595
      (*dpyinfo).visual = DefaultVisualOfScreen! (screen);                // C 7596
      vinfo_template.visualid = XVisualIDFromVisual ((*dpyinfo).visual);  // C 7597
      vinfo = XGetVisualInfo (dpy, (VisualIDMask | VisualScreenMask) as c_long,  // C 7598-7599
			      &raw mut vinfo_template, &raw mut n_visuals);
      if n_visuals <= 0                                                   // C 7600
	{ fatal (c"Can't get proper X visual info".as_ptr ()); }          // C 7601
      (*dpyinfo).visual_info = *vinfo;                                    // C 7602
      (*dpyinfo).n_planes = (*vinfo).depth;                               // C 7603
      XFree (vinfo as *mut c_void);                                       // C 7604
    }
}

/* -----------------------------------------------------------------------------------------
   C 7609-7729: the connection group - `x_display_info_for_name` (its comment is C 7609-7610),
   `x-open-connection` (C 7637-7677), `x-close-connection` (C 7679-7695), `x-display-list`
   (C 7697-7708) and `x-synchronize` (C 7710-7729).
   -------------------------------------------------------------------------------------- */

/* C: xterm.h 1005: `extern struct x_display_info *x_term_init (Lisp_Object, char *, char *);` -
   C 7628/7660's calls.  `include/xterm.rs` 1759 records the prototype as text inside its
   prototype block (fifteen names of that block stand the same way), so the row is the use-site
   one (P-55/P-72 rule 3).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_term_init (name: Lisp_Object, resource_name: *mut c_char,
		      resource_string: *mut c_char) -> *mut x_display_info;
}

/* C: xterm.h 1784: `extern void x_delete_terminal (struct terminal *);` - C 7692's call
   (`src/term.c` defines it; no mirror carries the row).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_delete_terminal (terminal: *mut terminal);
}

/* C: Xlib.h 1518-1523: `extern int (*XSynchronize (Display *, Bool)) (Display *);` - C 7726's
   call, which drops the returned handler (the header's own pointer-to-function form, so the
   declaration's result is the function pointer itself).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XSynchronize (display: *mut Display, onoff: Bool)
		       -> Option<unsafe extern "C" fn (*mut Display) -> c_int>;
}

/* C 7609-7610: "Return the X display structure for the display named NAME.  Open a new connection
   if necessary."  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_display_info_for_name (name: Lisp_Object) -> *mut x_display_info
{
  let mut dpyinfo: *mut x_display_info;                                   // C 7615

  CHECK_STRING (name);                                                    // C 7617

  dpyinfo = x_display_list;                                               // C 7619
  while !dpyinfo.is_null ()
    {
      if !NILP (Fstring_equal (XCAR ((*dpyinfo).name_list_element), name))   // C 7620
	{ return dpyinfo; }                                               // C 7621
      dpyinfo = (*dpyinfo).next;                                          // C 7619's step
    }

  /* Use this general default value to start with.  */                    // C 7623
  Vx_resource_name = Vinvocation_name;                                    // C 7624

  validate_x_resource_name ();                                            // C 7626

  dpyinfo = x_term_init (name, 0 as *mut c_char,                          // C 7628
			 SSDATA (Vx_resource_name));

  if dpyinfo.is_null ()                                                   // C 7630
    { error (c"Cannot connect to X server %s".as_ptr (), SDATA (name)); }  // C 7631

  return dpyinfo;                                                         // C 7633
}

/* C 7637-7677.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-open-connection", Fx_open_connection, Sx_open_connection,
	1, 3, 0,
	doc: "Open a connection to a display server.\nDISPLAY is the name of the display to connect to.\nOptional second arg XRM-STRING is a string of resources in xrdb format.\nIf the optional third arg MUST-SUCCEED is non-nil,\nterminate Emacs if we can't open the connection.\n(In the Nextstep version, the last two arguments are currently ignored.)  ",
	(display: Lisp_Object, xrm_string: Lisp_Object, must_succeed: Lisp_Object),
	{
  let mut xrm_option: *mut c_char;                                        // C 7647
  let mut dpyinfo: *mut x_display_info;                                   // C 7648

  CHECK_STRING (display);                                                 // C 7650
  if !NILP (xrm_string)                                                   // C 7651
    { CHECK_STRING (xrm_string); }                                        // C 7652

  xrm_option = if NILP (xrm_string) { 0 as *mut c_char }                  // C 7654
	       else { SSDATA (xrm_string) };

  validate_x_resource_name ();                                            // C 7656

  /* This is what opens the connection and sets x_current_display.
     This also initializes many symbols, such as those used for input.  */  // C 7658-7659
  dpyinfo = x_term_init (display, xrm_option,                             // C 7660-7661
			 SSDATA (Vx_resource_name));

  if dpyinfo.is_null ()                                                   // C 7663
    {
      if !NILP (must_succeed)                                             // C 7665
	{ fatal (c"Cannot connect to X server %s.\nCheck the DISPLAY environment variable or use `-d'.\nAlso use the `xauth' program to verify that you have the proper\nauthorization information needed to connect the X server.\nAn insecure way to solve the problem may be to use `xhost'.\n".as_ptr (),   // C 7666-7670
		 SDATA (display)); }                                      // C 7671
      else
	{ error (c"Cannot connect to X server %s".as_ptr (),              // C 7673
		 SDATA (display)); }
    }

  return Qnil ();                                                         // C 7676
});

/* C 7679-7695.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-close-connection", Fx_close_connection, Sx_close_connection,
	1, 1, 0,
	doc: "Close the connection to TERMINAL's X server.\nFor TERMINAL, specify a terminal object, a frame or a display name (a\nstring).  If TERMINAL is nil, that stands for the selected frame's terminal.\n(On MS Windows, this function does not accept terminal objects.)  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);     // C 7687

  if (*dpyinfo).reference_count > 0                                       // C 7689
    { error (c"Display still has frames on it".as_ptr ()); }              // C 7690

  x_delete_terminal ((*dpyinfo).terminal);                                // C 7692

  return Qnil ();                                                         // C 7694
});

/* C 7697-7708.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-list", Fx_display_list, Sx_display_list,
	0, 0, 0,
	doc: "Return the list of display names that Emacs has connections to.  ",
	(),
	{
  let mut result: Lisp_Object = Qnil ();                                  // C 7701
  let mut xdi: *mut x_display_info;                                       // C 7702

  xdi = x_display_list;                                                   // C 7704
  while !xdi.is_null ()
    {
      result = Fcons (XCAR ((*xdi).name_list_element), result);           // C 7705
      xdi = (*xdi).next;
    }

  return result;                                                          // C 7707
});

/* C 7710-7729.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-synchronize", Fx_synchronize, Sx_synchronize,
	1, 2, 0,
	doc: "If ON is non-nil, report X errors as soon as the erring request is made.\nThis function has an effect only on X Windows.  With MS Windows, it is\ndefined but does nothing.\n\nIf ON is nil, allow buffering of requests.\nTurning on synchronization prohibits the Xlib routines from buffering\nrequests and seriously degrades performance, but makes debugging much\neasier.\nThe optional second argument TERMINAL specifies which display to act on.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf TERMINAL is omitted or nil, that stands for the selected frame's display.  ",
	(on: Lisp_Object, terminal: Lisp_Object),
	{
  let dpyinfo: *mut x_display_info = check_x_display_info (terminal);     // C 7724

  XSynchronize ((*dpyinfo).display, !NILP (on) as Bool);                  // C 7726

  return Qnil ();                                                         // C 7728
});

/* -----------------------------------------------------------------------------------------
   C 7731-7950: the `Window properties` section comment (C 7731-7734) and
   `x-change-window-property` - the FORMAT check, the two VALUE arms, the target-window choice,
   C 7783-7791's `USE_XCB` declarations and C 7858-7933's two arms, C 7937-7939's
   `XChangeProperty` and C 7944-7946's error pair.
   -------------------------------------------------------------------------------------- */

/* C: xterm.h 1841/1934/1935: `extern Atom x_intern_cached_atom (struct x_display_info *,
   const char *, bool);` / `extern int x_check_property_data (Lisp_Object);` /
   `extern void x_fill_property_data (struct x_display_info *, Lisp_Object, void *, ptrdiff_t,
   int);` - C 7859/7872/7886, C 7812 and C 7824's calls.  `src/xterm.rs` and `src/xselect.rs`
   define them (`src/xselect.rs` declares the same three rows) and no mirror carries them, so they
   stand at the use site (P-55/P-72 rule 3).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_intern_cached_atom (dpyinfo: *mut x_display_info, name: *const c_char,
			       only_if_exists: bool) -> Atom;
  pub fn x_check_property_data (value: Lisp_Object) -> c_int;
  pub fn x_fill_property_data (dpyinfo: *mut x_display_info, value: Lisp_Object,
			       data: *mut c_void, nelements: ptrdiff_t, format: c_int);
}

/* C: xcb/xproto.h 1851-1856, 1875-1881, 6676-6680 and 6723-6726:
   `typedef struct xcb_intern_atom_cookie_t { unsigned int sequence; } xcb_intern_atom_cookie_t;`,
   `typedef struct xcb_intern_atom_reply_t { uint8_t response_type; uint8_t pad0; uint16_t
   sequence; uint32_t length; xcb_atom_t atom; } xcb_intern_atom_reply_t;`, `xcb_intern_atom_cookie_t
   xcb_intern_atom (xcb_connection_t *, uint8_t, uint16_t, const char *);` and
   `xcb_intern_atom_reply_t *xcb_intern_atom_reply (xcb_connection_t *, xcb_intern_atom_cookie_t,
   xcb_generic_error_t **);` - C 7786-7788 and C 7878/7892/7899/7916's names, the `xcb_*` family
   this file's slice 14 declared the `xproto.h` rows of (262-268).  This machine has
   `<xcb/xproto.h>`, so the records' members are the header's.  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct xcb_intern_atom_cookie_t
{
  pub sequence: c_uint,                    /* C: `unsigned int sequence;`  */
}
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct xcb_intern_atom_reply_t
{
  pub response_type: u8,                   /* C: `uint8_t response_type;`  */
  pub pad0: u8,                            /* C: `uint8_t pad0;`  */
  pub sequence: u16,                       /* C: `uint16_t sequence;`  */
  pub length: u32,                         /* C: `uint32_t length;`  */
  pub atom: xcb_atom_t,                    /* C: `xcb_atom_t atom;` (C 7904/7921 read it)  */
}
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
unsafe extern "C" {
  pub fn xcb_intern_atom (c: *mut xcb_connection_t, only_if_exists: u8, name_len: u16,
			  name: *const c_char) -> xcb_intern_atom_cookie_t;
  pub fn xcb_intern_atom_reply (c: *mut xcb_connection_t, cookie: xcb_intern_atom_cookie_t,
				e: *mut *mut xcb_generic_error_t) -> *mut xcb_intern_atom_reply_t;
}

/* C 7736-7950.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-change-window-property", Fx_change_window_property, Sx_change_window_property,
	2, 7, 0,
	doc: "Change window property PROP to VALUE on the X window of FRAME.\nPROP must be a string.  VALUE may be a string or a list of conses,\nnumbers and/or strings.  If an element in the list is a string, it is\nconverted to an atom and the value of the atom is used.  If an element\nis a cons, it is converted to a 32 bit number where the car is the 16\ntop bits and the cdr is the lower 16 bits.\n\nFRAME nil or omitted means use the selected frame.  If TYPE is given\nand non-nil, it is the name of the type of VALUE.  If TYPE is not\ngiven or nil, the type is STRING.\n\nFORMAT gives the size in bits of each element if VALUE is a list.  It\nmust be one of 8, 16 or 32.\n\nIf VALUE is a string or FORMAT is nil or not given, FORMAT defaults to\n8.  If OUTER-P is non-nil, the property is changed for the outer X\nwindow of FRAME.  Default is to change on the edit X window.\n\nIf WINDOW-ID is non-nil, change the property of that window instead of\nFRAME's X window; the number 0 denotes the root window.  This argument\nis separate from FRAME because window IDs are not unique across X\ndisplays or screens on the same display, so FRAME provides context for\nthe window ID.\n\nIf VALUE is a string and FORMAT is 32, then the format of VALUE is\nsystem-specific.  VALUE must contain unsigned integer data in native\nendian-ness in multiples of the size of the C type 'long': the low 32\nbits of each such number are used as the value of each element of the\nproperty.\n\nWait for the request to complete and signal any error, unless\n`x-fast-protocol-requests' is non-nil, in which case errors will be\nsilently ignored.  ",
	(prop: Lisp_Object, value: Lisp_Object, frame: Lisp_Object, r#type: Lisp_Object,
	 format: Lisp_Object, outer_p: Lisp_Object, window_id: Lisp_Object),
	{
  use crate::x11_constants::PropModeReplace;                             // C 7939

  let f: *mut frame;                                                     // C 7775
  let mut prop_atom: Atom;                                               // C 7776
  let mut target_type: Atom = XA_STRING;                                 // C 7777
  let mut element_format: c_int = 8;                                     // C 7778
  let mut data: *mut c_uchar;                                            // C 7779
  let mut nelements: c_int;                                              // C 7780
  let mut target_window: Window;                                         // C 7781
  let dpyinfo: *mut x_display_info;                                      // C 7782
  #[cfg (feature = "use_xcb")]                                           // C 7783
  let mut intern_prop: bool = false;                                     // C 7784
  #[cfg (feature = "use_xcb")]
  let mut intern_target: bool = false;                                   // C 7785
  #[cfg (feature = "use_xcb")]
  let mut prop_atom_cookie: xcb_intern_atom_cookie_t = std::mem::zeroed ();   // C 7786
  #[cfg (feature = "use_xcb")]
  let mut target_type_cookie: xcb_intern_atom_cookie_t = std::mem::zeroed ();  // C 7787
  #[cfg (feature = "use_xcb")]
  let mut reply: *mut xcb_intern_atom_reply_t;                           // C 7788
  #[cfg (feature = "use_xcb")]
  let mut generic_error: *mut xcb_generic_error_t = std::ptr::null_mut ();   // C 7789
  #[cfg (feature = "use_xcb")]
  let mut rc: bool;                                                      // C 7790

  f = decode_window_system_frame (frame);                                // C 7793
  dpyinfo = FRAME_DISPLAY_INFO! (f);                                     // C 7794

  CHECK_STRING (prop);                                                   // C 7796

  if !NILP (format)                                                      // C 7798
    {
      CHECK_FIXNUM (format);                                             // C 7800

      if XFIXNUM (format) != 8 && XFIXNUM (format) != 16                 // C 7802-7803
	  && XFIXNUM (format) != 32
	{ error (c"FORMAT must be one of 8, 16 or 32".as_ptr ()); }      // C 7804
      element_format = XFIXNUM (format) as c_int;                        // C 7805
    }

  if CONSP (value)                                                       // C 7808
    {
      let mut elsize: ptrdiff_t;                                         // C 7810

      nelements = x_check_property_data (value);                         // C 7812
      if nelements == -1                                                 // C 7813
	{ error (c"Bad data in VALUE, must be number, string or cons".as_ptr ()); }   // C 7814

      /* The man page for XChangeProperty:
	 "If the specified format is 32, the property data must be a
	  long array."
	 This applies even if long is more than 32 bits.  The X library
	 converts to 32 bits before sending to the X server.  */         // C 7816-7820
      elsize = if element_format == 32                                    // C 7821
	{ std::mem::size_of::<c_long> () as ptrdiff_t }
      else { (element_format >> 3) as ptrdiff_t };
      data = xnmalloc (nelements as isize, elsize as isize) as *mut c_uchar;   // C 7822

      x_fill_property_data (FRAME_DISPLAY_INFO! (f), value,              // C 7824-7825
			    data as *mut c_void, nelements as ptrdiff_t, element_format);
    }
  else
    {
      let mut elsize: ptrdiff_t;                                          // C 7829

      CHECK_STRING (value);                                               // C 7831
      data = SDATA (value);                                               // C 7832
      if (INT_MAX as EMACS_INT) < SBYTES (value)                          // C 7833
	{ error (c"VALUE too long".as_ptr ()); }                          // C 7834

      /* See comment above about longs and format=32.  */                 // C 7836
      elsize = if element_format == 32                                    // C 7837
	{ std::mem::size_of::<c_long> () as ptrdiff_t }
      else { (element_format >> 3) as ptrdiff_t };
      if SBYTES (value) % elsize != 0                                     // C 7838
	{ error (c"VALUE must contain an integral number of octets for FORMAT".as_ptr ()); }   // C 7839
      nelements = (SBYTES (value) / elsize) as c_int;                     // C 7840
    }

  if !NILP (window_id)                                                    // C 7843
    {
      CONS_TO_INTEGER! (window_id, target_window);                        // C 7845
      if target_window == 0                                               // C 7846
	{ target_window = (*dpyinfo).root_window; }                       // C 7847
    }
  else
    {
      if !NILP (outer_p)                                                  // C 7851
	{ target_window = FRAME_OUTER_WINDOW! (f); }                      // C 7852
      else
	{ target_window = FRAME_X_WINDOW! (f); }                          // C 7854
    }

  block_input ();                                                         // C 7857
  #[cfg (not (feature = "use_xcb"))]                                      // C 7858
  {
    prop_atom = x_intern_cached_atom (dpyinfo, SSDATA (prop), false);      // C 7859-7860
    if !NILP (r#type)                                                     // C 7861
      {
	CHECK_STRING (r#type);                                            // C 7863
	target_type = x_intern_cached_atom (dpyinfo, SSDATA (r#type), false);   // C 7864-7865
      }
  }
  #[cfg (feature = "use_xcb")]                                            // C 7867
  {
    use crate::x11_constants::None;                                       // C 7875's `None`

    rc = true;                                                            // C 7868
    intern_target = true;                                                 // C 7869
    intern_prop = true;                                                   // C 7870

    prop_atom = x_intern_cached_atom (dpyinfo, SSDATA (prop), true);      // C 7872-7873

    if prop_atom != None as Atom                                          // C 7875
      { intern_prop = false; }                                            // C 7876
    else
      {
	prop_atom_cookie = xcb_intern_atom ((*dpyinfo).xcb_connection,    // C 7878-7880
					    0, SBYTES (prop) as u16, SSDATA (prop));
      }

    if !NILP (r#type)                                                     // C 7882
      {
	CHECK_STRING (r#type);                                            // C 7884

	target_type = x_intern_cached_atom (dpyinfo, SSDATA (r#type), true);   // C 7886-7887

	if target_type != 0                                               // C 7889
	  { intern_target = false; }                                      // C 7890
	else
	  {
	    target_type_cookie = xcb_intern_atom ((*dpyinfo).xcb_connection,   // C 7892-7894
						  0, SBYTES (r#type) as u16, SSDATA (r#type));
	  }
      }

    if intern_prop                                                        // C 7897
      {
	reply = xcb_intern_atom_reply ((*dpyinfo).xcb_connection,         // C 7899-7900
				       prop_atom_cookie, &raw mut generic_error);

	if !reply.is_null ()                                              // C 7902
	  {
	    prop_atom = (*reply).atom as Atom;                            // C 7904
	    free (reply as *mut c_void);                                  // C 7905
	  }
	else
	  {
	    free (generic_error as *mut c_void);                          // C 7909
	    rc = false;                                                   // C 7910
	  }
      }

    if !NILP (r#type) && intern_target                                     // C 7914
      {
	reply = xcb_intern_atom_reply ((*dpyinfo).xcb_connection,         // C 7916-7917
				       target_type_cookie, &raw mut generic_error);

	if !reply.is_null ()                                              // C 7919
	  {
	    target_type = (*reply).atom as Atom;                          // C 7921
	    free (reply as *mut c_void);                                  // C 7922
	  }
	else
	  {
	    free (generic_error as *mut c_void);                          // C 7926
	    rc = false;                                                   // C 7927
	  }
      }

    if !rc                                                               // C 7931
      { error (c"Failed to intern type or property atom".as_ptr ()); }   // C 7932
  }

  x_catch_errors_for_lisp (dpyinfo);                                      // C 7935

  XChangeProperty ((*dpyinfo).display, target_window,                     // C 7937-7939
		   prop_atom, target_type, element_format,
		   PropModeReplace, data, nelements);

  if CONSP (value)                                                        // C 7941
    { xfree (data as *mut c_void); }                                      // C 7942

  x_check_errors_for_lisp (dpyinfo,                                       // C 7944-7945
			   c"Couldn't change window property: %s".as_ptr ());
  x_uncatch_errors_for_lisp (dpyinfo);                                    // C 7946

  unblock_input ();                                                       // C 7948
  return value;                                                           // C 7949
});

/* C 7953-7995.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-delete-window-property", Fx_delete_window_property, Sx_delete_window_property,
	1, 3, 0,
	doc: "Remove window property PROP from X window of FRAME.
FRAME nil or omitted means use the selected frame.
If WINDOW-ID is non-nil, remove property from that window instead
 of FRAME's X window; the number 0 denotes the root window.  This
 argument is separate from FRAME because window IDs are not unique
 across X displays or screens on the same display, so FRAME provides
 context for the window ID.

Value is PROP.

Wait for the request to complete and signal any error, unless
`x-fast-protocol-requests' is non-nil, in which case errors will be
silently ignored.  ",
	(prop: Lisp_Object, frame: Lisp_Object, window_id: Lisp_Object),
	{
  let f: *mut frame = decode_window_system_frame (frame);                 // C 7970
  let mut target_window: Window = FRAME_X_WINDOW! (f);                    // C 7971
  let prop_atom: Atom;                                                    // C 7972

  CHECK_STRING (prop);                                                    // C 7974

  if !NILP (window_id)                                                    // C 7976
    {
      CONS_TO_INTEGER! (window_id, target_window);                        // C 7978
      if target_window == 0                                               // C 7979
	{ target_window = (*FRAME_DISPLAY_INFO! (f)).root_window; }          // C 7980
    }

  block_input ();                                                         // C 7983
  prop_atom = x_intern_cached_atom (FRAME_DISPLAY_INFO! (f),              // C 7984-7985
				    SSDATA (prop), false);

  x_catch_errors_for_lisp (FRAME_DISPLAY_INFO! (f));                      // C 7987
  XDeleteProperty (FRAME_X_DISPLAY! (f), target_window, prop_atom);       // C 7988
  x_check_errors_for_lisp (FRAME_DISPLAY_INFO! (f),                       // C 7989-7990
			   c"Couldn't delete window property: %s".as_ptr ());
  x_uncatch_errors_for_lisp (FRAME_DISPLAY_INFO! (f));                    // C 7991

  unblock_input ();                                                       // C 7993
  return prop;                                                            // C 7994
});

/* C: xterm.h 1937-1941: `extern Lisp_Object x_property_data_to_lisp (struct frame *,
   const unsigned char *, Atom, int, unsigned long);` - C 8062-8066's call.  `src/xselect.rs`
   defines it (C 3074) and no mirror carries the row, so it stands at the use site
   (P-55/P-72 rule 3).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_property_data_to_lisp (f: *mut frame, data: *const c_uchar, actual_type: Atom,
				  actual_format: c_int, actual_size: c_ulong) -> Lisp_Object;
}

/* C 7998-8073.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_window_property_intern (f: *mut frame, target_window: Window,   // C 7998-7999
				    prop_atom: Atom, target_type: Atom,
				    delete_p: Lisp_Object, vector_ret_p: Lisp_Object,
				    found: *mut bool) -> Lisp_Object       // C 8000-8005
{                                                                          // C 8006
  use crate::x11_constants::Success;                                       // C 8021/8032
  let mut tmp_data: *mut c_uchar = std::ptr::null_mut ();                  // C 8007
  let mut prop_value: Lisp_Object = Qnil ();                               // C 8008
  let mut actual_type: Atom = 0;                                           // C 8009
  let mut actual_format: c_int = 0;                                        // C 8010
  let mut actual_size: c_ulong = 0;                                        // C 8011
  let mut bytes_remaining: c_ulong = 0;
  let mut rc: c_int;                                                       // C 8012

  rc = XGetWindowProperty (FRAME_X_DISPLAY! (f), target_window,            // C 8014-8017
			   prop_atom, 0, 0, False, target_type,
			   &raw mut actual_type, &raw mut actual_format, &raw mut actual_size,
			   &raw mut bytes_remaining, &raw mut tmp_data);

  *found = actual_format != 0;                                             // C 8019

  if rc == Success && *found                                               // C 8021
    {                                                                      // C 8022
      XFree (tmp_data as *mut c_void);                                     // C 8023
      tmp_data = std::ptr::null_mut ();                                    // C 8024

      rc = XGetWindowProperty (FRAME_X_DISPLAY! (f), target_window,        // C 8026-8031
			       prop_atom, 0, bytes_remaining as c_long,
			       !NILP (delete_p) as Bool, target_type,
			       &raw mut actual_type, &raw mut actual_format,
			       &raw mut actual_size, &raw mut bytes_remaining,
			       &raw mut tmp_data);
      if rc == Success && ! tmp_data.is_null ()                            // C 8032
	{                                                                      // C 8033
          /* The man page for XGetWindowProperty says:
             "If the returned format is 32, the returned data is represented
             as a long array and should be cast to that type to obtain the
             elements."
             This applies even if long is more than 32 bits, the X library
             converts from 32 bit elements received from the X server to long
             and passes the long array to us.  Thus, for that case memcpy can not
             be used.  We convert to a 32 bit type here, because so much code
             assume on that.

             The bytes and offsets passed to XGetWindowProperty refers to the
             property and those are indeed in 32 bit quantities if format is
             32.  */

          if LONG_WIDTH > 32 && actual_format == 32                       // C 8048
	    {                                                                  // C 8049
	      let mut i: c_ulong;                                              // C 8050
	      let idata: *mut c_int = tmp_data as *mut c_int;                   // C 8051
	      let ldata: *const c_long = tmp_data as *const c_long;              // C 8052

	      i = 0;
	      while i < actual_size                                            // C 8054
		{
		  *idata.offset (i as isize) = *ldata.offset (i as isize) as c_int;   // C 8055
		  i += 1;
		}
	    }

	  if NILP (vector_ret_p)                                               // C 8058
	    { prop_value = make_string (tmp_data as *mut c_char,                // C 8059-8060
					((actual_format >> 3) as c_ulong * actual_size) as ptrdiff_t); }
	  else
	    { prop_value = x_property_data_to_lisp (f,                          // C 8062-8066
						      tmp_data, actual_type, actual_format,
						      actual_size); }
	}                                                                      // C 8067

      if ! tmp_data.is_null () { XFree (tmp_data as *mut c_void); }        // C 8069
    }                                                                      // C 8070

  return prop_value;                                                       // C 8072
}                                                                          // C 8073

/* C 8075-8176.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-window-property", Fx_window_property, Sx_window_property,
	1, 6, 0,
	doc: "Value is the value of window property PROP on FRAME.
If FRAME is nil or omitted, use the selected frame.

On X Windows, the following optional arguments are also accepted: If
TYPE is nil or omitted, get the property as a string.  Otherwise TYPE
is the name of the atom that denotes the expected type.

If TYPE is the string \"AnyPropertyType\", decode and return the data
regardless of what the type really is.

The format of the data returned is the same as a selection conversion
to the given type.  For example, if `x-get-selection-internal' returns
an integer when the selection data is a given type,
`x-window-property' will do the same for that type.

If WINDOW-ID is non-nil, get the property of that window instead of
FRAME's X window; the number 0 denotes the root window.  This argument
is separate from FRAME because window IDs are not unique across X
displays, so FRAME provides context for the window ID.

If DELETE-P is non-nil, delete the property after retrieving it.
If VECTOR-RET-P is non-nil, return a vector of values instead of a string.

X allows an arbitrary number of properties to be set on any window.
However, properties are most often set by the window manager or other
programs on the root window or FRAME's X window in order to
communicate information to Emacs and other programs.  Most of these
properties are specified as part of the Extended Window Manager Hints
and the Inter-Client Communication Conventions Manual, which are
located here:

  https://specifications.freedesktop.org/wm-spec/wm-spec-latest.html

and

  https://x.org/releases/X11R7.6/doc/xorg-docs/specs/ICCCM/icccm.html

Return value is nil if FRAME doesn't have a property with name PROP or
if PROP has no value of TYPE (always a string in the MS Windows case).",
	(prop: Lisp_Object, frame: Lisp_Object, r#type: Lisp_Object,
	 window_id: Lisp_Object, delete_p: Lisp_Object, vector_ret_p: Lisp_Object),
	{
  use crate::x11_constants::AnyPropertyType;                               // C 8141

  let f: *mut frame = decode_window_system_frame (frame);                 // C 8119
  let mut prop_atom: Atom;                                                // C 8120
  let mut prop_value: Lisp_Object = Qnil ();                              // C 8121
  let mut target_type: Atom = XA_STRING;                                  // C 8122
  let mut target_window: Window = FRAME_X_WINDOW! (f);                    // C 8123
  let mut found: bool = false;                                            // C 8124

  CHECK_STRING (prop);                                                    // C 8126

  if !NILP (window_id)                                                    // C 8128
    {
      CONS_TO_INTEGER! (window_id, target_window);                        // C 8130
      if target_window == 0                                               // C 8131
	{ target_window = (*FRAME_DISPLAY_INFO! (f)).root_window; }          // C 8132
    }

  block_input ();                                                         // C 8135
  x_catch_errors (FRAME_X_DISPLAY! (f));                                  // C 8136

  if STRINGP (r#type)                                                     // C 8138
    {
      if strcmp (c"AnyPropertyType".as_ptr (), SSDATA (r#type)) == 0      // C 8140
	{ target_type = AnyPropertyType as Atom; }                         // C 8141
      else
	{ target_type = x_intern_cached_atom (FRAME_DISPLAY_INFO! (f),       // C 8143-8144
					      SSDATA (r#type), false); }
    }

  prop_atom = x_intern_cached_atom (FRAME_DISPLAY_INFO! (f),              // C 8147-8148
				    SSDATA (prop), false);
  prop_value = x_window_property_intern (f,                               // C 8149-8155
					 target_window,
					 prop_atom,
					 target_type,
					 delete_p,
					 vector_ret_p,
					 &raw mut found);
  if NILP (prop_value)                                                    // C 8156
    && ! found                                                            // C 8157
    && NILP (window_id)                                                   // C 8158
    && target_window != FRAME_OUTER_WINDOW! (f)                           // C 8159
    {
      prop_value = x_window_property_intern (f,                           // C 8161-8167
					     FRAME_OUTER_WINDOW! (f),
					     prop_atom,
					     target_type,
					     delete_p,
					     vector_ret_p,
					     &raw mut found);
    }

  x_check_errors (FRAME_X_DISPLAY! (f),                                   // C 8170-8171
		  c"Can't retrieve window property: %s".as_ptr ());
  x_uncatch_errors_after_check ();                                        // C 8172

  unblock_input ();                                                       // C 8174
  return prop_value;                                                      // C 8175
});

/* C 8178-8256.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-window-property-attributes", Fx_window_property_attributes,
	Sx_window_property_attributes, 1, 3, 0,
	doc: "Retrieve metadata about window property PROP on FRAME.
If FRAME is nil or omitted, use the selected frame.
If WINDOW-ID is non-nil, get the property of that window instead of
 FRAME's X window; the number 0 denotes the root window.  This
 argument is separate from FRAME because window IDs are not unique
 across X displays or screens on the same display, so FRAME provides
 context for the window ID.

Return value is nil if FRAME doesn't have a property named PROP.
Otherwise, the return value is a vector with the following fields:

0. The property type, as an integer.  The symbolic name of
 the type can be obtained with `x-get-atom-name'.
1. The format of each element; one of 8, 16, or 32.
2. The length of the property, in number of elements.",
	(prop: Lisp_Object, frame: Lisp_Object, window_id: Lisp_Object),
	{
  use crate::x11_constants::{AnyPropertyType, Success};                   // C 8222/8225/8235/8240

  let f: *mut frame = decode_window_system_frame (frame);                 // C 8197
  let mut target_window: Window = FRAME_X_WINDOW! (f);                    // C 8198
  let mut prop_atom: Atom;                                                // C 8199
  let mut prop_attr: Lisp_Object = Qnil ();                               // C 8200
  let mut actual_type: Atom = 0;                                          // C 8201
  let mut actual_format: c_int = 0;                                       // C 8202
  let mut actual_size: c_ulong = 0;                                       // C 8203
  let mut bytes_remaining: c_ulong = 0;
  let mut tmp_data: *mut c_uchar = std::ptr::null_mut ();                 // C 8204
  let mut rc: c_int;                                                      // C 8205

  CHECK_STRING (prop);                                                    // C 8207

  if !NILP (window_id)                                                    // C 8209
    {
      CONS_TO_INTEGER! (window_id, target_window);                        // C 8211
      if target_window == 0                                               // C 8212
	{ target_window = (*FRAME_DISPLAY_INFO! (f)).root_window; }          // C 8213
    }

  block_input ();                                                         // C 8216

  x_catch_errors (FRAME_X_DISPLAY! (f));                                  // C 8218
  prop_atom = x_intern_cached_atom (FRAME_DISPLAY_INFO! (f),              // C 8219-8220
				    SSDATA (prop), false);
  rc = XGetWindowProperty (FRAME_X_DISPLAY! (f), target_window,           // C 8221-8224
			   prop_atom, 0, 0, False, AnyPropertyType as Atom,          // C 8222/8235
			   &raw mut actual_type, &raw mut actual_format, &raw mut actual_size,
			   &raw mut bytes_remaining, &raw mut tmp_data);
  if rc == Success          /* no invalid params */                       // C 8225
    && actual_format == 0  /* but prop not found */                       // C 8226
    && NILP (window_id)                                                   // C 8227
    && target_window != FRAME_OUTER_WINDOW! (f)                           // C 8228
    {
      /* analogous behavior to x-window-property: if property isn't found
         on the frame's inner window and no alternate window id was
         provided, try the frame's outer window. */
      target_window = FRAME_OUTER_WINDOW! (f);                            // C 8233
      rc = XGetWindowProperty (FRAME_X_DISPLAY! (f), target_window,       // C 8234-8237
			       prop_atom, 0, 0, False, AnyPropertyType as Atom,          // C 8222/8235
			       &raw mut actual_type, &raw mut actual_format, &raw mut actual_size,
			       &raw mut bytes_remaining, &raw mut tmp_data);
    }

  if rc == Success && actual_format != 0                                  // C 8240
    {
      XFree (tmp_data as *mut c_void);                                    // C 8242

      prop_attr = CALLN! (Fvector,                                        // C 8244-8247
			  make_fixnum (actual_type as EMACS_INT),
			  make_fixnum (actual_format as EMACS_INT),
			  make_fixnum ((bytes_remaining / (actual_format >> 3) as c_ulong) as EMACS_INT));
    }

  x_check_errors (FRAME_X_DISPLAY! (f),                                   // C 8250-8251
		  c"Can't retrieve window property: %s".as_ptr ());
  x_uncatch_errors_after_check ();                                        // C 8252

  unblock_input ();                                                       // C 8254
  return prop_attr;                                                       // C 8255
});
/* -----------------------------------------------------------------------------------------
   C 8258-8360: the `Coordinate management` section comment (C 8259-8261; the banners are
   described in these blocks rather than transcribed, the file's standing treatment - see the
   ledger) and `x-translate-coordinates` - the two `CHECK_FIXNUM`/`XFIXNUM` pairs, C 8313's null
   `source_frame`, C 8315-8321's `source_window` pair, C 8323-8332's `require_child` early
   return, C 8334-8340's `src`/`dest` defaults, C 8342-8350's `XTranslateCoordinates` with the
   error pair, and C 8352-8359's `list3` result.
   -------------------------------------------------------------------------------------- */

/* C: xterm.h 1836-1837: `extern Lisp_Object x_handle_translate_coordinates (struct frame *,
   Lisp_Object, int, int);` - C 8328's call.  `src/xterm.rs` defines it (C 14481) and no mirror
   carries the row, so it stands at the use site (P-55/P-72 rule 3).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_handle_translate_coordinates (f: *mut frame, dest_window: Lisp_Object,
					 dest_x: c_int, dest_y: c_int) -> Lisp_Object;
}

/* C 8263-8360.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-translate-coordinates", Fx_translate_coordinates, Sx_translate_coordinates,
	1, 6, 0, doc: "Translate coordinates from FRAME.
Translate the given coordinates SOURCE-X and SOURCE-Y from
SOURCE-WINDOW's coordinate space to that of DEST-WINDOW, on FRAME.

If SOURCE-X and SOURCE-Y are nil, use 0 instead.

FRAME can either be a terminal or a frame.  If nil, it defaults to the
selected frame.  SOURCE-WINDOW must be an X window ID, 0 (which means
to use the root window), or nil, which means to use FRAME's inner
window.  DEST-WINDOW must be another X window ID, or nil (which means
to use the root window).

Return a list of (X Y CHILD) if the given coordinates are on the same
screen, or nil otherwise, where X and Y are the coordinates in
DEST-WINDOW's coordinate space, and CHILD is the window ID of any
mapped child in DEST-WINDOW at those coordinates, or nil if there is
no such window.  If REQUIRE-CHILD is nil, avoid fetching CHILD if it
would result in an avoidable request to the X server, thereby
improving performance when the X connection is over a slow network.
Otherwise, always obtain the mapped child window from the X
server. ",
	(frame: Lisp_Object, source_window: Lisp_Object,
	 dest_window: Lisp_Object, source_x: Lisp_Object,
	 source_y: Lisp_Object, require_child: Lisp_Object),
	{
  use crate::x11_constants::None;                                         // C 8357

  let dpyinfo: *mut x_display_info;                                       // C 8290
  let mut source_frame: *mut frame;                                       // C 8291
  let mut dest_x: c_int = 0;                                              // C 8292
  let mut dest_y: c_int = 0;
  let mut child_return: Window = 0;                                       // C 8293
  let mut src: Window = 0;
  let mut dest: Window = 0;
  let mut rc: Bool;                                                       // C 8294
  let temp_result: Lisp_Object;                                           // C 8295

  dpyinfo = check_x_display_info (frame);                                 // C 8297
  dest_x = 0;                                                             // C 8298
  dest_y = 0;                                                             // C 8299

  if !NILP (source_x)                                                     // C 8301
    {
      CHECK_FIXNUM (source_x);                                            // C 8303
      dest_x = XFIXNUM (source_x) as c_int;                               // C 8304
    }

  if !NILP (source_y)                                                     // C 8307
    {
      CHECK_FIXNUM (source_y);                                            // C 8309
      dest_y = XFIXNUM (source_y) as c_int;                               // C 8310
    }

  source_frame = std::ptr::null_mut ();                                   // C 8313

  if !NILP (source_window)                                                // C 8315
    { CONS_TO_INTEGER! (source_window, src); }                            // C 8316
  else
    {
      source_frame = decode_window_system_frame (frame);                  // C 8319
      src = FRAME_X_WINDOW! (source_frame);                               // C 8320
    }

  /* If require_child is nil, try to avoid an avoidable roundtrip to
     the X server.  */
  if NILP (require_child) && ! source_frame.is_null ()                    // C 8325
    {
      temp_result                                                         // C 8327-8329
	= x_handle_translate_coordinates (source_frame, dest_window, dest_x,
					  dest_y);
      if !NILP (temp_result)                                              // C 8330
	{ return temp_result; }                                             // C 8331
    }

  if src == 0                                                             // C 8334
    { src = (*dpyinfo).root_window; }                                     // C 8335

  if !NILP (dest_window)                                                  // C 8337
    { CONS_TO_INTEGER! (dest_window, dest); }                             // C 8338
  else
    { dest = (*dpyinfo).root_window; }                                    // C 8340

  block_input ();                                                         // C 8342
  x_catch_errors ((*dpyinfo).display);                                    // C 8343
  rc = XTranslateCoordinates ((*dpyinfo).display, src, dest,              // C 8344-8346
			      dest_x, dest_y, &raw mut dest_x, &raw mut dest_y,
			      &raw mut child_return);
  x_check_errors ((*dpyinfo).display,                                     // C 8347-8348
		  c"Couldn't translate coordinates: %s".as_ptr ());
  x_uncatch_errors_after_check ();                                        // C 8349
  unblock_input ();                                                       // C 8350

  if rc == 0                                                              // C 8352
    { return Qnil (); }                                                   // C 8353

  return list3 (make_int (dest_x as intmax_t),                            // C 8355-8359
		make_int (dest_y as intmax_t),
		(if child_return != None as Window
		   { make_uint (child_return as uintmax_t) }
		 else { Qnil () }));
});

/* -----------------------------------------------------------------------------------------
   C 8362-8404: the `Tool tips` section banner (C 8362-8364; banners are described in these
   blocks rather than transcribed - see the slice 32 note), C 8366-8367's forward declaration of
   `compute_tip_xy` (Rust needs none: its definition later in this file is visible to callers),
   the eight objects of C 8369-8390 and `unwind_create_tip_frame` (C 8393-8404).
   -------------------------------------------------------------------------------------- */

/* C 8369-8390.  The four externally-linked ones carry `#[unsafe(no_mangle)]` (PORTING.md 0.2;
   xterm.h 1010-1013 declares `tip_window`, `tip_dx`, `tip_dy` and `tip_frame`), the four
   `static` ones stay private, and each tentative definition's zero value is C's (`std::ptr::null_mut`
   for a `Lisp_Object`, `0` for the `Window`) - the `src/haikufns.rs` 556-567 shape.  */
/// C 8369-8370: `/* The frame of the currently visible tooltip, or nil if none.  */` /
/// `Lisp_Object tip_frame;`
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut tip_frame: Lisp_Object = std::ptr::null_mut ();                 // C 8370 (tentative)
/// C 8372-8374: `/* The window-system window corresponding to the frame of the
///    currently visible tooltip.  */` / `Window tip_window;`
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut tip_window: Window = 0;                                         // C 8374 (tentative)
/// C 8376-8377: `/* The X and Y deltas of the last call to `x-show-tip'.  */` /
/// `Lisp_Object tip_dx, tip_dy;`
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut tip_dx: Lisp_Object = std::ptr::null_mut ();                    // C 8377 (tentative)
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut tip_dy: Lisp_Object = std::ptr::null_mut ();
/// C 8379-8381: `/* A timer that hides or deletes the currently visible tooltip when it
///    fires.  */` / `static Lisp_Object tip_timer;`
#[cfg (feature = "have_x_windows")]
static mut tip_timer: Lisp_Object = std::ptr::null_mut ();                     // C 8381 (tentative)
/// C 8383-8384: `/* STRING argument of last `x-show-tip' call.  */` /
/// `static Lisp_Object tip_last_string;`
#[cfg (feature = "have_x_windows")]
static mut tip_last_string: Lisp_Object = std::ptr::null_mut ();               // C 8384 (tentative)
/// C 8386-8387: `/* Normalized FRAME argument of last `x-show-tip' call.  */` /
/// `static Lisp_Object tip_last_frame;`
#[cfg (feature = "have_x_windows")]
static mut tip_last_frame: Lisp_Object = std::ptr::null_mut ();                // C 8387 (tentative)
/// C 8389-8390: `/* PARMS argument of last `x-show-tip' call.  */` /
/// `static Lisp_Object tip_last_parms;`
#[cfg (feature = "have_x_windows")]
static mut tip_last_parms: Lisp_Object = std::ptr::null_mut ();                // C 8390 (tentative)

/* C 8393-8404.  The handler C's `record_unwind_protect` takes (C 8442), so it is
   `unsafe extern "C" fn` - the `do_unwind_create_frame` shape at C 4816-4820.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn unwind_create_tip_frame (frame: Lisp_Object)             // C 8393-8394
{                                                                              // C 8395
  use crate::x11_constants::None;                                              // C 8401
  let deleted: Lisp_Object;                                                    // C 8396

  deleted = unwind_create_frame (frame);                                       // C 8398
  if EQ (deleted, Qt ())                                                       // C 8399
    {                                                                          // C 8400
      tip_window = None as Window;                                             // C 8401
      tip_frame = Qnil ();                                                      // C 8402
    }
}                                                                              // C 8404

/* -----------------------------------------------------------------------------------------
   C 8406-8775: C 8407-8414's comment and `x_create_tip_frame` - the terminal check, the name
   lookup, the frame's creation with its unwind-protect, the X output setup, the color block,
   the name/`explicit_name` pair, the font drivers, the `gui_default_parameter` runs, the
   `USE_XCB`/`not USE_XCB` window-creation pair, `init_frame_faces`, the `tooltip`/`display-type`
   parameters, the face-after-default pair and the tail.
   -------------------------------------------------------------------------------------- */

/* C 8440/8463/8751/8762 write four frame bitfields; this unit carries each macro (the
   `src/pgtkfns.rs` 267-300 device, `include/frame.rs` carries the `_SHIFT`/`_UNIT` rows).  */
macro_rules! wants_modeline {
  ($f:expr) => { frame_flag ($f, FRAME_WANTS_MODELINE_UNIT, FRAME_WANTS_MODELINE_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_WANTS_MODELINE_UNIT, FRAME_WANTS_MODELINE_SHIFT, ($v) as u32) };
}
macro_rules! tooltip {
  ($f:expr) => { frame_flag ($f, FRAME_TOOLTIP_UNIT, FRAME_TOOLTIP_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TOOLTIP_UNIT, FRAME_TOOLTIP_SHIFT, ($v) as u32) };
}
macro_rules! no_split {
  ($f:expr) => { frame_flag ($f, FRAME_NO_SPLIT_UNIT, FRAME_NO_SPLIT_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_SPLIT_UNIT, FRAME_NO_SPLIT_SHIFT, ($v) as u32) };
}
macro_rules! can_set_window_size {
  ($f:expr) => { frame_flag ($f, FRAME_CAN_SET_WINDOW_SIZE_UNIT, FRAME_CAN_SET_WINDOW_SIZE_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_CAN_SET_WINDOW_SIZE_UNIT, FRAME_CAN_SET_WINDOW_SIZE_SHIFT,
		    ($v) as u32) };
}

/* The names this slice needs that no mirror carries: the `use_xcb` arm's xcb entry points,
   `xcb_cursor_t` and the enum constants (`xcb_colormap_t`/`xcb_visualid_t` are this file's own
   rows at 266-268), and `CWSaveUnder` for the plain arm (C 8592).  Each
   row keeps C's prototype with its header and line (xcb 1.17.0 under `/opt/homebrew/include`).  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub type xcb_cursor_t = u32;                /// C: xcb/xproto.h 60: `typedef uint32_t xcb_cursor_t;`
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: u32 = 131072;   /* C: xcb/xproto.h 360  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: u8 = 1;           /* C: xcb/xproto.h 1350  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CW_BACK_PIXEL: u32 = 2;        /* C: xcb/xproto.h 1369  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CW_BORDER_PIXEL: u32 = 8;      /* C: xcb/xproto.h 1383  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CW_OVERRIDE_REDIRECT: u32 = 512;   /* C: xcb/xproto.h 1420  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CW_EVENT_MASK: u32 = 2048;     /* C: xcb/xproto.h 1429  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CW_COLORMAP: u32 = 8192;       /* C: xcb/xproto.h 1437  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CW_CURSOR: u32 = 16384;        /* C: xcb/xproto.h 1448  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_PROP_MODE_REPLACE: u8 = 0;     /* C: xcb/xproto.h 1916  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
unsafe extern "C" {
  /* C: xcb/xcb.h 603: `uint32_t xcb_generate_id (xcb_connection_t *c);`  */
  pub fn xcb_generate_id (c: *mut xcb_connection_t) -> u32;
  /* C: xcb/xproto.h 5774-5787: `xcb_void_cookie_t xcb_create_window (xcb_connection_t *c,
     uint8_t depth, xcb_window_t wid, xcb_window_t parent, int16_t x, int16_t y, uint16_t width,
     uint16_t height, uint16_t border_width, uint16_t _class, xcb_visualid_t visual,
     uint32_t value_mask, const void *value_list);`  */
  pub fn xcb_create_window (c: *mut xcb_connection_t, depth: u8, wid: xcb_window_t,
			    parent: xcb_window_t, x: i16, y: i16, width: u16, height: u16,
			    border_width: u16, class: u16, visual: xcb_visualid_t,
			    value_mask: u32, value_list: *const c_void) -> c_int;
  /* C: xcb/xproto.h 6847-6854: `xcb_void_cookie_t xcb_change_property (xcb_connection_t *c,
     uint8_t mode, xcb_window_t window, xcb_atom_t property, xcb_atom_t type, uint8_t format,
     uint32_t data_len, const void *data);`  */
  pub fn xcb_change_property (c: *mut xcb_connection_t, mode: u8, window: xcb_window_t,
			      property: xcb_atom_t, r#type: xcb_atom_t, format: u8,
			      data_len: u32, data: *const c_void) -> c_int;
}
/* C: X11/X.h 405: `#define CWSaveUnder (1L<<13)` (C 8592).  */
#[cfg (feature = "have_x_windows")]
pub const CWSaveUnder: c_ulong = 1 << 13;


/* Create a frame for a tooltip on the display described by DPYINFO.
   PARMS is a list of frame parameters.  TEXT is the string to
   display in the tip frame.  Value is the frame.

   Note that functions called here, esp. gui_default_parameter can
   signal errors, for instance when a specified color name is
   undefined.  We have to make sure that we're in a consistent state
   when this happens.  */

/* C 8416-8775.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_create_tip_frame (dpyinfo: *mut x_display_info,                // C 8416-8417
				mut parms: Lisp_Object) -> Lisp_Object
{                                                                         // C 8418
  use crate::dispextern_unit_names::face_change;                            // C 8423/8771
  /* C 8589-8606's names: `CWOverrideRedirect`, `CWEventMask`, `CWCursor` and `StructureNotifyMask`
     are `include/xlib.rs`'s `x11_constants`; `CWBackPixel`, `CWBorderPixel`, `CWColormap`,
     `InputOutput` and `True` are this file's own block at 6055-6090.  */
  use crate::x11_constants::{CWCursor, CWEventMask, CWOverrideRedirect, PropModeReplace,
			      StructureNotifyMask, XA_ATOM};                          // C 8619-8621

  let f: *mut frame;                                                       // C 8419
  let mut frame: Lisp_Object;                                              // C 8420
  let mut name: Lisp_Object;                                               // C 8421
  let count: specpdl_ref = SPECPDL_INDEX ();                               // C 8422
  let face_change_before: bool = face_change;                              // C 8423

  if (*(*dpyinfo).terminal).name.is_null ()                                // C 8425
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }   // C 8426

  parms = Fcopy_alist (parms);                                             // C 8428

  /* Get the name of the frame to use for resource lookup.  */
  name = gui_display_get_arg (dpyinfo, parms, Qname (), c"name".as_ptr (),  // C 8431-8432
			      c"Name".as_ptr (), RES_TYPE_STRING);
  if !STRINGP (name)                                                       // C 8433
    && !BASE_EQ (name, Qunbound ())                                        // C 8434
    && !NILP (name)                                                        // C 8435
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }       // C 8436

  frame = Qnil ();                                                         // C 8438
  f = make_frame (false);                                                  // C 8439
  wants_modeline! (f, false);                                              // C 8440
  XSETFRAME! (frame, f);                                                   // C 8441
  record_unwind_protect (Some (unwind_create_tip_frame), frame);           // C 8442

  (*f).terminal = (*dpyinfo).terminal;                                     // C 8444

  /* By setting the output method, we're essentially saying that
     the frame is live, as per FRAME_LIVE_P.  If we get a signal
     from this point on, x_destroy_window might screw up reference
     counts etc.  */
  output_method! (f, output_x_window);                                     // C 8450
  (*f).output_data.x = xzalloc (std::mem::size_of::<x_output> ());         // C 8451
  (*((*f).output_data.x as *mut x_output)).icon_bitmap = -1;               // C 8452
  /* C 8453: `FRAME_FONTSET (f) = -1;` (`frame_fontset`), the C 5049 case: C's macro is an l-value
     and the mirror carries the read arm only, so the write is C's member path.  */
  (*((*f).output_data.x as *mut x_output)).fontset = -1;                   // C 8453 (C: `c_int`)
  (*((*f).output_data.x as *mut x_output)).scroll_bar_foreground_pixel = (-1 as c_long) as c_ulong;  // C 8454
  (*((*f).output_data.x as *mut x_output)).scroll_bar_background_pixel = (-1 as c_long) as c_ulong;  // C 8455
#[cfg (all (feature = "use_lucid", feature = "use_toolkit_scroll_bars"))]  // C 8456
  {
    (*((*f).output_data.x as *mut x_output)).scroll_bar_top_shadow_pixel = (-1 as c_long) as c_ulong;     // C 8457
    (*((*f).output_data.x as *mut x_output)).scroll_bar_bottom_shadow_pixel = (-1 as c_long) as c_ulong;  // C 8458
  }                                                                        // C 8459
  (*((*f).output_data.x as *mut x_output)).white_relief.pixel = (-1 as c_long) as c_ulong;   // C 8460
  (*((*f).output_data.x as *mut x_output)).black_relief.pixel = (-1 as c_long) as c_ulong;   // C 8461

  tooltip! (f, true);                                                      // C 8463
  fset_icon_name (f, Qnil ());                                             // C 8464
  /* C 8465: `FRAME_DISPLAY_INFO (f) = dpyinfo;` - the same l-value case as C 5049.  */
  (*((*f).output_data.x as *mut x_output)).display_info = dpyinfo;          // C 8465
  (*((*f).output_data.x as *mut x_output)).parent_desc = (*FRAME_DISPLAY_INFO! (f)).root_window;   // C 8466
  (*((*f).output_data.x as *mut x_output)).explicit_parent = false;         // C 8467

  /* These colors will be set anyway later, but it's important
     to get the color reference counts right, so initialize them!  */
  /* C 8477/8478/8485/8487 write `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL` (`f->`
     fields; the mirror's macros read only), so the writes are C's member paths.  */
  {                                                                        // C 8471
    let black: Lisp_Object;                                                // C 8472

    /* Function x_decode_color can signal an error.  Make
       sure to initialize color slots so that we won't try
       to free colors we haven't allocated.  */
    (*f).foreground_pixel = (-1 as c_long) as c_ulong;              // C 8477
    (*f).background_pixel = (-1 as c_long) as c_ulong;              // C 8478
    (*((*f).output_data.x as *mut x_output)).cursor_pixel = (-1 as c_long) as c_ulong;      // C 8479
    (*((*f).output_data.x as *mut x_output)).cursor_foreground_pixel = (-1 as c_long) as c_ulong;  // C 8480
    (*((*f).output_data.x as *mut x_output)).border_pixel = (-1 as c_long) as c_ulong;      // C 8481
    (*((*f).output_data.x as *mut x_output)).mouse_pixel = (-1 as c_long) as c_ulong;       // C 8482

    black = build_string (c"black".as_ptr ());                             // C 8484
    (*f).foreground_pixel = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 8485-8486
    (*f).background_pixel = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 8487-8488
    (*((*f).output_data.x as *mut x_output)).cursor_pixel = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 8489-8490
    (*((*f).output_data.x as *mut x_output)).cursor_foreground_pixel = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 8491-8492
    (*((*f).output_data.x as *mut x_output)).border_pixel = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;   // C 8493-8494
    (*((*f).output_data.x as *mut x_output)).mouse_pixel = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f) as c_int) as c_ulong;    // C 8495-8496
  }                                                                        // C 8497

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if BASE_EQ (name, Qunbound ()) || NILP (name)                            // C 8501
    {
      fset_name (f, build_string ((*dpyinfo).x_id_name));                  // C 8503
      explicit_name! (f, false);                                           // C 8504
    }
  else
    {
      fset_name (f, name);                                                 // C 8508
      explicit_name! (f, true);                                            // C 8509
      /* use the frame's title when getting resources for this frame.  */  // C 8510
      specbind (Qx_resource_name (), name);                                // C 8511
    }

#[cfg (feature = "use_cairo")]                                             // C 8514
  {
    register_font_driver (&raw mut ftcrfont_driver as *const c_void, f);   // C 8515
#[cfg (feature = "harfbuzz")]
    { register_font_driver (&raw mut ftcrhbfont_driver as *const c_void, f); }  // C 8517
  }
#[cfg (not (feature = "use_cairo"))]
  {
#[cfg (feature = "freetype")]
    {
#[cfg (feature = "have_xft")]
      {
	register_font_driver (&raw mut xftfont_driver as *const c_void, f);   // C 8522
#[cfg (feature = "harfbuzz")]
	{ register_font_driver (&raw mut xfthbfont_driver as *const c_void, f); }  // C 8524
      }
    }
  }
  register_font_driver (&raw mut xfont_driver as *const c_void, f);        // C 8529

#[cfg (feature = "glyph_debug")]
  { dpyinfo_refcount = (*dpyinfo).reference_count; }                       // C 8532

  gui_default_parameter (f, parms, Qfont_backend (), Qnil (),              // C 8535-8536
			 c"fontBackend".as_ptr (), c"FontBackend".as_ptr (),
			 RES_TYPE_STRING);

  /* Extract the window parameters from the supplied values that are
     needed to determine window geometry.  */
  x_default_font_parameter (f, parms);                                     // C 8540

  gui_default_parameter (f, parms, Qborder_width (), make_fixnum (0),      // C 8542-8543
			 c"borderWidth".as_ptr (), c"BorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);

  /* This defaults to 1 in order to match xterm.  We recognize either
     internalBorderWidth or internalBorder (which is what xterm calls
     it).  */
  if NILP (Fassq (Qinternal_border_width (), parms))                       // C 8548
    {
      let value: Lisp_Object;                                              // C 8550

      value = gui_display_get_arg (dpyinfo, parms, Qinternal_border_width (),   // C 8552-8554
				   c"internalBorder".as_ptr (), c"internalBorder".as_ptr (),
				   RES_TYPE_NUMBER);
      if !BASE_EQ (value, Qunbound ())                                     // C 8555
	{ parms = Fcons (Fcons (Qinternal_border_width (), value),           // C 8556-8557
			 parms); }
    }

  gui_default_parameter (f, parms, Qinternal_border_width (), make_fixnum (1),   // C 8560-8562
			 c"internalBorderWidth".as_ptr (), c"internalBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qright_divider_width (), make_fixnum (0),   // C 8563-8564
			 std::ptr::null_mut () as *const c_char,
			 std::ptr::null_mut () as *const c_char, RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qbottom_divider_width (), make_fixnum (0),  // C 8565-8566
			 std::ptr::null_mut () as *const c_char,
			 std::ptr::null_mut () as *const c_char, RES_TYPE_NUMBER);

  /* Also do the stuff which must be set before the window exists.  */
  gui_default_parameter (f, parms, Qforeground_color (), build_string (c"black".as_ptr ()),  // C 8569-8570
			 c"foreground".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qbackground_color (), build_string (c"white".as_ptr ()),  // C 8571-8572
			 c"background".as_ptr (), c"Background".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qmouse_color (), build_string (c"black".as_ptr ()),       // C 8573-8574
			 c"pointerColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qcursor_color (), build_string (c"black".as_ptr ()),      // C 8575-8576
			 c"cursorColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qborder_color (), build_string (c"black".as_ptr ()),      // C 8577-8578
			 c"borderColor".as_ptr (), c"BorderColor".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qno_special_glyphs (), Qnil (),         // C 8579-8580
			 std::ptr::null_mut () as *const c_char,
			 std::ptr::null_mut () as *const c_char, RES_TYPE_BOOLEAN);

  {                                                                        // C 8582
#[cfg (not (feature = "use_xcb"))]                                         // C 8583
    {
      let mut attrs: XSetWindowAttributes = std::mem::zeroed ();           // C 8584
      let mut mask: c_ulong;                                               // C 8585
      let r#type: Atom = (*FRAME_DISPLAY_INFO! (f)).Xatom_net_window_type_tooltip;   // C 8586

      block_input ();                                                      // C 8588
      mask = (CWBackPixel | CWOverrideRedirect | CWEventMask               // C 8589-8590
	      | CWCursor | CWColormap | CWBorderPixel) as c_ulong;
      if DoesSaveUnders! ((*dpyinfo).screen) == True                       // C 8591
	{ mask |= CWSaveUnder; }                                            // C 8592

    /* Window managers look at the override-redirect flag to determine
       whether or net to give windows a decoration (Xlib spec, chapter
       3.2.8).  */
      attrs.override_redirect = True;                                      // C 8597
      attrs.save_under = True;                                             // C 8598
      attrs.background_pixel = (*f).background_pixel;                      // C 8599
      attrs.colormap = FRAME_X_COLORMAP! (f);                              // C 8600
      /* C 8601-8603's chain is two stores in C's order.  */
      (*((*f).output_data.x as *mut x_output)).current_cursor               // C 8602-8603
	= (*((*f).output_data.x as *mut x_output)).text_cursor;
      attrs.cursor = (*((*f).output_data.x as *mut x_output)).current_cursor;   // C 8601
      attrs.border_pixel = (*((*f).output_data.x as *mut x_output)).border_pixel;   // C 8604
      /* Arrange for getting MapNotify and UnmapNotify events.  */         // C 8605
      attrs.event_mask = StructureNotifyMask as c_long;                    // C 8606
      FRAME_X_WINDOW! (f)                                                  // C 8607-8613
	= XCreateWindow (FRAME_X_DISPLAY! (f), (*FRAME_DISPLAY_INFO! (f)).root_window,
			 0, 0, 1, 1, (*f).border_width as c_uint,
			 (*dpyinfo).n_planes, InputOutput, FRAME_X_VISUAL! (f),
			 mask, &raw mut attrs);
      tip_window = FRAME_X_WINDOW! (f);                                    // C 8607 (`tip_window =`)
      initial_set_up_x_back_buffer (f);                                    // C 8618
      XChangeProperty (FRAME_X_DISPLAY! (f), tip_window,                   // C 8619-8622
		       (*FRAME_DISPLAY_INFO! (f)).Xatom_net_window_type,
		       XA_ATOM, 32, PropModeReplace,
		       &raw mut r#type as *mut c_uchar, 1);
      unblock_input ();                                                    // C 8623
    }
#[cfg (feature = "use_xcb")]                                               // C 8624
    {
      let mut value_list: [u32; 6] = [0; 6];                               // C 8625
      let net_wm_window_type_tooltip: xcb_atom_t =                         // C 8626-8627
	(*dpyinfo).Xatom_net_window_type_tooltip as xcb_atom_t;
      let visual_id: xcb_visualid_t =                                      // C 8628-8629
	XVisualIDFromVisual (FRAME_X_VISUAL! (f)) as xcb_visualid_t;

      (*((*f).output_data.x as *mut x_output)).current_cursor              // C 8631
	= (*((*f).output_data.x as *mut x_output)).text_cursor;
    /* Values are set in the order of their enumeration in `enum
       xcb_cw_t'.  */
      value_list[0] = (*f).background_pixel as u32;                        // C 8634
      value_list[1] = (*((*f).output_data.x as *mut x_output)).border_pixel as u32;   // C 8635
      value_list[2] = true as u32;                                         // C 8636
      value_list[3] = XCB_EVENT_MASK_STRUCTURE_NOTIFY;                     // C 8637
      value_list[4] = FRAME_X_COLORMAP! (f) as xcb_colormap_t;             // C 8638
      value_list[5] = (*((*f).output_data.x as *mut x_output)).text_cursor as xcb_cursor_t;   // C 8639

      block_input ();                                                      // C 8641
      FRAME_X_WINDOW! (f)                                                  // C 8642-8644
	= xcb_generate_id ((*dpyinfo).xcb_connection) as Window;
      tip_window = FRAME_X_WINDOW! (f);                                    // C 8642 (`tip_window =`)

      xcb_create_window ((*dpyinfo).xcb_connection,                        // C 8646-8659
			 (*dpyinfo).n_planes as u8,
			 tip_window as xcb_window_t,
			 (*dpyinfo).root_window as xcb_window_t,
			 0, 0, 1, 1, (*f).border_width as u16,
			 XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
			 visual_id,
			 (XCB_CW_BACK_PIXEL
			  | XCB_CW_BORDER_PIXEL
			  | XCB_CW_OVERRIDE_REDIRECT
			  | XCB_CW_EVENT_MASK
			  | XCB_CW_COLORMAP
			  | XCB_CW_CURSOR),
			 &raw const value_list as *const c_void);

      xcb_change_property ((*dpyinfo).xcb_connection,                      // C 8661-8666
			   XCB_PROP_MODE_REPLACE,
			   tip_window as xcb_window_t,
			   (*dpyinfo).Xatom_net_window_type as xcb_atom_t,
			   (*dpyinfo).Xatom_ATOM as xcb_atom_t,
			   32, 1, &raw const net_wm_window_type_tooltip as *const c_void);

      initial_set_up_x_back_buffer (f);                                    // C 8668
      unblock_input ();                                                    // C 8669
    }
  }                                                                        // C 8671

  /* Init faces before gui_default_parameter is called for the
     scroll-bar-width parameter because otherwise we end up in
     init_iterator with a null face cache, which should not happen.  */
  init_frame_faces (f);                                                    // C 8676

  gui_default_parameter (f, parms, Qinhibit_double_buffering (), Qnil (),  // C 8678-8680
			 c"inhibitDoubleBuffering".as_ptr (),
			 c"InhibitDoubleBuffering".as_ptr (), RES_TYPE_BOOLEAN);

  gui_figure_window_size (f, parms, false, false);                         // C 8682

  (*((*f).output_data.x as *mut x_output)).parent_desc = (*FRAME_DISPLAY_INFO! (f)).root_window;   // C 8684

  x_make_gc (f);                                                           // C 8686

  gui_default_parameter (f, parms, Qauto_raise (), Qnil (),                // C 8688-8689
			 c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qauto_lower (), Qnil (),                // C 8690-8691
			 c"autoLower".as_ptr (), c"AutoRaiseLower".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qcursor_type (), Qbox (),               // C 8692-8693
			 c"cursorType".as_ptr (), c"CursorType".as_ptr (), RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qalpha (), Qnil (),                     // C 8694-8695
			 c"alpha".as_ptr (), c"Alpha".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qalpha_background (), Qnil (),          // C 8696-8697
			 c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qborders_respect_alpha_background (), Qnil (),   // C 8698-8700
			 c"bordersRespectAlphaBackground".as_ptr (),
			 c"BordersRespectAlphaBackground".as_ptr (), RES_TYPE_NUMBER);

  /* Add `tooltip' frame parameter's default value. */                    // C 8702
  if NILP (Fframe_parameter (frame, Qtooltip ()))                          // C 8703
    {
      AUTO_FRAME_ARG! (arg, Qtooltip (), Qt ());                           // C 8705
      Fmodify_frame_parameters (frame, arg);                               // C 8706
    }

  /* FIXME - can this be done in a similar way to normal frames?
     https://lists.gnu.org/r/emacs-devel/2007-10/msg00641.html */

  /* Set the `display-type' frame parameter before setting up faces. */
  {                                                                        // C 8713
    let mut disptype: Lisp_Object;                                         // C 8714

    /* C 8718-8719 read `FRAME_X_VISUAL_INFO (f)->class`; `FRAME_X_VISUAL_INFO` is xterm.h
       1301's `FRAME_DISPLAY_INFO (f)->visual_info`, which this file reads directly (8060/8150).  */
    if (*FRAME_DISPLAY_INFO! (f)).n_planes == 1                            // C 8716
      { disptype = Qmono (); }                                             // C 8717
    else if (*FRAME_DISPLAY_INFO! (f)).visual_info.class == GrayScale      // C 8718
             || (*FRAME_DISPLAY_INFO! (f)).visual_info.class == StaticGray  // C 8719
      { disptype = Qgrayscale (); }                                        // C 8720
    else
      { disptype = Qcolor (); }                                            // C 8722

    if NILP (Fframe_parameter (frame, Qdisplay_type ()))                   // C 8724
      {
	AUTO_FRAME_ARG! (arg, Qdisplay_type (), disptype);                   // C 8726
	Fmodify_frame_parameters (frame, arg);                               // C 8727
      }
  }                                                                        // C 8729

  /* Set up faces after all frame parameters are known.  This call
     also merges in face attributes specified for new frames.

     Frame parameters may be changed if .Xdefaults contains
     specifications for the default font.  For example, if there is an
     `Emacs.default.attributeBackground: pink', the `background-color'
     attribute of the frame gets set, which let's the internal border
     of the tooltip frame appear in pink.  Prevent this.  */
  {                                                                        // C 8739
    let bg: Lisp_Object = Fframe_parameter (frame, Qbackground_color ());  // C 8740

    calln! (Qface_set_after_frame_default (), frame, Qnil ());             // C 8742

    if !EQ (bg, Fframe_parameter (frame, Qbackground_color ()))            // C 8744
      {
	AUTO_FRAME_ARG! (arg, Qbackground_color (), bg);                     // C 8746
	Fmodify_frame_parameters (frame, arg);                               // C 8747
      }
  }                                                                        // C 8749

  no_split! (f, true);                                                     // C 8751

  /* Now that the frame will be official, it counts as a reference to
     its display and terminal.  */
  (*FRAME_DISPLAY_INFO! (f)).reference_count += 1;                         // C 8755
  (*(*f).terminal).reference_count += 1;                                   // C 8756

  /* It is now ok to make the frame official even if we get an error
     below.  And the frame needs to be on Vframe_list or making it
     visible won't work.  */
  Vframe_list = Fcons (frame, Vframe_list);                                // C 8761
  can_set_window_size! (f, true);                                          // C 8762
  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),       // C 8763-8764
		     0, true, Qtip_frame ());

  /* Setting attributes of faces of the tooltip frame from resources
     and similar will set face_change, which leads to the clearing of
     all current matrices.  Since this isn't necessary here, avoid it
     by resetting face_change to the value it had before we created
     the tip frame.  */
  face_change = face_change_before;                                        // C 8771

  /* Discard the unwind_protect.  */                                       // C 8773
  return unbind_to (count, frame);                                         // C 8774
}                                                                          // C 8775

/* Compute where to display tip frame F.  PARMS is the list of frame
   parameters for F.  DX and DY are specified offsets from the current
   location of the mouse.  WIDTH and HEIGHT are the width and height
   of the tooltip.  Return coordinates relative to the root window of
   the display in *ROOT_X, and *ROOT_Y.  */

/* C 8784-8893.  */
#[cfg (feature = "have_x_windows")]
/* C's out-parameters and the geometry locals take the file's zero initialisers (`root`,
   `child`, `win_x`, `win_y`, `pmask`, `min_x`, `min_y`, `max_x`; C 8790-8793).  */
#[cfg (feature = "have_x_windows")]
unsafe fn compute_tip_xy (f: *mut frame, parms: Lisp_Object, dx: Lisp_Object,   // C 8784-8787
			  dy: Lisp_Object, width: c_int, height: c_int, root_x: *mut c_int,
			  root_y: *mut c_int)
{                                                                              // C 8788
  let mut left: Lisp_Object;                                                   // C 8789
  let mut top: Lisp_Object;
  let mut right: Lisp_Object;
  let mut bottom: Lisp_Object;
  let mut win_x: c_int = 0;                                                    // C 8790
  let mut win_y: c_int = 0;
  let mut root: Window = 0;                                                    // C 8791
  let mut child: Window = 0;
  let mut pmask: c_uint = 0;                                                   // C 8792
  let mut min_x: c_int = 0;                                                    // C 8793
  let mut min_y: c_int = 0;
  let mut max_x: c_int = 0;
  let mut max_y: c_int = -1;

  /* User-specified position?  */                                              // C 8795
  left = CDR (Fassq (Qleft (), parms));                                        // C 8796
  top  = CDR (Fassq (Qtop (), parms));                                         // C 8797
  right = CDR (Fassq (Qright (), parms));                                      // C 8798
  bottom = CDR (Fassq (Qbottom (), parms));                                    // C 8799

  /* Move the tooltip window where the mouse pointer is.  Resize and
     show it.  */
  if (!FIXNUMP (left) && !FIXNUMP (right))                                     // C 8803
    || (!FIXNUMP (top) && !FIXNUMP (bottom))                                   // C 8804
    {                                                                          // C 8805
      let mut frame: Lisp_Object;                                              // C 8806
      let mut attributes: Lisp_Object;
      let mut monitor: Lisp_Object;
      let mut geometry: Lisp_Object;

      block_input ();                                                          // C 8808
      x_query_pointer (FRAME_X_DISPLAY! (f), (*FRAME_DISPLAY_INFO! (f)).root_window,   // C 8809-8810
		       &raw mut root, &raw mut child, root_x, root_y, &raw mut win_x,
		       &raw mut win_y, &raw mut pmask);
      unblock_input ();                                                        // C 8811

      XSETFRAME! (frame, f);                                                   // C 8813

#[cfg (any (feature = "have_xrandr", feature = "use_gtk"))]                    // C 8815
      if !NILP ((*FRAME_DISPLAY_INFO! (f)).last_monitor_attributes_list)       // C 8816
	{ attributes = (*FRAME_DISPLAY_INFO! (f)).last_monitor_attributes_list; }   // C 8821
      else
	{ attributes = Fx_display_monitor_attributes_list (frame); }                // C 8824
#[cfg (not (any (feature = "have_xrandr", feature = "use_gtk")))]
      { attributes = Fx_display_monitor_attributes_list (frame); }             // C 8824

      /* Try to determine the monitor where the mouse pointer is and
         its geometry.  See bug#22549.  */
      while CONSP (attributes)                                                 // C 8828
	{                                                                      // C 8829
	  monitor = XCAR (attributes);                                         // C 8830
	  geometry = Fassq (Qgeometry (), monitor);                            // C 8831
	  if CONSP (geometry)                                                  // C 8832
	    {                                                                  // C 8833
	      min_x = XFIXNUM (Fnth (make_fixnum (1), geometry)) as c_int;     // C 8834
	      min_y = XFIXNUM (Fnth (make_fixnum (2), geometry)) as c_int;     // C 8835
	      max_x = min_x + (XFIXNUM (Fnth (make_fixnum (3), geometry)) as c_int);   // C 8836
	      max_y = min_y + (XFIXNUM (Fnth (make_fixnum (4), geometry)) as c_int);   // C 8837

	      if min_x <= *root_x && *root_x < max_x                         // C 8839
		  && min_y <= *root_y && *root_y < max_y                       // C 8840
		{
		  break;                                                       // C 8842
		}

	      max_y = -1;                                                      // C 8845
	    }

	  attributes = XCDR (attributes);                                      // C 8848
	}                                                                      // C 8849
    }                                                                          // C 8850

  /* It was not possible to determine the monitor's geometry, so we
     assign some sane defaults here: */
  if max_y < 0                                                                 // C 8854
    {                                                                          // C 8855
      min_x = 0;                                                               // C 8856
      min_y = 0;                                                               // C 8857
      max_x = x_display_pixel_width (FRAME_DISPLAY_INFO! (f));                 // C 8858
      max_y = x_display_pixel_height (FRAME_DISPLAY_INFO! (f));                // C 8859
    }                                                                          // C 8860

  if FIXNUMP (top)                                                             // C 8862
    { *root_y = XFIXNUM (top) as c_int; }                                      // C 8863
  else if FIXNUMP (bottom)                                                     // C 8864
    { *root_y = XFIXNUM (bottom) as c_int - height; }                          // C 8865
  else if *root_y + (XFIXNUM (dy) as c_int) <= min_y                           // C 8866
    { *root_y = min_y; } /* Can happen for negative dy */                      // C 8867
  else if *root_y + (XFIXNUM (dy) as c_int) + height <= max_y                  // C 8868
    /* It fits below the pointer */                                            // C 8869
    { *root_y += XFIXNUM (dy) as c_int; }                                      // C 8870
  else if height + (XFIXNUM (dy) as c_int) + min_y <= *root_y                  // C 8871
    /* It fits above the pointer.  */                                          // C 8872
    { *root_y -= height + (XFIXNUM (dy) as c_int); }                           // C 8873
  else
    /* Put it on the top.  */                                                  // C 8875
    { *root_y = min_y; }                                                       // C 8876

  if FIXNUMP (left)                                                            // C 8878
    { *root_x = XFIXNUM (left) as c_int; }                                     // C 8879
  else if FIXNUMP (right)                                                      // C 8880
    { *root_x = XFIXNUM (right) as c_int - width; }                            // C 8881
  else if *root_x + (XFIXNUM (dx) as c_int) <= min_x                           // C 8882
    { *root_x = 0; } /* Can happen for negative dx */                          // C 8883
  else if *root_x + (XFIXNUM (dx) as c_int) + width <= max_x                   // C 8884
    /* It fits to the right of the pointer.  */                                // C 8885
    { *root_x += XFIXNUM (dx) as c_int; }                                      // C 8886
  else if width + (XFIXNUM (dx) as c_int) + min_x <= *root_x                   // C 8887
    /* It fits to the left of the pointer.  */                                 // C 8888
    { *root_x -= width + (XFIXNUM (dx) as c_int); }                            // C 8889
  else
    /* Put it left justified on the screen -- it ought to fit that way.  */    // C 8891
    { *root_x = min_x; }                                                       // C 8892
}                                                                              // C 8893

/* C: gtkutil.h 216: `extern bool xg_hide_tooltip (struct frame *f);` - C 8949's call in the
   `USE_GTK` arm.  `src/gtkutil.rs` defines it twice, one per `use_gtk` arm, so it stands at the
   use site (P-55/P-72 rule 3; the `src/pgtkfns.rs` 448-451 row).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn xg_hide_tooltip (f: *mut frame) -> bool;
}

/**
 * x_hide_tip:
 *
 * Hide currently visible tooltip and cancel its timer.
 *
 * If GTK+ system tooltips are used, this will try to hide the tooltip
 * referenced by the x_output structure of tooltip_last_frame.  For
 * Emacs tooltips this will try to make tooltip_frame invisible (if
 * DELETE is false) or delete tooltip_frame (if DELETE is true).
 *
 * Return Qt if the tooltip was either deleted or made invisible, Qnil
 * otherwise.
 */


/* C 8909-9028.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_hide_tip (delete: bool) -> Lisp_Object                            // C 8909-8910
{                                                                            // C 8911
  if !NILP (tip_timer)                                                       // C 8912
    {
      calln! (Qcancel_timer (), tip_timer);                                  // C 8914
      tip_timer = Qnil ();                                                   // C 8915
    }

#[cfg (feature = "use_gtk")]                                                 // C 8918
  {
  /* Any GTK+ system tooltip can be found via the x_output structure
     of tip_last_frame, provided that frame is still live.  Any Emacs
     tooltip is found via the tip_frame variable.  Note that the
     current value of use_system_tooltips might not be the same as
     used for the tooltip we have to hide, see Bug#30399.  */
    if (NILP (tip_last_frame) && NILP (tip_frame))                           // C 8924
      || (!use_system_tooltips                                               // C 8925
	  && !delete                                                           // C 8926
	  && !NILP (tip_frame)                                                 // C 8927
	  && FRAME_LIVE_P (XFRAME (tip_frame))                                 // C 8928
	  && !FRAME_VISIBLE_P (XFRAME (tip_frame)))                            // C 8929
    /* Either there's no tooltip to hide or it's an already invisible
       Emacs tooltip and we don't want to change its type.  Return
       quickly.  */
      { return Qnil (); }                                                    // C 8933
    else
      {
	let mut was_open: Lisp_Object = Qnil ();                              // C 8936

	let count: specpdl_ref = SPECPDL_INDEX ();                            // C 8938
	specbind (Qinhibit_redisplay (), Qt ());                              // C 8939
	specbind (Qinhibit_quit (), Qt ());                                   // C 8940

	/* Try to hide the GTK+ system tip first.  */                         // C 8942
	if !NILP (tip_last_frame)                                             // C 8943
	  {
	    let f: *mut frame = XFRAME (tip_last_frame);                      // C 8945

	    if FRAME_LIVE_P (f)                                               // C 8947
	      {
		if xg_hide_tooltip (f)                                        // C 8949
		  { was_open = Qt (); }                                       // C 8950
	      }
	  }

      /* When using GTK+ system tooltips (compare Bug#41200) reset
	 tip_last_frame.  It will be reassigned when showing the next
	 GTK+ system tooltip.  */
	if use_system_tooltips                                               // C 8957
	  { tip_last_frame = Qnil (); }                                       // C 8958

	/* Now look whether there's an Emacs tip around.  */                  // C 8960
	if !NILP (tip_frame)                                                 // C 8961
	  {
	    let f: *mut frame = XFRAME (tip_frame);                           // C 8963

	    if FRAME_LIVE_P (f)                                               // C 8965
	      {
		if delete || use_system_tooltips                              // C 8967
		  {
		  /* Delete the Emacs tooltip frame when DELETE is true
		     or we change the tooltip type from an Emacs one to
		     a GTK+ system one.  */
		    delete_frame (tip_frame, Qnil ());                        // C 8972
		    tip_frame = Qnil ();                                      // C 8973
		  }
		else
		  { x_make_frame_invisible (f); }                             // C 8976

		was_open = Qt ();                                             // C 8978
	      }
	    else
	      { tip_frame = Qnil (); }                                     // C 8981
	  }
	else
	  { tip_frame = Qnil (); }                                           // C 8984

	return unbind_to (count, was_open);                                  // C 8986
      }
  }
#[cfg (not (feature = "use_gtk"))]                                          // C 8988
  {
    if NILP (tip_frame)                                                      // C 8989
      || (!delete                                                            // C 8990
	  && !NILP (tip_frame)                                                  // C 8991
	  && FRAME_LIVE_P (XFRAME (tip_frame))                                  // C 8992
	  && !FRAME_VISIBLE_P (XFRAME (tip_frame)))                             // C 8993
      { return Qnil (); }                                                    // C 8994
    else
      {
	let mut was_open: Lisp_Object = Qnil ();                              // C 8997

	let count: specpdl_ref = SPECPDL_INDEX ();                            // C 8999
	specbind (Qinhibit_redisplay (), Qt ());                              // C 9000
	specbind (Qinhibit_quit (), Qt ());                                   // C 9001

	if !NILP (tip_frame)                                                 // C 9003
	  {
	    let f: *mut frame = XFRAME (tip_frame);                           // C 9005

	    if FRAME_LIVE_P (f)                                               // C 9007
	      {
		if delete                                                     // C 9009
		  {
		    delete_frame (tip_frame, Qnil ());                        // C 9011
		    tip_frame = Qnil ();                                      // C 9012
		  }
		else
		  { x_make_frame_invisible (XFRAME (tip_frame)); }             // C 9015

		was_open = Qt ();                                             // C 9017
	      }
	    else
	      { tip_frame = Qnil (); }                                     // C 9020
	  }
	else
	  { tip_frame = Qnil (); }                                           // C 9023

	return unbind_to (count, was_open);                                  // C 9025
      }
  }                                                                        // C 9027
}                                                                            // C 9028
/* C: dispextern.h 3069-3196's `struct redisplay_interface` - the mirror carries it behind
   `feature = "dispextern_face_records"` (off in every configuration today), so this unit
   carries C's record itself for `flush_frame!`'s expansion, as `src/haikufns.rs` 1331-1379
   and `src/androidterm.rs` 111-126 do for their own uses.  */
#[cfg (all (feature = "have_x_windows", not (feature = "dispextern_face_records")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct redisplay_interface
{
  pub frame_parm_handlers: *mut c_void,                                        // offset 0
  /// C: dispextern.h 3076: `void (*produce_glyphs) (struct it *it);` - `struct it` is a record in
  /// the mirror's `dispextern_unit_names` module (not glob-imported), so only the pointee is
  /// untyped here (PORTING.md P-55/P-72 rule 3); the field is one pointer wide either way.
  pub produce_glyphs: Option<unsafe extern "C" fn (*mut c_void)>,             // offset 8
  pub write_glyphs: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, string: *mut glyph, area: c_int, len: c_int)>,
  pub insert_glyphs: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, start: *mut glyph, area: c_int, len: c_int)>,
  pub clear_end_of_line: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, x: c_int)>,
  pub scroll_run_hook: Option<unsafe extern "C" fn (w: *mut c_void, run: *mut run)>,
  pub after_update_window_line_hook: Option<unsafe extern "C" fn (w: *mut c_void, desired_row: *mut glyph_row)>,
  pub update_window_begin_hook: Option<unsafe extern "C" fn (w: *mut c_void)>,
  pub update_window_end_hook: Option<unsafe extern "C" fn (w: *mut c_void, cursor_on_p: bool, mouse_face_overwritten_p: bool)>,
  /* the mirror's `flush_frame!` passes its own `$f` (a `*mut frame`) to this hook, so the
     pointee is C's `struct frame` here (unlike the other hooks, which this unit never calls) */
  pub flush_display: Option<unsafe extern "C" fn (f: *mut frame)>,            // 72
  pub clear_window_mouse_face: Option<unsafe extern "C" fn (w: *mut c_void)>,  // 80
  pub get_glyph_overhangs: Option<unsafe extern "C" fn (glyph: *mut glyph, f: *mut c_void, left: *mut c_int, right: *mut c_int)>,
  pub fix_overlapping_area: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, overlaps: c_int)>,  // 96
  /// C: dispextern.h 3134: `void (*draw_fringe_bitmap) (struct window *, struct glyph_row *,
  /// struct draw_fringe_bitmap_params *);` - `draw_fringe_bitmap_params` is in the same
  /// non-glob-imported module, so only the pointee is untyped (as above).
  pub draw_fringe_bitmap: Option<unsafe extern "C" fn (*mut c_void, *mut glyph_row, *mut c_void)>,
  pub define_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int, bits: *mut c_ushort, h: c_int, wd: c_int)>,
  pub destroy_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int)>,
  /* C's `glyph_string` is in the mirror's non-glob-imported module, so only the pointee is untyped */
  pub compute_glyph_string_overhangs: Option<unsafe extern "C" fn (s: *mut c_void)>,
  pub draw_glyph_string: Option<unsafe extern "C" fn (s: *mut c_void)>,
  pub define_frame_cursor: Option<unsafe extern "C" fn (f: *mut c_void, cursor: Emacs_Cursor)>,
  pub clear_frame_area: Option<unsafe extern "C" fn (f: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int)>,
  pub clear_under_internal_border: Option<unsafe extern "C" fn (f: *mut c_void)>,
  pub draw_window_cursor: Option<unsafe extern "C" fn (w: *mut c_void, glyph_row: *mut glyph_row, x: c_int, y: c_int, cursor_type: text_cursor_kinds, cursor_width: c_int, on_p: bool, active_p: bool)>,
  pub draw_vertical_window_border: Option<unsafe extern "C" fn (w: *mut c_void, x: c_int, y_0: c_int, y_1: c_int)>,
  pub draw_window_divider: Option<unsafe extern "C" fn (w: *mut c_void, x_0: c_int, x_1: c_int, y_0: c_int, y_1: c_int)>,
  pub shift_glyphs_for_insert: Option<unsafe extern "C" fn (f: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int, shift_by: c_int)>,
  pub show_hourglass: Option<unsafe extern "C" fn (f: *mut c_void)>,
  pub hide_hourglass: Option<unsafe extern "C" fn (f: *mut c_void)>,
  pub default_font_parameter: Option<unsafe extern "C" fn (f: *mut c_void, parms: Lisp_Object)>,  // 216
}
#[cfg (all (feature = "have_x_windows", not (feature = "dispextern_face_records")))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, default_font_parameter)
                             == 216);

/* C: frame.h 1835-1840: `INLINE void flush_frame (struct frame *f)` - the mirror
   `include/frame.rs`'s macro spells `FRAME_RIF` without the `*mut c_void` cast this unit's
   `FRAME_RIF!` needs (`terminal.rif` is `*mut c_void` there), so the unit shadows it with C's
   own body (the `src/haikufns.rs` 2049-2056 shape).  */
#[cfg (feature = "have_x_windows")]
macro_rules! flush_frame {
  ($f:expr) => {
    {
      let rif: *mut redisplay_interface = FRAME_RIF! ($f) as *mut redisplay_interface;
      if !rif.is_null () && (*rif).flush_display.is_some ()
	{ ((*rif).flush_display.unwrap ()) ($f); }
    }
  };
}

/* C 9293/9294/9277/9249/9358's macro and function names are not in this unit's include graph
   (each ported unit that touches them keeps its own copy - the `src/haikufns.rs` 279-289,
   2070-2077 and 193-199 device; the `WINDOW_*_SHIFT` rows are `include/window.rs`'s).  */
/* C: dispextern.h 291-294: `#define SET_TEXT_POS(POS, CHARPOS, BYTEPOS) ((POS).charpos = (CHARPOS),  */
/* (POS).bytepos = BYTEPOS)`  */
#[cfg (feature = "have_x_windows")]
macro_rules! SET_TEXT_POS {
  ($pos:expr, $charpos:expr, $bytepos:expr) => {
    { ($pos).charpos = ($charpos); ($pos).bytepos = ($bytepos); }
  };
}
/* C: frame.h 1118-1120: `#define FRAME_TOTAL_COLS(f) ((f)->total_cols)` - C 9277 writes it.  */
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_TOTAL_COLS {
  ($f:expr, $v:expr) => { (*($f)).total_cols = $v };
}
/* C: window.h's `WINDOW_PSEUDO_WINDOW_P`/`WINDOW_MUST_BE_UPDATED_P` setters (C 9249/9358).  */
#[cfg (feature = "have_x_windows")]
macro_rules! window_bits {
  (update_mode_line, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_UPDATE_MODE_LINE_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_UPDATE_MODE_LINE_SHIFT) }; };
  (pseudo_window_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }; };
  (must_be_updated_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_MUST_BE_UPDATED_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_MUST_BE_UPDATED_P_SHIFT) }; };
}
/* C: window.h 1139-1140: `void set_window_buffer (Lisp_Object window, Lisp_Object buffer,  */
/* bool run_hooks_p, bool keep_margins_p);` - C 9247's call.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn set_window_buffer (window: Lisp_Object, buffer: Lisp_Object, run_hooks_p: bool,
			    keep_margins_p: bool);
}
/* C: dispextern.h 3573: `int try_window (Lisp_Object, struct text_pos, int);` - C 9294's call.  */
#[cfg (feature = "have_x_windows")]
/* C: dispextern.h 3918: `void update_single_window (struct window *);` - C 9359's call.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn update_single_window (w: *mut window);
}

/* C: Xlib.h 2844: `extern int XMoveWindow (Display *, Window, int, int);` - C 9148 calls it
   in every arm (it sits outside the `#ifndef USE_XCB` pair), so its gate is the file gate.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XMoveWindow (display: *mut Display, w: Window, x: c_int, y: c_int) -> c_int;
}

/* The names C 9107-9343's two arms call and that no mirror carries: the two `gtkutil.h`
   entry points (207-210 and 212), the three Xlib calls (Xlib.h 2806/2835/2844) and the xcb pair
   with its five enum members (xproto.h 6298/6447 and 1717-1727).  Each keeps C's prototype with
   its header and line (P-55/P-72 rule 3).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  /* C: gtkutil.h 207-210: `extern bool xg_prepare_tooltip (struct frame *f, Lisp_Object string,  */
  /* int *width, int *height);`  */
  pub fn xg_prepare_tooltip (f: *mut frame, string: Lisp_Object, width: *mut c_int,
			     height: *mut c_int) -> bool;
  /* C: gtkutil.h 212: `extern void xg_show_tooltip (struct frame *f, int root_x, int root_y);`  */
  /* (the `#ifndef HAVE_PGTK` arm; `src/gtkutil.rs` 1380 defines it)  */
  pub fn xg_show_tooltip (f: *mut frame, root_x: c_int, root_y: c_int);
}
/* C: xterm.h 1812's `x_cr_update_surface_desired_size` is declared once, in the file-gate block
   above (the reference declares it once too); the `use_cairo` duplicate that stood here was an
   `E0428` no arm could see until status 3.949 added the CAIRO arm.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_xcb")))]
unsafe extern "C" {
  /* C: Xlib.h 2806: `extern int XMapRaised (Display *, Window);`  */
  pub fn XMapRaised (display: *mut Display, w: Window) -> c_int;
  /* C: Xlib.h 2835: `extern int XMoveResizeWindow (Display *, Window, int, int, unsigned int,  */
  /* unsigned int);`  */
  pub fn XMoveResizeWindow (display: *mut Display, w: Window, x: c_int, y: c_int, width: c_uint,
			    height: c_uint) -> c_int;
}
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CONFIG_WINDOW_X: u32 = 1;              /* C: xcb/xproto.h 1717  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CONFIG_WINDOW_Y: u32 = 2;              /* C: xcb/xproto.h 1718  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CONFIG_WINDOW_WIDTH: u32 = 4;          /* C: xcb/xproto.h 1719  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CONFIG_WINDOW_HEIGHT: u32 = 8;         /* C: xcb/xproto.h 1720  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_CONFIG_WINDOW_STACK_MODE: u32 = 64;    /* C: xcb/xproto.h 1723  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub const XCB_STACK_MODE_ABOVE: u32 = 0;             /* C: xcb/xproto.h 1727  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
unsafe extern "C" {
  /* C: xcb/xproto.h 6298-6301: `xcb_void_cookie_t xcb_map_window (xcb_connection_t *c,
     xcb_window_t window);`  */
  pub fn xcb_map_window (c: *mut xcb_connection_t, window: xcb_window_t) -> c_int;
  /* C: xcb/xproto.h 6447-6452: `xcb_void_cookie_t xcb_configure_window (xcb_connection_t *c,
     xcb_window_t window, uint16_t value_mask, const void *value_list);`  */
  pub fn xcb_configure_window (c: *mut xcb_connection_t, window: xcb_window_t,
			       value_mask: u16, value_list: *const c_void) -> c_int;
}

/* C 9031-9372.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-show-tip", Fx_show_tip, Sx_show_tip, 1, 6, 0,
	doc: "Show STRING in a \"tooltip\" window on frame FRAME.
A tooltip window is a small X window displaying a string.

This is an internal function; Lisp code should call `tooltip-show'.

FRAME nil or omitted means use the selected frame.

PARMS is an optional list of frame parameters which can be used to
change the tooltip's appearance.

Automatically hide the tooltip after TIMEOUT seconds.  TIMEOUT nil
means use the default timeout from the `x-show-tooltip-timeout'
variable.

If the list of frame parameters PARMS contains a `left' parameter,
display the tooltip at that x-position.  If the list of frame parameters
PARMS contains no `left' but a `right' parameter, display the tooltip
right-adjusted at that x-position. Otherwise display it at the
x-position of the mouse, with offset DX added (default is 5 if DX isn't
specified).

Likewise for the y-position: If a `top' frame parameter is specified, it
determines the position of the upper edge of the tooltip window.  If a
`bottom' parameter but no `top' frame parameter is specified, it
determines the position of the lower edge of the tooltip window.
Otherwise display the tooltip window at the y-position of the mouse,
with offset DY added (default is -10).

A tooltip's maximum size is specified by `x-max-tooltip-size'.
Text larger than the specified size is clipped. ",
	(string: Lisp_Object, frame: Lisp_Object, parms: Lisp_Object,
	 timeout: Lisp_Object, dx: Lisp_Object, dy: Lisp_Object),
	{
  let mut f: *mut frame;                                                   // C 9065
  let mut tip_f: *mut frame;
  let mut w: *mut window;                                                  // C 9066
  let mut root_x: c_int = 0;                                               // C 9067
  let mut root_y: c_int = 0;
  let mut old_buffer: *mut buffer;                                         // C 9068
  let mut pos: text_pos = std::mem::zeroed ();                             // C 9069
  let mut width: c_int = 0;                                                // C 9070
  let mut height: c_int = 0;
  let old_windows_or_buffers_changed: c_int = windows_or_buffers_changed;   // C 9071
  let count: specpdl_ref = SPECPDL_INDEX ();                               // C 9072
  let mut window: Lisp_Object;                                             // C 9073
  let mut size: Lisp_Object;
  let mut tip_buf: Lisp_Object;
  let mut displayed: bool;                                                 // C 9074
  /* C 9075-9077: `#ifdef ENABLE_CHECKING` around the two row locals.  */
  #[cfg (feature = "enable_checking")]
  let mut row: *mut glyph_row = std::ptr::null_mut ();
  #[cfg (feature = "enable_checking")]
  let mut end: *mut glyph_row;
  AUTO_STRING! (tip, c" *tip*".as_ptr ());                                  // C 9078

  /* C's two `goto start_timer` jumps (C 9125/9152) and its label (C 9367) become this labelled
     block, whose `break 'start_timer` is the port's spelling of the jump (PORTING.md 2.2).  */
  let mut string = string;
  let mut frame = frame;
  let mut parms = parms;
  let mut timeout = timeout;
  let mut dx = dx;
  let mut dy = dy;

  'start_timer: {
  specbind (Qinhibit_redisplay (), Qt ());                                  // C 9080

  CHECK_STRING (string);                                                    // C 9082
  if SCHARS (string) == 0                                                   // C 9083
    { string = make_unibyte_string (c" ".as_ptr (), 1); }                    // C 9084

  if NILP (frame)                                                           // C 9086
    { frame = selected_frame; }                                              // C 9087
  f = decode_window_system_frame (frame);                                   // C 9088

  if NILP (timeout)                                                         // C 9090
    { timeout = Vx_show_tooltip_timeout; }                                   // C 9091
  CHECK_FIXNAT (timeout);                                                   // C 9092

  if NILP (dx)                                                              // C 9094
    { dx = make_fixnum (5); }                                                // C 9095
  else
    { CHECK_FIXNUM (dx); }                                                   // C 9097

  if NILP (dy)                                                              // C 9099
    { dy = make_fixnum (-10); }                                              // C 9100
  else
    { CHECK_FIXNUM (dy); }                                                   // C 9102

  tip_dx = dx;                                                              // C 9104
  tip_dy = dy;                                                              // C 9105

#[cfg (feature = "use_gtk")]                                                // C 9107
  {
    if use_system_tooltips                                                   // C 9108
      {
	let ok: bool;                                                          // C 9110

	/* Hide a previous tip, if any.  */                                   // C 9112
	Fx_hide_tip ();                                                       // C 9113

	block_input ();                                                       // C 9115
	ok = xg_prepare_tooltip (f, string, &raw mut width, &raw mut height);  // C 9116
	if ok                                                                 // C 9117
	  {
	    compute_tip_xy (f, parms, dx, dy, width, height, &raw mut root_x,   // C 9119
			    &raw mut root_y);
	    xg_show_tooltip (f, root_x, root_y);                               // C 9120
	    tip_last_frame = frame;                                            // C 9121
	  }

	unblock_input ();                                                     // C 9124
	if ok { break 'start_timer; }                                          // C 9125
      }
  }                                                                        // C 9127

  if !NILP (tip_frame) && FRAME_LIVE_P (XFRAME (tip_frame))                 // C 9129
    {
      if (FRAME_VISIBLE_P (XFRAME (tip_frame))                              // C 9131
	  && (FRAME_X_DISPLAY! (XFRAME (frame))                               // C 9132
	      == FRAME_X_DISPLAY! (XFRAME (tip_last_frame)))                   // C 9133
	  && !NILP (Fequal_including_properties (tip_last_string, string))     // C 9134
	  && !NILP (Fequal (tip_last_parms, parms)))                           // C 9135
	{                                                                      // C 9136
	  /* Only DX and DY have changed.  */                                 // C 9137
	  tip_f = XFRAME (tip_frame);                                          // C 9138
	  if !NILP (tip_timer)                                                 // C 9139
	    {
	      calln! (Qcancel_timer (), tip_timer);                            // C 9141
	      tip_timer = Qnil ();                                              // C 9142
	    }

	  block_input ();                                                       // C 9145
	  compute_tip_xy (tip_f, parms, dx, dy, FRAME_PIXEL_WIDTH (tip_f),      // C 9146-9147
			  FRAME_PIXEL_HEIGHT (tip_f), &raw mut root_x, &raw mut root_y);
	  XMoveWindow (FRAME_X_DISPLAY! (tip_f), FRAME_X_WINDOW! (tip_f),       // C 9148-9149
		       root_x, root_y);
	  unblock_input ();                                                     // C 9150

	  break 'start_timer;                                                    // C 9152
	}
      else if (tooltip_reuse_hidden_frame && BASE_EQ (frame, tip_last_frame))
	{                                                                      // C 9155
	  let mut delete: bool = false;                                         // C 9156
	  let mut tail: Lisp_Object;                                            // C 9157
	  let mut elt: Lisp_Object;
	  let mut parm: Lisp_Object;
	  let mut last: Lisp_Object;

	  /* Check if every parameter in PARMS has the same value in
	     tip_last_parms.  This may destruct tip_last_parms which,
	     however, will be recreated below.  */
	  tail = parms;                                                         // C 9162
	  while CONSP (tail)
	    {
	      elt = XCAR (tail);                                                 // C 9164
	      parm = CAR (elt);                                                 // C 9165
	      /* The left, top, right and bottom parameters are handled
		 by compute_tip_xy so they can be ignored here.  */
	      if !EQ (parm, Qleft ()) && !EQ (parm, Qtop ())                     // C 9168
		  && !EQ (parm, Qright ()) && !EQ (parm, Qbottom ())              // C 9169
		{
		  last = Fassq (parm, tip_last_parms);                            // C 9171
		  if NILP (Fequal (CDR (elt), CDR (last)))                      // C 9172
		    {
		      /* We lost, delete the old tooltip.  */
		      delete = true;                                              // C 9175
		      break;
		    }
		  else
		    { tip_last_parms                                                   // C 9179-9180
			= calln! (Qassq_delete_all (), parm, tip_last_parms); }
		}
	      else
		{ tip_last_parms                                                     // C 9183-9184
		    = calln! (Qassq_delete_all (), parm, tip_last_parms); }
	      tail = XCDR (tail);                                                // (C 9162's increment)
	    }

	  /* Now check if every parameter in what is left of
	     tip_last_parms with a non-nil value has an association in
	     PARMS.  */
	  tail = tip_last_parms;                                                // C 9190
	  while CONSP (tail)
	    {
	      elt = XCAR (tail);                                                 // C 9192
	      parm = CAR (elt);                                                 // C 9193
	      if (!EQ (parm, Qleft ()) && !EQ (parm, Qtop ()) && !EQ (parm, Qright ())   // C 9194
		  && !EQ (parm, Qbottom ()) && !NILP (CDR (elt)))                 // C 9195
		{
		  /* We lost, delete the old tooltip.  */
		  delete = true;                                                  // C 9198
		  break;
		}
	      tail = XCDR (tail);                                                // (C 9190's increment)
	    }

	  x_hide_tip (delete);                                                   // C 9203
	}
      else
	{ x_hide_tip (true); }                                                  // C 9206
    }
  else
    { x_hide_tip (true); }                                                   // C 9209

  tip_last_frame = frame;                                                   // C 9211
  tip_last_string = string;                                                 // C 9212
  tip_last_parms = parms;                                                   // C 9213

  if NILP (tip_frame) || !FRAME_LIVE_P (XFRAME (tip_frame))                  // C 9215
    {
      /* Add default values to frame parameters.  */
      if NILP (Fassq (Qname (), parms))                                      // C 9218
	{ parms = Fcons (Fcons (Qname (), build_string (c"tooltip".as_ptr ())), parms); }   // C 9219
      if NILP (Fassq (Qinternal_border_width (), parms))                     // C 9220
	{ parms = Fcons (Fcons (Qinternal_border_width (), make_fixnum (3)), parms); }   // C 9221
      if NILP (Fassq (Qborder_width (), parms))                              // C 9222
	{ parms = Fcons (Fcons (Qborder_width (), make_fixnum (1)), parms); }   // C 9223
      if NILP (Fassq (Qborder_color (), parms))                              // C 9224
	{ parms = Fcons (Fcons (Qborder_color (),
			   build_string (c"lightyellow".as_ptr ())), parms); }         // C 9225
      if NILP (Fassq (Qbackground_color (), parms))                          // C 9226
	{ parms = Fcons (Fcons (Qbackground_color (),
			   build_string (c"lightyellow".as_ptr ())), parms); }         // C 9227-9228

      /* Create a frame for the tooltip, and record it in the global
	 variable tip_frame.  */
      tip_frame = x_create_tip_frame (FRAME_DISPLAY_INFO! (f), parms);        // C 9232
      if NILP (tip_frame)
	/* Creating the tip frame failed.  */                                   // C 9233
	{ return unbind_to (count, Qnil ()); }                                  // C 9234
    }
  else
    /* Required by X11 drag and drop.  */                                   // C 9237
    { tip_window = FRAME_X_WINDOW! (XFRAME (tip_frame)); }                   // C 9238

  tip_f = XFRAME (tip_frame);                                               // C 9240
  window = FRAME_ROOT_WINDOW (tip_f);                                       // C 9241
  tip_buf = Fget_buffer_create (tip, Qnil ());                              // C 9242
  /* We will mark the tip window a "pseudo-window" below, and such
     windows cannot have display margins.  */
  bset_left_margin_cols (XBUFFER (tip_buf), make_fixnum (0));               // C 9245
  bset_right_margin_cols (XBUFFER (tip_buf), make_fixnum (0));              // C 9246
  set_window_buffer (window, tip_buf, false, false);                        // C 9247
  w = XWINDOW (window);                                                      // C 9248
  window_bits! (pseudo_window_p, w, true);                                   // C 9249
  /* Try to avoid that `other-window' select us (Bug#47207).  */             // C 9250
  Fset_window_parameter (window, Qno_other_window (), Qt ());                 // C 9251

  /* Set up the frame's root window.  Note: The following code does not
     try to size the window or its frame correctly.  Its only purpose is
     to make the subsequent text size calculations work.  The right
     sizes should get installed when the toolkit gets back to us.  */
  (*w).left_col = 0;                                                        // C 9257
  (*w).top_line = 0;                                                        // C 9258
  (*w).pixel_left = 0;                                                      // C 9259
  (*w).pixel_top = 0;                                                       // C 9260

  if CONSP (Vx_max_tooltip_size)                                            // C 9262
      && RANGED_FIXNUMP (1, XCAR (Vx_max_tooltip_size), INT_MAX as EMACS_INT)   // C 9263
      && RANGED_FIXNUMP (1, XCDR (Vx_max_tooltip_size), INT_MAX as EMACS_INT)   // C 9264
    {
      (*w).total_cols = XFIXNAT (XCAR (Vx_max_tooltip_size)) as c_int;       // C 9266
      (*w).total_lines = XFIXNAT (XCDR (Vx_max_tooltip_size)) as c_int;      // C 9267
    }
  else
    {
      (*w).total_cols = 80;                                                  // C 9271
      (*w).total_lines = 40;                                                 // C 9272
    }

  (*w).pixel_width = (*w).total_cols * FRAME_COLUMN_WIDTH (tip_f);           // C 9275
  (*w).pixel_height = (*w).total_lines * FRAME_LINE_HEIGHT (tip_f);          // C 9276
  FRAME_TOTAL_COLS! (tip_f, (*w).total_cols);                                // C 9277
  adjust_frame_glyphs (tip_f);                                               // C 9278

  /* Insert STRING into root window's buffer and fit the frame to the
     buffer.  */
  let count_1: specpdl_ref = SPECPDL_INDEX ();                               // C 9282
  old_buffer = current_buffer! ();                                           // C 9283
  set_buffer_internal_1 (XBUFFER ((*w).contents));                           // C 9284
  bset_truncate_lines (current_buffer! (), Qnil ());                         // C 9285
  specbind (Qinhibit_read_only (), Qt ());                                   // C 9286
  specbind (Qinhibit_modification_hooks (), Qt ());                          // C 9287
  specbind (Qinhibit_point_motion_hooks (), Qt ());                          // C 9288
  Ferase_buffer ();                                                          // C 9289
  Finsert (1, &raw mut string);                                              // C 9290
  clear_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);             // C 9291
  clear_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);             // C 9292
  SET_TEXT_POS! (pos, BEGV! (), BEGV_BYTE! ());                              // C 9293
  displayed = try_window (window, pos, TRY_WINDOW_IGNORE_FONTS_CHANGE) != 0;  // C 9294

  if !displayed && NILP (Vx_max_tooltip_size)                                // C 9296
    {
      /* C 9298-9311: `#ifdef ENABLE_CHECKING`.  */
      #[cfg (feature = "enable_checking")]
      {
	row = (*((*w).desired_matrix as *mut glyph_matrix)).rows;              // C 9299
	end = (*((*w).desired_matrix as *mut glyph_matrix)).rows                // C 9300
	  .offset ((*((*w).desired_matrix as *mut glyph_matrix)).nrows as isize);

	while row < end                                                       // C 9302
	  {
	    if !glyph_row_bits! (displays_text_p, row)                          // C 9304
		|| glyph_row_bits! (ends_at_zv_p, row)                           // C 9305
	      { break; }                                                        // C 9306
	    row = row.offset (1);                                               // C 9307
	  }

	eassert! (row < end && glyph_row_bits! (ends_at_zv_p, row));           // C 9310
      }
    }

  /* Calculate size of tooltip window.  */                                  // C 9314
  size = Fwindow_text_pixel_size (window, Qnil (), Qnil (), Qnil (),         // C 9315-9317
				  make_fixnum ((*w).pixel_height as EMACS_INT), Qnil (),
				  Qnil ());
  /* Add the frame's internal border to calculated size.  */                // C 9318
  width = (XFIXNUM (CAR (size)) as c_int) + 2 * FRAME_INTERNAL_BORDER_WIDTH (tip_f);   // C 9319
  height = (XFIXNUM (CDR (size)) as c_int) + 2 * FRAME_INTERNAL_BORDER_WIDTH (tip_f);  // C 9320

  /* Calculate position of tooltip frame.  */                               // C 9322
  compute_tip_xy (tip_f, parms, dx, dy, width, height, &raw mut root_x,      // C 9323
		  &raw mut root_y);

  /* Show tooltip frame.  */                                                // C 9325
  block_input ();                                                           // C 9326
#[cfg (not (feature = "use_xcb"))]                                         // C 9327
  {
    XMoveResizeWindow (FRAME_X_DISPLAY! (tip_f), FRAME_X_WINDOW! (tip_f),   // C 9328-9329
		       root_x, root_y, width as c_uint, height as c_uint);
    XMapRaised (FRAME_X_DISPLAY! (tip_f), FRAME_X_WINDOW! (tip_f));         // C 9330
  }
#[cfg (feature = "use_xcb")]                                               // C 9331
  {
    let values: [u32; 5] = [root_x as u32, root_y as u32, width as u32, height as u32,   // C 9332
			   XCB_STACK_MODE_ABOVE];

    xcb_configure_window ((*FRAME_DISPLAY_INFO! (tip_f)).xcb_connection,    // C 9334-9340
			  FRAME_X_WINDOW! (tip_f) as xcb_window_t,
			  (XCB_CONFIG_WINDOW_X
			   | XCB_CONFIG_WINDOW_Y
			   | XCB_CONFIG_WINDOW_WIDTH
			   | XCB_CONFIG_WINDOW_HEIGHT
			   | XCB_CONFIG_WINDOW_STACK_MODE) as u16,
			  &raw const values as *const c_void);
    xcb_map_window ((*FRAME_DISPLAY_INFO! (tip_f)).xcb_connection,          // C 9341-9342
		    FRAME_X_WINDOW! (tip_f) as xcb_window_t);
  }
  unblock_input ();                                                         // C 9344

#[cfg (feature = "use_cairo")]                                             // C 9346
  { x_cr_update_surface_desired_size (tip_f, width, height); }              // C 9347

  /* Garbage the tip frame too.  */                                         // C 9350
  SET_FRAME_GARBAGED! (tip_f);                                              // C 9351

  /* Block input around `update_single_window' and `flush_frame', lest a
     ConfigureNotify and Expose event arrive during the update, and set
     flags, e.g. garbaged_p, that are cleared once the update completes,
     leaving the requested exposure or configuration outstanding.  */
  block_input ();                                                           // C 9357
  window_bits! (must_be_updated_p, w, true);                                // C 9358
  update_single_window (w);                                                 // C 9359
  flush_frame! (tip_f);                                                     // C 9360
  unblock_input ();                                                         // C 9361

  set_buffer_internal_1 (old_buffer);                                       // C 9363
  unbind_to (count_1, Qnil ());                                             // C 9364
  windows_or_buffers_changed = old_windows_or_buffers_changed;              // C 9365

  }   /* 'start_timer: */

  /* Let the tip disappear after timeout seconds.  */                       // C 9368
  tip_timer = calln! (Qrun_at_time (), timeout, Qnil (), Qx_hide_tip ());   // C 9369

  return unbind_to (count, Qnil ());                                        // C 9371
});                                                                        // C 9372

/* C: gtkutil.h 86: `extern bool xg_uses_old_file_dialog (void);` - C 9411's call in the
   `USE_GTK` arm.  `src/gtkutil.rs` defines it and no mirror carries the row, so it stands at
   the use site (P-55/P-72 rule 3).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn xg_uses_old_file_dialog () -> bool;
}

/* C 9375-9381.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-hide-tip", Fx_hide_tip, Sx_hide_tip, 0, 0, 0,
	doc: "Hide the current tooltip window, if there is any.\nValue is t if tooltip was open, nil otherwise.  ",
	(),
	{
  return x_hide_tip (!tooltip_reuse_hidden_frame);                           // C 9380
});

/* C 9383-9394.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-double-buffered-p", Fx_double_buffered_p, Sx_double_buffered_p,
	0, 1, 0,
	doc: "Return t if FRAME is being double buffered.  ",
	(frame: Lisp_Object),
	{
#[cfg (feature = "have_xdbe")]                                                 // C 9388
  {
    let f: *mut frame = decode_live_frame (frame);                           // C 9389
    return if FRAME_X_DOUBLE_BUFFERED_P! (f) { Qt () } else { Qnil () };      // C 9390
  }
#[cfg (not (feature = "have_xdbe"))]                                           // C 9391
  { return Qnil (); }                                                        // C 9392
});


/***********************************************************************
			File selection dialog
 ***********************************************************************/

/* C 9401-9415.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-uses-old-gtk-dialog", Fx_uses_old_gtk_dialog,
	Sx_uses_old_gtk_dialog,
	0, 0, 0,
	doc: "Return t if the old Gtk+ file selection dialog is used.  ",
	(),
	{
#[cfg (feature = "use_gtk")]                                                   // C 9407
  {
    if use_dialog_box                                                        // C 9408
      && use_file_dialog                                                     // C 9409
      && window_system_available (SELECTED_FRAME ())                        // C 9410
      && xg_uses_old_file_dialog ()                                          // C 9411
      { return Qt (); }                                                      // C 9412
  }
  return Qnil ();                                                            // C 9414
});


/* C 9416-9453: C's `#ifdef USE_MOTIF` (C 9418) opens here and runs to C 9660, so this is the
   unit's first `USE_MOTIF`-gated region - the `<Xm/FileSB.h>` file selection dialog's two
   callbacks and its teardown callback.  The names they read have no mirror in this tree:
   `XmAnyCallbackStruct` and `XmCR_CANCEL` are `<Xm/Xm.h>`'s (C 102), `XtUnmanageChild` and
   `XtDestroyWidget` are `<X11/Intrinsic.h>`'s (the header the toolkit arm's block above already
   names), so each row stands at its use site (P-88 ①, the `<Xm/MwmUtil.h>` block's twin up in the
   include list).  `x_menu_set_in_use` is `xterm.h` 1991's, declared under that header's own
   `#if defined USE_GTK || defined USE_MOTIF`; `src/xmenu.rs` 976 defines it and no mirror carries
   the row, so C 9451's call is its use site (P-55/P-72 rule 3).  */

/* C 9419-9427: "Callback for "OK" and "Cancel" on file selection dialog."  */
/// C: Xm/Xm.h: `typedef struct { int reason; XEvent *event; } XmAnyCallbackStruct;` - C 9425's
/// `cb` and C 9426's `cb->reason` are the record's only two reads, carried in the header's order;
/// every offset is **UNVERIFIED** (no Motif headers on this machine).
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XmAnyCallbackStruct
{
  pub reason: c_int,        // C: int reason;
  pub event: *mut XEvent,   // C: XEvent *event;
}

/* C: X11/Intrinsic.h: `extern void XtUnmanageChild (Widget);` and `extern void XtDestroyWidget
   (Widget);` - C 9449-9450's two calls.  */
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
unsafe extern "C" {
  pub fn XtUnmanageChild (widget: Widget);
  pub fn XtDestroyWidget (widget: Widget);
}

/* C: xterm.h 1991: `extern void x_menu_set_in_use (bool);`, inside that header's own
   `#if defined USE_GTK || defined USE_MOTIF` (xterm.h 1990-1991) - C 9451's, C 9667's and
   C 9698's calls all read it, so the row carries C's own pair of arms.  */
#[cfg (all (feature = "have_x_windows", any (feature = "use_gtk", feature = "use_motif")))]
unsafe extern "C" {
  pub fn x_menu_set_in_use (in_use: bool);
}

/* C 9421-9427.  */
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
unsafe extern "C" fn file_dialog_cb (widget: Widget, client_data: XtPointer, call_data: XtPointer)
{
  let result: *mut c_int = client_data as *mut c_int;                        // C 9424
  let cb: *mut XmAnyCallbackStruct = call_data as *mut XmAnyCallbackStruct;  // C 9425
  *result = (*cb).reason;                                                    // C 9426
}

/* C: Xm/Xm.h: `#define XmCR_CANCEL 20` - C 9439's assignment.  **UNVERIFIED** (no Motif headers
   here); the value is Motif's own reason code.  */
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmCR_CANCEL: c_int = 20;

/* C 9430-9440: "Callback for unmapping a file selection dialog.  This is used to capture the
   case where a dialog is closed via a window manager's closer button, for example.  Using a
   XmNdestroyCallback didn't work in this case."  */
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
unsafe extern "C" fn file_dialog_unmap_cb (widget: Widget, client_data: XtPointer,
					   call_data: XtPointer)
{
  let result: *mut c_int = client_data as *mut c_int;                        // C 9438
  *result = XmCR_CANCEL;                                                     // C 9439
}

/* C 9442-9453: the teardown the Motif `x-file-dialog` registers with C 9561's
   `record_unwind_protect_ptr`.  */
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
unsafe extern "C" fn clean_up_file_dialog (arg: *mut c_void)
{
  let dialog: Widget = arg;                                                  // C 9445

  /* Clean up.  */                                                           // C 9447
  block_input ();                                                            // C 9448
  XtUnmanageChild (dialog);                                                  // C 9449
  XtDestroyWidget (dialog);                                                  // C 9450
  x_menu_set_in_use (false);                                                 // C 9451
  unblock_input ();                                                          // C 9452
}

/* ==============================================================================================
   C 9456-9658: the Motif `x-file-dialog` - the first `USE_MOTIF`-only subr of this unit, and its
   only Xt event loop of its own.

   The names below are the rest of the file's `USE_MOTIF` surface (slice 39 carried the first
   part): the Xlib records C 9568's `XEvent event, copy;` is read through, the two `<X11/Xlib.h>`
   entry points C 9579/9599 spend, `<X11/Intrinsic.h>`'s app-context, callback and teardown
   vocabulary, `xterm.h`'s three rows C 9520/9569/9630 read plus the `DEFER_SELECTIONS` pair C
   9483 expands to, `math.h`'s `lrint`, `dispextern.h`'s `popup_activated` and `lisp.h`'s `quit` -
   and then `<Xm/Xm.h>`/`<Xm/FileSB.h>`/`<Xm/TextF.h>`/`<Xm/List.h>`'s own surface (C 102-107),
   which has no mirror in this tree and no header on this machine, so every Xm row is C's spelling
   with the values this file can state and the rest marked **UNVERIFIED** (the
   `lwlib/lwlib-Xm.rs` rule for the same headers).

   **The union-member pattern** is `src/xmenu.rs` 1163-1170's (the same idiom in the sibling unit
   of the same library): a pointer view bound to a local named after C's *member* (`xany`, `xkey`,
   `xgeneric`, `xcookie`), because `include/xlib.rs`'s `XEvent` is the measured 192-byte record
   with an opaque body; `copy = event` - C's struct assignment - is `copy_nonoverlapping`, since
   `XEvent` carries no `Copy`.  */

/* C: <X11/Xlib.h> 936 closes the record opened at 933: `typedef struct { int type; unsigned long
   serial; Bool send_event; Display *display; Window window; } XAnyEvent;` - measured: `sizeof`
   **40**, `window`@32.  It is the view C 9576/9586 read `event.type` through.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XAnyEvent
{
  pub r#type: c_int,          // Xlib.h 934
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,         // Xlib.h 935
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XAnyEvent> () == 40);
  core::assert! (core::mem::offset_of! (XAnyEvent, r#type) == 0);
  core::assert! (core::mem::offset_of! (XAnyEvent, serial) == 8);
  core::assert! (core::mem::offset_of! (XAnyEvent, send_event) == 16);
  core::assert! (core::mem::offset_of! (XAnyEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XAnyEvent, window) == 32);
};

/* C: <X11/Xlib.h> 567 closes the record opened at 556: `typedef struct { int type; unsigned long
   serial; Bool send_event; Display *display; Window window; Window root; Window subwindow; Time
   time; int x, y; int x_root, y_root; unsigned int state; unsigned int keycode; Bool same_screen;
   } XKeyEvent;` - the view C 9577/9603-9617/9619/9622 reads `event.xkey`/`copy.xkey` through;
   measured: `sizeof` **96**, `serial`@8, `send_event`@16, `display`@24, `window`@32, `root`@40,
   `subwindow`@48, `time`@56, `x`@64, `y`@68, `x_root`@72, `y_root`@76, `state`@80, `keycode`@84,
   `same_screen`@88.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XKeyEvent
{
  pub r#type: c_int,          // Xlib.h 557
  pub serial: c_ulong,        // Xlib.h 558
  pub send_event: Bool,       // Xlib.h 559
  pub display: *mut Display,  // Xlib.h 560
  pub window: Window,         // Xlib.h 561
  pub root: Window,           // Xlib.h 562
  pub subwindow: Window,      // Xlib.h 563
  pub time: Time,             // Xlib.h 564
  pub x: c_int,               // Xlib.h 565
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,          // Xlib.h 566
  pub keycode: c_uint,
  pub same_screen: Bool,
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XKeyEvent> () == 96);
  core::assert! (core::mem::offset_of! (XKeyEvent, r#type) == 0);
  core::assert! (core::mem::offset_of! (XKeyEvent, serial) == 8);
  core::assert! (core::mem::offset_of! (XKeyEvent, send_event) == 16);
  core::assert! (core::mem::offset_of! (XKeyEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XKeyEvent, window) == 32);
  core::assert! (core::mem::offset_of! (XKeyEvent, root) == 40);
  core::assert! (core::mem::offset_of! (XKeyEvent, subwindow) == 48);
  core::assert! (core::mem::offset_of! (XKeyEvent, time) == 56);
  core::assert! (core::mem::offset_of! (XKeyEvent, x) == 64);
  core::assert! (core::mem::offset_of! (XKeyEvent, y) == 68);
  core::assert! (core::mem::offset_of! (XKeyEvent, x_root) == 72);
  core::assert! (core::mem::offset_of! (XKeyEvent, y_root) == 76);
  core::assert! (core::mem::offset_of! (XKeyEvent, state) == 80);
  core::assert! (core::mem::offset_of! (XKeyEvent, keycode) == 84);
  core::assert! (core::mem::offset_of! (XKeyEvent, same_screen) == 88);
};

/* C: <X11/Xlib.h> 952 closes the record opened at 943: `typedef struct { int type; unsigned long
   serial; Bool send_event; Display *display; int extension; int evtype; } XGenericEvent;` - the
   view C 9587-9591 reads `event.xgeneric` through; measured: `sizeof` **40**, `display`@24,
   `extension`@32, `evtype`@36.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XGenericEvent
{
  pub r#type: c_int,          // Xlib.h 944
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub extension: c_int,       // Xlib.h 948
  pub evtype: c_int,          // Xlib.h 951
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XGenericEvent> () == 40);
  core::assert! (core::mem::offset_of! (XGenericEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XGenericEvent, extension) == 32);
  core::assert! (core::mem::offset_of! (XGenericEvent, evtype) == 36);
};

/* C: <X11/Xlib.h> 963 closes the record opened at 955: `typedef struct { int type; unsigned long
   serial; Bool send_event; Display *display; int extension; int evtype; unsigned int cookie;
   void *data; } XGenericEventCookie;` - the view C 9596/9599/9601/9625 reads `event.xcookie`
   through; measured: `sizeof` **56**, `evtype`@36, `cookie`@40, `data`@48.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XGenericEventCookie
{
  pub r#type: c_int,          // Xlib.h 956
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub extension: c_int,
  pub evtype: c_int,
  pub cookie: c_uint,         // Xlib.h 961
  pub data: *mut c_void,      // Xlib.h 962
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XGenericEventCookie> () == 56);
  core::assert! (core::mem::offset_of! (XGenericEventCookie, evtype) == 36);
  core::assert! (core::mem::offset_of! (XGenericEventCookie, cookie) == 40);
  core::assert! (core::mem::offset_of! (XGenericEventCookie, data) == 48);
};

/* C: `<X11/X.h>` and `<X11/keysymdef.h>`: the three values C 9576/9586/9582 compares against,
   each measured on this machine's `xorgproto` headers (`/opt/homebrew/include/X11`).  */
/// C: X11/X.h 181: `#define KeyPress 2`
#[cfg (feature = "have_x_windows")]
pub const KeyPress: c_int = 2;
/// C: X11/X.h 214: `#define GenericEvent 35`
#[cfg (feature = "have_x_windows")]
pub const GenericEvent: c_int = 35;
/// C: X11/keysymdef.h 696: `#define XK_g 0x0067 /* U+0067 LATIN SMALL LETTER G */`
#[cfg (feature = "have_x_windows")]
pub const XK_g: KeySym = 0x0067;

/* C: <X11/extensions/XInput2.h> (libXi - not on this machine, C 71): the record C 9601's cast
   and C 9604-9616's reads spend.  The members are the header's documented order up to `mods`
   (whose `effective` C 9615 reads); `buttons`/`valuators` are carried because `mods` sits behind
   them, and every offset here is **UNVERIFIED**.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIButtonState
{
  pub num_buttons: c_int,
  pub labels: *mut Atom,
  pub state: *mut c_uchar,
}
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIValuatorState
{
  pub num_valuators: c_int,
  pub valuators: *mut c_int,
  pub values: *mut f64,
}
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIModifierState
{
  pub base: c_int,
  pub latched: c_int,
  pub locked: c_int,
  pub effective: c_int,       // C 9615's `xev->mods.effective`
}
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XIDeviceEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,        // C 9604
  pub send_event: Bool,       // C 9605
  pub display: *mut Display,
  pub extension: c_int,
  pub evtype: c_int,
  pub deviceid: c_int,
  pub sourceid: c_int,
  pub detail: c_int,          // C 9616
  pub flags: c_int,
  pub root: Window,           // C 9608
  pub event: Window,          // C 9607
  pub child: Window,          // C 9609
  pub root_x: f64,            // C 9613
  pub root_y: f64,            // C 9614
  pub event_x: f64,           // C 9611
  pub event_y: f64,           // C 9612
  pub time: Time,             // C 9610
  pub buttons_len: c_int,
  pub buttons: XIButtonState,
  pub valuators: XIValuatorState,
  pub mods: XIModifierState,  // C 9615
}

/* C: <X11/Xlib.h>: `extern KeySym XLookupKeysym (XKeyEvent *, int);`, `extern Bool
   XGetEventData (Display *, XGenericEventCookie *);` and `extern void XFreeEventData (Display *,
   XGenericEventCookie *);` - C 9579/9619, C 9599 and C 9625.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XLookupKeysym (event: *mut XKeyEvent, index: c_int) -> KeySym;       // Xlib.h 1575
  pub fn XGetEventData (display: *mut Display,
			cookie: *mut XGenericEventCookie) -> Bool;            // Xlib.h 4009
  pub fn XFreeEventData (display: *mut Display,
			 cookie: *mut XGenericEventCookie);                   // Xlib.h 4014
}

/* C: <X11/Intrinsic.h>: `typedef struct _XtAppContextRec *XtAppContext;`, `extern Boolean
   XtAppPending (XtAppContext);`, `extern void XtAppNextEvent (XtAppContext, XEvent *);`,
   `typedef void (*XtCallbackProc) (Widget, XtPointer, XtPointer);`, `extern void XtAddCallback
   (Widget, const char *, XtCallbackProc, XtPointer);`, `extern void XtSetSensitive (Widget,
   Boolean);` and `extern void XtFree (char *);` - C 9571/9573, C 9504-9509, C 9528-9529 and
   C 9644.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type XtAppContext = *mut c_void;
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type XtCallbackProc = Option<unsafe extern "C" fn (Widget, XtPointer, XtPointer)>;
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe extern "C" {
  pub fn XtAppPending (app_context: XtAppContext) -> Boolean;
  pub fn XtAppNextEvent (app_context: XtAppContext, event_return: *mut XEvent);
  pub fn XtAddCallback (widget: Widget, name: *const c_char, callback: XtCallbackProc,
			client_data: XtPointer);
  pub fn XtSetSensitive (widget: Widget, sensitive: Boolean);
  pub fn XtFree (data: *mut c_char);
}

/* C: `src/xterm.h` 1788/1806/1855/1856/1993: the toolkit's application context, the dispatcher
   C 9630 hands the copy to, the two names `DEFER_SELECTIONS` (C 9483) expands to, and the
   waiter C 9569 calls.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_dispatch_event (event: *mut XEvent, display: *mut Display) -> c_int;  // xterm.h 1806
  pub fn x_defer_selection_requests ();                                       // xterm.h 1855
  pub fn x_release_selection_requests_and_flush ();                            // xterm.h 1856
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);  // lisp.h 4825
  pub fn x_menu_wait_for_event (data: *mut c_void);                           // xterm.h 1993
}
/* C: `xterm.h` 1787-1790: `extern XtAppContext Xt_app_con;`, inside that header's own
   `#ifdef USE_X_TOOLKIT` - the toolkit's application context C 9571/9573 pass to
   `XtAppPending`/`XtAppNextEvent`.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe extern "C" {
  pub static mut Xt_app_con: XtAppContext;                                    // xterm.h 1788
}

/* C: `math.h`'s `lrint` (C 9611-9614), `dispextern.h` 3879's `int popup_activated (void);`
   (C 9473), `frame.h` 1504's `extern WINDOW_SYSTEM_RETURN void check_window_system (struct
   frame *);` (C 9471) and `lisp.h` 4786's `extern Lisp_Object quit (void);` (C 9653).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn lrint (x: c_double) -> c_long;
                                         // dispextern.h 3879
                               // frame.h 1504
                                            // lisp.h 4786
}

/* C: `Xm/Xm.h`, `Xm/FileSB.h`, `Xm/TextF.h` and `Xm/List.h` (C 102-106): the opaque handle, the
   two scalars and the fourteen entry points C 9490-9642 spend.  **UNVERIFIED**: no Motif headers
   are installed here, so the prototypes are the headers' own text as `lwlib/lwlib-Xm.rs` records
   the same family, and the return/argument widths are that file's readings.  */
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub type XmString = *mut c_void;
/// C: Xm/Xm.h: `typedef String XmStringTag;`
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub type XmStringTag = String;
/// C: Xm/Xm.h: `typedef long XmTextPosition;`
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub type XmTextPosition = c_long;
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
unsafe extern "C" {
  /// C: Xm/Xm.h: `XmString XmStringCreateLocalized (String);` - C 9490/9491/9548.
  pub fn XmStringCreateLocalized (string: String) -> XmString;
  /// C: Xm/Xm.h: `void XmStringFree (XmString);` - C 9500/9501/9558.
  pub fn XmStringFree (string: XmString);
  /// C: Xm/FileSB.h: `Widget XmCreateFileSelectionDialog (Widget, String, ArgList, Cardinal);` -
  /// C 9498.
  pub fn XmCreateFileSelectionDialog (parent: Widget, name: String, args: *mut Arg,
				      n: c_uint) -> Widget;
  /// C: Xm/FileSB.h: `Widget XmFileSelectionBoxGetChild (Widget, unsigned char);` - C 9512/9516/
  /// 9523/9527/9538/9539.
  pub fn XmFileSelectionBoxGetChild (widget: Widget, child: c_uchar) -> Widget;
  /// C: Xm/TextF.h: `XmTextPosition XmTextFieldGetLastPosition (Widget);` - C 9541.
  pub fn XmTextFieldGetLastPosition (widget: Widget) -> XmTextPosition;
  /// C: Xm/TextF.h: `void XmTextFieldReplace (Widget, XmTextPosition, XmTextPosition, String);` -
  /// C 9542.
  pub fn XmTextFieldReplace (widget: Widget, from: XmTextPosition, to: XmTextPosition,
			     string: String);
  /// C: Xm/List.h: `Boolean XmListItemExists (Widget, XmString);` - C 9550.
  pub fn XmListItemExists (widget: Widget, item: XmString) -> Boolean;
  /// C: Xm/List.h: `int XmListItemPos (Widget, XmString);` - C 9552.
  pub fn XmListItemPos (widget: Widget, item: XmString) -> c_int;
  /// C: Xm/List.h: `void XmListSelectPos (Widget, int, Boolean);` - C 9554.
  pub fn XmListSelectPos (widget: Widget, position: c_int, notify: Boolean);
  /// C: Xm/List.h: `void XmListSetPos (Widget, int);` - C 9555.
  pub fn XmListSetPos (widget: Widget, position: c_int);
  /// C: Xm/Xm.h: `Boolean XmStringGetLtoR (XmString, XmStringTag, char **);` - C 9641.
  pub fn XmStringGetLtoR (string: XmString, tag: XmStringTag,
			  text: *mut *mut c_char) -> Boolean;
}

/* C: `Xm/Xm.h`'s resource names, the values C 9496/9497 hand `XtSetArg`, and `Xm/FileSB.h`'s
   child-type constants C 9512/9516/9523/9527/9538/9539 pass to `XmFileSelectionBoxGetChild`.
   C's spelling is kept; every value below is **UNVERIFIED** (no Motif headers here) and the
   child-type row is the header's documented order.  */
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNdirectory: &core::ffi::CStr = c"directory";
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNdialogStyle: &core::ffi::CStr = c"dialogStyle";
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNcancelCallback: &core::ffi::CStr = c"cancelCallback";
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNokCallback: &core::ffi::CStr = c"okCallback";
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNpattern: &core::ffi::CStr = c"pattern";
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNresizePolicy: &core::ffi::CStr = c"resizePolicy";
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNshowAsDefault: &core::ffi::CStr = c"showAsDefault";
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNtextString: &core::ffi::CStr = c"textString";
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNtitle: &core::ffi::CStr = c"title";
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmNunmapCallback: &core::ffi::CStr = c"unmapCallback";
/// C: Xm/Xm.h: `#define XmRESIZE_GROW 1` - C 9496's `XtSetArg` value.  **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmRESIZE_GROW: c_int = 1;
/// C: Xm/Xm.h: `#define XmDIALOG_APPLICATION_MODAL 3` (the dialog-style enum, whose documented
/// order is NONE 0, … `XmDIALOG_SYSTEM_MODAL` 1, `XmDIALOG_FULL_APPLICATION_MODAL` 2,
/// `XmDIALOG_APPLICATION_MODAL` 3) - C 9497's `XtSetArg` value.  **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmDIALOG_APPLICATION_MODAL: c_int = 3;
/// C: Xm/Xm.h: `#define XmCR_OK 21` - C 9635's `result` test, the constant pair of slice 39's
/// `XmCR_CANCEL`.  **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmCR_OK: c_int = 21;
/// C: Xm/Xm.h: `#define XmFONTLIST_DEFAULT_TAG "FONTLIST_DEFAULT_TAG_STRING"` - C 9641's
/// `XmStringGetLtoR` tag.  **UNVERIFIED**.
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmFONTLIST_DEFAULT_TAG: &core::ffi::CStr = c"FONTLIST_DEFAULT_TAG_STRING";
/// C: Xm/FileSB.h: `typedef unsigned char XmFileSelectionBoxChildType;` and the four members C
/// 9512/9516/9523/9527/9538/9539 asks for.  The values are the header's documented child order
/// (APPLY_BUTTON 0, CANCEL_BUTTON 1, DEFAULT_BUTTON 2, OK_BUTTON 3, FILTER_LABEL 4, FILTER_TEXT 5,
/// HELP_BUTTON 6, LIST 7, LIST_LABEL 8, MESSAGE_LABEL 9, SELECTION_LABEL 10, SEPARATOR 11,
/// TEXT 12, …) and are **UNVERIFIED** placeholders.
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmDIALOG_OK_BUTTON: c_uchar = 3;
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmDIALOG_HELP_BUTTON: c_uchar = 6;
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmDIALOG_LIST: c_uchar = 7;
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmDIALOG_SELECTION_LABEL: c_uchar = 10;
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
pub const XmDIALOG_TEXT: c_uchar = 12;

/* C 9456-9658.  */
#[cfg (all (feature = "have_x_windows", feature = "use_motif"))]
DEFUN! ("x-file-dialog", Fx_file_dialog, Sx_file_dialog, 2, 5, 0,              // C 9456
	doc: "SKIP: real doc in USE_GTK definition in xfns.c.",                   // C 9457
	(prompt: Lisp_Object, dir: Lisp_Object, default_filename: Lisp_Object,    // C 9458-9459
	 mustmatch: Lisp_Object, only_dir_p: Lisp_Object),
	{
  /* C's `dir` is written at C 9489 and the `DEFUN!` macro's `$arg:ident` cannot say `mut`.  */
  let mut dir = dir;                                                          // C 9489
  let mut result: c_int = 0;                                                  // C 9461
  let f: *mut frame = SELECTED_FRAME ();                                      // C 9462
  let mut file: Lisp_Object = Qnil ();                                        // C 9463
  let decoded_file: Lisp_Object;                                              // C 9464
  let dialog: Widget;                                                         // C 9465
  let text: Widget;
  let help: Widget;
  let mut al: [Arg; 10] = std::mem::zeroed ();                                // C 9466
  let mut ac: c_int = 0;                                                      // C 9467
  let dir_xmstring: XmString;                                                 // C 9468
  let pattern_xmstring: XmString;
  let count: specpdl_ref = SPECPDL_INDEX ();                                  // C 9469
  use crate::x11_constants::ControlMask;   // C 9582/9622 (X.h 223, the module's `c_long` row)

  check_window_system (f);                                                    // C 9471

  if popup_activated () != 0                                                  // C 9473
    { error (c"Trying to use a menu from within a menu-entry".as_ptr ()); }   // C 9474

  CHECK_STRING (prompt);                                                      // C 9476
  CHECK_STRING (dir);                                                         // C 9477

  /* Prevent redisplay.  */                                                   // C 9479
  specbind (Qinhibit_redisplay (), Qt ());                                    // C 9480

  /* Defer selection requests.  */                                            // C 9482
  DEFER_SELECTIONS! ();                                                       // C 9483

  block_input ();                                                             // C 9485

  /* Create the dialog with PROMPT as title, using DIR as initial
     directory and using "*" as pattern.  */                                  // C 9487-9488
  dir = Fexpand_file_name (dir, Qnil ());                                     // C 9489
  dir_xmstring = XmStringCreateLocalized (SSDATA (dir));                      // C 9490
  pattern_xmstring = XmStringCreateLocalized (c"*".as_ptr () as String);      // C 9491

  XtSetArg! (al[ac as usize], XmNtitle.as_ptr () as String, SDATA (prompt));
  ac += 1;                                                                    // C 9493
  XtSetArg! (al[ac as usize], XmNdirectory.as_ptr () as String, dir_xmstring);
  ac += 1;                                                                    // C 9494
  XtSetArg! (al[ac as usize], XmNpattern.as_ptr () as String, pattern_xmstring);
  ac += 1;                                                                    // C 9495
  XtSetArg! (al[ac as usize], XmNresizePolicy.as_ptr () as String, XmRESIZE_GROW);
  ac += 1;                                                                    // C 9496
  XtSetArg! (al[ac as usize], XmNdialogStyle.as_ptr () as String,
	     XmDIALOG_APPLICATION_MODAL);
  ac += 1;                                                                    // C 9497
  dialog = XmCreateFileSelectionDialog ((*((*f).output_data.x as *mut x_output)).widget,
					c"fsb".as_ptr () as String, al.as_mut_ptr (),
					ac as c_uint);                        // C 9498-9499
  XmStringFree (dir_xmstring);                                                // C 9500
  XmStringFree (pattern_xmstring);                                            // C 9501

  /* Add callbacks for OK and Cancel.  */                                     // C 9503
  XtAddCallback (dialog, XmNokCallback.as_ptr (), Some (file_dialog_cb),
		 &raw mut result as XtPointer);                               // C 9504-9505
  XtAddCallback (dialog, XmNcancelCallback.as_ptr (), Some (file_dialog_cb),
		 &raw mut result as XtPointer);                               // C 9506-9507
  XtAddCallback (dialog, XmNunmapCallback.as_ptr (), Some (file_dialog_unmap_cb),
		 &raw mut result as XtPointer);                               // C 9508-9509

  /* Remove the help button since we can't display help.  */                  // C 9511
  help = XmFileSelectionBoxGetChild (dialog, XmDIALOG_HELP_BUTTON);          // C 9512
  XtUnmanageChild (help);                                                     // C 9513

  /* Mark OK button as default.  */                                           // C 9515
  XtVaSetValues (XmFileSelectionBoxGetChild (dialog, XmDIALOG_OK_BUTTON),     // C 9516-9517
		 XmNshowAsDefault.as_ptr () as String, True,
		 std::ptr::null_mut::<c_void> ());

  /* If MUSTMATCH is non-nil, disable the file entry field of the
     dialog, so that the user must select a file from the files list
     box.  We can't remove it because we wouldn't have a way to get at
     the result file name, then.  */                                         // C 9519-9522
  text = XmFileSelectionBoxGetChild (dialog, XmDIALOG_TEXT);                  // C 9523
  if !NILP (mustmatch)                                                        // C 9524
    {
      let label: Widget;                                                      // C 9526
      label = XmFileSelectionBoxGetChild (dialog, XmDIALOG_SELECTION_LABEL);  // C 9527
      XtSetSensitive (text, False);                                           // C 9528
      XtSetSensitive (label, False);                                          // C 9529
    }

  /* Manage the dialog, so that list boxes get filled.  */                     // C 9532
  XtManageChild (dialog);                                                     // C 9533

  if STRINGP (default_filename)                                               // C 9535
    {
      let default_xmstring: XmString;                                         // C 9537
      let wtext: Widget = XmFileSelectionBoxGetChild (dialog, XmDIALOG_TEXT);  // C 9538
      let list: Widget = XmFileSelectionBoxGetChild (dialog, XmDIALOG_LIST);  // C 9539

      let last_pos: XmTextPosition = XmTextFieldGetLastPosition (wtext);      // C 9541
      XmTextFieldReplace (wtext, 0, last_pos,
			  SSDATA (Ffile_name_nondirectory (default_filename)));  // C 9542-9543

      /* Select DEFAULT_FILENAME in the files list box.  DEFAULT_FILENAME
         must include the path for this to work.  */                         // C 9545-9546

      default_xmstring = XmStringCreateLocalized (SSDATA (default_filename));  // C 9548

      if XmListItemExists (list, default_xmstring) != 0                       // C 9550
	{
	  let item_pos: c_int = XmListItemPos (list, default_xmstring);       // C 9552
	  /* Select the item and scroll it into view.  */                     // C 9553
	  XmListSelectPos (list, item_pos, True);                             // C 9554
	  XmListSetPos (list, item_pos);                                      // C 9555
	}

      XmStringFree (default_xmstring);                                        // C 9558
    }

  record_unwind_protect_ptr (Some (clean_up_file_dialog), dialog);            // C 9561

  /* Process events until the user presses Cancel or OK.  */                   // C 9563
  x_menu_set_in_use (true);                                                   // C 9564
  result = 0;                                                                 // C 9565
  while result == 0                                                           // C 9566
    {
      let mut event: XEvent = std::mem::zeroed ();                            // C 9568
      let mut copy: XEvent = std::mem::zeroed ();
      x_menu_wait_for_event (std::ptr::null_mut ());                          // C 9569

      if XtAppPending (Xt_app_con) != 0                                       // C 9571
	{
	  XtAppNextEvent (Xt_app_con, &raw mut event);                        // C 9573

	  core::ptr::copy_nonoverlapping (&raw const event, &raw mut copy, 1);  // C 9575
	  let xany: *mut XAnyEvent = &raw mut event as *mut XAnyEvent;
	  let xkey: *mut XKeyEvent = &raw mut event as *mut XKeyEvent;
	  if (*xany).r#type == KeyPress                                       // C 9576
	      && FRAME_X_DISPLAY! (f) == (*xkey).display
	    {
	      let keysym: KeySym = XLookupKeysym (xkey, 0);                   // C 9579

	      /* Pop down on C-g.  */                                         // C 9581
	      if keysym == XK_g                                               // C 9582
		  && ((*xkey).state & (ControlMask as c_uint)) != 0
		{ XtUnmanageChild (dialog); }                                 // C 9583
	    }
	  else
	    {
	      /* C's `else if` is C 9586, the first statement of C's else arm, and C's whole arm
		 sits inside `#ifdef HAVE_XINPUT2` (C 9585/C 9628); an attribute is not allowed on
		 an `else if` clause, so the arm is the else block's one statement.  */
	      #[cfg (feature = "have_xinput2")]
	      let xgeneric: *mut XGenericEvent = &raw mut event as *mut XGenericEvent;
	      #[cfg (feature = "have_xinput2")]
	      let xcookie: *mut XGenericEventCookie
		= &raw mut event as *mut XGenericEventCookie;
	      #[cfg (feature = "have_xinput2")]
	      if (*xany).r#type == GenericEvent                               // C 9586
		  && FRAME_X_DISPLAY! (f) == (*xgeneric).display
		  && (*FRAME_DISPLAY_INFO! (f)).supports_xi2
		  && ((*xgeneric).extension == (*FRAME_DISPLAY_INFO! (f)).xi2_opcode)
		  && (*xgeneric).evtype == XI_KeyPress
		{
		  let mut keysym: KeySym;                                     // C 9593
		  let mut xev: *mut XIDeviceEvent;                            // C 9594

		  if !(*xcookie).data.is_null ()                              // C 9596
		    { emacs_abort (); }                                       // C 9597

		  if XGetEventData (FRAME_X_DISPLAY! (f), xcookie) != 0      // C 9599
		    {
		      xev = (*xcookie).data as *mut XIDeviceEvent;            // C 9601

		      let xkey: *mut XKeyEvent = &raw mut copy as *mut XKeyEvent;  // C 9603-9617
		      (*xkey).r#type = KeyPress;
		      (*xkey).serial = (*xev).serial;
		      (*xkey).send_event = (*xev).send_event;
		      (*xkey).display = FRAME_X_DISPLAY! (f);
		      (*xkey).window = (*xev).event;
		      (*xkey).root = (*xev).root;
		      (*xkey).subwindow = (*xev).child;
		      (*xkey).time = (*xev).time;
		      (*xkey).x = lrint ((*xev).event_x) as c_int;
		      (*xkey).y = lrint ((*xev).event_y) as c_int;
		      (*xkey).x_root = lrint ((*xev).root_x) as c_int;
		      (*xkey).y_root = lrint ((*xev).root_y) as c_int;
		      (*xkey).state = (*xev).mods.effective as c_uint;
		      (*xkey).keycode = (*xev).detail as c_uint;
		      (*xkey).same_screen = True;

		      keysym = XLookupKeysym (xkey, 0);                         // C 9619

		      if keysym == XK_g                                         // C 9621-9622
			  && ((*xkey).state & (ControlMask as c_uint)) != 0
			{ XtUnmanageChild (dialog); }                         // C 9623

		      XFreeEventData (FRAME_X_DISPLAY! (f), xcookie);            // C 9625
		    }
		}
	    }

	  /* C 9630's `(void)` cast: the result is dropped.  */
	  x_dispatch_event (&raw mut copy, FRAME_X_DISPLAY! (f));            // C 9630
	}
    }

  /* Get the result.  */                                                      // C 9634
  if result == XmCR_OK                                                        // C 9635
    {
      let mut text_string: XmString = std::ptr::null_mut ();                  // C 9637
      let mut data: *mut c_char = std::ptr::null_mut ();                      // C 9638

      XtVaGetValues (dialog, XmNtextString.as_ptr () as String,               // C 9640
		     &raw mut text_string, std::ptr::null_mut::<c_void> ());
      XmStringGetLtoR (text_string, XmFONTLIST_DEFAULT_TAG.as_ptr () as XmStringTag,
		       &raw mut data);                                        // C 9641
      XmStringFree (text_string);                                             // C 9642
      file = build_string (data);                                             // C 9643
      XtFree (data);                                                          // C 9644
    }
  else
    { file = Qnil (); }                                                       // C 9646-9647

  unblock_input ();                                                           // C 9649

  /* Make "Cancel" equivalent to C-g.  */                                     // C 9651
  if NILP (file)                                                              // C 9652
    { quit (); }                                                              // C 9653

  decoded_file = DECODE_FILE (file);                                          // C 9655

  return unbind_to (count, decoded_file);                                     // C 9657
});


/* ==============================================================================================
   C 9660-9733: `#endif /* USE_MOTIF */` (C 9660), `#ifdef USE_GTK` (C 9662) and the two items that
   guard holds - `clean_up_dialog` (C 9664-9668) and the file's **second** `x-file-dialog`
   (C 9670-9733), whose docstring is the real one C 9457's Motif `DEFUN` delegates to ("SKIP: real
   doc in USE_GTK definition in xfns.c.").  `#endif /* USE_GTK */` is C 9787, so the rest of the
   region (`x-select-font`, C 9736-9785, under `#ifdef HAVE_FREETYPE`) is the remainder's.

   The GTK arm compiles this region and the `have_gtk` gate is C's own; the one row this slice
   moves is `x_menu_set_in_use`'s, which C declares under `#if defined USE_GTK || defined
   USE_MOTIF` (xterm.h 1990-1991) - slice 39 gated it `use_motif` because that was the only arm
   reading it, and C 9667/C 9698 read it under `use_gtk`.  */

/* C: `gtkutil.h` 88-92: `extern char *xg_get_file_name (struct frame *f, char *prompt, char
   *default_filename, bool mustmatch_p, bool only_dir_p);` - C 9714's call.  `src/gtkutil.rs`
   defines it and no mirror carries the row, so it stands at the use site (P-55/P-72 rule 3, the
   `xg_uses_old_file_dialog` row of slice 38).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn xg_get_file_name (f: *mut frame, prompt: *mut c_char, default_filename: *mut c_char,
			   mustmatch_p: bool, only_dir_p: bool) -> *mut c_char;
}

/* C 9664-9668.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" fn clean_up_dialog ()
{
  x_menu_set_in_use (false);                                                 // C 9667
}

/* C 9670-9733.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
DEFUN! ("x-file-dialog", Fx_file_dialog, Sx_file_dialog, 2, 5, 0,              // C 9670
	doc: "Read file name, prompting with PROMPT in directory DIR.\nUse a file selection dialog.  Select DEFAULT-FILENAME in the dialog's file\nselection box, if specified.  If MUSTMATCH is non-nil, the returned file\nor directory must exist.\n\nThis function is defined only on PGTK, NS, Haiku, MS Windows, and X Windows with\nthe Motif or Gtk toolkits.  With the Motif toolkit, ONLY-DIR-P is ignored.\nOtherwise, if ONLY-DIR-P is non-nil, the user can select only directories.\n\nOn MS Windows 7 and later, the file selection dialog \"remembers\" the last\ndirectory where the user selected a file, and will open that directory\ninstead of DIR on subsequent invocations of this function with the same\nvalue of DIR as in previous invocations; this is standard MS Windows behavior.",  // C 9671-9683
	(prompt: Lisp_Object, dir: Lisp_Object, default_filename: Lisp_Object,
	 mustmatch: Lisp_Object, only_dir_p: Lisp_Object),                     // C 9684
	{
  let f: *mut frame = SELECTED_FRAME ();                                      // C 9686
  let mut fn_: *mut c_char;                                                   // C 9687
  let mut file: Lisp_Object = Qnil ();                                        // C 9688
  let decoded_file: Lisp_Object;                                              // C 9689
  let count: specpdl_ref = SPECPDL_INDEX ();                                  // C 9690
  let mut cdef_file: *mut c_char;                                             // C 9691

  check_window_system (f);                                                    // C 9693

  if popup_activated () != 0                                                  // C 9695
    { error (c"Trying to use a menu from within a menu-entry".as_ptr ()); }   // C 9696
  else
    { x_menu_set_in_use (true); }                                             // C 9697-9698

  CHECK_STRING (prompt);                                                      // C 9700
  CHECK_STRING (dir);                                                         // C 9701

  /* Prevent redisplay.  */                                                   // C 9703
  specbind (Qinhibit_redisplay (), Qt ());                                    // C 9704
  record_unwind_protect_void (Some (clean_up_dialog));                        // C 9705

  block_input ();                                                             // C 9707

  if STRINGP (default_filename)                                               // C 9709
    { cdef_file = SSDATA (default_filename); }                                // C 9710
  else
    { cdef_file = SSDATA (dir); }                                             // C 9712

  fn_ = xg_get_file_name (f, SSDATA (prompt), cdef_file,                      // C 9714
			  !NILP (mustmatch), !NILP (only_dir_p));             // C 9715-9716

  if !fn_.is_null ()                                                          // C 9718
    {
      file = build_string (fn_);                                              // C 9720
      xfree (fn_ as *mut c_void);                                             // C 9721
    }

  unblock_input ();                                                           // C 9724

  /* Make "Cancel" equivalent to C-g.  */                                     // C 9726
  if NILP (file)                                                              // C 9727
    { quit (); }                                                              // C 9728

  decoded_file = DECODE_FILE (file);                                          // C 9730

  return unbind_to (count, decoded_file);                                     // C 9732
});


/* ==============================================================================================
   C 9736-9785: `#ifdef HAVE_FREETYPE` inside C 9662's `#ifdef USE_GTK` - `x-select-font`, the
   file's second GTK dialog and the GTK region's last item (`#endif /* USE_GTK */` is C 9787).

   Two of the names it reads are already in this unit's graph: `clean_up_dialog` (C 9760, slice
   41) and `XSETFONT!` (`include/font.rs` 275, C 9764's assignment, C's `font.h` 552).  The third,
   `xg_get_font`, is `gtkutil.h`'s and stands at its use site (P-55/P-72 rule 3, the
   `xg_get_file_name` shape above); `xlispstrdup`'s row moves to the file gate here, because
   C's `lisp.h` 5553 declares it unconditionally and slice 9 gated it `use_x_toolkit` when that
   was its only arm.  */

/* C: `gtkutil.h` 94: `extern Lisp_Object xg_get_font (struct frame *f, char *);` - C 9775's
   call; `src/gtkutil.rs` defines it and no mirror carries the row.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk", feature = "freetype"))]
unsafe extern "C" {
  pub fn xg_get_font (f: *mut frame, default_name: *mut c_char) -> Lisp_Object;
}

/* C 9738-9784.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk", feature = "freetype"))]
DEFUN! ("x-select-font", Fx_select_font, Sx_select_font, 0, 2, 0,              // C 9738
	doc: "Read a font using a GTK dialog.\nReturn either a font spec (for GTK versions >= 3.2) or a string\ncontaining a GTK-style font name.\n\nFRAME is the frame on which to pop up the font chooser.  If omitted or\nnil, it defaults to the selected frame.",  // C 9739-9744
	(frame: Lisp_Object, ignored: Lisp_Object),                               // C 9745
	{
  let f: *mut frame = decode_window_system_frame (frame);                     // C 9747
  let mut font: Lisp_Object;                                                  // C 9748
  let mut font_param: Lisp_Object;                                            // C 9749
  let mut default_name: *mut c_char = std::ptr::null_mut ();                  // C 9750
  let count: specpdl_ref = SPECPDL_INDEX ();                                  // C 9751

  if popup_activated () != 0                                                  // C 9753
    { error (c"Trying to use a menu from within a menu-entry".as_ptr ()); }   // C 9754
  else
    { x_menu_set_in_use (true); }                                             // C 9755-9756

  /* Prevent redisplay.  */                                                   // C 9758
  specbind (Qinhibit_redisplay (), Qt ());                                    // C 9759
  record_unwind_protect_void (Some (clean_up_dialog));                        // C 9760

  block_input ();                                                             // C 9762

  XSETFONT! (font, FRAME_FONT! (f));                                          // C 9764
  font_param = Ffont_get (font, QCname ());                                   // C 9765
  if STRINGP (font_param)                                                     // C 9766
    { default_name = xlispstrdup (font_param); }                              // C 9767
  else
    {
      font_param = Fframe_parameter (frame, Qfont_parameter ());              // C 9770
      if STRINGP (font_param)                                                 // C 9771
	{ default_name = xlispstrdup (font_param); }                          // C 9772
    }

  font = xg_get_font (f, default_name);                                       // C 9775
  xfree (default_name as *mut c_void);                                        // C 9776

  unblock_input ();                                                           // C 9778

  if NILP (font)                                                              // C 9780
    { quit (); }                                                              // C 9781

  return unbind_to (count, font);                                             // C 9783
});


/* ==============================================================================================
   C 9787-9893: `#endif /* USE_GTK */` (C 9787), the `Keyboard` banner (C 9790-9792), C 9794's
   `#ifdef HAVE_XKB` pair of includes, `x-backspace-delete-keys-p` (C 9799-9874) and
   `x-get-modifier-masks` (C 9876-9893), which stands between it and the `Printing` banner
   (C 9895-9898).

   The XKB surface C 9799 reads is `<X11/XKBlib.h>`'s (C 9795), whose records
   `include/xterm.rs` 708-711 deliberately leaves as `*mut c_void` (`xkb_desc`), so the three
   records C 9809/9846-9859 names stand at their use site - **measured on this machine's
   xorgproto headers** (`X11/extensions/XKBstr.h` with `XKB.h`'s three sizes), the
   `docs/ports/xfns.md` Evidence ② reading.  The `have_xkb` feature is the tree's registered
   spelling of C's `HAVE_XKB` (`src/xterm.rs` 1498/1501, 20818).  */

/* C: X11/extensions/XKBstr.h 404-406: `typedef struct _XkbKeyNameRec { char
   name[XkbKeyNameLength] _X_NONSTRING; } XkbKeyNameRec, *XkbKeyNamePtr;` with XKB.h 564's
   `#define XkbKeyNameLength 4` - C 9857/9859's `keys[i].name`; measured: `sizeof` **4**,
   `name`@0.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XkbKeyNameRec
{
  pub name: [c_char; 4],      // C: char name[XkbKeyNameLength];
}
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
pub type XkbKeyNamePtr = *mut XkbKeyNameRec;

/* C: X11/extensions/XKBstr.h 416-437: `typedef struct _XkbNamesRec { Atom keycodes; Atom
   geometry; Atom symbols; Atom types; Atom compat; Atom vmods[XkbNumVirtualMods]; Atom
   indicators[XkbNumIndicators]; Atom groups[XkbNumKbdGroups]; XkbKeyNamePtr keys; XkbKeyAliasPtr
   key_aliases; Atom *radio_groups; Atom phys_symbols; unsigned char num_keys; unsigned char
   num_key_aliases; unsigned short num_rg; } XkbNamesRec, *XkbNamesPtr;` with XKB.h 555/556/348's
   16, 32 and 4 - C 9846-9859 read `keys` only; measured: `sizeof` **496**, `keycodes`@0,
   `keys`@456.  `key_aliases`' pointee is not read by this unit, so its pointer stands as
   `*mut c_void` (the layout is a pointer either way).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XkbNamesRec
{
  pub keycodes: Atom,                     // C: Atom keycodes;
  pub geometry: Atom,
  pub symbols: Atom,
  pub types: Atom,
  pub compat: Atom,
  pub vmods: [Atom; 16],                  // C: Atom vmods[XkbNumVirtualMods];
  pub indicators: [Atom; 32],             // C: Atom indicators[XkbNumIndicators];
  pub groups: [Atom; 4],                  // C: Atom groups[XkbNumKbdGroups];
  pub keys: XkbKeyNamePtr,                // C: XkbKeyNamePtr keys;
  pub key_aliases: *mut c_void,           // C: XkbKeyAliasPtr key_aliases;
  pub radio_groups: *mut Atom,            // C: Atom *radio_groups;
  pub phys_symbols: Atom,                 // C: Atom phys_symbols;
  pub num_keys: c_uchar,                  // C: unsigned char num_keys;
  pub num_key_aliases: c_uchar,
  pub num_rg: c_ushort,                   // C: unsigned short num_rg;
}
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
pub type XkbNamesPtr = *mut XkbNamesRec;

/* C: X11/extensions/XKBstr.h 444-457: `typedef struct _XkbDesc { struct _XDisplay *dpy;
   unsigned short flags; unsigned short device_spec; KeyCode min_key_code; KeyCode
   max_key_code; XkbControlsPtr ctrls; XkbServerMapPtr server; XkbClientMapPtr map;
   XkbIndicatorPtr indicators; XkbNamesPtr names; XkbCompatMapPtr compat; XkbGeometryPtr geom; }
   XkbDescRec, *XkbDescPtr;` - C 9809/9846-9865 read `min_key_code`, `max_key_code` and `names`;
   measured: `sizeof` **72**, `dpy`@0, `flags`@8, `device_spec`@10, `min_key_code`@12,
   `max_key_code`@13, `names`@48.  The five records between `max_key_code` and `geom` are not
   read by this unit, so their pointers stand as `*mut c_void`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XkbDescRec
{
  pub dpy: *mut Display,                  // C: struct _XDisplay *dpy;
  pub flags: c_ushort,                    // C: unsigned short flags;
  pub device_spec: c_ushort,
  pub min_key_code: KeyCode,              // C: KeyCode min_key_code;
  pub max_key_code: KeyCode,
  pub ctrls: *mut c_void,                 // C: XkbControlsPtr ctrls;
  pub server: *mut c_void,
  pub map: *mut c_void,
  pub indicators: *mut c_void,
  pub names: XkbNamesPtr,                 // C: XkbNamesPtr names;
  pub compat: *mut c_void,
  pub geom: *mut c_void,
}
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
pub type XkbDescPtr = *mut XkbDescRec;
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
const _: () = {
  core::assert! (core::mem::size_of::<XkbKeyNameRec> () == 4);
  core::assert! (core::mem::offset_of! (XkbKeyNameRec, name) == 0);
  core::assert! (core::mem::size_of::<XkbNamesRec> () == 496);
  core::assert! (core::mem::offset_of! (XkbNamesRec, keys) == 456);
  core::assert! (core::mem::size_of::<XkbDescRec> () == 72);
  core::assert! (core::mem::offset_of! (XkbDescRec, dpy) == 0);
  core::assert! (core::mem::offset_of! (XkbDescRec, flags) == 8);
  core::assert! (core::mem::offset_of! (XkbDescRec, device_spec) == 10);
  core::assert! (core::mem::offset_of! (XkbDescRec, min_key_code) == 12);
  core::assert! (core::mem::offset_of! (XkbDescRec, max_key_code) == 13);
  core::assert! (core::mem::offset_of! (XkbDescRec, names) == 48);
};

/* C: `<X11/keysymdef.h>`: the two keysyms C 9864-9865 pass `XKeysymToKeycode`, measured on this
   machine's header.  */
/// C: X11/keysymdef.h 200: `#define XK_BackSpace 0xff08`
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
pub const XK_BackSpace: KeySym = 0xff08;
/// C: X11/keysymdef.h 209: `#define XK_Delete 0xffff`
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
pub const XK_Delete: KeySym = 0xffff;

/* C: `<X11/Xlib.h>`: `extern KeyCode XKeysymToKeycode (Display *, KeySym);` - C 9864-9865's two
   calls; the mirror carries no row for it.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xkb"))]
unsafe extern "C" {
  pub fn XKeysymToKeycode (display: *mut Display, keysym: KeySym) -> KeyCode;   // Xlib.h 1687
}

/* C: `xterm.h` 1969: `extern Lisp_Object x_get_keyboard_modifiers (struct x_display_info *);` -
   C 9892's call; `src/xterm.c` 32643 defines it and no mirror carries the row (P-55/P-72 rule 3).
   */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_get_keyboard_modifiers (dpyinfo: *mut x_display_info) -> Lisp_Object;   // xterm.h 1969
}


/* C 9799-9874.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-backspace-delete-keys-p", Fx_backspace_delete_keys_p,
	Sx_backspace_delete_keys_p, 0, 1, 0,                                   // C 9799-9800
	doc: "Check if both Backspace and Delete keys are on the keyboard of FRAME.\nFRAME nil means use the selected frame.\nValue is t if we know that both keys are present, and are mapped to the\nusual X keysyms.  Value is `lambda' if we cannot determine if both keys are\npresent and mapped to the usual X keysyms.",  // C 9801-9805
	(frame: Lisp_Object),                                                  // C 9806
	{
  /* C 9808-9814's `#ifdef HAVE_XKB` declarations and C 9816-9873's `#ifndef HAVE_XKB` / `#else`
     pair as two cfg blocks, in C's own order (the `HAVE_XDBE` device of slice 38).  */
  #[cfg (not (feature = "have_xkb"))]                                      // C 9816-9817
  {
    return Qlambda ();                                                     // C 9817
  }
  #[cfg (feature = "have_xkb")]                                            // C 9808-9814, C 9818-9873
  {
    let kb: XkbDescPtr;                                                    // C 9809
    let f: *mut frame;                                                     // C 9810
    let dpy: *mut Display;                                                 // C 9811
    let mut have_keys: Lisp_Object;                                        // C 9812
    let mut delete_keycode: c_int;                                         // C 9813
    let mut backspace_keycode: c_int;
    let mut i: c_int;

    delete_keycode = 0;                                                    // C 9819
    backspace_keycode = 0;                                                 // C 9820
    f = decode_window_system_frame (frame);                                // C 9821
    dpy = FRAME_X_DISPLAY! (f);                                            // C 9822

    if !(*FRAME_DISPLAY_INFO! (f)).supports_xkb                            // C 9824
      { return Qlambda (); }                                               // C 9825

    block_input ();                                                        // C 9827

    /* In this code we check that the keyboard has physical keys with names
       that start with BKSP (Backspace) and DELE (Delete), and that they
       generate keysym XK_BackSpace and XK_Delete respectively.
       This function is used to test if normal-erase-is-backspace should be
       turned on.
       An alternative approach would be to just check if XK_BackSpace and
       XK_Delete are mapped to any key.  But if any of those are mapped to
       some non-intuitive key combination (Meta-Shift-Ctrl-whatever) and the
       user doesn't know about it, it is better to return false here.
       It is more obvious to the user what to do if there are two keys
       clearly marked with names/symbols and one key does something not
       expected (and the user then tries the other).
       The cases where Backspace/Delete is mapped to some other key combination
       are rare, and in those cases, normal-erase-is-backspace can be turned on
       manually.  */                                                      // C 9829-9843

    have_keys = Qnil ();                                                   // C 9845
    /* C 9846's `FRAME_DISPLAY_INFO (f)->xkb_desc` is the mirror's `*mut c_void` row, so the
       assignment states the cast C's typedef gives it for free.  */
    kb = (*FRAME_DISPLAY_INFO! (f)).xkb_desc as XkbDescPtr;
    if !kb.is_null () && !(*kb).names.is_null ()                           // C 9847
      {
	i = (*kb).min_key_code as c_int;                                   // C 9849
	while i < (*kb).max_key_code as c_int
	    && (delete_keycode == 0 || backspace_keycode == 0)             // C 9849-9851
	  {
	    /* The XKB symbolic key names can be seen most easily in
	       the PS file generated by `xkbprint -label name
	       $DISPLAY'.  */                                              // C 9854-9856
	    if memcmp (c"DELE".as_ptr () as *const c_void,
		       (*((*(*kb).names).keys.offset (i as isize))).name.as_ptr ()
		       as *const c_void, 4) == 0                           // C 9857
	      { delete_keycode = i; }                                      // C 9858
	    else if memcmp (c"BKSP".as_ptr () as *const c_void,
			    (*((*(*kb).names).keys.offset (i as isize))).name.as_ptr ()
			    as *const c_void, 4) == 0                      // C 9859
	      { backspace_keycode = i; }                                   // C 9860
	    i += 1;                                                        // C 9852
	  }

	if delete_keycode != 0 && backspace_keycode != 0                   // C 9863
	    && XKeysymToKeycode (dpy, XK_Delete) as c_int == delete_keycode  // C 9864
	    && XKeysymToKeycode (dpy, XK_BackSpace) as c_int == backspace_keycode  // C 9865
	  { have_keys = Qt (); }                                           // C 9866
      }
    else
      /* The keyboard names couldn't be obtained for some reason.  */      // C 9869
      { have_keys = Qlambda (); }                                          // C 9870

    unblock_input ();                                                      // C 9871
    return have_keys;                                                      // C 9872
  }
});

/* C 9876-9893.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-get-modifier-masks", Fx_get_modifier_masks, Sx_get_modifier_masks,
	0, 1, 0,                                                               // C 9876-9877
	doc: "Return the X modifier masks corresponding to keyboard modifiers.\nThe optional second argument TERMINAL specifies which display to fetch\nmodifier masks from.  TERMINAL should be a terminal object, a frame or\na display name (a string).  If TERMINAL is omitted or nil, that stands\nfor the selected frame's display.\n\nReturn a list of (HYPER SUPER ALT SHIFT-LOCK META), each element being\na number describing the modifier mask for the corresponding Emacs\nmodifier.",  // C 9878-9886
	(terminal: Lisp_Object),                                               // C 9887
	{
  let dpyinfo: *mut x_display_info;                                            // C 9889

  dpyinfo = check_x_display_info (terminal);                                   // C 9891
  return x_get_keyboard_modifiers (dpyinfo);                                   // C 9892
});


/* ==============================================================================================
   C 9895-10045: the `Printing` banner (C 9896-9898), C 9900's `#ifdef USE_CAIRO` with
   `x-export-frames` (C 9901-9964) and C 9966's `#ifdef USE_GTK` with the three print dialogs
   (`x-page-setup-dialog`, C 9967-9977; `x-get-page-setup`, C 9979-10006; `x-print-frames-dialog`,
   C 10008-10043), closing at C 10044/C 10045.  The whole `USE_CAIRO` region ends here.

   C 9932-9961's four `#ifdef`s each wrap one `if (…) …; else`, so the chain C compiles holds
   exactly the arms whose cairo feature is defined and ends in C 9961's `error`; Rust cannot let
   the `else` of one `#[cfg]` arm be supplied by the next, so each arm's presence is the
   compile-time `cfg!` of its feature and C's chain follows once, in C's order - the arm-chooser
   idiom `src/pgtkfns.rs` 1874-1907 carries for the same C text (`pgtk-export-frames`), recorded
   in `docs/ports/gtkutil.md`'s Deviation 4.  The `use_cairo` feature is the tree's registered
   spelling of C's `USE_CAIRO`; the four cairo features are `cairo_has_pdf_surface`,
   `cairo_has_png_functions`, `cairo_has_ps_surface` and `cairo_has_svg_surface`.

   This slice also carries the two rows the `use_cairo` dimension had hidden: `use_cairo` had no
   arm until status 3.949, and with the **CAIRO** arm the file read 50 / 20 against the GTK arm's
   47 / 20 - `x_cr_destroy_frame_context` (xterm.h 1811) was undeclared at C 3990/4033 and
   `x_cr_update_surface_desired_size` (xterm.h 1812) was declared twice (its file-gate row and the
   `use_cairo` row below).  Both are fixed here.  */

/* C: `xterm.h` 1818: `extern Lisp_Object x_cr_export_frames (Lisp_Object,
   cairo_surface_type_t);` - C 9963's call; `src/xterm.c` defines it and no mirror carries the row
   (P-55/P-72 rule 3).  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe extern "C" {
  pub fn x_cr_export_frames (frames: Lisp_Object,
			     surface_type: cairo_surface_type_t) -> Lisp_Object;   // xterm.h 1818
}

/* C 9901-9964.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
DEFUN! ("x-export-frames", Fx_export_frames, Sx_export_frames, 0, 2, 0,        // C 9901
	doc: "Return image data of FRAMES in TYPE format.\nFRAMES should be nil (the selected frame), a frame, or a list of\nframes (each of which corresponds to one page).  Each frame should be\nvisible.  Optional arg TYPE should be either `pdf' (default), `png',\n`postscript', or `svg'.  Supported types are determined by the\ncompile-time configuration of cairo.\n\nNote: Text drawn with the `x' font backend is shown with hollow boxes\nunless TYPE is `png'.",  // C 9902-9910
	(frames: Lisp_Object, r#type: Lisp_Object),                            // C 9911
	{
  /* C's `frames` is reassigned on the next line and the `DEFUN!` macro's `$arg:ident` cannot say
     `mut`.  */
  let mut frames = frames;
  let mut rest: Lisp_Object;                                                   // C 9913
  let mut tmp: Lisp_Object;
  let surface_type: cairo_surface_type_t;                                      // C 9914

  if !CONSP (frames)                                                           // C 9916
    { frames = list1 (frames); }                                               // C 9917

  tmp = Qnil ();                                                               // C 9919
  /* C 9920's `for (rest = frames; CONSP (rest); rest = XCDR (rest))`: init, test, body, then the
     increment C writes in the loop head (the `src/pgtkfns.rs` 1861-1872 shape).  */
  rest = frames;
  while CONSP (rest)
    {
      let f: *mut frame = decode_window_system_frame (XCAR (rest));            // C 9922
      let mut frame: Lisp_Object;                                              // C 9923

      XSETFRAME! (frame, f);                                                   // C 9925
      if !FRAME_VISIBLE_P (f)                                                  // C 9926
	{ error (c"Frames to be exported must be visible".as_ptr ()); }        // C 9927
      tmp = Fcons (frame, tmp);                                                // C 9928
      rest = XCDR (rest);
    }
  frames = Fnreverse (tmp);                                                    // C 9930

  /* C 9932-9961's four `#ifdef`s as one chain of `cfg!` guards (see the banner above).  */
  if cfg! (feature = "cairo_has_pdf_surface") && (NILP (r#type) || EQ (r#type, Qpdf ()))
    { surface_type = CAIRO_SURFACE_TYPE_PDF; }                                 // C 9933-9934
  else if cfg! (feature = "cairo_has_png_functions") && EQ (r#type, Qpng ())   // C 9937-9938
    {
      if !NILP (XCDR (frames))                                                 // C 9940
	{ error (c"PNG export cannot handle multiple frames".as_ptr ()); }     // C 9941
      surface_type = CAIRO_SURFACE_TYPE_IMAGE;                                 // C 9942
    }
  else if cfg! (feature = "cairo_has_ps_surface") && EQ (r#type, Qpostscript ())  // C 9946-9947
    { surface_type = CAIRO_SURFACE_TYPE_PS; }                                  // C 9948
  else if cfg! (feature = "cairo_has_svg_surface") && EQ (r#type, Qsvg ())     // C 9951-9952
    {
      /* For now, we stick to SVG 1.1.  */                                     // C 9954
      if !NILP (XCDR (frames))                                                 // C 9955
	{ error (c"SVG export cannot handle multiple frames".as_ptr ()); }     // C 9956
      surface_type = CAIRO_SURFACE_TYPE_SVG;                                   // C 9957
    }
  else
    { error (c"Unsupported export type".as_ptr ()); }                          // C 9961

  return x_cr_export_frames (frames, surface_type);                            // C 9963
});

/* C: `gtkutil.h` 219-221: `extern void xg_page_setup_dialog (void);`, `extern Lisp_Object
   xg_get_page_setup (void);` and `extern void xg_print_frames_dialog (Lisp_Object);` - C
   9973/10002/10039's calls; `src/gtkutil.rs` defines all three and no mirror carries the rows.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn xg_page_setup_dialog ();                                              // gtkutil.h 219
  pub fn xg_get_page_setup () -> Lisp_Object;                                  // gtkutil.h 220
  pub fn xg_print_frames_dialog (frames: Lisp_Object);                          // gtkutil.h 221
}

/* C: `dispextern.h` 3589: `void redisplay_preserve_echo_area (int);` - C 10036's call; the
   mirror carries no row.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo", feature = "use_gtk"))]

/* C 9967-9977.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo", feature = "use_gtk"))]
DEFUN! ("x-page-setup-dialog", Fx_page_setup_dialog, Sx_page_setup_dialog, 0, 0, 0,  // C 9967
	doc: "Pop up a page setup dialog.\nThe current page setup can be obtained using `x-get-page-setup'.",  // C 9968-9969
	(),                                                                    // C 9970
	{
  block_input ();                                                              // C 9972
  xg_page_setup_dialog ();                                                     // C 9973
  unblock_input ();                                                            // C 9974

  return Qnil ();                                                              // C 9976
});

/* C 9979-10006.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo", feature = "use_gtk"))]
DEFUN! ("x-get-page-setup", Fx_get_page_setup, Sx_get_page_setup, 0, 0, 0,     // C 9979
	doc: "Return the value of the current page setup.\nThe return value is an alist containing the following keys:\n\n  orientation: page orientation (symbol `portrait', `landscape',\n\t`reverse-portrait', or `reverse-landscape').\n  width, height: page width/height in points not including margins.\n  left-margin, right-margin, top-margin, bottom-margin: print margins,\n\twhich is the parts of the page that the printer cannot print\n\ton, in points.\n\nThe paper width can be obtained as the sum of width, left-margin, and\nright-margin values if the page orientation is `portrait' or\n`reverse-portrait'.  Otherwise, it is the sum of width, top-margin,\nand bottom-margin values.  Likewise, the paper height is the sum of\nheight, top-margin, and bottom-margin values if the page orientation\nis `portrait' or `reverse-portrait'.  Otherwise, it is the sum of\nheight, left-margin, and right-margin values.",  // C 9980-9996
	(),                                                                    // C 9997
	{
  let result: Lisp_Object;                                                     // C 9999

  block_input ();                                                              // C 10001
  result = xg_get_page_setup ();                                               // C 10002
  unblock_input ();                                                            // C 10003

  return result;                                                               // C 10005
});

/* C 10008-10043.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo", feature = "use_gtk"))]
DEFUN! ("x-print-frames-dialog", Fx_print_frames_dialog, Sx_print_frames_dialog,
	0, 1, "",                                                              // C 10008
	doc: "Pop up a print dialog to print the current contents of FRAMES.\nFRAMES should be nil (the selected frame), a frame, or a list of\nframes (each of which corresponds to one page).  Each frame should be\nvisible.\n\nNote: Text drawn with the `x' font backend is shown with hollow boxes.",  // C 10009-10014
	(frames: Lisp_Object),                                                 // C 10015
	{
  /* C's `frames` is written at C 10020 and C 10033, and the `DEFUN!` macro's `$arg:ident` cannot
     say `mut`.  */
  let mut frames = frames;
  let mut rest: Lisp_Object;                                                   // C 10017
  let mut tmp: Lisp_Object;

  if !CONSP (frames)                                                           // C 10019
    { frames = list1 (frames); }                                               // C 10020

  tmp = Qnil ();                                                               // C 10022
  rest = frames;                                                               // C 10023
  while CONSP (rest)
    {
      let f: *mut frame = decode_window_system_frame (XCAR (rest));            // C 10025
      let mut frame: Lisp_Object;                                              // C 10026

      XSETFRAME! (frame, f);                                                   // C 10028
      if !FRAME_VISIBLE_P (f)                                                  // C 10029
	{ error (c"Frames to be printed must be visible".as_ptr ()); }         // C 10030
      tmp = Fcons (frame, tmp);                                                // C 10031
      rest = XCDR (rest);
    }
  frames = Fnreverse (tmp);                                                    // C 10033

  /* Make sure the current matrices are up-to-date.  */                        // C 10035
  redisplay_preserve_echo_area (32);                                           // C 10036

  block_input ();                                                              // C 10038
  xg_print_frames_dialog (frames);                                             // C 10039
  unblock_input ();                                                            // C 10040

  return Qnil ();                                                              // C 10042
});


/* ==============================================================================================
   C 10047-10129: the last three subrs of the unit's body - `x-gtk-debug` (C 10050-10061, under
   C 10047's `#ifdef USE_GTK`, C 10048's `#ifdef HAVE_GTK3` and C 10049's `#if GTK_CHECK_VERSION
   (3, 14, 0)`), `x-display-set-last-user-time` (C 10066-10088) and `x-internal-focus-input-context`
   (C 10090-10129, whose body is C 10097's `#ifdef USE_GTK` and whose `return Qnil ();` (C 10128)
   sits outside it).  What follows them is C 10131's dead `#if 0` block and the `syms_of_xfns`.

   `gtk_check_version_ge_3_14` is this slice's reading of C 10049's version test; like
   `use_motif`/`have_xrandr`/`have_xaw3d` it is **not registered** in `femacs/xtask/Cargo.toml`
   (the `## Owed` row), and the probe passes it with `--cfg`.  */

/* C: `<gtk/gtkimcontext.h>`: `extern void gtk_im_context_focus_in (GtkIMContext *);`, `extern void
   gtk_im_context_focus_out (GtkIMContext *);` and `extern void gtk_im_context_set_client_window
   (GtkIMContext *, GdkWindow *);` - C 10113-10121's three pairs; `include/gtk.rs` carries
   `gtk_im_context_set_use_preedit` (1626-1631) and `GtkIMContext` but not these three rows.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn gtk_im_context_focus_in (context: *mut GtkIMContext);
  pub fn gtk_im_context_focus_out (context: *mut GtkIMContext);
  pub fn gtk_im_context_set_client_window (context: *mut GtkIMContext,
					   window: *mut GdkWindow);
}

/* C 10050-10061.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk", feature = "have_gtk3",
	    feature = "gtk_check_version_ge_3_14"))]
DEFUN! ("x-gtk-debug", Fx_gtk_debug, Sx_gtk_debug, 1, 1, 0,                    // C 10050
	doc: "Toggle interactive GTK debugging.",                              // C 10051
	(enable: Lisp_Object),                                                 // C 10052
	{
  let enable_debug: gboolean = (!NILP (enable)) as gboolean;                   // C 10054

  block_input ();                                                              // C 10056
  gtk_window_set_interactive_debugging (enable_debug);                         // C 10057
  unblock_input ();                                                            // C 10058

  return if NILP (enable) { Qnil () } else { Qt () };                          // C 10060
});

/* C: `xterm.h` 1772: `extern void x_set_last_user_time_from_lisp (struct x_display_info *, Time);`
   - C 10086's call; `src/xterm.c` defines it and no mirror carries the row (P-55/P-72 rule 3).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_set_last_user_time_from_lisp (dpyinfo: *mut x_display_info, time: Time);  // xterm.h 1772
}

/* C 10066-10088.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-display-set-last-user-time", Fx_display_set_last_user_time,
	Sx_display_set_last_user_time, 1, 2, 0,                                // C 10066-10067
	doc: "Set the last user time of TERMINAL to TIME-OBJECT.\nTIME-OBJECT is the X server time, in milliseconds, of the last user\ninteraction.  This is the timestamp that `x-get-selection-internal'\nwill use by default to fetch selection data.\nThe optional second argument TERMINAL specifies which display to act\non.  TERMINAL should be a terminal object, a frame or a display name\n(a string).  If TERMINAL is omitted or nil, that stands for the\nselected frame's display.",  // C 10068-10075
	(time_object: Lisp_Object, terminal: Lisp_Object),                     // C 10076
	{
  let dpyinfo: *mut x_display_info;                                            // C 10078
  /* C 10079's `uint32_t time;` with C 10084's `CONS_TO_INTEGER (time_object, uint32_t, time)`: the
     port's trait has the two signed/unsigned 64-bit impls and picks the helper from the binding's
     type, so C's `uint32_t` conversion is the explicit cast (the `src/inotify.rs` 449-453 shape).
     */
  let mut time_wide: u64 = 0;
  let time: u32;

  /* time should be a 32-bit integer, regardless of what the size of
     the X type `Time' is on this system.  */                                 // C 10081-10082
  dpyinfo = check_x_display_info (terminal);                                   // C 10083
  CONS_TO_INTEGER! (time_object, time_wide);                                   // C 10084
  time = time_wide as u32;

  x_set_last_user_time_from_lisp (dpyinfo, time as Time);                      // C 10086
  return Qnil ();                                                              // C 10087
});

/* C 10090-10129.  */
#[cfg (feature = "have_x_windows")]
DEFUN! ("x-internal-focus-input-context", Fx_internal_focus_input_context,
	Sx_internal_focus_input_context, 1, 1, 0,                              // C 10090-10091
	doc: "Focus and set the client window of all focused frames' GTK input context.\nIf FOCUS is nil, focus out and remove the client window instead.\nThis should be called from a variable watcher for `x-gtk-use-native-input'.",  // C 10092-10094
	(focus: Lisp_Object),                                                  // C 10095
	{
  #[cfg (feature = "use_gtk")]                                            // C 10097-10126
  {
    let mut dpyinfo: *mut x_display_info;                                      // C 10098
    let mut f: *mut frame;                                                     // C 10099
    let mut widget: *mut GtkWidget;                                            // C 10100

    block_input ();                                                            // C 10102
    /* C 10103's `for (dpyinfo = x_display_list; dpyinfo; dpyinfo = dpyinfo->next)`: init, test,
       body, then the increment C writes in the loop head (the `src/pgtkfns.rs` 1861-1872 shape).
       */
    dpyinfo = x_display_list;
    while !dpyinfo.is_null ()
      {
	f = (*dpyinfo).x_focus_frame;                                          // C 10105

	if !f.is_null ()                                                       // C 10107
	  {
	    widget = FRAME_GTK_OUTER_WIDGET! (f);                              // C 10109

	    if !NILP (focus)                                                   // C 10111
	      {
		gtk_im_context_focus_in ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).im_context);   // C 10113
		gtk_im_context_set_client_window ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).im_context,  // C 10114
						  gtk_widget_get_window (widget));  // C 10115
	      }
	    else
	      {
		gtk_im_context_focus_out ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).im_context);  // C 10119
		gtk_im_context_set_client_window ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).im_context,  // C 10120
						  std::ptr::null_mut ());      // C 10121
	      }
	  }
	dpyinfo = (*dpyinfo).next;
      }
    unblock_input ();                                                          // C 10125
  }

  return Qnil ();                                                              // C 10128
});


/* ==============================================================================================
   C 10131-10296: C 10131's dead `#if 0` block, the `Initialization` banner (C 10178-10180), the
   frame-parameter handler table (C 10185-10243) and the four libX11 replacements (C 10248-10296).

   The `#if 0` block (C 10131-10175) carries `x-test-string-conversion` (C 10133-10173), its
   `XIMStringConversionCallbackStruct`/`XIMStringConversionText` locals and C 10163's
   `xic_string_conversion_callback` call.  C's own condition is the literal `0`, so the block is
   **recorded, not transcribed** - the `#if false` class of this file's C 699-712 (`x_decode_color`)
   and of `src/xterm.rs`, which has the same rule for C's dead arms; `tools/extract-abi.py`'s
   inventory drops it too, so no subr row exists for `Fx_test_string_conversion`.

   The four replacements are C's own fallbacks for libX11 builds whose symbols are missing
   (C 10245-10246's comment); each is exported with `#[unsafe(no_mangle)]` because C links them by
   their libX11 names.  */

/* The `Initialization` banner (C 10178-10180) and C 10182-10183's "Keep this list in the same
   order as frame_parms in frame.c.  Use 0 for unsupported frame parameters." are comments with no
   item of their own.  */

/* C: `dispextern.h` 3063: `typedef void (*frame_parm_handler) (struct frame *, Lisp_Object,
   Lisp_Object);` - the element type of C 10185's table.  The mirror `include/dispextern.rs` does
   not carry the alias (its `redisplay_interface.frame_parm_handlers` is `*mut c_void`), so the
   table spells the typedef's own body here - what `src/pgtkfns.rs` 519-520 and
   `src/haikufns.rs` 223-224 do for their own handler arrays.  */
#[cfg (feature = "have_x_windows")]
type frame_parm_handler =
  Option<unsafe extern "C" fn (f: *mut frame, val: Lisp_Object, oldval: Lisp_Object)>;

/* C: `frame.h` 1844-1871 and `xterm.h` 1893-1896 - the twenty-four handlers C 10187-10242 names
   that this unit does not define: the twenty `gui_set_*` rows live in `frame.c` and the four
   `x_set_*` rows (`sticky`, `shaded`, `skip_taskbar`, `z_group`) in `xterm.c`, and no mirror of
   this unit's include graph carries any of them (P-55/P-72 rule 3).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  // frame.h 1844
  // frame.h 1845
  // frame.h 1846
        // frame.h 1847
  // frame.h 1848
  // frame.h 1849
  // frame.h 1850
  // frame.h 1851
                      // frame.h 1852
                     // frame.h 1854
   // frame.h 1856
    // frame.h 1857
    // frame.h 1858
  // frame.h 1859
                     // frame.h 1860
                   // frame.h 1861
                         // frame.h 1862
                        // frame.h 1863
         // frame.h 1869
                        // frame.h 1871
  pub fn x_set_sticky (f: *mut frame, val: Lisp_Object, oldval: Lisp_Object);         // xterm.h 1893
  pub fn x_set_shaded (f: *mut frame, val: Lisp_Object, oldval: Lisp_Object);         // xterm.h 1894
  pub fn x_set_skip_taskbar (f: *mut frame, val: Lisp_Object, oldval: Lisp_Object);   // xterm.h 1895
  pub fn x_set_z_group (f: *mut frame, val: Lisp_Object, oldval: Lisp_Object);        // xterm.h 1896
}

/* C 10185-10243.  The table lives in this module, which is glob-imported just below, because the
   mirror `include/xterm.rs` 1630 declares the name at the crate root (xterm.h 1735's row, whose
   reader is `xterm.c` 32026) and a crate-root definition beside that declaration is `E0428` in
   the single-crate layout - the `xterm_globals` device (`src/xterm.rs` 754-780, PORTING.md
   P-21/P-90 7, where `x_display_list` has the same pair).  */
#[cfg (feature = "have_x_windows")]
pub mod xfns_globals {
  use super::*;

  /* C 10226-10230's `#ifdef HAVE_XDBE` pair: C's element is either the handler or `NULL`, and
     Rust cannot put an attribute on an array element, so the pair is two cfg'd constants - the
     `internal_border_default!` device of 7230/7232 read as a value.  */
  #[cfg (feature = "have_xdbe")]
  const X_SET_INHIBIT_DOUBLE_BUFFERING_ENTRY: frame_parm_handler =
    Some (x_set_inhibit_double_buffering);
  #[cfg (not (feature = "have_xdbe"))]
  const X_SET_INHIBIT_DOUBLE_BUFFERING_ENTRY: frame_parm_handler = None;

  /// C: xfns.c 10185-10243: `frame_parm_handler x_frame_parm_handlers[] = …`
  #[unsafe(no_mangle)]
  pub static mut x_frame_parm_handlers: [frame_parm_handler; 53] =
    [
      Some (gui_set_autoraise),                     // C 10187
      Some (gui_set_autolower),                     // C 10188
      Some (x_set_background_color),                // C 10189
      Some (x_set_border_color),                    // C 10190
      Some (gui_set_border_width),                  // C 10191
      Some (x_set_cursor_color),                    // C 10192
      Some (x_set_cursor_type),                     // C 10193
      Some (gui_set_font),                          // C 10194
      Some (x_set_foreground_color),                // C 10195
      Some (x_set_icon_name),                       // C 10196
      Some (x_set_icon_type),                       // C 10197
      Some (x_set_child_frame_border_width),        // C 10198
      Some (x_set_internal_border_width),           // C 10199
      Some (gui_set_right_divider_width),           // C 10200
      Some (gui_set_bottom_divider_width),          // C 10201
      Some (x_set_menu_bar_lines),                  // C 10202
      Some (x_set_mouse_color),                     // C 10203
      Some (x_explicitly_set_name),                 // C 10204
      Some (gui_set_scroll_bar_width),              // C 10205
      Some (gui_set_scroll_bar_height),             // C 10206
      Some (x_set_title),                           // C 10207
      Some (gui_set_unsplittable),                  // C 10208
      Some (gui_set_vertical_scroll_bars),          // C 10209
      Some (gui_set_horizontal_scroll_bars),        // C 10210
      Some (gui_set_visibility),                    // C 10211
      Some (x_set_tab_bar_lines),                   // C 10212
      Some (x_set_tool_bar_lines),                  // C 10213
      Some (x_set_scroll_bar_foreground),           // C 10214
      Some (x_set_scroll_bar_background),           // C 10215
      Some (gui_set_screen_gamma),                  // C 10216
      Some (gui_set_line_spacing),                  // C 10217
      Some (gui_set_left_fringe),                   // C 10218
      Some (gui_set_right_fringe),                  // C 10219
      Some (x_set_wait_for_wm),                     // C 10220
      Some (gui_set_fullscreen),                    // C 10221
      Some (gui_set_font_backend),                  // C 10222
      Some (x_set_alpha),                           // C 10223
      Some (x_set_sticky),                          // C 10224
      Some (x_set_tool_bar_position),               // C 10225
      X_SET_INHIBIT_DOUBLE_BUFFERING_ENTRY,        // C 10227 (`HAVE_XDBE`)
      X_SET_INHIBIT_DOUBLE_BUFFERING_ENTRY,        // C 10229 (the `#else` arm)
      Some (x_set_undecorated),                     // C 10231
      Some (x_set_parent_frame),                    // C 10232
      Some (x_set_skip_taskbar),                    // C 10233
      Some (x_set_no_focus_on_map),                 // C 10234
      Some (x_set_no_accept_focus),                 // C 10235
      Some (x_set_z_group),                         // C 10236
      Some (x_set_override_redirect),               // C 10237
      Some (gui_set_no_special_glyphs),             // C 10238
      Some (x_set_alpha_background),                // C 10239
      Some (gui_set_borders_respect_alpha_background), // C 10240
      Some (x_set_use_frame_synchronization),       // C 10241
      Some (x_set_shaded),                          // C 10242
    ];
}
#[cfg (feature = "have_x_windows")]
use xfns_globals::*;

/* C 10245-10246's "Some versions of libX11 don't have symbols for a few functions we need, so
   define replacements here." is the sentence the four items below answer.  */

/* C: `<X11/Xlib.h>`: `typedef int Status;` (measured: **4** bytes on this machine's xorgproto),
   `extern Status XQueryTree (Display *, Window, Window *, Window *, Window **, unsigned int *);`
   and `extern Status XDestroyWindow (Display *, Window);` - C 10250/10286/10290's uses; the
   mirror carries no row for the typedef and none for `XDestroyWindow`.  */
#[cfg (feature = "have_x_windows")]
pub type Status = c_int;
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XDestroyWindow (display: *mut Display, w: Window) -> Status;          // Xlib.h 961
}

/* C: `<X11/extensions/XKBlib.h>` 61-88: `typedef struct _XkbMapNotifyEvent { … }
   XkbMapNotifyEvent;` - C 10251's parameter, which C 10253 never reads.  The record is carried as
   its measured shape (the `XEvent` rule of `include/xlib.rs` 1405-1413): `sizeof` **104**,
   align 8, and its member records are owed.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xkb",
	    not (feature = "have_xkbrefreshkeyboardmapping")))]
#[repr(C, align(8))]
pub struct XkbMapNotifyEvent { _private: [u8; 104] }
#[cfg (all (feature = "have_x_windows", feature = "have_xkb",
	    not (feature = "have_xkbrefreshkeyboardmapping")))]
const _: () = core::assert! (core::mem::size_of::<XkbMapNotifyEvent> () == 104);
#[cfg (all (feature = "have_x_windows", feature = "have_xkb",
	    not (feature = "have_xkbrefreshkeyboardmapping")))]
const _: () = core::assert! (core::mem::align_of::<XkbMapNotifyEvent> () == 8);

/* C 10248-10264's first pair: C 10249's `#ifndef HAVE_XKBREFRESHKEYBOARDMAPPING` and C 10257's
   `#ifndef HAVE_XKBFREENAMES`, both inside C 10248's `#ifdef HAVE_XKB`.  Neither feature is in
   `femacs/xtask/Cargo.toml`'s table, so `not (…)` is true in every arm and the two replacements
   compile wherever `have_xkb` does.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xkb",
	    not (feature = "have_xkbrefreshkeyboardmapping")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XkbRefreshKeyboardMapping (event: *mut XkbMapNotifyEvent) -> Status
{
  use crate::x11_constants::Success;   // C 10253's `Success` (X.h 471, the module's `c_int` row)
  return Success;                                                              // C 10253
}

/// C 10259-10262.
#[cfg (all (feature = "have_x_windows", feature = "have_xkb",
	    not (feature = "have_xkbfreenames")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XkbFreeNames (xkb: XkbDescPtr, which: c_uint, free_map: Bool)
{
  return;                                                                      // C 10261
}

/* C 10266-10277.  */
#[cfg (all (feature = "have_x_windows", not (feature = "have_xdisplaycells")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XDisplayCells (dpy: *mut Display, screen_number: c_int) -> c_int
{
  let dpyinfo: *mut x_display_info = x_dpyinfo (dpy);                           // C 10270

  /* Not strictly correct, since the display could be using a
     non-default visual, but it satisfies the callers we need to care
     about.  */                                                                 // C 10272-10274
  return (*dpyinfo).visual_info.colormap_size;                                  // C 10275
}

/* C 10279-10296.  */
#[cfg (all (feature = "have_x_windows", not (feature = "have_xdestroysubwindows")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XDestroySubwindows (dpy: *mut Display, w: Window) -> c_int
{
  let mut root: Window = 0;                                                     // C 10283
  let mut parent: Window = 0;
  let mut children: *mut Window = std::ptr::null_mut ();
  let mut nchildren: c_uint = 0;                                                // C 10284
  let mut i: c_uint;

  if XQueryTree (dpy, w, &raw mut root, &raw mut parent, &raw mut children,
		 &raw mut nchildren) != 0                                      // C 10286
    {
      /* C 10289's `for (i = 0; i < nchildren; ++i)`: init, test, body, then the increment C
	 writes in the loop head.  */
      i = 0;
      while i < nchildren
	{
	  XDestroyWindow (dpy, *children.offset (i as isize));                  // C 10290
	  i += 1;
	}
      XFree (children as *mut c_void);                                          // C 10291
    }

  return 0;                                                                     // C 10294
}


/* ==============================================================================================
   C 10298-10662: `syms_of_xfns` - the file's last definition, and with it the whole of
   `emacs/src/xfns.c` (10,662 lines) is ported.  It holds the `DEFSYM` tables (C 10301-10307,
   C 10310-10318 under `USE_CAIRO`, C 10321-10325 and C 10506-10520), the two `Fput`s
   (C 10327-10330), the twenty-two `DEFVAR_LISP`/`DEFVAR_BOOL` declarations with their docstrings,
   the six `Fprovide`s (C 10503/10525/10529/10531/10544/10545), the 44 live `defsubr`
   registrations (C 10573-10616 and C 10636-10661) and the seven `staticpro` pairs
   (C 10621-10634).

   Recorded, not transcribed: C 10338-10343 and C 10352-10357's `#if false` blocks (C's own
   literal `false`, around the `x-nontext-pointer-shape` and `x-mode-pointer-shape` DEFVARs, whose
   C 10344/10358 assignments are the live statements) and C 10617-10619's `#if 0` around
   `defsubr (&Sx_test_string_conversion)` (the subr has no definition, since C 10131's block is
   dead too).  All three are the C 699-712 class.

   The `defsubr` calls keep C's own conditions: C 10637-10639's `defined (USE_MOTIF) || defined
   (USE_GTK)`, C 10641-10643's `defined (USE_GTK) && defined (HAVE_FREETYPE)`, C 10647-10654's
   `USE_CAIRO` with `USE_GTK` inside and C 10655-10661's `USE_GTK`/`HAVE_GTK3`/
   `GTK_CHECK_VERSION (3, 14, 0)` triple - the last measured by the **GTK314** arm.  */

/* C: `<Xm/Xm.h>`: `#define XmVERSION_STRING "@(#)Motif Version …"` - C 10535's
   `build_string (XmVERSION_STRING)`.  **UNVERIFIED**: no Motif headers are installed here, so the
   string is the banner's own shape and the ledger's Owed names the row (the `XmCR_CANCEL` class).
   */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_motif"))]
pub const XmVERSION_STRING: &core::ffi::CStr = c"@(#)Motif Version 2.3.8";

/* C: `gtk/gtkversion.h`'s `GTK_MAJOR_VERSION` / `GTK_MINOR_VERSION` / `GTK_MICRO_VERSION`
   (C 10552) and `<cairo.h>`'s `CAIRO_VERSION_MAJOR` / `MINOR` / `MICRO` (C 10565) - the six
   operands of the two version strings below.  No GTK or cairo header is installed here, so the
   values are the ones this tree already carries: `src/pgtkfns.rs` 5183-5185 (3.22.23, the
   requirement `configure.ac` 3745 sets) and `src/haikufns.rs` 343-347 (1.18.0).  **UNVERIFIED**
   against a linked toolkit; the ledger's Owed names both rows.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
const GTK_MAJOR_VERSION: c_int = 3;
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
const GTK_MINOR_VERSION: c_int = 22;
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
const GTK_MICRO_VERSION: c_int = 23;
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
const CAIRO_VERSION_MAJOR: c_int = 1;
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
const CAIRO_VERSION_MINOR: c_int = 18;
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
const CAIRO_VERSION_MICRO: c_int = 0;

/* C: `lisp.h` 4566-4567: `extern Lisp_Object make_specified_string (const char *chars,
   ptrdiff_t nchars, ptrdiff_t nbytes, bool multibyte);` (the docstring's own source) - C
   10553/10567-10568's two calls; `alloc.c` defines it and no mirror carries the row (the
   `src/editfns.rs` 213-218 use-site shape).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn make_specified_string (chars: *const c_char, nchars: ptrdiff_t,
				nbytes: ptrdiff_t, multibyte: bool) -> Lisp_Object;
}

/* C's twenty-six `globals.f_<name>` fields this unit's `syms_of_xfns` registers (its
   `DEFVAR_LISP`/`DEFVAR_BOOL` calls at C 10332-10484 and the three version strings at C 10533,
   10547 and 10560), defined here as the port's storage - C's `struct emacs_globals` is
   zero-initialised and the DEFVAR macros only take each field's address, so the initialisers
   are `lisp_h_Qnil ()` / `false` (docs/status.md 3.29, the `src/cmds.rs` 489-492 row and the
   `src/image.rs` block shape).  Each carries the `#[cfg]` its call site sits under: the
   twenty-three in `syms_of_xfns`'s body take the function's `have_x_windows` (C's two dead
   `#if false` blocks at C 10338-10343 and C 10352-10357 leave their DEFVARs live text), and
   the three version strings take their C `#ifdef` too - `USE_X_TOOLKIT` + `USE_MOTIF`,
   `USE_GTK` and `USE_CAIRO`.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_pointer_shape: Lisp_Object = lisp_h_Qnil ();                     // C 10332
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_nontext_pointer_shape: Lisp_Object = lisp_h_Qnil ();             // C 10339
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_hourglass_pointer_shape: Lisp_Object = lisp_h_Qnil ();           // C 10346
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_mode_pointer_shape: Lisp_Object = lisp_h_Qnil ();                // C 10353
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_sensitive_text_pointer_shape: Lisp_Object = lisp_h_Qnil ();      // C 10360
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_horizontal_drag_shape: Lisp_Object = lisp_h_Qnil ();      // C 10367
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_vertical_drag_shape: Lisp_Object = lisp_h_Qnil ();        // C 10374
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_left_edge_shape: Lisp_Object = lisp_h_Qnil ();            // C 10381
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_top_left_corner_shape: Lisp_Object = lisp_h_Qnil ();      // C 10388
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_top_edge_shape: Lisp_Object = lisp_h_Qnil ();             // C 10395
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_top_right_corner_shape: Lisp_Object = lisp_h_Qnil ();     // C 10402
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_right_edge_shape: Lisp_Object = lisp_h_Qnil ();           // C 10409
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_bottom_right_corner_shape: Lisp_Object = lisp_h_Qnil ();  // C 10416
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_bottom_edge_shape: Lisp_Object = lisp_h_Qnil ();          // C 10423
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_window_bottom_left_corner_shape: Lisp_Object = lisp_h_Qnil ();   // C 10430
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_cursor_fore_pixel: Lisp_Object = lisp_h_Qnil ();                 // C 10437
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_max_tooltip_size: Lisp_Object = lisp_h_Qnil ();                  // C 10441
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_no_window_manager: Lisp_Object = lisp_h_Qnil ();                 // C 10446
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut Vx_pixel_size_width_font_regexp: Lisp_Object = lisp_h_Qnil ();      // C 10454
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut x_gtk_use_old_file_dialog: bool = false;                            // C 10465
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut x_gtk_show_hidden_files: bool = false;                              // C 10472
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut x_gtk_file_dialog_help_text: bool = false;                          // C 10478
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut x_gtk_resize_child_frames: Lisp_Object = lisp_h_Qnil ();            // C 10484
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_motif"))]
#[unsafe(no_mangle)]
pub static mut Vmotif_version_string: Lisp_Object = lisp_h_Qnil ();                // C 10533
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
#[unsafe(no_mangle)]
pub static mut Vgtk_version_string: Lisp_Object = lisp_h_Qnil ();                  // C 10547
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub static mut Vcairo_version_string: Lisp_Object = lisp_h_Qnil ();                // C 10560

/* C 10298-10662.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xfns ()
{
  /* C's `DEFVAR_LISP`/`DEFVAR_BOOL` invocations carry their text in C's own `doc:` field, which
     the port's macros take as their third argument (`include/lisp.rs` 3483-3496, the
     `src/frame.rs` 7644-7645 shape); the docstrings below are C's values, and the two dead
     `#if false` blocks' `doc:` texts are recorded above them rather than written.  */
  DEFSYM! (Qundefined_color, "undefined-color");
  DEFSYM! (Qcompound_text, "compound-text");
  DEFSYM! (Qcancel_timer, "cancel-timer");
  DEFSYM! (Qfont_parameter, "font-parameter");
  DEFSYM! (Qmono, "mono");
  DEFSYM! (Qassq_delete_all, "assq-delete-all");
  DEFSYM! (Qresize_mode, "resize-mode");

#[cfg (feature = "use_cairo")]
  {
  DEFSYM! (Qpdf, "pdf");
  DEFSYM! (Qorientation, "orientation");
  DEFSYM! (Qtop_margin, "top-margin");
  DEFSYM! (Qbottom_margin, "bottom-margin");
  DEFSYM! (Qportrait, "portrait");
  DEFSYM! (Qlandscape, "landscape");
  DEFSYM! (Qreverse_portrait, "reverse-portrait");
  DEFSYM! (Qreverse_landscape, "reverse-landscape");
  }

  DEFSYM! (QXdndActionCopy, "XdndActionCopy");
  DEFSYM! (QXdndActionMove, "XdndActionMove");
  DEFSYM! (QXdndActionLink, "XdndActionLink");
  DEFSYM! (QXdndActionAsk, "XdndActionAsk");
  DEFSYM! (QXdndActionPrivate, "XdndActionPrivate");

  Fput (Qundefined_color (), Qerror_conditions (), list! (Qundefined_color (), Qerror ()));
  Fput (Qundefined_color (), Qerror_message (), build_string (c"Undefined color".as_ptr ()));

  DEFVAR_LISP! (c"x-pointer-shape", Vx_pointer_shape,
		doc: "The shape of the pointer when over text.\nChanging the value does not affect existing frames\nunless you set the mouse color.");   // C 10332-10335
  Vx_pointer_shape = Qnil ();

/* C 10338-10343: `#if false /* This doesn't really do anything.  */` around the
   `x-nontext-pointer-shape` DEFVAR - C's own literal `false`, so the block is recorded
   and the assignment below it (C 10344) is the live statement (the C 699-712 class).  */
  DEFVAR_LISP! (c"x-nontext-pointer-shape", Vx_nontext_pointer_shape,
		doc: "The shape of the pointer when not over text.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10339-10342
  Vx_nontext_pointer_shape = Qnil ();
  DEFVAR_LISP! (c"x-hourglass-pointer-shape", Vx_hourglass_pointer_shape,
		doc: "The shape of the pointer when Emacs is busy.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10346-10349
  Vx_hourglass_pointer_shape = Qnil ();
/* C 10352-10357: the same `#if false` around the `x-mode-pointer-shape` DEFVAR; C 10358's
   assignment below is the live statement.  */
  DEFVAR_LISP! (c"x-mode-pointer-shape", Vx_mode_pointer_shape,
		doc: "The shape of the pointer when over the mode line.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10353-10356
  Vx_mode_pointer_shape = Qnil ();
  DEFVAR_LISP! (c"x-sensitive-text-pointer-shape", Vx_sensitive_text_pointer_shape,
		doc: "The shape of the pointer when over mouse-sensitive text.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10360-10364
  Vx_sensitive_text_pointer_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-horizontal-drag-cursor", Vx_window_horizontal_drag_shape,
		doc: "Pointer shape to use for indicating a window can be dragged horizontally.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10367-10371
  Vx_window_horizontal_drag_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-vertical-drag-cursor", Vx_window_vertical_drag_shape,
		doc: "Pointer shape to use for indicating a window can be dragged vertically.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10374-10378
  Vx_window_vertical_drag_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-left-edge-cursor", Vx_window_left_edge_shape,
		doc: "Pointer shape indicating a left x-window edge can be dragged.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10381-10385
  Vx_window_left_edge_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-top-left-corner-cursor", Vx_window_top_left_corner_shape,
		doc: "Pointer shape indicating a top left x-window corner can be dragged.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10388-10392
  Vx_window_top_left_corner_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-top-edge-cursor", Vx_window_top_edge_shape,
		doc: "Pointer shape indicating a top x-window edge can be dragged.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10395-10399
  Vx_window_top_edge_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-top-right-corner-cursor", Vx_window_top_right_corner_shape,
		doc: "Pointer shape indicating a top right x-window corner can be dragged.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10402-10406
  Vx_window_top_right_corner_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-right-edge-cursor", Vx_window_right_edge_shape,
		doc: "Pointer shape indicating a right x-window edge can be dragged.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10409-10413
  Vx_window_right_edge_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-bottom-right-corner-cursor", Vx_window_bottom_right_corner_shape,
		doc: "Pointer shape indicating a bottom right x-window corner can be dragged.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10416-10420
  Vx_window_bottom_right_corner_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-bottom-edge-cursor", Vx_window_bottom_edge_shape,
		doc: "Pointer shape indicating a bottom x-window edge can be dragged.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10423-10427
  Vx_window_bottom_edge_shape = Qnil ();
  DEFVAR_LISP! (c"x-window-bottom-left-corner-cursor", Vx_window_bottom_left_corner_shape,
		doc: "Pointer shape indicating a bottom left x-window corner can be dragged.\nThis variable takes effect when you create a new frame\nor when you set the mouse color.");   // C 10430-10434
  Vx_window_bottom_left_corner_shape = Qnil ();
  DEFVAR_LISP! (c"x-cursor-fore-pixel", Vx_cursor_fore_pixel,
		doc: "A string indicating the foreground color of the cursor box.");   // C 10437-10438
  Vx_cursor_fore_pixel = Qnil ();
  DEFVAR_LISP! (c"x-max-tooltip-size", Vx_max_tooltip_size,
		doc: "Maximum size for tooltips.\nValue is a pair (COLUMNS . ROWS).  Text larger than this is clipped.");   // C 10441-10443
  Vx_max_tooltip_size = Qnil ();
  DEFVAR_LISP! (c"x-no-window-manager", Vx_no_window_manager,
		doc: "Non-nil if no X window manager is in use.\nEmacs doesn't try to figure this out; this is always nil\nunless you set it to something else.");   // C 10446-10449
  /* We don't have any way to find this out, so set it to nil
     and maybe the user would like to set it to t.  */
  Vx_no_window_manager = Qnil ();
  DEFVAR_LISP! (c"x-pixel-size-width-font-regexp", Vx_pixel_size_width_font_regexp,
		doc: "Regexp matching a font name whose width is the same as `PIXEL_SIZE'.\n\nSince Emacs gets the width of a font matching this regexp from the\nPIXEL_SIZE field of the name, the font-finding mechanism gets faster for\nsuch a font.  This is especially effective for large fonts such as\nChinese, Japanese, and Korean.");   // C 10454-10461
  Vx_pixel_size_width_font_regexp = Qnil ();
/* This is not ifdef:ed, so other builds than GTK can customize it.  */
  DEFVAR_BOOL! (c"x-gtk-use-old-file-dialog", x_gtk_use_old_file_dialog,
		doc: "Non-nil means prompt with the old GTK file selection dialog.\nIf nil or if the file selection dialog is not available, the new GTK file\nchooser is used instead.  To turn off all file dialogs set the\nvariable `use-file-dialog'.");   // C 10465-10469
  x_gtk_use_old_file_dialog = false;
  DEFVAR_BOOL! (c"x-gtk-show-hidden-files", x_gtk_show_hidden_files,
		doc: "If non-nil, the GTK file chooser will by default show hidden files.\nNote that this is just the default, there is a toggle button on the file\nchooser to show or not show hidden files on a case by case basis.");   // C 10472-10475
  x_gtk_show_hidden_files = false;
  DEFVAR_BOOL! (c"x-gtk-file-dialog-help-text", x_gtk_file_dialog_help_text,
		doc: "If non-nil, the GTK file chooser will show additional help text.\nIf more space for files in the file chooser dialog is wanted, set this to nil\nto turn the additional text off.");   // C 10478-10481
  x_gtk_file_dialog_help_text = true;
  DEFVAR_LISP! (c"x-gtk-resize-child-frames", x_gtk_resize_child_frames,
		doc: "If non-nil, resize child frames specially with GTK builds.\nIf this is nil, resize child frames like any other frames.  This is the\ndefault and usually works with most desktops.  Some desktop environments\n(GNOME shell in particular when using the mutter window manager),\nhowever, may refuse to resize a child frame when Emacs is built with\nGTK3.  For those environments, the two settings below are provided.\n\nIf this equals the symbol `resize-mode', Emacs uses GTK's resize mode to\nalways trigger an immediate resize of the child frame.  This method is\ndeprecated by GTK and may not work in future versions of that toolkit.\nIt also may freeze Emacs when used with other desktop environments.  It\navoids, however, the unpleasant flicker induced by the hiding approach.\n\nThis variable is considered a temporary workaround and will be hopefully\neliminated in future versions of Emacs.");   // C 10484-10499
  x_gtk_resize_child_frames = Qnil ();
  Fprovide (Qx (), Qnil ());
  DEFSYM! (Qrun_at_time, "run-at-time");
  DEFSYM! (Qx_hide_tip, "x-hide-tip");
  DEFSYM! (Qalways, "always");
  DEFSYM! (Qwhen_mapped, "when-mapped");
  DEFSYM! (Qnot_useful, "not-useful");
  DEFSYM! (Qstatic_gray, "static-gray");
  DEFSYM! (Qgray_scale, "gray-scale");
  DEFSYM! (Qstatic_color, "static-color");
  DEFSYM! (Qpseudo_color, "pseudo-color");
  DEFSYM! (Qtrue_color, "true-color");
  DEFSYM! (Qdirect_color, "direct-color");
  DEFSYM! (Qgrayscale, "grayscale");
  DEFSYM! (Qcolor, "color");
#[cfg (feature = "have_xinput2")]
  {
  DEFSYM! (Qxinput2, "xinput2");
  Fprovide (Qxinput2 (), Qnil ());
  }
#[cfg (feature = "use_x_toolkit")]
  {
  Fprovide (intern_c_string (c"x-toolkit".as_ptr ()), Qnil ());
#[cfg (feature = "use_motif")]
  {
  Fprovide (intern_c_string (c"motif".as_ptr ()), Qnil ());
  DEFVAR_LISP! (c"motif-version-string", Vmotif_version_string,
		doc: "Version info for LessTif/Motif.");   // C 10533-10534
    Vmotif_version_string = build_string (XmVERSION_STRING.as_ptr ());   // C 10535
  }
  }
#[cfg (feature = "use_gtk")]
  {
  /* Provide x-toolkit also for GTK.  Internally GTK does not use Xt so it
     is not an X toolkit in that sense (USE_X_TOOLKIT is not defined).
     But for a user it is a toolkit for X, and indeed, configure
     accepts --with-x-toolkit=gtk.  */
  Fprovide (intern_c_string (c"x-toolkit".as_ptr ()), Qnil ());
  Fprovide (intern_c_string (c"gtk".as_ptr ()), Qnil ());
  DEFVAR_LISP! (c"gtk-version-string", Vgtk_version_string,
		doc: "Version info for GTK+.");   // C 10547-10548
  {
    let mut gtk_version: [c_char; 6 + 3 * INT_STRLEN_BOUND! (c_int)] =
      [0; 6 + 3 * INT_STRLEN_BOUND! (c_int)];
    /* C 10550's `sizeof ".." + 3 * INT_STRLEN_BOUND (int)` is inside `countof!`; the
       port's own `INT_STRLEN_BOUND!` (6781) is the same expression, so the array length
       is C's.  */
    let len: c_int = sprintf (gtk_version.as_mut_ptr (), c"%d.%d.%d".as_ptr (),
			      GTK_MAJOR_VERSION, GTK_MINOR_VERSION,
			      GTK_MICRO_VERSION);   // C 10551-10552
    Vgtk_version_string = make_specified_string (gtk_version.as_ptr (),
					      len as ptrdiff_t, len as ptrdiff_t,
					      false);   // C 10553
  }
  }
#[cfg (feature = "use_cairo")]
  {
  Fprovide (intern_c_string (c"cairo".as_ptr ()), Qnil ());
  DEFVAR_LISP! (c"cairo-version-string", Vcairo_version_string,
		doc: "Version info for cairo.");   // C 10560-10561
  {
    let mut cairo_version: [c_char; 6 + 3 * INT_STRLEN_BOUND! (c_int)] =
      [0; 6 + 3 * INT_STRLEN_BOUND! (c_int)];
    let len: c_int = sprintf (cairo_version.as_mut_ptr (), c"%d.%d.%d".as_ptr (),
			      CAIRO_VERSION_MAJOR, CAIRO_VERSION_MINOR,
			      CAIRO_VERSION_MICRO);   // C 10563-10566
    Vcairo_version_string = make_specified_string (cairo_version.as_ptr (),
						len as ptrdiff_t, len as ptrdiff_t,
						false);   // C 10567-10568
  }
  }
  /* X window properties.  */
  defsubr (&raw mut Sx_change_window_property);
  defsubr (&raw mut Sx_delete_window_property);
  defsubr (&raw mut Sx_window_property);
  defsubr (&raw mut Sx_window_property_attributes);
  defsubr (&raw mut Sxw_display_color_p);
  defsubr (&raw mut Sx_display_grayscale_p);
  defsubr (&raw mut Sxw_color_defined_p);
  defsubr (&raw mut Sxw_color_values);
  defsubr (&raw mut Sx_server_max_request_size);
  defsubr (&raw mut Sx_server_vendor);
  defsubr (&raw mut Sx_server_version);
  defsubr (&raw mut Sx_server_input_extension_version);
  defsubr (&raw mut Sx_display_pixel_width);
  defsubr (&raw mut Sx_display_pixel_height);
  defsubr (&raw mut Sx_display_mm_width);
  defsubr (&raw mut Sx_display_mm_height);
  defsubr (&raw mut Sx_display_screens);
  defsubr (&raw mut Sx_display_planes);
  defsubr (&raw mut Sx_display_color_cells);
  defsubr (&raw mut Sx_display_visual_class);
  defsubr (&raw mut Sx_display_backing_store);
  defsubr (&raw mut Sx_display_save_under);
  defsubr (&raw mut Sx_display_monitor_attributes_list);
  defsubr (&raw mut Sx_frame_geometry);
  defsubr (&raw mut Sx_frame_edges);
  defsubr (&raw mut Sx_frame_list_z_order);
  defsubr (&raw mut Sx_frame_restack);
  defsubr (&raw mut Sx_mouse_absolute_pixel_position);
  defsubr (&raw mut Sx_set_mouse_absolute_pixel_position);
  defsubr (&raw mut Sx_wm_set_size_hint);
  defsubr (&raw mut Sx_create_frame);
  defsubr (&raw mut Sx_open_connection);
  defsubr (&raw mut Sx_close_connection);
  defsubr (&raw mut Sx_display_list);
  defsubr (&raw mut Sx_synchronize);
  defsubr (&raw mut Sx_backspace_delete_keys_p);
  defsubr (&raw mut Sx_show_tip);
  defsubr (&raw mut Sx_hide_tip);
  defsubr (&raw mut Sx_double_buffered_p);
  defsubr (&raw mut Sx_begin_drag);
  defsubr (&raw mut Sx_display_set_last_user_time);
  defsubr (&raw mut Sx_translate_coordinates);
  defsubr (&raw mut Sx_get_modifier_masks);
  /* C 10617-10619: `#if 0` around `defsubr (&Sx_test_string_conversion);` - C's own
     literal `0`, recorded with the block above (no `Sx_test_string_conversion` exists).  */
  tip_timer = Qnil ();
  staticpro (&raw mut tip_timer);
  tip_frame = Qnil ();
  staticpro (&raw mut tip_frame);
  tip_last_frame = Qnil ();
  staticpro (&raw mut tip_last_frame);
  tip_last_string = Qnil ();
  staticpro (&raw mut tip_last_string);
  tip_last_parms = Qnil ();
  staticpro (&raw mut tip_last_parms);
  tip_dx = Qnil ();
  staticpro (&raw mut tip_dx);
  tip_dy = Qnil ();
  staticpro (&raw mut tip_dy);
  defsubr (&raw mut Sx_uses_old_gtk_dialog);
#[cfg (any (feature = "use_motif", feature = "use_gtk"))]
  {
  defsubr (&raw mut Sx_file_dialog);
  }
#[cfg (all (feature = "use_gtk", feature = "freetype"))]
  {
  defsubr (&raw mut Sx_select_font);
  }
  defsubr (&raw mut Sx_internal_focus_input_context);
#[cfg (feature = "use_cairo")]
  {
  defsubr (&raw mut Sx_export_frames);
#[cfg (feature = "use_gtk")]
  {
  defsubr (&raw mut Sx_page_setup_dialog);
  defsubr (&raw mut Sx_get_page_setup);
  defsubr (&raw mut Sx_print_frames_dialog);
  }
  }
#[cfg (feature = "use_gtk")]
  {
#[cfg (feature = "have_gtk3")]
  {
#[cfg (feature = "gtk_check_version_ge_3_14")]
  {
  defsubr (&raw mut Sx_gtk_debug);
  }
  }
}

}
