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

/* Emacs' own entry points the arms above call, from headers whose mirrors do not carry them
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
  /// C: Xlib.h 1806: `extern unsigned long XNextRequest (Display *);`
  pub fn XNextRequest (display: *mut Display) -> c_ulong;
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

/* Emacs' own entry points this batch calls (P-72 rule 4).  */
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
