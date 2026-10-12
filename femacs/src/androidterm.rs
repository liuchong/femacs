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

/* C 831-834: C 831-832's form feed and blank, C 833's `/* Forward declaration.  */` and C 834's
   `static void android_notify_conversion (unsigned long);`.  Rust needs no forward declaration for
   a definition later in the same unit (PORTING.md 10.10), so this slice carries the comment and no
   item - which leaves the call at C 1761 an `E0425` until C 2098's definition lands: the named
   blocker class the ledger already records for C 458's `android_frame_rehighlight`.  */

/* `Mouse_HLInfo`'s third flag bit (`dispextern.h` 3023's `bool_bf mouse_face_hidden : 1`, mask
   `include/dispextern.rs` 2139's `MOUSE_FACE_HIDDEN`).  The mirror carries the mask and the
   `mouse_face_defer` pair, and no accessor for this bit; C 959/967/1170/1172 read and write the
   member, so the unit names the pair - the `src/xterm.rs` 31036-31046 copy (whose own note points
   at `src/haikuterm.rs` 4884-4893 and `src/pgtkterm.rs` for the same macro).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! mouse_face_hidden {
  ($h:expr) => { (*$h).bits & MOUSE_FACE_HIDDEN != 0 };
  ($h:expr, $v:expr) => {
    (*$h).bits = if ($v) as c_int != 0 { (*$h).bits | MOUSE_FACE_HIDDEN }
		 else { (*$h).bits & ! MOUSE_FACE_HIDDEN } };
}

/* The eleven names of this slice's body that no mirror in this unit's include graph carries.  Each
   stands at its use site with C's prototype in a `/// C:` line (P-72 rule 4/P-88 ①), the
   `android_seq_filter`-style device slice 1 used for its own eight.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: frame.h 1515-1516: `extern void frame_size_history_extra (struct frame *, Lisp_Object,
  /// int, int, int, int, int, int);`
  pub fn frame_size_history_extra (f: *mut frame, parameter: Lisp_Object,
				   a: c_int, b: c_int, c: c_int, d: c_int,
				   e: c_int, g: c_int);
  /// C: frame.h 1911: `extern bool frame_ancestor_p (struct frame *af, struct frame *df);`
  pub fn frame_ancestor_p (a: *mut frame, b: *mut frame) -> bool;
  /// C: dispextern.h 3675: `extern void expose_frame (struct frame *, int, int, int, int);`
  pub fn expose_frame (f: *mut frame, x: c_int, y: c_int, w: c_int, h: c_int);
  /// C: dispextern.h 3668-3669: `extern Lisp_Object handle_tab_bar_click (struct frame *,
  /// int, int, bool, int);`
  pub fn handle_tab_bar_click (f: *mut frame, x: c_int, y: c_int, down_p: bool,
			       modifiers: c_int) -> Lisp_Object;
  /// C: dispextern.h 3670-3671: `extern void handle_tool_bar_click (struct frame *,
  /// int, int, bool, int);`
  pub fn handle_tool_bar_click (f: *mut frame, x: c_int, y: c_int, down_p: bool,
				modifiers: c_int);
  /// C: dispextern.h 3688: `extern void note_mouse_highlight (struct frame *, int, int);`
  /// (slice 1 carries this one; repeated here for this slice's C 1491/1575 calls.)
  /// C: dispextern.h 3690: `extern void cancel_mouse_face (struct frame *);`
  pub fn cancel_mouse_face (f: *mut frame);
  /// C: dispextern.h 3691: `extern bool clear_mouse_face (Mouse_HLInfo *);`
  pub fn clear_mouse_face (hlinfo: *mut Mouse_HLInfo) -> bool;
  /// C: dispextern.h 3924: `void change_frame_size (struct frame *, int, int, bool, bool, bool);`
  pub fn change_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
			    pretend: bool, delay: bool, safe: bool);
  /// C: keyboard.h 513-514: `extern void gen_help_event (Lisp_Object, Lisp_Object, Lisp_Object,
  /// Lisp_Object, ptrdiff_t);`
  pub fn gen_help_event (help: Lisp_Object, frame: Lisp_Object, window: Lisp_Object,
			 object: Lisp_Object, pos: ptrdiff_t);
  /// C: coding.h 737: `extern Lisp_Object from_unicode_buffer (const wchar_t *wstr);`
  pub fn from_unicode_buffer (wstr: *const wchar_t) -> Lisp_Object;
  /// C: `<math.h>`: `double fabs (double);`
  pub fn fabs (x: c_double) -> c_double;
}

/* dispextern.h 3614-3616's five objects (`keyboard.c` defines all five), which the mirror declares
   inside a module no glob import brings into this unit - the `src/xterm.rs` 31179-31192 device.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3614: `extern Lisp_Object help_echo_string, help_echo_window;`
  pub static mut help_echo_string: Lisp_Object;
  pub static mut help_echo_window: Lisp_Object;
  /// C: dispextern.h 3615: `extern Lisp_Object help_echo_object, previous_help_echo_string;`
  pub static mut help_echo_object: Lisp_Object;
  pub static mut previous_help_echo_string: Lisp_Object;
  /// C: dispextern.h 3616: `extern ptrdiff_t help_echo_pos;`
  pub static mut help_echo_pos: ptrdiff_t;
}

/* C 836-848's declarations and C 836-1892's `handle_one_android_event`, the file's largest
   function.  C takes no address of it, so it stays a Rust-ABI `unsafe fn` (PORTING.md 1.4); it is
   `static` in C, hence no `no_mangle`.  C's `switch (event->type)` (C 870) is a `match`, and C's
   `OTHER:` label (C 1860) - which is also where every `break` out of that switch lands, since the
   label's statement immediately follows it - is the labelled block `'other`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn handle_one_android_event (dpyinfo: *mut android_display_info,
				    event: *mut android_event, finish: *mut c_int,
				    hold_quit: *mut input_event) -> c_int
{
  /* C leaves every one of these uninitialised (PORTING.md 1.5): each is assigned before any path
     that reads it.  */
  let mut configureEvent: android_event = std::mem::zeroed ();          // C 841
  let mut f: *mut frame = std::ptr::null_mut ();                        // C 842
  let mut any: *mut frame = std::ptr::null_mut ();
  let mut mouse_frame: *mut frame = std::ptr::null_mut ();
  let mut hlinfo: *mut Mouse_HLInfo = std::ptr::null_mut ();            // C 843
  let mut inev: buffered_input_event = std::mem::zeroed ();             // C 844
  let mut modifiers: c_int = 0;                                         // C 845
  let mut count: c_int = 0;
  let mut do_help: c_int = 0;
  let mut touchpoint: *mut android_touch_point = std::ptr::null_mut (); // C 846
  let mut last: *mut *mut android_touch_point = std::ptr::null_mut ();
  let mut window: Lisp_Object = Qnil ();                                // C 847
  let mut scroll_height: c_int = 0;                                     // C 848
  let mut scroll_unit: f64 = 0.0;                                       // C 849
  let mut keysym: c_int = 0;                                            // C 850
  let mut nchars: ptrdiff_t = 0;                                        // C 851
  let mut i: ptrdiff_t = 0;
  let mut w: *mut window = std::ptr::null_mut ();                       // C 852

  /* C 853's `static struct android_compose_status compose_status;` is a *function-local* static:
     one object for the life of the process, zero-initialised, and the port keeps C's own storage
     class by writing a function-local `static mut` (PORTING.md 1.6).  */
  static mut compose_status: android_compose_status = android_compose_status { accent: 0, chars_matched: 0 };

  /* It is okay for this to not resemble handle_one_xevent so much.
     Differences in event handling code are much less nasty than
     stuble differences in the graphics code.  */                       // C 855-857

  count = 0;                                                           // C 859 (C's chained
  do_help = count;                                                     //  `do_help = count = 0;`)
  hlinfo = &raw mut (*dpyinfo).mouse_highlight;                         // C 860
  *finish = ANDROID_EVENT_NORMAL;                                       // C 861
  any = android_window_to_frame (dpyinfo, (*event).xany.window);        // C 862
  nchars = 0;                                                           // C 863

  if ! any.is_null () && (*any).wait_event_type == (*event).r#type     // C 865
    { (*any).wait_event_type = 0; }                                    // C 866
									//  (Indicates we got it.)

  EVENT_INIT (&raw mut inev.ie);                                        // C 868

  'other: {
  match (*event).r#type                                                // C 870
    {
    ANDROID_CONFIGURE_NOTIFY =>
      {
	configureEvent = *event;                                       // C 873

	f = android_window_to_frame (dpyinfo,                           // C 875
				     configureEvent.xconfigure.window);   // C 876

	if f.is_null ()                                                // C 878
	  { break 'other; }                                            // C 879 (C's `goto OTHER`)

	if FRAME_TOOLTIP_P (f)                                         // C 881
	  {
	    if FRAME_PIXEL_HEIGHT (f) != configureEvent.xconfigure.height   // C 883
		|| FRAME_PIXEL_WIDTH (f) != configureEvent.xconfigure.width     // C 884
	      { SET_FRAME_GARBAGED! (f); }                              // C 885

	    /* C: `FRAME_PIXEL_HEIGHT (f) = …;`/`FRAME_PIXEL_WIDTH (f) = …;` - the mirror's two
	       accessors are `fn`s (frame.rs 1036/1038), so C's l-value use is the field write
	       (`src/xterm.rs` 34289-34290 is the same pair of statements).  */
	    (*f).pixel_height = configureEvent.xconfigure.height;      // C 887
	    (*f).pixel_width = configureEvent.xconfigure.width;        // C 888
	  }

	let width: c_int = configureEvent.xconfigure.width;            // C 891
	let height: c_int = configureEvent.xconfigure.height;          // C 892

	if CONSP (frame_size_history)                                 // C 894
	  { frame_size_history_extra (f, build_string (c"ConfigureNotify".as_ptr ()),   // C 895
				      FRAME_PIXEL_WIDTH (f),           // C 896
				      FRAME_PIXEL_HEIGHT (f),          // C 897
				      width, height, (*f).new_width,   // C 898
				      (*f).new_height); }              // C 899

	/* Even if the number of character rows and columns has
	   not changed, the font size may have changed, so we need
	   to check the pixel dimensions as well.  */                  // C 901-903

	if width != FRAME_PIXEL_WIDTH (f)                              // C 905
	    || height != FRAME_PIXEL_HEIGHT (f)                        // C 906
	    || ((*f).new_size_p                                       // C 907
		&& (((*f).new_width >= 0 && width != (*f).new_width)   // C 908
		    || ((*f).new_height >= 0 && height != (*f).new_height)))   // C 909
	  {
	    change_frame_size (f, width, height, false, true, false);  // C 911
	    android_clear_under_internal_border (f);                   // C 912
	    SET_FRAME_GARBAGED! (f);                                    // C 913
	    cancel_mouse_face (f);                                     // C 914
	  }

	/* Now change the left and top position of this window.  */     // C 917

	{
	  let old_left: c_int = (*f).left_pos;                         // C 920
	  let old_top: c_int = (*f).top_pos;                           // C 921
	  let mut frame: Lisp_Object = Qnil ();                        // C 922

	  XSETFRAME! (frame, f);                                       // C 924

	  {
	    let mut root: android_window = 0;                          // C 927
	    let mut dummy_uint: c_uint = 0;                            // C 928

	    android_get_geometry (FRAME_ANDROID_WINDOW! (f),           // C 930
				  &raw mut root, &raw mut (*f).left_pos, &raw mut (*f).top_pos,   // C 931
				  &raw mut dummy_uint, &raw mut dummy_uint,   // C 932
				  &raw mut dummy_uint);                        // C 933
	  }

	  if ! FRAME_TOOLTIP_P (f)                                     // C 936
	      && (old_left != (*f).left_pos || old_top != (*f).top_pos)   // C 937
	    {
	      input_event_bits! (kind, &raw mut inev.ie, MOVE_FRAME_EVENT);   // C 939
	      XSETFRAME! (inev.ie.frame_or_window, f);                 // C 940
	    }

	  if ! f.is_null ()                                            // C 943
	      && (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).bits
	      & ANDROID_OUTPUT_NEED_CURSOR_UPDATES != 0              // C 943
	    {
	      w = XWINDOW ((*f).selected_window);                      // C 945
	      android_set_preeditarea (w as *mut c_void, (*w).cursor.x, (*w).cursor.y);   // C 946
	    }
	}

	break 'other;                                                  // C 950 (C's `goto OTHER`)
      }

    ANDROID_KEY_PRESS =>
      {
	/* Set f to any.  There are no ``outer windows'' on Android.  */   // C 954
	f = any;                                                       // C 955

	/* If mouse-highlight is an integer, input clears out
	   mouse highlighting.  */                                     // C 957-958
	if ! mouse_face_hidden! (hlinfo) && FIXNUMP (Vmouse_highlight)   // C 959
	    && (any.is_null ()                                         // C 960
		|| (! EQ ((*any).tool_bar_window, (*hlinfo).mouse_face_window)   // C 961
		    && ! EQ ((*any).tab_bar_window, (*hlinfo).mouse_face_window)))   // C 962
	  {
	    mouse_frame = (*hlinfo).mouse_face_mouse_frame as *mut frame;   // C 964

	    clear_mouse_face (hlinfo);                                 // C 966
	    mouse_face_hidden! (hlinfo, true);                       // C 967

	    if ! mouse_frame.is_null ()                                // C 969
	      { android_flush_dirty_back_buffer_on (mouse_frame); }    // C 970
	  }

	if f.is_null ()                                                // C 973
	  { break 'other; }                                            // C 974 (C's `goto OTHER`)

	if (*event).xkey.counter > 0                                   // C 976
	  /* This event was generated by `performEditorAction'.  Make
	     sure it is processed before any subsequent edits.  */     // C 977-978
	  { textconv_barrier (f, (*event).xkey.counter); }             // C 979

	let mut copy_buffer: [wchar_t; 512] = [0; 512];                // C 981
	let mut copy_bufptr: *mut wchar_t = copy_buffer.as_mut_ptr ();  // C 982
	let copy_bufsiz: c_int = 512;                                  // C 983

	(*event).xkey.state                                            // C 985
	  |= android_emacs_to_android_modifiers (dpyinfo,              // C 986
						 extra_keyboard_modifiers as intmax_t) as c_uint;   // C 987
	modifiers = (*event).xkey.state as c_int;                               // C 988

	/* In case Meta is ComposeCharacter, clear its status.  According
	   to Markus Ehrnsperger
	   Markus.Ehrnsperger@lehrstuhl-bross.physik.uni-muenchen.de this
	   enables ComposeCharacter to work whether or not it is combined
	   with Meta.  */                                              // C 990-994
	if modifiers & ANDROID_ALT_MASK != 0                           // C 995
	  { memset (&raw mut compose_status as *mut c_void, 0, std::mem::size_of::<android_compose_status> ()); }   // C 996

	/* Common for all keysym input events.  */                     // C 998
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 999
	inev.ie.modifiers                                              // C 1000
	  = android_android_to_emacs_modifiers (dpyinfo, modifiers) as c_uint;   // C 1001
	inev.ie.timestamp = (*event).xkey.time as u64;                 // C 1002

	keysym = (*event).xkey.keycode as c_int;                                // C 1004

	/* C 1110's `done_keysym:` label sits after C 1108's `goto done_keysym`, and every jump to it
	   starts inside the C 1006-1050 block, so the port's labelled block opens here and its tail
	   (the label's own statements, C 1115-1121) follows the block.  */
	'done_keysym: {
	  {
	    let mut status_return: android_lookup_status = 0;          // C 1007

	    nchars = android_wc_lookup_string (&raw mut (*event).xkey, copy_bufptr,   // C 1009
					       copy_bufsiz, &raw mut keysym,   // C 1010
					       &raw mut status_return,    // C 1011
					       &raw mut compose_status) as ptrdiff_t;  // C 1012

	    /* android_lookup_string can't be called twice, so there's no
	       way to recover from buffer overflow.  */                // C 1014-1015
	    if status_return == ANDROID_BUFFER_OVERFLOW                // C 1016
	      { break 'done_keysym; }                                  // C 1017 (C's `goto done_keysym`)
	    else if status_return == ANDROID_LOOKUP_NONE               // C 1018
	      {
		/* Don't skip preedit text events.  */                 // C 1020
		if (*event).xkey.keycode != u32::MAX                   // C 1021 (C's `(uint32_t) -1`)
		  { break 'done_keysym; }                              // C 1022
	      }
	    else if status_return == ANDROID_LOOKUP_CHARS              // C 1024
	      { keysym = ANDROID_NO_SYMBOL; }                          // C 1025
	    else if status_return != ANDROID_LOOKUP_KEYSYM             // C 1026
		     && status_return != ANDROID_LOOKUP_BOTH           // C 1027
	      { emacs_abort (); }                                      // C 1028

	    /* Deal with pre-edit text events.  On Android, these are
	       simply encoded as events with associated strings and a
	       keycode set to ``-1''.  */                             // C 1030-1032

	    if (*event).xkey.keycode == u32::MAX                       // C 1034 (C's `(uint32_t) -1`)
	      {
		input_event_bits! (kind, &raw mut inev.ie, PREEDIT_TEXT_EVENT);   // C 1036
		inev.ie.arg = Qnil ();                                 // C 1037

		/* If text was looked up, decode it and make it the
		   preedit text.  */                                   // C 1039-1040

		if status_return == ANDROID_LOOKUP_CHARS && nchars != 0   // C 1042
		  {
		    *copy_bufptr.offset (nchars) = 0;                   // C 1044
		    inev.ie.arg = from_unicode_buffer (copy_bufptr);    // C 1045
		  }

		break 'done_keysym;                                    // C 1048
	      }
	  }

	  /* If a compose sequence is in progress, we break here.
	     Otherwise, chars_matched is always 0.  */                 // C 1052-1053
	  if compose_status.chars_matched > 0 && nchars == 0           // C 1054
	    { break 'other; }                                          // C 1055 (C's `break` out of the switch)

	  memset (&raw mut compose_status as *mut c_void, 0, std::mem::size_of::<android_compose_status> ());   // C 1057

	  if nchars == 1 && *copy_bufptr >= 32                         // C 1059
	    {
	      /* Deal with characters.  */                             // C 1061

	      if *copy_bufptr < 128                                    // C 1063
		{ input_event_bits! (kind, &raw mut inev.ie, ASCII_KEYSTROKE_EVENT); }   // C 1064
	      else
		{ input_event_bits! (kind, &raw mut inev.ie, MULTIBYTE_CHAR_KEYSTROKE_EVENT); }   // C 1066

	      inev.ie.code = *copy_bufptr as c_uint;                   // C 1068
	    }
	  else if nchars < 2 && keysym != 0                            // C 1070
	    {
	      /* If the key is a modifier key, just return.  */        // C 1072
	      if ANDROID_IS_MODIFIER_KEY! (keysym)                     // C 1073
		{ break 'done_keysym; }                                // C 1074

	      /* Next, deal with special ``characters'' by giving the
		 keycode to keyboard.c.  */                            // C 1076-1077
	      input_event_bits! (kind, &raw mut inev.ie, NON_ASCII_KEYSTROKE_EVENT);   // C 1078
	      inev.ie.code = keysym as c_uint;                         // C 1079
	    }
	  else
	    {
	      /* Finally, deal with strings.  */                       // C 1083

	      i = 0;                                                   // C 1085
	      while i < nchars                                       // C 1085
		{
		  input_event_bits! (kind, &raw mut inev.ie,           // C 1087
				     if SINGLE_BYTE_CHAR_P (*copy_bufptr.offset (i) as intmax_t)   // C 1087
				       { ASCII_KEYSTROKE_EVENT }       // C 1088
				     else { MULTIBYTE_CHAR_KEYSTROKE_EVENT });   // C 1089
		  inev.ie.code = *copy_bufptr.offset (i) as c_uint;    // C 1090

		  /* If the character is actually '\n', then change this
		     to RET.  */                                       // C 1092-1093

		  if *copy_bufptr.offset (i) == b'\n' as wchar_t       // C 1095
		    {
		      input_event_bits! (kind, &raw mut inev.ie, NON_ASCII_KEYSTROKE_EVENT);   // C 1097
		      inev.ie.code = 66;                               // C 1098
		    }

		  kbd_buffer_store_buffered_event (&raw mut inev, hold_quit);   // C 1101
		  i += 1;                                              // C 1085 (the `for` step)
		}

	      count += nchars as c_int;                                // C 1104
	      input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);    // C 1105 (Already stored above.)
	    }

	  break 'done_keysym;                                          // C 1108 (C's `goto done_keysym`)
	}

	/* Now proceed to tell the input method the current position of
	   the cursor, if required.  */                               // C 1112-1113

	if ! f.is_null ()                                              // C 1115
	    && (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).bits
	      & ANDROID_OUTPUT_NEED_CURSOR_UPDATES != 0              // C 1115
	  {
	    w = XWINDOW ((*f).selected_window);                        // C 1117
	    android_set_preeditarea (w as *mut c_void, (*w).cursor.x, (*w).cursor.y);   // C 1118
	  }

	break 'other;                                                  // C 1121
      }

    ANDROID_FOCUS_IN | ANDROID_FOCUS_OUT =>
      {
	android_detect_focus_change (dpyinfo, any, event, &raw mut inev.ie);   // C 1125
	break 'other;                                                  // C 1126
      }

    /* C's `case ANDROID_WINDOW_ACTION` (C 1128) falls through into `case ANDROID_ENTER_NOTIFY`
       (C 1158) - its block has no `break` - so the two arms share one `match` arm and both bodies
       stand in C's order.  */                                        // C 1158
    ANDROID_WINDOW_ACTION | ANDROID_ENTER_NOTIFY =>
      {
	/* This is a special event sent by android_run_in_emacs_thread
	   used to make Android run stuff.  */                        // C 1130-1131

	if (*event).xaction.window == 0 && (*event).xaction.action == 0   // C 1133
	  /* Don't run queries here, as it may run inside editor
	     commands, which can expose an inconsistent view of buffer
	     contents to the input method during command execution.

	     Instead, wait for Emacs to return to `android_select'.  */   // C 1134-1138
	  { break 'other; }                                            // C 1139

	f = any;                                                       // C 1141

	if (*event).xaction.action == 0                                // C 1143
	  {
	    /* Action 0 either means that a window has been destroyed
	       and its associated frame should be as well.  */         // C 1145-1146

	    if (*event).xaction.window != 0                            // C 1148
	      {
		if f.is_null ()                                        // C 1150
		  { break 'other; }                                    // C 1151

		input_event_bits! (kind, &raw mut inev.ie, DELETE_WINDOW_EVENT);   // C 1153
		XSETFRAME! (inev.ie.frame_or_window, f);               // C 1154
	      }
	  }

	/* (C 1158's `case ANDROID_ENTER_NOTIFY:`, which control reaches by falling through.)  */
	f = any;                                                       // C 1159

	if ! f.is_null ()                                              // C 1161
	  { android_note_mouse_movement (f, &raw mut (*event).xmotion); }   // C 1162
	break 'other;                                                  // C 1163
      }

    ANDROID_MOTION_NOTIFY =>
      {
	previous_help_echo_string = help_echo_string;                  // C 1167
	help_echo_string = Qnil ();                                    // C 1168

	if mouse_face_hidden! (hlinfo)                                 // C 1170
	  {
	    mouse_face_hidden! (hlinfo, false);                       // C 1172
	    clear_mouse_face (hlinfo);                                 // C 1173
	  }

	f = any;                                                       // C 1176

	if ! f.is_null ()                                              // C 1178
	  {
	    /* Maybe generate a SELECT_WINDOW_EVENT for
	       `mouse-autoselect-window' but don't let popup menus
	       interfere with this (Bug#1261).  */                     // C 1180-1182
	    if ! NILP (Vmouse_autoselect_window)                       // C 1183
		&& popup_activated () == 0                                // C 1184
		/* Don't switch if we're currently in the minibuffer.
		   This tries to work around problems where the
		   minibuffer gets unselected unexpectedly, and where
		   you then have to move your mouse all the way down to
		   the minibuffer to select it.  */                    // C 1185-1189
		&& ! MINI_WINDOW_P (XWINDOW (selected_window))         // C 1190
		/* With `focus-follows-mouse' non-nil create an event
		   also when the target window is on another frame.  */   // C 1191-1192
		&& (f == XFRAME (selected_frame)                       // C 1193
		    || ! NILP (focus_follows_mouse))                   // C 1194
	      {
		let window: Lisp_Object                                // C 1196
		  = window_from_coordinates (f as *mut c_void, (*event).xmotion.x,    // C 1197
					     (*event).xmotion.y, std::ptr::null_mut (),    // C 1198
					     false, false, false);     // C 1199

		/* A window will be autoselected only when it is not
		   selected now and the last mouse movement event was
		   not in it.  The remainder of the code is a bit vague
		   wrt what a "window" is.  For immediate autoselection,
		   the window is usually the entire window but for GTK
		   where the scroll bars don't count.  For delayed
		   autoselection the window is usually the window's text
		   area including the margins.  */                     // C 1201-1208
		if WINDOWP (window)                                    // C 1209
		    && ! EQ (window, last_mouse_window)                // C 1210
		    && ! EQ (window, selected_window)                  // C 1211
		  {
		    input_event_bits! (kind, &raw mut inev.ie, SELECT_WINDOW_EVENT);   // C 1213
		    inev.ie.frame_or_window = window;                  // C 1214
		  }

		/* Remember the last window where we saw the mouse.  */   // C 1217
		last_mouse_window = window;                            // C 1218
	      }

	    if ! android_note_mouse_movement (f, &raw mut (*event).xmotion)   // C 1221
	      { help_echo_string = previous_help_echo_string; }        // C 1222
	  }

	/* If the contents of the global variable help_echo_string
	   has changed, generate a HELP_EVENT.  */                     // C 1225-1226
	if ! NILP (help_echo_string)                                   // C 1227
	    || ! NILP (previous_help_echo_string)                      // C 1228
	  { do_help = 1; }                                             // C 1229

	if ! f.is_null ()                                              // C 1231
	  { android_flush_dirty_back_buffer_on (f); }                  // C 1232

	break 'other;                                                  // C 1234
      }

    ANDROID_LEAVE_NOTIFY =>
      {
	f = any;                                                       // C 1237

	if ! f.is_null ()                                              // C 1239
	  {
	    /* Now clear dpyinfo->last_mouse_motion_frame, or
	       gui_redo_mouse_highlight will end up highlighting the
	       last known position of the mouse if a tooltip frame is
	       later unmapped.  */                                     // C 1241-1244

	    if f == (*dpyinfo).last_mouse_motion_frame                 // C 1246
	      { (*dpyinfo).last_mouse_motion_frame = std::ptr::null_mut (); }   // C 1247

	    /* Something similar applies to
	       dpyinfo->last_mouse_glyph_frame.  */                    // C 1249-1250
	    if f == (*dpyinfo).last_mouse_glyph_frame                  // C 1251
	      { (*dpyinfo).last_mouse_glyph_frame = std::ptr::null_mut (); }   // C 1252

	    if f == (*hlinfo).mouse_face_mouse_frame as *mut frame          // C 1254
	      {
		/* If we move outside the frame, then we're
		   certainly no longer on any text in the frame.  */   // C 1256-1257
		clear_mouse_face (hlinfo);                             // C 1258
		(*hlinfo).mouse_face_mouse_frame = std::ptr::null_mut ();   // C 1259
		android_flush_dirty_back_buffer_on (f);                // C 1260
	      }

	    /* Generate a nil HELP_EVENT to cancel a help-echo.
	       Do it only if there's something to cancel.
	       Otherwise, the startup message is cleared when
	       the mouse leaves the frame.  */                        // C 1263-1266
	    if any_help_event_p                                        // C 1267
		/* But never if `mouse-drag-and-drop-region' is in
		   progress, since that results in the tooltip being
		   dismissed when the mouse moves on top.  */          // C 1268-1270
		&& !((EQ (track_mouse, Qdrag_source ())                 // C 1271
		      || EQ (track_mouse, Qdropping ()))               // C 1272
		     && gui_mouse_grabbed (dpyinfo))                   // C 1273
	      { do_help = -1; }                                        // C 1274
	  }

	break 'other;                                                  // C 1277
      }

    ANDROID_EXPOSE =>
      {
	f = any;                                                       // C 1281

	if ! f.is_null ()                                              // C 1283
	  {
	    if ! FRAME_VISIBLE_P (f)                                   // C 1285
	      {
		(*( (*f).output_data.android as *mut android_output )).bits |= ANDROID_OUTPUT_HAS_BEEN_VISIBLE;   // C 1287 (C's `has_been_visible = true;`)
		SET_FRAME_GARBAGED! (f);                                // C 1288
	      }

	    if ! FRAME_GARBAGED_P (f)                                  // C 1291
	      {
		expose_frame (f, (*event).xexpose.x, (*event).xexpose.y,   // C 1293
			      (*event).xexpose.width, (*event).xexpose.height);   // C 1294

		/* Do not display the back buffer if F is yet being
		   updated, as this might trigger premature bitmap
		   reconfiguration.  */                                // C 1296-1298
		if FRAME_ANDROID_COMPLETE_P! (f)                        // C 1299
		  { show_back_buffer (f); }                            // C 1300
	      }
	  }

	break 'other;                                                  // C 1304
      }

    ANDROID_BUTTON_PRESS | ANDROID_BUTTON_RELEASE =>
      {
	/* If we decide we want to generate an event to be seen
	   by the rest of Emacs, we put it here.  */                   // C 1308-1309

	f = any;                                                       // C 1311

	let mut tab_bar_arg: Lisp_Object = Qnil ();                    // C 1313
	let mut tab_bar_p: bool = false;                               // C 1314
	let mut tool_bar_p: bool = false;                              // C 1315

	(*dpyinfo).last_mouse_glyph_frame = std::ptr::null_mut ();     // C 1317

	f = mouse_or_wdesc_frame (dpyinfo, (*event).xbutton.window as c_int);   // C 1319

	if ! f.is_null ()                                              // C 1321
	    && (*event).xbutton.r#type == ANDROID_BUTTON_PRESS         // C 1321
	    && popup_activated () == 0                                // C 1322
	    /* && !x_window_to_scroll_bar (event->xbutton.display, */   // C 1323
	    /* 			      event->xbutton.window, 2) */   // C 1324
	    && FRAME_NO_ACCEPT_FOCUS (f) == 0                           // C 1325
	  {
	    /* When clicking into a child frame or when clicking
	       into a parent frame with the child frame selected and
	       `no-accept-focus' is not set, select the clicked
	       frame.  */                                              // C 1327-1330
	    let hf: *mut frame = (*dpyinfo).highlight_frame;           // C 1331

	    if ! FRAME_PARENT_FRAME (f).is_null ()                      // C 1333
		|| (! hf.is_null () && frame_ancestor_p (f, hf))        // C 1333
	      {
		android_set_input_focus (FRAME_ANDROID_WINDOW! (f),    // C 1335
					 (*event).xbutton.time);       // C 1336

		if ! FRAME_PARENT_FRAME (f).is_null ()                  // C 1338
		  { android_raise_window (FRAME_ANDROID_WINDOW! (f)); }   // C 1339
	      }
	  }

	if ! f.is_null ()                                              // C 1343
	  {
	    /* Is this in the tab-bar?  */                             // C 1345
	    if WINDOWP ((*f).tab_bar_window)                           // C 1346
		&& WINDOW_TOTAL_LINES (XWINDOW ((*f).tab_bar_window)) != 0   // C 1347
	      {
		let mut window: Lisp_Object = Qnil ();                 // C 1349
		let x: c_int = (*event).xbutton.x;                     // C 1350
		let y: c_int = (*event).xbutton.y;                     // C 1351

		window = window_from_coordinates (f as *mut c_void, x, y, std::ptr::null_mut (), true, true, true);   // C 1353
		tab_bar_p = EQ (window, (*f).tab_bar_window);          // C 1354

		if tab_bar_p                                           // C 1356
		  {
		    tab_bar_arg = handle_tab_bar_click                // C 1358
		      (f, x, y, ((*event).xbutton.r#type             // C 1359
				 == ANDROID_BUTTON_PRESS),             // C 1360
		       android_android_to_emacs_modifiers (dpyinfo,   // C 1361
							   (*event).xbutton.state as c_int));   // C 1362
		    android_flush_dirty_back_buffer_on (f);            // C 1363
		  }
	      }

	    /* Is this in the tool-bar?  */                            // C 1367
	    if WINDOWP ((*f).tool_bar_window)                          // C 1368
		&& WINDOW_TOTAL_LINES (XWINDOW ((*f).tool_bar_window)) != 0   // C 1369
	      {
		let mut window: Lisp_Object = Qnil ();                 // C 1371
		let x: c_int = (*event).xbutton.x;                     // C 1372
		let y: c_int = (*event).xbutton.y;                     // C 1373

		window = window_from_coordinates (f as *mut c_void, x, y, std::ptr::null_mut (), true, true, true);   // C 1375
		tool_bar_p = (EQ (window, (*f).tool_bar_window)        // C 1376
			      && (((*event).xbutton.r#type             // C 1377
				   != ANDROID_BUTTON_RELEASE)          // C 1378
				  || (*f).last_tool_bar_item != -1));   // C 1379

		if tool_bar_p && (*event).xbutton.button < 4           // C 1381
		  {
		    handle_tool_bar_click                              // C 1383
		      (f, x, y, ((*event).xbutton.r#type               // C 1384
				 == ANDROID_BUTTON_PRESS),             // C 1385
		       android_android_to_emacs_modifiers (dpyinfo,   // C 1386
							   (*event).xbutton.state as c_int));   // C 1387
		    android_flush_dirty_back_buffer_on (f);            // C 1388
		  }
	      }

	    if !(tab_bar_p && NILP (tab_bar_arg)) && ! tool_bar_p      // C 1392
	      {
		if popup_activated () == 0                            // C 1393
		  {
		    android_construct_mouse_click (&raw mut inev.ie, &raw mut (*event).xbutton, f);   // C 1395

		    if ! NILP (tab_bar_arg)                            // C 1397
		      { inev.ie.arg = tab_bar_arg; }                   // C 1398
		  }
	      }
	  }

	if (*event).r#type == ANDROID_BUTTON_PRESS                     // C 1402
	  {
	    (*dpyinfo).grabbed |= 1 << (*event).xbutton.button;        // C 1404
	    (*dpyinfo).last_mouse_frame = f;                           // C 1405
	    if ! f.is_null () && ! tab_bar_p                           // C 1406
	      { (*f).last_tab_bar_item = -1; }                         // C 1407
	    if ! f.is_null () && ! tool_bar_p                          // C 1408
	      { (*f).last_tool_bar_item = -1; }                        // C 1409
	  }
	else
	  { (*dpyinfo).grabbed &= ! (1 << (*event).xbutton.button); }   // C 1412

	/* Ignore any mouse motion that happened before this event;
	   any subsequent mouse-movement Emacs events should reflect
	   only motion after the ButtonPress/Release.  */              // C 1414-1416
	if ! f.is_null ()                                              // C 1417
	  { mouse_moved! (f, false); }                                 // C 1418

	break 'other;                                                  // C 1420
      }

    /* Touch events.  The events here don't parallel X so much.  */   // C 1422
    ANDROID_TOUCH_DOWN =>
      {
	if any.is_null ()                                              // C 1425
	  { break 'other; }                                            // C 1426

	/* This event is sent when a tool is put on the screen.  X and Y
	   are the location of the finger, and pointer_id identifies the
	   tool for as long as it is still held down.  First, see if the
	   touch point already exists and can be reused (this shouldn't
	   happen, but be safe.)  */                                   // C 1428-1432

	touchpoint = android_find_tool (any, (*event).touch.pointer_id as c_int);   // C 1434

	if ! touchpoint.is_null ()                                     // C 1436
	  {
	    /* Simply update the tool position and send an update.  */   // C 1438
	    (*touchpoint).x = (*event).touch.x;                        // C 1439
	    (*touchpoint).y = (*event).touch.y;                        // C 1440
	    android_update_tools (any, &raw mut inev.ie);              // C 1441
	    inev.ie.timestamp = (*event).touch.time as u64;            // C 1442

	    break 'other;                                              // C 1444
	  }

	/* Otherwise, link a new touchpoint onto the output's list of
	   pressed tools.  */                                          // C 1447-1448

	touchpoint = xmalloc (std::mem::size_of::<android_touch_point> ()) as *mut android_touch_point;   // C 1450
	(*touchpoint).tool_id = (*event).touch.pointer_id as c_int;    // C 1451
	(*touchpoint).x = (*event).touch.x;                            // C 1452
	(*touchpoint).y = (*event).touch.y;                            // C 1453
	(*touchpoint).next = (*(FRAME_OUTPUT_DATA! (any) as *mut android_output)).touch_points;   // C 1454
	(*touchpoint).tool_bar_p = false;                              // C 1455
	(*(FRAME_OUTPUT_DATA! (any) as *mut android_output)).touch_points = touchpoint;   // C 1456

	/* Figure out whether or not the tool was pressed on the tool
	   bar.  Note that the code which runs when it was is more or
	   less an abuse of the mouse highlight machinery, but it works
	   well enough in practice.  */                                // C 1458-1461

	if WINDOWP ((*any).tool_bar_window)                            // C 1463
	    && WINDOW_TOTAL_LINES (XWINDOW ((*any).tool_bar_window)) != 0   // C 1464
	  {
	    let mut window: Lisp_Object = Qnil ();                     // C 1466
	    let x: c_int = (*event).touch.x;                           // C 1467
	    let y: c_int = (*event).touch.y;                           // C 1468

	    window = window_from_coordinates (any as *mut c_void, x, y, std::ptr::null_mut (), true,      // C 1470
					      true, true);             // C 1471

	    /* If this touch has started in the tool bar, do not
	       send it to Lisp.  Instead, simulate a tool bar
	       click, releasing it once it goes away.  */              // C 1473-1475

	    if EQ (window, (*any).tool_bar_window)                     // C 1477
	      {
		/* Call note_mouse_highlight on the tool bar
		   item.  Otherwise, get_tool_bar_item will
		   return 1.

		   This is not necessary when mouse-highlight is
		   nil.  */                                            // C 1479-1484

		if ! NILP (Vmouse_highlight)                           // C 1486
		  {
		    /* Clear the pointer invisible flag to always make
		       note_mouse_highlight do its thing.  */          // C 1488-1489
		    pointer_invisible! (any, false);                   // C 1490
		    note_mouse_highlight (any, x, y);                  // C 1491

		    /* Always allow future mouse motion to
		       update the mouse highlight, no matter
		       where it is.  */                               // C 1493-1495
		    memset (&raw mut (*dpyinfo).last_mouse_glyph as *mut c_void, 0,   // C 1496
			    std::mem::size_of_val (&(*dpyinfo).last_mouse_glyph));   // C 1497
		    (*dpyinfo).last_mouse_glyph_frame = any;           // C 1498
		  }

		handle_tool_bar_click (any, x, y, true, 0);            // C 1501

		/* Flush any changes made by that to the front
		   buffer.  */                                         // C 1503-1504
		android_flush_dirty_back_buffer_on (any);              // C 1505

		/* Mark the touch point as being grabbed by the tool
		   bar.  */                                            // C 1507-1508
		(*touchpoint).tool_bar_p = true;                       // C 1509
		break 'other;                                          // C 1510
	      }
	  }

	/* Now generate the Emacs event.  */                            // C 1514
	input_event_bits! (kind, &raw mut inev.ie, TOUCHSCREEN_BEGIN_EVENT);   // C 1515
	inev.ie.timestamp = (*event).touch.time as u64;                // C 1516
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1517
	XSETINT! (inev.ie.x, (*event).touch.x as isize);   // C 1518
	XSETINT! (inev.ie.y, (*event).touch.y as isize);   // C 1519
	XSETINT! (inev.ie.arg, (*event).touch.pointer_id as isize);   // C 1520

	break 'other;                                                  // C 1522
      }

    ANDROID_TOUCH_MOVE =>
      {
	if any.is_null ()                                              // C 1526
	  { break 'other; }                                            // C 1527

	/* Look for the tool that moved.  */                            // C 1529

	touchpoint = android_find_tool (any, (*event).touch.pointer_id as c_int);   // C 1531

	/* If it doesn't exist or has been grabbed by the tool bar, skip
	   processing this event.  */                                  // C 1533-1534

	if touchpoint.is_null () || (*touchpoint).tool_bar_p           // C 1536
	  { break 'other; }                                            // C 1537

	/* Otherwise, update the position and send the update event.  */   // C 1539

	(*touchpoint).x = (*event).touch.x;                            // C 1541
	(*touchpoint).y = (*event).touch.y;                            // C 1542
	android_update_tools (any, &raw mut inev.ie);                  // C 1543
	inev.ie.timestamp = (*event).touch.time as u64;                // C 1544

	break 'other;                                                  // C 1546
      }

    ANDROID_TOUCH_UP =>
      {
	if any.is_null ()                                              // C 1550
	  { break 'other; }                                            // C 1551

	/* Now find and unlink the tool in question.  */                // C 1553

	last = &raw mut (*(FRAME_OUTPUT_DATA! (any) as *mut android_output)).touch_points;   // C 1555
	loop
	  {
	    touchpoint = *last;                                        // C 1556
	    if touchpoint.is_null ()
	      { break; }
	    if (*touchpoint).tool_id == (*event).touch.pointer_id as c_int   // C 1558
	      {
		*last = (*touchpoint).next;                            // C 1560

		if (*touchpoint).tool_bar_p                            // C 1562
		  {
		    xfree (touchpoint as *mut c_void);                 // C 1564

		    /* Do what is necessary to release the tool bar and
		       possibly trigger a click.  */                   // C 1566-1567

		    if (*any).last_tool_bar_item != -1                 // C 1569
		      {
			handle_tool_bar_click (any, (*event).touch.x,   // C 1570
					       (*event).touch.y, false,   // C 1571
					       0);                      // C 1572
		      }

		    /* Cancel any outstanding mouse highlight.  */     // C 1574
		    note_mouse_highlight (any, -1, -1);                // C 1575
		    android_flush_dirty_back_buffer_on (any);          // C 1576

		    break 'other;                                      // C 1578
		  }

		/* The tool was unlinked.  Free it and generate the
		   appropriate Emacs event (assuming that it was not
		   grabbed by the tool bar).  */                      // C 1581-1583
		xfree (touchpoint as *mut c_void);                     // C 1584

		input_event_bits! (kind, &raw mut inev.ie, TOUCHSCREEN_END_EVENT);   // C 1586
		inev.ie.timestamp = (*event).touch.time as u64;        // C 1587

		/* Report whether the sequence has been canceled.  */   // C 1589

		if (*event).touch.flags & ANDROID_TOUCH_SEQUENCE_CANCELED != 0   // C 1591
		  { inev.ie.modifiers = 1; }                           // C 1592

		XSETFRAME! (inev.ie.frame_or_window, any);             // C 1594
		XSETINT! (inev.ie.x, (*event).touch.x as isize);   // C 1595
		XSETINT! (inev.ie.y, (*event).touch.y as isize);   // C 1596
		XSETINT! (inev.ie.arg, (*event).touch.pointer_id as isize);   // C 1597

		/* Break out of the loop.  */                          // C 1599
		break 'other;                                          // C 1600
	      }
	    else
	      { last = &raw mut (*touchpoint).next; }                  // C 1603
	  }

	/* No touch point was found.  This shouldn't happen.  */        // C 1606
	break 'other;                                                  // C 1607
      }

    /* Wheel motion.  The events here don't parallel X because
       Android doesn't have scroll valuators.  */                    // C 1609-1610

    ANDROID_WHEEL =>
      {
	if any.is_null ()                                              // C 1614
	  { break 'other; }                                            // C 1615

	if fabs ((*event).wheel.x_delta) > 0.0                         // C 1617
	    || fabs ((*event).wheel.y_delta) > 0.0                     // C 1618
	  {
	    if mwheel_coalesce_scroll_events                           // C 1620
	      {
		if signbit! ((*event).wheel.x_delta)                    // C 1622
		    != signbit! (wheel_event_x)                         // C 1623
		  { wheel_event_x = 0.0; }                             // C 1624

		if signbit! ((*event).wheel.y_delta)                    // C 1626
		    != signbit! (wheel_event_y)                         // C 1627
		  { wheel_event_y = 0.0; }                             // C 1628

		/* Tally up deltas until one of them exceeds 1.0.  */   // C 1630
		wheel_event_x += (*event).wheel.x_delta;               // C 1631
		wheel_event_y += (*event).wheel.y_delta;               // C 1632

		if fabs (wheel_event_x) < 1.0                          // C 1634
		    && fabs (wheel_event_y) < 1.0                      // C 1635
		  { break 'other; }                                    // C 1636
	      }
	    else
	      {
		/* Use the deltas in the event.  */                    // C 1640
		wheel_event_x = (*event).wheel.x_delta;                // C 1641
		wheel_event_y = (*event).wheel.y_delta;                // C 1642
	      }

	    /* Determine what kind of event to send.  */               // C 1645
	    input_event_bits! (kind, &raw mut inev.ie,                 // C 1646
			       if fabs (wheel_event_y)                 // C 1646
				  >= fabs (wheel_event_x)              // C 1647
				 { WHEEL_EVENT } else { HORIZ_WHEEL_EVENT });   // C 1648
	    inev.ie.timestamp = (*event).wheel.time as u64;            // C 1649

	    /* Set the event coordinates.  */                          // C 1651
	    XSETINT! (inev.ie.x, (*event).wheel.x as isize);   // C 1652
	    XSETINT! (inev.ie.y, (*event).wheel.y as isize);   // C 1653

	    /* Set the frame.  */                                      // C 1655
	    XSETFRAME! (inev.ie.frame_or_window, any);                 // C 1656

	    /* Figure out the scroll direction.  */                    // C 1658
	    inev.ie.modifiers = (if signbit! (if fabs (wheel_event_x)   // C 1659
					       >= fabs (wheel_event_y)   // C 1660
					      { wheel_event_x }        // C 1661
					    else { wheel_event_y })    // C 1662
				 { down_modifier as c_uint } else { up_modifier as c_uint });   // C 1663

	    /* Figure out how much to scale the deltas by.  */         // C 1665
	    window = window_from_coordinates (any as *mut c_void, (*event).wheel.x,   // C 1666
					      (*event).wheel.y, std::ptr::null_mut (),   // C 1667
					      false, false, false);    // C 1668

	    if WINDOWP (window)                                        // C 1670
	      { scroll_height = (*XWINDOW (window)).pixel_height; }    // C 1671
	    else
	      /* EVENT_X and EVENT_Y can be outside the
		 frame if F holds the input grab, so fall
		 back to the height of the frame instead.  */          // C 1673-1675
	      { scroll_height = FRAME_PIXEL_HEIGHT (any); }            // C 1676

	    scroll_unit = pow (scroll_height as f64, 2.0 / 3.0);       // C 1678

	    /* Add the keyboard modifiers.  */                         // C 1680
	    inev.ie.modifiers                                          // C 1681
	      |= android_android_to_emacs_modifiers (dpyinfo,          // C 1682
						     (*event).wheel.state as c_int) as c_uint;   // C 1683

	    /* Finally include the scroll deltas.  */                  // C 1685
	    inev.ie.arg = list3 (Qnil (),                              // C 1686
				 make_float (wheel_event_x              // C 1687
					     * scroll_unit),           // C 1688
				 make_float (wheel_event_y              // C 1689
					     * scroll_unit));          // C 1690

	    wheel_event_x = 0.0;                                       // C 1692
	    wheel_event_y = 0.0;                                       // C 1693
	  }

	break 'other;                                                  // C 1696
      }

    /* Iconification.  This is vastly simpler than on X.  */           // C 1698
    ANDROID_ICONIFIED =>
      {
	if any.is_null ()                                              // C 1701
	  { break 'other; }                                            // C 1702

	if FRAME_ICONIFIED_P (any)                                     // C 1704
	  { break 'other; }                                            // C 1705

	SET_FRAME_VISIBLE! (any, false);                                // C 1707
	SET_FRAME_ICONIFIED (any, 1);                                  // C 1708

	input_event_bits! (kind, &raw mut inev.ie, ICONIFY_EVENT);      // C 1710
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1711
	break 'other;                                                  // C 1712
      }

    ANDROID_DEICONIFIED =>
      {
	if any.is_null ()                                              // C 1716
	  { break 'other; }                                            // C 1717

	if ! FRAME_ICONIFIED_P (any)                                   // C 1719
	  { break 'other; }                                            // C 1720

	SET_FRAME_VISIBLE! (any, true);                                 // C 1722
	SET_FRAME_ICONIFIED (any, 0);                                  // C 1723

	input_event_bits! (kind, &raw mut inev.ie, DEICONIFY_EVENT);    // C 1725
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1726
	break 'other;                                                  // C 1727
      }

    /* Context menu handling.  */                                      // C 1729
    ANDROID_CONTEXT_MENU =>
      {
	if (*dpyinfo).menu_event_id == -1                              // C 1732
	    /* Previously displayed popup menus might generate events
	       after dismissal, which might interfere.
	       `current_menu_serial' is always set to an identifier
	       identifying the last context menu to be displayed.  */   // C 1733-1736
	    && (*event).menu.menu_event_serial as c_uint == current_menu_serial   // C 1737
	  { (*dpyinfo).menu_event_id = (*event).menu.menu_event_id; }   // C 1738

	break 'other;                                                  // C 1740
      }

    /* Input method events.  textconv.c functions are called here to
       queue events, which are then executed in a safe context
       inside keyboard.c.  */                                         // C 1742-1744
    ANDROID_INPUT_METHOD =>
      {
	if any.is_null ()                                              // C 1747
	  {
	    /* Free any text allocated for this event.  */             // C 1749
	    xfree ((*event).ime.text as *mut c_void);                  // C 1750

	    /* If edits associated with this event haven't been
	       processed yet, signal their completion to avoid delays
	       the next time a call to `android_sync_edit' is made.

	       If events for a deleted frame are interleaved with events
	       for another frame, the edit counter may be prematurely
	       incremented before edits associated with the other frames
	       are processed.  This is not a problem in practice.  */   // C 1752-1759

	    android_notify_conversion ((*event).ime.counter);          // C 1761
	  }
	else
	  { android_handle_ime_event (event, any); }                   // C 1764

	break 'other;                                                  // C 1766
      }

    ANDROID_DND_DRAG_EVENT =>
      {
	if any.is_null ()                                              // C 1770
	  { break 'other; }                                            // C 1771

	/* Generate a drag and drop event to convey its position.  */   // C 1773
	input_event_bits! (kind, &raw mut inev.ie, DRAG_N_DROP_EVENT);   // C 1774
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1775
	inev.ie.timestamp = ANDROID_CURRENT_TIME as u64;               // C 1776
	XSETINT! (inev.ie.x, (*event).dnd.x as isize);   // C 1777
	XSETINT! (inev.ie.y, (*event).dnd.y as isize);   // C 1778
	inev.ie.arg = Fcons (inev.ie.x, inev.ie.y);                    // C 1779
	break 'other;                                                  // C 1780
      }

    ANDROID_DND_URI_EVENT | ANDROID_DND_TEXT_EVENT =>
      {
	if any.is_null ()                                              // C 1785
	  {
	    free ((*event).dnd.uri_or_string as *mut c_void);          // C 1787
	    break 'other;                                              // C 1788
	  }

	/* An item was dropped over ANY, and is a file in the form of a
	   content or file URI or a string to be inserted.  Generate an
	   event with this information.  */                            // C 1791-1793

	input_event_bits! (kind, &raw mut inev.ie, DRAG_N_DROP_EVENT);   // C 1795
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1796
	inev.ie.timestamp = ANDROID_CURRENT_TIME as u64;               // C 1797
	XSETINT! (inev.ie.x, (*event).dnd.x as isize);   // C 1798
	XSETINT! (inev.ie.y, (*event).dnd.y as isize);   // C 1799
	inev.ie.arg = Fcons (if (*event).r#type == ANDROID_DND_TEXT_EVENT   // C 1800
			       { Qtext () } else { Quri () },          // C 1801
			     android_decode_utf16 ((*event).dnd.uri_or_string,   // C 1802
						   (*event).dnd.length));   // C 1803
	free ((*event).dnd.uri_or_string as *mut c_void);              // C 1804
	break 'other;                                                  // C 1805
      }

    ANDROID_NOTIFICATION_DELETED | ANDROID_NOTIFICATION_ACTION =>
      {
	if (*event).notification.r#type == ANDROID_NOTIFICATION_DELETED   // C 1810
	  { android_notification_deleted (&raw mut (*event).notification as *mut c_void, &raw mut inev.ie); }   // C 1811
	else
	  {
	    let mut action: Lisp_Object = Qnil ();                     // C 1814

	    action = android_decode_utf16 ((*event).notification.action,   // C 1816
					   (*event).notification.length);   // C 1817
	    android_notification_action (&raw mut (*event).notification as *mut c_void, &raw mut inev.ie,   // C 1818
					 action);                      // C 1819
	  }

	/* Free dynamically allocated data.  */                        // C 1822
	free ((*event).notification.tag as *mut c_void);               // C 1823
	free ((*event).notification.action as *mut c_void);            // C 1824
	break 'other;                                                  // C 1825
      }

    ANDROID_CONFIGURATION_CHANGED =>
      {
	if (*event).config.detail == ANDROID_PIXEL_DENSITY_CHANGED     // C 1829
	  {
	    /* Update the display configuration from the event.  */    // C 1831
	    (*dpyinfo).resx = (*event).config.u.pixel_density.dpi_x;   // C 1832
	    (*dpyinfo).resy = (*event).config.u.pixel_density.dpi_y;   // C 1833
	    (*dpyinfo).font_resolution                                // C 1834
	      = (*event).config.u.pixel_density.dpi_scaled;            // C 1835
	    /* C 1836-1841's `#if notdef` block (the `__android_log_print` trace) is dead text in
	       every configuration, so it is recorded in the ledger and not transcribed (the slice-1
	       `#if false` rule).  */
	    input_event_bits! (kind, &raw mut inev.ie, CONFIG_CHANGED_EVENT);   // C 1842
	    inev.ie.frame_or_window = XCAR ((*dpyinfo).name_list_element);   // C 1843
	    inev.ie.arg = Qfont_render ();                             // C 1844
	  }
	else if (*event).config.detail == ANDROID_UI_MODE_CHANGED      // C 1846
	  {
	    android_ui_mode = (*event).config.u.ui_mode;                 // C 1848
	    input_event_bits! (kind, &raw mut inev.ie, TOOLKIT_THEME_CHANGED_EVENT);   // C 1849
	    inev.ie.arg = if android_ui_mode == UI_MODE_NIGHT_YES      // C 1850
			    { Qdark () } else { Qlight () };           // C 1851
	  }

	break 'other;                                                  // C 1854
      }

    _ => { break 'other; }                                             // C 1856-1857
    }
  }

  /* OTHER: */
  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT            // C 1861
    {
      kbd_buffer_store_buffered_event (&raw mut inev, hold_quit);      // C 1863
      count += 1;                                                      // C 1864
    }

  if do_help != 0                                                      // C 1867
      && !(! hold_quit.is_null () && (*hold_quit).bits != NO_EVENT as u32)    // C 1868
    {
      let mut frame: Lisp_Object;                                      // C 1870

      if ! f.is_null ()                                                // C 1872
	{ XSETFRAME! (frame, f); }                                     // C 1873
      else
	{ frame = Qnil (); }                                           // C 1875
									// C 1870 (C leaves `frame`
									//  uninitialised; every path
									//  assigns it)

      if do_help > 0                                                   // C 1877
	{
	  any_help_event_p = true;                                     // C 1879
	  gen_help_event (help_echo_string, frame, help_echo_window,   // C 1880
			  help_echo_object, help_echo_pos);            // C 1881
	}
      else
	{
	  help_echo_string = Qnil ();                                  // C 1885
	  gen_help_event (Qnil (), frame, Qnil (), Qnil (), 0);        // C 1886
	}
      count += 1;                                                      // C 1888
    }

  return count;                                                        // C 1891
}

/* C 1894-2238: the run of frame, terminal and redisplay-interface callbacks that follows the event
   dispatcher - the event reader, the buffer-flipping pair, the four colour helpers, the
   focus/highlight hooks, the visibility hooks and the iconification hook.  Four names this text
   needs are not in the frozen mirrors, so they stand at their use sites below (P-72 rule 4/P-88 ①).
   Which item is `#[unsafe(no_mangle)]` and which is an `unsafe extern "C" fn` follows C's linkage
   and the addresses `syms_of_androidterm` (C 6745-6761) and the interface arrays take.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3656: `extern void gui_update_cursor (struct frame *, bool);`
  pub fn gui_update_cursor (f: *mut frame, on_p: bool);
  /// C: dispextern.h 3798-3799: `bool parse_color_spec (const char *,
  /// unsigned short *, unsigned short *, unsigned short *);`
  pub fn parse_color_spec (spec: *const c_char, r: *mut c_ushort, g: *mut c_ushort,
			   b: *mut c_ushort) -> bool;
  /// C: dispextern.h 3839: `void gamma_correct (struct frame *, XColor *);` - `XColor` is this
  /// tree's `Emacs_Color` (dispextern.h 22-64's X-like record, `include/dispextern.rs` 25).
  pub fn gamma_correct (f: *mut frame, color: *mut Emacs_Color);
  /// C: lisp.h 5117: `extern void store_frame_param (struct frame *, Lisp_Object, Lisp_Object);`
  /// (`src/androidfns.rs` 103-104 declares the same header line for the same unit family.)
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
}

/* C 1894-1927.  `static` in C and C 6745 takes its address, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_read_socket (terminal: *mut terminal,
					  hold_quit: *mut input_event) -> c_int
{
  let mut count: c_int = 0;                                             // C 1898
  let mut dpyinfo: *mut android_display_info;                           // C 1899

  dpyinfo = (*terminal).display_info.android as *mut android_display_info;   // C 1901

  block_input ();                                                       // C 1903
  while android_pending () != 0                                         // C 1904
    {
      let mut finish: c_int = 0;                                        // C 1906
      let mut event: android_event = std::mem::zeroed ();               // C 1907

      android_next_event (&raw mut event);                              // C 1909
      count += handle_one_android_event (dpyinfo, &raw mut event, &raw mut finish,   // C 1910
					 hold_quit);                    // C 1911

      if finish == ANDROID_EVENT_GOTO_OUT                               // C 1913
	{ break; }                                                      // C 1914
    }
  unblock_input ();                                                     // C 1916

  /* If the focus was just given to an auto-raising frame, raise it
     now.  */                                                           // C 1918-1919
  if ! (*dpyinfo).pending_autoraise_frame.is_null ()                    // C 1920
    {
      android_raise_frame ((*dpyinfo).pending_autoraise_frame);          // C 1922
      (*dpyinfo).pending_autoraise_frame = std::ptr::null_mut ();        // C 1923
    }

  return count;                                                         // C 1926
}

/* C 1929-1946.  `static`, and C 6746 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_frame_up_to_date (f: *mut frame)
{
  eassert! (FRAME_ANDROID_P (f));                                       // C 1932
  block_input ();                                                       // C 1933
  FRAME_MOUSE_UPDATE! (f);                                              // C 1934

  if ! buffer_flipping_blocked_p ()                                     // C 1936
      && FRAME_ANDROID_NEED_BUFFER_FLIP! (f)                            // C 1937
    { show_back_buffer (f); }                                          // C 1938

  /* The frame is now complete, as its contents have been drawn.  */     // C 1940
  /* C 1941's `FRAME_ANDROID_COMPLETE_P (f) = true;` is an l-value use of the folded member: the
     mirror's macro is a read (include/androidterm.rs 474-479) and its own comment says a writer
     site spells `bits |= MASK` (the `cm.rs` convention, status 3.516).  */
  (*( (*f).output_data.android as *mut android_output )).bits |= ANDROID_OUTPUT_COMPLETE;   // C 1941

  /* Shrink the scanline buffer used by the font backend.  */            // C 1943
  sfntfont_android_shrink_scanline_buffer ();                           // C 1944
  unblock_input ();                                                     // C 1945
}

/* C 1948-1957.  `static`, and C 6748 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_buffer_flipping_unblocked_hook (f: *mut frame)
{
  block_input ();                                                       // C 1951

  if FRAME_ANDROID_NEED_BUFFER_FLIP! (f)                                // C 1953
    { show_back_buffer (f); }                                          // C 1954

  unblock_input ();                                                     // C 1956
}

/* C 1959-1968.  `static`, and C 6751 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_query_frame_background_color (f: *mut frame,
							   bgcolor: *mut Emacs_Color)
{
  let mut background: c_ulong;                                          // C 1962

  background = FRAME_BACKGROUND_PIXEL (f);                              // C 1964
  (*bgcolor).pixel = background;                                        // C 1965

  android_query_colors (f, bgcolor, 1);                                 // C 1967
}

/* C 1970-2004.  Non-`static` in C and no address is taken: `#[unsafe(no_mangle)]` with the C ABI
   the mirror's declaration uses (`include/androidterm.rs` 584).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_parse_color (f: *mut frame, color_name: *const c_char,
					      color: *mut Emacs_Color) -> c_int
{
  let mut r: c_ushort = 0;                                              // C 1974
  let mut g: c_ushort = 0;
  let mut b: c_ushort = 0;
  let mut tem: Lisp_Object = Qnil ();                                   // C 1975
  let mut tem1: Lisp_Object = Qnil ();
  let mut lisp_color: c_ulong = 0;                                      // C 1976

  if parse_color_spec (color_name, &raw mut r, &raw mut g, &raw mut b)   // C 1978
    {
      (*color).red = r;                                                 // C 1980
      (*color).green = g;                                               // C 1981
      (*color).blue = b;                                                // C 1982

      return 1;                                                         // C 1984
    }

  tem = (*x_display_list).color_map;                                    // C 1987
  while CONSP (tem)                                                     // C 1988
    {
      tem1 = XCAR (tem);                                                // C 1990

      if CONSP (tem1)                                                   // C 1992
	  && dispextern_externs::xstrcasecmp (SSDATA (XCAR (tem1)), color_name) == 0        // C 1993
	{
	  lisp_color = XFIXNUM (XCDR (tem1)) as c_ulong;                 // C 1995
	  (*color).red = (RED_FROM_ULONG! (lisp_color) * 257) as c_ushort;   // C 1996
	  (*color).green = (GREEN_FROM_ULONG! (lisp_color) * 257) as c_ushort;   // C 1997
	  (*color).blue = (BLUE_FROM_ULONG! (lisp_color) * 257) as c_ushort;   // C 1998
	  return 1;                                                     // C 1999
	}
      tem = XCDR (tem);                                                 // C 1988 (the `for` step)
    }

  return 0;                                                             // C 2003
}

/* C 2006-2038.  Non-`static` in C and no address is taken.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_alloc_nearest_color (f: *mut frame,
						      color: *mut Emacs_Color) -> bool
{
  let mut ntsc: c_uint;                                                 // C 2009

  gamma_correct (f, color);                                             // C 2011

  if (*FRAME_DISPLAY_INFO! (f)).n_planes == 1                           // C 2013
    {
      /* Black and white.  I think this is the luminance formula applied
	 by the X server on generic monochrome framebuffers.  */        // C 2015-2016
      (*color).pixel = (if ((((30 * (*color).red as c_ulong)        // C 2017
			      + (59 * (*color).green as c_ulong)        // C 2018
			      + (11 * (*color).blue as c_ulong)) >> 8)  // C 2019
			     >= (((1 << 8) - 1) * 50))                  // C 2020
			 { 0xffffff } else { 0 }) as c_ulong;           // C 2021
    }
  else if (*FRAME_DISPLAY_INFO! (f)).n_planes <= 8                      // C 2023
    {
      /* 256 grays.  */                                                 // C 2025
      ntsc = min (255, ((((*color).red as f64 * 0.299              // C 2026
			   + (*color).green as f64 * 0.587            // C 2027
			   + (*color).blue as f64 * 0.114)            // C 2028
			  / 256.0) as c_uint));                       // C 2029
      (*color).pixel = RGB_TO_ULONG! (ntsc as c_ulong, ntsc as c_ulong,   // C 2030
				      ntsc as c_ulong);                    // C 2030
    }
  else
    { (*color).pixel = RGB_TO_ULONG! (((*color).red / 256) as c_ulong,   // C 2033
				      ((*color).green / 256) as c_ulong,   // C 2034
				      ((*color).blue / 256) as c_ulong); }   // C 2035

  return true;                                                          // C 2037
}

/* C 2040-2051.  Non-`static` in C and C 6752 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_query_colors (f: *mut frame, colors: *mut Emacs_Color,
					       ncolors: c_int)
{
  let mut i: c_int;                                                     // C 2043

  i = 0;                                                                // C 2045
  while i < ncolors
    {
      (*colors.offset (i as isize)).red = (RED_FROM_ULONG! ((*colors.offset (i as isize)).pixel) * 257) as c_ushort;   // C 2047
      (*colors.offset (i as isize)).green = (GREEN_FROM_ULONG! ((*colors.offset (i as isize)).pixel) * 257) as c_ushort;   // C 2048
      (*colors.offset (i as isize)).blue = (BLUE_FROM_ULONG! ((*colors.offset (i as isize)).pixel) * 257) as c_ushort;   // C 2049
      i += 1;                                                           // C 2045 (the `for` step)
    }
}

/* C 2053-2085.  `static`, and C 6753 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_mouse_position (fp: *mut *mut frame, insist: c_int,
					     bar_window: *mut Lisp_Object,
					     part: *mut scroll_bar_part, x: *mut Lisp_Object,
					     y: *mut Lisp_Object, timestamp: *mut Time)
{
  let mut tail: Lisp_Object = Qnil ();                                  // C 2059
  let mut frame: Lisp_Object = Qnil ();
  let mut dpyinfo: *mut android_display_info;                           // C 2060

  dpyinfo = FRAME_DISPLAY_INFO! (*fp);                                  // C 2062

  /* This is the best implementation possible on Android, where the
     system doesn't let Emacs obtain any information about the mouse
     pointer at all.  */                                                // C 2064-2066

  if ! (*dpyinfo).last_mouse_motion_frame.is_null ()                    // C 2068
    {
      *fp = (*dpyinfo).last_mouse_motion_frame;                         // C 2070
      *timestamp = (*dpyinfo).last_mouse_movement_time;                 // C 2071
      *x = make_fixnum ((*dpyinfo).last_mouse_motion_x as EMACS_INT);   // C 2072
      *y = make_fixnum ((*dpyinfo).last_mouse_motion_y as EMACS_INT);   // C 2073
      *bar_window = Qnil ();                                            // C 2074
      *part = scroll_bar_nowhere;                                       // C 2075

      FOR_EACH_FRAME! (tail, frame, {                                 // C 2077
	  if FRAME_ANDROID_P (XFRAME (frame))                           // C 2079
	    { mouse_moved! (XFRAME (frame), false); }                   // C 2080
	});

      mouse_moved! ((*dpyinfo).last_mouse_motion_frame, false);         // C 2083
    }
}

/* C 2087-2100.  `static`, and C 6754 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_get_focus_frame (f: *mut frame) -> Lisp_Object
{
  let mut lisp_focus: Lisp_Object = Qnil ();                            // C 2090
  let mut focus: *mut frame;                                            // C 2091

  focus = (*FRAME_DISPLAY_INFO! (f)).focus_frame;                       // C 2093

  if focus.is_null ()                                                   // C 2095
    { return Qnil (); }                                                 // C 2096

  XSETFRAME! (lisp_focus, focus);                                       // C 2098
  return lisp_focus;                                                    // C 2099
}

/* C 2102-2109.  `static`, and C 6755 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_focus_frame (f: *mut frame, noactivate: bool)
{
  /* Set the input focus to the frame's window.  The system only lets
     this work on child frames.  */                                     // C 2105-2106
  android_set_input_focus (FRAME_ANDROID_WINDOW! (f),                   // C 2107
			   ANDROID_CURRENT_TIME as u64);                 // C 2108
}

/* The two procedures below only have to update the cursor on Android,
   as there are no window borders there.  */                           // C 2111-2112

/* C 2114-2118.  `static`; only called (C 2152/2154), so a plain Rust-ABI item.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_frame_highlight (f: *mut frame)
{
  gui_update_cursor (f, true);                                          // C 2117
}

/* C 2120-2124.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_frame_unhighlight (f: *mut frame)
{
  gui_update_cursor (f, true);                                          // C 2123
}

/* C 2126-2156.  `static`; only called (C 2161), so a plain Rust-ABI item.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_frame_rehighlight (dpyinfo: *mut android_display_info)
{
  let mut old_highlight: *mut frame;                                    // C 2129

  old_highlight = (*dpyinfo).highlight_frame;                           // C 2131

  if ! (*dpyinfo).focus_frame.is_null ()                                // C 2133
    {
      (*dpyinfo).highlight_frame                                        // C 2135
	= (if FRAMEP (FRAME_FOCUS_FRAME ((*dpyinfo).focus_frame))       // C 2136
	     { XFRAME (FRAME_FOCUS_FRAME ((*dpyinfo).focus_frame)) }    // C 2137
	   else { (*dpyinfo).focus_frame });                            // C 2138
      if ! FRAME_LIVE_P ((*dpyinfo).highlight_frame)                    // C 2139
	{
	  fset_focus_frame ((*dpyinfo).focus_frame, Qnil ());           // C 2141
	  (*dpyinfo).highlight_frame = (*dpyinfo).focus_frame;          // C 2142
	}
    }
  else
    { (*dpyinfo).highlight_frame = std::ptr::null_mut (); }             // C 2146

  if (*dpyinfo).highlight_frame != old_highlight                        // C 2148
    {
      /* This is not yet required on Android.  */                       // C 2150
      if ! old_highlight.is_null ()                                     // C 2151
	{ android_frame_unhighlight (old_highlight); }                  // C 2152
      if ! (*dpyinfo).highlight_frame.is_null ()                        // C 2153
	{ android_frame_highlight ((*dpyinfo).highlight_frame); }        // C 2154
    }
}

/* C 2158-2162.  `static`, and C 6756 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_frame_rehighlight_hook (f: *mut frame)
{
  android_frame_rehighlight (FRAME_DISPLAY_INFO! (f));                  // C 2161
}

/* C 2164-2171.  `static`, and C 6757 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_frame_raise_lower (f: *mut frame, raise_flag: bool)
{
  if raise_flag                                                         // C 2167
    { android_raise_frame (f); }                                       // C 2168
  else
    { android_lower_frame (f); }                                       // C 2170
}

/* C 2173-2180.  Non-`static` in C and no address is taken.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_make_frame_visible (f: *mut frame)
{
  android_map_window (FRAME_ANDROID_WINDOW! (f));                       // C 2176

  SET_FRAME_VISIBLE! (f, true);                                         // C 2178
  SET_FRAME_ICONIFIED (f, 0);                                          // C 2179
}

/* C 2182-2193.  Non-`static` in C and no address is taken.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_make_frame_invisible (f: *mut frame)
{
  /* Don't keep the highlight on an invisible frame.  */                 // C 2185
  if (*FRAME_DISPLAY_INFO! (f)).highlight_frame == f                    // C 2186
    { (*FRAME_DISPLAY_INFO! (f)).highlight_frame = std::ptr::null_mut (); }   // C 2187

  android_unmap_window (FRAME_ANDROID_WINDOW! (f));                     // C 2189

  SET_FRAME_VISIBLE! (f, false);                                        // C 2191
  SET_FRAME_ICONIFIED (f, 0);                                          // C 2192
}

/* C 2195-2202.  `static`, and C 6759 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_make_frame_visible_invisible (f: *mut frame, visible: bool)
{
  if visible                                                            // C 2198
    { android_make_frame_visible (f); }                                // C 2199
  else
    { android_make_frame_invisible (f); }                              // C 2201
}

/* C 2204-2231.  `static`, and C 6760 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_fullscreen_hook (f: *mut frame)
{
  let mut wanted: Lisp_Object = Qnil ();                                // C 2207

  if ! FRAME_PARENT_FRAME (f).is_null ()                                // C 2209
    {
      /* Explicitly setting fullscreen is not supported on older
	 Android versions.  */                                         // C 2211-2212

      wanted = (if (*f).want_fullscreen == FULLSCREEN_BOTH            // C 2214
		  { Qfullscreen () } else { Qmaximized () });          // C 2215

      if android_set_fullscreen (FRAME_ANDROID_WINDOW! (f),            // C 2217
				 EQ (wanted, Qfullscreen ())) != 0     // C 2218
	{ store_frame_param (f, Qfullscreen (), Qmaximized ()); }      // C 2219
      else
	{ store_frame_param (f, Qfullscreen (), wanted); }             // C 2221
    }
  else
    {
      store_frame_param (f, Qfullscreen (), Qnil ());                  // C 2225

      /* If this is a child frame, don't keep it fullscreen
	 anymore.  */                                                 // C 2227-2228
      android_set_fullscreen (FRAME_ANDROID_WINDOW! (f), false);       // C 2229
    }
}

/* C 2233-2238.  Non-`static` in C and C 6761 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_iconify_frame (f: *mut frame)
{
  /* This really doesn't work on Android.  */                            // C 2236
  error (c"Can't notify window manager of iconification".as_ptr ());    // C 2237
}

/* C 2240-2648: the wait/size/position group and the frame-teardown group.  Four names this text
   needs are not in the frozen mirrors, so they stand at their use sites below (P-72 rule 4), and
   four frame.h accessors C *writes* need the write arm the mirror does not carry, so the unit
   carries the two-arm macros - the `src/xterm.rs` 15127-15145 shape.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_COLUMN_WIDTH {
  ($f:expr) => { FRAME_COLUMN_WIDTH ($f) };
  ($f:expr, $v:expr) => { (*$f).column_width = $v };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_LINE_HEIGHT {
  ($f:expr) => { FRAME_LINE_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).line_height = $v };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr) => { FRAME_TAB_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).tab_bar_height = $v };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr) => { FRAME_CONFIG_SCROLL_BAR_COLS ($f) };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_cols = $v };
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: frame.h 1511-1512: `extern void adjust_frame_size (struct frame *, int, int, int, bool,
  /// Lisp_Object);`
  pub fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
			    inhibit: c_int, pretend: bool, parameter: Lisp_Object);
  /// C: dispextern.h 3923: `void do_pending_window_change (bool);`
  pub fn do_pending_window_change (safe: bool);
  /// C: dispextern.h 3631: `extern void get_font_ascent_descent (struct font *, int *, int *);`
  pub fn get_font_ascent_descent (font: *mut font, ascent: *mut c_int, descent: *mut c_int);
  /// C: dispextern.h 3822: `void free_frame_faces (struct frame *);`
  pub fn free_frame_faces (f: *mut frame);
  /// C: fontset.h 39: `extern int fontset_from_font (Lisp_Object);`
  pub fn fontset_from_font (font_object: Lisp_Object) -> c_int;
}

/* C 2240-2280.  `static` and only called (C 2296/2351), so a plain Rust-ABI item.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_wait_for_event (f: *mut frame, eventtype: c_int)
{
  if ! FLOATP (Vandroid_wait_for_event_timeout)                       // C 2243
    { return; }                                                       // C 2244

  let level: c_int = interrupt_input_blocked;                          // C 2246
  let mut tmo: timespec = std::mem::zeroed ();                         // C 2247
  let mut tmo_at: timespec = std::mem::zeroed ();
  let mut time_now: timespec = std::mem::zeroed ();

  (*f).wait_event_type = eventtype;                                    // C 2249

  /* Default timeout is 0.1 second.  Hopefully not noticeable.  */       // C 2251
  let timeout: f64 = XFLOAT_DATA (Vandroid_wait_for_event_timeout);     // C 2252
  let timeout_seconds: time_t = timeout as time_t;                     // C 2253
  tmo = make_timespec (timeout_seconds,                                // C 2254
		       ((timeout - timeout_seconds as f64)             // C 2255
			* 1000.0 * 1000.0 * 1000.0) as c_long);        // C 2256
  tmo_at = timespec_add (current_timespec (), tmo);                    // C 2257

  while (*f).wait_event_type != 0                                      // C 2259
    {
      pending_signals = true;                                          // C 2261
      totally_unblock_input ();                                        // C 2262
      /* android_read_socket is called after unblock.  */               // C 2263
      block_input ();                                                  // C 2264
      interrupt_input_blocked = level;                                 // C 2265

      if (*f).wait_event_type == 0                                     // C 2267
	{ break; }                                                     // C 2268

      time_now = current_timespec ();                                  // C 2270
      if timespec_cmp (tmo_at, time_now) < 0                           // C 2271
	{ break; }                                                     // C 2272

      tmo = timespec_sub (tmo_at, time_now);                           // C 2274
      if android_select (0, std::ptr::null_mut (), std::ptr::null_mut (),   // C 2275
			 std::ptr::null_mut (), &raw mut tmo) == 0     // C 2275
	{ break; }                                                     // C 2276 (Timeout)
    }

  (*f).wait_event_type = 0;                                            // C 2279
}

/* C 2282-2315.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_window_size_1 (f: *mut frame, change_gravity: bool,
				     width: c_int, height: c_int)
{
  if change_gravity                                                    // C 2286
    { (*f).win_gravity = NorthWestGravity; }                           // C 2287

  android_resize_window (FRAME_ANDROID_WINDOW! (f), width as c_uint,   // C 2289
			 height as c_uint);                            // C 2290

  SET_FRAME_GARBAGED! (f);                                             // C 2292

  if FRAME_VISIBLE_P (f)                                               // C 2294
    {
      android_wait_for_event (f, ANDROID_CONFIGURE_NOTIFY);            // C 2296

      if CONSP (frame_size_history)                                    // C 2298
	{ frame_size_history_extra (f, build_string (c"set_window_size_1 visible".as_ptr ()),   // C 2299
				    FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f),   // C 2300
				    width, height, (*f).new_width, (*f).new_height); }   // C 2301
    }
  else
    {
      if CONSP (frame_size_history)                                    // C 2305
	{ frame_size_history_extra (f, build_string (c"set_window_size_1 invisible".as_ptr ()),   // C 2306
				    FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f),   // C 2308
				    width, height, (*f).new_width, (*f).new_height); }   // C 2309

      adjust_frame_size (f, FRAME_PIXEL_TO_TEXT_WIDTH (f, width),      // C 2311
			 FRAME_PIXEL_TO_TEXT_HEIGHT (f, height),       // C 2312
			 5, false, Qx_set_window_size_1 ());           // C 2313
    }
}

/* C 2317-2338.  Non-`static` in C and C 6762 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_window_size (f: *mut frame, change_gravity: bool,
						  width: c_int, height: c_int)
{
  block_input ();                                                       // C 2321

  android_set_window_size_1 (f, change_gravity, width, height);          // C 2323
  android_clear_under_internal_border (f);                              // C 2324

  /* If cursor was outside the new size, mark it as off.  */             // C 2326
  mark_window_cursors_off (XWINDOW ((*f).root_window));                 // C 2327

  /* Clear out any recollection of where the mouse highlighting was,
     since it might be in a place that's outside the new frame size.
     Actually checking whether it is outside is a pain in the neck,
     so don't try--just let the highlighting be done afresh with new size.  */   // C 2329-2332
  cancel_mouse_face (f);                                                // C 2333

  unblock_input ();                                                     // C 2335

  do_pending_window_change (false);                                     // C 2337
}

/* C 2340-2358.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_window_size_and_position_1 (f: *mut frame, width: c_int, height: c_int)
{
  let x: c_int = (*f).left_pos;                                         // C 2343
  let y: c_int = (*f).top_pos;                                          // C 2344

  android_move_resize_window (FRAME_ANDROID_WINDOW! (f), x, y,   // C 2346
				    width as c_uint, height as c_uint);        // C 2346

  SET_FRAME_GARBAGED! (f);                                             // C 2348

  if FRAME_VISIBLE_P (f)                                               // C 2350
    { android_wait_for_event (f, ANDROID_CONFIGURE_NOTIFY); }           // C 2351
  else
    /* Call adjust_frame_size right.  It might be tempting to clear out
       f->new_width and f->new_height here.  */                        // C 2353-2354
    { adjust_frame_size (f, FRAME_PIXEL_TO_TEXT_WIDTH (f, width),      // C 2355
			 FRAME_PIXEL_TO_TEXT_HEIGHT (f, height),       // C 2356
			 5, false, Qx_set_window_size_1 ()); }         // C 2357
}

/* C 2360-2380.  Non-`static` in C and C 6764 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_window_size_and_position (f: *mut frame, width: c_int,
							       height: c_int)
{
  block_input ();                                                       // C 2363

  android_set_window_size_and_position_1 (f, width, height);             // C 2365
  android_clear_under_internal_border (f);                              // C 2366

  /* If cursor was outside the new size, mark it as off.  */             // C 2368
  mark_window_cursors_off (XWINDOW (FRAME_ROOT_WINDOW (f)));            // C 2369

  /* Clear out any recollection of where the mouse highlighting was,
     since it might be in a place that's outside the new frame size.
     Actually checking whether it is outside is a pain in the neck,
     so don't try--just let the highlighting be done afresh with new size.  */   // C 2371-2374
  cancel_mouse_face (f);                                                // C 2375

  unblock_input ();                                                     // C 2377

  do_pending_window_change (false);                                     // C 2379
}

/* Calculate the absolute position in frame F
   from its current recorded position values and gravity.  */         // C 2382-2383

/* C 2385-2455.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_calc_absolute_position (f: *mut frame)
{
  let flags: c_int = (*f).size_hint_flags;                              // C 2388
  let p: *mut frame = FRAME_PARENT_FRAME (f);                           // C 2389

  /* We have nothing to do if the current position
     is already for the top-left corner.  */                            // C 2391-2392
  if !((flags & XNegative != 0) || (flags & YNegative != 0))            // C 2393
    { return; }                                                         // C 2394

  /* Treat negative positions as relative to the leftmost bottommost
     position that fits on the screen.  */                              // C 2396-2397
  if flags & XNegative != 0                                             // C 2398
    {
      let mut width: c_int = FRAME_PIXEL_WIDTH (f);                     // C 2400

      /* A frame that has been visible at least once should have outer
	 edges.  */                                                     // C 2402-2403
      if (*( (*f).output_data.android as *mut android_output )).bits    // C 2404
	  & ANDROID_OUTPUT_HAS_BEEN_VISIBLE != 0                        // C 2404 (C's `has_been_visible`)
	  && p.is_null ()
	{
	  let mut frame: Lisp_Object = Qnil ();                         // C 2406
	  let mut edges: Lisp_Object = Qnil ();                         // C 2407

	  XSETFRAME! (frame, f);                                        // C 2409
	  edges = Fandroid_frame_edges (frame, Qouter_edges ());         // C 2410
	  if ! NILP (edges)                                             // C 2411
	    { width = (XFIXNUM (Fnth (make_fixnum (2), edges))          // C 2412
		       - XFIXNUM (Fnth (make_fixnum (0), edges))) as c_int; }   // C 2413
	}

      if ! p.is_null ()                                                // C 2416
	{ (*f).left_pos = (FRAME_PIXEL_WIDTH (p) - width - 2 * (*f).border_width   // C 2417
			   + (*f).left_pos); }                          // C 2418
      else
	/* Not that this is of much significance, for Android programs
	   cannot position their windows at absolute positions in the
	   screen.  */                                                  // C 2420-2422
	{ (*f).left_pos = (android_get_screen_width () - width + (*f).left_pos); }   // C 2423
    }

  if flags & YNegative != 0                                             // C 2427
    {
      let mut height: c_int = FRAME_PIXEL_HEIGHT (f);                   // C 2429

      if (*( (*f).output_data.android as *mut android_output )).bits    // C 2431
	  & ANDROID_OUTPUT_HAS_BEEN_VISIBLE != 0                        // C 2431
	  && p.is_null ()
	{
	  let mut frame: Lisp_Object = Qnil ();                         // C 2433
	  let mut edges: Lisp_Object = Qnil ();                         // C 2434

	  XSETFRAME! (frame, f);                                        // C 2436
	  if NILP (edges)                                               // C 2437
	    { edges = Fandroid_frame_edges (frame, Qouter_edges ()); }  // C 2438
	  if ! NILP (edges)                                             // C 2439
	    { height = (XFIXNUM (Fnth (make_fixnum (3), edges))         // C 2440
			- XFIXNUM (Fnth (make_fixnum (1), edges))) as c_int; }   // C 2441
	}

      if ! p.is_null ()                                                // C 2444
	{ (*f).top_pos = (FRAME_PIXEL_HEIGHT (p) - height - 2 * (*f).border_width   // C 2445
			  + (*f).top_pos); }                           // C 2446
      else
	{ (*f).top_pos = (android_get_screen_height () - height + (*f).top_pos); }   // C 2448
  }

  /* The left_pos and top_pos
     are now relative to the top and left screen edges,
     so the flags should correspond.  */                               // C 2451-2453
  (*f).size_hint_flags &= !(XNegative | YNegative);                     // C 2454
}

/* C 2457-2476.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_offset (f: *mut frame, xoff: c_int, yoff: c_int, change_gravity: c_int)
{
  if change_gravity > 0                                                // C 2461
    {
      (*f).top_pos = yoff;                                            // C 2463
      (*f).left_pos = xoff;                                           // C 2464
      (*f).size_hint_flags &= !(XNegative | YNegative);                 // C 2465
      if xoff < 0                                                     // C 2466
	{ (*f).size_hint_flags |= XNegative; }                        // C 2467
      if yoff < 0                                                     // C 2468
	{ (*f).size_hint_flags |= YNegative; }                        // C 2469
      (*f).win_gravity = NorthWestGravity;                            // C 2470
    }

  android_calc_absolute_position (f);                                  // C 2473
  android_move_window (FRAME_ANDROID_WINDOW! (f), (*f).left_pos,        // C 2474
		       (*f).top_pos);                                  // C 2475
}

/* C 2478-2482.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_alpha (f: *mut frame)
{
  /* Not supported on Android.  */                                       // C 2481
}

/* C 2484-2528.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_new_font (f: *mut frame, font_object: Lisp_Object, mut fontset: c_int)
    -> Lisp_Object
{
  let font: *mut font = XFONT_OBJECT (font_object);                     // C 2487
  let mut unit: c_int = 0;                                             // C 2488
  let mut font_ascent: c_int = 0;
  let mut font_descent: c_int = 0;

  if fontset < 0                                                       // C 2490
    { fontset = fontset_from_font (font_object); }                     // C 2491
  FRAME_FONTSET! (f) = fontset;                                        // C 2492
  if FRAME_FONT! (f) == font                                           // C 2493
    /* This font is already set in frame F.  There's nothing more to
       do.  */                                                         // C 2494-2495
    { return font_object; }                                            // C 2496

  FRAME_FONT! (f) = font;                                              // C 2498
  FRAME_BASELINE_OFFSET! (f) = (*font).baseline_offset;                 // C 2499
  FRAME_COLUMN_WIDTH! (f, (*font).average_width);                       // C 2500
  get_font_ascent_descent (font, &raw mut font_ascent, &raw mut font_descent);   // C 2501
  FRAME_LINE_HEIGHT! (f, font_ascent + font_descent);                   // C 2502

  /* We could use a more elaborate calculation here.  */                // C 2504
  FRAME_TAB_BAR_HEIGHT! (f, FRAME_TAB_BAR_LINES (f) * FRAME_LINE_HEIGHT (f));   // C 2505

  /* Compute character columns occupied by scrollbar.

     Don't do things differently for non-toolkit scrollbars
     (Bug#17163).  */                                                  // C 2507-2510
  unit = FRAME_COLUMN_WIDTH (f);                                       // C 2511
  if FRAME_CONFIG_SCROLL_BAR_WIDTH (f) > 0                             // C 2512
    { FRAME_CONFIG_SCROLL_BAR_COLS! (f, (FRAME_CONFIG_SCROLL_BAR_WIDTH (f) + unit - 1) / unit); }   // C 2513-2514
  else
    { FRAME_CONFIG_SCROLL_BAR_COLS! (f, (14 + unit - 1) / unit); }      // C 2516

  /* Don't change the size of a tip frame; there's no point in doing it
     because it's done in Fx_show_tip, and it leads to problems because
     the tip frame has no widget.  */                                  // C 2519-2521
  if FRAME_ANDROID_WINDOW! (f) != 0 && ! FRAME_TOOLTIP_P (f)            // C 2522
    { adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),   // C 2523
			 FRAME_LINES (f) * FRAME_LINE_HEIGHT (f), 3,   // C 2524
			 false, Qfont ()); }                           // C 2525

  return font_object;                                                  // C 2527
}

/* C 2530-2534.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_bitmap_icon (f: *mut frame, file: Lisp_Object) -> bool
{
  return false;                                                        // C 2533
}

/* C 2536-2540.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_free_pixmap_hook (f: *mut frame, pixmap: Emacs_Pixmap)
{
  android_free_pixmap (pixmap);                                        // C 2539
}

/* C 2542-2634.  Non-`static` in C and no address is taken in this file.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_free_frame_resources (f: *mut frame)
{
  let mut dpyinfo: *mut android_display_info;                           // C 2545
  let mut hlinfo: *mut Mouse_HLInfo;                                    // C 2546
  let mut last: *mut android_touch_point;                               // C 2547
  let mut next: *mut android_touch_point;

  dpyinfo = FRAME_DISPLAY_INFO! (f);                                    // C 2549
  hlinfo = &raw mut (*dpyinfo).mouse_highlight;                         // C 2550

  block_input ();                                                       // C 2552
  free_frame_faces (f);                                                 // C 2553

  /* FRAME_ANDROID_WINDOW can be 0 if frame creation failed.  */         // C 2555
  if FRAME_ANDROID_WINDOW! (f) != 0                                     // C 2556
    { android_destroy_window (FRAME_ANDROID_WINDOW! (f)); }             // C 2557

  android_free_gcs (f);                                                 // C 2559

  /* Free cursors.  */                                                  // C 2561
  if (*( (*f).output_data.android as *mut android_output )).text_cursor != 0   // C 2562
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).text_cursor); }   // C 2563
  if (*( (*f).output_data.android as *mut android_output )).nontext_cursor != 0   // C 2564
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).nontext_cursor); }   // C 2565
  if (*( (*f).output_data.android as *mut android_output )).modeline_cursor != 0   // C 2566
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).modeline_cursor); }   // C 2567
  if (*( (*f).output_data.android as *mut android_output )).hand_cursor != 0   // C 2568
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).hand_cursor); }   // C 2569
  if (*( (*f).output_data.android as *mut android_output )).hourglass_cursor != 0   // C 2570
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).hourglass_cursor); }   // C 2571
  if (*( (*f).output_data.android as *mut android_output )).horizontal_drag_cursor != 0   // C 2572
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).horizontal_drag_cursor); }   // C 2573
  if (*( (*f).output_data.android as *mut android_output )).vertical_drag_cursor != 0   // C 2574
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).vertical_drag_cursor); }   // C 2575
  if (*( (*f).output_data.android as *mut android_output )).left_edge_cursor != 0   // C 2576
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).left_edge_cursor); }   // C 2577
  if (*( (*f).output_data.android as *mut android_output )).top_left_corner_cursor != 0   // C 2578
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).top_left_corner_cursor); }   // C 2579
  if (*( (*f).output_data.android as *mut android_output )).top_edge_cursor != 0   // C 2580
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).top_edge_cursor); }   // C 2581
  if (*( (*f).output_data.android as *mut android_output )).top_right_corner_cursor != 0   // C 2582
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).top_right_corner_cursor); }   // C 2583
  if (*( (*f).output_data.android as *mut android_output )).right_edge_cursor != 0   // C 2584
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).right_edge_cursor); }   // C 2585
  if (*( (*f).output_data.android as *mut android_output )).bottom_right_corner_cursor != 0   // C 2586
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).bottom_right_corner_cursor); }   // C 2587
  if (*( (*f).output_data.android as *mut android_output )).bottom_edge_cursor != 0   // C 2588
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).bottom_edge_cursor); }   // C 2589
  if (*( (*f).output_data.android as *mut android_output )).bottom_left_corner_cursor != 0   // C 2590
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).bottom_left_corner_cursor); }   // C 2591

  /* Free extra GCs allocated by android_setup_relief_colors.  */        // C 2593
  if ! (*( (*f).output_data.android as *mut android_output )).white_relief.gc.is_null ()   // C 2594
    {
      android_free_gc ((*( (*f).output_data.android as *mut android_output )).white_relief.gc);   // C 2596
      (*( (*f).output_data.android as *mut android_output )).white_relief.gc = std::ptr::null_mut ();   // C 2597
    }
  if ! (*( (*f).output_data.android as *mut android_output )).black_relief.gc.is_null ()   // C 2599
    {
      android_free_gc ((*( (*f).output_data.android as *mut android_output )).black_relief.gc);   // C 2601
      (*( (*f).output_data.android as *mut android_output )).black_relief.gc = std::ptr::null_mut ();   // C 2602
    }

  if f == (*dpyinfo).focus_frame                                      // C 2605
    { (*dpyinfo).focus_frame = std::ptr::null_mut (); }                 // C 2606
  if f == (*dpyinfo).x_focus_event_frame                              // C 2607
    { (*dpyinfo).x_focus_event_frame = std::ptr::null_mut (); }         // C 2608
  if f == (*dpyinfo).highlight_frame                                  // C 2609
    { (*dpyinfo).highlight_frame = std::ptr::null_mut (); }             // C 2610
  if f == (*hlinfo).mouse_face_mouse_frame as *mut frame                // C 2611
    { reset_mouse_highlight (hlinfo); }                                 // C 2612

  /* These two need to be freed now that they are used to compute the
     mouse position, I think.  */                                      // C 2614-2615
  if f == (*dpyinfo).last_mouse_motion_frame                          // C 2616
    { (*dpyinfo).last_mouse_motion_frame = std::ptr::null_mut (); }     // C 2617
  if f == (*dpyinfo).last_mouse_frame                                 // C 2618
    { (*dpyinfo).last_mouse_frame = std::ptr::null_mut (); }            // C 2619

  /* Free all tool presses currently active on this frame.  */           // C 2621
  next = (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).touch_points;   // C 2622
  while ! next.is_null ()                                               // C 2623
    {
      last = next;                                                      // C 2625
      next = (*next).next;                                              // C 2626
      xfree (last as *mut c_void);                                      // C 2627
    }

  /* Clear this in case unblock_input reads events.  */                  // C 2630
  (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).touch_points = std::ptr::null_mut ();   // C 2631

  unblock_input ();                                                     // C 2633
}

/* C 2636-2642.  `static`, and C 6779 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_delete_frame (f: *mut frame)
{
  android_free_frame_resources (f);                                     // C 2639
  xfree ((*f).output_data.android);                                     // C 2640
  (*f).output_data.android = std::ptr::null_mut ();                      // C 2641
}

/* C 2644-2648.  `static`, and C 6780 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_delete_terminal (terminal: *mut terminal)
{
  error (c"Cannot terminate connection to Android display server".as_ptr ());   // C 2647
}
