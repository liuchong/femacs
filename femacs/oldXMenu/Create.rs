// femacs: port of oldXMenu/Create.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The one non-`static` definition (`XMenuCreate`) carries `#[unsafe(no_mangle)]`
// (PORTING.md 0.2); the `static` helper `XAllocDisplayColor` and the twelve file-scope `static`
// XBM objects the file includes stay plain.
//
// **The file gate is `feature = "have_x_windows"`** (P-86 clause 8, the build's condition rather
// than one the C text spells).  `configure.ac` 7287-7295 selects this library - the arm that sets
// `HAVE_OLDXMENU=yes` and `LIBXMENU='$(oldXMenudir)/libXMenu11.a'` is `HAVE_X11 = yes` with
// `HAVE_GTK != yes` and `USE_X_TOOLKIT = none` - and `oldXMenu/Makefile.in`'s `OBJS` block (60-85)
// lists every object of the directory unconditionally (this file's object is `Create.o`, the
// sixth), so the reference text carries **no macro of its own** for this file: the condition is a
// configure result and the feature is the repo's name for the X11 window system
// (`HAVE_X_WINDOWS`), the gate `src/xfns.rs`'s unit uses.  P-86 clause 8 asks for the reference's
// own macro lower-cased; this file has none, which the ledger's `## Deviations` 1 records.
//
// **The one sub-gate the C text spells is `EMACS_BITMAP_FILES` at C 54, and the reference's own
// build decides it**: `oldXMenu/Makefile.in` 110-113's `ALL_CFLAGS` carries
// `-DEMACS_BITMAP_FILES` on the rule that compiles every object of the directory, so the live arm
// is C 55-67 - the twelve in-tree `../src/bitmaps/*.xbm` includes - and C 69-85's
// `<X11/bitmaps/*>` arm is dead for this library.  A *constant* condition rather than a `cfg`, so
// the port carries the live arm's twelve objects and records the dead arm by range (the ledger's
// `## C line ranges` and `## Deviations` 3).  The two arms name the same twelve X11R4 bitmaps
// anyway (`emacs/src/bitmaps/README` 9-20's correspondence table).
//
// `HAVE_X11`, `HAVE_X_WINDOWS` and `USE_X_TOOLKIT` are `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv`, so the frozen configuration compiles not one
// line of this unit and `xtask build-tu` leaves the empty object - and this machine has no X11
// *program* to run it against.  The `<X11/*>` declarations below are therefore measured against
// `/opt/homebrew/include` (xorgproto 2025.1 and libX11 1.8.13) with clang rather than taken from a
// build; every value, size and offset this unit carries is one of those readings, and the commands
// are in the ledger's `## Evidence`.
//
// The `<X11/Xlib.h>` surface this file reads is the shared mirror `include/xlib.rs` where that
// mirror carries the name (P-90 clause 1); what it does not carry yet is declared in this unit at
// the use site, with C's own text in a `/// C:` comment (P-88 clause 1 / P-55).  The ledger's
// `## Owed` names the move back into the mirror.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_mut, unused_parens,
         unused_comparisons, clippy::all)]

/* C 1-48: the file's opening block - the MIT notice, the FSF notice and the XMenuCreate
   description comment - is not transcribed (PORTING.md 0.1-19; the ledger's `## C line ranges`
   records the range).  */

/* C 50: `#include "XMenuInt.h"`.  Its mirror holds the declarations of the sixteen internal
   functions this unit calls, and Rust's "declaration + definition" of one name in one scope is
   `E0428`, while an explicit item shadows a glob import - which is C's pair (PORTING.md 0.1-10 /
   P-21, the `oldXMenu/Internal.rs` 44-53 shape).  So the mirror is included inside a module that
   is glob-imported, and the `#[macro_use]` is what carries `min`/`max`/`abs` across (a glob
   import does not import `macro_rules!` items).  */
#[cfg (feature = "have_x_windows")]
#[macro_use]
mod xmenu_int {
  use super::*;
  include!("include/XMenuInt.rs");             // C 50: `#include "XMenuInt.h"`
}
#[cfg (feature = "have_x_windows")]
use xmenu_int::*;

/* C 51: `#include <stdlib.h>` - the header `calloc` and `malloc` come from.  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/stdlib.rs");          // C 51: `#include <stdlib.h>`

/* `lib/include/stdlib.rs` carries the allocator family and `strtod`, but not the two ASCII
   conversions this file reads (`atoi` C 368/429, `atof` C 371/432): they have no home in a mirror
   yet, so P-88 clause 1 declares them here at their use site with the header's own text.  The
   ledger's `## Owed` 2 names the move.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: `<stdlib.h>`: `int atoi (const char *nptr);`
  pub fn atoi (nptr: *const c_char) -> c_int;
  /// C: `<stdlib.h>`: `double atof (const char *nptr);`
  pub fn atof (nptr: *const c_char) -> c_double;
}

/* C 52: `#include <string.h>` - the header `strcmp` comes from.  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/string.rs");          // C 52: `#include <string.h>`

/* XMenuInt.h 46: `#include <X11/Xlib.h>` - the header this file reaches the X11 surface through.
   The mirror that owns that surface is `include/xlib.rs` (P-90 clause 1), and the names below it
   lacks are declared in this unit at their use sites (P-88 clause 1).  */
#[cfg (feature = "have_x_windows")]
include!("../src/include/xlib.rs");            // XMenuInt.h 46: `#include <X11/Xlib.h>`

/* The C fixed-width spellings.  Every unit that includes `lisp.h` gets them from that mirror's
   crate-root `use std::ffi::{…}`; XMenu's headers include no `lisp.h`, so this unit brings the
   whole set itself, the shape `oldXMenu/Internal.rs` 63-66 uses - and the `include!`d mirrors
   below (`xlib.rs`, `XMenu.rs`, `XMenuInt.rs`, `X10.rs`) read these names at the crate root.
   `c_double` is here for `lib/include/stdlib.rs` alone, whose `strtod` names it (the
   `lib-src/hexl.rs` 23 and `exec/exec1.rs` 21 sets carry it for that same declaration).  */
#[cfg (feature = "have_x_windows")]
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};

/// The Xlib names this unit's own text spells.  `include/xlib.rs` wraps the Xlib surface of
/// `<X11/Xlib.h>` in a module (`xlib_surface`) and ends with a *private* `use xlib_surface::*;`,
/// so the crate root of a unit that includes it does not have those names; `Bool` and `x11_constants`
/// sit at the mirror's top level, `Display`/`Window`/…/`XColor` inside that module, and this unit
/// takes exactly the names it needs from where each one lives (PORTING.md 0.1-9's rule, the
/// `oldXMenu/Activate.rs` 67 shape).
#[cfg (feature = "have_x_windows")]
use xlib_surface::{Atom, Colormap, Cursor, Display, Drawable, Font, GC, Pixmap, Window, XColor,
                   XExtData};
/// The three entry points `include/xlib.rs` carries in its own `xlib_surface` module and this unit
/// calls (the `XCreateBitmapFromData` of C 493/498/514/519/535/540/661, the `XFreePixmap` of
/// C 510-511/531-532/552-553 and the `XCreateGC` of C 632/645/657/678).
#[cfg (feature = "have_x_windows")]
use xlib_surface::{XCreateBitmapFromData, XCreateGC, XFreePixmap};

/// Xlib's event masks and the two GC value-mask bits the mirror carries come from the mirror's own
/// `x11_constants` module; the five this file reads are imported here (PORTING.md 0.1-9: a unit
/// takes the names it needs from the module rather than glob-importing it).  `None` - the sixth
/// name this file reads - is imported inside `XMenuCreate` instead, because a crate-root `None`
/// would shadow the prelude's for the whole unit (the `oldXMenu/Activate.rs` 76-81 note).
#[cfg (feature = "have_x_windows")]
use x11_constants::{EnterWindowMask, ExposureMask, GCForeground, GCBackground, LeaveWindowMask};

/* ------------------------------------------------------------------------------------------
   The X11 surface `include/xlib.rs` does not carry yet (P-88 clause 1: no mirror holds these
   names, and this file is their only consumer today).  Every number below is measured with clang
   against this machine's headers; each `/// C:` line keeps the header's own text and line.
   ------------------------------------------------------------------------------------------ */

/// C: Xlib.h 79: `#define Status int` - C spells the return type of the Xlib calls that report
/// success or failure.  Measured `sizeof (Status) == 4`.
#[cfg (feature = "have_x_windows")]
pub type Status = c_int;

/// C: Xlib.h 1013-1020: `typedef struct { short lbearing; short rbearing; short width; short
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
/// descent; } XFontStruct;` - the record `XLoadQueryFont` returns and C 584-585 reads
/// (`max_bounds.ascent` + `.descent`), and the one `XMenu.h` 274/288 stores as `*mut XFontStruct`.
/// Carried whole (the `oldXMenu/Internal.rs` 150-183 reading of the same record); measured
/// `[sizeof=96, align=8]` with `fid` 8, `direction` 16, `all_chars_exist` 36, `properties` 48,
/// `min_bounds` 56, `max_bounds` **68**, `per_char` 80, `ascent` 88, `descent` 92.
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
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XFontStruct, descent) == 92);

/// C: Xlib.h 177-203: `typedef struct { int function; unsigned long plane_mask; unsigned long
/// foreground; unsigned long background; int line_width; … Pixmap tile; Pixmap stipple; … Font
/// font; int subwindow_mode; Bool graphics_exposures; … char dashes; } XGCValues;` - the record
/// C 618 allocates and C 627-684 fills for the four `XCreateGC` calls.  `include/xlib.rs`'s
/// `XCreateGC` takes the record's pointer as `*mut c_void` ("a record no ported arm reads" was
/// true until this unit, and this one does), so this unit casts at those four call sites.  Carried
/// whole; measured `[sizeof=128, align=8]` with `foreground` 16, `background` 24, `line_width` 32,
/// `line_style` 36, `fill_style` 48, `stipple` 72, `font` 88, `graphics_exposures` 100,
/// `dash_offset` 120.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XGCValues
{
  pub function: c_int,
  pub plane_mask: c_ulong,
  pub foreground: c_ulong,
  pub background: c_ulong,
  pub line_width: c_int,
  pub line_style: c_int,
  pub cap_style: c_int,
  pub join_style: c_int,
  pub fill_style: c_int,
  pub fill_rule: c_int,
  pub arc_mode: c_int,
  pub tile: Pixmap,
  pub stipple: Pixmap,
  pub ts_x_origin: c_int,
  pub ts_y_origin: c_int,
  pub font: Font,
  pub subwindow_mode: c_int,
  pub graphics_exposures: Bool,
  pub clip_x_origin: c_int,
  pub clip_y_origin: c_int,
  pub clip_mask: Pixmap,
  pub dash_offset: c_int,
  pub dashes: c_char,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XGCValues> () == 128);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XGCValues> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XGCValues, foreground) == 16);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XGCValues, background) == 24);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XGCValues, line_width) == 32);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XGCValues, fill_style) == 48);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XGCValues, stipple) == 72);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XGCValues, font) == 88);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XGCValues, graphics_exposures) == 100);

/* The entry points `include/xlib.rs` does not carry yet, each with the header's own prototype
   (P-88 clause 1).  All ten are exported by libX11 1.8.13 on this machine (the ledger's
   Evidence 3).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: Xlib.h 1765-1768: `extern Window XRootWindow (Display*, int);` - `RootWindow`'s entry.
  pub fn XRootWindow (display: *mut Display, screen_number: c_int) -> Window;
  /// C: Xlib.h 2237-2239: `extern int XDefaultScreen (Display*);` - `DefaultScreen`'s entry.
  pub fn XDefaultScreen (display: *mut Display) -> c_int;
  /// C: Xlib.h 2276-2279: `extern int XDisplayCells (Display*, int);` - `DisplayCells`' entry.
  pub fn XDisplayCells (display: *mut Display, screen_number: c_int) -> c_int;
  /// C: Xlib.h 1818-1821: `extern Colormap XDefaultColormap (Display*, int);` - `DefaultColormap`'s
  /// entry.
  pub fn XDefaultColormap (display: *mut Display, screen_number: c_int) -> Colormap;
  /// C: Xlib.h 2297-2300: `extern int XDisplayPlanes (Display*, int);` - `DisplayPlanes`' entry.
  pub fn XDisplayPlanes (display: *mut Display, screen_number: c_int) -> c_int;
  /// C: Xlib.h 2860-2865: `extern Status XParseColor (Display*, Colormap, _Xconst char*,
  /// XColor*);`
  pub fn XParseColor (display: *mut Display, colormap: Colormap, spec: *const c_char,
                      exact_def_return: *mut XColor) -> Status;
  /// C: Xlib.h 1968-1972: `extern Status XAllocColor (Display*, Colormap, XColor*);`
  pub fn XAllocColor (display: *mut Display, colormap: Colormap,
                      screen_in_out: *mut XColor) -> Status;
  /// C: Xlib.h 1390-1393: `extern XFontStruct *XLoadQueryFont (Display*, _Xconst char*);`
  pub fn XLoadQueryFont (display: *mut Display, name: *const c_char) -> *mut XFontStruct;
  /// C: Xlib.h 1554-1562: `extern Cursor XCreatePixmapCursor (Display*, Pixmap, Pixmap, XColor*,
  /// XColor*, unsigned int, unsigned int);`
  pub fn XCreatePixmapCursor (display: *mut Display, source: Pixmap, mask: Pixmap,
                              foreground_color: *mut XColor, background_color: *mut XColor,
                              x: c_uint, y: c_uint) -> Cursor;
  /// C: Xlib.h 1607-1616: `extern Pixmap XCreatePixmapFromBitmapData (Display*, Drawable, char*,
  /// unsigned int, unsigned int, unsigned long, unsigned long, unsigned int);`
  pub fn XCreatePixmapFromBitmapData (display: *mut Display, d: Drawable, data: *mut c_char,
                                      width: c_uint, height: c_uint, fg: c_ulong, bg: c_ulong,
                                      depth: c_uint) -> Pixmap;
}

/* The five Xlib macros C 185, 265, 267, 289, 291, 330, 332, 350, 352, 386, 388, 409, 411, 484
   and 598 read.  Their bodies read Xlib's private display record (`_XPrivDisplay`, Xlib.h 85-119),
   which no mirror here carries, so the port calls the five entry points that read the same fields -
   the same values, the same parameter lists, and the reading `include/xlib.rs`'s
   `DefaultScreenOfDisplay` and `oldXMenu/Internal.rs` 298-303 already take (the ledger's
   `## Deviations` 4).  */
/// C: Xlib.h 88: `#define RootWindow(dpy, scr) (ScreenOfDisplay(dpy,scr)->root)`.
#[cfg (feature = "have_x_windows")]
#[inline (always)]
pub unsafe fn RootWindow (dpy: *mut Display, scr: c_int) -> Window { XRootWindow (dpy, scr) }
/// C: Xlib.h 89: `#define DefaultScreen(dpy) (((_XPrivDisplay)(dpy))->default_screen)`.
#[cfg (feature = "have_x_windows")]
#[inline (always)]
pub unsafe fn DefaultScreen (dpy: *mut Display) -> c_int { XDefaultScreen (dpy) }
/// C: Xlib.h 102: `#define DisplayCells(dpy, scr) (DefaultVisual(dpy,scr)->map_entries)`.
#[cfg (feature = "have_x_windows")]
#[inline (always)]
pub unsafe fn DisplayCells (dpy: *mut Display, scr: c_int) -> c_int { XDisplayCells (dpy, scr) }
/// C: Xlib.h 110: `#define DefaultColormap(dpy, scr)(ScreenOfDisplay(dpy,scr)->cmap)`.
#[cfg (feature = "have_x_windows")]
#[inline (always)]
pub unsafe fn DefaultColormap (dpy: *mut Display, scr: c_int) -> Colormap
{ XDefaultColormap (dpy, scr) }
/// C: Xlib.h 101: `#define DisplayPlanes(dpy, scr) (ScreenOfDisplay(dpy,scr)->root_depth)`.
#[cfg (feature = "have_x_windows")]
#[inline (always)]
pub unsafe fn DisplayPlanes (dpy: *mut Display, scr: c_int) -> c_int { XDisplayPlanes (dpy, scr) }

/// C: Xlib.h 80-81: `#define True 1` / `#define False 0` (the header spells them with `#define`,
/// and `Bool` is Xlib.h 78's `int`).  Measured: one and zero.  C 684 reads `False`; the mirror
/// carries `Bool` but neither value (the `oldXMenu/Activate.rs` 143-149 shape).
#[cfg (feature = "have_x_windows")]
pub const True: Bool = 1;
#[cfg (feature = "have_x_windows")]
pub const False: Bool = 0;

/* The five GC value-mask bits and the fill-style value `include/xlib.rs`'s `x11_constants`
   module does not carry yet (it has `GCForeground`/`GCBackground`, its 77-80).  Measured on this
   machine's `<X11/X.h>`: `GCFont` 16384, `GCLineWidth` 16, `GCFillStyle` 256, `GCStipple` 2048,
   `GCGraphicsExposures` 65536, `FillStippled` 2.  */
/// C: X11/X.h 578: `#define GCFont (1L<<14)`.
#[cfg (feature = "have_x_windows")]
pub const GCFont: c_int = 1 << 14;
/// C: X11/X.h 568: `#define GCLineWidth (1L<<4)`.
#[cfg (feature = "have_x_windows")]
pub const GCLineWidth: c_int = 1 << 4;
/// C: X11/X.h 572: `#define GCFillStyle (1L<<8)`.
#[cfg (feature = "have_x_windows")]
pub const GCFillStyle: c_int = 1 << 8;
/// C: X11/X.h 575: `#define GCStipple (1L<<11)`.
#[cfg (feature = "have_x_windows")]
pub const GCStipple: c_int = 1 << 11;
/// C: X11/X.h 580: `#define GCGraphicsExposures (1L<<16)`.
#[cfg (feature = "have_x_windows")]
pub const GCGraphicsExposures: c_int = 1 << 16;
/// C: X11/X.h 525: `#define FillStippled 2` - the `XGCValues.fill_style` C 675 sets.
#[cfg (feature = "have_x_windows")]
pub const FillStippled: c_int = 2;
/* C 55-67: the live arm of C 54's `#ifdef EMACS_BITMAP_FILES`.  The twelve `.xbm` files have no
   mirror in this tree (the reference's `bitmaps/` directory has no ported file) and this unit is
   their only consumer, so P-88 clause 1 declares the twelve objects here, byte for byte from the
   twelve reference files; the sizes and the three hot spots come from the `#define`s at the head of
   each file.  The byte literals are spelled `0xNNu8 as c_char`: `c_char` is signed on this target
   and C's `'\xNN'` char literals with the high bit set are out of range for `i8` (measured:
   `error: literal out of range for i8`, the ledger's `## Deviations` 5).  */

/// C 55: `#include "../src/bitmaps/dimple1.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut dimple1_bits: [c_char; 32] =          // C: src/bitmaps/dimple1.xbm 3-6
  [
    0x55u8 as c_char, 0x55u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char,
    0x55u8 as c_char, 0x55u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char,
    0x55u8 as c_char, 0x55u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char,
  ];
/// C: src/bitmaps/dimple1.xbm 1: `#define dimple1_width 16`
#[cfg (feature = "have_x_windows")]
pub const dimple1_width: c_int = 16;
/// C: src/bitmaps/dimple1.xbm 2: `#define dimple1_height 16`
#[cfg (feature = "have_x_windows")]
pub const dimple1_height: c_int = 16;

/// C 56: `#include "../src/bitmaps/dimple3.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut dimple3_bits: [c_char; 32] =          // C: src/bitmaps/dimple3.xbm 3-6
  [
    0x11u8 as c_char, 0x11u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char, 0x11u8 as c_char, 0x11u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x11u8 as c_char, 0x11u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char,
    0x11u8 as c_char, 0x11u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char,
  ];
/// C: src/bitmaps/dimple3.xbm 1: `#define dimple3_width 16`
#[cfg (feature = "have_x_windows")]
pub const dimple3_width: c_int = 16;
/// C: src/bitmaps/dimple3.xbm 2: `#define dimple3_height 16`
#[cfg (feature = "have_x_windows")]
pub const dimple3_height: c_int = 16;

/// C 57: `#include "../src/bitmaps/gray1.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut gray1_bits: [c_char; 2] =          // C: src/bitmaps/gray1.xbm 3-4
  [
    0x01u8 as c_char, 0x02u8 as c_char,
  ];
/// C: src/bitmaps/gray1.xbm 1: `#define gray1_width 2`
#[cfg (feature = "have_x_windows")]
pub const gray1_width: c_int = 2;
/// C: src/bitmaps/gray1.xbm 2: `#define gray1_height 2`
#[cfg (feature = "have_x_windows")]
pub const gray1_height: c_int = 2;

/// C 58: `#include "../src/bitmaps/gray3.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut gray3_bits: [c_char; 4] =          // C: src/bitmaps/gray3.xbm 3-4
  [
    0x01u8 as c_char, 0x00u8 as c_char, 0x04u8 as c_char, 0x00u8 as c_char,
  ];
/// C: src/bitmaps/gray3.xbm 1: `#define gray3_width 4`
#[cfg (feature = "have_x_windows")]
pub const gray3_width: c_int = 4;
/// C: src/bitmaps/gray3.xbm 2: `#define gray3_height 4`
#[cfg (feature = "have_x_windows")]
pub const gray3_height: c_int = 4;

/// C 59: `#include "../src/bitmaps/crosswv.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut cross_weave_bits: [c_char; 32] =          // C: src/bitmaps/crosswv.xbm 3-6
  [
    0x55u8 as c_char, 0x55u8 as c_char, 0x88u8 as c_char, 0x88u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char,
    0x22u8 as c_char, 0x22u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char, 0x88u8 as c_char, 0x88u8 as c_char,
    0x55u8 as c_char, 0x55u8 as c_char, 0x22u8 as c_char, 0x22u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char,
    0x88u8 as c_char, 0x88u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char, 0x22u8 as c_char, 0x22u8 as c_char,
    0x55u8 as c_char, 0x55u8 as c_char, 0x88u8 as c_char, 0x88u8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char,
    0x22u8 as c_char, 0x22u8 as c_char,
  ];
/// C: src/bitmaps/crosswv.xbm 1: `#define cross_weave_width 16`
#[cfg (feature = "have_x_windows")]
pub const cross_weave_width: c_int = 16;
/// C: src/bitmaps/crosswv.xbm 2: `#define cross_weave_height 16`
#[cfg (feature = "have_x_windows")]
pub const cross_weave_height: c_int = 16;

/// C 61: `#include "../src/bitmaps/leftptr.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut left_ptr_bits: [c_char; 32] =          // C: src/bitmaps/leftptr.xbm 5-8
  [
    0x00u8 as c_char, 0x00u8 as c_char, 0x08u8 as c_char, 0x00u8 as c_char, 0x18u8 as c_char, 0x00u8 as c_char,
    0x38u8 as c_char, 0x00u8 as c_char, 0x78u8 as c_char, 0x00u8 as c_char, 0xf8u8 as c_char, 0x00u8 as c_char,
    0xf8u8 as c_char, 0x01u8 as c_char, 0xf8u8 as c_char, 0x03u8 as c_char, 0xf8u8 as c_char, 0x07u8 as c_char,
    0xf8u8 as c_char, 0x00u8 as c_char, 0xd8u8 as c_char, 0x00u8 as c_char, 0x88u8 as c_char, 0x01u8 as c_char,
    0x80u8 as c_char, 0x01u8 as c_char, 0x00u8 as c_char, 0x03u8 as c_char, 0x00u8 as c_char, 0x03u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char,
  ];
/// C: src/bitmaps/leftptr.xbm 1: `#define left_ptr_width 16`
#[cfg (feature = "have_x_windows")]
pub const left_ptr_width: c_int = 16;
/// C: src/bitmaps/leftptr.xbm 2: `#define left_ptr_height 16`
#[cfg (feature = "have_x_windows")]
pub const left_ptr_height: c_int = 16;
/// C: src/bitmaps/leftptr.xbm 3: `#define left_ptr_x_hot 3`
#[cfg (feature = "have_x_windows")]
pub const left_ptr_x_hot: c_int = 3;
/// C: src/bitmaps/leftptr.xbm 4: `#define left_ptr_y_hot 1`
#[cfg (feature = "have_x_windows")]
pub const left_ptr_y_hot: c_int = 1;

/// C 62: `#include "../src/bitmaps/leftpmsk.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut left_ptrmsk_bits: [c_char; 32] =          // C: src/bitmaps/leftpmsk.xbm 3-6
  [
    0x0cu8 as c_char, 0x00u8 as c_char, 0x1cu8 as c_char, 0x00u8 as c_char, 0x3cu8 as c_char, 0x00u8 as c_char,
    0x7cu8 as c_char, 0x00u8 as c_char, 0xfcu8 as c_char, 0x00u8 as c_char, 0xfcu8 as c_char, 0x01u8 as c_char,
    0xfcu8 as c_char, 0x03u8 as c_char, 0xfcu8 as c_char, 0x07u8 as c_char, 0xfcu8 as c_char, 0x0fu8 as c_char,
    0xfcu8 as c_char, 0x0fu8 as c_char, 0xfcu8 as c_char, 0x01u8 as c_char, 0xdcu8 as c_char, 0x03u8 as c_char,
    0xccu8 as c_char, 0x03u8 as c_char, 0x80u8 as c_char, 0x07u8 as c_char, 0x80u8 as c_char, 0x07u8 as c_char,
    0x00u8 as c_char, 0x03u8 as c_char,
  ];
/// C: src/bitmaps/leftpmsk.xbm 1: `#define left_ptrmsk_width 16`
#[cfg (feature = "have_x_windows")]
pub const left_ptrmsk_width: c_int = 16;
/// C: src/bitmaps/leftpmsk.xbm 2: `#define left_ptrmsk_height 16`
#[cfg (feature = "have_x_windows")]
pub const left_ptrmsk_height: c_int = 16;

/// C 63: `#include "../src/bitmaps/rtptr.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut right_ptr_bits: [c_char; 32] =          // C: src/bitmaps/rtptr.xbm 5-8
  [
    0x00u8 as c_char, 0x00u8 as c_char, 0x00u8 as c_char, 0x10u8 as c_char, 0x00u8 as c_char, 0x18u8 as c_char,
    0x00u8 as c_char, 0x1cu8 as c_char, 0x00u8 as c_char, 0x1eu8 as c_char, 0x00u8 as c_char, 0x1fu8 as c_char,
    0x80u8 as c_char, 0x1fu8 as c_char, 0xc0u8 as c_char, 0x1fu8 as c_char, 0xe0u8 as c_char, 0x1fu8 as c_char,
    0x00u8 as c_char, 0x1fu8 as c_char, 0x00u8 as c_char, 0x1bu8 as c_char, 0x80u8 as c_char, 0x11u8 as c_char,
    0x80u8 as c_char, 0x01u8 as c_char, 0xc0u8 as c_char, 0x00u8 as c_char, 0xc0u8 as c_char, 0x00u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char,
  ];
/// C: src/bitmaps/rtptr.xbm 1: `#define right_ptr_width 16`
#[cfg (feature = "have_x_windows")]
pub const right_ptr_width: c_int = 16;
/// C: src/bitmaps/rtptr.xbm 2: `#define right_ptr_height 16`
#[cfg (feature = "have_x_windows")]
pub const right_ptr_height: c_int = 16;
/// C: src/bitmaps/rtptr.xbm 3: `#define right_ptr_x_hot 12`
#[cfg (feature = "have_x_windows")]
pub const right_ptr_x_hot: c_int = 12;
/// C: src/bitmaps/rtptr.xbm 4: `#define right_ptr_y_hot 1`
#[cfg (feature = "have_x_windows")]
pub const right_ptr_y_hot: c_int = 1;

/// C 64: `#include "../src/bitmaps/rtpmsk.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut right_ptrmsk_bits: [c_char; 32] =          // C: src/bitmaps/rtpmsk.xbm 3-6
  [
    0x00u8 as c_char, 0x30u8 as c_char, 0x00u8 as c_char, 0x38u8 as c_char, 0x00u8 as c_char, 0x3cu8 as c_char,
    0x00u8 as c_char, 0x3eu8 as c_char, 0x00u8 as c_char, 0x3fu8 as c_char, 0x80u8 as c_char, 0x3fu8 as c_char,
    0xc0u8 as c_char, 0x3fu8 as c_char, 0xe0u8 as c_char, 0x3fu8 as c_char, 0xf0u8 as c_char, 0x3fu8 as c_char,
    0xf0u8 as c_char, 0x3fu8 as c_char, 0x80u8 as c_char, 0x3fu8 as c_char, 0xc0u8 as c_char, 0x3bu8 as c_char,
    0xc0u8 as c_char, 0x33u8 as c_char, 0xe0u8 as c_char, 0x01u8 as c_char, 0xe0u8 as c_char, 0x01u8 as c_char,
    0xc0u8 as c_char, 0x00u8 as c_char,
  ];
/// C: src/bitmaps/rtpmsk.xbm 1: `#define right_ptrmsk_width 16`
#[cfg (feature = "have_x_windows")]
pub const right_ptrmsk_width: c_int = 16;
/// C: src/bitmaps/rtpmsk.xbm 2: `#define right_ptrmsk_height 16`
#[cfg (feature = "have_x_windows")]
pub const right_ptrmsk_height: c_int = 16;

/// C 65: `#include "../src/bitmaps/cntrptr.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut cntr_ptr_bits: [c_char; 32] =          // C: src/bitmaps/cntrptr.xbm 5-8
  [
    0x00u8 as c_char, 0x00u8 as c_char, 0x80u8 as c_char, 0x01u8 as c_char, 0x80u8 as c_char, 0x01u8 as c_char,
    0xc0u8 as c_char, 0x03u8 as c_char, 0xc0u8 as c_char, 0x03u8 as c_char, 0xe0u8 as c_char, 0x07u8 as c_char,
    0xe0u8 as c_char, 0x07u8 as c_char, 0xf0u8 as c_char, 0x0fu8 as c_char, 0xf0u8 as c_char, 0x0fu8 as c_char,
    0x98u8 as c_char, 0x19u8 as c_char, 0x88u8 as c_char, 0x11u8 as c_char, 0x80u8 as c_char, 0x01u8 as c_char,
    0x80u8 as c_char, 0x01u8 as c_char, 0x80u8 as c_char, 0x01u8 as c_char, 0x80u8 as c_char, 0x01u8 as c_char,
    0x00u8 as c_char, 0x00u8 as c_char,
  ];
/// C: src/bitmaps/cntrptr.xbm 1: `#define cntr_ptr_width 16`
#[cfg (feature = "have_x_windows")]
pub const cntr_ptr_width: c_int = 16;
/// C: src/bitmaps/cntrptr.xbm 2: `#define cntr_ptr_height 16`
#[cfg (feature = "have_x_windows")]
pub const cntr_ptr_height: c_int = 16;
/// C: src/bitmaps/cntrptr.xbm 3: `#define cntr_ptr_x_hot 7`
#[cfg (feature = "have_x_windows")]
pub const cntr_ptr_x_hot: c_int = 7;
/// C: src/bitmaps/cntrptr.xbm 4: `#define cntr_ptr_y_hot 1`
#[cfg (feature = "have_x_windows")]
pub const cntr_ptr_y_hot: c_int = 1;

/// C 66: `#include "../src/bitmaps/cntrpmsk.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut cntr_ptrmsk_bits: [c_char; 32] =          // C: src/bitmaps/cntrpmsk.xbm 3-6
  [
    0xc0u8 as c_char, 0x03u8 as c_char, 0xc0u8 as c_char, 0x03u8 as c_char, 0xe0u8 as c_char, 0x07u8 as c_char,
    0xe0u8 as c_char, 0x07u8 as c_char, 0xf0u8 as c_char, 0x0fu8 as c_char, 0xf0u8 as c_char, 0x0fu8 as c_char,
    0xf8u8 as c_char, 0x1fu8 as c_char, 0xf8u8 as c_char, 0x1fu8 as c_char, 0xfcu8 as c_char, 0x3fu8 as c_char,
    0xfcu8 as c_char, 0x3fu8 as c_char, 0xfcu8 as c_char, 0x3fu8 as c_char, 0xdcu8 as c_char, 0x3bu8 as c_char,
    0xc0u8 as c_char, 0x03u8 as c_char, 0xc0u8 as c_char, 0x03u8 as c_char, 0xc0u8 as c_char, 0x03u8 as c_char,
    0xc0u8 as c_char, 0x03u8 as c_char,
  ];
/// C: src/bitmaps/cntrpmsk.xbm 1: `#define cntr_ptrmsk_width 16`
#[cfg (feature = "have_x_windows")]
pub const cntr_ptrmsk_width: c_int = 16;
/// C: src/bitmaps/cntrpmsk.xbm 2: `#define cntr_ptrmsk_height 16`
#[cfg (feature = "have_x_windows")]
pub const cntr_ptrmsk_height: c_int = 16;

/// C 67: `#include "../src/bitmaps/stipple.xbm"`.
#[cfg (feature = "have_x_windows")]
static mut stipple_bits: [c_char; 8] =          // C: src/bitmaps/stipple.xbm 3-4
  [
    0x55u8 as c_char, 0x55u8 as c_char, 0xeeu8 as c_char, 0xeeu8 as c_char, 0x55u8 as c_char, 0x55u8 as c_char,
    0xbau8 as c_char, 0xbbu8 as c_char,
  ];
/// C: src/bitmaps/stipple.xbm 1: `#define stipple_width 16`
#[cfg (feature = "have_x_windows")]
pub const stipple_width: c_int = 16;
/// C: src/bitmaps/stipple.xbm 2: `#define stipple_height 4`
#[cfg (feature = "have_x_windows")]
pub const stipple_height: c_int = 4;

/* C 87-106: the file's own defaults and the association-table size.  Each one is an object-like
   `#define` of the C text, so the port writes it as the constant C's name spells (PORTING.md
   0.1-2; the `oldXMenu/Internal.rs` 380-386 shape).  C's `DEF_P_EVENTS`/`DEF_S_EVENTS` are ORs of
   the mirror's `c_long` masks and their only use assigns to `int menu->p_events`/`s_events`
   (C 704/720), so the port's constants carry C's narrowing once, where C's use site performs it.  */
/// C 87: `#define DEF_FREEZE		0`
#[cfg (feature = "have_x_windows")]
pub const DEF_FREEZE: c_int = 0;
/// C 88: `#define DEF_REVERSE		0`
#[cfg (feature = "have_x_windows")]
pub const DEF_REVERSE: c_int = 0;
/// C 89: `#define DEF_MENU_STYLE		LEFT`
#[cfg (feature = "have_x_windows")]
pub const DEF_MENU_STYLE: XMStyle = LEFT;
/// C 90: `#define DEF_MENU_MODE		BOX`
#[cfg (feature = "have_x_windows")]
pub const DEF_MENU_MODE: XMMode = BOX;
/// C 91: `#define DEF_INACT_PNUM		3`
#[cfg (feature = "have_x_windows")]
pub const DEF_INACT_PNUM: c_int = 3;

/// C 93: `#define DEF_P_STYLE		CENTER`
#[cfg (feature = "have_x_windows")]
pub const DEF_P_STYLE: XMStyle = CENTER;

/// C 95: `#define DEF_P_EVENTS		(EnterWindowMask | ExposureMask)` - measured 16 | 32768.
#[cfg (feature = "have_x_windows")]
pub const DEF_P_EVENTS: c_int = (EnterWindowMask | ExposureMask) as c_int;
/// C 96: `#define DEF_P_FNT_NAME		"fixed"` - C's macro *is* the string literal, and the port
/// carries the literal itself (the `src/frame.rs` 6099-6101 `EMACS_CLASS` shape; the use site
/// calls `.as_ptr ()`, one spelling more than C's bare name).
#[cfg (feature = "have_x_windows")]
pub const DEF_P_FNT_NAME: &core::ffi::CStr = c"fixed";
/// C 97: `#define DEF_P_SPREAD		0.5`
#[cfg (feature = "have_x_windows")]
pub const DEF_P_SPREAD: f64 = 0.5;
/// C 98: `#define DEF_P_BDR_WIDTH		2`
#[cfg (feature = "have_x_windows")]
pub const DEF_P_BDR_WIDTH: c_int = 2;

/// C 100: `#define DEF_S_STYLE		LEFT`
#[cfg (feature = "have_x_windows")]
pub const DEF_S_STYLE: XMStyle = LEFT;
/// C 101: `#define DEF_S_EVENTS		(EnterWindowMask | LeaveWindowMask)` - measured 16 | 32.
#[cfg (feature = "have_x_windows")]
pub const DEF_S_EVENTS: c_int = (EnterWindowMask | LeaveWindowMask) as c_int;
/// C 102: `#define DEF_S_FNT_NAME		"fixed"` - the C 96 shape.
#[cfg (feature = "have_x_windows")]
pub const DEF_S_FNT_NAME: &core::ffi::CStr = c"fixed";
/// C 103: `#define DEF_S_SPREAD		0.10`
#[cfg (feature = "have_x_windows")]
pub const DEF_S_SPREAD: f64 = 0.10;
/// C 104: `#define DEF_S_BDR_WIDTH		1`
#[cfg (feature = "have_x_windows")]
pub const DEF_S_BDR_WIDTH: c_int = 1;

/// C 106: `#define XASSOC_TABLE_SIZE	64`
#[cfg (feature = "have_x_windows")]
pub const XASSOC_TABLE_SIZE: c_int = 64;

/* C 108: `const char *x_get_resource_string (char const *, char const *);` - the declaration this
   file carries for the resource lookup `src/frame.c` 6345 defines (its `frame.h` 1888 prototype is
   not reached here: this file includes no `frame.h`).  The definition is guarded by
   `HAVE_X_WINDOWS && !USE_X_TOOLKIT && !USE_GTK` in the reference, the same arm this file's own
   build sits in (`src/frame.rs` 6624-6631 carries it).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C 108: `const char *x_get_resource_string (char const *, char const *);`
  pub fn x_get_resource_string (attribute: *const c_char, class: *const c_char) -> *const c_char;
}

/// C 112-119: `static Status XAllocDisplayColor (Display *display, Colormap map, char const
/// *colorName, XColor *color, XColor *junk) { return (colorName!=0 && XParseColor (display, map,
/// colorName, color) && XAllocColor (display, map, color)); }` - the `&&` chain yields C's 1/0,
/// which the port spells `as c_int` (the `src/msdos.rs` 2627 shape); `static` keeps C's linkage,
/// so no `#[unsafe(no_mangle)]` (PORTING.md 0.2).
#[cfg (feature = "have_x_windows")]
unsafe fn XAllocDisplayColor (display: *mut Display, map: Colormap, colorName: *const c_char,
                              color: *mut XColor, junk: *mut XColor) -> Status
{
  return (colorName != core::ptr::null ()
          && XParseColor (display, map, colorName, color) != 0
          && XAllocColor (display, map, color) != 0) as c_int;
}

/// C 122-752: `XMenu *XMenuCreate (Display *display, Window parent, register char const
/// *def_env)`.  `register` is C's storage hint and has no Rust counterpart, so the declared type is
/// kept (PORTING.md 1.2); C 213 assigns to the parameter, so the binding is `mut` (a Rust
/// requirement with no C counterpart).
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XMenuCreate (display: *mut Display, parent: Window,
                                      mut def_env: *const c_char) -> *mut XMenu
{
  /* Xlib's `None` (X11/X.h 115's `0L`) is imported here rather than at the crate root: a
     crate-root `None` would shadow the prelude's for the whole unit (the `oldXMenu/Activate.rs`
     76-81 note).  C 740-744 is its five assignments.  */
  use x11_constants::None;

  let mut def_val: *const c_char = core::ptr::null (); /* X Default value temp variable. */

  let mut menu: *mut XMenu = core::ptr::null_mut ();   /* Pointer to the new menu. */
  let mut menu_style: XMStyle = 0;                     /* Menu display style. */
  let mut menu_mode: XMMode = 0;                       /* Menu display mode. */
  let mut pane: *mut XMPane = core::ptr::null_mut ();  /* Pane list header. */
  let mut assoc_tab: *mut XAssocTable = core::ptr::null_mut (); /* XAssocTable pointer. */

  let mut freeze: c_int = 0;                           /* Freeze server mode. */
  let mut reverse: c_int = 0;                          /* Reverse video mode. */

  let mut p_style: XMStyle = 0;                        /* Pane display style. */
  let mut p_fnt_name: *const c_char = core::ptr::null (); /* Flag font name. */
  let mut p_fnt_info: *mut XFontStruct = core::ptr::null_mut (); /* Flag font structure */
  let mut p_fnt_pad: c_int = 0;                        /* Flag font padding in pixels. */
  let mut p_spread: f64 = 0.0;                         /* Pane spread in flag height fractions. */
  let mut p_fnt_height: c_int = 0;                     /* Pane character height. */
  let mut p_bdr_width: c_int = 0;                      /* Pane border width. */
  let mut flag_height: c_int = 0;                      /* Flag window height. */
  let mut p_height: c_int = 0;                         /* Pane window height. */
  let mut p_x_off: c_int = 0;                          /* Pane X offset. */
  let mut p_y_off: c_int = 0;                          /* Pane Y offset. */
  let mut pane_GC: GC = core::ptr::null_mut ();        /* Pane graphics context. */

  let mut s_style: XMStyle = 0;                        /* Selection display style. */
  let mut s_fnt_name: *const c_char = core::ptr::null (); /* Selection font name. */
  let mut s_fnt_info: *mut XFontStruct = core::ptr::null_mut (); /* Selection font structure. */
  let mut s_fnt_pad: c_int = 0;                        /* Selection font padding in pixels. */
  let mut s_fnt_height: c_int = 0;                     /* Selection font character height */
  let mut s_spread: f64 = 0.0;                         /* Select spread in line height fractions. */
  let mut s_bdr_width: c_int = 0;                      /* Highlight border width. */
  let mut s_height: c_int = 0;                         /* Selection window height. */
  let mut s_x_off: c_int = 0;                          /* Selection window X offset. */
  let mut s_y_off: c_int = 0;                          /* Selection window Y offset. */
  let mut normal_select_GC: GC = core::ptr::null_mut (); /* GC used for normal video selection. */
  let mut inverse_select_GC: GC = core::ptr::null_mut (); /* GC used for inverse video selection. */
  let mut inact_GC: GC = core::ptr::null_mut ();       /* GC for inactive pane header and */
                                                       /* selections. */

  let mut color_def: XColor = core::mem::zeroed ();    /* Temp color definition holder. */
  let mut p_bdr_color: XColor = core::mem::zeroed ();  /* Color of border. */
  let mut s_bdr_color: XColor = core::mem::zeroed ();  /* Color of highlight. */
  let mut p_frg_color: XColor = core::mem::zeroed ();  /* Color of pane foreground color. */
  let mut s_frg_color: XColor = core::mem::zeroed ();  /* Color of selection foreground. */
  let mut bkgnd_color: XColor = core::mem::zeroed ();  /* Color of background.. */
  let mut mouse_color: XColor = core::mem::zeroed ();  /* Color of mouse cursor. */
  let mut mouse_cursor: Cursor = 0;                    /* Mouse cursor. */
  let mut inact_bitmap: Pixmap = 0;                    /* Menu inactive pixmap. */

  let mut inact_pnum: c_int = 0;                       /* Inactive background pattern number. */

  let mut cursor: Pixmap = 0;                          /* Cursor pixmap holder. */
  let mut cursor_mask: Pixmap = 0;                     /* Cursor mask pixmap holder. */
  let mut stipple_pixmap: Pixmap = 0;                  /* Stipple mask for half-tone text. */
  let mut valuemask: c_ulong = 0;
  let mut values: *mut XGCValues = core::ptr::null_mut ();

  let root: Window = RootWindow (display, DefaultScreen (display));

  /* C 187-189: "Calloc the XMenu structure and the initial pane."  */
  menu = calloc (1, core::mem::size_of::<XMenu> ()) as *mut XMenu;
  if menu.is_null ()
    {
      _XMErrorCode = XME_CALLOC;
      return core::ptr::null_mut ();
    }
  pane = calloc (1, core::mem::size_of::<XMPane> ()) as *mut XMPane;
  if pane.is_null ()
    {
      _XMErrorCode = XME_CALLOC;
      return core::ptr::null_mut ();
    }

  /* C 201-203: "Create the XAssocTable" */
  assoc_tab = XCreateAssocTable (XASSOC_TABLE_SIZE);
  if assoc_tab.is_null ()
    {
      _XMErrorCode = XME_CREATE_ASSOC;
      return core::ptr::null_mut ();
    }

  /* C 210-212: "Set up the default environment name."  */
  if def_env.is_null () || *def_env == 0 { def_env = c"XMenu".as_ptr (); }

  /* C 215-217: "Set up internal fail-safe defaults."  */
  freeze = DEF_FREEZE;
  reverse = DEF_REVERSE;
  menu_style = DEF_MENU_STYLE;
  menu_mode = DEF_MENU_MODE;
  inact_pnum = DEF_INACT_PNUM;

  p_style = DEF_P_STYLE;
  p_spread = DEF_P_SPREAD;
  p_fnt_name = DEF_P_FNT_NAME.as_ptr ();
  p_bdr_width = DEF_P_BDR_WIDTH;

  s_style = DEF_S_STYLE;
  s_spread = DEF_S_SPREAD;
  s_fnt_name = DEF_S_FNT_NAME.as_ptr ();
  s_bdr_width = DEF_S_BDR_WIDTH;

  /* C 234-236: "Get default values from X."  */
  def_val = x_get_resource_string (c"menuFreeze".as_ptr (), c"MenuFreeze".as_ptr ());
  if !def_val.is_null ()
    {
      if strcmp (def_val, c"on".as_ptr ()) == 0 { freeze = 1; }
      else if strcmp (def_val, c"off".as_ptr ()) == 0 { freeze = 0; }
    }

  def_val = x_get_resource_string (c"menuReverseVideo".as_ptr (), c"MenuReverseVideo".as_ptr ());
  if !def_val.is_null ()
    {
      if strcmp (def_val, c"on".as_ptr ()) == 0 { reverse = 1; }
      else if strcmp (def_val, c"off".as_ptr ()) == 0 { reverse = 0; }
    }

  def_val = x_get_resource_string (c"menuStyle".as_ptr (), c"MenuStyle".as_ptr ());
  if !def_val.is_null ()
    {
      if strcmp (def_val, c"right_hand".as_ptr ()) == 0 { menu_style = RIGHT; }
      else if strcmp (def_val, c"left_hand".as_ptr ()) == 0 { menu_style = LEFT; }
      else if strcmp (def_val, c"center".as_ptr ()) == 0 { menu_style = CENTER; }
    }

  def_val = x_get_resource_string (c"menuMode".as_ptr (), c"MenuMode".as_ptr ());
  if !def_val.is_null ()
    {
      if strcmp (def_val, c"box".as_ptr ()) == 0 { menu_mode = BOX; }
      else if strcmp (def_val, c"invert".as_ptr ()) == 0 { menu_mode = INVERT; }
    }

  def_val = x_get_resource_string (c"menuMouse".as_ptr (), c"MenuMouse".as_ptr ());
  if !def_val.is_null ()
     && DisplayCells (display, DefaultScreen (display)) > 2
     && XAllocDisplayColor (display,
                            DefaultColormap (display, DefaultScreen (display)),
                            def_val,
                            &raw mut mouse_color, &raw mut color_def) != 0
    { }

  else if reverse != 0
          && XAllocDisplayColor (display,
                                 DefaultColormap (display, DefaultScreen (display)),
                                 c"white".as_ptr (),
                                 &raw mut mouse_color, &raw mut color_def) != 0
    { }

  else if XAllocDisplayColor (display,
                              DefaultColormap (display, DefaultScreen (display)),
                              c"black".as_ptr (),
                              &raw mut mouse_color, &raw mut color_def) != 0
    { }

  else { }

  def_val = x_get_resource_string (c"menuBackground".as_ptr (), c"MenuBackground".as_ptr ());
  if !def_val.is_null ()
     && DisplayCells (display, DefaultScreen (display)) > 2
     && XAllocDisplayColor (display,
                            DefaultColormap (display, DefaultScreen (display)),
                            def_val,
                            &raw mut bkgnd_color, &raw mut color_def) != 0
    { }
  else if reverse != 0
          && XAllocDisplayColor (display,
                                 DefaultColormap (display, DefaultScreen (display)),
                                 c"black".as_ptr (),
                                 &raw mut bkgnd_color, &raw mut color_def) != 0
    { }
  else if XAllocDisplayColor (display,
                              DefaultColormap (display, DefaultScreen (display)),
                              c"white".as_ptr (),
                              &raw mut bkgnd_color, &raw mut color_def) != 0
    { }
  else { }

  def_val = x_get_resource_string (c"menuInactivePattern".as_ptr (),
                                  c"MenuInactivePattern".as_ptr ());
  if !def_val.is_null ()
    {
      if strcmp (def_val, c"dimple1".as_ptr ()) == 0 { inact_pnum = 0; }
      else if strcmp (def_val, c"dimple3".as_ptr ()) == 0 { inact_pnum = 1; }
      else if strcmp (def_val, c"gray1".as_ptr ()) == 0 { inact_pnum = 2; }
      else if strcmp (def_val, c"gray3".as_ptr ()) == 0 { inact_pnum = 3; }
      else if strcmp (def_val, c"cross_weave".as_ptr ()) == 0 { inact_pnum = 4; }
    }

  def_val = x_get_resource_string (c"paneStyle".as_ptr (), c"PaneStyle".as_ptr ());
  if !def_val.is_null ()
    {
      if strcmp (def_val, c"flush_left".as_ptr ()) == 0 { p_style = LEFT; }
      else if strcmp (def_val, c"flush_right".as_ptr ()) == 0 { p_style = RIGHT; }
      else if strcmp (def_val, c"center".as_ptr ()) == 0 { p_style = CENTER; }
    }

  def_val = x_get_resource_string (c"paneFont".as_ptr (), c"PaneFont".as_ptr ());
  if !def_val.is_null () { p_fnt_name = def_val; }

  def_val = x_get_resource_string (c"paneForeground".as_ptr (), c"PaneForeground".as_ptr ());
  if !def_val.is_null ()
     && DisplayCells (display, DefaultScreen (display)) > 2
    { XAllocDisplayColor (display,
                          DefaultColormap (display, DefaultScreen (display)),
                          def_val,
                          &raw mut p_frg_color, &raw mut color_def); }

  else if reverse != 0
    { XAllocDisplayColor (display,
                          DefaultColormap (display, DefaultScreen (display)),
                          c"white".as_ptr (),
                          &raw mut p_frg_color, &raw mut color_def); }
  else
    { XAllocDisplayColor (display,
                          DefaultColormap (display, DefaultScreen (display)),
                          c"black".as_ptr (),
                          &raw mut p_frg_color, &raw mut color_def); }

  def_val = x_get_resource_string (c"paneBorder".as_ptr (), c"PaneBorder".as_ptr ());
  if !def_val.is_null ()
     && DisplayCells (display, DefaultScreen (display)) > 2
     && XAllocDisplayColor (display,
                            DefaultColormap (display, DefaultScreen (display)),
                            def_val,
                            &raw mut p_bdr_color, &raw mut color_def) != 0
    { }
  else if reverse != 0
          && XAllocDisplayColor (display,
                                 DefaultColormap (display, DefaultScreen (display)),
                                 c"white".as_ptr (),
                                 &raw mut p_bdr_color, &raw mut color_def) != 0
    { }
  else
    { XAllocDisplayColor (display,
                          DefaultColormap (display, DefaultScreen (display)),
                          c"black".as_ptr (),
                          &raw mut p_bdr_color, &raw mut color_def); }

  def_val = x_get_resource_string (c"paneBorderWidth".as_ptr (), c"PaneBorderWidth".as_ptr ());
  if !def_val.is_null () { p_bdr_width = atoi (def_val); }

  def_val = x_get_resource_string (c"paneSpread".as_ptr (), c"PaneSpread".as_ptr ());
  if !def_val.is_null () { p_spread = atof (def_val); }

  def_val = x_get_resource_string (c"selectionStyle".as_ptr (), c"SelectionStyle".as_ptr ());
  if !def_val.is_null ()
    {
      if strcmp (def_val, c"flush_left".as_ptr ()) == 0 { s_style = LEFT; }
      else if strcmp (def_val, c"flush_right".as_ptr ()) == 0 { s_style = RIGHT; }
      else if strcmp (def_val, c"center".as_ptr ()) == 0 { s_style = CENTER; }
    }

  def_val = x_get_resource_string (c"selectionFont".as_ptr (), c"SelectionFont".as_ptr ());
  if !def_val.is_null () { s_fnt_name = def_val; }

  def_val = x_get_resource_string (c"selectionForeground".as_ptr (),
                                  c"SelectionForeground".as_ptr ());
  if !def_val.is_null ()
     && DisplayCells (display, DefaultScreen (display)) > 2
     && XAllocDisplayColor (display,
                            DefaultColormap (display, DefaultScreen (display)),
                            def_val,
                            &raw mut s_frg_color, &raw mut color_def) != 0
    { }
  else if reverse != 0
          && XAllocDisplayColor (display,
                                 DefaultColormap (display, DefaultScreen (display)),
                                 c"white".as_ptr (),
                                 &raw mut s_frg_color, &raw mut color_def) != 0
    { }
  else if XAllocDisplayColor (display,
                              DefaultColormap (display, DefaultScreen (display)),
                              c"black".as_ptr (),
                              &raw mut s_frg_color, &raw mut color_def) != 0
    { }
  else { }

  def_val = x_get_resource_string (c"selectionBorder".as_ptr (), c"SelectionBorder".as_ptr ());
  if !def_val.is_null ()
     && DisplayCells (display, DefaultScreen (display)) > 2
     && XAllocDisplayColor (display,
                            DefaultColormap (display, DefaultScreen (display)),
                            def_val,
                            &raw mut s_bdr_color, &raw mut color_def) != 0
    { }
  else if reverse != 0
          && XAllocDisplayColor (display,
                                 DefaultColormap (display, DefaultScreen (display)),
                                 c"white".as_ptr (),
                                 &raw mut s_bdr_color, &raw mut color_def) != 0
    { }
  else if XAllocDisplayColor (display,
                              DefaultColormap (display, DefaultScreen (display)),
                              c"black".as_ptr (),
                              &raw mut s_bdr_color, &raw mut color_def) != 0
    { }
  else { }

  def_val = x_get_resource_string (c"selectionBorderWidth".as_ptr (),
                                  c"SelectionBorderWidth".as_ptr ());
  if !def_val.is_null () { s_bdr_width = atoi (def_val); }

  def_val = x_get_resource_string (c"selectionSpread".as_ptr (), c"SelectionSpread".as_ptr ());
  if !def_val.is_null () { s_spread = atof (def_val); }

  /* C 434-436: "Create and store the inactive pattern pixmap."  C 437-485's braced block is the
     port's block, with its two locals and C's `switch (inact_pnum)` as a `match` (C's five `case`s
     are the five arms and C's absent `default:` is the `_` arm; PORTING.md 2.2, the ledger's
     `## Deviations` 6).  */
  {
    let mut data: *mut c_char = core::ptr::null_mut ();
    let mut width: c_int = 0;
    let mut height: c_int = 0;

    match inact_pnum
      {
      0 =>
        {
          data = &raw mut dimple1_bits as *mut c_char;
          width = dimple1_width;
          height = dimple1_height;
        }

      1 =>
        {
          data = &raw mut dimple3_bits as *mut c_char;
          width = dimple3_width;
          height = dimple3_height;
        }

      2 =>
        {
          data = &raw mut gray1_bits as *mut c_char;
          width = gray1_width;
          height = gray1_height;
        }

      3 =>
        {
          data = &raw mut gray3_bits as *mut c_char;
          width = gray3_width;
          height = gray3_height;
        }

      4 =>
        {
          data = &raw mut cross_weave_bits as *mut c_char;
          width = cross_weave_width;
          height = cross_weave_height;
        }
      _ => { }
      }

    if data.is_null ()
      {
        _XMErrorCode = XME_STORE_BITMAP;
        return core::ptr::null_mut ();
      }

    inact_bitmap =
      XCreatePixmapFromBitmapData
        (display, root, data, width as c_uint, height as c_uint,
         p_frg_color.pixel, bkgnd_color.pixel,
         DisplayPlanes (display, DefaultScreen (display)) as c_uint);
  }

  /* C 487-490: "Load the mouse cursor."  C 491-559's `switch (menu_style)` is the port's `match`,
     its three labels the three constant patterns and C 555-558's `default:` the `_` arm
     (the ledger's `## Deviations` 6).  */
  match menu_style
    {
    LEFT =>
      {
        cursor = XCreateBitmapFromData (display,
                                        root,
                                        &raw const left_ptr_bits as *const c_char,
                                        left_ptr_width as c_uint,
                                        left_ptr_height as c_uint);
        cursor_mask = XCreateBitmapFromData (display,
                                             root,
                                             &raw const left_ptrmsk_bits as *const c_char,
                                             left_ptrmsk_width as c_uint,
                                             left_ptrmsk_height as c_uint);
        mouse_cursor = XCreatePixmapCursor
                         (display,
                          cursor, cursor_mask,
                          &raw mut mouse_color, &raw mut bkgnd_color,
                          left_ptr_x_hot as c_uint,
                          left_ptr_y_hot as c_uint);
        XFreePixmap (display, cursor);
        XFreePixmap (display, cursor_mask);
      }
    RIGHT =>
      {
        cursor = XCreateBitmapFromData (display,
                                        root,
                                        &raw const right_ptr_bits as *const c_char,
                                        right_ptr_width as c_uint,
                                        right_ptr_height as c_uint);
        cursor_mask = XCreateBitmapFromData (display,
                                             root,
                                             &raw const right_ptrmsk_bits as *const c_char,
                                             right_ptrmsk_width as c_uint,
                                             right_ptrmsk_height as c_uint);
        mouse_cursor = XCreatePixmapCursor
                         (display,
                          cursor, cursor_mask,
                          &raw mut mouse_color, &raw mut bkgnd_color,
                          right_ptr_x_hot as c_uint,
                          right_ptr_y_hot as c_uint);
        XFreePixmap (display, cursor);
        XFreePixmap (display, cursor_mask);
      }
    CENTER =>
      {
        cursor = XCreateBitmapFromData (display,
                                        root,
                                        &raw const cntr_ptr_bits as *const c_char,
                                        cntr_ptr_width as c_uint,
                                        cntr_ptr_height as c_uint);
        cursor_mask = XCreateBitmapFromData (display,
                                             root,
                                             &raw const cntr_ptrmsk_bits as *const c_char,
                                             cntr_ptrmsk_width as c_uint,
                                             cntr_ptrmsk_height as c_uint);
        mouse_cursor = XCreatePixmapCursor
                         (display,
                          cursor, cursor_mask,
                          &raw mut mouse_color, &raw mut bkgnd_color,
                          cntr_ptr_x_hot as c_uint,
                          cntr_ptr_y_hot as c_uint);
        XFreePixmap (display, cursor);
        XFreePixmap (display, cursor_mask);
      }
    _ =>
      {
        /* Error! Invalid style parameter.  */
        _XMErrorCode = XME_STYLE_PARAM;
        return core::ptr::null_mut ();
      }
    }
  if mouse_cursor == _X_FAILURE as Cursor
    {
      _XMErrorCode = XME_CREATE_CURSOR;
      return core::ptr::null_mut ();
    }

  /* C 565-567: "Open the pane and selection fonts."  */

  p_fnt_info = XLoadQueryFont (display, p_fnt_name);
  if p_fnt_info.is_null ()
    {
      _XMErrorCode = XME_OPEN_FONT;
      return core::ptr::null_mut ();

    }

  s_fnt_info = XLoadQueryFont (display, s_fnt_name);
  if s_fnt_info.is_null ()
    {
      _XMErrorCode = XME_OPEN_FONT;
      return core::ptr::null_mut ();
    }
  /* C 581-583: "Calculate the fixed padding value in pixels for each font."  */
  p_fnt_height = (*p_fnt_info).max_bounds.ascent as c_int
                 + (*p_fnt_info).max_bounds.descent as c_int;
  s_fnt_height = (*s_fnt_info).max_bounds.ascent as c_int
                 + (*s_fnt_info).max_bounds.descent as c_int;
  p_fnt_pad = (s_spread * p_fnt_height as f64) as c_int;
  s_fnt_pad = (s_spread * s_fnt_height as f64) as c_int;

  /* C 589-591: "Calculate fixed height and offset requirements."  */
  flag_height = p_fnt_height + (p_fnt_pad << 1);

  p_height = 0;
  p_y_off = flag_height + p_bdr_width;
  p_x_off = (p_y_off as f64 * p_spread) as c_int;

  s_height = s_fnt_height + (s_fnt_pad << 1) + (s_bdr_width << 1);
  s_y_off = s_height;
  s_x_off = p_x_off;

  /* C 602-604: "Set up the pane list header."  */
  (*pane).next = pane;
  (*pane).prev = pane;
  (*pane).r#type = PL_HEADER;
  (*pane).serial = -1;

  /* C 610-612: "Initialize the internal pane and selection creation queues."  */
  _XMWinQueInit ();

  /* C 615-617: "Create pane, active, and inactive GC's."  */
  values = malloc (core::mem::size_of::<XGCValues> ()) as *mut XGCValues;
  if values.is_null ()
    { return core::ptr::null_mut (); }
  valuemask = (GCForeground | GCBackground | GCFont | GCLineWidth) as c_ulong;

  /* C 623-625: "First, pane."  */

  (*values).foreground = p_frg_color.pixel;
  (*values).background = bkgnd_color.pixel;
  (*values).font = (*p_fnt_info).fid;
  (*values).line_width = p_bdr_width;

  pane_GC = XCreateGC (display, root, valuemask, values as *mut c_void);
  /* C 637-639: "Then normal video selection."  */

  (*values).foreground = s_frg_color.pixel;
  (*values).background = bkgnd_color.pixel;
  (*values).font = (*s_fnt_info).fid;
  (*values).line_width = s_bdr_width;
  normal_select_GC = XCreateGC (display, root, valuemask, values as *mut c_void);
  /* C 649-651: "Inverse video selection."  */

  (*values).foreground = bkgnd_color.pixel;
  (*values).background = s_frg_color.pixel;
  (*values).font = (*s_fnt_info).fid;
  (*values).line_width = s_bdr_width;
  inverse_select_GC = XCreateGC (display, root, valuemask, values as *mut c_void);
  stipple_pixmap = XCreateBitmapFromData (display,
                                          root,
                                          &raw const stipple_bits as *const c_char,
                                          stipple_width as c_uint,
                                          stipple_height as c_uint);

  /* C 667-669: "Finally, inactive pane header and selections"  */
  valuemask |= (GCFillStyle | GCStipple) as c_ulong;
  (*values).foreground = s_frg_color.pixel;
  (*values).background = bkgnd_color.pixel;
  (*values).font = (*s_fnt_info).fid;
  (*values).line_width = s_bdr_width;
  (*values).fill_style = FillStippled;
  (*values).stipple = stipple_pixmap;

  inact_GC = XCreateGC (display, root, valuemask, values as *mut c_void);

  valuemask |= (GCGraphicsExposures) as c_ulong;
  (*values).graphics_exposures = False;

  /* C 687-689: "Construct the XMenu object."  */
  /* -------------------- Menu data -------------------- */
  (*menu).menu_style = menu_style;
  (*menu).menu_mode = menu_mode;
  (*menu).freeze = freeze;
  (*menu).aeq = 0;
  (*menu).recompute = 1;
  (*menu).parent = parent;
  (*menu).height = 0;
  (*menu).width = 0;
  (*menu).mouse_cursor = mouse_cursor;
  (*menu).assoc_tab = assoc_tab;
  (*menu).p_list = pane;
  /* -------------------- Pane window data -------------------- */
  (*menu).p_style = p_style;
  (*menu).p_events = DEF_P_EVENTS;
  (*menu).p_fnt_info = p_fnt_info;
  (*menu).p_fnt_pad = p_fnt_pad;
  (*menu).p_spread = p_spread;
  (*menu).p_bdr_width = p_bdr_width;
  (*menu).flag_height = flag_height;
  (*menu).p_width = 0;
  (*menu).p_height = p_height;
  (*menu).p_x_off = p_x_off;
  (*menu).p_y_off = p_y_off;
  (*menu).p_count = 0;
  (*menu).pane_GC = pane_GC;
  (*menu).x_pos = 0;
  (*menu).y_pos = 0;
  /* -------------------- Selection window data -------------------- */
  (*menu).s_style = s_style;
  (*menu).s_events = DEF_S_EVENTS;
  (*menu).s_fnt_info = s_fnt_info;
  (*menu).s_fnt_pad = s_fnt_pad;
  (*menu).s_spread = s_spread;
  (*menu).s_bdr_width = s_bdr_width; /* unnecessary */
  (*menu).s_width = 0;
  (*menu).s_height = s_height;
  (*menu).s_x_off = s_x_off;
  (*menu).s_y_off = s_y_off;
  (*menu).s_count = 0;
  (*menu).normal_select_GC = normal_select_GC;
  (*menu).inverse_select_GC = inverse_select_GC;
  (*menu).inact_GC = inact_GC;
  /* -------------------- Color data -------------------- */
  (*menu).p_bdr_color = p_bdr_color.pixel;
  (*menu).s_bdr_color = s_bdr_color.pixel;
  (*menu).p_frg_color = p_frg_color.pixel;
  (*menu).s_frg_color = s_frg_color.pixel;
  (*menu).bkgnd_color = bkgnd_color.pixel;
  /* -------------------- Pixmap data -------------------- */
  (*menu).p_bdr_pixmap = None as Pixmap;
  (*menu).s_bdr_pixmap = None as Pixmap;
  (*menu).p_frg_pixmap = None as Pixmap;
  (*menu).s_frg_pixmap = None as Pixmap;
  (*menu).bkgnd_pixmap = None as Pixmap;
  (*menu).inact_pixmap = inact_bitmap;

  /* C 747-749: "Return the completed XMenu."  */
  _XMErrorCode = XME_NO_ERROR;
  return menu;
}
