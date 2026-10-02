// femacs: port of src/macros.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The six
// subrs use `DEFUN!` (PORTING.md 5.1).
//
// The `#include` list mirrors macros.c 24-27 - config.h, lisp.h, macros.h, window.h,
// keyboard.h - with the mirrors those pull in (`KVAR` is keyboard.h's token-pasting macro;
// the port writes the field it pastes, `defining_kbd_macro_`, at each use site, which
// docs/ports/excluded-macros.txt records).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Keyboard macros.

Copyright (C) 1985-1986, 1993, 2000-2026 Free Software Foundation, Inc.

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

include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("include/lisp.rs");
include!("include/macros.rs");
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
include!("include/keyboard.rs");
include!("include/termhooks.rs");
include!("include/window.rs");
include!("include/process.rs");
include!("include/frame.rs");
include!("include/keymap.rs");

/* Number of successful iterations so far
   for innermost keyboard macro.
   This is not bound at each level,
   so after an error, it describes the innermost interrupted macro.  */

// macros.c 37: `EMACS_INT executing_kbd_macro_iterations;` - a bss object, so the port's
// zero is C's zero (the `UNINIT` class of the existing ledgers).
#[unsafe(no_mangle)]
pub static mut executing_kbd_macro_iterations: EMACS_INT = 0;

/* This is the macro that was executing.
   This is not bound at each level,
   so after an error, it describes the innermost interrupted macro.
   We use it only as a kind of flag; it could be a simple bool, but we
   keep the string/vector around to aid debugging.  */

#[unsafe(no_mangle)]
pub static mut executing_kbd_macro: Lisp_Object = lisp_h_Qnil ();

DEFUN! ("start-kbd-macro", Fstart_kbd_macro, Sstart_kbd_macro, 1, 2, "P",
       doc: "Record subsequent keyboard input, defining a keyboard macro.\nThe commands are recorded even as they are executed.\nUse \\[end-kbd-macro] to finish recording and make the macro available.\nUse \\[name-last-kbd-macro] to give it a permanent name.\nNon-nil arg (prefix arg) means append to last macro defined;\nthis begins by re-executing that macro as if you typed it again.\nIf optional second arg, NO-EXEC, is non-nil, do not re-execute last\nmacro before appending to it.",
       (append: Lisp_Object, no_exec: Lisp_Object),
       {
  if ! NILP ((*current_kboard).defining_kbd_macro_)
    { error (c"Already defining kbd macro".as_ptr ()); }

  if (*current_kboard).kbd_macro_buffer.is_null ()
    {
      (*current_kboard).kbd_macro_buffer
        = xmalloc ((30 * word_size) as usize) as *mut Lisp_Object;
      (*current_kboard).kbd_macro_bufsize = 30;
      (*current_kboard).kbd_macro_ptr = (*current_kboard).kbd_macro_buffer;
      (*current_kboard).kbd_macro_end = (*current_kboard).kbd_macro_buffer;
    }
  update_mode_lines = 19;
  if NILP (append)
    {
      if (*current_kboard).kbd_macro_bufsize > 200
        {
          (*current_kboard).kbd_macro_buffer
            = xrealloc ((*current_kboard).kbd_macro_buffer as *mut c_void,
                        (30 * word_size) as usize) as *mut Lisp_Object;
          (*current_kboard).kbd_macro_bufsize = 30;
        }
      (*current_kboard).kbd_macro_ptr = (*current_kboard).kbd_macro_buffer;
      (*current_kboard).kbd_macro_end = (*current_kboard).kbd_macro_buffer;
      message1 (c"Defining kbd macro...".as_ptr ());
    }
  else
    {
      let mut incr: c_int = 30;
      let mut i: ptrdiff_t;
      let mut len: ptrdiff_t;
      let mut cvt: bool;

      /* Check the type of last-kbd-macro in case Lisp code changed it.  */
      len = CHECK_VECTOR_OR_STRING ((*current_kboard).Vlast_kbd_macro_);

      /* Copy last-kbd-macro into the buffer, in case the Lisp code
	 has put another macro there.  */
      if ((*current_kboard).kbd_macro_bufsize - incr as ptrdiff_t) < len
        {
          (*current_kboard).kbd_macro_buffer =
            xpalloc ((*current_kboard).kbd_macro_buffer as *mut c_void,
                     &raw mut (*current_kboard).kbd_macro_bufsize,
                     len - (*current_kboard).kbd_macro_bufsize + incr as ptrdiff_t, -1,
                     std::mem::size_of::<Lisp_Object> () as ptrdiff_t)
              as *mut Lisp_Object;
        }

      /* Must convert meta modifier when copying string to vector.  */
      cvt = STRINGP ((*current_kboard).Vlast_kbd_macro_);
      i = 0;
      while i < len
        {
          let mut c: Lisp_Object;
          c = Faref ((*current_kboard).Vlast_kbd_macro_, make_fixnum (i));
          if cvt && FIXNATP (c) && (XFIXNAT (c) & 0x80) != 0
            { XSETFASTINT! (c, CHAR_META as EMACS_INT | (XFIXNAT (c) & ! 0x80)); }
          unsafe { *(*current_kboard).kbd_macro_buffer.offset (i) = c };
          i += 1;
        }

      (*current_kboard).kbd_macro_ptr = (*current_kboard).kbd_macro_buffer.offset (len);
      (*current_kboard).kbd_macro_end = (*current_kboard).kbd_macro_ptr;

      /* Re-execute the macro we are appending to,
	 for consistency of behavior.  */
      if NILP (no_exec)
        {
          Fexecute_kbd_macro ((*current_kboard).Vlast_kbd_macro_,
                              make_fixnum (1), Qnil ());
        }

      message1 (c"Appending to kbd macro...".as_ptr ());
    }
  kset_defining_kbd_macro (current_kboard, Qt ());

  return Qnil ();
});

/* Finish defining the current keyboard macro.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_kbd_macro ()
{
  kset_defining_kbd_macro (current_kboard, Qnil ());
  update_mode_lines = 20;
  kset_last_kbd_macro
    (current_kboard,
     Fvector ((*current_kboard).kbd_macro_end
                .offset_from ((*current_kboard).kbd_macro_buffer),
              (*current_kboard).kbd_macro_buffer));
}

DEFUN! ("end-kbd-macro", Fend_kbd_macro, Send_kbd_macro, 0, 2, "p",
       doc: "Finish defining a keyboard macro.\nThe definition was started by \\[start-kbd-macro].\nThe macro is now available for use via \\[call-last-kbd-macro],\nor it can be given a name with \\[name-last-kbd-macro] and then invoked\nunder that name.\n\nWith numeric arg, repeat macro now that many times,\ncounting the definition just completed as the first repetition.\nAn argument of zero means repeat until error.\n\nIn Lisp, optional second arg LOOPFUNC may be a function that is called prior to\neach iteration of the macro.  Iteration stops if LOOPFUNC returns nil.",
       (repeat: Lisp_Object, loopfunc: Lisp_Object),
       {
  let mut repeat: Lisp_Object = repeat;
  if NILP ((*current_kboard).defining_kbd_macro_)
    { error (c"Not defining kbd macro".as_ptr ()); }

  if NILP (repeat)
    { XSETFASTINT! (repeat, 1); }
  else
    { CHECK_FIXNUM (repeat); }

  if ! NILP ((*current_kboard).defining_kbd_macro_)
    {
      end_kbd_macro ();
      message1 (c"Keyboard macro defined".as_ptr ());
    }

  if XFIXNAT (repeat) == 0
    { Fexecute_kbd_macro ((*current_kboard).Vlast_kbd_macro_, repeat, loopfunc); }
  else if XFIXNUM (repeat) > 1
    {
      XSETINT! (repeat, XFIXNUM (repeat) - 1);
      Fexecute_kbd_macro ((*current_kboard).Vlast_kbd_macro_, repeat, loopfunc);
    }
  return Qnil ();
});

/* Store character c into kbd macro being defined.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_kbd_macro_char (c: Lisp_Object)
{
  let kb: *mut kboard = current_kboard;

  if ! NILP ((*kb).defining_kbd_macro_)
    {
      if (*kb).kbd_macro_ptr.offset_from ((*kb).kbd_macro_buffer)
         == (*kb).kbd_macro_bufsize
        {
          let ptr_offset: ptrdiff_t = (*kb).kbd_macro_ptr.offset_from ((*kb).kbd_macro_buffer);
          let end_offset: ptrdiff_t = (*kb).kbd_macro_end.offset_from ((*kb).kbd_macro_buffer);
          (*kb).kbd_macro_buffer
            = xpalloc ((*kb).kbd_macro_buffer as *mut c_void,
                       &raw mut (*kb).kbd_macro_bufsize,
                       1, -1, std::mem::size_of::<Lisp_Object> () as ptrdiff_t)
              as *mut Lisp_Object;
          (*kb).kbd_macro_ptr = (*kb).kbd_macro_buffer.offset (ptr_offset);
          (*kb).kbd_macro_end = (*kb).kbd_macro_buffer.offset (end_offset);
        }

      unsafe { *(*kb).kbd_macro_ptr = c };
      (*kb).kbd_macro_ptr = (*kb).kbd_macro_ptr.offset (1);
    }
}

/* Declare that all chars stored so far in the kbd macro being defined
 really belong to it.  This is done in between editor commands.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn finalize_kbd_macro_chars ()
{
  (*current_kboard).kbd_macro_end = (*current_kboard).kbd_macro_ptr;
}

DEFUN! ("cancel-kbd-macro-events", Fcancel_kbd_macro_events,
       Scancel_kbd_macro_events, 0, 0, 0,
       doc: "Cancel the events added to a keyboard macro for this command.",
       (),
       {
  (*current_kboard).kbd_macro_ptr = (*current_kboard).kbd_macro_end;
  return Qnil ();
});

DEFUN! ("store-kbd-macro-event", Fstore_kbd_macro_event,
       Sstore_kbd_macro_event, 1, 1, 0,
       doc: "Store EVENT into the keyboard macro being defined.",
       (event: Lisp_Object),
       {
  store_kbd_macro_char (event);
  return Qnil ();
});

DEFUN! ("call-last-kbd-macro", Fcall_last_kbd_macro, Scall_last_kbd_macro,
       0, 2, "p",
       doc: "Call the last keyboard macro that you defined with \\[start-kbd-macro].\n\nA prefix argument serves as a repeat count.  Zero means repeat until error.\n\nTo make a macro permanent so you can call it even after\ndefining others, use \\[name-last-kbd-macro].\n\nIn Lisp, optional second arg LOOPFUNC may be a function that is called prior to\neach iteration of the macro.  Iteration stops if LOOPFUNC returns nil.",
       (prefix: Lisp_Object, loopfunc: Lisp_Object),
       {
  /* Don't interfere with recognition of the previous command
     from before this macro started.  */
  Vthis_command = (*current_kboard).Vlast_command_;
  /* C-x z after the macro should repeat the macro.  */
  Vreal_this_command = (*current_kboard).Vlast_kbd_macro_;

  if ! NILP ((*current_kboard).defining_kbd_macro_)
    { error (c"Can't execute anonymous macro while defining one".as_ptr ()); }
  else if NILP ((*current_kboard).Vlast_kbd_macro_)
    { error (c"No kbd macro has been defined".as_ptr ()); }
  else
    { Fexecute_kbd_macro ((*current_kboard).Vlast_kbd_macro_, prefix, loopfunc); }

  /* command_loop_1 sets this to nil before it returns;
     get back the last command within the macro
     so that it can be last, again, after we return.  */
  Vthis_command = (*current_kboard).Vlast_command_;

  return Qnil ();
});

/* Restore Vexecuting_kbd_macro and executing_kbd_macro_index.
   Called when the unwind-protect in Fexecute_kbd_macro gets invoked.  */

unsafe extern "C" fn pop_kbd_macro (info: Lisp_Object)
{
  let mut tem: Lisp_Object;
  Vexecuting_kbd_macro = XCAR (info);
  tem = XCDR (info);
  // C's `&executing_kbd_macro_index` is `EMACS_INT *`, and EMACS_INT is intmax_t there;
  // the port's EMACS_INT is isize, so the pointer is cast to the mirror's intmax_t.
  integer_to_intmax (XCAR (tem), (&raw mut executing_kbd_macro_index) as *mut i64);
  Vreal_this_command = XCDR (tem);
  run_hook (Qkbd_macro_termination_hook ());
}

DEFUN! ("execute-kbd-macro", Fexecute_kbd_macro, Sexecute_kbd_macro, 1, 3, 0,
       doc: "Execute MACRO as a sequence of events.\nIf MACRO is a string or vector, then the events in it are executed\nexactly as if they had been input by the user.\n\nIf MACRO is a symbol, its function definition is used.  If that is\nanother symbol, this process repeats.  Eventually the result should be\na string or vector.  If the result is not a symbol, string, or vector,\nan error is signaled.\n\nCOUNT is a repeat count, or nil for once, or 0 for infinite loop.\n\nOptional third arg LOOPFUNC may be a function that is called prior to\neach iteration of the macro.  Iteration stops if LOOPFUNC returns nil.\n\nThe buffer shown in the currently selected window will be made the current\nbuffer before the macro is executed.",
       (macro_: Lisp_Object, count: Lisp_Object, loopfunc: Lisp_Object),
       {
  let mut count: Lisp_Object = count;
  let mut final_: Lisp_Object;
  let mut tem: Lisp_Object;
  let pdlcount: specpdl_ref = SPECPDL_INDEX ();
  let mut repeat: EMACS_INT = 1;
  let mut success_count: EMACS_INT = 0;

  executing_kbd_macro_iterations = 0;

  if ! NILP (count)
    {
      count = Fprefix_numeric_value (count);
      repeat = XFIXNUM (count);
    }

  final_ = indirect_function (macro_);
  if ! STRINGP (final_) && ! VECTORP (final_)
    { error (c"Keyboard macros must be strings or vectors".as_ptr ()); }

  tem = Fcons (Vexecuting_kbd_macro,
               Fcons (make_int (executing_kbd_macro_index as intmax_t),
                      Vreal_this_command));
  record_unwind_protect (Some (pop_kbd_macro), tem);

  /* The following loop starts the execution of possibly multiple
     iterations of the macro.

     The state variables that control the execution of a single
     iteration are Vexecuting_kbd_macro and executing_kbd_macro_index,
     which can be accessed from lisp. The purpose of the variables
     executing_kbd_macro and executing_kbd_macro_iteration is to
     remember the most recently started macro and its iteration count.
     This makes it possible to produce a meaningful message in case of
     errors during the execution of the macro.

     In a single iteration, individual characters from the macro are
     read by read_char, which takes care of incrementing
     executing_kbd_macro_index after each character.

     The end of a macro iteration is handled as follows:
      - read_key_sequence asks at_end_of_macro_p whether the end of the
        iteration has been reached.  If so, it returns the magic value 0
        to command_loop_1.
      - command_loop_1 returns Qnil to command_loop_2.
      - command_loop_2 returns Qnil to this function
        (but only the returning is relevant, not the actual value).

     Macro executions form a stack.  After the last iteration of the
     execution of one stack item, or in case of an error during one of
     the iterations, pop_kbd_macro (invoked via unwind-protect) will
     restore Vexecuting_kbd_macro and executing_kbd_macro_index, and
     run 'kbd-macro-termination-hook'.

     If read_char happens to be called at the end of a macro iteration,
     but before read_key_sequence could handle the end (e.g., when lisp
     code calls 'read-event', 'read-char', or 'read-char-exclusive'),
     read_char will simply continue reading other available input
     (Bug#68272).  Vexecuting_kbd_macro and executing_kbd_macro remain
     untouched until the end of the iteration is handled.

     This is similar (in observable behavior) to a possibly simpler
     implementation of keyboard macros in which this function pushed all
     characters of the macro into the incoming event queue and returned
     immediately.  Maybe this is the implementation that we ideally
     would like to have, but switching to it will require a larger code
     change.  */
  loop
    {
      Vexecuting_kbd_macro = final_;
      executing_kbd_macro = final_;
      executing_kbd_macro_index = 0;

      kset_prefix_arg (current_kboard, Qnil ());

      if ! NILP (loopfunc)
        {
          let cont: Lisp_Object = call0 (loopfunc);
          if NILP (cont)
            { break; }
        }

      command_loop_2 (list1 (Qminibuffer_quit ()));

      success_count += 1;
      executing_kbd_macro_iterations = success_count;

      maybe_quit ();

      repeat -= 1;
      if ! (repeat != 0
            && (STRINGP (Vexecuting_kbd_macro) || VECTORP (Vexecuting_kbd_macro)))
        { break; }
    }

  executing_kbd_macro = Qnil ();

  Vreal_this_command = Vexecuting_kbd_macro;

  return unbind_to (pdlcount, Qnil ());
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_macros ()
{
  Vexecuting_kbd_macro = Qnil ();
  executing_kbd_macro = Qnil ();
}

/* Whether the execution of a macro has reached its end.
   This should be called only while executing a macro.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn at_end_of_macro_p () -> bool
{
  eassume! (! NILP (Vexecuting_kbd_macro));
  /* Some things replace the macro with t to force an early exit.  */
  return EQ (Vexecuting_kbd_macro, Qt ())
    || executing_kbd_macro_index >= XFIXNAT (Flength (Vexecuting_kbd_macro));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_macros ()
{
  DEFVAR_LISP! (c"kbd-macro-termination-hook", Vkbd_macro_termination_hook,
                "Normal hook run whenever a keyboard macro terminates.\nThis is run whether the macro ends normally or prematurely due to an error.");
  Vkbd_macro_termination_hook = Qnil ();
  DEFSYM! (Qkbd_macro_termination_hook, "kbd-macro-termination-hook");

  defsubr (&raw mut Sstart_kbd_macro);
  defsubr (&raw mut Send_kbd_macro);
  defsubr (&raw mut Scall_last_kbd_macro);
  defsubr (&raw mut Sexecute_kbd_macro);
  defsubr (&raw mut Scancel_kbd_macro_events);
  defsubr (&raw mut Sstore_kbd_macro_event);

  DEFVAR_KBOARD! (c"defining-kbd-macro",
                  std::mem::offset_of! (kboard, defining_kbd_macro_),
                  "Non-nil while a keyboard macro is being defined.  Don't set this!\nThe value is the symbol `append' while appending to the definition of\nan existing macro.");

  executing_kbd_macro = Qnil ();
  staticpro (&raw const executing_kbd_macro);

  DEFVAR_LISP! (c"executing-kbd-macro", Vexecuting_kbd_macro,
                "Currently executing keyboard macro (string or vector).\nThis is nil when not executing a keyboard macro.");

  DEFVAR_INT! (c"executing-kbd-macro-index", executing_kbd_macro_index,
               "Index in currently executing keyboard macro; undefined if none executing.");

  DEFVAR_KBOARD! (c"last-kbd-macro", std::mem::offset_of! (kboard, Vlast_kbd_macro_),
                  "Last kbd macro defined, as a string or vector; nil if none defined.");
}
