// femacs: port of oldXMenu/Internal.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The three non-`static` objects and the twelve functions carry
// `#[unsafe(no_mangle)]` (PORTING.md 0.2); the two file-scope `static` objects stay plain
// `static mut`.
//
// **The file gate is `feature = "have_x_windows"`** (P-86 clause 8, the build's condition rather
// than one the C text spells).  `configure.ac` 7287-7295 selects this library - the arm that sets
// `HAVE_OLDXMENU=yes` and `LIBXMENU='$(oldXMenudir)/libXMenu11.a'` is `HAVE_X11 = yes` with
// `HAVE_GTK != yes` and `USE_X_TOOLKIT = none` - and `oldXMenu/Makefile.in`'s `OBJS` block (60-85)
// lists every object of the directory unconditionally, so the reference text carries **no macro
// of its own** for this file: the condition is a configure result and the feature is the repo's
// name for the X11 window system (`HAVE_X_WINDOWS`), the gate `src/xfns.rs`'s unit uses.  P-86
// clause 8 asks for the reference's own macro lower-cased; this file has none, which the ledger's
// `## Deviations` 1 records.
//
// `HAVE_X11`, `HAVE_X_WINDOWS` and `USE_X_TOOLKIT` are `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv`, so the frozen configuration compiles not one
// line of this unit and `xtask build-tu` leaves the empty object - and this machine has no X11
// *program* to run it against.  The `<X11/*>` declarations below are therefore measured against
// `/opt/homebrew/include` (xorgproto 2025.1 and libX11 1.8.13) with clang rather than taken from a
// build; every value, size and offset this unit carries is one of those readings, and the
// commands are in the ledger's `## Evidence`.
//
// The `<X11/Xlib.h>` surface this file reads is the shared mirror `include/xlib.rs` where that
// mirror carries the name (P-90 clause 1); what it does not carry yet is declared in this unit at
// the use site, with C's own prototype in a `/// C:` comment (P-88 clause 1 / P-55).  The ledger's
// `## Owed` names the move back into the mirror.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_mut, unused_parens,
         unused_comparisons, clippy::all)]

/* C 1-49: the file's opening block - the MIT and FSF notices and the XMenu description comment -
   is not transcribed (PORTING.md 0.1-19; the ledger's `## C line ranges` records the range).  */

/* C 50: `#include "XMenuInt.h"`.  Its mirror holds the declarations of the twelve functions this
   unit *defines*, and Rust's "declaration + definition" of one name in one scope is `E0428`, while
   an explicit item shadows a glob import - which is C's pair (PORTING.md 0.1-10 / P-21, the
   `lwlib-Xaw.rs` shape).  So the mirror is included inside a module that is glob-imported, and the
   `#[macro_use]` is what carries `min`/`max`/`abs` across (a glob import does not import
   `macro_rules!` items).  The ledger's `## Deviations` 2 records it, and `## Owed` 1 names the
   mirror-side move that would let the wrapper go.  */
#[cfg (feature = "have_x_windows")]
#[macro_use]
mod xmenu_int {
  use super::*;
  include!("include/XMenuInt.rs");             // C 50: `#include "XMenuInt.h"`
}
#[cfg (feature = "have_x_windows")]
use xmenu_int::*;

/* XMenuInt.h 46: `#include <X11/Xlib.h>` - the header this file reaches the X11 surface through.
   The mirror that owns that surface is `include/xlib.rs` (P-90 clause 1), and the names below it
   lacks are declared in this unit at their use sites (P-88 clause 1).  */
#[cfg (feature = "have_x_windows")]
include!("../src/include/xlib.rs");            // XMenuInt.h 46: `#include <X11/Xlib.h>`
/// Xlib's `None`/`CopyFromParent`/`InputOnly`/`CWOverrideRedirect` come from the mirror's own
/// `x11_constants` module; the four this file names are imported here (PORTING.md 0.1-9: a unit
/// takes the names it needs from the module rather than glob-importing `None` over the prelude).
#[cfg (feature = "have_x_windows")]
use x11_constants::{CopyFromParent, InputOnly, CWOverrideRedirect};

/* The C fixed-width spellings.  Every unit that includes `lisp.h` gets them from that mirror's
   crate-root `use std::ffi::{…}`; XMenu's headers include no `lisp.h`, so this unit brings the
   set itself, the shape the `lib-src` and `exec` units use.  */
#[cfg (feature = "have_x_windows")]
use std::ffi::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};

/* ------------------------------------------------------------------------------------------
   The X11 surface `include/xlib.rs` does not carry yet (P-88 clause 1: no mirror holds these
   names, and this file is their only consumer today).  Every number below is measured with clang
   against this machine's headers; the `/// C:` line keeps the header's own text and line.
   ------------------------------------------------------------------------------------------ */

/// C: Xlib.h 80-81: `#define True 1` / `#define False 0` (the header spells them with `#define`,
/// and `Bool` is Xlib.h 78's `int`).  Measured: one and zero.
#[cfg (feature = "have_x_windows")]
pub const True: Bool = 1;
#[cfg (feature = "have_x_windows")]
pub const False: Bool = 0;

/// C: X.h 387-388: `#define InputOutput 1` / `#define InputOnly 2`.  Measured: 1 and 2.
#[cfg (feature = "have_x_windows")]
pub const InputOutput: c_int = 1;

/// C: X.h 392-413: the `CW*` value-mask bits `XCreateWindow`'s `valuemask` and
/// `XConfigureWindow`'s `change_mask` are built from.  C spells the pair XCreateWindow takes with
/// `1L` (a `long`) and the quartet XConfigureWindow takes with `1` (an `int`), so the port keeps
/// both spellings and the conversions the C expressions rely on are spelled out at their two
/// sites.  Measured: `CWBackPixmap` 1, `CWBorderPixel` 8, `CWX` 1, `CWY` 2, `CWWidth` 4,
/// `CWHeight` 8 (`CWOverrideRedirect` 512 is the mirror's `x11_constants` row).
#[cfg (feature = "have_x_windows")]
pub const CWBackPixmap: c_long = 1 << 0;
#[cfg (feature = "have_x_windows")]
pub const CWBorderPixel: c_long = 1 << 3;
#[cfg (feature = "have_x_windows")]
pub const CWX: c_int = 1 << 0;
#[cfg (feature = "have_x_windows")]
pub const CWY: c_int = 1 << 1;
#[cfg (feature = "have_x_windows")]
pub const CWWidth: c_int = 1 << 2;
#[cfg (feature = "have_x_windows")]
pub const CWHeight: c_int = 1 << 3;

/// C: Xlib.h 1003-1021: `typedef struct { short lbearing; short rbearing; short width; short
/// ascent; short descent; unsigned short attributes; } XCharStruct;` - the per-character metric
/// `XFontStruct`'s `max_bounds` is.  Measured `[sizeof=12, align=2]` with `lbearing` 0, `width` 4,
/// `ascent` 6, `descent` 8, `attributes` 10.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XCharStruct
{
  pub lbearing: c_short,
  pub rbearing: c_short,
  pub width: c_short,
  pub ascent: c_short,
  pub descent: c_short,
  pub attributes: c_ushort,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XCharStruct> () == 12);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XCharStruct> () == 2);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XCharStruct, ascent) == 6);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XCharStruct, descent) == 8);

/// C: Xlib.h 1026-1029: `typedef struct { Atom name; unsigned long card32; } XFontProp;` - the
/// font-property record `XFontStruct` points at.  Measured `[sizeof=16, align=8]`, `name` 0,
/// `card32` 8.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XFontProp
{
  pub name: Atom,
  pub card32: c_ulong,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XFontProp> () == 16);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XFontProp, card32) == 8);

/// C: Xlib.h 1031-1048: `typedef struct { XExtData *ext_data; Font fid; unsigned direction; …
/// XCharStruct min_bounds; XCharStruct max_bounds; XCharStruct *per_char; int ascent; int
/// descent; } XFontStruct;` - the record whose `max_bounds.ascent`/`.descent` C 675-676, 798 and
/// 894 consume.  Carried whole (xlib.rs's `Screen` reading of a public record); measured
/// `[sizeof=96, align=8]` with `fid` 8, `direction` 16, `all_chars_exist` 36, `properties` 48,
/// `min_bounds` 56, `max_bounds` 68, `per_char` 80, `ascent` 88, `descent` 92.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XFontStruct
{
  pub ext_data: *mut XExtData,
  pub fid: Font,
  pub direction: c_uint,
  pub min_char_or_byte2: c_uint,
  pub max_char_or_byte2: c_uint,
  pub min_byte1: c_uint,
  pub max_byte1: c_uint,
  pub all_chars_exist: Bool,
  pub default_char: c_uint,
  pub n_properties: c_int,
  pub properties: *mut XFontProp,
  pub min_bounds: XCharStruct,
  pub max_bounds: XCharStruct,
  pub per_char: *mut XCharStruct,
  pub ascent: c_int,
  pub descent: c_int,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XFontStruct> () == 96);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XFontStruct> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XFontStruct, max_bounds) == 68);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XFontStruct, ascent) == 88);

/// C: Xlib.h 286-302: `typedef struct { Pixmap background_pixmap; unsigned long background_pixel;
/// … Bool override_redirect; Colormap colormap; Cursor cursor; } XSetWindowAttributes;` - the
/// record `_XMWinQueFlush` fills three fields of and hands to `XCreateWindow`.  Carried whole;
/// measured `[sizeof=112, align=8]` with `background_pixmap` 0, `border_pixmap` 16, `border_pixel`
/// 24, `bit_gravity` 32, `override_redirect` 88, `colormap` 96, `cursor` 104.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
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
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XSetWindowAttributes> () == 112);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XSetWindowAttributes> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XSetWindowAttributes, border_pixel) == 24);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (
  core::mem::offset_of! (XSetWindowAttributes, override_redirect) == 88);

/// C: Xlib.h 395-401: `typedef struct { int x, y; int width, height; int border_width; Window
/// sibling; int stack_mode; } XWindowChanges;` - the record `_XMRecomputePane` and
/// `_XMRecomputeSelection` pass to `XConfigureWindow`.  Carried whole; measured `[sizeof=40,
/// align=8]` with `x` 0, `y` 4, `width` 8, `height` 12, `border_width` 16, `sibling` 24,
/// `stack_mode` 32.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XWindowChanges
{
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub sibling: Window,
  pub stack_mode: c_int,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XWindowChanges> () == 40);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XWindowChanges> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XWindowChanges, width) == 8);

/* The entry points `include/xlib.rs` does not carry yet, each with the header's own prototype
   (P-88 clause 1).  `XDefaultScreen`, `XDisplayWidth` and `XDisplayHeight` are the three *macros*
   of that list: C's `default_screen`/`width`/`height` bodies read Xlib's private display record
   (`_XPrivDisplay`, Xlib.h 85-119), which no mirror here carries, so the port calls those three
   entry points - the same values, and the mirror's own `DefaultScreenOfDisplay` reading of the
   same header (xlib.rs's degradation note).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: Xlib.h 1632-1645: `extern Window XCreateWindow (Display*, Window, int, int, unsigned int,
  /// unsigned int, unsigned int, int, unsigned int, Visual*, unsigned long,
  /// XSetWindowAttributes*);`
  pub fn XCreateWindow (display: *mut Display, parent: Window, x: c_int, y: c_int,
                        width: c_uint, height: c_uint, border_width: c_uint, depth: c_int,
                        class: c_uint, visual: *mut Visual, valuemask: c_ulong,
                        attributes: *mut XSetWindowAttributes) -> Window;
  /// C: Xlib.h 3139-3142: `extern int XSelectInput (Display*, Window, long);`
  pub fn XSelectInput (display: *mut Display, w: Window, event_mask: c_long) -> c_int;
  /// C: Xlib.h 2517-2518: `extern int XFlush (Display*);`
  pub fn XFlush (display: *mut Display) -> c_int;
  /// C: Xlib.h 2174-2179: `extern int XConfigureWindow (Display*, Window, unsigned int,
  /// XWindowChanges*);`
  pub fn XConfigureWindow (display: *mut Display, w: Window, value_mask: c_uint,
                           values: *mut XWindowChanges) -> c_int;
  /// C: Xlib.h 2165-2168: `extern int XClearWindow (Display*, Window);`
  pub fn XClearWindow (display: *mut Display, w: Window) -> c_int;
  /// C: Xlib.h 2499-2508: `extern int XFillRectangle (Display*, Drawable, GC, int, int,
  /// unsigned int, unsigned int);`
  pub fn XFillRectangle (display: *mut Display, d: Drawable, gc: GC, x: c_int, y: c_int,
                         width: c_uint, height: c_uint) -> c_int;
  /// C: Xlib.h 2414-2422: `extern int XDrawString (Display*, Drawable, GC, int, int,
  /// _Xconst char*, int);`
  pub fn XDrawString (display: *mut Display, d: Drawable, gc: GC, x: c_int, y: c_int,
                      string: *const c_char, length: c_int) -> c_int;
  /// C: Xlib.h 2352-2360: `extern int XDrawLine (Display*, Drawable, GC, int, int, int, int);`
  pub fn XDrawLine (display: *mut Display, d: Drawable, gc: GC, x1: c_int, y1: c_int,
                    x2: c_int, y2: c_int) -> c_int;
  /// C: Xlib.h 2388-2396: `extern int XDrawRectangle (Display*, Drawable, GC, int, int,
  /// unsigned int, unsigned int);`
  pub fn XDrawRectangle (display: *mut Display, d: Drawable, gc: GC, x: c_int, y: c_int,
                         width: c_uint, height: c_uint) -> c_int;
  /// C: Xlib.h 2155-2163: `extern int XClearArea (Display*, Window, int, int, unsigned int,
  /// unsigned int, Bool);`
  pub fn XClearArea (display: *mut Display, w: Window, x: c_int, y: c_int, width: c_uint,
                     height: c_uint, exposures: Bool) -> c_int;
  /// C: Xlib.h 2237-2239: `extern int XDefaultScreen (Display*);` - `DefaultScreen`'s entry.
  pub fn XDefaultScreen (display: *mut Display) -> c_int;
  /// C: Xlib.h 2302-2305: `extern int XDisplayWidth (Display*, int);` - `DisplayWidth`'s entry.
  pub fn XDisplayWidth (display: *mut Display, screen_number: c_int) -> c_int;
  /// C: Xlib.h 2281-2284: `extern int XDisplayHeight (Display*, int);` - `DisplayHeight`'s entry.
  pub fn XDisplayHeight (display: *mut Display, screen_number: c_int) -> c_int;
}

/// C: Xlib.h 89: `#define DefaultScreen(dpy) (((_XPrivDisplay)(dpy))->default_screen)` - C's call
/// shape is a function call, so the port's is one too (the xlib.rs `XGetPixel` note on why a
/// `macro_rules!` cannot be called that way), over the entry point the macro's expansion reads.
#[cfg (feature = "have_x_windows")]
#[inline (always)]
pub unsafe fn DefaultScreen (dpy: *mut Display) -> c_int { XDefaultScreen (dpy) }
/// C: Xlib.h 97: `#define DisplayWidth(dpy, scr) (ScreenOfDisplay(dpy,scr)->width)`.
#[cfg (feature = "have_x_windows")]
#[inline (always)]
pub unsafe fn DisplayWidth (dpy: *mut Display, scr: c_int) -> c_int { XDisplayWidth (dpy, scr) }
/// C: Xlib.h 98: `#define DisplayHeight(dpy, scr) (ScreenOfDisplay(dpy,scr)->height)`.
#[cfg (feature = "have_x_windows")]
#[inline (always)]
pub unsafe fn DisplayHeight (dpy: *mut Display, scr: c_int) -> c_int { XDisplayHeight (dpy, scr) }

/* ------------------------------------------------------------------------------------------
   The file's definitions, in C's order.
   ------------------------------------------------------------------------------------------ */

/* C 52-56: the queue sizes.  */
/// C 55: `#define S_QUE_SIZE	300`
#[cfg (feature = "have_x_windows")]
pub const S_QUE_SIZE: c_int = 300;
/// C 56: `#define P_QUE_SIZE	20`
#[cfg (feature = "have_x_windows")]
pub const P_QUE_SIZE: c_int = 20;

/* C 58-73: `XMWinQue` - the internal window creation queue datatype, defined in this file (not in
   the header) and used by the four queue routines below.  */
/// C 60-72: `typedef struct _xmwinquedef { int sq_size; XMSelect *sq[S_QUE_SIZE]; XMSelect
/// **sq_ptr; int pq_size; XMPane *pq[P_QUE_SIZE]; XMPane **pq_ptr; } XMWinQue;`
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct _xmwinquedef
{
  pub sq_size: c_int,
  pub sq: [*mut XMSelect; S_QUE_SIZE as usize],
  pub sq_ptr: *mut *mut XMSelect,
  pub pq_size: c_int,
  pub pq: [*mut XMPane; P_QUE_SIZE as usize],
  pub pq_ptr: *mut *mut XMPane,
}
/// C 60-72: the typedef name.
#[cfg (feature = "have_x_windows")]
pub type XMWinQue = _xmwinquedef;

/* C 75-78: the internal static window creation queue.  C's `static` is internal linkage, so no
   `#[unsafe(no_mangle)]`.  The record's zero initialisation is C's `static` rule (0.1-13: Rust
   has no uninitialised `static`, and the null pointers are what C's `static` storage gives).  */
/// C 77: `static Bool _XMWinQueIsInit = False;`
#[cfg (feature = "have_x_windows")]
static mut _XMWinQueIsInit: Bool = False;
/// C 78: `static XMWinQue _XMWinQue;`
#[cfg (feature = "have_x_windows")]
static mut _XMWinQue: XMWinQue =
  XMWinQue {
    sq_size: 0,
    sq: [core::ptr::null_mut (); S_QUE_SIZE as usize],
    sq_ptr: core::ptr::null_mut (),
    pq_size: 0,
    pq: [core::ptr::null_mut (); P_QUE_SIZE as usize],
    pq_ptr: core::ptr::null_mut (),
  };

/* C 80-83: the global XMenu error code.  */
/// C 82: `int _XMErrorCode = XME_NO_ERROR;`
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut _XMErrorCode: c_int = XME_NO_ERROR;

/* C 84-103: the error code description strings.  C's array is `char const *const`
   (read-only); a Rust `static` of raw pointers has to be `static mut` - a non-`mut` `static`
   demands `Sync` (`E0277`), which `*const c_char` is not - the `keyboard.rs`
   `lispy_accent_keys`/`modifier_names` reading of the same C construct (the ledger's
   `## Deviations` 3).  */
/// C 85-103: `char const *const _XMErrorList[XME_CODE_COUNT] = { "No error", … };`
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut _XMErrorList: [*const c_char; XME_CODE_COUNT as usize] = [
  c"No error".as_ptr (),                        /* XME_NO_ERROR */
  c"Menu not initialized".as_ptr (),            /* XME_NOT_INIT */
  c"Argument out of bounds".as_ptr (),          /* XME_ARG_BOUNDS */
  c"Pane not found".as_ptr (),                  /* XME_P_NOT_FOUND */
  c"Selection not found".as_ptr (),             /* XME_S_NOT_FOUND */
  c"Invalid menu style parameter".as_ptr (),    /* XME_STYLE_PARAM */
  c"Unable to grab mouse".as_ptr (),            /* XME_GRAB_MOUSE */
  c"Unable to interpret locator".as_ptr (),     /* XME_INTERP_LOC */
  c"Unable to calloc memory".as_ptr (),         /* XME_CALLOC */
  c"Unable to create XAssocTable".as_ptr (),    /* XME_CREATE_ASSOC */
  c"Unable to store bitmap".as_ptr (),          /* XME_STORE_BITMAP */
  c"Unable to make tile pixmaps".as_ptr (),     /* XME_MAKE_TILES */
  c"Unable to make pixmap".as_ptr (),           /* XME_MAKE_PIXMAP */
  c"Unable to create cursor".as_ptr (),         /* XME_CREATE_CURSOR */
  c"Unable to open font".as_ptr (),             /* XME_OPEN_FONT */
  c"Unable to create windows".as_ptr (),        /* XME_CREATE_WINDOW */
  c"Unable to create transparencies".as_ptr (), /* XME_CREATE_TRANSP */
];

/* C 105-108: `_XMEventHandler` - internal event handler variable.  A C function pointer is
   nullable, so the port stores an `Option` (P-27).  */
/// C 108: `int (*_XMEventHandler)(XEvent*) = NULL;`
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut _XMEventHandler: Option<unsafe extern "C" fn (event: *mut XEvent) -> c_int> = None;

/* C 110-142: `_XMWinQueInit` - Internal routine to initialize the window queue.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMWinQueInit ()
{
  /* C 119-121: "If the queue is not initialized initialize it."  */
  if !(_XMWinQueIsInit != 0)
    {
      /* C 123-125: "Blank the queue structure."  */
      let mut i: c_int = 0;

      /* C 128-129: `for (i = 0; i < S_QUE_SIZE; i++) _XMWinQue.sq[i] = 0;`  */
      while i < S_QUE_SIZE
        {
          _XMWinQue.sq[i as usize] = core::ptr::null_mut ();
          i += 1;
        }

      /* C 131-132: `for (i = 0; i < P_QUE_SIZE; i++) _XMWinQue.pq[i] = 0;` - C's second loop
         re-initialises `i` in its own for-header.  */
      i = 0;
      while i < P_QUE_SIZE
        {
          _XMWinQue.pq[i as usize] = core::ptr::null_mut ();
          i += 1;
        }

      /* C 134: `_XMWinQue.sq_size = _XMWinQue.pq_size = 0;`  */
      _XMWinQue.sq_size = 0;
      _XMWinQue.pq_size = 0;

      /* C 136-138: "Initialize the next free location pointers."  */
      _XMWinQue.sq_ptr = _XMWinQue.sq.as_mut_ptr ();
      _XMWinQue.pq_ptr = _XMWinQue.pq.as_mut_ptr ();
    }
}

/* C 144-176: `_XMWinQueAddPane` - Internal routine to add a pane to the pane window
   queue.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMWinQueAddPane (display: *mut Display, menu: *mut XMenu,
                                           p_ptr: *mut XMPane) -> c_int
{
  /* C 156-158: "If the queue is currently full then flush it."  */
  if _XMWinQue.pq_size == P_QUE_SIZE
    {
      if _XMWinQueFlush (display, menu, 0 as *mut XMPane, 0 as *mut XMSelect) == _FAILURE
        { return _FAILURE; }
    }

  /* C 163-166: "Insert the new XMPane pointer and increment the queue pointer and the queue
     size."  */
  *_XMWinQue.pq_ptr = p_ptr;
  _XMWinQue.pq_ptr = _XMWinQue.pq_ptr.offset (1);
  _XMWinQue.pq_size += 1;

  /* C 171-173: "All went well, return successfully."  */
  _XMErrorCode = XME_NO_ERROR;
  return _SUCCESS;
}

/* C 178-210: `_XMWinQueAddSelection` - Internal routine to add a selection to the selection
   window queue.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMWinQueAddSelection (display: *mut Display, menu: *mut XMenu,
                                                s_ptr: *mut XMSelect) -> c_int
{
  /* C 190-192: "If this entry will overflow the queue then flush it."  */
  if _XMWinQue.sq_size == S_QUE_SIZE
    {
      if _XMWinQueFlush (display, menu, 0 as *mut XMPane, 0 as *mut XMSelect) == _FAILURE
        { return _FAILURE; }
    }

  /* C 197-200: "Insert the new XMSelect pointer and increment the queue pointer and the queue
     size."  */
  *_XMWinQue.sq_ptr = s_ptr;
  _XMWinQue.sq_ptr = _XMWinQue.sq_ptr.offset (1);
  _XMWinQue.sq_size += 1;

  /* C 205-207: "All went well, return successfully."  */
  _XMErrorCode = XME_NO_ERROR;
  return _SUCCESS;
}

/* C 212-350: `_XMWinQueFlush` - Internal routine to flush the pane and selection window
   queues.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMWinQueFlush (display: *mut Display, menu: *mut XMenu,
                                         pane: *mut XMPane, sel: *mut XMSelect) -> c_int
{
  let mut pq_index: c_int;                      /* Pane queue index. */
  let mut sq_index: c_int;                      /* Selection queue index. */
  let mut p_ptr: *mut XMPane;                   /* XMPane pointer. */
  let mut s_ptr: *mut XMSelect;                 /* XMSelect pointer. */
  let mut valuemask: c_ulong;                   /* Which attributes to set. */
  let mut attributes_buf: XSetWindowAttributes = core::mem::zeroed (); /* Attributes to be set. */
  let attributes: *mut XSetWindowAttributes = &raw mut attributes_buf;

  /* C 232-234: "If the pane window queue is not empty..."  */
  if _XMWinQue.pq_size > 0
    {
      /* C 237-239: "set up attributes for pane window to be created."  */
      valuemask = (CWBackPixmap | CWBorderPixel | CWOverrideRedirect) as c_ulong;
      (*attributes).border_pixel = (*menu).p_bdr_color;
      (*attributes).background_pixmap = (*menu).inact_pixmap;
      (*attributes).override_redirect = True;

      /* C 245-249: "Create all the pending panes in order, so that the current pane will be on
         top, with the others stacked appropriately under it."  */
      pq_index = _XMWinQue.pq_size - 1;
      while pq_index >= 0
        {
          p_ptr = _XMWinQue.pq[pq_index as usize];    /* Retrieve next pane. */
          if p_ptr == pane { break; }
          (*p_ptr).window = XCreateWindow (display,
                                           (*menu).parent,
                                           (*p_ptr).window_x,
                                           (*p_ptr).window_y,
                                           (*p_ptr).window_w as c_uint,
                                           (*p_ptr).window_h as c_uint,
                                           (*menu).p_bdr_width as c_uint,
                                           CopyFromParent as c_int,
                                           InputOutput as c_uint,
                                           CopyFromParent as *mut Visual,
                                           valuemask,
                                           attributes);
          XMakeAssoc (display, (*menu).assoc_tab, (*p_ptr).window, p_ptr as *mut c_void);
          XSelectInput (display, (*p_ptr).window, (*menu).p_events as c_long);
          pq_index -= 1;
        }
      pq_index = 0;
      while pq_index < _XMWinQue.pq_size
        {
          p_ptr = _XMWinQue.pq[pq_index as usize];    /* Retrieve next pane. */
          (*p_ptr).window = XCreateWindow (display,
                                           (*menu).parent,
                                           (*p_ptr).window_x,
                                           (*p_ptr).window_y,
                                           (*p_ptr).window_w as c_uint,
                                           (*p_ptr).window_h as c_uint,
                                           (*menu).p_bdr_width as c_uint,
                                           CopyFromParent as c_int,
                                           InputOutput as c_uint,
                                           CopyFromParent as *mut Visual,
                                           valuemask,
                                           attributes);
          XMakeAssoc (display, (*menu).assoc_tab, (*p_ptr).window, p_ptr as *mut c_void);
          XSelectInput (display, (*p_ptr).window, (*menu).p_events as c_long);
          if p_ptr == pane { break; }
          pq_index += 1;
        }

      /* C 293-295: "Reset the pane queue pointer and size."  */
      _XMWinQue.pq_size = 0;
      _XMWinQue.pq_ptr = _XMWinQue.pq.as_mut_ptr ();
    }

  /* C 300-302: "If the selection window queue is not empty..."  */
  if _XMWinQue.sq_size > 0
    {
      sq_index = 0;
      while sq_index < _XMWinQue.sq_size
        {
          /* C 307-309: "Retrieve the XMSelect pointer."  */
          s_ptr = _XMWinQue.sq[sq_index as usize];
          (*s_ptr).window = XCreateWindow (display,
                                           (*(*s_ptr).parent_p).window,
                                           (*s_ptr).window_x,
                                           (*s_ptr).window_y,
                                           (*s_ptr).window_w as c_uint,
                                           (*s_ptr).window_h as c_uint,
                                           0,                /* border width*/
                                           CopyFromParent as c_int,
                                           InputOnly as c_uint,
                                           CopyFromParent as *mut Visual,
                                           0,
                                           attributes);

          /* C 324-328: "Insert the new window id and its associated XMSelect structure into the
             association table."  */
          XMakeAssoc (display, (*menu).assoc_tab, (*s_ptr).window, s_ptr as *mut c_void);
          XSelectInput (display, (*s_ptr).window, (*menu).s_events as c_long);
          sq_index += 1;
        }

      /* C 333-335: "Reset the selection queue pointer and size."  */
      _XMWinQue.sq_size = 0;
      _XMWinQue.sq_ptr = _XMWinQue.sq.as_mut_ptr ();
    }

  /* C 340-342: "Flush X's internal queues."  */
  XFlush (display);

  /* C 345-347: "All went well, return successfully."  */
  _XMErrorCode = XME_NO_ERROR;
  return _SUCCESS;
}

/* C 352-385: `_XMGetPanePtr` - Given a menu pointer and a pane index number, return a pane
   pointer that points to the indexed pane.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMGetPanePtr (menu: *mut XMenu, p_num: c_int) -> *mut XMPane
{
  let mut p_ptr: *mut XMPane;                   /* Pane pointer to be returned. */
  let mut i: c_int;                             /* Loop counter. */

  /* C 366-368: "Is the pane number out of range?"  */
  if (p_num < 0) || (p_num > ((*menu).p_count - 1))
    {
      _XMErrorCode = XME_P_NOT_FOUND;
      return core::ptr::null_mut ();
    }

  /* C 374-378: "Find the right pane."  */
  p_ptr = (*(*menu).p_list).next;
  i = 0;
  while i < p_num
    {
      p_ptr = (*p_ptr).next;
      i += 1;
    }

  /* C 380-382: "Return successfully."  */
  _XMErrorCode = XME_NO_ERROR;
  return p_ptr;
}

/* C 387-421: `_XMGetSelectionPtr` - Given pane pointer and a selection index number, return a
   selection pointer that points to the indexed selection.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMGetSelectionPtr (p_ptr: *mut XMPane, s_num: c_int) -> *mut XMSelect
{
  let mut s_ptr: *mut XMSelect;                 /* Selection pointer to be returned. */
  let mut i: c_int;                             /* Loop counter. */

  /* C 402-404: "Is the selection number out of range?"  */
  if (s_num < 0) || (s_num > ((*p_ptr).s_count - 1))
    {
      _XMErrorCode = XME_S_NOT_FOUND;
      return core::ptr::null_mut ();
    }

  /* C 410-414: "Find the right selection."  */
  s_ptr = (*(*p_ptr).s_list).next;
  i = 0;
  while i < s_num
    {
      s_ptr = (*s_ptr).next;
      i += 1;
    }

  /* C 416-418: "Return successfully."  */
  _XMErrorCode = XME_NO_ERROR;
  return s_ptr;
}

/* C 423-537: `_XMRecomputeGlobals` - Internal subroutine to recompute menu wide global
   values.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMRecomputeGlobals (display: *mut Display, menu: *mut XMenu)
{
  let mut p_ptr: *mut XMPane;                   /* Pane pointer. */
  let mut s_ptr: *mut XMSelect;                 /* Selection pointer. */

  let mut max_p_label: c_int = 0;               /* Maximum pane label width. */
  let mut max_s_label: c_int = 0;               /* Maximum selection label width. */
  let mut s_count: c_int = 0;                   /* Maximum selection count. */

  let mut p_s_pad: c_int;                       /* Pane <-> selection padding. */
  let mut p_s_diff: c_int;                      /* Pane <-> selection separation. */

  let mut p_height: c_int;                      /* Pane window height. */
  let mut p_width: c_int;                       /* Pane window width. */
  let mut s_width: c_int;                       /* Selection window width. */

  let mut screen: c_int;                        /* DefaultScreen holder. */

  /* C 450-452: "For each pane..."  */
  p_ptr = (*(*menu).p_list).next;
  while p_ptr != (*menu).p_list
    {
      /* C 459-462: "Recompute maximum pane label width."  */
      max_p_label = max! (max_p_label, (*p_ptr).label_width);

      /* C 464-467: "Recompute maximum selection count."  */
      s_count = max! (s_count, (*p_ptr).s_count);

      /* C 469-471: "For each selection in the current pane..."  */
      s_ptr = (*(*p_ptr).s_list).next;
      while s_ptr != (*p_ptr).s_list
        {
          /* C 478-481: "Recompute maximum selection label width."  */
          max_s_label = max! (max_s_label, (*s_ptr).label_width);
          s_ptr = (*s_ptr).next;
        }
      p_ptr = (*p_ptr).next;
    }

  /* C 485-488: "Recompute pane height."  */
  p_height = ((*menu).flag_height << 1) + ((*menu).s_y_off * s_count);

  /* C 490-494: "Recompute horizontal padding between the pane window and the selection
     windows."  */
  p_s_pad = (*menu).p_x_off << 1;

  /* C 496-507: "Recompute pane and selection window widths.  …"  */
  p_width = max_p_label + p_s_pad;
  s_width = max_s_label + ((*menu).s_fnt_pad << 1) + ((*menu).s_bdr_width << 1);
  p_s_diff = p_width - s_width;
  if p_s_diff < p_s_pad
    { p_width = s_width + p_s_pad; }
  else if p_s_diff > p_s_pad
    { s_width = p_width - p_s_pad; }

  /* C 518-520: "Reset menu wide global values."  */
  (*menu).s_count = s_count;
  (*menu).p_height = p_height;
  (*menu).p_width = p_width;
  (*menu).s_width = s_width;

  /* C 526-529: "Ensure that the origin of the menu is placed so that None of the panes or
     selections are off the screen."  */
  screen = DefaultScreen (display);
  if (*menu).x_pos + (*menu).width > DisplayWidth (display, screen)
    { (*menu).x_pos = DisplayWidth (display, screen) - (*menu).width; }
  else if (*menu).x_pos < 0
    { (*menu).x_pos = 0; }
  if (*menu).y_pos + (*menu).height > DisplayHeight (display, screen)
    { (*menu).y_pos = DisplayHeight (display, screen) - (*menu).height; }
  else if (*menu).y_pos < 0
    { (*menu).y_pos = 0; }
}

/* C 539-683: `_XMRecomputePane` - Internal subroutine to recompute pane window
   dependencies.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMRecomputePane (display: *mut Display, menu: *mut XMenu,
                                           p_ptr: *mut XMPane, p_num: c_int) -> c_int
{
  let mut window_x: c_int;                      /* Recomputed window X coordinate. */
  let mut window_y: c_int;                      /* Recomputed window Y coordinate. */

  let mut change_mask: c_ulong;                 /* Value mask to reconfigure window. */

  let mut config_p: Bool = False;               /* Reconfigure pane window? */

  /* C 558-561: "Update the pane serial number."  */
  (*p_ptr).serial = p_num;

  /* C 563-566: "Recompute window X and Y coordinates."  */
  match (*menu).menu_style
    {
      LEFT =>
        {
          window_x = (*menu).p_x_off * (((*menu).p_count - 1) - p_num);
          window_y = (*menu).p_y_off * (((*menu).p_count - 1) - p_num);
        }
      RIGHT =>
        {
          window_x = (*menu).p_x_off * p_num;
          window_y = (*menu).p_y_off * (((*menu).p_count - 1) - p_num);
        }
      CENTER =>
        {
          window_x = 0;
          window_y = (*menu).p_y_off * (((*menu).p_count - 1) - p_num);
        }
      _ =>
        {
          /* C 580-582: "Error! Invalid style parameter."  */
          _XMErrorCode = XME_STYLE_PARAM;
          return _FAILURE;
        }
    }
  window_x += (*menu).x_pos;
  window_y += (*menu).y_pos;

  /* C 587-595: "If the newly compute pane coordinates differ from the current coordinates, reset
     the current coordinates and reconfigure the pane."  */
  if ((window_x != (*p_ptr).window_x) ||
      (window_y != (*p_ptr).window_y))
    {
      /* C 596-599: "Reset the coordinates and schedule the pane for reconfiguration."  */
      (*p_ptr).window_x = window_x;
      (*p_ptr).window_y = window_y;
      config_p = True;
    }

  /* C 605-612: "If the local pane width and height differs from the menu pane width and height,
     reset the local values."  */
  if (((*p_ptr).window_w != (*menu).p_width) ||
      ((*p_ptr).window_h != (*menu).p_height))
    {
      /* C 613-616: "Reset window width and height and schedule the pane for reconfiguration."  */
      (*p_ptr).window_w = (*menu).p_width;
      (*p_ptr).window_h = (*menu).p_height;
      config_p = True;
    }

  /* C 622-624: "If we need to reconfigure the pane window do it now."  */
  if config_p == True
    {
      /* C 626-630: "If the pane window has already been created then reconfigure the existing
         window, otherwise queue it for creation with the new configuration."  */
      if (*p_ptr).window != 0
        {
          let mut changes: XWindowChanges = core::mem::zeroed ();
          change_mask = (CWX | CWY | CWWidth | CWHeight) as c_ulong;
          changes.x = (*p_ptr).window_x;
          changes.y = (*p_ptr).window_y;
          changes.width = (*p_ptr).window_w;
          changes.height = (*p_ptr).window_h;

          XConfigureWindow (display,
                            (*p_ptr).window,
                            change_mask as c_uint,
                            &raw mut changes);
        }
      else
        {
          if _XMWinQueAddPane (display, menu, p_ptr) == _FAILURE
            { return _FAILURE; }
        }
    }

  /* C 653-655: "Recompute label X position."  */
  match (*menu).p_style
    {
      LEFT =>
        {
          (*p_ptr).label_x = (*menu).p_x_off + (*menu).p_fnt_pad;
        }
      RIGHT =>
        {
          (*p_ptr).label_x = (*menu).p_width -
            ((*p_ptr).label_width + (*menu).p_x_off + (*menu).p_fnt_pad);
        }
      CENTER =>
        {
          (*p_ptr).label_x = ((*menu).p_width - (*p_ptr).label_width) >> 1;
        }
      _ =>
        {
          /* C 667-670: "Error! Invalid style parameter."  */
          _XMErrorCode = XME_STYLE_PARAM;
          return _FAILURE;
        }
    }
  /* C 672-674: "Recompute label Y positions."  */
  (*p_ptr).label_uy = (*menu).p_fnt_pad + (*(*menu).p_fnt_info).max_bounds.ascent as c_int;
  (*p_ptr).label_ly = ((*menu).p_height - (*menu).p_fnt_pad
                       - (*(*menu).p_fnt_info).max_bounds.descent as c_int);

  /* C 678-680: "All went well, return successfully."  */
  _XMErrorCode = XME_NO_ERROR;
  return _SUCCESS;
}

/* C 685-805: `_XMRecomputeSelection` - Internal subroutine to recompute selection window
   dependencies.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMRecomputeSelection (display: *mut Display, menu: *mut XMenu,
                                                s_ptr: *mut XMSelect, s_num: c_int) -> c_int
{
  let mut config_s: Bool = False;               /* Reconfigure selection window? */
  let mut change_mask: c_ulong;                 /* Value mask for XConfigureWindow. */

  /* C 701-709: "If the selection serial numbers are out of order, begin resequencing selections. 
     … When selections are created they are given a serial number of -1, this causes this routine
     to give a new selection its initial coordinates and serial number."  */
  if (*s_ptr).serial != s_num
    {
      /* C 711-714: "Fix the sequence number."  */
      (*s_ptr).serial = s_num;
      /* C 715-719: "Recompute window X and Y coordinates."  */
      (*s_ptr).window_x = (*menu).s_x_off;
      (*s_ptr).window_y = (*menu).flag_height + ((*menu).s_y_off * s_num);
      /* C 720-723: "We must reconfigure the window."  */
      config_s = True;
    }

  /* C 726-733: "If the local selection width and height differs from the menu selection width and
     height, reset the local values."  */
  if (((*s_ptr).window_w != (*menu).s_width) ||
      ((*s_ptr).window_h != (*menu).s_height))
    {
      /* C 734-737: "We must reconfigure the window."  */
      config_s = True;
      /* C 738-742: "Reset window width and height."  */
      (*s_ptr).window_w = (*menu).s_width;
      (*s_ptr).window_h = (*menu).s_height;
    }

  /* C 745-747: "If we need to reconfigure the selection window do it now."  */
  if config_s == True
    {
      /* C 749-753: "If the selection window has already been created then reconfigure the
         existing window, otherwise queue it for creation with the new configuration."  */
      if (*s_ptr).window != 0
        {
          let mut changes: XWindowChanges = core::mem::zeroed ();
          change_mask = (CWX | CWY | CWWidth | CWHeight) as c_ulong;
          changes.x = (*s_ptr).window_x;
          changes.y = (*s_ptr).window_y;
          changes.width = (*s_ptr).window_w;
          changes.height = (*s_ptr).window_h;

          XConfigureWindow (display,
                            (*s_ptr).window,
                            change_mask as c_uint,
                            &raw mut changes);
        }
      else
        {
          if _XMWinQueAddSelection (display, menu, s_ptr) == _FAILURE
            { return _FAILURE; }
        }
    }

  /* C 776-778: "Recompute label X position."  */
  match (*menu).s_style
    {
      LEFT =>
        {
          (*s_ptr).label_x = (*menu).s_bdr_width + (*menu).s_fnt_pad + (*s_ptr).window_x;
        }
      RIGHT =>
        {
          (*s_ptr).label_x = (*s_ptr).window_x + (*menu).s_width -
            ((*s_ptr).label_width + (*menu).s_bdr_width + (*menu).s_fnt_pad);
        }
      CENTER =>
        {
          (*s_ptr).label_x = (*s_ptr).window_x
            + (((*menu).s_width - (*s_ptr).label_width) >> 1);
        }
      _ =>
        {
          /* C 790-793: "Error! Invalid style parameter."  */
          _XMErrorCode = XME_STYLE_PARAM;
          return _FAILURE;
        }
    }
  /* C 795-798: "Recompute label Y position."  */
  (*s_ptr).label_y = (*s_ptr).window_y + (*(*menu).s_fnt_info).max_bounds.ascent as c_int
    + (*menu).s_fnt_pad + (*menu).s_bdr_width;

  /* C 800-802: "All went well, return successfully."  */
  _XMErrorCode = XME_NO_ERROR;
  return _SUCCESS;
}

/* C 807-873: `_XMTransToOrigin` - Internal subroutine to translate the point at the center of the
   current pane and selection to the menu origin.  "WARNING! ****** Be certain that all menu
   dependencies have been recomputed before calling this routine or unpredictable results will
   follow."  `display` is unused in the C text too ("Not used. Included for consistency.").  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMTransToOrigin (display: *mut Display, menu: *mut XMenu,
                                           p_ptr: *mut XMPane, s_ptr: *mut XMSelect,
                                           x_pos: c_int, y_pos: c_int, orig_x: *mut c_int,
                                           orig_y: *mut c_int)
{
  let mut l_orig_x: c_int;                      /* Local X coordinate of the menu origin. */
  let mut l_orig_y: c_int;                      /* Local Y coordinate of the menu origin. */

  /* C 832-837: "Translate the menu origin such that the cursor hot point will be in the center of
     the desired current selection and pane.  If the current selection pointer is NULL then assume
     that the hot point will be in the center of the current pane flag."  */
  if s_ptr.is_null ()
    {
      /* C 840-845: "Translate from the center of the pane flag to the upper left of the current
         pane window."  */
      l_orig_x = x_pos - ((*menu).p_width >> 1) - (*menu).p_bdr_width;
      l_orig_y = y_pos - ((*menu).flag_height >> 1) - (*menu).p_bdr_width;
    }
  else
    {
      /* C 848-853: "First translate from the center of the current selection to the upper left of
         the current selection window."  */
      l_orig_x = x_pos - ((*menu).s_width >> 1);
      l_orig_y = y_pos - ((*menu).s_height >> 1);

      /* C 855-859: "Then translate to the upper left of the current pane window."  */
      l_orig_x -= ((*s_ptr).window_x + (*menu).p_bdr_width);
      l_orig_y -= ((*s_ptr).window_y + (*menu).p_bdr_width);
    }

  /* C 862-866: "Finally translate to the upper left of the menu."  */
  l_orig_x -= ((*p_ptr).window_x - (*menu).x_pos);
  l_orig_y -= ((*p_ptr).window_y - (*menu).y_pos);

  /* C 868-872: "Set the return values."  */
  *orig_x = l_orig_x;
  *orig_y = l_orig_y;
}

/* C 875-938: `_XMRefreshPane` - Internal subroutine to completely refresh the contents of a
   pane.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMRefreshPane (display: *mut Display, menu: *mut XMenu, pane: *mut XMPane)
{
  let s_list: *mut XMSelect = (*pane).s_list;
  let mut s_ptr: *mut XMSelect;

  /* C 885-887: "First clear the pane."  */
  XClearWindow (display, (*pane).window);
  if !((*pane).activated != 0)
    {
      XFillRectangle (display,
                      (*pane).window,
                      (*menu).inverse_select_GC,
                      (*pane).label_x - (*menu).p_fnt_pad,
                      (*pane).label_uy - (*(*menu).p_fnt_info).max_bounds.ascent as c_int
                        - (*menu).p_fnt_pad,
                      ((*pane).label_width + ((*menu).p_fnt_pad << 1)) as c_uint,
                      (*menu).flag_height as c_uint);

      XFillRectangle (display,
                      (*pane).window,
                      (*menu).inverse_select_GC,
                      (*pane).label_x - (*menu).p_fnt_pad,
                      (*pane).label_ly - (*(*menu).p_fnt_info).max_bounds.ascent as c_int
                        - (*menu).p_fnt_pad,
                      ((*pane).label_width + ((*menu).p_fnt_pad << 1)) as c_uint,
                      (*menu).flag_height as c_uint);
    }
  if !((*pane).active != 0)
    {
      XDrawString (display,
                   (*pane).window,
                   (*menu).inact_GC,
                   (*pane).label_x, (*pane).label_uy,
                   (*pane).label, (*pane).label_length);
      XDrawString (display,
                   (*pane).window,
                   (*menu).inact_GC,
                   (*pane).label_x, (*pane).label_ly,
                   (*pane).label, (*pane).label_length);
    }
  else
    {
      XDrawString (display,
                   (*pane).window,
                   (*menu).pane_GC,
                   (*pane).label_x, (*pane).label_uy,
                   (*pane).label, (*pane).label_length);
      XDrawString (display,
                   (*pane).window,
                   (*menu).pane_GC,
                   (*pane).label_x, (*pane).label_ly,
                   (*pane).label, (*pane).label_length);

      /* C 930-932: "Finally refresh each selection if the pane is activated."  */
      if (*pane).activated != 0
        {
          s_ptr = (*s_list).next;
          while s_ptr != s_list
            {
              _XMRefreshSelection (display, menu, s_ptr);
              s_ptr = (*s_ptr).next;
            }
        }
    }
}

/* C 940-1023: `_XMRefreshSelection` - Internal subroutine that refreshes a single selection
   window.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _XMRefreshSelection (display: *mut Display, menu: *mut XMenu,
                                              sel: *mut XMSelect)
{
  let width: c_int = (*sel).window_w;
  let height: c_int = (*sel).window_h;
  let bdr_width: c_int = (*menu).s_bdr_width;

  if (*sel).r#type == SEPARATOR
    {
      XDrawLine (display,
                 (*(*sel).parent_p).window,
                 (*menu).normal_select_GC,
                 (*sel).window_x,
                 (*sel).window_y + height / 2,
                 (*sel).window_x + width,
                 (*sel).window_y + height / 2);
    }
  else if (*sel).activated != 0
    {
      if (*menu).menu_mode == INVERT
        {
          XFillRectangle (display,
                          (*(*sel).parent_p).window,
                          (*menu).normal_select_GC,
                          (*sel).window_x, (*sel).window_y,
                          width as c_uint, height as c_uint);
          XDrawString (display,
                       (*(*sel).parent_p).window,
                       (*menu).inverse_select_GC,
                       (*sel).label_x,
                       (*sel).label_y,
                       (*sel).label, (*sel).label_length);
        }
      else
        {
          /* C 978-983: "Using BOX mode.  Since most drawing routines with arbitrary width lines
             are slow compared to raster-ops let's use a raster-op to draw the boxes."  */

          XDrawRectangle (display,
                          (*(*sel).parent_p).window,
                          (*menu).normal_select_GC,
                          (*sel).window_x + (bdr_width >> 1),
                          (*sel).window_y + (bdr_width >> 1),
                          (width - bdr_width) as c_uint,
                          (height - bdr_width) as c_uint);
          XDrawString (display,
                       (*(*sel).parent_p).window,
                       (*menu).normal_select_GC,
                       (*sel).label_x,
                       (*sel).label_y,
                       (*sel).label, (*sel).label_length);
        }
    }
  else
    {
      XClearArea (display,
                  (*(*sel).parent_p).window,
                  (*sel).window_x, (*sel).window_y,
                  width as c_uint, height as c_uint,
                  False);
      if (*sel).active != 0
        {
          XDrawString (display,
                       (*(*sel).parent_p).window,
                       (*menu).normal_select_GC,
                       (*sel).label_x,
                       (*sel).label_y,
                       (*sel).label, (*sel).label_length);
        }
      else
        {
          XDrawString (display,
                       (*(*sel).parent_p).window,
                       (*menu).inact_GC,
                       (*sel).label_x,
                       (*sel).label_y,
                       (*sel).label, (*sel).label_length);
        }
    }
}
