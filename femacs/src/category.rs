// femacs: port of src/category.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The
// thirteen subrs use `DEFUN!` (PORTING.md 5.1).
//
// The `#include` list mirrors category.c 32-36 - config.h, lisp.h, character.h, buffer.h,
// category.h - with the mirrors those pull in.  category.c has no preprocessor conditional
// at all, so the whole file is live text.  `Vstandard_category_table` is category.h's
// token-pasting macro, so the port writes the BVAR! expansion it stands for (the macro name
// is in docs/ports/excluded-macros.txt).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Copyright (C) 1992, 1995, 1997-2026 Free Software Foundation, Inc.

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

/* Here we handle three objects: category, category set, and category
   table.  Read comments in the file category.h to understand them.  */

include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("include/lisp.rs");
include!("../lib/include/mini-gmp.rs");
include!("include/bignum.rs");
include!("include/character.rs");
include!("include/itree.rs");
include!("include/systime.rs");
include!("include/dispextern.rs");
include!("include/font.rs");
include!("include/composite.rs");
include!("include/coding.rs");
include!("include/buffer.rs");
include!("include/category.rs");
include!("include/keyboard.rs");
include!("include/termhooks.rs");
include!("include/window.rs");
include!("include/process.rs");
include!("include/frame.rs");
include!("include/keymap.rs");

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status §3.29, the casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vword_combining_categories: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vword_separating_categories: Lisp_Object = lisp_h_Qnil ();

/* This setter is used only in this file, so it can be private.  */
unsafe fn bset_category_table (b: *mut buffer, val: Lisp_Object)
{
  (*b).category_table_ = val;
}

/* Category set staff.  */

unsafe fn hash_get_category_set (table: Lisp_Object, category_set: Lisp_Object)
  -> Lisp_Object
{
  if NILP (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 1))
    {
      set_char_table_extras
        (table, 1,
         make_hash_table (&raw const hashtest_equal, DEFAULT_HASH_SIZE as EMACS_INT,
                          Weak_None));
    }
  let h: *mut Lisp_Hash_Table =
    XHASH_TABLE (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 1));
  let mut hash: hash_hash_t = 0;
  let i: ptrdiff_t = hash_find_get_hash (h, category_set, &raw mut hash);
  if i >= 0
    { return HASH_KEY (h, i as hash_idx_t); }
  hash_put (h, category_set, Qnil (), hash);
  return category_set;
}

/* Make CATEGORY_SET include (if VAL) or exclude (if !VAL) CATEGORY.  */

unsafe fn set_category_set (category_set: Lisp_Object, category: EMACS_INT, val: bool)
{
  bool_vector_set (category_set, category, val);
}

DEFUN! ("make-category-set", Fmake_category_set, Smake_category_set, 1, 1, 0,
       doc: "Return a newly created category-set which contains CATEGORIES.\nCATEGORIES is a string of category mnemonics.\nThe value is a bool-vector which has t at the indices corresponding to\nthose categories.",
       (categories: Lisp_Object),
       {
  let mut val: Lisp_Object;
  let mut len: ptrdiff_t;

  CHECK_STRING (categories);
  val = MAKE_CATEGORY_SET ();

  if STRING_MULTIBYTE (categories)
    { error (c"Multibyte string in `make-category-set'".as_ptr ()); }

  len = SCHARS (categories);
  len -= 1;
  while len >= 0
    {
      let cat: u8 = SREF (categories, len as usize) as u8;
      let category: Lisp_Object = make_fixnum (cat as EMACS_INT);

      CHECK_CATEGORY (category);
      set_category_set (val, cat as EMACS_INT, true);
      len -= 1;
    }
  return val;
});

/* Category staff.  */

DEFUN! ("define-category", Fdefine_category, Sdefine_category, 2, 3, 0,
       doc: "Define CATEGORY as a category which is described by DOCSTRING.\nCATEGORY should be an ASCII printing character in the range ` ' to `~'.\nDOCSTRING is the documentation string of the category.  The first line\nshould be a terse text (preferably less than 16 characters),\nand the rest lines should be the full description.\nThe category is defined only in category table TABLE, which defaults to\nthe current buffer's category table.",
       (category: Lisp_Object, docstring: Lisp_Object, table: Lisp_Object),
       {
  let mut table: Lisp_Object = table;
  CHECK_CATEGORY (category);
  CHECK_STRING (docstring);
  table = check_category_table (table);

  if ! NILP (CATEGORY_DOCSTRING (table, XFIXNAT (category) as c_int))
    { error (c"Category `%c' is already defined".as_ptr (),
             XFIXNAT (category) as c_int); }
  SET_CATEGORY_DOCSTRING! (table, XFIXNAT (category), docstring);

  return Qnil ();
});

DEFUN! ("category-docstring", Fcategory_docstring, Scategory_docstring, 1, 2, 0,
       doc: "Return the documentation string of CATEGORY, as defined in TABLE.\nTABLE should be a category table and defaults to the current buffer's\ncategory table.",
       (category: Lisp_Object, table: Lisp_Object),
       {
  let mut table: Lisp_Object = table;
  CHECK_CATEGORY (category);
  table = check_category_table (table);

  return CATEGORY_DOCSTRING (table, XFIXNAT (category) as c_int);
});

DEFUN! ("get-unused-category", Fget_unused_category, Sget_unused_category,
       0, 1, 0,
       doc: "Return a category which is not yet defined in TABLE.\nIf no category remains available, return nil.\nThe optional argument TABLE specifies which category table to modify;\nit defaults to the current buffer's category table.",
       (table: Lisp_Object),
       {
  let mut table: Lisp_Object = table;
  let mut i: c_int;

  table = check_category_table (table);

  i = ' ' as c_int;
  while i <= '~' as c_int
    {
      if NILP (CATEGORY_DOCSTRING (table, i))
        { return make_fixnum (i as EMACS_INT); }
      i += 1;
    }

  return Qnil ();
});

/* Category-table staff.  */

DEFUN! ("category-table-p", Fcategory_table_p, Scategory_table_p, 1, 1, 0,
       doc: "Return t if ARG is a category table.",
       (arg: Lisp_Object),
       {
  if CHAR_TABLE_P (arg)
      && EQ ((*XCHAR_TABLE (arg)).purpose, Qcategory_table ())
    { return Qt (); }
  return Qnil ();
});

/* If TABLE is nil, return the current category table.  If TABLE is
   not nil, check the validity of TABLE as a category table.  If
   valid, return TABLE itself, but if not valid, signal an error of
   wrong-type-argument.  */

unsafe fn check_category_table (table: Lisp_Object) -> Lisp_Object
{
  if NILP (table)
    { return BVAR! (current_buffer, category_table_); }
  CHECK_TYPE (! NILP (Fcategory_table_p (table)), Qcategory_table_p (), table);
  return table;
}

DEFUN! ("category-table", Fcategory_table, Scategory_table, 0, 0, 0,
       doc: "Return the current category table.\nThis is the one specified by the current buffer.",
       (),
       {
  return BVAR! (current_buffer, category_table_);
});

DEFUN! ("standard-category-table", Fstandard_category_table,
   Sstandard_category_table, 0, 0, 0,
       doc: "Return the standard category table.\nThis is the one used for new buffers.",
       (),
       {
  return BVAR! (&raw mut buffer_defaults, category_table_);
});

unsafe extern "C" fn copy_category_entry (table: Lisp_Object, c: Lisp_Object, mut val: Lisp_Object)
{
  val = Fcopy_sequence (val);
  if CONSP (c)
    { char_table_set_range (table, XFIXNUM (XCAR (c)) as c_int, XFIXNUM (XCDR (c)) as c_int, val); }
  else
    { char_table_set (table, XFIXNUM (c) as c_int, val); }
}

/* Return a copy of category table TABLE.  We can't simply use the
   function copy-sequence because no contents should be shared between
   the original and the copy.  This function is called recursively by
   binding TABLE to a sub char table.  */

unsafe fn copy_category_table (mut table: Lisp_Object) -> Lisp_Object
{
  table = copy_char_table (table);

  if ! NILP ((*XCHAR_TABLE (table)).defalt)
    {
      set_char_table_defalt (table,
                             Fcopy_sequence ((*XCHAR_TABLE (table)).defalt));
    }
  set_char_table_extras
    (table, 0, Fcopy_sequence (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 0)));
  map_char_table (Some (copy_category_entry), Qnil (), table, table);

  return table;
}

DEFUN! ("copy-category-table", Fcopy_category_table, Scopy_category_table,
       0, 1, 0,
       doc: "Construct a new category table and return it.\nIt is a copy of the TABLE, which defaults to the standard category table.",
       (table: Lisp_Object),
       {
  let mut table: Lisp_Object = table;
  if ! NILP (table)
    { check_category_table (table); }
  else
    { table = BVAR! (&raw mut buffer_defaults, category_table_); }

  return copy_category_table (table);
});

DEFUN! ("make-category-table", Fmake_category_table, Smake_category_table,
       0, 0, 0,
       doc: "Construct a new and empty category table and return it.",
       (),
       {
  let mut val: Lisp_Object;
  let mut i: c_int;

  val = Fmake_char_table (Qcategory_table (), Qnil ());
  set_char_table_defalt (val, MAKE_CATEGORY_SET ());
  i = 0;
  while i < 1 << CHARTAB_SIZE_BITS_0
    {
      set_char_table_contents (val, i as isize, MAKE_CATEGORY_SET ());
      i += 1;
    }
  Fset_char_table_extra_slot (val, make_fixnum (0), make_nil_vector (95));
  return val;
});

DEFUN! ("set-category-table", Fset_category_table, Sset_category_table, 1, 1, 0,
       doc: "Specify TABLE as the category table for the current buffer.\nReturn TABLE.",
       (table: Lisp_Object),
       {
  let mut table: Lisp_Object = table;
  let mut idx: c_int;
  table = check_category_table (table);
  bset_category_table (current_buffer, table);
  /* Indicate that this buffer now has a specified category table.  */
  idx = PER_BUFFER_VAR_IDX! (category_table_);
  SET_PER_BUFFER_VALUE_P (current_buffer, idx, true);
  return table;
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn char_category_set (c: c_int) -> Lisp_Object
{
  return CHAR_TABLE_REF (BVAR! (current_buffer, category_table_), c as isize);
}

DEFUN! ("char-category-set", Fchar_category_set, Schar_category_set, 1, 1, 0,
       doc: "Return the category set of CHAR.\nusage: (char-category-set CHAR)",
       (ch: Lisp_Object),
       {
  CHECK_CHARACTER (ch);
  return CATEGORY_SET (XFIXNAT (ch) as c_int);
});

DEFUN! ("category-set-mnemonics", Fcategory_set_mnemonics,
       Scategory_set_mnemonics, 1, 1, 0,
       doc: "Return a string containing mnemonics of the categories in CATEGORY-SET.\nCATEGORY-SET is a bool-vector, and the categories \"in\" it are those\nthat are indexes where t occurs in the bool-vector.\nThe return value is a string containing those same categories.",
       (category_set: Lisp_Object),
       {
  let mut i: c_int;
  let mut j: c_int;
  let mut str_: [c_char; 96] = [0; 96];

  CHECK_CATEGORY_SET (category_set);

  j = 0;
  i = 32;
  while i < 127
    {
      if CATEGORY_MEMBER (i as EMACS_INT, category_set)
        {
          str_[j as usize] = i as c_char;
          j += 1;
        }
      i += 1;
    }
  str_[j as usize] = 0;

  return build_string (str_.as_ptr ());
});

DEFUN! ("modify-category-entry", Fmodify_category_entry,
       Smodify_category_entry, 2, 4, 0,
       doc: "Modify the category set of CHARACTER by adding CATEGORY to it.\nThe category is changed only for table TABLE, which defaults to\nthe current buffer's category table.\nCHARACTER can be either a single character or a cons representing the\nlower and upper ends of an inclusive character range to modify.\nCATEGORY must be a category name (a character between ` ' and `~').\nUse `describe-categories' to see existing category names.\nIf optional fourth argument RESET is non-nil,\nthen delete CATEGORY from the category set instead of adding it.",
       (character: Lisp_Object, category: Lisp_Object, table: Lisp_Object,
        reset: Lisp_Object),
       {
  let mut table: Lisp_Object = table;
  let set_value: bool;	/* Actual value to be set in category sets.  */
  let mut category_set: Lisp_Object;
  let mut start: c_int;
  let mut end: c_int;
  let mut from: c_int;
  let mut to: c_int;

  if FIXNUMP (character)
    {
      CHECK_CHARACTER (character);
      start = XFIXNAT (character) as c_int;
      end = start;
    }
  else
    {
      CHECK_CONS (character);
      CHECK_CHARACTER_CAR (character);
      CHECK_CHARACTER_CDR (character);
      start = XFIXNAT (XCAR (character)) as c_int;
      end = XFIXNAT (XCDR (character)) as c_int;
    }

  CHECK_CATEGORY (category);
  table = check_category_table (table);

  if NILP (CATEGORY_DOCSTRING (table, XFIXNAT (category) as c_int))
    { error (c"Undefined category: %c".as_ptr (), XFIXNAT (category) as c_int); }

  set_value = NILP (reset);

  while start <= end
    {
      from = start;
      to = end;
      category_set = char_table_ref_and_range (table, start, &raw mut from, &raw mut to);
      if CATEGORY_MEMBER (XFIXNAT (category), category_set) != NILP (reset)
        {
          category_set = Fcopy_sequence (category_set);
          set_category_set (category_set, XFIXNAT (category), set_value);
          category_set = hash_get_category_set (table, category_set);
          char_table_set_range (table, start, to, category_set);
        }
      start = to + 1;
    }

  return Qnil ();
});

/* Return true if there is a word boundary between two word-constituent
   characters C1 and C2 if they appear in this order.
   Use the macro WORD_BOUNDARY_P instead of calling this function
   directly.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn word_boundary_p (c1: c_int, c2: c_int) -> bool
{
  let mut category_set1: Lisp_Object;
  let mut category_set2: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut default_result: bool;

  if EQ (CHAR_TABLE_REF (Vchar_script_table, c1 as isize),
         CHAR_TABLE_REF (Vchar_script_table, c2 as isize))
    {
      tail = Vword_separating_categories;
      default_result = false;
    }
  else
    {
      tail = Vword_combining_categories;
      default_result = true;
    }

  category_set1 = CATEGORY_SET (c1);
  if NILP (category_set1)
    { return default_result; }
  category_set2 = CATEGORY_SET (c2);
  if NILP (category_set2)
    { return default_result; }

  while CONSP (tail)
    {
      let elt: Lisp_Object = XCAR (tail);

      if CONSP (elt)
          && (NILP (XCAR (elt))
              || (CATEGORYP (XCAR (elt))
                  && CATEGORY_MEMBER (XFIXNAT (XCAR (elt)), category_set1)
                  && ! CATEGORY_MEMBER (XFIXNAT (XCAR (elt)), category_set2)))
          && (NILP (XCDR (elt))
              || (CATEGORYP (XCDR (elt))
                  && ! CATEGORY_MEMBER (XFIXNAT (XCDR (elt)), category_set1)
                  && CATEGORY_MEMBER (XFIXNAT (XCDR (elt)), category_set2)))
        { return ! default_result; }
      tail = XCDR (tail);
    }
  return default_result;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_category_once ()
{
  /* This has to be done here, before we call Fmake_char_table.  */
  DEFSYM! (Qcategory_table, "category-table");
  Fput (Qcategory_table (), Qchar_table_extra_slots (), make_fixnum (2));

  BVAR! (&raw mut buffer_defaults, category_table_) = Fmake_char_table (Qcategory_table (), Qnil ());
  /* Set a category set which contains nothing to the default.  */
  set_char_table_defalt (BVAR! (&raw mut buffer_defaults, category_table_),
                         MAKE_CATEGORY_SET ());
  Fset_char_table_extra_slot (BVAR! (&raw mut buffer_defaults, category_table_),
                              make_fixnum (0),
                              make_nil_vector (95));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_category ()
{
  DEFSYM! (Qcategoryp, "categoryp");
  DEFSYM! (Qcategorysetp, "categorysetp");
  DEFSYM! (Qcategory_table_p, "category-table-p");

  DEFVAR_LISP! (c"word-combining-categories", Vword_combining_categories,
                "List of pairs (cons cells) of categories to determine word boundary.\n\nEmacs treats a sequence of word constituent characters as a single\nword (i.e. finds no word boundary between them) only if they belong to\nthe same script (see `char-script-table').  Exceptions to this rule\nare allowed in the following cases.\n\n(1) The case that characters are in different scripts is controlled\nby the variable `word-combining-categories'.\n\nEmacs finds no word boundary between characters of different scripts\nif they have categories matching some element of this list.\n\nMore precisely, if an element of this list is a cons of category CAT1\nand CAT2, and a multibyte character C1 which has CAT1 (but not CAT2)\nis followed by C2 which has CAT2 (but not CAT1), there's no word\nboundary between C1 and C2.\n\nFor instance, to tell that Han characters followed by Hiragana\ncharacters can form a single word, the element `(?C . ?H)' should be\nin this list.\n\nCAT1 or CAT2 nil means that any character will do if it doesn't have\nthe other element of the cons in its category set.  For instance, to\ntell that ASCII characters can form a single word with non-ASCII\ncharacters, this list should have the elements `(nil . ?a)' and `(?a)'.\n\n(2) The case that character are in the same script is controlled by\nthe variable `word-separating-categories'.\n\nEmacs finds a word boundary between characters of the same script\nif they have categories matching some element of this list.\n\nMore precisely, if an element of this list is a cons of category CAT1\nand CAT2, and a multibyte character C1 which has CAT1 but not CAT2 is\nfollowed by C2 which has CAT2 but not CAT1, there's a word boundary\nbetween C1 and C2.\n\nFor instance, to tell that there's a word boundary between Hiragana\nand Katakana (both are in the same script `kana'),\nthe element `(?H . ?K)' should be in this list.");

  Vword_combining_categories = Qnil ();

  DEFVAR_LISP! (c"word-separating-categories", Vword_separating_categories,
                "List of pairs (cons cells) of categories to determine word boundary.\nSee the documentation of the variable `word-combining-categories'.");

  Vword_separating_categories = Qnil ();

  defsubr (&raw mut Smake_category_set);
  defsubr (&raw mut Sdefine_category);
  defsubr (&raw mut Scategory_docstring);
  defsubr (&raw mut Sget_unused_category);
  defsubr (&raw mut Scategory_table_p);
  defsubr (&raw mut Scategory_table);
  defsubr (&raw mut Sstandard_category_table);
  defsubr (&raw mut Scopy_category_table);
  defsubr (&raw mut Smake_category_table);
  defsubr (&raw mut Sset_category_table);
  defsubr (&raw mut Schar_category_set);
  defsubr (&raw mut Scategory_set_mnemonics);
  defsubr (&raw mut Smodify_category_entry);
}
