// femacs: port of src/doc.c @ fd7afced403.
//
// Logic lives in `#![forbid(unsafe_code)]` `crate::safe_doc` (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

#[unsafe(no_mangle)]
pub static Vdoc_file_name: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vbuild_files: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vtext_quoting_style: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static documentation_dynamic_reload: VmGlobal<bool> = VmGlobal::<bool>::new (false);
#[unsafe(no_mangle)]
pub static text_quoting_flag: VmGlobal<bool> = VmGlobal::<bool>::new (false);

#[unsafe(no_mangle)]
pub extern "C-unwind" fn get_doc_string (filepos: Lisp_Object, unibyte: bool) -> Lisp_Object
{
  crate::safe_doc::get_doc_string_impl (filepos, unibyte)
}

DEFUN! ("documentation-stringp", Fdocumentation_stringp, Sdocumentation_stringp,
       1, 1, 0,
       doc: "Return non-nil if OBJECT is a well-formed docstring object.\nOBJECT can be either a string or a reference if it's kept externally.",
       (object: Lisp_Object),
       {
  crate::safe_doc::documentation_stringp_subr (object)
});

DEFUN! ("documentation", Fdocumentation, Sdocumentation, 1, 2, 0,
       doc: "Return the documentation string of FUNCTION.\nUnless a non-nil second argument RAW is given, the\nstring is passed through `substitute-command-keys'.",
       (function: Lisp_Object, raw: Lisp_Object),
       {
  crate::safe_doc::documentation_subr (function, raw)
});

DEFUN! ("internal-subr-documentation", Fsubr_documentation, Ssubr_documentation, 1, 1, 0,
       doc: "Return the raw documentation info of a C primitive.",
       (function: Lisp_Object),
       {
  crate::safe_doc::subr_documentation_subr (function)
});

DEFUN! ("documentation-property", Fdocumentation_property,
       Sdocumentation_property, 2, 3, 0,
       doc: "Return the documentation string that is SYMBOL's PROP property.\nThird argument RAW omitted or nil means pass the result through\n`substitute-command-keys' if it is a string.\n\nThis differs from `get' in that it can refer to strings stored in the\n`etc/DOC' file; and that it evaluates documentation properties that\naren't strings.",
       (symbol: Lisp_Object, prop: Lisp_Object, raw: Lisp_Object),
       {
  crate::safe_doc::documentation_property_subr (symbol, prop, raw)
});

DEFUN! ("Snarf-documentation", Fsnarf_documentation, Ssnarf_documentation,
       1, 1, 0,
       doc: "Used during Emacs initialization to scan the `etc/DOC...' file.\nThis searches the `etc/DOC...' file for doc strings and\nrecords them in function and variable definitions.\nThe function takes one argument, FILENAME, a string;\nit specifies the file name (without a directory) of the DOC file.\nThat file is found in `../etc' now; later, when the dumped Emacs is run,\nthe same file name is found in the `doc-directory'.",
       (filename: Lisp_Object),
       {
  crate::safe_doc::snarf_documentation_subr (filename)
});

DEFUN! ("text-quoting-style", Ftext_quoting_style,
       Stext_quoting_style, 0, 0, 0,
       doc: "Return the current effective text quoting style.\nIf the variable `text-quoting-style' is `grave', `straight' or\n`curve', just return that value.  If it is nil (the default), return\n`grave' if curved quotes cannot be displayed (for instance, on a\nterminal with no support for these characters), otherwise return\n`curve'.  Any other value is treated as `curve'.\n\nNote that in contrast to the variable `text-quoting-style', this\nfunction will never return nil.",
       (),
       {
  crate::safe_doc::text_quoting_style_subr ()
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_doc ()
{
  DEFSYM! (Qlisp_directory, "lisp-directory");
  DEFSYM! (Qsubstitute_command_keys, "substitute-command-keys");
  DEFSYM! (Qfunction_documentation, "function-documentation");
  DEFSYM! (Qgrave, "grave");
  DEFSYM! (Qstraight, "straight");
  DEFSYM! (Qcurve, "curve");

  DEFVAR_LISP! (c"internal-doc-file-name", Vdoc_file_name,
	       "Name of file containing documentation strings of built-in symbols.  ");
  Vdoc_file_name.set (Qnil ());

  DEFVAR_LISP! (c"build-files", Vbuild_files,
               "A list of files used to build this Emacs binary.  ");
  Vbuild_files.set (Qnil ());

  DEFVAR_LISP! (c"text-quoting-style", Vtext_quoting_style,
               "Style to use for single quotes in help and messages.\n\nThe value of this variable determines substitution of grave accents\nand apostrophes in help output (but not for display of Info\nmanuals) and in functions like `message' and `format-message', but not\nin `format'.\n\nThe value should be one of these symbols:\n  `curve':    quote with curved single quotes ‘like this’.\n  `straight': quote with straight apostrophes \\='like this\\='.\n  `grave':    quote with grave accent and apostrophe \\=`like this\\=';\n	      i.e., do not alter the original quote marks.\n  nil:        like `curve' if curved single quotes are displayable,\n	      and like `grave' otherwise.  This is the default.\n\nYou should never read the value of this variable directly from a Lisp\nprogram.  Use the function `text-quoting-style' instead, as that will\ncompute the correct value for the current terminal in the nil case.  ");
  Vtext_quoting_style.set (Qnil ());

  DEFVAR_BOOL! (c"documentation-dynamic-reload", documentation_dynamic_reload,
	       "If non-nil, reload changed `DOC' and Lisp files when calling `documentation'.\n\nFor `etc/DOC' and for files byte-compiled with non-nil\n`byte-compile-dynamic-docstring' (the default), documentation strings\nare loaded on-demand only when `documentation' is called for a symbol.\n\nIf these files have changed since they were initially loaded, reading\nthe documentation string for that symbol out of the files may fail.  If\nit fails, and this variable is non-nil, then the files will be loaded\nagain to redefine all its functions and variables.  `documentation' will\nthen retry; if the symbol was redefined by reloading the file, reading\nthe documentation string will then succeed.  ");
  documentation_dynamic_reload.set (true);

  DEFVAR_BOOL! (c"internal--text-quoting-flag", text_quoting_flag,
	       "If nil, a nil `text-quoting-style' is treated as `grave'.  ");

  defsubr_global (&Sdocumentation_stringp);
  defsubr_global (&Sdocumentation);
  defsubr_global (&Ssubr_documentation);
  defsubr_global (&Sdocumentation_property);
  defsubr_global (&Ssnarf_documentation);
  defsubr_global (&Stext_quoting_style);
  DEFSYM! (Qcustom_delayed_init_variables, "custom-delayed-init-variables");
}
