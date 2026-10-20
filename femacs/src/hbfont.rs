// femacs: port of src/hbfont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The three non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2).
//
// **This is a P-86 unit, and no `#if` in the file names its gate**: `hbfont.o` joins `FONT_OBJ`
// only when `HAVE_HARFBUZZ` is defined (`emacs/src/Makefile.in` 341-342, whose frozen
// `FONT_OBJ=` is empty; `docs/baseline/config/darwin-arm64.macros.tsv` 1389 records
// `HAVE_HARFBUZZ	undef`), so the frozen configuration compiles nothing from this file and the
// gate is the **build-level** `feature = "harfbuzz"` (the registered name, `femacs/xtask/
// Cargo.toml` 112; `docs/baseline/feature-coverage.tsv` 22) - on every item *and* on the include
// graph (P-86 ⑧).  Both arms of every `#if` in the file are carried:
//
//   * C 31-151's `#ifdef HAVE_NTGUI` arm is `feature = "have_ntgui"` (the C macro's literal
//     spelling, `femacs/xtask/Cargo.toml` 230), nested inside the file gate.  `HAVE_NTGUI` is
//     `undef` here and this machine has no Windows cross toolchain, so that arm's evidence is
//     `text-only` (P-86 ③⑵) - there is no compile reading of it.  C 87-114's 28 object-like
//     `#define`s (each `hb_x` to its loaded pointer `fn_hb_x`) have no Rust counterpart, so every
//     call site in the body is spelled the way C's preprocessor leaves it: `fn_<func> (…)` under
//     `have_ntgui`, the plain name otherwise (the `femacs/src/gnutls.rs` 1206-1210 shape).
//   * C 458-460's `#if 0` block is dead text in every configuration and is carried as a comment.
//
// **HarfBuzz is on this machine** (`/opt/homebrew/include/harfbuzz`, 14.4.0), so this port has a
// real compile arm and not a name-resolution reading: `docs/ports/hbfont.md` `## Evidence` ②
// records the flip.  `<hb.h>`/`<hb-ot.h>` have no file in the reference tree, so the declarations
// this unit reads stand in the unit at C 21-22's position (P-88 ①), and every layout number and
// constant in that segment is a clang reading of this machine's header (Evidence ⑤), never a
// recollection.  HarfBuzz has more than one consumer in the reference (`ftfont.c`, `sfntfont.c`
// and `w32uniscribe.c` all `#include <hb.h>`, and `font.h`/`ftfont.h` carry `hb_font_t` members),
// so P-90 ① says that surface belongs in a mirror of its own, `femacs/src/include/hb.rs`; this
// unit may not create it, so the segment is here for now and `## Owed` 2 names the move.
//
// C 33's `#include "w32common.h"` reaches this tree's mirror, and that mirror still keeps its
// `unsafe extern "C"` items at its own top level while this unit *defines* `hbfont_init_w32_funcs`
// - the P-39 `E0428` class, whose fix (a `pub mod …` plus a glob import *inside the mirror*) is
// the `femacs/src/include/sfntfont.rs` 96-141 shape.  The include is consumed through a module
// plus a glob import here, which is the P-21/P-28 shadowing mechanism C's "prototype plus
// definition" needs and not a new spelling of anything C has; `## Owed` 1 carries the move-back.
// See `## Deviations` 2.
//
// Four names the body reads are missing from the mirrors that own them and are therefore declared
// at their use sites with the reference header's line (P-72 rule 4, P-55): `lround` (`<math.h>`),
// `get_unicode_property` (lisp.h 4697), `bidi_mirror_char` (dispextern.h 3553) and
// `font_intern_prop` (font.h 885).  `## Owed` 3-6 names each.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_mut, unused_parens,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]

include!("include/config.rs");                  // C 19: `#include <config.h>`
#[cfg (feature = "harfbuzz")]
include!("../lib/include/math.rs");             // C 20: `#include <math.h>` (`lround` is the one
                                                // name read from it; see the use-site note below)

/* ----------------------------------------------------------------------------------------------
   C 21-22: `#include <hb.h>` and `#include <hb-ot.h>` - the HarfBuzz surface this file reads
   (P-88 ①; the header is not in the reference tree).  Each `/// C:` line quotes the header
   declaration verbatim, and the numbers are clang readings of this machine's header
   (HarfBuzz 14.4.0; the probe is in the ledger's `## Evidence` ⑤).  */

/// C: hb-common.h 95: `typedef int hb_bool_t;` - measured `sizeof` 4.
#[cfg (feature = "harfbuzz")]
pub type hb_bool_t = c_int;
/// C: hb-common.h 104: `typedef uint32_t hb_codepoint_t;` - measured `sizeof` 4.
#[cfg (feature = "harfbuzz")]
pub type hb_codepoint_t = u32;
/// C: hb-common.h 123: `typedef int32_t hb_position_t;` - measured `sizeof` 4.
#[cfg (feature = "harfbuzz")]
pub type hb_position_t = i32;
/// C: hb-common.h 130: `typedef uint32_t hb_mask_t;` - measured `sizeof` 4.
#[cfg (feature = "harfbuzz")]
pub type hb_mask_t = u32;
/// C: hb-common.h 165: `typedef uint32_t hb_tag_t;` - measured `sizeof` 4.
#[cfg (feature = "harfbuzz")]
pub type hb_tag_t = u32;
/// C: hb-common.h 132-139: `typedef union _hb_var_int_t { uint32_t u32; int32_t i32; uint16_t
/// u16[2]; int16_t i16[2]; uint8_t u8[4]; int8_t i8[4]; } hb_var_int_t;` - measured `sizeof` 4.
/// The port never reads a member (only the place it takes inside the two records below), and the
/// members keep C's names.
#[cfg (feature = "harfbuzz")]
#[repr (C)]
#[derive (Clone, Copy)]
pub union hb_var_int_t
{
  pub u32: u32,
  pub i32: i32,
  pub u16: [u16; 2],
  pub i16: [i16; 2],
  pub u8: [u8; 4],
  pub i8: [i8; 4],
}
/// C: hb-common.h 317: `typedef const struct hb_language_impl_t *hb_language_t;` - the record is
/// named only through a pointer (PORTING.md 0.1-2's zero-sized stand-in); measured `sizeof` 8.
#[cfg (feature = "harfbuzz")]
#[repr (C)]
pub struct hb_language_impl_t
{
  _private: [u8; 0],
}
#[cfg (feature = "harfbuzz")]
pub type hb_language_t = *const hb_language_impl_t;
/// C: hb-common.h 378: `typedef void (*hb_destroy_func_t) (void *user_data);` - measured
/// `sizeof` 8.
#[cfg (feature = "harfbuzz")]
pub type hb_destroy_func_t = Option<unsafe extern "C" fn (user_data: *mut c_void)>;
/// C: hb-common.h 238-243: `typedef enum { HB_DIRECTION_INVALID = 0, HB_DIRECTION_LTR = 4,
/// HB_DIRECTION_RTL, HB_DIRECTION_TTB, HB_DIRECTION_BTT } hb_direction_t;` - measured 0/4/5/6/7.
#[cfg (feature = "harfbuzz")]
pub type hb_direction_t = c_int;
#[cfg (feature = "harfbuzz")]
pub const HB_DIRECTION_INVALID: hb_direction_t = 0;
#[cfg (feature = "harfbuzz")]
pub const HB_DIRECTION_LTR: hb_direction_t = 4;
#[cfg (feature = "harfbuzz")]
pub const HB_DIRECTION_RTL: hb_direction_t = 5;
#[cfg (feature = "harfbuzz")]
pub const HB_DIRECTION_TTB: hb_direction_t = 6;
#[cfg (feature = "harfbuzz")]
pub const HB_DIRECTION_BTT: hb_direction_t = 7;
/// C: hb-common.h 296: `#define HB_DIRECTION_IS_BACKWARD(dir)	((((unsigned int) (dir)) & ~2U)
/// == 5)` - carried as the real expression, the `HB_DIRECTION_IS_BACKWARD (…)` call this file
/// makes included (measured: LTR 0, RTL 1).
#[cfg (feature = "harfbuzz")]
macro_rules! HB_DIRECTION_IS_BACKWARD {
  ($dir:expr) => { ((($dir) as c_uint & !2u32) == 5) };
}
/// C: hb-common.h 177: `#define HB_TAG(c1,c2,c3,c4) ((hb_tag_t)((((uint32_t)(c1)&0xFF)<<24)|((
/// (uint32_t)(c2)&0xFF)<<16)|(((uint32_t)(c3)&0xFF)<<8)|((uint32_t)(c4)&0xFF)))` - the real
/// expression; `HB_OT_TAG_GSUB`/`HB_OT_TAG_GPOS` below are its two uses.
#[cfg (feature = "harfbuzz")]
macro_rules! HB_TAG {
  ($c1:expr, $c2:expr, $c3:expr, $c4:expr) => {
    ((($c1 as hb_tag_t) & 0xff) << 24 | (($c2 as hb_tag_t) & 0xff) << 16
     | (($c3 as hb_tag_t) & 0xff) << 8 | (($c4 as hb_tag_t) & 0xff))
  };
}
/// C: hb-common.h 418-423: `typedef struct hb_feature_t { hb_tag_t tag; uint32_t value; unsigned
/// int start; unsigned int end; } hb_feature_t;` - measured `sizeof` 16, `tag`@0 `value`@4
/// `start`@8 `end`@12.  `hb_shape_full`'s third parameter and always a null pointer here.
#[cfg (feature = "harfbuzz")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct hb_feature_t
{
  pub tag: hb_tag_t,
  pub value: u32,
  pub start: c_uint,
  pub end: c_uint,
}
/// C: hb-unicode.h 289: `typedef struct hb_unicode_funcs_t hb_unicode_funcs_t;` - opaque here
/// (the record is the library's).
#[cfg (feature = "harfbuzz")]
#[repr (C)]
pub struct hb_unicode_funcs_t
{
  _private: [u8; 0],
}
/// C: hb-unicode.h 354-357: `typedef hb_unicode_combining_class_t (*hb_unicode_combining_class_
/// func_t) (hb_unicode_funcs_t *ufuncs, hb_codepoint_t unicode, void *user_data);`
#[cfg (feature = "harfbuzz")]
pub type hb_unicode_combining_class_func_t
  = Option<unsafe extern "C" fn (ufuncs: *mut hb_unicode_funcs_t, unicode: hb_codepoint_t,
				 user_data: *mut c_void) -> hb_unicode_combining_class_t>;
/// C: hb-unicode.h 372-375: `typedef hb_unicode_general_category_t (*hb_unicode_general_category_
/// func_t) (hb_unicode_funcs_t *ufuncs, hb_codepoint_t unicode, void *user_data);`
#[cfg (feature = "harfbuzz")]
pub type hb_unicode_general_category_func_t
  = Option<unsafe extern "C" fn (ufuncs: *mut hb_unicode_funcs_t, unicode: hb_codepoint_t,
				 user_data: *mut c_void) -> hb_unicode_general_category_t>;
/// C: hb-unicode.h 392-394: `typedef hb_codepoint_t (*hb_unicode_mirroring_func_t)
/// (hb_unicode_funcs_t *ufuncs, hb_codepoint_t unicode, void *user_data);`
#[cfg (feature = "harfbuzz")]
pub type hb_unicode_mirroring_func_t
  = Option<unsafe extern "C" fn (ufuncs: *mut hb_unicode_funcs_t, unicode: hb_codepoint_t,
				 user_data: *mut c_void) -> hb_codepoint_t>;
/// C: hb-unicode.h 123: the `hb_unicode_general_category_t` enum - measured in declaration order,
/// 0..29 (UNASSIGNED **2**, CONTROL 0, FORMAT 1, PRIVATE_USE 3, SURROGATE 4, LOWERCASE_LETTER 5,
/// MODIFIER_LETTER 6, OTHER_LETTER 7, TITLECASE_LETTER 8, UPPERCASE_LETTER 9, SPACING_MARK 10,
/// ENCLOSING_MARK 11, NON_SPACING_MARK 12, DECIMAL_NUMBER 13, LETTER_NUMBER 14, OTHER_NUMBER 15,
/// CONNECT_PUNCTUATION 16, DASH_PUNCTUATION 17, CLOSE_PUNCTUATION 18, FINAL_PUNCTUATION 19,
/// INITIAL_PUNCTUATION 20, OTHER_PUNCTUATION 21, OPEN_PUNCTUATION 22, CURRENCY_SYMBOL 23,
/// MODIFIER_SYMBOL 24, MATH_SYMBOL 25, OTHER_SYMBOL 26, LINE_SEPARATOR 27, PARAGRAPH_SEPARATOR
/// 28, SPACE_SEPARATOR 29); measured `sizeof` 4.
#[cfg (feature = "harfbuzz")]
pub type hb_unicode_general_category_t = c_int;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_UNASSIGNED: hb_unicode_general_category_t = 2;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_CONTROL: hb_unicode_general_category_t = 0;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_FORMAT: hb_unicode_general_category_t = 1;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_PRIVATE_USE: hb_unicode_general_category_t = 3;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_SURROGATE: hb_unicode_general_category_t = 4;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_LOWERCASE_LETTER: hb_unicode_general_category_t = 5;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_MODIFIER_LETTER: hb_unicode_general_category_t = 6;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_OTHER_LETTER: hb_unicode_general_category_t = 7;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_TITLECASE_LETTER: hb_unicode_general_category_t = 8;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_UPPERCASE_LETTER: hb_unicode_general_category_t = 9;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_SPACING_MARK: hb_unicode_general_category_t = 10;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_ENCLOSING_MARK: hb_unicode_general_category_t = 11;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_NON_SPACING_MARK: hb_unicode_general_category_t = 12;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_DECIMAL_NUMBER: hb_unicode_general_category_t = 13;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_LETTER_NUMBER: hb_unicode_general_category_t = 14;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_OTHER_NUMBER: hb_unicode_general_category_t = 15;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_CONNECT_PUNCTUATION: hb_unicode_general_category_t = 16;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_DASH_PUNCTUATION: hb_unicode_general_category_t = 17;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_CLOSE_PUNCTUATION: hb_unicode_general_category_t = 18;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_FINAL_PUNCTUATION: hb_unicode_general_category_t = 19;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_INITIAL_PUNCTUATION: hb_unicode_general_category_t = 20;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_OTHER_PUNCTUATION: hb_unicode_general_category_t = 21;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_OPEN_PUNCTUATION: hb_unicode_general_category_t = 22;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_CURRENCY_SYMBOL: hb_unicode_general_category_t = 23;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_MODIFIER_SYMBOL: hb_unicode_general_category_t = 24;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_MATH_SYMBOL: hb_unicode_general_category_t = 25;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_OTHER_SYMBOL: hb_unicode_general_category_t = 26;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_LINE_SEPARATOR: hb_unicode_general_category_t = 27;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_PARAGRAPH_SEPARATOR: hb_unicode_general_category_t = 28;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_GENERAL_CATEGORY_SPACE_SEPARATOR: hb_unicode_general_category_t = 29;
/// C: hb-unicode.h 195: `HB_UNICODE_COMBINING_CLASS_NOT_REORDERED = 0,` - the only member of
/// `hb_unicode_combining_class_t` this file names (measured 0; `sizeof` 4).
#[cfg (feature = "harfbuzz")]
pub type hb_unicode_combining_class_t = c_int;
#[cfg (feature = "harfbuzz")]
pub const HB_UNICODE_COMBINING_CLASS_NOT_REORDERED: hb_unicode_combining_class_t = 0;
/// C: hb-buffer.h 62-72: `typedef struct hb_glyph_info_t { hb_codepoint_t codepoint; hb_mask_t
/// mask; uint32_t cluster; hb_var_int_t var1; hb_var_int_t var2; } hb_glyph_info_t;` - measured
/// `sizeof` 20, `codepoint`@0 `mask`@4 `cluster`@8 `var1`@12 `var2`@16.
#[cfg (feature = "harfbuzz")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct hb_glyph_info_t
{
  pub codepoint: hb_codepoint_t,
  pub mask: hb_mask_t,
  pub cluster: u32,
  pub var1: hb_var_int_t,
  pub var2: hb_var_int_t,
}
/// C: hb-buffer.h 181-190: `typedef struct hb_glyph_position_t { hb_position_t x_advance;
/// hb_position_t y_advance; hb_position_t x_offset; hb_position_t y_offset; hb_var_int_t var; }` -
/// measured `sizeof` 20, `x_advance`@0 `y_advance`@4 `x_offset`@8 `y_offset`@12 `var`@16.
#[cfg (feature = "harfbuzz")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct hb_glyph_position_t
{
  pub x_advance: hb_position_t,
  pub y_advance: hb_position_t,
  pub x_offset: hb_position_t,
  pub y_offset: hb_position_t,
  pub var: hb_var_int_t,
}
/* The three records' measured layout, pinned the way the mirrors pin theirs (PORTING.md 1.3).
   Each assertion carries the file gate, because the records it names do: the frozen
   configuration compiles this unit to an empty object and the assertions must not outlive the
   types (measured: ungated, `build-tu src/hbfont.c` reported five `E0425`s).  */
#[cfg (feature = "harfbuzz")]
const _: () = core::assert! (std::mem::size_of::<hb_glyph_info_t> () == 20);
#[cfg (feature = "harfbuzz")]
const _: () = core::assert! (std::mem::offset_of! (hb_glyph_info_t, cluster) == 8);
#[cfg (feature = "harfbuzz")]
const _: () = core::assert! (std::mem::size_of::<hb_glyph_position_t> () == 20);
#[cfg (feature = "harfbuzz")]
const _: () = core::assert! (std::mem::offset_of! (hb_glyph_position_t, x_offset) == 8);
#[cfg (feature = "harfbuzz")]
const _: () = core::assert! (std::mem::size_of::<hb_feature_t> () == 16);
/// C: hb-buffer.h 292-295: `typedef enum { HB_BUFFER_CONTENT_TYPE_INVALID = 0,
/// HB_BUFFER_CONTENT_TYPE_UNICODE, HB_BUFFER_CONTENT_TYPE_GLYPHS } hb_buffer_content_type_t;` -
/// measured 0/1/2.
#[cfg (feature = "harfbuzz")]
pub type hb_buffer_content_type_t = c_int;
#[cfg (feature = "harfbuzz")]
pub const HB_BUFFER_CONTENT_TYPE_INVALID: hb_buffer_content_type_t = 0;
#[cfg (feature = "harfbuzz")]
pub const HB_BUFFER_CONTENT_TYPE_UNICODE: hb_buffer_content_type_t = 1;
#[cfg (feature = "harfbuzz")]
pub const HB_BUFFER_CONTENT_TYPE_GLYPHS: hb_buffer_content_type_t = 2;
/// C: hb-buffer.h 460-467: `typedef enum { HB_BUFFER_CLUSTER_LEVEL_MONOTONE_GRAPHEMES = 0,
/// HB_BUFFER_CLUSTER_LEVEL_MONOTONE_CHARACTERS = 1, HB_BUFFER_CLUSTER_LEVEL_CHARACTERS = 2,
/// HB_BUFFER_CLUSTER_LEVEL_DEFAULT = HB_BUFFER_CLUSTER_LEVEL_MONOTONE_GRAPHEMES }` - measured
/// 0/1/2/0.
#[cfg (feature = "harfbuzz")]
pub type hb_buffer_cluster_level_t = c_int;
#[cfg (feature = "harfbuzz")]
pub const HB_BUFFER_CLUSTER_LEVEL_MONOTONE_GRAPHEMES: hb_buffer_cluster_level_t = 0;
#[cfg (feature = "harfbuzz")]
pub const HB_BUFFER_CLUSTER_LEVEL_MONOTONE_CHARACTERS: hb_buffer_cluster_level_t = 1;
#[cfg (feature = "harfbuzz")]
pub const HB_BUFFER_CLUSTER_LEVEL_CHARACTERS: hb_buffer_cluster_level_t = 2;
/// C: hb-face.h 48: `typedef struct hb_face_t hb_face_t;` - opaque here.
#[cfg (feature = "harfbuzz")]
#[repr (C)]
pub struct hb_face_t
{
  _private: [u8; 0],
}
/// C: hb-font.h 63: `typedef struct hb_font_t hb_font_t;` - opaque here.
#[cfg (feature = "harfbuzz")]
#[repr (C)]
pub struct hb_font_t
{
  _private: [u8; 0],
}
/// C: hb-buffer.h 55: `typedef struct hb_buffer_t hb_buffer_t;` - opaque here.
#[cfg (feature = "harfbuzz")]
#[repr (C)]
pub struct hb_buffer_t
{
  _private: [u8; 0],
}
/// C: hb-ot-layout.h 58: `#define HB_OT_TAG_GSUB HB_TAG('G','S','U','B')` - measured
/// 1196643650 (0x47535542).
#[cfg (feature = "harfbuzz")]
pub const HB_OT_TAG_GSUB: hb_tag_t = HB_TAG! (b'G', b'S', b'U', b'B');
/// C: hb-ot-layout.h 64: `#define HB_OT_TAG_GPOS HB_TAG('G','P','O','S')` - measured
/// 1196445523 (0x47504f53).
#[cfg (feature = "harfbuzz")]
pub const HB_OT_TAG_GPOS: hb_tag_t = HB_TAG! (b'G', b'P', b'O', b'S');
/// C: hb-ot-layout.h 205: `#define HB_OT_LAYOUT_DEFAULT_LANGUAGE_INDEX	0xFFFFu` - measured
/// 65535.
#[cfg (feature = "harfbuzz")]
pub const HB_OT_LAYOUT_DEFAULT_LANGUAGE_INDEX: c_uint = 0xFFFF;
/// C: hb-unicode.h 296-300: `hb_unicode_funcs_t * hb_unicode_funcs_get_default (void);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_unicode_funcs_get_default () -> *mut hb_unicode_funcs_t;
}
/// C: hb-unicode.h 300: `hb_unicode_funcs_t * hb_unicode_funcs_create (hb_unicode_funcs_t
/// *parent);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_unicode_funcs_create (parent: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;
}
/// C: hb-unicode.h 479-482: `void hb_unicode_funcs_set_combining_class_func (hb_unicode_funcs_t
/// *ufuncs, hb_unicode_combining_class_func_t combining_class_func, void *user_data,
/// hb_destroy_func_t destroy);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_unicode_funcs_set_combining_class_func
    (ufuncs: *mut hb_unicode_funcs_t, combining_class_func: hb_unicode_combining_class_func_t,
     user_data: *mut c_void, destroy: hb_destroy_func_t);
}
/// C: hb-unicode.h 495-498: the same shape for `hb_unicode_funcs_set_general_category_func`.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_unicode_funcs_set_general_category_func
    (ufuncs: *mut hb_unicode_funcs_t, general_category_func: hb_unicode_general_category_func_t,
     user_data: *mut c_void, destroy: hb_destroy_func_t);
}
/// C: hb-unicode.h 511-514: the same shape for `hb_unicode_funcs_set_mirroring_func`.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_unicode_funcs_set_mirroring_func
    (ufuncs: *mut hb_unicode_funcs_t, mirroring_func: hb_unicode_mirroring_func_t,
     user_data: *mut c_void, destroy: hb_destroy_func_t);
}
/// C: hb-buffer.h 252-253: `hb_buffer_t * hb_buffer_create (void);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_create () -> *mut hb_buffer_t;
}
/// C: hb-buffer.h 305-307: `void hb_buffer_set_unicode_funcs (hb_buffer_t *buffer,
/// hb_unicode_funcs_t *unicode_funcs);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_set_unicode_funcs (buffer: *mut hb_buffer_t,
				      unicode_funcs: *mut hb_unicode_funcs_t);
}
/// C: hb-buffer.h 568: `void hb_buffer_clear_contents (hb_buffer_t *buffer);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_clear_contents (buffer: *mut hb_buffer_t);
}
/// C: hb-buffer.h 571-573: `hb_bool_t hb_buffer_pre_allocate (hb_buffer_t *buffer, unsigned int
/// size);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_pre_allocate (buffer: *mut hb_buffer_t, size: c_uint) -> hb_bool_t;
}
/// C: hb-buffer.h 592-596: `void hb_buffer_add (hb_buffer_t *buffer, hb_codepoint_t codepoint,
/// unsigned int cluster);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_add (buffer: *mut hb_buffer_t, codepoint: hb_codepoint_t, cluster: c_uint);
}
/// C: hb-buffer.h 297-299: `void hb_buffer_set_content_type (hb_buffer_t *buffer,
/// hb_buffer_content_type_t content_type);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_set_content_type (buffer: *mut hb_buffer_t,
				     content_type: hb_buffer_content_type_t);
}
/// C: hb-buffer.h 512-514: `void hb_buffer_set_cluster_level (hb_buffer_t *buffer,
/// hb_buffer_cluster_level_t cluster_level);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_set_cluster_level (buffer: *mut hb_buffer_t,
				      cluster_level: hb_buffer_cluster_level_t);
}
/// C: hb-buffer.h 312-314: `void hb_buffer_set_direction (hb_buffer_t *buffer, hb_direction_t
/// direction);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_set_direction (buffer: *mut hb_buffer_t, direction: hb_direction_t);
}
/// C: hb-buffer.h 326-328: `void hb_buffer_set_language (hb_buffer_t *buffer, hb_language_t
/// language);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_set_language (buffer: *mut hb_buffer_t, language: hb_language_t);
}
/// C: hb-common.h 319-320: `hb_language_t hb_language_from_string (const char *str, int len);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_language_from_string (str: *const c_char, len: c_int) -> hb_language_t;
}
/// C: hb-buffer.h 343-344: `void hb_buffer_guess_segment_properties (hb_buffer_t *buffer);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_guess_segment_properties (buffer: *mut hb_buffer_t);
}
/// C: hb-shape.h 49-54: `hb_bool_t hb_shape_full (hb_font_t *font, hb_buffer_t *buffer, const
/// hb_feature_t *features, unsigned int num_features, const char * const *shaper_list);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_shape_full (font: *mut hb_font_t, buffer: *mut hb_buffer_t,
			features: *const hb_feature_t, num_features: c_uint,
			shaper_list: *const *const c_char) -> hb_bool_t;
}
/// C: hb-buffer.h 641-642: `unsigned int hb_buffer_get_length (const hb_buffer_t *buffer);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_get_length (buffer: *const hb_buffer_t) -> c_uint;
}
/// C: hb-buffer.h 316-317: `hb_direction_t hb_buffer_get_direction (const hb_buffer_t *buffer);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_get_direction (buffer: *const hb_buffer_t) -> hb_direction_t;
}
/// C: hb-buffer.h 586: `void hb_buffer_reverse_clusters (hb_buffer_t *buffer);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_reverse_clusters (buffer: *mut hb_buffer_t);
}
/// C: hb-buffer.h 646-648: `hb_glyph_info_t * hb_buffer_get_glyph_infos (hb_buffer_t *buffer,
/// unsigned int *length);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_get_glyph_infos (buffer: *mut hb_buffer_t, length: *mut c_uint)
    -> *mut hb_glyph_info_t;
}
/// C: hb-buffer.h 650-652: `hb_glyph_position_t * hb_buffer_get_glyph_positions (hb_buffer_t
/// *buffer, unsigned int *length);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_buffer_get_glyph_positions (buffer: *mut hb_buffer_t, length: *mut c_uint)
    -> *mut hb_glyph_position_t;
}
/// C: hb-common.h 218-219: `void hb_tag_to_string (hb_tag_t tag, char *buf);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_tag_to_string (tag: hb_tag_t, buf: *mut c_char);
}
/// C: hb-font.h 1167: `hb_face_t * hb_font_get_face (hb_font_t *font);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_font_get_face (font: *mut hb_font_t) -> *mut hb_face_t;
}
/// C: hb-ot-layout.h 213-219: `unsigned int hb_ot_layout_table_get_script_tags (hb_face_t *face,
/// hb_tag_t table_tag, unsigned int start_offset, unsigned int *script_count, hb_tag_t
/// *script_tags);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_ot_layout_table_get_script_tags (face: *mut hb_face_t, table_tag: hb_tag_t,
					     start_offset: c_uint, script_count: *mut c_uint,
					     script_tags: *mut hb_tag_t) -> c_uint;
}
/// C: hb-ot-layout.h 234-240: the same shape for `hb_ot_layout_table_get_feature_tags`.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_ot_layout_table_get_feature_tags (face: *mut hb_face_t, table_tag: hb_tag_t,
					      start_offset: c_uint, feature_count: *mut c_uint,
					      feature_tags: *mut hb_tag_t) -> c_uint;
}
/// C: hb-ot-layout.h 241-247: `unsigned int hb_ot_layout_script_get_language_tags (hb_face_t
/// *face, hb_tag_t table_tag, unsigned int script_index, unsigned int start_offset, unsigned int
/// *language_count, hb_tag_t *language_tags);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_ot_layout_script_get_language_tags
    (face: *mut hb_face_t, table_tag: hb_tag_t, script_index: c_uint, start_offset: c_uint,
     language_count: *mut c_uint, language_tags: *mut hb_tag_t) -> c_uint;
}
/// C: hb-ot-layout.h 290-298: `unsigned int hb_ot_layout_language_get_feature_tags (hb_face_t
/// *face, hb_tag_t table_tag, unsigned int script_index, unsigned int language_index, unsigned int
/// start_offset, unsigned int *feature_count, hb_tag_t *feature_tags);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_ot_layout_language_get_feature_tags
    (face: *mut hb_face_t, table_tag: hb_tag_t, script_index: c_uint, language_index: c_uint,
     start_offset: c_uint, feature_count: *mut c_uint, feature_tags: *mut hb_tag_t) -> c_uint;
}

/* C 24-29: the six C headers, under the file gate (P-86 ⑧).  Each mirror is the one the
   corresponding header has; the edges their own text reaches through other headers are spelled
   with them (the `src/gfilenotify.rs` 40-52 shape).  */
#[cfg (feature = "harfbuzz")]
include!("include/lisp.rs");                    // C 24: `#include "lisp.h"`
#[cfg (feature = "harfbuzz")]
include!("include/frame.rs");                   // C 25: `#include "frame.h"`
#[cfg (feature = "harfbuzz")]
include!("include/composite.rs");               // C 26: `#include "composite.h"` (LGSTRING_*,
                                                // LGLYPH_*; it reaches `FONT_INVALID_CODE` through
                                                // font.h's position in composite.h - C 28 below)
#[cfg (feature = "harfbuzz")]
include!("include/font.rs");                    // C 27: `#include "font.h"` (`struct font`,
                                                // `font_driver`, `font_metrics`,
                                                // `CHECK_FONT_GET_OBJECT`, `FONT_INVALID_CODE`)
#[cfg (feature = "harfbuzz")]
include!("include/dispextern.rs");              // C 28: `#include "dispextern.h"`
#[cfg (feature = "harfbuzz")]
include!("include/buffer.rs");                  // C 29: `#include "buffer.h"` (`BVAR`)

/* The edges the six headers above reach through *other* headers, spelled with them (P-86 ⑧;
   the `femacs/src/gfilenotify.rs` 40-52 shape and `femacs/src/abi_shim.rs` 24-42's list for this
   same neighbourhood).  `frame.h` pulls `termhooks.h` and `window.h` (its mirror does that
   itself), `termhooks.h` names `Time` (systime.h) and `Emacs_Color` (dispextern.h, above),
   `dispextern.h` names `character.h`'s helpers and pulls `coding.h`, and `buffer.h` names
   `intervals.h`/`itree.h`; `lisp.h` 28-34's four system headers are the ones its mirror does not
   carry itself.  */
#[cfg (feature = "harfbuzz")]
include!("../lib/include/string.rs");           // lisp.h 30: `#include <string.h>`
#[cfg (feature = "harfbuzz")]
include!("../lib/include/stdcountof.rs");       // lisp.h 28: `#include <stdcountof.h>`
#[cfg (feature = "harfbuzz")]
include!("../lib/include/limits.in.rs");        // lisp.h 33: `#include <limits.h>`
#[cfg (feature = "harfbuzz")]
include!("include/character.rs");               // lisp.h's `MAX_CHAR` and the category enum
/* `unicode_category_t`'s members are named bare in C's `switch`; the port takes them into scope
   the way `femacs/src/character.rs` 72 does (a glob import of the enum's variants).  */
#[cfg (feature = "harfbuzz")]
use unicode_category_t::*;
#[cfg (feature = "harfbuzz")]
include!("include/systime.rs");                 // termhooks.h's `Time`
#[cfg (feature = "harfbuzz")]
include!("include/coding.rs");                  // dispextern.h 29: `#include "coding.h"`
#[cfg (feature = "harfbuzz")]
include!("include/window.rs");                  // frame.h 25: `#include "window.h"`
#[cfg (feature = "harfbuzz")]
include!("include/intervals.rs");               // buffer.h 27: `#include "intervals.h"`

/* The four names the body below reads that the mirrors above do not carry, each at its use site
   with the reference header's line (P-72 rule 4, P-55); `## Owed` 3-6 names the mirror each
   belongs to.  */
/// C: math.h 474: `extern long int lround (double __x) __attribute__ ((__nothrow__ ));`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn lround (x: c_double) -> c_long;
}
/// C: lisp.h 4697: `extern Lisp_Object get_unicode_property (Lisp_Object, int);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn get_unicode_property (table: Lisp_Object, ch: c_int) -> Lisp_Object;
}
/// C: dispextern.h 3553: `extern int  bidi_mirror_char (int);`
#[cfg (feature = "harfbuzz")]
/// C: font.h 885-886: `extern Lisp_Object font_intern_prop (const char *str, ptrdiff_t len,
/// bool force_symbol);`
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn font_intern_prop (str: *const c_char, len: ptrdiff_t, force_symbol: bool)
    -> Lisp_Object;
}

/* ----------------------------------------------------------------------------------------------
   C 31-151: `#ifdef HAVE_NTGUI` - the Windows build loads HarfBuzz from a DLL through function
   pointers.  `HAVE_NTGUI` is `undef` in the frozen configuration and this machine has no Windows
   cross toolchain, so this arm is text-only (P-86 ③⑵) and nothing below it has a compile
   reading.

   C 31's guard is `feature = "have_ntgui"` (the C macro's literal spelling).  C 33's
   `#include "w32common.h"` is consumed as C consumes it, but through a module plus a glob import:
   that mirror still keeps its `unsafe extern "C"` items at its own top level, and this unit
   *defines* `hbfont_init_w32_funcs` - the P-39 `E0428` class.  The module is the P-21/P-28
   shadowing mechanism ("a same-named definition shadows a glob import"), so the mirror's
   prototype and this unit's definition coexist exactly as C's do; `## Owed` 1 moves the module
   into the mirror and deletes this wrapper.  */

#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
mod w32common_decls
{
  include!("include/w32common.rs");             // C 33: `#include "w32common.h"`
}
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
use w32common_decls::*;

/* C 35-37 introduces the whole point of the arm: HarfBuzz is reached through pointers loaded at
   run time, so every entry point below is declared as a DLL function and then renamed to its
   loaded pointer (C 87-114).  */

/// C: w32common.h 77-79: `#define DEF_DLL_FN(type, func, ...)  typedef type (CDECL *W32_PFN_##
/// func) __VA_ARGS__; static W32_PFN_##func fn_##func` - the macro `femacs/src/include/
/// w32common.rs` keeps `deferred` (P-22).  This unit is its fourth consumer
/// (`femacs/src/decompress.rs`, `femacs/src/lcms.rs` and `femacs/src/gnutls.rs` are the first
/// three) and repeats the long form for the same reason: a unit is its own crate.  C pastes
/// `W32_PFN_##func` and `fn_##func` together, which Rust cannot do, so both generated names are
/// parameters here and C's `__VA_ARGS__` (the parameter list) is the `$args` fragment.  C's
/// `CDECL` is `__cdecl`, empty on this target's ABI.
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
macro_rules! DEF_DLL_FN {
  ($type:ty, $func:ident, $pfn:ident, $fnvar:ident, ($($arg:ty),* $(,)?)) => {
    pub type $pfn = Option<unsafe extern "C" fn ($($arg),*) -> $type>;
    static mut $fnvar: $pfn = None;
  };
}
/// C: w32common.h 82-89: `#define LOAD_DLL_FN(lib, func) do { fn_##func = (W32_PFN_##func)
/// get_proc_addr (lib, #func); if (!fn_##func) return false; } while (false)`.  `do { … } while
/// (false)` is a block expression (PORTING.md 2.1), C's `#func` is the stringified name (a
/// parameter here), and the cast between two function-pointer types is the `transmute` Rust has
/// instead of C's `as` cast; `get_proc_addr` is the mirror's own inline definition.
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
macro_rules! LOAD_DLL_FN {
  ($lib:expr, $func:ident, $pfn:ident, $fnvar:ident, $name:expr) => {
    {
      $fnvar = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
      if $fnvar.is_none () { return false; }
    }
  };
}
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (*mut hb_unicode_funcs_t, hb_unicode_funcs_create, W32_PFN_hb_unicode_funcs_create,
             fn_hb_unicode_funcs_create, (*mut hb_unicode_funcs_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (*mut hb_unicode_funcs_t, hb_unicode_funcs_get_default,
             W32_PFN_hb_unicode_funcs_get_default, fn_hb_unicode_funcs_get_default, ());
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_unicode_funcs_set_combining_class_func,
             W32_PFN_hb_unicode_funcs_set_combining_class_func,
             fn_hb_unicode_funcs_set_combining_class_func,
             (*mut hb_unicode_funcs_t, hb_unicode_combining_class_func_t, *mut c_void,
              hb_destroy_func_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_unicode_funcs_set_general_category_func,
             W32_PFN_hb_unicode_funcs_set_general_category_func,
             fn_hb_unicode_funcs_set_general_category_func,
             (*mut hb_unicode_funcs_t, hb_unicode_general_category_func_t, *mut c_void,
              hb_destroy_func_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_unicode_funcs_set_mirroring_func,
             W32_PFN_hb_unicode_funcs_set_mirroring_func, fn_hb_unicode_funcs_set_mirroring_func,
             (*mut hb_unicode_funcs_t, hb_unicode_mirroring_func_t, *mut c_void,
              hb_destroy_func_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (*mut hb_buffer_t, hb_buffer_create, W32_PFN_hb_buffer_create, fn_hb_buffer_create,
             ());
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_buffer_set_unicode_funcs, W32_PFN_hb_buffer_set_unicode_funcs,
             fn_hb_buffer_set_unicode_funcs, (*mut hb_buffer_t, *mut hb_unicode_funcs_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_buffer_clear_contents, W32_PFN_hb_buffer_clear_contents,
             fn_hb_buffer_clear_contents, (*mut hb_buffer_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (hb_bool_t, hb_buffer_pre_allocate, W32_PFN_hb_buffer_pre_allocate,
             fn_hb_buffer_pre_allocate, (*mut hb_buffer_t, c_uint));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_buffer_add, W32_PFN_hb_buffer_add, fn_hb_buffer_add,
             (*mut hb_buffer_t, hb_codepoint_t, c_uint));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_buffer_set_content_type, W32_PFN_hb_buffer_set_content_type,
             fn_hb_buffer_set_content_type, (*mut hb_buffer_t, hb_buffer_content_type_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_buffer_set_cluster_level, W32_PFN_hb_buffer_set_cluster_level,
             fn_hb_buffer_set_cluster_level, (*mut hb_buffer_t, hb_buffer_cluster_level_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_buffer_set_direction, W32_PFN_hb_buffer_set_direction,
             fn_hb_buffer_set_direction, (*mut hb_buffer_t, hb_direction_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_buffer_set_language, W32_PFN_hb_buffer_set_language,
             fn_hb_buffer_set_language, (*mut hb_buffer_t, hb_language_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (hb_language_t, hb_language_from_string, W32_PFN_hb_language_from_string,
             fn_hb_language_from_string, (*const c_char, c_int));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_buffer_guess_segment_properties, W32_PFN_hb_buffer_guess_segment_properties,
             fn_hb_buffer_guess_segment_properties, (*mut hb_buffer_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (hb_bool_t, hb_shape_full, W32_PFN_hb_shape_full, fn_hb_shape_full,
             (*mut hb_font_t, *mut hb_buffer_t, *const hb_feature_t, c_uint,
              *const *const c_char));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (c_uint, hb_buffer_get_length, W32_PFN_hb_buffer_get_length,
             fn_hb_buffer_get_length, (*const hb_buffer_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (hb_direction_t, hb_buffer_get_direction, W32_PFN_hb_buffer_get_direction,
             fn_hb_buffer_get_direction, (*const hb_buffer_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_buffer_reverse_clusters, W32_PFN_hb_buffer_reverse_clusters,
             fn_hb_buffer_reverse_clusters, (*mut hb_buffer_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (*mut hb_glyph_info_t, hb_buffer_get_glyph_infos, W32_PFN_hb_buffer_get_glyph_infos,
             fn_hb_buffer_get_glyph_infos, (*mut hb_buffer_t, *mut c_uint));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (*mut hb_glyph_position_t, hb_buffer_get_glyph_positions,
             W32_PFN_hb_buffer_get_glyph_positions, fn_hb_buffer_get_glyph_positions,
             (*mut hb_buffer_t, *mut c_uint));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! ((), hb_tag_to_string, W32_PFN_hb_tag_to_string, fn_hb_tag_to_string,
             (hb_tag_t, *mut c_char));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (*mut hb_face_t, hb_font_get_face, W32_PFN_hb_font_get_face, fn_hb_font_get_face,
             (*mut hb_font_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (c_uint, hb_ot_layout_table_get_script_tags,
             W32_PFN_hb_ot_layout_table_get_script_tags, fn_hb_ot_layout_table_get_script_tags,
             (*mut hb_face_t, hb_tag_t, c_uint, *mut c_uint, *mut hb_tag_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (c_uint, hb_ot_layout_table_get_feature_tags,
             W32_PFN_hb_ot_layout_table_get_feature_tags, fn_hb_ot_layout_table_get_feature_tags,
             (*mut hb_face_t, hb_tag_t, c_uint, *mut c_uint, *mut hb_tag_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (c_uint, hb_ot_layout_script_get_language_tags,
             W32_PFN_hb_ot_layout_script_get_language_tags,
             fn_hb_ot_layout_script_get_language_tags,
             (*mut hb_face_t, hb_tag_t, c_uint, c_uint, *mut c_uint, *mut hb_tag_t));
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
DEF_DLL_FN! (c_uint, hb_ot_layout_language_get_feature_tags,
             W32_PFN_hb_ot_layout_language_get_feature_tags,
             fn_hb_ot_layout_language_get_feature_tags,
             (*mut hb_face_t, hb_tag_t, c_uint, c_uint, c_uint, *mut c_uint, *mut hb_tag_t));

/* C 87-114: the 28 `#define hb_<name> fn_hb_<name>` renames.  An object-like `#define` has no
   Rust counterpart, so - as `femacs/src/lcms.rs` 42-45 and `femacs/src/gnutls.rs` 1122-1137
   record for the same class - the body below spells C's *preprocessed* text per arm at every call
   site: `fn_<func> (…)` under `have_ntgui` and the plain name otherwise.  The list is C's own
   (C 87-114), name for name: hb_unicode_funcs_create, hb_unicode_funcs_get_default,
   hb_unicode_funcs_set_combining_class_func, hb_unicode_funcs_set_general_category_func,
   hb_unicode_funcs_set_mirroring_func, hb_buffer_create, hb_buffer_set_unicode_funcs,
   hb_buffer_clear_contents, hb_buffer_pre_allocate, hb_buffer_add, hb_buffer_set_content_type,
   hb_buffer_set_cluster_level, hb_buffer_set_direction, hb_buffer_set_language,
   hb_language_from_string, hb_buffer_guess_segment_properties, hb_shape_full,
   hb_buffer_get_length, hb_buffer_get_direction, hb_buffer_reverse_clusters,
   hb_buffer_get_glyph_infos, hb_buffer_get_glyph_positions, hb_tag_to_string, hb_font_get_face,
   hb_ot_layout_table_get_script_tags, hb_ot_layout_table_get_feature_tags,
   hb_ot_layout_script_get_language_tags, hb_ot_layout_language_get_feature_tags.  */

/* This function is called from syms_of_w32uniscribe_for_pdumper to
   initialize the above function pointers.  */
/// C 116-150: `hbfont_init_w32_funcs`.  C's `HMODULE library` is the platform's module handle, a
/// pointer at the ABI (PORTING.md 0.2; the spelling `femacs/src/include/w32common.rs` 137-140
/// uses for this function's prototype).
#[cfg (all (feature = "harfbuzz", feature = "have_ntgui"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hbfont_init_w32_funcs (library: *mut c_void) -> bool
{
  LOAD_DLL_FN! (library, hb_unicode_funcs_create, W32_PFN_hb_unicode_funcs_create,
                fn_hb_unicode_funcs_create, c"hb_unicode_funcs_create");
  LOAD_DLL_FN! (library, hb_unicode_funcs_get_default, W32_PFN_hb_unicode_funcs_get_default,
                fn_hb_unicode_funcs_get_default, c"hb_unicode_funcs_get_default");
  LOAD_DLL_FN! (library, hb_unicode_funcs_set_combining_class_func,
                W32_PFN_hb_unicode_funcs_set_combining_class_func,
                fn_hb_unicode_funcs_set_combining_class_func,
                c"hb_unicode_funcs_set_combining_class_func");
  LOAD_DLL_FN! (library, hb_unicode_funcs_set_general_category_func,
                W32_PFN_hb_unicode_funcs_set_general_category_func,
                fn_hb_unicode_funcs_set_general_category_func,
                c"hb_unicode_funcs_set_general_category_func");
  LOAD_DLL_FN! (library, hb_unicode_funcs_set_mirroring_func,
                W32_PFN_hb_unicode_funcs_set_mirroring_func,
                fn_hb_unicode_funcs_set_mirroring_func,
                c"hb_unicode_funcs_set_mirroring_func");
  LOAD_DLL_FN! (library, hb_buffer_create, W32_PFN_hb_buffer_create, fn_hb_buffer_create,
                c"hb_buffer_create");
  LOAD_DLL_FN! (library, hb_buffer_set_unicode_funcs, W32_PFN_hb_buffer_set_unicode_funcs,
                fn_hb_buffer_set_unicode_funcs, c"hb_buffer_set_unicode_funcs");
  LOAD_DLL_FN! (library, hb_buffer_clear_contents, W32_PFN_hb_buffer_clear_contents,
                fn_hb_buffer_clear_contents, c"hb_buffer_clear_contents");
  LOAD_DLL_FN! (library, hb_buffer_pre_allocate, W32_PFN_hb_buffer_pre_allocate,
                fn_hb_buffer_pre_allocate, c"hb_buffer_pre_allocate");
  LOAD_DLL_FN! (library, hb_buffer_add, W32_PFN_hb_buffer_add, fn_hb_buffer_add,
                c"hb_buffer_add");
  LOAD_DLL_FN! (library, hb_buffer_set_content_type, W32_PFN_hb_buffer_set_content_type,
                fn_hb_buffer_set_content_type, c"hb_buffer_set_content_type");
  LOAD_DLL_FN! (library, hb_buffer_set_cluster_level, W32_PFN_hb_buffer_set_cluster_level,
                fn_hb_buffer_set_cluster_level, c"hb_buffer_set_cluster_level");
  LOAD_DLL_FN! (library, hb_buffer_set_direction, W32_PFN_hb_buffer_set_direction,
                fn_hb_buffer_set_direction, c"hb_buffer_set_direction");
  LOAD_DLL_FN! (library, hb_buffer_set_language, W32_PFN_hb_buffer_set_language,
                fn_hb_buffer_set_language, c"hb_buffer_set_language");
  LOAD_DLL_FN! (library, hb_language_from_string, W32_PFN_hb_language_from_string,
                fn_hb_language_from_string, c"hb_language_from_string");
  LOAD_DLL_FN! (library, hb_buffer_guess_segment_properties,
                W32_PFN_hb_buffer_guess_segment_properties, fn_hb_buffer_guess_segment_properties,
                c"hb_buffer_guess_segment_properties");
  LOAD_DLL_FN! (library, hb_shape_full, W32_PFN_hb_shape_full, fn_hb_shape_full,
                c"hb_shape_full");
  LOAD_DLL_FN! (library, hb_buffer_get_length, W32_PFN_hb_buffer_get_length,
                fn_hb_buffer_get_length, c"hb_buffer_get_length");
  LOAD_DLL_FN! (library, hb_buffer_get_direction, W32_PFN_hb_buffer_get_direction,
                fn_hb_buffer_get_direction, c"hb_buffer_get_direction");
  LOAD_DLL_FN! (library, hb_buffer_reverse_clusters, W32_PFN_hb_buffer_reverse_clusters,
                fn_hb_buffer_reverse_clusters, c"hb_buffer_reverse_clusters");
  LOAD_DLL_FN! (library, hb_buffer_get_glyph_infos, W32_PFN_hb_buffer_get_glyph_infos,
                fn_hb_buffer_get_glyph_infos, c"hb_buffer_get_glyph_infos");
  LOAD_DLL_FN! (library, hb_buffer_get_glyph_positions, W32_PFN_hb_buffer_get_glyph_positions,
                fn_hb_buffer_get_glyph_positions, c"hb_buffer_get_glyph_positions");
  LOAD_DLL_FN! (library, hb_tag_to_string, W32_PFN_hb_tag_to_string, fn_hb_tag_to_string,
                c"hb_tag_to_string");
  LOAD_DLL_FN! (library, hb_font_get_face, W32_PFN_hb_font_get_face, fn_hb_font_get_face,
                c"hb_font_get_face");
  LOAD_DLL_FN! (library, hb_ot_layout_table_get_script_tags,
                W32_PFN_hb_ot_layout_table_get_script_tags,
                fn_hb_ot_layout_table_get_script_tags, c"hb_ot_layout_table_get_script_tags");
  LOAD_DLL_FN! (library, hb_ot_layout_table_get_feature_tags,
                W32_PFN_hb_ot_layout_table_get_feature_tags,
                fn_hb_ot_layout_table_get_feature_tags, c"hb_ot_layout_table_get_feature_tags");
  LOAD_DLL_FN! (library, hb_ot_layout_script_get_language_tags,
                W32_PFN_hb_ot_layout_script_get_language_tags,
                fn_hb_ot_layout_script_get_language_tags,
                c"hb_ot_layout_script_get_language_tags");
  LOAD_DLL_FN! (library, hb_ot_layout_language_get_feature_tags,
                W32_PFN_hb_ot_layout_language_get_feature_tags,
                fn_hb_ot_layout_language_get_feature_tags,
                c"hb_ot_layout_language_get_feature_tags");
  return true;
}

/* C 153-214: `hbfont_otf_features` - the OTF feature table `otf_capability` reports.  Every
   `hb_*` call below is a rename site (C 87-114), so each is spelled per arm (see the note above
   C 116).  */

#[cfg (feature = "harfbuzz")]
unsafe fn hbfont_otf_features (face: *mut hb_face_t, table_tag: hb_tag_t) -> Lisp_Object
{
  let mut language_tags: *mut hb_tag_t = std::ptr::null_mut ();
  let mut feature_tags: *mut hb_tag_t = std::ptr::null_mut ();
  let mut buf: [c_char; 4] = [0; 4];
  #[cfg (not (feature = "have_ntgui"))]
  let mut script_count: c_uint
    = hb_ot_layout_table_get_script_tags (face, table_tag, 0, std::ptr::null_mut (),
					  std::ptr::null_mut ());
  #[cfg (feature = "have_ntgui")]
  let mut script_count: c_uint
    = (fn_hb_ot_layout_table_get_script_tags.unwrap ()) (face, table_tag, 0,
							 std::ptr::null_mut (),
							 std::ptr::null_mut ());
  let mut script_tags: *mut hb_tag_t
    = xnmalloc (script_count as isize, std::mem::size_of::<hb_tag_t> () as isize)
      as *mut hb_tag_t;
  #[cfg (not (feature = "have_ntgui"))]
  {
    hb_ot_layout_table_get_script_tags (face, table_tag, 0, &raw mut script_count, script_tags);
  }
  #[cfg (feature = "have_ntgui")]
  {
    (fn_hb_ot_layout_table_get_script_tags.unwrap ()) (face, table_tag, 0,
						       &raw mut script_count, script_tags);
  }
  let mut scripts: Lisp_Object = Qnil ();
  let mut i: c_int = script_count as c_int - 1;
  while i >= 0
    {
      #[cfg (not (feature = "have_ntgui"))]
      let mut language_count: c_uint
	= hb_ot_layout_script_get_language_tags (face, table_tag, i as c_uint, 0,
						 std::ptr::null_mut (),
						 std::ptr::null_mut ());
      #[cfg (feature = "have_ntgui")]
      let mut language_count: c_uint
	= (fn_hb_ot_layout_script_get_language_tags.unwrap ()) (face, table_tag, i as c_uint, 0,
							       std::ptr::null_mut (),
							       std::ptr::null_mut ());
      language_tags = xnrealloc (language_tags as *mut c_void, language_count as ptrdiff_t,
				 std::mem::size_of::<hb_tag_t> () as ptrdiff_t)
	as *mut hb_tag_t;
      #[cfg (not (feature = "have_ntgui"))]
      {
	hb_ot_layout_script_get_language_tags (face, table_tag, i as c_uint, 0,
					       &raw mut language_count, language_tags);
      }
      #[cfg (feature = "have_ntgui")]
      {
	(fn_hb_ot_layout_script_get_language_tags.unwrap ())
	  (face, table_tag, i as c_uint, 0, &raw mut language_count, language_tags);
      }
      let mut langsyses: Lisp_Object = Qnil ();
      let mut j: c_int = language_count as c_int - 1;
      while j >= -1
	{
	  let language_index: c_uint
	    = if j >= 0 { j as c_uint } else { HB_OT_LAYOUT_DEFAULT_LANGUAGE_INDEX };
	  #[cfg (not (feature = "have_ntgui"))]
	  let mut feature_count: c_uint
	    = hb_ot_layout_language_get_feature_tags (face, table_tag, i as c_uint,
						      language_index, 0, std::ptr::null_mut (),
						      std::ptr::null_mut ());
	  #[cfg (feature = "have_ntgui")]
	  let mut feature_count: c_uint
	    = (fn_hb_ot_layout_language_get_feature_tags.unwrap ())
	      (face, table_tag, i as c_uint, language_index, 0, std::ptr::null_mut (),
	       std::ptr::null_mut ());
	  /* C's `for (…; j--)` runs its increment on `continue`, which a Rust `while` does not;
	     the port writes it at the site (the `marker.rs` shape for a C `for` with `continue`).  */
	  if feature_count == 0
	    { j -= 1; continue; }
	  feature_tags = xnrealloc (feature_tags as *mut c_void, feature_count as ptrdiff_t,
				    std::mem::size_of::<hb_tag_t> () as ptrdiff_t)
	    as *mut hb_tag_t;
	  #[cfg (not (feature = "have_ntgui"))]
	  {
	    hb_ot_layout_language_get_feature_tags (face, table_tag, i as c_uint, language_index,
						    0, &raw mut feature_count, feature_tags);
	  }
	  #[cfg (feature = "have_ntgui")]
	  {
	    (fn_hb_ot_layout_language_get_feature_tags.unwrap ())
	      (face, table_tag, i as c_uint, language_index, 0, &raw mut feature_count,
	       feature_tags);
	  }
	  let mut features: Lisp_Object = Qnil ();
	  let mut k: c_int = feature_count as c_int - 1;
	  while k >= 0
	    {
	      #[cfg (not (feature = "have_ntgui"))]
	      {
		hb_tag_to_string (*feature_tags.add (k as usize), buf.as_mut_ptr ());
	      }
	      #[cfg (feature = "have_ntgui")]
	      {
		(fn_hb_tag_to_string.unwrap ()) (*feature_tags.add (k as usize),
						 buf.as_mut_ptr ());
	      }
	      features = Fcons (font_intern_prop (buf.as_ptr (), 4, true), features);
	      k -= 1;
	    }

	  let mut sym: Lisp_Object = Qnil ();
	  if j >= 0
	    {
	      #[cfg (not (feature = "have_ntgui"))]
	      {
		hb_tag_to_string (*language_tags.add (j as usize), buf.as_mut_ptr ());
	      }
	      #[cfg (feature = "have_ntgui")]
	      {
		(fn_hb_tag_to_string.unwrap ()) (*language_tags.add (j as usize),
						 buf.as_mut_ptr ());
	      }
	      sym = font_intern_prop (buf.as_ptr (), 4, true);
	    }
	  langsyses = Fcons (Fcons (sym, features), langsyses);
	  j -= 1;
	}

      #[cfg (not (feature = "have_ntgui"))]
      {
	hb_tag_to_string (*script_tags.add (i as usize), buf.as_mut_ptr ());
      }
      #[cfg (feature = "have_ntgui")]
      {
	(fn_hb_tag_to_string.unwrap ()) (*script_tags.add (i as usize), buf.as_mut_ptr ());
      }
      scripts = Fcons (Fcons (font_intern_prop (buf.as_ptr (), 4, true), langsyses), scripts);
      i -= 1;
    }
  xfree (feature_tags as *mut c_void);
  xfree (language_tags as *mut c_void);
  xfree (script_tags as *mut c_void);

  return scripts;
}

/// C 216-238: `hbfont_otf_capability` - the driver's `otf_capability`.
#[cfg (feature = "harfbuzz")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hbfont_otf_capability (font: *mut font) -> Lisp_Object
{
  /* C 219's `double position_unit;` is uninitialised in C and filled by `begin_hb_font`
     (PORTING.md 1.5/0.1-13).  */
  let mut position_unit: c_double = 0.0;
  let begin_hb_font = (*((*font).driver as *const font_driver)).begin_hb_font;
  let hb_font: *mut hb_font_t
    = if begin_hb_font.is_some ()
        { begin_hb_font.unwrap () (font, &raw mut position_unit) as *mut hb_font_t }
      else
        { std::ptr::null_mut () };
  if hb_font.is_null ()
    { return Qnil (); }

  let mut gsub_gpos: Lisp_Object = Fcons (Qnil (), Qnil ());
  #[cfg (not (feature = "have_ntgui"))]
  let face: *mut hb_face_t = hb_font_get_face (hb_font);
  #[cfg (feature = "have_ntgui")]
  let face: *mut hb_face_t = (fn_hb_font_get_face.unwrap ()) (hb_font);
  #[cfg (not (feature = "have_ntgui"))]
  if hb_ot_layout_table_get_feature_tags (face, HB_OT_TAG_GSUB, 0, std::ptr::null_mut (),
					  std::ptr::null_mut ()) != 0
    { XSETCAR (gsub_gpos, hbfont_otf_features (face, HB_OT_TAG_GSUB)); }
  #[cfg (feature = "have_ntgui")]
  if (fn_hb_ot_layout_table_get_feature_tags.unwrap ()) (face, HB_OT_TAG_GSUB, 0,
							 std::ptr::null_mut (),
							 std::ptr::null_mut ()) != 0
    { XSETCAR (gsub_gpos, hbfont_otf_features (face, HB_OT_TAG_GSUB)); }
  #[cfg (not (feature = "have_ntgui"))]
  if hb_ot_layout_table_get_feature_tags (face, HB_OT_TAG_GPOS, 0, std::ptr::null_mut (),
					  std::ptr::null_mut ()) != 0
    { XSETCDR (gsub_gpos, hbfont_otf_features (face, HB_OT_TAG_GPOS)); }
  #[cfg (feature = "have_ntgui")]
  if (fn_hb_ot_layout_table_get_feature_tags.unwrap ()) (face, HB_OT_TAG_GPOS, 0,
							 std::ptr::null_mut (),
							 std::ptr::null_mut ()) != 0
    { XSETCDR (gsub_gpos, hbfont_otf_features (face, HB_OT_TAG_GPOS)); }

  let end_hb_font = (*((*font).driver as *const font_driver)).end_hb_font;
  if end_hb_font.is_some ()
    { end_hb_font.unwrap () (font, hb_font as *mut c_void); }

  return gsub_gpos;
}

/* Support functions for HarfBuzz shaper.  */

/// C 242: `static bool combining_class_loaded = false;`
#[cfg (feature = "harfbuzz")]
static mut combining_class_loaded: bool = false;
/// C 243: `static Lisp_Object canonical_combining_class_table;` - C's tentative definition is the
/// zeroed `Lisp_Object`, i.e. `Qnil` (PORTING.md 1.5; the `gfilenotify.rs` 190-193 shape).
#[cfg (feature = "harfbuzz")]
static mut canonical_combining_class_table: Lisp_Object = lisp_h_Qnil ();

/// C 245-265: `uni_combining`.  C's `static hb_unicode_combining_class_t uni_combining (…)` is a
/// function whose address HarfBuzz takes, so it is an `unsafe extern "C" fn` item (PORTING.md
/// 1.4).
#[cfg (feature = "harfbuzz")]
unsafe extern "C" fn uni_combining (funcs: *mut hb_unicode_funcs_t, ch: hb_codepoint_t,
				    user_data: *mut c_void) -> hb_unicode_combining_class_t
{
  /* Load the Unicode table first time it is needed.  */
  if !combining_class_loaded
    {
      canonical_combining_class_table = uniprop_table (Qcanonical_combining_class ());
      if NILP (canonical_combining_class_table)
	{ emacs_abort (); }
      staticpro (&raw mut canonical_combining_class_table);
      combining_class_loaded = true;
    }

  let combining: Lisp_Object = get_unicode_property (canonical_combining_class_table,
						     ch as c_int);
  if FIXNUMP (combining)
    { return XFIXNUM (combining) as hb_unicode_combining_class_t; }

  return HB_UNICODE_COMBINING_CLASS_NOT_REORDERED;
}

/// C 267-342: `uni_general`.  C's `switch (XFIXNUM (category))` over the `unicode_category_t`
/// members becomes a `match` whose arms keep C's order and C's `return`
/// (PORTING.md 2.2/`## Deviations` 3).
#[cfg (feature = "harfbuzz")]
unsafe extern "C" fn uni_general (funcs: *mut hb_unicode_funcs_t, ch: hb_codepoint_t,
				  user_data: *mut c_void) -> hb_unicode_general_category_t
{
  let category: Lisp_Object = CHAR_TABLE_REF (Vunicode_category_table, ch as isize);

  if INTEGERP (category)
    {
      match XFIXNUM (category)
	{
	  cat if cat == UNICODE_CATEGORY_Cc as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_CONTROL; }
	  cat if cat == UNICODE_CATEGORY_Cf as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_FORMAT; }
	  cat if cat == UNICODE_CATEGORY_Cn as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_UNASSIGNED; }
	  cat if cat == UNICODE_CATEGORY_Co as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_PRIVATE_USE; }
	  cat if cat == UNICODE_CATEGORY_Cs as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_SURROGATE; }
	  cat if cat == UNICODE_CATEGORY_Ll as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_LOWERCASE_LETTER; }
	  cat if cat == UNICODE_CATEGORY_Lm as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_MODIFIER_LETTER; }
	  cat if cat == UNICODE_CATEGORY_Lo as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_OTHER_LETTER; }
	  cat if cat == UNICODE_CATEGORY_Lt as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_TITLECASE_LETTER; }
	  cat if cat == UNICODE_CATEGORY_Lu as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_UPPERCASE_LETTER; }
	  cat if cat == UNICODE_CATEGORY_Mc as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_SPACING_MARK; }
	  cat if cat == UNICODE_CATEGORY_Me as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_ENCLOSING_MARK; }
	  cat if cat == UNICODE_CATEGORY_Mn as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_NON_SPACING_MARK; }
	  cat if cat == UNICODE_CATEGORY_Nd as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_DECIMAL_NUMBER; }
	  cat if cat == UNICODE_CATEGORY_Nl as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_LETTER_NUMBER; }
	  cat if cat == UNICODE_CATEGORY_No as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_OTHER_NUMBER; }
	  cat if cat == UNICODE_CATEGORY_Pc as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_CONNECT_PUNCTUATION; }
	  cat if cat == UNICODE_CATEGORY_Pd as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_DASH_PUNCTUATION; }
	  cat if cat == UNICODE_CATEGORY_Pe as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_CLOSE_PUNCTUATION; }
	  cat if cat == UNICODE_CATEGORY_Pf as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_FINAL_PUNCTUATION; }
	  cat if cat == UNICODE_CATEGORY_Pi as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_INITIAL_PUNCTUATION; }
	  cat if cat == UNICODE_CATEGORY_Po as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_OTHER_PUNCTUATION; }
	  cat if cat == UNICODE_CATEGORY_Ps as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_OPEN_PUNCTUATION; }
	  cat if cat == UNICODE_CATEGORY_Sc as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_CURRENCY_SYMBOL; }
	  cat if cat == UNICODE_CATEGORY_Sk as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_MODIFIER_SYMBOL; }
	  cat if cat == UNICODE_CATEGORY_Sm as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_MATH_SYMBOL; }
	  cat if cat == UNICODE_CATEGORY_So as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_OTHER_SYMBOL; }
	  cat if cat == UNICODE_CATEGORY_Zl as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_LINE_SEPARATOR; }
	  cat if cat == UNICODE_CATEGORY_Zp as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_PARAGRAPH_SEPARATOR; }
	  cat if cat == UNICODE_CATEGORY_Zs as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_SPACE_SEPARATOR; }
	  cat if cat == UNICODE_CATEGORY_UNKNOWN as EMACS_INT =>
	    { return HB_UNICODE_GENERAL_CATEGORY_UNASSIGNED; }
	  _ => {}
	}
    }

  return HB_UNICODE_GENERAL_CATEGORY_UNASSIGNED;
}

/// C 344-348: `uni_mirroring`.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" fn uni_mirroring (funcs: *mut hb_unicode_funcs_t, ch: hb_codepoint_t,
				    user_data: *mut c_void) -> hb_codepoint_t
{
  return bidi_mirror_char (ch as c_int) as hb_codepoint_t;
}

/// C 350-374: `get_hb_unicode_funcs`.
#[cfg (feature = "harfbuzz")]
unsafe fn get_hb_unicode_funcs () -> *mut hb_unicode_funcs_t
{
  /* Subclass HarfBuzz's default Unicode functions and override functions that
   * use data Emacs can provide. This way changing Emacs data is reflected in
   * the shaped output. */
  #[cfg (not (feature = "have_ntgui"))]
  let funcs: *mut hb_unicode_funcs_t
    = hb_unicode_funcs_create (hb_unicode_funcs_get_default ());
  #[cfg (feature = "have_ntgui")]
  let funcs: *mut hb_unicode_funcs_t
    = (fn_hb_unicode_funcs_create.unwrap ()) ((fn_hb_unicode_funcs_get_default.unwrap ()) ());

  #[cfg (not (feature = "have_ntgui"))]
  {
    hb_unicode_funcs_set_combining_class_func (funcs, Some (uni_combining),
					       std::ptr::null_mut (), None);
  }
  #[cfg (feature = "have_ntgui")]
  {
    (fn_hb_unicode_funcs_set_combining_class_func.unwrap ()) (funcs, Some (uni_combining),
							      std::ptr::null_mut (), None);
  }
  #[cfg (not (feature = "have_ntgui"))]
  {
    hb_unicode_funcs_set_general_category_func (funcs, Some (uni_general),
						std::ptr::null_mut (), None);
  }
  #[cfg (feature = "have_ntgui")]
  {
    (fn_hb_unicode_funcs_set_general_category_func.unwrap ()) (funcs, Some (uni_general),
							       std::ptr::null_mut (), None);
  }
  #[cfg (not (feature = "have_ntgui"))]
  {
    hb_unicode_funcs_set_mirroring_func (funcs, Some (uni_mirroring), std::ptr::null_mut (),
					 None);
  }
  #[cfg (feature = "have_ntgui")]
  {
    (fn_hb_unicode_funcs_set_mirroring_func.unwrap ()) (funcs, Some (uni_mirroring),
							std::ptr::null_mut (), None);
  }

  /* Use default implementation for Unicode composition/decomposition.
     We might want to revisit this later.
  hb_unicode_funcs_set_compose_func (funcs, uni_compose, NULL, NULL);
  hb_unicode_funcs_set_decompose_func (funcs, uni_decompose, NULL, NULL);
  */

  /* Emacs own script mapping for characters differs from Unicode, so we want
   * to keep the default HarfBuzz's implementation here.
  hb_unicode_funcs_set_script_func (funcs, uni_script, NULL, NULL);
  */

  return funcs;
}

/* HarfBuzz implementation of shape for font backend.

   Shape text in LGSTRING.  See the docstring of
   'composition-get-gstring' for the format of LGSTRING.  If the
   (N+1)th element of LGSTRING is nil, input of shaping is from the
   1st to (N)th elements.  In each input glyph, FROM, TO, CHAR, and
   CODE are already set, but FROM and TO need adjustments according
   to the glyphs produced by the shaping function.
   DIRECTION is either L2R or R2L, or nil if unknown.  During
   redisplay, this comes from applying the UBA, is passed from
   composition_reseat_it, and is used by the HarfBuzz shaper.

   This function updates all fields of the input glyphs.  If the
   output glyphs (M) are more than the input glyphs (N), (N+1)th
   through (M)th elements of LGSTRING are updated possibly by making
   a new glyph object and storing it in LGSTRING.  If (M) is greater
   than the length of LGSTRING, nil should be returned.  In that case,
   this function is called again with a larger LGSTRING.  */

/// C 394-606: `hbfont_shape` - the driver's `shape`.
#[cfg (feature = "harfbuzz")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hbfont_shape (lgstring: Lisp_Object, direction: Lisp_Object)
  -> Lisp_Object
{
  let font: *mut font = CHECK_FONT_GET_OBJECT (LGSTRING_FONT (lgstring));
  let mut glyph_len: ptrdiff_t = 0;
  let mut text_len: ptrdiff_t = LGSTRING_GLYPH_LEN (lgstring);
  let mut i: ptrdiff_t;

  /* C 406: a function-scope `static`.  */
  static mut hb_buffer: *mut hb_buffer_t = std::ptr::null_mut ();
  if hb_buffer.is_null ()
    {
      #[cfg (not (feature = "have_ntgui"))]
      {
	hb_buffer = hb_buffer_create ();
      }
      #[cfg (feature = "have_ntgui")]
      {
	hb_buffer = (fn_hb_buffer_create.unwrap ()) ();
      }
      let ufuncs: *mut hb_unicode_funcs_t = get_hb_unicode_funcs ();
      #[cfg (not (feature = "have_ntgui"))]
      {
	hb_buffer_set_unicode_funcs (hb_buffer, ufuncs);
      }
      #[cfg (feature = "have_ntgui")]
      {
	(fn_hb_buffer_set_unicode_funcs.unwrap ()) (hb_buffer, ufuncs);
      }
    }

  #[cfg (not (feature = "have_ntgui"))]
  {
    hb_buffer_clear_contents (hb_buffer);
  }
  #[cfg (feature = "have_ntgui")]
  {
    (fn_hb_buffer_clear_contents.unwrap ()) (hb_buffer);
  }
  #[cfg (not (feature = "have_ntgui"))]
  {
    hb_buffer_pre_allocate (hb_buffer, text_len as c_uint);
  }
  #[cfg (feature = "have_ntgui")]
  {
    (fn_hb_buffer_pre_allocate.unwrap ()) (hb_buffer, text_len as c_uint);
  }

  /* Copy the characters in their original logical order, so we can
     assign them to glyphs correctly after shaping.  */
  /* C 419's bare `alloca` is the port's stack pool (PORTING.md P-05/3.2; the `editfns.rs`
     2477 shape).  */
  let mut chars: *mut c_int
    = sa_stack_alloc (text_len as usize * std::mem::size_of::<c_int> ()) as *mut c_int;
  i = 0;
  while i < text_len
    {
      let g: Lisp_Object = LGSTRING_GLYPH (lgstring, i);
      let c: c_int;

      if NILP (g)
	{ break; }
      c = LGLYPH_CHAR (g) as c_int;
      #[cfg (not (feature = "have_ntgui"))]
      {
	hb_buffer_add (hb_buffer, c as hb_codepoint_t, i as c_uint);
      }
      #[cfg (feature = "have_ntgui")]
      {
	(fn_hb_buffer_add.unwrap ()) (hb_buffer, c as hb_codepoint_t, i as c_uint);
      }
      *chars.add (i as usize) = c;
      i += 1;
    }

  text_len = i;
  if text_len == 0
    { return Qnil (); }

  #[cfg (not (feature = "have_ntgui"))]
  {
    hb_buffer_set_content_type (hb_buffer, HB_BUFFER_CONTENT_TYPE_UNICODE);
  }
  #[cfg (feature = "have_ntgui")]
  {
    (fn_hb_buffer_set_content_type.unwrap ()) (hb_buffer, HB_BUFFER_CONTENT_TYPE_UNICODE);
  }
  #[cfg (not (feature = "have_ntgui"))]
  {
    hb_buffer_set_cluster_level (hb_buffer, HB_BUFFER_CLUSTER_LEVEL_MONOTONE_GRAPHEMES);
  }
  #[cfg (feature = "have_ntgui")]
  {
    (fn_hb_buffer_set_cluster_level.unwrap ()) (hb_buffer,
						HB_BUFFER_CLUSTER_LEVEL_MONOTONE_GRAPHEMES);
  }

  /* If the caller didn't provide a meaningful DIRECTION, let HarfBuzz
     guess it. */
  if !NILP (direction)
      /* If they bind bidi-display-reordering to nil, the DIRECTION
	 they provide is meaningless, and we should let HarfBuzz guess
	 the real direction.  */
      && !NILP (BVAR! ((current_buffer! ()), bidi_display_reordering_))
    {
      let mut dir: hb_direction_t = HB_DIRECTION_LTR;
      if EQ (direction, QL2R ())
	{ dir = HB_DIRECTION_LTR; }
      else if EQ (direction, QR2L ())
	{ dir = HB_DIRECTION_RTL; }
      #[cfg (not (feature = "have_ntgui"))]
      {
	hb_buffer_set_direction (hb_buffer, dir);
      }
      #[cfg (feature = "have_ntgui")]
      {
	(fn_hb_buffer_set_direction.unwrap ()) (hb_buffer, dir);
      }
    }

  /* Leave the script determination to HarfBuzz, until Emacs has a
     better idea of the script of LGSTRING.  FIXME.  */
  /* C 458-460: `#if 0` ... `hb_buffer_set_script (hb_buffer, XXX);` ... `#endif` - dead text in
     every configuration, carried as a comment (P-86 ①).  */

  /* FIXME: This can only handle the single global language, which
     normally comes from the locale.  In addition, if
     current-iso639-language is a list, we arbitrarily use the first
     one.  We should instead have a notion of the language of the text
     being shaped.  */
  let mut lang: Lisp_Object = Vcurrent_iso639_language;
  if CONSP (Vcurrent_iso639_language)
    { lang = XCAR (Vcurrent_iso639_language); }
  if SYMBOLP (lang)
    {
      let lang_str: Lisp_Object = SYMBOL_NAME (lang);
      #[cfg (not (feature = "have_ntgui"))]
      {
	hb_buffer_set_language (hb_buffer,
				hb_language_from_string (SSDATA (lang_str),
							 SBYTES (lang_str) as c_int));
      }
      #[cfg (feature = "have_ntgui")]
      {
	(fn_hb_buffer_set_language.unwrap ())
	  (hb_buffer, (fn_hb_language_from_string.unwrap ()) (SSDATA (lang_str),
							      SBYTES (lang_str) as c_int));
      }
    }

  /* Guess the default properties for when they cannot be determined above.

     FIXME: maybe drop this guessing once script and language handling
     is fixed above; but then will need to guess the direction by
     ourselves, perhaps by looking at the characters using
     bidi_get_type or somesuch.  */
  #[cfg (not (feature = "have_ntgui"))]
  {
    hb_buffer_guess_segment_properties (hb_buffer);
  }
  #[cfg (feature = "have_ntgui")]
  {
    (fn_hb_buffer_guess_segment_properties.unwrap ()) (hb_buffer);
  }

  let mut position_unit: c_double = 0.0;
  let begin_hb_font = (*((*font).driver as *const font_driver)).begin_hb_font;
  let hb_font: *mut hb_font_t
    = if begin_hb_font.is_some ()
        { begin_hb_font.unwrap () (font, &raw mut position_unit) as *mut hb_font_t }
      else
        { std::ptr::null_mut () };
  if hb_font.is_null ()
    { return make_fixnum (0); }

  #[cfg (not (feature = "have_ntgui"))]
  let success: hb_bool_t
    = hb_shape_full (hb_font, hb_buffer, std::ptr::null (), 0, std::ptr::null ());
  #[cfg (feature = "have_ntgui")]
  let success: hb_bool_t
    = (fn_hb_shape_full.unwrap ()) (hb_font, hb_buffer, std::ptr::null (), 0,
				    std::ptr::null ());
  let end_hb_font = (*((*font).driver as *const font_driver)).end_hb_font;
  if end_hb_font.is_some ()
    { end_hb_font.unwrap () (font, hb_font as *mut c_void); }
  if success == 0
    { return Qnil (); }

  /* `unsigned int` to `ptrdiff_t` is C's own implicit conversion.  */
  #[cfg (not (feature = "have_ntgui"))]
  {
    glyph_len = hb_buffer_get_length (hb_buffer) as ptrdiff_t;
  }
  #[cfg (feature = "have_ntgui")]
  {
    glyph_len = (fn_hb_buffer_get_length.unwrap ()) (hb_buffer) as ptrdiff_t;
  }
  if glyph_len > LGSTRING_GLYPH_LEN (lgstring)
    { return Qnil (); }

  /* We need the clusters in logical order.  */
  let mut buf_reversed: bool = false;
  #[cfg (not (feature = "have_ntgui"))]
  if HB_DIRECTION_IS_BACKWARD! (hb_buffer_get_direction (hb_buffer))
    {
      buf_reversed = true;
      hb_buffer_reverse_clusters (hb_buffer);
    }
  #[cfg (feature = "have_ntgui")]
  if HB_DIRECTION_IS_BACKWARD! ((fn_hb_buffer_get_direction.unwrap ()) (hb_buffer))
    {
      buf_reversed = true;
      (fn_hb_buffer_reverse_clusters.unwrap ()) (hb_buffer);
    }
  /* C 401-402 declares `info`/`pos` uninitialised and fills them at C 511-512; a Rust local has
     no uninitialised form, so each is bound where C assigns it (PORTING.md 0.1-13).  */
  #[cfg (not (feature = "have_ntgui"))]
  let info: *mut hb_glyph_info_t = hb_buffer_get_glyph_infos (hb_buffer, std::ptr::null_mut ());
  #[cfg (feature = "have_ntgui")]
  let info: *mut hb_glyph_info_t
    = (fn_hb_buffer_get_glyph_infos.unwrap ()) (hb_buffer, std::ptr::null_mut ());
  #[cfg (not (feature = "have_ntgui"))]
  let pos: *mut hb_glyph_position_t
    = hb_buffer_get_glyph_positions (hb_buffer, std::ptr::null_mut ());
  #[cfg (feature = "have_ntgui")]
  let pos: *mut hb_glyph_position_t
    = (fn_hb_buffer_get_glyph_positions.unwrap ()) (hb_buffer, std::ptr::null_mut ());
  /* C 513's `UNINIT` is lisp.h's "deliberately uninitialised" marker, which a Rust `let mut` has
     no counterpart for (PORTING.md 1.5); the port writes the zero the macro stands for.  */
  let mut from: ptrdiff_t = -1;
  let mut to: ptrdiff_t = 0;
  let mut cluster_offset: ptrdiff_t = 0;
  let incr: c_int = if buf_reversed { -1 } else { 1 };
  i = 0;
  while i < glyph_len
    {
      let mut lglyph: Lisp_Object = LGSTRING_GLYPH (lgstring, i);
      let mut metrics: font_metrics
	= font_metrics { lbearing: 0, rbearing: 0, width: 0, ascent: 0, descent: 0 };
      let mut xoff: c_int;
      let mut yoff: c_int;
      let mut wadjust: c_int;
      let mut new_lglyph: bool = false;

      if NILP (lglyph)
	{
	  new_lglyph = true;
	  lglyph = LGLYPH_NEW ();
	  LGSTRING_SET_GLYPH (lgstring, i, lglyph);
	}

      if (*info.add (i as usize)).cluster as ptrdiff_t != from
	{
	  let mut j: c_int;
	  /* Found a new cluster.  Determine its FROM and TO, and the
	     offset to the first character of the cluster.  */
	  /* FROM is the index of the first character that contributed
	     to this cluster.  */
	  from = (*info.add (i as usize)).cluster as ptrdiff_t;
	  /* TO is the index of the last character that contributed to
	     this cluster.  */
	  j = i as c_int;
	  while (j as ptrdiff_t) < glyph_len
		&& (*info.add (j as usize)).cluster as ptrdiff_t == from
	    { j += 1; }
	  to = if j as ptrdiff_t == glyph_len { text_len - 1 }
	       else { (*info.add (j as usize)).cluster as ptrdiff_t - 1 };
	  cluster_offset = 0;
	  /* For RTL buffers, HarfBuzz produces glyphs in a cluster in
	     reverse order, so we need to account for that to record
	     the correct character in each glyph.

	     Implementation note: the character codepoint recorded in
	     each glyph is not really used, except when we display the
	     glyphs in descr-text.el.  So this is just an aesthetic
	     issue.  */
	  if buf_reversed
	    { cluster_offset = to - from; }
	}

      eassume! (0 <= from);

      /* All the glyphs in a cluster have the same values of FROM and TO.  */
      LGLYPH_SET_FROM (lglyph, from);
      /* This heuristic is for when the Lisp shape-gstring function
	 substitutes known precomposed characters for decomposed
	 sequences.  E.g., hebrew.el does that.  This makes TEXT_LEN
	 be smaller than the original length of the composed character
	 sequence.  In that case, we must not alter the largest TO,
	 because the display engine must know that all the characters
	 in the original sequence were processed by the composition.
	 If we don't do this, some of the composed characters will be
	 displayed again as separate glyphs.  */
      if !(!new_lglyph && to == text_len - 1 && LGLYPH_TO (lglyph) > to)
	{ LGLYPH_SET_TO (lglyph, to); }

      /* Not every glyph in a cluster maps directly to a single
	 character; in general, N characters can yield M glyphs, where
	 M could be smaller or greater than N.  However, in many cases
	 there is a one-to-one correspondence, and it would be a pity
	 to lose that information, even if it's sometimes inaccurate.  */
      let mut char_idx: ptrdiff_t = from + cluster_offset;
      cluster_offset += incr as ptrdiff_t;
      if char_idx > to
	{ char_idx = to; }
      if char_idx < from
	{ char_idx = from; }
      LGLYPH_SET_CHAR (lglyph, *chars.add (char_idx as usize) as EMACS_INT);
      LGLYPH_SET_CODE! (lglyph, (*info.add (i as usize)).codepoint);

      let code: c_uint = (*info.add (i as usize)).codepoint;
      (*((*font).driver as *const font_driver)).text_extents.unwrap ()
	(font, &raw const code, 1, &mut metrics);
      LGLYPH_SET_WIDTH (lglyph, metrics.width as EMACS_INT);
      LGLYPH_SET_LBEARING (lglyph, metrics.lbearing as EMACS_INT);
      LGLYPH_SET_RBEARING (lglyph, metrics.rbearing as EMACS_INT);
      LGLYPH_SET_ASCENT (lglyph, metrics.ascent as EMACS_INT);
      LGLYPH_SET_DESCENT (lglyph, metrics.descent as EMACS_INT);

      xoff = lround ((*pos.add (i as usize)).x_offset as c_double * position_unit) as c_int;
      yoff = -(lround ((*pos.add (i as usize)).y_offset as c_double * position_unit) as c_int);
      wadjust = lround ((*pos.add (i as usize)).x_advance as c_double * position_unit) as c_int;
      if xoff != 0 || yoff != 0 || wadjust != metrics.width as c_int
	{
	  LGLYPH_SET_ADJUSTMENT (lglyph, CALLN! (Fvector, make_fixnum (xoff as EMACS_INT),
						 make_fixnum (yoff as EMACS_INT),
						 make_fixnum (wadjust as EMACS_INT)));
	}
      i += 1;
    }

  return make_fixnum (glyph_len);
}

/// C 608-612: `hbfont_combining_capability` - the driver's `combining_capability`.
#[cfg (feature = "harfbuzz")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hbfont_combining_capability (font: *mut font) -> Lisp_Object
{
  return Qt ();
}
