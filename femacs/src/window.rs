// femacs: port of src/window.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, file-scope object, local), same control flow, same error
// model.  The 121 `DEFUN`s use `DEFUN!` (PORTING.md 5.1); the non-`static` definitions C
// exports carry `#[unsafe(no_mangle)]` (PORTING.md 0.2) and the C `static` helpers stay
// plain `unsafe fn`.
//
// `src/window.c` *is* in `docs/baseline/config/cond-eval-verified.txt`
// (`python3 tools/check-cond-eval.py src/window.c` = `17 conditions, 0 unknown,
// 0 disagreements [VERIFIED]`), so per P-08 the arms the frozen configuration does not
// compile are *recorded in `docs/ports/window.md`, not transcribed*.  The dead arms:
//
//   * C 24-26 — `#if 10 <= __GNUC__` (clang reports `__GNUC__` 4, so the diagnostic pragma is
//     dead; the port emits no pragma either way).
//   * C 42-44, 1716-1729, 2580-2583, 4624-4627, 7265-7267, 8732-8753 — the
//     `HAVE_WINDOW_SYSTEM` arms (`#include TERM_HEADER`, the tab-bar window probe, the
//     `phys_cursor_type`/`phys_cursor_width` reset, the `minimize_tab_bar_window_p` write,
//     the `CURRENT_TAB_LINE_HEIGHT` write, and the whole GUI body of `Fwindow_cursor_info`).
//   * C 45-47 — the `MSDOS` arm (`#include "msdos.h"`).
//   * C 1701-1714, 1731-1744 — `HAVE_EXT_MENU_BAR`/`HAVE_EXT_TOOL_BAR` (both defined for this
//     target, so their `#if … ! defined (HAVE_EXT_*)` probes for the toolkit menu/tool bar
//     windows are dead).
//   * C 2508-2513, 7518-7520, 7556-7567 — three `#if false` blocks (`unshow_buffer`'s
//     conditional, `move-to-window-line`'s dead local, and its scroll-margin clamp).
//   * C 2792-2801 — the `HAVE_X_WINDOWS` half of `candidate_window_p`'s visibility test.
//   * C 4282-4292 — the `HAVE_TEXT_CONVERSION` arm of `run_window_change_functions`.
//   * C 7268-7270 — the `! defined (HAVE_EXT_TOOL_BAR)` half of `Frecenter`.
//   * C 8809-8839 — `#if USE_HORIZONTAL_SCROLL_BARS`; the object-like macro is `false` here,
//     so C's live text is the `#else` arm's single `wset_horizontal_scroll_bar_type (w, Qnil);`
//     (the mirror keeps `USE_HORIZONTAL_SCROLL_BARS` as a `const`).
//
// **The geometry family is the mirror's.**  `include/window.rs` carries all of `window.h`'s
// accessors now — the frame-dereferencing geometry, the scroll-bar/fringe/divider widths, the
// three `WINDOW_*_LINE_HEIGHT` accessors and `WINDOW_XFRAME`/`WINDOW_FRAME` — so this unit no
// longer declares any of them at its use sites (the block that did was removed in the lane
// that landed them; PORTING.md P-26).  The ones the mirror mirrors as `#[inline(always)]`
// functions are called without the `!`; the seven C macros whose expansion names
// dispextern.h's `CURRENT_*_LINE_HEIGHT`/`window_box_left` keep C's `macro_rules!` shape and
// are called with it (`WINDOW_MODE_LINE_HEIGHT! (w)`, …).
//
// **L-value macros.**  `window.h`'s accessors are macros in C; `include/window.rs` mirrors the
// read-only ones as functions and packs the nineteen `bool_bf` flags into `bits` (PORTING.md
// 1.3/P-80).  The macros this file's C *writes* to are defined here at the use site under C's
// own name (`window_bits!`, …), so the port's body keeps C's token and the write goes through
// the mirror's mask; `NAME! (…)` is the macro call in Rust, whose `!` the token check ignores.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 20: `#include <config.h>`
include!("../lib/include/stdio.rs");           // lisp.h 30: `#include <stdio.h>`
include!("../lib/include/stdlib.rs");          // lisp.h: `#include <stdlib.h>`
include!("../lib/include/limits.in.rs");       // lisp.h: `INT_MAX`/`INT_MIN`/`LONG_MAX`
include!("../lib/include/stdint.in.rs");       // lisp.h: the width family
include!("../lib/include/string.rs");          // lisp.h 29: `#include <string.h>`
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>` (countof)
include!("include/lisp.rs");                   // C 31: `#include "lisp.h"`
/* The mirrors the headers above pull in (the text of C's own include chain, in C's order
   where one applies; Rust item order is irrelevant but macros must precede their uses).  */
include!("include/character.rs");              // lisp.h's `CHARACTERP`/`MAX_CHAR` users
include!("include/intervals.rs");              // buffer.h 27: `#include "intervals.h"`
include!("include/systime.rs");                // lisp.h: `Time`/`struct timespec`
include!("include/itree.rs");                  // intervals.h: `itree_node`/`itree_tree`
include!("include/composite.rs");              // coding.h 284: `#include "composite.h"`
include!("include/coding.rs");                 // dispextern.h 29: `#include "coding.h"`
include!("include/font.rs");                   // dispextern.h's font names
include!("include/termhooks.rs");              // C 44: `#include "termhooks.h"`
include!("include/frame.rs");                  // C 37: `#include "frame.h"`
include!("include/keyboard.rs");               // C 33: `#include "keyboard.h"`
include!("include/keymap.rs");                 // C 34: `#include "keymap.h"`
include!("include/buffer.rs");                 // C 32: `#include "buffer.h"`
include!("include/blockinput.rs");             // C 43: `#include "blockinput.h"`
include!("include/termchar.rs");               // C 39: `#include "termchar.h"`
include!("include/disptab.rs");                // C 40: `#include "disptab.h"`
include!("include/dispextern.rs");             // C 41: `#include "dispextern.h"`
include!("include/window.rs");                 // C 38: `#include "window.h"`
include!("include/commands.rs");               // C 36: `#include "commands.h"`
include!("include/indent.rs");                 // C 37: `#include "indent.h"`
include!("include/pdumper.rs");                // C 51: `#include "pdumper.h"`
/* C 43: `#include TERM_HEADER`, inside C's `#ifdef HAVE_WINDOW_SYSTEM`.  The frozen
   configuration leaves `TERM_HEADER` undefined (no window system), so no unit carried the edge and
   every `FRAME_DISPLAY_INFO` expansion in this unit's body read `cannot find macro` on
   `aarch64-linux-android` (measured; `dispnew.rs` carries the same five-way gate).  */
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
include!("include/androidterm.rs");            // C 43 (TERM_HEADER): `androidterm.h`


/* window.h 392-410: the nineteen `bool_bf : 1` members share the unit `bits`; the readers are
   in the mirror (`window_bit`), the writers are here (PORTING.md 1.3/P-80).  */
macro_rules! window_bits {
    ($w:expr, $name:ident) => { window_bit ($w, window_bits_shift! ($name)) };
    ($w:expr, $name:ident, $v:expr) => {{
        (*$w).bits = ((*$w).bits & ! (1u32 << window_bits_shift! ($name)))
                     | (($v as u32) << window_bits_shift! ($name));
    }};
}
macro_rules! window_bits_shift {
    (mini) => { WINDOW_MINI_SHIFT };
    (horizontal) => { WINDOW_HORIZONTAL_SHIFT };
    (update_mode_line) => { WINDOW_UPDATE_MODE_LINE_SHIFT };
    (last_had_star) => { WINDOW_LAST_HAD_STAR_SHIFT };
    (start_at_line_beg) => { WINDOW_START_AT_LINE_BEG_SHIFT };
    (force_start) => { WINDOW_FORCE_START_SHIFT };
    (optional_new_start) => { WINDOW_OPTIONAL_NEW_START_SHIFT };
    (phys_cursor_on_p) => { WINDOW_PHYS_CURSOR_ON_P_SHIFT };
    (cursor_off_p) => { WINDOW_CURSOR_OFF_P_SHIFT };
    (last_cursor_off_p) => { WINDOW_LAST_CURSOR_OFF_P_SHIFT };
    (must_be_updated_p) => { WINDOW_MUST_BE_UPDATED_P_SHIFT };
    (pseudo_window_p) => { WINDOW_PSEUDO_WINDOW_P_SHIFT };
    (fringes_outside_margins) => { WINDOW_FRINGES_OUTSIDE_MARGINS_SHIFT };
    (fringes_persistent) => { WINDOW_FRINGES_PERSISTENT_SHIFT };
    (scroll_bars_persistent) => { WINDOW_SCROLL_BARS_PERSISTENT_SHIFT };
    (window_end_valid) => { WINDOW_WINDOW_END_VALID_SHIFT };
    (redisplay) => { WINDOW_REDISPLAY_SHIFT };
    (suspend_auto_hscroll) => { WINDOW_SUSPEND_AUTO_HSCROLL_SHIFT };
    (preserve_vscroll_p) => { WINDOW_PRESERVE_VSCROLL_P_SHIFT };
}

/* window.h 1226-1233: `WINDOW_CONFIGURATIONP`.  */
macro_rules! WINDOW_CONFIGURATIONP {
    ($a:expr) => { PSEUDOVECTORP ($a, pvec_type::PVEC_WINDOW_CONFIGURATION) };
}
/* frame.h 1697-1720: the canonical <-> pixel conversions (the mirror carries neither).  */
macro_rules! FRAME_PIXEL_X_FROM_CANON_X {
    ($f:expr, $x:expr) => { (XFLOATINT ($x) * FRAME_COLUMN_WIDTH ($f) as f64) as c_int };
}
macro_rules! FRAME_PIXEL_Y_FROM_CANON_Y {
    ($f:expr, $y:expr) => { (XFLOATINT ($y) * FRAME_LINE_HEIGHT ($f) as f64) as c_int };
}
macro_rules! FRAME_CANON_X_FROM_PIXEL_X {
    ($f:expr, $x:expr) => {
        if ($x) % FRAME_COLUMN_WIDTH ($f) != 0
          { make_float (($x) as f64 / FRAME_COLUMN_WIDTH ($f) as f64) }
        else { make_fixnum ((($x) / FRAME_COLUMN_WIDTH ($f)) as EMACS_INT) }
    };
}
macro_rules! FRAME_CANON_Y_FROM_PIXEL_Y {
    ($f:expr, $y:expr) => {
        if ($y) % FRAME_LINE_HEIGHT ($f) != 0
          { make_float (($y) as f64 / FRAME_LINE_HEIGHT ($f) as f64) }
        else { make_fixnum ((($y) / FRAME_LINE_HEIGHT ($f)) as EMACS_INT) }
    };
}

/* frame.h 1387-1391: `CHECK_FRAME`/`CHECK_LIVE_FRAME` are the mirror's (`include/frame.rs`
   979/983, which this unit includes); the unit's body-identical copies were redundancy
   (P-26).  */

/* frame.h 331-441: the `bool_bf` flag run of `struct frame` whose writers this file carries
   (`select_mini_window_flag` 536, `window_change` 426, `frozen_window_starts` 441; their
   readers `FRAME_WINDOW_CHANGE`/`FRAME_WINDOWS_FROZEN` are frame.h 1191/1187).  The readers
   are in the mirror; the writers are here
   (PORTING.md 1.3/P-80).  */
macro_rules! set_frame_flag1 {
    ($f:expr, $shift:expr, $v:expr) => {
        (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
    };
}
macro_rules! set_frame_flag2 {
    ($f:expr, $shift:expr, $v:expr) => {
        (*$f).bits2 = ((*$f).bits2 & ! (1u32 << $shift)) | (($v as u32) << $shift)
    };
}
macro_rules! select_mini_window_flag {
    ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_SELECT_MINI_WINDOW_FLAG_SHIFT, $v) };
}

/* Definitions in other units whose headers' mirrors do not carry the prototype
   (PORTING.md 0.1-2/P-72 rule 3); each keeps C's own prototype in a `/// C:` line.  */
unsafe extern "C" {
  /// C: frame.h 1476: `struct frame *decode_live_frame (Lisp_Object);`
  pub fn decode_live_frame (frame: Lisp_Object) -> *mut frame;
  /// C: frame.h 1477: `struct frame *decode_any_frame (Lisp_Object);`
  pub fn decode_any_frame (frame: Lisp_Object) -> *mut frame;
  // window.h 1180's `redisplay_other_windows` (xdisp.c defines it) is the mirror's now.
}

unsafe extern "C" {
  /// C: dispextern.h 3582: `int estimate_mode_line_height (struct frame *, enum face_id);`
  /// (xdisp.c defines it; the mirror does not carry it).
  pub fn estimate_mode_line_height (f: *mut frame, face_id: face_id) -> c_int;
}


/* frame.h 300-341/1163-1226: the packed `bool_bf` flags this file reads and writes.  The
   mirror carries `FRAME_WINDOWS_FROZEN`/`FRAME_WINDOW_P` and the read-only `FRAME_VISIBLE_P`-
   class accessors; the ones below are not in it (PORTING.md 1.3/P-80, the frame.rs shape).  */
macro_rules! can_set_window_size {
    ($f:expr) => {{
        #[cfg (not (feature = "have_window_system"))]
        { (*$f).bits1 >> FRAME_CAN_SET_WINDOW_SIZE_SHIFT & 1 != 0 }
        #[cfg (feature = "have_window_system")]
        { (*$f).bits2 >> FRAME_CAN_SET_WINDOW_SIZE_SHIFT & 1 != 0 }
    }};
    ($f:expr, $v:expr) => {{
        #[cfg (not (feature = "have_window_system"))]
        { set_frame_flag1! ($f, FRAME_CAN_SET_WINDOW_SIZE_SHIFT, $v) }
        #[cfg (feature = "have_window_system")]
        { set_frame_flag2! ($f, FRAME_CAN_SET_WINDOW_SIZE_SHIFT, $v) }
    }};
}
macro_rules! after_make_frame {
    ($f:expr) => { (*$f).bits1 >> FRAME_AFTER_MAKE_FRAME_SHIFT & 1 != 0 };
}
macro_rules! FRAME_WINDOW_CHANGE {
    ($f:expr) => { (*$f).bits1 >> FRAME_WINDOW_CHANGE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_WINDOW_CHANGE_SHIFT, $v) };
}
macro_rules! FRAME_WINDOW_STATE_CHANGE {
    ($f:expr) => { (*$f).bits1 >> FRAME_WINDOW_STATE_CHANGE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_WINDOW_STATE_CHANGE_SHIFT, $v) };
}


macro_rules! face_change {
    ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_FACE_CHANGE_SHIFT, $v) };
}

unsafe extern "C" {
  /// C: lisp.h 4271: `extern Lisp_Object nconc2 (Lisp_Object, Lisp_Object);` (fns.c defines it).
  pub fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: frame.h 1508: `extern struct frame *root_frame (struct frame *f);` (frame.c defines it).
  pub fn root_frame (f: *mut frame) -> *mut frame;
  /// C: lisp.h 5097: `extern void safe_run_hooks (Lisp_Object);` (keyboard.c defines it).
  pub fn safe_run_hooks (hook: Lisp_Object);
  /// C: lisp.h 5055: `extern bool is_minibuffer (EMACS_INT, Lisp_Object);` (minibuf.c).
  pub fn is_minibuffer (depth: EMACS_INT, window: Lisp_Object) -> bool;
  /// C: lisp.h 5056: `extern EMACS_INT this_minibuffer_depth (Lisp_Object);`.
  pub fn this_minibuffer_depth (buffer: Lisp_Object) -> c_int;
  /// C: dispextern.h 3588: `void mark_window_display_accurate (Lisp_Object, bool);`
  /// (dispnew.c).
  pub fn mark_window_display_accurate (window: Lisp_Object, accurate_p: bool);
  /// C: dispextern.h 3903: `void free_window_matrices (struct window *);`.
  pub fn free_window_matrices (w: *mut window);
  // indent.h 39-40's `vmotion` is the include/indent.rs mirror's now; the mirror spells
  // `struct window *` as `*mut c_void` (P-72 rule 3, indent.rs cannot name the type), so the
  // six call sites below cast - the copy this unit used to keep spelled the real type, which
  // is exactly the second spelling P-26 asks to drop (status 3.371).
}



unsafe extern "C" {
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);` (eval.c; the mirror does not carry
  ///   it — P-132: this unit used to redefine it as a symbol-only macro, which rejected
  ///   closures that the real `FUNCTIONP` accepts).
  fn FUNCTIONP (o: Lisp_Object) -> bool;
  /// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));` (eval.c).
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
  /// C: lisp.h 4945: `extern Lisp_Object other_buffer_safely (Lisp_Object);` (buffer.c).
  pub fn other_buffer_safely (buffer: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 5340: `extern Lisp_Object run_hook_with_args_2 (Lisp_Object, Lisp_Object,
  /// Lisp_Object);` (eval.c).
  pub fn run_hook_with_args_2 (hook: Lisp_Object, arg1: Lisp_Object, arg2: Lisp_Object)
    -> Lisp_Object;
  /// C: dispextern.h 3901: `extern void adjust_frame_glyphs (struct frame *);` (dispnew.c).
  pub fn adjust_frame_glyphs (f: *mut frame);
}
/* The file-scope objects.  `window.h`'s mirror declares these at the crate root (its
   `unsafe extern "C"` blocks, C 1100-1206), so the definitions live in a module that is
   glob-imported: an item shadows a glob import, while a second crate-root item of the same
   name would be `E0428` (PORTING.md P-21's mechanism, from the other side; the frame.rs
   `frame_globals` shape).  The `static` objects stay plain `static mut`.  */
pub mod window_globals {
  use super::*;

  /* C 84: `Lisp_Object selected_window;`  */
  #[unsafe(no_mangle)]
  pub static mut selected_window: Lisp_Object = lisp_h_Qnil ();

  /* C 94: `Lisp_Object Vwindow_list;`  */
  #[unsafe(no_mangle)]
  pub static mut Vwindow_list: Lisp_Object = lisp_h_Qnil ();

  /* C 103: `Lisp_Object minibuf_window;`  */
  #[unsafe(no_mangle)]
  pub static mut minibuf_window: Lisp_Object = lisp_h_Qnil ();

  /* C 107: `Lisp_Object minibuf_selected_window;`  */
  #[unsafe(no_mangle)]
  pub static mut minibuf_selected_window: Lisp_Object = lisp_h_Qnil ();

  /* C 110: `Lisp_Object last_mouse_window;`  */
  #[unsafe(no_mangle)]
  pub static mut last_mouse_window: Lisp_Object = lisp_h_Qnil ();

  /* C 506: `EMACS_INT window_select_count;`  */
  #[unsafe(no_mangle)]
  pub static mut window_select_count: EMACS_INT = 0;
}
use window_globals::*;

/* C 89: `static Lisp_Object old_selected_window;` — the value of selected_window at the last
   time window change functions were run.  */
static mut old_selected_window: Lisp_Object = lisp_h_Qnil ();

/* C 97: `static bool window_change_record_frames;` — true means `window_change_record` has to
   record all live frames.  */
static mut window_change_record_frames: bool = false;

/* C 113: `static EMACS_INT sequence_number;` — incremented for each window created.  */
static mut sequence_number: EMACS_INT = 0;

/* C 116-117: used by `window_scroll_pixel_based`.  */
static mut window_scroll_pixel_based_preserve_x: c_int = 0;
static mut window_scroll_pixel_based_preserve_y: c_int = 0;

/* C 120-121: same for `window_scroll_line_based`.  */
static mut window_scroll_preserve_hpos: EMACS_INT = 0;
static mut window_scroll_preserve_vpos: EMACS_INT = 0;

unsafe fn CHECK_WINDOW_CONFIGURATION (x: Lisp_Object)
{
  CHECK_TYPE (WINDOW_CONFIGURATIONP! (x), Qwindow_configuration_p (), x);
}

/* These setters are used only in this file, so they can be private.  */
unsafe fn wset_combination_limit (w: *mut window, val: Lisp_Object)
{
  (*w).combination_limit = val;
}

unsafe fn wset_dedicated (w: *mut window, val: Lisp_Object)
{
  (*w).dedicated = val;
}

unsafe fn wset_display_table (w: *mut window, val: Lisp_Object)
{
  (*w).display_table = val;
}

unsafe fn wset_new_normal (w: *mut window, val: Lisp_Object)
{
  (*w).new_normal = val;
}

unsafe fn wset_new_total (w: *mut window, val: Lisp_Object)
{
  (*w).new_total = val;
}

unsafe fn wset_normal_cols (w: *mut window, val: Lisp_Object)
{
  (*w).normal_cols = val;
}

unsafe fn wset_normal_lines (w: *mut window, val: Lisp_Object)
{
  (*w).normal_lines = val;
}

unsafe fn wset_parent (w: *mut window, val: Lisp_Object)
{
  (*w).parent = val;
}

unsafe fn wset_pointm (w: *mut window, val: Lisp_Object)
{
  (*w).pointm = val;
}

unsafe fn wset_old_pointm (w: *mut window, val: Lisp_Object)
{
  (*w).old_pointm = val;
}

unsafe fn wset_start (w: *mut window, val: Lisp_Object)
{
  (*w).start = val;
}

unsafe fn wset_temslot (w: *mut window, val: Lisp_Object)
{
  (*w).temslot = val;
}

unsafe fn wset_vertical_scroll_bar_type (w: *mut window, val: Lisp_Object)
{
  (*w).vertical_scroll_bar_type = val;
}

unsafe fn wset_window_parameters (w: *mut window, val: Lisp_Object)
{
  (*w).window_parameters = val;
}

unsafe fn wset_combination (w: *mut window, horflag: bool, val: Lisp_Object)
{
  /* Since leaf windows never becomes non-leaf, there should
     be no buffer and markers in start and pointm fields of W.  */
  eassert! (!BUFFERP ((*w).contents) && NILP ((*w).start) && NILP ((*w).pointm));
  (*w).contents = val;
  /* When an internal window is deleted and VAL is nil, HORFLAG
     is meaningless.  */
  if !NILP (val)
    { window_bits! (w, horizontal, horflag); }
}

/* True if leaf window W doesn't reflect the actual state
   of displayed buffer due to its text or overlays change.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_outdated (w: *mut window) -> bool
{
  let b: *mut buffer = XBUFFER ((*w).contents);
  return ((*w).last_modified < BUF_MODIFF! (b)
	  || (*w).last_overlay_modified < BUF_OVERLAY_MODIFF! (b));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_live_window (window: Lisp_Object) -> *mut window
{
  let mut window = window;
  if NILP (window)
    { return XWINDOW (selected_window); }

  CHECK_LIVE_WINDOW (window);
  return XWINDOW (window);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_any_window (window: Lisp_Object) -> *mut window
{
  let mut window = window;
  let w: *mut window;

  if NILP (window)
    { return XWINDOW (selected_window); }

  CHECK_WINDOW (window);
  w = XWINDOW (window);
  return w;
}

unsafe fn decode_valid_window (window: Lisp_Object) -> *mut window
{
  let mut window = window;
  let w: *mut window;

  if NILP (window)
    { return XWINDOW (selected_window); }

  CHECK_VALID_WINDOW (window);
  w = XWINDOW (window);
  return w;
}

/* Called when W's buffer slot is changed.  ARG -1 means that W is about to
   cease its buffer, and 1 means that W is about to set up the new one.  */

unsafe fn adjust_window_count (w: *mut window, arg: c_int)
{
  eassert! (eabs! (arg) == 1);
  if BUFFERP ((*w).contents)
    {
      let mut b: *mut buffer = XBUFFER ((*w).contents);

      if !(*b).base_buffer.is_null ()
	{ b = (*b).base_buffer; }
      (*b).window_count += arg;
      eassert! ((*b).window_count >= 0);
      /* These should be recalculated by redisplay code.  */
      window_bits! (w, window_end_valid, false);
      (*w).base_line_pos = 0;
    }
}

/* Set W's buffer slot to VAL and recompute number
   of windows showing VAL if it is a buffer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wset_buffer (w: *mut window, val: Lisp_Object)
{
  adjust_window_count (w, -1);
  if BUFFERP (val)
    /* Make sure that we do not assign the buffer
       to an internal window.  */
    { eassert! (MARKERP ((*w).start) && MARKERP ((*w).pointm)); }
  (*w).contents = val;
  adjust_window_count (w, 1);
}

unsafe fn wset_old_buffer (w: *mut window, val: Lisp_Object)
{
  (*w).old_buffer = val;
}

DEFUN! ("windowp", Fwindowp, Swindowp, 1, 1, 0,
       doc: "Return t if OBJECT is a window and nil otherwise.  ",
       (object: Lisp_Object),
       {
  return if WINDOWP (object) { Qt () } else { Qnil () };
});

DEFUN! ("window-valid-p", Fwindow_valid_p, Swindow_valid_p, 1, 1, 0,
       doc: "Return t if OBJECT is a valid window and nil otherwise.\nA valid window is either a window that displays a buffer or an internal\nwindow.  Windows that have been deleted are not valid.  ",
       (object: Lisp_Object),
       {
  return if WINDOW_VALID_P (object) { Qt () } else { Qnil () };
});

DEFUN! ("window-live-p", Fwindow_live_p, Swindow_live_p, 1, 1, 0,
       doc: "Return t if OBJECT is a live window and nil otherwise.\nA live window is a window that displays a buffer.\nInternal windows and deleted windows are not live.  ",
       (object: Lisp_Object),
       {
  return if WINDOW_LIVE_P (object) { Qt () } else { Qnil () };
});

/* Frames and windows.  */
DEFUN! ("window-frame", Fwindow_frame, Swindow_frame, 0, 1, 0,
       doc: "Return the frame that window WINDOW is on.\nWINDOW must be a valid window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return (*decode_valid_window (window)).frame;
});

DEFUN! ("frame-root-window", Fframe_root_window, Sframe_root_window, 0, 1, 0,
       doc: "Return the root window of FRAME-OR-WINDOW.\nIf omitted, FRAME-OR-WINDOW defaults to the currently selected frame.\nWith a frame argument, return that frame's root window.\nWith a window argument, return the root window of that window's frame.  ",
       (frame_or_window: Lisp_Object),
       {
  let mut window: Lisp_Object;

  if NILP (frame_or_window)
    { window = (*SELECTED_FRAME ()).root_window; }
  else if WINDOW_VALID_P (frame_or_window)
      { window = (*XFRAME ((*XWINDOW (frame_or_window)).frame)).root_window; }
  else
    {
      CHECK_LIVE_FRAME! (frame_or_window);
      window = (*XFRAME (frame_or_window)).root_window;
    }

  return window;
});

DEFUN! ("minibuffer-window", Fminibuffer_window, Sminibuffer_window, 0, 1, 0,
       doc: "Return the minibuffer window for frame FRAME.\nIf FRAME is omitted or nil, it defaults to the selected frame.  ",
       (frame: Lisp_Object),
       {
  return FRAME_MINIBUF_WINDOW (decode_live_frame (frame));
});

DEFUN! ("window-minibuffer-p", Fwindow_minibuffer_p,
       Swindow_minibuffer_p, 0, 1, 0,
       doc: "Return t if WINDOW is a minibuffer window.\nWINDOW must be a valid window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return if MINI_WINDOW_P (decode_valid_window (window)) { Qt () } else { Qnil () };
});

/* Don't move this to window.el - this must be a safe routine.  */
DEFUN! ("frame-first-window", Fframe_first_window, Sframe_first_window, 0, 1, 0,
       doc: "Return the topmost, leftmost live window on FRAME-OR-WINDOW.\nIf omitted, FRAME-OR-WINDOW defaults to the currently selected frame.\nElse if FRAME-OR-WINDOW denotes a valid window, return the first window\nof that window's frame.  If FRAME-OR-WINDOW denotes a live frame, return\nthe first window of that frame.  ",
       (frame_or_window: Lisp_Object),
       {
  let mut window: Lisp_Object;

  if NILP (frame_or_window)
    { window = (*SELECTED_FRAME ()).root_window; }
  else if WINDOW_VALID_P (frame_or_window)
    { window = (*XFRAME (WINDOW_FRAME! (XWINDOW (frame_or_window)))).root_window; }
  else
    {
      CHECK_LIVE_FRAME! (frame_or_window);
      window = (*XFRAME (frame_or_window)).root_window;
    }

  while WINDOWP ((*XWINDOW (window)).contents)
    { window = (*XWINDOW (window)).contents; }

  return window;
});

DEFUN! ("frame-selected-window", Fframe_selected_window,
       Sframe_selected_window, 0, 1, 0,
       doc: "Return the selected window of FRAME-OR-WINDOW.\nIf omitted, FRAME-OR-WINDOW defaults to the currently selected frame.\nElse if FRAME-OR-WINDOW denotes a valid window, return the selected\nwindow of that window's frame.  If FRAME-OR-WINDOW denotes a live frame,\nreturn the selected window of that frame.  ",
       (frame_or_window: Lisp_Object),
       {
  let mut window: Lisp_Object;

  if NILP (frame_or_window)
    { window = (*SELECTED_FRAME ()).selected_window; }
  else if WINDOW_VALID_P (frame_or_window)
    { window = (*XFRAME (WINDOW_FRAME! (XWINDOW (frame_or_window)))).selected_window; }
  else
    {
      CHECK_LIVE_FRAME! (frame_or_window);
      window = (*XFRAME (frame_or_window)).selected_window;
    }

  return window;
});

DEFUN! ("frame-old-selected-window", Fframe_old_selected_window,
       Sframe_old_selected_window, 0, 1, 0,
       doc: "Return old selected window of FRAME.\nFRAME must be a live frame and defaults to the selected one.\n\nThe return value is the window selected on FRAME the last time window\nchange functions were run for FRAME.  ",
       (frame: Lisp_Object),
       {
  let mut frame = frame;
  if NILP (frame)
    { frame = selected_frame; }
  CHECK_LIVE_FRAME! (frame);

  return (*XFRAME (frame)).old_selected_window;
});

DEFUN! ("set-frame-selected-window", Fset_frame_selected_window,
       Sset_frame_selected_window, 2, 3, 0,
       doc: "Set selected window of FRAME to WINDOW.\nFRAME must be a live frame and defaults to the selected one.  If FRAME\nis the selected frame, this makes WINDOW the selected window.  Optional\nargument NORECORD non-nil means to neither change the order of recently\nselected windows nor the buffer list.  WINDOW must denote a live window.\nReturn WINDOW.  ",
       (frame: Lisp_Object, window: Lisp_Object, norecord: Lisp_Object),
       {
  let mut frame = frame;
  if NILP (frame)
    { frame = selected_frame; }

  CHECK_LIVE_FRAME! (frame);
  CHECK_LIVE_WINDOW (window);

  if ! EQ (frame, WINDOW_FRAME! (XWINDOW (window)))
    { error (c"In `set-frame-selected-window', WINDOW is not on FRAME".as_ptr ()); }

  if EQ (frame, selected_frame)
    { return Fselect_window (window, norecord); }
  else
    {
      fset_selected_window (XFRAME (frame), window);
      /* Don't clear FRAME's select_mini_window_flag here.  */
      return window;
    }
});

DEFUN! ("selected-window", Fselected_window, Sselected_window, 0, 0, 0,
       doc: "Return the selected window.\nThe selected window is the window in which the standard cursor for\nselected windows appears and to which many commands apply.\n\nAlso see `old-selected-window' and `minibuffer-selected-window'.  ",
       (),
       {
  return selected_window;
});

DEFUN! ("old-selected-window", Fold_selected_window,
       Sold_selected_window, 0, 0, 0,
       doc: "Return the old selected window.\nThe return value is the window selected the last time window change\nfunctions were run.  ",
       (),
       {
  return old_selected_window;
});

/* If select_window is called with inhibit_point_swap true it will
   not store point of the old selected window's buffer back into that
   window's pointm slot.  This is needed by Fset_window_configuration to
   avoid that the display routine is called with selected_window set to
   Qnil causing a subsequent crash.  */
unsafe fn select_window (window: Lisp_Object, norecord: Lisp_Object,
	       inhibit_point_swap: bool) -> Lisp_Object
{
  let w: *mut window;
  let mut sf: *mut frame;
  let frame: Lisp_Object;
  let mut f: *mut frame;

  CHECK_LIVE_WINDOW (window);

  w = XWINDOW (window);
  frame = WINDOW_FRAME! (w);
  f = XFRAME (frame);

  if FRAME_TOOLTIP_P (f)
    /* Do not select a tooltip window (Bug#47207).  */
    { error (c"Cannot select a tooltip window".as_ptr ()); }

  /* We definitely want to select WINDOW, not the mini-window.  */
  select_mini_window_flag! (f, false);

  /* Make the selected window's buffer current.  */
  Fset_buffer ((*w).contents);

  'record_and_return: {
  if EQ (window, selected_window) && !inhibit_point_swap
    /* `switch-to-buffer' uses (select-window (selected-window)) as a "clever"
       way to call record_buffer from Elisp, so it's important that we call
       record_buffer before returning here.  */
    { break 'record_and_return; }

  if NILP (norecord) || EQ (norecord, Qmark_for_redisplay ())
    { /* Mark the window for redisplay since the selected-window has
	 a different mode-line.  */
      wset_redisplay (XWINDOW (selected_window));
      wset_redisplay (w);
    }
  else
    { redisplay_other_windows (); }

  sf = SELECTED_FRAME ();
  if f != sf
    {
      fset_selected_window (f, window);
      /* Use this rather than Fhandle_switch_frame
	 so that FRAME_FOCUS_FRAME is moved appropriately as we
	 move around in the state where a minibuffer in a separate
	 frame is active.  */
      Fselect_frame (frame, norecord);
      /* Fselect_frame called us back so we've done all the work already.  */
      eassert! (EQ (window, selected_window)
	       || (EQ (window, (*f).minibuffer_window)
		   && NILP (Fminibufferp ((*XWINDOW (window)).contents, Qt ()))));
      return window;
    }
  else
    { fset_selected_window (sf, window); }

  select_window_1 (window, inhibit_point_swap);
  bset_last_selected_window (XBUFFER ((*w).contents), window);
  }

  /* record_buffer can call maybe_quit, so make sure it is run only
     after we have re-established the invariant between
     selected_window and selected_frame, otherwise the temporary
     broken invariant might "escape" (Bug#14161).  */
  if NILP (norecord)
    {
      (*w).use_time = { window_select_count += 1; window_select_count };
      record_buffer ((*w).contents);
    }

  return window;
}

/* Select window with a minimum of fuss, i.e. don't record the change anywhere
   (not even for redisplay's benefit), and assume that the window's frame is
   already selected.  */
unsafe fn select_window_1 (window: Lisp_Object, inhibit_point_swap: bool)
{
  /* Store the old selected window's buffer's point in pointm of the old
     selected window.  It belongs to that window, and when the window is
     not selected, must be in the window.  */
  if !inhibit_point_swap
    {
      let ow: *mut window = XWINDOW (selected_window);
      if BUFFERP ((*ow).contents)
	{ set_marker_both ((*ow).pointm, (*ow).contents,
			   BUF_PT (XBUFFER ((*ow).contents)),
			   BUF_PT_BYTE (XBUFFER ((*ow).contents))); }
    }

  selected_window = window;

  /* Go to the point recorded in the window.
     This is important when the buffer is in more
     than one window.  It also matters when
     redisplay_window has altered point after scrolling,
     because it makes the change only in the window.  */
  set_point_from_marker ((*XWINDOW (window)).pointm);
}

DEFUN! ("select-window", Fselect_window, Sselect_window, 1, 2, 0,
       doc: "Select WINDOW which must be a live window.\nAlso make WINDOW's frame the selected frame and WINDOW that frame's\nselected window.  In addition, make WINDOW's buffer current and set its\nbuffer's value of `point' to the value of WINDOW's `window-point'.\nReturn WINDOW.\n\nOptional second arg NORECORD non-nil means do not put this buffer at the\nfront of the buffer list and do not make this window the most recently\nselected one.  Also, do not mark WINDOW for redisplay unless NORECORD\nequals the special symbol `mark-for-redisplay'.\n\nRun `buffer-list-update-hook' unless NORECORD is non-nil.  Note that\napplications and internal routines often select a window temporarily for\nvarious purposes; mostly, to simplify coding.  As a rule, such\nselections should not be recorded and therefore will not pollute\n`buffer-list-update-hook'.  Selections that \"really count\" are those\ncausing a visible change in the next redisplay of WINDOW's frame and\nshould always be recorded.  So if you think of running a function each\ntime a window gets selected, put it on `buffer-list-update-hook' or\n`window-selection-change-functions'.\n\nAlso note that the main editor command loop sets the current buffer to\nthe buffer of the selected window before each command.  ",
       (window: Lisp_Object, norecord: Lisp_Object),
       {
  return select_window (window, norecord, false);
});

DEFUN! ("window-buffer", Fwindow_buffer, Swindow_buffer, 0, 1, 0,
       doc: "Return the buffer displayed in window WINDOW.\nIf WINDOW is omitted or nil, it defaults to the selected window.\nReturn nil for an internal window or a deleted window.  ",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_any_window (window);

  return if WINDOW_LEAF_P (w) { (*w).contents } else { Qnil () };
});

DEFUN! ("window-old-buffer", Fwindow_old_buffer, Swindow_old_buffer, 0, 1, 0,
       doc: "Return the old buffer displayed by WINDOW.\nWINDOW can be any window and defaults to the selected one.\n\nThe return value is the buffer shown in WINDOW at the last time window\nchange functions were run or WINDOW is a former live window that was\ndeleted.  It is nil if WINDOW was created after that.  It is t if WINDOW\nhas been restored from a window configuration after that.  It is always\nnil if WINDOW is an internal window.  ",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_any_window (window);

  return if NILP ((*w).old_buffer)
	  /* A new window.  */
	  { Qnil () }
	  else if (*w).change_stamp != (*WINDOW_XFRAME! (w)).change_stamp
	  /* A window restored from a configuration.  */
	  { Qt () }
	  /* A window that was live the last time seen by window change
	     functions or was deleted.  */
	  else { (*w).old_buffer };
});

DEFUN! ("window-parent", Fwindow_parent, Swindow_parent, 0, 1, 0,
       doc: "Return the parent window of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\nReturn nil for a window with no parent (e.g. a root window).  ",
       (window: Lisp_Object),
       {
  return (*decode_valid_window (window)).parent;
});

DEFUN! ("window-top-child", Fwindow_top_child, Swindow_top_child, 0, 1, 0,
       doc: "Return the topmost child window of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\nReturn nil if WINDOW is a live window (live windows have no children).\nReturn nil if WINDOW is an internal window whose children form a\nhorizontal combination.  ",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_valid_window (window);
  return if WINDOW_VERTICAL_COMBINATION_P (w) { (*w).contents } else { Qnil () };
});

DEFUN! ("window-left-child", Fwindow_left_child, Swindow_left_child, 0, 1, 0,
       doc: "Return the leftmost child window of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\nReturn nil if WINDOW is a live window (live windows have no children).\nReturn nil if WINDOW is an internal window whose children form a\nvertical combination.  ",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_valid_window (window);
  return if WINDOW_HORIZONTAL_COMBINATION_P (w) { (*w).contents } else { Qnil () };
});

DEFUN! ("window-next-sibling", Fwindow_next_sibling, Swindow_next_sibling, 0, 1, 0,
       doc: "Return the next sibling window of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\nReturn nil if WINDOW has no next sibling.  ",
       (window: Lisp_Object),
       {
  return (*decode_valid_window (window)).next;
});

DEFUN! ("window-prev-sibling", Fwindow_prev_sibling, Swindow_prev_sibling, 0, 1, 0,
       doc: "Return the previous sibling window of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\nReturn nil if WINDOW has no previous sibling.  ",
       (window: Lisp_Object),
       {
  return (*decode_valid_window (window)).prev;
});

DEFUN! ("window-combination-limit", Fwindow_combination_limit, Swindow_combination_limit, 1, 1, 0,
       doc: "Return combination limit of window WINDOW.\nWINDOW must be a valid window used in horizontal or vertical combination.\nIf the return value is nil, child windows of WINDOW can be recombined with\nWINDOW's siblings.  A return value of t means that child windows of\nWINDOW are never (re-)combined with WINDOW's siblings.  ",
       (window: Lisp_Object),
       {
  let w: *mut window;

  CHECK_VALID_WINDOW (window);
  w = XWINDOW (window);
  if WINDOW_LEAF_P (w)
    { error (c"Combination limit is meaningful for internal windows only".as_ptr ()); }
  return (*w).combination_limit;
});

DEFUN! ("set-window-combination-limit", Fset_window_combination_limit, Sset_window_combination_limit, 2, 2, 0,
       doc: "Set combination limit of window WINDOW to LIMIT; return LIMIT.\nWINDOW must be a valid window used in horizontal or vertical combination.\nIf LIMIT is nil, child windows of WINDOW can be recombined with WINDOW's\nsiblings.  LIMIT t means that child windows of WINDOW are never\n(re-)combined with WINDOW's siblings.  Other values are reserved for\nfuture use.  ",
       (window: Lisp_Object, limit: Lisp_Object),
       {
  let w: *mut window;

  CHECK_VALID_WINDOW (window);
  w = XWINDOW (window);
  if WINDOW_LEAF_P (w)
    { error (c"Combination limit is meaningful for internal windows only".as_ptr ()); }
  wset_combination_limit (w, limit);
  return limit;
});

DEFUN! ("window-use-time", Fwindow_use_time, Swindow_use_time, 0, 1, 0,
       doc: "Return the use time of window WINDOW.\nWINDOW must specify a live window and defaults to the selected one.\n\nThe window with the highest use time is usually the one most recently\nselected by calling `select-window' with NORECORD nil.  The window with\nthe lowest use time is usually the least recently selected one chosen in\nsuch a way.\n\nNote that the use time of a window can be also changed by calling\n`window-bump-use-time' for that window.  ",
       (window: Lisp_Object),
       {
  return make_fixnum ((*decode_live_window (window)).use_time);
});

DEFUN! ("window-bump-use-time", Fwindow_bump_use_time,
       Swindow_bump_use_time, 0, 1, 0,
       doc: "Mark WINDOW as second most recently used.\nWINDOW must specify a live window.\n\nIf WINDOW is not selected and the selected window has the highest use\ntime of all windows, set the use time of WINDOW to that of the selected\nwindow, increase the use time of the selected window by one and return\nthe new use time of WINDOW.  Otherwise, do nothing and return nil.  ",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);
  let sw: *mut window = XWINDOW (selected_window);

  if w != sw && (*sw).use_time == window_select_count
    {
      (*w).use_time = window_select_count;
      (*sw).use_time = { window_select_count += 1; window_select_count };

      return make_fixnum ((*w).use_time);
    }
  else
    { return Qnil (); }
});

DEFUN! ("window-pixel-width", Fwindow_pixel_width, Swindow_pixel_width, 0, 1, 0,
       doc: "Return the width of window WINDOW in pixels.\nWINDOW must be a valid window and defaults to the selected one.\n\nThe return value includes the fringes and margins of WINDOW as well as\nany vertical dividers or scroll bars belonging to WINDOW.  If WINDOW is\nan internal window, its pixel width is the width of the screen areas\nspanned by its children.  ",
       (window: Lisp_Object),
       {
  return make_fixnum ((*decode_valid_window (window)).pixel_width as EMACS_INT);
});

DEFUN! ("window-pixel-height", Fwindow_pixel_height, Swindow_pixel_height, 0, 1, 0,
       doc: "Return the height of window WINDOW in pixels.\nWINDOW must be a valid window and defaults to the selected one.\n\nThe return value includes the mode line and header line and the bottom\ndivider, if any.  If WINDOW is an internal window, its pixel height is\nthe height of the screen areas spanned by its children.  ",
       (window: Lisp_Object),
       {
  return make_fixnum ((*decode_valid_window (window)).pixel_height as EMACS_INT);
});

/* C: `include/dispextern.rs` is the WIP mirror of `dispextern.h` and `include/font.rs` the WIP
   mirror of `font.h`; `struct font` (font.h 275-418) — this unit reads `height`,
   `space_width` and `average_width` of it — now comes from that mirror.  The records still
   declared here at their use site (PORTING.md P-72 ③) are `struct face` (only `font`) and
   `struct face_cache`, plus `FRAME_FACE_CACHE`, `FACE_FROM_ID_OR_NULL` and
   `lookup_named_face`, with the frozen clang's offsets (`clang -std=gnu23 -I
   build/emacs-ref/{lib,src} …`: `face.font` 216 and `sizeof face` 280).  Reported as a
   mirror gap in docs/ports/window.md.  */

#[repr(C)]
pub struct face
{
  pub lface: [Lisp_Object; LFACE_VECTOR_SIZE as usize],
  /* `font` is at 216 in a non-window-system build and at **232** with the `gc`/`stipple` block
     (168/176); Android has that block, so 216 read `underline_color` as the font pointer
     (docs/ports/layout-gaps.md, status 4.816).  */
  _pad0: [u8; 216 - 160 + (if cfg! (feature = "have_window_system") { 16 } else { 0 })],
  pub font: *mut font,
  _pad1: [u8; 280 - 224],
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, font) == 232);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::size_of::<face> () == 296);

#[repr(C)]
pub struct face_cache
{
  pub buckets: *mut *mut face,
  pub f: *mut frame,
  pub faces_by_id: *mut *mut face,
  pub size: ptrdiff_t,
  pub used: c_int,
  pub menu_face_changed_p: bool,
}
const _: () = core::assert! (std::mem::size_of::<face_cache> () == 40);
const _: () = core::assert! (std::mem::offset_of! (face_cache, used) == 32);

/// C: frame.h 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache` — `frame.face_cache` is an
/// opaque pointer in `include/frame.rs`, so the cast is the use-site conversion (P-72 ③).
macro_rules! FRAME_FACE_CACHE {
  ($f:expr) => { (*$f).face_cache as *mut face_cache };
}

/// C: frame.h 1656: `INLINE struct face *FACE_FROM_ID_OR_NULL (struct frame *f, int id);`
/// `include/font.rs` carries `FACE_FROM_ID` but not the "_OR_NULL" sibling (gap).
unsafe fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
{
  let used: c_int = (*FRAME_FACE_CACHE! (f)).used;
  eassume! (0 <= used);
  return if 0 <= id && id < used
    { *((*FRAME_FACE_CACHE! (f)).faces_by_id).offset (id as isize) }
    else { std::ptr::null_mut () };
}



/* dispextern.h 286-287 and 322-326: `BYTEPOS`/`CHARPOS` (both usable as an assignment target)
   and `SET_TEXT_POS_FROM_MARKER`; the mirror does not carry them (PORTING.md P-72 ③; the
   dispnew.rs/indent.rs shape).  */
macro_rules! BYTEPOS { ($pos:expr) => { ($pos).bytepos }; }
macro_rules! CHARPOS { ($pos:expr) => { ($pos).charpos }; }
macro_rules! SET_TEXT_POS_FROM_MARKER { ($pos:expr, $marker:expr) => {
  { CHARPOS! ($pos) = marker_position ($marker); BYTEPOS! ($pos) = marker_byte_position ($marker); } }; }

unsafe extern "C" {
  /// C: dispextern.h 3619-3621: `bool pos_visible_p (struct window *, ptrdiff_t, int *, int *,
  /// int *, int *, int *, int *);` (xdisp.c defines it).
  pub fn pos_visible_p (w: *mut window, charpos: ptrdiff_t, x: *mut c_int, y: *mut c_int,
                        rtop: *mut c_int, rbot: *mut c_int, rowh: *mut c_int, vpos: *mut c_int)
    -> bool;
  /// C: dispextern.h 3577: `int window_text_bottom_y (struct window *);` (xdisp.c).
  pub fn window_text_bottom_y (w: *mut window) -> c_int;
  // window.h 1176's `wset_update_mode_line` (xdisp.c defines it) is the mirror's `window_fn_decls`
  // now; the use-site copy is gone (P-26).
}
unsafe extern "C" {
  /// C: dispextern.h 3578: `int window_box_width (struct window *, enum glyph_row_area);`
  /// (xdisp.c defines it; the mirror does not carry it.)
  pub fn window_box_width (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3579: `int window_box_left (struct window *, enum glyph_row_area);`
  pub fn window_box_left (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3577: `int window_box (struct window *, enum glyph_row_area, int *, int *,
  /// int *, int *);` (dispextern.h; the mirror does not carry it.)
  pub fn window_box (w: *mut window, area: glyph_row_area, box_x: *mut c_int, box_y: *mut c_int,
                     box_width: *mut c_int, box_height: *mut c_int);
  /// C: dispextern.h 3576: `int window_box_height (struct window *);`
  pub fn window_box_height (w: *mut window) -> c_int;
  /// C: dispextern.h 3580: `int window_box_left_offset (struct window *, enum glyph_row_area);`
  pub fn window_box_left_offset (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3581: `int window_box_right_offset (struct window *, enum glyph_row_area);`
  pub fn window_box_right_offset (w: *mut window, area: glyph_row_area) -> c_int;
}
unsafe extern "C" {
  /// C: dispextern.h 3812: `int lookup_named_face (struct window *, struct frame *,
  /// Lisp_Object, bool);` (xfaces.c defines it; the mirror does not carry it).
  pub fn lookup_named_face (w: *mut window, f: *mut frame, symbol: Lisp_Object, signal_p: bool)
    -> c_int;
}

DEFUN! ("window-old-pixel-width", Fwindow_old_pixel_width,
       Swindow_old_pixel_width, 0, 1, 0,
       doc: "Return old total pixel width of WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\n\nThe return value is the total pixel width of WINDOW after the last\ntime window change functions found WINDOW live on its frame.  It is\nzero if WINDOW was created after that.  ",
       (window: Lisp_Object),
       {
  return (make_fixnum
	  ((*decode_valid_window (window)).old_pixel_width as EMACS_INT));
});

DEFUN! ("window-old-pixel-height", Fwindow_old_pixel_height,
       Swindow_old_pixel_height, 0, 1, 0,
       doc: "Return old total pixel height of WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\n\nThe return value is the total pixel height of WINDOW after the last\ntime window change functions found WINDOW live on its frame.  It is\nzero if WINDOW was created after that.  ",
       (window: Lisp_Object),
       {
  return (make_fixnum
	  ((*decode_valid_window (window)).old_pixel_height as EMACS_INT));
});

DEFUN! ("window-total-height", Fwindow_total_height, Swindow_total_height, 0, 2, 0,
       doc: "Return the height of window WINDOW in canonical lines.\nWINDOW must be a valid window and defaults to the selected one.\n\nThe return value includes the heights of WINDOW's mode and header line\nand its bottom divider, if any.  If WINDOW is an internal window, the\ntotal height is the height of the screen areas spanned by its children.\n\nIf WINDOW's pixel height is not an integral multiple of its frame's\ncanonical character height, the number of lines occupied by WINDOW is\nrounded internally.  This is done in a way such that, if WINDOW is a\nparent window, the sum of the total heights of all its children\ninternally equals the total height of WINDOW.\n\nIf the optional argument ROUND is `ceiling', return the smallest integer\nlarger than WINDOW's pixel height divided by the canonical character\nheight of WINDOW's frame.  ROUND `floor' means to return the largest\ninteger smaller than WINDOW's pixel height divided by the canonical\ncharacter height of WINDOW's frame.  Any other value of ROUND means to\nreturn the internal total height of WINDOW.  ",
       (window: Lisp_Object, round: Lisp_Object),
       {
  let w: *mut window = decode_valid_window (window);

  if ! EQ (round, Qfloor ()) && ! EQ (round, Qceiling ())
    { return make_fixnum ((*w).total_lines as EMACS_INT); }
  else
    {
      let unit: c_int = FRAME_LINE_HEIGHT (WINDOW_XFRAME! (w));

      return make_fixnum ((if EQ (round, Qceiling ())
			  { ((*w).pixel_height + unit - 1) / unit }
			  else { (*w).pixel_height / unit }) as EMACS_INT);
    }
});

DEFUN! ("window-total-width", Fwindow_total_width, Swindow_total_width, 0, 2, 0,
       doc: "Return the total width of window WINDOW in canonical columns.\nWINDOW must be a valid window and defaults to the selected one.\n\nThe return value includes the widths of WINDOW's fringes, margins,\nscroll bars and its right divider, if any.  If WINDOW is an internal\nwindow, the total width is the width of the screen areas spanned by its\nchildren.\n\nIf WINDOW's pixel width is not an integral multiple of its frame's\ncanonical character width, the number of lines occupied by WINDOW is\nrounded internally.  This is done in a way such that, if WINDOW is a\nparent window, the sum of the total widths of all its children\ninternally equals the total width of WINDOW.\n\nIf the optional argument ROUND is `ceiling', return the smallest integer\nlarger than WINDOW's pixel width divided by the canonical character\nwidth of WINDOW's frame.  ROUND `floor' means to return the largest\ninteger smaller than WINDOW's pixel width divided by the canonical\ncharacter width of WINDOW's frame.  Any other value of ROUND means to\nreturn the internal total width of WINDOW.  ",
       (window: Lisp_Object, round: Lisp_Object),
       {
  let w: *mut window = decode_valid_window (window);

  if ! EQ (round, Qfloor ()) && ! EQ (round, Qceiling ())
    { return make_fixnum ((*w).total_cols as EMACS_INT); }
  else
    {
      let unit: c_int = FRAME_COLUMN_WIDTH (WINDOW_XFRAME! (w));

      return make_fixnum ((if EQ (round, Qceiling ())
			  { ((*w).pixel_width + unit - 1) / unit }
			  else { (*w).pixel_width / unit }) as EMACS_INT);
    }
});

DEFUN! ("window-new-total", Fwindow_new_total, Swindow_new_total, 0, 1, 0,
       doc: "Return the new total size of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\n\nThe new total size of WINDOW is the value set by the last call of\n`set-window-new-total' for WINDOW.  If it is valid, it will be shortly\ninstalled as WINDOW's total height (see `window-total-height') or total\nwidth (see `window-total-width').  ",
       (window: Lisp_Object),
       {
  return (*decode_valid_window (window)).new_total;
});

DEFUN! ("window-normal-size", Fwindow_normal_size, Swindow_normal_size, 0, 2, 0,
       doc: "Return the normal height of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\nIf HORIZONTAL is non-nil, return the normal width of WINDOW.\n\nThe normal height of a frame's root window or a window that is\nhorizontally combined (a window that has a left or right sibling) is\n1.0.  The normal height of a window that is vertically combined (has a\nsibling above or below) is the fraction of the window's height with\nrespect to its parent.  The sum of the normal heights of all windows in a\nvertical combination equals 1.0.\n\nSimilarly, the normal width of a frame's root window or a window that is\nvertically combined equals 1.0.  The normal width of a window that is\nhorizontally combined is the fraction of the window's width with respect\nto its parent.  The sum of the normal widths of all windows in a\nhorizontal combination equals 1.0.\n\nThe normal sizes of windows are used to restore the proportional sizes\nof windows after they have been shrunk to their minimum sizes; for\nexample when a frame is temporarily made very small and afterwards gets\nre-enlarged to its previous size.  ",
       (window: Lisp_Object, horizontal: Lisp_Object),
       {
  let w: *mut window = decode_valid_window (window);

  return if NILP (horizontal) { (*w).normal_lines } else { (*w).normal_cols };
});

DEFUN! ("window-new-normal", Fwindow_new_normal, Swindow_new_normal, 0, 1, 0,
       doc: "Return new normal size of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\n\nThe new normal size of WINDOW is the value set by the last call of\n`set-window-new-normal' for WINDOW.  If valid, it will be shortly\ninstalled as WINDOW's normal size (see `window-normal-size').  ",
       (window: Lisp_Object),
       {
  return (*decode_valid_window (window)).new_normal;
});

DEFUN! ("window-new-pixel", Fwindow_new_pixel, Swindow_new_pixel, 0, 1, 0,
       doc: "Return new pixel size of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.\n\nThe new pixel size of WINDOW is the value set by the last call of\n`set-window-new-pixel' for WINDOW.  If it is valid, it will be shortly\ninstalled as WINDOW's pixel height (see `window-pixel-height') or pixel\nwidth (see `window-pixel-width').  ",
       (window: Lisp_Object),
       {
  return (*decode_valid_window (window)).new_pixel;
});

DEFUN! ("window-pixel-left", Fwindow_pixel_left, Swindow_pixel_left, 0, 1, 0,
       doc: "Return left pixel edge of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum ((*decode_valid_window (window)).pixel_left as EMACS_INT);
});

DEFUN! ("window-pixel-top", Fwindow_pixel_top, Swindow_pixel_top, 0, 1, 0,
       doc: "Return top pixel edge of window WINDOW.\nWINDOW must be a valid window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum ((*decode_valid_window (window)).pixel_top as EMACS_INT);
});

DEFUN! ("window-left-column", Fwindow_left_column, Swindow_left_column, 0, 1, 0,
       doc: "Return left column of window WINDOW.\nThis is the distance, in columns, between the left edge of WINDOW and\nthe left edge of the frame's window area.  For instance, the return\nvalue is 0 if there is no window to the left of WINDOW.\n\nWINDOW must be a valid window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum ((*decode_valid_window (window)).left_col as EMACS_INT);
});

DEFUN! ("window-top-line", Fwindow_top_line, Swindow_top_line, 0, 1, 0,
       doc: "Return top line of window WINDOW.\nThis is the distance, in lines, between the top of WINDOW and the top\nof the frame's window area.  For instance, the return value is 0 if\nthere is no window above WINDOW.\n\nWINDOW must be a valid window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum ((*decode_valid_window (window)).top_line as EMACS_INT);
});

unsafe fn window_body_unit_from_symbol (unit: Lisp_Object) -> window_body_unit
{
  return
    if EQ (unit, Qremap ()) { window_body_unit::WINDOW_BODY_IN_REMAPPED_CHARS }
    else if NILP (unit) { window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS }
    else { window_body_unit::WINDOW_BODY_IN_PIXELS };
}

/* Return the number of lines/pixels of W's body.  Don't count any mode
   or header line or horizontal divider of W.  Rounds down to nearest
   integer when not working pixelwise. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_body_height (w: *mut window,
                                             pixelwise: window_body_unit) -> c_int
{
  let mut pixelwise = pixelwise;
  let height: c_int = ((*w).pixel_height
		- WINDOW_TAB_LINE_HEIGHT! (w)
		- WINDOW_HEADER_LINE_HEIGHT! (w)
		- (if WINDOW_HAS_HORIZONTAL_SCROLL_BAR (w)
		   { WINDOW_SCROLL_BAR_AREA_HEIGHT! (w) }
		   else { 0 })
		- WINDOW_MODE_LINE_HEIGHT! (w)
		- WINDOW_BOTTOM_DIVIDER_WIDTH! (w));

  let mut denom: c_int = 1;
  if pixelwise == window_body_unit::WINDOW_BODY_IN_REMAPPED_CHARS
    {
      if !NILP (Vface_remapping_alist)
	{
	  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
	  let face_id: c_int = lookup_named_face (std::ptr::null_mut (), f, Qdefault (), true);
	  let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);
	  if !face.is_null () && !(*face).font.is_null () && (*(*face).font).height != 0
	    { denom = (*(*face).font).height; }
	}
      /* For performance, use canonical chars if no face remapping.  */
      else
	{ pixelwise = window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS; }
    }

  if pixelwise == window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS
    { denom = FRAME_LINE_HEIGHT (WINDOW_XFRAME! (w)); }

  /* Don't return a negative value.  */
  return max ((height / denom) as isize, 0) as c_int;
}

/* Return the number of columns/pixels of W's body.  Don't count columns
   occupied by the scroll bar or the divider/vertical bar separating W
   from its right sibling or margins.  On window-systems don't count
   fringes either.  Round down to nearest integer when not working
   pixelwise.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_body_width (w: *mut window,
                                            pixelwise: window_body_unit) -> c_int
{
  let mut pixelwise = pixelwise;
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));

  let width: c_int = ((*w).pixel_width
	       - WINDOW_RIGHT_DIVIDER_WIDTH! (w)
	       - (if WINDOW_HAS_VERTICAL_SCROLL_BAR! (w)
		  { WINDOW_SCROLL_BAR_AREA_WIDTH! (w) }
		  else { /* A vertical bar is either 1 or 0.  */
			 if !FRAME_WINDOW_P (f)
			    && !WINDOW_RIGHTMOST_P! (w)
			    && WINDOW_RIGHT_DIVIDER_WIDTH! (w) == 0
			   { 1 } else { 0 } })
		- WINDOW_MARGINS_WIDTH! (w)
		- (if FRAME_WINDOW_P (f) { WINDOW_FRINGES_WIDTH! (w) } else { 0 }));

  let mut denom: c_int = 1;
  if pixelwise == window_body_unit::WINDOW_BODY_IN_REMAPPED_CHARS
    {
      if !NILP (Vface_remapping_alist)
	{
	  let face_id: c_int = lookup_named_face (std::ptr::null_mut (), f, Qdefault (), true);
	  let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);
	  if !face.is_null () && !(*face).font.is_null ()
	    {
	      if (*(*face).font).average_width != 0
		{ denom = (*(*face).font).average_width; }
	      else if (*(*face).font).space_width != 0
		{ denom = (*(*face).font).space_width; }
	    }
	}
      /* For performance, use canonical chars if no face remapping.  */
      else
	{ pixelwise = window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS; }
    }

  if pixelwise == window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS
    { denom = FRAME_COLUMN_WIDTH (WINDOW_XFRAME! (w)); }

  /* Don't return a negative value.  */
  return max ((width / denom) as isize, 0) as c_int;
}

DEFUN! ("window-body-width", Fwindow_body_width, Swindow_body_width, 0, 2, 0,
       doc: "Return the width of WINDOW's text area.\nWINDOW must be a live window and defaults to the selected one.  The\nreturn value does not include any vertical dividers, fringes or\nmarginal areas, or scroll bars.\n\nThe optional argument PIXELWISE defines the units to use for the\nwidth.  If nil, return the largest integer smaller than WINDOW's pixel\nwidth in units of the canonical character width of WINDOW's frame.\nIf PIXELWISE is `remap' and the default face is remapped (see\n`face-remapping-alist'), use the remapped face to determine the\ncharacter width.  For any other non-nil value, return the width in\npixels.\n\nNote that the returned value includes the column reserved for the\ncontinuation glyph.\n\nAlso see `window-max-chars-per-line'.  ",
       (window: Lisp_Object, pixelwise: Lisp_Object),
       {
  return (make_fixnum
	  (window_body_width (decode_live_window (window),
			      window_body_unit_from_symbol (pixelwise)) as EMACS_INT));
});

DEFUN! ("window-body-height", Fwindow_body_height, Swindow_body_height, 0, 2, 0,
       doc: "Return the height of WINDOW's text area.\nWINDOW must be a live window and defaults to the selected one.  The\nreturn value does not include the mode line or header line or any\nhorizontal divider.\n\nThe optional argument PIXELWISE defines the units to use for the\nheight.  If nil, return the largest integer smaller than WINDOW's\npixel height in units of the canonical character height of WINDOW's\nframe.  If PIXELWISE is `remap' and the default face is remapped (see\n`face-remapping-alist'), use the remapped face to determine the\ncharacter height.  For any other non-nil value, return the height in\npixels.  ",
       (window: Lisp_Object, pixelwise: Lisp_Object),
       {
  return (make_fixnum
	  (window_body_height (decode_live_window (window),
			       window_body_unit_from_symbol (pixelwise)) as EMACS_INT));
});

DEFUN! ("window-old-body-pixel-width",
       Fwindow_old_body_pixel_width,
       Swindow_old_body_pixel_width, 0, 1, 0,
       doc: "Return old width of WINDOW's text area in pixels.\nWINDOW must be a live window and defaults to the selected one.\n\nThe return value is the pixel width of WINDOW's text area after the\nlast time window change functions found WINDOW live on its frame.  It\nis zero if WINDOW was created after that.  ",
       (window: Lisp_Object),
       {
  return (make_fixnum
	  ((*decode_live_window (window)).old_body_pixel_width as EMACS_INT));
});

DEFUN! ("window-old-body-pixel-height",
       Fwindow_old_body_pixel_height,
       Swindow_old_body_pixel_height, 0, 1, 0,
       doc: "Return old height of WINDOW's text area in pixels.\nWINDOW must be a live window and defaults to the selected one.\n\nThe return value is the pixel height of WINDOW's text area after the\nlast time window change functions found WINDOW live on its frame.  It\nis zero if WINDOW was created after that.  ",
       (window: Lisp_Object),
       {
  return (make_fixnum
	  ((*decode_live_window (window)).old_body_pixel_height as EMACS_INT));
});

DEFUN! ("window-mode-line-height", Fwindow_mode_line_height,
       Swindow_mode_line_height, 0, 1, 0,
       doc: "Return the height in pixels of WINDOW's mode-line.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum (WINDOW_MODE_LINE_HEIGHT! (decode_live_window (window)) as EMACS_INT);
});

DEFUN! ("window-header-line-height", Fwindow_header_line_height,
       Swindow_header_line_height, 0, 1, 0,
       doc: "Return the height in pixels of WINDOW's header-line.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum (WINDOW_HEADER_LINE_HEIGHT! (decode_live_window (window)) as EMACS_INT);
});

DEFUN! ("window-tab-line-height", Fwindow_tab_line_height,
       Swindow_tab_line_height, 0, 1, 0,
       doc: "Return the height in pixels of WINDOW's tab-line.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum (WINDOW_TAB_LINE_HEIGHT! (decode_live_window (window)) as EMACS_INT);
});

DEFUN! ("window-right-divider-width", Fwindow_right_divider_width,
       Swindow_right_divider_width, 0, 1, 0,
       doc: "Return the width in pixels of WINDOW's right divider.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum (WINDOW_RIGHT_DIVIDER_WIDTH! (decode_live_window (window)) as EMACS_INT);
});

DEFUN! ("window-bottom-divider-width", Fwindow_bottom_divider_width,
       Swindow_bottom_divider_width, 0, 1, 0,
       doc: "Return the width in pixels of WINDOW's bottom divider.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum (WINDOW_BOTTOM_DIVIDER_WIDTH! (decode_live_window (window)) as EMACS_INT);
});

DEFUN! ("window-scroll-bar-width", Fwindow_scroll_bar_width,
       Swindow_scroll_bar_width, 0, 1, 0,
       doc: "Return the width in pixels of WINDOW's vertical scrollbar.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum (WINDOW_SCROLL_BAR_AREA_WIDTH! (decode_live_window (window)) as EMACS_INT);
});

DEFUN! ("window-scroll-bar-height", Fwindow_scroll_bar_height,
       Swindow_scroll_bar_height, 0, 1, 0,
       doc: "Return the height in pixels of WINDOW's horizontal scrollbar.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum (WINDOW_SCROLL_BAR_AREA_HEIGHT! (decode_live_window (window)) as EMACS_INT);
});

DEFUN! ("window-hscroll", Fwindow_hscroll, Swindow_hscroll, 0, 1, 0,
       doc: "Return the number of columns by which WINDOW is scrolled from left margin.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return make_fixnum ((*decode_live_window (window)).hscroll);
});

/* Set W's horizontal scroll amount to HSCROLL clipped to a reasonable
   range, returning the new amount as a fixnum.  */
unsafe fn set_window_hscroll (w: *mut window, hscroll: EMACS_INT) -> Lisp_Object
{
  /* Horizontal scrolling has problems with large scroll amounts.
     It's too slow with long lines, and even with small lines the
     display can be messed up.  For now, though, impose only the limits
     required by the internal representation: horizontal scrolling must
     fit in fixnum (since it's visible to Elisp) and into ptrdiff_t
     (since it's stored in a ptrdiff_t).  */
  let hscroll_max: ptrdiff_t = min (MOST_POSITIVE_FIXNUM as isize, PTRDIFF_MAX);
  let new_hscroll: ptrdiff_t = clip_to_bounds (0, hscroll as isize, hscroll_max);

  /* Prevent redisplay shortcuts when changing the hscroll.  */
  if (*w).hscroll != new_hscroll
    {
      (*XBUFFER ((*w).contents)).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;
      wset_redisplay (w);
    }

  (*w).hscroll = new_hscroll;
  window_bits! (w, suspend_auto_hscroll, true);

  return make_fixnum (new_hscroll);
}

DEFUN! ("set-window-hscroll", Fset_window_hscroll, Sset_window_hscroll, 2, 2, 0,
       doc: "Set number of columns WINDOW is scrolled from left margin to NCOL.\nWINDOW must be a live window and defaults to the selected one.\nClip the number to a reasonable value if out of range.\nReturn the new number.  NCOL should be zero or positive.\n\nNote that if `auto-hscroll-mode' is non-nil, you cannot scroll the\nwindow so that the location of point moves off-window.  ",
       (window: Lisp_Object, ncol: Lisp_Object),
       {
  CHECK_FIXNUM (ncol);
  return set_window_hscroll (decode_live_window (window), XFIXNUM (ncol));
});

/* Test if the character at column X, row Y is within window W.
   If it is not, return ON_NOTHING;
   if it is on the window's vertical divider, return
      ON_RIGHT_DIVIDER;
   if it is on the window's horizontal divider, return
      ON_BOTTOM_DIVIDER;
   if it is in the window's text area, return ON_TEXT;
   if it is on the window's modeline, return ON_MODE_LINE;
   if it is on the border between the window and its right sibling,
      return ON_VERTICAL_BORDER;
   if it is on a scroll bar, return ON_SCROLL_BAR;
   if it is on the window's top line, return ON_TAB_LINE;
   if it is on the window's header line, return ON_HEADER_LINE;
   if it is in left or right fringe of the window,
      return ON_LEFT_FRINGE or ON_RIGHT_FRINGE;
   if it is in the marginal area to the left/right of the window,
      return ON_LEFT_MARGIN or ON_RIGHT_MARGIN.

   X and Y are frame relative pixel coordinates.  */

unsafe fn coordinates_in_window (w: *mut window, x: c_int, y: c_int) -> window_part
{
  let mut x = x;
  let mut y = y;
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let mut part: window_part;
  let ux: c_int = FRAME_COLUMN_WIDTH (f);
  let mut left_x: c_int = WINDOW_LEFT_EDGE_X! (w);
  let mut right_x: c_int = WINDOW_RIGHT_EDGE_X! (w);
  let top_y: c_int = WINDOW_TOP_EDGE_Y! (w);
  let bottom_y: c_int = WINDOW_BOTTOM_EDGE_Y! (w);
  /* The width of the area where the vertical line can be dragged.
     (Between mode lines for instance.  */
  let grabbable_width: c_int = ux;
  let lmargin_width: c_int;
  let rmargin_width: c_int;
  let mut text_left: c_int;
  let text_right: c_int;

  /* Outside any interesting row or column?  */
  if y < top_y || y >= bottom_y || x < left_x || x >= right_x
    { return ON_NOTHING; }

  /* On the horizontal window divider (which prevails the vertical
     divider)?  */
  if WINDOW_BOTTOM_DIVIDER_WIDTH! (w) > 0
      && y >= (bottom_y - WINDOW_BOTTOM_DIVIDER_WIDTH! (w))
      && y <= bottom_y
    { return ON_BOTTOM_DIVIDER; }
  /* On vertical window divider?  */
  else if !WINDOW_RIGHTMOST_P! (w)
	   && WINDOW_RIGHT_DIVIDER_WIDTH! (w) > 0
	   && x >= right_x - WINDOW_RIGHT_DIVIDER_WIDTH! (w)
	   && x <= right_x
    { return ON_RIGHT_DIVIDER; }
  /* On the horizontal scroll bar?  (Including the empty space at its
     right!)  */
  else if (WINDOW_HAS_HORIZONTAL_SCROLL_BAR (w)
	    && y >= (bottom_y
		     - WINDOW_SCROLL_BAR_AREA_HEIGHT! (w)
		     - CURRENT_MODE_LINE_HEIGHT! (w)
		     - WINDOW_BOTTOM_DIVIDER_WIDTH! (w))
	    && y <= (bottom_y
		     - CURRENT_MODE_LINE_HEIGHT! (w)
		     - WINDOW_BOTTOM_DIVIDER_WIDTH! (w)))
    { return ON_HORIZONTAL_SCROLL_BAR; }
  /* On the mode or header/tab line?   */
  else if ((window_wants_mode_line (w)
	    && y >= (bottom_y
		     - CURRENT_MODE_LINE_HEIGHT! (w)
		     - WINDOW_BOTTOM_DIVIDER_WIDTH! (w))
	    && y <= bottom_y - WINDOW_BOTTOM_DIVIDER_WIDTH! (w)
	    && { part = ON_MODE_LINE; true })
	   || (window_wants_tab_line (w)
	       && y < top_y + CURRENT_TAB_LINE_HEIGHT! (w)
	       && { part = ON_TAB_LINE; true })
	   || (window_wants_header_line (w)
	       && y < top_y + CURRENT_HEADER_LINE_HEIGHT! (w)
	       + (if window_wants_tab_line (w)
		  { CURRENT_TAB_LINE_HEIGHT! (w) }
		  else { 0 })
	       && { part = ON_HEADER_LINE; true }))
    {
      /* If it's under/over the scroll bar portion of the mode/header
	 line, say it's on the vertical line.  That's to be able to
	 resize windows horizontally in case we're using toolkit scroll
	 bars.  Note: If scrollbars are on the left, the window that
	 must be eventually resized is that on the left of WINDOW.  */
      if WINDOW_RIGHT_DIVIDER_WIDTH! (w) == 0
	  && ((WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT! (w)
	       && !WINDOW_LEFTMOST_P (w)
	       && eabs! (x - left_x) < grabbable_width)
	      || (!WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT! (w)
		  && !WINDOW_RIGHTMOST_P! (w)
		  && eabs! (x - right_x) < grabbable_width))
	{ return ON_VERTICAL_BORDER; }
      else
	{ return part; }
    }

  /* In what's below, we subtract 1 when computing right_x because we
     want the rightmost pixel, which is given by left_pixel+width-1.  */
  if window_bits! (w, pseudo_window_p)
    {
      left_x = 0;
      right_x = WINDOW_PIXEL_WIDTH (w) - 1;
    }
  else
    {
      left_x = WINDOW_BOX_LEFT_EDGE_X! (w);
      right_x = WINDOW_BOX_RIGHT_EDGE_X! (w) - 1;
    }

  /* Outside any interesting column?  */
  if x < left_x || x > right_x
    { return ON_VERTICAL_SCROLL_BAR; }

  lmargin_width = window_box_width (w, LEFT_MARGIN_AREA);
  rmargin_width = window_box_width (w, RIGHT_MARGIN_AREA);

  text_left = window_box_left (w, TEXT_AREA);
  text_right = text_left + window_box_width (w, TEXT_AREA);

  if FRAME_WINDOW_P (f)
    {
      if !window_bits! (w, pseudo_window_p)
	  && WINDOW_RIGHT_DIVIDER_WIDTH! (w) == 0
	  && !WINDOW_HAS_VERTICAL_SCROLL_BAR! (w)
	  && !WINDOW_RIGHTMOST_P! (w)
	  && (eabs! (x - right_x) < grabbable_width)
	{ return ON_VERTICAL_BORDER; }
    }
  /* Need to say "x > right_x" rather than >=, since on character
     terminals, the vertical line's x coordinate is right_x.  */
  else if !window_bits! (w, pseudo_window_p)
	   && WINDOW_RIGHT_DIVIDER_WIDTH! (w) == 0
	   && !WINDOW_RIGHTMOST_P! (w)
	   /* Why check ux if we are not the rightmost window?  Also
	      shouldn't a pseudo window always be rightmost?  */
	   && x > right_x - ux
    { return ON_VERTICAL_BORDER; }

  if x < text_left
    {
      if lmargin_width > 0
	  && (if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
	      { x >= left_x + WINDOW_LEFT_FRINGE_WIDTH! (w) }
	      else { x < left_x + lmargin_width })
	{ return ON_LEFT_MARGIN; }
      else
	{ return ON_LEFT_FRINGE; }
    }

  if x >= text_right
    {
      if rmargin_width > 0
	  && (if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
	      { x < right_x - WINDOW_RIGHT_FRINGE_WIDTH! (w) }
	      else { x >= right_x - rmargin_width })
	{ return ON_RIGHT_MARGIN; }
      else
	{ return ON_RIGHT_FRINGE; }
    }

  /* Everything special ruled out - must be on text area */
  return ON_TEXT;
}

/* Take X is the frame-relative pixel x-coordinate, and return the
   x-coordinate relative to part PART of window W. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_relative_x_coord (w: *mut window, part: window_part,
                                                  x: c_int) -> c_int
{
  let left_x: c_int = if window_bits! (w, pseudo_window_p)
    { 0 } else { WINDOW_BOX_LEFT_EDGE_X! (w) };

  match part
    {
    ON_TEXT =>
      { return x - window_box_left (w, TEXT_AREA); }

    ON_TAB_LINE | ON_HEADER_LINE | ON_MODE_LINE | ON_LEFT_FRINGE =>
      { return x - left_x; }

    ON_RIGHT_FRINGE =>
      { return x - left_x - WINDOW_LEFT_FRINGE_WIDTH! (w); }

    ON_LEFT_MARGIN =>
      { return (x - left_x
		- (if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
		   { WINDOW_LEFT_FRINGE_WIDTH! (w) } else { 0 })); }

    ON_RIGHT_MARGIN =>
      { return (x + 1
		- (if window_bits! (w, pseudo_window_p)
		   { WINDOW_PIXEL_WIDTH (w) }
		   else { WINDOW_BOX_RIGHT_EDGE_X! (w) })
		+ window_box_width (w, RIGHT_MARGIN_AREA)
		+ (if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
		   { WINDOW_RIGHT_FRINGE_WIDTH! (w) } else { 0 })); }

    ON_NOTHING | ON_VERTICAL_BORDER | ON_VERTICAL_SCROLL_BAR
    | ON_HORIZONTAL_SCROLL_BAR | ON_RIGHT_DIVIDER | ON_BOTTOM_DIVIDER =>
      { return 0; }

    _ =>
      { emacs_abort () }
    }
}

/* dispextern.h 2940 and 328-333: `IT_CHARPOS` and `CLIP_TEXT_POS_FROM_MARKER` (the mirror
   does not carry them; the dispnew.rs shape).  */
macro_rules! IT_CHARPOS { ($it:expr) => { CHARPOS! (($it).current.pos) }; }
/* C: dispextern.h 2941's `IT_BYTEPOS` and window.h 626-629's
   `#define WINDOW_BUFFER(W) (WINDOW_LEAF_P (W) ? (W)->contents : Qnil)` — the mirrors do not
   carry either (the dispnew.rs `IT_CHARPOS` shape).  */
macro_rules! IT_BYTEPOS { ($it:expr) => { BYTEPOS! (($it).current.pos) }; }
macro_rules! WINDOW_BUFFER {
    ($w:expr) => { if WINDOW_LEAF_P ($w) { (*$w).contents } else { Qnil () } };
}
macro_rules! CLIP_TEXT_POS_FROM_MARKER { ($pos:expr, $marker:expr) => { {
  CHARPOS! ($pos) = clip_to_bounds (BEGV! (), marker_position ($marker), ZV! ());
  BYTEPOS! ($pos) = clip_to_bounds (BEGV_BYTE! (), marker_byte_position ($marker), ZV_BYTE! ());
} }; }

unsafe extern "C" {
  /// C: dispextern.h 3556-3557: `extern void *bidi_shelve_cache (void);`
  /// `extern void bidi_unshelve_cache (void *, bool);`
  pub fn bidi_shelve_cache () -> *mut c_void;
  pub fn bidi_unshelve_cache (databuf: *mut c_void, really_free_p: bool);
  /// C: dispextern.h 3597: `void start_display (struct it *, struct window *, struct text_pos);`
  pub fn start_display (it: *mut it, w: *mut window, pos: text_pos);
  /// C: dispextern.h 3598: `void move_it_vertically (struct it *, int);`
  pub fn move_it_vertically (it: *mut it, dy: c_int);
  /// C: dispextern.h 3601: `void move_it_past_eol (struct it *);`
  pub fn move_it_past_eol (it: *mut it);
}


/* C: dispextern.h 2285-2315 `enum display_element_type` and 2590's `OVERLAY_STRING_CHUNK_SIZE`
   /2384's `IT_STACK_SIZE`.  dispextern.h is mirrored only to its batch-II prefix (P-66), so
   the enumeration (PORTING.md 1.3: C's enum ABI is `c_int`) and the two constants are
   declared here, the way indent.rs declares its own copies.  */
pub type display_element_type = c_int;
pub const IT_CHARACTER: display_element_type = 0;
pub const IT_COMPOSITION: display_element_type = 1;
pub const IT_GLYPHLESS: display_element_type = 2;
pub const IT_IMAGE: display_element_type = 3;
pub const IT_STRETCH: display_element_type = 4;
pub const IT_EOB: display_element_type = 5;
pub const IT_TRUNCATION: display_element_type = 6;
pub const IT_CONTINUATION: display_element_type = 7;
pub const IT_XWIDGET: display_element_type = 8;

pub const OVERLAY_STRING_CHUNK_SIZE: c_int = 16;
pub const IT_STACK_SIZE: c_int = 5;

/* C: dispextern.h 2430-2936 `struct it`, the display iterator.  Same mirror gap: this unit
   passes the record to xdisp.c's `start_display`/`move_it_vertically`/`move_it_past_eol`,
   which write every member, so the *size* matters as much as the named offsets.  The
   members this unit reads or writes are named; every other member is inside a measured pad
   (the `///` line above each pad says what it covers), and the whole layout is pinned by
   the `const _` assertions below.  clang against the frozen target (the `spikes/run.sh`
   command): sizeof 4968, align 8, and every offset asserted.  C's `struct it it1 = it` copies
   (window_scroll_pixel_based) are the port's `Clone`/`Copy` (a C struct is trivially
   copyable, PORTING.md 0.1-9).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct it
{
  /// The window in which we iterate over current_buffer (or a string).  (C 2434)
  pub window: Lisp_Object,              // offset 0
  pub w: *mut window,                   // 2435, offset 8
  /// The window's frame.  (C 2438)
  pub f: *mut frame,                    // offset 16
  /// C 2441-2457: `method` and the eight members up to `string_nchars` (offset 24 … 112).
  pub _pad_a: [u8; 88],
  /// C 2459-2471: the eight `bool_bf : 1` members (one byte at 112).
  pub _pad_b0: u8,
  /// C 2473-2506: `dp` … `start` (offset 113 … 336).
  pub _pad_b: [u8; 223],
  /// Current buffer or string position of the iterator, including
  /// position in overlay strings etc.  (C 2512)
  pub current: display_pos,             // offset 336
  /// C 2516-2522: `n_overlay_strings`, `overlay_strings_charpos`, `overlay_strings[16]`
  /// (offset 384 … 528).
  pub _pad_c: [u8; 144],
  /// For each overlay string, the overlay it came from.  (C 2526)
  pub string_overlays: [Lisp_Object; OVERLAY_STRING_CHUNK_SIZE as usize],  // offset 528
  /// If non-nil, a Lisp string being processed.  (C 2532)
  pub string: Lisp_Object,              // offset 656
  /// If non-nil, we are processing a string that came from a `display' property given by
  /// an overlay.  (C 2536)
  pub from_overlay: Lisp_Object,        // offset 664
  /// C 2541-2665: `stack[IT_STACK_SIZE]` (offset 672 … 2192).
  pub _pad_d: [u8; 1520],
  /// Stack pointer.  (C 2667)
  pub sp: c_int,                        // offset 2192
  /// C 2670-2695: the alignment hole and `selective` (offset 2196 … 2208).
  pub _pad_e: [u8; 12],
  /// What kind of element `current' describes (C 2697: `enum display_element_type what`).
  pub what: display_element_type,       // offset 2208
  /// Real face id of the element.  (C 2700)
  pub face_id: c_int,                   // offset 2212
  /// C 2702-2719: the thirteen `bool_bf : 1` members (offset 2216 … 2220).
  pub _pad_f: [u8; 4],
  /// C 2721-2727: `line_wrap` and `base_face_id` (offset 2220 … 2228).
  pub _pad_g: [u8; 8],
  /// If `what' == IT_CHARACTER, the character and the length in bytes of its multibyte
  /// sequence.  (C 2711)
  pub c: c_int,                         // offset 2228
  /// C 2731-2735: `len` and the alignment hole before `cmp_it` (offset 2232 … 2240).
  pub _pad_h: [u8; 8],
  /// If what == IT_COMPOSITION, the composition or glyph-string to display.  (C 2739)
  pub _pad_i: [u8; 88],
  /// C 2741-2745: `char_to_display` and `image_id` (offset 2328 … 2344).
  pub _pad_j: [u8; 8],
  /// If `what' == IT_IMAGE, the id of the image to display.  (C 2725)
  pub image_id: ptrdiff_t,              // offset 2336
  /// C 2344-2436: `xwidget`, `slice`, `space_width`, `voffset`, `tab_width`,
  /// `font_height`, `object`, `position`, `truncation_pixel_width`,
  /// `continuation_pixel_width` (offset 2344 … 2436).
  pub _pad_k: [u8; 92],
  /// First and last visible x-position in the display area.  (C 2811)
  pub first_visible_x: c_int,           // offset 2436
  pub last_visible_x: c_int,            // 2440
  /// Last visible y-position in the display area.  (C 2815)
  pub last_visible_y: c_int,            // 2444
  pub extra_line_spacing: c_int,        // 2818, offset 2448
  /// C 2821-2826: `extra_line_spacing_above` (offset 2452 … 2456).
  pub _pad_l: [u8; 4],
  pub max_extra_line_spacing: c_int,    // 2827, offset 2456
  /// C 2830-2833: `override_ascent`, `override_descent`, `override_boff`, `glyph_row`
  /// (offset 2460 … 2480).
  pub _pad_m: [u8; 20],
  /// The area of glyph_row to which glyphs are added.  (C 2935)
  pub area: glyph_row_area,             // offset 2480
  /// C 2837-2885: `nglyphs` and `pixel_width` (offset 2484 … 2492).
  pub _pad_n: [u8; 8],
  /// The following members are used to substitute a single face for each run of
  /// characters, so that the face of a run can be changed.  (C 2887)
  pub ascent: c_int,                    // offset 2492
  pub descent: c_int,                   // 2890, offset 2496
  pub max_ascent: c_int,                // 2498, offset 2500
  pub max_descent: c_int,               // 2500, offset 2504
  /// C 2903-2911: `phys_ascent` … `max_phys_descent` (offset 2508 … 2524).
  pub _pad_o: [u8; 16],
  /// Current x pixel position within the display line.  (C 2866)
  pub current_x: c_int,                 // offset 2524
  /// C 2877: `wrap_prefix_width` (offset 2528 … 2532).
  pub _pad_p: [u8; 4],
  /// Accumulated width of continuation lines.  (C 2882)
  pub continuation_lines_width: c_int,  // offset 2532
  /// C 2891: `eol_pos` (offset 2536 … 2552).
  pub _pad_q: [u8; 16],
  /// Current y-position.  (C 2895)
  pub current_y: c_int,                 // offset 2552
  /// C 2898: `first_vpos` (offset 2556 … 2560).
  pub _pad_r: [u8; 4],
  /// Current vertical matrix position, or line number.  (C 2902)
  pub vpos: c_int,                      // offset 2560
  /// Horizontal matrix position reached in move_it_in_display_line.  (C 2907)
  pub hpos: c_int,                      // 2564
  /// C 2570-2577: `lnum`, `lnum_bytepos` (offset 2568 … 2584).
  pub _pad_s: [u8; 16],
  /// The width, in columns and in pixels, needed for display of the line numbers.  (C 2915)
  pub lnum_width: c_int,                // offset 2584
  pub lnum_pixel_width: c_int,          // 2588
  /// C 2920-2930: `pt_lnum` … the four `unsigned` fringe bitfields (offset 2592 … 2616).
  pub _pad_t: [u8; 24],
  /// For iterating over bidirectional text.  (C 2933)
  pub bidi_it: [u8; 2328],              // offset 2616
  /// C 2934-2936: `paragraph_embedding`, `min_width_property`, `min_width_start`
  /// (offset 4944 … 4968).
  pub _pad_u: [u8; 24],
}

const _: () = core::assert! (std::mem::size_of::<it> () == 4968);
const _: () = core::assert! (std::mem::align_of::<it> () == 8);
const _: () = core::assert! (std::mem::offset_of! (it, window) == 0);
const _: () = core::assert! (std::mem::offset_of! (it, w) == 8);
const _: () = core::assert! (std::mem::offset_of! (it, f) == 16);
const _: () = core::assert! (std::mem::offset_of! (it, current) == 336);
const _: () = core::assert! (std::mem::offset_of! (it, string_overlays) == 528);
const _: () = core::assert! (std::mem::offset_of! (it, string) == 656);
const _: () = core::assert! (std::mem::offset_of! (it, from_overlay) == 664);
const _: () = core::assert! (std::mem::offset_of! (it, sp) == 2192);
const _: () = core::assert! (std::mem::offset_of! (it, what) == 2208);
const _: () = core::assert! (std::mem::offset_of! (it, face_id) == 2212);
const _: () = core::assert! (std::mem::offset_of! (it, c) == 2228);
const _: () = core::assert! (std::mem::offset_of! (it, image_id) == 2336);
const _: () = core::assert! (std::mem::offset_of! (it, first_visible_x) == 2436);
const _: () = core::assert! (std::mem::offset_of! (it, last_visible_x) == 2440);
const _: () = core::assert! (std::mem::offset_of! (it, last_visible_y) == 2444);
const _: () = core::assert! (std::mem::offset_of! (it, extra_line_spacing) == 2448);
const _: () = core::assert! (std::mem::offset_of! (it, max_extra_line_spacing) == 2456);
const _: () = core::assert! (std::mem::offset_of! (it, area) == 2480);
const _: () = core::assert! (std::mem::offset_of! (it, ascent) == 2492);
const _: () = core::assert! (std::mem::offset_of! (it, descent) == 2496);
const _: () = core::assert! (std::mem::offset_of! (it, max_ascent) == 2500);
const _: () = core::assert! (std::mem::offset_of! (it, max_descent) == 2504);
const _: () = core::assert! (std::mem::offset_of! (it, current_x) == 2524);
const _: () = core::assert! (std::mem::offset_of! (it, continuation_lines_width) == 2532);
const _: () = core::assert! (std::mem::offset_of! (it, current_y) == 2552);
const _: () = core::assert! (std::mem::offset_of! (it, vpos) == 2560);
const _: () = core::assert! (std::mem::offset_of! (it, hpos) == 2564);
const _: () = core::assert! (std::mem::offset_of! (it, lnum_width) == 2584);
const _: () = core::assert! (std::mem::offset_of! (it, lnum_pixel_width) == 2588);
const _: () = core::assert! (std::mem::offset_of! (it, bidi_it) == 2616);

DEFUN! ("coordinates-in-window-p", Fcoordinates_in_window_p,
       Scoordinates_in_window_p, 2, 2, 0,
       doc: " Return non-nil if COORDINATES are in WINDOW.\nWINDOW must be a live window and defaults to the selected one.\nCOORDINATES is a cons of the form (X . Y), X and Y being distances\nmeasured in characters from the upper-left corner of the frame.\n(0 . 0) denotes the character in the upper left corner of the\nframe.\nIf COORDINATES are in the text portion of WINDOW,\n   the coordinates relative to the window are returned.\nIf they are in the bottom divider of WINDOW, `bottom-divider' is returned.\nIf they are in the right divider of WINDOW, `right-divider' is returned.\nIf they are in the mode line of WINDOW, `mode-line' is returned.\nIf they are in the header line of WINDOW, `header-line' is returned.\nIf they are in the tab line of WINDOW, `tab-line' is returned.\nIf they are in the left fringe of WINDOW, `left-fringe' is returned.\nIf they are in the right fringe of WINDOW, `right-fringe' is returned.\nIf they are on the border between WINDOW and its right sibling,\n  `vertical-line' is returned.\nIf they are in the windows's left or right marginal areas, `left-margin'\n  or `right-margin' is returned.  ",
       (coordinates: Lisp_Object, window: Lisp_Object),
       {
  let w: *mut window;
  let f: *mut frame;
  let mut x: c_int;
  let mut y: c_int;
  let mut lx: Lisp_Object;
  let mut ly: Lisp_Object;

  w = decode_live_window (window);
  f = XFRAME ((*w).frame);
  CHECK_CONS (coordinates);
  lx = Fcar (coordinates);
  ly = Fcdr (coordinates);
  CHECK_NUMBER (lx);
  CHECK_NUMBER (ly);
  x = FRAME_PIXEL_X_FROM_CANON_X! (f, lx) + FRAME_INTERNAL_BORDER_WIDTH (f);
  y = FRAME_PIXEL_Y_FROM_CANON_Y! (f, ly) + FRAME_INTERNAL_BORDER_WIDTH (f);

  match coordinates_in_window (w, x, y)
    {
    ON_NOTHING =>
      { return Qnil (); }

    ON_TEXT =>
      {
      /* Convert X and Y to window relative pixel coordinates, and
	 return the canonical char units.  */
      x -= window_box_left (w, TEXT_AREA);
      y -= WINDOW_TOP_EDGE_Y! (w);
      return Fcons (FRAME_CANON_X_FROM_PIXEL_X! (f, x),
		    FRAME_CANON_Y_FROM_PIXEL_Y! (f, y));
      }

    ON_MODE_LINE =>
      { return Qmode_line (); }

    ON_VERTICAL_BORDER =>
      { return Qvertical_line (); }

    ON_HEADER_LINE =>
      { return Qheader_line (); }

    ON_TAB_LINE =>
      { return Qtab_line (); }

    ON_LEFT_FRINGE =>
      { return Qleft_fringe (); }

    ON_RIGHT_FRINGE =>
      { return Qright_fringe (); }

    ON_LEFT_MARGIN =>
      { return Qleft_margin (); }

    ON_RIGHT_MARGIN =>
      { return Qright_margin (); }

    ON_VERTICAL_SCROLL_BAR =>
      /* Historically we are supposed to return nil in this case.  */
      { return Qnil (); }

    ON_HORIZONTAL_SCROLL_BAR =>
      { return Qnil (); }

    ON_RIGHT_DIVIDER =>
      { return Qright_divider (); }

    ON_BOTTOM_DIVIDER =>
      { return Qbottom_divider (); }

    _ =>
      { emacs_abort () }
    }
});


/* Callback for foreach_window, used in window_from_coordinates.
   Check if window W contains coordinates specified by USER_DATA which
   is actually a pointer to a struct check_window_data CW.

   Check if window W contains coordinates *CW->x and *CW->y.  If it
   does, return W in *CW->window, as Lisp_Object, and return in
   *CW->part the part of the window under coordinates *X,*Y.  Return
   false from this function to stop iterating over windows.  */

#[repr(C)]
pub struct check_window_data
{
  pub window: *mut Lisp_Object,
  pub x: c_int,
  pub y: c_int,
  pub part: *mut window_part,
}

unsafe extern "C" fn check_window_containing (w: *mut window, user_data: *mut c_void) -> bool
{
  let cw: *mut check_window_data = user_data as *mut check_window_data;
  let found: window_part = coordinates_in_window (w, (*cw).x, (*cw).y);
  if found == ON_NOTHING
    { return true; }
  else
    {
      *(*cw).part = found;
      XSETWINDOW! (*(*cw).window, w);
      return false;
    }
}


/* Find the window containing frame-relative pixel position X/Y and
   return it as a Lisp_Object.

   If X, Y is on one of the window's special `window_part' elements,
   set *PART to the id of that element.

   If there is no window under X, Y return nil and leave *PART
   unmodified.  TOOL_BAR_P means detect tool-bar windows, and
   TAB_BAR_P means detect tab-bar windows.

   This function was previously implemented with a loop cycling over
   windows with Fnext_window, and starting with the frame's selected
   window.  It turned out that this doesn't work with an
   implementation of next_window using Vwindow_list, because
   FRAME_SELECTED_WINDOW (F) is not always contained in the window
   tree of F when this function is called asynchronously from
   note_mouse_highlight.  The original loop didn't terminate in this
   case.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_from_coordinates (f: *mut frame, x: c_int, y: c_int,
			 mut part: *mut window_part, menu_bar_p: bool,
			 tab_bar_p: bool, tool_bar_p: bool) -> Lisp_Object
{
  let mut window: Lisp_Object;
  let mut cw: check_window_data = std::mem::zeroed ();
  let mut dummy: window_part = 0;

  if part.is_null ()
    { part = &raw mut dummy; }

  window = Qnil ();
  cw.window = &raw mut window;
  cw.x = x;
  cw.y = y;
  cw.part = part;
  foreach_window (f, check_window_containing, &raw mut cw as *mut c_void);

  return window;
}

DEFUN! ("window-at", Fwindow_at, Swindow_at, 2, 3, 0,
       doc: "Return window containing coordinates X and Y on FRAME.\nFRAME must be a live frame and defaults to the selected one.\nX and Y are measured in units of canonical columns and rows.\nThe top left corner of the frame is considered to be column 0, row 0.\nTool-bar and tab-bar pseudo-windows are ignored by this function: if\nthe specified coordinates are in any of these two windows, this\nfunction returns nil.",
       (x: Lisp_Object, y: Lisp_Object, frame: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);

  CHECK_INTEGER (x);
  CHECK_INTEGER (y);

  if XFIXNUM (x) < 0 || XFIXNUM (x) > FRAME_PIXEL_WIDTH (f) as EMACS_INT
      || XFIXNUM (y) < 0 || XFIXNUM (y) > FRAME_PIXEL_HEIGHT (f) as EMACS_INT
    { return Qnil (); }

  CHECK_NUMBER (x);
  CHECK_NUMBER (y);

  return window_from_coordinates (f,
				  (FRAME_PIXEL_X_FROM_CANON_X! (f, x)
				   + FRAME_INTERNAL_BORDER_WIDTH (f)),
				  (FRAME_PIXEL_Y_FROM_CANON_Y! (f, y)
				   + FRAME_INTERNAL_BORDER_WIDTH (f)),
				  std::ptr::null_mut (), false, false, false);
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_point (w: *mut window) -> ptrdiff_t
{
  return if w == XWINDOW (selected_window)
          { BUF_PT (XBUFFER ((*w).contents)) }
          else { (*XMARKER ((*w).pointm)).charpos };
}

DEFUN! ("window-point", Fwindow_point, Swindow_point, 0, 1, 0,
       doc: "Return current value of point in WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nFor a nonselected window, this is the value point would have if that\nwindow were selected.\n\nNote that, when WINDOW is selected, the value returned is the same as\nthat returned by `point' for WINDOW's buffer.  It would be more strictly\ncorrect to return the top-level value of `point', outside of any\n`save-excursion' forms.  But that is hard to define.",
       (window: Lisp_Object),
       {
  return make_fixnum (window_point (decode_live_window (window)));
});

DEFUN! ("window-old-point", Fwindow_old_point, Swindow_old_point, 0, 1, 0,
       doc: "Return old value of point in WINDOW.\nWINDOW must be a live window and defaults to the selected one.",
       (window: Lisp_Object),
       {
  return Fmarker_position ((*decode_live_window (window)).old_pointm);
});

DEFUN! ("window-start", Fwindow_start, Swindow_start, 0, 1, 0,
       doc: "Return position at which display currently starts in WINDOW.\nWINDOW must be a live window and defaults to the selected one.\nThis is updated by redisplay or by calling `set-window-start'.",
       (window: Lisp_Object),
       {
  return Fmarker_position ((*decode_live_window (window)).start);
});

/* This is text temporarily removed from the doc string below.

This function returns nil if the position is not currently known.
That happens when redisplay is preempted and doesn't finish.
If in that case you want to compute where the end of the window would
have been if redisplay had finished, do this:
    (save-excursion
      (goto-char (window-start window))
      (vertical-motion (1- (window-height window)) window)
      (point))")  */

DEFUN! ("window-end", Fwindow_end, Swindow_end, 0, 2, 0,
       doc: "Return position at which display currently ends in WINDOW.\nThis is the position after the final character in WINDOW.\n\nWINDOW must be a live window and defaults to the selected one.  This\nis updated by redisplay, when it runs to completion.  Simply changing\nthe buffer text or setting `window-start' does not update this value.\n\nReturn nil if there is no recorded value.  (This can happen if the\nlast redisplay of WINDOW was preempted, and did not finish.)  If\nUPDATE is non-nil, compute the up-to-date position if it isn't already\nrecorded.",
       (window: Lisp_Object, update: Lisp_Object),
       {
  let mut value: Lisp_Object;
  let w: *mut window = decode_live_window (window);
  let buf: Lisp_Object;
  let b: *mut buffer;

  buf = (*w).contents;
  CHECK_BUFFER (buf);
  b = XBUFFER (buf);

  if !NILP (update)
      && (windows_or_buffers_changed != 0
	  || !window_bits! (w, window_end_valid)
	  || (*b).bits & BUFFER_CLIP_CHANGED != 0
	  || (*b).bits & BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P != 0
	  || window_outdated (w))
      /* Don't call display routines if we didn't yet create any real
	 frames, because the glyph matrices are not yet allocated in
	 that case.  This could happen in some code that runs in the
	 daemon during initialization (e.g., see bug#20565).  */
      && !(noninteractive || FRAME_INITIAL_P (WINDOW_XFRAME! (w)))
    {
      let mut startp: text_pos = std::mem::zeroed ();
      let mut it: it = std::mem::zeroed ();
      let mut old_buffer: *mut buffer = std::ptr::null_mut ();
      let mut itdata: *mut c_void = std::ptr::null_mut ();

      /* Cannot use Fvertical_motion because that function doesn't
	 cope with variable-height lines.  */
      if b != (current_buffer! ())
	{
	  old_buffer = (current_buffer! ());
	  set_buffer_internal (b);
	}

      /* In case W->start is out of the range, use something
         reasonable.  This situation occurred when loading a file with
         `-l' containing a call to `rmail' with subsequent other
         commands.  At the end, W->start happened to be BEG, while
         rmail had already narrowed the buffer.  */
      CLIP_TEXT_POS_FROM_MARKER! (startp, (*w).start);

      itdata = bidi_shelve_cache ();
      start_display (&raw mut it, w, startp);
      move_it_vertically (&raw mut it, window_box_height (w));
      if it.current_y < it.last_visible_y
	{ move_it_past_eol (&raw mut it); }
      value = make_fixnum (IT_CHARPOS! (it));
      bidi_unshelve_cache (itdata, false);

      if !old_buffer.is_null ()
	{ set_buffer_internal (old_buffer); }
    }
  else
    { XSETINT! (value, BUF_Z! (b) - (*w).window_end_pos); }

  return value;
});

DEFUN! ("set-window-point", Fset_window_point, Sset_window_point, 2, 2, 0,
       doc: "Make point value in WINDOW be at position POS in WINDOW's buffer.\nWINDOW must be a live window and defaults to the selected one.\nReturn POS.",
       (window: Lisp_Object, pos: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);
  let mut pos = pos;

  /* Type of POS is checked by Fgoto_char or set_marker_restricted ...  */

  if w == XWINDOW (selected_window)
    {
      if XBUFFER ((*w).contents) == (current_buffer! ())
	{ Fgoto_char (pos); }
      else
	{
	  let old_buffer: *mut buffer = (current_buffer! ());

	  /* ... but here we want to catch type error before buffer change.  */
	  CHECK_FIXNUM_COERCE_MARKER! (pos);
	  set_buffer_internal (XBUFFER ((*w).contents));
	  Fgoto_char (pos);
	  set_buffer_internal (old_buffer);
	}
    }
  else
    {
      set_marker_restricted ((*w).pointm, pos, (*w).contents);
      /* We have to make sure that redisplay updates the window to show
	 the new value of point.  */
      wset_redisplay (w);
    }

  return pos;
});

DEFUN! ("set-window-start", Fset_window_start, Sset_window_start, 2, 3, 0,
       doc: "Make display in WINDOW start at position POS in WINDOW's buffer.\nWINDOW must be a live window and defaults to the selected one.  Return\nPOS.\n\nOptional third arg NOFORCE non-nil prevents next redisplay from\nmoving point if displaying the window at POS makes point invisible;\nredisplay will then choose the WINDOW's start position by itself in\nthat case, i.e. it will disregard POS if adhering to it will make\npoint not visible in the window.\n\nFor reliable setting of WINDOW start position, make sure point is\nat a position that will be visible when that start is in effect,\notherwise there's a chance POS will be disregarded, e.g., if point\nwinds up in a partially-visible line.\n\nThe setting of the WINDOW's start position takes effect during the\nnext redisplay cycle, not immediately.  If NOFORCE is nil or\nomitted, forcing the display of WINDOW to start at POS cancels\nany setting of WINDOW's vertical scroll (\"vscroll\") amount\nset by `set-window-vscroll' and by scrolling functions.  ",
       (window: Lisp_Object, pos: Lisp_Object, noforce: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);

  set_marker_restricted ((*w).start, pos, (*w).contents);
  /* This is not right, but much easier than doing what is right.  */
  window_bits! (w, start_at_line_beg, false);
  if NILP (noforce)
    { window_bits! (w, force_start, true); }
  wset_update_mode_line (w);
  /* Bug#15957.  */
  window_bits! (w, window_end_valid, false);
  wset_redisplay (w);

  return pos;
});

DEFUN! ("pos-visible-in-window-p", Fpos_visible_in_window_p,
       Spos_visible_in_window_p, 0, 3, 0,
       doc: "Return non-nil if position POS is currently on the frame in WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nReturn nil if that position is scrolled vertically out of view.  If a\ncharacter is only partially visible, nil is returned, unless the\noptional argument PARTIALLY is non-nil.  If POS is only out of view\nbecause of horizontal scrolling, return non-nil.  If POS is t, it\nspecifies either the first position displayed on the last visible\nscreen line in WINDOW, or the end-of-buffer position, whichever comes\nfirst.  POS defaults to point in WINDOW; WINDOW defaults to the\nselected window.\n\nIf POS is visible, return t if PARTIALLY is nil; if PARTIALLY is non-nil,\nthe return value is a list of 2 or 6 elements (X Y [RTOP RBOT ROWH VPOS]),\nwhere X and Y are the pixel coordinates relative to the top left corner\nof the window.  The remaining elements are omitted if the character after\nPOS is fully visible; otherwise, RTOP and RBOT are the number of pixels\noff-window at the top and bottom of the screen line (\"row\") containing\nPOS, ROWH is the visible height of that row, and VPOS is the row number\n(zero-based).  ",
       (pos: Lisp_Object, window: Lisp_Object, partially: Lisp_Object),
       {
  let w: *mut window;
  let mut posint: EMACS_INT;
  let buf: *mut buffer;
  let mut top: text_pos = unsafe { std::mem::zeroed () };
  let mut in_window: Lisp_Object = Qnil ();
  let mut rtop: c_int = 0;
  let mut rbot: c_int = 0;
  let mut rowh: c_int = 0;
  let mut vpos: c_int = 0;
  let mut fully_p: bool = true;
  let mut x: c_int = 0;
  let mut y: c_int = 0;

  w = decode_live_window (window);
  buf = XBUFFER ((*w).contents);
  SET_TEXT_POS_FROM_MARKER! (top, (*w).start);

  if EQ (pos, Qt ())
    { posint = -1; }
  else if !NILP (pos)
    { posint = fix_position (pos); }
  else if w == XWINDOW (selected_window)
    { posint = PT! (); }
  else
    { posint = marker_position ((*w).pointm); }

  /* If position is above window start or outside buffer boundaries,
     or if window start is out of range, position is not visible.  */
  if (EQ (pos, Qt ())
       || (posint >= CHARPOS! (top) && posint <= BUF_ZV (buf)))
      && CHARPOS! (top) >= BUF_BEGV (buf)
      && CHARPOS! (top) <= BUF_ZV (buf)
      && pos_visible_p (w, posint, &mut x, &mut y, &mut rtop, &mut rbot, &mut rowh, &mut vpos)
    {
      fully_p = rtop == 0 && rbot == 0;
      if !NILP (partially) || fully_p
	{ in_window = Qt (); }
    }

  if !NILP (in_window) && !NILP (partially)
    {
      let mut part: Lisp_Object = Qnil ();
      if !fully_p
	{ part = list4i (rtop as EMACS_INT, rbot as EMACS_INT, rowh as EMACS_INT,
			 vpos as EMACS_INT); }
      in_window = Fcons (make_fixnum (x as EMACS_INT),
			 Fcons (make_fixnum (y as EMACS_INT), part));
    }

  return in_window;
});

DEFUN! ("window-line-height", Fwindow_line_height,
       Swindow_line_height, 0, 2, 0,
       doc: "Return height in pixels of text line LINE in window WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nReturn height of current line if LINE is omitted or nil.  Return height of\nheader or mode line if LINE is `header-line' or `mode-line'.\nOtherwise, LINE is a text line number starting from 0.  A negative number\ncounts from the end of the window.\n\nValue is a list (HEIGHT VPOS YPOS OFFBOT), where HEIGHT is the height\nin pixels of the visible part of the line, VPOS and YPOS are the\nvertical position in lines and pixels of the line, relative to the top\nof the first text line, and OFFBOT is the number of off-window pixels at\nthe bottom of the text line.  If there are off-window pixels at the top\nof the (first) text line, YPOS is negative.\n\nReturn nil if window display is not up-to-date.  In that case, use\n`pos-visible-in-window-p' to obtain the information.  ",
       (line: Lisp_Object, window: Lisp_Object),
       {
  let w: *mut window;
  let b: *mut buffer;
  let mut row: *mut glyph_row;
  let mut end_row: *mut glyph_row;
  let mut max_y: c_int;
  let crop: c_int;
  let mut i: c_int;
  let mut n: EMACS_INT;

  w = decode_live_window (window);

  if noninteractive || window_bits! (w, pseudo_window_p)
    { return Qnil (); }

  CHECK_BUFFER ((*w).contents);
  b = XBUFFER ((*w).contents);

  /* Fail if current matrix is not up-to-date.  */
  if !window_bits! (w, window_end_valid)
      || windows_or_buffers_changed != 0
      || (*b).bits & BUFFER_CLIP_CHANGED != 0
      || (*b).bits & BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P != 0
      || window_outdated (w)
    { return Qnil (); }

  'found_row: {
  if NILP (line)
    {
      i = (*w).cursor.vpos;
      if i < 0 || i >= (*((*w).current_matrix as *mut glyph_matrix)).nrows
	  || { row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, i); !glyph_row_bits! (enabled_p, row) }
	{ return Qnil (); }
      max_y = window_text_bottom_y (w);
      break 'found_row;
    }

  if EQ (line, Qtab_line ())
    {
      if !window_wants_tab_line (w)
	{ return Qnil (); }
      row = MATRIX_TAB_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix);
      return if glyph_row_bits! (enabled_p, row)
	{ list4i ((*row).height as EMACS_INT, 0, 0, 0) } else { Qnil () };
    }

  if EQ (line, Qheader_line ())
    {
      if !window_wants_header_line (w)
	{ return Qnil (); }
      row = MATRIX_HEADER_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix);
      return if glyph_row_bits! (enabled_p, row)
	{ list4i ((*row).height as EMACS_INT, 0, 0, 0) } else { Qnil () };
    }

  if EQ (line, Qmode_line ())
    {
      row = MATRIX_MODE_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix);
      return if glyph_row_bits! (enabled_p, row)
	{ list4i ((*row).height as EMACS_INT, 0,
		  (WINDOW_TAB_LINE_HEIGHT! (w)
		   + WINDOW_HEADER_LINE_HEIGHT! (w)
		   + window_text_bottom_y (w)) as EMACS_INT,
		  0) }
	else { Qnil () };
    }

  CHECK_FIXNUM (line);
  n = XFIXNUM (line);

  row = MATRIX_FIRST_TEXT_ROW! ((*w).current_matrix as *mut glyph_matrix);
  end_row = MATRIX_BOTTOM_TEXT_ROW! ((*w).current_matrix as *mut glyph_matrix, w);
  max_y = window_text_bottom_y (w);
  i = 0;

  while (n < 0 || (i as EMACS_INT) < n)
	 && row <= end_row && glyph_row_bits! (enabled_p, row)
	 && (*row).y + (*row).height < max_y
    { row = row.add (1); i += 1; }

  if row > end_row || !glyph_row_bits! (enabled_p, row)
    { return Qnil (); }

  if { n += 1; n } < 0
    {
      if -n > i as EMACS_INT
	{ return Qnil (); }
      row = row.offset (n as isize);
      i += n as c_int;
    }
  }

  crop = max (0, ((*row).y + (*row).height - max_y) as isize) as c_int;
  return list4i (((*row).height as EMACS_INT + min (0, (*row).y as isize)) as EMACS_INT - crop as EMACS_INT,
		 i as EMACS_INT, (*row).y as EMACS_INT, crop as EMACS_INT);
});

DEFUN! ("window-lines-pixel-dimensions", Fwindow_lines_pixel_dimensions, Swindow_lines_pixel_dimensions, 0, 6, 0,
       doc: "Return pixel dimensions of WINDOW's lines.\nThe return value is a list of the x- and y-coordinates of the lower\nright corner of the last character of each line.  Return nil if the\ncurrent glyph matrix of WINDOW is not up-to-date.\n\nOptional argument WINDOW specifies the window whose lines' dimensions\nshall be returned.  Nil or omitted means to return the dimensions for\nthe selected window.\n\nFIRST, if non-nil, specifies the index of the first line whose\ndimensions shall be returned.  If FIRST is nil and BODY is non-nil,\nstart with the first text line of WINDOW.  Otherwise, start with the\nfirst line of WINDOW.\n\nLAST, if non-nil, specifies the last line whose dimensions shall be\nreturned.  If LAST is nil and BODY is non-nil, the last line is the last\nline of the body (text area) of WINDOW.  Otherwise, last is the last\nline of WINDOW.\n\nINVERSE, if nil, means that the y-pixel value returned for a specific\nline specifies the distance in pixels from the left edge (body edge if\nBODY is non-nil) of WINDOW to the right edge of the last glyph of that\nline.  INVERSE non-nil means that the y-pixel value returned for a\nspecific line specifies the distance in pixels from the right edge of\nthe last glyph of that line to the right edge (body edge if BODY is\nnon-nil) of WINDOW.\n\nLEFT non-nil means to return the x- and y-coordinates of the lower left\ncorner of the leftmost character on each line.  This is the value that\nshould be used for buffers that mostly display text from right to left.\n\nIf LEFT is non-nil and INVERSE is nil, this means that the y-pixel value\nreturned for a specific line specifies the distance in pixels from the\nleft edge of the last (leftmost) glyph of that line to the right edge\n(body edge if BODY is non-nil) of WINDOW.  If LEFT and INVERSE are both\nnon-nil, the y-pixel value returned for a specific line specifies the\ndistance in pixels from the left edge (body edge if BODY is non-nil) of\nWINDOW to the left edge of the last (leftmost) glyph of that line.\n\nNormally, the value of this function is not available while Emacs is\nbusy, for example, when processing a command.  It should be retrievable\nthough when run from an idle timer with a delay of zero seconds.  ",
       (window: Lisp_Object, first: Lisp_Object, last: Lisp_Object, body: Lisp_Object,
        inverse: Lisp_Object, left: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);
  let b: *mut buffer;
  let mut row: *mut glyph_row;
  let mut end_row: *mut glyph_row;
  let mut max_y: c_int = if NILP (body) { WINDOW_PIXEL_HEIGHT (w) } else { window_text_bottom_y (w) };
  let mut rows: Lisp_Object = Qnil ();
  let window_width: c_int = if NILP (body)
    { (*w).pixel_width } else { window_body_width (w, window_body_unit::WINDOW_BODY_IN_PIXELS) };
  let tab_line_height: c_int = WINDOW_TAB_LINE_HEIGHT! (w);
  let header_line_height: c_int = WINDOW_HEADER_LINE_HEIGHT! (w);
  let subtract: c_int = if NILP (body) { 0 } else { (tab_line_height + header_line_height) };
  let invert: bool = !NILP (inverse);
  let left_flag: bool = !NILP (left);

  if noninteractive || window_bits! (w, pseudo_window_p)
    { return Qnil (); }

  CHECK_BUFFER ((*w).contents);
  b = XBUFFER ((*w).contents);

  /* Fail if current matrix is not up-to-date.  */
  if !window_bits! (w, window_end_valid)
      || windows_or_buffers_changed != 0
      || (*b).bits & BUFFER_CLIP_CHANGED != 0
      || (*b).bits & BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P != 0
      || window_outdated (w)
    { return Qnil (); }

  row = if !NILP (first)
	 { MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix,
			check_integer_range (first, 0,
					     (*((*w).current_matrix as *mut glyph_matrix)).nrows as intmax_t) as c_int) }
	 else if NILP (body)
	 { MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, 0) }
	 else { MATRIX_FIRST_TEXT_ROW! ((*w).current_matrix as *mut glyph_matrix) };
  end_row = if !NILP (last)
	     { MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix,
			    check_integer_range (last, 0,
						 (*((*w).current_matrix as *mut glyph_matrix)).nrows as intmax_t) as c_int) }
	     else if NILP (body)
	     { MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix,
			    (*((*w).current_matrix as *mut glyph_matrix)).nrows) }
	     else { MATRIX_BOTTOM_TEXT_ROW! ((*w).current_matrix as *mut glyph_matrix, w) };

  while row <= end_row && glyph_row_bits! (enabled_p, row)
	 && (*row).y + (*row).height < max_y
    {

      if left_flag
	{
	  let glyph: *mut glyph = *(*row).glyphs.as_ptr ().add (TEXT_AREA as usize);

	  rows = Fcons (Fcons (make_fixnum
			       ((if invert
				 { (*glyph).pixel_width as c_int as EMACS_INT }
				 else { (window_width - (*glyph).pixel_width as c_int) as EMACS_INT })),
			       make_fixnum (((*row).y + (*row).height - subtract) as EMACS_INT)),
			rows);
	}
      else
	{ rows = Fcons (Fcons (make_fixnum
			     ((if invert
			      { (window_width - (*row).pixel_width) as EMACS_INT }
			      else { (*row).pixel_width as EMACS_INT })),
			     make_fixnum (((*row).y + (*row).height - subtract) as EMACS_INT)),
		      rows); }
      row = row.add (1);
    }

  return Fnreverse (rows);
});

DEFUN! ("window-dedicated-p", Fwindow_dedicated_p, Swindow_dedicated_p,
       0, 1, 0,
       doc: "Return non-nil when WINDOW is dedicated to its buffer.\nMore precisely, return the value assigned by the last call of\n`set-window-dedicated-p' for WINDOW.  Return nil if that function was\nnever called with WINDOW as its argument, or the value set by that\nfunction was internally reset since its last call.  WINDOW must be a\nlive window and defaults to the selected one.\n\nWhen a window is dedicated to its buffer, `display-buffer' will refrain\nfrom displaying another buffer in it.  `get-lru-window' and\n`get-largest-window' treat dedicated windows specially.\n`delete-windows-on', `replace-buffer-in-windows', `quit-window' and\n`kill-buffer' can delete a dedicated window and the containing frame.\n\nFunctions like `set-window-buffer' may change the buffer displayed by a\nwindow, unless that window is \"strongly\" dedicated to its buffer, that\nis the value returned by `window-dedicated-p' is t.  ",
       (window: Lisp_Object),
       {
  return (*decode_live_window (window)).dedicated;
});

DEFUN! ("set-window-dedicated-p", Fset_window_dedicated_p,
       Sset_window_dedicated_p, 2, 2, 0,
       doc: "Mark WINDOW as dedicated according to FLAG.\nWINDOW must be a live window and defaults to the selected one.  FLAG\nnon-nil means mark WINDOW as dedicated to its buffer.  FLAG nil means\nmark WINDOW as non-dedicated.  Return FLAG.\n\nWhen a window is dedicated to its buffer, `display-buffer' will refrain\nfrom displaying another buffer in it.  `get-lru-window' and\n`get-largest-window' treat dedicated windows specially.\n`delete-windows-on', `replace-buffer-in-windows', `quit-window',\n`quit-restore-window' and `kill-buffer' can delete a dedicated window\nand the containing frame.\n\nAs a special case, if FLAG is t, mark WINDOW as \"strongly\" dedicated to\nits buffer.  Functions like `set-window-buffer' may change the buffer\ndisplayed by a window, unless that window is strongly dedicated to its\nbuffer.  If and when `set-window-buffer' displays another buffer in a\nwindow, it also makes sure that the window is no more dedicated.  ",
       (window: Lisp_Object, flag: Lisp_Object),
       {
  wset_dedicated (decode_live_window (window), flag);
  return flag;
});

DEFUN! ("window-prev-buffers", Fwindow_prev_buffers, Swindow_prev_buffers,
       0, 1, 0,
       doc: "Return buffers previously shown in WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nThe return value is a list of elements (BUFFER WINDOW-START POS),\nwhere BUFFER is a buffer, WINDOW-START is the start position of the\nwindow for that buffer, and POS is a window-specific point value.\n\nIn rare occasions BUFFER may have been already killed.  It's therefore\nadvisable to always check the return value for the occurrence of dead\nbuffers before using it.  ",
       (window: Lisp_Object),
       {
  return (*decode_live_window (window)).prev_buffers;
});

DEFUN! ("set-window-prev-buffers", Fset_window_prev_buffers,
       Sset_window_prev_buffers, 2, 2, 0,
       doc: "Set WINDOW's previous buffers to PREV-BUFFERS.\nWINDOW must be a live window and defaults to the selected one.\n\nPREV-BUFFERS should be a list of elements (BUFFER WINDOW-START POS),\nwhere BUFFER is a buffer, WINDOW-START is the start position of the\nwindow for that buffer, and POS is a window-specific point value.  ",
       (window: Lisp_Object, prev_buffers: Lisp_Object),
       {
  wset_prev_buffers (decode_live_window (window), prev_buffers);
  return prev_buffers;
});

DEFUN! ("window-next-buffers", Fwindow_next_buffers, Swindow_next_buffers,
       0, 1, 0,
       doc: "Return list of buffers recently re-shown in WINDOW.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return (*decode_live_window (window)).next_buffers;
});

DEFUN! ("set-window-next-buffers", Fset_window_next_buffers,
       Sset_window_next_buffers, 2, 2, 0,
       doc: "Set WINDOW's next buffers to NEXT-BUFFERS.\nWINDOW must be a live window and defaults to the selected one.\nNEXT-BUFFERS should be a list of buffers.  ",
       (window: Lisp_Object, next_buffers: Lisp_Object),
       {
  wset_next_buffers (decode_live_window (window), next_buffers);
  return next_buffers;
});

DEFUN! ("window-parameters", Fwindow_parameters, Swindow_parameters,
       0, 1, 0,
       doc: "Return the parameters of WINDOW and their values.\nWINDOW must be a valid window and defaults to the selected one.  The\nreturn value is a list of elements of the form (PARAMETER . VALUE).  ",
       (window: Lisp_Object),
       {
  return Fcopy_alist ((*decode_valid_window (window)).window_parameters);
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_parameter (w: *mut window, parameter: Lisp_Object) -> Lisp_Object
{
  let result: Lisp_Object = assq_no_quit (parameter, (*w).window_parameters);

  return CDR_SAFE (result);
}

DEFUN! ("window-parameter", Fwindow_parameter, Swindow_parameter,
       2, 2, 0,
       doc: "Return WINDOW's value for PARAMETER.\nWINDOW can be any window and defaults to the selected one.  ",
       (window: Lisp_Object, parameter: Lisp_Object),
       {
  let w: *mut window = decode_any_window (window);

  return window_parameter (w, parameter);
});

DEFUN! ("set-window-parameter", Fset_window_parameter,
       Sset_window_parameter, 3, 3, 0,
       doc: "Set WINDOW's value of PARAMETER to VALUE.\nWINDOW can be any window and defaults to the selected one.\nReturn VALUE.  ",
       (window: Lisp_Object, parameter: Lisp_Object, value: Lisp_Object),
       {
  let w: *mut window = decode_any_window (window);
  let old_alist_elt: Lisp_Object;
  let mut f: *mut frame;

  old_alist_elt = Fassq (parameter, (*w).window_parameters);

  /* If this window parameter has been used in a face remapping filter
     expression anywhere at any time and we changed its value, force a
     from-scratch redisplay to make sure that everything that depends on
     a face filtered on the window parameter value is up-to-date.

     We compare with Qt here instead of using !NILP so that users can
     set this property to a non-nil, non-t value to inhibit this
     mechanism for a specific window parameter.

     FIXME: instead of taking a sledgehammer to redisplay, we could be
     more precise in tracking which display bits depend on which
     remapped faces.  In particular, 1) if a window parameter named in a
     face filter affects only faces used in drawing fringes, we don't
     need to redraw TTY frames, but if the filter is ever used in a
     non-fringe context (e.g. the 'face' text property), we need to
     redraw TTY frames too.  2) In the fringe case, we should limit the
     redraw damage to the fringes of the affected window and not the
     whole frame containing the window.  Today, we seldom change window
     parameters named in face filters.  We should implement the
     optimizations above when this assumption no longer holds.  */
  if SYMBOLP (parameter)
      && WINDOW_LIVE_P (window)
      && EQ (Fget (parameter, QCfiltered ()), Qt ())
      && FRAME_WINDOW_P ({ f = WINDOW_XFRAME! (w); f })
      && !EQ (CDR_SAFE (old_alist_elt), value)
      && window_auto_redraw_on_parameter_change
    {
      face_change! (f, 1);
      fset_redisplay (f as *mut c_void);
    }

  if NILP (old_alist_elt)
    { wset_window_parameters
      (w, Fcons (Fcons (parameter, value), (*w).window_parameters)); }
  else
    { Fsetcdr (old_alist_elt, value); }
  return value;
});

DEFUN! ("window-display-table", Fwindow_display_table, Swindow_display_table,
       0, 1, 0,
       doc: "Return the display-table that WINDOW is using.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object),
       {
  return (*decode_live_window (window)).display_table;
});

/* Get the display table for use on window W.  This is either W's
   display table or W's buffer's display table.  Ignore the specified
   tables if they are not valid; if no valid table is specified,
   return 0.  */

/* `disptab.h`'s mirror declares `window_display_table` at its top level (disptab.h 67), so the
   definition lives in a module that is glob-imported and the declaration shadows it
   (PORTING.md 0.1-10/P-21; the `frame_globals` shape, from the function side).  */
pub mod window_defined_decls {
  use super::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_display_table (w: *mut window) -> *mut Lisp_Char_Table
{
  let mut dp: *mut Lisp_Char_Table = std::ptr::null_mut ();

  if DISP_TABLE_P ((*w).display_table)
    { dp = XCHAR_TABLE ((*w).display_table); }
  else if BUFFERP ((*w).contents)
    {
      let b: *mut buffer = XBUFFER ((*w).contents);

      if DISP_TABLE_P (BVAR! (b, display_table_))
	{ dp = XCHAR_TABLE (BVAR! (b, display_table_)); }
      else if DISP_TABLE_P (Vstandard_display_table)
	{ dp = XCHAR_TABLE (Vstandard_display_table); }
    }

  return dp;
}
}
use window_defined_decls::*;

DEFUN! ("set-window-display-table", Fset_window_display_table, Sset_window_display_table, 2, 2, 0,
       doc: "Set WINDOW's display-table to TABLE.\nWINDOW must be a live window and defaults to the selected one.  ",
       (window: Lisp_Object, table: Lisp_Object),
       {
  wset_display_table (decode_live_window (window), table);
  return table;
});

/* Record info on buffer window W is displaying
   when it is about to cease to display that buffer.  */
unsafe fn unshow_buffer (w: *mut window)
{
  let buf: Lisp_Object = (*w).contents;
  let b: *mut buffer = XBUFFER (buf);

  eassert! (b as *mut c_void == (*XMARKER ((*w).pointm)).buffer);

  /* C 2508-2513: `#if false` guards the conditional
     `if (w == XWINDOW (selected_window) || ! EQ (buf, XWINDOW (selected_window)->contents))`
     here; the port records the dead arm (P-08) and keeps the assignment unconditional, as the
     C comment says.  */
    /* last_window_start records the start position that this buffer
       had in the last window to be disconnected from it.
       Now that this statement is unconditional,
       it is possible for the buffer to be displayed in the
       selected window, while last_window_start reflects another
       window which was recently showing the same buffer.
       Some people might say that might be a good thing.  Let's see.  */
    (*b).last_window_start = marker_position ((*w).start);

  /* Point in the selected window's buffer
     is actually stored in that buffer, and the window's pointm isn't used.
     So don't clobber point in that buffer.  */
  if ! EQ (buf, (*XWINDOW (selected_window)).contents)
      /* Don't clobber point in current buffer either (this could be
	 useful in connection with bug#12208).
      && XBUFFER (buf) != current_buffer  */
      /* This line helps to fix Horsley's testbug.el bug.  */
      && !(WINDOWP (BVAR! (b, last_selected_window_))
	   && w != XWINDOW (BVAR! (b, last_selected_window_))
	   && EQ (buf, (*XWINDOW (BVAR! (b, last_selected_window_))).contents))
    { temp_set_point_both (b,
			 clip_to_bounds (BUF_BEGV (b),
					 marker_position ((*w).pointm),
					 BUF_ZV (b)),
			 clip_to_bounds (BUF_BEGV_BYTE (b),
					 marker_byte_position ((*w).pointm),
					 BUF_ZV_BYTE (b))); }

  if WINDOWP (BVAR! (b, last_selected_window_))
      && w == XWINDOW (BVAR! (b, last_selected_window_))
    { bset_last_selected_window (b, Qnil ()); }
}

/* Put NEW into the window structure in place of OLD.  SETFLAG false
   means change window structure only.  Otherwise store geometry and
   other settings as well.  */
unsafe fn replace_window (old: Lisp_Object, new: Lisp_Object, setflag: bool)
{
  let mut tem: Lisp_Object = Qnil ();
  let o: *mut window = XWINDOW (old);
  let n: *mut window = XWINDOW (new);

  /* If OLD is its frame's root window, then NEW is the new
     root window for that frame.  */
  if EQ (old, FRAME_ROOT_WINDOW (XFRAME ((*o).frame)))
    { fset_root_window (XFRAME ((*o).frame), new); }

  if setflag
    {
      (*n).pixel_left = (*o).pixel_left;
      (*n).pixel_top = (*o).pixel_top;
      (*n).pixel_width = (*o).pixel_width;
      (*n).pixel_height = (*o).pixel_height;
      (*n).left_col = (*o).left_col;
      (*n).top_line = (*o).top_line;
      (*n).total_cols = (*o).total_cols;
      (*n).total_lines = (*o).total_lines;
      wset_normal_cols (n, (*o).normal_cols);
      wset_normal_cols (o, make_float (1.0));
      wset_normal_lines (n, (*o).normal_lines);
      wset_normal_lines (o, make_float (1.0));
      (*n).current_matrix = std::ptr::null_mut ();
      (*n).desired_matrix = (*n).current_matrix;
      (*n).vscroll = 0;
      memset (&mut (*n).cursor as *mut cursor_pos as *mut c_void, 0,
	      std::mem::size_of::<cursor_pos> ());
      memset (&mut (*n).phys_cursor as *mut cursor_pos as *mut c_void, 0,
	      std::mem::size_of::<cursor_pos> ());
      (*n).last_cursor_vpos = 0;
      /* C 2580-2583: the `HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
      window_bits! (n, must_be_updated_p, false);
      window_bits! (n, pseudo_window_p, false);
      (*n).window_end_vpos = 0;
      (*n).window_end_pos = 0;
      window_bits! (n, window_end_valid, false);
    }

  tem = (*o).next;
  wset_next (n, tem);
  if !NILP (tem)
    { wset_prev (XWINDOW (tem), new); }

  tem = (*o).prev;
  wset_prev (n, tem);
  if !NILP (tem)
    { wset_next (XWINDOW (tem), new); }

  tem = (*o).parent;
  wset_parent (n, tem);
  if !NILP (tem) && EQ ((*XWINDOW (tem)).contents, old)
    { wset_combination (XWINDOW (tem), window_bits! (XWINDOW (tem), horizontal), new); }
}

/* If window WINDOW and its parent window are iso-combined, merge
   WINDOW's children into those of its parent window and mark WINDOW as
   deleted.  */

unsafe fn recombine_windows (window: Lisp_Object)
{
  let mut w: *mut window;
  let mut p: *mut window;
  let mut c: *mut window;
  let parent: Lisp_Object;
  let mut child: Lisp_Object;
  let horflag: bool;

  w = XWINDOW (window);
  parent = (*w).parent;
  if !NILP (parent) && NILP ((*w).combination_limit)
    {
      p = XWINDOW (parent);
      if WINDOWP ((*p).contents) && WINDOWP ((*w).contents)
	  && window_bits! (p, horizontal) == window_bits! (w, horizontal)
	/* WINDOW and PARENT are both either a vertical or a horizontal
	   combination.  */
	{
	  horflag = WINDOW_HORIZONTAL_COMBINATION_P (w);
	  child = (*w).contents;
	  c = XWINDOW (child);

	  /* Splice WINDOW's children into its parent's children and
	     assign new normal sizes.  */
	  if NILP ((*w).prev)
	    { wset_combination (p, horflag, child); }
	  else
	    {
	      wset_prev (c, (*w).prev);
	      wset_next (XWINDOW ((*w).prev), child);
	    }

	  while !c.is_null ()
	    {
	      wset_parent (c, parent);

	      if horflag
		{ wset_normal_cols
		  (c, make_float ((*c).pixel_width as f64
				  / (*p).pixel_width as f64)); }
	      else
		{ wset_normal_lines
		  (c, make_float ((*c).pixel_height as f64
				  / (*p).pixel_height as f64)); }

	      if NILP ((*c).next)
		{
		  if !NILP ((*w).next)
		    {
		      wset_next (c, (*w).next);
		      wset_prev (XWINDOW ((*c).next), child);
		    }

		  c = std::ptr::null_mut ();
		}
	      else
		{
		  child = (*c).next;
		  c = XWINDOW (child);
		}
	    }

	  /* WINDOW can be deleted now.  */
	  wset_combination (w, false, Qnil ());
	}
    }
}

/* If WINDOW can be deleted, delete it.  */
unsafe fn delete_deletable_window (window: Lisp_Object)
{
  if !NILP (calln! (Qwindow_deletable_p (), window))
    { calln! (Qdelete_window (), window); }
}

/***********************************************************************
			     Window List
 ***********************************************************************/

/* Add window W to *USER_DATA.  USER_DATA is actually a Lisp_Object
   pointer.  This is a callback function for foreach_window, used in
   the window_list function.  */

unsafe extern "C" fn add_window_to_list (w: *mut window, user_data: *mut c_void) -> bool
{
  let list: *mut Lisp_Object = user_data as *mut Lisp_Object;
  let mut window: Lisp_Object = Qnil ();
  XSETWINDOW! (window, w);
  *list = Fcons (window, *list);
  return true;
}

/* Return a list of all windows, for use by next_window.  If
   Vwindow_list is a list, return that list.  Otherwise, build a new
   list, cache it in Vwindow_list, and return that.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_list () -> Lisp_Object
{
  if !CONSP (Vwindow_list)
    {
      let mut tail: Lisp_Object;
      let mut frame: Lisp_Object;
      let count: specpdl_ref = SPECPDL_INDEX ();

      Vwindow_list = Qnil ();
      /*  Don't allow quitting in Fnconc.  Otherwise we might end up
	  with a too short Vwindow_list and Fkill_buffer not being able
	  to replace a buffer in all windows showing it (Bug#47244).  */
      specbind (Qinhibit_quit (), Qt ());
      FOR_EACH_FRAME! (tail, frame, {
	  let mut arglist: Lisp_Object = Qnil ();

	  /* We are visiting windows in canonical order, and add
	     new windows at the front of arglist, which means we
	     have to reverse this list at the end.  */
	  foreach_window (XFRAME (frame), add_window_to_list, &mut arglist as *mut Lisp_Object as *mut c_void);
	  arglist = Fnreverse (arglist);
	  Vwindow_list = nconc2 (Vwindow_list, arglist);
	});

      unbind_to (count, Qnil ());
    }

  return Vwindow_list;
}

/* Value is true if WINDOW satisfies the constraints given by
   OWINDOW, MINIBUF and ALL_FRAMES.

   MINIBUF	t means WINDOW may be minibuffer windows.
		`lambda' means WINDOW may not be a minibuffer window.
		a window means a specific minibuffer window

   ALL_FRAMES	t means search all frames,
		nil means search just current frame,
		`visible' means search just visible frames on the
                current terminal,
		0 means search visible and iconified frames on the
                current terminal,
		a window means search the frame that window belongs to,
		a frame means consider windows on that frame, only.  */

unsafe fn candidate_window_p (window: Lisp_Object, owindow: Lisp_Object,
		    minibuf: Lisp_Object, all_frames: Lisp_Object) -> bool
{
  let w: *mut window = XWINDOW (window);
  let f: *mut frame = XFRAME ((*w).frame);
  let mut candidate_p: bool = true;

  if !BUFFERP ((*w).contents)
    { candidate_p = false; }
  else if MINI_WINDOW_P (w)
           && (EQ (minibuf, Qlambda ())
	       || (WINDOW_LIVE_P (minibuf) && !EQ (minibuf, window)))
    {
      /* If MINIBUF is `lambda' don't consider any mini-windows.
         If it is a window, consider only that one.  */
      candidate_p = false;
    }
  else if EQ (all_frames, Qt ())
    { candidate_p = true; }
  else if NILP (all_frames)
    {
      eassert! (WINDOWP (owindow));
      candidate_p = EQ ((*w).frame, (*XWINDOW (owindow)).frame);
    }
  else if EQ (all_frames, Qvisible ())
    {
      candidate_p = FRAME_VISIBLE_P (f)
	&& (FRAME_TERMINAL! (XFRAME ((*w).frame))
	    == FRAME_TERMINAL! (XFRAME (selected_frame)));

    }
  else if FIXNUMP (all_frames) && XFIXNUM (all_frames) == 0
    {
      candidate_p = (FRAME_VISIBLE_P (f) || FRAME_ICONIFIED_P (f)
		     /* C 2792-2801: the `HAVE_X_WINDOWS` arm is dead (P-08).  */
		     )
	&& (FRAME_TERMINAL! (XFRAME ((*w).frame))
	    == FRAME_TERMINAL! (XFRAME (selected_frame)));
    }
  else if WINDOWP (all_frames)
    /* 	To qualify as candidate, it's not sufficient for WINDOW's frame
	to just share the minibuffer window - it must be active as well
	(see Bug#24500).  */
    { candidate_p = ((EQ ((*XWINDOW (all_frames)).frame, (*w).frame)
                    || (EQ ((*f).minibuffer_window, all_frames)
                        && EQ ((*XWINDOW (all_frames)).frame, FRAME_FOCUS_FRAME (f))))
                   && (EQ (minibuf, Qt ())
		       || !is_minibuffer (0, (*XWINDOW (all_frames)).contents))); }
  else if FRAMEP (all_frames)
    { candidate_p = EQ (all_frames, (*w).frame); }

  return candidate_p;
}

/* Decode arguments as allowed by Fnext_window, Fprevious_window, and
   Fwindow_list.  See candidate_window_p for the meaning of WINDOW,
   MINIBUF, and ALL_FRAMES.  */

unsafe fn decode_next_window_args (window: *mut Lisp_Object, minibuf: *mut Lisp_Object,
                          all_frames: *mut Lisp_Object)
{
  let w: *mut window = decode_live_window (*window);
  let miniwin: Lisp_Object = (*XFRAME ((*w).frame)).minibuffer_window;

  XSETWINDOW! (*window, w);
  /* MINIBUF nil may or may not include minibuffer windows.  Decide if
     it does.  But first make sure that this frame's minibuffer window
     is live (Bug#47207).  */
  if WINDOW_LIVE_P (miniwin) && NILP (*minibuf)
    { *minibuf = if this_minibuffer_depth ((*XWINDOW (miniwin)).contents) != 0
		{ miniwin } else { Qlambda () }; }
  else if !EQ (*minibuf, Qt ())
    { *minibuf = Qlambda (); }

  /* Now *MINIBUF can be t => count all minibuffer windows, `lambda'
     => count none of them, or a specific minibuffer window (the
     active one) to count.  */

  /* ALL_FRAMES nil doesn't specify which frames to include.  */
  if NILP (*all_frames)
    { *all_frames
      /* Once more make sure that this frame's minibuffer window is live
	 before including it (Bug#47207).  */
      = if WINDOW_LIVE_P (miniwin) && !EQ (*minibuf, Qlambda ())
	 { miniwin } else { Qnil () }; }
  else if EQ (*all_frames, Qvisible ())
    { }
  else if BASE_EQ (*all_frames, make_fixnum (0))
    { }
  else if FRAMEP (*all_frames)
    { }
  else if !EQ (*all_frames, Qt ())
    { *all_frames = Qnil (); }
}

/* Return the next or previous window of WINDOW in cyclic ordering
   of windows.  NEXT_P means return the next window.  See the
   documentation string of next-window for the meaning of MINIBUF and
   ALL_FRAMES.  */

unsafe fn next_window (window: Lisp_Object, minibuf: Lisp_Object, all_frames: Lisp_Object,
	     next_p: bool) -> Lisp_Object
{
  let mut window = window;
  let mut minibuf = minibuf;
  let mut all_frames = all_frames;
  let count: specpdl_ref = SPECPDL_INDEX ();

  decode_next_window_args (&mut window, &mut minibuf, &mut all_frames);

  /* If ALL_FRAMES is a frame, and WINDOW isn't on that frame, just
     return the first window on the frame.  */
  if FRAMEP (all_frames)
      && !EQ (all_frames, (*XWINDOW (window)).frame)
    { return Fframe_first_window (all_frames); }

  /*  Don't allow quitting in Fmemq.  */
  specbind (Qinhibit_quit (), Qt ());

  if next_p
    {
      let mut list: Lisp_Object;

      /* Find WINDOW in the list of all windows.  */
      list = Fmemq (window, window_list ());

      /* Scan forward from WINDOW to the end of the window list.  */
      if CONSP (list)
	{ list = XCDR (list);
	  while CONSP (list)
	    { if candidate_window_p (XCAR (list), window, minibuf, all_frames)
		{ break; }
	      list = XCDR (list); } }

      /* Scan from the start of the window list up to WINDOW.  */
      if !CONSP (list)
	{ list = Vwindow_list;
	  while CONSP (list) && !EQ (XCAR (list), window)
	    { if candidate_window_p (XCAR (list), window, minibuf, all_frames)
		{ break; }
	      list = XCDR (list); } }

      if CONSP (list)
	{ window = XCAR (list); }
    }
  else
    {
      let mut candidate: Lisp_Object;
      let mut list: Lisp_Object;

      /* Scan through the list of windows for candidates.  If there are
	 candidate windows in front of WINDOW, the last one of these
	 is the one we want.  If there are candidates following WINDOW
	 in the list, again the last one of these is the one we want.  */
      candidate = Qnil ();
      list = window_list ();
      while CONSP (list)
	{
	  if EQ (XCAR (list), window)
	    {
	      if WINDOWP (candidate)
		{ break; }
	    }
	  else if candidate_window_p (XCAR (list), window, minibuf,
				       all_frames)
	    { candidate = XCAR (list); }
	  list = XCDR (list);
	}

      if WINDOWP (candidate)
	{ window = candidate; }
    }

  unbind_to (count, Qnil ());

  return window;
}

DEFUN! ("next-window", Fnext_window, Snext_window, 0, 3, 0,
       doc: "Return live window after WINDOW in the cyclic ordering of windows.\nWINDOW must be a live window and defaults to the selected one.  The\noptional arguments MINIBUF and ALL-FRAMES specify the set of windows to\nconsider.\n\nMINIBUF nil or omitted means consider the minibuffer window only if the\nminibuffer is active.  MINIBUF t means consider the minibuffer window\neven if the minibuffer is not active.  Any other value means do not\nconsider the minibuffer window even if the minibuffer is active.\n\nALL-FRAMES nil or omitted means consider all windows on WINDOW's frame,\nplus the minibuffer window if specified by the MINIBUF argument.  If the\nminibuffer counts, consider all windows on all frames that share that\nminibuffer too.  The following non-nil values of ALL-FRAMES have special\nmeanings:\n\n- t means consider all windows on all existing frames.\n\n- `visible' means consider all windows on all visible frames.\n\n- 0 (the number zero) means consider all windows on all visible and\n  iconified frames.\n\n- A frame means consider all windows on that frame only.\n\nAnything else means consider all windows on WINDOW's frame and no\nothers.\n\nIf you use consistent values for MINIBUF and ALL-FRAMES, you can use\n`next-window' to iterate through the entire cycle of acceptable\nwindows, eventually ending up back at the window you started with.\n`previous-window' traverses the same cycle, in the reverse order.  ",
       (window: Lisp_Object, minibuf: Lisp_Object, all_frames: Lisp_Object),
       {
  return next_window (window, minibuf, all_frames, true);
});

DEFUN! ("previous-window", Fprevious_window, Sprevious_window, 0, 3, 0,
       doc: "Return live window before WINDOW in the cyclic ordering of windows.\nWINDOW must be a live window and defaults to the selected one.  The\noptional arguments MINIBUF and ALL-FRAMES specify the set of windows to\nconsider.\n\nMINIBUF nil or omitted means consider the minibuffer window only if the\nminibuffer is active.  MINIBUF t means consider the minibuffer window\neven if the minibuffer is not active.  Any other value means do not\nconsider the minibuffer window even if the minibuffer is active.\n\nALL-FRAMES nil or omitted means consider all windows on WINDOW's frame,\nplus the minibuffer window if specified by the MINIBUF argument.  If the\nminibuffer counts, consider all windows on all frames that share that\nminibuffer too.  The following non-nil values of ALL-FRAMES have special\nmeanings:\n\n- t means consider all windows on all existing frames.\n\n- `visible' means consider all windows on all visible frames.\n\n- 0 (the number zero) means consider all windows on all visible and\n  iconified frames.\n\n- A frame means consider all windows on that frame only.\n\nAnything else means consider all windows on WINDOW's frame and no\nothers.\n\nIf you use consistent values for MINIBUF and ALL-FRAMES, you can\nuse `previous-window' to iterate through the entire cycle of\nacceptable windows, eventually ending up back at the window you\nstarted with.  `next-window' traverses the same cycle, in the\nreverse order.  ",
       (window: Lisp_Object, minibuf: Lisp_Object, all_frames: Lisp_Object),
       {
  return next_window (window, minibuf, all_frames, false);
});

/* Return a list of windows in cyclic ordering.  Arguments are like
   for `next-window'.  */

unsafe fn window_list_1 (window: Lisp_Object, minibuf: Lisp_Object,
                         all_frames: Lisp_Object) -> Lisp_Object
{
  let mut window = window;
  let mut minibuf = minibuf;
  let mut all_frames = all_frames;
  let mut tail: Lisp_Object;
  let mut list: Lisp_Object;
  let mut rest: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();

  decode_next_window_args (&mut window, &mut minibuf, &mut all_frames);
  list = Qnil ();

  /*  Don't allow quitting in Fmemq and Fnconc.  */
  specbind (Qinhibit_quit (), Qt ());

  tail = window_list ();
  while CONSP (tail)
    { if candidate_window_p (XCAR (tail), window, minibuf, all_frames)
	{ list = Fcons (XCAR (tail), list); }
      tail = XCDR (tail); }

  /* Rotate the list to start with WINDOW.  */
  list = Fnreverse (list);
  rest = Fmemq (window, list);
  if !NILP (rest) && !EQ (rest, list)
    {
      tail = list;
      while !EQ (XCDR (tail), rest)
	{ tail = XCDR (tail); }
      XSETCDR (tail, Qnil ());
      list = nconc2 (rest, list);
    }

  unbind_to (count, Qnil ());

  return list;
}

DEFUN! ("window-list", Fwindow_list, Swindow_list, 0, 3, 0,
       doc: "Return a list of windows on FRAME, starting with WINDOW.\nFRAME nil or omitted means use the selected frame.\nWINDOW nil or omitted means use the window selected within FRAME.\nMINIBUF t means include the minibuffer window, even if it isn't active.\nMINIBUF nil or omitted means include the minibuffer window only\nif it's active.\nMINIBUF neither nil nor t means never include the minibuffer window.  ",
       (frame: Lisp_Object, minibuf: Lisp_Object, window: Lisp_Object),
       {
  let mut frame = frame;
  let mut window = window;
  if NILP (window)
    { window = if FRAMEP (frame) { (*XFRAME (frame)).selected_window } else { selected_window }; }
  CHECK_WINDOW (window);
  if NILP (frame)
    { frame = selected_frame; }

  if !EQ (frame, (*XWINDOW (window)).frame)
    { error (c"Window is on a different frame".as_ptr ()); }

  return window_list_1 (window, minibuf, frame);
});

DEFUN! ("window-list-1", Fwindow_list_1, Swindow_list_1, 0, 3, 0,
       doc: "Return a list of all live windows.\nWINDOW specifies the first window to list and defaults to the selected\nwindow.\n\nOptional argument MINIBUF nil or omitted means consider the minibuffer\nwindow only if the minibuffer is active.  MINIBUF t means consider the\nminibuffer window even if the minibuffer is not active.  Any other value\nmeans do not consider the minibuffer window even if the minibuffer is\nactive.\n\nOptional argument ALL-FRAMES nil or omitted means consider all windows\non WINDOW's frame, plus the minibuffer window if specified by the\nMINIBUF argument.  If the minibuffer counts, consider all windows on all\nframes that share that minibuffer too.  The following non-nil values of\nALL-FRAMES have special meanings:\n\n- t means consider all windows on all existing frames.\n\n- `visible' means consider all windows on all visible frames.\n\n- 0 (the number zero) means consider all windows on all visible and\n  iconified frames.\n\n- A frame means consider all windows on that frame only.\n\nAnything else means consider all windows on WINDOW's frame and no\nothers.\n\nIf WINDOW is not on the list of windows returned, some other window will\nbe listed first but no error is signaled.  ",
       (window: Lisp_Object, minibuf: Lisp_Object, all_frames: Lisp_Object),
       {
  return window_list_1 (window, minibuf, all_frames);
});

/** Return most recently selected frame that has the same root as a
    given frame.  It's defined here because the static window_list_1 is
    here too but in fact it's only needed in the frame code.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mru_rooted_frame (f: *mut frame) -> Lisp_Object
{
  let mut windows: Lisp_Object = window_list_1 (FRAME_SELECTED_WINDOW (f), Qnil (), Qt ());
  let r: *mut frame = root_frame (f);
  let mut b: *mut window = std::ptr::null_mut ();

  while CONSP (windows)
    {
      let w: *mut window = XWINDOW (XCAR (windows));
      let wf: *mut frame = WINDOW_XFRAME! (w);

      if wf != f && root_frame (wf) == r && FRAME_VISIBLE_P (wf)
	  && (b.is_null () || (*w).use_time > (*b).use_time)
	{ b = w; }
      windows = XCDR (windows);
    }

  if !b.is_null ()
    { return WINDOW_FRAME! (b); }
  else
    {
      let mut root: Lisp_Object = Qnil ();

      XSETFRAME! (root, r);

      return root;
    }
}

/* Look at all windows, performing an operation specified by TYPE
   with argument OBJ.
   If FRAMES is Qt, look at all frames;
                Qnil, look at just the selected frame;
		Qvisible, look at visible frames;
	        a frame, just look at windows on that frame.
   If MINI, perform the operation on minibuffer windows too.  */

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum window_loop
{
  WINDOW_LOOP_UNUSED,
  GET_BUFFER_WINDOW,		    /* Arg is buffer */
  REPLACE_BUFFER_IN_WINDOWS_SAFELY, /* Arg is buffer */
  REDISPLAY_BUFFER_WINDOWS,	    /* Arg is buffer */
  CHECK_ALL_WINDOWS                 /* Arg is ignored */
}

unsafe fn window_loop (r#type: window_loop, obj: Lisp_Object, mini: bool,
	     frames: Lisp_Object) -> Lisp_Object
{
  let mut window: Lisp_Object;
  let mut windows: Lisp_Object;
  let mut best_window: Lisp_Object;
  let mut frame_arg: Lisp_Object;
  let mut frame_best_window_flag: bool = false;
  let mut f: *mut frame;

  /* If we're only looping through windows on a particular frame,
     frame points to that frame.  If we're looping through windows
     on all frames, frame is 0.  */
  if FRAMEP (frames)
    { f = XFRAME (frames); }
  else if NILP (frames)
    { f = SELECTED_FRAME (); }
  else
    { f = std::ptr::null_mut (); }

  if !f.is_null ()
    { frame_arg = Qlambda (); }
  else if BASE_EQ (frames, make_fixnum (0))
    { frame_arg = frames; }
  else if EQ (frames, Qvisible ())
    { frame_arg = frames; }
  else
    { frame_arg = Qt (); }

  /* frame_arg is Qlambda to stick to one frame,
     Qvisible to consider all visible frames,
     or Qt otherwise.  */

  /* Pick a window to start with.  */
  if WINDOWP (obj)
    { window = obj; }
  else if !f.is_null ()
    { window = FRAME_SELECTED_WINDOW (f); }
  else
    { window = FRAME_SELECTED_WINDOW (SELECTED_FRAME ()); }

  windows = window_list_1 (window, if mini { Qt () } else { Qnil () }, frame_arg);
  best_window = Qnil ();

  while CONSP (windows)
    {
      let w: *mut window;

      window = XCAR (windows);
      w = XWINDOW (window);

      /* Note that we do not pay attention here to whether the frame
	 is visible, since Fwindow_list skips non-visible frames if
	 that is desired, under the control of frame_arg.  */
      if !MINI_WINDOW_P (w)
	  /* For REPLACE_BUFFER_IN_WINDOWS_SAFELY, we must always
	     consider all windows.  */
	  || r#type == window_loop::REPLACE_BUFFER_IN_WINDOWS_SAFELY
	  || (mini && minibuf_level > 0)
	{ match r#type
	  {
	  window_loop::GET_BUFFER_WINDOW =>
	    { if EQ ((*w).contents, obj)
		/* Don't find any minibuffer window except the one that
		   is currently in use.  */
		&& (!MINI_WINDOW_P (w) || EQ (window, minibuf_window))
	      {
		if EQ (window, selected_window)
		  /* Preferably return the selected window.  */
		  { return window; }
		else if EQ ((*XWINDOW (window)).frame, selected_frame)
			 && !frame_best_window_flag
		  /* Prefer windows on the current frame (but don't
		     choose another one if we have one already).  */
		  {
		    best_window = window;
		    frame_best_window_flag = true;
		  }
		else if NILP (best_window)
		  { best_window = window; }
	      } }

	  window_loop::REPLACE_BUFFER_IN_WINDOWS_SAFELY =>
	    /* We could simply check whether the buffer shown by window
	       is live, and show another buffer in case it isn't.  */
	    { if EQ ((*w).contents, obj)
	      {
		/* Undedicate WINDOW.  */
		wset_dedicated (w, Qnil ());
		/* Make WINDOW show the buffer returned by
		   other_buffer_safely, don't run any hooks.  */
		set_window_buffer
		  (window, other_buffer_safely ((*w).contents), false, false);
		/* If WINDOW is the selected window, make its buffer
		   current.  But do so only if the window shows the
		   current buffer (Bug#6454).  */
		if EQ (window, selected_window)
		    && XBUFFER ((*w).contents) == (current_buffer! ())
		  { Fset_buffer ((*w).contents); }
	      } }

	  window_loop::REDISPLAY_BUFFER_WINDOWS =>
	    { if EQ ((*w).contents, obj)
	      {
		mark_window_display_accurate (window, false);
		window_bits! (w, update_mode_line, true);
		(*XBUFFER (obj)).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;
		update_mode_lines = 27;
		best_window = window;
	      } }

	    /* Check for a leaf window that has a killed buffer
	       or broken markers.  */
	  window_loop::CHECK_ALL_WINDOWS =>
	    { if BUFFERP ((*w).contents)
	      {
		let b: *mut buffer = XBUFFER ((*w).contents);

		if !BUFFER_LIVE_P (b)
		  { emacs_abort (); }
		if !MARKERP ((*w).start) || (*XMARKER ((*w).start)).buffer != b as *mut c_void
		  { emacs_abort (); }
		if !MARKERP ((*w).pointm) || (*XMARKER ((*w).pointm)).buffer != b as *mut c_void
		  { emacs_abort (); }
	      } }

	  window_loop::WINDOW_LOOP_UNUSED =>
	    { }
	  } }
      windows = XCDR (windows);
    }

  return best_window;
}

/* Used for debugging.  Abort if any window has a dead buffer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_all_windows ()
{
  window_loop (window_loop::CHECK_ALL_WINDOWS, Qnil (), true, Qt ());
}

DEFUN! ("get-buffer-window", Fget_buffer_window, Sget_buffer_window, 0, 2, 0,
       doc: "Return a window currently displaying BUFFER-OR-NAME, or nil if none.\nBUFFER-OR-NAME may be a buffer or a buffer name and defaults to\nthe current buffer.\n\nThe optional argument ALL-FRAMES specifies the frames to consider:\n\n- t means consider all windows on all existing frames.\n\n- `visible' means consider all windows on all visible frames.\n\n- 0 (the number zero) means consider all windows on all visible\n    and iconified frames.\n\n- A frame means consider all windows on that frame only.\n\nAny other value of ALL-FRAMES means consider all windows on the\nselected frame and no others.  ",
       (buffer_or_name: Lisp_Object, all_frames: Lisp_Object),
       {
  let buffer: Lisp_Object;

  if NILP (buffer_or_name)
    { buffer = Fcurrent_buffer (); }
  else
    { buffer = Fget_buffer (buffer_or_name); }

  if BUFFERP (buffer)
    { return window_loop (window_loop::GET_BUFFER_WINDOW, buffer, true, all_frames); }
  else
    { return Qnil (); }
});

unsafe fn resize_root_window (window: Lisp_Object, delta: Lisp_Object,
		    horizontal: Lisp_Object, ignore: Lisp_Object,
		    pixelwise: Lisp_Object) -> Lisp_Object
{
  return calln! (Qwindow__resize_root_window (), window, delta,
		horizontal, ignore, pixelwise);
}

unsafe fn window_pixel_to_total (frame: Lisp_Object, horizontal: Lisp_Object) -> Lisp_Object
{
  return calln! (Qwindow__pixel_to_total (), frame, horizontal);
}

/** Remove all occurrences of element whose car is BUFFER from ALIST.
    Return changed ALIST.  */
unsafe fn window_discard_buffer_from_alist (buffer: Lisp_Object, alist: Lisp_Object)
                                            -> Lisp_Object
{
  let mut alist = alist;
  let mut tail: Lisp_Object;
  let mut prev: *mut Lisp_Object = &mut alist;

  tail = alist;
  while CONSP (tail)
    {
      let mut tem: Lisp_Object = XCAR (tail);

      tem = XCAR (tem);

      if EQ (tem, buffer)
	{ *prev = XCDR (tail); }
      else
	{ prev = xcdr_addr (tail); }
      tail = XCDR (tail);
    }

  return alist;
}

/** Remove all occurrences of BUFFER from LIST.  Return changed
    LIST.  */
unsafe fn window_discard_buffer_from_list (buffer: Lisp_Object, list: Lisp_Object)
                                           -> Lisp_Object
{
  let mut list = list;
  let mut tail: Lisp_Object;
  let mut prev: *mut Lisp_Object = &mut list;

  tail = list;
  while CONSP (tail)
    { if EQ (XCAR (tail), buffer)
	{ *prev = XCDR (tail); }
      else
	{ prev = xcdr_addr (tail); }
      tail = XCDR (tail); }

  return list;
}

/** Remove BUFFER from the lists of previous and next buffers of object
    WINDOW.  ALL true means remove any `quit-restore' and
    `quit-restore-prev' parameter of WINDOW referencing BUFFER too.  */
unsafe fn window_discard_buffer_from_window (buffer: Lisp_Object, window: Lisp_Object, all: bool)
{
  let w: *mut window = XWINDOW (window);

  wset_prev_buffers
    (w, window_discard_buffer_from_alist (buffer, (*w).prev_buffers));
  wset_next_buffers
    (w, window_discard_buffer_from_list (buffer, (*w).next_buffers));

  if all
    {
      let quit_restore: Lisp_Object = window_parameter (w, Qquit_restore ());
      let quit_restore_prev: Lisp_Object = window_parameter (w, Qquit_restore_prev ());
      let mut quint: Lisp_Object;

      if EQ (buffer, Fnth (make_fixnum (3), quit_restore_prev))
	  || (CONSP ({ quint = Fcar (Fcdr (quit_restore_prev)); quint })
	      && EQ (Fcar (quint), buffer))
	{ Fset_window_parameter (window, Qquit_restore_prev (), Qnil ()); }

      if EQ (buffer, Fnth (make_fixnum (3), quit_restore))
	  || (CONSP ({ quint = Fcar (Fcdr (quit_restore)); quint })
	      && EQ (Fcar (quint), buffer))
	{
	  Fset_window_parameter (window, Qquit_restore (),
				 window_parameter (w, Qquit_restore_prev ()));
	  Fset_window_parameter (window, Qquit_restore_prev (), Qnil ());
	}
    }
}

/** Remove BUFFER from the lists of previous and next buffers and the
    `quit-restore' and `quit-restore-prev' parameters of any dead
    WINDOW.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_discard_buffer_from_dead_windows (buffer: Lisp_Object)
{
  let h: *mut Lisp_Hash_Table = XHASH_TABLE (window_dead_windows_table);

  DOHASH! (h, k, v, {
    window_discard_buffer_from_window (buffer, v, true);
  });
}

DEFUN! ("window-discard-buffer-from-window", Fwindow_discard_buffer,
       Swindow_discard_buffer, 2, 3, 0,
       doc: "Discard BUFFER from WINDOW.\nDiscard specified live BUFFER from the lists of previous and next\nbuffers of specified live WINDOW.\n\nOptional argument ALL non-nil means discard any `quit-restore' and\n`quit-restore-prev' parameters of WINDOW referencing BUFFER too.  ",
       (buffer: Lisp_Object, window: Lisp_Object, all: Lisp_Object),
       {
  if !BUFFER_LIVE_P (XBUFFER (buffer))
    { error (c"Not a live buffer".as_ptr ()); }

  if !WINDOW_LIVE_P (window)
    { error (c"Not a live window".as_ptr ()); }

  window_discard_buffer_from_window (buffer, window, !NILP (all));

  return Qnil ();
});

DEFUN! ("delete-other-windows-internal", Fdelete_other_windows_internal,
       Sdelete_other_windows_internal, 0, 2, "",
       doc: "Make WINDOW fill its frame.\nOnly the frame WINDOW is on is affected.  WINDOW must be a valid window\nand defaults to the selected one.\n\nOptional argument ROOT, if non-nil, must specify an internal window such\nthat WINDOW is in its window subtree.  If this is the case, replace ROOT\nby WINDOW and leave alone any windows not part of ROOT's subtree.\n\nWhen WINDOW is live try to reduce display jumps by keeping the text\npreviously visible in WINDOW in the same place on the frame.  Doing this\ndepends on the value of (window-start WINDOW), so if calling this\nfunction in a program gives strange scrolling, make sure the\nwindow-start value is reasonable when this function is called.  ",
       (window: Lisp_Object, root: Lisp_Object),
       {
  let mut window = window;
  let mut root = root;
  let mut w: *mut window = decode_valid_window (window);
  let mut r: *mut window;
  let mut s: *mut window;
  let frame: Lisp_Object = (*w).frame;
  let f: *mut frame = XFRAME (frame);
  let mut sibling: Lisp_Object;
  let mut pwindow: Lisp_Object;
  let mut delta: Lisp_Object;
  /* C: `UNINIT` is `= {0,}` here (conf_post.h 369: `GCC_LINT` is 1, macros.tsv 1162, so the empty arm at 371 is dead), and the port writes the initializer out (PORTING.md 1.5).  */
  let mut swindow: Lisp_Object = Qnil ();
  let mut startpos: ptrdiff_t = 0;
  let mut startbyte: ptrdiff_t = 0;
  let mut top: c_int = 0;
  let new_top: c_int;
  let mut resize_failed: bool = false;

  XSETWINDOW! (window, w);

  if NILP (root)
    /* ROOT is the frame's root window.  */
    {
      root = FRAME_ROOT_WINDOW (f);
      r = XWINDOW (root);
    }
  else
    /* ROOT must be an ancestor of WINDOW.  */
    {
      r = decode_valid_window (root);
      pwindow = (*XWINDOW (window)).parent;
      while !NILP (pwindow)
	{ if EQ (pwindow, root)
	  { break; }
	else
	  { pwindow = (*XWINDOW (pwindow)).parent; } }
      if !EQ (pwindow, root)
	{ error (c"Specified root is not an ancestor of specified window".as_ptr ()); }
    }

  if EQ (window, root)
    /* A noop.  */
    { return Qnil (); }
  /* I don't understand the "top > 0" part below.  If we deal with a
     standalone minibuffer it would have been caught by the preceding
     test.  */
  else if MINI_WINDOW_P (w) /* && top > 0) */
    { error (c"Can't expand minibuffer to full frame".as_ptr ()); }

  if BUFFERP ((*w).contents)
    {
      startpos = marker_position ((*w).start);
      startbyte = marker_byte_position ((*w).start);
      top = (WINDOW_TOP_EDGE_LINE (w)
	     - FRAME_TOP_MARGIN (XFRAME (WINDOW_FRAME! (w))));
      /* Make sure WINDOW is the frame's selected window.  */
      if !EQ (window, FRAME_SELECTED_WINDOW (f))
	{
	  if EQ (selected_frame, frame)
	    { Fselect_window (window, Qnil ()); }
	  else
	    /* Do not clear f->select_mini_window_flag here.  If the
	       last selected window on F was an active minibuffer, we
	       want to return to it on a later Fselect_frame.  */
	    { fset_selected_window (f, window); }
	}
    }
  else
    {
      /* See if the frame's selected window is a part of the window
	 subtree rooted at WINDOW, by finding all the selected window's
	 parents and comparing each one with WINDOW.  If it isn't we
	 need a new selected window for this frame.  */
      swindow = FRAME_SELECTED_WINDOW (f);
      loop
	{
	  pwindow = swindow;
	  while !NILP (pwindow) && !EQ (window, pwindow)
	    { pwindow = (*XWINDOW (pwindow)).parent; }

	  if EQ (window, pwindow)
	    /* If WINDOW is an ancestor of SWINDOW, then SWINDOW is ok
	       as the new selected window.  */
	    { break; }
	  else
	    /* Else try the previous window of SWINDOW.  */
	    { swindow = Fprevious_window (swindow, Qlambda (), Qnil ()); }
	}

      if !EQ (swindow, FRAME_SELECTED_WINDOW (f))
	{
	  if EQ (selected_frame, frame)
	    { Fselect_window (swindow, Qnil ()); }
	  else
	    { fset_selected_window (f, swindow); }
	}
    }

  block_input ();
  if !FRAME_INITIAL_P (f)
    {
      let hlinfo: *mut Mouse_HLInfo = MOUSE_HL_INFO! (f);

      /* We are going to free the glyph matrices of WINDOW, and with
	 that we might lose any information about glyph rows that have
	 some of their glyphs highlighted in mouse face.  (These rows
	 are marked with a mouse_face_p flag.)  If WINDOW
	 indeed has some glyphs highlighted in mouse face, signal to
	 frame's up-to-date hook that mouse highlight was overwritten,
	 so that it will arrange for redisplaying the highlight.  */
      if EQ ((*hlinfo).mouse_face_window, window)
	{ reset_mouse_highlight (hlinfo); }
    }
  free_window_matrices (r);

  fset_redisplay (f as *mut c_void);
  Vwindow_list = Qnil ();

  if !WINDOW_LEAF_P (w)
    {
      /* Resize child windows vertically.  */
      XSETINT! (delta, ((*r).pixel_height - (*w).pixel_height) as EMACS_INT);
      (*w).pixel_top = (*r).pixel_top;
      (*w).top_line = (*r).top_line;
      resize_root_window (window, delta, Qnil (), Qnil (), Qt ());
      if window_resize_check (w, false)
	{ window_resize_apply (w, false); }
      else
	{
	  resize_root_window (window, delta, Qnil (), Qt (), Qt ());
	  if window_resize_check (w, false)
	    { window_resize_apply (w, false); }
	  else
	    { resize_failed = true; }
	}

      /* Resize child windows horizontally.  */
      if !resize_failed
	{
	  (*w).left_col = (*r).left_col;
	  (*w).pixel_left = (*r).pixel_left;
	  XSETINT! (delta, ((*r).pixel_width - (*w).pixel_width) as EMACS_INT);
	  resize_root_window (window, delta, Qt (), Qnil (), Qt ());
	  if window_resize_check (w, true)
	    { window_resize_apply (w, true); }
	  else
	    {
	      resize_root_window (window, delta, Qt (), Qt (), Qt ());
	      if window_resize_check (w, true)
		{ window_resize_apply (w, true); }
	      else
		{ resize_failed = true; }
	    }
	}

      if resize_failed
	/* Play safe, if we still can ...  */
	{
	  window = swindow;
	  w = XWINDOW (window);
	}
    }

  /* Cleanly unlink WINDOW from window-tree.  */
  if !NILP ((*w).prev)
    /* Get SIBLING above (on the left of) WINDOW.  */
    {
      sibling = (*w).prev;
      s = XWINDOW (sibling);
      wset_next (s, (*w).next);
      if !NILP ((*s).next)
	{ wset_prev (XWINDOW ((*s).next), sibling); }
    }
  else
    /* Get SIBLING below (on the right of) WINDOW.  */
    {
      sibling = (*w).next;
      s = XWINDOW (sibling);
      wset_prev (s, Qnil ());
      wset_combination (XWINDOW ((*w).parent),
			window_bits! (XWINDOW ((*w).parent), horizontal), sibling);
    }

  /* Delete ROOT and all child windows of ROOT.  */
  if WINDOWP ((*r).contents)
    {
      delete_all_child_windows ((*r).contents);
      wset_combination (r, false, Qnil ());
    }

  replace_window (root, window, true);
  /* Assign new total sizes to all windows on FRAME.  We can't do that
     _before_ WINDOW replaces ROOT since 'window--pixel-to-total' works
     on the whole frame and thus would work on the frame's old window
     configuration (Bug#51007).  */
  window_pixel_to_total (frame, Qnil ());
  window_pixel_to_total (frame, Qt ());

  /* This must become SWINDOW anyway .......  */
  if BUFFERP ((*w).contents) && !resize_failed
    {
      /* Try to minimize scrolling, by setting the window start to the
	 point will cause the text at the old window start to be at the
	 same place on the frame.  But don't try to do this if the
	 window start is outside the visible portion (as might happen
	 when the display is not current, due to typeahead).  */
      new_top = WINDOW_TOP_EDGE_LINE (w) - FRAME_TOP_MARGIN (XFRAME (WINDOW_FRAME! (w)));
      if new_top != top
	  && startpos >= BUF_BEGV (XBUFFER ((*w).contents))
	  && startpos <= BUF_ZV (XBUFFER ((*w).contents))
	{
	  let pos: position;
	  let obuf: *mut buffer = (current_buffer! ());

	  Fset_buffer ((*w).contents);
	  /* This computation used to temporarily move point, but that
	     can have unwanted side effects due to text properties.  */
	  pos = *vmotion (startpos, startbyte, (-top) as EMACS_INT, w as *mut c_void);

	  set_marker_both ((*w).start, (*w).contents, pos.bufpos, pos.bytepos);
	  window_bits! (w, window_end_valid, false);
	  window_bits! (w, start_at_line_beg,
			pos.bytepos == BEGV_BYTE! ()
			  || FETCH_BYTE! (pos.bytepos - 1) as c_int == b'\n' as c_int);
	  /* We need to do this, so that the window-scroll-functions
	     get called.  */
	  window_bits! (w, optional_new_start, true);

	  /* Reset the vscroll, as redisplay will not.  */
	  (*w).vscroll = 0;
	  window_bits! (w, preserve_vscroll_p, false);

	  set_buffer_internal (obuf);
	}
    }

  adjust_frame_glyphs (f);
  unblock_input ();

  FRAME_WINDOW_CHANGE! (f, true);

  return Qnil ();
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn replace_buffer_in_windows (buffer: Lisp_Object)
{
  /* When kill-buffer is called early during loadup, this function is
     undefined.  */
  if !NILP (Ffboundp (Qreplace_buffer_in_windows ()))
    { calln! (Qreplace_buffer_in_windows (), buffer); }
}

/** If BUFFER is shown in any window, safely replace it with some other
    buffer in all windows of all frames, even those on other keyboards.
    Do not delete any window.

    This function is called by Fkill_buffer when it detects that
    replacing BUFFER in some window showing BUFFER has failed.  It
    assumes that ‘replace-buffer-in-windows’ has removed any entry
    referencing BUFFER from any window's lists of previous and next
    buffers and that window's ‘quit-restore’ and 'quit-restore-prev'
    parameters.
*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn replace_buffer_in_windows_safely (buffer: Lisp_Object)
{
  if buffer_window_count (XBUFFER (buffer)) != 0
    {
      let mut tail: Lisp_Object = Qnil ();
      let mut frame: Lisp_Object = Qnil ();

      /* A single call to window_loop won't do the job because it only
	 considers frames on the current keyboard.  So loop manually over
	 frames, and handle each one.  */
      FOR_EACH_FRAME! (tail, frame, {
	window_loop (window_loop::REPLACE_BUFFER_IN_WINDOWS_SAFELY, buffer, true, frame);
      });
    }
}

/* The following three routines are needed for running a window's
   configuration change hook.  */
unsafe fn run_funs (funs: Lisp_Object)
{
  let mut funs = funs;
  while CONSP (funs)
    { if !EQ (XCAR (funs), Qt ())
      { call0 (XCAR (funs)); }
      funs = XCDR (funs); }
}

unsafe extern "C" fn select_window_norecord (window: Lisp_Object)
{
  if WINDOW_LIVE_P (window)
    { Fselect_window (window, Qt ()); }
}

unsafe extern "C" fn select_frame_norecord (frame: Lisp_Object)
{
  if FRAME_LIVE_P (XFRAME (frame))
    { Fselect_frame (frame, Qt ()); }
}

/**
 * run_window_configuration_change_hook:
 *
 * Run any functions on 'window-configuration-change-hook' for the
 * frame specified by F.  The buffer-local values are run with the
 * window showing the buffer selected.  The default value is run with
 * the frame specified by F selected.  All functions are called with
 * the selected window's buffer current.
 */
unsafe fn run_window_configuration_change_hook (f: *mut frame)
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut frame: Lisp_Object = Qnil ();
  let global_wcch: Lisp_Object
    = Fdefault_value (Qwindow_configuration_change_hook ());
  XSETFRAME! (frame, f);

  if NILP (Vrun_hooks)
      || !can_set_window_size! (f)
      || !after_make_frame! (f)
    { return; }

  /* Use the right buffer.  Matters when running the local hooks.  */
  if (current_buffer! ()) != XBUFFER (Fwindow_buffer (Qnil ()))
    {
      record_unwind_current_buffer ();
      Fset_buffer (Fwindow_buffer (Qnil ()));
    }

  if SELECTED_FRAME () != f
    {
      record_unwind_protect (Some (select_frame_norecord), selected_frame);
      select_frame_norecord (frame);
    }

  /* Look for buffer-local values.  */
  {
    let mut windows: Lisp_Object = Fwindow_list (frame, Qlambda (), Qnil ());
    while CONSP (windows)
      {
	let window: Lisp_Object = XCAR (windows);
	let buffer: Lisp_Object = Fwindow_buffer (window);
	if !NILP (Flocal_variable_p (Qwindow_configuration_change_hook (),
				      buffer))
	  {
	    let inner_count: specpdl_ref = SPECPDL_INDEX ();
	    record_unwind_protect (Some (select_window_norecord), selected_window);
	    select_window_norecord (window);
	    run_funs (Fbuffer_local_value (Qwindow_configuration_change_hook (),
					   buffer));
	    unbind_to (inner_count, Qnil ());
	  }
	windows = XCDR (windows);
      }
  }

  run_funs (global_wcch);
  unbind_to (count, Qnil ());
}

DEFUN! ("run-window-configuration-change-hook", Frun_window_configuration_change_hook,
       Srun_window_configuration_change_hook, 0, 1, 0,
       doc: "Run `window-configuration-change-hook' for FRAME.\nIf FRAME is omitted or nil, it defaults to the selected frame.\n\nThis function should not be needed any more and will be therefore\nconsidered obsolete.  ",
       (frame: Lisp_Object),
       {
  run_window_configuration_change_hook (decode_live_frame (frame));
  return Qnil ();
});

DEFUN! ("run-window-scroll-functions", Frun_window_scroll_functions,
       Srun_window_scroll_functions, 0, 1, 0,
       doc: "Run `window-scroll-functions' for WINDOW.\nIf WINDOW is omitted or nil, it defaults to the selected window.\n\nThis function is called by `split-window' for the new window, after it\nhas established the size of the new window.  ",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);
  let count: specpdl_ref = SPECPDL_INDEX ();

  record_unwind_current_buffer ();
  Fset_buffer ((*w).contents);
  if !NILP (Vwindow_scroll_functions)
    { run_hook_with_args_2 (Qwindow_scroll_functions (), window,
			  Fmarker_position ((*w).start)); }
  unbind_to (count, Qnil ());

  return Qnil ();
});

/**
 * window_sub_list:
 *
 * Return list of live windows constructed by traversing any window
 * sub-tree rooted at WINDOW in preorder followed by right siblings of
 * WINDOW.  Called from outside with second argument WINDOWS nil.  The
 * returned list is in reverse order.
 */
unsafe fn window_sub_list (window: Lisp_Object, windows: Lisp_Object) -> Lisp_Object
{
  let mut window = window;
  let mut windows = windows;

  let mut w: *mut window = XWINDOW (window);

  while !w.is_null ()
    {
      if WINDOW_INTERNAL_P (w)
	{ windows = window_sub_list ((*w).contents, windows); }
      else
	{ windows = Fcons (window, windows); }

      window = (*w).next;
      w = if NILP (window) { std::ptr::null_mut () } else { XWINDOW (window) };
    }

  return windows;
}

/**
 * window_change_record_windows:
 *
 * Record changes for all live windows found by traversing any window
 * sub-tree rooted at WINDOW in preorder followed by any right
 * siblings of WINDOW.  This sets the old buffer, old pixel and old
 * body pixel sizes of each live window found to the respective
 * current values.  It also sets the change stamp of each window found
 * to STAMP.  Return the number of live windows found.
 *
 * When not called by itself recursively, WINDOW is its frame's root
 * window, STAMP is the current change stamp of WINDOW's frame and
 * NUMBER is 0.
 */
unsafe fn window_change_record_windows (window: Lisp_Object, stamp: c_int,
                                        number: ptrdiff_t) -> ptrdiff_t
{
  let mut number = number;
  let mut w: *mut window = XWINDOW (window);

  while !w.is_null ()
    {
      if WINDOW_INTERNAL_P (w)
	{ number = window_change_record_windows ((*w).contents, stamp, number); }
      else
	{
	  number += 1;
	  (*w).change_stamp = stamp;
	  wset_old_buffer (w, (*w).contents);
	  (*w).old_pixel_width = (*w).pixel_width;
	  (*w).old_pixel_height = (*w).pixel_height;
	  (*w).old_body_pixel_width
	    = window_body_width (w, window_body_unit::WINDOW_BODY_IN_PIXELS);
	  (*w).old_body_pixel_height
	    = window_body_height (w, window_body_unit::WINDOW_BODY_IN_PIXELS);
	}

      w = if NILP ((*w).next) { std::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }

  return number;
}

/**
 * window_change_record:
 *
 * For each frame that has recorded changes, record its selected
 * window, update Fchange stamp, record the states of all its live
 * windows via window_change_record_windows and reset its
 * window_change and window_state_change flags.
 *
 * Record selected window in old_selected_window and selected frame in
 * old_selected_frame.
 */
unsafe extern "C" fn window_change_record ()
{
  if window_change_record_frames
    {
      let mut tail: Lisp_Object = Qnil ();
      let mut frame: Lisp_Object = Qnil ();

      FOR_EACH_FRAME! (tail, frame, {
	  let f: *mut frame = XFRAME (frame);

	  /* Record FRAME's selected window.  */
	  fset_old_selected_window (f, FRAME_SELECTED_WINDOW (f));

	  /* Bump up FRAME's change stamp.  If this wraps, make it 1 to avoid
	     that a new window (whose change stamp is always set to 0) gets
	     reported as "existing before".  */
	  (*f).change_stamp += 1;
	  if (*f).change_stamp == 0
	    { (*f).change_stamp = 1; }

	  /* Bump up the change stamps of all live windows on this frame so
	     the next call of this function can tell whether any of them
	     "existed before" and record state for each of these windows.  */
	  (*f).number_of_windows
	    = window_change_record_windows ((*f).root_window, (*f).change_stamp, 0);

	  /* Reset our flags.  */
	  FRAME_WINDOW_CHANGE! (f, false);
	  FRAME_WINDOW_STATE_CHANGE! (f, false);
	});
    }

  /* Strictly spoken we don't need old_selected_window at all - its
     value is the old selected window of old_selected_frame.  */
  old_selected_window = selected_window;
  old_selected_frame = selected_frame;
}

/** Run the buffer local value of SYMBOL in BUFFER, if any, with
    argument WINDOW.  WINDOW must specify a live window that shows or
    recently has shown BUFFER.
 */
unsafe fn run_window_change_functions_locally (symbol: Lisp_Object, buffer: Lisp_Object,
				     window: Lisp_Object)
{
  if !NILP (assq_no_quit (symbol, BVAR! (XBUFFER (buffer), local_var_alist_)))
    {
      let mut funs: Lisp_Object = buffer_local_value (symbol, buffer);

      Fset_buffer (buffer);

      while CONSP (funs)
	{
	  if !EQ (XCAR (funs), Qt ())
	    {
	      window_change_record_frames = true;
	      safe_calln! (XCAR (funs), window);
	    }

	  funs = XCDR (funs);
	}
    }
}

/** Run the default value of SYMBOL, if any, with argument FRAME.  */
unsafe fn run_window_change_functions_globally (symbol: Lisp_Object, frame: Lisp_Object)
{
  let mut funs: Lisp_Object = Fdefault_value (symbol);

  if !NILP (funs)
    {
      while CONSP (funs)
	{
	  if !EQ (XCAR (funs), Qt ())
	    {
	      window_change_record_frames = true;
	      safe_calln! (XCAR (funs), frame);
	    }

	  funs = XCDR (funs);
	}
    }
}

/**
 * run_window_change_functions:
 *
 * Run window change functions for each live frame.  This function
 * must be called from a "safe" position in redisplay_internal.
 *
 * Do not run any functions for a frame whose window_change flag is
 * nil, where no window selection happened and whose window state
 * change flag was not set since the last time this function was
 * called.  Never run any functions for tooltip frames.
 *
 * The change functions run are, in this order:
 *
 * 'window-buffer-change-functions' which are run for a window that
 * changed its buffer or that was not shown the last time window
 * change functions were run.  The default value is also run when a
 * window was deleted since the last time window change functions were
 * run.
 *
 * `window-size-change-functions' run for a window that changed its
 * body or total size, a window that changed its buffer or a window
 * that was not shown the last time window change functions were run.
 *
 * `window-selected-change-functions' run for a window that was
 * (de-)selected since the last time window change functions were run.
 *
 * `window-state-change-functions' run for a window for which any of
 * the above three changes occurred.
 *
 * A buffer-local value of these functions is run if and only if the
 * window for which the functions are run currently shows the buffer.
 * Each call gets one argument - the window showing the buffer.  This
 * means that the buffer-local value of these functions may be called
 * as many times as the buffer is shown on the frame.
 *
 * The default values of these functions are called only after all
 * buffer-local values for all of these functions have been run.  Each
 * such call receives one argument - the frame for which a change
 * occurred.  Functions on `window-state-change-functions' are run
 * also if the corresponding frame's window state change flag has been
 * set.
 *
 * After the four change functions cited above have been run in the
 * indicated way, functions on 'window-configuration-change-hook' are
 * run.  A buffer-local value is run if a window shows that buffer and
 * has either changed its buffer or its body or total size or did not
 * appear on this frame since the last time window change functions
 * were run.  The functions are called without argument and with the
 * buffer's window selected.  The default value is run without
 * argument and with the frame for which the function is run selected.
 *
 * In a final step, functions on `window-state-change-hook' are run
 * provided a window state change has occurred or the window state
 * change flag has been set on at least one frame.  Each of these
 * functions is called without argument.
 *
 * This function does not save and restore match data.  Any functions
 * it calls are responsible for doing that themselves.
 *
 * Additionally, report changes to each frame's selected window to the
 * input method in textconv.c.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_window_change_functions ()
{
  let mut tail: Lisp_Object = Qnil ();
  let mut frame: Lisp_Object = Qnil ();
  let selected_frame_change: bool = !EQ (selected_frame, old_selected_frame);
  let mut run_window_state_change_hook: bool = false;
  let count: specpdl_ref = SPECPDL_INDEX ();

  window_change_record_frames = false;
  record_unwind_protect_void (Some (window_change_record));
  specbind (Qinhibit_redisplay (), Qt ());

  FOR_EACH_FRAME! (tail, frame, {
      let f: *mut frame = XFRAME (frame);
      let root: Lisp_Object = FRAME_ROOT_WINDOW (f);
      let frame_window_change: bool = FRAME_WINDOW_CHANGE (f);
      let mut window_buffer_change: bool = false;
      let mut window_size_change: bool = false;
      let mut frame_buffer_change: bool = false;
      let mut frame_size_change: bool = false;
      let frame_selected_change: bool
	= (selected_frame_change
	   && (EQ (frame, old_selected_frame)
	       || EQ (frame, selected_frame)));
      let frame_selected_window_change: bool
	= !EQ (FRAME_OLD_SELECTED_WINDOW (f), FRAME_SELECTED_WINDOW (f));
      let frame_window_state_change: bool = FRAME_WINDOW_STATE_CHANGE (f);
      let mut window_deleted: bool = false;
      let mut windows: Lisp_Object;
      let mut number_of_windows: ptrdiff_t;

      /* C written as a guard: the body used to end with `continue', which a Rust `while`
         does not turn into the loop's increment (P-159).  */
      if !( !FRAME_LIVE_P (f)
	  || !can_set_window_size! (f)
	  || !after_make_frame! (f)
	  || FRAME_TOOLTIP_P (f)
	  || !(frame_window_change
	       || frame_selected_change
	       || frame_selected_window_change
	       || frame_window_state_change))
        {

      /* Analyze windows and run buffer locals hooks in pre-order.  */
      windows = Fnreverse (window_sub_list (root, Qnil ()));
      number_of_windows = 0;

      /* The following loop collects all data needed to tell whether
	 the default value of a hook shall be run and runs any buffer
	 local hooks right away.  */
      while CONSP (windows)
	{
	  let window: Lisp_Object = XCAR (windows);
	  let w: *mut window = XWINDOW (window);
	  let buffer: Lisp_Object = WINDOW_BUFFER! (w);
	  let count1: specpdl_ref = SPECPDL_INDEX ();

	  /* Count this window even if it has been deleted while
	     running a hook.  */
	  number_of_windows += 1;

	  if !WINDOW_LIVE_P (window)
	    { windows = XCDR (windows); continue; }

	  /* A "buffer change" means either the window's buffer
	     changed or the window was not part of this frame the last
	     time window change functions were run for it.  */
	  window_buffer_change =
	    (frame_window_change
	     && (!EQ (buffer, (*w).old_buffer)
		 || (*w).change_stamp != (*f).change_stamp));
	  /* A "size change" means either a buffer change or that the
	     total or body size of the window has changed.

	     Note: A buffer change implies a size change because either
	     this window didn't show the buffer before or this window
	     didn't show the buffer the last time the window change
	     functions were run.  In either case, an application
	     tracing size changes in a buffer-locally fashion might
	     want to be informed about that change.  */
	  window_size_change =
	    (frame_window_change
	     && (window_buffer_change
		 || (*w).pixel_width != (*w).old_pixel_width
		 || (*w).pixel_height != (*w).old_pixel_height
		 || (window_body_width (w, window_body_unit::WINDOW_BODY_IN_PIXELS)
		     != (*w).old_body_pixel_width)
		 || (window_body_height (w, window_body_unit::WINDOW_BODY_IN_PIXELS)
		     != (*w).old_body_pixel_height)));

	  /* The following two are needed when running the default
	     values for this frame below.  */
	  frame_buffer_change = frame_buffer_change || window_buffer_change;
	  frame_size_change = frame_size_change || window_size_change;

	  /* Prepare for running local hooks in their buffers.  */
	  record_unwind_current_buffer ();

	  if window_buffer_change
	    {
	      if BUFFERP ((*w).old_buffer)
		  && BUFFER_LIVE_P (XBUFFER ((*w).old_buffer))
		{ run_window_change_functions_locally
		  (Qwindow_buffer_change_functions (), (*w).old_buffer, window); }

	      run_window_change_functions_locally
		(Qwindow_buffer_change_functions (), buffer, window);
	    }

	  if window_size_change && WINDOW_LIVE_P (window)
	    { run_window_change_functions_locally
	      (Qwindow_size_change_functions (), buffer, window); }

	  /* This window's selection has changed when it was
	     (de-)selected as its frame's or the globally selected
	     window.  */
	  if (((frame_selected_change
		&& (EQ (window, old_selected_window)
		    || EQ (window, selected_window)))
	       || (frame_selected_window_change
		   && (EQ (window, FRAME_OLD_SELECTED_WINDOW (f))
		       || EQ (window, FRAME_SELECTED_WINDOW (f)))))
	      && WINDOW_LIVE_P (window))
	    { run_window_change_functions_locally
	      (Qwindow_selection_change_functions (), buffer, window); }

	  /* This window's state has changed when its buffer or size
	     changed or it was (de-)selected as its frame's or the
	     globally selected window.  */
	  if ((window_buffer_change
	       || window_size_change
	       || ((frame_selected_change
		    && (EQ (window, old_selected_window)
			|| EQ (window, selected_window)))
		   || (frame_selected_window_change
		       && (EQ (window, FRAME_OLD_SELECTED_WINDOW (f))
			   || EQ (window, FRAME_SELECTED_WINDOW (f))))))
	      && WINDOW_LIVE_P (window))
	    { run_window_change_functions_locally
	      (Qwindow_state_change_functions (), buffer, window); }

	  /* Restore current buffer for running the global hooks.  */
	  unbind_to (count1, Qnil ());
	  windows = XCDR (windows);
	}

      /* When the number of windows on a frame has decreased, at least
	 one window of that frame was deleted.  In that case, we want
	 to run the default buffer and configuration change hooks.  The
	 default size change hook is not necessarily run in that case,
	 but usually will be unless the deletion was "compensated" by
	 a reduction of the frame size or an increase of a minibuffer
	 window size.  */
      window_deleted = number_of_windows < (*f).number_of_windows;
      /* A frame changed buffers when one of its windows has changed
	 its buffer or at least one window was deleted.  */
      if (frame_buffer_change || window_deleted) && FRAME_LIVE_P (f)
	{ run_window_change_functions_globally
	  (Qwindow_buffer_change_functions (), frame); }

      /* A size change occurred when at least one of the frame's
	 windows has changed size.  */
      if frame_size_change && FRAME_LIVE_P (f)
	{ run_window_change_functions_globally
	  (Qwindow_size_change_functions (), frame); }

      /* A frame has changed its window selection when its selected
	 window has changed or when it was (de-)selected.  */
      if (frame_selected_change || frame_selected_window_change)
	  && FRAME_LIVE_P (f)
	{ run_window_change_functions_globally
	  (Qwindow_selection_change_functions (), frame); }

      /* C 4282-4292: the `HAVE_TEXT_CONVERSION` arm
	 (`report_selected_window_change`) is dead (P-08).  */

      /* A frame has changed state when a size or buffer change
	 occurred, its selected window has changed, when it was
	 (de-)selected or its window state change flag was set.  */
      if (frame_selected_change || frame_selected_window_change
	   || frame_buffer_change || window_deleted
	   || frame_size_change || frame_window_state_change)
	  && FRAME_LIVE_P (f)
	{
	  run_window_change_functions_globally
	    (Qwindow_state_change_functions (), frame);
	  /* Make sure to run 'window-state-change-hook' later.  */
	  run_window_state_change_hook = true;
	  /*  Make sure to record changes for each live frame in
	     window_change_record later.  */
	  window_change_record_frames = true;
	}

      /* A frame's configuration changed when one of its windows has
	 changed buffer or size or at least one window was deleted.  */
      if (frame_size_change || window_deleted) && FRAME_LIVE_P (f)
	/* This will run any buffer local window configuration change
	   hook as well.  */
	{ run_window_configuration_change_hook (f); }
	  }
    });

  /* Run 'window-state-change-hook' if at least one frame has changed
     state.  */
  if run_window_state_change_hook && !NILP (Vwindow_state_change_hook)
    { safe_run_hooks (Qwindow_state_change_hook ()); }

  /* Record changes for all frames (if asked for), selected window and
     frame.  */
  unbind_to (count, Qnil ());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_window_buffer (window: Lisp_Object, buffer: Lisp_Object,
		   run_hooks_p: bool, keep_margins_p: bool)
{
  let w: *mut window = XWINDOW (window);
  let b: *mut buffer = XBUFFER (buffer);
  let count: specpdl_ref = SPECPDL_INDEX ();
  let samebuf: bool = EQ (buffer, (*w).contents);

  /* It's never OK to assign WINDOW a dead buffer.  */
  eassert! (BUFFER_LIVE_P (b));

  wset_buffer (w, buffer);

  if EQ (window, selected_window)
    { bset_last_selected_window (b, window); }

  /* Let redisplay errors through.  */
  (*b).display_error_modiff = 0;

  /* Update time stamps of buffer display.  */
  if INTEGERP (BVAR! (b, display_count_))
    { bset_display_count (b, Fadd1 (BVAR! (b, display_count_))); }
  bset_display_time (b, Fcurrent_time ());

  (*w).window_end_pos = 0;
  (*w).window_end_vpos = 0;
  (*w).last_cursor_vpos = 0;

  /* Discard BUFFER from WINDOW's previous and next buffers.  */
  window_discard_buffer_from_window (buffer, window, false);

  if !(keep_margins_p && samebuf)
    { /* If we're not actually changing the buffer, don't reset hscroll
	 and vscroll.  Resetting hscroll and vscroll here is problematic
	 for things like image-mode and doc-view-mode since it resets
	 the image's position whenever we resize the frame.  */
      (*w).hscroll = 0; (*w).min_hscroll = 0; (*w).hscroll_whole = 0;
      window_bits! (w, suspend_auto_hscroll, false);
      (*w).vscroll = 0;
      set_marker_both ((*w).pointm, buffer, BUF_PT (b), BUF_PT_BYTE (b));
      set_marker_both ((*w).old_pointm, buffer, BUF_PT (b), BUF_PT_BYTE (b));
      set_marker_restricted ((*w).start,
			     make_fixnum ((*b).last_window_start as EMACS_INT),
			     buffer);
      window_bits! (w, start_at_line_beg, false);
      window_bits! (w, force_start, false);
      /* Flush the base_line cache since it applied to another buffer.  */
      (*w).base_line_number = 0;
    }

  wset_redisplay (w);
  wset_update_mode_line (w);

  /* We must select BUFFER to run the window-scroll-functions and to look up
     the buffer-local value of Vwindow_point_insertion_type.  */
  record_unwind_current_buffer ();
  Fset_buffer (buffer);

  /* lisp.h 2830-2833: `insertion_type` is bit 1 of `Lisp_Marker.bits` (the mirror's
     MARKER_INSERTION_TYPE; docs/ports/token-aliases.tsv's family keeps the token).  */
  let insertion_type: bool = !NILP (Vwindow_point_insertion_type);
  (*XMARKER ((*w).pointm)).bits = ((*XMARKER ((*w).pointm)).bits & ! MARKER_INSERTION_TYPE)
    | (if insertion_type { MARKER_INSERTION_TYPE } else { 0 });
  (*XMARKER ((*w).old_pointm)).bits = ((*XMARKER ((*w).old_pointm)).bits & ! MARKER_INSERTION_TYPE)
    | (if insertion_type { MARKER_INSERTION_TYPE } else { 0 });

  if !keep_margins_p
    {
      /* Set fringes and scroll bars from buffer unless they have been
	 declared as persistent.  */
      if !window_bits! (w, fringes_persistent)
	{ set_window_fringes (w, BVAR! (b, left_fringe_width_),
			    BVAR! (b, right_fringe_width_),
			    BVAR! (b, fringes_outside_margins_), Qnil ()); }
      if !window_bits! (w, scroll_bars_persistent)
	{ set_window_scroll_bars (w, BVAR! (b, scroll_bar_width_),
				BVAR! (b, vertical_scroll_bar_type_),
				BVAR! (b, scroll_bar_height_),
				BVAR! (b, horizontal_scroll_bar_type_), Qnil ()); }
      /* Set left and right marginal area width from buffer.  */
      set_window_margins (w, BVAR! (b, left_margin_cols_),
			  BVAR! (b, right_margin_cols_));
      apply_window_adjustment (w);
    }

  if run_hooks_p && !NILP (Vwindow_scroll_functions)
    { run_hook_with_args_2 (Qwindow_scroll_functions (), window,
			  Fmarker_position ((*w).start)); }

  /* Ensure that window change functions are run later if the buffer
     differs and the window is neither a mini nor a pseudo window.

     Note: Running window change functions for the minibuffer is noisy
     and was generally suppressed in the past.  Is there any reason we
     should run them?  */
  if !samebuf && !MINI_WINDOW_P (w) && !WINDOW_PSEUDO_P (w)
    { FRAME_WINDOW_CHANGE! (XFRAME ((*w).frame), true); }

  unbind_to (count, Qnil ());
}

DEFUN! ("set-window-buffer", Fset_window_buffer, Sset_window_buffer, 2, 3, 0,
       doc: "Make WINDOW display BUFFER-OR-NAME.\nWINDOW must be a live window and defaults to the selected one.\nBUFFER-OR-NAME must be a buffer or the name of an existing buffer.\n\nOptional third argument KEEP-MARGINS non-nil means that WINDOW's current\ndisplay margins, fringe widths, and scroll bar settings are preserved;\nthe default is to reset these from the local settings for BUFFER-OR-NAME\nor the frame defaults.  Return nil.\n\nThis function throws an error when WINDOW is strongly dedicated to its\nbuffer (that is `window-dedicated-p' returns t for WINDOW) and does not\nalready display BUFFER-OR-NAME.\n\nThis function runs `window-scroll-functions' before running\n`window-configuration-change-hook'.  ",
       (window: Lisp_Object, buffer_or_name: Lisp_Object, keep_margins: Lisp_Object),
       {
  let mut tem: Lisp_Object;
  let buffer: Lisp_Object;
  let mut window = window;
  let w: *mut window = decode_live_window (window);

  XSETWINDOW! (window, w);
  buffer = Fget_buffer (buffer_or_name);
  CHECK_BUFFER (buffer);
  if !BUFFER_LIVE_P (XBUFFER (buffer))
    { error (c"Attempt to display deleted buffer".as_ptr ()); }

  tem = (*w).contents;
  if NILP (tem)
    { error (c"Window is deleted".as_ptr ()); }
  else
    {
      if !EQ (tem, buffer)
	{
	  if EQ ((*w).dedicated, Qt ())
	    /* WINDOW is strongly dedicated to its buffer, signal an
	       error.  */
	    { error (c"Window is dedicated to `%s'".as_ptr (),
		     SDATA (BVAR! (XBUFFER (tem), name_))); }
	  else
	    /* WINDOW is weakly dedicated to its buffer, reset
	       dedication.  */
	    { wset_dedicated (w, Qnil ()); }

	  calln! (Qrecord_window_buffer (), window);
	}

      unshow_buffer (w);
    }

  set_window_buffer (window, buffer, true, !NILP (keep_margins));

  return Qnil ();
});

unsafe fn display_buffer (buffer: Lisp_Object, not_this_window_p: Lisp_Object,
                          override_frame: Lisp_Object) -> Lisp_Object
{
  return calln! (Qdisplay_buffer (), buffer, not_this_window_p, override_frame);
}

DEFUN! ("force-window-update", Fforce_window_update, Sforce_window_update,
       0, 1, 0,
       doc: "Force all windows to be updated on next redisplay.\nIf optional arg OBJECT is a window, force redisplay of that window only.\nIf OBJECT is a buffer or buffer name, force redisplay of all windows\ndisplaying that buffer.  ",
       (object: Lisp_Object),
       {
  let mut object = object;
  if NILP (object)
    {
      windows_or_buffers_changed = 29;
      update_mode_lines = 28;
      return Qt ();
    }

  if WINDOW_LIVE_P (object)
    {
      let w: *mut window = XWINDOW (object);
      mark_window_display_accurate (object, false);
      window_bits! (w, update_mode_line, true);
      if BUFFERP ((*w).contents)
	{ (*XBUFFER ((*w).contents)).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P; }
      update_mode_lines = 29;
      return Qt ();
    }

  if STRINGP (object)
    { object = Fget_buffer (object); }
  if BUFFERP (object) && BUFFER_LIVE_P (XBUFFER (object))
      && buffer_window_count (XBUFFER (object)) != 0
    {
      /* If buffer is live and shown in at least one window, find
	 all windows showing this buffer and force update of them.  */
      object = window_loop (window_loop::REDISPLAY_BUFFER_WINDOWS, object, false, Qvisible ());
      return if NILP (object) { Qnil () } else { Qt () };
    }

  /* If nothing suitable was found, just return.
     We could signal an error, but this feature will typically be used
     asynchronously in timers or process sentinels, so we don't.  */
  return Qnil ();
});

/* Obsolete since 24.3.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn temp_output_buffer_show (buf: Lisp_Object)
{
  let old: *mut buffer = (current_buffer! ());
  let mut window: Lisp_Object = Qnil ();
  let mut w: *mut window;

  bset_directory (XBUFFER (buf), BVAR! ((current_buffer! ()), directory_));

  Fset_buffer (buf);
  BUF_SAVE_MODIFF! (XBUFFER (buf)) = MODIFF! ();
  BEGV! () = BEG as ptrdiff_t;
  ZV! () = Z! ();
  SET_PT (BEG as ptrdiff_t);
  set_buffer_internal (old);

  if !NILP (Vtemp_buffer_show_function)
    { calln! (Vtemp_buffer_show_function, buf); }
  else if WINDOW_LIVE_P ({ window = display_buffer (buf, Qnil (), Qnil ()); window })
    {
      if !EQ ((*XWINDOW (window)).frame, selected_frame)
	{ Fmake_frame_visible (WINDOW_FRAME! (XWINDOW (window))); }
      Vminibuf_scroll_window = window;
      w = XWINDOW (window);
      (*w).hscroll = 0; (*w).min_hscroll = 0; (*w).hscroll_whole = 0;
      window_bits! (w, suspend_auto_hscroll, false);
      set_marker_restricted_both ((*w).start, buf, BEG as ptrdiff_t, BEG as ptrdiff_t);
      set_marker_restricted_both ((*w).pointm, buf, BEG as ptrdiff_t, BEG as ptrdiff_t);
      set_marker_restricted_both ((*w).old_pointm, buf, BEG as ptrdiff_t, BEG as ptrdiff_t);

      /* Run temp-buffer-show-hook, with the chosen window selected
	 and its buffer current.  */
      {
        let count: specpdl_ref = SPECPDL_INDEX ();
        let prev_window: Lisp_Object;
        /* C names this local `prev_buffer`; the port renames it because `keyboard.rs`'s
           mirror declares a `prev_buffer` static at the crate root (a local cannot shadow a
           static) — recorded in docs/ports/window.md.  */
        let mut prev_buffer_: Lisp_Object = Qnil ();
        prev_window = selected_window;
        XSETBUFFER! (prev_buffer_, old);

        /* Select the window that was chosen, for running the hook.
           Note: Both Fselect_window and select_window_norecord may
           set-buffer to the buffer displayed in the window,
           so we need to save the current buffer.  --stef  */
        record_unwind_protect (Some (restore_buffer), prev_buffer_);
        record_unwind_protect (Some (select_window_norecord), prev_window);
        Fselect_window (window, Qt ());
        Fset_buffer ((*w).contents);
        run_hook (Qtemp_buffer_show_hook ());
        unbind_to (count, Qnil ());
      }
    }
}

/* Allocate basically initialized window.  */

unsafe fn allocate_window () -> *mut window
{
  return ALLOCATE_ZEROED_PSEUDOVECTOR! (window, mode_line_help_echo, pvec_type::PVEC_WINDOW);
}

/* Make new window from scratch.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_window () -> Lisp_Object
{
  let mut window: Lisp_Object = Qnil ();
  let w: *mut window;

  w = allocate_window ();
  /* Initialize Lisp data.  Note that allocate_window initializes all
     Lisp data to nil, so do it only for slots which should not be nil.  */
  wset_normal_lines (w, make_float (1.0));
  wset_normal_cols (w, make_float (1.0));
  wset_new_total (w, make_fixnum (0));
  wset_new_normal (w, make_fixnum (0));
  wset_new_pixel (w, make_fixnum (0));
  wset_start (w, Fmake_marker ());
  wset_pointm (w, Fmake_marker ());
  wset_old_pointm (w, Fmake_marker ());
  wset_vertical_scroll_bar_type (w, Qt ());
  wset_horizontal_scroll_bar_type (w, Qt ());
  wset_cursor_type (w, Qt ());

  /* Initialize non-Lisp data.  Note that allocate_window zeroes out all
     non-Lisp data, so do it only for slots which should not be zero.  */
  (*w).nrows_scale_factor = 1; (*w).ncols_scale_factor = 1;
  (*w).left_fringe_width = -1; (*w).right_fringe_width = -1;
  (*w).mode_line_height = -1; (*w).tab_line_height = -1; (*w).header_line_height = -1;
  /* C 4624-4627: the `HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
  (*w).sequence_number = { sequence_number += 1; sequence_number };
  (*w).scroll_bar_width = -1;
  (*w).scroll_bar_height = -1;
  (*w).column_number_displayed = -1;
  /* Reset window_list.  */
  Vwindow_list = Qnil ();
  /* Return window.  */
  XSETWINDOW! (window, w);
  return window;
}

DEFUN! ("set-window-new-pixel", Fset_window_new_pixel, Sset_window_new_pixel, 2, 3, 0,
       doc: "Set new pixel size of WINDOW to SIZE.\nWINDOW must be a valid window and defaults to the selected one.\nReturn SIZE.\n\nOptional argument ADD non-nil means add SIZE to the new pixel size of\nWINDOW and return the sum.\n\nThe new pixel size of WINDOW, if valid, will be shortly installed as\nWINDOW's pixel height (see `window-pixel-height') or pixel width (see\n`window-pixel-width').\n\nNote: This function does not operate on any child windows of WINDOW.  ",
       (window: Lisp_Object, size: Lisp_Object, add: Lisp_Object),
       {
  let w: *mut window = decode_valid_window (window);
  let size_min: EMACS_INT = if NILP (add) { 0 } else { - XFIXNUM ((*w).new_pixel) };
  let size_max: EMACS_INT = size_min + min (INT_MAX as isize, MOST_POSITIVE_FIXNUM as isize);

  let checked_size: c_int = check_integer_range (size, size_min as intmax_t,
                                                 size_max as intmax_t) as c_int;
  if NILP (add)
    { wset_new_pixel (w, size); }
  else
    { wset_new_pixel (w, make_fixnum (XFIXNUM ((*w).new_pixel) + checked_size as EMACS_INT)); }

  return (*w).new_pixel;
});

DEFUN! ("set-window-new-total", Fset_window_new_total, Sset_window_new_total, 2, 3, 0,
       doc: "Set new total size of WINDOW to SIZE.\nWINDOW must be a valid window and defaults to the selected one.\nReturn SIZE.\n\nOptional argument ADD non-nil means add SIZE to the new total size of\nWINDOW and return the sum.\n\nThe new total size of WINDOW, if valid, will be shortly installed as\nWINDOW's total height (see `window-total-height') or total width (see\n`window-total-width').\n\nNote: This function does not operate on any child windows of WINDOW.  ",
       (window: Lisp_Object, size: Lisp_Object, add: Lisp_Object),
       {
  let w: *mut window = decode_valid_window (window);

  CHECK_FIXNUM (size);
  if NILP (add)
    { wset_new_total (w, size); }
  else
    { wset_new_total (w, make_fixnum (XFIXNUM ((*w).new_total) + XFIXNUM (size))); }

  return (*w).new_total;
});

DEFUN! ("set-window-new-normal", Fset_window_new_normal, Sset_window_new_normal, 1, 2, 0,
       doc: "Set new normal size of WINDOW to SIZE.\nWINDOW must be a valid window and defaults to the selected one.\nReturn SIZE.\n\nThe new normal size of WINDOW, if valid, will be shortly installed as\nWINDOW's normal size (see `window-normal-size').\n\nNote: This function does not operate on any child windows of WINDOW.  ",
       (window: Lisp_Object, size: Lisp_Object),
       {
  wset_new_normal (decode_valid_window (window), size);
  return size;
});

/* Return true if setting w->pixel_height (w->pixel_width if HORFLAG)
   to w->new_pixel would result in correct heights (widths)
   for window W and recursively all child windows of W.

   Note: This function does not check any of `window-fixed-size-p',
   `window-min-height' or `window-min-width'.  It does check that window
   sizes do not drop below one line (two columns). */
unsafe fn window_resize_check (w: *mut window, horflag: bool) -> bool
{
  let f: *mut frame = XFRAME ((*w).frame);
  let mut c: *mut window;

  if WINDOW_VERTICAL_COMBINATION_P (w)
    /* W is a vertical combination.  */
    {
      c = XWINDOW ((*w).contents);
      if horflag
	/* All child windows of W must have the same width as W.  */
	{
	  while !c.is_null ()
	    {
	      if XFIXNUM ((*c).new_pixel) != XFIXNUM ((*w).new_pixel)
		  || !window_resize_check (c, horflag)
		{ return false; }

	      c = if NILP ((*c).next) { std::ptr::null_mut () } else { XWINDOW ((*c).next) };
	    }

	  return true;
	}
      else
	/* The sum of the heights of the child windows of W must equal
	   W's height.  */
	{
	  let mut remaining_pixels: c_int = XFIXNUM ((*w).new_pixel) as c_int;

	  while !c.is_null ()
	    {
	      if !window_resize_check (c, horflag)
		{ return false; }

	      remaining_pixels -= XFIXNUM ((*c).new_pixel) as c_int;
	      if remaining_pixels < 0
		{ return false; }
	      c = if NILP ((*c).next) { std::ptr::null_mut () } else { XWINDOW ((*c).next) };
	    }

	  return remaining_pixels == 0;
	}
    }
  else if WINDOW_HORIZONTAL_COMBINATION_P (w)
    /* W is a horizontal combination.  */
    {
      c = XWINDOW ((*w).contents);
      if horflag
	/* The sum of the widths of the child windows of W must equal W's
	   width.  */
	{
	  let mut remaining_pixels: c_int = XFIXNUM ((*w).new_pixel) as c_int;

	  while !c.is_null ()
	    {
	      if !window_resize_check (c, horflag)
		{ return false; }

	      remaining_pixels -= XFIXNUM ((*c).new_pixel) as c_int;
	      if remaining_pixels < 0
		{ return false; }
	      c = if NILP ((*c).next) { std::ptr::null_mut () } else { XWINDOW ((*c).next) };
	    }

	  return remaining_pixels == 0;
	}
      else
	/* All child windows of W must have the same height as W.  */
	{
	  while !c.is_null ()
	    {
	      if XFIXNUM ((*c).new_pixel) != XFIXNUM ((*w).new_pixel)
		  || !window_resize_check (c, horflag)
		{ return false; }

	      c = if NILP ((*c).next) { std::ptr::null_mut () } else { XWINDOW ((*c).next) };
	    }

	  return true;
	}
    }
  else
    /* A leaf window.  Make sure it's not too small.  The following
       hardcodes the values of `window-safe-min-width' (2) and
       `window-safe-min-height' (1) which are defined in window.el.  */
    { return XFIXNUM ((*w).new_pixel) >= (if horflag
				       { 2 * FRAME_COLUMN_WIDTH (f) }
				       else { FRAME_LINE_HEIGHT (f) }) as EMACS_INT; }
}

/* Set w->pixel_height (w->pixel_width if HORFLAG) to
   w->new_pixel for window W and recursively all child windows of W.
   Also calculate and assign the new vertical (horizontal) pixel start
   positions of each of these windows.

   This function does not perform any error checks.  Make sure you have
   run window_resize_check on W before applying this function.  */
unsafe fn window_resize_apply (w: *mut window, horflag: bool)
{
  let mut c: *mut window;
  let mut edge: c_int;
  let unit: c_int = if horflag
	      { FRAME_COLUMN_WIDTH (WINDOW_XFRAME! (w)) }
	      else { FRAME_LINE_HEIGHT (WINDOW_XFRAME! (w)) };

  /* Note: Assigning new_normal requires that the new total size of the
     parent window has been set *before*.  */
  if horflag
    {
      (*w).pixel_width = XFIXNAT ((*w).new_pixel) as c_int;
      (*w).total_cols = (*w).pixel_width / unit;
      if NUMBERP ((*w).new_normal)
	{ wset_normal_cols (w, (*w).new_normal); }

      edge = (*w).pixel_left;
    }
  else
    {
      (*w).pixel_height = XFIXNAT ((*w).new_pixel) as c_int;
      (*w).total_lines = (*w).pixel_height / unit;
      if NUMBERP ((*w).new_normal)
	{ wset_normal_lines (w, (*w).new_normal); }

      edge = (*w).pixel_top;
    }

  if WINDOW_VERTICAL_COMBINATION_P (w)
    /* W is a vertical combination.  */
    {
      c = XWINDOW ((*w).contents);
      while !c.is_null ()
	{
	  if horflag
	    {
	      (*c).pixel_left = edge;
	      (*c).left_col = edge / unit;
	    }
	  else
	    {
	      (*c).pixel_top = edge;
	      (*c).top_line = edge / unit;
	    }
	  window_resize_apply (c, horflag);
	  if !horflag
	    { edge = edge + (*c).pixel_height; }

	  c = if NILP ((*c).next) { std::ptr::null_mut () } else { XWINDOW ((*c).next) };
	}
    }
  else if WINDOW_HORIZONTAL_COMBINATION_P (w)
    /* W is a horizontal combination.  */
    {
      c = XWINDOW ((*w).contents);
      while !c.is_null ()
	{
	  if horflag
	    {
	      (*c).pixel_left = edge;
	      (*c).left_col = edge / unit;
	    }
	  else
	    {
	      (*c).pixel_top = edge;
	      (*c).top_line = edge / unit;
	    }

	  window_resize_apply (c, horflag);
	  if horflag
	    { edge = edge + (*c).pixel_width; }

	  c = if NILP ((*c).next) { std::ptr::null_mut () } else { XWINDOW ((*c).next) };
	}
    }
  else
    /* Bug#15957.  */
    { window_bits! (w, window_end_valid, false); }

  if !WINDOW_PSEUDO_P (w)
    { FRAME_WINDOW_CHANGE! (WINDOW_XFRAME! (w), true); }
}

/* Set w->total_lines (w->total_cols if HORFLAG) to
   w->new_total for window W and recursively all child windows of W.
   Also calculate and assign the new vertical (horizontal) start
   positions of each of these windows.  */

unsafe fn window_resize_apply_total (w: *mut window, horflag: bool)
{
  let mut c: *mut window;
  let mut edge: c_int;

  /* Note: Assigning new_normal requires that the new total size of the
     parent window has been set *before*.  */
  if horflag
    {
      (*w).total_cols = XFIXNAT ((*w).new_total) as c_int;
      edge = (*w).left_col;
    }
  else
    {
      (*w).total_lines = XFIXNAT ((*w).new_total) as c_int;
      edge = (*w).top_line;
    }

  if WINDOW_VERTICAL_COMBINATION_P (w)
    /* W is a vertical combination.  */
    {
      c = XWINDOW ((*w).contents);
      while !c.is_null ()
	{
	  if horflag
	    { (*c).left_col = edge; }
	  else
	    { (*c).top_line = edge; }

	  window_resize_apply_total (c, horflag);
	  if !horflag
	    { edge = edge + (*c).total_lines; }

	  c = if NILP ((*c).next) { std::ptr::null_mut () } else { XWINDOW ((*c).next) };
	}
    }
  else if WINDOW_HORIZONTAL_COMBINATION_P (w)
    /* W is a horizontal combination.  */
    {
      c = XWINDOW ((*w).contents);
      while !c.is_null ()
	{
	  if horflag
	    { (*c).left_col = edge; }
	  else
	    { (*c).top_line = edge; }

	  window_resize_apply_total (c, horflag);
	  if horflag
	    { edge = edge + (*c).total_cols; }

	  c = if NILP ((*c).next) { std::ptr::null_mut () } else { XWINDOW ((*c).next) };
	}
    }
}

DEFUN! ("window-resize-apply", Fwindow_resize_apply, Swindow_resize_apply, 0, 2, 0,
       doc: "Apply requested size values for window-tree of FRAME.\nIf FRAME is omitted or nil, it defaults to the selected frame.\n\nOptional argument HORIZONTAL omitted or nil means apply requested\nheight values.  HORIZONTAL non-nil means apply requested width values.\n\nThe requested size values are those set by `set-window-new-pixel' and\n`set-window-new-normal'.  This function checks whether the requested\nvalues sum up to a valid window layout, recursively assigns the new\nsizes of all child windows and calculates and assigns the new start\npositions of these windows.\n\nReturn t if the requested values have been applied correctly, nil\notherwise.\n\nNote: This function does not check any of `window-fixed-size-p',\n`window-min-height' or `window-min-width'.  All these checks have to\nbe applied on the Elisp level.",
       (frame: Lisp_Object, horizontal: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let r: *mut window = XWINDOW (FRAME_ROOT_WINDOW (f));
  let horflag: bool = !NILP (horizontal);

  if !window_resize_check (r, horflag)
      || (XFIXNUM ((*r).new_pixel)
	  != (if horflag { (*r).pixel_width } else { (*r).pixel_height }) as EMACS_INT)
    { return Qnil (); }

  block_input ();
  window_resize_apply (r, horflag);

  fset_redisplay (f as *mut c_void);

  adjust_frame_glyphs (f);
  unblock_input ();

  return Qt ();
});


DEFUN! ("window-resize-apply-total", Fwindow_resize_apply_total, Swindow_resize_apply_total, 0, 2, 0,
       doc: "Apply requested total size values for window-tree of FRAME.\nIf FRAME is omitted or nil, it defaults to the selected frame.\n\nThis function does not assign pixel or normal size values.  You should\nhave run `window-resize-apply' before running this.\n\nOptional argument HORIZONTAL omitted or nil means apply requested\nheight values.  HORIZONTAL non-nil means apply requested width\nvalues.",
       (frame: Lisp_Object, horizontal: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let r: *mut window = XWINDOW (FRAME_ROOT_WINDOW (f));

  block_input ();
  /* Necessary when deleting the top-/or leftmost window.  */
  (*r).left_col = 0;
  (*r).top_line = FRAME_TOP_MARGIN (f);
  window_resize_apply_total (r, !NILP (horizontal));
  /* Handle the mini window.  */
  if FRAME_HAS_MINIBUF_P (f) && !FRAME_MINIBUF_ONLY_P (f)
    {
      let m: *mut window = XWINDOW ((*f).minibuffer_window);

      if NILP (horizontal)
	{
	  (*m).top_line = (*r).top_line + (*r).total_lines;
	  (*m).total_lines = XFIXNAT ((*m).new_total) as c_int;
	}
      else
	{ (*m).total_cols = XFIXNAT ((*m).new_total) as c_int; }
    }

  unblock_input ();

  return Qt ();
});

/* Resize frame F's windows when F's inner height (inner width if
   HORFLAG is true) has been set to SIZE pixels.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn resize_frame_windows (f: *mut frame, size: c_int, horflag: bool)
{
  let root: Lisp_Object = (*f).root_window;
  let r: *mut window = XWINDOW (root);
  let old_pixel_size: c_int = if horflag { (*r).pixel_width } else { (*r).pixel_height };
  let mut new_size: c_int;
  let mut new_pixel_size: c_int;
  let unit: c_int = if horflag { FRAME_COLUMN_WIDTH (f) } else { FRAME_LINE_HEIGHT (f) };
  let mini: Lisp_Object = (*f).minibuffer_window;
  let mut m: *mut window = if WINDOWP (mini) { XWINDOW (mini) } else { std::ptr::null_mut () };
  let mini_height: c_int = if FRAME_HAS_MINIBUF_P (f) && !FRAME_MINIBUF_ONLY_P (f)
			     { (unit + (*m).pixel_height
				- window_body_height (m, window_body_unit::WINDOW_BODY_IN_PIXELS)) }
			     else { 0 };

  new_pixel_size = max ((if horflag { size } else { size - mini_height }) as isize,
			unit as isize) as c_int;
  new_size = new_pixel_size / unit;

  if new_pixel_size == old_pixel_size
      && (horflag || (*r).pixel_top == FRAME_TOP_MARGIN_HEIGHT (f))
    { }
  else if WINDOW_LEAF_P (r)
    {
      /* For a leaf root window just set the size.  */
      if horflag
	{
	  (*r).total_cols = new_size;
	  (*r).pixel_width = new_pixel_size;
	}
      else
	{
	  (*r).top_line = FRAME_TOP_MARGIN (f);
	  (*r).pixel_top = FRAME_TOP_MARGIN_HEIGHT (f);

	  (*r).total_lines = new_size;
	  (*r).pixel_height = new_pixel_size;
	}

      FRAME_WINDOW_CHANGE! (f, !WINDOW_PSEUDO_P (r) && new_pixel_size != old_pixel_size);
    }
  else
    {
      let mut delta: Lisp_Object;

      if !horflag
	{
	  (*r).top_line = FRAME_TOP_MARGIN (f);
	  (*r).pixel_top = FRAME_TOP_MARGIN_HEIGHT (f);
	}

      XSETINT! (delta, (new_pixel_size - old_pixel_size) as EMACS_INT);

      /* Try a "normal" resize first.  */
      resize_root_window (root, delta, if horflag { Qt () } else { Qnil () }, Qnil (), Qt ());
      if window_resize_check (r, horflag)
	  && new_pixel_size as EMACS_INT == XFIXNUM ((*r).new_pixel)
	{
	  window_resize_apply (r, horflag);
	  window_pixel_to_total ((*r).frame, if horflag { Qt () } else { Qnil () });
	}
      else
	{
	  /* Try with "reasonable" minimum sizes next.  */
	  resize_root_window (root, delta, if horflag { Qt () } else { Qnil () }, Qt (), Qt ());
	  if window_resize_check (r, horflag)
	      && new_pixel_size as EMACS_INT == XFIXNUM ((*r).new_pixel)
	    {
	      window_resize_apply (r, horflag);
	      window_pixel_to_total ((*r).frame, if horflag { Qt () } else { Qnil () });
	    }
	}
    }

  if FRAME_HAS_MINIBUF_P (f) && !FRAME_MINIBUF_ONLY_P (f)
    {
      m = XWINDOW (mini);
      if horflag
	{
	  (*m).total_cols = new_size;
	  (*m).pixel_width = new_pixel_size;
	}
      else
	{
	  (*m).total_lines = mini_height / unit;
	  (*m).pixel_height = mini_height;
	  (*m).top_line = (*r).top_line + (*r).total_lines;
	  (*m).pixel_top = (*r).pixel_top + (*r).pixel_height;
	}
    }

  fset_redisplay (f as *mut c_void);
}


/** Make parent window on FRAME and return its object.  */
unsafe fn make_parent_window (frame: Lisp_Object) -> Lisp_Object
{
  let mut parent: Lisp_Object;
  let p: *mut window = allocate_window ();

  (*p).sequence_number = {
    sequence_number += 1;
    sequence_number
  };
  wset_frame (p, frame);
  XSETWINDOW! (parent, p);

  return parent;
}


/** Assign parent and normal sizes to all child windows of window P.
    Normal sizes are calculated from the child windows' pixel widths and
    heights so these must be in place already.  This function is called
    when parent/child window relationships change without resizing as in
    Fcombine_windows or Funcombine_window.  */
unsafe fn window_set_parent_and_normal_sizes (parent: Lisp_Object)
{
  let p: *mut window = XWINDOW (parent);
  let mut w: *mut window = XWINDOW ((*p).contents);
  /* Normal size left for yet unprocessed windows.  */
  let mut remainder: f64 = 1.0;

  if window_bits! (p, horizontal)
    {
      while true
	{
	  wset_parent (w, parent);
	  wset_normal_lines (w, make_float (1.0));

	  if NILP ((*w).next)
	    {
	      wset_normal_cols (w, make_float (remainder));
	      break;
	    }
	  else
	    {
	      wset_normal_cols (w, make_float ((*w).pixel_width as f64
					       / (*p).pixel_width as f64));
	      remainder -= XFLOAT_DATA ((*w).normal_cols);
	      w = XWINDOW ((*w).next);
	    }
	}
    }
  else
    {
      while true
	{
	  wset_parent (w, parent);
	  wset_normal_cols (w, make_float (1.0));

	  if NILP ((*w).next)
	    {
	      wset_normal_lines (w, make_float (remainder));
	      break;
	    }
	  else
	    {
	      wset_normal_lines (w, make_float ((*w).pixel_height as f64
						/ (*p).pixel_height as f64));
	      remainder -= XFLOAT_DATA ((*w).normal_lines);
	      w = XWINDOW ((*w).next);
	    }
	}
    }
}


DEFUN! ("combine-windows", Fcombine_windows, Scombine_windows, 2, 2, 0,
       doc: "Combine windows from FIRST to LAST inclusive.\nFIRST and LAST must be different, valid windows in the same combination,\nthat is, windows with the same parent window.  If LAST is not reachable\nfrom FIRST by applying `window-next-sibling' repeatedly, invert FIRST\nand LAST.\n\nIf FIRST has no previous and LAST has no next sibling, return nil.\nOtherwise, make a new parent window whose first child window becomes\nFIRST and whose last child window becomes LAST, insert that parent\nwindow in the window tree in lieu of the windows starting with FIRST and\nending with LAST and return the new parent window.",
       (first: Lisp_Object, last: Lisp_Object),
       {
  let mut f: *mut window = decode_valid_window (first);
  let mut l: *mut window = decode_valid_window (last);
  let mut w: *mut window = f;
  let mut first = first;
  let mut last = last;

  if f == l
    /* Don't make a matryoshka window.  */
    { error (c"Cannot combine a window with itself".as_ptr ()); }

  while w != l && !NILP ((*w).next)
    { w = XWINDOW ((*w).next); }

  if w != l
    {
      w = l;

      while w != f && !NILP ((*w).next)
	{ w = XWINDOW ((*w).next); }

      if w == f
	/* Invert FIRST and LAST.  */
	{
	  f = l;
	  l = w;
	  XSETWINDOW! (first, f);
	  XSETWINDOW! (last, l);
	}
      else
	{ error (c"Windows to combine must be children of same parent".as_ptr ()); }
    }

  if NILP ((*f).prev) && NILP ((*l).next)
    /* FIRST and LAST are already the first and last child of their
       parent.  */
    { return Qnil (); }

  /* Make new parent window PARENT.  */
  let parent: Lisp_Object = make_parent_window ((*f).frame);
  let p: *mut window = XWINDOW (parent);
  let horflag: bool = window_bits! (XWINDOW ((*f).parent), horizontal);

  /* Splice in PARENT into the window tree.  */
  wset_parent (p, (*f).parent);
  wset_combination (p, horflag, first);

  if NILP ((*f).prev)
    /* FIRST has no previous sibling.  Make PARENT the first child of
       FIRST's old parent.  */
    { wset_combination (XWINDOW ((*p).parent), horflag, parent); }
  else
    /* FIRST has a previous sibling.  Make PARENT the new next sibling
       of FIRST's previous sibling.  */
    {
      wset_next (XWINDOW ((*f).prev), parent);
      wset_prev (p, (*f).prev);
      wset_prev (f, Qnil ());
    }

  if !NILP ((*l).next)
    /* LAST has a next sibling.  Make PARENT the new previous sibling of
       LAST's old next sibling.  */
    {
      wset_prev (XWINDOW ((*l).next), parent);
      wset_next (p, (*l).next);
      wset_next (l, Qnil ());
    }

  /* Set up PARENT's positions and sizes.  */
  (*p).pixel_left = (*f).pixel_left;
  (*p).left_col = (*f).left_col;
  (*p).pixel_top = (*f).pixel_top;
  (*p).top_line = (*f).top_line;

  if horflag
    {
      (*p).pixel_width = (*l).pixel_left + (*l).pixel_width - (*f).pixel_left;
      (*p).total_cols = (*l).left_col + (*l).total_cols - (*f).left_col;
      (*p).pixel_height = (*f).pixel_height;
      (*p).total_lines = (*f).total_lines;
    }
  else
    {
      (*p).pixel_height = (*l).pixel_top + (*l).pixel_height - (*f).pixel_top;
      (*p).total_lines = (*l).top_line + (*l).total_lines - (*f).top_line;
      (*p).pixel_width = (*f).pixel_width;
      (*p).total_cols = (*f).total_cols;
    }

  /* Fix parent slots for PARENT's children and assign them new normal
     sizes.  */
  window_set_parent_and_normal_sizes (parent);
  /* Assign new normal sizes for PARENT and its siblings.  */
  window_set_parent_and_normal_sizes ((*p).parent);

  return parent;
});


DEFUN! ("uncombine-window", Funcombine_window, Suncombine_window, 1, 1, 0,
       doc: "Uncombine specified WINDOW.\nWINDOW should be an internal window whose parent window is an internal\nwindow of the same type.  This means, that WINDOW and its parent should\nbe either both horizontal or both vertical window combinations.  If this\nis the case, make the child windows of WINDOW become child windows of\nWINDOW's parent and return t.  Otherwise, leave the current configuration\nof WINDOW's frame unchanged and return nil.",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_valid_window (window);
  /* PARENT is WINDOW's parent and is supposed to be a combination of
     the same type as WINDOW.  */
  let parent: Lisp_Object = (*w).parent;

  if MINI_WINDOW_P (w)
    { error (c"Cannot uncombine a mini window".as_ptr ()); }

  if WINDOW_INTERNAL_P (w) && !NILP (parent)
      && window_bits! (w, horizontal) == window_bits! (XWINDOW (parent), horizontal)
    {
      let p: *mut window = XWINDOW ((*w).parent);
      /* WINDOW's first child.  */
      let first: Lisp_Object = (*w).contents;
      let f: *mut window = XWINDOW (first);
      /* WINDOW's last child.  */
      let mut last: Lisp_Object = Qnil ();
      let mut l: *mut window = f;

      /* Find last child window of WINDOW.  */
      while !NILP ((*l).next)
	{ l = XWINDOW ((*l).next); }
      XSETWINDOW! (last, l);

      wset_prev (f, (*w).prev);
      if NILP ((*f).prev)
	{ wset_combination (p, window_bits! (p, horizontal), first); }
      else
	{ wset_next (XWINDOW ((*f).prev), first); }

      wset_next (l, (*w).next);
      if !NILP ((*l).next)
	{ wset_prev (XWINDOW ((*w).next), last); }

      /* Fix parent slots for PARENT's new children and assign new normal
	 sizes.  */
      window_set_parent_and_normal_sizes (parent);

      return Qt ();
    }
  else
    { return Qnil (); }
});

DEFUN! ("split-window-internal", Fsplit_window_internal, Ssplit_window_internal, 4, 5, 0,
       doc: "Split window OLD.\nSecond argument PIXEL-SIZE specifies the number of pixels of the\nnew window.  It must be a positive integer.\n\nThird argument SIDE nil (or `below') specifies that the new window shall\nbe located below WINDOW.  SIDE `above' means the new window shall be\nlocated above WINDOW.  In both cases PIXEL-SIZE specifies the pixel\nheight of the new window including space reserved for the mode and/or\nheader/tab line.\n\nSIDE t (or `right') specifies that the new window shall be located on\nthe right side of WINDOW.  SIDE `left' means the new window shall be\nlocated on the left of WINDOW.  In both cases PIXEL-SIZE specifies the\nwidth of the new window including space reserved for fringes and the\nscroll bar or a divider column.\n\nFourth argument NORMAL-SIZE specifies the normal size of the new window\naccording to the SIDE argument.  Optional fifth argument REFER is as for\n'split-window'.\n\nThe new pixel and normal sizes of all involved windows must have been\nset correctly.  See the code of `split-window' for how this is done.",
       (old: Lisp_Object, pixel_size: Lisp_Object, side: Lisp_Object,
        normal_size: Lisp_Object, refer: Lisp_Object),
       {
  /* OLD (*o) is the window to split.  REFER (*r) is a reference window,
     either an arbitrary live window or a former live, now deleted
     window on the same frame as OLD.  NEW (*n) is the new window
     created anew or resurrected from REFER (*r), if specified.  *p
     refers either to OLD's parent window that will become NEW's parent
     window too or to a new internal window that becomes OLD's and NEW's
     new parent.  */
  let o: *mut window = decode_valid_window (old);
  let frame: Lisp_Object = WINDOW_FRAME! (o);
  let f: *mut frame = XFRAME (frame);
  let mut p: *mut window;
  let mut n: *mut window;
  let mut r: *mut window;
  let mut c: *mut window;
  let horflag: bool
    /* HORFLAG is true when we split side-by-side, false otherwise.  */
    = EQ (side, Qt ()) || EQ (side, Qleft ()) || EQ (side, Qright ());
  let mut new: Lisp_Object;
  let mut parent: Lisp_Object = Qnil ();
  let mut dead: bool = false;
  let mut refer = refer;

  CHECK_FIXNUM (pixel_size);
  let total_size: EMACS_INT
    = XFIXNUM (pixel_size) / (if horflag
			   { FRAME_COLUMN_WIDTH (f) }
			   else { FRAME_LINE_HEIGHT (f) }) as EMACS_INT;

  /* Set combination_limit if we have to make a new parent window.
     We do that if either `window-combination-limit' is t, or OLD has no
     parent, or OLD is ortho-combined.  */
  let combination_limit: bool
    = (EQ (Vwindow_combination_limit, Qt ())
       || NILP ((*o).parent)
       || (if horflag
	   { WINDOW_VERTICAL_COMBINATION_P (XWINDOW ((*o).parent)) }
	   else { WINDOW_HORIZONTAL_COMBINATION_P (XWINDOW ((*o).parent)) }));

  /* Set up reference window.  */
  if NILP (refer)
    {
      if WINDOW_LIVE_P (old)
	/* OLD is live, use it as reference window.  */
	{ refer = old; }
      else
	/* Use the frame's selected window as reference window.  */
	{ refer = FRAME_SELECTED_WINDOW (f); }

      r = XWINDOW (refer);
    }
  else if CONSP (refer)
    {
      /* If REFER is a cons, then its car must be a deleted, former live
	 window and its cdr must be a deleted former parent window.  Set
	 PARENT to the cdr of REFER and REFER to its car.  WINDOW and
	 REFER end up as the sole children of PARENT which replaces
	 WINDOW in the window tree.  As a special case, if REFER's cdr
	 is t, reuse REFER's car's old parent as new parent provided it
	 is a deleted former parent window.  */
      parent = Fcdr (refer);
      refer = Fcar (refer);
      r = decode_any_window (refer);

      if !NILP ((*r).contents) || !BUFFERP ((*r).old_buffer)
	{ error (c"REFER's car must specify a deleted, former live window".as_ptr ()); }
      else if !BUFFER_LIVE_P (XBUFFER ((*r).old_buffer))
	{ error (c"The buffer formerly shown by REFER's car has been killed".as_ptr ()); }
      else if !EQ ((*r).frame, frame)
	{ error (c"REFER's car must specify a window on same frame as WINDOW".as_ptr ()); }

      if EQ (parent, Qt ())
	/* If REFER's cdr is t, use the old parent of REFER's car as new
	   parent.  */
	{ parent = (*r).parent; }

      p = decode_any_window (parent);

      if !NILP ((*p).contents) || BUFFERP ((*p).old_buffer)
	{ error (c"REFER's cdr must specify a deleted, former parent window".as_ptr ()); }
      else if !EQ ((*p).frame, frame)
	{ error (c"REFER's cdr must specify window on same frame as WINDOW".as_ptr ()); }

      dead = true;
    }
  else
    {
      r = decode_any_window (refer);

      if NILP ((*r).contents)
	/* Presumably a deleted, former live window.  Check whether its
	   contents can be used.  */
	{
	  if !BUFFERP ((*r).old_buffer)
	    { error (c"REFER must specify a former live window (must have shown a buffer)".as_ptr ()); }
	  else if !BUFFER_LIVE_P (XBUFFER ((*r).old_buffer))
	    { error (c"The buffer formerly shown by REFER has been killed".as_ptr ()); }
	  else if !EQ ((*r).frame, frame)
	    { error (c"REFER must specify a window on same frame as WINDOW".as_ptr ()); }

	  dead = true;
	}
      else if !NILP (parent)
	{ error (c"If REFER is a cons, its car must not specify a live window".as_ptr ()); }
      else if !WINDOW_LIVE_P (refer)
	{ error (c"REFER is not a live window (does not show a buffer)".as_ptr ()); }
    }

  /* The following bugs are caught by `split-window'.  */
  if MINI_WINDOW_P (o)
    { error (c"Attempt to split minibuffer window".as_ptr ()); }
  else if total_size < (if horflag { 2 } else { 1 })
    { error (c"Size of new window too small (after split)".as_ptr ()); }
  else if !combination_limit && !NILP (Vwindow_combination_resize)
    /* `window-combination-resize' non-nil means try to resize OLD's siblings
       proportionally.  */
    {
      let op: *mut window = XWINDOW ((*o).parent);

      /* Temporarily pretend we split the parent window.  */
      wset_new_pixel
	(op, make_fixnum (((if horflag { (*op).pixel_width } else { (*op).pixel_height }) as EMACS_INT)
			 - XFIXNUM (pixel_size)));
      if !window_resize_check (op, horflag)
	{ error (c"Window sizes don't fit".as_ptr ()); }
      else
	/* Undo the temporary pretension.  */
	{ wset_new_pixel
	  (op, make_fixnum ((if horflag { (*op).pixel_width } else { (*op).pixel_height }) as EMACS_INT)); }
    }
  else
    {
      if !window_resize_check (o, horflag)
	{ error (c"Resizing old window failed".as_ptr ()); }
      else if XFIXNUM (pixel_size) + XFIXNUM ((*o).new_pixel)
	       != (if horflag { (*o).pixel_width } else { (*o).pixel_height }) as EMACS_INT
	{ error (c"Sum of sizes of old and new window don't fit".as_ptr ()); }
    }

  /* This is our point of no return.  */
  if combination_limit
    {
      /* Save the old value of o->normal_cols/lines.  It gets corrupted
	 by make_parent_window and we need it below for assigning it to
	 p->new_normal.  */
      let new_normal: Lisp_Object
	= if horflag { (*o).normal_cols } else { (*o).normal_lines };

      if NILP (parent)
	{
	  parent = make_parent_window (frame);
	  p = XWINDOW (parent);
	}
      else
	/* Pacify GCC.  */
	{ p = XWINDOW (parent); }

      replace_window (old, parent, true);
      wset_next (o, Qnil ());
      wset_prev (o, Qnil ());
      wset_parent (o, parent);
      wset_combination (p, horflag, old);

      if EQ (Vwindow_combination_limit, Qt ())
	/* Store t in the new parent's combination_limit slot to avoid
	   that its children get merged into another window.  */
	{ wset_combination_limit (p, Qt ()); }
      /* These get applied below.  */
      wset_new_pixel
	(p, make_fixnum ((if horflag { (*o).pixel_width } else { (*o).pixel_height }) as EMACS_INT));
      wset_new_total
	(p, make_fixnum ((if horflag { (*o).total_cols } else { (*o).total_lines }) as EMACS_INT));
      wset_new_normal (p, new_normal);
    }
  else
    { p = XWINDOW ((*o).parent); }

  fset_redisplay (f as *mut c_void);

  if dead
    { new = refer; }
  else
    { new = make_window (); }

  n = XWINDOW (new);
  wset_frame (n, frame);
  wset_parent (n, (*o).parent);

  if EQ (side, Qabove ()) || EQ (side, Qleft ())
    {
      wset_prev (n, (*o).prev);
      if NILP ((*n).prev)
	{ wset_combination (p, horflag, new); }
      else
	{ wset_next (XWINDOW ((*n).prev), new); }
      wset_next (n, old);
      wset_prev (o, new);
    }
  else
    {
      wset_next (n, (*o).next);
      if !NILP ((*n).next)
	{ wset_prev (XWINDOW ((*n).next), new); }
      wset_prev (n, old);
      wset_next (o, new);
    }

  window_bits! (n, window_end_valid, false);
  (*n).last_cursor_vpos = 0;

  if !dead
    {
      /* Get special geometry settings from reference window.  */
      (*n).left_margin_cols = (*r).left_margin_cols;
      (*n).right_margin_cols = (*r).right_margin_cols;
      (*n).left_fringe_width = (*r).left_fringe_width;
      (*n).right_fringe_width = (*r).right_fringe_width;
      window_bits! (n, fringes_outside_margins, window_bits! (r, fringes_outside_margins));
      (*n).scroll_bar_width = (*r).scroll_bar_width;
      (*n).scroll_bar_height = (*r).scroll_bar_height;
      wset_vertical_scroll_bar_type (n, (*r).vertical_scroll_bar_type);
      wset_horizontal_scroll_bar_type (n, (*r).horizontal_scroll_bar_type);
    }

  /* Directly assign orthogonal coordinates and sizes.  */
  if horflag
    {
      (*n).pixel_top = (*o).pixel_top;
      (*n).top_line = (*o).top_line;
      (*n).pixel_height = (*o).pixel_height;
      (*n).total_lines = (*o).total_lines;
    }
  else
    {
      (*n).pixel_left = (*o).pixel_left;
      (*n).left_col = (*o).left_col;
      (*n).pixel_width = (*o).pixel_width;
      (*n).total_cols = (*o).total_cols;
    }

  /* Iso-coordinates and sizes are assigned by window_resize_apply,
     get them ready here.  */
  wset_new_pixel (n, pixel_size);
  let mut sum: EMACS_INT = 0;
  c = XWINDOW ((*p).contents);
  while !c.is_null ()
    {
      if c != n
	{ sum = sum + XFIXNUM ((*c).new_total); }
      c = if NILP ((*c).next) { std::ptr::null_mut () } else { XWINDOW ((*c).next) };
    }

  wset_new_total (n, make_fixnum (((if horflag
				   { (*p).total_cols }
				   else { (*p).total_lines }) as EMACS_INT)
				  - sum));
  wset_new_normal (n, normal_size);

  block_input ();

  if dead
    {
      /* Get dead window back its old buffer and markers.  */
      wset_buffer (n, (*n).old_buffer);
      set_marker_restricted
	((*n).start, make_fixnum ((*XMARKER ((*n).start)).charpos as EMACS_INT), (*n).contents);
      set_marker_restricted
	((*n).pointm, make_fixnum ((*XMARKER ((*n).pointm)).charpos as EMACS_INT), (*n).contents);
      set_marker_restricted
	((*n).old_pointm, make_fixnum ((*XMARKER ((*n).old_pointm)).charpos as EMACS_INT),
	 (*n).contents);

      Vwindow_list = Qnil ();
      /* Remove window from the table of dead windows.  */
      Fremhash (make_fixnum ((*n).sequence_number as EMACS_INT),
		window_dead_windows_table);
    }

  window_resize_apply (p, horflag);
  adjust_frame_glyphs (f);

  set_window_buffer (new, (*r).contents, true, true);

  FRAME_WINDOW_CHANGE! (f, true);
  unblock_input ();

  return new;
});

/// C: xwidget.h 235: `INLINE void xwidget_view_delete_all_in_window (struct window *w) {}`
/// — the `#else` arm of `#ifdef HAVE_XWIDGETS` (undef in the frozen configuration); xwidget.h
/// has no mirror, so the port keeps C's per-unit `static inline` copy here (the buffer.rs
/// `kill_buffer_xwidgets` shape).  docs/ports/window.md reports the gap.
#[inline(always)]
unsafe fn xwidget_view_delete_all_in_window (_w: *mut window) {}

/* frame.h 1174-1176: `SET_FRAME_GARBAGED` is a three-term comma expression in C; the port keeps
   the same three effects in the same order.  The mirror's copy (`include/frame.rs` 947) cannot
   expand here - it passes `*mut frame` where this tree declares `fset_redisplay (*mut c_void)`
   (E0308; the cast below is the working spelling) - so the unit keeps its own, reported to the
   mirror's owner.  */

/* frame.h 378: `bool_bf redisplay : 1` — frame.rs defines the same member-named accessor for
   its own unit (PORTING.md 1.3/P-80); this unit does not include it, so it is defined here.  */
macro_rules! redisplay {
    ($f:expr) => { (*$f).bits1 >> FRAME_REDISPLAY_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_REDISPLAY_SHIFT, $v) };
}

/* frame.h 1187: `FRAME_WINDOWS_FROZEN` is `(f)->frozen_window_starts` — a `bool_bf`
   the mirror only reads (`FRAME_WINDOWS_FROZEN`'s `frame_bit1`), so the write goes through
   the pack helper (PORTING.md 1.3/P-80; the `FRAME_WINDOW_CHANGE` shape above).  */
macro_rules! FRAME_WINDOWS_FROZEN {
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_FROZEN_WINDOW_STARTS_SHIFT, $v) };
}

DEFUN! ("delete-window-internal", Fdelete_window_internal, Sdelete_window_internal, 1, 1, 0,
       doc: "Remove WINDOW from its frame.\nWINDOW defaults to the selected window.  Return nil.\nSignal an error when WINDOW is the only window on its frame.",
       (window: Lisp_Object),
       {
  let mut parent: Lisp_Object;
  let mut sibling: Lisp_Object;
  let frame: Lisp_Object;
  let root: Lisp_Object;
  let mut w: *mut window;
  let p: *mut window;
  let mut s: *mut window;
  let r: *mut window;
  let f: *mut frame;
  let horflag: bool;
  let mut before_sibling: bool = false;
  let mut window = window;

  w = decode_any_window (window);
  XSETWINDOW! (window, w);
  if NILP ((*w).contents)
    /* It's a no-op to delete an already deleted window.  */
    { return Qnil (); }

  parent = (*w).parent;
  if NILP (parent)
    /* Never delete a minibuffer or frame root window.  */
    { error (c"Attempt to delete minibuffer or sole ordinary window".as_ptr ()); }
  else if NILP ((*w).prev) && NILP ((*w).next)
    /* Rather bow out here, this case should be handled on the Elisp
       level.  */
    { error (c"Attempt to delete sole window of parent".as_ptr ()); }

  p = XWINDOW (parent);
  horflag = WINDOW_HORIZONTAL_COMBINATION_P (p);

  frame = WINDOW_FRAME! (w);
  f = XFRAME (frame);

  root = FRAME_ROOT_WINDOW (f);
  r = XWINDOW (root);

  /* Unlink WINDOW from window tree.  */
  if NILP ((*w).prev)
    /* Get SIBLING below (on the right of) WINDOW.  */
    {
      /* before_sibling means WINDOW is the first child of its
	 parent and thus before the sibling.  */
      before_sibling = true;
      sibling = (*w).next;
      s = XWINDOW (sibling);
      wset_prev (s, Qnil ());
      wset_combination (p, horflag, sibling);
    }
  else
    /* Get SIBLING above (on the left of) WINDOW.  */
    {
      sibling = (*w).prev;
      s = XWINDOW (sibling);
      wset_next (s, (*w).next);
      if !NILP ((*s).next)
	{ wset_prev (XWINDOW ((*s).next), sibling); }
    }

  if window_resize_check (r, horflag)
      && (XFIXNUM ((*r).new_pixel)
	  == (if horflag { (*r).pixel_width } else { (*r).pixel_height }) as EMACS_INT)
    /* We can delete WINDOW now.  */
    {

      /* Block input.  */
      block_input ();
      xwidget_view_delete_all_in_window (w);
      window_resize_apply (p, horflag);
      /* If this window is referred to by the dpyinfo's mouse
	 highlight, invalidate that slot to be safe (Bug#9904).  */
      if !FRAME_INITIAL_P (f)
	{
	  let hlinfo: *mut Mouse_HLInfo = MOUSE_HL_INFO! (f);

	  if EQ ((*hlinfo).mouse_face_window, window)
	    { (*hlinfo).mouse_face_window = Qnil (); }
	}

      fset_redisplay (f as *mut c_void);
      Vwindow_list = Qnil ();

      wset_next (w, Qnil ());  /* Don't delete w->next too.  */
      free_window_matrices (w);

      if WINDOWP ((*w).contents)
	{
	  delete_all_child_windows ((*w).contents);
	  wset_combination (w, false, Qnil ());
	}
      else
	{
	  /* Store WINDOW's buffer in old_buffer.  */
	  wset_old_buffer (w, (*w).contents);
	  unshow_buffer (w);
	  unchain_marker (XMARKER ((*w).pointm) as *mut c_void);
	  unchain_marker (XMARKER ((*w).old_pointm) as *mut c_void);
	  unchain_marker (XMARKER ((*w).start) as *mut c_void);
	  wset_buffer (w, Qnil ());
	  /* Add WINDOW to table of dead windows so when killing a buffer
	     WINDOW mentions, all references to that buffer can be removed
	     and the buffer be collected.  */
	  Fputhash (make_fixnum ((*w).sequence_number as EMACS_INT),
		    window, window_dead_windows_table);
	}

      if NILP ((*s).prev) && NILP ((*s).next)
	  /* A matrjoshka where SIBLING has become the only child of
	     PARENT.  */
	{
	  /* Put SIBLING into PARENT's place.  */
	  replace_window (parent, sibling, false);
	  /* Have SIBLING inherit the following three slot values from
	     PARENT (the combination_limit slot is not inherited).  */
	  wset_normal_cols (s, (*p).normal_cols);
	  wset_normal_lines (s, (*p).normal_lines);
	  /* Mark PARENT as deleted.  */
	  wset_combination (p, false, Qnil ());
	  /* Try to merge SIBLING into its new parent.  */
	  recombine_windows (sibling);
	}

      adjust_frame_glyphs (f);

      if !WINDOW_LIVE_P (FRAME_SELECTED_WINDOW (f))
	/* We apparently deleted the frame's selected window; use the
	   frame's first window as substitute but don't record it yet.
	   `delete-window' may have something better up its sleeves.  */
	{
	  /* Use the frame's first window as fallback ...  */
	  let new_selected_window: Lisp_Object = Fframe_first_window (frame);

	  if EQ (FRAME_SELECTED_WINDOW (f), selected_window)
	    { Fselect_window (new_selected_window, Qt ()); }
	  else
	    /* Do not clear f->select_mini_window_flag here.  If the
	       last selected window on F was an active minibuffer, we
	       want to return to it on a later Fselect_frame.  */
	    { fset_selected_window (f, new_selected_window); }
	}

      unblock_input ();
      FRAME_WINDOW_CHANGE! (f, true);
    }
  else
    /* We failed: Relink WINDOW into window tree.  */
    {
      if before_sibling
	{
	  wset_prev (s, window);
	  wset_combination (p, horflag, window);
	}
      else
	{
	  wset_next (s, window);
	  if !NILP ((*w).next)
	    { wset_prev (XWINDOW ((*w).next), window); }
	}
      error (c"Deletion failed".as_ptr ());
    }

  return Qnil ();
});

/***********************************************************************
			Resizing Mini-Windows
 ***********************************************************************/

/**
 * resize_mini_window_apply:
 *
 * Assign new window sizes after resizing a mini window W by DELTA
 * pixels.  No error checking performed.
  */
unsafe fn resize_mini_window_apply (w: *mut window, delta: c_int)
{
  let f: *mut frame = XFRAME ((*w).frame);
  let root: Lisp_Object = FRAME_ROOT_WINDOW (f);
  let r: *mut window = XWINDOW (root);

  block_input ();
  (*w).pixel_height = (*w).pixel_height + delta;
  (*w).total_lines = (*w).pixel_height / FRAME_LINE_HEIGHT (f);

  window_resize_apply (r, false);

  (*w).pixel_top = (*r).pixel_top + (*r).pixel_height;
  (*w).top_line = (*r).top_line + (*r).total_lines;

  /* Enforce full redisplay of the frame.  If f->redisplay is already
     set, which it generally is in the wake of a ConfigureNotify
     (frame resize) event, merely setting f->redisplay is insufficient
     for redisplay_internal to continue redisplaying the frame, as
     redisplay_internal cannot distinguish between f->redisplay set
     before it calls redisplay_window and that after, so garbage the
     frame as well.  */

  if redisplay! (f)
    { SET_FRAME_GARBAGED! (f); }

  /* FIXME: Shouldn't some of the caller do it?  */
  fset_redisplay (f as *mut c_void);
  adjust_frame_glyphs (f);
  unblock_input ();
}

/**
 * grow_mini_window:
 *
 * Grow mini-window W by DELTA pixels.  If DELTA is negative, this may
 * shrink the minibuffer window to the minimum height to display one
 * line of text.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn grow_mini_window (w: *mut window, delta: c_int, unit: c_int)
{
  let f: *mut frame = XFRAME ((*w).frame);
  let old_height: c_int = window_body_height (w, window_body_unit::WINDOW_BODY_IN_PIXELS);
  let min_height: c_int = unit;
  let mut delta = delta;

  eassert! (MINI_WINDOW_P (w));

  /* Never shrink mini-window to less than its minimum height.  */
  if old_height + delta < min_height
    { delta = if old_height > min_height { min_height - old_height } else { 0 }; }

  if delta != 0
    {
      let root: Lisp_Object = FRAME_ROOT_WINDOW (f);
      let r: *mut window = XWINDOW (root);
      let grow: Lisp_Object;

      grow = calln! (Qwindow__resize_root_window_vertically (), root,
		     make_fixnum ((- delta) as EMACS_INT), Qt ());

      if FIXNUMP (grow)
	  /* It might be impossible to resize the window, in which case
	     calling resize_mini_window_apply will set off an infinite
	     loop where the redisplay cycle so forced returns to
	     resize_mini_window, making endless attempts to expand the
	     minibuffer window to this impossible size.  (bug#69140) */
	  && XFIXNUM (grow) != 0
	  && window_resize_check (r, false)
	{ resize_mini_window_apply (w, (- XFIXNUM (grow)) as c_int); }
    }
  FRAME_WINDOWS_FROZEN!
    (f, window_body_height (w, window_body_unit::WINDOW_BODY_IN_PIXELS) > unit);
}

/**
 * shrink_mini_window:
 *
 * Shrink mini-window W to the minimum height needed to display one
 * line of text.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shrink_mini_window (w: *mut window, unit: c_int)
{
  let f: *mut frame = XFRAME ((*w).frame);
  let delta: c_int = (window_body_height (w, window_body_unit::WINDOW_BODY_IN_PIXELS) - unit);

  eassert! (MINI_WINDOW_P (w));

  if delta > 0
    {
      let root: Lisp_Object = FRAME_ROOT_WINDOW (f);
      let r: *mut window = XWINDOW (root);
      let grow: Lisp_Object;

      grow = calln! (Qwindow__resize_root_window_vertically (), root,
		     make_fixnum (delta as EMACS_INT), Qt ());

      if FIXNUMP (grow) && window_resize_check (r, false)
	{ resize_mini_window_apply (w, (- XFIXNUM (grow)) as c_int); }
    }
  else if delta < 0
    /* delta can be less than zero after adding horizontal scroll
       bar.  */
    { grow_mini_window (w, - delta, unit); }

  FRAME_WINDOWS_FROZEN!
    (f, window_body_height (w, window_body_unit::WINDOW_BODY_IN_PIXELS) > unit);
}

DEFUN! ("resize-mini-window-internal", Fresize_mini_window_internal,
       Sresize_mini_window_internal, 1, 1, 0,
       doc: "Resize mini window WINDOW.",
       (window: Lisp_Object),
       {
  let w: *mut window = XWINDOW (window);
  let r: *mut window;
  let f: *mut frame;
  let old_height: c_int;
  let delta: c_int;

  CHECK_LIVE_WINDOW (window);
  f = XFRAME ((*w).frame);

  if !EQ (FRAME_MINIBUF_WINDOW (XFRAME ((*w).frame)), window)
    { error (c"Not a valid minibuffer window".as_ptr ()); }
  else if FRAME_MINIBUF_ONLY_P (f)
    { error (c"Cannot resize a minibuffer-only frame".as_ptr ()); }

  r = XWINDOW (FRAME_ROOT_WINDOW (f));
  old_height = (*r).pixel_height + (*w).pixel_height;
  delta = (XFIXNUM ((*w).new_pixel) - (*w).pixel_height as EMACS_INT) as c_int;
  if window_resize_check (r, false)
      && XFIXNUM ((*w).new_pixel) > 0
      && old_height as EMACS_INT == XFIXNUM ((*r).new_pixel) + XFIXNUM ((*w).new_pixel)
    {
      resize_mini_window_apply (w, delta);

      return Qt ();
    }
  else
    { error (c"Cannot resize mini window".as_ptr ()); }
});

/* Mark window cursors off for all windows in the window tree rooted
   at W by setting their phys_cursor_on_p flag to zero.  Called from
   xterm.c, e.g. when a frame is cleared and thereby all cursors on
   the frame are cleared.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_window_cursors_off (w: *mut window)
{
  let mut w = w;
  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{ mark_window_cursors_off (XWINDOW ((*w).contents)); }
      else
	{ window_bits! (w, phys_cursor_on_p, false); }

      w = if NILP ((*w).next) { std::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
}


/**
 * window_wants_mode_line:
 *
 * Return 1 if window W wants a mode line and is high enough to
 * accommodate it, 0 otherwise.
 *
 * W wants a mode line if it's a leaf window and neither a minibuffer
 * nor a pseudo window.  Moreover, its 'window-mode-line-format'
 * parameter must not be 'none' and either that parameter or W's
 * buffer's 'mode-line-format' value must be non-nil.  Finally, W must
 * be higher than its frame's canonical character height.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_wants_mode_line (w: *mut window) -> bool
{
  let window_mode_line_format: Lisp_Object =
    window_parameter (w, Qmode_line_format ());

  return (WINDOW_LEAF_P (w)
	  && !MINI_WINDOW_P (w)
	  && !WINDOW_PSEUDO_P (w)
	  && !EQ (window_mode_line_format, Qnone ())
	  && (!NILP (window_mode_line_format)
	      || !NILP (BVAR! (XBUFFER (WINDOW_BUFFER! (w)), mode_line_format_)))
	  && WINDOW_PIXEL_HEIGHT (w) > WINDOW_FRAME_LINE_HEIGHT! (w));
}


/**
 * window_wants_header_line:
 *
 * Return 1 if window W wants a header line and is high enough to
 * accommodate it, 0 otherwise.
 *
 * W wants a header line if it's a leaf window and neither a
 * minibuffer nor a pseudo window.  Moreover, its
 * 'window-header-line-format' parameter must not be 'none' and either
 * that parameter or W's buffer's 'header-line-format' value must be
 * non-nil.  Finally, W must be higher than its frame's canonical
 * character height and be able to accommodate a mode line too if
 * necessary (the mode line prevails).
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_wants_header_line (w: *mut window) -> bool
{
  let window_header_line_format: Lisp_Object =
    window_parameter (w, Qheader_line_format ());

  return (WINDOW_LEAF_P (w)
	  && !MINI_WINDOW_P (w)
	  && !WINDOW_PSEUDO_P (w)
	  && !EQ (window_header_line_format, Qnone ())
	  && (!NILP (window_header_line_format)
	      || !NILP (BVAR! (XBUFFER (WINDOW_BUFFER! (w)), header_line_format_)))
	  && (WINDOW_PIXEL_HEIGHT (w)
	      > (if window_wants_mode_line (w)
		 { 2 * WINDOW_FRAME_LINE_HEIGHT! (w) }
		 else { WINDOW_FRAME_LINE_HEIGHT! (w) })));
}


/**
 * window_wants_tab_line:
 *
 * Return 1 if window W wants a tab line and is high enough to
 * accommodate it, 0 otherwise.
 *
 * W wants a tab line if it's a leaf window and neither a minibuffer
 * nor a pseudo window.  Moreover, its 'window-tab-line-format'
 * parameter must not be 'none' and either that parameter or W's
 * buffer's 'tab-line-format' value must be non-nil.  Finally, W must
 * be higher than its frame's canonical character height and be able
 * to accommodate a mode line and a header line too if necessary (the
 * mode line and a header line prevail).
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_wants_tab_line (w: *mut window) -> bool
{
  let window_tab_line_format: Lisp_Object =
    window_parameter (w, Qtab_line_format ());

  return (WINDOW_LEAF_P (w)
	  && !MINI_WINDOW_P (w)
	  && !WINDOW_PSEUDO_P (w)
	  && !EQ (window_tab_line_format, Qnone ())
	  && (!NILP (window_tab_line_format)
	      || !NILP (BVAR! (XBUFFER (WINDOW_BUFFER! (w)), tab_line_format_)))
	  && (WINDOW_PIXEL_HEIGHT (w)
	      > (((if window_wants_mode_line (w) { 1 } else { 0 })
		  + (if window_wants_header_line (w) { 1 } else { 0 })
		  + 1) * WINDOW_FRAME_LINE_HEIGHT! (w))));
}

/* Return number of lines of text in window W, not counting the mode
   line and header line, if any.  Do NOT use this for windows on GUI
   frames; use window_body_height instead.  This function is only for
   windows on TTY frames, where it is much more efficient.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_internal_height (w: *mut window) -> c_int
{
  let mut ht: c_int = (*w).total_lines;

  if window_wants_mode_line (w)
    { ht -= 1; }

  if window_wants_header_line (w)
    { ht -= 1; }

  if window_wants_tab_line (w)
    { ht -= 1; }

  return ht;
}


/************************************************************************
			   Window Scrolling
 ***********************************************************************/

/* Scroll contents of window WINDOW up.  If WHOLE, scroll
   N screen-fulls, which is defined as the height of the window minus
   next_screen_context_lines.  If WHOLE is zero, scroll up N lines
   instead.  Negative values of N mean scroll down.  NOERROR
   means don't signal an error if we try to move over BEGV or ZV,
   respectively.  */

unsafe fn window_scroll (window: Lisp_Object, n: EMACS_INT, whole: bool, noerror: bool)
{
  let w: *mut window = XWINDOW (window);
  let b: *mut buffer = XBUFFER ((*w).contents);
  let mut long_lines_truncated: bool =
    (*b).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P != 0 && !NILP (BVAR! (b, truncate_lines_));
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut n = n;

  n = clip_to_bounds (INT_MIN as EMACS_INT, n, INT_MAX as EMACS_INT);

  wset_redisplay (w);

  /* Does this window's buffer have very long and truncated lines?  */
  if (*b).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P != 0
      && !long_lines_truncated
      && !NILP (Vtruncate_partial_width_windows)
      && (*w).total_cols < FRAME_COLS (XFRAME (WINDOW_FRAME! (w)))
    {
      if FIXNUMP (Vtruncate_partial_width_windows)
	{ long_lines_truncated =
	  (*w).total_cols < XFIXNAT (Vtruncate_partial_width_windows) as c_int; }
      else
	{ long_lines_truncated = true; }
    }

  if whole && (fast_but_imprecise_scrolling || long_lines_truncated)
    { specbind (Qfontification_functions (), Qnil ()); }

  if whole && long_lines_truncated
    { window_scroll_for_long_lines (w, n as c_int, noerror); }
  else if FRAME_WINDOW_P (XFRAME ((*XWINDOW (window)).frame))
    {

      /* On GUI frames, use the pixel-based version which is much
	 slower than the line-based one, but can handle varying
	 line heights.  */
      record_unwind_protect_void (Some (unwind_display_working_on_window));
      display_working_on_window_p = true;
      window_scroll_pixel_based (window, n as c_int, whole, noerror);
    }
  else
    { window_scroll_line_based (window, n as c_int, whole, noerror); }

  unbind_to (count, Qnil ());

  /* Bug#15957.  */
  window_bits! (XWINDOW (window), window_end_valid, false);
}

/* Compute scroll margin for WINDOW.
   We scroll when point is within this distance from the top or bottom
   of the window.  The result is measured in lines or in pixels
   depending on the second parameter.  */

unsafe extern "C" {
  /// C: dispextern.h 3567: `int default_line_pixel_height (struct window *);` (indent.c).
  pub fn default_line_pixel_height (w: *mut window) -> c_int;
  /// C: dispextern.h 3610: `extern bool display_working_on_window_p;` (xdisp.c).
  pub static mut display_working_on_window_p: bool;
  /// C: dispextern.h 3612: `extern void unwind_display_working_on_window (void);` (xdisp.c).
  pub fn unwind_display_working_on_window ();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_scroll_margin (window: *mut window, unit: margin_unit) -> c_int
{
  if scroll_margin > 0
    {
      let frame_line_height: c_int = default_line_pixel_height (window);
      let window_lines: c_int = window_box_height (window) / frame_line_height;

      let mut ratio: f64 = 0.25;
      if FLOATP (Vmaximum_scroll_margin)
        {
          ratio = XFLOAT_DATA (Vmaximum_scroll_margin);
          ratio = f64::max (0.0, ratio);
          ratio = f64::min (ratio, 0.5);
        }
      let max_margin: c_int = min (((window_lines - 1) / 2) as isize,
				   (window_lines as f64 * ratio) as c_int as isize) as c_int;
      let margin: c_int = clip_to_bounds (0, scroll_margin as EMACS_INT, max_margin as EMACS_INT)
	as c_int;
      return if unit == margin_unit::MARGIN_IN_PIXELS
        { margin * frame_line_height }
        else { margin };
    }
  else
    { return 0; }
}

unsafe fn sanitize_next_screen_context_lines () -> c_int
{
  return clip_to_bounds (0, next_screen_context_lines as EMACS_INT, 1000000) as c_int;
}

/* Implementation of window_scroll for very long and truncated lines.
   This is a simplified version, it only handles WHOLE window scrolls,
   and doesn't honor scroll-preserve-screen-position nor scroll-margin.  */

unsafe fn window_scroll_for_long_lines (w: *mut window, n: c_int, noerror: bool)
{
  let mut startpos: ptrdiff_t = marker_position ((*w).start);
  let mut startbyte: ptrdiff_t = marker_byte_position ((*w).start);
  let nscls: c_int = sanitize_next_screen_context_lines ();
  let ht: c_int = window_internal_height (w);
  let mut n = n;

  n = n * (max (1, (ht - nscls) as isize) as c_int);

  /* If point is not visible in window, bring it inside window.  */
  let mut pos: position;
  let mut rtop: c_int = 0;
  let mut rbot: c_int = 0;
  let mut dummy_rowh: c_int = 0;
  let mut dummy_vpos: c_int = 0;
  let mut dummy_x: c_int = 0;
  let mut dummy_y: c_int = 0;
  if !(PT! () >= startpos
	&& PT! () <= ZV! ()
	&& startpos <= ZV! ()
	&& pos_visible_p (w, PT! (), &raw mut dummy_x, &raw mut dummy_y, &raw mut rtop,
			  &raw mut rbot, &raw mut dummy_rowh, &raw mut dummy_vpos)
	&& rtop == 0 && rbot == 0)
    {
      pos = *vmotion (PT! (), PT_BYTE! (), (- (ht / 2)) as EMACS_INT, w as *mut c_void);
      startpos = pos.bufpos;
      startbyte = pos.bytepos;
    }
  SET_PT_BOTH (startpos, startbyte);

  let lose: bool = n < 0 && PT! () == BEGV! ();
  pos = *vmotion (PT! (), PT_BYTE! (), n as EMACS_INT, w as *mut c_void);
  if lose
    {
      if noerror
	{ return; }
      else
	{ xsignal0 (Qbeginning_of_buffer ()); }
    }

  let bolp: bool = pos.bufpos == BEGV! ()
    || FETCH_BYTE! (pos.bytepos - 1) as c_int == b'\n' as c_int;
  if pos.bufpos < ZV! ()
    {
      set_marker_restricted_both ((*w).start, (*w).contents,
				  pos.bufpos, pos.bytepos);
      window_bits! (w, start_at_line_beg, bolp);
      wset_update_mode_line (w);
      /* Set force_start so that redisplay_window will run
	 the window-scroll-functions.  */
      window_bits! (w, force_start, true);
      SET_PT_BOTH (pos.bufpos, pos.bytepos);
      if n > 0
	{ pos = *vmotion (PT! (), PT_BYTE! (), (ht / 2) as EMACS_INT, w as *mut c_void); }
      else if n < 0
	{ pos = *vmotion (PT! (), PT_BYTE! (), (- (ht / 2)) as EMACS_INT, w as *mut c_void); }
      SET_PT_BOTH (pos.bufpos, pos.bytepos);

      /* Since `vmotion' computes coordinates after vscroll is applied,
         it is taken into account in POS, and vscroll must be reset by
         `force_start' in `redisplay_internal'.  */
      window_bits! (w, preserve_vscroll_p, false);
    }
  else
    {
      if noerror
	{ return; }
      else
	{ xsignal0 (Qend_of_buffer ()); }
    }
}

/* C: dispextern.h 2977-2988 `enum move_operation_enum` and 3566/3583/3590/3600/3605/3606's
   iterator entry points.  dispextern.h is mirrored only to its batch-II prefix (P-66), so
   the enumeration and the prototypes are declared here (indent.rs declares its own copies).
   `SET_TEXT_POS` (dispextern.h 291) is the same kind of gap.  */
pub type move_operation_enum = c_int;
pub const MOVE_TO_X: move_operation_enum = 0x01;
pub const MOVE_TO_Y: move_operation_enum = 0x02;
pub const MOVE_TO_VPOS: move_operation_enum = 0x04;
pub const MOVE_TO_POS: move_operation_enum = 0x08;

macro_rules! SET_TEXT_POS { ($pos:expr, $charpos:expr, $bytepos:expr) => {
  { CHARPOS! ($pos) = $charpos; BYTEPOS! ($pos) = $bytepos; } }; }

/* keyboard.h 38: `KVAR` is a token-pasting macro (`((kboard)->field ## _)`) with no Rust
   counterpart; the port keeps the macro's name and the member token C pastes onto, one arm
   per member this file names (the keyboard.rs/term.rs shape; `KVAR` is listed in
   docs/ports/excluded-macros.txt because its *body* cannot be mirrored).  */
macro_rules! KVAR {
  ($kb:expr, Vlast_command) => { (*$kb).Vlast_command_ };
}

unsafe extern "C" {
  /// C: dispextern.h 3566: `int line_bottom_y (struct it *);` (xdisp.c).
  pub fn line_bottom_y (it: *mut it) -> c_int;
  /// C: dispextern.h 3583: `int move_it_to (struct it *, ptrdiff_t, int, int, int, int);`
  pub fn move_it_to (it: *mut it, to_charpos: ptrdiff_t, to_x: c_int, to_y: c_int,
                     to_vpos: c_int, op: c_int) -> c_int;
  /// C: dispextern.h 3590-3591: `void init_iterator (struct it *, struct window *,
  /// ptrdiff_t, ptrdiff_t, struct glyph_row *, enum face_id);`
  pub fn init_iterator (it: *mut it, w: *mut window, charpos: ptrdiff_t, bytepos: ptrdiff_t,
                        row: *mut glyph_row, base_face_id: face_id);
  /// C: dispextern.h 3599: `void move_it_vertically_backward (struct it *, int);`
  pub fn move_it_vertically_backward (it: *mut it, dy: c_int);
  /// C: dispextern.h 3600: `void move_it_by_lines (struct it *, ptrdiff_t);`
  pub fn move_it_by_lines (it: *mut it, dvpos: ptrdiff_t);
  /// C: dispextern.h 3605: `int partial_line_height (struct it *it_origin);`
  pub fn partial_line_height (it: *mut it) -> c_int;
  /// C: dispextern.h 3606: `bool in_display_vector_p (struct it *);`
  pub fn in_display_vector_p (it: *mut it) -> bool;
}

/* Implementation of window_scroll that works based on pixel line
   heights.  See the comment of window_scroll for parameter
   descriptions.  */

unsafe fn window_scroll_pixel_based (window: Lisp_Object, n: c_int, whole: bool, noerror: bool)
{
  let mut it: it = std::mem::zeroed ();
  let w: *mut window = XWINDOW (window);
  let mut start: text_pos = std::mem::zeroed ();
  let this_scroll_margin: c_int;
  /* True if we fiddled the window vscroll field without really scrolling.  */
  let mut vscrolled: bool = false;
  let mut x: c_int = 0;
  let mut y: c_int = 0;
  let mut rtop: c_int = 0;
  let mut rbot: c_int = 0;
  let mut rowh: c_int = 0;
  let mut vpos: c_int = 0;
  let mut itdata: *mut c_void = std::ptr::null_mut ();
  let frame_line_height: c_int = default_line_pixel_height (w);
  let adjust_old_pointm: bool = !NILP (Fequal (Fwindow_point (window),
					       Fwindow_old_point (window)));

  SET_TEXT_POS_FROM_MARKER! (start, (*w).start);
  /* Scrolling a minibuffer window via scroll bar when the echo area
     shows long text sometimes resets the minibuffer contents behind
     our backs.  Also, someone might narrow-to-region and immediately
     call a scroll function.  */
  if CHARPOS! (start) > ZV! () || CHARPOS! (start) < BEGV! ()
    { SET_TEXT_POS! (start, BEGV! (), BEGV_BYTE! ()); }

  /* If PT is not visible in WINDOW, move back one half of
     the screen.  Allow PT to be partially visible, otherwise
     something like (scroll-down 1) with PT in the line before
     the partially visible one would recenter.  */

  if !pos_visible_p (w, PT! (), &raw mut x, &raw mut y, &raw mut rtop, &raw mut rbot,
		     &raw mut rowh, &raw mut vpos)
    {
      itdata = bidi_shelve_cache ();
      /* Move backward half the height of the window.  Performance note:
	 vmotion used here is about 10% faster, but would give wrong
	 results for variable height lines.  */
      init_iterator (&raw mut it, w, PT! (), PT_BYTE! (), std::ptr::null_mut (), DEFAULT_FACE_ID);
      it.current_y = it.last_visible_y;
      move_it_vertically_backward (&raw mut it, window_box_height (w) / 2);

      /* The function move_iterator_vertically may move over more than
	 the specified y-distance.  If it->w is small, e.g. a
	 mini-buffer window, we may end up in front of the window's
	 display area.  Start displaying at the start of the line
	 containing PT in this case.  */
      if it.current_y <= 0
	{
	  init_iterator (&raw mut it, w, PT! (), PT_BYTE! (), std::ptr::null_mut (), DEFAULT_FACE_ID);
	  move_it_vertically_backward (&raw mut it, 0);
	  it.current_y = 0;
	}

      start = it.current.pos;
      bidi_unshelve_cache (itdata, false);
    }
  else if auto_window_vscroll_p
    {
      if rtop != 0 || rbot != 0		/* Partially visible.  */
	{
	  let mut px: c_int;
	  let mut dy: c_int = frame_line_height;
	  /* In the below we divide the window box height by the
	     frame's line height to make the result predictable when
	     the window box is not an integral multiple of the line
	     height.  This is important to ensure we get back to the
	     same position when scrolling up, then down.  */
	  if whole
	    {
	      let ht: c_int = window_box_height (w);
	      let nscls: c_int = sanitize_next_screen_context_lines ();
	      dy = max (dy as isize, ((ht / dy - nscls) * dy) as isize) as c_int;
	    }
	  dy = dy * n;

	  if n < 0
	    {
	      /* Only vscroll backwards if already vscrolled forwards.  */
	      if (*w).vscroll < 0 && rtop > 0
		{
		  px = max (0, (- (*w).vscroll
				- min (rtop as isize, (- dy) as isize) as c_int) as isize) as c_int;
		  Fset_window_vscroll (window, make_fixnum (px as EMACS_INT), Qt (),
				       Qnil ());
		  return;
		}
	    }
	  if n > 0
	    {
	      /* Do vscroll if already vscrolled or only display line.  */
	      if rbot > 0 && ((*w).vscroll < 0 || vpos == 0)
		{
		  px = max (0, (- (*w).vscroll
				+ min (rbot as isize, dy as isize) as c_int) as isize) as c_int;
		  Fset_window_vscroll (window, make_fixnum (px as EMACS_INT), Qt (),
				       Qnil ());
		  return;
		}

	      /* Maybe modify window start instead of scrolling.  */
	      if rbot > 0 || (*w).vscroll < 0
		{
		  let mut spos: ptrdiff_t;

		  Fset_window_vscroll (window, make_fixnum (0), Qt (),
				       Qnil ());
		  /* If there are other text lines above the current row,
		     move window start to current row.  Else to next row. */
		  if rbot > 0
		    { spos = XFIXNUM (Fline_beginning_position (Qnil ())); }
		  else
		    { spos = min (XFIXNUM (Fline_end_position (Qnil ())) + 1, ZV! ()); }
		  set_marker_restricted ((*w).start, make_fixnum (spos),
					 (*w).contents);
		  window_bits! (w, start_at_line_beg, true);
		  wset_update_mode_line (w);
		  /* Set force_start so that redisplay_window will run the
		     window-scroll-functions.  */
		  window_bits! (w, force_start, true);
		  return;
		}
	    }
	}
      /* Cancel previous vscroll.  */
      Fset_window_vscroll (window, make_fixnum (0), Qt (), Qnil ());
    }

  itdata = bidi_shelve_cache ();
  /* If scroll_preserve_screen_position is non-nil, we try to set
     point in the same window line as it is now, so get that line.  */
  if !NILP (Vscroll_preserve_screen_position)
    {
      /* We preserve the goal pixel coordinate across consecutive
	 calls to scroll-up, scroll-down and other commands that
	 have the `scroll-command' property.  This avoids the
	 possibility of point becoming "stuck" on a tall line when
	 scrolling by one line.  */
      if window_scroll_pixel_based_preserve_y < 0
	  || !SYMBOLP (KVAR! (current_kboard, Vlast_command))
	  || NILP (Fget (KVAR! (current_kboard, Vlast_command), Qscroll_command ()))
	{
	  start_display (&raw mut it, w, start);
	  move_it_to (&raw mut it, PT! (), -1, -1, -1, MOVE_TO_POS);
	  window_scroll_pixel_based_preserve_y = it.current_y;
	  window_scroll_pixel_based_preserve_x = it.current_x;
	}
    }
  else
    { window_scroll_pixel_based_preserve_y
      = { window_scroll_pixel_based_preserve_x = -1;
	  window_scroll_pixel_based_preserve_y }; }

  /* Move iterator it from start the specified distance forward or
     backward.  The result is the new window start.  */
  start_display (&raw mut it, w, start);
  if whole
    {
      let start_pos: ptrdiff_t = IT_CHARPOS! (it);
      let flh: c_int = frame_line_height;
      let ht: c_int = window_box_height (w);
      let nscls: c_int = sanitize_next_screen_context_lines ();
      /* In the below we divide the window box height by the frame's
	 line height to make the result predictable when the window
	 box is not an integral multiple of the line height.  This is
	 important to ensure we get back to the same position when
	 scrolling up, then down.  */
      let dy: c_int = n * (max (flh as isize, ((ht / flh - nscls) * flh) as isize) as c_int);
      let mut goal_y: c_int;
      let mut it_data: *mut c_void;

      /* Note that move_it_vertically always moves the iterator to the
         start of a line.  So, if the last line doesn't have a newline,
	 we would end up at the start of the line ending at ZV.  */
      if dy <= 0
	{
	  goal_y = it.current_y + dy;
	  move_it_vertically_backward (&raw mut it, -dy);
	  /* move_it_vertically_backward above always overshoots if DY
	     cannot be reached exactly, i.e. if it falls in the middle
	     of a screen line.  But if that screen line is large
	     (e.g., a tall image), it might make more sense to
	     undershoot instead.  */
	  if (goal_y - it.current_y) as f64 > 0.5 * flh as f64
	    {
	      it_data = bidi_shelve_cache ();
	      let mut it1: it = it;
	      if line_bottom_y (&raw mut it1) - goal_y < goal_y - it.current_y
		{ move_it_by_lines (&raw mut it, 1); }
	      bidi_unshelve_cache (it_data, true);
	    }
	  /* Ensure we actually do move, e.g. in case we are currently
	     looking at an image that is taller that the window height.  */
	  while start_pos == IT_CHARPOS! (it)
		 && start_pos > BEGV! ()
	    { move_it_by_lines (&raw mut it, -1); }
	}
      else if dy > 0
	{
	  goal_y = it.current_y + dy;
	  move_it_to (&raw mut it, ZV! (), -1, goal_y, -1, MOVE_TO_POS | MOVE_TO_Y);
	  /* Extra precision for people who want us to preserve the
	     screen position of the cursor: effectively round DY to the
	     nearest screen line, instead of rounding to zero; the latter
	     causes point to move by one line after C-v followed by M-v,
	     if the buffer has lines of different height.  */
	  if !NILP (Vscroll_preserve_screen_position)
	      && (goal_y - it.current_y) as f64 > 0.5 * flh as f64
	    {
	      it_data = bidi_shelve_cache ();
	      let mut it2: it = it;

	      move_it_by_lines (&raw mut it, 1);
	      if it.current_y as f64 > goal_y as f64 + 0.5 * flh as f64
		{
		  it = it2;
		  bidi_unshelve_cache (it_data, false);
		}
	      else
		{ bidi_unshelve_cache (it_data, true); }
	    }
	  /* Ensure we actually do move, e.g. in case we are currently
	     looking at an image that is taller that the window height.  */
	  while start_pos == IT_CHARPOS! (it)
		 && start_pos < ZV! ()
	    { move_it_by_lines (&raw mut it, 1); }
	}
    }
  else
    { move_it_by_lines (&raw mut it, n as ptrdiff_t); }

  /* We failed if we find ZV is already on the screen (scrolling up,
     means there's nothing past the end), or if we can't start any
     earlier (scrolling down, means there's nothing past the top).  */
  if (n > 0 && IT_CHARPOS! (it) == ZV! ())
      || (n < 0 && IT_CHARPOS! (it) == CHARPOS! (start))
    {
      if IT_CHARPOS! (it) == ZV! ()
	{
	  if it.current_y < it.last_visible_y
	      && (it.current_y + it.max_ascent + it.max_descent
		  > it.last_visible_y)
	    {
	      /* The last line was only partially visible, make it fully
		 visible.  */
	      (*w).vscroll = (it.last_visible_y
			    - (it.current_y + it.max_ascent + it.max_descent));
	      adjust_frame_glyphs (it.f);
	    }
	  else
	    {
	      bidi_unshelve_cache (itdata, false);
	      if noerror
		{ return; }
	      else if n < 0	/* could happen with empty buffers */
		{ xsignal0 (Qbeginning_of_buffer ()); }
	      else
		{ xsignal0 (Qend_of_buffer ()); }
	    }
	}
      else
	{
	  if (*w).vscroll != 0
	    /* The first line was only partially visible, make it fully
	       visible. */
	    { (*w).vscroll = 0; }
	  else
	    {
	      bidi_unshelve_cache (itdata, false);
	      if noerror
		{ return; }
	      else
		{ xsignal0 (Qbeginning_of_buffer ()); }
	    }
	}

      /* If control gets here, then we vscrolled.  */

      (*XBUFFER ((*w).contents)).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;

      /* Don't try to change the window start below.  */
      vscrolled = true;
    }

  if !vscrolled
    {
      let mut pos: ptrdiff_t = IT_CHARPOS! (it);
      let bytepos: ptrdiff_t;

      /* If in the middle of a multi-glyph character move forward to
	 the next character.  */
      if in_display_vector_p (&raw mut it)
	{
	  pos += 1;
	  move_it_to (&raw mut it, pos, -1, -1, -1, MOVE_TO_POS);
	}

      /* Set the window start, and set up the window for redisplay.  */
      set_marker_restricted_both ((*w).start, (*w).contents, IT_CHARPOS! (it),
				  IT_BYTEPOS! (it));
      bytepos = marker_byte_position ((*w).start);
      window_bits! (w, start_at_line_beg,
		    pos == BEGV! ()
		      || FETCH_BYTE! (bytepos - 1) as c_int == b'\n' as c_int);
      wset_update_mode_line (w);
      /* Set force_start so that redisplay_window will run the
	 window-scroll-functions.  */
      window_bits! (w, force_start, true);
    }

  /* The rest of this function uses current_y in a nonstandard way,
     not including the height of the header line if any.  */
  it.current_y = { it.vpos = 0; it.vpos };

  /* Move PT out of scroll margins.
     This code wants current_y to be zero at the window start position
     even if there is a header line.  */
  this_scroll_margin = window_scroll_margin (w, margin_unit::MARGIN_IN_PIXELS);

  if n > 0
    {
      let last_y: c_int = it.last_visible_y - this_scroll_margin - 1;

      /* We moved the window start towards ZV, so PT may be now
	 in the scroll margin at the top.  */
      if IT_CHARPOS! (it) < PT! ()
	{ move_it_to (&raw mut it, PT! (), -1, -1, -1, MOVE_TO_POS); }
      if IT_CHARPOS! (it) == PT! ()
	  && it.current_y >= this_scroll_margin
	  && it.current_y <= last_y - WINDOW_TAB_LINE_HEIGHT! (w)
				    - WINDOW_HEADER_LINE_HEIGHT! (w)
	  && (NILP (Vscroll_preserve_screen_position)
	      || EQ (Vscroll_preserve_screen_position, Qt ()))
	/* We found PT at a legitimate height.  Leave it alone.  */
	{ }
      else
	{
	  if window_scroll_pixel_based_preserve_y >= 0
	    {
	      /* Don't enter the scroll margin at the end of the window.  */
	      let goal_y: c_int = min ((last_y) as isize,
				       window_scroll_pixel_based_preserve_y as isize) as c_int;

	      /* If we have a header line, take account of it.  This
		 is necessary because we set it.current_y to 0, above.  */
	      move_it_to (&raw mut it, -1,
			  window_scroll_pixel_based_preserve_x,
			  goal_y - WINDOW_TAB_LINE_HEIGHT! (w)
				 - WINDOW_HEADER_LINE_HEIGHT! (w),
			  -1, MOVE_TO_Y | MOVE_TO_X);
	    }

	  /* Get out of the scroll margin at the top of the window.  */
	  while it.current_y < this_scroll_margin
	    {
	      let prev: c_int = it.current_y;
	      move_it_by_lines (&raw mut it, 1);
	      if prev == it.current_y
		{ break; }
	    }
	  SET_PT_BOTH (IT_CHARPOS! (it), IT_BYTEPOS! (it));
	  /* Fix up the Y position to preserve, if it is inside the
	     scroll margin at the window top.  */
	  if window_scroll_pixel_based_preserve_y >= 0
	      && window_scroll_pixel_based_preserve_y < this_scroll_margin
	    { window_scroll_pixel_based_preserve_y = this_scroll_margin; }
	}
    }
  else if n < 0
    {
      let mut charpos: ptrdiff_t;
      let mut bytepos: ptrdiff_t;
      let mut partial_p: bool = false;

      /* Save our position, for the
	 window_scroll_pixel_based_preserve_y case.  */
      charpos = IT_CHARPOS! (it);
      bytepos = IT_BYTEPOS! (it);

      /* We moved the window start towards BEGV, so PT may be now
	 in the scroll margin at the bottom.  */
      move_it_to (&raw mut it, PT! (), -1,
		  /* We subtract WINDOW_HEADER_LINE_HEIGHT because
		     it.y is relative to the bottom of the header
		     line, see above.  */
		  (it.last_visible_y - WINDOW_TAB_LINE_HEIGHT! (w)
				     - WINDOW_HEADER_LINE_HEIGHT! (w)
		   - partial_line_height (&raw mut it) - this_scroll_margin - 1),
		  -1,
		  MOVE_TO_POS | MOVE_TO_Y);

      /* Save our position, in case it's correct.  */
      charpos = IT_CHARPOS! (it);
      bytepos = IT_BYTEPOS! (it);

      /* If PT is in the screen line at the last fully visible line,
	 move_it_to will stop at X = 0 in that line, because the
	 required Y coordinate is reached there.  See if we can get to
	 PT without descending lower in Y, and if we can, it means we
	 reached PT before the scroll margin.  */
      if charpos != PT! ()
	{
	  let mut it2: it = std::mem::zeroed ();
	  let mut it_data: *mut c_void;

	  it2 = it;
	  it_data = bidi_shelve_cache ();
	  move_it_to (&raw mut it, PT! (), -1, -1, -1, MOVE_TO_POS);
	  if IT_CHARPOS! (it) == PT! () && it.current_y == it2.current_y
	    {
	      charpos = IT_CHARPOS! (it);
	      bytepos = IT_BYTEPOS! (it);
	      bidi_unshelve_cache (it_data, true);
	    }
	  else
	    {
	      it = it2;
	      bidi_unshelve_cache (it_data, false);
	    }
	}

      /* See if point is on a partially visible line at the end.  */
      if it.what == IT_EOB
	{ partial_p =
	  it.current_y + it.ascent + it.descent
	  > it.last_visible_y - this_scroll_margin
	  - WINDOW_TAB_LINE_HEIGHT! (w) - WINDOW_HEADER_LINE_HEIGHT! (w); }
      else
	{
	  move_it_by_lines (&raw mut it, 1);
	  partial_p =
	    it.current_y
	    > it.last_visible_y - this_scroll_margin
	      - WINDOW_TAB_LINE_HEIGHT! (w) - WINDOW_HEADER_LINE_HEIGHT! (w);
	}

      if charpos == PT! () && !partial_p
          && (NILP (Vscroll_preserve_screen_position)
	      || EQ (Vscroll_preserve_screen_position, Qt ()))
	/* We found PT before we found the display margin, so PT is ok.  */
	{ }
      else if window_scroll_pixel_based_preserve_y >= 0
	{
	  let mut goal_y: c_int = min ((it.last_visible_y - this_scroll_margin - 1) as isize,
				       window_scroll_pixel_based_preserve_y as isize) as c_int;

	  /* Don't let the preserved screen Y coordinate put us inside
	     any of the two margins.  */
	  if goal_y < this_scroll_margin
	    { goal_y = this_scroll_margin; }
	  SET_TEXT_POS_FROM_MARKER! (start, (*w).start);
	  start_display (&raw mut it, w, start);
	  /* It would be wrong to subtract WINDOW_HEADER_LINE_HEIGHT
	     here because we called start_display again and did not
	     alter it.current_y this time.  */
	  move_it_to (&raw mut it, -1, window_scroll_pixel_based_preserve_x,
		      goal_y, -1, MOVE_TO_Y | MOVE_TO_X);
	  SET_PT_BOTH (IT_CHARPOS! (it), IT_BYTEPOS! (it));
	}
      else
	{
	  if partial_p
	    /* The last line was only partially visible, so back up two
	       lines to make sure we're on a fully visible line.  */
	    {
	      move_it_by_lines (&raw mut it, -2);
	      SET_PT_BOTH (IT_CHARPOS! (it), IT_BYTEPOS! (it));
	    }
	  else
	    /* No, the position we saved is OK, so use it.  */
	    { SET_PT_BOTH (charpos, bytepos); }
	}
    }
  bidi_unshelve_cache (itdata, false);

  if adjust_old_pointm
    { Fset_marker ((*w).old_pointm,
		   (if w == XWINDOW (selected_window)
		    { make_fixnum (BUF_PT (XBUFFER ((*w).contents)) as EMACS_INT) }
		    else { Fmarker_position ((*w).pointm) }),
		   (*w).contents); }
}

/* P-132: C's call sites here (window.c 7078, 7956, 7981, 8111) call the *function* FUNCTIONP
   that eval.c defines (lisp.h 4799); this revision's lisp.h has no such macro.  lisp.h 4826's
   `extern void record_unwind_protect_excursion (void);` is declared below as C 4826 has it.  */

unsafe extern "C" {
  /// C: lisp.h 4826: `extern void record_unwind_protect_excursion (void);` (eval.c).
  pub fn record_unwind_protect_excursion ();
}

/* Implementation of window_scroll that works based on screen lines.
   See the comment of window_scroll for parameter descriptions.  */

unsafe fn window_scroll_line_based (window: Lisp_Object, n: c_int, whole: bool, noerror: bool)
{
  let w: *mut window = XWINDOW (window);
  /* Fvertical_motion enters redisplay, which can trigger
     fontification, which in turn can modify buffer text (e.g., if the
     fontification functions replace escape sequences with faces, as
     in `grep-mode-font-lock-keywords').  So we use a marker to record
     the old point position, to prevent crashes in SET_PT_BOTH.  */
  let opoint_marker: Lisp_Object = Fpoint_marker ();
  let mut pos: ptrdiff_t;
  let mut pos_byte: ptrdiff_t;
  let ht: c_int = window_internal_height (w);
  let mut tem: Lisp_Object;
  let mut lose: bool;
  let mut bolp: Lisp_Object;
  let mut startpos: ptrdiff_t = marker_position ((*w).start);
  let mut startbyte: ptrdiff_t = marker_byte_position ((*w).start);
  let mut original_pos: Lisp_Object = Qnil ();
  let adjust_old_pointm: bool = !NILP (Fequal (Fwindow_point (window),
					       Fwindow_old_point (window)));
  let mut n = n;

  /* If scrolling screen-fulls, compute the number of lines to
     scroll from the window's height.  */
  if whole
    {
      let nscls: c_int = sanitize_next_screen_context_lines ();
      n = n * (max (1, (ht - nscls) as isize) as c_int);
    }

  if !NILP (Vscroll_preserve_screen_position)
    {
      if window_scroll_preserve_vpos <= 0
	  || !SYMBOLP (KVAR! (current_kboard, Vlast_command))
	  || NILP (Fget (KVAR! (current_kboard, Vlast_command), Qscroll_command ()))
	{
	  let posit: position
	    = *compute_motion (startpos, startbyte, 0, 0, false,
			       PT! (), ht as EMACS_INT, 0, -1, (*w).hscroll as ptrdiff_t, 0, w as *mut c_void);

	  window_scroll_preserve_vpos = posit.vpos;
	  window_scroll_preserve_hpos = posit.hpos + (*w).hscroll as EMACS_INT;
	}

      original_pos = Fcons (make_fixnum (window_scroll_preserve_hpos),
			    make_fixnum (window_scroll_preserve_vpos));
    }

  XSETFASTINT! (tem, PT! ());
  tem = Fpos_visible_in_window_p (tem, window, Qnil ());

  if NILP (tem)
    {
      Fvertical_motion (make_fixnum ((- (ht / 2)) as EMACS_INT), window, Qnil ());
      startpos = PT! ();
      startbyte = PT_BYTE! ();
    }

  SET_PT_BOTH (startpos, startbyte);
  lose = n < 0 && PT! () == BEGV! ();
  Fvertical_motion (make_fixnum (n as EMACS_INT), window, Qnil ());
  pos = PT! ();
  pos_byte = PT_BYTE! ();
  bolp = Fbolp ();
  SET_PT_BOTH (marker_position (opoint_marker),
	       marker_byte_position (opoint_marker));

  if lose
    {
      if noerror
	{ return; }
      else
	{ xsignal0 (Qbeginning_of_buffer ()); }
    }

  if pos < ZV! ()
    {
      let this_scroll_margin: c_int = window_scroll_margin (w, margin_unit::MARGIN_IN_LINES);

      set_marker_restricted_both ((*w).start, (*w).contents, pos, pos_byte);
      window_bits! (w, start_at_line_beg, !NILP (bolp));
      wset_update_mode_line (w);
      /* Set force_start so that redisplay_window will run
	 the window-scroll-functions.  */
      window_bits! (w, force_start, true);

      if !NILP (Vscroll_preserve_screen_position)
	  && this_scroll_margin == 0
	  && (whole || !EQ (Vscroll_preserve_screen_position, Qt ()))
	{
	  SET_PT_BOTH (pos, pos_byte);
	  Fvertical_motion (original_pos, window, Qnil ());
	}
      /* If we scrolled forward, put point enough lines down
	 that it is outside the scroll margin.  */
      else if n > 0
	{
	  let mut top_margin: c_int;

	  if this_scroll_margin > 0
	    {
	      SET_PT_BOTH (pos, pos_byte);
	      Fvertical_motion (make_fixnum (this_scroll_margin as EMACS_INT), window, Qnil ());
	      top_margin = PT! () as c_int;
	    }
	  else
	    { top_margin = pos as c_int; }

	  if (top_margin as ptrdiff_t) <= marker_position (opoint_marker)
	    { SET_PT_BOTH (marker_position (opoint_marker),
			   marker_byte_position (opoint_marker)); }
	  else if !NILP (Vscroll_preserve_screen_position)
	    {
	      let mut nlines: c_int = window_scroll_preserve_vpos as c_int;

	      SET_PT_BOTH (pos, pos_byte);
	      if window_scroll_preserve_vpos < this_scroll_margin as EMACS_INT
		{ nlines = this_scroll_margin; }
	      else if window_scroll_preserve_vpos
		       >= ((*w).total_lines - this_scroll_margin) as EMACS_INT
		{ nlines = (*w).total_lines - this_scroll_margin - 1; }
	      Fvertical_motion (Fcons (make_fixnum (window_scroll_preserve_hpos),
				       make_fixnum (nlines as EMACS_INT)), window, Qnil ());
	    }
	  else
	    { SET_PT (top_margin as ptrdiff_t); }
	}
      else if n < 0
	{
	  let mut bottom_margin: c_int;

	  /* If we scrolled backward, put point near the end of the window
	     but not within the scroll margin.  */
	  SET_PT_BOTH (pos, pos_byte);
	  tem = Fvertical_motion (make_fixnum ((ht - this_scroll_margin) as EMACS_INT), window,
				  Qnil ());
	  if XFIXNAT (tem) == (ht - this_scroll_margin) as EMACS_INT
	    { bottom_margin = PT! () as c_int; }
	  else
	    { bottom_margin = PT! () as c_int + 1; }

	  if (bottom_margin as ptrdiff_t) > marker_position (opoint_marker)
	    { SET_PT_BOTH (marker_position (opoint_marker),
			   marker_byte_position (opoint_marker)); }
	  else
	    {
	      if !NILP (Vscroll_preserve_screen_position)
		{
		  let mut nlines: c_int = window_scroll_preserve_vpos as c_int;

		  SET_PT_BOTH (pos, pos_byte);
		  if window_scroll_preserve_vpos < this_scroll_margin as EMACS_INT
		    { nlines = this_scroll_margin; }
		  else if window_scroll_preserve_vpos
			   >= (ht - this_scroll_margin) as EMACS_INT
		    { nlines = ht - this_scroll_margin - 1; }
		  Fvertical_motion (Fcons (make_fixnum (window_scroll_preserve_hpos),
					   make_fixnum (nlines as EMACS_INT)), window, Qnil ());
		}
	      else
		{ Fvertical_motion (make_fixnum (-1), window, Qnil ()); }
	    }
	}
    }
  else
    {
      if noerror
	{ return; }
      else
	{ xsignal0 (Qend_of_buffer ()); }
    }

  if adjust_old_pointm
    { Fset_marker ((*w).old_pointm,
		   (if w == XWINDOW (selected_window)
		    { make_fixnum (BUF_PT (XBUFFER ((*w).contents)) as EMACS_INT) }
		    else { Fmarker_position ((*w).pointm) }),
		   (*w).contents); }
}


/* Scroll WINDOW up or down.  If N is nil, scroll upward by a
   screen-full which is defined as the height of the window minus
   next_screen_context_lines.  If N is the symbol `-', scroll downward
   by a screen-full.  DIRECTION may be 1 meaning to scroll down, or -1
   meaning to scroll up.  */

unsafe fn scroll_command (window: Lisp_Object, n: Lisp_Object, direction: c_int)
{
  let w: *mut window;
  let other_window: bool;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut n = n;

  eassert! (eabs! (direction) == 1);

  w = XWINDOW (window);
  other_window = !EQ (window, selected_window);

  /* If given window's buffer isn't current, make it current for the
     moment.  If the window's buffer is the same, but it is not the
     selected window, we need to save-excursion to avoid affecting
     point in the selected window (which would cause the selected
     window to scroll).  Don't screw up if window_scroll gets an
     error.  */
  if other_window || XBUFFER ((*w).contents) != (current_buffer! ())
    {
      record_unwind_protect_excursion ();
      if XBUFFER ((*w).contents) != (current_buffer! ())
	{ Fset_buffer ((*w).contents); }
    }

  if other_window
    {
      SET_PT_BOTH (marker_position ((*w).pointm),
		   marker_byte_position ((*w).pointm));
      SET_PT_BOTH (marker_position ((*w).old_pointm),
		   marker_byte_position ((*w).old_pointm));
    }

  if NILP (n)
    { window_scroll (window, direction as EMACS_INT, true, false); }
  else if EQ (n, Qminus ())
    { window_scroll (window, (- direction) as EMACS_INT, true, false); }
  else
    {
      n = Fprefix_numeric_value (n);
      window_scroll (window, XFIXNUM (n) * direction as EMACS_INT, false, false);
    }

  if other_window
    {
      set_marker_both ((*w).pointm, Qnil (), PT! (), PT_BYTE! ());
      set_marker_both ((*w).old_pointm, Qnil (), PT! (), PT_BYTE! ());
    }

  unbind_to (count, Qnil ());
}

DEFUN! ("scroll-up", Fscroll_up, Sscroll_up, 0, 1, "^P",
       doc: "Scroll text of selected window upward ARG lines.\nIf ARG is omitted or nil, scroll upward by a near full screen.\nA near full screen is `next-screen-context-lines' less than a full screen.\nNegative ARG means scroll downward.\nIf ARG is the atom `-', scroll downward by nearly full screen.\nWhen calling from a program, supply as argument a number, nil, or `-'.",
       (arg: Lisp_Object),
       {
  scroll_command (selected_window, arg, 1);
  return Qnil ();
});

DEFUN! ("scroll-down", Fscroll_down, Sscroll_down, 0, 1, "^P",
       doc: "Scroll text of selected window down ARG lines.\nIf ARG is omitted or nil, scroll down by a near full screen.\nA near full screen is `next-screen-context-lines' less than a full screen.\nNegative ARG means scroll upward.\nIf ARG is the atom `-', scroll upward by nearly full screen.\nWhen calling from a program, supply as argument a number, nil, or `-'.",
       (arg: Lisp_Object),
       {
  scroll_command (selected_window, arg, -1);
  return Qnil ();
});

DEFUN! ("other-window-for-scrolling", Fother_window_for_scrolling, Sother_window_for_scrolling, 0, 0, 0,
       doc: "Return \"the other\" window for \"other window scroll\" commands.\nIf in the minibuffer, and `minibuffer-scroll-window' is non-nil,\nit specifies the window to use.\nOtherwise, if `other-window-scroll-buffer' is a buffer, a window\nshowing that buffer is the window to use, popping it up if necessary.\nOtherwise, if `other-window-scroll-default' is a function, call it,\nand the window it returns is the window to use.\nFinally, the function looks for a neighboring window on the selected\nframe, followed by windows on all the visible frames on the current\nterminal.",
       (),
       {
  let mut window: Lisp_Object;

  if MINI_WINDOW_P (XWINDOW (selected_window))
      && !NILP (Vminibuf_scroll_window)
    { window = Vminibuf_scroll_window; }
  /* If buffer is specified and live, scroll that buffer.  */
  else if BUFFERP (Vother_window_scroll_buffer)
	   && BUFFER_LIVE_P (XBUFFER (Vother_window_scroll_buffer))
    {
      window = Fget_buffer_window (Vother_window_scroll_buffer, Qnil ());
      if NILP (window)
	{ window = display_buffer (Vother_window_scroll_buffer, Qt (), Qnil ()); }
    }
  else if FUNCTIONP (Vother_window_scroll_default)
    /* Nothing specified; try to get a window from the function.  */
    { window = call0 (Vother_window_scroll_default); }
  else
    {
      /* Otherwise, look for a neighboring window on the same frame.  */
      window = Fnext_window (selected_window, Qlambda (), Qnil ());

      if EQ (window, selected_window)
	/* That didn't get us anywhere; look for a window on another
           visible frame on the current terminal.  */
        { window = Fnext_window (window, Qlambda (), Qvisible ()); }
    }

  CHECK_LIVE_WINDOW (window);

  if EQ (window, selected_window)
    { error (c"There is no other window".as_ptr ()); }

  return window;
});


DEFUN! ("scroll-left", Fscroll_left, Sscroll_left, 0, 2, "^P\np",
       doc: "Scroll selected window display ARG columns left.\nDefault for ARG is window width minus 2.\nValue is the total amount of leftward horizontal scrolling in\neffect after the change.\nIf SET-MINIMUM is non-nil, the new scroll amount becomes the\nlower bound for automatic scrolling, i.e. automatic scrolling\nwill not scroll a window to a column less than the value returned\nby this function.  This happens in an interactive call.",
       (arg: Lisp_Object, set_minimum: Lisp_Object),
       {
  let w: *mut window = XWINDOW (selected_window);
  let requested_arg: EMACS_INT =
    (if NILP (arg)
     { (window_body_width (w, window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS) - 2) as EMACS_INT }
     else { XFIXNUM (Fprefix_numeric_value (arg)) });
  let result: Lisp_Object = set_window_hscroll (w, (*w).hscroll as EMACS_INT + requested_arg);

  if !NILP (set_minimum)
    { (*w).min_hscroll = (*w).hscroll; }

  window_bits! (w, suspend_auto_hscroll, true);

  return result;
});

DEFUN! ("scroll-right", Fscroll_right, Sscroll_right, 0, 2, "^P\np",
       doc: "Scroll selected window display ARG columns right.\nDefault for ARG is window width minus 2.\nValue is the total amount of leftward horizontal scrolling in\neffect after the change.\nIf SET-MINIMUM is non-nil, the new scroll amount becomes the\nlower bound for automatic scrolling, i.e. automatic scrolling\nwill not scroll a window to a column less than the value returned\nby this function.  This happens in an interactive call.",
       (arg: Lisp_Object, set_minimum: Lisp_Object),
       {
  let w: *mut window = XWINDOW (selected_window);
  let requested_arg: EMACS_INT =
    (if NILP (arg)
     { (window_body_width (w, window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS) - 2) as EMACS_INT }
     else { XFIXNUM (Fprefix_numeric_value (arg)) });
  let result: Lisp_Object = set_window_hscroll (w, (*w).hscroll as EMACS_INT - requested_arg);

  if !NILP (set_minimum)
    { (*w).min_hscroll = (*w).hscroll; }

  window_bits! (w, suspend_auto_hscroll, true);

  return result;
});

DEFUN! ("minibuffer-selected-window", Fminibuffer_selected_window, Sminibuffer_selected_window, 0, 0, 0,
       doc: "Return window selected just before minibuffer window was selected.\nReturn nil if the selected window is not a minibuffer window.",
       (),
       {
  if minibuf_level > 0
      && MINI_WINDOW_P (XWINDOW (selected_window))
      && WINDOW_LIVE_P (minibuf_selected_window)
    { return minibuf_selected_window; }

  return Qnil ();
});

/* Value is the number of lines actually displayed in window W,
   as opposed to its height.  */

unsafe fn displayed_window_lines (w: *mut window) -> c_int
{
  let mut it: it = std::mem::zeroed ();
  let mut start: text_pos = std::mem::zeroed ();
  let mut height: c_int = window_box_height (w);
  let mut old_buffer: *mut buffer;
  let bottom_y: c_int;
  let mut itdata: *mut c_void = std::ptr::null_mut ();

  if XBUFFER ((*w).contents) != (current_buffer! ())
    {
      old_buffer = (current_buffer! ());
      set_buffer_internal (XBUFFER ((*w).contents));
    }
  else
    { old_buffer = std::ptr::null_mut (); }

  /* In case W->start is out of the accessible range, do something
     reasonable.  This happens in Info mode when Info-scroll-down
     calls (recenter -1) while W->start is 1.  */
  CLIP_TEXT_POS_FROM_MARKER! (start, (*w).start);

  itdata = bidi_shelve_cache ();

  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_void (Some (unwind_display_working_on_window));
  display_working_on_window_p = true;
  start_display (&raw mut it, w, start);
  move_it_vertically (&raw mut it, height);
  bottom_y = line_bottom_y (&raw mut it);
  unbind_to (count, Qnil ());
  bidi_unshelve_cache (itdata, false);

  /* Add in empty lines at the bottom of the window.  */
  if bottom_y < height
    {
      let uy: c_int = FRAME_LINE_HEIGHT (it.f);
      it.vpos += (height - bottom_y + uy - 1) / uy;
    }
  else if bottom_y == height
    { it.vpos += 1; }

  if !old_buffer.is_null ()
    { set_buffer_internal (old_buffer); }

  return it.vpos;
}


DEFUN! ("recenter", Frecenter, Srecenter, 0, 2, "P\np",
       doc: "Center point in selected window and maybe redisplay frame.\nWith a numeric prefix argument ARG, recenter putting point on screen line ARG\nrelative to the selected window.  If ARG is negative, it counts up from the\nbottom of the window.  (ARG should be less than the height of the window.)\n\nIf ARG is omitted or nil, then recenter with point on the middle line\nof the selected window; if REDISPLAY & `recenter-redisplay' are\nnon-nil, also erase the entire frame and redraw it (when\n`auto-resize-tool-bars' is set to `grow-only', this resets the\ntool-bar's height to the minimum height needed); if\n`recenter-redisplay' has the special value `tty', then only tty frames\nare redrawn.  Interactively, REDISPLAY is always non-nil.\n\nJust C-u as prefix means put point in the center of the window\nand redisplay normally--don't erase and redraw the frame.",
       (arg: Lisp_Object, redisplay: Lisp_Object),
       {
  let w: *mut window = XWINDOW (selected_window);
  let buf: *mut buffer = XBUFFER ((*w).contents);
  let mut center_p: bool = false;
  let mut charpos: ptrdiff_t;
  let mut bytepos: ptrdiff_t;
  let mut iarg: EMACS_INT = 0;
  let this_scroll_margin: c_int;
  let mut arg = arg;

  /* For reasons why we signal an error here, see
     https://lists.gnu.org/r/emacs-devel/2014-06/msg00053.html,
     https://lists.gnu.org/r/emacs-devel/2014-06/msg00094.html.  */
  if buf != (current_buffer! ())
    { error (c"`recenter'ing a window that does not display current-buffer".as_ptr ()); }

  /* If redisplay is suppressed due to an error, try again.  */
  (*buf).display_error_modiff = 0;

  if NILP (arg)
    {
      if !NILP (redisplay)
	  && !NILP (Vrecenter_redisplay)
	  && (!EQ (Vrecenter_redisplay, Qtty ())
	      || !NILP (Ftty_type (selected_frame)))
	{
	  let mut i: ptrdiff_t;

	  /* Invalidate pixel data calculated for all compositions.  */
	  i = 0;
	  while i < n_compositions
	    {
	      (**composition_table.add (i as usize)).font = std::ptr::null_mut ();
	      i += 1;
	    }
	  Fredraw_frame (WINDOW_FRAME! (w));
	  SET_FRAME_GARBAGED! (WINDOW_XFRAME! (w));
	}

      center_p = true;
    }
  else if CONSP (arg) /* Just C-u.  */
    { center_p = true; }
  else
    {
      arg = Fprefix_numeric_value (arg);
      CHECK_FIXNUM (arg);
      iarg = XFIXNUM (arg);
    }

  /* Do this after making BUF current
     in case scroll_margin is buffer-local.  */
  this_scroll_margin = window_scroll_margin (w, margin_unit::MARGIN_IN_LINES);

  /* Don't use the display code for initial frames, as the necessary
     data structures might not be set up yet then.  Also don't use it
     for buffers with very long lines, as it tremdously slows down
     redisplay, especially when lines are truncated.  */
  if !FRAME_INITIAL_P (XFRAME ((*w).frame))
      && (*(current_buffer! ())).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P == 0
    {
      let count: specpdl_ref = SPECPDL_INDEX ();

      record_unwind_protect_void (Some (unwind_display_working_on_window));
      display_working_on_window_p = true;
      if center_p
	{
	  let mut it: it = std::mem::zeroed ();
	  let mut pt: text_pos = std::mem::zeroed ();
	  let itdata: *mut c_void = bidi_shelve_cache ();

	  SET_TEXT_POS! (pt, PT! (), PT_BYTE! ());
	  start_display (&raw mut it, w, pt);
	  move_it_vertically_backward (&raw mut it, window_box_height (w) / 2);
	  charpos = IT_CHARPOS! (it);
	  bytepos = IT_BYTEPOS! (it);
	  bidi_unshelve_cache (itdata, false);
	}
      else if iarg < 0
	{
	  let mut it: it = std::mem::zeroed ();
	  let mut pt: text_pos = std::mem::zeroed ();
	  let mut nlines: ptrdiff_t = min (PTRDIFF_MAX, -iarg);
	  let mut extra_line_spacing: c_int;
	  let mut h: c_int = window_box_height (w);
	  let ht: c_int = window_internal_height (w);
	  let itdata: *mut c_void = bidi_shelve_cache ();

	  nlines = clip_to_bounds ((this_scroll_margin + 1) as EMACS_INT, nlines,
				   (ht - this_scroll_margin) as EMACS_INT);

	  SET_TEXT_POS! (pt, PT! (), PT_BYTE! ());
	  start_display (&raw mut it, w, pt);

	  /* Be sure we have the exact height of the full line containing PT.  */
	  move_it_by_lines (&raw mut it, 0);

	  /* The amount of pixels we have to move back is the window
	     height minus what's displayed in the line containing PT,
	     and the lines below.  */
	  it.current_y = 0;
	  it.vpos = 0;
	  move_it_by_lines (&raw mut it, nlines);

	  if it.vpos as ptrdiff_t == nlines
	    { h -= it.current_y; }
	  else
	    {
	      /* Last line has no newline.  */
	      h -= line_bottom_y (&raw mut it);
	      it.vpos += 1;
	    }

	  /* Don't reserve space for extra line spacing of last line.  */
	  extra_line_spacing = it.max_extra_line_spacing;

	  /* If we can't move down NLINES lines because we hit
	     the end of the buffer, count in some empty lines.  */
	  if (it.vpos as ptrdiff_t) < nlines
	    {
	      nlines -= it.vpos as ptrdiff_t;
	      extra_line_spacing = it.extra_line_spacing;
	      h -= (nlines as c_int) * (FRAME_LINE_HEIGHT (it.f) + extra_line_spacing);
	    }
	  if h <= 0
	    {
	      bidi_unshelve_cache (itdata, false);
	      unbind_to (count, Qnil ());
	      return Qnil ();
	    }

	  /* Now find the new top line (starting position) of the window.  */
	  start_display (&raw mut it, w, pt);
	  it.current_y = 0;
	  move_it_vertically_backward (&raw mut it, h);

	  /* If extra line spacing is present, we may move too far
	     back.  This causes the last line to be only partially
	     visible (which triggers redisplay to recenter that line
	     in the middle), so move forward.
	     But ignore extra line spacing on last line, as it is not
	     considered to be part of the visible height of the line.
	  */
	  h += extra_line_spacing;
	  while - it.current_y > h && it.what != IT_EOB
	    { move_it_by_lines (&raw mut it, 1); }

	  charpos = IT_CHARPOS! (it);
	  bytepos = IT_BYTEPOS! (it);

	  bidi_unshelve_cache (itdata, false);
	}
      else
	{
	  let mut it: it = std::mem::zeroed ();
	  let mut pt: text_pos = std::mem::zeroed ();
	  let mut nlines: ptrdiff_t = min (PTRDIFF_MAX, iarg);
	  let ht: c_int = window_internal_height (w);
	  let itdata: *mut c_void = bidi_shelve_cache ();

	  nlines = clip_to_bounds (this_scroll_margin as EMACS_INT, nlines,
				   (ht - this_scroll_margin - 1) as EMACS_INT);

	  SET_TEXT_POS! (pt, PT! (), PT_BYTE! ());
	  start_display (&raw mut it, w, pt);

	  /* Move to the beginning of screen line containing PT.  */
	  move_it_by_lines (&raw mut it, 0);

	  /* Move back to find the point which is ARG screen lines above PT.  */
	  if nlines > 0
	    {
	      it.current_y = 0;
	      it.vpos = 0;
	      move_it_by_lines (&raw mut it, -nlines);
	    }

	  charpos = IT_CHARPOS! (it);
	  bytepos = IT_BYTEPOS! (it);

	  bidi_unshelve_cache (itdata, false);
	}
      unbind_to (count, Qnil ());
    }
  else
    {
      let mut pos: position;
      let ht: c_int = window_internal_height (w);

      if center_p
	{ iarg = (ht / 2) as EMACS_INT; }
      else if iarg < 0
	{ iarg += ht as EMACS_INT; }

      /* Don't let it get into the margin at either top or bottom.  */
      iarg = clip_to_bounds (this_scroll_margin as EMACS_INT, iarg,
			     (ht - this_scroll_margin - 1) as EMACS_INT);

      pos = *vmotion (PT! (), PT_BYTE! (), - iarg, w as *mut c_void);
      charpos = pos.bufpos;
      bytepos = pos.bytepos;
    }

  /* Set the new window start.  */
  set_marker_both ((*w).start, (*w).contents, charpos, bytepos);

  /* The window start was calculated with an iterator already adjusted
     by the existing vscroll, so w->start must not be combined with
     retaining the existing vscroll, which redisplay will not reset if
     w->preserve_vscroll_p is enabled.  (bug#70386) */
  (*w).vscroll = 0;
  window_bits! (w, preserve_vscroll_p, false);
  window_bits! (w, window_end_valid, false);
  window_bits! (w, optional_new_start, true);

  window_bits! (w, start_at_line_beg,
		bytepos == BEGV_BYTE! ()
		  || FETCH_BYTE! (bytepos - 1) as c_int == b'\n' as c_int);

  wset_redisplay (w);

  return Qnil ();
});

DEFUN! ("window-text-width", Fwindow_text_width, Swindow_text_width,
       0, 2, 0,
       doc: "Return the width in columns of the text display area of WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nThe returned width does not include dividers, scrollbars, margins,\nfringes, nor any partial-width columns at the right of the text\narea.\n\nOptional argument PIXELWISE non-nil, means to return the width in\npixels.",
       (window: Lisp_Object, pixelwise: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);

  if NILP (pixelwise)
    { return make_fixnum ((window_box_width (w, TEXT_AREA)
			   / FRAME_COLUMN_WIDTH (WINDOW_XFRAME! (w))) as EMACS_INT); }
  else
    { return make_fixnum (window_box_width (w, TEXT_AREA) as EMACS_INT); }
});

DEFUN! ("window-text-height", Fwindow_text_height, Swindow_text_height,
       0, 2, 0,
       doc: "Return the height in lines of the text display area of WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nThe returned height does not include dividers, the mode line, any header\nline, nor any partial-height lines at the bottom of the text area.\n\nOptional argument PIXELWISE non-nil, means to return the height in\npixels.",
       (window: Lisp_Object, pixelwise: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);

  if NILP (pixelwise)
    { return make_fixnum ((window_box_height (w)
			   / FRAME_LINE_HEIGHT (WINDOW_XFRAME! (w))) as EMACS_INT); }
  else
    { return make_fixnum (window_box_height (w) as EMACS_INT); }
});

DEFUN! ("move-to-window-line", Fmove_to_window_line, Smove_to_window_line,
       1, 1, "P",
       doc: "Position point relative to window.\nARG nil means position point at center of window.\nElse, ARG specifies vertical position within the window;\nzero means top of window, negative means relative to bottom\nof window, -1 meaning the last fully visible display line\nof the window.\n\nValue is the screen line of the window point moved to, counting\nfrom the top of the window.",
       (arg: Lisp_Object),
       {
  let w: *mut window = XWINDOW (selected_window);
  let mut lines: c_int;
  let mut start: c_int;
  let mut window: Lisp_Object;
  let mut arg = arg;

  if !(BUFFERP ((*w).contents) && XBUFFER ((*w).contents) == (current_buffer! ()))
    /* This test is needed to make sure PT/PT_BYTE make sense in w->contents
       when passed below to set_marker_both.  */
    { error (c"move-to-window-line called from unrelated buffer".as_ptr ()); }

  window = selected_window;
  start = marker_position ((*w).start) as c_int;
  if (start as ptrdiff_t) < BEGV! () || (start as ptrdiff_t) > ZV! ()
    {
      let height: c_int = window_internal_height (w);
      Fvertical_motion (make_fixnum ((- (height / 2)) as EMACS_INT), window, Qnil ());
      set_marker_both ((*w).start, (*w).contents, PT! (), PT_BYTE! ());
      window_bits! (w, start_at_line_beg, !NILP (Fbolp ()));
      window_bits! (w, force_start, true);

      /* Since `Fvertical_motion' computes coordinates after vscroll is
         applied, it is taken into account in POS, and vscroll must be
         reset by `force_start' in `redisplay_internal'.  */
      window_bits! (w, preserve_vscroll_p, false);
    }
  else
    { Fgoto_char ((*w).start); }

  lines = displayed_window_lines (w);

  if NILP (arg)
    { XSETFASTINT! (arg, (lines / 2) as EMACS_INT); }
  else
    {
      let mut iarg: EMACS_INT = XFIXNUM (Fprefix_numeric_value (arg));

      if iarg < 0
	{ iarg = iarg + lines as EMACS_INT; }

      arg = make_fixnum (iarg);
    }

  /* Skip past a partially visible first line.  */
  if (*w).vscroll != 0
    { XSETINT! (arg, XFIXNUM (arg) + 1); }

  return Fvertical_motion (arg, window, Qnil ());
});


/***********************************************************************
			 Window Configuration
 ***********************************************************************/

#[repr(C, align(8))]
pub struct save_window_data
  {
    pub header: vectorlike_header,
    pub selected_frame: Lisp_Object,
    pub current_window: Lisp_Object,
    pub f_current_buffer: Lisp_Object,
    pub minibuf_scroll_window: Lisp_Object,
    pub minibuf_selected_window: Lisp_Object,
    pub root_window: Lisp_Object,
    pub focus_frame: Lisp_Object,
    /* A vector, each of whose elements is a struct saved_window
       for one window.  */
    pub saved_windows: Lisp_Object,

    /* All fields above are traced by the GC.
       After saved_windows, the fields are ignored by the GC.  */
    /* The change stamp of the frame at the time of saving.  */
    pub change_stamp: c_int,
    /* We should be able to do without the following two.  */
    pub frame_cols: c_int ,
    pub frame_lines: c_int,
    /* These three should get eventually replaced by their pixel
       counterparts.  */
    pub frame_menu_bar_lines: c_int ,
    pub frame_tab_bar_lines: c_int,
    pub frame_tool_bar_lines: c_int,
    pub frame_text_width: c_int ,
    pub frame_text_height: c_int,
    /* These are currently unused.  We need them as soon as we convert
       to pixels.  */
    pub frame_menu_bar_height: c_int ,
    pub frame_tab_bar_height: c_int,
    pub frame_tool_bar_height: c_int,
  }

/* This is saved as a Lisp_Vector.  */
#[repr(C, align(8))]
pub struct saved_window
{
  pub header: vectorlike_header,

  pub window: Lisp_Object,
  pub buffer: Lisp_Object,
  pub old_buffer: Lisp_Object,
  pub start: Lisp_Object,
  pub pointm: Lisp_Object,
  pub old_pointm: Lisp_Object,
  pub pixel_left: Lisp_Object,
  pub pixel_top: Lisp_Object,
  pub pixel_height: Lisp_Object,
  pub pixel_width: Lisp_Object,
  pub left_col: Lisp_Object,
  pub top_line: Lisp_Object,
  pub total_cols: Lisp_Object,
  pub total_lines: Lisp_Object,
  pub normal_cols: Lisp_Object,
  pub normal_lines: Lisp_Object,
  pub hscroll: Lisp_Object,
  pub min_hscroll: Lisp_Object,
  pub hscroll_whole: Lisp_Object,
  pub suspend_auto_hscroll: Lisp_Object,
  pub vscroll: Lisp_Object,
  pub parent: Lisp_Object,
  pub prev: Lisp_Object,
  pub start_at_line_beg: Lisp_Object,
  pub display_table: Lisp_Object,
  pub left_margin_cols: Lisp_Object,
  pub right_margin_cols: Lisp_Object,
  pub left_fringe_width: Lisp_Object,
  pub right_fringe_width: Lisp_Object,
  pub fringes_outside_margins: Lisp_Object,
  pub fringes_persistent: Lisp_Object,
  pub scroll_bar_width: Lisp_Object,
  pub vertical_scroll_bar_type: Lisp_Object,
  pub scroll_bar_height: Lisp_Object,
  pub horizontal_scroll_bar_type: Lisp_Object,
  pub scroll_bars_persistent: Lisp_Object,
  pub dedicated: Lisp_Object,
  pub combination_limit: Lisp_Object,
  pub window_parameters: Lisp_Object,
}

/* `SAVED_WINDOW_N` (C 7637-7638) reaches the N-th element of the vector SWV's `contents`
   flexmember; `Lisp_Vector.contents` is `[Lisp_Object; 0]` in the mirror, so the element is a
   pointer step (PORTING.md 1.3's flexmember rule).  */
macro_rules! SAVED_WINDOW_N {
  ($swv:expr, $n:expr) => {
    XVECTOR (*(*$swv).contents.as_ptr ().add ($n as usize)) as *mut saved_window
  };
}

DEFUN! ("window-configuration-p", Fwindow_configuration_p, Swindow_configuration_p, 1, 1, 0,
       doc: "Return t if OBJECT is a window-configuration object.",
       (object: Lisp_Object),
       {
  return if WINDOW_CONFIGURATIONP (object) { Qt () } else { Qnil () };
});

DEFUN! ("window-configuration-frame", Fwindow_configuration_frame, Swindow_configuration_frame, 1, 1, 0,
       doc: "Return the frame that CONFIG, a window-configuration object, is about.",
       (config: Lisp_Object),
       {
  let data: *mut save_window_data;
  let saved_windows: *mut Lisp_Vector;

  CHECK_WINDOW_CONFIGURATION (config);

  data = XVECTOR (config) as *mut save_window_data;
  saved_windows = XVECTOR ((*data).saved_windows);
  return (*XWINDOW ((*SAVED_WINDOW_N! (saved_windows, 0)).window)).frame;
});

/* C's `list(...)` is `listn (countof (...), __VA_ARGS__)` (lisp.h 4493-4494) and the mirror
   carries it (`include/lisp.rs` 3227, same double evaluation of __VA_ARGS__); the unit's copy
   was redundancy (P-26).  */

unsafe extern "C" {
  /// C: frame.h 1511-1512: `extern void adjust_frame_size (struct frame *, int, int, int,
  /// bool, Lisp_Object);` (frame.c; the mirror does not carry the prototype).
  pub fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
                            inhibit: c_int, pretend: bool, parameter: Lisp_Object);
  /// C: lisp.h 5119: `extern Lisp_Object do_switch_frame (Lisp_Object, int, int,
  /// Lisp_Object);` (frame.c).
  pub fn do_switch_frame (frame: Lisp_Object, track: c_int, for_deletion: c_int,
                          norecord: Lisp_Object) -> Lisp_Object;
}

DEFUN! ("set-window-configuration", Fset_window_configuration,
       Sset_window_configuration, 1, 3, 0,
       doc: "Set the configuration of windows and buffers as specified by CONFIGURATION.\nCONFIGURATION must be a value previously returned\nby `current-window-configuration' (which see).\n\nNormally, this function selects the frame of the CONFIGURATION, but if\nDONT-SET-FRAME is non-nil, it leaves selected the frame which was\ncurrent at the start of the function.  If DONT-SET-MINIWINDOW is non-nil,\nthe mini-window of the frame doesn't get set to the corresponding element\nof CONFIGURATION.\n\nThis function consults the variable `window-restore-killed-buffer-windows'\nwhen restoring a window whose buffer was killed after CONFIGURATION was\nrecorded.\n\nIf CONFIGURATION was made from a frame that is now deleted,\nonly frame-independent values can be restored.  In this case,\nthe return value is nil.  Otherwise the value is t.",
       (configuration: Lisp_Object, dont_set_frame: Lisp_Object,
        dont_set_miniwindow: Lisp_Object),
       {
  let data: *mut save_window_data;
  let saved_windows: *mut Lisp_Vector;
  let mut new_current_buffer: Lisp_Object;
  let frame: Lisp_Object;
  let mut kept_windows: Lisp_Object = Qnil ();
  let old_frame: Lisp_Object = selected_frame;
  let f: *mut frame;
  let mut old_point: ptrdiff_t = -1;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  CHECK_WINDOW_CONFIGURATION (configuration);

  data = XVECTOR (configuration) as *mut save_window_data;
  saved_windows = XVECTOR ((*data).saved_windows);

  new_current_buffer = (*data).f_current_buffer;
  if !BUFFER_LIVE_P (XBUFFER (new_current_buffer))
    { new_current_buffer = Qnil (); }
  else
    {
      if XBUFFER (new_current_buffer) == (current_buffer! ())
	/* The code further down "preserves point" by saving here PT in
	   old_point and then setting it later back into PT.  When the
	   current-selected-window and the final-selected-window both show
	   the current buffer, this suffers from the problem that the
	   current PT is the window-point of the current-selected-window,
	   while the final PT is the point of the final-selected-window, so
	   this copy from one PT to the other would end up moving the
	   window-point of the final-selected-window to the window-point of
	   the current-selected-window.  So we have to be careful which
	   point of the current-buffer we copy into old_point.  */
	{
	if EQ ((*XWINDOW ((*data).current_window)).contents, new_current_buffer)
	    && WINDOWP (selected_window)
	    && EQ ((*XWINDOW (selected_window)).contents, new_current_buffer)
	    && !EQ (selected_window, (*data).current_window)
	  { old_point = marker_position ((*XWINDOW ((*data).current_window)).pointm); }
	else
	  { old_point = PT! (); }
	}
      else
	/* BUF_PT (XBUFFER (new_current_buffer)) gives us the position of
	   point in new_current_buffer as of the last time this buffer was
	   used.  This can be non-deterministic since it can be changed by
	   things like jit-lock by mere temporary selection of some random
	   window that happens to show this buffer.
	   So if possible we want this arbitrary choice of "which point" to
	   be the one from the to-be-selected-window so as to prevent this
	   window's cursor from being copied from another window.  */
	{
	if EQ ((*XWINDOW ((*data).current_window)).contents, new_current_buffer)
	    /* If current_window = selected_window, its point is in BUF_PT.  */
	    && !EQ (selected_window, (*data).current_window)
	  { old_point = marker_position ((*XWINDOW ((*data).current_window)).pointm); }
	else
	  { old_point = BUF_PT (XBUFFER (new_current_buffer)); }
	}
    }

  frame = (*XWINDOW ((*SAVED_WINDOW_N! (saved_windows, 0)).window)).frame;
  f = XFRAME (frame);

  /* If f is a dead frame, don't bother rebuilding its window tree.
     However, there is other stuff we should still try to do below.  */
  if FRAME_LIVE_P (f)
    {
      let mut window: Lisp_Object;
      let mut dead_windows: Lisp_Object = Qnil ();
      let mut tem: Lisp_Object;
      let mut par: Lisp_Object;
      let mut pers: Lisp_Object;
      let mut w: *mut window;
      let mut p: *mut saved_window;
      let root_window: *mut window;
      let mut leaf_windows: *mut *mut window = std::ptr::null_mut ();
      let mut i: ptrdiff_t;
      let mut k: ptrdiff_t;
      let n_leaf_windows: ptrdiff_t;

      /* Don't do this within the main loop below: This may call Lisp
	 code and is thus potentially unsafe while input is blocked.  */
      k = 0;
      while k < (*saved_windows).header.size
	{
	  p = SAVED_WINDOW_N! (saved_windows, k);
	  window = (*p).window;
	  w = XWINDOW (window);

	  if BUFFERP ((*w).contents)
	      && !EQ ((*w).contents, (*p).buffer)
	      && BUFFER_LIVE_P (XBUFFER ((*p).buffer))
	      && (NILP (Fminibufferp ((*p).buffer, Qnil ())))
	    /* If a window we restore gets another buffer, record the
	       window's old buffer.  */
	    { calln! (Qrecord_window_buffer (), window); }
	  k += 1;
	}

      /* Disallow set_window_size_hook, temporarily.  */
      can_set_window_size! (f, false);
      /* The mouse highlighting code could get screwed up
	 if it runs during this.  */
      block_input ();

      /* "Swap out" point from the selected window's buffer
	 into the window itself.  (Normally the pointm of the selected
	 window holds garbage.)  We do this now, before
	 restoring the window contents, and prevent it from
	 being done later on when we select a new window.  */
      if !NILP ((*XWINDOW (selected_window)).contents)
	{
	  w = XWINDOW (selected_window);
	  set_marker_both ((*w).pointm,
			   (*w).contents,
			   BUF_PT (XBUFFER ((*w).contents)),
			   BUF_PT_BYTE (XBUFFER ((*w).contents)));
	}

      fset_redisplay (f as *mut c_void);

      /* Problem: Freeing all matrices and later allocating them again
	 is a serious redisplay flickering problem.  What we would
	 really like to do is to free only those matrices not reused
	 below.  */
      root_window = XWINDOW (FRAME_ROOT_WINDOW (f));
      let nwindows: ptrdiff_t = count_windows (root_window);
      SAFE_NALLOCA! (sa_avail, leaf_windows, 1, nwindows);
      n_leaf_windows = get_leaf_windows (root_window, leaf_windows, 0);

      /* Kludge Alert!
	 Mark all windows now on frame as "deleted".
	 Restoring the new configuration "undeletes" any that are in it.

	 Save their current buffers in their height fields, since we may
	 need it later, if a buffer saved in the configuration is now
	 dead.  */
      delete_all_child_windows (FRAME_ROOT_WINDOW (f));

      k = 0;
      while k < (*saved_windows).header.size
	{
	  p = SAVED_WINDOW_N! (saved_windows, k);
	  window = (*p).window;
	  w = XWINDOW (window);
	  wset_next (w, Qnil ());

	  if !NILP ((*p).parent)
	    { wset_parent
	      (w, (*SAVED_WINDOW_N! (saved_windows, XFIXNAT ((*p).parent))).window); }
	  else
	    { wset_parent (w, Qnil ()); }

	  if !NILP ((*p).prev)
	    {
	      wset_prev
		(w, (*SAVED_WINDOW_N! (saved_windows, XFIXNAT ((*p).prev))).window);
	      wset_next (XWINDOW ((*w).prev), (*p).window);
	    }
	  else
	    {
	      wset_prev (w, Qnil ());
	      if !NILP ((*w).parent)
		{ wset_combination (XWINDOW ((*w).parent),
				  (XFIXNUM ((*p).total_cols)
				   != (*XWINDOW ((*w).parent)).total_cols as EMACS_INT),
				  (*p).window); }
	    }

	  /* If we squirreled away the buffer, restore it now.  */
	  if BUFFERP ((*w).combination_limit)
	    { wset_buffer (w, (*w).combination_limit); }

	  if (*data).change_stamp == (*f).change_stamp
	    /* Restore W's old_buffer slot but only if the configuration
	       was saved and restored in between two redisplay cycles,
	       that is, if F's saved change stamp and its current change
	       stamp are equal.  In that case we should run W's buffer
	       change functions provided the saved old_buffer and the
	       restored buffer differ.  If the saved old_buffer and the
	       restored buffer are one and the same, the window
	       excursion was only temporary and it would be distracting
	       to run the buffer change functions for it.

	       If the change stamps are not equal, run the buffer change
	       functions provided W's current buffer (which was stored
	       by delete_all_child_windows above in W's old_buffer slot)
	       and the buffer that will be restored differ (Bug#81079
	       and Bug#81589).  */
	    { (*w).old_buffer = (*p).old_buffer; }

	  (*w).pixel_left = XFIXNAT ((*p).pixel_left) as c_int;
	  (*w).pixel_top = XFIXNAT ((*p).pixel_top) as c_int;
	  (*w).pixel_width = XFIXNAT ((*p).pixel_width) as c_int;
	  (*w).pixel_height = XFIXNAT ((*p).pixel_height) as c_int;
	  (*w).left_col = XFIXNAT ((*p).left_col) as c_int;
	  (*w).top_line = XFIXNAT ((*p).top_line) as c_int;
	  (*w).total_cols = XFIXNAT ((*p).total_cols) as c_int;
	  (*w).total_lines = XFIXNAT ((*p).total_lines) as c_int;
	  wset_normal_cols (w, (*p).normal_cols);
	  wset_normal_lines (w, (*p).normal_lines);
	  (*w).hscroll = XFIXNAT ((*p).hscroll);
	  window_bits! (w, suspend_auto_hscroll, !NILP ((*p).suspend_auto_hscroll));
	  (*w).min_hscroll = XFIXNAT ((*p).min_hscroll);
	  (*w).hscroll_whole = XFIXNAT ((*p).hscroll_whole);
	  (*w).vscroll = (- XFIXNAT ((*p).vscroll)) as c_int;
	  wset_display_table (w, (*p).display_table);
	  (*w).left_margin_cols = XFIXNUM ((*p).left_margin_cols) as c_int;
	  (*w).right_margin_cols = XFIXNUM ((*p).right_margin_cols) as c_int;
	  (*w).left_fringe_width = XFIXNUM ((*p).left_fringe_width) as c_int;
	  (*w).right_fringe_width = XFIXNUM ((*p).right_fringe_width) as c_int;
	  window_bits! (w, fringes_outside_margins, !NILP ((*p).fringes_outside_margins));
	  window_bits! (w, fringes_persistent, !NILP ((*p).fringes_persistent));
	  (*w).scroll_bar_width = XFIXNUM ((*p).scroll_bar_width) as c_int;
	  (*w).scroll_bar_height = XFIXNUM ((*p).scroll_bar_height) as c_int;
	  window_bits! (w, scroll_bars_persistent, !NILP ((*p).scroll_bars_persistent));
	  wset_vertical_scroll_bar_type (w, (*p).vertical_scroll_bar_type);
	  wset_horizontal_scroll_bar_type (w, (*p).horizontal_scroll_bar_type);
	  wset_dedicated (w, (*p).dedicated);
	  wset_combination_limit (w, (*p).combination_limit);
	  /* Restore any window parameters that have been saved.
	     Parameters that have not been saved are left alone.  */
	  tem = (*p).window_parameters;
	  while CONSP (tem)
	    {
	      pers = XCAR (tem);
	      if CONSP (pers)
		{
		  if NILP (XCDR (pers))
		    {
		      par = Fassq (XCAR (pers), (*w).window_parameters);
		      if CONSP (par) && !NILP (XCDR (par))
			/* Reset a parameter to nil if and only if it
			   has a non-nil association.  Don't make new
			   associations.  */
			{ Fsetcdr (par, Qnil ()); }
		    }
		  else
		    /* Always restore a non-nil value.  */
		    { Fset_window_parameter (window, XCAR (pers), XCDR (pers)); }
		}
	      tem = XCDR (tem);
	    }

	  /* Remove window from the table of dead windows.  */
	  Fremhash (make_fixnum ((*w).sequence_number as EMACS_INT),
		    window_dead_windows_table);

	  if (NILP (dont_set_miniwindow) || !MINI_WINDOW_P (w))
	      && BUFFERP ((*p).buffer) && BUFFER_LIVE_P (XBUFFER ((*p).buffer))
	    /* If saved buffer is alive, install it, unless it's a
	       minibuffer we explicitly prohibit.  */
	    {
	      if !EQ ((*w).contents, (*p).buffer)
		{
		  wset_buffer (w, (*p).buffer);
		  window_discard_buffer_from_window ((*w).contents, window, false);
		}

	      window_bits! (w, start_at_line_beg, !NILP ((*p).start_at_line_beg));
	      set_marker_restricted ((*w).start, (*p).start, (*w).contents);
	      set_marker_restricted ((*w).pointm, (*p).pointm, (*w).contents);
	      set_marker_restricted ((*w).old_pointm, (*p).old_pointm, (*w).contents);
	      /* As documented in Fcurrent_window_configuration, don't
		 restore the location of point in the buffer which was
		 current when the window configuration was recorded.  */
	      if !EQ ((*p).buffer, new_current_buffer)
		  && XBUFFER ((*p).buffer) == (current_buffer! ())
		{ Fgoto_char ((*w).pointm); }
	    }
	  else if BUFFERP ((*w).contents) && BUFFER_LIVE_P (XBUFFER ((*w).contents))
	    /* Keep window's old buffer; make sure the markers are real.  */
	    {
	      /* Set window markers at start of visible range.  */
	      if (*XMARKER ((*w).start)).buffer.is_null ()
		{ set_marker_restricted_both ((*w).start, (*w).contents, 0, 0); }
	      if (*XMARKER ((*w).pointm)).buffer.is_null ()
		{ set_marker_restricted_both
		  ((*w).pointm, (*w).contents,
		   BUF_PT (XBUFFER ((*w).contents)),
		   BUF_PT_BYTE (XBUFFER ((*w).contents))); }
	      if (*XMARKER ((*w).old_pointm)).buffer.is_null ()
		{ set_marker_restricted_both
		  ((*w).old_pointm, (*w).contents,
		   BUF_PT (XBUFFER ((*w).contents)),
		   BUF_PT_BYTE (XBUFFER ((*w).contents))); }
	      window_bits! (w, start_at_line_beg, true);
	      if FUNCTIONP (window_restore_killed_buffer_windows)
		  && !MINI_WINDOW_P (w)
		{ kept_windows = Fcons (list! (window, (*p).buffer,
					    Fmarker_last_position ((*p).start),
					    Fmarker_last_position ((*p).pointm),
					    (*p).dedicated, Qt ()),
				      kept_windows); }
	    }
	  else if !NILP ((*w).start)
	    /* Leaf window has no live buffer, get one.  */
	    {
	      /* Get the buffer via other_buffer_safely in order to
		 avoid showing an unimportant buffer and, if necessary, to
		 recreate *scratch* in the course (part of Juanma's bs-show
		 scenario from March 2011).  */
	      wset_buffer (w, other_buffer_safely (Fcurrent_buffer ()));
	      window_discard_buffer_from_window ((*w).contents, window, false);
	      /* This will set the markers to beginning of visible
		 range.  */
	      set_marker_restricted_both ((*w).start, (*w).contents, 0, 0);
	      set_marker_restricted_both ((*w).pointm, (*w).contents, 0, 0);
	      set_marker_restricted_both ((*w).old_pointm, (*w).contents, 0, 0);
	      window_bits! (w, start_at_line_beg, true);
	      if !MINI_WINDOW_P (w)
		{
		  if FUNCTIONP (window_restore_killed_buffer_windows)
		    { kept_windows
		      = Fcons (list! (window, (*p).buffer,
				     Fmarker_last_position ((*p).start),
				     Fmarker_last_position ((*p).pointm),
				     (*p).dedicated, Qnil ()),
			       kept_windows); }
		  else if EQ (window_restore_killed_buffer_windows, Qdelete ())
			   || (!NILP ((*p).dedicated)
			       && (NILP (window_restore_killed_buffer_windows)
				   || EQ (window_restore_killed_buffer_windows,
					  Qdedicated ())))
		    /* Try to delete this window later.  */
		    { dead_windows = Fcons (window, dead_windows); }
		  /* Make sure window is no more dedicated.  */
		  wset_dedicated (w, Qnil ());
		}
	    }
	  k += 1;
	}

      fset_root_window (f, (*data).root_window);
      /* Arrange *not* to restore point in the buffer that was
	 current when the window configuration was saved.  */
      if EQ ((*XWINDOW ((*data).current_window)).contents, new_current_buffer)
	{ set_marker_restricted ((*XWINDOW ((*data).current_window)).pointm,
			       make_fixnum (old_point),
			       (*XWINDOW ((*data).current_window)).contents); }

      /* In the following call to select_window, prevent "swapping out
	 point" in the old selected window using the buffer that has
	 been restored into it.  We already swapped out that point
	 from that window's old buffer.

	 Do not record the buffer here.  We do that in a separate call
	 to select_window below.  See also Bug#16207.  */
      select_window ((*data).current_window, Qt (), true);
      BVAR! (XBUFFER ((*XWINDOW (selected_window)).contents),
	    last_selected_window_)
	= selected_window;

      /* We may have deleted windows above.  Then again, maybe we
	 haven't: the functions we call to maybe delete windows can
	 decide a window cannot be deleted.  Force recalculation of
	 Vwindow_list next time it is needed, to make sure stale
	 windows with no buffers don't escape into the wild, which
	 will cause crashes elsewhere.  */
      Vwindow_list = Qnil ();

      if NILP ((*data).focus_frame)
	  || (FRAMEP ((*data).focus_frame)
	      && FRAME_LIVE_P (XFRAME ((*data).focus_frame)))
	{
	  let frame_focus_frame: Lisp_Object = (*f).focus_frame;

	  Fredirect_frame_focus (frame, (*data).focus_frame);

	  if EQ (focus_follows_mouse, Qauto_raise ())
	      && NILP ((*data).focus_frame)
	      && EQ (selected_frame, frame_focus_frame)
	    { calln! (Qselect_frame_set_input_focus (), frame, Qnil ()); }
	}

      /* Now, free glyph matrices in windows that were not reused.  */
      i = 0;
      while i < n_leaf_windows
	{
	  if NILP ((**leaf_windows.add (i as usize)).contents)
	    { free_window_matrices (*leaf_windows.add (i as usize)); }
	  i += 1;
	}

      /* Allow set_window_size_hook again and resize frame's windows
	 if necessary.  But change frame size only to preserve window
	 minimum sizes.  */
      can_set_window_size! (f, true);
      adjust_frame_size (f, -1, -1, 4, false, Qset_window_configuration ());

      adjust_frame_glyphs (f);
      unblock_input ();

      /* Scan dead buffer windows.  */
      while CONSP (dead_windows)
	{
	  window = XCAR (dead_windows);
	  if WINDOW_LIVE_P (window) && !EQ (window, FRAME_ROOT_WINDOW (f))
	    { delete_deletable_window (window); }
	  dead_windows = XCDR (dead_windows);
	}

      /* Record the selected window's buffer here.  The window should
	 already be the selected one from the call above.  */
      if WINDOW_LIVE_P ((*data).current_window)
	{ select_window ((*data).current_window, Qnil (), false); }

      /* select_window will have made f the selected frame, so we
	 reselect the proper frame here.  do_switch_frame will change
	 the selected window too, but that doesn't make the call to
	 select_window above totally superfluous; it still sets f's
	 selected window.  */
      if FRAME_LIVE_P (XFRAME ((*data).selected_frame))
	{ do_switch_frame (if NILP (dont_set_frame)
                         { (*data).selected_frame }
                         else { old_frame }
                         , 0, 0, Qnil ()); }
    }

  FRAME_WINDOW_CHANGE! (f, true);

  if !NILP (new_current_buffer)
    {
      Fset_buffer (new_current_buffer);
      /* If the new current buffer doesn't appear in the selected
	 window, go to its old point (Bug#12208).

	 The original fix used data->current_window below which caused
	 false positives (compare Bug#31695) when data->current_window
	 is not on data->selected_frame.  This happens, for example,
	 when read_minibuf restores the configuration of a stand-alone
	 minibuffer frame: After switching to the previously selected
	 "normal" frame, point of that frame's selected window jumped
	 unexpectedly because new_current_buffer is usually *not*
	 shown in data->current_window - the minibuffer frame's
	 selected window.  Using selected_window instead fixes this
	 because do_switch_frame has set up selected_window already to
	 the "normal" frame's selected window and that window *does*
	 show new_current_buffer.  */
      if !EQ ((*XWINDOW (selected_window)).contents, new_current_buffer)
	{ Fgoto_char (make_fixnum (old_point)); }
    }

  Vminibuf_scroll_window = (*data).minibuf_scroll_window;
  minibuf_selected_window = (*data).minibuf_selected_window;

  SAFE_FREE! (sa_count, sa_mark);

  if !NILP (Vrun_hooks) && FUNCTIONP (window_restore_killed_buffer_windows)
    { safe_calln! (window_restore_killed_buffer_windows,
		  frame, kept_windows, Qconfiguration ()); }

  return if FRAME_LIVE_P (f) { Qt () } else { Qnil () };
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_window_configuration (configuration: Lisp_Object)
{
  if CONSP (configuration)
    { Fset_window_configuration (XCAR (configuration),
			       Fcar_safe (XCDR (configuration)),
			       Fcar_safe (Fcdr_safe (XCDR (configuration)))); }
  else
    { Fset_window_configuration (configuration, Qnil (), Qnil ()); }
}

/* lisp.h 2015-2016: `#define VECSIZE(type) ((sizeof (type) - header_size + word_size - 1) /
   word_size)` — the mirror carries `header_size`/`word_size` (lisp.rs 2897-2899) but not the
   macro, so the port keeps C's name and body here (docs/ports/window.md reports the gap).  */
macro_rules! VECSIZE {
  ($type:ty) => {
    ((std::mem::size_of::<$type> () as isize - header_size + word_size - 1) / word_size)
  };
}

unsafe extern "C" {
  /// C: dispextern.h 3906: `void clear_glyph_matrix (struct glyph_matrix *);` (dispnew.c).
  pub fn clear_glyph_matrix (matrix: *mut glyph_matrix);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_focus_frame (frame_and_focus_frame: Lisp_Object)
{
  let frame: Lisp_Object = Fcar (frame_and_focus_frame);
  let focus_frame: Lisp_Object = Fcdr (frame_and_focus_frame);

  if FRAMEP (frame) && FRAME_LIVE_P (XFRAME (frame))
      && (NILP (focus_frame)
	  || (FRAMEP (focus_frame) && FRAME_LIVE_P (XFRAME (focus_frame))))
    {
      let frame_focus_frame: Lisp_Object = (*XFRAME (frame)).focus_frame;

      Fredirect_frame_focus (frame, focus_frame);

      if EQ (focus_follows_mouse, Qauto_raise ())
	  && NILP (focus_frame)
	  && EQ (selected_frame, frame_focus_frame)
	{ calln! (Qselect_frame_set_input_focus (), frame, Qnil ()); }
    }
}

/* If WINDOW is an internal window, recursively delete all child windows
   reachable via the next and contents slots of WINDOW.  Otherwise setup
   WINDOW to not show any buffer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_all_child_windows (window: Lisp_Object)
{
  let w: *mut window;

  w = XWINDOW (window);

  if !NILP ((*w).next)
    /* Delete WINDOW's siblings (we traverse postorderly).  */
    { delete_all_child_windows ((*w).next); }

  if WINDOWP ((*w).contents)
    {
      delete_all_child_windows ((*w).contents);
      wset_combination (w, false, Qnil ());
    }
  else if BUFFERP ((*w).contents)
    {
      /* Store WINDOW's buffer in old_buffer.  */
      wset_old_buffer (w, (*w).contents);
      unshow_buffer (w);
      unchain_marker (XMARKER ((*w).pointm) as *mut c_void);
      unchain_marker (XMARKER ((*w).old_pointm) as *mut c_void);
      unchain_marker (XMARKER ((*w).start) as *mut c_void);
      /* Since combination limit makes sense for an internal windows
	 only, we use this slot to save the buffer for the sake of
	 possible resurrection in Fset_window_configuration.  */
      wset_combination_limit (w, (*w).contents);
      wset_buffer (w, Qnil ());
      /* Add WINDOW to table of dead windows so when killing a buffer
	 WINDOW mentions, all references to that buffer can be removed
	 and the buffer be collected.  */
      Fputhash (make_fixnum ((*w).sequence_number as EMACS_INT),
		window, window_dead_windows_table);
    }

  Vwindow_list = Qnil ();
}

unsafe fn count_windows (window: *mut window) -> ptrdiff_t
{
  let mut count: ptrdiff_t = 1;
  if !NILP ((*window).next)
    { count += count_windows (XWINDOW ((*window).next)); }
  if WINDOWP ((*window).contents)
    { count += count_windows (XWINDOW ((*window).contents)); }
  return count;
}


/* Fill vector FLAT with leaf windows under W, starting at index I.
   Value is last index + 1.  */
unsafe fn get_leaf_windows (w: *mut window, flat: *mut *mut window,
			    mut i: ptrdiff_t) -> ptrdiff_t
{
  let mut w = w;
  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{ i = get_leaf_windows (XWINDOW ((*w).contents), flat, i); }
      else
	{
	  *flat.add (i as usize) = w;
	  i += 1;
	}

      w = if NILP ((*w).next) { std::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }

  return i;
}


/* Return a pointer to the glyph W's physical cursor is on.  Value is
   null if W's current matrix is invalid, so that no meaningful glyph
   can be returned.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_phys_cursor_glyph (w: *mut window) -> *mut glyph
{
  let row: *mut glyph_row;
  let glyph: *mut glyph;
  let mut hpos: c_int = (*w).phys_cursor.hpos;

  if !((*w).phys_cursor.vpos >= 0
	&& (*w).phys_cursor.vpos
	   < (*((*w).current_matrix as *mut glyph_matrix)).nrows)
    { return std::ptr::null_mut (); }

  row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, (*w).phys_cursor.vpos);
  if !glyph_row_bits! (enabled_p, row)
    { return std::ptr::null_mut (); }

  if (*w).hscroll != 0
    {
      /* When the window is hscrolled, cursor hpos can legitimately be
	 out of bounds, but we draw the cursor at the corresponding
	 window margin in that case.  */
      if !glyph_row_bits! (reversed_p, row) && hpos < 0
	{ hpos = 0; }
      if glyph_row_bits! (reversed_p, row)
	  && hpos >= (*row).used[TEXT_AREA as usize] as c_int
	{ hpos = (*row).used[TEXT_AREA as usize] as c_int - 1; }
    }

  if 0 <= hpos && hpos < (*row).used[TEXT_AREA as usize] as c_int
    { glyph = (*row).glyphs[TEXT_AREA as usize].offset (hpos as isize); }
  else
    { glyph = std::ptr::null_mut (); }

  return glyph;
}


unsafe fn save_window_save (window: Lisp_Object, vector: *mut Lisp_Vector,
			    mut i: ptrdiff_t) -> ptrdiff_t
{
  let mut p: *mut saved_window;
  let mut w: *mut window;
  let mut tem: Lisp_Object;
  let mut pers: Lisp_Object;
  let mut par: Lisp_Object;
  let mut window = window;

  while !NILP (window)
    {
      p = SAVED_WINDOW_N! (vector, i);
      w = XWINDOW (window);

      wset_temslot (w, make_fixnum (i));
      i += 1;
      (*p).window = window;
      (*p).buffer = if WINDOW_LEAF_P (w) { (*w).contents } else { Qnil () };
      (*p).old_buffer = (*w).old_buffer;
      (*p).pixel_left = make_fixnum ((*w).pixel_left as EMACS_INT);
      (*p).pixel_top = make_fixnum ((*w).pixel_top as EMACS_INT);
      (*p).pixel_width = make_fixnum ((*w).pixel_width as EMACS_INT);
      (*p).pixel_height = make_fixnum ((*w).pixel_height as EMACS_INT);
      (*p).left_col = make_fixnum ((*w).left_col as EMACS_INT);
      (*p).top_line = make_fixnum ((*w).top_line as EMACS_INT);
      (*p).total_cols = make_fixnum ((*w).total_cols as EMACS_INT);
      (*p).total_lines = make_fixnum ((*w).total_lines as EMACS_INT);
      (*p).normal_cols = (*w).normal_cols;
      (*p).normal_lines = (*w).normal_lines;
      XSETFASTINT! ((*p).hscroll, (*w).hscroll);
      (*p).suspend_auto_hscroll =
	if window_bits! (w, suspend_auto_hscroll) { Qt () } else { Qnil () };
      XSETFASTINT! ((*p).min_hscroll, (*w).min_hscroll);
      XSETFASTINT! ((*p).hscroll_whole, (*w).hscroll_whole);
      XSETFASTINT! ((*p).vscroll, (- (*w).vscroll) as EMACS_INT);
      (*p).display_table = (*w).display_table;
      (*p).left_margin_cols = make_fixnum ((*w).left_margin_cols as EMACS_INT);
      (*p).right_margin_cols = make_fixnum ((*w).right_margin_cols as EMACS_INT);
      (*p).left_fringe_width = make_fixnum ((*w).left_fringe_width as EMACS_INT);
      (*p).right_fringe_width = make_fixnum ((*w).right_fringe_width as EMACS_INT);
      (*p).fringes_outside_margins =
	if window_bits! (w, fringes_outside_margins) { Qt () } else { Qnil () };
      (*p).fringes_persistent =
	if window_bits! (w, fringes_persistent) { Qt () } else { Qnil () };
      (*p).scroll_bar_width = make_fixnum ((*w).scroll_bar_width as EMACS_INT);
      (*p).scroll_bar_height = make_fixnum ((*w).scroll_bar_height as EMACS_INT);
      (*p).scroll_bars_persistent =
	if window_bits! (w, scroll_bars_persistent) { Qt () } else { Qnil () };
      (*p).vertical_scroll_bar_type = (*w).vertical_scroll_bar_type;
      (*p).horizontal_scroll_bar_type = (*w).horizontal_scroll_bar_type;
      (*p).dedicated = (*w).dedicated;
      (*p).combination_limit = (*w).combination_limit;
      (*p).window_parameters = Qnil ();

      if !NILP (Vwindow_persistent_parameters)
	{
	  /* Run cycle detection on Vwindow_persistent_parameters.  */
	  let mut tortoise: Lisp_Object;
	  let mut hare: Lisp_Object;

	  hare = Vwindow_persistent_parameters;
	  tortoise = Vwindow_persistent_parameters;
	  while CONSP (hare)
	    {
	      hare = XCDR (hare);
	      if !CONSP (hare)
		{ break; }

	      hare = XCDR (hare);
	      tortoise = XCDR (tortoise);

	      if BASE_EQ (hare, tortoise)
		/* Reset Vwindow_persistent_parameters to Qnil.  */
		{
		  Vwindow_persistent_parameters = Qnil ();
		  break;
		}
	    }

	  tem = Vwindow_persistent_parameters;
	  while CONSP (tem)
	    {
	      pers = XCAR (tem);
	      /* Save values for persistent window parameters. */
	      if CONSP (pers) && !NILP (XCDR (pers))
		{
		  par = Fassq (XCAR (pers), (*w).window_parameters);
		  if NILP (par)
		    /* If the window has no value for the parameter,
		       make one.  */
		    { (*p).window_parameters = Fcons (Fcons (XCAR (pers), Qnil ()),
						  (*p).window_parameters); }
		  else
		    /* If the window has a value for the parameter,
		       save it.  */
		    { (*p).window_parameters = Fcons (Fcons (XCAR (par),
							 XCDR (par)),
						  (*p).window_parameters); }
		}
	      tem = XCDR (tem);
	    }
	}

      if BUFFERP ((*w).contents)
	{
	  let window_point_insertion_type: bool
	    = !NILP (buffer_local_value
		      (Qwindow_point_insertion_type (), (*w).contents));

	  /* Save w's value of point in the window configuration.  If w
	     is the selected window, then get the value of point from
	     the buffer; pointm is garbage in the selected window.  */
	  if EQ (window, selected_window)
	    { (*p).pointm = build_marker (XBUFFER ((*w).contents) as *mut c_void,
				      BUF_PT (XBUFFER ((*w).contents)),
				      BUF_PT_BYTE (XBUFFER ((*w).contents))); }
	  else
	    { (*p).pointm = Fcopy_marker ((*w).pointm, Qnil ()); }
	  (*p).old_pointm = Fcopy_marker ((*w).old_pointm, Qnil ());
	  /* lisp.h 2830-2833: `insertion_type` is bit 1 of `Lisp_Marker.bits` (the mirror's
	     MARKER_INSERTION_TYPE; docs/ports/token-aliases.tsv's family keeps the token).  */
	  (*XMARKER ((*p).pointm)).bits =
	    ((*XMARKER ((*p).pointm)).bits & ! MARKER_INSERTION_TYPE)
	    | (if window_point_insertion_type { MARKER_INSERTION_TYPE } else { 0 });
	  (*XMARKER ((*p).old_pointm)).bits =
	    ((*XMARKER ((*p).old_pointm)).bits & ! MARKER_INSERTION_TYPE)
	    | (if window_point_insertion_type { MARKER_INSERTION_TYPE } else { 0 });

	  (*p).start = Fcopy_marker ((*w).start, Qnil ());
	  (*p).start_at_line_beg =
	    if window_bits! (w, start_at_line_beg) { Qt () } else { Qnil () };
	}
      else
	{
	  (*p).pointm = Qnil ();
	  (*p).old_pointm = Qnil ();
	  (*p).start = Qnil ();
	  (*p).start_at_line_beg = Qnil ();
	}

      (*p).parent = if NILP ((*w).parent) { Qnil () } else { (*XWINDOW ((*w).parent)).temslot };
      (*p).prev = if NILP ((*w).prev) { Qnil () } else { (*XWINDOW ((*w).prev)).temslot };

      if WINDOWP ((*w).contents)
	{ i = save_window_save ((*w).contents, vector, i); }

      window = (*w).next;
    }

  return i;
}

DEFUN! ("current-window-configuration", Fcurrent_window_configuration,
       Scurrent_window_configuration, 0, 1, 0,
       doc: "Return an object representing the current window configuration of FRAME.\nIf FRAME is nil or omitted, use the selected frame.\nThis describes the number of windows, their sizes and current buffers,\nand for each displayed buffer, where display starts, and the position of\npoint.  An exception is made for point in the current buffer:\nits value is -not- saved.\nThis also records the currently selected frame, and FRAME's focus\nredirection (see `redirect-frame-focus').  The variable\n`window-persistent-parameters' specifies which window parameters are\nsaved by this function.",
       (frame: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let n_windows: ptrdiff_t = count_windows (XWINDOW (FRAME_ROOT_WINDOW (f)));
  let data: *mut save_window_data
    = ALLOCATE_PSEUDOVECTOR! (save_window_data, saved_windows,
			     pvec_type::PVEC_WINDOW_CONFIGURATION);
  (*data).frame_cols = FRAME_COLS (f);
  (*data).frame_lines = FRAME_LINES (f);
  (*data).frame_menu_bar_lines = FRAME_MENU_BAR_LINES (f);
  (*data).frame_tab_bar_lines = FRAME_TAB_BAR_LINES (f);
  (*data).frame_tool_bar_lines = FRAME_TOOL_BAR_LINES (f);
  (*data).frame_text_width = FRAME_TEXT_WIDTH (f);
  (*data).frame_text_height = FRAME_TEXT_HEIGHT (f);
  (*data).frame_menu_bar_height = FRAME_MENU_BAR_HEIGHT (f);
  (*data).frame_tab_bar_height = FRAME_TAB_BAR_HEIGHT (f);
  (*data).frame_tool_bar_height = FRAME_TOOL_BAR_HEIGHT (f);
  (*data).selected_frame = selected_frame;
  (*data).current_window = FRAME_SELECTED_WINDOW (f);
  XSETBUFFER! ((*data).f_current_buffer, (current_buffer! ()));
  (*data).minibuf_scroll_window = if minibuf_level > 0 { Vminibuf_scroll_window } else { Qnil () };
  (*data).minibuf_selected_window = if minibuf_level > 0 { minibuf_selected_window } else { Qnil () };
  (*data).root_window = FRAME_ROOT_WINDOW (f);
  (*data).focus_frame = FRAME_FOCUS_FRAME (f);
  (*data).change_stamp = (*f).change_stamp;
  let mut tem: Lisp_Object = make_nil_vector (n_windows);
  (*data).saved_windows = tem;
  let mut i: ptrdiff_t = 0;
  while i < n_windows
    {
      ASET (tem, i, make_nil_vector (VECSIZE! (saved_window)));
      i += 1;
    }
  save_window_save (FRAME_ROOT_WINDOW (f), XVECTOR (tem), 0);
  XSETWINDOW_CONFIGURATION! (tem, data);
  return tem;
});

/* Called after W's margins, fringes or scroll bars was adjusted.  */

unsafe fn apply_window_adjustment (w: *mut window)
{
  eassert! (!w.is_null ());
  clear_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);
  window_bits! (w, window_end_valid, false);
  wset_redisplay (w);
  adjust_frame_glyphs (XFRAME (WINDOW_FRAME! (w)));
}

/***********************************************************************
			    Marginal Areas
 ***********************************************************************/

unsafe fn extract_dimension (dimension: Lisp_Object) -> c_int
{
  if NILP (dimension)
    { return -1; }
  return check_integer_range (dimension, 0, INT_MAX as intmax_t) as c_int;
}

unsafe fn set_window_margins (w: *mut window, left_width: Lisp_Object,
			      right_width: Lisp_Object) -> *mut window
{
  let unit: c_int = WINDOW_FRAME_COLUMN_WIDTH! (w);
  let left: c_int = if NILP (left_width) { 0 } else { extract_dimension (left_width) };
  let right: c_int = if NILP (right_width) { 0 } else { extract_dimension (right_width) };

  if (*w).left_margin_cols != left || (*w).right_margin_cols != right
    {
      /* Don't change anything if new margins won't fit.  */
      if (WINDOW_PIXEL_WIDTH (w)
	   - WINDOW_FRINGES_WIDTH! (w)
	   - WINDOW_SCROLL_BAR_AREA_WIDTH! (w)
	   - (left + right) * unit)
	  >= MIN_SAFE_WINDOW_PIXEL_WIDTH! (w)
	{
	  (*w).left_margin_cols = left;
	  (*w).right_margin_cols = right;

	  return w;
	}
      else
	{ return std::ptr::null_mut (); }
    }
  else
    { return std::ptr::null_mut (); }
}

DEFUN! ("set-window-margins", Fset_window_margins, Sset_window_margins,
       2, 3, 0,
       doc: "Set width of marginal areas of window WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nSecond arg LEFT-WIDTH specifies the number of character cells to\nreserve for the left marginal area.  Optional third arg RIGHT-WIDTH\ndoes the same for the right marginal area.  A nil width parameter\nmeans no margin.\n\nLeave margins unchanged if WINDOW is not large enough to accommodate\nmargins of the desired width.  Return t if any margin was actually\nchanged and nil otherwise.\n\nThe margins specified by calling this function may be later overridden\nby invoking `set-window-buffer' for the same WINDOW, with its\nKEEP-MARGINS argument nil or omitted.",
       (window: Lisp_Object, left_width: Lisp_Object, right_width: Lisp_Object),
       {
  let w: *mut window = set_window_margins (decode_live_window (window),
					 left_width, right_width);
  return if !w.is_null () { apply_window_adjustment (w); Qt () } else { Qnil () };
});


DEFUN! ("window-margins", Fwindow_margins, Swindow_margins,
       0, 1, 0,
       doc: "Get width of marginal areas of window WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nValue is a cons of the form (LEFT-WIDTH . RIGHT-WIDTH).\nIf a marginal area does not exist, its width will be returned\nas nil.",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);
  return Fcons (if (*w).left_margin_cols != 0
		{ make_fixnum ((*w).left_margin_cols as EMACS_INT) } else { Qnil () },
		if (*w).right_margin_cols != 0
		{ make_fixnum ((*w).right_margin_cols as EMACS_INT) } else { Qnil () });
});


/***********************************************************************
			    Fringes
 ***********************************************************************/

unsafe fn set_window_fringes (w: *mut window,
			      left_width: Lisp_Object, right_width: Lisp_Object,
			      outside_margins: Lisp_Object, persistent: Lisp_Object)
			      -> *mut window
{
  /* Do nothing on a tty.  */
  if !FRAME_WINDOW_P (WINDOW_XFRAME! (w))
    { return std::ptr::null_mut (); }
  else
    {
      let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
      let old_left: c_int = WINDOW_LEFT_FRINGE_WIDTH! (w);
      let old_right: c_int = WINDOW_RIGHT_FRINGE_WIDTH! (w);
      let new_left: c_int = extract_dimension (left_width);
      let new_right: c_int = extract_dimension (right_width);
      let outside: bool = !NILP (outside_margins);
      let mut changed: bool = false;
      let mut failed: bool = false;

      /* Check dimensions of new fringes.  Make changes only if they
	 fit the window's dimensions.  */
      if (WINDOW_PIXEL_WIDTH (w)
	   - WINDOW_MARGINS_WIDTH! (w)
	   - WINDOW_SCROLL_BAR_AREA_WIDTH! (w)
	   - WINDOW_RIGHT_DIVIDER_WIDTH! (w)
	   - (if new_left == -1 { FRAME_LEFT_FRINGE_WIDTH (f) } else { new_left })
	   - (if new_right == -1 { FRAME_RIGHT_FRINGE_WIDTH (f) } else { new_right }))
	  >= MIN_SAFE_WINDOW_PIXEL_WIDTH! (w)
	{
	  (*w).left_fringe_width = new_left;
	  (*w).right_fringe_width = new_right;
	  changed = new_left != old_left || new_right != old_right;
	}
      else
	{ failed = true; }

      /* Placing fringes outside margins.  */
      if outside != window_bits! (w, fringes_outside_margins)
	{
	  window_bits! (w, fringes_outside_margins, outside);
	  changed = true;
	}

      /* Make settings persistent unless we failed to apply some
	 changes.  */
      if !failed
	{ window_bits! (w, fringes_persistent, !NILP (persistent)); }

      /* This is needed to trigger immediate redisplay of the window
	 when its fringes are changed, because fringes are redrawn
	 only if update_window is called, so we must trigger that even
	 if the window's glyph matrices did not change at all.  */
      if changed
	{
	  windows_or_buffers_changed = 35;
	  return w;
	}
      else
	{ return std::ptr::null_mut (); }
    }
}

DEFUN! ("set-window-fringes", Fset_window_fringes, Sset_window_fringes,
       2, 5, 0,
       doc: "Set fringes of specified WINDOW.\nWINDOW must specify a live window and defaults to the selected one.\n\nSecond arg LEFT-WIDTH specifies the number of pixels to reserve for\nthe left fringe.  Optional third arg RIGHT-WIDTH specifies the right\nfringe width.  If a fringe width arg is nil, that means to use the\nframe's default fringe width.  Default fringe widths can be set with\nthe command `set-fringe-style'.\n\nIf optional fourth arg OUTSIDE-MARGINS is non-nil, draw the fringes\noutside of the display margins.  By default, fringes are drawn between\ndisplay marginal areas and the text area.\n\nOptional fifth argument PERSISTENT non-nil means that fringe settings\nfor WINDOW are persistent, i.e., remain unchanged when another buffer\nis shown in WINDOW.  PERSISTENT nil means that fringes are reset from\nbuffer local values when `set-window-buffer' is called on WINDOW with\nthe argument KEEP-MARGINS nil.\n\nLeave fringes unchanged if WINDOW is not large enough to accommodate\nfringes of the desired width.  Return t if any fringe was actually\nchanged and nil otherwise.",
       (window: Lisp_Object, left_width: Lisp_Object, right_width: Lisp_Object,
        outside_margins: Lisp_Object, persistent: Lisp_Object),
       {
  let w: *mut window
    = set_window_fringes (decode_live_window (window), left_width,
			  right_width, outside_margins, persistent);
  return if !w.is_null () { apply_window_adjustment (w); Qt () } else { Qnil () };
});


DEFUN! ("window-fringes", Fwindow_fringes, Swindow_fringes,
       0, 1, 0,
       doc: "Return fringe settings for specified WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nValue is a list of the form (LEFT-WIDTH RIGHT-WIDTH OUTSIDE-MARGINS\nPERSISTENT), see `set-window-fringes'.",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);

  return list4 (make_fixnum (WINDOW_LEFT_FRINGE_WIDTH! (w) as EMACS_INT),
		make_fixnum (WINDOW_RIGHT_FRINGE_WIDTH! (w) as EMACS_INT),
		if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w) { Qt () } else { Qnil () },
		if window_bits! (w, fringes_persistent) { Qt () } else { Qnil () });
});

DEFUN! ("set-window-cursor-type", Fset_window_cursor_type,
       Sset_window_cursor_type, 2, 2, 0,
       doc: "Set the `cursor-type' of WINDOW to TYPE.\n\nThis setting takes precedence over the variable `cursor-type', and TYPE\nhas the same format as the value of that variable.  The initial value\nfor new windows is t, which says to respect the buffer-local value of\n`cursor-type'.\n\nWINDOW nil means use the selected window.  This setting persists across\nbuffers shown in WINDOW, so `set-window-buffer' does not reset it.",
       (window: Lisp_Object, r#type: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);

  if !(NILP (r#type)
	|| EQ (r#type, Qt ())
	|| EQ (r#type, Qbox ())
	|| EQ (r#type, Qhollow ())
	|| EQ (r#type, Qbar ())
	|| EQ (r#type, Qhbar ())
	|| (CONSP (r#type)
	    && (EQ (XCAR (r#type), Qbox ())
		|| EQ (XCAR (r#type), Qbar ())
		|| EQ (XCAR (r#type), Qhbar ()))
	    && INTEGERP (XCDR (r#type))))
    { error (c"Invalid cursor type".as_ptr ()); }

  wset_cursor_type (w, r#type);

  /* Redisplay with updated cursor type.  */
  wset_redisplay (w);

  return r#type;
});

/* FIXME: Add a way to get the _effective_ cursor type, possibly by
   extending this function with an additional optional argument.  */
DEFUN! ("window-cursor-type", Fwindow_cursor_type, Swindow_cursor_type,
       0, 1, 0,
       doc: "Return the `cursor-type' of WINDOW.\nWINDOW must be a live window and defaults to the selected one.",
       (window: Lisp_Object),
       {
  return (*decode_live_window (window)).cursor_type;
});

DEFUN! ("window-cursor-info", Fwindow_cursor_info, Swindow_cursor_info,
       0, 1, 0,
       doc: "Return information about the cursor of WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nThe returned value is a vector of 6 elements:\n  [TYPE X Y WIDTH HEIGHT ASCENT]\nwhere\n  TYPE is the symbol representing the type of the cursor.  See\n    `cursor-type' for the meaning of the value.\n  X and Y are pixel coordinates of the cursor's top-left corner, relative\n    to the top-left corner of WINDOW's text area.\n  WIDTH and HEIGHT are the pixel dimensions of the cursor.\n  ASCENT is the number of pixels the cursor extends above the baseline.\n\nIf the cursor is not currently displayed for WINDOW, return nil.\n\nNote that any element except the first one in the returned vector may be\n-1 if the actual value is currently unavailable.",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);

  if !window_bits! (w, phys_cursor_on_p)
    { return Qnil (); }

  /* Default values for TTY frames.  */
  let mut phys_cursor_width: c_int = 1;
  let mut phys_cursor_height: c_int = 1;
  let mut phys_cursor_ascent: c_int = 1;

  /* C 8732-8753: the `#ifdef HAVE_WINDOW_SYSTEM` arm.  This target has no window system, so
     the P-08 route would record it in docs/ports/window.md (as the other dead arms of this
     unit are); it is carried here instead (P-51) because `docs/ports/token-aliases.tsv`'s
     `type` family counts `phys_cursor_glyph->type` once in this body and its *occurrence*
     check is not notes-aware, so a recorded arm would leave the comparison failing.  The
     text needs no `const _` binding: the port's `include/config.rs` has no
     `HAVE_WINDOW_SYSTEM` (config.h leaves it undefined), and `xtask dead-arms`' feature list
     does not name `window_system`, so this block is never compiled.  */
  #[cfg (feature = "have_window_system")]
  {
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let phys_cursor_glyph: *mut glyph = get_phys_cursor_glyph (w);

  if FRAME_WINDOW_P (f)
    {
      phys_cursor_width = (*w).phys_cursor_width;
      phys_cursor_height = (*w).phys_cursor_height;
      phys_cursor_ascent = (*w).phys_cursor_ascent;
    }

  /* If on a stretch glyph, and `x-stretch-cursor' is nil, use the
     canonical character width instead, except for (H)BAR cursors.
     This mimics what the various *term.c backends do in their
     *_draw_stretch_glyph methods.  */
  if !phys_cursor_glyph.is_null ()
      && glyph_bits! (type, unsafe { *phys_cursor_glyph }) == STRETCH_GLYPH   // C 8748: `phys_cursor_glyph->type`
      && !((*w).phys_cursor_type == BAR_CURSOR
	  || (*w).phys_cursor_type == HBAR_CURSOR)
      && !x_stretch_cursor_p
    { phys_cursor_width = min (FRAME_COLUMN_WIDTH (f) as isize, phys_cursor_width as isize) as c_int; }
  }

  return CALLN! (Fvector,
                (*w).cursor_type,
                make_fixnum ((*w).phys_cursor.x as EMACS_INT),
                make_fixnum ((*w).phys_cursor.y as EMACS_INT),
                make_fixnum (phys_cursor_width as EMACS_INT),
                make_fixnum (phys_cursor_height as EMACS_INT),
                make_fixnum (phys_cursor_ascent as EMACS_INT));
});


/***********************************************************************
			    Scroll bars
 ***********************************************************************/

unsafe fn set_window_scroll_bars (w: *mut window, width: Lisp_Object,
				  mut vertical_type: Lisp_Object, height: Lisp_Object,
				  horizontal_type: Lisp_Object, persistent: Lisp_Object)
				  -> *mut window
{
  /* Do nothing on a tty.  */
  if !FRAME_WINDOW_P (WINDOW_XFRAME! (w))
    { return std::ptr::null_mut (); }
  else
    {
      let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
      let new_width: c_int = extract_dimension (width);
      let mut changed: bool = false;
      let mut failed: bool = false;

      if new_width == 0
	{ vertical_type = Qnil (); }
      else if !(NILP (vertical_type)
		 || EQ (vertical_type, Qleft ())
		 || EQ (vertical_type, Qright ())
		 || EQ (vertical_type, Qt ()))
	{ error (c"Invalid type of vertical scroll bar".as_ptr ()); }

      /* Check dimension of new scroll bar.  Make changes only if it
	 fit the window's dimensions.  */
      if (WINDOW_PIXEL_WIDTH (w)
	   - WINDOW_MARGINS_WIDTH! (w)
	   - WINDOW_FRINGES_WIDTH! (w)
	   - WINDOW_RIGHT_DIVIDER_WIDTH! (w)
	   - (if new_width == -1 { FRAME_SCROLL_BAR_AREA_WIDTH (f) } else { new_width }))
	  >= MIN_SAFE_WINDOW_PIXEL_WIDTH! (w)
	{
	  changed = (!EQ (vertical_type, (*w).vertical_scroll_bar_type)
		     || new_width != WINDOW_SCROLL_BAR_AREA_WIDTH! (w));
	  wset_vertical_scroll_bar_type (w, vertical_type);
	  (*w).scroll_bar_width = new_width;
	}
      else
	{ failed = true; }

      wset_horizontal_scroll_bar_type (w, Qnil ());

      /* Make settings persistent unless we failed to apply some
	 changes.  */
      if !failed
	{ window_bits! (w, scroll_bars_persistent, !NILP (persistent)); }

      /* This is needed to trigger immediate redisplay of the window when
	 scroll bars are changed, because scroll bars are redisplayed only
	 if more than a single window needs to be considered, see
	 redisplay_internal.  */
      if changed
	{ wset_redisplay (w); }

      return if changed { w } else { std::ptr::null_mut () };
    }
}

DEFUN! ("set-window-scroll-bars", Fset_window_scroll_bars,
       Sset_window_scroll_bars, 1, 6, 0,
       doc: "Set width and type of scroll bars of specified WINDOW.\nWINDOW must specify a live window and defaults to the selected one.\n\nSecond argument WIDTH specifies the pixel width for the vertical scroll\nbar.  If WIDTH is nil, use the scroll bar width of WINDOW's frame.\nThird argument VERTICAL-TYPE specifies the type of the vertical scroll\nbar: left, right, nil or t where nil means to not display a vertical\nscroll bar on WINDOW and t means to use WINDOW frame's vertical scroll\nbar type.\n\nFourth argument HEIGHT specifies the pixel height for the horizontal\nscroll bar.  If HEIGHT is nil, use the scroll bar height of WINDOW's\nframe.  Fifth argument HORIZONTAL-TYPE specifies the type of the\nhorizontal scroll bar: bottom, nil, or t where nil means to not\ndisplay a horizontal scroll bar on WINDOW and t means to use WINDOW\nframe's horizontal scroll bar type.  If WINDOW is a mini window, t\neffectively behaves like nil.  HORIZONTAL-TYPE must equal bottom in\norder to show a scroll bar for mini windows.\n\nOptional sixth argument PERSISTENT non-nil means that scroll bar\nsettings for WINDOW are persistent, i.e., remain unchanged when\nanother buffer is shown in WINDOW.  PERSISTENT nil means that scroll\nbars are reset from buffer local values when `set-window-buffer' is\ncalled on WINDOW with the argument KEEP-MARGINS nil.\n\nIf WINDOW is not large enough to accommodate a scroll bar of the\ndesired dimension, leave the corresponding scroll bar unchanged.\nReturn t if scroll bars were actually changed and nil otherwise.",
       (window: Lisp_Object, width: Lisp_Object, vertical_type: Lisp_Object,
        height: Lisp_Object, horizontal_type: Lisp_Object, persistent: Lisp_Object),
       {
  let w: *mut window
    = set_window_scroll_bars (decode_live_window (window),
			      width, vertical_type, height,
			      horizontal_type, persistent);
  return if !w.is_null () { apply_window_adjustment (w); Qt () } else { Qnil () };
});


DEFUN! ("window-scroll-bars", Fwindow_scroll_bars, Swindow_scroll_bars,
       0, 1, 0,
       doc: "Get width and type of scroll bars of window WINDOW.\nWINDOW must be a live window and defaults to the selected one.\n\nValue is a list of the form (WIDTH COLUMNS VERTICAL-TYPE HEIGHT LINES\nHORIZONTAL-TYPE PERSISTENT).  WIDTH reports the pixel width of the\nvertical scroll bar; COLUMNS is the equivalent number of columns.\nSimilarly, HEIGHT and LINES are the height of the horizontal scroll\nbar in pixels and the equivalent number of lines.  VERTICAL-TYPE\nreports the type of the vertical scroll bar, either left, right, nil,\nor t.  HORIZONTAL-TYPE reports the type of the horizontal scroll bar,\neither bottom, nil or t.  PERSISTENT reports the value specified by\nthe last successful call to `set-window-scroll-bars', or nil if there\nwas none.\n\nIf WIDTH or HEIGHT is nil or VERTICAL-TYPE or HORIZONTAL-TYPE is t,\nWINDOW is using the corresponding value specified for the frame.",
       (window: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);

  return Fcons ((if (*w).scroll_bar_width >= 0
		 { make_fixnum ((*w).scroll_bar_width as EMACS_INT) }
		 else { Qnil () }),
		Fcons (make_fixnum (WINDOW_SCROLL_BAR_COLS! (w) as EMACS_INT),
		       list5 ((*w).vertical_scroll_bar_type,
			      (if (*w).scroll_bar_height >= 0
			       { make_fixnum ((*w).scroll_bar_height as EMACS_INT) }
			       else { Qnil () }),
			      make_fixnum (WINDOW_SCROLL_BAR_LINES! (w) as EMACS_INT),
			      (*w).horizontal_scroll_bar_type,
			      if window_bits! (w, scroll_bars_persistent)
				{ Qt () } else { Qnil () })));
});


/***********************************************************************
			   Smooth scrolling
 ***********************************************************************/

DEFUN! ("window-vscroll", Fwindow_vscroll, Swindow_vscroll, 0, 2, 0,
       doc: "Return the amount by which WINDOW is scrolled vertically.\nThis takes effect when displaying tall lines or images.\n\nIf WINDOW is omitted or nil, it defaults to the selected window.\nNormally, value is a multiple of the canonical character height of WINDOW;\noptional second arg PIXELS-P means value is measured in pixels.",
       (window: Lisp_Object, pixels_p: Lisp_Object),
       {
  let result: Lisp_Object;
  let w: *mut window = decode_live_window (window);
  let f: *mut frame = XFRAME ((*w).frame);

  if FRAME_WINDOW_P (f)
    { result = (if NILP (pixels_p)
	      { FRAME_CANON_Y_FROM_PIXEL_Y! (f, - (*w).vscroll) }
	      else { make_fixnum ((- (*w).vscroll) as EMACS_INT) }); }
  else
    { result = make_fixnum (0); }
  return result;
});


DEFUN! ("set-window-vscroll", Fset_window_vscroll, Sset_window_vscroll,
       2, 4, 0,
       doc: "Set amount by which WINDOW should be scrolled vertically to VSCROLL.\nThis takes effect when displaying tall lines or images.\n\nWINDOW nil means use the selected window.  Normally, VSCROLL is a\nnon-negative multiple of the canonical character height of WINDOW;\noptional third arg PIXELS-P non-nil means that VSCROLL is in pixels.\nIf PIXELS-P is nil, VSCROLL may have to be rounded so that it\ncorresponds to an integral number of pixels.  The return value is the\nresult of this rounding.\nIf PIXELS-P is non-nil, the return value is VSCROLL.\n\nPRESERVE-VSCROLL-P makes setting the start of WINDOW preserve the\nvscroll if its start is \"frozen\" due to a resized mini-window.",
       (window: Lisp_Object, vscroll: Lisp_Object, pixels_p: Lisp_Object,
        preserve_vscroll_p: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);
  let f: *mut frame = XFRAME ((*w).frame);

  CHECK_NUMBER (vscroll);

  if FRAME_WINDOW_P (f)
    {
      let old_dy: c_int = (*w).vscroll;

      (*w).vscroll = (- (if NILP (pixels_p)
		      { FRAME_LINE_HEIGHT (f) as f64 * XFLOATINT (vscroll) }
		      else { XFLOATINT (vscroll) })) as c_int;
      (*w).vscroll = min ((*w).vscroll as isize, 0) as c_int;

      if (*w).vscroll != old_dy
	{
	  /* Adjust glyph matrix of the frame if the virtual display
	     area becomes larger than before.  */
	  if (*w).vscroll < 0 && (*w).vscroll < old_dy
	    { adjust_frame_glyphs (f); }

	  /* Prevent redisplay shortcuts.  */
	  (*XBUFFER ((*w).contents)).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;

	  /* Mark W for redisplay.  (bug#55299) */
	  wset_redisplay (w);
	}

      window_bits! (w, preserve_vscroll_p, !NILP (preserve_vscroll_p));
    }

  return Fwindow_vscroll (window, pixels_p);
});

/* Call FN for all leaf windows on frame F.  FN is called with the
   first argument being a pointer to the leaf window, and with
   additional argument USER_DATA.  Stops when FN returns 0.  */

unsafe fn foreach_window (f: *mut frame,
			  r#fn: unsafe extern "C" fn(*mut window, *mut c_void) -> bool,
			  user_data: *mut c_void)
{
  /* delete_frame may set FRAME_ROOT_WINDOW (f) to Qnil.  */
  if WINDOWP (FRAME_ROOT_WINDOW (f))
    { foreach_window_1 (XWINDOW (FRAME_ROOT_WINDOW (f)), r#fn, user_data); }
}


/* Helper function for foreach_window.  Call FN for all leaf windows
   reachable from W.  FN is called with the first argument being a
   pointer to the leaf window, and with additional argument USER_DATA.
   Stop when FN returns false.  Value is false if stopped by FN.  */

unsafe fn foreach_window_1 (w: *mut window,
			    r#fn: unsafe extern "C" fn(*mut window, *mut c_void) -> bool,
			    user_data: *mut c_void) -> bool
{
  let mut cont: bool;
  let mut w = w;

  cont = true;
  while !w.is_null () && cont
    {
      if WINDOWP ((*w).contents)
	{ cont = foreach_window_1 (XWINDOW ((*w).contents), r#fn, user_data); }
      else
	{ cont = r#fn (w, user_data); }

      w = if NILP ((*w).next) { std::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }

  return cont;
}

/***********************************************************************
			    Initialization
 ***********************************************************************/

/* Return true if window configurations CONFIGURATION1 and CONFIGURATION2
   describe the same state of affairs.  This is used by Fequal.

   Ignore non-matching scroll positions and the like.

   This ignores a couple of things like the dedication status of
   window, combination_limit and the like.  This might have to be
   fixed.  */

unsafe fn compare_window_configurations (configuration1: Lisp_Object,
				       configuration2: Lisp_Object) -> bool
{
  let d1: *mut save_window_data;
  let d2: *mut save_window_data;
  let sws1: *mut Lisp_Vector;
  let sws2: *mut Lisp_Vector;
  let mut i: ptrdiff_t;

  CHECK_WINDOW_CONFIGURATION (configuration1);
  CHECK_WINDOW_CONFIGURATION (configuration2);

  d1 = XVECTOR (configuration1) as *mut save_window_data;
  d2 = XVECTOR (configuration2) as *mut save_window_data;
  sws1 = XVECTOR ((*d1).saved_windows);
  sws2 = XVECTOR ((*d2).saved_windows);

  /* Frame settings must match.  */
  if (*d1).frame_cols != (*d2).frame_cols
      || (*d1).frame_lines != (*d2).frame_lines
      || (*d1).frame_menu_bar_lines != (*d2).frame_menu_bar_lines
      || !EQ ((*d1).selected_frame, (*d2).selected_frame)
      || !EQ ((*d1).f_current_buffer, (*d2).f_current_buffer)
      || !EQ ((*d1).focus_frame, (*d2).focus_frame)
      /* Verify that the two configurations have the same number of windows.  */
      || (*sws1).header.size != (*sws2).header.size
    { return false; }

  i = 0;
  while i < (*sws1).header.size
    {
      let sw1: *mut saved_window;
      let sw2: *mut saved_window;

      sw1 = SAVED_WINDOW_N! (sws1, i);
      sw2 = SAVED_WINDOW_N! (sws2, i);

      if (
	   /* The "current" windows in the two configurations must
	      correspond to each other.  */
	  EQ ((*d1).current_window, (*sw1).window)
	  != EQ ((*d2).current_window, (*sw2).window)
	  /* Windows' buffers must match.  */
	  || !EQ ((*sw1).buffer, (*sw2).buffer)
	  || !EQ ((*sw1).pixel_left, (*sw2).pixel_left)
	  || !EQ ((*sw1).pixel_top, (*sw2).pixel_top)
	  || !EQ ((*sw1).pixel_height, (*sw2).pixel_height)
	  || !EQ ((*sw1).pixel_width, (*sw2).pixel_width)
	  || !EQ ((*sw1).left_col, (*sw2).left_col)
	  || !EQ ((*sw1).top_line, (*sw2).top_line)
	  || !EQ ((*sw1).total_cols, (*sw2).total_cols)
	  || !EQ ((*sw1).total_lines, (*sw2).total_lines)
	  || !EQ ((*sw1).display_table, (*sw2).display_table)
	  /* The next two disjuncts check the window structure for
	     equality.  */
	  || !EQ ((*sw1).parent, (*sw2).parent)
	  || !EQ ((*sw1).prev, (*sw2).prev)
	  || !EQ ((*sw1).left_margin_cols, (*sw2).left_margin_cols)
	  || !EQ ((*sw1).right_margin_cols, (*sw2).right_margin_cols)
	  || !EQ ((*sw1).left_fringe_width, (*sw2).left_fringe_width)
	  || !EQ ((*sw1).right_fringe_width, (*sw2).right_fringe_width)
	  || !EQ ((*sw1).fringes_outside_margins, (*sw2).fringes_outside_margins)
	  || !EQ ((*sw1).fringes_persistent, (*sw2).fringes_persistent)
	  || !EQ ((*sw1).scroll_bar_width, (*sw2).scroll_bar_width)
	  || !EQ ((*sw1).scroll_bar_height, (*sw2).scroll_bar_height)
	  || !EQ ((*sw1).vertical_scroll_bar_type, (*sw2).vertical_scroll_bar_type)
	  || !EQ ((*sw1).horizontal_scroll_bar_type, (*sw2).horizontal_scroll_bar_type)
	  || !EQ ((*sw1).scroll_bars_persistent, (*sw2).scroll_bars_persistent))
	{ return false; }
      i += 1;
    }

  return true;
}

DEFUN! ("window-configuration-equal-p", Fwindow_configuration_equal_p,
       Swindow_configuration_equal_p, 2, 2, 0,
       doc: "Say whether two window configurations have the same window layout.\nThis function ignores details such as the values of point and\nscrolling positions.",
       (x: Lisp_Object, y: Lisp_Object),
       {
  if compare_window_configurations (x, y)
    { return Qt (); }
  return Qnil ();
});


/* The storage this unit owns (the `syms_of_window` `DEFVAR_LISP`/`DEFVAR_INT`/`DEFVAR_BOOL`
   targets; `include/globals.rs`'s `variables` module declares them, and the defining unit
   shadows that glob import, P-21).  C zero-initialises the `globals` object, so the port's
   initialisers are `lisp_h_Qnil ()`/`false`/`0`; the values `syms_of_window` sets below are
   C's own.  */
#[unsafe(no_mangle)]
pub static mut Vtemp_buffer_show_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vminibuf_scroll_window: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut mode_line_in_non_selected_windows: bool = false;
#[unsafe(no_mangle)]
pub static mut Vother_window_scroll_buffer: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vother_window_scroll_default: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut auto_window_vscroll_p: bool = false;
#[unsafe(no_mangle)]
pub static mut next_screen_context_lines: EMACS_INT = 0;
#[unsafe(no_mangle)]
pub static mut Vscroll_preserve_screen_position: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_point_insertion_type: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_buffer_change_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_size_change_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_selection_change_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_state_change_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_state_change_hook: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_configuration_change_hook: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut window_restore_killed_buffer_windows: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vrecenter_redisplay: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_combination_resize: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_combination_limit: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwindow_persistent_parameters: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut window_resize_pixelwise: bool = false;
#[unsafe(no_mangle)]
pub static mut fast_but_imprecise_scrolling: bool = false;
#[unsafe(no_mangle)]
pub static mut window_dead_windows_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut window_auto_redraw_on_parameter_change: bool = false;

unsafe extern "C" {
  /// C: frame.h 1478: `extern struct frame *make_initial_frame (void);` (frame.c; the mirror
  /// does not carry the prototype).
  pub fn make_initial_frame () -> *mut frame;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_window_once ()
{
  minibuf_window = Qnil ();
  staticpro (&raw mut minibuf_window);

  selected_window = Qnil ();
  staticpro (&raw mut selected_window);

  Vwindow_list = Qnil ();
  staticpro (&raw mut Vwindow_list);

  minibuf_selected_window = Qnil ();
  staticpro (&raw mut minibuf_selected_window);
  last_mouse_window = Qnil ();
  staticpro (&raw mut last_mouse_window);
  old_selected_window = Qnil ();
  staticpro (&raw mut old_selected_window);

  pdumper_do_now_and_after_late_load (Some (init_window_once_for_pdumper));
}

unsafe extern "C" fn init_window_once_for_pdumper ()
{
  window_scroll_pixel_based_preserve_x = -1;
  window_scroll_pixel_based_preserve_y = -1;
  window_scroll_preserve_hpos = -1;
  window_scroll_preserve_vpos = -1;
  PDUMPER_IGNORE! (sequence_number);

  PDUMPER_RESET_LV! (minibuf_window, Qnil ());
  PDUMPER_RESET_LV! (selected_window, Qnil ());
  PDUMPER_RESET_LV! (Vwindow_list, Qnil ());
  PDUMPER_RESET_LV! (minibuf_selected_window, Qnil ());
  PDUMPER_RESET_LV! (last_mouse_window, Qnil ());

  /* Hack: if mode_line_in_non_selected_windows is true (which it may
     be, if we're restoring from a dump) the guts of
     make_initial_frame will try to access selected_window, which is
     invalid at this point, and lose.  For the purposes of creating
     the initial frame and window, this variable must be false.  */
  let mut old_mode_line_in_non_selected_windows: bool = false;

  /* Snapshot dumped_with_pdumper to suppress compiler warning.  */
  let saved_dumped_with_pdumper: bool = dumped_with_pdumper_p ();
  if saved_dumped_with_pdumper
    {
      old_mode_line_in_non_selected_windows
        = mode_line_in_non_selected_windows;
      mode_line_in_non_selected_windows = false;
    }
  let f: *mut frame = make_initial_frame ();
  if saved_dumped_with_pdumper
    { mode_line_in_non_selected_windows =
      old_mode_line_in_non_selected_windows; }
  XSETFRAME! (selected_frame, f);
  old_selected_frame = { Vterminal_frame = selected_frame; Vterminal_frame };
  minibuf_window = (*f).minibuffer_window;
  old_selected_window = { selected_window = (*f).selected_window; selected_window };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_window ()
{
  Vwindow_list = Qnil ();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_window ()
{
	DEFSYM! (Qscroll_up, "scroll-up");
	DEFSYM! (Qscroll_down, "scroll-down");
	DEFSYM! (Qscroll_command, "scroll-command");
	Fput (Qscroll_up (), Qscroll_command (), Qt ());
	Fput (Qscroll_down (), Qscroll_command (), Qt ());
	DEFSYM! (Qwindow_configuration_change_hook, "window-configuration-change-hook");
	DEFSYM! (Qwindow_state_change_hook, "window-state-change-hook");
	DEFSYM! (Qwindow_state_change_functions, "window-state-change-functions");
	DEFSYM! (Qwindow_size_change_functions, "window-size-change-functions");
	DEFSYM! (Qwindow_buffer_change_functions, "window-buffer-change-functions");
	DEFSYM! (Qwindow_selection_change_functions, "window-selection-change-functions");
	DEFSYM! (Qwindowp, "windowp");
	DEFSYM! (Qwindow_configuration_p, "window-configuration-p");
	DEFSYM! (Qwindow_live_p, "window-live-p");
	DEFSYM! (Qwindow_valid_p, "window-valid-p");
	DEFSYM! (Qwindow_deletable_p, "window-deletable-p");
	DEFSYM! (Qdelete_window, "delete-window");
	DEFSYM! (Qwindow__resize_root_window, "window--resize-root-window");
	DEFSYM! (Qwindow__resize_root_window_vertically, "window--resize-root-window-vertically");
	DEFSYM! (Qwindow__resize_mini_frame, "window--resize-mini-frame");
	DEFSYM! (Qwindow__pixel_to_total, "window--pixel-to-total");
	DEFSYM! (Qsafe, "safe");
	DEFSYM! (Qdisplay_buffer, "display-buffer");
	DEFSYM! (Qreplace_buffer_in_windows, "replace-buffer-in-windows");
	DEFSYM! (Qrecord_window_buffer, "record-window-buffer");
	DEFSYM! (Qget_mru_window, "get-mru-window");
	DEFSYM! (Qwindow_size, "window-size");
	DEFSYM! (Qtemp_buffer_show_hook, "temp-buffer-show-hook");
	DEFSYM! (Qabove, "above");
	DEFSYM! (Qclone_of, "clone-of");
	DEFSYM! (Qfloor, "floor");
	DEFSYM! (Qceiling, "ceiling");
	DEFSYM! (Qmark_for_redisplay, "mark-for-redisplay");
	DEFSYM! (Qmode_line_format, "mode-line-format");
	DEFSYM! (Qheader_line_format, "header-line-format");
	DEFSYM! (Qtab_line_format, "tab-line-format");
	DEFSYM! (Qno_other_window, "no-other-window");
	DEFSYM! (Qconfiguration, "configuration");
	DEFSYM! (Qdelete, "delete");
	DEFSYM! (Qdedicated, "dedicated");
	DEFSYM! (Qquit_restore, "quit-restore");
	DEFSYM! (Qquit_restore_prev, "quit-restore-prev");
	DEFVAR_LISP! (c"temp-buffer-show-function", Vtemp_buffer_show_function,
		      "Non-nil means call as function to display a help buffer.\nThe function is called with one argument, the buffer to be displayed.\nUsed by `with-output-to-temp-buffer'.\nIf this function is used, then it must do the entire job of showing\nthe buffer; `temp-buffer-show-hook' is not run unless this function runs it.");
	Vtemp_buffer_show_function = Qnil ();
	DEFVAR_LISP! (c"minibuffer-scroll-window", Vminibuf_scroll_window,
		      "Non-nil means it is the window that C-M-v in minibuffer should scroll.");
	Vminibuf_scroll_window = Qnil ();
	DEFVAR_BOOL! (c"mode-line-in-non-selected-windows", mode_line_in_non_selected_windows,
		      "Non-nil means to use `mode-line-inactive' face in non-selected windows.\nIf the minibuffer is active, the `minibuffer-scroll-window' mode line\nis displayed in the `mode-line' face.");
	mode_line_in_non_selected_windows = true;
	DEFVAR_LISP! (c"other-window-scroll-buffer", Vother_window_scroll_buffer,
		      "If this is a live buffer, \\[scroll-other-window] should scroll its window.");
	Vother_window_scroll_buffer = Qnil ();
	DEFVAR_LISP! (c"other-window-scroll-default", Vother_window_scroll_default,
		      "Function that provides the window to scroll by \\[scroll-other-window].\nThe function `other-window-for-scrolling' first tries to use\n`minibuffer-scroll-window' and `other-window-scroll-buffer'.\nBut when both are nil, then by default it uses a neighboring window.\nThis variable is intended to get another default instead of `next-window'.");
	Vother_window_scroll_default = Qnil ();
	DEFVAR_BOOL! (c"auto-window-vscroll", auto_window_vscroll_p,
		      "Non-nil means to automatically adjust `window-vscroll' to view tall lines.");
	auto_window_vscroll_p = true;
	DEFVAR_INT! (c"next-screen-context-lines", next_screen_context_lines,
		      "Number of lines of continuity when scrolling by screenfuls.");
	next_screen_context_lines = 2;
	DEFVAR_LISP! (c"scroll-preserve-screen-position", Vscroll_preserve_screen_position,
		      "Controls if scroll commands move point to keep its screen position unchanged.\n\nA value of nil means point does not keep its screen position except\nat the scroll margin or window boundary respectively.\n\nA value of t means point keeps its screen position if the scroll\ncommand moved it vertically out of the window, e.g. when scrolling\nby full screens.  If point is within `next-screen-context-lines' lines\nfrom the edges of the window, point will typically not keep its screen\nposition when doing commands like `scroll-up-command'/`scroll-down-command'\nand the like.\n\nAny other value means point always keeps its screen position.\nScroll commands should have the `scroll-command' property\non their symbols to be controlled by this variable.");
	Vscroll_preserve_screen_position = Qnil ();
	DEFVAR_LISP! (c"window-point-insertion-type", Vwindow_point_insertion_type,
		      "Insertion type of marker to use for `window-point'.\nSee `marker-insertion-type' for the meaning of the possible values.");
	Vwindow_point_insertion_type = Qnil ();
	DEFSYM! (Qwindow_point_insertion_type, "window-point-insertion-type");
	DEFVAR_LISP! (c"window-buffer-change-functions", Vwindow_buffer_change_functions,
		      "Functions called during redisplay when window buffers have changed.\nThe value should be a list of functions that take one argument.\n\nFunctions specified buffer-locally are called for each window showing\nthe corresponding buffer if and only if that window has been added or\nchanged its buffer since the last redisplay.  This means that functions\nmay be run twice for such a window: Once for the buffer shown in the\nwindow at the time of the last redisplay (provided that buffer is still\nlive) and once for the buffer currently shown in the window.  In either\ncase the window is passed as argument and the respective buffer is made\ntemporarily current.\n\nFunctions specified by the default value are called for each frame if\nat least one window on that frame has been added, deleted or changed\nits buffer since the last redisplay.  In this case the frame is passed\nas argument.");
	Vwindow_buffer_change_functions = Qnil ();
	DEFVAR_LISP! (c"window-size-change-functions", Vwindow_size_change_functions,
		      "Functions called during redisplay when window sizes have changed.\nThe value should be a list of functions that take one argument.\n\nFunctions specified buffer-locally are called for each window showing\nthe corresponding buffer if and only if that window has been added or\nchanged its buffer or its total or body size since the last redisplay.\nIn this case the window is passed as argument and the respective buffer\nis temporarily made current.\n\nFunctions specified by the default value are called for each frame if\nat least one window on that frame has been added or changed its buffer\nor its total or body size since the last redisplay.  In this case the\nframe is passed as argument.\n\nFor instance, to hide the title bar when the frame is maximized, you\ncan add `frame-hide-title-bar-when-maximized' to this variable.");
	Vwindow_size_change_functions = Qnil ();
	DEFVAR_LISP! (c"window-selection-change-functions", Vwindow_selection_change_functions,
		      "Functions called during redisplay when the selected window has changed.\nThe value should be a list of functions that take one argument.\n\nFunctions specified buffer-locally are called for each window showing\nthe corresponding buffer if and only if that window has been selected or\ndeselected since the last redisplay.  In this case the window is passed\nas argument and the respective buffer is temporarily made current.\n\nFunctions specified by the default value are called for each frame if\nthe frame's selected window has changed since the last redisplay.  In\nthis case the frame is passed as argument.");
	Vwindow_selection_change_functions = Qnil ();
	DEFVAR_LISP! (c"window-state-change-functions", Vwindow_state_change_functions,
		      "Functions called during redisplay when the window state changed.\nThe value should be a list of functions that take one argument.\n\nFunctions specified buffer-locally are called for each window showing\nthe corresponding buffer if and only if that window has been added,\nresized, changed its buffer or has been (de-)selected since the last\nredisplay.  In this case the window is passed as argument and the\nrespective buffer is temporarily made current.\n\nFunctions specified by the default value are called for each frame if\nat least one window on that frame has been added, deleted, changed its\nbuffer or its total or body size or the frame has been (de-)selected,\nits selected window has changed or the window state change flag has\nbeen set for this frame since the last redisplay.  In this case the\nframe is passed as argument.");
	Vwindow_state_change_functions = Qnil ();
	DEFVAR_LISP! (c"window-state-change-hook", Vwindow_state_change_hook,
		      "Functions called during redisplay when the window state changed.\nThe value should be a list of functions that take no argument.\n\nThis hook is called during redisplay when at least one window has been\nadded, deleted, (de-)selected, changed its buffer or its total or body\nsize or the window state change flag has been set for at least one\nframe.  This hook is called after all other window change functions\nhave been run and should be used only if a function should react to\nchanges that happened on at least two frames since last redisplay or\nthe function intends to change the window configuration.");
	Vwindow_state_change_hook = Qnil ();
	DEFVAR_LISP! (c"window-configuration-change-hook", Vwindow_configuration_change_hook,
		      "Functions called during redisplay when window configuration has changed.\nThe value should be a list of functions that take no argument.\n\nFunctions specified buffer-locally are called for each window showing\nthe corresponding buffer if at least one window on that frame has been\nadded, deleted or changed its buffer or its total or body size since\nthe last redisplay.  Each call is performed with the window showing\nthe buffer temporarily selected.\n\nFunctions specified by the default value are called for each frame if\nat least one window on that frame has been added, deleted or changed\nits buffer or its total or body size since the last redisplay.  Each\ncall is performed with the frame temporarily selected.");
	Vwindow_configuration_change_hook = Qnil ();
	DEFVAR_LISP! (c"window-restore-killed-buffer-windows", window_restore_killed_buffer_windows,
		      " Control restoring windows whose buffer was killed.\nThis variable specifies how the functions `set-window-configuration' and\n`window-state-put' shall handle a window whose buffer has been killed\nsince the corresponding configuration or state was recorded.  Any such\nwindow may be live -- in which case it shows some other buffer -- or\ndead at the time one of these functions is called.\n\nBy default, `set-window-configuration' leaves the window alone if it is\nlive, while `window-state-put' deletes it.  The following values can be\nused to override the default behavior for dead windows in the case of\n`set-window-configuration' and for dead and live windows in the case of\n`window-state-put'.\n\n - t means to restore the window and show some other buffer in it.\n\n - `delete' means to try to delete the window.\n\n - `dedicated' means to try to delete the window if and only if it is\n   dedicated to its buffer.\n\n - nil, the default, which means that `set-window-configuration' will\n   try to delete the window if and only if it is dedicated to its\n   buffer while `window-state-put' will unconditionally try to delete\n   it.\n\n - a function means to restore the window and show some other buffer in\n   it, like if the value were t, but also to add an entry for that\n   window to a list that will be later passed as argument to that\n   function.\n\nIf a window cannot be deleted (typically, because it is the last window\non its frame), show another buffer in it.\n\nIf the value is a function, it should take three arguments.  The first\nargument specifies the frame whose windows have been restored.  The\nthird argument is the symbol `configuration' if the windows are\nrestored by `set-window-configuration' and the symbol `state' if the\nwindows are restored by `window-state-put'.\n\nThe second argument specifies a list of entries for all windows whose\nprevious buffers have been found dead at the time\n`set-window-configuration' or `window-state-put' tried to restore them\n(minibuffer windows are excluded).  This means that the function\nspecified by this variable may also delete windows which were found to\nbe alive by `set-window-configuration'.\n\nEach entry is a list of six values: the window whose buffer was found\ndead, the dead buffer or its name, the positions of window-start and\nwindow-point of the buffer in that window, the dedicated state of the\nwindow as reported by `window-dedicated-p', and a boolean -- t if the\nwindow was live when `set-window-configuration' tried to restore it,\nand nil otherwise.  ");
	window_restore_killed_buffer_windows = Qnil ();
	DEFVAR_LISP! (c"recenter-redisplay", Vrecenter_redisplay,
		      "Non-nil means `recenter' redraws entire frame.\nIf this option is non-nil, then the `recenter' command with a nil\nargument will redraw the entire frame; the special value `tty' causes\nthe frame to be redrawn only if it is a tty frame.");
	Vrecenter_redisplay = Qtty ();
	DEFVAR_LISP! (c"window-combination-resize", Vwindow_combination_resize,
		      "If t, resize window combinations proportionally.\nIf this variable is nil, splitting a window gets the entire screen space\nfor displaying the new window from the window to split.  Deleting and\nresizing a window preferably resizes one adjacent window only.\n\nIf this variable is t, splitting a window tries to get the space\nproportionally from all windows in the same combination.  This means\nthat one can also split a window that is otherwise too small or of fixed\nsize.  Resizing and deleting a window then proportionally resizes all\nwindows in the same combination.\n\nOther values are reserved for future use.\n\nA specific split operation may ignore the value of this variable if it\nis affected by a non-nil value of `window-combination-limit'.  If you\nwant to use a sequence of `split-window' calls to produce a specific,\npredefined layout of windows on a frame, bind this variable temporarily\nto nil.");
	Vwindow_combination_resize = Qnil ();
	DEFVAR_LISP! (c"window-combination-limit", Vwindow_combination_limit,
		      " If non-nil, splitting a window makes a new parent window.\nThe following values are recognized:\n\nnil means splitting a window will create a new parent window only if the\n    window has no parent window or the window shall become part of a\n    combination orthogonal to the one it is part of.\n\n`window-size' means that splitting a window for displaying a buffer\n    makes a new parent window provided `display-buffer' is supposed to\n    explicitly set the window's size due to the presence of a\n    `window-height' or `window-width' entry in the alist used by\n    `display-buffer'.  Otherwise, this value is handled like nil.\n\n`temp-buffer-resize' means that splitting a window for displaying a\n    temporary buffer via `with-temp-buffer-window' makes a new parent\n    window only if `temp-buffer-resize-mode' is enabled.  Otherwise,\n    this value is handled like nil.\n\n`temp-buffer' means that splitting a window for displaying a temporary\n    buffer via `with-temp-buffer-window' always makes a new parent\n    window.  Otherwise, this value is handled like nil.\n\n`display-buffer' means that splitting a window for displaying a buffer\n    always makes a new parent window.  Since temporary buffers are\n    displayed by the function `display-buffer', this value is stronger\n    than `temp-buffer'.  Splitting a window for other purpose makes a\n    new parent window only if needed.\n\nt means that splitting a window always creates a new parent window.  If\n    all splits behave this way, each frame's window tree is a binary\n    tree and every window but the frame's root window has exactly one\n    sibling.\n\nThe default value is `window-size'.  Other values are reserved for\nfuture use.  ");
	Vwindow_combination_limit = Qwindow_size ();
	DEFVAR_LISP! (c"window-persistent-parameters", Vwindow_persistent_parameters,
		      "Alist of persistent window parameters.\nThis alist specifies which window parameters shall get saved by\n`current-window-configuration' and `window-state-get' and subsequently\nrestored to their previous values by `set-window-configuration' and\n`window-state-put'.\n\nThe car of each entry of this alist is the symbol specifying the\nparameter.  The cdr is one of the following:\n\nnil means the parameter is neither saved by `window-state-get' nor by\n`current-window-configuration'.\n\nt means the parameter is saved by `current-window-configuration' and,\nprovided its WRITABLE argument is nil, by `window-state-get'.\n\nThe symbol `writable' means the parameter is saved unconditionally by\nboth `current-window-configuration' and `window-state-get'.  Do not use\nthis value for parameters without read syntax (like windows or frames).\n\nParameters not saved by `current-window-configuration' or\n`window-state-get' are left alone by `set-window-configuration'\nrespectively are not installed by `window-state-put'.");
	Vwindow_persistent_parameters = list1 (Fcons (Qclone_of (), Qt ()));
	DEFVAR_BOOL! (c"window-resize-pixelwise", window_resize_pixelwise,
		      "Non-nil means resize windows pixelwise.\nThis currently affects the functions: `split-window', `maximize-window',\n`minimize-window', `fit-window-to-buffer' and `fit-frame-to-buffer', and\nall functions that symmetrically resize a parent window.\n\nNote that when a frame's pixel size is not a multiple of the\nframe's character size, at least one window may get resized\npixelwise even if this option is nil.");
	window_resize_pixelwise = false;
	DEFVAR_BOOL! (c"fast-but-imprecise-scrolling", fast_but_imprecise_scrolling,
		      "When non-nil, accelerate scrolling operations.\nThis comes into play when scrolling rapidly over previously\nunfontified buffer regions.  Only those portions of the buffer which\nare actually going to be displayed get fontified.\n\nNote that this optimization can cause the portion of the buffer\ndisplayed after a scrolling operation to be somewhat inaccurate.");
	fast_but_imprecise_scrolling = false;
	DEFVAR_LISP! (c"window-dead-windows-table", window_dead_windows_table,
		      "Hash table of dead windows.\nEach entry in this table maps a window number to a window object.\nEntries are added by `delete-window-internal' and are removed by the\ngarbage collector.");
	window_dead_windows_table = CALLN! (Fmake_hash_table, QCweakness (), Qvalue ());
	DEFVAR_BOOL! (c"window-auto-redraw-on-parameter-change", window_auto_redraw_on_parameter_change,
		      "When non-nil, redraw based on face filters.\nWhen this variable is non-nil, force a potentially expensive redraw when\na window parameter named in a `:window' expression for ':filtered'\nchanges.  This redraw is necessary for correctness; this variable is an\nescape hatch to recover performance in the case that our assumption that\nthese parameter changes are rare does not hold.\n\nYou can also inhibit the automatic redraw for a specific window\nparameter by setting the `:filtered` symbol property of the parameter\nname to `'ignore'.");
	window_auto_redraw_on_parameter_change = true;
	defsubr (&raw mut Sselected_window);
	defsubr (&raw mut Sold_selected_window);
	defsubr (&raw mut Sminibuffer_window);
	defsubr (&raw mut Swindow_minibuffer_p);
	defsubr (&raw mut Swindowp);
	defsubr (&raw mut Swindow_valid_p);
	defsubr (&raw mut Swindow_live_p);
	defsubr (&raw mut Swindow_frame);
	defsubr (&raw mut Sframe_root_window);
	defsubr (&raw mut Sframe_first_window);
	defsubr (&raw mut Sframe_selected_window);
	defsubr (&raw mut Sframe_old_selected_window);
	defsubr (&raw mut Sset_frame_selected_window);
	defsubr (&raw mut Spos_visible_in_window_p);
	defsubr (&raw mut Swindow_line_height);
	defsubr (&raw mut Swindow_buffer);
	defsubr (&raw mut Swindow_old_buffer);
	defsubr (&raw mut Swindow_parent);
	defsubr (&raw mut Swindow_top_child);
	defsubr (&raw mut Swindow_left_child);
	defsubr (&raw mut Swindow_next_sibling);
	defsubr (&raw mut Swindow_prev_sibling);
	defsubr (&raw mut Swindow_combination_limit);
	defsubr (&raw mut Sset_window_combination_limit);
	defsubr (&raw mut Swindow_use_time);
	defsubr (&raw mut Swindow_pixel_width);
	defsubr (&raw mut Swindow_pixel_height);
	defsubr (&raw mut Swindow_old_pixel_width);
	defsubr (&raw mut Swindow_old_pixel_height);
	defsubr (&raw mut Swindow_old_body_pixel_width);
	defsubr (&raw mut Swindow_old_body_pixel_height);
	defsubr (&raw mut Swindow_total_width);
	defsubr (&raw mut Swindow_total_height);
	defsubr (&raw mut Swindow_normal_size);
	defsubr (&raw mut Swindow_new_pixel);
	defsubr (&raw mut Swindow_new_total);
	defsubr (&raw mut Swindow_new_normal);
	defsubr (&raw mut Swindow_pixel_left);
	defsubr (&raw mut Swindow_pixel_top);
	defsubr (&raw mut Swindow_left_column);
	defsubr (&raw mut Swindow_top_line);
	defsubr (&raw mut Sset_window_new_pixel);
	defsubr (&raw mut Sset_window_new_total);
	defsubr (&raw mut Sset_window_new_normal);
	defsubr (&raw mut Swindow_resize_apply);
	defsubr (&raw mut Swindow_resize_apply_total);
	defsubr (&raw mut Swindow_body_height);
	defsubr (&raw mut Swindow_body_width);
	defsubr (&raw mut Swindow_hscroll);
	defsubr (&raw mut Sset_window_hscroll);
	defsubr (&raw mut Swindow_mode_line_height);
	defsubr (&raw mut Swindow_header_line_height);
	defsubr (&raw mut Swindow_tab_line_height);
	defsubr (&raw mut Swindow_right_divider_width);
	defsubr (&raw mut Swindow_bottom_divider_width);
	defsubr (&raw mut Swindow_scroll_bar_width);
	defsubr (&raw mut Swindow_scroll_bar_height);
	defsubr (&raw mut Scoordinates_in_window_p);
	defsubr (&raw mut Swindow_at);
	defsubr (&raw mut Swindow_point);
	defsubr (&raw mut Swindow_old_point);
	defsubr (&raw mut Swindow_start);
	defsubr (&raw mut Swindow_end);
	defsubr (&raw mut Sset_window_point);
	defsubr (&raw mut Sset_window_start);
	defsubr (&raw mut Swindow_dedicated_p);
	defsubr (&raw mut Swindow_lines_pixel_dimensions);
	defsubr (&raw mut Sset_window_dedicated_p);
	defsubr (&raw mut Swindow_display_table);
	defsubr (&raw mut Sset_window_display_table);
	defsubr (&raw mut Snext_window);
	defsubr (&raw mut Sprevious_window);
	defsubr (&raw mut Sget_buffer_window);
	defsubr (&raw mut Sdelete_other_windows_internal);
	defsubr (&raw mut Sdelete_window_internal);
	defsubr (&raw mut Sresize_mini_window_internal);
	defsubr (&raw mut Sset_window_buffer);
	defsubr (&raw mut Srun_window_configuration_change_hook);
	defsubr (&raw mut Srun_window_scroll_functions);
	defsubr (&raw mut Sselect_window);
	defsubr (&raw mut Sforce_window_update);
	defsubr (&raw mut Ssplit_window_internal);
	defsubr (&raw mut Scombine_windows);
	defsubr (&raw mut Suncombine_window);
	defsubr (&raw mut Sscroll_up);
	defsubr (&raw mut Sscroll_down);
	defsubr (&raw mut Sscroll_left);
	defsubr (&raw mut Sscroll_right);
	defsubr (&raw mut Sother_window_for_scrolling);
	defsubr (&raw mut Sminibuffer_selected_window);
	defsubr (&raw mut Srecenter);
	defsubr (&raw mut Swindow_text_width);
	defsubr (&raw mut Swindow_text_height);
	defsubr (&raw mut Smove_to_window_line);
	defsubr (&raw mut Swindow_configuration_p);
	defsubr (&raw mut Swindow_configuration_frame);
	defsubr (&raw mut Sset_window_configuration);
	defsubr (&raw mut Scurrent_window_configuration);
	defsubr (&raw mut Sset_window_margins);
	defsubr (&raw mut Swindow_margins);
	defsubr (&raw mut Sset_window_fringes);
	defsubr (&raw mut Swindow_fringes);
	defsubr (&raw mut Sset_window_scroll_bars);
	defsubr (&raw mut Swindow_scroll_bars);
	defsubr (&raw mut Swindow_vscroll);
	defsubr (&raw mut Sset_window_vscroll);
	defsubr (&raw mut Swindow_configuration_equal_p);
	defsubr (&raw mut Swindow_bump_use_time);
	defsubr (&raw mut Swindow_list);
	defsubr (&raw mut Swindow_list_1);
	defsubr (&raw mut Swindow_prev_buffers);
	defsubr (&raw mut Sset_window_prev_buffers);
	defsubr (&raw mut Swindow_next_buffers);
	defsubr (&raw mut Sset_window_next_buffers);
	defsubr (&raw mut Swindow_parameters);
	defsubr (&raw mut Swindow_parameter);
	defsubr (&raw mut Sset_window_parameter);
	defsubr (&raw mut Swindow_discard_buffer);
	defsubr (&raw mut Swindow_cursor_type);
	defsubr (&raw mut Swindow_cursor_info);
	defsubr (&raw mut Sset_window_cursor_type);
}
