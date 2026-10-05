// femacs: port of src/androidterm.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same
// error model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2);
// the C `static` helpers stay plain `unsafe fn` except the seven whose address C takes - they
// become `unsafe extern "C" fn` items (a function-pointer member cannot be a Rust-ABI item; the
// androidmenu.rs / androidselect.rs / androidfont.rs shape).
//
// **This is the first slice of P-86's largest Android unit: C 1-829.**  `configure.ac` 2597 puts
// `androidterm.o` in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (2594) - both Android
// branches, like `androidmenu.o` - so `docs/baseline/frozen-build-sources.txt` lists no
// `androidterm.o` for this target, `extract-abi.py`'s checklist for the file is empty, and P-86
// ②'s substitute is what the ledger `docs/ports/androidterm.md` carries: the C line-range table,
// the `## Evidence` readings and the `## Deviations`.  Two gates, in C's order:
//
//   * the file gate (`HAVE_ANDROID`) is `feature = "have_android"`, on every item *and on C 21-36's
//     include graph* - which is why the frozen configuration compiles nothing here (P-86 ⑧);
//   * C 47's `#ifndef ANDROID_STUBIFY` ... C 6662's `#endif` (this file's file-level arm has **no**
//     `#else`, so its stub arm is *nothing*; the three stub `#if`s the file carries are all past
//     this slice) is `feature = "not_android_stubify"`, the feature the tree registered for C's
//     `HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640).
//
// **Why 829.**  P-86 ④ says to stop at a whole-item boundary and P-86 ⑦ that the ledger must name
// who is next.  C 836-1892 is `handle_one_android_event`, one 1,056-line function - the next
// whole-item boundary after it is C 1892, far past 1,000 - so this slice ends at C 829's `}`, the
// end of `android_handle_ime_event`.  **The next live definition is `handle_one_android_event`
// (C 836-1892), with its forward declaration `static void android_notify_conversion (unsigned
// long);` at C 834** (the port's table records C 830-7117 as the remainder).
//
// A third C condition has no `#if` in this file and therefore no gate either: `HAVE_WINDOW_SYSTEM`
// (configure.ac 7518; the `window_system=android` branch at 2594-2596 is what puts this file in the
// build list at all, so an Android configuration always has it).  Four places in this slice's text
// need it - C 519's `Emacs_Rectangle *r` (dispextern.h 47-64 sits under `#ifdef
// HAVE_WINDOW_SYSTEM`), C 543's `note_mouse_highlight`, C 545's `remember_mouse_glyph` and C 350's
// `buffer_flipping_blocked_p` - so the ledger records that the arm reading is a
// `have_android`-plus-`not_android_stubify` reading of the mirror's *frozen* side, and names the
// `include/frame.rs`/`include/dispextern.rs` window-system round that is owed by other lanes.
//
// The three known mirror blockers this slice touches are named in the ledger, not repaired here:
// the three `bool_bf` macros `FRAME_ANDROID_DRAWABLE`/`FRAME_ANDROID_COMPLETE_P`/
// `FRAME_ANDROID_NEED_BUFFER_FLIP` and the direct `need_cursor_updates` access (the mirror folds
// those five `android_output` bits into `bits`, `include/androidterm.rs` 229-247), the forward
// call to `android_frame_rehighlight` (defined at C 2127, past the slice), and the seven
// use-site declarations in the block below.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 20: `#include <config.h>` (ungated)

/* C 21-24: the four system headers, on the file gate like the rest of the graph (P-86 ⑧).
   C 24's `<semaphore.h>` has no mirror in the tree; the only names C takes from it (`sem_t`,
   `sem_timedwait`, `sem_getvalue`, `sem_post`, `sem_init`, C 5032-6864) are all past this slice,
   so the include stands as this comment and the ledger records the gap (P-88 ①).  */
#[cfg (feature = "have_android")]
include!("../lib/include/stdio.rs");                  // C 21: `#include <stdio.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/math.rs");                   // C 22: `#include <math.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/stdlib.rs");                 // C 23: `#include <stdlib.h>`

/* C 26-36: the file's own include list.  It sits outside C 47's `#ifndef`, so the whole graph is
   on the file gate alone - no translation unit the frozen configuration builds reaches any of it.  */
#[cfg (feature = "have_android")]
include!("include/lisp.rs");                          // C 26: `#include "lisp.h"`
#[cfg (feature = "have_android")]
include!("include/androidterm.rs");                   // C 27: `#include "androidterm.h"`
#[cfg (feature = "have_android")]
include!("include/keyboard.rs");                      // C 28: `#include "keyboard.h"`
#[cfg (feature = "have_android")]
include!("include/blockinput.rs");                    // C 29: `#include "blockinput.h"`
#[cfg (feature = "have_android")]
include!("include/android.rs");                       // C 30: `#include "android.h"`
#[cfg (feature = "have_android")]
include!("include/buffer.rs");                        // C 31: `#include "buffer.h"`
#[cfg (feature = "have_android")]
include!("include/window.rs");                        // C 32: `#include "window.h"`
/* `MOUSE_HL_INFO!` is `include/frame.rs` 450-461's macro, and its body names `tty_output` -
   which frame.rs cannot include (termchar.rs includes frame.rs instead), so the *including* unit
   brings termchar.rs in, exactly as that mirror's comment says.  The frozen configuration has no
   window system, so the macro's live arm is C 1037-1039's TTY one and the HAVE_ANDROID arm this
   file needs (`&FRAME_ANDROID_OUTPUT (f)->mouse_highlight`) is the recorded mirror round the
   ledger owes: until it lands, C 332's expansion reads the wrong arm and is a named blocker, not
   a port choice.  */
#[cfg (feature = "have_android")]
include!("include/termchar.rs");                      // `tty_output` (frame.rs 450's MOUSE_HL_INFO!)
#[cfg (feature = "have_android")]
include!("include/textconv.rs");                      // C 33: `#include "textconv.h"`
#[cfg (feature = "have_android")]
include!("include/coding.rs");                        // C 34: `#include "coding.h"`
#[cfg (feature = "have_android")]
include!("include/pdumper.rs");                       // C 35: `#include "pdumper.h"`
#[cfg (feature = "have_android")]
include!("include/keymap.rs");                        // C 36: `#include "keymap.h"`

/* Three headers the file reaches through `lisp.h`/`systime.h` rather than naming, plus the two
   `include/androidterm.rs` pulls in (`dispextern.rs`/`androidgui.rs`/`frame.rs`/`termhooks.rs`).
   The C side gets `memset` from lisp.h 30's `<string.h>`, `fd_set`/`FD_ZERO`/`pselect` from
   `<sys/select.h>` (lisp.h 33's own `#include <limits.h>` supplies INT_MAX, used at C 390).  */
#[cfg (feature = "have_android")]
include!("../lib/include/string.rs");                 // lisp.h 30's `#include <string.h>` (memset)
#[cfg (feature = "have_android")]
include!("../lib/include/limits.in.rs");              // lisp.h 33's `#include <limits.h>` (INT_MAX, C 390)
#[cfg (feature = "have_android")]
include!("../lib/include/sys_select.rs");             // `fd_set`/`FD_ZERO`/`pselect` (C 199-251)

/* `struct redisplay_interface` - dispextern.h 3069-3196 - has no mirror (`include/termhooks.rs`
   181/219 degrades `rif` to `*mut c_void`), and `include/frame.rs` 1193-1200's `flush_frame!`
   body names the type, so a unit that calls C's `flush_frame (f)` declares it at the use site
   (P-72 rule 4, the `minibuf.rs` 652-668 shape: the ten members C declares before `flush_display`
   are all pointers, so the prefix is an opaque pointer array and the one offset that matters is
   pinned).  `docs/ports/androidterm.md` records the gap.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct redisplay_interface
{
  _prefix: [*mut c_void; 9],
  pub flush_display: Option<unsafe extern "C" fn (f: *mut frame)>,
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  assert! (std::mem::offset_of! (redisplay_interface, flush_display) == 72);
};

/* C 353/408's `flush_frame (f)`.  C's is frame.h 1914-1921's inline and `include/frame.rs`
   1193-1200 carries its `macro_rules!`, but that body's `FRAME_RIF!` hands back the mirror's
   `*mut c_void` (`include/termhooks.rs` 219 degrades `rif`), so the macro cannot type-check as
   written (measured: `E0308 expected *mut redisplay_interface, found *mut c_void` at
   `termhooks.rs` 425, two call sites).  The unit therefore writes C's own body with C's own text,
   the `minibuf.rs` 645-651 shape, and the ledger records the mirror round.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn flush_frame (f: *mut frame)
{
  let rif: *mut redisplay_interface =
    (*(*f).terminal).rif as *mut redisplay_interface;   /* C: `FRAME_RIF (f)`.  */

  if !rif.is_null () && (*rif).flush_display.is_some ()
    { ((*rif).flush_display).unwrap () (f); }
}

/* `include/textconv.rs` 160-243 puts textconv.h's non-Android declarations in
   `pub mod textconv_decls`, and that mirror's own header comment says the module carries "a glob
   import (P-17/P-21)"; the glob line is not in the frozen mirror (no unit called these ten
   functions yet), so this unit writes the use site rather than repeating the ten declarations
   (PORTING.md P-26: one declaration per name, in the mirror).  The ledger records the missing
   line for the integrator.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
use textconv_decls::*;

/* Eight declarations the sixteen headers above reach, which the frozen mirrors do not carry yet
   (P-72 rule 4: a mirror must reach what it needs through its own `include!`s).  Until each lands
   in its mirror the unit supplies it at the use site - the `alloc.rs` `mark_haiku_display` /
   `androidfont.rs` seven-declaration shape; every line names the C header it comes from, and
   `docs/ports/androidterm.md` records the gap for the integrator.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3879: `int popup_activated (void);` (androidmenu.c defines it).
  pub fn popup_activated () -> c_int;
  /// C: dispextern.h 4006: `extern bool gui_mouse_grabbed (Display_Info *);` -
  /// dispextern.h 165's `typedef struct android_display_info Display_Info;`.
  pub fn gui_mouse_grabbed (dpyinfo: *mut android_display_info) -> bool;
  /// C: dispextern.h 3719: `extern bool buffer_flipping_blocked_p (void);`
  pub fn buffer_flipping_blocked_p () -> bool;
  /// C: dispextern.h 3586: `void remember_mouse_glyph (struct frame *, int, int,
  /// NativeRectangle *);` - androidgui.h 218's `#define NativeRectangle Emacs_Rectangle`.
  pub fn remember_mouse_glyph (f: *mut frame, x: c_int, y: c_int,
                               rectangle: *mut Emacs_Rectangle);
  /// C: dispextern.h 3688: `extern void note_mouse_highlight (struct frame *, int, int);`
  pub fn note_mouse_highlight (f: *mut frame, x: c_int, y: c_int);
  /// C: coding.h 686: `extern void setup_coding_system (Lisp_Object, struct coding_system *);`
  pub fn setup_coding_system (system: Lisp_Object, coding: *mut coding_system);
  /// C: lisp.h 5095: `extern bool detect_input_pending (void);`
  pub fn detect_input_pending () -> bool;
  /// C: androidgui.h 926-927: `extern struct android_gc *android_create_gc (enum
  /// android_gc_value_mask, struct android_gc_values *);`
  pub fn android_create_gc (valuemask: android_gc_value_mask, values: *mut android_gc_values)
    -> *mut android_gc;
}

/* C 49: `#include <android/log.h>`.  The NDK's own header is not part of the reference tree, so it
   has no mirror (P-88 ①).  This slice takes no name from it - the file's only use,
   `__android_log_print` (C 1837), is inside the remainder - so the include stands as this comment.  */

/* C 61-67: C's anonymous `enum` with the three event-dispatch codes.  C names no tag and the
   members are never assigned, so they are 0/1/2 (`cm.rs`'s `CM_*` unnamed-enum shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_EVENT_NORMAL: c_int = 0;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_EVENT_GOTO_OUT: c_int = 1;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_EVENT_DROP: c_int = 2;

/* termhooks.h 373-412 says `enum event_kind kind : 16`, which the mirror's `input_event::bits`
   shares with `part : 16` (PORTING.md 1.3/P-80).  C writes `e->kind = x;`, so the port keeps C's
   field name in a per-unit macro - the `androidselect.rs` 136-143 shape, body-identical.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* frame.h 300-341's `bool_bf` flag run.  C's `f->auto_raise`, `f->auto_lower`, `f->mouse_moved`
   and `f->pointer_invisible` are struct fields, so no header mirror carries an accessor; the unit
   that owns `struct frame` (frame.rs) and each unit whose bodies touch the fields (keyboard.rs,
   term.rs) define their own per-unit macro, and this unit does the same for the four its slice
   touches (PORTING.md 1.3; docs/ports/frame.md).  The shift constants are `include/frame.rs`'s and
   are gated to the frozen (non-window-system) arm - a window-system configuration has no numbering
   there yet, which is the `include/frame.rs` round the ledger owes (docs/ports/include-frame.md).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! auto_raise {
  ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_RAISE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_AUTO_RAISE_SHIFT, $v) };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! auto_lower {
  ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_LOWER_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_AUTO_LOWER_SHIFT, $v) };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! mouse_moved {
  ($f:expr) => { (*$f).bits1 >> FRAME_MOUSE_MOVED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_MOUSE_MOVED_SHIFT, $v) };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! pointer_invisible {
  ($f:expr) => { (*$f).bits1 >> FRAME_POINTER_INVISIBLE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_POINTER_INVISIBLE_SHIFT, $v) };
}

/* C 41: `struct android_display_info *x_display_list;` - a non-`static` definition, so
   `#[unsafe(no_mangle)]` (PORTING.md 0.2); `include/androidterm.rs` 517 declares it in the
   glob-imported `androidterm_objs` module, which this definition shadows (PORTING.md 0.1-10).  */
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub static mut x_display_list: *mut android_display_info = std::ptr::null_mut ();

/* ------------------------------------------------------------------------------------------
   C 47-829: the whole body of this slice, under C 47's `#ifndef ANDROID_STUBIFY`.  C writes no
   `#else` here - the file's three stub arms (C 6666/6712/6737/6884) are all past C 829 - so a
   configuration with `ANDROID_STUBIFY` compiles the include graph and no item of this slice.  */

/* C 54: `static bool any_help_event_p;` (tentative, i.e. zero).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut any_help_event_p: bool = false;

/* C 59: `static double wheel_event_x, wheel_event_y;` (two names, one C statement).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut wheel_event_x: f64 = 0.0;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut wheel_event_y: f64 = 0.0;

/* Find the frame whose window has the identifier WDESC.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_window_to_frame (dpyinfo: *mut android_display_info,
                                   wdesc: android_window) -> *mut frame
{                                                              // C 74-96
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut f: *mut frame;

  if wdesc == ANDROID_NONE as android_window
    { return std::ptr::null_mut (); }

  /* C's `continue` jumps to the `for` increment; the mirror's FOR_EACH_FRAME puts the increment
     at the body's end, so the body is a labelled block and `break 'continue_;` leaves it - the
     increment still runs (`frame.rs` 2836-2845, docs/ports/frame.md).  */
  FOR_EACH_FRAME! (tail, frame, {
    'continue_: {
      f = XFRAME (frame);

      if !FRAME_ANDROID_P (f)
	{ break 'continue_; }

      if FRAME_ANDROID_WINDOW! (f) == wdesc
        { return f; }
    }
  });

  return std::ptr::null_mut ();
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_clear_frame (f: *mut frame)             // C 98-105
{
  /* Clearing the frame will erase any cursor, so mark them all as no
     longer visible.  */
  mark_window_cursors_off (XWINDOW (FRAME_ROOT_WINDOW (f)));
  android_clear_window (FRAME_ANDROID_DRAWABLE! (f));
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_show_hourglass (f: *mut frame)          // C 107-132
{
  let mut x: *mut android_output;

  /* This isn't implemented like X because a window brings alongside
     too many unneeded resources.  */

  x = FRAME_ANDROID_OUTPUT! (f) as *mut android_output;

  /* If the hourglass window is mapped inside a popup menu, input
     could be lost if the menu is popped down and the grab is
     relinquished, but the hourglass window is still up.  Just
     avoid displaying the hourglass at all while popups are
     active.  */

  if popup_activated () != 0
    { return; }

  (*x).hourglass = true;

  /* An hourglass cursor ought to be visible whether or not the standard
     cursor is invisible.  */
  android_define_cursor (FRAME_ANDROID_WINDOW! (f),
			 (*x).hourglass_cursor);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn make_invisible_cursor (dpyinfo: *mut android_display_info) -> android_cursor
{                                                                    // C 134-138
  return android_create_font_cursor (ANDROID_XC_NULL);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_hide_hourglass (f: *mut frame)          // C 140-161
{
  let mut x: *mut android_output;
  let mut dpyinfo: *mut android_display_info;

  x = FRAME_ANDROID_OUTPUT! (f) as *mut android_output;
  dpyinfo = FRAME_DISPLAY_INFO! (f);
  (*x).hourglass = false;

  if !pointer_invisible! (f)
    { android_define_cursor (FRAME_ANDROID_WINDOW! (f), (*x).current_cursor); }
  else
    {
      if (*dpyinfo).invisible_cursor == 0
	{ (*dpyinfo).invisible_cursor = make_invisible_cursor (dpyinfo); }

      android_define_cursor (FRAME_ANDROID_WINDOW! (f),
			     (*dpyinfo).invisible_cursor);
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_flash (f: *mut frame)                              // C 163-264
{
  let mut gc: *mut android_gc;
  let mut values: android_gc_values = std::mem::zeroed ();   // C 166 (PORTING.md 1.5)
  let mut rc: c_int;
  let mut fds: fd_set = std::mem::zeroed ();                 // C 168 (PORTING.md 1.5)

  block_input ();
  values.function = ANDROID_GC_INVERT;
  gc = android_create_gc (ANDROID_GC_FUNCTION, &raw mut values);

  /* Get the height not including a menu bar widget.  */
  let mut height: c_int = FRAME_PIXEL_HEIGHT (f);
  /* Height of each line to flash.  */
  let mut flash_height: c_int = FRAME_LINE_HEIGHT (f);
  /* These will be the left and right margins of the rectangles.  */
  let mut flash_left: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);
  let mut flash_right: c_int = FRAME_PIXEL_WIDTH (f) - FRAME_INTERNAL_BORDER_WIDTH (f);
  let mut width: c_int = flash_right - flash_left;

  /* If window is tall, flash top and bottom line.  */
  if height > 3 * FRAME_LINE_HEIGHT (f)
    {
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left,
			      (FRAME_INTERNAL_BORDER_WIDTH (f)
			       + FRAME_TOP_MARGIN_HEIGHT (f)),
			      width as c_uint, flash_height as c_uint);
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left,
			      (height - flash_height
			       - FRAME_INTERNAL_BORDER_WIDTH (f)
			       - FRAME_BOTTOM_MARGIN_HEIGHT (f)),
			      width as c_uint, flash_height as c_uint);

    }
  else
    /* If it is short, flash it all.  */
    {
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left, FRAME_INTERNAL_BORDER_WIDTH (f),
			      width as c_uint, (height - 2
						* FRAME_INTERNAL_BORDER_WIDTH (f)) as c_uint);
    }

  flush_frame (f);

  let mut delay: timespec = make_timespec (0, 150 * 1000 * 1000);
  let mut wakeup: timespec = timespec_add (current_timespec (), delay);

  /* Keep waiting until past the time wakeup or any input gets
     available.  */
  while !detect_input_pending ()
    {
      let mut current: timespec = current_timespec ();
      let mut timeout: timespec;

      /* Break if result would not be positive.  */
      if timespec_cmp (wakeup, current) <= 0
	{ break; }

      /* How long `select' should wait.  */
      timeout = make_timespec (0, 10 * 1000 * 1000);

      /* Wait for some input to become available on the X
	 connection.  */
      FD_ZERO (&raw mut fds);

      /* Try to wait that long--but we might wake up sooner.  */
      rc = pselect (0, &raw mut fds, std::ptr::null_mut (), std::ptr::null_mut (),
		    &raw const timeout, std::ptr::null ());

      /* Some input is available, exit the visible bell.  */
      if rc >= 0
	{ break; }
    }

  /* If window is tall, flash top and bottom line.  */
  if height > 3 * FRAME_LINE_HEIGHT (f)
    {
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left,
			      (FRAME_INTERNAL_BORDER_WIDTH (f)
			       + FRAME_TOP_MARGIN_HEIGHT (f)),
			      width as c_uint, flash_height as c_uint);
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left,
			      (height - flash_height
			       - FRAME_INTERNAL_BORDER_WIDTH (f)
			       - FRAME_BOTTOM_MARGIN_HEIGHT (f)),
			      width as c_uint, flash_height as c_uint);
    }
  else
    /* If it is short, flash it all.  */
    {
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left, FRAME_INTERNAL_BORDER_WIDTH (f),
			      width as c_uint, (height - 2
						* FRAME_INTERNAL_BORDER_WIDTH (f)) as c_uint);
    }

  android_free_gc (gc);
  flush_frame (f);

  unblock_input ();
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_ring_bell (f: *mut frame)               // C 266-277
{
  if visible_bell
    { android_flash (f); }
  else
    {
      block_input ();
      android_bell ();
      unblock_input ();
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_toggle_visible_pointer (f: *mut frame, invisible: bool)   // C 279-302
{
  let mut dpyinfo: *mut android_display_info;

  dpyinfo = FRAME_DISPLAY_INFO! (f);

  /* An hourglass cursor overrides invisibility.  */
  /* C's `goto set_invisibility` is a forward jump to the trailing statement, so the port writes
     the labelled block with a `break` (PORTING.md 2.2, the androidmenu.rs `'finish` shape).  */
  'set_invisibility: {
    if (*(FRAME_ANDROID_OUTPUT! (f) as *mut android_output)).hourglass
      { break 'set_invisibility; }

    if (*dpyinfo).invisible_cursor == 0
      { (*dpyinfo).invisible_cursor = make_invisible_cursor (dpyinfo); }

    if invisible
      { android_define_cursor (FRAME_ANDROID_WINDOW! (f),
			       (*dpyinfo).invisible_cursor); }
    else
      { android_define_cursor (FRAME_ANDROID_WINDOW! (f),
			       (*((*f).output_data.android as *mut android_output)).current_cursor); }
  }

  pointer_invisible! (f, invisible);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_toggle_invisible_pointer (f: *mut frame, invisible: bool)
{                                                                    // C 304-310
  block_input ();
  android_toggle_visible_pointer (f, invisible);
  unblock_input ();
}

/* Start an update of frame F.  This function is installed as a hook
   for update_begin, i.e. it is called when update_begin is called.
   This function is called prior to calls to gui_update_window_begin
   for each window being updated.  Currently, there is nothing to do
   here because all interesting stuff is done on a window basis.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_update_begin (f: *mut frame)            // C 318-324
{
  /* The frame is no longer complete, as it is in the midst of an
     update.  */
  /* C: `FRAME_ANDROID_COMPLETE_P (f) = false;` - C's macro *is* `((f)->output_data.android->complete)`,
     a place, and the port's folded byte has no such field: the writer site spells the mask
     (PORTING.md P-80, the `cm.rs` convention; macro-body row for C's member name).  */
  (*( (*f).output_data.android as *mut android_output )).bits &= !ANDROID_OUTPUT_COMPLETE;
}

/* End update of frame F.  This function is installed as a hook in
   update_end.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_update_end (f: *mut frame)              // C 329-334
{
  /* Mouse highlight may be displayed again.  */
  /* C 3039: `hlinfo->mouse_face_defer = false;` - the member is one of C 3019-3023's three
     `bool_bf : 1` flags the mirror folds and names through accessors (PORTING.md P-80, the
     `get_mouse_face_defer`/`set_mouse_face_defer` pair at `include/dispextern.rs` 2114/2117).  */
  set_mouse_face_defer (MOUSE_HL_INFO! (f), false);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn show_back_buffer (f: *mut frame)                           // C 336-348
{
  let mut swap_info: android_swap_info = std::mem::zeroed ();   // C 338 (PORTING.md 1.5)

  memset (&raw mut swap_info as *mut c_void, 0,
	  std::mem::size_of::<android_swap_info> ());
  swap_info.swap_window = FRAME_ANDROID_WINDOW! (f);
  swap_info.swap_action = ANDROID_COPIED;
  android_swap_buffers (&raw mut swap_info, 1);

  /* Now the back buffer no longer needs to be flipped.  */
  /* C 347: `FRAME_ANDROID_NEED_BUFFER_FLIP (f) = false;` - the same folded bit as above,
     written through the mask (PORTING.md P-80).  */
  (*( (*f).output_data.android as *mut android_output )).bits &= !ANDROID_OUTPUT_NEED_BUFFER_FLIP;
}

/* Flip back buffers on F if it has undrawn content.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_flush_dirty_back_buffer_on (f: *mut frame)         // C 350-364
{
  if FRAME_GARBAGED_P (f)
      || buffer_flipping_blocked_p ()
      /* If the frame is not already up to date, do not flush buffers
	 on input, as that will result in flicker.  */
      || !FRAME_ANDROID_COMPLETE_P! (f)
      || !FRAME_ANDROID_NEED_BUFFER_FLIP! (f)
    { return; }

  show_back_buffer (f);
}

/* Convert between the modifier bits Android uses and the modifier
   bits Emacs uses.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_android_to_emacs_modifiers (dpyinfo: *mut android_display_info,
                                              state: c_int) -> c_int   // C 369-393
{
  let mut mod_ctrl: c_int = ctrl_modifier;
  let mut mod_meta: c_int = meta_modifier;
  let mut mod_alt: c_int  = alt_modifier;
  let mut mod_super: c_int = super_modifier;
  let mut tem: Lisp_Object;

  tem = Fget (Vx_ctrl_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_ctrl = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }
  tem = Fget (Vx_alt_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_alt = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }
  tem = Fget (Vx_meta_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_meta = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }
  tem = Fget (Vx_super_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_super = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }

  return (if (state & ANDROID_CONTROL_MASK) != 0 { mod_ctrl } else { 0 })
	  | (if (state & ANDROID_SHIFT_MASK) != 0 { shift_modifier } else { 0 })
	  | (if (state & ANDROID_ALT_MASK)   != 0 { mod_meta } else { 0 })
	  | (if (state & ANDROID_SUPER_MASK) != 0 { mod_super } else { 0 })
	  | (if (state & ANDROID_META_MASK)  != 0 { mod_alt } else { 0 });
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_emacs_to_android_modifiers (dpyinfo: *mut android_display_info,
                                              state: intmax_t) -> c_int   // C 395-419
{
  let mut mod_ctrl: EMACS_INT  = ctrl_modifier as EMACS_INT;
  let mut mod_meta: EMACS_INT  = meta_modifier as EMACS_INT;
  let mut mod_alt: EMACS_INT   = alt_modifier as EMACS_INT;
  let mut mod_super: EMACS_INT = super_modifier as EMACS_INT;
  let mut tem: Lisp_Object;

  tem = Fget (Vx_ctrl_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_ctrl = XFIXNUM (tem); }
  tem = Fget (Vx_alt_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_alt = XFIXNUM (tem); }
  tem = Fget (Vx_meta_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_meta = XFIXNUM (tem); }
  tem = Fget (Vx_super_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_super = XFIXNUM (tem); }

  return (if (state & (mod_ctrl as intmax_t))  != 0 { ANDROID_CONTROL_MASK } else { 0 })
	  | (if (state & (shift_modifier as intmax_t)) != 0 { ANDROID_SHIFT_MASK } else { 0 })
	  | (if (state & (mod_meta as intmax_t))  != 0 { ANDROID_ALT_MASK   } else { 0 })
	  | (if (state & (mod_super as intmax_t)) != 0 { ANDROID_SUPER_MASK } else { 0 })
	  | (if (state & (mod_alt as intmax_t))   != 0 { ANDROID_META_MASK  } else { 0 });
}

/* C 421: `static void android_frame_rehighlight (struct android_display_info *);` - a forward
   declaration of a `static` function defined at C 2127, past this slice.  Rust has no forward
   declarations (PORTING.md 0.1-12: a declaration plus a definition is `E0428`, and items are
   order-independent), so the line has no counterpart here; C 458's call is written as C writes it
   and `docs/ports/androidterm.md` names it as the first thing the next slice must land.
   C: static void android_frame_rehighlight (struct android_display_info *);  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_lower_frame (f: *mut frame)                        // C 423-427
{
  android_lower_window (FRAME_ANDROID_WINDOW! (f));
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_raise_frame (f: *mut frame)                        // C 429-433
{
  android_raise_window (FRAME_ANDROID_WINDOW! (f));
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_new_focus_frame (dpyinfo: *mut android_display_info, frame: *mut frame)
{                                                                    // C 435-459
  let mut old_focus: *mut frame;

  old_focus = (*dpyinfo).focus_frame;

  if frame != (*dpyinfo).focus_frame
    {
      /* Set this before calling other routines, so that they see
	 the correct value of x_focus_frame.  */
      (*dpyinfo).focus_frame = frame;

      if !old_focus.is_null () && auto_lower! (old_focus)
	{ android_lower_frame (old_focus); }

      if !(*dpyinfo).focus_frame.is_null () && auto_raise! ((*dpyinfo).focus_frame)
	{ (*dpyinfo).pending_autoraise_frame = (*dpyinfo).focus_frame; }
      else
	{ (*dpyinfo).pending_autoraise_frame = std::ptr::null_mut (); }
    }

  android_frame_rehighlight (dpyinfo);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_focus_changed (r#type: c_int, state: c_int,
                                 dpyinfo: *mut android_display_info,
                                 frame: *mut frame, bufp: *mut input_event)   // C 461-494
{
  if r#type == ANDROID_FOCUS_IN
    {
      if (*dpyinfo).x_focus_event_frame != frame
        {
          android_new_focus_frame (dpyinfo, frame);
          (*dpyinfo).x_focus_event_frame = frame;
          input_event_bits! (kind, bufp, FOCUS_IN_EVENT);
          XSETFRAME! ((*bufp).frame_or_window, frame);
        }

      (*((*frame).output_data.android as *mut android_output)).focus_state |= state;
    }
  else if r#type == ANDROID_FOCUS_OUT
    {
      (*((*frame).output_data.android as *mut android_output)).focus_state &= !state;

      if (*dpyinfo).x_focus_event_frame == frame
        {
          (*dpyinfo).x_focus_event_frame = std::ptr::null_mut ();
          android_new_focus_frame (dpyinfo, std::ptr::null_mut ());

          input_event_bits! (kind, bufp, FOCUS_OUT_EVENT);
          XSETFRAME! ((*bufp).frame_or_window, frame);
        }

      if pointer_invisible! (frame)
        { android_toggle_invisible_pointer (frame, false); }
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_detect_focus_change (dpyinfo: *mut android_display_info,
                                       frame: *mut frame,
                                       event: *mut android_event,
                                       bufp: *mut input_event)       // C 496-516
{
  if frame.is_null ()
    { return; }

  /* C 502's `switch (event->type)` with a shared `case`, an empty `default` and a `break` becomes
     the `match` below (PORTING.md 2.2); Rust has no `switch`/`case`/`default`/`break` here.  */
  match (*event).r#type
    {
    ANDROID_FOCUS_IN | ANDROID_FOCUS_OUT =>
      android_focus_changed ((*event).r#type, FOCUS_EXPLICIT, dpyinfo, frame, bufp),

    _ => {}
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_note_mouse_movement (frame: *mut frame,
                                       event: *mut android_motion_event) -> bool   // C 518-549
{
  let mut dpyinfo: *mut android_display_info;
  let mut r: *mut Emacs_Rectangle;

  if FRAME_ANDROID_OUTPUT! (frame).is_null ()
    { return false; }

  dpyinfo = FRAME_DISPLAY_INFO! (frame);
  (*dpyinfo).last_mouse_motion_frame = frame;
  (*dpyinfo).last_mouse_motion_x = (*event).x;
  (*dpyinfo).last_mouse_motion_y = (*event).y;
  (*dpyinfo).last_mouse_movement_time = (*event).time;

  /* Has the mouse moved off the glyph it was on at the last sighting?  */
  r = &raw mut (*dpyinfo).last_mouse_glyph;
  if frame != (*dpyinfo).last_mouse_glyph_frame
      || (*event).x < (*r).x || (*event).x >= (*r).x + (*r).width as c_int
      || (*event).y < (*r).y || (*event).y >= (*r).y + (*r).height as c_int
    {
      mouse_moved! (frame, true);
      note_mouse_highlight (frame, (*event).x, (*event).y);
      /* Remember which glyph we're now on.  */
      remember_mouse_glyph (frame, (*event).x, (*event).y, r);
      (*dpyinfo).last_mouse_glyph_frame = frame;
      return true;
    }

  return false;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn mouse_or_wdesc_frame (dpyinfo: *mut android_display_info, wdesc: c_int) -> *mut frame
{                                                                    // C 551-573
  let mut lm_f: *mut frame = if gui_mouse_grabbed (dpyinfo)
    { (*dpyinfo).last_mouse_frame } else { std::ptr::null_mut () };

  if !lm_f.is_null () && !EQ (track_mouse, Qdropping ())
      && !EQ (track_mouse, Qdrag_source ())
    { return lm_f; }
  else
    {
      let mut w_f: *mut frame = android_window_to_frame (dpyinfo, wdesc as android_window);

      /* Do not return a tooltip frame.  */
      if w_f.is_null () || FRAME_TOOLTIP_P (w_f)
	{ return if EQ (track_mouse, Qdropping ()) { lm_f } else { std::ptr::null_mut () }; }
      else
	/* When dropping it would be probably nice to raise w_f
	   here.  */
	{ return w_f; }
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_construct_mouse_click (result: *mut input_event,
                                         event: *mut android_button_event,
                                         f: *mut frame) -> Lisp_Object   // C 575-602
{
  let mut dpyinfo: *mut android_display_info;
  let mut x: c_int;
  let mut y: c_int;

  dpyinfo = FRAME_DISPLAY_INFO! (f);
  x = (*event).x;
  y = (*event).y;

  /* Make the event type NO_EVENT; we'll change that when we decide
     otherwise.  */
  input_event_bits! (kind, result, MOUSE_CLICK_EVENT);
  (*result).code = (*event).button - 1;
  (*result).timestamp = (*event).time;
  (*result).modifiers = (android_android_to_emacs_modifiers (dpyinfo,
							     (*event).state as c_int)
			  | (if (*event).r#type == ANDROID_BUTTON_RELEASE
			     { up_modifier } else { down_modifier })) as c_uint;

  XSETINT! ((*result).x, x as EMACS_INT);
  XSETINT! ((*result).y, y as EMACS_INT);
  XSETFRAME! ((*result).frame_or_window, f);
  (*result).arg = Qnil ();
  return Qnil ();
}

/* Generate a TOUCHSCREEN_UPDATE_EVENT for all pressed tools in FRAME.
   Return the event in IE.  Do not set IE->timestamp, as that is left
   to the caller.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_update_tools (f: *mut frame, ie: *mut input_event)   // C 608-631
{
  let mut touchpoint: *mut android_touch_point;

  input_event_bits! (kind, ie, TOUCHSCREEN_UPDATE_EVENT);
  XSETFRAME! ((*ie).frame_or_window, f);
  (*ie).arg = Qnil ();

  /* Build the list of active touches.  */
  touchpoint = (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).touch_points;
  while !touchpoint.is_null ()
    {
      /* C's `continue` jumps to the `for` increment; the port's labelled block puts that
	 increment after the block, so it still runs (frame.rs 2836-2845).  */
      'continue_: {
	/* Skip touch points which originated on the tool bar.  */

	if (*touchpoint).tool_bar_p
	  { break 'continue_; }

	(*ie).arg = Fcons (list3i ((*touchpoint).x as EMACS_INT,
				   (*touchpoint).y as EMACS_INT,
				   (*touchpoint).tool_id as EMACS_INT),
			   (*ie).arg);
      }

      touchpoint = (*touchpoint).next;
    }
}

/* Find and return an existing tool pressed against FRAME, identified
   by POINTER_ID.  Return NULL if no tool by that ID was found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_find_tool (f: *mut frame, pointer_id: c_int) -> *mut android_touch_point
{                                                                    // C 636-649
  let mut touchpoint: *mut android_touch_point;

  touchpoint = (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).touch_points;
  while !touchpoint.is_null ()
    {
      if (*touchpoint).tool_id == pointer_id
	{ return touchpoint; }

      touchpoint = (*touchpoint).next;
    }

  return std::ptr::null_mut ();
}

/* Decode STRING, an array of N little endian UTF-16 characters, into
   a Lisp string.  Return Qnil if the string is too large, and the
   encoded string otherwise.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_decode_utf16 (utf16: *mut c_ushort, n: size_t) -> Lisp_Object   // C 655-675
{
  let mut coding: coding_system = std::mem::zeroed ();   // C 660 (PORTING.md 1.5)
  let mut size: ptrdiff_t = 0;                           // C 661 (PORTING.md 1.5)

  /* C 662: `if (ckd_mul (&size, n, sizeof *utf16)) return Qnil;` - the P-15
     `ckd_mul`/`overflowing_mul` pair (docs/ports/token-aliases.tsv 13).  */
  let (size_, mul_overflow) = (n as ptrdiff_t)
    .overflowing_mul (std::mem::size_of::<c_ushort> () as ptrdiff_t);
  size = size_;
  if mul_overflow
    { return Qnil (); }

  /* Set up the coding system.  Decoding a UTF-16 string (with no BOM)
     should not signal.  */

  memset (&raw mut coding as *mut c_void, 0, std::mem::size_of::<coding_system> ());

  setup_coding_system (Qutf_16le (), &raw mut coding);
  coding.source = utf16 as *const u8;
  decode_coding_object (&raw mut coding, Qnil (), 0, 0, size,
			size, Qt ());

  return coding.dst_object;
}

/* Handle a cursor update request for F from the input method.
   MODE specifies whether or not an update should be sent immediately,
   and whether or not they are needed in the future.

   If MODE & ANDROID_CURSOR_UPDATE_IMMEDIATE, report the position of
   F's old selected window's phys cursor now.

   If MODE & ANDROID_CURSOR_UPDATE_MONITOR, set
   `need_cursor_updates'.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_request_cursor_updates (f: *mut frame, mode: c_int)   // C 687-708
{
  let mut w: *mut window;

  if (mode & ANDROID_CURSOR_UPDATE_IMMEDIATE) != 0
      && WINDOWP (if WINDOW_LIVE_P ((*f).old_selected_window)
		  { (*f).old_selected_window } else { (*f).selected_window })
    {
      /* Prefer the old selected window, as its selection is what was
	 reported to the IME previously.  */

      w = XWINDOW (if WINDOW_LIVE_P ((*f).old_selected_window)
		   { (*f).old_selected_window } else { (*f).selected_window });
      android_set_preeditarea (w as *mut c_void, (*w).cursor.x, (*w).cursor.y);
    }

  /* C 707-708: `FRAME_OUTPUT_DATA (f)->need_cursor_updates
     = (mode & ANDROID_CURSOR_UPDATE_MONITOR);` - the folded bit's value write (the member is one
     bit wide, so C's int expression lands as 0/1; PORTING.md P-80 spells it as the two arms).  */
  if (mode & ANDROID_CURSOR_UPDATE_MONITOR) != 0
    { (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).bits |= ANDROID_OUTPUT_NEED_CURSOR_UPDATES; }
  else
    { (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).bits &= !ANDROID_OUTPUT_NEED_CURSOR_UPDATES; }
}

/* Handle a single input method event EVENT, delivered to the frame
   F.

   Perform the text conversion action specified inside.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_handle_ime_event (event: *mut android_event, f: *mut frame)   // C 716-829
{
  let mut text: Lisp_Object = Qnil ();   // C 718: `Lisp_Object text UNINIT;` (PORTING.md 1.5)
  let mut output: *mut android_output;

  /* First, decode the text if necessary.  */

  /* C 722's `switch (event->ime.operation)`: three `case`s share one body and a `break`, and the
     `default` is empty (PORTING.md 2.2); the `match` below has no `switch`/`case`/`break`.  */
  match (*event).ime.operation
    {
    ANDROID_IME_COMMIT_TEXT
    | ANDROID_IME_SET_COMPOSING_TEXT
    | ANDROID_IME_REPLACE_TEXT =>
      {
	text = android_decode_utf16 ((*event).ime.text,
				     (*event).ime.length);
	xfree ((*event).ime.text as *mut c_void);

	/* Return should text be long enough that it overflows ptrdiff_t.
	   Such circumstances are detected within android_decode_utf16.  */

	if NILP (text)
	  { return; }
      }

    _ => {}
    }

  /* Finally, perform the appropriate conversion action.  */

  /* C 744's second `switch (event->ime.operation)`: one `case` per operation, each with a
     `break` (PORTING.md 2.2).  */
  match (*event).ime.operation
    {
    ANDROID_IME_COMMIT_TEXT =>
      {
	commit_text (f, text, (*event).ime.position,
		     (*event).ime.counter);
      }

    ANDROID_IME_DELETE_SURROUNDING_TEXT =>
      {
	delete_surrounding_text (f, (*event).ime.start,
				 (*event).ime.end,
				 (*event).ime.counter);
      }

    ANDROID_IME_FINISH_COMPOSING_TEXT =>
      {
	if (*event).ime.length == 2
	  {
	    output = FRAME_ANDROID_OUTPUT! (f) as *mut android_output;

	    /* A new input method has connected to Emacs.  Stop
	       reporting changes that the previous input method has
	       asked to monitor.  */

	    (*output).extracted_text_flags = 0;
	    (*output).extracted_text_token = 0;
	    (*output).extracted_text_hint = 0;
	    (*output).bits &= !ANDROID_OUTPUT_NEED_CURSOR_UPDATES;   /* C: `output->need_cursor_updates = false;` (P-80 mask) */
	  }

	finish_composing_text (f, (*event).ime.counter,
			       (*event).ime.length == 1);

	if (*event).ime.length == 2
	  {
	    /* Now cancel outstanding batch edits if a new input method
	       has connected.  */

	    (*f).conversion.batch_edit_flags = 0;
	    (*f).conversion.batch_edit_count = 0;
	  }
      }

    ANDROID_IME_SET_COMPOSING_TEXT =>
      {
	set_composing_text (f, text, (*event).ime.position,
			    (*event).ime.counter);
      }

    ANDROID_IME_SET_COMPOSING_REGION =>
      {
	set_composing_region (f, (*event).ime.start,
			      (*event).ime.end,
			      (*event).ime.counter);
      }

    ANDROID_IME_SET_POINT =>
      {
	textconv_set_point_and_mark (f, (*event).ime.start,
				     (*event).ime.end,
				     (*event).ime.counter);
      }

    ANDROID_IME_START_BATCH_EDIT =>
      { start_batch_edit (f, (*event).ime.counter); }

    ANDROID_IME_END_BATCH_EDIT =>
      { end_batch_edit (f, (*event).ime.counter); }

    ANDROID_IME_REQUEST_SELECTION_UPDATE =>
      { request_point_update (f, (*event).ime.counter); }

    ANDROID_IME_REQUEST_CURSOR_UPDATES =>
      { android_request_cursor_updates (f, (*event).ime.length as c_int); }

    ANDROID_IME_REPLACE_TEXT =>
      {
	replace_text (f, (*event).ime.start, (*event).ime.end,
		      text, (*event).ime.position,
		      (*event).ime.counter);
      }

    _ => {}
    }
}
