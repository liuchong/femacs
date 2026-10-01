// femacs: port of src/character.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// Header mirror: constants, inline functions and `unsafe extern "C"` declarations
// (PORTING.md 0.1).  The inline functions keep C's bodies; `eassert`/`eassume` and
// the accessors come from lisp.rs, as they do from lisp.h in C.

/* Copyright (C) 1991-1993, 1995, 1997-1999, 2001-2026 Free Software
   Foundation, Inc.

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
pub const EMACS_CHARACTER_H: bool = true;

/* character code	1st byte   byte sequence
   --------------	--------   -------------
        0-7F		00..7F	   0xxxxxxx
       80-7FF		C2..DF	   110yyyyx 10xxxxxx
      800-FFFF		E0..EF	   1110yyyy 10yxxxxx 10xxxxxx
    10000-1FFFFF	F0..F7	   11110yyy 10yyxxxx 10xxxxxx 10xxxxxx
   200000-3FFF7F	F8	   11111000 1000yxxx 10xxxxxx 10xxxxxx 10xxxxxx
   3FFF80-3FFFFF	C0..C1	   1100000x 10xxxxxx (for eight-bit-char)
   400000-...		invalid

   invalid 1st byte	80..BF	   10xxxxxx
			F9..FF	   11111yyy

   In each bit pattern, 'x' and 'y' each represent a single bit of the
   character code payload, and at least one 'y' must be a 1 bit.
   In the 5-byte sequence, the 22-bit payload cannot exceed 3FFF7F.
*/

/* Maximum character code ((1 << CHARACTERBITS) - 1).  */
pub const MAX_CHAR: c_int = 0x3FFFFF;

/* Maximum Unicode character code.  */
pub const MAX_UNICODE_CHAR: c_int = 0x10FFFF;

/* Maximum N-byte character codes.  */
pub const MAX_1_BYTE_CHAR: c_int = 0x7F;
pub const MAX_2_BYTE_CHAR: c_int = 0x7FF;
pub const MAX_3_BYTE_CHAR: c_int = 0xFFFF;
pub const MAX_4_BYTE_CHAR: c_int = 0x1FFFFF;
pub const MAX_5_BYTE_CHAR: c_int = 0x3FFF7F;

/* Minimum leading code of multibyte characters.  */
pub const MIN_MULTIBYTE_LEADING_CODE: c_int = 0xC0;
/* Maximum leading code of multibyte characters.  Note: this must be
   updated if we ever increase MAX_CHAR above.  */
pub const MAX_MULTIBYTE_LEADING_CODE: c_int = 0xF8;

/* Unicode character values.  */
pub const NO_BREAK_SPACE: c_int = 0x00A0;
pub const SOFT_HYPHEN: c_int = 0x00AD;
pub const ZERO_WIDTH_NON_JOINER: c_int = 0x200C;
pub const ZERO_WIDTH_JOINER: c_int = 0x200D;
pub const HYPHEN: c_int = 0x2010;
pub const NON_BREAKING_HYPHEN: c_int = 0x2011;
pub const LEFT_SINGLE_QUOTATION_MARK: c_int = 0x2018;
pub const RIGHT_SINGLE_QUOTATION_MARK: c_int = 0x2019;
pub const PARAGRAPH_SEPARATOR: c_int = 0x2029;
pub const LEFT_POINTING_ANGLE_BRACKET: c_int = 0x2329;
pub const RIGHT_POINTING_ANGLE_BRACKET: c_int = 0x232A;
pub const LEFT_ANGLE_BRACKET: c_int = 0x3008;
pub const RIGHT_ANGLE_BRACKET: c_int = 0x3009;
pub const OBJECT_REPLACEMENT_CHARACTER: c_int = 0xFFFC;
pub const TAG_SPACE: c_int = 0xE0020;
pub const CANCEL_TAG: c_int = 0xE007F;

unsafe extern "C" {
  pub fn char_string (c: c_uint, p: *mut u8) -> c_int;
}

/* UTF-8 encodings.  Use \x escapes, so they are portable to pre-C11
   compilers and can be concatenated with ordinary string literals.  */
// C spells these as `"\xE2\x80\x98"` / `"\xE2\x80\x99"`; Rust spells the same
// code point, and `--check` compares the decoded bytes (tools/extract-abi.py).
pub const uLSQM: &str = "\u{2018}"; /* U+2018 LEFT SINGLE QUOTATION MARK */
pub const uRSQM: &str = "\u{2019}"; /* U+2019 RIGHT SINGLE QUOTATION MARK */

/* True iff C is a character of code less than 0x100.  */
#[inline(always)]
pub fn SINGLE_BYTE_CHAR_P (c: intmax_t) -> bool
{
  return 0 <= c && c < 0x100;
}

/* True iff C is a character that corresponds to a raw 8-bit
   byte.  */
#[inline(always)]
pub fn CHAR_BYTE8_P (c: c_int) -> bool
{
  return MAX_5_BYTE_CHAR < c;
}

/* Return the character code for raw 8-bit byte BYTE.  */
#[inline(always)]
pub fn BYTE8_TO_CHAR (byte: c_int) -> c_int
{
  return byte + 0x3FFF00;
}

#[inline(always)]
pub fn UNIBYTE_TO_CHAR (byte: c_int) -> c_int
{
  return if ASCII_CHAR_P (byte as isize) { byte } else { BYTE8_TO_CHAR (byte) };
}

/* Return the raw 8-bit byte for character C.  */
#[inline(always)]
pub fn CHAR_TO_BYTE8 (c: c_int) -> c_int
{
  return if CHAR_BYTE8_P (c) { c - 0x3FFF00 } else { c & 0xFF };
}

/* Return the raw 8-bit byte for character C,
   or -1 if C doesn't correspond to a byte.  */
#[inline(always)]
pub fn CHAR_TO_BYTE_SAFE (c: c_int) -> c_int
{
  return if ASCII_CHAR_P (c as isize) { c }
         else if CHAR_BYTE8_P (c) { c - 0x3FFF00 } else { -1 };
}

/* True iff BYTE is the 1st byte of a multibyte form of a character
   that corresponds to a raw 8-bit byte.  */
#[inline(always)]
pub fn CHAR_BYTE8_HEAD_P (byte: c_int) -> bool
{
  return byte == 0xC0 || byte == 0xC1;
}

/* If C is not ASCII, make it multibyte.  Assumes C < 256.  */
#[inline(always)]
pub fn make_char_multibyte (c: c_int) -> c_int
{
  eassert! (SINGLE_BYTE_CHAR_P (c as intmax_t));
  return UNIBYTE_TO_CHAR (c);
}

/* This is the maximum byte length of multibyte form.  */
pub const MAX_MULTIBYTE_LENGTH: c_int = 5;

/* Nonzero iff C is valid as a character code.  */
#[inline(always)]
pub fn CHAR_VALID_P (c: intmax_t) -> bool
{
  return 0 <= c && c <= MAX_CHAR as intmax_t;
}

/* Nonzero iff X is a character.  */
#[inline(always)]
pub fn CHARACTERP (x: Lisp_Object) -> bool
{
  return FIXNUMP (x) && CHAR_VALID_P (XFIXNUM (x) as intmax_t);
}

/* Check if Lisp object X is a character or not.  */
#[inline(always)]
pub fn CHECK_CHARACTER (x: Lisp_Object)
{
  CHECK_TYPE (CHARACTERP (x), Qcharacterp (), x);
}

#[inline(always)]
pub fn CHECK_CHARACTER_CAR (x: Lisp_Object)
{
  CHECK_CHARACTER (XCAR (x));
}

#[inline(always)]
pub fn CHECK_CHARACTER_CDR (x: Lisp_Object)
{
  CHECK_CHARACTER (XCDR (x));
}

/* True if character C has a printable glyph.  */
#[inline(always)]
pub fn CHAR_PRINTABLE_P (c: c_int) -> bool
{
  return (32 <= c && c < 127)
	 || ! NILP (CHAR_TABLE_REF (unsafe { Vprintable_chars }, c as isize));
}

/* Return byte length of multibyte form for character C.  */
#[inline(always)]
pub fn CHAR_BYTES (c: c_int) -> c_int
{
  return (if MAX_5_BYTE_CHAR < c { -2 } else { 1 })
	 + (MAX_1_BYTE_CHAR < c) as c_int
	 + (MAX_2_BYTE_CHAR < c) as c_int
	 + (MAX_3_BYTE_CHAR < c) as c_int
	 + (MAX_4_BYTE_CHAR < c) as c_int;
}

/* Return the leading code of multibyte form of C.  */
#[inline(always)]
pub fn CHAR_LEADING_CODE (c: c_int) -> c_int
{
  return if c <= MAX_1_BYTE_CHAR { c }
	 else if c <= MAX_2_BYTE_CHAR { 0xC0 | (c >> 6) }
	 else if c <= MAX_3_BYTE_CHAR { 0xE0 | (c >> 12) }
	 else if c <= MAX_4_BYTE_CHAR { 0xF0 | (c >> 18) }
	 else if c <= MAX_5_BYTE_CHAR { 0xF8 }
	 else { 0xC0 | ((c >> 6) & 0x01) };
}


/* Store multibyte form of the character C in P.  The caller should
   allocate at least MAX_MULTIBYTE_LENGTH bytes area at P in advance.
   Returns the length of the multibyte form.  */

#[inline(always)]
pub fn CHAR_STRING (c: c_int, p: *mut u8) -> c_int
{
  eassume! (0 <= c);
  if c <= MAX_1_BYTE_CHAR
    {
      unsafe { *p.offset (0) = c as u8; }
      return 1;
    }
  if c <= MAX_2_BYTE_CHAR
    {
      unsafe {
	*p.offset (0) = (0xC0 | (c >> 6)) as u8;
	*p.offset (1) = (0x80 | (c & 0x3F)) as u8;
      }
      return 2;
    }
  if c <= MAX_3_BYTE_CHAR
    {
      unsafe {
	*p.offset (0) = (0xE0 | (c >> 12)) as u8;
	*p.offset (1) = (0x80 | ((c >> 6) & 0x3F)) as u8;
	*p.offset (2) = (0x80 | (c & 0x3F)) as u8;
      }
      return 3;
    }
  let len = unsafe { char_string (c as c_uint, p) };
  eassume! (0 < len && len <= MAX_MULTIBYTE_LENGTH);
  return len;
}

/* Store multibyte form of byte B in P.  The caller should allocate at
   least MAX_MULTIBYTE_LENGTH bytes area at P in advance.  Returns the
   length of the multibyte form.  */

#[inline(always)]
pub fn BYTE8_STRING (b: c_int, p: *mut u8) -> c_int
{
  unsafe {
    *p.offset (0) = (0xC0 | ((b >> 6) & 0x01)) as u8;
    *p.offset (1) = (0x80 | (b & 0x3F)) as u8;
  }
  return 2;
}


/* True iff BYTE starts a non-ASCII character in a multibyte form.  */
#[inline(always)]
pub fn LEADING_CODE_P (byte: c_int) -> bool
{
  return (byte & 0xC0) == 0xC0;
}

/* True iff BYTE is a trailing code of a non-ASCII character in a
   multibyte form.  */
#[inline(always)]
pub fn TRAILING_CODE_P (byte: c_int) -> bool
{
  return (byte & 0xC0) == 0x80;
}

/* True iff BYTE starts a character in a multibyte form.
   This is equivalent to:
	(ASCII_CHAR_P (byte) || LEADING_CODE_P (byte))  */
#[inline(always)]
pub fn CHAR_HEAD_P (byte: c_int) -> bool
{
  return (byte & 0xC0) != 0x80;
}

/* How many bytes a character that starts with BYTE occupies in a
   multibyte form.  Unlike multibyte_length, this function does not
   validate the multibyte form, but looks only at its first byte.  */
#[inline(always)]
pub fn BYTES_BY_CHAR_HEAD (byte: c_int) -> c_int
{
  return if (byte & 0x80) == 0 { 1 }
	 else if (byte & 0x20) == 0 { 2 }
	 else if (byte & 0x10) == 0 { 3 }
	 else if (byte & 0x08) == 0 { 4 }
	 else { 5 };
}


/* The byte length of the multibyte form at the unibyte string P,
   ending at PEND if CHECK, and without a length check if !CHECK.
   If ALLOW_8BIT, allow multibyte forms of eight-bit characters.
   If the string doesn't point to a valid multibyte form, return 0.
   Unlike BYTES_BY_CHAR_HEAD, this function validates the multibyte form.  */

#[inline(always)]
pub fn multibyte_length (p: *const u8, pend: *const u8, check: bool, allow_8bit: bool) -> c_int
{
  if !check || p < pend
    {
      let c: c_int = unsafe { *p } as c_int;
      if c < 0x80
	{ return 1; }
      if !check || unsafe { p.offset (1) } < pend
	{
	  let d: c_int = unsafe { *p.offset (1) } as c_int;
	  let mut w: c_int = ((d & 0xC0) << 2) + c;
	  if (if allow_8bit { 0x2C0 } else { 0x2C2 }) <= w && w <= 0x2DF
	    { return 2; }
	  if !check || unsafe { p.offset (2) } < pend
	    {
	      let e: c_int = unsafe { *p.offset (2) } as c_int;
	      w += (e & 0xC0) << 4;
	      let w1: c_int = w | ((d & 0x20) >> 2);
	      if 0xAE1 <= w1 && w1 <= 0xAEF
		{ return 3; }
	      if !check || unsafe { p.offset (3) } < pend
		{
		  let f: c_int = unsafe { *p.offset (3) } as c_int;
		  w += (f & 0xC0) << 6;
		  let w2: c_int = w | ((d & 0x30) >> 3);
		  if 0x2AF1 <= w2 && w2 <= 0x2AF7
		    { return 4; }
		  if !check || unsafe { p.offset (4) } < pend
		    {
		      let lw: int_fast64_t = (w as int_fast64_t)
			+ (((unsafe { *p.offset (4) } as c_int) & 0xC0) as int_fast64_t) << 8;
		      let w3: int_fast64_t = (lw << 24) + ((d as int_fast64_t) << 16)
			                     + ((e as int_fast64_t) << 8) + f as int_fast64_t;
		      if 0xAAF8888080 <= w3 && w3 <= 0xAAF88FBFBD
			{ return 5; }
		    }
		}
	    }
	}
    }

  return 0;
}


/* Return number of bytes in the multibyte character just before P.
   Assumes that P is already at a character boundary of the same
   multibyte form, and is not at the start of that form.  */

#[inline(always)]
pub fn raw_prev_char_len (p: *const u8) -> c_int
{
  let mut len: c_int = 1;
  loop
    {
      if CHAR_HEAD_P (unsafe { *p.offset (-(len as isize)) } as c_int)
	{ return len; }
      len += 1;
    }
}


/* Return the character code of character whose multibyte form is at P,
   and set *LENGTH to its length.  */

#[inline(always)]
pub fn string_char_and_length (p: *const u8, length: *mut c_int) -> c_int
{
  let c: c_int = unsafe { *p } as c_int;
  if c & 0x80 == 0
    {
      unsafe { *length = 1; }
      return c;
    }
  eassume! (0xC0 <= c);

  let mut d: c_int = (c << 6) + (unsafe { *p.offset (1) } as c_int) - ((0xC0 << 6) + 0x80);
  if c & 0x20 == 0
    {
      unsafe { *length = 2; }
      return d + (if c < 0xC2 { 0x3FFF80 } else { 0 });
    }

  d = (d << 6) + (unsafe { *p.offset (2) } as c_int) - ((0x20 << 12) + 0x80);
  if c & 0x10 == 0
    {
      unsafe { *length = 3; }
      eassume! (MAX_2_BYTE_CHAR < d && d <= MAX_3_BYTE_CHAR);
      return d;
    }

  d = (d << 6) + (unsafe { *p.offset (3) } as c_int) - ((0x10 << 18) + 0x80);
  if c & 0x08 == 0
    {
      unsafe { *length = 4; }
      eassume! (MAX_3_BYTE_CHAR < d && d <= MAX_4_BYTE_CHAR);
      return d;
    }

  d = (d << 6) + (unsafe { *p.offset (4) } as c_int) - ((0x08 << 24) + 0x80);
  unsafe { *length = 5; }
  eassume! (MAX_4_BYTE_CHAR < d && d <= MAX_5_BYTE_CHAR);
  return d;
}

/* Return the character code of character whose multibyte form is at P.  */

#[inline(always)]
pub fn STRING_CHAR (p: *const u8) -> c_int
{
  let mut len: c_int = 0;
  return string_char_and_length (p, &raw mut len);
}


/* Like STRING_CHAR (*PP), but advance *PP to the end of multibyte form.  */

#[inline(always)]
pub fn string_char_advance (pp: *mut *const u8) -> c_int
{
  let p: *const u8 = unsafe { *pp };
  let mut len: c_int = 0;
  let c = string_char_and_length (p, &raw mut len);
  unsafe { *pp = p.offset (len as isize); }
  return c;
}


/* Return the next character from Lisp string STRING at byte position
   *BYTEIDX, character position *CHARIDX.  Update *BYTEIDX and
   *CHARIDX past the character fetched.  */

#[inline(always)]
pub fn fetch_string_char_advance (string: Lisp_Object, charidx: *mut ptrdiff_t,
				  byteidx: *mut ptrdiff_t) -> c_int
{
  let output: c_int;
  let mut b: ptrdiff_t = unsafe { *byteidx };
  let chp: *mut u8 = unsafe { SDATA (string).offset (b) };
  if STRING_MULTIBYTE (string)
    {
      let mut chlen: c_int = 0;
      output = string_char_and_length (chp, &raw mut chlen);
      b += chlen as ptrdiff_t;
    }
  else
    {
      output = unsafe { *chp } as c_int;
      b += 1;
    }
  unsafe { *charidx += 1; }
  unsafe { *byteidx = b; }
  return output;
}

/* Like fetch_string_char_advance, but return a multibyte character
   even if STRING is unibyte.  */

#[inline(always)]
pub fn fetch_string_char_as_multibyte_advance (string: Lisp_Object, charidx: *mut ptrdiff_t,
					       byteidx: *mut ptrdiff_t) -> c_int
{
  let output: c_int;
  let mut b: ptrdiff_t = unsafe { *byteidx };
  let chp: *mut u8 = unsafe { SDATA (string).offset (b) };
  if STRING_MULTIBYTE (string)
    {
      let mut chlen: c_int = 0;
      output = string_char_and_length (chp, &raw mut chlen);
      b += chlen as ptrdiff_t;
    }
  else
    {
      output = make_char_multibyte (unsafe { *chp } as c_int);
      b += 1;
    }
  unsafe { *charidx += 1; }
  unsafe { *byteidx = b; }
  return output;
}


/* Like fetch_string_char_advance, but assumes STRING is multibyte.  */

#[inline(always)]
pub fn fetch_string_char_advance_no_check (string: Lisp_Object, charidx: *mut ptrdiff_t,
					   byteidx: *mut ptrdiff_t) -> c_int
{
  let b: ptrdiff_t = unsafe { *byteidx };
  let chp: *mut u8 = unsafe { SDATA (string).offset (b) };
  let mut chlen: c_int = 0;
  let output: c_int = string_char_and_length (chp, &raw mut chlen);
  unsafe { *charidx += 1; }
  unsafe { *byteidx = b + chlen as ptrdiff_t; }
  return output;
}


/* If C is a variation selector, return the index of the
   variation selector (1..256).  Otherwise, return 0.  */

#[inline(always)]
pub fn CHAR_VARIATION_SELECTOR_P (c: c_int) -> c_int
{
  return if c < 0xFE00 { 0 }
	 else if c <= 0xFE0F { c - 0xFE00 + 1 }
	 else if c < 0xE0100 { 0 }
	 else if c <= 0xE01EF { c - 0xE0100 + 17 }
	 else { 0 };
}

/* Return true if C is a surrogate.  */

#[inline(always)]
pub fn char_surrogate_p (c: c_int) -> bool
{
  return 0xD800 <= c && c <= 0xDFFF;
}

/* Data type for Unicode general category.

   The order of members must be in sync with the 8th element of the
   member of unidata-prop-alist (in admin/unidata/unidata-gen.el) for
   Unicode character property `general-category'.  */

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum unicode_category_t
  {
    UNICODE_CATEGORY_UNKNOWN = 0,
    UNICODE_CATEGORY_Lu,
    UNICODE_CATEGORY_Ll,
    UNICODE_CATEGORY_Lt,
    UNICODE_CATEGORY_Lm,
    UNICODE_CATEGORY_Lo,
    UNICODE_CATEGORY_Mn,
    UNICODE_CATEGORY_Mc,
    UNICODE_CATEGORY_Me,
    UNICODE_CATEGORY_Nd,
    UNICODE_CATEGORY_Nl,
    UNICODE_CATEGORY_No,
    UNICODE_CATEGORY_Pc,
    UNICODE_CATEGORY_Pd,
    UNICODE_CATEGORY_Ps,
    UNICODE_CATEGORY_Pe,
    UNICODE_CATEGORY_Pi,
    UNICODE_CATEGORY_Pf,
    UNICODE_CATEGORY_Po,
    UNICODE_CATEGORY_Sm,
    UNICODE_CATEGORY_Sc,
    UNICODE_CATEGORY_Sk,
    UNICODE_CATEGORY_So,
    UNICODE_CATEGORY_Zs,
    UNICODE_CATEGORY_Zl,
    UNICODE_CATEGORY_Zp,
    UNICODE_CATEGORY_Cc,
    UNICODE_CATEGORY_Cf,
    UNICODE_CATEGORY_Cs,
    UNICODE_CATEGORY_Co,
    UNICODE_CATEGORY_Cn
  }

unsafe extern "C" {
  // Parameter names come from the definitions in character.c (C's prototypes
  // often have none).
  pub fn char_resolve_modifier_mask (c: EMACS_INT) -> EMACS_INT;
  pub fn translate_char (table: Lisp_Object, c: c_int) -> c_int;
  pub fn count_size_as_multibyte (str: *const u8, len: ptrdiff_t) -> ptrdiff_t;
  pub fn str_as_multibyte (str: *mut u8, len: ptrdiff_t, nbytes: ptrdiff_t,
			   nchars: *mut ptrdiff_t) -> ptrdiff_t;
  pub fn str_to_multibyte (dst: *mut u8, src: *const u8, nchars: ptrdiff_t) -> ptrdiff_t;
  pub fn str_as_unibyte (str: *mut u8, bytes: ptrdiff_t) -> ptrdiff_t;
  pub fn strwidth (str: *const c_char, len: ptrdiff_t) -> ptrdiff_t;
  pub fn c_string_width (str: *const u8, len: ptrdiff_t, precision: c_int,
			 nchars: *mut ptrdiff_t, nbytes: *mut ptrdiff_t) -> ptrdiff_t;
  pub fn lisp_string_width (string: Lisp_Object, from: ptrdiff_t, to: ptrdiff_t,
			    precision: ptrdiff_t, nchars: *mut ptrdiff_t,
			    nbytes: *mut ptrdiff_t, auto_comp: bool) -> ptrdiff_t;

  pub static mut Vchar_unify_table: Lisp_Object;
  pub fn string_escape_byte8 (string: Lisp_Object) -> Lisp_Object;

  pub fn alphabeticp (c: c_int) -> bool;
  pub fn alphanumericp (c: c_int) -> bool;
  pub fn graphicp (c: c_int) -> bool;
  pub fn printablep (c: c_int) -> bool;
  pub fn blankp (c: c_int) -> bool;
  pub fn graphic_base_p (c: c_int) -> bool;

  /// C declares `extern signed char const hexdigit[];` (an incomplete array);
  /// the size is the definition's `hexdigit[UCHAR_MAX + 1]` in character.c.
  pub static hexdigit: [c_char; 256];
}

/* Look up the element in char table OBJ at index CH, and return it as
   an integer.  If the element is not a character, return CH itself.  */

#[inline(always)]
pub fn char_table_translate (obj: Lisp_Object, ch: c_int) -> c_int
{
  /* This internal function is expected to be called with valid arguments,
     so there is an eassert instead of CHECK_xxx for the sake of speed.  */
  eassert! (CHAR_VALID_P (ch as intmax_t));
  eassert! (CHAR_TABLE_P (obj));
  let obj = CHAR_TABLE_REF (obj, ch as isize);
  return if CHARACTERP (obj) { XFIXNUM (obj) as c_int } else { ch };
}

/* If C is a hexadecimal digit ('0'-'9', 'a'-'f', 'A'-'F'), return its
   value (0-15).  Otherwise return -1.  */

#[inline(always)]
pub fn char_hexdigit (c: c_int) -> c_int
{
  return if 0 <= c && c <= UCHAR_MAX { (unsafe { hexdigit[c as usize] }) as c_int - 1 }
	 else { -1 };
}
