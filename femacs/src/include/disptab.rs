// femacs: WIP mirror of src/disptab.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (PORTING.md P-66): `character.c` is the first unit on this target that reads
// the display table, so this file carries the parts of the header that do not depend on the
// glyph datatype — the slot accessors, C's `enum box` (whose last member fixes
// `DISP_TABLE_EXTRA_SLOTS`), `DISP_CHAR_VECTOR` and the two `*_display_table` prototypes.
// The glyph-table macros at the bottom of the header (`GLYPH_TABLE_LENGTH`/`BASE`,
// `GLYPH_SIMPLE_P` … `GLYPH_STRING`, `SPACEGLYPH`, `SET_GLYPH_FROM_CHAR`) need
// dispextern.h's `GLYPH_*` accessors, which are not mirrored on this target yet; their
// boundary is marked below and they land with the display batch.

/* Things for GLYPHS and glyph tables.
   Copyright (C) 1993, 2001-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

// C's `#include "lisp.h"` (disptab.h 23) is the including unit's `include!("include/lisp.rs")`.

// C 20-21's include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_DISPTAB_H: bool = true;

/* Access the slots of a display-table, according to their purpose.  */

// disptab.h 25-28.
#[inline(always)]
pub fn DISP_TABLE_P (obj: Lisp_Object) -> bool
{
  return CHAR_TABLE_P (obj)
	 && EQ (unsafe { (*XCHAR_TABLE (obj)).purpose }, Qdisplay_table ())
	 && CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (obj)) == DISP_TABLE_EXTRA_SLOTS;
}

// disptab.h 30-35: the six slots at the head of the table's `extras` area.
#[inline(always)]
pub fn DISP_TRUNC_GLYPH (dp: *mut Lisp_Char_Table) -> Lisp_Object
{ return unsafe { FAM_ELEM! ((*dp).extras, 0) }; }
#[inline(always)]
pub fn DISP_CONTINUE_GLYPH (dp: *mut Lisp_Char_Table) -> Lisp_Object
{ return unsafe { FAM_ELEM! ((*dp).extras, 1) }; }
#[inline(always)]
pub fn DISP_ESCAPE_GLYPH (dp: *mut Lisp_Char_Table) -> Lisp_Object
{ return unsafe { FAM_ELEM! ((*dp).extras, 2) }; }
#[inline(always)]
pub fn DISP_CTRL_GLYPH (dp: *mut Lisp_Char_Table) -> Lisp_Object
{ return unsafe { FAM_ELEM! ((*dp).extras, 3) }; }
#[inline(always)]
pub fn DISP_INVIS_VECTOR (dp: *mut Lisp_Char_Table) -> Lisp_Object
{ return unsafe { FAM_ELEM! ((*dp).extras, 4) }; }
#[inline(always)]
pub fn DISP_BORDER_GLYPH (dp: *mut Lisp_Char_Table) -> Lisp_Object
{ return unsafe { FAM_ELEM! ((*dp).extras, 5) }; }

// disptab.h 37-52: C's `enum box` (the deprecated box-drawing slots).  `box` is a Rust
// keyword, so the type keeps C's name as a raw identifier (the `r#type` shape); the members
// are C's constants, the way dispextern.rs spells the `draw_glyphs_face` family.
pub type r#box = c_int;
pub const BOX_VERTICAL: r#box = 6;
pub const BOX_HORIZONTAL: r#box = 7;
pub const BOX_DOWN_RIGHT: r#box = 8;
pub const BOX_DOWN_LEFT: r#box = 9;
pub const BOX_UP_RIGHT: r#box = 10;
pub const BOX_UP_LEFT: r#box = 11;
pub const BOX_DOUBLE_VERTICAL: r#box = 12;
pub const BOX_DOUBLE_HORIZONTAL: r#box = 13;
pub const BOX_DOUBLE_DOWN_RIGHT: r#box = 14;
pub const BOX_DOUBLE_DOWN_LEFT: r#box = 15;
pub const BOX_DOUBLE_UP_RIGHT: r#box = 16;
pub const BOX_DOUBLE_UP_LEFT: r#box = 17;

// C spells `DISP_TABLE_EXTRA_SLOTS` as a `#define` inside the enum's body (disptab.h 51);
// its value is `BOX_DOUBLE_UP_LEFT + 1`.
pub const DISP_TABLE_EXTRA_SLOTS: isize = BOX_DOUBLE_UP_LEFT as isize + 1;

unsafe extern "C" { pub fn disp_char_vector (dp: *mut Lisp_Char_Table, c: c_int)
                                              -> Lisp_Object; }                     // disptab.h 55

// disptab.h 57-64.
#[inline(always)]
pub fn DISP_CHAR_VECTOR (dp: *mut Lisp_Char_Table, c: c_int) -> Lisp_Object
{
  return if ASCII_CHAR_P (c as isize)
    {
      if NILP (unsafe { (*dp).ascii })
	{ unsafe { (*dp).defalt } }
      else if SUB_CHAR_TABLE_P (unsafe { (*dp).ascii })
	{ unsafe { FAM_ELEM! ((*XSUB_CHAR_TABLE ((*dp).ascii)).contents, c as isize) } }
      else
	{ unsafe { (*dp).ascii } }
    }
    else
    { unsafe { disp_char_vector (dp, c) } };
}

/* Defined in window.c.  */
unsafe extern "C" { pub fn window_display_table (w: *mut window)
                                                 -> *mut Lisp_Char_Table; }         // disptab.h 67

/* Defined in indent.c.  */
unsafe extern "C" { pub fn buffer_display_table () -> *mut Lisp_Char_Table; }      // disptab.h 70

// C: disptab.h 72-116's GLYPH_TABLE_LENGTH/GLYPH_TABLE_BASE, GLYPH_SIMPLE_P/GLYPH_ALIAS_P/
// GLYPH_FOLLOW_ALIASES, GLYPH_LENGTH/GLYPH_STRING, SPACEGLYPH, NULL_GLYPH and
// SET_GLYPH_FROM_CHAR — (not mirrored on this target yet: they need dispextern.h's
// GLYPH_FACE/GLYPH_CHAR/SET_GLYPH/DEFAULT_FACE_ID accessors; they land with the display
// batch, PORTING.md P-66).
