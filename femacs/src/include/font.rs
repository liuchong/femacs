// femacs: port of src/font.h @ fd7afced403 .
//
// WIP mirror (P-66): data.c's `cl-type-of` reaches font.h through
// keyboard.h -> coding.h -> composite.h -> font.h (keyboard.h 23, coding.h 284,
// composite.h 28), and needs the three font predicates below.  The rest of the header -
// `struct font`, the driver interface, the property accessors - lands with font.c and the
// terminal backends, which is when docs/ports/include-font.md's remaining boxes are ticked.

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
