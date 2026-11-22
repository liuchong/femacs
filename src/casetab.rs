// femacs: port of src/casetab.c @ fd7afced403.
//
// Logic lives in `#![forbid(unsafe_code)]` `crate::safe_casetab` (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

#[unsafe(no_mangle)]
pub static Vascii_downcase_table: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
pub(crate) static Vascii_upcase_table: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vascii_canon_table: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
pub(crate) static Vascii_eqv_table: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

DEFUN! ("case-table-p", Fcase_table_p, Scase_table_p, 1, 1, 0,
       doc: "Return t if OBJECT is a case table.\nSee `set-case-table' for more information on these data structures.",
       (object: Lisp_Object),
       {
  crate::safe_casetab::case_table_p (object)
});

DEFUN! ("current-case-table", Fcurrent_case_table, Scurrent_case_table, 0, 0, 0,
       doc: "Return the case table of the current buffer.",
       (),
       {
  crate::safe_casetab::current_case_table ()
});

DEFUN! ("standard-case-table", Fstandard_case_table, Sstandard_case_table, 0, 0, 0,
       doc: "Return the standard case table.\nThis is the one used for new buffers.",
       (),
       {
  crate::safe_casetab::standard_case_table ()
});

DEFUN! ("set-case-table", Fset_case_table, Sset_case_table, 1, 1, 0,
       doc: "Select a new case table for the current buffer.\nA case table is a char-table which maps characters\nto their lower-case equivalents.  It also has three \"extra\" slots\nwhich may be additional char-tables or nil.\nThese slots are called UPCASE, CANONICALIZE and EQUIVALENCES.\nUPCASE maps each non-upper-case character to its upper-case equivalent.\n (The value in UPCASE for an upper-case character is never used.)\n If lower and upper case characters are in 1-1 correspondence,\n you may use nil and the upcase table will be deduced from DOWNCASE.\nCANONICALIZE maps each character to a canonical equivalent;\n any two characters that are related by case-conversion have the same\n canonical equivalent character; it may be nil, in which case it is\n deduced from DOWNCASE and UPCASE.\nEQUIVALENCES is a map that cyclically permutes each equivalence class\n (of characters with the same canonical equivalent); it may be nil,\n in which case it is deduced from CANONICALIZE.",
       (table: Lisp_Object),
       {
  crate::safe_casetab::set_case_table (table, false)
});

DEFUN! ("set-standard-case-table", Fset_standard_case_table, Sset_standard_case_table, 1, 1, 0,
       doc: "Select a new standard case table for new buffers.\nSee `set-case-table' for more info on case tables.",
       (table: Lisp_Object),
       {
  crate::safe_casetab::set_case_table (table, true)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn init_casetab_once ()
{
  DEFSYM! (Qcase_table, "case-table");
  crate::safe_casetab::init_casetab_once_impl ();
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_casetab ()
{
  DEFSYM! (Qcase_table_p, "case-table-p");

  staticpro_global (&Vascii_canon_table);
  staticpro_global (&Vascii_downcase_table);
  staticpro_global (&Vascii_eqv_table);
  staticpro_global (&Vascii_upcase_table);

  defsubr_global (&Scase_table_p);
  defsubr_global (&Scurrent_case_table);
  defsubr_global (&Sstandard_case_table);
  defsubr_global (&Sset_case_table);
  defsubr_global (&Sset_standard_case_table);
}
