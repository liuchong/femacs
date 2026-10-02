// femacs: port of src/dispextern.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (PORTING.md P-66): it carries the *frozen configuration's* live text as it is
// needed, which is the platform-independent part of the display interface — the scalars,
// enums and the glyph datatype so far.  The rest of the header (the glyph matrix/row
// machinery, faces, images and the `HAVE_WINDOW_SYSTEM`/`HAVE_X_WINDOWS`/… branches) is
// tracked in docs/ports/include-dispextern.md with unchecked boxes; a section that is
// deliberately not mirrored yet is marked with a `// C: … (not mirrored on this target)`
// line at its boundary, and it lands when a GUI target is frozen (batch E/F).
//
// `<config.h>`'s platform branch decides the colour/rectangle/GC types: the frozen
// configuration has no X and no Android, so C takes the `#else` branch of the
// `#ifdef HAVE_X_WINDOWS` at dispextern.h 27-98, which is what this file mirrors.

/* Interface definitions for display code.

Copyright (C) 1985, 1993-1994, 1997-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const DISPEXTERN_H_INCLUDED: bool = true;

// dispextern.h 46-98: the `#else` of `#ifdef HAVE_X_WINDOWS` (P-08: the frozen target has
// no X11 and no Android).  C spells the three structs as *anonymous* typedefs; a Rust struct
// needs a name, so the port uses the typedef's name and records the deviation (P-67).

/* XColor-like struct used by non-X code.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Emacs_Color
{
  pub pixel: c_ulong,
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
}

/* Accommodate X's usage of None as a null resource ID.  */
// `#define No_Cursor (NULL)` (dispextern.h 58; the X branch's `(None)` is dead).
pub const No_Cursor: *mut c_void = std::ptr::null_mut ();

/* XRectangle-like struct used by non-X GUI code.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Emacs_Rectangle
{
  pub x: c_int,
  pub y: c_int,
  pub width: c_uint,
  pub height: c_uint,
}

/* XGCValues-like struct used by non-X GUI code.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Emacs_GC
{
  pub foreground: c_ulong,
  pub background: c_ulong,
}

/* Mask values to select foreground/background.  */
pub const GCForeground: c_int = 0x01;
pub const GCBackground: c_int = 0x02;

// dispextern.h 175: `typedef void *Emacs_Cursor;` (outside the X branch).
pub type Emacs_Cursor = *mut c_void;

/* Text cursor types.  */
// C's members have type `int`; the port keeps the enum as a type alias plus constants, which
// is how the C code uses it (`enum text_cursor_kinds cursor_type;` and `== BAR_CURSOR`).
pub type text_cursor_kinds = c_int;
pub const DEFAULT_CURSOR: text_cursor_kinds = -2;      // dispextern.h 216-225
pub const NO_CURSOR: text_cursor_kinds = -1;
pub const FILLED_BOX_CURSOR: text_cursor_kinds = 0;
pub const HOLLOW_BOX_CURSOR: text_cursor_kinds = 1;
pub const BAR_CURSOR: text_cursor_kinds = 2;
pub const HBAR_CURSOR: text_cursor_kinds = 3;

/* Values returned from coordinates_in_window.  */
pub type window_part = c_int;
pub const ON_NOTHING: window_part = 0;                 // dispextern.h 228-246
pub const ON_TEXT: window_part = 1;
pub const ON_MODE_LINE: window_part = 2;
pub const ON_VERTICAL_BORDER: window_part = 3;
pub const ON_HEADER_LINE: window_part = 4;
pub const ON_TAB_LINE: window_part = 5;
pub const ON_LEFT_FRINGE: window_part = 6;
pub const ON_RIGHT_FRINGE: window_part = 7;
pub const ON_LEFT_MARGIN: window_part = 8;
pub const ON_RIGHT_MARGIN: window_part = 9;
pub const ON_VERTICAL_SCROLL_BAR: window_part = 10;
pub const ON_HORIZONTAL_SCROLL_BAR: window_part = 11;
pub const ON_RIGHT_DIVIDER: window_part = 12;
pub const ON_BOTTOM_DIVIDER: window_part = 13;

/* The glyph datatype, used to represent characters on the display.
   It consists of a char code and a face id.  */
// C: `typedef struct { int ch; int face_id; } GLYPH;` (dispextern.h 383-385) — an anonymous
// struct typedef, named after the typedef here (P-67).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLYPH
{
  pub ch: c_int,
  pub face_id: c_int,
}

/* Return a glyph's character code.  */
#[inline(always)]
pub fn GLYPH_CHAR (glyph: GLYPH) -> c_int { glyph.ch }

/* Return a glyph's face ID.  */
#[inline(always)]
pub fn GLYPH_FACE (glyph: GLYPH) -> c_int { glyph.face_id }

/* The following are valid only if GLYPH_CODE_P (gc).  */

#[inline(always)]
pub fn GLYPH_CODE_CHAR (gc: Lisp_Object) -> c_int
{
  if CONSP (gc) { XFIXNUM (XCAR (gc)) as c_int } else { (XFIXNUM (gc) & MAX_CHAR as EMACS_INT) as c_int }
}

#[inline(always)]
pub fn GLYPH_CODE_FACE (gc: Lisp_Object) -> c_int
{
  if CONSP (gc) { XFIXNUM (XCDR (gc)) as c_int }
  else { (XFIXNUM (gc) >> CHARACTERBITS) as c_int }
}

/* The ID of the mode line highlighting face.  */
pub const GLYPH_MODE_LINE_FACE: c_int = 1;             // dispextern.h 426

/* Enumeration of glyph types.  Glyph structures contain a type field
   containing one of the enumerators defined here.  */
pub type glyph_type = c_int;
pub const CHAR_GLYPH: glyph_type = 0;                  // dispextern.h 431-452
pub const COMPOSITE_GLYPH: glyph_type = 1;
pub const GLYPHLESS_GLYPH: glyph_type = 2;
pub const IMAGE_GLYPH: glyph_type = 3;
pub const STRETCH_GLYPH: glyph_type = 4;
pub const XWIDGET_GLYPH: glyph_type = 5;

/* Structure describing how to use partial glyphs (images slicing) */
// C's four 16-bit bitfields (dispextern.h 455-461) share one 64-bit unit; the frozen
// target's clang puts `x` in bytes 0-1, `y` in 2-3, `width` in 4-5 and `height` in 6-7
// (`sizeof` 8, `alignof` 4 — measured, the probe is in the commit):
//
//     s.x = 1; s.y = 2; s.width = 3; s.height = 4;
//     bytes: 01 00 02 00 03 00 04 00
//
// PORTING.md 1.3: the storage unit is recorded and the offsets/size asserted; the bit
// numbering is not provable from `offsetof`.
// The unit is 8 bytes but only 4-aligned (clang packs the four bitfields into one 4-byte
// unit per two), so the port stores it as two `u32`s: `#[repr(C, align(4))]` cannot lower a
// `u64`'s natural alignment, and the gate asserts the C compiler's 8/4 (E0080 otherwise).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_slice
{
  pub bits: [u32; 2],
}

// x : 16 and y : 16 share `bits[0]` (bytes 0-3), width : 16 and height : 16 share `bits[1]`.
pub const GLYPH_SLICE_X_MASK: u32 = 0xffff;             // bits 0-15 of bits[0]
pub const GLYPH_SLICE_Y_SHIFT: u32 = 16;                // bits 16-31 of bits[0]
pub const GLYPH_SLICE_WIDTH_MASK: u32 = 0xffff;         // bits 0-15 of bits[1]
pub const GLYPH_SLICE_HEIGHT_SHIFT: u32 = 16;           // bits 16-31 of bits[1]

/* Enumeration of the areas in a glyph row.  */
pub type glyph_row_area = c_int;
pub const LEFT_MARGIN_AREA: glyph_row_area = 0;         // dispextern.h 881-888
pub const TEXT_AREA: glyph_row_area = 1;
pub const RIGHT_MARGIN_AREA: glyph_row_area = 2;
pub const LAST_AREA: glyph_row_area = 3;

/* Values returned from coordinates_in_window — drawing operations.  */
pub type draw_glyphs_face = c_int;                      // dispextern.h 1348-1357
pub const DRAW_NORMAL_TEXT: draw_glyphs_face = 0;
pub const DRAW_CURSOR: draw_glyphs_face = 1;
pub const DRAW_MOUSE_FACE: draw_glyphs_face = 2;
pub const DRAW_IMAGE_RAISED: draw_glyphs_face = 3;
pub const DRAW_IMAGE_SUNKEN: draw_glyphs_face = 4;

/* Indices of face attributes in Lisp face vectors.  */
pub type lface_attribute_index = c_int;                 // dispextern.h 1723-1748
pub const LFACE_FAMILY_INDEX: lface_attribute_index = 1;
pub const LFACE_FOUNDRY_INDEX: lface_attribute_index = 2;
pub const LFACE_SWIDTH_INDEX: lface_attribute_index = 3;
pub const LFACE_HEIGHT_INDEX: lface_attribute_index = 4;
pub const LFACE_WEIGHT_INDEX: lface_attribute_index = 5;
pub const LFACE_SLANT_INDEX: lface_attribute_index = 6;
pub const LFACE_UNDERLINE_INDEX: lface_attribute_index = 7;
pub const LFACE_INVERSE_INDEX: lface_attribute_index = 8;
pub const LFACE_FOREGROUND_INDEX: lface_attribute_index = 9;
pub const LFACE_BACKGROUND_INDEX: lface_attribute_index = 10;
pub const LFACE_STIPPLE_INDEX: lface_attribute_index = 11;
pub const LFACE_OVERLINE_INDEX: lface_attribute_index = 12;
pub const LFACE_STRIKE_THROUGH_INDEX: lface_attribute_index = 13;
pub const LFACE_BOX_INDEX: lface_attribute_index = 14;
pub const LFACE_FONT_INDEX: lface_attribute_index = 15;
pub const LFACE_INHERIT_INDEX: lface_attribute_index = 16;
pub const LFACE_FONTSET_INDEX: lface_attribute_index = 17;
pub const LFACE_DISTANT_FOREGROUND_INDEX: lface_attribute_index = 18;
pub const LFACE_EXTEND_INDEX: lface_attribute_index = 19;
pub const LFACE_VECTOR_SIZE: lface_attribute_index = 20;

/* Box types of faces.  */
pub type face_box_type = c_int;                         // dispextern.h 1750-1765
pub const FACE_NO_BOX: face_box_type = 0;
pub const FACE_SIMPLE_BOX: face_box_type = 1;
pub const FACE_RAISED_BOX: face_box_type = 2;
pub const FACE_SUNKEN_BOX: face_box_type = 3;

/* Underline type. */
pub type face_underline_type = c_int;                   // dispextern.h 1767-1780
pub const FACE_NO_UNDERLINE: face_underline_type = 0;
pub const FACE_UNDERLINE_SINGLE: face_underline_type = 1;
pub const FACE_UNDERLINE_DOUBLE_LINE: face_underline_type = 2;
pub const FACE_UNDERLINE_WAVE: face_underline_type = 3;
pub const FACE_UNDERLINE_DOTS: face_underline_type = 4;
pub const FACE_UNDERLINE_DASHES: face_underline_type = 5;

/* Basic face identifiers.  */
pub type face_id = c_int;                               // dispextern.h 1959-1985
pub const DEFAULT_FACE_ID: face_id = 0;
pub const MODE_LINE_ACTIVE_FACE_ID: face_id = 1;
pub const MODE_LINE_INACTIVE_FACE_ID: face_id = 2;
pub const TOOL_BAR_FACE_ID: face_id = 3;
pub const FRINGE_FACE_ID: face_id = 4;
pub const HEADER_LINE_ACTIVE_FACE_ID: face_id = 5;
pub const HEADER_LINE_INACTIVE_FACE_ID: face_id = 6;
pub const SCROLL_BAR_FACE_ID: face_id = 7;
pub const BORDER_FACE_ID: face_id = 8;
pub const CURSOR_FACE_ID: face_id = 9;
pub const MOUSE_FACE_ID: face_id = 10;
pub const MENU_FACE_ID: face_id = 11;
pub const VERTICAL_BORDER_FACE_ID: face_id = 12;
pub const WINDOW_DIVIDER_FACE_ID: face_id = 13;
pub const WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID: face_id = 14;
pub const WINDOW_DIVIDER_LAST_PIXEL_FACE_ID: face_id = 15;
pub const INTERNAL_BORDER_FACE_ID: face_id = 16;
pub const CHILD_FRAME_BORDER_FACE_ID: face_id = 17;
pub const TAB_BAR_FACE_ID: face_id = 18;
pub const TAB_LINE_ACTIVE_FACE_ID: face_id = 19;
pub const TAB_LINE_INACTIVE_FACE_ID: face_id = 20;
pub const MARGIN_FACE_ID: face_id = 21;
pub const BASIC_FACE_ID_SENTINEL: face_id = 22;

// C: dispextern.h 100-2000's remaining types and macros, and 2000-4026's `it`/fringe/
// X-widget machinery — (not mirrored on this target yet; docs/ports/include-dispextern.md).
