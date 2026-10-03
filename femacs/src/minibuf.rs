// femacs: port of src/minibuf.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// subr, macro, file-scope object, local), same control flow, same error model.  C's eight
// non-`static` functions (`move_minibuffers_onto_frame`, `this_minibuffer_depth`,
// `is_minibuffer`, `get_minibuffer`, `barf_if_interaction_inhibited`,
// `set_initial_minibuffer_mode`, `init_minibuf_once`, `syms_of_minibuf`) and its three
// non-`static` objects (`Vminibuffer_list`, `minibuf_level`, `last_minibuf_string`) carry
// `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`
// except the five whose address C takes — `minibuffer_ent_greater` (handed to `merge_c`),
// `run_exit_minibuf_hook` and `read_minibuf_unwind`/`minibuffer_unwind` (handed to
// `record_unwind_protect`/`record_unwind_protect_void`) and
// `init_minibuf_once_for_pdumper` (handed to `pdumper_do_now_and_after_load`) — which are
// `unsafe extern "C" fn` (PORTING.md 1.4).  The six `static` objects
// (`Vcommand_loop_level_list`, `minibuf_save_list`, `minibuf_prompt`, `MB_frame`,
// `minibuf_prompt_width`, `exp_MB_frame`) are unexported `static mut`s.
//
// **`src/minibuf.c` is in `docs/baseline/config/cond-eval-verified.txt`**
// (`python3 tools/check-cond-eval.py src/minibuf.c` reports
// `7 conditions, 0 unknown, 0 disagreements [VERIFIED]`), so the frozen configuration's
// branch selection is clang's and the dead arms are **recorded in docs/ports/minibuf.md,
// not transcribed** (P-08, status 3.49/3.51): the `#ifdef HAVE_NTGUI` include (C 37-39),
// the literal `#if 0` blocks (C 92-100 the whole `minibuf_stays_put`, C 1173-1177 the
// `temp`/`minibuf_window` restore in `read_minibuf_unwind`), the
// `#ifdef HAVE_WINDOW_SYSTEM` arm in `read_minibuf` (C 607-610, `HAVE_WINDOW_SYSTEM` is
// `undef`: macros.tsv 1647), the `#else` arm of the `HAVE_NTGUI` pair at C 933-943 (the
// live text is `flush_frame`), the `#ifdef HAVE_TEXT_CONVERSION` arm in
// `Fread_from_minibuffer` (C 1396-1408, `undef`: macros.tsv 1614) and the `#ifdef NOTDEF`
// `read-function` DEFUN (C 1485-1494).  That is why the ledger declares the `live` basis
// in `docs/ports/line-drift-tolerance.tsv`.
//
// The names this unit takes from headers whose mirrors do not carry them are declared at
// their use site below, each with C's own prototype in a `/// C:` line (the
// insdel.rs/indent.rs shape); `sysstdio.h` is `<stdio.h>` (the port's stdio mirror),
// `systty.h` has no mirror at all, and `sysdep.h`'s `struct emacs_tty` is reached by value
// here, which is why its record is declared in full rather than degraded to a pointer.
//
// C's uninitialised locals are zero-initialised (PORTING.md 1.5) and C 77-80's, 260-262's
// and 2265's forward declarations are dropped (PORTING.md 0.1-12); both are named in
// `docs/ports/minibuf.md`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut, unused_labels, unused_comparisons,
         overflowing_literals)]

include!("include/config.rs");                 // C 21: `#include <config.h>`
include!("../lib/include/errno.rs");        // C 22: `#include <errno.h>`
include!("../lib/include/binary-io.rs");    // C 24: `#include <binary-io.h>`
include!("../lib/include/stdio.rs");        // C 33: `#include "sysstdio.h"` (FILE, sprintf, EOF)
include!("include/lisp.rs");                   // C 26: `#include "lisp.h"`
include!("include/character.rs");              // C 27: `#include "character.h"`
include!("include/itree.rs");                  // buffer.h 28: `#include "itree.h"`
include!("include/buffer.rs");                 // C 28: `#include "buffer.h"`
include!("include/systime.rs");                 // termhooks.h 28: `#include "systime.h"` (Time)
include!("include/dispextern.rs");             // frame.h 24: `#include "dispextern.h"`
include!("include/coding.rs");                 // termhooks.h 26: `#include "coding.h"`
include!("include/composite.rs");              // dispextern.h 25: `#include "composite.h"`
include!("include/keyboard.rs");               // C 29: `#include "keyboard.h"`
include!("include/termhooks.rs");              // frame.h 22: `#include "termhooks.h"`
include!("include/window.rs");                 // C 31: `#include "window.h"`
include!("include/frame.rs");                  // C 30: `#include "frame.h"`
include!("include/keymap.rs");                 // C 32: `#include "keymap.h"`
include!("include/systty.rs");               // C 34: `#include "systty.h"`
include!("include/pdumper.rs");                // C 35: `#include "pdumper.h"`

/* C 34's `#include "systty.h"` has no mirror (PORTING.md P-26/P-66): the record it defines
   and the three functions it declares are below, with C's own text.  C 37-39's
   `#ifdef HAVE_NTGUI #include "w32term.h" #endif` is dead in the frozen configuration and
   is recorded in docs/ports/minibuf.md (P-08).  */

/* The names C gets from headers whose mirrors do not carry them yet (the terminal.rs /
   insdel.rs shape), each with C's own prototype in a `/// C:` line.  */

/* <stdio.h>: `getchar` and `putchar` now come from the `stdio.rs` mirror (2026-10-04); this
   unit's use-site copies are gone, and the mirror is the single declaration point.  */

/// <unistd.h>: `#define STDIN_FILENO 0` (measured; the callproc.rs shape).
const STDIN_FILENO: c_int = 0;

/* C 34's `#include "systty.h"`: `struct emacs_tty` (and the real `struct termios` inside
   it), `CDISABLE` and the three tty declarations now come from `include/systty.rs`, which
   brings `../lib/include/termios.rs` with it.  Until 2026-10-04 this unit carried an opaque
   `struct termios { _storage: [u8; 72] }` stand-in (it could not be read field by field) plus
   its own `emacs_tty` and three use-site declarations; all three are gone - the mirror is the
   single definition point (PORTING.md 0.1-3, P-21/P-72 (3)).  */

unsafe extern "C" {
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);`
  fn FUNCTIONP (a: Lisp_Object) -> bool;
  /// C: lisp.h 4266: `extern Lisp_Object merge_c (Lisp_Object, Lisp_Object, bool (*)
  /// (Lisp_Object, Lisp_Object));`
  fn merge_c (org_l1: Lisp_Object, org_l2: Lisp_Object,
              pred: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object) -> bool>)
    -> Lisp_Object;
  /// C: lisp.h 4271: `extern Lisp_Object nconc2 (Lisp_Object, Lisp_Object);`
  fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));`
  fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
  /// C: lisp.h 5097: `extern void safe_run_hooks (Lisp_Object);`
  fn safe_run_hooks (hook: Lisp_Object);
  /// C: lisp.h 5102: `extern Lisp_Object recursive_edit_1 (void);`
  fn recursive_edit_1 () -> Lisp_Object;
  /// C: lisp.h 5119: `extern Lisp_Object do_switch_frame (Lisp_Object, int, int,
  /// Lisp_Object);`
  fn do_switch_frame (frame: Lisp_Object, for_deletion: c_int, track: c_int,
                      no_catch: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 5120: `extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);`
  fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;
  /// C: window.h 1127: `extern void restore_window_configuration (Lisp_Object);`
  fn restore_window_configuration (configuration: Lisp_Object);
  /// C: window.h 1128: `extern void restore_focus_frame (Lisp_Object);`
  fn restore_focus_frame (frame_and_focus_frame: Lisp_Object);
  /// C: window.h 1139-1140: `void set_window_buffer (Lisp_Object window, Lisp_Object
  /// buffer, bool run_hooks_p, bool keep_margins_p);`
  fn set_window_buffer (window: Lisp_Object, buffer: Lisp_Object, run_hooks_p: bool,
                        keep_margins_p: bool);
  /// C: dispextern.h 3570: `bool resize_mini_window (struct window *, bool);`
  fn resize_mini_window (w: *mut window, pixelwise: bool) -> bool;
  /// C: lisp.h 4565-4566: `extern Lisp_Object make_specified_string (const char *,
  /// ptrdiff_t, ptrdiff_t, bool);` (ccl.rs declares it too; the mirror does not carry it).
  fn make_specified_string (contents: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
                            multibyte: bool) -> Lisp_Object;
}

/* C: lisp.h 2504-2506: `#define DOOBARRAY(oa, it) for (obarray_iter_t it =
   make_obarray_iter (oa); !obarray_iter_at_end (&it); obarray_iter_step (&it))`.  C's loop
   macro takes the body at the call site; the mirror's `obarray_iter_t` and its four helpers
   are lisp.rs's, and the `for` keyword is the token with no counterpart (the
   FOR_EACH_FRAME note in docs/ports/macro-body-notes.tsv).  */
macro_rules! DOOBARRAY {
  ($oa:expr, $it:ident, $body:block) => {
    {
      let mut $it: obarray_iter_t = make_obarray_iter ($oa);
      while ! obarray_iter_at_end (&raw mut $it)
        {
          $body
          obarray_iter_step (&raw mut $it);
        }
    }
  };
}

/* List of buffers for use as minibuffers.
   The first element of the list is used for the outermost minibuffer
   invocation, the next element is used for a recursive minibuffer
   invocation, etc.  The list is extended at the end as deeper
   minibuffer recursions are encountered.  */

#[unsafe(no_mangle)]
pub static mut Vminibuffer_list: Lisp_Object = lisp_h_Qnil ();

static mut Vcommand_loop_level_list: Lisp_Object = lisp_h_Qnil ();

/* Data to remember during recursive minibuffer invocations.  */

static mut minibuf_save_list: Lisp_Object = lisp_h_Qnil ();

/* Depth in minibuffer invocations.  */

#[unsafe(no_mangle)]
pub static mut minibuf_level: EMACS_INT = 0;

/* Fread_minibuffer leaves the input here as a string.  */

#[unsafe(no_mangle)]
pub static mut last_minibuf_string: Lisp_Object = lisp_h_Qnil ();

/* Prompt to display in front of the mini-buffer contents.  */

static mut minibuf_prompt: Lisp_Object = lisp_h_Qnil ();

/* The frame containing the most recently opened minibuffer.  This is
   used only when `minibuffer-follows-selected-frame' is neither nil
   nor t.  */

static mut MB_frame: Lisp_Object = lisp_h_Qnil ();

/* Width of current mini-buffer prompt.  Only set after display_line
   of the line that contains the prompt.  */

static mut minibuf_prompt_width: ptrdiff_t = 0;

/* C 77-80's forward declarations (`nth_minibuffer`, `minibuf_c_loop_level`,
   `set_minibuffer_mode`, `live_minibuffer_p`) and C 260-262's three are dropped: Rust's
   items are order-independent and a declaration plus the definition is E0428
   (PORTING.md 0.1-12).  */

/* Return TRUE when a frame switch causes a minibuffer on the old
   frame to move onto the new one. */
unsafe fn minibuf_follows_frame () -> bool
{
  return EQ (Fdefault_toplevel_value (Qminibuffer_follows_selected_frame ()),
             Qt ());
}

/* C 92-100's `#if 0` block — `minibuf_stays_put`, "Return TRUE when a minibuffer always
   remains on the frame where it was first invoked" — is dead text; it is recorded in
   docs/ports/minibuf.md rather than transcribed (P-08).  */

/* Return TRUE when opening a (recursive) minibuffer causes
   minibuffers on other frames to move to the selected frame.  */
unsafe fn minibuf_moves_frame_when_opened () -> bool
{
  return !NILP (Fdefault_toplevel_value (Qminibuffer_follows_selected_frame ()));
}

/* Put minibuf on currently selected frame's minibuffer.
   We do this whenever the user starts a new minibuffer
   or when a minibuffer exits.  */

unsafe fn choose_minibuf_frame ()
{
  if FRAMEP (selected_frame)
      && FRAME_LIVE_P (XFRAME (selected_frame))
      && WINDOW_LIVE_P ((*XFRAME (selected_frame)).minibuffer_window)
      && !EQ (minibuf_window, (*XFRAME (selected_frame)).minibuffer_window)
    {
      let sf: *mut frame = XFRAME (selected_frame);
      /* I don't think that any frames may validly have a null
	 minibuffer window anymore.  (2021-04-15): Tooltip frames have
	 a null MB.  Comment out the following.  */
      /* if (NILP (sf->minibuffer_window)) */
      /* 	emacs_abort (); */

      minibuf_window = (*sf).minibuffer_window;
    }
}

/* If ENT1 has a higher minibuffer index than ENT2, return true.  More
precisely, compare the buffer components of each window->prev_buffers
entry.  */
unsafe extern "C" fn minibuffer_ent_greater (ent1: Lisp_Object, ent2: Lisp_Object) -> bool
{
  return this_minibuffer_depth (Fcar (ent1))
    > this_minibuffer_depth (Fcar (ent2)) ;
}

/* Move the ordered "stack" of minibuffers from SOURCE_WINDOW to
   DEST_WINDOW, interleaving those minibuffers with any in DEST_WINDOW
   to produce an ordered combination.  The ordering is by minibuffer
   depth.  A stack of minibuffers consists of the minibuffer currently
   in DEST/SOURCE_WINDOW together with any recorded in the
   ->prev_buffers field of the struct window.  */
unsafe fn zip_minibuffer_stacks (dest_window: Lisp_Object, source_window: Lisp_Object)
{
  let dw: *mut window = XWINDOW (dest_window);
  let sw: *mut window = XWINDOW (source_window);
  let mut acc: Lisp_Object;
  let mut d_ent: Lisp_Object;	/* Entry from dw->prev_buffers */

  if !live_minibuffer_p ((*dw).contents)
      && NILP ((*dw).prev_buffers)
    {
      set_window_buffer (dest_window, (*sw).contents, false, false);
      Fset_window_start (dest_window, Fwindow_start (source_window), Qnil ());
      Fset_window_point (dest_window, Fwindow_point (source_window));
      wset_prev_buffers (dw, (*sw).prev_buffers);
      set_window_buffer (source_window, nth_minibuffer (0), false, false);
      wset_prev_buffers (sw, Qnil ());
      return;
    }

  calln! (Qrecord_window_buffer (), dest_window);
  calln! (Qrecord_window_buffer (), source_window);

  acc = merge_c ((*dw).prev_buffers, (*sw).prev_buffers, Some (minibuffer_ent_greater));

  if !NILP (acc)
    {
      d_ent = Fcar (acc);
      acc = Fcdr (acc);
      set_window_buffer (dest_window, Fcar (d_ent), false, false);
      Fset_window_start (dest_window, Fcar (Fcdr (d_ent)), Qnil ());
      Fset_window_point (dest_window, Fcar (Fcdr (Fcdr (d_ent))));
    }

  wset_prev_buffers (dw, acc);
  wset_prev_buffers (sw, Qnil ());
  set_window_buffer (source_window, nth_minibuffer (0), false, false);
}

/* If `minibuffer_follows_selected_frame' is t, or we're about to
   delete a frame which potentially "contains" minibuffers, move them
   from the old frame to the to-be-selected frame.  This function is
   intended to be called from `do_switch_frame' in frame.c.  OF is the
   old frame, FRAME is the to-be-selected frame, and FOR_DELETION is true
   if OF is about to be deleted.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn move_minibuffers_onto_frame (of: *mut frame, frame: Lisp_Object,
                                                      for_deletion: bool)
{
  let f: *mut frame = XFRAME (frame);

  minibuf_window = (*f).minibuffer_window;
  if !(minibuf_level != 0
	&& (for_deletion || minibuf_follows_frame () || FRAME_INITIAL_P (of)))
    { return; }
  if FRAME_LIVE_P (f)
      && !EQ ((*f).minibuffer_window, (*of).minibuffer_window)
      && WINDOW_LIVE_P ((*f).minibuffer_window) /* F not a tooltip frame */
      && WINDOW_LIVE_P ((*of).minibuffer_window)
    {
      zip_minibuffer_stacks ((*f).minibuffer_window, (*of).minibuffer_window);
      if for_deletion && XFRAME (MB_frame) != of
	{ MB_frame = frame; }
    }
}

DEFUN! ("active-minibuffer-window", Factive_minibuffer_window,
       Sactive_minibuffer_window, 0, 0, 0,
       doc: "Return the currently active minibuffer window, or nil if none.",
       (),
       {
  let mut frames: Lisp_Object = Qnil ();   /* PORTING.md 1.5: C declares these uninitialized.  */
  let mut frame: Lisp_Object = Qnil ();
  let mut f: *mut frame;
  let mut innermost_MB: Lisp_Object;

  if minibuf_level == 0
    { return Qnil (); }

  innermost_MB = nth_minibuffer (minibuf_level);
  if NILP (innermost_MB)
    { emacs_abort (); }
  FOR_EACH_FRAME! (frames, frame, {
      f = XFRAME (frame);
      if FRAME_LIVE_P (f)
	  && WINDOW_LIVE_P ((*f).minibuffer_window)
	  && EQ ((*XWINDOW ((*f).minibuffer_window)).contents, innermost_MB)
	{ return (*f).minibuffer_window; }
  });
  return minibuf_window;	/* "Can't happen." */
});

DEFUN! ("set-minibuffer-window", Fset_minibuffer_window,
       Sset_minibuffer_window, 1, 1, 0,
       doc: "Specify which minibuffer window to use for the minibuffer.\nThis affects where the minibuffer is displayed if you put text in it\nwithout invoking the usual minibuffer commands.",
       (window: Lisp_Object),
       {
  CHECK_WINDOW (window);
  if ! MINI_WINDOW_P (XWINDOW (window))
    { error (c"Window is not a minibuffer window".as_ptr ()); }

  minibuf_window = window;

  return window;
});

/* Actual minibuffer invocation.  */

/* Read a Lisp object from VAL and return it.  If VAL is an empty
   string, and DEFALT is a string, read from DEFALT instead of VAL.  */

unsafe fn string_to_object (val: Lisp_Object, defalt: Lisp_Object) -> Lisp_Object
{
  let mut val: Lisp_Object = val;
  let mut expr_and_pos: Lisp_Object = Qnil ();
  let mut pos: ptrdiff_t;

  if STRINGP (val) && SCHARS (val) == 0
    {
      if STRINGP (defalt)
	{ val = defalt; }
      else if CONSP (defalt) && STRINGP (XCAR (defalt))
	{ val = XCAR (defalt); }
    }

  expr_and_pos = Fread_from_string (val, Qnil (), Qnil ());
  pos = XFIXNUM (Fcdr (expr_and_pos));
  if pos != SCHARS (val)
    {
      /* Ignore trailing whitespace; any other trailing junk
	 is an error.  */
      let mut i: ptrdiff_t;
      pos = string_char_to_byte (val, pos);
      i = pos;
      while i < SBYTES (val)
	{
	  let c: c_int = SREF (val, i as usize) as c_int;
	  if c != b' ' as c_int && c != b'\t' as c_int && c != b'\n' as c_int
	    {
	      xsignal1 (Qinvalid_read_syntax (),
			build_string (c"Trailing garbage following expression".as_ptr ()));
	    }
	  i += 1;
	}
    }

  val = Fcar (expr_and_pos);
  return val;
}

/* Like read_minibuf but reading from stdin.  This function is called
   from read_minibuf to do the job if noninteractive.  */

unsafe fn read_minibuf_noninteractive (prompt: Lisp_Object, expflag: bool,
				       defalt: Lisp_Object) -> Lisp_Object
{
  let mut size: ptrdiff_t;
  let mut len: ptrdiff_t;
  let mut line: *mut c_char;
  let mut val: Lisp_Object;
  let mut c: c_int;
  let mut hide_char: c_uchar = 0;
  let mut etty: emacs_tty = std::mem::zeroed ();   /* PORTING.md 1.5 (`struct emacs_tty etty;`).  */
  let mut etty_valid: bool = false;                /* C's `bool etty_valid UNINIT;`.  */

  /* Check, whether we need to suppress echoing.  */
  if CHARACTERP (Vread_hide_char)
    { hide_char = XFIXNAT (Vread_hide_char) as c_uchar; }

  /* Manipulate tty.  */
  if hide_char != 0
    {
      etty_valid = emacs_get_tty (STDIN_FILENO, &raw mut etty) == 0;
      if etty_valid
	{ set_binary_mode (STDIN_FILENO, O_BINARY); }
      suppress_echo_on_tty (STDIN_FILENO);
    }

  fwrite (SDATA (prompt) as *const c_void, 1, SBYTES (prompt) as usize, stdout);
  fflush (stdout);

  val = Qnil ();
  size = 100;
  len = 0;
  line = xmalloc (size as usize) as *mut c_char;

  while { c = getchar (); c != b'\n' as c_int && c != b'\r' as c_int }
    {
      if c == EOF
	{
	  if *errno () != EINTR
	    { break; }
	}
      else
	{
	  if hide_char != 0
	    { putchar (hide_char as c_int); }
	  if len == size
	    { line = xpalloc (line as *mut c_void, &raw mut size, 1, -1, 1) as *mut c_char; }
	  *line.add (len as usize) = c as c_char;
	  len += 1;
	}
    }

  /* Reset tty.  */
  if hide_char != 0
    {
      putc (b'\n' as c_int, stdout);
      if etty_valid
	{
	  emacs_set_tty (STDIN_FILENO, &raw mut etty, false);
	  set_binary_mode (STDIN_FILENO, O_TEXT);
	}
    }

  if len != 0 || c == b'\n' as c_int || c == b'\r' as c_int
    {
      val = make_string (line, len);
      xfree (line as *mut c_void);
    }
  else
    {
      xfree (line as *mut c_void);
      xsignal1 (Qend_of_file (), build_string (c"Error reading from stdin".as_ptr ()));
    }

  /* If Lisp form desired instead of string, parse it.  */
  if expflag
    { val = string_to_object (val, if CONSP (defalt) { XCAR (defalt) } else { defalt }); }

  return val;
}

/* Return true when BUFFER is an active minibuffer.  */
unsafe fn live_minibuffer_p (buffer: Lisp_Object) -> bool
{
  let mut tem: Lisp_Object;
  let mut i: EMACS_INT;

  if EQ (buffer, Fcar (Vminibuffer_list))
    /*  *Minibuf-0* is never active.  */
    { return false; }
  tem = Fcdr (Vminibuffer_list);
  i = 1;
  while i <= minibuf_level
    {
      if EQ (Fcar (tem), buffer)
	{ return true; }
      i += 1;
      tem = Fcdr (tem);
    }
  return false;
}

DEFUN! ("minibufferp", Fminibufferp,
       Sminibufferp, 0, 2, 0,
       doc: "Return t if BUFFER is a minibuffer.\nNo argument or nil as argument means use current buffer as BUFFER.\nBUFFER can be a buffer or a buffer name.  If LIVE is non-nil, then\nreturn t only if BUFFER is an active minibuffer.",
       (buffer: Lisp_Object, live: Lisp_Object),
       {
  let mut buffer: Lisp_Object = buffer;
  if NILP (buffer)
    { buffer = Fcurrent_buffer (); }
  else if STRINGP (buffer)
    { buffer = Fget_buffer (buffer); }
  else
    { CHECK_BUFFER (buffer); }

  let found: bool = if NILP (live)
    { !NILP (Fmemq (buffer, Vminibuffer_list)) }
    else
    { live_minibuffer_p (buffer) };
  return if found { Qt () } else { Qnil () };
});

DEFUN! ("innermost-minibuffer-p", Finnermost_minibuffer_p,
       Sinnermost_minibuffer_p, 0, 1, 0,
       doc: "Return t if BUFFER is the most nested active minibuffer.\nNo argument or nil as argument means use the current buffer as BUFFER.",
       (buffer: Lisp_Object),
       {
  let mut buffer: Lisp_Object = buffer;
  if NILP (buffer)
    { buffer = Fcurrent_buffer (); }
  return if BASE_EQ (buffer, Fcar (Fnthcdr (make_fixnum (minibuf_level),
					    Vminibuffer_list)))
    { Qt () }
    else
    { Qnil () };
});

DEFUN! ("minibuffer-innermost-command-loop-p", Fminibuffer_innermost_command_loop_p,
       Sminibuffer_innermost_command_loop_p, 0, 1, 0,
       doc: "Return t if BUFFER is a minibuffer at the current command loop level.\nNo argument or nil as argument means use the current buffer as BUFFER.",
       (buffer: Lisp_Object),
       {
  let mut buffer: Lisp_Object = buffer;
  let mut depth: EMACS_INT;
  if NILP (buffer)
    { buffer = Fcurrent_buffer (); }
  depth = this_minibuffer_depth (buffer);
  return if depth != 0 && minibuf_c_loop_level (depth) == command_loop_level
    { Qt () }
    else
    { Qnil () };
});

/* Return the nesting depth of the active minibuffer BUFFER, or 0 if
   BUFFER isn't such a thing.  If BUFFER is nil, this means use the current
   buffer.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn this_minibuffer_depth (buffer: Lisp_Object) -> EMACS_INT
{
  let mut buffer: Lisp_Object = buffer;
  let mut i: EMACS_INT;
  let mut bufs: Lisp_Object;

  if NILP (buffer)
    { buffer = Fcurrent_buffer (); }
  i = 1;
  bufs = Fcdr (Vminibuffer_list);
  while i <= minibuf_level
    {
      if EQ (Fcar (bufs), buffer)
	{ return i; }
      i += 1;
      bufs = Fcdr (bufs);
    }
  return 0;
}

DEFUN! ("abort-minibuffers", Fabort_minibuffers, Sabort_minibuffers, 0, 0, "",
       doc: "Abort the current minibuffer.\nIf we are not currently in the innermost minibuffer, prompt the user to\nconfirm the aborting of the current minibuffer and all contained ones.",
       (),
       {
  let minibuf_depth: EMACS_INT = this_minibuffer_depth (Qnil ());
  let mut array: [Lisp_Object; 2] = [Qnil (); 2];
  AUTO_STRING! (fmt, c"Abort %s minibuffer levels? ".as_ptr ());

  if minibuf_depth == 0
    { error (c"Not in a minibuffer".as_ptr ()); }
  if NILP (Fminibuffer_innermost_command_loop_p (Qnil ()))
    { error (c"Not in most nested command loop".as_ptr ()); }
  if minibuf_depth < minibuf_level
    {
      array[0] = fmt;
      array[1] = make_fixnum (minibuf_level - minibuf_depth + 1);
      if !NILP (Fyes_or_no_p (Fformat (2, array.as_mut_ptr ())))
	{
	  /* Due to the above check, the current minibuffer is in the
	     most nested command loop, which means that we don't have
	     to abort any extra non-minibuffer recursive edits.  Thus,
	     the number of recursive edits we have to abort equals the
	     number of minibuffers we have to abort.  */
	  calln! (Qminibuffer_quit_recursive_edit (), array[1]);
	}
    }
  else
    { call0 (Qminibuffer_quit_recursive_edit ()); }
  return Qnil ();
});

DEFUN! ("minibuffer-prompt-end", Fminibuffer_prompt_end,
       Sminibuffer_prompt_end, 0, 0, 0,
       doc: "Return the buffer position of the end of the minibuffer prompt.\nReturn (point-min) if current buffer is not a minibuffer.",
       (),
       {
  /* This function is written to be most efficient when there's a prompt.  */
  let mut beg: Lisp_Object;
  let mut end: Lisp_Object;
  let mut tem: Lisp_Object;
  beg = make_fixnum (BEGV! ());

  tem = Fmemq (Fcurrent_buffer (), Vminibuffer_list);
  if NILP (tem)
    { return beg; }

  end = Ffield_end (beg, Qnil (), Qnil ());

  if XFIXNUM (end) == ZV! () && NILP (Fget_char_property (beg, Qfield (), Qnil ()))
    { return beg; }
  else
    { return end; }
});

DEFUN! ("minibuffer-contents", Fminibuffer_contents,
       Sminibuffer_contents, 0, 0, 0,
       doc: "Return the user input in a minibuffer as a string.\nIf the current buffer is not a minibuffer, return its entire contents.",
       (),
       {
  let prompt_end: ptrdiff_t = XFIXNUM (Fminibuffer_prompt_end ());
  return make_buffer_string (prompt_end, ZV! (), true);
});

DEFUN! ("minibuffer-contents-no-properties", Fminibuffer_contents_no_properties,
       Sminibuffer_contents_no_properties, 0, 0, 0,
       doc: "Return the user input in a minibuffer as a string, without text-properties.\nIf the current buffer is not a minibuffer, return its entire contents.",
       (),
       {
  let prompt_end: ptrdiff_t = XFIXNUM (Fminibuffer_prompt_end ());
  return make_buffer_string (prompt_end, ZV! (), false);
});

/* C: frame.h 1913-1920: `INLINE void flush_frame (struct frame *f)` — the frame.h mirror
   does not carry it, nor `FRAME_RIF`; C's body is written here with C's own text.  */
#[inline(always)]
unsafe fn flush_frame (f: *mut frame)
{
  let rif: *mut redisplay_interface =
    (*(*f).terminal).rif as *mut redisplay_interface;   /* C: `FRAME_RIF (f)`.  */

  if !rif.is_null () && (*rif).flush_display.is_some ()
    { ((*rif).flush_display).unwrap () (f); }
}

/* C: dispextern.h 3110-3200: `struct redisplay_interface`.  It has no mirror (termhooks.rs
   181 degrades `rif` to `*mut c_void`), and `flush_frame` reaches exactly one member,
   `flush_display` at offset 72 (clang, frozen target).  The ten members C declares before it
   are all pointers, so the port mirrors the prefix as an opaque pointer array and pins the
   one offset that matters; `sizeof` is *not* pinned, because this is a prefix by
   construction (docs/ports/minibuf.md records the gap).  */
#[repr(C)]
pub struct redisplay_interface
{
  _prefix: [*mut c_void; 9],
  pub flush_display: Option<unsafe extern "C" fn (f: *mut frame)>,
}

const _: () = {
  assert! (std::mem::offset_of! (redisplay_interface, flush_display) == 72);
};

unsafe extern "C" {
  /// C: dispextern.h 3897: `void update_frame (struct frame *, bool);`
  fn update_frame (f: *mut frame, force_p: bool);
  /// C: dispextern.h 4012: `bool is_tty_frame (struct frame *f);`
  fn is_tty_frame (f: *mut frame) -> bool;
  /// C: dispextern.h 4017: `void combine_updates_for_frame (struct frame *f, bool
  /// inhibit_id_p);`
  fn combine_updates_for_frame (f: *mut frame, inhibit_id_p: bool);
  /// C: lisp.h 4388: `extern void clear_message (bool, bool);`
  fn clear_message (inhibit_message: bool, inhibit_clear_message: bool);
}

/* C: window.h 543: `#define WINDOW_FRAME(w) ((w)->frame)`.  The window.h mirror is WIP and
   stops before the macro block (PORTING.md P-66), so the port keeps C's macro name, as
   indent.rs/menu.rs do.  */
macro_rules! WINDOW_FRAME { ($w:expr) => { (*($w)).frame }; }

/* Read from the minibuffer using keymap MAP and initial contents INITIAL,
   putting point minus BACKUP_N bytes from the end of INITIAL,
   prompting with PROMPT (a string), using history list HISTVAR
   with initial position HISTPOS.  INITIAL should be a string or a
   cons of a string and an integer.  BACKUP_N should be <= 0, or
   Qnil, which is equivalent to 0.  If INITIAL is a cons, BACKUP_N is
   ignored and replaced with an integer that puts point at one-indexed
   position N in INITIAL, where N is the CDR of INITIAL, or at the
   beginning of INITIAL if N <= 0.

   Normally return the result as a string (the text that was read),
   but if EXPFLAG, read it and return the object read.
   If HISTVAR is given, save the value read on that history only if it doesn't
   match the front of that history list exactly.  The value is pushed onto
   the list as the string that was read.

   DEFALT specifies the default value for the sake of history commands.

   If ALLOW_PROPS or `minibuffer-allow-text-properties' (possibly
   buffer-local) is non-nil, do not throw away text properties.

   if INHERIT_INPUT_METHOD, the minibuffer inherits the
   current input method.  */

unsafe fn read_minibuf (map: Lisp_Object, initial: Lisp_Object, prompt: Lisp_Object,
			expflag: bool,
			histvar: Lisp_Object, histpos: Lisp_Object, defalt: Lisp_Object,
			allow_props: bool, inherit_input_method: bool) -> Lisp_Object
{
  let mut prompt: Lisp_Object = prompt;
  let mut initial: Lisp_Object = initial;
  let mut val: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut mini_frame: Lisp_Object;
  let mut ambient_dir: Lisp_Object;
  let mut minibuffer: Lisp_Object;
  let mut input_method: Lisp_Object;
  let calling_frame: Lisp_Object = selected_frame;
  let calling_window: Lisp_Object = selected_window;
  let mut enable_multibyte: Lisp_Object;
  let mut pos: EMACS_INT = 0;
  /* String to add to the history.  */
  let mut histstring: Lisp_Object;
  let mut histval: Lisp_Object;

  let mut empty_minibuf: Lisp_Object;

  specbind (Qminibuffer_default (), defalt);
  specbind (Qinhibit_read_only (), Qnil ());
  /* Bound recursively so that code can check the current command from
     code running from minibuffer hooks (and the like), without being
     overwritten by subsequent minibuffer calls.  */
  specbind (Qcurrent_minibuffer_command (), Vthis_command);

  /* If Vminibuffer_completing_file_name is `lambda' on entry, it was t
     in previous recursive minibuffer, but was not set explicitly
     to t for this invocation, so set it to nil in this minibuffer.
     Save the old value now, before we change it.  */
  specbind (Qminibuffer_completing_file_name (),
	    Vminibuffer_completing_file_name);
  if EQ (Vminibuffer_completing_file_name, Qlambda ())
    { Vminibuffer_completing_file_name = Qnil (); }

  /* C 607-610's `#ifdef HAVE_WINDOW_SYSTEM` arm (`if (display_hourglass_p)
     cancel_hourglass ();`) is dead in the frozen configuration (macros.tsv 1647) and is
     recorded in docs/ports/minibuf.md (P-08).  */

  if !NILP (initial)
    {
      if CONSP (initial)
	{
	  let backup_n: Lisp_Object = XCDR (initial);
	  initial = XCAR (initial);
	  CHECK_STRING (initial);
	  if !NILP (backup_n)
	    {
	      CHECK_FIXNUM (backup_n);
	      /* Convert to distance from end of input.  */
	      if XFIXNUM (backup_n) < 1
		/* A number too small means the beginning of the string.  */
		{ pos = - SCHARS (initial); }
	      else
		{ pos = XFIXNUM (backup_n) - 1 - SCHARS (initial); }
	    }
	}
      else
	{ CHECK_STRING (initial); }
    }
  val = Qnil ();
  ambient_dir = BVAR! (current_buffer, directory_);
  input_method = Qnil ();
  enable_multibyte = Qnil ();

  if !STRINGP (prompt)
    { prompt = empty_unibyte_string; }

  if !enable_recursive_minibuffers
      && minibuf_level > 0
    {
      let str: Lisp_Object
	= build_string (c"Command attempted to use minibuffer while in minibuffer".as_ptr ());
      if EQ (selected_window, minibuf_window)
	{ Fsignal (Quser_error (), list1 (str)); }
      else
	/* If we're in another window, cancel the minibuffer that's active.  */
	{ Fthrow (Qexit (), str); }
    }

  if (noninteractive
       /* In case we are running as a daemon, only do this before
	  detaching from the terminal.  */
       || (IS_DAEMON () && DAEMON_RUNNING ()))
      && NILP (Vexecuting_kbd_macro)
    {
      val = read_minibuf_noninteractive (prompt, expflag, defalt);
      return unbind_to (count, val);
    }

  /* Ensure now that the latest minibuffer has been created and pushed
     onto Vminibuffer_list before incrementing minibuf_level, in case
     a hook called during the minibuffer creation calls
     Factive_minibuffer_window.  */
  minibuffer = get_minibuffer (minibuf_level + 1);
  minibuf_level += 1;		/* Before calling choose_minibuf_frame.  */

  /* Choose the minibuffer window and frame, and take action on them.  */

  /* Prepare for restoring the current buffer since choose_minibuf_frame
     calling Fset_frame_selected_window may change it (Bug#12766).  Make
     sure the current buffer is still live at that time (Bug#81828).  */
  record_unwind_current_buffer ();

  choose_minibuf_frame ();
  mini_frame = WINDOW_FRAME! (XWINDOW (minibuf_window));

  if minibuf_level > 1
      && WINDOW_LIVE_P ((*XFRAME (MB_frame)).minibuffer_window)
      && !EQ ((*XWINDOW ((*XFRAME (selected_frame)).minibuffer_window)).frame,
	      MB_frame)
      && minibuf_moves_frame_when_opened ()
      && (!minibuf_follows_frame ())
    {
      let of: *mut frame = XFRAME (MB_frame);

      zip_minibuffer_stacks (minibuf_window, (*of).minibuffer_window);
      /* MB_frame's minibuffer can be on a different frame.  */
      if MINI_WINDOW_P (XWINDOW (FRAME_SELECTED_WINDOW (of)))
	{
	  Fset_frame_selected_window (MB_frame,
				      Fframe_first_window (MB_frame), Qnil ());
	}
    }
  MB_frame = (*XWINDOW ((*XFRAME (selected_frame)).minibuffer_window)).frame;

  calln! (Qrecord_window_buffer (), minibuf_window);

  record_unwind_protect_void (Some (minibuffer_unwind));
  if read_minibuffer_restore_windows
    {
      record_unwind_protect (Some (restore_window_configuration),
			     list3 (Fcurrent_window_configuration (Qnil ()), Qt (), Qt ()));

      /* If the minibuffer window is on a different frame, save that
	 frame's configuration too.  */
      if !EQ (mini_frame, selected_frame)
	{
	  record_unwind_protect (Some (restore_window_configuration),
				 list3 (Fcurrent_window_configuration (mini_frame), Qnil (), Qt ()));
	}
    }
  else if !EQ (mini_frame, selected_frame)
    {
      record_unwind_protect
	(Some (restore_focus_frame),
	 Fcons (selected_frame, (*XFRAME (selected_frame)).focus_frame));
    }

  /* If the minibuffer is on an iconified or invisible frame,
     make it visible now.  */
  Fmake_frame_visible (mini_frame);

  if minibuffer_auto_raise
    { Fraise_frame (mini_frame); }

  if !multiple_terminals_merge_keyboards
    { temporarily_switch_to_single_kboard (XFRAME (mini_frame) as *mut c_void); }

  /* We have to do this after saving the window configuration
     since that is what restores the current buffer.  */

  /* Arrange to restore a number of minibuffer-related variables.
     We could bind each variable separately, but that would use lots of
     specpdl slots.  */
  minibuf_save_list
    = Fcons (Voverriding_local_map,
	     Fcons (minibuf_window,
		    Fcons (calling_frame,
			   Fcons (calling_window,
				  minibuf_save_list))));
  minibuf_save_list
    = Fcons (minibuf_prompt,
	     Fcons (make_fixnum (minibuf_prompt_width),
		    Fcons (Vhelp_form,
			   Fcons (Vcurrent_prefix_arg,
				  Fcons (Vminibuffer_history_position,
					 Fcons (Vminibuffer_history_variable,
						minibuf_save_list))))));
  minibuf_save_list
    = Fcons (Fthis_command_keys_vector (), minibuf_save_list);

  record_unwind_protect_void (Some (read_minibuf_unwind));
  /* We are exiting the minibuffer one way or the other, so run the hook.
     It should be run before unwinding the minibuf settings.  Do it
     separately from read_minibuf_unwind because we need to make sure that
     read_minibuf_unwind is fully executed even if exit-minibuffer-hook
     signals an error.  --Stef  */
  record_unwind_protect (Some (run_exit_minibuf_hook), minibuffer);

  /* Now that we can restore all those variables, start changing them.  */

  minibuf_prompt_width = 0;
  minibuf_prompt = Fcopy_sequence (prompt);
  Vminibuffer_history_position = histpos;
  Vminibuffer_history_variable = histvar;
  Vhelp_form = Vminibuffer_help_form;
  /* If this minibuffer is reading a file name, that doesn't mean
     recursive ones are.  But we cannot set it to nil, because
     completion code still need to know the minibuffer is completing a
     file name.  So use `lambda' as intermediate value meaning
     "t" in this minibuffer, but "nil" in next minibuffer.  */
  if !NILP (Vminibuffer_completing_file_name)
    { Vminibuffer_completing_file_name = Qlambda (); }

  /* If variable is unbound, make it nil.  */
  histval = find_symbol_value (histvar);
  if BASE_EQ (histval, Qunbound ())
    {
      Fset (histvar, Qnil ());
      histval = Qnil ();
    }

  if inherit_input_method
    {
      /* `current-input-method' is buffer local.  So, remember it in
	 INPUT_METHOD before changing the current buffer.  */
      input_method = Fsymbol_value (Qcurrent_input_method ());
      enable_multibyte = BVAR! (current_buffer, enable_multibyte_characters_);
    }

  /* Switch to the minibuffer.  */

  set_minibuffer_mode (minibuffer, minibuf_level);
  Fset_buffer (minibuffer);

  /* Defeat (setq-default truncate-lines t), since truncated lines do
     not work correctly in minibuffers.  (Bug#5715, etc)  */
  bset_truncate_lines (current_buffer, Qnil ());

  /* The current buffer's default directory is usually the right thing
     for our minibuffer here.  However, if you're typing a command at
     a minibuffer-only frame when minibuf_level is zero, then buf IS
     the current_buffer, so reset_buffer leaves buf's default
     directory unchanged.  This is a bummer when you've just started
     up Emacs and buf's default directory is Qnil.  Here's a hack; can
     you think of something better to do?  Find another buffer with a
     better directory, and use that one instead.  */
  if STRINGP (ambient_dir)
    { bset_directory (current_buffer, ambient_dir); }
  else
    {
      let mut tail: Lisp_Object = Qnil ();   /* PORTING.md 1.5.  */
      let mut buf: Lisp_Object = Qnil ();

      /* C's `FOR_EACH_LIVE_BUFFER (tail, buf)` is the port's open P-22 loop macro (the
	 buffer.rs mirror records it in docs/ports/excluded-macros.txt), so the port writes
	 FOR_EACH_ALIST_VALUE's own loop with C's `tail`/`buf` names, the same way
	 unlock_all_files and store_symval_forwarding do.  */
      tail = Vbuffer_alist;
      while CONSP (tail) && { buf = XCAR (tail); true }
	{
	  if STRINGP (BVAR! (XBUFFER (buf), directory_))
	    {
	      bset_directory (current_buffer,
			      BVAR! (XBUFFER (buf), directory_));
	      break;
	    }
	  tail = XCDR (tail);
	}
    }

  if !EQ (mini_frame, selected_frame)
    { Fredirect_frame_focus (selected_frame, mini_frame); }

  Vminibuf_scroll_window = selected_window;
  if minibuf_level == 1 || !EQ (minibuf_window, selected_window)
    { minibuf_selected_window = selected_window; }

  /* Empty out the minibuffers of all frames, except those frames
     where there is an active minibuffer.
     Set them to point to ` *Minibuf-0*', which is always empty.  */
  empty_minibuf = nth_minibuffer (0);
  set_minibuffer_mode (empty_minibuf, 0);

  /* Display this minibuffer in the proper window.  */
  /* Use set_window_buffer instead of Fset_window_buffer (see
     discussion of bug#11984, bug#12025, bug#12026).  */
  set_window_buffer (minibuf_window, Fcurrent_buffer (), false, false);
  Fselect_window (minibuf_window, Qnil ());
  (*XWINDOW (minibuf_window)).hscroll = 0;
  (*XWINDOW (minibuf_window)).bits &= ! (1 << WINDOW_SUSPEND_AUTO_HSCROLL_SHIFT);

  /* Erase the buffer.  */
  {
    let count1: specpdl_ref = SPECPDL_INDEX ();
    specbind (Qinhibit_read_only (), Qt ());
    specbind (Qinhibit_modification_hooks (), Qt ());
    Ferase_buffer ();

    /* If appropriate, copy enable-multibyte-characters into the minibuffer.
       In any case don't blindly inherit the multibyteness used previously.  */
    bset_enable_multibyte_characters (current_buffer,
                                      if inherit_input_method { enable_multibyte }
                                      else { Qt () });

    /* Insert the prompt, record where it ends.  */
    Finsert (1, &raw mut minibuf_prompt);
    if PT! () > BEG as ptrdiff_t
      {
	Fput_text_property (make_fixnum (BEG as EMACS_INT), make_fixnum (PT! ()),
			    Qfront_sticky (), Qt (), Qnil ());
	Fput_text_property (make_fixnum (BEG as EMACS_INT), make_fixnum (PT! ()),
			    Qrear_nonsticky (), Qt (), Qnil ());
	Fput_text_property (make_fixnum (BEG as EMACS_INT), make_fixnum (PT! ()),
			    Qfield (), Qt (), Qnil ());
	if CONSP (Vminibuffer_prompt_properties)
	  {
	    /* We want to apply all properties from
	       `minibuffer-prompt-properties' to the region normally,
	       but if the `face' property is present, add that
	       property to the end of the face properties to avoid
	       overwriting faces. */
	    let mut list: Lisp_Object = Vminibuffer_prompt_properties;
	    while CONSP (list)
	      {
		let key: Lisp_Object = XCAR (list);
		list = XCDR (list);
		if CONSP (list)
		  {
		    let val: Lisp_Object = XCAR (list);
		    list = XCDR (list);
		    if EQ (key, Qface ())
		      {
			Fadd_face_text_property (make_fixnum (BEG as EMACS_INT),
						 make_fixnum (PT! ()), val, Qt (), Qnil ());
		      }
		    else
		      {
			Fput_text_property (make_fixnum (BEG as EMACS_INT), make_fixnum (PT! ()),
					    key, val, Qnil ());
		      }
		  }
	      }
	  }
      }
    unbind_to (count1, Qnil ());
  }

  minibuf_prompt_width = current_column ();

  /* Put in the initial input.  */
  if !NILP (initial)
    {
      Finsert (1, &raw mut initial);
      Fforward_char (make_fixnum (pos));
    }

  clear_message (true, true);
  bset_keymap (current_buffer, map);

  /* Turn on an input method stored in INPUT_METHOD if any.  */
  if STRINGP (input_method) && !NILP (Ffboundp (Qactivate_input_method ()))
    { calln! (Qactivate_input_method (), input_method); }

  run_hook (Qminibuffer_setup_hook ());

  /* Don't allow the user to undo past this point.  */
  bset_undo_list (current_buffer, Qnil ());

  recursive_edit_1 ();

  /* If cursor is on the minibuffer line,
     show the user we have exited by putting it in column 0.  */
  if (*XWINDOW (minibuf_window)).cursor.vpos >= 0
      && !noninteractive
      && !FRAME_INITIAL_P (SELECTED_FRAME ())
    {
      (*XWINDOW (minibuf_window)).cursor.hpos = 0;
      (*XWINDOW (minibuf_window)).cursor.x = 0;
      (*XWINDOW (minibuf_window)).bits |= 1 << WINDOW_MUST_BE_UPDATED_P_SHIFT;
      let sf: *mut frame = XFRAME (selected_frame);
      update_frame (sf, true);
      if is_tty_frame (sf)
	{ combine_updates_for_frame (sf, true); }

      /* C's `#ifndef HAVE_NTGUI` arm is the live one (`HAVE_NTGUI` is undef, macros.tsv
	 1451); its `#else` arm (`w32_flip_buffers_if_dirty`) is dead and is recorded in
	 docs/ports/minibuf.md (P-08).  */
      flush_frame (XFRAME ((*XWINDOW (minibuf_window)).frame));
    }

  /* Make minibuffer contents into a string.  */
  Fset_buffer (minibuffer);
  if allow_props || minibuffer_allow_text_properties
    { val = Fminibuffer_contents (); }
  else
    { val = Fminibuffer_contents_no_properties (); }

  /* VAL is the string of minibuffer text.  */

  last_minibuf_string = val;

  /* Choose the string to add to the history.  */
  if SCHARS (val) != 0
    { histstring = val; }
  else if STRINGP (defalt)
    { histstring = defalt; }
  else if CONSP (defalt) && STRINGP (XCAR (defalt))
    { histstring = XCAR (defalt); }
  else
    { histstring = Qnil (); }

  /* The appropriate frame will get selected
     in set-window-configuration.  */
  unbind_to (count, Qnil ());

  /* Switch the frame back to the calling frame.  */
  if FRAMEP (calling_frame)
      && FRAME_LIVE_P (XFRAME (calling_frame))
      && (!EQ (selected_frame, calling_frame)
	  || (WINDOW_LIVE_P ((*XFRAME (calling_frame)).minibuffer_window)
	      && !EQ ((*XWINDOW ((*XFRAME (calling_frame)).minibuffer_window)).frame,
		      calling_frame)))
    { calln! (Qselect_frame_set_input_focus (), calling_frame, Qnil ()); }

  /* Add the value to the appropriate history list, if any.  This is
     done after the previous buffer has been made current again, in
     case the history variable is buffer-local.  */
  if ! (NILP (Vhistory_add_new_input) || NILP (histstring))
    { calln! (Qadd_to_history (), histvar, histstring); }

  /* If Lisp form desired instead of string, parse it.  */
  if expflag
    { val = string_to_object (val, defalt); }

  return val;
}

/* Return true if BUF is a particular existing minibuffer.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_minibuffer (depth: EMACS_INT, buf: Lisp_Object) -> bool
{
  let tail: Lisp_Object = Fnthcdr (make_fixnum (depth), Vminibuffer_list);
  return
    !NILP (tail)
    && EQ (Fcar (tail), buf);
}

/* Return the DEPTHth minibuffer, or nil if such does not yet exist.  */
unsafe fn nth_minibuffer (depth: EMACS_INT) -> Lisp_Object
{
  let tail: Lisp_Object = Fnthcdr (make_fixnum (depth), Vminibuffer_list);
  return Fcar (tail);
}

/* Set the major mode of the minibuffer BUF, depending on DEPTH, the
   minibuffer depth.  */

unsafe fn set_minibuffer_mode (buf: Lisp_Object, depth: EMACS_INT)
{
  let mut buf: Lisp_Object = buf;
  let count: specpdl_ref = SPECPDL_INDEX ();

  record_unwind_current_buffer ();
  Fset_buffer (buf);
  if depth > 0
    {
      if !NILP (Ffboundp (Qminibuffer_mode ()))
	{ call0 (Qminibuffer_mode ()); }
    }
  else
    {
      if !NILP (Ffboundp (Qminibuffer_inactive_mode ()))
	{ call0 (Qminibuffer_inactive_mode ()); }
      else
	{ Fkill_all_local_variables (Qnil ()); }
    }
  buf = unbind_to (count, buf);
}

/* Return a buffer to be used as the minibuffer at depth `depth'.
   depth = 0 is the lowest allowed argument, and that is the value
   used for nonrecursive minibuffer invocations.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_minibuffer (depth: EMACS_INT) -> Lisp_Object
{
  let mut tail: Lisp_Object = Fnthcdr (make_fixnum (depth), Vminibuffer_list);
  let mut cll_tail: Lisp_Object = Fnthcdr (make_fixnum (depth),
					   Vcommand_loop_level_list);
  if NILP (tail)
    {
      tail = list1 (Qnil ());
      Vminibuffer_list = nconc2 (Vminibuffer_list, tail);
      cll_tail = list1 (Qnil ());
      Vcommand_loop_level_list = nconc2 (Vcommand_loop_level_list, cll_tail);
    }
  XSETCAR (tail, make_fixnum (if depth != 0 { command_loop_level } else { 0 }));
  let mut buf: Lisp_Object = Fcar (tail);
  if NILP (buf) || !BUFFER_LIVE_P (XBUFFER (buf))
    {
      let name_fmt = c" *Minibuf-%ld*";
      let mut name: [c_char; NAME_BUF_SIZE] = [0; NAME_BUF_SIZE];
      AUTO_STRING_WITH_LEN! (lname, name.as_ptr (),
			     sprintf (name.as_mut_ptr (), name_fmt.as_ptr (), depth) as ptrdiff_t);
      buf = Fget_buffer_create (lname, Qnil ());
      /* Do this before set_minibuffer_mode.  */
      XSETCAR (tail, buf);
      /* Although the buffer's name starts with a space, undo should be
	 enabled in it.  */
      Fbuffer_enable_undo (buf);
    }
  else
    {
      /* We have to empty both overlay lists.  Otherwise we end
	 up with overlays that think they belong to this buffer
	 while the buffer doesn't know about them any more.  */
      delete_all_overlays (XBUFFER (buf));
      reset_buffer (XBUFFER (buf));
    }

  return buf;
}

/* gnulib's INT_STRLEN_BOUND (intprops.h) is `INT_BITS_STRLEN_BOUND (TYPE_WIDTH (t) -
   signed) + signed` with `INT_BITS_STRLEN_BOUND (b)` = `(b * 146 + 484) / 485`; for the
   signed 64-bit EMACS_INT it is 20, and C's `char name[sizeof name_fmt +
   INT_STRLEN_BOUND (EMACS_INT)]` is therefore 15 + 20 (the json.rs/data.rs shape, P-26).
   The macro has no generic mirror, so this is its one instantiation here.  */
const NAME_BUF_SIZE: usize = 15 + ((EMACS_INT_WIDTH - 1) * 146 + 484) / 485 + 1;

unsafe fn minibuf_c_loop_level (depth: EMACS_INT) -> EMACS_INT
{
  let cll: Lisp_Object = Fnth (make_fixnum (depth), Vcommand_loop_level_list);
  if FIXNUMP (cll)
    { return XFIXNUM (cll); }
  return 0;
}

unsafe extern "C" fn run_exit_minibuf_hook (minibuf: Lisp_Object)
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_current_buffer ();
  if BUFFER_LIVE_P (XBUFFER (minibuf))
    { Fset_buffer (minibuf); }
  safe_run_hooks (Qminibuffer_exit_hook ());
  unbind_to (count, Qnil ());
}

/* This variable records the expired minibuffer's frame between the
   calls of `read_minibuf_unwind' and `minibuffer_unwind'.  It should
   be used only by these two functions.  Note that the same search
   method for the MB's frame won't always work in `minibuffer_unwind'
   because the intervening `restore-window-configuration' will have
   changed the buffer in the mini-window.  */
static mut exp_MB_frame: Lisp_Object = lisp_h_Qnil ();

/* This function is called on exiting minibuffer, whether normally or
   not, and it restores the current window, buffer, etc.  */

unsafe extern "C" fn read_minibuf_unwind ()
{
  let mut old_deactivate_mark: Lisp_Object;
  let mut calling_frame: Lisp_Object;
  let mut calling_window: Lisp_Object;
  let mut future_mini_window: Lisp_Object;
  let saved_selected_frame: Lisp_Object = selected_frame;
  let mut window: Lisp_Object;
  let mut frames: Lisp_Object = Qnil ();   /* PORTING.md 1.5 (`Lisp_Object ... frames;`).  */
  let expired_MB: Lisp_Object = nth_minibuffer (minibuf_level);
  let mut w: *mut window;
  let mut f: *mut frame;

  if NILP (expired_MB)
    { emacs_abort (); }

  /* Locate the expired minibuffer.  */
  /* C's `goto found` is the labeled block below (PORTING.md 2.2).  */
  'found: {
    FOR_EACH_FRAME! (frames, exp_MB_frame, {
	f = XFRAME (exp_MB_frame);
	window = (*f).minibuffer_window;
	if WINDOW_LIVE_P (window)
	  {
	    w = XWINDOW (window);
	    if EQ ((*w).frame, exp_MB_frame)
		&& EQ ((*w).contents, expired_MB)
	      { break 'found; }
	  }
    });
    exp_MB_frame = Qnil ();		/* "Can't happen." */
  }

  if !EQ (exp_MB_frame, saved_selected_frame)
      && !NILP (exp_MB_frame)
    { do_switch_frame (exp_MB_frame, 0, 0, Qt ()); }	/* This also sets
							     minibuf_window */

  /* To keep things predictable, in case it matters, let's be in the
     minibuffer when we reset the relevant variables.  Don't depend on
     `minibuf_window' here.  This could by now be the mini-window of any
     frame.  */
  Fset_buffer (expired_MB);
  minibuf_level -= 1;

  /* Restore prompt, etc, from outer minibuffer level.  */
  let key_vec: Lisp_Object = Fcar (minibuf_save_list);
  this_command_key_count = ASIZE (key_vec);
  this_command_keys = key_vec;
  minibuf_save_list = Fcdr (minibuf_save_list);
  minibuf_prompt = Fcar (minibuf_save_list);
  minibuf_save_list = Fcdr (minibuf_save_list);
  minibuf_prompt_width = XFIXNAT (Fcar (minibuf_save_list));
  minibuf_save_list = Fcdr (minibuf_save_list);
  Vhelp_form = Fcar (minibuf_save_list);
  minibuf_save_list = Fcdr (minibuf_save_list);
  Vcurrent_prefix_arg = Fcar (minibuf_save_list);
  minibuf_save_list = Fcdr (minibuf_save_list);
  Vminibuffer_history_position = Fcar (minibuf_save_list);
  minibuf_save_list = Fcdr (minibuf_save_list);
  Vminibuffer_history_variable = Fcar (minibuf_save_list);
  minibuf_save_list = Fcdr (minibuf_save_list);
  Voverriding_local_map = Fcar (minibuf_save_list);
  minibuf_save_list = Fcdr (minibuf_save_list);
  /* C 1173-1177's `#if 0` arm (the `temp`/`minibuf_window` restore) is dead text and is
     recorded in docs/ports/minibuf.md rather than transcribed (P-08).  */
  future_mini_window = Fcar (minibuf_save_list);
  minibuf_save_list = Fcdr (minibuf_save_list);
  calling_frame = Fcar (minibuf_save_list);
  minibuf_save_list = Fcdr (minibuf_save_list);
  calling_window = Fcar (minibuf_save_list);
  minibuf_save_list = Fcdr (minibuf_save_list);

  /* Erase the minibuffer we were using at this level.  */
  {
    let count: specpdl_ref = SPECPDL_INDEX ();
    /* Prevent error in erase-buffer.  */
    specbind (Qinhibit_read_only (), Qt ());
    specbind (Qinhibit_modification_hooks (), Qt ());
    old_deactivate_mark = Vdeactivate_mark;
    Ferase_buffer ();
    Vdeactivate_mark = old_deactivate_mark;
    unbind_to (count, Qnil ());
  }

  /* When we get to the outmost level, make sure we resize the
     mini-window back to its normal size.  */
  if minibuf_level == 0
      || !EQ (selected_frame, WINDOW_FRAME! (XWINDOW (future_mini_window)))
    { resize_mini_window (XWINDOW (minibuf_window), false); }

  /* Deal with frames that should be removed when exiting the
     minibuffer.  */
  {
    let mut frames1: Lisp_Object = Qnil ();   /* PORTING.md 1.5.  */
    let mut frame1: Lisp_Object = Qnil ();
    let mut val: Lisp_Object = Qnil ();
    let mut f1: *mut frame;

    FOR_EACH_FRAME! (frames1, frame1, {
	f1 = XFRAME (frame1);

	if (FRAME_PARENT_FRAME (f1) != std::ptr::null_mut ()
	    || !NILP (get_frame_param (f1, Qdelete_before ())))
	  {
	    val = get_frame_param (f1, Qminibuffer_exit ());
	    if !NILP (val)
	      {
		if EQ (val, Qiconify_frame ())
		  { Ficonify_frame (frame1); }
		else if EQ (val, Qdelete_frame ())
		  { Fdelete_frame (frame1, Qnil ()); }
		else
		  { Fmake_frame_invisible (frame1, Qnil ()); }
	      }
	  }
    });
  }

  /* In case the previous minibuffer displayed in this miniwindow is
     dead, we may keep displaying this buffer (tho it's inactive), so reset it,
     to make sure we don't leave around bindings and stuff which only
     made sense during the read_minibuf invocation.  */
  call0 (Qminibuffer_inactive_mode ());

  /* We've exited the recursive edit, so switch the current windows
     away from the expired minibuffer window, both in the current
     minibuffer's frame and the original calling frame.  */
  choose_minibuf_frame ();
  if NILP ((*XWINDOW (minibuf_window)).prev_buffers)
    {
      if !EQ (WINDOW_FRAME! (XWINDOW (minibuf_window)), calling_frame)
	{
	  let prev: Lisp_Object = Fprevious_window (minibuf_window, Qnil (), Qnil ());
	  /* PREV can be on a different frame when we have a minibuffer only
	     frame, the other frame's minibuffer window is MINIBUF_WINDOW,
	     and its "focus window" is also MINIBUF_WINDOW.  */
	  if !EQ (prev, minibuf_window)
	      && EQ (WINDOW_FRAME! (XWINDOW (prev)),
		     WINDOW_FRAME! (XWINDOW (minibuf_window)))
	    { Fset_frame_selected_window (selected_frame, prev, Qnil ()); }
	}
      else if WINDOW_LIVE_P (calling_window)
	{ Fset_frame_selected_window (calling_frame, calling_window, Qnil ()); }
    }

  /* Restore the selected frame. */
  if !EQ (exp_MB_frame, saved_selected_frame)
      && !NILP (exp_MB_frame)
    { do_switch_frame (saved_selected_frame, 0, 0, Qt ()); }
}

/* Replace the expired minibuffer in frame exp_MB_frame with the next less
   nested minibuffer in that frame, if any.  Otherwise, replace it
   with the null minibuffer.  MINIBUF_WINDOW is not changed.  */
unsafe extern "C" fn minibuffer_unwind ()
{
  let mut f: *mut frame;

  if NILP (exp_MB_frame) { return; } /* "Can't happen." */
  f = XFRAME (exp_MB_frame);
  if FRAME_LIVE_P (f)
    {
      let window: Lisp_Object = (*f).minibuffer_window;

      if WINDOW_LIVE_P (window)
	{
	  let w: *mut window = XWINDOW (window);

	  /* minibuf_window = sf->minibuffer_window; */
	  if !NILP ((*w).prev_buffers)
	    {
	      let entry: Lisp_Object = Fcar ((*w).prev_buffers);

	      if BUFFERP (Fcar (entry))
		  && BUFFER_LIVE_P (XBUFFER (Fcar (entry)))
		{
		  wset_prev_buffers (w, Fcdr ((*w).prev_buffers));
		  set_window_buffer (window, Fcar (entry), false, false);
		  Fset_window_start (window, Fcar (Fcdr (entry)), Qnil ());
		  Fset_window_point (window, Fcar (Fcdr (Fcdr (entry))));
		}
	      else
		{ set_window_buffer (window, nth_minibuffer (0), false, false); }
	    }
	  else
	    { set_window_buffer (window, nth_minibuffer (0), false, false); }
	}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn barf_if_interaction_inhibited ()
{
  if inhibit_interaction
    { xsignal0 (Qinhibited_interaction ()); }
}

DEFUN! ("read-from-minibuffer", Fread_from_minibuffer,
       Sread_from_minibuffer, 1, 7, 0,
       doc: "Read and return a string from the minibuffer, prompting with string PROMPT.\nThe optional second arg INITIAL-CONTENTS is an obsolete alternative to\n  DEFAULT-VALUE.  It normally should be nil in new code, except when\n  HIST is a cons.  It is discussed in more detail below.\n\nThird arg KEYMAP is a keymap to use whilst reading;\n  if omitted or nil, the default is `minibuffer-local-map'.\n\nIf fourth arg READ is non-nil, interpret the result as a Lisp object\n  and return that object:\n  in other words, do `(car (read-from-string INPUT-STRING))'\n\nFifth arg HIST, if non-nil, specifies a history list and optionally\n  the initial position in the list.  It can be a symbol, which is the\n  history list variable to use, or a cons cell (HISTVAR . HISTPOS).\n  In that case, HISTVAR is the history list variable to use, and\n  HISTPOS is the initial position for use by the minibuffer history\n  commands.  For consistency, you should also specify that element of\n  the history as the value of INITIAL-CONTENTS.  Positions are counted\n  starting from 1 at the beginning of the list.  If HIST is nil, the\n  default history list `minibuffer-history' is used.  If HIST is t,\n  history is not recorded.\n\n  If `history-add-new-input' is non-nil (the default), the result will\n  be added to the history list using `add-to-history'.\n\nSixth arg DEFAULT-VALUE, if non-nil, should be a string, which is used\n  as the default to `read' if READ is non-nil and the user enters\n  empty input.  But if READ is nil, this function does _not_ return\n  DEFAULT-VALUE for empty input!  Instead, it returns the empty string.\n\n  Whatever the value of READ, DEFAULT-VALUE is made available via the\n  minibuffer history commands.  DEFAULT-VALUE can also be a list of\n  strings, in which case all the strings are available in the history,\n  and the first string is the default to `read' if READ is non-nil.\n\nSeventh arg INHERIT-INPUT-METHOD, if non-nil, means the minibuffer inherits\n the current input method and the setting of `enable-multibyte-characters'.\n\nIf the variable `minibuffer-allow-text-properties' is non-nil\n (either let-bound or buffer-local in the minibuffer),\n then the string which is returned includes whatever text properties\n were present in the minibuffer.  Otherwise the value has no text properties.\n\nIf `inhibit-interaction' is non-nil, this function will signal an\n  `inhibited-interaction' error.\n\nThe remainder of this documentation string describes the\nINITIAL-CONTENTS argument in more detail.  It is only relevant when\nstudying existing code, or when HIST is a cons.  If non-nil,\nINITIAL-CONTENTS is a string to be inserted into the minibuffer before\nreading input.  Normally, point is put at the end of that string.\nHowever, if INITIAL-CONTENTS is (STRING . POSITION), the initial\ninput is STRING, but point is placed at _one-indexed_ position\nPOSITION in the minibuffer.  Any integer value less than or equal to\none puts point at the beginning of the string.  *Note* that this\nbehavior differs from the way such arguments are used in `completing-read'\nand some related functions, which use zero-indexing for POSITION.",
       (prompt: Lisp_Object, initial_contents: Lisp_Object, keymap: Lisp_Object,
	read: Lisp_Object, hist: Lisp_Object, default_value: Lisp_Object,
	inherit_input_method: Lisp_Object),
       {
  let mut keymap: Lisp_Object = keymap;
  let mut histvar: Lisp_Object;
  let mut histpos: Lisp_Object;
  let val: Lisp_Object;

  barf_if_interaction_inhibited ();

  CHECK_STRING (prompt);
  if NILP (keymap)
    { keymap = Vminibuffer_local_map; }
  else
    { keymap = get_keymap (keymap, true, false); }

  if SYMBOLP (hist)
    {
      histvar = hist;
      histpos = Qnil ();
    }
  else
    {
      histvar = Fcar_safe (hist);
      histpos = Fcdr_safe (hist);
    }
  if NILP (histvar)
    { histvar = Qminibuffer_history (); }
  if NILP (histpos)
    { XSETFASTINT! (histpos, 0); }

  /* C 1396-1408's `#ifdef HAVE_TEXT_CONVERSION` arm (the
     `overriding-text-conversion-style` reset) is dead in the frozen configuration
     (macros.tsv 1614) and is recorded in docs/ports/minibuf.md (P-08).  */

  val = read_minibuf (keymap, initial_contents, prompt,
		      !NILP (read),
		      histvar, histpos, default_value,
		      minibuffer_allow_text_properties,
		      !NILP (inherit_input_method));
  return val;
});

DEFUN! ("read-string", Fread_string, Sread_string, 1, 5, 0,
       doc: "Read and return a string from the minibuffer, prompting with PROMPT.\n\nPROMPT is a string, which should normally end with the string \": \".\n\nIf non-nil, second arg INITIAL-INPUT is a string to insert before\nreading.  This argument has been superseded by DEFAULT-VALUE and should\nnormally be nil in new code.  It behaves as INITIAL-CONTENTS in\n`read-from-minibuffer' (which see).\n\nThe third arg HISTORY, if non-nil, specifies a history list and\noptionally the initial position in the list.\n\nSee `read-from-minibuffer' for details of HISTORY argument.\n\nFourth arg DEFAULT-VALUE is the default value or the list of default\nvalues.  If non-nil, it is used for history commands, and as the value\n(or the first element of the list of default values) to return if the\nuser enters the empty string.\n\nFifth arg INHERIT-INPUT-METHOD, if non-nil, means the minibuffer\ninherits the current input method and the setting of\n`enable-multibyte-characters'.",
       (prompt: Lisp_Object, initial_input: Lisp_Object, history: Lisp_Object,
	default_value: Lisp_Object, inherit_input_method: Lisp_Object),
       {
  let mut val: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();

  /* Just in case we're in a recursive minibuffer, make it clear that the
     previous minibuffer's completion table does not apply to the new
     minibuffer.
     FIXME: `minibuffer-completion-table' should be buffer-local instead.  */
  specbind (Qminibuffer_completion_table (), Qnil ());

  val = Fread_from_minibuffer (prompt, initial_input, Qnil (),
			       Qnil (), history, default_value,
			       inherit_input_method);
  if STRINGP (val) && SCHARS (val) == 0 && ! NILP (default_value)
    { val = if CONSP (default_value) { XCAR (default_value) } else { default_value }; }
  return unbind_to (count, val);
});

DEFUN! ("read-command", Fread_command, Sread_command, 1, 2, 0,
       doc: "Read the name of a command and return it as a symbol.\nPrompt with PROMPT.  By default, return DEFAULT-VALUE or its first element\nif it is a list.  If DEFAULT-VALUE is omitted or nil, and the user enters\nnull input, return a symbol whose name is an empty string.",
       (prompt: Lisp_Object, default_value: Lisp_Object),
       {
  let name: Lisp_Object;
  let mut default_string: Lisp_Object;

  if NILP (default_value)
    { default_string = Qnil (); }
  else if SYMBOLP (default_value)
    { default_string = SYMBOL_NAME (default_value); }
  else
    { default_string = default_value; }

  name = Fcompleting_read (prompt, Vobarray, Qcommandp (), Qt (),
			   Qnil (), Qnil (), default_string, Qnil ());
  if NILP (name)
    { return name; }
  return Fintern (name, Qnil ());
});

DEFUN! ("read-variable", Fread_variable, Sread_variable, 1, 2, 0,
       doc: "Read the name of a user option and return it as a symbol.\nPrompt with PROMPT.  By default, return DEFAULT-VALUE or its first element\nif it is a list of strings.\nA user option, or customizable variable, is one for which\n`custom-variable-p' returns non-nil.",
       (prompt: Lisp_Object, default_value: Lisp_Object),
       {
  let mut name: Lisp_Object;
  let mut default_string: Lisp_Object;

  if NILP (default_value)
    { default_string = Qnil (); }
  else if SYMBOLP (default_value)
    { default_string = SYMBOL_NAME (default_value); }
  else
    { default_string = default_value; }

  name = Fcompleting_read (prompt, Vobarray,
			   Qcustom_variable_p (), Qt (),
			   Qnil (), Qcustom_variable_history (),
			   default_string, Qnil ());
  if NILP (name)
    { return name; }
  return Fintern (name, Qnil ());
});

DEFUN! ("read-buffer", Fread_buffer, Sread_buffer, 1, 4, 0,
       doc: "Read the name of a buffer and return it as a string.\nPrompt with PROMPT, which should be a string ending with a colon and a space.\nProvides completion on buffer names the user types.\nOptional second arg DEF is value to return if user enters an empty line,\n instead of that empty string.\n If DEF is a list of default values, return its first element.\nOptional third arg REQUIRE-MATCH has the same meaning as the\n REQUIRE-MATCH argument of `completing-read'.\nOptional arg PREDICATE, if non-nil, is a function limiting the buffers that\ncan be considered.  It will be called with each potential candidate, in\nthe form of either a string or a cons cell whose `car' is a string, and\nshould return non-nil to accept the candidate for completion, nil otherwise.\nIf `read-buffer-completion-ignore-case' is non-nil, completion ignores\ncase while reading the buffer name.\nIf `read-buffer-function' is non-nil, this works by calling it as a\nfunction, instead of the usual behavior.",
       (prompt: Lisp_Object, def: Lisp_Object, require_match: Lisp_Object,
	predicate: Lisp_Object),
       {
  let mut prompt: Lisp_Object = prompt;
  let mut def: Lisp_Object = def;
  let mut result: Lisp_Object;
  let mut s: *mut c_char = std::ptr::null_mut ();   /* PORTING.md 1.5: assigned below.  */
  let mut len: ptrdiff_t;
  let count: specpdl_ref = SPECPDL_INDEX ();

  if BUFFERP (def)
    { def = BVAR! (XBUFFER (def), name_); }

  specbind (Qcompletion_ignore_case (),
	    if read_buffer_completion_ignore_case { Qt () } else { Qnil () });

  if NILP (Vread_buffer_function)
    {
      if !NILP (def)
	{
	  /* A default value was provided: we must change PROMPT,
	     editing the default value in before the colon.  To achieve
	     this, we replace PROMPT with a substring that doesn't
	     contain the terminal space and colon (if present).  They
	     are then added back using Fformat.  */

	  if STRINGP (prompt)
	    {
	      s = SSDATA (prompt);
	      len = SBYTES (prompt);
	      if len >= 2 && *s.offset (len - 2) == b':' as c_char
		  && *s.offset (len - 1) == b' ' as c_char
		{ len = len - 2; }
	      else if len >= 1 && (*s.offset (len - 1) == b':' as c_char
				   || *s.offset (len - 1) == b' ' as c_char)
		{ len -= 1; }

	      prompt = make_specified_string (s, -1, len,
					      STRING_MULTIBYTE (prompt));
	    }

	  prompt = calln! (Qformat_prompt (), prompt,
			   if CONSP (def) { XCAR (def) } else { def });
	}

      result = Fcompleting_read (prompt, Qinternal_complete_buffer (),
				 predicate, require_match, Qnil (),
				 Qbuffer_name_history (), def, Qnil ());
    }
  else
    {
      result = if NILP (predicate)
		 /* Partial backward compatibility for older read_buffer_functions
		    which don't expect a `predicate' argument.  */
		 { calln! (Vread_buffer_function, prompt, def, require_match) }
	       else
		 { calln! (Vread_buffer_function, prompt, def, require_match,
			   predicate) };
    }
  return unbind_to (count, result);
});

unsafe fn match_regexps (string: Lisp_Object, regexps: Lisp_Object,
			 ignore_case: bool) -> bool
{
  let mut regexps: Lisp_Object = regexps;
  let mut val: ptrdiff_t;
  while CONSP (regexps)
    {
      CHECK_STRING (XCAR (regexps));

      val = fast_string_match_internal
	(XCAR (regexps), string,
	 if ignore_case { BVAR! (current_buffer, case_canon_table_) } else { Qnil () });

      if val == -2
	{ error (c"Stack overflow in regexp matcher".as_ptr ()); }
      if val < 0
	{ return false; }
      regexps = XCDR (regexps);
    }
  return true;
}

/* C's `enum { function_table, list_table, obarray_table, hash_table}` is declared inside
   `Ftry_completion`; the port declares it at file scope (it is the only name it needs) and
   keeps C's variable spelling through the raw identifier `r#type`, which Rust's keyword
   would otherwise block.  */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum try_completion_type
  {
    function_table = 0,
    list_table,
    obarray_table,
    hash_table,
  }

DEFUN! ("try-completion", Ftry_completion, Stry_completion, 2, 3, 0,
       doc: "Return longest common substring of all completions of STRING in COLLECTION.\n\nTest each possible completion specified by COLLECTION\nto see if it begins with STRING.  The possible completions may be\nstrings or symbols.  Symbols are converted to strings before testing,\nby using `symbol-name'.\n\nIf no possible completions match, the function returns nil; if\nthere's just one exact match, it returns t; otherwise it returns\nthe longest initial substring common to all possible completions\nthat begin with STRING.\n\nIf COLLECTION is an alist, the keys (cars of elements) are the\npossible completions.  If an element is not a cons cell, then the\nelement itself is a possible completion.\nIf COLLECTION is a hash-table, all the keys that are either strings\nor symbols are the possible completions.\nIf COLLECTION is an obarray, the names of all symbols in the obarray\nare the possible completions.\n\nCOLLECTION can also be a function to do the completion itself.\nIt receives three arguments: STRING, PREDICATE and nil.\nWhatever it returns becomes the value of `try-completion'.\n\nIf optional third argument PREDICATE is non-nil, it must be a function\nof one or two arguments, and is used to test each possible completion.\nA possible completion is accepted only if PREDICATE returns non-nil.\n\nThe argument given to PREDICATE is either a string or a cons cell (whose\ncar is a string) from the alist, or a symbol from the obarray.\nIf COLLECTION is a hash-table, PREDICATE is called with two arguments:\nthe string key and the associated value.\n\nTo be acceptable, a possible completion must also match all the regexps\nin `completion-regexp-list' (unless COLLECTION is a function, in\nwhich case that function should itself handle `completion-regexp-list').\n\nIf `completion-ignore-case' is non-nil, possible completions are matched\nwhile ignoring letter-case, but no guarantee is made about the letter-case\nof the return value, except that it comes either from the user's input\nor from one of the possible completions.",
       (string: Lisp_Object, collection: Lisp_Object, predicate: Lisp_Object),
       {
  let mut collection: Lisp_Object = collection;

  let mut bestmatch: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut elt: Lisp_Object;
  let mut eltstring: Lisp_Object;
  /* Size in bytes of BESTMATCH.  */
  let mut bestmatchsize: ptrdiff_t = 0;
  /* These are in bytes, too.  */
  let mut compare: ptrdiff_t;
  let mut matchsize: ptrdiff_t;
  if VECTORP (collection)
    { collection = check_obarray (collection); }
  let r#type: try_completion_type = if HASH_TABLE_P (collection) { try_completion_type::hash_table }
    else if OBARRAYP (collection) { try_completion_type::obarray_table }
    else if NILP (collection)
	 || (CONSP (collection) && !FUNCTIONP (collection))
      { try_completion_type::list_table }
    else { try_completion_type::function_table };
  let mut idx: ptrdiff_t = 0;
  let mut matchcount: c_int = 0;
  let mut bucket: Lisp_Object;
  let mut zero: Lisp_Object;
  let mut end: Lisp_Object;
  let mut tem: Lisp_Object;

  CHECK_STRING (string);
  if r#type == try_completion_type::function_table
    { return calln! (collection, string, predicate, Qnil ()); }

  bestmatch = Qnil ();
  bucket = Qnil ();
  zero = make_fixnum (0);

  /* If COLLECTION is not a list, set TAIL just for gc pro.  */
  tail = collection;
  let mut obit: obarray_iter_t = std::mem::zeroed ();   /* PORTING.md 1.5.  */
  if r#type == try_completion_type::obarray_table
    { obit = make_obarray_iter (XOBARRAY (collection)); }

  while true
    {
      /* Get the next element of the alist, obarray, or hash-table.  */
      /* Exit the loop if the elements are all used up.  */
      /* elt gets the alist element or symbol.
	 eltstring gets the name to check as a completion.  */

      if r#type == try_completion_type::list_table
	{
	  if !CONSP (tail)
	    { break; }
	  elt = XCAR (tail);
	  eltstring = if CONSP (elt) { XCAR (elt) } else { elt };
	  tail = XCDR (tail);
	}
      else if r#type == try_completion_type::obarray_table
	{
	  if obarray_iter_at_end (&raw mut obit)
	    { break; }
	  elt = obarray_iter_symbol (&raw mut obit);
	  eltstring = elt;
	  obarray_iter_step (&raw mut obit);
	}
      else /* if (type == hash_table) */
	{
	  while idx < HASH_TABLE_SIZE (XHASH_TABLE (collection)) as ptrdiff_t
		 && hash_unused_entry_key_p (HASH_KEY (XHASH_TABLE (collection),
						       idx as hash_idx_t))
	    { idx += 1; }
	  if idx >= HASH_TABLE_SIZE (XHASH_TABLE (collection)) as ptrdiff_t
	    { break; }
	  else
	    {
	      elt = HASH_KEY (XHASH_TABLE (collection), idx as hash_idx_t);
	      eltstring = elt;
	      idx += 1;
	    }
	}

      /* Is this element a possible completion?  */

      if SYMBOLP (eltstring)
	{ eltstring = Fsymbol_name (eltstring); }

      if STRINGP (eltstring)
	  && SCHARS (string) <= SCHARS (eltstring)
	  && { tem = Fcompare_strings (eltstring, zero,
				       make_fixnum (SCHARS (string)),
				       string, zero, Qnil (),
				       if completion_ignore_case { Qt () } else { Qnil () });
	       EQ (Qt (), tem) }
	{
	  /* Ignore this element if it fails to match all the regexps.  */
	  if !match_regexps (eltstring, Vcompletion_regexp_list,
			     completion_ignore_case)
	    { continue; }

	  /* Ignore this element if there is a predicate
	     and the predicate doesn't like it.  */

	  if !NILP (predicate)
	    {
	      if EQ (predicate, Qcommandp ())
		{ tem = Fcommandp (elt, Qnil ()); }
	      else
		{
		  if r#type == try_completion_type::hash_table
		    {
		      tem = calln! (predicate, elt,
				    HASH_VALUE (XHASH_TABLE (collection),
						(idx - 1) as hash_idx_t));
		    }
		  else
		    { tem = calln! (predicate, elt); }
		}
	      if NILP (tem) { continue; }
	    }

	  /* Update computation of how much all possible completions match */

	  if NILP (bestmatch)
	    {
	      matchcount = 1;
	      bestmatch = eltstring;
	      bestmatchsize = SCHARS (eltstring);
	    }
	  else
	    {
	      compare = min (bestmatchsize, SCHARS (eltstring));
	      let lcompare: Lisp_Object = make_fixnum (compare);
	      tem = Fcompare_strings (bestmatch, zero, lcompare,
				      eltstring, zero, lcompare,
				      if completion_ignore_case { Qt () } else { Qnil () });
	      matchsize = if EQ (tem, Qt ()) { compare } else { eabs! (XFIXNUM (tem)) - 1 };

	      let old_bestmatch: Lisp_Object = bestmatch;
	      if completion_ignore_case
		{
		  /* If this is an exact match except for case,
		     use it as the best match rather than one that is not an
		     exact match.  This way, we get the case pattern
		     of the actual match.  */
		  if (matchsize == SCHARS (eltstring)
		      && matchsize < SCHARS (bestmatch))
		      ||
		      /* If there is more than one exact match ignoring case,
			 and one of them is exact including case,
			 prefer that one.  */
		      /* If there is no exact match ignoring case,
			 prefer a match that does not change the case
			 of the input.  */
		      ((matchsize == SCHARS (eltstring))
		       ==
		       (matchsize == SCHARS (bestmatch))
		       && { tem = Fcompare_strings (eltstring, zero,
						    make_fixnum (SCHARS (string)),
						    string, zero,
						    Qnil (),
						    Qnil ());
			    EQ (Qt (), tem) }
		       && { tem = Fcompare_strings (bestmatch, zero,
						    make_fixnum (SCHARS (string)),
						    string, zero,
						    Qnil (),
						    Qnil ());
			    ! EQ (Qt (), tem) })
		    { bestmatch = eltstring; }
		}
	      if bestmatchsize != SCHARS (eltstring)
		  || bestmatchsize != matchsize
		  || (completion_ignore_case
		      && !BASE_EQ (Fcompare_strings (old_bestmatch, zero,
						     lcompare, eltstring, zero,
						     lcompare, Qnil ()),
				   Qt ()))
		/* Don't count the same string multiple times.  */
		{ matchcount += if matchcount <= 1 { 1 } else { 0 }; }
	      bestmatchsize = matchsize;
	      if matchsize <= SCHARS (string)
		  /* If completion-ignore-case is non-nil, don't
		     short-circuit because we want to find the best
		     possible match *including* case differences.  */
		  && !completion_ignore_case
		  && matchcount > 1
		/* No need to look any further.  */
		{ break; }
	    }
	}
    }

  if NILP (bestmatch)
    { return Qnil (); }		/* No completions found.  */

  /* Return t if the supplied string is an exact match (counting case);
     it does not require any change to be made.  */
  if matchcount == 1 && !NILP (Fequal (bestmatch, string))
    { return Qt (); }

  XSETFASTINT! (zero, 0);		/* Else extract the part in which */
  XSETFASTINT! (end, bestmatchsize);	/* all completions agree.  */
  return Fsubstring (bestmatch, zero, end);
});

DEFUN! ("all-completions", Fall_completions, Sall_completions, 2, 3, 0,
       doc: "Search for partial matches of STRING in COLLECTION.\n\nTest each possible completion specified by COLLECTION\nto see if it begins with STRING.  The possible completions may be\nstrings or symbols.  Symbols are converted to strings before testing,\nby using `symbol-name'.\n\nThe value is a list of all the possible completions that match STRING.\n\nIf COLLECTION is an alist, the keys (cars of elements) are the\npossible completions.  If an element is not a cons cell, then the\nelement itself is the possible completion.\nIf COLLECTION is a hash-table, all the keys that are strings or symbols\nare the possible completions.\nIf COLLECTION is an obarray, the names of all symbols in the obarray\nare the possible completions.\n\nCOLLECTION can also be a function to do the completion itself.\nIt receives three arguments: STRING, PREDICATE and t.\nWhatever it returns becomes the value of `all-completions'.\n\nIf optional third argument PREDICATE is non-nil, it must be a function\nof one or two arguments, and is used to test each possible completion.\nA possible completion is accepted only if PREDICATE returns non-nil.\n\nThe argument given to PREDICATE is either a string or a cons cell (whose\ncar is a string) from the alist, or a symbol from the obarray.\nIf COLLECTION is a hash-table, PREDICATE is called with two arguments:\nthe string key and the associated value.\n\nTo be acceptable, a possible completion must also match all the regexps\nin `completion-regexp-list' (unless COLLECTION is a function, in\nwhich case that function should itself handle `completion-regexp-list').",
       (string: Lisp_Object, collection: Lisp_Object, predicate: Lisp_Object),
       {
  let mut collection: Lisp_Object = collection;
  let mut tail: Lisp_Object;
  let mut elt: Lisp_Object;
  let mut eltstring: Lisp_Object;
  let mut allmatches: Lisp_Object;
  if VECTORP (collection)
    { collection = check_obarray (collection); }
  let r#type: c_int = if HASH_TABLE_P (collection)
    { 3 } else if OBARRAYP (collection)
    { 2 } else if NILP (collection)
	 || (CONSP (collection) && !FUNCTIONP (collection))
    { 1 } else { 0 };
  let mut idx: ptrdiff_t = 0;
  let mut bucket: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut zero: Lisp_Object;

  CHECK_STRING (string);
  if r#type == 0
    { return calln! (collection, string, predicate, Qt ()); }
  allmatches = Qnil ();
  bucket = Qnil ();
  zero = make_fixnum (0);

  /* If COLLECTION is not a list, set TAIL just for gc pro.  */
  tail = collection;
  let mut obit: obarray_iter_t = std::mem::zeroed ();   /* PORTING.md 1.5.  */
  if r#type == 2
    { obit = make_obarray_iter (XOBARRAY (collection)); }

  while true
    {
      /* Get the next element of the alist, obarray, or hash-table.  */
      /* Exit the loop if the elements are all used up.  */
      /* elt gets the alist element or symbol.
	 eltstring gets the name to check as a completion.  */

      if r#type == 1
	{
	  if !CONSP (tail)
	    { break; }
	  elt = XCAR (tail);
	  eltstring = if CONSP (elt) { XCAR (elt) } else { elt };
	  tail = XCDR (tail);
	}
      else if r#type == 2
	{
	  if obarray_iter_at_end (&raw mut obit)
	    { break; }
	  elt = obarray_iter_symbol (&raw mut obit);
	  eltstring = elt;
	  obarray_iter_step (&raw mut obit);
	}
      else /* if (type == 3) */
	{
	  while idx < HASH_TABLE_SIZE (XHASH_TABLE (collection)) as ptrdiff_t
		 && hash_unused_entry_key_p (HASH_KEY (XHASH_TABLE (collection),
						       idx as hash_idx_t))
	    { idx += 1; }
	  if idx >= HASH_TABLE_SIZE (XHASH_TABLE (collection)) as ptrdiff_t
	    { break; }
	  else
	    {
	      elt = HASH_KEY (XHASH_TABLE (collection), idx as hash_idx_t);
	      eltstring = elt;
	      idx += 1;
	    }
	}

      /* Is this element a possible completion?  */

      if SYMBOLP (eltstring)
	{ eltstring = Fsymbol_name (eltstring); }

      if STRINGP (eltstring)
	  && SCHARS (string) <= SCHARS (eltstring)
	  && { tem = Fcompare_strings (eltstring, zero,
				       make_fixnum (SCHARS (string)),
				       string, zero,
				       make_fixnum (SCHARS (string)),
				       if completion_ignore_case { Qt () } else { Qnil () });
	       EQ (Qt (), tem) }
	{
	  /* Ignore this element if it fails to match all the regexps.  */
	  if !match_regexps (eltstring, Vcompletion_regexp_list,
			     completion_ignore_case)
	    { continue; }

	  /* Ignore this element if there is a predicate
	     and the predicate doesn't like it.  */

	  if !NILP (predicate)
	    {
	      if EQ (predicate, Qcommandp ())
		{ tem = Fcommandp (elt, Qnil ()); }
	      else
		{
		  if r#type == 3
		    {
		      tem = calln! (predicate, elt,
				    HASH_VALUE (XHASH_TABLE (collection),
						(idx - 1) as hash_idx_t));
		    }
		  else
		    { tem = calln! (predicate, elt); }
		}
	      if NILP (tem) { continue; }
	    }
	  /* Ok => put it on the list.  */
	  allmatches = Fcons (eltstring, allmatches);
	}
    }

  return Fnreverse (allmatches);
});

DEFUN! ("completing-read", Fcompleting_read, Scompleting_read, 2, 8, 0,
       doc: "Read a string in the minibuffer, with completion.\nWhile in the minibuffer, you can use \\<minibuffer-local-completion-map>\\[minibuffer-complete] and \\[minibuffer-complete-word] to complete your input.\nYou can also use \\<minibuffer-local-map>\\[minibuffer-complete-history] to complete using history items in the\ninput history HIST, and you can use \\[minibuffer-complete-defaults] to complete using\nthe default items in DEFAULT-VALUE.\n\nPROMPT is a string to prompt with; normally it ends in a colon and a space.\nCOLLECTION can be a list of strings, an alist, an obarray or a hash table.\nCOLLECTION can also be a function to do the completion itself.\nPREDICATE limits completion to a subset of COLLECTION.\nSee `try-completion', `all-completions', `test-completion',\nand `completion-boundaries', for more details on completion,\nCOLLECTION, and PREDICATE.  See also Info node `(elisp)Basic Completion'\nfor the details about completion, and Info node `(elisp)Programmed\nCompletion' for expectations from COLLECTION when it's a function.\n\nREQUIRE-MATCH can take the following values:\n- t means that the user is not allowed to exit unless the input is (or\n  completes to) an element of COLLECTION or is null.\n- nil means that the user can exit with any input.\n- `confirm' means that the user can exit with any input, but she needs\n  to confirm her choice if the input is not an element of COLLECTION.\n- `confirm-after-completion' means that the user can exit with any\n  input, but she needs to confirm her choice if she called\n  `minibuffer-complete' right before `minibuffer-complete-and-exit'\n  and the input is not an element of COLLECTION.\n- a function, which will be called with the input as the\n  argument.  If the function returns a non-nil value, the\n  minibuffer is exited with that argument as the value.\n- anything else behaves like t except that typing RET does not exit if it\n  does non-null completion.\n\nIf the input is null, `completing-read' returns DEF, or the first\nelement of the list of default values, or an empty string if DEF is\nnil, regardless of the value of REQUIRE-MATCH.\n\nIf INITIAL-INPUT is non-nil, insert it in the minibuffer initially,\n  with point positioned at the end.  If it is (STRING . POSITION), the\n  initial input is STRING, but point is placed at _zero-indexed_\n  position POSITION in STRING.  (*Note* that this is different from\n  `read-from-minibuffer' and related functions, which use one-indexing\n  for POSITION.)  This feature is deprecated--it is best to pass nil\n  for INITIAL-INPUT and supply the default value DEF instead.  The\n  user can yank the default value into the minibuffer easily using\n  \\<minibuffer-local-map>\\[next-history-element].\n\nHIST, if non-nil, specifies a history list and optionally the initial\n  position in the list.  It can be a symbol, which is the history list\n  variable to use, or it can be a cons cell (HISTVAR . HISTPOS).  In\n  that case, HISTVAR is the history list variable to use, and HISTPOS\n  is the initial position (the position in the list used by the\n  minibuffer history commands).  For consistency, you should also\n  specify that element of the history as the value of INITIAL-INPUT.\n  (This is the only case in which you should use INITIAL-INPUT instead\n  of DEF.)  Positions are counted starting from 1 at the beginning of\n  the list.  The variable `history-length' controls the maximum length\n  of a history list.  If HIST is t, history is not recorded.\n\nDEF, if non-nil, is the default value or the list of default values.\n\nIf INHERIT-INPUT-METHOD is non-nil, the minibuffer inherits the\n  current input method and the setting of `enable-multibyte-characters'.\n\nCompletion ignores case if the ambient value of\n  `completion-ignore-case' is non-nil.\n\nSee also `completing-read-function'.",
       (prompt: Lisp_Object, collection: Lisp_Object, predicate: Lisp_Object,
	require_match: Lisp_Object, initial_input: Lisp_Object, hist: Lisp_Object,
	def: Lisp_Object, inherit_input_method: Lisp_Object),
       {
  return calln! (Fsymbol_value (Qcompleting_read_function ()),
		 prompt, collection, predicate, require_match, initial_input,
		 hist, def, inherit_input_method);
});

/* Test whether TXT is an exact completion.  */
DEFUN! ("test-completion", Ftest_completion, Stest_completion, 2, 3, 0,
       doc: "Return non-nil if STRING is a valid completion.\nFor instance, if COLLECTION is a list of strings, STRING is a\nvalid completion if it appears in the list and PREDICATE is satisfied.\n\nTakes the same arguments as `all-completions' and `try-completion'.\n\nIf COLLECTION is a function, it is called with three arguments:\nthe values STRING, PREDICATE and `lambda'.",
       (string: Lisp_Object, collection: Lisp_Object, predicate: Lisp_Object),
       {
  let mut collection: Lisp_Object = collection;
  let mut tem: Lisp_Object = Qnil ();
  let mut arg: Lisp_Object = Qnil ();

  CHECK_STRING (string);

  if NILP (collection) || (CONSP (collection) && !FUNCTIONP (collection))
    {
      tem = Fassoc_string (string, collection, if completion_ignore_case { Qt () } else { Qnil () });
      if NILP (tem)
	{ return Qnil (); }
    }
  else if OBARRAYP (collection) || VECTORP (collection)
    {
      collection = check_obarray (collection);
      /* Bypass intern-soft as that loses for nil.  */
      tem = oblookup (collection,
		      SSDATA (string),
		      SCHARS (string),
		      SBYTES (string));
      if completion_ignore_case && !BARE_SYMBOL_P (tem)
	{
	  DOOBARRAY! (XOBARRAY (collection), it, {
	      let obj: Lisp_Object = obarray_iter_symbol (&raw mut it);
	      if BASE_EQ (Fcompare_strings (string, make_fixnum (0),
					    Qnil (),
					    Fsymbol_name (obj),
					    make_fixnum (0), Qnil (), Qt ()),
			  Qt ())
		{
		  tem = obj;
		  break;
		}
	  });
	}

      if !BARE_SYMBOL_P (tem)
	{ return Qnil (); }
    }
  else if HASH_TABLE_P (collection)
    {
      let h: *mut Lisp_Hash_Table = XHASH_TABLE (collection);
      let i: ptrdiff_t = hash_find (h, string);
      /* C's two `goto found_matching_key;` are the `break` out of the labeled block below,
	 whose tail is C's `found_matching_key: ;` at the end of this arm; control then falls
	 out into the shared statements after the chain, exactly as C's does
	 (PORTING.md 2.2).  */
      'found_matching_key: {
	if i >= 0
	  {
	    tem = HASH_KEY (h, i as hash_idx_t);
	    arg = HASH_VALUE (h, i as hash_idx_t);
	  }
	else
	  {
	    DOHASH! (h, k, v, {
		tem = k;
		let strkey: Lisp_Object = if SYMBOLP (tem) { Fsymbol_name (tem) } else { tem };
		/* C's `if (!STRINGP (strkey)) continue;` advances the loop, and the DOHASH!
		   macro's increment (which carries C's own eassert) runs at the end of the
		   body, so the port nests the rest under C's positive test rather than
		   writing a `continue` that would skip it.  */
		if STRINGP (strkey)
		  {
		    if BASE_EQ (Fcompare_strings (string, Qnil (), Qnil (),
						  strkey, Qnil (), Qnil (),
						  if completion_ignore_case { Qt () } else { Qnil () }),
				Qt ())
		      {
			arg = v;
			break 'found_matching_key;
		      }
		  }
	    });
	    return Qnil ();
	  }
      }
    }
  else
    { return calln! (collection, string, predicate, Qlambda ()); }

  /* Reject this element if it fails to match all the regexps.  */
  if !match_regexps (string, Vcompletion_regexp_list,
		     completion_ignore_case)
    { return Qnil (); }

  /* Finally, check the predicate.  */
  if !NILP (predicate)
    {
      return if HASH_TABLE_P (collection)
	       { calln! (predicate, tem, arg) }
	     else
	       { calln! (predicate, tem) };
    }
  else
    { return Qt (); }
});

DEFUN! ("internal-complete-buffer", Finternal_complete_buffer, Sinternal_complete_buffer, 3, 3, 0,
       doc: "Perform completion on buffer names.\nSTRING and PREDICATE have the same meanings as in `try-completion',\n`all-completions', and `test-completion'.\n\nIf FLAG is nil, invoke `try-completion'; if it is t, invoke\n`all-completions'; otherwise invoke `test-completion'.",
       (string: Lisp_Object, predicate: Lisp_Object, flag: Lisp_Object),
       {
  if NILP (flag)
    { return Ftry_completion (string, Vbuffer_alist, predicate); }
  else if EQ (flag, Qt ())
    {
      let res: Lisp_Object = Fall_completions (string, Vbuffer_alist, predicate);
      if SCHARS (string) > 0
	{ return res; }
      else
	{ /* Strip out internal buffers.  */
	  let mut bufs: Lisp_Object = res;
	  let mut res: Lisp_Object = res;
	  /* First, look for a non-internal buffer in `res'.  */
	  while CONSP (bufs) && SREF (XCAR (bufs), 0) == b' '
	    { bufs = XCDR (bufs); }
	  if NILP (bufs)
	    {
	      return if list_length (res) == list_length (Vbuffer_alist)
		       /* If all bufs are internal don't strip them out.  */
		       { res } else { bufs };
	    }
	  res = bufs;
	  while CONSP (XCDR (bufs))
	    {
	      if SREF (XCAR (XCDR (bufs)), 0) == b' '
		{ XSETCDR (bufs, XCDR (XCDR (bufs))); }
	      else
		{ bufs = XCDR (bufs); }
	    }
	  return res;
	}
    }
  else if EQ (flag, Qlambda ())
    { return Ftest_completion (string, Vbuffer_alist, predicate); }
  else if EQ (flag, Qmetadata ())
    {
      return list3 (Qmetadata (),
		    Fcons (Qcategory (), Qbuffer ()),
		    Fcons (Qcycle_sort_function (), Qidentity ()));
    }
  else
    { return Qnil (); }
});

/* Like assoc but assumes KEY is a string, and ignores case if appropriate.  */

DEFUN! ("assoc-string", Fassoc_string, Sassoc_string, 2, 3, 0,
       doc: "Like `assoc' but specifically for strings (and symbols).\n\nThis returns the first element of LIST whose car matches the string or\nsymbol KEY, or nil if no match exists.  When performing the\ncomparison, symbols are first converted to strings, and unibyte\nstrings to multibyte.  If the optional arg CASE-FOLD is non-nil, both\nKEY and the elements of LIST are upcased for comparison.\n\nUnlike `assoc', KEY can also match an entry in LIST consisting of a\nsingle string, rather than a cons cell whose car is a string.",
       (key: Lisp_Object, list: Lisp_Object, case_fold: Lisp_Object),
       {
  let mut key: Lisp_Object = key;
  let mut tail: Lisp_Object;

  if SYMBOLP (key)
    { key = Fsymbol_name (key); }

  tail = list;
  while CONSP (tail)
    {
      let elt: Lisp_Object;
      let mut tem: Lisp_Object;
      let mut thiscar: Lisp_Object;
      elt = XCAR (tail);
      thiscar = if CONSP (elt) { XCAR (elt) } else { elt };
      if SYMBOLP (thiscar)
	{ thiscar = Fsymbol_name (thiscar); }
      else if !STRINGP (thiscar)
	{ tail = XCDR (tail); continue; }
      tem = Fcompare_strings (thiscar, make_fixnum (0), Qnil (),
			      key, make_fixnum (0), Qnil (),
			      case_fold);
      if EQ (tem, Qt ())
	{ return elt; }
      maybe_quit ();
      tail = XCDR (tail);
    }
  return Qnil ();
});

DEFUN! ("minibuffer-depth", Fminibuffer_depth, Sminibuffer_depth, 0, 0, 0,
       doc: "Return current depth of activations of minibuffer, a nonnegative integer.",
       (),
       {
  return make_fixnum (minibuf_level);
});

DEFUN! ("minibuffer-prompt", Fminibuffer_prompt, Sminibuffer_prompt, 0, 0, 0,
       doc: "Return the prompt string of the currently-active minibuffer.\nIf no minibuffer is active, return nil.",
       (),
       {
  return Fcopy_sequence (minibuf_prompt);
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_initial_minibuffer_mode ()
{
  let minibuf: Lisp_Object = get_minibuffer (0);
  set_minibuffer_mode (minibuf, 0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_minibuf_once ()
{
  staticpro (&raw const Vminibuffer_list);
  staticpro (&raw const Vcommand_loop_level_list);
  pdumper_do_now_and_after_load (Some (init_minibuf_once_for_pdumper));
  /* Ensure our inactive minibuffer exists.  */
  get_minibuffer (0);
}

unsafe extern "C" fn init_minibuf_once_for_pdumper ()
{
  PDUMPER_IGNORE! (minibuf_level);
  PDUMPER_IGNORE! (minibuf_prompt_width);

  /* We run this function on first initialization and whenever we
     restore from a dump file.  pdumper doesn't try to preserve
     frames, windows, and so on, so reset everything related here.  */
  Vminibuffer_list = Qnil ();
  Vcommand_loop_level_list = Qnil ();
  minibuf_level = 0;
  minibuf_prompt = Qnil ();
  minibuf_save_list = Qnil ();
  last_minibuf_string = Qnil ();
}

/* C 2355-2357's `#define FLEX_MAX_STR_SIZE 512`, `#define FLEX_MAX_PAT_SIZE 128` and
   `#define FLEX_MAX_MATRIX_SIZE FLEX_MAX_PAT_SIZE * FLEX_MAX_STR_SIZE` sit between the
   `DEFUN` docstring and its parameter list, which Rust has no position for, so they are
   written just above the invocation; the three are C's integer constants and the port
   spells them as Rust constants, with C's own expression for the third.  C 2359's `MAT` is
   written inside the body, after `width`, because a `macro_rules!` resolves the locals it
   mentions at its definition site (docs/ports/macro-body-notes.tsv).  */
pub const FLEX_MAX_STR_SIZE: c_int = 512;
pub const FLEX_MAX_PAT_SIZE: c_int = 128;
pub const FLEX_MAX_MATRIX_SIZE: c_int = FLEX_MAX_PAT_SIZE * FLEX_MAX_STR_SIZE;

/* FLEX/GOTOH algorithm for the 'flex' completion-style.  Adapted from
   GOTOH, Osamu.  An improved algorithm for matching biological
   sequences. Journal of molecular biology, 1982, 162.3: 705-708.

   This algorithm matches patterns to candidate strings, or needles to
   haystacks.  It works with cost matrices: imagine rows of these
   matrices as pattern characters, and columns as the candidate string
   characters.  There is a -1 row, and a -1 column.  The values there
   hold real costs used for situations "before the first ever" match of
   a pattern character to a string character.

   M and D are cost matrices.  At the end of the algorithm, M will have
   non-infinite values only for the spots where a pattern character
   matches a string character.  So a non-infinite M[i,j] means the i-th
   character of the pattern matches the j-th character of the string.
   The value stored is the lowest possible cost the algorithm had to
   "pay" to be able to make that match there, given everything that may
   have happened before/to the left.  An infinite value simply means no
   match at this pattern/string position.  Note that both rows and
   columns of M may have more than one match at multiple indices.  If
   some row or column of M has no match at all, the final cost will be
   infinite, and the funtion will return nil.

   D (originally stands for 'Distance' in the Gotoh paper) has "running
   costs".  Each value D[i,j] represents what the algorithm has to pay
   to make or extend a gap when a match is found at i+1, j+1.  By that
   time, that cost may or may not be lower than continuing from a match
   that had also been found at i,j.  We always pick the lowest cost, and
   by the time we reach the final column, we know we have picked the
   cheapest possible path choosing when to gap, and when to follow up.

   Here's an illustration of the final state of M and D when matching
   "goto" to "eglot--goto".  Notice how the algorithm detects but
   ultimately discards the scattered match at indexes 1,3,4,8, which
   would have total cost 27, and ultimately settles on the match at
   positions 7,8,9,10 which has cost 5:

             -1  0  1  2  3  4  5  6  7  8  9 10
              -  e  g  l  o  t  -  -  g  o  t  o
   M:  -1  -  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞
        0  g  ∞  ∞  5  ∞  ∞  ∞  ∞  ∞  5  ∞  ∞  ∞
        1  o  ∞  ∞  ∞  ∞ 15  ∞  ∞  ∞  ∞  5  ∞ 16
        2  t  ∞  ∞  ∞  ∞  ∞ 15  ∞  ∞  ∞  ∞  5  ∞
        3  o  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞ 27  ∞  5

   D:  -1  -  0  5  5  5  5  5  5  5  5  5  5  5
        0  g  ∞  ∞  ∞ 15 16 17 18 19 20 15 16 17
        1  o  ∞  ∞  ∞  ∞  ∞ 25 26 27 28 29 15 16
        2  t  ∞  ∞  ∞  ∞  ∞  ∞ 25 26 27 28 29 15
        3  o  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞  ∞ 37 38
 **/
DEFUN! ("completion--flex-cost-gotoh", Fcompletion__flex_cost_gotoh,
       Scompletion__flex_cost_gotoh, 2, 2, 0,
       doc: "Compute cost of PAT matching STR using modified Gotoh\nalgorithm.  Return nil if no match found, else return (COST . MATCHES)\nwhere COST is a fixnum (lower is better) and MATCHES is a list of the\nsame length as PAT.  Each i-th element is a FIXNUM indicating where in\nSTR the i-th character of PAT matched.",
       (pat: Lisp_Object, str: Lisp_Object),
       {
  /* Pre-allocated matrices for flex completion scoring.  */

  CHECK_STRING (pat);
  CHECK_STRING (str);

  let patlen: ptrdiff_t = SCHARS (pat);
  let strlen: ptrdiff_t = SCHARS (str);
  let width: ptrdiff_t = strlen + 1;

  /* C: `#define MAT(matrix, i, j) ((matrix)[((i) + 1) * width + ((j) + 1)])`.  C's `int`
     operands promote to the `ptrdiff_t` the index needs; the port writes those promotions
     as `as ptrdiff_t` (PORTING.md 2.6).  */
  macro_rules! MAT { ($matrix:expr, $i:expr, $j:expr) =>
    { $matrix[((($i) as ptrdiff_t + 1) * width + (($j) as ptrdiff_t + 1)) as usize] }; }

  let size: ptrdiff_t = (patlen + 1) * width;
  let gap_open_cost: c_int = 10;
  let gap_extend_cost: c_int = 1;
  let pos_inf: c_int = INT_MAX / 2;
  static mut M: [c_int; FLEX_MAX_MATRIX_SIZE as usize] = [0; FLEX_MAX_MATRIX_SIZE as usize];
  static mut D: [c_int; FLEX_MAX_MATRIX_SIZE as usize] = [0; FLEX_MAX_MATRIX_SIZE as usize];

  /* Bail if strings are empty or matrix too large.  */
  if patlen == 0 || strlen == 0 || size > FLEX_MAX_MATRIX_SIZE as ptrdiff_t
    { return Qnil (); }

  /* Also bail if PAT is not a subsequence of STR, so bail "cheaply"
     before the O(N*M) DP algorithm.  Walking both strings
     byte-by-byte for this purpose (and only for case-sensitive common
     case) should be valid even for multibyte strings.  */
  if !completion_ignore_case
    {
      let p: *const c_uchar = SDATA (pat) as *const c_uchar;
      let s: *const c_uchar = SDATA (str) as *const c_uchar;
      let mut pi: c_int = 0;
      let mut si: c_int = 0;
      while (si as ptrdiff_t) < strlen && (pi as ptrdiff_t) < patlen
	{
	  if *s.offset (si as isize) == *p.offset (pi as isize)
	    { pi += 1; }
	  si += 1;
	}
      if (pi as ptrdiff_t) < patlen
	{ return Qnil (); }
    }

  /* Initialize M and D with positive infinity...  */
  let mut j: c_int = 0;
  while (j as ptrdiff_t) < size
    {
      M[j as usize] = pos_inf;
      D[j as usize] = pos_inf;
      j += 1;
    }
  /* ...except for D[-1,-1], which is 0 to promote matches at the
     beginning.  Rest of first row has gap_open_cost/2 for cheaper
     leading gaps.  */
  j = 0;
  while (j as ptrdiff_t) < width
    {
      D[j as usize] = gap_open_cost / 2;
      j += 1;
    }
  D[0] = 0;

  /* Poor man's iterator type: cur char idx, next char idx, next byte idx.  */
  #[repr(C)]
  #[derive(Clone, Copy)]
  struct iter { x: c_int, n: ptrdiff_t, b: ptrdiff_t }
  /* C: `typedef struct iter { ... } iter_t;` — the port keeps both C names, so the tag and
     the typedef each have a counterpart.  */
  type iter_t = iter;

  /* Position of first match found in the previous row, to save
     iterations.  */
  let mut prev_match: iter_t = iter_t { x: 0, n: 0, b: 0 };

  /* Forward pass.  */
  let mut i: iter_t = iter_t { x: 0, n: 0, b: 0 };
  while (i.x as ptrdiff_t) < patlen
    {
      let pat_char: c_int = fetch_string_char_advance (pat, &raw mut i.n, &raw mut i.b);
      let mut match_seen: bool = false;

      let mut jj: iter_t = prev_match;
      while (jj.x as ptrdiff_t) < strlen
	{
	  let jcopy: iter_t = jj; /* else advance function destroys it...  */
	  let str_char: c_int = fetch_string_char_advance (str, &raw mut jj.n, &raw mut jj.b);

	  /* Check if characters match (case-insensitive if needed).  */
	  let cmatch: bool;
	  if completion_ignore_case
	    { cmatch = downcase (pat_char) == downcase (str_char); }
	  else
	    { cmatch = pat_char == str_char; }

	  if cmatch
	    {
	      /* There is a match here, so compute match cost
		 M[i][j], i.e. replace its infinite value with
		 something finite.  */
	      if !match_seen
		{
		  match_seen = true;
		  prev_match = jcopy;
		}
	      /* Compute M[i,j]. If we pick M[i-1,j-1] it means that
		 not only did the previous char also match (else
		 M[i-1,j-1] would have been infinite) but following
		 it up with this match is best overall.  If we pick
		 D[i-1, j-1] it means that gapping is best,
		 regardless of whether the previous char also
		 matched.  That is, it's better to arrive at this
		 match from a gap.  */
	      MAT! (M, i.x, jj.x) = min (MAT! (M, i.x - 1, jj.x - 1) as isize,
					 MAT! (D, i.x - 1, jj.x - 1) as isize) as c_int;
	    }
	  /* Regardless of a match here, compute D[i,j], the best
	     accumulated gapping cost at this point, considering
	     whether it's more advantageous to open from a previous
	     match on this row (a cost which may well be infinite if
	     no such match ever existed) or extend a gap started
	     sometime before.  The next iteration will take this
	     into account, and so will the next row when analyzing a
	     possible match for the j+1-th string character.  */
	  MAT! (D, i.x, jj.x)
	    = min ((MAT! (M, i.x, jj.x - 1) + gap_open_cost) as isize,
		   (MAT! (D, i.x, jj.x - 1) + gap_extend_cost) as isize) as c_int;
	  jj.x += 1;
	}
      i.x += 1;
    }
  /* Find lowest cost in last row.  */
  let mut best_cost: c_int = pos_inf;
  let mut lastcol: c_int = -1;
  let mut j2: c_int = 0;
  while (j2 as ptrdiff_t) < strlen
    {
      let cost: c_int = MAT! (M, patlen - 1, j2);
      if cost < best_cost
	{
	  best_cost = cost;
	  lastcol = j2;
	}
      j2 += 1;
    }

  /* Return early if no match.  */
  if lastcol < 0 || best_cost >= pos_inf
    { return Qnil (); }

  /* Go backwards to build match positions list.  */
  let mut matches: Lisp_Object = Fcons (make_fixnum (lastcol as EMACS_INT), Qnil ());
  let mut i2: c_int = patlen as c_int - 2;
  let mut l: c_int = lastcol;
  while i2 >= 0
    {
      loop
	{
	  l -= 1;
	  if ! (l >= 0
		&& MAT! (M, i2, l) >= MAT! (D, i2, l))
	    { break; }
	}
      matches = Fcons (make_fixnum (l as EMACS_INT), matches);
      i2 -= 1;
    }

  return Fcons (make_fixnum (best_cost as EMACS_INT), matches);
});

/* The `DEFVAR_*` variables this unit owns.  C's storage for them is `struct emacs_globals`
   (lisp.h's `DEFVAR_LISP` reads `&globals.f_<vname>`), and the unit that owns a variable
   defines it (status §3.29, the casetab.rs/cmds.rs shape); `include/globals.rs`'s
   `variables` module declares the names, so these definitions shadow the glob import.
   C's `emacs_globals` is zero-initialised, so the port starts them at zero and lets
   `syms_of_minibuf` assign C's initial values.  */

#[unsafe(no_mangle)]
pub static mut Vread_expression_history: Lisp_Object = lisp_h_Qnil ();      /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vread_buffer_function: Lisp_Object = lisp_h_Qnil ();          /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut minibuffer_follows_selected_frame: Lisp_Object = lisp_h_Qnil (); /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut read_buffer_completion_ignore_case: bool = false;             /* DEFVAR_BOOL */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_setup_hook: Lisp_Object = lisp_h_Qnil ();         /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_exit_hook: Lisp_Object = lisp_h_Qnil ();          /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vhistory_length: Lisp_Object = lisp_h_Qnil ();                /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut history_delete_duplicates: bool = false;                      /* DEFVAR_BOOL */
#[unsafe(no_mangle)]
pub static mut Vhistory_add_new_input: Lisp_Object = lisp_h_Qnil ();         /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut completion_ignore_case: bool = false;                         /* DEFVAR_BOOL */
#[unsafe(no_mangle)]
pub static mut enable_recursive_minibuffers: bool = false;                   /* DEFVAR_BOOL */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_completion_table: Lisp_Object = lisp_h_Qnil ();   /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_completion_predicate: Lisp_Object = lisp_h_Qnil (); /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_completion_confirm: Lisp_Object = lisp_h_Qnil (); /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_completing_file_name: Lisp_Object = lisp_h_Qnil (); /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_help_form: Lisp_Object = lisp_h_Qnil ();          /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_history_variable: Lisp_Object = lisp_h_Qnil ();   /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_history_position: Lisp_Object = lisp_h_Qnil ();   /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut minibuffer_auto_raise: bool = false;                          /* DEFVAR_BOOL */
#[unsafe(no_mangle)]
pub static mut Vcompletion_regexp_list: Lisp_Object = lisp_h_Qnil ();        /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut minibuffer_allow_text_properties: bool = false;               /* DEFVAR_BOOL */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_prompt_properties: Lisp_Object = lisp_h_Qnil ();  /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut Vread_hide_char: Lisp_Object = lisp_h_Qnil ();                /* DEFVAR_LISP */
#[unsafe(no_mangle)]
pub static mut inhibit_interaction: bool = false;                            /* DEFVAR_BOOL */
#[unsafe(no_mangle)]
pub static mut read_minibuffer_restore_windows: bool = false;                /* DEFVAR_BOOL */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_minibuf ()
{
  staticpro (&raw const minibuf_prompt);
  staticpro (&raw const minibuf_save_list);
  staticpro (&raw const MB_frame);
  MB_frame = Qnil ();
  staticpro (&raw const exp_MB_frame);

  DEFSYM! (Qminibuffer_follows_selected_frame, c"minibuffer-follows-selected-frame");
  DEFSYM! (Qcompletion_ignore_case, c"completion-ignore-case");
  DEFSYM! (Qminibuffer_default, c"minibuffer-default");
  Fset (Qminibuffer_default (), Qnil ());

  DEFSYM! (Qminibuffer_completion_table, c"minibuffer-completion-table");

  staticpro (&raw const last_minibuf_string);

  DEFSYM! (Qcustom_variable_history, c"custom-variable-history");
  Fset (Qcustom_variable_history (), Qnil ());

  DEFSYM! (Qminibuffer_history, c"minibuffer-history");
  DEFSYM! (Qbuffer_name_history, c"buffer-name-history");
  Fset (Qbuffer_name_history (), Qnil ());

  DEFSYM! (Qcustom_variable_p, c"custom-variable-p");

  /* Normal hooks for entry to and exit from minibuffer.  */
  DEFSYM! (Qminibuffer_setup_hook, c"minibuffer-setup-hook");
  DEFSYM! (Qminibuffer_exit_hook, c"minibuffer-exit-hook");

  DEFSYM! (Qcurrent_input_method, c"current-input-method");
  DEFSYM! (Qactivate_input_method, c"activate-input-method");
  DEFSYM! (Qmetadata, c"metadata");
  DEFSYM! (Qcycle_sort_function, c"cycle-sort-function");

  /* A frame parameter.  */
  DEFSYM! (Qminibuffer_exit, c"minibuffer-exit");

  DEFSYM! (Qminibuffer_mode, c"minibuffer-mode");
  DEFSYM! (Qminibuffer_inactive_mode, c"minibuffer-inactive-mode");
  DEFSYM! (Qminibuffer_completing_file_name, c"minibuffer-completing-file-name");
  DEFSYM! (Qselect_frame_set_input_focus, c"select-frame-set-input-focus");
  DEFSYM! (Qadd_to_history, c"add-to-history");
  DEFSYM! (Qpush_window_buffer_onto_prev, c"push-window-buffer-onto-prev");

  DEFVAR_LISP! (c"read-expression-history", Vread_expression_history,
	       "A history list for arguments that are Lisp expressions to evaluate.\nFor example, `eval-expression' uses this.");
  Vread_expression_history = Qnil ();

  DEFVAR_LISP! (c"read-buffer-function", Vread_buffer_function,
	       "If this is non-nil, `read-buffer' does its work by calling this function.\nThe function is called with the arguments passed to `read-buffer'.");
  Vread_buffer_function = Qnil ();

  DEFVAR_LISP! (c"minibuffer-follows-selected-frame", minibuffer_follows_selected_frame,
               "t means the active minibuffer always displays on the selected frame.\nNil means that a minibuffer will appear only in the frame which created it.\nAny other value means the minibuffer will move onto another frame, but\nonly when the user starts using a minibuffer there.\n\nAny buffer local or dynamic binding of this variable is ignored.  Only the\ndefault top level value is used.");
  minibuffer_follows_selected_frame = Qt ();

  DEFVAR_BOOL! (c"read-buffer-completion-ignore-case",
	       read_buffer_completion_ignore_case,
	       "Non-nil means completion ignores case when reading a buffer name.");
  read_buffer_completion_ignore_case = false;

  DEFVAR_LISP! (c"minibuffer-setup-hook", Vminibuffer_setup_hook,
	       "Normal hook run just after entry to minibuffer.");
  Vminibuffer_setup_hook = Qnil ();

  DEFVAR_LISP! (c"minibuffer-exit-hook", Vminibuffer_exit_hook,
	       "Normal hook run whenever a minibuffer is exited.");
  Vminibuffer_exit_hook = Qnil ();

  DEFVAR_LISP! (c"history-length", Vhistory_length,
	       "Maximum length of history lists before truncation takes place.\nA number means truncate to that length; truncation deletes old\nelements, and is done just after inserting a new element.\nA value of t means no truncation.\n\nThis variable only affects history lists that don't specify their own\nmaximum lengths.  Setting the `history-length' property of a history\nvariable overrides this default.");
  XSETFASTINT! (Vhistory_length, 100);

  DEFVAR_BOOL! (c"history-delete-duplicates", history_delete_duplicates,
	       "Non-nil means to delete duplicates in history.\nIf set to t when adding a new history element, all previous identical\nelements are deleted from the history list.");
  history_delete_duplicates = false;

  DEFVAR_LISP! (c"history-add-new-input", Vhistory_add_new_input,
	       "Non-nil means to add new elements in history.\nIf set to nil, minibuffer reading functions don't add new elements to the\nhistory list, so it is possible to do this afterwards by calling\n`add-to-history' explicitly.");
  Vhistory_add_new_input = Qt ();

  DEFVAR_BOOL! (c"completion-ignore-case", completion_ignore_case,
	       "Non-nil means don't consider case significant in completion.\nFor file-name completion, `read-file-name-completion-ignore-case'\ncontrols the behavior, rather than this variable.\nFor buffer name completion, `read-buffer-completion-ignore-case'\ncontrols the behavior, rather than this variable.");
  completion_ignore_case = false;

  DEFVAR_BOOL! (c"enable-recursive-minibuffers", enable_recursive_minibuffers,
	       "Non-nil means to allow minibuffer commands while in the minibuffer.\nThis variable makes a difference whenever the minibuffer window is active.\nAlso see `minibuffer-depth-indicate-mode', which may be handy if this\nvariable is non-nil.");
  enable_recursive_minibuffers = false;

  DEFVAR_LISP! (c"minibuffer-completion-table", Vminibuffer_completion_table,
	       "Alist or obarray used for completion in the minibuffer.\nThis becomes the ALIST argument to `try-completion' and `all-completions'.\nThe value can also be a list of strings or a hash table.\n\nThe value may alternatively be a function, which is given three arguments:\n  STRING, the current buffer contents;\n  PREDICATE, the predicate for filtering possible matches;\n  CODE, which says what kind of things to do.\nCODE can be nil, t or `lambda':\n  nil    -- return the best completion of STRING, or nil if there is none.\n  t      -- return a list of all possible completions of STRING.\n  lambda -- return t if STRING is a valid completion as it stands.");
  Vminibuffer_completion_table = Qnil ();

  DEFVAR_LISP! (c"minibuffer-completion-predicate", Vminibuffer_completion_predicate,
	       "Within call to `completing-read', this holds the PREDICATE argument.");
  Vminibuffer_completion_predicate = Qnil ();

  DEFVAR_LISP! (c"minibuffer-completion-confirm", Vminibuffer_completion_confirm,
	       "Whether to demand confirmation of completion before exiting minibuffer.\nIf nil, confirmation is not required.\nIf the value is `confirm', the user may exit with an input that is not\n a valid completion alternative, but Emacs asks for confirmation.\nIf the value is `confirm-after-completion', the user may exit with an\n input that is not a valid completion alternative, but Emacs asks for\n confirmation if the user submitted the input right after any of the\n completion commands listed in `minibuffer-confirm-exit-commands'.");
  Vminibuffer_completion_confirm = Qnil ();

  DEFVAR_LISP! (c"minibuffer-completing-file-name",
	       Vminibuffer_completing_file_name,
	       "Non-nil means completing file names.");
  Vminibuffer_completing_file_name = Qnil ();

  DEFVAR_LISP! (c"minibuffer-help-form", Vminibuffer_help_form,
	       "Value that `help-form' takes on inside the minibuffer.");
  Vminibuffer_help_form = Qnil ();

  DEFVAR_LISP! (c"minibuffer-history-variable", Vminibuffer_history_variable,
	       "History list symbol to add minibuffer values to.\nEach string of minibuffer input, as it appears on exit from the minibuffer,\nis added with\n\n  (set minibuffer-history-variable\n       (cons STRING (symbol-value minibuffer-history-variable)))\n\n If the variable is t, no history is recorded.");
  XSETFASTINT! (Vminibuffer_history_variable, 0);

  DEFVAR_LISP! (c"minibuffer-history-position", Vminibuffer_history_position,
	       "Current position of redoing in the history list.");
  Vminibuffer_history_position = Qnil ();

  DEFVAR_BOOL! (c"minibuffer-auto-raise", minibuffer_auto_raise,
	       "Non-nil means entering the minibuffer raises the minibuffer's frame.\nSome uses of the echo area also raise that frame (since they use it too).");
  minibuffer_auto_raise = false;

  DEFVAR_LISP! (c"completion-regexp-list", Vcompletion_regexp_list,
	       "List of regexps that should restrict possible completions.\nThe basic completion functions only consider a completion acceptable\nif it matches all regular expressions in this list, with\n`case-fold-search' bound to the value of `completion-ignore-case'.\nSee Info node `(elisp)Basic Completion', for a description of these\nfunctions.\n\nDo not set this variable to a non-nil value globally, as that is not\nsafe and will probably cause errors in completion commands.  This\nvariable should be only let-bound to non-nil values around calls to\nbasic completion functions like `try-completion' and `all-completions'.");
  Vcompletion_regexp_list = Qnil ();

  DEFVAR_BOOL! (c"minibuffer-allow-text-properties",
	       minibuffer_allow_text_properties,
	       "Non-nil means `read-from-minibuffer' should not discard text properties.\nLisp code can let-bind this, or make it buffer-local in the minibuffer.\nThis also affects `read-string', or any of the functions that do\nminibuffer input with completion, but it does not affect `read-minibuffer'\nthat always discards text properties.");
  minibuffer_allow_text_properties = false;

  DEFVAR_LISP! (c"minibuffer-prompt-properties", Vminibuffer_prompt_properties,
	       "Text properties that are added to minibuffer prompts.\nThese are in addition to the basic `field' property, and stickiness\nproperties.");
  Vminibuffer_prompt_properties = list2 (Qread_only (), Qt ());

  DEFVAR_LISP! (c"read-hide-char", Vread_hide_char,
	       "Whether to hide input characters in noninteractive mode.\nIf non-nil, it must be a character, which will be used to mask the\ninput characters.  This variable should never be set globally.\n\nThis variable also overrides the default character that `read-passwd'\nuses to hide passwords.");
  Vread_hide_char = Qnil ();

  DEFVAR_BOOL! (c"inhibit-interaction",
	       inhibit_interaction,
	       "Non-nil means any user interaction will signal an error.\nThis variable can be bound when user interaction can't be performed,\nfor instance when running a headless Emacs server.  Functions like\n`read-from-minibuffer' (and the like) will signal `inhibited-interaction'\ninstead.");
  inhibit_interaction = false;

  DEFVAR_BOOL! (c"read-minibuffer-restore-windows", read_minibuffer_restore_windows,
	       "Non-nil means restore window configurations on exit from minibuffer.\nIf this is non-nil (the default), reading input with the minibuffer will\nrestore, on exit, the window configurations of the frame where the\nminibuffer was entered from and, if it is different, the frame that owns\nthe associated minibuffer window.\n\nIf this is nil, window configurations are not restored upon exiting\nthe minibuffer.  However, if `minibuffer-restore-windows' is present\nin `minibuffer-exit-hook', exiting the minibuffer will remove the window\nshowing the *Completions* buffer, if any.");
  read_minibuffer_restore_windows = true;

  defsubr (&raw mut Sactive_minibuffer_window);
  defsubr (&raw mut Sset_minibuffer_window);
  defsubr (&raw mut Sread_from_minibuffer);
  defsubr (&raw mut Sread_string);
  defsubr (&raw mut Sread_command);
  defsubr (&raw mut Sread_variable);
  defsubr (&raw mut Sinternal_complete_buffer);
  defsubr (&raw mut Sread_buffer);
  defsubr (&raw mut Sminibuffer_depth);
  defsubr (&raw mut Sminibuffer_prompt);

  defsubr (&raw mut Sminibufferp);
  defsubr (&raw mut Sinnermost_minibuffer_p);
  defsubr (&raw mut Sminibuffer_innermost_command_loop_p);
  defsubr (&raw mut Sabort_minibuffers);
  defsubr (&raw mut Sminibuffer_prompt_end);
  defsubr (&raw mut Sminibuffer_contents);
  defsubr (&raw mut Sminibuffer_contents_no_properties);

  defsubr (&raw mut Stry_completion);
  defsubr (&raw mut Sall_completions);
  defsubr (&raw mut Stest_completion);
  defsubr (&raw mut Sassoc_string);
  defsubr (&raw mut Scompleting_read);
  defsubr (&raw mut Scompletion__flex_cost_gotoh);
  DEFSYM! (Qminibuffer_quit_recursive_edit, c"minibuffer-quit-recursive-edit");
  DEFSYM! (Qinternal_complete_buffer, c"internal-complete-buffer");
  DEFSYM! (Qcompleting_read_function, c"completing-read-function");
  DEFSYM! (Qformat_prompt, c"format-prompt");
}
