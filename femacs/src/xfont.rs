// femacs: port of src/xfont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The two C non-`static` definitions carry `#[unsafe(no_mangle)]` and `extern "C"`
// (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn` - `compare_font_names` is the
// one exception, spelled `unsafe extern "C" fn` because C hands its address to `qsort` (PORTING.md
// P-12: one definition per linkage unit, which is what C's `static` gives too).
//
// **`src/xfont.c` is a P-86 unit**: the frozen configuration compiles nothing from it.  Its object
// belongs to `FONT_OBJ`, and `configure.ac` 7195-7197 fills that variable with `xfont.o` only under
// `HAVE_X_WINDOWS`; the macro is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1720,
// `FONT_OBJ=` is empty in `build/emacs-ref/src/Makefile` 342 (C's own `xfont.o` also appears at
// that Makefile's 484, inside the `SOME_MACHINE_OBJECTS` source-distribution list, which no rule
// builds), and `docs/baseline/frozen-build-sources.txt` lists no `xfont.o`.  P-86 ① therefore
// applies: this is *transcription*, both arms of every `#if` are carried, and the file gate
// `have_x_windows` goes on every item **and on the include graph** (P-86 ⑧ - an include outside the
// gate fails the frozen build at once, which `build-tu src/xfont.c`'s 336-byte empty object
// measures).  `src/xfont.c` *is* in `docs/baseline/config/cond-eval-verified.txt` (415), so
// `extract-abi.py` reads its `#if`s; the file's own body sits under no `#if` of its own (the gate
// is the build's - the `lwlib-Xlw.c` shape), so the checklist is *not* empty and the ledger
// `docs/ports/xfont.md` carries both P-86 ②'s C line-range table and the readings.
//
// The file's two `#if`s are C 600's `#if defined GCC_LINT || defined lint` - the `last_family`
// initialisation, whose gate is `feature = "gcc_lint"` (`GCC_LINT` is **1** in the frozen
// configuration, macros.tsv 1162, so that arm is live) - and C 1006's
// `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 || (RENDER_MINOR >= 2))`.  `HAVE_XRENDER` is
// `undef` (macros.tsv 1710), and the version half of the condition is computed from
// `<X11/extensions/Xrender.h>`'s `RENDER_MAJOR`/`RENDER_MINOR`, which no Rust `#[cfg]` can
// evaluate from the header: it is the derived feature `render_version_ge_0_2` (the
// `xft_version_lt_2_3_6` shape, `femacs/xtask/Cargo.toml` 755), and the ledger's Owed 1 names it
// for the integrator.
//
// The Xlib surface this unit names is declared in the unit at C's `<X11/Xlib.h>` line (PORTING.md
// P-88 ①: a unit-local segment, *not* a file under `femacs/src/include/`, which mirrors the
// reference's own headers): `<X11/Xlib.h>`'s font records and entry points, `<X11/Xatom.h>`'s
// three font property atoms and - under the `have_xrender` gate - `<X11/extensions/render.h>`'s
// two names and `<X11/extensions/Xrender.h>`'s one record.  Every layout below is a clang reading
// from this machine's headers (`/opt/homebrew/include`: `xorgproto` 2025.1, `libx11`; the probe
// and its output are in the ledger's `## Evidence`), not memory (P-88 ②).  That surface has more
// than one consumer in the tree (`src/xterm.c`, `src/xfns.c`, `src/ftfont.c` ...), so P-90 ① wants
// it in `femacs/src/include/xlib.rs`, which C 24 already reaches: Owed 2 names the move, and when
// it lands this segment disappears from here (P-90 ③ / P-88 ⑤).
//
// `struct face` (C 892-895 reads `face->gc`, C 990 reads `face->font`) is carried in the unit the
// way `dispnew.rs`'s and `font.rs`'s copies are (P-72 rule 3): `include/dispextern.rs`'s record is
// behind `dispextern_face_records` (status 3.220) and so is not available to a unit compiled on its
// own.  `include/font.rs`'s `font_driver` types the members C declares `struct frame *` as untyped
// `*mut c_void` (its own note: `struct frame` need not be in an including unit's graph), so the
// seven driver entries whose C parameter is `struct frame *` or `struct face *` convert their
// function pointer at the table - the P-72 ③ use-point conversion.  The definitions keep C's own
// parameter types.
//
// Names the mirrors this unit includes do not carry are declared at the use site with the
// reference header's line - one citation per line (`font.h`, `xterm.h`; `qsort` from the system
// `<stdlib.h>`, whose mirror carries no such name), so a citation is never paired with the wrong
// name.  The ledger's `## Owed` 3-6 name each and the file it belongs in.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 22: `#include <config.h>` (every unit includes it)

/* C 23-33: the file's own include list.  It sits outside every `#if` of the file body, so with the
   file gate on it (P-86 ⑧) the frozen configuration compiles not one line of this unit - the
   reading the ledger's Evidence ① records.  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/stdlib.rs");          // C 23: `#include <stdlib.h>` (qsort)
#[cfg (feature = "have_x_windows")]
include!("include/xlib.rs");                   // C 24: `#include <X11/Xlib.h>`
#[cfg (feature = "have_x_windows")]
include!("include/lisp.rs");                   // C 26: `#include "lisp.h"`
#[cfg (feature = "have_x_windows")]
include!("include/xterm.rs");                  // C 27: `#include "xterm.h"` (brings `dispextern.rs`)
#[cfg (feature = "have_x_windows")]
include!("include/frame.rs");                  // C 28: `#include "frame.h"`
#[cfg (feature = "have_x_windows")]
include!("include/blockinput.rs");             // C 29: `#include "blockinput.h"`
#[cfg (feature = "have_x_windows")]
include!("include/character.rs");              // C 30: `#include "character.h"`
#[cfg (feature = "have_x_windows")]
include!("include/charset.rs");                // C 31: `#include "charset.h"`
#[cfg (feature = "have_x_windows")]
include!("include/font.rs");                   // C 32: `#include "font.h"`
#[cfg (feature = "have_x_windows")]
include!("include/pdumper.rs");                // C 33: `#include "pdumper.h"`
/* C's `xfont.c` reaches `<string.h>` through `lisp.h` 30, and `composite.h`'s `struct
   composition` through `dispextern.h`'s `struct glyph_string` (this mirror types `cmp` as
   `*mut composition`, so the record's mirror has to be in this unit's graph - P-72 rule 4).  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (memcmp, memcpy, strcpy)
#[cfg (feature = "have_x_windows")]
include!("include/composite.rs");              // P-72 rule 4: `struct composition` (glyph_string.cmp)

/* ----------------------------------------------------------------------------------------------
   C 23 and C 24: the `<stdlib.h>` (system) and `<X11/Xlib.h>` (X11 SDK) surfaces.  Both headers
   are outside the reference tree, so this is the unit-local segment P-88 ① prescribes, and `qsort`
   is declared here because `lib/include/stdlib.rs` carries `malloc`/`free`/`strtol` and not it (the
   `buffer.rs` 129 shape).  Every number below is a clang reading on this machine (the ledger's
   Evidence ②).  */

/// C: <stdlib.h> 489: `extern void qsort (void *__base, size_t __nmemb, size_t __size,
/// __compar_fn_t __compar) __nonnull ((1, 4));` (the port's stdlib mirror does not carry it;
/// declared here, the `buffer.rs` shape).
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn qsort (base: *mut c_void, nmemb: usize, size: usize,
                compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
}

/// Xlib.h 1013-1020: `typedef struct { short lbearing; short rbearing; short width; short ascent;
/// short descent; unsigned short attributes; } XCharStruct;` - measured: `sizeof` **12**, align
/// **2**, the six members at 0, 2, 4, 6, 8 and 10.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XCharStruct
{
  pub lbearing: c_short,                        // Xlib.h 1014
  pub rbearing: c_short,                        // Xlib.h 1015
  pub width: c_short,                           // Xlib.h 1016
  pub ascent: c_short,                          // Xlib.h 1017
  pub descent: c_short,                         // Xlib.h 1018
  pub attributes: c_ushort,                     // Xlib.h 1019
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (std::mem::size_of::<XCharStruct> () == 12);
  core::assert! (std::mem::align_of::<XCharStruct> () == 2);
  core::assert! (std::mem::offset_of! (XCharStruct, lbearing) == 0);
  core::assert! (std::mem::offset_of! (XCharStruct, rbearing) == 2);
  core::assert! (std::mem::offset_of! (XCharStruct, width) == 4);
  core::assert! (std::mem::offset_of! (XCharStruct, ascent) == 6);
  core::assert! (std::mem::offset_of! (XCharStruct, descent) == 8);
  core::assert! (std::mem::offset_of! (XCharStruct, attributes) == 10);
};

/// Xlib.h 1026-1029: `typedef struct { Atom name; unsigned long card32; } XFontProp;` - named by
/// `XFontStruct.properties` only, so the pointer type is what C 1042 needs; measured: `sizeof`
/// **16**, align **8** (`name`@0, `card32`@8).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XFontProp
{
  pub name: Atom,                               // Xlib.h 1027
  pub card32: c_ulong,                          // Xlib.h 1028
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (std::mem::size_of::<XFontProp> () == 16);
  core::assert! (std::mem::align_of::<XFontProp> () == 8);
  core::assert! (std::mem::offset_of! (XFontProp, name) == 0);
  core::assert! (std::mem::offset_of! (XFontProp, card32) == 8);
};

/// Xlib.h 1031-1048: `typedef struct { XExtData *ext_data; Font fid; unsigned direction; unsigned
/// min_char_or_byte2; unsigned max_char_or_byte2; unsigned min_byte1; unsigned max_byte1; Bool
/// all_chars_exist; unsigned default_char; int n_properties; XFontProp *properties; XCharStruct
/// min_bounds; XCharStruct max_bounds; XCharStruct *per_char; int ascent; int descent; }
/// XFontStruct;` - measured: `sizeof` **96**, align **8**, `ext_data`@0, `fid`@8, `direction`@16,
/// `min_char_or_byte2`@20, `max_char_or_byte2`@24, `min_byte1`@28, `max_byte1`@32,
/// `all_chars_exist`@36, `default_char`@40, `n_properties`@44, `properties`@48, `min_bounds`@56,
/// `max_bounds`@68, `per_char`@80, `ascent`@88, `descent`@92.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct XFontStruct
{
  pub ext_data: *mut XExtData,                  // Xlib.h 1032
  pub fid: Font,                                // Xlib.h 1033
  pub direction: c_uint,                        // Xlib.h 1034
  pub min_char_or_byte2: c_uint,                // Xlib.h 1035
  pub max_char_or_byte2: c_uint,                // Xlib.h 1036
  pub min_byte1: c_uint,                        // Xlib.h 1037
  pub max_byte1: c_uint,                        // Xlib.h 1038
  pub all_chars_exist: Bool,                    // Xlib.h 1039
  pub default_char: c_uint,                     // Xlib.h 1040
  pub n_properties: c_int,                      // Xlib.h 1041
  pub properties: *mut XFontProp,               // Xlib.h 1042
  pub min_bounds: XCharStruct,                  // Xlib.h 1043
  pub max_bounds: XCharStruct,                  // Xlib.h 1044
  pub per_char: *mut XCharStruct,               // Xlib.h 1045
  pub ascent: c_int,                            // Xlib.h 1046
  pub descent: c_int,                           // Xlib.h 1047
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (std::mem::size_of::<XFontStruct> () == 96);
  core::assert! (std::mem::align_of::<XFontStruct> () == 8);
  core::assert! (std::mem::offset_of! (XFontStruct, ext_data) == 0);
  core::assert! (std::mem::offset_of! (XFontStruct, fid) == 8);
  core::assert! (std::mem::offset_of! (XFontStruct, min_char_or_byte2) == 20);
  core::assert! (std::mem::offset_of! (XFontStruct, min_byte1) == 28);
  core::assert! (std::mem::offset_of! (XFontStruct, max_byte1) == 32);
  core::assert! (std::mem::offset_of! (XFontStruct, properties) == 48);
  core::assert! (std::mem::offset_of! (XFontStruct, min_bounds) == 56);
  core::assert! (std::mem::offset_of! (XFontStruct, max_bounds) == 68);
  core::assert! (std::mem::offset_of! (XFontStruct, per_char) == 80);
  core::assert! (std::mem::offset_of! (XFontStruct, ascent) == 88);
  core::assert! (std::mem::offset_of! (XFontStruct, descent) == 92);
};

/// Xlib.h 1060-1063: `typedef struct { unsigned char byte1; unsigned char byte2; } XChar2b;` -
/// measured: `sizeof` **2**, align **1** (`byte1`@0, `byte2`@1).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XChar2b
{
  pub byte1: c_uchar,                           // Xlib.h 1061
  pub byte2: c_uchar,                           // Xlib.h 1062
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (std::mem::size_of::<XChar2b> () == 2);
  core::assert! (std::mem::align_of::<XChar2b> () == 1);
  core::assert! (std::mem::offset_of! (XChar2b, byte1) == 0);
  core::assert! (std::mem::offset_of! (XChar2b, byte2) == 1);
};

/// Xatom.h 26: `#define XA_FONT ((Atom) 18)` - measured 18 (`Xlib.h`'s `Atom` is `unsigned long`,
/// `include/xlib.rs` 844).  `include/xlib.rs`'s `x11_constants` module carries the other `XA_*`
/// names and not this one; Owed 2 moves it there.
#[cfg (feature = "have_x_windows")]
pub const XA_FONT: c_ulong = 18;
/// Xatom.h 59: `#define XA_UNDERLINE_POSITION ((Atom) 51)` - measured 51.
#[cfg (feature = "have_x_windows")]
pub const XA_UNDERLINE_POSITION: c_ulong = 51;
/// Xatom.h 60: `#define XA_UNDERLINE_THICKNESS ((Atom) 52)` - measured 52.
#[cfg (feature = "have_x_windows")]
pub const XA_UNDERLINE_THICKNESS: c_ulong = 52;

/// C: Xlib.h 1390-1393: `extern XFontStruct *XLoadQueryFont (Display *, _Xconst char *);`
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XLoadQueryFont (display: *mut Display, name: *const c_char) -> *mut XFontStruct;
  /// C: Xlib.h 1496-1499: `extern char *XGetAtomName (Display *, Atom);`
  pub fn XGetAtomName (display: *mut Display, atom: Atom) -> *mut c_char;
  /// C: Xlib.h 1651-1655: `extern char **XListFonts (Display *, _Xconst char *, int, int *);`
  pub fn XListFonts (display: *mut Display, pattern: *const c_char, maxnames: c_int,
                     actual_count_return: *mut c_int) -> *mut *mut c_char;
  /// C: Xlib.h 2332-2340: `extern int XDrawImageString (Display *, Drawable, GC, int, int,
  /// _Xconst char *, int);`
  pub fn XDrawImageString (display: *mut Display, d: Drawable, gc: GC, x: c_int, y: c_int,
                           string: *const c_char, length: c_int) -> c_int;
  /// C: Xlib.h 2342-2350: `extern int XDrawImageString16 (Display *, Drawable, GC, int, int,
  /// _Xconst XChar2b *, int);`
  pub fn XDrawImageString16 (display: *mut Display, d: Drawable, gc: GC, x: c_int, y: c_int,
                             string: *const XChar2b, length: c_int) -> c_int;
  /// C: Xlib.h 2414-2422: `extern int XDrawString (Display *, Drawable, GC, int, int,
  /// _Xconst char *, int);`
  pub fn XDrawString (display: *mut Display, d: Drawable, gc: GC, x: c_int, y: c_int,
                      string: *const c_char, length: c_int) -> c_int;
  /// C: Xlib.h 2424-2432: `extern int XDrawString16 (Display *, Drawable, GC, int, int,
  /// _Xconst XChar2b *, int);`
  pub fn XDrawString16 (display: *mut Display, d: Drawable, gc: GC, x: c_int, y: c_int,
                        string: *const XChar2b, length: c_int) -> c_int;
  /// C: Xlib.h 2526-2528: `extern int XFree (void *);` - `include/xlib.rs` does not carry it; the
  /// ledger's Owed 2 names it for that mirror.
  pub fn XFree (data: *mut c_void) -> c_int;
  /// C: Xlib.h 2552-2555: `extern int XFreeFont (Display *, XFontStruct *);`
  pub fn XFreeFont (display: *mut Display, font_struct: *mut XFontStruct) -> c_int;
  /// C: Xlib.h 2563-2565: `extern int XFreeFontNames (char **);`
  pub fn XFreeFontNames (list: *mut *mut c_char) -> c_int;
  /// C: Xlib.h 2617-2621: `extern Bool XGetFontProperty (XFontStruct *, Atom, unsigned long *);`
  pub fn XGetFontProperty (font_struct: *mut XFontStruct, atom: Atom,
                           value_return: *mut c_ulong) -> Bool;
  /// C: Xlib.h 3225-3229: `extern int XSetFont (Display *, GC, Font);`
  pub fn XSetFont (display: *mut Display, gc: GC, font: Font) -> c_int;
}

/* C 1006's second header: `<X11/extensions/Xrender.h>` (and `<X11/extensions/render.h>`, which it
   includes).  The whole block is behind `have_xrender` because that is the arm C's `#if` selects;
   measured on this machine: `XRenderColor` `sizeof` **8**, align **2** (`red`@0, `green`@2,
   `blue`@4, `alpha`@6), `PictOpSrc` **1**.  */
/// render.h 31: `typedef XID Picture;` - Xlib's `XID`, `include/xlib.rs` 837.
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
pub type Picture = XID;
/// Xrender.h 173-182: `typedef struct { unsigned short red; unsigned short green; unsigned short
/// blue; unsigned short alpha; } XRenderColor;` - measured: `sizeof` **8**, align **2**.
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XRenderColor
{
  pub red: c_ushort,                            // Xrender.h 175
  pub green: c_ushort,                          // Xrender.h 177
  pub blue: c_ushort,                           // Xrender.h 179
  pub alpha: c_ushort,                          // Xrender.h 181
}
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
const _: () = {
  core::assert! (std::mem::size_of::<XRenderColor> () == 8);
  core::assert! (std::mem::align_of::<XRenderColor> () == 2);
  core::assert! (std::mem::offset_of! (XRenderColor, red) == 0);
  core::assert! (std::mem::offset_of! (XRenderColor, green) == 2);
  core::assert! (std::mem::offset_of! (XRenderColor, blue) == 4);
  core::assert! (std::mem::offset_of! (XRenderColor, alpha) == 6);
};
/// render.h 94: `#define PictOpSrc 1` - measured 1.  `include/xlib.rs`'s `x11_constants` module is
/// where the other X11 constants live; Owed 2 moves this one there.
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
pub const PictOpSrc: c_int = 1;
/// C: Xrender.h 824-826: `void XRenderFillRectangle (Display *dpy, int op, Picture dst,
/// _Xconst XRenderColor *color, int x, int y, unsigned int width, unsigned int height);`
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
unsafe extern "C" {
  pub fn XRenderFillRectangle (dpy: *mut Display, op: c_int, dst: Picture,
                               color: *const XRenderColor, x: c_int, y: c_int,
                               width: c_uint, height: c_uint);
}

/* The Emacs-side names the mirrors this unit includes do not carry.  Each keeps its own header's
   line (P-55/P-72 rule 3), so a citation is never paired with the wrong name.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: font.h 839: `extern Lisp_Object copy_font_spec (Lisp_Object);` (font.c defines it).
  pub fn copy_font_spec (font: Lisp_Object) -> Lisp_Object;
  /// C: font.h 842: `extern Lisp_Object font_make_entity (void);`
  pub fn font_make_entity () -> Lisp_Object;
  /// C: font.h 846: `extern Lisp_Object font_make_object (int, Lisp_Object, int);`
  pub fn font_make_object (size: c_int, entity: Lisp_Object, pixel_size: c_int) -> Lisp_Object;
  /// C: font.h 852-853: `extern int font_registry_charsets (Lisp_Object, struct charset **,
  /// struct charset **);`
  pub fn font_registry_charsets (registry: Lisp_Object, encoding: *mut *mut charset,
                                 repertory: *mut *mut charset) -> c_int;
  /// C: font.h 885-886: `extern Lisp_Object font_intern_prop (const char *str, ptrdiff_t len,
  /// bool force_symbol);`
  pub fn font_intern_prop (str_: *const c_char, len: ptrdiff_t, force_symbol: bool)
                           -> Lisp_Object;
  /// C: font.h 893: `extern int font_parse_xlfd (char *name, ptrdiff_t len, Lisp_Object font);`
  pub fn font_parse_xlfd (name: *mut c_char, len: ptrdiff_t, font: Lisp_Object) -> c_int;
  /// C: font.h 895: `extern ptrdiff_t font_unparse_xlfd (Lisp_Object, int, char *, int);`
  pub fn font_unparse_xlfd (font: Lisp_Object, pixel_size: c_int, fname: *mut c_char,
                            len: c_int) -> ptrdiff_t;
  /// C: font.h 896: `extern void register_font_driver (struct font_driver const *,
  /// struct frame *);`
  pub fn register_font_driver (d: *const font_driver, f: *mut frame);
  /// C: font.h 1001 `font_add_log` and C: xterm.h 1405 `x_mark_frame_dirty` are the mirrors'
  /// own declarations now (`include/font.rs` `FONT_ADD_LOG!` body, `include/xterm.rs`
  /// `FRAME_X_DRAWABLE!` body; status 3.521 moved them there, 3.524 retired the use-site
  /// copies this block carried because they were duplicates of them).
  /// C: xterm.h 1003: `extern struct x_display_info *x_display_info_for_display (Display *);`
  pub fn x_display_info_for_display (display: *mut Display) -> *mut x_display_info;
  /// C: xterm.h 1752: `extern void x_catch_errors (Display *);`
  pub fn x_catch_errors (dpy: *mut Display);
  /// C: xterm.h 1762: `extern bool x_had_errors_p (Display *);`
  pub fn x_had_errors_p (dpy: *mut Display) -> bool;
  /// C: xterm.h 1769: `extern void x_clear_errors (Display *);`
  pub fn x_clear_errors (dpy: *mut Display);
  /// C: xterm.h 1764: `extern void x_uncatch_errors (void);`
  pub fn x_uncatch_errors ();
  /// C: xterm.h 1826-1827: `extern void x_xrender_color_from_gc_background (struct frame *, GC,
  /// XRenderColor *, bool);` (declared in the reference only under `HAVE_XRENDER`; the gate here
  /// is C's own.)
  #[cfg (feature = "have_xrender")]
  pub fn x_xrender_color_from_gc_background (f: *mut frame, gc: GC, color: *mut XRenderColor,
                                             enabled: bool);
  /// C: xterm.h 1828: `extern void x_xr_ensure_picture (struct frame *);`
  #[cfg (feature = "have_xrender")]
  pub fn x_xr_ensure_picture (f: *mut frame);
  /// C: xterm.h 1829: `extern void x_xr_apply_ext_clip (struct frame *, GC);`
  #[cfg (feature = "have_xrender")]
  pub fn x_xr_apply_ext_clip (f: *mut frame, gc: GC);
  /// C: xterm.h 1830: `extern void x_xr_reset_ext_clip (struct frame *);`
  #[cfg (feature = "have_xrender")]
  pub fn x_xr_reset_ext_clip (f: *mut frame);
}

/* `struct face` (dispextern.h 1787-1937) is this unit's other use-site record (P-72 rule 3):
   `include/dispextern.rs`'s copy is behind `dispextern_face_records` and so is not available to a
   unit compiled on its own, and `xfont_prepare_face` reads `face->gc` while `xfont_draw` reads
   `face->font`.  Only those two members are read, so the layout is pinned the way `dispnew.rs`'s
   copy is - clang, `spikes/gated-layout-probe.py 'struct face' dispextern.h c --prelude lisp.h`,
   this unit's own arm (`HAVE_WINDOW_SYSTEM` + `HAVE_X_WINDOWS`: `gc`@168, `stipple`@176,
   `font`@232) and the frozen one (`font`@216).  C's `HAVE_X_WINDOWS` arm of the member is `GC gc;`
   (dispextern.h 1800), which is `include/xlib.rs` 892's `GC`.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct face
{
  _prefix: [u8; 168],                           // dispextern.h 1791-1795 (`lface`, `id`; clang: `gc`@168)
  pub gc: GC,                                   // dispextern.h 1802: `GC gc;`
  _stipple: ptrdiff_t,                          // dispextern.h 1810: `ptrdiff_t stipple;`
  _colors: [c_ulong; 6],                        // dispextern.h 1816-1826: `foreground`…`box_color`
  pub font: *mut font,                          // dispextern.h 1828: `struct font *font;`
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (std::mem::offset_of! (face, gc) == 168);
  core::assert! (std::mem::offset_of! (face, font) == 232);
};

/* File-scope objects (C 239, C 242, C 1131).  The two caches are C `static` and stay unit-local;
   they carry the file gate like every other item (P-86 ⑧: the names they need come from the
   gated include graph).  */
#[cfg (feature = "have_x_windows")]
static mut xfont_scripts_cache: Lisp_Object = lisp_h_Qnil ();   // C 239
#[cfg (feature = "have_x_windows")]
static mut xfont_scratch_props: Lisp_Object = lisp_h_Qnil ();   // C 242

/* X core font driver.  */

#[cfg (feature = "have_x_windows")]
#[repr(C)]
struct xfont_info                                        // C 38-44
{
  font: font,              // C 40: `struct font font;`
  display: *mut Display,   // C 41: `Display *display;`
  xfont: *mut XFontStruct, // C 42: `XFontStruct *xfont;`
  x_display_id: c_uint,    // C 43: `unsigned x_display_id;`
}

/* The record's layout follows `include/font.rs`'s `font` (184 frozen, +32 when
   `HAVE_WINDOW_SYSTEM`, which is what that mirror's `FONT_WS_SHIFT` is), so the offsets shift with
   the same 32 bytes.  Measured with clang on the reference's own headers in this unit's arm
   (`-DHAVE_WINDOW_SYSTEM=1 -DHAVE_X_WINDOWS=1`: `sizeof` 240, align 8, `display`@216, `xfont`@224,
   `x_display_id`@232) and in the frozen arm (`sizeof` 208, `display`@184); the probe is in the
   ledger's Evidence ③.  */
#[cfg (not (feature = "have_window_system"))]
const XFONT_INFO_WS_SHIFT: usize = 0;
#[cfg (feature = "have_window_system")]
const XFONT_INFO_WS_SHIFT: usize = 32;
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (std::mem::size_of::<xfont_info> () == 208 + XFONT_INFO_WS_SHIFT);
  core::assert! (std::mem::align_of::<xfont_info> () == 8);
  core::assert! (std::mem::offset_of! (xfont_info, font) == 0);
  core::assert! (std::mem::offset_of! (xfont_info, display) == 184 + XFONT_INFO_WS_SHIFT);
  core::assert! (std::mem::offset_of! (xfont_info, xfont) == 192 + XFONT_INFO_WS_SHIFT);
  core::assert! (std::mem::offset_of! (xfont_info, x_display_id) == 200 + XFONT_INFO_WS_SHIFT);
};

/* Prototypes of support functions.  C 48's forward declaration of `xfont_get_pcm` has no Rust
   counterpart (PORTING.md 0.1-12): its definition follows here, and Rust resolves the two uses
   inside `xfont_chars_supported` and `xfont_open` from that one definition.  */

/* Get metrics of character CHAR2B in XFONT.  Value is null if CHAR2B
   is not contained in the font.  */

#[cfg (feature = "have_x_windows")]
unsafe fn xfont_get_pcm (xfont: *mut XFontStruct, char2b: c_uint) -> *mut XCharStruct  // C 53-114
{
  /* The result metric information.  */
  let mut pcm: *mut XCharStruct = std::ptr::null_mut ();
  let byte1: c_uchar = (char2b >> 8) as c_uchar;
  let byte2: c_uchar = (char2b & 0xFF) as c_uchar;

  eassert! (! xfont.is_null ());

  if ! (*xfont).per_char.is_null ()
    {
      if (*xfont).min_byte1 == 0 && (*xfont).max_byte1 == 0
	{
	  /* min_char_or_byte2 specifies the linear character index
	     corresponding to the first element of the per_char array,
	     max_char_or_byte2 is the index of the last character.  A
	     character with non-zero byte1 is not in the font.
	     A character with byte2 less than min_char_or_byte2 or
	     greater max_char_or_byte2 is not in the font.  */
	  if byte1 == 0
	     && byte2 as c_uint >= (*xfont).min_char_or_byte2
	     && byte2 as c_uint <= (*xfont).max_char_or_byte2
	    { pcm = (*xfont).per_char.offset ((byte2 as c_uint
					       - (*xfont).min_char_or_byte2) as isize); }
	}
      else
	{
	  /* If either min_byte1 or max_byte1 are nonzero, both
	     min_char_or_byte2 and max_char_or_byte2 are less than
	     256, and the 2-byte character index values corresponding
	     to the per_char array element N (counting from 0) are:

	     byte1 = N/D + min_byte1
	     byte2 = N\D + min_char_or_byte2

	     where:

	     D = max_char_or_byte2 - min_char_or_byte2 + 1
	     / = integer division
	     \ = integer modulus  */
	  if byte1 as c_uint >= (*xfont).min_byte1
	     && byte1 as c_uint <= (*xfont).max_byte1
	     && byte2 as c_uint >= (*xfont).min_char_or_byte2
	     && byte2 as c_uint <= (*xfont).max_char_or_byte2
	    { pcm = (*xfont).per_char.offset (((*xfont).max_char_or_byte2
					       - (*xfont).min_char_or_byte2 + 1)
					      .wrapping_mul (byte1 as c_uint
							     - (*xfont).min_byte1)
					      .wrapping_add (byte2 as c_uint
							     - (*xfont).min_char_or_byte2)
					      as isize); }
	}
    }
  else
    {
      /* If the per_char pointer is null, all glyphs between the first
	 and last character indexes inclusive have the same
	 information, as given by both min_bounds and max_bounds.  */
      if byte2 as c_uint >= (*xfont).min_char_or_byte2
	 && byte2 as c_uint <= (*xfont).max_char_or_byte2
	{ pcm = &raw mut (*xfont).max_bounds; }
    }

  return (if pcm.is_null ()
	     || ((*pcm).width == 0
		 && ((*pcm).rbearing as c_int - (*pcm).lbearing as c_int) == 0)
	  { std::ptr::null_mut () } else { pcm });
}

/// C: xfont.c 118-119: `Lisp_Object xfont_get_cache (struct frame *f)` - the record's first
/// member is `struct font`, so the driver entry takes C's `struct frame *`; `include/font.rs`
/// types `font_driver.get_cache`'s parameter as the untyped `*mut c_void` (its own note), and
/// the definition follows the mirror, converting at the use point (P-72 rule 3).  The four
/// other entries with a `struct frame *` parameter - `xfont_list`, `xfont_match`,
/// `xfont_list_family`, `xfont_open` - and `xfont_prepare_face`'s `struct face *` do the same.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfont_get_cache (f: *mut c_void) -> Lisp_Object      // C 118-123
{
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! ((f as *mut frame));

  return (*dpyinfo).name_list_element;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn compare_font_names (name1: *const c_void, name2: *const c_void) -> c_int
{                                                                           // C 126-132
  let n1: *const *const c_char = name1 as *const *const c_char;
  let n2: *const *const c_char = name2 as *const *const c_char;
  /* The mirror's `INLINE int xstrcasecmp (...)` lives inside `pub mod dispextern_externs`,
     which no glob import brings into this unit, so the call is qualified (the `ftfont.rs` 1494
     shape; P-72 rule 4).  */
  return dispextern_externs::xstrcasecmp (*n1, *n2);
}

/* Decode XLFD as iso-8859-1 into OUTPUT, and return the byte length
   of the decoding result.  LEN is the byte length of XLFD, or -1 if
   XLFD is NULL terminated.  The caller must assure that OUTPUT is at
   least twice (plus 1) as large as XLFD.  */

#[cfg (feature = "have_x_windows")]
unsafe fn xfont_decode_coding_xlfd (xlfd: *mut c_char, mut len: c_int, output: *mut c_char)
                                    -> ptrdiff_t                          // C 139-158
{
  let mut p0: *mut c_char = xlfd;
  let mut p1: *mut c_char = output;
  let mut c: c_int;

  while *p0 != 0
    {
      c = *p0 as c_uchar as c_int;
      p0 = p0.offset (1);
      p1 = p1.offset (CHAR_STRING (c, p1 as *mut u8) as isize);
      len -= 1;
      if len == 0
	{ break; }
    }
  *p1 = 0;
  return p1.offset_from (output);
}

/* Encode XLFD from UTF-8 to iso-8859-1 destructively, and return the
   resulting byte length.  If XLFD contains unencodable character,
   return -1.  */

#[cfg (feature = "have_x_windows")]
unsafe fn xfont_encode_coding_xlfd (xlfd: *mut c_char) -> c_int              // C 160-183
{
  let mut p0: *const u8 = xlfd as *const u8;
  let mut p1: *mut u8 = xlfd as *mut u8;
  let mut len: c_int = 0;

  while *p0 != 0
    {
      let c: c_int = string_char_advance (&raw mut p0);

      if c >= 0x100
	{ return -1; }
      *p1 = c as u8;
      p1 = p1.offset (1);
      len += 1;
    }
  *p1 = 0;
  return len;
}

/* Check if CHARS (cons or vector) is supported by XFONT whose
   encoding charset is ENCODING (XFONT is NULL) or by a font whose
   registry corresponds to ENCODING and REPERTORY.
   Return true if supported.  */

#[cfg (feature = "have_x_windows")]
unsafe fn xfont_chars_supported (chars: Lisp_Object, xfont: *mut XFontStruct,
                                 encoding: *mut charset, repertory: *mut charset) -> bool
{                                                                           // C 185-232
  let charset: *mut charset = if repertory.is_null () { encoding } else { repertory };

  if CONSP (chars)
    {
      let mut chars: Lisp_Object = chars;
      while CONSP (chars)
	{
	  let c: c_int = XFIXNUM (XCAR (chars)) as c_int;
	  let code: c_uint = ENCODE_CHAR! (charset, c);

	  if code == CHARSET_INVALID_CODE! (charset)
	    { break; }
	  if xfont.is_null ()
	    { chars = XCDR (chars); continue; }
	  if code >= 0x10000
	    { break; }
	  if xfont_get_pcm (xfont, code).is_null ()
	    { break; }
	  chars = XCDR (chars);
	}
      return NILP (chars);
    }
  else if VECTORP (chars)
    {
      let mut i: ptrdiff_t = ASIZE (chars) - 1;

      while i >= 0
	{
	  let c: c_int = XFIXNUM (AREF (chars, i)) as c_int;
	  let code: c_uint = ENCODE_CHAR! (charset, c);

	  if code == CHARSET_INVALID_CODE! (charset)
	    { i -= 1; continue; }
	  if xfont.is_null ()
	    { break; }
	  if code >= 0x10000
	    { i -= 1; continue; }
	  if ! xfont_get_pcm (xfont, code).is_null ()
	    { break; }
	  i -= 1;
	}
      return i >= 0;
    }
  return false;
}

/* A hash table recoding which font supports which scripts.  Each key
   is a vector of characteristic font properties FOUNDRY to WIDTH and
   ADDSTYLE, and each value is a list of script symbols.

   We assume that fonts that have the same value in the above
   properties supports the same set of characters on all displays.  */

/* Reusable vector to store characteristic font properties.   */

/* Return a list of scripts supported by the font of FONTNAME whose
   characteristic properties are in PROPS and whose encoding charset
   is ENCODING.  A caller must call BLOCK_INPUT in advance.  */

#[cfg (feature = "have_x_windows")]
unsafe fn xfont_supported_scripts (display: *mut Display, fontname: *mut c_char,
                                   props: Lisp_Object, encoding: *mut charset) -> Lisp_Object
{                                                                           // C 248-286
  let mut scripts: Lisp_Object;

  /* Two special cases to avoid opening rather big fonts.  */
  if EQ (AREF (props, 2), Qja ())
    { return list2 (Qkana (), Qhan ()); }
  if EQ (AREF (props, 2), Qko ())
    { return list1 (Qhangul ()); }
  scripts = Fgethash (props, xfont_scripts_cache, Qt ());
  if EQ (scripts, Qt ())
    {
      let mut xfont: *mut XFontStruct;
      let mut val: Lisp_Object;

      scripts = Qnil ();
      xfont = XLoadQueryFont (display, fontname);
      if ! xfont.is_null ()
	{
	  if ! (*xfont).per_char.is_null ()
	    {
	      val = Vscript_representative_chars;
	      while CONSP (val)
		{
		  if CONSP (XCAR (val)) && SYMBOLP (XCAR (XCAR (val)))
		    {
		      let script: Lisp_Object = XCAR (XCAR (val));
		      let chars: Lisp_Object = XCDR (XCAR (val));

		      if xfont_chars_supported (chars, xfont, encoding,
						std::ptr::null_mut ())
			{ scripts = Fcons (script, scripts); }
		    }
		  val = XCDR (val);
		}
	    }
	  XFreeFont (display, xfont);
	}
      if EQ (AREF (props, 3), Qiso10646_1 ())
	 && NILP (Fmemq (Qlatin (), scripts))
	{ scripts = Fcons (Qlatin (), scripts); }
      Fputhash (Fcopy_sequence (props), scripts, xfont_scripts_cache);
    }
  return scripts;
}

#[cfg (feature = "have_x_windows")]
unsafe fn xfont_list_pattern (display: *mut Display, pattern: *const c_char,
                              registry: Lisp_Object, script: Lisp_Object) -> Lisp_Object
{                                                                           // C 292-457
  let mut list: Lisp_Object = Qnil ();
  let mut chars: Lisp_Object = Qnil ();
  let mut encoding: *mut charset = std::ptr::null_mut ();
  let mut repertory: *mut charset = std::ptr::null_mut ();
  let mut registry: Lisp_Object = registry;
  let mut i: c_int;
  let mut limit: c_int;
  let mut num_fonts: c_int = 0;
  let mut names: *mut *mut c_char = std::ptr::null_mut ();
  /* Large enough to decode the longest XLFD (255 bytes). */
  let mut buf: [c_char; 512] = [0; 512];

  if ! NILP (registry)
     && font_registry_charsets (registry, &raw mut encoding, &raw mut repertory) < 0
    /* Unknown REGISTRY, not supported.  */
    { return Qnil (); }
  if ! NILP (script)
    {
      chars = assq_no_quit (script, Vscript_representative_chars);
      if NILP (chars)
	/* We can't tell whether or not a font supports SCRIPT.  */
	{ return Qnil (); }
      chars = XCDR (chars);
      if ! repertory.is_null ()
	{
	  if ! xfont_chars_supported (chars, std::ptr::null_mut (), encoding, repertory)
	    { return Qnil (); }
	  registry = Qnil ();
	}
    }

  block_input ();
  x_catch_errors (display);

  limit = 512;
  loop
    {
      names = XListFonts (display, pattern, limit, &raw mut num_fonts);
      if x_had_errors_p (display)
	{
	  /* This error is perhaps due to insufficient memory on X
	     server.  Let's just ignore it.  */
	  x_clear_errors (display);
	  num_fonts = 0;
	  break;
	}
      if num_fonts < limit
	{ break; }
      XFreeFontNames (names);
      limit *= 2;
    }

  if num_fonts > 0
    {
      let mut indices: *mut *mut c_char;
      let props: *mut Lisp_Object
	= (*XVECTOR (xfont_scratch_props)).contents.as_mut_ptr ();
      let mut scripts: Lisp_Object = Qnil ();
      let mut entity: Lisp_Object = Qnil ();

      /* C's bare `alloca` is the port's stack pool (PORTING.md P-05's decision; the editfns.rs
	 shape).  */
      indices = sa_stack_alloc (std::mem::size_of::<*mut c_char> () * num_fonts as usize)
	        as *mut *mut c_char;

      i = 0;
      while i < ASIZE (xfont_scratch_props) as c_int
	{
	  ASET (xfont_scratch_props, i as isize, Qnil ());
	  i += 1;
	}
      i = 0;
      while i < num_fonts
	{
	  *indices.offset (i as isize) = *names.offset (i as isize);
	  i += 1;
	}
      qsort (indices as *mut c_void, num_fonts as usize, std::mem::size_of::<*mut c_char> (),
	     Some (compare_font_names));

      /* Take one or two passes over the font list.  Do the second
	 pass only if we really need it, i.e., only if the first pass
	 found no fonts and skipped some scalable fonts.  */
      let mut skipped_some_scalable_fonts: bool = false;
      let mut i_pass: c_int = 0;
      while i_pass == 0
	    || (i_pass == 1 && NILP (list) && skipped_some_scalable_fonts)
	{
	  i = 0;
	  while i < num_fonts
	    {
	      let mut len: ptrdiff_t;

	      if i > 0
		 && dispextern_externs::xstrcasecmp (*indices.offset ((i - 1) as isize),
						     *indices.offset (i as isize)) == 0
		{ i += 1; continue; }
	      if NILP (entity)
		{ entity = font_make_entity (); }
	      len = xfont_decode_coding_xlfd (*indices.offset (i as isize), -1, buf.as_mut_ptr ());
	      if font_parse_xlfd (buf.as_mut_ptr (), len, entity) < 0
		{ i += 1; continue; }
	      ASET (entity, FONT_TYPE_INDEX as isize, Qx ());
	      /* Avoid auto-scaled fonts.  */
	      if FIXNUMP (AREF (entity, FONT_DPI_INDEX as isize))
		 && FIXNUMP (AREF (entity, FONT_AVGWIDTH_INDEX as isize))
		 && XFIXNUM (AREF (entity, FONT_DPI_INDEX as isize)) != 0
		 && XFIXNUM (AREF (entity, FONT_AVGWIDTH_INDEX as isize)) == 0
		{ i += 1; continue; }
	      /* Avoid not-allowed scalable fonts.  */
	      if NILP (Vscalable_fonts_allowed)
		{
		  let mut size: c_int = 0;

		  if FIXNUMP (AREF (entity, FONT_SIZE_INDEX as isize))
		    { size = XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) as c_int; }
		  else if FLOATP (AREF (entity, FONT_SIZE_INDEX as isize))
		    { size = XFLOAT_DATA (AREF (entity, FONT_SIZE_INDEX as isize)) as c_int; }
		  if size == 0 && i_pass == 0
		    {
		      skipped_some_scalable_fonts = true;
		      i += 1; continue;
		    }
		}
	      else if CONSP (Vscalable_fonts_allowed)
		{
		  let mut tail: Lisp_Object;

		  tail = Vscalable_fonts_allowed;
		  while CONSP (tail)
		    {
		      let elt: Lisp_Object = XCAR (tail);
		      if STRINGP (elt)
			 && (fast_c_string_match_ignore_case (elt, *indices.offset (i as isize),
							      len as usize)
			     >= 0)
			{ break; }
		      tail = XCDR (tail);
		    }
		  if ! CONSP (tail)
		    { i += 1; continue; }
		}

	      /* Avoid fonts of invalid registry.  */
	      if NILP (AREF (entity, FONT_REGISTRY_INDEX as isize))
		{ i += 1; continue; }

	      /* Update encoding and repertory if necessary.  */
	      if ! EQ (registry, AREF (entity, FONT_REGISTRY_INDEX as isize))
		{
		  registry = AREF (entity, FONT_REGISTRY_INDEX as isize);
		  if font_registry_charsets (registry, &raw mut encoding, &raw mut repertory) < 0
		    { encoding = std::ptr::null_mut (); }
		}
	      if encoding.is_null ()
		/* Unknown REGISTRY, not supported.  */
		{ i += 1; continue; }
	      if ! repertory.is_null ()
		{
		  if NILP (script)
		     || xfont_chars_supported (chars, std::ptr::null_mut (), encoding, repertory)
		    { list = Fcons (entity, list); entity = Qnil (); }
		  i += 1; continue;
		}
	      if memcmp (props as *const c_void,
			 aref_addr (entity, FONT_FOUNDRY_INDEX as isize) as *const c_void,
			 (word_size * 7) as usize) != 0
		 || ! EQ (AREF (entity, FONT_SPACING_INDEX as isize), *props.offset (7))
		{
		  vcopy (xfont_scratch_props, 0,
			 aref_addr (entity, FONT_FOUNDRY_INDEX as isize) as *const Lisp_Object, 7);
		  ASET (xfont_scratch_props, 7, AREF (entity, FONT_SPACING_INDEX as isize));
		  scripts = xfont_supported_scripts (display, *indices.offset (i as isize),
						     xfont_scratch_props, encoding);
		}
	      if NILP (script)
		 || ! NILP (Fmemq (script, scripts))
		{ list = Fcons (entity, list); entity = Qnil (); }
	      i += 1;
	    }
	  i_pass += 1;
	}
      XFreeFontNames (names);
    }

  x_uncatch_errors ();
  unblock_input ();

  FONT_ADD_LOG! (c"xfont-list".as_ptr (), build_string (pattern), list);
  return list;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_list (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{                                                             // C 459-536
  let display: *mut Display = (*FRAME_DISPLAY_INFO! ((f as *mut frame))).display;
  let mut registry: Lisp_Object;
  let mut list: Lisp_Object;
  let mut val: Lisp_Object;
  let extra: Lisp_Object;
  let mut script: Lisp_Object;
  let mut len: c_int;
  /* Large enough to contain the longest XLFD (255 bytes) in UTF-8.  */
  let mut name: [c_char; 512] = [0; 512];

  extra = AREF (spec, FONT_EXTRA_INDEX as isize);
  if CONSP (extra)
    {
      val = assq_no_quit (QCotf (), extra);
      if ! NILP (val)
	{ return Qnil (); }
      val = assq_no_quit (QClang (), extra);
      if ! NILP (val)
	{ return Qnil (); }
    }

  registry = AREF (spec, FONT_REGISTRY_INDEX as isize);
  len = font_unparse_xlfd (spec, 0, name.as_mut_ptr (), 512) as c_int;
  if len < 0 || { len = xfont_encode_coding_xlfd (name.as_mut_ptr ()) as c_int; len < 0 }
    { return Qnil (); }

  val = assq_no_quit (QCscript (), extra);
  script = CDR (val);
  list = xfont_list_pattern (display, name.as_mut_ptr (), registry, script);
  if NILP (list) && NILP (registry)
    {
      /* Try iso10646-1 */
      let r: *mut c_char = name.as_mut_ptr ()
	.offset (len as isize - (c"iso8859-1".to_bytes ().len () as isize - 1));

      if (r.offset_from (name.as_mut_ptr ()) as usize)
	 .wrapping_add (c"iso10646-1".to_bytes ().len () - 1) < 256
	{
	  strcpy (r, c"iso10646-1".as_ptr ());
	  list = xfont_list_pattern (display, name.as_mut_ptr (), Qiso10646_1 (), script);
	}
    }
  if NILP (list) && ! NILP (registry)
    {
      /* Try alternate registries.  */
      let mut alter: Lisp_Object;

      /* C's comma expression in the `if`: the assignment is the first operand, `CONSP (alter)`
	 the second (PORTING.md 0.1-13's statement split, the editfns.rs tuple shape).  */
      if { alter = Fassoc (SYMBOL_NAME (registry),
			   Vface_alternative_font_registry_alist,
			   Qnil ());
	   CONSP (alter) }
	{
	  /* Pointer to REGISTRY-ENCODING field.  */
	  let r: *mut c_char = name.as_mut_ptr ()
	    .offset (len as isize - SBYTES (SYMBOL_NAME (registry)));

	  alter = XCDR (alter);
	  while CONSP (alter)
	    {
	      if STRINGP (XCAR (alter))
		 && (r.offset_from (name.as_mut_ptr ()) + SBYTES (XCAR (alter))) < 256
		{
		  lispstpcpy (r, XCAR (alter));
		  list = xfont_list_pattern (display, name.as_mut_ptr (), registry, script);
		  if ! NILP (list)
		    { break; }
		}
	      alter = XCDR (alter);
	    }
	}
    }
  if NILP (list)
    {
      /* Try alias.  */
      val = assq_no_quit (QCname (), AREF (spec, FONT_EXTRA_INDEX as isize));
      if CONSP (val) && STRINGP (XCDR (val)) && SBYTES (XCDR (val)) < 512
	{
	  memcpy (name.as_mut_ptr () as *mut c_void, SDATA (XCDR (val)) as *const c_void,
		  (SBYTES (XCDR (val)) + 1) as usize);
	  if xfont_encode_coding_xlfd (name.as_mut_ptr ()) < 0
	    { return Qnil (); }
	  list = xfont_list_pattern (display, name.as_mut_ptr (), registry, script);
	}
    }

  return list;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_match (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{                                                             // C 538-590
  let display: *mut Display = (*FRAME_DISPLAY_INFO! ((f as *mut frame))).display;
  let extra: Lisp_Object;
  let val: Lisp_Object;
  let mut entity: Lisp_Object;
  let mut name: [c_char; 512] = [0; 512];
  let mut xfont: *mut XFontStruct;
  let mut value: c_ulong = 0;

  extra = AREF (spec, FONT_EXTRA_INDEX as isize);
  val = assq_no_quit (QCname (), extra);
  if ! CONSP (val) || ! STRINGP (XCDR (val))
    {
      if font_unparse_xlfd (spec, 0, name.as_mut_ptr (), 512) < 0
	{ return Qnil (); }
    }
  else if SBYTES (XCDR (val)) < 512
    { memcpy (name.as_mut_ptr () as *mut c_void, SDATA (XCDR (val)) as *const c_void,
	      (SBYTES (XCDR (val)) + 1) as usize); }
  else
    { return Qnil (); }
  if xfont_encode_coding_xlfd (name.as_mut_ptr ()) < 0
    { return Qnil (); }

  block_input ();
  entity = Qnil ();
  xfont = XLoadQueryFont (display, name.as_mut_ptr ());
  if ! xfont.is_null ()
    {
      if XGetFontProperty (xfont, XA_FONT, &raw mut value) != 0
	{
	  let s: *mut c_char = XGetAtomName (display, value as Atom);

	  /* If DXPC (a Differential X Protocol Compressor)
	     Ver.3.7 is running, XGetAtomName will return null
	     string.  We must avoid such a name.  */
	  if *s != 0
	    {
	      let len: ptrdiff_t;
	      entity = font_make_entity ();
	      ASET (entity, FONT_TYPE_INDEX as isize, Qx ());
	      len = xfont_decode_coding_xlfd (s, -1, name.as_mut_ptr ());
	      if font_parse_xlfd (name.as_mut_ptr (), len, entity) < 0
		{ entity = Qnil (); }
	    }
	  XFree (s as *mut c_void);
	}
      XFreeFont (display, xfont);
    }
  unblock_input ();

  FONT_ADD_LOG! (c"xfont-match".as_ptr (), spec, entity);
  return entity;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_list_family (f: *mut c_void) -> Lisp_Object
{                                                             // C 592-650
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! ((f as *mut frame));
  let mut names: *mut *mut c_char;
  let mut num_fonts: c_int = 0;
  let mut i: c_int;
  let mut list: Lisp_Object;
  /* C leaves `last_family` uninitialised without `GCC_LINT` (the two arms write it in exactly
     the same place); the port's zero is PORTING.md 1.5.  */
  let mut last_family: *const c_char = std::ptr::null ();
  #[cfg (feature = "gcc_lint")]
  { last_family = c"".as_ptr (); }
  let mut last_len: c_int;

  block_input ();
  x_catch_errors ((*dpyinfo).display);
  names = XListFonts ((*dpyinfo).display, c"-*-*-*-*-*-*-*-*-*-*-*-*-*-*".as_ptr (),
		      0x8000, &raw mut num_fonts);
  if x_had_errors_p ((*dpyinfo).display)
    {
      /* This error is perhaps due to insufficient memory on X server.
	 Let's just ignore it.  */
      x_clear_errors ((*dpyinfo).display);
      num_fonts = 0;
    }

  list = Qnil ();
  i = 0;
  last_len = 0;
  while i < num_fonts
    {
      let mut p0: *mut c_char = *names.offset (i as isize);
      let mut p1: *mut c_char;
      let mut buf: [c_char; 512] = [0; 512];
      let mut family: Lisp_Object;
      let mut decoded_len: c_int;

      p0 = p0.offset (1);		/* skip the leading '-' */
      while *p0 != 0 && *p0 != b'-' as c_char { p0 = p0.offset (1); } /* skip foundry */
      if *p0 == 0
	{ i += 1; continue; }
      p0 = p0.offset (1);
      p1 = p0;
      while *p1 != 0 && *p1 != b'-' as c_char { p1 = p1.offset (1); } /* find the end of family */
      if *p1 == 0 || p1 == p0
	{ i += 1; continue; }
      if last_len as isize == p1.offset_from (p0)
	 && memcmp (last_family as *const c_void, p0 as *const c_void, last_len as usize) == 0
	{ i += 1; continue; }
      last_len = p1.offset_from (p0) as c_int;
      last_family = p0;

      decoded_len = xfont_decode_coding_xlfd (p0, last_len, buf.as_mut_ptr ()) as c_int;
      family = font_intern_prop (p0, decoded_len as ptrdiff_t, true);
      if NILP (assq_no_quit (family, list))
	{ list = Fcons (family, list); }
      i += 1;
    }

  XFreeFontNames (names);
  x_uncatch_errors ();
  unblock_input ();

  return list;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_open (f: *mut c_void, entity: Lisp_Object, pixel_size: c_int)
                            -> Lisp_Object                   // C 651-862
{
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! ((f as *mut frame));
  let display: *mut Display = (*dpyinfo).display;
  let mut name: [c_char; 512] = [0; 512];
  let mut len: c_int;
  let mut value: c_ulong = 0;
  let mut registry: Lisp_Object;
  let mut encoding: *mut charset = std::ptr::null_mut ();
  let mut repertory: *mut charset = std::ptr::null_mut ();
  let mut font_object: Lisp_Object;
  let mut fullname: Lisp_Object;
  let mut font: *mut font;
  let mut xfont: *mut XFontStruct;
  let mut pixel_size: c_int = pixel_size;

  /* At first, check if we know how to encode characters for this
     font.  */
  registry = AREF (entity, FONT_REGISTRY_INDEX as isize);
  if font_registry_charsets (registry, &raw mut encoding, &raw mut repertory) < 0
    {
      FONT_ADD_LOG! (c"  x:unknown registry".as_ptr (), registry, Qnil ());
      return Qnil ();
    }

  if XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) != 0
    { pixel_size = XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) as c_int; }
  else if pixel_size == 0
    {
      if ! FRAME_FONT! ((f as *mut frame)).is_null ()
	{ pixel_size = (*FRAME_FONT! ((f as *mut frame))).pixel_size; }
      else
	{ pixel_size = 14; }
    }
  len = font_unparse_xlfd (entity, pixel_size, name.as_mut_ptr (), 512) as c_int;
  if len <= 0 || { len = xfont_encode_coding_xlfd (name.as_mut_ptr ()) as c_int; len < 0 }
    {
      FONT_ADD_LOG! (c"  x:unparse failed".as_ptr (), entity, Qnil ());
      return Qnil ();
    }

  block_input ();
  x_catch_errors (display);
  xfont = XLoadQueryFont (display, name.as_mut_ptr ());
  if x_had_errors_p (display)
    {
      /* This error is perhaps due to insufficient memory on X server.
	 Let's just ignore it.  */
      x_clear_errors (display);
      xfont = std::ptr::null_mut ();
    }
  else if xfont.is_null ()
    {
      /* Some version of X lists:
	   -misc-fixed-medium-r-normal--20-*-75-75-c-100-iso8859-1
	   -misc-fixed-medium-r-normal--20-*-100-100-c-100-iso8859-1
	 but can open only:
	   -misc-fixed-medium-r-normal--20-*-100-100-c-100-iso8859-1
	 and
	   -misc-fixed-medium-r-normal--20-*-*-*-c-100-iso8859-1
	 So, we try again with wildcards in RESX and RESY.  */
      let mut temp: Lisp_Object;

      temp = copy_font_spec (entity);
      ASET (temp, FONT_DPI_INDEX as isize, Qnil ());
      len = font_unparse_xlfd (temp, pixel_size, name.as_mut_ptr (), 512) as c_int;
      if len <= 0 || { len = xfont_encode_coding_xlfd (name.as_mut_ptr ()) as c_int; len < 0 }
	{
	  FONT_ADD_LOG! (c"  x:unparse failed".as_ptr (), temp, Qnil ());
	  return Qnil ();
	}
      xfont = XLoadQueryFont (display, name.as_mut_ptr ());
      if x_had_errors_p (display)
	{
	  /* This error is perhaps due to insufficient memory on X server.
	     Let's just ignore it.  */
	  x_clear_errors (display);
	  xfont = std::ptr::null_mut ();
	}
    }
  fullname = Qnil ();
  /* Try to get the full name of FONT.  */
  if ! xfont.is_null () && XGetFontProperty (xfont, XA_FONT, &raw mut value) != 0
    {
      let mut p0: *mut c_char;
      let mut p: *mut c_char;
      let mut dashes: c_int = 0;

      p0 = XGetAtomName (FRAME_X_DISPLAY! ((f as *mut frame)), value as Atom);
      p = p0;
      /* Count the number of dashes in the "full name".
	 If it is too few, this isn't really the font's full name,
	 so don't use it.
	 In X11R4, the fonts did not come with their canonical names
	 stored in them.  */
      while *p != 0
	{
	  if *p == b'-' as c_char
	    { dashes += 1; }
	  p = p.offset (1);
	}

      if dashes >= 13
	{
	  len = xfont_decode_coding_xlfd (p0, -1, name.as_mut_ptr ()) as c_int;
	  fullname = Fdowncase (make_string (name.as_ptr (), len as ptrdiff_t));
	}
      XFree (p0 as *mut c_void);
    }
  x_uncatch_errors ();
  unblock_input ();

  if xfont.is_null ()
    {
      FONT_ADD_LOG! (c"  x:open failed".as_ptr (), build_string (name.as_ptr ()), Qnil ());
      return Qnil ();
    }

  font_object = font_make_object (VECSIZE (std::mem::size_of::<xfont_info> ()) as c_int,
				  entity, pixel_size);
  ASET (font_object, FONT_TYPE_INDEX as isize, Qx ());
  if STRINGP (fullname)
    {
      font_parse_xlfd (SSDATA (fullname), SBYTES (fullname), font_object);
      ASET (font_object, FONT_NAME_INDEX as isize, fullname);
    }
  else
    {
      let mut buf: [c_char; 512] = [0; 512];

      len = xfont_decode_coding_xlfd (name.as_mut_ptr (), -1, buf.as_mut_ptr ()) as c_int;
      ASET (font_object, FONT_NAME_INDEX as isize,
	    make_string (buf.as_ptr (), len as ptrdiff_t));
    }
  ASET (font_object, FONT_FULLNAME_INDEX as isize, fullname);
  font = XFONT_OBJECT (font_object);
  (*(font as *mut xfont_info)).xfont = xfont;
  (*((font as *mut xfont_info))).display = FRAME_X_DISPLAY! ((f as *mut frame));
  (*((font as *mut xfont_info))).x_display_id
    = (*FRAME_DISPLAY_INFO! ((f as *mut frame))).x_id;
  (*font).pixel_size = pixel_size;
  (*font).driver = std::ptr::addr_of! (xfont_driver) as *const c_void;
  (*font).encoding_charset = (*encoding).id;
  (*font).repertory_charset = if repertory.is_null () { -1 } else { (*repertory).id };
  (*font).ascent = (*xfont).ascent;
  (*font).descent = (*xfont).descent;
  (*font).height = (*font).ascent + (*font).descent;
  (*font).min_width = (*xfont).min_bounds.width as c_int;
  (*font).max_width = (*xfont).max_bounds.width as c_int;
  if (*xfont).min_bounds.width == (*xfont).max_bounds.width
    {
      /* Fixed width font.  */
      (*font).average_width = (*xfont).min_bounds.width as c_int;
      (*font).space_width = (*xfont).min_bounds.width as c_int;
    }
  else
    {
      let mut pcm: *mut XCharStruct;
      let mut val: Lisp_Object;

      pcm = xfont_get_pcm (xfont, 0x20);
      if ! pcm.is_null ()
	{ (*font).space_width = (*pcm).width as c_int; }
      else
	{ (*font).space_width = 0; }

      val = Ffont_get (font_object, QCavgwidth ());
      if FIXNUMP (val)
	{ (*font).average_width = (XFIXNUM (val) / 10) as c_int; }
      if (*font).average_width < 0
	{ (*font).average_width = - (*font).average_width; }
      else
	{
	  if (*font).average_width == 0
	     && charset_bits! (ascii_compatible_p, encoding)
	    {
	      let mut width: c_int = (*font).space_width;
	      let mut n: c_int = (! pcm.is_null ()) as c_int;
	      let mut char2b: c_uint = 33;

	      while char2b <= 126
		{
		  if { pcm = xfont_get_pcm (xfont, char2b); ! pcm.is_null () }
		    { width += (*pcm).width as c_int; n += 1; }
		  char2b += 1;
		}
	      if n > 0
		{ (*font).average_width = width / n; }
	    }
	  if (*font).average_width == 0
	    /* No easy way other than this to get a reasonable
	       average_width.  */
	    { (*font).average_width
		= ((*xfont).min_bounds.width as c_int + (*xfont).max_bounds.width as c_int) / 2; }
	}
    }

  block_input ();
  (*font).underline_thickness
    = (if XGetFontProperty (xfont, XA_UNDERLINE_THICKNESS, &raw mut value) != 0
	 { value as c_long } else { 0 }) as c_int;
  (*font).underline_position
    = (if XGetFontProperty (xfont, XA_UNDERLINE_POSITION, &raw mut value) != 0
	 { value as c_long } else { -1 }) as c_int;
  (*font).baseline_offset
    = (if XGetFontProperty (xfont, (*dpyinfo).Xatom_MULE_BASELINE_OFFSET, &raw mut value) != 0
	 { value as c_long } else { 0 }) as c_int;
  (*font).relative_compose
    = (if XGetFontProperty (xfont, (*dpyinfo).Xatom_MULE_RELATIVE_COMPOSE, &raw mut value) != 0
	 { value as c_long } else { 0 }) as c_int;
  (*font).default_ascent
    = (if XGetFontProperty (xfont, (*dpyinfo).Xatom_MULE_DEFAULT_ASCENT, &raw mut value) != 0
	 { value as c_long } else { 0 }) as c_int;
  unblock_input ();

  if NILP (fullname)
    { fullname = AREF (font_object, FONT_NAME_INDEX as isize); }
  (*font).vertical_centering
    = STRINGP (Vvertical_centering_font_regexp)
      && (fast_string_match_ignore_case (Vvertical_centering_font_regexp, fullname) >= 0);

  return font_object;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_close (font: *mut font)                         // C 864-887
{
  let mut xdi: *mut x_display_info = std::ptr::null_mut ();
  let xfi: *mut xfont_info = font as *mut xfont_info;

  /* This function may be called from GC when X connection is gone
     (Bug#16093), and an attempt to free font resources on invalid
     display may lead to X protocol errors or segfaults.  Moreover,
     the memory referenced by 'Display *' pointer may be reused for
     the logically different X connection after the previous display
     connection was closed.  That's why we also check whether font's
     ID matches the one recorded in x_display_info for this display.
     See https://debbugs.gnu.org/cgi/bugreport.cgi?bug=16069.  */
  if ! (*xfi).xfont.is_null ()
     && { xdi = x_display_info_for_display ((*xfi).display); ! xdi.is_null () }
     && (*xfi).x_display_id == (*xdi).x_id
    {
      block_input ();
      XFreeFont ((*xfi).display, (*xfi).xfont);
      unblock_input ();
      (*xfi).xfont = std::ptr::null_mut ();
    }
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_prepare_face (f: *mut c_void, face: *mut c_void)
{                                                             // C 889-896
  block_input ();
  XSetFont (FRAME_X_DISPLAY! ((f as *mut frame)), (*(face as *mut face)).gc,
	    (*(*((*(face as *mut face)).font as *mut xfont_info)).xfont).fid);
  unblock_input ();
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_has_char (font: Lisp_Object, c: c_int) -> c_int  // C 898-925
{
  let registry: Lisp_Object = AREF (font, FONT_REGISTRY_INDEX as isize);
  /* C declares `encoding` uninitialised, and passes its address to `font_registry_charsets`
     below; the port's zero is PORTING.md 1.5.  */
  let mut encoding: *mut charset = std::ptr::null_mut ();
  let mut repertory: *mut charset = std::ptr::null_mut ();

  if EQ (registry, Qiso10646_1 ())
    {
      encoding = CHARSET_FROM_ID! (charset_unicode);
      /* We use a font of `ja' and `ko' adstyle only for a character
	 in JISX0208 and KSC5601 charsets respectively.  */
      if EQ (AREF (font, FONT_ADSTYLE_INDEX as isize), Qja ())
	 && charset_jisx0208 >= 0
	{ repertory = CHARSET_FROM_ID! (charset_jisx0208); }
      else if EQ (AREF (font, FONT_ADSTYLE_INDEX as isize), Qko ())
	       && charset_ksc5601 >= 0
	{ repertory = CHARSET_FROM_ID! (charset_ksc5601); }
    }
  else if font_registry_charsets (registry, &raw mut encoding, &raw mut repertory) < 0
    /* Unknown REGISTRY, not usable.  */
    { return 0; }
  if ASCII_CHAR_P (c as isize) && charset_bits! (ascii_compatible_p, encoding)
    { return 1; }
  if repertory.is_null ()
    { return -1; }
  return (ENCODE_CHAR! (repertory, c) != CHARSET_INVALID_CODE! (repertory)) as c_int;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_encode_char (font: *mut font, c: c_int) -> c_uint // C 927-945
{
  let xfont: *mut XFontStruct = (*(font as *mut xfont_info)).xfont;
  let mut charset: *mut charset;
  let code: c_uint;

  charset = CHARSET_FROM_ID! ((*font).encoding_charset);
  code = ENCODE_CHAR! (charset, c);
  if code == CHARSET_INVALID_CODE! (charset)
    { return FONT_INVALID_CODE; }
  if (*font).repertory_charset >= 0
    {
      charset = CHARSET_FROM_ID! ((*font).repertory_charset);
      return if ENCODE_CHAR! (charset, c) != CHARSET_INVALID_CODE! (charset)
	     { code } else { FONT_INVALID_CODE };
    }
  return if ! xfont_get_pcm (xfont, code).is_null () { code } else { FONT_INVALID_CODE };
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_text_extents (font: *mut font, code: *const c_uint,
                                          nglyphs: c_int, metrics: *mut font_metrics)
{                                                             // C 947-987
  let xfont: *mut XFontStruct = (*(font as *mut xfont_info)).xfont;
  let mut i: c_int;
  let mut width: c_int = 0;
  let mut first: bool;
  let mut pcm: *mut XCharStruct;

  i = 0;
  first = true;
  while i < nglyphs
    {
      if *code.offset (i as isize) >= 0x10000
	{ i += 1; continue; }
      pcm = xfont_get_pcm (xfont, *code.offset (i as isize));
      if pcm.is_null ()
	{ i += 1; continue; }
      if first
	{
	  (*metrics).lbearing = (*pcm).lbearing;
	  (*metrics).rbearing = (*pcm).rbearing;
	  (*metrics).ascent = (*pcm).ascent;
	  (*metrics).descent = (*pcm).descent;
	  first = false;
	}
      else
	{
	  if ((*metrics).lbearing as c_int) > (width + (*pcm).lbearing as c_int)
	    { (*metrics).lbearing = (width + (*pcm).lbearing as c_int) as c_short; }
	  if ((*metrics).rbearing as c_int) < (width + (*pcm).rbearing as c_int)
	    { (*metrics).rbearing = (width + (*pcm).rbearing as c_int) as c_short; }
	  if (*metrics).ascent < (*pcm).ascent
	    { (*metrics).ascent = (*pcm).ascent; }
	  if (*metrics).descent < (*pcm).descent
	    { (*metrics).descent = (*pcm).descent; }
	}
      width += (*pcm).width as c_int;
      i += 1;
    }

  (*metrics).width = width as c_short;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_draw (s: *mut glyph_string, from: c_int, to: c_int, x: c_int,
                                 y: c_int, with_background: bool) -> c_int
{                                                             // C 989-1120
  let xfont: *mut XFontStruct = (*( (*s).font as *mut xfont_info )).xfont;
  let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);
  let len: c_int = to - from;
  let gc: GC = (*s).gc as GC;
  let mut i: c_int;
  let mut with_background: bool = with_background;

  if (*s).gc != (*( (*s).face as *mut face )).gc as *mut c_void
    {
      block_input ();
      XSetFont (display, gc, (*xfont).fid);
      unblock_input ();
    }

  #[cfg (all (feature = "have_xrender", feature = "render_version_ge_0_2"))]
  if with_background
     && (*FRAME_DISPLAY_INFO! ((*s).f)).alpha_bits != 0
     && FRAME_CHECK_XR_VERSION! ((*s).f, 0, 2)
    {
      use crate::x11_constants::None;

      x_xr_ensure_picture ((*s).f);

      if FRAME_X_PICTURE! ((*s).f) != None as Picture
	{
	  let mut xc: XRenderColor = std::mem::zeroed ();
	  let height: c_int = FONT_HEIGHT! ((*s).font);
	  let ascent: c_int = FONT_BASE! ((*s).font);

	  x_xr_apply_ext_clip ((*s).f, gc);
	  x_xrender_color_from_gc_background ((*s).f, gc, &raw mut xc,
					      (*s).hl != DRAW_CURSOR);
	  XRenderFillRectangle (FRAME_X_DISPLAY! ((*s).f),
				PictOpSrc, FRAME_X_PICTURE! ((*s).f),
				&raw const xc, x, y - ascent,
				(*s).width as c_uint, height as c_uint);
	  x_xr_reset_ext_clip ((*s).f);
	  x_mark_frame_dirty ((*s).f);

	  with_background = false;
	}
    }

  if (*xfont).min_byte1 == 0 && (*xfont).max_byte1 == 0
    {
      USE_SAFE_ALLOCA! (sa_avail, sa_count);
      let str_: *mut c_char = SAFE_ALLOCA! (sa_avail, len) as *mut c_char;
      i = 0;
      while i < len
	{
	  *str_.offset (i as isize)
	    = (*((*s).char2b.offset ((from + i) as isize)) & 0xFF) as c_char;
	  i += 1;
	}
      block_input ();
      if with_background
	{
	  if glyph_string_bits! (padding_p, (*s))
	    {
	      i = 0;
	      while i < len
		{
		  XDrawImageString (display, FRAME_X_DRAWABLE! ((*s).f),
				    gc, x + i, y, str_.offset (i as isize), 1);
		  i += 1;
		}
	    }
	  else
	    { XDrawImageString (display, FRAME_X_DRAWABLE! ((*s).f),
				gc, x, y, str_, len); }
	}
      else
	{
	  if glyph_string_bits! (padding_p, (*s))
	    {
	      i = 0;
	      while i < len
		{
		  XDrawString (display, FRAME_X_DRAWABLE! ((*s).f),
			       gc, x + i, y, str_.offset (i as isize), 1);
		  i += 1;
		}
	    }
	  else
	    { XDrawString (display, FRAME_X_DRAWABLE! ((*s).f),
			   gc, x, y, str_, len); }
	}
      unblock_input ();
      SAFE_FREE! (sa_count);
      return (*s).nchars;
    }

  block_input ();
  if with_background
    {
      if glyph_string_bits! (padding_p, (*s))
	{
	  i = 0;
	  while i < len
	    {
	      let code: c_uint = *((*s).char2b.offset ((from + i) as isize));
	      let char2b: XChar2b = XChar2b { byte1: (code >> 8) as c_uchar,
					       byte2: (code & 0xFF) as c_uchar };
	      XDrawImageString16 (display, FRAME_X_DRAWABLE! ((*s).f),
				  gc, x + i, y, &raw const char2b, 1);
	      i += 1;
	    }
	}
      else
	{
	  USE_SAFE_ALLOCA! (sa_avail, sa_count);
	  let code: *const c_uint = (*s).char2b.offset (from as isize);
	  let mut char2b: *mut XChar2b = std::ptr::null_mut ();
	  let mut i: c_int = 0;
	  SAFE_NALLOCA! (sa_avail, char2b, 1, len);
	  while i < len
	    {
	      *char2b.offset (i as isize)
		= XChar2b { byte1: (*code.offset (i as isize) >> 8) as c_uchar,
			    byte2: (*code.offset (i as isize) & 0xFF) as c_uchar };
	      i += 1;
	    }
	  XDrawImageString16 (display, FRAME_X_DRAWABLE! ((*s).f),
			      gc, x, y, char2b, len);
	  SAFE_FREE! (sa_count);
	}
    }
  else
    {
      if glyph_string_bits! (padding_p, (*s))
	{
	  i = 0;
	  while i < len
	    {
	      let code: c_uint = *((*s).char2b.offset ((from + i) as isize));
	      let char2b: XChar2b = XChar2b { byte1: (code >> 8) as c_uchar,
					       byte2: (code & 0xFF) as c_uchar };
	      XDrawString16 (display, FRAME_X_DRAWABLE! ((*s).f),
			     gc, x + i, y, &raw const char2b, 1);
	      i += 1;
	    }
	}
      else
	{
	  USE_SAFE_ALLOCA! (sa_avail, sa_count);
	  let code: *const c_uint = (*s).char2b.offset (from as isize);
	  let mut char2b: *mut XChar2b = std::ptr::null_mut ();
	  let mut i: c_int = 0;
	  SAFE_NALLOCA! (sa_avail, char2b, 1, len);
	  while i < len
	    {
	      *char2b.offset (i as isize)
		= XChar2b { byte1: (*code.offset (i as isize) >> 8) as c_uchar,
			    byte2: (*code.offset (i as isize) & 0xFF) as c_uchar };
	      i += 1;
	    }
	  XDrawString16 (display, FRAME_X_DRAWABLE! ((*s).f),
			 gc, x, y, char2b, len);
	  SAFE_FREE! (sa_count);
	}
    }
  unblock_input ();

  return len;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xfont_check (f: *mut c_void, font: *mut font) -> c_int  // C 1120-1127
{
  let xfont: *mut xfont_info = font as *mut xfont_info;

  return if FRAME_X_DISPLAY! ((f as *mut frame)) == (*xfont).display { 0 } else { -1 };
}

/* C 1129's forward declaration of `syms_of_xfont_for_pdumper` has no Rust counterpart (PORTING.md
   0.1-12); its definition follows, and `syms_of_xfont` reaches it from there.  */

/// The `C` record: `struct font_driver const xfont_driver` (C 1131-1146).  C's designated
/// initialiser names thirteen members and leaves the rest zero; Rust has no designated
/// initialisers, so the rest are written out (the `androidfont.rs` shape).  `font.h`'s
/// `font_driver` mirrors the members C declares with a `struct frame *` parameter as untyped
/// `*mut c_void` (`include/font.rs`'s own note: `struct frame` need not be in an including unit's
/// graph), so the seven entries whose C parameter is `struct frame *` or `struct face *` convert
/// their function pointer here - the P-72 ③ use-point conversion.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut xfont_driver: font_driver = font_driver {     // C 1131-1146
  /* C 1133: `.type = LISPSYM_INITIALLY (Qx)`.  The mirror's `LISPSYM_INITIALLY` is not a `const
     fn`, so the port writes the same constant expression over the generated `iQx` index (the
     androidfont.rs `androidfont_driver` precedent).  */
  r#type: (((iQx * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	    + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),
  case_sensitive: false,
  get_cache: Some (xfont_get_cache),
  list: Some (xfont_list),
  r#match: Some (xfont_match),
  list_family: Some (xfont_list_family),
  open_font: Some (xfont_open),
  close_font: Some (xfont_close),
  prepare_face: Some (xfont_prepare_face),
  has_char: Some (xfont_has_char),
  encode_char: Some (xfont_encode_char),
  text_extents: Some (xfont_text_extents),
  #[cfg (feature = "have_window_system")]
  draw: Some (xfont_draw),
  check: Some (xfont_check),
  free_entity: None,
  done_face: None,
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

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xfont ()                                // C 1148-1160
{
  staticpro (&raw mut xfont_scripts_cache);
  xfont_scripts_cache = CALLN! (Fmake_hash_table, QCtest (), Qequal ());
  staticpro (&raw mut xfont_scratch_props);
  xfont_scratch_props = make_nil_vector (8);
  pdumper_do_now_and_after_load (Some (syms_of_xfont_for_pdumper));

  DEFSYM! (Qkana, "kana");
  DEFSYM! (Qhan, "han");
  DEFSYM! (Qhangul, "hangul");
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn syms_of_xfont_for_pdumper ()                         // C 1162-1166
{
  register_font_driver (std::ptr::addr_of! (xfont_driver), std::ptr::null_mut ());
}
