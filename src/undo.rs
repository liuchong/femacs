// femacs: port of src/undo.c @ fd7afced403.
//
// Logic lives in `#![forbid(unsafe_code)]` `crate::safe_undo` (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

#[unsafe(no_mangle)]
pub static undo_limit: VmGlobal<EMACS_INT> = VmGlobal::<EMACS_INT>::new (0);
#[unsafe(no_mangle)]
pub static undo_strong_limit: VmGlobal<EMACS_INT> = VmGlobal::<EMACS_INT>::new (0);
#[unsafe(no_mangle)]
pub static Vundo_outer_limit: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vundo_outer_limit_function: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static undo_inhibit_record_point: VmGlobal<bool> = VmGlobal::<bool>::new (false);

pub(crate) static pending_boundary: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

#[unsafe(no_mangle)]
pub extern "C-unwind" fn record_insert (beg: ptrdiff_t, length: ptrdiff_t)
{
  crate::safe_undo::record_insert_impl (beg, length);
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn record_delete (beg: ptrdiff_t, string: Lisp_Object, record_markers: bool)
{
  crate::safe_undo::record_delete_impl (beg, string, record_markers);
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn record_change (beg: ptrdiff_t, length: ptrdiff_t)
{
  crate::safe_undo::record_change_impl (beg, length);
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn record_first_change ()
{
  crate::safe_undo::record_first_change_impl ();
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn record_property_change (
  beg: ptrdiff_t,
  length: ptrdiff_t,
  prop: Lisp_Object,
  value: Lisp_Object,
  buffer: Lisp_Object,
)
{
  crate::safe_undo::record_property_change_impl (beg, length, prop, value, buffer);
}

DEFUN! ("undo-boundary", Fundo_boundary, Sundo_boundary, 0, 0, 0,
       doc: "Mark a boundary between units of undo.\nAn undo command will stop at this point,\nbut another undo command will undo to the previous boundary.",
       (),
       {
  crate::safe_undo::undo_boundary_subr ()
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn truncate_undo_list (b: *mut buffer)
{
  assert! (!b.is_null ());
  let buf_obj = make_lisp_ptr (b as *mut c_void, Lisp_Type::Lisp_Vectorlike);
  crate::safe_undo::truncate_undo_list_obj (buf_obj);
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_undo ()
{
  DEFSYM! (Qinhibit_read_only, "inhibit-read-only");
  DEFSYM! (Qundo_auto__last_boundary_cause, "undo-auto--last-boundary-cause");
  DEFSYM! (Qexplicit, "explicit");
  DEFSYM! (Qapply, "apply");

  pending_boundary.set (Qnil ());
  staticpro_global (&pending_boundary);

  defsubr_global (&Sundo_boundary);

  DEFVAR_INT! (c"undo-limit", undo_limit,
               "Keep no more undo information once it exceeds this size.\nThis limit is applied when garbage collection happens.\nWhen a previous command increases the total undo list size past this\nvalue, the earlier commands that came before it are forgotten.\n\nThe size is counted as the number of bytes occupied,\nwhich includes both saved text and other data.");
  undo_limit.set (160000);

  DEFVAR_INT! (c"undo-strong-limit", undo_strong_limit,
               "Don't keep more than this much size of undo information.\nThis limit is applied when garbage collection happens.\nWhen a previous command increases the total undo list size past this\nvalue, that command and the earlier commands that came before it are forgotten.\nHowever, the most recent buffer-modifying command's undo info\nis never discarded for this reason.\n\nThe size is counted as the number of bytes occupied,\nwhich includes both saved text and other data.");
  undo_strong_limit.set (240000);

  DEFVAR_LISP! (c"undo-outer-limit", Vundo_outer_limit,
                "Outer limit on size of undo information for one command.\nAt garbage collection time, if the current command has produced\nmore than this much undo information, it discards the info and displays\na warning.  This is a last-ditch limit to prevent memory overflow.\n\nThe size is counted as the number of bytes occupied, which includes\nboth saved text and other data.  A value of nil means no limit.  In\nthis case, accumulating one huge undo entry could make Emacs crash as\na result of memory overflow.\n\nIn fact, this calls the function which is the value of\n`undo-outer-limit-function' with one argument, the size.\nThe text above describes the behavior of the function\nthat variable usually specifies.");
  Vundo_outer_limit.set (make_fixnum (24000000));

  DEFVAR_LISP! (c"undo-outer-limit-function", Vundo_outer_limit_function,
                "Function to call when an undo list exceeds `undo-outer-limit'.\nThis function is called with one argument, the current undo list size\nfor the most recent command (since the last undo boundary).\nIf the function returns t, that means truncation has been fully handled.\nIf it returns nil, the other forms of truncation are done.\n\nGarbage collection is inhibited around the call to this function,\nso it must make sure not to do a lot of consing.");
  Vundo_outer_limit_function.set (Qnil ());

  DEFVAR_BOOL! (c"undo-inhibit-record-point", undo_inhibit_record_point,
                "Non-nil means do not record `point' in `buffer-undo-list'.");
  undo_inhibit_record_point.set (false);
}
