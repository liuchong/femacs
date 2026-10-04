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
