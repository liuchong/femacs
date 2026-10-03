// femacs: port of src/character.c @ fd7afced403 .
//
// One-to-one port: same statements in the same order, same names (PORTING.md).  The ten
// subrs use `DEFUN!` (PORTING.md 5.1); the two `MANY` subrs take C's `(n, args)` pair.
//
// The two `#ifdef HAVE_WINDOW_SYSTEM` regions in `lisp_string_width` (character.c 361-364
// and 396-420) are dead on the frozen target (config.h leaves HAVE_WINDOW_SYSTEM
// undefined), so their text stays behind `feature = "window_system"`, the feature that
// names the non-frozen side (P-51); `lisp_string_width`'s `#if 0` block
// (character.c 91-97) is a comment, as C's own `#if 0` never compiles.
//
// C's seven octal literals in `char_resolve_modifier_mask` (0377, 0177, 0137, 0101, 0132,
// 037, 0100) have no Rust spelling (`0o…` is a different token pair), so they are the one
// recorded token deviation of this file: docs/ports/macro-body-notes.tsv.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Basic character support.

Copyright (C) 2001-2026 Free Software Foundation, Inc.
Copyright (C) 1995, 1997, 1998, 2001 Electrotechnical Laboratory, JAPAN.
  Licensed to the Free Software Foundation.
Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011
  National Institute of Advanced Industrial Science and Technology (AIST)
  Registration Number H13PRO009

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

/* At first, see the document in `character.h' to understand the code
   in this file.  */

include!("include/config.rs");
include!("../lib/include/stdio.rs");
// C's <string.h> reaches this unit through lisp.h (lisp.h 38-40); `str_as_multibyte`'s
// `memmove` and `lisp_string_width`'s `memcmp` come from that mirror.
include!("../lib/include/string.rs");
// C's <sys/types.h> (character.c 29) has no declaration this unit needs; `ptrdiff_t` comes
// from lisp.rs, as it does in C through <stddef.h>.  <intprops.h> (character.c 30) is the
// `ckd_*` family, which the port spells `overflowing_*` at each site (PORTING.md P-15).
include!("include/lisp.rs");
include!("include/character.rs");
// C's chain, in C's order: buffer.h 28 includes itree.h; frame.h 22-23 includes termhooks.h
// (which includes dispextern.h and systime.h) and window.h; composite.h includes font.h and
// dispextern.h.  The port's include order is the one callint.rs proved (PORTING.md P-61).
include!("include/itree.rs");
include!("include/systime.rs");
include!("include/dispextern.rs");
include!("include/composite.rs");
include!("include/coding.rs");
include!("include/buffer.rs");
include!("include/termhooks.rs");
include!("include/window.rs");
include!("include/frame.rs");
include!("include/dispextern.rs");
include!("include/composite.rs");
include!("include/disptab.rs");

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status §3.29, the casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vtranslation_table_vector: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vauto_fill_chars: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vchar_width_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vambiguous_width_chars: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprintable_chars: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vchar_script_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vscript_representative_chars: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vunicode_category_table: Lisp_Object = lisp_h_Qnil ();

// C spells unicode_category_t's members unqualified (they are an `enum`); Rust names an
// enum's variants through the enum, so the port imports them (character.rs mirror 549-582).
use unicode_category_t::*;

/* Char-table of information about which character to unify to which
   Unicode character.  Mainly used by the macro MAYBE_UNIFY_CHAR.  */
#[unsafe(no_mangle)]
pub static mut Vchar_unify_table: Lisp_Object = lisp_h_Qnil ();



/* If character code C has modifier masks, reflect them to the
   character code if possible.  Return the resulting code.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn char_resolve_modifier_mask (mut c: EMACS_INT) -> EMACS_INT
{
  /* A non-ASCII character can't reflect modifier bits to the code.  */
  if ! ASCII_CHAR_P ((c & !(CHAR_MODIFIER_MASK as EMACS_INT)) as isize)
    { return c; }

  /* For Meta, Shift, and Control modifiers, we need special care.  */
  if (c & CHAR_SHIFT as EMACS_INT) != 0
    {
      /* Shift modifier is valid only with [A-Za-z].  */
      if (c & 0o377) >= b'A' as EMACS_INT && (c & 0o377) <= b'Z' as EMACS_INT
	{ c &= !(CHAR_SHIFT as EMACS_INT); }
      else if (c & 0o377) >= b'a' as EMACS_INT && (c & 0o377) <= b'z' as EMACS_INT
	{ c = (c & !(CHAR_SHIFT as EMACS_INT)) - (b'a' as EMACS_INT - b'A' as EMACS_INT); }
      /* Shift modifier for control characters and SPC is ignored.  */
      else if (c & !(CHAR_MODIFIER_MASK as EMACS_INT)) <= 0x20
	{ c &= !(CHAR_SHIFT as EMACS_INT); }
    }
  if (c & CHAR_CTL as EMACS_INT) != 0
    {
      /* Simulate the code in lread.c.  */
      /* Allow `\C- ' and `\C-?'.  */
      if (c & 0o377) == b' ' as EMACS_INT
	{ c &= !0o177 & !(CHAR_CTL as EMACS_INT); }
      else if (c & 0o377) == b'?' as EMACS_INT
	{ c = 0o177 | (c & !0o177 & !(CHAR_CTL as EMACS_INT)); }
      /* ASCII control chars are made from letters (both cases),
	 as well as the non-letters within 0100...0137.  */
      else if (c & 0o137) >= 0o101 && (c & 0o137) <= 0o132
	{ c &= 0o37 | (!0o177 & !(CHAR_CTL as EMACS_INT)); }
      else if (c & 0o177) >= 0o100 && (c & 0o177) <= 0o137
	{ c &= 0o37 | (!0o177 & !(CHAR_CTL as EMACS_INT)); }
    }
  /* C: `#if 0` (character.c 91-97): "This is outside the scope of this function.
     (bug#4751)" — it never compiles, so it stays a comment.  */

  return c;
}


/* Store multibyte form of character C at P.  If C has modifier bits,
   handle them appropriately.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn char_string (mut c: c_uint, p: *mut u8) -> c_int
{
  let mut bytes: c_int;

  if (c & CHAR_MODIFIER_MASK as c_uint) != 0
    {
      c = char_resolve_modifier_mask (c as EMACS_INT) as c_uint;
      /* If C still has any modifier bits, just ignore it.  */
      c &= !(CHAR_MODIFIER_MASK as c_uint);
    }

  if c <= MAX_3_BYTE_CHAR as c_uint
    {
      bytes = CHAR_STRING (c as c_int, p);
    }
  else if c <= MAX_4_BYTE_CHAR as c_uint
    {
      *p.offset (0) = 0xF0 | ((c >> 18) as u8);
      *p.offset (1) = 0x80 | (((c >> 12) & 0x3F) as u8);
      *p.offset (2) = 0x80 | (((c >> 6) & 0x3F) as u8);
      *p.offset (3) = 0x80 | ((c & 0x3F) as u8);
      bytes = 4;
    }
  else if c <= MAX_5_BYTE_CHAR as c_uint
    {
      *p.offset (0) = 0xF8;
      *p.offset (1) = 0x80 | (((c >> 18) & 0x0F) as u8);
      *p.offset (2) = 0x80 | (((c >> 12) & 0x3F) as u8);
      *p.offset (3) = 0x80 | (((c >> 6) & 0x3F) as u8);
      *p.offset (4) = 0x80 | ((c & 0x3F) as u8);
      bytes = 5;
    }
  else if c <= MAX_CHAR as c_uint
    {
      c = CHAR_TO_BYTE8 (c as c_int) as c_uint;
      bytes = BYTE8_STRING (c as c_int, p);
    }
  else
    { error (c"Invalid character: %x".as_ptr (), c); }

  return bytes;
}


/* Translate character C by translation table TABLE.  If no translation is
   found in TABLE, return the untranslated character.  If TABLE is a list,
   elements are char tables.  In that case, recursively translate C by all the
   tables in the list.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn translate_char (mut table: Lisp_Object, mut c: c_int) -> c_int
{
  if CHAR_TABLE_P (table)
    {
      let ch: Lisp_Object;

      ch = CHAR_TABLE_REF (table, c as isize);
      if CHARACTERP (ch)
	{ c = XFIXNUM (ch) as c_int; }
    }
  else
    {
      while CONSP (table)
	{
	  c = translate_char (XCAR (table), c);
	  table = XCDR (table);
	}
    }
  return c;
}

DEFUN! ("characterp", Fcharacterp, Scharacterp, 1, 2, 0,
       doc: "Return non-nil if OBJECT is a character.\nIn Emacs Lisp, characters are represented by character codes, which\nare non-negative integers.  The function `max-char' returns the\nmaximum character code.\nusage: (characterp OBJECT)",
       (object: Lisp_Object, ignore: Lisp_Object),
       {
  return if CHARACTERP (object) { Qt () } else { Qnil () };
});

DEFUN! ("max-char", Fmax_char, Smax_char, 0, 1, 0,
       doc: "Return the maximum character code.\nIf UNICODE is non-nil, return the maximum character code defined\nby the Unicode Standard.",
       (unicode: Lisp_Object),
       {
  return if ! NILP (unicode)
	 { make_fixnum (MAX_UNICODE_CHAR as EMACS_INT) }
	 else
	 { make_fixnum (MAX_CHAR as EMACS_INT) };
});

DEFUN! ("unibyte-char-to-multibyte", Funibyte_char_to_multibyte,
       Sunibyte_char_to_multibyte, 1, 1, 0,
       doc: "Convert the byte CH to multibyte character.",
       (ch: Lisp_Object),
       {
  let mut c: c_int;

  CHECK_CHARACTER (ch);
  c = XFIXNAT (ch) as c_int;
  if c >= 0x100
    { error (c"Not a unibyte character: %d".as_ptr (), c); }
  return make_fixnum (make_char_multibyte (c) as EMACS_INT);
});

DEFUN! ("multibyte-char-to-unibyte", Fmultibyte_char_to_unibyte,
       Smultibyte_char_to_unibyte, 1, 1, 0,
       doc: "Convert the multibyte character CH to a byte.\nIf the multibyte character does not represent a byte, return -1.",
       (ch: Lisp_Object),
       {
  let mut cm: c_int;

  CHECK_CHARACTER (ch);
  cm = XFIXNAT (ch) as c_int;
  if cm < 256
    /* Can't distinguish a byte read from a unibyte buffer from
       a latin1 char, so let's let it slide.  */
    { return ch; }
  else
    {
      let cu: c_int = CHAR_TO_BYTE_SAFE (cm);
      return make_fixnum (cu as EMACS_INT);
    }
});


/* Return width (columns) of C considering the buffer display table DP. */

unsafe fn char_width (c: c_int, dp: *mut Lisp_Char_Table) -> ptrdiff_t
{
  let mut width: ptrdiff_t = CHARACTER_WIDTH (c) as ptrdiff_t;

  if ! dp.is_null ()
    {
      let disp: Lisp_Object = DISP_CHAR_VECTOR (dp, c);
      let mut ch: Lisp_Object;
      let mut i: c_int;

      if VECTORP (disp)
	{
	  i = 0;
	  width = 0;
	  while i < ASIZE (disp) as c_int
	    {
	      let mut c: c_int = -1;
	      ch = AREF (disp, i as isize);
	      if GLYPH_CODE_P (ch)
		{ c = GLYPH_CODE_CHAR (ch); }
	      else if CHARACTERP (ch)
		{ c = XFIXNUM (ch) as c_int; }
	      if c >= 0
		{
		  let w: c_int = CHARACTER_WIDTH (c);
		  let (sum, over) = width.overflowing_add (w as ptrdiff_t);
		  width = sum;
		  if over
		    { string_overflow (); }
		}
	      i += 1;
	    }
	}
    }
  return width;
}


DEFUN! ("char-width", Fchar_width, Schar_width, 1, 1, 0,
       doc: "Return width of CHAR in columns when displayed in the current buffer.\nThe width of CHAR is measured by how many columns it will occupy on the screen.\nThis is based on data in `char-width-table', and ignores the actual\nmetrics of the character's glyph as determined by its font.\nIf the display table in effect replaces CHAR on display with\nsomething else, the function returns the width of the replacement.\nTab is taken to occupy `tab-width' columns.\nusage: (char-width CHAR)",
       (ch: Lisp_Object),
       {
  let mut c: c_int;
  let width: ptrdiff_t;

  CHECK_CHARACTER (ch);
  c = XFIXNUM (ch) as c_int;
  width = char_width (c, buffer_display_table ());
  return make_fixnum (width as EMACS_INT);
});

/* Return width of string STR of length LEN when displayed in the
   current buffer.  The width is measured by how many columns it
   occupies on the screen.  If PRECISION > 0, return the width of
   longest substring that doesn't exceed PRECISION, and set number of
   characters and bytes of the substring in *NCHARS and *NBYTES
   respectively.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_string_width (str: *const u8, len: ptrdiff_t, precision: c_int,
					 nchars: *mut ptrdiff_t, nbytes: *mut ptrdiff_t)
  -> ptrdiff_t
{
  let mut i: ptrdiff_t = 0;
  let mut i_byte: ptrdiff_t = 0;
  let mut width: ptrdiff_t = 0;
  let dp: *mut Lisp_Char_Table = buffer_display_table ();

  while i_byte < len
    {
      let mut bytes: c_int = 0;
      let c: c_int = string_char_and_length (str.offset (i_byte), &raw mut bytes);
      let thiswidth: ptrdiff_t = char_width (c, dp) as ptrdiff_t;

      if 0 < precision && precision as ptrdiff_t - width < thiswidth
	{
	  *nchars = i;
	  *nbytes = i_byte;
	  return width;
	}
      let (sum, over) = width.overflowing_add (thiswidth);
      width = sum;
      if over
	{ string_overflow (); }
      i += 1;
      i_byte += bytes as ptrdiff_t;
  }

  if precision > 0
    {
      *nchars = i;
      *nbytes = i_byte;
    }

  return width;
}

/* Return width of string STR of length LEN when displayed in the
   current buffer.  The width is measured by how many columns it
   occupies on the screen.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strwidth (str: *const c_char, len: ptrdiff_t) -> ptrdiff_t
{
  return c_string_width (str as *const u8, len, -1, std::ptr::null_mut (),
			 std::ptr::null_mut ());
}

/* Return width of a (substring of a) Lisp string STRING when
   displayed in the current buffer.  The width is measured by how many
   columns it occupies on the screen while paying attention to
   compositions.  If PRECISION > 0, return the width of longest
   substring that doesn't exceed PRECISION, and set number of
   characters and bytes of the substring in *NCHARS and *NBYTES
   respectively.  FROM and TO are zero-based character indices that
   define the substring of STRING to consider.  If AUTO_COMP is
   non-zero, account for automatic compositions in STRING.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lisp_string_width (string: Lisp_Object, from: ptrdiff_t,
					    to: ptrdiff_t, precision: ptrdiff_t,
					    nchars: *mut ptrdiff_t, nbytes: *mut ptrdiff_t,
					    auto_comp: bool) -> ptrdiff_t
{
  /* This set multibyte to 0 even if STRING is multibyte when it
     contains only ascii and eight-bit-graphic, but that's
     intentional.  */
  let multibyte: bool = SCHARS (string) < SBYTES (string);
  let mut i: ptrdiff_t = from;
  let mut i_byte: ptrdiff_t = if from != 0 { string_char_to_byte (string, from) } else { 0 };
  let from_byte: ptrdiff_t = i_byte;
  let mut width: ptrdiff_t = 0;
  let dp: *mut Lisp_Char_Table = buffer_display_table ();
  let f: *mut frame =
    if FRAMEP (selected_frame) && FRAME_LIVE_P (XFRAME (selected_frame))
    { XFRAME (selected_frame) }
    else
    { std::ptr::null_mut () };
  let mut font_width: c_int = -1;
  // C's `#ifdef HAVE_WINDOW_SYSTEM` declarations (character.c 361-364) are dead on this
  // target; the feature that names the non-frozen side keeps the text (P-51).
  #[cfg(feature = "window_system")]
  let mut default_font: Lisp_Object = Qnil ();
  #[cfg(feature = "window_system")]
  let mut frame_font: Lisp_Object = Qnil ();

  eassert! (precision <= 0 || (! nchars.is_null () && ! nbytes.is_null ()));

  while i < to
    {
      let mut chars: ptrdiff_t;
      let mut bytes: ptrdiff_t;
      let mut thiswidth: ptrdiff_t;
      let mut cmp_id: ptrdiff_t;
      let mut ignore: ptrdiff_t = 0;
      let mut end: ptrdiff_t = 0;
      let mut val: Lisp_Object = Qnil ();

      if find_composition (i, -1, &raw mut ignore, &raw mut end, &raw mut val, string)
	 && { cmp_id = get_composition_id (i, i_byte, end - i, val, string); cmp_id >= 0 }
	{
	  thiswidth = (**composition_table.add (cmp_id as usize)).width as ptrdiff_t;
	  chars = end - i;
	  bytes = string_char_to_byte (string, end) - i_byte;
	}
      else if auto_comp
	       && ! f.is_null ()
	       && multibyte
	       && find_automatic_composition (i, -1, i, &raw mut ignore,
					      &raw mut end, &raw mut val, string)
	       && end > i
	{
	  let mut j: c_int;
	  j = 0;
	  while (j as ptrdiff_t) < LGSTRING_GLYPH_LEN (val)
	    {
	      if NILP (LGSTRING_GLYPH (val, j as isize))
		{ break; }
	      j += 1;
	    }

	  let pixelwidth: c_int = composition_gstring_width (val, 0, j as ptrdiff_t,
							     std::ptr::null_mut ());

	  // C: `#ifdef HAVE_WINDOW_SYSTEM` (character.c 396-420), dead on this target (P-51).
	  #[cfg(feature = "window_system")]
	  {
	    /* The below is somewhat expensive, so compute it only once
	       for the entire loop, and only if needed.  */
	    if FRAME_WINDOW_P (f) && font_width < 0
	      {
		font_width = FRAME_COLUMN_WIDTH (f);
		default_font = Fface_font (Qdefault (), Qnil (), Qnil ());
		frame_font = Fframe_parameter (Qnil (), Qfont ());

		if STRINGP (default_font) && STRINGP (frame_font)
		    && (SCHARS (default_font) != SCHARS (frame_font)
			|| SBYTES (default_font) != SBYTES (frame_font)
			|| memcmp (SDATA (default_font) as *const c_void,
				   SDATA (frame_font) as *const c_void,
				   SBYTES (default_font) as usize) != 0)
		  {
		    let font_info: Lisp_Object = Ffont_info (default_font, Qnil ());
		    if VECTORP (font_info)
		      {
			font_width = XFIXNUM (AREF (font_info, 11)) as c_int;
			if font_width <= 0
			  { font_width = XFIXNUM (AREF (font_info, 10)) as c_int; }
		      }
		  }
	      }
	  }

	  thiswidth = if FRAME_WINDOW_P (f)
	    { (pixelwidth as c_double / font_width as c_double + 0.5) as ptrdiff_t }
	    else
	    { pixelwidth as ptrdiff_t };
	  chars = end - i;
	  bytes = string_char_to_byte (string, end) - i_byte;
	}
      else
	{
	  let mut c: c_int;
	  let str: *mut u8 = SDATA (string);

	  if multibyte
	    {
	      let mut cbytes: c_int = 0;
	      c = string_char_and_length (str.offset (i_byte), &raw mut cbytes);
	      bytes = cbytes as ptrdiff_t;
	    }
	  else
	    { c = *str.offset (i_byte) as c_int; bytes = 1; }
	  chars = 1;
	  thiswidth = char_width (c, dp) as ptrdiff_t;
	}

      if 0 < precision && precision - width < thiswidth
	{
	  *nchars = i - from;
	  *nbytes = i_byte - from_byte;
	  return width;
	}
      let (sum, over) = width.overflowing_add (thiswidth);
      width = sum;
      if over
	{ string_overflow (); }
      i += chars;
      i_byte += bytes;
    }

  if precision > 0
    {
      *nchars = i - from;
      *nbytes = i_byte - from_byte;
    }

  return width;
}

DEFUN! ("string-width", Fstring_width, Sstring_width, 1, 3, 0,
       doc: "Return width of STRING in columns when displayed in the current buffer.\nWidth of STRING is measured by how many columns it will occupy on the screen.\n\nOptional arguments FROM and TO specify the substring of STRING to\nconsider, and are interpreted as in `substring'.\n\nWidth of each character in STRING is generally taken according to\n`char-width', but character compositions and the display table in\neffect are taken into consideration.\nTabs in STRING are always assumed to occupy `tab-width' columns,\nalthough they might take fewer columns depending on the column where\nthey begin on display.\nThe effect of faces and fonts, including fonts used for non-Latin and\nother unusual characters, such as emoji, is ignored, as are display\nproperties and invisible text.\n\nFor these reasons, the results are just an approximation, especially\non GUI frames; for accurate dimensions of text as it will be\ndisplayed, use `string-pixel-width' or `window-text-pixel-size'\ninstead.\nusage: (string-width STRING &optional FROM TO)",
       (str: Lisp_Object, from: Lisp_Object, to: Lisp_Object),
       {
  let mut val: Lisp_Object;
  let mut ifrom: ptrdiff_t = 0;
  let mut ito: ptrdiff_t = 0;

  CHECK_STRING (str);
  validate_subarray (str, from, to, SCHARS (str), &raw mut ifrom, &raw mut ito);
  XSETFASTINT! (val, lisp_string_width (str, ifrom, ito, -1, std::ptr::null_mut (),
					std::ptr::null_mut (), true) as EMACS_INT);
  return val;
});

/* Return the number of characters in the NBYTES bytes at PTR.
   This works by looking at the contents and checking for multibyte
   sequences while assuming that there's no invalid sequence.
   However, if the current buffer has enable-multibyte-characters =
   nil, we treat each byte as a character.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn chars_in_text (ptr: *const u8, nbytes: ptrdiff_t) -> ptrdiff_t
{
  /* current_buffer is null at early stages of Emacs initialization.  */
  if current_buffer.is_null ()
      || NILP (BVAR! (current_buffer, enable_multibyte_characters_))
    { return nbytes; }

  return multibyte_chars_in_text (ptr, nbytes);
}

/* Return the number of characters in the NBYTES bytes at PTR.
   This works by looking at the contents and checking for multibyte
   sequences while assuming that there's no invalid sequence.  It
   ignores enable-multibyte-characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn multibyte_chars_in_text (mut ptr: *const u8, nbytes: ptrdiff_t)
  -> ptrdiff_t
{
  let endp: *const u8 = ptr.add (nbytes as usize);
  let mut chars: ptrdiff_t = 0;

  while ptr < endp
    {
      let len: c_int = multibyte_length (ptr, endp, true, true);

      if len == 0
	{ emacs_abort (); }
      ptr = ptr.add (len as usize);
      chars += 1;
    }

  return chars;
}

/* Parse unibyte text at STR of LEN bytes as a multibyte text, count
   characters and bytes in it, and store them in *NCHARS and *NBYTES
   respectively.  On counting bytes, pay attention to that 8-bit
   characters not constructing a valid multibyte sequence are
   represented by 2-byte in a multibyte text.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_str_as_multibyte (mut str: *const u8, len: ptrdiff_t,
						 nchars: *mut ptrdiff_t,
						 nbytes: *mut ptrdiff_t)
{
  let endp: *const u8 = str.add (len as usize);
  let mut chars: ptrdiff_t = 0;
  let mut bytes: ptrdiff_t = 0;

  if len >= MAX_MULTIBYTE_LENGTH as ptrdiff_t
    {
      let adjusted_endp: *const u8 =
	endp.offset (-((MAX_MULTIBYTE_LENGTH - 1) as isize));
      while str < adjusted_endp
	{
	  let n: c_int = multibyte_length (str, std::ptr::null (), false, false);
	  if 0 < n
	    { str = str.add (n as usize); bytes += n as ptrdiff_t; }
	  else
	    { str = str.add (1); bytes += 2; }
	  chars += 1;
	}
    }
  while str < endp
    {
      let n: c_int = multibyte_length (str, endp, true, false);
      if 0 < n
	{ str = str.add (n as usize); bytes += n as ptrdiff_t; }
      else
	{ str = str.add (1); bytes += 2; }
      chars += 1;
    }

  *nchars = chars;
  *nbytes = bytes;
}

/* Arrange unibyte text at STR of NBYTES bytes as a multibyte text.
   It actually converts only such 8-bit characters that don't construct
   a multibyte sequence to multibyte forms of raw bytes.  If NCHARS
   is nonzero, set *NCHARS to the number of characters in the text.
   It is assured that we can use LEN bytes at STR as a work
   area and that is enough.  Return the number of bytes of the
   resulting text.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn str_as_multibyte (str: *mut u8, len: ptrdiff_t,
					   mut nbytes: ptrdiff_t,
					   nchars: *mut ptrdiff_t) -> ptrdiff_t
{
  let mut p: *mut u8 = str;
  let mut endp: *mut u8 = str.add (nbytes as usize);
  let mut to: *mut u8;
  let mut chars: ptrdiff_t = 0;

  if nbytes >= MAX_MULTIBYTE_LENGTH as ptrdiff_t
    {
      let adjusted_endp: *mut u8 = endp.offset (-((MAX_MULTIBYTE_LENGTH - 1) as isize));
      while p < adjusted_endp
	{
	  let n: c_int = multibyte_length (p, std::ptr::null (), false, false);
	  if n <= 0
	    { break; }
	  p = p.add (n as usize); chars += 1;
	}
    }
  loop
    {
      let n: c_int = multibyte_length (p, endp, true, false);
      if n <= 0
	{ break; }
      p = p.add (n as usize); chars += 1;
    }
  if ! nchars.is_null ()
    { *nchars = chars; }
  if p == endp
    { return nbytes; }

  to = p;
  nbytes = endp.offset_from (p);
  endp = str.add (len as usize);
  memmove (endp.offset (-nbytes) as *mut c_void, p as *const c_void, nbytes as usize);
  p = endp.offset (-nbytes);

  if nbytes >= MAX_MULTIBYTE_LENGTH as ptrdiff_t
    {
      let adjusted_endp: *mut u8 = endp.offset (-((MAX_MULTIBYTE_LENGTH - 1) as isize));
      while p < adjusted_endp
	{
	  let n: c_int = multibyte_length (p, std::ptr::null (), false, false);
	  if 0 < n
	    {
	      let mut n = n;
	      while n != 0
		{ n -= 1; *to = *p; to = to.add (1); p = p.add (1); }
	    }
	  else
	    {
	      let c: c_int = *p as c_int;
	      p = p.add (1);
	      let c = BYTE8_TO_CHAR (c);
	      to = to.add (CHAR_STRING (c, to) as usize);
	    }
	  chars += 1;
	}
    }
  while p < endp
    {
      let n: c_int = multibyte_length (p, endp, true, false);
      if 0 < n
	{
	  let mut n = n;
	  while n != 0
	    { n -= 1; *to = *p; to = to.add (1); p = p.add (1); }
	}
      else
	{
	  let c: c_int = *p as c_int;
	  p = p.add (1);
	  let c = BYTE8_TO_CHAR (c);
	  to = to.add (CHAR_STRING (c, to) as usize);
	}
      chars += 1;
    }
  if ! nchars.is_null ()
    { *nchars = chars; }
  return to.offset_from (str);
}

/* Parse unibyte string at STR of LEN bytes, and return the number of
   bytes it may occupy when converted to multibyte string by
   `str_to_multibyte'.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn count_size_as_multibyte (str: *const u8, len: ptrdiff_t)
  -> ptrdiff_t
{
  /* Count the number of non-ASCII (raw) bytes, since they will occupy
     two bytes in a multibyte string.  */
  let mut nonascii: ptrdiff_t = 0;
  let mut i: ptrdiff_t = 0;
  while i < len
    {
      nonascii += (*str.offset (i) >> 7) as ptrdiff_t;
      i += 1;
    }
  let (bytes, over) = len.overflowing_add (nonascii);
  if over
    { string_overflow (); }
  return bytes;
}


/* Convert unibyte text at SRC of NCHARS chars to a multibyte text
   at DST, that contains the same single-byte characters.
   Return the number of bytes written at DST.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn str_to_multibyte (dst: *mut u8, src: *const u8,
					   nchars: ptrdiff_t) -> ptrdiff_t
{
  let mut d: *mut u8 = dst;
  let mut i: ptrdiff_t = 0;
  while i < nchars
    {
      let c: u8 = *src.offset (i);
      if c <= 0x7f
	{ *d = c; d = d.add (1); }
      else
	{
	  *d = (0xc0 + ((c >> 6) & 1)) as u8; d = d.add (1);
	  *d = (0x80 + (c & 0x3f)) as u8; d = d.add (1);
	}
      i += 1;
    }
  return d.offset_from (dst);
}

/* Arrange multibyte text at STR of LEN bytes as a unibyte text.  It
   actually converts characters in the range 0x80..0xFF to
   unibyte.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn str_as_unibyte (str: *mut u8, bytes: ptrdiff_t) -> ptrdiff_t
{
  let mut p: *const u8 = str;
  let endp: *const u8 = str.add (bytes as usize);
  let mut to: *mut u8;
  let mut c: c_int;
  let mut len: c_int;

  while p < endp
    {
      c = *p as c_int;
      len = BYTES_BY_CHAR_HEAD (c);
      if CHAR_BYTE8_HEAD_P (c)
	{ break; }
      p = p.add (len as usize);
    }
  to = str.add (p.offset_from (str) as usize);
  while p < endp
    {
      c = *p as c_int;
      len = BYTES_BY_CHAR_HEAD (c);
      if CHAR_BYTE8_HEAD_P (c)
	{
	  c = string_char_advance (&raw mut p);
	  *to = CHAR_TO_BYTE8 (c) as u8; to = to.add (1);
	}
      else
	{
	  while len != 0
	    { len -= 1; *to = *p; to = to.add (1); p = p.add (1); }
	}
    }
  return to.offset_from (str);
}

unsafe fn string_count_byte8 (string: Lisp_Object) -> ptrdiff_t
{
  let multibyte: bool = STRING_MULTIBYTE (string);
  let nbytes: ptrdiff_t = SBYTES (string);
  let mut p: *mut u8 = SDATA (string);
  let pend: *mut u8 = p.add (nbytes as usize);
  let mut count: ptrdiff_t = 0;
  let mut c: c_int;
  let mut len: c_int;

  if multibyte
    {
      while p < pend
	{
	  c = *p as c_int;
	  len = BYTES_BY_CHAR_HEAD (c);

	  if CHAR_BYTE8_HEAD_P (c)
	    { count += 1; }
	  p = p.add (len as usize);
	}
    }
  else
    {
      while p < pend
	{
	  let byte = *p;
	  p = p.add (1);
	  if byte >= 0x80
	    { count += 1; }
	}
    }
  return count;
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn string_escape_byte8 (string: Lisp_Object) -> Lisp_Object
{
  let nchars: ptrdiff_t = SCHARS (string);
  let nbytes: ptrdiff_t = SBYTES (string);
  let multibyte: bool = STRING_MULTIBYTE (string);
  let byte8_count: ptrdiff_t;
  let thrice_byte8_count: ptrdiff_t;
  let uninit_nchars: ptrdiff_t;
  let uninit_nbytes: ptrdiff_t;
  let mut src: *const u8;
  let src_end: *const u8;
  let mut dst: *mut u8;
  let val: Lisp_Object;
  let mut c: c_int;
  let mut len: c_int;

  if multibyte && nchars == nbytes
    { return string; }

  byte8_count = string_count_byte8 (string);

  if byte8_count == 0
    { return string; }

  let (product, mul_over) = byte8_count.overflowing_mul (3);
  thrice_byte8_count = product;
  if mul_over
    { string_overflow (); }

  if multibyte
    {
      /* Convert 2-byte sequence of byte8 chars to 4-byte octal.  */
      let (sum, over) = nchars.overflowing_add (thrice_byte8_count);
      uninit_nchars = sum;
      if over
	{ string_overflow (); }
      let (sum, over) = nbytes.overflowing_add (2 * byte8_count);
      uninit_nbytes = sum;
      if over
	{ string_overflow (); }
      val = make_uninit_multibyte_string (uninit_nchars, uninit_nbytes);
    }
  else
    {
      /* Convert 1-byte sequence of byte8 chars to 4-byte octal.  */
      let (sum, over) = thrice_byte8_count.overflowing_add (nbytes);
      uninit_nbytes = sum;
      if over
	{ string_overflow (); }
      val = make_uninit_string (uninit_nbytes);
    }

  src = SDATA (string);
  src_end = src.add (nbytes as usize);
  dst = SDATA (val);
  if multibyte
    {
      while src < src_end
	{
	  c = *src as c_int;
	  len = BYTES_BY_CHAR_HEAD (c);

	  if CHAR_BYTE8_HEAD_P (c)
	    {
	      c = string_char_advance (&raw mut src);
	      c = CHAR_TO_BYTE8 (c);
	      dst = dst.add (sprintf (dst as *mut c_char, c"\\%03o".as_ptr (),
				      c as c_uint) as usize);
	    }
	  else
	    {
	      while len != 0
		{ len -= 1; *dst = *src; dst = dst.add (1); src = src.add (1); }
	    }
	}
    }
  else
    {
      while src < src_end
	{
	  c = *src as c_int;
	  src = src.add (1);
	  if c >= 0x80
	    { dst = dst.add (sprintf (dst as *mut c_char, c"\\%03o".as_ptr (),
				     c as c_uint) as usize); }
	  else
	    { *dst = c as u8; dst = dst.add (1); }
	}
    }
  return val;
}


DEFUN! ("string", Fstring, Sstring, 0, MANY, 0,
       doc: "Concatenate all the argument characters and make the result a string.\nusage: (string &rest CHARACTERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut nbytes: ptrdiff_t = 0;
  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      CHECK_CHARACTER (*args.offset (i));
      nbytes += CHAR_BYTES (XFIXNUM (*args.offset (i)) as c_int) as ptrdiff_t;
      i += 1;
    }
  if nbytes == nargs
    { return Funibyte_string (nargs, args); }
  let str: Lisp_Object = make_uninit_multibyte_string (nargs, nbytes);
  let mut p: *mut u8 = SDATA (str);
  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      eassume! (CHARACTERP (*args.offset (i)));
      let c: c_int = XFIXNUM (*args.offset (i)) as c_int;
      p = p.add (CHAR_STRING (c, p) as usize);
      i += 1;
    }
  return str;
});

DEFUN! ("unibyte-string", Funibyte_string, Sunibyte_string, 0, MANY, 0,
       doc: "Concatenate all the argument bytes and make the result a unibyte string.\nusage: (unibyte-string &rest BYTES)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let str: Lisp_Object = make_uninit_string (nargs);
  let mut p: *mut u8 = SDATA (str);
  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      *p = check_integer_range (*args.offset (i), 0, 255) as u8;
      p = p.add (1);
      i += 1;
    }
  return str;
});

DEFUN! ("char-resolve-modifiers", Fchar_resolve_modifiers,
       Schar_resolve_modifiers, 1, 1, 0,
       doc: "Resolve modifiers in the character CHAR.\nThe value is a character with modifiers resolved into the character\ncode.  Unresolved modifiers are kept in the value.\nusage: (char-resolve-modifiers CHAR)",
       (character: Lisp_Object),
       {
  let c: EMACS_INT;

  CHECK_FIXNUM (character);
  c = XFIXNUM (character);
  return make_fixnum (char_resolve_modifier_mask (c));
});

DEFUN! ("get-byte", Fget_byte, Sget_byte, 0, 2, 0,
       doc: "Return a byte value of a character at point.\nOptional 1st arg POSITION, if non-nil, is a position of a character to get\na byte value.\nOptional 2nd arg STRING, if non-nil, is a string of which first\ncharacter is a target to get a byte value.  In this case, POSITION, if\nnon-nil, is an index of a target character in the string.\n\nIf the current buffer (or STRING) is multibyte, and the target\ncharacter is not ASCII nor 8-bit character, an error is signaled.",
       (position: Lisp_Object, string: Lisp_Object),
       {
  let mut c: c_int;
  let mut pos: ptrdiff_t;
  let mut p: *mut u8;

  if NILP (string)
    {
      if NILP (position)
	{
	  p = PT_ADDR! ();
	}
      else
	{
	  let fixed_pos: EMACS_INT = fix_position (position);
	  if ! (BEGV! () <= fixed_pos && fixed_pos < ZV! ())
	    { args_out_of_range_3 (position, make_fixnum (BEGV! ()), make_fixnum (ZV! ())); }
	  pos = fixed_pos;
	  p = CHAR_POS_ADDR (pos);
	}
      if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	{ return make_fixnum (*p as EMACS_INT); }
    }
  else
    {
      CHECK_STRING (string);
      if NILP (position)
	{
	  p = SDATA (string);
	}
      else
	{
	  CHECK_FIXNAT (position);
	  if XFIXNUM (position) >= SCHARS (string)
	    { args_out_of_range (string, position); }
	  pos = XFIXNAT (position);
	  p = SDATA (string).offset (string_char_to_byte (string, pos));
	}
      if ! STRING_MULTIBYTE (string)
	{ return make_fixnum (*p as EMACS_INT); }
    }
  c = STRING_CHAR (p);
  if CHAR_BYTE8_P (c)
    { c = CHAR_TO_BYTE8 (c); }
  else if ! ASCII_CHAR_P (c as isize)
    { error (c"Not an ASCII nor an 8-bit character: %d".as_ptr (), c); }
  return make_fixnum (c as EMACS_INT);
});


/* Return true if C is an alphabetic character.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alphabeticp (c: c_int) -> bool
{
  let category: Lisp_Object = CHAR_TABLE_REF (Vunicode_category_table, c as isize);
  if ! FIXNUMP (category)
    { return false; }
  let gen_cat: EMACS_INT = XFIXNUM (category);

  /* See UTS #18.  There are additional characters that should be
     here, those designated as Other_uppercase, Other_lowercase,
     and Other_alphabetic; FIXME.  */
  return gen_cat == UNICODE_CATEGORY_Lu as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Ll as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Lt as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Lm as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Lo as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Mn as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Mc as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Me as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Nl as EMACS_INT;
}

/* Return true if C is an alphabetic or decimal-number character.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alphanumericp (c: c_int) -> bool
{
  let category: Lisp_Object = CHAR_TABLE_REF (Vunicode_category_table, c as isize);
  if ! FIXNUMP (category)
    { return false; }
  let gen_cat: EMACS_INT = XFIXNUM (category);

  /* See UTS #18.  Same comment as for alphabeticp applies.  FIXME. */
  return gen_cat == UNICODE_CATEGORY_Lu as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Ll as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Lt as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Lm as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Lo as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Mn as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Mc as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Me as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Nl as EMACS_INT
	 || gen_cat == UNICODE_CATEGORY_Nd as EMACS_INT;
}

/* Return true if C is a graphic character.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn graphicp (c: c_int) -> bool
{
  let category: Lisp_Object = CHAR_TABLE_REF (Vunicode_category_table, c as isize);
  if ! FIXNUMP (category)
    { return false; }
  let gen_cat: EMACS_INT = XFIXNUM (category);

  /* See UTS #18.  */
  return !(gen_cat == UNICODE_CATEGORY_Zs as EMACS_INT /* space separator */
	   || gen_cat == UNICODE_CATEGORY_Zl as EMACS_INT /* line separator */
	   || gen_cat == UNICODE_CATEGORY_Zp as EMACS_INT /* paragraph separator */
	   || gen_cat == UNICODE_CATEGORY_Cc as EMACS_INT /* control */
	   || gen_cat == UNICODE_CATEGORY_Cs as EMACS_INT /* surrogate */
	   || gen_cat == UNICODE_CATEGORY_Cn as EMACS_INT); /* unassigned */
}

/* Return true if C is a printable character.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn printablep (c: c_int) -> bool
{
  let category: Lisp_Object = CHAR_TABLE_REF (Vunicode_category_table, c as isize);
  if ! FIXNUMP (category)
    { return false; }
  let gen_cat: EMACS_INT = XFIXNUM (category);

  /* See UTS #18.  */
  return !(gen_cat == UNICODE_CATEGORY_Cc as EMACS_INT /* control */
	   || gen_cat == UNICODE_CATEGORY_Cs as EMACS_INT /* surrogate */
	   || gen_cat == UNICODE_CATEGORY_Cn as EMACS_INT); /* unassigned */
}

/* Return true if C is graphic character that can be printed independently.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn graphic_base_p (c: c_int) -> bool
{
  let category: Lisp_Object = CHAR_TABLE_REF (Vunicode_category_table, c as isize);
  if ! FIXNUMP (category)
    { return false; }
  let gen_cat: EMACS_INT = XFIXNUM (category);

  return !(gen_cat == UNICODE_CATEGORY_Mn as EMACS_INT       /* mark, nonspacing */
	   || gen_cat == UNICODE_CATEGORY_Mc as EMACS_INT    /* mark, combining */
	   || gen_cat == UNICODE_CATEGORY_Me as EMACS_INT    /* mark, enclosing */
	   || gen_cat == UNICODE_CATEGORY_Zs as EMACS_INT    /* separator, space */
	   || gen_cat == UNICODE_CATEGORY_Zl as EMACS_INT    /* separator, line */
	   || gen_cat == UNICODE_CATEGORY_Zp as EMACS_INT    /* separator, paragraph */
	   || gen_cat == UNICODE_CATEGORY_Cc as EMACS_INT    /* other, control */
	   || gen_cat == UNICODE_CATEGORY_Cs as EMACS_INT    /* other, surrogate */
	   || gen_cat == UNICODE_CATEGORY_Cf as EMACS_INT    /* other, format */
	   || gen_cat == UNICODE_CATEGORY_Cn as EMACS_INT);  /* other, unassigned */
}

/* Return true if C is a horizontal whitespace character, as defined
   by https://www.unicode.org/reports/tr18/tr18-19.html#blank.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn blankp (c: c_int) -> bool
{
  let category: Lisp_Object = CHAR_TABLE_REF (Vunicode_category_table, c as isize);
  if ! FIXNUMP (category)
    { return false; }

  return XFIXNUM (category) == UNICODE_CATEGORY_Zs as EMACS_INT; /* separator, space */
}

/* hexdigit[C] is one greater than C's numeric value if C is a
   hexadecimal digit, zero otherwise.  */
pub static hexdigit: [c_char; 256] =
  {
    let mut t = [0 as c_char; 256];
    t[b'0' as usize] = 1 + 0;
    t[b'1' as usize] = 1 + 1;
    t[b'2' as usize] = 1 + 2;
    t[b'3' as usize] = 1 + 3;
    t[b'4' as usize] = 1 + 4;
    t[b'5' as usize] = 1 + 5;
    t[b'6' as usize] = 1 + 6;
    t[b'7' as usize] = 1 + 7;
    t[b'8' as usize] = 1 + 8;
    t[b'9' as usize] = 1 + 9;
    t[b'A' as usize] = 1 + 10;
    t[b'B' as usize] = 1 + 11;
    t[b'C' as usize] = 1 + 12;
    t[b'D' as usize] = 1 + 13;
    t[b'E' as usize] = 1 + 14;
    t[b'F' as usize] = 1 + 15;
    t[b'a' as usize] = 1 + 10;
    t[b'b' as usize] = 1 + 11;
    t[b'c' as usize] = 1 + 12;
    t[b'd' as usize] = 1 + 13;
    t[b'e' as usize] = 1 + 14;
    t[b'f' as usize] = 1 + 15;
    t
  };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_character ()
{
  DEFSYM! (Qcharacterp, "characterp");
  DEFSYM! (Qauto_fill_chars, "auto-fill-chars");

  staticpro (&raw const Vchar_unify_table);
  Vchar_unify_table = Qnil ();

  defsubr (&raw mut Smax_char);
  defsubr (&raw mut Scharacterp);
  defsubr (&raw mut Sunibyte_char_to_multibyte);
  defsubr (&raw mut Smultibyte_char_to_unibyte);
  defsubr (&raw mut Schar_width);
  defsubr (&raw mut Sstring_width);
  defsubr (&raw mut Sstring);
  defsubr (&raw mut Sunibyte_string);
  defsubr (&raw mut Schar_resolve_modifiers);
  defsubr (&raw mut Sget_byte);

  DEFVAR_LISP! ("translation-table-vector", Vtranslation_table_vector,
		"Vector recording all translation tables ever defined.\nEach element is a pair (SYMBOL . TABLE) relating the table to the\nsymbol naming it.  The ID of a translation table is an index into this vector.");
  Vtranslation_table_vector = make_nil_vector (16);

  DEFVAR_LISP! ("auto-fill-chars", Vauto_fill_chars,
		"A char-table for characters which invoke auto-filling.\nSuch characters have the value t in this table.");
  Vauto_fill_chars = Fmake_char_table (Qauto_fill_chars (), Qnil ());
  CHAR_TABLE_SET (Vauto_fill_chars, b' ' as isize, Qt ());
  CHAR_TABLE_SET (Vauto_fill_chars, b'\n' as isize, Qt ());

  DEFVAR_LISP! ("char-width-table", Vchar_width_table,
		"A char-table for width (columns) of each character.");
  Vchar_width_table = Fmake_char_table (Qnil (), make_fixnum (1));
  char_table_set_range (Vchar_width_table, 0x80, 0x9F, make_fixnum (4));
  char_table_set_range (Vchar_width_table, MAX_5_BYTE_CHAR + 1, MAX_CHAR,
			make_fixnum (4));

  DEFVAR_LISP! ("ambiguous-width-chars", Vambiguous_width_chars,
		"A char-table for characters whose width (columns) can be 1 or 2.\n\nThe actual width depends on the language-environment and on the\nvalue of `cjk-ambiguous-chars-are-wide'.");
  Vambiguous_width_chars = Fmake_char_table (Qnil (), Qnil ());

  DEFVAR_LISP! ("printable-chars", Vprintable_chars,
		"A char-table for printable characters.\nSuch characters have the value t in this table.");
  Vprintable_chars = Fmake_char_table (Qnil (), Qnil ());
  Fset_char_table_range (Vprintable_chars,
			 Fcons (make_fixnum (32), make_fixnum (126)), Qt ());
  Fset_char_table_range (Vprintable_chars,
			 Fcons (make_fixnum (160),
				make_fixnum (MAX_5_BYTE_CHAR as EMACS_INT)), Qt ());

  DEFVAR_LISP! ("char-script-table", Vchar_script_table,
		"Char table of script symbols.\nIt has one extra slot whose value is a list of script symbols.");

  DEFSYM! (Qchar_script_table, "char-script-table");
  Fput (Qchar_script_table (), Qchar_table_extra_slots (), make_fixnum (1));
  Vchar_script_table = Fmake_char_table (Qchar_script_table (), Qnil ());

  DEFVAR_LISP! ("script-representative-chars", Vscript_representative_chars,
		"Alist of scripts vs the representative characters.\nEach element is a cons (SCRIPT . CHARS).\nSCRIPT is a symbol representing a script or a subgroup of a script.\nCHARS is a list or a vector of characters.\nIf it is a list, all characters in the list are necessary for supporting SCRIPT.\nIf it is a vector, one of the characters in the vector is necessary.\nThis variable is used to find a font for a specific script.");
  Vscript_representative_chars = Qnil ();

  DEFVAR_LISP! ("unicode-category-table", Vunicode_category_table,
		"Char table of Unicode's \"General Category\".\nAll Unicode characters have one of the following values (symbol):\n  Lu, Ll, Lt, Lm, Lo, Mn, Mc, Me, Nd, Nl, No, Pc, Pd, Ps, Pe, Pi, Pf, Po,\n  Sm, Sc, Sk, So, Zs, Zl, Zp, Cc, Cf, Cs, Co, Cn\nSee The Unicode Standard for the meaning of those values.");
  /* The correct char-table is setup in characters.el.  */
  Vunicode_category_table = Qnil ();
}
