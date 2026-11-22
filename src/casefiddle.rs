// femacs: port of src/casefiddle.c @ fd7afced403.
//
// Logic lives in `#![forbid(unsafe_code)]` `crate::safe_casefiddle` (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

#[unsafe(no_mangle)]
pub static Vregion_extract_function: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static case_symbols_as_words: VmGlobal<bool> = VmGlobal::<bool>::new (false);

pub type case_action = c_int;
pub const CASE_UP: case_action = crate::safe_casefiddle::CASE_UP;
pub const CASE_DOWN: case_action = crate::safe_casefiddle::CASE_DOWN;
pub const CASE_CAPITALIZE: case_action = crate::safe_casefiddle::CASE_CAPITALIZE;
pub const CASE_CAPITALIZE_UP: case_action = crate::safe_casefiddle::CASE_CAPITALIZE_UP;

DEFUN! ("upcase", Fupcase, Supcase, 1, 1, 0,
       doc: "Convert argument to upper case and return that.\nThe argument may be a character or string.  The result has the same\ntype.  (See `downcase' for further details about the type.)\n\nThe argument object is not altered--the value is a copy.  If argument\nis a character, characters which map to multiple code points when\ncased, e.g. ﬁ, are returned unchanged.\n\nSee also `capitalize', `downcase' and `upcase-initials'.",
       (obj: Lisp_Object),
       {
  crate::safe_casefiddle::casify_object (CASE_UP, obj)
});

DEFUN! ("downcase", Fdowncase, Sdowncase, 1, 1, 0,
       doc: "Convert argument to lower case and return that.\nThe argument may be a character or string.  The result has the same type,\nincluding the multibyteness of the string.\n\nThis means that if this function is called with a unibyte string\nargument, and downcasing it would turn it into a multibyte string\n(according to the current locale), the downcasing is done using ASCII\n\"C\" rules instead.  To accurately downcase according to the current\nlocale, the string must be converted into multibyte first.\n\nThe argument object is not altered--the value is a copy.",
       (obj: Lisp_Object),
       {
  crate::safe_casefiddle::casify_object (CASE_DOWN, obj)
});

DEFUN! ("capitalize", Fcapitalize, Scapitalize, 1, 1, 0,
       doc: "Convert argument to capitalized form and return that.\nThis means that each word's first character is converted to either\ntitle case or upper case, and the rest to lower case.\n\nThe argument may be a character or string.  The result has the same\ntype.  (See `downcase' for further details about the type.)\n\nThe argument object is not altered--the value is a copy.  If argument\nis a character, characters which map to multiple code points when\ncased, e.g. ﬁ, are returned unchanged.",
       (obj: Lisp_Object),
       {
  crate::safe_casefiddle::casify_object (CASE_CAPITALIZE, obj)
});

DEFUN! ("upcase-initials", Fupcase_initials, Supcase_initials, 1, 1, 0,
       doc: "Convert the initial of each word in the argument to upper case.\nThis means that each word's first character is converted to either\ntitle case or upper case, and the rest are left unchanged.\n\nThe argument may be a character or string.  The result has the same\ntype.  (See `downcase' for further details about the type.)\n\nThe argument object is not altered--the value is a copy.  If argument\nis a character, characters which map to multiple code points when\ncased, e.g. ﬁ, are returned unchanged.",
       (obj: Lisp_Object),
       {
  crate::safe_casefiddle::casify_object (CASE_CAPITALIZE_UP, obj)
});

DEFUN! ("upcase-region", Fupcase_region, Supcase_region, 2, 3,
       "(list (region-beginning) (region-end) (region-noncontiguous-p))",
       doc: "Convert the region to upper case.  In programs, wants two arguments.\nThese arguments specify the starting and ending character numbers of\nthe region to operate on.  When used as a command, the text between\npoint and the mark is operated on.\nSee also `capitalize-region'.",
       (beg: Lisp_Object, end: Lisp_Object, region_noncontiguous_p: Lisp_Object),
       {
  crate::safe_casefiddle::casify_pnc_region (CASE_UP, beg, end, region_noncontiguous_p)
});

DEFUN! ("downcase-region", Fdowncase_region, Sdowncase_region, 2, 3,
       "(list (region-beginning) (region-end) (region-noncontiguous-p))",
       doc: "Convert the region to lower case.  In programs, wants two arguments.\nThese arguments specify the starting and ending character numbers of\nthe region to operate on.  When used as a command, the text between\npoint and the mark is operated on.",
       (beg: Lisp_Object, end: Lisp_Object, region_noncontiguous_p: Lisp_Object),
       {
  crate::safe_casefiddle::casify_pnc_region (CASE_DOWN, beg, end, region_noncontiguous_p)
});

DEFUN! ("capitalize-region", Fcapitalize_region, Scapitalize_region, 2, 3,
       "(list (region-beginning) (region-end) (region-noncontiguous-p))",
       doc: "Convert the region to capitalized form.\nThis means that each word's first character is converted to either\ntitle case or upper case, and the rest to lower case.\nIn programs, give two arguments, the starting and ending\ncharacter positions to operate on.",
       (beg: Lisp_Object, end: Lisp_Object, region_noncontiguous_p: Lisp_Object),
       {
  crate::safe_casefiddle::casify_pnc_region (CASE_CAPITALIZE, beg, end, region_noncontiguous_p)
});

DEFUN! ("upcase-initials-region", Fupcase_initials_region,
       Supcase_initials_region, 2, 3,
       "(list (region-beginning) (region-end) (region-noncontiguous-p))",
       doc: "Upcase the initial of each word in the region.\nThis means that each word's first character is converted to either\ntitle case or upper case, and the rest are left unchanged.\nIn programs, give two arguments, the starting and ending\ncharacter positions to operate on.",
       (beg: Lisp_Object, end: Lisp_Object, region_noncontiguous_p: Lisp_Object),
       {
  crate::safe_casefiddle::casify_pnc_region (CASE_CAPITALIZE_UP, beg, end, region_noncontiguous_p)
});

DEFUN! ("upcase-word", Fupcase_word, Supcase_word, 1, 1, "p",
       doc: "Convert to upper case from point to end of word, moving over.\n\nIf point is in the middle of a word, the part of that word before point\nis ignored when moving forward.\n\nWith negative argument, convert previous words but do not move.\nSee also `capitalize-word'.",
       (arg: Lisp_Object),
       {
  crate::safe_casefiddle::casify_word (CASE_UP, arg)
});

DEFUN! ("downcase-word", Fdowncase_word, Sdowncase_word, 1, 1, "p",
       doc: "Convert to lower case from point to end of word, moving over.\n\nIf point is in the middle of a word, the part of that word before point\nis ignored when moving forward.\n\nWith negative argument, convert previous words but do not move.",
       (arg: Lisp_Object),
       {
  crate::safe_casefiddle::casify_word (CASE_DOWN, arg)
});

DEFUN! ("capitalize-word", Fcapitalize_word, Scapitalize_word, 1, 1, "p",
       doc: "Capitalize from point to the end of word, moving over.\nWith numerical argument ARG, capitalize the next ARG-1 words as well.\nThis gives the word(s) a first character in upper case\nand the rest lower case.\n\nIf point is in the middle of a word, the part of that word before point\nis ignored when moving forward.\n\nWith negative argument, capitalize previous words but do not move.",
       (arg: Lisp_Object),
       {
  crate::safe_casefiddle::casify_word (CASE_CAPITALIZE, arg)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_casefiddle ()
{
  DEFSYM! (Qbounds, "bounds");
  DEFSYM! (Qidentity, "identity");
  DEFSYM! (Qtitlecase, "titlecase");
  DEFSYM! (Qlowercase, "lowercase");
  DEFSYM! (Quppercase, "uppercase");
  DEFSYM! (Qspecial_uppercase, "special-uppercase");
  DEFSYM! (Qspecial_lowercase, "special-lowercase");
  DEFSYM! (Qspecial_titlecase, "special-titlecase");

  DEFVAR_LISP! (c"region-extract-function", Vregion_extract_function,
	       "Function to get the region's content.\nCalled with one argument METHOD which can be:\n- nil: return the content as a string (list of strings for\n  non-contiguous regions).\n- `delete-only': delete the region; the return value is undefined.\n- `bounds': return the boundaries of the region as a list of one\n  or more cons cells of the form (START . END).\n- anything else: delete the region and return its content\n  as a string (or list of strings for non-contiguous regions),\n  after filtering it with `filter-buffer-substring', which\n  is called, for each contiguous sub-region, with METHOD as its\n  3rd argument.  ");
  Vregion_extract_function.set (Qnil ());

  DEFVAR_BOOL! (c"case-symbols-as-words", case_symbols_as_words,
    "If non-nil, case functions treat symbol syntax as part of words.\n\nFunctions such as `upcase-initials' and `replace-match' check or modify\nthe case pattern of sequences of characters.  Normally, these operate on\nsequences of characters whose syntax is word constituent.  If this\nvariable is non-nil, then they operate on sequences of characters whose\nsyntax is either word constituent or symbol constituent.\n\nThis is useful for programming languages and styles where only the first\nletter of a symbol's name is ever capitalized.");
  case_symbols_as_words.set (false);
  DEFSYM! (Qcase_symbols_as_words, "case-symbols-as-words");
  Fmake_variable_buffer_local (Qcase_symbols_as_words ());

  defsubr_global (&Supcase);
  defsubr_global (&Sdowncase);
  defsubr_global (&Scapitalize);
  defsubr_global (&Supcase_initials);
  defsubr_global (&Supcase_region);
  defsubr_global (&Sdowncase_region);
  defsubr_global (&Scapitalize_region);
  defsubr_global (&Supcase_initials_region);
  defsubr_global (&Supcase_word);
  defsubr_global (&Sdowncase_word);
  defsubr_global (&Scapitalize_word);
}
