// femacs: port of src/ftcrfont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The C non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2) -
// `syms_of_ftcrfont`, `ftcrfont_get_default_font_options`, `ftcrfont_driver` and
// `ftcrhbfont_driver`; the C `static` helpers stay plain `unsafe fn`s.
//
// **`src/ftcrfont.c` is a P-86 unit**: the frozen configuration compiles nothing from it.  Its
// object joins `FONT_OBJ` in three cairo configurations (`configure.ac` 7191-7215): C 7199's
// `FONT_OBJ="$FONT_OBJ ftfont.o ftcrfont.o"` inside the `HAVE_X_WINDOWS` branch under
// `HAVE_CAIRO` (whose cairo block AC_DEFINEs `USE_CAIRO`, C 4409), C 7208's unconditional pair
// under `test "${window_system}" = "pgtk"` (cairo AC_DEFINEs `USE_CAIRO`, C 4483) and C 7213's
// pair under `HAVE_BE_APP` + `HAVE_CAIRO` (which AC_DEFINEs `USE_BE_CAIRO`, C 4500).  None of
// them holds here: `FONT_OBJ=` is empty in `build/emacs-ref/src/Makefile` and
// `docs/baseline/frozen-build-sources.txt` lists no `ftcrfont.o`, so this target's `build-tu`
// object is the 336-byte empty one.  The file has **no `#if` of its own** - the gate is the build
// membership - and `any (feature = "use_cairo", feature = "be_cairo")` is that condition's
// spelling (`USE_CAIRO` for the X11 and the pgtk configurations, `USE_BE_CAIRO` for the Haiku one).
// P-86 (1) therefore applies: this is *transcription*, both arms of every `#if` are carried, and
// the gate goes on every item **and on the include graph** (P-86 (8)).
//
// The conditions the text carries, each arm named by the lowercased macro (P-86 (1), status
// 3.171's rule), in C's order:
//
//   * C 25-33 `#ifdef HAVE_X_WINDOWS` / `#elif HAVE_HAIKU` / `#else` choose the terminal header
//     (`feature = "have_x_windows"` / `feature = "have_haiku"` / the pgtk arm - the `#else` of a
//     three-way split, so its gate is *neither* of the two).
//   * C 40-42 `#ifdef HAVE_PGTK` (the `<xsettings.h>` include and C 175-179's font-options
//     choice) is `feature = "have_pgtk"`.
//   * C 44-48 `#ifdef USE_BE_CAIRO` (the three colour macros) is `feature = "be_cairo"`, the
//     registered spelling of `USE_BE_CAIRO` (`xtask/Cargo.toml` 73).
//   * C 180-183, C 548-559, C 563-581, C 585-598, C 614-626 and C 629-638 are the Haiku ones
//     (`be_cairo` against `not (feature = "be_cairo")`, which nests the `have_x_windows` /
//     pgtk pair C 564/566 and 586/588 spell).
//   * C 208-213, C 303-305, C 340-346, C 666-724, C 759-761, C 767-770 and C 814-825
//     `#ifdef HAVE_HARFBUZZ` are `feature = "harfbuzz"` (the registered spelling, the
//     `ftfont.rs`/`hbfont.rs` reading).
//   * C 299-302, C 333-339, C 482-497 and C 744-746 `#ifdef HAVE_LIBOTF` are
//     `feature = "have_libotf"`; C 499-519 and C 747-749's `#if defined HAVE_M17N_FLT &&
//     defined HAVE_LIBOTF` is `all (feature = "have_m17n_flt", feature = "have_libotf")`;
//     C 521-536 and C 750-752's `#if defined HAVE_OTF_GET_VARIATION_GLYPHS || defined
//     HAVE_FT_FACE_GETCHARVARIANTINDEX` is `any (feature = "have_otf_get_variation_glyphs",
//     feature = "have_ft_face_getcharvariantindex")`.
//   * C 644-664 and C 755-757 `#ifdef HAVE_PGTK` are `feature = "have_pgtk"`, and C 774-808
//     `#ifdef HAVE_X_WINDOWS` is `feature = "have_x_windows"`.
//
// `src/ftcrfont.c` is in `docs/baseline/config/cond-eval-verified.txt` (its 285th line), so
// `tools/check-cond-eval.py src/ftcrfont.c` decides every one of its `#if`s against the frozen
// macro table: `33 conditions, 0 unknown, 0 disagreements [VERIFIED]` (the ledger's Evidence 1).
// The arms above are nevertheless all carried as text, and the ledger `docs/ports/ftcrfont.md`
// names every C line - P-86 (2)'s `## C line ranges` table, which is this file's substitute for a
// checklist the frozen build cannot give.
//
// **The third-party surfaces stand in the unit**, at the C `#include` lines (P-88 (1)): `math.h`
// through its gnulib mirror (C 21), `<cairo-ft.h>` (C 22, whose `<cairo.h>` and `<ft2build.h>`
// halves arrive with it), `<fontconfig/fontconfig.h>` (through `ftfont.h`'s chain), FreeType
// through `ftfont.h`, `<otf.h>` under `have_libotf`, `<hb.h>` under `harfbuzz` and `xterm.h` /
// `haikuterm.h` / `haiku_support.h` / `pgtkterm.h`'s own cairo entry points.  None of those
// header files is in the reference tree, and **every one of them has two or more consumers among
// the ported units** (`src/ftfont.c`, `src/ftfont.h`, `src/xftfont.c`, `src/ftcrfont.c`,
// `src/hbfont.c`, `src/pgtkterm.c`, `src/haikuterm.c`, `src/xsettings.c`), so P-90 (1) wants a
// dedicated mirror each (`include/cairo.rs`, `include/fontconfig.rs`, `include/freetype.rs`,
// `include/hb.rs`, `include/otf.rs`): the ledger's `## Owed` names the moves, and when they land
// this unit's segments disappear in the same commit (P-88 (5) / P-90 (3)).  `<cairo.h>`'s
// surface/pattern half is already a mirror (`include/gtk.rs` 236-320, which the `pgtkterm.rs`
// unit and the `xsettings.rs` unit read the same way), so this unit includes it instead of
// spelling a second copy of it.
//
// All of those layouts are **clang readings of this machine's own headers** (cairo 1.18.4,
// Fontconfig 2.17.1, FreeType 2.14.3, HarfBuzz 14.4.0, all under `/opt/homebrew/include`) and are
// asserted in the unit; `<otf.h>` is **not** on this machine, so the two libotf names are their
// published prototypes and **UNVERIFIED** (P-88 (2): both are opaque pointers, so there is no
// layout to measure).  The ledger's `## Evidence` records the probe and the provenance.
//
// `struct face` (C 553-554's `s->face->foreground` / `->background`, the Haiku arm only) is
// carried in the unit the way `xftfont.rs`'s copy is (P-72 rule 3): `include/dispextern.rs`'s
// record is behind `dispextern_face_records` (status 3.220) and the mirror types
// `glyph_string.face` as the untyped `*mut c_void`, so the two offsets this unit reads are
// pinned against the reference's own `dispextern.h` (the ledger's Evidence 5).
//
// Names the mirrors this unit includes do not carry are declared at the use site with the
// reference header's line - one citation per line (`xterm.h`, `haikuterm.h`, `haiku_support.h`,
// `font.h`).  The ledger's `## Owed` names each and the file it belongs in.  The one exception is
// deliberate and recorded: `include/ftfont.rs`'s `struct font_info` carries the record's *XFT*
// arm, so the four cairo members C reads and writes (`cr_scaled_font`, `bitmap_position_unit`,
// `metrics`, `metrics_nrows`) are missing from the mirror, and this unit's own lines report
// `E0609` until that mirror grows them (P-86 (7); the ledger's Evidence 3 and Owed 3).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

use std::ffi::CStr;                            // the FC_* object names of the Fontconfig segment

include!("include/config.rs");                 // C 20: `#include <config.h>` (every unit includes it)

/* C 21: `#include <math.h>` - `floor`, `ceil` and `lround` (C 101, 102, 103, 109, 110, 272, 276,
   281).  The three are this machine's libm, reached through the gnulib mirror the `hbfont.rs` unit
   reads the same way.  */
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("../lib/include/math.rs");

/// C: math.h 474: `extern long int lround (double __x) __attribute__ ((__nothrow__ ));` - the one
/// `<math.h>` name this unit reads that the gnulib mirror does not carry (`include/math.rs`'s 53-54
/// declare `ceil`/`floor` only), declared at its use site (P-72 rule 4, P-55); the `hbfont.rs`
/// 585-588 shape, and the ledger's `## Owed` names the mirror it belongs in.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" {
  pub fn lround (x: c_double) -> c_long;
}

/* C 22: `#include <cairo-ft.h>` - cairo's own half plus the FreeType half.  `include/gtk.rs`
   236-320 is the tree's cairo *surface* mirror (it declares `cairo_t`, `cairo_surface_t`,
   `cairo_pattern_t`, `cairo_format_t`, `cairo_operator_t`, `cairo_pattern_type_t`,
   `cairo_status_t` and ten entry points), so this unit includes it and declares only the half it
   does not carry (`cairo_glyph_t`, the two extents records, `cairo_matrix_t`, the font-face,
   scaled-font and font-options records and the twenty-six entry points below).  */
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("include/gtk.rs");

/// C: cairo.h 1163-1166: `typedef struct { unsigned long index; double x; double y; }
/// cairo_glyph_t;` - measured `sizeof` 24, align 8 (`index`@0, `x`@8, `y`@16).
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct cairo_glyph_t
{
  pub index: c_ulong,                          // cairo.h 1164
  pub x: c_double,                             // cairo.h 1164
  pub y: c_double,                             // cairo.h 1165
}

/// C: cairo.h 1225-1254: `typedef struct { double x_bearing; double y_bearing; double width;
/// double height; double x_advance; double y_advance; } cairo_text_extents_t;` - measured
/// `sizeof` 48, align 8, the six members at 0/8/16/24/32/40.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct cairo_text_extents_t
{
  pub x_bearing: c_double,                     // cairo.h 1226
  pub y_bearing: c_double,                     // cairo.h 1227
  pub width: c_double,                         // cairo.h 1228
  pub height: c_double,                        // cairo.h 1229
  pub x_advance: c_double,                     // cairo.h 1230
  pub y_advance: c_double,                     // cairo.h 1231
}

/// C: cairo.h 1294-1305: `typedef struct { double ascent; double descent; double height; double
/// max_x_advance; double max_y_advance; } cairo_font_extents_t;` - measured `sizeof` 40, align 8,
/// the five members at 0/8/16/24/32.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct cairo_font_extents_t
{
  pub ascent: c_double,                        // cairo.h 1295
  pub descent: c_double,                       // cairo.h 1296
  pub height: c_double,                        // cairo.h 1297
  pub max_x_advance: c_double,                 // cairo.h 1298
  pub max_y_advance: c_double,                 // cairo.h 1299
}

/// C: cairo.h 201-205: `typedef struct _cairo_matrix { double xx; double yx; double xy; double
/// yy; double x0; double y0; } cairo_matrix_t;` - measured `sizeof` 48, align 8, the six members
/// at 0/8/16/24/32/40.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct cairo_matrix_t
{
  pub xx: c_double,                            // cairo.h 202
  pub yx: c_double,
  pub xy: c_double,                            // cairo.h 203
  pub yy: c_double,
  pub x0: c_double,                            // cairo.h 204
  pub y0: c_double,
}

/// C: cairo.h 1134: `typedef struct _cairo_font_face cairo_font_face_t;` - opaque; only pointers
/// are made, tested and destroyed.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct _cairo_font_face { _private: [u8; 0] }
/// C: cairo.h 1134: the pointer type C spells `cairo_font_face_t *`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type cairo_font_face_t = _cairo_font_face;
/// C: cairo.h 1113: `typedef struct _cairo_scaled_font cairo_scaled_font_t;` - opaque.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct _cairo_scaled_font { _private: [u8; 0] }
/// C: cairo.h 1113: the pointer type C spells `cairo_scaled_font_t *`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type cairo_scaled_font_t = _cairo_scaled_font;
/// C: cairo.h 1462: `typedef struct _cairo_font_options cairo_font_options_t;` - opaque.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct _cairo_font_options { _private: [u8; 0] }
/// C: cairo.h 1462: the pointer type C spells `cairo_font_options_t *`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type cairo_font_options_t = _cairo_font_options;

/// C: cairo.h 325-374: `typedef enum _cairo_status { CAIRO_STATUS_SUCCESS = 0, … }
/// cairo_status_t;` - C's enum ABI is `c_int` (PORTING.md 1.3); `CAIRO_STATUS_SUCCESS` measured 0.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub const CAIRO_STATUS_SUCCESS: c_int = 0;
/// C: cairo.h 750-765: `typedef enum _cairo_antialias { CAIRO_ANTIALIAS_DEFAULT,
/// CAIRO_ANTIALIAS_NONE, CAIRO_ANTIALIAS_GRAY, CAIRO_ANTIALIAS_SUBPIXEL, … }
/// cairo_antialias_t;` - measured `SUBPIXEL` 3.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub const CAIRO_ANTIALIAS_SUBPIXEL: c_int = 3;

#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" {
  /// C: cairo.h 1743-1747: `void cairo_scaled_font_glyph_extents (cairo_scaled_font_t
  /// *scaled_font, const cairo_glyph_t *glyphs, int num_glyphs, cairo_text_extents_t *extents);`
  pub fn cairo_scaled_font_glyph_extents (scaled_font: *mut cairo_scaled_font_t,
                                          glyphs: *const cairo_glyph_t, num_glyphs: c_int,
                                          extents: *mut cairo_text_extents_t);
  /// C: cairo.h 1748-1759: `cairo_status_t cairo_scaled_font_text_to_glyphs (cairo_scaled_font_t
  /// *scaled_font, double x, double y, const char *utf8, int utf8_len, cairo_glyph_t **glyphs,
  /// int *num_glyphs, cairo_text_cluster_t **clusters, int *num_clusters,
  /// cairo_text_cluster_flags_t *cluster_flags);`
  pub fn cairo_scaled_font_text_to_glyphs (scaled_font: *mut cairo_scaled_font_t,
                                           x: c_double, y: c_double, utf8: *const c_char,
                                           utf8_len: c_int, glyphs: *mut *mut cairo_glyph_t,
                                           num_glyphs: *mut c_int, clusters: *mut *mut c_void,
                                           num_clusters: *mut c_int,
                                           cluster_flags: *mut c_int) -> cairo_status_t;
  /// C: cairo.h 1171-1172: `void cairo_glyph_free (cairo_glyph_t *glyphs);`
  pub fn cairo_glyph_free (glyphs: *mut cairo_glyph_t);
  /// C: cairo.h 1733-1736: `void cairo_scaled_font_extents (cairo_scaled_font_t *scaled_font,
  /// cairo_font_extents_t *extents);`
  pub fn cairo_scaled_font_extents (scaled_font: *mut cairo_scaled_font_t,
                                    extents: *mut cairo_font_extents_t);
  /// C: cairo-ft.h 89-90: `cairo_font_face_t *cairo_ft_font_face_create_for_pattern
  /// (FcPattern *pattern);`
  pub fn cairo_ft_font_face_create_for_pattern (pattern: *mut FcPattern) -> *mut cairo_font_face_t;
  /// C: cairo.h 1634-1635: `cairo_status_t cairo_font_face_status (cairo_font_face_t
  /// *font_face);`
  pub fn cairo_font_face_status (font_face: *mut cairo_font_face_t) -> cairo_status_t;
  /// C: cairo.h 1628-1629: `void cairo_font_face_destroy (cairo_font_face_t *font_face);`
  pub fn cairo_font_face_destroy (font_face: *mut cairo_font_face_t);
  /// C: cairo.h 3196-3198: `void cairo_matrix_init_scale (cairo_matrix_t *matrix, double sx,
  /// double sy);`
  pub fn cairo_matrix_init_scale (matrix: *mut cairo_matrix_t, sx: c_double, sy: c_double);
  /// C: cairo.h 3189-3190: `void cairo_matrix_init_identity (cairo_matrix_t *matrix);`
  pub fn cairo_matrix_init_identity (matrix: *mut cairo_matrix_t);
  /// C: cairo.h 1464-1465: `cairo_font_options_t *cairo_font_options_create (void);`
  pub fn cairo_font_options_create () -> *mut cairo_font_options_t;
  /// C: cairo.h 1470-1471: `void cairo_font_options_destroy (cairo_font_options_t *options);`
  pub fn cairo_font_options_destroy (options: *mut cairo_font_options_t);
  /// C: cairo.h 1486-1488: `void cairo_font_options_set_antialias (cairo_font_options_t *options,
  /// cairo_antialias_t antialias);`
  pub fn cairo_font_options_set_antialias (options: *mut cairo_font_options_t,
                                           antialias: c_int);
  /// C: cairo.h 1479-1481: `cairo_bool_t cairo_font_options_equal (const cairo_font_options_t
  /// *options, const cairo_font_options_t *other);`
  pub fn cairo_font_options_equal (options: *const cairo_font_options_t,
                                   other: *const cairo_font_options_t) -> c_int;
  /// C: cairo.h 1702-1706: `cairo_scaled_font_t *cairo_scaled_font_create (cairo_font_face_t
  /// *font_face, const cairo_matrix_t *font_matrix, const cairo_matrix_t *ctm, const
  /// cairo_font_options_t *options);`
  pub fn cairo_scaled_font_create (font_face: *mut cairo_font_face_t,
                                   font_matrix: *const cairo_matrix_t,
                                   ctm: *const cairo_matrix_t,
                                   options: *const cairo_font_options_t)
                                   -> *mut cairo_scaled_font_t;
  /// C: cairo.h 1717-1718: `cairo_status_t cairo_scaled_font_status (cairo_scaled_font_t
  /// *scaled_font);`
  pub fn cairo_scaled_font_status (scaled_font: *mut cairo_scaled_font_t) -> cairo_status_t;
  /// C: cairo.h 1711-1712: `void cairo_scaled_font_destroy (cairo_scaled_font_t *scaled_font);`
  pub fn cairo_scaled_font_destroy (scaled_font: *mut cairo_scaled_font_t);
  /// C: cairo.h 1775-1777: `void cairo_scaled_font_get_font_options (cairo_scaled_font_t
  /// *scaled_font, cairo_font_options_t *options);`
  pub fn cairo_scaled_font_get_font_options (scaled_font: *mut cairo_scaled_font_t,
                                             options: *mut cairo_font_options_t);
  /// C: cairo-ft.h 95-96: `FT_Face cairo_ft_scaled_font_lock_face (cairo_scaled_font_t
  /// *scaled_font);`
  pub fn cairo_ft_scaled_font_lock_face (scaled_font: *mut cairo_scaled_font_t) -> FT_Face;
  /// C: cairo-ft.h 98-99: `void cairo_ft_scaled_font_unlock_face (cairo_scaled_font_t
  /// *scaled_font);`
  pub fn cairo_ft_scaled_font_unlock_face (scaled_font: *mut cairo_scaled_font_t);
  /// C: cairo.h 704-705: `void cairo_set_source_rgb (cairo_t *cr, double red, double green,
  /// double blue);`
  pub fn cairo_set_source_rgb (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double);
  /// C: cairo.h 954-956: `void cairo_rectangle (cairo_t *cr, double x, double y, double width,
  /// double height);`
  pub fn cairo_rectangle (cr: *mut cairo_t, x: c_double, y: c_double, width: c_double,
                          height: c_double);
  /// C: cairo.h 996: `void cairo_fill (cairo_t *cr);`
  pub fn cairo_fill (cr: *mut cairo_t);
  /// C: cairo.h 1579-1581: `void cairo_set_scaled_font (cairo_t *cr, const cairo_scaled_font_t
  /// *scaled_font);`
  pub fn cairo_set_scaled_font (cr: *mut cairo_t, scaled_font: *const cairo_scaled_font_t);
  /// C: cairo.h 1590: `void cairo_show_glyphs (cairo_t *cr, const cairo_glyph_t *glyphs, int
  /// num_glyphs);`
  pub fn cairo_show_glyphs (cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: c_int);
}

/* C 786-806's two Xlib-typed cairo entry points: `Display` and `Visual` are `xterm.h`'s chain, so
   they are declared with the X11 arm rather than with the file gate (the arm that calls them).  */
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_x_windows"))]
unsafe extern "C" {
  /// C: cairo-xlib.h 48-52: `cairo_surface_t *cairo_xlib_surface_create (Display *dpy,
  /// Drawable drawable, Visual *visual, int width, int height);`
  pub fn cairo_xlib_surface_create (dpy: *mut Display, drawable: c_ulong, visual: *mut Visual,
                                    width: c_int, height: c_int) -> *mut cairo_surface_t;
  /// C: cairo.h 2651-2653: `void cairo_surface_get_font_options (cairo_surface_t *surface,
  /// cairo_font_options_t *options);`
  pub fn cairo_surface_get_font_options (surface: *mut cairo_surface_t,
                                         options: *mut cairo_font_options_t);
}

/* ----------------------------------------------------------------------------------------------
   C 22's Fontconfig half (`<cairo-ft.h>` pulls `<fontconfig/fontconfig.h>` in, and `ftfont.h`
   reaches it too).  Six names are read: `FcResult`, `FcMatrix`, `FcPattern`, `FcMatchPattern`-
   valued `FcConfigSubstitute`, `FcDefaultSubstitute`, `FcFontMatch`, `FcPatternGetMatrix` and
   `FcPatternDestroy`.  Every number is a clang reading of this machine's Fontconfig 2.17.1.
   ---------------------------------------------------------------------------------------------- */
/// C: fontconfig.h 279-285: `typedef enum _FcResult { FcResultMatch, FcResultNoMatch,
/// FcResultTypeMismatch, FcResultNoId, FcResultOutOfMemory } FcResult;` - an `int`; measured
/// `FcResultMatch` 0.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FcResult = c_int;
/// C: fontconfig.h 280: `FcResultMatch,` - measured 0 (the enum's first member).
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub const FcResultMatch: FcResult = 0;
/// C: fontconfig.h 295: `typedef struct _FcPattern FcPattern;` - the record is incomplete for
/// every use this unit makes of it (pointers only).
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct _FcPattern { _private: [u8; 0] }
/// C: fontconfig.h 295: the pointer type C spells `FcPattern *`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FcPattern = _FcPattern;
/// C: fontconfig.h 254-256: `typedef struct _FcMatrix { double xx, xy, yx, yy; } FcMatrix;` -
/// measured `sizeof` 32, align 8, `xx`@0 `xy`@8 `yx`@16 `yy`@24.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct FcMatrix
{
  pub xx: c_double,                            // fontconfig.h 255
  pub xy: c_double,
  pub yx: c_double,                            // fontconfig.h 256
  pub yy: c_double,
}
/// C: fontconfig.h 154: `#define FC_MATRIX               "matrix"     /* FcMatrix */`
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub const FC_MATRIX: &CStr = c"matrix";
/// C: fontconfig.h 266-268: `typedef enum _FcMatchKind { FcMatchPattern, FcMatchFont,
/// FcMatchScan } FcMatchKind;` - measured `FcMatchPattern` 0.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub const FcMatchPattern: c_int = 0;

#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" {
  /// C: fontconfig.h 1082-1085: `void FcConfigSubstitute (FcConfig *config, FcPattern *p,
  /// FcMatchKind kind);`
  pub fn FcConfigSubstitute (config: *mut c_void, p: *mut FcPattern, kind: c_int);
  /// C: fontconfig.h 1091-1092: `void FcDefaultSubstitute (FcPattern *pattern);`
  pub fn FcDefaultSubstitute (pattern: *mut FcPattern);
  /// C: fontconfig.h 1078-1080: `FcPattern *FcFontMatch (FcConfig *config, FcPattern *p,
  /// FcResult *result);`
  pub fn FcFontMatch (config: *mut c_void, p: *mut FcPattern, result: *mut FcResult)
                      -> *mut FcPattern;
  /// C: fontconfig.h 1070-1072: `FcResult FcPatternGetMatrix (const FcPattern *p, const char
  /// *object, int n, FcMatrix **m);`
  pub fn FcPatternGetMatrix (p: *const FcPattern, object: *const c_char, n: c_int,
                             m: *mut *mut FcMatrix) -> FcResult;
  /// C: fontconfig.h 961-962: `void FcPatternDestroy (FcPattern *p);`
  pub fn FcPatternDestroy (p: *mut FcPattern);
}

/* ----------------------------------------------------------------------------------------------
   C 22's FreeType half and `ftfont.h`'s `FT_SIZES_H`/`FT_FREETYPE_H` chain: `FT_Face`,
   `FT_Size`, `FT_SizeRec_`, `FT_Size_Metrics`, `FT_Matrix`, `FT_FACE_FLAG_SFNT` and the
   `FT_FaceRec_` members C 287-310 reads (`units_per_EM`, `underline_position`,
   `underline_thickness`, `face_flags`, `size`).  The record is written out up to `size` - every
   member above it is needed to place those fields at their measured offsets - and the members
   after it (`charmap` … `internal`) are not read by this unit, so the record stops there (the
   `xftfont.rs` `struct face` shape: a partial record whose *read* offsets are pinned; the fuller
   copy is `src/ftfont.rs`'s and both move into `include/freetype.rs`, P-90 (1)).  Every number is
   a clang reading of this machine's FreeType 2.14.3.
   ---------------------------------------------------------------------------------------------- */
/// C: fttypes.h 186: `typedef signed long FT_Long;` - measured `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Long = c_long;
/// C: fttypes.h 172: `typedef signed short FT_Short;` - measured `sizeof` 2.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Short = c_short;
/// C: fttypes.h 180: `typedef unsigned short FT_UShort;` - measured `sizeof` 2.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_UShort = c_ushort;
/// C: fttypes.h 166: `typedef signed int FT_Int;` - measured `sizeof` 4.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Int = c_int;
/// C: fttypes.h 328: `typedef char* FT_String;` - measured `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_String = c_char;
/// C: fttypes.h 393-400: `typedef struct FT_Matrix_ { FT_Fixed xx, xy; FT_Fixed yx, yy; }
/// FT_Matrix;` - measured `sizeof` 32, align 8, the four members at 0/8/16/24.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct FT_Matrix
{
  pub xx: FT_Long,                             // fttypes.h 395
  pub xy: FT_Long,
  pub yx: FT_Long,                             // fttypes.h 396
  pub yy: FT_Long,
}
/// C: freetype.h 679: `typedef struct FT_SizeRec_* FT_Size;` - measured `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Size = *mut FT_SizeRec_;
/// C: freetype.h 641: `typedef struct FT_FaceRec_* FT_Face;` - measured `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Face = *mut FT_FaceRec_;
/// C: fttypes.h 320-321: `typedef struct FT_Generic_Finalizer_ { void (*finalizer) (void*
/// object); } FT_Generic_Finalizer;` / 325-329: `typedef struct FT_Generic_ { void* data;
/// FT_Generic_Finalizer finalizer; } FT_Generic;` - measured `sizeof` 16, align 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct FT_Generic
{
  pub data: *mut c_void,                       // fttypes.h 327
  pub finalizer: *mut c_void,                  // fttypes.h 328
}
/// C: ftimage.h 213-221: `typedef struct FT_BBox_ { FT_Pos xMin, yMin; FT_Pos xMax, yMax; }
/// FT_BBox;` - measured `sizeof` 32, align 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct FT_BBox
{
  pub xMin: FT_Long,
  pub yMin: FT_Long,
  pub xMax: FT_Long,
  pub yMax: FT_Long,
}
/// C: fttypes.h 110: `typedef struct FT_CharMapRec_* FT_CharMap;` - measured `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct FT_CharMapRec_ { _private: [u8; 0] }
/// C: fttypes.h 110: the pointer type C spells `FT_CharMap`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_CharMap = *mut FT_CharMapRec_;
/// C: fttypes.h 234: `typedef struct FT_Bitmap_Size_ FT_Bitmap_Size;` - only a pointer appears
/// here (`available_sizes`, which this unit does not read), so the record is incomplete.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct FT_Bitmap_Size_ { _private: [u8; 0] }
/// C: fttypes.h 234: the record type C spells `FT_Bitmap_Size`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Bitmap_Size = FT_Bitmap_Size_;
/// C: ftimage.h 63: `typedef struct FT_GlyphSlotRec_* FT_GlyphSlot;` - measured `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct FT_GlyphSlotRec_ { _private: [u8; 0] }
/// C: ftimage.h 63: the pointer type C spells `FT_GlyphSlot`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec_;
/// C: ftmodapi.h 130: `typedef struct FT_DriverRec_* FT_Driver;` - measured `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct FT_DriverRec_ { _private: [u8; 0] }
/// C: ftmodapi.h 130: the pointer type C spells `FT_Driver`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Driver = *mut FT_DriverRec_;
/// C: fttypes.h 101: `typedef struct FT_MemoryRec_* FT_Memory;` - measured `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct FT_MemoryRec_ { _private: [u8; 0] }
/// C: fttypes.h 101: the pointer type C spells `FT_Memory`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Memory = *mut FT_MemoryRec_;
/// C: fttypes.h 247: `typedef struct FT_StreamRec_* FT_Stream;` - measured `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct FT_StreamRec_ { _private: [u8; 0] }
/// C: fttypes.h 247: the pointer type C spells `FT_Stream`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Stream = *mut FT_StreamRec_;
/// C: fttypes.h 262: `typedef struct FT_ListRec_ FT_ListRec;` - a `FT_ListNode head` and nothing
/// this unit reads; measured at its `FT_FaceRec_` offset only.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct FT_ListRec
{
  pub head: *mut c_void,                       // fttypes.h 264
  pub tail: *mut c_void,
}
/// C: fttypes.h 155: `typedef struct FT_Face_InternalRec_* FT_Face_Internal;` - measured
/// `sizeof` 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub struct FT_Face_InternalRec_ { _private: [u8; 0] }
/// C: fttypes.h 155: the pointer type C spells `FT_Face_Internal`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub type FT_Face_Internal = *mut FT_Face_InternalRec_;
/// C: freetype.h 1229-1288: `typedef struct FT_FaceRec_ { … } FT_FaceRec;` - written out to the
/// last member this unit reads (`size`, C 1270); measured `sizeof` 248 for the full record,
/// `face_flags`@16 `units_per_EM`@136 `underline_position`@148 `underline_thickness`@150
/// `size`@160.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct FT_FaceRec_
{
  pub num_faces: FT_Long,                      // freetype.h 1231
  pub face_index: FT_Long,
  pub face_flags: FT_Long,                     // freetype.h 1233
  pub style_flags: FT_Long,
  pub num_glyphs: FT_Long,                     // freetype.h 1235
  pub family_name: *mut FT_String,             // freetype.h 1242
  pub style_name: *mut FT_String,
  pub num_fixed_sizes: FT_Int,                 // freetype.h 1245
  pub available_sizes: *mut FT_Bitmap_Size,
  pub num_charmaps: FT_Int,                    // freetype.h 1248
  pub charmaps: *mut FT_CharMap,
  pub generic: FT_Generic,                     // freetype.h 1251
  pub bbox: FT_BBox,                           // freetype.h 1256
  pub units_per_EM: FT_UShort,                 // freetype.h 1258
  pub ascender: FT_Short,
  pub descender: FT_Short,
  pub height: FT_Short,                        // freetype.h 1261
  pub max_advance_width: FT_Short,             // freetype.h 1263
  pub max_advance_height: FT_Short,
  pub underline_position: FT_Short,            // freetype.h 1266
  pub underline_thickness: FT_Short,
  pub glyph: FT_GlyphSlot,                     // freetype.h 1269
  pub size: FT_Size,                           // freetype.h 1270
}
/// C: freetype.h 1974-1987: `typedef struct FT_Size_Metrics_ { FT_UShort x_ppem; FT_UShort
/// y_ppem; FT_Fixed x_scale; FT_Fixed y_scale; FT_Pos ascender; FT_Pos descender; FT_Pos height;
/// FT_Pos max_advance; } FT_Size_Metrics;` - measured `sizeof` 56, align 8, `height`@40.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct FT_Size_Metrics
{
  pub x_ppem: FT_UShort,                       // freetype.h 1976
  pub y_ppem: FT_UShort,
  pub x_scale: FT_Long,                        // freetype.h 1977
  pub y_scale: FT_Long,
  pub ascender: FT_Long,                       // freetype.h 1978
  pub descender: FT_Long,
  pub height: FT_Long,                         // freetype.h 1979
  pub max_advance: FT_Long,
}
/// C: freetype.h 1943-1952: `typedef struct FT_SizeRec_ { FT_Face face; FT_Generic generic;
/// FT_Size_Metrics metrics; FT_Size_Internal internal; } FT_SizeRec;` - measured `sizeof` 88,
/// align 8, `metrics`@24.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[repr(C)]
pub struct FT_SizeRec_
{
  pub face: FT_Face,                           // freetype.h 1945
  pub generic: FT_Generic,                     // freetype.h 1946
  pub metrics: FT_Size_Metrics,                // freetype.h 1949
  pub internal: *mut c_void,                   // freetype.h 1951
}
/// C: freetype.h 549: `#define FT_FACE_FLAG_SFNT ( 1L << 3 )` - measured 8.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub const FT_FACE_FLAG_SFNT: FT_Long = 8;

#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
const _: () = {
  core::assert! (std::mem::size_of::<cairo_glyph_t> () == 24);
  core::assert! (std::mem::align_of::<cairo_glyph_t> () == 8);
  core::assert! (std::mem::offset_of! (cairo_glyph_t, x) == 8);
  core::assert! (std::mem::size_of::<cairo_text_extents_t> () == 48);
  core::assert! (std::mem::offset_of! (cairo_text_extents_t, x_advance) == 32);
  core::assert! (std::mem::size_of::<cairo_font_extents_t> () == 40);
  core::assert! (std::mem::offset_of! (cairo_font_extents_t, height) == 16);
  core::assert! (std::mem::size_of::<cairo_matrix_t> () == 48);
  core::assert! (std::mem::offset_of! (cairo_matrix_t, xy) == 16);
  core::assert! (std::mem::size_of::<FcMatrix> () == 32);
  core::assert! (std::mem::offset_of! (FcMatrix, yx) == 16);
  core::assert! (std::mem::size_of::<FT_Matrix> () == 32);
  core::assert! (std::mem::offset_of! (FT_Matrix, yx) == 16);
  core::assert! (std::mem::size_of::<FT_FaceRec_> () == 168);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, face_flags) == 16);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, units_per_EM) == 136);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, underline_position) == 148);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, underline_thickness) == 150);
  core::assert! (std::mem::offset_of! (FT_FaceRec_, size) == 160);
  core::assert! (std::mem::size_of::<FT_SizeRec_> () == 88);
  core::assert! (std::mem::offset_of! (FT_SizeRec_, metrics) == 24);
  core::assert! (std::mem::size_of::<FT_Size_Metrics> () == 56);
  core::assert! (std::mem::offset_of! (FT_Size_Metrics, height) == 40);
};

/* ----------------------------------------------------------------------------------------------
   `ftfont.h`'s HarfBuzz chain (C 37-40 of the header: `<hb.h>` and `<hb-ft.h>` under
   `HAVE_HARFBUZZ`).  Three names are read: `hb_font_t`, `hb_font_destroy` and `hb_version_atleast`
   (C 696).  Both numbers are clang readings of this machine's HarfBuzz 14.4.0; the same surface is
   `src/hbfont.rs`'s segment, and `include/hb.rs` is its owed home (P-90 (1)).
   ---------------------------------------------------------------------------------------------- */
/// C: hb-common.h 95: `typedef int hb_bool_t;` - measured `sizeof` 4.
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "harfbuzz"))]
pub type hb_bool_t = c_int;
/// C: hb-common.h 520: `typedef struct hb_font_t hb_font_t;` - the record has no layout this unit
/// reads, so the P-72 (3) pointer alias is what stands for `hb_font_t *` (the `xftfont.rs` 848
/// spelling; HarfBuzz 14.4.0, `sizeof (hb_font_t *)` measured 8).
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "harfbuzz"))]
pub type hb_font_t = *mut c_void;
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "harfbuzz"))]
unsafe extern "C" {
  /// C: hb-font.h 1128-1129: `void hb_font_destroy (hb_font_t *font);`
  pub fn hb_font_destroy (font: hb_font_t);
  /// C: hb-version.h 111-112: `hb_bool_t hb_version_atleast (unsigned int major, unsigned int
  /// minor, unsigned int micro);`
  pub fn hb_version_atleast (major: c_uint, minor: c_uint, micro: c_uint) -> hb_bool_t;
}

/* ----------------------------------------------------------------------------------------------
   `ftfont.h`'s libotf chain (C 42-47 of the header: `<otf.h>` under `HAVE_LIBOTF`, `<m17n-flt.h>`
   under `HAVE_M17N_FLT`).  This file reads exactly two names of it (`OTF`, `OTF_close`, C 336) and
   no `m17n-flt.h` name at all.  **libotf is not installed on this machine** (`find
   /opt/homebrew/include -name otf.h` is empty), so both declarations are the published
   prototype's and **UNVERIFIED** (P-88 (2): `OTF` is an opaque pointer, so there is no layout to
   measure).
   ---------------------------------------------------------------------------------------------- */
/// C: otf.h: `typedef struct OTF *OTF;` - **UNVERIFIED**: libotf is not installed on this machine
/// (no `<otf.h>` anywhere under `/opt/homebrew/include`), so this is the published prototype's
/// spelling; both of this unit's uses are pointers, so no layout is involved (P-88 (2)).
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_libotf"))]
pub type OTF = *mut c_void;
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_libotf"))]
unsafe extern "C" {
  /// C: otf.h: `int OTF_close (OTF otf);` - **UNVERIFIED** for the same reason.
  pub fn OTF_close (otf: OTF) -> c_int;
}

/* C 24: `#include "lisp.h"` (`Lisp_Object`, `AREF`/`ASET`, `CONSP`/`EQ`/`NILP`, `XFIXNUM`,
   `assq_no_quit`, `make_fixnum`, `xmalloc`/`xrealloc`/`xfree`, `DEFSYM`, …).  */
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("include/lisp.rs");                   // C 24: the `#include "lisp.h"`

/* P-72 rule 4's two extra edges, taken *before* the term header below because the materializer
   expands the first reach of a file and the term-header mirrors have their own gated edges into
   them: C's `xterm.h` names `struct frame` and `struct font` as incomplete tags, which the
   mirrors spell as real pointer types (`*mut frame`, `*mut font`), so a unit that includes them
   must declare those records (P-72 rule 4 - the unit adds the mirror).  The *order* of the edges
   is the materializer's, not C's (the `xsettings.rs` Deviation 3 reading); the C positions are
   C 26's chain for `frame.h` (which `xterm.h` uses) and C 37's own line for `font.h`.  */
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("include/frame.rs");                  // C 26's chain: `struct frame` (P-72 rule 4)
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("include/font.rs");                   // C 37: `#include "font.h"`

/* C 25-33: the three-way terminal header.  Each arm is C's own: `xterm.h` for the X11 build
   (whose `USE_CAIRO` is C 4409's), `haikuterm.h`+`haiku_support.h`+`termchar.h` for the Haiku one
   (whose `USE_BE_CAIRO` is C 4500's) and `pgtkterm.h` for the pgtk one (C 4483's `USE_CAIRO`).
   The three are mutually exclusive, which is what makes `any (use_cairo, be_cairo)` the file gate
   and not a disjunction of three arms.  */
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_x_windows"))]
include!("include/xterm.rs");                  // C 26: `#include "xterm.h"`
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_haiku"))]
include!("include/haikuterm.rs");              // C 28: `#include "haikuterm.h"`
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_haiku"))]
include!("include/haiku_support.rs");          // C 29: `#include "haiku_support.h"`
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_haiku"))]
include!("include/termchar.rs");               // C 30: `#include "termchar.h"`
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"),
            not (feature = "have_x_windows"), not (feature = "have_haiku")))]
include!("include/pgtkterm.rs");               // C 32: `#include "pgtkterm.h"` (C's `#else` arm)
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("include/blockinput.rs");             // C 34: `#include "blockinput.h"`
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("include/charset.rs");                // C 35: `#include "charset.h"`
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("include/composite.rs");              // C 36: `#include "composite.h"`
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("include/ftfont.rs");                 // C 38: `#include "ftfont.h"` (`struct font_info`)
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
include!("include/pdumper.rs");                // C 39: `#include "pdumper.h"`
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_pgtk"))]
include!("include/xsettings.rs");              // C 41: `#include "xsettings.h"`

/* Use-site declarations the mirrors this unit includes do not carry, each with the reference
   header's line.  `xterm.h`'s four cairo-plumbing entry points (C 563-568 and 585-598 call them)
   are declared for the X11 arm; `haikuterm.h`'s two and `haiku_support.h`'s four for the Haiku
   one; `font.h`'s `font_build_object`, `register_font_driver`, the `ftfont_*` set and the
   `hbfont_*` trio for both (`include/font.rs` declares none of them; `include/ftfont.rs` carries
   only the three `ftfont.h` prototypes C 156-160 uses).  */
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_x_windows"))]
unsafe extern "C" {
  /// C: xterm.h 1813: `extern cairo_t *x_begin_cr_clip (struct frame *, GC);`
  pub fn x_begin_cr_clip (f: *mut c_void, gc: *mut c_void) -> *mut cairo_t;
  /// C: xterm.h 1814: `extern void x_end_cr_clip (struct frame *);`
  pub fn x_end_cr_clip (f: *mut c_void);
  /// C: xterm.h 1815: `extern void x_set_cr_source_with_gc_foreground (struct frame *, GC,
  /// bool);`
  pub fn x_set_cr_source_with_gc_foreground (f: *mut c_void, gc: *mut c_void, respect_alpha: bool);
  /// C: xterm.h 1816: `extern void x_set_cr_source_with_gc_background (struct frame *, GC,
  /// bool);`
  pub fn x_set_cr_source_with_gc_background (f: *mut c_void, gc: *mut c_void, respect_alpha: bool);
  /// C: Xlib.h 1593-1598: `extern Pixmap XCreatePixmap (Display *display, Drawable d, unsigned
  /// int width, unsigned int height, unsigned int depth);` (`include/xlib.rs` carries
  /// `XFreePixmap` but not this half; the use-site declaration P-55 prescribes.)
  pub fn XCreatePixmap (display: *mut Display, d: Drawable, width: c_uint, height: c_uint,
                        depth: c_uint) -> Pixmap;
}
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_haiku"))]
unsafe extern "C" {
  /// C: haikuterm.h 363: `extern cairo_t *haiku_begin_cr_clip (struct frame *, struct
  /// glyph_string *);`
  pub fn haiku_begin_cr_clip (f: *mut c_void, s: *mut glyph_string) -> *mut cairo_t;
  /// C: haikuterm.h 365: `extern void haiku_end_cr_clip (cairo_t *);`
  pub fn haiku_end_cr_clip (cr: *mut cairo_t);
  /// C: haiku_support.h 716: `extern void BView_cr_dump_clipping (void *, cairo_t *);`
  pub fn BView_cr_dump_clipping (view: *mut c_void, cr: *mut cairo_t);
  /// C: haiku_support.h 717: `extern void EmacsWindow_begin_cr_critical_section (void *);`
  pub fn EmacsWindow_begin_cr_critical_section (window: *mut c_void);
  /// C: haiku_support.h 718: `extern void EmacsWindow_end_cr_critical_section (void *);`
  pub fn EmacsWindow_end_cr_critical_section (window: *mut c_void);
  /* `be_use_subpixel_antialiasing` (haiku_support.h 728) is *not* redeclared: the mirror carries
     it (`include/haiku_support.rs` 705) and a second copy is the duplicate
     `check-mirror-dupes.py` exists to find.  */
}
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" {
  /// C: font.h 848: `extern Lisp_Object font_build_object (int, Lisp_Object, Lisp_Object,
  /// double);`
  pub fn font_build_object (size: c_int, r#type: Lisp_Object, props: Lisp_Object,
                            pixel_size: c_double) -> Lisp_Object;
  /// C: font.h 896: `extern void register_font_driver (struct font_driver const *, struct frame
  /// *);`
  pub fn register_font_driver (driver: *const font_driver, f: *mut c_void);












  /// C: font.h 955: `extern hb_font_t *fthbfont_begin_hb_font (struct font *, double *);`
  /// (`#ifdef HAVE_HARFBUZZ`; the return is C's `hb_font_t *`, the pointer alias above.)
  pub fn fthbfont_begin_hb_font (font: *mut font, position_unit: *mut c_double) -> hb_font_t;
}
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "harfbuzz"))]
unsafe extern "C" {



}

/* C 553-554's `s->face->foreground` / `->background` (the Haiku arm): `struct face` is
   `dispextern.h` 1787's record, and `include/dispextern.rs` keeps it behind
   `dispextern_face_records` while typing `glyph_string.face` as the untyped `*mut c_void`
   (P-72 (3)), so the two offsets this unit reads are pinned here against the reference's own
   header (the `xftfont.rs` 986-1000 shape).  The members between `id` and `foreground` are the
   `HAVE_WINDOW_SYSTEM` pair, and the Haiku arm's `#else` spells `gc` as `Emacs_GC *`; a
   window-system build's offsets are the same in both arms, which is what the ledger's Evidence 5
   measures.  */
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "be_cairo"))]
#[repr(C)]
pub struct face
{
  pub lface: [Lisp_Object; LFACE_VECTOR_SIZE as usize],   // dispextern.h 1791
  pub id: c_int,                                          // dispextern.h 1794
  pub gc: *mut c_void,                                    /// dispextern.h 1806: `Emacs_GC *gc;`
  pub stipple: isize,                                     // dispextern.h 1808
  pub foreground: c_ulong,                                // dispextern.h 1811
  pub background: c_ulong,                                // dispextern.h 1814
}
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "be_cairo"))]
const _: () = {
  core::assert! (std::mem::offset_of! (face, foreground) == 184);
  core::assert! (std::mem::offset_of! (face, background) == 192);
};

/* C 44-59: the file's own macros and the metrics cache's status enum, in C's order.  The file
   gate is spelled out on every item; `FG` below is never written - each `#[cfg]` names
   `any (feature = "use_cairo", feature = "be_cairo")` in full (P-86 (8)).  */

/* C 44-48: `#ifdef USE_BE_CAIRO` and the three colour macros (C 595-597 and 623-625 use them as
   expressions, so each is a `macro_rules!`; the Haiku arm's `uint32_t col` is their argument).  */
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "be_cairo"))]
macro_rules! RED_FROM_ULONG {
  ($color:expr) => { ((($color) >> 16) & 0xff) };
}
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "be_cairo"))]
macro_rules! GREEN_FROM_ULONG {
  ($color:expr) => { ((($color) >> 8) & 0xff) };
}
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "be_cairo"))]
macro_rules! BLUE_FROM_ULONG {
  ($color:expr) => { (($color) & 0xff) };
}

/// C 50: `#define METRICS_NCOLS_PER_ROW	(128)`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub const METRICS_NCOLS_PER_ROW: c_int = 128;

/* C 52-55: `enum metrics_status { METRICS_INVALID = -1, /* metrics entry is invalid */ };` - the
   enum's only member, and this file's only spelling of it (the `#define` pair below stores it in
   `descent`, so its value is what the status test compares).  */
/// C 54: `METRICS_INVALID = -1,    /* metrics entry is invalid */`
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
pub const METRICS_INVALID: c_int = -1;

/// C 57: `#define METRICS_STATUS(metrics)	((metrics)->ascent + (metrics)->descent)`.  The two
/// fields are `short`s, so C's sum is promoted to `int`; the port writes the promotion (C's own
/// `METRICS_SET_STATUS` stores `-1` there and this test must read it back).
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
macro_rules! METRICS_STATUS {
  ($metrics:expr) =>
    { ((*($metrics)).ascent as c_int + (*($metrics)).descent as c_int) };
}

/// C 58-59: `#define METRICS_SET_STATUS(metrics, status) ((metrics)->ascent = 0,
/// (metrics)->descent = (status))` - C's comma expression is a statement, and the store into
/// `descent` is the one C narrows to `short`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
macro_rules! METRICS_SET_STATUS {
  ($metrics:expr, $status:expr) => {{
    (*($metrics)).ascent = 0;
    (*($metrics)).descent = $status as c_short;
  }};
}

/// C 61-117: `ftcrfont_glyph_extents` - the metrics cache's lookup and fill.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe fn ftcrfont_glyph_extents (font: *mut font, glyph: c_uint,
                                  metrics: *mut font_metrics) -> c_int
{
  let ftcrfont_info: *mut font_info = font as *mut font_info;      // C 66
  let mut row: c_int;                                             // C 67
  let mut col: c_int;
  let mut cache: *mut font_metrics;                               // C 68

  row = (glyph / METRICS_NCOLS_PER_ROW as c_uint) as c_int;       // C 70
  col = (glyph % METRICS_NCOLS_PER_ROW as c_uint) as c_int;       // C 71
  if row >= (*ftcrfont_info).metrics_nrows as c_int               // C 72
    {
      (*ftcrfont_info).metrics =
	xrealloc ((*ftcrfont_info).metrics as *mut c_void,
		  std::mem::size_of::<*mut font_metrics> () * (row + 1) as usize)
	as *mut *mut font_metrics;                                // C 74-76
      /* C 77-79's `memset` counts bytes, so the destination goes through a `u8` pointer
	 (PORTING.md P-114).  */
      std::ptr::write_bytes (((*ftcrfont_info).metrics
			       .add ((*ftcrfont_info).metrics_nrows as usize)) as *mut u8,
			     0,
			     std::mem::size_of::<*mut font_metrics> ()
			       * (row + 1 - (*ftcrfont_info).metrics_nrows as c_int) as usize);  // C 77-79
      (*ftcrfont_info).metrics_nrows = (row + 1) as c_short;      // C 80
    }
  if (*ftcrfont_info).metrics.add (row as usize).read ().is_null ()   // C 82
    {
      let mut new: *mut font_metrics;                             // C 84
      let mut i: c_int;                                           // C 85

      new = xmalloc (std::mem::size_of::<font_metrics> ()
		     * METRICS_NCOLS_PER_ROW as usize) as *mut font_metrics;   // C 87
      i = 0;
      while i < METRICS_NCOLS_PER_ROW                             // C 88
	{
	  METRICS_SET_STATUS! (new.add (i as usize), METRICS_INVALID);   // C 89
	  i += 1;
	}
      (*ftcrfont_info).metrics.add (row as usize).write (new);     // C 90
    }
  cache = (*ftcrfont_info).metrics.add (row as usize).read ().add (col as usize);   // C 92

  if METRICS_STATUS! (cache) == METRICS_INVALID                   // C 94
    {
      let mut cr_glyph: cairo_glyph_t =
	cairo_glyph_t { index: glyph as c_ulong, x: 0.0, y: 0.0 };   // C 96
      let mut extents: cairo_text_extents_t = std::mem::zeroed ();   // C 97

      cairo_scaled_font_glyph_extents ((*ftcrfont_info).cr_scaled_font,
				       &raw const cr_glyph, 1, &raw mut extents);   // C 99-100
      (*cache).lbearing = floor (extents.x_bearing) as c_short;   // C 101
      (*cache).rbearing = ceil (extents.width + extents.x_bearing) as c_short;   // C 102
      (*cache).width = lround (extents.x_advance) as c_short;     // C 103
      /* The subtraction of a small number is to avoid rounding up
	 to floating-point inaccuracies with some fonts, which then
	 could cause unpleasant effects while scrolling (see bug
	 #44284), since we then think that a glyph row's ascent is too
	 small to accommodate a glyph with a higher phys_ascent.  (C 104-108)  */
      (*cache).ascent = ceil (- extents.y_bearing - 1.0 / 256.0) as c_short;   // C 109
      (*cache).descent = ceil (extents.height + extents.y_bearing) as c_short;   // C 110
    }

  if ! metrics.is_null ()                                         // C 113
    { std::ptr::write (metrics, std::ptr::read (cache)); }         // C 114

  return (*cache).width as c_int;                                 // C 116
}

/// C 119-123: `ftcrfont_list`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_list (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{
  return ftfont_list2 (f, spec, Qftcr ());                        // C 122
}

/// C 125-129: `ftcrfont_match`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_match (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{
  return ftfont_match2 (f, spec, Qftcr ());                       // C 128
}

/// C 131-322: `ftcrfont_open`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_open (f: *mut c_void, entity: Lisp_Object, pixel_size: c_int)
				    -> Lisp_Object
{
  let mut result: FcResult = 0;                                   // C 134
  let mut val: Lisp_Object;                                       // C 135
  let mut filename: Lisp_Object;
  let mut font_object: Lisp_Object;
  let mut pat: *mut FcPattern;                                    // C 136
  let mut r#match: *mut FcPattern;
  let mut ftcrfont_info: *mut font_info;                          // C 137
  let mut font: *mut font;                                        // C 138
  let mut size: c_double = 0.0;                                   // C 139
  let mut font_face: *mut cairo_font_face_t;                      // C 140
  let mut extents: cairo_font_extents_t = std::mem::zeroed ();    // C 141
  let mut ft_face: FT_Face;                                       // C 142
  let mut matrix: *mut FcMatrix = std::ptr::null_mut ();           // C 143

  val = assq_no_quit (QCfont_entity (), AREF (entity, FONT_EXTRA_INDEX as isize));   // C 145
  if ! CONSP (val)                                                // C 146
    { return Qnil (); }                                           // C 147
  val = XCDR (val);                                               // C 148
  filename = XCAR (val);                                          // C 149
  size = XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) as c_double;   // C 150
  if size == 0.0                                                  // C 151
    { size = pixel_size as c_double; }                            // C 152

  block_input ();                                                 // C 154

  pat = ftfont_entity_pattern (entity, pixel_size) as *mut FcPattern;   // C 156
  FcConfigSubstitute (std::ptr::null_mut (), pat, FcMatchPattern);   // C 157
  FcDefaultSubstitute (pat);                                      // C 158
  r#match = FcFontMatch (std::ptr::null_mut (), pat, &raw mut result);   // C 159
  ftfont_fix_match (pat as *mut c_void, r#match as *mut c_void);   // C 160

  FcPatternDestroy (pat);                                         // C 162
  font_face = cairo_ft_font_face_create_for_pattern (r#match);    // C 163
  if font_face.is_null ()                                         // C 164
      || cairo_font_face_status (font_face) != CAIRO_STATUS_SUCCESS   // C 165
    {
      unblock_input ();                                           // C 167
      FcPatternDestroy (r#match);                                 // C 168
      return Qnil ();                                             // C 169
    }
  let mut font_matrix: cairo_matrix_t = std::mem::zeroed ();      // C 171
  let mut ctm: cairo_matrix_t = std::mem::zeroed ();
  cairo_matrix_init_scale (&raw mut font_matrix, pixel_size as c_double,
			   pixel_size as c_double);               // C 172
  cairo_matrix_init_identity (&raw mut ctm);                      // C 173

  /* C 175-179: `#ifdef HAVE_PGTK` / `xsettings_get_font_options` / `#else` /
     `cairo_font_options_create` / `#endif` - both arms carried, the frozen one selected by the
     feature (P-51).  */
  let mut options: *mut cairo_font_options_t;                     // C 176/178
  #[cfg (feature = "have_pgtk")]
  { options = xsettings_get_font_options (); }                    // C 176
  #[cfg (not (feature = "have_pgtk"))]
  { options = cairo_font_options_create (); }                     // C 178
  /* C 180-183: `#ifdef USE_BE_CAIRO` / `if (be_use_subpixel_antialiasing ()) …` / `#endif`.  */
  #[cfg (feature = "be_cairo")]
  if be_use_subpixel_antialiasing ()                              // C 181
    { cairo_font_options_set_antialias (options, CAIRO_ANTIALIAS_SUBPIXEL); }   // C 182

  let mut scaled_font: *mut cairo_scaled_font_t                   // C 184-185
    = cairo_scaled_font_create (font_face, &raw const font_matrix, &raw const ctm, options);
  cairo_font_face_destroy (font_face);                            // C 186
  cairo_font_options_destroy (options);                           // C 187
  unblock_input ();                                               // C 188
  if scaled_font.is_null ()                                       // C 189
      || cairo_scaled_font_status (scaled_font) != CAIRO_STATUS_SUCCESS   // C 190
    {
      FcPatternDestroy (r#match);                                 // C 192
      return Qnil ();                                             // C 193
    }
  ft_face = cairo_ft_scaled_font_lock_face (scaled_font);         // C 195
  if ft_face.is_null ()                                           // C 196
    {
      FcPatternDestroy (r#match);                                 // C 198
      return Qnil ();                                             // C 199
    }

  font_object = font_build_object (VECSIZE (std::mem::size_of::<font_info> ()) as c_int,
				   AREF (entity, FONT_TYPE_INDEX as isize),
				   entity, size);                 // C 202-204
  ASET (font_object, FONT_FILE_INDEX as isize, filename);         // C 205
  font = XFONT_OBJECT (font_object);                              // C 206
  (*font).pixel_size = size as c_int;                             // C 207
  /* C 208-213: `#ifdef HAVE_HARFBUZZ` / the `ftcrhbfont_driver` choice / `#endif`, then the
     `ftcrfont_driver` assignment; both arms are carried as cfg-selected statements (P-51).  */
  #[cfg (feature = "harfbuzz")]
  if EQ (AREF (font_object, FONT_TYPE_INDEX as isize), Qftcrhb ())   // C 209
    { (*font).driver = std::ptr::addr_of! (ftcrhbfont_driver) as *const c_void; }   // C 210
  else
    { (*font).driver = std::ptr::addr_of! (ftcrfont_driver) as *const c_void; }     // C 213
  #[cfg (not (feature = "harfbuzz"))]
  {
    (*font).driver = std::ptr::addr_of! (ftcrfont_driver) as *const c_void;         // C 213
  }
  (*font).encoding_charset = -1;                                  // C 214
  (*font).repertory_charset = -1;

  ftcrfont_info = font as *mut font_info;                         // C 216
  (*ftcrfont_info).cr_scaled_font = scaled_font;                  // C 217

  /* This means that there's no need of transformation.  (C 219)  */
  (*ftcrfont_info).matrix.xx = 0;                                 // C 220
  if FcPatternGetMatrix (r#match, FC_MATRIX.as_ptr (), 0, &raw mut matrix) == FcResultMatch   // C 221
    {
      (*ftcrfont_info).matrix.xx = (0x10000 as c_double * (*matrix).xx) as FT_Long;   // C 223
      (*ftcrfont_info).matrix.yy = (0x10000 as c_double * (*matrix).yy) as FT_Long;   // C 224
      (*ftcrfont_info).matrix.xy = (0x10000 as c_double * (*matrix).xy) as FT_Long;   // C 225
      (*ftcrfont_info).matrix.yx = (0x10000 as c_double * (*matrix).yx) as FT_Long;   // C 226
    }

  (*ftcrfont_info).metrics = std::ptr::null_mut ();               // C 229
  (*ftcrfont_info).metrics_nrows = 0;                             // C 230

  block_input ();                                                 // C 232
  let mut stack_glyph: cairo_glyph_t = std::mem::zeroed ();       // C 233
  (*font).min_width = 0;                                          // C 234
  (*font).max_width = 0;
  (*font).average_width = 0;                                      // C 235
  (*font).space_width = 0;
  let mut n: c_int = 0;                                           // C 236
  let mut c: c_char = 32;                                         // C 237
  while c < 127
    {
      let mut glyphs: *mut cairo_glyph_t = &raw mut stack_glyph;   // C 239
      let mut num_glyphs: c_int = 1;                              // C 240
      let status: cairo_status_t =
	cairo_scaled_font_text_to_glyphs ((*ftcrfont_info).cr_scaled_font,
					  0.0, 0.0, &raw const c, 1, &raw mut glyphs,
					  &raw mut num_glyphs, std::ptr::null_mut (),
					  std::ptr::null_mut (), std::ptr::null_mut ());   // C 241-244

      /* In order to simulate the Xft behavior, we use metrics of
	 glyph ID 0 if there is no glyph for an ASCII printable.  (C 246-247)  */
      if status != CAIRO_STATUS_SUCCESS                           // C 248
	{ stack_glyph.index = 0; }                                // C 249
      else if glyphs != &raw mut stack_glyph                      // C 250
	{
	  cairo_glyph_free (glyphs);                              // C 252
	  stack_glyph.index = 0;                                  // C 253
	}
      let this_width: c_int =
	ftcrfont_glyph_extents (font, stack_glyph.index as c_uint,
				std::ptr::null_mut ());           // C 255
      if this_width > 0                                           // C 256
	{
	  if (*font).min_width == 0 || (*font).min_width > this_width   // C 258
	    { (*font).min_width = this_width; }                   // C 259
	  if this_width > (*font).max_width                       // C 260
	    { (*font).max_width = this_width; }                   // C 261
	  if c == 32                                              // C 262
	    { (*font).space_width = this_width; }                 // C 263
	  (*font).average_width += this_width;                    // C 264
	  n += 1;                                                 // C 265
	}
      c += 1;                                                     // C 237's `c++` (C 267)
    }
  if n != 0                                                       // C 268
    { (*font).average_width /= n; }                               // C 269

  cairo_scaled_font_extents ((*ftcrfont_info).cr_scaled_font, &raw mut extents);   // C 271
  (*font).ascent = lround (extents.ascent) as c_int;              // C 272
  val = assq_no_quit (QCminspace (), AREF (entity, FONT_EXTRA_INDEX as isize));   // C 273
  if ! (CONSP (val) && NILP (XCDR (val)))                         // C 274
    {
      (*font).descent = lround (extents.descent) as c_int;        // C 276
      (*font).height = (*font).ascent + (*font).descent;          // C 277
    }
  else
    {
      (*font).height = lround (extents.height) as c_int;          // C 281
      (*font).descent = (*font).height - (*font).ascent;          // C 282
    }

  if XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) == 0       // C 285
    {
      let upEM: c_int = (*ft_face).units_per_EM as c_int;         // C 287

      (*font).underline_position
	= (-((*ft_face).underline_position as c_double) * size / upEM as c_double) as c_int;   // C 289
      (*font).underline_thickness
	= (((*ft_face).underline_thickness as c_double) * size / upEM as c_double) as c_int;   // C 290
      if (*font).underline_thickness > 2                          // C 291
	{ (*font).underline_position -= (*font).underline_thickness / 2; }   // C 292
    }
  else
    {
      (*font).underline_position = -1;                            // C 296
      (*font).underline_thickness = 0;                            // C 297
    }
  /* C 299-302: `#ifdef HAVE_LIBOTF` (`maybe_otf` and `otf`).  */
  #[cfg (feature = "have_libotf")]
  { (*ftcrfont_info).maybe_otf = ((*ft_face).face_flags & FT_FACE_FLAG_SFNT) != 0; }   // C 300
  #[cfg (feature = "have_libotf")]
  { (*ftcrfont_info).otf = std::ptr::null_mut (); }               // C 301
  /* C 303-305: `#ifdef HAVE_HARFBUZZ` (`hb_font`).  */
  #[cfg (feature = "harfbuzz")]
  { (*ftcrfont_info).hb_font = std::ptr::null_mut (); }           // C 304
  if (*ft_face).units_per_EM != 0                                 // C 306
    { (*ftcrfont_info).bitmap_position_unit = 0.0; }              // C 307
  else
    { (*ftcrfont_info).bitmap_position_unit =
	extents.height / (*(*ft_face).size).metrics.height as c_double; }   // C 309-310
  cairo_ft_scaled_font_unlock_face (scaled_font);                 // C 311
  (*ftcrfont_info).ft_size = std::ptr::null_mut ();               // C 312
  unblock_input ();                                               // C 313

  (*font).baseline_offset = 0;                                    // C 315
  (*font).relative_compose = 0;                                   // C 316
  (*font).default_ascent = 0;                                     // C 317
  (*font).vertical_centering = false;                             // C 318
  eassert! ((*font).max_width < 512 * 1024 * 1024);               // C 319

  return font_object;                                             // C 321
}

/// C 324-362: `ftcrfont_close`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_close (font: *mut font)
{
  if font_data_structures_may_be_ill_formed ()                    // C 327
    { return; }                                                   // C 328

  let ftcrfont_info: *mut font_info = font as *mut font_info;     // C 330

  block_input ();                                                 // C 332
  /* C 333-339: `#ifdef HAVE_LIBOTF`.  */
  #[cfg (feature = "have_libotf")]
  if ! (*ftcrfont_info).otf.is_null ()                            // C 334
    {
      OTF_close ((*ftcrfont_info).otf);                           // C 336
      (*ftcrfont_info).otf = std::ptr::null_mut ();               // C 337
    }
  /* C 340-346: `#ifdef HAVE_HARFBUZZ`.  */
  #[cfg (feature = "harfbuzz")]
  if ! (*ftcrfont_info).hb_font.is_null ()                        // C 341
    {
      hb_font_destroy ((*ftcrfont_info).hb_font);                 // C 343
      (*ftcrfont_info).hb_font = std::ptr::null_mut ();           // C 344
    }
  if ! (*ftcrfont_info).metrics.is_null ()                        // C 347
    {
      let mut i: c_int = 0;
      while i < (*ftcrfont_info).metrics_nrows as c_int           // C 349
	{
	  if ! (*ftcrfont_info).metrics.add (i as usize).read ().is_null ()   // C 350
	    { xfree ((*ftcrfont_info).metrics.add (i as usize).read () as *mut c_void); }   // C 351
	  i += 1;
	}
      if ! (*ftcrfont_info).metrics.is_null ()                    // C 352
	{ xfree ((*ftcrfont_info).metrics as *mut c_void); }      // C 353
      (*ftcrfont_info).metrics = std::ptr::null_mut ();           // C 354
    }
  if ! (*ftcrfont_info).cr_scaled_font.is_null ()                 // C 356
    {
      cairo_scaled_font_destroy ((*ftcrfont_info).cr_scaled_font);   // C 358
      (*ftcrfont_info).cr_scaled_font = std::ptr::null_mut ();    // C 359
    }
  unblock_input ();                                               // C 361
}

/// C 364-382: `ftcrfont_has_char`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_has_char (font: Lisp_Object, c: c_int) -> c_int
{
  if FONT_ENTITY_P (font)                                         // C 367
    { return ftfont_has_char (font, c); }                         // C 368

  let mut cs: *mut charset = std::ptr::null_mut ();               // C 370

  if EQ (AREF (font, FONT_ADSTYLE_INDEX as isize), Qja ())        // C 372
      && charset_jisx0208 >= 0
    { cs = CHARSET_FROM_ID! (charset_jisx0208); }                  // C 374
  else if EQ (AREF (font, FONT_ADSTYLE_INDEX as isize), Qko ())   // C 375
      && charset_ksc5601 >= 0
    { cs = CHARSET_FROM_ID! (charset_ksc5601); }                   // C 377
  if ! cs.is_null ()                                              // C 378
    { return (ENCODE_CHAR! (cs, c) != CHARSET_INVALID_CODE! (cs)) as c_int; }   // C 379

  return -1;                                                      // C 381
}

/// C 384-408: `ftcrfont_encode_char`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_encode_char (font: *mut font, c: c_int) -> c_uint
{
  let ftcrfont_info: *mut font_info = font as *mut font_info;     // C 387
  let mut code: c_uint = FONT_INVALID_CODE;                       // C 388
  let mut utf8: [c_uchar; MAX_MULTIBYTE_LENGTH as usize]
    = [0; MAX_MULTIBYTE_LENGTH as usize];                         // C 389
  let utf8len: c_int = CHAR_STRING (c, &raw mut utf8 as *mut c_uchar);   // C 390
  let mut stack_glyph: cairo_glyph_t = std::mem::zeroed ();       // C 391
  let mut glyphs: *mut cairo_glyph_t = &raw mut stack_glyph;      // C 392
  let mut num_glyphs: c_int = 1;                                  // C 393

  if cairo_scaled_font_text_to_glyphs ((*ftcrfont_info).cr_scaled_font, 0.0, 0.0,
				       &raw mut utf8 as *mut c_char, utf8len,
				       &raw mut glyphs, &raw mut num_glyphs,
				       std::ptr::null_mut (), std::ptr::null_mut (),
				       std::ptr::null_mut ())
      == CAIRO_STATUS_SUCCESS                                     // C 395-399
    {
      if glyphs != &raw mut stack_glyph                           // C 401
	{ cairo_glyph_free (glyphs); }                            // C 402
      else if stack_glyph.index != 0                              // C 403
	{ code = stack_glyph.index as c_uint; }                   // C 404
    }

  return code;                                                    // C 407
}

/// C 410-440: `ftcrfont_text_extents`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_text_extents (font: *mut font, code: *const c_uint, nglyphs: c_int,
					    metrics: *mut font_metrics)
{
  let mut width: c_int;                                           // C 416
  let mut i: c_int;

  width = ftcrfont_glyph_extents (font, *code.offset (0), metrics);   // C 418
  i = 1;
  while i < nglyphs                                                // C 419
    {
      let mut m: font_metrics = std::mem::zeroed ();               // C 421
      let w: c_int = ftcrfont_glyph_extents (font, *code.offset (i as isize),
					     if ! metrics.is_null () { &raw mut m }
					     else { std::ptr::null_mut () });   // C 422

      if ! metrics.is_null ()                                      // C 424
	{
	  if (width + m.lbearing as c_int) < ((*metrics).lbearing as c_int)   // C 426
	    { (*metrics).lbearing = (width + m.lbearing as c_int) as c_short; }   // C 427
	  if (width + m.rbearing as c_int) > ((*metrics).rbearing as c_int)   // C 428
	    { (*metrics).rbearing = (width + m.rbearing as c_int) as c_short; }   // C 429
	  if m.ascent > (*metrics).ascent                              // C 430
	    { (*metrics).ascent = m.ascent; }                          // C 431
	  if m.descent > (*metrics).descent                            // C 432
	    { (*metrics).descent = m.descent; }                        // C 433
	}
      width += w;                                                   // C 435
      i += 1;
    }

  if ! metrics.is_null ()                                           // C 438
    { (*metrics).width = width as c_short; }                        // C 439
}

/// C 442-460: `ftcrfont_get_bitmap`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_get_bitmap (font: *mut font, code: c_uint,
					  bitmap: *mut font_bitmap, bits_per_pixel: c_int)
					  -> c_int
{
  let ftcrfont_info: *mut font_info = font as *mut font_info;      // C 446

  if (*ftcrfont_info).bitmap_position_unit != 0.0                  // C 448
    { return -1; }                                                 // C 449

  let scaled_font: *mut cairo_scaled_font_t = (*ftcrfont_info).cr_scaled_font;   // C 451
  let ft_face: FT_Face = cairo_ft_scaled_font_lock_face (scaled_font);   // C 452

  (*ftcrfont_info).ft_size = (*ft_face).size;                      // C 454
  let result: c_int = ftfont_get_bitmap (font, code, bitmap, bits_per_pixel);   // C 455
  cairo_ft_scaled_font_unlock_face (scaled_font);                  // C 456
  (*ftcrfont_info).ft_size = std::ptr::null_mut ();                // C 457

  return result;                                                   // C 459
}

/// C 462-480: `ftcrfont_anchor_point`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_anchor_point (font: *mut font, code: c_uint, idx: c_int,
					    x: *mut c_int, y: *mut c_int) -> c_int
{
  let ftcrfont_info: *mut font_info = font as *mut font_info;      // C 466

  if (*ftcrfont_info).bitmap_position_unit != 0.0                  // C 468
    { return -1; }                                                 // C 469

  let scaled_font: *mut cairo_scaled_font_t = (*ftcrfont_info).cr_scaled_font;   // C 471
  let ft_face: FT_Face = cairo_ft_scaled_font_lock_face (scaled_font);   // C 472

  (*ftcrfont_info).ft_size = (*ft_face).size;                      // C 474
  let result: c_int = ftfont_anchor_point (font, code, idx, x, y);   // C 475
  cairo_ft_scaled_font_unlock_face (scaled_font);                  // C 476
  (*ftcrfont_info).ft_size = std::ptr::null_mut ();                // C 477

  return result;                                                   // C 479
}

/* C 482-497: `#ifdef HAVE_LIBOTF` / `ftcrfont_otf_capability` / `#endif`.  */
/// C 483-496: `ftcrfont_otf_capability`.
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_libotf"))]
unsafe extern "C" fn ftcrfont_otf_capability (font: *mut font) -> Lisp_Object
{
  let ftcrfont_info: *mut font_info = font as *mut font_info;      // C 486
  let scaled_font: *mut cairo_scaled_font_t = (*ftcrfont_info).cr_scaled_font;   // C 487
  let ft_face: FT_Face = cairo_ft_scaled_font_lock_face (scaled_font);   // C 488

  (*ftcrfont_info).ft_size = (*ft_face).size;                      // C 490
  let result: Lisp_Object = ftfont_otf_capability (font);          // C 491
  cairo_ft_scaled_font_unlock_face (scaled_font);                  // C 492
  (*ftcrfont_info).ft_size = std::ptr::null_mut ();                // C 493

  return result;                                                   // C 495
}

/* C 499-519: `#if defined HAVE_M17N_FLT && defined HAVE_LIBOTF` / `ftcrfont_shape` / `#endif`. */
/// C 500-518: `ftcrfont_shape`.
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_m17n_flt",
	    feature = "have_libotf"))]
unsafe extern "C" fn ftcrfont_shape (lgstring: Lisp_Object, direction: Lisp_Object) -> Lisp_Object
{
  let font: *mut font = CHECK_FONT_GET_OBJECT (LGSTRING_FONT (lgstring));   // C 503
  let ftcrfont_info: *mut font_info = font as *mut font_info;      // C 504

  if (*ftcrfont_info).bitmap_position_unit != 0.0                  // C 506
    { return make_fixnum (0); }                                    // C 507

  let scaled_font: *mut cairo_scaled_font_t = (*ftcrfont_info).cr_scaled_font;   // C 509
  let ft_face: FT_Face = cairo_ft_scaled_font_lock_face (scaled_font);   // C 510

  (*ftcrfont_info).ft_size = (*ft_face).size;                      // C 512
  let result: Lisp_Object = ftfont_shape (lgstring, direction);    // C 513
  cairo_ft_scaled_font_unlock_face (scaled_font);                  // C 514
  (*ftcrfont_info).ft_size = std::ptr::null_mut ();                // C 515

  return result;                                                   // C 517
}

/* C 521-536: `#if defined HAVE_OTF_GET_VARIATION_GLYPHS || defined
   HAVE_FT_FACE_GETCHARVARIANTINDEX` / `ftcrfont_variation_glyphs` / `#endif`.  */
/// C 522-535: `ftcrfont_variation_glyphs`.
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"),
	    any (feature = "have_otf_get_variation_glyphs",
		 feature = "have_ft_face_getcharvariantindex")))]
unsafe extern "C" fn ftcrfont_variation_glyphs (font: *mut font, c: c_int,
						variations: *mut c_uint) -> c_int
{
  let ftcrfont_info: *mut font_info = font as *mut font_info;      // C 525
  let scaled_font: *mut cairo_scaled_font_t = (*ftcrfont_info).cr_scaled_font;   // C 526
  let ft_face: FT_Face = cairo_ft_scaled_font_lock_face (scaled_font);   // C 527

  (*ftcrfont_info).ft_size = (*ft_face).size;                      // C 529
  let result: c_int = ftfont_variation_glyphs (font, c, variations);   // C 530
  cairo_ft_scaled_font_unlock_face (scaled_font);                  // C 531
  (*ftcrfont_info).ft_size = std::ptr::null_mut ();                // C 532

  return result;                                                   // C 534
}

/// C 538-642: `ftcrfont_draw` - the one function whose four-way arm (`use_be_cairo` against the
/// `have_x_windows`/pgtk pair) is spelled at each site, in C's own nesting order.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn ftcrfont_draw (s: *mut glyph_string, from: c_int, to: c_int, x: c_int,
				    y: c_int, with_background: bool) -> c_int
{
  let f: *mut frame = (*s).f;                                     // C 542
  let ftcrfont_info: *mut font_info = (*s).font as *mut font_info;   // C 543
  let mut cr: *mut cairo_t;                                       // C 544
  let mut glyphs: *mut cairo_glyph_t;                             // C 545
  let len: c_int = to - from;                                     // C 546
  let mut i: c_int;                                               // C 547
  /* C 610 reassigns the parameter (the `term.rs` 4279 `let mut x = x;` shape).  */
  let mut x = x;
  /* C 548-559: `#ifdef USE_BE_CAIRO` and the two cursor colours.  */
  #[cfg (feature = "be_cairo")]
  let mut be_foreground: c_ulong = 0;                             // C 549
  #[cfg (feature = "be_cairo")]
  let mut be_background: c_ulong = 0;
  /* C 548-559's `if` / `else`: the first arm is the frozen one when `be_cairo` is off, so the
     gated `if` is what C's text holds and the `#else`'s one statement stands a second time under
     the negation (P-51 - an attribute on `else` is `expected expression, found keyword else`,
     the `xftfont.rs` Deviation 3 shape).  */
  #[cfg (feature = "be_cairo")]
  if (*s).hl != DRAW_CURSOR                                       // C 551
    {
      be_foreground = (*((*s).face as *mut face)).foreground;     // C 553
      be_background = (*((*s).face as *mut face)).background;     // C 554
    }
  #[cfg (feature = "be_cairo")]
  if (*s).hl == DRAW_CURSOR                                       // C 551
    { haiku_merge_cursor_foreground (s as *mut c_void, &raw mut be_foreground,
				      &raw mut be_background); }   // C 557-558

  block_input ();                                                 // C 561

  /* C 563-581: `#ifndef USE_BE_CAIRO` / `#ifdef HAVE_X_WINDOWS` / `#else` / `#endif` / `#else`
     / `#endif` - the three arms, each gated by its own condition.  */
  #[cfg (not (feature = "be_cairo"))]
  #[cfg (feature = "have_x_windows")]
  { cr = x_begin_cr_clip (f as *mut c_void, (*s).gc); }           // C 565
  #[cfg (not (feature = "be_cairo"))]
  #[cfg (not (feature = "have_x_windows"))]
  { cr = pgtk_begin_cr_clip (f); }                                // C 567
  #[cfg (feature = "be_cairo")]
  {
    /* Presumably the draw lock is already held by haiku_draw_glyph_string.  (C 570-571)  */
    EmacsWindow_begin_cr_critical_section (FRAME_HAIKU_WINDOW! (f));   // C 572
    cr = haiku_begin_cr_clip (f as *mut c_void, s);               // C 573
    if cr.is_null ()                                              // C 574
      {
	EmacsWindow_end_cr_critical_section (FRAME_HAIKU_WINDOW! (f));   // C 576
	unblock_input ();                                         // C 577
	return 0;                                                 // C 578
      }
    BView_cr_dump_clipping (FRAME_HAIKU_DRAWABLE! (f), cr);        // C 580
  }

  if with_background                                             // C 583
    {
      #[cfg (not (feature = "be_cairo"))]
      #[cfg (feature = "have_x_windows")]
      { x_set_cr_source_with_gc_background (f as *mut c_void, (*s).gc, (*s).hl != DRAW_CURSOR); }   // C 587
      #[cfg (not (feature = "be_cairo"))]
      #[cfg (not (feature = "have_x_windows"))]
      { pgtk_set_cr_source_with_color (f, (*s).xgcv.background, (*s).hl != DRAW_CURSOR); }   // C 589-590
      #[cfg (feature = "be_cairo")]
      {
	let col: u32 = be_background as u32;                      // C 593

	cairo_set_source_rgb (cr, RED_FROM_ULONG! (col) as c_double / 255.0,
			      GREEN_FROM_ULONG! (col) as c_double / 255.0,
			      BLUE_FROM_ULONG! (col) as c_double / 255.0);   // C 595-597
      }
      cairo_rectangle (cr, x as c_double, y as c_double - FONT_BASE! ((*s).font) as c_double,
		       (*s).width as c_double, FONT_HEIGHT! ((*s).font) as c_double);   // C 599-600
      cairo_fill (cr);                                            // C 601
    }

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  glyphs = SAFE_ALLOCA! (sa_avail,
			 std::mem::size_of::<cairo_glyph_t> () * len as usize)
	     as *mut cairo_glyph_t;                               // C 604
  i = 0;
  while i < len                                                  // C 605
    {
      (*glyphs.offset (i as isize)).index =
	*(*s).char2b.offset ((from + i) as isize) as c_ulong;     // C 607
      (*glyphs.offset (i as isize)).x = x as c_double;            // C 608
      (*glyphs.offset (i as isize)).y = y as c_double;            // C 609
      x += if glyph_string_bits! (padding_p, (*s)) { 1 }
	   else { ftcrfont_glyph_extents ((*s).font,
					  (*glyphs.offset (i as isize)).index as c_uint,
					  std::ptr::null_mut ()) };   // C 610-612
      i += 1;
    }
  /* C 614-626: `#ifndef USE_BE_CAIRO` / `#ifdef HAVE_X_WINDOWS` / `#else` / `#endif` / `#else`
     / `#endif` - the foreground colour, the same three arms.  */
  #[cfg (not (feature = "be_cairo"))]
  #[cfg (feature = "have_x_windows")]
  { x_set_cr_source_with_gc_foreground (f as *mut c_void, (*s).gc, false); }   // C 616
  #[cfg (not (feature = "be_cairo"))]
  #[cfg (not (feature = "have_x_windows"))]
  { pgtk_set_cr_source_with_color (f, (*s).xgcv.foreground, false); }   // C 618
  #[cfg (feature = "be_cairo")]
  {
    let col: u32 = be_foreground as u32;                          // C 621

    cairo_set_source_rgb (cr, RED_FROM_ULONG! (col) as c_double / 255.0,
			  GREEN_FROM_ULONG! (col) as c_double / 255.0,
			  BLUE_FROM_ULONG! (col) as c_double / 255.0);   // C 623-625
  }
  cairo_set_scaled_font (cr, (*ftcrfont_info).cr_scaled_font);    // C 627
  cairo_show_glyphs (cr, glyphs, len);                            // C 628
  /* C 629-638: the three `end` arms.  */
  #[cfg (not (feature = "be_cairo"))]
  #[cfg (feature = "have_x_windows")]
  { x_end_cr_clip (f as *mut c_void); }                           // C 631
  #[cfg (not (feature = "be_cairo"))]
  #[cfg (not (feature = "have_x_windows"))]
  { pgtk_end_cr_clip (f); }                                       // C 633
  #[cfg (feature = "be_cairo")]
  {
    haiku_end_cr_clip (cr);                                       // C 636
    EmacsWindow_end_cr_critical_section (FRAME_HAIKU_WINDOW! (f));   // C 637
  }
  SAFE_FREE! (sa_count, sa_mark);
  unblock_input ();                                               // C 639

  return len;                                                     // C 641
}

/* C 644-664: `#ifdef HAVE_PGTK` / `ftcrfont_cached_font_ok` / `#endif`.  */
/// C 648-663: `ftcrfont_cached_font_ok`.
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_pgtk"))]
unsafe extern "C" fn ftcrfont_cached_font_ok (f: *mut c_void, font_object: Lisp_Object,
					      entity: Lisp_Object) -> bool
{
  let info: *mut font_info = XFONT_OBJECT (font_object) as *mut font_info;   // C 652

  let options: *mut cairo_font_options_t = cairo_font_options_create ();   // C 654
  cairo_scaled_font_get_font_options ((*info).cr_scaled_font, options);   // C 655
  let gsettings_options: *mut cairo_font_options_t = xsettings_get_font_options ();   // C 656

  let equal: bool = cairo_font_options_equal (options, gsettings_options) != 0;   // C 658
  cairo_font_options_destroy (options);                           // C 659
  cairo_font_options_destroy (gsettings_options);                 // C 660

  return equal;                                                   // C 662
}

/* C 666-724: `#ifdef HAVE_HARFBUZZ` / the four `ftcrhb*` helpers / `#endif`.  */
/// C 668-672: `ftcrhbfont_list`.
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "harfbuzz"))]
unsafe extern "C" fn ftcrhbfont_list (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{
  return ftfont_list2 (f, spec, Qftcrhb ());                      // C 671
}

/// C 674-678: `ftcrhbfont_match`.
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "harfbuzz"))]
unsafe extern "C" fn ftcrhbfont_match (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{
  return ftfont_match2 (f, spec, Qftcrhb ());                     // C 677
}

/// C 680-701: `ftcrhbfont_begin_hb_font`.
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "harfbuzz"))]
unsafe extern "C" fn ftcrhbfont_begin_hb_font (font: *mut font, position_unit: *mut c_double)
					       -> hb_font_t
{
  let ftcrfont_info: *mut font_info = font as *mut font_info;      // C 683
  let scaled_font: *mut cairo_scaled_font_t = (*ftcrfont_info).cr_scaled_font;   // C 684
  let ft_face: FT_Face = cairo_ft_scaled_font_lock_face (scaled_font);   // C 685

  (*ftcrfont_info).ft_size = (*ft_face).size;                      // C 687
  let hb_font: hb_font_t = fthbfont_begin_hb_font (font, position_unit);   // C 688
  /* HarfBuzz 5 correctly scales bitmap-only fonts without position
     unit adjustment.
     (https://github.com/harfbuzz/harfbuzz/issues/489)

     Update: HarfBuzz 5.2.0 no longer does this for an hb_font_t font
     object created from a given FT_Face.
     (https://github.com/harfbuzz/harfbuzz/issues/3788)  (C 689-695)  */
  if (hb_version_atleast (5, 2, 0) != 0 || hb_version_atleast (5, 0, 0) == 0)   // C 696
      && (*ftcrfont_info).bitmap_position_unit != 0.0             // C 697
    { *position_unit = (*ftcrfont_info).bitmap_position_unit; }    // C 698

  return hb_font;                                                 // C 700
}

/// C 703-722: `ftcrhbfont_end_hb_font`.
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "harfbuzz"))]
unsafe extern "C" fn ftcrhbfont_end_hb_font (font: *mut font, hb_font: hb_font_t)
{
  let ftcrfont_info: *mut font_info = font as *mut font_info;      // C 706
  let scaled_font: *mut cairo_scaled_font_t = (*ftcrfont_info).cr_scaled_font;   // C 707

  eassert! (hb_font == (*ftcrfont_info).hb_font);                 // C 709
  /* ftcrfont_info->hb_font holds a reference to the FT_Face returned by
     cairo_ft_scaled_font_lock_face.  Keeping it around after the matching
     unlock call would violate the API contract, and cause corrupted
     display of composed characters (Bug#73752).  We destroy and NULLify
     hb_font here, which will then cause fthbfont_begin_hb_font, called by
     ftcrhbfont_begin_hb_font, to recreate hb_font anew, taking into
     consideration any scale changes in FT_Face.  (C 710-716)  */
  hb_font_destroy ((*ftcrfont_info).hb_font);                     // C 717
  (*ftcrfont_info).hb_font = std::ptr::null_mut ();               // C 718

  cairo_ft_scaled_font_unlock_face (scaled_font);                 // C 720
  (*ftcrfont_info).ft_size = std::ptr::null_mut ();               // C 721
}

/* C 727: `static void syms_of_ftcrfont_for_pdumper (void);` - C's forward declaration has no Rust
   counterpart (PORTING.md 0.1-12); the definition follows at C 810 and `syms_of_ftcrfont` reaches
   it from there.  */

/// C 729-758: `struct font_driver const ftcrfont_driver`.  C's designated initialiser names
/// sixteen members and leaves the rest zero; Rust has no designated initialisers, so the rest are
/// written out (the `xftfont.rs` 1818 shape).  `include/font.rs`'s `font_driver` types the
/// members C declares with a `struct frame *` parameter as untyped `*mut c_void` (its own note),
/// so the entries whose C parameter is `struct frame *` take the same shape C's table does.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[unsafe(no_mangle)]
pub static mut ftcrfont_driver: font_driver = font_driver {   // C 729-758
  /* C 731: `.type = LISPSYM_INITIALLY (Qftcr)`.  The mirror's `LISPSYM_INITIALLY` is not a
     `const fn`, so the port writes the same constant expression over the generated `iQftcr`
     index (the `xftfont.rs` 1822 precedent).  */
  r#type: (((iQftcr * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	    + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),
  case_sensitive: false,
  get_cache: Some (ftfont_get_cache),                       // C 732
  list: Some (ftcrfont_list),                               // C 733
  r#match: Some (ftcrfont_match),                           // C 734
  list_family: Some (ftfont_list_family),                   // C 735
  free_entity: None,
  open_font: Some (ftcrfont_open),                          // C 736
  close_font: Some (ftcrfont_close),                        // C 737
  prepare_face: None,
  done_face: None,
  has_char: Some (ftcrfont_has_char),                       // C 738
  encode_char: Some (ftcrfont_encode_char),                 // C 739
  text_extents: Some (ftcrfont_text_extents),               // C 740
  #[cfg (feature = "have_window_system")]
  draw: Some (ftcrfont_draw),                               // C 741
  #[cfg (feature = "have_window_system")]
  get_bitmap: Some (ftcrfont_get_bitmap),                   // C 742
  #[cfg (feature = "have_window_system")]
  free_bitmap: None,
  anchor_point: Some (ftcrfont_anchor_point),               // C 743
  #[cfg (not (feature = "have_libotf"))]
  otf_capability: None,
  #[cfg (feature = "have_libotf")]
  otf_capability: Some (ftcrfont_otf_capability),           // C 745
  otf_drive: None,
  start_for_frame: None,
  end_for_frame: None,
  #[cfg (not (all (feature = "have_m17n_flt", feature = "have_libotf")))]
  shape: None,
  #[cfg (all (feature = "have_m17n_flt", feature = "have_libotf"))]
  shape: Some (ftcrfont_shape),                             // C 748
  check: None,
  #[cfg (not (any (feature = "have_otf_get_variation_glyphs",
		   feature = "have_ft_face_getcharvariantindex")))]
  get_variation_glyphs: None,
  #[cfg (any (feature = "have_otf_get_variation_glyphs",
	      feature = "have_ft_face_getcharvariantindex"))]
  get_variation_glyphs: Some (ftcrfont_variation_glyphs),   // C 751
  filter_properties: Some (ftfont_filter_properties),       // C 753
  #[cfg (not (feature = "have_pgtk"))]
  cached_font_ok: None,
  #[cfg (feature = "have_pgtk")]
  cached_font_ok: Some (ftcrfont_cached_font_ok),           // C 756
  combining_capability: Some (ftfont_combining_capability), // C 754
  drop_xrender_surfaces: None,
  #[cfg (feature = "harfbuzz")]
  begin_hb_font: None,
  #[cfg (feature = "harfbuzz")]
  end_hb_font: None,
};

/* C 759-761: `#ifdef HAVE_HARFBUZZ` / `struct font_driver ftcrhbfont_driver;` / `#endif` - C's
   tentative definition of a second file-scope driver object, which `syms_of_ftcrfont_for_pdumper`
   (C 814-825) fills from `ftcrfont_driver`.  Rust has no tentative definition, so the object is
   written with C's zero value and its members are assigned where C assigns them
   (PORTING.md 0.1-12's object half).  */
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "harfbuzz"))]
#[unsafe(no_mangle)]
pub static mut ftcrhbfont_driver: font_driver = font_driver {   // C 760
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

/// C 763-772: `syms_of_ftcrfont`.
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_ftcrfont ()
{
  DEFSYM! (Qftcr, "ftcr");                                  // C 766
  /* C 767-770: `#ifdef HAVE_HARFBUZZ` and its two statements.  */
  #[cfg (feature = "harfbuzz")]
  DEFSYM! (Qftcrhb, "ftcrhb");                              // C 768
  #[cfg (feature = "harfbuzz")]
  Fput (Qftcr (), Qfont_driver_superseded_by (), Qftcrhb ());   // C 769
  pdumper_do_now_and_after_load (Some (syms_of_ftcrfont_for_pdumper));   // C 771
}

/* C 774-808: `#ifdef HAVE_X_WINDOWS` / `ftcrfont_get_default_font_options` / `#endif`.  */
/// C 779-806: `ftcrfont_get_default_font_options`.  `XCreatePixmap` is declared here with its
/// `Xlib.h` line because `include/xlib.rs` carries only the `XFreePixmap` half (the `font.h`
/// rule: a name the mirrors do not carry is declared at the use site, P-55).
#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftcrfont_get_default_font_options (dpyinfo: *mut x_display_info,
							    options: *mut cairo_font_options_t)
{
  let mut drawable: Pixmap;                                       // C 783
  let mut surface: *mut cairo_surface_t;                          // C 784

  /* Cairo doesn't allow fetching the default font options for a
     display, so the only option is to create a drawable, and an Xlib
     surface for that drawable, and to get the font options from there
     instead.  (C 786-789)  */

  drawable = XCreatePixmap ((*dpyinfo).display, (*dpyinfo).root_window, 1, 1,
			    (*dpyinfo).n_planes as c_uint);        // C 791-792
  surface = cairo_xlib_surface_create ((*dpyinfo).display, drawable, (*dpyinfo).visual, 1, 1);   // C 793-794

  if surface.is_null ()                                           // C 796
    {
      XFreePixmap ((*dpyinfo).display, drawable);                 // C 798
      return;                                                     // C 799
    }

  cairo_surface_get_font_options (surface, options);              // C 802
  XFreePixmap ((*dpyinfo).display, drawable);                     // C 803
  cairo_surface_destroy (surface);                                // C 804
  return;                                                         // C 805
}

/// C 810-826: `syms_of_ftcrfont_for_pdumper`.  C 815's `ftcrhbfont_driver = ftcrfont_driver;` is
/// a whole-record assignment of a `Copy` record, and the eight member writes that follow it are
/// C's own (C 816-823).
#[cfg (any (feature = "use_cairo", feature = "be_cairo"))]
unsafe extern "C" fn syms_of_ftcrfont_for_pdumper ()
{
  register_font_driver (std::ptr::addr_of! (ftcrfont_driver), std::ptr::null_mut ());   // C 813
  /* C 814-825: `#ifdef HAVE_HARFBUZZ` and its block.  */
  #[cfg (feature = "harfbuzz")]
  {
    ftcrhbfont_driver = std::ptr::read (std::ptr::addr_of! (ftcrfont_driver));   // C 815
    ftcrhbfont_driver.r#type = Qftcrhb ();                    // C 816
    ftcrhbfont_driver.list = Some (ftcrhbfont_list);          // C 817
    ftcrhbfont_driver.r#match = Some (ftcrhbfont_match);      // C 818
    ftcrhbfont_driver.otf_capability = Some (hbfont_otf_capability);   // C 819
    ftcrhbfont_driver.shape = Some (hbfont_shape);            // C 820
    ftcrhbfont_driver.combining_capability = Some (hbfont_combining_capability);   // C 821
    ftcrhbfont_driver.begin_hb_font = Some (ftcrhbfont_begin_hb_font);   // C 822
    ftcrhbfont_driver.end_hb_font = Some (ftcrhbfont_end_hb_font);   // C 823
    register_font_driver (std::ptr::addr_of! (ftcrhbfont_driver), std::ptr::null_mut ());   // C 824
  }
}
