// femacs: WIP mirror of src/disptab.h @ fd7afced403.
//
// The header is complete: the slot accessors, C's `enum box` (whose last member fixes
// `DISP_TABLE_EXTRA_SLOTS`), `DISP_CHAR_VECTOR`, the two `*_display_table` prototypes and -
// since the dispextern mirror landed its `GLYPH_*` accessors - the whole glyph-table family
// at the bottom (`GLYPH_TABLE_LENGTH`/`BASE`, `GLYPH_SIMPLE_P`/`GLYPH_ALIAS_P`/
// `GLYPH_FOLLOW_ALIASES`, `GLYPH_LENGTH`/`GLYPH_STRING`, `SPACEGLYPH`, `NULL_GLYPH`,
// `SET_GLYPH_FROM_CHAR`).

// C's `#include "lisp.h"` (disptab.h 23) is the including unit's `include!("include/lisp.rs")`.

// C 20-21's include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
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

unsafe extern "C-unwind" { pub fn disp_char_vector (dp: *mut Lisp_Char_Table, c: c_int)
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
unsafe extern "C-unwind" { pub fn window_display_table (w: *mut window)
                                                 -> *mut Lisp_Char_Table; }         // disptab.h 67

/* Defined in indent.c.  `buffer_display_table` is that unit's own definition, so the
   prototype sits in a module + glob import: a same-named declaration at this scope would be
   `E0428` in the crate that defines it (PORTING.md 0.1-10/P-70; the change lands with the
   definition, the `buffer_point_decls` shape).  */
pub mod disptab_decls {
  use super::*;

  unsafe extern "C-unwind" { pub fn buffer_display_table () -> *mut Lisp_Char_Table; }   // disptab.h 70
}
pub use self::disptab_decls::*;

/* disptab.h 72-115: the glyph-table family.  C's `GLYPH_CHAR`/`GLYPH_FACE`/`SET_GLYPH_CHAR`/
   `GLYPH_CHAR_VALID_P`/`SET_GLYPH` and `DEFAULT_FACE_ID` come from the dispextern mirror, which
   is where dispextern.h puts them; each macro here is a `macro_rules!` because C's is
   (PORTING.md P-33).  */

/* Return the current length of the GLYPH table, or 0 if the table isn't currently valid.  */
macro_rules! GLYPH_TABLE_LENGTH {
  () => { { if VECTORP (Vglyph_table) { ASIZE (Vglyph_table) } else { 0 } } };
}

/* Return the current base (for indexing) of the GLYPH table, or 0 if invalid.  */
macro_rules! GLYPH_TABLE_BASE {
  () => { { if VECTORP (Vglyph_table) { unsafe { (*XVECTOR (Vglyph_table)).contents.as_mut_ptr () } }
            else { 0 as *mut Lisp_Object } } };
}

/* disptab.h 81-83.  */
macro_rules! GLYPH_SIMPLE_P {
  ($base:expr, $len:expr, $g:expr) => {
    (GLYPH_FACE ($g) != DEFAULT_FACE_ID || GLYPH_CHAR ($g) >= ($len) as c_int
     || ! STRINGP (*(($base)).add (GLYPH_CHAR ($g) as usize)))
  };
}

/* disptab.h 87-88.  */
macro_rules! GLYPH_ALIAS_P {
  ($base:expr, $len:expr, $g:expr) => {
    (GLYPH_FACE ($g) == DEFAULT_FACE_ID && GLYPH_CHAR ($g) < ($len) as c_int
     && FIXNUMP (*(($base)).add (GLYPH_CHAR ($g) as usize)))
  };
}

/* disptab.h 92-98: C's `do { … } while (false)`, written as a block (the SET_GLYPH_FROM_GLYPH_CODE
   shape in dispnew.rs).  */
macro_rules! GLYPH_FOLLOW_ALIASES {
  ($base:expr, $length:expr, $g:expr) => {{
    while GLYPH_ALIAS_P! ($base, $length, $g)
      { SET_GLYPH_CHAR! ($g, XFIXNUM (*(($base)).add (GLYPH_CHAR ($g) as usize)) as c_int); }
    if ! GLYPH_CHAR_VALID_P ($g) { SET_GLYPH_CHAR! ($g, ' ' as c_int); }
  }};
}

/* disptab.h 103-104.  */
macro_rules! GLYPH_LENGTH {
  ($base:expr, $g:expr) => { SCHARS (*(($base)).add (GLYPH_CHAR ($g) as usize)) };
}
macro_rules! GLYPH_STRING {
  ($base:expr, $g:expr) => { SDATA (*(($base)).add (GLYPH_CHAR ($g) as usize)) };
}

/* disptab.h 108-110: octal 040 is 32 (C's value; the extractor folds `040` and `32` alike).  */
pub const SPACEGLYPH: c_int = 32;
pub const NULL_GLYPH: c_int = 0;

/* disptab.h 112-113.  */
macro_rules! SET_GLYPH_FROM_CHAR {
  ($glyph:expr, $c:expr) => { SET_GLYPH! ($glyph, $c, DEFAULT_FACE_ID) };
}
