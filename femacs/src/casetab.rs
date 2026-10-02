// femacs: port of src/casetab.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// One-to-one port: same statements in the same order, same names (PORTING.md).  The five
// subrs use `DEFUN!` (C's `DEFUN`, PORTING.md 5.1); a flexible array member is reached
// with `FAM_ELEM!` (PORTING.md 1.3); the two globals this unit defines shadow the
// declarations lisp.rs carries for them (P-21, P-39).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]

/* GNU Emacs routines to deal with case tables.
   Copyright (C) 1993-1994, 2001-2026 Free Software Foundation, Inc.

Author: Howard Gayle

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

include!("include/config.rs");
include!("include/lisp.rs");
include!("include/character.rs");
include!("include/itree.rs");
include!("include/buffer.rs");

#[unsafe(no_mangle)]
pub static mut Vascii_downcase_table: Lisp_Object = lisp_h_Qnil ();
static mut Vascii_upcase_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vascii_canon_table: Lisp_Object = lisp_h_Qnil ();
static mut Vascii_eqv_table: Lisp_Object = lisp_h_Qnil ();

// C's forward declarations of the callbacks (casetab.c 33-35) have no counterpart here;
// the callbacks are private `unsafe extern "C" fn`s so map_char_table can take their address.
DEFUN! ("case-table-p", Fcase_table_p, Scase_table_p, 1, 1, 0,
       doc: "Return t if OBJECT is a case table.\nSee `set-case-table' for more information on these data structures.",
       (object: Lisp_Object),
       {
  let up: Lisp_Object;
  let canon: Lisp_Object;
  let eqv: Lisp_Object;

  if ! CHAR_TABLE_P (object)
    { return Qnil (); }
  if ! EQ (unsafe { (*XCHAR_TABLE (object)).purpose }, Qcase_table ())
    { return Qnil (); }

  up = unsafe { FAM_ELEM! ((*XCHAR_TABLE (object)).extras, 0) };
  canon = unsafe { FAM_ELEM! ((*XCHAR_TABLE (object)).extras, 1) };
  eqv = unsafe { FAM_ELEM! ((*XCHAR_TABLE (object)).extras, 2) };

  return if (NILP (up) || CHAR_TABLE_P (up))
    && ((NILP (canon) && NILP (eqv))
 	|| (CHAR_TABLE_P (canon)
 	    && (NILP (eqv) || CHAR_TABLE_P (eqv))))
    { Qt () } else { Qnil () };
});

fn check_case_table (obj: Lisp_Object) -> Lisp_Object
{
  CHECK_TYPE (!NILP (unsafe { Fcase_table_p (obj) }), Qcase_table_p (), obj);
  return obj;
}

DEFUN! ("current-case-table", Fcurrent_case_table, Scurrent_case_table, 0, 0, 0,
       doc: "Return the case table of the current buffer.",
       (),
       {
  return unsafe { BVAR! (current_buffer, downcase_table_) };
});

DEFUN! ("standard-case-table", Fstandard_case_table, Sstandard_case_table, 0, 0, 0,
       doc: "Return the standard case table.\nThis is the one used for new buffers.",
       (),
       {
  return unsafe { Vascii_downcase_table };
});

DEFUN! ("set-case-table", Fset_case_table, Sset_case_table, 1, 1, 0,
       doc: "Select a new case table for the current buffer.\nA case table is a char-table which maps characters\nto their lower-case equivalents.  It also has three \"extra\" slots\nwhich may be additional char-tables or nil.\nThese slots are called UPCASE, CANONICALIZE and EQUIVALENCES.\nUPCASE maps each non-upper-case character to its upper-case equivalent.\n (The value in UPCASE for an upper-case character is never used.)\n If lower and upper case characters are in 1-1 correspondence,\n you may use nil and the upcase table will be deduced from DOWNCASE.\nCANONICALIZE maps each character to a canonical equivalent;\n any two characters that are related by case-conversion have the same\n canonical equivalent character; it may be nil, in which case it is\n deduced from DOWNCASE and UPCASE.\nEQUIVALENCES is a map that cyclically permutes each equivalence class\n (of characters with the same canonical equivalent); it may be nil,\n in which case it is deduced from CANONICALIZE.",
       (table: Lisp_Object),
       {
  return set_case_table (table, false);
});

DEFUN! ("set-standard-case-table", Fset_standard_case_table, Sset_standard_case_table, 1, 1, 0,
       doc: "Select a new standard case table for new buffers.\nSee `set-case-table' for more info on case tables.",
       (table: Lisp_Object),
       {
  return set_case_table (table, true);
});

fn set_case_table (table: Lisp_Object, standard: bool) -> Lisp_Object
{
  let mut up: Lisp_Object;
  let mut canon: Lisp_Object;
  let mut eqv: Lisp_Object;

  check_case_table (table);

  up = unsafe { FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 0) };
  canon = unsafe { FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 1) };
  eqv = unsafe { FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 2) };

  if NILP (up)
    {
      up = unsafe { Fmake_char_table (Qcase_table (), Qnil ()) };
      unsafe { map_char_table (Some (set_identity), Qnil (), table, up); }
      unsafe { map_char_table (Some (shuffle), Qnil (), table, up); }
      set_char_table_extras (table, 0, up);
    }

  if NILP (canon)
    {
      canon = unsafe { Fmake_char_table (Qcase_table (), Qnil ()) };
      set_char_table_extras (table, 1, canon);
      unsafe { map_char_table (Some (set_canon), Qnil (), table, table); }
    }

  if NILP (eqv)
    {
      eqv = unsafe { Fmake_char_table (Qcase_table (), Qnil ()) };
      unsafe { map_char_table (Some (set_identity), Qnil (), canon, eqv); }
      unsafe { map_char_table (Some (shuffle), Qnil (), canon, eqv); }
      set_char_table_extras (table, 2, eqv);
    }

  /* This is so set_image_of_range_1 in regex-emacs.c can find the EQV
     table.  */
  set_char_table_extras (canon, 2, eqv);

  if standard
    {
      unsafe {
        Vascii_downcase_table = table;
        Vascii_upcase_table = up;
        Vascii_canon_table = canon;
        Vascii_eqv_table = eqv;
      }
    }
  else
    {
      bset_downcase_table (unsafe { current_buffer }, table);
      bset_upcase_table (unsafe { current_buffer }, up);
      bset_case_canon_table (unsafe { current_buffer }, canon);
      bset_case_eqv_table (unsafe { current_buffer }, eqv);
    }

  return table;
}

/* The following functions are called in map_char_table.  */

/* Set CANON char-table element for characters in RANGE to a
   translated ELT by UP and DOWN char-tables.  This is done only when
   ELT is a character.  The char-tables CANON, UP, and DOWN are in
   CASE_TABLE.  */

unsafe extern "C" fn set_canon (case_table: Lisp_Object, range: Lisp_Object, elt: Lisp_Object)
{
  let up = unsafe { FAM_ELEM! ((*XCHAR_TABLE (case_table)).extras, 0) };
  let canon = unsafe { FAM_ELEM! ((*XCHAR_TABLE (case_table)).extras, 1) };

  if FIXNATP (elt)
    { Fset_char_table_range (canon, range, Faref (case_table, Faref (up, elt))); }
}

/* Set elements of char-table TABLE for C to C itself.  C may be a
   cons specifying a character range.  In that case, set characters in
   that range to themselves.  This is done only when ELT is a
   character.  This is called in map_char_table.  */

unsafe extern "C" fn set_identity (table: Lisp_Object, c: Lisp_Object, elt: Lisp_Object)
{
  if FIXNATP (elt)
    {
      let mut from: c_int;
      let mut to: c_int;

      if CONSP (c)
	{
	  from = XFIXNUM (XCAR (c)) as c_int;
	  to = XFIXNUM (XCDR (c)) as c_int;
	}
      else
	{ from = XFIXNUM (c) as c_int;
	  to = from; }

      to += 1;
      while from < to
	{
	  CHAR_TABLE_SET (table, from as isize, make_fixnum (from as EMACS_INT));
	  from += 1;
	}
    }
}

/* Permute the elements of TABLE (which is initially an identity
   mapping) so that it has one cycle for each equivalence class
   induced by the translation table on which map_char_table is
   operated.  */

unsafe extern "C" fn shuffle (table: Lisp_Object, c: Lisp_Object, elt: Lisp_Object)
{
  if FIXNATP (elt)
    {
      let mut from: c_int;
      let mut to: c_int;

      if CONSP (c)
	{
	  from = XFIXNUM (XCAR (c)) as c_int;
	  to = XFIXNUM (XCDR (c)) as c_int;
	}
      else
	{ from = XFIXNUM (c) as c_int;
	  to = from; }

      to += 1;
      while from < to
	{
	  let tem = Faref (table, elt);
	  Faset (table, elt, make_fixnum (from as EMACS_INT));
	  Faset (table, make_fixnum (from as EMACS_INT), tem);
	  from += 1;
	}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_casetab_once ()
{
  let mut i: c_int;
  let mut down: Lisp_Object;
  let mut up: Lisp_Object;
  let mut eqv: Lisp_Object;

  DEFSYM! (Qcase_table, "case-table");
  Fput (Qcase_table (), Qchar_table_extra_slots (), make_fixnum (3));

  down = Fmake_char_table (Qcase_table (), Qnil ());
  Vascii_downcase_table = down;
  set_char_table_purpose (down, Qcase_table ());

  i = 0;
  while i < 128
    {
      let c: c_int = if i >= 'A' as c_int && i <= 'Z' as c_int
	{ i + ('a' as c_int - 'A' as c_int) } else { i };
      CHAR_TABLE_SET (down, i as isize, make_fixnum (c as EMACS_INT));
      i += 1;
    }

  set_char_table_extras (down, 1, Fcopy_sequence (down));

  up = Fmake_char_table (Qcase_table (), Qnil ());
  set_char_table_extras (down, 0, up);

  i = 0;
  while i < 128
    {
      let c: c_int = if i >= 'a' as c_int && i <= 'z' as c_int
	{ i + ('A' as c_int - 'a' as c_int) } else { i };
      CHAR_TABLE_SET (up, i as isize, make_fixnum (c as EMACS_INT));
      i += 1;
    }

  eqv = Fmake_char_table (Qcase_table (), Qnil ());

  i = 0;
  while i < 128
    {
      let c: c_int = if i >= 'A' as c_int && i <= 'Z' as c_int
	{ i + ('a' as c_int - 'A' as c_int) }
      else if i >= 'a' as c_int && i <= 'z' as c_int
	{ i + ('A' as c_int - 'a' as c_int) }
      else { i };
      CHAR_TABLE_SET (eqv, i as isize, make_fixnum (c as EMACS_INT));
      i += 1;
    }

  set_char_table_extras (down, 2, eqv);

  /* Fill in what isn't filled in.  */
  set_case_table (down, true);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_casetab ()
{
  DEFSYM! (Qcase_table_p, "case-table-p");

  staticpro (&raw const Vascii_canon_table);
  staticpro (&raw const Vascii_downcase_table);
  staticpro (&raw const Vascii_eqv_table);
  staticpro (&raw const Vascii_upcase_table);

  defsubr (&raw mut Scase_table_p);
  defsubr (&raw mut Scurrent_case_table);
  defsubr (&raw mut Sstandard_case_table);
  defsubr (&raw mut Sset_case_table);
  defsubr (&raw mut Sset_standard_case_table);
}
