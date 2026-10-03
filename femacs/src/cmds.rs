// femacs: port of src/cmds.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// subr, global, statement), same control flow.  The seven subrs use `DEFUN!` (PORTING.md
// 5.1); `move_point` and `internal_self_insert` stay private `unsafe fn`s, because C keeps
// them `static` (0.1-3).
//
// The `#include` list mirrors cmds.c 21-30 — config.h, lisp.h, commands.h, character.h,
// buffer.h, syntax.h, keyboard.h, keymap.h, frame.h — with the mirrors those pull in
// (dispextern.h carries `bitch_at_user`, string.h `memcpy`).
//
// Deviations this file carries, all recorded in docs/ports/cmds.md:
//   * C's forward declaration of `internal_self_insert` (C 32) has no counterpart (0.1-12).
//   * C's `pgrow`-style uninitialised locals (`ptrdiff_t pos, pos_byte;` filled by
//     `scan_newline_from_point`'s out-parameters) are zero-initialised first, then assigned
//     exactly where C assigns them (0.1-13).
//   * C's `%"pI"d` (lisp.h's `pI` is `"t"`) is written `%td`, as region-cache.rs does.
//   * `Vpost_self_insert_hook` is C's `globals.f_Vpost_self_insert_hook`; the port defines it
//     as the unit's own object, the shape casetab.rs/character.rs already use for the `V*`
//     their C files own (status §3.29).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]


include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("../lib/include/string.rs");
include!("include/lisp.rs");
include!("include/character.rs");
include!("include/itree.rs");
include!("include/buffer.rs");
include!("include/commands.rs");
include!("include/syntax.rs");
include!("include/systime.rs");
include!("include/dispextern.rs");
include!("include/coding.rs");
include!("include/composite.rs");
include!("include/keyboard.rs");
include!("include/termhooks.rs");
include!("include/window.rs");
include!("include/frame.rs");
include!("include/keymap.rs");

/* Add N to point; or subtract N if FORWARD is false.  N defaults to 1.
   Validate the new location.  Return nil.  */
unsafe fn move_point (mut n: Lisp_Object, forward: bool) -> Lisp_Object
{
  /* This used to just set point to point + XFIXNUM (n), and then check
     to see if it was within boundaries.  But now that SET_PT can
     potentially do a lot of stuff (calling entering and exiting
     hooks, etcetera), that's not a good approach.  So we validate the
     proposed position, then set point.  */

  /* C declares `EMACS_INT new_point;` and assigns it once, below (0.1-13).  */
  let new_point: EMACS_INT;

  if NILP (n)
    { XSETFASTINT! (n, 1); }
  else
    { CHECK_FIXNUM (n); }

  new_point = PT! () + (if forward { XFIXNUM (n) } else { - XFIXNUM (n) });

  if new_point < BEGV! ()
    {
      SET_PT (BEGV! ());
      xsignal0 (Qbeginning_of_buffer ());
    }
  if new_point > ZV! ()
    {
      SET_PT (ZV! ());
      xsignal0 (Qend_of_buffer ());
    }

  SET_PT (new_point);
  return Qnil ();
}

DEFUN! ("forward-char", Fforward_char, Sforward_char, 0, 1, "^p",
       doc: "Move point N characters forward (backward if N is negative).\nOn reaching end or beginning of buffer, stop and signal error.\nInteractively, N is the numeric prefix argument.\nIf N is omitted or nil, move point 1 character forward.\n\nDepending on the bidirectional context, the movement may be to the\nright or to the left on the screen.  This is in contrast with\n\\[right-char], which see.",
       (n: Lisp_Object),
       {
  return move_point (n, true);
});

DEFUN! ("backward-char", Fbackward_char, Sbackward_char, 0, 1, "^p",
       doc: "Move point N characters backward (forward if N is negative).\nOn attempt to pass beginning or end of buffer, stop and signal error.\nInteractively, N is the numeric prefix argument.\nIf N is omitted or nil, move point 1 character backward.\n\nDepending on the bidirectional context, the movement may be to the\nright or to the left on the screen.  This is in contrast with\n\\[left-char], which see.",
       (n: Lisp_Object),
       {
  return move_point (n, false);
});

DEFUN! ("forward-line", Fforward_line, Sforward_line, 0, 1, "^p",
       doc: "Move N lines forward (backward if N is negative).\nPrecisely, if point is on line I, move to the start of line I + N\n(\"start of line\" in the logical order).\nIf there isn't room, go as far as possible (no error).\nInteractively, N is the numeric prefix argument and defaults to 1.\n\nReturns the count of lines left to move.  If moving forward,\nthat is N minus number of lines moved; if backward, N plus number\nmoved.\n\nException: With positive N, a non-empty line at the end of the\nbuffer, or of its accessible portion, counts as one line\nsuccessfully moved (for the return value).  This means that the\nfunction will move point to the end of such a line and will count\nit as a line moved across, even though there is no next line to\ngo to its beginning.",
       (n: Lisp_Object),
       {
  /* C declares `ptrdiff_t opoint = PT, pos, pos_byte, count;` and lets
     scan_newline_from_point fill the middle two; the port zeroes them first (0.1-13).  */
  let opoint: ptrdiff_t = PT! ();
  let mut pos: ptrdiff_t = 0;
  let mut pos_byte: ptrdiff_t = 0;
  let mut count: ptrdiff_t;
  let mut excessive: bool = false;

  if NILP (n)
    { count = 1; }
  else
    {
      CHECK_INTEGER (n);
      if FIXNUMP (n)
	  && -BUF_BYTES_MAX <= XFIXNUM (n) && XFIXNUM (n) <= BUF_BYTES_MAX
	{ count = XFIXNUM (n); }
      else
	{
	  count = if !NILP (Fnatnump (n)) { BUF_BYTES_MAX } else { -BUF_BYTES_MAX };
	  excessive = true;
	}
    }

  let counted: ptrdiff_t = scan_newline_from_point (count, &raw mut pos, &raw mut pos_byte);

  SET_PT_BOTH (pos, pos_byte);

  let mut shortage: ptrdiff_t = count - (count <= 0) as ptrdiff_t - counted;
  if shortage != 0
    {
      shortage -= if count <= 0
	{ -1 }
      else
	{ (BEGV! () < ZV! () && PT! () != opoint
	   && FETCH_BYTE! (PT_BYTE! () - 1) != b'\n') as ptrdiff_t };
    }
  return if excessive
    { CALLN! (Fplus, make_fixnum (shortage - count), n) }
  else
    { make_fixnum (shortage) };
});

DEFUN! ("beginning-of-line", Fbeginning_of_line, Sbeginning_of_line, 0, 1, "^p",
       doc: "Move point to beginning of current line (in the logical order).\nWith argument N not nil or 1, move forward N - 1 lines first.\nIf point reaches the beginning or end of buffer, it stops there.\n\nThis function constrains point to the current field unless this moves\npoint to a different line from the original, unconstrained result.\nIf N is nil or 1, and a front-sticky field starts at point, the point\ndoes not move.  To ignore field boundaries bind\n`inhibit-field-text-motion' to t, or use the `forward-line' function\ninstead.  For instance, `(forward-line 0)' does the same thing as\n`(beginning-of-line)', except that it ignores field boundaries.",
       (n: Lisp_Object),
       {
  /* C assigns to its parameter N (C 199-200's `XSETFASTINT (n, 1)`); the port shadows it.  */
  let mut n: Lisp_Object = n;
  if NILP (n)
    { XSETFASTINT! (n, 1); }
  else
    { CHECK_FIXNUM (n); }

  SET_PT (XFIXNUM (Fline_beginning_position (n)));

  return Qnil ();
});

DEFUN! ("end-of-line", Fend_of_line, Send_of_line, 0, 1, "^p",
       doc: "Move point to end of current line (in the logical order).\nWith argument N not nil or 1, move forward N - 1 lines first.\nIf point reaches the beginning or end of buffer, it stops there.\nTo ignore intangibility, bind `inhibit-point-motion-hooks' to t.\n\nThis function constrains point to the current field unless this moves\npoint to a different line from the original, unconstrained result.  If\nN is nil or 1, and a rear-sticky field ends at point, the point does\nnot move.  To ignore field boundaries bind `inhibit-field-text-motion'\nto t.",
       (n: Lisp_Object),
       {
  /* C assigns to its parameter N (C 262's `n = make_fixnum (1);`); the port shadows it.  */
  let mut n: Lisp_Object = n;
  let mut newpos: ptrdiff_t;

  if NILP (n)
    { XSETFASTINT! (n, 1); }
  else
    { CHECK_FIXNUM (n); }

  loop
    {
      newpos = XFIXNUM (Fline_end_position (n));
      SET_PT (newpos);

      if PT! () > newpos
	  && FETCH_BYTE! (PT_BYTE! () - 1) == b'\n'
	{
	  /* If we skipped over a newline that follows
	     an invisible intangible run,
	     move back to the last tangible position
	     within the line.  */

	  SET_PT (PT! () - 1);
	  break;
	}
      else if PT! () > newpos && PT! () < ZV! ()
	       && FETCH_BYTE! (PT_BYTE! ()) != b'\n'
	/* If we skipped something intangible
	   and now we're not really at eol,
	   keep going.  */
	{ n = make_fixnum (1); }
      else
	{ break; }
    }

  return Qnil ();
});

DEFUN! ("delete-char", Fdelete_char, Sdelete_char, 1, 2, "p\nP",
       doc: "Delete the following N characters (previous if N is negative).\nOptional second arg KILLFLAG non-nil means kill instead (save in kill ring).\nInteractively, N is the prefix arg, and KILLFLAG is set if\nN was explicitly specified.\n\nThe command `delete-forward-char' is preferable for interactive use, e.g.\nbecause it respects values of `delete-active-region' and `overwrite-mode'.",
       (n: Lisp_Object, killflag: Lisp_Object),
       {
  let pos: EMACS_INT;

  CHECK_FIXNUM (n);

  if eabs! (XFIXNUM (n)) < 2
    { call0 (Qundo_auto_amalgamate ()); }

  pos = PT! () + XFIXNUM (n);
  if NILP (killflag)
    {
      if XFIXNUM (n) < 0
	{
	  if pos < BEGV! ()
	    { xsignal0 (Qbeginning_of_buffer ()); }
	  else
	    { del_range (pos, PT! ()); }
	}
      else
	{
	  if pos > ZV! ()
	    { xsignal0 (Qend_of_buffer ()); }
	  else
	    { del_range (PT! (), pos); }
	}
    }
  else
    {
      calln! (Qkill_forward_chars (), n);
    }
  return Qnil ();
});

DEFUN! ("self-insert-command", Fself_insert_command, Sself_insert_command, 1, 2,
       "(list (prefix-numeric-value current-prefix-arg) last-command-event)",
       doc: "Insert the character you type.\nWhichever character C you type to run this command is inserted.\nThe numeric prefix argument N says how many times to repeat the insertion.\nBefore insertion, `expand-abbrev' is executed if the inserted character does\nnot have word syntax and the previous character in the buffer does.\nAfter insertion, `internal-auto-fill' is called if\n`auto-fill-function' is non-nil and if the `auto-fill-chars' table has\na non-nil value for the inserted character.  At the end, it runs\n`post-self-insert-hook'.",
       (n: Lisp_Object, c: Lisp_Object),
       {
  /* C assigns to its parameter C (C 297-299); the port shadows it.  */
  let mut c: Lisp_Object = c;
  CHECK_FIXNUM (n);

  /* Backward compatibility.  */
  if NILP (c)
    { c = last_command_event; }
  else
    { last_command_event = c; }

  if XFIXNUM (n) < 0
    { error (c"Negative repetition argument %td".as_ptr (), XFIXNUM (n)); }

  if XFIXNAT (n) < 2
    { call0 (Qundo_auto_amalgamate ()); }

  /* Barf if the key that invoked this was not a character.  */
  if !CHARACTERP (c)
    { bitch_at_user (); }
  else
    {
      let character: c_int = translate_char (Vtranslation_table_for_input,
					     XFIXNUM (c) as c_int);
      let val: c_int = internal_self_insert (character, XFIXNAT (n));
      if val == 2
	{ Fset (Qundo_auto__this_command_amalgamating (), Qnil ()); }
      frame_make_pointer_invisible (SELECTED_FRAME ());
    }

  return Qnil ();
});

/* Insert N times character C

   If this insertion is suitable for direct output (completely simple),
   return 0.  A value of 1 indicates this *might* not have been simple.
   A value of 2 means this did things that call for an undo boundary.  */

unsafe fn internal_self_insert (mut c: c_int, mut n: EMACS_INT) -> c_int
{
  let mut hairy: c_int = 0;
  let mut tem: Lisp_Object;
  let mut synt: syntaxcode;
  let mut overwrite: Lisp_Object;
  /* Length of multi-byte form of C.  */
  let mut len: c_int;
  /* Working buffer and pointer for multi-byte form of C.  */
  /* C: `unsigned char str[MAX_MULTIBYTE_LENGTH];` - the port names it `str_`, the spelling
     the mirrors use for C's `str` (PORTING.md 5.1).  */
  let mut str_: [u8; MAX_MULTIBYTE_LENGTH as usize] = [0; MAX_MULTIBYTE_LENGTH as usize];
  let mut chars_to_delete: ptrdiff_t = 0;
  let mut spaces_to_insert: ptrdiff_t = 0;

  overwrite = BVAR! (current_buffer, overwrite_mode_);
  if !NILP (Vbefore_change_functions) || !NILP (Vafter_change_functions)
    { hairy = 1; }

  /* At first, get multi-byte form of C in STR.  */
  if !NILP (BVAR! (current_buffer, enable_multibyte_characters_))
    {
      len = CHAR_STRING (c, str_.as_mut_ptr ());
      if len == 1
	/* If C has modifier bits, this makes C an appropriate
           one-byte char.  */
	{ c = str_[0] as c_int; }
    }
  else
    {
      str_[0] = (if SINGLE_BYTE_CHAR_P (c as intmax_t) { c } else { CHAR_TO_BYTE8 (c) }) as u8;
      len = 1;
    }
  if !NILP (overwrite)
      && PT! () < ZV! ()
    {
      /* In overwrite-mode, we substitute a character at point (C2,
	 hereafter) by C.  For that, we delete C2 in advance.  But,
	 just substituting C2 by C may move a remaining text in the
	 line to the right or to the left, which is not preferable.
	 So we insert more spaces or delete more characters in the
	 following cases: if C is narrower than C2, after deleting C2,
	 we fill columns with spaces, if C is wider than C2, we delete
	 C2 and several characters following C2.  */

      /* This is the character after point.  */
      let c2: c_int = FETCH_CHAR (PT_BYTE! ());

      let mut cwidth: c_int = 0;

      /* Overwriting in binary-mode always replaces C2 by C.
	 Overwriting in textual-mode doesn't always do that.
	 It inserts newlines in the usual way,
	 and inserts any character at end of line
	 or before a tab if it doesn't use the whole width of the tab.  */
      if EQ (overwrite, Qoverwrite_mode_binary ())
	{ chars_to_delete = min (n, PTRDIFF_MAX); }
      else if c != b'\n' as c_int && c2 != b'\n' as c_int
	       && { cwidth = XFIXNAT (Fchar_width (make_fixnum (c as EMACS_INT))) as c_int; cwidth != 0 }
	{
	  let pos: ptrdiff_t = PT! ();
	  let pos_byte: ptrdiff_t = PT_BYTE! ();
	  let curcol: ptrdiff_t = current_column ();

	  if n <= (min (MOST_POSITIVE_FIXNUM, PTRDIFF_MAX) - curcol) / cwidth as ptrdiff_t
	    {
	      /* Column the cursor should be placed at after this insertion.
		 The value should be calculated only when necessary.  */
	      let target_clm: ptrdiff_t = curcol + n * cwidth as ptrdiff_t;

	      /* The actual cursor position after the trial of moving
		 to column TARGET_CLM.  It is greater than TARGET_CLM
		 if the TARGET_CLM is middle of multi-column
		 character.  In that case, the new point is set after
		 that character.  */
	      let actual_clm: ptrdiff_t
		= XFIXNAT (Fmove_to_column (make_fixnum (target_clm), Qnil ())) as ptrdiff_t;

	      chars_to_delete = PT! () - pos;

	      if actual_clm > target_clm
		{
		  /* We will delete too many columns.  Let's fill columns
		     by spaces so that the remaining text won't move.  */
		  let mut actual: ptrdiff_t = PT_BYTE! ();
		  actual -= prev_char_len (actual) as ptrdiff_t;
		  if FETCH_BYTE! (actual) == b'\t'
		    /* Rather than add spaces, let's just keep the tab. */
		    { chars_to_delete -= 1; }
		  else
		    { spaces_to_insert = actual_clm - target_clm; }
		}

	      SET_PT_BOTH (pos, pos_byte);
	    }
	}
      hairy = 2;
    }

  synt = SYNTAX (c);

  if !NILP (BVAR! (current_buffer, abbrev_mode_))
      && synt != Sword
      && NILP (BVAR! (current_buffer, read_only_))
      && PT! () > BEGV! ()
      && (SYNTAX (if !NILP (BVAR! (current_buffer, enable_multibyte_characters_))
		  { XFIXNAT (Fprevious_char ()) as c_int }
		  else
		  { UNIBYTE_TO_CHAR (XFIXNAT (Fprevious_char ()) as c_int) })
	  == Sword)
    {
      let modiff: modiff_count = MODIFF! ();
      let mut sym: Lisp_Object;

      sym = call0 (Qexpand_abbrev ());

      /* If we expanded an abbrev which has a hook,
	 and the hook has a non-nil `no-self-insert' property,
	 return right away--don't really self-insert.  */
      if SYMBOLP (sym) && ! NILP (sym)
	  && ! NILP ((*XSYMBOL (sym)).u.s.function)
	  && SYMBOLP ((*XSYMBOL (sym)).u.s.function)
	{
	  let prop: Lisp_Object;
	  prop = Fget ((*XSYMBOL (sym)).u.s.function, Qno_self_insert ());
	  if ! NILP (prop)
	    { return 1; }
	}

      if MODIFF! () != modiff
	{ hairy = 2; }
    }

  if chars_to_delete != 0
    {
      let mc: c_int = (if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
			 && SINGLE_BYTE_CHAR_P (c as intmax_t)
		       { UNIBYTE_TO_CHAR (c) } else { c });
      let mut string: Lisp_Object = Fmake_string (make_fixnum (n), make_fixnum (mc as EMACS_INT),
					 Qnil ());

      if spaces_to_insert != 0
	{
	  tem = Fmake_string (make_fixnum (spaces_to_insert),
			      make_fixnum (b' ' as EMACS_INT), Qnil ());
	  string = concat2 (string, tem);
	}

      let mut to: ptrdiff_t;
      /* C 469: `if (ckd_add (&to, PT, chars_to_delete)) to = PTRDIFF_MAX;` - the sum is
	 stored either way (the timespec-add.rs shape).  */
      {
	let (sum, overflowed) = PT! ().overflowing_add (chars_to_delete);
	to = sum;
	if overflowed
	  { to = PTRDIFF_MAX; }
      }
      replace_range (PT! (), to, string, true, true, false);
      Fforward_char (make_fixnum (n));
    }
  else if n > 1
    {
      USE_SAFE_ALLOCA! (sa_avail, sa_count);
      /* C declares `char *strn, *p;` and lets SAFE_NALLOCA fill `strn`; the port's macro
	 reads the pointee's size from the binding, so it starts null (the callint.rs shape).  */
      let mut strn: *mut c_char = std::ptr::null_mut ();
      SAFE_NALLOCA! (sa_avail, strn, len, n);
      let mut p: *mut c_char = strn;
      while n > 0
	{
	  memcpy (p as *mut c_void, str_.as_ptr () as *const c_void, len as usize);
	  n -= 1;
	  p = p.add (len as usize);
	}
      insert_and_inherit (strn, p.offset_from (strn));
      SAFE_FREE! (sa_count);
    }
  else if n > 0
    { insert_and_inherit (str_.as_ptr () as *const c_char, len as ptrdiff_t); }

  if (if CHAR_TABLE_P (Vauto_fill_chars)
      { !NILP (CHAR_TABLE_REF (Vauto_fill_chars, c as isize)) }
      else
      { c == b' ' as c_int || c == b'\n' as c_int })
      && !NILP (BVAR! (current_buffer, auto_fill_function_))
      && n > 0
    {
      let auto_fill_result: Lisp_Object;

      if c == b'\n' as c_int
	/* After inserting a newline, move to previous line and fill
	   that.  Must have the newline in place already so filling and
	   justification, if any, know where the end is going to be.  */
	{ SET_PT_BOTH (PT! () - 1, PT_BYTE! () - 1); }
      auto_fill_result = call0 (Qinternal_auto_fill ());
      /* Test PT < ZV in case the auto-fill-function is strange.  */
      if c == b'\n' as c_int && PT! () < ZV! () && FETCH_BYTE! (PT_BYTE! ()) == b'\n'
	{ SET_PT_BOTH (PT! () + 1, PT_BYTE! () + 1); }
      if !NILP (auto_fill_result)
	{ hairy = 2; }
    }

  /* Run hooks for electric keys.  */
  run_hook (Qpost_self_insert_hook ());

  return hairy;
}

/* module initialization */

// C: cmds.c 522's `Lisp_Object Vpost_self_insert_hook;` is `globals.f_Vpost_self_insert_hook`
// in C (lisp.h's DEFVAR_LISP 3495-3500 reads `&globals.f_##vname`), and the unit owns it, so
// the port defines the object here (status §3.29; the casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vpost_self_insert_hook: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_cmds ()
{
  DEFSYM! (Qinternal_auto_fill, "internal-auto-fill");

  DEFSYM! (Qundo_auto_amalgamate, "undo-auto-amalgamate");
  DEFSYM! (Qundo_auto__this_command_amalgamating,
	   "undo-auto--this-command-amalgamating");
  DEFSYM! (Qno_self_insert, "no-self-insert");

  DEFSYM! (Qkill_forward_chars, "kill-forward-chars");

  /* A possible value for a buffer's overwrite-mode variable.  */
  DEFSYM! (Qoverwrite_mode_binary, "overwrite-mode-binary");

  DEFSYM! (Qexpand_abbrev, "expand-abbrev");
  DEFSYM! (Qpost_self_insert_hook, "post-self-insert-hook");

  DEFVAR_LISP! (c"post-self-insert-hook", Vpost_self_insert_hook,
	       "Hook run at the end of `self-insert-command'.\nThis is run after inserting a character.\nThe hook can access the inserted character via `last-command-event'.");
  Vpost_self_insert_hook = Qnil ();

  defsubr (&raw mut Sforward_char);
  defsubr (&raw mut Sbackward_char);
  defsubr (&raw mut Sforward_line);
  defsubr (&raw mut Sbeginning_of_line);
  defsubr (&raw mut Send_of_line);

  defsubr (&raw mut Sdelete_char);
  defsubr (&raw mut Sself_insert_command);
}
