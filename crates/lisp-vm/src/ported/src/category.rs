// femacs: port of src/category.c @ fd7afced403.
//
// Logic lives in `#![forbid(unsafe_code)]` `crate::safe_category` (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

#[unsafe(no_mangle)]
pub static Vword_combining_categories: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vword_separating_categories: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

DEFUN! ("make-category-set", Fmake_category_set, Smake_category_set, 1, 1, 0,
       doc: "Return a newly created category-set which contains CATEGORIES.\nCATEGORIES is a string of category mnemonics.\nThe value is a bool-vector which has t at the indices corresponding to\nthose categories.",
       (categories: Lisp_Object),
       {
  crate::safe_category::make_category_set (categories)
});

DEFUN! ("define-category", Fdefine_category, Sdefine_category, 2, 3, 0,
       doc: "Define CATEGORY as a category which is described by DOCSTRING.\nCATEGORY should be an ASCII printing character in the range ` ' to `~'.\nDOCSTRING is the documentation string of the category.  The first line\nshould be a terse text (preferably less than 16 characters),\nand the rest lines should be the full description.\nThe category is defined only in category table TABLE, which defaults to\nthe current buffer's category table.",
       (category: Lisp_Object, docstring: Lisp_Object, table: Lisp_Object),
       {
  crate::safe_category::define_category (category, docstring, table)
});

DEFUN! ("category-docstring", Fcategory_docstring, Scategory_docstring, 1, 2, 0,
       doc: "Return the documentation string of CATEGORY, as defined in TABLE.\nTABLE should be a category table and defaults to the current buffer's\ncategory table.",
       (category: Lisp_Object, table: Lisp_Object),
       {
  crate::safe_category::category_docstring (category, table)
});

DEFUN! ("get-unused-category", Fget_unused_category, Sget_unused_category,
       0, 1, 0,
       doc: "Return a category which is not yet defined in TABLE.\nIf no category remains available, return nil.\nThe optional argument TABLE specifies which category table to modify;\nit defaults to the current buffer's category table.",
       (table: Lisp_Object),
       {
  crate::safe_category::get_unused_category (table)
});

DEFUN! ("category-table-p", Fcategory_table_p, Scategory_table_p, 1, 1, 0,
       doc: "Return t if ARG is a category table.",
       (arg: Lisp_Object),
       {
  crate::safe_category::category_table_p (arg)
});

DEFUN! ("category-table", Fcategory_table, Scategory_table, 0, 0, 0,
       doc: "Return the current category table.\nThis is the one specified by the current buffer.",
       (),
       {
  crate::safe_category::category_table ()
});

DEFUN! ("standard-category-table", Fstandard_category_table,
   Sstandard_category_table, 0, 0, 0,
       doc: "Return the standard category table.\nThis is the one used for new buffers.",
       (),
       {
  crate::safe_category::standard_category_table_subr ()
});

DEFUN! ("copy-category-table", Fcopy_category_table, Scopy_category_table,
       0, 1, 0,
       doc: "Construct a new category table and return it.\nIt is a copy of the TABLE, which defaults to the standard category table.",
       (table: Lisp_Object),
       {
  crate::safe_category::copy_category_table_subr (table)
});

DEFUN! ("make-category-table", Fmake_category_table, Smake_category_table,
       0, 0, 0,
       doc: "Construct a new and empty category table and return it.",
       (),
       {
  crate::safe_category::make_category_table ()
});

DEFUN! ("set-category-table", Fset_category_table, Sset_category_table, 1, 1, 0,
       doc: "Specify TABLE as the category table for the current buffer.\nReturn TABLE.",
       (table: Lisp_Object),
       {
  crate::safe_category::set_category_table (table)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn char_category_set (c: c_int) -> Lisp_Object
{
  crate::safe_category::char_category_set_impl (c)
}

DEFUN! ("char-category-set", Fchar_category_set, Schar_category_set, 1, 1, 0,
       doc: "Return the category set of CHAR.\nusage: (char-category-set CHAR)",
       (ch: Lisp_Object),
       {
  crate::safe_category::char_category_set_subr (ch)
});

DEFUN! ("category-set-mnemonics", Fcategory_set_mnemonics,
       Scategory_set_mnemonics, 1, 1, 0,
       doc: "Return a string containing mnemonics of the categories in CATEGORY-SET.\nCATEGORY-SET is a bool-vector, and the categories \"in\" it are those\nthat are indexes where t occurs in the bool-vector.\nThe return value is a string containing those same categories.",
       (category_set: Lisp_Object),
       {
  crate::safe_category::category_set_mnemonics (category_set)
});

DEFUN! ("modify-category-entry", Fmodify_category_entry,
       Smodify_category_entry, 2, 4, 0,
       doc: "Modify the category set of CHARACTER by adding CATEGORY to it.\nThe category is changed only for table TABLE, which defaults to\nthe current buffer's category table.\nCHARACTER can be either a single character or a cons representing the\nlower and upper ends of an inclusive character range to modify.\nCATEGORY must be a category name (a character between ` ' and `~').\nUse `describe-categories' to see existing category names.\nIf optional fourth argument RESET is non-nil,\nthen delete CATEGORY from the category set instead of adding it.",
       (character: Lisp_Object, category: Lisp_Object, table: Lisp_Object,
        reset: Lisp_Object),
       {
  crate::safe_category::modify_category_entry (character, category, table, reset)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn word_boundary_p (c1: c_int, c2: c_int) -> bool
{
  crate::safe_category::word_boundary_p_impl (c1, c2)
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn init_category_once ()
{
  DEFSYM! (Qcategory_table, "category-table");
  crate::safe_category::init_category_once_impl ();
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_category ()
{
  DEFSYM! (Qcategoryp, "categoryp");
  DEFSYM! (Qcategorysetp, "categorysetp");
  DEFSYM! (Qcategory_table_p, "category-table-p");

  DEFVAR_LISP! (c"word-combining-categories", Vword_combining_categories,
                "List of pairs (cons cells) of categories to determine word boundary.\n\nEmacs treats a sequence of word constituent characters as a single\nword (i.e. finds no word boundary between them) only if they belong to\nthe same script (see `char-script-table').  Exceptions to this rule\nare allowed in the following cases.\n\n(1) The case that characters are in different scripts is controlled\nby the variable `word-combining-categories'.\n\nEmacs finds no word boundary between characters of different scripts\nif they have categories matching some element of this list.\n\nMore precisely, if an element of this list is a cons of category CAT1\nand CAT2, and a multibyte character C1 which has CAT1 (but not CAT2)\nis followed by C2 which has CAT2 (but not CAT1), there's no word\nboundary between C1 and C2.\n\nFor instance, to tell that Han characters followed by Hiragana\ncharacters can form a single word, the element `(?C . ?H)' should be\nin this list.\n\nCAT1 or CAT2 nil means that any character will do if it doesn't have\nthe other element of the cons in its category set.  For instance, to\ntell that ASCII characters can form a single word with non-ASCII\ncharacters, this list should have the elements `(nil . ?a)' and `(?a)'.\n\n(2) The case that character are in the same script is controlled by\nthe variable `word-separating-categories'.\n\nEmacs finds a word boundary between characters of the same script\nif they have categories matching some element of this list.\n\nMore precisely, if an element of this list is a cons of category CAT1\nand CAT2, and a multibyte character C1 which has CAT1 but not CAT2 is\nfollowed by C2 which has CAT2 but not CAT1, there's a word boundary\nbetween C1 and C2.\n\nFor instance, to tell that there's a word boundary between Hiragana\nand Katakana (both are in the same script `kana'),\nthe element `(?H . ?K)' should be in this list.");
  Vword_combining_categories.set (Qnil ());

  DEFVAR_LISP! (c"word-separating-categories", Vword_separating_categories,
                "List of pairs (cons cells) of categories to determine word boundary.\nSee the documentation of the variable `word-combining-categories'.");
  Vword_separating_categories.set (Qnil ());

  defsubr_global (&Smake_category_set);
  defsubr_global (&Sdefine_category);
  defsubr_global (&Scategory_docstring);
  defsubr_global (&Sget_unused_category);
  defsubr_global (&Scategory_table_p);
  defsubr_global (&Scategory_table);
  defsubr_global (&Sstandard_category_table);
  defsubr_global (&Scopy_category_table);
  defsubr_global (&Smake_category_table);
  defsubr_global (&Sset_category_table);
  defsubr_global (&Schar_category_set);
  defsubr_global (&Scategory_set_mnemonics);
  defsubr_global (&Smodify_category_entry);
}
