// femacs: port of src/xftfont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The C non-`static` definitions carry `#[unsafe(no_mangle)]` and `extern "C"`
// (PORTING.md 0.2) - `syms_of_xftfont`, `xftfont_driver` and `xfthbfont_driver`; the C `static`
// helpers stay plain `unsafe fn`.
//
// **`src/xftfont.c` is a P-86 unit**: the frozen configuration compiles nothing from it.  Its
// object belongs to `FONT_OBJ`, and `configure.ac` 7194-7203 fills that variable with
// `ftfont.o xftfont.o` only inside the `HAVE_X_WINDOWS` branch under `HAVE_CAIRO`, `HAVE_XFT` or
// `HAVE_FREETYPE` (the `#elif test "$HAVE_XFT" = "yes"` arm is the one that names this file);
// `HAVE_XFT` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1696, `FONT_OBJ=` is
// empty in `build/emacs-ref/src/Makefile` 342, and `docs/baseline/frozen-build-sources.txt` lists
// no `xftfont.o`.  P-86 (1) therefore applies: this is *transcription*, both arms of every `#if`
// are carried, and the file gate `have_xft` goes on every item **and on the include graph**
// (P-86 (8) - an include outside the gate fails the frozen build at once, which
// `build-tu src/xftfont.c`'s 336-byte empty object measures).  `HAVE_XFT=yes` can only be reached
// from the `HAVE_X_WINDOWS=yes` branch, so `have_x_windows` is implied by this file's gate and the
// mirror of `ftfont.h` (whose `struct font_info` is what C 217 casts to) is itself declared behind
// it; `have_window_system` is implied in turn, because `struct font`'s eight window-system members
// and `font_driver`'s `draw`/`get_bitmap` are behind that macro and C 208-320 writes them all.
//
// `src/xftfont.c` is *not* in `docs/baseline/config/cond-eval-verified.txt`, so `extract-abi.py`
// reads its `#if`s blind (P-66): every arm below is carried as text and its C line is named in the
// ledger `docs/ports/xftfont.md`, whose `## C line ranges` table is P-86 (2)'s substitute for a
// checklist this file's frozen inventory cannot give.  The gates, in C's order:
//
//   * C 36-38 `#ifdef HAVE_XRENDER` (the `<X11/extensions/Xrender.h>` include) and C 546's
//     `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 || (RENDER_MINOR >= 2))` are
//     `feature = "have_xrender"` and the derived pair ( `feature = "have_xrender"` +
//     `feature = "render_version_ge_0_2"` ), the `xfont.rs` 1006 shape: no `#[cfg]` can read
//     `RENDER_MAJOR` out of the header, so the version half is its own registered name.
//   * C 40-43 `#ifndef FC_LCD_FILTER` is the header's own fallback definition of a string macro;
//     the port declares the name once in the Fontconfig segment below (C 24's chain), the
//     `ftfont.rs` 1235-1238 shape.
//   * C 209-215 / C 285-287 / C 312-314 / C 354-360 / C 713-746 / C 769-770 / C 785-787 /
//     C 793-796 / C 820-830 `#ifdef HAVE_HARFBUZZ` are `feature = "harfbuzz"` (the registered
//     spelling of `HAVE_HARFBUZZ`, the `ftfont.rs` 50-52 reading).
//   * C 308-311 / C 347-353 `#ifdef HAVE_LIBOTF` are `feature = "have_libotf"`; C 612 / C 772's
//     `#if defined HAVE_M17N_FLT && defined HAVE_LIBOTF` is `all (feature = "have_m17n_flt",
//     feature = "have_libotf")`.
//   * C 247-249 `#ifdef FC_DUAL` and C 250-252 `#ifdef FC_CHARCELL` are Fontconfig's own header
//     macros (`fc_dual`, `fc_charcell`).
//   * C 321-336 `#ifdef FT_BDF_H` is `feature = "ft_bdf_h"` (FreeType's own header macro, the
//     `ft_load_target_mono` shape).
//   * C 655-666 / C 781-783 `#ifdef HAVE_XDBE` are `feature = "have_xdbe"`.
//   * C 692-696 `#ifdef FC_EMBOLDEN` is `feature = "fc_embolden"`, and C 775-777's
//     `#if defined HAVE_OTF_GET_VARIATION_GLYPHS || defined HAVE_FT_FACE_GETCHARVARIANTINDEX` is
//     `any (feature = "have_otf_get_variation_glyphs", feature = "have_ft_face_getcharvariantindex")`.
//   * C 380-387 and C 403-408's `#if false` is dead in every configuration, so it is recorded at
//     its place rather than transcribed (P-08; the `ftfont.c` C 942-968 `#if 0` shape).
//
// The external SDK surfaces stand in the unit, at the C `#include` lines (P-88 (1)):
// `<X11/Xlib.h>`, `<X11/Xft/Xft.h>` (which pulls in `<fontconfig/fontconfig.h>` and
// `<X11/extensions/Xrender.h>` - its own lines 42-43), `<X11/extensions/Xrender.h>` again under
// C 36 and `FT_FREETYPE_H` through `ftfont.h`.  None of those headers has a file in the reference
// tree, so each name C 1-832 uses is declared here with its header's own line, and every record
// layout is a clang reading: `<X11/Xlib.h>`/`<X11/extensions/Xrender.h>`/`<fontconfig/…>`/
// `<ft2build.h>` are this machine's own (`/opt/homebrew/include`, `xorgproto`/`libx11`, `libXrender`,
// Fontconfig 2.17.1, FreeType 2.14.3), while `<X11/Xft/Xft.h>` is **not installed here**
// (`/opt/homebrew/include/X11/Xft` is absent): the three Xft records are measured against the
// libXft 2.3.6 release's own `Xft.h`/`XftCompat.h` (the version C 546's `XFT_MAJOR`/`XFT_MINOR`
// test is about) plus the machine's Xrender/Fontconfig/FreeType headers, and the ledger records the
// provenance and the missing toolchain.  `<otf.h>` (libotf) has no copy here at all: the two
// libotf names this unit needs (`OTF`, `OTF_close`) are declared from their published prototypes
// and marked in the ledger, because they are opaque pointers with no layout to measure (P-88 (2)).
// All four surfaces have two or more consumers in the tree (`src/ftfont.c`, `src/ftfont.h`,
// `src/xftfont.c`, `src/ftcrfont.c`; Xlib also `src/xfont.c`, `src/xterm.c` …), so P-90 (1) wants a
// dedicated mirror each (`include/xft.rs`, `include/fontconfig.rs`, `include/freetype.rs`,
// `include/xrender.rs`, plus the `<X11/Xlib.h>` names into `include/xlib.rs`): the ledger's `## Owed`
// names the moves, and when they land this segment disappears in the same commit (P-88 (5) /
// P-90 (3)).
//
// `struct face` (C 72-97, C 390-395, C 410-426, C 505-516) is carried in the unit the way
// `xfont.rs`'s and `dispnew.rs`'s copies are (P-72 rule 3): `include/dispextern.rs`'s record is
// behind `dispextern_face_records` (status 3.220) and so is not available to a unit compiled on
// its own.  C reads `gc`, `foreground`, `background`, `font` and writes `extra`, so only those
// offsets are pinned (clang, the ledger's `## Evidence`).
//
// Names the mirrors this unit includes do not carry are declared at the use site with the
// reference header's line - one citation per line (`font.h`, `xterm.h`; `XftDrawSetClipRectangles`
// and the rest keep the SDK header's line).  The ledger's `## Owed` names each and the file it
// belongs in.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

use std::ffi::CStr;                            // the FC_* constants of the Xft segment (C 223-704)

include!("include/config.rs");                 // C 22: `#include <config.h>` (every unit includes it)

/* C 23-35: the file's own include list.  It sits outside every `#if` of the file body, so with the
   file gate on it (P-86 (8)) the frozen configuration compiles not one line of this unit - the
   reading the ledger's Evidence 1 records.  C 36-38's `<X11/extensions/Xrender.h>` carries
   `have_xrender` with the rest of the file's own gate (`HAVE_XFT` implies `HAVE_XRENDER` on the
   platforms that have it; the macro is still C's own, so the arm keeps its own feature).  */
#[cfg (feature = "have_xft")]
include!("include/xlib.rs");                   // C 23: `#include <X11/Xlib.h>`
#[cfg (feature = "have_xft")]
include!("include/lisp.rs");                   // C 26: `#include "lisp.h"`
#[cfg (feature = "have_xft")]
include!("include/xterm.rs");                  // C 27: `#include "xterm.h"` (brings `dispextern.rs`)
#[cfg (feature = "have_xft")]
include!("include/frame.rs");                  // C 28: `#include "frame.h"`
#[cfg (feature = "have_xft")]
include!("include/blockinput.rs");             // C 29: `#include "blockinput.h"`
#[cfg (feature = "have_xft")]
include!("include/charset.rs");                // C 30: `#include "charset.h"`
#[cfg (feature = "have_xft")]
include!("include/composite.rs");              // C 31: `#include "composite.h"` (P-72 rule 4)
#[cfg (feature = "have_xft")]
include!("include/font.rs");                   // C 32: `#include "font.h"`
#[cfg (feature = "have_xft")]
include!("include/ftfont.rs");                 // C 33: `#include "ftfont.h"` (`struct font_info`)
#[cfg (feature = "have_xft")]
include!("include/pdumper.rs");                // C 34: `#include "pdumper.h"`

/* ----------------------------------------------------------------------------------------------
   C 23: the `<X11/Xlib.h>` surface this unit names and `include/xlib.rs` does not carry - the GC
   value record and its accessor, plus the two value-mask bits C 85 tests.  (Xlib.h 566-567's
   `GCForeground`/`GCBackground` live in `<X11/X.h>`, which `<X11/Xlib.h>` includes.)  Every number
   is a clang reading of this machine's headers (`xorgproto`/`libx11` under `/opt/homebrew/include`;
   the probe and its verbatim output are in the ledger's `## Evidence`).  `include/xlib.rs` is the
   mirror for this header and this unit includes it at C 23; Owed 2 moves these five names there.  */

/// Xlib.h 177-203: `typedef struct { int function; unsigned long plane_mask; unsigned long
/// foreground; unsigned long background; int line_width; int line_style; … char dashes; }
/// XGCValues;` - measured: `sizeof` **128**, align **8**, `function`@0, `plane_mask`@8,
/// `foreground`@16, `background`@24, `line_width`@32.  C 80-104 reads `foreground`/`background`
/// and one `XColor.pixel` each, so those two offsets are asserted.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XGCValues
{
  pub function: c_int,                          // Xlib.h 178
  pub plane_mask: c_ulong,                      // Xlib.h 179
  pub foreground: c_ulong,                      // Xlib.h 180
  pub background: c_ulong,                      // Xlib.h 181
  pub line_width: c_int,                        // Xlib.h 182
  pub line_style: c_int,                        // Xlib.h 183
  pub cap_style: c_int,                         // Xlib.h 184-185
  pub join_style: c_int,                        // Xlib.h 186
  pub fill_style: c_int,                        // Xlib.h 187-188
  pub fill_rule: c_int,                         // Xlib.h 189
  pub arc_mode: c_int,                          // Xlib.h 190
  pub tile: Pixmap,                             // Xlib.h 191
  pub stipple: Pixmap,                          // Xlib.h 192
  pub ts_x_origin: c_int,                       // Xlib.h 193
  pub ts_y_origin: c_int,                       // Xlib.h 194
  pub font: Font,                               // Xlib.h 195
  pub subwindow_mode: c_int,                    // Xlib.h 196
  pub graphics_exposures: Bool,                 // Xlib.h 197
  pub clip_x_origin: c_int,                     // Xlib.h 198
  pub clip_y_origin: c_int,                     // Xlib.h 199
  pub clip_mask: Pixmap,                        // Xlib.h 200
  pub dash_offset: c_int,                       // Xlib.h 201
  pub dashes: c_char,                           // Xlib.h 202
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<XGCValues> () == 128);
  core::assert! (std::mem::align_of::<XGCValues> () == 8);
  core::assert! (std::mem::offset_of! (XGCValues, foreground) == 16);
  core::assert! (std::mem::offset_of! (XGCValues, background) == 24);
};
/* X.h 566-567: `#define GCForeground (1L<<2)` / `#define GCBackground (1L<<3)` - the two value
   masks C 85 ORs together.  The arm split this comment used to ask for landed in status 3.535:
   `include/dispextern.rs`'s pair is the *fallback* arm's now (`1`/`2`, C's `ANDROID_GC_*` values,
   gated `not (feature = "have_x_windows")`) and `include/xlib.rs`'s `x11_constants` module
   carries X11's (`4`/`8`, clang-measured against the real header).  The two names live in that
   module - it is a nested module, so its items are named through it - and the single use site
   below does exactly that (the `widget.rs` `use x11_constants::CopyFromParent;` shape).  This is
   also the ledger's Deviations 10, closed.  */
/// C: Xlib.h 1839-1842: `extern int XScreenNumberOfScreen (Screen *);` - the name
/// `include/xterm.rs` 1166's `FRAME_X_SCREEN_NUMBER!` body calls (C 186-187 and 674), which no
/// mirror declares; Owed 5 names it for `include/xlib.rs`.
#[cfg (feature = "have_xft")]
unsafe extern "C" {
  pub fn XScreenNumberOfScreen (screen: *mut Screen) -> c_int;
}
/// C: Xlib.h 2623-2628: `extern Status XGetGCValues (Display *, GC, unsigned long, XGCValues *);`
/// (`Status` is Xlib.h's `typedef int Status`).  `include/xlib.rs` carries the other Xlib entry
/// points this tree names and not this one; Owed 2 moves it there.
#[cfg (feature = "have_xft")]
unsafe extern "C" {
  pub fn XGetGCValues (display: *mut Display, gc: GC, valuemask: c_ulong,
                       values_return: *mut XGCValues) -> c_int;
}

/* ----------------------------------------------------------------------------------------------
   C 24: the `<X11/Xft/Xft.h>` surface, and the two headers it pulls in - Fontconfig (its line 42)
   and `<X11/extensions/Xrender.h>` (43).  The Xft records are measured against libXft 2.3.6's own
   `Xft.h`/`XftCompat.h` (this machine has no `X11/Xft/Xft.h`: the toolchain is named in the ledger)
   plus this machine's Xrender/Fontconfig/FreeType headers, which are the real ones.  Every number
   in this segment is a clang reading; the probe is the ledger's `## Evidence` 4.  */

/// Xft.h 66-73: `typedef struct _XftFont { int ascent; int descent; int height; int
/// max_advance_width; FcCharSet *charset; FcPattern *pattern; } XftFont;` - measured: `sizeof`
/// **32**, align **8**, `ascent`@0, `descent`@4, `height`@8, `max_advance_width`@12, `charset`@16,
/// `pattern`@24.  C 223 reads `pattern`, C 256-273 `max_advance_width`/`ascent`/`descent`.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XftFont
{
  pub ascent: c_int,                            // Xft.h 67
  pub descent: c_int,                           // Xft.h 68
  pub height: c_int,                            // Xft.h 69
  pub max_advance_width: c_int,                 // Xft.h 70
  pub charset: *mut FcCharSet,                  // Xft.h 71
  pub pattern: *mut FcPattern,                  // Xft.h 72
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<XftFont> () == 32);
  core::assert! (std::mem::align_of::<XftFont> () == 8);
  core::assert! (std::mem::offset_of! (XftFont, max_advance_width) == 12);
  core::assert! (std::mem::offset_of! (XftFont, pattern) == 24);
};
/// Xft.h 75: `typedef struct _XftDraw XftDraw;` - an incomplete record this unit only ever names
/// through a pointer, so the stand-in is P-88 (1)'s pointer-only shape (no layout to measure).
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct _XftDraw { _private: [u8; 0] }
/// Xft.h 75: the pointer type C spells `XftDraw`.
#[cfg (feature = "have_xft")]
pub type XftDraw = *mut _XftDraw;
/// Xft.h 77-80: `typedef struct _XftColor { unsigned long pixel; XRenderColor color; } XftColor;`
/// - measured: `sizeof` **16**, align **8**, `pixel`@0, `color`@8 (`XRenderColor` is 8 bytes with
/// align 2, its four `unsigned short` members at 0/2/4/6 - Xrender.h 173-182).
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XftColor
{
  pub pixel: c_ulong,                           // Xft.h 78
  pub color: XRenderColor,                      // Xft.h 79
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<XftColor> () == 16);
  core::assert! (std::mem::align_of::<XftColor> () == 8);
  core::assert! (std::mem::offset_of! (XftColor, color) == 8);
  core::assert! (std::mem::offset_of! (XRenderColor, alpha) == 6);
};
/// XftCompat.h 126: `#define XftPatternDestroy FcPatternDestroy` - the name C 195 uses for the
/// same entry point.  C's spelling is a macro, so the port carries it as one (the
/// `include/xlib.rs` `XDestroyImage` shape), and the call site keeps C's own name.
#[cfg (feature = "have_xft")]
macro_rules! XftPatternDestroy {
  ($p:expr) => { FcPatternDestroy ($p) };
}

/// C: Xft.h 119-124: `Bool XftColorAllocValue (Display *dpy, Visual *visual, Colormap cmap,
/// _Xconst XRenderColor *color, XftColor *result);`  `Bool` is `int` (Xlib.h 185's mirror).
#[cfg (feature = "have_xft")]
unsafe extern "C" {
  pub fn XftColorAllocValue (dpy: *mut Display, visual: *mut Visual, cmap: Colormap,
                             color: *const XRenderColor, result: *mut XftColor) -> Bool;
  /// C: Xft.h 126-130: `void XftColorFree (Display *dpy, Visual *visual, Colormap cmap,
  /// XftColor *color);`
  pub fn XftColorFree (dpy: *mut Display, visual: *mut Visual, cmap: Colormap,
                       color: *mut XftColor);
  /// C: Xft.h 139-140: `void XftDefaultSubstitute (Display *dpy, int screen, FcPattern
  /// *pattern);`
  pub fn XftDefaultSubstitute (dpy: *mut Display, screen: c_int, pattern: *mut FcPattern);
  /// C: Xft.h 144-148: `XftDraw *XftDrawCreate (Display *dpy, Drawable drawable, Visual *visual,
  /// Colormap colormap);`
  pub fn XftDrawCreate (dpy: *mut Display, drawable: Drawable, visual: *mut Visual,
                        colormap: Colormap) -> XftDraw;
  /// C: Xft.h 175-176: `void XftDrawDestroy (XftDraw *draw);`
  pub fn XftDrawDestroy (draw: XftDraw);
  /// C: Xft.h 184-191: `void XftDrawGlyphs (XftDraw *draw, _Xconst XftColor *color, XftFont
  /// *pub, int x, int y, _Xconst FT_UInt *glyphs, int nglyphs);`
  pub fn XftDrawGlyphs (draw: XftDraw, color: *const XftColor, pub_: *mut XftFont, x: c_int,
                        y: c_int, glyphs: *const FT_UInt, nglyphs: c_int);
  /// C: Xft.h 265-271: `void XftDrawRect (XftDraw *draw, _Xconst XftColor *color, int x, int y,
  /// unsigned int width, unsigned int height);`
  pub fn XftDrawRect (draw: XftDraw, color: *const XftColor, x: c_int, y: c_int, width: c_uint,
                      height: c_uint);
  /// C: Xft.h 274-276: `Bool XftDrawSetClip (XftDraw *draw, Region r);`  `Region` is
  /// `<X11/Xutil.h>` 273's `typedef struct _XRegion *Region;` (declared below).
  pub fn XftDrawSetClip (draw: XftDraw, r: Region) -> Bool;
  /// C: Xft.h 279-284: `Bool XftDrawSetClipRectangles (XftDraw *draw, int xOrigin, int yOrigin,
  /// _Xconst XRectangle *rects, int n);`
  pub fn XftDrawSetClipRectangles (draw: XftDraw, xOrigin: c_int, yOrigin: c_int,
                                   rects: *const XRectangle, n: c_int) -> Bool;
  /// C: Xft.h 292-297: `void XftGlyphExtents (Display *dpy, XftFont *pub, _Xconst FT_UInt
  /// *glyphs, int nglyphs, XGlyphInfo *extents);`
  pub fn XftGlyphExtents (dpy: *mut Display, pub_: *mut XftFont, glyphs: *const FT_UInt,
                          nglyphs: c_int, extents: *mut XGlyphInfo);
  /// C: Xft.h 299-304: `void XftTextExtents8 (Display *dpy, XftFont *pub, _Xconst FcChar8
  /// *string, int len, XGlyphInfo *extents);`
  pub fn XftTextExtents8 (dpy: *mut Display, pub_: *mut XftFont, string: *const FcChar8,
                          len: c_int, extents: *mut XGlyphInfo);
  /// C: Xft.h 336-340: `FcPattern *XftFontMatch (Display *dpy, int screen, _Xconst FcPattern
  /// *pattern, FcResult *result);`
  pub fn XftFontMatch (dpy: *mut Display, screen: c_int, pattern: *const FcPattern,
                       result: *mut FcResult) -> *mut FcPattern;
  /// C: Xft.h 353-354: `FT_Face XftLockFace (XftFont *pub);`
  pub fn XftLockFace (pub_: *mut XftFont) -> FT_Face;
  /// C: Xft.h 356-357: `void XftUnlockFace (XftFont *pub);`
  pub fn XftUnlockFace (pub_: *mut XftFont);
  /// C: Xft.h 376-377: `XftFont *XftFontOpenPattern (Display *dpy, FcPattern *pattern);`
  pub fn XftFontOpenPattern (dpy: *mut Display, pattern: *mut FcPattern) -> *mut XftFont;
  /// C: Xft.h 382-383: `void XftFontClose (Display *dpy, XftFont *pub);`
  pub fn XftFontClose (dpy: *mut Display, pub_: *mut XftFont);
  /// C: Xft.h 412-415: `FcBool XftCharExists (Display *dpy, XftFont *pub, FcChar32 ucs4);`
  pub fn XftCharExists (dpy: *mut Display, pub_: *mut XftFont, ucs4: FcChar32) -> FcBool;
  /// C: Xft.h 417-420: `FT_UInt XftCharIndex (Display *dpy, XftFont *pub, FcChar32 ucs4);`
  pub fn XftCharIndex (dpy: *mut Display, pub_: *mut XftFont, ucs4: FcChar32) -> FT_UInt;
}

/* C 24's Fontconfig half (`<X11/Xft/Xft.h>` 42 includes `<fontconfig/fontconfig.h>`; C 190, 223,
   675-709 and 798-811 name these).  The records are measured against this machine's Fontconfig
   2.17.1 (`/opt/homebrew/include/fontconfig`) - the probe in the ledger's `## Evidence` 4.  */
/// C: fontconfig.h 48: `typedef int FcBool;` (`FcFalse` 85 / `FcTrue` 86 are `0`/`1`, measured).
#[cfg (feature = "have_xft")]
pub type FcBool = c_int;
#[cfg (feature = "have_xft")]
pub const FcFalse: FcBool = 0;
#[cfg (feature = "have_xft")]
pub const FcTrue: FcBool = 1;
/// C: fontconfig.h 279-285: `typedef enum _FcResult { FcResultMatch, … } FcResult;` - an `int`,
/// measured 4; the port keeps C's name and the one constant C 224, 683-704 compare against.
#[cfg (feature = "have_xft")]
pub type FcResult = c_int;
/// C: fontconfig.h 280: `FcResultMatch,` - measured 0 (the enum's first member).
#[cfg (feature = "have_xft")]
pub const FcResultMatch: FcResult = 0;
/// C: fontconfig.h 45: `typedef unsigned char  FcChar8;` - measured 1.
#[cfg (feature = "have_xft")]
pub type FcChar8 = c_uchar;
/// C: fontconfig.h 47: `typedef unsigned int   FcChar32;` - measured 4 (C 449 and 457 cast to
/// it).
#[cfg (feature = "have_xft")]
pub type FcChar32 = c_uint;
/// C: fontconfig.h 266: `typedef struct _FcCharSet FcCharSet;` - an incomplete record this unit
/// only names through `XftFont.charset`, so the stand-in has no layout (P-88 (1)).
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct FcCharSet { _private: [u8; 0] }
/// C: fontconfig.h 295: `typedef struct _FcPattern FcPattern;` - the record is incomplete for
/// every caller (`fcint.h` is private), so the pointer is the whole surface; C 163-223 passes it
/// to the five entry points below.
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct FcPattern { _private: [u8; 0] }
/// C: fontconfig.h 254-256: `typedef struct _FcMatrix { double xx, xy, yx, yy; } FcMatrix;` -
/// measured: `sizeof` **32**, align **8**, `xx`@0, `xy`@8, `yx`@16, `yy`@24.  C 223-229 reads all
/// four through a pointer.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct FcMatrix
{
  pub xx: c_double,                             // fontconfig.h 255
  pub xy: c_double,                             // fontconfig.h 255
  pub yx: c_double,                             // fontconfig.h 255
  pub yy: c_double,                             // fontconfig.h 255
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<FcMatrix> () == 32);
  core::assert! (std::mem::offset_of! (FcMatrix, xy) == 8);
  core::assert! (std::mem::offset_of! (FcMatrix, yy) == 24);
};
/// C: fontconfig.h 98-99, 100, 102, 117, 129: the object names C 223, 683-704 and 798-811 pass to
/// the pattern getters.  Their bodies are the header's own string literals, measured with
/// `clang -dM` on this machine's header; `&CStr` is the port's spelling for a `#define` string
/// (the `ftfont.rs` 1187-1205 shape).
/// C: fontconfig.h 98: `#define FC_ANTIALIAS            "antialias"      /* Bool (depends) */`
#[cfg (feature = "have_xft")]
pub const FC_ANTIALIAS: &CStr = c"antialias";
/// C: fontconfig.h 99: `#define FC_HINTING              "hinting"        /* Bool (true) */`
#[cfg (feature = "have_xft")]
pub const FC_HINTING: &CStr = c"hinting";
/// C: fontconfig.h 100: `#define FC_HINT_STYLE           "hintstyle"      /* Int */`
#[cfg (feature = "have_xft")]
pub const FC_HINT_STYLE: &CStr = c"hintstyle";
/// C: fontconfig.h 102: `#define FC_AUTOHINT             "autohint"       /* Bool (false) */`
#[cfg (feature = "have_xft")]
pub const FC_AUTOHINT: &CStr = c"autohint";
/// C: fontconfig.h 117: `#define FC_RGBA                 "rgba"           /* Int */`
#[cfg (feature = "have_xft")]
pub const FC_RGBA: &CStr = c"rgba";
/// C: fontconfig.h 129: `#define FC_EMBOLDEN             "embolden"       /* Bool - true if
/// emboldening needed*/`
#[cfg (feature = "have_xft")]
pub const FC_EMBOLDEN: &CStr = c"embolden";
/// C: fontconfig.h 132: `#define FC_LCD_FILTER           "lcdfilter"      /* Int */` - the name
/// C 40-43's `#ifndef` gives a fallback body for; the port declares it once, here, which is what
/// that `#ifndef` stands for (the `ftfont.rs` 1235-1238 reading).
#[cfg (feature = "have_xft")]
pub const FC_LCD_FILTER: &CStr = c"lcdfilter";
/// C: fontconfig.h 154: `#define FC_MATRIX               "matrix"     /* FcMatrix */`
#[cfg (feature = "have_xft")]
pub const FC_MATRIX: &CStr = c"matrix";
/// C: fontconfig.h 190-197: `#define FC_SPACING_PROPORTIONAL 0` … `#define FC_PROPORTIONAL
/// FC_SPACING_PROPORTIONAL` - measured 0/90/100/110 and the three aliases C 234, 248 and 251 use.
#[cfg (feature = "have_xft")]
pub const FC_SPACING_PROPORTIONAL: c_int = 0;
/// C: fontconfig.h 191: `#define FC_SPACING_DUAL         90`
#[cfg (feature = "have_xft")]
pub const FC_SPACING_DUAL: c_int = 90;
/// C: fontconfig.h 193: `#define FC_SPACING_CHARCELL     110`
#[cfg (feature = "have_xft")]
pub const FC_SPACING_CHARCELL: c_int = 110;
/// C: fontconfig.h 194: `#define FC_PROPORTIONAL         FC_SPACING_PROPORTIONAL`
#[cfg (feature = "have_xft")]
pub const FC_PROPORTIONAL: c_int = FC_SPACING_PROPORTIONAL;
/// C: fontconfig.h 195: `#define FC_DUAL                 FC_SPACING_DUAL`
#[cfg (feature = "have_xft")]
pub const FC_DUAL: c_int = FC_SPACING_DUAL;
/// C: fontconfig.h 197: `#define FC_CHARCELL             FC_SPACING_CHARCELL`
#[cfg (feature = "have_xft")]
pub const FC_CHARCELL: c_int = FC_SPACING_CHARCELL;

/// C: fontconfig.h 940-941: `FcPattern *FcPatternCreate (void);`
#[cfg (feature = "have_xft")]
unsafe extern "C" {
  pub fn FcPatternCreate () -> *mut FcPattern;
  /// C: fontconfig.h 961-962: `void FcPatternDestroy (FcPattern *p);` (C 190, 195 via
  /// `XftPatternDestroy` and 709 call it).
  pub fn FcPatternDestroy (p: *mut FcPattern);
  /// C: fontconfig.h 1033-1034: `FcResult FcPatternGetBool (const FcPattern *p, const char
  /// *object, int n, FcBool *b);`
  pub fn FcPatternGetBool (p: *const FcPattern, object: *const c_char, n: c_int,
                           b: *mut FcBool) -> FcResult;
  /// C: fontconfig.h 1018-1019: `FcResult FcPatternGetInteger (const FcPattern *p, const char
  /// *object, int n, int *i);`
  pub fn FcPatternGetInteger (p: *const FcPattern, object: *const c_char, n: c_int,
                              i: *mut c_int) -> FcResult;
  /// C: fontconfig.h 1027-1028: `FcResult FcPatternGetMatrix (const FcPattern *p, const char
  /// *object, int n, FcMatrix **s);`
  pub fn FcPatternGetMatrix (p: *const FcPattern, object: *const c_char, n: c_int,
                             s: *mut *mut FcMatrix) -> FcResult;
}

/* ----------------------------------------------------------------------------------------------
   C 36-38: `#ifdef HAVE_XRENDER` / `#include <X11/extensions/Xrender.h>` / `#endif`, and C 546's
   arm.  `<X11/Xft/Xft.h>` 43 includes the same header, so this is the second reach of one surface;
   it keeps the file's own feature (`have_xrender`) and the version half of C 546's condition as the
   derived `render_version_ge_0_2` (the `xfont.rs` 1006 reading).  Measured against this machine's
   `libXrender` (`/opt/homebrew/include`; `RENDER_MAJOR` 0, `RENDER_MINOR` 11 - so the version test
   is true here, which is why the arm is in the probe's cfg set).  */

/// render.h 31: `typedef XID Picture;` - `XID` is `<X11/Xlib.h>`'s `unsigned long`
/// (`include/xlib.rs` 837), measured 8.
#[cfg (all (feature = "have_xft", feature = "have_xrender"))]
pub type Picture = XID;
/// Xrender.h 173-182: `typedef struct { unsigned short red, green, blue, alpha; } XRenderColor;`
/// - measured: `sizeof` **8**, align **2**, the four members at 0/2/4/6.  C 108-134 writes
/// `color.alpha`/`red`/`green`/`blue` through `XftColor`'s embedded record and C 555-567 builds
/// one by value.
#[cfg (all (feature = "have_xft", feature = "have_xrender"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XRenderColor
{
  pub red: c_ushort,                            // Xrender.h 175
  pub green: c_ushort,                          // Xrender.h 177
  pub blue: c_ushort,                           // Xrender.h 179
  pub alpha: c_ushort,                          // Xrender.h 181
}
#[cfg (all (feature = "have_xft", feature = "have_xrender"))]
const _: () = {
  core::assert! (std::mem::size_of::<XRenderColor> () == 8);
  core::assert! (std::mem::align_of::<XRenderColor> () == 2);
  core::assert! (std::mem::offset_of! (XRenderColor, green) == 2);
};
/// Xrender.h 190-205: `typedef struct _XGlyphInfo { unsigned short width; unsigned short height;
/// short x; short y; short xOff; short yOff; } XGlyphInfo;` - measured: `sizeof` **12**, align
/// **2**, `width`@0, `height`@2, `x`@4, `y`@6, `xOff`@8, `yOff`@10.  C 169/466 declare one and
/// C 257-268 and 472-477 read it.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XGlyphInfo
{
  pub width: c_ushort,                          // Xrender.h 192
  pub height: c_ushort,                         // Xrender.h 194
  pub x: c_short,                               // Xrender.h 197
  pub y: c_short,                               // Xrender.h 199
  pub xOff: c_short,                            // Xrender.h 202
  pub yOff: c_short,                            // Xrender.h 204
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<XGlyphInfo> () == 12);
  core::assert! (std::mem::align_of::<XGlyphInfo> () == 2);
  core::assert! (std::mem::offset_of! (XGlyphInfo, xOff) == 8);
  core::assert! (std::mem::offset_of! (XGlyphInfo, yOff) == 10);
};
/// render.h 94: `#define PictOpSrc			    1` - measured 1.
#[cfg (all (feature = "have_xft", feature = "have_xrender"))]
pub const PictOpSrc: c_int = 1;
/// C: Xrender.h 585-587: `void XRenderSetPictureClipRectangles (Display *dpy, Picture picture,
/// int xOrigin, int yOrigin, _Xconst XRectangle *rects, int n);`
#[cfg (all (feature = "have_xft", feature = "have_xrender"))]
unsafe extern "C" {
  pub fn XRenderSetPictureClipRectangles (dpy: *mut Display, picture: Picture, xOrigin: c_int,
                                         yOrigin: c_int, rects: *const XRectangle,
                                         n: c_int);
  /// C: Xrender.h 824-826: `void XRenderFillRectangle (Display *dpy, int op, Picture dst,
  /// _Xconst XRenderColor *color, int x, int y, unsigned int width, unsigned int height);`
  pub fn XRenderFillRectangle (dpy: *mut Display, op: c_int, dst: Picture,
                               color: *const XRenderColor, x: c_int, y: c_int, width: c_uint,
                               height: c_uint);
}

/// Xutil.h 273: `typedef struct _XRegion *Region;` - the record is opaque to every caller
/// (`Xlib.h`'s region primitives deal in the pointer), and C 524 passes C's own `NULL`; the
/// stand-in is P-88 (1)'s pointer-only shape, so there is no layout to measure.  `include/xlib.rs`
/// carries the Xlib records this tree names and not this one; Owed 2 moves it there.
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct _XRegion { _private: [u8; 0] }
/// Xutil.h 273: the pointer type C spells `Region`.
#[cfg (feature = "have_xft")]
pub type Region = *mut _XRegion;

/* ----------------------------------------------------------------------------------------------
   C 33's `<ft2build.h>`/`FT_FREETYPE_H` chain (which `src/ftfont.h` 25-31 pulls in): the FreeType
   surface this unit names.  Every layout is a clang reading of this machine's headers
   (`/opt/homebrew/include/freetype2`, FreeType `2.14.3`: `FREETYPE_MAJOR` 2 `FREETYPE_MINOR` 14
   `FREETYPE_PATCH` 3) - measured: `sizeof (FT_Face)` **8**, `sizeof (FT_Size)` **8**,
   `sizeof (FT_Matrix)` **32** align **8**, `sizeof (BDF_PropertyRec)` **16** align **8**
   (`type`@0, `u`@8, `u.integer`@8), `sizeof (FT_FaceRec)` **248** align **8** with
   `face_flags`@16, `units_per_EM`@136, `max_advance_width`@144, `underline_position`@148,
   `underline_thickness`@150, `glyph`@152, `size`@160, `driver`@176, `internal`@240;
   `FT_FACE_FLAG_SFNT` **8**; `BDF_PROPERTY_TYPE_INTEGER` **2**; `FT_BDF_H` is defined once
   `FT_FREETYPE_H` is in (so C 321-336's arm is live on any FreeType of the Xft era).  The probe
   and its verbatim output are in the ledger's `## Evidence` 4; `src/ftfont.rs`'s segment measures
   the same headers for `src/ftfont.c`, and Owed 4 asks for the one shared mirror.  */

/// C: fttypes.h 302: `typedef int  FT_Error;`
#[cfg (feature = "have_xft")]
pub type FT_Error = c_int;
/// C: fttypes.h 256/234: `typedef unsigned long  FT_ULong;` / `typedef unsigned int  FT_UInt;`
/// (`FT_UInt` is the 32-bit half; measured 4).
#[cfg (feature = "have_xft")]
pub type FT_UInt = c_uint;
/// C: fttypes.h 245: `typedef signed long  FT_Long;`
#[cfg (feature = "have_xft")]
pub type FT_Long = c_long;
/// C: fttypes.h 190: `typedef char  FT_String;`
#[cfg (feature = "have_xft")]
pub type FT_String = c_char;
/// C: fttypes.h 223: `typedef signed int  FT_Int;`
#[cfg (feature = "have_xft")]
pub type FT_Int = c_int;
/// C: fttypes.h 201: `typedef signed short  FT_Short;`
#[cfg (feature = "have_xft")]
pub type FT_Short = c_short;
/// C: fttypes.h 212: `typedef unsigned short  FT_UShort;`
#[cfg (feature = "have_xft")]
pub type FT_UShort = c_ushort;
/// C: fttypes.h 290: `typedef signed long  FT_Fixed;`
#[cfg (feature = "have_xft")]
pub type FT_Fixed = c_long;
/// C: freetype.h 641: `typedef struct FT_FaceRec_*  FT_Face;` (measured 8).
#[cfg (feature = "have_xft")]
pub type FT_Face = *mut FT_FaceRec_;
/// C: freetype.h 680: `typedef struct FT_SizeRec_*  FT_Size;` (measured 8).
#[cfg (feature = "have_xft")]
pub type FT_Size = *mut FT_SizeRec_;
/// C: freetype.h 706: `typedef struct FT_GlyphSlotRec_*  FT_GlyphSlot;`
#[cfg (feature = "have_xft")]
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec_;
/// C: freetype.h 747: `typedef struct FT_CharMapRec_*  FT_CharMap;`
#[cfg (feature = "have_xft")]
pub type FT_CharMap = *mut FT_CharMapRec_;
/// C: freetype.h 589: `typedef struct FT_DriverRec_*  FT_Driver;`
#[cfg (feature = "have_xft")]
pub type FT_Driver = *mut FT_DriverRec_;
/// C: fttypes.h 111: `typedef unsigned char  FT_Byte;`
#[cfg (feature = "have_xft")]
pub type FT_Byte = c_uchar;
/// C: fttypes.h 438: `typedef void (*FT_Generic_Finalizer)(void* object);`
#[cfg (feature = "have_xft")]
pub type FT_Generic_Finalizer = Option<unsafe extern "C" fn (object: *mut c_void)>;
/// C: ftbdf.h 74-81: `typedef enum BDF_PropertyType_ { BDF_PROPERTY_TYPE_NONE = 0,
/// BDF_PROPERTY_TYPE_ATOM = 1, BDF_PROPERTY_TYPE_INTEGER = 2, BDF_PROPERTY_TYPE_CARDINAL = 3 }
/// BDF_PropertyType;`
#[cfg (feature = "have_xft")]
pub type BDF_PropertyType = c_int;
/// C: ftbdf.h 74-81: `BDF_PROPERTY_TYPE_INTEGER` - measured 2.
#[cfg (feature = "have_xft")]
pub const BDF_PROPERTY_TYPE_INTEGER: BDF_PropertyType = 2;

/* The incomplete records the FreeType surface names only through a pointer, plus the three the
   `FT_FaceRec_` below carries by value or as a field.  C reads no field of any of them, so each
   keeps P-88 (1)'s pointer-only stand-in (the `src/ftfont.rs` slice-1 shape for `FT_LibraryRec_`)
   and there is no layout to measure.  */
/// C: freetype.h 2011: `typedef struct FT_SizeRec_ { … } FT_SizeRec;` - C 315 stores the pointer
/// only (`xftfont_info->ft_size = ft_face->size`), so the record stays the stand-in.
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct FT_SizeRec_ { _private: [u8; 0] }
/// C: freetype.h 2264: `typedef struct FT_GlyphSlotRec_ { … } FT_GlyphSlotRec;` - the pointer is
/// `FT_FaceRec_`'s `glyph` member (offset 152, measured).
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct FT_GlyphSlotRec_ { _private: [u8; 0] }
/// C: freetype.h 1004: `typedef struct FT_CharMapRec_ { … } FT_CharMapRec;`
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct FT_CharMapRec_ { _private: [u8; 0] }
/// C: freetype.h 589: `typedef struct FT_DriverRec_ { … } FT_DriverRec;`
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct FT_DriverRec_ { _private: [u8; 0] }
/// C: ftsystem.h 64: `typedef struct FT_MemoryRec_*  FT_Memory;`
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct FT_MemoryRec_ { _private: [u8; 0] }
/// C: ftsystem.h 64: the pointer type C spells `FT_Memory`.
#[cfg (feature = "have_xft")]
pub type FT_Memory = *mut FT_MemoryRec_;
/// C: ftsystem.h 198: `typedef struct FT_StreamRec_*  FT_Stream;`
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct FT_StreamRec_ { _private: [u8; 0] }
/// C: ftsystem.h 198: the pointer type C spells `FT_Stream`.
#[cfg (feature = "have_xft")]
pub type FT_Stream = *mut FT_StreamRec_;
/// C: freetype.h 1042: `typedef struct FT_Face_InternalRec_*  FT_Face_Internal;`
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct FT_Face_InternalRec_ { _private: [u8; 0] }
/// C: freetype.h 1042: the pointer type C spells `FT_Face_Internal`.
#[cfg (feature = "have_xft")]
pub type FT_Face_Internal = *mut FT_Face_InternalRec_;
/// C: freetype.h 506-514: `typedef struct FT_Bitmap_Size_ { FT_Short height; FT_Short width;
/// FT_Pos size; FT_Pos x_ppem; FT_Pos y_ppem; } FT_Bitmap_Size;` - measured `sizeof` **32**
/// (`height`@0, `width`@2, `size`@8, `x_ppem`@16, `y_ppem`@24); `FT_FaceRec_.available_sizes` is
/// the pointer and C reads no member.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct FT_Bitmap_Size
{
  pub height: FT_Short,
  pub width: FT_Short,
  pub size: c_long,                             // FT_Pos (ftimage.h 57)
  pub x_ppem: c_long,
  pub y_ppem: c_long,
}
#[cfg (feature = "have_xft")]
const _: () = core::assert! (std::mem::size_of::<FT_Bitmap_Size> () == 32);

/// C: fttypes.h 470-475: `typedef struct FT_Generic_ { void* data; FT_Generic_Finalizer
/// finalizer; } FT_Generic;` - measured `sizeof` **16** (`data`@0, `finalizer`@8).
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct FT_Generic
{
  pub data: *mut c_void,
  pub finalizer: FT_Generic_Finalizer,
}
#[cfg (feature = "have_xft")]
const _: () = core::assert! (std::mem::size_of::<FT_Generic> () == 16);

/// C: ftimage.h 118-124: `typedef struct FT_BBox_ { FT_Pos xMin, yMin, xMax, yMax; } FT_BBox;` -
/// measured `sizeof` **32**.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct FT_BBox
{
  pub xMin: c_long,                             // FT_Pos (ftimage.h 57)
  pub yMin: c_long,
  pub xMax: c_long,
  pub yMax: c_long,
}
#[cfg (feature = "have_xft")]
const _: () = core::assert! (std::mem::size_of::<FT_BBox> () == 32);

/// C: fttypes.h 582-587: `typedef struct FT_ListRec_ { FT_ListNode head; FT_ListNode tail; }
/// FT_ListRec;` - measured **16** (`head`@0, `tail`@8); `FT_FaceRec_.sizes_list` is by value.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct FT_ListRec
{
  pub head: *mut c_void,                        // FT_ListNode
  pub tail: *mut c_void,
}
#[cfg (feature = "have_xft")]
const _: () = core::assert! (std::mem::size_of::<FT_ListRec> () == 16);

/// C: fttypes.h 393-400: `typedef struct FT_Matrix_ { FT_Fixed xx, xy; FT_Fixed yx, yy; }
/// FT_Matrix;` - the record `struct font_info`'s `matrix` member is (C 222-229 writes all four),
/// measured: `sizeof` **32**, align **8**, 0/8/16/24.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct FT_Matrix
{
  pub xx: FT_Fixed,
  pub xy: FT_Fixed,
  pub yx: FT_Fixed,
  pub yy: FT_Fixed,
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<FT_Matrix> () == 32);
  core::assert! (std::mem::align_of::<FT_Matrix> () == 8);
  core::assert! (std::mem::offset_of! (FT_Matrix, xy) == 8);
  core::assert! (std::mem::offset_of! (FT_Matrix, yy) == 24);
};

/// C: freetype.h 1229-1284: `typedef struct FT_FaceRec_ { FT_Long num_faces; … } FT_FaceRec;` -
/// C 296-333 reads `units_per_EM`, `underline_position`, `underline_thickness`, `size` and
/// `face_flags`, so the record is written out from the header's own lines with this machine's
/// measured layout (see the segment note above).  The private members after `size` are kept so the
/// record's own size is C's 248 (P-86 (1): a record declared short of its C text is a different
/// type).
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct FT_FaceRec_
{
  /// C: freetype.h 1231-1237: `FT_Long num_faces; FT_Long face_index; FT_Long face_flags;
  /// FT_Long style_flags; FT_Long num_glyphs;`
  pub num_faces: FT_Long,
  pub face_index: FT_Long,
  pub face_flags: FT_Long,
  pub style_flags: FT_Long,
  pub num_glyphs: FT_Long,
  /// C: freetype.h 1239-1240: `FT_String* family_name; FT_String* style_name;`
  pub family_name: *mut FT_String,
  pub style_name: *mut FT_String,
  /// C: freetype.h 1242-1243: `FT_Int num_fixed_sizes; FT_Bitmap_Size* available_sizes;`
  pub num_fixed_sizes: FT_Int,
  pub available_sizes: *mut FT_Bitmap_Size,
  /// C: freetype.h 1245-1246: `FT_Int num_charmaps; FT_CharMap* charmaps;`
  pub num_charmaps: FT_Int,
  pub charmaps: *mut FT_CharMap,
  /// C: freetype.h 1248: `FT_Generic generic;`
  pub generic: FT_Generic,
  /// C: freetype.h 1253: `FT_BBox bbox;`
  pub bbox: FT_BBox,
  /// C: freetype.h 1255-1258: `FT_UShort units_per_EM; FT_Short ascender; FT_Short descender;
  /// FT_Short height;`
  pub units_per_EM: FT_UShort,
  pub ascender: FT_Short,
  pub descender: FT_Short,
  pub height: FT_Short,
  /// C: freetype.h 1260-1261: `FT_Short max_advance_width; FT_Short max_advance_height;`
  pub max_advance_width: FT_Short,
  pub max_advance_height: FT_Short,
  /// C: freetype.h 1263-1264: `FT_Short underline_position; FT_Short underline_thickness;`
  pub underline_position: FT_Short,
  pub underline_thickness: FT_Short,
  /// C: freetype.h 1266-1268: `FT_GlyphSlot glyph; FT_Size size; FT_CharMap charmap;`
  pub glyph: FT_GlyphSlot,
  pub size: FT_Size,
  pub charmap: FT_CharMap,
  /// C: freetype.h 1272-1274: `FT_Driver driver; FT_Memory memory; FT_Stream stream;`
  pub driver: FT_Driver,
  pub memory: FT_Memory,
  pub stream: FT_Stream,
  /// C: freetype.h 1276: `FT_ListRec sizes_list;`
  pub sizes_list: FT_ListRec,
  /// C: freetype.h 1278-1279: `FT_Generic autohint; void* extensions;`
  pub autohint: FT_Generic,
  pub extensions: *mut c_void,
  /// C: freetype.h 1281: `FT_Face_Internal internal;`
  pub internal: FT_Face_Internal,
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<FT_FaceRec_> () == 248);
  core::assert! (std::mem::align_of::<FT_FaceRec_> () == 8);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, face_flags) == 16);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, units_per_EM) == 136);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, max_advance_width) == 144);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, underline_position) == 148);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, underline_thickness) == 150);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, glyph) == 152);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, size) == 160);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, internal) == 240);
};

/// C: freetype.h 1416: `#define FT_FACE_FLAG_SFNT              ( 1L <<  3 )` - measured 8.  C 309
/// test it against `face_flags`.
#[cfg (feature = "have_xft")]
pub const FT_FACE_FLAG_SFNT: FT_Long = 1 << 3;
/// C: ftbdf.h 118-128: `typedef struct BDF_PropertyRec_ { BDF_PropertyType type; union { const
/// char* atom; FT_Int32 integer; FT_UInt32 cardinal; } u; } BDF_PropertyRec;` - C 326-334 reads
/// `type` and `u.integer`, so the record and the union are written out.  Measured: `sizeof` **16**,
/// align **8**, `type`@0, `u`@8, `u.integer`@8.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub union BDF_PropertyRec_u
{
  /// C: ftbdf.h 122: `const char*     atom;`
  pub atom: *const c_char,
  /// C: ftbdf.h 123: `FT_Int32        integer;`
  pub integer: c_int,
  /// C: ftbdf.h 124: `FT_UInt32       cardinal;`
  pub cardinal: c_uint,
}
/// C: ftbdf.h 118-128: the record itself.
#[cfg (feature = "have_xft")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct BDF_PropertyRec
{
  pub r#type: BDF_PropertyType,
  pub u: BDF_PropertyRec_u,
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<BDF_PropertyRec> () == 16);
  core::assert! (std::mem::align_of::<BDF_PropertyRec> () == 8);
  core::assert! (std::mem::offset_of! (BDF_PropertyRec, r#type) == 0);
  core::assert! (std::mem::offset_of! (BDF_PropertyRec, u) == 8);
};
/// C: ftbdf.h 206-208: `FT_EXPORT( FT_Error ) FT_Get_BDF_Property ( FT_Face face, const char*
/// prop_name, BDF_PropertyRec* aproperty );`  C 326-334 calls it three times, inside the
/// `#ifdef FT_BDF_H` arm.
#[cfg (feature = "have_xft")]
unsafe extern "C" {
  pub fn FT_Get_BDF_Property (face: FT_Face, prop_name: *const c_char,
                              aproperty: *mut BDF_PropertyRec) -> FT_Error;
}

/* C 308-311 / C 347-353 / C 612 / C 727-744 / C 769 / C 820-830 name libotf's and HarfBuzz's own
   records.  `hb_font_t` is `hb-common.h` 520's `typedef struct hb_font_t hb_font_t;` (HarfBuzz
   14.4.0 is installed here, keg-only: `/opt/homebrew/Cellar/harfbuzz/14.4.0/include/harfbuzz`), and
   `hb_font_destroy` is `hb-font.h` 1129's `void hb_font_destroy (hb_font_t *font);`.
   `<otf.h>` (libotf) has no copy on this machine: `OTF` and `OTF_close` are declared from their
   published prototypes and **marked UNVERIFIED** in the ledger - both are opaque pointers whose
   layout nothing reads, so there is no number to measure (P-88 (2)).  */
/// C: hb-common.h 520: `typedef struct hb_font_t hb_font_t;` (HarfBuzz 14.4.0, measured
/// `sizeof (hb_font_t *)` 8).
#[cfg (feature = "have_xft")]
pub type hb_font_t = *mut c_void;
/// C: hb-font.h 1128-1129: `void hb_font_destroy (hb_font_t *font);`
#[cfg (all (feature = "have_xft", feature = "harfbuzz"))]
unsafe extern "C" {
  pub fn hb_font_destroy (font: hb_font_t);
}
/// C: otf.h: `typedef struct OTF *OTF;` - **UNVERIFIED**: libotf is not installed on this machine,
/// so the header could not be read or measured.  The name is `struct font_info`'s `otf` member,
/// and C 350 passes it to `OTF_close`; both uses are pointers, so no layout is involved.
#[cfg (feature = "have_xft")]
pub type OTF = *mut c_void;
/// C: otf.h: `int OTF_close (OTF otf);` - UNVERIFIED for the same reason (libotf absent); C 350
/// calls it and discards the result.
#[cfg (all (feature = "have_xft", feature = "have_libotf"))]
unsafe extern "C" {
  pub fn OTF_close (otf: OTF) -> c_int;
}

/* The C-side names this unit calls that the mirrors it includes do not carry.  Each keeps its
   own header's line (P-55/P-72 rule 3), so a citation is never paired with the wrong name; the
   ledger's Owed 5 names the mirror each belongs in.  `x_mark_frame_dirty` (xterm.h 1405) and
   `font_add_log` (font.h 1001) are *not* here: `include/xterm.rs` 228 and `include/font.rs` 299
   declare them for the macro bodies that call them (status 3.521/3.524), and a second copy would
   be the `check-mirror-dupes.py` finding that rule exists for.  */
#[cfg (feature = "have_xft")]
unsafe extern "C" {
  /// C: font.h 848: `extern Lisp_Object font_build_object (int, Lisp_Object, Lisp_Object,
  /// double);` (font.c defines it; C 203 builds the font object with
  /// `VECSIZE (struct font_info)`).
  pub fn font_build_object (vectorsize: c_int, r#type: Lisp_Object, entity: Lisp_Object,
                            pixelsize: c_double) -> Lisp_Object;
  /// C: font.h 923: `extern void font_put_frame_data (struct frame *, Lisp_Object, void *);`
  /// (C 493 and 644 store the frame's `XftDraw`).
  pub fn font_put_frame_data (f: *mut frame, prop: Lisp_Object, data: *mut c_void);
  /// C: font.h 924: `extern void *font_get_frame_data (struct frame *f, Lisp_Object);` (C 482 and
  /// 638 read it back).
  pub fn font_get_frame_data (f: *mut frame, prop: Lisp_Object) -> *mut c_void;
  /// C: font.h 896: `extern void register_font_driver (struct font_driver const *,
  /// struct frame *);` (C 819 and 830 register the two drivers).
  pub fn register_font_driver (d: *const font_driver, f: *mut frame);
  /// C: font.h 961: `extern Lisp_Object xfont_get_cache (struct frame *);` - `src/xfont.c` 118
  /// defines it and C 754 uses it for this driver's `get_cache`.
  pub fn xfont_get_cache (f: *mut c_void) -> Lisp_Object;
  /// C: xterm.h 1003: `extern struct x_display_info *x_display_info_for_display (Display *);`
  /// (C 364 asks whether the display the font was opened on still exists).
  pub fn x_display_info_for_display (display: *mut Display) -> *mut x_display_info;
  /// C: xterm.h 1797: `extern void x_query_colors (struct frame *f, XColor *, int);` (C 107).
  pub fn x_query_colors (f: *mut frame, colors: *mut XColor, n: c_int);
  /// C: font.h 935-936: `extern int ftfont_anchor_point (struct font *, unsigned int, int,
  /// int *, int *);` (C 767).
  pub fn ftfont_anchor_point (font: *mut font, code: c_uint, idx: c_int, x: *mut c_int,
                              y: *mut c_int) -> c_int;
  /// C: font.h 937-938: `extern int ftfont_get_bitmap (struct font *, unsigned int,
  /// struct font_bitmap *, int);` (C 766).
  pub fn ftfont_get_bitmap (font: *mut font, code: c_uint, bitmap: *mut font_bitmap,
                            bits_per_pixel: c_int) -> c_int;
  /// C: font.h 939: `extern int ftfont_has_char (Lisp_Object, int);` (C 446).
  pub fn ftfont_has_char (font: Lisp_Object, c: c_int) -> c_int;
  /// C: font.h 940: `extern int ftfont_variation_glyphs (struct font *, int, unsigned[256]);`
  /// (C 776).
  pub fn ftfont_variation_glyphs (font: *mut font, c: c_int, variations: *mut c_uint) -> c_int;
  /// C: font.h 941: `extern Lisp_Object ftfont_combining_capability (struct font *);` (C 780).
  pub fn ftfont_combining_capability (font: *mut font) -> Lisp_Object;
  /// C: font.h 943: `extern Lisp_Object ftfont_list2 (struct frame *, Lisp_Object, Lisp_Object);`
  /// (C 146 and 718).
  pub fn ftfont_list2 (f: *mut c_void, spec: Lisp_Object, r#type: Lisp_Object) -> Lisp_Object;
  /// C: font.h 944: `extern Lisp_Object ftfont_list_family (struct frame *);` (C 757).
  pub fn ftfont_list_family (f: *mut c_void) -> Lisp_Object;
  /// C: font.h 945: `extern Lisp_Object ftfont_match2 (struct frame *, Lisp_Object,
  /// Lisp_Object);` (C 152 and 724).
  pub fn ftfont_match2 (f: *mut c_void, spec: Lisp_Object, r#type: Lisp_Object) -> Lisp_Object;
  /// C: font.h 947: `extern Lisp_Object ftfont_otf_capability (struct font *);` (C 769).
  pub fn ftfont_otf_capability (font: *mut font) -> Lisp_Object;
  /// C: font.h 948: `extern Lisp_Object ftfont_shape (Lisp_Object, Lisp_Object);` (C 620).
  pub fn ftfont_shape (lgstring: Lisp_Object, direction: Lisp_Object) -> Lisp_Object;
  /// C: font.h 951: `extern void ftfont_filter_properties (Lisp_Object, Lisp_Object);` (C 778).
  pub fn ftfont_filter_properties (font: Lisp_Object, alist: Lisp_Object);
  /// C: font.h 917: `extern Lisp_Object hbfont_otf_capability (struct font *);` (C 825).
  pub fn hbfont_otf_capability (font: *mut font) -> Lisp_Object;
  /// C: font.h 918: `extern Lisp_Object hbfont_shape (Lisp_Object, Lisp_Object);` (C 826).
  pub fn hbfont_shape (lgstring: Lisp_Object, direction: Lisp_Object) -> Lisp_Object;
  /// C: font.h 919: `extern Lisp_Object hbfont_combining_capability (struct font *);` (C 827).
  pub fn hbfont_combining_capability (font: *mut font) -> Lisp_Object;
  /// C: font.h 955: `extern hb_font_t *fthbfont_begin_hb_font (struct font *, double *);`
  /// (C 735).
  pub fn fthbfont_begin_hb_font (font: *mut font, position_unit: *mut c_double) -> hb_font_t;
}
/// C: xterm.h 1826-1827: `extern void x_xrender_color_from_gc_background (struct frame *, GC,
/// XRenderColor *, bool);` (C 564) - declared in the reference only under `HAVE_XRENDER`, so the
/// gate here is C's own.
#[cfg (all (feature = "have_xft", feature = "have_xrender"))]
unsafe extern "C" {
  pub fn x_xrender_color_from_gc_background (f: *mut frame, gc: GC, color: *mut XRenderColor,
                                             enabled: bool);
  /// C: xterm.h 1828: `extern void x_xr_ensure_picture (struct frame *);` (C 551).
  pub fn x_xr_ensure_picture (f: *mut frame);
  /// C: xterm.h 1830: `extern void x_xr_reset_ext_clip (struct frame *);` (C 563 and 568).
  pub fn x_xr_reset_ext_clip (f: *mut frame);
}

/* `struct face` (dispextern.h 1787-1937) is this unit's use-site record (P-72 rule 3):
   `include/dispextern.rs`'s copy is behind `dispextern_face_records` and so is not available to a
   unit compiled on its own.  C reads `gc` (C 72, 390), `foreground`/`background` (C 88-96),
   `font` (C 515) and writes `extra` (C 395, 426), so those five members are pinned - clang,
   `-DHAVE_WINDOW_SYSTEM=1 -DHAVE_XWINDOWS=1 -DHAVE_XFT=1` over the reference's own `dispextern.h`
   (the ledger's `## Evidence` 5): `gc`@168, `stipple`@176, `foreground`@184, `background`@192,
   `font`@232, `fontset`@240, `extra`@296, `sizeof` **304**, align **8**.  C's `HAVE_X_WINDOWS` arm
   of the member is `GC gc;` (dispextern.h 1802), which is `include/xlib.rs` 892's `GC`.  */
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct face
{
  _prefix: [u8; 168],                           // dispextern.h 1791-1799 (`lface`, `id`; clang: `gc`@168)
  pub gc: GC,                                   // dispextern.h 1802: `GC gc;`
  _stipple: ptrdiff_t,                          // dispextern.h 1810: `ptrdiff_t stipple;`
  pub foreground: c_ulong,                      // dispextern.h 1816: `unsigned long foreground;`
  pub background: c_ulong,                      // dispextern.h 1819: `unsigned long background;`
  _colors: [c_ulong; 4],                        // dispextern.h 1822-1826: `underline_color`…`box_color`
  pub font: *mut font,                          // dispextern.h 1828: `struct font *font;`
  _tail: [u8; 56],                              // dispextern.h 1832-1868 (`fontset`…`ascii_face`; clang: `extra`@296)
  pub extra: *mut c_void,                       // dispextern.h 1870: `void *extra;` (HAVE_XFT)
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<face> () == 304);
  core::assert! (std::mem::offset_of! (face, gc) == 168);
  core::assert! (std::mem::offset_of! (face, foreground) == 184);
  core::assert! (std::mem::offset_of! (face, background) == 192);
  core::assert! (std::mem::offset_of! (face, font) == 232);
  core::assert! (std::mem::offset_of! (face, extra) == 296);
};

/* C 39-43: `#ifndef FC_LCD_FILTER` / `# define FC_LCD_FILTER "lcdfilter"` / `#endif` - the
   header's own fallback body for a string macro.  The port declares the name once, in the
   Fontconfig segment above (`FC_LCD_FILTER`, cited to fontconfig.h 132), which is what an
   `#ifndef`-guarded `#define` of the same name stands for: there is no second copy to carry (the
   `ftfont.rs` 1235-1238 reading).  */

/* C 46-56: "Xft font driver." (C 46), "Structure pointed by (struct face *)->extra" (C 48) and
   `struct xftface_info`.  Measured with the Xft segment's headers: `sizeof` **40**, align **8**,
   `bg_allocated_p`@0, `fg_allocated_p`@1, `xft_fg`@8, `xft_bg`@24 (two `bool`s, six bytes of
   padding, then the two 16-byte XftColors).  */
/// C: C 50-56: `struct xftface_info { bool bg_allocated_p; bool fg_allocated_p; XftColor xft_fg;
/// XftColor xft_bg; };`
#[cfg (feature = "have_xft")]
#[repr(C)]
pub struct xftface_info
{
  pub bg_allocated_p: bool,                     // C 52
  pub fg_allocated_p: bool,                     // C 53
  pub xft_fg: XftColor,                         // C 54 (`color for face->foreground`)
  pub xft_bg: XftColor,                         // C 55 (`color for face->background`)
}
#[cfg (feature = "have_xft")]
const _: () = {
  core::assert! (std::mem::size_of::<xftface_info> () == 40);
  core::assert! (std::mem::align_of::<xftface_info> () == 8);
  core::assert! (std::mem::offset_of! (xftface_info, xft_fg) == 8);
  core::assert! (std::mem::offset_of! (xftface_info, xft_bg) == 24);
};

/* `s->first_glyph->u.glyphless.method` (C 540-541) is the union's packed bitfield
   (dispextern.h 624-634, `include/dispextern.rs` 454-465); the mirror carries the storage
   (`bits`) and the shift, and the accessor is the unit's own - the `w32term.rs` 1733 shape.  */
#[cfg (feature = "have_xft")]
macro_rules! glyphless_bits {
  (method, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_METHOD_SHIFT) & 0x3) as c_int };
}

/* C 140 and C 155: the file's two `static` caches.  `ascii_printable` is C's `static FcChar8
   ascii_printable[95]` and stays unit-local; it carries the file gate like every other item
   (P-86 (8): the names it needs come from the gated include graph).  */
#[cfg (feature = "have_xft")]
static mut ascii_printable: [FcChar8; 95] = [0; 95];

/* C 58-141: the comment at C 58-61 and `xftfont_get_colors`.  C's `struct frame *f` and
   `struct face *face` parameters take the `font_driver` table's untyped pointers where C's own
   callers are the driver (P-72 rule 3, the `xfont.rs` reading), and the body converts at the use
   point.  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_get_colors (f: *mut c_void, face: *mut c_void, gc: GC,
                                         xftface_info: *mut xftface_info,
                                         fg: *mut XftColor, bg: *mut XftColor,
                                         bg_allocated_p: *mut bool, fg_allocated_p: *mut bool)
{                                                          // C 63-141
  let face: *mut face = face as *mut face;

  *bg_allocated_p = false;                                 // C 69
  *fg_allocated_p = false;                                 // C 70

  if ! xftface_info.is_null () && (*face).gc == gc          // C 72
    {
      *fg = (*xftface_info).xft_fg;                         // C 74
      if ! bg.is_null ()                                    // C 75
	{ *bg = (*xftface_info).xft_bg; }                   // C 76
    }
  else
    {
      let mut xgcv: XGCValues = std::mem::zeroed ();        // C 80 (C leaves it for XGetGCValues)
      let mut fg_done: bool = false;                        // C 81
      let mut bg_done: bool = false;

      block_input ();                                       // C 83
      use x11_constants::{GCBackground, GCForeground};      // X.h 566-567 (status 3.535)
      XGetGCValues (FRAME_X_DISPLAY! ((f as *mut frame)), gc,
		    (GCForeground | GCBackground) as c_ulong,
		    &raw mut xgcv);                      // C 84-85
      if ! xftface_info.is_null ()                          // C 86
	{
	  if xgcv.foreground == (*face).foreground          // C 88
	    { *fg = (*xftface_info).xft_fg; fg_done = true; }
	  else if xgcv.foreground == (*face).background     // C 90
	    { *fg = (*xftface_info).xft_bg; fg_done = true; }
	  if bg.is_null ()                                  // C 92
	    { bg_done = true; }                             // C 93
	  else if xgcv.background == (*face).background     // C 94
	    { *bg = (*xftface_info).xft_bg; bg_done = true; }
	  else if xgcv.background == (*face).foreground     // C 96
	    { *bg = (*xftface_info).xft_fg; bg_done = true; }
	}

      if ! (fg_done & bg_done)                              // C 100
	{
	  let mut colors: [XColor; 2] = std::mem::zeroed ();  // C 102

	  colors[0].pixel = xgcv.foreground;                // C 104
	  if ! bg.is_null ()                                // C 105
	    { colors[1].pixel = xgcv.background; }          // C 106
	  x_query_colors (f as *mut frame, colors.as_mut_ptr (),   // C 107
			  if ! bg.is_null () { 2 } else { 1 });
	  (*fg).color.alpha = 0xFFFF;                       // C 108
	  (*fg).color.red = colors[0].red;                  // C 109
	  (*fg).color.green = colors[0].green;              // C 110
	  (*fg).color.blue = colors[0].blue;                // C 111

	  if XftColorAllocValue (FRAME_X_DISPLAY! ((f as *mut frame)),   // C 113-116
				 FRAME_X_VISUAL! ((f as *mut frame)),
				 FRAME_X_COLORMAP! ((f as *mut frame)),
				 &raw const (*fg).color, fg) == 0
	    /* This color should've been allocated when creating the GC.  (C 117-118)  */
	    { emacs_abort (); }                             // C 119
	  else
	    { *fg_allocated_p = true; }                      // C 121

	  if ! bg.is_null ()                                // C 123
	    {
	      (*bg).color.alpha = 0xFFFF;                   // C 125
	      (*bg).color.red = colors[1].red;              // C 126
	      (*bg).color.green = colors[1].green;          // C 127
	      (*bg).color.blue = colors[1].blue;            // C 128

	      if XftColorAllocValue (FRAME_X_DISPLAY! ((f as *mut frame)),   // C 130-133
				     FRAME_X_VISUAL! ((f as *mut frame)),
				     FRAME_X_COLORMAP! ((f as *mut frame)),
				     &raw const (*bg).color, bg) == 0
		{ emacs_abort (); }                         // C 134
	      else
		{ *bg_allocated_p = true; }                  // C 136
	    }
	}
      unblock_input ();                                    // C 139
    }
}

/* C 143-153: the two one-line driver entry points.  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_list (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{                                                          // C 143-147
  return ftfont_list2 (f, spec, Qxft ());                  // C 146
}

#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_match (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{                                                          // C 149-153
  return ftfont_match2 (f, spec, Qxft ());                 // C 152
}

/* C 157-339: `xftfont_open`.  C's uninitialised locals take the port's zero (PORTING.md 1.5):
   `result`, `extents`, the two `XftColor`s of C 510 and the four `b1`/`b2`/`i1`/`i2` of C 676
   (Xft and Fontconfig write them only on a match).  C's `match` is a Rust keyword, so the local
   keeps C's name in its raw spelling `r#match` (the `font_driver.r#match` convention of
   `include/font.rs`).  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_open (f: *mut c_void, entity: Lisp_Object, pixel_size: c_int)
                                   -> Lisp_Object
{                                                          // C 157-339
  let mut result: FcResult = 0;                            // C 160
  let display: *mut Display = FRAME_X_DISPLAY! ((f as *mut frame));       // C 161
  let mut val: Lisp_Object;                                // C 162
  let mut filename: Lisp_Object;
  let mut font_object: Lisp_Object;
  let mut pat: *mut FcPattern = std::ptr::null_mut ();     // C 163
  let mut r#match: *mut FcPattern;
  let mut xftfont_info: *mut font_info = std::ptr::null_mut ();           // C 164
  let mut font: *mut font;                                 // C 165
  let mut size: c_double = 0.0;                            // C 166
  let mut xftfont: *mut XftFont = std::ptr::null_mut ();   // C 167
  let mut spacing: c_int;                                  // C 168
  let mut extents: XGlyphInfo = std::mem::zeroed ();       // C 169
  let mut ft_face: FT_Face;                                // C 170
  let mut matrix: *mut FcMatrix = std::ptr::null_mut ();   // C 171 (C leaves it for the getter)

  val = assq_no_quit (QCfont_entity (), AREF (entity, FONT_EXTRA_INDEX as isize));   // C 173
  if ! CONSP (val)                                         // C 174
    { return Qnil (); }                                    // C 175
  val = XCDR (val);                                        // C 176
  filename = XCAR (val);                                   // C 177
  size = XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) as c_double;    // C 178
  if size == 0.0                                           // C 179
    { size = pixel_size as c_double; }                     // C 180

  block_input ();                                          // C 182

  pat = ftfont_entity_pattern (entity, pixel_size) as *mut FcPattern;   // C 184
  /* Substitute in values from X resources and XftDefaultSet.  (C 185)  */
  XftDefaultSubstitute (display, FRAME_X_SCREEN_NUMBER! (f as *mut frame), pat);   // C 186
  r#match = XftFontMatch (display, FRAME_X_SCREEN_NUMBER! (f as *mut frame), pat,
			  &raw mut result);                // C 187
  ftfont_fix_match (pat as *mut c_void, r#match as *mut c_void);   // C 188

  FcPatternDestroy (pat);                                  // C 190
  xftfont = XftFontOpenPattern (display, r#match);         // C 191
  if xftfont.is_null ()                                    // C 192
    {
      unblock_input ();                                    // C 194
      XftPatternDestroy! (r#match);                        // C 195
      return Qnil ();                                      // C 196
    }
  ft_face = XftLockFace (xftfont);                         // C 198
  unblock_input ();                                        // C 199

  /* We should not destroy PAT here because it is kept in XFTFONT and destroyed automatically
     when XFTFONT is closed.  (C 201-202)  */
  font_object = font_build_object (VECSIZE (std::mem::size_of::<font_info> ()) as c_int,
				   AREF (entity, FONT_TYPE_INDEX as isize),
				   entity, size);          // C 203-205
  ASET (font_object, FONT_FILE_INDEX as isize, filename);  // C 206
  font = XFONT_OBJECT (font_object);                       // C 207
  (*font).pixel_size = size as c_int;                      // C 208
  /* C 209-215: `#ifdef HAVE_HARFBUZZ` / the `xfthbfont_driver` choice / `#endif`, then the
     `xftfont_driver` assignment; both arms are carried as cfg-selected statements (P-51).  */
  #[cfg (feature = "harfbuzz")]
  if EQ (AREF (font_object, FONT_TYPE_INDEX as isize), Qxfthb ())       // C 210
    { (*font).driver = std::ptr::addr_of! (xfthbfont_driver) as *const c_void; }   // C 211
  else
    { (*font).driver = std::ptr::addr_of! (xftfont_driver) as *const c_void; }     // C 214
  #[cfg (not (feature = "harfbuzz"))]
  {
    (*font).driver = std::ptr::addr_of! (xftfont_driver) as *const c_void;         // C 214
  }
  (*font).encoding_charset = -1;                           // C 215
  (*font).repertory_charset = -1;                          // C 215

  xftfont_info = font as *mut font_info;                   // C 217
  (*xftfont_info).display = display;                       // C 218
  (*xftfont_info).xftfont = xftfont;                       // C 219
  (*xftfont_info).x_display_id = (*FRAME_DISPLAY_INFO! (f as *mut frame)).x_id;   // C 220
  /* This means that there's no need of transformation.  (C 221)  */
  (*xftfont_info).matrix.xx = 0;                           // C 222
  if FcPatternGetMatrix ((*xftfont).pattern, FC_MATRIX.as_ptr (), 0,
			 &raw mut matrix) == FcResultMatch       // C 223-224
    {
      (*xftfont_info).matrix.xx = (0x10000 as c_double * (*matrix).xx) as FT_Fixed;   // C 226
      (*xftfont_info).matrix.yy = (0x10000 as c_double * (*matrix).yy) as FT_Fixed;   // C 227
      (*xftfont_info).matrix.xy = (0x10000 as c_double * (*matrix).xy) as FT_Fixed;   // C 228
      (*xftfont_info).matrix.yx = (0x10000 as c_double * (*matrix).yx) as FT_Fixed;   // C 229
    }
  if FIXNUMP (AREF (entity, FONT_SPACING_INDEX as isize))   // C 231
    { spacing = XFIXNUM (AREF (entity, FONT_SPACING_INDEX as isize)) as c_int; }   // C 232
  else
    { spacing = FC_PROPORTIONAL; }                         // C 234
  if ascii_printable[0] == 0                               // C 235
    {
      let mut ch: c_int;                                   // C 237
      ch = 0;
      while ch < 95                                        // C 238
	{
	  ascii_printable[ch as usize] = (b' ' as c_int + ch) as FcChar8;   // C 239
	  ch += 1;
	}
    }
  block_input ();                                          // C 241

  /* Unfortunately Xft doesn't provide a way to get minimum char width.  So, we set min_width to
     space_width.  (C 243-244)
     C 246-252's `#ifdef FC_DUAL` / `#ifdef FC_CHARCELL` are two further conjuncts of one
     condition; each is carried as a cfg-selected statement (P-51) that folds it into the value
     the `if` below tests, so both arms stay in the text and the frozen arm is C's `ifdef`-less
     one.  */
  let mut spacing_not_proportional: bool = spacing != FC_PROPORTIONAL;   // C 246
  #[cfg (feature = "fc_dual")]
  {
    spacing_not_proportional = spacing_not_proportional && (spacing != FC_DUAL);   // C 248
  }
  #[cfg (feature = "fc_charcell")]
  {
    spacing_not_proportional = spacing_not_proportional && (spacing != FC_CHARCELL);   // C 251
  }

  if spacing_not_proportional                              // C 246-253
    {
      (*font).space_width = (*xftfont).max_advance_width;   // C 255-256
      (*font).average_width = (*font).space_width;
      (*font).max_width = (*font).average_width;
      (*font).min_width = (*font).max_width;
      XftTextExtents8 (display, xftfont,
		       (&raw mut ascii_printable).cast::<FcChar8> ().offset (1), 94,
		       &raw mut extents);                   // C 257
    }
  else
    {
      XftTextExtents8 (display, xftfont,
		       (&raw mut ascii_printable).cast::<FcChar8> (), 1, &raw mut extents);   // C 261
      (*font).space_width = extents.xOff as c_int;          // C 262-263
      (*font).max_width = (*font).space_width;
      (*font).min_width = (*font).max_width;
      if (*font).space_width <= 0                           // C 264
	/* dirty workaround (C 265)  */
	{ (*font).space_width = pixel_size; }               // C 266
      XftTextExtents8 (display, xftfont,
		       (&raw mut ascii_printable).cast::<FcChar8> ().offset (1), 94,
		       &raw mut extents);                   // C 267
      (*font).average_width = ((*font).space_width + extents.xOff as c_int) / 95;   // C 268
    }
  unblock_input ();                                        // C 270

  (*font).ascent = (*xftfont).ascent;                      // C 272
  (*font).descent = (*xftfont).descent;                    // C 273
  /* The following workaround is unnecessary on most systems … (C 274-280)  */
  if pixel_size >= 5                                       // C 281
      && xft_font_ascent_descent_override                  // C 282
    {
      /* The above condition is a dirty workaround because XftTextExtents8 behaves strangely for
	 some fonts (e.g. "Dejavu Sans Mono") when pixel_size is less than 5.  (C 284-286)  */
      if (*font).ascent < extents.y as c_int               // C 287
	{ (*font).ascent = extents.y as c_int; }            // C 288
      if (*font).descent < extents.height as c_int - extents.y as c_int   // C 289
	{ (*font).descent = extents.height as c_int - extents.y as c_int; }   // C 290
    }
  (*font).height = (*font).ascent + (*font).descent;       // C 292

  if XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) == 0    // C 294
    {
      let upEM: c_int = (*ft_face).units_per_EM as c_int;  // C 296

      (*font).underline_position
	= (-((*ft_face).underline_position as c_double) * size / upEM as c_double) as c_int;   // C 298
      (*font).underline_thickness
	= (((*ft_face).underline_thickness as c_double) * size / upEM as c_double) as c_int;   // C 299
      if (*font).underline_thickness > 2                   // C 300
	{ (*font).underline_position -= (*font).underline_thickness / 2; }   // C 301
    }
  else
    {
      (*font).underline_position = -1;                     // C 305
      (*font).underline_thickness = 0;                     // C 306
    }
  #[cfg (feature = "have_libotf")]
  {
    (*xftfont_info).maybe_otf = ((*ft_face).face_flags & FT_FACE_FLAG_SFNT) != 0;   // C 309
    (*xftfont_info).otf = std::ptr::null_mut ();           // C 310
  }
  #[cfg (feature = "harfbuzz")]
  {
    (*xftfont_info).hb_font = std::ptr::null_mut ();       // C 313
  }
  (*xftfont_info).ft_size = (*ft_face).size;               // C 315

  (*font).baseline_offset = 0;                             // C 317
  (*font).relative_compose = 0;                            // C 318
  (*font).default_ascent = 0;                              // C 319
  (*font).vertical_centering = false;                      // C 320
  #[cfg (feature = "ft_bdf_h")]
  if ((*ft_face).face_flags & FT_FACE_FLAG_SFNT) == 0      // C 322
    {
      let mut rec: BDF_PropertyRec = std::mem::zeroed ();  // C 324

      if FT_Get_BDF_Property (ft_face, c"_MULE_BASELINE_OFFSET".as_ptr (),
			      &raw mut rec) == 0           // C 326-327
	  && rec.r#type == BDF_PROPERTY_TYPE_INTEGER
	{ (*font).baseline_offset = rec.u.integer; }        // C 328
      if FT_Get_BDF_Property (ft_face, c"_MULE_RELATIVE_COMPOSE".as_ptr (),
			      &raw mut rec) == 0           // C 329-330
	  && rec.r#type == BDF_PROPERTY_TYPE_INTEGER
	{ (*font).relative_compose = rec.u.integer; }       // C 331
      if FT_Get_BDF_Property (ft_face, c"_MULE_DEFAULT_ASCENT".as_ptr (),
			      &raw mut rec) == 0           // C 332-333
	  && rec.r#type == BDF_PROPERTY_TYPE_INTEGER
	{ (*font).default_ascent = rec.u.integer; }         // C 334
    }

  return font_object;                                      // C 338
}

/* C 341-373: `xftfont_close`.  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_close (font: *mut font)
{                                                          // C 341-373
  let mut xdi: *mut x_display_info;                        // C 344
  let xftfont_info: *mut font_info = font as *mut font_info;   // C 345

  #[cfg (feature = "have_libotf")]
  if ! (*xftfont_info).otf.is_null ()                      // C 348
    {
      OTF_close ((*xftfont_info).otf);                     // C 350
      (*xftfont_info).otf = std::ptr::null_mut ();         // C 351
    }
  #[cfg (feature = "harfbuzz")]
  if ! (*xftfont_info).hb_font.is_null ()                  // C 355
    {
      hb_font_destroy ((*xftfont_info).hb_font);           // C 357
      (*xftfont_info).hb_font = std::ptr::null_mut ();     // C 358
    }

  /* See comment in xfont_close.  (C 362)  */
  if ! (*xftfont_info).xftfont.is_null ()                  // C 363
      && { xdi = x_display_info_for_display ((*xftfont_info).display);
	   ! xdi.is_null () && (*xftfont_info).x_display_id == (*xdi).x_id }   // C 364-365
    {
      block_input ();                                      // C 367
      XftUnlockFace ((*xftfont_info).xftfont);             // C 368
      XftFontClose ((*xftfont_info).display, (*xftfont_info).xftfont);   // C 369
      unblock_input ();                                    // C 370
      (*xftfont_info).xftfont = std::ptr::null_mut ();     // C 371
    }
}

/* C 375-396: `xftfont_prepare_face`.  C 380-387's `#if false` block is dead in every
   configuration, so it is recorded here rather than transcribed (P-08):
     `/* This doesn't work if face->ascii_face doesn't use an Xft font. */`
     `if (face != face->ascii_face) { face->extra = face->ascii_face->extra; return; }`  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_prepare_face (f: *mut c_void, face: *mut c_void)
{                                                          // C 375-396
  let face: *mut face = face as *mut face;
  let xftface_info: *mut xftface_info;                     // C 378

  xftface_info = xzalloc (std::mem::size_of::<xftface_info> ()) as *mut xftface_info;   // C 389
  xftfont_get_colors (f, face as *mut c_void, (*face).gc, std::ptr::null_mut (),   // C 390-394
		      &raw mut (*xftface_info).xft_fg,
		      &raw mut (*xftface_info).xft_bg,
		      &raw mut (*xftface_info).bg_allocated_p,
		      &raw mut (*xftface_info).fg_allocated_p);
  (*face).extra = xftface_info as *mut c_void;             // C 395
}

/* C 398-428: `xftfont_done_face`.  C 403-408's `#if false` block is recorded the same way:
     `/* This doesn't work if face->ascii_face doesn't use an Xft font. */`
     `if (face != face->ascii_face || ! face->extra) return;`  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_done_face (f: *mut c_void, face: *mut c_void)
{                                                          // C 398-428
  let face: *mut face = face as *mut face;
  let xftface_info: *mut xftface_info;                     // C 401

  xftface_info = (*face).extra as *mut xftface_info;       // C 410
  if ! xftface_info.is_null ()                             // C 411
    {
      if (*xftface_info).fg_allocated_p                    // C 413
	{
	  XftColorFree (FRAME_X_DISPLAY! (f as *mut frame),   // C 414-417
			FRAME_X_VISUAL! (f as *mut frame),
			FRAME_X_COLORMAP! (f as *mut frame),
			&raw mut (*xftface_info).xft_fg);
	}

      if (*xftface_info).bg_allocated_p                    // C 419
	{
	  XftColorFree (FRAME_X_DISPLAY! (f as *mut frame),   // C 420-423
			FRAME_X_VISUAL! (f as *mut frame),
			FRAME_X_COLORMAP! (f as *mut frame),
			&raw mut (*xftface_info).xft_bg);
	}

      xfree (xftface_info as *mut c_void);                 // C 425
      (*face).extra = std::ptr::null_mut ();               // C 426
    }
}

/* C 430-450: `xftfont_has_char`.  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_has_char (font: Lisp_Object, c: c_int) -> c_int
{                                                          // C 430-450
  let xftfont_info: *mut font_info;                        // C 433
  let mut cs: *mut charset = std::ptr::null_mut ();        // C 434

  if EQ (AREF (font, FONT_ADSTYLE_INDEX as isize), Qja ())       // C 436
      && charset_jisx0208 >= 0                             // C 437
    { cs = CHARSET_FROM_ID! (charset_jisx0208); }          // C 438
  else if EQ (AREF (font, FONT_ADSTYLE_INDEX as isize), Qko ())  // C 439
      && charset_ksc5601 >= 0                              // C 440
    { cs = CHARSET_FROM_ID! (charset_ksc5601); }           // C 441
  if ! cs.is_null ()                                       // C 442
    { return (ENCODE_CHAR! (cs, c) != CHARSET_INVALID_CODE! (cs)) as c_int; }   // C 443

  if FONT_ENTITY_P (font)                                  // C 445
    { return ftfont_has_char (font, c); }                  // C 446
  xftfont_info = XFONT_OBJECT (font) as *mut font_info;    // C 447
  return (XftCharExists ((*xftfont_info).display, (*xftfont_info).xftfont,
			 c as FcChar32) == FcTrue) as c_int;   // C 448-449
}

/* C 452-460: `xftfont_encode_char`.  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_encode_char (font: *mut font, c: c_int) -> c_uint
{                                                          // C 452-460
  let xftfont_info: *mut font_info = font as *mut font_info;   // C 455
  let code: c_uint = XftCharIndex ((*xftfont_info).display, (*xftfont_info).xftfont,
				   c as FcChar32);         // C 456-457

  return if code != 0 { code } else { FONT_INVALID_CODE };  // C 459
}

/* C 462-477: `xftfont_text_extents`.  C's `struct font_metrics *metrics` member types are
   `short` (`include/font.rs` 362-374), so the five assignments narrow with C's own truncation.  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_text_extents (font: *mut font, code: *const c_uint, nglyphs: c_int,
                                           metrics: *mut font_metrics)
{                                                          // C 462-477
  let xftfont_info: *mut font_info = font as *mut font_info;   // C 466
  let mut extents: XGlyphInfo = std::mem::zeroed ();       // C 467

  XftGlyphExtents ((*xftfont_info).display, (*xftfont_info).xftfont, code, nglyphs,
		   &raw mut extents);                      // C 469-470

  (*metrics).lbearing = (- (extents.x as c_int)) as c_short;   // C 472
  (*metrics).rbearing = (- (extents.x as c_int) + extents.width as c_int) as c_short;   // C 473
  (*metrics).width = extents.xOff;                         // C 474
  (*metrics).ascent = extents.y;                           // C 475
  (*metrics).descent = (extents.height as c_int - extents.y as c_int) as c_short;   // C 476
}

/* C 479-496: `xftfont_get_xft_draw`.  */
#[cfg (feature = "have_xft")]
unsafe fn xftfont_get_xft_draw (f: *mut c_void) -> XftDraw
{                                                          // C 479-496
  let mut xft_draw: XftDraw
    = font_get_frame_data (f as *mut frame, Qxft ()) as XftDraw;   // C 482

  if xft_draw.is_null ()                                   // C 484
    {
      block_input ();                                      // C 486
      xft_draw = XftDrawCreate (FRAME_X_DISPLAY! (f as *mut frame),   // C 487-490
				FRAME_X_DRAWABLE! (f as *mut frame),
				FRAME_X_VISUAL! (f as *mut frame),
				FRAME_X_COLORMAP! (f as *mut frame));
      unblock_input ();                                    // C 491
      eassert! (! xft_draw.is_null ());                    // C 492
      font_put_frame_data (f as *mut frame, Qxft (), xft_draw as *mut c_void);   // C 493
    }
  return xft_draw;                                         // C 495
}

/* C 498-610: `xftfont_draw`.  C 546-578's `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 ||
   (RENDER_MINOR >= 2))` arm is an `if`/`else` pair whose `else` clause C keeps outside the
   `#if`; both arms are carried as two cfg-selected statements (P-51), which is why the
   `XftDrawRect` line stands twice in this text (the `font_data_structures_may_be_ill_formed`
   shape of `include/font.rs`).  `s->padding_p` is `glyph_string_bits!`'s arm for the record's
   bitfield (dispextern.h 1461, `include/dispextern.rs` 1828).  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_draw (s: *mut glyph_string, from: c_int, to: c_int, x: c_int,
                                   y: c_int, with_background: bool) -> c_int
{                                                          // C 498-610
  block_input ();                                          // C 502

  let f: *mut frame = (*s).f;                              // C 504
  let face: *mut face = (*s).face as *mut face;            // C 505
  let xftfont_info: *mut font_info = (*s).font as *mut font_info;   // C 506
  let mut xftface_info: *mut xftface_info = std::ptr::null_mut ();  // C 507
  let xft_draw: XftDraw = xftfont_get_xft_draw (f as *mut c_void);  // C 508
  let mut code: *mut FT_UInt;                              // C 509
  let mut fg: XftColor = std::mem::zeroed ();              // C 510
  let mut bg: XftColor = std::mem::zeroed ();
  let mut bg_allocated_p: bool = false;                    // C 511
  let mut fg_allocated_p: bool = false;
  let len: c_int = to - from;                              // C 512
  let mut i: c_int;                                        // C 513
  let mut with_background: bool = with_background;         // C 518/C 571 (the body writes it)

  if (*s).font == (*face).font                             // C 515
    { xftface_info = (*face).extra as *mut xftface_info; }   // C 516
  xftfont_get_colors (f as *mut c_void, face as *mut c_void, (*s).gc as GC, xftface_info,
		      &raw mut fg,                         // C 517-519
		      if with_background { &raw mut bg } else { std::ptr::null_mut () },
		      &raw mut bg_allocated_p, &raw mut fg_allocated_p);

  if (*s).num_clips > 0                                    // C 521
    {
      XftDrawSetClipRectangles (xft_draw, 0, 0,            // C 522
				(*s).clip.as_ptr () as *const XRectangle, (*s).num_clips);
    }
  else
    { XftDrawSetClip (xft_draw, std::ptr::null_mut ()); }   // C 524

  if with_background                                       // C 526
    {
      let mut height: c_int = FONT_HEIGHT! ((*s).font);     // C 528
      let mut ascent: c_int = FONT_BASE! ((*s).font);

      /* Font's global height and ascent values might be preposterously large for some fonts.  We
	 fix here the case when those fonts are used for display of glyphless characters, because
	 drawing background with font dimensions in those cases makes the display illegible.  There's
	 only one more call to the draw method with with_background set to true, and that's in
	 x_draw_glyph_string_foreground, when drawing the cursor, where we have no such heuristics
	 available.  FIXME.  (C 530-538)  */
      let glyph: *mut glyph = (*s).first_glyph;            // C 539
      if glyph_bits! (type, (*glyph)) == GLYPHLESS_GLYPH  // C 539
	  && (glyphless_bits! (method, glyph)
	      == dispextern_unit_names::GLYPHLESS_DISPLAY_HEX_CODE           // C 540
	      || glyphless_bits! (method, glyph)
		 == dispextern_unit_names::GLYPHLESS_DISPLAY_ACRONYM)        // C 541
	{
	  ascent = ((*glyph).slice.glyphless.lower_yoff as c_int
		    - (*glyph).slice.glyphless.upper_yoff as c_int);   // C 542-544
	  height = ascent;                                     // C 542
	}

      #[cfg (all (feature = "have_xrender", feature = "render_version_ge_0_2"))]
      if with_background                                    // C 547
	  && (*FRAME_DISPLAY_INFO! ((*s).f)).alpha_bits != 0   // C 548
	  && FRAME_CHECK_XR_VERSION! ((*s).f, 0, 2)          // C 549
	{
	  use crate::x11_constants::None;                    // X.h 115's `None`, the mirror's home

	  x_xr_ensure_picture ((*s).f);                      // C 551

	  if FRAME_X_PICTURE! ((*s).f) != None as Picture    // C 553
	    {
	      let mut xc: XRenderColor = std::mem::zeroed ();   // C 555
	      let height: c_int = FONT_HEIGHT! ((*s).font);   // C 556
	      let ascent: c_int = FONT_BASE! ((*s).font);

	      if (*s).num_clips > 0                          // C 558
		{
		  XRenderSetPictureClipRectangles (FRAME_X_DISPLAY! ((*s).f),   // C 559-561
						  FRAME_X_PICTURE! ((*s).f),
						  0, 0,
						  (*s).clip.as_ptr () as *const XRectangle,
						  (*s).num_clips);
		}
	      else
		{ x_xr_reset_ext_clip (f); }                    // C 563
	      x_xrender_color_from_gc_background ((*s).f, (*s).gc as GC, &raw mut xc,
						  (*s).hl != DRAW_CURSOR);   // C 564
	      XRenderFillRectangle (FRAME_X_DISPLAY! ((*s).f),   // C 565-567
				    PictOpSrc, FRAME_X_PICTURE! ((*s).f),
				    &raw const xc, x, y - ascent,
				    (*s).width as c_uint, height as c_uint);
	      x_xr_reset_ext_clip (f);                          // C 568
	      x_mark_frame_dirty ((*s).f);                      // C 569

	      with_background = false;                          // C 571
	    }
	  else
	    {
	      XftDrawRect (xft_draw, &raw const bg, x, y - ascent,   // C 574
			   (*s).width as c_uint, height as c_uint);
	    }
	}
      else
	{
	  XftDrawRect (xft_draw, &raw const bg, x, y - ascent, (*s).width as c_uint,   // C 578
		       height as c_uint);
	}
      /* The `#if`'s arm is off: C reaches C 578 directly, without the `if` above.  This is the
	 second cfg-selected statement of the pair (P-51).  */
      #[cfg (not (all (feature = "have_xrender", feature = "render_version_ge_0_2")))]
      {
	XftDrawRect (xft_draw, &raw const bg, x, y - ascent, (*s).width as c_uint,   // C 578
		     height as c_uint);
      }
    }
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  code = SAFE_ALLOCA! (sa_avail,
		       std::mem::size_of::<FT_UInt> () * len as usize) as *mut FT_UInt;   // C 580
  i = 0;                                                   // C 581
  while i < len
    {
      *code.offset (i as isize) = *(*s).char2b.offset ((from + i) as isize);   // C 583
      i += 1;
    }

  if glyph_string_bits! (padding_p, (*s))                  // C 584
    {
      i = 0;
      while i < len
	{
	  XftDrawGlyphs (xft_draw, &raw const fg, (*xftfont_info).xftfont,   // C 586-587
			 x + i, y, code.offset (i as isize), 1);
	  i += 1;
	}
    }
  else
    {
      XftDrawGlyphs (xft_draw, &raw const fg, (*xftfont_info).xftfont,   // C 589-590
		     x, y, code, len);
    }
  /* Need to explicitly mark the frame dirty because we didn't call FRAME_X_DRAWABLE in order to
     draw: we cached the drawable in the XftDraw structure.  (C 591-593)  */
  x_mark_frame_dirty (f);                                  // C 594

  if bg_allocated_p                                        // C 596
    {
      XftColorFree (FRAME_X_DISPLAY! (f), FRAME_X_VISUAL! (f),   // C 597-600
		    FRAME_X_COLORMAP! (f), &raw mut bg);
    }

  if fg_allocated_p                                        // C 602
    {
      XftColorFree (FRAME_X_DISPLAY! (f), FRAME_X_VISUAL! (f),   // C 603-606
		    FRAME_X_COLORMAP! (f), &raw mut fg);
    }

  SAFE_FREE! (sa_count);
  unblock_input ();                                        // C 608
  return len;                                              // C 609
}

/* C 612-624: `#if defined HAVE_M17N_FLT && defined HAVE_LIBOTF` / `xftfont_shape` / `#endif`.  */
#[cfg (all (feature = "have_xft", feature = "have_m17n_flt", feature = "have_libotf"))]
unsafe extern "C" fn xftfont_shape (lgstring: Lisp_Object, direction: Lisp_Object) -> Lisp_Object
{                                                          // C 612-624
  let font: *mut font = CHECK_FONT_GET_OBJECT (LGSTRING_FONT (lgstring));   // C 616
  let xftfont_info: *mut font_info = font as *mut font_info;   // C 617
  let ft_face: FT_Face = XftLockFace ((*xftfont_info).xftfont);   // C 618
  (*xftfont_info).ft_size = (*ft_face).size;               // C 619
  let val: Lisp_Object = ftfont_shape (lgstring, direction);   // C 620
  XftUnlockFace ((*xftfont_info).xftfont);                 // C 621
  return val;                                              // C 622
}

/* C 626-648: `xftfont_end_for_frame`.  C's `if (!FRAME_X_DISPLAY (f))` is a null test on a
   pointer, which Rust spells `is_null ()` (the `MACRO_TOKEN_EQUIV` `null` family).  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_end_for_frame (f: *mut c_void) -> c_int
{                                                          // C 626-648
  /* XftDrawDestroy tries to access dpyinfo->display, which could've been destroyed by now,
     causing the program to crash.  The alternative is to leak the XftDraw, but that's better
     than a crash.  (C 629-631)  */
  if FRAME_X_DISPLAY! (f as *mut frame).is_null ()         // C 632
    { return 0; }                                          // C 633

  block_input ();                                          // C 635
  let mut xft_draw: XftDraw;                               // C 636

  xft_draw = font_get_frame_data (f as *mut frame, Qxft ()) as XftDraw;   // C 638
  if ! xft_draw.is_null ()                                 // C 639
    {
      block_input ();                                      // C 641
      XftDrawDestroy (xft_draw);                           // C 642
      unblock_input ();                                    // C 643
      font_put_frame_data (f as *mut frame, Qxft (), std::ptr::null_mut ());   // C 644
    }
  unblock_input ();                                        // C 646
  return 0;                                                // C 647
}

/* C 650-666: the comment at C 650-653 and, under `#ifdef HAVE_XDBE`, `xftfont_drop_xrender_surfaces`.  */
#[cfg (all (feature = "have_xft", feature = "have_xdbe"))]
unsafe extern "C" fn xftfont_drop_xrender_surfaces (f: *mut c_void)
{                                                          // C 655-666
  if FRAME_X_DOUBLE_BUFFERED_P! (f as *mut frame)          // C 659
    {
      block_input ();                                      // C 661
      xftfont_end_for_frame (f);                           // C 662
      unblock_input ();                                    // C 663
    }
}

/* C 668-711: `xftfont_cached_font_ok`.  C's `goto out` and `out:` label are the port's labelled
   block plus `break 'out` (PORTING.md 2.2, the `ftfont.rs` `'err` shape); the statements after
   the label (C 709-710) follow the block, so both paths run them.  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn xftfont_cached_font_ok (f: *mut c_void, font_object: Lisp_Object,
                                             entity: Lisp_Object) -> bool
{                                                          // C 668-711
  let info: *mut font_info = XFONT_OBJECT (font_object) as *mut font_info;   // C 672
  let oldpat: *mut FcPattern = (*(*info).xftfont).pattern;   // C 673
  let display: *mut Display = FRAME_X_DISPLAY! (f as *mut frame);   // C 674
  let pat: *mut FcPattern = FcPatternCreate ();            // C 675
  let mut b1: FcBool = 0;                                  // C 676 (C leaves them for the getters)
  let mut b2: FcBool = 0;
  let mut ok: bool = false;                                // C 677
  let mut i1: c_int = 0;                                   // C 678
  let mut i2: c_int = 0;
  let mut r1: c_int = 0;
  let mut r2: c_int = 0;

  ftfont_add_rendering_parameters (pat as *mut c_void, entity);   // C 680
  XftDefaultSubstitute (display, FRAME_X_SCREEN_NUMBER! (f as *mut frame), pat);   // C 681

  'out: {                                                  // C 685-708 (C's `out:` label)
    r1 = FcPatternGetBool (pat, FC_ANTIALIAS.as_ptr (), 0, &raw mut b1);   // C 683
    r2 = FcPatternGetBool (oldpat, FC_ANTIALIAS.as_ptr (), 0, &raw mut b2);   // C 684
    if r1 != r2 || b1 != b2 { break 'out; }                // C 685
    r1 = FcPatternGetBool (pat, FC_HINTING.as_ptr (), 0, &raw mut b1);   // C 686
    r2 = FcPatternGetBool (oldpat, FC_HINTING.as_ptr (), 0, &raw mut b2);   // C 687
    if r1 != r2 || b1 != b2 { break 'out; }                // C 688
    r1 = FcPatternGetBool (pat, FC_AUTOHINT.as_ptr (), 0, &raw mut b1);   // C 689
    r2 = FcPatternGetBool (oldpat, FC_AUTOHINT.as_ptr (), 0, &raw mut b2);   // C 690
    if r1 != r2 || b1 != b2 { break 'out; }                // C 691
    #[cfg (feature = "fc_embolden")]
    {
      r1 = FcPatternGetBool (pat, FC_EMBOLDEN.as_ptr (), 0, &raw mut b1);   // C 693
      r2 = FcPatternGetBool (oldpat, FC_EMBOLDEN.as_ptr (), 0, &raw mut b2);   // C 694
      if r1 != r2 || b1 != b2 { break 'out; }              // C 695
    }
    r1 = FcPatternGetInteger (pat, FC_HINT_STYLE.as_ptr (), 0, &raw mut i1);   // C 697
    r2 = FcPatternGetInteger (oldpat, FC_HINT_STYLE.as_ptr (), 0, &raw mut i2);   // C 698
    if r1 != r2 || i1 != i2 { break 'out; }                // C 699
    r1 = FcPatternGetInteger (pat, FC_LCD_FILTER.as_ptr (), 0, &raw mut i1);   // C 700
    r2 = FcPatternGetInteger (oldpat, FC_LCD_FILTER.as_ptr (), 0, &raw mut i2);   // C 701
    if r1 != r2 || i1 != i2 { break 'out; }                // C 702
    r1 = FcPatternGetInteger (pat, FC_RGBA.as_ptr (), 0, &raw mut i1);   // C 703
    r2 = FcPatternGetInteger (oldpat, FC_RGBA.as_ptr (), 0, &raw mut i2);   // C 704
    if r1 != r2 || i1 != i2 { break 'out; }                // C 705

    ok = true;                                             // C 707
  }
  FcPatternDestroy (pat);                                  // C 709
  return ok;                                               // C 710
}

/* C 713-746: `#ifdef HAVE_HARFBUZZ` / the four Xft-HarfBuzz entry points / `#endif`.  */
#[cfg (all (feature = "have_xft", feature = "harfbuzz"))]
unsafe extern "C" fn xfthbfont_list (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{                                                          // C 715-719
  return ftfont_list2 (f, spec, Qxfthb ());                // C 718
}

#[cfg (all (feature = "have_xft", feature = "harfbuzz"))]
unsafe extern "C" fn xfthbfont_match (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{                                                          // C 721-725
  return ftfont_match2 (f, spec, Qxfthb ());               // C 724
}

#[cfg (all (feature = "have_xft", feature = "harfbuzz"))]
unsafe extern "C" fn xfthbfont_begin_hb_font (font: *mut font, position_unit: *mut c_double)
                                              -> hb_font_t
{                                                          // C 727-736
  let xftfont_info: *mut font_info = font as *mut font_info;   // C 730
  let ft_face: FT_Face = XftLockFace ((*xftfont_info).xftfont);   // C 731

  (*xftfont_info).ft_size = (*ft_face).size;               // C 733

  return fthbfont_begin_hb_font (font, position_unit);     // C 735
}

#[cfg (all (feature = "have_xft", feature = "harfbuzz"))]
unsafe extern "C" fn xfthbfont_end_hb_font (font: *mut font, hb_font: hb_font_t)
{                                                          // C 738-744
  let xftfont_info: *mut font_info = font as *mut font_info;   // C 741

  XftUnlockFace ((*xftfont_info).xftfont);                 // C 743
}

/* C 748: `static void syms_of_xftfont_for_pdumper (void);` - C's forward declaration has no Rust
   counterpart (PORTING.md 0.1-12); the definition follows at C 816 and `syms_of_xftfont` reaches
   it from there.  */

/// The `C` record: `struct font_driver const xftfont_driver` (C 750-784).  C's designated
/// initialiser names eighteen members and leaves the rest zero; Rust has no designated
/// initialisers, so the rest are written out (the `xfont.rs` 1666 shape).  `include/font.rs`'s
/// `font_driver` types the members C declares with a `struct frame *` or `struct face *`
/// parameter as untyped `*mut c_void` (its own note: `struct frame` need not be in an including
/// unit's graph), so the entries whose C parameter is `struct frame *` convert their function
/// pointer there - the P-72 rule 3 use-point conversion.
#[cfg (feature = "have_xft")]
#[unsafe(no_mangle)]
pub static mut xftfont_driver: font_driver = font_driver {   // C 750-784
  /* C 753: `.type = LISPSYM_INITIALLY (Qxft)`.  The mirror's `LISPSYM_INITIALLY` is not a
     `const fn`, so the port writes the same constant expression over the generated `iQxft`
     index (the `xfont.rs` 1669 precedent).  */
  r#type: (((iQxft * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	    + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),
  case_sensitive: false,
  get_cache: Some (xfont_get_cache),                       // C 754
  list: Some (xftfont_list),                               // C 755
  r#match: Some (xftfont_match),                           // C 756
  list_family: Some (ftfont_list_family),                  // C 757
  free_entity: None,
  open_font: Some (xftfont_open),                          // C 758
  close_font: Some (xftfont_close),                        // C 759
  prepare_face: Some (xftfont_prepare_face),               // C 760
  done_face: Some (xftfont_done_face),                     // C 761
  has_char: Some (xftfont_has_char),                       // C 762
  encode_char: Some (xftfont_encode_char),                 // C 763
  text_extents: Some (xftfont_text_extents),               // C 764
  #[cfg (feature = "have_window_system")]
  draw: Some (xftfont_draw),                               // C 765
  #[cfg (feature = "have_window_system")]
  get_bitmap: Some (ftfont_get_bitmap),                    // C 766
  #[cfg (feature = "have_window_system")]
  free_bitmap: None,
  anchor_point: Some (ftfont_anchor_point),                // C 767
  #[cfg (not (feature = "have_libotf"))]
  otf_capability: None,
  #[cfg (feature = "have_libotf")]
  otf_capability: Some (ftfont_otf_capability),            // C 769
  otf_drive: None,
  start_for_frame: None,
  end_for_frame: Some (xftfont_end_for_frame),             // C 771
  #[cfg (not (all (feature = "have_m17n_flt", feature = "have_libotf")))]
  shape: None,
  #[cfg (all (feature = "have_m17n_flt", feature = "have_libotf"))]
  shape: Some (xftfont_shape),                             // C 773
  check: None,
  #[cfg (not (any (feature = "have_otf_get_variation_glyphs",
		   feature = "have_ft_face_getcharvariantindex")))]
  get_variation_glyphs: None,
  #[cfg (any (feature = "have_otf_get_variation_glyphs",
	      feature = "have_ft_face_getcharvariantindex"))]
  get_variation_glyphs: Some (ftfont_variation_glyphs),    // C 776
  filter_properties: Some (ftfont_filter_properties),      // C 778
  cached_font_ok: Some (xftfont_cached_font_ok),           // C 779
  combining_capability: Some (ftfont_combining_capability),   // C 780
  #[cfg (not (feature = "have_xdbe"))]
  drop_xrender_surfaces: None,
  #[cfg (feature = "have_xdbe")]
  drop_xrender_surfaces: Some (xftfont_drop_xrender_surfaces),   // C 782
  #[cfg (feature = "harfbuzz")]
  begin_hb_font: None,
  #[cfg (feature = "harfbuzz")]
  end_hb_font: None,
};

/// C 785-787: `#ifdef HAVE_HARFBUZZ` / `struct font_driver xfthbfont_driver;` / `#endif` - C's
/// tentative definition of a second file-scope driver object, which `syms_of_xftfont_for_pdumper`
/// (C 821-830) fills from `xftfont_driver`.  Rust has no tentative definition, so the object is
/// written with C's zero value and its members are assigned where C assigns them
/// (PORTING.md 0.1-12's object half).
#[cfg (all (feature = "have_xft", feature = "harfbuzz"))]
#[unsafe(no_mangle)]
pub static mut xfthbfont_driver: font_driver = font_driver {   // C 786
  r#type: std::ptr::null_mut (),   /* C's zero: the record starts out all-zero */
  case_sensitive: false,
  get_cache: None,
  list: None,
  r#match: None,
  list_family: None,
  free_entity: None,
  open_font: None,
  close_font: None,
  prepare_face: None,
  done_face: None,
  has_char: None,
  encode_char: None,
  text_extents: None,
  #[cfg (feature = "have_window_system")]
  draw: None,
  #[cfg (feature = "have_window_system")]
  get_bitmap: None,
  #[cfg (feature = "have_window_system")]
  free_bitmap: None,
  anchor_point: None,
  otf_capability: None,
  otf_drive: None,
  start_for_frame: None,
  end_for_frame: None,
  shape: None,
  check: None,
  get_variation_glyphs: None,
  filter_properties: None,
  cached_font_ok: None,
  combining_capability: None,
  drop_xrender_surfaces: None,
  #[cfg (feature = "harfbuzz")]
  begin_hb_font: None,
  #[cfg (feature = "harfbuzz")]
  end_hb_font: None,
};

/* The storage this unit owns (docs/status.md 3.29): C's `DEFVAR_BOOL`/`DEFVAR_LISP` name a field
   of the single `globals` object (`lisp.h` 3495-3530), whose declaration the port mirrors in
   `include/globals.rs`'s `variables` module; the unit whose `syms_of_xftfont` registers the
   variable defines the object here, and its address must be stable for the forwarding records
   (the `buffer.rs` 4919 shape).  C zero-initialises the `globals` object, so the port's
   initialisers are C's own values: `xft-font-ascent-descent-override` is a `bool` (C 802 assigns
   0) and `xft-color-font-whitelist` is a `Lisp_Object` (C 811 assigns a list).  */
#[cfg (feature = "have_xft")]
#[unsafe(no_mangle)]
pub static mut xft_font_ascent_descent_override: bool = false;
#[cfg (feature = "have_xft")]
#[unsafe(no_mangle)]
pub static mut Vxft_color_font_whitelist: Lisp_Object = lisp_h_Qnil ();

/// C 789-814: `syms_of_xftfont`.  The two docstrings are C's own `doc:` comments, byte for byte
/// (the doc gate compares them with the reference).  */
#[cfg (feature = "have_xft")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xftfont ()
{                                                          // C 789-814
  DEFSYM! (Qxft, "xft");                                   // C 792
  #[cfg (feature = "harfbuzz")]
  {
    DEFSYM! (Qxfthb, "xfthb");                             // C 794
    Fput (Qxft (), Qfont_driver_superseded_by (), Qxfthb ());   // C 795
  }

  DEFVAR_BOOL! (c"xft-font-ascent-descent-override",        // C 798-801
		xft_font_ascent_descent_override,
		"Non-nil means override the ascent and descent values for Xft font driver.\nThis is needed with some fonts to correct vertical overlap of glyphs.  ");
  xft_font_ascent_descent_override = false;                // C 802

  DEFVAR_LISP! (c"xft-color-font-whitelist", Vxft_color_font_whitelist,   // C 804-810
		"List of \"color\" font families that don't actually have color glyphs.\nSome fonts (such as Source Code Pro) are reported as color fonts, but\ndo not actually have glyphs with colors that can cause Xft crashes.\n\nThe font families in this list will not be ignored when\n`xft-ignore-color-fonts' is non-nil.  ");
  Vxft_color_font_whitelist
    = list1 (build_string (c"Source Code Pro".as_ptr ()));   // C 811

  pdumper_do_now_and_after_load (Some (syms_of_xftfont_for_pdumper));   // C 813
}

/// C 816-832: `syms_of_xftfont_for_pdumper`.  C 821's `xfthbfont_driver = xftfont_driver;` is a
/// whole-record assignment of a `Copy` record, and the seven member writes that follow it are
/// C's own (C 822-829).  */
#[cfg (feature = "have_xft")]
unsafe extern "C" fn syms_of_xftfont_for_pdumper ()
{                                                          // C 816-832
  register_font_driver (std::ptr::addr_of! (xftfont_driver), std::ptr::null_mut ());   // C 819
  #[cfg (feature = "harfbuzz")]
  {
    xfthbfont_driver = std::ptr::read (std::ptr::addr_of! (xftfont_driver));   // C 821
    xfthbfont_driver.r#type = Qxfthb ();                    // C 822
    xfthbfont_driver.list = Some (xfthbfont_list);          // C 823
    xfthbfont_driver.r#match = Some (xfthbfont_match);      // C 824
    xfthbfont_driver.otf_capability = Some (hbfont_otf_capability);   // C 825
    xfthbfont_driver.shape = Some (hbfont_shape);           // C 826
    xfthbfont_driver.combining_capability = Some (hbfont_combining_capability);   // C 827
    xfthbfont_driver.begin_hb_font = Some (xfthbfont_begin_hb_font);   // C 828
    xfthbfont_driver.end_hb_font = Some (xfthbfont_end_hb_font);   // C 829
    register_font_driver (std::ptr::addr_of! (xfthbfont_driver), std::ptr::null_mut ());   // C 830
  }
}
