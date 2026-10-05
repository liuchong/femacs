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
   `Xm/DialogS.h`, `Xm/FileSB.h`, `Xm/List.h`, `Xm/TextF.h`, `Xm/MwmUtil.h`; USE_MOTIF, C 102-107) and `"../lwlib/xlwmenu.h"` (USE_LUCID, C 111).  The two Emacs-side headers have
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

/* Emacs' own entry points this slice calls, from headers whose mirrors do not carry them.  */
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
  /// C: `<gtk/gtkwidget.h>`: `extern void gtk_widget_set_app_paintable (GtkWidget *widget,
  /// gboolean app_paintable);` - GTK's own header, with no file in the reference tree and no
  /// mirror; this machine has no GTK tree, so the citation is the header and not a line.
  pub fn gtk_widget_set_app_paintable (widget: *mut GtkWidget, app_paintable: c_int);
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
   toolchain's own and this machine has no GTK tree (P-86 ③⑵'s second tier: `text-only`), so the
   four names the `USE_GTK`/`HAVE_GTK3` arms reach are declared here, each with the SDK header it
   comes from (P-88 ①).  `GtkWidget` is also the type `include/xterm.rs`'s `FRAME_GTK_OUTER_WIDGET`
   names bare (xterm.rs 378 lists it among that mirror's un-mirrored names).  */
/// C: gtk/gtkwidget.h: `typedef struct _GtkWidget GtkWidget;`
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub struct GtkWidget
{
  _private: [u8; 0],
}
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
/// C: gtk/gtkwidget.h's `GtkWidgetClass`.  C 797-798 reads exactly one member of it; the port
/// carries that member and owes the record's other members and this one's offset to a GTK
/// toolchain measurement (P-80's proof requirement) - `docs/ports/xfns.md` records it.
#[cfg (all (feature = "have_x_windows", feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct GtkWidgetClass
{
  /// C: gtk/gtkwidget.h: `void (*style_updated) (GtkWidget *widget);`
  pub style_updated: Option<unsafe extern "C" fn (widget: *mut GtkWidget) -> ()>,
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
