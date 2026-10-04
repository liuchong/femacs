// femacs: port of src/font.h @ fd7afced403.
//
// WIP mirror (P-66): data.c's `cl-type-of` reaches font.h through
// keyboard.h -> coding.h -> composite.h -> font.h (keyboard.h 23, coding.h 284,
// composite.h 28), and needs the three font predicates below.  `struct font` (font.h
// 275-418) landed with haikuterm.h's `struct haikufont_info`, whose first member is that
// record *by value*; the five font units that kept use-site copies of it read this
// declaration now, and docs/ports/include-font.md records the change.  The rest of the
// header - the driver interface, the property accessors, the five other records - lands
// with font.c and the terminal backends, which is when that ledger's remaining boxes are
// ticked.

// font.h 24: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_FONT_H: bool = true;

/* font.h 70-190: `enum font_property_index`.  The members are `pub const`s rather than a
   Rust enum because two of them are written as other members (`FONT_OBJLIST_INDEX =
   FONT_SPEC_MAX`, `FONT_NAME_INDEX = FONT_ENTITY_MAX`), which a Rust enum discriminant
   cannot name; the C type itself is an int, so the alias below is what a prototype's
   parameter uses.  The `#if false` block (font.h 128-163: FONT_STYLE_INDEX,
   FONT_METRICS_INDEX) is dead in every configuration.  */
pub type font_property_index = c_int;

/* FONT-TYPE is a symbol indicating a font backend; currently `x',
   `xft', `xfthb', `ftrc', and `ftcrhb' are available on X; `harfbuzz',
   `uniscribe', and `gdi' on Windows, and `ns' under Cocoa / GNUstep.  */
pub const FONT_TYPE_INDEX: font_property_index = 0;
/* FONT-FOUNDRY is a foundry name (symbol).  */
pub const FONT_FOUNDRY_INDEX: font_property_index = 1;
/* FONT-FAMILY is a family name (symbol).  */
pub const FONT_FAMILY_INDEX: font_property_index = 2;
/* FONT-ADSTYLE is an additional style name (symbol).  */
pub const FONT_ADSTYLE_INDEX: font_property_index = 3;
/* FONT-REGISTRY is a combination of a charset-registry and charset-encoding name (symbol).  */
pub const FONT_REGISTRY_INDEX: font_property_index = 4;
/* FONT-WEIGHT is a numeric value of weight (e.g. medium, bold) of the font.  */
pub const FONT_WEIGHT_INDEX: font_property_index = 5;
/* FONT-SLANT is a numeric value of slant (e.g. r, i, o) of the font.  */
pub const FONT_SLANT_INDEX: font_property_index = 6;
/* FONT-WIDTH is a numeric value of setwidth (e.g. normal) of the font.  */
pub const FONT_WIDTH_INDEX: font_property_index = 7;
/* FONT-SIZE is a size of the font; zero means the font is scalable.  */
pub const FONT_SIZE_INDEX: font_property_index = 8;
/* FONT-DPI is a resolution (dot per inch) for which the font is designed.  */
pub const FONT_DPI_INDEX: font_property_index = 9;
/* FONT-SPACING is a spacing (mono, proportional, charcell) of the font.  */
pub const FONT_SPACING_INDEX: font_property_index = 10;
/* FONT-AVGWIDTH is an average width (1/10 pixel unit) of the font.  */
pub const FONT_AVGWIDTH_INDEX: font_property_index = 11;
/* In a font-spec, the value is an alist of extra information; in a font-entity this holds
   font driver-specific information.  */
pub const FONT_EXTRA_INDEX: font_property_index = 12;
/* This value is the length of font-spec vector.  */
pub const FONT_SPEC_MAX: font_property_index = 13;
/* List of font-objects opened from the font-entity.  */
pub const FONT_OBJLIST_INDEX: font_property_index = FONT_SPEC_MAX;
/* This value is the length of font-entity vector.  */
pub const FONT_ENTITY_MAX: font_property_index = 14;
/* XLFD name of the font (string). */
pub const FONT_NAME_INDEX: font_property_index = FONT_ENTITY_MAX;
/* Full name of the font (string), possibly different from the above.  */
pub const FONT_FULLNAME_INDEX: font_property_index = 15;
/* File name of the font or nil if a file associated with the font is not available.  */
pub const FONT_FILE_INDEX: font_property_index = 16;
/* This value is the length of font-object vector.  */
pub const FONT_OBJECT_MAX: font_property_index = 17;

/* font.h 275-418: `struct font`, the font-object record.  It is this mirror's first record;
   `haikuterm.h`'s `struct haikufont_info` carries it **by value** as its first member
   (haikuterm.h 42-44), which is the dependency this round unblocks, and the five font units
   that kept use-site copies of it now read this declaration (docs/ports/include-font.md).

   `HAVE_WINDOW_SYSTEM` is undef in the frozen configuration, so the eight members C guards
   with it (font.h 325-403) are absent from this layout and are carried under the feature
   that names the condition's other side (PORTING.md P-51).  Measured with the frozen clang,
   `clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim -I build/emacs-ref/src
   -I emacs/src -I emacs/lib`: `sizeof` 184, `alignof` 8, `props`@8, `min_width`@144,
   `max_width`@148, `pixel_size`@152, `height`@156, `space_width`@160, `average_width`@164,
   `ascent`@168, `descent`@172, `driver`@176.  */
#[repr(C)]
pub struct font
{
  pub header: vectorlike_header,

  /* All Lisp_Object components must come first.  That ensures they are all aligned
     normally; beyond them there should be no more Lisp_Object components.  */
  pub props: [Lisp_Object; FONT_OBJECT_MAX as usize],

  /* Minimum and maximum glyph widths, in pixels.  */
  pub min_width: c_int,
  pub max_width: c_int,

  /* By which pixel size the font is opened.  */
  pub pixel_size: c_int,

  /* Height of the font.  On X, this is `ascent` + `descent`.  */
  pub height: c_int,

  /* Width of the space glyph of the font; 0 if the font has no SPACE glyph.  */
  pub space_width: c_int,

  /* Average width of the glyphs of ASCII characters; 0 if the font has no glyph for any.  */
  pub average_width: c_int,

  /* Ascent and descent of the font (in pixels).  */
  pub ascent: c_int,
  pub descent: c_int,

  /* font.h 325-403's `#if defined (HAVE_WINDOW_SYSTEM)` block: underline_thickness,
     underline_position, vertical_centering, baseline_offset, relative_compose,
     default_ascent, encoding_charset, repertory_charset.  */
  #[cfg(feature = "have_window_system")]
  pub underline_thickness: c_int,
  #[cfg(feature = "have_window_system")]
  pub underline_position: c_int,
  #[cfg(feature = "have_window_system")]
  pub vertical_centering: bool,
  #[cfg(feature = "have_window_system")]
  pub baseline_offset: c_int,
  #[cfg(feature = "have_window_system")]
  pub relative_compose: c_int,
  #[cfg(feature = "have_window_system")]
  pub default_ascent: c_int,
  #[cfg(feature = "have_window_system")]
  pub encoding_charset: c_int,
  #[cfg(feature = "have_window_system")]
  pub repertory_charset: c_int,

  /// C: `struct font_driver const *driver;` (font.h 400) - `struct font_driver` is this
  /// header's record too and has not landed in this mirror yet, so the pointer keeps the
  /// untyped form with this line (PORTING.md P-55/P-72 rule 3, and the pointee's `const` is
  /// C's), and the units that read it cast at the use site, the `frame.rs`
  /// `face_cache`/`font_driver_list` precedent.
  pub driver: *const c_void,
}

const _: () = assert! (std::mem::size_of::<font> () == 184);
const _: () = assert! (std::mem::align_of::<font> () == 8);
const _: () = assert! (std::mem::offset_of! (font, header) == 0);
const _: () = assert! (std::mem::offset_of! (font, props) == 8);
const _: () = assert! (std::mem::offset_of! (font, min_width) == 144);
const _: () = assert! (std::mem::offset_of! (font, max_width) == 148);
const _: () = assert! (std::mem::offset_of! (font, pixel_size) == 152);
const _: () = assert! (std::mem::offset_of! (font, height) == 156);
const _: () = assert! (std::mem::offset_of! (font, space_width) == 160);
const _: () = assert! (std::mem::offset_of! (font, average_width) == 164);
const _: () = assert! (std::mem::offset_of! (font, ascent) == 168);
const _: () = assert! (std::mem::offset_of! (font, descent) == 172);
const _: () = assert! (std::mem::offset_of! (font, driver) == 176);

/* font.h 434-472: the predicates.  */
#[inline(always)]
pub fn FONTP (x: Lisp_Object) -> bool { PSEUDOVECTORP (x, pvec_type::PVEC_FONT) }

/* True iff X is font-spec.  */
#[inline(always)]
pub fn FONT_SPEC_P (x: Lisp_Object) -> bool { FONTP (x) && PVSIZE (x) == FONT_SPEC_MAX as isize }

/* True iff X is font-entity.  */
#[inline(always)]
pub fn FONT_ENTITY_P (x: Lisp_Object) -> bool
{
  FONTP (x) && PVSIZE (x) == FONT_ENTITY_MAX as isize
}

/* True iff X is font-object.  */
#[inline(always)]
pub fn FONT_OBJECT_P (x: Lisp_Object) -> bool
{
  FONTP (x) && PVSIZE (x) == FONT_OBJECT_MAX as isize
}

/* ---------------------------------------------------------------------------------------
   font.h 236-585 and 1004-1010: the style/numeric/log macros and the two "not decided"
   constants.  Every body below is C's frozen text (arguments as `$`-metavariables) - the
   policy this port uses for macros whose C spelling is not a Rust expression Rust could
   evaluate: they are compared by the token gate and nothing in the port expands them.
   The live arms are the verified reading's (`python3 tools/check-cond-eval.py src/font.h`
   -> `28 conditions, 0 unknown, 0 disagreements [VERIFIED]`, line 279 of
   docs/baseline/config/cond-eval-verified.txt), and the arms the frozen target does *not*
   compile are recorded in docs/ports/include-font.md, not carried - this header's
   conditions are decided, so the gate does not ask for them.
   --------------------------------------------------------------------------------------- */

/* C 193-201: `FONT_WEIGHT_NUMERIC`/`FONT_WIDTH_NUMERIC`/`FONT_SLANT_NUMERIC`.  */
macro_rules! FONT_WEIGHT_NUMERIC {
  ($font:expr) => { (FIXNUMP (AREF ($font, FONT_WEIGHT_INDEX)) ? (XFIXNUM (AREF ($font, FONT_WEIGHT_INDEX)) >> 8) : -1) };
}
macro_rules! FONT_WIDTH_NUMERIC {
  ($font:expr) => { (FIXNUMP (AREF ($font, FONT_WIDTH_INDEX)) ? (XFIXNUM (AREF ($font, FONT_WIDTH_INDEX)) >> 8) : -1) };
}
macro_rules! FONT_SLANT_NUMERIC {
  ($font:expr) => { (FIXNUMP (AREF ($font, FONT_SLANT_INDEX)) ? (XFIXNUM (AREF ($font, FONT_SLANT_INDEX)) >> 8) : -1) };
}

/* C 205-217: the `*_SYMBOLIC` and `*_FOR_FACE` queries.  */
macro_rules! FONT_WEIGHT_SYMBOLIC { ($font:expr) => { font_style_symbolic ($font, FONT_WEIGHT_INDEX, false) }; }
macro_rules! FONT_SLANT_SYMBOLIC { ($font:expr) => { font_style_symbolic ($font, FONT_SLANT_INDEX, false) }; }
macro_rules! FONT_WIDTH_SYMBOLIC { ($font:expr) => { font_style_symbolic ($font, FONT_WIDTH_INDEX, false) }; }
macro_rules! FONT_WEIGHT_FOR_FACE { ($font:expr) => { font_style_symbolic ($font, FONT_WEIGHT_INDEX, true) }; }
macro_rules! FONT_SLANT_FOR_FACE { ($font:expr) => { font_style_symbolic ($font, FONT_SLANT_INDEX, true) }; }
macro_rules! FONT_WIDTH_FOR_FACE { ($font:expr) => { font_style_symbolic ($font, FONT_WIDTH_INDEX, true) }; }

/* C 224-230: the `*_NAME_NUMERIC` lifts.  */
macro_rules! FONT_WEIGHT_NAME_NUMERIC {
  ($name:expr) => { (font_style_to_value (FONT_WEIGHT_INDEX, $name, false) >> 8) };
}
macro_rules! FONT_SLANT_NAME_NUMERIC {
  ($name:expr) => { (font_style_to_value (FONT_SLANT_INDEX, $name, false) >> 8) };
}
macro_rules! FONT_WIDTH_NAME_NUMERIC {
  ($name:expr) => { (font_style_to_value (FONT_WIDTH_INDEX, $name, false) >> 8) };
}

/* C 236: `FONT_SET_STYLE (font, prop, val)`.  */
macro_rules! FONT_SET_STYLE {
  ($font:expr, $prop:expr, $val:expr) => { ASET ($font, $prop, make_fixnum (font_style_to_value ($prop, $val, true))) };
}

/* C 244-246: the metrics accessors.  */
macro_rules! FONT_HEIGHT { ($f:expr) => { (($f)->height) }; }
macro_rules! FONT_BASE { ($f:expr) => { (($f)->ascent) }; }
macro_rules! FONT_DESCENT { ($f:expr) => { (($f)->descent) }; }
macro_rules! FONT_WIDTH { ($f:expr) => { (($f)->max_width) }; }

/* C 273: the "encoding not decided yet" marker, and C 583-585's two code limits.  */
pub const FONT_ENCODING_NOT_DECIDED: c_int = 255;
pub const FONT_PIXEL_SIZE_QUANTUM: c_int = 1;
pub const FONT_INVALID_CODE: c_uint = 0xFFFFFFFF;

/* C 297-302: `enum font_spacing` - its four members, spelled as constants of C's type.  */
pub const FONT_SPACING_PROPORTIONAL: c_int = 0;
pub const FONT_SPACING_DUAL: c_int = 90;
pub const FONT_SPACING_MONO: c_int = 100;
pub const FONT_SPACING_CHARCELL: c_int = 110;

/* C 552: `XSETFONT (a, b)`.  */
macro_rules! XSETFONT { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR ($a, $b, PVEC_FONT) }; }

/* C 563: `PT_PER_INCH` - the TeXbook value, which is the live arm (`HAVE_ANDROID` is undef);
   C 566-570's 160.00 arm is dead and recorded in the ledger.  */
pub const PT_PER_INCH: f64 = 72.27;

/* C 572, 579: the point/pixel conversions.  */
macro_rules! POINT_TO_PIXEL { ($point:expr, $dpi:expr) => { (($point) * ($dpi) / PT_PER_INCH + 0.5) }; }
macro_rules! PIXEL_TO_POINT { ($pixel:expr, $dpi:expr) => { (($pixel) * PT_PER_INCH / ($dpi) + 0.5) }; }

/* C 1004-1011: the two logging macros.  */
macro_rules! FONT_ADD_LOG {
  ($action:expr, $arg:expr, $result:expr) => {
    do { if (! EQ (Vfont_log, Qt)) font_add_log ($action, $arg, $result); } while (false)
  };
}
macro_rules! FONT_DEFERRED_LOG {
  ($action:expr, $arg:expr, $result:expr) => {
    do { if (! EQ (Vfont_log, Qt)) font_deferred_log ($action, $arg, $result); } while (false)
  };
}

/* font.h 249-256: `struct font_spec` - C 251-255's `union vectorlike_header header;
   Lisp_Object props[FONT_SPEC_MAX];`.  Measured (Apple clang 21.0.0, `clang -std=gnu23 -O0
   -I build/emacs-ref/lib -idirafter spikes/shim -I build/emacs-ref/src -I emacs/src
   -I emacs/lib`, probe recorded in docs/ports/include-font.md): sizeof 112, align 8,
   header@0, props@8 - which is the same shape `struct font` above uses.  */
#[repr(C)]
pub struct font_spec
{
  pub header: vectorlike_header,
  pub props: [Lisp_Object; FONT_SPEC_MAX as usize],
}
const _: () = assert! (std::mem::size_of::<font_spec> () == 112);
const _: () = assert! (std::mem::align_of::<font_spec> () == 8);
const _: () = assert! (std::mem::offset_of! (font_spec, header) == 0);
const _: () = assert! (std::mem::offset_of! (font_spec, props) == 8);

/* font.h 258-269: `struct font_entity` - C 259-263's header + `props[FONT_ENTITY_MAX]`, then
   C 264-268's `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` `bool is_android`
   (dead: `HAVE_ANDROID` is undef, macros.tsv 1240) which is carried under `have_android`.
   Measured: sizeof 120, align 8, header@0, props@8.  */
#[repr(C)]
pub struct font_entity
{
  pub header: vectorlike_header,
  pub props: [Lisp_Object; FONT_ENTITY_MAX as usize],
  #[cfg (feature = "have_android")]
  pub is_android: bool,
}
const _: () = assert! (std::mem::size_of::<font_entity> () == 120);
const _: () = assert! (std::mem::align_of::<font_entity> () == 8);
const _: () = assert! (std::mem::offset_of! (font_entity, header) == 0);
const _: () = assert! (std::mem::offset_of! (font_entity, props) == 8);

/* font.h 414-417: `struct font_metrics` - five `short`s (C 416 is one line).  Measured:
   sizeof 10, align 2, lbearing@0 rbearing@2 width@4 ascent@6 descent@8.  */
#[repr(C)]
pub struct font_metrics
{
  pub lbearing: c_short,
  pub rbearing: c_short,
  pub width: c_short,
  pub ascent: c_short,
  pub descent: c_short,
}
const _: () = assert! (std::mem::size_of::<font_metrics> () == 10);
const _: () = assert! (std::mem::align_of::<font_metrics> () == 2);
const _: () = assert! (std::mem::offset_of! (font_metrics, ascent) == 6);
const _: () = assert! (std::mem::offset_of! (font_metrics, descent) == 8);

/* font.h 419-429: `struct font_bitmap`.  Measured: sizeof 40, align 8, `bits_per_pixel`@0
   `rows`@4 `width`@8 `pitch`@12 `buffer`@16 `left`@24 `top`@28 `advance`@32 (the four bytes
   of padding after `buffer` are C's own, which `repr(C)` reproduces).  */
#[repr(C)]
pub struct font_bitmap
{
  pub bits_per_pixel: c_int,
  pub rows: c_int,
  pub width: c_int,
  pub pitch: c_int,
  /// C: `unsigned char *buffer;`
  pub buffer: *mut c_uchar,
  pub left: c_int,
  pub top: c_int,
  pub advance: c_int,
}
const _: () = assert! (std::mem::size_of::<font_bitmap> () == 40);
const _: () = assert! (std::mem::align_of::<font_bitmap> () == 8);
const _: () = assert! (std::mem::offset_of! (font_bitmap, buffer) == 16);
const _: () = assert! (std::mem::offset_of! (font_bitmap, advance) == 32);


/* ---------------------------------------------------------------------------------------
   font.h 449-560, 902, 1021: the predicate/type-check/pointer-extraction families and the
   two capability functions.  Bodies are C's own text with the port's spellings (PORTING.md
   0.1-2/P-55: `eassert!`, `XUNTAG!`, `CHECK_TYPE`, the `Q*` accessors take `()`), each
   cross-header pointer keeping its `/// C:` prototype line.
   --------------------------------------------------------------------------------------- */

/* C 445-450: `GC_FONT_SPEC_P`.  */
pub fn GC_FONT_SPEC_P (x: Lisp_Object) -> bool
{
  FONTP (x) && (gc_asize (x) & PSEUDOVECTOR_SIZE_MASK) == FONT_SPEC_MAX as isize
}

/* C 459-464: `GC_FONT_ENTITY_P`.  */
pub fn GC_FONT_ENTITY_P (x: Lisp_Object) -> bool
{
  FONTP (x) && (gc_asize (x) & PSEUDOVECTOR_SIZE_MASK) == FONT_ENTITY_MAX as isize
}

/* C 473-478: `GC_FONT_OBJECT_P`.  */
pub fn GC_FONT_OBJECT_P (x: Lisp_Object) -> bool
{
  FONTP (x) && (gc_asize (x) & PSEUDOVECTOR_SIZE_MASK) == FONT_OBJECT_MAX as isize
}

/* C 484-486: `CHECK_FONT` - "type checking functions for various font-related objects".  */
pub fn CHECK_FONT (x: Lisp_Object)
{
  CHECK_TYPE (FONTP (x), Qfont (), x);
}

/* C 490-492: `CHECK_FONT_SPEC`.  */
pub fn CHECK_FONT_SPEC (x: Lisp_Object)
{
  CHECK_TYPE (FONT_SPEC_P (x), Qfont_spec (), x);
}

/* C 496-498: `CHECK_FONT_ENTITY`.  */
pub fn CHECK_FONT_ENTITY (x: Lisp_Object)
{
  CHECK_TYPE (FONT_ENTITY_P (x), Qfont_entity (), x);
}

/* C 502-504: `CHECK_FONT_OBJECT`.  */
pub fn CHECK_FONT_OBJECT (x: Lisp_Object)
{
  CHECK_TYPE (FONT_OBJECT_P (x), Qfont_object (), x);
}

/* C 511-516: `XFONT_SPEC` - "C pointer extraction functions for various font-related
   objects".  */
pub unsafe fn XFONT_SPEC (p: Lisp_Object) -> *mut font_spec
{
  eassert! (FONT_SPEC_P (p));
  XUNTAG! (p, Lisp_Type::Lisp_Vectorlike, font_spec)
}

/* C 518-523: `GC_XFONT_SPEC`.  */
pub unsafe fn GC_XFONT_SPEC (p: Lisp_Object) -> *mut font_spec
{
  eassert! (GC_FONT_SPEC_P (p));
  XUNTAG! (p, Lisp_Type::Lisp_Vectorlike, font_spec)
}

/* C 525-530: `XFONT_ENTITY`.  */
pub unsafe fn XFONT_ENTITY (p: Lisp_Object) -> *mut font_entity
{
  eassert! (FONT_ENTITY_P (p));
  XUNTAG! (p, Lisp_Type::Lisp_Vectorlike, font_entity)
}

/* C 532-537: `GC_XFONT_ENTITY`.  */
pub unsafe fn GC_XFONT_ENTITY (p: Lisp_Object) -> *mut font_entity
{
  eassert! (GC_FONT_ENTITY_P (p));
  XUNTAG! (p, Lisp_Type::Lisp_Vectorlike, font_entity)
}

/* C 539-544: `XFONT_OBJECT`.  */
pub unsafe fn XFONT_OBJECT (p: Lisp_Object) -> *mut font
{
  eassert! (FONT_OBJECT_P (p));
  XUNTAG! (p, Lisp_Type::Lisp_Vectorlike, font)
}

/* C 546-551: `GC_XFONT_OBJECT`.  */
pub unsafe fn GC_XFONT_OBJECT (p: Lisp_Object) -> *mut font
{
  eassert! (GC_FONT_OBJECT_P (p));
  XUNTAG! (p, Lisp_Type::Lisp_Vectorlike, font)
}

/* C 555-560: `CHECK_FONT_GET_OBJECT`.  */
pub unsafe fn CHECK_FONT_GET_OBJECT (x: Lisp_Object) -> *mut font
{
  CHECK_FONT_OBJECT (x);
  XFONT_OBJECT (x)
}

/* C 899-908: `valid_font_driver`.  `ENABLE_CHECKING` is undef (macros.tsv 872), so the frozen
   arm is C 903-907's `return true;`; C 902's prototype (the checking build's) is the
   `extern` declaration, spelled in the port's checking shape (the lisp.rs convention,
   PORTING.md P-11).  */
#[cfg (not (enable_checking))]
pub fn valid_font_driver (d: *const c_void) -> bool
{
  let _ = d;                      /* C's `d` is unused in this arm */
  true
}
/* C 902's prototype (the checking build's) is **not** declared here: `src/font.rs` 737-759
   *defines* the function under the same `enable_checking` cfg, and a Rust declaration plus a
   definition of one name in a crate is `E0428` (measured: `rustc --cfg enable_checking` on the
   font unit's root at status 3.225).  The units that call it declare it themselves - C's own
   reading of "header prototype, one definition" (PORTING.md P-11's shape, the
   `mark_haiku_display`/`haiku_handle_selection_clear` precedents).  */

/* C 1017-1030: `font_data_structures_may_be_ill_formed` - "for use in functions that can be
   called while garbage-collecting".  The frozen `USE_CAIRO`/`USE_BE_CAIRO` are undef (macros.tsv
   3857/3858), so C 1023-1026's `return gc_in_progress;` arm is dead and C 1028's `return
   false;` is live; both are carried so the token gate sees C's whole body.  */
pub fn font_data_structures_may_be_ill_formed () -> bool
{
  #[cfg (any (feature = "use_cairo", feature = "use_be_cairo"))]
  {
    gc_in_progress
  }
  #[cfg (not (any (feature = "use_cairo", feature = "use_be_cairo")))]
  {
    false
  }
}

/* ---------------------------------------------------------------------------------------
   font.h 589-837: the driver interface.  Both records moved here from femacs/src/font.rs (the
   unit that carried them at the use site until this round), so the mirror owns them; the unit
   reads them through this include now (PORTING.md P-26, the 3.181/3.211/3.220 ownership rule).
   Measured (Apple clang 21.0.0, the probe recorded in docs/ports/include-font.md):
   `struct font_driver` sizeof 208, align 8, `type`@0 `get_cache`@16 `list`@24 `match`@32
   `close_font`@64 `text_extents`@104 `otf_capability`@120 `shape`@152 `check`@160
   `cached_font_ok`@184 `combining_capability`@192; `struct font_driver_list` sizeof 24,
   align 8, on@0 driver@8 next@16.
   --------------------------------------------------------------------------------------- */
/// C: font.h 589-821: `struct font_driver`.  The members C guards with `HAVE_WINDOW_SYSTEM`
/// (`draw`, `get_bitmap`, `free_bitmap`) and `HAVE_HARFBUZZ` (`begin_hb_font`,
/// `end_hb_font`) are absent from the frozen layout and carried under the feature naming
/// the condition's other side (P-51).  Measured: `sizeof` 208, `type`@0,
/// `case_sensitive`@8, `get_cache`@16, `list`@24, `match`@32, `list_family`@40,
/// `free_entity`@48, `open_font`@56, `close_font`@64, `prepare_face`@72, `done_face`@80,
/// `has_char`@88, `encode_char`@96, `text_extents`@104, `anchor_point`@112,
/// `otf_capability`@120, `otf_drive`@128, `start_for_frame`@136, `end_for_frame`@144,
/// `shape`@152, `check`@160, `get_variation_glyphs`@168, `filter_properties`@176,
/// `cached_font_ok`@184, `combining_capability`@192, `drop_xrender_surfaces`@200.
#[repr(C)]
pub struct font_driver
{
  pub r#type: Lisp_Object,
  pub case_sensitive: bool,
  /// C 600: `Lisp_Object (*get_cache) (struct frame *f);` - `struct frame` is frame.rs's and
  /// not every unit that includes this mirror has it (C needs only the incomplete tag), so the
  /// pointer is untyped per P-72 (3), the `face_cache.f` precedent.
  pub get_cache: Option<unsafe extern "C" fn (*mut c_void) -> Lisp_Object>,
  /// C 623: `Lisp_Object (*list) (struct frame *frame, Lisp_Object font_spec);` (untyped, see get_cache)
  pub list: Option<unsafe extern "C" fn (*mut c_void, Lisp_Object) -> Lisp_Object>,
  /// C 632: `Lisp_Object (*match) (struct frame *f, Lisp_Object font_spec);` (untyped, see get_cache)
  pub r#match: Option<unsafe extern "C" fn (*mut c_void, Lisp_Object) -> Lisp_Object>,
  /// C 638: `Lisp_Object (*list_family) (struct frame *f);` (untyped, see get_cache)
  pub list_family: Option<unsafe extern "C" fn (*mut c_void) -> Lisp_Object>,
  pub free_entity: Option<unsafe extern "C" fn (Lisp_Object)>,
  /// C 645-646: `Lisp_Object (*open_font) (struct frame *f, Lisp_Object font_entity, int pixel_size);`
  /// (untyped, see get_cache)
  pub open_font: Option<unsafe extern "C" fn (*mut c_void, Lisp_Object, c_int) -> Lisp_Object>,
  pub close_font: Option<unsafe extern "C" fn (*mut font)>,
  /// C: `void (*prepare_face) (struct frame *f, struct face *face);` - `struct face` is
  /// dispextern.rs's and is `#[cfg (feature = "dispextern_face_records")]`-gated there (status
  /// 3.220), so the layout probe has no `face` and this keeps P-72 (3)'s `*mut c_void`.
  pub prepare_face: Option<unsafe extern "C" fn (*mut c_void, *mut c_void)>,
  /// C: `void (*done_face) (struct frame *f, struct face *face);`
  pub done_face: Option<unsafe extern "C" fn (*mut c_void, *mut c_void)>,
  pub has_char: Option<unsafe extern "C" fn (Lisp_Object, c_int) -> c_int>,
  pub encode_char: Option<unsafe extern "C" fn (*mut font, c_int) -> c_uint>,
  pub text_extents: Option<unsafe extern "C" fn (*mut font, *const c_uint, c_int,
                                                  *mut font_metrics)>,
  #[cfg(feature = "have_window_system")]
  pub draw: Option<unsafe extern "C" fn (*mut glyph_string, c_int, c_int, c_int, c_int,
                                         bool) -> c_int>,
  #[cfg(feature = "have_window_system")]
  pub get_bitmap: Option<unsafe extern "C" fn (*mut font, c_uint, *mut font_bitmap,
                                               c_int) -> c_int>,
  #[cfg(feature = "have_window_system")]
  pub free_bitmap: Option<unsafe extern "C" fn (*mut font, *mut font_bitmap)>,
  pub anchor_point: Option<unsafe extern "C" fn (*mut font, c_uint, c_int, *mut c_int,
                                                 *mut c_int) -> c_int>,
  pub otf_capability: Option<unsafe extern "C" fn (*mut font) -> Lisp_Object>,
  pub otf_drive: Option<unsafe extern "C" fn (*mut font, Lisp_Object, Lisp_Object, c_int,
                                              c_int, Lisp_Object, c_int, bool) -> c_int>,
  /// C 742: `int (*start_for_frame) (struct frame *f);` (untyped, see get_cache)
  pub start_for_frame: Option<unsafe extern "C" fn (*mut c_void) -> c_int>,
  /// C 747: `int (*end_for_frame) (struct frame *f);` (untyped, see get_cache)
  pub end_for_frame: Option<unsafe extern "C" fn (*mut c_void) -> c_int>,
  pub shape: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object) -> Lisp_Object>,
  /// C 771: `int (*check) (struct frame *f, struct font *font);` (the frame is untyped, see get_cache)
  pub check: Option<unsafe extern "C" fn (*mut c_void, *mut font) -> c_int>,
  pub get_variation_glyphs: Option<unsafe extern "C" fn (*mut font, c_int,
                                                         *mut c_uint) -> c_int>,
  pub filter_properties: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object)>,
  /// C 792: `bool (*cached_font_ok) (struct frame *f, Lisp_Object, Lisp_Object);` (untyped, see get_cache)
  pub cached_font_ok: Option<unsafe extern "C" fn (*mut c_void, Lisp_Object, Lisp_Object)
                                                 -> bool>,
  pub combining_capability: Option<unsafe extern "C" fn (*mut font) -> Lisp_Object>,
  /// C 805: `void (*drop_xrender_surfaces) (struct frame *f);` (untyped, see get_cache)
  pub drop_xrender_surfaces: Option<unsafe extern "C" fn (*mut c_void)>,
  #[cfg(feature = "harfbuzz")]
  pub begin_hb_font: Option<unsafe extern "C" fn (*mut font, *mut c_double) -> *mut c_void>,
  #[cfg(feature = "harfbuzz")]
  pub end_hb_font: Option<unsafe extern "C" fn (*mut font, *mut c_void)>,
}
/// C: font.h 828-837: `struct font_driver_list`.  Measured: `sizeof` 24, `on`@0,
/// `driver`@8, `next`@16.
#[repr(C)]
pub struct font_driver_list
{
  pub on: bool,
  pub driver: *const font_driver,
  pub next: *mut font_driver_list,
}

const _: () = assert! (std::mem::size_of::<font_driver> () == 208);
const _: () = assert! (std::mem::align_of::<font_driver> () == 8);
const _: () = assert! (std::mem::offset_of! (font_driver, r#type) == 0);
const _: () = assert! (std::mem::offset_of! (font_driver, get_cache) == 16);
const _: () = assert! (std::mem::offset_of! (font_driver, list) == 24);
const _: () = assert! (std::mem::offset_of! (font_driver, r#match) == 32);
const _: () = assert! (std::mem::offset_of! (font_driver, close_font) == 64);
const _: () = assert! (std::mem::offset_of! (font_driver, text_extents) == 104);
const _: () = assert! (std::mem::offset_of! (font_driver, otf_capability) == 120);
const _: () = assert! (std::mem::offset_of! (font_driver, shape) == 152);
const _: () = assert! (std::mem::offset_of! (font_driver, check) == 160);
const _: () = assert! (std::mem::offset_of! (font_driver, cached_font_ok) == 184);
const _: () = assert! (std::mem::offset_of! (font_driver, combining_capability) == 192);
const _: () = assert! (std::mem::size_of::<font_driver_list> () == 24);
const _: () = assert! (std::mem::align_of::<font_driver_list> () == 8);
const _: () = assert! (std::mem::offset_of! (font_driver_list, driver) == 8);
const _: () = assert! (std::mem::offset_of! (font_driver_list, next) == 16);
