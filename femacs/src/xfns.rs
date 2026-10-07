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
  /// C: frame.h 1868: `extern void gui_set_alpha_background (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn gui_set_alpha_background (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
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
unsafe fn x_set_wait_for_wm (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
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
unsafe fn x_set_alpha_background (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
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
  /// C: frame.h 1511: `extern void adjust_frame_size (struct frame *, int, int, int, bool, Lisp_Object);`
  pub fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int, inhibit: c_int,
                            pretend: bool, parameter: Lisp_Object);
  /// C: xterm.h 1900: `extern void x_clear_under_internal_border (struct frame *f);`
  pub fn x_clear_under_internal_border (f: *mut frame);
  /// C: xterm.h 1773/1774: `extern void x_make_frame_visible (struct frame *);` / `extern void
  /// x_make_frame_invisible (struct frame *);`
  pub fn x_make_frame_visible (f: *mut frame);
  pub fn x_make_frame_invisible (f: *mut frame);
  /// C: dispextern.h 3748: `extern Lisp_Object image_find_image_file (Lisp_Object);`
  pub fn image_find_image_file (file: Lisp_Object) -> Lisp_Object;
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
unsafe fn x_set_tool_bar_position (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
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
unsafe fn x_set_inhibit_double_buffering (f: *mut frame, new_value: Lisp_Object,
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
unsafe fn x_set_undecorated (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
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
unsafe fn x_set_parent_frame (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
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
unsafe fn x_set_no_focus_on_map (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
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
unsafe fn x_set_no_accept_focus (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  if !EQ (new_value, old_value)
    {
      #[cfg (feature = "use_gtk")]
      { xg_set_no_accept_focus (f, new_value); }
      #[cfg (all (not (feature = "use_gtk"), feature = "use_x_toolkit"))]
      {
        let mut al: [Arg; 1] = [Arg { name: std::ptr::null_mut (), value: 0 }];

        XtSetArg! (al[0], XtNinput (), if NILP (new_value) { 1 } else { 0 }); /* C: True / False */
        XtSetValues ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).widget, al.as_mut_ptr (), 1);
      }
      #[cfg (all (not (feature = "use_gtk"), not (feature = "use_x_toolkit")))]
      {
        let window: Window = FRAME_X_WINDOW! (f);

        (*(FRAME_X_OUTPUT! (f) as *mut x_output)).wm_hints.input
          = if NILP (new_value) { 1 } else { 0 };   /* C: True / False */
        XSetWMHints (FRAME_X_DISPLAY! (f), window,
                     &raw mut (*(FRAME_X_OUTPUT! (f) as *mut x_output)).wm_hints);
      }
      FRAME_NO_ACCEPT_FOCUS! (f, !NILP (new_value));
    }
}

/* C 1071-1100: the doc comment C 1071-1079 and the function.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_override_redirect (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
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

        attributes.override_redirect = if NILP (new_value) { 0 } else { 1 }; /* C: False / True */
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
unsafe fn x_set_foreground_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let x: *mut x_output = FRAME_X_OUTPUT! (f) as *mut x_output;
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
unsafe fn x_set_background_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let x: *mut x_output = FRAME_X_OUTPUT! (f) as *mut x_output;
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
  /// C: lisp.h 612: `extern uintmax_t check_uinteger_max (Lisp_Object, uintmax_t);`
  pub fn check_uinteger_max (x: Lisp_Object, max: uintmax_t) -> uintmax_t;
  /// C: gtkutil.h 183: `extern void xg_set_background_color (struct frame *f, unsigned long bg);`
  pub fn xg_set_background_color (f: *mut frame, bg: c_ulong);
  /* C: Xlib.h 1806's `extern unsigned long XNextRequest (Display *);` moved to
     `include/xlib.rs` in the sixty-sixth `xterm` slice (P-90 5's move-back): xfns.rs reaches that
     mirror, and the row's second copy was one of `check-mirror-dupes.py`'s two failures.  */
  /// C: Xlib.h 1839-1842: `extern int XScreenNumberOfScreen (Screen *);`
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
  /// C: frame.h 1883: `extern void free_frame_menubar (struct frame *);`
  pub fn free_frame_menubar (f: *mut frame);
  /// C: dispextern.h 3906: `void clear_glyph_matrix (struct glyph_matrix *);`
  pub fn clear_glyph_matrix (matrix: *mut glyph_matrix);
  /* `fset_redisplay` (window.h 1168) is already declared by `include/window.rs`.  */
  /// C: xterm.h 1876: `extern void font_drop_xrender_surfaces (struct frame *f);`
  pub fn font_drop_xrender_surfaces (f: *mut frame);
  /// C: lisp.h 5120: `extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);`
  pub fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;
  /// C: dispextern.h 3940: `extern void clear_frame (struct frame *);`
  pub fn clear_frame (f: *mut frame);
  /// C: dispextern.h 3907: `void clear_current_matrices (struct frame *f);`
  pub fn clear_current_matrices (f: *mut frame);
  /// C: lisp.h 607: `extern int check_int_nonnegative (Lisp_Object);`
  pub fn check_int_nonnegative (value: Lisp_Object) -> c_int;
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
unsafe fn x_set_mouse_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let x: *mut x_output = FRAME_X_OUTPUT! (f) as *mut x_output;
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
      (*(FRAME_X_OUTPUT! (f) as *mut x_output)).current_cursor
        = cursor_data.cursor[mouse_cursor_text as usize];
      XDefineCursor (dpy, FRAME_X_WINDOW! (f),
                     (*(FRAME_X_OUTPUT! (f) as *mut x_output)).current_cursor);
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
unsafe fn x_set_cursor_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut fore_pixel: c_ulong;
  let mut pixel: c_ulong;
  let mut fore_pixel_allocated_p: bool = false;
  let mut pixel_allocated_p: bool = false;
  let x: *mut x_output = FRAME_X_OUTPUT! (f) as *mut x_output;

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
  unload_color (f, (*(FRAME_X_OUTPUT! (f) as *mut x_output)).border_pixel);
  (*(FRAME_X_OUTPUT! (f) as *mut x_output)).border_pixel = pix;

  #[cfg (feature = "use_x_toolkit")]
  {
    if !(*(FRAME_X_OUTPUT! (f) as *mut x_output)).widget.is_null () && (*f).border_width > 0
      {
        block_input ();
        XtVaSetValues ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).widget, XtNborderColor (),
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
unsafe fn x_set_border_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let pix: c_ulong;

  CHECK_STRING (arg);
  pix = x_decode_color (f, arg, BLACK_PIX_DEFAULT! (f) as c_int);
  x_set_border_pixel (f, pix);
  update_face_from_frame_parameter (f, Qborder_color (), arg);
}

/* C 1606-1610.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_cursor_type (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  set_frame_cursor_types (f, arg);
}

/* C 1612-1642.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_icon_type (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
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
unsafe fn x_set_icon_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
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

  if (*(FRAME_X_OUTPUT! (f) as *mut x_output)).icon_bitmap != 0
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
unsafe fn x_set_menu_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
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
        if FRAME_X_P (f) && (*(FRAME_X_OUTPUT! (f) as *mut x_output)).menubar_widget.is_null ()
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
          { (*(FRAME_X_OUTPUT! (f) as *mut x_output)).menubar_widget = std::ptr::null_mut (); }
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
unsafe fn x_set_tab_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
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
unsafe fn x_set_tool_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
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
            && (*(FRAME_X_OUTPUT! (f) as *mut x_output)).toolbar_widget.is_null ()
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
unsafe fn x_set_child_frame_border_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
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
unsafe fn x_set_internal_border_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
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
unsafe fn x_set_scroll_bar_foreground (f: *mut frame, value: Lisp_Object,
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

  if (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_foreground_pixel != (-1 as c_int) as c_ulong
    { unload_color (f, (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_foreground_pixel); }

  (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_foreground_pixel = pixel;
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
unsafe fn x_set_scroll_bar_background (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
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

  if (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_background_pixel != (-1 as c_int) as c_ulong
    {
      unload_color (f,
                    (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_background_pixel);
    }

  #[cfg (all (feature = "use_lucid", feature = "use_toolkit_scroll_bars"))]
  {
    /* Scrollbar shadow colors.  */
    if (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_top_shadow_pixel != -1
      {
        unload_color (f,
                      (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_top_shadow_pixel);
        (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_top_shadow_pixel = -1;
      }
    if (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_bottom_shadow_pixel != -1
      {
        unload_color (f,
                      (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_bottom_shadow_pixel);
        (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_bottom_shadow_pixel = -1;
      }
  }

  (*(FRAME_X_OUTPUT! (f) as *mut x_output)).scroll_bar_background_pixel = pixel;
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
  pub fn setup_coding_system (coding_system: Lisp_Object, coding: *mut coding_system);
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
unsafe fn x_explicitly_set_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
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
unsafe fn x_set_title (f: *mut frame, mut name: Lisp_Object, old_name: Lisp_Object)
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
unsafe fn x_set_alpha (f: *mut frame, mut arg: Lisp_Object, oldval: Lisp_Object)
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
unsafe fn x_set_use_frame_synchronization (f: *mut frame, arg: Lisp_Object,
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
  /// C: frame.h 1843: `extern void gui_set_frame_parameters (struct frame *, Lisp_Object);`
  pub fn gui_set_frame_parameters (f: *mut frame, alist: Lisp_Object);
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

  USE_SAFE_ALLOCA! (sa_avail, sa_count);
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
  SAFE_FREE! (sa_count);
}

/* C 4430-4621: the two `#else` arms of `x_window` - the GTK one (C 4431-4474) and the plain-X one
   (C 4476-4619).  The Xt arm (C 4172-4428) is outside this slice and opens the C-range table's
   next row.  `create_frame_xic` (C 2740's block) and `append_wm_protocols` (C 1573) are this
   file's own definitions, `setup_xi_event_mask` is slice 5's, and the four Xlib entry points
   neither the file nor the mirror declares stand in the use-site blocks below, each under the
   guard its arm has.  */

/// C: Xutil.h 632-636: `typedef struct { char *res_name; char *res_class; } XClassHint;` - no
/// mirror of this unit's graph declares it, so it stands here with the header's field order
/// (P-88 rule 1); C 4482/4552-4554 write the two fields.
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit")))]
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

/// C: X11/X.h 387, 393, 395, 396, 405, 431, 439 and 568, plus X11/Xutil.h 74, 75 and 134, every
/// value measured on this machine's headers: `InputOutput 1`, `CWBackPixel (1L<<1)`,
/// `CWBorderPixel (1L<<3)`, `CWBitGravity (1L<<4)`, `CWColormap (1L<<13)`, `StaticGravity 10`,
/// `NotUseful 0`, `GCLineWidth (1L<<4)`, `XNegative 0x0010`, `YNegative 0x0020` and
/// `InputHint (1L<<0)` - the eleven C 4486-4562's plain-X arm reads and the five `x11_constants`
/// does not carry (`CWEventMask`, `CWOverrideRedirect`, `GCForeground`, `GCBackground` and
/// `PropModeReplace` are the mirror's).
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const InputOutput: c_uint = 1;                        // X11/X.h 387
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const CWBackPixel: c_long = 1 << 1;                 // X11/X.h 393
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const CWBorderPixel: c_long = 1 << 3;               // X11/X.h 395
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const CWBitGravity: c_long = 1 << 4;                // X11/X.h 396
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const CWColormap: c_long = 1 << 13;                 // X11/X.h 405
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const StaticGravity: c_int = 10;                     // X11/X.h 431
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const NotUseful: c_int = 0;                          // X11/X.h 439
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const XNegative: c_int = 0x0010;                    // X11/Xutil.h 74
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const YNegative: c_int = 0x0020;                    // X11/Xutil.h 75
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
pub const InputHint: c_long = 1 << 0;                    // X11/Xutil.h 134

/* C: Xlib.h 2995-3003: `extern Window XCreateWindow (Display *, Window, int, int, unsigned int,
   unsigned int, unsigned int, int, unsigned int, Visual *, unsigned long,
   XSetWindowAttributes *);` - C 4512-4521's call.
   C: Xutil.h 642: `extern void XSetClassHint (Display *, Window, XClassHint *);` - C 4554's.
   C: Xlib.h 2896: `extern Status XSetWMProtocols (Display *, Window, Atom *, int);` - C 4572's.
   C: Xlib.h 1424: `extern Status XGetWindowAttributes (Display *, Window, XWindowAttributes *);`
   - C 4456-4457's.  Block comments, because a doc comment on an `extern` block is the
   `unused doc comment` warning the file's baseline already counts (slice 5's note).  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit")))]
unsafe extern "C" {
  pub fn XCreateWindow (dpy: *mut Display, parent: Window, x: c_int, y: c_int, width: c_uint,
			height: c_uint, border_width: c_uint, depth: c_int, class: c_uint,
			visual: *mut Visual, valuemask: c_ulong,
			attributes: *mut XSetWindowAttributes) -> Window;
  pub fn XSetClassHint (dpy: *mut Display, w: Window, class_hints: *mut XClassHint) -> c_int;
  pub fn XSetWMProtocols (dpy: *mut Display, w: Window, protocols: *mut Atom, count: c_int)
			  -> c_int;
  pub fn XGetWindowAttributes (dpy: *mut Display, w: Window, attr: *mut XWindowAttributes)
			       -> c_int;
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

  let mut class_hints: XClassHint;                                     // C 4482
  let mut attributes: XSetWindowAttributes;                            // C 4483
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
    let mut protocols: [Atom; 2];                                       // C 4569
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
  pub fn x_wm_set_icon_position (f: *mut frame, icon_x: c_int, icon_y: c_int);
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
					 std::ptr::null::<c_char> (),
					 std::ptr::null::<c_char> (), RES_TYPE_NUMBER);
  icon_y = gui_frame_get_and_record_arg (f, parms, Qicon_top (),        // C 4634
					 std::ptr::null::<c_char> (),
					 std::ptr::null::<c_char> (), RES_TYPE_NUMBER);
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
      icon_xval = check_integer_range (icon_x, c_int::MIN as intmax_t,  // C 4665
				       c_int::MAX as intmax_t) as c_int;
      icon_yval = check_integer_range (icon_y, c_int::MIN as intmax_t,  // C 4666
				       c_int::MAX as intmax_t) as c_int;
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
