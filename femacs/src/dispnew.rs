// femacs: port of src/dispnew.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct, struct field, local), same control flow, same error model.  C's non-`static`
// definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2/P-52); the `static` helpers stay
// plain `unsafe fn`.  The 13 subrs use `DEFUN!` (PORTING.md 5.1).
//
// **Carried arms.**  `src/dispnew.c` is *not* in
// `docs/baseline/config/cond-eval-verified.txt` (`python3 tools/check-cond-eval.py
// src/dispnew.c` prints `84 conditions, 0 unknown, 0 disagreements [VERIFIED]`, but the list
// is the gate's input and this unit does not own it), so `tools/extract-abi.py` is blind for
// this file: every `#if` keeps both arms live and the ledger demands each arm's tokens.
// PORTING.md P-51/status 3.49 therefore apply: the arm the frozen configuration does not
// compile stays as text under a `#[cfg (feature = "...")]` that names the non-frozen side.
// The frozen macros this file tests are `GLYPH_DEBUG`, `ENABLE_CHECKING`, `PROFILING`,
// `HAVE___EXECUTABLE_START`, `HAVE_ANDROID`, `HAVE_WINDOW_SYSTEM`, `HAVE_EXT_MENU_BAR`,
// `HAVE_EXT_TOOL_BAR`, `WINDOWSNT`, `HAVE_NTGUI`, `HAVE_PGTK`, `HAVE_HAIKU` and `MSDOS`
// (all undefined in `docs/baseline/config/darwin-arm64.macros.tsv`), so those arms are the
// gated ones.  The full list of carried arms is in `docs/ports/dispnew.md`'s Decisions.
//
// **Deleted C text** (PORTING.md 10.10): the forward declarations of C 74-104 are not needed
// in Rust and are not carried; the `#if 0` debugging aid of C 107-119 and the other `#if 0`
// regions are dead in C itself and are recorded only.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_assignments,
         unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens,
         clashing_extern_declarations, unused_labels, unreachable_code, unused_comparisons,
         unnecessary_cast, unused_must_use, overflowing_literals)]

/* C 21-62: the `#include`s, each with C's own header named.  `xwidget.h` (C 43) has no
   mirror; the one name this unit takes from it (`xwidget_end_redisplay`, xwidget.h 26) is
   declared at its use site (PORTING.md P-72 rule 3).  `TERM_HEADER` (C 53) is the platform
   terminal header and is only reached under `HAVE_WINDOW_SYSTEM`; `w32.h` (C 61) and
   `android.h` (C 49) belong to their own gated arms.  */
include!("include/config.rs");                 // C 21: `#include <config.h>`
include!("../lib/include/stdio.rs");           // C 23: `#include "sysstdio.h"`
include!("../lib/include/stdlib.rs");          // C 24: `#include <stdlib.h>`
include!("../lib/include/unistd.in.rs");       // C 25: `#include <unistd.h>`
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>`
include!("../lib/include/limits.in.rs");       // lisp.h's <limits.h> (`INT_MAX`)
include!("../lib/include/errno.rs");           // C 56: `#include <errno.h>`
include!("../lib/include/fpending.rs");        // C 58: `#include <fpending.h>`
include!("../lib/include/signal.rs");          // syssignal.h: `sigaction`, `SIGWINCH`
include!("include/lisp.rs");                   // C 27: `#include "lisp.h"`
include!("include/character.rs");              // dispextern.h 25: `#include "character.h"`
include!("include/itree.rs");                  // buffer.h 27: `#include "itree.h"`
include!("include/buffer.rs");                 // C 32: `#include "buffer.h"`
include!("include/systime.rs");                // termhooks.h: `Time`
include!("include/coding.rs");                 // buffer.h 26: `#include "coding.h"`
include!("include/composite.rs");              // coding.h 284: `#include "composite.h"`
include!("include/dispextern.rs");             // C 30: `#include "dispextern.h"`
include!("include/font.rs");                   // dispextern.h's font names
include!("include/termhooks.rs");              // C 35: `#include "termhooks.h"`
include!("include/cm.rs");                     // C 31: `#include "cm.h"`
include!("include/window.rs");                 // C 36: `#include "window.h"`
include!("include/frame.rs");                  // C 34: `#include "frame.h"`
include!("include/keyboard.rs");               // C 33: `#include "keyboard.h"`
include!("include/commands.rs");               // C 37: `#include "commands.h"`
include!("include/disptab.rs");                // C 38: `#include "disptab.h"`
include!("include/termchar.rs");               // C 28: `#include "termchar.h"`
include!("include/blockinput.rs");             // C 39: `#include "blockinput.h"`
include!("include/syssignal.rs");              // C 40: `#include "syssignal.h"`
include!("include/tparam.rs");                 // C 42: `#include "tparam.h"`
include!("include/pdumper.rs");                // C 44: `#include "pdumper.h"`
/* C 49: `#include "android.h"` (inside C 48's `#ifdef HAVE_ANDROID`).  The mirror glob-imports
   its object module, so this edge is what supplies `android_init_gui` (measured).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/android.rs");                // C 49: `#include "android.h"`
/* C 53: `#include TERM_HEADER`, inside C 52's `#ifdef HAVE_WINDOW_SYSTEM`.  The frozen
   configuration leaves `TERM_HEADER` undefined (C's own `config.h` 2361 `/* #undef TERM_HEADER */`,
   no window system), which is why the port's units carried no edge for it - and the five
   window-system mirrors are where `FRAME_DISPLAY_INFO` and the other term macros live, so every
   `MOUSE_HL_INFO`/`FRAME_RIF`-shaped expansion in this unit's body read
   `cannot find macro FRAME_DISPLAY_INFO in this scope` on `aarch64-linux-android` (measured; the
   `fontset.rs` block is the same five-way gate, C 39 there).  */
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
include!("include/androidterm.rs");            // C 53 (TERM_HEADER): `androidterm.h`

/// `<inttypes.h>` has no mirror (P-26/P-50, the print.rs/doprnt.rs/pdumper.rs shape): the
/// frozen target's `PRIuMAX` is `"ju"` (measured with clang; `intmax_t` is `long long`).
/// C concatenates it into its format strings, and `concat!` expands a `macro_rules!` call
/// where a `const` cannot (P-57), so the name survives as a macro.
macro_rules! PRIuMAX { () => { "ju" } }

/* Structure to pass dimensions around.  Used for character bounding
   boxes, glyph matrix dimensions and alike.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct dim
{
  pub width: c_int,
  pub height: c_int,
}

const _: () = core::assert! (core::mem::size_of::<dim> () == 8);          // clang-measured
const _: () = core::assert! (core::mem::offset_of! (dim, width) == 0);
const _: () = core::assert! (core::mem::offset_of! (dim, height) == 4);

/* C 74-104: the forward declarations (`write_row`, `required_matrix_height`, …).  Rust
   needs no forward declaration for a function defined later in the same unit, so this
   block has no text (PORTING.md 10.10).  */

/* C 106-119: `#if 0 /* Please leave this in as a debugging aid.  */ … #endif` around
   `check_rows`.  `#if 0` is dead in C itself and in the extractor's inventory, so the text
   is recorded in `docs/ports/dispnew.md` and not carried.  */

/* True means SIGWINCH happened when not safe.  */

static mut delayed_size_change: bool = false;

/* A glyph for a space.  `include/dispextern.rs` declares it at the crate root
   (dispextern.h 1338), so the definition lives in a glob-imported module: a second
   crate-root item of the same name would be `E0428` (the `frame_pointer_fns` device,
   `docs/ports/frame.md`).  */

pub mod space_glyph_def {
  use super::*;

  #[unsafe(no_mangle)] pub static mut space_glyph: glyph = unsafe { std::mem::zeroed () };
}
use space_glyph_def::*;

/* C 129-137: `#if defined GLYPH_DEBUG && defined ENABLE_CHECKING` — the counts of allocated
   structures, used to diagnose memory leaks and double frees.  `GLYPH_DEBUG` and
   `ENABLE_CHECKING` are both undefined in the frozen configuration, so the block is carried
   under the feature that names its non-frozen side (P-51).  */

#[cfg (all (feature = "glyph_debug", enable_checking))]
static mut glyph_matrix_count: c_int = 0;

#[cfg (all (feature = "glyph_debug", enable_checking))]
static mut glyph_pool_count: c_int = 0;

/* C 139-232: `#ifdef GLYPH_DEBUG` — the redisplay history ring buffer, its two writers and
   `Fdump_redisplay_history`.  Dead on the frozen target; carried as text (P-51).  */

#[cfg (feature = "glyph_debug")]
#[repr(C)] pub struct redisplay_history { pub trace: [c_char; 512 + 100] }

/* The size of the history buffer.  */

#[cfg (feature = "glyph_debug")]
const REDISPLAY_HISTORY_SIZE: usize = 30;

/* The redisplay history buffer.  */

#[cfg (feature = "glyph_debug")]
static mut redisplay_history: [redisplay_history; REDISPLAY_HISTORY_SIZE] =
    [const { redisplay_history { trace: [0; 512 + 100] } }; REDISPLAY_HISTORY_SIZE];

/* Next free entry in redisplay_history.  */

#[cfg (feature = "glyph_debug")]
static mut history_idx: c_int = 0;

/* A tick that's incremented each time something is added to the
   history.  */

#[cfg (feature = "glyph_debug")]
static mut history_tick: uintmax_t = 0;

/* Add to the redisplay history how window W has been displayed.
   MSG is a trace containing the information how W's glyph matrix
   has been constructed.  */

#[cfg (feature = "glyph_debug")]
unsafe fn add_window_display_history (w: *mut window, msg: *const c_char)
{
  let mut buf: *mut c_char;
  let ptr: *mut c_void = w as *mut c_void;

  if history_idx >= REDISPLAY_HISTORY_SIZE as c_int
    { history_idx = 0; }
  buf = (*core::ptr::addr_of_mut! (redisplay_history))[history_idx as usize].trace.as_mut_ptr ();
  history_idx += 1;

  let tick: uintmax_t = history_tick;
  history_tick += 1;
  snprintf (buf, core::mem::size_of_val (&(*core::ptr::addr_of! (redisplay_history))[0].trace),
	    concat! ("%", PRIuMAX! (), ": window %p %s\n%s").as_ptr () as *const c_char,
	    tick,
	    ptr,
	    if BUFFERP ((*w).contents)
	      && STRINGP (BVAR! (XBUFFER ((*w).contents), name_))
	      { SSDATA (BVAR! (XBUFFER ((*w).contents), name_)) }
	      else { c"???".as_ptr () },
	    msg);
}

/* Add to the redisplay history that frame F has been displayed.
   PAUSED_P means that the update has been interrupted for
   pending input.  */

#[cfg (feature = "glyph_debug")]
unsafe fn add_frame_display_history (f: *mut frame, paused_p: bool)
{
  let mut buf: *mut c_char;
  let ptr: *mut c_void = f as *mut c_void;

  if history_idx >= REDISPLAY_HISTORY_SIZE as c_int
    { history_idx = 0; }
  buf = (*core::ptr::addr_of_mut! (redisplay_history))[history_idx as usize].trace.as_mut_ptr ();
  history_idx += 1;

  let tick: uintmax_t = history_tick;
  history_tick += 1;
  sprintf (buf, concat! ("%", PRIuMAX! (), ": update frame %p%s").as_ptr () as *const c_char,
	   tick,
	   ptr, if paused_p { c" ***paused***".as_ptr () } else { c"".as_ptr () });
}

#[cfg (feature = "glyph_debug")]
DEFUN! ("dump-redisplay-history", Fdump_redisplay_history,
        Sdump_redisplay_history, 0, 0, "",
        doc: "Dump redisplay history to stderr.",
        (),
        {
  let mut i: c_int;

  i = history_idx - 1;
  while i != history_idx
    {
      if i < 0
	{ i = REDISPLAY_HISTORY_SIZE as c_int - 1; }
      fprintf (stderr, c"%s\n".as_ptr (), redisplay_history[i as usize].trace.as_ptr ());
      i -= 1;
    }

  return Qnil ();
});

/* C 234-244: `#if defined PROFILING && !HAVE___EXECUTABLE_START` — the function that comes
   first in the executable and is used to estimate the text start for profiling.  Dead on
   the frozen target; carried as text (P-51).  */

#[cfg (feature = "profiling")]
#[unsafe(no_mangle)] pub unsafe extern "C" fn __executable_start () { emacs_abort (); }

/* Names the mirrors this unit includes do not declare, declared here at their use site
   (PORTING.md P-72 rule 3; the C prototype is quoted per name).  */


/* frame.h 1387-1388: `CHECK_FRAME` is the mirror's (`include/frame.rs` 979, which this unit
   includes); the unit's body-identical copy was redundancy (P-26).  */

/// disptab.h 114-115: `#define SET_GLYPH_FROM_CHAR(glyph, c) SET_GLYPH (glyph, c,
/// DEFAULT_FACE_ID)`.  The disptab mirror stops before the glyph-table block (its own note),
/// so the two `SET_GLYPH_FROM_*` macros C's dispnew.c needs are declared here at their use
/// site.  dispextern.h 414-422's `SET_GLYPH_FROM_GLYPH_CODE` keeps C's `do { … } while (0)`
/// as a block (the `SET_CHAR_GLYPH` shape).
macro_rules! SET_GLYPH_FROM_GLYPH_CODE {
  ($glyph:expr, $gc:expr) => { {
    if CONSP ($gc)
      { SET_GLYPH! ($glyph, XFIXNUM (XCAR ($gc)) as c_int, XFIXNUM (XCDR ($gc)) as c_int); }
    else
      { SET_GLYPH! ($glyph, (XFIXNUM ($gc) & ((1 << CHARACTERBITS) - 1)) as c_int,
		    (XFIXNUM ($gc) >> CHARACTERBITS) as c_int); }
  } };
}


unsafe extern "C" {
  /// C: dispextern.h 3717: `extern unsigned row_hash (struct glyph_row *);` (term.c defines it)
  fn row_hash (row: *mut glyph_row) -> c_uint;
  /// C: dispextern.h 3582: `int estimate_mode_line_height (struct frame *, enum face_id);`
  fn estimate_mode_line_height (f: *mut frame, face_id: face_id) -> c_int;
  /// C: dispextern.h 3813: `int lookup_basic_face (struct window *, struct frame *, int);`
  /// (xfaces.c defines it; xfaces.rs is not in this unit's include list.)
  fn lookup_basic_face (f: *mut frame, frame_: *mut frame, face_id: c_int) -> c_int;
  /// C: dispextern.h 3831: `int merge_faces (struct window *, Lisp_Object, int, int);`
  /// (xfaces.c defines it; xfaces.rs is not in this unit's include list.)
  fn merge_faces (w: *mut window, face_name: Lisp_Object, face_id: c_int,
                  base_face_id: c_int) -> c_int;
  /// C: dispextern.h 3934: `extern void update_begin (struct frame *);`
  fn update_begin (f: *mut frame);
  /// C: dispextern.h 3935: `extern void update_end (struct frame *);`
  fn update_end (f: *mut frame);
  /// C: dispextern.h 3940: `extern void clear_frame (struct frame *);`
  fn clear_frame (f: *mut frame);
  /// C: frame.h 1476: `extern struct frame *decode_live_frame (Lisp_Object);`
  fn decode_live_frame (frame: Lisp_Object) -> *mut frame;
  /// C: frame.h 1480: `extern bool frame_redisplay_p (struct frame *);`
  fn frame_redisplay_p (f: *mut frame) -> bool;
  /// C: dispextern.h 3588: `void mark_window_display_accurate (Lisp_Object, bool);`
  fn mark_window_display_accurate (window: Lisp_Object, flag: bool);
  /// C: dispextern.h 3688: `extern void note_mouse_highlight (struct frame *, int, int);`
  fn note_mouse_highlight (f: *mut frame, x: c_int, y: c_int);
  // (window.h 1211's `decode_any_window` is the window.rs mirror's now; the use-site copy is gone, P-26.)
  /// C: keyboard.h 513-514: `extern void gen_help_event (Lisp_Object, Lisp_Object, Lisp_Object,
  /// Lisp_Object, ptrdiff_t);`
  fn gen_help_event (string: Lisp_Object, frame: Lisp_Object, window: Lisp_Object,
                     object: Lisp_Object, pos: ptrdiff_t);
  /// C: dispextern.h 3614-3616: `extern Lisp_Object help_echo_string, help_echo_window;`
  /// `extern Lisp_Object help_echo_object, previous_help_echo_string;`
  /// `extern ptrdiff_t help_echo_pos;` (keyboard.c defines all five; the mirror does not
  /// carry them).
  static mut help_echo_string: Lisp_Object;
  static mut help_echo_window: Lisp_Object;
  static mut help_echo_object: Lisp_Object;
  static mut previous_help_echo_string: Lisp_Object;
  static mut help_echo_pos: ptrdiff_t;
  /// C: frame.h 1508: `extern struct frame *root_frame (struct frame *f);`
  fn root_frame (f: *mut frame) -> *mut frame;
  /// C: lisp.h 4244: `extern EMACS_INT next_almost_prime (EMACS_INT) ATTRIBUTE_CONST;`
  fn next_almost_prime (n: EMACS_INT) -> EMACS_INT;
  /// C: dispextern.h 3955: `extern void calculate_costs (struct frame *);`
  fn calculate_costs (f: *mut frame);
  /// C: dispextern.h 3780: `void get_tty_size (int, int *, int *);`
  fn get_tty_size (fileno: c_int, width: *mut c_int, height: *mut c_int);
  /// C: frame.h 1506: `extern void adjust_frame_size (struct frame *, int, int, int, bool,
  /// Lisp_Object);`
  fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
                        inhibit: c_int, pretend: bool, parameter: Lisp_Object);
  /// C: frame.h 1789: `void frame_size_history_extra (struct frame *, Lisp_Object, int, int,
  /// int, int, int, int);` (frame.rs defines the same helper for its own unit.)
  fn frame_size_history_extra (f: *mut frame, parameter: Lisp_Object, width: c_int,
                               height: c_int, new_width: c_int, new_height: c_int,
                               new_native_width: c_int, new_native_height: c_int);
  /// C: lisp.h 5371: `extern AVOID fatal (const char *msgid, ...) ATTRIBUTE_FORMAT_PRINTF (1, 2);`
  fn fatal (msgid: *const c_char, ...);
  /// C: <stdlib.h> 3xx: `void exit (int);`
  fn exit (status: c_int) -> !;
  /// C: xterm.c: `extern bool x_display_ok (const char *);`
  fn x_display_ok (display: *const c_char) -> bool;
  /// C: tparam.c: `extern struct terminal *init_tty (const char *, const char *, bool);`
  fn init_tty (name: *const c_char, terminal_type: *const c_char, must_succeed: bool)
    -> *mut terminal;
  /// C: term.c: `extern void create_tty_output (struct frame *);`
  fn create_tty_output (f: *mut frame);
  /// C: sysdep.c: `extern void init_foreground_group (void);`
  fn init_foreground_group ();
  /// C: lisp.h 5322: `extern FILE *emacs_fopen (char const *, char const *);`
  fn emacs_fopen (file: *const c_char, mode: *const c_char) -> *mut FILE;
  /// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);`
  fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal;
  /// C: keyboard.h 500-501: `extern void request_sigio (void);` `extern void unrequest_sigio
  /// (void);`
  fn request_sigio ();
  fn unrequest_sigio ();
  /// C: dispextern.h 3933: `extern void ring_bell (struct frame *);`
  fn ring_bell (f: *mut frame);
  // `putchar` now comes from the `stdio.rs` mirror (2026-10-04), which is its single
  // declaration point; the use-site copy this block carried is gone.
  /// C: lisp.h 5181-5182: `extern int wait_reading_process_output (intmax_t, int, int, bool,
  /// Lisp_Object, struct Lisp_Process *, int);`
  fn wait_reading_process_output (timeout: i64, nsecs: c_int, read_kbd: c_int,
                                  do_display: bool, wait_for_cell: Lisp_Object,
                                  wait_proc: *mut c_void, just_wait_proc: c_int) -> c_int;
  /// C: dispextern.h 3589: `void redisplay_preserve_echo_area (int);`
  fn redisplay_preserve_echo_area (from_where: c_int);
  /// C: lisp.h 5095-5096: `extern bool detect_input_pending (void);`
  /// `extern bool detect_input_pending_run_timers (bool);`
  fn detect_input_pending () -> bool;
  fn detect_input_pending_run_timers (do_display: bool) -> bool;
  /// C: keyboard.h 488/496: `extern int gobble_input (void);`
  /// `extern void swallow_events (bool);`
  fn gobble_input () -> c_int;
  fn swallow_events (do_display: bool);
  /// C: keyboard.c/term.c: `extern bool redisplaying_p;` and
  /// `extern bool dont_resize_frames;` (both are plain globals the mirrors do not carry).
  static mut redisplaying_p: bool;
  static mut dont_resize_frames: bool;
  /// C: dispextern.h 3941-3944: `extern void write_glyphs (struct frame *, struct glyph *, int);`
  /// `extern void insert_glyphs (struct frame *, struct glyph *, int);`
  /// `extern void delete_glyphs (struct frame *, int);`
  /// `extern void clear_end_of_line (struct frame *, int);`
  fn write_glyphs (f: *mut frame, string: *mut glyph, len: c_int);
  fn insert_glyphs (f: *mut frame, string: *mut glyph, len: c_int);
  fn delete_glyphs (f: *mut frame, n: c_int);
  fn clear_end_of_line (f: *mut frame, first_unused_hpos: c_int);
  /// C: dispextern.h 3857-3866: `int scrolling_max_lines_saved (int, int, unsigned *, unsigned *,
  /// int *);` `void scrolling_1 (struct frame *, int, int, int, int *, int *, unsigned *,
  /// unsigned *, int);` (scroll.c defines both; scroll.rs is not in this unit's include list.)
  fn scrolling_max_lines_saved (start: c_int, end: c_int, old_hash: *mut c_uint,
                                new_hash: *mut c_uint, draw_cost: *mut c_int) -> c_int;
  fn scrolling_1 (f: *mut frame, window_size: c_int, unchanged_at_top: c_int,
                  unchanged_at_bottom: c_int, draw_cost: *mut c_int,
                  old_draw_cost: *mut c_int, old_hash: *mut c_uint, new_hash: *mut c_uint,
                  free_at_end_vpos: c_int);
  /// C: dispextern.h 3937: `extern void cursor_to (struct frame *, int, int);`
  fn cursor_to (f: *mut frame, vpos: c_int, hpos: c_int);
  /// C: dispextern.h 3577: `int window_text_bottom_y (struct window *);`
  fn window_text_bottom_y (w: *mut window) -> c_int;
  /// C: termhooks.h 988-989: `void tty_hide_cursor (struct tty_display_info *tty);`
  /// `void tty_show_cursor (struct tty_display_info *tty);`
  fn tty_hide_cursor (tty: *mut tty_display_info);
  fn tty_show_cursor (tty: *mut tty_display_info);

  /// C: frame.h 1911: `extern bool frame_ancestor_p (struct frame *af, struct frame *df);`
  fn frame_ancestor_p (a: *mut frame, b: *mut frame) -> bool;
  /// C: dispextern.h 3574-3575: `void window_box (struct window *, enum glyph_row_area,
  /// int *, int *, int *, int *);`
  fn window_box (w: *mut window, area: glyph_row_area, x: *mut c_int, y: *mut c_int,
                 width: *mut c_int, height: *mut c_int);
  /// C: dispextern.h 3578: `int window_box_width (struct window *, enum glyph_row_area);`
  fn window_box_width (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3576: `int window_box_height (struct window *);`
  fn window_box_height (w: *mut window) -> c_int;
}

/// P-80's member-name form for window.h 392-410's nineteen flags: the mirror stores them in
/// `window::bits` (offset 500, asserted by its own layout keys) and names each bit's shift;
/// C's `w->NAME` is `window_bits! (NAME, w)` and `w->NAME = v` is `window_bits! (NAME, w, v)`.
/// This unit needs two of the nineteen; `window.rs` has no dispatcher yet (a mirror gap the
/// ledger records — this block moves there when it gets one).
macro_rules! window_bits {
  (update_mode_line, $w:expr) => { ((*($w)).bits >> WINDOW_UPDATE_MODE_LINE_SHIFT) & 1 != 0 };
  (update_mode_line, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_UPDATE_MODE_LINE_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_UPDATE_MODE_LINE_SHIFT) }; };
  (cursor_off_p, $w:expr) => { ((*($w)).bits >> WINDOW_CURSOR_OFF_P_SHIFT) & 1 != 0 };
  (cursor_off_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_CURSOR_OFF_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_CURSOR_OFF_P_SHIFT) }; };
  (phys_cursor_on_p, $w:expr) =>
    { ((*($w)).bits >> WINDOW_PHYS_CURSOR_ON_P_SHIFT) & 1 != 0 };
  (phys_cursor_on_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_PHYS_CURSOR_ON_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_PHYS_CURSOR_ON_P_SHIFT) }; };
  (must_be_updated_p, $w:expr) =>
    { ((*($w)).bits >> WINDOW_MUST_BE_UPDATED_P_SHIFT) & 1 != 0 };
  (must_be_updated_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_MUST_BE_UPDATED_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_MUST_BE_UPDATED_P_SHIFT) }; };
  (pseudo_window_p, $w:expr) => { ((*($w)).bits >> WINDOW_PSEUDO_WINDOW_P_SHIFT) & 1 != 0 };
  (pseudo_window_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }; };
  (window_end_valid, $w:expr) => { ((*($w)).bits >> WINDOW_WINDOW_END_VALID_SHIFT) & 1 != 0 };
  (window_end_valid, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_WINDOW_END_VALID_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_WINDOW_END_VALID_SHIFT) }; };
}

/// C: dispextern.h 648-651: `#define CHAR_GLYPH_SPACE_P(FRAME, GLYPH) ((GLYPH).u.ch ==
/// SPACEGLYPH && (GLYPH).face_id == DEFAULT_FACE_ID && (GLYPH).frame == (FRAME))`.
/// `face_id` is a bitfield (P-80), so the read goes through `glyph_bits!`; the dispextern
/// mirror stops before this macro.
macro_rules! CHAR_GLYPH_SPACE_P {
  ($frame:expr, $glyph:expr) => {
    (($glyph).u.ch == SPACEGLYPH as c_uint
     && glyph_bits! (face_id, $glyph) == DEFAULT_FACE_ID
     && ($glyph).frame == ($frame) as *mut c_void)
  };
}


// dispextern.h 286-287: C's `#define BYTEPOS(POS) (POS).bytepos` and
// `#define CHARPOS(POS) (POS).charpos`, both usable as an assignment target the way C's
// macro is (`MATRIX_ROW_START_CHARPOS! (row) += delta`).  dispextern.rs keeps only the
// MATRIX_ROW_* pair, so the two-element pair is declared here at its use site.
macro_rules! BYTEPOS { ($pos:expr) => { ($pos).bytepos }; }
macro_rules! CHARPOS { ($pos:expr) => { ($pos).charpos }; }

/// frame.h 372's `fonts_changed` flag is packed into `frame::bits1` (the mirror names the
/// shift).  `frame.rs` defines this one-line accessor for its own unit, which this unit does
/// not include (C's dispnew.c does not include frame.c either), so the line is repeated here
/// and the ledger records the gap.
/// frame.h 494, 398: the two more members this unit reads (`after_make_frame`,
/// `garbaged`); `frame.rs` defines them for its own unit, which this unit does not include.
macro_rules! after_make_frame {
  ($f:expr) => { (*$f).bits1 >> FRAME_AFTER_MAKE_FRAME_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    (*$f).bits1 = if $v { (*$f).bits1 | (1 << FRAME_AFTER_MAKE_FRAME_SHIFT) }
      else { (*$f).bits1 & !(1 << FRAME_AFTER_MAKE_FRAME_SHIFT) } };
}
macro_rules! garbaged {
  ($f:expr) => { (*$f).bits1 >> FRAME_GARBAGED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    (*$f).bits1 = if $v { (*$f).bits1 | (1 << FRAME_GARBAGED_SHIFT) }
      else { (*$f).bits1 & !(1 << FRAME_GARBAGED_SHIFT) } };
}

/* frame.h 1174-1176: `SET_FRAME_GARBAGED` is a three-term comma expression in C; the port keeps
   the same three effects in the same order.  The mirror's copy (`include/frame.rs` 947) cannot
   expand here - it passes `*mut frame` where this tree declares `fset_redisplay (*mut c_void)`
   (E0308; the cast below is the working spelling) - so the unit keeps its own, reported to the
   mirror's owner.  */

/// frame.h 1804-1814: `FRAME_PIXEL_TO_TEXT_WIDTH`/`FRAME_PIXEL_TO_TEXT_HEIGHT`
/// (`frame.rs` defines them for its own unit, which this unit does not include).
macro_rules! FRAME_PIXEL_TO_TEXT_WIDTH {
  ($f:expr, $width:expr) => {
    ($width) - FRAME_SCROLL_BAR_AREA_WIDTH ($f) - FRAME_TOTAL_FRINGE_WIDTH ($f)
    - 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)
  };
}
macro_rules! FRAME_PIXEL_TO_TEXT_HEIGHT {
  ($f:expr, $height:expr) => {
    ($height) - FRAME_MARGIN_HEIGHT ($f) - FRAME_SCROLL_BAR_AREA_HEIGHT ($f)
    - 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)
  };
}

/// frame.h 526, 331 (`new_size_p`, `glyphs_initialized_p`).  `frame.rs` defines these accessors
/// for its own unit, which this unit does not include, so the one this unit writes is
/// repeated here (the ledger records the gap).
macro_rules! new_size_p {
  ($f:expr) => { (*$f).bits2 >> FRAME_NEW_SIZE_P_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    (*$f).bits2 = if $v { (*$f).bits2 | (1u32 << FRAME_NEW_SIZE_P_SHIFT) }
      else { (*$f).bits2 & !(1u32 << FRAME_NEW_SIZE_P_SHIFT) } };
}
macro_rules! glyphs_initialized_p {
  ($f:expr) => { (*$f).bits1 >> FRAME_GLYPHS_INITIALIZED_P_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    (*$f).bits1 = if $v { (*$f).bits1 | (1 << FRAME_GLYPHS_INITIALIZED_P_SHIFT) }
      else { (*$f).bits1 & !(1 << FRAME_GLYPHS_INITIALIZED_P_SHIFT) } };
}

macro_rules! fonts_changed {
  ($f:expr) => { (*$f).bits1 >> FRAME_FONTS_CHANGED_SHIFT & 1 != 0 };
}

/// C: xwidget.h 26 and 238: `#ifdef HAVE_XWIDGETS extern void xwidget_end_redisplay (struct
/// window *, struct glyph_matrix *); #else INLINE void xwidget_end_redisplay (struct window
/// *w, struct glyph_matrix *m) {} #endif`.  xwidget.h has no mirror at all and HAVE_XWIDGETS
/// is undef in the frozen configuration, so C's call *is* that empty inline body (which is
/// why the reference object has no `xwidget_end_redisplay` reference); the port keeps the
/// call site and gives it the same empty expansion (P-72 rule 3).
macro_rules! xwidget_end_redisplay {
  ($w:expr, $m:expr) => { { let w = $w; let m = $m; let _ = (w, m); } };
}

/***********************************************************************
			    Glyph Matrices
 ***********************************************************************/

/* Allocate and return a glyph_matrix structure.  POOL is the glyph
   pool from which memory for the matrix should be allocated, or null
   for window-based redisplay where no glyph pools are used.  The
   member `pool' of the glyph matrix structure returned is set to
   POOL, the structure is otherwise zeroed.  */

unsafe fn new_glyph_matrix (pool: *mut glyph_pool) -> *mut glyph_matrix
{
  let mut result: *mut glyph_matrix =
    xzalloc (core::mem::size_of::<glyph_matrix> ()) as *mut glyph_matrix;

  #[cfg (all (feature = "glyph_debug", enable_checking))]
  {
    /* Increment number of allocated matrices.  This count is used
       to detect memory leaks.  */
    glyph_matrix_count += 1;
  }

  /* Set pool and return.  */
  (*result).pool = pool;
  return result;
}


/* Free glyph matrix MATRIX.  Passing in a null MATRIX is allowed.

   If GLYPH_DEBUG and ENABLE_CHECKING are in effect, the global counter
   glyph_matrix_count is decremented when a matrix is freed.  If the count
   gets negative, more structures were freed than allocated, i.e. one matrix
   was freed more than once or a bogus pointer was passed to this function.

   If MATRIX->pool is null, this means that the matrix manages its own
   glyph memory---this is done for matrices on X frames.  Freeing the
   matrix also frees the glyph memory in this case.  */

unsafe fn free_glyph_matrix (matrix: *mut glyph_matrix)
{
  if !matrix.is_null ()
    {
      let mut i: c_int;

      #[cfg (all (feature = "glyph_debug", enable_checking))]
      {
	/* Detect the case that more matrices are freed than were
	   allocated.  */
	glyph_matrix_count -= 1;
	eassert! (glyph_matrix_count >= 0);
      }

      /* Free glyph memory if MATRIX owns it.  */
      if (*matrix).pool.is_null ()
	{
	  i = 0;
	  while i < (*matrix).rows_allocated as c_int
	    {
	      xfree ((*(*matrix).rows.add (i as usize)).glyphs [LEFT_MARGIN_AREA as usize]
		     as *mut c_void);
	      i += 1;
	    }
	}

      /* Free row structures and the matrix itself.  */
      xfree ((*matrix).rows as *mut c_void);
      xfree (matrix as *mut c_void);
    }
}


/* Return the number of glyphs to reserve for a marginal area of
   window W.  TOTAL_GLYPHS is the number of glyphs in a complete
   display line of window W.  MARGIN gives the width of the marginal
   area in canonical character units.  */

unsafe fn margin_glyphs_to_reserve (w: *mut window, total_glyphs: c_int,
					   margin: c_int) -> c_int
{
  if margin > 0
    {
      let width: c_int = (*w).total_cols;
      let mut d: c_double = std::cmp::max (0, margin) as c_double;
      d = f64::min ((width / 2 - 1) as c_double, d);
      /* Since MARGIN is positive, we cannot possibly have less than
	 one glyph for the marginal area.  */
      return std::cmp::max (1, (total_glyphs as c_double / width as c_double * d) as c_int);
    }
  return 0;
}

/* Return true if ROW's hash value is correct.
   Optimized away if ENABLE_CHECKING is not defined.  */

unsafe fn verify_row_hash (row: *mut glyph_row) -> bool
{
  return (*row).hash == row_hash (row);
}

/* Adjust glyph matrix MATRIX on window W or on a frame to changed
   window sizes.

   W is null if the function is called for a frame glyph matrix.
   Otherwise it is the window MATRIX is a member of.  X and Y are the
   indices of the first column and row of MATRIX within the frame
   matrix, if such a matrix exists.  They are zero for purely
   window-based redisplay.  DIM is the needed size of the matrix.

   In window-based redisplay, where no frame matrices exist, glyph
   matrices manage their own glyph storage.  Otherwise, they allocate
   storage from a common frame glyph pool which can be found in
   MATRIX->pool.

   The reason for this memory management strategy is to avoid complete
   frame redraws if possible.  When we allocate from a common pool, a
   change of the location or size of a sub-matrix within the pool
   requires a complete redisplay of the frame because we cannot easily
   make sure that the current matrices of all windows still agree with
   what is displayed on the screen.  While this is usually fast, it
   leads to screen flickering.  */

unsafe fn adjust_glyph_matrix (w: *mut window, matrix: *mut glyph_matrix,
				      x: c_int, y: c_int, dim: dim)
{
  let mut i: c_int;
  let mut new_rows: c_int;
  let mut marginal_areas_changed_p: bool = false;
  let mut tab_line_changed_p: bool = false;
  let mut tab_line_p: bool = false;
  let mut header_line_changed_p: bool = false;
  let mut header_line_p: bool = false;
  let mut left: c_int = -1;
  let mut right: c_int = -1;
  let mut window_width: c_int = -1;
  let mut window_height: c_int = -1;

  /* See if W had a header line that has disappeared now, or vice versa.
     Get W's size.  */
  if !w.is_null ()
    {
      window_box (w, ANY_AREA, core::ptr::null_mut (), core::ptr::null_mut (),
		  &mut window_width, &mut window_height);

      tab_line_p = window_wants_tab_line (w);
      tab_line_changed_p = tab_line_p != glyph_matrix_bits! (tab_line_p, matrix);

      header_line_p = window_wants_header_line (w);
      header_line_changed_p = header_line_p != glyph_matrix_bits! (header_line_p, matrix);
    }
  glyph_matrix_bits! (tab_line_p, matrix, tab_line_p);
  glyph_matrix_bits! (header_line_p, matrix, header_line_p);

  /* If POOL is null, MATRIX is a window matrix for window-based redisplay.
     Do nothing if MATRIX' size, position, vscroll, and marginal areas
     haven't changed.  This optimization is important because preserving
     the matrix means preventing redisplay.  */
  eassume! (!w.is_null () || !(*matrix).pool.is_null ());
  if (*matrix).pool.is_null ()
    {
      left = margin_glyphs_to_reserve (w, dim.width, (*w).left_margin_cols);
      right = margin_glyphs_to_reserve (w, dim.width, (*w).right_margin_cols);
      eassert! (left >= 0 && right >= 0);
      marginal_areas_changed_p = (left != (*matrix).left_margin_glyphs
				  || right != (*matrix).right_margin_glyphs);

      if !marginal_areas_changed_p
	  && !fonts_changed! (XFRAME ((*w).frame))
	  && !tab_line_changed_p
	  && !header_line_changed_p
	  && (*matrix).window_pixel_left == WINDOW_LEFT_PIXEL_EDGE (w)
	  && (*matrix).window_pixel_top == WINDOW_TOP_PIXEL_EDGE (w)
	  && (*matrix).window_height == window_height
	  && (*matrix).window_vscroll == (*w).vscroll
	  && (*matrix).window_width == window_width
	{ return; }
    }

  /* Enlarge MATRIX->rows if necessary.  New rows are cleared.  */
  if ((*matrix).rows_allocated < dim.height as ptrdiff_t)
    {
      let old_alloc: c_int = (*matrix).rows_allocated as c_int;
      new_rows = dim.height - (*matrix).rows_allocated as c_int;
      (*matrix).rows = xpalloc ((*matrix).rows as *mut c_void,
				core::ptr::addr_of_mut! ((*matrix).rows_allocated),
				new_rows as ptrdiff_t, INT_MAX as ptrdiff_t,
				core::mem::size_of::<glyph_row> () as ptrdiff_t)
	as *mut glyph_row;
      /* C's `memset` counts *bytes*; `write_bytes` counts `glyph_row`s and multiplies by
	 `size_of::<glyph_row> ()` itself, so the destination goes through a `u8` pointer and C's
	 byte expression stays as it is (P-93's shape, which P-114 extends to the `memset` family).
	 Handing the byte count to the row pointer wrote 256 times too much - the dump died at this
	 statement with a 1,572,864-byte ASan report (docs/status.md 4.425/4.426).  */
      std::ptr::write_bytes ((*matrix).rows.add (old_alloc as usize) as *mut u8, 0,
			     ((*matrix).rows_allocated as c_int - old_alloc) as usize
			     * core::mem::size_of::<glyph_row> ());
      /* A diagnostic switch (PORTING.md P-115's shape): the two operands of the clear below and
	 the element size, so a size disagreement between this call site and the allocator is
	 visible in one line.  Off unless the variable is set.  */
      if unsafe { !getenv (c"FEMACS_XPALLOC_TRACE".as_ptr ()).is_null () }
	{
	  fprintf (stderr, c"adjust_glyph_matrix: sizeof(glyph_row)=%d old_alloc=%d rows_allocated=%d dim.height=%d\n".as_ptr (),
		   core::mem::size_of::<glyph_row> () as c_int, old_alloc,
		   (*matrix).rows_allocated as c_int, dim.height);
	}
    }
  else
    { new_rows = 0; }

  /* If POOL is not null, MATRIX is a frame matrix or a window matrix
     on a frame not using window-based redisplay.  Set up pointers for
     each row into the glyph pool.  */
  if !(*matrix).pool.is_null ()
    {
      eassert! (!(*(*matrix).pool).glyphs.is_null ());

      if !w.is_null ()
	{
	  left = margin_glyphs_to_reserve (w, dim.width,
					   (*w).left_margin_cols);
	  right = margin_glyphs_to_reserve (w, dim.width,
					    (*w).right_margin_cols);
	}
      else
	{ left = 0; right = 0; }

      i = 0;
      while i < dim.height
	{
	  let row: *mut glyph_row = (*matrix).rows.add (i as usize);

	  (*row).glyphs [LEFT_MARGIN_AREA as usize]
	    = (*(*matrix).pool).glyphs
	      .offset (((y + i) * (*(*matrix).pool).ncolumns + x) as isize);

	  if w.is_null ()
	      || (row == (*matrix).rows.offset (dim.height as isize - 1)
		  && window_wants_mode_line (w))
	      || (row == (*matrix).rows && glyph_matrix_bits! (tab_line_p, matrix))
	      || (row == (*matrix).rows
		  && !glyph_matrix_bits! (tab_line_p, matrix)
		  && glyph_matrix_bits! (header_line_p, matrix))
	      || (row == (*matrix).rows.offset (1)
		  && glyph_matrix_bits! (tab_line_p, matrix)
		  && glyph_matrix_bits! (header_line_p, matrix))
	    {
	      (*row).glyphs [TEXT_AREA as usize] = (*row).glyphs [LEFT_MARGIN_AREA as usize];
	      (*row).glyphs [RIGHT_MARGIN_AREA as usize]
		= (*row).glyphs [TEXT_AREA as usize].offset (dim.width as isize);
	      (*row).glyphs [LAST_AREA as usize]
		= (*row).glyphs [RIGHT_MARGIN_AREA as usize];
	    }
	  else
	    {
	      (*row).glyphs [TEXT_AREA as usize]
		= (*row).glyphs [LEFT_MARGIN_AREA as usize].offset (left as isize);
	      (*row).glyphs [RIGHT_MARGIN_AREA as usize]
		= (*row).glyphs [TEXT_AREA as usize]
		  .offset ((dim.width - left - right) as isize);
	      /* Leave room for a border glyph.  */
	      if !FRAME_WINDOW_P (XFRAME ((*w).frame))
		  && !WINDOW_RIGHTMOST_P! (w)
		  && right > 0
		{ (*row).glyphs [RIGHT_MARGIN_AREA as usize]
		    = (*row).glyphs [RIGHT_MARGIN_AREA as usize].offset (-1); }
	      (*row).glyphs [LAST_AREA as usize]
		= (*row).glyphs [LEFT_MARGIN_AREA as usize].offset (dim.width as isize);
	    }
	  i += 1;
	}

      (*matrix).left_margin_glyphs = left;
      (*matrix).right_margin_glyphs = right;
    }
  else
    {
      /* If MATRIX->pool is null, MATRIX is responsible for managing
	 its own memory.  It is a window matrix for window-based redisplay.
	 Allocate glyph memory from the heap.  */
      if dim.width > (*matrix).matrix_w
	  || new_rows != 0
	  || tab_line_changed_p
	  || header_line_changed_p
	  || marginal_areas_changed_p
	{
	  let mut row: *mut glyph_row = (*matrix).rows;
	  let end: *mut glyph_row = row.offset ((*matrix).rows_allocated as isize);

	  while row < end
	    {
	      /* Realloc if matrix got wider or taller (bug#77961).  */
	      if dim.width > (*matrix).matrix_w || new_rows != 0
		{
		  xfree ((*row).glyphs [LEFT_MARGIN_AREA as usize] as *mut c_void);
		  (*row).glyphs [LEFT_MARGIN_AREA as usize] = core::ptr::null_mut ();
		  /* We actually need to clear only the 'frame' member, but
		     it's easier to clear everything.  */
		  (*row).glyphs [LEFT_MARGIN_AREA as usize]
		    = xcalloc (dim.width as usize, core::mem::size_of::<glyph> ())
		      as *mut glyph;
		}

	      if (row == (*matrix).rows.offset (dim.height as isize - 1)
		   /* The mode line, if displayed, never has marginal
                      areas.  */
		   && !(!w.is_null () && window_wants_mode_line (w)))
		  || (row == (*matrix).rows && glyph_matrix_bits! (tab_line_p, matrix))
		  || (row == (*matrix).rows
		      && !glyph_matrix_bits! (tab_line_p, matrix)
		      && glyph_matrix_bits! (header_line_p, matrix))
		  || (row == (*matrix).rows.offset (1)
		      && glyph_matrix_bits! (tab_line_p, matrix)
		      && glyph_matrix_bits! (header_line_p, matrix))
		{
		  (*row).glyphs [TEXT_AREA as usize] = (*row).glyphs [LEFT_MARGIN_AREA as usize];
		  (*row).glyphs [RIGHT_MARGIN_AREA as usize]
		    = (*row).glyphs [TEXT_AREA as usize].offset (dim.width as isize);
		  (*row).glyphs [LAST_AREA as usize]
		    = (*row).glyphs [RIGHT_MARGIN_AREA as usize];
		}
	      else
		{
		  (*row).glyphs [TEXT_AREA as usize]
		    = (*row).glyphs [LEFT_MARGIN_AREA as usize].offset (left as isize);
		  (*row).glyphs [RIGHT_MARGIN_AREA as usize]
		    = (*row).glyphs [TEXT_AREA as usize]
		      .offset ((dim.width - left - right) as isize);
		  (*row).glyphs [LAST_AREA as usize]
		    = (*row).glyphs [LEFT_MARGIN_AREA as usize].offset (dim.width as isize);
		}
	      row = row.offset (1);
	    }
	}

      eassert! (left >= 0 && right >= 0);
      (*matrix).left_margin_glyphs = left;
      (*matrix).right_margin_glyphs = right;

      /* If we are resizing a window, make sure the previous mode-line
	 row of the window's current matrix is no longer marked as such.  */
      if !w.is_null () && (matrix as *mut c_void) == (*w).current_matrix
	  && (*matrix).nrows > 0
	  && dim.height != (*matrix).nrows
	  && (*matrix).nrows as ptrdiff_t <= (*matrix).rows_allocated
	{ glyph_row_bits! (mode_line_p, MATRIX_MODE_LINE_ROW! (matrix), false); }
    }

  /* Number of rows to be used by MATRIX.  */
  (*matrix).nrows = dim.height;
  eassert! ((*matrix).nrows >= 0);

  if !w.is_null ()
    {
      if (matrix as *mut c_void) == (*w).current_matrix
	{
	  /* Mark rows in a current matrix of a window as not having
	     valid contents.  It's important to not do this for
	     desired matrices.  When Emacs starts, it may already be
	     building desired matrices when this function runs.  */
	  if window_width < 0
	    { window_width = window_box_width (w, -1); }

	  /* Optimize the case that only the height has changed (C-x 2,
	     upper window).  Invalidate all rows that are no longer part
	     of the window.  */
	  if !marginal_areas_changed_p
	      && !tab_line_changed_p
	      && !header_line_changed_p
	      && new_rows == 0
	      && dim.width == (*matrix).matrix_w
	      && (*matrix).window_pixel_left == WINDOW_LEFT_PIXEL_EDGE (w)
	      && (*matrix).window_pixel_top == WINDOW_TOP_PIXEL_EDGE (w)
	      && (*matrix).window_width == window_width
	    {
	      /* Find the last row in the window.  */
	      i = 0;
	      while i < (*matrix).nrows
		    && glyph_row_bits! (enabled_p, (*matrix).rows.offset (i as isize))
		{
		  if MATRIX_ROW_BOTTOM_Y! ((*matrix).rows.offset (i as isize)) >= window_height
		    {
		      i += 1;
		      break;
		    }
		  i += 1;
		}

	      /* Window end is invalid, if inside of the rows that
		 are invalidated below.  */
	      if (*w).window_end_vpos >= i
		{ window_bits! (window_end_valid, w, false); }

	      while i < (*matrix).nrows
		{
		  glyph_row_bits! (enabled_p, (*matrix).rows.offset (i as isize), false);
		  i += 1;
		}
	    }
	  else
	    {
	      i = 0;
	      while i < (*matrix).nrows
		{
		  glyph_row_bits! (enabled_p, (*matrix).rows.offset (i as isize), false);
		  i += 1;
		}
	    }
	  /* We've disabled the mode-line row, so force redrawing of
	     the mode line, if any, since otherwise it will remain
	     disabled in the current matrix, and expose events won't
	     redraw it.  */
	  if window_wants_mode_line (w)
	    { window_bits! (update_mode_line, w, true); }
	}
      else if (matrix as *mut c_void) == (*w).desired_matrix
	{
	  /* Rows in desired matrices always have to be cleared;
	     redisplay expects this is the case when it runs, so it
	     had better be the case when we adjust matrices between
	     redisplays.  */
	  i = 0;
	  while i < (*matrix).nrows
	    {
	      glyph_row_bits! (enabled_p, (*matrix).rows.offset (i as isize), false);
	      i += 1;
	    }
	}
    }


  /* Remember last values to be able to optimize frame redraws.  */
  (*matrix).matrix_x = x;
  (*matrix).matrix_y = y;
  (*matrix).matrix_w = dim.width;
  (*matrix).matrix_h = dim.height;

  /* Record the top y location and height of W at the time the matrix
     was last adjusted.  This is used to optimize redisplay above.  */
  if !w.is_null ()
    {
      (*matrix).window_pixel_left = WINDOW_LEFT_PIXEL_EDGE (w);
      (*matrix).window_pixel_top = WINDOW_TOP_PIXEL_EDGE (w);
      (*matrix).window_height = window_height;
      (*matrix).window_width = window_width;
      (*matrix).window_vscroll = (*w).vscroll;
    }
}

/* Reverse the contents of rows in MATRIX between START and END.  The
   contents of the row at END - 1 end up at START, END - 2 at START +
   1 etc.  This is part of the implementation of rotate_matrix (see
   below).  */

unsafe fn reverse_rows (matrix: *mut glyph_matrix, start: c_int, end: c_int)
{
  let mut i: c_int;
  let mut j: c_int;

  i = start; j = end - 1;
  while i < j
    {
      /* Non-ISO HP/UX compiler doesn't like auto struct
	 initialization.  */
      let mut temp: glyph_row;
      temp = *(*matrix).rows.add (i as usize);
      *(*matrix).rows.add (i as usize) = *(*matrix).rows.add (j as usize);
      *(*matrix).rows.add (j as usize) = temp;
      i += 1; j -= 1;
    }
}


/* Rotate the contents of rows in MATRIX in the range FIRST .. LAST -
   1 by BY positions.  BY < 0 means rotate left, i.e. towards lower
   indices.  (Note: this does not copy glyphs, only glyph pointers in
   row structures are moved around).

   The algorithm used for rotating the vector was, I believe, first
   described by Kernighan.  See the vector R as consisting of two
   sub-vectors AB, where A has length BY for BY >= 0.  The result
   after rotating is then BA.  Reverse both sub-vectors to get ArBr
   and reverse the result to get (ArBr)r which is BA.  Similar for
   rotating right.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn rotate_matrix (matrix: *mut glyph_matrix,
							     first: c_int, last: c_int,
							     mut by: c_int)
{
  if by < 0
    {
      /* Up (rotate left, i.e. towards lower indices).  */
      by = -by;
      reverse_rows (matrix, first, first + by);
      reverse_rows (matrix, first + by, last);
      reverse_rows (matrix, first, last);
    }
  else if by > 0
    {
      /* Down (rotate right, i.e. towards higher indices).  */
      reverse_rows (matrix, last - by, last);
      reverse_rows (matrix, first, last - by);
      reverse_rows (matrix, first, last);
    }
}


/* Increment buffer positions in glyph rows of MATRIX.  Do it for rows
   with indices START <= index < END.  Increment positions by DELTA/
   DELTA_BYTES.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn increment_matrix_positions
  (matrix: *mut glyph_matrix, mut start: c_int, end: c_int, delta: ptrdiff_t,
   delta_bytes: ptrdiff_t)
{
  /* Check that START and END are reasonable values.  */
  eassert! (start >= 0 && start <= (*matrix).nrows);
  eassert! (end >= 0 && end <= (*matrix).nrows);
  eassert! (start <= end);

  while start < end
    {
      increment_row_positions ((*matrix).rows.offset (start as isize), delta, delta_bytes);
      start += 1;
    }
}


/* Clear the enable_p flags in a range of rows in glyph matrix MATRIX.
   START and END are the row indices of the first and last + 1 row to clear.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn clear_glyph_matrix_rows
  (matrix: *mut glyph_matrix, mut start: c_int, end: c_int)
{
  eassert! (start <= end);
  eassert! (start >= 0 && (start < (*matrix).nrows
			   /* matrix->nrows can be 0 for the initial frame.  */
			   || ((*matrix).nrows == 0)));
  eassert! (end >= 0 && end <= (*matrix).nrows);

  while start < end
    {
      glyph_row_bits! (enabled_p, (*matrix).rows.offset (start as isize), false);
      start += 1;
    }
}


/* Clear MATRIX.

   Empty all rows in MATRIX by clearing their enabled_p flags.
   The function prepare_desired_row will eventually really clear a row
   when it sees one with a false enabled_p flag.

   Reset update hints to default values.  The only update hint
   currently present is the flag MATRIX->no_scrolling_p.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn clear_glyph_matrix (matrix: *mut glyph_matrix)
{
  if !matrix.is_null ()
    {
      clear_glyph_matrix_rows (matrix, 0, (*matrix).nrows);
      glyph_matrix_bits! (no_scrolling_p, matrix, false);
    }
}


/* Shift part of the glyph matrix MATRIX of window W up or down.
   Increment y-positions in glyph rows between START and END by DY,
   and recompute their visible height.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn shift_glyph_matrix
  (w: *mut window, matrix: *mut glyph_matrix, mut start: c_int, end: c_int, dy: c_int)
{
  let mut min_y: c_int;
  let mut max_y: c_int;

  eassert! (start <= end);
  eassert! (start >= 0 && start < (*matrix).nrows);
  eassert! (end >= 0 && end <= (*matrix).nrows);

  min_y = WINDOW_TAB_LINE_HEIGHT! (w) + WINDOW_HEADER_LINE_HEIGHT! (w);
  max_y = WINDOW_BOX_HEIGHT_NO_MODE_LINE! (w);

  while start < end
    {
      let row: *mut glyph_row = (*matrix).rows.offset (start as isize);

      (*row).y += dy;
      (*row).visible_height = (*row).height;

      if (*row).y < min_y
	{ (*row).visible_height -= min_y - (*row).y; }
      if (*row).y + (*row).height > max_y
	{ (*row).visible_height -= (*row).y + (*row).height - max_y; }
      if glyph_row_bits! (fringe_bitmap_periodic_p, row)
	{ glyph_row_bits! (redraw_fringe_bitmaps_p, row, true); }
      start += 1;
    }
}

/* Mark all rows in current matrices of frame F as invalid.  Marking
   invalid is done by setting enabled_p to zero for all rows in a
   current matrix.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn clear_current_matrices (f: *mut frame)
{
  /* Clear frame current matrix, if we have one.  */
  if !(*f).current_matrix.is_null ()
    { clear_glyph_matrix ((*f).current_matrix as *mut glyph_matrix); }

  #[cfg (feature = "window_system_no_ext_menu_bar")]
  {
  /* Clear the matrix of the menu bar window, if such a window exists.
     The menu bar window is currently used to display menus on X when
     no toolkit support is compiled in.  */
  if WINDOWP ((*f).menu_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).menu_bar_window)).current_matrix
			  as *mut glyph_matrix); }
  }

  #[cfg (feature = "have_window_system")]
  {
  /* Clear the matrix of the tab-bar window, if any.  */
  if WINDOWP ((*f).tab_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).tab_bar_window)).current_matrix
			  as *mut glyph_matrix); }
  }

  #[cfg (feature = "window_system_no_ext_tool_bar")]
  {
  /* Clear the matrix of the tool-bar window, if any.  */
  if WINDOWP ((*f).tool_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).tool_bar_window)).current_matrix
			  as *mut glyph_matrix); }
  }

  /* Clear current window matrices.  */
  eassert! (WINDOWP (FRAME_ROOT_WINDOW (f)));
  clear_window_matrices (XWINDOW (FRAME_ROOT_WINDOW (f)), false);
}


/* Clear out all display lines of F for a coming redisplay.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn clear_desired_matrices (f: *mut frame)
{
  if !(*f).desired_matrix.is_null ()
    { clear_glyph_matrix ((*f).desired_matrix as *mut glyph_matrix); }

  #[cfg (feature = "window_system_no_ext_menu_bar")]
  {
  if WINDOWP ((*f).menu_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).menu_bar_window)).desired_matrix
			  as *mut glyph_matrix); }
  }

  #[cfg (feature = "have_window_system")]
  {
  if WINDOWP ((*f).tab_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).tab_bar_window)).desired_matrix
			  as *mut glyph_matrix); }
  }

  #[cfg (feature = "window_system_no_ext_tool_bar")]
  {
  if WINDOWP ((*f).tool_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).tool_bar_window)).desired_matrix
			  as *mut glyph_matrix); }
  }

  /* Do it for window matrices.  */
  eassert! (WINDOWP (FRAME_ROOT_WINDOW (f)));
  clear_window_matrices (XWINDOW (FRAME_ROOT_WINDOW (f)), true);
}


/* Clear matrices in window tree rooted in W.  If DESIRED_P,
   clear desired matrices, otherwise clear current matrices.  */

unsafe fn clear_window_matrices (w: *mut window, desired_p: bool)
{
  let mut w: *mut window = w;

  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{ clear_window_matrices (XWINDOW ((*w).contents), desired_p); }
      else
	{
	  if desired_p
	    { clear_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix); }
	  else
	    {
	      clear_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);
	      window_bits! (window_end_valid, w, false);
	    }
	}

      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
}


/***********************************************************************
			      Glyph Rows

      See dispextern.h for an overall explanation of glyph rows.
 ***********************************************************************/

/* Clear glyph row ROW.  NOTE: this code relies on the current
   layout of `glyphs' and `used' fields of `struct glyph_row'.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn clear_glyph_row (row: *mut glyph_row)
{
  let off: usize = core::mem::offset_of! (glyph_row, used);

  /* Zero everything except pointers in `glyphs'.  */
  std::ptr::write_bytes ((row as *mut u8).offset (off as isize), 0,
			 core::mem::size_of::<glyph_row> () - off);
}


/* Make ROW an empty, enabled row of canonical character height,
   in window W starting at y-position Y.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn blank_row (w: *mut window, row: *mut glyph_row,
							y: c_int)
{
  let mut min_y: c_int;
  let mut max_y: c_int;

  min_y = WINDOW_TAB_LINE_HEIGHT! (w) + WINDOW_HEADER_LINE_HEIGHT! (w);
  max_y = WINDOW_BOX_HEIGHT_NO_MODE_LINE! (w);

  clear_glyph_row (row);
  (*row).y = y;
  (*row).phys_ascent = 0;
  (*row).ascent = (*row).phys_ascent;
  (*row).phys_height = FRAME_LINE_HEIGHT (XFRAME ((*w).frame));
  (*row).height = (*row).phys_height;
  (*row).visible_height = (*row).height;

  if (*row).y < min_y
    { (*row).visible_height -= min_y - (*row).y; }
  if (*row).y + (*row).height > max_y
    { (*row).visible_height -= (*row).y + (*row).height - max_y; }

  glyph_row_bits! (enabled_p, row, true);
}


/* Increment buffer positions in glyph row ROW.  DELTA and DELTA_BYTES
   are the amounts by which to change positions.  Note that the first
   glyph of the text area of a row can have a buffer position even if
   the used count of the text area is zero.  Such rows display line
   ends.  */

unsafe fn increment_row_positions (row: *mut glyph_row, delta: ptrdiff_t,
				   delta_bytes: ptrdiff_t)
{
  let mut area: c_int;
  let mut i: c_int;

  /* Increment start and end positions.  */
  MATRIX_ROW_START_CHARPOS! (row) += delta;
  MATRIX_ROW_START_BYTEPOS! (row) += delta_bytes;
  MATRIX_ROW_END_CHARPOS! (row) += delta;
  MATRIX_ROW_END_BYTEPOS! (row) += delta_bytes;
  CHARPOS! ((*row).start.pos) += delta;
  BYTEPOS! ((*row).start.pos) += delta_bytes;
  CHARPOS! ((*row).end.pos) += delta;
  BYTEPOS! ((*row).end.pos) += delta_bytes;

  if !glyph_row_bits! (enabled_p, row)
    { return; }

  /* Increment positions in glyphs.  */
  area = 0;
  while area < LAST_AREA
    {
      i = 0;
      while i < (*row).used [area as usize] as c_int
	{
	  if BUFFERP ((*(*row).glyphs [area as usize].add (i as usize)).object)
	      && (*(*row).glyphs [area as usize].add (i as usize)).charpos > 0
	    { (*(*row).glyphs [area as usize].add (i as usize)).charpos += delta; }
	  i += 1;
	}
      area += 1;
    }

  /* Capture the case of rows displaying a line end.  */
  if (*row).used [TEXT_AREA as usize] == 0
      && MATRIX_ROW_DISPLAYS_TEXT_P! (row)
    { (*((*row).glyphs [TEXT_AREA as usize])).charpos += delta; }
}

/* Exchange pointers to glyph memory between glyph rows A and B.  Also
   exchange the used[] array and the hash values of the rows, because
   these should all go together for the row's hash value to be
   correct.  */

unsafe fn swap_glyph_pointers (a: *mut glyph_row, b: *mut glyph_row)
{
  let mut i: c_int;
  let mut hash_tem: c_uint = (*a).hash;

  i = 0;
  while i < LAST_AREA + 1
    {
      let temp: *mut glyph = (*a).glyphs [i as usize];

      (*a).glyphs [i as usize] = (*b).glyphs [i as usize];
      (*b).glyphs [i as usize] = temp;
      if i < LAST_AREA
	{
	  let used_tem: c_short = (*a).used [i as usize];

	  (*a).used [i as usize] = (*b).used [i as usize];
	  (*b).used [i as usize] = used_tem;
	}
      i += 1;
    }
  (*a).hash = (*b).hash;
  (*b).hash = hash_tem;
}


/* Copy glyph row structure FROM to glyph row structure TO, except that
   glyph pointers, the `used' counts, and the hash values in the structures
   are left unchanged.  NOTE: this code relies on the current layout of
   `glyphs', `used', `hash' and `x' fields of `struct glyph_row'.  */

unsafe fn copy_row_except_pointers (to: *mut glyph_row, from: *mut glyph_row)
{
  let off: usize = core::mem::offset_of! (glyph_row, x);

  std::ptr::copy_nonoverlapping ((from as *const u8).offset (off as isize),
				 (to as *mut u8).offset (off as isize),
				 core::mem::size_of::<glyph_row> () - off);
}


/* Assign glyph row FROM to glyph row TO.  This works like a structure
   assignment TO = FROM, except that glyph pointers are not copied but
   exchanged between TO and FROM.  Pointers must be exchanged to avoid
   a memory leak.  */

unsafe fn assign_row (to: *mut glyph_row, from: *mut glyph_row)
{
  swap_glyph_pointers (to, from);
  copy_row_except_pointers (to, from);
}


/* Test whether the glyph memory of the glyph row WINDOW_ROW, which is
   a row in a window matrix, is a slice of the glyph memory of the
   glyph row FRAME_ROW which is a row in a frame glyph matrix.  Value
   is true if the glyph memory of WINDOW_ROW is part of the glyph
   memory of FRAME_ROW.  */

#[cfg (feature = "glyph_debug")]
unsafe fn glyph_row_slice_p (window_row: *mut glyph_row, frame_row: *mut glyph_row) -> bool
{
  let window_glyph_start: *mut glyph = (*window_row).glyphs [0];
  let frame_glyph_start: *mut glyph = (*frame_row).glyphs [0];
  let frame_glyph_end: *mut glyph = (*frame_row).glyphs [LAST_AREA as usize];

  return frame_glyph_start <= window_glyph_start
	 && window_glyph_start < frame_glyph_end;
}

/* C 1081-1101: `#if 0` around `find_glyph_row_slice`.  Dead in C itself; recorded only.  */

/* Prepare ROW for display in windows W.  Desired rows are cleared
   lazily, i.e. they are only marked as to be cleared by setting their
   enabled_p flag to zero.  When a row is to be displayed, a prior
   call to this function really clears it.  In addition, this function
   makes sure the marginal areas of ROW are in sync with the window's
   display margins.  MODE_LINE_P non-zero means we are preparing a
   glyph row for tab/header line or mode line.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn prepare_desired_row
  (w: *mut window, row: *mut glyph_row, mode_line_p: bool)
{
  if !glyph_row_bits! (enabled_p, row)
    {
      let rp: bool = glyph_row_bits! (reversed_p, row);

      clear_glyph_row (row);
      glyph_row_bits! (enabled_p, row, true);
      glyph_row_bits! (reversed_p, row, rp);
    }
  if mode_line_p
    {
      /* Mode and header/tab lines, if displayed, never have marginal
	 areas.  If we are called with MODE_LINE_P non-zero, we are
	 displaying the mode/header/tab line in this window, and so the
	 marginal areas of this glyph row should be eliminated.  This
	 is needed when the mode/header/tab line is switched on in a
	 window that has display margins.  */
      if (*w).left_margin_cols > 0
	{ (*row).glyphs [TEXT_AREA as usize] = (*row).glyphs [LEFT_MARGIN_AREA as usize]; }
      if (*w).right_margin_cols > 0
	{ (*row).glyphs [RIGHT_MARGIN_AREA as usize] = (*row).glyphs [LAST_AREA as usize]; }
    }
  else
    {
      /* The real number of glyphs reserved for the margins is
	 recorded in the glyph matrix, and can be different from
	 window's left_margin_cols and right_margin_cols; see
	 margin_glyphs_to_reserve for when that happens.  */
      let left: c_int = (*( (*w).desired_matrix as *mut glyph_matrix )).left_margin_glyphs;
      let right: c_int = (*( (*w).desired_matrix as *mut glyph_matrix )).right_margin_glyphs;

      /* Make sure the marginal areas of this row are in sync with
	 what the window wants, when the row actually displays text
	 and not tab/header/mode line.  */
      if (*w).left_margin_cols > 0
	  && (left as ptrdiff_t
	      != (*row).glyphs [TEXT_AREA as usize]
		 .offset_from ((*row).glyphs [LEFT_MARGIN_AREA as usize]))
	{ (*row).glyphs [TEXT_AREA as usize]
	    = (*row).glyphs [LEFT_MARGIN_AREA as usize].offset (left as isize); }
      if (*w).right_margin_cols > 0
	  && (right as ptrdiff_t
	      != (*row).glyphs [LAST_AREA as usize]
		 .offset_from ((*row).glyphs [RIGHT_MARGIN_AREA as usize]))
	{
	  (*row).glyphs [RIGHT_MARGIN_AREA as usize]
	    = (*row).glyphs [LAST_AREA as usize].offset (-(right as isize));
	  /* Leave room for a border glyph.  */
	  if !FRAME_WINDOW_P (XFRAME ((*w).frame))
	      && !WINDOW_RIGHTMOST_P! (w)
	      && right > 0
	    { (*row).glyphs [RIGHT_MARGIN_AREA as usize]
		= (*row).glyphs [RIGHT_MARGIN_AREA as usize].offset (-1); }
	}
    }
}

/* C 1165-3827: `#ifndef HAVE_ANDROID` — live on the frozen target (HAVE_ANDROID is
   undefined), so nothing below is gated; the two `#else` arms C has inside this region
   (`#else /* HAVE_ANDROID */`, C 4118) are carried under `feature = "have_android"`.  */

/* Return a hash code for glyph row ROW, which may
   be from current or desired matrix of frame F.  */

unsafe fn line_hash_code (f: *mut frame, row: *mut glyph_row) -> c_uint
{
  let mut hash: c_uint = 0;

  if glyph_row_bits! (enabled_p, row)
    {
      let mut glyph: *mut glyph = (*row).glyphs [TEXT_AREA as usize];
      let end: *mut glyph = glyph.offset ((*row).used [TEXT_AREA as usize] as isize);

      while glyph < end
	{
	  let mut c: c_int = (*glyph).u.ch as c_int;
	  let mut face_id: c_uint = glyph_bits! (face_id, *glyph) as c_uint;
	  /* A given row of a frame glyph matrix could have glyphs
	     from more than one frame, if child frames are displayed.
	     Since face_id of a face depends on the frame (it's an
	     index into the frame's face cache), we need the hash
	     value to include something specific to the frame, and we
	     use the frame cache's address for that purpose.  */
	  if !(*glyph).frame.is_null () && (*glyph).frame != (f as *mut c_void)
	    { face_id = face_id.wrapping_add (
		(*( (*glyph).frame as *mut frame )).face_cache as uintptr_t as c_uint); }
	  if FRAME_MUST_WRITE_SPACES! (f)
	    { c -= SPACEGLYPH; }
	  hash = ((((hash as u64) << 4) as c_uint).wrapping_add (hash >> 24) & 0x0fffffff)
		 .wrapping_add (c as c_uint);
	  hash = ((((hash as u64) << 4) as c_uint).wrapping_add (hash >> 24) & 0x0fffffff)
		 .wrapping_add (face_id);
	  glyph = glyph.offset (1);
	}

      if hash == 0
	{ hash = 1; }
    }

  return hash;
}


/* Return the cost of drawing line VPOS in MATRIX, which may
   be current or desired matrix of frame F.  The cost equals
   the number of characters in the line.  If must_write_spaces
   is zero, leading and trailing spaces are ignored.  */

unsafe fn line_draw_cost (f: *mut frame, matrix: *mut glyph_matrix, vpos: c_int) -> c_int
{
  let row: *mut glyph_row = (*matrix).rows.offset (vpos as isize);
  let mut beg: *mut glyph = (*row).glyphs [TEXT_AREA as usize];
  let mut end: *mut glyph = beg.offset ((*row).used [TEXT_AREA as usize] as isize);
  let mut len: c_int;
  let glyph_table_base: *mut Lisp_Object = GLYPH_TABLE_BASE! ();
  let glyph_table_len: ptrdiff_t = GLYPH_TABLE_LENGTH! ();

  /* Ignore trailing and leading spaces if we can.  */
  if !FRAME_MUST_WRITE_SPACES! (f)
    {
      /* Skip from the end over trailing spaces.  */
      while end > beg && CHAR_GLYPH_SPACE_P! (f, *(end.offset (-1)))
	{ end = end.offset (-1); }

      /* All blank line.  */
      if end == beg
	{ return 0; }

      /* Skip over leading spaces.  */
      while CHAR_GLYPH_SPACE_P! (f, *beg)
	{ beg = beg.offset (1); }
    }

  /* If we don't have a glyph-table, each glyph is one character,
     so return the number of glyphs.  */
  if glyph_table_base.is_null ()
    { len = end.offset_from (beg) as c_int; }
  else
    {
      /* Otherwise, scan the glyphs and accumulate their total length
	 in LEN.  */
      len = 0;
      while beg < end
	{
	  let mut g: GLYPH = GLYPH { ch: 0, face_id: 0 };

	  SET_GLYPH_FROM_CHAR_GLYPH! (g, *beg);

	  if GLYPH_INVALID_P! (g)
	      || GLYPH_SIMPLE_P! (glyph_table_base, glyph_table_len, g)
	    { len += 1; }
	  else
	    { len += GLYPH_LENGTH! (glyph_table_base, g) as c_int; }

	  beg = beg.offset (1);
	}
    }

  return len;
}

/* Return true if the glyph rows A and B have equal contents.
   MOUSE_FACE_P means compare the mouse_face_p flags of A and B, too.  */

unsafe fn row_equal_p (a: *mut glyph_row, b: *mut glyph_row, mouse_face_p: bool) -> bool
{
  eassert! (verify_row_hash (a));
  eassert! (verify_row_hash (b));

  if a == b
    { return true; }
  else if (*a).hash != (*b).hash
    { return false; }
  else
    {
      let mut a_glyph: *mut glyph;
      let mut b_glyph: *mut glyph;
      let mut a_end: *mut glyph;
      let mut area: c_int;

      if mouse_face_p && glyph_row_bits! (mouse_face_p, a) != glyph_row_bits! (mouse_face_p, b)
	{ return false; }

      /* Compare glyphs.  */
      area = LEFT_MARGIN_AREA;
      while area < LAST_AREA
	{
	  if (*a).used [area as usize] != (*b).used [area as usize]
	    { return false; }

	  a_glyph = (*a).glyphs [area as usize];
	  a_end = a_glyph.offset ((*a).used [area as usize] as isize);
	  b_glyph = (*b).glyphs [area as usize];

	  while a_glyph < a_end
		&& GLYPH_EQUAL_P! (a_glyph, b_glyph)
	    { a_glyph = a_glyph.offset (1); b_glyph = b_glyph.offset (1); }

	  if a_glyph != a_end
	    { return false; }
	  area += 1;
	}

      if glyph_row_bits! (fill_line_p, a) != glyph_row_bits! (fill_line_p, b)
	  || glyph_row_bits! (cursor_in_fringe_p, a) != glyph_row_bits! (cursor_in_fringe_p, b)
	  || glyph_row_bits! (left_fringe_bitmap, a) != glyph_row_bits! (left_fringe_bitmap, b)
	  || glyph_row_bits! (left_fringe_face_id, a) != glyph_row_bits! (left_fringe_face_id, b)
	  || glyph_row_bits! (left_fringe_offset, a) != glyph_row_bits! (left_fringe_offset, b)
	  || glyph_row_bits! (right_fringe_bitmap, a) != glyph_row_bits! (right_fringe_bitmap, b)
	  || glyph_row_bits! (right_fringe_face_id, a)
	     != glyph_row_bits! (right_fringe_face_id, b)
	  || (*a).right_fringe_offset != (*b).right_fringe_offset
	  || glyph_row_bits! (fringe_bitmap_periodic_p, a)
	     != glyph_row_bits! (fringe_bitmap_periodic_p, b)
	  || (*a).overlay_arrow_bitmap != (*b).overlay_arrow_bitmap
	  || glyph_row_bits! (exact_window_width_line_p, a)
	     != glyph_row_bits! (exact_window_width_line_p, b)
	  || glyph_row_bits! (overlapped_p, a) != glyph_row_bits! (overlapped_p, b)
	  || (MATRIX_ROW_CONTINUATION_LINE_P! (a)
	      != MATRIX_ROW_CONTINUATION_LINE_P! (b))
	  || glyph_row_bits! (reversed_p, a) != glyph_row_bits! (reversed_p, b)
	  /* Different partially visible characters on left margin.  */
	  || (*a).x != (*b).x
	  /* Different height.  */
	  || (*a).ascent != (*b).ascent
	  || (*a).phys_ascent != (*b).phys_ascent
	  || (*a).phys_height != (*b).phys_height
	  || (*a).visible_height != (*b).visible_height
	{ return false; }
    }

  return true;
}


/***********************************************************************
			      Glyph Pool

     See dispextern.h for an overall explanation of glyph pools.
 ***********************************************************************/

/* Allocate a glyph_pool structure.  The structure returned is initialized
   with zeros.  If GLYPH_DEBUG and ENABLE_CHECKING are in effect, the global
   variable glyph_pool_count is incremented for each pool allocated.  */

unsafe fn new_glyph_pool () -> *mut glyph_pool
{
  let mut result: *mut glyph_pool =
    xzalloc (core::mem::size_of::<glyph_pool> ()) as *mut glyph_pool;

  #[cfg (all (feature = "glyph_debug", enable_checking))]
  {
    /* For memory leak and double deletion checking.  */
    glyph_pool_count += 1;
  }

  return result;
}


/* Free a glyph_pool structure POOL.  The function may be called with
   a null POOL pointer.  If GLYPH_DEBUG and ENABLE_CHECKING are in effect,
   global variable glyph_pool_count is decremented with every pool structure
   freed.  If this count gets negative, more structures were freed than
   allocated, i.e. one structure must have been freed more than once or
   a bogus pointer was passed to free_glyph_pool.  */

unsafe fn free_glyph_pool (pool: *mut glyph_pool)
{
  if !pool.is_null ()
    {
      #[cfg (all (feature = "glyph_debug", enable_checking))]
      {
	/* More freed than allocated?  */
	glyph_pool_count -= 1;
	eassert! (glyph_pool_count >= 0);
      }
      xfree ((*pool).glyphs as *mut c_void);
      xfree (pool as *mut c_void);
    }
}


/* Enlarge a glyph pool POOL.  MATRIX_DIM gives the number of rows and
   columns we need.  This function never shrinks a pool.  The only
   case in which this would make sense, would be when a frame's size
   is changed from a large value to a smaller one.  But, if someone
   does it once, we can expect that he will do it again.

   Return true if the pool changed in a way which makes
   re-adjusting window glyph matrices necessary.  */

unsafe fn realloc_glyph_pool (pool: *mut glyph_pool, matrix_dim: dim) -> bool
{
  let mut needed: ptrdiff_t;
  let mut changed_p: bool;

  changed_p = ((*pool).glyphs.is_null ()
	       || matrix_dim.height != (*pool).nrows
	       || matrix_dim.width != (*pool).ncolumns);

  /* Enlarge the glyph pool.  */
  let (needed_, mul_overflow) = (matrix_dim.height as ptrdiff_t)
    .overflowing_mul (matrix_dim.width as ptrdiff_t);
  needed = needed_;
  if mul_overflow
    { memory_full_up (); }
  if needed > (*pool).nglyphs
    {
      let old_nglyphs: ptrdiff_t = (*pool).nglyphs;
      (*pool).glyphs = xpalloc ((*pool).glyphs as *mut c_void,
				core::ptr::addr_of_mut! ((*pool).nglyphs),
				needed - old_nglyphs, -1, core::mem::size_of::<glyph> () as ptrdiff_t)
	as *mut glyph;
      memclear ((*pool).glyphs.offset (old_nglyphs as isize) as *mut c_void,
		((*pool).nglyphs - old_nglyphs) * core::mem::size_of::<glyph> () as ptrdiff_t);
    }

  /* Remember the number of rows and columns because (a) we use them
     to do sanity checks, and (b) the number of columns determines
     where rows in the frame matrix start---this must be available to
     determine pointers to rows of window sub-matrices.  */
  (*pool).nrows = matrix_dim.height;
  (*pool).ncolumns = matrix_dim.width;

  return changed_p;
}


/***********************************************************************
			      Debug Code
 ***********************************************************************/

/* C 1396-1556: `#ifdef GLYPH_DEBUG` — `flush_stdout`, `check_matrix_pointer_lossage` and
   `matrix_row`.  Dead on the frozen target; carried as text (P-51).  */

#[cfg (feature = "glyph_debug")]
/* Flush standard output.  This is sometimes useful to call from the debugger.
   XXX Maybe this should be changed to flush the current terminal instead of
   stdout.
*/

#[unsafe(no_mangle)] pub unsafe extern "C" fn flush_stdout ()
{
  fflush (stdout);
}


/* Check that no glyph pointers have been lost in MATRIX.  If a
   pointer has been lost, e.g. by using a structure assignment between
   rows, at least one pointer must occur more than once in the rows of
   MATRIX.  */

/* `dispextern.rs` declares `check_matrix_pointer_lossage` at its crate root (dispextern.h
   864, gated by the same feature), so the definition lives in a glob-imported module: a
   second crate-root item of the same name would be `E0428` (the `space_glyph_def` device).  */
#[cfg (feature = "glyph_debug")]
pub mod check_matrix_pointer_lossage_def {
  use super::*;

  #[unsafe(no_mangle)] pub unsafe extern "C" fn check_matrix_pointer_lossage
    (matrix: *mut glyph_matrix)
  {
    let mut i: c_int;
    let mut j: c_int;

    i = 0;
    while i < (*matrix).nrows
      {
	j = 0;
	while j < (*matrix).nrows
	  {
	    eassert! (i == j
		      || ((*(*matrix).rows.offset (i as isize)).glyphs [TEXT_AREA as usize]
			  != (*(*matrix).rows.offset (j as isize)).glyphs [TEXT_AREA as usize]));
	    j += 1;
	  }
	i += 1;
      }
  }
}
#[cfg (feature = "glyph_debug")]
use check_matrix_pointer_lossage_def::*;


/* Get a pointer to glyph row ROW in MATRIX, with bounds checks.  */

#[cfg (feature = "glyph_debug")]
#[unsafe(no_mangle)] pub unsafe extern "C" fn matrix_row (matrix: *mut glyph_matrix,
							  row: c_int) -> *mut glyph_row
{
  eassert! (!matrix.is_null () && !(*matrix).rows.is_null ());
  eassert! (row >= 0 && row < (*matrix).nrows);

  /* That's really too slow for normal testing because this function
     is called almost everywhere.  Although---it's still astonishingly
     fast, so it is valuable to have for debugging purposes.  */
  /* C 1461-1463: `#if 0 check_matrix_pointer_lossage (matrix); #endif`.  Dead in C
     itself; recorded only.  */

  return (*matrix).rows.offset (row as isize);
}

/* C 1465-1556: `#if 0 /* This function makes invalid assumptions … */` around
   `check_matrix_invariants`.  Dead in C itself; recorded only.  */


/**********************************************************************
		 Allocating/ Adjusting Glyph Matrices
 **********************************************************************/

/* Allocate glyph matrices over a window tree for a frame-based
   redisplay

   X and Y are column/row within the frame glyph matrix where
   sub-matrices for the window tree rooted at WINDOW must be
   allocated.  DIM_ONLY_P means that the caller of this
   function is only interested in the result matrix dimension, and
   matrix adjustments should not be performed.

   The function returns the total width/height of the sub-matrices of
   the window tree.  If called on a frame root window, the computation
   will take the mini-buffer window into account.

   *WINDOW_CHANGE_FLAGS is set to a bit mask with bits

   NEW_LEAF_MATRIX set if any window in the tree did not have a
   glyph matrices yet, and

   CHANGED_LEAF_MATRIX set if the dimension or location of a matrix of
   any window in the tree will be changed or have been changed (see
   DIM_ONLY_P)

   *WINDOW_CHANGE_FLAGS must be initialized by the caller of this
   function.

   Windows are arranged into chains of windows on the same level
   through the next fields of window structures.  Such a level can be
   either a sequence of horizontally adjacent windows from left to
   right, or a sequence of vertically adjacent windows from top to
   bottom.  Each window in a horizontal sequence can be either a leaf
   window or a vertical sequence; a window in a vertical sequence can
   be either a leaf or a horizontal sequence.  All windows in a
   horizontal sequence have the same height, and all windows in a
   vertical sequence have the same width.

   This function uses, for historical reasons, a more general
   algorithm to determine glyph matrix dimensions that would be
   necessary.

   The matrix height of a horizontal sequence is determined by the
   maximum height of any matrix in the sequence.  The matrix width of
   a horizontal sequence is computed by adding up matrix widths of
   windows in the sequence.

   |<------- result width ------->|
   +---------+----------+---------+ ---
   |         |		|	  |  |
   |         |		|	  |
   +---------+		|	  |  result height
	     |		+---------+
	     |		|            |
	     +----------+	    ---

   The matrix width of a vertical sequence is the maximum matrix width
   of any window in the sequence.  Its height is computed by adding up
   matrix heights of windows in the sequence.

   |<---- result width -->|
   +---------+		    ---
   |         |               |
   |         |               |
   +---------+--+            |
   |		|            |
   |		|	     result height
   |		|
   +------------+---------+  |
   |			  |  |
   |			  |  |
   +------------+---------+ ---  */

/* Bit indicating that a new matrix will be allocated or has been
   allocated.  */

macro_rules! NEW_LEAF_MATRIX { () => { (1 << 0) }; }

/* Bit indicating that a matrix will or has changed its location or
   size.  */

macro_rules! CHANGED_LEAF_MATRIX { () => { (1 << 1) }; }

unsafe fn allocate_matrices_for_frame_redisplay (window: Lisp_Object, x: c_int, y: c_int,
						  dim_only_p: bool,
						  window_change_flags: *mut c_int) -> dim
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (XWINDOW (window)));
  let x0: c_int = x; let y0: c_int = y;
  let mut x: c_int = x; let mut y: c_int = y;
  let mut wmax: c_int = 0; let mut hmax: c_int = 0;
  let mut total: dim = dim { width: 0, height: 0 };
  let mut dim: dim = dim { width: 0, height: 0 };
  let mut w: *mut window;
  let in_horz_combination_p: bool;
  let mut window: Lisp_Object = window;

  /* What combination is WINDOW part of?  Compute this once since the
     result is the same for all windows in the `next' chain.  The
     special case of a root window (parent equal to nil) is treated
     like a vertical combination because a root window's `next'
     points to the mini-buffer window, if any, which is arranged
     vertically below other windows.  */
  in_horz_combination_p
    = (!NILP ((*XWINDOW (window)).parent)
       && WINDOW_HORIZONTAL_COMBINATION_P (XWINDOW ((*XWINDOW (window)).parent)));

  /* For WINDOW and all windows on the same level.  */
  loop
    {
      w = XWINDOW (window);

      /* Get the dimension of the window sub-matrix for W, depending
	 on whether this is a combination or a leaf window.  */
      if WINDOWP ((*w).contents)
	{ dim = allocate_matrices_for_frame_redisplay ((*w).contents, x, y,
						       dim_only_p,
						       window_change_flags); }
      else
	{
	  /* If not already done, allocate sub-matrix structures.  */
	  if (*w).desired_matrix.is_null ()
	    {
	      (*w).desired_matrix = new_glyph_matrix ((*f).desired_pool as *mut glyph_pool) as *mut c_void;
	      (*w).current_matrix = new_glyph_matrix ((*f).current_pool as *mut glyph_pool) as *mut c_void;
	      *window_change_flags |= NEW_LEAF_MATRIX! ();
	    }

	  /* Width and height MUST be chosen so that there are no
	     holes in the frame matrix.  */
	  dim.width = required_matrix_width (w);
	  dim.height = required_matrix_height (w);

	  /* Will matrix be re-allocated?  */
	  if x != (*( (*w).desired_matrix as *mut glyph_matrix )).matrix_x
	      || y != (*( (*w).desired_matrix as *mut glyph_matrix )).matrix_y
	      || dim.width != (*( (*w).desired_matrix as *mut glyph_matrix )).matrix_w
	      || dim.height != (*( (*w).desired_matrix as *mut glyph_matrix )).matrix_h
	      || (margin_glyphs_to_reserve (w, dim.width, (*w).left_margin_cols)
		  != (*( (*w).desired_matrix as *mut glyph_matrix )).left_margin_glyphs)
	      || (margin_glyphs_to_reserve (w, dim.width, (*w).right_margin_cols)
		  != (*( (*w).desired_matrix as *mut glyph_matrix )).right_margin_glyphs)
	    { *window_change_flags |= CHANGED_LEAF_MATRIX! (); }

	  /* Actually change matrices, if allowed.  Do not consider
	     CHANGED_LEAF_MATRIX computed above here because the pool
	     may have been changed which we don't know here.  We trust
	     that we only will be called with DIM_ONLY_P when
	     necessary.  */
	  if !dim_only_p
	    {
	      adjust_glyph_matrix (w, (*w).desired_matrix as *mut glyph_matrix, x, y, dim);
	      adjust_glyph_matrix (w, (*w).current_matrix as *mut glyph_matrix, x, y, dim);
	    }
	}

      /* If we are part of a horizontal combination, advance x for
	 windows to the right of W; otherwise advance y for windows
	 below W.  */
      if in_horz_combination_p
	{ x += dim.width; }
      else
	{ y += dim.height; }

      /* Remember maximum glyph matrix dimensions.  */
      wmax = std::cmp::max (wmax, dim.width);
      hmax = std::cmp::max (hmax, dim.height);

      /* Next window on same level.  */
      window = (*w).next;
      if NILP (window)
	{ break; }
    }

  /* Set `total' to the total glyph matrix dimension of this window
     level.  In a vertical combination, the width is the width of the
     widest window; the height is the y we finally reached, corrected
     by the y we started with.  In a horizontal combination, the total
     height is the height of the tallest window, and the width is the
     x we finally reached, corrected by the x we started with.  */
  if in_horz_combination_p
    {
      total.width = x - x0;
      total.height = hmax;
    }
  else
    {
      total.width = wmax;
      total.height = y - y0;
    }

  return total;
}


/* Return the required height of glyph matrices for window W.  */

unsafe fn required_matrix_height (w: *mut window) -> c_int
{
  #[cfg (feature = "have_window_system")]
  {
  let f: *mut frame = XFRAME ((*w).frame);

  if FRAME_WINDOW_P (f)
    {
      /* https://lists.gnu.org/r/emacs-devel/2015-11/msg00194.html  */
      let ch_height: c_int = std::cmp::max (FRAME_SMALLEST_FONT_HEIGHT! (f), 1);
      let window_pixel_height: c_int = window_box_height (w) + eabs! ((*w).vscroll);

      return (((window_pixel_height + ch_height - 1)
	       / ch_height) * (*w).nrows_scale_factor
	      /* One partially visible line at the top and
		 bottom of the window.  */
	      + 2
	      /* 3 for tab, header and mode line.  */
	      + 3);
    }
  }

  return WINDOW_TOTAL_LINES (w);
}


/* Return the required width of glyph matrices for window W.  */

unsafe fn required_matrix_width (w: *mut window) -> c_int
{
  #[cfg (feature = "have_window_system")]
  {
  let f: *mut frame = XFRAME ((*w).frame);
  if FRAME_WINDOW_P (f)
    {
      /* https://lists.gnu.org/r/emacs-devel/2015-11/msg00194.html  */
      let ch_width: c_int = std::cmp::max (FRAME_SMALLEST_CHAR_WIDTH! (f), 1);

      /* Compute number of glyphs needed in a glyph row.  */
      return (((WINDOW_PIXEL_WIDTH (w) + ch_width - 1)
	       / ch_width) * (*w).ncols_scale_factor
	      /* 2 partially visible columns in the text area.  */
	      + 2
	      /* One partially visible column at the right
		 edge of each marginal area.  */
	      + 1 + 1);
    }
  }

  return (*w).total_cols;
}


/* Allocate window matrices for window-based redisplay.  W is the
   window whose matrices must be allocated/reallocated.  */

unsafe fn allocate_matrices_for_window_redisplay (w: *mut window)
{
  let mut w: *mut window = w;

  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{ allocate_matrices_for_window_redisplay (XWINDOW ((*w).contents)); }
      else
	{
	  /* W is a leaf window.  */
	  let mut dim: dim = dim { width: 0, height: 0 };

	  /* If matrices are not yet allocated, allocate them now.  */
	  if (*w).desired_matrix.is_null ()
	    {
	      (*w).desired_matrix = new_glyph_matrix (core::ptr::null_mut ()) as *mut c_void;
	      eassert! ((*w).current_matrix.is_null ());
	    }

	  if (*w).current_matrix.is_null ()
	    { (*w).current_matrix = new_glyph_matrix (core::ptr::null_mut ()) as *mut c_void; }

	  dim.width = required_matrix_width (w);
	  dim.height = required_matrix_height (w);
	  adjust_glyph_matrix (w, (*w).desired_matrix as *mut glyph_matrix, 0, 0, dim);
	  adjust_glyph_matrix (w, (*w).current_matrix as *mut glyph_matrix, 0, 0, dim);
	}

      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
}

/* Allocate/reallocate glyph matrices of a single frame F.
   This function must be called when a new frame is created,
   its size changes, or its window configuration changes.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn adjust_frame_glyphs (f: *mut frame)
{
  /* Block input so that expose events and other events that access
     glyph matrices are not processed while we are changing them.  */
  block_input ();

  if FRAME_WINDOW_P (f)
    { adjust_frame_glyphs_for_window_redisplay (f); }
  else
    {
      adjust_frame_glyphs_for_frame_redisplay (f);
      eassert! (FRAME_INITIAL_P (f)
	       || noninteractive
	       || !initialized
	       || (*(*f).terminal).name.is_null () /* frame is being deleted */
	       || (!(*f).current_matrix.is_null ()
		   && (*( (*f).current_matrix as *mut glyph_matrix )).nrows > 0
		   && !(*( (*f).current_matrix as *mut glyph_matrix )).rows.is_null ()
		   && !(*f).desired_matrix.is_null ()
		   && (*( (*f).desired_matrix as *mut glyph_matrix )).nrows > 0
		   && !(*( (*f).desired_matrix as *mut glyph_matrix )).rows.is_null ()));
    }

  /* Don't forget the buffer for decode_mode_spec.  */
  adjust_decode_mode_spec_buffer (f);

  glyphs_initialized_p! (f, true);

  unblock_input ();
}

/* Return true if any window in the tree has nonzero window margins.  See
   the hack at the end of adjust_frame_glyphs_for_frame_redisplay.  */
unsafe fn showing_window_margins_p (w: *mut window) -> bool
{
  let mut w: *mut window = w;

  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{
	  if showing_window_margins_p (XWINDOW ((*w).contents))
	    { return true; }
	}
      else if (*w).left_margin_cols > 0 || (*w).right_margin_cols > 0
	{ return true; }

      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
  return false;
}


/* In the window tree with root W, build current matrices of leaf
   windows from the frame's current matrix.  */

unsafe fn fake_current_matrices (window: Lisp_Object)
{
  let mut window: Lisp_Object = window;
  let mut w: *mut window;

  while !NILP (window)
    {
      w = XWINDOW (window);

      if WINDOWP ((*w).contents)
	{ fake_current_matrices ((*w).contents); }
      else
	{
	  let mut i: c_int;
	  let f: *mut frame = XFRAME ((*w).frame);
	  let m: *mut glyph_matrix = (*w).current_matrix as *mut glyph_matrix;
	  let fm: *mut glyph_matrix = (*f).current_matrix as *mut glyph_matrix;

	  eassert! ((*m).matrix_h == WINDOW_TOTAL_LINES (w));
	  eassert! ((*m).matrix_w == WINDOW_TOTAL_COLS (w));

	  i = 0;
	  while i < (*m).matrix_h
	    {
	      let r: *mut glyph_row = (*m).rows.offset (i as isize);
	      let fr: *mut glyph_row = (*fm).rows
		.offset ((i + WINDOW_TOP_EDGE_LINE (w)) as isize);

	      eassert! ((*r).glyphs [TEXT_AREA as usize] >= (*fr).glyphs [TEXT_AREA as usize]
			&& (*r).glyphs [LAST_AREA as usize] <= (*fr).glyphs [LAST_AREA as usize]);

	      glyph_row_bits! (enabled_p, r, glyph_row_bits! (enabled_p, fr));
	      if glyph_row_bits! (enabled_p, r)
		{
		  (*r).used [LEFT_MARGIN_AREA as usize] = (*m).left_margin_glyphs as c_short;
		  (*r).used [RIGHT_MARGIN_AREA as usize] = (*m).right_margin_glyphs as c_short;
		  /* Non-rightmost windows have the border glyph at the
		     end of the right margin, if any, in addition to the
		     glyphs reserved for the margin itself.  */
		  if (*m).right_margin_glyphs > 0 && !WINDOW_RIGHTMOST_P! (w)
		    { (*r).used [RIGHT_MARGIN_AREA as usize] += 1; }
		  (*r).used [TEXT_AREA as usize] = ((*m).matrix_w
						    - (*r).used [LEFT_MARGIN_AREA as usize] as c_int
						    - (*r).used [RIGHT_MARGIN_AREA as usize] as c_int)
		    as c_short;
		  glyph_row_bits! (mode_line_p, r, false);
		  glyph_row_bits! (tab_line_p, r, false);
		}
	      i += 1;
	    }
	}
      window = (*w).next;
    }
}


/* Save away the contents of frame F's current frame matrix.  Value is
   a glyph matrix holding the contents of F's current frame matrix.  */

unsafe fn save_current_matrix (f: *mut frame) -> *mut glyph_matrix
{
  let mut i: c_int;
  let mut saved: *mut glyph_matrix =
    xzalloc (core::mem::size_of::<glyph_matrix> ()) as *mut glyph_matrix;
  (*saved).nrows = (*( (*f).current_matrix as *mut glyph_matrix )).nrows;
  (*saved).rows = xcalloc ((*saved).nrows as usize,
			   core::mem::size_of::<glyph_row> ()) as *mut glyph_row;

  i = 0;
  while i < (*saved).nrows
    {
      let from: *mut glyph_row =
	(*( (*f).current_matrix as *mut glyph_matrix )).rows.offset (i as isize);
      let to: *mut glyph_row = (*saved).rows.offset (i as isize);
      let mut nbytes: ptrdiff_t = (*from).used [TEXT_AREA as usize] as ptrdiff_t
	* core::mem::size_of::<glyph> () as ptrdiff_t;

      (*to).glyphs [TEXT_AREA as usize] = xmalloc (nbytes as usize) as *mut glyph;
      /* C's `memcpy (to->glyphs[TEXT_AREA], from->glyphs[TEXT_AREA], nbytes)` counts bytes;
	 `copy_nonoverlapping` counts `T`s (PORTING.md P-93), so both pointers go through `u8`.  */
      std::ptr::copy_nonoverlapping ((*from).glyphs [TEXT_AREA as usize] as *const u8,
				     (*to).glyphs [TEXT_AREA as usize] as *mut u8, nbytes as usize);
      (*to).used [TEXT_AREA as usize] = (*from).used [TEXT_AREA as usize];
      glyph_row_bits! (enabled_p, to, glyph_row_bits! (enabled_p, from));
      (*to).hash = (*from).hash;
      if (*from).used [LEFT_MARGIN_AREA as usize] != 0
	{
	  nbytes = (*from).used [LEFT_MARGIN_AREA as usize] as ptrdiff_t
	    * core::mem::size_of::<glyph> () as ptrdiff_t;
	  (*to).glyphs [LEFT_MARGIN_AREA as usize] = xmalloc (nbytes as usize) as *mut glyph;
	  std::ptr::copy_nonoverlapping ((*from).glyphs [LEFT_MARGIN_AREA as usize] as *const u8,
					 (*to).glyphs [LEFT_MARGIN_AREA as usize] as *mut u8,
					 nbytes as usize);
	  (*to).used [LEFT_MARGIN_AREA as usize] = (*from).used [LEFT_MARGIN_AREA as usize];
	}
      if (*from).used [RIGHT_MARGIN_AREA as usize] != 0
	{
	  nbytes = (*from).used [RIGHT_MARGIN_AREA as usize] as ptrdiff_t
	    * core::mem::size_of::<glyph> () as ptrdiff_t;
	  (*to).glyphs [RIGHT_MARGIN_AREA as usize] = xmalloc (nbytes as usize) as *mut glyph;
	  std::ptr::copy_nonoverlapping ((*from).glyphs [RIGHT_MARGIN_AREA as usize] as *const u8,
					 (*to).glyphs [RIGHT_MARGIN_AREA as usize] as *mut u8,
					 nbytes as usize);
	  (*to).used [RIGHT_MARGIN_AREA as usize] = (*from).used [RIGHT_MARGIN_AREA as usize];
	}
      i += 1;
    }

  return saved;
}


/* Restore the contents of frame F's current frame matrix from SAVED,
   and free memory associated with SAVED.  */

unsafe fn restore_current_matrix (f: *mut frame, saved: *mut glyph_matrix)
{
  let mut i: c_int;

  i = 0;
  while i < (*saved).nrows
    {
      let from: *mut glyph_row = (*saved).rows.offset (i as isize);
      let to: *mut glyph_row =
	(*( (*f).current_matrix as *mut glyph_matrix )).rows.offset (i as isize);
      let mut nbytes: ptrdiff_t = (*from).used [TEXT_AREA as usize] as ptrdiff_t
	* core::mem::size_of::<glyph> () as ptrdiff_t;

      std::ptr::copy_nonoverlapping ((*from).glyphs [TEXT_AREA as usize] as *const u8,
				     (*to).glyphs [TEXT_AREA as usize] as *mut u8, nbytes as usize);
      (*to).used [TEXT_AREA as usize] = (*from).used [TEXT_AREA as usize];
      xfree ((*from).glyphs [TEXT_AREA as usize] as *mut c_void);
      nbytes = (*from).used [LEFT_MARGIN_AREA as usize] as ptrdiff_t
	* core::mem::size_of::<glyph> () as ptrdiff_t;
      if nbytes != 0
	{
	  std::ptr::copy_nonoverlapping ((*from).glyphs [LEFT_MARGIN_AREA as usize] as *const u8,
					 (*to).glyphs [LEFT_MARGIN_AREA as usize] as *mut u8,
					 nbytes as usize);
	  (*to).used [LEFT_MARGIN_AREA as usize] = (*from).used [LEFT_MARGIN_AREA as usize];
	  xfree ((*from).glyphs [LEFT_MARGIN_AREA as usize] as *mut c_void);
	}
      else
	{ (*to).used [LEFT_MARGIN_AREA as usize] = 0; }
      nbytes = (*from).used [RIGHT_MARGIN_AREA as usize] as ptrdiff_t
	* core::mem::size_of::<glyph> () as ptrdiff_t;
      if nbytes != 0
	{
	  std::ptr::copy_nonoverlapping ((*from).glyphs [RIGHT_MARGIN_AREA as usize] as *const u8,
					 (*to).glyphs [RIGHT_MARGIN_AREA as usize] as *mut u8,
					 nbytes as usize);
	  (*to).used [RIGHT_MARGIN_AREA as usize] = (*from).used [RIGHT_MARGIN_AREA as usize];
	  xfree ((*from).glyphs [RIGHT_MARGIN_AREA as usize] as *mut c_void);
	}
      else
	{ (*to).used [RIGHT_MARGIN_AREA as usize] = 0; }
      i += 1;
    }

  xfree ((*saved).rows as *mut c_void);
  xfree (saved as *mut c_void);
}


/* Allocate/reallocate glyph matrices of a single frame F for
   frame-based redisplay.  */

unsafe fn adjust_frame_glyphs_for_frame_redisplay (f: *mut frame)
{
  let mut matrix_dim: dim = dim { width: 0, height: 0 };
  let pool_changed_p: bool;
  let mut window_change_flags: c_int;
  let top_window_y: c_int;

  if !FRAME_LIVE_P (f)
    { return; }

  top_window_y = FRAME_TOP_MARGIN (f);

  /* Allocate glyph pool structures if not already done.  */
  if (*f).desired_pool.is_null ()
    {
      (*f).desired_pool = new_glyph_pool () as *mut c_void;
      (*f).current_pool = new_glyph_pool () as *mut c_void;
    }

  /* Allocate frames matrix structures if needed.  */
  if (*f).desired_matrix.is_null ()
    {
      (*f).desired_matrix = new_glyph_matrix ((*f).desired_pool as *mut glyph_pool) as *mut c_void;
      (*f).current_matrix = new_glyph_matrix ((*f).current_pool as *mut glyph_pool) as *mut c_void;
    }

  /* Compute window glyph matrices.  (This takes the mini-buffer
     window into account).  The result is the size of the frame glyph
     matrix needed.  The variable window_change_flags is set to a bit
     mask indicating whether new matrices will be allocated or
     existing matrices change their size or location within the frame
     matrix.  */
  window_change_flags = 0;
  matrix_dim
    = allocate_matrices_for_frame_redisplay (FRAME_ROOT_WINDOW (f),
					     0, top_window_y,
					     true,
					     &mut window_change_flags);

  /* Add in menu bar lines, if any.  */
  matrix_dim.height += top_window_y;

  /* Enlarge pools as necessary.  */
  pool_changed_p = realloc_glyph_pool ((*f).desired_pool as *mut glyph_pool, matrix_dim);
  realloc_glyph_pool ((*f).current_pool as *mut glyph_pool, matrix_dim);

  /* Set up glyph pointers within window matrices.  Do this only if
     absolutely necessary since it requires a frame redraw.  */
  if pool_changed_p || window_change_flags != 0
    {
      /* Do it for window matrices.  */
      allocate_matrices_for_frame_redisplay (FRAME_ROOT_WINDOW (f),
					     0, top_window_y, false,
					     &mut window_change_flags);

      /* Size of frame matrices must equal size of frame.  Note
	 that we are called for X frames with window widths NOT equal
	 to the frame width (from CHANGE_FRAME_SIZE_1).  */
      if matrix_dim.width != FRAME_TOTAL_COLS (f)
	  || matrix_dim.height != FRAME_TOTAL_LINES (f)
	{
	  /* We have reallocated the frame's glyph pools, but didn't
	     update the glyph pointers in the frame's glyph matrices
	     to use the reallocated pools (that happens below, in the
	     call to adjust_glyph_matrix).  Set the frame's garbaged
	     flag, so that when we are called again from
	     redisplay_internal, we don't erroneously call
	     save_current_matrix, because it will use the wrong glyph
	     pointers, and will most probably crash.  */
	  if !FRAME_WINDOW_P (f) && pool_changed_p
	    { SET_FRAME_GARBAGED! (f); }
	  return;
	}

      eassert! (matrix_dim.width == FRAME_TOTAL_COLS (f)
		&& matrix_dim.height == FRAME_TOTAL_LINES (f));

      /* Pointers to glyph memory in glyph rows are exchanged during
	 the update phase of redisplay, which means in general that a
	 frame's current matrix consists of pointers into both the
	 desired and current glyph pool of the frame.  Adjusting a
	 matrix sets the frame matrix up so that pointers are all into
	 the same pool.  If we want to preserve glyph contents of the
	 current matrix over a call to adjust_glyph_matrix, we must
	 make a copy of the current glyphs, and restore the current
	 matrix' contents from that copy.  */
      if !FRAME_GARBAGED_P (f)
	  && matrix_dim.width == (*( (*f).current_matrix as *mut glyph_matrix )).matrix_w
	  && matrix_dim.height == (*( (*f).current_matrix as *mut glyph_matrix )).matrix_h
	  /* For some reason, the frame glyph matrix gets corrupted if
	     any of the windows contain margins.  I haven't been able
	     to hunt down the reason, but for the moment this prevents
	     the problem from manifesting. -- cyd  */
	  && !showing_window_margins_p (XWINDOW (FRAME_ROOT_WINDOW (f)))
	{
	  let copy: *mut glyph_matrix = save_current_matrix (f);
	  adjust_glyph_matrix (core::ptr::null_mut (), (*f).desired_matrix as *mut glyph_matrix,
			       0, 0, matrix_dim);
	  adjust_glyph_matrix (core::ptr::null_mut (), (*f).current_matrix as *mut glyph_matrix,
			       0, 0, matrix_dim);
	  restore_current_matrix (f, copy);
	  fake_current_matrices (FRAME_ROOT_WINDOW (f));
	}
      else
	{
	  adjust_glyph_matrix (core::ptr::null_mut (), (*f).desired_matrix as *mut glyph_matrix,
			       0, 0, matrix_dim);
	  adjust_glyph_matrix (core::ptr::null_mut (), (*f).current_matrix as *mut glyph_matrix,
			       0, 0, matrix_dim);
	  SET_FRAME_GARBAGED! (f);
	}
    }
  else if !FRAME_INITIAL_P (f) && !noninteractive && initialized
    {
      if (*( (*f).desired_matrix as *mut glyph_matrix )).nrows == 0
	  || (*( (*f).desired_matrix as *mut glyph_matrix )).rows.is_null ()
	{
	  adjust_glyph_matrix (core::ptr::null_mut (), (*f).desired_matrix as *mut glyph_matrix,
			       0, 0, matrix_dim);
	  SET_FRAME_GARBAGED! (f);
	}
      if (*( (*f).current_matrix as *mut glyph_matrix )).nrows == 0
	  || (*( (*f).current_matrix as *mut glyph_matrix )).rows.is_null ()
	{
	  adjust_glyph_matrix (core::ptr::null_mut (), (*f).current_matrix as *mut glyph_matrix,
			       0, 0, matrix_dim);
	  SET_FRAME_GARBAGED! (f);
	}
    }
}


/* Allocate/reallocate glyph matrices of a single frame F for
   window-based redisplay.  */

unsafe fn adjust_frame_glyphs_for_window_redisplay (f: *mut frame)
{
  eassert! (FRAME_WINDOW_P (f) && FRAME_LIVE_P (f));

  /* Allocate/reallocate window matrices.  */
  allocate_matrices_for_window_redisplay (XWINDOW (FRAME_ROOT_WINDOW (f)));

  #[cfg (feature = "window_system_no_ext_menu_bar")]
  {
  /* Allocate/ reallocate matrices of the dummy window used to display
     the menu bar under X when no X toolkit support is available.  */
  {
    /* Allocate a dummy window if not already done.  */
    let mut w: *mut window;
    if NILP ((*f).menu_bar_window)
      {
	let mut frame: Lisp_Object;
	fset_menu_bar_window (f, make_window ());
	w = XWINDOW ((*f).menu_bar_window);
	XSETFRAME! (frame, f);
	wset_frame (w, frame);
	window_bits! (pseudo_window_p, w, true);
      }
    else
      { w = XWINDOW ((*f).menu_bar_window); }

    /* Set window dimensions to frame dimensions and allocate or
       adjust glyph matrices of W.  */
    (*w).pixel_left = 0;
    (*w).left_col = 0;
    (*w).pixel_top = 0;
    (*w).top_line = 0;
    (*w).pixel_width = (FRAME_PIXEL_WIDTH (f)
			- 2 * FRAME_INTERNAL_BORDER_WIDTH (f));
    (*w).total_cols = FRAME_TOTAL_COLS (f);
    (*w).pixel_height = FRAME_MENU_BAR_HEIGHT (f);
    (*w).total_lines = FRAME_MENU_BAR_LINES (f);
    allocate_matrices_for_window_redisplay (w);
  }
  }

  #[cfg (feature = "have_window_system")]
  {
  {
    /* Allocate/ reallocate matrices of the tab bar window.  If we
       don't have a tab bar window yet, make one.  */
    let mut w: *mut window;
    if NILP ((*f).tab_bar_window)
      {
	let mut frame: Lisp_Object;
	fset_tab_bar_window (f, make_window ());
	w = XWINDOW ((*f).tab_bar_window);
	XSETFRAME! (frame, f);
	wset_frame (w, frame);
	window_bits! (pseudo_window_p, w, true);
      }
    else
      { w = XWINDOW ((*f).tab_bar_window); }

    (*w).pixel_left = 0;
    (*w).left_col = 0;

    /* Note that tab and tool bar windows appear above the internal
       border, as enforced by WINDOW_TOP_EDGE_Y.  */

    (*w).pixel_top = (FRAME_MENU_BAR_HEIGHT (f)
		      + (if !NILP (Vtab_bar_position)
			   { FRAME_TOOL_BAR_TOP_HEIGHT (f) } else { 0 }));
    (*w).top_line = (FRAME_MENU_BAR_LINES (f)
		     + (if !NILP (Vtab_bar_position)
			  { FRAME_TOOL_BAR_TOP_LINES (f) } else { 0 }));
    (*w).total_cols = FRAME_TOTAL_COLS (f);
    (*w).pixel_width = (FRAME_PIXEL_WIDTH (f)
			- 2 * FRAME_INTERNAL_BORDER_WIDTH (f));
    (*w).total_lines = FRAME_TAB_BAR_LINES (f);
    (*w).pixel_height = FRAME_TAB_BAR_HEIGHT (f);
    allocate_matrices_for_window_redisplay (w);
  }
  }

  #[cfg (feature = "window_system_no_ext_tool_bar")]
  {
  {
    /* Allocate/ reallocate matrices of the tool bar window.  If we
       don't have a tool bar window yet, make one.  */
    let mut w: *mut window;
    if NILP ((*f).tool_bar_window)
      {
	let mut frame: Lisp_Object;
	fset_tool_bar_window (f, make_window ());
	w = XWINDOW ((*f).tool_bar_window);
	XSETFRAME! (frame, f);
	wset_frame (w, frame);
	window_bits! (pseudo_window_p, w, true);
      }
    else
      { w = XWINDOW ((*f).tool_bar_window); }

    (*w).pixel_left = 0;
    (*w).left_col = 0;

    /* If the tool bar should be placed at the bottom of the frame,
       place it there instead, outside the internal border.  */

    if EQ (FRAME_TOOL_BAR_POSITION! (f), Qbottom ())
      {
	(*w).pixel_top = (FRAME_PIXEL_HEIGHT! (f)
			  - FRAME_TOOL_BAR_HEIGHT! (f));
	(*w).top_line = (FRAME_LINES! (f)
			 - FRAME_TOOL_BAR_LINES! (f));
      }
    else
      {
	/* Otherwise, place the window at the top of the frame.  */

	(*w).pixel_top = (FRAME_MENU_BAR_HEIGHT (f)
			  + (if NILP (Vtab_bar_position)
			       { FRAME_TAB_BAR_HEIGHT (f) } else { 0 }));
	(*w).top_line = (FRAME_MENU_BAR_LINES (f)
			 + (if NILP (Vtab_bar_position)
			      { FRAME_TAB_BAR_LINES (f) } else { 0 }));
      }

    (*w).total_cols = FRAME_TOTAL_COLS (f);
    (*w).pixel_width = (FRAME_PIXEL_WIDTH (f)
			- 2 * FRAME_INTERNAL_BORDER_WIDTH (f));
    (*w).total_lines = FRAME_TOOL_BAR_LINES! (f);
    (*w).pixel_height = FRAME_TOOL_BAR_HEIGHT! (f);
    allocate_matrices_for_window_redisplay (w);
  }
  }
}


/* Re-allocate buffer for decode_mode_spec on frame F.  */

unsafe fn adjust_decode_mode_spec_buffer (f: *mut frame)
{
  let frame_message_buf_size: c_int = FRAME_MESSAGE_BUF_SIZE (f);

  eassert! (frame_message_buf_size >= 0);
  (*f).decode_mode_spec_buffer = xrealloc ((*f).decode_mode_spec_buffer as *mut c_void,
					   (frame_message_buf_size + 1) as usize) as *mut c_char;
}


/**********************************************************************
			Freeing Glyph Matrices
 **********************************************************************/

/* Free glyph memory for a frame F.  F may be null.  This function can
   be called for the same frame more than once.  The root window of
   F may be nil when this function is called.  This is the case when
   the function is called when F is destroyed.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn free_glyphs (f: *mut frame)
{
  if !f.is_null () && glyphs_initialized_p! (f)
    {
      /* Block interrupt input so that we don't get surprised by an X
         event while we're in an inconsistent state.  */
      block_input ();
      glyphs_initialized_p! (f, false);

      /* Release window sub-matrices.  */
      if !NILP ((*f).root_window)
        { free_window_matrices (XWINDOW ((*f).root_window)); }

      #[cfg (feature = "window_system_no_ext_menu_bar")]
      {
      /* Free the dummy window for menu bars without X toolkit and its
	 glyph matrices.  */
      if !NILP ((*f).menu_bar_window)
	{
	  let w: *mut window = XWINDOW ((*f).menu_bar_window);
	  free_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);
	  free_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);
	  (*w).current_matrix = core::ptr::null_mut ();
	  (*w).desired_matrix = (*w).current_matrix;
	  fset_menu_bar_window (f, Qnil ());
	}
      }

      #[cfg (feature = "have_window_system")]
      {
      /* Free the tab bar window and its glyph matrices.  */
      if !NILP ((*f).tab_bar_window)
	{
	  let w: *mut window = XWINDOW ((*f).tab_bar_window);
	  free_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);
	  free_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);
	  (*w).current_matrix = core::ptr::null_mut ();
	  (*w).desired_matrix = (*w).current_matrix;
	  fset_tab_bar_window (f, Qnil ());
	}
      }

      #[cfg (feature = "window_system_no_ext_tool_bar")]
      {
      /* Free the tool bar window and its glyph matrices.  */
      if !NILP ((*f).tool_bar_window)
	{
	  let w: *mut window = XWINDOW ((*f).tool_bar_window);
	  free_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);
	  free_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);
	  (*w).current_matrix = core::ptr::null_mut ();
	  (*w).desired_matrix = (*w).current_matrix;
	  fset_tool_bar_window (f, Qnil ());
	}
      }

      /* Release frame glyph matrices.  Reset fields to zero in
	 case we are called a second time.  */
      if !(*f).desired_matrix.is_null ()
	{
	  free_glyph_matrix ((*f).desired_matrix as *mut glyph_matrix);
	  free_glyph_matrix ((*f).current_matrix as *mut glyph_matrix);
	  (*f).current_matrix = core::ptr::null_mut ();
	  (*f).desired_matrix = (*f).current_matrix;
	}

      /* Release glyph pools.  */
      if !(*f).desired_pool.is_null ()
	{
	  free_glyph_pool ((*f).desired_pool as *mut glyph_pool);
	  free_glyph_pool ((*f).current_pool as *mut glyph_pool);
	  (*f).current_pool = core::ptr::null_mut ();
	  (*f).desired_pool = (*f).current_pool;
	}

      unblock_input ();
    }
}


/* Free glyph sub-matrices in the window tree rooted at W.  This
   function may be called with a null pointer, and it may be called on
   the same tree more than once.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn free_window_matrices (w: *mut window)
{
  let mut w: *mut window = w;

  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{ free_window_matrices (XWINDOW ((*w).contents)); }
      else
	{
	  /* This is a leaf window.  Free its memory and reset fields
	     to zero in case this function is called a second time for
	     W.  */
	  free_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);
	  free_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);
	  (*w).desired_matrix = core::ptr::null_mut ();
	  (*w).current_matrix = (*w).desired_matrix;
	}

      /* Next window on same level.  */
      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
}


/* Check glyph memory leaks.  This function is called from
   shut_down_emacs.  Note that frames are not destroyed when Emacs
   exits.  We therefore free all glyph memory for all active frames
   explicitly and check that nothing is left allocated.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn check_glyph_memory ()
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;

  /* Free glyph memory for all frames.  */
  FOR_EACH_FRAME! (tail, frame, {
    free_glyphs (XFRAME (frame));
  });

  #[cfg (all (feature = "glyph_debug", enable_checking))]
  {
    /* Check that nothing is left allocated.  */
    eassert! (glyph_matrix_count == 0);
    eassert! (glyph_pool_count == 0);
  }
}


/**********************************************************************
		       Building a Frame Matrix
 **********************************************************************/

/* Most of the redisplay code works on glyph matrices attached to
   windows.  This is a good solution most of the time, but it is not
   suitable for terminal code.  Terminal output functions cannot rely
   on being able to set an arbitrary terminal window.  Instead they
   must be provided with a view of the whole frame, i.e. the whole
   screen.  We build such a view by constructing a frame matrix from
   window matrices in this section.

   Windows that must be updated have their must_be_updated_p flag set.
   For all such windows, their desired matrix is made part of the
   desired frame matrix.  For other windows, their current matrix is
   made part of the desired frame matrix.

   +-----------------+----------------+
   |     desired     |   desired      |
   |                 |                |
   +-----------------+----------------+
   |               current            |
   |                                  |
   +----------------------------------+

   Desired window matrices can be made part of the frame matrix in a
   cheap way: We exploit the fact that the desired frame matrix and
   desired window matrices share their glyph memory.  This is not
   possible for current window matrices.  Their glyphs are copied to
   the desired frame matrix.  The latter is equivalent to
   preserve_other_columns in the old redisplay.

   Used glyphs counters for frame matrix rows are the result of adding
   up glyph lengths of the window matrices.  A line in the frame
   matrix is enabled, if a corresponding line in a window matrix is
   enabled.

   After building the desired frame matrix, it will be passed to
   terminal code, which will manipulate both the desired and current
   frame matrix.  Changes applied to the frame's current matrix have
   to be visible in current window matrices afterwards, of course.

   This problem is solved like this:

   1. Window and frame matrices share glyphs.  Window matrices are
   constructed in a way that their glyph contents ARE the glyph
   contents needed in a frame matrix.  Thus, any modification of
   glyphs done in terminal code will be reflected in window matrices
   automatically.

   2. Exchanges of rows in a frame matrix done by terminal code are
   intercepted by hook functions so that corresponding row operations
   on window matrices can be performed.  This is necessary because we
   use pointers to glyphs in glyph row structures.  To satisfy the
   assumption of point 1 above that glyphs are updated implicitly in
   window matrices when they are manipulated via the frame matrix,
   window and frame matrix must of course agree where to find the
   glyphs for their rows.  Possible manipulations that must be
   mirrored are assignments of rows of the desired frame matrix to the
   current frame matrix and scrolling the current frame matrix.  */

/* Build frame F's desired matrix from window matrices.  Only windows
   which have the flag must_be_updated_p set have to be updated.  Menu
   bar lines of a frame are not covered by window matrices, so make
   sure not to touch them in this function.  */

unsafe fn build_frame_matrix (f: *mut frame)
{
  let mut i: c_int;

  /* F must have a frame matrix when this function is called.  */
  eassert! (!FRAME_WINDOW_P (f));

  /* Clear all rows in the frame matrix covered by window matrices.
     Menu bar lines are not covered by windows.  */
  i = FRAME_TOP_MARGIN (f);
  while i < (*( (*f).desired_matrix as *mut glyph_matrix )).nrows
    {
      clear_glyph_row (MATRIX_ROW! ((*f).desired_matrix as *mut glyph_matrix, i));
      i += 1;
    }

  /* Build the matrix by walking the window tree.  */
  build_frame_matrix_from_window_tree ((*f).desired_matrix as *mut glyph_matrix,
				       XWINDOW (FRAME_ROOT_WINDOW (f)));
}


/* Walk a window tree, building a frame matrix MATRIX from window
   matrices.  W is the root of a window tree.  */

unsafe fn build_frame_matrix_from_window_tree (matrix: *mut glyph_matrix, w: *mut window)
{
  let mut w: *mut window = w;

  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{ build_frame_matrix_from_window_tree (matrix, XWINDOW ((*w).contents)); }
      else
	{ build_frame_matrix_from_leaf_window (matrix, w); }

      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
}


/* Add a window's matrix to a frame matrix.  FRAME_MATRIX is the
   desired frame matrix built.  W is a leaf window whose desired or
   current matrix is to be added to FRAME_MATRIX.  W's flag
   must_be_updated_p determines which matrix it contributes to
   FRAME_MATRIX.  If W->must_be_updated_p, W's desired matrix
   is added to FRAME_MATRIX, otherwise W's current matrix is added.
   Adding a desired matrix means setting up used counters and such in
   frame rows, while adding a current window matrix to FRAME_MATRIX
   means copying glyphs.  The latter case corresponds to
   preserve_other_columns in the old redisplay.  */

unsafe fn build_frame_matrix_from_leaf_window (frame_matrix: *mut glyph_matrix,
					       w: *mut window)
{
  let mut window_matrix: *mut glyph_matrix;
  let mut window_y: c_int; let mut frame_y: c_int;
  /* If non-zero, a glyph to insert at the right border of W.  */
  let mut right_border_glyph: GLYPH;
  let f: *mut frame = XFRAME ((*w).frame);

  right_border_glyph = GLYPH { ch: 0, face_id: 0 };
  SET_GLYPH_FROM_CHAR! (right_border_glyph, 0);

  /* Set window_matrix to the matrix we have to add to FRAME_MATRIX.  */
  if window_bits! (must_be_updated_p, w)
    {
      window_matrix = (*w).desired_matrix as *mut glyph_matrix;

      /* Decide whether we want to add a vertical border glyph.  */
      if !WINDOW_RIGHTMOST_P! (w)
	{
	  let dp: *mut Lisp_Char_Table = window_display_table (w);
	  let mut gc: Lisp_Object = Qnil ();

	  SET_GLYPH_FROM_CHAR! (right_border_glyph, '|' as c_int);
	  if !dp.is_null ()
	      && { gc = DISP_BORDER_GLYPH (dp); GLYPH_CODE_P (gc) }
	    {
	      SET_GLYPH_FROM_GLYPH_CODE! (right_border_glyph, gc);
	      spec_glyph_lookup_face (w, &mut right_border_glyph);
	    }

	  if GLYPH_FACE (right_border_glyph) <= 0
	    { SET_GLYPH_FACE! (right_border_glyph, VERTICAL_BORDER_FACE_ID); }
	}
    }
  else
    { window_matrix = (*w).current_matrix as *mut glyph_matrix; }

  /* For all rows in the window matrix and corresponding rows in the
     frame matrix.  */
  window_y = 0;
  frame_y = (*window_matrix).matrix_y;
  while window_y < (*window_matrix).nrows
    {
      let frame_row: *mut glyph_row = (*frame_matrix).rows.offset (frame_y as isize);
      let mut window_row: *mut glyph_row = (*window_matrix).rows.offset (window_y as isize);
      let mut current_row_p: bool =
	(window_matrix as *mut c_void) == (*w).current_matrix;

      /* Fill up the frame row with spaces up to the left margin of the
	 window row.  */
      fill_up_frame_row_with_spaces (f, frame_row, (*window_matrix).matrix_x);

      /* Fill up areas in the window matrix row with spaces.  */
      fill_up_glyph_row_with_spaces (f, window_row);

      /* If only part of W's desired matrix has been built, and
         window_row wasn't displayed, use the corresponding current
         row instead.  */
      if (window_matrix as *mut c_void) == (*w).desired_matrix
	  && !glyph_row_bits! (enabled_p, window_row)
	{
	  window_row = (*( (*w).current_matrix as *mut glyph_matrix )).rows
	    .offset (window_y as isize);
	  current_row_p = true;
	}

      /* If someone asks why we are copying current glyphs here, and
	 maybe never enable the desired frame row we copy to:

	 - there might be a window to the right of this one that has a
	   corresponding desired window row.
	 - we need the complete frame row for scrolling.  */
      if current_row_p
	{
	  /* If the desired glyphs for this row haven't been built, copy
	     from the corresponding current row. If that row is not
	     enabled, its contents might be invalid.  Make sure that
	     glyphs have valid frames set in that case.  This is closer
	     to what we did before child frames were added, and seems to
	     be something tty redisplay implicitly relies on.  */
	  let to: *mut glyph = (*frame_row).glyphs [TEXT_AREA as usize]
	    .offset ((*window_matrix).matrix_x as isize);
	  let from: *mut glyph = (*window_row).glyphs [0];
	  let mut i: c_int = 0;
	  while i < (*window_matrix).matrix_w
	    {
	      *to.offset (i as isize) = *from.offset (i as isize);
	      if !glyph_row_bits! (enabled_p, window_row)
		{ (*to.offset (i as isize)).frame = f as *mut c_void; }
	      i += 1;
	    }
	}
      else
	{
	  eassert! (glyph_row_bits! (enabled_p, window_row));

	  /* Only when a desired row has been displayed, we want
	     the corresponding frame row to be updated.  */
	  glyph_row_bits! (enabled_p, frame_row, true);

	  /* Maybe insert a vertical border between horizontally adjacent
	     windows.  */
	  if GLYPH_CHAR (right_border_glyph) != 0
	    {
	      let border: *mut glyph = (*window_row).glyphs [LAST_AREA as usize].offset (-1);
	      /* It's a subtle bug if we are overwriting some non-char
		 glyph with the vertical border glyph.  */
	      eassert! (glyph_bits! (type, *border) == CHAR_GLYPH);
	      glyph_bits! (type, *border, CHAR_GLYPH);
	      SET_CHAR_GLYPH_FROM_GLYPH! (f as *mut c_void, *border, right_border_glyph);
	    }

	  #[cfg (feature = "glyph_debug")]
	  {
	  /* Window row window_y must be a slice of frame row
	     frame_y.  */
	  eassert! (frame_size_change_delayed (XFRAME ((*w).frame))
		    || glyph_row_slice_p (window_row, frame_row));

	  /* If rows are in sync, we don't have to copy glyphs because
	     frame and window share glyphs.  */

	  strcpy ((*( (*w).current_matrix as *mut glyph_matrix )).method.as_mut_ptr (),
		  (*( (*w).desired_matrix as *mut glyph_matrix )).method.as_ptr ());
	  add_window_display_history (w,
				      (*( (*w).current_matrix as *mut glyph_matrix )).method
				      .as_ptr ());
	  }
	}

      /* Set number of used glyphs in the frame matrix.  Since we fill
         up with spaces, and visit leaf windows from left to right it
         can be done simply.  */
      (*frame_row).used [TEXT_AREA as usize]
	= ((*window_matrix).matrix_x + (*window_matrix).matrix_w) as c_short;

      /* Next row.  */
      window_y += 1;
      frame_y += 1;
    }
}

/* Given a user-specified glyph, possibly including a Lisp-level face
   ID, return a glyph that has a realized face ID.
   This is used for glyphs displayed specially and not part of the text;
   for instance, vertical separators, truncation markers, etc.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn spec_glyph_lookup_face
  (w: *mut window, glyph: *mut GLYPH)
{
  let lface_id: c_int = GLYPH_FACE (*glyph);
  /* Convert the glyph's specified face to a realized (cache) face.  */
  if lface_id > 0
    {
      let face_id: c_int = merge_faces (w, Qt (), lface_id, DEFAULT_FACE_ID);
      SET_GLYPH_FACE! (*glyph, face_id);
    }
}

/* Add spaces to a glyph row ROW in a window matrix.

   Each row has the form:

   +---------+-----------------------------+------------+
   | left    |	text			   | right	|
   +---------+-----------------------------+------------+

   Left and right marginal areas are optional.  This function adds
   spaces to areas so that there are no empty holes between areas.
   In other words:  If the right area is not empty, the text area
   is filled up with spaces up to the right area.   If the text area
   is not empty, the left area is filled up.

   To be called for frame-based redisplay, only.  */

unsafe fn fill_up_glyph_row_with_spaces (f: *mut frame, row: *mut glyph_row)
{
  fill_up_glyph_row_area_with_spaces (f, row, LEFT_MARGIN_AREA);
  fill_up_glyph_row_area_with_spaces (f, row, TEXT_AREA);
  fill_up_glyph_row_area_with_spaces (f, row, RIGHT_MARGIN_AREA);
}


/* Fill area AREA of glyph row ROW with spaces.  To be called for
   frame-based redisplay only.  */

unsafe fn fill_up_glyph_row_area_with_spaces (f: *mut frame, row: *mut glyph_row, area: c_int)
{
  if (*row).glyphs [area as usize] < (*row).glyphs [area as usize + 1]
    {
      let end: *mut glyph = (*row).glyphs [area as usize + 1];
      let mut text: *mut glyph = (*row).glyphs [area as usize]
	.offset ((*row).used [area as usize] as isize);

      while text < end
	{
	  *text = space_glyph;
	  (*text).frame = f as *mut c_void;
	  text = text.offset (1);
	}
      (*row).used [area as usize] = text.offset_from ((*row).glyphs [area as usize]) as c_short;
    }
}


/* Add spaces to the end of ROW in a frame matrix until index UPTO is
   reached.  In frame matrices only one area, TEXT_AREA, is used.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn fill_up_frame_row_with_spaces
  (f: *mut frame, row: *mut glyph_row, upto: c_int)
{
  let mut i: c_int = (*row).used [TEXT_AREA as usize] as c_int;
  let glyph: *mut glyph = (*row).glyphs [TEXT_AREA as usize];

  while i < upto
    {
      *glyph.offset (i as isize) = space_glyph;
      (*glyph.offset (i as isize)).frame = f as *mut c_void;
      i += 1;
    }

  (*row).used [TEXT_AREA as usize] = i as c_short;
}


/**********************************************************************
      Mirroring operations on frame matrices in window matrices
 **********************************************************************/

/* Make sure glyph row ROW in CURRENT_MATRIX is up to date.
   DESIRED_MATRIX is the desired matrix corresponding to
   CURRENT_MATRIX.  The update is done by exchanging glyph pointers
   between rows in CURRENT_MATRIX and DESIRED_MATRIX.  If
   frame_matrix_frame is non-null, this indicates that the exchange is
   done in frame matrices, and that we have to perform analogous
   operations in window matrices of frame_matrix_frame.  */

unsafe fn make_current (f: *mut frame, w: *mut window, row: c_int)
{
  let desired_matrix: *mut glyph_matrix = if !f.is_null ()
    { (*f).desired_matrix as *mut glyph_matrix }
    else { (*w).desired_matrix as *mut glyph_matrix };
  let current_matrix: *mut glyph_matrix = if !f.is_null ()
    { (*f).current_matrix as *mut glyph_matrix }
    else { (*w).current_matrix as *mut glyph_matrix };
  let current_row: *mut glyph_row = MATRIX_ROW! (current_matrix, row);
  let desired_row: *mut glyph_row = MATRIX_ROW! (desired_matrix, row);
  let mouse_face_p: bool = glyph_row_bits! (mouse_face_p, current_row);

  /* If we aborted redisplay of this window, a row in the desired
     matrix might not have its hash computed.  But update_window
     relies on each row having its correct hash, so do it here if
     needed.  */
  if (*desired_row).hash == 0
      /* A glyph row that is not completely empty is unlikely to have
	 a zero hash value.  */
      && !((*desired_row).used [0] == 0
	   && (*desired_row).used [1] == 0
	   && (*desired_row).used [2] == 0)
    { (*desired_row).hash = row_hash (desired_row); }

  /* Do current_row = desired_row.  This exchanges glyph pointers
     between both rows, and does a structure assignment otherwise.  */
  assign_row (current_row, desired_row);

  /* Enable current_row to mark it as valid.  */
  glyph_row_bits! (enabled_p, current_row, true);
  glyph_row_bits! (mouse_face_p, current_row, mouse_face_p);

  /* If we are called on frame matrices, perform analogous operations
     for window matrices.  */
  if !f.is_null ()
    { mirror_make_current (XWINDOW ((*f).root_window), row); }
}


/* W is the root of a window tree.  FRAME_ROW is the index of a row in
   W's frame which has been made current (by swapping pointers between
   current and desired matrix).  Perform analogous operations in the
   matrices of leaf windows in the window tree rooted at W.  */

unsafe fn mirror_make_current (w: *mut window, frame_row: c_int)
{
  let mut w: *mut window = w;

  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
 	{ mirror_make_current (XWINDOW ((*w).contents), frame_row); }
      else
	{
	  /* Row relative to window W.  Don't use FRAME_TO_WINDOW_VPOS
	     here because the checks performed in debug mode there
	     will not allow the conversion.  */
	  let row: c_int = frame_row
	    - (*( (*w).desired_matrix as *mut glyph_matrix )).matrix_y;

	  /* If FRAME_ROW is within W, assign the desired row to the
	     current row (exchanging glyph pointers).  */
	  if row >= 0 && row < (*( (*w).desired_matrix as *mut glyph_matrix )).matrix_h
	    {
	      let current_row: *mut glyph_row
		= MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, row);
	      let desired_row: *mut glyph_row
		= MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, row);

	      if glyph_row_bits! (enabled_p, desired_row)
		{ assign_row (current_row, desired_row); }
	      else
		{ swap_glyph_pointers (desired_row, current_row); }
	      glyph_row_bits! (enabled_p, current_row, true);

	      /* Set the Y coordinate of the mode/header line's row.
		 It is needed in draw_row_with_mouse_face to find the
		 screen coordinates.  (Window-based redisplay sets
		 this in update_window, but no one seems to do that
		 for frame-based redisplay.)  */
	      if glyph_row_bits! (mode_line_p, current_row)
		{ (*current_row).y = row; }
	    }
	}

      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
}


/* Perform row dance after scrolling.  We are working on the range of
   lines UNCHANGED_AT_TOP + 1 to UNCHANGED_AT_TOP + NLINES (not
   including) in MATRIX.  COPY_FROM is a vector containing, for each
   row I in the range 0 <= I < NLINES, the index of the original line
   to move to I.  This index is relative to the row range, i.e. 0 <=
   index < NLINES.  RETAINED_P is a vector containing zero for each
   row 0 <= I < NLINES which is empty.

   This function is called from do_scrolling and do_direct_scrolling.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn mirrored_line_dance
  (f: *mut frame, unchanged_at_top: c_int, nlines: c_int, copy_from: *mut c_int,
   retained_p: *mut c_char)
{
  let matrix: *mut glyph_matrix = (*f).current_matrix as *mut glyph_matrix;

  /* A copy of original rows.  */
  let mut old_rows: *mut glyph_row = core::ptr::null_mut ();

  /* Rows to assign to.  */
  let new_rows: *mut glyph_row = MATRIX_ROW! (matrix, unchanged_at_top);

  let mut i: c_int;

  /* Make a copy of the original rows.  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  SAFE_NALLOCA! (sa_avail, old_rows, 1, nlines);
  std::ptr::copy_nonoverlapping (new_rows as *const u8, old_rows as *mut u8,
				 nlines as usize * core::mem::size_of::<glyph_row> ());

  /* Assign new rows, maybe clear lines.  */
  i = 0;
  while i < nlines
    {
      let enabled_before_p: bool = glyph_row_bits! (enabled_p, new_rows.offset (i as isize));

      eassert! (i + unchanged_at_top < (*matrix).nrows);
      eassert! (unchanged_at_top + *copy_from.offset (i as isize) < (*matrix).nrows);
      *new_rows.offset (i as isize) = *old_rows.offset (*copy_from.offset (i as isize) as isize);
      glyph_row_bits! (enabled_p, new_rows.offset (i as isize), enabled_before_p);

      /* RETAINED_P is zero for empty lines.  */
      if *retained_p.offset (*copy_from.offset (i as isize) as isize) == 0
	{ glyph_row_bits! (enabled_p, new_rows.offset (i as isize), false); }
      i += 1;
    }

  /* Do the same for window matrices, if MATRIX is a frame matrix.  */
  mirror_line_dance (XWINDOW ((*f).root_window),
		     unchanged_at_top, nlines, copy_from, retained_p);

  SAFE_FREE! (sa_count, sa_mark);
}


/* Synchronize glyph pointers in the current matrix of window W with
   the current frame matrix.  */

unsafe fn sync_window_with_frame_matrix_rows (w: *mut window)
{
  let f: *mut frame = XFRAME ((*w).frame);
  let mut window_row: *mut glyph_row;
  let window_row_end: *mut glyph_row;
  let mut frame_row: *mut glyph_row;
  let left: c_int; let right: c_int; let x: c_int; let width: c_int;

  /* Preconditions: W must be a live window on a tty frame.  */
  eassert! (BUFFERP ((*w).contents));
  eassert! (!FRAME_WINDOW_P (f));

  left = margin_glyphs_to_reserve (w, 1, (*w).left_margin_cols);
  right = margin_glyphs_to_reserve (w, 1, (*w).right_margin_cols);
  x = (*( (*w).current_matrix as *mut glyph_matrix )).matrix_x;
  width = (*( (*w).current_matrix as *mut glyph_matrix )).matrix_w;

  window_row = (*( (*w).current_matrix as *mut glyph_matrix )).rows;
  window_row_end = window_row.offset (
      (*( (*w).current_matrix as *mut glyph_matrix )).nrows as isize);
  frame_row = (*( (*f).current_matrix as *mut glyph_matrix )).rows
    .offset (WINDOW_TOP_EDGE_LINE (w) as isize);

  while window_row < window_row_end
    {
      (*window_row).glyphs [LEFT_MARGIN_AREA as usize]
	= (*frame_row).glyphs [0].offset (x as isize);
      (*window_row).glyphs [TEXT_AREA as usize]
	= (*window_row).glyphs [LEFT_MARGIN_AREA as usize].offset (left as isize);
      (*window_row).glyphs [LAST_AREA as usize]
	= (*window_row).glyphs [LEFT_MARGIN_AREA as usize].offset (width as isize);
      (*window_row).glyphs [RIGHT_MARGIN_AREA as usize]
	= (*window_row).glyphs [LAST_AREA as usize].offset (-(right as isize));
      window_row = window_row.offset (1); frame_row = frame_row.offset (1);
    }
}


/* Return the window in the window tree rooted in W containing frame
   row ROW.  Value is null if none is found.  */

unsafe fn frame_row_to_window (w: *mut window, row: c_int) -> *mut window
{
  let mut found: *mut window = core::ptr::null_mut ();
  let mut w: *mut window = w;

  while !w.is_null () && found.is_null ()
    {
      if WINDOWP ((*w).contents)
 	{ found = frame_row_to_window (XWINDOW ((*w).contents), row); }
      else if row >= WINDOW_TOP_EDGE_LINE (w)
	       && row < WINDOW_BOTTOM_EDGE_LINE (w)
	{ found = w; }

      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }

  return found;
}


/* Perform a line dance in the window tree rooted at W, after
   scrolling a frame matrix in mirrored_line_dance.

   We are working on the range of lines UNCHANGED_AT_TOP + 1 to
   UNCHANGED_AT_TOP + NLINES (not including) in W's frame matrix.
   COPY_FROM is a vector containing, for each row I in the range 0 <=
   I < NLINES, the index of the original line to move to I.  This
   index is relative to the row range, i.e. 0 <= index < NLINES.
   RETAINED_P is a vector containing zero for each row 0 <= I < NLINES
   which is empty.  */

unsafe fn mirror_line_dance (w: *mut window, unchanged_at_top: c_int, nlines: c_int,
			     copy_from: *mut c_int, retained_p: *mut c_char)
{
  let mut w: *mut window = w;

  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{ mirror_line_dance (XWINDOW ((*w).contents), unchanged_at_top,
			     nlines, copy_from, retained_p); }
      else
	{
	  /* W is a leaf window, and we are working on its current
	     matrix m.  */
	  let m: *mut glyph_matrix = (*w).current_matrix as *mut glyph_matrix;
	  let mut i: c_int;
	  let mut sync_p: bool = false;
	  let mut old_rows: *mut glyph_row = core::ptr::null_mut ();

	  /* Make a copy of the original rows of matrix m.  */
	  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
	  SAFE_NALLOCA! (sa_avail, old_rows, 1, (*m).nrows);
	  std::ptr::copy_nonoverlapping ((*m).rows as *const u8, old_rows as *mut u8,
					 (*m).nrows as usize
					 * core::mem::size_of::<glyph_row> ());

	  i = 0;
	  while i < nlines
	    {
	      /* Frame relative line assigned to.  */
	      let frame_to: c_int = i + unchanged_at_top;

	      /* Frame relative line assigned.  */
	      let frame_from: c_int = *copy_from.offset (i as isize) + unchanged_at_top;

	      /* Window relative line assigned to.  */
	      let window_to: c_int = frame_to - (*m).matrix_y;

	      /* Window relative line assigned.  */
	      let window_from: c_int = frame_from - (*m).matrix_y;

	      /* Is assigned line inside window?  */
	      let from_inside_window_p: bool
		= window_from >= 0 && window_from < (*m).matrix_h;

	      /* Is assigned to line inside window?  */
	      let to_inside_window_p: bool
		= window_to >= 0 && window_to < (*m).matrix_h;

	      if from_inside_window_p && to_inside_window_p
		{
		  /* Do the assignment.  The enabled_p flag is saved
		     over the assignment because the old redisplay did
		     that.  */
		  let enabled_before_p: bool =
		    glyph_row_bits! (enabled_p, (*m).rows.offset (window_to as isize));
		  *(*m).rows.offset (window_to as isize)
		    = *old_rows.offset (window_from as isize);
		  glyph_row_bits! (enabled_p, (*m).rows.offset (window_to as isize),
				   enabled_before_p);

		  /* If frame line is empty, window line is empty, too.  */
		  if *retained_p.offset (*copy_from.offset (i as isize) as isize) == 0
		    { glyph_row_bits! (enabled_p, (*m).rows.offset (window_to as isize), false); }
		}
	      else if to_inside_window_p
		{
		  /* A copy between windows.  This is an infrequent
		     case not worth optimizing.  */
		  let f: *mut frame = XFRAME ((*w).frame);
		  let root: *mut window = XWINDOW (FRAME_ROOT_WINDOW (f));
		  let w2: *mut window;
		  let m2: *mut glyph_matrix;
		  let m2_from: c_int;

		  w2 = frame_row_to_window (root, frame_from);
		  /* ttn@surf.glug.org: when enabling menu bar using `emacs
		     -nw', FROM_FRAME sometimes has no associated window.
		     This check avoids a segfault if W2 is null.  */
		  if !w2.is_null ()
		    {
		      m2 = (*w2).current_matrix as *mut glyph_matrix;
		      m2_from = frame_from - (*m2).matrix_y;
		      copy_row_except_pointers ((*m).rows.offset (window_to as isize),
						(*m2).rows.offset (m2_from as isize));

		      /* If frame line is empty, window line is empty, too.  */
		      if *retained_p.offset (*copy_from.offset (i as isize) as isize) == 0
			{ glyph_row_bits! (enabled_p, (*m).rows.offset (window_to as isize),
					   false); }
		    }
		  sync_p = true;
		}
	      else if from_inside_window_p
		{ sync_p = true; }
	      i += 1;
	    }

	  /* If there was a copy between windows, make sure glyph
	     pointers are in sync with the frame matrix.  */
	  if sync_p
	    { sync_window_with_frame_matrix_rows (w); }

	  /* Check that no pointers are lost.  */
	  CHECK_MATRIX! (m);

	  SAFE_FREE! (sa_count, sa_mark);
	}

      /* Next window on same level.  */
      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
}


/**********************************************************************
		      VPOS and HPOS translations
 **********************************************************************/

/* Translate vertical position VPOS which is relative to window W to a
   vertical position relative to W's frame.  */

unsafe fn window_to_frame_vpos (w: *mut window, vpos: c_int) -> c_int
{
  let mut vpos = vpos;
  eassert! (!FRAME_WINDOW_P (XFRAME ((*w).frame)));
  eassert! (vpos >= 0 && vpos <= (*( (*w).desired_matrix as *mut glyph_matrix )).nrows);
  vpos += WINDOW_TOP_EDGE_LINE (w);
  eassert! (frame_size_change_delayed (XFRAME ((*w).frame))
	    || (vpos >= 0 && vpos <= FRAME_TOTAL_LINES (XFRAME ((*w).frame))));
  return vpos;
}


/* Translate horizontal position HPOS which is relative to window W to
   a horizontal position relative to W's frame.  */

unsafe fn window_to_frame_hpos (w: *mut window, hpos: c_int) -> c_int
{
  let mut hpos = hpos;
  eassert! (!FRAME_WINDOW_P (XFRAME ((*w).frame)));
  hpos += WINDOW_LEFT_EDGE_COL (w);
  return hpos;
}


/**********************************************************************
			    Redrawing Frames
 **********************************************************************/

/* Redraw frame F.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn redraw_frame (f: *mut frame)
{
  /* Error if F has no glyphs.  */
  eassert! (glyphs_initialized_p! (f));
  update_begin (f);
  if FRAME_MSDOS_P (f)
    { ((*FRAME_TERMINAL! (f)).set_terminal_modes_hook.unwrap ()) (FRAME_TERMINAL! (f)); }

  if FRAME_WINDOW_P (f)
    /* Garbage the frame now.  Otherwise, platforms that support
       double buffering will display the blank contents of the frame
       even though the frame should be redrawn at some point in the
       future.  */
    { SET_FRAME_GARBAGED! (f); }

  /* clear_frame is actually a "clear_terminal", i.e.
     it clears the entire screen.  */
  if FRAME_PARENT_FRAME (f).is_null ()
    { clear_frame (f); }
  clear_current_matrices (f);
  update_end (f);
  fset_redisplay (f as *mut c_void);

  /* Mark all windows as inaccurate, so that every window will have
     its redisplay done.  */
  mark_window_display_accurate (FRAME_ROOT_WINDOW (f), false);
  set_window_update_flags (XWINDOW (FRAME_ROOT_WINDOW (f)), true);

  garbaged! (f, false);
}

DEFUN! ("redraw-frame", Fredraw_frame, Sredraw_frame, 0, 1, 0,
        doc: "Clear frame FRAME and output again what is supposed to appear on it.\nIf FRAME is omitted or nil, the selected frame is used.",
        (frame: Lisp_Object),
        {
  redraw_frame (decode_live_frame (frame));
  return Qnil ();
});

DEFUN! ("redraw-display", Fredraw_display, Sredraw_display, 0, 0, "",
        doc: "Clear and redisplay all visible frames.",
        (),
        {
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;

  FOR_EACH_FRAME! (tail, frame, {
    if frame_redisplay_p (XFRAME (frame))
      { redraw_frame (XFRAME (frame)); }
  });

  return Qnil ();
});


/**********************************************************************
			    TTY Child Frames
 **********************************************************************/

/* Child frames on ttys break the assumption that frames on a tty
   always occupy the whole terminal.  They can overlap instead.

   Let a "root" frame be a frame that has no parent frame.  Such root
   frames are required to be the size of the terminal screen.  The
   current glyph matrix of a root frame of a terminal represents what
   is on the screen.  The desired matrix of a root frame represents
   what should be one the screen.

   Building the desired matrix of root frame proceeds by

   - building the desired matrix of the root frame itself which is
     the bottommost frame in z-order;
   - building desired matrices of child frames in z-order, topmost last;
   - copying the desired glyphs from child frames to the desired glyphs
     of the root frame

   Updating the screen is then done using root frame matrices as it
   was before child frames were introduced.  Child frame's current
   matrices are updated by copying glyph contents of the current
   matrix of the root frames to the current matrices of child
   frames.  This implicitly also updates the glyph contents of their
   windows' current matrices.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct rect
{
  pub x: c_int,
  pub y: c_int,
  pub w: c_int,
  pub h: c_int,
}

const _: () = core::assert! (core::mem::size_of::<rect> () == 16);        // clang-measured
const _: () = core::assert! (core::mem::offset_of! (rect, x) == 0);
const _: () = core::assert! (core::mem::offset_of! (rect, h) == 12);

/// C: dispextern.h 3067-3200: `struct redisplay_interface`.  termhooks.rs degrades
/// `terminal.rif` to `*mut c_void` (P-72 rule 3), so a unit that reaches into the record must
/// name it itself (the minibuf.rs shape).  Every member C declares before the first one this
/// unit uses is a pointer, so the fields this unit does *not* touch stay opaque fixed-size
/// prefixes and each offset it depends on is pinned below (clang, frozen target).  The
/// `#ifdef HAVE_WINDOW_SYSTEM` tail is dead on this target and is not declared.
#[repr(C)]
pub struct redisplay_interface
{
  _prefix_parm_and_produce: [*mut c_void; 2],
  pub write_glyphs: Option<unsafe extern "C" fn (w: *mut window, row: *mut glyph_row,
                                                  string: *mut glyph, area: glyph_row_area,
                                                  len: c_int)>,
  _prefix_insert_glyphs: [*mut c_void; 1],
  pub clear_end_of_line: Option<unsafe extern "C" fn (w: *mut window, row: *mut glyph_row,
                                                      area: glyph_row_area, x: c_int)>,
  pub scroll_run_hook: Option<unsafe extern "C" fn (w: *mut window, run: *mut run)>,
  pub after_update_window_line_hook: Option<unsafe extern "C" fn (w: *mut window,
                                                                  desired_row: *mut glyph_row)>,
  pub update_window_begin_hook: Option<unsafe extern "C" fn (w: *mut window)>,
  pub update_window_end_hook: Option<unsafe extern "C" fn (w: *mut window, cursor_on_p: bool,
                                                           mouse_face_overwritten_p: bool)>,
  _prefix_flush_display: [*mut c_void; 1],
  pub clear_window_mouse_face: Option<unsafe extern "C" fn (w: *mut window)>,
  pub get_glyph_overhangs: Option<unsafe extern "C" fn (glyph: *mut glyph, f: *mut frame,
                                                       left: *mut c_int, right: *mut c_int)>,
  pub fix_overlapping_area: Option<unsafe extern "C" fn (w: *mut window, row: *mut glyph_row,
                                                        area: glyph_row_area,
                                                        overlaps: c_int)>,
}

const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, write_glyphs) == 16);
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, clear_end_of_line) == 32);
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, scroll_run_hook) == 40);
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface,
					      after_update_window_line_hook) == 48);
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, update_window_begin_hook) == 56);
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, update_window_end_hook) == 64);
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, clear_window_mouse_face) == 80);
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, get_glyph_overhangs) == 88);
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, fix_overlapping_area) == 96);

/* C: dispextern.h 1730-1945: `struct face` and `struct face_cache`, and dispextern.h
   1998-2010's `FACE_FROM_ID`.  `xfaces.rs` carries the records for its own unit, which this
   unit does not include; only `face.background` is read here, so the two offsets this unit
   depends on are pinned and the rest of the layout stays opaque where that is enough (the
   minibuf.rs `redisplay_interface` shape; clang, frozen target).  */
#[repr(C)]
pub struct face
{
  _prefix: [u8; 176],
  pub background: c_ulong,
}
const _: () = core::assert! (core::mem::offset_of! (face, background) == 176);

#[repr(C)]
pub struct face_cache
{
  _prefix: [u8; 16],
  pub faces_by_id: *mut *mut face,
  _prefix_size: ptrdiff_t,
  pub used: c_int,
}
const _: () = core::assert! (core::mem::offset_of! (face_cache, faces_by_id) == 16);
const _: () = core::assert! (core::mem::offset_of! (face_cache, used) == 32);

#[inline(always)]
fn FRAME_FACE_CACHE (f: *mut frame) -> *mut face_cache
{ unsafe { (*f).face_cache as *mut face_cache } }

/// C: dispextern.h 1998-2010: `FACE_FROM_ID`.
#[inline(always)]
pub fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
{
  eassert! (0 <= id && id < unsafe { (*FRAME_FACE_CACHE (f)).used });
  unsafe { *((*FRAME_FACE_CACHE (f)).faces_by_id).offset (id as isize) }
}

/* dispextern.h 1936-1940.  */
pub const FACE_TTY_DEFAULT_FG_COLOR: c_ulong = -2i64 as c_ulong;
pub const FACE_TTY_DEFAULT_BG_COLOR: c_ulong = -3i64 as c_ulong;

/* dispextern.h 1445-1446: the `OVERLAPS_PRED`/`OVERLAPS_SUCC` bits that
   `redraw_overlapping_rows` (dispextern.h 3126) ors together.  C defines them in
   `dispextern.h`, i.e. in the mirror - which carries them under `have_window_system`; the
   copies here are the same two constants for the configurations that do not compile that arm,
   and the same gate is what keeps the Android build from reading `E0428` twice over.  */
#[cfg (not (feature = "have_window_system"))]
pub const OVERLAPS_PRED: c_int = (1 << 0);
#[cfg (not (feature = "have_window_system"))]
pub const OVERLAPS_SUCC: c_int = (1 << 1);

/* C 3288-3827: `#ifndef HAVE_ANDROID` (live on the frozen target).  */

/* Compute the intersection of R1 and R2 in R.  Value is true if R1 and
   R2 intersect, false otherwise.  */

unsafe fn rect_intersect (r: *mut rect, r1: rect, r2: rect) -> bool
{
  let x: c_int = std::cmp::max (r1.x, r2.x);
  let y: c_int = std::cmp::max (r1.y, r2.y);
  let w: c_int = std::cmp::min (r1.x + r1.w, r2.x + r2.w) - x;
  let h: c_int = std::cmp::min (r1.y + r1.h, r2.y + r2.h) - y;

  if w <= 0 || h <= 0
    { return false; }

  *r = rect { x: x, y: y, w: w, h: h };
  return true;
}

/* Translate (X, Y) relative to frame F to absolute coordinates
   in (*X, *Y).  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn root_xy (f: *mut frame, x: c_int, y: c_int,
						      rx: *mut c_int, ry: *mut c_int)
{
  let mut f = f;
  *rx = x;
  *ry = y;
  while !f.is_null ()
    {
      *rx += (*f).left_pos;
      *ry += (*f).top_pos;
      f = FRAME_PARENT_FRAME (f);
    }
}

/* Translate absolute coordinates (X, Y) to coordinates relative to F's origin.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn child_xy (f: *mut frame, x: c_int, y: c_int,
						       cx: *mut c_int, cy: *mut c_int)
{
  let mut rx: c_int = 0; let mut ry: c_int = 0;
  root_xy (f, 0, 0, &mut rx, &mut ry);
  *cx = x - rx;
  *cy = y - ry;
}

/* Return the rectangle frame F occupies.  X and Y are in absolute
   coordinates.  */

unsafe fn frame_rect_abs (f: *mut frame) -> rect
{
  let mut x: c_int = 0; let mut y: c_int = 0;
  root_xy (f, 0, 0, &mut x, &mut y);
  return rect { x: x, y: y, w: (*f).total_cols, h: (*f).total_lines };
}

/* C 3356-3357: `#endif /* !HAVE_ANDROID */` closes the `#ifndef HAVE_ANDROID` that C 3299
   opened; the text above is live on the frozen target.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn max_child_z_order (parent: *mut frame) -> c_int
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut z_order: c_int = 0;
  FOR_EACH_FRAME! (tail, frame, {
      let f: *mut frame = XFRAME (frame);
      if FRAME_PARENT_FRAME (f) == parent
	{ z_order = std::cmp::max (z_order, (*f).z_order); }
  });
  return z_order;
}

/* Return true if and only if F and all its ancestors are visible.  */

unsafe fn frame_ancestors_visible_p (f: *mut frame) -> bool
{
  let mut f = f;
  while !f.is_null ()
    {
      if !FRAME_VISIBLE_P (f)
	{ return false; }
      else
	{ f = FRAME_PARENT_FRAME (f); }
    }

  return true;
}

/* Return a list of all frames having root frame ROOT.

   If VISIBLE_ONLY is true, return only frames that are visible and have
   visible ancestors only.  */

unsafe fn frames_with_root (root: *mut frame, visible_only: bool) -> Lisp_Object
{
  let mut list: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  FOR_EACH_FRAME! (tail, frame, {
      let f: *mut frame = XFRAME (frame);

      if root_frame (f) == root
	  && (!visible_only || frame_ancestors_visible_p (f))
	{ list = Fcons (frame, list); }
  });
  return list;
}

/* Return a list of frames having parent frame PARENT.
   If VISIBLE_ONLY is true, return only visible frames.  */

unsafe fn frames_with_parent (parent: *mut frame, visible_only: bool) -> Lisp_Object
{
  let mut list: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  FOR_EACH_FRAME! (tail, frame, {
      let f: *mut frame = XFRAME (frame);
      if FRAME_PARENT_FRAME (f) == parent
	  && (!visible_only || FRAME_VISIBLE_P (f))
	{ list = Fcons (frame, list); }
  });
  return list;
}

/* Compare frames F1 and F2 for z-order.  Value is like strcmp.  */

unsafe fn frame_z_order_cmp (f1: *mut frame, f2: *mut frame) -> c_int
{
  if f1 == f2
    { return 0; }
  if frame_ancestor_p (f1, f2)
    { return -1; }
  if frame_ancestor_p (f2, f1)
    { return 1; }
  return (*f1).z_order - (*f2).z_order;
}

DEFUN! ("frame--z-order-lessp", Fframe__z_order_lessp, Sframe__z_order_lessp,
        2, 2, 0,
        doc: "Internal frame sorting function A < B.",
        (a: Lisp_Object, b: Lisp_Object),
        {
  eassert! (FRAMEP (a) && FRAMEP (b));
  return if frame_z_order_cmp (XFRAME (a), XFRAME (b)) < 0 { Qt () } else { Qnil () };
});

/* Return a z-order list of frames with the same root as F.  The list
   is ordered topmost frame last.  Note that this list contains
   the root frame of F itself as first element.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn frames_in_reverse_z_order
  (f: *mut frame, visible_only: bool) -> Lisp_Object
{
  let root: *mut frame = root_frame (f);
  let mut frames: Lisp_Object = frames_with_root (root, visible_only);
  frames = CALLN! (Fsort, frames, QClessp (), Qframe__z_order_lessp ());
  eassert! (FRAMEP (XCAR (frames)));
  eassert! (XFRAME (XCAR (frames)) == root);
  return frames;
}

/* Raise or lower frame F in z-order.  If RAISE is true, raise F, else
   lower f.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn tty_raise_lower_frame (f: *mut frame, raise: bool)
{
  let parent: *mut frame = FRAME_PARENT_FRAME (f);
  if parent.is_null ()
    { return; }

  let mut siblings: Lisp_Object = frames_with_parent (parent, false);
  siblings = CALLN! (Fsort, siblings, QClessp (), Qframe__z_order_lessp ());

  let mut i: c_int = 0;
  let mut tail: Lisp_Object = siblings;
  while CONSP (tail)
    {
      let child: *mut frame = XFRAME (XCAR (tail));
      if child != f
	{ (*child).z_order = i; i += 1; }
      tail = XCDR (tail);
    }
  (*f).z_order = if raise { i } else { 0 };
}

/* Return true if frame F is a tty frame.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn is_tty_frame (f: *mut frame) -> bool
{
  return FRAME_TERMCAP_P (f) || FRAME_MSDOS_P (f);
}

/* Return true if frame F is a tty child frame.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn is_tty_child_frame (f: *mut frame) -> bool
{
  return !FRAME_PARENT_FRAME (f).is_null () && is_tty_frame (f);
}

/* Return true if frame F is a tty root frame.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn is_tty_root_frame (f: *mut frame) -> bool
{
  return FRAME_PARENT_FRAME (f).is_null () && is_tty_frame (f);
}

/* Return true if frame F is a tty root frame that has a visible child
   frame..  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn is_tty_root_frame_with_visible_child
  (f: *mut frame) -> bool
{
  if !is_tty_root_frame (f)
    { return false; }
  let z_order: Lisp_Object = frames_in_reverse_z_order (f, true);
  return CONSP (XCDR (z_order));
}

/* Return the index of the first enabled row in MATRIX, or -1 if there
   is none.  */

unsafe fn first_enabled_row (matrix: *mut glyph_matrix) -> c_int
{
  let mut i: c_int = 0;
  while i < (*matrix).nrows
    {
      if MATRIX_ROW_ENABLED_P! (matrix, i)
	{ return i; }
      i += 1;
    }
  return -1;
}


/* On tty frame F, make desired matrix current, without writing
   to the terminal.  */

unsafe fn make_matrix_current (f: *mut frame)
{
  let first_row: c_int = first_enabled_row ((*f).desired_matrix as *mut glyph_matrix);
  if first_row >= 0
    {
      let mut i: c_int = first_row;
      while i < (*( (*f).desired_matrix as *mut glyph_matrix )).nrows
	{
	  if MATRIX_ROW_ENABLED_P! ((*f).desired_matrix as *mut glyph_matrix, i)
	    { make_current (f, core::ptr::null_mut (), i); }
	  i += 1;
	}
    }
}

/* C 3550-3551: `#ifndef HAVE_ANDROID` — live on the frozen target.  */

/* Prepare ROOT's desired row at index Y for copying child frame
   contents to it.  Value is the prepared desired row or NULL if we
   don't have, and can't construct a desired row.  */

unsafe fn prepare_desired_root_row (root: *mut frame, y: c_int) -> *mut glyph_row
{
  /* If we have a desired row that has been displayed, use that.  */
  let desired_row: *mut glyph_row =
    MATRIX_ROW! ((*root).desired_matrix as *mut glyph_matrix, y);
  if glyph_row_bits! (enabled_p, desired_row)
    { return desired_row; }

  /* If we have a current row that is up to date, copy that to the
     desired row and use that.  Don't copy rows that are bot enabled, in
     particular because they might not have the 'frame' member of glyphs
     set.  */
  let current_row: *mut glyph_row =
    MATRIX_ROW! ((*root).current_matrix as *mut glyph_matrix, y);
  if glyph_row_bits! (enabled_p, current_row)
    {
      #[cfg (feature = "glyph_debug")]
      {
      /* Safety belt: Try to make sure that we don't copy glyphs from a
         stale current matrix that contains glyphs referring to dead
         frames. */
      let mut i: c_int = 0;
      while i < (*current_row).used [TEXT_AREA as usize] as c_int
	{
	  let glyph: *mut glyph = (*current_row).glyphs [TEXT_AREA as usize].offset (i as isize);
	  eassert! (!(*glyph).frame.is_null ()
		    && FRAME_LIVE_P ((*glyph).frame as *mut frame));
	  i += 1;
	}
      }
      std::ptr::copy_nonoverlapping ((*current_row).glyphs [0] as *const u8,
				     (*desired_row).glyphs [0] as *mut u8,
				     (*( (*root).current_matrix as *mut glyph_matrix )).matrix_w
				     as usize * core::mem::size_of::<glyph> ());
      glyph_row_bits! (enabled_p, desired_row, true);
      return desired_row;
    }

  return core::ptr::null_mut ();
}

/* Change GLYPH to be a space glyph.  */

unsafe fn make_glyph_space (glyph: *mut glyph)
{
  (*glyph).u.ch = ' ' as c_uint;
  (*glyph).pixel_width = 1;
  glyph_bits! (padding_p, *glyph, false);
}

/* On root frame ROOT, if the glyph in ROW at position X is part of a
   sequence of glyphs for a wide character, change every glyph belonging
   to the sequence to a space.  If X is outside of ROOT, do nothing.  */

unsafe fn neutralize_wide_char (root: *mut frame, row: *mut glyph_row, x: c_int)
{
  if x < 0 || x >= (*( (*root).desired_matrix as *mut glyph_matrix )).matrix_w
    { return; }

  let mut glyph: *mut glyph = (*row).glyphs [TEXT_AREA as usize].offset (x as isize);
  if glyph_bits! (type, *glyph) == CHAR_GLYPH && CHARACTER_WIDTH ((*glyph).u.ch as c_int) > 1
    {
      /* Glyph is somewhere in a sequence of glyphs for a wide
	 character, find the start.  */
      let row_start: *mut glyph = (*row).glyphs [TEXT_AREA as usize];
      while glyph > row_start && glyph_bits! (padding_p, *glyph)
	{ glyph = glyph.offset (-1); }

      /* Make everything in the sequence a space glyph.  */
      eassert! (!glyph_bits! (padding_p, *glyph));
      make_glyph_space (glyph);
      let row_limit: *mut glyph =
	row_start.offset ((*row).used [TEXT_AREA as usize] as isize);
      glyph = glyph.offset (1);
      while glyph < row_limit && glyph_bits! (padding_p, *glyph)
	{
	  make_glyph_space (glyph);
	  glyph = glyph.offset (1);
	}
    }
}

/* Fill *G with the code and face for box character BOX on frame F
   Value is true if a standard display table entry for BOX exists.  */

unsafe fn box_from_display_table (f: *mut frame, r#box: r#box, g: *mut GLYPH) -> bool
{
  if DISP_TABLE_P (Vstandard_display_table)
    {
      let dp: *mut Lisp_Char_Table = XCHAR_TABLE (Vstandard_display_table);
      let gc: Lisp_Object = FAM_ELEM! ((*dp).extras, r#box);
      if GLYPH_CODE_P (gc)
	{
	  SET_GLYPH_FROM_GLYPH_CODE! (*g, gc);
	  spec_glyph_lookup_face (XWINDOW ((*f).root_window), g);
	  if GLYPH_FACE (*g) == 0
	    { SET_GLYPH_FACE! (*g, BORDER_FACE_ID); }
	  return true;
	}
    }
  return false;
}

/* Fill *G with the default code and face for box BOX on frame F.  */

unsafe fn box_default (f: *mut frame, r#box: r#box, g: *mut GLYPH)
{
  let mut dflt: c_int;   /* C's `int dflt UNINIT;` — assigned on every path below.  */
  match r#box
    {
      BOX_VERTICAL => { dflt = '|' as c_int; }
      BOX_HORIZONTAL => { dflt = '-' as c_int; }
      BOX_DOWN_RIGHT | BOX_DOWN_LEFT | BOX_UP_RIGHT | BOX_UP_LEFT => { dflt = '+' as c_int; }
      BOX_DOUBLE_VERTICAL | BOX_DOUBLE_HORIZONTAL | BOX_DOUBLE_DOWN_RIGHT
      | BOX_DOUBLE_DOWN_LEFT | BOX_DOUBLE_UP_RIGHT | BOX_DOUBLE_UP_LEFT =>
	{ emacs_abort (); }
      _ => { emacs_abort (); }
    }

  let face_id: c_int = lookup_basic_face (core::ptr::null_mut (), f, BORDER_FACE_ID);
  SET_GLYPH! (*g, dflt, face_id);
}


/* Return the glyph for displaying BOX on frame F.  */

unsafe fn box_glyph (f: *mut frame, r#box: r#box) -> GLYPH
{
  let mut g: GLYPH = GLYPH { ch: 0, face_id: 0 };
  if !box_from_display_table (f, r#box, &mut g)
    { box_default (f, r#box, &mut g); }
  return g;
}

/* Produce glyphs for box character BOX in ROW.  X is the position in
   ROW where to start producing glyphs.  N is the number of glyphs to
   produce.  CHILD is the frame to use for the face of the glyphs.  */

unsafe fn produce_box_glyphs (r#box: r#box, row: *mut glyph_row, x: c_int, n: c_int,
			      child: *mut frame)
{
  let g: GLYPH = box_glyph (child, r#box);
  let mut glyph: *mut glyph = (*row).glyphs [0].offset (x as isize);
  let mut i: c_int = 0;
  while i < n
    {
      glyph_bits! (type, *glyph, CHAR_GLYPH);
      (*glyph).u.ch = GLYPH_CHAR (g) as c_uint;
      (*glyph).charpos = -1;
      (*glyph).pixel_width = 1;
      glyph_bits! (multibyte_p, *glyph, true);
      glyph_bits! (face_id, *glyph, GLYPH_FACE (g));
      (*glyph).frame = child as *mut c_void;
      glyph_bits! (padding_p, *glyph, false);
      (*glyph).object = Qnil ();
      glyph_bits! (padding_p, *glyph, false);
      i += 1; glyph = glyph.offset (1);
    }
}

/* Produce box glyphs LEFT and RIGHT in ROOT_ROW.  X and W are the start
   and width of a range in ROOT_ROW before and after which to put the
   box glyphs, if they fit.  ROOT and CHILD are root and child frame we
   are working on.  ROOT is the root frame whose matrix dimensions
   determines if the box glyphs fit.  CHILD is the frame whose faces to
   use for the box glyphs.  */

unsafe fn produce_box_sides (left: r#box, right: r#box, root_row: *mut glyph_row,
			     x: c_int, w: c_int, root: *mut frame, child: *mut frame)
{
  if x > 0
    {
      neutralize_wide_char (root, root_row, x - 1);
      produce_box_glyphs (left, root_row, x - 1, 1, child);
    }

  if x + w < (*( (*root).desired_matrix as *mut glyph_matrix )).matrix_w
    {
      neutralize_wide_char (root, root_row, x + w);
      produce_box_glyphs (right, root_row, x + w, 1, child);
    }
}

unsafe fn produce_box_line (root: *mut frame, child: *mut frame, x: c_int, y: c_int,
			    w: c_int, first: bool)
{
  let root_row: *mut glyph_row = prepare_desired_root_row (root, y);
  if root_row.is_null ()
    { return; }
  if first
    { produce_box_sides (BOX_DOWN_RIGHT, BOX_DOWN_LEFT, root_row, x, w, root, child); }
  else
    { produce_box_sides (BOX_UP_RIGHT, BOX_UP_LEFT, root_row, x, w, root, child); }
  produce_box_glyphs (BOX_HORIZONTAL, root_row, x, w, child);
  (*root_row).hash = row_hash (root_row);
}

/* Copy to ROOT's desired matrix what we need from CHILD.  */

unsafe fn copy_child_glyphs (root: *mut frame, child: *mut frame)
{
  eassert! (FRAME_PARENT_FRAME (root).is_null ());
  eassert! (frame_ancestor_p (root, child));

  /* Determine the intersection of the child frame rectangle with the
     root frame.  This is basically clipping the child frame to the
     root frame rectangle.  */
  let mut r: rect = rect { x: 0, y: 0, w: 0, h: 0 };
  if !rect_intersect (&mut r, frame_rect_abs (root), frame_rect_abs (child))
    { return; }

  /* Build CHILD's current matrix which we need to copy from it.  */
  make_matrix_current (child);

  /* Draw borders around the child frame.  */
  if FRAME_UNDECORATED (child) == 0
    {
      /* Horizontal line above.  */
      if r.y > 0
	{ produce_box_line (root, child, r.x, r.y - 1, r.w, true); }

      let mut y: c_int = r.y;
      while y < r.y + r.h
	{
	  let root_row: *mut glyph_row = prepare_desired_root_row (root, y);
	  if !root_row.is_null ()
	    { produce_box_sides (BOX_VERTICAL, BOX_VERTICAL, root_row, r.x, r.w,
				 root, child); }
	  y += 1;
	}

      /* Horizontal line below.  */
      if r.y + r.h < (*( (*root).desired_matrix as *mut glyph_matrix )).matrix_h
	{ produce_box_line (root, child, r.x, r.y + r.h, r.w, false); }
    }

  /* First visible row/col, relative to the child frame.  */
  let mut child_x: c_int = 0; let mut child_y: c_int = 0;
  child_xy (child, r.x, r.y, &mut child_x, &mut child_y);

  /* For all rows in the intersection, copy glyphs from the child's
     current matrix to the root's desired matrix, enabling those rows
     if they aren't already.  */
  let mut y: c_int = r.y;
  while y < r.y + r.h
    {
      let root_row: *mut glyph_row = prepare_desired_root_row (root, y);
      if root_row.is_null ()
	{ y += 1; child_y += 1; continue; }

      /* Deal with wide characters unless already done as part of
	 drawing a box around the child frame.  */
      if FRAME_UNDECORATED (child) != 0
	{
	  neutralize_wide_char (root, root_row, r.x - 1);
	  neutralize_wide_char (root, root_row, r.x + r.w);
	}

      /* Copy what's visible from the child's current row.  If that row
	 is not enabled_p, we can't copy anything that makes sense.  */
      let child_row: *mut glyph_row =
	MATRIX_ROW! ((*child).current_matrix as *mut glyph_matrix, child_y);
      if glyph_row_bits! (enabled_p, child_row)
	{ std::ptr::copy_nonoverlapping ((*child_row).glyphs [0].offset (child_x as isize) as *const u8,
					 (*root_row).glyphs [0].offset (r.x as isize) as *mut u8,
					 r.w as usize * core::mem::size_of::<glyph> ()); }

      /* Compute a new hash since we changed glyphs.  */
      (*root_row).hash = row_hash (root_row);
      y += 1; child_y += 1;
    }
}

/* C 3833-3834: `#endif /* !HAVE_ANDROID */` closes C 3550's `#ifndef HAVE_ANDROID`; the
   text above is live on the frozen target.  */

/***********************************************************************
			     Frame Update
 ***********************************************************************/

/* Update the menu bar on X frames that don't have toolkit
   support.  */

unsafe fn update_menu_bar (f: *mut frame)
{
  #[cfg (feature = "window_system_no_ext_menu_bar")]
  {
  if WINDOWP ((*f).menu_bar_window)
    { update_window (XWINDOW ((*f).menu_bar_window)); }
  }
}

#[cfg (feature = "have_window_system")]
unsafe fn update_bar_window (window: Lisp_Object, current: *mut Lisp_Object,
			     desired: *mut Lisp_Object)
{
  if WINDOWP (window)
    {
      let w: *mut window = XWINDOW (window);
      if window_bits! (must_be_updated_p, w)
	{
	  update_window (w);
	  window_bits! (must_be_updated_p, w, false);
	  let tem: Lisp_Object = *current;
	  *current = *desired;
	  *desired = tem;
	}
    }
}

/* Update the tab-bar window of frame F, if present.  */

unsafe fn update_tab_bar (f: *mut frame)
{
  #[cfg (feature = "have_window_system")]
  {
  update_bar_window ((*f).tab_bar_window, &mut (*f).current_tab_bar_string,
		     &mut (*f).desired_tab_bar_string);
  }
}

unsafe fn update_tool_bar (f: *mut frame)
{
  #[cfg (feature = "window_system_no_ext_tool_bar")]
  {
  update_bar_window ((*f).tool_bar_window, &mut (*f).current_tool_bar_string,
		     &mut (*f).desired_tool_bar_string);
  }
}

unsafe fn update_window_frame (f: *mut frame)
{
  eassert! (FRAME_WINDOW_P (f));
  update_begin (f);
  update_menu_bar (f);
  update_tab_bar (f);
  update_tool_bar (f);
  let root_window: *mut window = XWINDOW ((*f).root_window);
  update_window_tree (root_window);
  update_end (f);
  set_window_update_flags (root_window, false);
}

unsafe fn update_initial_frame (f: *mut frame)
{
  build_frame_matrix (f);
  let root_window: *mut window = XWINDOW ((*f).root_window);
  set_window_update_flags (root_window, false);
}

unsafe fn flush_terminal (f: *mut frame)
{
  if !(*FRAME_TTY! (f)).termscript.is_null ()
    { fflush ((*FRAME_TTY! (f)).termscript); }
  fflush ((*FRAME_TTY! (f)).output);
}

unsafe fn update_tty_frame (f: *mut frame)
{
  build_frame_matrix (f);
}

/* C 3921-3923: `#ifndef HAVE_ANDROID` — live on the frozen target.  */

/* Return the cursor position of the selected window of frame F, in
   absolute coordinates in *X and *Y.  Note that if F is a child frame,
   its cursor may be clipped, i.e. outside of the bounds of the terminal
   window.  Value is false if the selected window of F doesn't have
   valid cursor position info.  */

unsafe fn abs_cursor_pos (f: *mut frame, x: *mut c_int, y: *mut c_int) -> bool
{
  let w: *mut window = XWINDOW ((*f).selected_window);
  if (*w).cursor.vpos >= 0
      /* The cursor vpos may be temporarily out of bounds
	 in the following situation:  There is one window,
	 with the cursor in the lower half of it.  The window
	 is split, and a message causes a redisplay before
	 a new cursor position has been computed.  */
      && (*w).cursor.vpos < WINDOW_TOTAL_LINES (w)
    {
      let mut wx: c_int = window_to_frame_hpos (w, (*w).cursor.hpos);
      let wy: c_int = window_to_frame_vpos (w, (*w).cursor.vpos);

      wx += std::cmp::max (0, (*w).left_margin_cols);

      root_xy (f, wx, wy, x, y);
      return true;
    }

  *y = 0;
  *x = *y;
  return false;
}

unsafe fn is_in_matrix (f: *mut frame, x: c_int, y: c_int) -> bool
{
  let root: *mut frame = root_frame (f);
  if x < 0 || x >= (*( (*root).current_matrix as *mut glyph_matrix )).matrix_w || y < 0
      || y >= (*( (*root).current_matrix as *mut glyph_matrix )).matrix_h
    { return false; }
  return true;
}

/* Return the frame of the selected window of frame F.
   Value is NULL if we can't tell.  */

unsafe fn frame_selected_window_frame (f: *mut frame) -> *mut frame
{
  /* Paranoia.  It should not happen that window or frame not valid.  */
  let mut frame: Lisp_Object;
  if WINDOWP ((*f).selected_window)
      && { frame = (*XWINDOW ((*f).selected_window)).frame; FRAMEP (frame) }
    { return XFRAME (frame); }
  return core::ptr::null_mut ();
}

/* Is the terminal cursor of ROOT obscured by a child frame?  */

unsafe fn is_cursor_obscured (root: *mut frame) -> bool
{
  /* Which frame contains the cursor?  If the selected frame is in
     root's z-order, it's the selected frame.  Otherwise fall back to
     the root itself.  */
  let sf: *mut frame = if frame_ancestor_p (root, SELECTED_FRAME ())
			 { SELECTED_FRAME () }
			 else { root };

  /* Give up if we can't tell where the cursor currently is.  */
  let mut x: c_int = 0; let mut y: c_int = 0;
  if !abs_cursor_pos (sf, &mut x, &mut y)
    { return false; }

  /* (x, y) may be outside of the root frame in case the selected frame is a
     child frame which is clipped.  */
  if !is_in_matrix (root, x, y)
    { return true; }

  let cursor_row: *mut glyph_row = MATRIX_ROW! ((*root).current_matrix as *mut glyph_matrix, y);
  let cursor_glyph: *mut glyph = (*cursor_row).glyphs [0].offset (x as isize);
  return (*cursor_glyph).frame != (sf as *mut c_void);
}

/* Decide where to show the cursor, and whether to hide it.

   This works very well for Vertico-Posframe, Transient-Posframe and
   Corfu, but it's debatable if it's the right thing for a general use
   of child frames of all sorts, nested and so on.  But it is also
   debatable if that's a realistic use case from my POV.  */

unsafe fn terminal_cursor_magic (root: *mut frame, topmost_child: *mut frame)
{
  /* By default, prevent the cursor "shining through" child frames.  */
  if is_cursor_obscured (root)
    { tty_hide_cursor (FRAME_TTY! (root)); }

  /* If the terminal cursor is not in the topmost child, the topmost
     child's tty-cursor-if-topmost determines what to do.  If it is
     non-nil, display the cursor in this "non-selected" topmost child
     frame to compensate for the fact that we can't display a
     non-selected cursor like on a window system frame.  */
  let sf: *mut frame = frame_selected_window_frame (root);
  if !sf.is_null () && topmost_child != sf
    {
      let mut frame: Lisp_Object;
      XSETFRAME! (frame, topmost_child);

      let mut x: c_int = 0; let mut y: c_int = 0;
      let cursor: Lisp_Object = Fframe_parameter (frame, Qtty_non_selected_cursor ());
      if !NILP (cursor) && abs_cursor_pos (topmost_child, &mut x, &mut y)
	{
	  if is_in_matrix (root, x, y)
	    {
	      cursor_to (root, y, x);
	      tty_show_cursor (FRAME_TTY! (root));
	    }
	  else
	    { tty_hide_cursor (FRAME_TTY! (root)); }
	}
    }

  /* Hide cursor if selected frame has (cursor-type . nil).  */
  {
    let sf: *mut frame = SELECTED_FRAME ();
    let cursor: Lisp_Object = assq_no_quit (Qcursor_type (), (*sf).param_alist);
    if CONSP (cursor) && NILP (XCDR (cursor))
      { tty_hide_cursor (FRAME_TTY! (root)); }
  }
}

/* C 3921-4123: the real body sits in the `#ifndef HAVE_ANDROID` arm and C 4118-4123's empty
   body is the `#else`; the port wrote the first arm ungated (it is the frozen target's own arm)
   and gated the second with `have_android`, so on Android both were live and the unit read
   `E0428 combine_updates_for_frame`.  The gate below is C's own condition.  */
#[cfg (not (feature = "have_android"))]
#[unsafe(no_mangle)] pub unsafe extern "C" fn combine_updates_for_frame
  (f: *mut frame, inhibit_scrolling: bool)
{
  let root: *mut frame = root_frame (f);

  if !after_make_frame! (root)
    { return; }

  /* Determine visible frames on the root frame, including the root
     frame itself.  Note that there are cases, see bug#75056, where we
     can be called for invisible frames.  This looks like a bug with
     multi-tty, but the old update code didn't check visibility either.  */
  let mut z_order: Lisp_Object = frames_in_reverse_z_order (root, true);
  if NILP (z_order)
    {
      let mut root_frame: Lisp_Object;
      XSETFRAME! (root_frame, root);
      z_order = Fcons (root_frame, Qnil ());
    }

  /* Process child frames in reverse z-order, topmost last.  For each
     child, copy what we need to the root's desired matrix.  */
  let mut topmost_child: *mut frame = core::ptr::null_mut ();
  let mut tail: Lisp_Object = XCDR (z_order);
  while CONSP (tail)
    {
      topmost_child = XFRAME (XCAR (tail));
      if after_make_frame! (topmost_child)
	{ copy_child_glyphs (root, topmost_child); }
      tail = XCDR (tail);
    }

  update_begin (root);
  write_matrix (root, inhibit_scrolling, false);
  make_matrix_current (root);
  update_end (root);

  /* The selected frame determines where the cursor on ttys goes, except
     when it is a frame that is completely unrelated to the frame being
     displayed.  This can happen with multi-tty, when the selected frame
     can be a window-system frame.  */
  if frame_ancestor_p (root, SELECTED_FRAME ())
    { tty_set_cursor (SELECTED_FRAME ()); }
  else
    { tty_set_cursor (root); }

  /* If a child is displayed, and the cursor is displayed in another
     frame, the child might lay above the cursor, so that it appears to
     "shine through" the child.  Avoid that because it's confusing.  */
  if !topmost_child.is_null ()
    { terminal_cursor_magic (root, topmost_child); }
  flush_terminal (root);

  let mut tail: Lisp_Object = z_order;
  while CONSP (tail)
    {
      let f: *mut frame = XFRAME (XCAR (tail));
      let root_window: *mut window = XWINDOW ((*f).root_window);
      set_window_update_flags (root_window, false);
      clear_desired_matrices (f);
      #[cfg (feature = "glyph_debug")]
      {
      check_window_matrix_pointers (root_window);
      add_frame_display_history (f, false);
      }
      tail = XCDR (tail);
    }
}

#[cfg (feature = "have_android")]
#[unsafe(no_mangle)] pub unsafe extern "C" fn combine_updates_for_frame
  (f: *mut frame, inhibit_scrolling: bool)
{
}

/* Update on the screen all root frames ROOTS.  Called from
   redisplay_internal as the last step of redisplaying.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn combine_updates (roots: Lisp_Object)
{
  let mut roots: Lisp_Object = roots;
  while CONSP (roots)
    {
      let root: *mut frame = XFRAME (XCAR (roots));
      if after_make_frame! (root)
	{ combine_updates_for_frame (root, false); }
      roots = XCDR (roots);
    }
}

/* Update frame F based on the data in desired matrices.
   If INHIBIT_SCROLLING, don't try scrolling. */

#[unsafe(no_mangle)] pub unsafe extern "C" fn update_frame (f: *mut frame,
							    inhibit_scrolling: bool)
{
  if FRAME_WINDOW_P (f)
    { update_window_frame (f); }
  else if FRAME_INITIAL_P (f)
    { update_initial_frame (f); }
  else
    { update_tty_frame (f); }
}

/* Update a TTY frame F that has a menu dropped down over some of its
   glyphs.  This is like the second part of update_frame, but it
   doesn't call build_frame_matrix, because we already have the
   desired matrix prepared, and don't want it to be overwritten by the
   text of the normal display.

   ROW and COL, if non-negative, are the row and column of the TTY
   frame where to position the cursor after the frame update is
   complete.  Negative values mean ask update_frame_1 to position the
   cursor "normally", i.e. at point in the selected window.  */
#[unsafe(no_mangle)] pub unsafe extern "C" fn update_frame_with_menu
  (f: *mut frame, row: c_int, col: c_int)
{
  let root_window: *mut window = XWINDOW ((*f).root_window);

  eassert! (FRAME_TERMCAP_P (f));

  /* Update the display.  */
  update_begin (f);
  write_matrix (f, true, true);
  make_matrix_current (f);
  clear_desired_matrices (f);
  /* ROW and COL tell us where in the menu to position the cursor, so
     that screen readers know the active region on the screen.  */
  if row >= 0 && col >= 0
    { cursor_to (f, row, col); }
  else
    { tty_set_cursor (f); }
  update_end (f);
  flush_terminal (f);

  /* Check window matrices for lost pointers.  */
  #[cfg (feature = "glyph_debug")]
  {
  /* C 4203-4210: `#if 0` around `check_window_matrix_pointers (root_window);` — dead in C
     itself; recorded only.  */
  add_frame_display_history (f, false);
  }

  /* Reset flags indicating that a window should be updated.  */
  set_window_update_flags (root_window, false);
}

/* Update the mouse position for a frame F.  This handles both
   updating the display for mouse-face properties and updating the
   help echo text.

   Returns the number of events generated.  */
#[unsafe(no_mangle)] pub unsafe extern "C" fn update_mouse_position
  (f: *mut frame, x: c_int, y: c_int) -> c_int
{
  previous_help_echo_string = help_echo_string;
  help_echo_string = Qnil ();

  note_mouse_highlight (f, x, y);

  /* If the contents of the global variable help_echo_string
     has changed, generate a HELP_EVENT.  */
  if !NILP (help_echo_string)
      || !NILP (previous_help_echo_string)
    {
      let mut frame: Lisp_Object;
      XSETFRAME! (frame, f);

      gen_help_event (help_echo_string, frame, help_echo_window,
                      help_echo_object, help_echo_pos);
      return 1;
    }

  return 0;
}

DEFUN! ("display--update-for-mouse-movement", Fdisplay__update_for_mouse_movement,
        Sdisplay__update_for_mouse_movement, 3, 3, 0,
        doc: "Handle mouse movement detected by Lisp code.\n\nThis function should be called when Lisp code detects the mouse has\nmoved, even if `track-mouse' is nil.  This handles updates that do not\nrely on input events such as updating display for mouse-face\nproperties or updating the help echo text.",
        (mouse_frame: Lisp_Object, mouse_x: Lisp_Object, mouse_y: Lisp_Object),
        {
  CHECK_FRAME! (mouse_frame);
  CHECK_FIXNUM (mouse_x);
  CHECK_FIXNUM (mouse_y);

  update_mouse_position (XFRAME (mouse_frame), XFIXNUM (mouse_x) as c_int,
                         XFIXNUM (mouse_y) as c_int);
  return Qnil ();
});


/************************************************************************
			 Window-based updates
 ************************************************************************/

/* Perform updates in window tree rooted at W.  */

unsafe fn update_window_tree (w: *mut window)
{
  let mut w: *mut window = w;

  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{ update_window_tree (XWINDOW ((*w).contents)); }
      else if window_bits! (must_be_updated_p, w)
	{ update_window (w); }

      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
}


/* Update window W if its flag must_be_updated_p is set.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn update_single_window (w: *mut window)
{
  if window_bits! (must_be_updated_p, w)
    {
      let f: *mut frame = XFRAME (WINDOW_FRAME! (w));

      /* Update W.  */
      update_begin (f);
      update_window (w);
      update_end (f);

      /* Reset flag in W.  */
      window_bits! (must_be_updated_p, w, false);
    }
}

#[cfg (feature = "have_window_system")]
/* Redraw lines from the current matrix of window W that are
   overlapped by other rows.  YB is bottom-most y-position in W.  */

#[cfg (feature = "have_window_system")]
unsafe fn redraw_overlapped_rows (w: *mut window, yb: c_int)
{
  let mut i: c_int;
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));

  /* If rows overlapping others have been changed, the rows being
     overlapped have to be redrawn.  This won't draw lines that have
     already been drawn in update_window_line because overlapped_p in
     desired rows is 0, so after row assignment overlapped_p in
     current rows is 0.  */
  i = 0;
  while i < (*( (*w).current_matrix as *mut glyph_matrix )).nrows
    {
      let row: *mut glyph_row =
	(*( (*w).current_matrix as *mut glyph_matrix )).rows.offset (i as isize);

      if !glyph_row_bits! (enabled_p, row)
	{ break; }
      else if glyph_row_bits! (mode_line_p, row)
	{ i += 1; continue; }

      if glyph_row_bits! (overlapped_p, row)
	{
	  let mut area: glyph_row_area;

	  area = LEFT_MARGIN_AREA;
	  while area < LAST_AREA
	    {
	      output_cursor_to (w, i, 0, (*row).y,
				if area == TEXT_AREA { (*row).x } else { 0 });
	      if (*row).used [area as usize] != 0
		{ ((*((FRAME_RIF! (f)) as *mut redisplay_interface)).write_glyphs.unwrap ()) (w, row, (*row).glyphs [area as usize],
						   area, (*row).used [area as usize] as c_int); }
	      ((*((FRAME_RIF! (f)) as *mut redisplay_interface)).clear_end_of_line.unwrap ()) (w, row, area, -1);
	      area += 1;
	    }

	  glyph_row_bits! (overlapped_p, row, false);
	}

      if MATRIX_ROW_BOTTOM_Y! (row) >= yb
	{ break; }
      i += 1;
    }
}


/* Redraw lines from the current matrix of window W that overlap
   others.  YB is bottom-most y-position in W.  */

#[cfg (feature = "have_window_system")]
unsafe fn redraw_overlapping_rows (w: *mut window, yb: c_int)
{
  let mut i: c_int; let mut bottom_y: c_int;
  let mut row: *mut glyph_row;
  let rif: *mut redisplay_interface = FRAME_RIF! (XFRAME (WINDOW_FRAME! (w))) as *mut redisplay_interface;

  i = 0;
  while i < (*( (*w).current_matrix as *mut glyph_matrix )).nrows
    {
      row = (*( (*w).current_matrix as *mut glyph_matrix )).rows.offset (i as isize);

      if !glyph_row_bits! (enabled_p, row)
	{ break; }
      else if glyph_row_bits! (mode_line_p, row)
	{ i += 1; continue; }

      bottom_y = MATRIX_ROW_BOTTOM_Y! (row);

      if glyph_row_bits! (overlapping_p, row)
	{
	  let mut overlaps: c_int = 0;

	  if MATRIX_ROW_OVERLAPS_PRED_P! (row) && i > 0
	      && !glyph_row_bits! (overlapped_p,
				   MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, i - 1))
	    { overlaps |= OVERLAPS_PRED as c_int; }
	  if MATRIX_ROW_OVERLAPS_SUCC_P! (row) && bottom_y < yb
	      && !glyph_row_bits! (overlapped_p,
				   MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, i + 1))
	    { overlaps |= OVERLAPS_SUCC as c_int; }

	  if overlaps != 0
	    {
	      if (*row).used [LEFT_MARGIN_AREA as usize] != 0
		{ ((*rif).fix_overlapping_area.unwrap ()) (w, row, LEFT_MARGIN_AREA, overlaps); }

	      if (*row).used [TEXT_AREA as usize] != 0
		{ ((*rif).fix_overlapping_area.unwrap ()) (w, row, TEXT_AREA, overlaps); }

	      if (*row).used [RIGHT_MARGIN_AREA as usize] != 0
		{ ((*rif).fix_overlapping_area.unwrap ()) (w, row, RIGHT_MARGIN_AREA, overlaps); }

	      /* Record in neighbor rows that ROW overwrites part of
		 their display.  */
	      if overlaps & OVERLAPS_PRED as c_int != 0
		{ glyph_row_bits! (overlapped_p,
				   MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, i - 1),
				   true); }
	      if overlaps & OVERLAPS_SUCC as c_int != 0
		{ glyph_row_bits! (overlapped_p,
				   MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, i + 1),
				   true); }
	    }
	}

      if bottom_y >= yb
	{ break; }
      i += 1;
    }
}

/* C 3110-3170: `#ifdef GLYPH_DEBUG` returns — `check_window_matrix_pointers` and
   `check_matrix_pointers`.  Dead on the frozen target; carried as text (P-51).  */

/// C: dispextern.h 4012: `bool is_tty_frame (struct frame *f);` is defined by this unit
/// (below), and `frame.h 1480`'s `is_tty_frame` call here reads it through the module.

#[cfg (feature = "glyph_debug")]
unsafe fn check_window_matrix_pointers (w: *mut window)
{
  let f: *mut frame = XFRAME ((*w).frame);
  let mut w: *mut window = w;

  eassert! (is_tty_frame (f));

  if after_make_frame! (f)
    {
      while !w.is_null ()
	{
	  if WINDOWP ((*w).contents)
	    { check_window_matrix_pointers (XWINDOW ((*w).contents)); }
	  else
	    {
	      check_matrix_pointers ((*w).desired_matrix as *mut glyph_matrix,
				     (*f).desired_matrix as *mut glyph_matrix);
	      check_matrix_pointers ((*w).current_matrix as *mut glyph_matrix,
				     (*f).current_matrix as *mut glyph_matrix);
	    }

	  w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
	}
    }
}


/* Check that window rows are slices of frame rows.  WINDOW_MATRIX is
   a window and FRAME_MATRIX is the corresponding frame matrix.  For
   each row in WINDOW_MATRIX check that it's a slice of the
   corresponding frame row.  If it isn't, abort.  */

#[cfg (feature = "glyph_debug")]
unsafe fn check_matrix_pointers (window_matrix: *mut glyph_matrix,
				 frame_matrix: *mut glyph_matrix)
{
  /* Row number in WINDOW_MATRIX.  */
  let mut i: c_int = 0;

  /* Row number corresponding to I in FRAME_MATRIX.  */
  let mut j: c_int = (*window_matrix).matrix_y;

  /* For all rows check that the row in the window matrix is a
     slice of the row in the frame matrix.  If it isn't we didn't
     mirror an operation on the frame matrix correctly.  */
  while i < (*window_matrix).nrows
    {
      if !glyph_row_slice_p ((*window_matrix).rows.offset (i as isize),
			     (*frame_matrix).rows.offset (j as isize))
        { emacs_abort (); }
      i += 1; j += 1;
    }
}

/* C 4396-4417: `#if defined GLYPH_DEBUG && 0` around `check_current_matrix_flags`.  C's own
   `&& 0` makes it dead there; the extractor does not fold the `&& 0` and still lists the
   function, so the text is carried under the feature it names (P-51).  */

#[cfg (feature = "glyph_debug")]
unsafe fn check_current_matrix_flags (w: *mut window)
{
  let mut last_seen_p: bool = false;
  let mut i: c_int; let yb: c_int = window_text_bottom_y (w);

  i = 0;
  while i < (*( (*w).current_matrix as *mut glyph_matrix )).nrows - 1
    {
      let row: *mut glyph_row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, i);
      if !last_seen_p && MATRIX_ROW_BOTTOM_Y! (row) >= yb
	{ last_seen_p = true; }
      else if last_seen_p && glyph_row_bits! (enabled_p, row)
	{ emacs_abort (); }
      i += 1;
    }
}

/* Update display of window W.  */

unsafe fn update_window (w: *mut window)
{
  let desired_matrix: *mut glyph_matrix = (*w).desired_matrix as *mut glyph_matrix;
  #[cfg (feature = "have_window_system")]
  let rif: *mut redisplay_interface = FRAME_RIF! (XFRAME (WINDOW_FRAME! (w))) as *mut redisplay_interface;
  #[cfg (feature = "glyph_debug")]
  /* Check that W's frame doesn't have glyph matrices.  */
  eassert! (FRAME_WINDOW_P (XFRAME (WINDOW_FRAME! (w))));

  /* If forced to complete the update, no input is pending, or we are
     tracking the mouse, do the update.  */
  let mut row: *mut glyph_row; let mut end: *mut glyph_row;
  let mut mode_line_row: *mut glyph_row;
  let mut tab_line_row: *mut glyph_row;
  let mut header_line_row: *mut glyph_row;
  let mut yb: c_int;
  let mut changed_p: bool = false; let mut mouse_face_overwritten_p: bool = false;
  let mut invisible_rows_marked: bool = false;

  #[cfg (feature = "have_window_system")]
  {
  gui_update_window_begin (w);
  }
  #[cfg (not (feature = "have_window_system"))]
  {
  let _ = changed_p;   /* C's `(void) changed_p;` */
  }
  yb = window_text_bottom_y (w);
  row = MATRIX_ROW! (desired_matrix, 0);
  end = MATRIX_MODE_LINE_ROW! (desired_matrix);

  'set_cursor: loop
    {
  /* Take note of the tab line, if there is one.  We will
     update it below, after updating all of the window's lines.  */
  if glyph_row_bits! (mode_line_p, row) && glyph_row_bits! (tab_line_p, row)
    {
      tab_line_row = row;
      row = row.offset (1);
    }
  else
    { tab_line_row = core::ptr::null_mut (); }

  /* Take note of the header line, if there is one.  We will
     update it below, after updating all of the window's lines.  */
  if glyph_row_bits! (mode_line_p, row)
    {
      header_line_row = row;
      row = row.offset (1);
    }
  else
    { header_line_row = core::ptr::null_mut (); }

  /* Update the mode line, if necessary.  */
  mode_line_row = MATRIX_MODE_LINE_ROW! (desired_matrix);
  if glyph_row_bits! (mode_line_p, mode_line_row) && glyph_row_bits! (enabled_p, mode_line_row)
    {
      (*mode_line_row).y = yb + WINDOW_SCROLL_BAR_AREA_HEIGHT! (w);
      update_window_line (w, MATRIX_ROW_VPOS! (mode_line_row, desired_matrix)
			  as c_int,
			  &mut mouse_face_overwritten_p);
    }

  /* Find first enabled row.  Optimizations in redisplay_internal
     may lead to an update with only one row enabled.  There may
     be also completely empty matrices.  */
  while row < end && !glyph_row_bits! (enabled_p, row)
    { row = row.offset (1); }

  /* Try reusing part of the display by copying.  */
  if row < end && !glyph_matrix_bits! (no_scrolling_p, desired_matrix)
    {
      let rc: c_int = scrolling_window (w,
					(if !tab_line_row.is_null () { 1 } else { 0 })
					+ (if !header_line_row.is_null () { 1 } else { 0 }));
      if rc < 0
	{
	  /* All rows were found to be equal.  */
	  break 'set_cursor;
	}
      else if rc > 0
	{
	  /* We've scrolled the display.  */
	  changed_p = true;
	}
    }

  /* Update the rest of the lines.  */
  while row < end
    {
    /* scrolling_window resets the enabled_p flag of the rows it
       reuses from current_matrix.  */
    if glyph_row_bits! (enabled_p, row)
      {
	let vpos: c_int = MATRIX_ROW_VPOS! (row, desired_matrix) as c_int;
	let mut i: c_int;

	changed_p = update_window_line (w, vpos, &mut mouse_face_overwritten_p) || changed_p;

	/* Mark all rows below the last visible one in the current
	   matrix as invalid.  This is necessary because of
	   variable line heights.  Consider the case of three
	   successive redisplays, where the first displays 5
	   lines, the second 3 lines, and the third 5 lines again.
	   If the second redisplay wouldn't mark rows in the
	   current matrix invalid, the third redisplay might be
	   tempted to optimize redisplay based on lines displayed
	   in the first redisplay.  */
	if MATRIX_ROW_BOTTOM_Y! (row) >= yb
	  {
	    i = vpos + 1;
	    while i < (*( (*w).current_matrix as *mut glyph_matrix )).nrows - 1
	      {
		SET_MATRIX_ROW_ENABLED_P! ((*w).current_matrix as *mut glyph_matrix, i, false);
		i += 1;
	      }
	    invisible_rows_marked = true;
	  }
      }
    row = row.offset (1);
    }

  /* If the window doesn't display its mode line, make sure the
     corresponding row of the current glyph matrix is disabled, so
     that if and when the mode line is displayed again, it will be
     cleared and completely redrawn.  */
  if !window_wants_mode_line (w)
    { SET_MATRIX_ROW_ENABLED_P! ((*w).current_matrix as *mut glyph_matrix,
				 (*( (*w).current_matrix as *mut glyph_matrix )).nrows - 1,
				 false); }

  if !invisible_rows_marked
    {
      /* If we didn't mark the invisible rows in the current
	 matrix as invalid above, do that now.  This can happen if
	 scrolling_window updates the last visible rows of the
	 current matrix, in which case the above loop doesn't get
	 to examine the last visible row.  */
      let mut i: c_int;
      i = 0;
      while i < (*( (*w).current_matrix as *mut glyph_matrix )).nrows - 1
	{
	  let current_row: *mut glyph_row =
	    MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, i);
	  if glyph_row_bits! (enabled_p, current_row)
	      && MATRIX_ROW_BOTTOM_Y! (current_row) >= yb
	    {
	      i += 1;
	      while i < (*( (*w).current_matrix as *mut glyph_matrix )).nrows - 1
		{
		  SET_MATRIX_ROW_ENABLED_P! ((*w).current_matrix as *mut glyph_matrix, i,
					     false);
		  i += 1;
		}
	    }
	  i += 1;
	}
    }

      /* set_cursor: */
      break 'set_cursor;
    }

  /* Update the tab line after scrolling because a new tab
     line would otherwise overwrite lines at the top of the window
     that can be scrolled.  */
  if !tab_line_row.is_null () && glyph_row_bits! (enabled_p, tab_line_row)
    {
      (*tab_line_row).y = 0;
      update_window_line (w, 0, &mut mouse_face_overwritten_p);
    }

  /* Update the header line after scrolling because a new header
     line would otherwise overwrite lines at the top of the window
     that can be scrolled.  */
  if !header_line_row.is_null () && glyph_row_bits! (enabled_p, header_line_row)
    {
      (*header_line_row).y = if !tab_line_row.is_null ()
	{ CURRENT_TAB_LINE_HEIGHT! (w) } else { 0 };
      update_window_line (w, if !tab_line_row.is_null () { 1 } else { 0 },
			  &mut mouse_face_overwritten_p);
    }

  /* Fix the appearance of overlapping/overlapped rows.  */
  if !window_bits! (pseudo_window_p, w)
    {
      #[cfg (feature = "have_window_system")]
      {
      if changed_p && (*rif).fix_overlapping_area.is_some ()
	{
	  redraw_overlapped_rows (w, yb);
	  redraw_overlapping_rows (w, yb);
	}
      }

      /* Make cursor visible at cursor position of W.  */
      set_window_cursor_after_update (w);

      /* C 4626-4629: `#if 0` around `IF_DEBUG (check_matrix_invariants (w));` — dead in C
	 itself; recorded only.  */
    }

  #[cfg (feature = "glyph_debug")]
  {
  /* Remember the redisplay method used to display the matrix.  */
  strcpy ((*( (*w).current_matrix as *mut glyph_matrix )).method.as_mut_ptr (),
	  (*( (*w).desired_matrix as *mut glyph_matrix )).method.as_ptr ());
  }

  #[cfg (feature = "have_window_system")]
  {
  update_window_fringes (w, false);

  /* End the update of window W.  Don't set the cursor if we
     paused updating the display because in this case,
     set_window_cursor_after_update hasn't been called, and
     W->output_cursor doesn't contain the cursor location.  */
  gui_update_window_end (w, true, mouse_face_overwritten_p);
  }
  /* If the update wasn't interrupted, this window has been
     completely updated.  */
  window_bits! (must_be_updated_p, w, false);

  #[cfg (feature = "glyph_debug")]
  {
  /* check_current_matrix_flags (w); */
  add_window_display_history (w, (*( (*w).current_matrix as *mut glyph_matrix )).method
			      .as_ptr ());
  }

  xwidget_end_redisplay! (w, (*w).current_matrix);

  clear_glyph_matrix (desired_matrix);
}

/* C 4633-4711: `#ifdef HAVE_WINDOW_SYSTEM` — `gui_update_window_begin` and
   `gui_update_window_end`.  Dead on the frozen target; carried as text (P-51).  */

#[cfg (feature = "have_window_system")]
/* Start update of window W.  */

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)] pub unsafe extern "C" fn gui_update_window_begin (w: *mut window)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let hlinfo: *mut Mouse_HLInfo = MOUSE_HL_INFO! (f);

  block_input ();

  if (*((FRAME_RIF! (f)) as *mut redisplay_interface)).update_window_begin_hook.is_some ()
    { ((*((FRAME_RIF! (f)) as *mut redisplay_interface)).update_window_begin_hook.unwrap ()) (w); }

  (*w).output_cursor = (*w).cursor;

  if f as *mut c_void == (*hlinfo).mouse_face_mouse_frame
    {
      /* Don't do highlighting for mouse motion during the update.  */
      set_mouse_face_defer (hlinfo, true);   // C 3020's bit, through the mirror's setter

      /* If the frame needs to be redrawn, simply forget about any
	 prior mouse highlighting.  */
      if FRAME_GARBAGED_P (f)
	{ (*hlinfo).mouse_face_window = Qnil (); }
    }

  unblock_input ();
}

/* End update of window W.

   Draw vertical borders between horizontally adjacent windows, and
   display W's cursor if CURSOR_ON_P is non-zero.

   MOUSE_FACE_OVERWRITTEN_P non-zero means that some row containing
   glyphs in mouse-face were overwritten.  In that case we have to
   make sure that the mouse-highlight is properly redrawn.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)] pub unsafe extern "C" fn gui_update_window_end
  (w: *mut window, cursor_on_p: bool, mouse_face_overwritten_p: bool)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));

  /* Pseudo windows don't have cursors, so don't display them here.  */
  if !window_bits! (pseudo_window_p, w)
    {
      block_input ();

      if cursor_on_p
	{ display_and_set_cursor (w, true,
				  (*w).output_cursor.hpos, (*w).output_cursor.vpos,
				  (*w).output_cursor.x, (*w).output_cursor.y); }

      if draw_window_fringes (w, true)
	{
	  if WINDOW_RIGHT_DIVIDER_WIDTH! (w) != 0
	    { gui_draw_right_divider (w); }
	  else
	    { gui_draw_vertical_border (w); }
	}
      unblock_input ();
    }

  /* If a row with mouse-face was overwritten, arrange for
     frame_up_to_date_hook to redisplay the mouse highlight.  */
  if mouse_face_overwritten_p
    {
      let hlinfo: *mut Mouse_HLInfo = MOUSE_HL_INFO! (f);

      (*hlinfo).mouse_face_beg_row = -1;
      (*hlinfo).mouse_face_beg_col = -1;
      (*hlinfo).mouse_face_end_row = -1;
      (*hlinfo).mouse_face_end_col = -1;
      (*hlinfo).mouse_face_window = Qnil ();
    }

  if (*((FRAME_RIF! (f)) as *mut redisplay_interface)).update_window_end_hook.is_some ()
    { ((*((FRAME_RIF! (f)) as *mut redisplay_interface)).update_window_end_hook.unwrap ()) (w,
							   cursor_on_p,
							   mouse_face_overwritten_p); }
}

/* Update the display of area AREA in window W, row number VPOS.
   AREA can be either LEFT_MARGIN_AREA or RIGHT_MARGIN_AREA.  */

unsafe fn update_marginal_area (w: *mut window, updated_row: *mut glyph_row,
				area: glyph_row_area, vpos: c_int)
{
  let desired_row: *mut glyph_row = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix,
						 vpos);
  let rif: *mut redisplay_interface = FRAME_RIF! (XFRAME (WINDOW_FRAME! (w))) as *mut redisplay_interface;

  /* Set cursor to start of glyphs, write them, and clear to the end
     of the area.  I don't think that something more sophisticated is
     necessary here, since marginal areas will not be the default.  */
  output_cursor_to (w, vpos, 0, (*desired_row).y, 0);
  if (*desired_row).used [area as usize] != 0
    { ((*rif).write_glyphs.unwrap ()) (w, updated_row, (*desired_row).glyphs [area as usize],
				      area, (*desired_row).used [area as usize] as c_int); }
  ((*rif).clear_end_of_line.unwrap ()) (w, updated_row, area, -1);
}


/* Update the display of the text area of row VPOS in window W.
   Value is true if display has changed.  */

unsafe fn update_text_area (w: *mut window, updated_row: *mut glyph_row, vpos: c_int,
			    partial_p: *mut bool) -> bool
{
  let current_row: *mut glyph_row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix,
						 vpos);
  let desired_row: *mut glyph_row = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix,
						 vpos);
  let rif: *mut redisplay_interface = FRAME_RIF! (XFRAME (WINDOW_FRAME! (w))) as *mut redisplay_interface;
  let mut changed_p: bool = false;

  /* If rows are at different X or Y, or rows have different height,
     or the current row is marked invalid, write the entire line.  */
  if !glyph_row_bits! (enabled_p, current_row)
      || (*desired_row).y != (*current_row).y
      || (*desired_row).ascent != (*current_row).ascent
      || (*desired_row).phys_ascent != (*current_row).phys_ascent
      || (*desired_row).phys_height != (*current_row).phys_height
      || (*desired_row).visible_height != (*current_row).visible_height
      || glyph_row_bits! (overlapped_p, current_row)
      /* This next line is necessary for correctly redrawing
	 mouse-face areas after scrolling and other operations.
	 However, it causes excessive flickering when mouse is moved
	 across the mode line.  Luckily, turning it off for the mode
	 line doesn't seem to hurt anything. -- cyd.
         But it is still needed for the header line. -- kfs.
         The header line vpos is 1 if a tab line is enabled.  (18th
         Apr 2022) */
      || (glyph_row_bits! (mouse_face_p, current_row)
	  && !(glyph_row_bits! (mode_line_p, current_row)
	       && (vpos > glyph_matrix_bits! (tab_line_p,
					      (*w).current_matrix as *mut glyph_matrix) as c_int
		   && glyph_matrix_bits! (header_line_p,
					  (*w).current_matrix as *mut glyph_matrix))))
      || (*current_row).x != (*desired_row).x
    {
      output_cursor_to (w, vpos, 0, (*desired_row).y, (*desired_row).x);

      if (*desired_row).used [TEXT_AREA as usize] != 0
	{ ((*rif).write_glyphs.unwrap ()) (w, updated_row, (*desired_row).glyphs [TEXT_AREA as usize],
					   TEXT_AREA, (*desired_row).used [TEXT_AREA as usize] as c_int); }

      /* Clear to end of window.  */
      ((*rif).clear_end_of_line.unwrap ()) (w, updated_row, TEXT_AREA, -1);
      changed_p = true;

      /* This erases the cursor.  We do this here because
         notice_overwritten_cursor cannot easily check this, which
         might indicate that the whole functionality of
         notice_overwritten_cursor would better be implemented here.
         On the other hand, we need notice_overwritten_cursor as long
         as mouse highlighting is done asynchronously outside of
         redisplay.  */
      if vpos == (*w).phys_cursor.vpos
	{ window_bits! (phys_cursor_on_p, w, false); }
    }
  else
    {
      let mut stop: c_int; let mut i: c_int; let mut x: c_int;
      let mut current_glyph: *mut glyph = (*current_row).glyphs [TEXT_AREA as usize];
      let mut desired_glyph: *mut glyph = (*desired_row).glyphs [TEXT_AREA as usize];
      let overlapping_glyphs_p: bool = glyph_row_bits! (contains_overlapping_glyphs_p,
							current_row);
      let mut desired_stop_pos: c_int = (*desired_row).used [TEXT_AREA as usize] as c_int;
      let mut abort_skipping: bool = false;

      /* If the desired row extends its face to the text area end, and
	 unless the current row also does so at the same position,
	 make sure we write at least one glyph, so that the face
	 extension actually takes place.  */
      if MATRIX_ROW_EXTENDS_FACE_P! (desired_row)
	  && (desired_stop_pos < (*current_row).used [TEXT_AREA as usize] as c_int
	      || (desired_stop_pos == (*current_row).used [TEXT_AREA as usize] as c_int
		  && !MATRIX_ROW_EXTENDS_FACE_P! (current_row)))
	{ desired_stop_pos -= 1; }

      stop = std::cmp::min ((*current_row).used [TEXT_AREA as usize] as c_int, desired_stop_pos);
      i = 0;
      x = (*desired_row).x;

      /* Loop over glyphs that current and desired row may have
	 in common.  */
      while i < stop
	{
	  let mut can_skip_p: bool = !abort_skipping;

	  /* Skip over glyphs that both rows have in common.  These
	     don't have to be written.  We can't skip if the last
	     current glyph overlaps the glyph to its right.  For
	     example, consider a current row of `if ' with the `f' in
	     Courier bold so that it overlaps the ` ' to its right.
	     If the desired row is ` ', we would skip over the space
	     after the `if' and there would remain a pixel from the
	     `f' on the screen.  */
	  if overlapping_glyphs_p && i > 0
	    {
	      let glyph: *mut glyph = (*current_row).glyphs [TEXT_AREA as usize]
		.offset (i as isize - 1);
	      let mut left: c_int = 0; let mut right: c_int = 0;

	      ((*rif).get_glyph_overhangs.unwrap ()) (glyph, XFRAME ((*w).frame),
						      &mut left, &mut right);
	      can_skip_p = (right == 0 && !abort_skipping);
	    }

	  if can_skip_p
	    {
	      let start_hpos: c_int = i;

	      while i < stop
		     && GLYPH_EQUAL_P! (desired_glyph, current_glyph)
		{
		  x += (*desired_glyph).pixel_width as c_int;
		  desired_glyph = desired_glyph.offset (1);
		  current_glyph = current_glyph.offset (1);
		  i += 1;

		  /* Say that only a partial update was performed of
		     the current row (i.e. not all the glyphs were
		     drawn).  This is used to preserve the stipple_p
		     flag of the current row inside
		     update_window_line.  */
		  *partial_p = true;
		}

	      /* Consider the case that the current row contains "xxx
		 ppp ggg" in italic Courier font, and the desired row
		 is "xxx ggg".  The character `p' has lbearing, `g'
		 has not.  The loop above will stop in front of the
		 first `p' in the current row.  If we would start
		 writing glyphs there, we wouldn't erase the lbearing
		 of the `p'.  The rest of the lbearing problem is then
		 taken care of by draw_glyphs.  */
	      if overlapping_glyphs_p
		  && i > 0
		  && i < (*current_row).used [TEXT_AREA as usize] as c_int
		  && ((*current_row).used [TEXT_AREA as usize]
		      != (*desired_row).used [TEXT_AREA as usize])
		{
		  let mut left: c_int = 0; let mut right: c_int = 0;

		  ((*rif).get_glyph_overhangs.unwrap ()) (current_glyph,
							  XFRAME ((*w).frame),
							  &mut left, &mut right);
		  while left > 0 && i > 0
		    {
		      i -= 1; desired_glyph = desired_glyph.offset (-1);
		      current_glyph = current_glyph.offset (-1);
		      x -= (*desired_glyph).pixel_width as c_int;
		      left -= (*desired_glyph).pixel_width as c_int;
		    }

		  /* Abort the skipping algorithm if we end up before
		     our starting point, to avoid looping (bug#1070).
		     This can happen when the lbearing is larger than
		     the pixel width.  */
		  abort_skipping = (i < start_hpos);
		}
	    }

	  /* Try to avoid writing the entire rest of the desired row
	     by looking for a resync point.  This mainly prevents
	     mode line flickering in the case the mode line is in
	     fixed-pitch font, which it usually will be.  */
	  if i < (*desired_row).used [TEXT_AREA as usize] as c_int
	    {
	      let start_x: c_int = x; let start_hpos: c_int = i;
	      let start: *mut glyph = desired_glyph;
	      let mut current_x: c_int = x;
	      let mut skip_first_p: bool = !can_skip_p;

	      /* Find the next glyph that's equal again.  */
	      while i < stop
		     && (skip_first_p
			 || !GLYPH_EQUAL_P! (desired_glyph, current_glyph))
		     && x == current_x
		{
		  x += (*desired_glyph).pixel_width as c_int;
		  current_x += (*current_glyph).pixel_width as c_int;
		  desired_glyph = desired_glyph.offset (1);
		  current_glyph = current_glyph.offset (1);
		  i += 1;
		  skip_first_p = false;
		}

	      if i == start_hpos || x != current_x
		{
		  i = start_hpos;
		  x = start_x;
		  desired_glyph = start;
		  break;
		}

	      output_cursor_to (w, vpos, start_hpos, (*desired_row).y, start_x);
	      ((*rif).write_glyphs.unwrap ()) (w, updated_row, start,
					       TEXT_AREA, i - start_hpos);
	      changed_p = true;
	      *partial_p = true;
	    }
	}

      /* This means we will draw from the start, so no partial update
	 is being performed.  */
      if i == 0
	{ *partial_p = false; }

      /* Write the rest.  */
      if i < (*desired_row).used [TEXT_AREA as usize] as c_int
	{
	  output_cursor_to (w, vpos, i, (*desired_row).y, x);
	  ((*rif).write_glyphs.unwrap ()) (w, updated_row, desired_glyph,
					   TEXT_AREA,
					   (*desired_row).used [TEXT_AREA as usize] as c_int - i);
	  changed_p = true;
	}

      /* Maybe clear to end of line.  */
      if MATRIX_ROW_EXTENDS_FACE_P! (desired_row)
	{
	  /* If new row extends to the end of the text area, nothing
	     has to be cleared, if and only if we did a write_glyphs
	     above.  This is made sure by setting desired_stop_pos
	     appropriately above.  */
	  eassert! (i < (*desired_row).used [TEXT_AREA as usize] as c_int
		    || (((*desired_row).used [TEXT_AREA as usize]
			 == (*current_row).used [TEXT_AREA as usize])
			&& MATRIX_ROW_EXTENDS_FACE_P! (current_row)));
	}
      else if MATRIX_ROW_EXTENDS_FACE_P! (current_row)
	{
	  /* If old row extends to the end of the text area, clear.  */
	  if i >= (*desired_row).used [TEXT_AREA as usize] as c_int
	    { output_cursor_to (w, vpos, i, (*desired_row).y,
				(*desired_row).pixel_width); }
	  ((*rif).clear_end_of_line.unwrap ()) (w, updated_row, TEXT_AREA, -1);
	  changed_p = true;
	}
      else if (*desired_row).pixel_width < (*current_row).pixel_width
	{
	  /* Otherwise clear to the end of the old row.  Everything
	     after that position should be clear already.  */
	  let mut xlim: c_int;

	  if i >= (*desired_row).used [TEXT_AREA as usize] as c_int
	    { output_cursor_to (w, vpos, i, (*desired_row).y,
				(*desired_row).pixel_width); }

	  /* If cursor is displayed at the end of the line, make sure
	     it's cleared.  Nowadays we don't have a phys_cursor_glyph
	     with which to erase the cursor (because this method
	     doesn't work with lbearing/rbearing), so we must do it
	     this way.  */
	  if vpos == (*w).phys_cursor.vpos
	      && (if glyph_row_bits! (reversed_p, desired_row)
		    { (*w).phys_cursor.hpos < 0 }
		    else { (*w).phys_cursor.hpos >= (*desired_row).used [TEXT_AREA as usize] as c_int })
	    {
	      window_bits! (phys_cursor_on_p, w, false);
	      xlim = -1;
	    }
	  else
	    { xlim = (*current_row).pixel_width; }
	  ((*rif).clear_end_of_line.unwrap ()) (w, updated_row, TEXT_AREA, xlim);
	  changed_p = true;
	}
    }

  return changed_p;
}


/* Update row VPOS in window W.  Value is true if display has been changed.  */

unsafe fn update_window_line (w: *mut window, vpos: c_int,
			      mouse_face_overwritten_p: *mut bool) -> bool
{
  let current_row: *mut glyph_row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix,
						 vpos);
  let desired_row: *mut glyph_row = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix,
						 vpos);
  let rif: *mut redisplay_interface = FRAME_RIF! (XFRAME (WINDOW_FRAME! (w))) as *mut redisplay_interface;

  /* partial_p is true if not all of desired_row was drawn.  */
  let mut changed_p: bool = false; let mut partial_p: bool = false;
  let mut was_stipple: bool;

  /* A row can be completely invisible in case a desired matrix was
     built with a vscroll and then make_cursor_line_fully_visible shifts
     the matrix.  Make sure to make such rows current anyway, since
     we need the correct y-position, for example, in the current matrix.  */
  if glyph_row_bits! (mode_line_p, desired_row)
      || (*desired_row).visible_height > 0
    {
      eassert! (glyph_row_bits! (enabled_p, desired_row));

      /* Update display of the left margin area, if there is one.  */
      if !glyph_row_bits! (full_width_p, desired_row) && (*w).left_margin_cols > 0
	{
	  changed_p = true;
	  update_marginal_area (w, desired_row, LEFT_MARGIN_AREA, vpos);
	  /* Setting this flag will ensure the vertical border, if
	     any, between this window and the one on its left will be
	     redrawn.  This is necessary because updating the left
	     margin area can potentially draw over the border.  */
	  glyph_row_bits! (redraw_fringe_bitmaps_p, current_row, true);
	}

      /* Update the display of the text area.  */
      if update_text_area (w, desired_row, vpos, &mut partial_p)
	{
	  changed_p = true;
	  if glyph_row_bits! (mouse_face_p, current_row)
	    { *mouse_face_overwritten_p = true; }
	}

      /* Update display of the right margin area, if there is one.  */
      if !glyph_row_bits! (full_width_p, desired_row) && (*w).right_margin_cols > 0
	{
	  changed_p = true;
	  update_marginal_area (w, desired_row, RIGHT_MARGIN_AREA, vpos);
	}

      /* Draw truncation marks etc.  */
      if !glyph_row_bits! (enabled_p, current_row)
	  || (*desired_row).y != (*current_row).y
	  || (*desired_row).visible_height != (*current_row).visible_height
	  || glyph_row_bits! (cursor_in_fringe_p, desired_row)
	     != glyph_row_bits! (cursor_in_fringe_p, current_row)
	  || (*desired_row).overlay_arrow_bitmap != (*current_row).overlay_arrow_bitmap
	  || glyph_row_bits! (redraw_fringe_bitmaps_p, current_row)
	  || glyph_row_bits! (mode_line_p, desired_row)
	     != glyph_row_bits! (mode_line_p, current_row)
	  || glyph_row_bits! (exact_window_width_line_p, desired_row)
	     != glyph_row_bits! (exact_window_width_line_p, current_row)
	  || (MATRIX_ROW_CONTINUATION_LINE_P! (desired_row)
	      != MATRIX_ROW_CONTINUATION_LINE_P! (current_row))
	{ ((*rif).after_update_window_line_hook.unwrap ()) (w, desired_row); }
    }

  /* Update current_row from desired_row.  */
  was_stipple = glyph_row_bits! (stipple_p, current_row);
  make_current (core::ptr::null_mut (), w, vpos);

  /* If only a partial update was performed, any stipple already
     displayed in MATRIX_ROW (w->current_matrix, vpos) might still be
     there, so don't hurry to clear that flag if it's not in
     desired_row.  */

  if partial_p && was_stipple
    { glyph_row_bits! (stipple_p, current_row, true); }

  return changed_p;
}


/* Set the cursor after an update of window W.  This function may only
   be called from update_window.  */

unsafe fn set_window_cursor_after_update (w: *mut window)
{
  let f: *mut frame = XFRAME ((*w).frame);
  let mut cx: c_int; let mut cy: c_int; let mut vpos: c_int; let mut hpos: c_int;

  /* Not intended for frame matrix updates.  */
  eassert! (FRAME_WINDOW_P (f));

  if cursor_in_echo_area
      && !NILP (echo_area_buffer [0])
      /* If we are showing a message instead of the mini-buffer,
	 show the cursor for the message instead.  */
      && XWINDOW (minibuf_window) == w
      && BASE_EQ (minibuf_window, echo_area_window)
      /* These cases apply only to the frame that contains
	 the active mini-buffer window.  */
      && FRAME_HAS_MINIBUF_P (f)
      && BASE_EQ (FRAME_MINIBUF_WINDOW (f), echo_area_window)
    {
      cx = 0; cy = 0; vpos = 0; hpos = 0;

      /* If the mini-buffer is several lines high, find the last
	 line that has any text on it.  Note: either all lines
	 are enabled or none.  Otherwise we wouldn't be able to
	 determine Y.  */
      let mut last_row: *mut glyph_row = core::ptr::null_mut ();
      let yb: c_int = window_text_bottom_y (w);

      let mut row: *mut glyph_row = (*((*w).current_matrix as *mut glyph_matrix)).rows;
      while glyph_row_bits! (enabled_p, row)
	    && (last_row.is_null () || MATRIX_ROW_BOTTOM_Y! (row) <= yb)
	{
	  if (*row).used [TEXT_AREA as usize] != 0
	      && (*(*row).glyphs [TEXT_AREA as usize]).charpos >= 0
	    { last_row = row; }
	  row = row.offset (1);
	}

      if !last_row.is_null ()
	{
	  let start: *mut glyph = (*last_row).glyphs [TEXT_AREA as usize];
	  let mut last: *mut glyph =
	    start.offset ((*last_row).used [TEXT_AREA as usize] as isize - 1);

	  while last > start && (*last).charpos < 0
	    { last = last.offset (-1); }

	  let mut glyph: *mut glyph = start;
	  while glyph < last
	    {
	      cx += (*glyph).pixel_width as c_int;
	      hpos += 1;
	      glyph = glyph.offset (1);
	    }

	  cy = (*last_row).y;
	  vpos = MATRIX_ROW_VPOS! (last_row, (*w).current_matrix as *mut glyph_matrix) as c_int;
	}
    }
  else
    {
      cx = (*w).cursor.x;
      cy = (*w).cursor.y;
      hpos = (*w).cursor.hpos;
      vpos = (*w).cursor.vpos;
    }

  /* Window cursor can be out of sync for horizontally split windows.
     Horizontal position is -1 when cursor is on the left fringe.   */
  hpos = clip_to_bounds (-1, hpos as isize, (*( (*w).current_matrix as *mut glyph_matrix )).matrix_w as isize - 1) as c_int;
  vpos = clip_to_bounds (0, vpos as isize, (*( (*w).current_matrix as *mut glyph_matrix )).nrows as isize - 1) as c_int;
  output_cursor_to (w, vpos, hpos, cy, cx);
}


/* Set WINDOW->must_be_updated_p to ON_P for all windows in
   the window tree rooted at W.  */

unsafe fn set_window_update_flags (w: *mut window, on_p: bool)
{
  let mut w: *mut window = w;

  while !w.is_null ()
    {
      if WINDOWP ((*w).contents)
	{ set_window_update_flags (XWINDOW ((*w).contents), on_p); }
      else
	{ window_bits! (must_be_updated_p, w, on_p); }

      w = if NILP ((*w).next) { core::ptr::null_mut () } else { XWINDOW ((*w).next) };
    }
}


/***********************************************************************
			Window-Based Scrolling
 ***********************************************************************/

/* Structure describing rows in scrolling_window.  */

#[repr(C)]
pub struct row_entry
{
  /* Number of occurrences of this row in desired and current matrix.  */
  pub old_uses: c_int,
  pub new_uses: c_int,

  /* Vpos of row in new matrix.  */
  pub new_line_number: c_int,

  /* Bucket index of this row_entry in the hash table row_table.  */
  pub bucket: ptrdiff_t,

  /* The row described by this entry.  */
  pub row: *mut glyph_row,

  /* Hash collision chain.  */
  pub next: *mut row_entry,
}

const _: () = core::assert! (core::mem::size_of::<row_entry> () == 40);      // clang-measured
const _: () = core::assert! (core::mem::offset_of! (row_entry, old_uses) == 0);
const _: () = core::assert! (core::mem::offset_of! (row_entry, new_uses) == 4);
const _: () = core::assert! (core::mem::offset_of! (row_entry, new_line_number) == 8);
const _: () = core::assert! (core::mem::offset_of! (row_entry, bucket) == 16);
const _: () = core::assert! (core::mem::offset_of! (row_entry, row) == 24);
const _: () = core::assert! (core::mem::offset_of! (row_entry, next) == 32);

/* A pool to allocate row_entry structures from, and the size of the
   pool.  The pool is reallocated in scrolling_window when we find
   that we need a larger one.  */

static mut row_entry_pool: *mut row_entry = core::ptr::null_mut ();
static mut row_entry_pool_size: ptrdiff_t = 0;

/* Index of next free entry in row_entry_pool.  */

static mut row_entry_idx: ptrdiff_t = 0;

/* The hash table used during scrolling, and the table's size.  This
   table is used to quickly identify equal rows in the desired and
   current matrix.  */

static mut row_table: *mut *mut row_entry = core::ptr::null_mut ();
static mut row_table_size: ptrdiff_t = 0;

/* Vectors of pointers to row_entry structures belonging to the
   current and desired matrix, and the size of the vectors.  */

static mut old_lines: *mut *mut row_entry = core::ptr::null_mut ();
static mut new_lines: *mut *mut row_entry = core::ptr::null_mut ();
static mut old_lines_size: ptrdiff_t = 0;
static mut new_lines_size: ptrdiff_t = 0;

/* A pool to allocate run structures from, and its size.  */

static mut run_pool: *mut run = core::ptr::null_mut ();
static mut runs_size: ptrdiff_t = 0;

/* A vector of runs of lines found during scrolling.  */

static mut runs: *mut *mut run = core::ptr::null_mut ();

/* Add glyph row ROW to the scrolling hash table.  */

unsafe fn add_row_entry (row: *mut glyph_row) -> *mut row_entry
{
  let mut entry: *mut row_entry;
  let i: ptrdiff_t = (*row).hash as ptrdiff_t % row_table_size;

  entry = *row_table.offset (i as isize);
  eassert! (!entry.is_null () || verify_row_hash (row));
  while !entry.is_null () && !row_equal_p ((*entry).row, row, true)
    { entry = (*entry).next; }

  if entry.is_null ()
    {
      row_entry_idx += 1;
      entry = row_entry_pool.offset (row_entry_idx as isize - 1);
      (*entry).row = row;
      (*entry).new_uses = 0;
      (*entry).old_uses = (*entry).new_uses;
      (*entry).new_line_number = 0;
      (*entry).bucket = i;
      (*entry).next = *row_table.offset (i as isize);
      *row_table.offset (i as isize) = entry;
    }

  return entry;
}

/* Try to reuse part of the current display of W by scrolling lines.
   HEADER_LINE_P means W has a header line.

   The algorithm is taken from Communications of the ACM, Apr78 "A
   Technique for Isolating Differences Between Files."  It should take
   O(N) time.

   A short outline of the steps of the algorithm

   1. Skip lines equal at the start and end of both matrices.

   2. Enter rows in the current and desired matrix into a symbol
   table, counting how often they appear in both matrices.

   3. Rows that appear exactly once in both matrices serve as anchors,
   i.e. we assume that such lines are likely to have been moved.

   4. Starting from anchor lines, extend regions to be scrolled both
   forward and backward.

   Value is

   -1	if all rows were found to be equal.
   0	to indicate that we did not scroll the display, or
   1	if we did scroll.  */

unsafe fn scrolling_window (w: *mut window, tab_line_p: c_int) -> c_int
{
  let desired_matrix: *mut glyph_matrix = (*w).desired_matrix as *mut glyph_matrix;
  let current_matrix: *mut glyph_matrix = (*w).current_matrix as *mut glyph_matrix;
  let yb: c_int = window_text_bottom_y (w);
  let mut i: ptrdiff_t;
  let mut j: c_int; let mut first_old: c_int; let mut first_new: c_int;
  let mut last_old: c_int; let mut last_new: c_int;
  let mut nruns: c_int; let mut run_idx: c_int;
  let mut n: ptrdiff_t;
  let mut entry: *mut row_entry;
  let rif: *mut redisplay_interface = FRAME_RIF! (XFRAME (WINDOW_FRAME! (w))) as *mut redisplay_interface;

  /* Skip over rows equal at the start.  */
  i = tab_line_p as ptrdiff_t;
  while i < (*current_matrix).nrows as ptrdiff_t - 1
    {
      let d: *mut glyph_row = MATRIX_ROW! (desired_matrix, i);
      let c: *mut glyph_row = MATRIX_ROW! (current_matrix, i);

      /* If there is a row with a stipple currently on the glass, give
	 up.  Stipples look different depending on where on the
	 display they are drawn, so scrolling the display will produce
	 incorrect results.  */

      if glyph_row_bits! (stipple_p, c)
	{ return 0; }

      if glyph_row_bits! (enabled_p, c)
	  && glyph_row_bits! (enabled_p, d)
	  && !glyph_row_bits! (redraw_fringe_bitmaps_p, d)
	  && (*c).y == (*d).y
	  && MATRIX_ROW_BOTTOM_Y! (c) <= yb
	  && MATRIX_ROW_BOTTOM_Y! (d) <= yb
	  && row_equal_p (c, d, true)
	{
	  assign_row (c, d);
	  glyph_row_bits! (enabled_p, d, false);
	}
      else
	{ break; }
      i += 1;
    }

  /* Can't scroll the display of w32 GUI frames when position of point
     is indicated by the system caret, because scrolling the display
     will then "copy" the pixels used by the caret.  */
  #[cfg (feature = "have_ntgui")]
  {
  if w32_use_visible_system_caret
    { return 0; }
  }

  /* Give up if some rows in the desired matrix are not enabled.  */
  if !MATRIX_ROW_ENABLED_P! (desired_matrix, i as c_int)
    { return -1; }

  first_old = i as c_int; first_new = first_old;

  while i < (*current_matrix).nrows as ptrdiff_t - 1
    {
      /* If there is a stipple after the first change, give up as
	 well.  */
      if glyph_row_bits! (stipple_p, MATRIX_ROW! (current_matrix, i))
	{ return 0; }

      i += 1;
    }

  /* Set last_new to the index + 1 of the row that reaches the
     bottom boundary in the desired matrix.  Give up if we find a
     disabled row before we reach the bottom boundary.  */
  i = first_new as ptrdiff_t + 1;
  while i < (*desired_matrix).nrows as ptrdiff_t - 1
    {
      let mut bottom: c_int;

      if !MATRIX_ROW_ENABLED_P! (desired_matrix, i as c_int)
	{ return 0; }
      bottom = MATRIX_ROW_BOTTOM_Y! (MATRIX_ROW! (desired_matrix, i as c_int));
      if bottom <= yb
	{ i += 1; }
      if bottom >= yb
	{ break; }
    }

  last_new = i as c_int;

  /* Set last_old to the index + 1 of the row that reaches the bottom
     boundary in the current matrix.  We don't look at the enabled
     flag here because we plan to reuse part of the display even if
     other parts are disabled.  */
  i = first_old as ptrdiff_t + 1;
  while i < (*current_matrix).nrows as ptrdiff_t - 1
    {
      let bottom: c_int = MATRIX_ROW_BOTTOM_Y! (MATRIX_ROW! (current_matrix, i as c_int));
      if bottom <= yb
	{ i += 1; }
      if bottom >= yb
	{ break; }
    }

  last_old = i as c_int;

  /* Skip over rows equal at the bottom.  */
  i = last_new as ptrdiff_t;
  j = last_old;
  while i - 1 > first_new as ptrdiff_t
         && j as ptrdiff_t - 1 > first_old as ptrdiff_t
         && MATRIX_ROW_ENABLED_P! (current_matrix, j - 1)
	 && ((*MATRIX_ROW! (current_matrix, j - 1)).y
	     == (*MATRIX_ROW! (desired_matrix, (i - 1) as c_int)).y)
	 && !glyph_row_bits! (redraw_fringe_bitmaps_p,
			      MATRIX_ROW! (desired_matrix, (i - 1) as c_int))
         && row_equal_p (MATRIX_ROW! (desired_matrix, (i - 1) as c_int),
                         MATRIX_ROW! (current_matrix, j - 1), true)
    { i -= 1; j -= 1; }
  last_new = i as c_int;
  last_old = j;

  /* Nothing to do if all rows are equal.  */
  if last_new == first_new
    { return 0; }

  /* Check for integer overflow in size calculation.

     If next_almost_prime checks (N) for divisibility by 2..10, then
     it can return at most N + 10, e.g., next_almost_prime (1) == 11.
     So, set next_almost_prime_increment_max to 10.

     It's just a coincidence that next_almost_prime_increment_max ==
     NEXT_ALMOST_PRIME_LIMIT - 1.  If NEXT_ALMOST_PRIME_LIMIT were
     13, then next_almost_prime_increment_max would be 14, e.g.,
     because next_almost_prime (113) would be 127.  */
  {
    const _: () = core::assert! (NEXT_ALMOST_PRIME_LIMIT == 11);
    let next_almost_prime_increment_max: ptrdiff_t = 10;
    let row_table_max: ptrdiff_t =
      (std::cmp::min (PTRDIFF_MAX as usize, SIZE_MAX)  /* P-127 */
       / (3 * core::mem::size_of::<*mut row_entry> () as usize)
       - next_almost_prime_increment_max as usize) as ptrdiff_t;
    let current_nrows_max: ptrdiff_t = row_table_max - (*desired_matrix).nrows as ptrdiff_t;
    if current_nrows_max < (*current_matrix).nrows as ptrdiff_t
      { memory_full_up (); }
  }

  /* Reallocate vectors, tables etc. if necessary.  */

  if (*current_matrix).nrows as ptrdiff_t > old_lines_size
    { old_lines = xpalloc (old_lines as *mut c_void,
			   core::ptr::addr_of_mut! (old_lines_size),
			   (*current_matrix).nrows as ptrdiff_t - old_lines_size,
			   INT_MAX as ptrdiff_t,
			   core::mem::size_of::<*mut row_entry> () as ptrdiff_t)
	as *mut *mut row_entry; }

  if (*desired_matrix).nrows as ptrdiff_t > new_lines_size
    { new_lines = xpalloc (new_lines as *mut c_void,
			   core::ptr::addr_of_mut! (new_lines_size),
			   (*desired_matrix).nrows as ptrdiff_t - new_lines_size,
			   INT_MAX as ptrdiff_t,
			   core::mem::size_of::<*mut row_entry> () as ptrdiff_t)
	as *mut *mut row_entry; }

  n = (*desired_matrix).nrows as ptrdiff_t;
  n += (*current_matrix).nrows as ptrdiff_t;
  if row_table_size < 3 * n
    {
      let size: ptrdiff_t = next_almost_prime (3 * n);
      row_table = xnrealloc (row_table as *mut c_void, size,
			     core::mem::size_of::<*mut row_entry> () as ptrdiff_t)
	as *mut *mut row_entry;
      row_table_size = size;
      /* C 5436: `memset (row_table, 0, size * sizeof *row_table)` - bytes, through a `u8`
	 pointer (P-114).  */
      std::ptr::write_bytes (row_table as *mut u8, 0,
			     size as usize * core::mem::size_of::<*mut row_entry> ());
    }

  if n > row_entry_pool_size
    { row_entry_pool = xpalloc (row_entry_pool as *mut c_void,
				core::ptr::addr_of_mut! (row_entry_pool_size),
				n - row_entry_pool_size,
				-1, core::mem::size_of::<row_entry> () as ptrdiff_t)
	as *mut row_entry; }

  if (*desired_matrix).nrows as ptrdiff_t > runs_size
    {
      runs = xnrealloc (runs as *mut c_void, (*desired_matrix).nrows as ptrdiff_t,
			core::mem::size_of::<*mut run> () as ptrdiff_t) as *mut *mut run;
      run_pool = xnrealloc (run_pool as *mut c_void, (*desired_matrix).nrows as ptrdiff_t,
			    core::mem::size_of::<run> () as ptrdiff_t) as *mut run;
      runs_size = (*desired_matrix).nrows as ptrdiff_t;
    }

  nruns = 0; run_idx = 0;
  row_entry_idx = 0;

  /* Add rows from the current and desired matrix to the hash table
     row_hash_table to be able to find equal ones quickly.  */

  i = first_old as ptrdiff_t;
  while i < last_old as ptrdiff_t
    {
      if MATRIX_ROW_ENABLED_P! (current_matrix, i as c_int)
	{
	  entry = add_row_entry (MATRIX_ROW! (current_matrix, i as c_int));
	  *old_lines.offset (i as isize) = entry;
	  (*entry).old_uses += 1;
	}
      else
	{ *old_lines.offset (i as isize) = core::ptr::null_mut (); }
      i += 1;
    }

  i = first_new as ptrdiff_t;
  while i < last_new as ptrdiff_t
    {
      eassert! (MATRIX_ROW_ENABLED_P! (desired_matrix, i as c_int));
      entry = add_row_entry (MATRIX_ROW! (desired_matrix, i as c_int));
      (*entry).new_uses += 1;
      (*entry).new_line_number = i as c_int;
      *new_lines.offset (i as isize) = entry;
      i += 1;
    }

  /* Identify moves based on lines that are unique and equal
     in both matrices.  */
  i = first_old as ptrdiff_t;
  while i < last_old as ptrdiff_t
    {
    if !(*old_lines.offset (i as isize)).is_null ()
	&& (*(*old_lines.offset (i as isize))).old_uses == 1
        && (*(*old_lines.offset (i as isize))).new_uses == 1
      {
	let mut p: c_int; let mut q: c_int;
	let new_line: c_int = (*(*old_lines.offset (i as isize))).new_line_number;
	run_idx += 1;
	let run: *mut run = run_pool.offset (run_idx as isize - 1);

	/* Record move.  */
	(*run).current_vpos = i as c_int;
	(*run).current_y = (*MATRIX_ROW! (current_matrix, i as c_int)).y;
	(*run).desired_vpos = new_line;
	(*run).desired_y = (*MATRIX_ROW! (desired_matrix, new_line)).y;
	(*run).nrows = 1;
	(*run).height = (*MATRIX_ROW! (current_matrix, i as c_int)).height;

	/* Extend backward.  */
	p = i as c_int - 1;
	q = new_line - 1;
	while p > first_old
	       && q > first_new
	       && *old_lines.offset (p as isize) == *new_lines.offset (q as isize)
	  {
	    let h: c_int = (*MATRIX_ROW! (current_matrix, p)).height;
	    (*run).current_vpos -= 1;
	    (*run).desired_vpos -= 1;
	    (*run).nrows += 1;
	    (*run).height += h;
	    (*run).desired_y -= h;
	    (*run).current_y -= h;
	    p -= 1; q -= 1;
	  }

	/* Extend forward.  */
	p = i as c_int + 1;
	q = new_line + 1;
	while p < last_old
	       && q < last_new
	       && *old_lines.offset (p as isize) == *new_lines.offset (q as isize)
	  {
	    let h: c_int = (*MATRIX_ROW! (current_matrix, p)).height;
	    (*run).nrows += 1;
	    (*run).height += h;
	    p += 1; q += 1;
	  }

	/* Insert run into list of all runs.  Order runs by copied
	   pixel lines.  Note that we record runs that don't have to
	   be copied because they are already in place.  This is done
	   because we can avoid calling update_window_line in this
	   case.  */
	p = 0;
	while p < nruns && (*(*runs.offset (p as isize))).height > (*run).height
	  { p += 1; }
	q = nruns;
	while q > p
	  {
	    *runs.offset (q as isize) = *runs.offset (q as isize - 1);
	    q -= 1;
	  }
	*runs.offset (p as isize) = run;
	nruns += 1;

	i += (*run).nrows as ptrdiff_t;
      }
    else
      { i += 1; }
    }

  /* Do the moves.  Do it in a way that we don't overwrite something
     we want to copy later on.  This is not solvable in general
     because there is only one display and we don't have a way to
     exchange areas on this display.  Example:

          +-----------+       +-----------+
          |     A     |       |     B     |
          +-----------+  -->  +-----------+
          |     B     |       |     A     |
          +-----------+       +-----------+

     Instead, prefer bigger moves, and invalidate moves that would
     copy from where we copied to.  */

  i = 0;
  while i < nruns as ptrdiff_t
    {
    if (*(*runs.offset (i as isize))).nrows > 0
      {
	let r: *mut run = *runs.offset (i as isize);

	/* Copy on the display.  */
	if (*r).current_y != (*r).desired_y
	  {
	    ((*rif).clear_window_mouse_face.unwrap ()) (w);
	    ((*rif).scroll_run_hook.unwrap ()) (w, r);
	  }

	/* Truncate runs that copy to where we copied to, and
	   invalidate runs that copy from where we copied to.  */
	j = nruns - 1;
	while j as ptrdiff_t > i
	  {
	    let p: *mut run = *runs.offset (j as isize);
	    let mut truncated_p: bool = false;

	    if (*p).nrows > 0
		&& (*p).desired_y < (*r).desired_y + (*r).height
		&& (*p).desired_y + (*p).height > (*r).desired_y
	      {
		if (*p).desired_y < (*r).desired_y
		  {
		    (*p).nrows = (*r).desired_vpos - (*p).desired_vpos;
		    (*p).height = (*r).desired_y - (*p).desired_y;
		    truncated_p = true;
		  }
		else
		  {
		    let nrows_copied: c_int = ((*r).desired_vpos + (*r).nrows
					       - (*p).desired_vpos);

		    if (*p).nrows <= nrows_copied
		      { (*p).nrows = 0; }
		    else
		      {
			let height_copied: c_int = ((*r).desired_y + (*r).height
						    - (*p).desired_y);

			(*p).current_vpos += nrows_copied;
			(*p).desired_vpos += nrows_copied;
			(*p).nrows -= nrows_copied;
			(*p).current_y += height_copied;
			(*p).desired_y += height_copied;
			(*p).height -= height_copied;
			truncated_p = true;
		      }
		  }
	      }

	    if (*r).current_y != (*r).desired_y
		/* The condition below is equivalent to
		   ((p->current_y >= r->desired_y
		     && p->current_y < r->desired_y + r->height)
		    || (p->current_y + p->height > r->desired_y
			&& (p->current_y + p->height
			    <= r->desired_y + r->height)))
		   because we have 0 < p->height <= r->height.  */
		&& (*p).current_y < (*r).desired_y + (*r).height
		&& (*p).current_y + (*p).height > (*r).desired_y
	      { (*p).nrows = 0; }

	    /* Reorder runs by copied pixel lines if truncated.  */
	    if truncated_p && (*p).nrows > 0
	      {
		let mut k: c_int = nruns - 1;

		while (*(*runs.offset (k as isize))).nrows == 0
		      || (*(*runs.offset (k as isize))).height < (*p).height
		  { k -= 1; }
		memmove (runs.offset (j as isize) as *mut c_void,
			 runs.offset (j as isize + 1) as *const c_void,
			 (k - j) as usize * core::mem::size_of::<*mut run> ());
		*runs.offset (k as isize) = p;
	      }
	    j -= 1;
	  }

	/* Assign matrix rows.  */
	j = 0;
	while j < (*r).nrows
	  {
	    let from: *mut glyph_row;
	    let to: *mut glyph_row;
	    let to_overlapped_p: bool;

	    to = MATRIX_ROW! (current_matrix, (*r).desired_vpos + j);
	    from = MATRIX_ROW! (desired_matrix, (*r).desired_vpos + j);
	    to_overlapped_p = glyph_row_bits! (overlapped_p, to);
	    glyph_row_bits! (redraw_fringe_bitmaps_p, from,
			     glyph_row_bits! (fringe_bitmap_periodic_p, from));
	    assign_row (to, from);
	    /* The above `assign_row' actually does swap, so if we had
	       an overlap in the copy destination of two runs, then
	       the second run would assign a previously disabled bogus
	       row.  But thanks to the truncation code in the
	       preceding for-loop, we no longer have such an overlap,
	       and thus the assigned row should always be enabled.  */
	    eassert! (glyph_row_bits! (enabled_p, to));
	    glyph_row_bits! (enabled_p, from, false);
	    glyph_row_bits! (overlapped_p, to, to_overlapped_p);
	    j += 1;
	  }
      }
    i += 1;
    }

  /* Clear the hash table, for the next time.  */
  i = 0;
  while i < row_entry_idx
    {
      *row_table.offset ((*row_entry_pool.offset (i as isize)).bucket as isize)
	= core::ptr::null_mut ();
      i += 1;
    }

  /* Value is 1 to indicate that we scrolled the display.  */
  return if nruns > 0 { 1 } else { 0 };
}


/************************************************************************
			 Frame-Based Updates
 ************************************************************************/

unsafe fn tty_set_cursor (f: *mut frame)
{
  if (cursor_in_echo_area
       /* If we are showing a message instead of the mini-buffer,
	  show the cursor for the message instead of for the
	  (now hidden) mini-buffer contents.  */
       || (BASE_EQ (minibuf_window, selected_window)
	   && BASE_EQ (minibuf_window, echo_area_window)
	   && !NILP (echo_area_buffer [0])))
      /* These cases apply only to the frame that contains
	 the active mini-buffer window.  */
      && FRAME_HAS_MINIBUF_P (f)
      && BASE_EQ (FRAME_MINIBUF_WINDOW (f), echo_area_window)
    {
      let top: c_int = WINDOW_TOP_EDGE_LINE (XWINDOW (FRAME_MINIBUF_WINDOW (f)));
      let mut col: c_int;

      /* Put cursor at the end of the prompt.  If the mini-buffer
	 is several lines high, find the last line that has
	 any text on it.  */
      let mut row: c_int = FRAME_TOTAL_LINES (f);
      loop
	{
	  row -= 1;
	  col = 0;

	  if MATRIX_ROW_ENABLED_P! ((*f).current_matrix as *mut glyph_matrix, row)
	    {
	      /* Frame rows are filled up with spaces that
		 must be ignored here.  */
	      let r: *mut glyph_row = MATRIX_ROW! ((*f).current_matrix as *mut glyph_matrix,
						  row);
	      let start: *mut glyph = (*r).glyphs [TEXT_AREA as usize];

	      col = (*r).used [TEXT_AREA as usize] as c_int;
	      while 0 < col && (*start.offset (col as isize - 1)).charpos < 0
		{ col -= 1; }
	    }
	  if !(row > top && col == 0)
	    { break; }
	}

      /* We exit the loop with COL at the glyph _after_ the last one.  */
      if col > 0
	{ col -= 1; }

      /* Make sure COL is not out of range.  */
      if col >= FRAME_CURSOR_X_LIMIT (f)
	{
	  /* If we have another row, advance cursor into it.  */
	  if row < FRAME_TOTAL_LINES (f) - 1
	    {
	      col = FRAME_LEFT_SCROLL_BAR_COLS (f);
	      row += 1;
	    }
	  /* Otherwise move it back in range.  */
	  else
	    { col = FRAME_CURSOR_X_LIMIT (f) - 1; }
	}

      cursor_to (f, row, col);
    }
  else
    {
      /* We have only one cursor on terminal frames.  Use it to
	 display the cursor of the selected window of the frame.  */
      let w: *mut window = XWINDOW (FRAME_SELECTED_WINDOW (f));
      if (*w).cursor.vpos >= 0
	  /* The cursor vpos may be temporarily out of bounds
	     in the following situation:  There is one window,
	     with the cursor in the lower half of it.  The window
	     is split, and a message causes a redisplay before
	     a new cursor position has been computed.  */
	  && (*w).cursor.vpos < WINDOW_TOTAL_LINES (w)
	{
	  let mut x: c_int = window_to_frame_hpos (w, (*w).cursor.hpos);
	  let y: c_int = window_to_frame_vpos (w, (*w).cursor.vpos);

	  x += std::cmp::max (0, (*w).left_margin_cols);
	  cursor_to (f, y, x);
	}
    }
}

/* Write desired matrix of tty frame F and make it current.
   INHIBIT_ID_P means that scrolling by insert/delete should not be tried.
   UPDATING_MENU_P true means we are called for updating a tty menu.  */

unsafe fn write_matrix (f: *mut frame, inhibit_id_p: bool, updating_menu_p: bool)
{
  let mut inhibit_id_p = inhibit_id_p;
  /* If we cannot insert/delete lines, it's no use trying it.  */
  if !FRAME_LINE_INS_DEL_OK! (f)
    { inhibit_id_p = true; }

  if baud_rate != FRAME_COST_BAUD_RATE (f) as isize
    { calculate_costs (f); }

 /* See if any of the desired lines are enabled; don't compute for
     i/d line if just want cursor motion.  */
  let first_row: c_int = first_enabled_row ((*f).desired_matrix as *mut glyph_matrix);
  if !inhibit_id_p && first_row >= 0
    { scrolling (f); }

  /* Update the individual lines as needed.  Do bottom line first.  This
     is done so that messages are made visible when pausing.  */
  let last_row: c_int = (*( (*f).desired_matrix as *mut glyph_matrix )).nrows - 1;
  if MATRIX_ROW_ENABLED_P! ((*f).desired_matrix as *mut glyph_matrix, last_row)
    { write_row (f, last_row, updating_menu_p); }

  if first_row >= 0
    {
      let mut i: c_int = first_row;
      while i < last_row
	{
	  if MATRIX_ROW_ENABLED_P! ((*f).desired_matrix as *mut glyph_matrix, i)
	    { write_row (f, i, updating_menu_p); }
	  i += 1;
	}
    }
}

/* Do line insertions/deletions on frame F for frame-based redisplay.  */

unsafe fn scrolling (frame: *mut frame)
{
  /* In fact this code should never be reached at all under
     Android.  */

/* C 5794-5926: `#ifndef HAVE_ANDROID` — live on the frozen target.  */
  let mut unchanged_at_top: c_int; let mut unchanged_at_bottom: c_int;
  let mut window_size: c_int;
  let mut changed_lines: c_int;
  let mut i: c_int;
  let height: c_int = FRAME_TOTAL_LINES (frame);
  let mut free_at_end_vpos: c_int = height;
  let current_matrix: *mut glyph_matrix = (*frame).current_matrix as *mut glyph_matrix;
  let desired_matrix: *mut glyph_matrix = (*frame).desired_matrix as *mut glyph_matrix;
  const _: () = core::assert! (core::mem::size_of::<c_int> () <= core::mem::size_of::<c_uint> ());
  const _: () = core::assert! (core::mem::align_of::<c_uint> () % core::mem::align_of::<c_int> () == 0);
  let old_hash: *mut c_uint;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let mut old_hash_buf: *mut c_uint = core::ptr::null_mut ();
  SAFE_NALLOCA! (sa_avail, old_hash_buf, 4, height);
  old_hash = old_hash_buf;
  let new_hash: *mut c_uint = old_hash.offset (height as isize);
  let draw_cost: *mut c_int = new_hash.offset (height as isize) as *mut c_int;
  let old_draw_cost: *mut c_int = draw_cost.offset (height as isize);
  eassert! (!current_matrix.is_null ());

  /* Compute hash codes of all the lines.  Also calculate number of
     changed lines, number of unchanged lines at the beginning, and
     number of unchanged lines at the end.  */
  changed_lines = 0;
  unchanged_at_top = 0;
  unchanged_at_bottom = height;
  i = 0;
  while i < height
    {
      /* Give up on this scrolling if some old lines are not enabled.  */
      if !MATRIX_ROW_ENABLED_P! (current_matrix, i)
	{
	  SAFE_FREE! (sa_count, sa_mark);
	  return;
	}
      *old_hash.offset (i as isize) = line_hash_code (frame, MATRIX_ROW! (current_matrix, i));
      if !MATRIX_ROW_ENABLED_P! (desired_matrix, i)
	{
	  /* This line cannot be redrawn, so don't let scrolling mess it.  */
	  *new_hash.offset (i as isize) = *old_hash.offset (i as isize);
	  *draw_cost.offset (i as isize) = SCROLL_INFINITY;
	}
      else
	{
	  *new_hash.offset (i as isize)
	    = line_hash_code (frame, MATRIX_ROW! (desired_matrix, i));
	  *draw_cost.offset (i as isize) = line_draw_cost (frame, desired_matrix, i);
	}

      if *old_hash.offset (i as isize) != *new_hash.offset (i as isize)
	{
	  changed_lines += 1;
	  unchanged_at_bottom = height - i - 1;
	}
      else if i == unchanged_at_top
	{ unchanged_at_top += 1; }
      *old_draw_cost.offset (i as isize) = line_draw_cost (frame, current_matrix, i);
      i += 1;
    }

  /* If changed lines are few, don't allow preemption, don't scroll.  */
  if ((!FRAME_SCROLL_REGION_OK! (frame)
       && (changed_lines as isize) < baud_rate / 2400)
      || unchanged_at_bottom == height)
    {
      SAFE_FREE! (sa_count, sa_mark);
      return;
    }

  window_size = (height - unchanged_at_top
		 - unchanged_at_bottom);

  if FRAME_SCROLL_REGION_OK! (frame)
    { free_at_end_vpos -= unchanged_at_bottom; }
  else if FRAME_MEMORY_BELOW_FRAME! (frame)
    { free_at_end_vpos = -1; }

  /* Do id/calc only if small window, or slow terminal, or many lines
     in common between current frame and desired frame.  But the
     window size must be at least 2.  */
  if (FRAME_SCROLL_REGION_OK! (frame)
       || window_size < 18 || baud_rate <= 2400
       || (window_size
	   < 10 * scrolling_max_lines_saved (unchanged_at_top,
					     height - unchanged_at_bottom,
					     old_hash, new_hash, draw_cost)))
      && 2 <= window_size
    { scrolling_1 (frame, window_size, unchanged_at_top, unchanged_at_bottom,
		   draw_cost.offset (unchanged_at_top as isize - 1),
		   old_draw_cost.offset (unchanged_at_top as isize - 1),
		   old_hash.offset (unchanged_at_top as isize - 1),
		   new_hash.offset (unchanged_at_top as isize - 1),
		   free_at_end_vpos - unchanged_at_top); }

  SAFE_FREE! (sa_count, sa_mark);
/* C 5926: `#endif /* HAVE_ANDROID */`.  */
}


/* Count the number of blanks at the start of the vector of glyphs R
   which is LEN glyphs long.  */

unsafe fn count_blanks (f: *mut frame, r: *mut glyph, len: c_int) -> c_int
{
  let mut i: c_int;

  i = 0;
  while i < len
    {
      if !CHAR_GLYPH_SPACE_P! (f, *r.offset (i as isize))
	{ break; }
      i += 1;
    }

  return i;
}


/* Count the number of glyphs in common at the start of the glyph
   vectors STR1 and STR2.  END1 is the end of STR1 and END2 is the end
   of STR2.  Value is the number of equal glyphs equal at the start.  */

unsafe fn count_match (str1: *mut glyph, end1: *mut glyph, str2: *mut glyph,
		       end2: *mut glyph) -> c_int
{
  let mut p1: *mut glyph = str1;
  let mut p2: *mut glyph = str2;

  while p1 < end1
	 && p2 < end2
	 && GLYPH_CHAR_AND_FACE_EQUAL_P! (p1, p2)
    { p1 = p1.offset (1); p2 = p2.offset (1); }

  return p1.offset_from (str1) as c_int;
}


/* Char insertion/deletion cost vector, from term.c */

/* C 5925-5927: `#ifndef HAVE_ANDROID` — live on the frozen target.  */
macro_rules! char_ins_del_cost {
  ($f:expr) => { char_ins_del_vector.offset (FRAME_TOTAL_COLS ($f) as isize) };
}


/* Perform a frame-based update on line VPOS in frame FRAME.  */

unsafe fn write_row (f: *mut frame, vpos: c_int, updating_menu_p: bool)
{
  let mut obody: *mut glyph; let mut nbody: *mut glyph; let mut op1: *mut glyph;
  let mut op2: *mut glyph; let mut np1: *mut glyph; let mut nend: *mut glyph;
  let mut tem: c_int;
  let mut osp: c_int; let mut nsp: c_int; let mut begmatch: c_int; let mut endmatch: c_int;
  let mut olen: c_int; let mut nlen: c_int;
  let current_matrix: *mut glyph_matrix = (*f).current_matrix as *mut glyph_matrix;
  let desired_matrix: *mut glyph_matrix = (*f).desired_matrix as *mut glyph_matrix;
  let current_row: *mut glyph_row = MATRIX_ROW! (current_matrix, vpos);
  let desired_row: *mut glyph_row = MATRIX_ROW! (desired_matrix, vpos);
  let mut must_write_whole_line_p: bool;
  let mut write_spaces_p: bool = FRAME_MUST_WRITE_SPACES! (f);
  let colored_spaces_p: bool = ((*(FACE_FROM_ID (f, DEFAULT_FACE_ID))).background
				!= FACE_TTY_DEFAULT_BG_COLOR);

  if colored_spaces_p
    { write_spaces_p = true; }

  'just_erase: loop
    {
  /* Current row not enabled means it has unknown contents.  We must
     write the whole desired line in that case.  */
  must_write_whole_line_p = !glyph_row_bits! (enabled_p, current_row);
  if must_write_whole_line_p
    {
      obody = core::ptr::null_mut ();
      olen = 0;
    }
  else
    {
      obody = MATRIX_ROW_GLYPH_START! (current_matrix, vpos);
      olen = (*current_row).used [TEXT_AREA as usize] as c_int;

      /* Ignore trailing spaces, if we can.  */
      if !write_spaces_p
	{
	  while olen > 0 && CHAR_GLYPH_SPACE_P! (f, *obody.offset (olen as isize - 1))
	    { olen -= 1; }
	}
    }

  glyph_row_bits! (enabled_p, current_row, true);
  (*current_row).used [TEXT_AREA as usize] = (*desired_row).used [TEXT_AREA as usize];

  /* For some reason, cursor is sometimes moved behind our back when a
     frame with a TTY menu is redrawn.  Homing the cursor as below
     fixes that.  */
  if updating_menu_p
    { cursor_to (f, 0, 0); }

  /* If desired line is empty, just clear the line.  */
  if !glyph_row_bits! (enabled_p, desired_row)
    {
      nlen = 0;
      break 'just_erase;
    }

  nbody = (*desired_row).glyphs [TEXT_AREA as usize];
  nlen = (*desired_row).used [TEXT_AREA as usize] as c_int;
  nend = nbody.offset (nlen as isize);

  /* If display line has unknown contents, write the whole line.  */
  if must_write_whole_line_p
    {
      /* Ignore spaces at the end, if we can.  */
      if !write_spaces_p
	{
	  while nlen > 0 && CHAR_GLYPH_SPACE_P! (f, *nbody.offset (nlen as isize - 1))
	    { nlen -= 1; }
	}

      /* Write the contents of the desired line.  */
      if nlen != 0
	{
          cursor_to (f, vpos, 0);
	  write_glyphs (f, nbody, nlen);
	}

      /* Don't call clear_end_of_line if we already wrote the whole
	 line.  The cursor will not be at the right margin in that
	 case but in the line below.  */
      if nlen < FRAME_TOTAL_COLS (f)
	{
	  cursor_to (f, vpos, nlen);
          clear_end_of_line (f, FRAME_TOTAL_COLS (f));
	}
      else
	/* Make sure we are in the right row, otherwise cursor movement
	   with cmgoto might use `ch' in the wrong row.  */
	{ cursor_to (f, vpos, 0); }
      return;
    }

  /* Pretend trailing spaces are not there at all,
     unless for one reason or another we must write all spaces.  */
  if !write_spaces_p
    {
      while nlen > 0 && CHAR_GLYPH_SPACE_P! (f, *nbody.offset (nlen as isize - 1))
	{ nlen -= 1; }
    }

  /* If there's no i/d char, quickly do the best we can without it.  */
  if !FRAME_CHAR_INS_DEL_OK! (f)
    {
      let mut i: c_int; let mut j: c_int;

      /* Find the first glyph in desired row that doesn't agree with
	 a glyph in the current row, and write the rest from there on.  */
      i = 0;
      while i < nlen
	{
	  if i >= olen || !GLYPH_EQUAL_P! (nbody.offset (i as isize), obody.offset (i as isize))
	    {
	      /* Find the end of the run of different glyphs.  */
	      j = i + 1;
	      while j < nlen
		     && (j >= olen
			 || !GLYPH_EQUAL_P! (nbody.offset (j as isize), obody.offset (j as isize))
			 || CHAR_GLYPH_PADDING_P! (*nbody.offset (j as isize)))
		{ j += 1; }

	      /* Output this run of non-matching chars.  */
	      cursor_to (f, vpos, i);
	      write_glyphs (f, nbody.offset (i as isize), j - i);
	      i = j - 1;

	      /* Now find the next non-match.  */
	    }
	  i += 1;
	}

      /* Clear the rest of the line, or the non-clear part of it.  */
      if olen > nlen
	{
	  cursor_to (f, vpos, nlen);
	  clear_end_of_line (f, olen);
	}
      return;
    }

  /* Here when CHAR_INS_DEL_OK != 0, i.e. we can insert or delete
     characters in a row.  */

  if olen == 0
    {
      /* If current line is blank, skip over initial spaces, if
	 possible, and write the rest.  */
      if write_spaces_p
	{ nsp = 0; }
      else
	{ nsp = count_blanks (f, nbody, nlen); }

      if nlen > nsp
	{
	  cursor_to (f, vpos, nsp);
	  write_glyphs (f, nbody.offset (nsp as isize), nlen - nsp);
	}

      return;
    }

  /* Compute number of leading blanks in old and new contents.  */
  osp = count_blanks (f, obody, olen);
  nsp = if colored_spaces_p { 0 } else { count_blanks (f, nbody, nlen) };

  /* Compute number of matching chars starting with first non-blank.  */
  begmatch = count_match (obody.offset (osp as isize), obody.offset (olen as isize),
			  nbody.offset (nsp as isize), nbody.offset (nlen as isize));

  /* Spaces in new match implicit space past the end of old.  */
  /* A bug causing this to be a no-op was fixed in 18.29.  */
  if !write_spaces_p && osp + begmatch == olen
    {
      np1 = nbody.offset (nsp as isize);
      while np1.offset (begmatch as isize) < nend
	    && CHAR_GLYPH_SPACE_P! (f, *np1.offset (begmatch as isize))
	{ begmatch += 1; }
    }

  /* Avoid doing insert/delete char
     just cause number of leading spaces differs
     when the following text does not match.  */
  if begmatch == 0 && osp != nsp
    { osp = std::cmp::min (osp, nsp); nsp = osp; }

  /* Find matching characters at end of line */
  op1 = obody.offset (olen as isize);
  np1 = nbody.offset (nlen as isize);
  op2 = op1.offset ((begmatch - std::cmp::min (olen - osp, nlen - nsp)) as isize);
  while op1 > op2
	 && GLYPH_EQUAL_P! (op1.offset (-1), np1.offset (-1))
    {
      op1 = op1.offset (-1);
      np1 = np1.offset (-1);
    }
  endmatch = obody.offset_from (op1) as c_int + olen;

  /* tem gets the distance to insert or delete.
     endmatch is how many characters we save by doing so.
     Is it worth it?  */

  tem = (nlen - nsp) - (olen - osp);
  if endmatch != 0 && tem != 0
      && (!FRAME_CHAR_INS_DEL_OK! (f)
/* C 6126-6128: `#ifndef HAVE_ANDROID` (live on the frozen target).  */
          || endmatch <= *char_ins_del_cost! (f).offset (tem as isize)
	)
    { endmatch = 0; }

  /* nsp - osp is the distance to insert or delete.
     If that is nonzero, begmatch is known to be nonzero also.
     begmatch + endmatch is how much we save by doing the ins/del.
     Is it worth it?  */

  if nsp != osp
      && (!FRAME_CHAR_INS_DEL_OK! (f)
/* C 6139-6141: `#ifndef HAVE_ANDROID` (live on the frozen target).  */
	  || begmatch + endmatch <= *char_ins_del_cost! (f).offset ((nsp - osp) as isize)
	)
    {
      begmatch = 0;
      endmatch = 0;
      osp = std::cmp::min (osp, nsp); nsp = osp;
    }

  /* Now go through the line, inserting, writing and
     deleting as appropriate.  */

  if osp > nsp
    {
      cursor_to (f, vpos, nsp);
      delete_glyphs (f, osp - nsp);
    }
  else if nsp > osp
    {
      /* If going to delete chars later in line
	 and insert earlier in the line,
	 must delete first to avoid losing data in the insert */
      if endmatch != 0 && nlen < olen + nsp - osp
	{
	  cursor_to (f, vpos, nlen - endmatch + osp - nsp);
	  delete_glyphs (f, olen + nsp - osp - nlen);
	  olen = nlen - (nsp - osp);
	}
      cursor_to (f, vpos, osp);
      insert_glyphs (f, core::ptr::null_mut (), nsp - osp);
    }
  olen += nsp - osp;

  tem = nsp + begmatch + endmatch;
  if nlen != tem || olen != tem
    {
      if endmatch == 0 || nlen == olen
	{
	  /* If new text being written reaches right margin, there is
	     no need to do clear-to-eol at the end of this function
	     (and it would not be safe, since cursor is not going to
	     be "at the margin" after the text is done).  */
	  if nlen == FRAME_TOTAL_COLS (f)
	    { olen = 0; }

	  /* Function write_glyphs is prepared to do nothing
	     if passed a length <= 0.  Check it here to avoid
	     unnecessary cursor movement.  */
	  if nlen - tem > 0
	    {
	      cursor_to (f, vpos, nsp + begmatch);
	      write_glyphs (f, nbody.offset ((nsp + begmatch) as isize), nlen - tem);
	    }
	}
      else if nlen > olen
	{
	  /* Here, we used to have the following simple code:
	     ----------------------------------------
	     write_glyphs (nbody + nsp + begmatch, olen - tem);
	     insert_glyphs (nbody + nsp + begmatch + olen - tem, nlen - olen);
	     ----------------------------------------
	     but it doesn't work if nbody[nsp + begmatch + olen - tem]
	     is a padding glyph.  */
	  let mut out: c_int = olen - tem;   /* Columns to be overwritten originally.  */
	  let mut del: c_int;

	  cursor_to (f, vpos, nsp + begmatch);

	  /* Calculate columns we can actually overwrite.  */
	  while CHAR_GLYPH_PADDING_P! (*nbody.offset ((nsp + begmatch + out) as isize))
	    { out -= 1; }
	  write_glyphs (f, nbody.offset ((nsp + begmatch) as isize), out);

	  /* If we left columns to be overwritten, we must delete them.  */
	  del = olen - tem - out;
	  if del > 0
	    { delete_glyphs (f, del); }

	  /* At last, we insert columns not yet written out.  */
	  insert_glyphs (f, nbody.offset ((nsp + begmatch + out) as isize),
			 nlen - olen + del);
	  olen = nlen;
	}
      else if olen > nlen
	{
	  cursor_to (f, vpos, nsp + begmatch);
	  write_glyphs (f, nbody.offset ((nsp + begmatch) as isize), nlen - tem);
	  delete_glyphs (f, olen - nlen);
	  olen = nlen;
	}
    }

      /* just_erase: */
      break 'just_erase;
    }

  /* If any unerased characters remain after the new line, erase them.  */
  if olen > nlen
    {
      cursor_to (f, vpos, nlen);
      clear_end_of_line (f, olen);
    }
}


/***********************************************************************
		   X/Y Position -> Buffer Position
 ***********************************************************************/

/* dispextern.h 2285-2318, 2338-2344, 2430-2936 and 2977-2990: `enum display_element_type`,
   `enum line_wrap_method`, `struct it` and `enum move_operation_enum`.  The mirrors stop
   before them (dispextern.rs keeps the glyph/face block), so this unit declares what it
   reaches, with clang's measured offsets for the frozen target — the term.rs/indent.rs shape.
   The record is large and almost all of it is not read here, so the members this unit does
   not name stay measured pads; every offset that is named is pinned below.  */

pub type display_element_type = c_int;
pub const IT_CHARACTER: display_element_type = 0;
pub const IT_COMPOSITION: display_element_type = 1;
pub const IT_GLYPHLESS: display_element_type = 2;
pub const IT_IMAGE: display_element_type = 3;
pub const IT_STRETCH: display_element_type = 4;
pub const IT_EOB: display_element_type = 5;
pub const IT_TRUNCATION: display_element_type = 6;
pub const IT_CONTINUATION: display_element_type = 7;

pub type line_wrap_method = c_int;
pub const TRUNCATE: line_wrap_method = 0;
pub const WORD_WRAP: line_wrap_method = 1;
pub const WINDOW_WRAP: line_wrap_method = 2;

pub const MOVE_TO_X: c_int = 0x01;
pub const MOVE_TO_Y: c_int = 0x02;
pub const MOVE_TO_VPOS: c_int = 0x04;
pub const MOVE_TO_POS: c_int = 0x08;
pub const MOVE_TO_POS_X: c_int = MOVE_TO_POS | MOVE_TO_X;
pub const MOVE_TO_POS_Y: c_int = MOVE_TO_POS | MOVE_TO_Y;

/// C: dispextern.h 2386-2424: `struct composition_it`; only `reversed_p` and `nchars` are
/// read here (clang, frozen target: 44 and 64).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct composition_it
{
  _pad_a: [u8; 44],
  pub reversed_p: bool,
  _pad_b: [u8; 19],
  pub nchars: c_int,
  _pad_c: [u8; 20],
}
const _: () = core::assert! (core::mem::size_of::<composition_it> () == 88);
const _: () = core::assert! (core::mem::offset_of! (composition_it, reversed_p) == 44);
const _: () = core::assert! (core::mem::offset_of! (composition_it, nchars) == 64);

/// C: dispextern.h 2159-2222: `struct bidi_it`; only `paragraph_dir` is read here
/// (clang, frozen target: 2304, sizeof 2328).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bidi_it
{
  _pad_a: [u8; 2304],
  pub paragraph_dir: bidi_dir_t,
  _pad_b: [u8; 20],
}
const _: () = core::assert! (core::mem::size_of::<bidi_it> () == 2328);
const _: () = core::assert! (core::mem::offset_of! (bidi_it, paragraph_dir) == 2304);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct it
{
  pub window: Lisp_Object,              // 2434, offset 0
  pub w: *mut window,                   // 2435, offset 8
  pub f: *mut frame,                    // 2438, offset 16
  _pad_a: [u8; 88],                     // 2441-2458
  pub bits: u8,                         // 2459-2471 (byte 112)
  _pad_b: [u8; 223],                    // 2473-2511
  pub current: display_pos,             // 2512, offset 336
  _pad_c: [u8; 272],                    // 2516-2531
  pub string: Lisp_Object,              // 2532, offset 656
  _pad_d: [u8; 8],                      // 2536-2540
  _pad_e: [u8; 1520],                   // 2541-2666 (`stack`, `sp`)
  _pad_f: [u8; 16],                     // 2670-2689 (`selective`)
  pub what: display_element_type,       // 2690, offset 2208
  pub face_id: c_int,                   // 2693, offset 2212
  pub bits2: [u8; 2],                   // 2696-2705 (bytes 2216-2217)
  _pad_g: [u8; 2],                      // 2706-2707
  pub line_wrap: line_wrap_method,      // 2635, offset 2220
  _pad_g2: [u8; 4],                     // `base_face_id`
  pub c: c_int,                         // 2711, offset 2228
  pub len: c_int,                       // 2232
  _pad_h: [u8; 4],
  pub cmp_it: composition_it,           // 2715, offset 2240
  _pad_i: [u8; 8],                      // `char_to_display`
  pub image_id: ptrdiff_t,              // 2725, offset 2336
  _pad_j: [u8; 64],                     // 2734-2769
  pub object: Lisp_Object,              // 2775, offset 2408
  pub position: text_pos,               // 2777, offset 2416
  _pad_k: [u8; 4],
  pub first_visible_x: c_int,           // 2806, offset 2436
  pub last_visible_x: c_int,            // 2440
  _pad_l: [u8; 28],
  pub glyph_row: *mut glyph_row,        // 2836, offset 2472
  pub area: glyph_row_area,             // 2839, offset 2480
  pub nglyphs: c_int,                   // 2484
  pub pixel_width: c_int,               // 2488
  _pad_m: [u8; 32],
  pub current_x: c_int,                 // 2866, offset 2524
  _pad_n: [u8; 24],
  pub current_y: c_int,                 // 2895, offset 2552
  _pad_o: [u8; 4],
  pub vpos: c_int,                      // 2902, offset 2560
  pub hpos: c_int,                      // 2907, offset 2564
  _pad_p: [u8; 44],
  pub bits3: u32,                       // 2915, offset 2612
  pub bidi_it: bidi_it,                 // 2933, offset 2616
  _pad_q: [u8; 24],
}

const _: () = core::assert! (core::mem::size_of::<it> () == 4968);
const _: () = core::assert! (core::mem::align_of::<it> () == 8);
const _: () = core::assert! (core::mem::offset_of! (it, f) == 16);
const _: () = core::assert! (core::mem::offset_of! (it, current) == 336);
const _: () = core::assert! (core::mem::offset_of! (it, string) == 656);
const _: () = core::assert! (core::mem::offset_of! (it, what) == 2208);
const _: () = core::assert! (core::mem::offset_of! (it, line_wrap) == 2220);
const _: () = core::assert! (core::mem::offset_of! (it, cmp_it) == 2240);
const _: () = core::assert! (core::mem::offset_of! (it, image_id) == 2336);
const _: () = core::assert! (core::mem::offset_of! (it, object) == 2408);
const _: () = core::assert! (core::mem::offset_of! (it, first_visible_x) == 2436);
const _: () = core::assert! (core::mem::offset_of! (it, last_visible_x) == 2440);
const _: () = core::assert! (core::mem::offset_of! (it, glyph_row) == 2472);
const _: () = core::assert! (core::mem::offset_of! (it, pixel_width) == 2488);
const _: () = core::assert! (core::mem::offset_of! (it, current_x) == 2524);
const _: () = core::assert! (core::mem::offset_of! (it, current_y) == 2552);
const _: () = core::assert! (core::mem::offset_of! (it, vpos) == 2560);
const _: () = core::assert! (core::mem::offset_of! (it, hpos) == 2564);
const _: () = core::assert! (core::mem::offset_of! (it, bidi_it) == 2616);

/* dispextern.h 2940 and 328-333.  */
macro_rules! IT_CHARPOS { ($it:expr) => { CHARPOS! (($it).current.pos) }; }
macro_rules! CLIP_TEXT_POS_FROM_MARKER { ($pos:expr, $marker:expr) => { {
  CHARPOS! ($pos) = clip_to_bounds (BEGV! (), marker_position ($marker), ZV! ());
  BYTEPOS! ($pos) = clip_to_bounds (BEGV_BYTE! (), marker_byte_position ($marker), ZV_BYTE! ());
} }; }

unsafe extern "C" {
  /// C: dispextern.h 3556-3557: `extern void *bidi_shelve_cache (void);`
  /// `extern void bidi_unshelve_cache (void *, bool);`
  fn bidi_shelve_cache () -> *mut c_void;
  fn bidi_unshelve_cache (databuf: *mut c_void, really_free_p: bool);
  /// C: dispextern.h 3597: `void start_display (struct it *, struct window *, struct text_pos);`
  fn start_display (it: *mut it, w: *mut window, pos: text_pos);
  /// C: dispextern.h 3602-3604: `void move_it_in_display_line (struct it *, ptrdiff_t, int,
  /// enum move_operation_enum);`
  fn move_it_in_display_line (it: *mut it, to_charpos: ptrdiff_t, to_x: c_int, op: c_int);
  /// C: dispextern.h 3615-3616: `void move_it_to (struct it *, ptrdiff_t, int, int, int, int);`
  fn move_it_to (it: *mut it, to_charpos: ptrdiff_t, to_x: c_int, to_y: c_int,
                 to_vpos: c_int, op: c_int);
  /// C: dispextern.h 3556: `void *bidi_shelve_cache (void);` — see above.
  /// C: frame.h 1677: `struct image *IMAGE_OPT_FROM_ID (struct frame *, int);`
  fn IMAGE_OPT_FROM_ID (f: *mut frame, id: c_int) -> *mut c_void;
}

/* dispextern.h 3613: `extern bool mouse_prefer_closest_glyph;` — owned by this unit and
   defined below; `automatic_hscrolling` (window.c) is the other one this unit reads
   (`Qcurrent_line`'s symbol comes from globals.rs).  */
unsafe extern "C" {
  static mut automatic_hscrolling: Lisp_Object;
}

/* Determine what's under window-relative pixel position (*X, *Y).
   Return the object (string or buffer) that's there.
   Return in *POS the position in that object.
   Adjust *X and *Y to character positions.
   If an image is shown at the specified position, return
   in *OBJECT its image-spec.
   Return in *DX and *DY the pixel coordinates of the click,
   relative to the top left corner of object, or relative to
   the top left corner of the character glyph at (*X, *Y)
   if the object at (*X, *Y) is nil.
   Return WIDTH and HEIGHT of the object at (*X, *Y), or zero
   if the coordinates point to an empty area of the display.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn buffer_posn_from_coords
  (w: *mut window, x: *mut c_int, y: *mut c_int, pos: *mut display_pos,
   object: *mut Lisp_Object, dx: *mut c_int, dy: *mut c_int, width: *mut c_int,
   height: *mut c_int) -> Lisp_Object
{
  let mut it: it = std::mem::zeroed ();
  let old_current_buffer: Lisp_Object = Fcurrent_buffer ();
  let mut startp: text_pos = text_pos { charpos: 0, bytepos: 0 };
  let mut string: Lisp_Object;
  let mut row: *mut glyph_row;
  #[cfg (feature = "have_window_system")]
  let mut img: *mut c_void = core::ptr::null_mut ();   // C 7015: reassigned at 7125/7249/7352
  let mut x0: c_int; let x1: c_int; let mut to_x: c_int; let mut it_vpos: c_int;
  let mut itdata: *mut c_void = core::ptr::null_mut ();

  /* We used to set current_buffer directly here, but that does the
     wrong thing with `face-remapping-alist' (bug#2044).  */
  Fset_buffer ((*w).contents);
  itdata = bidi_shelve_cache ();
  CLIP_TEXT_POS_FROM_MARKER! (startp, (*w).start);
  start_display (&mut it, w, startp);
  x0 = *x;

  /* First, move to the beginning of the row corresponding to *Y.  We
     need to be in that row to get the correct value of base paragraph
     direction for the text at (*X, *Y).  */
  move_it_to (&mut it, -1, 0, *y, -1, MOVE_TO_X | MOVE_TO_Y);

  /* TO_X is the pixel position that the iterator will compute for the
     glyph at *X.  */
  to_x = x0;
  if it.bidi_it.paragraph_dir == R2L
    /* For lines in an R2L paragraph, we need to mirror TO_X wrt the
       text area.  This is because the iterator, even in R2L
       paragraphs, delivers glyphs as if they started at the left
       margin of the window.  (When we actually produce glyphs for
       display, we reverse their order in PRODUCE_GLYPHS, but the
       iterator doesn't know about that.)  The following line adjusts
       the pixel position to the iterator geometry, which is what
       move_it_* routines use.  (The -1 is because in a window whose
       text-area width is W, the rightmost pixel position is W-1, and
       it should be mirrored into zero pixel position.)  */
    { to_x = window_box_width (w, TEXT_AREA) - to_x - 1; }

  /* We need to add it.first_visible_x because iterator positions
     include the hscroll. */
  to_x += it.first_visible_x;

  /* If we are hscrolling only the current line, and Y is at the line
     containing point, augment TO_X with the hscroll amount of the
     current line.  */
  if it.line_wrap == TRUNCATE
      && EQ (automatic_hscrolling, Qcurrent_line ()) && IT_CHARPOS! (it) < PT! ()
    {
      let mut it2: it = it;
      let it2data: *mut c_void = bidi_shelve_cache ();
      it2.last_visible_x = 1000000;
      /* If the line at Y shows point, the call below to
	 move_it_in_display_line will succeed in reaching point.  */
      move_it_in_display_line (&mut it2, PT! (), -1, MOVE_TO_POS);
      if IT_CHARPOS! (it2) >= PT! ()
	{
	  to_x += (((*w).hscroll - (*w).min_hscroll) as c_int) * FRAME_COLUMN_WIDTH (it.f);
	  /* We need to pretend the window is hscrolled, so that
	     move_it_in_display_line below will DTRT with TO_X.  */
	  it.first_visible_x += ((*w).hscroll as c_int) * FRAME_COLUMN_WIDTH (it.f);
	  it.last_visible_x += ((*w).hscroll as c_int) * FRAME_COLUMN_WIDTH (it.f);
	}
      bidi_unshelve_cache (it2data, false);
    }

  /* Now move horizontally in the row to the glyph under *X.  Second
     argument is ZV to prevent move_it_in_display_line from matching
     based on buffer positions.  */
  move_it_in_display_line (&mut it, ZV! (), to_x, MOVE_TO_X);
  if mouse_prefer_closest_glyph
    {
      let next_x: c_int = it.current_x + it.pixel_width;
      let before_dx: c_int = to_x - it.current_x;
      let after_dx: c_int = next_x - to_x;
      if before_dx > after_dx
        { move_it_in_display_line (&mut it, ZV! (), next_x, MOVE_TO_X); }
    }

  bidi_unshelve_cache (itdata, false);

  Fset_buffer (old_current_buffer);

  *dx = to_x - it.current_x;
  *dy = *y - it.current_y;

  string = (*w).contents;
  if STRINGP (it.string)
    { string = it.string; }
  *pos = it.current;
  if it.what == IT_COMPOSITION
      && it.cmp_it.nchars > 1
      && it.cmp_it.reversed_p
    {
      /* The current display element is a grapheme cluster in a
	 composition.  In that case, we need the position of the first
	 character of the cluster.  But, as it.cmp_it.reversed_p is 1,
	 it.current points to the last character of the cluster, thus
	 we must move back to the first character of the same
	 cluster.  */
      CHARPOS! ((*pos).pos) -= (it.cmp_it.nchars - 1) as ptrdiff_t;
      if STRINGP (it.string)
	{ BYTEPOS! ((*pos).pos) = string_char_to_byte (string, CHARPOS! ((*pos).pos)); }
      else
	{ BYTEPOS! ((*pos).pos) = buf_charpos_to_bytepos (XBUFFER ((*w).contents) as *mut c_void,
							  CHARPOS! ((*pos).pos)); }
    }

  #[cfg (feature = "have_window_system")]
  {
  if it.what == IT_IMAGE
    {
      /* Note that this ignores images that are fringe bitmaps,
	 because their image ID is zero, and so IMAGE_OPT_FROM_ID will
	 return NULL.  This is okay, since fringe bitmaps are not
	 displayed in the text area, and so are never the object we
	 are interested in.  */
      img = IMAGE_OPT_FROM_ID (it.f, it.image_id as c_int);
      if !img.is_null () && !NILP ((*(img as *mut image)).spec)
	{ *object = (*(img as *mut image)).spec; }
    }
  }

  /* IT's vpos counts from the glyph row that includes the window's
     start position, i.e. it excludes the header-line row, but
     MATRIX_ROW includes the header-line row.  Adjust for a possible
     header-line row.  */
  it_vpos = it.vpos + window_wants_header_line (w) as c_int
    + window_wants_tab_line (w) as c_int;
  if it_vpos < (*( (*w).current_matrix as *mut glyph_matrix )).nrows
    {
      row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, it_vpos);
      if glyph_row_bits! (enabled_p, row)
	{
      if it.hpos < (*row).used [TEXT_AREA as usize] as c_int
	{
	  let glyph: *mut glyph = (*row).glyphs [TEXT_AREA as usize].offset (it.hpos as isize);
	  /* C 6353-6372: `#ifdef HAVE_WINDOW_SYSTEM if (img) { … } else #endif` — the `if`
	     arm is dead text on this target and the `else` arm is shared; the port writes the
	     pair once per configuration.  */
	  #[cfg (feature = "have_window_system")]
	  {
	    if !img.is_null ()
	      {
		*dy -= (*row).ascent - (*glyph).ascent as c_int;
		*dx += glyph_slice_bits! (x, (*glyph).slice.img) as c_int;
		*dy += glyph_slice_bits! (y, (*glyph).slice.img) as c_int;
		/* Image slices positions are still relative to the entire image */
		*width = (*(img as *mut image)).width as c_int;
		*height = (*(img as *mut image)).height as c_int;
	      }
	    else
	      {
		*width = (*glyph).pixel_width as c_int;
		*height = (*glyph).ascent as c_int + (*glyph).descent as c_int;
	      }
	  }
	  #[cfg (not (feature = "have_window_system"))]
	  {
	    *width = (*glyph).pixel_width as c_int;
	    *height = (*glyph).ascent as c_int + (*glyph).descent as c_int;
	  }
	}
      else
	{
	  *width = 0;
	  *height = (*row).height;
	}
	}
      else
	{
	  *width = 0;
	  *height = 0;
	}
    }
  else
    {
      *width = 0;
      *height = 0;
    }

  /* Add extra (default width) columns if clicked after EOL. */
  x1 = std::cmp::max (0, it.current_x + it.pixel_width);
  if to_x > x1
    { it.hpos += (to_x - x1) / WINDOW_FRAME_COLUMN_WIDTH! (w); }

  *x = it.hpos;
  *y = it.vpos;

  return string;
}


/* Value is the string under window-relative coordinates X/Y in the
   mode line or header line (PART says which) of window W, or nil if none.
   *CHARPOS is set to the position in the string returned.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn mode_line_string
  (w: *mut window, part: window_part, x: *mut c_int, y: *mut c_int, charpos: *mut ptrdiff_t,
   object: *mut Lisp_Object, dx: *mut c_int, dy: *mut c_int, width: *mut c_int,
   height: *mut c_int) -> Lisp_Object
{
  let mut row: *mut glyph_row;
  let mut glyph: *mut glyph; let mut end: *mut glyph;
  let mut x0: c_int; let mut y0: c_int;
  let mut string: Lisp_Object = Qnil ();

  if part == ON_MODE_LINE
    { row = MATRIX_MODE_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix); }
  else if part == ON_TAB_LINE
    { row = MATRIX_TAB_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix); }
  else
    { row = MATRIX_HEADER_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix); }
  y0 = *y - (*row).y;
  *y = row.offset_from (MATRIX_FIRST_TEXT_ROW! ((*w).current_matrix as *mut glyph_matrix))
    as c_int;

  if glyph_row_bits! (mode_line_p, row) && glyph_row_bits! (enabled_p, row)
    {
      /* Find the glyph under X.  If we find one with a string object,
         it's the one we were looking for.  */
      glyph = (*row).glyphs [TEXT_AREA as usize];
      end = glyph.offset ((*row).used [TEXT_AREA as usize] as isize);
      x0 = *x;
      while glyph < end && x0 >= (*glyph).pixel_width as c_int
	{
	  x0 -= (*glyph).pixel_width as c_int;
	  glyph = glyph.offset (1);
	}
      *x = glyph.offset_from ((*row).glyphs [TEXT_AREA as usize]) as c_int;
      if glyph < end
	{
	  string = (*glyph).object;
	  *charpos = (*glyph).charpos;
	  *width = (*glyph).pixel_width as c_int;
	  *height = (*glyph).ascent as c_int + (*glyph).descent as c_int;
	  #[cfg (feature = "have_window_system")]
	  {
	  if glyph_bits! (type, *glyph) == IMAGE_GLYPH
	    {
	      let img: *mut c_void;
	      img = IMAGE_OPT_FROM_ID (WINDOW_XFRAME! (w), (*glyph).u.img_id);
	      if !img.is_null ()
		{
		  *object = (*(img as *mut image)).spec;
		  x0 += glyph_slice_bits! (x, (*glyph).slice.img) as c_int;
		  y0 += glyph_slice_bits! (y, (*glyph).slice.img) as c_int;
		}
	      y0 -= (*row).ascent - (*glyph).ascent as c_int;
	    }
	  }
	}
      else
	{
	  /* Add extra (default width) columns if clicked after EOL. */
	  *x += x0 / WINDOW_FRAME_COLUMN_WIDTH! (w);
	  *width = 0;
	  *height = (*row).height;
	}
    }
  else
    {
      *x = 0;
      x0 = 0;
      *width = 0;
      *height = 0;
    }

  *dx = x0;
  *dy = y0;

  return string;
}


/* Value is the string under window-relative coordinates X/Y in either
   marginal area, or nil if none.  *CHARPOS is set to the position in
   the string returned.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn marginal_area_string
  (w: *mut window, part: window_part, x: *mut c_int, y: *mut c_int, charpos: *mut ptrdiff_t,
   object: *mut Lisp_Object, dx: *mut c_int, dy: *mut c_int, width: *mut c_int,
   height: *mut c_int) -> Lisp_Object
{
  let mut row: *mut glyph_row =
    (*( (*w).current_matrix as *mut glyph_matrix )).rows;
  let mut glyph: *mut glyph; let mut end: *mut glyph;
  let mut x0: c_int; let mut y0: c_int; let mut i: c_int; let wy: c_int = *y;
  let area: c_int;
  let mut string: Lisp_Object = Qnil ();

  if part == ON_LEFT_MARGIN
    { area = LEFT_MARGIN_AREA; }
  else if part == ON_RIGHT_MARGIN
    { area = RIGHT_MARGIN_AREA; }
  else
    { emacs_abort (); }

  i = 0;
  while glyph_row_bits! (enabled_p, row)
	&& i < (*( (*w).current_matrix as *mut glyph_matrix )).nrows
    {
      if wy >= (*row).y && wy < MATRIX_ROW_BOTTOM_Y! (row)
	{ break; }
      i += 1; row = row.offset (1);
    }
  y0 = *y - (*row).y;
  *y = row.offset_from (MATRIX_FIRST_TEXT_ROW! ((*w).current_matrix as *mut glyph_matrix))
    as c_int;

  if glyph_row_bits! (enabled_p, row)
    {
      /* Find the glyph under X.  If we find one with a string object,
	 it's the one we were looking for.  */
      if area == RIGHT_MARGIN_AREA
	{ x0 = ((if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
		   { WINDOW_LEFT_FRINGE_WIDTH! (w) }
		   else { WINDOW_FRINGES_WIDTH! (w) })
		+ window_box_width (w, LEFT_MARGIN_AREA)
		+ window_box_width (w, TEXT_AREA)); }
      else
	{ x0 = if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
	    { WINDOW_LEFT_FRINGE_WIDTH! (w) } else { 0 }; }

      glyph = (*row).glyphs [area as usize];
      end = glyph.offset ((*row).used [area as usize] as isize);
      x0 = *x - x0;
      while glyph < end && x0 >= (*glyph).pixel_width as c_int
	{
	  x0 -= (*glyph).pixel_width as c_int;
	  glyph = glyph.offset (1);
	}
      *x = glyph.offset_from ((*row).glyphs [area as usize]) as c_int;
      if glyph < end
	{
	  string = (*glyph).object;
	  *charpos = (*glyph).charpos;
	  *width = (*glyph).pixel_width as c_int;
	  *height = (*glyph).ascent as c_int + (*glyph).descent as c_int;
	  #[cfg (feature = "have_window_system")]
	  {
	  if glyph_bits! (type, *glyph) == IMAGE_GLYPH
	    {
	      let img: *mut c_void;
	      img = IMAGE_OPT_FROM_ID (WINDOW_XFRAME! (w), (*glyph).u.img_id);
	      if !img.is_null ()
		{ *object = (*(img as *mut image)).spec; }
	      y0 -= (*row).ascent - (*glyph).ascent as c_int;
	      x0 += glyph_slice_bits! (x, (*glyph).slice.img) as c_int;
	      y0 += glyph_slice_bits! (y, (*glyph).slice.img) as c_int;
	    }
	  }
	}
      else
	{
	  /* Add extra (default width) columns if clicked after EOL. */
	  *x += x0 / WINDOW_FRAME_COLUMN_WIDTH! (w);
	  *width = 0;
	  *height = (*row).height;
	}
    }
  else
    {
      x0 = 0;
      *x = 0;
      *width = 0;
      *height = 0;
    }

  *dx = x0;
  *dy = y0;

  return string;
}


/***********************************************************************
		   Terminal Related Lisp Functions
 ***********************************************************************/

/// frame.h 356's `ENUM_BF (output_method) output_method : 4` (unit 1, bits 21-24).  The mirror
/// has the reader; this unit writes the field, so C's field name stays in the text as a named
/// accessor pair (P-80's member-name form).
macro_rules! output_method {
  ($f:expr) => { frame_output_method ($f) };
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & !(FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
      | ((($v) as u32) << FRAME_OUTPUT_METHOD_SHIFT) };
}

/// frame.h 1425-1426: `AUTO_FRAME_ARG (name, parameter, value)` is `AUTO_LIST1 (name,
/// AUTO_CONS_EXPR (parameter, value))`.  The mirror's copy (`include/frame.rs` 1009) cannot
/// expand: it binds the name `$name:expr` and `AUTO_LIST1!` takes an `ident` there ("no rules
/// expected `expr` metavariable", E-none), so the unit keeps C's two lines with C's own name
/// position (reported to the mirror's owner, not changed there).

/* <unistd.h>: `STDIN_FILENO` now comes from the unistd.in.rs mirror (2026-10-04); this unit's
   use-site copy is gone, and the mirror is the single declaration point (PORTING.md
   0.1-3/P-26).  */

/// frame.h 1375-1376: `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL` are C's plain
/// `foreground_pixel`/`background_pixel` fields.  The mirror has the readers; this unit also
/// writes them, so the two-line pair is declared here at its use site (P-72 rule 3).
macro_rules! FRAME_FOREGROUND_PIXEL { ($f:expr) => { (*$f).foreground_pixel }; }
macro_rules! FRAME_BACKGROUND_PIXEL { ($f:expr) => { (*$f).background_pixel }; }

/// termchar.h 62-64: `bool_bf term_initted : 1` — the mirror stores the run of one as the
/// byte `tty_display_info.bits_initted` and names no mask for it, so the one-line accessor is
/// written here (P-80's member-name form).
macro_rules! term_initted { ($tty:expr) => { ((*($tty)).bits_initted & 1) != 0 }; }

/* C 6606-6657: `#ifdef SIGWINCH` — live on this target (SIGWINCH is `1 << 21` here); the
   arm is not gated because the mirror only names the branch for the platforms that lack it.  */
unsafe extern "C" fn handle_window_change_signal (sig: c_int)
{
  let mut width: c_int = 0; let mut height: c_int = 0;   /* C declares these uninitialised.  */
  let mut tty: *mut tty_display_info;

  /* The frame size change obviously applies to a single
     termcap-controlled terminal, but we can't decide which.
     Therefore, we resize the frames corresponding to each tty.
  */
  tty = tty_list;
  while !tty.is_null ()
    {
      if !term_initted! (tty)
	{ tty = (*tty).next; continue; }

      /* Suspended tty frames have tty->input == NULL avoid trying to
	 use it.  */
      if (*tty).input.is_null ()
	{ tty = (*tty).next; continue; }

      get_tty_size (fileno ((*tty).input), &mut width, &mut height);

      if width > 5 && height > 2
	{
	  let mut tail: Lisp_Object;
	  let mut frame: Lisp_Object;

	  FOR_EACH_FRAME! (tail, frame, {
	      let f: *mut frame = XFRAME (frame);

	      if FRAME_TERMCAP_P (f) && FRAME_TTY! (f) == tty
		  && FRAME_PARENT_FRAME (f).is_null ()
		/* Record the new sizes, but don't reallocate the data
		   structures now.  Let that be done later outside of the
		   signal handler.  */
		{ change_frame_size (f, width, height, false, true, false); }
	  });
	}
      tty = (*tty).next;
    }
}

unsafe extern "C" fn deliver_window_change_signal (sig: c_int)
{
  deliver_process_signal (sig, Some (handle_window_change_signal));
}


/* Do any change in frame size that was requested by a signal.
   SAFE means this function is called from a place where it is
   safe to change frame sizes while a redisplay is in progress.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn do_pending_window_change (safe: bool)
{
  if (redisplaying_p || dont_resize_frames) && !safe
    { return; }

  while delayed_size_change
    {
      let mut tail: Lisp_Object;
      let mut frame: Lisp_Object;

      delayed_size_change = false;

      FOR_EACH_FRAME! (tail, frame, {
	  let f: *mut frame = XFRAME (frame);

	  /* Negative new_width or new_height values mean no change is
	     required (a native size can never drop below zero).  If
	     new_size_p is not set, this means the size change was
	     requested by adjust_frame_size but has not been honored by
	     the window manager yet.  */
	  if new_size_p! (f) && ((*f).new_height >= 0 || (*f).new_width >= 0)
	    { change_frame_size (f, (*f).new_width, (*f).new_height,
			       false, false, safe); }
      });
    }
}


unsafe fn change_frame_size_1 (f: *mut frame, new_width: c_int, new_height: c_int,
			       pretend: bool, delay: bool, safe: bool)
{
  if delay || (redisplaying_p && !safe)
    {
      if CONSP (frame_size_history)
	  && (((*f).new_width != new_width
	       || (*f).new_height != new_height
	       || new_width != FRAME_PIXEL_WIDTH (f)
	       || new_height != FRAME_PIXEL_HEIGHT (f)))
	{
	  frame_size_history_extra
	  (f, build_string (c"change_frame_size_1, delayed".as_ptr ()),
	   FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f),
	   new_width, new_height, (*f).new_width, (*f).new_height);
	}

      /* We can't deal with the change now, queue it for later.  */
      (*f).new_width = new_width;
      (*f).new_height = new_height;
      new_size_p! (f, true);
      delayed_size_change = true;
    }
  else
    {
      /* Storing -1 in the new_width/new_height slots means that no size
	 change is pending.  Native sizes are always non-negative.
	 Reset the new_size_p slot as well.  */
      (*f).new_height = -1;
      (*f).new_width = -1;
      new_size_p! (f, false);
      /* adjust_frame_size wants its arguments in terms of text_width
	 and text_height, so convert them here.  For pathologically
	 small frames, the resulting values may be negative though.  */
      adjust_frame_size (f, FRAME_PIXEL_TO_TEXT_WIDTH! (f, new_width),
			 FRAME_PIXEL_TO_TEXT_HEIGHT! (f, new_height), 5,
			 pretend, Qchange_frame_size ());
    }
}


/* Change native height/width of frame F to NEW_WIDTH/NEW_HEIGHT pixels.
   Values may be given as -1 to indicate that no change is needed.

   If DELAY, assume we're being called from a signal handler, and queue
   the change for later - perhaps the next redisplay.  Since this tries
   to resize windows, we can't call it from a signal handler.

   SAFE means this function is called from a place where it's safe to
   change frame sizes while a redisplay is in progress.  */
#[unsafe(no_mangle)] pub unsafe extern "C" fn change_frame_size
  (f: *mut frame, new_width: c_int, new_height: c_int, pretend: bool, delay: bool, safe: bool)
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;

  if FRAME_MSDOS_P (f) && FRAME_PARENT_FRAME (f).is_null ()
    {
      /* On MS-DOS, all frames use the same screen, so a change in
         size affects all frames.  Termcap now supports multiple
         ttys. */
      FOR_EACH_FRAME! (tail, frame, {
	if !FRAME_WINDOW_P (XFRAME (frame))
	    && FRAME_PARENT_FRAME (XFRAME (frame)).is_null ()
	  { change_frame_size_1 (XFRAME (frame), new_width, new_height,
			       pretend, delay, safe); }
      });
    }
  else
    { change_frame_size_1 (f, new_width, new_height, pretend, delay, safe); }
}

/* Return non-zero if we delayed size-changes of frame F and haven't
   handled them yet, which means we cannot be sure about the exact
   dimensions of our frames.  */
#[unsafe(no_mangle)] pub unsafe extern "C" fn frame_size_change_delayed
  (f: *mut frame) -> bool
{
  return delayed_size_change || new_size_p! (f);
}

DEFUN! ("open-termscript", Fopen_termscript, Sopen_termscript,
        1, 1, "FOpen termscript file: ",
        doc: "Start writing all terminal output to FILE as well as the terminal.\nFILE = nil means just close any termscript file currently open.",
        (file: Lisp_Object),
        {
  let mut tty: *mut tty_display_info;

  if !is_tty_frame (SELECTED_FRAME ())
    { error (c"Current frame is not on a tty device".as_ptr ()); }

  tty = CURTTY! ();

  if !(*tty).termscript.is_null ()
    {
      block_input ();
      emacs_fclose! ((*tty).termscript);
      (*tty).termscript = core::ptr::null_mut ();
      unblock_input ();
    }

  let mut file = file;
  if !NILP (file)
    {
      file = Fexpand_file_name (file, Qnil ());
      (*tty).termscript = emacs_fopen (SSDATA (file), c"w".as_ptr ());
      if (*tty).termscript.is_null ()
	{ report_file_error (c"Opening termscript".as_ptr (), file); }
    }
  return Qnil ();
});


DEFUN! ("send-string-to-terminal", Fsend_string_to_terminal,
        Ssend_string_to_terminal, 1, 2, 0,
        doc: "Send STRING to the terminal without alteration.\nControl characters in STRING will have terminal-dependent effects.\n\nOptional parameter TERMINAL specifies the tty terminal device to use.\nIt may be a terminal object, a frame, or nil for the terminal used by\nthe currently selected frame.  In batch mode, STRING is sent to stdout\nwhen TERMINAL is nil.",
        (string: Lisp_Object, terminal: Lisp_Object),
        {
  let t: *mut terminal = decode_live_terminal (terminal);
  let out: *mut FILE;

  /* ??? Perhaps we should do something special for multibyte strings here.  */
  CHECK_STRING (string);
  block_input ();

  if (*t).r#type == output_initial
    { out = stdout; }
  else if (*t).r#type != output_termcap && (*t).r#type != output_msdos_raw
    { error (c"Device %d is not a termcap terminal device".as_ptr (), (*t).id); }
  else
    {
      let tty: *mut tty_display_info = (*t).display_info.tty as *mut tty_display_info;

      if (*tty).output.is_null ()
	{ error (c"Terminal is currently suspended".as_ptr ()); }

      if !(*tty).termscript.is_null ()
	{
	  fwrite (SDATA (string) as *const c_void, 1, SBYTES (string) as usize,
		  (*tty).termscript);
	  fflush ((*tty).termscript);
	}
      out = (*tty).output;
    }
  /* STRING might be very long, in which case fwrite could be
     interrupted by SIGIO.  So we temporarily block SIGIO.  */
  unrequest_sigio ();
  fwrite (SDATA (string) as *const c_void, 1, SBYTES (string) as usize, out);
  fflush (out);
  request_sigio ();
  unblock_input ();
  return Qnil ();
});


DEFUN! ("ding", Fding, Sding, 0, 1, 0,
        doc: "Beep, or flash the screen.\nAlso, unless an argument is given,\nterminate any keyboard macro currently executing.",
        (arg: Lisp_Object),
        {
  if !NILP (arg)
    {
      if noninteractive
	{ putchar (7); }
      else
	{ ring_bell (XFRAME (selected_frame)); }
    }
  else
    { bitch_at_user (); }

  return Qnil ();
});

/* `dispextern.rs` declares `bitch_at_user` at its crate root (dispextern.h 3900), so the
   definition lives in a glob-imported module: a second crate-root item of the same name
   would be `E0428` (the `space_glyph_def` device).  */
pub mod bitch_at_user_def {
  use super::*;

  #[unsafe(no_mangle)] pub unsafe extern "C" fn bitch_at_user ()
  {
    if noninteractive
      { putchar (7); }
    else if !INTERACTIVE! ()   /* Stop executing a keyboard macro.  */
      {
	let msg: *const c_char
	  = c"Keyboard macro terminated by a command ringing the bell".as_ptr ();
	Fsignal (Quser_error (), list1 (build_string (msg)));
      }
    else
      { ring_bell (XFRAME (selected_frame)); }
  }
}
use bitch_at_user_def::*;

DEFUN! ("sleep-for", Fsleep_for, Ssleep_for, 1, 2, 0,
        doc: "Pause, without updating display, for SECONDS seconds.\nSECONDS may be a floating-point value, meaning that you can wait for a\nfraction of a second.\nAn optional second arg MILLISECONDS can be provided but is deprecated:\nit specifies an additional wait period, in milliseconds.",
        (seconds: Lisp_Object, milliseconds: Lisp_Object),
        {
  let mut duration: c_double = extract_float (seconds);

  if !NILP (milliseconds)
    {
      CHECK_FIXNUM (milliseconds);
      duration += XFIXNUM (milliseconds) as c_double / 1000.0;
    }

  if duration > 0.0
    {
      let mut t: timespec = dtotimespec (duration);
      let tend: timespec = timespec_add (current_timespec (), t);

      /* wait_reading_process_output returns as soon as it detects
	 output from any subprocess, so we wait in a loop until the
	 time expires.  */
      loop {
	wait_reading_process_output (std::cmp::min (t.tv_sec, WAIT_READING_MAX),
				     t.tv_nsec as c_int, 0, false, Qnil (),
				     core::ptr::null_mut (), 0);
	t = timespec_sub (tend, current_timespec ());
	if !(timespec_sign (t) > 0)
	  { break; }
      }
    }

  return Qnil ();
});


/* This is just like wait_reading_process_output, except that
   it does redisplay.

   TIMEOUT is number of seconds to wait (float or integer),
   or t to wait forever.
   READING is true if reading input.
   If DISPLAY_OPTION is >0 display process output while waiting.
   If DISPLAY_OPTION is >1 perform an initial redisplay before waiting.

   Returns a boolean Qt if we waited the full time and returns Qnil if the
   wait was interrupted by incoming process output or keyboard events.

   FIXME: When `wait_reading_process_output` returns early because of
   process output, instead of returning nil we should loop and wait some
   more (i.e. until either there's pending input events or the timeout
   expired).  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn sit_for (timeout: Lisp_Object, reading: bool,
						       display_option: c_int) -> Lisp_Object
{
  let mut sec: i64 = 0;   /* C writes it through an out parameter.  */
  let mut nsec: c_int;
  let do_display: bool = display_option > 0;
  let curbuf_eq_winbuf: bool
    = ((current_buffer! ()) == XBUFFER ((*XWINDOW (selected_window)).contents));

  swallow_events (do_display);

  if detect_input_pending_run_timers (do_display)
      || !NILP (Vexecuting_kbd_macro)
    { return Qnil (); }

  if display_option > 1
    { redisplay_preserve_echo_area (2); }

  if INTEGERP (timeout)
    {
      if integer_to_intmax (timeout, &mut sec)
	{
	  if sec <= 0
	    { return Qt (); }
	  sec = std::cmp::min (sec, WAIT_READING_MAX as i64);
	}
      else
	{
	  if NILP (Fnatnump (timeout))
	    { return Qt (); }
	  sec = WAIT_READING_MAX as i64;
	}
      nsec = 0;
    }
  else if FLOATP (timeout)
    {
      let seconds: c_double = XFLOAT_DATA (timeout);
      if !(0.0 < seconds)
	{ return Qt (); }
      else
	{
	  let t: timespec = dtotimespec (seconds);
	  sec = std::cmp::min (t.tv_sec, WAIT_READING_MAX as i64);
	  nsec = t.tv_nsec as c_int;
	}
    }
  else if EQ (timeout, Qt ())
    {
      sec = 0;
      nsec = 0;
    }
  else
    { wrong_type_argument (Qnumberp (), timeout); }


/* C 6996-6998: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` — live on the frozen
   target (`USABLE_SIGIO` is 1 in docs/baseline/config/darwin-arm64.macros.tsv).  */
  gobble_input ();

  let nbytes: c_int
    = wait_reading_process_output (sec, nsec, if reading { -1 } else { 1 }, do_display,
			           Qnil (), core::ptr::null_mut (), 0);

  if reading && curbuf_eq_winbuf
    /* Timers and process filters/sentinels may have changed the selected
       window (e.g. in response to a connection from emacsclient), in which
       case we should follow it (unless we weren't in the selected-window's
       buffer to start with).  */
    { set_buffer_internal (XBUFFER ((*XWINDOW (selected_window)).contents)); }

  return if nbytes > 0 || detect_input_pending () { Qnil () } else { Qt () };
}


DEFUN! ("redisplay", Fredisplay, Sredisplay, 0, 1, 0,
        doc: "Perform redisplay.\nOptional arg FORCE exists for historical reasons and is ignored.\nValue is t if redisplay has been performed, nil if executing a\nkeyboard macro.",
        (force: Lisp_Object),
        {
  swallow_events (true);
  if !NILP (Vexecuting_kbd_macro)
    { return Qnil (); }

  redisplay_preserve_echo_area (2);
  return Qt ();
});


/***********************************************************************
			 Other Lisp Functions
 ***********************************************************************/

/* A vector of size >= 2 * NFRAMES + 3 * NBUFFERS + 1, containing the
   session's frames, frame names, buffers, buffer-read-only flags, and
   buffer-modified-flags.  */

static mut frame_and_buffer_state: Lisp_Object = core::ptr::null_mut ();


DEFUN! ("frame-or-buffer-changed-p", Fframe_or_buffer_changed_p,
        Sframe_or_buffer_changed_p, 0, 1, 0,
        doc: "Return non-nil if the frame and buffer state appears to have changed.\nVARIABLE is a variable name whose value is either nil or a state vector\nthat will be updated to contain all frames and buffers,\naside from buffers whose names start with space,\nalong with the buffers' read-only and modified flags.  This allows a fast\ncheck to see whether buffer menus might need to be recomputed.\nIf this function returns non-nil, it updates the internal vector to reflect\nthe current state.\n\nIf VARIABLE is nil, an internal variable is used.  Users should not\npass nil for VARIABLE.",
        (variable: Lisp_Object),
        {
  let mut state: Lisp_Object;
  let mut tail: Lisp_Object; let mut frame: Lisp_Object; let mut buf: Lisp_Object;
  let mut n: ptrdiff_t; let mut idx: ptrdiff_t;

  /* C's `FOR_EACH_LIVE_BUFFER` (buffer.h 1445-1447) wraps lisp.h 5908-5911's
     `FOR_EACH_ALIST_VALUE (Vbuffer_alist, list_var, buf_var)`, which the P-22 decision leaves
     unmirrored, so the loop is written out here (the ITREE_FOREACH shape, status 3.7) and
     `docs/ports/macro-body-notes.tsv` records the one token the port therefore cannot spell.  */
  'changed: loop
    {
  if !NILP (variable)
    {
      CHECK_SYMBOL (variable);
      state = Fsymbol_value (variable);
      if !VECTORP (state)
	{ break 'changed; }
    }
  else
    { state = frame_and_buffer_state; }

  idx = 0;
  tail = Vframe_list;
  while CONSP (tail) && { frame = XCAR (tail); true }
    {
      if idx == ASIZE (state)
	{ break 'changed; }
      if !BASE_EQ (AREF (state, idx), frame)
	{ break 'changed; }
      idx += 1;
      if idx == ASIZE (state)
	{ break 'changed; }
      if !EQ (AREF (state, idx), (*XFRAME (frame)).name)
	{ break 'changed; }
      idx += 1;
      tail = XCDR (tail);
    }
  /* Check that the buffer info matches.  */
  tail = Vbuffer_alist;
  while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
    {
      /* Ignore buffers that aren't included in buffer lists.  */
      if SREF (BVAR! (XBUFFER (buf), name_), 0) == b' '
	{ tail = XCDR (tail); continue; }
      if idx == ASIZE (state)
	{ break 'changed; }
      if !BASE_EQ (AREF (state, idx), buf)
	{ break 'changed; }
      idx += 1;
      if idx == ASIZE (state)
	{ break 'changed; }
      if !EQ (AREF (state, idx), BVAR! (XBUFFER (buf), read_only_))
	{ break 'changed; }
      idx += 1;
      if idx == ASIZE (state)
	{ break 'changed; }
      if !EQ (AREF (state, idx), Fbuffer_modified_p (buf))
	{ break 'changed; }
      idx += 1;
      tail = XCDR (tail);
    }
  if idx == ASIZE (state)
    { break 'changed; }
  /* Detect deletion of a buffer at the end of the list.  */
  if EQ (AREF (state, idx), Qlambda ())
    { return Qnil (); }

      /* changed: */
      break 'changed;
    }

      /* Come here if we decide the data has changed.  */
  /* Count the size we will need.
     Start with 1 so there is room for at least one lambda at the end.  */
  n = 1;
  tail = Vframe_list;
  while CONSP (tail) && { frame = XCAR (tail); true }
    {
    n += 2;
      tail = XCDR (tail);
    }
  tail = Vbuffer_alist;
  while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
    {
    n += 3;
      tail = XCDR (tail);
    }
  /* Reallocate the vector if data has grown to need it,
     or if it has shrunk a lot.  */
  if !VECTORP (state)
      || n > ASIZE (state)
      || n + 20 < ASIZE (state) / 2
    /* Add 20 extra so we grow it less often.  */
    {
      state = make_vector (n + 20, Qlambda ());
      if !NILP (variable)
	{ Fset (variable, state); }
      else
	{ frame_and_buffer_state = state; }
    }

  /* Record the new data in the (possibly reallocated) vector.  */
  idx = 0;
  tail = Vframe_list;
  while CONSP (tail) && { frame = XCAR (tail); true }
    {
      ASET (state, idx, frame);
      idx += 1;
      ASET (state, idx, (*XFRAME (frame)).name);
      idx += 1;
      tail = XCDR (tail);
    }
  tail = Vbuffer_alist;
  while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
    {
      /* Ignore buffers that aren't included in buffer lists.  */
      if SREF (BVAR! (XBUFFER (buf), name_), 0) == b' '
	{ tail = XCDR (tail); continue; }
      ASET (state, idx, buf);
      idx += 1;
      ASET (state, idx, BVAR! (XBUFFER (buf), read_only_));
      idx += 1;
      ASET (state, idx, Fbuffer_modified_p (buf));
      idx += 1;
      tail = XCDR (tail);
    }
  /* Fill up the vector with lambdas (always at least one).  */
  ASET (state, idx, Qlambda ());
  idx += 1;
  while idx < ASIZE (state)
    {
      ASET (state, idx, Qlambda ());
      idx += 1;
    }
  /* Make sure we didn't overflow the vector.  */
  eassert! (idx <= ASIZE (state));
  return Qt ();
});


/***********************************************************************
			    Initialization
***********************************************************************/

unsafe fn init_faces_initial ()
{
  /* For the initial frame, we don't have any way of knowing what
     are the foreground and background colors of the terminal.  */
  let sf: *mut frame = SELECTED_FRAME ();

  FRAME_FOREGROUND_PIXEL! (sf) = FACE_TTY_DEFAULT_FG_COLOR;
  FRAME_BACKGROUND_PIXEL! (sf) = FACE_TTY_DEFAULT_BG_COLOR;
  call0 (Qtty_set_up_initial_frame_faces ());
}

/* Initialization done when Emacs fork is started, before doing stty.
   Determine terminal type and set terminal_driver.  Then invoke its
   decoding routine to set up variables in the terminal package.  */

unsafe fn init_display_interactive ()
{
  let mut terminal_type: *mut c_char;

  /* Construct the space glyph.  */
  glyph_bits! (type, space_glyph, CHAR_GLYPH);
  SET_CHAR_GLYPH! (core::ptr::null_mut (), space_glyph, ' ' as c_uint, DEFAULT_FACE_ID, false);
  space_glyph.charpos = -1;

  inverse_video = false;
  cursor_in_echo_area = false;

  /* Now is the time to initialize this; it's used by init_sys_modes
     during startup.  */
  Vinitial_window_system = Qnil ();

  /* SIGWINCH needs to be handled no matter what display we start
     with.  Otherwise newly opened tty frames will not resize
     automatically. */
/* C 7222-7232: `#ifdef SIGWINCH` — live on the frozen target.  */
  if !will_dump_p ()
    {
      let mut action: sigaction = std::mem::zeroed ();
      emacs_sigaction_init (&mut action, Some (deliver_window_change_signal));
      sigaction (SIGWINCH, &action, core::ptr::null_mut ());
    }

  /* If running as a daemon, no need to initialize any frames/terminal,
     except on Windows, where we at least want to initialize it.  */
  if IS_DAEMON ()
    {
      /* Pdump'ed Emacs doesn't record the initial frame from temacs,
	 so the non-basic faces realized for that frame in temacs
	 aren't in emacs.  This causes errors when users try to
	 customize those faces in their init file.  The call to
	 init_faces_initial will realize these faces now.  (Non-daemon
	 Emacs does this either near the end of this function or when
	 the GUI frame is created.)  */
      if dumped_with_pdumper_p ()
        { init_faces_initial (); }
/* C 7245-7248: `#ifndef WINDOWSNT` — live on the frozen target.  */
      return;
    }

  /* If the user wants to use a window system, we shouldn't bother
     initializing the terminal.  This is especially important when the
     terminal is so dumb that emacs gives up before and doesn't bother
     using the window system.

     If the DISPLAY environment variable is set and nonempty,
     try to use X, and if that fails output a line to stderr
     reporting that -nw will be simulated.  */

  #[cfg (feature = "have_x_windows")]
  {
  if !inhibit_window_system && !display_arg
    {
      let display: *mut c_char;
      display = getenv (c"DISPLAY".as_ptr ());
      display_arg = !display.is_null () && *display != 0;

      if display_arg && !x_display_ok (display)
	{
	  fprintf (stderr, c"Display %s unavailable, simulating -nw\n".as_ptr (),
		   display);
	  inhibit_window_system = 1;
	}
    }

  if !inhibit_window_system && display_arg
    {
      Vinitial_window_system = Qx ();
      #[cfg (feature = "use_ncurses")]
      {
      /* In some versions of ncurses,
	 tputs crashes if we have not called tgetent.
	 So call tgetent.  */
      { let mut b: [c_char; 2044] = [0; 2044]; tgetent (b.as_mut_ptr (), c"xterm".as_ptr ()); }
      }
      return;
    }
  }

  #[cfg (feature = "have_android")]
  {
  if !inhibit_window_system && android_init_gui
    {
      Vinitial_window_system = Qandroid ();
      android_term_init ();
      return;
    }
  }

  #[cfg (feature = "have_ntgui")]
  {
  if !inhibit_window_system
    {
      Vinitial_window_system = Qw32 ();
      return;
    }
  }

  #[cfg (feature = "have_ns")]
  {
  if !inhibit_window_system && !will_dump_p ()
    {
      Vinitial_window_system = Qns ();
      return;
    }
  }

  #[cfg (feature = "have_pgtk")]
  {
  if !inhibit_window_system && !will_dump_p ()
    {
      Vinitial_window_system = Qpgtk ();
      return;
    }
  }

  #[cfg (feature = "have_haiku")]
  {
  if !inhibit_window_system && !will_dump_p ()
    {
      Vinitial_window_system = Qhaiku ();
      return;
    }
  }

  /* If no window system has been specified, try to use the terminal.  */
  if isatty (STDIN_FILENO) == 0
    { fatal (c"standard input is not a tty".as_ptr ()); }

  #[cfg (feature = "windowsnt")]
  { terminal_type = c"w32console".as_ptr () as *mut c_char; }
  #[cfg (not (feature = "windowsnt"))]
  { terminal_type = getenv (c"TERM".as_ptr ()); }
  if terminal_type.is_null ()
    {
      let mut msg: *const c_char
	= c"Please set the environment variable TERM; see 'tset'.\n".as_ptr ();
      #[cfg (feature = "have_window_system")]
      {
      if !inhibit_window_system
	{ msg = c"Please set the environment variable DISPLAY or TERM; see 'tset'.\n".as_ptr (); }
      }
      fputs (msg, stderr);
      exit (1);
    }

/* C 7339-7410: `#ifndef HAVE_ANDROID` — live on the frozen target.  */
  {
    let t: *mut terminal;
    let f: *mut frame = XFRAME (selected_frame);

    init_foreground_group ();

    /* Open a display on the controlling tty. */
    t = init_tty (core::ptr::null (), terminal_type, true); /* Errors are fatal. */

    /* Convert the initial frame to use the new display. */
    if !FRAME_INITIAL_P (f)
      { emacs_abort (); }
    output_method! (f, (*t).r#type);
    (*f).terminal = t;

    (*t).reference_count += 1;
    #[cfg (feature = "msdos")]
    {
    (*f).output_data.tty = &mut the_only_tty_output;
    (*f).output_data.tty.display_info = &mut the_only_display_info;
    }
    #[cfg (not (feature = "msdos"))]
    {
    if FRAME_TERMCAP_P (f)
      { create_tty_output (f); }
    }
    (*( (*t).display_info.tty as *mut tty_display_info )).top_frame = selected_frame;
    change_frame_size (XFRAME (selected_frame),
                       FrameCols! ((*t).display_info.tty as *mut tty_display_info),
                       FrameRows! ((*t).display_info.tty as *mut tty_display_info),
		       false, false, true);

    /* Delete the initial terminal. */
    if { (*initial_terminal).reference_count -= 1; (*initial_terminal).reference_count == 0 }
        && (*initial_terminal).delete_terminal_hook.is_some ()
      { ((*initial_terminal).delete_terminal_hook.unwrap ()) (initial_terminal); }

    /* Update frame parameters to reflect the new type. */
    AUTO_FRAME_ARG! (tty_type_arg, Qtty_type (), Ftty_type (selected_frame));
    Fmodify_frame_parameters (selected_frame, tty_type_arg);
    AUTO_FRAME_ARG! (tty_arg, Qtty (),
		     if !(*( (*t).display_info.tty as *mut tty_display_info )).name.is_null ()
		       { build_string ((*( (*t).display_info.tty as *mut tty_display_info )).name) }
		       else { Qnil () });
    Fmodify_frame_parameters (selected_frame, tty_arg);
  }

  {
    let sf: *mut frame = SELECTED_FRAME ();
    let width: c_int = FRAME_TOTAL_COLS (sf);
    let height: c_int = FRAME_TOTAL_LINES (sf);
    let mut area: c_int = 0;

    /* If these sizes are so big they cause overflow, just ignore the
       change.  It's not clear what better we could do.  The rest of
       the code assumes that (width + 2) * height * sizeof (struct glyph)
       does not overflow and does not exceed PTRDIFF_MAX or SIZE_MAX.  */
    let (area1, add_overflow) = width.overflowing_add (2);
    area = area1;
    let (area2, mul_overflow) = area.overflowing_mul (height);
    area = area2;
    if add_overflow || mul_overflow
	|| (std::cmp::min (PTRDIFF_MAX as usize, SIZE_MAX)  /* P-127 */
	    / core::mem::size_of::<glyph> () as usize) < area as usize
      { fatal (c"screen size %dx%d too big".as_ptr (), width, height); }
  }

  calculate_costs (XFRAME (selected_frame));

  /* Set up faces of the initial terminal frame.  */
  if initialized && !noninteractive && NILP (Vinitial_window_system)
    { init_faces_initial (); }
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn init_display ()
{
  if noninteractive
    {
      if dumped_with_pdumper_p ()
        { init_faces_initial (); }
    }
  else
    { init_display_interactive (); }
}


DEFUN! ("internal-show-cursor", Finternal_show_cursor,
        Sinternal_show_cursor, 2, 2, 0,
        doc: "Set the cursor-visibility flag of WINDOW to SHOW.\nWINDOW nil means use the selected window.  SHOW non-nil means\nshow a cursor in WINDOW in the next redisplay.  SHOW nil means\ndon't show a cursor.",
        (window: Lisp_Object, show: Lisp_Object),
        {
  /* Don't change cursor state while redisplaying.  This could confuse
     output routines.  */
  if !redisplaying_p
    { window_bits! (cursor_off_p, decode_any_window (window), NILP (show)); }
  return Qnil ();
});


DEFUN! ("internal-show-cursor-p", Finternal_show_cursor_p,
        Sinternal_show_cursor_p, 0, 1, 0,
        doc: "Value is non-nil if next redisplay will display a cursor in WINDOW.\nWINDOW nil or omitted means report on the selected window.",
        (window: Lisp_Object),
        {
  return if window_bits! (cursor_off_p, decode_any_window (window)) { Qnil () } else { Qt () };
});


/***********************************************************************
			    Initialization
 ***********************************************************************/

/* C 3530-3545: the `DEFVAR_*`/`DEFVAR_INT` storage this file owns.  C keeps it in the single
   `globals` object (lisp.h 3495-3530) and `include/globals.rs` mirrors the names as
   declarations, so the owning unit defines them here (docs/status.md 3.29, the term.rs
   shape).  */
#[unsafe(no_mangle)]
pub static mut baud_rate: EMACS_INT = 0;
#[unsafe(no_mangle)]
pub static mut inverse_video: bool = false;
#[unsafe(no_mangle)]
pub static mut visible_bell: bool = false;
#[unsafe(no_mangle)]
pub static mut no_redraw_on_reenter: bool = false;
#[unsafe(no_mangle)]
pub static mut Vinitial_window_system: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut cursor_in_echo_area: bool = false;
#[unsafe(no_mangle)]
pub static mut mouse_prefer_closest_glyph: bool = false;
#[unsafe(no_mangle)]
pub static mut Vglyph_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vstandard_display_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vx_show_tooltip_timeout: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vtab_bar_position: Lisp_Object = lisp_h_Qnil ();

unsafe extern "C" fn syms_of_display_for_pdumper ()
{
  Vinitial_window_system = Qnil ();
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn syms_of_display ()
{
  defsubr (&raw mut Sredraw_frame);
  defsubr (&raw mut Sredraw_display);
  defsubr (&raw mut Sdisplay__update_for_mouse_movement);
  defsubr (&raw mut Sframe_or_buffer_changed_p);
  defsubr (&raw mut Sopen_termscript);
  defsubr (&raw mut Sding);
  defsubr (&raw mut Sredisplay);
  defsubr (&raw mut Ssleep_for);
  defsubr (&raw mut Ssend_string_to_terminal);
  defsubr (&raw mut Sinternal_show_cursor);
  defsubr (&raw mut Sinternal_show_cursor_p);
  defsubr (&raw mut Sframe__z_order_lessp);

  frame_and_buffer_state = make_vector (20, Qlambda ());
  staticpro (&raw const frame_and_buffer_state);

  /* This is the "purpose" slot of a display table.  */
  DEFSYM! (Qdisplay_table, c"display-table");
  DEFSYM! (Qframe__z_order_lessp, c"frame--z-order-lessp");
  DEFSYM! (Qtty_non_selected_cursor, c"tty-non-selected-cursor");

  DEFVAR_INT! (c"baud-rate", baud_rate,
	      "The output baud rate of the terminal.\nOn most systems, changing this value will affect the amount of padding\nand the other strategic decisions made during redisplay.");

  DEFVAR_BOOL! (c"inverse-video", inverse_video,
	       "Non-nil means invert the entire frame display.\nThis means everything is in inverse video which otherwise would not be.");

  DEFVAR_BOOL! (c"visible-bell", visible_bell,
	       "Non-nil means try to flash the frame to represent a bell.\n\nSee also `ring-bell-function'.");

  DEFVAR_BOOL! (c"no-redraw-on-reenter", no_redraw_on_reenter,
	       "Non-nil means no need to redraw entire frame after suspending.\nA non-nil value is useful if the terminal can automatically preserve\nEmacs's frame display when you reenter Emacs.\nIt is up to you to set this variable if your terminal can do that.");

  DEFVAR_LISP! (c"initial-window-system", Vinitial_window_system,
	       "Name of the window system that Emacs uses for the first frame.\nThe value is a symbol:\n nil for a termcap frame (a character-only terminal),\n `x' for an Emacs frame that is really an X window,\n `w32' for an Emacs frame that is a window on MS-Windows display,\n `ns' for an Emacs frame on a GNUstep or Macintosh Cocoa display,\n `pc' for a direct-write MS-DOS frame.\n `pgtk' for an Emacs frame using pure GTK facilities.\n `haiku' for an Emacs frame running in Haiku.\n\nUse of this variable as a boolean is deprecated.  Instead,\nuse `display-graphic-p' or any of the other `display-*-p'\npredicates which report frame's specific UI-related capabilities.");

  DEFVAR_KBOARD! (c"window-system", std::mem::offset_of! (kboard, Vwindow_system_),
		 "Name of window system through which the selected frame is displayed.\nThe value is a symbol:\n nil for a termcap frame (a character-only terminal),\n `x' for an Emacs frame that is really an X window,\n `w32' for an Emacs frame that is a window on MS-Windows display,\n `ns' for an Emacs frame on a GNUstep or Macintosh Cocoa display,\n `pc' for a direct-write MS-DOS frame.\n `pgtk' for an Emacs frame using pure GTK facilities.\n `haiku' for an Emacs frame running in Haiku.\n `android' for an Emacs frame running in Android.\n\nUse of this variable as a boolean is deprecated.  Instead,\nuse `display-graphic-p' or any of the other `display-*-p'\npredicates which report frame's specific UI-related capabilities.");

  DEFVAR_BOOL! (c"cursor-in-echo-area", cursor_in_echo_area,
	       "Non-nil means put cursor in minibuffer, at end of any message there.");

  DEFVAR_BOOL! (c"mouse-prefer-closest-glyph", mouse_prefer_closest_glyph,
	       "Non-nil means mouse click position is taken from glyph closest to click.\n\nWhen non-nil, mouse position lists will report buffer position set to\nthe position of the glyph that is the closest to the mouse pointer\nat the time of the click, instead of the glyph immediately under it.");
  mouse_prefer_closest_glyph = false;

  DEFVAR_LISP! (c"glyph-table", Vglyph_table,
	       "Table defining how to output a glyph code to the frame.\nIf not nil, this is a vector indexed by glyph code to define the glyph.\nEach element can be:\n integer: a glyph code which this glyph is an alias for.\n string: output this glyph using that string (not impl. in X windows).\n nil: this glyph mod 524288 is the code of a character to output,\n    and this glyph / 524288 is the face number (see `face-id') to use\n    while outputting it.");
  Vglyph_table = Qnil ();

  DEFVAR_LISP! (c"standard-display-table", Vstandard_display_table,
	       "Display table to use for buffers that specify none.\nIt is also used for standard output and error streams.\nSee `buffer-display-table' for more information.");
  Vstandard_display_table = Qnil ();

  DEFVAR_LISP! (c"x-show-tooltip-timeout", Vx_show_tooltip_timeout,
	      "The default timeout (in seconds) for `x-show-tip'.");
  Vx_show_tooltip_timeout = make_fixnum (5);

  DEFVAR_LISP! (c"tab-bar-position", Vtab_bar_position,
	       "Specify on which side from the tool bar the tab bar shall be.\nPossible values are t (below the tool bar), nil (above the tool bar).\nThis option affects only builds where the tool bar is not external.");

  pdumper_do_now_and_after_load (Some (syms_of_display_for_pdumper));

  Fprovide (intern_c_string (c"tty-child-frames".as_ptr ()), Qnil ());
}
