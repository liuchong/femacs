// femacs: port of src/callint.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The four subrs
// use `DEFUN!` (PORTING.md 5.1); `interactive` is the one UNEVALLED subr here.
//
// Fcall_interactively's two `switch` fall-throughs and its one `goto` (C 636-656: `case 'N'`
// jumps to `have_prefix_arg:`, the middle of `case 'p'`, or falls through into `case 'n'`;
// C 658-679: `case 'R'` falls through into `case 'r'`) have no `match` counterpart, so the
// port keeps one copy of each shared body and chooses the entry, the way tparam.c's chain does;
// the notes file carries the `goto`/`FALLTHROUGH` tokens those two arms cannot spell.
//
// The three `static Lisp_Object`s are the port's `lisp_h_Qnil ()` (the C statics are
// zero-initialized and NIL_IS_ZERO holds, PORTING.md 1.3).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Call a Lisp function interactively.
   Copyright (C) 1985-1986, 1993-1995, 1997, 2000-2026 Free Software
   Foundation, Inc.

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
include!("include/termhooks.rs");
include!("include/keyboard.rs");
include!("include/window.rs");


/* C's `switch (*tem)` is a `match` on the character; the arms below name the letters the way
   C's `case 'a':` spells them.  */
const letter_a: c_int = b'a' as c_int;
const letter_b: c_int = b'b' as c_int;
const letter_B: c_int = b'B' as c_int;
const letter_c: c_int = b'c' as c_int;
const letter_C: c_int = b'C' as c_int;
const letter_d: c_int = b'd' as c_int;
const letter_D: c_int = b'D' as c_int;
const letter_e: c_int = b'e' as c_int;
const letter_f: c_int = b'f' as c_int;
const letter_F: c_int = b'F' as c_int;
const letter_G: c_int = b'G' as c_int;
const letter_i: c_int = b'i' as c_int;
const letter_k: c_int = b'k' as c_int;
const letter_K: c_int = b'K' as c_int;
const letter_U: c_int = b'U' as c_int;
const letter_m: c_int = b'm' as c_int;
const letter_M: c_int = b'M' as c_int;
const letter_N: c_int = b'N' as c_int;
const letter_n: c_int = b'n' as c_int;
const letter_p: c_int = b'p' as c_int;
const letter_P: c_int = b'P' as c_int;
const letter_R: c_int = b'R' as c_int;
const letter_r: c_int = b'r' as c_int;
const letter_s: c_int = b's' as c_int;
const letter_S: c_int = b'S' as c_int;
const letter_v: c_int = b'v' as c_int;
const letter_x: c_int = b'x' as c_int;
const letter_X: c_int = b'X' as c_int;
const letter_Z: c_int = b'Z' as c_int;
const letter_z: c_int = b'z' as c_int;
const letter_plus: c_int = b'+' as c_int;

/* C 732-750's `default:` arm: the invalid control letter error.  It is a helper so that the
   `case '+':` arm and the `default:` arm keep C's single body (a `match` has no fall-through
   label to share).  */
unsafe fn invalid_control_letter (tem: *const c_char, string: *const c_char,
				  string_len: ptrdiff_t) -> !
{
    /* How many bytes are left unprocessed in the specs string?
       (Note that this excludes the trailing null byte.)  */
    let bytes_left: ptrdiff_t = string_len - (tem as ptrdiff_t - string as ptrdiff_t);
    let letter: c_uint;

    /* If we have enough bytes left to treat the sequence as a
       character, show that character's codepoint; otherwise
       show only its first byte.  */
    if bytes_left >= BYTES_BY_CHAR_HEAD (*(tem as *const u8) as c_int) as ptrdiff_t
      { letter = STRING_CHAR (tem as *const u8) as c_uint; }
    else
      { letter = *(tem as *const u8) as c_uint; }

    error (c"Invalid control letter `%c' (#o%03o, #x%04x) in interactive calling string".as_ptr (),
	   letter as c_int, letter, letter);
}

static mut preserved_fns: Lisp_Object = lisp_h_Qnil ();

/* Marker used within call-interactively to refer to point.  */
static mut point_marker: Lisp_Object = lisp_h_Qnil ();

/* String for the prompt text used in Fcall_interactively.  */
static mut callint_message: Lisp_Object = lisp_h_Qnil ();

DEFUN! ("interactive", Finteractive, Sinteractive, 0, UNEVALLED, 0,
       doc: "Specify a way of parsing arguments for interactive use of a function.\nFor example, write\n (defun foo (arg buf) \"Doc string\" (interactive \"P\\nbbuffer: \") .... )\n to make ARG be the raw prefix argument, and set BUF to an existing buffer,\n when `foo' is called as a command.\n\nThe \"call\" to `interactive' is actually a declaration rather than a\n function; it tells `call-interactively' how to read arguments to pass\n to the function.  When actually called, `interactive' just returns\n nil.\n\nUsually the argument of `interactive' is a string containing a code\n letter followed optionally by a prompt.  (Some code letters do not\n use I/O to get the argument and do not use prompts.)  To pass several\n arguments to the command, concatenate the individual strings,\n separating them by newline characters.\n\nPrompts are passed to `format', and may use %s escapes to print the\n arguments that have already been read.\n\nIf the argument is not a string, it is evaluated to get a list of\n arguments to pass to the command.\n\nJust `(interactive)' means pass no arguments to the command when\n calling interactively.\n\nCode letters available are:\na -- Function name: symbol with a function definition.\nb -- Name of existing buffer.\nB -- Name of buffer, possibly nonexistent.\nc -- Character (no input method is used).\nC -- Command name: symbol with interactive function definition.\nd -- Value of point as number.  Does not do I/O.\nD -- Directory name.\ne -- Parameterized event (i.e., one that's a list) that invoked this command.\n     If used more than once, the Nth `e' returns the Nth parameterized event.\n     This skips events that are integers or symbols.\nf -- Existing file name.\nF -- Possibly nonexistent file name.\nG -- Possibly nonexistent file name, defaulting to just directory name.\ni -- Ignored, i.e. always nil.  Does not do I/O.\nk -- Key sequence (downcase the last event if needed to get a definition).\nK -- Key sequence to be redefined (do not downcase the last event).\nm -- Value of mark as number.  Does not do I/O.\nM -- Any string.  Inherits the current input method.\nn -- Number read using minibuffer.\nN -- Numeric prefix arg, or if none, do like code `n'.\np -- Prefix arg converted to number.  Does not do I/O.\nP -- Prefix arg in raw form.  Does not do I/O.\nr -- Region: point and mark as 2 numeric args, smallest first.  Does no I/O.\nR -- Active region: as `r' but both nil unless `use-region-p'.  Does no I/O.\ns -- Any string.  Does not inherit the current input method.\nS -- Any symbol.\nU -- Mouse up event discarded by a previous k or K argument.\nv -- Variable name: symbol that is `custom-variable-p'.\nx -- Lisp expression read but not evaluated.\nX -- Lisp expression read and evaluated.\nz -- Coding system.\nZ -- Coding system, nil if no prefix arg.\n\nIn addition, if the string begins with `*', an error is signaled if\n  the buffer is read-only.\nIf `@' appears at the beginning of the string, and if the key sequence\n used to invoke the command includes any mouse events, then the window\n associated with the first of those events is selected before the\n command is run.\nIf the string begins with `^' and `shift-select-mode' is non-nil,\n Emacs first calls the function `handle-shift-selection'.\nYou may use `@', `*', and `^' together.  They are processed in the\n order that they appear, before reading any arguments.\n\nIf MODES is present, it should be one or more mode names (symbols)\nfor which this command is applicable.  This is so that `M-x TAB'\nwill be able to exclude this command from the list of completion\ncandidates if the current buffer's mode doesn't match the list.\nWhich commands are excluded from the list of completion\ncandidates based on this information is controlled by the value\nof `read-extended-command-predicate', which see.\n\nusage: (interactive &optional ARG-DESCRIPTOR &rest MODES)  ",
       (args: Lisp_Object),
       {
  return Qnil ();
});

/* Quotify EXP: if EXP is constant, return it.
   If EXP is not constant, return (quote EXP).  */
unsafe fn quotify_arg (exp: Lisp_Object) -> Lisp_Object
{
  if CONSP (exp)
      || (SYMBOLP (exp)
	  && !NILP (exp) && !EQ (exp, Qt ()))
    { return list2 (Qquote (), exp); }

  return exp;
}

/* Modify EXP by quotifying each element (except the first).  */
unsafe fn quotify_args (exp: Lisp_Object) -> Lisp_Object
{
  let mut tail: Lisp_Object;
  let mut next: Lisp_Object;
  tail = exp;
  while CONSP (tail)
    {
      next = XCDR (tail);
      XSETCAR (tail, quotify_arg (XCAR (tail)));
      tail = next;
    }
  return exp;
}

static mut callint_argfuns: [*const c_char; 7]
  = [c"".as_ptr (), c"point".as_ptr (), c"mark".as_ptr (), c"region-beginning".as_ptr (),
     c"region-end".as_ptr (), c"use-region-beginning".as_ptr (), c"use-region-end".as_ptr ()];

unsafe fn check_mark (for_region: bool)
{
  let mut tem: Lisp_Object;
  tem = Fmarker_buffer (BVAR! (current_buffer, mark_));
  if NILP (tem) || (XBUFFER (tem) != current_buffer)
    { error (if for_region { c"The mark is not set now, so there is no region".as_ptr () }
	     else { c"The mark is not set now".as_ptr () }); }
  if !NILP (Vtransient_mark_mode) && NILP (Vmark_even_if_inactive)
      && NILP (BVAR! (current_buffer, mark_active_))
    { xsignal0 (Qmark_inactive ()); }
}

/* If FUNCTION has an `interactive-args' spec, replace relevant
   elements in VALUES with those forms instead.

   This function doesn't return a value because it modifies elements
   of VALUES to do its job.  */

unsafe fn fix_command (function: Lisp_Object, values: Lisp_Object)
{
  /* Quick exit if there's no values to alter.  */
  if !CONSP (values) || !SYMBOLP (function)
    { return; }

  let reps: Lisp_Object = Fget (function, Qinteractive_args ());

  if CONSP (reps)
    {
      let mut i: c_int = 0;
      let mut vals: Lisp_Object = values;

      while !NILP (vals)
	{
	  let rep: Lisp_Object = Fassq (make_fixnum (i as EMACS_INT), reps);
	  if !NILP (rep)
	    { Fsetcar (vals, XCDR (rep)); }
	  vals = XCDR (vals);
	  i += 1;
	}
    }

  /* If the list contains a bunch of trailing nil values, and they are
     optional, remove them from the list.  This makes navigating the
     history less confusing, since it doesn't contain a lot of
     parameters that aren't used.  */
  let arity: Lisp_Object = Ffunc_arity (function);
  /* We don't want to do this simplification if we have an &rest
     function, because (cl-defun foo (a &optional (b 'zot)) ..)
     etc.  */
  if FIXNUMP (XCAR (arity)) && FIXNUMP (XCDR (arity))
    {
      let mut final_: Lisp_Object = Qnil ();
      let mut final_i: ptrdiff_t = 0;
      let mut i: ptrdiff_t = 0;
      let mut tail: Lisp_Object = values;
      while CONSP (tail)
	{
	  if !NILP (XCAR (tail))
	    {
	      final_ = tail;
	      final_i = i;
	    }
	  tail = XCDR (tail);
	  i += 1;
	}

      /* Chop the trailing optional values.  */
      if final_i > 0 && final_i >= XFIXNUM (XCAR (arity)) - 1
	{ XSETCDR (final_, Qnil ()); }
    }
}

/* Helper function to call `read-file-name' from C.  */

unsafe fn read_file_name (default_filename: Lisp_Object, mustmatch: Lisp_Object,
			  initial: Lisp_Object, predicate: Lisp_Object) -> Lisp_Object
{
  return calln! (Qread_file_name (),
		 callint_message, Qnil (), default_filename,
		 mustmatch, initial, predicate);
}

/* BEWARE: Calling this directly from C would defeat the purpose!  */
DEFUN! ("funcall-interactively", Ffuncall_interactively, Sfuncall_interactively,
       1, MANY, 0, doc: "Like `funcall' but marks the call as interactive.\nI.e. arrange that within the called function `called-interactively-p' will\nreturn non-nil.\nusage: (funcall-interactively FUNCTION &rest ARGUMENTS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let speccount: specpdl_ref = SPECPDL_INDEX ();
  if !multiple_terminals_merge_keyboards
    { temporarily_switch_to_single_kboard (std::ptr::null_mut ()); }

  /* Nothing special to do here, all the work is inside
     `called-interactively-p'.  Which will look for us as a marker in the
     backtrace.  */
  return unbind_to (speccount, Ffuncall (nargs, args));
});

DEFUN! ("call-interactively", Fcall_interactively, Scall_interactively, 1, 3, 0,
       doc: "Call FUNCTION, providing args according to its interactive calling specs.\nReturn the value FUNCTION returns.\nThe function contains a specification of how to do the argument reading.\nIn the case of user-defined functions, this is specified by placing a call\nto the function `interactive' at the top level of the function body.\nSee `interactive'.\n\nOptional second arg RECORD-FLAG non-nil\nmeans unconditionally put this command in the variable `command-history'.\nOtherwise, this is done only if an arg is read using the minibuffer.\n\nOptional third arg KEYS, if given, specifies the sequence of events to\nsupply, as a vector, if FUNCTION inquires which events were used to\ninvoke it (via an `interactive' spec that contains, for instance, an\n\"e\" code letter).  If KEYS is omitted or nil, the return value of\n`this-command-keys-vector' is used.",
       (function: Lisp_Object, record_flag: Lisp_Object, keys: Lisp_Object),
       {
  let mut arg_from_tty: bool = false;
  let mut key_count: ptrdiff_t;
  let mut record_then_fail: bool = false;

  /* FIXME: We should probably specbind these vars in recursive
     _edit instead so they're automatically saved&restored as needed.  */
  let save_this_command: Lisp_Object = Vthis_command;
  let save_this_original_command: Lisp_Object = Vthis_original_command;
  let save_real_this_command: Lisp_Object = Vreal_this_command;
  let save_last_command: Lisp_Object = (*current_kboard).Vlast_command_;

  let mut keys: Lisp_Object = keys;
  if NILP (keys)
    {
      keys = this_command_keys;
      key_count = this_command_key_count;
    }
  else
    {
      CHECK_VECTOR (keys);
      key_count = ASIZE (keys);
    }

  /* Save this now, since use of minibuffer will clobber it.  */
  let prefix_arg: Lisp_Object = Vcurrent_prefix_arg;

  let enable: Lisp_Object = if SYMBOLP (function)
			      { Fget (function, Qenable_recursive_minibuffers ()) }
			    else
			      { Qnil () };

  /* If k or K discard an up-event, save it here so it can be retrieved with
     U.  */
  let mut up_event: Lisp_Object = Qnil ();

  /* Set SPECS to the interactive form, or barf if not interactive.  */
  let mut form: Lisp_Object = calln! (Qinteractive_form (), function);
  if !CONSP (form)
    { wrong_type_argument (Qcommandp (), function); }
  let mut specs: Lisp_Object = Fcar (XCDR (form));

  /* At this point the value of SPECS could help provide a way to
     specify how to represent the arguments in command history.
     The feature is not fully implemented.  */

  /* If SPECS is not a string, invent one.  */
  if !STRINGP (specs)
    {
      let funval: Lisp_Object = Findirect_function (function, Qt ());
      let events: uintmax_t = num_input_events;
      let env: Lisp_Object = if CLOSUREP (funval) && CONSP (AREF (funval, CLOSURE_CODE as isize))
			       { AREF (funval, CLOSURE_CONSTANTS as isize) } else { Qnil () };
      /* Compute the arg values using the user's expression.  */
      specs = Feval (specs, env);
      if events != num_input_events || !NILP (record_flag)
	{
	  /* We should record this command on the command history.
	     Make a copy of the list of values, for the command history,
	     and turn them into things we can eval.  */
	  let values: Lisp_Object = quotify_args (Fcopy_sequence (specs));
	  fix_command (function, values);
	  calln! (Qadd_to_history (), Qcommand_history (),
		  Fcons (function, values), Qnil (), Qt ());
	}

      Vthis_command = save_this_command;
      Vthis_original_command = save_this_original_command;
      Vreal_this_command = save_real_this_command;
      kset_last_command (current_kboard, save_last_command);

      return CALLN! (Fapply, Qfuncall_interactively (), function, specs);
    }

  /* SPECS is set to a string; use it as an interactive prompt.
     Copy it so that STRING will be valid even if a GC relocates SPECS.  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let string_len: ptrdiff_t = SBYTES (specs);
  let string: *mut c_char = SAFE_ALLOCA! (sa_avail, string_len + 1) as *mut c_char;
  std::ptr::copy_nonoverlapping (SDATA (specs) as *const c_void, string as *mut c_void,
				 (string_len + 1) as usize);
  let string_end: *mut c_char = string.offset (string_len as isize);

  /* The index of the next element of this_command_keys to examine for
     the 'e' interactive code.  Initialize it to point to the first
     event with parameters.  When `inhibit_mouse_event_check' is non-nil,
     the command can accept an event without parameters,
     so don't search for the event with parameters in this case.  */
  let mut next_event: ptrdiff_t = 0;
  if !inhibit_mouse_event_check
    {
      while next_event < key_count
	{
	  if EVENT_HAS_PARAMETERS (AREF (keys, next_event))
	    { break; }
	  next_event += 1;
	}
    }

  /* Handle special starting chars `*' and `@'.  Also `-'.  */
  /* Note that `+' is reserved for user extensions.  */
  let mut string: *mut c_char = string;
  loop
    {
      if *string == b'+' as c_char
	{ error (c"`+' is not used in `interactive' for ordinary commands".as_ptr ()); }
      else if *string == b'*' as c_char
	{
	  if !NILP (BVAR! (current_buffer, read_only_))
	    {
	      if !NILP (record_flag)
		{
		  let mut p: *mut c_char = string.offset (1);
		  while p < string_end
		    {
		      if !(*p == b'r' as c_char || *p == b'R' as c_char
			   || *p == b'p' as c_char || *p == b'P' as c_char
			   || *p == b'\n' as c_char)
			{ Fbarf_if_buffer_read_only (Qnil ()); }
		      p = p.offset (1);
		    }
		  record_then_fail = true;
		}
	      else
		{ Fbarf_if_buffer_read_only (Qnil ()); }
	    }
	}
      /* Ignore this for semi-compatibility with Lucid.  */
      else if *string == b'-' as c_char
	{ }
      else if *string == b'@' as c_char
	{
	  let mut w: Lisp_Object;
	  let event: Lisp_Object = if next_event < key_count
				     { AREF (keys, next_event) } else { Qnil () };
	  if EVENT_HAS_PARAMETERS (event)
	      && { w = XCDR (event); CONSP (w) }
	      && { w = XCAR (w); CONSP (w) }
	      && { w = XCAR (w); WINDOWP (w) }
	    {
	      if MINI_WINDOW_P (XWINDOW (w))
		  && !(minibuf_level > 0 && BASE_EQ (w, minibuf_window))
		{ error (c"Attempt to select inactive minibuffer window".as_ptr ()); }

	      /* If the current buffer wants to clean up, let it.  */
	      run_hook (Qmouse_leave_buffer_hook ());

	      Fselect_window (w, Qnil ());
	    }
	}
      else if *string == b'^' as c_char
	{ call0 (Qhandle_shift_selection ()); }
      else
	{ break; }
      string = string.offset (1);
    }

  /* Count the number of arguments, which is two (the function itself and
     `funcall-interactively') plus the number of arguments the interactive spec
     would have us give to the function.  */
  let mut nargs: ptrdiff_t = 2;
  let mut tem: *const c_char = string;
  while tem < string_end
    {
      /* 'r'/'R' specifications ("point and mark as 2 numeric args")
	 produce *two* arguments.  */
      nargs += 1 + ((*tem == b'r' as c_char || *tem == b'R' as c_char) as ptrdiff_t);
      tem = memchr (tem as *const c_void, b'\n' as c_int,
		    (string_len - (tem as ptrdiff_t - string as ptrdiff_t)) as usize)
	as *const c_char;
      if tem.is_null ()
	{ break; }
    }

  if MOST_POSITIVE_FIXNUM < min (PTRDIFF_MAX, SIZE_MAX as ptrdiff_t) / word_size
      && MOST_POSITIVE_FIXNUM < nargs
    { memory_full_up (); }

  /* ARGS will contain the array of arguments to pass to the function.
     VISARGS will contain the same list but in a nicer form, so that if we
     pass it to Fformat_message it will be understandable to a human.
     Allocate them all at one go.  This wastes a bit of memory, but
     it's OK to trade space for speed.  */
  /* C declares `Lisp_Object *args;` and lets SAFE_NALLOCA fill it; the port's macro reads the
     pointee's size from the binding, so it starts null (the SAFE_ALLOCA shape).  */
  let mut args: *mut Lisp_Object = std::ptr::null_mut ();
  SAFE_NALLOCA! (sa_avail, args, 3, nargs);
  let visargs: *mut Lisp_Object = args.offset (nargs as isize);
  /* If varies[I] > 0, the Ith argument shouldn't just have its value
     in this call quoted in the command history.  It should be
     recorded as a call to the function named callint_argfuns[varies[I]].  */
  let varies: *mut i8 = visargs.offset (nargs as isize) as *mut i8;

  memclear (args as *mut c_void, (nargs * (2 * word_size + 1)) as isize);

  let speccount: specpdl_ref = SPECPDL_INDEX ();

  if !NILP (enable)
    { specbind (Qenable_recursive_minibuffers (), Qt ()); }

  let mut tem: *const c_char = string;
  let mut i: ptrdiff_t = 2;
  while tem < string_end
    {
      let pnl: *const c_char = memchr ((tem as *const c_void).offset (1), b'\n' as c_int,
				      (string_len - ((tem as ptrdiff_t + 1)
						     - string as ptrdiff_t)) as usize)
	as *const c_char;
      let sz: ptrdiff_t = if !pnl.is_null ()
			    { (pnl as ptrdiff_t) - (tem as ptrdiff_t + 1) }
			  else
			    { (string_end as ptrdiff_t) - (tem as ptrdiff_t + 1) };

      *visargs.offset (1) = make_string (tem.offset (1), sz);
      callint_message = Fformat_message (i - 1, visargs.offset (1));

      /* C 459-751's `switch (*tem)`.  Two arms share bodies with another arm: `case 'N'`
	 jumps to `have_prefix_arg:` (inside `case 'p'`) when there is a prefix arg and
	 otherwise falls through into `case 'n'`, and `case 'R'` falls through into `case 'r'`
	 unless the region is inactive.  A `match` neither falls through nor can be jumped
	 into, so those two entries keep one copy of each shared body and select the entry the
	 way tparam.c's chain does (PORTING.md 2.2); the notes file carries the `goto` and
	 `FALLTHROUGH` tokens they cannot spell.  */
      match *tem as c_int
	{
	letter_a =>
	  {
	    /* Symbol defined as a function.  */
	    *visargs.offset (i) = Fcompleting_read (callint_message,
						    Vobarray, Qfboundp (), Qt (),
						    Qnil (), Qnil (), Qnil (), Qnil ());
	    *args.offset (i) = Fintern (*visargs.offset (i), Qnil ());
	  }

	letter_b | letter_B =>
	  {
	    if *tem as c_int == letter_b
	      {
		/* Name of existing buffer.  */
		*args.offset (i) = Fcurrent_buffer ();
		if BASE_EQ (selected_window, minibuf_window)
		  { *args.offset (i) = Fother_buffer (*args.offset (i), Qnil (), Qnil ()); }
		*args.offset (i) = Fread_buffer (callint_message, *args.offset (i),
						 Qt (), Qnil ());
	      }
	    else
	      {
		/* Name of buffer, possibly nonexistent.  */
		*args.offset (i) = Fread_buffer (callint_message,
						 Fother_buffer (Fcurrent_buffer (),
								Qnil (), Qnil ()),
						 Qnil (), Qnil ());
	      }
	  }

	letter_c =>
	  {
	    /* Character.  */
	    /* Prompt in `minibuffer-prompt' face.  */
	    Fput_text_property (make_fixnum (0),
				make_fixnum (SCHARS (callint_message) as EMACS_INT),
				Qface (), Qminibuffer_prompt (), callint_message);
	    *args.offset (i) = Fread_char (callint_message, Qnil (), Qnil ());
	    message1_nolog (0 as *const c_char);
	    /* See bug#8479.  */
	    if !CHARACTERP (*args.offset (i))
	      { error (c"Non-character input-event".as_ptr ()); }
	    *visargs.offset (i) = Fchar_to_string (*args.offset (i));
	  }

	letter_C =>
	  {
	    /* Command: symbol with interactive function.  */
	    *visargs.offset (i) = Fcompleting_read (callint_message,
						    Vobarray, Qcommandp (),
						    Qt (), Qnil (), Qnil (), Qnil (), Qnil ());
	    *args.offset (i) = Fintern (*visargs.offset (i), Qnil ());
	  }

	letter_d =>
	  {
	    /* Value of point.  Does not do I/O.  */
	    set_marker_both (point_marker, Qnil (), PT! (), PT_BYTE! ());
	    *args.offset (i) = point_marker;
	    /* visargs[i] = Qnil; */
	    *varies.offset (i) = 1;
	  }

	letter_D =>
	  {
	    /* Directory name.  */
	    *args.offset (i) = read_file_name (BVAR! (current_buffer, directory_), Qlambda (),
					       Qnil (), Qfile_directory_p ());
	  }

	letter_f =>
	  {
	    /* Existing file name.  */
	    *args.offset (i) = read_file_name (Qnil (), Qlambda (), Qnil (), Qnil ());
	  }

	letter_F =>
	  {
	    /* Possibly nonexistent file name.  */
	    *args.offset (i) = read_file_name (Qnil (), Qnil (), Qnil (), Qnil ());
	  }

	letter_G =>
	  {
	    /* Possibly nonexistent file name,
	       default to directory alone.  */
	    *args.offset (i) = read_file_name (Qnil (), Qnil (), empty_unibyte_string, Qnil ());
	  }

	letter_i =>
	  {
	    /* Ignore an argument -- Does not do I/O.  */
	    *varies.offset (i) = -1;
	  }

	letter_k | letter_K =>
	  {
	    /* Key sequence (k) or key sequence to be defined (K).  */
	    let speccount1: specpdl_ref = SPECPDL_INDEX ();
	    specbind (Qcursor_in_echo_area (), Qt ());
	    /* Prompt in `minibuffer-prompt' face.  */
	    Fput_text_property (make_fixnum (0),
				make_fixnum (SCHARS (callint_message) as EMACS_INT),
				Qface (), Qminibuffer_prompt (), callint_message);
	    if *tem as c_int == letter_k
	      {
		*args.offset (i) = Fread_key_sequence (callint_message,
						       Qnil (), Qnil (), Qnil (), Qnil (),
						       Qnil ());
		unbind_to (speccount1, Qnil ());
		*visargs.offset (i) = Fkey_description (*args.offset (i), Qnil ());

		/* If the key sequence ends with a down-event,
		   discard the following up-event.  */
		let mut teml: Lisp_Object
		  = Faref (*args.offset (i),
			   make_fixnum (XFIXNUM (Flength (*args.offset (i))) - 1));
		if CONSP (teml)
		  { teml = XCAR (teml); }
		if SYMBOLP (teml)
		  {
		    teml = Fget (teml, Qevent_symbol_elements ());
		    /* Ignore first element, which is the base key.  */
		    let tem2: Lisp_Object = Fmemq (Qdown (), Fcdr (teml));
		    if !NILP (tem2)
		      { up_event = Fread_event (Qnil (), Qnil (), Qnil ()); }
		  }
	      }
	    else
	      {
		*args.offset (i) = Fread_key_sequence_vector (callint_message,
							      Qnil (), Qt (), Qnil (), Qnil (),
							      Qnil ());
		*visargs.offset (i) = Fkey_description (*args.offset (i), Qnil ());
		unbind_to (speccount1, Qnil ());

		/* If the key sequence ends with a down-event,
		   discard the following up-event.  */
		let mut teml: Lisp_Object
		  = Faref (*args.offset (i), make_fixnum (ASIZE (*args.offset (i)) - 1));
		if CONSP (teml)
		  { teml = XCAR (teml); }
		if SYMBOLP (teml)
		  {
		    teml = Fget (teml, Qevent_symbol_elements ());
		    /* Ignore first element, which is the base key.  */
		    let tem2: Lisp_Object = Fmemq (Qdown (), Fcdr (teml));
		    if !NILP (tem2)
		      { up_event = Fread_event (Qnil (), Qnil (), Qnil ()); }
		  }
	      }
	  }

	letter_U =>
	  {
	    /* Up event from last k or K.  */
	    if !NILP (up_event)
	      {
		*args.offset (i) = make_vector (1, up_event);
		up_event = Qnil ();
		*visargs.offset (i) = Fkey_description (*args.offset (i), Qnil ());
	      }
	  }

	letter_e =>
	  {
	    /* The invoking event.  */
	    if next_event >= key_count
	      { error (c"%s must be bound to an event with parameters".as_ptr (),
		       if SYMBOLP (function) { SSDATA (SYMBOL_NAME (function)) }
		       else { c"command".as_ptr () }); }
	    *args.offset (i) = AREF (keys, next_event);
	    *varies.offset (i) = -1;

	    /* `inhibit_mouse_event_check' allows non-parameterized events.  */
	    if inhibit_mouse_event_check
	      { next_event += 1; }
	    else
	      {
		/* Find the next parameterized event.  */
		loop
		  {
		    next_event += 1;
		    if !(next_event < key_count
			 && !EVENT_HAS_PARAMETERS (AREF (keys, next_event)))
		      { break; }
		  }
	      }
	  }

	letter_m =>
	  {
	    /* Value of mark.  Does not do I/O.  */
	    check_mark (false);
	    /* visargs[i] = Qnil; */
	    *args.offset (i) = BVAR! (current_buffer, mark_);
	    *varies.offset (i) = 2;
	  }

	letter_M =>
	  {
	    /* String read via minibuffer with
	       inheriting the current input method.  */
	    *args.offset (i) = Fread_string (callint_message,
					     Qnil (), Qnil (), Qnil (), Qt ());
	  }

	letter_N | letter_n | letter_p =>
	  {
	    /* C 636-656: `case 'N'` jumps to the `have_prefix_arg:` label inside
	       `case 'p'` when there is a prefix arg, and otherwise falls through into
	       `case 'n'`; `case 'n'` and `case 'p'` enter those two bodies directly.
	       One copy of each body, with the entry selected here.  */
	    let mut take_prefix_arg: bool = *tem as c_int == letter_p;
	    if *tem as c_int == letter_N && !NILP (prefix_arg)
	      { take_prefix_arg = true; }
	    if take_prefix_arg
	      {
		/* have_prefix_arg: */
		*args.offset (i) = Fprefix_numeric_value (prefix_arg);
		/* visargs[i] = Qnil; */
		*varies.offset (i) = -1;
	      }
	    else
	      {
		*args.offset (i) = calln! (Qread_number (), callint_message);
		*visargs.offset (i) = Fnumber_to_string (*args.offset (i));
	      }
	  }

	letter_P =>
	  {
	    /* Prefix arg in raw form.  Does no I/O.  */
	    *args.offset (i) = prefix_arg;
	    /* visargs[i] = Qnil; */
	    *varies.offset (i) = -1;
	  }

	letter_R | letter_r =>
	  {
	    /* C 658-679: `case 'R'` breaks out when the region is inactive, and falls
	       through into `case 'r'` otherwise; the shared body is written once.  */
	    let mut do_region: bool = *tem as c_int == letter_r;
	    if *tem as c_int == letter_R
	      {
		if NILP (calln! (Quse_region_p ()))
		  {
		    /* args[i] = args[i + 1] = visargs[i] = visargs[i + 1] = Qnil;  */
		    *varies.offset (i) = 5;
		    i += 1;
		    *varies.offset (i) = 6;
		  }
		else
		  { do_region = true; }
	      }
	    if do_region
	      {
		check_mark (true);
		set_marker_both (point_marker, Qnil (), PT! (), PT_BYTE! ());
		let mark: ptrdiff_t = marker_position (BVAR! (current_buffer, mark_));
		/* visargs[i] = visargs[i + 1] = Qnil; */
		*args.offset (i) = if PT! () < mark { point_marker }
				  else { BVAR! (current_buffer, mark_) };
		*varies.offset (i) = if *tem as c_int == letter_R { 5 } else { 3 };
		i += 1;
		*args.offset (i) = if PT! () > mark { point_marker }
				  else { BVAR! (current_buffer, mark_) };
		*varies.offset (i) = if *tem as c_int == letter_R { 6 } else { 4 };
	      }
	  }

	letter_s =>
	  {
	    /* String read via minibuffer without
	       inheriting the current input method.  */
	    *args.offset (i) = Fread_string (callint_message,
					     Qnil (), Qnil (), Qnil (), Qnil ());
	  }

	letter_S =>
	  {
	    /* Any symbol.  */
	    *visargs.offset (i) = Fread_string (callint_message,
						Qnil (), Qnil (), Qnil (), Qnil ());
	    *args.offset (i) = Fintern (*visargs.offset (i), Qnil ());
	  }

	letter_v =>
	  {
	    /* Variable name: symbol that is
	       custom-variable-p.  */
	    *args.offset (i) = Fread_variable (callint_message, Qnil ());
	    *visargs.offset (i) = last_minibuf_string;
	  }

	letter_x =>
	  {
	    /* Lisp expression read but not evaluated.  */
	    *args.offset (i) = calln! (Qread_minibuffer (), callint_message);
	    *visargs.offset (i) = last_minibuf_string;
	  }

	letter_X =>
	  {
	    /* Lisp expression read and evaluated.  */
	    *args.offset (i) = calln! (Qeval_minibuffer (), callint_message);
	    *visargs.offset (i) = last_minibuf_string;
	  }

	letter_Z =>	    /* Coding-system symbol, or ignore the
			       argument if no prefix.  */
	  {
	    if NILP (prefix_arg)
	      {
		/* args[i] = Qnil; */
		*varies.offset (i) = -1;
	      }
	    else
	      {
		*args.offset (i) = Fread_non_nil_coding_system (callint_message);
		*visargs.offset (i) = last_minibuf_string;
	      }
	  }

	letter_z =>	    /* Coding-system symbol or nil.  */
	  {
	    *args.offset (i) = Fread_coding_system (callint_message, Qnil ());
	    *visargs.offset (i) = last_minibuf_string;
	  }

	letter_plus | _ =>
	  {
	    /* C 731-732's `case '+': default:` - one body for both, as C writes it.  */
	    invalid_control_letter (tem, string, string_len);
	  }
	}

      if *varies.offset (i) == 0
	{ arg_from_tty = true; }

      if NILP (*visargs.offset (i)) && STRINGP (*args.offset (i))
	{ *visargs.offset (i) = *args.offset (i); }

      tem = memchr (tem as *const c_void, b'\n' as c_int,
		    (string_len - (tem as ptrdiff_t - string as ptrdiff_t)) as usize)
	as *const c_char;
      if !tem.is_null ()
	{ tem = tem.offset (1); }
      else
	{ tem = string_end; }
      i += 1;
    }
  unbind_to (speccount, Qnil ());

  maybe_quit ();

  *args.offset (0) = Qfuncall_interactively ();
  *args.offset (1) = function;

  if arg_from_tty || !NILP (record_flag)
    {
      /* We don't need `visargs' any more, so let's recycle it since we need
	 an array of just the same size.  */
      *visargs.offset (1) = function;
      let mut i: ptrdiff_t = 2;
      while i < nargs
	{
	  *visargs.offset (i) = if *varies.offset (i) > 0
				  { list1 (intern (*(callint_argfuns
						     .as_ptr ()
						     .offset (*varies.offset (i) as isize)))) }
				else
				  { quotify_arg (*args.offset (i)) };
	  i += 1;
	}
      calln! (Qadd_to_history (), Qcommand_history (),
	      Flist (nargs - 1, visargs.offset (1)), Qnil (), Qt ());
    }

  /* If we used a marker to hold point, mark, or an end of the region,
     temporarily, convert it to an integer now.  */
  let mut i: ptrdiff_t = 2;
  while i < nargs
    {
      if *varies.offset (i) >= 1 && *varies.offset (i) <= 4
	{ XSETINT! (*args.offset (i), marker_position (*args.offset (i))); }
      i += 1;
    }

  if record_then_fail
    { Fbarf_if_buffer_read_only (Qnil ()); }

  Vthis_command = save_this_command;
  Vthis_original_command = save_this_original_command;
  Vreal_this_command = save_real_this_command;
  kset_last_command (current_kboard, save_last_command);

  specbind (Qcommand_debug_status (), Qnil ());

  let val: Lisp_Object = Ffuncall (nargs, args);
  unbind_to (speccount, Qnil ());
  SAFE_FREE! (sa_count);
  return val;
});

DEFUN! ("prefix-numeric-value", Fprefix_numeric_value, Sprefix_numeric_value,
       1, 1, 0,
       doc: "Return numeric meaning of raw prefix argument RAW.\nA raw prefix argument is what you get from `(interactive \"P\")'.\nIts numeric meaning is what you would get from `(interactive \"p\")'.  ",
       (raw: Lisp_Object),
       {
  let mut val: Lisp_Object;

  if NILP (raw)
    { XSETFASTINT! (val, 1); }
  else if EQ (raw, Qminus ())
    { XSETINT! (val, -1); }
  else if CONSP (raw) && FIXNUMP (XCAR (raw))
    { val = XCAR (raw); }
  else if FIXNUMP (raw)
    { val = raw; }
  else
    { XSETFASTINT! (val, 1); }

  return val;
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_callint ()
{
  point_marker = Fmake_marker ();
  staticpro (&raw const point_marker);

  callint_message = Qnil ();
  staticpro (&raw const callint_message);

  preserved_fns = list! (intern_c_string (c"use-region-beginning".as_ptr ()),
			 intern_c_string (c"use-region-end".as_ptr ()),
			 intern_c_string (c"region-beginning".as_ptr ()),
			 intern_c_string (c"region-end".as_ptr ()),
			 intern_c_string (c"point".as_ptr ()),
			 intern_c_string (c"mark".as_ptr ()));
  staticpro (&raw const preserved_fns);

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

  DEFVAR_KBOARD! ("prefix-arg", Vprefix_arg,
		  "The value of the prefix argument for the next editing command.\nIt may be a number, or the symbol `-' for just a minus sign as arg,\nor a list whose car is a number for just one or more C-u's\nor nil if no argument has been specified.\n\nYou cannot examine this variable to find the argument for this command\nsince it has been set to nil by the time you can look.\nInstead, you should use the variable `current-prefix-arg', although\nnormally commands can get this prefix argument with (interactive \"P\").  ");

  DEFVAR_KBOARD! ("last-prefix-arg", Vlast_prefix_arg,
		  "The value of the prefix argument for the previous editing command.\nSee `prefix-arg' for the meaning of the value.  ");

  DEFVAR_LISP! ("current-prefix-arg", Vcurrent_prefix_arg,
		"The value of the prefix argument for this editing command.\nIt may be a number, or the symbol `-' for just a minus sign as arg,\nor a list whose car is a number for just one or more C-u's\nor nil if no argument has been specified.\nThis is what `(interactive \"P\")' returns.  ");
  Vcurrent_prefix_arg = Qnil ();

  DEFVAR_LISP! ("command-history", Vcommand_history,
		"List of recent commands that read arguments from terminal.\nEach command is represented as a form to evaluate.\n\nMaximum length of the history list is determined by the value\nof `history-length', which see.  ");
  Vcommand_history = Qnil ();

  DEFVAR_LISP! ("command-debug-status", Vcommand_debug_status,
		"Debugging status of current interactive command.\nBound each time `call-interactively' is called;\nmay be set by the debugger as a reminder for itself.  ");
  Vcommand_debug_status = Qnil ();

  DEFVAR_LISP! ("mark-even-if-inactive", Vmark_even_if_inactive,
		"Non-nil means you can use the mark even when inactive.\nThis option makes a difference in Transient Mark mode.\nWhen the option is non-nil, deactivation of the mark\nturns off region highlighting, but commands that use the mark\nbehave as if the mark were still active.  ");
  Vmark_even_if_inactive = Qt ();

  DEFVAR_LISP! ("mouse-leave-buffer-hook", Vmouse_leave_buffer_hook,
		"Hook run when the user mouse-clicks in a window.\nIt can be run both before and after switching windows, or even when\nnot actually switching windows.\n\nIts purpose is to give temporary modes such as Isearch mode\na way to turn themselves off when a mouse command switches windows.  ");
  Vmouse_leave_buffer_hook = Qnil ();

  DEFVAR_BOOL! ("inhibit-mouse-event-check", inhibit_mouse_event_check,
    "Whether the interactive spec \"e\" requires a mouse gesture event.\nIf non-nil, `(interactive \"e\")' doesn't signal an error when the command\nwas invoked by an input event that is not a mouse gesture: a click, a drag,\netc.  To create the event data when the input was some other event,\nuse `event-start', `event-end', and `event-click-count'.  ");
  inhibit_mouse_event_check = false;

  defsubr (&raw mut Sinteractive);
  defsubr (&raw mut Scall_interactively);
  defsubr (&raw mut Sfuncall_interactively);
  defsubr (&raw mut Sprefix_numeric_value);

  DEFSYM! (Qinteractive_args, "interactive-args");
  DEFSYM! (Qread_file_name, "read-file-name");
  DEFSYM! (Qcommand_history, "command-history");
  DEFSYM! (Qeval_minibuffer, "eval-minibuffer");
  DEFSYM! (Quse_region_p, "use-region-p");
}
