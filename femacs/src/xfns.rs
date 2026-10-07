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
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit")))]
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
  if (*f).terminal.is_null ()                                           // C 4797 (FRAME_LIVE_P)
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
/// (`lib/include/intprops.rs`) that no unit includes - the `src/xterm.rs` 18944 shape.
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
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
   `docs/ports/xfns.md` owed row names `include/lisp.rs`).  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe extern "C" {
  pub fn xlispstrdup (string: Lisp_Object) -> *mut c_char;
}

/* C: `frame.c` 6180 defines `validate_x_resource_name` and no header declares it - the
   reference calls it from xfns.c 4329/4550, pgtkfns.c 156, xsmfns.c 377 and frame.c 6263 the
   same way.  Both arms of this file call it, so the row stands under the whole family's guard
   (P-72 rule 4; the owed row for the declaring header is in `docs/ports/xfns.md`).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn validate_x_resource_name ();
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
  pub fn delete_frame (frame: Lisp_Object, force: Lisp_Object) -> Lisp_Object;
  pub fn gui_figure_window_size (f: *mut frame, parms: Lisp_Object, default_pixelwise: bool,
				 window_pixelwise: bool) -> c_long;
  pub fn x_iconify_frame (f: *mut frame);
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
  pub fn make_frame (mini_p: bool) -> *mut frame;
  pub fn make_minibuffer_frame () -> *mut frame;
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
    (*((*f).output_data.x as *mut x_output)).scrollbar_background_css_provider
      = gtk_css_provider_new ();                                        // C 5244-5245
    (*((*f).output_data.x as *mut x_output)).scrollbar_foreground_css_provider
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
