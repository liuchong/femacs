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

// dispextern.h 2052-2062: the predicate character.c's char_width reads a display vector's
// element through.  `FACE_ID_BITS` (562) and `MAX_FACE_ID` (1986) are the two values it
// needs; the rest of the face/glyph block they live in is still unmirrored (the boundary
// comment at the end of this file lists it).
pub const FACE_ID_BITS: c_int = 20;                                        // dispextern.h 562
pub const MAX_FACE_ID: EMACS_INT = ((1 << FACE_ID_BITS) - 1) as EMACS_INT; // dispextern.h 1986

#[inline(always)]
pub fn GLYPH_CODE_P (gc: Lisp_Object) -> bool
{
  return if CONSP (gc)
    {
      CHARACTERP (XCAR (gc)) && RANGED_FIXNUMP (0, XCDR (gc), MAX_FACE_ID)
    }
    else
    {
      RANGED_FIXNUMP (0, gc,
		      if MAX_FACE_ID < EMACS_INT_MAX >> CHARACTERBITS
			{ (MAX_FACE_ID << CHARACTERBITS) | MAX_CHAR as EMACS_INT }
			else
			{ EMACS_INT_MAX })
    };
}

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
pub const ANY_AREA: glyph_row_area = -1;                // dispextern.h 881-888
pub const LEFT_MARGIN_AREA: glyph_row_area = 0;
pub const TEXT_AREA: glyph_row_area = 1;
pub const RIGHT_MARGIN_AREA: glyph_row_area = 2;
pub const LAST_AREA: glyph_row_area = 3;

/* Values returned from coordinates_in_window — drawing operations.  */
pub type draw_glyphs_face = c_int;                      // dispextern.h 1348-1357
pub const DRAW_NORMAL_TEXT: draw_glyphs_face = 0;
pub const DRAW_INVERSE_VIDEO: draw_glyphs_face = 1;
pub const DRAW_CURSOR: draw_glyphs_face = 2;
pub const DRAW_MOUSE_FACE: draw_glyphs_face = 3;
pub const DRAW_IMAGE_RAISED: draw_glyphs_face = 4;
pub const DRAW_IMAGE_SUNKEN: draw_glyphs_face = 5;

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

// --- batch II: the position pairs, the slice and the bidi state (dispextern.h 275-370,
//     2080-2160, 2360-2366, 3048-3058) --------------------------------------------------
// P-66's order: `text_pos`/`display_pos` are the pair every display function passes around,
// `it_slice` is four Lisp_Objects, `run` is six ints, and the `bidi_*` trio is the state
// `struct it` carries.  None of them needs a platform type, so they land before the matrix
// and face structures (which need `struct frame`/`struct window`/`struct font`).

/* Starting with Emacs 20.3, characters from strings and buffers have
   both a character and a byte position associated with them.  The
   following structure holds such a pair of positions.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct text_pos
{
  /* Character position.  */
  pub charpos: ptrdiff_t,               // dispextern.h 278
  /* Corresponding byte position.  */
  pub bytepos: ptrdiff_t,
}

// BYTEPOS/CHARPOS/SET_TEXT_POS/INC_TEXT_POS arrive with the functions that use them; like
// every other place-expression macro they are spelled as functions (P-17).

/* When rendering glyphs, redisplay scans string or buffer text,
   overlay strings in that text, and does display table or control
   character translations.  The following structure captures a
   position taking all this into account.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct display_pos
{
  pub pos: text_pos,                    // dispextern.h 354
  pub overlay_string_index: ptrdiff_t,  // < 0 when `pos' is not in an overlay string
  pub string_pos: text_pos,
  pub dpvec_index: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct it_slice
{
  pub x: Lisp_Object,                   // dispextern.h 2362
  pub y: Lisp_Object,
  pub width: Lisp_Object,
  pub height: Lisp_Object,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct run
{
  pub desired_y: c_int,                 // dispextern.h 3051
  pub current_y: c_int,
  pub desired_vpos: c_int,
  pub current_vpos: c_int,
  pub height: c_int,
  pub nrows: c_int,
}

// dispextern.h 2080-2108: `typedef enum { … } bidi_type_t;`.  The first seven members are
// the only ones valid in `struct glyph`'s `bidi_type` field, which reserves three bits.
pub type bidi_type_t = c_int;
pub const UNKNOWN_BT: bidi_type_t = 0;
pub const STRONG_L: bidi_type_t = 1;
pub const STRONG_R: bidi_type_t = 2;
pub const WEAK_EN: bidi_type_t = 3;
pub const WEAK_AN: bidi_type_t = 4;
pub const WEAK_BN: bidi_type_t = 5;
pub const NEUTRAL_B: bidi_type_t = 6;
pub const STRONG_AL: bidi_type_t = 7;
pub const LRE: bidi_type_t = 8;
pub const LRO: bidi_type_t = 9;
pub const RLE: bidi_type_t = 10;
pub const RLO: bidi_type_t = 11;
pub const PDF: bidi_type_t = 12;
pub const LRI: bidi_type_t = 13;
pub const RLI: bidi_type_t = 14;
pub const FSI: bidi_type_t = 15;
pub const PDI: bidi_type_t = 16;
pub const WEAK_ES: bidi_type_t = 17;
pub const WEAK_ET: bidi_type_t = 18;
pub const WEAK_CS: bidi_type_t = 19;
pub const WEAK_NSM: bidi_type_t = 20;
pub const NEUTRAL_S: bidi_type_t = 21;
pub const NEUTRAL_WS: bidi_type_t = 22;
pub const NEUTRAL_ON: bidi_type_t = 23;
pub const BIDI_MAXDEPTH: c_int = 125;   // dispextern.h 2076 (UAX#9's max_depth)

// dispextern.h 2115-2119: the paired-bracket type (`bracket-type` property order).
pub type bidi_bracket_type_t = c_int;
pub const BIDI_BRACKET_NONE: bidi_bracket_type_t = 1;
pub const BIDI_BRACKET_OPEN: bidi_bracket_type_t = 2;
pub const BIDI_BRACKET_CLOSE: bidi_bracket_type_t = 3;

// dispextern.h 2122: the basic directionality.
pub type bidi_dir_t = c_int;
pub const NEUTRAL_DIR: bidi_dir_t = 0;
pub const L2R: bidi_dir_t = 1;
pub const R2L: bidi_dir_t = 2;

/* Data type for storing information about characters we need to
   remember.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct bidi_saved_info
{
  pub charpos: ptrdiff_t,               // dispextern.h 2127
  pub r#type: bidi_type_t,              // C spells it `type`; `type` is a Rust keyword
  pub orig_type: bidi_type_t,
}

// dispextern.h 2137-2144.  The three 3-bit members share a 2-byte unit at offset 8, and the
// frozen clang then puts `level` at 10 and `flags` at 11 (measured; the probe is in the
// commit).  PORTING.md 1.3: the storage unit is what the gate can assert.
//
//     s.next_for_neutral_type = 5; s.last_strong_type = 6; s.prev_for_neutral_type = 7;
//     s.level = 9; s.flags = 0x11;      bytes: 00×8 f5 01 09 11 00 00 00 00
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bidi_stack
{
  pub next_for_neutral_pos: ptrdiff_t,  // dispextern.h 2138
  pub bits: u16,                        // next_for_neutral_type:3 | last_strong_type:3 | prev_for_neutral_type:3
  pub level: u8,
  pub flags: u8,                        // sos, override, isolate_status
}

// dispextern.h 2147-2160.  `lstring`/`s`/`schars`/`bufpos` are 8 bytes each, and the two
// `bool_bf : 1` members share a 4-byte unit at offset 32 (`bool_bf` is a typedef in
// conf_post.h 58-62: `bool`, one byte, on this target); sizeof 40, align 8 (measured).
//
//     d.lstring = 0x1122334455667788; d.schars = 5; d.bufpos = 6;
//     d.from_disp_str = 1; d.unibyte = 1;   bytes: 88 77 66 55 44 33 22 11 … 03 00 00 00
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bidi_string_data
{
  pub lstring: Lisp_Object,             // dispextern.h 2148
  pub s: *const u8,                     // `const unsigned char *`
  pub schars: ptrdiff_t,
  pub bufpos: ptrdiff_t,
  pub bits: u32,                        // from_disp_str:1 | unibyte:1
}

/* Check that glyph pointers stored in glyph rows of MATRIX are okay.
   This aborts if any pointer is found twice.  */

// dispextern.h 862-870: the block is inside `#ifdef GLYPH_DEBUG`, and GLYPH_DEBUG is `undef`
// in the frozen configuration (docs/baseline/config/darwin-arm64.macros.tsv), so C compiles
// the `#else` arm: `((void) 0)`.  `CHECK_MATRIX` therefore does not evaluate its argument,
// and neither does the port's live arm (the `aclinfo_free` family's shape in
// lib/include/acl.rs).  The dead arm keeps C's text (PORTING.md P-51) and types its pointee
// as `c_void` until `struct glyph_matrix`'s mirror lands (P-72 rule ③).
#[cfg(feature = "glyph_debug")]
unsafe extern "C" { pub fn check_matrix_pointer_lossage (matrix: *mut c_void); }  // C: `struct glyph_matrix *`
// dispextern.h 869.
macro_rules! CHECK_MATRIX { ($matrix:expr) => { { let _ = ($matrix); 0 } }; }

/***********************************************************************
			    Mouse Highlight
 ***********************************************************************/

/* Structure to hold mouse highlight data.  */

// dispextern.h 2998-3024 (an anonymous `typedef struct`).  `mouse_face_mouse_frame` is a
// `struct frame *` in C; frame.h is not in this mirror's include list (callint.c, json.c and
// marker.c include `dispextern.h` without `frame.h`, which is exactly the P-72 hazard), so the
// pointer keeps the untyped form its use sites cast.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mouse_HLInfo
{
  /* These variables describe the range of text currently shown in its
     mouse-face, together with the window they apply to.  As long as
     the mouse stays within this range, we need not redraw anything on
     its account.  Rows and columns are glyph matrix positions in
     MOUSE_FACE_WINDOW.  */
  pub mouse_face_beg_row: c_int,
  pub mouse_face_beg_col: c_int,
  pub mouse_face_beg_x: c_int,
  pub mouse_face_end_row: c_int,
  pub mouse_face_end_col: c_int,
  pub mouse_face_end_x: c_int,
  pub mouse_face_window: Lisp_Object,
  pub mouse_face_face_id: c_int,
  pub mouse_face_overlay: Lisp_Object,

  /* FRAME and X, Y position of mouse when last checked for
     highlighting.  X and Y can be negative or out of range for the frame.  */
  pub mouse_face_mouse_frame: *mut c_void,   /// C: `struct frame *mouse_face_mouse_frame;`
  pub mouse_face_mouse_x: c_int,
  pub mouse_face_mouse_y: c_int,

  // dispextern.h 3019-3023: three consecutive `bool_bf : 1` members, i.e. one byte.
  pub bits: u8,
}

pub const MOUSE_FACE_PAST_END: u8 = 1 << 0;   // dispextern.h 3019
pub const MOUSE_FACE_DEFER: u8 = 1 << 1;      // dispextern.h 3022
pub const MOUSE_FACE_HIDDEN: u8 = 1 << 2;     // dispextern.h 3023

// dispextern.h 3966: `enum { SCROLL_INFINITY = 1000000 };`
pub const SCROLL_INFINITY: c_int = 1000000;

// C: dispextern.h 472-2600's matrix/face/image structures (`glyph`, `glyph_pool`,
// `glyph_matrix`, `glyph_row`, `face`, `face_cache`, `image`, `it`) and 2600-4026's fringe,
// X-widget and cursor machinery — (not mirrored on this target yet; they need
// `struct frame`/`struct window`/`struct font`, and docs/ports/include-dispextern.md tracks
// the rest of the file's macros, functions and globals).
