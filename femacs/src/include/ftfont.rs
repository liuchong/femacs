// femacs: port of src/ftfont.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is the FreeType backend's - it is what the XFT,
// FreeType and Cairo builds share - and nothing on this target compiles it: the frozen
// configuration has no window system (`HAVE_X_WINDOWS` undefined), `src/ftfont.c` is not in the
// frozen build's source list, and every type it names beyond `font.h` comes from FreeType, Xft or
// Fontconfig, none of which has a mirror here.
//
// The mirror therefore keeps C's own names under `#[cfg (feature = "have_x_windows")]` (P-51's arm
// for the non-frozen side; the feature name is the C macro's literal spelling, status 3.171's
// rule): `struct font_info`'s FreeType members (`FT_Size`, `FT_Matrix`) and its XFT arm
// (`Display`, `XftFont`) are spelled as C spells them, because that target supplies them.  C's
// *other* arms are dead text here and are recorded rather than mirrored - the `HAVE_LIBOTF` pair
// (`maybe_otf`/`otf`) and `hb_font` are the exception: they are carried as gated members since
// status 3.531, because `src/xftfont.rs` reads them under exactly those gates.  The remaining
// (`maybe_otf`, `otf`), the `HAVE_HARFBUZZ` member (`hb_font`) and the
// `USE_CAIRO || USE_BE_CAIRO` block (`cr_scaled_font`, `bitmap_position_unit`, `metrics`,
// `metrics_nrows`) - with their guards named so a cairo/HarfBuzz target knows which `#if` to
// re-read.  The C 26-43 include chain (ft2build/FreeType/cairo/hb/otf/m17n-flt) is recorded the
// same way: those headers' names belong to the platform target, not to a Rust mirror.

// C 23-24: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const EMACS_FTFONT_H: bool = true;

/// C 26-43: the platform include chain.  `FT_FREETYPE_H`/`FT_SIZES_H`/`FT_TRUETYPE_TABLES_H`
/// (and `FT_BDF_H` under `#ifdef FT_BDF_H`) are FreeType's; `cairo.h` is under `USE_BE_CAIRO`,
/// `hb.h`/`hb-ft.h` under `HAVE_HARFBUZZ`, `otf.h` (with `m17n-flt.h` under `HAVE_M17N_FLT`)
/// under `HAVE_LIBOTF`.  None is carried as a Rust include because this tree has no mirror for
/// any of them; the names arrive with the platform target (PORTING.md 0.1-9).

/// C 45-54: the three Fontconfig entry points `src/ftfont.c` defines.
/// C: extern void ftfont_fix_match (FcPattern *, FcPattern *);
/// C: extern void ftfont_add_rendering_parameters (FcPattern *, Lisp_Object);
/// C: extern FcPattern *ftfont_entity_pattern (Lisp_Object, int);
/// C 54's `FcPattern *` return is Fontconfig's record, so it keeps the P-55 stand-in.
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn ftfont_fix_match (pattern: *mut c_void, current: *mut c_void);
  pub fn ftfont_add_rendering_parameters (pattern: *mut c_void, prop: Lisp_Object);
  pub fn ftfont_entity_pattern (prop: Lisp_Object, pixel_size: c_int) -> *mut c_void;
}

/// C 56-87: `struct font_info` - the record the XFT, FreeType and Cairo backends share.  Members
/// up to and including `matrix` are common (C's own comment); the rest depend on the backend, and
/// on this target the live arm is the `#else` one (XFT), because neither `USE_CAIRO` nor
/// `USE_BE_CAIRO` is defined.  `struct font` by value is `font.h`'s record; the FreeType and XFT
/// names are spelled as C spells them.
/// C: struct font_info { struct font font; FT_Size ft_size; int index; FT_Matrix matrix;
/// C:                     Display *display; XftFont *xftfont; unsigned x_display_id; };
/// C 59-62: `#ifdef HAVE_LIBOTF / bool maybe_otf; OTF *otf; / #endif` and C 65-67:
/// `#ifdef HAVE_HARFBUZZ / hb_font_t *hb_font; / #endif` - carried as *gated members* below
/// (status 3.531).  They used to be recorded as dead text, and the `xftfont` arm paid for that:
/// nine `E0609 no field` rows on the unit's own lines (`(*xftfont_info).maybe_otf` C 309, `.otf`
/// C 310/348/350/351, `.hb_font` C 313/355/357/358) although `have_libotf`/`harfbuzz` are what
/// the unit's own gates name.  `OTF` (`<otf.h>`) and `hb_font_t` (`<hb.h>`) have no mirror here,
/// so both pointers are the untyped one (P-72 rule 3) - which is also what the unit's own
/// `OTF`/`hb_font_t` aliases are (`*mut c_void`, measured against HarfBuzz 14.4.0).
/// C (dead here): #if defined (USE_CAIRO) || defined (USE_BE_CAIRO) / cairo_scaled_font_t
/// C: *cr_scaled_font; double bitmap_position_unit; struct font_metrics **metrics;
/// C: short metrics_nrows; / #else … / #endif
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct font_info
{
  pub font: font,
  #[cfg (feature = "have_libotf")]
  pub maybe_otf: bool,                       // C 60: `bool maybe_otf;`
  #[cfg (feature = "have_libotf")]
  pub otf: *mut c_void,                      /// C 61: `OTF *otf;` (P-72 (3))
  pub ft_size: FT_Size,
  pub index: c_int,
  pub matrix: FT_Matrix,
  #[cfg (feature = "harfbuzz")]
  pub hb_font: *mut c_void,                  /// C 66: `hb_font_t *hb_font;` (P-72 (3))
  pub display: *mut Display,
  pub xftfont: *mut XftFont,
  pub x_display_id: c_uint,
}

// C 89-92: the Cairo arm's one entry point (`USE_CAIRO && HAVE_X_WINDOWS` - dead on this
// target, recorded with its guard named).  A plain comment, not a doc comment: there is no Rust
// item for it, and a `///` block with nothing under it does not compile (measured: the layout
// probe's crate reported `expected item after doc comment` at this site, status 3.181).
// C: extern void ftcrfont_get_default_font_options (struct x_display_info *,
// C:                                               cairo_font_options_t *);
