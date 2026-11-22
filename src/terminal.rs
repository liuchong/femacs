// femacs: port of src/terminal.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The nine subrs
// use `DEFUN!` (PORTING.md 5.1).  The two objects C gives external linkage (`terminal_list`,
// `initial_terminal`) carry `#[unsafe(no_mangle)]`; `next_terminal_id` is C's `static` and
// stays unexported (PORTING.md 0.2).
//
// The `#include` list mirrors terminal.c 19-27 - config.h, lisp.h, character.h, frame.h,
// termchar.h, blockinput.h, termhooks.h, keyboard.h - through the mirrors those pull in.
// lisp.h includes <string.h> (lisp.h 30), which is where `strcmp` comes from.
//
// terminal.c 29-33's `#if HAVE_STRUCT_UNIPAIR_UNICODE` block pulls in <errno.h>,
// <linux/kd.h> and <sys/ioctl.h>; the constant is undefined in the frozen configuration, so
// that block is dead text and stays behind `feature = "unipair_unicode"` (P-51).  No mirror
// carries those three headers, and `HAVE_STRUCT_UNIPAIR_UNICODE` has no entry in
// docs/baseline/config, so no `const _: () = core::assert!` binds the feature and `dead-arms`
// cannot flip it (the scroll.rs `glyph_debug` precedent); the arm is therefore text that is
// never name-resolved.
//
// `FRAME_TERMINAL` and `TERMINAL_ACTIVE_P` (termhooks.h 932/935) are the two termhooks.h
// macros this unit calls, and the mirror writes them as `macro_rules!`: they dereference
// `struct frame`/`struct tty_display_info`, which termhooks.h cannot name (the same reason
// its `FRAME_SCROLL_REGION_OK`/`_COST` are macros).
//
// The mirror spells a hook's `struct frame *`/`struct glyph *` arguments `*mut c_void`
// (P-72 rule ③), so each hook call casts once, and `if (h->hook) (*h->hook) (...)` is
// `if let Some (hook) = (…).hook { hook (…) }` - `.unwrap ()` is not allowed (hard
// constraint 4).
//
// Nine names this unit calls are declared in headers whose mirrors do not carry them yet, so
// each is declared at its use site (the doc.rs/scroll.rs shape): `delete_frame`,
// `decode_live_frame`, `free_frame_faces`, `tty_defined_color`, `root_xy`,
// `setup_coding_system`, `delete_kboard`, and the two names only the dead arm mentions
// (`encode_coding_utf_8`, `calculate_glyph_code_table`).
//
// C 44's `static void delete_initial_terminal (struct terminal *);` is a forward declaration
// with no Rust counterpart (PORTING.md 0.1-12): it is deleted here, the definition keeps its
// C place, and the deviation is recorded in docs/ports/terminal.md.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// C's `switch (t->type)` carries a `default: emacs_abort ()` arm; the port's `match` lists
// every enumerator *and* that arm, so the catch-all is unreachable by construction (P-78).
#![allow(unreachable_patterns)]
// [U1-MODULE-UNIT]
use crate::common::*;

// [U1] config.rs provided by common
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/stdcountof.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/frame.rs imported via common
// termchar.h 23 includes dispextern.h and is written for a unit that has already seen
// frame.h/termhooks.h (its FRAME_TTY reads them); naming it last is C's own order.
// [U1] include/termchar.rs imported via common
// [U1] include/blockinput.rs imported via common
// [U1] include/keyboard.rs imported via common

/* Chain of all terminals currently in use.  */
// C 36-42 gives these three objects file scope.  The mirror (include/termhooks.rs) declares
// `terminal_list` and `initial_terminal` at the crate root, so the definitions live in a
// module that is glob-imported: an item shadows a glob import while a second crate-root item
// of the same name would be `E0428` (PORTING.md P-21's mechanism, from the other side).

/* Chain of all terminals currently in use.  */
  #[unsafe(no_mangle)]
  pub static mut terminal_list: *mut terminal = std::ptr::null_mut ();

  /* The initial terminal device, created by initial_term_init.  */
  #[unsafe(no_mangle)]
  pub static mut initial_terminal: *mut terminal = std::ptr::null_mut ();

/* The first unallocated terminal id.  */
// C 39: `static int next_terminal_id;` — C's `static` is internal linkage, so the port
// keeps it a plain unexported `static mut` (PORTING.md 0.2); unlike the two objects
// above, nothing declares it in a mirror, so it needs no module.
static mut next_terminal_id: c_int = 0;

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status §3.29, the casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vring_bell_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vdelete_terminal_functions: Lisp_Object = lisp_h_Qnil ();

/* This setter is used only in this file, so it can be private.  */
// C: `static void tset_param_alist (struct terminal *t, Lisp_Object val)`
unsafe fn tset_param_alist (t: *mut terminal, val: Lisp_Object)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  (*t).param_alist = val;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ring_bell (f: *mut frame)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if ! NILP (Vring_bell_function)
    {
      let function: Lisp_Object;

      /* Temporarily set the global variable to nil
	 so that if we get an error, it stays nil
	 and we don't call it over and over.

	 We don't specbind it, because that would carefully
	 restore the bad value if there's an error
	 and make the loop of errors happen anyway.  */

      function = Vring_bell_function;
      Vring_bell_function = Qnil ();

      call0 (function);

      Vring_bell_function = function;
    }
  else if let Some (hook) = (*FRAME_TERMINAL! (f)).ring_bell_hook
    { hook (f as *mut c_void); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn update_begin (f: *mut frame)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if let Some (hook) = (*FRAME_TERMINAL! (f)).update_begin_hook
    { hook (f as *mut c_void); }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn update_end (f: *mut frame)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if let Some (hook) = (*FRAME_TERMINAL! (f)).update_end_hook
    { hook (f as *mut c_void); }

  }
}

/* Specify how many text lines, from the top of the window,
   should be affected by insert-lines and delete-lines operations.
   This, and those operations, are used only within an update
   that is bounded by calls to update_begin and update_end.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_terminal_window (f: *mut frame, size: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if let Some (hook) = (*FRAME_TERMINAL! (f)).set_terminal_window_hook
    { hook (f as *mut c_void, size); }

  }
}

/* Move cursor to row/column position VPOS/HPOS.  HPOS/VPOS are
   frame-relative coordinates.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn cursor_to (f: *mut frame, vpos: c_int, hpos: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let term: *mut terminal = FRAME_TERMINAL! (f);
  if let Some (hook) = (*term).cursor_to_hook
    {
      // PORTING.md 1.5: C declares `int x, y;` and lets root_xy write them.
      let mut x: c_int = 0;
      let mut y: c_int = 0;
      // C 117-121's `#ifndef HAVE_ANDROID` arm is the live one; the `#else` arm
      // (`x = hpos, y = vpos;`) is dead text on this target and stays behind the feature
      // that names the non-frozen side (P-51).  HAVE_ANDROID has no entry in
      // docs/baseline/config, so there is no assert to bind here.
      unsafe extern "C-unwind" {
	/// C: void root_xy (struct frame *f, int x, int y, int *rx, int *ry)  (dispnew.c)
	pub fn root_xy (f: *mut frame, x: c_int, y: c_int,
			rx: *mut c_int, ry: *mut c_int);
      }
      root_xy (f, hpos, vpos, &raw mut x, &raw mut y);
      #[cfg (feature = "have_android")]
      { x = hpos; y = vpos; }
      hook (f as *mut c_void, y, x);
    }

  }
}

/* Similar but don't take any account of the wasted characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn raw_cursor_to (f: *mut frame, row: c_int, col: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let term: *mut terminal = FRAME_TERMINAL! (f);
  if let Some (hook) = (*term).raw_cursor_to_hook
    {
      let mut x: c_int = 0;
      let mut y: c_int = 0;
      // C 135-139: the `#ifndef HAVE_ANDROID` arm is live, the `#else` arm is dead text
      // (see cursor_to).
      unsafe extern "C-unwind" {
	/// C: void root_xy (struct frame *f, int x, int y, int *rx, int *ry)  (dispnew.c)
	pub fn root_xy (f: *mut frame, x: c_int, y: c_int,
			rx: *mut c_int, ry: *mut c_int);
      }
      root_xy (f, col, row, &raw mut x, &raw mut y);
      #[cfg (feature = "have_android")]
      { x = col; y = row; }
      hook (f as *mut c_void, y, x);
    }

  }
}

/* Erase operations.  */

/* Clear from cursor to end of frame.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn clear_to_end (f: *mut frame)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if let Some (hook) = (*FRAME_TERMINAL! (f)).clear_to_end_hook
    { hook (f as *mut c_void); }

  }
}

/* Clear entire frame.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn clear_frame (f: *mut frame)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if let Some (hook) = (*FRAME_TERMINAL! (f)).clear_frame_hook
    { hook (f as *mut c_void); }

  }
}

/* Clear from cursor to end of line.
   Assume that the line is already clear starting at column first_unused_hpos.

   Note that the cursor may be moved, on terminals lacking a `ce' string.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn clear_end_of_line (f: *mut frame, first_unused_hpos: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if let Some (hook) = (*FRAME_TERMINAL! (f)).clear_end_of_line_hook
    { hook (f as *mut c_void, first_unused_hpos); }

  }
}

/* Output LEN glyphs starting at STRING at the nominal cursor position.
   Advance the nominal cursor over the text.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn write_glyphs (f: *mut frame, string: *mut glyph, len: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if let Some (hook) = (*FRAME_TERMINAL! (f)).write_glyphs_hook
    { hook (f as *mut c_void, string as *mut c_void, len); }

  }
}

/* Insert LEN glyphs from START at the nominal cursor position.

   If start is zero, insert blanks instead of a string at start */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn insert_glyphs (f: *mut frame, start: *mut glyph, len: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if len <= 0
    { return; }

  if let Some (hook) = (*FRAME_TERMINAL! (f)).insert_glyphs_hook
    { hook (f as *mut c_void, start as *mut c_void, len); }

  }
}

/* Delete N glyphs at the nominal cursor position. */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn delete_glyphs (f: *mut frame, n: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if let Some (hook) = (*FRAME_TERMINAL! (f)).delete_glyphs_hook
    { hook (f as *mut c_void, n); }

  }
}

/* Insert N lines at vpos VPOS.  If N is negative, delete -N lines.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ins_del_lines (f: *mut frame, vpos: c_int, n: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if let Some (hook) = (*FRAME_TERMINAL! (f)).ins_del_lines_hook
    { hook (f as *mut c_void, vpos, n); }

  }
}

/* Return the terminal object specified by TERMINAL.  TERMINAL may
   be a terminal object, a frame, or nil for the terminal device of
   the current frame.  If TERMINAL is neither from the above or the
   resulting terminal object is deleted, return NULL.  */

// C: `static struct terminal *decode_terminal (Lisp_Object terminal)`
unsafe fn decode_terminal (mut terminal: Lisp_Object) -> *mut terminal
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let t: *mut terminal;

  if NILP (terminal)
    { terminal = selected_frame; }
  t = if TERMINALP (terminal)
      { XTERMINAL (terminal) }
      else if FRAMEP (terminal) { FRAME_TERMINAL! (XFRAME (terminal)) }
      else { std::ptr::null_mut () };
  return if ! t.is_null () && ! (*t).name.is_null () { t } else { std::ptr::null_mut () };

  }
}

/* Like above, but throw an error if TERMINAL is not valid or deleted.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let t: *mut terminal = decode_terminal (terminal);

  if t.is_null ()
    { wrong_type_argument (Qterminal_live_p (), terminal); }
  return t;

  }
}

/* Like decode_live_terminal, but ensure that the resulting terminal
   object refers to a text-based terminal device.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn decode_tty_terminal (terminal: Lisp_Object) -> *mut terminal
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let t: *mut terminal = decode_live_terminal (terminal);

  return if (*t).r#type == output_termcap || (*t).r#type == output_msdos_raw
	 { t } else { std::ptr::null_mut () };

  }
}

/* Return an active (not suspended) text-based terminal device that uses
   the tty device with the given NAME, or NULL if the named terminal device
   is not opened.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn get_named_terminal (name: *const c_char) -> *mut terminal
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut t: *mut terminal;

  eassert! (! name.is_null ());

  t = terminal_list;
  while ! t.is_null ()
    {
      let tty: *mut tty_display_info = (*t).display_info.tty.cast::<tty_display_info> ();
      if ((*t).r#type == output_termcap || (*t).r#type == output_msdos_raw)
	  && strcmp ((*tty).name, name) == 0
	  && TERMINAL_ACTIVE_P! (t)
	{ return t; }
      t = (*t).next_terminal;
    }
  return std::ptr::null_mut ();

  }
}

/* Allocate basically initialized terminal.  */

// C: `static struct terminal *allocate_terminal (void)`
unsafe fn allocate_terminal () -> *mut terminal
{
  return ALLOCATE_ZEROED_PSEUDOVECTOR! (terminal, glyph_code_table,
					pvec_type::PVEC_TERMINAL) as *mut terminal;
}

/* Create a new terminal object of TYPE and add it to the terminal list.  RIF
   may be NULL if this terminal type doesn't support window-based redisplay.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn create_terminal (r#type: output_method, rif: *mut c_void)
     -> *mut terminal
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let terminal: *mut terminal = allocate_terminal ();
  let mut terminal_coding: Lisp_Object;
  let mut keyboard_coding: Lisp_Object;

  (*terminal).next_terminal = terminal_list;
  terminal_list = terminal;
  (*terminal).r#type = r#type;
  (*terminal).rif = rif;
  // C's `next_terminal_id++` is the fetch-then-increment pair (C has no Rust spelling).
  (*terminal).id = next_terminal_id;
  next_terminal_id += 1;

  (*terminal).keyboard_coding
    = xmalloc (std::mem::size_of::<coding_system> ()) as *mut c_void;
  (*terminal).terminal_coding
    = xmalloc (std::mem::size_of::<coding_system> ()) as *mut c_void;

  /* If default coding systems for the terminal and the keyboard are
     already defined, use them in preference to the defaults.  This is
     needed when Emacs runs in daemon mode.  */
  keyboard_coding = find_symbol_value (Qdefault_keyboard_coding_system ());
  if NILP (keyboard_coding)
      || BASE_EQ (keyboard_coding, Qunbound ())
      || NILP (Fcoding_system_p (keyboard_coding))
    { keyboard_coding = Qno_conversion (); }
  terminal_coding = find_symbol_value (Qdefault_terminal_coding_system ());
  if NILP (terminal_coding)
      || BASE_EQ (terminal_coding, Qunbound ())
      || NILP (Fcoding_system_p (terminal_coding))
    { terminal_coding = Qundecided (); }

  unsafe extern "C" {

  }
  setup_coding_system (keyboard_coding, (*terminal).keyboard_coding as *mut coding_system);
  setup_coding_system (terminal_coding, (*terminal).terminal_coding as *mut coding_system);

  return terminal;

  }
}

/* Low-level function to close all frames on a terminal, remove it
   from the terminal list and free its memory.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn delete_terminal (terminal: *mut terminal)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  // PORTING.md 1.5: C declares `Lisp_Object tail, frame;` uninitialized; the port starts
  // them at Qnil (the loop assigns both before either is read).
  let mut tail: Lisp_Object = Qnil ();
  let mut frame: Lisp_Object = Qnil ();

  /* Protect against recursive calls.  delete_frame calls the
     delete_terminal_hook when we delete our last frame.  */
  if (*terminal).name.is_null ()
    { return; }

  /* Protection while we are in inconsistent state.  */
  block_input ();
  xfree ((*terminal).name as *mut c_void);
  (*terminal).name = std::ptr::null_mut ();

  unsafe extern "C" {

  }
  /* Check for live frames that are still on this terminal.  */
  // C's `FOR_EACH_FRAME (tail, frame)` loop macro (frame.h 1404) is the mirror's
  // `FOR_EACH_FRAME!`, whose last argument is the body (the FOR_EACH_PER_BUFFER_OBJECT_AT
  // shape; see the mirror and docs/ports/macro-body-notes.tsv).
  FOR_EACH_FRAME! (tail, frame, {
      let f: *mut frame = XFRAME (frame);
      if FRAME_LIVE_P (f) && (*f).terminal == terminal
	{
	  /* Pass Qnoelisp rather than Qt.  */
	  delete_frame (frame, Qnoelisp ());
	}
  });

  delete_terminal_internal (terminal);
  unblock_input ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn delete_terminal_internal (terminal: *mut terminal)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut tp: *mut *mut terminal;

  tp = &raw mut terminal_list;
  while *tp != terminal
    {
      if (*tp).is_null ()
	{ emacs_abort (); }
      tp = &raw mut (**tp).next_terminal;
    }
  *tp = (*terminal).next_terminal;

  xfree ((*terminal).keyboard_coding as *mut c_void);
  (*terminal).keyboard_coding = std::ptr::null_mut ();
  xfree ((*terminal).terminal_coding as *mut c_void);
  (*terminal).terminal_coding = std::ptr::null_mut ();

  unsafe extern "C" {

  }
  // C's `terminal->kboard && --terminal->kboard->reference_count == 0`: the block operand
  // keeps C's single `if` and performs the pre-decrement before the comparison.
  if ! (*terminal).kboard.is_null ()
     && {
	  let kb: *mut KBOARD = (*terminal).kboard as *mut KBOARD;
	  (*kb).reference_count -= 1;
	  (*kb).reference_count == 0
	}
    {
      delete_kboard ((*terminal).kboard as *mut KBOARD);
      (*terminal).kboard = std::ptr::null_mut ();
    }

  }
}


DEFUN! ("delete-terminal", Fdelete_terminal, Sdelete_terminal, 0, 2, 0,
       doc: "Delete TERMINAL by deleting all frames on it and closing the terminal.\nTERMINAL may be a terminal object, a frame, or nil (meaning the\nselected frame's terminal).\n\nNormally, you may not delete a display if all other displays are suspended,\nbut if the second argument FORCE is non-nil, you may do so.",
       (terminal: Lisp_Object, force: Lisp_Object),
       {
  let t: *mut terminal = decode_terminal (terminal);

  if t.is_null ()
    { return Qnil (); }

  if NILP (force)
    {
      let mut p: *mut terminal = terminal_list;
      while ! p.is_null () && (p == t || ! TERMINAL_ACTIVE_P! (p))
	{ p = (*p).next_terminal; }

      if p.is_null ()
	{ error (c"Attempt to delete the sole active display terminal".as_ptr ()); }
    }

  if NILP (Vrun_hooks)
    { }
  else if EQ (force, Qnoelisp ())
    { pending_funcalls
	= Fcons (list3 (Qrun_hook_with_args (), Qdelete_terminal_functions (),
			terminal),
		 pending_funcalls); }
  else
    { safe_calln! (Qrun_hook_with_args (), Qdelete_terminal_functions (), terminal); }

  if let Some (hook) = (*t).delete_terminal_hook
    { hook (t); }
  else
    { delete_terminal (t); }

  return Qnil ();
});


DEFUN! ("frame-terminal", Fframe_terminal, Sframe_terminal, 0, 1, 0,
       doc: "Return the terminal that FRAME is displayed on.\nIf FRAME is nil, use the selected frame.\n\nThe terminal device is represented by its integer identifier.",
       (frame: Lisp_Object),
       {
  unsafe extern "C" {

  }
  let t: *mut terminal = FRAME_TERMINAL! (decode_live_frame (frame));

  if t.is_null ()
    { return Qnil (); }
  else
    {
      // PORTING.md 1.5: C declares `Lisp_Object terminal;` and XSETTERMINAL writes it.
      let mut terminal: Lisp_Object = Qnil ();
      XSETTERMINAL! (terminal, t);
      return terminal;
    }
});

DEFUN! ("terminal-live-p", Fterminal_live_p, Sterminal_live_p, 1, 1, 0,
       doc: "Return non-nil if OBJECT is a terminal which has not been deleted.\nReturn nil if OBJECT is not a live display terminal.\nOBJECT may be a terminal object, a frame, or nil (meaning the\nselected frame's terminal).\nIf OBJECT is a live display terminal, return what sort of output\nterminal it uses.  See the documentation of `framep' for possible\nreturn values.",
       (object: Lisp_Object),
       {
  let t: *mut terminal = decode_terminal (object);

  if t.is_null ()
    { return Qnil (); }

  match (*t).r#type
    {
      output_initial =>                    /* The initial frame is like a termcap frame. */
	{ return Qt (); }
      output_termcap =>
	{ return Qt (); }
      output_x_window =>
	{ return Qx (); }
      output_w32 =>
	{ return Qw32 (); }
      output_msdos_raw =>
	{ return Qpc (); }
      output_ns =>
	{ return Qns (); }
      output_pgtk =>
	{ return Qpgtk (); }
      output_haiku =>
	{ return Qhaiku (); }
      output_android =>
	{ return Qandroid (); }
      _ =>
	{ emacs_abort (); }
    }
});

DEFUN! ("frame-initial-p", Fframe_initial_p, Sframe_initial_p, 0, 1, 0,
       doc: "Return non-nil if FRAME is the initial frame.\nThat is, the initial text frame used internally during daemon mode,\nbatch mode, and the early stages of startup.\nIf FRAME is a terminal object, return non-nil if it holds\nthe initial frame.  FRAME defaults to the selected frame.",
       (frame: Lisp_Object),
       {
  let mut frame: Lisp_Object = frame;
  if NILP (frame)
    { frame = selected_frame; }
  if FRAMEP (frame)
    {
      let f: *mut frame = XFRAME (frame);
      return if FRAME_LIVE_P (f) && FRAME_INITIAL_P (f) { Qt () } else { Qnil () };
    }
  let t: *mut terminal = decode_terminal (frame);
  return if ! t.is_null () && (*t).r#type == output_initial { Qt () } else { Qnil () };
});

DEFUN! ("terminal-list", Fterminal_list, Sterminal_list, 0, 0, 0,
       doc: "Return a list of all terminal devices.",
       (),
       {
  // PORTING.md 1.5: C declares `Lisp_Object terminal, terminals = Qnil;`.
  let mut terminal: Lisp_Object = Qnil ();
  let mut terminals: Lisp_Object = Qnil ();
  let mut t: *mut terminal;

  t = terminal_list;
  while ! t.is_null ()
    {
      XSETTERMINAL! (terminal, t);
      terminals = Fcons (terminal, terminals);
      t = (*t).next_terminal;
    }

  return terminals;
});

DEFUN! ("terminal-name", Fterminal_name, Sterminal_name, 0, 1, 0,
       doc: "Return the name of the terminal device TERMINAL.\nIt is not guaranteed that the returned value is unique among opened devices.\n\nTERMINAL may be a terminal object, a frame, or nil (meaning the\nselected frame's terminal).",
       (terminal: Lisp_Object),
       {
  let t: *mut terminal = decode_live_terminal (terminal);

  return if ! (*t).name.is_null () { build_string ((*t).name) } else { Qnil () };
});



/* Set the value of terminal parameter PARAMETER in terminal D to VALUE.
   Return the previous value.  */

// C: `static Lisp_Object store_terminal_param (struct terminal *t, Lisp_Object parameter,
// Lisp_Object value)`
unsafe fn store_terminal_param (t: *mut terminal, parameter: Lisp_Object,
			       value: Lisp_Object) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let old_alist_elt: Lisp_Object = Fassq (parameter, (*t).param_alist);
  if NILP (old_alist_elt)
    {
      tset_param_alist (t, Fcons (Fcons (parameter, value), (*t).param_alist));
      return Qnil ();
    }
  else
    {
      let result: Lisp_Object = Fcdr (old_alist_elt);
      Fsetcdr (old_alist_elt, value);
      return result;
    }

  }
}


DEFUN! ("terminal-parameters", Fterminal_parameters, Sterminal_parameters, 0, 1, 0,
       doc: "Return the parameter-alist of terminal TERMINAL.\nThe value is a list of elements of the form (PARM . VALUE), where PARM\nis a symbol.\n\nTERMINAL can be a terminal object, a frame, or nil (meaning the\nselected frame's terminal).",
       (terminal: Lisp_Object),
       {
  return Fcopy_alist ((*decode_live_terminal (terminal)).param_alist);
});

DEFUN! ("terminal-parameter", Fterminal_parameter, Sterminal_parameter, 2, 2, 0,
       doc: "Return TERMINAL's value for parameter PARAMETER.\nTERMINAL can be a terminal object, a frame, or nil (meaning the\nselected frame's terminal).",
       (terminal: Lisp_Object, parameter: Lisp_Object),
       {
  CHECK_SYMBOL (parameter);
  return Fcdr (Fassq (parameter, (*decode_live_terminal (terminal)).param_alist));
});

DEFUN! ("set-terminal-parameter", Fset_terminal_parameter,
       Sset_terminal_parameter, 3, 3, 0,
       doc: "Set TERMINAL's value for parameter PARAMETER to VALUE.\nReturn the previous value of PARAMETER.\n\nTERMINAL can be a terminal object, a frame or nil (meaning the\nselected frame's terminal).",
       (terminal: Lisp_Object, parameter: Lisp_Object, value: Lisp_Object),
       {
  return store_terminal_param (decode_live_terminal (terminal), parameter, value);
});

// C 587-622: `#if HAVE_STRUCT_UNIPAIR_UNICODE` wraps the includes of <errno.h>,
// <linux/kd.h> and <sys/ioctl.h> *and* `calculate_glyph_code_table`.  The constant is
// undefined in the frozen configuration, so the whole block is dead text (P-51).  The two
// call sites keep C's text behind `feature = "unipair_unicode"` (below and in
// `terminal_glyph_code`), which is what the `--check` token set matches; the function body
// itself is *not* written out, because it needs three headers no mirror carries and a
// `struct unipair`/`struct unimapdesc` the port would have to invent (recorded in
// docs/ports/terminal.md's Decisions).

/* Return the glyph code in T of character CH, or -1 if CH does not
   have a font position in T, or nil if T does not report glyph codes.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn terminal_glyph_code (t: *mut terminal, ch: c_int) -> Lisp_Object
{
  #[cfg (feature = "unipair_unicode")]
  {
    /* Heuristically assume that a terminal supporting glyph codes is in
       UTF-8 mode if and only if its coding system is UTF-8 (Bug#26396).  */
    if (*t).r#type == output_termcap
	&& (*(*t).terminal_coding as *mut coding_system).encoder == Some (encode_coding_utf_8)
      {
	/* As a hack, recompute the table when CH is the maximum
	   character.  */
	if NILP ((*t).glyph_code_table) || ch == MAX_CHAR
	  { calculate_glyph_code_table (t); }

	if ! EQ ((*t).glyph_code_table, Qt ())
	  { return char_table_ref ((*t).glyph_code_table, ch); }
      }
  }

  return Qnil ();
}

/* Initial frame has no device-dependent output data, but has
   face cache which should be freed when the frame is deleted.  */

// C: `static void initial_free_frame_resources (struct frame *f)` — its address goes into
// the mirror's `delete_frame_hook`, which is spelled `*mut c_void` (P-72 rule ③), so the one
// cast lives here.
unsafe extern "C-unwind" fn initial_free_frame_resources (f: *mut c_void)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  eassert! (FRAME_INITIAL_P (f as *mut frame));
  unsafe extern "C-unwind" {
    /// C: void free_frame_faces (struct frame *)  (xfaces.c)
    pub fn free_frame_faces (f: *mut frame);
  }
  free_frame_faces (f as *mut frame);

  }
}

/* Create the bootstrap display terminal for the initial frame.
   Returns a terminal of type output_initial.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn init_initial_terminal () -> *mut terminal
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if initialized || ! terminal_list.is_null () || ! tty_list.is_null ()
    { emacs_abort (); }

  initial_terminal = create_terminal (output_initial, std::ptr::null_mut ());
  (*initial_terminal).name = xstrdup (c"initial_terminal".as_ptr ());
  (*initial_terminal).kboard = initial_kboard as *mut c_void;
  (*initial_terminal).delete_terminal_hook = Some (delete_initial_terminal);
  (*initial_terminal).delete_frame_hook = Some (initial_free_frame_resources);
  unsafe extern "C-unwind" {
    /// C: bool tty_defined_color (struct frame *f, const char *color_name,
    ///                            Emacs_Color *color_def, bool alloc, bool makeIndex)
    ///    (xfaces.c 1141-1144, the `defined_color_hook` at termhooks.h 593-596)
    pub fn tty_defined_color (f: *mut c_void, color_name: *const c_char,
			      color_def: *mut Emacs_Color, alloc: bool,
			      makeIndex: bool) -> bool;
  }
  (*initial_terminal).defined_color_hook = Some (tty_defined_color); /* xfaces.c */
  /* Other hooks are NULL by default.  */

  return initial_terminal;

  }
}

/* Deletes the bootstrap terminal device.
   Called through delete_terminal_hook. */

// C: `static void delete_initial_terminal (struct terminal *terminal)` — its address goes
// into `delete_terminal_hook`, whose parameter is the mirror's `*mut terminal`.
unsafe extern "C-unwind" fn delete_initial_terminal (terminal: *mut terminal)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if terminal != initial_terminal
    { emacs_abort (); }

  delete_terminal (terminal);
  initial_terminal = std::ptr::null_mut ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_terminal ()
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  DEFVAR_LISP! (c"ring-bell-function", Vring_bell_function,
    "Non-nil means call this function to ring the bell.\nThe function should accept no arguments.  ");
  Vring_bell_function = Qnil ();

  DEFVAR_LISP! (c"delete-terminal-functions", Vdelete_terminal_functions,
    "Special hook run when a terminal is deleted.\nEach function is called with argument, the terminal.\nThis may be called just before actually deleting the terminal,\nor some time later.  ");
  Vdelete_terminal_functions = Qnil ();

  DEFSYM! (Qterminal_live_p, "terminal-live-p");
  DEFSYM! (Qframe_initial_p, "frame-initial-p");
  DEFSYM! (Qdelete_terminal_functions, "delete-terminal-functions");
  DEFSYM! (Qrun_hook_with_args, "run-hook-with-args");

  defsubr (&raw mut Sdelete_terminal);
  defsubr (&raw mut Sframe_terminal);
  defsubr (&raw mut Sterminal_live_p);
  defsubr (&raw mut Sframe_initial_p);
  defsubr (&raw mut Sterminal_list);
  defsubr (&raw mut Sterminal_name);
  defsubr (&raw mut Sterminal_parameters);
  defsubr (&raw mut Sterminal_parameter);
  defsubr (&raw mut Sset_terminal_parameter);

  Fprovide (intern_c_string (c"multi-tty".as_ptr ()), Qnil ());
  DEFSYM! (Qdefault_keyboard_coding_system, "default-keyboard-coding-system");
  DEFSYM! (Qdefault_terminal_coding_system, "default-terminal-coding-system");

  }
}
