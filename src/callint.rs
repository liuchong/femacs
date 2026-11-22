// femacs: port of src/callint.c @ fd7afced403.
//
// Logic lives in `#![forbid(unsafe_code)]` `crate::safe_callint` (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

#[unsafe(no_mangle)]
pub static Vcurrent_prefix_arg: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vcommand_history: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vcommand_debug_status: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vmark_even_if_inactive: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vmouse_leave_buffer_hook: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static inhibit_mouse_event_check: VmGlobal<bool> = VmGlobal::<bool>::new (false);

pub(crate) static preserved_fns: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
pub(crate) static point_marker: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
pub(crate) static callint_message: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

DEFUN! ("interactive", Finteractive, Sinteractive, 0, UNEVALLED, 0,
       doc: "Specify a way of parsing arguments for interactive use of a function.\nFor example, write\n (defun foo (arg buf) \"Doc string\" (interactive \"P\\nbbuffer: \") .... )\n to make ARG be the raw prefix argument, and set BUF to an existing buffer,\n when `foo' is called as a command.\n\nThe \"call\" to `interactive' is actually a declaration rather than a\n function; it tells `call-interactively' how to read arguments to pass\n to the function.  When actually called, `interactive' just returns\n nil.\n\nUsually the argument of `interactive' is a string containing a code\n letter followed optionally by a prompt.  (Some code letters do not\n use I/O to get the argument and do not use prompts.)  To pass several\n arguments to the command, concatenate the individual strings,\n separating them by newline characters.\n\nPrompts are passed to `format', and may use %s escapes to print the\n arguments that have already been read.\n\nIf the argument is not a string, it is evaluated to get a list of\n arguments to pass to the command.\n\nJust `(interactive)' means pass no arguments to the command when\n calling interactively.\n\nCode letters available are:\na -- Function name: symbol with a function definition.\nb -- Name of existing buffer.\nB -- Name of buffer, possibly nonexistent.\nc -- Character (no input method is used).\nC -- Command name: symbol with interactive function definition.\nd -- Value of point as number.  Does not do I/O.\nD -- Directory name.\ne -- Parameterized event (i.e., one that's a list) that invoked this command.\n     If used more than once, the Nth `e' returns the Nth parameterized event.\n     This skips events that are integers or symbols.\nf -- Existing file name.\nF -- Possibly nonexistent file name.\nG -- Possibly nonexistent file name, defaulting to just directory name.\ni -- Ignored, i.e. always nil.  Does not do I/O.\nk -- Key sequence (downcase the last event if needed to get a definition).\nK -- Key sequence to be redefined (do not downcase the last event).\nm -- Value of mark as number.  Does not do I/O.\nM -- Any string.  Inherits the current input method.\nn -- Number read using minibuffer.\nN -- Numeric prefix arg, or if none, do like code `n'.\np -- Prefix arg converted to number.  Does not do I/O.\nP -- Prefix arg in raw form.  Does not do I/O.\nr -- Region: point and mark as 2 numeric args, smallest first.  Does no I/O.\nR -- Active region: as `r' but both nil unless `use-region-p'.  Does no I/O.\ns -- Any string.  Does not inherit the current input method.\nS -- Any symbol.\nU -- Mouse up event discarded by a previous k or K argument.\nv -- Variable name: symbol that is `custom-variable-p'.\nx -- Lisp expression read but not evaluated.\nX -- Lisp expression read and evaluated.\nz -- Coding system.\nZ -- Coding system, nil if no prefix arg.\n\nIn addition, if the string begins with `*', an error is signaled if\n  the buffer is read-only.\nIf `@' appears at the beginning of the string, and if the key sequence\n used to invoke the command includes any mouse events, then the window\n associated with the first of those events is selected before the\n command is run.\nIf the string begins with `^' and `shift-select-mode' is non-nil,\n Emacs first calls the function `handle-shift-selection'.\nYou may use `@', `*', and `^' together.  They are processed in the\n order that they appear, before reading any arguments.\n\nIf MODES is present, it should be one or more mode names (symbols)\nfor which this command is applicable.  This is so that `M-x TAB'\nwill be able to exclude this command from the list of completion\ncandidates if the current buffer's mode doesn't match the list.\nWhich commands are excluded from the list of completion\ncandidates based on this information is controlled by the value\nof `read-extended-command-predicate', which see.\n\nusage: (interactive &optional ARG-DESCRIPTOR &rest MODES)  ",
       (args: Lisp_Object),
       {
  crate::safe_callint::interactive_subr (args)
});

DEFUN! ("funcall-interactively", Ffuncall_interactively, Sfuncall_interactively,
       1, MANY, 0, doc: "Like `funcall' but marks the call as interactive.\nI.e. arrange that within the called function `called-interactively-p' will\nreturn non-nil.\nusage: (funcall-interactively FUNCTION &rest ARGUMENTS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  // SAFETY: `MANY` subr ABI guarantees `args` points to `nargs >= 1` rooted `Lisp_Object`s.
  let slice = unsafe { core::slice::from_raw_parts (args, nargs as usize) };
  crate::safe_callint::funcall_interactively_slice (slice)
});

DEFUN! ("call-interactively", Fcall_interactively, Scall_interactively, 1, 3, 0,
       doc: "Call FUNCTION, providing args according to its interactive calling specs.\nReturn the value FUNCTION returns.\nThe function contains a specification of how to do the argument reading.\nIn the case of user-defined functions, this is specified by placing a call\nto the function `interactive' at the top level of the function body.\nSee `interactive'.\n\nOptional second arg RECORD-FLAG non-nil\nmeans unconditionally put this command in the variable `command-history'.\nOtherwise, this is done only if an arg is read using the minibuffer.\n\nOptional third arg KEYS, if given, specifies the sequence of events to\nsupply, as a vector, if FUNCTION inquires which events were used to\ninvoke it (via an `interactive' spec that contains, for instance, an\n\"e\" code letter).  If KEYS is omitted or nil, the return value of\n`this-command-keys-vector' is used.",
       (function: Lisp_Object, record_flag: Lisp_Object, keys: Lisp_Object),
       {
  crate::safe_callint::call_interactively (function, record_flag, keys)
});

DEFUN! ("prefix-numeric-value", Fprefix_numeric_value, Sprefix_numeric_value,
       1, 1, 0,
       doc: "Return numeric meaning of raw prefix argument RAW.\nA raw prefix argument is what you get from `(interactive \"P\")'.\nIts numeric meaning is what you would get from `(interactive \"p\")'.  ",
       (raw: Lisp_Object),
       {
  crate::safe_callint::prefix_numeric_value (raw)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_callint ()
{
  crate::safe_callint::init_syms_of_callint ();

  DEFSYM! (Qlist, "list");
  DEFSYM! (Qlet, "let");
  DEFSYM! (Qif, "if");
  DEFSYM! (Qwhen, "when");
  DEFSYM! (Qletx, "let*");
  DEFSYM! (Qsave_excursion, "save-excursion");
  DEFSYM! (Qprogn, "progn");
  DEFSYM! (Qminus, "-");
  DEFSYM! (Qplus, "+");
  DEFSYM! (Qhandle_shift_selection, "handle-shift-selection");
  DEFSYM! (Qread_number, "read-number");
  DEFSYM! (Qfuncall_interactively, "funcall-interactively");
  DEFSYM! (Qcommand_debug_status, "command-debug-status");
  DEFSYM! (Qenable_recursive_minibuffers, "enable-recursive-minibuffers");
  DEFSYM! (Qmouse_leave_buffer_hook, "mouse-leave-buffer-hook");

  DEFVAR_KBOARD! (c"prefix-arg", std::mem::offset_of! (kboard, Vprefix_arg_),
		  "The value of the prefix argument for the next editing command.\nIt may be a number, or the symbol `-' for just a minus sign as arg,\nor a list whose car is a number for just one or more C-u's\nor nil if no argument has been specified.\n\nYou cannot examine this variable to find the argument for this command\nsince it has been set to nil by the time you can look.\nInstead, you should use the variable `current-prefix-arg', although\nnormally commands can get this prefix argument with (interactive \"P\").  ");

  DEFVAR_KBOARD! (c"last-prefix-arg", std::mem::offset_of! (kboard, Vlast_prefix_arg_),
		  "The value of the prefix argument for the previous editing command.\nSee `prefix-arg' for the meaning of the value.  ");

  DEFVAR_LISP! (c"current-prefix-arg", Vcurrent_prefix_arg,
		"The value of the prefix argument for this editing command.\nIt may be a number, or the symbol `-' for just a minus sign as arg,\nor a list whose car is a number for just one or more C-u's\nor nil if no argument has been specified.\nThis is what `(interactive \"P\")' returns.  ");
  Vcurrent_prefix_arg.set (Qnil ());

  DEFVAR_LISP! (c"command-history", Vcommand_history,
		"List of recent commands that read arguments from terminal.\nEach command is represented as a form to evaluate.\n\nMaximum length of the history list is determined by the value\nof `history-length', which see.  ");
  Vcommand_history.set (Qnil ());

  DEFVAR_LISP! (c"command-debug-status", Vcommand_debug_status,
		"Debugging status of current interactive command.\nBound each time `call-interactively' is called;\nmay be set by the debugger as a reminder for itself.  ");
  Vcommand_debug_status.set (Qnil ());

  DEFVAR_LISP! (c"mark-even-if-inactive", Vmark_even_if_inactive,
		"Non-nil means you can use the mark even when inactive.\nThis option makes a difference in Transient Mark mode.\nWhen the option is non-nil, deactivation of the mark\nturns off region highlighting, but commands that use the mark\nbehave as if the mark were still active.  ");
  Vmark_even_if_inactive.set (Qt ());

  DEFVAR_LISP! (c"mouse-leave-buffer-hook", Vmouse_leave_buffer_hook,
		"Hook run when the user mouse-clicks in a window.\nIt can be run both before and after switching windows, or even when\nnot actually switching windows.\n\nIts purpose is to give temporary modes such as Isearch mode\na way to turn themselves off when a mouse command switches windows.  ");
  Vmouse_leave_buffer_hook.set (Qnil ());

  DEFVAR_BOOL! (c"inhibit-mouse-event-check", inhibit_mouse_event_check,
    "Whether the interactive spec \"e\" requires a mouse gesture event.\nIf non-nil, `(interactive \"e\")' doesn't signal an error when the command\nwas invoked by an input event that is not a mouse gesture: a click, a drag,\netc.  To create the event data when the input was some other event,\nuse `event-start', `event-end', and `event-click-count'.  ");
  inhibit_mouse_event_check.set (false);

  defsubr_global (&Sinteractive);
  defsubr_global (&Scall_interactively);
  defsubr_global (&Sfuncall_interactively);
  defsubr_global (&Sprefix_numeric_value);

  DEFSYM! (Qinteractive_args, "interactive-args");
  DEFSYM! (Qread_file_name, "read-file-name");
  DEFSYM! (Qcommand_history, "command-history");
  DEFSYM! (Qeval_minibuffer, "eval-minibuffer");
  DEFSYM! (Quse_region_p, "use-region-p");
}
