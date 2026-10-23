// femacs: port of lwlib/xlwmenu.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the
// `static` helpers whose address the Xt tables take are `unsafe extern "C" fn` (PORTING.md 1.4).
//
// **The file gate is `feature = "use_lucid"`** (P-86 clause 8, the build's condition rather than
// one the C text spells): `lwlib/Makefile.in` 52 (`LUCID_OBJS = lwlib-Xlw.o xlwmenu.o
// lwlib-Xaw.o`), 56 (`TOOLKIT_OBJS = $(@X_TOOLKIT_TYPE@_OBJS)`) and 58 (`OBJS = lwlib.o
// $(TOOLKIT_OBJS) lwlib-utils.o`) are what put this object in the build, with `configure.ac`
// 7231's `AC_DEFINE ([USE_LUCID], [1], [Define to 1 if using the Lucid X toolkit.])` as the
// configure condition.  `USE_LUCID`, `USE_X_TOOLKIT` and `HAVE_X11` are `undef` in
// docs/baseline/config/darwin-arm64.macros.tsv, so the frozen configuration compiles none of
// this file and the port's frozen object is the empty one.
//
// `feature = "use_x_toolkit"` is the C macro the *text* relies on besides the gate: it is what
// makes `widget_value`'s `toolkit_data`/`free_toolkit_data` pair live (lwlib-widget.h 87-101),
// and C 421, 446 and 447 read both.  The build's own `-Demacs` is the second such condition;
// P-86 clause 1's "both arms" rule carries it as `feature = "emacs"` (the frozen side is the
// standalone lwlib build C 44-48, 1474-1487, 1832-1835, 1861-1872, 1882-1892, 2203-2210,
// 2821-2830 and 2877-2914 spell).
//
// **This file is `text-only`** (P-86 clause 3(2)): this machine has no Xt and no Motif header.
// `/opt/homebrew/include/X11` carries `xorgproto` 2025.1's Xlib half, so the Xlib records and
// scalars this file addresses are **measured with clang** against those headers (`## Evidence`
// of docs/ports/xlwmenu.md); every Xt, Lucid and Xft width, offset, enum value, resource name
// and entry point below is C's spelling with the value **UNVERIFIED**.
//
// The sub-conditions the C text spells keep their own gates (P-86 clause 1 and 6):
// `#ifdef HAVE_X_I18N` is `feature = "have_x_i18n"`, `#ifdef USE_CAIRO` is `feature = "use_cairo"`
// and `#if defined USE_CAIRO || defined HAVE_XFT` is `feature = "use_cairo_or_have_xft"` (the
// derived name `lwlib/include/xlwmenuP.rs` 53 registered).
//
// The `<X11/*>` and Lucid surface stands in this unit (P-88 clause 1: no mirror in this tree
// carries any of those headers and this file is their only consumer here);
// `lwlib/include/{lwlib,lwlib-widget,lwlib-int,xlwmenu,xlwmenuP}.rs` name `Widget`, `WidgetClass`,
// `Window`, `String`, `Pixmap`, `Pixel`, `Cursor`, `Position`, `Dimension`, `XFontStruct`,
// `XFontSet`, `XFontSetExtents`, `XtCallbackList`, `GC`, `CorePart` and `CoreClassPart` and leave
// them to the unit that includes them.
//
// C's static forward declarations (C 171-189) are **deleted**: Rust items are order-independent
// (PORTING.md 0.1-12 / checklist 10) - the ledger's `## C line ranges` names the region.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, unused_mut, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, clippy::all)]

include!("../src/include/config.rs");            // C 21: `#include <config.h>`
/* C 23's `#include <setjmp.h>` has no edge of its own: nothing in this file names a declaration
   that header carries.  */
#[cfg (feature = "use_lucid")]
include!("../src/include/lisp.rs");              // C 24: `#include <lisp.h>`
                                                 // (`emacs_abort`; lisp.h 30 is this unit's
                                                 //  `<string.h>`: `strlen`/`strcpy`/`stpcpy`/`strcmp`)
/* C 26's `#include <stdio.h>` (`fprintf`) has its own mirror.  */
#[cfg (feature = "use_lucid")]
include!("../lib/include/stdio.rs");             // C 26: `#include <stdio.h>`
/* C 27's `#include <sys/types.h>` is covered by lisp.rs's own chain.  */
/* C 28-32's `<X11/Xos.h>`, `<X11/IntrinsicP.h>`, `<X11/ObjectP.h>`, `<X11/StringDefs.h>`,
   `<X11/cursorfont.h>` and `<X11/Shell.h>` are the surface declared below: no mirror in this
   tree carries any of them and this file is their only consumer here (P-88 clause 1).  */
/* C 33's `#include "xlwmenuP.h"` and its chain (`xlwmenu.h` -> `lwlib.h` -> `lwlib-widget.h`,
   `lwlib-int.h`, and - under the cairo arm - `lwlib-utils.h`).  The mirrors' declarations live
   in a module plus a glob import (PORTING.md 0.1-10 / P-39): this unit *defines*
   `xlwMenuClassRec`, `xlwMenuWidgetClass`, `xlwmenu_window_p` and `xlwmenu_redisplay`, which
   `include/xlwmenu.rs` and `include/xlwmenuP.rs` declare, and a Rust definition cannot share a
   scope with the foreign declaration of the same name (E0428) - an explicit item shadows a glob
   import instead.  */
#[cfg (feature = "use_lucid")]
#[cfg (feature = "use_lucid")]
pub mod xlwmenu_decls {
  use super::*;
  #[cfg (feature = "use_lucid")]
  include!("include/xlwmenuP.rs");               // C 33: `#include "xlwmenuP.h"`
                                                 // (its `<X11/CoreP.h>` edge is the surface above)
  #[cfg (feature = "use_lucid")]
  include!("include/lwlib-int.rs");              // xlwmenuP.h's chain: `widget_instance`
  /* C 28-31 of xlwmenuP.h - `#if defined USE_CAIRO || defined HAVE_XFT / #ifdef USE_CAIRO /
     #include "lwlib-utils.h" / #else <X11/Xft/Xft.h>` - is the arm this unit's own Xft/cairo
     declarations answer: this file reads no symbol `lwlib-utils.h` declares (only the
     `XftFont`/`XftColor`/`XftDraw`/`XGlyphInfo` types and the cairo entry points, all declared
     below), and including the mirror would put a second, different `XftFont`/`XftColor` in
     scope beside this unit's (`## Deviations`).  The `widget_xft_data` tag the `widget_instance`
     field needs is declared in the unit instead.  */
}
#[cfg (feature = "use_lucid")]
use xlwmenu_decls::*;
/* C 40's `#include <xterm.h>` is the four entry points declared below, with a `/// C:` line each
   (`src/xterm.h`); C 41's `#include "bitmaps/gray.xbm"` is the three constants below (the
   `bitmaps/gray.xbm` data, `emacs/src/bitmaps/gray.xbm`).  C 46's `<X11/bitmaps/gray>` is the
   `not (feature = "emacs")` arm of the same pair.  */

/* ------------------------------------------------------------------------------------------
   The Xlib half (`<X11/Xlib.h>`, `<X11/X.h>`, `<X11/Xutil.h>`, `<X11/Xos.h>`,
   `<X11/cursorfont.h>`) and the Xt half (`<X11/Intrinsic.h>`, `<X11/IntrinsicP.h>`,
   `<X11/ObjectP.h>`, `<X11/CoreP.h>`, `<X11/StringDefs.h>`, `<X11/Shell.h>`).  The Xlib records
   and scalars are **measured** (`## Evidence` of docs/ports/xlwmenu.md: `sizeof (XEvent)` 192,
   `sizeof (XButtonEvent)` 96, the fifteen offsets of `XButtonEvent`, …); the Xt records carry
   the members this file reads in C's relative order with the members between them omitted (their
   offsets are knowingly not the real ones) and every Xt/Lucid/Xft name is `UNVERIFIED`.
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "use_lucid")]
pub type XtPointer = *mut c_void;                     // C: `typedef char *XtPointer;`
#[cfg (feature = "use_lucid")]
pub type String = *mut c_char;                        // C: `typedef char *String;` (X11's)
#[cfg (feature = "use_lucid")]
pub type Boolean = c_int;                             // C: `typedef int Boolean;`
#[cfg (feature = "use_lucid")]
pub const False: Boolean = 0;                         // C: `#define False 0`
#[cfg (feature = "use_lucid")]
pub const True: Boolean = 1;                          // C: `#define True 1`
#[cfg (feature = "use_lucid")]
pub const NULL: *mut c_void = core::ptr::null_mut (); // C: `<stddef.h>`: `#define NULL ((void *) 0)`
#[cfg (feature = "use_lucid")]
pub type Cardinal = c_uint;                           // C: `typedef unsigned int Cardinal;`
/// C: X.h 66-88: `typedef unsigned long XID;` and `typedef XID Window;` (measured: 8 bytes).
#[cfg (feature = "use_lucid")]
pub type XID = c_ulong;
#[cfg (feature = "use_lucid")]
pub type Window = XID;
#[cfg (feature = "use_lucid")]
pub type Drawable = XID;                              // C: X.h: `typedef XID Drawable;`
#[cfg (feature = "use_lucid")]
pub type Pixmap = XID;                                // C: X.h 78: `typedef XID Pixmap;`
#[cfg (feature = "use_lucid")]
pub type Cursor = XID;                                // C: X.h: `typedef XID Cursor;`
#[cfg (feature = "use_lucid")]
pub type Font = XID;                                  // C: X.h: `typedef XID Font;`
#[cfg (feature = "use_lucid")]
pub type Atom = XID;                                  // C: X.h: `typedef XID Atom;`
#[cfg (feature = "use_lucid")]
pub type Colormap = XID;                              // C: X.h: `typedef XID Colormap;`
#[cfg (feature = "use_lucid")]
pub type Pixel = c_ulong;                             // C: X.h: `typedef unsigned long Pixel;`
#[cfg (feature = "use_lucid")]
pub type Mask = c_ulong;                              // C: X.h 70: `typedef unsigned long Mask;`
#[cfg (feature = "use_lucid")]
pub type Time = c_ulong;                              // C: X.h: `typedef unsigned long Time;`
#[cfg (feature = "use_lucid")]
pub type KeySym = c_ulong;                            // C: X.h: `typedef unsigned long KeySym;`
/// C: X.h: `typedef int Position;` - **UNVERIFIED**: this machine's Xlib.h/X.h do not carry the
/// name (the Xt half of the surface is the half that is missing here).
#[cfg (feature = "use_lucid")]
pub type Position = c_int;
/// C: X.h: `typedef unsigned short Dimension;` - **UNVERIFIED**, same reason as `Position`.
#[cfg (feature = "use_lucid")]
pub type Dimension = c_ushort;
/// C: Xlib.h: `typedef struct _XGC *GC;`
#[cfg (feature = "use_lucid")]
pub type GC = *mut c_void;
/// C: Xutil.h: `typedef struct _XRegion *Region;` (`XlwMenuRedisplay`'s third parameter).
#[cfg (feature = "use_lucid")]
pub type Region = *mut c_void;
/// C: Xlib.h 251: `struct _XDisplay;` and its completion as the opaque `Display`.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Display { pub _private: [u8; 0] }
/// C: Xlib.h: `typedef struct _XRegion *Region;`'s sibling `Screen` (opaque here).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Screen { pub _private: [u8; 0] }
/// C: Xlib.h: `typedef struct _XVisual *Visual;` (opaque here).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Visual { pub _private: [u8; 0] }

/// C: Xlib.h 418-424 (measured): `typedef struct { short x, y; } XPoint;` - 4 bytes, offsets 0/2.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XPoint { pub x: c_short, pub y: c_short }
/// C: Xlib.h 426-429 (measured): `typedef struct { short x, y; unsigned short width, height; }
/// XRectangle;` - 8 bytes.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XRectangle { pub x: c_short, pub y: c_short,
                        pub width: c_ushort, pub height: c_ushort }
/// C: Xlib.h (measured): `typedef struct { short lbearing, rbearing; short width; short ascent;
/// short descent; unsigned short attributes; } XCharStruct;` - 12 bytes, the five offsets this
/// file reads are 0/2/4/6/8.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XCharStruct { pub lbearing: c_short, pub rbearing: c_short, pub width: c_short,
                         pub ascent: c_short, pub descent: c_short, pub attributes: c_ushort }
/// C: Xlib.h: `typedef struct { unsigned long pixel; unsigned short red, green, blue; char
/// flags; char pad; } XColor;` - 16 bytes (measured).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XColor { pub pixel: c_ulong, pub red: c_ushort, pub green: c_ushort,
                    pub blue: c_ushort, pub flags: c_char, pub pad: c_char }
/// C: Xlib.h: `typedef struct { XExtData *ext_data; short private1; … short ascent, descent; …
/// XCharStruct max_bounds; short min_char_or_byte2, max_char_or_byte2; … } XFontStruct;` - 96
/// bytes (measured); the five members this file reads are carried, in C's order.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XFontStruct {
  pub ext_data: *mut c_void,        // C: `XExtData *ext_data;`
  pub private1: c_ushort,           // C: `unsigned short private1;`
  pub private2: c_ushort,           // C: `unsigned short private2;`
  pub min_bounds: XCharStruct,      // measured offset 56
  pub max_bounds: XCharStruct,      // measured offset 68
  pub max_char_or_byte2: c_ushort,  // C: `unsigned short max_char_or_byte2;`
  pub min_char_or_byte2: c_ushort,  // C: `unsigned short min_char_or_byte2;`
  pub default_char: c_ushort,       // C: `unsigned short default_char;`
  pub per_char: *mut c_void,        // C: `XCharStruct *per_char;`
  pub fid: Font,                    // C: `Font fid;` - the member `make_drawing_gcs` reads
  pub ascent: c_short,              // measured offset 88
  pub descent: c_short,             // measured offset 92
}
/// C: Xlib.h (measured): `typedef struct { long foreground, background; … int line_style,
/// cap_style, join_style; … unsigned long font; … Bool graphics_exposures; … Pixmap clip_mask;
/// … } XGCValues;` - 128 bytes; the nine members this file reads are carried, in C's order.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XGCValues {
  pub function: c_int,              // C: `int function;`
  pub plane_mask: c_ulong,
  pub foreground: c_ulong,          // measured offset 16
  pub background: c_ulong,          // measured offset 24
  pub line_width: c_int,
  pub line_style: c_int,            // measured offset 36
  pub cap_style: c_int,             // measured offset 40
  pub join_style: c_int,            // measured offset 44
  pub fill_style: c_int,            // measured offset 48
  pub fill_rule: c_int,
  pub arc_mode: c_int,
  pub tile: Pixmap,
  pub stipple: Pixmap,
  pub ts_x_origin: c_int,
  pub ts_y_origin: c_int,
  pub font: Font,                   // measured offset 88
  pub subwindow_mode: c_int,
  pub graphics_exposures: c_int,    // measured offset 100
  pub clip_x_origin: c_int,
  pub clip_y_origin: c_int,
  pub clip_mask: Pixmap,            // measured offset 112
  pub dash_offset: c_int,
  pub dashes: c_char,
}
/// C: Xlib.h (measured): `typedef struct { … } XAnyEvent;` - 40 bytes, the two members this file
/// reads are `type` (0) and `window` (32).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XAnyEvent {
  pub r#type: c_int,                // measured offset 0
  pub serial: c_ulong,              // measured offset 8
  pub send_event: c_int,            // measured offset 16
  pub display: *mut Display,        // measured offset 24
  pub window: Window,               // measured offset 32
}
/// C: Xlib.h (measured): `typedef struct { int type; unsigned long serial; Bool send_event;
/// Display *display; Window window, root, subwindow; Time time; int x, y, x_root, y_root;
/// unsigned int state, button; Bool same_screen; } XButtonEvent;` - 96 bytes, the fifteen
/// offsets measured: 0/8/16/24/32/40/48/56/64/68/72/76/80/84/88.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XButtonEvent {
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: c_int,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub button: c_uint,
  pub same_screen: c_int,
}
/// C: Xlib.h: `typedef XButtonEvent XButtonPressedEvent;` - Xlib's own alias.
#[cfg (feature = "use_lucid")]
pub type XButtonPressedEvent = XButtonEvent;
/// C: Xlib.h (measured): `typedef struct { … } XMotionEvent;` - 96 bytes; `type` 0, `window` 32,
/// `time` 56, `x` 64, `y` 68, `x_root` 72, `y_root` 76, `state` 80, `is_hint` 84.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XMotionEvent {
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: c_int,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub is_hint: c_char,
  pub same_screen: c_int,
}
/// C: Xlib.h (measured): `typedef struct { … } XKeyEvent;` - 96 bytes; `window` 32, `state` 80,
/// `keycode` 84.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XKeyEvent {
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: c_int,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub keycode: c_uint,
  pub same_screen: c_int,
}
/// C: Xlib.h (measured): `typedef struct { … } XExposeEvent;` - 64 bytes.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XExposeEvent {
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: c_int,
  pub display: *mut Display,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub count: c_int,
}
/// C: Xlib.h (measured): `typedef union _XEvent { … } XEvent;` - 192 bytes, alignment 8.  C
/// writes `menu_post_event.xbutton` and the three casts this file makes; `_private` pads the
/// union to Xlib's size.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub union XEvent {
  pub r#type: c_int,
  pub xany: XAnyEvent,
  pub xkey: XKeyEvent,
  pub xbutton: XButtonEvent,
  pub xmotion: XMotionEvent,
  pub xexpose: XExposeEvent,
  pub _private: [u8; 192],
}
/// C: Xlib.h (measured): `typedef struct { … } XSetWindowAttributes;` - 112 bytes; `background_pixel`
/// 8, `border_pixel` 24, `bit_gravity` 32, `event_mask` 72, `override_redirect` 88, `colormap` 96,
/// `cursor` 104.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XSetWindowAttributes {
  pub background_pixmap: Pixmap,
  pub background_pixel: c_ulong,
  pub border_pixmap: Pixmap,
  pub border_pixel: c_ulong,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: c_int,
  pub event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: c_int,
  pub colormap: Colormap,
  pub cursor: Cursor,
}
/// C: Xlib.h (measured): `typedef struct { … } XWindowAttributes;` - 136 bytes; the members this
/// file reads are `x` 0, `y` 4, `width` 8, `height` 12, `border_width` 16, `class` 40,
/// `bit_gravity` 44, `colormap` 80, `map_installed` 88, `override_redirect` 120.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XWindowAttributes {
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub depth: c_int,
  pub visual: *mut Visual,
  pub root: Window,
  pub class: c_int,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: c_int,
  pub colormap: Colormap,
  pub map_installed: c_int,
  pub map_state: c_int,
  pub all_event_masks: c_long,
  pub your_event_mask: c_long,
  pub no_event_mask: c_long,
  pub override_redirect: c_int,
  pub screen: *mut Screen,
}

/* ---- the Xt records (UNVERIFIED: no Xt header on this machine) ---- */
/// C: `<X11/Intrinsic.h>`: `typedef struct _WidgetRec *Widget;`
#[cfg (feature = "use_lucid")]
pub type Widget = *mut WidgetRec;
/// C: `<X11/Intrinsic.h>`: `typedef struct _WidgetClassRec *WidgetClass;`
#[cfg (feature = "use_lucid")]
pub type WidgetClass = *mut WidgetClassRec;
/// C: `<X11/IntrinsicP.h>`: `typedef struct _WidgetRec { CorePart core; … } WidgetRec;` - the
/// members between `core` and `core` (the subclass fields) are omitted, and `core` is the first
/// member so its offset is C's.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct WidgetRec { pub core: CorePart }
/// C: `<X11/IntrinsicP.h>`: `typedef struct _WidgetClassRec { CoreClassPart core_class; … }`.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct WidgetClassRec { pub core_class: CoreClassPart }
/// C: `<X11/CoreP.h>`: `typedef struct _CorePart { Widget self; WidgetClass widget_class;
/// Widget parent; … String name; … Cardinal width, height, border_width; … Dimension depth;
/// … Window window; … Screen *screen; … Colormap colormap; … Boolean being_destroyed; … }
/// CorePart;` - the members this file reads are carried in C's relative order (their offsets are
/// knowingly not the real ones).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CorePart {
  pub self_: Widget,                // C: `Widget self;`
  pub widget_class: WidgetClass,    // C: `WidgetClass widget_class;`
  pub parent: Widget,               // C: `Widget parent;`
  pub name: String,                 // C: `String name;`
  pub x: Position,                  // C: `Position x, y;`
  pub y: Position,
  pub width: Dimension,             // C: `Dimension width, height, border_width;`
  pub height: Dimension,
  pub border_width: Dimension,
  pub depth: Dimension,             // C: `Dimension depth;`
  pub background_pixel: Pixel,      // C: `Pixel background_pixel;`
  pub border_pixel: Pixel,          // C: `Pixel border_pixel;`
  pub window: Window,               // C: `Window window;`
  pub screen: *mut Screen,          // C: `Screen *screen;`
  pub colormap: Colormap,           // C: `Colormap colormap;`
  pub being_destroyed: Boolean,     // C: `Boolean being_destroyed;`
}
/// C: `<X11/CoreP.h>`: `typedef struct _CoreClassPart { WidgetClass superclass; String
/// class_name; Cardinal widget_size; XtProc class_initialize; XtProc class_part_initialize;
/// Boolean class_inited; XtInitProc initialize; XtArgsProc initialize_hook; XtRealizeProc
/// realize; XtActionList actions; Cardinal num_actions; XtResourceList resources; Cardinal
/// resource_count; String xrm_class; Boolean compress_motion; XtEnum compress_exposure;
/// Boolean compress_enterleave; Boolean visible_interest; XtWidgetProc destroy; XtWidgetProc
/// resize; XtExposeProc expose; XtSetValuesFunc set_values; XtArgsFunc set_values_hook;
/// XtAlmostProc set_values_almost; XtArgsProc get_values_hook; XtAcceptFocusProc accept_focus;
/// XtVersionType version; XtPointer callback_private; String tm_table; XtGeometryHandler
/// query_geometry; XtStringProc display_accelerator; XtPointer extension; } CoreClassPart;` -
/// the member order C's own initializer (C 210-242) spells out in its comments.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CoreClassPart {
  pub superclass: WidgetClass,
  pub class_name: String,
  pub widget_size: Cardinal,
  pub class_initialize: Option<unsafe extern "C" fn ()>,
  pub class_part_initialize: Option<unsafe extern "C" fn (WidgetClass)>,
  pub class_inited: Boolean,
  pub initialize: Option<unsafe extern "C" fn (Widget, Widget, ArgList, *mut Cardinal)>,
  pub initialize_hook: Option<unsafe extern "C" fn (Widget)>,
  pub realize: Option<unsafe extern "C" fn (Widget, *mut Mask, *mut XSetWindowAttributes)>,
  pub actions: *mut XtActionsRec,
  pub num_actions: Cardinal,
  pub resources: *mut XtResource,
  pub resource_count: Cardinal,
  pub xrm_class: *mut c_char,
  pub compress_motion: Boolean,
  pub compress_exposure: c_int,
  pub compress_enterleave: Boolean,
  pub visible_interest: Boolean,
  pub destroy: Option<unsafe extern "C" fn (Widget)>,
  pub resize: Option<unsafe extern "C" fn (Widget)>,
  pub expose: Option<unsafe extern "C" fn (Widget, *mut XEvent, Region)>,
  pub set_values: Option<unsafe extern "C" fn (Widget, Widget, Widget, ArgList, *mut Cardinal)
                                           -> Boolean>,
  pub set_values_hook: Option<unsafe extern "C" fn (Widget, Widget, Widget, ArgList,
                                                    *mut Cardinal) -> Boolean>,
  pub set_values_almost: Option<unsafe extern "C" fn (Widget, Widget, Widget, *mut c_int,
                                                     *mut XtGeometryResult)>,
  pub get_values_hook: Option<unsafe extern "C" fn (Widget, Widget, Widget, ArgList,
                                                    *mut Cardinal)>,
  pub accept_focus: Option<unsafe extern "C" fn (Widget, *mut Time) -> Boolean>,
  pub version: u32,
  pub callback_private: XtPointer,
  pub tm_table: String,
  pub query_geometry: Option<unsafe extern "C" fn (Widget, *mut XtWidgetGeometry,
                                                   *mut XtWidgetGeometry) -> c_int>,
  pub display_accelerator: Option<unsafe extern "C" fn (Widget, String, String)>,
  pub extension: XtPointer,
}
/// C: `<X11/Intrinsic.h>`: `typedef enum { XtGeometryYes, XtGeometryNo, XtGeometryAlmost,
/// XtGeometryDone } XtGeometryResult;` - UNVERIFIED (the values are Xt's).
#[cfg (feature = "use_lucid")]
pub type XtGeometryResult = c_int;
/// C: `<X11/IntrinsicP.h>`: `typedef struct { … } XtWidgetGeometry;` - opaque here; only its
/// address crosses the class record's two function pointers.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XtWidgetGeometry { pub request_mode: c_int }
/// C: `<X11/Intrinsic.h>`: `typedef struct { String resource_name, resource_class;
/// String resource_type; Cardinal resource_size; Cardinal resource_offset; String default_type;
/// XtPointer default_addr; } XtResource, *XtResourceList;` - UNVERIFIED, the field order C 108-167
/// writes.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XtResource {
  pub resource_name: String,
  pub resource_class: String,
  pub resource_type: String,
  pub resource_size: Cardinal,
  pub resource_offset: Cardinal,
  pub default_type: String,
  pub default_addr: XtPointer,
}
/// C: `<X11/Intrinsic.h>`: `typedef struct { char *string; XtActionProc proc; } XtActionsRec;`
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XtActionsRec {
  pub string: String,
  pub proc: Option<unsafe extern "C" fn (Widget, *mut XEvent, *mut String, *mut Cardinal)>,
}
/// C: `<X11/Intrinsic.h>`: `typedef struct { String name; XtArgVal value; } Arg, *ArgList;`
/// - `XtArgVal` is C's `long` and the port's `XtPointer` (the `lwlib-Xlw.rs` 155-165 and
/// `lwlib-Xm.rs` 320-331 reading: one machine word either way).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Arg {
  pub name: String,
  pub value: XtPointer,
}
#[cfg (feature = "use_lucid")]
pub type ArgList = *mut Arg;
/// C: `<X11/Intrinsic.h>`: `typedef struct _XtCallbackRec { XtCallbackProc callback;
/// XtPointer closure; } XtCallbackRec, *XtCallbackList;` - the list head is what the records hold.
#[cfg (feature = "use_lucid")]
pub type XtCallbackList = *mut c_void;
/// C: `<X11/Intrinsic.h>`: `typedef unsigned long XtGCMask;` - `make_drawing_gcs`'s local.
#[cfg (feature = "use_lucid")]
pub type XtGCMask = c_ulong;
/// C: `<X11/Intrinsic.h>`: `typedef int (*XtErrorHandler) (String);` and the X11 error handler
/// `XtSetErrorHandler`'s parameter; not named by this file.
#[cfg (feature = "use_lucid")]
pub type XtEnum = c_int;

/* ---- the Xt/Lucid resource names this file needs and `lwlib/include/xlwmenu.rs` does not
   carry (that mirror has the 43 `xlwmenu.h` strings; these come from `<X11/StringDefs.h>`,
   `<X11/Intrinsic.h>`, `<X11/Xtos.h>` and Lucid's `XlwMenu.h`) - the values are C's own strings,
   the width of each `&CStr` is the port's reading of the Xt spelling rule (status 3.91).  ---- */
#[cfg (feature = "use_lucid")]
pub const XtNfont: &core::ffi::CStr = c"font";                       // C: `#define XtNfont "font"`
#[cfg (feature = "use_lucid")]
pub const XtCFont: &core::ffi::CStr = c"Font";                       // C: `#define XtCFont "Font"`
#[cfg (feature = "use_lucid")]
pub const XtNfontSet: &core::ffi::CStr = c"fontSet";
#[cfg (feature = "use_lucid")]
pub const XtCFontSet: &core::ffi::CStr = c"FontSet";
#[cfg (feature = "use_lucid")]
pub const XtNforeground: &core::ffi::CStr = c"foreground";
#[cfg (feature = "use_lucid")]
pub const XtCForeground: &core::ffi::CStr = c"Foreground";
#[cfg (feature = "use_lucid")]
pub const XtCMargin: &core::ffi::CStr = c"Margin";
#[cfg (feature = "use_lucid")]
pub const XtCCallback: &core::ffi::CStr = c"Callback";
#[cfg (feature = "use_lucid")]
pub const XtCCursor: &core::ffi::CStr = c"Cursor";
#[cfg (feature = "use_lucid")]
pub const XtRString: &core::ffi::CStr = c"String";
#[cfg (feature = "use_lucid")]
pub const XtRImmediate: &core::ffi::CStr = c"Immediate";
#[cfg (feature = "use_lucid")]
pub const XtRPixel: &core::ffi::CStr = c"Pixel";
#[cfg (feature = "use_lucid")]
pub const XtRDimension: &core::ffi::CStr = c"Dimension";
#[cfg (feature = "use_lucid")]
pub const XtRPixmap: &core::ffi::CStr = c"Pixmap";
#[cfg (feature = "use_lucid")]
pub const XtRCallback: &core::ffi::CStr = c"Callback";
#[cfg (feature = "use_lucid")]
pub const XtRPointer: &core::ffi::CStr = c"Pointer";
#[cfg (feature = "use_lucid")]
pub const XtRCursor: &core::ffi::CStr = c"Cursor";
#[cfg (feature = "use_lucid")]
pub const XtRInt: &core::ffi::CStr = c"Int";
#[cfg (feature = "use_lucid")]
pub const XtRFontSet: &core::ffi::CStr = c"FontSet";
#[cfg (feature = "use_lucid")]
pub const XtRFont: &core::ffi::CStr = c"Font";
/* `<X11/StringDefs.h>`'s other four, read by the shadow-colour code (C 1874-1880, 1898-1940).  */
#[cfg (feature = "use_lucid")]
pub const XtNbackground: &core::ffi::CStr = c"background";
#[cfg (feature = "use_lucid")]
pub const XtNborderColor: &core::ffi::CStr = c"borderColor";
#[cfg (feature = "use_lucid")]
pub const XtNborderWidth: &core::ffi::CStr = c"borderWidth";
#[cfg (feature = "use_lucid")]
pub const XtNheight: &core::ffi::CStr = c"height";
#[cfg (feature = "use_lucid")]
pub const XtNwidth: &core::ffi::CStr = c"width";
#[cfg (feature = "use_lucid")]
pub const XtNsaveUnder: &core::ffi::CStr = c"saveUnder";

/* ---- the Xt scalars and the two macros C's own text writes (measured values are the X11
   protocol's, `## Evidence` of docs/ports/xlwmenu.md) ---- */
#[cfg (feature = "use_lucid")]
pub const NULLQUARK: c_int = 0;                       // C: `<X11/Intrinsic.h>`: `#define NULLQUARK ((XrmQuark)0)` - UNVERIFIED
#[cfg (feature = "use_lucid")]
pub const XtVersion: u32 = 0x0000000b;                // C: `<X11/Intrinsic.h>`: X11R6's `#define XtVersion XtSpecificationRelease * 1000 + 1` - UNVERIFIED
#[cfg (feature = "use_lucid")]
pub const XtExposeCompressMaximal: c_int = 1;         // C: `<X11/Intrinsic.h>`: `(1L << 6)` - UNVERIFIED
#[cfg (feature = "use_lucid")]
pub const XtInheritSetValuesAlmost: *mut c_void = core::ptr::null_mut ();  // C: `#define XtInheritSetValuesAlmost ((XtAlmostProc) _XtInherit)`
#[cfg (feature = "use_lucid")]
pub const XtInheritQueryGeometry: *mut c_void = core::ptr::null_mut ();    // C: `#define XtInheritQueryGeometry ((XtGeometryHandler) _XtInherit)`
#[cfg (feature = "use_lucid")]
pub const XtInheritDisplayAccelerator: *mut c_void = core::ptr::null_mut (); // C: `#define XtInheritDisplayAccelerator ((XtStringProc) _XtInherit)`
#[cfg (feature = "use_lucid")]
pub const XtGrabNone: c_int = 0;                      // C: `<X11/Intrinsic.h>`: `#define XtGrabNone ((Cardinal) 0)` - UNVERIFIED
/// C: `<X11/Intrinsic.h>`: `#define XtNumber(arr) ((Cardinal) (sizeof (arr) / sizeof (arr[0])))`
/// - the port spells it as the macro C writes (the value is the array's own length, computed at
/// the same place).
#[cfg (feature = "use_lucid")]
macro_rules! XtNumber {
  ($ty:ty, $len:expr) => {
    (::core::mem::size_of::<[$ty; $len]> () / ::core::mem::size_of::<$ty> ()) as Cardinal
  };
}
/// C: `<X11/Intrinsic.h>`: `#define XtSetArg(arg, n, d) ((void)((arg).name = (n),
/// (arg).value = (XtArgVal)(d)))` - the `lwlib-Xlw.rs` 161-165 spelling of the same macro.
#[cfg (feature = "use_lucid")]
macro_rules! XtSetArg {
  ($arg:expr, $name:expr, $value:expr) => {
    { ($arg).name = ($name) as String; ($arg).value = ($value) as XtPointer; }
  };
}

/* ---- the X11 constants (all measured, `## Evidence`) ---- */
#[cfg (feature = "use_lucid")]
pub const ButtonPress: c_int = 4;
#[cfg (feature = "use_lucid")]
pub const ButtonRelease: c_int = 5;
#[cfg (feature = "use_lucid")]
pub const MotionNotify: c_int = 6;
#[cfg (feature = "use_lucid")]
pub const KeyPress: c_int = 2;
#[cfg (feature = "use_lucid")]
pub const KeyRelease: c_int = 3;
#[cfg (feature = "use_lucid")]
pub const EnterNotify: c_int = 7;
#[cfg (feature = "use_lucid")]
pub const LeaveNotify: c_int = 8;
#[cfg (feature = "use_lucid")]
pub const Expose: c_int = 12;
#[cfg (feature = "use_lucid")]
pub const CurrentTime: Time = 0;
#[cfg (feature = "use_lucid")]
pub const None: XID = 0;
#[cfg (feature = "use_lucid")]
pub const LineSolid: c_int = 0;
#[cfg (feature = "use_lucid")]
pub const LineOnOffDash: c_int = 1;
#[cfg (feature = "use_lucid")]
pub const LineDoubleDash: c_int = 2;
#[cfg (feature = "use_lucid")]
pub const Convex: c_int = 2;
#[cfg (feature = "use_lucid")]
pub const CoordModeOrigin: c_int = 0;
#[cfg (feature = "use_lucid")]
pub const GCForeground: c_ulong = 4;
#[cfg (feature = "use_lucid")]
pub const GCBackground: c_ulong = 8;
#[cfg (feature = "use_lucid")]
pub const GCLineStyle: c_ulong = 32;
#[cfg (feature = "use_lucid")]
pub const GCFillStyle: c_ulong = 256;
#[cfg (feature = "use_lucid")]
pub const GCStipple: c_ulong = 2048;
#[cfg (feature = "use_lucid")]
pub const GCFont: c_ulong = 16384;
#[cfg (feature = "use_lucid")]
pub const GCGraphicsExposures: c_ulong = 65536;
#[cfg (feature = "use_lucid")]
pub const FillStippled: c_int = 2;
#[cfg (feature = "use_lucid")]
pub const GrabModeAsync: c_int = 1;
#[cfg (feature = "use_lucid")]
pub const Success: c_int = 0;
#[cfg (feature = "use_lucid")]
pub const ButtonPressMask: c_long = 4;
#[cfg (feature = "use_lucid")]
pub const ButtonReleaseMask: c_long = 8;
#[cfg (feature = "use_lucid")]
pub const PointerMotionMask: c_long = 64;
#[cfg (feature = "use_lucid")]
pub const PointerMotionHintMask: c_long = 128;
#[cfg (feature = "use_lucid")]
pub const XA_ATOM: Atom = 4;
#[cfg (feature = "use_lucid")]
pub const PropModeReplace: c_int = 0;
#[cfg (feature = "use_lucid")]
pub const XC_right_ptr: c_uint = 94;
/* `<X11/X.h>`'s window-attribute masks and the two shell widget classes the create path names
   (C 1379-1385, 1429-1431, 2204, 2846-2848) - the masks are measured, the classes UNVERIFIED.  */
#[cfg (feature = "use_lucid")]
pub const CWSaveUnder: c_ulong = 1 << 10;
#[cfg (feature = "use_lucid")]
pub const CWCursor: c_ulong = 1 << 14;
#[cfg (feature = "use_lucid")]
pub const ExposureMask: c_long = 32768;
#[cfg (feature = "use_lucid")]
pub const XtGrabExclusive: c_int = 1;                 // C: `<X11/Intrinsic.h>`: `#define XtGrabExclusive ((Cardinal) 1)` - UNVERIFIED

/* ---- `<X11/Xlib.h>` and `<X11/Xutil.h>` entry points this file calls (UNVERIFIED: the
   declarations are C's prototypes, the names are the library's) ---- */
#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  /// C: Xlib.h 3256: `extern int XAllocColor (Display *, Colormap, XColor *);`
  pub fn XAllocColor (dpy: *mut Display, cmap: Colormap, xc: *mut XColor) -> c_int;
  /// C: Xlib.h: `extern int XChangeGC (Display *, GC, unsigned long, XGCValues *);`
  pub fn XChangeGC (dpy: *mut Display, gc: GC, valuemask: c_ulong,
                    values: *mut XGCValues) -> c_int;
  /// C: Xlib.h: `extern int XChangeProperty (Display *, Window, Atom, Atom, int, int,
  /// const unsigned char *, int);`
  pub fn XChangeProperty (dpy: *mut Display, w: Window, property: Atom, r#type: Atom,
                          format: c_int, mode: c_int, data: *const c_uchar, nelements: c_int)
                          -> c_int;
  /// C: Xlib.h: `extern int XChangeWindowAttributes (Display *, Window, unsigned long,
  /// XSetWindowAttributes *);`
  pub fn XChangeWindowAttributes (dpy: *mut Display, w: Window, valuemask: c_ulong,
                                 attributes: *mut XSetWindowAttributes) -> c_int;
  /// C: Xlib.h: `extern int XClearArea (Display *, Window, int, int, unsigned int,
  /// unsigned int, Bool);`
  pub fn XClearArea (dpy: *mut Display, w: Window, x: c_int, y: c_int, width: c_uint,
                     height: c_uint, exposures: c_int) -> c_int;
  /// C: Xlib.h: `extern int XCopyArea (Display *, Drawable, Drawable, GC, int, int,
  /// unsigned int, unsigned int, int, int);`
  pub fn XCopyArea (dpy: *mut Display, src: Drawable, dest: Drawable, gc: GC, src_x: c_int,
                    src_y: c_int, width: c_uint, height: c_uint, dest_x: c_int, dest_y: c_int)
                    -> c_int;
  /// C: Xlib.h: `extern Pixmap XCreatePixmap (Display *, Drawable, unsigned int,
  /// unsigned int, unsigned int);`
  pub fn XCreatePixmap (dpy: *mut Display, d: Drawable, width: c_uint, height: c_uint,
                        depth: c_uint) -> Pixmap;
  /// C: Xlib.h: `extern Pixmap XCreatePixmapFromBitmapData (Display *, Drawable, char *,
  /// unsigned int, unsigned int, unsigned long, unsigned long, unsigned int);`
  pub fn XCreatePixmapFromBitmapData (dpy: *mut Display, d: Drawable, data: *mut c_char,
                                      width: c_uint, height: c_uint, fg: c_ulong, bg: c_ulong,
                                      depth: c_uint) -> Pixmap;
  /// C: Xlib.h: `extern int XDrawLine (Display *, Drawable, GC, int, int, int, int);`
  pub fn XDrawLine (dpy: *mut Display, d: Drawable, gc: GC, x1: c_int, y1: c_int, x2: c_int,
                    y2: c_int) -> c_int;
  /// C: Xlib.h: `extern int XDrawString (Display *, Drawable, GC, int, int, const char *, int);`
  pub fn XDrawString (dpy: *mut Display, d: Drawable, gc: GC, x: c_int, y: c_int,
                      string: *const c_char, length: c_int) -> c_int;
  /// C: Xlib.h: `extern int XFillPolygon (Display *, Drawable, GC, XPoint *, int, int, int);`
  pub fn XFillPolygon (dpy: *mut Display, d: Drawable, gc: GC, points: *mut XPoint,
                       npoints: c_int, shape: c_int, mode: c_int) -> c_int;
  /// C: Xlib.h: `extern int XFillRectangle (Display *, Drawable, GC, int, int,
  /// unsigned int, unsigned int);`
  pub fn XFillRectangle (dpy: *mut Display, d: Drawable, gc: GC, x: c_int, y: c_int,
                         width: c_uint, height: c_uint) -> c_int;
  /// C: Xlib.h: `extern int XFlush (Display *);`
  pub fn XFlush (dpy: *mut Display) -> c_int;
  /// C: Xlib.h: `extern int XFreeFont (Display *, XFontStruct *);`
  pub fn XFreeFont (dpy: *mut Display, font: *mut XFontStruct) -> c_int;
  /// C: Xlib.h: `extern int XFreePixmap (Display *, Pixmap);`
  pub fn XFreePixmap (dpy: *mut Display, pixmap: Pixmap) -> c_int;
  /// C: Xlib.h: `extern Atom XInternAtom (Display *, const char *, Bool);`
  pub fn XInternAtom (dpy: *mut Display, name: *const c_char, only_if_exists: c_int) -> Atom;
  /// C: Xlib.h: `extern XFontStruct *XLoadQueryFont (Display *, const char *);`
  pub fn XLoadQueryFont (dpy: *mut Display, name: *const c_char) -> *mut XFontStruct;
  /// C: Xlib.h: `extern int XQueryColor (Display *, Colormap, XColor *);`
  pub fn XQueryColor (dpy: *mut Display, cmap: Colormap, def: *mut XColor) -> c_int;
  /// C: Xlib.h: `extern int XQueryColors (Display *, Colormap, XColor *, int);`
  pub fn XQueryColors (dpy: *mut Display, cmap: Colormap, defs: *mut XColor, ncolors: c_int)
                       -> c_int;
  /// C: Xlib.h: `extern int XQueryPointer (Display *, Window, Window *, Window *, int *, int *,
  /// int *, int *, unsigned int *);`
  pub fn XQueryPointer (dpy: *mut Display, w: Window, root: *mut Window,
                        child: *mut Window, root_x: *mut c_int, root_y: *mut c_int,
                        win_x: *mut c_int, win_y: *mut c_int, mask: *mut c_uint) -> c_int;
  /// C: Xlib.h: `extern int XScreenNumberOfScreen (Screen *);`
  pub fn XScreenNumberOfScreen (screen: *mut Screen) -> c_int;
  /// C: Xlib.h: `extern int XSetWindowBackground (Display *, Window, unsigned long);`
  pub fn XSetWindowBackground (dpy: *mut Display, w: Window, background: c_ulong) -> c_int;
  /// C: Xlib.h: `extern int XSync (Display *, Bool);`
  pub fn XSync (dpy: *mut Display, discard: c_int) -> c_int;
  /// C: Xlib.h: `extern int XTextExtents (XFontStruct *, const char *, int, int *, int *,
  /// int *, XCharStruct *);`
  pub fn XTextExtents (font: *mut XFontStruct, string: *const c_char, nchars: c_int,
                       direction: *mut c_int, font_ascent: *mut c_int, font_descent: *mut c_int,
                       overall: *mut XCharStruct) -> c_int;
  /// C: Xlib.h: `extern int XmbTextExtents (XFontSet, const char *, int, XRectangle *,
  /// XRectangle *);`
  pub fn XmbTextExtents (font_set: XFontSet, string: *const c_char, count: c_int,
                         ink: *mut XRectangle, logical: *mut XRectangle) -> c_int;
  /// C: Xlib.h: `extern void XmbDrawString (Display *, Drawable, XFontSet, GC, int, int,
  /// const char *, int);`
  pub fn XmbDrawString (dpy: *mut Display, d: Drawable, font_set: XFontSet, gc: GC, x: c_int,
                        y: c_int, string: *const c_char, count: c_int);
  /// C: Xlib.h: `extern int XExtentsOfFontSet (XFontSet);`
  pub fn XExtentsOfFontSet (font_set: XFontSet) -> *mut XFontSetExtents;
  /// C: Xlib.h: `extern int DefaultDepthOfScreen (Screen *);`
  pub fn DefaultDepthOfScreen (screen: *mut Screen) -> c_int;
  /// C: Xlib.h: `extern Screen *DefaultScreenOfDisplay (Display *);`
  pub fn DefaultScreenOfDisplay (dpy: *mut Display) -> *mut Screen;
  /// C: Xlib.h: `extern Visual *DefaultVisual (Display *, int);`
  pub fn DefaultVisual (dpy: *mut Display, screen_number: c_int) -> *mut Visual;
  /// C: Xlib.h: `extern Window RootWindowOfScreen (Screen *);`
  pub fn RootWindowOfScreen (screen: *mut Screen) -> Window;
  /// C: Xlib.h: `extern int WidthOfScreen (Screen *);`
  pub fn WidthOfScreen (screen: *mut Screen) -> c_int;
  /// C: Xlib.h: `extern int HeightOfScreen (Screen *);`
  pub fn HeightOfScreen (screen: *mut Screen) -> c_int;
}

/* ---- `<X11/Intrinsic.h>` and `<X11/IntrinsicP.h>` entry points this file calls.  Four of them
   are macros in C (`XtDisplay`, `XtParent`, `XtScreen`, `XtWindow`, and the `XtIsRealized`/
   `XtIsShell` predicates); the port declares them as entry points, the `lwlib-Xlw.rs` and
   `lwlib-Xm.rs` reading of the same shape. ---- */
#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  /// C: Intrinsic.h: `extern void XtAddEventHandler (Widget, EventMask, Boolean, XtEventHandler,
  /// XtPointer);`
  pub fn XtAddEventHandler (w: Widget, event_mask: c_long, nonmaskable: Boolean,
                            proc: Option<unsafe extern "C" fn (Widget, XtPointer, *mut XEvent,
                                                              *mut Boolean)>,
                            client_data: XtPointer);
  /// C: Intrinsic.h: `extern void XtAddGrab (Widget, Boolean, Boolean);`
  pub fn XtAddGrab (w: Widget, exclusive: Boolean, spring_loaded: Boolean);
  /// C: Intrinsic.h: `extern void XtCallCallbackList (Widget, XtCallbackList, XtPointer);`
  pub fn XtCallCallbackList (widget: Widget, callbacks: XtCallbackList, call_data: XtPointer);
  /// C: Intrinsic.h: `extern char *XtCalloc (Cardinal, Cardinal);`
  pub fn XtCalloc (num: Cardinal, size: Cardinal) -> *mut c_char;
  /// C: Intrinsic.h: `extern void XtConfigureWidget (Widget, Position, Position, Dimension,
  /// Dimension, Dimension);`
  pub fn XtConfigureWidget (w: Widget, x: Position, y: Position, width: Dimension,
                            height: Dimension, border_width: Dimension);
  /// C: Intrinsic.h: `extern Widget XtCreatePopupShell (const char *, WidgetClass, Widget, ArgList,
  /// Cardinal);`
  pub fn XtCreatePopupShell (name: *const c_char, widget_class: WidgetClass, parent: Widget,
                             args: ArgList, num_args: Cardinal) -> Widget;
  /// C: Intrinsic.h: `#define XtDisplay(w) ((w)->core.screen->display)` - a macro in C.
  pub fn XtDisplay (w: Widget) -> *mut Display;
  /// C: Intrinsic.h: `extern void XtFree (char *);`
  pub fn XtFree (ptr: *mut c_char);
  /// C: Intrinsic.h: `extern GC XtGetGC (Widget, XtGCMask, XGCValues *);`
  pub fn XtGetGC (w: Widget, value_mask: XtGCMask, values: *mut XGCValues) -> GC;
  /// C: Intrinsic.h: `extern Time XtLastTimestampProcessed (Display *);`
  pub fn XtLastTimestampProcessed (dpy: *mut Display) -> Time;
  /// C: Intrinsic.h: `extern int XtGetMultiClickTime (Display *);`
  pub fn XtGetMultiClickTime (dpy: *mut Display) -> c_int;
  /// C: Intrinsic.h: `extern void XtGetSubresources (Widget, XtPointer, const char *,
  /// const char *, XtResourceList, Cardinal, ArgList, Cardinal);` - a variadic function in C
  /// (`XtVaGetSubresources`'s non-variadic sibling keeps the two `NULL, 0` arguments).
  pub fn XtGetSubresources (w: Widget, base: XtPointer, name: *const c_char,
                            class: *const c_char, resources: *mut XtResource,
                            num_resources: Cardinal, args: ArgList, num_args: Cardinal);
  /// C: Intrinsic.h: `extern void XtGrabKeyboard (Widget, Boolean, int, int, Time);`
  pub fn XtGrabKeyboard (widget: Widget, owner_events: Boolean, pointer_mode: c_int,
                         keyboard_mode: c_int, time: Time) -> c_int;
  /// C: Intrinsic.h: `extern void XtGrabPointer (Widget, Boolean, unsigned int, int, int,
  /// Window, Cursor, Time);`
  pub fn XtGrabPointer (widget: Widget, owner_events: Boolean, event_mask: c_uint,
                        pointer_mode: c_int, keyboard_mode: c_int, confine_to: Window,
                        cursor: Cursor, time: Time) -> c_int;
  /// C: IntrinsicP.h: `#define XtIsRealized(object) (XtWindowOfObject(object) != 0)` - a macro.
  pub fn XtIsRealized (w: Widget) -> Boolean;
  /// C: Intrinsic.h: `#define XtIsShell(w) XtIsSubclass (w, shellWidgetClass)` - a macro.
  pub fn XtIsShell (w: Widget) -> Boolean;
  /// C: Intrinsic.h: `extern char *XtMalloc (Cardinal);`
  pub fn XtMalloc (size: Cardinal) -> *mut c_char;
  /// C: IntrinsicP.h: `#define XtParent(w) ((w)->core.parent)` - a macro.
  pub fn XtParent (w: Widget) -> Widget;
  /// C: Intrinsic.h: `extern void XtPopdown (Widget);`
  pub fn XtPopdown (popup: Widget);
  /// C: Intrinsic.h: `extern void XtPopup (Widget, XtGrabKind);`
  pub fn XtPopup (popup: Widget, grab_kind: c_int);
  /// C: Intrinsic.h: `extern void XtRealizeWidget (Widget);`
  pub fn XtRealizeWidget (w: Widget);
  /// C: Intrinsic.h: `extern char *XtRealloc (char *, Cardinal);`
  pub fn XtRealloc (ptr: *mut c_char, num: Cardinal) -> *mut c_char;
  /// C: Intrinsic.h: `extern void XtReleaseGC (Widget, GC);`
  pub fn XtReleaseGC (w: Widget, gc: GC);
  /// C: Intrinsic.h: `extern void XtRemoveGrab (Widget);`
  pub fn XtRemoveGrab (w: Widget);
  /// C: IntrinsicP.h: `#define XtScreen(w) ((w)->core.screen)` - a macro.
  pub fn XtScreen (w: Widget) -> *mut Screen;
  /// C: Intrinsic.h: `extern void XtSetKeyboardFocus (Widget, Widget);`
  pub fn XtSetKeyboardFocus (subtree: Widget, descendant: Widget);
  /// C: Intrinsic.h: `extern void XtUngrabKeyboard (Widget, Time);`
  pub fn XtUngrabKeyboard (widget: Widget, time: Time);
  /// C: Intrinsic.h: `extern void XtUngrabPointer (Widget, Time);`
  pub fn XtUngrabPointer (widget: Widget, time: Time);
  /// C: Intrinsic.h: `extern void XtUnrealizeWidget (Widget);`
  pub fn XtUnrealizeWidget (w: Widget);
  /// C: IntrinsicP.h: `#define XtWindow(w) ((w)->core.window)` - a macro.
  pub fn XtWindow (w: Widget) -> Window;
  /// C: Intrinsic.h: `extern WidgetClass shellWidgetClass;` (`XtIsShell`'s second operand).
  pub static mut shellWidgetClass: WidgetClass;
  /// C: Shell.h: `extern WidgetClass overrideShellWidgetClass;` (`make_windows_if_needed`'s and
  /// `xlw_create_popup_menu`'s popup class; the `lwlib-Xlw.rs` 144-147 reading).
  pub static mut overrideShellWidgetClass: WidgetClass;
}

/* ---- `<X11/Xlib.h>`'s integer/atom types and the Lucid/Xft half ---- */
/// C: lwlib-utils.h: `typedef struct _widget_xft_data widget_xft_data;` - the tag
/// `widget_instance.xft_data` names; `lwlib-int.h` gates the field on `USE_CAIRO`/`HAVE_XFT`
/// and the header that defines the tag is the arm this unit answers for itself (above).
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct widget_xft_data { pub _private: [u8; 0] }
/// C: Xlib.h: `typedef struct _XFontSet *XFontSet;` - Xlib's i18n font set.
#[cfg (feature = "use_lucid")]
pub type XFontSet = *mut c_void;
/// C: Xlib.h: `typedef struct { XRectangle max_ink_extent, max_logical_extent; }
/// XFontSetExtents;` - 16 bytes (two measured `XRectangle`s).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XFontSetExtents { pub max_ink_extent: XRectangle,
                             pub max_logical_extent: XRectangle }
/// C: Xutil.h: `typedef struct { long xOff, yOff; } XGlyphInfo;` under the Xft/cairo arm -
/// **UNVERIFIED** (Xft is not on this machine): C's `<X11/Xft/Xft.h>` spelling.
#[cfg (feature = "use_cairo_or_have_xft")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XGlyphInfo {
  pub width: c_ushort, pub height: c_ushort, pub x: c_short, pub y: c_short,
  pub xOff: c_short, pub yOff: c_short,
}
/// C: `<X11/Xft/Xft.h>`: `typedef struct _XftDraw XftDraw;` - **UNVERIFIED**; Xft's own
/// record is opaque here (C 1158's `cairo_get_target (ws->xft_draw)` and C 1546's
/// `XftDrawDestroy (ws->xft_draw)` are the two shapes the port spells with a cast).
#[cfg (feature = "use_cairo_or_have_xft")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XftDraw { pub _private: [u8; 0] }
/// C: `<X11/Xft/Xft.h>`: `typedef struct _XftFont { int ascent, descent, height, … } XftFont;`
/// - **UNVERIFIED**; the three members `string_width` and the `MENU_FONT_*` macros read.
#[cfg (feature = "use_cairo_or_have_xft")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XftFont { pub ascent: c_int, pub descent: c_int, pub height: c_int,
                     pub max_advance_width: c_int }
/// C: `<X11/Xft/Xft.h>`: `typedef struct { short red, green, blue, alpha; } XftColor;` -
/// **UNVERIFIED**.
#[cfg (feature = "use_cairo_or_have_xft")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XftColor { pub pixel: c_ulong, pub color: XRenderColor }
/// C: `<X11/extensions/Xrender.h>`: `typedef struct { unsigned short red, green, blue, alpha; }
/// XRenderColor;` - **UNVERIFIED** (through the cairo/Xft arm only).
#[cfg (feature = "use_cairo_or_have_xft")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XRenderColor { pub red: c_ushort, pub green: c_ushort, pub blue: c_ushort,
                          pub alpha: c_ushort }
#[cfg (feature = "use_cairo_or_have_xft")]
unsafe extern "C" {
  /// C: `<X11/Xft/Xft.h>`: `extern void XftTextExtentsUtf8 (Display *, XftFont *, const FcChar8 *,
  /// int, XGlyphInfo *);`
  pub fn XftTextExtentsUtf8 (dpy: *mut Display, font: *mut XftFont, string: *const FcChar8,
                             len: c_int, extents: *mut XGlyphInfo);
  /// C: `<X11/Xft/Xft.h>`: `extern void XftDrawStringUtf8 (XftDraw *, const XftColor *,
  /// XftFont *, int, int, const FcChar8 *, int);`
  pub fn XftDrawStringUtf8 (d: *mut XftDraw, color: *const XftColor, font: *mut XftFont,
                            x: c_int, y: c_int, string: *const FcChar8, len: c_int);
  /// C: `<X11/Xft/Xft.h>`: `extern XftFont *XftFontOpenName (Display *, int, const char *);`
  pub fn XftFontOpenName (dpy: *mut Display, screen: c_int, name: *const c_char)
                          -> *mut XftFont;
  /// C: `<X11/Xft/Xft.h>`: `extern void XftFontClose (Display *, XftFont *);`
  pub fn XftFontClose (dpy: *mut Display, font: *mut XftFont);
  /// C: `<X11/Xft/Xft.h>`: `extern XftDraw *XftDrawCreate (Display *, Drawable, Visual *,
  /// Colormap);`
  pub fn XftDrawCreate (dpy: *mut Display, drawable: Drawable, visual: *mut Visual,
                        colormap: Colormap) -> *mut XftDraw;
  /// C: `<X11/Xft/Xft.h>`: `extern void XftDrawDestroy (XftDraw *);`
  pub fn XftDrawDestroy (draw: *mut XftDraw);
}
/// C: `<fontconfig/fontconfig.h>`: `typedef unsigned char FcChar8;` - **UNVERIFIED**.
#[cfg (feature = "use_cairo_or_have_xft")]
pub type FcChar8 = c_uchar;
/// C: cairo's `<cairo.h>`: `typedef void *cairo_t;` and `cairo_surface_t *cairo_get_target
/// (cairo_t *);` under `USE_CAIRO` - **UNVERIFIED** (the shapes are cairo's).
#[cfg (feature = "use_cairo")]
pub type cairo_t = *mut c_void;
#[cfg (feature = "use_cairo")]
pub type cairo_surface_t = *mut c_void;
/// C: cairo's `<cairo.h>`: `typedef struct _cairo_scaled_font cairo_scaled_font_t;`
#[cfg (feature = "use_cairo")]
pub type cairo_scaled_font_t = *mut c_void;
#[cfg (feature = "use_cairo")]
unsafe extern "C" {
  /// C: `<cairo.h>`: `extern cairo_surface_t *cairo_get_target (cairo_t *);`
  pub fn cairo_get_target (cr: cairo_t) -> cairo_surface_t;
  /// C: `<cairo.h>`: `extern void cairo_surface_flush (cairo_surface_t *);`
  pub fn cairo_surface_flush (surface: cairo_surface_t);
  /// C: `<cairo.h>`: `extern void cairo_surface_mark_dirty (cairo_surface_t *);`
  pub fn cairo_surface_mark_dirty (surface: cairo_surface_t);
}

/* ---- `src/xterm.h`'s four entry points and one global (C 40's `#include <xterm.h>`, the
   `emacs` arm).  No mirror in this tree carries them under this unit's gate (`src/include/
   xterm.rs` is the `have_x_windows` arm's mirror), so they are declared here with C's own line
   (the `lwlib-Xm.rs` 16 / `src/term.rs` reading of the same situation). ---- */
#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  /// C: src/xterm.h 1752: `extern void x_catch_errors (Display *);`
  pub fn x_catch_errors (dpy: *mut Display);
  /// C: src/xterm.h 1764: `extern void x_uncatch_errors (void);`
  pub fn x_uncatch_errors ();
  /// C: src/xterm.h 1762: `extern bool x_had_errors_p (Display *);`
  pub fn x_had_errors_p (dpy: *mut Display) -> bool;
  /// C: src/xterm.h 1792-1794: `extern bool x_alloc_lighter_color_for_widget (Widget, Display *,
  /// Colormap, unsigned long *, double, int);`
  pub fn x_alloc_lighter_color_for_widget (widget: Widget, dpy: *mut Display, cmap: Colormap,
                                          pixel: *mut c_ulong, factor: f64,
                                          delta: c_int) -> bool;
  /// C: src/xterm.h 1980-1981: `extern void x_free_dpy_colors (Display *, Screen *, Colormap,
  /// unsigned long *, int);`
  pub fn x_free_dpy_colors (dpy: *mut Display, screen: *mut Screen, cmap: Colormap,
                            pixels: *mut c_ulong, n: c_int);
  /// C: src/xterm.h 1913-1914: `extern void xlw_monitor_dimensions_at_pos (Display *, Screen *,
  /// int, int, int *, int *, int *, int *);`
  pub fn xlw_monitor_dimensions_at_pos (dpy: *mut Display, screen: *mut Screen, x: c_int,
                                        y: c_int, x_min: *mut c_int, y_min: *mut c_int,
                                        width: *mut c_int, height: *mut c_int);
}

/* ---- C 41's `#include "bitmaps/gray.xbm"` (in the `emacs` arm) and C 46's
   `#include <X11/bitmaps/gray>` (the standalone arm): the same three names, and X11's bitmap
   file C fetches carries the same 2x2 data. ---- */
/// C: emacs/src/bitmaps/gray.xbm 1: `#define gray_width 2`
#[cfg (feature = "use_lucid")]
pub const gray_width: c_int = 2;
/// C: emacs/src/bitmaps/gray.xbm 2: `#define gray_height 2`
#[cfg (feature = "use_lucid")]
pub const gray_height: c_int = 2;
/// C: emacs/src/bitmaps/gray.xbm 3-4: `static char gray_bits[] = { '\x01', '\x02' };`
#[cfg (feature = "use_lucid")]
static mut gray_bits: [c_char; 2] = [1, 2];

/* ================================ the translation unit ================================ */

/* C 50-52: the three file-scope statics the grab code shares.  */
#[cfg (feature = "use_lucid")]
static mut pointer_grabbed: c_int = 0;
#[cfg (feature = "use_lucid")]
static mut keyboard_grabbed: c_int = 0;
#[cfg (feature = "use_lucid")]
static mut menu_post_event: XEvent = XEvent { _private: [0; 192] };

/* C 54-94: `xlwMenuTranslations` (static).  The C literal is one `\`-continued string whose
   bytes are the table Xt parses; the port keeps the same bytes as a `&CStr`.  */
#[cfg (feature = "use_lucid")]
static xlwMenuTranslations: &core::ffi::CStr = c"<BtnDown>:\t  start()\n<Motion>:\t  drag()\n<BtnUp>:\t  select()\n<Key>Shift_L:     nothing()\n<Key>Shift_R:     nothing()\n<Key>Meta_L:      nothing()\n<Key>Meta_R:      nothing()\n<Key>Control_L:   nothing()\n<Key>Control_R:   nothing()\n<Key>Hyper_L:     nothing()\n<Key>Hyper_R:     nothing()\n<Key>Super_L:     nothing()\n<Key>Super_R:     nothing()\n<Key>Alt_L:       nothing()\n<Key>Alt_R:       nothing()\n<Key>Caps_Lock:   nothing()\n<Key>Shift_Lock:  nothing()\n<KeyUp>Shift_L:   nothing()\n<KeyUp>Shift_R:   nothing()\n<KeyUp>Meta_L:    nothing()\n<KeyUp>Meta_R:    nothing()\n<KeyUp>Control_L: nothing()\n<KeyUp>Control_R: nothing()\n<KeyUp>Hyper_L:   nothing()\n<KeyUp>Hyper_R:   nothing()\n<KeyUp>Super_L:   nothing()\n<KeyUp>Super_R:   nothing()\n<KeyUp>Alt_L:     nothing()\n<KeyUp>Alt_R:     nothing()\n<KeyUp>Caps_Lock: nothing()\n<KeyUp>Shift_Lock:nothing()\n<Key>Return:      select()\n<Key>Down:        down()\n<Key>Up:          up()\n<Key>Left:        left()\n<Key>Right:       right()\n<Key>:            key()\n<KeyUp>:          key()\n";

/* C 96-102: three FIXME comments with no code.  */

/* C 103: `#define offset(field) offsetof (XlwMenuRec, field)` - C's `field` argument is always
   the two-token `menu.<member>` path, so the port's macro takes it in that shape (an
   `offsetof`-style `offset_of!` needs the path spelled out).  The offset's width is C's
   `size_t` narrowed to `Cardinal` by the field it initialises.  */
#[cfg (feature = "use_lucid")]
macro_rules! offset {
  ($field:ident . $sub:ident) => {
    ::core::mem::offset_of! (XlwMenuRec, $field.$sub) as Cardinal
  };
}

/* C 104-169: `xlwMenuResources`.  C's `#if defined USE_CAIRO || defined HAVE_XFT` arm selects
   `DEFAULT_FONTNAME` (C 112) and its `#else` arm the string at C 114; both are carried.  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
macro_rules! DEFAULT_FONTNAME { () => { c"Sans-10".as_ptr () }; }       // C 112
#[cfg (all (feature = "use_lucid", not (feature = "use_cairo_or_have_xft")))]
macro_rules! DEFAULT_FONTNAME { () => { c"XtDefaultFont".as_ptr () }; } // C 114
/* The frozen configuration's own value, as a constant (P-08): the macro above is what the
   resource table and `openXftFont` read under each arm; this is the same bytes C 114 defines.  */
#[cfg (all (feature = "use_lucid", not (feature = "use_cairo_or_have_xft")))]
pub const DEFAULT_FONTNAME: &core::ffi::CStr = c"XtDefaultFont";
/* C's tables are arrays whose length C reads with `XtNumber`; the port declares the length as a
   name so the array's type and the class record's count cannot drift apart (Rust's const
   evaluator may not read another static's value, which is why the count is not `len ()`).  */
#[cfg (all (feature = "use_lucid", feature = "have_x_i18n"))]
pub const XLWMENU_RESOURCES_LEN: usize = 25;
#[cfg (all (feature = "use_lucid", not (feature = "have_x_i18n")))]
pub const XLWMENU_RESOURCES_LEN: usize = 24;
#[cfg (feature = "use_lucid")]
pub const XLWMENU_ACTIONS_LEN: usize = 10;
#[cfg (feature = "use_lucid")]
static mut xlwMenuResources: [XtResource; XLWMENU_RESOURCES_LEN] = [
  /* C 107-110: `#ifdef HAVE_X_I18N` */
  #[cfg (feature = "have_x_i18n")]
  XtResource { resource_name: XtNfontSet.as_ptr () as String,
               resource_class: XtCFontSet.as_ptr () as String,
               resource_type: XtRFontSet.as_ptr () as String,
               resource_size: ::core::mem::size_of::<XFontSet> () as Cardinal,
               resource_offset: offset! (menu.fontSet),
               default_type: XtRFontSet.as_ptr () as String,
               default_addr: core::ptr::null_mut () },
  XtResource { resource_name: XtNfont.as_ptr () as String,
               resource_class: XtCFont.as_ptr () as String,
               resource_type: XtRString.as_ptr () as String,
               resource_size: ::core::mem::size_of::<String> () as Cardinal,
               resource_offset: offset! (menu.fontName),
               default_type: XtRString.as_ptr () as String,
               default_addr: DEFAULT_FONTNAME! () as XtPointer },
  XtResource { resource_name: XtNforeground.as_ptr () as String,
               resource_class: XtCForeground.as_ptr () as String,
               resource_type: XtRPixel.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Pixel> () as Cardinal,
               resource_offset: offset! (menu.foreground),
               default_type: XtRString.as_ptr () as String,
               default_addr: c"XtDefaultForeground".as_ptr () as XtPointer },
  XtResource { resource_name: XtNdisabledForeground.as_ptr () as String,
               resource_class: XtCDisabledForeground.as_ptr () as String,
               resource_type: XtRPixel.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Pixel> () as Cardinal,
               resource_offset: offset! (menu.disabled_foreground),
               default_type: XtRString.as_ptr () as String,
               default_addr: core::ptr::null_mut () },
  XtResource { resource_name: XtNbuttonForeground.as_ptr () as String,
               resource_class: XtCButtonForeground.as_ptr () as String,
               resource_type: XtRPixel.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Pixel> () as Cardinal,
               resource_offset: offset! (menu.button_foreground),
               default_type: XtRString.as_ptr () as String,
               default_addr: c"XtDefaultForeground".as_ptr () as XtPointer },
  XtResource { resource_name: XtNhighlightForeground.as_ptr () as String,
               resource_class: XtCHighlightForeground.as_ptr () as String,
               resource_type: XtRPixel.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Pixel> () as Cardinal,
               resource_offset: offset! (menu.highlight_foreground),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: (-1isize) as XtPointer },
  XtResource { resource_name: XtNhighlightBackground.as_ptr () as String,
               resource_class: XtCHighlightBackground.as_ptr () as String,
               resource_type: XtRPixel.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Pixel> () as Cardinal,
               resource_offset: offset! (menu.highlight_background),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: (-1isize) as XtPointer },
  XtResource { resource_name: XtNmargin.as_ptr () as String,
               resource_class: XtCMargin.as_ptr () as String,
               resource_type: XtRDimension.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Dimension> () as Cardinal,
               resource_offset: offset! (menu.margin),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: 1isize as XtPointer },
  XtResource { resource_name: XtNhorizontalSpacing.as_ptr () as String,
               resource_class: XtCMargin.as_ptr () as String,
               resource_type: XtRDimension.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Dimension> () as Cardinal,
               resource_offset: offset! (menu.horizontal_spacing),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: 3isize as XtPointer },
  XtResource { resource_name: XtNverticalSpacing.as_ptr () as String,
               resource_class: XtCMargin.as_ptr () as String,
               resource_type: XtRDimension.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Dimension> () as Cardinal,
               resource_offset: offset! (menu.vertical_spacing),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: 2isize as XtPointer },
  XtResource { resource_name: XtNarrowSpacing.as_ptr () as String,
               resource_class: XtCMargin.as_ptr () as String,
               resource_type: XtRDimension.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Dimension> () as Cardinal,
               resource_offset: offset! (menu.arrow_spacing),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: 10isize as XtPointer },
  /* C 137-147: the five Lucid resource names `xlwmenu.h` and `<Xm/…>` provide.  */
  XtResource { resource_name: XmNshadowThickness.as_ptr () as String,
               resource_class: XmCShadowThickness.as_ptr () as String,
               resource_type: XtRDimension.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Dimension> () as Cardinal,
               resource_offset: offset! (menu.shadow_thickness),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: 1isize as XtPointer },
  XtResource { resource_name: XmNtopShadowColor.as_ptr () as String,
               resource_class: XmCTopShadowColor.as_ptr () as String,
               resource_type: XtRPixel.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Pixel> () as Cardinal,
               resource_offset: offset! (menu.top_shadow_color),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: (-1isize) as XtPointer },
  XtResource { resource_name: XmNbottomShadowColor.as_ptr () as String,
               resource_class: XmCBottomShadowColor.as_ptr () as String,
               resource_type: XtRPixel.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Pixel> () as Cardinal,
               resource_offset: offset! (menu.bottom_shadow_color),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: (-1isize) as XtPointer },
  XtResource { resource_name: XmNtopShadowPixmap.as_ptr () as String,
               resource_class: XmCTopShadowPixmap.as_ptr () as String,
               resource_type: XtRPixmap.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Pixmap> () as Cardinal,
               resource_offset: offset! (menu.top_shadow_pixmap),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: None as XtPointer },
  XtResource { resource_name: XmNbottomShadowPixmap.as_ptr () as String,
               resource_class: XmCBottomShadowPixmap.as_ptr () as String,
               resource_type: XtRPixmap.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Pixmap> () as Cardinal,
               resource_offset: offset! (menu.bottom_shadow_pixmap),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: None as XtPointer },
  XtResource { resource_name: XtNopen.as_ptr () as String,
               resource_class: XtCCallback.as_ptr () as String,
               resource_type: XtRCallback.as_ptr () as String,
               resource_size: ::core::mem::size_of::<XtPointer> () as Cardinal,
               resource_offset: offset! (menu.open),
               default_type: XtRCallback.as_ptr () as String,
               default_addr: core::ptr::null_mut () },
  XtResource { resource_name: XtNselect.as_ptr () as String,
               resource_class: XtCCallback.as_ptr () as String,
               resource_type: XtRCallback.as_ptr () as String,
               resource_size: ::core::mem::size_of::<XtPointer> () as Cardinal,
               resource_offset: offset! (menu.select),
               default_type: XtRCallback.as_ptr () as String,
               default_addr: core::ptr::null_mut () },
  XtResource { resource_name: XtNhighlightCallback.as_ptr () as String,
               resource_class: XtCCallback.as_ptr () as String,
               resource_type: XtRCallback.as_ptr () as String,
               resource_size: ::core::mem::size_of::<XtPointer> () as Cardinal,
               resource_offset: offset! (menu.highlight),
               default_type: XtRCallback.as_ptr () as String,
               default_addr: core::ptr::null_mut () },
  XtResource { resource_name: XtNenterCallback.as_ptr () as String,
               resource_class: XtCCallback.as_ptr () as String,
               resource_type: XtRCallback.as_ptr () as String,
               resource_size: ::core::mem::size_of::<XtPointer> () as Cardinal,
               resource_offset: offset! (menu.enter),
               default_type: XtRCallback.as_ptr () as String,
               default_addr: core::ptr::null_mut () },
  XtResource { resource_name: XtNleaveCallback.as_ptr () as String,
               resource_class: XtCCallback.as_ptr () as String,
               resource_type: XtRCallback.as_ptr () as String,
               resource_size: ::core::mem::size_of::<XtPointer> () as Cardinal,
               resource_offset: offset! (menu.leave),
               default_type: XtRCallback.as_ptr () as String,
               default_addr: core::ptr::null_mut () },
  XtResource { resource_name: XtNmenu.as_ptr () as String,
               resource_class: XtCMenu.as_ptr () as String,
               resource_type: XtRPointer.as_ptr () as String,
               resource_size: ::core::mem::size_of::<XtPointer> () as Cardinal,
               resource_offset: offset! (menu.contents),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: core::ptr::null_mut () },
  XtResource { resource_name: XtNcursor.as_ptr () as String,
               resource_class: XtCCursor.as_ptr () as String,
               resource_type: XtRCursor.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Cursor> () as Cardinal,
               resource_offset: offset! (menu.cursor_shape),
               default_type: XtRString.as_ptr () as String,
               default_addr: c"right_ptr".as_ptr () as XtPointer },
  XtResource { resource_name: XtNhorizontal.as_ptr () as String,
               resource_class: XtCHorizontal.as_ptr () as String,
               resource_type: XtRInt.as_ptr () as String,
               resource_size: ::core::mem::size_of::<c_int> () as Cardinal,
               resource_offset: offset! (menu.horizontal),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: True as XtPointer },
  XtResource { resource_name: XtNborderThickness.as_ptr () as String,
               resource_class: XtCBorderThickness.as_ptr () as String,
               resource_type: XtRDimension.as_ptr () as String,
               resource_size: ::core::mem::size_of::<Dimension> () as Cardinal,
               resource_offset: offset! (menu.border_thickness),
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: 1isize as XtPointer },
];
/* C 169: `#undef offset` - a preprocessor effect with no Rust counterpart.  */

/* C 171-189: C's fourteen static forward declarations are **deleted** (PORTING.md 0.1-12 /
   checklist 10); Rust items are order-independent and the ledger names the region.  */

/* C 191-204: `xlwMenuActionsList` (static).  */
#[cfg (feature = "use_lucid")]
static mut xlwMenuActionsList: [XtActionsRec; XLWMENU_ACTIONS_LEN] = [
  XtActionsRec { string: c"start".as_ptr () as String, proc: Some (Start) },
  XtActionsRec { string: c"drag".as_ptr () as String, proc: Some (Drag) },
  XtActionsRec { string: c"down".as_ptr () as String, proc: Some (Down) },
  XtActionsRec { string: c"up".as_ptr () as String, proc: Some (Up) },
  XtActionsRec { string: c"left".as_ptr () as String, proc: Some (Left) },
  XtActionsRec { string: c"right".as_ptr () as String, proc: Some (Right) },
  XtActionsRec { string: c"select".as_ptr () as String, proc: Some (Select) },
  XtActionsRec { string: c"key".as_ptr () as String, proc: Some (Key) },
  /* Compatibility with Lesstif/Motif.  */
  XtActionsRec { string: c"MenuGadgetEscape".as_ptr () as String, proc: Some (Key) },
  XtActionsRec { string: c"nothing".as_ptr () as String, proc: Some (Nothing) },
];

/* C 206: `#define SuperClass ((CoreWidgetClass)&coreClassRec)`.  Xt's own
   `CoreClassRec coreClassRec` is the object it names; the cast's type has no mirror here, so the
   port declares the tag (`CoreWidgetClass`) and the object beside it.  */
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CoreClassRec { pub core_class: CoreClassPart }
#[cfg (feature = "use_lucid")]
pub type CoreWidgetClass = *mut CoreClassRec;
#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  /// C: CoreP.h: `extern CoreClassRec coreClassRec;` (`SuperClass`'s operand).
  pub static mut coreClassRec: CoreClassRec;
}
#[cfg (feature = "use_lucid")]
macro_rules! SuperClass {
  () => { (&raw mut coreClassRec) as CoreWidgetClass as WidgetClass };
}

/* C 208-247: `xlwMenuClassRec`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub static mut xlwMenuClassRec: XlwMenuClassRec = XlwMenuClassRec {
  core_class: CoreClassPart {
    superclass: SuperClass! (),                            /* superclass            */
    class_name: c"XlwMenu".as_ptr () as String,            /* class_name            */
    widget_size: ::core::mem::size_of::<XlwMenuRec> () as Cardinal, /* size          */
    class_initialize: Option::Some (XlwMenuClassInitialize),       /* class_initialize      */
    class_part_initialize: Option::None,                           /* class_part_initialize */
    class_inited: False,                                   /* class_inited          */
    initialize: Option::Some (XlwMenuInitialize),                  /* initialize            */
    initialize_hook: Option::None,                                 /* initialize_hook       */
    realize: Option::Some (XlwMenuRealize),                        /* realize               */
    actions: (&raw mut xlwMenuActionsList) as *mut XtActionsRec, /* actions          */
    num_actions: XtNumber! (XtActionsRec, XLWMENU_ACTIONS_LEN), /* num_actions       */
    resources: (&raw mut xlwMenuResources) as *mut XtResource, /* resources           */
    resource_count: XtNumber! (XtResource, XLWMENU_RESOURCES_LEN), /* resource_count */
    xrm_class: core::ptr::null_mut (),                     /* xrm_class             */
    compress_motion: True,                                 /* compress_motion       */
    compress_exposure: XtExposeCompressMaximal,            /* compress_exposure     */
    compress_enterleave: True,                             /* compress_enterleave   */
    visible_interest: False,                               /* visible_interest      */
    destroy: Option::Some (XlwMenuDestroy),                        /* destroy               */
    resize: Option::Some (XlwMenuResize),                          /* resize                */
    expose: Option::Some (XlwMenuRedisplay),                       /* expose                */
    set_values: Option::Some (XlwMenuSetValues),                   /* set_values            */
    set_values_hook: Option::None,                                 /* set_values_hook       */
    set_values_almost: Option::None,                               /* set_values_almost     */
    get_values_hook: Option::None,                                 /* get_values_hook       */
    accept_focus: Option::None,                                    /* accept_focus          */
    version: XtVersion,                                    /* version               */
    callback_private: core::ptr::null_mut (),              /* callback_private      */
    tm_table: xlwMenuTranslations.as_ptr () as String,     /* tm_table              */
    query_geometry: Option::None,                                  /* query_geometry        */
    display_accelerator: Option::None,                             /* display_accelerator   */
    extension: core::ptr::null_mut (),                     /* extension             */
  },  /* XlwMenuClass fields initialization */
  menu_class: XlwMenuClassPart {
    dummy: 0                                               /* dummy                 */
  },
};

/* C 249: `WidgetClass xlwMenuWidgetClass = (WidgetClass) &xlwMenuClassRec;` - non-static, so the
   port exports it (C `lwlib-Xlw.c` reads it).  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub static mut xlwMenuWidgetClass: WidgetClass =
  (&raw mut xlwMenuClassRec) as WidgetClass;

/* C 251 and C 253: the two file-scope statics the redisplay path shares.  */
#[cfg (feature = "use_lucid")]
static mut submenu_destroyed: c_int = 0;
#[cfg (feature = "use_lucid")]
static mut next_release_must_exit: c_int = 0;

/* Utilities */

/* C 258-265: `ungrab_all` (static; its address is not taken).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn ungrab_all (w: Widget, ungrabtime: Time)
{
  XtUngrabPointer (w, ungrabtime);

  if keyboard_grabbed != 0
    { XtUngrabKeyboard (w, ungrabtime); }
}

/* Like abort, but remove grabs from widget W before.  */

/* C 269-276: `abort_gracefully` (static, `_Noreturn`).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn abort_gracefully (w: Widget) -> !
{
  if XtIsShell (XtParent (w)) != 0
    { XtRemoveGrab (w); }
  ungrab_all (w, CurrentTime);
  emacs_abort ();
}

/* C 278-296: `push_new_stack` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn push_new_stack (mw: XlwMenuWidget, val: *mut widget_value)
{
  if (*mw).menu.new_stack.is_null ()
    {
      (*mw).menu.new_stack_length = 10;
      (*mw).menu.new_stack =
        XtCalloc ((*mw).menu.new_stack_length as Cardinal,
                  ::core::mem::size_of::<*mut widget_value> () as Cardinal)
          as *mut *mut widget_value;
    }
  else if (*mw).menu.new_depth == (*mw).menu.new_stack_length
    {
      (*mw).menu.new_stack_length *= 2;
      (*mw).menu.new_stack =
        XtRealloc ((*mw).menu.new_stack as *mut c_char,
                   ((*mw).menu.new_stack_length as usize
                    * ::core::mem::size_of::<*mut widget_value> ()) as Cardinal)
          as *mut *mut widget_value;
    }
  (*(*mw).menu.new_stack.offset ((*mw).menu.new_depth as isize)) = val;
  (*mw).menu.new_depth += 1;
}

/* C 298-306: `pop_new_stack_if_no_contents` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn pop_new_stack_if_no_contents (mw: XlwMenuWidget)
{
  if (*mw).menu.new_depth > 1
    {
      if (*(*(*mw).menu.new_stack.offset (((*mw).menu.new_depth - 1) as isize))).contents.is_null ()
        { (*mw).menu.new_depth -= 1; }
    }
}

/* C 308-325: `make_old_stack_space` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn make_old_stack_space (mw: XlwMenuWidget, n: c_int)
{
  if (*mw).menu.old_stack.is_null ()
    {
      (*mw).menu.old_stack_length = 10;
      (*mw).menu.old_stack =
        XtCalloc ((*mw).menu.old_stack_length as Cardinal,
                  ::core::mem::size_of::<*mut widget_value> () as Cardinal)
          as *mut *mut widget_value;
    }
  else if (*mw).menu.old_stack_length < n
    {
      (*mw).menu.old_stack_length *= 2;
      (*mw).menu.old_stack =
        XtRealloc ((*mw).menu.old_stack as *mut c_char,
                   ((*mw).menu.old_stack_length as usize
                    * ::core::mem::size_of::<*mut widget_value> ()) as Cardinal)
          as *mut *mut widget_value;
    }
}

/* Size code */

/* C 328-355: `string_width` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn string_width (mw: XlwMenuWidget, s: *mut c_char) -> c_int
{
  let mut xcs: XCharStruct = core::mem::zeroed ();
  let mut drop: c_int = 0;
  /* C 333-342: `#if defined USE_CAIRO || defined HAVE_XFT` */
  #[cfg (feature = "use_cairo_or_have_xft")]
  {
    if !(*mw).menu.xft_font.is_null ()
      {
        let mut gi: XGlyphInfo = core::mem::zeroed ();
        XftTextExtentsUtf8 (XtDisplay (mw as Widget), (*mw).menu.xft_font,
                            s as *const FcChar8,
                            strlen (s) as c_int, &raw mut gi);
        return gi.xOff as c_int;
      }
  }
  /* C 343-350: `#ifdef HAVE_X_I18N` */
  #[cfg (feature = "have_x_i18n")]
  {
    if !(*mw).menu.fontSet.is_null ()
      {
        let mut ink: XRectangle = core::mem::zeroed ();
        let mut logical: XRectangle = core::mem::zeroed ();
        XmbTextExtents ((*mw).menu.fontSet, s, strlen (s) as c_int, &raw mut ink,
                        &raw mut logical);
        return logical.width as c_int;
      }
  }

  XTextExtents ((*mw).menu.font, s, strlen (s) as c_int, &raw mut drop, &raw mut drop,
                &raw mut drop, &raw mut xcs);
  xcs.width as c_int

}

/* C 357-385: the `MENU_FONT_HEIGHT`/`MENU_FONT_ASCENT` pair, one definition per arm.  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
macro_rules! MENU_FONT_HEIGHT {
  ($mw:expr) => {
    (if !(*$mw).menu.xft_font.is_null () { (*(*$mw).menu.xft_font).height }
     else if !(*$mw).menu.fontSet.is_null () {
       ((*(*$mw).menu.font_extents).max_logical_extent.height) as c_int }
     else { ((*(*$mw).menu.font).ascent as c_int) + ((*(*$mw).menu.font).descent as c_int) })
  };
}
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
macro_rules! MENU_FONT_ASCENT {
  ($mw:expr) => {
    (if !(*$mw).menu.xft_font.is_null () { (*(*$mw).menu.xft_font).ascent }
     else if !(*$mw).menu.fontSet.is_null () {
       -(((*(*$mw).menu.font_extents).max_logical_extent.y) as c_int) }
     else { (*(*$mw).menu.font).ascent as c_int })
  };
}
#[cfg (all (feature = "use_lucid", feature = "have_x_i18n",
            not (feature = "use_cairo_or_have_xft")))]
macro_rules! MENU_FONT_HEIGHT {
  ($mw:expr) => {
    (if !(*$mw).menu.fontSet.is_null () {
       ((*(*$mw).menu.font_extents).max_logical_extent.height) as c_int }
     else { ((*(*$mw).menu.font).ascent as c_int) + ((*(*$mw).menu.font).descent as c_int) })
  };
}
#[cfg (all (feature = "use_lucid", feature = "have_x_i18n",
            not (feature = "use_cairo_or_have_xft")))]
macro_rules! MENU_FONT_ASCENT {
  ($mw:expr) => {
    (if !(*$mw).menu.fontSet.is_null () {
       -(((*(*$mw).menu.font_extents).max_logical_extent.y) as c_int) }
     else { (*(*$mw).menu.font).ascent as c_int })
  };
}
#[cfg (all (feature = "use_lucid", not (feature = "have_x_i18n"),
            not (feature = "use_cairo_or_have_xft")))]
macro_rules! MENU_FONT_HEIGHT {
  ($mw:expr) => {
    (((*(*$mw).menu.font).ascent as c_int) + ((*(*$mw).menu.font).descent as c_int))
  };
}
#[cfg (all (feature = "use_lucid", not (feature = "have_x_i18n"),
            not (feature = "use_cairo_or_have_xft")))]
macro_rules! MENU_FONT_ASCENT {
  ($mw:expr) => { ((*(*$mw).menu.font).ascent as c_int) };
}

/* C 387-391: `arrow_width` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn arrow_width (mw: XlwMenuWidget) -> c_int
{
  return (MENU_FONT_ASCENT! (mw) * 3 / 4) | 1;
}

/* Return the width of toggle buttons of widget MW.  */

/* C 395-399: `toggle_button_width` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn toggle_button_width (mw: XlwMenuWidget) -> c_int
{
  return (MENU_FONT_HEIGHT! (mw) * 2 / 3) | 1;
}

/* Return the width of radio buttons of widget MW.  */

/* C 404-408: `radio_button_width` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn radio_button_width (mw: XlwMenuWidget) -> c_int
{
  return (toggle_button_width (mw) as f64 * 1.41) as c_int;
}

/* C 411-416: `nameResource` (static).  */
#[cfg (feature = "use_lucid")]
static mut nameResource: [XtResource; 1] = [
  XtResource { resource_name: c"labelString".as_ptr () as String,
               resource_class: c"LabelString".as_ptr () as String,
               resource_type: XtRString.as_ptr () as String,
               resource_size: ::core::mem::size_of::<String> () as Cardinal,
               resource_offset: 0,
               default_type: XtRImmediate.as_ptr () as String,
               default_addr: core::ptr::null_mut () },
];

/* C 418-450: `resource_widget_value` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn resource_widget_value (mw: XlwMenuWidget,
                                                   val: *mut widget_value) -> *mut c_char
{
  if (*val).toolkit_data.is_null ()
    {
      let mut resourced_name: *mut c_char = core::ptr::null_mut ();
      let complete_name: *mut c_char;
      XtGetSubresources (mw as Widget,
                         (&raw mut resourced_name) as XtPointer,
                         (*val).name, (*val).name,
                         (&raw mut nameResource) as *mut XtResource, 1,
                         core::ptr::null_mut (), 0);
      if resourced_name.is_null ()
        { resourced_name = (*val).name; }
      if (*val).value.is_null ()
        {
          complete_name = XtMalloc ((strlen (resourced_name) + 1) as Cardinal);
          strcpy (complete_name, resourced_name);
        }
      else
        {
          let complete_length: c_int =
            (strlen (resourced_name) + strlen ((*val).value) + 2) as c_int;
          complete_name = XtMalloc (complete_length as Cardinal);
          let mut z: *mut c_char = stpcpy (complete_name, resourced_name);
          *z = b' ' as c_char;
          z = z.offset (1);
          strcpy (z, (*val).value);
        }

      (*val).toolkit_data = complete_name as *mut c_void;
      (*val).free_toolkit_data = true;
    }
  return (*val).toolkit_data as *mut c_char;
}

/* Returns the sizes of an item */
/* C 452-499: `size_menu_item` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn size_menu_item (mw: XlwMenuWidget,
                                            val: *mut widget_value,
                                            horizontal_p: c_int,
                                            label_width: *mut c_int,
                                            rest_width: *mut c_int,
                                            button_width: *mut c_int,
                                            height: *mut c_int)
{
  let mut separator: menu_separator = 0;

  if lw_separator_p ((*val).name, &raw mut separator, 0) != 0
    {
      *height = separator_height (separator);
      *label_width = 1;
      *rest_width = 0;
      *button_width = 0;
    }
  else
    {
      *height = MENU_FONT_HEIGHT! (mw)
        + 2 * ((*mw).menu.vertical_spacing as c_int)
        + 2 * ((*mw).menu.shadow_thickness as c_int);

      *label_width =
        string_width (mw, resource_widget_value (mw, val))
          + ((*mw).menu.horizontal_spacing as c_int)
          + ((*mw).menu.shadow_thickness as c_int);

      *rest_width = (*mw).menu.horizontal_spacing as c_int
                    + ((*mw).menu.shadow_thickness as c_int);
      if horizontal_p == 0
        {
          if !(*val).contents.is_null ()
            /* Add width of the arrow displayed for submenus.  */
            { *rest_width += arrow_width (mw) + ((*mw).menu.arrow_spacing as c_int); }
          else if !(*val).key.is_null ()
            /* Add width of key equivalent string.  */
            { *rest_width += string_width (mw, (*val).key)
                             + ((*mw).menu.arrow_spacing as c_int); }

          if (*val).button_type == BUTTON_TYPE_TOGGLE
            { *button_width = toggle_button_width (mw)
                              + ((*mw).menu.horizontal_spacing as c_int); }
          else if (*val).button_type == BUTTON_TYPE_RADIO
            { *button_width = radio_button_width (mw)
                              + ((*mw).menu.horizontal_spacing as c_int); }
        }
    }
}

/* C 501-562: `size_menu` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn size_menu (mw: XlwMenuWidget, level: c_int)
{
  let mut label_width: c_int = 0;
  let mut rest_width: c_int = 0;
  let mut button_width: c_int = 0;
  let mut max_rest_width: c_int = 0;
  let mut max_button_width: c_int = 0;
  let mut height: c_int = 0;
  let horizontal_p =
    if (*mw).menu.horizontal != 0 && level == 0 { 1 } else { 0 };
  let mut val: *mut widget_value;
  let ws: *mut window_state;

  if level >= (*mw).menu.old_depth
    { abort_gracefully (mw as Widget); }

  ws = &raw mut (*(*mw).menu.windows.offset (level as isize));
  (*ws).width = 0;
  (*ws).height = 0;
  (*ws).label_width = 0;
  (*ws).button_width = 0;

  val = (*(*(*mw).menu.old_stack.offset (level as isize))).contents;
  while !val.is_null ()
    {
      size_menu_item (mw, val, horizontal_p, &raw mut label_width, &raw mut rest_width,
                      &raw mut button_width, &raw mut height);
      if horizontal_p != 0
        {
          (*ws).width += (label_width + rest_width) as Dimension;
          if height > (*ws).height as c_int
            { (*ws).height = height as Dimension; }
        }
      else
        {
          if label_width > (*ws).label_width as c_int
            { (*ws).label_width = label_width as Dimension; }
          if rest_width > max_rest_width
            { max_rest_width = rest_width; }
          if button_width > max_button_width
            { max_button_width = button_width; }
          (*ws).height += height as Dimension;
        }
      val = (*val).next;
    }

  if horizontal_p != 0
    { (*ws).label_width = 0; (*ws).button_width = 0; }
  else
    {
      (*ws).width = ((*ws).label_width as c_int + max_rest_width
                     + max_button_width) as Dimension;
      (*ws).button_width = max_button_width as Dimension;
    }

  (*ws).width += (2 * ((*mw).menu.shadow_thickness as c_int)) as Dimension;
  (*ws).height += (2 * ((*mw).menu.shadow_thickness as c_int)) as Dimension;
  (*ws).max_rest_width = max_rest_width;

  if horizontal_p != 0
    {
      (*ws).width += (2 * ((*mw).menu.margin as c_int)) as Dimension;
      (*ws).height += (2 * ((*mw).menu.margin as c_int)) as Dimension;
    }
}


/* Display code */

/* C 567-628: `draw_arrow` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn draw_arrow (mw: XlwMenuWidget,
                                        window: Window,
                                        gc: GC,
                                        x: c_int,
                                        y: c_int,
                                        width: c_int,
                                        down_p: c_int)
{
  let dpy: *mut Display = XtDisplay (mw as Widget);
  let top_gc: GC;
  let mut bottom_gc: GC;
  let thickness = (*mw).menu.shadow_thickness as c_int;
  let height = width;
  let mut pt: [XPoint; 10] = core::mem::zeroed ();
  /* alpha = atan (0.5)
     factor = (1 + sin (alpha)) / cos (alpha) */
  let factor: f64 = 1.62;
  let thickness2 = (thickness as f64 * factor) as c_int;
  let mut x = x;
  let mut y = y;

  y += (MENU_FONT_HEIGHT! (mw) - height) / 2;

  if down_p != 0
    {
      top_gc = (*mw).menu.highlight_shadow_bottom_gc;
      bottom_gc = (*mw).menu.highlight_shadow_top_gc;
    }
  else
    {
      top_gc = (*mw).menu.shadow_top_gc;
      bottom_gc = (*mw).menu.shadow_bottom_gc;
    }

  pt[0].x = x as c_short;
  pt[0].y = (y + height) as c_short;
  pt[1].x = (x + thickness) as c_short;
  pt[1].y = (y + height - thickness2) as c_short;
  pt[2].x = (x + thickness2) as c_short;
  pt[2].y = (y + thickness2) as c_short;
  pt[3].x = x as c_short;
  pt[3].y = y as c_short;
  XFillPolygon (dpy, window, top_gc, pt.as_mut_ptr (), 4, Convex, CoordModeOrigin);

  pt[0].x = x as c_short;
  pt[0].y = y as c_short;
  pt[1].x = (x + thickness) as c_short;
  pt[1].y = (y + thickness2) as c_short;
  pt[2].x = (x + width - thickness2) as c_short;
  pt[2].y = (y + height / 2) as c_short;
  pt[3].x = (x + width) as c_short;
  pt[3].y = (y + height / 2) as c_short;
  XFillPolygon (dpy, window, top_gc, pt.as_mut_ptr (), 4, Convex, CoordModeOrigin);

  pt[0].x = x as c_short;
  pt[0].y = (y + height) as c_short;
  pt[1].x = (x + thickness) as c_short;
  pt[1].y = (y + height - thickness2) as c_short;
  pt[2].x = (x + width - thickness2) as c_short;
  pt[2].y = (y + height / 2) as c_short;
  pt[3].x = (x + width) as c_short;
  pt[3].y = (y + height / 2) as c_short;
  XFillPolygon (dpy, window, bottom_gc, pt.as_mut_ptr (), 4, Convex, CoordModeOrigin);
}

/* Generic draw shadow rectangle function.  It is used to draw shadows
   on menus, menu items and also toggle buttons.  When ERASE_P is
   true, it clears shadows.  DOWN_P is true when a menu item is pushed
   or a button toggled.  TOP_GC and BOTTOM_GC are the graphic contexts
   used to draw the top and bottom shadow respectively.  */
/* C 635-718: `draw_shadow_rectangle` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn draw_shadow_rectangle (mw: XlwMenuWidget, window: Window,
                                                   x: c_int, y: c_int,
                                                   width: c_int, height: c_int,
                                                   erase_p: c_int, down_p: c_int,
                                                   top_gc: GC, bottom_gc: GC)
{
  let dpy: *mut Display = XtDisplay (mw as Widget);
  let thickness = if x == 0 && y == 0 { (*mw).menu.border_thickness as c_int }
                  else { (*mw).menu.shadow_thickness as c_int };
  let mut points: [XPoint; 4] = core::mem::zeroed ();
  let mut top_gc = top_gc;
  let mut bottom_gc = bottom_gc;

  /* Choose correct GC with a standard default if NULL.  */
  if erase_p != 0
    {
      top_gc = (*mw).menu.background_gc;
      bottom_gc = (*mw).menu.background_gc;
    }
  else
    {
      if top_gc.is_null ()
        { top_gc = (*mw).menu.shadow_top_gc; }
      if bottom_gc.is_null ()
        { bottom_gc = (*mw).menu.shadow_bottom_gc; }
    }

  if erase_p == 0 && width == height && width == toggle_button_width (mw)
    {
      points [0].x = x as c_short;
      points [0].y = y as c_short;
      points [1].x = (x + width) as c_short;
      points [1].y = y as c_short;
      points [2].x = (x + width) as c_short;
      points [2].y = (y + height) as c_short;
      points [3].x = x as c_short;
      points [3].y = (y + height) as c_short;
      XFillPolygon (dpy, window,
                    if down_p != 0 { (*mw).menu.button_gc }
                    else { (*mw).menu.inactive_button_gc },
                    points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
    }

  if erase_p == 0 && down_p != 0
    {
      let temp: GC;
      temp = top_gc;
      top_gc = bottom_gc;
      bottom_gc = temp;
    }

  /* Do draw (or erase) shadows */
  points [0].x = x as c_short;
  points [0].y = y as c_short;
  points [1].x = (x + width) as c_short;
  points [1].y = y as c_short;
  points [2].x = (x + width - thickness) as c_short;
  points [2].y = (y + thickness) as c_short;
  points [3].x = x as c_short;
  points [3].y = (y + thickness) as c_short;
  XFillPolygon (dpy, window, top_gc, points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
  points [0].x = x as c_short;
  points [0].y = (y + thickness) as c_short;
  points [1].x = x as c_short;
  points [1].y = (y + height) as c_short;
  points [2].x = (x + thickness) as c_short;
  points [2].y = (y + height - thickness) as c_short;
  points [3].x = (x + thickness) as c_short;
  points [3].y = (y + thickness) as c_short;
  XFillPolygon (dpy, window, top_gc, points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
  points [0].x = (x + width) as c_short;
  points [0].y = y as c_short;
  points [1].x = (x + width - thickness) as c_short;
  points [1].y = (y + thickness) as c_short;
  points [2].x = (x + width - thickness) as c_short;
  points [2].y = (y + height - thickness) as c_short;
  points [3].x = (x + width) as c_short;
  points [3].y = (y + height - thickness) as c_short;
  XFillPolygon (dpy, window, bottom_gc, points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
  points [0].x = x as c_short;
  points [0].y = (y + height) as c_short;
  points [1].x = (x + width) as c_short;
  points [1].y = (y + height) as c_short;
  points [2].x = (x + width) as c_short;
  points [2].y = (y + height - thickness) as c_short;
  points [3].x = (x + thickness) as c_short;
  points [3].y = (y + height - thickness) as c_short;
  XFillPolygon (dpy, window, bottom_gc, points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
}

/* C 721-803: `draw_shadow_rhombus` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn draw_shadow_rhombus (mw: XlwMenuWidget, window: Window,
                                                 x: c_int, y: c_int,
                                                 width: c_int, height: c_int,
                                                 erase_p: c_int, down_p: c_int,
                                                 top_gc: GC, bottom_gc: GC)
{
  let dpy: *mut Display = XtDisplay (mw as Widget);
  let thickness = (*mw).menu.shadow_thickness as c_int;
  let mut points: [XPoint; 4] = core::mem::zeroed ();
  let mut top_gc = top_gc;
  let mut bottom_gc = bottom_gc;

  /* Choose correct GC with a standard default if NULL */
  if erase_p != 0
    {
      top_gc = (*mw).menu.background_gc;
      bottom_gc = (*mw).menu.background_gc;
    }
  else
    {
      if top_gc.is_null ()
        { top_gc = (*mw).menu.shadow_top_gc; }
      if bottom_gc.is_null ()
        { top_gc = (*mw).menu.shadow_bottom_gc; }
    }

  if erase_p == 0 && width == height && width == radio_button_width (mw)
    {
      points [0].x = x as c_short;
      points [0].y = (y + width / 2) as c_short;
      points [1].x = (x + height / 2) as c_short;
      points [1].y = (y + width) as c_short;
      points [2].x = (x + height) as c_short;
      points [2].y = (y + width / 2) as c_short;
      points [3].x = (x + height / 2) as c_short;
      points [3].y = y as c_short;
      XFillPolygon (dpy, window,
                    if down_p != 0 { (*mw).menu.button_gc }
                    else { (*mw).menu.inactive_button_gc },
                    points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
    }

  if erase_p == 0 && down_p != 0
    {
      let temp: GC;
      temp = top_gc;
      top_gc = bottom_gc;
      bottom_gc = temp;
    }

  points [0].x = x as c_short;
  points [0].y = (y + height / 2) as c_short;
  points [1].x = (x + thickness) as c_short;
  points [1].y = (y + height / 2) as c_short;
  points [2].x = (x + width / 2) as c_short;
  points [2].y = (y + thickness) as c_short;
  points [3].x = (x + width / 2) as c_short;
  points [3].y = y as c_short;
  XFillPolygon (dpy, window, top_gc, points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
  points [0].x = (x + width / 2) as c_short;
  points [0].y = y as c_short;
  points [1].x = (x + width / 2) as c_short;
  points [1].y = (y + thickness) as c_short;
  points [2].x = (x + width - thickness) as c_short;
  points [2].y = (y + height / 2) as c_short;
  points [3].x = (x + width) as c_short;
  points [3].y = (y + height / 2) as c_short;
  XFillPolygon (dpy, window, top_gc, points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
  points [0].x = x as c_short;
  points [0].y = (y + height / 2) as c_short;
  points [1].x = (x + thickness) as c_short;
  points [1].y = (y + height / 2) as c_short;
  points [2].x = (x + width / 2) as c_short;
  points [2].y = (y + height - thickness) as c_short;
  points [3].x = (x + width / 2) as c_short;
  points [3].y = (y + height) as c_short;
  XFillPolygon (dpy, window, bottom_gc, points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
  points [0].x = (x + width / 2) as c_short;
  points [0].y = (y + height) as c_short;
  points [1].x = (x + width / 2) as c_short;
  points [1].y = (y + height - thickness) as c_short;
  points [2].x = (x + width - thickness) as c_short;
  points [2].y = (y + height / 2) as c_short;
  points [3].x = (x + width) as c_short;
  points [3].y = (y + height / 2) as c_short;
  XFillPolygon (dpy, window, bottom_gc, points.as_mut_ptr (), 4, Convex, CoordModeOrigin);
}

/* Draw a toggle button on widget MW, X window WINDOW.  X/Y is the
   top-left corner of the menu item.  SELECTED_P non-zero means the
   toggle button is selected.  */

/* C 810-834: `draw_toggle` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn draw_toggle (mw: XlwMenuWidget, window: Window, x: c_int,
                                         y: c_int, selected_p: c_int,
                                         highlighted_p: c_int)
{
  let mut width: c_int;
  let height: c_int;
  let top_gc: GC;
  let bottom_gc: GC;
  let mut x = x;
  let mut y = y;

  if highlighted_p != 0
    {
      top_gc = (*mw).menu.highlight_shadow_top_gc;
      bottom_gc = (*mw).menu.highlight_shadow_bottom_gc;
    }
  else
    {
      top_gc = (*mw).menu.shadow_top_gc;
      bottom_gc = (*mw).menu.shadow_bottom_gc;
    }

  width = toggle_button_width (mw);
  height = width;
  x += (*mw).menu.horizontal_spacing as c_int;
  y += (MENU_FONT_ASCENT! (mw) - height) / 2;
  draw_shadow_rectangle (mw, window, x, y, width, height, False,
                         selected_p, top_gc, bottom_gc);
}


/* Draw a radio button on widget MW, X window WINDOW.  X/Y is the
   top-left corner of the menu item.  SELECTED_P non-zero means the
   toggle button is selected.  */

/* C 841-865: `draw_radio` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn draw_radio (mw: XlwMenuWidget, window: Window, x: c_int,
                                        y: c_int, selected_p: c_int,
                                        highlighted_p: c_int)
{
  let mut width: c_int;
  let height: c_int;
  let top_gc: GC;
  let bottom_gc: GC;
  let mut x = x;
  let mut y = y;

  if highlighted_p != 0
    {
      top_gc = (*mw).menu.highlight_shadow_top_gc;
      bottom_gc = (*mw).menu.highlight_shadow_bottom_gc;
    }
  else
    {
      top_gc = (*mw).menu.shadow_top_gc;
      bottom_gc = (*mw).menu.shadow_bottom_gc;
    }

  width = radio_button_width (mw);
  height = width;
  x += (*mw).menu.horizontal_spacing as c_int;
  y += (MENU_FONT_ASCENT! (mw) - height) / 2;
  draw_shadow_rhombus (mw, window, x, y, width, height, False, selected_p,
                       top_gc, bottom_gc);
}

/* Draw a menu separator on widget MW, X window WINDOW.  X/Y is the
   top-left corner of the menu item.  WIDTH is the width of the
   separator to draw.  TYPE is the separator type.  */

/* C 872-982: `draw_separator` (static).  C's `switch`/`case`/`break` become a `match` and its
   arms (PORTING.md 2.2; the ledger's `macro-body-notes.tsv` row names the four tokens).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn draw_separator (mw: XlwMenuWidget,
                                           window: Window,
                                           x: c_int,
                                           y: c_int,
                                           width: c_int,
                                           r#type: menu_separator)
{
  let dpy: *mut Display = XtDisplay (mw as Widget);
  let mut xgcv: XGCValues = core::mem::zeroed ();

  match r#type
    {
    SEPARATOR_NO_LINE => {}

    SEPARATOR_SINGLE_LINE => {
      XDrawLine (dpy, window, (*mw).menu.foreground_gc,
                 x, y, x + width, y);
    }

    SEPARATOR_DOUBLE_LINE => {
      draw_separator (mw, window, x, y, width, SEPARATOR_SINGLE_LINE);
      draw_separator (mw, window, x, y + 2, width, SEPARATOR_SINGLE_LINE);
    }

    SEPARATOR_SINGLE_DASHED_LINE => {
      xgcv.line_style = LineOnOffDash;
      XChangeGC (dpy, (*mw).menu.foreground_gc, GCLineStyle, &raw mut xgcv);
      XDrawLine (dpy, window, (*mw).menu.foreground_gc,
                 x, y, x + width, y);
      xgcv.line_style = LineSolid;
      XChangeGC (dpy, (*mw).menu.foreground_gc, GCLineStyle, &raw mut xgcv);
    }

    SEPARATOR_DOUBLE_DASHED_LINE => {
      draw_separator (mw, window, x, y, width,
                      SEPARATOR_SINGLE_DASHED_LINE);
      draw_separator (mw, window, x, y + 2, width,
                      SEPARATOR_SINGLE_DASHED_LINE);
    }

    SEPARATOR_SHADOW_ETCHED_IN => {
      XDrawLine (dpy, window, (*mw).menu.shadow_bottom_gc,
                 x, y, x + width, y);
      XDrawLine (dpy, window, (*mw).menu.shadow_top_gc,
                 x, y + 1, x + width, y + 1);
    }

    SEPARATOR_SHADOW_ETCHED_OUT => {
      XDrawLine (dpy, window, (*mw).menu.shadow_top_gc,
                 x, y, x + width, y);
      XDrawLine (dpy, window, (*mw).menu.shadow_bottom_gc,
                 x, y + 1, x + width, y + 1);
    }

    SEPARATOR_SHADOW_ETCHED_IN_DASH => {
      xgcv.line_style = LineOnOffDash;
      XChangeGC (dpy, (*mw).menu.shadow_bottom_gc, GCLineStyle, &raw mut xgcv);
      XChangeGC (dpy, (*mw).menu.shadow_top_gc, GCLineStyle, &raw mut xgcv);
      draw_separator (mw, window, x, y, width, SEPARATOR_SHADOW_ETCHED_IN);
      xgcv.line_style = LineSolid;
      XChangeGC (dpy, (*mw).menu.shadow_bottom_gc, GCLineStyle, &raw mut xgcv);
      XChangeGC (dpy, (*mw).menu.shadow_top_gc, GCLineStyle, &raw mut xgcv);
    }

    SEPARATOR_SHADOW_ETCHED_OUT_DASH => {
      xgcv.line_style = LineOnOffDash;
      XChangeGC (dpy, (*mw).menu.shadow_bottom_gc, GCLineStyle, &raw mut xgcv);
      XChangeGC (dpy, (*mw).menu.shadow_top_gc, GCLineStyle, &raw mut xgcv);
      draw_separator (mw, window, x, y, width, SEPARATOR_SHADOW_ETCHED_OUT);
      xgcv.line_style = LineSolid;
      XChangeGC (dpy, (*mw).menu.shadow_bottom_gc, GCLineStyle, &raw mut xgcv);
      XChangeGC (dpy, (*mw).menu.shadow_top_gc, GCLineStyle, &raw mut xgcv);
    }

    SEPARATOR_SHADOW_DOUBLE_ETCHED_IN => {
      draw_separator (mw, window, x, y, width, SEPARATOR_SHADOW_ETCHED_IN);
      draw_separator (mw, window, x, y + 3, width, SEPARATOR_SHADOW_ETCHED_IN);
    }

    SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT => {
      draw_separator (mw, window, x, y, width,
                      SEPARATOR_SHADOW_ETCHED_OUT);
      draw_separator (mw, window, x, y + 3, width,
                      SEPARATOR_SHADOW_ETCHED_OUT);
    }

    SEPARATOR_SHADOW_DOUBLE_ETCHED_IN_DASH => {
      xgcv.line_style = LineOnOffDash;
      XChangeGC (dpy, (*mw).menu.shadow_bottom_gc, GCLineStyle, &raw mut xgcv);
      XChangeGC (dpy, (*mw).menu.shadow_top_gc, GCLineStyle, &raw mut xgcv);
      draw_separator (mw, window, x, y, width,
                      SEPARATOR_SHADOW_DOUBLE_ETCHED_IN);
      xgcv.line_style = LineSolid;
      XChangeGC (dpy, (*mw).menu.shadow_bottom_gc, GCLineStyle, &raw mut xgcv);
      XChangeGC (dpy, (*mw).menu.shadow_top_gc, GCLineStyle, &raw mut xgcv);
    }

    SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT_DASH => {
      xgcv.line_style = LineOnOffDash;
      XChangeGC (dpy, (*mw).menu.shadow_bottom_gc, GCLineStyle, &raw mut xgcv);
      XChangeGC (dpy, (*mw).menu.shadow_top_gc, GCLineStyle, &raw mut xgcv);
      draw_separator (mw, window, x, y, width,
                      SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT);
      xgcv.line_style = LineSolid;
      XChangeGC (dpy, (*mw).menu.shadow_bottom_gc, GCLineStyle, &raw mut xgcv);
      XChangeGC (dpy, (*mw).menu.shadow_top_gc, GCLineStyle, &raw mut xgcv);
    }

    _ => {
      emacs_abort ();
    }
    }
}


/* Return the pixel height of menu separator SEPARATOR.  */

/* C 990-1023: `separator_height` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn separator_height (separator: menu_separator) -> c_int
{
  match separator
    {
    SEPARATOR_NO_LINE => {
      return 2;
    }

    SEPARATOR_SINGLE_LINE | SEPARATOR_SINGLE_DASHED_LINE => {
      return 1;
    }

    SEPARATOR_DOUBLE_LINE | SEPARATOR_DOUBLE_DASHED_LINE => {
      return 3;
    }

    SEPARATOR_SHADOW_ETCHED_IN | SEPARATOR_SHADOW_ETCHED_OUT
    | SEPARATOR_SHADOW_ETCHED_IN_DASH | SEPARATOR_SHADOW_ETCHED_OUT_DASH => {
      return 2;
    }

    SEPARATOR_SHADOW_DOUBLE_ETCHED_IN | SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT
    | SEPARATOR_SHADOW_DOUBLE_ETCHED_IN_DASH
    | SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT_DASH => {
      return 5;
    }

    _ => {
      emacs_abort ();
    }
    }
}

/* Draw the highlighted background and shadows.  */

/* C 1025-1050: `draw_highlight` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn draw_highlight (mw: XlwMenuWidget, window: Window, x: c_int,
                                            y: c_int, width: c_int,
                                            height: c_int)
{
  let dpy: *mut Display = XtDisplay (mw as Widget);
  let mut points: [XPoint; 4] = core::mem::zeroed ();

  points [0].x = x as c_short;
  points [0].y = y as c_short;
  points [1].x = (x + width) as c_short;
  points [1].y = y as c_short;
  points [2].x = (x + width) as c_short;
  points [2].y = (y + height) as c_short;
  points [3].x = x as c_short;
  points [3].y = (y + height) as c_short;
  XFillPolygon (dpy, window,
                (*mw).menu.highlight_background_gc,
                points.as_mut_ptr (), 4, Convex, CoordModeOrigin);

  draw_shadow_rectangle (mw, window, x, y, width, height, False, False,
                         (*mw).menu.highlight_shadow_top_gc,
                         (*mw).menu.highlight_shadow_bottom_gc);
}

/* Display the menu item and increment where.x and where.y to show how large
   the menu item was.  */

/* C 1052-1248: `display_menu_item` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn display_menu_item (mw: XlwMenuWidget,
                                               val: *mut widget_value,
                                               ws: *mut window_state,
                                               r#where: *mut XPoint,
                                               highlighted_p: Boolean,
                                               horizontal_p: Boolean,
                                               just_compute_p: Boolean)
{
  let mut deco_gc: GC;
  let mut text_gc: GC;
  let font_ascent = MENU_FONT_ASCENT! (mw);
  let shadow = (*mw).menu.shadow_thickness as c_int;
  let margin = (*mw).menu.margin as c_int;
  let h_spacing = (*mw).menu.horizontal_spacing as c_int;
  let v_spacing = (*mw).menu.vertical_spacing as c_int;
  let mut label_width: c_int = 0;
  let mut rest_width: c_int = 0;
  let mut button_width: c_int = 0;
  let mut height: c_int = 0;
  let mut width: c_int = 0;
  let mut separator: menu_separator = 0;
  let separator_p = lw_separator_p ((*val).name, &raw mut separator, 0);
  /* C 1075-1077: `#if defined USE_CAIRO || defined HAVE_XFT` */
  #[cfg (feature = "use_cairo_or_have_xft")]
  let mut xftfg: *mut XftColor;
  let mut highlighted_p = highlighted_p;

  /* compute the sizes of the item */
  size_menu_item (mw, val, horizontal_p, &raw mut label_width, &raw mut rest_width,
                  &raw mut button_width, &raw mut height);

  if horizontal_p != 0
    { width = label_width + rest_width; }
  else
    {
      label_width = (*ws).label_width as c_int;
      width = (*ws).width as c_int - 2 * shadow;
    }

  /* Only highlight an enabled item that has a callback. */
  if highlighted_p != 0
    {
      if !(*val).enabled || !(!(*val).call_data.is_null () || !(*val).contents.is_null ())
        { highlighted_p = 0; }
    }

  /* do the drawing. */
  if just_compute_p == 0
    {
      /* Add the shadow border of the containing menu */
      let mut x = (*r#where).x as c_int + shadow;
      let mut y = (*r#where).y as c_int + shadow;

      if horizontal_p != 0
        {
          x += margin;
          y += margin;
        }

      /* pick the foreground and background GC. */
      if (*val).enabled
        {
          if highlighted_p != 0
            { text_gc = (*mw).menu.highlight_foreground_gc; }
          else
            { text_gc = (*mw).menu.foreground_gc; }
        }
      else
        { text_gc = (*mw).menu.disabled_gc; }
      deco_gc = (*mw).menu.foreground_gc;
      /* C 1118-1126: `#if defined USE_CAIRO || defined HAVE_XFT` */
      #[cfg (feature = "use_cairo_or_have_xft")]
      {
        if (*val).enabled
          {
            if highlighted_p != 0
              { xftfg = &raw mut (*mw).menu.xft_highlight_fg; }
            else
              { xftfg = &raw mut (*mw).menu.xft_fg; }
          }
        else
          { xftfg = &raw mut (*mw).menu.xft_disabled_fg; }
      }

      if separator_p != 0
        { draw_separator (mw, (*ws).pixmap, x, y, width, separator); }
      else
        {
          let mut x_offset = x + h_spacing + shadow;
          let display_string = resource_widget_value (mw, val);
          /* Clears shadows and maybe highlight */
          draw_shadow_rectangle (mw, (*ws).pixmap, x, y, width, height,
                                 True, False, core::ptr::null_mut (),
                                 core::ptr::null_mut ());
          if highlighted_p != 0
            { draw_highlight (mw, (*ws).pixmap, x, y, width, height); }

          /* Deal with centering a menu title. */
          if horizontal_p == 0 && (*val).contents.is_null ()
             && (*val).call_data.is_null ()
            {
              let l = string_width (mw, display_string);

              if width > l
                { x_offset = (width - l) >> 1; }
            }
          else if horizontal_p == 0 && (*ws).button_width != 0
            { x_offset += (*ws).button_width as c_int; }


          /* C 1118-1166 is `#if defined USE_CAIRO || defined HAVE_XFT` (this port's
             `feature = "use_cairo_or_have_xft"`) and C 1167-1174 is `#ifdef HAVE_X_I18N`: the
             two gates open the arms of one if/else-if/else chain.  P-86 clause 1 carries both
             arms; the chain's "an earlier arm answered" state is this `painted_p` flag, which is
             what the C chain's `else` spells (`## Deviations`).  */
          let mut painted_p = false;
          #[cfg (feature = "use_cairo_or_have_xft")]
          {
            if !(*ws).xft_draw.is_null ()
              {
                let draw_y = y + v_spacing + shadow;
                /* C 1156-1158: `#ifdef USE_CAIRO` */
                #[cfg (feature = "use_cairo")]
                {
                  cairo_surface_mark_dirty (cairo_get_target ((*ws).xft_draw as cairo_t));
                }
                XftDrawStringUtf8 ((*ws).xft_draw, xftfg,
                                   (*mw).menu.xft_font,
                                   x_offset, draw_y + font_ascent,
                                   display_string as *const c_uchar,
                                   strlen (display_string) as c_int);
                painted_p = true;
              }
          }
          #[cfg (feature = "have_x_i18n")]
          {
            if !painted_p && !(*mw).menu.fontSet.is_null ()
              {
                XmbDrawString (XtDisplay (mw as Widget), (*ws).pixmap, (*mw).menu.fontSet,
                               text_gc, x_offset,
                               y + v_spacing + shadow + font_ascent,
                               display_string, strlen (display_string) as c_int);
                painted_p = true;
              }
          }
          if !painted_p
            {
              XDrawString (XtDisplay (mw as Widget), (*ws).pixmap,
                           text_gc, x_offset,
                           y + v_spacing + shadow + font_ascent,
                           display_string, strlen (display_string) as c_int);
            }

          if horizontal_p == 0
            {
              if (*val).button_type == BUTTON_TYPE_TOGGLE
                { draw_toggle (mw, (*ws).pixmap, x, y + v_spacing + shadow,
                               (*val).selected as c_int, highlighted_p); }
              else if (*val).button_type == BUTTON_TYPE_RADIO
                { draw_radio (mw, (*ws).pixmap, x, y + v_spacing + shadow,
                              (*val).selected as c_int, highlighted_p); }

              if !(*val).contents.is_null ()
                {
                  let a_w = arrow_width (mw);
                  draw_arrow (mw, (*ws).pixmap, deco_gc,
                              x + width - a_w
                              - ((*mw).menu.horizontal_spacing as c_int)
                              - ((*mw).menu.shadow_thickness as c_int),
                              y + v_spacing + shadow, a_w,
                              highlighted_p);
                }
              else if !(*val).key.is_null ()
                {
                  /* C 1201-1214 is the cairo/Xft arm of the key string's painting chain and
                     C 1215-1224 its i18n arm - the same if/else-if/else shape as above.  */
                  let mut key_painted_p = false;
                  #[cfg (feature = "use_cairo_or_have_xft")]
                  {
                    if !(*ws).xft_draw.is_null ()
                      {
                        let draw_x = (*ws).width as c_int - (*ws).max_rest_width
                          + ((*mw).menu.arrow_spacing as c_int);
                        let draw_y = y + v_spacing + shadow + font_ascent;
                        XftDrawStringUtf8 ((*ws).xft_draw, xftfg,
                                           (*mw).menu.xft_font,
                                           draw_x, draw_y,
                                           (*val).key as *const c_uchar,
                                           strlen ((*val).key) as c_int);
                        key_painted_p = true;
                      }
                  }
                  #[cfg (feature = "have_x_i18n")]
                  {
                    if !key_painted_p && !(*mw).menu.fontSet.is_null ()
                      {
                        XmbDrawString (XtDisplay (mw as Widget), (*ws).pixmap,
                                       (*mw).menu.fontSet,
                                       text_gc,
                                       x + label_width + ((*mw).menu.arrow_spacing as c_int),
                                       y + v_spacing + shadow + font_ascent,
                                       (*val).key, strlen ((*val).key) as c_int);
                        key_painted_p = true;
                      }
                  }
                  if !key_painted_p
                    {
                      XDrawString (XtDisplay (mw as Widget), (*ws).pixmap,
                                   text_gc,
                                   x + label_width + ((*mw).menu.arrow_spacing as c_int),
                                   y + v_spacing + shadow + font_ascent,
                                   (*val).key, strlen ((*val).key) as c_int);
                    }
                }
            }
          else
            {
              /* If not highlighted, clears shadows for horizontal
                 menu item */
              if highlighted_p == 0
                { draw_shadow_rectangle (mw, (*ws).pixmap, x, y, width, height,
                                         True, False, core::ptr::null_mut (),
                                         core::ptr::null_mut ()); }
            }
          /* C 1240-1243: `#ifdef USE_CAIRO` */
          #[cfg (feature = "use_cairo")]
          {
            if !(*ws).xft_draw.is_null ()
              { cairo_surface_flush (cairo_get_target ((*ws).xft_draw as cairo_t)); }
          }
        }
    }
  (*r#where).x += width as c_short;
  (*r#where).y += height as c_short;
}

/* C 1250-1345: `display_menu` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn display_menu (mw: XlwMenuWidget,
                                          level: c_int,
                                          just_compute_p: Boolean,
                                          highlighted_pos: *mut XPoint,
                                          hit: *mut XPoint,
                                          hit_return: *mut *mut widget_value)
{
  let mut val: *mut widget_value;
  let following_item: *mut widget_value;
  let ws: *mut window_state;
  let mut r#where: XPoint = core::mem::zeroed ();
  let horizontal_p =
    if (*mw).menu.horizontal != 0 && level == 0 { 1 } else { 0 };
  let mut highlighted_p: c_int;
  let mut no_return: c_int = 0;
  let mut separator: menu_separator = 0;

  if level >= (*mw).menu.old_depth
    { abort_gracefully (mw as Widget); }

  if level < (*mw).menu.old_depth - 1
    { following_item = (*(*mw).menu.old_stack.offset ((level + 1) as isize)); }
  else
    { following_item = core::ptr::null_mut (); }

  if !hit_return.is_null ()
    { *hit_return = core::ptr::null_mut (); }

  r#where.x = 0;
  r#where.y = 0;

  ws = &raw mut (*(*mw).menu.windows.offset (level as isize));

  if just_compute_p == 0
    { XFillRectangle (XtDisplay (mw as Widget), (*ws).pixmap, (*mw).menu.background_gc,
                      0, 0, (*ws).width as c_uint, (*ws).height as c_uint); }

  val = (*(*(*mw).menu.old_stack.offset (level as isize))).contents;
  while !val.is_null ()
    {
      highlighted_p = if val == following_item { 1 } else { 0 };
      if highlighted_p != 0 && !highlighted_pos.is_null ()
        {
          if horizontal_p != 0
            { (*highlighted_pos).x = r#where.x; }
          else
            { (*highlighted_pos).y = r#where.y; }
        }

      display_menu_item (mw, val, ws, &raw mut r#where, highlighted_p, horizontal_p,
                         just_compute_p);

      if highlighted_p != 0 && !highlighted_pos.is_null ()
        {
          if horizontal_p != 0
            { (*highlighted_pos).y = r#where.y; }
          else
            { (*highlighted_pos).x = r#where.x; }
        }

      if !hit.is_null ()
         && (*hit_return).is_null ()
         && (if horizontal_p != 0 { (*hit).x < r#where.x } else { (*hit).y < r#where.y })
         && lw_separator_p ((*val).name, &raw mut separator, 0) == 0
         && no_return == 0
        {
          if (*val).enabled
            { *hit_return = val; }
          else
            { no_return = 1; }
          if (*mw).menu.inside_entry != val
            {
              if !(*mw).menu.inside_entry.is_null ()
                { XtCallCallbackList (mw as Widget, (*mw).menu.leave,
                                      (*mw).menu.inside_entry as XtPointer); }
              (*mw).menu.inside_entry = val;
              XtCallCallbackList (mw as Widget, (*mw).menu.enter,
                                  (*mw).menu.inside_entry as XtPointer);
            }
        }

      if horizontal_p != 0
        { r#where.y = 0; }
      else
        { r#where.x = 0; }
      val = (*val).next;
    }

  if just_compute_p == 0
    {
      draw_shadow_rectangle (mw, (*ws).pixmap, 0, 0, (*ws).width as c_int,
                             (*ws).height as c_int, False, False,
                             core::ptr::null_mut (), core::ptr::null_mut ());
      XCopyArea (XtDisplay (mw as Widget), (*ws).pixmap, (*ws).window,
                 (*mw).menu.foreground_gc, 0, 0, (*ws).width as c_uint,
                 (*ws).height as c_uint, 0, 0);
    }
}

/* Motion code */
/* C 1346-1355: `set_new_state` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn set_new_state (mw: XlwMenuWidget, val: *mut widget_value,
                                          level: c_int)
{
  let mut i: c_int;

  (*mw).menu.new_depth = 0;
  i = 0;
  while i < level
    {
      push_new_stack (mw, (*(*mw).menu.old_stack.offset (i as isize)));
      i += 1;
    }
  push_new_stack (mw, val);
}

/* C 1357-1371: `expose_cb` (static; its address goes into `XtAddEventHandler`).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn expose_cb (widget: Widget,
                                       closure: XtPointer,
                                       _event: *mut XEvent,
                                       continue_to_dispatch: *mut Boolean)
{
  let mw: XlwMenuWidget = closure as XlwMenuWidget;
  let mut i: c_int;

  *continue_to_dispatch = False;
  i = 0;
  loop {
    if i >= (*mw).menu.windows_length
      { break; }
    if (*(*mw).menu.windows.offset (i as isize)).w == widget
      { break; }
    i += 1;
  }
  if i < (*mw).menu.windows_length && i < (*mw).menu.old_depth
    { display_menu (mw, i, False, core::ptr::null_mut (), core::ptr::null_mut (),
                    core::ptr::null_mut ()); }
}

/* C 1373-1387: `set_window_type` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn set_window_type (w: Widget, mw: XlwMenuWidget)
{
  let popup_menu_p = if (*mw).menu.top_depth == 1 { 1 } else { 0 };
  let r#type: Atom = XInternAtom (XtDisplay (w),
                                  if popup_menu_p != 0
                                  { c"_NET_WM_WINDOW_TYPE_POPUP_MENU".as_ptr () }
                                  else { c"_NET_WM_WINDOW_TYPE_DROPDOWN_MENU".as_ptr () },
                                  False);

  XChangeProperty (XtDisplay (w), XtWindow (w),
                   XInternAtom (XtDisplay (w), c"_NET_WM_WINDOW_TYPE".as_ptr (), False),
                   XA_ATOM, 32, PropModeReplace,
                   (&raw const r#type) as *const c_uchar, 1);
}


/* C 1390-1438: `make_windows_if_needed` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn make_windows_if_needed (mw: XlwMenuWidget, n: c_int)
{
  let mut i: c_int;
  let mut start_at: c_int;
  let mut windows: *mut window_state;

  if (*mw).menu.windows_length >= n
    { return; }

  if (*mw).menu.windows.is_null ()
    {
      (*mw).menu.windows =
        XtMalloc ((n as usize * ::core::mem::size_of::<window_state> ()) as Cardinal)
          as *mut window_state;
      start_at = 0;
    }
  else
    {
      (*mw).menu.windows =
        XtRealloc ((*mw).menu.windows as *mut c_char,
                   (n as usize * ::core::mem::size_of::<window_state> ()) as Cardinal)
          as *mut window_state;
      start_at = (*mw).menu.windows_length;
    }
  (*mw).menu.windows_length = n;

  windows = (*mw).menu.windows;

  i = start_at;
  while i < n
   {
     let mut av: [Arg; 10] = core::mem::zeroed ();
     let mut ac: c_int = 0;
     (*windows.offset (i as isize)).x = 0;
     (*windows.offset (i as isize)).y = 0;
     (*windows.offset (i as isize)).width = 1;
     (*windows.offset (i as isize)).height = 1;
     (*windows.offset (i as isize)).max_rest_width = 0;
     XtSetArg! (av[ac as usize], XtNwidth.as_ptr (), 1); ac += 1;
     XtSetArg! (av[ac as usize], XtNheight.as_ptr (), 1); ac += 1;
     XtSetArg! (av[ac as usize], XtNsaveUnder.as_ptr (), True); ac += 1;
     XtSetArg! (av[ac as usize], XtNbackground.as_ptr (), (*mw).core.background_pixel);
     ac += 1;
     XtSetArg! (av[ac as usize], XtNborderColor.as_ptr (), (*mw).core.border_pixel); ac += 1;
     XtSetArg! (av[ac as usize], XtNborderWidth.as_ptr (), (*mw).core.border_width); ac += 1;
     XtSetArg! (av[ac as usize], XtNcursor.as_ptr (), (*mw).menu.cursor_shape); ac += 1;
     (*windows.offset (i as isize)).w =
       XtCreatePopupShell (c"sub".as_ptr (), overrideShellWidgetClass,
                           mw as Widget, av.as_mut_ptr (), ac as Cardinal);
     XtRealizeWidget ((*windows.offset (i as isize)).w);
     XtAddEventHandler ((*windows.offset (i as isize)).w, ExposureMask, False,
                        Some (expose_cb), mw as XtPointer);
     (*windows.offset (i as isize)).window = XtWindow ((*windows.offset (i as isize)).w);
     (*windows.offset (i as isize)).pixmap = None;
     /* C 1440-1442: `#if defined USE_CAIRO || defined HAVE_XFT` */
     #[cfg (feature = "use_cairo_or_have_xft")]
     { (*windows.offset (i as isize)).xft_draw = core::ptr::null_mut (); }
     set_window_type ((*windows.offset (i as isize)).w, mw);
     i += 1;
   }
  XFlush (XtDisplay (mw as Widget));
}

/* Value is non-zero if WINDOW is part of menu bar widget W.  */

/* C 1450-1461: `xlwmenu_window_p` (non-static).  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xlwmenu_window_p (w: Widget, window: Window) -> c_int
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;
  let mut i: c_int;

  i = 0;
  loop {
    if i >= (*mw).menu.windows_length
      { break; }
    if window == (*(*mw).menu.windows.offset (i as isize)).window
      { break; }
    i += 1;
  }

  return (if i < (*mw).menu.windows_length { 1 } else { 0 });
}

/* Make the window fit in the screen */
/* C 1464-1539: `fit_to_screen` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn fit_to_screen (mw: XlwMenuWidget,
                                          ws: *mut window_state,
                                          previous_ws: *mut window_state,
                                          horizontal_p: Boolean)
{
  let mut screen_width: c_int = 0;
  let mut screen_height: c_int = 0;
  let mut screen_x: c_int = 0;
  let mut screen_y: c_int = 0;
  let mut prev_screen_x: c_int = 0;
  let mut prev_screen_y: c_int = 0;

  /* C 1474-1481: `#ifdef emacs` */
  #[cfg (feature = "emacs")]
  {
    xlw_monitor_dimensions_at_pos (XtDisplay (mw as Widget), XtScreen (mw as Widget),
                                   (*previous_ws).x as c_int, (*previous_ws).y as c_int,
                                   &raw mut prev_screen_x, &raw mut prev_screen_y,
                                   &raw mut screen_width, &raw mut screen_height);
    xlw_monitor_dimensions_at_pos (XtDisplay (mw as Widget), XtScreen (mw as Widget),
                                   (*ws).x as c_int, (*ws).y as c_int,
                                   &raw mut screen_x, &raw mut screen_y,
                                   &raw mut screen_width, &raw mut screen_height);
  }
  /* C 1482-1487: `#else` (the standalone lwlib build) */
  #[cfg (not (feature = "emacs"))]
  {
    screen_width = WidthOfScreen (XtScreen (mw as Widget));
    screen_height = HeightOfScreen (XtScreen (mw as Widget));
    prev_screen_x = 0; screen_x = 0;
    prev_screen_y = 0; screen_y = 0;
  }
  /* 1 if we are unable to avoid an overlap between
     this menu and the parent menu in the X dimension.  */
  let mut horizontal_overlap: c_int = 0;

  if ((*ws).x as c_int) < screen_x
    { (*ws).x = screen_x as Position; }
  else if (*ws).x as c_int + (*ws).width as c_int > screen_x + screen_width
    {
      if horizontal_p == 0
        /* The addition of shadow-thickness for a sub-menu's position is
           to reflect a similar adjustment when the menu is displayed to
           the right of the invoking menu-item; it makes the sub-menu
           look more `attached' to the menu-item.  */
        { (*ws).x = (screen_x + ((*previous_ws).x as c_int
                                 - prev_screen_x
                                 - (*ws).width as c_int
                                 + ((*mw).menu.shadow_thickness as c_int))) as Position; }
      else
        { (*ws).x = (screen_x + (screen_width - (*ws).width as c_int)) as Position; }
      if ((*ws).x as c_int) < screen_x
        {
          (*ws).x = screen_x as Position;
          horizontal_overlap = 1;
        }
    }
  /* If we overlap in X, try to avoid overlap in Y.  */
  if horizontal_overlap != 0
     && ((*ws).y as c_int) < ((*previous_ws).y as c_int + (*previous_ws).height as c_int)
     && ((*previous_ws).y as c_int) < ((*ws).y as c_int + (*ws).height as c_int)
    {
      /* Put this menu right below or right above PREVIOUS_WS
         if there's room.  */
      if ((*previous_ws).y as c_int + (*previous_ws).height as c_int
          + (*ws).height as c_int) < screen_height
        { (*ws).y = ((*previous_ws).y as c_int + (*previous_ws).height as c_int) as Position; }
      else if ((*previous_ws).y as c_int - (*ws).height as c_int) > 0
        { (*ws).y = ((*previous_ws).y as c_int - (*ws).height as c_int) as Position; }
    }

  if ((*ws).y as c_int) < screen_y
    { (*ws).y = screen_y as Position; }
  else if (*ws).y as c_int + (*ws).height as c_int > screen_y + screen_height
    {
      if horizontal_p != 0
        { (*ws).y = (screen_y + ((*previous_ws).y as c_int
                                 - prev_screen_y
                                 - (*ws).height as c_int)) as Position; }
      else
        { (*ws).y = (screen_y + (screen_height - (*ws).height as c_int)) as Position; }
      if ((*ws).y as c_int) < screen_y
        { (*ws).y = screen_y as Position; }
    }
}

/* C 1541-1567: `create_pixmap_for_menu` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn create_pixmap_for_menu (ws: *mut window_state, mw: XlwMenuWidget)
{
  if (*ws).pixmap != None
    {
      XFreePixmap (XtDisplay ((*ws).w), (*ws).pixmap);
      (*ws).pixmap = None;
    }
  (*ws).pixmap = XCreatePixmap (XtDisplay ((*ws).w), (*ws).window,
                                (*ws).width as c_uint, (*ws).height as c_uint,
                                DefaultDepthOfScreen (XtScreen ((*ws).w)) as c_uint);
  /* C 1552-1565: `#if defined USE_CAIRO || defined HAVE_XFT` */
  #[cfg (feature = "use_cairo_or_have_xft")]
  {
    if !(*ws).xft_draw.is_null ()
      { XftDrawDestroy ((*ws).xft_draw); }
    if !(*mw).menu.xft_font.is_null ()
      {
        let screen = XScreenNumberOfScreen ((*mw).core.screen);
        (*ws).xft_draw = XftDrawCreate (XtDisplay ((*ws).w),
                                        (*ws).pixmap,
                                        DefaultVisual (XtDisplay ((*ws).w), screen),
                                        (*mw).core.colormap);
      }
    else
      { (*ws).xft_draw = core::ptr::null_mut (); }
  }
}

/* Updates old_stack from new_stack and redisplays. */
/* C 1569-1666: `remap_menubar` (static).  C 1635-1638's `#if 0` block (the
   `if (!mw->menu.horizontal || i > 1) ws->x += mw->menu.shadow_thickness;` pair) is dead text in
   every configuration and is recorded in the ledger instead of transcribed.  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn remap_menubar (mw: XlwMenuWidget)
{
  let mut i: c_int;
  let mut last_same: c_int;
  let mut selection_position: XPoint = core::mem::zeroed ();
  let old_depth = (*mw).menu.old_depth;
  let new_depth = (*mw).menu.new_depth;
  let old_stack: *mut *mut widget_value;
  let new_stack: *mut *mut widget_value;
  let windows: *mut window_state;
  let mut old_selection: *mut widget_value;
  let mut new_selection: *mut widget_value;

  /* Check that enough windows and old_stack are ready. */
  make_windows_if_needed (mw, new_depth);
  make_old_stack_space (mw, new_depth);
  windows = (*mw).menu.windows;
  old_stack = (*mw).menu.old_stack;
  new_stack = (*mw).menu.new_stack;

  /* compute the last identical different entry */
  i = 1;
  while i < old_depth && i < new_depth
    {
      if *old_stack.offset (i as isize) != *new_stack.offset (i as isize)
        { break; }
      i += 1;
    }
  last_same = i - 1;

  /* Memorize the previously selected item to be able to refresh it */
  old_selection = if last_same + 1 < old_depth
                  { *old_stack.offset ((last_same + 1) as isize) }
                  else { core::ptr::null_mut () };
  if !old_selection.is_null () && !(*old_selection).enabled
    { old_selection = core::ptr::null_mut (); }
  new_selection = if last_same + 1 < new_depth
                  { *new_stack.offset ((last_same + 1) as isize) }
                  else { core::ptr::null_mut () };
  if !new_selection.is_null () && !(*new_selection).enabled
    { new_selection = core::ptr::null_mut (); }

  /* Call callback when the highlighted item changes.  */
  if !old_selection.is_null () || !new_selection.is_null ()
    { XtCallCallbackList (mw as Widget, (*mw).menu.highlight,
                          new_selection as XtPointer); }

  /* updates old_state from new_state.  It has to be done now because
     display_menu (called below) uses the old_stack to know what to display. */
  i = last_same + 1;
  while i < new_depth
    {
      XtPopdown ((*(*mw).menu.windows.offset (i as isize)).w);
      *old_stack.offset (i as isize) = *new_stack.offset (i as isize);
      i += 1;
    }
  (*mw).menu.old_depth = new_depth;

  /* refresh the last selection */
  selection_position.x = 0;
  selection_position.y = 0;
  display_menu (mw, last_same, if new_selection == old_selection { True } else { False },
                &raw mut selection_position, core::ptr::null_mut (),
                core::ptr::null_mut ());

  /* Now place the new menus.  */
  i = last_same + 1;
  while i < new_depth && !(*new_stack.offset (i as isize)).is_null ()
        && !(*(*new_stack.offset (i as isize))).contents.is_null ()
    {
      let previous_ws: *mut window_state = &raw mut (*windows.offset ((i - 1) as isize));
      let ws: *mut window_state = &raw mut (*windows.offset (i as isize));

      (*ws).x = ((*previous_ws).x as c_int + selection_position.x as c_int
                 + ((*mw).menu.shadow_thickness as c_int)) as Position;
      if (*mw).menu.horizontal != 0 && i == 1
        { (*ws).x = ((*ws).x as c_int + (*mw).menu.margin as c_int) as Position; }

      (*ws).y = ((*previous_ws).y as c_int + selection_position.y as c_int
                 + ((*mw).menu.shadow_thickness as c_int)) as Position;
      if (*mw).menu.horizontal != 0 && i == 1
        { (*ws).y = ((*ws).y as c_int + (*mw).menu.margin as c_int) as Position; }

      /* WMs like Gnome 3 ignores requests to move windows.  So we
         must destroy the current one and create a new to get it to move.  */
      XtUnrealizeWidget ((*ws).w);
      XtRealizeWidget ((*ws).w);
      (*ws).window = XtWindow ((*ws).w);

      size_menu (mw, i);

      fit_to_screen (mw, ws, previous_ws,
                     if (*mw).menu.horizontal != 0 && i == 1 { True } else { False });

      create_pixmap_for_menu (ws, mw);
      XtConfigureWidget ((*ws).w, (*ws).x, (*ws).y, (*ws).width, (*ws).height,
                         (*(*ws).w).core.border_width);
      display_menu (mw, i, False, &raw mut selection_position, core::ptr::null_mut (),
                    core::ptr::null_mut ());
      XtPopup ((*ws).w, XtGrabNone);
      i += 1;
    }

  /* unmap the menus that popped down */
  i = new_depth - 1;
  while i < old_depth
    {
      if i >= new_depth || (i > 0 && (*(*new_stack.offset (i as isize))).contents.is_null ())
        { XtPopdown ((*windows.offset (i as isize)).w); }
      i += 1;
    }
}

/* C 1668-1682: `motion_event_is_in_menu` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn motion_event_is_in_menu (mw: XlwMenuWidget,
                                                    ev: *mut XMotionEvent,
                                                    level: c_int,
                                                    relative_pos: *mut XPoint) -> Boolean
{
  let ws: *mut window_state = &raw mut (*(*mw).menu.windows.offset (level as isize));
  let shadow = if level == 0 { 0 } else { (*mw).menu.shadow_thickness as c_int };
  let x = (*ws).x as c_int + shadow;
  let y = (*ws).y as c_int + shadow;
  (*relative_pos).x = ((*ev).x_root - x) as c_short;
  (*relative_pos).y = ((*ev).y_root - y) as c_short;
  return (x - shadow < (*ev).x_root && (*ev).x_root < x + (*ws).width as c_int
          && y - shadow < (*ev).y_root && (*ev).y_root < y + (*ws).height as c_int)
         as Boolean;
}

/* C 1684-1724: `map_event_to_widget_value` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn map_event_to_widget_value (mw: XlwMenuWidget,
                                                       ev: *mut XMotionEvent,
                                                       val: *mut *mut widget_value,
                                                       level: *mut c_int) -> Boolean
{
  let mut i: c_int;
  let mut relative_pos: XPoint = core::mem::zeroed ();
  let mut ws: *mut window_state;
  let mut inside: c_int = 0;

  *val = core::ptr::null_mut ();

  /* Find the window */
  i = (*mw).menu.old_depth - 1;
  while i >= 0
    {
      ws = &raw mut (*(*mw).menu.windows.offset (i as isize));
      if !ws.is_null ()
         && motion_event_is_in_menu (mw, ev, i, &raw mut relative_pos) != 0
        {
          inside = 1;
          display_menu (mw, i, True, core::ptr::null_mut (), &raw mut relative_pos, val);

          if !(*val).is_null ()
            {
              *level = i + 1;
              return True;
            }
        }
      i -= 1;
    }

  if inside == 0
    {
      if !(*mw).menu.inside_entry.is_null ()
        { XtCallCallbackList (mw as Widget, (*mw).menu.leave,
                              (*mw).menu.inside_entry as XtPointer); }
      (*mw).menu.inside_entry = core::ptr::null_mut ();
    }

  return False;
}


/* C 1726-1810: `make_drawing_gcs` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn make_drawing_gcs (mw: XlwMenuWidget)
{
  let mut xgcv: XGCValues = core::mem::zeroed ();
  let mut scale: f32;
  let mut mask: XtGCMask = GCForeground | GCBackground;

  /* C 1733-1745: `#ifdef HAVE_X_I18N` */
  #[cfg (feature = "have_x_i18n")]
  {
    if (*mw).menu.fontSet.is_null () && !(*mw).menu.font.is_null ()
      {
        xgcv.font = (*((*mw).menu.font)).fid;
        mask |= GCFont;
      }
  }
  /* C 1739-1745: the `#else` arm */
  #[cfg (not (feature = "have_x_i18n"))]
  {
    if !(*mw).menu.font.is_null ()
      {
        xgcv.font = (*((*mw).menu.font)).fid;
        mask |= GCFont;
      }
  }
  xgcv.foreground = (*mw).menu.foreground;
  xgcv.background = (*mw).core.background_pixel;
  (*mw).menu.foreground_gc = XtGetGC (mw as Widget, mask, &raw mut xgcv);

  xgcv.foreground = (*mw).menu.button_foreground;
  (*mw).menu.button_gc = XtGetGC (mw as Widget, mask, &raw mut xgcv);

  xgcv.background = (*mw).core.background_pixel;

  /* C 1755: `#define BRIGHTNESS(color) (((color) & 0xff) + (((color) >> 8) & 0xff)
     + (((color) >> 16) & 0xff))` - defined here, at its C position, and used below.  */
  macro_rules! BRIGHTNESS {
    ($color:expr) => {
      (((($color) & 0xff) + ((($color) >> 8) & 0xff) + ((($color) >> 16) & 0xff)) as c_int)
    };
  }

  /* Allocate color for disabled menu-items.  */
  if BRIGHTNESS! ((*mw).menu.foreground) < BRIGHTNESS! ((*mw).core.background_pixel)
    { scale = 2.3; }
  else
    { scale = 0.55; }

  x_alloc_lighter_color_for_widget (mw as Widget, XtDisplay (mw as Widget),
                                    (*mw).core.colormap,
                                    &raw mut (*mw).menu.disabled_foreground,
                                    scale as f64,
                                    0x8000);

  if (*mw).menu.foreground == (*mw).menu.disabled_foreground
      || (*mw).core.background_pixel == (*mw).menu.disabled_foreground
    {
      /* Too few colors, use stipple.  */
      xgcv.foreground = (*mw).menu.foreground;
      xgcv.fill_style = FillStippled;
      xgcv.stipple = (*mw).menu.gray_pixmap;
      (*mw).menu.disabled_gc = XtGetGC (mw as Widget, mask
                                        | GCFillStyle | GCStipple, &raw mut xgcv);
    }
  else
    {
      /* Many colors available, use disabled pixel.  */
      xgcv.foreground = (*mw).menu.disabled_foreground;
      (*mw).menu.disabled_gc = XtGetGC (mw as Widget, mask, &raw mut xgcv);
    }

  xgcv.foreground = (*mw).menu.button_foreground;
  xgcv.background = (*mw).core.background_pixel;
  xgcv.fill_style = FillStippled;
  xgcv.stipple = (*mw).menu.gray_pixmap;
  (*mw).menu.inactive_button_gc = XtGetGC (mw as Widget, mask
                                           | GCFillStyle | GCStipple, &raw mut xgcv);

  xgcv.foreground = (*mw).core.background_pixel;
  xgcv.background = (*mw).menu.foreground;
  (*mw).menu.background_gc = XtGetGC (mw as Widget, mask, &raw mut xgcv);

  xgcv.foreground = if (*mw).menu.highlight_foreground == (-1isize) as Pixel
                    { (*mw).menu.foreground }
                    else { (*mw).menu.highlight_foreground };
  xgcv.background = if (*mw).menu.highlight_background == (-1isize) as Pixel
                    { (*mw).core.background_pixel }
                    else { (*mw).menu.highlight_background };
  (*mw).menu.highlight_foreground_gc = XtGetGC (mw as Widget, mask, &raw mut xgcv);

  xgcv.foreground = if (*mw).menu.highlight_background == (-1isize) as Pixel
                    { (*mw).core.background_pixel }
                    else { (*mw).menu.highlight_background };
  xgcv.background = (*mw).menu.foreground;
  (*mw).menu.highlight_background_gc = XtGetGC (mw as Widget, mask, &raw mut xgcv);
}

/* C 1812-1830: `release_drawing_gcs` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn release_drawing_gcs (mw: XlwMenuWidget)
{
  XtReleaseGC (mw as Widget, (*mw).menu.foreground_gc);
  XtReleaseGC (mw as Widget, (*mw).menu.button_gc);
  XtReleaseGC (mw as Widget, (*mw).menu.disabled_gc);
  XtReleaseGC (mw as Widget, (*mw).menu.inactive_button_gc);
  XtReleaseGC (mw as Widget, (*mw).menu.background_gc);
  XtReleaseGC (mw as Widget, (*mw).menu.highlight_foreground_gc);
  XtReleaseGC (mw as Widget, (*mw).menu.highlight_background_gc);
  /* let's get some segvs if we try to use these... */
  (*mw).menu.foreground_gc = (-1isize) as GC;
  (*mw).menu.button_gc = (-1isize) as GC;
  (*mw).menu.disabled_gc = (-1isize) as GC;
  (*mw).menu.inactive_button_gc = (-1isize) as GC;
  (*mw).menu.background_gc = (-1isize) as GC;
  (*mw).menu.highlight_foreground_gc = (-1isize) as GC;
  (*mw).menu.highlight_background_gc = (-1isize) as GC;
}

/* C 1832-1835: `#ifndef emacs`'s `MINL` - the standalone build's clipping macro (the
   `feature = "emacs"` arm has no `MINL` at all, because C 1861-1872's arm does not use it).  */
#[cfg (all (feature = "use_lucid", not (feature = "emacs")))]
macro_rules! MINL {
  ($x:expr, $y:expr) => {
    (if (($x) as c_ulong) < (($y) as c_ulong) { (($x) as c_ulong) } else { (($y) as c_ulong) })
  };
}

/* C 1837-1945: `compute_shadow_colors` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn compute_shadow_colors (mw: XlwMenuWidget, top_color: *mut Pixel,
                                                   bottom_color: *mut Pixel,
                                                   free_top_p: *mut Boolean,
                                                   free_bottom_p: *mut Boolean,
                                                   top_pixmap: *mut Pixmap,
                                                   bottom_pixmap: *mut Pixmap,
                                                   fore_color: Pixel, back_color: Pixel)
{
  let dpy: *mut Display = XtDisplay (mw as Widget);
  let screen: *mut Screen = XtScreen (mw as Widget);
  let cmap: Colormap = (*mw).core.colormap;
  let mut topc: XColor = core::mem::zeroed ();
  let mut botc: XColor = core::mem::zeroed ();
  let mut top_frobbed: c_int = 0;
  let mut bottom_frobbed: c_int = 0;

  *free_top_p = False;
  *free_bottom_p = False;

  if *top_color == (-1isize) as Pixel
    { *top_color = back_color; }

  if *bottom_color == (-1isize) as Pixel
    { *bottom_color = fore_color; }

  if *top_color == back_color || *top_color == fore_color
    {
      topc.pixel = back_color;
      /* C 1861-1872: `#ifdef emacs` and its `#else` arm are one `if` whose condition and body
         the two arms spell differently; the port keeps each arm behind its own gate and the
         `#ifdef emacs` arm's `if (...)  { ... }` shape.  */
      #[cfg (feature = "emacs")]
      let frobbed_ok = if x_alloc_lighter_color_for_widget (mw as Widget, dpy, cmap,
                                                            &raw mut topc.pixel,
                                                            1.2, 0x8000)
                       { 1 } else { 0 };
      #[cfg (not (feature = "emacs"))]
      let frobbed_ok = {
        XQueryColor (dpy, cmap, &raw mut topc);
        /* Don't overflow/wrap!  */
        topc.red = MINL! (65535, (topc.red as f64 * 1.2)) as c_ushort;
        topc.green = MINL! (65535, (topc.green as f64 * 1.2)) as c_ushort;
        topc.blue = MINL! (65535, (topc.blue as f64 * 1.2)) as c_ushort;
        if XAllocColor (dpy, cmap, &raw mut topc) != 0 { 1 } else { 0 }
      };
      if frobbed_ok != 0
        {
          *top_color = topc.pixel;
          *free_top_p = True;
          top_frobbed = 1;
        }
    }
  if *bottom_color == fore_color || *bottom_color == back_color
    {
      botc.pixel = back_color;
      #[cfg (feature = "emacs")]
      let frobbed_ok = if x_alloc_lighter_color_for_widget (mw as Widget, dpy, cmap,
                                                            &raw mut botc.pixel,
                                                            0.6, 0x4000)
                       { 1 } else { 0 };
      #[cfg (not (feature = "emacs"))]
      let frobbed_ok = {
        XQueryColor (dpy, cmap, &raw mut botc);
        botc.red = (botc.red as f64 * 0.6) as c_ushort;
        botc.green = (botc.green as f64 * 0.6) as c_ushort;
        botc.blue = (botc.blue as f64 * 0.6) as c_ushort;
        if XAllocColor (dpy, cmap, &raw mut botc) != 0 { 1 } else { 0 }
      };
      if frobbed_ok != 0
        {
          *bottom_color = botc.pixel;
          *free_bottom_p = True;
          bottom_frobbed = 1;
        }
    }

  if top_frobbed != 0 && bottom_frobbed != 0
    {
      if topc.pixel == botc.pixel
        {
          if botc.pixel == fore_color
            {
              if *free_top_p != 0
                {
                  x_free_dpy_colors (dpy, screen, cmap, top_color, 1);
                  *free_top_p = False;
                }
              *top_color = back_color;
            }
          else
            {
              if *free_bottom_p != 0
                {
                  x_free_dpy_colors (dpy, screen, cmap, bottom_color, 1);
                  *free_bottom_p = False;
                }
              *bottom_color = fore_color;
            }
        }
    }

  if *top_pixmap == None
    {
      if *top_color == back_color
        {
          *top_pixmap = (*mw).menu.gray_pixmap;
          if *free_top_p != 0
            {
              x_free_dpy_colors (dpy, screen, cmap, top_color, 1);
              *free_top_p = False;
            }
          *top_color = fore_color;
        }
    }
  if *bottom_pixmap == None
    {
      if *bottom_color == back_color
        {
          *bottom_pixmap = (*mw).menu.gray_pixmap;
          if *free_bottom_p != 0
            {
              x_free_dpy_colors (dpy, screen, cmap, bottom_color, 1);
              *free_bottom_p = False;
            }
          *bottom_color = fore_color;
        }
    }
}

/* C 1947-1997: `make_shadow_gcs` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn make_shadow_gcs (mw: XlwMenuWidget)
{
  let mut xgcv: XGCValues = core::mem::zeroed ();
  let mut pm: c_ulong = 0;
  let mut highlight_fg: Pixel;

  highlight_fg = (*mw).menu.highlight_foreground;

  if highlight_fg == (-1isize) as Pixel
    { highlight_fg = (*mw).menu.foreground; }

  /* Normal shadows */
  compute_shadow_colors (mw, &raw mut (*mw).menu.top_shadow_color,
                         &raw mut (*mw).menu.bottom_shadow_color,
                         &raw mut (*mw).menu.free_top_shadow_color_p,
                         &raw mut (*mw).menu.free_bottom_shadow_color_p,
                         &raw mut (*mw).menu.top_shadow_pixmap,
                         &raw mut (*mw).menu.bottom_shadow_pixmap,
                         (*mw).menu.foreground, (*mw).core.background_pixel);

  /* Highlight shadows */
  compute_shadow_colors (mw, &raw mut (*mw).menu.top_highlight_shadow_color,
                         &raw mut (*mw).menu.bottom_highlight_shadow_color,
                         &raw mut (*mw).menu.free_top_highlight_shadow_color_p,
                         &raw mut (*mw).menu.free_bottom_highlight_shadow_color_p,
                         &raw mut (*mw).menu.top_highlight_shadow_pixmap,
                         &raw mut (*mw).menu.bottom_highlight_shadow_pixmap,
                         highlight_fg, (*mw).menu.highlight_background);

  xgcv.fill_style = FillStippled;
  xgcv.foreground = (*mw).menu.top_shadow_color;
  xgcv.stipple = (*mw).menu.top_shadow_pixmap;
  pm = if xgcv.stipple != 0 { GCStipple | GCFillStyle } else { 0 };
  (*mw).menu.shadow_top_gc = XtGetGC (mw as Widget, GCForeground | pm, &raw mut xgcv);

  xgcv.foreground = (*mw).menu.bottom_shadow_color;
  xgcv.stipple = (*mw).menu.bottom_shadow_pixmap;
  pm = if xgcv.stipple != 0 { GCStipple | GCFillStyle } else { 0 };
  (*mw).menu.shadow_bottom_gc = XtGetGC (mw as Widget, GCForeground | pm, &raw mut xgcv);

  xgcv.foreground = (*mw).menu.top_highlight_shadow_color;
  xgcv.stipple = (*mw).menu.top_highlight_shadow_pixmap;
  pm = if xgcv.stipple != 0 { GCStipple | GCFillStyle } else { 0 };
  (*mw).menu.highlight_shadow_top_gc = XtGetGC (mw as Widget, GCForeground | pm,
                                                &raw mut xgcv);

  xgcv.foreground = (*mw).menu.bottom_highlight_shadow_color;
  xgcv.stipple = (*mw).menu.bottom_highlight_shadow_pixmap;
  pm = if xgcv.stipple != 0 { GCStipple | GCFillStyle } else { 0 };
  (*mw).menu.highlight_shadow_bottom_gc = XtGetGC (mw as Widget, GCForeground | pm,
                                                   &raw mut xgcv);
}

/* C 1999-2023: `release_shadow_gcs` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn release_shadow_gcs (mw: XlwMenuWidget)
{
  let dpy: *mut Display = XtDisplay (mw as Widget);
  let screen: *mut Screen = XtScreen (mw as Widget);
  let cmap: Colormap = (*mw).core.colormap;
  let mut px: [Pixel; 4] = core::mem::zeroed ();
  let mut i: c_int = 0;

  if (*mw).menu.free_top_shadow_color_p != 0
    { px[i as usize] = (*mw).menu.top_shadow_color; i += 1; }
  if (*mw).menu.free_bottom_shadow_color_p != 0
    { px[i as usize] = (*mw).menu.bottom_shadow_color; i += 1; }
  if (*mw).menu.free_top_highlight_shadow_color_p != 0
    { px[i as usize] = (*mw).menu.top_highlight_shadow_color; i += 1; }
  if (*mw).menu.free_bottom_highlight_shadow_color_p != 0
    { px[i as usize] = (*mw).menu.bottom_highlight_shadow_color; i += 1; }
  if i > 0
    { x_free_dpy_colors (dpy, screen, cmap, px.as_mut_ptr (), i); }

  XtReleaseGC (mw as Widget, (*mw).menu.shadow_top_gc);
  XtReleaseGC (mw as Widget, (*mw).menu.shadow_bottom_gc);
  XtReleaseGC (mw as Widget, (*mw).menu.highlight_shadow_top_gc);
  XtReleaseGC (mw as Widget, (*mw).menu.highlight_shadow_bottom_gc);
}

/* C 2025-2104: the cairo/Xft block - `getDefaultXftFont`, `openXftFont` and `update_xft_colors`,
   all three `#if defined USE_CAIRO || defined HAVE_XFT`.  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe extern "C" fn getDefaultXftFont (mw: XlwMenuWidget) -> *mut XftFont
{
  let screen = XScreenNumberOfScreen ((*mw).core.screen);
  return XftFontOpenName (XtDisplay (mw as Widget), screen, DEFAULT_FONTNAME! ());
}

#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe extern "C" fn openXftFont (mw: XlwMenuWidget) -> c_int
{
  let fname_in: *const c_char = (*mw).menu.fontName;
  let mut fname: *const c_char = fname_in;
  let mut fname_minus: *mut c_char = core::ptr::null_mut ();

  (*mw).menu.xft_font = core::ptr::null_mut ();
  (*mw).menu.default_face =
    (if !fname.is_null () && strcmp (fname, DEFAULT_FONTNAME! ()) == 0
     { 1 } else { 0 });

  if !fname.is_null () && strcmp (fname, c"none".as_ptr ()) != 0
    {
      let screen = XScreenNumberOfScreen ((*mw).core.screen);
      let len = strlen (fname);
      let mut i = len - 1;
      /* Try to convert Gtk-syntax (Sans 9) to Xft syntax Sans-9.  */
      while i > 0 && b'0' as c_char <= *fname.offset (i as isize)
            && *fname.offset (i as isize) <= b'9' as c_char
        { i -= 1; }
      if *fname.offset (i as isize) == b' ' as c_char
        {
          fname_minus = xstrdup ((*mw).menu.fontName);
          fname = fname_minus;
          *fname_minus.offset (i as isize) = b'-' as c_char;
        }

      (*mw).menu.xft_font = XftFontOpenName (XtDisplay (mw as Widget), screen, fname);
      if (*mw).menu.xft_font.is_null ()
        { (*mw).menu.xft_font = getDefaultXftFont (mw); }
    }

  xfree (fname_minus as *mut c_void);

  return (if !(*mw).menu.xft_font.is_null () { 1 } else { 0 });
}

#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe extern "C" fn update_xft_colors (w: Widget)
{
  let mw: XlwMenuWidget;
  let mut colors: [XColor; 4] = core::mem::zeroed ();

  mw = w as XlwMenuWidget;

  colors[0].pixel = (*mw).menu.foreground;
  (*mw).menu.xft_fg.pixel = (*mw).menu.foreground;
  colors[1].pixel = (*mw).core.background_pixel;
  (*mw).menu.xft_bg.pixel = (*mw).core.background_pixel;
  colors[2].pixel = (*mw).menu.disabled_foreground;
  (*mw).menu.xft_disabled_fg.pixel = (*mw).menu.disabled_foreground;
  colors[3].pixel = if (*mw).menu.highlight_foreground != (-1isize) as Pixel
                    { (*mw).menu.highlight_foreground }
                    else { (*mw).menu.foreground };
  (*mw).menu.xft_highlight_fg.pixel = colors[3].pixel;

  XQueryColors (XtDisplay (mw as Widget), (*mw).core.colormap,
                colors.as_mut_ptr (), 4);

  (*mw).menu.xft_fg.color.alpha = 0xFFFF;
  (*mw).menu.xft_fg.color.red = colors[0].red;
  (*mw).menu.xft_fg.color.green = colors[0].green;
  (*mw).menu.xft_fg.color.blue = colors[0].blue;
  (*mw).menu.xft_bg.color.alpha = 0xFFFF;
  (*mw).menu.xft_bg.color.red = colors[1].red;
  (*mw).menu.xft_bg.color.green = colors[1].green;
  (*mw).menu.xft_bg.color.blue = colors[1].blue;
  (*mw).menu.xft_disabled_fg.color.alpha = 0xFFFF;
  (*mw).menu.xft_disabled_fg.color.red = colors[2].red;
  (*mw).menu.xft_disabled_fg.color.green = colors[2].green;
  (*mw).menu.xft_disabled_fg.color.blue = colors[2].blue;
  (*mw).menu.xft_highlight_fg.color.alpha = 0xFFFF;
  (*mw).menu.xft_highlight_fg.color.red = colors[3].red;
  (*mw).menu.xft_highlight_fg.color.green = colors[3].green;
  (*mw).menu.xft_highlight_fg.color.blue = colors[3].blue;
}

/* C 2106-2180: `XlwMenuInitialize` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn XlwMenuInitialize (_request: Widget, w: Widget,
                                              _args: ArgList, _num_args: *mut Cardinal)
{
  /* Get the GCs and the widget size.  */
  let mw: XlwMenuWidget = w as XlwMenuWidget;
  let window: Window = RootWindowOfScreen (DefaultScreenOfDisplay (XtDisplay (mw as Widget)));
  let display: *mut Display = XtDisplay (mw as Widget);

  /*  mw->menu.cursor = XCreateFontCursor (display, mw->menu.cursor_shape); */
  (*mw).menu.cursor = (*mw).menu.cursor_shape;

  (*mw).menu.gray_pixmap
    = XCreatePixmapFromBitmapData (display, window, gray_bits.as_mut_ptr (),
                                   gray_width as c_uint, gray_height as c_uint,
                                   1 as c_ulong, 0 as c_ulong, 1);

  /* C 2122-2139: `#if defined USE_CAIRO || defined HAVE_XFT`, whose `if (openXftFont (mw))
     mw->menu.font = NULL; else` is the chain's first arm and C 2127-2139's braced block the
     second.  Both arms are carried; the chain's state is the `xft_opened_p` flag.  */
  #[cfg (feature = "use_cairo_or_have_xft")]
  let xft_opened_p = if openXftFont (mw) != 0
                     { (*mw).menu.font = core::ptr::null_mut (); true }
                     else { false };
  #[cfg (not (feature = "use_cairo_or_have_xft"))]
  let xft_opened_p = false;
  if !xft_opened_p
    {
      (*mw).menu.font = XLoadQueryFont (display, (*mw).menu.fontName);
      if (*mw).menu.font.is_null ()
        {
          (*mw).menu.font = XLoadQueryFont (display, c"fixed".as_ptr ());
          if (*mw).menu.font.is_null ()
            {
              fprintf (stderr, c"Menu font fixed not found, can't continue.\n".as_ptr ());
              emacs_abort ();
            }
        }
    }

  /* C 2140-2143: `#ifdef HAVE_X_I18N` */
  #[cfg (feature = "have_x_i18n")]
  {
    if !(*mw).menu.fontSet.is_null ()
      { (*mw).menu.font_extents = XExtentsOfFontSet ((*mw).menu.fontSet); }
  }

  (*mw).menu.top_highlight_shadow_color = (-1isize) as Pixel;
  (*mw).menu.bottom_highlight_shadow_color = (-1isize) as Pixel;
  (*mw).menu.top_highlight_shadow_pixmap = None;
  (*mw).menu.bottom_highlight_shadow_pixmap = None;

  make_drawing_gcs (mw);
  make_shadow_gcs (mw);

  (*mw).menu.popped_up = False;

  (*mw).menu.old_depth = 1;
  (*mw).menu.old_stack =
    XtMalloc (::core::mem::size_of::<*mut widget_value> () as Cardinal)
      as *mut *mut widget_value;
  (*mw).menu.old_stack_length = 1;
  (*(*mw).menu.old_stack.offset (0)) = (*mw).menu.contents;

  (*mw).menu.new_depth = 0;
  (*mw).menu.new_stack = core::ptr::null_mut ();
  (*mw).menu.new_stack_length = 0;
  push_new_stack (mw, (*mw).menu.contents);

  (*mw).menu.windows =
    XtMalloc (::core::mem::size_of::<window_state> () as Cardinal) as *mut window_state;
  (*mw).menu.windows_length = 1;
  (*(*mw).menu.windows.offset (0)).x = 0;
  (*(*mw).menu.windows.offset (0)).y = 0;
  (*(*mw).menu.windows.offset (0)).width = 0;
  (*(*mw).menu.windows.offset (0)).height = 0;
  (*(*mw).menu.windows.offset (0)).max_rest_width = 0;
  (*(*mw).menu.windows.offset (0)).pixmap = None;
  /* C 2173-2175: `#if defined USE_CAIRO || defined HAVE_XFT` */
  #[cfg (feature = "use_cairo_or_have_xft")]
  { (*(*mw).menu.windows.offset (0)).xft_draw = core::ptr::null_mut (); }
  size_menu (mw, 0);

  (*mw).core.width = (*(*mw).menu.windows.offset (0)).width;
  (*mw).core.height = (*(*mw).menu.windows.offset (0)).height;
}

/* C 2182-2185: `XlwMenuClassInitialize` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn XlwMenuClassInitialize ()
{
}

/* C 2187-2228: `XlwMenuRealize` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn XlwMenuRealize (w: Widget, valueMask: *mut Mask,
                                            attributes: *mut XSetWindowAttributes)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;
  let mut xswa: XSetWindowAttributes = core::mem::zeroed ();
  let mut mask: c_int;

  ((*(*xlwMenuWidgetClass).core_class.superclass).core_class.realize.unwrap ())
    (w, valueMask, attributes);

  xswa.save_under = True;
  xswa.cursor = (*mw).menu.cursor_shape;
  mask = (CWSaveUnder | CWCursor) as c_int;
  /* I sometimes get random BadCursor errors while creating the first
     frame on a display.  I can not find their reason, but they are
     annoying so for now let's ignore any errors here.  -- lorentey  */
  /* C 2203-2205: `#ifdef emacs` */
  #[cfg (feature = "emacs")]
  { x_catch_errors (XtDisplay (w)); }
  XChangeWindowAttributes (XtDisplay (w), XtWindow (w), mask as c_ulong, &raw mut xswa);
  /* C 2207-2209: `#ifdef emacs` */
  #[cfg (feature = "emacs")]
  { x_uncatch_errors (); }

  (*(*mw).menu.windows.offset (0)).w = w;
  (*(*mw).menu.windows.offset (0)).window = XtWindow (w);
  (*(*mw).menu.windows.offset (0)).x = (*w).core.x;
  (*(*mw).menu.windows.offset (0)).y = (*w).core.y;
  (*(*mw).menu.windows.offset (0)).width = (*w).core.width;
  (*(*mw).menu.windows.offset (0)).height = (*w).core.height;

  set_window_type ((*(*mw).menu.windows.offset (0)).w, mw);
  create_pixmap_for_menu (&raw mut (*(*mw).menu.windows.offset (0)), mw);

  /* C 2221-2224: `#if defined USE_CAIRO || defined HAVE_XFT` */
  #[cfg (feature = "use_cairo_or_have_xft")]
  {
    if !(*mw).menu.xft_font.is_null ()
      { update_xft_colors (w); }
  }
}

/* Only the toplevel menubar/popup is a widget so it's the only one that
   receives expose events through Xt.  So we repaint all the other panes
   when receiving an Expose event.  */
/* C 2230-2248: `XlwMenuRedisplay` (static; its address goes into `CoreClassPart.expose`).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn XlwMenuRedisplay (w: Widget, _ev: *mut XEvent, _region: Region)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;

  /* If we have a depth beyond 1, it's because a submenu was displayed.
     If the submenu has been destroyed, set the depth back to 1.  */
  if submenu_destroyed != 0
    {
      (*mw).menu.old_depth = 1;
      submenu_destroyed = 0;
    }

  display_menu (mw, 0, False, core::ptr::null_mut (), core::ptr::null_mut (),
                core::ptr::null_mut ());
}


/* Part of a hack to make the menu redisplay when a tooltip frame
   over a menu item is unmapped.  */

/* C 2250-2254: `xlwmenu_redisplay` (non-static).  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xlwmenu_redisplay (w: Widget)
{
  XlwMenuRedisplay (w, core::ptr::null_mut (), None as Region);
}

/* C 2256-2323: `XlwMenuDestroy` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn XlwMenuDestroy (w: Widget)
{
  let mut i: c_int;
  let mw: XlwMenuWidget = w as XlwMenuWidget;

  if pointer_grabbed != 0
    { ungrab_all (w, CurrentTime); }
  pointer_grabbed = 0;
  keyboard_grabbed = 0;

  if XtIsShell (XtParent (w)) == 0
    { submenu_destroyed = 1; }

  release_drawing_gcs (mw);
  release_shadow_gcs (mw);

  /* This doesn't come from the resource db but is created explicitly
     so we must free it ourselves.  */
  XFreePixmap (XtDisplay (mw as Widget), (*mw).menu.gray_pixmap);
  (*mw).menu.gray_pixmap = (-1isize) as Pixmap;

  /* Don't free mw->menu.contents because that comes from our creator.
     The `*_stack' elements are just pointers into `contents' so leave
     that alone too.  But free the stacks themselves.  */
  if !(*mw).menu.old_stack.is_null ()
    { XtFree ((*mw).menu.old_stack as *mut c_char); }
  if !(*mw).menu.new_stack.is_null ()
    { XtFree ((*mw).menu.new_stack as *mut c_char); }

  /* Original comment was:

     Remember, you can't free anything that came from the resource
     database.  This includes:
         mw->menu.cursor
         mw->menu.top_shadow_pixmap
         mw->menu.bottom_shadow_pixmap
         mw->menu.font
     Also the color cells of top_shadow_color, bottom_shadow_color,
     foreground, and button_foreground will never be freed until this
     client exits.  Nice, eh?

     But now I can free font without any visible glitches.  */

  if !(*mw).menu.font.is_null ()
    { XFreeFont (XtDisplay (mw as Widget), (*mw).menu.font); }

  /* C 2301-2318: `#if defined USE_CAIRO || defined HAVE_XFT` */
  #[cfg (feature = "use_cairo_or_have_xft")]
  {
    if !(*(*mw).menu.windows.offset (0)).xft_draw.is_null ()
      { XftDrawDestroy ((*(*mw).menu.windows.offset (0)).xft_draw); }
    if !(*mw).menu.xft_font.is_null ()
      { XftFontClose (XtDisplay (mw as Widget), (*mw).menu.xft_font); }
  }

  if (*(*mw).menu.windows.offset (0)).pixmap != None
    { XFreePixmap (XtDisplay (mw as Widget), (*(*mw).menu.windows.offset (0)).pixmap); }
  /* Start from 1 because the one in slot 0 is w->core.window.  */
  i = 1;
  while i < (*mw).menu.windows_length
    {
      if (*(*mw).menu.windows.offset (i as isize)).pixmap != None
        { XFreePixmap (XtDisplay (mw as Widget), (*(*mw).menu.windows.offset (i as isize)).pixmap); }
      /* C 2315-2318: `#if defined USE_CAIRO || defined HAVE_XFT` */
      #[cfg (feature = "use_cairo_or_have_xft")]
      {
        if !(*(*mw).menu.windows.offset (i as isize)).xft_draw.is_null ()
          { XftDrawDestroy ((*(*mw).menu.windows.offset (i as isize)).xft_draw); }
      }
      i += 1;
    }

  if !(*mw).menu.windows.is_null ()
    { XtFree ((*mw).menu.windows as *mut c_char); }
}

/* C 2325-2335: `fontname_changed` (static, `#if defined USE_CAIRO || defined HAVE_XFT`).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe extern "C" fn fontname_changed (newmw: XlwMenuWidget,
                                              oldmw: XlwMenuWidget) -> c_int
{
  /* This will force a new XftFont even if the same string is set.
     This is good, as rendering parameters may have changed and
     we just want to do a redisplay.  */
  return (if (*newmw).menu.fontName != (*oldmw).menu.fontName { 1 } else { 0 });
}

/* C 2337-2431: `XlwMenuSetValues` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn XlwMenuSetValues (current: Widget, _request: Widget, new: Widget,
                                              _args: ArgList, _num_args: *mut Cardinal)
                                              -> Boolean
{
  let oldmw: XlwMenuWidget = current as XlwMenuWidget;
  let newmw: XlwMenuWidget = new as XlwMenuWidget;
  let mut do_redisplay: Boolean = False;

  if !(*newmw).menu.contents.is_null ()
      && !(*(*newmw).menu.contents).contents.is_null ()
      && (*(*(*newmw).menu.contents).contents).change >= VISIBLE_CHANGE
    { do_redisplay = True; }
  /* Do redisplay if the contents are entirely eliminated.  */
  if !(*newmw).menu.contents.is_null ()
      && (*(*newmw).menu.contents).contents.is_null ()
      && (*(*newmw).menu.contents).change >= VISIBLE_CHANGE
    { do_redisplay = True; }

  /* C 2357-2365: the cairo/Xft arm and the `HAVE_X_I18N`/`#else` pair are two more branches of
     the condition; each is carried behind its own gate and contributes the same
     `do_redisplay` decision the C condition spells (`## Deviations`).  */
  #[cfg (feature = "use_cairo_or_have_xft")]
  let fontname_changed_p = fontname_changed (newmw, oldmw) != 0;
  #[cfg (not (feature = "use_cairo_or_have_xft"))]
  let fontname_changed_p = false;
  #[cfg (feature = "have_x_i18n")]
  let font_changed_p = (*newmw).menu.fontSet != (*oldmw).menu.fontSet
                       || ((*newmw).menu.fontSet.is_null ()
                           && (*newmw).menu.font != (*oldmw).menu.font);
  #[cfg (not (feature = "have_x_i18n"))]
  let font_changed_p = (*newmw).menu.font != (*oldmw).menu.font;

  if (*newmw).core.background_pixel != (*oldmw).core.background_pixel
      || (*newmw).menu.foreground != (*oldmw).menu.foreground
      || fontname_changed_p
      || font_changed_p
    {
      let mut i: c_int;
      release_drawing_gcs (newmw);
      make_drawing_gcs (newmw);

      release_shadow_gcs (newmw);
      /* Cause the shadow colors to be recalculated.  */
      (*newmw).menu.top_shadow_color = (-1isize) as Pixel;
      (*newmw).menu.bottom_shadow_color = (-1isize) as Pixel;
      make_shadow_gcs (newmw);

      do_redisplay = True;

      if XtIsRealized (current) != 0
        /* If the menu is currently displayed, change the display.  */
        {
          i = 0;
          while i < (*oldmw).menu.windows_length
            {
              XSetWindowBackground (XtDisplay (oldmw as Widget),
                                    (*(*oldmw).menu.windows.offset (i as isize)).window,
                                    (*newmw).core.background_pixel);
              /* Clear windows and generate expose events.  */
              XClearArea (XtDisplay (oldmw as Widget), (*(*oldmw).menu.windows.offset (i as isize)).window,
                          0, 0, 0, 0, True);
              i += 1;
            }
        }

      /* Colors changed.  Update the Xft colors as well.  */
      /* C 2393-2396: `#if defined USE_CAIRO || defined HAVE_XFT` */
      #[cfg (feature = "use_cairo_or_have_xft")]
      {
        if !(*oldmw).menu.xft_font.is_null ()
          { update_xft_colors (new); }
      }
    }

  /* C 2399-2421: `#if defined USE_CAIRO || defined HAVE_XFT` */
  #[cfg (feature = "use_cairo_or_have_xft")]
  {
    if fontname_changed (newmw, oldmw) != 0
      {
        let mut i: c_int;
        let screen = XScreenNumberOfScreen ((*newmw).core.screen);
        if !(*newmw).menu.xft_font.is_null ()
          { XftFontClose (XtDisplay (newmw as Widget), (*newmw).menu.xft_font); }
        openXftFont (newmw);
        i = 0;
        while i < (*newmw).menu.windows_length
          {
            if !(*(*newmw).menu.windows.offset (i as isize)).xft_draw.is_null ()
              { XftDrawDestroy ((*(*newmw).menu.windows.offset (i as isize)).xft_draw); }
            (*(*newmw).menu.windows.offset (i as isize)).xft_draw = core::ptr::null_mut ();
            i += 1;
          }
        if !(*newmw).menu.xft_font.is_null ()
          {
            i = 0;
            while i < (*newmw).menu.windows_length
              {
                (*(*newmw).menu.windows.offset (i as isize)).xft_draw
                  = XftDrawCreate (XtDisplay (newmw as Widget),
                                   (*(*newmw).menu.windows.offset (i as isize)).window,
                                   DefaultVisual (XtDisplay (newmw as Widget), screen),
                                   (*newmw).core.colormap);
                i += 1;
              }
          }
      }
  }
  /* C 2422-2428: `#ifdef HAVE_X_I18N` */
  #[cfg (feature = "have_x_i18n")]
  {
    if (*newmw).menu.fontSet != (*oldmw).menu.fontSet
       && !(*newmw).menu.fontSet.is_null ()
      {
        do_redisplay = True;
        (*newmw).menu.font_extents = XExtentsOfFontSet ((*newmw).menu.fontSet);
      }
  }

  return do_redisplay;
}

/* C 2433-2452: `XlwMenuResize` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn XlwMenuResize (w: Widget)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;

  if (*mw).menu.popped_up != 0
    {
      /* Don't allow the popup menu to resize itself.  */
      (*mw).core.width = (*(*mw).menu.windows.offset (0)).width;
      (*mw).core.height = (*(*mw).menu.windows.offset (0)).height;
      (*(*mw).core.parent).core.width = (*mw).core.width;
      (*(*mw).core.parent).core.height = (*mw).core.height;
    }
  else
    {
      (*(*mw).menu.windows.offset (0)).width = (*mw).core.width;
      (*(*mw).menu.windows.offset (0)).height = (*mw).core.height;
      create_pixmap_for_menu (&raw mut (*(*mw).menu.windows.offset (0)), mw);
    }
}

/* Action procedures */
/* C 2454-2469: `handle_single_motion_event` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn handle_single_motion_event (mw: XlwMenuWidget, ev: *mut XMotionEvent)
{
  let mut val: *mut widget_value = core::ptr::null_mut ();
  let mut level: c_int = 0;

  if map_event_to_widget_value (mw, ev, &raw mut val, &raw mut level) == 0
    { pop_new_stack_if_no_contents (mw); }
  else
    { set_new_state (mw, val, level); }
  remap_menubar (mw);

  /* Sync with the display.  Makes it feel better on X terms.  */
  XSync (XtDisplay (mw as Widget), False);
}

/* C 2471-2491: `handle_motion_event` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn handle_motion_event (mw: XlwMenuWidget, ev: *mut XMotionEvent)
{
  let x = (*ev).x_root;
  let y = (*ev).y_root;
  let state = (*ev).state;
  let mut oldev: XMotionEvent = *ev;

  /* Allow motion events to be generated again.  */
  if (*ev).is_hint != 0
      && XQueryPointer (XtDisplay (mw as Widget), (*ev).window,
                        &raw mut (*ev).root, &raw mut (*ev).subwindow,
                        &raw mut (*ev).x_root, &raw mut (*ev).y_root,
                        &raw mut (*ev).x, &raw mut (*ev).y,
                        &raw mut (*ev).state) != 0
      && (*ev).state == state
      && ((*ev).x_root != x || (*ev).y_root != y)
    { handle_single_motion_event (mw, ev); }
  else
    { handle_single_motion_event (mw, &raw mut oldev); }
}

/* C 2493-2524: `Start` (static; its address goes into `xlwMenuActionsList`).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn Start (w: Widget, ev: *mut XEvent, _params: *mut String,
                                  _num_params: *mut Cardinal)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;

  if (*mw).menu.popped_up == 0
    {
      menu_post_event = *ev;
      /* If event is set to CurrentTime, get the last known time stamp.
         This is for calculating if (popup) menus should stay up after
         a fast click.  */
      if menu_post_event.xbutton.time == CurrentTime
        { menu_post_event.xbutton.time
            = XtLastTimestampProcessed (XtDisplay (w)); }

      pop_up_menu (mw, ev as *mut XButtonPressedEvent);
    }
  else
    {
      /* If we push a button while the menu is posted semipermanently,
         releasing the button should always pop the menu down.  */
      next_release_must_exit = 1;

      /* Notes the absolute position of the menubar window.  */
      (*(*mw).menu.windows.offset (0)).x = ((*ev).xmotion.x_root - (*ev).xmotion.x) as Position;
      (*(*mw).menu.windows.offset (0)).y = ((*ev).xmotion.y_root - (*ev).xmotion.y) as Position;

      /* Handles the down like a move, slots are compatible.  */
      (*ev).xmotion.is_hint = 0;
      handle_motion_event (mw, &raw mut (*ev).xmotion);
    }
}

/* C 2526-2533: `Drag` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn Drag (w: Widget, ev: *mut XEvent, _params: *mut String,
                                  _num_params: *mut Cardinal)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;
  if (*mw).menu.popped_up != 0
    { handle_motion_event (mw, &raw mut (*ev).xmotion); }
}

/* Do nothing.
   This is how we handle presses and releases of modifier keys.  */
/* C 2536-2539: `Nothing` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn Nothing (_w: Widget, _ev: *mut XEvent, _params: *mut String,
                                     _num_params: *mut Cardinal)
{
}

/* C 2541-2555: `find_first_selectable` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn find_first_selectable (_mw: XlwMenuWidget,
                                                   item: *mut widget_value,
                                                   skip_titles: c_int) -> *mut widget_value
{
  let mut current: *mut widget_value = item;
  let mut separator: menu_separator = 0;

  while lw_separator_p ((*current).name, &raw mut separator, 0) != 0
        || !(*current).enabled
        || (skip_titles != 0 && (*current).call_data.is_null ()
            && (*current).contents.is_null ())
    {
      if !(*current).next.is_null ()
        { current = (*current).next; }
      else
        { return core::ptr::null_mut (); }
    }

  return current;
}

/* C 2557-2589: `find_next_selectable` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn find_next_selectable (mw: XlwMenuWidget,
                                                  item: *mut widget_value,
                                                  skip_titles: c_int) -> *mut widget_value
{
  let mut current: *mut widget_value = item;
  let mut separator: menu_separator = 0;

  loop {
    if !(!(*current).next.is_null ()
          && { current = (*current).next; true }
          && (lw_separator_p ((*current).name, &raw mut separator, 0) != 0
              || !(*current).enabled
              || (skip_titles != 0 && (*current).call_data.is_null ()
                  && (*current).contents.is_null ())))
      { break; }
  }

  if current == item
    {
      if (*mw).menu.old_depth < 2
        { return current; }
      current = (*(*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 2) as isize))).contents;

      loop {
        if !(lw_separator_p ((*current).name, &raw mut separator, 0) != 0
             || !(*current).enabled
             || (skip_titles != 0 && (*current).call_data.is_null ()
                 && (*current).contents.is_null ()))
          { break; }
        {
          if !(*current).next.is_null ()
            { current = (*current).next; }

          if current == item
            { break; }
        }
      }

    }

  return current;
}

/* C 2591-2606: `find_prev_selectable` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn find_prev_selectable (mw: XlwMenuWidget,
                                                  item: *mut widget_value,
                                                  skip_titles: c_int) -> *mut widget_value
{
  let mut current: *mut widget_value = item;
  let mut prev: *mut widget_value = item;

  loop {
    current = find_next_selectable (mw, current, skip_titles);
    if current == item
      { break; }
    {
      if prev == current
        { break; }
      prev = current;
    }
  }

  return prev;
}

/* C 2608-2631: `Down` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn Down (w: Widget, _ev: *mut XEvent, _params: *mut String,
                                  _num_params: *mut Cardinal)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;
  let selected_item: *mut widget_value =
    (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 1) as isize));
  let popup_menu_p = if (*mw).menu.top_depth == 1 { 1 } else { 0 };

  /* Inside top-level menu-bar?  */
  if (*mw).menu.old_depth == (*mw).menu.top_depth
    /* When <down> in the menu-bar is pressed, display the corresponding
       sub-menu and select the first selectable menu item there.
       If this is a popup menu, skip title item of the popup.  */
    { set_new_state (mw,
                     find_first_selectable (mw,
                                            (*selected_item).contents,
                                            popup_menu_p),
                     (*mw).menu.old_depth); }
  else
    /* Highlight next possible (enabled and not separator) menu item.  */
    { set_new_state (mw, find_next_selectable (mw, selected_item, popup_menu_p),
                     (*mw).menu.old_depth - 1); }

  remap_menubar (mw);
}

/* C 2633-2668: `Up` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn Up (w: Widget, _ev: *mut XEvent, _params: *mut String,
                                _num_params: *mut Cardinal)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;
  let mut selected_item: *mut widget_value =
    (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 1) as isize));
  let popup_menu_p = if (*mw).menu.top_depth == 1 { 1 } else { 0 };

  /* Inside top-level menu-bar?  */
  if (*mw).menu.old_depth == (*mw).menu.top_depth
    {
      /* FIXME: this is tricky.  <up> in the menu-bar should select the
         last selectable item in the list.  So we select the first
         selectable one and find the previous selectable item.  Is there
         a better way?  */
      /* If this is a popup menu, skip title item of the popup.  */
      set_new_state (mw,
                     find_first_selectable (mw,
                                            (*selected_item).contents,
                                            popup_menu_p),
                     (*mw).menu.old_depth);
      remap_menubar (mw);
      selected_item = (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 1) as isize));
      set_new_state (mw,
                     find_prev_selectable (mw,
                                           selected_item,
                                           popup_menu_p),
                     (*mw).menu.old_depth - 1);
    }
  else
    /* Highlight previous (enabled and not separator) menu item.  */
    { set_new_state (mw, find_prev_selectable (mw, selected_item, popup_menu_p),
                     (*mw).menu.old_depth - 1); }

  remap_menubar (mw);
}

/* C 2670-2704: `Left` (non-static).  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Left (w: Widget, _ev: *mut XEvent, _params: *mut String,
                               _num_params: *mut Cardinal)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;
  let mut selected_item: *mut widget_value =
    (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 1) as isize));

  /* Inside top-level menu-bar?  */
  if (*mw).menu.old_depth == (*mw).menu.top_depth
    /* When <left> in the menu-bar is pressed, display the previous item on
       the menu-bar. If the current item is the first one, highlight the
       last item in the menubar (probably Help).  */
    { set_new_state (mw, find_prev_selectable (mw, selected_item, 0),
                     (*mw).menu.old_depth - 1); }
  else if (*mw).menu.old_depth == 1
          && !(*selected_item).contents.is_null ()  /* Is this menu item expandable?  */
    {
      set_new_state (mw, (*selected_item).contents, (*mw).menu.old_depth);
      remap_menubar (mw);
      selected_item = (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 1) as isize));
      if !(*selected_item).enabled
         && !find_first_selectable (mw, selected_item, 0).is_null ()
        { set_new_state (mw, find_first_selectable (mw, selected_item, 0),
                         (*mw).menu.old_depth - 1); }
    }

  else
    {
      pop_new_stack_if_no_contents (mw);
      set_new_state (mw, (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 2) as isize)),
                     (*mw).menu.old_depth - 2);
    }

  remap_menubar (mw);
}

/* C 2706-2740: `Right` (non-static).  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Right (w: Widget, _ev: *mut XEvent, _params: *mut String,
                                _num_params: *mut Cardinal)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;
  let mut selected_item: *mut widget_value =
    (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 1) as isize));

  /* Inside top-level menu-bar?  */
  if (*mw).menu.old_depth == (*mw).menu.top_depth
    /* When <right> in the menu-bar is pressed, display the next item on
       the menu-bar. If the current item is the last one, highlight the
       first item (probably File).  */
    { set_new_state (mw, find_next_selectable (mw, selected_item, 0),
                     (*mw).menu.old_depth - 1); }
  else if !(*selected_item).contents.is_null ()  /* Is this menu item expandable?  */
    {
      set_new_state (mw, (*selected_item).contents, (*mw).menu.old_depth);
      remap_menubar (mw);
      selected_item = (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 1) as isize));
      if !(*selected_item).enabled
         && !find_first_selectable (mw, selected_item, 0).is_null ()
        { set_new_state (mw, find_first_selectable (mw, selected_item, 0),
                         (*mw).menu.old_depth - 1); }
    }
  else
    {
      pop_new_stack_if_no_contents (mw);
      set_new_state (mw, (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 2) as isize)),
                     (*mw).menu.old_depth - 2);
    }

  remap_menubar (mw);
}

/* Handle key press and release events while menu is popped up.
   Our action is to get rid of the menu.  */
/* C 2741-2765: `Key` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn Key (w: Widget, ev: *mut XEvent, _params: *mut String,
                                 _num_params: *mut Cardinal)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;

  /* Pop down everything.  */
  (*mw).menu.new_depth = 1;
  remap_menubar (mw);

  if (*mw).menu.popped_up != 0
    {
      (*mw).menu.popped_up = False;
      ungrab_all (mw as Widget, (*ev).xmotion.time);
      if XtIsShell (XtParent (mw as Widget)) != 0
        { XtPopdown (XtParent (mw as Widget)); }
      else
        {
          XtRemoveGrab (mw as Widget);
          display_menu (mw, 0, False, core::ptr::null_mut (), core::ptr::null_mut (),
                        core::ptr::null_mut ());
        }
    }

  /* callback */
  XtCallCallbackList (mw as Widget, (*mw).menu.select, 0 as XtPointer);
}

/* C 2767-2805: `Select` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn Select (w: Widget, ev: *mut XEvent, _params: *mut String,
                                    _num_params: *mut Cardinal)
{
  let mw: XlwMenuWidget = w as XlwMenuWidget;
  let selected_item: *mut widget_value =
    (*(*mw).menu.old_stack.offset (((*mw).menu.old_depth - 1) as isize));

  /* If user releases the button quickly, without selecting anything,
     after the initial down-click that brought the menu up,
     do nothing.  */
  if (selected_item.is_null ()
      || (*selected_item).call_data.is_null ())
      && next_release_must_exit == 0
      && ((*ev).xbutton.time - menu_post_event.xbutton.time
          < XtGetMultiClickTime (XtDisplay (w)) as c_ulong)
    { return; }

  /* Pop down everything.  */
  (*mw).menu.new_depth = 1;
  remap_menubar (mw);

  if (*mw).menu.popped_up != 0
    {
      (*mw).menu.popped_up = False;
      ungrab_all (mw as Widget, (*ev).xmotion.time);
      if XtIsShell (XtParent (mw as Widget)) != 0
        { XtPopdown (XtParent (mw as Widget)); }
      else
        {
          XtRemoveGrab (mw as Widget);
          display_menu (mw, 0, False, core::ptr::null_mut (), core::ptr::null_mut (),
                        core::ptr::null_mut ());
        }
    }

  /* callback */
  XtCallCallbackList (mw as Widget, (*mw).menu.select, selected_item as XtPointer);
}


/* Special code to pop-up a menu.  */
/* C 2806-2919: `pop_up_menu` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn pop_up_menu (mw: XlwMenuWidget, event: *mut XButtonPressedEvent)
{
  let mut x: c_int = (*event).x_root;
  let mut y: c_int = (*event).y_root;
  let w: c_int;
  let h: c_int;
  let borderwidth = (*mw).menu.shadow_thickness as c_int;
  let screen: *mut Screen = XtScreen (mw as Widget);
  let display: *mut Display = XtDisplay (mw as Widget);
  let mut screen_x: c_int = 0;
  let mut screen_y: c_int = 0;
  let mut screen_w: c_int = 0;
  let mut screen_h: c_int = 0;

  /* C 2821-2830: `#ifdef emacs` and its `#else` arm.  */
  #[cfg (feature = "emacs")]
  {
    xlw_monitor_dimensions_at_pos (display, screen, x, y,
                                   &raw mut screen_x, &raw mut screen_y,
                                   &raw mut screen_w, &raw mut screen_h);
  }
  #[cfg (not (feature = "emacs"))]
  {
    screen_x = 0;
    screen_y = 0;
    screen_w = WidthOfScreen (screen);
    screen_h = HeightOfScreen (screen);
  }
  next_release_must_exit = 0;

  (*mw).menu.inside_entry = core::ptr::null_mut ();
  XtCallCallbackList (mw as Widget, (*mw).menu.open, core::ptr::null_mut ());

  if XtIsShell (XtParent (mw as Widget)) != 0
    { size_menu (mw, 0); }

  w = (*(*mw).menu.windows.offset (0)).width as c_int;
  h = (*(*mw).menu.windows.offset (0)).height as c_int;

  x -= borderwidth;
  y -= borderwidth;
  if x < screen_x + borderwidth
    { x = screen_x + borderwidth; }
  if x + w + 2 * borderwidth > screen_x + screen_w
    { x = (screen_x + screen_w) - w - 2 * borderwidth; }
  if y < screen_y + borderwidth
    { y = screen_y + borderwidth; }
  if y + h + 2 * borderwidth > screen_y + screen_h
    { y = (screen_y + screen_h) - h - 2 * borderwidth; }

  (*mw).menu.popped_up = True;
  if XtIsShell (XtParent (mw as Widget)) != 0
    {
      /* fprintf (stderr, "Config %d %d\n", x, y); */
      XtConfigureWidget (XtParent (mw as Widget), x as Position, y as Position,
                         w as Dimension, h as Dimension,
                         (*(XtParent (mw as Widget))).core.border_width);
      XtPopup (XtParent (mw as Widget), XtGrabExclusive);
      display_menu (mw, 0, False, core::ptr::null_mut (), core::ptr::null_mut (),
                    core::ptr::null_mut ());
      (*(*mw).menu.windows.offset (0)).x = (x + borderwidth) as Position;
      (*(*mw).menu.windows.offset (0)).y = (y + borderwidth) as Position;
      (*mw).menu.top_depth = 1;  /* Popup menus don't have a bar so top is 1.  */
    }
  else
    {
      let ev: *mut XEvent = event as *mut XEvent;

      XtAddGrab (mw as Widget, True, True);

      /* Notes the absolute position of the menubar window.  */
      (*(*mw).menu.windows.offset (0)).x = ((*ev).xmotion.x_root - (*ev).xmotion.x) as Position;
      (*(*mw).menu.windows.offset (0)).y = ((*ev).xmotion.y_root - (*ev).xmotion.y) as Position;
      (*mw).menu.top_depth = 2;
    }

  /* C 2877-2879: `#ifdef emacs` */
  #[cfg (feature = "emacs")]
  { x_catch_errors (display); }
  if XtGrabPointer (mw as Widget, False,
                    (PointerMotionMask
                     | PointerMotionHintMask
                     | ButtonReleaseMask
                     | ButtonPressMask) as c_uint,
                    GrabModeAsync, GrabModeAsync, None,
                    (*mw).menu.cursor_shape,
                    (*event).time) == Success
    {
      /* C 2890-2892: `#ifdef emacs` (the `&& lucid__menu_grab_keyboard` conjunct).  */
      let mut want_keyboard_p = true;
      #[cfg (feature = "emacs")]
      { if !lucid__menu_grab_keyboard { want_keyboard_p = false; } }
      if want_keyboard_p
         && XtGrabKeyboard (mw as Widget, False, GrabModeAsync,
                            GrabModeAsync, (*event).time) == Success
        {
          XtSetKeyboardFocus (mw as Widget, None as Widget);
          pointer_grabbed = 1;
          keyboard_grabbed = 1;
        }
      else
        {
          XtUngrabPointer (mw as Widget, (*event).time);
          keyboard_grabbed = 0;
        }
    }

  /* C 2907-2914: `#ifdef emacs` */
  #[cfg (feature = "emacs")]
  {
    if x_had_errors_p (display)
      {
        pointer_grabbed = 0;
        XtUngrabPointer (mw as Widget, (*event).time);
      }
    x_uncatch_errors ();
  }

  (*(event as *mut XMotionEvent)).is_hint = 0;
  handle_motion_event (mw, event as *mut XMotionEvent);

  XlwMenuRedisplay (mw as Widget, core::ptr::null_mut (), None as Region);
}
