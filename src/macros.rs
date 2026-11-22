// femacs: port of src/macros.c @ fd7afced403.
//
// Logic lives in `#![forbid(unsafe_code)]` `crate::safe_macros` (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

#[unsafe(no_mangle)]
pub static Vkbd_macro_termination_hook: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vexecuting_kbd_macro: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static executing_kbd_macro_index: VmGlobal<EMACS_INT> = VmGlobal::<EMACS_INT>::new (0);
#[unsafe(no_mangle)]
pub static executing_kbd_macro_iterations: VmGlobal<EMACS_INT> = VmGlobal::<EMACS_INT>::new (0);
#[unsafe(no_mangle)]
pub static executing_kbd_macro: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

DEFUN! ("start-kbd-macro", Fstart_kbd_macro, Sstart_kbd_macro, 1, 2, "P",
       doc: "Record subsequent keyboard input, defining a keyboard macro.\nThe commands are recorded even as they are executed.\nUse \\[end-kbd-macro] to finish recording and make the macro available.\nUse \\[name-last-kbd-macro] to give it a permanent name.\nNon-nil arg (prefix arg) means append to last macro defined;\nthis begins by re-executing that macro as if you typed it again.\nIf optional second arg, NO-EXEC, is non-nil, do not re-execute last\nmacro before appending to it.",
       (append: Lisp_Object, no_exec: Lisp_Object),
       {
  crate::safe_macros::start_kbd_macro_subr (append, no_exec)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn end_kbd_macro ()
{
  crate::safe_macros::end_kbd_macro_impl ();
}

DEFUN! ("end-kbd-macro", Fend_kbd_macro, Send_kbd_macro, 0, 2, "p",
       doc: "Finish defining a keyboard macro.\nThe definition was started by \\[start-kbd-macro].\nThe macro is now available for use via \\[call-last-kbd-macro],\nor it can be given a name with \\[name-last-kbd-macro] and then invoked\nunder that name.\n\nWith numeric arg, repeat macro now that many times,\ncounting the definition just completed as the first repetition.\nAn argument of zero means repeat until error.\n\nIn Lisp, optional second arg LOOPFUNC may be a function that is called prior to\neach iteration of the macro.  Iteration stops if LOOPFUNC returns nil.",
       (repeat: Lisp_Object, loopfunc: Lisp_Object),
       {
  crate::safe_macros::end_kbd_macro_subr (repeat, loopfunc)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn store_kbd_macro_char (c: Lisp_Object)
{
  crate::safe_macros::store_kbd_macro_char_impl (c);
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn finalize_kbd_macro_chars ()
{
  crate::safe_macros::finalize_kbd_macro_chars_impl ();
}

DEFUN! ("cancel-kbd-macro-events", Fcancel_kbd_macro_events,
       Scancel_kbd_macro_events, 0, 0, 0,
       doc: "Cancel the events added to a keyboard macro for this command.",
       (),
       {
  crate::safe_macros::cancel_kbd_macro_events_subr ()
});

DEFUN! ("store-kbd-macro-event", Fstore_kbd_macro_event,
       Sstore_kbd_macro_event, 1, 1, 0,
       doc: "Store EVENT into the keyboard macro being defined.",
       (event: Lisp_Object),
       {
  crate::safe_macros::store_kbd_macro_event_subr (event)
});

DEFUN! ("call-last-kbd-macro", Fcall_last_kbd_macro, Scall_last_kbd_macro,
       0, 2, "p",
       doc: "Call the last keyboard macro that you defined with \\[start-kbd-macro].\n\nA prefix argument serves as a repeat count.  Zero means repeat until error.\n\nTo make a macro permanent so you can call it even after\ndefining others, use \\[name-last-kbd-macro].\n\nIn Lisp, optional second arg LOOPFUNC may be a function that is called prior to\neach iteration of the macro.  Iteration stops if LOOPFUNC returns nil.",
       (prefix: Lisp_Object, loopfunc: Lisp_Object),
       {
  crate::safe_macros::call_last_kbd_macro_subr (prefix, loopfunc)
});

DEFUN! ("execute-kbd-macro", Fexecute_kbd_macro, Sexecute_kbd_macro, 1, 3, 0,
       doc: "Execute MACRO as a sequence of events.\nIf MACRO is a string or vector, then the events in it are executed\nexactly as if they had been input by the user.\n\nIf MACRO is a symbol, its function definition is used.  If that is\nanother symbol, this process repeats.  Eventually the result should be\na string or vector.  If the result is not a symbol, string, or vector,\nan error is signaled.\n\nCOUNT is a repeat count, or nil for once, or 0 for infinite loop.\n\nOptional third arg LOOPFUNC may be a function that is called prior to\neach iteration of the macro.  Iteration stops if LOOPFUNC returns nil.\n\nThe buffer shown in the currently selected window will be made the current\nbuffer before the macro is executed.",
       (macro_: Lisp_Object, count: Lisp_Object, loopfunc: Lisp_Object),
       {
  crate::safe_macros::execute_kbd_macro_subr (macro_, count, loopfunc)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn init_macros ()
{
  crate::safe_macros::init_macros_impl ();
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn at_end_of_macro_p () -> bool
{
  crate::safe_macros::at_end_of_macro_p_impl ()
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_macros ()
{
  DEFVAR_LISP! (c"kbd-macro-termination-hook", Vkbd_macro_termination_hook,
                "Normal hook run whenever a keyboard macro terminates.\nThis is run whether the macro ends normally or prematurely due to an error.");
  Vkbd_macro_termination_hook.set (Qnil ());
  DEFSYM! (Qkbd_macro_termination_hook, "kbd-macro-termination-hook");

  defsubr_global (&Sstart_kbd_macro);
  defsubr_global (&Send_kbd_macro);
  defsubr_global (&Scall_last_kbd_macro);
  defsubr_global (&Sexecute_kbd_macro);
  defsubr_global (&Scancel_kbd_macro_events);
  defsubr_global (&Sstore_kbd_macro_event);

  DEFVAR_KBOARD! (c"defining-kbd-macro",
                  std::mem::offset_of! (kboard, defining_kbd_macro_),
                  "Non-nil while a keyboard macro is being defined.  Don't set this!\nThe value is the symbol `append' while appending to the definition of\nan existing macro.");

  executing_kbd_macro.set (Qnil ());
  staticpro_global (&executing_kbd_macro);

  DEFVAR_LISP! (c"executing-kbd-macro", Vexecuting_kbd_macro,
                "Currently executing keyboard macro (string or vector).\nThis is nil when not executing a keyboard macro.");

  DEFVAR_INT! (c"executing-kbd-macro-index", executing_kbd_macro_index,
               "Index in currently executing keyboard macro; undefined if none executing.");

  DEFVAR_KBOARD! (c"last-kbd-macro", std::mem::offset_of! (kboard, Vlast_kbd_macro_),
                  "Last kbd macro defined, as a string or vector; nil if none defined.");
}
