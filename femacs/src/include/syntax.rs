// femacs: port of src/syntax.h @ fd7afced403 .
//
// WIP mirror (P-66): syntax.h is 215 lines and the single conditional in it is the
// include guard's `#ifndef`, so nothing here is missing because of the frozen
// configuration.  Everything the header declares is below; what is *not* here is the
// code that uses it (`src/syntax.c` is not ported yet), which is why the file carries no
// `extern` definition of its own beyond `gl_state`'s declaration.
//
// The two `#include`s (syntax.h 32-33, `buffer.h` and `lisp.h`) have no counterpart here:
// a `src/` mirror never includes another mirror (PORTING.md 0.1-9), so the unit that
// includes this one includes `buffer.rs` and `lisp.rs` first, in C's order.  The inline
// functions below therefore use `CHAR_TABLE_REF` (lisp.rs), `BVAR!` (buffer.rs) and
// `BEGV_BYTE`/`BYTE_TO_CHAR`/`BUF_BEGV_BYTE` (buffer.rs) from the including unit.

/* Declarations having to do with GNU Emacs syntax tables.

Copyright (C) 1985, 1993-1994, 1997-1998, 2001-2026 Free Software
Foundation, Inc.

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

// syntax.h 22: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_SYNTAX_H: bool = true;

// INLINE_HEADER_BEGIN has no Rust counterpart (PORTING.md 0.1).

/* The standard syntax table is stored where it will automatically
   be used in all new buffers.  */
// C: `#define Vstandard_syntax_table BVAR (&buffer_defaults, syntax_table)`.  The call
// site spells the member with the `_` C's BVAR appends to it (PORTING.md P-37, the
// buffer.rs BVAR! row), because `macro_rules!` cannot paste onto an identifier.
macro_rules! Vstandard_syntax_table {
    () => { BVAR! (&raw mut buffer_defaults, syntax_table_) };
}

/* A syntax table is a chartable whose elements are cons cells
   (CODE+FLAGS . MATCHING-CHAR).  MATCHING-CHAR can be nil if the char
   is not a kind of parenthesis.

   The low 8 bits of CODE+FLAGS is a code, as follows:  */

// syntax.h 43-63: `enum syntaxcode`; the port spells an integer enum as its C type and
// one constant per member (PORTING.md 1.3, the coding.rs precedent).
pub type syntaxcode = c_int;
pub const Swhitespace: syntaxcode = 0;        /* for a whitespace character */
pub const Spunct: syntaxcode = 1;             /* for random punctuation characters */
pub const Sword: syntaxcode = 2;              /* for a word constituent */
pub const Ssymbol: syntaxcode = 3;            /* symbol constituent but not word constituent */
pub const Sopen: syntaxcode = 4;              /* for a beginning delimiter */
pub const Sclose: syntaxcode = 5;             /* for an ending delimiter */
pub const Squote: syntaxcode = 6;             /* for a prefix character like Lisp ' */
pub const Sstring: syntaxcode = 7;            /* for a string-grouping character like Lisp " */
pub const Smath: syntaxcode = 8;              /* for delimiters like $ in Tex.  */
pub const Sescape: syntaxcode = 9;            /* for a character that begins a C-style escape */
pub const Scharquote: syntaxcode = 10;        /* for a character that quotes the following character */
pub const Scomment: syntaxcode = 11;          /* for a comment-starting character */
pub const Sendcomment: syntaxcode = 12;       /* for a comment-ending character */
pub const Sinherit: syntaxcode = 13;          /* use the standard syntax table for this character */
pub const Scomment_fence: syntaxcode = 14;    /* Starts/ends comment which is delimited on the
                                                 other side by any char with the same syntaxcode.  */
pub const Sstring_fence: syntaxcode = 15;     /* Starts/ends string which is delimited on the
                                                 other side by any char with the same syntaxcode.  */
pub const Smax: syntaxcode = 16;              /* Upper bound on codes that are meaningful.  */


#[repr(C)]
#[derive(Clone, Copy)]
pub struct gl_state_s
{
  pub object: Lisp_Object,		/* The object we are scanning.  */
  pub start: ptrdiff_t,			/* Where to stop.  */
  pub stop: ptrdiff_t,			/* Where to stop.  */
  pub use_global: bool,			/* Whether to use global_code
					   or c_s_t.  */
  pub global_code: Lisp_Object,		/* Syntax code of current char.  */
  pub current_syntax_table: Lisp_Object,	/* Syntax table for current pos.  */
  pub old_prop: Lisp_Object,			/* Syntax-table prop at prev pos.  */
  pub b_property: ptrdiff_t,			/* First index where c_s_t is valid.  */
  pub e_property: ptrdiff_t,			/* First index where c_s_t is
					   not valid.  */
  pub e_property_truncated: bool,		/* true if e_property if was truncated
					   by parse_sexp_propertize_done.  */
  pub forward_i: INTERVAL,			/* Where to start lookup on forward.  */
  pub backward_i: INTERVAL,			/* or backward movement.  The
					   data in c_s_t is valid
					   between these intervals,
					   and possibly at the
					   intervals too, depending
					   on:  */
}

/* syntax.h 90: `extern struct gl_state_s gl_state;`.  */
/// `src/syntax.c` 205 defines this object, so the declaration lives in a module and the
/// defining unit shadows it through the glob import (PORTING.md P-39/P-21).
pub mod syntax_h_own_definitions {
  use super::*;
  unsafe extern "C" {
    pub static mut gl_state: gl_state_s;
    /// C: void update_syntax_table (ptrdiff_t, EMACS_INT, Lisp_Object)
    pub fn update_syntax_table (charpos: ptrdiff_t, count: EMACS_INT, init: Lisp_Object);
    /// C: void update_syntax_table_forward (ptrdiff_t, Lisp_Object)
    pub fn update_syntax_table_forward (charpos: ptrdiff_t, init: Lisp_Object);
    /// C: bool syntax_prefix_flag_p (c_int)
    pub fn syntax_prefix_flag_p (c: c_int) -> bool;
    /// C: ptrdiff_t scan_words (ptrdiff_t, EMACS_INT)
    pub fn scan_words (from: ptrdiff_t, count: EMACS_INT) -> ptrdiff_t;
    /// C: void RE_SETUP_SYNTAX_TABLE_FOR_OBJECT (Lisp_Object, ptrdiff_t)
    pub fn RE_SETUP_SYNTAX_TABLE_FOR_OBJECT (object: Lisp_Object, pos: ptrdiff_t);
    /// C: `extern unsigned char const syntax_spec_code[0400];` (syntax.h 101).
    pub static syntax_spec_code: [u8; 256];
  }

  #[inline(always)]
  pub fn SYNTAX_ENTRY (c: c_int) -> Lisp_Object
  {
    return syntax_property_entry (c, false);
  }

  #[inline(always)]
  pub fn SYNTAX_WITH_FLAGS (c: c_int) -> c_int
  {
    return syntax_property_with_flags (c, false);
  }

  #[inline(always)]
  pub fn SYNTAX (c: c_int) -> syntaxcode
  {
    return syntax_property (c, false);
  }
}
// `src/syntax.c` owns every name in that module - `gl_state`, `syntax_spec_code`,
// `syntax_prefix_flag_p`, `scan_words`, `update_syntax_table`, `update_syntax_table_forward`,
// `RE_SETUP_SYNTAX_TABLE_FOR_OBJECT`, and the three macros it redefines with
// `via_property = 1` - so they live in one module and are pulled in with a glob import: the
// owning unit's own definitions shadow the declarations (PORTING.md P-39/P-21).
use syntax_h_own_definitions::*;

/* Fetch the information from the entry for character C
   in the current buffer's syntax table,
   or (if VIA_PROPERTY) from globally kept data (gl_state).
   Does inheritance.  */

#[inline(always)]
pub fn syntax_property_entry (c: c_int, via_property: bool) -> Lisp_Object
{
  if via_property
    { return if unsafe { gl_state.use_global }
	{ unsafe { gl_state.global_code } }
	else { CHAR_TABLE_REF (unsafe { gl_state.current_syntax_table }, c as isize) }; }
  return CHAR_TABLE_REF (unsafe { BVAR! (current_buffer, syntax_table_) }, c as isize);
}
/* Extract the information from the entry for character C
   in the current syntax table.  */

#[inline(always)]
pub fn syntax_property_with_flags (c: c_int, via_property: bool) -> c_int
{
  let ent = syntax_property_entry (c, via_property);
  return if CONSP (ent) { XFIXNUM (XCAR (ent)) as c_int } else { Swhitespace };
}
#[inline(always)]
pub fn syntax_property (c: c_int, via_property: bool) -> syntaxcode
{
  return syntax_property_with_flags (c, via_property) & 0xff;
}
/* This array, indexed by a character less than 256, contains the
   syntax code which that character signifies (as an unsigned char).
   For example, syntax_spec_code['w'] == Sword.  */

/* Convert the regexp's BYTEOFFSET into a character position,
   for the object recorded in gl_state with RE_SETUP_SYNTAX_TABLE_FOR_OBJECT.

   The value is meant for use in code that does nothing when
   parse_sexp_lookup_properties is false, so return 0 in that case,
   for speed.  */

#[inline(always)]
pub fn RE_SYNTAX_TABLE_BYTE_TO_CHAR (byteoffset: ptrdiff_t) -> ptrdiff_t
{
  return if !unsafe { parse_sexp_lookup_properties }
    { 0 }
  else if STRINGP (unsafe { gl_state.object })
    { unsafe { string_byte_to_char (gl_state.object, byteoffset) } }
  else if BUFFERP (unsafe { gl_state.object })
    { unsafe { buf_bytepos_to_charpos (XBUFFER (gl_state.object) as *mut c_void,
				       byteoffset + BUF_BEGV_BYTE (XBUFFER (gl_state.object))) } }
  else if NILP (unsafe { gl_state.object })
    { BYTE_TO_CHAR (byteoffset + unsafe { BEGV_BYTE! () }) }
  else { byteoffset };
}

/* Make syntax table state (gl_state) good for CHARPOS, assuming it is
   currently good for a position before CHARPOS.  */

#[inline(always)]
pub fn UPDATE_SYNTAX_TABLE_FORWARD (charpos: ptrdiff_t)
{ /* Performs just-in-time syntax-propertization.  */
  if unsafe { parse_sexp_lookup_properties } && charpos >= unsafe { gl_state.e_property }
    { unsafe { update_syntax_table_forward (charpos, gl_state.object) } }
}

/* Make syntax table state (gl_state) good for CHARPOS, assuming it is
   currently good for a position after CHARPOS.  */

#[inline(always)]
pub fn UPDATE_SYNTAX_TABLE_BACKWARD (charpos: ptrdiff_t)
{
  if unsafe { parse_sexp_lookup_properties } && charpos < unsafe { gl_state.b_property }
    { unsafe { update_syntax_table (charpos, -1, gl_state.object) } }
}

/* Make syntax table good for CHARPOS.  */

#[inline(always)]
pub fn UPDATE_SYNTAX_TABLE (charpos: ptrdiff_t)
{
  UPDATE_SYNTAX_TABLE_BACKWARD (charpos);
  UPDATE_SYNTAX_TABLE_FORWARD (charpos);
}

/* Set up the buffer-global syntax table.  */

#[inline(always)]
pub fn SETUP_BUFFER_SYNTAX_TABLE ()
{
  unsafe { gl_state.use_global = false; }
  unsafe { gl_state.e_property_truncated = false; }
  unsafe { gl_state.current_syntax_table = BVAR! (current_buffer, syntax_table_) };
}

// INLINE_HEADER_END has no Rust counterpart (PORTING.md 0.1).
