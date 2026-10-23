// femacs: port of src/w32font.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header belongs to the Windows font backends (Uniscribe,
// HarfBuzz and DirectWrite), and nothing on this target compiles it: the platform object
// variables are empty, `src/w32font.c` is not in the frozen build's source list, and the types
// it names beyond `font.h` (`TEXTMETRICW`, `HFONT`, `LOGFONT`, `HDC`, `COLORREF`) come from
// `<windows.h>`, which has no mirror.  The mirror therefore splits in two, the way
// `include/macfont.rs` splits for macOS:
//
//   * the frozen half - the numeric macros and `struct w32_metric_cache`, whose fields are C
//     scalars only, so the frozen clang *can* measure them (assertions below);
//   * the Windows half under `#[cfg (feature = "windowsnt")]` (P-51's arm for the non-frozen
//     side): `struct w32font_info`, `struct uniscribe_font_info` and every prototype that names
//     a `<windows.h>` type, spelled with C's own names because that target supplies them.
//
// `struct glyph_string *` inside `w32font_draw`'s prototype stays a `*mut c_void` stand-in even
// under that arm, because the record is dispextern.h's and this mirror does not include it (the
// `include/sfntfont.rs` reading, status 3.170/3.171).

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const EMACS_W32FONT_H: bool = true;

include!("font.rs");                 // C 23: `#include "font.h"` (`struct font`, `struct font_metrics`)

/// C 26-28: bit 17 of `NEWTEXTMETRIC.ntmFlags` is set for PostScript OpenType fonts.
/// C: #ifndef NTM_PS_OPENTYPE / #define NTM_PS_OPENTYPE 0x00020000 / #endif
pub const NTM_PS_OPENTYPE: c_uint = 0x00020000;

/// C 29-31: bit 18, TrueType OpenType fonts.
/// C: #define NTM_TT_OPENTYPE 0x00040000
pub const NTM_TT_OPENTYPE: c_uint = 0x00040000;

/// C 32-34: bit 20, Type1 fonts.
/// C: #define NTM_TYPE1 0x00100000
pub const NTM_TYPE1: c_uint = 0x00100000;

/// C 36: the two OpenType bits together.  C's body is the `|` of the two names.
/// C: #define NTMFLAGS_OPENTYPE (NTM_PS_OPENTYPE | NTM_TT_OPENTYPE)
pub const NTMFLAGS_OPENTYPE: c_uint = NTM_PS_OPENTYPE | NTM_TT_OPENTYPE;

/// C 40-45: the per-glyph metric cache record.  Every field is a C scalar, so the frozen clang
/// measured it: `sizeof` 12, `alignof` 2, and the eight offsets asserted below.
/// C: struct w32_metric_cache { short lbearing, rbearing, width, ascent, descent;
/// C:                           unsigned char status; };
#[repr(C)]
pub struct w32_metric_cache
{
  pub lbearing: c_short,
  pub rbearing: c_short,
  pub width: c_short,
  pub ascent: c_short,
  pub descent: c_short,
  pub status: c_uchar,
}

const _: () = core::assert! (std::mem::size_of::<w32_metric_cache> () == 12);
const _: () = core::assert! (std::mem::align_of::<w32_metric_cache> () == 2);
const _: () = core::assert! (std::mem::offset_of! (w32_metric_cache, lbearing) == 0);
const _: () = core::assert! (std::mem::offset_of! (w32_metric_cache, rbearing) == 2);
const _: () = core::assert! (std::mem::offset_of! (w32_metric_cache, width) == 4);
const _: () = core::assert! (std::mem::offset_of! (w32_metric_cache, ascent) == 6);
const _: () = core::assert! (std::mem::offset_of! (w32_metric_cache, descent) == 8);
const _: () = core::assert! (std::mem::offset_of! (w32_metric_cache, status) == 10);

/// C 44-46: the three cache states the backend assigns to `status`.
/// C: #define W32METRIC_NO_ATTEMPT 0 / #define W32METRIC_SUCCESS 1 / #define W32METRIC_FAIL 2
pub const W32METRIC_NO_ATTEMPT: c_uchar = 0;
pub const W32METRIC_SUCCESS: c_uchar = 1;
pub const W32METRIC_FAIL: c_uchar = 2;

/// C 84: how many metrics one cache block holds.
/// C: #define CACHE_BLOCKSIZE 128
pub const CACHE_BLOCKSIZE: c_int = 128;

/// C 81-82: the two accessors that reach a font's OS-specific fields.  C casts to
/// `struct w32font_info *` and reads a member; the port's macro does the same, and a body is not
/// resolved until it is expanded.
/// C: #define FONT_HANDLE(f) (((struct w32font_info *)(f))->hfont)
/// C: #define FONT_TEXTMETRIC(f) (((struct w32font_info *)(f))->metrics)
macro_rules! FONT_HANDLE {
  ($f:expr) => { (*(($f) as *mut w32font_info)).hfont };
}
macro_rules! FONT_TEXTMETRIC {
  ($f:expr) => { (*(($f) as *mut w32font_info)).metrics };
}

/// C 52-60: `struct w32font_info` - the Windows half, spelled with C's names under the arm that
/// has `<windows.h>`.  `struct font` by value is `font.h`'s record (this mirror includes it) and
/// `struct w32_metric_cache **` is this mirror's own.
/// C: struct w32font_info { struct font font; TEXTMETRICW metrics; unsigned int glyph_idx;
/// C:                       struct w32_metric_cache **cached_metrics; int n_cache_blocks;
/// C:                       HFONT hfont; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct w32font_info
{
  pub font: font,
  pub metrics: TEXTMETRICW,
  pub glyph_idx: c_uint,
  pub cached_metrics: *mut *mut w32_metric_cache,
  pub n_cache_blocks: c_int,
  pub hfont: HFONT,
}

/// C 61-79: `struct uniscribe_font_info` - the Uniscribe/HarfBuzz/DirectWrite extension of the
/// record above (C's comment says who uses `cache` for what).  `void *` members are C's own
/// untyped pointers, so they map to `*mut c_void`.
/// C: struct uniscribe_font_info { struct w32font_info w32_font; void *cache; double scale;
/// C:                              void *dwrite_cache; float dwrite_font_size;
/// C:                              bool dwrite_skip_font; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct uniscribe_font_info
{
  pub w32_font: w32font_info,
  pub cache: *mut c_void,
  pub scale: c_double,
  pub dwrite_cache: *mut c_void,
  pub dwrite_font_size: c_float,
  pub dwrite_skip_font: bool,
}

/// C 86-120: the entry points (`src/w32font.c` and `src/w32uniscribe.c` define them; C 117-120 are
/// the DirectWrite set `src/w32dwrite.c` defines).  Under the Windows arm they keep C's spellings;
/// `struct frame`/`struct font`/`struct font_metrics` are named because owner mirrors exist, and
/// `struct glyph_string *` keeps the P-55 stand-in (the file note above).
/// C: Lisp_Object w32font_get_cache (struct frame *fe);
/// C: Lisp_Object w32font_list_internal (struct frame *f, Lisp_Object font_spec, bool opentype_only);
/// C: Lisp_Object w32font_match_internal (struct frame *f, Lisp_Object font_spec, bool opentype_only);
/// C: int w32font_open_internal (struct frame *f, Lisp_Object font_entity, int pixel_size,
/// C:                             Lisp_Object font_object);
/// C: void w32font_close (struct font *font);
/// C: int w32font_has_char (Lisp_Object entity, int c);
/// C: void w32font_text_extents (struct font *font, const unsigned *code, int nglyphs,
/// C:                             struct font_metrics *metrics);
/// C: int w32font_draw (struct glyph_string *s, int from, int to, int x, int y, bool with_background);
/// C: int uniscribe_check_otf (LOGFONT *font, Lisp_Object otf_spec);
/// C: Lisp_Object intern_font_name (char *);
/// C: void w32_initialize_direct_write (void);
/// C: bool w32_use_direct_write (struct w32font_info *w32font);
/// C: bool w32_dwrite_draw (HDC hdc, int x, int y, unsigned *glyphs, int len, COLORREF color,
/// C:                       struct font *font);
/// C: bool w32_dwrite_text_extents (struct font *font, const unsigned *code, int nglyphs,
/// C:                                struct font_metrics *metrics);
/// C: unsigned w32_dwrite_encode_char (struct font *font, int c);
/// C: void w32_dwrite_free_cached_face (void *cache);
/// C: void syms_of_w32dwrite (void);
/// C: extern void globals_of_w32font (void);
/* C 71-96's non-static entry points, in C's order.  They sit in a module the mirror
   glob-imports because `src/w32font.rs` **defines** nine of them (`w32font_get_cache`,
   `w32font_list_internal`, `w32font_match_internal`, `w32font_open_internal`, `w32font_close`,
   `w32font_has_char`, `w32font_text_extents`, `w32font_draw`, `intern_font_name`): at the crate
   root those declarations are `E0428` in that unit the moment it includes its own header mirror -
   measured by the lane's arm reading, ten rows at this mirror's 151-176 (status 3.539).  The
   module + glob shape is P-39's (`fontset_decls`, `font_log_decls`, `ccl_spec`); the unit's own
   definitions shadow the glob, so nothing changes for the units that only *call* them.  */
#[cfg (feature = "windowsnt")]
pub mod w32font_decls {
  use super::*;

  unsafe extern "C" {
  pub fn w32font_get_cache (fe: *mut frame) -> Lisp_Object;
  pub fn w32font_list_internal (f: *mut frame, font_spec: Lisp_Object, opentype_only: bool)
                                -> Lisp_Object;
  pub fn w32font_match_internal (f: *mut frame, font_spec: Lisp_Object, opentype_only: bool)
                                 -> Lisp_Object;
  pub fn w32font_open_internal (f: *mut frame, font_entity: Lisp_Object, pixel_size: c_int,
                                font_object: Lisp_Object) -> c_int;
  pub fn w32font_close (font: *mut font);
  pub fn w32font_has_char (entity: Lisp_Object, c: c_int) -> c_int;
  pub fn w32font_text_extents (font: *mut font, code: *const c_uint, nglyphs: c_int,
                               metrics: *mut font_metrics);
  /// `struct glyph_string *` is the stand-in of the file note (P-55).
  pub fn w32font_draw (s: *mut c_void, from: c_int, to: c_int, x: c_int, y: c_int,
                       with_background: bool) -> c_int;
  pub fn uniscribe_check_otf (font: *mut LOGFONT, otf_spec: Lisp_Object) -> c_int;
  pub fn intern_font_name (name: *mut c_char) -> Lisp_Object;
  pub fn w32_initialize_direct_write ();
  pub fn w32_use_direct_write (w32font: *mut w32font_info) -> bool;
  pub fn w32_dwrite_draw (hdc: HDC, x: c_int, y: c_int, glyphs: *mut c_uint, len: c_int,
                          color: COLORREF, font: *mut font) -> bool;
  pub fn w32_dwrite_text_extents (font: *mut font, code: *const c_uint, nglyphs: c_int,
                                  metrics: *mut font_metrics) -> bool;
  pub fn w32_dwrite_encode_char (font: *mut font, c: c_int) -> c_uint;
  pub fn w32_dwrite_free_cached_face (cache: *mut c_void);
  pub fn syms_of_w32dwrite ();
  pub fn globals_of_w32font ();
  }
}
#[cfg (feature = "windowsnt")]
use w32font_decls::*;
